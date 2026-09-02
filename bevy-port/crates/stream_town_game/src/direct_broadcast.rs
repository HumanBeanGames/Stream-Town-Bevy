//! In-process Twitch video broadcast for the Windows build.
//!
//! Video comes either from Windows Graphics Capture of the game preview or from
//! an asynchronous Bevy offscreen-target readback in stream-only mode. Process
//! audio is captured with WASAPI application loopback. Stream-only Tidal music
//! and native game audio are mixed from pre-monitor PCM routes so they remain
//! on-stream while silent at the operator output. Dynamically linked `FFmpeg` libraries
//! encode/mux H.264 + AAC into Twitch's RTMP ingest. No subprocess, virtual
//! cable, or OBS installation is involved.

use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fmt,
    fs::{self, OpenOptions},
    io::Write as _,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{self, Receiver, SyncSender, TrySendError},
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, anyhow, bail};
use bevy::{
    camera::RenderTarget,
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
        mouse::{MouseScrollUnit, MouseWheel},
    },
    prelude::*,
    render::{
        gpu_readback::{Readback, ReadbackComplete},
        render_resource::{TextureFormat, TextureUsages},
    },
    window::{CursorOptions, PrimaryWindow, WindowCloseRequested, WindowRef, WindowResolution},
    winit::{UpdateMode, WinitSettings},
};
use bevy_tidal::{NativeAudioFrame, NativeAudioRouting};
use ffmpeg::{
    ChannelLayout, Codec, Dictionary, Packet, Rational, codec, encoder, format, frame, software,
};
use ffmpeg_next as ffmpeg;
use stream_town_domain::{
    BroadcastConfig, BroadcastEncoderPreference, BroadcastRenderMode, PlayerSettingsStore,
};
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
    OperatorChatRuntime, RuntimeConfig, RuntimePlayerSettings, SensitiveScreenActive,
    SensitiveScreenUpdateSet, SimulationRuntime, TwitchConnection,
    twitch::{CredentialVault, OAuthClient, StoredOAuthToken, TwitchIngest},
};

const AUDIO_SAMPLE_RATE: u32 = 48_000;
const AUDIO_CHANNELS: usize = 2;
const AUDIO_FRAME_SAMPLES: usize = 1_024;
const AUDIO_QUEUE_CAPACITY: usize = 32;
const STREAM_HEALTH_REPORT_INTERVAL: Duration = Duration::from_secs(5);
const TWITCH_LIVE_VERIFICATION_TIMEOUT: Duration = Duration::from_mins(1);
const TWITCH_LIVE_VERIFICATION_INTERVAL: Duration = Duration::from_secs(2);
const TWITCH_LIVE_REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_RECONNECT_DELAY_SECONDS: u64 = 30;
const DIRECT_BROADCAST_LOG_MAX_BYTES: u64 = 1_048_576;
const DIRECT_BROADCAST_LOG_PATH: &str = ".stream-town/diagnostics/direct-broadcast.log";
const DIRECT_BROADCAST_LOG_QUEUE_CAPACITY: usize = 256;
const OPERATOR_WINDOW_WIDTH: u32 = 1_100;
const OPERATOR_WINDOW_HEIGHT: u32 = 680;
const NATIVE_GAME_AUDIO_QUEUE_CAPACITY: usize = 64;
const OFFLINE_FRAME_HOLD: Duration = Duration::from_secs(1);
const BROADCAST_EXIT_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_STREAM_READBACKS_IN_FLIGHT: usize = 4;
const MAX_STREAM_COMPLETED_READBACKS: usize = MAX_STREAM_READBACKS_IN_FLIGHT;
const STREAM_READBACK_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Clone)]
struct NativeGameAudioClip {
    key: String,
    samples: Arc<[f32]>,
    gain: f32,
}

#[derive(Default)]
struct NativeGameAudioState {
    looping: Option<NativeGameAudioClip>,
    pending: VecDeque<NativeGameAudioClip>,
}

struct NativeGameAudioRoutingInner {
    local_monitor_enabled: AtomicBool,
    stream_output_enabled: AtomicBool,
    state: Mutex<NativeGameAudioState>,
}

/// Pre-monitor route for the game's Bevy ambience and sound effects.
///
/// Headed broadcasts continue to use process loopback. Stream-only broadcasts
/// mute the local Bevy sinks and mix this route beside Tidal in the encoder
/// worker, so the operator dashboard remains silent without muting Twitch.
#[derive(Resource, Clone)]
pub(crate) struct NativeGameAudioRouting(Arc<NativeGameAudioRoutingInner>);

impl Default for NativeGameAudioRouting {
    fn default() -> Self {
        Self(Arc::new(NativeGameAudioRoutingInner {
            local_monitor_enabled: AtomicBool::new(true),
            stream_output_enabled: AtomicBool::new(false),
            state: Mutex::new(NativeGameAudioState::default()),
        }))
    }
}

impl NativeGameAudioRouting {
    pub(crate) fn set_local_monitor_enabled(&self, enabled: bool) {
        self.0
            .local_monitor_enabled
            .store(enabled, Ordering::Relaxed);
    }

    pub(crate) fn local_monitor_enabled(&self) -> bool {
        self.0.local_monitor_enabled.load(Ordering::Relaxed)
    }

    fn set_stream_output_enabled(&self, enabled: bool) {
        self.0
            .stream_output_enabled
            .store(enabled, Ordering::Relaxed);
        if !enabled && let Ok(mut state) = self.0.state.lock() {
            state.pending.clear();
        }
    }

    pub(crate) fn set_looping_pcm16_wav(&self, key: &str, wav: &[u8], gain: f32) {
        if !self.0.stream_output_enabled.load(Ordering::Relaxed) {
            return;
        }
        let Ok(mut state) = self.0.state.lock() else {
            return;
        };
        if let Some(looping) = state.looping.as_mut()
            && looping.key == key
        {
            looping.gain = gain;
            return;
        }
        state.looping = pcm16_wav_clip(key, wav, gain);
    }

    pub(crate) fn clear_looping(&self) {
        if let Ok(mut state) = self.0.state.lock() {
            state.looping = None;
        }
    }

    pub(crate) fn play_pcm16_wav(&self, key: &str, wav: &[u8], gain: f32) {
        if !self.0.stream_output_enabled.load(Ordering::Relaxed) {
            return;
        }
        let Some(clip) = pcm16_wav_clip(key, wav, gain) else {
            return;
        };
        if let Ok(mut state) = self.0.state.lock() {
            if state.pending.len() == NATIVE_GAME_AUDIO_QUEUE_CAPACITY {
                state.pending.pop_front();
            }
            state.pending.push_back(clip);
        }
    }

    fn subscribe(&self) -> NativeGameAudioMix {
        NativeGameAudioMix {
            routing: self.clone(),
            looping_key: None,
            looping_samples: None,
            looping_gain: 0.0,
            looping_position: 0,
            voices: Vec::new(),
        }
    }
}

fn pcm16_wav_clip(key: &str, wav: &[u8], gain: f32) -> Option<NativeGameAudioClip> {
    let (channels, sample_rate, data) = pcm16_wav_data(wav)?;
    if sample_rate != AUDIO_SAMPLE_RATE || !(channels == 1 || channels == 2) {
        return None;
    }
    let decoded = data
        .chunks_exact(usize::from(channels) * 2)
        .flat_map(|frame| {
            let left = f32::from(i16::from_le_bytes([frame[0], frame[1]])) / 32_768.0;
            let right = if channels == 2 {
                f32::from(i16::from_le_bytes([frame[2], frame[3]])) / 32_768.0
            } else {
                left
            };
            [left, right]
        })
        .collect::<Vec<_>>();
    (!decoded.is_empty()).then(|| NativeGameAudioClip {
        key: key.to_owned(),
        samples: decoded.into(),
        gain,
    })
}

fn pcm16_wav_data(wav: &[u8]) -> Option<(u16, u32, &[u8])> {
    if wav.get(0..4)? != b"RIFF" || wav.get(8..12)? != b"WAVE" {
        return None;
    }
    let mut cursor = 12_usize;
    let mut format = None;
    let mut data = None;
    while cursor.saturating_add(8) <= wav.len() {
        let chunk = wav.get(cursor..cursor + 4)?;
        let size = usize::try_from(u32::from_le_bytes(
            wav.get(cursor + 4..cursor + 8)?.try_into().ok()?,
        ))
        .ok()?;
        let start = cursor + 8;
        let end = start.checked_add(size)?;
        let payload = wav.get(start..end)?;
        if chunk == b"fmt " && payload.len() >= 16 {
            let encoding = u16::from_le_bytes(payload[0..2].try_into().ok()?);
            let channels = u16::from_le_bytes(payload[2..4].try_into().ok()?);
            let sample_rate = u32::from_le_bytes(payload[4..8].try_into().ok()?);
            let bits = u16::from_le_bytes(payload[14..16].try_into().ok()?);
            if encoding != 1 || bits != 16 {
                return None;
            }
            format = Some((channels, sample_rate));
        } else if chunk == b"data" {
            data = Some(payload);
        }
        cursor = end.saturating_add(size & 1);
    }
    let (channels, sample_rate) = format?;
    Some((channels, sample_rate, data?))
}

struct NativeGameAudioVoice {
    samples: Arc<[f32]>,
    gain: f32,
    position: usize,
}

struct NativeGameAudioMix {
    routing: NativeGameAudioRouting,
    looping_key: Option<String>,
    looping_samples: Option<Arc<[f32]>>,
    looping_gain: f32,
    looping_position: usize,
    voices: Vec<NativeGameAudioVoice>,
}

impl NativeGameAudioMix {
    fn receive(&mut self) {
        let Ok(mut state) = self.routing.0.state.lock() else {
            return;
        };
        match state.looping.as_ref() {
            Some(looping) if self.looping_key.as_deref() == Some(looping.key.as_str()) => {
                self.looping_gain = looping.gain;
            }
            Some(looping) => {
                self.looping_key = Some(looping.key.clone());
                self.looping_samples = Some(looping.samples.clone());
                self.looping_gain = looping.gain;
                self.looping_position = 0;
            }
            None => {
                self.looping_key = None;
                self.looping_samples = None;
                self.looping_position = 0;
            }
        }
        self.voices
            .extend(state.pending.drain(..).map(|clip| NativeGameAudioVoice {
                samples: clip.samples,
                gain: clip.gain,
                position: 0,
            }));
    }

    fn mix_into(&mut self, output: &mut [f32]) {
        self.receive();
        for output_frame in output.chunks_exact_mut(AUDIO_CHANNELS) {
            if let Some(samples) = self.looping_samples.as_ref()
                && samples.len() >= AUDIO_CHANNELS
            {
                output_frame[0] += samples[self.looping_position] * self.looping_gain;
                output_frame[1] += samples[self.looping_position + 1] * self.looping_gain;
                self.looping_position = (self.looping_position + AUDIO_CHANNELS) % samples.len();
            }
            for voice in &mut self.voices {
                if voice.position + 1 >= voice.samples.len() {
                    continue;
                }
                output_frame[0] += voice.samples[voice.position] * voice.gain;
                output_frame[1] += voice.samples[voice.position + 1] * voice.gain;
                voice.position += AUDIO_CHANNELS;
            }
            output_frame[0] = output_frame[0].clamp(-1.0, 1.0);
            output_frame[1] = output_frame[1].clamp(-1.0, 1.0);
        }
        self.voices
            .retain(|voice| voice.position + 1 < voice.samples.len());
    }
}

fn direct_broadcast_log_path() -> PathBuf {
    std::env::var_os("STREAM_TOWN_BROADCAST_LOG")
        .map_or_else(|| PathBuf::from(DIRECT_BROADCAST_LOG_PATH), PathBuf::from)
}

fn append_direct_broadcast_diagnostic(level: &str, message: &str) {
    static DIAGNOSTICS: OnceLock<SyncSender<(String, String)>> = OnceLock::new();
    let sender = DIAGNOSTICS.get_or_init(|| {
        let (sender, receiver) =
            mpsc::sync_channel::<(String, String)>(DIRECT_BROADCAST_LOG_QUEUE_CAPACITY);
        let path = direct_broadcast_log_path();
        let _ = thread::Builder::new()
            .name("stream-town-broadcast-log".to_owned())
            .spawn(move || {
                while let Ok((level, message)) = receiver.recv() {
                    let _ = append_direct_broadcast_diagnostic_to(&path, &level, &message);
                }
            });
        sender
    });
    // Diagnostics must never stall rendering or encoding. A full queue means
    // the disk is already unhealthy, so dropping a sample is safer than
    // blocking the game thread and causing the very stream hitch being logged.
    let _ = sender.try_send((level.to_owned(), message.to_owned()));
}

fn append_direct_broadcast_diagnostic_to(
    path: &Path,
    level: &str,
    message: &str,
) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    if path
        .metadata()
        .is_ok_and(|metadata| metadata.len() >= DIRECT_BROADCAST_LOG_MAX_BYTES)
    {
        let previous = path.with_extension("previous.log");
        if previous.exists() {
            fs::remove_file(&previous)?;
        }
        fs::rename(path, previous)?;
    }
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{timestamp} {level} {message}")
}
const OPERATOR_CHAT_VISIBLE_ROWS: usize = 8;
const OPERATOR_CHAT_LEFT: f32 = 568.0;
const OPERATOR_CHAT_TOP: f32 = 326.0;
const OPERATOR_CHAT_WIDTH: f32 = 500.0;
const OPERATOR_CHAT_HEIGHT: f32 = 208.0;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DirectBroadcastPhase {
    Disabled,
    WaitingForBroadcasterAuthorization,
    WaitingForGameplay,
    ResolvingIngest,
    Connecting,
    VerifyingTwitch,
    Broadcasting,
    BandwidthTesting,
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
                | Self::VerifyingTwitch
                | Self::Broadcasting
                | Self::BandwidthTesting
                | Self::Reconnecting
                | Self::Stopping
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectBroadcastSnapshot {
    pub phase: DirectBroadcastPhase,
    pub encoder: Option<String>,
    pub encoder_rejections: Vec<String>,
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
    encoder_rejections: Vec<String>,
    ingest: Option<String>,
    authorization: Option<Arc<Mutex<Receiver<AuthorizationEvent>>>>,
    pending_target: Option<BroadcastTarget>,
    verification_target: Option<LiveVerificationTarget>,
    live_verification: Option<LiveVerification>,
    verification_status: Option<String>,
    controller: Option<BroadcastController>,
    capture_elapsed: f32,
    broadcast_started: Option<Instant>,
    health_reported_at: Option<Instant>,
    health_reported_metrics: BroadcastMetricsSnapshot,
    recent_video_replacements: u64,
    rolling_captured_video_fps: f64,
    rolling_encoded_video_fps: f64,
}

#[derive(Resource, Default)]
struct StreamOnlyCaptureState {
    target: Option<Handle<Image>>,
    readback_requests: HashMap<Entity, (u64, Instant)>,
    completed_readbacks: BTreeMap<u64, Option<VideoFrame>>,
    next_readback_sequence: u64,
    next_publish_sequence: u64,
    next_readback_at: Option<Instant>,
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
struct StreamOnlyReadbackArmed;

#[derive(Component)]
struct StreamOperatorInfoText;

#[derive(Component)]
struct StreamOperatorLiveButton;

#[derive(Component)]
struct StreamOperatorLiveButtonText;

#[derive(Component)]
struct StreamOperatorRestartButton;

#[derive(Component)]
struct StreamOperatorChatInput;

#[derive(Component)]
struct StreamOperatorChatInputText;

#[derive(Component)]
struct StreamOperatorChatSendButton;

#[derive(Component)]
struct StreamOperatorChatTimeoutButton;

#[derive(Component)]
struct StreamOperatorChatBanButton;

#[derive(Component)]
struct StreamOperatorChatSelectedText;

#[derive(Component)]
struct StreamOperatorChatRow {
    slot: usize,
    line_id: Option<u64>,
    user_id: String,
    login: String,
}

#[derive(Component)]
struct StreamOperatorChatRowText(usize);

#[derive(Clone, Copy, Component)]
struct StreamOperatorChatBadge {
    slot: usize,
    kind: OperatorChatBadgeKind,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum OperatorChatBadgeKind {
    Broadcaster,
    Moderator,
    Subscriber,
}

#[derive(Clone, Copy, Component)]
enum StreamOperatorChatScrollAction {
    Older,
    Newer,
}

#[derive(Component)]
struct StreamOperatorChatScrollThumb;

#[derive(Clone, Copy, Component)]
enum StreamOperatorSettingAction {
    BrightnessDown,
    BrightnessUp,
    MasterDown,
    MasterUp,
    MusicDown,
    MusicUp,
    EffectsDown,
    EffectsUp,
    AmbienceDown,
    AmbienceUp,
    ToggleShadows,
    ToggleReducedMotion,
}

#[derive(Component)]
struct StreamOperatorSettingsText;

type StreamCameraTargetQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static mut RenderTarget),
    (With<Camera>, Without<StreamOperatorCamera>),
>;
type StreamOperatorLiveButtonQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
    ),
    (With<StreamOperatorLiveButton>, Changed<Interaction>),
>;
type StreamOperatorRestartButtonQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static mut BackgroundColor),
    (With<StreamOperatorRestartButton>, Changed<Interaction>),
>;
type StreamOperatorChatRowTextQuery<'w, 's> = Query<
    'w,
    's,
    (&'static StreamOperatorChatRowText, &'static mut Text),
    (
        Without<StreamOperatorChatInputText>,
        Without<StreamOperatorChatSelectedText>,
    ),
>;
type StreamOperatorChatBadgeQuery<'w, 's> = Query<
    'w,
    's,
    (&'static StreamOperatorChatBadge, &'static mut Node),
    Without<StreamOperatorChatScrollThumb>,
>;
type StreamOperatorChatInputTextQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut Text,
    (
        With<StreamOperatorChatInputText>,
        Without<StreamOperatorChatSelectedText>,
        Without<StreamOperatorChatRowText>,
    ),
>;
type StreamOperatorChatSelectedTextQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut Text,
    (
        With<StreamOperatorChatSelectedText>,
        Without<StreamOperatorChatInputText>,
        Without<StreamOperatorChatRowText>,
    ),
>;

impl Default for DirectBroadcastRuntime {
    fn default() -> Self {
        Self {
            phase: DirectBroadcastPhase::Disabled,
            encoder: None,
            encoder_rejections: Vec::new(),
            ingest: None,
            authorization: None,
            pending_target: None,
            verification_target: None,
            live_verification: None,
            verification_status: None,
            controller: None,
            capture_elapsed: 0.0,
            broadcast_started: None,
            health_reported_at: None,
            health_reported_metrics: BroadcastMetricsSnapshot::default(),
            recent_video_replacements: 0,
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
            encoder_rejections: self.encoder_rejections.clone(),
            ingest: self.ingest.clone(),
            captured_video_frames: metrics.captured_video,
            encoded_video_frames: metrics.encoded_video,
            dropped_video_frames: metrics.dropped_video,
            encoded_audio_frames: metrics.encoded_audio,
            dropped_audio_frames: metrics.dropped_audio,
            replaced_video_frames: self.recent_video_replacements,
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

#[derive(Default)]
enum BroadcastStopDisposition {
    #[default]
    Stay,
    ReturnToMainMenu,
    Exit {
        requested_at: Instant,
    },
}

#[derive(Resource, Default)]
pub(crate) struct DirectBroadcastControl {
    restart_requested: bool,
    stop_requested: bool,
    stop_disposition: BroadcastStopDisposition,
}

#[derive(Resource)]
struct AutomaticBroadcastStart {
    requested: bool,
}

impl Default for AutomaticBroadcastStart {
    fn default() -> Self {
        Self {
            requested: std::env::var_os("STREAM_TOWN_AUTO_GO_LIVE").is_some(),
        }
    }
}

impl DirectBroadcastControl {
    pub(crate) fn request_restart(&mut self) {
        self.restart_requested = true;
        self.stop_requested = false;
        self.stop_disposition = BroadcastStopDisposition::Stay;
    }

    pub(crate) fn request_stop(&mut self) {
        self.stop_requested = true;
        self.restart_requested = false;
        self.stop_disposition = BroadcastStopDisposition::Stay;
    }

    pub(crate) fn request_stop_and_return_to_main_menu(&mut self) {
        self.request_stop();
        self.stop_disposition = BroadcastStopDisposition::ReturnToMainMenu;
    }

    fn request_stop_and_exit(&mut self) {
        self.request_stop();
        self.stop_disposition = BroadcastStopDisposition::Exit {
            requested_at: Instant::now(),
        };
    }

    #[cfg(test)]
    pub(crate) const fn stop_requested_for_test(&self) -> bool {
        self.stop_requested
    }

    #[cfg(test)]
    pub(crate) const fn restart_requested_for_test(&self) -> bool {
        self.restart_requested
    }
}

pub struct DirectTwitchBroadcastPlugin;

impl Plugin for DirectTwitchBroadcastPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AppExit>()
            .add_message::<WindowCloseRequested>()
            .init_resource::<DirectBroadcastRuntime>()
            .init_resource::<DirectBroadcastControl>()
            .init_resource::<AutomaticBroadcastStart>()
            .init_resource::<NativeGameAudioRouting>()
            .init_resource::<StreamOnlyCaptureState>()
            .init_resource::<OperatorChatRuntime>()
            .add_systems(First, disarm_stream_only_readbacks)
            .add_systems(
                Update,
                (
                    start_local_broadcast_diagnostic,
                    operator_window_close_requests_exit,
                    request_automatic_broadcast_start,
                    apply_direct_broadcast_control,
                    poll_direct_broadcast_authorization,
                    start_prepared_broadcast_when_gameplay_ready,
                    poll_direct_broadcast_worker,
                    poll_twitch_live_verification,
                    exit_after_broadcast_stops,
                    return_to_main_menu_after_broadcast_stops,
                    sync_stream_only_capture,
                    cleanup_completed_stream_only_readbacks,
                    stream_operator_live_button,
                    stream_operator_restart_button,
                    update_stream_operator_info,
                    stream_operator_chat_controls,
                    update_stream_operator_chat,
                    stream_operator_settings_controls,
                    update_stream_operator_settings,
                    capture_direct_broadcast_frame.after(SensitiveScreenUpdateSet),
                )
                    .chain(),
            )
            .add_systems(Last, arm_stream_only_readback);
    }
}

fn request_automatic_broadcast_start(
    mut automatic: ResMut<AutomaticBroadcastStart>,
    mut control: ResMut<DirectBroadcastControl>,
) {
    if std::mem::take(&mut automatic.requested) {
        control.request_restart();
    }
}

fn operator_window_close_requests_exit(
    mut closed: MessageReader<WindowCloseRequested>,
    operator_windows: Query<(), With<StreamOperatorWindow>>,
    mut control: ResMut<DirectBroadcastControl>,
) {
    if closed
        .read()
        .any(|request| operator_windows.contains(request.window))
    {
        control.request_stop_and_exit();
    }
}

fn exit_after_broadcast_stops(
    mut runtime: ResMut<DirectBroadcastRuntime>,
    mut control: ResMut<DirectBroadcastControl>,
    mut exit: MessageWriter<AppExit>,
) {
    let BroadcastStopDisposition::Exit { requested_at } = &control.stop_disposition else {
        return;
    };
    let stopped = runtime.controller.is_none()
        && matches!(
            runtime.phase,
            DirectBroadcastPhase::Stopped
                | DirectBroadcastPhase::Disabled
                | DirectBroadcastPhase::Error(_)
        );
    let timed_out = requested_at.elapsed() >= BROADCAST_EXIT_TIMEOUT;
    if !stopped && !timed_out {
        return;
    }
    if timed_out && !stopped {
        warn!("timed out publishing the offline broadcast frame; forcing game exit");
        if let Some(controller) = runtime.controller.take() {
            controller.request_abort();
        }
    }
    control.stop_disposition = BroadcastStopDisposition::Stay;
    exit.write(AppExit::Success);
}

fn start_local_broadcast_diagnostic(
    config: Res<RuntimeConfig>,
    gameplay_ready: Option<Res<crate::GameplayReady>>,
    tidal_routing: Option<Res<NativeAudioRouting>>,
    game_audio_routing: Res<NativeGameAudioRouting>,
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
        tidal_routing.as_ref().map(AsRef::as_ref),
        Some(&game_audio_routing),
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
        runtime.verification_target = None;
        runtime.live_verification = None;
        runtime.verification_status = None;
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
    // Going live is an operator action during ordinary launches. Patch
    // redeployment makes that action explicit through STREAM_TOWN_AUTO_GO_LIVE;
    // persisted settings and legacy `start_on_launch` values never do so.
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
    let bandwidth_test = twitch.broadcast.bandwidth_test
        || std::env::var_os("STREAM_TOWN_FORCE_BANDWIDTH_TEST").is_some();
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
) -> Result<PreparedBroadcast> {
    let tokio = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to construct the Twitch broadcast network runtime")?;
    tokio.block_on(async {
        let oauth = OAuthClient::broadcaster(client_id.to_owned())?;
        let vault = CredentialVault::broadcaster(client_id, channel_login);
        let (token, validation) = oauth.load_validated_token(&vault).await.with_context(|| {
            format!(
                "Twitch broadcaster '{channel_login}' is not authorized; open Main Menu > Secrets"
            )
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
        Ok(PreparedBroadcast {
            target: BroadcastTarget {
                ingest_name: ingest.name.clone(),
                url,
            },
            verification: LiveVerificationTarget {
                client_id: client_id.to_owned(),
                broadcaster_id: validation.user_id,
                bandwidth_test,
                token: Some(token),
            },
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
        AuthorizationEvent::Ready(prepared) => {
            info!(ingest = %prepared.target.ingest_name, "Twitch broadcast authorization ready");
            runtime.ingest = Some(prepared.target.ingest_name.clone());
            runtime.pending_target = Some(prepared.target);
            runtime.verification_target = Some(prepared.verification);
            runtime.phase = DirectBroadcastPhase::WaitingForGameplay;
        }
        AuthorizationEvent::Error(error) => {
            error!(%error, "Twitch broadcast authorization failed");
            runtime.phase = DirectBroadcastPhase::Error(error);
        }
    }
}

fn start_prepared_broadcast_when_gameplay_ready(
    config: Res<RuntimeConfig>,
    gameplay_ready: Option<Res<crate::GameplayReady>>,
    tidal_routing: Option<Res<NativeAudioRouting>>,
    game_audio_routing: Res<NativeGameAudioRouting>,
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
        tidal_routing.as_ref().map(AsRef::as_ref),
        Some(&game_audio_routing),
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
    capture: Res<StreamOnlyCaptureState>,
    mut runtime: ResMut<DirectBroadcastRuntime>,
) {
    let events = runtime
        .controller
        .as_ref()
        .map(BroadcastController::events)
        .unwrap_or_default();
    for event in events {
        match event {
            WorkerEvent::Connecting => {
                runtime.phase = DirectBroadcastPhase::Connecting;
                append_direct_broadcast_diagnostic("INFO", "event=session_connecting");
            }
            WorkerEvent::Broadcasting {
                encoder,
                rejected_encoders,
            } => {
                info!(%encoder, "direct Twitch broadcast encoder active");
                append_direct_broadcast_diagnostic(
                    "INFO",
                    &format!(
                        "event=session_broadcasting encoder={encoder:?} rejected_encoders={}",
                        rejected_encoders.len()
                    ),
                );
                runtime.encoder = Some(encoder);
                runtime.encoder_rejections = rejected_encoders;
                let now = Instant::now();
                runtime.broadcast_started.get_or_insert(now);
                reset_stream_health_window(&mut runtime, now);
                begin_twitch_live_verification(&mut runtime);
            }
            WorkerEvent::Reconnecting(error) => {
                let metrics = runtime.controller.as_ref().map_or_else(
                    BroadcastMetricsSnapshot::default,
                    BroadcastController::metrics,
                );
                runtime.live_verification = None;
                runtime.verification_status = Some(format!("RTMP reconnect: {error}"));
                runtime.phase = DirectBroadcastPhase::Reconnecting;
                reset_stream_health_window(&mut runtime, Instant::now());
                warn!(%error, "direct Twitch broadcast reconnecting");
                append_direct_broadcast_diagnostic(
                    "WARN",
                    &format!(
                        "event=session_reconnecting cause={error:?} captured_video={} encoded_video={} video_drops={} capture_replacements={} cadence_skips={} encoded_audio={} audio_drops={} audio_queue={} maximum_capture_ms={:.2} maximum_encode_ms={:.2}",
                        metrics.captured_video,
                        metrics.encoded_video,
                        metrics.dropped_video,
                        metrics.replaced_video,
                        metrics.skipped_video,
                        metrics.encoded_audio,
                        metrics.dropped_audio,
                        metrics.queued_audio,
                        micros_to_milliseconds(metrics.maximum_capture_micros),
                        micros_to_milliseconds(metrics.maximum_video_encode_micros),
                    ),
                );
            }
            WorkerEvent::Stopped => {
                runtime.live_verification = None;
                if !matches!(runtime.phase, DirectBroadcastPhase::Error(_)) {
                    runtime.phase = DirectBroadcastPhase::Stopped;
                    runtime.verification_status = Some("Broadcast output stopped".to_owned());
                }
                append_direct_broadcast_diagnostic("INFO", "event=session_stopped");
                runtime.controller = None;
            }
            WorkerEvent::Error(error) => {
                error!(%error, "direct Twitch broadcast worker stopped");
                append_direct_broadcast_diagnostic(
                    "ERROR",
                    &format!("event=worker_stopped cause={error:?}"),
                );
                runtime.phase = DirectBroadcastPhase::Error(error);
            }
        }
    }
    report_stream_health(
        &mut runtime,
        config.0.twitch.broadcast.frames_per_second,
        capture.readback_requests.len(),
        capture.completed_readbacks.len(),
    );
}

fn reset_stream_health_window(runtime: &mut DirectBroadcastRuntime, now: Instant) {
    runtime.health_reported_at = Some(now);
    runtime.health_reported_metrics = runtime.controller.as_ref().map_or_else(
        BroadcastMetricsSnapshot::default,
        BroadcastController::metrics,
    );
    runtime.recent_video_replacements = 0;
    runtime.rolling_captured_video_fps = 0.0;
    runtime.rolling_encoded_video_fps = 0.0;
}

fn begin_twitch_live_verification(runtime: &mut DirectBroadcastRuntime) {
    let Some(target) = runtime.verification_target.clone() else {
        runtime.phase = DirectBroadcastPhase::Broadcasting;
        return;
    };
    if target.bandwidth_test {
        runtime.live_verification = None;
        runtime.verification_status =
            Some("Bandwidth-test output is intentionally not publicly listed".to_owned());
        runtime.phase = DirectBroadcastPhase::BandwidthTesting;
        return;
    }
    runtime.live_verification = None;
    match LiveVerification::start(target) {
        Ok(verification) => {
            runtime.live_verification = Some(verification);
            runtime.verification_status =
                Some("Waiting for Twitch's public channel status...".to_owned());
            runtime.phase = DirectBroadcastPhase::VerifyingTwitch;
        }
        Err(error) => {
            if let Some(controller) = &runtime.controller {
                controller.request_stop();
            }
            runtime.verification_status = Some(format!("Verifier startup failed: {error:#}"));
            runtime.phase = DirectBroadcastPhase::Error(format!(
                "could not start Twitch live verification: {error:#}"
            ));
        }
    }
}

fn poll_twitch_live_verification(mut runtime: ResMut<DirectBroadcastRuntime>) {
    let events = runtime
        .live_verification
        .as_ref()
        .map(LiveVerification::events)
        .unwrap_or_default();
    for event in events {
        match event {
            LiveVerificationEvent::Status(status) => {
                status.clone_into(runtime.verification_status.get_or_insert_default());
                info!(%status, "Twitch live verification status");
            }
            LiveVerificationEvent::Live => {
                info!("Twitch Helix confirmed that the channel is publicly live");
                runtime.live_verification = None;
                runtime.verification_status =
                    Some("Twitch confirmed public LIVE status".to_owned());
                runtime.phase = DirectBroadcastPhase::Broadcasting;
            }
            LiveVerificationEvent::Error(error) => {
                if let Some(controller) = &runtime.controller {
                    controller.request_stop();
                }
                error!(%error, "Twitch never confirmed the public stream");
                runtime.live_verification = None;
                runtime.verification_status = Some(error.clone());
                runtime.phase = DirectBroadcastPhase::Error(error);
            }
        }
    }
}

fn return_to_main_menu_after_broadcast_stops(
    state: Option<Res<State<crate::GameState>>>,
    next_state: Option<ResMut<NextState<crate::GameState>>>,
    runtime: Res<DirectBroadcastRuntime>,
    mut control: ResMut<DirectBroadcastControl>,
) {
    if !matches!(
        &control.stop_disposition,
        BroadcastStopDisposition::ReturnToMainMenu
    ) || !matches!(
        runtime.phase,
        DirectBroadcastPhase::Stopped
            | DirectBroadcastPhase::Disabled
            | DirectBroadcastPhase::Error(_)
    ) {
        return;
    }
    control.stop_disposition = BroadcastStopDisposition::Stay;
    let (Some(state), Some(mut next_state)) = (state, next_state) else {
        return;
    };
    if *state.get() == crate::GameState::InGame {
        next_state.set(crate::GameState::MainMenu);
    }
}

fn report_stream_health(
    runtime: &mut DirectBroadcastRuntime,
    target_fps: u8,
    readbacks_in_flight: usize,
    readbacks_completed: usize,
) {
    if !matches!(
        runtime.phase,
        DirectBroadcastPhase::VerifyingTwitch
            | DirectBroadcastPhase::Broadcasting
            | DirectBroadcastPhase::BandwidthTesting
    ) {
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
    let new_video_replacements = metrics
        .replaced_video
        .saturating_sub(runtime.health_reported_metrics.replaced_video);
    runtime.recent_video_replacements = new_video_replacements;
    let new_video_skips = metrics
        .skipped_video
        .saturating_sub(runtime.health_reported_metrics.skipped_video);
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
        || new_video_drops > 0
        || new_video_skips > 0
        || new_audio_drops > 0;
    append_direct_broadcast_diagnostic(
        if unhealthy { "WARN" } else { "INFO" },
        &format!(
            "event=health target_fps={target_fps} captured_fps={captured_fps:.2} encoded_fps={encoded_fps:.2} audio_fps={audio_fps:.2} video_drops={new_video_drops} capture_replacements={new_video_replacements} cadence_skips={new_video_skips} audio_drops={new_audio_drops} audio_queue={} readbacks_in_flight={readbacks_in_flight} readbacks_completed={readbacks_completed} average_encode_ms={average_encode_ms:.2} maximum_encode_ms={:.2}",
            metrics.queued_audio,
            micros_to_milliseconds(metrics.maximum_video_encode_micros),
        ),
    );
    if unhealthy {
        warn!(
            target_fps,
            captured_fps,
            encoded_fps,
            audio_fps,
            new_video_drops,
            new_video_replacements,
            new_video_skips,
            new_audio_drops,
            audio_queue_depth = metrics.queued_audio,
            readbacks_in_flight,
            readbacks_completed,
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
            new_video_replacements,
            new_video_skips,
            new_audio_drops,
            audio_queue_depth = metrics.queued_audio,
            readbacks_in_flight,
            readbacks_completed,
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
    gameplay_ready: Option<Res<crate::GameplayReady>>,
    runtime: Res<DirectBroadcastRuntime>,
    tidal_routing: Option<Res<NativeAudioRouting>>,
    game_audio_routing: Res<NativeGameAudioRouting>,
    mut state: ResMut<StreamOnlyCaptureState>,
    mut images: Option<ResMut<Assets<Image>>>,
    mut camera_targets: StreamCameraTargetQuery,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut winit: Option<ResMut<WinitSettings>>,
) {
    let stream_only_configured =
        config.0.twitch.broadcast.render_mode == BroadcastRenderMode::StreamOnly;
    let operator_required = gameplay_ready.is_some();
    let target_required = operator_required && stream_only_configured;
    let operator_stream_only = stream_only_configured && operator_required;
    let stream_only_active = operator_stream_only
        && runtime.controller.is_some()
        && matches!(
            runtime.phase,
            DirectBroadcastPhase::Connecting
                | DirectBroadcastPhase::VerifyingTwitch
                | DirectBroadcastPhase::Broadcasting
                | DirectBroadcastPhase::BandwidthTesting
                | DirectBroadcastPhase::Reconnecting
                | DirectBroadcastPhase::Stopping
        );
    if let Some(routing) = tidal_routing {
        routing.set_local_monitor_enabled(!operator_stream_only);
    }
    game_audio_routing.set_local_monitor_enabled(!operator_stream_only);
    game_audio_routing.set_stream_output_enabled(stream_only_active);

    if target_required && state.target.is_none() {
        let Some(images) = images.as_deref_mut() else {
            return;
        };
        let width = u32::from(config.0.twitch.broadcast.width);
        let height = u32::from(config.0.twitch.broadcast.height);
        let mut target =
            Image::new_target_texture(width, height, TextureFormat::Bgra8UnormSrgb, None);
        target.texture_descriptor.usage |= TextureUsages::COPY_SRC;
        let target = images.add(target);
        state.target = Some(target);
        state.width = width;
        state.height = height;
        info!(width, height, "stream-only offscreen target ready");
    }

    if operator_required && state.operator_window.is_none() {
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
                CursorOptions {
                    visible: true,
                    hit_test: true,
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
        let operator_root =
            spawn_stream_operator_view(&mut commands, operator_camera, state.target.as_ref());
        state.operator_window = Some(operator_window);
        state.operator_camera = Some(operator_camera);
        state.operator_root = Some(operator_root);
        info!(stream_only_configured, "local stream operator panel ready");
    }

    if stream_only_active {
        if state.target.is_none() {
            return;
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
    if !target_required && state.target.is_some() {
        for entity in state.readback_requests.keys().copied().collect::<Vec<_>>() {
            commands.entity(entity).try_despawn();
        }
        state.readback_requests.clear();
        state.completed_readbacks.clear();
        state.next_publish_sequence = state.next_readback_sequence;
        state.next_readback_at = None;
        if let Some(target) = state.target.take()
            && let Some(images) = images.as_deref_mut()
        {
            images.remove(target.id());
        }
        state.width = 0;
        state.height = 0;
        info!("stream-only offscreen render target disabled; local preview restored");
    }
    if operator_required {
        return;
    }
    let operator_entities = [
        state.operator_root.take(),
        state.operator_camera.take(),
        state.operator_window.take(),
    ];
    let operator_was_open = operator_entities.iter().any(Option::is_some);
    for entity in operator_entities.into_iter().flatten() {
        commands.entity(entity).despawn();
    }
    if operator_was_open {
        info!("local stream operator panel closed");
    }
}

fn disarm_stream_only_readbacks(
    mut commands: Commands,
    armed: Query<Entity, With<StreamOnlyReadbackArmed>>,
) {
    // A persistent Readback queues a new asynchronous copy every render frame.
    // Remove it on the following main-world frame so each request has exactly
    // one source frame and therefore one sequence number.
    for entity in &armed {
        commands.entity(entity).remove::<Readback>();
    }
}

fn cleanup_completed_stream_only_readbacks(
    mut commands: Commands,
    armed: Query<Entity, With<StreamOnlyReadbackArmed>>,
    runtime: Res<DirectBroadcastRuntime>,
    mut state: ResMut<StreamOnlyCaptureState>,
) {
    let now = Instant::now();
    let stalled = state
        .readback_requests
        .iter()
        .filter_map(|(entity, (sequence, started))| {
            (now.saturating_duration_since(*started) >= STREAM_READBACK_TIMEOUT)
                .then_some((*entity, *sequence))
        })
        .collect::<Vec<_>>();
    let mut retired = u64::try_from(stalled.len()).unwrap_or(u64::MAX);
    for (entity, sequence) in stalled {
        state.readback_requests.remove(&entity);
        state.completed_readbacks.entry(sequence).or_insert(None);
    }

    // An abandoned early GPU copy used to block ordered publication forever
    // while every later 1080p frame accumulated behind it. Bound the reorder
    // window and retire the missing prefix as soon as that window fills.
    if state.completed_readbacks.len() >= MAX_STREAM_COMPLETED_READBACKS
        && let Some(first_completed) = state
            .completed_readbacks
            .first_key_value()
            .map(|(key, _)| *key)
        && first_completed > state.next_publish_sequence
    {
        let missing = first_completed.saturating_sub(state.next_publish_sequence);
        retired = retired.saturating_add(missing);
        state.next_publish_sequence = first_completed;
        let obsolete = state
            .readback_requests
            .iter()
            .filter_map(|(entity, (sequence, _))| (*sequence < first_completed).then_some(*entity))
            .collect::<Vec<_>>();
        for entity in obsolete {
            state.readback_requests.remove(&entity);
        }
    }

    let (next_sequence, frames) =
        take_ordered_readback_frames(state.next_publish_sequence, &mut state.completed_readbacks);
    state.next_publish_sequence = next_sequence;
    if let Some(controller) = runtime.controller.as_ref() {
        if retired > 0 {
            controller.drop_video_frames(retired);
        }
        for frame in frames {
            let _ = controller.send_video(frame);
        }
    }
    for entity in &armed {
        if !state.readback_requests.contains_key(&entity) {
            commands.entity(entity).try_despawn();
        }
    }
}

fn arm_stream_only_readback(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    gameplay_ready: Option<Res<crate::GameplayReady>>,
    sensitive_screen: Res<SensitiveScreenActive>,
    runtime: Res<DirectBroadcastRuntime>,
    mut state: ResMut<StreamOnlyCaptureState>,
) {
    let active = gameplay_ready.is_some()
        && config.0.twitch.broadcast.render_mode == BroadcastRenderMode::StreamOnly
        && !sensitive_screen.0
        && runtime.controller.is_some()
        && matches!(
            runtime.phase,
            DirectBroadcastPhase::Connecting
                | DirectBroadcastPhase::VerifyingTwitch
                | DirectBroadcastPhase::Broadcasting
                | DirectBroadcastPhase::BandwidthTesting
                | DirectBroadcastPhase::Reconnecting
        );
    if !active || state.readback_requests.len() >= MAX_STREAM_READBACKS_IN_FLIGHT {
        if !active {
            state.next_readback_at = None;
        }
        return;
    }
    let now = Instant::now();
    if !stream_readback_due(
        &mut state.next_readback_at,
        now,
        config.0.twitch.broadcast.frames_per_second,
    ) {
        return;
    }
    let Some(target) = state.target.clone() else {
        return;
    };
    let sequence = state.next_readback_sequence;
    state.next_readback_sequence = state.next_readback_sequence.saturating_add(1);
    let entity = commands
        .spawn((Readback::texture(target), StreamOnlyReadbackArmed))
        .observe(publish_stream_only_frame)
        .id();
    state.readback_requests.insert(entity, (sequence, now));
}

fn stream_readback_due(next: &mut Option<Instant>, now: Instant, frames_per_second: u8) -> bool {
    let period = Duration::from_secs_f64(1.0 / f64::from(frames_per_second.max(1)));
    let deadline = next.get_or_insert(now);
    if now < *deadline {
        return false;
    }
    let overdue = now.saturating_duration_since(*deadline);
    let period_nanos = period.as_nanos().max(1);
    let elapsed_slots = 1_u128.saturating_add(overdue.as_nanos() / period_nanos);
    let advance = u32::try_from(elapsed_slots).unwrap_or(u32::MAX);
    *deadline += period.saturating_mul(advance);
    true
}

fn spawn_stream_operator_view(
    commands: &mut Commands,
    camera: Entity,
    stream_target: Option<&Handle<Image>>,
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
                    font_size: FontSize::Px(26.0),
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
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextColor(Color::srgb(0.70, 0.78, 0.86)),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(50),
                    top: px(92),
                    width: px(510),
                    ..default()
                },
            ));
            root.spawn((
                StreamOperatorLiveButton,
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(48),
                    bottom: px(42),
                    width: px(230),
                    height: px(52),
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(px(10)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.08, 0.31, 0.18)),
                BorderColor::all(Color::srgb(0.31, 0.78, 0.46)),
            ))
            .with_child((
                StreamOperatorLiveButtonText,
                Text::new("● GO LIVE"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            root.spawn((
                StreamOperatorRestartButton,
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(292),
                    bottom: px(42),
                    width: px(190),
                    height: px(52),
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(px(10)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.22, 0.34)),
                BorderColor::all(Color::srgb(0.31, 0.58, 0.82)),
            ))
            .with_child((
                Text::new("RESTART STREAM"),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            root.spawn((
                Text::new("LOCAL SETTINGS · EXCLUDED FROM STREAM"),
                TextFont {
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.64, 0.76, 0.86)),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(48),
                    top: px(320),
                    ..default()
                },
            ));
            root.spawn((
                StreamOperatorSettingsText,
                Text::new("Loading settings…"),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::srgb(0.68, 0.75, 0.82)),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(48),
                    top: px(348),
                    width: px(470),
                    ..default()
                },
            ));
            let setting_buttons = [
                (
                    StreamOperatorSettingAction::BrightnessDown,
                    "BRIGHT −",
                    48.0,
                    430.0,
                ),
                (
                    StreamOperatorSettingAction::BrightnessUp,
                    "BRIGHT +",
                    148.0,
                    430.0,
                ),
                (
                    StreamOperatorSettingAction::MasterDown,
                    "MASTER −",
                    248.0,
                    430.0,
                ),
                (
                    StreamOperatorSettingAction::MasterUp,
                    "MASTER +",
                    348.0,
                    430.0,
                ),
                (
                    StreamOperatorSettingAction::MusicDown,
                    "MUSIC −",
                    48.0,
                    470.0,
                ),
                (
                    StreamOperatorSettingAction::MusicUp,
                    "MUSIC +",
                    148.0,
                    470.0,
                ),
                (
                    StreamOperatorSettingAction::EffectsDown,
                    "SFX −",
                    248.0,
                    470.0,
                ),
                (
                    StreamOperatorSettingAction::EffectsUp,
                    "SFX +",
                    348.0,
                    470.0,
                ),
                (
                    StreamOperatorSettingAction::AmbienceDown,
                    "AMBIENT −",
                    48.0,
                    510.0,
                ),
                (
                    StreamOperatorSettingAction::AmbienceUp,
                    "AMBIENT +",
                    148.0,
                    510.0,
                ),
                (
                    StreamOperatorSettingAction::ToggleShadows,
                    "SHADOWS",
                    248.0,
                    510.0,
                ),
                (
                    StreamOperatorSettingAction::ToggleReducedMotion,
                    "REDUCE MOTION",
                    348.0,
                    510.0,
                ),
            ];
            for (action, label, left, top) in setting_buttons {
                root.spawn((
                    action,
                    Button,
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(left),
                        top: px(top),
                        width: px(92),
                        height: px(30),
                        border_radius: BorderRadius::all(px(4)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.08, 0.15, 0.22)),
                ))
                .with_child((
                    Text::new(label),
                    TextFont {
                        font_size: FontSize::Px(10.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.78, 0.85, 0.91)),
                    Pickable::IGNORE,
                ));
            }
            let mut preview = root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: px(32),
                    top: px(32),
                    width: px(500),
                    height: px(281),
                    padding: UiRect::all(px(8)),
                    border: UiRect::all(px(2)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.035, 0.045, 0.06)),
                BorderColor::all(Color::srgb(0.22, 0.35, 0.48)),
            ));
            if let Some(stream_target) = stream_target {
                preview.with_children(|preview| {
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
            } else {
                preview.with_children(|preview| {
                    preview.spawn((
                        Name::new("Low-resolution stream preview"),
                        Text::new("HEADED MODE\nThe full game remains in the primary window"),
                        TextFont {
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::srgb(0.58, 0.68, 0.78)),
                        Node {
                            width: percent(100),
                            height: percent(100),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ));
                });
            }
            root.spawn((
                Name::new("Operator Twitch chat"),
                Node {
                    position_type: PositionType::Absolute,
                    right: px(32),
                    top: px(OPERATOR_CHAT_TOP),
                    width: px(OPERATOR_CHAT_WIDTH),
                    height: px(OPERATOR_CHAT_HEIGHT),
                    padding: UiRect::all(px(6)),
                    border: UiRect::all(px(1)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.025, 0.034, 0.048)),
                BorderColor::all(Color::srgb(0.16, 0.25, 0.34)),
            ))
            .with_children(|chat| {
                for slot in 0..OPERATOR_CHAT_VISIBLE_ROWS {
                    chat.spawn((
                        StreamOperatorChatRow {
                            slot,
                            line_id: None,
                            user_id: String::new(),
                            login: String::new(),
                        },
                        Button,
                        Node {
                            width: px(462),
                            height: px(24),
                            padding: UiRect::horizontal(px(5)),
                            border: UiRect::all(px(1)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderColor::all(Color::NONE),
                    ))
                    .with_children(|row| {
                        for (kind, icon, color) in [
                            (
                                OperatorChatBadgeKind::Broadcaster,
                                "★",
                                Color::srgb(1.0, 0.76, 0.2),
                            ),
                            (
                                OperatorChatBadgeKind::Moderator,
                                "◆",
                                Color::srgb(0.35, 0.9, 0.52),
                            ),
                            (
                                OperatorChatBadgeKind::Subscriber,
                                "♥",
                                Color::srgb(0.72, 0.45, 1.0),
                            ),
                        ] {
                            row.spawn((
                                StreamOperatorChatBadge { slot, kind },
                                Text::new(icon),
                                TextFont {
                                    font_size: FontSize::Px(11.0),
                                    ..default()
                                },
                                TextColor(color),
                                Node {
                                    display: Display::None,
                                    width: px(14),
                                    flex_shrink: 0.0,
                                    ..default()
                                },
                                Pickable::IGNORE,
                            ));
                        }
                        row.spawn((
                            StreamOperatorChatRowText(slot),
                            Text::new(""),
                            TextFont {
                                font_size: FontSize::Px(12.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.78, 0.84, 0.9)),
                            Node {
                                flex_shrink: 1.0,
                                ..default()
                            },
                            Pickable::IGNORE,
                        ));
                    });
                }
                for (action, icon, top) in [
                    (StreamOperatorChatScrollAction::Older, "▲", 4.0),
                    (StreamOperatorChatScrollAction::Newer, "▼", 174.0),
                ] {
                    chat.spawn((
                        action,
                        Button,
                        Node {
                            position_type: PositionType::Absolute,
                            right: px(4),
                            top: px(top),
                            width: px(18),
                            height: px(18),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.10, 0.16, 0.22)),
                    ))
                    .with_child((
                        Text::new(icon),
                        TextFont {
                            font_size: FontSize::Px(9.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.72, 0.8, 0.88)),
                        Pickable::IGNORE,
                    ));
                }
                chat.spawn((
                    Name::new("Operator chat scrollbar"),
                    Node {
                        position_type: PositionType::Absolute,
                        right: px(8),
                        top: px(25),
                        width: px(10),
                        height: px(145),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.055, 0.075, 0.1)),
                    Pickable::IGNORE,
                ))
                .with_child((
                    StreamOperatorChatScrollThumb,
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(1),
                        top: px(0),
                        width: px(8),
                        height: px(145),
                        border_radius: BorderRadius::all(px(4)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.28, 0.42, 0.56)),
                    Pickable::IGNORE,
                ));
            });
            root.spawn((
                StreamOperatorChatInput,
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    right: px(120),
                    top: px(546),
                    width: px(412),
                    height: px(38),
                    padding: UiRect::horizontal(px(10)),
                    border: UiRect::all(px(1)),
                    border_radius: BorderRadius::all(px(5)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.035, 0.047, 0.065)),
                BorderColor::all(Color::srgb(0.22, 0.35, 0.48)),
            ))
            .with_child((
                StreamOperatorChatInputText,
                Text::new("Click to type a Twitch chat message…"),
                TextFont {
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.64, 0.72, 0.8)),
                Pickable::IGNORE,
            ));
            root.spawn((
                StreamOperatorChatSendButton,
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    right: px(32),
                    top: px(546),
                    width: px(80),
                    height: px(38),
                    border_radius: BorderRadius::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.13, 0.31, 0.48)),
            ))
            .with_child((
                Text::new("SEND"),
                TextFont {
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            root.spawn((
                StreamOperatorChatTimeoutButton,
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    right: px(164),
                    top: px(594),
                    width: px(128),
                    height: px(34),
                    border_radius: BorderRadius::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.42, 0.26, 0.07)),
            ))
            .with_child((
                Text::new("TIMEOUT 10M"),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            root.spawn((
                StreamOperatorChatBanButton,
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    right: px(32),
                    top: px(594),
                    width: px(124),
                    height: px(34),
                    border_radius: BorderRadius::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.42, 0.08, 0.07)),
            ))
            .with_child((
                Text::new("BAN"),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            root.spawn((
                StreamOperatorChatSelectedText,
                Text::new("Select a chat row to moderate that user"),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::srgb(0.62, 0.7, 0.78)),
                Node {
                    position_type: PositionType::Absolute,
                    right: px(300),
                    top: px(603),
                    width: px(232),
                    ..default()
                },
            ));
        })
        .id()
}

fn stream_operator_live_button(
    runtime: Res<DirectBroadcastRuntime>,
    mut control: ResMut<DirectBroadcastControl>,
    mut buttons: StreamOperatorLiveButtonQuery,
    mut labels: Query<&mut Text, With<StreamOperatorLiveButtonText>>,
) {
    let phase = runtime.snapshot().phase;
    let active = phase.is_active();
    let broadcasting = phase == DirectBroadcastPhase::Broadcasting;
    let bandwidth_testing = phase == DirectBroadcastPhase::BandwidthTesting;
    let ending_output = broadcasting || bandwidth_testing;
    if let Ok(mut label) = labels.single_mut() {
        operator_live_button_label(&phase).clone_into(&mut **label);
    }
    for (interaction, mut background, mut border) in &mut buttons {
        let hovered = *interaction == Interaction::Hovered;
        background.0 = if ending_output {
            if hovered {
                Color::srgb(0.47, 0.08, 0.07)
            } else {
                Color::srgb(0.34, 0.045, 0.04)
            }
        } else if hovered {
            Color::srgb(0.11, 0.42, 0.24)
        } else {
            Color::srgb(0.08, 0.31, 0.18)
        };
        *border = BorderColor::all(if ending_output {
            Color::srgb(1.0, 0.32, 0.28)
        } else {
            Color::srgb(0.31, 0.78, 0.46)
        });
        if *interaction == Interaction::Pressed {
            if active {
                control.request_stop_and_return_to_main_menu();
            } else {
                control.request_restart();
            }
        }
    }
}

fn stream_operator_restart_button(
    mut control: ResMut<DirectBroadcastControl>,
    mut buttons: StreamOperatorRestartButtonQuery,
) {
    for (interaction, mut background) in &mut buttons {
        background.0 = if *interaction == Interaction::Hovered {
            Color::srgb(0.17, 0.34, 0.52)
        } else {
            Color::srgb(0.12, 0.22, 0.34)
        };
        if *interaction == Interaction::Pressed {
            control.request_restart();
        }
    }
}

fn operator_live_button_label(phase: &DirectBroadcastPhase) -> &'static str {
    match phase {
        DirectBroadcastPhase::Broadcasting => "● LIVE · END STREAM",
        DirectBroadcastPhase::BandwidthTesting => "● BANDWIDTH TEST · END TEST",
        DirectBroadcastPhase::VerifyingTwitch => "● VERIFYING TWITCH · CANCEL",
        phase if phase.is_active() => "● NOT LIVE · CANCEL START",
        _ => "● NOT LIVE · GO LIVE",
    }
}

fn update_stream_operator_info(
    runtime: Res<DirectBroadcastRuntime>,
    config: Res<RuntimeConfig>,
    simulation: Option<Res<SimulationRuntime>>,
    mut text: Query<&mut Text, With<StreamOperatorInfoText>>,
) {
    let Ok(mut text) = text.single_mut() else {
        return;
    };
    let snapshot = runtime.snapshot();
    let fallback = if snapshot.encoder_rejections.is_empty() {
        "Auto selected its first usable backend".to_owned()
    } else {
        format!(
            "Auto fallbacks: {}",
            snapshot.encoder_rejections.join(" | ")
        )
    };
    let twitch_status = runtime
        .verification_status
        .as_deref()
        .unwrap_or("No Twitch public-status check is active");
    let enemy_status = stream_operator_enemy_status(&config.0, simulation.as_deref());
    **text = format!(
        "Status: {:?}\nTwitch check: {}\nEncoder: {}\n{}\nStream motion: {:.1} FPS\nOutput cadence: {:.1} FPS\nRecent capture replacements: {} · Output cadence skips: {}\nRejected video frames: {} · Audio drops: {}\nEncode latency: {:.2} ms average / {:.2} ms maximum\n{}\nDrop log: {}",
        snapshot.phase,
        twitch_status,
        snapshot.encoder.as_deref().unwrap_or("starting"),
        fallback,
        snapshot.captured_video_fps,
        snapshot.encoded_video_fps,
        snapshot.replaced_video_frames,
        snapshot.skipped_video_frames,
        snapshot.dropped_video_frames,
        snapshot.dropped_audio_frames,
        snapshot.average_encode_ms,
        snapshot.maximum_encode_ms,
        enemy_status,
        DIRECT_BROADCAST_LOG_PATH,
    );
}

fn stream_operator_enemy_status(
    config: &stream_town_domain::GameConfig,
    simulation: Option<&SimulationRuntime>,
) -> String {
    let Some(simulation) = simulation else {
        return "Enemy threat: world not loaded".to_owned();
    };
    let living_enemies = simulation
        .0
        .actors
        .values()
        .filter(|actor| actor.alive && actor.role.as_str() == "role:enemy")
        .count();
    let camps = simulation.0.enemy_camps.len();
    let timing = if let Some(raid) = &simulation.0.active_raid {
        format!(
            "raid wave {}/{} · {} tracked",
            raid.current_wave,
            raid.total_waves,
            raid.tracked_enemies.len()
        )
    } else if config.time.sample(simulation.0.elapsed_seconds).is_daytime {
        let remaining = seconds_until_enemy_night(&config.time, simulation.0.elapsed_seconds);
        format!("night spawning in {}", format_minutes_seconds(remaining))
    } else {
        "night spawning enabled".to_owned()
    };
    format!("Enemy threat: {living_enemies} active · {camps} camps · {timing}")
}

fn seconds_until_enemy_night(
    time: &stream_town_domain::TimeCycleConfig,
    elapsed_seconds: f64,
) -> u64 {
    let cycle = f64::from(time.seconds_per_day.max(1));
    let phase = elapsed_seconds.max(0.0).rem_euclid(cycle);
    let night_start = cycle * f64::from(time.daylight_per_thousand) / 1_000.0;
    let remaining = Duration::from_secs_f64((night_start - phase).max(0.0));
    remaining
        .as_secs()
        .saturating_add(u64::from(remaining.subsec_nanos() > 0))
}

fn format_minutes_seconds(seconds: u64) -> String {
    format!("{:02}:{:02}", seconds / 60, seconds % 60)
}

fn stream_operator_settings_controls(
    settings: Option<ResMut<RuntimePlayerSettings>>,
    interactions: Query<(&Interaction, &StreamOperatorSettingAction), Changed<Interaction>>,
) {
    let Some(mut settings) = settings else {
        return;
    };
    let mut changed = false;
    for (interaction, action) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            StreamOperatorSettingAction::BrightnessDown => {
                settings.0.video.brightness_ev =
                    (settings.0.video.brightness_ev - 0.5).clamp(-5.0, 5.0);
            }
            StreamOperatorSettingAction::BrightnessUp => {
                settings.0.video.brightness_ev =
                    (settings.0.video.brightness_ev + 0.5).clamp(-5.0, 5.0);
            }
            StreamOperatorSettingAction::MasterDown => {
                settings.0.audio.master = (settings.0.audio.master - 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::MasterUp => {
                settings.0.audio.master = (settings.0.audio.master + 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::MusicDown => {
                settings.0.audio.music = (settings.0.audio.music - 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::MusicUp => {
                settings.0.audio.music = (settings.0.audio.music + 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::EffectsDown => {
                settings.0.audio.sound_effects =
                    (settings.0.audio.sound_effects - 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::EffectsUp => {
                settings.0.audio.sound_effects =
                    (settings.0.audio.sound_effects + 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::AmbienceDown => {
                settings.0.audio.ambience = (settings.0.audio.ambience - 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::AmbienceUp => {
                settings.0.audio.ambience = (settings.0.audio.ambience + 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::ToggleShadows => {
                settings.0.video.shadows_enabled = !settings.0.video.shadows_enabled;
            }
            StreamOperatorSettingAction::ToggleReducedMotion => {
                settings.0.interface.reduced_motion = !settings.0.interface.reduced_motion;
            }
        }
        changed = true;
    }

    if changed {
        let store = PlayerSettingsStore::new(crate::player_settings_path());
        if let Err(error) = store.write(&settings.0) {
            warn!("could not persist operator player settings: {error}");
        }
    }
}

fn update_stream_operator_settings(
    settings: Option<Res<RuntimePlayerSettings>>,
    mut text: Query<&mut Text, With<StreamOperatorSettingsText>>,
) {
    let Some(settings) = settings else {
        return;
    };
    if !settings.is_changed() {
        return;
    }
    let Ok(mut text) = text.single_mut() else {
        return;
    };
    **text = format!(
        "Brightness {:+.1} EV  ·  Master {:>3.0}%  ·  Music {:>3.0}%\nSFX {:>3.0}%  ·  Ambience {:>3.0}%  ·  Shadows {}  ·  Motion {}",
        settings.0.video.brightness_ev,
        settings.0.audio.master * 100.0,
        settings.0.audio.music * 100.0,
        settings.0.audio.sound_effects * 100.0,
        settings.0.audio.ambience * 100.0,
        if settings.0.video.shadows_enabled {
            "ON"
        } else {
            "OFF"
        },
        if settings.0.interface.reduced_motion {
            "REDUCED"
        } else {
            "FULL"
        },
    );
}

fn stream_operator_chat_controls(
    state: Res<StreamOnlyCaptureState>,
    mut chat: ResMut<OperatorChatRuntime>,
    connection: Option<Res<TwitchConnection>>,
    keyboard: Option<MessageReader<KeyboardInput>>,
    wheel: Option<MessageReader<MouseWheel>>,
    operator_windows: Query<&Window, With<StreamOperatorWindow>>,
    input: Query<&Interaction, (Changed<Interaction>, With<StreamOperatorChatInput>)>,
    send: Query<&Interaction, (Changed<Interaction>, With<StreamOperatorChatSendButton>)>,
    timeout: Query<&Interaction, (Changed<Interaction>, With<StreamOperatorChatTimeoutButton>)>,
    ban: Query<&Interaction, (Changed<Interaction>, With<StreamOperatorChatBanButton>)>,
    rows: Query<(&Interaction, &StreamOperatorChatRow), Changed<Interaction>>,
    scroll: Query<(&Interaction, &StreamOperatorChatScrollAction), Changed<Interaction>>,
) {
    if input
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        chat.input_focused = true;
    }
    for (interaction, row) in &rows {
        if *interaction == Interaction::Pressed
            && let Some(line_id) = row.line_id
        {
            chat.selected_line = Some(line_id);
            chat.selected_user =
                (!row.user_id.is_empty()).then(|| (row.user_id.clone(), row.login.clone()));
        }
    }
    for (interaction, action) in &scroll {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            StreamOperatorChatScrollAction::Older => {
                chat.scroll_older(1, OPERATOR_CHAT_VISIBLE_ROWS);
            }
            StreamOperatorChatScrollAction::Newer => chat.scroll_newer(1),
        }
    }

    let operator_window = state.operator_window;
    if let Some(mut wheel) = wheel {
        let cursor_is_over_chat = operator_window
            .and_then(|entity| operator_windows.get(entity).ok())
            .and_then(Window::cursor_position)
            .is_some_and(|cursor| {
                cursor.x >= OPERATOR_CHAT_LEFT
                    && cursor.x <= OPERATOR_CHAT_LEFT + OPERATOR_CHAT_WIDTH
                    && cursor.y >= OPERATOR_CHAT_TOP
                    && cursor.y <= OPERATOR_CHAT_TOP + OPERATOR_CHAT_HEIGHT
            });
        for event in wheel
            .read()
            .filter(|event| Some(event.window) == operator_window)
        {
            if !cursor_is_over_chat {
                continue;
            }
            let rows = operator_chat_scroll_rows(event.unit, event.y);
            if event.y > 0.0 {
                chat.scroll_older(rows, OPERATOR_CHAT_VISIBLE_ROWS);
            } else if event.y < 0.0 {
                chat.scroll_newer(rows);
            }
        }
    }
    let mut submit = send
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed);
    if let Some(mut keyboard) = keyboard {
        for event in keyboard.read() {
            if !chat.input_focused
                || event.state != ButtonState::Pressed
                || Some(event.window) != operator_window
            {
                continue;
            }
            match &event.logical_key {
                Key::Enter => submit = true,
                Key::Escape => chat.input_focused = false,
                Key::Backspace => {
                    chat.draft.pop();
                }
                _ => {
                    if let Some(text) = &event.text {
                        for character in text.chars().filter(|character| !character.is_control()) {
                            if chat.draft.chars().count() < 500 {
                                chat.draft.push(character);
                            }
                        }
                    }
                }
            }
        }
    }
    if submit {
        send_operator_chat_message(&mut chat, connection.as_deref());
    }

    if timeout
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        moderate_selected_operator_user(&mut chat, connection.as_deref(), false);
    }
    if ban
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        moderate_selected_operator_user(&mut chat, connection.as_deref(), true);
    }
}

fn send_operator_chat_message(
    chat: &mut OperatorChatRuntime,
    connection: Option<&TwitchConnection>,
) {
    let message = chat.draft.trim();
    if message.is_empty() {
        "Enter a message before sending".clone_into(&mut chat.feedback);
        return;
    }
    let Some(transport) = connection.and_then(|connection| connection.transport.as_ref()) else {
        "Twitch chat is not connected".clone_into(&mut chat.feedback);
        return;
    };
    match transport.send(crate::twitch::TwitchControl::SendBroadcasterMessage(
        message.to_owned(),
    )) {
        Ok(()) => {
            "Sending from the broadcaster account…".clone_into(&mut chat.feedback);
            chat.draft.clear();
        }
        Err(error) => chat.feedback = format!("Could not send chat message: {error}"),
    }
}

fn moderate_selected_operator_user(
    chat: &mut OperatorChatRuntime,
    connection: Option<&TwitchConnection>,
    ban: bool,
) {
    let Some((user_id, login)) = chat.selected_user.clone() else {
        "Select a Twitch chat row before moderating".clone_into(&mut chat.feedback);
        return;
    };
    let Some(transport) = connection.and_then(|connection| connection.transport.as_ref()) else {
        "Twitch chat is not connected".clone_into(&mut chat.feedback);
        return;
    };
    let control = if ban {
        crate::twitch::TwitchControl::Ban {
            user_id,
            reason: "Moderated from Stream Town operator panel".to_owned(),
        }
    } else {
        crate::twitch::TwitchControl::Timeout {
            user_id,
            duration_seconds: 600,
            reason: "Timed out from Stream Town operator panel".to_owned(),
        }
    };
    match transport.send(control) {
        Ok(()) => {
            chat.feedback = if ban {
                format!("Ban requested for {login}")
            } else {
                format!("10-minute timeout requested for {login}")
            };
        }
        Err(error) => chat.feedback = format!("Could not moderate {login}: {error}"),
    }
}

fn update_stream_operator_chat(
    chat: Res<OperatorChatRuntime>,
    mut rows: Query<(
        &mut StreamOperatorChatRow,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
    mut row_text: StreamOperatorChatRowTextQuery,
    mut badges: StreamOperatorChatBadgeQuery,
    mut scroll_thumb: Query<
        &mut Node,
        (
            With<StreamOperatorChatScrollThumb>,
            Without<StreamOperatorChatBadge>,
        ),
    >,
    mut input_text: StreamOperatorChatInputTextQuery,
    mut selected_text: StreamOperatorChatSelectedTextQuery,
) {
    let visible = chat.visible_lines(OPERATOR_CHAT_VISIBLE_ROWS);
    for (mut row, mut background, mut border) in &mut rows {
        if let Some(line) = visible.get(row.slot) {
            row.line_id = Some(line.line_id);
            line.user_id.clone_into(&mut row.user_id);
            line.login.clone_into(&mut row.login);
            let selected = chat.selected_line == Some(line.line_id);
            background.0 = if selected {
                Color::srgb(0.12, 0.31, 0.47)
            } else {
                Color::NONE
            };
            *border = BorderColor::all(if selected {
                Color::srgb(0.35, 0.78, 1.0)
            } else {
                Color::NONE
            });
        } else {
            row.line_id = None;
            row.user_id.clear();
            row.login.clear();
            background.0 = Color::NONE;
            *border = BorderColor::all(Color::NONE);
        }
    }
    for (badge, mut node) in &mut badges {
        let show = visible.get(badge.slot).is_some_and(|line| {
            !line.is_system
                && match badge.kind {
                    OperatorChatBadgeKind::Broadcaster => line.badges.broadcaster,
                    OperatorChatBadgeKind::Moderator => line.badges.moderator,
                    OperatorChatBadgeKind::Subscriber => line.badges.subscriber,
                }
        });
        node.display = if show { Display::Flex } else { Display::None };
    }
    for (slot, mut text) in &mut row_text {
        **text = visible.get(slot.0).map_or_else(String::new, |line| {
            if line.is_system {
                format!("SYSTEM · {}", line.message)
            } else {
                format!("{}: {}", line.display_name, line.message)
            }
        });
    }
    if let Ok(mut thumb) = scroll_thumb.single_mut() {
        const TRACK_HEIGHT: f32 = 145.0;
        let total = chat.lines.len();
        let height = if total <= OPERATOR_CHAT_VISIBLE_ROWS {
            TRACK_HEIGHT
        } else {
            (TRACK_HEIGHT * bounded_history_f32(OPERATOR_CHAT_VISIBLE_ROWS)
                / bounded_history_f32(total))
            .max(18.0)
        };
        let maximum_scroll = chat.maximum_scroll(OPERATOR_CHAT_VISIBLE_ROWS);
        let newest_fraction = if maximum_scroll == 0 {
            1.0
        } else {
            1.0 - bounded_history_f32(chat.scroll_from_latest) / bounded_history_f32(maximum_scroll)
        };
        thumb.height = px(height);
        thumb.top = px((TRACK_HEIGHT - height) * newest_fraction);
    }
    if let Ok(mut text) = input_text.single_mut() {
        **text = if chat.draft.is_empty() {
            if chat.input_focused {
                "▌".to_owned()
            } else {
                "Click to type a Twitch chat message…".to_owned()
            }
        } else if chat.input_focused {
            format!("{}▌", chat.draft)
        } else {
            chat.draft.clone()
        };
    }
    if let Ok(mut text) = selected_text.single_mut() {
        let selected = chat.selected_user.as_ref().map_or_else(
            || "Select a chat row to moderate".to_owned(),
            |(_, login)| format!("Selected: {login}"),
        );
        **text = if chat.feedback.is_empty() {
            selected
        } else {
            format!("{selected}\n{}", chat.feedback)
        };
    }
}

fn operator_chat_scroll_rows(unit: MouseScrollUnit, delta: f32) -> usize {
    let mut remaining = match unit {
        MouseScrollUnit::Line => delta.abs(),
        MouseScrollUnit::Pixel => delta.abs() / 24.0,
    };
    let mut rows = 1;
    while remaining > 1.0 && rows < OPERATOR_CHAT_VISIBLE_ROWS {
        rows += 1;
        remaining -= 1.0;
    }
    rows
}

fn bounded_history_f32(value: usize) -> f32 {
    f32::from(u16::try_from(value).unwrap_or(u16::MAX))
}

const fn camera_targets_primary_window(target: &RenderTarget) -> bool {
    matches!(target, RenderTarget::Window(WindowRef::Primary))
}

fn publish_stream_only_frame(
    mut event: On<ReadbackComplete>,
    mut state: ResMut<StreamOnlyCaptureState>,
    sensitive_screen: Res<SensitiveScreenActive>,
    runtime: Res<DirectBroadcastRuntime>,
) {
    let Some((sequence, capture_started)) = state.readback_requests.remove(&event.entity) else {
        return;
    };
    let completed = if sensitive_screen.0 || state.width == 0 || state.height == 0 {
        None
    } else if let Some(controller) = runtime.controller.as_ref() {
        let readback = std::mem::take(&mut event.event_mut().data);
        let pixels = remove_gpu_row_padding(readback, state.width, state.height);
        if pixels.is_empty() {
            controller.drop_video_frames(1);
            None
        } else {
            controller
                .metrics
                .observe_capture_latency(capture_started.elapsed());
            Some(VideoFrame {
                width: state.width,
                height: state.height,
                pixel_format: VideoPixelFormat::Bgra,
                pixels,
            })
        }
    } else {
        None
    };
    state.completed_readbacks.insert(sequence, completed);
    let (next_sequence, frames) =
        take_ordered_readback_frames(state.next_publish_sequence, &mut state.completed_readbacks);
    state.next_publish_sequence = next_sequence;
    if let Some(controller) = runtime.controller.as_ref() {
        for frame in frames {
            let _ = controller.send_video(frame);
        }
    }
}

fn take_ordered_readback_frames(
    mut next_sequence: u64,
    completed: &mut BTreeMap<u64, Option<VideoFrame>>,
) -> (u64, Vec<VideoFrame>) {
    let mut ordered = Vec::new();
    while let Some(frame) = completed.remove(&next_sequence) {
        next_sequence = next_sequence.saturating_add(1);
        if let Some(frame) = frame {
            ordered.push(frame);
        }
    }
    (next_sequence, ordered)
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
    if !matches!(
        runtime.phase,
        DirectBroadcastPhase::VerifyingTwitch
            | DirectBroadcastPhase::Broadcasting
            | DirectBroadcastPhase::BandwidthTesting
    ) {
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
    labeled_black_rgba_frame(width, height, "SENSITIVE INFORMATION HIDDEN")
}

fn offline_rgba_frame(width: u32, height: u32) -> Vec<u8> {
    labeled_black_rgba_frame(width, height, "OFFLINE")
}

fn labeled_black_rgba_frame(width: u32, height: u32, label: &str) -> Vec<u8> {
    let bytes = usize::try_from(width)
        .unwrap_or(0)
        .saturating_mul(usize::try_from(height).unwrap_or(0))
        .saturating_mul(4);
    let mut rgba = vec![0; bytes];
    for alpha in rgba.iter_mut().skip(3).step_by(4) {
        *alpha = 255;
    }
    draw_centered_label(&mut rgba, width, height, label);
    rgba
}

fn draw_centered_label(rgba: &mut [u8], width: u32, height: u32, label: &str) {
    const GLYPH_WIDTH: u32 = 5;
    const GLYPH_HEIGHT: u32 = 7;
    const GLYPH_GAP: u32 = 1;
    let unscaled_width = u32::try_from(label.chars().count())
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

    for (glyph_index, character) in label.chars().enumerate() {
        let glyph = label_glyph(character);
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

const fn label_glyph(character: char) -> [u8; 7] {
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
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
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
    Ready(PreparedBroadcast),
    Error(String),
}

#[derive(Debug)]
struct PreparedBroadcast {
    target: BroadcastTarget,
    verification: LiveVerificationTarget,
}

#[derive(Clone, Debug)]
struct LiveVerificationTarget {
    client_id: String,
    broadcaster_id: String,
    bandwidth_test: bool,
    token: Option<StoredOAuthToken>,
}

#[derive(Debug)]
enum LiveVerificationEvent {
    Status(String),
    Live,
    Error(String),
}

struct LiveVerification {
    events: Arc<Mutex<Receiver<LiveVerificationEvent>>>,
    cancel: Arc<AtomicBool>,
}

impl LiveVerification {
    fn start(target: LiveVerificationTarget) -> Result<Self> {
        let (sender, receiver) = mpsc::channel();
        let cancel = Arc::new(AtomicBool::new(false));
        let thread_cancel = Arc::clone(&cancel);
        thread::Builder::new()
            .name("stream-town-live-verification".to_owned())
            .spawn(move || {
                let event = verify_twitch_public_stream(&target, &thread_cancel, &sender)
                    .map_or_else(
                        |error| LiveVerificationEvent::Error(format!("{error:#}")),
                        |()| LiveVerificationEvent::Live,
                    );
                if !thread_cancel.load(Ordering::Relaxed) {
                    let _ = sender.send(event);
                }
            })
            .context("failed to spawn the Twitch live-verification worker")?;
        Ok(Self {
            events: Arc::new(Mutex::new(receiver)),
            cancel,
        })
    }

    fn events(&self) -> Vec<LiveVerificationEvent> {
        self.events
            .lock()
            .ok()
            .map(|events| events.try_iter().collect())
            .unwrap_or_default()
    }
}

impl Drop for LiveVerification {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

fn verify_twitch_public_stream(
    target: &LiveVerificationTarget,
    cancel: &AtomicBool,
    events: &mpsc::Sender<LiveVerificationEvent>,
) -> Result<()> {
    let tokio = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to construct the Twitch live-verification runtime")?;
    tokio.block_on(async {
        let oauth = OAuthClient::broadcaster(target.client_id.clone())?;
        let token = target
            .token
            .as_ref()
            .context("Twitch live verification is missing its validated broadcaster token")?;
        let started = Instant::now();
        let mut last_error = None;
        let mut attempt = 0_u32;
        while started.elapsed() < TWITCH_LIVE_VERIFICATION_TIMEOUT {
            if cancel.load(Ordering::Relaxed) {
                bail!("Twitch live verification was cancelled");
            }
            attempt = attempt.saturating_add(1);
            let remaining = TWITCH_LIVE_VERIFICATION_TIMEOUT.saturating_sub(started.elapsed());
            let request_timeout = twitch_live_request_timeout(remaining);
            let status = tokio::time::timeout(
                request_timeout,
                oauth.is_stream_live(token, &target.broadcaster_id),
            )
            .await;
            match status {
                Ok(Ok(true)) => return Ok(()),
                Ok(Ok(false)) => {
                    last_error = None;
                    let _ = events.send(LiveVerificationEvent::Status(format!(
                        "Check {attempt}: Twitch has not listed the channel as live yet"
                    )));
                }
                Ok(Err(error)) => {
                    let error = format!("{error:#}");
                    let _ = events.send(LiveVerificationEvent::Status(format!(
                        "Check {attempt} failed: {error}"
                    )));
                    last_error = Some(error);
                }
                Err(_) => {
                    let error = format!(
                        "Twitch live-status request timed out after {} seconds",
                        request_timeout.as_secs()
                    );
                    let _ = events.send(LiveVerificationEvent::Status(format!(
                        "Check {attempt} failed: {error}"
                    )));
                    last_error = Some(error);
                }
            }
            let remaining = TWITCH_LIVE_VERIFICATION_TIMEOUT.saturating_sub(started.elapsed());
            if !remaining.is_zero() {
                tokio::time::sleep(remaining.min(TWITCH_LIVE_VERIFICATION_INTERVAL)).await;
            }
        }
        if let Some(error) = last_error {
            bail!("Twitch did not confirm the channel as live within 60 seconds; last status check failed: {error}");
        }
        bail!("Twitch did not confirm the channel as live within 60 seconds; the encoder session was stopped instead of reporting a false LIVE state")
    })
}

fn twitch_live_request_timeout(remaining: Duration) -> Duration {
    remaining.min(TWITCH_LIVE_REQUEST_TIMEOUT)
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
    Broadcasting {
        encoder: String,
        rejected_encoders: Vec<String>,
    },
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
    video_consumer_ready: Arc<AtomicBool>,
}

struct WindowCaptureHandler {
    video: Arc<Mutex<Option<VideoFrame>>>,
    metrics: Arc<BroadcastMetrics>,
    stop: Arc<AtomicBool>,
    sensitive_screen: Arc<AtomicBool>,
    video_consumer_ready: Arc<AtomicBool>,
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
            video_consumer_ready: context.flags.video_consumer_ready,
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
            &self.video_consumer_ready,
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
    video_consumer_ready: Arc<AtomicBool>,
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
            video_consumer_ready,
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
    video_consumer_ready: &AtomicBool,
    frame: VideoFrame,
) -> bool {
    if stop.load(Ordering::Relaxed) {
        return false;
    }
    let Ok(mut latest) = video.lock() else {
        metrics.dropped_video.fetch_add(1, Ordering::Relaxed);
        return false;
    };
    // Shutdown publishes the offline card through this same mailbox. Recheck
    // after taking the lock so a readback that passed the first check cannot
    // race in behind it and become the final encoded frame.
    if stop.load(Ordering::Acquire) {
        return false;
    }
    if latest.replace(frame).is_some() && video_consumer_ready.load(Ordering::Relaxed) {
        metrics.replaced_video.fetch_add(1, Ordering::Relaxed);
    }
    metrics.captured_video.fetch_add(1, Ordering::Relaxed);
    true
}

struct BroadcastController {
    audio: SyncSender<AudioInput>,
    video: Arc<Mutex<Option<VideoFrame>>>,
    events: Arc<Mutex<Receiver<WorkerEvent>>>,
    capture_stop: Arc<AtomicBool>,
    graceful_stop: Arc<AtomicBool>,
    stop: Arc<AtomicBool>,
    sensitive_screen: Arc<AtomicBool>,
    video_consumer_ready: Arc<AtomicBool>,
    metrics: Arc<BroadcastMetrics>,
    width: u32,
    height: u32,
}

impl BroadcastController {
    fn start(
        target: BroadcastTarget,
        config: BroadcastConfig,
        window_title: String,
        tidal_routing: Option<&NativeAudioRouting>,
        game_audio_routing: Option<&NativeGameAudioRouting>,
    ) -> Result<Self> {
        let (audio, receiver) = mpsc::sync_channel(AUDIO_QUEUE_CAPACITY);
        let (event_sender, event_receiver) = mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let capture_stop = Arc::new(AtomicBool::new(false));
        let graceful_stop = Arc::new(AtomicBool::new(false));
        let metrics = Arc::new(BroadcastMetrics::default());
        let video = Arc::new(Mutex::new(None));
        let sensitive_screen = Arc::new(AtomicBool::new(false));
        let video_consumer_ready = Arc::new(AtomicBool::new(false));
        let capture_fps = config.frames_per_second;
        let width = u32::from(config.width);
        let height = u32::from(config.height);
        let stream_only = config.render_mode == BroadcastRenderMode::StreamOnly;
        let tidal_audio = stream_only
            .then(|| tidal_routing.map(NativeAudioRouting::subscribe))
            .flatten();
        let game_audio = stream_only
            .then(|| game_audio_routing.map(NativeGameAudioRouting::subscribe))
            .flatten();

        let worker_stop = Arc::clone(&stop);
        let worker_graceful_stop = Arc::clone(&graceful_stop);
        let worker_metrics = Arc::clone(&metrics);
        let worker_video = Arc::clone(&video);
        let worker_video_consumer_ready = Arc::clone(&video_consumer_ready);
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
                    &worker_graceful_stop,
                    &worker_metrics,
                    &worker_video_consumer_ready,
                );
            })
            .context("failed to start the in-process FFmpeg worker")?;

        if !stream_only
            && let Err(error) = start_window_capture(
                &window_title,
                capture_fps,
                Arc::clone(&video),
                Arc::clone(&metrics),
                Arc::clone(&capture_stop),
                Arc::clone(&sensitive_screen),
                Arc::clone(&video_consumer_ready),
                capture_event_sender,
            )
        {
            stop.store(true, Ordering::Relaxed);
            return Err(error);
        }

        let audio_sender = audio.clone();
        let audio_stop = Arc::clone(&capture_stop);
        let worker_abort = Arc::clone(&stop);
        let audio_metrics = Arc::clone(&metrics);
        let audio_events = Arc::new(Mutex::new(event_receiver));
        let audio_spawn = thread::Builder::new()
            .name("stream-town-wasapi".to_owned())
            .spawn(move || {
                if let Err(error) = capture_process_audio(
                    audio_sender,
                    &audio_stop,
                    &audio_metrics,
                    tidal_audio,
                    game_audio,
                ) {
                    worker_abort.store(true, Ordering::Relaxed);
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
            capture_stop,
            graceful_stop,
            stop,
            sensitive_screen,
            video_consumer_ready,
            metrics,
            width,
            height,
        })
    }

    fn send_video(&self, frame: VideoFrame) -> bool {
        publish_latest_video(
            &self.video,
            &self.capture_stop,
            &self.metrics,
            &self.video_consumer_ready,
            frame,
        )
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
        self.capture_stop.store(true, Ordering::Relaxed);
        if let Ok(mut video) = self.video.lock() {
            *video = Some(VideoFrame {
                width: self.width,
                height: self.height,
                pixel_format: VideoPixelFormat::Rgba,
                pixels: offline_rgba_frame(self.width, self.height),
            });
        }
        self.graceful_stop.store(true, Ordering::Release);
    }

    fn request_abort(&self) {
        self.capture_stop.store(true, Ordering::Relaxed);
        self.stop.store(true, Ordering::Relaxed);
        let _ = self.audio.try_send(AudioInput::Stop);
    }
}

impl Drop for BroadcastController {
    fn drop(&mut self) {
        self.request_abort();
    }
}

fn capture_process_audio(
    audio: SyncSender<AudioInput>,
    stop: &AtomicBool,
    metrics: &BroadcastMetrics,
    tidal_audio: Option<Receiver<NativeAudioFrame>>,
    game_audio: Option<NativeGameAudioMix>,
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
    let mut tidal_mix = TidalPcmMix::new(tidal_audio);
    let mut game_mix = game_audio;
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
            let mut samples = raw
                .chunks_exact(4)
                .map(|sample| f32::from_le_bytes([sample[0], sample[1], sample[2], sample[3]]))
                .collect::<Vec<_>>();
            if let Some(game_mix) = game_mix.as_mut() {
                game_mix.mix_into(&mut samples);
            }
            tidal_mix.mix_into(&mut samples);
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

struct TidalPcmMix {
    receiver: Option<Receiver<NativeAudioFrame>>,
    source: VecDeque<[f32; 2]>,
    sample_rate: u32,
    source_position: f32,
}

impl TidalPcmMix {
    fn new(receiver: Option<Receiver<NativeAudioFrame>>) -> Self {
        Self {
            receiver,
            source: VecDeque::new(),
            sample_rate: AUDIO_SAMPLE_RATE,
            source_position: 0.0,
        }
    }

    fn receive(&mut self) {
        let Some(receiver) = &self.receiver else {
            return;
        };
        while let Ok(frame) = receiver.try_recv() {
            if frame.sample_rate == 0 {
                continue;
            }
            if frame.sample_rate != self.sample_rate {
                self.source.clear();
                self.source_position = 0.0;
                self.sample_rate = frame.sample_rate;
            }
            self.source.extend(
                frame
                    .samples
                    .chunks_exact(AUDIO_CHANNELS)
                    .map(|sample| [sample[0], sample[1]]),
            );
        }
        // A stalled encoder should recover at the current music position, not
        // replay an arbitrarily old local-monitor buffer.
        let maximum_frames = usize::try_from(self.sample_rate / 2).unwrap_or(usize::MAX);
        if self.source.len() > maximum_frames {
            let stale = self.source.len() - maximum_frames;
            self.source.drain(..stale);
            self.source_position = 0.0;
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn mix_into(&mut self, output: &mut [f32]) {
        self.receive();
        let source_step = self.sample_rate as f32 / AUDIO_SAMPLE_RATE as f32;
        for output_frame in output.chunks_exact_mut(AUDIO_CHANNELS) {
            let (Some(current), Some(next)) = (self.source.front(), self.source.get(1)) else {
                break;
            };
            let fraction = self.source_position;
            let music = [
                current[0] + (next[0] - current[0]) * fraction,
                current[1] + (next[1] - current[1]) * fraction,
            ];
            output_frame[0] = (output_frame[0] + music[0]).clamp(-1.0, 1.0);
            output_frame[1] = (output_frame[1] + music[1]).clamp(-1.0, 1.0);
            self.source_position += source_step;
            while self.source_position >= 1.0 && self.source.len() > 1 {
                self.source.pop_front();
                self.source_position -= 1.0;
            }
        }
    }
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
    graceful_stop: &AtomicBool,
    metrics: &BroadcastMetrics,
    video_consumer_ready: &AtomicBool,
) {
    let mut reconnect_delay = 1_u64;
    loop {
        if stop.load(Ordering::Relaxed) {
            let _ = events.send(WorkerEvent::Stopped);
            return;
        }
        let _ = events.send(WorkerEvent::Connecting);
        let mut session_published = false;
        let result = encode_broadcast_session(
            &target,
            &config,
            &receiver,
            &video,
            stop,
            graceful_stop,
            metrics,
            events,
            video_consumer_ready,
            &mut session_published,
        );
        video_consumer_ready.store(false, Ordering::Relaxed);
        match result {
            Ok(SessionEnd::Stopped | SessionEnd::InputClosed) => {
                let _ = events.send(WorkerEvent::Stopped);
                return;
            }
            Err(error) => {
                if graceful_stop.load(Ordering::Acquire) {
                    let _ = events.send(WorkerEvent::Stopped);
                    return;
                }
                let wait_seconds = reconnect_wait_seconds(&mut reconnect_delay, session_published);
                let message = redact_broadcast_target(&format!("{error:#}"), &target.url);
                let _ = events.send(WorkerEvent::Reconnecting(message));
                for _ in 0..wait_seconds.saturating_mul(4) {
                    if stop.load(Ordering::Relaxed) {
                        let _ = events.send(WorkerEvent::Stopped);
                        return;
                    }
                    thread::sleep(Duration::from_millis(250));
                }
            }
        }
    }
}

fn redact_broadcast_target(message: &str, target_url: &str) -> String {
    let mut redacted = message.replace(target_url, "[RTMP target redacted]");
    if let Some(secret) = target_url
        .rsplit('/')
        .next()
        .filter(|secret| !secret.is_empty())
    {
        redacted = redacted.replace(secret, "[stream key redacted]");
    }
    redacted
}

fn reconnect_wait_seconds(delay: &mut u64, session_published: bool) -> u64 {
    if session_published {
        *delay = 1;
    }
    let wait = *delay;
    *delay = delay.saturating_mul(2).min(MAX_RECONNECT_DELAY_SECONDS);
    wait
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct VideoEncoderSelection {
    name: String,
    hardware: bool,
    rejections: Vec<String>,
}

impl VideoEncoderSelection {
    fn display_name(&self) -> String {
        format!(
            "{} ({})",
            self.name,
            if self.hardware {
                "hardware"
            } else {
                "software"
            }
        )
    }
}

impl fmt::Display for VideoEncoderSelection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.display_name())
    }
}

fn encode_broadcast_session(
    target: &BroadcastTarget,
    config: &BroadcastConfig,
    receiver: &Receiver<AudioInput>,
    video_mailbox: &Mutex<Option<VideoFrame>>,
    stop: &AtomicBool,
    graceful_stop: &AtomicBool,
    metrics: &BroadcastMetrics,
    events: &mpsc::Sender<WorkerEvent>,
    video_consumer_ready: &AtomicBool,
    session_published: &mut bool,
) -> Result<SessionEnd> {
    ffmpeg::init().context("could not initialize the linked FFmpeg libraries")?;
    ffmpeg::log::set_level(ffmpeg::log::Level::Quiet);
    let (mut encoder, encoder_selection) = BroadcastEncoder::open(target, config)?;
    video_consumer_ready.store(true, Ordering::Relaxed);
    if discard_pending_audio(receiver, metrics) {
        encoder.finish()?;
        return Ok(SessionEnd::Stopped);
    }
    let mut encoder_selection = Some(encoder_selection);
    let mut cadence = VideoCadence::new(config.frames_per_second);
    let mut latest_video = take_latest_video(video_mailbox);
    let mut graceful_deadline = None;
    if latest_video.is_some() {
        cadence.start(Instant::now());
    }
    loop {
        if stop.load(Ordering::Relaxed) {
            encoder.finish()?;
            return Ok(SessionEnd::Stopped);
        }
        if graceful_stop.load(Ordering::Acquire) && graceful_deadline.is_none() {
            latest_video = take_latest_video(video_mailbox).or(latest_video);
            if latest_video.is_none() {
                encoder.finish()?;
                return Ok(SessionEnd::Stopped);
            }
            cadence.start(Instant::now());
            graceful_deadline = Some(Instant::now() + OFFLINE_FRAME_HOLD);
        } else if graceful_deadline.is_none()
            && let Some(video) = take_latest_video(video_mailbox)
        {
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
            }
            let encode_started = Instant::now();
            let published_packets = encoder.encode_video(video, tick.pts)?;
            metrics.observe_video_encode_latency(encode_started.elapsed());
            metrics.encoded_video.fetch_add(1, Ordering::Relaxed);
            if published_packets > 0
                && let Some(selection) = encoder_selection.take()
            {
                *session_published = true;
                let _ = events.send(WorkerEvent::Broadcasting {
                    encoder: selection.display_name(),
                    rejected_encoders: selection.rejections,
                });
            }
            continue;
        }
        if graceful_deadline.is_some_and(|deadline| Instant::now() >= deadline) {
            encoder.finish()?;
            return Ok(SessionEnd::Stopped);
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
    fn open(
        target: &BroadcastTarget,
        config: &BroadcastConfig,
    ) -> Result<(Self, VideoEncoderSelection)> {
        let mut output_options = Dictionary::new();
        // FFmpeg protocol timeout is in microseconds. Keep a dead ingest from
        // pinning the encoder worker indefinitely; reconnect owns the retry.
        output_options.set("rw_timeout", "15000000");
        if target.url.starts_with("rtmp://") || target.url.starts_with("rtmps://") {
            // Publish as a live source and flush packets promptly. The default
            // protocol buffering is useful for playback clients, but it adds
            // avoidable latency and turns a brief ingest stall into a visible
            // burst for an always-live game producer.
            output_options.set("rtmp_live", "live");
            output_options.set("tcp_nodelay", "1");
            output_options.set("flush_packets", "1");
            output_options.set("flvflags", "no_duration_filesize");
        }
        let mut output = format::output_as_with(&target.url, "flv", output_options)
            .context("could not connect to the selected Twitch RTMP ingest")?;
        let global_header = output
            .format()
            .flags()
            .contains(format::Flags::GLOBAL_HEADER);
        let (video, video_codec, encoder_selection, video_input_format) =
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
            encoder_selection,
        ))
    }

    fn encode_video(&mut self, video: &VideoFrame, pts: i64) -> Result<u64> {
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

    fn drain_video(&mut self) -> Result<u64> {
        let mut packet = Packet::empty();
        let mut published = 0_u64;
        while self.video.receive_packet(&mut packet).is_ok() {
            packet.set_stream(self.video_stream);
            packet.rescale_ts(self.video.time_base(), self.video_time_base);
            let pts = packet.pts();
            let dts = packet.dts();
            let duration = packet.duration();
            packet
                .write_interleaved(&mut self.output)
                .with_context(|| {
                    format!(
                        "could not publish an H.264 packet to Twitch (pts={pts:?}, dts={dts:?}, duration={duration})"
                    )
                })?;
            published = published.saturating_add(1);
        }
        Ok(published)
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

fn configure_amf_quality(options: &mut Dictionary<'_>) {
    options.set("profile", "high");
    options.set("usage", "lowlatency_high_quality");
    options.set("quality", "quality");
    // Retain broad AMF-driver compatibility for Twitch's constant-rate
    // contract while spending the RX 7800 XT's quality budget on stable fine
    // detail. The linked AMF build rejects the newer HQCBR enum at open time.
    options.set("rc", "cbr");
    options.set("enforce_hrd", "1");
    options.set("filler_data", "1");
    options.set("frame_skipping", "0");
    options.set("forced_idr", "1");
    // AMF's reordered B-frame path produced periodic chroma/luminance pulses
    // during large lighting changes and occasionally handed the FLV muxer an
    // invalid reordered packet. Twitch permits zero B-frames. A monotonic IP
    // stream is lower-latency and removes both failure modes while retaining
    // the GPU encoder, CBR, and the required two-second IDR cadence.
    options.set("max_b_frames", "0");
    options.set("bf", "0");
    options.set("coder", "cabac");
    options.set("me_half_pel", "1");
    options.set("me_quarter_pel", "1");
    options.set("latency", "1");
    options.set("async_depth", "2");
    // The shipping terrain grid is mostly static, high-frequency detail.
    // AMF's automatic static-scene and adaptive-mini-GOP decisions repeatedly
    // starved that detail between Twitch's required two-second IDR frames.
    options.set("vbaq", "1");
    options.set("preanalysis", "0");
}

fn open_video_encoder(
    config: &BroadcastConfig,
    global_header: bool,
) -> Result<(
    encoder::video::Encoder,
    Codec,
    VideoEncoderSelection,
    ffmpeg::format::Pixel,
)> {
    let mut failures = Vec::new();
    for &name in encoder_candidates(config.encoder) {
        let Some(codec) = encoder::find_by_name(name) else {
            failures.push(format!("{name}: not included in the linked FFmpeg build"));
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
            // Every live backend uses decode-order timestamps. In particular,
            // AMD's reordered B-frame path caused intermittent FLV publish
            // failures and visible dark-scene pulsing on the RX 7800 XT.
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
                    configure_amf_quality(&mut options);
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
            Ok(video) => {
                let selection = VideoEncoderSelection {
                    name: name.to_owned(),
                    hardware: encoder_is_hardware(name),
                    rejections: failures,
                };
                return Ok((video, codec, selection, input_format));
            }
            Err(error) => failures.push(format!("{name}: {error:#}")),
        }
    }
    bail!(
        "no requested H.264 encoder could be opened ({})",
        failures.join("; ")
    )
}

fn encoder_is_hardware(name: &str) -> bool {
    matches!(name, "h264_nvenc" | "h264_qsv" | "h264_amf" | "h264_mf")
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
    pub rejected_encoders: Vec<String>,
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
        selected_encoder: selected_encoder.display_name(),
        rejected_encoders: selected_encoder.rejections,
        process_audio_capture_available,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconnect_diagnostics_redact_the_rtmp_target_and_stream_key() {
        let target = "rtmp://live.example.invalid/app/live_secret_value";
        let error = format!("could not write to {target}; key live_secret_value rejected");
        let redacted = redact_broadcast_target(&error, target);
        assert!(!redacted.contains(target));
        assert!(!redacted.contains("live_secret_value"));
        assert!(redacted.contains("[RTMP target redacted]"));
    }

    #[test]
    fn direct_broadcast_diagnostics_are_persisted_without_a_live_session() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("direct-broadcast.log");
        append_direct_broadcast_diagnostic_to(&path, "WARN", "event=test cause=network").unwrap();
        let written = fs::read_to_string(path).unwrap();
        assert!(written.contains("WARN event=test cause=network"));
    }

    #[test]
    fn enemy_operator_countdown_matches_the_unity_day_boundary() {
        let time = stream_town_domain::TimeCycleConfig::default();
        assert_eq!(seconds_until_enemy_night(&time, 343.0), 2_055);
        assert_eq!(format_minutes_seconds(2_055), "34:15");
        assert_eq!(seconds_until_enemy_night(&time, 2_400.0), 0);
    }

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
    fn stream_only_music_tap_mixes_pre_monitor_pcm_into_wasapi_audio() {
        let (sender, receiver) = mpsc::channel();
        sender
            .send(NativeAudioFrame {
                sample_rate: AUDIO_SAMPLE_RATE,
                samples: vec![0.25, -0.5, 0.5, -0.25, 0.75, 0.25].into(),
            })
            .unwrap();
        let mut mix = TidalPcmMix::new(Some(receiver));
        let mut output = vec![0.1; 4];

        mix.mix_into(&mut output);

        assert_eq!(output, vec![0.35, -0.4, 0.6, -0.15]);
    }

    #[test]
    fn stream_only_music_tap_resamples_to_the_twitch_clock() {
        let (sender, receiver) = mpsc::channel();
        sender
            .send(NativeAudioFrame {
                sample_rate: 24_000,
                samples: vec![0.0, 0.0, 1.0, 1.0, 0.0, 0.0].into(),
            })
            .unwrap();
        let mut mix = TidalPcmMix::new(Some(receiver));
        let mut output = vec![0.0; 6];

        mix.mix_into(&mut output);

        assert_eq!(output, vec![0.0, 0.0, 0.5, 0.5, 1.0, 1.0]);
    }

    #[test]
    fn public_ingest_url_does_not_enable_bandwidth_test_mode() {
        let url = build_ingest_url(
            "rtmp://example.invalid/app/{stream_key}",
            "live_secret_key",
            false,
        )
        .unwrap();
        assert_eq!(url, "rtmp://example.invalid/app/live_secret_key");
        assert!(!url.contains("bandwidthtest"));
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
    fn offline_frame_is_opaque_black_with_a_centered_white_notice() {
        let width = 320;
        let height = 180;
        let frame = offline_rgba_frame(width, height);
        assert_eq!(frame.len(), usize::try_from(width * height * 4).unwrap());
        assert!(frame.chunks_exact(4).all(|pixel| pixel[3] == 255));
        assert!(
            frame
                .chunks_exact(4)
                .any(|pixel| pixel == [255, 255, 255, 255])
        );
    }

    #[test]
    fn stream_only_game_audio_is_muted_locally_and_mixed_before_the_monitor() {
        let routing = NativeGameAudioRouting::default();
        routing.set_stream_output_enabled(true);
        routing.set_local_monitor_enabled(false);
        let wav = crate::procedural_seagull_call_wav(0, AUDIO_SAMPLE_RATE);
        let mut mix = routing.subscribe();
        routing.play_pcm16_wav("test:seagull", &wav, 1.0);
        let mut output = vec![0.0; AUDIO_FRAME_SAMPLES * AUDIO_CHANNELS];

        mix.mix_into(&mut output);

        assert!(!routing.local_monitor_enabled());
        assert!(output.iter().any(|sample| sample.abs() > f32::EPSILON));
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
    fn explicit_automatic_start_uses_the_normal_broadcast_configuration_path_once() {
        let mut config = stream_town_domain::GameConfig::default();
        config.twitch.broadcast.enabled = true;
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(RuntimeConfig(config))
            .insert_resource(AutomaticBroadcastStart { requested: true })
            .init_resource::<SensitiveScreenActive>()
            .add_plugins(DirectTwitchBroadcastPlugin);

        app.update();

        assert!(matches!(
            &app.world().resource::<DirectBroadcastRuntime>().phase,
            DirectBroadcastPhase::Error(error)
                if error == "direct broadcast requires the Twitch public client ID"
        ));
        assert!(!app.world().resource::<AutomaticBroadcastStart>().requested);
        let control = app.world().resource::<DirectBroadcastControl>();
        assert!(!control.restart_requested);
        assert!(!control.stop_requested);
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
            DirectBroadcastPhase::VerifyingTwitch,
            DirectBroadcastPhase::Broadcasting,
            DirectBroadcastPhase::BandwidthTesting,
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
    fn gpu_readbacks_are_published_in_render_order_even_when_they_finish_out_of_order() {
        let frame = |value| VideoFrame {
            width: 1,
            height: 1,
            pixel_format: VideoPixelFormat::Bgra,
            pixels: vec![value; 4],
        };
        let mut completed = BTreeMap::from([(2, Some(frame(2))), (1, None)]);

        let (next_sequence, frames) = take_ordered_readback_frames(0, &mut completed);

        assert_eq!(next_sequence, 0);
        assert!(frames.is_empty());
        completed.insert(0, Some(frame(0)));

        let (next_sequence, frames) = take_ordered_readback_frames(next_sequence, &mut completed);

        assert_eq!(next_sequence, 3);
        assert_eq!(
            frames
                .into_iter()
                .map(|frame| frame.pixels[0])
                .collect::<Vec<_>>(),
            [0, 2]
        );
        assert!(completed.is_empty());
    }

    #[test]
    fn stalled_gpu_readback_cannot_accumulate_full_resolution_frames() {
        let mut app = App::new();
        let stalled = app.world_mut().spawn(StreamOnlyReadbackArmed).id();
        let frame = |value| VideoFrame {
            width: 1,
            height: 1,
            pixel_format: VideoPixelFormat::Bgra,
            pixels: vec![value; 4],
        };
        let mut capture = StreamOnlyCaptureState::default();
        capture
            .readback_requests
            .insert(stalled, (0, Instant::now()));
        capture.completed_readbacks = (1..=MAX_STREAM_COMPLETED_READBACKS)
            .map(|sequence| {
                (
                    u64::try_from(sequence).unwrap(),
                    Some(frame(u8::try_from(sequence).unwrap())),
                )
            })
            .collect();
        app.insert_resource(DirectBroadcastRuntime::default())
            .insert_resource(capture)
            .add_systems(Update, cleanup_completed_stream_only_readbacks);

        app.update();

        let capture = app.world().resource::<StreamOnlyCaptureState>();
        assert!(capture.readback_requests.is_empty());
        assert!(capture.completed_readbacks.is_empty());
        assert_eq!(
            capture.next_publish_sequence,
            u64::try_from(MAX_STREAM_COMPLETED_READBACKS).unwrap() + 1
        );
        assert!(app.world().get_entity(stalled).is_err());
    }

    #[test]
    fn gpu_readback_cadence_never_captures_faster_than_the_stream_rate() {
        let started = Instant::now();
        let mut next = None;

        assert!(stream_readback_due(&mut next, started, 30));
        assert!(!stream_readback_due(
            &mut next,
            started + Duration::from_millis(20),
            30
        ));
        assert!(stream_readback_due(
            &mut next,
            started + Duration::from_millis(34),
            30
        ));
        assert!(!stream_readback_due(
            &mut next,
            started + Duration::from_millis(50),
            30
        ));
        assert!(stream_readback_due(
            &mut next,
            started + Duration::from_millis(68),
            30
        ));
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
    fn operator_only_reports_live_after_twitch_verification() {
        assert_eq!(
            operator_live_button_label(&DirectBroadcastPhase::Connecting),
            "● NOT LIVE · CANCEL START"
        );
        assert_eq!(
            operator_live_button_label(&DirectBroadcastPhase::VerifyingTwitch),
            "● VERIFYING TWITCH · CANCEL"
        );
        assert_eq!(
            operator_live_button_label(&DirectBroadcastPhase::Broadcasting),
            "● LIVE · END STREAM"
        );
    }

    #[test]
    fn operator_panel_uses_compact_telemetry_and_bottom_left_live_control() {
        fn spawn_test_view(mut commands: Commands) {
            let camera = commands.spawn_empty().id();
            let _ = spawn_stream_operator_view(&mut commands, camera, None);
        }

        let mut app = App::new();
        app.add_systems(Startup, spawn_test_view);
        app.update();

        let world = app.world_mut();
        let mut telemetry =
            world.query_filtered::<(&TextFont, &Node), With<StreamOperatorInfoText>>();
        let (font, node) = telemetry.single(world).unwrap();
        assert_eq!(font.font_size, FontSize::Px(15.0));
        assert_eq!(node.top, px(92));
        let mut live_button = world.query_filtered::<&Node, With<StreamOperatorLiveButton>>();
        let node = live_button.single(world).unwrap();
        assert_eq!(node.left, px(48));
        assert_eq!(node.bottom, px(42));
        assert_eq!(node.top, Val::Auto);
        let mut restart_button = world.query_filtered::<&Node, With<StreamOperatorRestartButton>>();
        let node = restart_button.single(world).unwrap();
        assert_eq!(node.left, px(292));
        assert_eq!(node.bottom, px(42));
        let mut chat_inputs = world.query_filtered::<Entity, With<StreamOperatorChatInput>>();
        assert_eq!(chat_inputs.iter(world).count(), 1);
        let mut chat_rows = world.query_filtered::<Entity, With<StreamOperatorChatRow>>();
        assert_eq!(chat_rows.iter(world).count(), OPERATOR_CHAT_VISIBLE_ROWS);
        let mut chat_badges = world.query_filtered::<Entity, With<StreamOperatorChatBadge>>();
        assert_eq!(
            chat_badges.iter(world).count(),
            OPERATOR_CHAT_VISIBLE_ROWS * 3
        );
        let mut scroll_actions =
            world.query_filtered::<Entity, With<StreamOperatorChatScrollAction>>();
        assert_eq!(scroll_actions.iter(world).count(), 2);
        let mut scroll_thumb =
            world.query_filtered::<Entity, With<StreamOperatorChatScrollThumb>>();
        assert_eq!(scroll_thumb.iter(world).count(), 1);
        let mut moderation = world.query_filtered::<Entity, Or<(
            With<StreamOperatorChatTimeoutButton>,
            With<StreamOperatorChatBanButton>,
        )>>();
        assert_eq!(moderation.iter(world).count(), 2);
        let mut settings = world.query_filtered::<Entity, With<StreamOperatorSettingAction>>();
        assert_eq!(settings.iter(world).count(), 12);
        let mut text = world.query::<&Text>();
        assert!(
            text.iter(world)
                .all(|text| !text.0.contains("This local operator panel is excluded"))
        );
        assert!(
            text.iter(world)
                .all(|text| !text.0.contains("Preview · 320 × 180"))
        );
        assert!(
            text.iter(world)
                .any(|text| text.0.contains("RESTART STREAM"))
        );
    }

    #[test]
    fn operator_restart_button_requests_a_stream_restart() {
        let mut app = App::new();
        app.init_resource::<DirectBroadcastControl>()
            .add_systems(Update, stream_operator_restart_button);
        app.world_mut().spawn((
            StreamOperatorRestartButton,
            Interaction::Pressed,
            BackgroundColor(Color::NONE),
        ));

        app.update();

        assert!(
            app.world()
                .resource::<DirectBroadcastControl>()
                .restart_requested_for_test()
        );
    }

    #[test]
    fn twitch_live_requests_cannot_outlive_the_verification_deadline() {
        assert_eq!(
            twitch_live_request_timeout(Duration::from_secs(30)),
            TWITCH_LIVE_REQUEST_TIMEOUT
        );
        assert_eq!(
            twitch_live_request_timeout(Duration::from_secs(3)),
            Duration::from_secs(3)
        );
    }

    #[test]
    fn bandwidth_test_never_claims_to_be_publicly_live() {
        let mut runtime = DirectBroadcastRuntime {
            verification_target: Some(LiveVerificationTarget {
                client_id: "client".to_owned(),
                broadcaster_id: "42".to_owned(),
                bandwidth_test: true,
                token: None,
            }),
            ..default()
        };
        begin_twitch_live_verification(&mut runtime);
        assert_eq!(runtime.phase, DirectBroadcastPhase::BandwidthTesting);
        assert_eq!(
            operator_live_button_label(&runtime.phase),
            "● BANDWIDTH TEST · END TEST"
        );
        assert!(runtime.live_verification.is_none());
    }

    #[test]
    fn ending_stream_returns_the_operator_to_main_menu_after_shutdown() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
            .init_state::<crate::GameState>()
            .insert_resource(RuntimeConfig(stream_town_domain::GameConfig::default()))
            .init_resource::<SensitiveScreenActive>()
            .add_plugins(DirectTwitchBroadcastPlugin);
        app.world_mut()
            .resource_mut::<NextState<crate::GameState>>()
            .set(crate::GameState::InGame);
        app.update();
        app.world_mut()
            .resource_mut::<DirectBroadcastRuntime>()
            .phase = DirectBroadcastPhase::Connecting;
        app.world_mut()
            .resource_mut::<DirectBroadcastControl>()
            .request_stop_and_return_to_main_menu();

        app.update();
        app.update();

        assert_eq!(
            *app.world().resource::<State<crate::GameState>>().get(),
            crate::GameState::MainMenu
        );
        assert!(matches!(
            &app.world()
                .resource::<DirectBroadcastControl>()
                .stop_disposition,
            BroadcastStopDisposition::Stay
        ));
    }

    #[test]
    fn closing_the_operator_window_requests_a_graceful_game_exit() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(RuntimeConfig(stream_town_domain::GameConfig::default()))
            .init_resource::<SensitiveScreenActive>()
            .add_plugins(DirectTwitchBroadcastPlugin);
        let operator = app.world_mut().spawn(StreamOperatorWindow).id();
        app.world_mut()
            .write_message(WindowCloseRequested { window: operator });

        app.update();

        let runtime = app.world().resource::<DirectBroadcastRuntime>();
        let control = app.world().resource::<DirectBroadcastControl>();
        assert_eq!(runtime.phase, DirectBroadcastPhase::Stopped);
        assert!(matches!(
            &control.stop_disposition,
            BroadcastStopDisposition::Stay
        ));
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
    fn stopped_capture_cannot_replace_the_terminal_mailbox_frame() {
        let terminal = VideoFrame {
            width: 1,
            height: 1,
            pixel_format: VideoPixelFormat::Rgba,
            pixels: vec![0, 0, 0, 255],
        };
        let mailbox = Mutex::new(Some(terminal.clone()));
        let stop = AtomicBool::new(true);
        let metrics = BroadcastMetrics::default();
        let ready = AtomicBool::new(true);

        assert!(!publish_latest_video(
            &mailbox,
            &stop,
            &metrics,
            &ready,
            VideoFrame {
                pixels: vec![255, 0, 0, 255],
                ..terminal.clone()
            }
        ));
        assert_eq!(take_latest_video(&mailbox).unwrap().pixels, terminal.pixels);
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
            capture_stop: Arc::new(AtomicBool::new(false)),
            graceful_stop: Arc::new(AtomicBool::new(false)),
            stop: Arc::new(AtomicBool::new(false)),
            sensitive_screen: Arc::new(AtomicBool::new(false)),
            video_consumer_ready: Arc::new(AtomicBool::new(true)),
            metrics: Arc::clone(&metrics),
            width: 1,
            height: 1,
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
        assert_eq!(snapshot.dropped_video, 0);
        assert_eq!(
            take_latest_video(&controller.video).unwrap().pixels,
            [5, 6, 7, 8]
        );
    }

    #[test]
    fn graceful_stop_replaces_capture_with_the_offline_frame_before_aborting() {
        let (audio, _audio_receiver) = mpsc::sync_channel(1);
        let (_event_sender, event_receiver) = mpsc::channel();
        let controller = BroadcastController {
            audio,
            video: Arc::new(Mutex::new(None)),
            events: Arc::new(Mutex::new(event_receiver)),
            capture_stop: Arc::new(AtomicBool::new(false)),
            graceful_stop: Arc::new(AtomicBool::new(false)),
            stop: Arc::new(AtomicBool::new(false)),
            sensitive_screen: Arc::new(AtomicBool::new(false)),
            video_consumer_ready: Arc::new(AtomicBool::new(true)),
            metrics: Arc::new(BroadcastMetrics::default()),
            width: 320,
            height: 180,
        };

        controller.request_stop();

        assert!(controller.capture_stop.load(Ordering::Relaxed));
        assert!(controller.graceful_stop.load(Ordering::Acquire));
        assert!(!controller.stop.load(Ordering::Relaxed));
        let frame = take_latest_video(&controller.video).unwrap();
        assert_eq!(frame.pixel_format, VideoPixelFormat::Rgba);
        assert_eq!((frame.width, frame.height), (320, 180));
        assert!(
            frame
                .pixels
                .chunks_exact(4)
                .any(|pixel| pixel == [255, 255, 255, 255])
        );
    }

    #[test]
    fn reconnect_opening_replacements_do_not_pollute_live_health_metrics() {
        let mailbox = Mutex::new(None);
        let stop = AtomicBool::new(false);
        let ready = AtomicBool::new(false);
        let metrics = BroadcastMetrics::default();
        let frame = || VideoFrame {
            width: 1,
            height: 1,
            pixel_format: VideoPixelFormat::Bgra,
            pixels: vec![1, 2, 3, 4],
        };

        assert!(publish_latest_video(
            &mailbox,
            &stop,
            &metrics,
            &ready,
            frame()
        ));
        assert!(publish_latest_video(
            &mailbox,
            &stop,
            &metrics,
            &ready,
            frame()
        ));
        assert_eq!(metrics.snapshot().replaced_video, 0);

        ready.store(true, Ordering::Relaxed);
        assert!(publish_latest_video(
            &mailbox,
            &stop,
            &metrics,
            &ready,
            frame()
        ));
        assert_eq!(metrics.snapshot().replaced_video, 1);
    }

    #[test]
    fn recovered_session_resets_exponential_reconnect_delay() {
        let mut delay = 8;
        assert_eq!(reconnect_wait_seconds(&mut delay, true), 1);
        assert_eq!(delay, 2);
        assert_eq!(reconnect_wait_seconds(&mut delay, false), 2);
        assert_eq!(delay, 4);
    }

    #[test]
    fn amf_quality_profile_keeps_static_grid_detail_between_keyframes() {
        let mut options = Dictionary::new();
        configure_amf_quality(&mut options);

        assert_eq!(options.get("usage"), Some("lowlatency_high_quality"));
        assert_eq!(options.get("quality"), Some("quality"));
        assert_eq!(options.get("rc"), Some("cbr"));
        assert_eq!(options.get("vbaq"), Some("1"));
        assert_eq!(options.get("preanalysis"), Some("0"));
        assert_eq!(options.get("forced_idr"), Some("1"));
        assert_eq!(options.get("max_b_frames"), Some("0"));
        assert_eq!(options.get("bf"), Some("0"));
        assert_eq!(options.get("coder"), Some("cabac"));
        assert_eq!(options.get("me_half_pel"), Some("1"));
        assert_eq!(options.get("me_quarter_pel"), Some("1"));
        assert_eq!(options.get("latency"), Some("1"));
        assert_eq!(options.get("async_depth"), Some("2"));
    }

    #[test]
    #[ignore = "local 1080p60 hardware-encoder throughput diagnostic"]
    fn configured_1080p60_encoder_sustains_realtime_output() {
        let directory = tempfile::tempdir().unwrap();
        let output = std::env::var_os("STREAM_TOWN_BROADCAST_DIAGNOSTIC_OUTPUT").map_or_else(
            || directory.path().join("direct-broadcast-1080p60.flv"),
            std::path::PathBuf::from,
        );
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
        if selected.name == "h264_amf" {
            let encoded_bytes = std::fs::metadata(&output).unwrap().len();
            let expected_bytes =
                u64::from(config.video_bitrate_kbps) * 1_000 * u64::from(frame_count)
                    / u64::from(config.frames_per_second)
                    / 8;
            assert!(
                encoded_bytes >= expected_bytes * 9 / 10,
                "AMF CBR output was only {encoded_bytes} bytes; expected about {expected_bytes}"
            );
        }
        let elapsed = started.elapsed().as_secs_f64();
        let frames_per_second = f64::from(frame_count) / elapsed;
        eprintln!(
            "1080p60 local encoder diagnostic: {selected}, {frames_per_second:.1} FPS, {:.2} ms/frame; rejected before selection: {}",
            elapsed * 1_000.0 / f64::from(frame_count),
            if selected.rejections.is_empty() {
                "none".to_owned()
            } else {
                selected.rejections.join(" | ")
            }
        );
        assert!(
            frames_per_second >= 60.0,
            "{selected} only sustained {frames_per_second:.1} FPS"
        );
    }

    #[test]
    fn linked_ffmpeg_encodes_h264_aac_flv_without_a_subprocess() {
        let directory = tempfile::tempdir().unwrap();
        let output = std::env::var_os("STREAM_TOWN_BROADCAST_DIAGNOSTIC_OUTPUT").map_or_else(
            || directory.path().join("direct-broadcast-smoke.flv"),
            std::path::PathBuf::from,
        );
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
        assert!(!selected.name.is_empty());
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
        let mut published_video_packets = 0_u64;
        for video_pts in 0..15_i64 {
            published_video_packets = published_video_packets.saturating_add(
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
                    .unwrap(),
            );
            let samples = vec![0.0; AUDIO_FRAME_SAMPLES * AUDIO_CHANNELS];
            encoder
                .encode_audio(AudioFrame {
                    pts: audio_pts,
                    samples,
                })
                .unwrap();
            audio_pts += i64::try_from(AUDIO_FRAME_SAMPLES).unwrap();
        }
        assert!(
            published_video_packets > 0,
            "the live handshake requires a video packet before verification begins"
        );
        encoder.finish().unwrap();
        assert!(std::fs::metadata(output).unwrap().len() > 1_024);
    }
}
