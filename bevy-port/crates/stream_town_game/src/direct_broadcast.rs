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
    collections::{HashMap, VecDeque},
    fmt,
    fs::{self, OpenOptions},
    io::Write as _,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
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
        Render, RenderApp, RenderSystems,
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets as GpuRenderAssets,
        render_resource::{
            Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, MapMode,
            PollType, TexelCopyBufferInfo, TexelCopyBufferLayout, TextureFormat, TextureUsages,
        },
        renderer::{RenderDevice, RenderQueue},
        texture::GpuImage,
    },
    window::{
        CursorOptions, PresentMode, PrimaryWindow, WindowCloseRequested, WindowRef,
        WindowResolution,
    },
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
use wasapi::{
    AudioClient, Direction, SampleType, StreamMode, WaveFormat, initialize_mta, initialize_sta,
};
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
const BROADCAST_IO_TIMEOUT_MICROSECONDS: &str = "3000000";
// At roughly 77 interleaved audio/video packets per second this gives the RTMP
// writer more than six seconds to absorb a transient ingest stall without
// blocking capture or the hardware encoder.
const BROADCAST_FIFO_QUEUE_PACKETS: &str = "512";
const STREAM_CAPTURE_STAGING_BUFFERS: usize = 4;
const STREAM_CAPTURE_CPU_QUEUE_CAPACITY: usize = 2;
const STREAM_CAPTURE_RECYCLED_PIXEL_BUFFERS: usize =
    STREAM_CAPTURE_STAGING_BUFFERS + STREAM_CAPTURE_CPU_QUEUE_CAPACITY + 2;
const ENCODER_VIDEO_FRAME_BUFFERS: usize = 4;
const STREAM_OPERATOR_INFO_REFRESH_INTERVAL: Duration = Duration::from_secs(1);
fn stream_operator_window() -> Window {
    Window {
        title: "Stream Town — Operator".to_owned(),
        resolution: WindowResolution::new(OPERATOR_WINDOW_WIDTH, OPERATOR_WINDOW_HEIGHT),
        resizable: false,
        // The dashboard must not pace the offscreen stream render. In particular,
        // remote and low-refresh displays can otherwise cap capture at about 30 Hz,
        // leaving no timing margin for a fixed 30 FPS GPU readback cadence.
        present_mode: PresentMode::AutoNoVsync,
        ..default()
    }
}

fn stream_render_update_mode(frames_per_second: u8) -> UpdateMode {
    // Every stream-only render produces one readback. Pacing the whole
    // update/render pipeline at the broadcast cadence avoids running transform
    // propagation and GPU uploads several times for each encoded frame. GPU
    // completion remains asynchronous so CPU update work and GPU rendering can
    // overlap within the frame period.
    let producer_hz = u32::from(frames_per_second.max(1));
    UpdateMode::Reactive {
        wait: Duration::from_secs_f64(1.0 / f64::from(producer_hz)),
        react_to_device_events: false,
        react_to_user_events: false,
        react_to_window_events: false,
    }
}

fn stream_hidden_window_present_mode() -> PresentMode {
    // The primary window has no camera while stream-only capture is active, but
    // Bevy still presents its hidden swapchain. Do not let the player's VSync
    // preference pace the offscreen image that supplies the broadcast.
    PresentMode::AutoNoVsync
}

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
    pub average_mux_write_ms: f64,
    pub maximum_mux_write_ms: f64,
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
    reconnects: u64,
}

#[derive(Resource, Default)]
struct StreamOnlyCaptureState {
    target: Option<Handle<Image>>,
    operator_window: Option<Entity>,
    operator_camera: Option<Entity>,
    operator_root: Option<Entity>,
    previous_camera_targets: HashMap<Entity, RenderTarget>,
    previous_primary_visibility: Option<bool>,
    previous_primary_present_mode: Option<PresentMode>,
    previous_focused_mode: Option<UpdateMode>,
    previous_unfocused_mode: Option<UpdateMode>,
    width: u32,
    height: u32,
}

#[derive(Resource, Clone, Default, ExtractResource)]
struct StreamOnlyCaptureExtract {
    target: Option<Handle<Image>>,
    video_sink: Option<BroadcastVideoSink>,
    active: bool,
    width: u32,
    height: u32,
}

#[derive(Clone)]
struct BroadcastVideoSink {
    video: Arc<Mutex<Option<VideoFrame>>>,
    metrics: Arc<BroadcastMetrics>,
    stop: Arc<AtomicBool>,
    video_consumer_ready: Arc<AtomicBool>,
}

impl BroadcastVideoSink {
    fn send_video(&self, frame: VideoFrame) -> bool {
        publish_latest_video(
            &self.video,
            &self.stop,
            &self.metrics,
            &self.video_consumer_ready,
            frame,
        )
    }
}

struct StreamOnlyCapturedFrame {
    width: u32,
    height: u32,
    captured_at: Instant,
    pixels: Vec<u8>,
    recycle_pool: Arc<Mutex<Vec<Vec<u8>>>>,
}

#[derive(Default)]
struct GpuStreamCaptureCounters {
    in_flight: AtomicUsize,
    queued: AtomicUsize,
    dropped: AtomicU64,
    map_failures: AtomicU64,
    completed: AtomicU64,
    slot_misses: AtomicU64,
    queue_drops: AtomicU64,
    copy_failures: AtomicU64,
    total_map_failures: AtomicU64,
}

#[derive(Resource, Clone, Default)]
struct GpuStreamCaptureShared(Arc<GpuStreamCaptureCounters>);

#[derive(Resource)]
struct StreamOnlyCaptureInbox(Mutex<Receiver<StreamOnlyCapturedFrame>>);

struct GpuStreamCaptureSlot {
    buffer: Buffer,
    busy: Arc<AtomicBool>,
}

#[derive(Resource)]
struct GpuStreamCaptureRing {
    sender: SyncSender<StreamOnlyCapturedFrame>,
    shared: GpuStreamCaptureShared,
    slots: Vec<GpuStreamCaptureSlot>,
    recycled_pixels: Arc<Mutex<Vec<Vec<u8>>>>,
    width: u32,
    height: u32,
    aligned_row_bytes: u32,
    next_slot: usize,
}

impl GpuStreamCaptureRing {
    fn new(sender: SyncSender<StreamOnlyCapturedFrame>, shared: GpuStreamCaptureShared) -> Self {
        Self {
            sender,
            shared,
            slots: Vec::new(),
            recycled_pixels: Arc::new(Mutex::new(Vec::new())),
            width: 0,
            height: 0,
            aligned_row_bytes: 0,
            next_slot: 0,
        }
    }
}

#[derive(Component)]
pub(crate) struct StreamOperatorCamera;

#[derive(Component)]
pub(crate) struct StreamOperatorWindow;

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
            reconnects: 0,
        }
    }
}

#[cfg(feature = "stream-profiling")]
#[derive(Clone, Debug, serde::Serialize)]
pub struct DirectBroadcastProfile {
    pub phase: &'static str,
    pub rtmp_connected: bool,
    pub twitch_live_confirmed: bool,
    pub session_age_seconds: f64,
    pub encoder: Option<String>,
    pub ingest_name: Option<String>,
    pub captured_fps: f64,
    pub encoded_fps: f64,
    pub captured_frames: u64,
    pub encoded_frames: u64,
    pub dropped_frames: u64,
    pub replaced_frames: u64,
    pub skipped_frames: u64,
    pub encoded_audio_frames: u64,
    pub dropped_audio_frames: u64,
    pub audio_queue_depth: u64,
    pub audio_queue_high_water: u64,
    pub capture_samples: u64,
    pub capture_micros: u64,
    pub maximum_capture_micros: u64,
    pub encode_micros: u64,
    pub maximum_encode_micros: u64,
    pub mux_write_samples: u64,
    pub mux_write_micros: u64,
    pub maximum_mux_write_micros: u64,
    pub video_packet_bytes: u64,
    pub keyframe_packets: u64,
    pub keyframe_bytes: u64,
    pub maximum_video_packet_bytes: u64,
    pub maximum_keyframe_bytes: u64,
    pub reconnects: u64,
    pub readbacks_in_flight: usize,
    pub readbacks_queued: usize,
    pub readbacks_completed: u64,
    pub readback_slot_misses: u64,
    pub readback_queue_drops: u64,
    pub readback_copy_failures: u64,
    pub readback_map_failures: u64,
}

#[cfg(feature = "stream-profiling")]
pub(crate) fn profiling_snapshot(world: &World) -> Option<DirectBroadcastProfile> {
    let runtime = world.get_resource::<DirectBroadcastRuntime>()?;
    let metrics = runtime.controller.as_ref().map_or_else(
        BroadcastMetricsSnapshot::default,
        BroadcastController::metrics,
    );
    let capture = world.get_resource::<GpuStreamCaptureShared>();
    let phase = match &runtime.phase {
        DirectBroadcastPhase::Disabled => "disabled",
        DirectBroadcastPhase::WaitingForBroadcasterAuthorization => "authorizing",
        DirectBroadcastPhase::WaitingForGameplay => "waiting_for_gameplay",
        DirectBroadcastPhase::ResolvingIngest => "resolving_ingest",
        DirectBroadcastPhase::Connecting => "connecting",
        DirectBroadcastPhase::VerifyingTwitch => "verifying_twitch",
        DirectBroadcastPhase::Broadcasting => "broadcasting",
        DirectBroadcastPhase::BandwidthTesting => "bandwidth_testing",
        DirectBroadcastPhase::Reconnecting => "reconnecting",
        DirectBroadcastPhase::Stopping => "stopping",
        DirectBroadcastPhase::Stopped => "stopped",
        DirectBroadcastPhase::Error(_) => "error",
    };
    Some(DirectBroadcastProfile {
        phase,
        rtmp_connected: matches!(
            runtime.phase,
            DirectBroadcastPhase::VerifyingTwitch
                | DirectBroadcastPhase::Broadcasting
                | DirectBroadcastPhase::BandwidthTesting
        ),
        twitch_live_confirmed: runtime.phase == DirectBroadcastPhase::Broadcasting,
        session_age_seconds: runtime
            .broadcast_started
            .map_or(0.0, |started| started.elapsed().as_secs_f64()),
        encoder: runtime.encoder.clone(),
        ingest_name: runtime.ingest.clone(),
        captured_fps: runtime.rolling_captured_video_fps,
        encoded_fps: runtime.rolling_encoded_video_fps,
        captured_frames: metrics.captured_video,
        encoded_frames: metrics.encoded_video,
        dropped_frames: metrics.dropped_video,
        replaced_frames: metrics.replaced_video,
        skipped_frames: metrics.skipped_video,
        encoded_audio_frames: metrics.encoded_audio,
        dropped_audio_frames: metrics.dropped_audio,
        audio_queue_depth: metrics.queued_audio,
        audio_queue_high_water: metrics.audio_queue_high_water,
        capture_samples: metrics.capture_samples,
        capture_micros: metrics.capture_micros,
        maximum_capture_micros: metrics.maximum_capture_micros,
        encode_micros: metrics.video_encode_micros,
        maximum_encode_micros: metrics.maximum_video_encode_micros,
        mux_write_samples: metrics.mux_write_samples,
        mux_write_micros: metrics.mux_write_micros,
        maximum_mux_write_micros: metrics.maximum_mux_write_micros,
        video_packet_bytes: metrics.video_packet_bytes,
        keyframe_packets: metrics.keyframe_packets,
        keyframe_bytes: metrics.keyframe_bytes,
        maximum_video_packet_bytes: metrics.maximum_video_packet_bytes,
        maximum_keyframe_bytes: metrics.maximum_keyframe_bytes,
        reconnects: runtime.reconnects,
        readbacks_in_flight: capture
            .map_or(0, |capture| capture.0.in_flight.load(Ordering::Relaxed)),
        readbacks_queued: capture.map_or(0, |capture| capture.0.queued.load(Ordering::Relaxed)),
        readbacks_completed: capture
            .map_or(0, |capture| capture.0.completed.load(Ordering::Relaxed)),
        readback_slot_misses: capture
            .map_or(0, |capture| capture.0.slot_misses.load(Ordering::Relaxed)),
        readback_queue_drops: capture
            .map_or(0, |capture| capture.0.queue_drops.load(Ordering::Relaxed)),
        readback_copy_failures: capture
            .map_or(0, |capture| capture.0.copy_failures.load(Ordering::Relaxed)),
        readback_map_failures: capture.map_or(0, |capture| {
            capture.0.total_map_failures.load(Ordering::Relaxed)
        }),
    })
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
            average_mux_write_ms: average_milliseconds(
                metrics.mux_write_micros,
                metrics.mux_write_samples,
            ),
            maximum_mux_write_ms: micros_to_milliseconds(metrics.maximum_mux_write_micros),
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

include!("direct_broadcast/plugin.rs");
include!("direct_broadcast/capture.rs");
include!("direct_broadcast/authorization.rs");
include!("direct_broadcast/controller.rs");
include!("direct_broadcast/encoder.rs");
include!("direct_broadcast/tests.rs");
