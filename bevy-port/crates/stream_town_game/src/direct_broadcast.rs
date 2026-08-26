//! In-process Twitch video broadcast for the Windows build.
//!
//! Frames are read back from Bevy's primary render target, game-process audio
//! is captured with WASAPI application loopback, and dynamically linked `FFmpeg`
//! libraries encode/mux H.264 + AAC into Twitch's RTMP ingest. No subprocess,
//! desktop capture, virtual cable, or OBS installation is involved.

use std::{
    collections::VecDeque,
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
    prelude::*,
    render::{
        render_resource::TextureFormat,
        view::screenshot::{Screenshot, ScreenshotCaptured},
    },
};
use ffmpeg::{
    ChannelLayout, Codec, Dictionary, Packet, Rational, codec, encoder, format, frame, software,
};
use ffmpeg_next as ffmpeg;
use stream_town_domain::{BroadcastConfig, BroadcastEncoderPreference};
use wasapi::{AudioClient, Direction, SampleType, StreamMode, WaveFormat, initialize_mta};

use crate::{
    RuntimeConfig, SensitiveScreenActive, SensitiveScreenUpdateSet,
    twitch::{CredentialVault, OAuthClient, TwitchIngest},
};

const AUDIO_SAMPLE_RATE: u32 = 48_000;
const AUDIO_CHANNELS: usize = 2;
const AUDIO_FRAME_SAMPLES: usize = 1_024;
const MEDIA_QUEUE_CAPACITY: usize = 12;
const MAX_RECONNECT_DELAY_SECONDS: u64 = 30;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DirectBroadcastPhase {
    Disabled,
    WaitingForBroadcasterAuthorization,
    ResolvingIngest,
    Connecting,
    Broadcasting,
    Reconnecting,
    Stopping,
    Stopped,
    Error(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectBroadcastSnapshot {
    pub phase: DirectBroadcastPhase,
    pub encoder: Option<String>,
    pub ingest: Option<String>,
    pub captured_video_frames: u64,
    pub encoded_video_frames: u64,
    pub dropped_video_frames: u64,
    pub encoded_audio_frames: u64,
}

#[derive(Resource)]
pub struct DirectBroadcastRuntime {
    phase: DirectBroadcastPhase,
    encoder: Option<String>,
    ingest: Option<String>,
    authorization: Option<Arc<Mutex<Receiver<AuthorizationEvent>>>>,
    controller: Option<BroadcastController>,
    capture_elapsed: f32,
    capture_in_flight: bool,
    captured_video_frames: u64,
}

impl Default for DirectBroadcastRuntime {
    fn default() -> Self {
        Self {
            phase: DirectBroadcastPhase::Disabled,
            encoder: None,
            ingest: None,
            authorization: None,
            controller: None,
            capture_elapsed: 0.0,
            capture_in_flight: false,
            captured_video_frames: 0,
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
        DirectBroadcastSnapshot {
            phase: self.phase.clone(),
            encoder: self.encoder.clone(),
            ingest: self.ingest.clone(),
            captured_video_frames: self.captured_video_frames,
            encoded_video_frames: metrics.encoded_video,
            dropped_video_frames: metrics.dropped_video,
            encoded_audio_frames: metrics.encoded_audio,
        }
    }
}

#[derive(Resource, Default)]
pub(crate) struct DirectBroadcastControl {
    restart_requested: bool,
}

impl DirectBroadcastControl {
    pub(crate) fn request_restart(&mut self) {
        self.restart_requested = true;
    }
}

pub struct DirectTwitchBroadcastPlugin;

impl Plugin for DirectTwitchBroadcastPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DirectBroadcastRuntime>()
            .init_resource::<DirectBroadcastControl>()
            .add_systems(Startup, begin_direct_broadcast)
            .add_systems(
                Update,
                (
                    restart_direct_broadcast,
                    poll_direct_broadcast_authorization,
                    poll_direct_broadcast_worker,
                    capture_direct_broadcast_frame.after(SensitiveScreenUpdateSet),
                )
                    .chain(),
            );
    }
}

fn begin_direct_broadcast(config: Res<RuntimeConfig>, mut runtime: ResMut<DirectBroadcastRuntime>) {
    configure_direct_broadcast(&config.0, &mut runtime);
}

fn restart_direct_broadcast(
    config: Res<RuntimeConfig>,
    mut control: ResMut<DirectBroadcastControl>,
    mut runtime: ResMut<DirectBroadcastRuntime>,
) {
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
    let twitch = &config.twitch;
    if !twitch.broadcast.enabled || !twitch.broadcast.start_on_launch {
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

fn poll_direct_broadcast_authorization(
    config: Res<RuntimeConfig>,
    mut runtime: ResMut<DirectBroadcastRuntime>,
) {
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
            runtime.phase = DirectBroadcastPhase::Connecting;
            runtime.ingest = Some(target.ingest_name.clone());
            match BroadcastController::start(target, config.0.twitch.broadcast.clone()) {
                Ok(controller) => runtime.controller = Some(controller),
                Err(error) => {
                    runtime.phase = DirectBroadcastPhase::Error(format!("{error:#}"));
                }
            }
        }
        AuthorizationEvent::Error(error) => {
            runtime.phase = DirectBroadcastPhase::Error(error);
        }
    }
}

fn poll_direct_broadcast_worker(mut runtime: ResMut<DirectBroadcastRuntime>) {
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
            }
            WorkerEvent::Reconnecting(error) => {
                runtime.phase = DirectBroadcastPhase::Reconnecting;
                warn!(%error, "direct Twitch broadcast reconnecting");
            }
            WorkerEvent::Stopped => runtime.phase = DirectBroadcastPhase::Stopped,
            WorkerEvent::Error(error) => {
                runtime.phase = DirectBroadcastPhase::Error(error);
            }
        }
    }
}

fn capture_direct_broadcast_frame(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    sensitive_screen: Res<SensitiveScreenActive>,
    mut runtime: ResMut<DirectBroadcastRuntime>,
) {
    if runtime.phase != DirectBroadcastPhase::Broadcasting {
        return;
    }
    let frame_period = 1.0 / f32::from(config.0.twitch.broadcast.frames_per_second);
    runtime.capture_elapsed += time.delta_secs();
    if runtime.capture_elapsed < frame_period {
        return;
    }
    runtime.capture_elapsed = runtime.capture_elapsed.rem_euclid(frame_period);
    if sensitive_screen.0 {
        let width = u32::from(config.0.twitch.broadcast.width);
        let height = u32::from(config.0.twitch.broadcast.height);
        let rgba = black_rgba_frame(width, height);
        let sent = runtime.controller.as_ref().is_some_and(|controller| {
            controller.send_video(VideoFrame {
                width,
                height,
                rgba,
            })
        });
        if sent {
            runtime.captured_video_frames = runtime.captured_video_frames.saturating_add(1);
        }
        return;
    }
    if runtime.capture_in_flight {
        if let Some(controller) = &runtime.controller {
            controller.drop_video_frame();
        }
        return;
    }
    runtime.capture_in_flight = true;
    commands.spawn(Screenshot::primary_window()).observe(
        move |captured: On<ScreenshotCaptured>,
              sensitive_screen: Res<SensitiveScreenActive>,
              mut runtime: ResMut<DirectBroadcastRuntime>| {
            runtime.capture_in_flight = false;
            let Some(controller) = runtime.controller.as_ref() else {
                return;
            };
            if sensitive_screen.0 {
                let width = captured.image.texture_descriptor.size.width;
                let height = captured.image.texture_descriptor.size.height;
                if controller.send_video(VideoFrame {
                    width,
                    height,
                    rgba: black_rgba_frame(width, height),
                }) {
                    runtime.captured_video_frames = runtime.captured_video_frames.saturating_add(1);
                }
                return;
            }
            match screenshot_rgba(&captured.image) {
                Ok((width, height, rgba)) => {
                    if controller.send_video(VideoFrame {
                        width,
                        height,
                        rgba,
                    }) {
                        runtime.captured_video_frames =
                            runtime.captured_video_frames.saturating_add(1);
                    }
                }
                Err(error) => warn!(%error, "could not convert broadcast frame"),
            }
        },
    );
}

fn black_rgba_frame(width: u32, height: u32) -> Vec<u8> {
    let bytes = usize::try_from(width)
        .unwrap_or(0)
        .saturating_mul(usize::try_from(height).unwrap_or(0))
        .saturating_mul(4);
    let mut rgba = vec![0; bytes];
    for alpha in rgba.iter_mut().skip(3).step_by(4) {
        *alpha = 255;
    }
    rgba
}

fn screenshot_rgba(image: &Image) -> Result<(u32, u32, Vec<u8>)> {
    let width = image.texture_descriptor.size.width;
    let height = image.texture_descriptor.size.height;
    if width == 0 || height == 0 {
        bail!("captured image has zero dimensions");
    }
    let rgba = match image.texture_descriptor.format {
        TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => image
            .data
            .clone()
            .context("captured RGBA image has no CPU data")?,
        TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => {
            let mut data = image
                .data
                .clone()
                .context("captured BGRA image has no CPU data")?;
            for pixel in data.chunks_exact_mut(4) {
                pixel.swap(0, 2);
            }
            data
        }
        _ => image
            .clone()
            .try_into_dynamic()
            .map_err(|error| anyhow!("unsupported screenshot format: {error}"))?
            .to_rgba8()
            .into_raw(),
    };
    let expected = usize::try_from(width)
        .ok()
        .and_then(|width| {
            usize::try_from(height)
                .ok()
                .map(|height| width * height * 4)
        })
        .context("captured image dimensions overflow")?;
    if rgba.len() != expected {
        bail!(
            "captured RGBA buffer has {} bytes, expected {expected}",
            rgba.len()
        );
    }
    Ok((width, height, rgba))
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
    rgba: Vec<u8>,
}

#[derive(Clone, Debug)]
struct AudioFrame {
    pts: i64,
    samples: Vec<f32>,
}

#[derive(Clone, Debug)]
enum MediaInput {
    Video(VideoFrame),
    Audio(AudioFrame),
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
    encoded_video: AtomicU64,
    dropped_video: AtomicU64,
    encoded_audio: AtomicU64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct BroadcastMetricsSnapshot {
    encoded_video: u64,
    dropped_video: u64,
    encoded_audio: u64,
}

impl BroadcastMetrics {
    fn snapshot(&self) -> BroadcastMetricsSnapshot {
        BroadcastMetricsSnapshot {
            encoded_video: self.encoded_video.load(Ordering::Relaxed),
            dropped_video: self.dropped_video.load(Ordering::Relaxed),
            encoded_audio: self.encoded_audio.load(Ordering::Relaxed),
        }
    }
}

struct BroadcastController {
    media: SyncSender<MediaInput>,
    events: Arc<Mutex<Receiver<WorkerEvent>>>,
    stop: Arc<AtomicBool>,
    metrics: Arc<BroadcastMetrics>,
}

impl BroadcastController {
    fn start(target: BroadcastTarget, config: BroadcastConfig) -> Result<Self> {
        let (media, receiver) = mpsc::sync_channel(MEDIA_QUEUE_CAPACITY);
        let (event_sender, event_receiver) = mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let metrics = Arc::new(BroadcastMetrics::default());

        let worker_stop = Arc::clone(&stop);
        let worker_metrics = Arc::clone(&metrics);
        let audio_event_sender = event_sender.clone();
        thread::Builder::new()
            .name("stream-town-ffmpeg".to_owned())
            .spawn(move || {
                run_broadcast_worker(
                    target,
                    config,
                    receiver,
                    &event_sender,
                    &worker_stop,
                    &worker_metrics,
                );
            })
            .context("failed to start the in-process FFmpeg worker")?;

        let audio_sender = media.clone();
        let audio_stop = Arc::clone(&stop);
        let audio_events = Arc::new(Mutex::new(event_receiver));
        let audio_spawn = thread::Builder::new()
            .name("stream-town-wasapi".to_owned())
            .spawn(move || {
                if let Err(error) = capture_process_audio(audio_sender, &audio_stop) {
                    let message = format!("game-process audio capture stopped: {error:#}");
                    error!(%error, "game-process audio capture stopped");
                    let _ = audio_event_sender.send(WorkerEvent::Error(message));
                }
            });
        if let Err(error) = audio_spawn {
            stop.store(true, Ordering::Relaxed);
            let _ = media.try_send(MediaInput::Stop);
            return Err(anyhow!(
                "failed to start WASAPI game-audio capture: {error}"
            ));
        }

        Ok(Self {
            media,
            events: audio_events,
            stop,
            metrics,
        })
    }

    fn send_video(&self, frame: VideoFrame) -> bool {
        match self.media.try_send(MediaInput::Video(frame)) {
            Ok(()) => true,
            Err(TrySendError::Full(_) | TrySendError::Disconnected(_)) => {
                self.drop_video_frame();
                false
            }
        }
    }

    fn drop_video_frame(&self) {
        self.metrics.dropped_video.fetch_add(1, Ordering::Relaxed);
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
}

impl Drop for BroadcastController {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        let _ = self.media.try_send(MediaInput::Stop);
    }
}

fn capture_process_audio(media: SyncSender<MediaInput>, stop: &AtomicBool) -> Result<()> {
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
            if media
                .send(MediaInput::Audio(AudioFrame { pts, samples }))
                .is_err()
            {
                break;
            }
            pts = pts.saturating_add(i64::try_from(AUDIO_FRAME_SAMPLES).unwrap_or(i64::MAX));
        }
    }
    let _ = client.stop_stream();
    Ok(())
}

fn run_broadcast_worker(
    target: BroadcastTarget,
    config: BroadcastConfig,
    receiver: Receiver<MediaInput>,
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
        match encode_broadcast_session(&target, &config, &receiver, stop, metrics, events) {
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

    fn take_due_pts(&mut self, now: Instant) -> Option<i64> {
        let deadline = self.next_deadline.as_mut()?;
        if now < *deadline {
            return None;
        }
        *deadline += self.frame_period;
        let pts = self.next_pts;
        self.next_pts = self.next_pts.saturating_add(1);
        Some(pts)
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
    scaler: Option<(u32, u32, software::scaling::Context)>,
    resampler: software::resampling::Context,
    width: u32,
    height: u32,
    audio_pts_base: Option<i64>,
}

fn encode_broadcast_session(
    target: &BroadcastTarget,
    config: &BroadcastConfig,
    receiver: &Receiver<MediaInput>,
    stop: &AtomicBool,
    metrics: &BroadcastMetrics,
    events: &mpsc::Sender<WorkerEvent>,
) -> Result<SessionEnd> {
    ffmpeg::init().context("could not initialize the linked FFmpeg libraries")?;
    ffmpeg::log::set_level(ffmpeg::log::Level::Quiet);
    let (mut encoder, encoder_name) = BroadcastEncoder::open(target, config)?;
    let _ = events.send(WorkerEvent::Broadcasting {
        encoder: encoder_name,
    });
    let mut cadence = VideoCadence::new(config.frames_per_second);
    let mut latest_video = None;
    loop {
        if stop.load(Ordering::Relaxed) {
            encoder.finish()?;
            return Ok(SessionEnd::Stopped);
        }
        while let Some(video) = latest_video.as_ref() {
            let Some(pts) = cadence.take_due_pts(Instant::now()) else {
                break;
            };
            encoder.encode_video(video, pts)?;
            metrics.encoded_video.fetch_add(1, Ordering::Relaxed);
        }
        match receiver.recv_timeout(cadence.receive_timeout(Instant::now())) {
            Ok(MediaInput::Video(video)) => {
                cadence.start(Instant::now());
                latest_video = Some(video);
            }
            Ok(MediaInput::Audio(audio)) => {
                // Establish both media timelines at the first video frame. This
                // avoids publishing an audio lead while the first GPU readback
                // is still pending, then keeps audio continuous while the
                // cadence worker repeats the latest image through game stalls.
                if latest_video.is_some() {
                    encoder.encode_audio(audio)?;
                    metrics.encoded_audio.fetch_add(1, Ordering::Relaxed);
                }
            }
            Ok(MediaInput::Stop) => {
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
        let (video, video_codec, encoder_name) = open_video_encoder(config, global_header)?;
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
                audio_pts_base: None,
            },
            encoder_name,
        ))
    }

    fn encode_video(&mut self, video: &VideoFrame, pts: i64) -> Result<()> {
        if self
            .scaler
            .as_ref()
            .is_none_or(|(width, height, _)| *width != video.width || *height != video.height)
        {
            let scaler = software::scaling::Context::get(
                ffmpeg::format::Pixel::RGBA,
                video.width,
                video.height,
                ffmpeg::format::Pixel::YUV420P,
                self.width,
                self.height,
                software::scaling::Flags::BILINEAR,
            )
            .context("could not initialize the broadcast video scaler")?;
            self.scaler = Some((video.width, video.height, scaler));
        }
        let mut rgba = frame::Video::new(ffmpeg::format::Pixel::RGBA, video.width, video.height);
        let source_stride = usize::try_from(video.width).unwrap_or(0) * 4;
        let target_stride = rgba.stride(0);
        for (row, source) in video.rgba.chunks_exact(source_stride).enumerate() {
            let start = row * target_stride;
            rgba.data_mut(0)[start..start + source_stride].copy_from_slice(source);
        }
        rgba.set_pts(Some(pts));
        let mut yuv = frame::Video::new(ffmpeg::format::Pixel::YUV420P, self.width, self.height);
        self.scaler
            .as_mut()
            .context("broadcast scaler was not initialized")?
            .2
            .run(&rgba, &mut yuv)
            .context("could not scale a broadcast frame")?;
        yuv.set_pts(Some(pts));
        self.video
            .send_frame(&yuv)
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

fn open_video_encoder(
    config: &BroadcastConfig,
    global_header: bool,
) -> Result<(encoder::video::Encoder, Codec, String)> {
    let mut failures = Vec::new();
    for &name in encoder_candidates(config.encoder) {
        let Some(codec) = encoder::find_by_name(name) else {
            continue;
        };
        let outcome = (|| -> Result<encoder::video::Encoder> {
            let mut video = codec::context::Context::new_with_codec(codec)
                .encoder()
                .video()
                .context("encoder is not a video encoder")?;
            video.set_width(u32::from(config.width));
            video.set_height(u32::from(config.height));
            video.set_format(ffmpeg::format::Pixel::YUV420P);
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
                    options.set("preset", "p4");
                    options.set("tune", "ll");
                    options.set("rc", "cbr");
                    options.set("zerolatency", "1");
                }
                "h264_qsv" => {
                    options.set("profile", "high");
                    options.set("preset", "medium");
                    options.set("look_ahead", "0");
                }
                "h264_amf" => {
                    options.set("profile", "high");
                    options.set("usage", "ultralowlatency");
                    options.set("quality", "balanced");
                    options.set("rc", "cbr");
                }
                "libopenh264" => options.set("profile", "high"),
                _ => {}
            }
            video
                .open_as_with(codec, options)
                .with_context(|| format!("could not open {name}"))
        })();
        match outcome {
            Ok(video) => return Ok((video, codec, name.to_owned())),
            Err(error) => failures.push(format!("{name}: {error:#}")),
        }
    }
    bail!(
        "no requested H.264 encoder could be opened ({})",
        failures.join("; ")
    )
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
                rgba: rgba.clone(),
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
    fn sensitive_screen_frame_is_fully_opaque_black() {
        let frame = black_rgba_frame(2, 3);
        assert_eq!(frame.len(), 24);
        for pixel in frame.chunks_exact(4) {
            assert_eq!(pixel, [0, 0, 0, 255]);
        }
    }

    #[test]
    fn video_cadence_keeps_advancing_when_the_game_stops_supplying_frames() {
        let started = Instant::now();
        let mut cadence = VideoCadence::new(30);
        cadence.start(started);
        assert_eq!(cadence.take_due_pts(started), Some(0));
        assert_eq!(cadence.take_due_pts(started), None);

        let after_three_periods = started + cadence.frame_period * 3;
        assert_eq!(cadence.take_due_pts(after_three_periods), Some(1));
        assert_eq!(cadence.take_due_pts(after_three_periods), Some(2));
        assert_eq!(cadence.take_due_pts(after_three_periods), Some(3));
        assert_eq!(cadence.take_due_pts(after_three_periods), None);
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
                        rgba: rgba.clone(),
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
