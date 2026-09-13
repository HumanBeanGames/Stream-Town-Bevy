#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_info_refresh_is_bounded_to_once_per_second() {
        let started_at = Instant::now();
        let mut next_refresh = None;

        assert!(operator_info_refresh_due(&mut next_refresh, started_at));
        assert!(!operator_info_refresh_due(
            &mut next_refresh,
            started_at + Duration::from_millis(999)
        ));
        assert!(operator_info_refresh_due(
            &mut next_refresh,
            started_at + STREAM_OPERATOR_INFO_REFRESH_INTERVAL
        ));
    }

    #[test]
    fn live_broadcast_uses_a_bounded_recovering_fifo_muxer() {
        let (muxer, options) =
            broadcast_output_options("rtmp://example.invalid/app/live_secret_value");

        assert_eq!(muxer, "fifo");
        assert_eq!(options.get("fifo_format"), Some("flv"));
        assert_eq!(
            options.get("queue_size"),
            Some(BROADCAST_FIFO_QUEUE_PACKETS)
        );
        assert_eq!(options.get("drop_pkts_on_overflow"), Some("1"));
        assert_eq!(options.get("attempt_recovery"), Some("1"));
        assert_eq!(options.get("restart_with_keyframe"), Some("1"));
        let format_options = options.get("format_opts").unwrap_or_default();
        assert!(format_options.contains("rtmp_live=live"));
        assert!(format_options.contains("tcp_keepalive=1"));
        assert!(!format_options.contains("tcp_nodelay=1"));
        assert!(!format_options.contains("flush_packets=1"));
    }

    #[test]
    fn linked_ffmpeg_runtime_contains_the_fifo_muxer() {
        ffmpeg::init().unwrap();
        let output =
            stream_town_ffmpeg_bridge::allocate_fifo_output("rtmp://example.invalid/app/test")
                .unwrap();

        assert_eq!(output.format().name(), "fifo");
    }

    #[test]
    fn local_diagnostics_keep_the_direct_flv_muxer() {
        let (muxer, options) = broadcast_output_options("diagnostic.flv");

        assert_eq!(muxer, "flv");
        assert_eq!(
            options.get("rw_timeout"),
            Some(BROADCAST_IO_TIMEOUT_MICROSECONDS)
        );
        assert_eq!(options.get("fifo_format"), None);
    }

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

        let mut pixels = Vec::new();
        assert!(copy_gpu_rows_into(&padded, width, height, &mut pixels));

        assert_eq!(pixels.len(), row_bytes * usize::try_from(height).unwrap());
        assert!(pixels[..row_bytes].iter().all(|byte| *byte == 0x11));
        assert!(pixels[row_bytes..].iter().all(|byte| *byte == 0x22));
    }

    #[test]
    fn streamed_capture_pixels_return_to_the_bounded_reuse_pool() {
        let pool = Arc::new(Mutex::new(Vec::new()));
        let pixels = vec![0x44; 64];
        let allocation_capacity = pixels.capacity();
        drop(VideoFrame {
            width: 4,
            height: 4,
            pixel_format: VideoPixelFormat::Bgra,
            pixels,
            recycle_pool: Some(Arc::clone(&pool)),
        });

        let recycled = take_stream_capture_pixels(&pool, 64);
        assert_eq!(recycled.len(), 64);
        assert!(recycled.capacity() >= allocation_capacity);
        assert!(pool.lock().unwrap().is_empty());
    }

    #[test]
    fn operator_dashboard_does_not_vsync_the_stream_render_loop() {
        assert_eq!(
            stream_operator_window().present_mode,
            PresentMode::AutoNoVsync
        );
        assert_eq!(
            stream_hidden_window_present_mode(),
            PresentMode::AutoNoVsync
        );
    }

    #[test]
    fn stream_render_mode_bounds_the_producer_without_event_bypass() {
        assert_eq!(
            stream_render_update_mode(30),
            UpdateMode::Reactive {
                wait: Duration::from_secs_f64(1.0 / 30.0),
                react_to_device_events: false,
                react_to_user_events: false,
                react_to_window_events: false,
            }
        );
    }

    #[test]
    fn stream_render_mode_clamps_an_invalid_zero_fps_to_one() {
        assert_eq!(
            stream_render_update_mode(0),
            UpdateMode::Reactive {
                wait: Duration::from_secs(1),
                react_to_device_events: false,
                react_to_user_events: false,
                react_to_window_events: false,
            }
        );
    }

    #[test]
    fn public_status_timeout_does_not_stop_a_healthy_rtmp_session() {
        let (sender, receiver) = mpsc::channel();
        sender
            .send(LiveVerificationEvent::Error(
                "Twitch status check timed out".to_owned(),
            ))
            .unwrap();
        let mut app = App::new();
        app.insert_resource(DirectBroadcastRuntime {
            phase: DirectBroadcastPhase::VerifyingTwitch,
            live_verification: Some(LiveVerification {
                events: Arc::new(Mutex::new(receiver)),
                cancel: Arc::new(AtomicBool::new(false)),
            }),
            ..default()
        })
        .add_systems(Update, poll_twitch_live_verification);

        app.update();

        let runtime = app.world().resource::<DirectBroadcastRuntime>();
        assert_eq!(runtime.phase, DirectBroadcastPhase::VerifyingTwitch);
        assert!(runtime.controller.is_none());
        assert!(runtime.live_verification.is_none());
        assert!(
            runtime
                .verification_status
                .as_deref()
                .is_some_and(|status| status.contains("RTMP remains active"))
        );
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
    fn operator_live_button_applies_stop_in_the_pressed_frame() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(RuntimeConfig(stream_town_domain::GameConfig::default()))
            .init_resource::<SensitiveScreenActive>()
            .add_plugins(DirectTwitchBroadcastPlugin);
        app.world_mut()
            .resource_mut::<DirectBroadcastRuntime>()
            .phase = DirectBroadcastPhase::Connecting;
        app.world_mut().spawn((
            StreamOperatorLiveButton,
            Interaction::Pressed,
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
        ));

        app.update();

        assert_eq!(
            app.world().resource::<DirectBroadcastRuntime>().phase,
            DirectBroadcastPhase::Stopped
        );
        assert!(
            !app.world()
                .resource::<DirectBroadcastControl>()
                .stop_requested
        );
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
    fn auto_encoder_order_prefers_hardware_and_has_software_fallbacks() {
        assert_eq!(
            encoder_candidates(BroadcastEncoderPreference::Auto),
            [
                "h264_nvenc",
                "h264_qsv",
                "h264_amf",
                "h264_mf",
                "libx264",
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
            encoder_input_format("libx264"),
            ffmpeg::format::Pixel::YUV420P
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
            recycle_pool: None,
        };
        let second = VideoFrame {
            width: first.width,
            height: first.height,
            pixel_format: first.pixel_format,
            pixels: vec![5, 6, 7, 8],
            recycle_pool: None,
        };
        mailbox.lock().unwrap().replace(first);
        mailbox.lock().unwrap().replace(second);
        assert_eq!(
            take_latest_video(&mailbox).unwrap().pixels.as_slice(),
            [5, 6, 7, 8]
        );
        assert!(take_latest_video(&mailbox).is_none());
    }

    #[test]
    fn video_packet_metrics_separate_keyframe_pressure_from_mux_stalls() {
        let metrics = BroadcastMetrics::default();
        metrics.observe_video_packet(1_024, false);
        metrics.observe_video_packet(8_192, true);
        metrics.observe_video_packet(4_096, true);

        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.video_packet_bytes, 13_312);
        assert_eq!(snapshot.keyframe_packets, 2);
        assert_eq!(snapshot.keyframe_bytes, 12_288);
        assert_eq!(snapshot.maximum_video_packet_bytes, 8_192);
        assert_eq!(snapshot.maximum_keyframe_bytes, 8_192);
    }

    #[test]
    fn stopped_capture_cannot_replace_the_terminal_mailbox_frame() {
        let terminal = VideoFrame {
            width: 1,
            height: 1,
            pixel_format: VideoPixelFormat::Rgba,
            pixels: vec![0, 0, 0, 255],
            recycle_pool: None,
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
                width: terminal.width,
                height: terminal.height,
                pixel_format: terminal.pixel_format,
                pixels: vec![255, 0, 0, 255],
                recycle_pool: None,
            }
        ));
        assert_eq!(
            take_latest_video(&mailbox).unwrap().pixels.as_slice(),
            terminal.pixels.as_slice()
        );
    }

    #[test]
    fn render_video_sink_counts_replaced_video_without_rejecting_the_newest_frame() {
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
            recycle_pool: None,
        };
        let second = VideoFrame {
            width: first.width,
            height: first.height,
            pixel_format: first.pixel_format,
            pixels: vec![5, 6, 7, 8],
            recycle_pool: None,
        };
        let sink = controller.video_sink();
        assert!(sink.send_video(first));
        assert!(sink.send_video(second));
        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.replaced_video, 1);
        assert_eq!(snapshot.dropped_video, 0);
        assert_eq!(
            take_latest_video(&controller.video)
                .unwrap()
                .pixels
                .as_slice(),
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
            recycle_pool: None,
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
        assert_eq!(options.get("high_motion_quality_boost_enable"), Some("1"));
        assert_eq!(options.get("me_half_pel"), Some("1"));
        assert_eq!(options.get("me_quarter_pel"), Some("1"));
        assert_eq!(options.get("latency"), Some("1"));
        assert_eq!(options.get("async_depth"), Some("2"));
    }

    #[test]
    fn x264_quality_profile_uses_cbr_adaptive_b_frames_and_a_two_second_gop() {
        let mut options = Dictionary::new();
        configure_x264_quality(&mut options, 30, 6_000);

        assert_eq!(options.get("profile"), Some("high"));
        assert_eq!(options.get("preset"), Some("faster"));
        assert_eq!(options.get("threads"), Some("3"));
        assert_eq!(options.get("maxrate"), Some("6000k"));
        assert_eq!(options.get("bufsize"), Some("6000k"));
        let params = options.get("x264-params").unwrap();
        assert!(params.contains("nal-hrd=cbr"));
        assert!(params.contains("keyint=60:min-keyint=60"));
        assert!(params.contains("bframes=2:b-adapt=1"));
        assert!(params.contains("rc-lookahead=20"));
        assert!(params.contains("lookahead-threads=1"));
        assert!(params.contains("aq-mode=3"));
    }

    #[test]
    #[ignore = "local 1080p30 x264 throughput diagnostic"]
    fn configured_1080p30_x264_encoder_sustains_realtime_output() {
        let directory = tempfile::tempdir().unwrap();
        let output = std::env::var_os("STREAM_TOWN_BROADCAST_DIAGNOSTIC_OUTPUT").map_or_else(
            || directory.path().join("direct-broadcast-1080p30-x264.flv"),
            std::path::PathBuf::from,
        );
        let target = BroadcastTarget {
            ingest_name: "local-x264-performance-diagnostic".to_owned(),
            url: output.to_string_lossy().into_owned(),
        };
        let config = BroadcastConfig {
            enabled: true,
            width: 1_920,
            height: 1_080,
            frames_per_second: 30,
            video_bitrate_kbps: 6_000,
            encoder: BroadcastEncoderPreference::X264,
            ..BroadcastConfig::default()
        };
        ffmpeg::init().unwrap();
        ffmpeg::log::set_level(ffmpeg::log::Level::Quiet);
        let (mut encoder, selected) = BroadcastEncoder::open(&target, &config).unwrap();
        assert_eq!(selected.name, "libx264");
        let width = usize::from(config.width);
        let height = usize::from(config.height);
        let frames = (0..16_usize)
            .map(|shift| {
                let mut pixels = vec![0_u8; width * height * 4];
                for y in 0..height {
                    for x in 0..width {
                        let offset = (y * width + x) * 4;
                        let checker = (((x + shift * 7) / 8) ^ ((y + shift * 3) / 8)) & 1;
                        let value = if checker == 0 { 36_u8 } else { 212_u8 };
                        pixels[offset] = value;
                        pixels[offset + 1] = value.saturating_add(
                            u8::try_from((x + shift) & 31).expect("masked sample fits u8"),
                        );
                        pixels[offset + 2] = value.saturating_sub(
                            u8::try_from((y + shift) & 31).expect("masked sample fits u8"),
                        );
                        pixels[offset + 3] = 255;
                    }
                }
                VideoFrame {
                    width: u32::from(config.width),
                    height: u32::from(config.height),
                    pixel_format: VideoPixelFormat::Bgra,
                    pixels,
                    recycle_pool: None,
                }
            })
            .collect::<Vec<_>>();
        let frame_count = 120_u32;
        let metrics = BroadcastMetrics::default();
        let started = Instant::now();
        for pts in 0..frame_count {
            let frame = &frames[usize::try_from(pts).unwrap_or_default() % frames.len()];
            encoder
                .encode_video(frame, i64::from(pts), &metrics)
                .unwrap();
        }
        encoder.finish(&metrics).unwrap();
        let elapsed = started.elapsed().as_secs_f64();
        let frames_per_second = f64::from(frame_count) / elapsed;
        eprintln!(
            "1080p30 x264 diagnostic: {frames_per_second:.1} FPS, {:.2} ms/frame",
            elapsed * 1_000.0 / f64::from(frame_count)
        );
        assert!(
            frames_per_second >= 30.0,
            "x264 only sustained {frames_per_second:.1} FPS"
        );
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
            recycle_pool: None,
        };
        let frame_count = 120_u32;
        let metrics = BroadcastMetrics::default();
        let started = Instant::now();
        for pts in 0..frame_count {
            encoder
                .encode_video(&frame, i64::from(pts), &metrics)
                .unwrap();
        }
        encoder.finish(&metrics).unwrap();
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
    fn dropping_encoder_after_header_finalizes_the_muxer_context() {
        let directory = tempfile::tempdir().unwrap();
        let output = directory.path().join("direct-broadcast-drop.flv");
        let target = BroadcastTarget {
            ingest_name: "local-drop-finalization".to_owned(),
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

        {
            let (encoder, _) = BroadcastEncoder::open(&target, &config).unwrap();
            assert!(encoder.output.trailer_pending);
        }

        let input = format::input(&output).expect("dropped encoder should leave a valid FLV");
        assert_eq!(input.streams().len(), 2);
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
        let metrics = BroadcastMetrics::default();
        for video_pts in 0..15_i64 {
            published_video_packets = published_video_packets.saturating_add(
                encoder
                    .encode_video(
                        &VideoFrame {
                            width: u32::from(config.width),
                            height: u32::from(config.height),
                            pixel_format: VideoPixelFormat::Rgba,
                            pixels: rgba.clone(),
                            recycle_pool: None,
                        },
                        video_pts,
                        &metrics,
                    )
                    .unwrap(),
            );
            let samples = vec![0.0; AUDIO_FRAME_SAMPLES * AUDIO_CHANNELS];
            encoder
                .encode_audio(
                    AudioFrame {
                        pts: audio_pts,
                        samples,
                    },
                    &metrics,
                )
                .unwrap();
            audio_pts += i64::try_from(AUDIO_FRAME_SAMPLES).unwrap();
        }
        assert!(
            published_video_packets > 0,
            "the live handshake requires a video packet before verification begins"
        );
        encoder.finish(&metrics).unwrap();
        assert!(std::fs::metadata(output).unwrap().len() > 1_024);
    }
}
