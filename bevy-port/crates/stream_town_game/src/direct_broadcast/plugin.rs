pub struct DirectTwitchBroadcastPlugin;

impl Plugin for DirectTwitchBroadcastPlugin {
    fn build(&self, app: &mut App) {
        let (capture_sender, capture_receiver) =
            mpsc::sync_channel(STREAM_CAPTURE_CPU_QUEUE_CAPACITY);
        let capture_shared = GpuStreamCaptureShared::default();
        app.add_message::<AppExit>()
            .add_message::<WindowCloseRequested>()
            .init_resource::<DirectBroadcastRuntime>()
            .init_resource::<DirectBroadcastControl>()
            .init_resource::<AutomaticBroadcastStart>()
            .init_resource::<NativeGameAudioRouting>()
            .init_resource::<StreamOnlyCaptureState>()
            .init_resource::<StreamOnlyCaptureExtract>()
            .insert_resource(capture_shared.clone())
            .insert_resource(StreamOnlyCaptureInbox(Mutex::new(capture_receiver)))
            .init_resource::<OperatorChatRuntime>()
            .add_plugins(ExtractResourcePlugin::<StreamOnlyCaptureExtract>::default())
            .add_systems(
                Update,
                (
                    (
                        start_local_broadcast_diagnostic,
                        operator_window_close_requests_exit,
                        request_automatic_broadcast_start,
                        stream_operator_live_button,
                        stream_operator_restart_button,
                        apply_direct_broadcast_control,
                        poll_direct_broadcast_authorization,
                        start_prepared_broadcast_when_gameplay_ready,
                        poll_direct_broadcast_worker,
                        poll_twitch_live_verification,
                        exit_after_broadcast_stops,
                        return_to_main_menu_after_broadcast_stops,
                    )
                        .chain(),
                    (
                        sync_stream_only_capture,
                        sync_stream_only_capture_extract,
                        receive_stream_only_captured_frames,
                        update_stream_operator_info,
                        stream_operator_chat_controls,
                        update_stream_operator_chat,
                        stream_operator_settings_controls,
                        update_stream_operator_settings,
                        capture_direct_broadcast_frame.after(SensitiveScreenUpdateSet),
                    )
                        .chain(),
                )
                    .chain(),
            );
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .insert_resource(GpuStreamCaptureRing::new(capture_sender, capture_shared))
                .add_systems(
                    Render,
                    capture_stream_only_target.in_set(RenderSystems::Cleanup),
                );
        }
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
    capture: Res<GpuStreamCaptureShared>,
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
                runtime.reconnects = runtime.reconnects.saturating_add(1);
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
                        "event=session_reconnecting cause={error:?} captured_video={} encoded_video={} video_drops={} capture_replacements={} cadence_skips={} encoded_audio={} audio_drops={} audio_queue={} maximum_capture_ms={:.2} maximum_encode_ms={:.2} maximum_mux_write_ms={:.2}",
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
                        micros_to_milliseconds(metrics.maximum_mux_write_micros),
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
        capture.0.in_flight.load(Ordering::Relaxed),
        capture.0.queued.load(Ordering::Relaxed),
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
            warn!(%error, "could not start Twitch live verification; keeping RTMP output active");
            runtime.verification_status = Some(format!(
                "Public status unconfirmed; RTMP remains active: {error:#}"
            ));
            runtime.phase = DirectBroadcastPhase::VerifyingTwitch;
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
                warn!(%error, "Twitch public status remained unconfirmed; keeping RTMP output active");
                runtime.live_verification = None;
                runtime.verification_status = Some(format!(
                    "Public status unconfirmed; RTMP remains active: {error}"
                ));
                runtime.phase = DirectBroadcastPhase::VerifyingTwitch;
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
    let average_mux_write_ms = average_milliseconds(
        metrics
            .mux_write_micros
            .saturating_sub(runtime.health_reported_metrics.mux_write_micros),
        metrics
            .mux_write_samples
            .saturating_sub(runtime.health_reported_metrics.mux_write_samples),
    );
    let video_bytes = metrics
        .video_packet_bytes
        .saturating_sub(runtime.health_reported_metrics.video_packet_bytes);
    let video_kbps = rate_per_second(video_bytes.saturating_mul(8), elapsed) / 1_000.0;
    let keyframe_packets = metrics
        .keyframe_packets
        .saturating_sub(runtime.health_reported_metrics.keyframe_packets);
    let keyframe_bytes = metrics
        .keyframe_bytes
        .saturating_sub(runtime.health_reported_metrics.keyframe_bytes);
    let average_keyframe_kib = if keyframe_packets == 0 {
        0
    } else {
        keyframe_bytes
            .saturating_div(keyframe_packets)
            .saturating_add(512)
            / 1_024
    };
    let maximum_keyframe_kib = metrics.maximum_keyframe_bytes.saturating_add(512) / 1_024;
    let maximum_video_packet_kib = metrics.maximum_video_packet_bytes.saturating_add(512) / 1_024;
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
            "event=health target_fps={target_fps} captured_fps={captured_fps:.2} encoded_fps={encoded_fps:.2} audio_fps={audio_fps:.2} video_kbps={video_kbps:.0} keyframes={keyframe_packets} average_keyframe_kib={average_keyframe_kib} maximum_keyframe_kib={maximum_keyframe_kib} maximum_video_packet_kib={maximum_video_packet_kib} video_drops={new_video_drops} capture_replacements={new_video_replacements} cadence_skips={new_video_skips} audio_drops={new_audio_drops} audio_queue={} readbacks_in_flight={readbacks_in_flight} readbacks_completed={readbacks_completed} average_encode_ms={average_encode_ms:.2} maximum_encode_ms={:.2} average_mux_write_ms={average_mux_write_ms:.2} maximum_mux_write_ms={:.2}",
            metrics.queued_audio,
            micros_to_milliseconds(metrics.maximum_video_encode_micros),
            micros_to_milliseconds(metrics.maximum_mux_write_micros),
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
            video_kbps,
            keyframe_packets,
            average_keyframe_kib,
            maximum_keyframe_kib,
            maximum_video_packet_kib,
            maximum_encode_ms = micros_to_milliseconds(metrics.maximum_video_encode_micros),
            average_mux_write_ms,
            maximum_mux_write_ms = micros_to_milliseconds(metrics.maximum_mux_write_micros),
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
            video_kbps,
            keyframe_packets,
            average_keyframe_kib,
            average_mux_write_ms,
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
                stream_operator_window(),
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
        if let Ok(mut window) = primary_window.single_mut() {
            state
                .previous_primary_visibility
                .get_or_insert(window.visible);
            state
                .previous_primary_present_mode
                .get_or_insert(window.present_mode);
            if window.visible {
                window.visible = false;
            }
            let present_mode = stream_hidden_window_present_mode();
            if window.present_mode != present_mode {
                window.present_mode = present_mode;
            }
        }
        if let Some(winit) = winit.as_deref_mut() {
            state
                .previous_focused_mode
                .get_or_insert(winit.focused_mode);
            state
                .previous_unfocused_mode
                .get_or_insert(winit.unfocused_mode);
            let update_mode =
                stream_render_update_mode(config.0.twitch.broadcast.frames_per_second);
            if winit.focused_mode != update_mode {
                winit.focused_mode = update_mode;
            }
            if winit.unfocused_mode != update_mode {
                winit.unfocused_mode = update_mode;
            }
        }
        return;
    }

    for (entity, previous_target) in state.previous_camera_targets.drain() {
        if let Ok((_, mut camera_target)) = camera_targets.get_mut(entity) {
            *camera_target = previous_target;
        }
    }
    if let Ok(mut window) = primary_window.single_mut() {
        if let Some(visible) = state.previous_primary_visibility.take() {
            window.visible = visible;
        }
        if let Some(present_mode) = state.previous_primary_present_mode.take() {
            window.present_mode = present_mode;
        }
    }
    if let Some(winit) = winit.as_deref_mut() {
        if let Some(previous) = state.previous_focused_mode.take() {
            winit.focused_mode = previous;
        }
        if let Some(previous) = state.previous_unfocused_mode.take() {
            winit.unfocused_mode = previous;
        }
    }
    if !target_required && state.target.is_some() {
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
        commands.entity(entity).try_despawn();
    }
    if operator_was_open {
        info!("local stream operator panel closed");
    }
}
