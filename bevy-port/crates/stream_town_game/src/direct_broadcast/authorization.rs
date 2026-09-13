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
        bail!("Twitch did not confirm the channel as live within 60 seconds; RTMP output remains active while public status is unconfirmed")
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

#[derive(Debug)]
struct VideoFrame {
    width: u32,
    height: u32,
    pixel_format: VideoPixelFormat,
    pixels: Vec<u8>,
    recycle_pool: Option<Arc<Mutex<Vec<Vec<u8>>>>>,
}

impl Clone for VideoFrame {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            pixel_format: self.pixel_format,
            pixels: self.pixels.clone(),
            recycle_pool: None,
        }
    }
}

impl Drop for VideoFrame {
    fn drop(&mut self) {
        if let Some(pool) = self.recycle_pool.take() {
            recycle_stream_capture_pixels(pool, std::mem::take(&mut self.pixels));
        }
    }
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
    mux_write_samples: AtomicU64,
    mux_write_micros: AtomicU64,
    maximum_mux_write_micros: AtomicU64,
    video_packet_bytes: AtomicU64,
    keyframe_packets: AtomicU64,
    keyframe_bytes: AtomicU64,
    maximum_video_packet_bytes: AtomicU64,
    maximum_keyframe_bytes: AtomicU64,
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
    mux_write_samples: u64,
    mux_write_micros: u64,
    maximum_mux_write_micros: u64,
    video_packet_bytes: u64,
    keyframe_packets: u64,
    keyframe_bytes: u64,
    maximum_video_packet_bytes: u64,
    maximum_keyframe_bytes: u64,
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
            mux_write_samples: self.mux_write_samples.load(Ordering::Relaxed),
            mux_write_micros: self.mux_write_micros.load(Ordering::Relaxed),
            maximum_mux_write_micros: self.maximum_mux_write_micros.load(Ordering::Relaxed),
            video_packet_bytes: self.video_packet_bytes.load(Ordering::Relaxed),
            keyframe_packets: self.keyframe_packets.load(Ordering::Relaxed),
            keyframe_bytes: self.keyframe_bytes.load(Ordering::Relaxed),
            maximum_video_packet_bytes: self.maximum_video_packet_bytes.load(Ordering::Relaxed),
            maximum_keyframe_bytes: self.maximum_keyframe_bytes.load(Ordering::Relaxed),
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

    fn observe_mux_write_latency(&self, duration: Duration) {
        let micros = duration_as_micros(duration);
        self.mux_write_samples.fetch_add(1, Ordering::Relaxed);
        self.mux_write_micros.fetch_add(micros, Ordering::Relaxed);
        self.maximum_mux_write_micros
            .fetch_max(micros, Ordering::Relaxed);
    }

    fn observe_video_packet(&self, bytes: usize, keyframe: bool) {
        let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
        self.video_packet_bytes.fetch_add(bytes, Ordering::Relaxed);
        self.maximum_video_packet_bytes
            .fetch_max(bytes, Ordering::Relaxed);
        if keyframe {
            self.keyframe_packets.fetch_add(1, Ordering::Relaxed);
            self.keyframe_bytes.fetch_add(bytes, Ordering::Relaxed);
            self.maximum_keyframe_bytes
                .fetch_max(bytes, Ordering::Relaxed);
        }
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
                recycle_pool: None,
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
