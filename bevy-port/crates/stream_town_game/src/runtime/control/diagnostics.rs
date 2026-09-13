pub(crate) fn capture_screenshot(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut runtime_request: ResMut<RuntimeCaptureRequest>,
    time: Res<Time>,
    state: Res<State<GameState>>,
    menu_loading: Option<Res<MenuLoadingRuntime>>,
    menu_reveal: Option<Res<MenuRevealRuntime>>,
    world_reveal: Option<Res<WorldRevealRuntime>>,
    loading_screens: Query<(), With<LoadingScreenEntity>>,
    mut elapsed: Local<f32>,
    mut automatic_state: Local<Option<GameState>>,
    mut automatic_complete: Local<bool>,
    mut counter: Local<u32>,
    mut exit_delay: Local<Option<f32>>,
    mut exit: MessageWriter<AppExit>,
    mut runtime_console: ResMut<RuntimeConsoleRuntime>,
) {
    // The foliage sweep owns primary-window capture while it is active. A
    // simultaneous F12/runtime/automatic screenshot would make wgpu discard
    // one target and leave a hole in the deterministic frame sequence.
    if std::env::var_os("STREAM_TOWN_FOLIAGE_CAPTURE_DIR").is_some() {
        return;
    }
    if let Some(remaining) = exit_delay.as_mut() {
        *remaining -= time.delta_secs();
        if *remaining <= 0.0 {
            exit.write(AppExit::Success);
            *exit_delay = None;
        }
    }
    let current_state = *state.get();
    let automatic_ready = !matches!(current_state, GameState::Boot | GameState::WorldLoading)
        && menu_loading.is_none()
        && menu_reveal.is_none()
        && world_reveal.is_none()
        && loading_screens.is_empty();
    if *automatic_state != Some(current_state) || !automatic_ready {
        *elapsed = 0.0;
        *automatic_state = Some(current_state);
    } else {
        *elapsed += time.delta_secs();
    }
    let automatic_delay = std::env::var("STREAM_TOWN_SCREENSHOT_DELAY")
        .ok()
        .and_then(|value| value.parse::<f32>().ok())
        .filter(|value| value.is_finite() && *value >= 0.25)
        .unwrap_or(3.0);
    let automatic_path = if automatic_ready && !*automatic_complete && *elapsed >= automatic_delay {
        std::env::var_os("STREAM_TOWN_SCREENSHOT").map(PathBuf::from)
    } else {
        None
    };
    let requested = std::mem::take(&mut runtime_request.0);
    let path = if keyboard.just_pressed(KeyCode::F12) || requested {
        let directory = PathBuf::from(".stream-town").join("screenshots");
        if let Err(error) = std::fs::create_dir_all(&directory) {
            error!(%error, path = %directory.display(), "failed to create screenshot directory");
            return;
        }
        let path = directory.join(format!("stream-town-{:04}.png", *counter));
        *counter += 1;
        Some(path)
    } else {
        automatic_path
    };
    let Some(path) = path else {
        return;
    };
    runtime_console.last_result = format!("Capturing frame to {}", path.display());
    *automatic_complete = true;
    info!(path = %path.display(), "capturing frame");
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(path));
    if std::env::var_os("STREAM_TOWN_EXIT_AFTER_SCREENSHOT").is_some() {
        *exit_delay = Some(1.0);
    }
}

pub(crate) fn report_frame_time_gate(
    time: Res<Time>,
    diagnostics: Option<ResMut<DiagnosticsStore>>,
    stats: Res<WorldRenderStats>,
    mut exit: MessageWriter<AppExit>,
    mut warmed_up: Local<bool>,
    mut reported: Local<bool>,
) {
    if *reported || std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_none() {
        return;
    }
    let Some(mut diagnostics) = diagnostics else {
        return;
    };
    let warmup = std::env::var("STREAM_TOWN_FRAME_TIME_WARMUP")
        .ok()
        .and_then(|value| value.parse::<f32>().ok())
        .filter(|value| value.is_finite() && *value >= 1.0)
        .unwrap_or(10.0);
    let sample_seconds = std::env::var("STREAM_TOWN_FRAME_TIME_SAMPLE_SECONDS")
        .ok()
        .and_then(|value| value.parse::<f32>().ok())
        .filter(|value| value.is_finite() && *value >= 1.0)
        .unwrap_or(10.0);
    if !*warmed_up && time.elapsed_secs() >= warmup {
        if let Some(frame_time) = diagnostics.get_mut(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
            frame_time.clear_history();
        }
        *warmed_up = true;
        return;
    }
    if !*warmed_up || time.elapsed_secs() < warmup + sample_seconds {
        return;
    }
    let Some(frame_time) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) else {
        return;
    };
    let mut values: Vec<_> = frame_time
        .values()
        .copied()
        .filter(|value| value.is_finite())
        .collect();
    if values.is_empty() {
        return;
    }
    values.sort_by(f64::total_cmp);
    let p95_index = (values.len() * 95).div_ceil(100).saturating_sub(1);
    let p95_ms = values[p95_index];
    let sample_count = u32::try_from(values.len()).unwrap_or(u32::MAX);
    let average_ms = values.iter().sum::<f64>() / f64::from(sample_count);
    warn!(
        samples = values.len(),
        average_ms,
        p95_ms,
        budget_ms = 16.7,
        passed = p95_ms < 16.7,
        terrain_high = stats.terrain_high_chunks,
        terrain_medium = stats.terrain_medium_chunks,
        terrain_low = stats.terrain_low_chunks,
        foliage_instances = stats.foliage_instances,
        foliage_visible = stats.foliage_visible_instances,
        foliage_batches = stats.foliage_batches,
        foliage_spatial_groups = stats.foliage_spatial_groups,
        foliage_unbatched = stats.foliage_unbatched_instances,
        actor_detail_budget = actor_scene_budget(),
        animation_detail_budget = animation_detail_budget(),
        crowd_adjusted = stats.crowd_adjusted_agents,
        crowd_yielding = stats.crowd_yielding_agents,
        "steady-state frame-time gate"
    );
    if let Some(path) = std::env::var_os("STREAM_TOWN_PERFORMANCE_REPORT_PATH") {
        let path = PathBuf::from(path);
        if let Some(parent) = path.parent()
            && let Err(error) = std::fs::create_dir_all(parent)
        {
            error!(%error, path = %path.display(), "could not create performance report directory");
        } else {
            let report = serde_json::json!({
                "schema_version": 1,
                "samples": values.len(),
                "average_frame_ms": average_ms,
                "p95_frame_ms": p95_ms,
                "budget_frame_ms": 16.7,
                "passed": p95_ms < 16.7,
                "terrain_lod": {
                    "high": stats.terrain_high_chunks,
                    "medium": stats.terrain_medium_chunks,
                    "low": stats.terrain_low_chunks,
                },
                "foliage_instances": stats.foliage_instances,
                "foliage": {
                    "instances": stats.foliage_instances,
                    "visible_instances": stats.foliage_visible_instances,
                    "batches": stats.foliage_batches,
                    "spatial_groups": stats.foliage_spatial_groups,
                    "unbatched_instances": stats.foliage_unbatched_instances,
                },
                "actor_detail_budget": actor_scene_budget(),
                "animation_detail_budget": animation_detail_budget(),
                "crowd_adjusted_agents": stats.crowd_adjusted_agents,
                "crowd_yielding_agents": stats.crowd_yielding_agents,
            });
            match serde_json::to_vec_pretty(&report)
                .map_err(std::io::Error::other)
                .and_then(|bytes| std::fs::write(&path, bytes))
            {
                Ok(()) => info!(path = %path.display(), "performance report written"),
                Err(error) => {
                    error!(%error, path = %path.display(), "could not write performance report");
                }
            }
        }
    }
    *reported = true;
    if std::env::var_os("STREAM_TOWN_EXIT_AFTER_FRAME_TIME").is_some() {
        exit.write(AppExit::Success);
    }
}
