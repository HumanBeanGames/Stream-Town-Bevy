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

    fn video_sink(&self) -> BroadcastVideoSink {
        BroadcastVideoSink {
            video: Arc::clone(&self.video),
            metrics: Arc::clone(&self.metrics),
            stop: Arc::clone(&self.capture_stop),
            video_consumer_ready: Arc::clone(&self.video_consumer_ready),
        }
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
                recycle_pool: None,
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
    // Process-loopback activation can fail with RPC_E_CHANGED_MODE from an MTA
    // after the GUI/WinRT runtime is active. Keep the capture client and all of
    // its use on this dedicated STA thread for the lifetime of the session.
    initialize_sta()
        .ok()
        .map_err(|error| anyhow!("could not initialize Windows audio COM: {error}"))?;
    let mut client = AudioClient::new_application_loopback_client(std::process::id(), true)
        .map_err(|error| anyhow!("could not create process-scoped WASAPI loopback: {error}"))?;
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

struct BroadcastOutput {
    output: format::context::Output,
    trailer_pending: bool,
}

impl BroadcastOutput {
    fn new(output: format::context::Output) -> Self {
        Self {
            output,
            trailer_pending: false,
        }
    }

    fn mark_header_written(&mut self) {
        self.trailer_pending = true;
    }

    fn finish(&mut self) -> std::result::Result<(), ffmpeg::Error> {
        if !self.trailer_pending {
            return Ok(());
        }
        // `fifo_write_trailer` signals EOF and joins FFmpeg's private writer
        // thread. Clear the flag first because even an error return has already
        // consumed this output and must never cause a second trailer attempt.
        self.trailer_pending = false;
        self.output.write_trailer()
    }
}

impl std::ops::Deref for BroadcastOutput {
    type Target = format::context::Output;

    fn deref(&self) -> &Self::Target {
        &self.output
    }
}

impl std::ops::DerefMut for BroadcastOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.output
    }
}

impl Drop for BroadcastOutput {
    fn drop(&mut self) {
        // Every successful FIFO header starts a native writer thread. Any Rust
        // error path that bypasses BroadcastEncoder::finish must still join it
        // before ffmpeg-next frees the AVFormatContext, queue, and mutex.
        let _ = self.finish();
    }
}

struct BroadcastEncoder {
    output: BroadcastOutput,
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
    source_video_frames: Vec<frame::Video>,
    source_video_frame_cursor: usize,
    converted_video_frames: Vec<frame::Video>,
    converted_video_frame_cursor: usize,
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
