pub(crate) fn inject_environment_commands(
    mut injected: ResMut<InjectedCommands>,
    mut injected_debug_commands: Local<bool>,
) {
    if !*injected_debug_commands {
        *injected_debug_commands = true;
        if let Some(commands) = std::env::var_os("STREAM_TOWN_DEBUG_COMMANDS") {
            for command in commands.to_string_lossy().split(';') {
                match command.trim().parse() {
                    Ok(command) => injected.0.push_back(PendingChatCommand {
                        actor_id: StableId::new("twitch:debug_viewer").expect("static ID"),
                        login_name: "debug_viewer".to_owned(),
                        display_name: "debug_viewer".to_owned(),
                        command,
                        is_broadcaster: true,
                        is_moderator: true,
                        is_subscriber: true,
                        origin: CommandOrigin::LocalDebug,
                    }),
                    Err(error) => warn!(command, %error, "ignored invalid debug command"),
                }
            }
        }
    }
}

pub(crate) fn start_twitch_transport(
    config: Res<RuntimeConfig>,
    mut connection: ResMut<TwitchConnection>,
) {
    if !config.0.twitch.enabled {
        connection.status = TwitchStatus::Disabled;
        connection.moderation_status = TwitchModerationStatus::Disabled;
        return;
    }
    connection
        .fish_god_reward_id
        .clone_from(&config.0.twitch.fish_god_reward_id);
    connection.status = TwitchStatus::Authorizing;
    connection.moderation_status = TwitchModerationStatus::Authorizing;
    match TwitchTransport::start(config.0.twitch.clone()) {
        Ok(transport) => connection.transport = Some(transport),
        Err(error) => {
            connection.status = TwitchStatus::Error(error.to_string());
            connection.moderation_status = TwitchModerationStatus::Disabled;
        }
    }
}

pub(crate) fn twitch_connection_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<RuntimeConfig>,
    state: Res<State<GameState>>,
    mut connection: ResMut<TwitchConnection>,
) {
    // Once the town is active, gameplay input is exclusively text-command driven.
    if *state.get() == GameState::InGame {
        return;
    }
    if keyboard.just_pressed(KeyCode::F1) {
        if let Some(transport) = connection.transport.take() {
            let _ = transport.send(TwitchControl::Disconnect);
        }
        connection.status = TwitchStatus::Disconnected;
        connection.moderation_status = TwitchModerationStatus::Disabled;
    } else if keyboard.just_pressed(KeyCode::F2) && config.0.twitch.enabled {
        connection.transport = None;
        connection
            .fish_god_reward_id
            .clone_from(&config.0.twitch.fish_god_reward_id);
        connection.status = TwitchStatus::Authorizing;
        connection.moderation_status = TwitchModerationStatus::Authorizing;
        match TwitchTransport::start(config.0.twitch.clone()) {
            Ok(transport) => connection.transport = Some(transport),
            Err(error) => {
                connection.status = TwitchStatus::Error(error.to_string());
                connection.moderation_status = TwitchModerationStatus::Disabled;
            }
        }
    }
}

pub(crate) fn poll_twitch_transport(
    mut connection: ResMut<TwitchConnection>,
    mut injected: ResMut<InjectedCommands>,
    mut operator_chat: ResMut<OperatorChatRuntime>,
    mut acknowledgements: ResMut<CommandAcknowledgementRuntime>,
) {
    let events: Vec<_> = connection
        .transport
        .as_ref()
        .into_iter()
        .flat_map(|transport| std::iter::from_fn(|| transport.try_recv()))
        .collect();
    for event in events {
        let dispatch = match &event {
            TwitchEvent::Chat(message) => operator_chat.push_chat(message),
            TwitchEvent::BroadcasterChatSent(message) => {
                operator_chat.push_chat(message);
                operator_chat.feedback = format!("Sent as @{}", message.login);
                false
            }
            TwitchEvent::BroadcasterChatSendFailed(error) => {
                operator_chat.feedback = format!("Broadcaster chat send failed: {error}");
                false
            }
            TwitchEvent::Notice(message) => {
                operator_chat.push_system(message.clone());
                true
            }
            TwitchEvent::Status(_) | TwitchEvent::ModerationStatus(_) => true,
        };
        if dispatch && handle_twitch_event(event, &mut connection, &mut injected) {
            acknowledgements.acknowledge();
        }
    }
}

pub(crate) fn handle_twitch_event(
    event: TwitchEvent,
    connection: &mut TwitchConnection,
    injected: &mut InjectedCommands,
) -> bool {
    match event {
        TwitchEvent::Status(status) => {
            match &status {
                TwitchStatus::Connected => info!("Twitch IRC connected"),
                TwitchStatus::Error(error) => error!(%error, "Twitch transport error"),
                _ => info!(?status, "Twitch connection state changed"),
            }
            if matches!(
                &status,
                TwitchStatus::Disabled | TwitchStatus::Disconnected | TwitchStatus::Error(_)
            ) && !matches!(
                connection.moderation_status,
                TwitchModerationStatus::Error(_)
            ) {
                connection.moderation_status = TwitchModerationStatus::Disabled;
            }
            connection.status = status;
            false
        }
        TwitchEvent::ModerationStatus(status) => {
            match &status {
                TwitchModerationStatus::Ready => {
                    info!("Twitch broadcaster moderation authorized");
                }
                TwitchModerationStatus::Error(error) => {
                    error!(%error, "Twitch broadcaster moderation authorization error");
                }
                _ => info!(?status, "Twitch broadcaster moderation state changed"),
            }
            connection.moderation_status = status;
            false
        }
        TwitchEvent::Chat(message) => {
            if connection
                .fish_god_reward_id
                .as_deref()
                .is_some_and(|id| message.custom_reward_id.as_deref() == Some(id))
            {
                injected.0.push_back(PendingChatCommand {
                    actor_id: message.actor_id,
                    login_name: message.login,
                    display_name: message.display_name,
                    command: ChatCommand::Praise,
                    is_broadcaster: message.is_broadcaster,
                    is_moderator: message.is_moderator,
                    is_subscriber: message.is_subscriber,
                    origin: CommandOrigin::Twitch,
                });
                return false;
            }
            match parse_chat_commands(&message.message) {
                Ok(commands) => {
                    for command in commands {
                        injected.0.push_back(PendingChatCommand {
                            actor_id: message.actor_id.clone(),
                            login_name: message.login.clone(),
                            display_name: message.display_name.clone(),
                            command,
                            is_broadcaster: message.is_broadcaster,
                            is_moderator: message.is_moderator,
                            is_subscriber: message.is_subscriber,
                            origin: CommandOrigin::Twitch,
                        });
                    }
                    false
                }
                Err(parse_error) => {
                    debug!(user = %message.login, %parse_error, "ignored invalid Twitch command");
                    let acknowledged = invalid_twitch_command_reply(&message.message);
                    if let Some(reply) = acknowledged
                        && let Some(transport) = &connection.transport
                    {
                        let _ = transport.send(TwitchControl::SendBotMessage(reply.to_owned()));
                    }
                    acknowledged.is_some()
                }
            }
        }
        TwitchEvent::Notice(_)
        | TwitchEvent::BroadcasterChatSent(_)
        | TwitchEvent::BroadcasterChatSendFailed(_) => false,
    }
}

pub(crate) fn invalid_twitch_command_reply(message: &str) -> Option<&'static str> {
    message
        .trim_start()
        .starts_with('!')
        .then_some(INVALID_TWITCH_COMMAND_REPLY)
}

pub(crate) fn process_runtime_console(
    state: Res<State<GameState>>,
    mut runtime: ResMut<RuntimeConsoleRuntime>,
    mut injected: ResMut<InjectedCommands>,
    mut persistence: MessageWriter<TownPersistenceRequest>,
    mut capture: ResMut<RuntimeCaptureRequest>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if !runtime.enabled {
        return;
    }
    let request = match runtime.store.read_request() {
        Ok(Some(request)) if request.sequence > runtime.last_processed_sequence => request,
        Ok(_) => return,
        Err(error) => {
            runtime.last_result = format!("Runtime console request failed: {error}");
            return;
        }
    };
    runtime.last_processed_sequence = request.sequence;
    runtime.last_result = match request.action {
        RuntimeConsoleAction::InjectChat {
            actor_id,
            login_name,
            display_name,
            command,
            is_broadcaster,
            is_moderator,
            is_subscriber,
        } => match parse_chat_commands(&command) {
            Ok(commands) => {
                let count = commands.len();
                for command in commands {
                    injected.0.push_back(PendingChatCommand {
                        actor_id: actor_id.clone(),
                        login_name: login_name.clone(),
                        display_name: display_name.clone(),
                        command,
                        is_broadcaster,
                        is_moderator,
                        is_subscriber,
                        origin: CommandOrigin::LocalDebug,
                    });
                }
                format!("Injected {count} local debug command(s)")
            }
            Err(error) => format!("Rejected invalid command: {error}"),
        },
        RuntimeConsoleAction::Save if *state.get() == GameState::InGame => {
            persistence.write(TownPersistenceRequest::Save);
            "Save requested".to_owned()
        }
        RuntimeConsoleAction::SaveJumpStart if *state.get() == GameState::InGame => {
            persistence.write(TownPersistenceRequest::SaveJumpStart);
            "Protected jump-start save requested".to_owned()
        }
        RuntimeConsoleAction::Load if *state.get() == GameState::InGame => {
            persistence.write(TownPersistenceRequest::Load(None));
            "Load requested".to_owned()
        }
        RuntimeConsoleAction::Save
        | RuntimeConsoleAction::SaveJumpStart
        | RuntimeConsoleAction::Load => "Save/load requires an active town".to_owned(),
        RuntimeConsoleAction::CaptureFrame => {
            capture.0 = true;
            "Frame capture requested".to_owned()
        }
        RuntimeConsoleAction::ReturnToMainMenu if *state.get() == GameState::InGame => {
            next_state.set(GameState::MainMenu);
            "Returning to Main Menu".to_owned()
        }
        RuntimeConsoleAction::ReturnToMainMenu => "Already outside the town".to_owned(),
        RuntimeConsoleAction::Exit => {
            exit.write(AppExit::Success);
            "Exit requested".to_owned()
        }
    };
}

pub(crate) fn publish_runtime_console_status(
    time: Res<Time>,
    state: Res<State<GameState>>,
    mut runtime: ResMut<RuntimeConsoleRuntime>,
    world: Option<Res<WorldRuntime>>,
    simulation: Option<Res<SimulationRuntime>>,
    session: Res<SessionStats>,
    render_stats: Res<WorldRenderStats>,
    save: Res<SaveRuntime>,
    twitch: Res<TwitchConnection>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
    diagnostics: Option<Res<DiagnosticsStore>>,
) {
    if !runtime.enabled {
        return;
    }
    runtime.status_elapsed_seconds += time.delta_secs();
    if runtime.status_elapsed_seconds < 0.25 {
        return;
    }
    runtime.status_elapsed_seconds = 0.0;
    let town_resources = simulation
        .as_ref()
        .map_or_else(BTreeMap::new, |simulation| {
            simulation
                .0
                .town_resources
                .iter()
                .map(|(id, amount)| (id.as_str().to_owned(), u64::from(*amount)))
                .collect()
        });
    let frame_times = diagnostics
        .as_ref()
        .and_then(|diagnostics| diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME))
        .map_or_else(Vec::new, |frame_time| {
            frame_time
                .values()
                .copied()
                .filter(|value| value.is_finite())
                .collect::<Vec<_>>()
        });
    let average_frame_ms = (!frame_times.is_empty()).then(|| {
        let sample_count = u32::try_from(frame_times.len()).unwrap_or(u32::MAX);
        frame_times.iter().sum::<f64>() / f64::from(sample_count)
    });
    let mut sorted_frame_times = frame_times;
    sorted_frame_times.sort_by(f64::total_cmp);
    let p95_frame_ms = (!sorted_frame_times.is_empty()).then(|| {
        let index = ((sorted_frame_times.len() - 1) * 95) / 100;
        sorted_frame_times[index]
    });
    let status = RuntimeConsoleStatus {
        schema_version: CURRENT_RUNTIME_CONSOLE_SCHEMA,
        process_id: std::process::id(),
        updated_unix_millis: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| {
                u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
            }),
        state: format!("{:?}", state.get()),
        world_seed: world.as_ref().map(|world| world.generated.seed),
        world_hash: world
            .as_ref()
            .map(|world| world.generated.deterministic_hash.clone()),
        elapsed_seconds: session.elapsed_seconds,
        actor_count: simulation
            .as_ref()
            .map_or(0, |simulation| simulation.0.actors.len()),
        building_count: simulation
            .as_ref()
            .map_or(0, |simulation| simulation.0.buildings.len()),
        town_resources,
        paths_completed: session.paths_completed,
        commands_processed: session.commands_processed,
        average_frame_ms,
        p95_frame_ms,
        terrain_high_chunks: render_stats.terrain_high_chunks,
        terrain_medium_chunks: render_stats.terrain_medium_chunks,
        terrain_low_chunks: render_stats.terrain_low_chunks,
        foliage_instances: render_stats.foliage_instances,
        foliage_visible_instances: render_stats.foliage_visible_instances,
        foliage_batches: render_stats.foliage_batches,
        foliage_spatial_groups: render_stats.foliage_spatial_groups,
        foliage_unbatched_instances: render_stats.foliage_unbatched_instances,
        crowd_adjusted_agents: render_stats.crowd_adjusted_agents,
        crowd_yielding_agents: render_stats.crowd_yielding_agents,
        save_exists: save.store.path().is_file(),
        save_path: save.store.path().display().to_string(),
        twitch_status: format!("{:?}", twitch.status),
        direct_broadcast_status: {
            #[cfg(target_os = "windows")]
            {
                let snapshot = broadcast.snapshot();
                format!(
                    "{:?}; encoder={}; ingest={}; video={}/{} fps={:.1}/{:.1} dropped={} replaced={} skipped={}; audio={} dropped={} queue={}/{}; capture_ms={:.2}/{:.2} encode_ms={:.2}/{:.2}",
                    snapshot.phase,
                    snapshot.encoder.as_deref().unwrap_or("pending"),
                    snapshot.ingest.as_deref().unwrap_or("pending"),
                    snapshot.encoded_video_frames,
                    snapshot.captured_video_frames,
                    snapshot.captured_video_fps,
                    snapshot.encoded_video_fps,
                    snapshot.dropped_video_frames,
                    snapshot.replaced_video_frames,
                    snapshot.skipped_video_frames,
                    snapshot.encoded_audio_frames,
                    snapshot.dropped_audio_frames,
                    snapshot.audio_queue_depth,
                    snapshot.audio_queue_high_water,
                    snapshot.average_capture_ms,
                    snapshot.maximum_capture_ms,
                    snapshot.average_encode_ms,
                    snapshot.maximum_encode_ms,
                )
            }
            #[cfg(not(target_os = "windows"))]
            {
                "Unsupported on this platform".to_owned()
            }
        },
        last_processed_sequence: runtime.last_processed_sequence,
        last_result: runtime.last_result.clone(),
    };
    if let Err(error) = runtime.store.write_status(&status) {
        warn!(%error, "could not publish runtime console status");
    }
}
