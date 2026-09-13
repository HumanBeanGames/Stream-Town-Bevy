fn runtime_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Runtime developer console");
    let attached = runtime_console_attached(state.runtime_status.as_ref());
    ui.horizontal_wrapped(|ui| {
        ui.label(if attached {
            "Connected"
        } else {
            "Not attached"
        });
        ui.separator();
        ui.checkbox(&mut state.runtime_profile_launch, "Profile launch");
        if ui.button("Launch game").clicked() {
            state.status = match launch_runtime_game(state) {
                Ok(()) => "Launched the Bevy game and waiting for runtime status".to_owned(),
                Err(error) => format!("Could not launch game: {error:#}"),
            };
        }
        if ui.button("Refresh/attach").clicked() {
            poll_runtime_console(state);
            state.status = if runtime_console_attached(state.runtime_status.as_ref()) {
                "Attached to the local Bevy runtime".to_owned()
            } else {
                "No live local runtime status was found".to_owned()
            };
        }
    });
    ui.label(format!(
        "Control channel: {}",
        state.runtime_console.root().display()
    ));
    if let Some(status) = &state.runtime_status {
        egui::Grid::new("runtime_status")
            .num_columns(4)
            .show(ui, |ui| {
                ui.label("PID");
                ui.monospace(status.process_id.to_string());
                ui.label("State");
                ui.monospace(&status.state);
                ui.end_row();
                ui.label("World");
                ui.monospace(status.world_hash.as_deref().unwrap_or("Not loaded"));
                ui.label("Seed");
                ui.monospace(
                    status
                        .world_seed
                        .map_or_else(|| "-".to_owned(), |seed| seed.to_string()),
                );
                ui.end_row();
                ui.label("Actors / buildings");
                ui.monospace(format!(
                    "{} / {}",
                    status.actor_count, status.building_count
                ));
                ui.label("Elapsed / routes");
                ui.monospace(format!(
                    "{:.1}s / {}",
                    status.elapsed_seconds, status.paths_completed
                ));
                ui.end_row();
                ui.label("Frame avg / p95");
                ui.monospace(format_runtime_frame_times(status));
                ui.label("Commands");
                ui.monospace(status.commands_processed.to_string());
                ui.end_row();
                ui.label("Terrain H / M / L");
                ui.monospace(format!(
                    "{} / {} / {}",
                    status.terrain_high_chunks,
                    status.terrain_medium_chunks,
                    status.terrain_low_chunks
                ));
                ui.label("Foliage active / total");
                ui.monospace(format!(
                    "{} / {}",
                    status.foliage_visible_instances, status.foliage_instances
                ));
                ui.end_row();
                ui.label("Foliage GPU / spatial batches");
                ui.monospace(format!(
                    "{} / {}",
                    status.foliage_batches, status.foliage_spatial_groups
                ));
                ui.label("Fallback / separated / yielding");
                ui.monospace(format!(
                    "{} / {} / {}",
                    status.foliage_unbatched_instances,
                    status.crowd_adjusted_agents,
                    status.crowd_yielding_agents
                ));
                ui.end_row();
                ui.label("Twitch");
                ui.monospace(&status.twitch_status);
                ui.label("Direct broadcast");
                ui.monospace(&status.direct_broadcast_status);
                ui.end_row();
                ui.label("Save");
                ui.monospace(if status.save_exists {
                    "Available"
                } else {
                    "Missing"
                });
                ui.label("");
                ui.label("");
                ui.end_row();
            });
        ui.label(format!("Last result: {}", status.last_result));
        if !status.town_resources.is_empty() {
            ui.label(format!(
                "Resources: {}",
                status
                    .town_resources
                    .iter()
                    .map(|(id, amount)| format!("{id}={amount}"))
                    .collect::<Vec<_>>()
                    .join("  ")
            ));
        }
    }
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Stable actor ID");
        ui.text_edit_singleline(&mut state.runtime_actor_id);
        ui.label("Login");
        ui.text_edit_singleline(&mut state.runtime_login);
    });
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut state.command);
        if ui
            .add_enabled(attached, egui::Button::new("Inject command"))
            .clicked()
        {
            state.status = match inject_runtime_command(state) {
                Ok(()) => format!("Injected {} through the local debug channel", state.command),
                Err(error) => format!("Could not inject command: {error:#}"),
            };
        }
    });
    ui.horizontal(|ui| {
        if ui.button("Prepare Fish God start").clicked() {
            "!event fish_god".clone_into(&mut state.command);
        }
        if ui.button("Prepare praise").clicked() {
            "!praise".clone_into(&mut state.command);
        }
        if ui.button("Prepare raid").clicked() {
            "!event raid".clone_into(&mut state.command);
        }
        if ui.button("Prepare ruler vote").clicked() {
            "!rulervote".clone_into(&mut state.command);
        }
        if ui.button("Prepare recruit").clicked() {
            "!recruit miner 1".clone_into(&mut state.command);
        }
    });
    ui.horizontal_wrapped(|ui| {
        for (label, action) in [
            ("Save", RuntimeConsoleAction::Save),
            ("Save Jump-Start", RuntimeConsoleAction::SaveJumpStart),
            ("Load", RuntimeConsoleAction::Load),
            ("Capture frame", RuntimeConsoleAction::CaptureFrame),
            ("Main Menu", RuntimeConsoleAction::ReturnToMainMenu),
            ("Exit game", RuntimeConsoleAction::Exit),
        ] {
            if ui.add_enabled(attached, egui::Button::new(label)).clicked() {
                state.status = match send_runtime_action(state, action) {
                    Ok(()) => format!("Sent runtime action: {label}"),
                    Err(error) => format!("Could not send runtime action: {error:#}"),
                };
            }
        }
    });
    ui.label("Local tool injection uses an explicit debug-only bypass; the control directory contains no OAuth credentials.");
}

fn poll_runtime_console(state: &mut ToolState) {
    match state.runtime_console.read_status() {
        Ok(status) => state.runtime_status = status,
        Err(error) => state.status = format!("Runtime status could not be read: {error}"),
    }
    if state
        .runtime_process
        .as_mut()
        .is_some_and(|process| process.try_wait().ok().flatten().is_some())
    {
        state.runtime_process = None;
    }
}

fn runtime_console_attached(status: Option<&RuntimeConsoleStatus>) -> bool {
    let Some(status) = status else {
        return false;
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| {
            u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
        });
    now.saturating_sub(status.updated_unix_millis) < 2_000
}

fn launch_runtime_game(state: &mut ToolState) -> anyhow::Result<()> {
    if state.runtime_process.is_some() {
        anyhow::bail!("the tools app already owns a running game process");
    }
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut command = Command::new("cargo");
    command
        .args(["run", "--release", "-p", "stream_town_game"])
        .current_dir(workspace)
        .env(
            "STREAM_TOWN_RUNTIME_CONSOLE_DIR",
            state.runtime_console.root(),
        )
        .env("STREAM_TOWN_AUTOSTART", "1")
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if state.runtime_profile_launch {
        command
            .env("STREAM_TOWN_REPORT_FRAME_TIME", "1")
            .env("STREAM_TOWN_FRAME_TIME_WARMUP", "5")
            .env("STREAM_TOWN_FRAME_TIME_SAMPLE_SECONDS", "10");
    }
    state.runtime_process = Some(command.spawn()?);
    Ok(())
}

fn inject_runtime_command(state: &mut ToolState) -> anyhow::Result<()> {
    let _: ChatCommand = state.command.parse()?;
    let actor_id = StableId::new(state.runtime_actor_id.clone())?;
    let login_name = state.runtime_login.trim();
    if login_name.is_empty() {
        anyhow::bail!("runtime login cannot be empty");
    }
    send_runtime_action(
        state,
        RuntimeConsoleAction::InjectChat {
            actor_id,
            login_name: login_name.to_owned(),
            display_name: login_name.to_owned(),
            command: state.command.clone(),
            is_broadcaster: true,
            is_moderator: true,
            is_subscriber: true,
        },
    )
}

fn send_runtime_action(state: &mut ToolState, action: RuntimeConsoleAction) -> anyhow::Result<()> {
    let acknowledged = state
        .runtime_status
        .as_ref()
        .map_or(0, |status| status.last_processed_sequence);
    state.runtime_sequence = state.runtime_sequence.max(acknowledged).saturating_add(1);
    state
        .runtime_console
        .write_request(&RuntimeConsoleRequest::new(state.runtime_sequence, action))?;
    Ok(())
}

fn format_runtime_frame_times(status: &RuntimeConsoleStatus) -> String {
    match (status.average_frame_ms, status.p95_frame_ms) {
        (Some(average), Some(p95)) => format!("{average:.2} / {p95:.2} ms"),
        _ => "Collecting".to_owned(),
    }
}

fn parse_game_master_ids(value: &str) -> BTreeSet<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(str::to_owned)
        .collect()
}

fn format_game_master_ids(ids: &BTreeSet<String>) -> String {
    ids.iter().cloned().collect::<Vec<_>>().join(", ")
}

fn sync_twitch_tool_fields(state: &mut ToolState) {
    state.game_master_ids = format_game_master_ids(&state.config.twitch.game_master_ids);
    state.fish_god_reward_id = state
        .config
        .twitch
        .fish_god_reward_id
        .clone()
        .unwrap_or_default();
    state.twitch_channel_identity = None;
    state.twitch_irc_verified = false;
}
