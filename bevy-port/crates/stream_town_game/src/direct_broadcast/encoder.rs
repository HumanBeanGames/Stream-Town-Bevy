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
        encoder.finish(metrics)?;
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
            encoder.finish(metrics)?;
            return Ok(SessionEnd::Stopped);
        }
        if graceful_stop.load(Ordering::Acquire) && graceful_deadline.is_none() {
            latest_video = take_latest_video(video_mailbox).or(latest_video);
            if latest_video.is_none() {
                encoder.finish(metrics)?;
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
            let published_packets = encoder.encode_video(video, tick.pts, metrics)?;
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
            // The live FLV trailer itself is not useful to Twitch, but FFmpeg's
            // FIFO muxer uses this call to stop and join its writer thread.
            encoder.finish(metrics)?;
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
                    encoder.encode_audio(audio, metrics)?;
                    metrics.encoded_audio.fetch_add(1, Ordering::Relaxed);
                }
            }
            Ok(AudioInput::Stop) => {
                if graceful_deadline.is_none() {
                    encoder.finish(metrics)?;
                    return Ok(SessionEnd::Stopped);
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                encoder.finish(metrics)?;
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

fn broadcast_output_options(url: &str) -> (&'static str, Dictionary<'static>) {
    let mut options = Dictionary::new();
    if url.starts_with("rtmp://") || url.starts_with("rtmps://") {
        // FFmpeg's FIFO pseudo-muxer owns the potentially blocking RTMP writes
        // on a separate bounded thread. The queue absorbs short ingest stalls;
        // once full it discards stale packets instead of backpressuring capture,
        // audio, and the hardware encoder. Recovery waits for a keyframe below,
        // so viewers resume on a decodable boundary after a longer outage.
        options.set("fifo_format", "flv");
        options.set("queue_size", BROADCAST_FIFO_QUEUE_PACKETS);
        options.set("drop_pkts_on_overflow", "1");
        options.set("attempt_recovery", "1");
        options.set("recover_any_error", "1");
        options.set("max_recovery_attempts", "3");
        options.set("recovery_wait_time", "1");
        options.set("recovery_wait_streamtime", "0");
        options.set("restart_with_keyframe", "1");
        options.set(
            "format_opts",
            &format!(
                "rw_timeout={BROADCAST_IO_TIMEOUT_MICROSECONDS}:rtmp_live=live:tcp_keepalive=1:flvflags=no_duration_filesize"
            ),
        );
        ("fifo", options)
    } else {
        options.set("rw_timeout", BROADCAST_IO_TIMEOUT_MICROSECONDS);
        ("flv", options)
    }
}

impl BroadcastEncoder {
    fn open(
        target: &BroadcastTarget,
        config: &BroadcastConfig,
    ) -> Result<(Self, VideoEncoderSelection)> {
        let (muxer, output_options) = broadcast_output_options(&target.url);
        let live_network_output = muxer == "fifo";
        let output = if live_network_output {
            stream_town_ffmpeg_bridge::allocate_fifo_output(&target.url)?
        } else {
            format::output_as_with(&target.url, muxer, output_options.clone())
                .context("could not open the local FLV broadcast target")?
        };
        let mut output = BroadcastOutput::new(output);
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
        if live_network_output {
            let rejected_option = {
                let unused = output
                    .write_header_with(output_options)
                    .context("Twitch rejected the buffered FLV stream header")?;
                unused.iter().next().map(|(option, _)| option.to_owned())
            };
            output.mark_header_written();
            if let Some(option) = rejected_option {
                bail!("linked FFmpeg FIFO muxer rejected broadcast option '{option}'");
            }
        } else {
            output
                .write_header()
                .context("could not write the local FLV stream header")?;
            output.mark_header_written();
        }
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
                source_video_frames: Vec::new(),
                source_video_frame_cursor: 0,
                converted_video_frames: Vec::new(),
                converted_video_frame_cursor: 0,
                audio_pts_base: None,
            },
            encoder_selection,
        ))
    }

    fn encode_video(
        &mut self,
        video: &VideoFrame,
        pts: i64,
        metrics: &BroadcastMetrics,
    ) -> Result<u64> {
        let source_format = video.pixel_format.ffmpeg();
        let source = reusable_video_frame(
            &mut self.source_video_frames,
            &mut self.source_video_frame_cursor,
            source_format,
            video.width,
            video.height,
        );
        copy_packed_video_frame(video, source)?;
        if source_format == self.video_input_format
            && video.width == self.width
            && video.height == self.height
        {
            source.set_pts(Some(pts));
            self.video
                .send_frame(source)
                .context("H.264 encoder rejected a packed frame")?;
            return self.drain_video(metrics);
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
        let converted = reusable_video_frame(
            &mut self.converted_video_frames,
            &mut self.converted_video_frame_cursor,
            self.video_input_format,
            self.width,
            self.height,
        );
        self.scaler
            .as_mut()
            .context("broadcast scaler was not initialized")?
            .3
            .run(source, converted)
            .context("could not scale a broadcast frame")?;
        converted.set_pts(Some(pts));
        self.video
            .send_frame(converted)
            .context("H.264 encoder rejected a frame")?;
        self.drain_video(metrics)
    }

    fn encode_audio(&mut self, audio: AudioFrame, metrics: &BroadcastMetrics) -> Result<()> {
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
        self.drain_audio(metrics)
    }

    fn drain_video(&mut self, metrics: &BroadcastMetrics) -> Result<u64> {
        let mut packet = Packet::empty();
        let mut published = 0_u64;
        while self.video.receive_packet(&mut packet).is_ok() {
            metrics.observe_video_packet(packet.size(), packet.is_key());
            packet.set_stream(self.video_stream);
            packet.rescale_ts(self.video.time_base(), self.video_time_base);
            let pts = packet.pts();
            let dts = packet.dts();
            let duration = packet.duration();
            let publish_started = Instant::now();
            let publish_result = packet.write_interleaved(&mut self.output);
            metrics.observe_mux_write_latency(publish_started.elapsed());
            publish_result.with_context(|| {
                    format!(
                        "could not publish an H.264 packet to Twitch (pts={pts:?}, dts={dts:?}, duration={duration})"
                    )
                })?;
            published = published.saturating_add(1);
        }
        Ok(published)
    }

    fn drain_audio(&mut self, metrics: &BroadcastMetrics) -> Result<()> {
        let mut packet = Packet::empty();
        while self.audio.receive_packet(&mut packet).is_ok() {
            packet.set_stream(self.audio_stream);
            packet.rescale_ts(self.audio.time_base(), self.audio_time_base);
            let publish_started = Instant::now();
            let publish_result = packet.write_interleaved(&mut self.output);
            metrics.observe_mux_write_latency(publish_started.elapsed());
            publish_result.context("could not publish an AAC packet to Twitch")?;
        }
        Ok(())
    }

    fn finish(&mut self, metrics: &BroadcastMetrics) -> Result<()> {
        self.video.send_eof().ok();
        self.drain_video(metrics)?;
        self.audio.send_eof().ok();
        self.drain_audio(metrics)?;
        let publish_started = Instant::now();
        let finished = self.output.finish();
        metrics.observe_mux_write_latency(publish_started.elapsed());
        finished.context("could not finish the Twitch FLV stream")
    }
}

fn reusable_video_frame<'a>(
    frames: &'a mut Vec<frame::Video>,
    cursor: &mut usize,
    format: ffmpeg::format::Pixel,
    width: u32,
    height: u32,
) -> &'a mut frame::Video {
    let layout_changed = frames.first().is_some_and(|frame| {
        frame.format() != format || frame.width() != width || frame.height() != height
    });
    if layout_changed || frames.len() != ENCODER_VIDEO_FRAME_BUFFERS {
        frames.clear();
        frames.extend(
            (0..ENCODER_VIDEO_FRAME_BUFFERS).map(|_| frame::Video::new(format, width, height)),
        );
        *cursor = 0;
    }
    let index = *cursor % frames.len();
    *cursor = (index + 1) % frames.len();
    // Four slots exceed the zero-B-frame encoder's retained input depth. Each
    // slot is therefore released before the cadence worker cycles back to it,
    // without allocating another full-resolution AVFrame every tick.
    &mut frames[index]
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
    // The linked FFmpeg 8 / AMF combination accepts the quality preset on the
    // RX 7800 XT. Its newer high_quality enum is exposed only by later FFmpeg
    // builds and makes this shipped runtime reject AMF at encoder open.
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
    // Camera pans invalidate much of the previous frame at once. Ask AMF to
    // preserve those moving details instead of spending nearly all of the GOP
    // quality budget on its two-second IDR frames.
    options.set("high_motion_quality_boost_enable", "1");
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

fn configure_x264_quality(
    options: &mut Dictionary<'_>,
    frames_per_second: u8,
    video_bitrate_kbps: u32,
) {
    let keyframe_interval = u32::from(frames_per_second) * 2;
    let vbv_rate = format!("{video_bitrate_kbps}k");
    options.set("profile", "high");
    // `faster` leaves CPU headroom for the simulation while x264's lookahead,
    // adaptive B-frames, and AQ preserve moving terrain detail much better
    // than the affected AMF path. Bound x264's worker fan-out as well: its
    // automatic thread count can occupy every logical processor during encode
    // bursts, starving Bevy's render preparation even though encode latency is
    // comfortably below the frame budget. Three frame threads retain the same
    // quality tools while reserving half of the deployment host's six physical
    // cores for Bevy, the GPU driver, audio, and RTMP work.
    options.set("preset", "faster");
    options.set("threads", "3");
    options.set("maxrate", &vbv_rate);
    options.set("bufsize", &vbv_rate);
    options.set(
        "x264-params",
        &format!(
            "nal-hrd=cbr:force-cfr=1:keyint={keyframe_interval}:min-keyint={keyframe_interval}:scenecut=0:bframes=2:b-adapt=1:ref=3:rc-lookahead=20:lookahead-threads=1:aq-mode=3:aq-strength=1.0"
        ),
    );
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
            // Hardware paths retain decode-order-only timestamps because AMD's
            // reordered B-frame path caused intermittent FLV publish failures.
            // libx264's mature reorder path safely uses two adaptive B-frames.
            video.set_max_b_frames(if name == "libx264" { 2 } else { 0 });
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
                "libx264" => {
                    configure_x264_quality(
                        &mut options,
                        config.frames_per_second,
                        config.video_bitrate_kbps,
                    );
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
            "libx264",
            "libopenh264",
        ],
        BroadcastEncoderPreference::Nvidia => &["h264_nvenc"],
        BroadcastEncoderPreference::Intel => &["h264_qsv"],
        BroadcastEncoderPreference::Amd => &["h264_amf"],
        BroadcastEncoderPreference::MediaFoundation => &["h264_mf"],
        BroadcastEncoderPreference::X264 => &["libx264"],
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
        "libx264",
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
    let metrics = BroadcastMetrics::default();
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
                recycle_pool: None,
            },
            pts,
            &metrics,
        )?;
        encoder.encode_audio(
            AudioFrame {
                pts: pts * i64::try_from(AUDIO_FRAME_SAMPLES).unwrap_or(i64::MAX),
                samples: vec![0.0; AUDIO_FRAME_SAMPLES * AUDIO_CHANNELS],
            },
            &metrics,
        )?;
    }
    encoder
        .finish(&metrics)
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
