//! In-process Twitch video broadcast for the Windows build.
//!
//! Video comes either from Windows Graphics Capture of the game preview or from
//! an asynchronous Bevy offscreen-target readback in stream-only mode. Process
//! audio is captured with WASAPI application loopback, and dynamically linked
//! `FFmpeg` libraries encode/mux H.264 + AAC into Twitch's RTMP ingest. No
//! subprocess, virtual cable, or OBS installation is involved.

use std::{
    collections::{HashMap, VecDeque},
    fmt,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{self, Receiver, SyncSender, TrySendError},
    },
    thread,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, anyhow, bail};
use bevy::{
    camera::RenderTarget,
    prelude::*,
    render::{
        gpu_readback::{Readback, ReadbackComplete},
        render_resource::{TextureFormat, TextureUsages},
    },
    window::{PrimaryWindow, WindowRef, WindowResolution},
    winit::{UpdateMode, WinitSettings},
};
use ffmpeg::{
    ChannelLayout, Codec, Dictionary, Packet, Rational, codec, encoder, format, frame, software,
};
use ffmpeg_next as ffmpeg;
use stream_town_domain::{BroadcastConfig, BroadcastEncoderPreference, BroadcastRenderMode};
use wasapi::{AudioClient, Direction, SampleType, StreamMode, WaveFormat, initialize_mta};
use windows_capture::{
    capture::{Context as CaptureContext, GraphicsCaptureApiHandler},
    frame::Frame as CapturedWindowFrame,
    graphics_capture_api::InternalCaptureControl,
    settings::{
        ColorFormat, CursorCaptureSettings, DirtyRegionSettings, DrawBorderSettings,
        MinimumUpdateIntervalSettings, SecondaryWindowSettings, Settings as CaptureSettings,
    },
    window::Window as CapturableWindow,
};

use crate::{
    RuntimeConfig, SensitiveScreenActive, SensitiveScreenUpdateSet,
    twitch::{CredentialVault, OAuthClient, TwitchIngest},
};

const AUDIO_SAMPLE_RATE: u32 = 48_000;
const AUDIO_CHANNELS: usize = 2;
const AUDIO_FRAME_SAMPLES: usize = 1_024;
const AUDIO_QUEUE_CAPACITY: usize = 32;
const STREAM_HEALTH_REPORT_INTERVAL: Duration = Duration::from_secs(5);
const MAX_RECONNECT_DELAY_SECONDS: u64 = 30;
const OPERATOR_WINDOW_WIDTH: u32 = 960;
const OPERATOR_WINDOW_HEIGHT: u32 = 540;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DirectBroadcastPhase {
    Disabled,
    WaitingForBroadcasterAuthorization,
    WaitingForGameplay,
    ResolvingIngest,
    Connecting,
    Broadcasting,
    Reconnecting,
    Stopping,
    Stopped,
    Error(String),
}

impl DirectBroadcastPhase {
    #[must_use]
    pub(crate) const fn is_active(&self) -> bool {
        matches!(
            self,
            Self::WaitingForBroadcasterAuthorization
                | Self::WaitingForGameplay
                | Self::ResolvingIngest
                | Self::Connecting
                | Self::Broadcasting
                | Self::Reconnecting
                | Self::Stopping
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectBroadcastSnapshot {
    pub phase: DirectBroadcastPhase,
    pub encoder: Option<String>,
    pub ingest: Option<String>,
    pub captured_video_frames: u64,
    pub encoded_video_frames: u64,
    pub dropped_video_frames: u64,
    pub encoded_audio_frames: u64,
    pub dropped_audio_frames: u64,
    pub replaced_video_frames: u64,
    pub skipped_video_frames: u64,
    pub audio_queue_depth: u64,
    pub audio_queue_high_water: u64,
    pub captured_video_fps: f64,
    pub encoded_video_fps: f64,
    pub average_capture_ms: f64,
    pub maximum_capture_ms: f64,
    pub average_encode_ms: f64,
    pub maximum_encode_ms: f64,
}

#[derive(Resource)]
pub struct DirectBroadcastRuntime {
    phase: DirectBroadcastPhase,
    encoder: Option<String>,
    ingest: Option<String>,
    authorization: Option<Arc<Mutex<Receiver<AuthorizationEvent>>>>,
    pending_target: Option<BroadcastTarget>,
    controller: Option<BroadcastController>,
    capture_elapsed: f32,
    broadcast_started: Option<Instant>,
    health_reported_at: Option<Instant>,
    health_reported_metrics: BroadcastMetricsSnapshot,
    rolling_captured_video_fps: f64,
    rolling_encoded_video_fps: f64,
}

#[derive(Resource, Default)]
struct StreamOnlyCaptureState {
    target: Option<Handle<Image>>,
    readback_entity: Option<Entity>,
    operator_window: Option<Entity>,
    operator_camera: Option<Entity>,
    operator_root: Option<Entity>,
    previous_camera_targets: HashMap<Entity, RenderTarget>,
    previous_primary_visibility: Option<bool>,
    previous_unfocused_mode: Option<UpdateMode>,
    width: u32,
    height: u32,
}

#[derive(Component)]
pub(crate) struct StreamOperatorCamera;

#[derive(Component)]
pub(crate) struct StreamOperatorWindow;

#[derive(Component)]
struct StreamOperatorInfoText;

type StreamCameraTargetQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static mut RenderTarget),
    (With<Camera>, Without<StreamOperatorCamera>),
>;

impl Default for DirectBroadcastRuntime {
    fn default() -> Self {
        Self {
            phase: DirectBroadcastPhase::Disabled,
            encoder: None,
            ingest: None,
            authorization: None,
            pending_target: None,
            controller: None,
            capture_elapsed: 0.0,
            broadcast_started: None,
            health_reported_at: None,
            health_reported_metrics: BroadcastMetricsSnapshot::default(),
            rolling_captured_video_fps: 0.0,
            rolling_encoded_video_fps: 0.0,
        }
    }
}

impl DirectBroadcastRuntime {
    #[must_use]
    pub fn snapshot(&self) -> DirectBroadcastSnapshot {
        let metrics = self.controller.as_ref().map_or_else(
            BroadcastMetricsSnapshot::default,
            BroadcastController::metrics,
        );
        let elapsed = self
            .broadcast_started
            .map_or(0.0, |started| started.elapsed().as_secs_f64());
        DirectBroadcastSnapshot {
            phase: self.phase.clone(),
            encoder: self.encoder.clone(),
            ingest: self.ingest.clone(),
            captured_video_frames: metrics.captured_video,
            encoded_video_frames: metrics.encoded_video,
            dropped_video_frames: metrics.dropped_video,
            encoded_audio_frames: metrics.encoded_audio,
            dropped_audio_frames: metrics.dropped_audio,
            replaced_video_frames: metrics.replaced_video,
            skipped_video_frames: metrics.skipped_video,
            audio_queue_depth: metrics.queued_audio,
            audio_queue_high_water: metrics.audio_queue_high_water,
            captured_video_fps: if self.rolling_captured_video_fps > 0.0 {
                self.rolling_captured_video_fps
            } else {
                rate_per_second(metrics.captured_video, elapsed)
            },
            encoded_video_fps: if self.rolling_encoded_video_fps > 0.0 {
                self.rolling_encoded_video_fps
            } else {
                rate_per_second(metrics.encoded_video, elapsed)
            },
            average_capture_ms: average_milliseconds(
                metrics.capture_micros,
                metrics.capture_samples,
            ),
            maximum_capture_ms: micros_to_milliseconds(metrics.maximum_capture_micros),
            average_encode_ms: average_milliseconds(
                metrics.video_encode_micros,
                metrics.encoded_video,
            ),
            maximum_encode_ms: micros_to_milliseconds(metrics.maximum_video_encode_micros),
        }
    }

    #[cfg(test)]
    pub(crate) fn set_phase_for_test(&mut self, phase: DirectBroadcastPhase) {
        self.phase = phase;
    }
}

fn rate_per_second(count: u64, elapsed_seconds: f64) -> f64 {
    if elapsed_seconds > f64::EPSILON {
        f64::from(u32::try_from(count).unwrap_or(u32::MAX)) / elapsed_seconds
    } else {
        0.0
    }
}

fn average_milliseconds(total_micros: u64, samples: u64) -> f64 {
    if samples == 0 {
        0.0
    } else {
        micros_to_milliseconds(total_micros) / f64::from(u32::try_from(samples).unwrap_or(u32::MAX))
    }
}

fn micros_to_milliseconds(micros: u64) -> f64 {
    Duration::from_micros(micros).as_secs_f64() * 1_000.0
}

fn duration_as_micros(duration: Duration) -> u64 {
    u64::try_from(duration.as_micros()).unwrap_or(u64::MAX)
}

#[derive(Resource, Default)]
pub(crate) struct DirectBroadcastControl {
    restart_requested: bool,
    stop_requested: bool,
}

impl DirectBroadcastControl {
    pub(crate) fn request_restart(&mut self) {
        self.restart_requested = true;
        self.stop_requested = false;
    }

    pub(crate) fn request_stop(&mut self) {
        self.stop_requested = true;
        self.restart_requested = false;
    }

    #[cfg(test)]
    pub(crate) const fn stop_requested_for_test(&self) -> bool {
        self.stop_requested
    }
}

pub struct DirectTwitchBroadcastPlugin;

impl Plugin for DirectTwitchBroadcastPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DirectBroadcastRuntime>()
            .init_resource::<DirectBroadcastControl>()
            .init_resource::<StreamOnlyCaptureState>()
            .add_systems(
                Update,
                (
                    start_local_broadcast_diagnostic,
                    apply_direct_broadcast_control,
                    poll_direct_broadcast_authorization,
                    start_prepared_broadcast_when_gameplay_ready,
                    poll_direct_broadcast_worker,
                    sync_stream_only_capture,
                    update_stream_operator_info,
                    capture_direct_broadcast_frame.after(SensitiveScreenUpdateSet),
                )
                    .chain(),
            );
    }
}

fn start_local_broadcast_diagnostic(
    config: Res<RuntimeConfig>,
    gameplay_ready: Option<Res<crate::GameplayReady>>,
    mut runtime: ResMut<DirectBroadcastRuntime>,
) {
    if runtime.phase != DirectBroadcastPhase::Disabled
        || gameplay_ready.is_none()
        || std::env::var_os("STREAM_TOWN_AUTOSTART_BROADCAST_DIAGNOSTIC").is_none()
    {
        return;
    }
    let Some(output) = std::env::var_os("STREAM_TOWN_BROADCAST_DIAGNOSTIC_OUTPUT") else {
        runtime.phase = DirectBroadcastPhase::Error(
            "broadcast diagnostic autostart requires STREAM_TOWN_BROADCAST_DIAGNOSTIC_OUTPUT"
                .to_owned(),
        );
        return;
    };
    if !config.0.twitch.broadcast.enabled {
        runtime.phase = DirectBroadcastPhase::Error(
            "broadcast diagnostic autostart requires direct streaming to be enabled".to_owned(),
        );
        return;
    }
    let output = std::path::PathBuf::from(output);
    if let Some(parent) = output.parent()
        && let Err(error) = std::fs::create_dir_all(parent)
    {
        runtime.phase = DirectBroadcastPhase::Error(format!(
            "could not create broadcast diagnostic directory: {error}"
        ));
        return;
    }
    let target = BroadcastTarget {
        ingest_name: "local FLV diagnostic".to_owned(),
        url: output.to_string_lossy().into_owned(),
    };
    runtime.ingest = Some(target.ingest_name.clone());
    runtime.phase = DirectBroadcastPhase::Connecting;
    match BroadcastController::start(
        target,
        config.0.twitch.broadcast.clone(),
        config.0.window.title.clone(),
    ) {
        Ok(controller) => runtime.controller = Some(controller),
        Err(error) => {
            runtime.phase = DirectBroadcastPhase::Error(format!(
                "could not start local broadcast diagnostic: {error:#}"
            ));
        }
    }
}

fn apply_direct_broadcast_control(
    config: Res<RuntimeConfig>,
    mut control: ResMut<DirectBroadcastControl>,
    mut runtime: ResMut<DirectBroadcastRuntime>,
) {
    if std::mem::take(&mut control.stop_requested) {
        runtime.authorization = None;
        runtime.pending_target = None;
        if let Some(controller) = &runtime.controller {
            controller.request_stop();
            runtime.phase = DirectBroadcastPhase::Stopping;
        } else {
            runtime.phase = DirectBroadcastPhase::Stopped;
        }
        return;
    }
    if !std::mem::take(&mut control.restart_requested) {
        return;
    }
    *runtime = DirectBroadcastRuntime::default();
    configure_direct_broadcast(&config.0, &mut runtime);
}

fn configure_direct_broadcast(
    config: &stream_town_domain::GameConfig,
    runtime: &mut DirectBroadcastRuntime,
) {
    if std::env::var_os("STREAM_TOWN_DISABLE_DIRECT_BROADCAST").is_some() {
        runtime.phase = DirectBroadcastPhase::Disabled;
        return;
    }
    let twitch = &config.twitch;
    // Going live is deliberately an operator action. Persisted settings and
    // legacy `start_on_launch` values configure the encoder, but never start a
    // public stream while the application is still booting.
    if !twitch.broadcast.enabled {
        runtime.phase = DirectBroadcastPhase::Disabled;
        return;
    }
    if twitch.client_id.trim().is_empty() {
        runtime.phase = DirectBroadcastPhase::Error(
            "direct broadcast requires the Twitch public client ID".to_owned(),
        );
        return;
    }

    let client_id = twitch.client_id.clone();
    let channel_login = twitch.channel_login.clone();
    let requested_ingest = twitch.broadcast.ingest.clone();
    let bandwidth_test = twitch.broadcast.bandwidth_test;
    let (sender, receiver) = mpsc::channel();
    runtime.authorization = Some(Arc::new(Mutex::new(receiver)));
    runtime.phase = DirectBroadcastPhase::WaitingForBroadcasterAuthorization;
    let spawn = thread::Builder::new()
        .name("stream-town-broadcast-auth".to_owned())
        .spawn(move || {
            let outcome = resolve_broadcast_target(
                &client_id,
                &channel_login,
                &requested_ingest,
                bandwidth_test,
            );
            let _ = sender.send(outcome.map_or_else(
                |error| AuthorizationEvent::Error(format!("{error:#}")),
                AuthorizationEvent::Ready,
            ));
        });
    if let Err(error) = spawn {
        runtime.authorization = None;
        runtime.phase = DirectBroadcastPhase::Error(format!(
            "could not start Twitch broadcast authorization: {error}"
        ));
    }
}

fn resolve_broadcast_target(
    client_id: &str,
    channel_login: &str,
    requested_ingest: &str,
    bandwidth_test: bool,
) -> Result<BroadcastTarget> {
    let tokio = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to construct the Twitch broadcast network runtime")?;
    tokio.block_on(async {
        let oauth = OAuthClient::broadcaster(client_id.to_owned())?;
        let vault = CredentialVault::broadcaster(client_id, channel_login);
        let (token, validation) = oauth.load_validated_token(&vault).await.with_context(|| {
            format!("Twitch broadcaster '{channel_login}' is not authorized; use stream_town_tools")
        })?;
        if validation.login != channel_login {
            bail!(
                "stored broadcaster token belongs to '{}', expected '{}'",
                validation.login,
                channel_login
            );
        }
        let stream_key = oauth.stream_key(&token, &validation.user_id).await?;
        let ingests = oauth.ingests().await?;
        let ingest = select_ingest(&ingests, requested_ingest)?;
        let url = build_ingest_url(&ingest.url_template, stream_key.expose(), bandwidth_test)?;
        Ok(BroadcastTarget {
            ingest_name: ingest.name.clone(),
            url,
        })
    })
}

fn poll_direct_broadcast_authorization(mut runtime: ResMut<DirectBroadcastRuntime>) {
    let event = runtime
        .authorization
        .as_ref()
        .and_then(|receiver| receiver.lock().ok())
        .and_then(|receiver| receiver.try_recv().ok());
    let Some(event) = event else {
        return;
    };
    runtime.authorization = None;
    match event {
        AuthorizationEvent::Ready(target) => {
            runtime.ingest = Some(target.ingest_name.clone());
            runtime.pending_target = Some(target);
            runtime.phase = DirectBroadcastPhase::WaitingForGameplay;
        }
        AuthorizationEvent::Error(error) => {
            runtime.phase = DirectBroadcastPhase::Error(error);
        }
    }
}

fn start_prepared_broadcast_when_gameplay_ready(
    config: Res<RuntimeConfig>,
    gameplay_ready: Option<Res<crate::GameplayReady>>,
    mut runtime: ResMut<DirectBroadcastRuntime>,
) {
    if !prepared_broadcast_can_start(&runtime.phase, gameplay_ready.is_some()) {
        return;
    }
    let Some(target) = runtime.pending_target.take() else {
        runtime.phase = DirectBroadcastPhase::Error(
            "prepared broadcast target disappeared before gameplay became ready".to_owned(),
        );
        return;
    };
    runtime.phase = DirectBroadcastPhase::Connecting;
    match BroadcastController::start(
        target,
        config.0.twitch.broadcast.clone(),
        config.0.window.title.clone(),
    ) {
        Ok(controller) => runtime.controller = Some(controller),
        Err(error) => {
            runtime.phase = DirectBroadcastPhase::Error(format!("{error:#}"));
        }
    }
}

const fn prepared_broadcast_can_start(phase: &DirectBroadcastPhase, gameplay_ready: bool) -> bool {
    gameplay_ready && matches!(phase, DirectBroadcastPhase::WaitingForGameplay)
}

fn poll_direct_broadcast_worker(
    config: Res<RuntimeConfig>,
    mut runtime: ResMut<DirectBroadcastRuntime>,
) {
    let events = runtime
        .controller
        .as_ref()
        .map(BroadcastController::events)
        .unwrap_or_default();
    for event in events {
        match event {
            WorkerEvent::Connecting => runtime.phase = DirectBroadcastPhase::Connecting,
            WorkerEvent::Broadcasting { encoder } => {
                runtime.encoder = Some(encoder);
                runtime.phase = DirectBroadcastPhase::Broadcasting;
                let now = Instant::now();
                runtime.broadcast_started.get_or_insert(now);
                runtime.health_reported_at.get_or_insert(now);
            }
            WorkerEvent::Reconnecting(error) => {
                runtime.phase = DirectBroadcastPhase::Reconnecting;
                warn!(%error, "direct Twitch broadcast reconnecting");
            }
            WorkerEvent::Stopped => {
                runtime.phase = DirectBroadcastPhase::Stopped;
                runtime.controller = None;
            }
            WorkerEvent::Error(error) => {
                runtime.phase = DirectBroadcastPhase::Error(error);
            }
        }
    }
    report_stream_health(&mut runtime, config.0.twitch.broadcast.frames_per_second);
}

fn report_stream_health(runtime: &mut DirectBroadcastRuntime, target_fps: u8) {
    if runtime.phase != DirectBroadcastPhase::Broadcasting {
        return;
    }
    let now = Instant::now();
    let Some(previous_at) = runtime.health_reported_at else {
        runtime.health_reported_at = Some(now);
        return;
    };
    let interval = now.saturating_duration_since(previous_at);
    if interval < STREAM_HEALTH_REPORT_INTERVAL {
        return;
    }
    let metrics = runtime.controller.as_ref().map_or_else(
        BroadcastMetricsSnapshot::default,
        BroadcastController::metrics,
    );
    let elapsed = interval.as_secs_f64();
    let captured_fps = rate_per_second(
        metrics
            .captured_video
            .saturating_sub(runtime.health_reported_metrics.captured_video),
        elapsed,
    );
    let encoded_fps = rate_per_second(
        metrics
            .encoded_video
            .saturating_sub(runtime.health_reported_metrics.encoded_video),
        elapsed,
    );
    let audio_fps = rate_per_second(
        metrics
            .encoded_audio
            .saturating_sub(runtime.health_reported_metrics.encoded_audio),
        elapsed,
    );
    let new_video_drops = metrics
        .dropped_video
        .saturating_sub(runtime.health_reported_metrics.dropped_video);
    let new_audio_drops = metrics
        .dropped_audio
        .saturating_sub(runtime.health_reported_metrics.dropped_audio);
    let average_encode_ms = average_milliseconds(
        metrics
            .video_encode_micros
            .saturating_sub(runtime.health_reported_metrics.video_encode_micros),
        metrics
            .encoded_video
            .saturating_sub(runtime.health_reported_metrics.encoded_video),
    );
    runtime.rolling_captured_video_fps = captured_fps;
    runtime.rolling_encoded_video_fps = encoded_fps;
    let minimum_healthy_fps = f64::from(target_fps) * 0.9;
    let unhealthy = captured_fps < minimum_healthy_fps
        || encoded_fps < minimum_healthy_fps
        || new_audio_drops > 0;
    if unhealthy {
        warn!(
            target_fps,
            captured_fps,
            encoded_fps,
            audio_fps,
            new_video_drops,
            new_audio_drops,
            audio_queue_depth = metrics.queued_audio,
            average_encode_ms,
            maximum_encode_ms = micros_to_milliseconds(metrics.maximum_video_encode_micros),
            "direct Twitch broadcast health is below target"
        );
    } else {
        info!(
            target_fps,
            captured_fps,
            encoded_fps,
            audio_fps,
            new_video_drops,
            new_audio_drops,
            audio_queue_depth = metrics.queued_audio,
            average_encode_ms,
            "direct Twitch broadcast health"
        );
    }
    runtime.health_reported_at = Some(now);
    runtime.health_reported_metrics = metrics;
}

fn sync_stream_only_capture(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    runtime: Res<DirectBroadcastRuntime>,
    mut state: ResMut<StreamOnlyCaptureState>,
    mut images: Option<ResMut<Assets<Image>>>,
    mut camera_targets: StreamCameraTargetQuery,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut winit: Option<ResMut<WinitSettings>>,
) {
    let stream_only_active = config.0.twitch.broadcast.render_mode
        == BroadcastRenderMode::StreamOnly
        && runtime.controller.is_some()
        && matches!(
            runtime.phase,
            DirectBroadcastPhase::Connecting
                | DirectBroadcastPhase::Broadcasting
                | DirectBroadcastPhase::Reconnecting
        );

    if stream_only_active {
        if state.target.is_none() {
            let Some(images) = images.as_deref_mut() else {
                return;
            };
            let width = u32::from(config.0.twitch.broadcast.width);
            let height = u32::from(config.0.twitch.broadcast.height);
            let mut target =
                Image::new_target_texture(width, height, TextureFormat::Bgra8UnormSrgb, None);
            target.texture_descriptor.usage |= TextureUsages::COPY_SRC;
            let target = images.add(target);
            let readback_entity = commands
                .spawn(Readback::texture(target.clone()))
                .observe(publish_stream_only_frame)
                .id();
            state.target = Some(target);
            state.readback_entity = Some(readback_entity);
            state.width = width;
            state.height = height;
            let operator_window = commands
                .spawn((
                    StreamOperatorWindow,
                    Window {
                        title: "Stream Town — Operator".to_owned(),
                        resolution: WindowResolution::new(
                            OPERATOR_WINDOW_WIDTH,
                            OPERATOR_WINDOW_HEIGHT,
                        ),
                        resizable: false,
                        ..default()
                    },
                ))
                .id();
            let operator_camera = commands
                .spawn((
                    StreamOperatorCamera,
                    Camera2d,
                    Camera {
                        order: 1_000,
                        ..default()
                    },
                    RenderTarget::Window(WindowRef::Entity(operator_window)),
                ))
                .id();
            let operator_root = spawn_stream_operator_view(
                &mut commands,
                operator_camera,
                state.target.as_ref().expect("target was just installed"),
            );
            state.operator_window = Some(operator_window);
            state.operator_camera = Some(operator_camera);
            state.operator_root = Some(operator_root);
            info!(width, height, "stream-only offscreen render target enabled");
        }
        let Some(target) = state.target.clone() else {
            return;
        };
        for (entity, mut camera_target) in &mut camera_targets {
            if matches!(&*camera_target, RenderTarget::Image(image) if image.handle == target) {
                continue;
            }
            if camera_targets_primary_window(&camera_target) {
                state
                    .previous_camera_targets
                    .entry(entity)
                    .or_insert_with(|| camera_target.clone());
                *camera_target = RenderTarget::Image(target.clone().into());
            }
        }
        if let Ok(mut window) = primary_window.single_mut()
            && state.previous_primary_visibility.is_none()
        {
            state.previous_primary_visibility = Some(window.visible);
            window.visible = false;
        }
        if let Some(winit) = winit.as_deref_mut() {
            state
                .previous_unfocused_mode
                .get_or_insert(winit.unfocused_mode);
            winit.unfocused_mode = UpdateMode::Continuous;
        }
        return;
    }

    if state.target.is_none() {
        return;
    }
    for (entity, previous_target) in state.previous_camera_targets.drain() {
        if let Ok((_, mut camera_target)) = camera_targets.get_mut(entity) {
            *camera_target = previous_target;
        }
    }
    if let Ok(mut window) = primary_window.single_mut()
        && let Some(visible) = state.previous_primary_visibility.take()
    {
        window.visible = visible;
    }
    if let Some(previous) = state.previous_unfocused_mode.take()
        && let Some(winit) = winit.as_deref_mut()
    {
        winit.unfocused_mode = previous;
    }
    if let Some(entity) = state.readback_entity.take() {
        commands.entity(entity).despawn();
    }
    if let Some(entity) = state.operator_root.take() {
        commands.entity(entity).despawn();
    }
    if let Some(entity) = state.operator_camera.take() {
        commands.entity(entity).despawn();
    }
    if let Some(entity) = state.operator_window.take() {
        commands.entity(entity).despawn();
    }
    if let Some(target) = state.target.take()
        && let Some(images) = images.as_deref_mut()
    {
        images.remove(target.id());
    }
    state.width = 0;
    state.height = 0;
    info!("stream-only offscreen render target disabled; local preview restored");
}

fn spawn_stream_operator_view(
    commands: &mut Commands,
    camera: Entity,
    stream_target: &Handle<Image>,
) -> Entity {
    commands
        .spawn((
            Name::new("Stream-only operator information view"),
            UiTargetCamera(camera),
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                padding: UiRect::all(px(48)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.018, 0.024, 0.034)),
            GlobalZIndex(10_000),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("STREAM TOWN · OPERATOR VIEW"),
                TextFont {
                    font_size: FontSize::Px(30.0),
                    ..default()
                },
                TextColor(Color::srgb(0.82, 0.92, 1.0)),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(48),
                    top: px(42),
                    ..default()
                },
            ));
            root.spawn((
                StreamOperatorInfoText,
                Text::new("Preparing direct stream…"),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(Color::srgb(0.70, 0.78, 0.86)),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(50),
                    top: px(100),
                    ..default()
                },
            ));
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: px(42),
                    bottom: px(42),
                    width: px(336),
                    height: px(196),
                    padding: UiRect::all(px(8)),
                    border: UiRect::all(px(2)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.035, 0.045, 0.06)),
                BorderColor::all(Color::srgb(0.22, 0.35, 0.48)),
            ))
            .with_children(|preview| {
                preview.spawn((
                    Name::new("Low-resolution stream preview"),
                    ImageNode::new(stream_target.clone()),
                    Node {
                        width: percent(100),
                        height: percent(100),
                        ..default()
                    },
                ));
            });
            root.spawn((
                Text::new(
                    "Preview · 320 × 180\nUse the local LIVE control or Esc menu to end the stream",
                ),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(Color::srgb(0.52, 0.62, 0.72)),
                Node {
                    position_type: PositionType::Absolute,
                    right: px(48),
                    bottom: px(250),
                    ..default()
                },
            ));
        })
        .id()
}

fn update_stream_operator_info(
    runtime: Res<DirectBroadcastRuntime>,
    mut text: Query<&mut Text, With<StreamOperatorInfoText>>,
) {
    let Ok(mut text) = text.single_mut() else {
        return;
    };
    let snapshot = runtime.snapshot();
    **text = format!(
        "Status: {:?}\nEncoder: {}\nStream motion: {:.1} FPS\nOutput cadence: {:.1} FPS\nVideo drops: {} · Audio drops: {}\nEncode latency: {:.2} ms average / {:.2} ms maximum",
        snapshot.phase,
        snapshot.encoder.as_deref().unwrap_or("starting"),
        snapshot.captured_video_fps,
        snapshot.encoded_video_fps,
        snapshot.dropped_video_frames,
        snapshot.dropped_audio_frames,
        snapshot.average_encode_ms,
        snapshot.maximum_encode_ms,
    );
}

const fn camera_targets_primary_window(target: &RenderTarget) -> bool {
    matches!(target, RenderTarget::Window(WindowRef::Primary))
}

fn publish_stream_only_frame(
    mut event: On<ReadbackComplete>,
    state: Res<StreamOnlyCaptureState>,
    sensitive_screen: Res<SensitiveScreenActive>,
    runtime: Res<DirectBroadcastRuntime>,
) {
    if sensitive_screen.0 || state.width == 0 || state.height == 0 {
        return;
    }
    let Some(controller) = runtime.controller.as_ref() else {
        return;
    };
    let capture_started = Instant::now();
    let readback = std::mem::take(&mut event.event_mut().data);
    let pixels = remove_gpu_row_padding(readback, state.width, state.height);
    if pixels.is_empty() {
        controller.drop_video_frames(1);
        return;
    }
    controller
        .metrics
        .observe_capture_latency(capture_started.elapsed());
    let _ = controller.send_video(VideoFrame {
        width: state.width,
        height: state.height,
        pixel_format: VideoPixelFormat::Bgra,
        pixels,
    });
}

fn remove_gpu_row_padding(mut data: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let row_bytes = usize::try_from(width).unwrap_or_default().saturating_mul(4);
    let aligned_row_bytes = row_bytes.div_ceil(256).saturating_mul(256);
    let height = usize::try_from(height).unwrap_or_default();
    let expected = aligned_row_bytes.saturating_mul(height);
    if row_bytes == 0 || height == 0 || data.len() < expected {
        return Vec::new();
    }
    if row_bytes == aligned_row_bytes {
        data.truncate(row_bytes.saturating_mul(height));
        return data;
    }
    for row in 1..height {
        let source = row.saturating_mul(aligned_row_bytes);
        let destination = row.saturating_mul(row_bytes);
        data.copy_within(source..source + row_bytes, destination);
    }
    data.truncate(row_bytes.saturating_mul(height));
    data
}

fn capture_direct_broadcast_frame(
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    sensitive_screen: Res<SensitiveScreenActive>,
    mut runtime: ResMut<DirectBroadcastRuntime>,
) {
    if let Some(controller) = &runtime.controller {
        controller.set_sensitive_screen(sensitive_screen.0);
    }
    if runtime.phase != DirectBroadcastPhase::Broadcasting {
        return;
    }
    if !sensitive_screen.0 {
        runtime.capture_elapsed = 0.0;
        return;
    }
    let frame_period = 1.0 / f32::from(config.0.twitch.broadcast.frames_per_second);
    runtime.capture_elapsed += time.delta_secs();
    if runtime.capture_elapsed < frame_period {
        return;
    }
    let mut due_frames = 0_u64;
    while runtime.capture_elapsed >= frame_period {
        runtime.capture_elapsed -= frame_period;
        due_frames = due_frames.saturating_add(1);
    }
    if due_frames > 1
        && let Some(controller) = &runtime.controller
    {
        controller.drop_video_frames(due_frames - 1);
    }
    let width = u32::from(config.0.twitch.broadcast.width);
    let height = u32::from(config.0.twitch.broadcast.height);
    let rgba = sensitive_rgba_frame(width, height);
    let _ = runtime.controller.as_ref().is_some_and(|controller| {
        controller.send_video(VideoFrame {
            width,
            height,
            pixel_format: VideoPixelFormat::Rgba,
            pixels: rgba,
        })
    });
}

fn sensitive_rgba_frame(width: u32, height: u32) -> Vec<u8> {
    let bytes = usize::try_from(width)
        .unwrap_or(0)
        .saturating_mul(usize::try_from(height).unwrap_or(0))
        .saturating_mul(4);
    let mut rgba = vec![0; bytes];
    for alpha in rgba.iter_mut().skip(3).step_by(4) {
        *alpha = 255;
    }
    draw_centered_sensitive_label(&mut rgba, width, height);
    rgba
}

fn draw_centered_sensitive_label(rgba: &mut [u8], width: u32, height: u32) {
    const LABEL: &str = "SENSITIVE INFORMATION HIDDEN";
    const GLYPH_WIDTH: u32 = 5;
    const GLYPH_HEIGHT: u32 = 7;
    const GLYPH_GAP: u32 = 1;
    let unscaled_width = u32::try_from(LABEL.chars().count())
        .unwrap_or_default()
        .saturating_mul(GLYPH_WIDTH + GLYPH_GAP)
        .saturating_sub(GLYPH_GAP);
    if width < unscaled_width || height < GLYPH_HEIGHT {
        return;
    }
    let scale = (width / unscaled_width.max(1))
        .min(height / (GLYPH_HEIGHT * 4).max(1))
        .clamp(1, 6);
    let label_width = unscaled_width.saturating_mul(scale);
    let label_height = GLYPH_HEIGHT.saturating_mul(scale);
    let origin_x = width.saturating_sub(label_width) / 2;
    let origin_y = height.saturating_sub(label_height) / 2;
    let stride = usize::try_from(width).unwrap_or_default().saturating_mul(4);

    for (glyph_index, character) in LABEL.chars().enumerate() {
        let glyph = sensitive_label_glyph(character);
        let glyph_x = origin_x.saturating_add(
            u32::try_from(glyph_index)
                .unwrap_or_default()
                .saturating_mul((GLYPH_WIDTH + GLYPH_GAP) * scale),
        );
        for (row, bits) in glyph.into_iter().enumerate() {
            for column in 0..GLYPH_WIDTH {
                if bits & (1 << (GLYPH_WIDTH - 1 - column)) == 0 {
                    continue;
                }
                for dy in 0..scale {
                    for dx in 0..scale {
                        let x = glyph_x + column * scale + dx;
                        let y = origin_y + u32::try_from(row).unwrap_or_default() * scale + dy;
                        let offset = usize::try_from(y)
                            .unwrap_or_default()
                            .saturating_mul(stride)
                            .saturating_add(
                                usize::try_from(x).unwrap_or_default().saturating_mul(4),
                            );
                        if let Some(pixel) = rgba.get_mut(offset..offset.saturating_add(4)) {
                            pixel.copy_from_slice(&[255, 255, 255, 255]);
                        }
                    }
                }
            }
        }
    }
}

const fn sensitive_label_glyph(character: char) -> [u8; 7] {
    match character {
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        _ => [0; 7],
    }
}

#[derive(Debug)]
enum AuthorizationEvent {
    Ready(BroadcastTarget),
    Error(String),
}

struct BroadcastTarget {
    ingest_name: String,
    url: String,
}

impl fmt::Debug for BroadcastTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BroadcastTarget")
            .field("ingest_name", &self.ingest_name)
            .field("url", &"[redacted]")
            .finish()
    }
}

fn select_ingest<'a>(ingests: &'a [TwitchIngest], requested: &str) -> Result<&'a TwitchIngest> {
    if ingests.is_empty() {
        bail!("Twitch returned no ingest endpoints");
    }
    let requested = requested.trim().to_ascii_lowercase();
    if requested.is_empty() {
        return ingests
            .iter()
            .find(|ingest| ingest.is_default)
            .or_else(|| ingests.first())
            .context("Twitch returned no ingest endpoints");
    }
    ingests
        .iter()
        .find(|ingest| ingest.name.to_ascii_lowercase().contains(&requested))
        .with_context(|| format!("no Twitch ingest matches '{requested}'"))
}

fn build_ingest_url(template: &str, stream_key: &str, bandwidth_test: bool) -> Result<String> {
    if stream_key.trim().is_empty() {
        bail!("Twitch stream key is empty");
    }
    if !template.starts_with("rtmp://") && !template.starts_with("rtmps://") {
        bail!("Twitch returned a non-RTMP ingest URL");
    }
    if !template.contains("{stream_key}") {
        bail!("Twitch ingest URL is missing its stream-key placeholder");
    }
    let mut url = template.replace("{stream_key}", stream_key);
    if bandwidth_test {
        url.push_str(if url.contains('?') {
            "&bandwidthtest=true"
        } else {
            "?bandwidthtest=true"
        });
    }
    Ok(url)
}

#[derive(Clone, Debug)]
struct VideoFrame {
    width: u32,
    height: u32,
    pixel_format: VideoPixelFormat,
    pixels: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum VideoPixelFormat {
    Rgba,
    Bgra,
}

impl VideoPixelFormat {
    const fn ffmpeg(self) -> ffmpeg::format::Pixel {
        match self {
            Self::Rgba => ffmpeg::format::Pixel::RGBA,
            Self::Bgra => ffmpeg::format::Pixel::BGRA,
        }
    }
}

#[derive(Clone, Debug)]
struct AudioFrame {
    pts: i64,
    samples: Vec<f32>,
}

#[derive(Clone, Debug)]
enum AudioInput {
    Frame(AudioFrame),
    Stop,
}

#[derive(Clone, Debug)]
enum WorkerEvent {
    Connecting,
    Broadcasting { encoder: String },
    Reconnecting(String),
    Stopped,
    Error(String),
}

#[derive(Default)]
struct BroadcastMetrics {
    captured_video: AtomicU64,
    encoded_video: AtomicU64,
    dropped_video: AtomicU64,
    encoded_audio: AtomicU64,
    dropped_audio: AtomicU64,
    replaced_video: AtomicU64,
    skipped_video: AtomicU64,
    queued_audio: AtomicU64,
    audio_queue_high_water: AtomicU64,
    capture_samples: AtomicU64,
    capture_micros: AtomicU64,
    maximum_capture_micros: AtomicU64,
    video_encode_micros: AtomicU64,
    maximum_video_encode_micros: AtomicU64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct BroadcastMetricsSnapshot {
    captured_video: u64,
    encoded_video: u64,
    dropped_video: u64,
    encoded_audio: u64,
    dropped_audio: u64,
    replaced_video: u64,
    skipped_video: u64,
    queued_audio: u64,
    audio_queue_high_water: u64,
    capture_samples: u64,
    capture_micros: u64,
    maximum_capture_micros: u64,
    video_encode_micros: u64,
    maximum_video_encode_micros: u64,
}

impl BroadcastMetrics {
    fn snapshot(&self) -> BroadcastMetricsSnapshot {
        BroadcastMetricsSnapshot {
            captured_video: self.captured_video.load(Ordering::Relaxed),
            encoded_video: self.encoded_video.load(Ordering::Relaxed),
            dropped_video: self.dropped_video.load(Ordering::Relaxed),
            encoded_audio: self.encoded_audio.load(Ordering::Relaxed),
            dropped_audio: self.dropped_audio.load(Ordering::Relaxed),
            replaced_video: self.replaced_video.load(Ordering::Relaxed),
            skipped_video: self.skipped_video.load(Ordering::Relaxed),
            queued_audio: self.queued_audio.load(Ordering::Relaxed),
            audio_queue_high_water: self.audio_queue_high_water.load(Ordering::Relaxed),
            capture_samples: self.capture_samples.load(Ordering::Relaxed),
            capture_micros: self.capture_micros.load(Ordering::Relaxed),
            maximum_capture_micros: self.maximum_capture_micros.load(Ordering::Relaxed),
            video_encode_micros: self.video_encode_micros.load(Ordering::Relaxed),
            maximum_video_encode_micros: self.maximum_video_encode_micros.load(Ordering::Relaxed),
        }
    }

    fn observe_capture_latency(&self, duration: Duration) {
        let micros = duration_as_micros(duration);
        self.capture_samples.fetch_add(1, Ordering::Relaxed);
        self.capture_micros.fetch_add(micros, Ordering::Relaxed);
        self.maximum_capture_micros
            .fetch_max(micros, Ordering::Relaxed);
    }

    fn observe_video_encode_latency(&self, duration: Duration) {
        let micros = duration_as_micros(duration);
        self.video_encode_micros
            .fetch_add(micros, Ordering::Relaxed);
        self.maximum_video_encode_micros
            .fetch_max(micros, Ordering::Relaxed);
    }
}

struct WindowCaptureFlags {
    video: Arc<Mutex<Option<VideoFrame>>>,
    metrics: Arc<BroadcastMetrics>,
    stop: Arc<AtomicBool>,
    sensitive_screen: Arc<AtomicBool>,
}

struct WindowCaptureHandler {
    video: Arc<Mutex<Option<VideoFrame>>>,
    metrics: Arc<BroadcastMetrics>,
    stop: Arc<AtomicBool>,
    sensitive_screen: Arc<AtomicBool>,
    row_scratch: Vec<u8>,
}

impl GraphicsCaptureApiHandler for WindowCaptureHandler {
    type Flags = WindowCaptureFlags;
    type Error = anyhow::Error;

    fn new(context: CaptureContext<Self::Flags>) -> Result<Self, Self::Error> {
        Ok(Self {
            video: context.flags.video,
            metrics: context.flags.metrics,
            stop: context.flags.stop,
            sensitive_screen: context.flags.sensitive_screen,
            row_scratch: Vec::new(),
        })
    }

    fn on_frame_arrived(
        &mut self,
        frame: &mut CapturedWindowFrame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        if self.stop.load(Ordering::Relaxed) {
            capture_control.stop();
            return Ok(());
        }
        if self.sensitive_screen.load(Ordering::Relaxed) {
            return Ok(());
        }
        let capture_started = Instant::now();
        let buffer = frame
            .buffer_without_title_bar()
            .context("could not map the captured game window")?;
        let width = buffer.width();
        let height = buffer.height();
        let pixels = buffer.as_nopadding_buffer(&mut self.row_scratch).to_vec();
        self.metrics
            .observe_capture_latency(capture_started.elapsed());
        let _ = publish_latest_video(
            &self.video,
            &self.stop,
            &self.metrics,
            VideoFrame {
                width,
                height,
                pixel_format: VideoPixelFormat::Bgra,
                pixels,
            },
        );
        Ok(())
    }

    fn on_closed(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn start_window_capture(
    window_title: &str,
    frames_per_second: u8,
    video: Arc<Mutex<Option<VideoFrame>>>,
    metrics: Arc<BroadcastMetrics>,
    stop: Arc<AtomicBool>,
    sensitive_screen: Arc<AtomicBool>,
    events: mpsc::Sender<WorkerEvent>,
) -> Result<()> {
    let window = CapturableWindow::from_name(window_title)
        .with_context(|| format!("could not find the game window '{window_title}' for capture"))?;
    let settings = CaptureSettings::new(
        window,
        CursorCaptureSettings::WithoutCursor,
        DrawBorderSettings::WithoutBorder,
        SecondaryWindowSettings::Exclude,
        MinimumUpdateIntervalSettings::Custom(Duration::from_secs_f64(
            1.0 / f64::from(frames_per_second),
        )),
        DirtyRegionSettings::Default,
        ColorFormat::Bgra8,
        WindowCaptureFlags {
            video,
            metrics,
            stop: Arc::clone(&stop),
            sensitive_screen,
        },
    );
    thread::Builder::new()
        .name("stream-town-window-capture".to_owned())
        .spawn(move || {
            if let Err(error) = WindowCaptureHandler::start(settings)
                && !stop.load(Ordering::Relaxed)
            {
                stop.store(true, Ordering::Relaxed);
                let _ = events.send(WorkerEvent::Error(format!(
                    "game-window capture stopped: {error}"
                )));
            }
        })
        .context("failed to start Windows Graphics Capture")?;
    Ok(())
}

fn publish_latest_video(
    video: &Mutex<Option<VideoFrame>>,
    stop: &AtomicBool,
    metrics: &BroadcastMetrics,
    frame: VideoFrame,
) -> bool {
    if stop.load(Ordering::Relaxed) {
        metrics.dropped_video.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    let Ok(mut latest) = video.lock() else {
        metrics.dropped_video.fetch_add(1, Ordering::Relaxed);
        return false;
    };
    if latest.replace(frame).is_some() {
        metrics.replaced_video.fetch_add(1, Ordering::Relaxed);
        metrics.dropped_video.fetch_add(1, Ordering::Relaxed);
    }
    metrics.captured_video.fetch_add(1, Ordering::Relaxed);
    true
}

struct BroadcastController {
    audio: SyncSender<AudioInput>,
    video: Arc<Mutex<Option<VideoFrame>>>,
    events: Arc<Mutex<Receiver<WorkerEvent>>>,
    stop: Arc<AtomicBool>,
    sensitive_screen: Arc<AtomicBool>,
    metrics: Arc<BroadcastMetrics>,
}

impl BroadcastController {
    fn start(
        target: BroadcastTarget,
        config: BroadcastConfig,
        window_title: String,
    ) -> Result<Self> {
        let (audio, receiver) = mpsc::sync_channel(AUDIO_QUEUE_CAPACITY);
        let (event_sender, event_receiver) = mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let metrics = Arc::new(BroadcastMetrics::default());
        let video = Arc::new(Mutex::new(None));
        let sensitive_screen = Arc::new(AtomicBool::new(false));
        let capture_fps = config.frames_per_second;
        let stream_only = config.render_mode == BroadcastRenderMode::StreamOnly;

        let worker_stop = Arc::clone(&stop);
        let worker_metrics = Arc::clone(&metrics);
        let worker_video = Arc::clone(&video);
        let audio_event_sender = event_sender.clone();
        let capture_event_sender = event_sender.clone();
        thread::Builder::new()
            .name("stream-town-ffmpeg".to_owned())
            .spawn(move || {
                run_broadcast_worker(
                    target,
                    config,
                    receiver,
                    worker_video,
                    &event_sender,
                    &worker_stop,
                    &worker_metrics,
                );
            })
            .context("failed to start the in-process FFmpeg worker")?;

        if !stream_only
            && let Err(error) = start_window_capture(
                &window_title,
                capture_fps,
                Arc::clone(&video),
                Arc::clone(&metrics),
                Arc::clone(&stop),
                Arc::clone(&sensitive_screen),
                capture_event_sender,
            )
        {
            stop.store(true, Ordering::Relaxed);
            return Err(error);
        }

        let audio_sender = audio.clone();
        let audio_stop = Arc::clone(&stop);
        let audio_metrics = Arc::clone(&metrics);
        let audio_events = Arc::new(Mutex::new(event_receiver));
        let audio_spawn = thread::Builder::new()
            .name("stream-town-wasapi".to_owned())
            .spawn(move || {
                if let Err(error) = capture_process_audio(audio_sender, &audio_stop, &audio_metrics)
                {
                    audio_stop.store(true, Ordering::Relaxed);
                    let message = format!("game-process audio capture stopped: {error:#}");
                    error!(%error, "game-process audio capture stopped");
                    let _ = audio_event_sender.send(WorkerEvent::Error(message));
                }
            });
        if let Err(error) = audio_spawn {
            stop.store(true, Ordering::Relaxed);
            let _ = audio.try_send(AudioInput::Stop);
            return Err(anyhow!(
                "failed to start WASAPI game-audio capture: {error}"
            ));
        }

        Ok(Self {
            audio,
            video,
            events: audio_events,
            stop,
            sensitive_screen,
            metrics,
        })
    }

    fn send_video(&self, frame: VideoFrame) -> bool {
        publish_latest_video(&self.video, &self.stop, &self.metrics, frame)
    }

    fn drop_video_frames(&self, count: u64) {
        self.metrics
            .dropped_video
            .fetch_add(count, Ordering::Relaxed);
    }

    fn set_sensitive_screen(&self, active: bool) {
        self.sensitive_screen.store(active, Ordering::Relaxed);
    }

    fn events(&self) -> Vec<WorkerEvent> {
        self.events
            .lock()
            .map(|receiver| receiver.try_iter().collect())
            .unwrap_or_default()
    }

    fn metrics(&self) -> BroadcastMetricsSnapshot {
        self.metrics.snapshot()
    }

    fn request_stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
        let _ = self.audio.try_send(AudioInput::Stop);
    }
}

impl Drop for BroadcastController {
    fn drop(&mut self) {
        self.request_stop();
    }
}

fn capture_process_audio(
    audio: SyncSender<AudioInput>,
    stop: &AtomicBool,
    metrics: &BroadcastMetrics,
) -> Result<()> {
    initialize_mta()
        .ok()
        .map_err(|error| anyhow!("could not initialize Windows audio COM: {error}"))?;
    let format = WaveFormat::new(
        32,
        32,
        &SampleType::Float,
        AUDIO_SAMPLE_RATE as usize,
        AUDIO_CHANNELS,
        None,
    );
    let block_align = usize::try_from(format.get_blockalign())
        .context("WASAPI block alignment does not fit in memory")?;
    let mut client = AudioClient::new_application_loopback_client(std::process::id(), true)
        .map_err(|error| anyhow!("could not create process-scoped WASAPI loopback: {error}"))?;
    client
        .initialize_client(
            &format,
            &Direction::Capture,
            &StreamMode::EventsShared {
                autoconvert: true,
                buffer_duration_hns: 0,
            },
        )
        .map_err(|error| anyhow!("could not initialize process-scoped WASAPI loopback: {error}"))?;
    let event = client
        .set_get_eventhandle()
        .map_err(|error| anyhow!("could not create WASAPI event: {error}"))?;
    let capture = client
        .get_audiocaptureclient()
        .map_err(|error| anyhow!("could not get WASAPI capture client: {error}"))?;
    let mut bytes = VecDeque::new();
    let chunk_bytes = AUDIO_FRAME_SAMPLES * block_align;
    let mut pts = 0_i64;
    client
        .start_stream()
        .map_err(|error| anyhow!("could not start game-process audio capture: {error}"))?;

    while !stop.load(Ordering::Relaxed) {
        let _ = event.wait_for_event(250);
        while capture
            .get_next_packet_size()
            .map_err(|error| anyhow!("could not query WASAPI capture packet: {error}"))?
            .unwrap_or(0)
            > 0
        {
            capture
                .read_from_device_to_deque(&mut bytes)
                .map_err(|error| anyhow!("could not read game-process audio: {error}"))?;
        }
        while bytes.len() >= chunk_bytes {
            let raw = bytes.drain(..chunk_bytes).collect::<Vec<_>>();
            let samples = raw
                .chunks_exact(4)
                .map(|sample| f32::from_le_bytes([sample[0], sample[1], sample[2], sample[3]]))
                .collect::<Vec<_>>();
            if !queue_audio_frame(&audio, metrics, AudioFrame { pts, samples }) {
                let _ = client.stop_stream();
                return Ok(());
            }
            pts = pts.saturating_add(i64::try_from(AUDIO_FRAME_SAMPLES).unwrap_or(i64::MAX));
        }
    }
    let _ = client.stop_stream();
    Ok(())
}

fn queue_audio_frame(
    audio: &SyncSender<AudioInput>,
    metrics: &BroadcastMetrics,
    frame: AudioFrame,
) -> bool {
    let depth = metrics.queued_audio.fetch_add(1, Ordering::Relaxed) + 1;
    metrics
        .audio_queue_high_water
        .fetch_max(depth, Ordering::Relaxed);
    match audio.try_send(AudioInput::Frame(frame)) {
        Ok(()) => true,
        Err(TrySendError::Full(_)) => {
            metrics.queued_audio.fetch_sub(1, Ordering::Relaxed);
            metrics.dropped_audio.fetch_add(1, Ordering::Relaxed);
            true
        }
        Err(TrySendError::Disconnected(_)) => {
            metrics.queued_audio.fetch_sub(1, Ordering::Relaxed);
            false
        }
    }
}

fn run_broadcast_worker(
    target: BroadcastTarget,
    config: BroadcastConfig,
    receiver: Receiver<AudioInput>,
    video: Arc<Mutex<Option<VideoFrame>>>,
    events: &mpsc::Sender<WorkerEvent>,
    stop: &AtomicBool,
    metrics: &BroadcastMetrics,
) {
    let mut reconnect_delay = 1_u64;
    loop {
        if stop.load(Ordering::Relaxed) {
            let _ = events.send(WorkerEvent::Stopped);
            return;
        }
        let _ = events.send(WorkerEvent::Connecting);
        match encode_broadcast_session(&target, &config, &receiver, &video, stop, metrics, events) {
            Ok(SessionEnd::Stopped | SessionEnd::InputClosed) => {
                let _ = events.send(WorkerEvent::Stopped);
                return;
            }
            Err(error) => {
                let message = format!("{error:#}");
                let _ = events.send(WorkerEvent::Reconnecting(message));
                for _ in 0..reconnect_delay.saturating_mul(4) {
                    if stop.load(Ordering::Relaxed) {
                        let _ = events.send(WorkerEvent::Stopped);
                        return;
                    }
                    thread::sleep(Duration::from_millis(250));
                }
                reconnect_delay = (reconnect_delay * 2).min(MAX_RECONNECT_DELAY_SECONDS);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SessionEnd {
    Stopped,
    InputClosed,
}

#[derive(Debug)]
struct VideoCadence {
    frame_period: Duration,
    next_deadline: Option<Instant>,
    next_pts: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CadenceTick {
    pts: i64,
    skipped: u64,
}

impl VideoCadence {
    fn new(frames_per_second: u8) -> Self {
        Self {
            frame_period: Duration::from_secs_f64(1.0 / f64::from(frames_per_second)),
            next_deadline: None,
            next_pts: 0,
        }
    }

    fn start(&mut self, now: Instant) {
        self.next_deadline.get_or_insert(now);
    }

    fn take_due_tick(&mut self, now: Instant) -> Option<CadenceTick> {
        let deadline = self.next_deadline.as_mut()?;
        if now < *deadline {
            return None;
        }
        let overdue = now.saturating_duration_since(*deadline);
        let period_nanos = self.frame_period.as_nanos().max(1);
        let due_slots = 1_u128.saturating_add(overdue.as_nanos() / period_nanos);
        let due_slots_u64 = u64::try_from(due_slots).unwrap_or(u64::MAX);
        let skipped = due_slots_u64.saturating_sub(1);
        let pts = self
            .next_pts
            .saturating_add(i64::try_from(skipped).unwrap_or(i64::MAX));
        self.next_pts = self
            .next_pts
            .saturating_add(i64::try_from(due_slots_u64).unwrap_or(i64::MAX));
        let advance = u32::try_from(due_slots_u64).unwrap_or(u32::MAX);
        *deadline += self.frame_period.saturating_mul(advance);
        Some(CadenceTick { pts, skipped })
    }

    fn receive_timeout(&self, now: Instant) -> Duration {
        self.next_deadline
            .map_or(Duration::from_millis(250), |deadline| {
                deadline.saturating_duration_since(now)
            })
    }
}

struct BroadcastEncoder {
    output: format::context::Output,
    video: encoder::video::Encoder,
    audio: encoder::audio::Encoder,
    video_stream: usize,
    audio_stream: usize,
    video_time_base: Rational,
    audio_time_base: Rational,
    scaler: Option<(u32, u32, ffmpeg::format::Pixel, software::scaling::Context)>,
    resampler: software::resampling::Context,
    width: u32,
    height: u32,
    video_input_format: ffmpeg::format::Pixel,
    audio_pts_base: Option<i64>,
}

fn encode_broadcast_session(
    target: &BroadcastTarget,
    config: &BroadcastConfig,
    receiver: &Receiver<AudioInput>,
    video_mailbox: &Mutex<Option<VideoFrame>>,
    stop: &AtomicBool,
    metrics: &BroadcastMetrics,
    events: &mpsc::Sender<WorkerEvent>,
) -> Result<SessionEnd> {
    ffmpeg::init().context("could not initialize the linked FFmpeg libraries")?;
    ffmpeg::log::set_level(ffmpeg::log::Level::Quiet);
    let (mut encoder, encoder_name) = BroadcastEncoder::open(target, config)?;
    if discard_pending_audio(receiver, metrics) {
        encoder.finish()?;
        return Ok(SessionEnd::Stopped);
    }
    let _ = events.send(WorkerEvent::Broadcasting {
        encoder: encoder_name,
    });
    let mut cadence = VideoCadence::new(config.frames_per_second);
    let mut latest_video = take_latest_video(video_mailbox);
    if latest_video.is_some() {
        cadence.start(Instant::now());
    }
    loop {
        if stop.load(Ordering::Relaxed) {
            encoder.finish()?;
            return Ok(SessionEnd::Stopped);
        }
        if let Some(video) = take_latest_video(video_mailbox) {
            cadence.start(Instant::now());
            latest_video = Some(video);
        }
        if let Some((video, tick)) = latest_video.as_ref().and_then(|video| {
            cadence
                .take_due_tick(Instant::now())
                .map(|tick| (video, tick))
        }) {
            if tick.skipped > 0 {
                metrics
                    .skipped_video
                    .fetch_add(tick.skipped, Ordering::Relaxed);
                metrics
                    .dropped_video
                    .fetch_add(tick.skipped, Ordering::Relaxed);
            }
            let encode_started = Instant::now();
            encoder.encode_video(video, tick.pts)?;
            metrics.observe_video_encode_latency(encode_started.elapsed());
            metrics.encoded_video.fetch_add(1, Ordering::Relaxed);
            continue;
        }
        match receiver.recv_timeout(cadence.receive_timeout(Instant::now())) {
            Ok(AudioInput::Frame(audio)) => {
                metrics.queued_audio.fetch_sub(1, Ordering::Relaxed);
                // Establish both media timelines at the first video frame. This
                // avoids publishing an audio lead while the first GPU readback
                // is still pending, then keeps audio continuous while the
                // cadence worker repeats the latest image through game stalls.
                if latest_video.is_some() {
                    encoder.encode_audio(audio)?;
                    metrics.encoded_audio.fetch_add(1, Ordering::Relaxed);
                }
            }
            Ok(AudioInput::Stop) => {
                encoder.finish()?;
                return Ok(SessionEnd::Stopped);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                encoder.finish()?;
                return Ok(SessionEnd::InputClosed);
            }
        }
    }
}

fn take_latest_video(mailbox: &Mutex<Option<VideoFrame>>) -> Option<VideoFrame> {
    mailbox.lock().ok().and_then(|mut latest| latest.take())
}

fn discard_pending_audio(receiver: &Receiver<AudioInput>, metrics: &BroadcastMetrics) -> bool {
    let mut stopped = false;
    for input in receiver.try_iter() {
        match input {
            AudioInput::Frame(_) => {
                metrics.queued_audio.fetch_sub(1, Ordering::Relaxed);
                metrics.dropped_audio.fetch_add(1, Ordering::Relaxed);
            }
            AudioInput::Stop => stopped = true,
        }
    }
    stopped
}

impl BroadcastEncoder {
    fn open(target: &BroadcastTarget, config: &BroadcastConfig) -> Result<(Self, String)> {
        let mut output_options = Dictionary::new();
        // FFmpeg protocol timeout is in microseconds. Keep a dead ingest from
        // pinning the encoder worker indefinitely; reconnect owns the retry.
        output_options.set("rw_timeout", "15000000");
        let mut output = format::output_as_with(&target.url, "flv", output_options)
            .context("could not connect to the selected Twitch RTMP ingest")?;
        let global_header = output
            .format()
            .flags()
            .contains(format::Flags::GLOBAL_HEADER);
        let (video, video_codec, encoder_name, video_input_format) =
            open_video_encoder(config, global_header)?;
        let (audio, audio_codec) = open_audio_encoder(config, global_header)?;

        let video_stream = {
            let mut stream = output
                .add_stream(video_codec)
                .context("could not add the H.264 stream to the FLV muxer")?;
            stream.set_time_base((1, i32::from(config.frames_per_second)));
            stream.set_rate((i32::from(config.frames_per_second), 1));
            stream.set_parameters(&video);
            stream.index()
        };
        let audio_stream = {
            let mut stream = output
                .add_stream(audio_codec)
                .context("could not add the AAC stream to the FLV muxer")?;
            stream.set_time_base((1, i32::try_from(AUDIO_SAMPLE_RATE).unwrap_or(i32::MAX)));
            stream.set_parameters(&audio);
            stream.index()
        };
        output
            .write_header()
            .context("Twitch rejected the FLV stream header")?;
        let video_time_base = output
            .stream(video_stream)
            .context("FLV video stream disappeared after header write")?
            .time_base();
        let audio_time_base = output
            .stream(audio_stream)
            .context("FLV audio stream disappeared after header write")?
            .time_base();
        let resampler = software::resampling::Context::get(
            ffmpeg::format::Sample::F32(ffmpeg::format::sample::Type::Packed),
            ChannelLayout::STEREO,
            AUDIO_SAMPLE_RATE,
            audio.format(),
            audio.channel_layout(),
            audio.rate(),
        )
        .context("could not initialize the broadcast audio resampler")?;
        Ok((
            Self {
                output,
                video,
                audio,
                video_stream,
                audio_stream,
                video_time_base,
                audio_time_base,
                scaler: None,
                resampler,
                width: u32::from(config.width),
                height: u32::from(config.height),
                video_input_format,
                audio_pts_base: None,
            },
            encoder_name,
        ))
    }

    fn encode_video(&mut self, video: &VideoFrame, pts: i64) -> Result<()> {
        let source_format = video.pixel_format.ffmpeg();
        let mut source = frame::Video::new(source_format, video.width, video.height);
        copy_packed_video_frame(video, &mut source)?;
        if source_format == self.video_input_format
            && video.width == self.width
            && video.height == self.height
        {
            source.set_pts(Some(pts));
            self.video
                .send_frame(&source)
                .context("H.264 encoder rejected a packed frame")?;
            return self.drain_video();
        }
        if self
            .scaler
            .as_ref()
            .is_none_or(|(width, height, format, _)| {
                *width != video.width || *height != video.height || *format != source_format
            })
        {
            let scaler = software::scaling::Context::get(
                source_format,
                video.width,
                video.height,
                self.video_input_format,
                self.width,
                self.height,
                software::scaling::Flags::FAST_BILINEAR,
            )
            .context("could not initialize the broadcast video scaler")?;
            self.scaler = Some((video.width, video.height, source_format, scaler));
        }
        let mut converted = frame::Video::new(self.video_input_format, self.width, self.height);
        self.scaler
            .as_mut()
            .context("broadcast scaler was not initialized")?
            .3
            .run(&source, &mut converted)
            .context("could not scale a broadcast frame")?;
        converted.set_pts(Some(pts));
        self.video
            .send_frame(&converted)
            .context("H.264 encoder rejected a frame")?;
        self.drain_video()
    }

    fn encode_audio(&mut self, audio: AudioFrame) -> Result<()> {
        if audio.samples.len() != AUDIO_FRAME_SAMPLES * AUDIO_CHANNELS {
            bail!("WASAPI returned an incomplete audio frame");
        }
        let audio_pts_base = *self.audio_pts_base.get_or_insert(audio.pts);
        let pts = audio.pts.saturating_sub(audio_pts_base);
        let mut input = frame::Audio::new(
            ffmpeg::format::Sample::F32(ffmpeg::format::sample::Type::Packed),
            AUDIO_FRAME_SAMPLES,
            ChannelLayout::STEREO,
        );
        input.set_rate(AUDIO_SAMPLE_RATE);
        for (target, sample) in input
            .data_mut(0)
            .chunks_exact_mut(std::mem::size_of::<f32>())
            .zip(&audio.samples)
        {
            target.copy_from_slice(&sample.to_le_bytes());
        }
        input.set_pts(Some(pts));
        let mut converted = frame::Audio::empty();
        self.resampler
            .run(&input, &mut converted)
            .context("could not resample game audio for AAC")?;
        converted.set_pts(Some(pts));
        self.audio
            .send_frame(&converted)
            .context("AAC encoder rejected an audio frame")?;
        self.drain_audio()
    }

    fn drain_video(&mut self) -> Result<()> {
        let mut packet = Packet::empty();
        while self.video.receive_packet(&mut packet).is_ok() {
            packet.set_stream(self.video_stream);
            packet.rescale_ts(self.video.time_base(), self.video_time_base);
            packet
                .write_interleaved(&mut self.output)
                .context("could not publish an H.264 packet to Twitch")?;
        }
        Ok(())
    }

    fn drain_audio(&mut self) -> Result<()> {
        let mut packet = Packet::empty();
        while self.audio.receive_packet(&mut packet).is_ok() {
            packet.set_stream(self.audio_stream);
            packet.rescale_ts(self.audio.time_base(), self.audio_time_base);
            packet
                .write_interleaved(&mut self.output)
                .context("could not publish an AAC packet to Twitch")?;
        }
        Ok(())
    }

    fn finish(&mut self) -> Result<()> {
        self.video.send_eof().ok();
        self.drain_video()?;
        self.audio.send_eof().ok();
        self.drain_audio()?;
        self.output
            .write_trailer()
            .context("could not finish the Twitch FLV stream")
    }
}

fn copy_packed_video_frame(video: &VideoFrame, target: &mut frame::Video) -> Result<()> {
    let source_stride = usize::try_from(video.width)
        .context("broadcast frame width does not fit in memory")?
        .saturating_mul(4);
    let expected = source_stride.saturating_mul(
        usize::try_from(video.height).context("broadcast frame height does not fit in memory")?,
    );
    if video.pixels.len() != expected {
        bail!(
            "broadcast frame has {} bytes, expected {expected}",
            video.pixels.len()
        );
    }
    let target_stride = target.stride(0);
    for (row, source) in video.pixels.chunks_exact(source_stride).enumerate() {
        let start = row.saturating_mul(target_stride);
        target.data_mut(0)[start..start + source_stride].copy_from_slice(source);
    }
    Ok(())
}

fn open_video_encoder(
    config: &BroadcastConfig,
    global_header: bool,
) -> Result<(
    encoder::video::Encoder,
    Codec,
    String,
    ffmpeg::format::Pixel,
)> {
    let mut failures = Vec::new();
    for &name in encoder_candidates(config.encoder) {
        let Some(codec) = encoder::find_by_name(name) else {
            continue;
        };
        let input_format = encoder_input_format(name);
        let outcome = (|| -> Result<encoder::video::Encoder> {
            let mut video = codec::context::Context::new_with_codec(codec)
                .encoder()
                .video()
                .context("encoder is not a video encoder")?;
            video.set_width(u32::from(config.width));
            video.set_height(u32::from(config.height));
            video.set_format(input_format);
            video.set_time_base((1, i32::from(config.frames_per_second)));
            video.set_frame_rate(Some((i32::from(config.frames_per_second), 1)));
            video.set_bit_rate(config.video_bitrate_kbps as usize * 1_000);
            video.set_max_bit_rate(config.video_bitrate_kbps as usize * 1_000);
            video.set_gop(u32::from(config.frames_per_second) * 2);
            video.set_max_b_frames(0);
            if global_header {
                video.set_flags(codec::Flags::GLOBAL_HEADER);
            }
            let mut options = Dictionary::new();
            options.set("forced-idr", "1");
            match name {
                "h264_nvenc" => {
                    options.set("profile", "high");
                    options.set("preset", "p2");
                    options.set("tune", "ull");
                    options.set("rc", "cbr");
                    options.set("rc-lookahead", "0");
                    options.set("multipass", "disabled");
                    options.set("zerolatency", "1");
                }
                "h264_qsv" => {
                    options.set("profile", "high");
                    options.set("preset", "veryfast");
                    options.set("look_ahead", "0");
                    options.set("async_depth", "2");
                    options.set("scenario", "livestreaming");
                }
                "h264_amf" => {
                    options.set("profile", "high");
                    options.set("usage", "ultralowlatency");
                    options.set("quality", "speed");
                    options.set("rc", "cbr");
                    options.set("latency", "1");
                    options.set("async_depth", "4");
                }
                "h264_mf" => {
                    options.set("rate_control", "cbr");
                    options.set("scenario", "live_streaming");
                    options.set("hw_encoding", "1");
                }
                "libopenh264" => options.set("profile", "high"),
                _ => {}
            }
            video
                .open_as_with(codec, options)
                .with_context(|| format!("could not open {name}"))
        })();
        match outcome {
            Ok(video) => return Ok((video, codec, name.to_owned(), input_format)),
            Err(error) => failures.push(format!("{name}: {error:#}")),
        }
    }
    bail!(
        "no requested H.264 encoder could be opened ({})",
        failures.join("; ")
    )
}

fn encoder_input_format(name: &str) -> ffmpeg::format::Pixel {
    match name {
        "h264_nvenc" | "h264_amf" => ffmpeg::format::Pixel::BGRA,
        "h264_qsv" | "h264_mf" => ffmpeg::format::Pixel::NV12,
        _ => ffmpeg::format::Pixel::YUV420P,
    }
}

fn open_audio_encoder(
    config: &BroadcastConfig,
    global_header: bool,
) -> Result<(encoder::audio::Encoder, Codec)> {
    let codec = encoder::find_by_name("aac").context("linked FFmpeg has no AAC encoder")?;
    let audio_codec = codec
        .audio()
        .context("FFmpeg AAC codec is not an audio encoder")?;
    let sample_format = audio_codec
        .formats()
        .and_then(|mut formats| formats.next())
        .context("FFmpeg AAC encoder reports no supported sample formats")?;
    let mut audio = codec::context::Context::new_with_codec(codec)
        .encoder()
        .audio()
        .context("could not construct the AAC encoder")?;
    audio.set_rate(i32::try_from(AUDIO_SAMPLE_RATE).unwrap_or(i32::MAX));
    audio.set_channel_layout(ChannelLayout::STEREO);
    audio.set_format(sample_format);
    audio.set_bit_rate(usize::from(config.audio_bitrate_kbps) * 1_000);
    audio.set_time_base((1, i32::try_from(AUDIO_SAMPLE_RATE).unwrap_or(i32::MAX)));
    if global_header {
        audio.set_flags(codec::Flags::GLOBAL_HEADER);
    }
    let audio = audio
        .open_as(codec)
        .context("could not open the FFmpeg AAC encoder")?;
    Ok((audio, codec))
}

fn encoder_candidates(preference: BroadcastEncoderPreference) -> &'static [&'static str] {
    match preference {
        BroadcastEncoderPreference::Auto => &[
            "h264_nvenc",
            "h264_qsv",
            "h264_amf",
            "h264_mf",
            "libopenh264",
        ],
        BroadcastEncoderPreference::Nvidia => &["h264_nvenc"],
        BroadcastEncoderPreference::Intel => &["h264_qsv"],
        BroadcastEncoderPreference::Amd => &["h264_amf"],
        BroadcastEncoderPreference::MediaFoundation => &["h264_mf"],
        BroadcastEncoderPreference::OpenH264 => &["libopenh264"],
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BroadcastPrerequisites {
    pub available_encoders: Vec<String>,
    pub selected_encoder: String,
    pub process_audio_capture_available: bool,
}

pub fn inspect_broadcast_prerequisites(config: &BroadcastConfig) -> Result<BroadcastPrerequisites> {
    ffmpeg::init().context("could not initialize the linked FFmpeg libraries")?;
    ffmpeg::log::set_level(ffmpeg::log::Level::Quiet);
    let available_encoders = [
        "h264_nvenc",
        "h264_qsv",
        "h264_amf",
        "h264_mf",
        "libopenh264",
    ]
    .into_iter()
    .filter(|name| encoder::find_by_name(name).is_some())
    .map(ToOwned::to_owned)
    .collect::<Vec<_>>();
    if available_encoders.is_empty() {
        bail!("linked FFmpeg libraries contain no supported H.264 encoder");
    }
    let output = std::env::temp_dir().join(format!(
        "stream-town-broadcast-diagnostic-{}.flv",
        std::process::id()
    ));
    let target = BroadcastTarget {
        ingest_name: "local-diagnostic".to_owned(),
        url: output.to_string_lossy().into_owned(),
    };
    let (mut encoder, selected_encoder) = BroadcastEncoder::open(&target, config)
        .context("the configured H.264/AAC encoder could not open for a local FLV test")?;
    let rgba = vec![
        0_u8;
        usize::from(config.width)
            .saturating_mul(usize::from(config.height))
            .saturating_mul(4)
    ];
    for pts in 0..2_i64 {
        encoder.encode_video(
            &VideoFrame {
                width: u32::from(config.width),
                height: u32::from(config.height),
                pixel_format: VideoPixelFormat::Rgba,
                pixels: rgba.clone(),
            },
            pts,
        )?;
        encoder.encode_audio(AudioFrame {
            pts: pts * i64::try_from(AUDIO_FRAME_SAMPLES).unwrap_or(i64::MAX),
            samples: vec![0.0; AUDIO_FRAME_SAMPLES * AUDIO_CHANNELS],
        })?;
    }
    encoder
        .finish()
        .context("the configured encoder could not finish a local FLV test")?;
    let encoded_bytes = std::fs::metadata(&output)
        .with_context(|| format!("local encoder test did not create {}", output.display()))?
        .len();
    let _ = std::fs::remove_file(&output);
    if encoded_bytes < 512 {
        bail!("local H.264/AAC encoder test produced an empty FLV stream");
    }
    let _ = initialize_mta().ok();
    let process_audio_capture_available =
        AudioClient::new_application_loopback_client(std::process::id(), true).is_ok();
    Ok(BroadcastPrerequisites {
        available_encoders,
        selected_encoder,
        process_audio_capture_available,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ingest(name: &str, is_default: bool, priority: u32) -> TwitchIngest {
        TwitchIngest {
            name: name.to_owned(),
            url_template: "rtmp://example.invalid/app/{stream_key}".to_owned(),
            priority,
            availability: 1.0,
            is_default,
        }
    }

    #[test]
    fn ingest_selection_prefers_default_or_named_region() {
        let ingests = [ingest("Sydney", false, 2), ingest("Adelaide", true, 1)];
        assert_eq!(select_ingest(&ingests, "").unwrap().name, "Adelaide");
        assert_eq!(select_ingest(&ingests, "syd").unwrap().name, "Sydney");
        assert!(select_ingest(&ingests, "nowhere").is_err());
    }

    #[test]
    fn bandwidth_test_url_is_constructed_without_logging_the_key() {
        let key = "live_secret_key";
        let url = build_ingest_url("rtmp://example.invalid/app/{stream_key}", key, true).unwrap();
        assert_eq!(
            url,
            "rtmp://example.invalid/app/live_secret_key?bandwidthtest=true"
        );
        let target = BroadcastTarget {
            ingest_name: "test".to_owned(),
            url,
        };
        assert!(!format!("{target:?}").contains(key));
    }

    #[test]
    fn sensitive_screen_frame_is_opaque_black_when_too_small_for_the_notice() {
        let frame = sensitive_rgba_frame(2, 3);
        assert_eq!(frame.len(), 24);
        for pixel in frame.chunks_exact(4) {
            assert_eq!(pixel, [0, 0, 0, 255]);
        }
    }

    #[test]
    fn sensitive_screen_frame_centres_a_white_privacy_notice() {
        let width = 640;
        let height = 360;
        let frame = sensitive_rgba_frame(width, height);
        assert_eq!(frame.len(), usize::try_from(width * height * 4).unwrap());
        assert_eq!(&frame[..4], &[0, 0, 0, 255]);
        let white_pixels = frame
            .chunks_exact(4)
            .filter(|pixel| *pixel == [255, 255, 255, 255])
            .count();
        assert!(
            white_pixels > 500,
            "privacy notice must be visibly rendered"
        );
        let centre_band_start = usize::try_from((height / 3) * width * 4).unwrap();
        let centre_band_end = usize::try_from((height * 2 / 3) * width * 4).unwrap();
        assert!(
            frame[centre_band_start..centre_band_end]
                .chunks_exact(4)
                .any(|pixel| pixel == [255, 255, 255, 255])
        );
    }

    #[test]
    fn direct_broadcast_stays_offline_until_operator_requests_it() {
        let mut config = stream_town_domain::GameConfig::default();
        config.twitch.broadcast.enabled = true;
        config.twitch.broadcast.start_on_launch = true;
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(RuntimeConfig(config))
            .init_resource::<SensitiveScreenActive>()
            .add_plugins(DirectTwitchBroadcastPlugin);
        app.update();
        assert_eq!(
            app.world().resource::<DirectBroadcastRuntime>().phase,
            DirectBroadcastPhase::Disabled
        );
    }

    #[test]
    fn prepared_broadcast_waits_for_the_truthful_gameplay_ready_gate() {
        assert!(!prepared_broadcast_can_start(
            &DirectBroadcastPhase::WaitingForGameplay,
            false,
        ));
        assert!(prepared_broadcast_can_start(
            &DirectBroadcastPhase::WaitingForGameplay,
            true,
        ));
        assert!(!prepared_broadcast_can_start(
            &DirectBroadcastPhase::Connecting,
            true,
        ));
    }

    #[test]
    fn active_phase_contract_covers_every_session_that_locks_streaming_settings() {
        for phase in [
            DirectBroadcastPhase::WaitingForBroadcasterAuthorization,
            DirectBroadcastPhase::WaitingForGameplay,
            DirectBroadcastPhase::ResolvingIngest,
            DirectBroadcastPhase::Connecting,
            DirectBroadcastPhase::Broadcasting,
            DirectBroadcastPhase::Reconnecting,
            DirectBroadcastPhase::Stopping,
        ] {
            assert!(phase.is_active(), "{phase:?} must remain operator-active");
        }
        for phase in [
            DirectBroadcastPhase::Disabled,
            DirectBroadcastPhase::Stopped,
            DirectBroadcastPhase::Error("test".to_owned()),
        ] {
            assert!(!phase.is_active(), "{phase:?} must be operator-inactive");
        }
    }

    #[test]
    fn gpu_readback_padding_is_removed_without_corrupting_rows() {
        let width = 65_u32;
        let height = 2_u32;
        let row_bytes = usize::try_from(width).unwrap() * 4;
        let aligned_row_bytes = row_bytes.div_ceil(256) * 256;
        let mut padded = vec![0xEE; aligned_row_bytes * usize::try_from(height).unwrap()];
        padded[..row_bytes].fill(0x11);
        padded[aligned_row_bytes..aligned_row_bytes + row_bytes].fill(0x22);

        let pixels = remove_gpu_row_padding(padded, width, height);

        assert_eq!(pixels.len(), row_bytes * usize::try_from(height).unwrap());
        assert!(pixels[..row_bytes].iter().all(|byte| *byte == 0x11));
        assert!(pixels[row_bytes..].iter().all(|byte| *byte == 0x22));
    }

    #[test]
    fn operator_stop_cancels_an_in_flight_session_without_restarting_it() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(RuntimeConfig(stream_town_domain::GameConfig::default()))
            .init_resource::<SensitiveScreenActive>()
            .add_plugins(DirectTwitchBroadcastPlugin);
        app.world_mut()
            .resource_mut::<DirectBroadcastRuntime>()
            .phase = DirectBroadcastPhase::Connecting;
        app.world_mut()
            .resource_mut::<DirectBroadcastControl>()
            .request_stop();

        app.update();

        assert_eq!(
            app.world().resource::<DirectBroadcastRuntime>().phase,
            DirectBroadcastPhase::Stopped
        );
        let control = app.world().resource::<DirectBroadcastControl>();
        assert!(!control.restart_requested);
        assert!(!control.stop_requested);
    }

    #[test]
    fn video_cadence_skips_stale_slots_instead_of_bursting_after_a_stall() {
        let started = Instant::now();
        let mut cadence = VideoCadence::new(30);
        cadence.start(started);
        assert_eq!(
            cadence.take_due_tick(started),
            Some(CadenceTick { pts: 0, skipped: 0 })
        );
        assert_eq!(cadence.take_due_tick(started), None);

        let after_three_periods = started + cadence.frame_period * 3;
        assert_eq!(
            cadence.take_due_tick(after_three_periods),
            Some(CadenceTick { pts: 3, skipped: 2 })
        );
        assert_eq!(cadence.take_due_tick(after_three_periods), None);
        assert_eq!(
            cadence.receive_timeout(after_three_periods),
            cadence.frame_period
        );
    }

    #[test]
    fn auto_encoder_order_prefers_hardware_and_has_lgpl_fallback() {
        assert_eq!(
            encoder_candidates(BroadcastEncoderPreference::Auto),
            [
                "h264_nvenc",
                "h264_qsv",
                "h264_amf",
                "h264_mf",
                "libopenh264"
            ]
        );
    }

    #[test]
    fn hardware_encoders_accept_packed_gpu_readback_without_cpu_yuv_conversion() {
        assert_eq!(
            encoder_input_format("h264_nvenc"),
            ffmpeg::format::Pixel::BGRA
        );
        assert_eq!(
            encoder_input_format("h264_amf"),
            ffmpeg::format::Pixel::BGRA
        );
        assert_eq!(
            encoder_input_format("h264_qsv"),
            ffmpeg::format::Pixel::NV12
        );
        assert_eq!(
            encoder_input_format("libopenh264"),
            ffmpeg::format::Pixel::YUV420P
        );
    }

    #[test]
    fn video_mailbox_replaces_stale_frames_instead_of_building_latency() {
        let mailbox = Mutex::new(None);
        let first = VideoFrame {
            width: 1,
            height: 1,
            pixel_format: VideoPixelFormat::Bgra,
            pixels: vec![1, 2, 3, 4],
        };
        let second = VideoFrame {
            pixels: vec![5, 6, 7, 8],
            ..first.clone()
        };
        mailbox.lock().unwrap().replace(first);
        mailbox.lock().unwrap().replace(second);
        assert_eq!(take_latest_video(&mailbox).unwrap().pixels, [5, 6, 7, 8]);
        assert!(take_latest_video(&mailbox).is_none());
    }

    #[test]
    fn controller_counts_replaced_video_without_rejecting_the_newest_frame() {
        let (audio, _audio_receiver) = mpsc::sync_channel(1);
        let (_event_sender, event_receiver) = mpsc::channel();
        let metrics = Arc::new(BroadcastMetrics::default());
        let controller = BroadcastController {
            audio,
            video: Arc::new(Mutex::new(None)),
            events: Arc::new(Mutex::new(event_receiver)),
            stop: Arc::new(AtomicBool::new(false)),
            sensitive_screen: Arc::new(AtomicBool::new(false)),
            metrics: Arc::clone(&metrics),
        };
        let first = VideoFrame {
            width: 1,
            height: 1,
            pixel_format: VideoPixelFormat::Bgra,
            pixels: vec![1, 2, 3, 4],
        };
        let second = VideoFrame {
            pixels: vec![5, 6, 7, 8],
            ..first.clone()
        };
        assert!(controller.send_video(first));
        assert!(controller.send_video(second));
        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.replaced_video, 1);
        assert_eq!(snapshot.dropped_video, 1);
        assert_eq!(
            take_latest_video(&controller.video).unwrap().pixels,
            [5, 6, 7, 8]
        );
    }

    #[test]
    #[ignore = "local 1080p60 hardware-encoder throughput diagnostic"]
    fn configured_1080p60_encoder_sustains_realtime_output() {
        let directory = tempfile::tempdir().unwrap();
        let output = directory.path().join("direct-broadcast-1080p60.flv");
        let target = BroadcastTarget {
            ingest_name: "local-performance-diagnostic".to_owned(),
            url: output.to_string_lossy().into_owned(),
        };
        let config = BroadcastConfig {
            enabled: true,
            width: 1_920,
            height: 1_080,
            frames_per_second: 60,
            video_bitrate_kbps: 6_000,
            encoder: BroadcastEncoderPreference::Auto,
            ..BroadcastConfig::default()
        };
        ffmpeg::init().unwrap();
        ffmpeg::log::set_level(ffmpeg::log::Level::Quiet);
        let (mut encoder, selected) = BroadcastEncoder::open(&target, &config).unwrap();
        let frame = VideoFrame {
            width: u32::from(config.width),
            height: u32::from(config.height),
            pixel_format: VideoPixelFormat::Bgra,
            pixels: vec![
                0;
                usize::from(config.width)
                    .saturating_mul(usize::from(config.height))
                    .saturating_mul(4)
            ],
        };
        let frame_count = 120_u32;
        let started = Instant::now();
        for pts in 0..frame_count {
            encoder.encode_video(&frame, i64::from(pts)).unwrap();
        }
        encoder.finish().unwrap();
        let elapsed = started.elapsed().as_secs_f64();
        let frames_per_second = f64::from(frame_count) / elapsed;
        eprintln!(
            "1080p60 local encoder diagnostic: {selected}, {frames_per_second:.1} FPS, {:.2} ms/frame",
            elapsed * 1_000.0 / f64::from(frame_count)
        );
        assert!(
            frames_per_second >= 60.0,
            "{selected} only sustained {frames_per_second:.1} FPS"
        );
    }

    #[test]
    fn linked_ffmpeg_encodes_h264_aac_flv_without_a_subprocess() {
        let directory = tempfile::tempdir().unwrap();
        let output = directory.path().join("direct-broadcast-smoke.flv");
        let target = BroadcastTarget {
            ingest_name: "local-file".to_owned(),
            url: output.to_string_lossy().into_owned(),
        };
        let config = BroadcastConfig {
            width: 320,
            height: 180,
            frames_per_second: 30,
            video_bitrate_kbps: 500,
            audio_bitrate_kbps: 96,
            encoder: BroadcastEncoderPreference::OpenH264,
            ..BroadcastConfig::default()
        };
        ffmpeg::init().unwrap();
        ffmpeg::log::set_level(ffmpeg::log::Level::Quiet);
        let (mut encoder, selected) = BroadcastEncoder::open(&target, &config).unwrap();
        assert!(!selected.is_empty());
        let rgba = (0..u32::from(config.height))
            .flat_map(|y| {
                (0..u32::from(config.width)).flat_map(move |x| {
                    [
                        u8::try_from(x * 255 / u32::from(config.width)).unwrap_or(255),
                        u8::try_from(y * 255 / u32::from(config.height)).unwrap_or(255),
                        96,
                        255,
                    ]
                })
            })
            .collect::<Vec<_>>();
        let mut audio_pts = 0_i64;
        for video_pts in 0..15_i64 {
            encoder
                .encode_video(
                    &VideoFrame {
                        width: u32::from(config.width),
                        height: u32::from(config.height),
                        pixel_format: VideoPixelFormat::Rgba,
                        pixels: rgba.clone(),
                    },
                    video_pts,
                )
                .unwrap();
            let samples = vec![0.0; AUDIO_FRAME_SAMPLES * AUDIO_CHANNELS];
            encoder
                .encode_audio(AudioFrame {
                    pts: audio_pts,
                    samples,
                })
                .unwrap();
            audio_pts += i64::try_from(AUDIO_FRAME_SAMPLES).unwrap();
        }
        encoder.finish().unwrap();
        assert!(std::fs::metadata(output).unwrap().len() > 1_024);
    }
}
