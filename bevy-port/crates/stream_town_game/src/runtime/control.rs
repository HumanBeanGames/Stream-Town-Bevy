use super::super::*;

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
    mut io: ResMut<MenuIoRequest>,
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
            io.save = true;
            "Save requested".to_owned()
        }
        RuntimeConsoleAction::SaveJumpStart if *state.get() == GameState::InGame => {
            io.save_jump_start = true;
            "Protected jump-start save requested".to_owned()
        }
        RuntimeConsoleAction::Load if *state.get() == GameState::InGame => {
            io.load = true;
            io.load_source = None;
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

pub(crate) fn save_input(
    mut io: ResMut<MenuIoRequest>,
    save: Res<SaveRuntime>,
    world: Res<WorldRuntime>,
    stats: Res<SessionStats>,
    simulation: Res<SimulationRuntime>,
    traversal_wear: Res<TraversalWearRuntime>,
    mut runtime_console: ResMut<RuntimeConsoleRuntime>,
) {
    let save_requested = std::mem::take(&mut io.save);
    let jump_start_requested = std::mem::take(&mut io.save_jump_start);
    if !save_requested && !jump_start_requested {
        return;
    }
    let snapshot = snapshot_world(&world, &stats, &simulation, &traversal_wear);
    let destination = if jump_start_requested {
        NativeSaveStore::new(jump_start_snapshot_path(save.store.path()))
    } else {
        NativeSaveStore::new(save.store.path())
    };
    match destination.write(&snapshot) {
        Ok(()) => {
            runtime_console.last_result = if jump_start_requested {
                format!(
                    "Protected jump-start saved to {}",
                    destination.path().display()
                )
            } else {
                format!("Saved {}", destination.path().display())
            };
            info!(
                path = %destination.path().display(),
                protected = jump_start_requested,
                "native save written"
            );
        }
        Err(error) => {
            runtime_console.last_result = format!("Save failed: {error}");
            error!(%error, "native save failed");
        }
    }
}

pub(crate) fn autosave_game(
    time: Res<Time>,
    mut settings: ResMut<PlayerSettingsRuntime>,
    player_settings: Res<RuntimePlayerSettings>,
    save: Res<SaveRuntime>,
    world: Res<WorldRuntime>,
    stats: Res<SessionStats>,
    simulation: Res<SimulationRuntime>,
    traversal_wear: Res<TraversalWearRuntime>,
) {
    if player_settings.0.autosave_minutes == 0 {
        settings.autosave_elapsed_seconds = 0.0;
        return;
    }
    settings.autosave_elapsed_seconds += time.delta_secs();
    let interval = f32::from(player_settings.0.autosave_minutes) * 60.0;
    if settings.autosave_elapsed_seconds < interval {
        return;
    }
    settings.autosave_elapsed_seconds = settings.autosave_elapsed_seconds.rem_euclid(interval);
    let snapshot = snapshot_world(&world, &stats, &simulation, &traversal_wear);
    match save.store.write(&snapshot) {
        Ok(()) => info!(path = %save.store.path().display(), "autosave written"),
        Err(error) => error!(%error, "autosave failed"),
    }
}

pub(crate) fn save_on_gameplay_exit(
    save: Res<SaveRuntime>,
    restart: Res<TownRestartRuntime>,
    ready: Option<Res<GameplayReady>>,
    world: Option<Res<WorldRuntime>>,
    stats: Option<Res<SessionStats>>,
    simulation: Option<Res<SimulationRuntime>>,
    traversal_wear: Option<Res<TraversalWearRuntime>>,
) {
    if restart.suppress_exit_save || ready.is_none() {
        return;
    }
    let (Some(world), Some(stats), Some(simulation), Some(traversal_wear)) =
        (world, stats, simulation, traversal_wear)
    else {
        return;
    };
    let snapshot = snapshot_world(&world, &stats, &simulation, &traversal_wear);
    match save.store.write(&snapshot) {
        Ok(()) => info!(path = %save.store.path().display(), "gameplay-exit save written"),
        Err(error) => error!(%error, "gameplay-exit save failed"),
    }
}

pub(crate) fn automatic_load_requested(
    automatic_complete: bool,
    legacy: bool,
    resume: bool,
) -> bool {
    !automatic_complete && (legacy || resume)
}

pub(crate) fn migrate_legacy_path_navigation_positions(simulation: &mut WorldSimulation) -> usize {
    const LOCAL_CANDIDATES: [(u16, u16); 9] = [
        (1, 1),
        (1, 0),
        (2, 1),
        (1, 2),
        (0, 1),
        (2, 0),
        (2, 2),
        (0, 2),
        (0, 0),
    ];
    let mut occupied = simulation
        .path_navigation_positions
        .values()
        .copied()
        .collect::<BTreeSet<_>>();
    let legacy_paths = simulation
        .buildings
        .values()
        .filter(|building| building.archetype.as_str() == "archetype:building:path")
        .filter(|building| {
            !simulation
                .path_navigation_positions
                .contains_key(&building.id)
        })
        .map(|building| (building.id.clone(), building.position))
        .collect::<Vec<_>>();
    let mut migrated = 0;
    for (id, coarse) in legacy_paths {
        let base_x = coarse.x.saturating_mul(NAVIGATION_SUBDIVISIONS);
        let base_z = coarse.z.saturating_mul(NAVIGATION_SUBDIVISIONS);
        let Some(position) = LOCAL_CANDIDATES
            .iter()
            .map(|(x, z)| GridPos {
                x: base_x.saturating_add(*x),
                z: base_z.saturating_add(*z),
            })
            .find(|position| !occupied.contains(position))
        else {
            continue;
        };
        occupied.insert(position);
        simulation.path_navigation_positions.insert(id, position);
        migrated += 1;
    }
    migrated
}

pub(crate) fn load_input(
    mut ecs: Commands,
    mut io: ResMut<MenuIoRequest>,
    save: Res<SaveRuntime>,
    mut automatic_resume: ResMut<AutomaticResumeRuntime>,
    mut world: ResMut<WorldRuntime>,
    mut config: ResMut<RuntimeConfig>,
    content: Res<RuntimeContent>,
    mut load_render: LoadRenderParams,
    mut placers: ResMut<BuildingPlacers>,
    mut stats: ResMut<SessionStats>,
    mut simulation: ResMut<SimulationRuntime>,
    mut traversal_wear: ResMut<TraversalWearRuntime>,
    mut selected: ResMut<SelectedCell>,
    mut entities: LoadWorldEntities,
    mut automatic_complete: Local<bool>,
    mut runtime_console: ResMut<RuntimeConsoleRuntime>,
) {
    let automatic = automatic_load_requested(
        *automatic_complete,
        std::env::var_os("STREAM_TOWN_AUTO_LOAD").is_some(),
        automatic_resume.path.is_some(),
    );
    let requested = std::mem::take(&mut io.load);
    let requested_source = io.load_source.take();
    if !automatic && !requested {
        return;
    }
    *automatic_complete = true;
    let source_store = requested_source.as_ref().map_or_else(
        || NativeSaveStore::new(save.store.path()),
        NativeSaveStore::new,
    );
    let source_path = source_store.path().to_path_buf();
    let mut snapshot = match source_store.load() {
        Ok(snapshot) => snapshot,
        Err(error) => {
            runtime_console.last_result = format!("Load failed: {error}");
            error!(%error, "native load failed");
            return;
        }
    };
    let mut persisted_upgrade = snapshot.simulation.schema_version < CURRENT_SIMULATION_SCHEMA;
    selected.0 = None;
    ecs.insert_resource(SelectedActor::default());
    let mut restored_config = config.0.clone();
    restored_config.world.seed = snapshot.world_seed;
    let mut restored_world = generate_world_with_content(&restored_config.world, &content.0);
    let saved_generator_version = snapshot.generator_version;
    let compatibility = native_world_compatibility(
        snapshot.world_seed,
        snapshot.generator_version,
        &snapshot.world_hash,
        &restored_world,
    );
    if compatibility.is_none() {
        runtime_console.last_result = format!(
            "Load failed: saved world {} does not match generator {}",
            snapshot.world_hash, restored_world.deterministic_hash
        );
        error!(
            saved_seed = snapshot.world_seed,
            runtime_seed = world.generated.seed,
            "native save world identity does not match the loaded world"
        );
        return;
    }
    if matches!(
        compatibility,
        Some(
            NativeWorldCompatibility::UpgradeV1
                | NativeWorldCompatibility::UpgradeV2
                | NativeWorldCompatibility::UpgradeV3
                | NativeWorldCompatibility::RegeneratePrior
        )
    ) {
        persisted_upgrade = true;
        info!(
            saved_generator_version = snapshot.generator_version,
            runtime_generator_version = restored_world.generator_version,
            "upgrading native save world fingerprint and proportionally migrating resource stock"
        );
        snapshot.generator_version = restored_world.generator_version;
        snapshot
            .world_hash
            .clone_from(&restored_world.deterministic_hash);
    }

    if !snapshot.resource_nodes.is_empty() {
        let restored_regrowth = snapshot
            .resource_nodes
            .iter()
            .filter(|(id, _)| {
                !restored_world
                    .resources
                    .iter()
                    .any(|resource| resource.id == **id)
            })
            .filter_map(|(id, amount)| restored_regenerated_resource(id, *amount))
            .filter(|resource| {
                resource.position.x < restored_world.navigation.width()
                    && resource.position.z < restored_world.navigation.height()
            })
            .collect::<Vec<_>>();
        restored_world.resources.extend(restored_regrowth);
        let mut depleted_land = Vec::new();
        for resource in &mut restored_world.resources {
            if let Some(remaining) = snapshot.resource_nodes.get(&resource.id) {
                resource.amount =
                    upgraded_resource_remaining(saved_generator_version, resource, *remaining);
                if resource.amount == 0 && resource.target_kind.as_str() != "target:fish" {
                    depleted_land.push(resource.position);
                }
            }
        }
        for position in depleted_land {
            if !resource_cell_has_active_generation_occupant(&restored_world.resources, position) {
                let _ = restored_world.navigation.set_blocked(
                    stream_town_domain::DirtyRegion {
                        min: position,
                        max: position,
                    },
                    false,
                );
            }
        }
        for resource in restored_world.resources.iter().filter(|resource| {
            resource.amount > 0 && resource.id.as_str().starts_with("resource:regrown_")
        }) {
            let _ = restored_world.navigation.set_blocked(
                stream_town_domain::DirtyRegion {
                    min: resource.position,
                    max: resource.position,
                },
                true,
            );
        }
    }
    ensure_town_hall_state(&content.0, &restored_config, &mut snapshot.simulation);
    snapshot
        .simulation
        .upgrade_time_schema(restored_config.time.seconds_per_day);
    let migrated_paths = migrate_legacy_path_navigation_positions(&mut snapshot.simulation);
    if migrated_paths > 0 {
        persisted_upgrade = true;
        info!(
            paths = migrated_paths,
            "migrated coarse saved paths onto the fine navigation grid"
        );
    }
    normalize_building_health(&content.0, &mut snapshot.simulation);
    let capped_recruit_progress =
        normalize_recruit_role_progression(&content.0, &mut snapshot.simulation);
    if capped_recruit_progress > 0 {
        persisted_upgrade = true;
        info!(
            adjusted_professions = capped_recruit_progress,
            maximum_level = content.0.progression.maximum_recruit_role_level,
            "capped loaded recruit profession levels"
        );
    }
    if entities.town_halls.single_mut().is_err() {
        "Load failed: the persistent Town Hall visual is unavailable"
            .clone_into(&mut runtime_console.last_result);
        error!("the persistent Town Hall visual is unavailable during native load preflight");
        return;
    }
    for saved in snapshot.simulation.buildings.values() {
        let Some((building_id, building)) = content
            .0
            .buildings
            .iter()
            .find(|(_, building)| building.archetype == saved.archetype)
        else {
            runtime_console.last_result = format!(
                "Load failed: building {} references unknown archetype {}",
                saved.id, saved.archetype
            );
            error!(
                building = %saved.id,
                archetype = %saved.archetype,
                "native save references an unknown building archetype"
            );
            return;
        };
        if !content.0.archetypes.contains_key(&building.archetype) {
            runtime_console.last_result = format!(
                "Load failed: building definition {} references unknown prefab archetype {}",
                building_id, building.archetype
            );
            error!(
                building = %saved.id,
                definition = %building_id,
                archetype = %building.archetype,
                "native save building definition references an unknown prefab archetype"
            );
            return;
        }
        let Some(region) = building_navigation_region(
            saved.position,
            building,
            saved.rotation_quarter_turns,
            &restored_world,
        ) else {
            runtime_console.last_result =
                format!("Load failed: building {} lies outside the world", saved.id);
            error!(building = %saved.id, "native save building lies outside the world");
            return;
        };
        if building_blocks_navigation(building)
            && let Err(error) = restored_world.navigation.set_blocked(region, true)
        {
            runtime_console.last_result = format!(
                "Load failed: building {} cannot update navigation",
                saved.id
            );
            error!(building = %saved.id, %error, "native save building could not update navigation");
            return;
        }
    }
    let repaired_enemy_camps = seed_generated_enemy_camps(
        &restored_config,
        &content.0,
        &mut restored_world,
        &mut snapshot.simulation,
    );
    if repaired_enemy_camps > 0 {
        persisted_upgrade = true;
        info!(
            camps = repaired_enemy_camps,
            "repaired native save created before enemy camp generation was enabled"
        );
    }
    for camp in snapshot.simulation.enemy_camps.values() {
        let Some(archetype) = content.0.archetypes.get(&camp.archetype) else {
            runtime_console.last_result = format!(
                "Load failed: enemy camp {} references unknown archetype {}",
                camp.id, camp.archetype
            );
            error!(camp = %camp.id, archetype = %camp.archetype, "native save references an unknown enemy camp");
            return;
        };
        let Some(region) =
            enemy_camp_navigation_region(camp.position, archetype.footprint, &restored_world)
        else {
            runtime_console.last_result =
                format!("Load failed: enemy camp {} lies outside the world", camp.id);
            error!(camp = %camp.id, "native save enemy camp lies outside the world");
            return;
        };
        if let Err(error) = restored_world.navigation.set_blocked(region, true) {
            runtime_console.last_result = format!(
                "Load failed: enemy camp {} cannot update navigation",
                camp.id
            );
            error!(camp = %camp.id, %error, "native save enemy camp could not update navigation");
            return;
        }
    }

    let terrain_replacement = if let Some(meshes) = load_render.meshes.as_mut() {
        let terrain_meshes = match snapshot.legacy_terrain_mesh.as_ref() {
            Some(saved) => match retained_terrain_mesh(saved) {
                Ok(mesh) => vec![("TerrainLegacy".to_owned(), mesh, None)],
                Err(error) => {
                    runtime_console.last_result = format!(
                        "Load failed: retained terrain could not be reconstructed: {error}"
                    );
                    error!(%error, "native save retained terrain could not be reconstructed");
                    return;
                }
            },
            None => generated_terrain_chunks(&restored_world, &restored_config)
                .into_iter()
                .map(|chunk| {
                    (
                        format!("TerrainChunk_{}_{}", chunk.chunk_x, chunk.chunk_z),
                        chunk.high,
                        Some((chunk.centre, chunk.medium, chunk.low)),
                    )
                })
                .collect(),
        };
        let mut replacements = Vec::with_capacity(terrain_meshes.len());
        for (name, mesh, lod_meshes) in terrain_meshes {
            let Some(collider) = Collider::trimesh_from_mesh(&mesh) else {
                "Load failed: saved terrain does not produce a valid collider"
                    .clone_into(&mut runtime_console.last_result);
                error!("native save terrain does not produce a valid triangle collider");
                return;
            };
            let high = meshes.add(mesh);
            let lod = lod_meshes.map(|(centre, medium, low)| TerrainChunkLod {
                centre,
                high: high.clone(),
                medium: meshes.add(medium),
                low: meshes.add(low),
                current: TerrainLodLevel::High,
            });
            replacements.push((name, high, lod, collider));
        }
        Some(replacements)
    } else {
        if snapshot.legacy_terrain_mesh.is_some() {
            "Load failed: retained terrain requires rendering assets"
                .clone_into(&mut runtime_console.last_result);
            error!("native save contains retained terrain but rendering assets are unavailable");
            return;
        }
        None
    };
    let water_replacement = load_render
        .meshes
        .as_mut()
        .map(|meshes| meshes.add(generated_water_mesh(&restored_world, &restored_config)));

    placers.0.clear();
    for (entity, building) in &entities.runtime_buildings {
        debug!(building = %building.id, "despawning runtime building before native load");
        ecs.entity(entity).despawn();
    }
    for (entity, camp) in &entities.enemy_camps {
        debug!(camp = %camp.id, "despawning enemy camp before native load");
        ecs.entity(entity).despawn();
    }
    for saved in snapshot.simulation.buildings.values() {
        let (building_id, building) = content
            .0
            .buildings
            .iter()
            .find(|(_, building)| building.archetype == saved.archetype)
            .expect("building references were checked during native load preflight");
        if saved.id.as_str() == "building:townhall" {
            continue;
        }
        spawn_runtime_building(
            &mut ecs,
            &restored_config,
            &restored_world,
            &load_render.presentation.0,
            load_render.asset_server.as_deref(),
            &load_render.asset_root.0,
            &load_render.render,
            saved,
            building,
            content
                .0
                .archetypes
                .get(&building.archetype)
                .expect("building prefab was checked during native load preflight"),
            saved.position,
            building.footprint,
            building_age(&content.0, &snapshot.simulation, building_id, saved.level),
        );
    }
    for camp in snapshot.simulation.enemy_camps.values() {
        let archetype = content
            .0
            .archetypes
            .get(&camp.archetype)
            .expect("enemy camp archetype was checked during native load preflight");
        spawn_enemy_camp(
            &mut ecs,
            &restored_config,
            &restored_world,
            &load_render.presentation.0,
            load_render.asset_server.as_deref(),
            &load_render.asset_root.0,
            &load_render.render,
            &camp.id,
            archetype,
            camp.position,
        );
    }
    if let Some(replacements) = terrain_replacement {
        for entity in &load_render.terrain_surfaces {
            ecs.entity(entity).despawn();
        }
        for (name, mesh, lod, collider) in replacements {
            let mut terrain = ecs.spawn((
                WorldEntity,
                TerrainSurface,
                Name::new(name),
                Mesh3d(mesh),
                MeshMaterial3d(load_render.render.ground.clone()),
                collider,
                RigidBody::Static,
            ));
            if let Some(lod) = lod {
                terrain.insert(lod);
            }
        }
    }
    if let Some(water_mesh) = water_replacement {
        for entity in &load_render.water_surfaces {
            ecs.entity(entity).despawn();
        }
        ecs.spawn((
            WorldEntity,
            WaterSurface,
            Mesh3d(water_mesh),
            MeshMaterial3d(load_render.render.water.clone()),
        ));
    }
    for entity in &entities.world_fish_schools {
        ecs.entity(entity).despawn();
    }
    spawn_fish_school_scene(
        &mut ecs,
        &load_render.presentation.0,
        &load_render.render,
        WORLD_SCENE_PATH,
        restored_world.seed,
        f32::from(restored_config.world.water_level_centimetres) * 0.01,
        false,
        None,
        Some((&restored_world, &restored_config)),
    );
    // Manual/native loads may replace the generated height field after these
    // visuals have already resolved their AABBs. Force every foliage/resource
    // renderer back through the final-height grounding and habitat gate.
    for entity in &entities.surface_visuals {
        ecs.entity(entity).insert((
            PendingSurfaceGrounding {
                surface_height: 0.0,
            },
            Visibility::Hidden,
        ));
    }
    let town_hall_id = StableId::new("building:townhall").expect("static ID");
    let saved_town_hall = &snapshot.simulation.buildings[&town_hall_id];
    let town_hall_definition = &content.0.buildings[&town_hall_id];
    let town_hall_footprint = rotated_footprint(
        town_hall_definition.footprint,
        saved_town_hall.rotation_quarter_turns,
    );
    let town_hall_centre = GridPos {
        x: saved_town_hall.position.x + town_hall_footprint[0] / 2,
        z: saved_town_hall.position.z + town_hall_footprint[1] / 2,
    };
    let town_hall_surface =
        grid_to_world_on_surface(town_hall_centre, &restored_config, &restored_world);
    if let Ok((mut location, mut presentation, mut transform)) = entities.town_halls.single_mut() {
        location.0 = saved_town_hall.position;
        presentation.base_translation =
            town_hall_surface + Vec3::Y * presentation.base_height_offset;
        transform.translation = presentation.base_translation;
        transform.rotation = quarter_turn_rotation(saved_town_hall.rotation_quarter_turns);
    } else {
        error!("the persistent Town Hall visual is unavailable during native load");
        return;
    }
    world.generated = restored_world;
    world
        .legacy_terrain_mesh
        .clone_from(&snapshot.legacy_terrain_mesh);
    world
        .legacy_migration
        .clone_from(&snapshot.legacy_migration);

    let saved_by_id: BTreeMap<StableId, SavedActor> = snapshot
        .actors
        .iter()
        .filter(|actor| matches!(actor.kind, ActorKind::Player | ActorKind::Enemy))
        .cloned()
        .map(|actor| (actor.id.clone(), actor))
        .collect();
    let mut restored_ids = BTreeSet::new();
    for (entity, mut agent, mut location, animation, mut transform) in &mut entities.agents {
        let Some(saved) = saved_by_id.get(&agent.id) else {
            ecs.entity(entity).despawn();
            continue;
        };
        let position = restored_actor_position(
            &world.generated,
            &content.0,
            &snapshot.simulation,
            &saved.kind,
            saved.grid_position,
        )
        .unwrap_or(saved.grid_position);
        if let Some(actor) = snapshot.simulation.actors.get_mut(&saved.id) {
            actor.position = position;
        }
        let mut world_position =
            grid_to_world_on_surface(position, &restored_config, &world.generated);
        if !animation.native {
            world_position.y += animation.base_scale.y * 0.5;
        }
        agent.kind = saved.kind.clone();
        agent.archetype = saved.archetype.clone();
        agent.goal = AgentGoal::Wander;
        agent.action_started = false;
        agent.spawn = position;
        agent.origin = position;
        agent.navigation_position = placement_to_navigation_centre(position);
        agent.path.clear();
        agent.path_index = 0;
        agent.target = deterministic_wander_target(&world.generated, &agent.id, position);
        agent.action_cooldown_seconds = 0.0;
        agent.action_started = false;
        agent.repath_remaining_seconds = 0.0;
        agent.wander_sequence = 0;
        agent.previous_wander_origin = None;
        location.0 = position;
        transform.translation = world_position;
        restored_ids.insert(saved.id.clone());
    }

    for saved in saved_by_id.values() {
        if restored_ids.contains(&saved.id) {
            continue;
        }
        let position = restored_actor_position(
            &world.generated,
            &content.0,
            &snapshot.simulation,
            &saved.kind,
            saved.grid_position,
        )
        .unwrap_or(saved.grid_position);
        if let Some(actor) = snapshot.simulation.actors.get_mut(&saved.id) {
            actor.position = position;
        }
        if saved.kind == ActorKind::Enemy {
            spawn_runtime_enemy_entity(
                &mut ecs,
                &restored_config,
                &world.generated,
                &content.0,
                &load_render.presentation.0,
                load_render.asset_server.as_deref(),
                &load_render.asset_root.0,
                &load_render.render,
                &saved.id,
                saved.archetype.clone(),
                position,
            );
            continue;
        }
        let world_position = grid_to_world_on_surface(position, &restored_config, &world.generated);
        let base_scale = Vec3::new(
            restored_config.world.cell_size * 0.3,
            restored_config.world.cell_size * 0.55,
            restored_config.world.cell_size * 0.3,
        );
        ecs.spawn((
            WorldEntity,
            GridLocation(position),
            Agent {
                id: saved.id.clone(),
                kind: saved.kind.clone(),
                archetype: saved.archetype.clone(),
                goal: AgentGoal::Wander,
                spawn: position,
                origin: position,
                navigation_position: placement_to_navigation_centre(position),
                path: Vec::new(),
                path_index: 0,
                target: deterministic_wander_target(&world.generated, &saved.id, position),
                action_cooldown_seconds: 0.0,
                action_started: false,
                repath_remaining_seconds: 0.0,
                health_regen_accumulator: 0.0,
                wander_sequence: 0,
                previous_wander_origin: None,
            },
            AgentLocomotion::default(),
            AgentAnimation {
                base_scale,
                ..default()
            },
            Mesh3d(load_render.render.actor_lod.clone()),
            MeshMaterial3d(actor_material(&load_render.render, &saved.kind, false)),
            Transform::from_xyz(
                world_position.x,
                world_position.y + base_scale.y * 0.5,
                world_position.z,
            )
            .with_scale(base_scale),
        ));
    }
    stats.elapsed_seconds = Duration::from_secs(snapshot.elapsed_seconds).as_secs_f64();
    stats.paths_completed = 0;
    traversal_wear.restore(
        &snapshot.traversal_wear,
        world.generated.navigation.width(),
        world.generated.navigation.height(),
        &config.0.terrain,
    );
    let forked_from_template = source_path != save.store.path();
    if forked_from_template || persisted_upgrade {
        match save.store.write(&snapshot) {
            Ok(()) => {
                if forked_from_template {
                    info!(
                        source = %source_path.display(),
                        destination = %save.store.path().display(),
                        "jump-start template forked to writable town save"
                    );
                } else {
                    info!(
                        destination = %save.store.path().display(),
                        "persisted native save schema and content migrations"
                    );
                }
            }
            Err(error) => error!(
                %error,
                destination = %save.store.path().display(),
                "loaded native save but could not persist its migrated town state"
            ),
        }
    }
    simulation.0 = snapshot.simulation;
    config.0.world.seed = snapshot.world_seed;
    if automatic_resume.path.is_some() {
        automatic_resume.applied = true;
    }
    runtime_console.last_result = if forked_from_template {
        format!(
            "Loaded protected {} as {}",
            source_path.display(),
            save.store.path().display()
        )
    } else {
        format!("Loaded {}", source_path.display())
    };
    info!(
        path = %source_path.display(),
        retained_terrain = snapshot.legacy_terrain_mesh.is_some(),
        terrain_vertices = snapshot
            .legacy_terrain_mesh
            .as_ref()
            .map_or(0, |mesh| mesh.vertices.len()),
        "native save loaded and applied"
    );
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NativeWorldCompatibility {
    Current,
    UpgradeV1,
    UpgradeV2,
    UpgradeV3,
    RegeneratePrior,
}

pub(crate) fn native_world_compatibility(
    seed: u64,
    generator_version: u32,
    world_hash: &str,
    world: &GeneratedWorld,
) -> Option<NativeWorldCompatibility> {
    if seed != world.seed {
        return None;
    }
    if generator_version == world.generator_version && world_hash == world.deterministic_hash {
        return Some(NativeWorldCompatibility::Current);
    }
    if generator_version == 1 && world_hash == stream_town_domain::legacy_v1_world_hash(world) {
        return Some(NativeWorldCompatibility::UpgradeV1);
    }
    if generator_version == 2 && world_hash == stream_town_domain::legacy_v2_world_hash(world) {
        return Some(NativeWorldCompatibility::UpgradeV2);
    }
    if generator_version == 3 && world_hash == stream_town_domain::legacy_v3_world_hash(world) {
        return Some(NativeWorldCompatibility::UpgradeV3);
    }
    // NativeSaveStore has already verified the envelope checksum and snapshot
    // schema before this point. Versions 4+ changed terrain/resource topology
    // too substantially to reproduce their old fingerprints from the current
    // generator, so regenerate the seeded world and use the existing stable-ID
    // restoration/relocation path instead of rejecting a valid native save.
    (generator_version >= 4 && generator_version < world.generator_version)
        .then_some(NativeWorldCompatibility::RegeneratePrior)
}

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

pub(crate) fn actor_idle_anchor(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    config: &GameConfig,
    regeneration: &RegenerationRoleRuntime,
    actor_id: &StableId,
    current: GridPos,
) -> GridPos {
    let Some(actor) = simulation.actors.get(actor_id) else {
        return current;
    };
    if matches!(actor.role.as_str(), "role:forester" | "role:tender")
        && let Some(station_id) = regeneration
            .workers
            .get(actor_id)
            .and_then(|worker| worker.last_station.as_ref())
        && let Some(station) = simulation.buildings.get(station_id)
        && station.complete
        && building_def_for_archetype(content, &station.archetype).is_some_and(|definition| {
            definition
                .role_slots
                .iter()
                .any(|slot| slot.role == actor.role)
        })
    {
        return station.position;
    }
    if let Some(station) = assigned_station(content, simulation, config, actor) {
        return station.position;
    }
    let Some(station_id) = best_station_id(content, simulation, config, &actor.role, current)
    else {
        return current;
    };
    station_candidate(content, simulation, config, &station_id)
        .map_or(current, |station| station.position)
}

pub(crate) fn deterministic_wander_target_step(
    world: &GeneratedWorld,
    actor: &StableId,
    anchor: GridPos,
    current: GridPos,
    sequence: u64,
    avoid: Option<GridPos>,
    mut target_is_usable: impl FnMut(GridPos) -> bool,
) -> GridPos {
    // Unity samples Random.insideUnitCircle around the actor's station whenever
    // its three-second idle timer completes. Stable sequence state supplies a
    // fresh deterministic sample without deriving the next destination only
    // from the current cell (which created A/B pendulum routes).
    let mut actor_hash = 0xcbf2_9ce4_8422_2325_u64 ^ world.seed;
    for byte in actor.as_str().bytes() {
        actor_hash = (actor_hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3);
    }
    for attempt in 0_u64..96 {
        let serial = sequence.wrapping_mul(96).wrapping_add(attempt);
        let mut mixed = actor_hash.wrapping_add(serial.wrapping_mul(0x9e37_79b9_7f4a_7c15));
        mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        mixed ^= mixed >> 31;
        let offset_x = i32::try_from(mixed % 11).expect("bounded idle offset") - 5;
        let offset_z = i32::try_from((mixed >> 16) % 11).expect("bounded idle offset") - 5;
        let distance_squared = offset_x * offset_x + offset_z * offset_z;
        if !(4..=25).contains(&distance_squared) {
            continue;
        }
        let candidate_x = i32::from(anchor.x) + offset_x;
        let candidate_z = i32::from(anchor.z) + offset_z;
        if candidate_x < 0
            || candidate_z < 0
            || candidate_x >= i32::from(world.navigation.width())
            || candidate_z >= i32::from(world.navigation.height())
        {
            continue;
        }
        let candidate = GridPos {
            x: u16::try_from(candidate_x).expect("checked idle x"),
            z: u16::try_from(candidate_z).expect("checked idle z"),
        };
        if candidate == current || Some(candidate) == avoid {
            continue;
        }
        if world.navigation.is_walkable(candidate) && target_is_usable(candidate) {
            return candidate;
        }
    }
    let fallback = deterministic_wander_target(world, actor, current);
    if fallback == current || Some(fallback) == avoid || !target_is_usable(fallback) {
        current
    } else {
        fallback
    }
}

pub(crate) fn deterministic_wander_target(
    world: &GeneratedWorld,
    actor: &StableId,
    position: GridPos,
) -> GridPos {
    // Unity's idle state samples a fresh point from a ten-world-unit circle.
    // Mirroring the whole map produced a deterministic two-point pendulum.
    // These integer offsets cover the same five-cell radius while the stable
    // location hash makes each reached cell lead to a new local destination.
    const OFFSETS: [(i16, i16); 32] = [
        (5, 0),
        (5, 2),
        (4, 3),
        (3, 4),
        (2, 5),
        (0, 5),
        (-2, 5),
        (-3, 4),
        (-4, 3),
        (-5, 2),
        (-5, 0),
        (-5, -2),
        (-4, -3),
        (-3, -4),
        (-2, -5),
        (0, -5),
        (2, -5),
        (3, -4),
        (4, -3),
        (5, -2),
        (3, 0),
        (3, 2),
        (2, 3),
        (0, 3),
        (-2, 3),
        (-3, 2),
        (-3, 0),
        (-3, -2),
        (-2, -3),
        (0, -3),
        (2, -3),
        (3, -2),
    ];
    let mut hash = 0xcbf2_9ce4_8422_2325_u64 ^ world.seed;
    for byte in actor.as_str().bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash ^= u64::from(position.x) << 16 | u64::from(position.z);
    hash = hash.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    let offset_count = u64::try_from(OFFSETS.len()).expect("wander offset count fits u64");
    let start = usize::try_from(hash % offset_count).expect("bounded wander index");
    for attempt in 0..OFFSETS.len() {
        // Seven is coprime with 32, so every direction is visited exactly once.
        let (offset_x, offset_z) = OFFSETS[(start + attempt * 7) % OFFSETS.len()];
        let candidate_x = i32::from(position.x) + i32::from(offset_x);
        let candidate_z = i32::from(position.z) + i32::from(offset_z);
        if candidate_x < 0
            || candidate_z < 0
            || candidate_x >= i32::from(world.navigation.width())
            || candidate_z >= i32::from(world.navigation.height())
        {
            continue;
        }
        let candidate = GridPos {
            x: u16::try_from(candidate_x).expect("checked wander x"),
            z: u16::try_from(candidate_z).expect("checked wander z"),
        };
        if world.navigation.is_walkable(candidate)
            && world.navigation.find_path(position, candidate).is_ok()
        {
            return candidate;
        }
    }
    position
}

pub(crate) fn prefixed_id(requested: &StableId, prefix: &str) -> Option<StableId> {
    if requested.as_str().starts_with(prefix) {
        Some(requested.clone())
    } else {
        StableId::new(format!("{prefix}{}", requested.as_str())).ok()
    }
}

pub(crate) fn normalized_content_name(value: &str) -> String {
    value
        .chars()
        .filter_map(|character| {
            if character.is_ascii_alphanumeric() {
                Some(character.to_ascii_lowercase())
            } else if character == ' ' || character == '-' || character == '_' {
                Some('_')
            } else {
                None
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_owned()
}

pub(crate) fn resolve_technology_id(
    content: &ContentCatalog,
    requested: &StableId,
) -> Option<StableId> {
    if content.technology.nodes.contains_key(requested) {
        return Some(requested.clone());
    }
    let requested_name = normalized_content_name(requested.as_str());
    content
        .technology
        .nodes
        .iter()
        .find(|(_, technology)| normalized_content_name(&technology.display_name) == requested_name)
        .map(|(id, _)| id.clone())
}

pub(crate) fn resolve_active_technology_vote_option(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    requested: &StableId,
) -> Option<StableId> {
    let vote = simulation.active_vote.as_ref()?;
    let options = technology_vote_options(vote);
    if let Ok(index) = requested.as_str().parse::<usize>() {
        return index
            .checked_sub(1)
            .and_then(|index| options.get(index))
            .copied()
            .cloned();
    }
    let technology = resolve_technology_id(content, requested)?;
    options.contains(&&technology).then_some(technology)
}

pub(crate) fn eligible_technology_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> Vec<StableId> {
    let town_hall = StableId::new("building:townhall").expect("static building ID");
    let global_age = unlocked_building_age(content, simulation, &town_hall);
    let town_hall_level = completed_building_level(content, simulation, &town_hall);
    content
        .technology
        .nodes
        .iter()
        .filter(|(id, node)| {
            let technology_age = authored_technology_age(&node.age);
            let is_global_age_up = node.aged_buildings.contains(&town_hall);
            let age_is_available = if is_global_age_up {
                technology_age == global_age.saturating_add(1)
                    && town_hall_level >= u16::from(technology_age)
            } else {
                technology_age <= global_age
            };
            !node.unavailable
                && age_is_available
                && !simulation.unlocked_technology.contains(*id)
                && !simulation
                    .active_goals
                    .iter()
                    .any(|goal| goal.technology == **id)
                && simulation
                    .active_vote
                    .as_ref()
                    .is_none_or(|vote| !technology_vote_options(vote).contains(id))
                && node
                    .prerequisites
                    .iter()
                    .all(|required| simulation.unlocked_technology.contains(required))
        })
        .map(|(id, _)| id.clone())
        .collect()
}

pub(crate) fn authored_technology_age(value: &str) -> u8 {
    value
        .split_whitespace()
        .last()
        .and_then(|value| value.parse::<u8>().ok())
        .unwrap_or(1)
        .max(1)
}

pub(crate) fn completed_building_level(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
) -> u16 {
    let Some(archetype) = content
        .buildings
        .get(building)
        .map(|definition| &definition.archetype)
    else {
        return 0;
    };
    simulation
        .buildings
        .values()
        .filter(|state| state.complete && &state.archetype == archetype)
        .map(|state| state.level)
        .max()
        .unwrap_or(0)
}

pub(crate) fn technology_ballot_rank(
    world_seed: u64,
    unlocked_count: usize,
    salt: u64,
    id: &StableId,
) -> u64 {
    let mut hash = world_seed
        ^ salt
        ^ u64::try_from(unlocked_count)
            .unwrap_or(u64::MAX)
            .wrapping_mul(0x9E37_79B9_7F4A_7C15);
    for byte in id.as_str().bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash
}

pub(crate) fn technology_node_depth(
    content: &ContentCatalog,
    id: &StableId,
    memo: &mut BTreeMap<StableId, u32>,
    visiting: &mut BTreeSet<StableId>,
) -> u32 {
    if let Some(depth) = memo.get(id) {
        return *depth;
    }
    if !visiting.insert(id.clone()) {
        return 0;
    }
    let depth = content.technology.nodes.get(id).map_or(0, |node| {
        node.prerequisites
            .iter()
            .filter(|required| content.technology.nodes.contains_key(*required))
            .map(|required| technology_node_depth(content, required, memo, visiting))
            .max()
            .map_or(0, |depth| depth.saturating_add(1))
    });
    visiting.remove(id);
    memo.insert(id.clone(), depth);
    depth
}

pub(crate) fn technology_depths(content: &ContentCatalog) -> BTreeMap<StableId, u32> {
    let mut memo = BTreeMap::new();
    for id in content.technology.nodes.keys() {
        technology_node_depth(content, id, &mut memo, &mut BTreeSet::new());
    }
    memo
}

pub(crate) fn take_ranked_technology(
    eligible: &mut Vec<StableId>,
    world_seed: u64,
    unlocked_count: usize,
    salt: u64,
    required_depth: Option<u32>,
    depths: &BTreeMap<StableId, u32>,
) -> Option<StableId> {
    let index = eligible
        .iter()
        .enumerate()
        .filter(|(_, id)| required_depth.is_none_or(|depth| depths.get(*id) == Some(&depth)))
        .min_by_key(|(_, id)| {
            (
                technology_ballot_rank(world_seed, unlocked_count, salt, id),
                (*id).clone(),
            )
        })
        .map(|(index, _)| index)?;
    Some(eligible.remove(index))
}

pub(crate) fn technology_ballot_options(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> Vec<StableId> {
    let mut eligible = eligible_technology_ids(content, simulation);
    let depths = technology_depths(content);
    let deepest = eligible
        .iter()
        .filter_map(|id| depths.get(id))
        .max()
        .copied();
    let shallowest = eligible
        .iter()
        .filter_map(|id| depths.get(id))
        .min()
        .copied();
    let mut options = Vec::with_capacity(TECHNOLOGY_VOTE_OPTION_COUNT);
    if let Some(technology) = take_ranked_technology(
        &mut eligible,
        simulation.world_seed,
        simulation.unlocked_technology.len(),
        0x0073_7065_6369_616c,
        deepest,
        &depths,
    ) {
        options.push(technology);
    }
    if let Some(technology) = take_ranked_technology(
        &mut eligible,
        simulation.world_seed,
        simulation.unlocked_technology.len(),
        0x6675_6e64_616d_656e,
        shallowest,
        &depths,
    ) {
        options.push(technology);
    }
    if let Some(technology) = take_ranked_technology(
        &mut eligible,
        simulation.world_seed,
        simulation.unlocked_technology.len(),
        0x7261_6e64_6f6d,
        None,
        &depths,
    ) {
        options.push(technology);
    }
    options
}

pub(crate) fn technology_vote_depth_tag(option_count: usize, index: u8) -> &'static str {
    match (option_count, index) {
        (1, 0) => "Both",
        (_, 0) => "Specialized",
        (_, 1) => "Fundamental",
        _ => "",
    }
}

pub(crate) fn start_scheduled_technology_vote(
    mut simulation: ResMut<SimulationRuntime>,
    content: Res<RuntimeContent>,
) {
    if simulation.0.active_vote.is_some()
        || !simulation.0.active_goals.is_empty()
        || simulation.0.active_event.is_some()
    {
        return;
    }
    if !simulation
        .0
        .technology_vote_cooldown_seconds
        .is_some_and(|remaining| remaining <= f32::EPSILON)
    {
        return;
    }
    let options = technology_ballot_options(&content.0, &simulation.0);
    simulation.0.technology_vote_cooldown_seconds = None;
    if options.is_empty() {
        return;
    }
    if let Err(error) = simulation
        .0
        .start_technology_ballot(options, TECHNOLOGY_VOTE_DURATION_SECONDS)
    {
        warn!(%error, "scheduled technology vote could not start");
    }
}

pub(crate) fn technology_vote_announcement(
    content: &ContentCatalog,
    vote: &stream_town_domain::TechVote,
) -> String {
    let options = technology_vote_options(vote);
    let labels = options
        .iter()
        .enumerate()
        .map(|(index, technology)| {
            let label = content.technology.nodes.get(*technology).map_or_else(
                || technology.to_string(),
                |node| compact_technology_label(&node.display_name).replace('\n', " "),
            );
            format!("{}: {label}", index + 1)
        })
        .collect::<Vec<_>>()
        .join("; ");
    let commands = (1..=options.len())
        .map(|index| format!("!vote {index}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("Technology vote started! {labels}. Vote with {commands}.")
}

pub(crate) fn announce_technology_vote(
    simulation: Res<SimulationRuntime>,
    content: Res<RuntimeContent>,
    connection: Res<TwitchConnection>,
    mut announced_vote: Local<Option<String>>,
) {
    let Some(vote) = simulation.0.active_vote.as_ref() else {
        *announced_vote = None;
        return;
    };
    let signature = technology_vote_options(vote)
        .iter()
        .map(|technology| technology.as_str())
        .collect::<Vec<_>>()
        .join("|");
    if announced_vote.as_deref() == Some(signature.as_str()) || connection.transport.is_none() {
        return;
    }
    let message = technology_vote_announcement(&content.0, vote);
    info!(announcement = %message, "technology vote announced");
    send_command_feedback(&connection, message);
    *announced_vote = Some(signature);
}

pub(crate) fn unlock_reachable_technologies(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    age_one_only: bool,
) -> usize {
    let mut unlocked = 0;
    loop {
        let available = eligible_technology_ids(content, simulation)
            .into_iter()
            .filter(|id| {
                !age_one_only
                    || content.technology.nodes[id]
                        .age
                        .trim()
                        .eq_ignore_ascii_case("Age 1")
            })
            .collect::<Vec<_>>();
        if available.is_empty() {
            break;
        }
        unlocked += available.len();
        simulation.unlocked_technology.extend(available);
    }
    unlocked
}

pub(crate) fn maximum_building_level(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
) -> u16 {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .filter_map(|technology| technology.building_level_caps.get(building).copied())
        .max()
        .unwrap_or(1)
}

pub(crate) fn building_is_unlocked(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
) -> bool {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .any(|technology| technology.unlocked_buildings.contains(building))
}

pub(crate) fn technology_stat_boost_percent(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    role: &StableId,
    stat: &StableId,
) -> i32 {
    if role.as_str() == "role:enemy" {
        return 0;
    }
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .fold(0_i32, |total, technology| {
            // Unity records PlayerRole.Count boosts in GlobalStatModifiers, but
            // PlayerRoleData only consumes role modifiers and per-character
            // passives. No shipping caller copies the global technology table
            // into those passives, so activating it here changes live balance.
            total.saturating_add(
                technology
                    .role_stat_boost_percent
                    .get(role)
                    .and_then(|stats| stats.get(stat))
                    .copied()
                    .unwrap_or_default(),
            )
        })
}

pub(crate) fn building_cost_reduction_percent(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
) -> i32 {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .fold(0_i32, |total, technology| {
            total
                .saturating_add(technology.global_building_cost_reduction_percent)
                .saturating_add(
                    technology
                        .building_cost_reduction_percent
                        .get(building)
                        .copied()
                        .unwrap_or_default(),
                )
        })
}

pub(crate) fn storage_boost_percent(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    resource: &StableId,
) -> i32 {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .filter_map(|technology| technology.storage_boost_percent.get(resource).copied())
        .fold(0_i32, i32::saturating_add)
}

pub(crate) fn unlocked_building_age(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
) -> u8 {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .filter(|technology| technology.aged_buildings.contains(building))
        .map(|technology| authored_technology_age(&technology.age))
        .max()
        .unwrap_or(1)
}

pub(crate) fn building_age(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
    level: u16,
) -> u8 {
    let town_hall = StableId::new("building:townhall").expect("static building ID");
    let local_age = simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .filter(|technology| technology.aged_buildings.contains(building))
        .filter(|technology| {
            technology
                .building_level_caps
                .get(building)
                .is_none_or(|minimum_level| level >= *minimum_level)
        })
        .map(|technology| authored_technology_age(&technology.age))
        .max()
        .unwrap_or(1);
    let global_age = unlocked_building_age(content, simulation, &town_hall);
    local_age.min(global_age)
}

pub(crate) fn percentage_adjusted(base: u32, percent: i32) -> u32 {
    let base = i64::from(base);
    let adjusted = base.saturating_add(base.saturating_mul(i64::from(percent)) / 100);
    u32::try_from(adjusted.clamp(0, i64::from(u32::MAX))).unwrap_or(u32::MAX)
}

pub(crate) fn percentage_reduced(base: u32, reduction_percent: i32) -> u32 {
    percentage_adjusted(base, reduction_percent.saturating_neg())
}

pub(crate) fn percentage_adjusted_i64(base: i64, percent: i32) -> i64 {
    base.saturating_add(base.saturating_mul(i64::from(percent)) / 100)
}

pub(crate) fn milli_units_as_f32(value: u32) -> f32 {
    f32::from(u16::try_from(value.min(u32::from(u16::MAX))).expect("clamped milli-unit fits u16"))
        / 1_000.0
}

pub(crate) fn resource_storage_capacity(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    resource: &StableId,
) -> u32 {
    let Some(base) = config
        .gameplay
        .base_town_resource_capacity
        .get(resource)
        .copied()
    else {
        return u32::MAX;
    };
    let boost = storage_boost_percent(content, simulation, resource);
    simulation
        .buildings
        .values()
        .filter(|building| building.complete)
        .filter_map(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            Some((definition, building.level))
        })
        .flat_map(|(definition, level)| {
            definition
                .storage
                .iter()
                .filter(move |storage| storage.resource == *resource)
                .map(move |storage| {
                    let amount = storage.base_amount.saturating_add(
                        storage
                            .increment_amount
                            .saturating_mul(u32::from(level.saturating_sub(1))),
                    );
                    percentage_adjusted(amount, boost)
                })
        })
        .fold(base, u32::saturating_add)
}

pub(crate) fn take_passive_building_income_delta(
    accumulated: &mut Duration,
    delta: Duration,
) -> Option<Duration> {
    *accumulated = accumulated.saturating_add(delta);
    if *accumulated < PASSIVE_BUILDING_INCOME_INTERVAL {
        return None;
    }
    Some(std::mem::take(accumulated))
}

pub(crate) fn apply_passive_building_income(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    delta: Duration,
) {
    let mut rates = BTreeMap::<(StableId, StableId), u64>::new();
    for building in simulation
        .buildings
        .values()
        .filter(|building| building.complete)
    {
        let Some(definition) = building_def_for_archetype(content, &building.archetype) else {
            continue;
        };
        for (resource, rate) in passive_resource_rate_milli_per_second(definition, building.level) {
            let entry = rates.entry((building.id.clone(), resource)).or_default();
            *entry = entry.saturating_add(rate);
        }
    }

    let mut active = BTreeMap::<StableId, BTreeSet<StableId>>::new();
    for (building, resource) in rates.keys() {
        active
            .entry(building.clone())
            .or_default()
            .insert(resource.clone());
    }
    simulation
        .passive_resource_accumulators
        .retain(|building, resources| {
            let Some(active_resources) = active.get(building) else {
                return false;
            };
            resources.retain(|resource, _| active_resources.contains(resource));
            !resources.is_empty()
        });

    let delta_nanos = delta.as_nanos();
    if delta_nanos == 0 {
        return;
    }
    for ((building, resource), rate) in rates {
        let previous = simulation
            .passive_resource_accumulators
            .entry(building)
            .or_default()
            .entry(resource.clone())
            .or_default();
        let accumulated =
            u128::from(*previous).saturating_add(u128::from(rate).saturating_mul(delta_nanos));
        // Unity uses a strict `> 1` threshold, so exactly one accumulated unit waits
        // until the next positive update rather than being emitted immediately.
        let generated = if accumulated > PASSIVE_RESOURCE_FIXED_POINT_DENOMINATOR {
            accumulated / PASSIVE_RESOURCE_FIXED_POINT_DENOMINATOR
        } else {
            0
        };
        *previous = u64::try_from(
            accumulated
                .saturating_sub(generated.saturating_mul(PASSIVE_RESOURCE_FIXED_POINT_DENOMINATOR)),
        )
        .expect("passive resource remainder fits u64");
        if generated == 0 {
            continue;
        }
        let generated = u32::try_from(generated).unwrap_or(u32::MAX);
        let capacity = resource_storage_capacity(config, content, simulation, &resource);
        let current = simulation
            .town_resources
            .get(&resource)
            .copied()
            .unwrap_or_default();
        simulation.town_resources.insert(
            resource.clone(),
            current.saturating_add(generated).min(capacity),
        );
        let _ = simulation.record_objective_event(
            &content.objectives,
            &ObjectiveEvent::ResourceGained {
                resource,
                amount: generated,
            },
        );
    }
}

pub(crate) fn building_construction_cost(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building_id: &StableId,
    building: &BuildingDef,
) -> BTreeMap<StableId, u32> {
    let reduction = building_cost_reduction_percent(content, simulation, building_id);
    let existing_count = simulation
        .buildings
        .values()
        .filter(|existing| existing.archetype == building.archetype)
        .count();
    let existing_count = u64::try_from(existing_count).unwrap_or(u64::MAX);
    let multiplier_per_thousand = 1_000_u64.saturating_add(existing_count.saturating_mul(
        u64::from(building.construction_cost_multiplier_per_thousand),
    ));
    building
        .cost
        .iter()
        .map(|(resource, cost)| {
            let scaled = u64::from(*cost)
                .saturating_mul(multiplier_per_thousand)
                .saturating_add(999)
                / 1_000;
            (
                resource.clone(),
                percentage_reduced(u32::try_from(scaled).unwrap_or(u32::MAX), reduction),
            )
        })
        .collect()
}

pub(crate) fn building_upgrade_cost(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building_id: &StableId,
    building: &BuildingDef,
    current_level: u16,
    target_level: u16,
) -> BTreeMap<StableId, u32> {
    let target_level = target_level.max(current_level);
    let reduction = building_cost_reduction_percent(content, simulation, building_id);
    building
        .cost
        .iter()
        .map(|(resource, base)| {
            let scaled = (u64::from(current_level)..u64::from(target_level)).fold(
                0_u64,
                |total, target_minus_one| {
                    let rung = u64::from(*base)
                        .saturating_mul(target_minus_one)
                        .saturating_mul(u64::from(
                            building.upgrade_cost_per_target_level_per_thousand,
                        ))
                        .saturating_add(999)
                        / 1_000;
                    total.saturating_add(rung)
                },
            );
            (
                resource.clone(),
                percentage_reduced(u32::try_from(scaled).unwrap_or(u32::MAX), reduction),
            )
        })
        .collect()
}

pub(crate) fn town_event_from_id(requested: &StableId) -> Option<TownEvent> {
    match requested.as_str().trim_start_matches("event:") {
        "festival" => Some(TownEvent::Festival),
        "raid" | "enemy_raid" => Some(TownEvent::EnemyRaid),
        "harsh_weather" | "weather" => Some(TownEvent::HarshWeather),
        "resource_boom" | "wood_boom" => Some(TownEvent::ResourceBoom(
            StableId::new("resource:wood").expect("static ID"),
        )),
        "fish_god" | "fishgod" => Some(TownEvent::FishGod),
        _ => None,
    }
}

pub(crate) fn community_event_from_id(requested: &StableId) -> Option<CommunityEvent> {
    match requested.as_str().trim_start_matches("event:") {
        "prospecting" | "prospecting_boom" | "prospectingboom" | "prospector" => {
            Some(CommunityEvent::ProspectingBoom)
        }
        "reforestation" | "reforestation_boom" | "reforestationboom" | "forester" => {
            Some(CommunityEvent::ReforestationBoom)
        }
        "agricultural" | "agricultural_boom" | "agriculturalboom" | "agriculture" | "tender" => {
            Some(CommunityEvent::AgriculturalBoom)
        }
        "rebalance" | "balanced" => Some(CommunityEvent::Rebalance),
        "awakening" | "xp" => Some(CommunityEvent::Awakening),
        "economic" | "economic_boom" | "economicboom" | "economy" => {
            Some(CommunityEvent::EconomicBoom)
        }
        "invasion" => Some(CommunityEvent::Invasion),
        "market" | "market_event" => Some(CommunityEvent::Market),
        _ => None,
    }
}

pub(crate) const fn community_event_name(event: CommunityEvent) -> &'static str {
    match event {
        CommunityEvent::ProspectingBoom => "Prospecting Boom",
        CommunityEvent::ReforestationBoom => "Reforestation Boom",
        CommunityEvent::AgriculturalBoom => "Agricultural Boom",
        CommunityEvent::Rebalance => "Rebalance",
        CommunityEvent::Awakening => "Awakening",
        CommunityEvent::EconomicBoom => "Economic Boom",
        CommunityEvent::Invasion => "Invasion",
        CommunityEvent::Market => "Market",
    }
}

pub(crate) const fn community_event_description(event: CommunityEvent) -> &'static str {
    match event {
        CommunityEvent::ProspectingBoom => {
            "Prospectors work at 3x speed; Foresters and Tenders at 0.5x."
        }
        CommunityEvent::ReforestationBoom => {
            "Foresters work at 3x speed; Prospectors and Tenders at 0.5x."
        }
        CommunityEvent::AgriculturalBoom => {
            "Tenders work at 3x speed; Prospectors and Foresters at 0.5x."
        }
        CommunityEvent::Rebalance => {
            "Prospectors, Foresters, and Tenders work at their normal rates."
        }
        CommunityEvent::Awakening => "All experience gains are increased by 20%.",
        CommunityEvent::EconomicBoom => "All gathering actions are 10% faster.",
        CommunityEvent::Invasion => {
            "Monster waves are 50% larger and monster gold rewards are doubled."
        }
        CommunityEvent::Market => {
            "Overflow gathered resources are sold automatically for 25% of normal trade value."
        }
    }
}

pub(crate) fn community_event_role_rate_multiplier(event: CommunityEvent, role: &StableId) -> f32 {
    let role = role.as_str();
    match event {
        CommunityEvent::ProspectingBoom if role == "role:prospector" => 3.0,
        CommunityEvent::ProspectingBoom if matches!(role, "role:forester" | "role:tender") => 0.5,
        CommunityEvent::ReforestationBoom if role == "role:forester" => 3.0,
        CommunityEvent::ReforestationBoom if matches!(role, "role:prospector" | "role:tender") => {
            0.5
        }
        CommunityEvent::AgriculturalBoom if role == "role:tender" => 3.0,
        CommunityEvent::AgriculturalBoom if matches!(role, "role:prospector" | "role:forester") => {
            0.5
        }
        _ => 1.0,
    }
}

pub(crate) fn event_adjusted_wave_size(base: u16, event: Option<CommunityEvent>) -> u16 {
    if event == Some(CommunityEvent::Invasion) {
        base.saturating_mul(3).saturating_add(1) / 2
    } else {
        base
    }
}

pub(crate) fn building_region(
    position: GridPos,
    footprint: [u16; 2],
    world: &GeneratedWorld,
) -> Option<stream_town_domain::DirtyRegion> {
    let max_x = position.x.checked_add(footprint[0].checked_sub(1)?)?;
    let max_z = position.z.checked_add(footprint[1].checked_sub(1)?)?;
    if max_x >= world.navigation.width() || max_z >= world.navigation.height() {
        return None;
    }
    Some(stream_town_domain::DirtyRegion {
        min: position,
        max: GridPos { x: max_x, z: max_z },
    })
}

pub(crate) fn rotated_footprint(footprint: [u16; 2], rotation_quarter_turns: i32) -> [u16; 2] {
    if rotation_quarter_turns.rem_euclid(2) == 0 {
        footprint
    } else {
        [footprint[1], footprint[0]]
    }
}

pub(crate) fn building_navigation_region(
    position: GridPos,
    definition: &BuildingDef,
    rotation_quarter_turns: i32,
    world: &GeneratedWorld,
) -> Option<stream_town_domain::DirtyRegion> {
    // The coarse grid remains the authoritative placement and exclusion map.
    // Physical movement uses `FineNavigationRuntime`, where the independently
    // authored footprint can be inset without weakening placement validation.
    building_region(
        position,
        rotated_footprint(definition.footprint, rotation_quarter_turns),
        world,
    )
}

pub(crate) fn placement_to_navigation_centre(position: GridPos) -> GridPos {
    GridPos {
        x: position
            .x
            .saturating_mul(NAVIGATION_SUBDIVISIONS)
            .saturating_add(NAVIGATION_SUBDIVISIONS / 2),
        z: position
            .z
            .saturating_mul(NAVIGATION_SUBDIVISIONS)
            .saturating_add(NAVIGATION_SUBDIVISIONS / 2),
    }
}

pub(crate) fn navigation_to_placement(position: GridPos) -> GridPos {
    GridPos {
        x: position.x / NAVIGATION_SUBDIVISIONS,
        z: position.z / NAVIGATION_SUBDIVISIONS,
    }
}

pub(crate) fn navigation_to_world_on_surface(
    position: GridPos,
    config: &GameConfig,
    world: &GeneratedWorld,
) -> Vec3 {
    let half = f32::from(NAVIGATION_SUBDIVISIONS / 2);
    let subdivision = f32::from(NAVIGATION_SUBDIVISIONS);
    let logical_x = (f32::from(position.x) - half) / subdivision;
    let logical_z = (f32::from(position.z) - half) / subdivision;
    let mut result = Vec3::new(
        (logical_x - f32::from(config.world.width.saturating_sub(1)) * 0.5)
            * config.world.cell_size,
        0.0,
        (logical_z - f32::from(config.world.height.saturating_sub(1)) * 0.5)
            * config.world.cell_size,
    );
    result.y = terrain_surface_height_at_world(world, config, result.x, result.z)
        .unwrap_or_else(|| terrain_height(world, navigation_to_placement(position)));
    result
}

pub(crate) fn default_navigation_footprint_thirds(placement: [u16; 2]) -> [u16; 2] {
    placement.map(|axis| {
        axis.saturating_mul(NAVIGATION_SUBDIVISIONS)
            .saturating_sub(2)
            .max(1)
    })
}

pub(crate) fn linear_neighbour_value(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    state: &BuildingState,
    building_id: &StableId,
) -> u8 {
    if is_path_building(building_id) {
        let Some(path_archetype) = content
            .buildings
            .get(building_id)
            .map(|definition| &definition.archetype)
        else {
            return 0;
        };
        let centre = simulation
            .path_navigation_positions
            .get(&state.id)
            .copied()
            .unwrap_or_else(|| placement_to_navigation_centre(state.position));
        return simulation
            .buildings
            .values()
            .filter(|other| other.id != state.id && other.archetype == *path_archetype)
            .fold(0_u8, |value, other| {
                let other = simulation
                    .path_navigation_positions
                    .get(&other.id)
                    .copied()
                    .unwrap_or_else(|| placement_to_navigation_centre(other.position));
                let delta_x = i32::from(other.x) - i32::from(centre.x);
                let delta_z = i32::from(other.z) - i32::from(centre.z);
                let connection = match (delta_x, delta_z) {
                    (1..=3, 0) => 8,
                    (-3..=-1, 0) => 2,
                    (0, 1..=3) => 16,
                    (0, -3..=-1) => 4,
                    _ => 0,
                };
                value | connection
            });
    }
    let connects = |other: &BuildingState| {
        content.buildings.iter().any(|(id, definition)| {
            definition.archetype == other.archetype
                && if building_id.as_str() == "building:path" {
                    id.as_str() == "building:path"
                } else {
                    matches!(id.as_str(), "building:wall" | "building:gate")
                }
        })
    };
    simulation
        .buildings
        .values()
        .filter(|other| other.id != state.id && connects(other))
        .fold(0_u8, |value, other| {
            value
                | match (
                    i32::from(other.position.x) - i32::from(state.position.x),
                    i32::from(other.position.z) - i32::from(state.position.z),
                ) {
                    (1, 0) => 8,
                    (-1, 0) => 2,
                    (0, 1) => 16,
                    (0, -1) => 4,
                    _ => 0,
                }
        })
}

pub(crate) fn linear_navigation_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    state: &BuildingState,
    building_id: &StableId,
) -> Vec<GridPos> {
    let path = is_path_building(building_id);
    let centre = if path {
        simulation
            .path_navigation_positions
            .get(&state.id)
            .copied()
            .unwrap_or_else(|| placement_to_navigation_centre(state.position))
    } else {
        placement_to_navigation_centre(state.position)
    };
    let mut connections = linear_neighbour_value(content, simulation, state, building_id);
    if path && connections == 0 {
        return vec![centre];
    }
    let horizontal = connections & (2 | 8) != 0;
    let vertical = connections & (4 | 16) != 0;
    if horizontal ^ vertical {
        // A line endpoint still spans its full placement cell, avoiding a one-third
        // gap between a wall/gate and the next non-linear obstruction.
        connections |= if horizontal { 2 | 8 } else { 4 | 16 };
    } else if !path && connections == 0 {
        // An isolated segment has no neighbours from which to infer its axis,
        // so retain the direction selected by the placement command.
        connections = if state.rotation_quarter_turns.rem_euclid(2) == 0 {
            4 | 16
        } else {
            2 | 8
        };
    }
    let mut cells = vec![centre];
    if connections & 2 != 0 {
        cells.push(GridPos {
            x: centre.x.saturating_sub(1),
            z: centre.z,
        });
    }
    if connections & 8 != 0 {
        cells.push(GridPos {
            x: centre.x.saturating_add(1),
            z: centre.z,
        });
    }
    if connections & 4 != 0 {
        cells.push(GridPos {
            x: centre.x,
            z: centre.z.saturating_sub(1),
        });
    }
    if connections & 16 != 0 {
        cells.push(GridPos {
            x: centre.x,
            z: centre.z.saturating_add(1),
        });
    }
    cells.sort_unstable();
    cells.dedup();
    cells
}

pub(crate) fn rectangular_navigation_cells(
    position: GridPos,
    placement: [u16; 2],
    navigation: [u16; 2],
    rotation_quarter_turns: i32,
) -> Vec<GridPos> {
    let placement = rotated_footprint(placement, rotation_quarter_turns);
    let navigation = rotated_footprint(navigation, rotation_quarter_turns);
    let full = placement.map(|axis| axis.saturating_mul(NAVIGATION_SUBDIVISIONS));
    let offset = [
        full[0].saturating_sub(navigation[0]) / 2,
        full[1].saturating_sub(navigation[1]) / 2,
    ];
    let origin = GridPos {
        x: position
            .x
            .saturating_mul(NAVIGATION_SUBDIVISIONS)
            .saturating_add(offset[0]),
        z: position
            .z
            .saturating_mul(NAVIGATION_SUBDIVISIONS)
            .saturating_add(offset[1]),
    };
    (0..navigation[1])
        .flat_map(|z| {
            (0..navigation[0]).map(move |x| GridPos {
                x: origin.x.saturating_add(x),
                z: origin.z.saturating_add(z),
            })
        })
        .collect()
}

pub(crate) fn rectangular_building_fine_cells(
    position: GridPos,
    legacy_placement: [u16; 2],
    fine_footprint: [u16; 2],
    rotation_quarter_turns: i32,
) -> Vec<GridPos> {
    let legacy_placement = rotated_footprint(legacy_placement, rotation_quarter_turns);
    let fine_footprint = rotated_footprint(fine_footprint, rotation_quarter_turns);
    if fine_footprint[0] == 0 || fine_footprint[1] == 0 {
        return Vec::new();
    }
    let visual = GridPos {
        x: position.x.saturating_add(legacy_placement[0] / 2),
        z: position.z.saturating_add(legacy_placement[1] / 2),
    };
    let centre = placement_to_navigation_centre(visual);
    let origin_x = i32::from(centre.x) - i32::from(fine_footprint[0].saturating_sub(1) / 2);
    let origin_z = i32::from(centre.z) - i32::from(fine_footprint[1].saturating_sub(1) / 2);
    (0..fine_footprint[1])
        .flat_map(|z| {
            (0..fine_footprint[0]).filter_map(move |x| {
                Some(GridPos {
                    x: u16::try_from(origin_x + i32::from(x)).ok()?,
                    z: u16::try_from(origin_z + i32::from(z)).ok()?,
                })
            })
        })
        .collect()
}

pub(crate) fn fine_cells_for_coarse_cell(position: GridPos) -> Vec<GridPos> {
    let origin = GridPos {
        x: position.x.saturating_mul(NAVIGATION_SUBDIVISIONS),
        z: position.z.saturating_mul(NAVIGATION_SUBDIVISIONS),
    };
    (0..NAVIGATION_SUBDIVISIONS)
        .flat_map(|z| {
            (0..NAVIGATION_SUBDIVISIONS).map(move |x| GridPos {
                x: origin.x.saturating_add(x),
                z: origin.z.saturating_add(z),
            })
        })
        .collect()
}

pub(crate) fn fine_cells_for_coarse_region(
    region: stream_town_domain::DirtyRegion,
) -> Vec<GridPos> {
    (region.min.z..=region.max.z)
        .flat_map(|z| {
            (region.min.x..=region.max.x)
                .flat_map(move |x| fine_cells_for_coarse_cell(GridPos { x, z }))
        })
        .collect()
}

pub(crate) fn building_fine_placement_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    state: &BuildingState,
    building_id: &StableId,
    definition: &BuildingDef,
) -> Vec<GridPos> {
    if is_path_building(building_id) {
        return simulation
            .path_navigation_positions
            .get(&state.id)
            .copied()
            .into_iter()
            .collect();
    }
    if matches!(building_id.as_str(), "building:wall" | "building:gate") {
        return linear_navigation_cells(content, simulation, state, building_id);
    }
    rectangular_building_fine_cells(
        state.position,
        definition.footprint,
        definition
            .placement_footprint_thirds
            .unwrap_or_else(|| definition.footprint.map(|axis| axis.saturating_mul(3))),
        state.rotation_quarter_turns,
    )
}

pub(crate) fn building_fine_navigation_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    state: &BuildingState,
    building_id: &StableId,
    definition: &BuildingDef,
) -> Vec<GridPos> {
    if is_path_building(building_id) {
        return Vec::new();
    }
    if matches!(building_id.as_str(), "building:wall" | "building:gate") {
        return linear_navigation_cells(content, simulation, state, building_id);
    }
    let navigation = definition
        .navigation_footprint_thirds
        .unwrap_or_else(|| default_navigation_footprint_thirds(definition.footprint));
    rectangular_building_fine_cells(
        state.position,
        definition.footprint,
        navigation,
        state.rotation_quarter_turns,
    )
}

pub(crate) fn fine_navigation_signature(
    world: &GeneratedWorld,
    simulation: &WorldSimulation,
) -> u64 {
    let mut signature = world.navigation.topology_signature() ^ 0x6a09_e667_f3bc_c909;
    for building in simulation.buildings.values().filter(|building| {
        !simulation
            .path_navigation_positions
            .contains_key(&building.id)
    }) {
        signature = signature
            .wrapping_mul(0x0000_0100_0000_01b3)
            .wrapping_add(stable_id_hash(&building.id))
            .wrapping_add(u64::from(building.position.x) << 16)
            .wrapping_add(u64::from(building.position.z))
            .wrapping_add(
                u64::from(u32::from_ne_bytes(
                    building.rotation_quarter_turns.to_ne_bytes(),
                )) << 32,
            );
    }
    signature
}

pub(crate) fn build_fine_navigation(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
) -> Result<stream_town_domain::NavGrid, stream_town_domain::NavigationError> {
    let width = config
        .world
        .width
        .checked_mul(NAVIGATION_SUBDIVISIONS)
        .ok_or(stream_town_domain::NavigationError::BufferSize)?;
    let height = config
        .world
        .height
        .checked_mul(NAVIGATION_SUBDIVISIONS)
        .ok_or(stream_town_domain::NavigationError::BufferSize)?;
    let mut blocked = Vec::with_capacity(usize::from(width) * usize::from(height));
    let mut heights = Vec::with_capacity(blocked.capacity());
    for z in 0..height {
        for x in 0..width {
            let placement = navigation_to_placement(GridPos { x, z });
            let terrain_height = world.navigation.height_at(placement).unwrap_or_default();
            heights.push(terrain_height);
            blocked.push(terrain_height < NAVIGATION_TERRAIN_MIN_HEIGHT_CENTIMETRES);
        }
    }
    let index = |position: GridPos| {
        (position.x < width && position.z < height)
            .then(|| usize::from(position.z) * usize::from(width) + usize::from(position.x))
    };
    let spawn = GridPos {
        x: config.world.width / 2,
        z: config.world.height / 2,
    };
    for z in 0..NAVIGATION_SUBDIVISIONS {
        for x in 0..NAVIGATION_SUBDIVISIONS {
            let position = GridPos {
                x: spawn
                    .x
                    .saturating_mul(NAVIGATION_SUBDIVISIONS)
                    .saturating_add(x),
                z: spawn
                    .z
                    .saturating_mul(NAVIGATION_SUBDIVISIONS)
                    .saturating_add(z),
            };
            if let Some(index) = index(position) {
                blocked[index] = false;
            }
        }
    }
    for resource in world
        .resources
        .iter()
        .filter(|resource| resource.amount > 0 && resource.target_kind.as_str() != "target:fish")
    {
        if let Some(index) = index(placement_to_navigation_centre(resource.position)) {
            blocked[index] = true;
        }
    }
    for state in simulation.buildings.values() {
        let Some((building_id, definition)) = content
            .buildings
            .iter()
            .find(|(_, definition)| definition.archetype == state.archetype)
        else {
            continue;
        };
        if !building_blocks_navigation(definition) {
            continue;
        }
        for position in
            building_fine_navigation_cells(content, simulation, state, building_id, definition)
        {
            if let Some(index) = index(position) {
                blocked[index] = true;
            }
        }
    }
    for camp in simulation.enemy_camps.values() {
        let Some(archetype) = content.archetypes.get(&camp.archetype) else {
            continue;
        };
        let footprint = [archetype.footprint[0].min(3), archetype.footprint[1].min(3)];
        for position in rectangular_navigation_cells(
            camp.position,
            footprint,
            footprint.map(|axis| axis.saturating_mul(NAVIGATION_SUBDIVISIONS)),
            0,
        ) {
            if let Some(index) = index(position) {
                blocked[index] = true;
            }
        }
    }
    stream_town_domain::NavGrid::new(width, height, blocked, heights)
}

pub(crate) fn sync_fine_navigation(
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    mut runtime: ResMut<FineNavigationRuntime>,
) {
    let signature = fine_navigation_signature(&world.generated, &simulation.0);
    if runtime.grid.is_some() && runtime.applied_signature == signature {
        return;
    }
    match build_fine_navigation(&config.0, &content.0, &simulation.0, &world.generated) {
        Ok(grid) => {
            runtime.grid = Some(grid);
            runtime.applied_signature = signature;
        }
        Err(error) => error!(%error, "failed to build the navigation-only third-cell grid"),
    }
}

pub(crate) fn floorplan_diagnostic_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
) -> BTreeSet<GridPos> {
    let mut cells = world
        .resources
        .iter()
        .filter(|resource| resource.amount > 0)
        .flat_map(|resource| fine_cells_for_coarse_cell(resource.position))
        .collect::<BTreeSet<_>>();
    for building in simulation.buildings.values() {
        let Some((building_id, definition)) = content
            .buildings
            .iter()
            .find(|(_, definition)| definition.archetype == building.archetype)
        else {
            continue;
        };
        cells.extend(building_fine_placement_cells(
            content,
            simulation,
            building,
            building_id,
            definition,
        ));
    }
    for camp in simulation.enemy_camps.values() {
        let Some(archetype) = content.archetypes.get(&camp.archetype) else {
            continue;
        };
        if let Some(region) = building_region(camp.position, archetype.footprint, world) {
            cells.extend(fine_cells_for_coarse_region(region));
        }
    }
    cells
}

pub(crate) fn append_world_diagnostic_quad(
    corners: [Vec3; 4],
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
) {
    let base = u32::try_from(positions.len()).expect("diagnostic overlay vertex count fits u32");
    positions.extend(corners.map(|corner| corner.to_array()));
    normals.extend_from_slice(&[[0.0, 1.0, 0.0]; 4]);
    uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
    indices.extend_from_slice(&[base, base + 3, base + 1, base + 1, base + 3, base + 2]);
}

pub(crate) fn diagnostic_navigation_surface_quad(
    cell: GridPos,
    size: f32,
    config: &GameConfig,
    world: &GeneratedWorld,
    water_height: f32,
) -> [Vec3; 4] {
    let centre = navigation_to_world_on_surface(cell, config, world);
    let half = size * 0.5;
    [
        Vec2::new(centre.x - half, centre.z - half),
        Vec2::new(centre.x + half, centre.z - half),
        Vec2::new(centre.x + half, centre.z + half),
        Vec2::new(centre.x - half, centre.z + half),
    ]
    .map(|point| {
        let surface = terrain_surface_height_at_world(world, config, point.x, point.y)
            .unwrap_or(centre.y)
            .max(water_height);
        Vec3::new(point.x, surface + WORLD_DIAGNOSTIC_OVERLAY_LIFT, point.y)
    })
}

pub(crate) fn world_diagnostic_overlay_mesh(
    mode: WorldDiagnosticMode,
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    fine_navigation: &stream_town_domain::NavGrid,
) -> Option<Mesh> {
    let water_height = f32::from(config.world.water_level_centimetres) * 0.01;
    if mode == WorldDiagnosticMode::Pathfinding {
        return pathfinding_diagnostic_overlay_mesh(config, world, fine_navigation, water_height);
    }
    let estimated_cells = world.resources.len() + simulation.buildings.len() * 4;
    let mut positions = Vec::with_capacity(estimated_cells.saturating_mul(4));
    let mut normals = Vec::with_capacity(positions.capacity());
    let mut uvs = Vec::with_capacity(positions.capacity());
    let mut indices = Vec::with_capacity(estimated_cells.saturating_mul(6));
    let fine_tile_size =
        config.world.cell_size / f32::from(NAVIGATION_SUBDIVISIONS) * WORLD_DIAGNOSTIC_TILE_SCALE;
    for cell in floorplan_diagnostic_cells(content, simulation, world) {
        append_world_diagnostic_quad(
            diagnostic_navigation_surface_quad(cell, fine_tile_size, config, world, water_height),
            &mut positions,
            &mut normals,
            &mut uvs,
            &mut indices,
        );
    }
    if positions.is_empty() {
        return None;
    }
    Some(
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices)),
    )
}

pub(crate) fn pathfinding_diagnostic_overlay_mesh(
    config: &GameConfig,
    world: &GeneratedWorld,
    navigation: &stream_town_domain::NavGrid,
    water_height: f32,
) -> Option<Mesh> {
    let width = navigation.width();
    let height = navigation.height();
    let vertex_width = usize::from(width) + 1;
    let vertex_height = usize::from(height) + 1;
    let vertex_count = vertex_width.saturating_mul(vertex_height);
    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = Vec::with_capacity(vertex_count);
    let mut uvs = Vec::with_capacity(vertex_count);
    let subdivision = f32::from(NAVIGATION_SUBDIVISIONS);
    let fine_half_offset = subdivision * 0.5;
    let coarse_half_x = f32::from(config.world.width.saturating_sub(1)) * 0.5;
    let coarse_half_z = f32::from(config.world.height.saturating_sub(1)) * 0.5;
    let terrain_limit_x = coarse_half_x * config.world.cell_size;
    let terrain_limit_z = coarse_half_z * config.world.cell_size;
    for z in 0..=height {
        for x in 0..=width {
            let logical_x = (f32::from(x) - fine_half_offset) / subdivision;
            let logical_z = (f32::from(z) - fine_half_offset) / subdivision;
            let world_x = (logical_x - coarse_half_x) * config.world.cell_size;
            let world_z = (logical_z - coarse_half_z) * config.world.cell_size;
            let surface = terrain_surface_height_at_world(
                world,
                config,
                world_x.clamp(-terrain_limit_x, terrain_limit_x),
                world_z.clamp(-terrain_limit_z, terrain_limit_z),
            )
            .unwrap_or(water_height);
            positions.push([
                world_x,
                surface.max(water_height) + WORLD_DIAGNOSTIC_OVERLAY_LIFT,
                world_z,
            ]);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([
                f32::from(x) / f32::from(width.max(1)),
                f32::from(z) / f32::from(height.max(1)),
            ]);
        }
    }
    let mut indices = Vec::new();
    for z in 0..height {
        for x in 0..width {
            if navigation.is_walkable(GridPos { x, z }) {
                continue;
            }
            let top_left = u32::try_from(usize::from(z) * vertex_width + usize::from(x))
                .expect("diagnostic overlay vertex count fits u32");
            let bottom_left = top_left
                .checked_add(u32::try_from(vertex_width).expect("diagnostic row width fits u32"))
                .expect("diagnostic overlay vertex count fits u32");
            indices.extend_from_slice(&[
                top_left,
                bottom_left,
                top_left + 1,
                top_left + 1,
                bottom_left,
                bottom_left + 1,
            ]);
        }
    }
    if indices.is_empty() {
        return None;
    }
    Some(
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices)),
    )
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn sync_world_diagnostic_view(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    fine_navigation: Res<FineNavigationRuntime>,
    mut runtime: ResMut<WorldDiagnosticRuntime>,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut visual_roots: Query<
        (
            Entity,
            &mut Visibility,
            Option<&WorldDiagnosticVisibilityBackup>,
        ),
        Or<(
            With<RuntimeBuilding>,
            With<ResourceNode>,
            With<EnemyCamp>,
            With<BuildingPlacementGhost>,
            With<BuildingHealthOverlay>,
        )>,
    >,
    overlays: Query<(Entity, &WorldDiagnosticOverlay)>,
) {
    if runtime.mode.is_some() {
        runtime.remaining_seconds = (runtime.remaining_seconds - time.delta_secs()).max(0.0);
        if runtime.remaining_seconds <= f32::EPSILON {
            runtime.mode = None;
        }
    }

    let Some(mode) = runtime.mode else {
        for (entity, mut visibility, backup) in &mut visual_roots {
            let Some(backup) = backup else {
                continue;
            };
            *visibility = backup.0;
            commands
                .entity(entity)
                .remove::<WorldDiagnosticVisibilityBackup>();
        }
        for (entity, _) in &overlays {
            commands.entity(entity).try_despawn();
        }
        return;
    };

    for (entity, mut visibility, backup) in &mut visual_roots {
        if backup.is_none() {
            commands
                .entity(entity)
                .insert(WorldDiagnosticVisibilityBackup(*visibility));
        }
        *visibility = Visibility::Hidden;
    }
    if overlays.iter().any(|(_, overlay)| overlay.mode == mode) {
        return;
    }
    for (entity, _) in &overlays {
        commands.entity(entity).try_despawn();
    }
    let (Some(meshes), Some(materials), Some(fine_navigation)) = (
        meshes.as_deref_mut(),
        materials.as_deref_mut(),
        fine_navigation.grid.as_ref(),
    ) else {
        return;
    };
    let Some(mesh) = world_diagnostic_overlay_mesh(
        mode,
        &config.0,
        &content.0,
        &simulation.0,
        &world.generated,
        fine_navigation,
    ) else {
        return;
    };
    let material = runtime.black_material.clone().unwrap_or_else(|| {
        let material = materials.add(StandardMaterial {
            base_color: Color::BLACK,
            unlit: true,
            depth_bias: WORLD_DIAGNOSTIC_DEPTH_BIAS,
            ..default()
        });
        runtime.black_material = Some(material.clone());
        material
    });
    commands.spawn((
        WorldEntity,
        WorldDiagnosticOverlay { mode },
        Name::new(match mode {
            WorldDiagnosticMode::Pathfinding => "Pathfinding Accessibility Overlay",
            WorldDiagnosticMode::Floorplan => "Placement Footprint Overlay",
        }),
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(material),
        NotShadowCaster,
        NoFrustumCulling,
    ));
}

pub(crate) fn enemy_camp_navigation_region(
    position: GridPos,
    visual: [u16; 2],
    world: &GeneratedWorld,
) -> Option<stream_town_domain::DirtyRegion> {
    let footprint = [visual[0].min(3), visual[1].min(3)];
    let offset = [
        visual[0].saturating_sub(footprint[0]) / 2,
        visual[1].saturating_sub(footprint[1]) / 2,
    ];
    building_region(
        GridPos {
            x: position.x.checked_add(offset[0])?,
            z: position.z.checked_add(offset[1])?,
        },
        footprint,
        world,
    )
}

pub(crate) fn foliage_clearance_regions(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
) -> Vec<stream_town_domain::DirtyRegion> {
    let building_regions = simulation.buildings.values().filter_map(|building| {
        let definition = building_def_for_archetype(content, &building.archetype)?;
        if definition.archetype.as_str() == "archetype:building:path" {
            return None;
        }
        building_region(
            building.position,
            rotated_footprint(definition.footprint, building.rotation_quarter_turns),
            world,
        )
    });
    let camp_regions = simulation.enemy_camps.values().filter_map(|camp| {
        let archetype = content.archetypes.get(&camp.archetype)?;
        building_region(camp.position, archetype.footprint, world)
    });
    building_regions.chain(camp_regions).collect()
}

pub(crate) fn foliage_clearance_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
) -> HashSet<GridPos> {
    foliage_clearance_regions(content, simulation, world)
        .into_iter()
        .flat_map(|region| {
            (region.min.z..=region.max.z)
                .flat_map(move |z| (region.min.x..=region.max.x).map(move |x| GridPos { x, z }))
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct FoliageClearanceInputSignature {
    structures: u64,
    paths: u64,
    wear: u64,
}

#[derive(Default)]
pub(crate) struct FoliageClearanceSyncState {
    initialized: bool,
    signature: FoliageClearanceInputSignature,
}

pub(crate) fn foliage_clearance_input_signature(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    traversal_wear: &TraversalWearRuntime,
    paths: Option<&PathSurfaceRuntime>,
    terrain: &stream_town_domain::TerrainAppearanceConfig,
) -> FoliageClearanceInputSignature {
    fn fold_u64(mut hash: u64, value: u64) -> u64 {
        for byte in value.to_le_bytes() {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    let mut structures = fold_u64(
        0xcbf2_9ce4_8422_2325_u64,
        u64::from(world.navigation.width()) | (u64::from(world.navigation.height()) << 16),
    );
    for building in simulation.buildings.values() {
        let Some(definition) = building_def_for_archetype(content, &building.archetype) else {
            continue;
        };
        if definition.archetype.as_str() == "archetype:building:path" {
            continue;
        }
        structures = fold_u64(structures, stable_id_hash(&building.id));
        structures = fold_u64(
            structures,
            u64::from(building.position.x) | (u64::from(building.position.z) << 16),
        );
        structures = fold_u64(
            structures,
            u64::from(u32::from_ne_bytes(
                building.rotation_quarter_turns.to_ne_bytes(),
            )),
        );
        structures = fold_u64(
            structures,
            u64::from(definition.footprint[0]) | (u64::from(definition.footprint[1]) << 16),
        );
    }
    for camp in simulation.enemy_camps.values() {
        let Some(archetype) = content.archetypes.get(&camp.archetype) else {
            continue;
        };
        structures = fold_u64(structures, stable_id_hash(&camp.id));
        structures = fold_u64(
            structures,
            u64::from(camp.position.x) | (u64::from(camp.position.z) << 16),
        );
        structures = fold_u64(
            structures,
            u64::from(archetype.footprint[0]) | (u64::from(archetype.footprint[1]) << 16),
        );
    }

    // Hash the set of cells whose wear currently hides foliage. HashMap order is
    // intentionally neutralized so an unchanged set cannot invalidate the scan.
    let wear_threshold = traversal_score_for_rate(
        terrain.traversal_fade_start_per_minute,
        terrain.traversal_half_life_seconds,
    );
    let mut wear_xor = u64::from(wear_threshold.to_bits());
    let mut wear_sum = 0_u64;
    let mut wear_count = 0_u64;
    for (position, cell) in &traversal_wear.cells {
        if cell.score < wear_threshold {
            continue;
        }
        let cell_hash = fold_u64(
            0xcbf2_9ce4_8422_2325_u64,
            u64::from(position.x) | (u64::from(position.z) << 16),
        );
        wear_xor ^= cell_hash;
        wear_sum = wear_sum.wrapping_add(cell_hash.rotate_left(23));
        wear_count += 1;
    }
    let wear = fold_u64(fold_u64(wear_xor, wear_sum), wear_count);
    let paths = paths.map_or(0, |paths| {
        paths.applied_signature ^ u64::from(paths.initialized)
    });

    FoliageClearanceInputSignature {
        structures,
        paths,
        wear,
    }
}

pub(crate) fn region_contains_grid_position(
    region: stream_town_domain::DirtyRegion,
    position: GridPos,
) -> bool {
    (region.min.x..=region.max.x).contains(&position.x)
        && (region.min.z..=region.max.z).contains(&position.z)
}

/// Mirrors Unity's placement-time foliage clearing without destroying the
/// deterministic generated instances. Deriving visibility from current
/// structural occupancy also restores foliage when a building is removed and
/// recomputes the correct result after loading a different save.
pub(crate) fn sync_foliage_clearance(
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    traversal_wear: Res<TraversalWearRuntime>,
    paths: Option<Res<PathSurfaceRuntime>>,
    added_foliage: Query<(), Added<FoliageVisual>>,
    added_grounding: Query<(), (With<FoliageVisual>, Added<PendingSurfaceGrounding>)>,
    mut removed_foliage: RemovedComponents<FoliageVisual>,
    mut removed_grounding: RemovedComponents<PendingSurfaceGrounding>,
    mut stats: ResMut<WorldRenderStats>,
    mut sync: Local<FoliageClearanceSyncState>,
    mut foliage: FoliageClearanceQuery,
) {
    let signature = foliage_clearance_input_signature(
        &content.0,
        &simulation.0,
        &world.generated,
        &traversal_wear,
        paths.as_deref(),
        &config.0.terrain,
    );
    let membership_changed = !added_foliage.is_empty()
        || !added_grounding.is_empty()
        || removed_foliage.read().next().is_some()
        || removed_grounding.read().next().is_some();
    if sync.initialized && sync.signature == signature && !membership_changed {
        return;
    }
    sync.initialized = true;
    sync.signature = signature;

    // A generated town contains many more foliage entities than occupied
    // structural cells. Expanding the small set of building/camp rectangles
    // once avoids testing every foliage entity against every rectangle. The
    // input signature above also avoids repeating this full scan on frames where
    // no structure, path, wear threshold, grounding, or foliage membership changed.
    let structural_cells = foliage_clearance_cells(&content.0, &simulation.0, &world.generated);
    let mut visible_instances = 0;
    for (location, navigation_location, batch, pending_grounding, mut visibility) in &mut foliage {
        if let Some(batch) = batch {
            debug_assert_eq!(batch.0.chunk_x, location.0.x / FOLIAGE_BATCH_CHUNK_CELLS);
            debug_assert_eq!(batch.0.chunk_z, location.0.z / FOLIAGE_BATCH_CHUNK_CELLS);
        }
        let structure_hidden = structural_cells.contains(&location.0);
        let path_hidden = navigation_location.is_some_and(|location| {
            paths
                .as_deref()
                .is_some_and(|paths| paths.levels.contains_key(&location.0))
        });
        let wear_score = traversal_wear
            .cells
            .get(&location.0)
            .map(|cell| cell.score)
            .unwrap_or_default();
        let should_be_hidden = foliage_should_be_hidden(
            structure_hidden,
            path_hidden,
            pending_grounding.is_some(),
            wear_score,
            &config.0.terrain,
        );
        if should_be_hidden && !matches!(*visibility, Visibility::Hidden) {
            *visibility = Visibility::Hidden;
        } else if !should_be_hidden && matches!(*visibility, Visibility::Hidden) {
            *visibility = Visibility::Inherited;
        }
        if !should_be_hidden {
            visible_instances += 1;
        }
    }
    stats.foliage_visible_instances = visible_instances;
}

pub(crate) fn foliage_should_be_hidden(
    structure_hidden: bool,
    path_hidden: bool,
    pending_grounding: bool,
    wear_score: f32,
    settings: &stream_town_domain::TerrainAppearanceConfig,
) -> bool {
    structure_hidden
        || path_hidden
        || pending_grounding
        || wear_score
            >= traversal_score_for_rate(
                settings.traversal_fade_start_per_minute,
                settings.traversal_half_life_seconds,
            )
}

pub(crate) fn decay_traversal_wear(
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    mut traversal_wear: ResMut<TraversalWearRuntime>,
) {
    traversal_wear.decay(time.delta_secs(), &config.0.terrain);
}

pub(crate) fn sync_traversal_wear_texture(
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    mut traversal_wear: ResMut<TraversalWearRuntime>,
    images: Option<ResMut<Assets<Image>>>,
    terrain_materials: Option<ResMut<Assets<TerrainMaterial>>>,
) {
    if !traversal_wear.texture_dirty {
        return;
    }
    let (Some(mut images), Some(mut terrain_materials)) = (images, terrain_materials) else {
        return;
    };
    let width = config.0.world.width.max(1);
    let height = config.0.world.height.max(1);
    let Some(mut image) = images.get_mut(&render.traversal_wear) else {
        return;
    };
    if image.width() != u32::from(width) || image.height() != u32::from(height) {
        *image = traversal_wear_image(width, height);
    }
    write_traversal_wear_pixels(
        &mut image,
        width,
        height,
        &traversal_wear.cells,
        &config.0.terrain,
    );
    if let Some(mut material) = terrain_materials.get_mut(&render.ground) {
        material.extension.parameters.traversal_grid = Vec4::new(
            f32::from(width),
            f32::from(height),
            config.0.world.cell_size,
            1.0,
        );
    }
    traversal_wear.texture_dirty = false;
}

pub(crate) fn path_surface_signature(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    width: u16,
    height: u16,
) -> u64 {
    let path_archetype = content
        .buildings
        .get(&StableId::new("building:path").expect("static path ID"))
        .map(|definition| &definition.archetype);
    let mut signature = u64::from(width) << 48 | u64::from(height) << 32;
    for building in simulation.buildings.values().filter(|building| {
        building.complete
            && path_archetype.is_some_and(|archetype| *archetype == building.archetype)
    }) {
        let fine_position = simulation.path_navigation_positions.get(&building.id);
        signature = signature
            .wrapping_mul(0x0000_0100_0000_01b3)
            .wrapping_add(stable_id_hash(&building.id))
            .wrapping_add(u64::from(building.position.x) << 16)
            .wrapping_add(u64::from(building.position.z))
            .wrapping_add(fine_position.map_or(0, |position| {
                (u64::from(position.x) << 24) | (u64::from(position.z) << 8)
            }))
            .wrapping_add(
                u64::from(u32::from_ne_bytes(
                    building.rotation_quarter_turns.to_ne_bytes(),
                )) << 8,
            )
            .wrapping_add(u64::from(building.level) << 40);
    }
    signature
}

pub(crate) fn sync_path_surface_texture(
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    render: Res<RenderAssets>,
    mut runtime: ResMut<PathSurfaceRuntime>,
    images: Option<ResMut<Assets<Image>>>,
    mut terrain_materials: Option<ResMut<Assets<TerrainMaterial>>>,
) {
    let width = config
        .0
        .world
        .width
        .saturating_mul(NAVIGATION_SUBDIVISIONS)
        .max(1);
    let height = config
        .0
        .world
        .height
        .saturating_mul(NAVIGATION_SUBDIVISIONS)
        .max(1);
    let signature = path_surface_signature(&content.0, &simulation.0, width, height);
    if runtime.initialized && runtime.applied_signature == signature {
        return;
    }
    let Some(mut images) = images else {
        return;
    };
    let Some(mut image) = images.get_mut(&render.path_surface) else {
        return;
    };
    if image.width() != u32::from(width) || image.height() != u32::from(height) {
        *image = traversal_wear_image(width, height);
    }
    let path_id = StableId::new("building:path").expect("static path ID");
    let path_archetype = content
        .0
        .buildings
        .get(&path_id)
        .map(|definition| &definition.archetype);
    let Some(data) = image.data.as_mut() else {
        return;
    };
    for pixel in data.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0, 0, 0, u8::MAX]);
    }
    runtime.levels.clear();
    for building in simulation.0.buildings.values().filter(|building| {
        building.complete
            && path_archetype.is_some_and(|archetype| *archetype == building.archetype)
    }) {
        for position in linear_navigation_cells(&content.0, &simulation.0, building, &path_id) {
            if position.x >= width || position.z >= height {
                continue;
            }
            let index =
                (usize::from(position.z) * usize::from(width) + usize::from(position.x)) * 4;
            if let Some(level) = data.get_mut(index) {
                *level = u8::try_from(building.level.min(u16::from(u8::MAX))).unwrap_or(u8::MAX);
            }
            runtime
                .levels
                .entry(position)
                .and_modify(|level| *level = (*level).max(building.level))
                .or_insert(building.level);
        }
    }
    if let Some(materials) = terrain_materials.as_mut()
        && let Some(mut material) = materials.get_mut(&render.ground)
    {
        material.extension.parameters.path_grid = Vec4::new(
            f32::from(width),
            f32::from(height),
            config.0.world.cell_size / f32::from(NAVIGATION_SUBDIVISIONS),
            1.0,
        );
    }
    runtime.applied_signature = signature;
    runtime.initialized = true;
}

pub(crate) fn write_traversal_wear_pixels(
    image: &mut Image,
    width: u16,
    height: u16,
    cells: &HashMap<GridPos, TraversalWearCell>,
    settings: &stream_town_domain::TerrainAppearanceConfig,
) {
    let Some(data) = image.data.as_mut() else {
        return;
    };
    for pixel in data.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0, 0, 0, u8::MAX]);
    }
    for (position, cell) in cells {
        if position.x >= width || position.z >= height {
            continue;
        }
        let index = (usize::from(position.z) * usize::from(width) + usize::from(position.x)) * 4;
        let wear = traversal_wear_byte(cell.score, settings);
        if let Some(red) = data.get_mut(index) {
            *red = wear;
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub(crate) fn traversal_wear_byte(
    score: f32,
    settings: &stream_town_domain::TerrainAppearanceConfig,
) -> u8 {
    // The fraction is clamped to [0, 1], so the rounded value is always in
    // the complete u8 range before this intentional texture conversion.
    (traversal_wear_fraction(score, settings) * f32::from(u8::MAX)).round() as u8
}

pub(crate) fn quarter_turn_rotation(rotation_quarter_turns: i32) -> Quat {
    let normalized = i16::try_from(rotation_quarter_turns.rem_euclid(4))
        .expect("normalized quarter turn fits i16");
    Quat::from_rotation_y(-f32::from(normalized) * std::f32::consts::FRAC_PI_2)
}

pub(crate) fn building_site_is_available(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
) -> bool {
    let Some(region) = building_region(position, footprint, world) else {
        return false;
    };
    (region.min.z..=region.max.z).all(|cell_z| {
        (region.min.x..=region.max.x).all(|cell_x| {
            world.navigation.is_walkable(GridPos {
                x: cell_x,
                z: cell_z,
            })
        })
    }) && !world.resources.iter().any(|resource| {
        resource.amount > 0
            && (region.min.x..=region.max.x).contains(&resource.position.x)
            && (region.min.z..=region.max.z).contains(&resource.position.z)
    })
}

#[cfg(test)]
pub(crate) fn regions_overlap(
    left: stream_town_domain::DirtyRegion,
    right: stream_town_domain::DirtyRegion,
) -> bool {
    left.min.x <= right.max.x
        && left.max.x >= right.min.x
        && left.min.z <= right.max.z
        && left.max.z >= right.min.z
}

#[cfg(test)]
pub(crate) fn building_site_is_available_for_simulation(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
) -> bool {
    if !building_site_is_available(world, position, footprint) {
        return false;
    }
    let Some(candidate) = building_region(position, footprint, world) else {
        return false;
    };
    let overlaps_building = simulation.buildings.values().any(|building| {
        let Some(definition) = building_def_for_archetype(content, &building.archetype) else {
            return false;
        };
        // Paths belong to the fine navigation grid and never reserve their
        // containing coarse floorplan cell. A later building may therefore
        // coexist with a path running through its inset pedestrian margin.
        if definition.archetype.as_str() == "archetype:building:path" {
            return false;
        }
        building_region(
            building.position,
            rotated_footprint(definition.footprint, building.rotation_quarter_turns),
            world,
        )
        .is_some_and(|occupied| regions_overlap(candidate, occupied))
    });
    let overlaps_camp = simulation.enemy_camps.values().any(|camp| {
        content
            .archetypes
            .get(&camp.archetype)
            .and_then(|archetype| building_region(camp.position, archetype.footprint, world))
            .is_some_and(|occupied| regions_overlap(candidate, occupied))
    });
    !overlaps_building && !overlaps_camp
}

pub(crate) fn find_building_site(
    world: &GeneratedWorld,
    near: GridPos,
    footprint: [u16; 2],
) -> Option<GridPos> {
    let mut candidates = Vec::new();
    for z in 0..world.navigation.height() {
        for x in 0..world.navigation.width() {
            let position = GridPos { x, z };
            if building_site_is_available(world, position, footprint) {
                candidates.push(position);
            }
        }
    }
    candidates.sort_by_key(|position| {
        (
            position.x.abs_diff(near.x) + position.z.abs_diff(near.z),
            position.z,
            position.x,
        )
    });
    candidates.into_iter().next()
}

pub(crate) const ENEMY_CAMP_PLACEMENT_ATTEMPTS: u64 = 500;

pub(crate) fn generated_enemy_camp_hash(seed: u64, layer: &StableId, serial: u64) -> u64 {
    let mut mixed = layer.as_str().bytes().fold(seed, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    });
    mixed = mixed.wrapping_add(serial.wrapping_mul(0x9e37_79b9_7f4a_7c15));
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    mixed ^ (mixed >> 31)
}

pub(crate) fn rounded_milli_cells(value: u32) -> i32 {
    i32::try_from(value.saturating_add(500) / 1_000).unwrap_or(i32::MAX)
}

pub(crate) fn enemy_camp_candidate(
    config: &GameConfig,
    layer: &EnemyCampGenerationDef,
    footprint: [u16; 2],
    camp_index: u16,
    attempt: u64,
) -> Option<(GridPos, GridPos)> {
    let serial = u64::from(camp_index)
        .saturating_mul(ENEMY_CAMP_PLACEMENT_ATTEMPTS)
        .saturating_add(attempt);
    let random = generated_enemy_camp_hash(config.world.seed, &layer.id, serial);
    let sample_axis = |axis: usize, bits: u32| -> i32 {
        let minimum = rounded_milli_cells(layer.minimum_absolute_offset_milli_cells[axis]);
        let maximum = rounded_milli_cells(layer.maximum_absolute_offset_milli_cells[axis]);
        let span = u64::try_from(maximum.saturating_sub(minimum))
            .unwrap_or_default()
            .saturating_add(1);
        let magnitude = minimum.saturating_add(
            i32::try_from((random >> bits) % span).expect("camp offset span fits i32"),
        );
        if (random >> (bits + 15)) & 1 == 0 {
            -magnitude
        } else {
            magnitude
        }
    };
    let map_centre = GridPos {
        x: config.world.width / 2,
        z: config.world.height / 2,
    };
    let centre_x = i32::from(map_centre.x).saturating_add(sample_axis(0, 0));
    let centre_z = i32::from(map_centre.z).saturating_add(sample_axis(1, 32));
    let origin_x = centre_x.saturating_sub(i32::from(footprint[0] / 2));
    let origin_z = centre_z.saturating_sub(i32::from(footprint[1] / 2));
    let origin = GridPos {
        x: u16::try_from(origin_x).ok()?,
        z: u16::try_from(origin_z).ok()?,
    };
    let centre = GridPos {
        x: u16::try_from(centre_x).ok()?,
        z: u16::try_from(centre_z).ok()?,
    };
    Some((origin, centre))
}

pub(crate) fn enemy_camp_region_is_clear(
    world: &GeneratedWorld,
    region: stream_town_domain::DirtyRegion,
    excluded: &[stream_town_domain::DirtyRegion],
) -> bool {
    if excluded.iter().any(|other| {
        region.min.x <= other.max.x
            && region.max.x >= other.min.x
            && region.min.z <= other.max.z
            && region.max.z >= other.min.z
    }) {
        return false;
    }
    (region.min.z..=region.max.z).all(|z| {
        (region.min.x..=region.max.x).all(|x| {
            let position = GridPos { x, z };
            world.navigation.height_at(position).unwrap_or_default() > 0
        })
    })
}

pub(crate) fn enemy_camp_has_town_route(
    world: &GeneratedWorld,
    region: stream_town_domain::DirtyRegion,
    town_reachable: &HashSet<GridPos>,
) -> bool {
    let min_x = region.min.x.saturating_sub(1);
    let min_z = region.min.z.saturating_sub(1);
    let max_x = region
        .max
        .x
        .saturating_add(1)
        .min(world.navigation.width().saturating_sub(1));
    let max_z = region
        .max
        .z
        .saturating_add(1)
        .min(world.navigation.height().saturating_sub(1));
    (min_z..=max_z)
        .flat_map(|z| (min_x..=max_x).map(move |x| GridPos { x, z }))
        .filter(|position| {
            position.x < region.min.x
                || position.x > region.max.x
                || position.z < region.min.z
                || position.z > region.max.z
        })
        .any(|position| town_reachable.contains(&position))
}

pub(crate) fn enemy_camp_town_reachable_cells(
    world: &GeneratedWorld,
    town_hall_approach: GridPos,
) -> HashSet<GridPos> {
    if !world.navigation.is_walkable(town_hall_approach) {
        return HashSet::new();
    }
    let mut reachable = HashSet::from([town_hall_approach]);
    let mut open = VecDeque::from([town_hall_approach]);
    while let Some(position) = open.pop_front() {
        for (neighbour, _) in world
            .navigation
            .walkable_neighbours(position)
            .into_iter()
            .flatten()
        {
            if reachable.insert(neighbour) {
                open.push_back(neighbour);
            }
        }
    }
    reachable
}

pub(crate) fn seed_generated_enemy_camps(
    config: &GameConfig,
    content: &ContentCatalog,
    world: &mut GeneratedWorld,
    simulation: &mut WorldSimulation,
) -> usize {
    if !simulation.enemy_camps.is_empty() {
        return 0;
    }
    let town_hall_centre = restored_town_hall_position(content, simulation, config);
    let town_hall_id = StableId::new("building:townhall").expect("static building ID");
    let town_hall_approach = simulation
        .buildings
        .get(&town_hall_id)
        .and_then(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            building_approach(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                town_hall_centre,
            )
        })
        .or_else(|| nearest_walkable(world, town_hall_centre))
        .unwrap_or(town_hall_centre);
    let mut town_reachable = enemy_camp_town_reachable_cells(world, town_hall_approach);
    let mut excluded = simulation
        .buildings
        .values()
        .filter_map(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            building_region(
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                world,
            )
        })
        .collect::<Vec<_>>();
    let mut centres = Vec::<GridPos>::new();
    let mut spawned = 0_usize;
    for layer in &content.enemy_camp_generation {
        let Some(archetype) = content.archetypes.get(&layer.camp_archetype) else {
            continue;
        };
        for camp_index in 0..layer.maximum_camps {
            let placement = (0..ENEMY_CAMP_PLACEMENT_ATTEMPTS).find_map(|attempt| {
                let (origin, centre) =
                    enemy_camp_candidate(config, layer, archetype.footprint, camp_index, attempt)?;
                let visual_region = building_region(origin, archetype.footprint, world)?;
                let navigation_region =
                    enemy_camp_navigation_region(origin, archetype.footprint, world)?;
                let centre_distance = grid_distance_squared(
                    centre,
                    GridPos {
                        x: config.world.width / 2,
                        z: config.world.height / 2,
                    },
                );
                let centre_minimum = u128::from(layer.minimum_distance_from_centre_milli_cells);
                if u128::from(centre_distance).saturating_mul(1_000_000)
                    < centre_minimum.saturating_mul(centre_minimum)
                    || centres.iter().any(|other| {
                        let distance = u128::from(grid_distance_squared(*other, centre))
                            .saturating_mul(1_000_000);
                        let minimum = u128::from(layer.minimum_distance_between_camps_milli_cells);
                        distance < minimum.saturating_mul(minimum)
                    })
                    || !enemy_camp_region_is_clear(world, visual_region, &excluded)
                    || !enemy_camp_has_town_route(world, navigation_region, &town_reachable)
                {
                    return None;
                }
                Some((origin, centre, navigation_region, visual_region))
            });
            let Some((origin, centre, navigation_region, visual_region)) = placement else {
                warn!(
                    layer = %layer.id,
                    camp_index,
                    attempts = ENEMY_CAMP_PLACEMENT_ATTEMPTS,
                    "could not place every authored enemy camp"
                );
                continue;
            };
            let id = StableId::new(format!("enemy_camp:{}:{camp_index:02}", layer.id.as_str()))
                .expect("generated camp IDs are valid");
            let health = archetype.health.as_ref().map_or(1_000, |health| {
                i32::try_from(health.max_health).unwrap_or(i32::MAX)
            });
            simulation.enemy_camps.insert(
                id.clone(),
                EnemyCampState {
                    id,
                    archetype: layer.camp_archetype.clone(),
                    position: origin,
                    health,
                    // EnemySpawner.Awake seeds its elapsed timer to the full
                    // interval, making the first eligible night spawn immediate.
                    spawn_remaining_seconds: 0.0,
                    spawned_enemies: BTreeSet::new(),
                },
            );
            world
                .navigation
                .set_blocked(navigation_region, true)
                .expect("validated enemy camp region updates navigation");
            town_reachable = enemy_camp_town_reachable_cells(world, town_hall_approach);
            centres.push(centre);
            excluded.push(visual_region);
            spawned += 1;
        }
    }
    spawned
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_enemy_camp(
    commands: &mut Commands,
    config: &GameConfig,
    world: &GeneratedWorld,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    render: &RenderAssets,
    id: &StableId,
    archetype: &ArchetypeDef,
    position: GridPos,
) {
    let centre = GridPos {
        x: position.x + archetype.footprint[0] / 2,
        z: position.z + archetype.footprint[1] / 2,
    };
    let world_position = grid_to_world_on_surface(centre, config, world);
    let mut entity = commands.spawn((
        WorldEntity,
        EnemyCamp { id: id.clone() },
        GridLocation(position),
        Transform::from_translation(world_position),
    ));
    if let Some(scene) = default_archetype_scene(archetype).filter(|scene| {
        asset_server.is_some() && converted_asset_exists(asset_root, &scene.asset_path)
    }) {
        entity.insert((
            WorldAssetRoot(
                asset_server
                    .expect("asset server checked above")
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            Transform::from_translation(world_position)
                .with_scale(Vec3::splat(config.world.cell_size / 2.0)),
        ));
        if let Some(material) = prefab_material_spec(archetype, scene, presentation, render) {
            entity.insert(material);
        }
    } else {
        let size = Vec3::new(
            f32::from(archetype.footprint[0]) * config.world.cell_size * 0.8,
            config.world.cell_size * 1.2,
            f32::from(archetype.footprint[1]) * config.world.cell_size * 0.8,
        );
        entity.insert((
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.building.clone()),
            Transform::from_translation(world_position + Vec3::Y * size.y * 0.5).with_scale(size),
        ));
    }
}

pub(crate) fn enemy_spawn_position(
    world: &GeneratedWorld,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    enemy_navigation: Option<&EnemyNavigationField>,
    camp: &EnemyCampState,
    spawner: &stream_town_domain::EnemySpawnerDef,
    footprint: [u16; 2],
    serial: u64,
) -> GridPos {
    let offset = spawner.spawn_offsets_milli_cells
        [usize::try_from(serial % spawner.spawn_offsets_milli_cells.len() as u64).unwrap_or(0)];
    let offset_cells = |value: i32| -> i32 {
        if value >= 0 {
            (value + 500) / 1_000
        } else {
            (value - 500) / 1_000
        }
    };
    let x = i64::from(camp.position.x)
        + i64::from(footprint[0] / 2)
        + i64::from(offset_cells(offset[0]));
    let z = i64::from(camp.position.z)
        + i64::from(footprint[1] / 2)
        + i64::from(offset_cells(offset[1]));
    let desired = GridPos {
        x: u16::try_from(x.clamp(0, i64::from(world.navigation.width() - 1))).unwrap_or(0),
        z: u16::try_from(z.clamp(0, i64::from(world.navigation.height() - 1))).unwrap_or(0),
    };
    let limit = world.navigation.width().max(world.navigation.height());
    for radius in 0..limit {
        for z in desired.z.saturating_sub(radius)
            ..=desired
                .z
                .saturating_add(radius)
                .min(world.navigation.height() - 1)
        {
            for x in desired.x.saturating_sub(radius)
                ..=desired
                    .x
                    .saturating_add(radius)
                    .min(world.navigation.width() - 1)
            {
                let candidate = GridPos { x, z };
                let reaches_town = enemy_navigation.map_or_else(
                    || {
                        nearest_reachable_building_to_town_hall(
                            content, simulation, world, candidate,
                        )
                        .is_some()
                    },
                    |field| {
                        field
                            .destination(placement_to_navigation_centre(candidate))
                            .is_some()
                    },
                );
                if world.navigation.is_walkable(candidate) && reaches_town {
                    return candidate;
                }
            }
        }
    }
    nearest_walkable(world, desired)
        .unwrap_or_else(|| nearest_walkable(world, camp.position).unwrap_or(camp.position))
}

pub(crate) fn weighted_enemy_archetype(
    spawner: &stream_town_domain::EnemySpawnerDef,
    seed: u64,
    serial: u64,
) -> StableId {
    let total = spawner
        .weighted_enemies
        .iter()
        .map(|entry| u64::from(entry.weight_milli))
        .sum::<u64>();
    let mut mixed = seed.wrapping_add(serial.wrapping_mul(0x9e37_79b9_7f4a_7c15));
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    let mut value = (mixed ^ (mixed >> 31)) % total;
    for entry in &spawner.weighted_enemies {
        if value < u64::from(entry.weight_milli) {
            return entry.enemy_archetype.clone();
        }
        value -= u64::from(entry.weight_milli);
    }
    spawner.weighted_enemies[0].enemy_archetype.clone()
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_runtime_enemy(
    commands: &mut Commands,
    config: &GameConfig,
    world: &GeneratedWorld,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    render: &RenderAssets,
    simulation: &mut WorldSimulation,
    archetype_id: StableId,
    position: GridPos,
) -> Option<StableId> {
    let archetype = content.archetypes.get(&archetype_id)?;
    let enemy = archetype.enemy.as_ref()?;
    let base_health = archetype.health.as_ref()?.max_health;
    let player_count = simulation
        .actors
        .values()
        .filter(|actor| actor.role.as_str() != "role:enemy")
        .count();
    let additional = u64::from(enemy.additional_health_milli_per_player)
        .saturating_mul(u64::try_from(player_count).unwrap_or(u64::MAX))
        / 1_000;
    let max_health =
        u32::try_from(u64::from(base_health).saturating_add(additional)).unwrap_or(u32::MAX);
    let id = loop {
        let serial = simulation.next_enemy_serial;
        simulation.next_enemy_serial = simulation.next_enemy_serial.saturating_add(1);
        let id =
            StableId::new(format!("actor:enemy_{serial:08}")).expect("runtime enemy IDs are valid");
        if !simulation.actors.contains_key(&id) {
            break id;
        }
    };
    if !simulation.spawn_enemy(
        id.clone(),
        archetype_id.clone(),
        position,
        i32::try_from(max_health).unwrap_or(i32::MAX),
    ) {
        return None;
    }
    spawn_runtime_enemy_entity(
        commands,
        config,
        world,
        content,
        presentation,
        asset_server,
        asset_root,
        render,
        &id,
        archetype_id,
        position,
    );
    Some(id)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_runtime_enemy_entity(
    commands: &mut Commands,
    config: &GameConfig,
    world: &GeneratedWorld,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    render: &RenderAssets,
    id: &StableId,
    archetype_id: StableId,
    position: GridPos,
) {
    let Some(archetype) = content.archetypes.get(&archetype_id) else {
        return;
    };
    let world_position = grid_to_world_on_surface(position, config, world);
    let scene = runtime_archetype_scene(archetype).filter(|scene| {
        asset_server.is_some() && converted_asset_exists(asset_root, &scene.asset_path)
    });
    let converted_animation = scene
        .as_ref()
        .and_then(|_| converted_animation_spec(archetype, presentation));
    let native_animation = converted_animation
        .is_none()
        .then(|| {
            scene
                .as_ref()
                .and_then(|scene| native_animation_request(archetype, scene, presentation))
        })
        .flatten();
    let base_scale = if scene.is_some() {
        Vec3::splat(config.world.cell_size / 2.0)
    } else {
        Vec3::new(
            config.world.cell_size * 0.3,
            config.world.cell_size * 0.55,
            config.world.cell_size * 0.3,
        )
    };
    let visual_height = if scene.is_some() {
        world_position.y
    } else {
        world_position.y + base_scale.y * 0.5
    };
    let mut entity = commands.spawn((
        WorldEntity,
        GridLocation(position),
        Agent {
            id: id.clone(),
            kind: ActorKind::Enemy,
            archetype: archetype_id,
            goal: AgentGoal::Wander,
            spawn: position,
            origin: position,
            navigation_position: placement_to_navigation_centre(position),
            path: Vec::new(),
            path_index: 0,
            target: deterministic_wander_target(world, id, position),
            action_cooldown_seconds: 0.0,
            action_started: false,
            repath_remaining_seconds: initial_enemy_repath_delay(config, id),
            health_regen_accumulator: 0.0,
            wander_sequence: 0,
            previous_wander_origin: None,
        },
        AgentLocomotion::default(),
        AgentAnimation {
            base_scale,
            native: converted_animation.is_some() || native_animation.is_some(),
            ..default()
        },
        Transform::from_xyz(world_position.x, visual_height, world_position.z)
            .with_scale(base_scale),
    ));
    if let Some(scene) = scene {
        entity.insert(WorldAssetRoot(
            asset_server
                .expect("asset server checked above")
                .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
        ));
        if let Some(converted_animation) = converted_animation {
            entity.insert(converted_animation);
        } else if let Some(native_animation) = native_animation {
            entity.insert(native_animation);
        }
        if let Some(material) = prefab_material_spec(archetype, &scene, presentation, render) {
            entity.insert(material);
        }
    } else {
        entity.insert((
            Mesh3d(render.actor_lod.clone()),
            MeshMaterial3d(render.enemy_idle.clone()),
        ));
    }
}

pub(crate) fn runtime_building_id(simulation: &WorldSimulation) -> StableId {
    for sequence in constructed_building_count(simulation)..usize::MAX {
        let candidate = StableId::new(format!("building:runtime_{sequence:08}"))
            .expect("runtime building IDs are valid");
        if !simulation.buildings.contains_key(&candidate) {
            return candidate;
        }
    }
    unreachable!("runtime building identifier space exhausted")
}

pub(crate) fn spawn_runtime_building(
    commands: &mut Commands,
    config: &GameConfig,
    world: &GeneratedWorld,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    render: &RenderAssets,
    building: &BuildingState,
    definition: &BuildingDef,
    archetype: &ArchetypeDef,
    position: GridPos,
    footprint: [u16; 2],
    age: u8,
) {
    let occupied_footprint = rotated_footprint(footprint, building.rotation_quarter_turns);
    let centre = GridPos {
        x: position.x + occupied_footprint[0] / 2,
        z: position.z + occupied_footprint[1] / 2,
    };
    let world_position = grid_to_world_on_surface(centre, config, world);
    let rotation = quarter_turn_rotation(building.rotation_quarter_turns);
    let mut entity = commands.spawn((
        WorldEntity,
        RuntimeBuilding {
            id: building.id.clone(),
        },
        GridLocation(position),
        Transform::from_translation(world_position).with_rotation(rotation),
    ));
    if definition.projectile_shooter.is_some() {
        entity.insert(TowerShooter {
            cooldown_seconds: definition
                .projectile_shooter
                .as_ref()
                .map_or(0.0, |shooter| milli_units_as_f32(shooter.fire_milliseconds)),
        });
    }
    if archetype.source_path == "native://buildings/path" {
        return;
    }
    if archetype.source_path == "native://buildings/streetlight" {
        let pole_height = config.world.cell_size * 1.55;
        let pole_width = config.world.cell_size * 0.11;
        let lamp_size = config.world.cell_size * 0.22;
        entity.insert((
            BuildingPresentation {
                base_translation: world_position,
                base_scale: Vec3::ONE,
                base_height_offset: 0.0,
                applied_stage: u8::MAX,
                applied_level: u16::MAX,
                applied_age: age,
                applied_scene: None,
            },
            BuildingDamageEmitter::default(),
        ));
        entity.with_children(|root| {
            root.spawn((
                Name::new("Streetlight wooden pole"),
                Mesh3d(render.cube.clone()),
                MeshMaterial3d(render.building.clone()),
                Transform::from_xyz(0.0, pole_height * 0.5, 0.0).with_scale(Vec3::new(
                    pole_width,
                    pole_height,
                    pole_width,
                )),
            ));
            root.spawn((
                Name::new("Streetlight lamp"),
                Mesh3d(render.cube.clone()),
                MeshMaterial3d(render.streetlight_lamp.clone()),
                Transform::from_xyz(0.0, pole_height + lamp_size * 0.35, 0.0)
                    .with_scale(Vec3::new(lamp_size, lamp_size * 0.65, lamp_size)),
            ));
        });
        return;
    }
    if let Some(scene) = archetype_scene_for_age(archetype, age).filter(|scene| {
        asset_server.is_some() && converted_asset_exists(asset_root, &scene.asset_path)
    }) {
        let base_scale = Vec3::splat(config.world.cell_size / 2.0);
        entity.insert((
            WorldAssetRoot(
                asset_server
                    .expect("asset server checked above")
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            BuildingPresentation {
                base_translation: world_position,
                base_scale,
                base_height_offset: 0.0,
                applied_stage: u8::MAX,
                applied_level: u16::MAX,
                applied_age: age,
                applied_scene: Some(scene.asset_path.clone()),
            },
            BuildingDamageEmitter::default(),
            Transform::from_translation(world_position)
                .with_rotation(rotation)
                .with_scale(base_scale),
        ));
        if let Some(material) =
            building_prefab_material_spec(archetype, scene, presentation, render)
        {
            entity.insert(material);
        }
    } else {
        let size = Vec3::new(
            f32::from(footprint[0]) * config.world.cell_size * 0.88,
            config.world.cell_size * 1.25,
            f32::from(footprint[1]) * config.world.cell_size * 0.88,
        );
        let base_translation = world_position + Vec3::Y * size.y * 0.5;
        entity.insert((
            BuildingPresentation {
                base_translation,
                base_scale: size,
                base_height_offset: size.y * 0.5,
                applied_stage: u8::MAX,
                applied_level: u16::MAX,
                applied_age: age,
                applied_scene: None,
            },
            BuildingDamageEmitter::default(),
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(if building.complete {
                render.building.clone()
            } else {
                render.construction.clone()
            }),
            Transform::from_translation(base_translation)
                .with_rotation(rotation)
                .with_scale(size),
        ));
    }
}

pub(crate) fn require_staff(pending: &PendingChatCommand) -> Result<(), String> {
    if pending.is_broadcaster || pending.is_moderator {
        Ok(())
    } else {
        Err("this command requires broadcaster or moderator permission".to_owned())
    }
}

pub(crate) fn require_game_master(
    config: &GameConfig,
    pending: &PendingChatCommand,
) -> Result<(), String> {
    if pending.origin == CommandOrigin::LocalDebug {
        return Ok(());
    }
    let raw_user_id = pending
        .actor_id
        .as_str()
        .strip_prefix("twitch:")
        .unwrap_or_else(|| pending.actor_id.as_str());
    if config.twitch.game_master_ids.contains(raw_user_id) {
        Ok(())
    } else {
        Err("this command requires an explicitly configured game-master Twitch user ID".to_owned())
    }
}

pub(crate) fn pending_stream_user_type(
    config: &GameConfig,
    pending: &PendingChatCommand,
) -> StreamUserType {
    let raw_user_id = pending
        .actor_id
        .as_str()
        .strip_prefix("twitch:")
        .unwrap_or_else(|| pending.actor_id.as_str());
    if pending.origin == CommandOrigin::LocalDebug
        || config.twitch.game_master_ids.contains(raw_user_id)
    {
        StreamUserType::GameMaster
    } else if pending.is_broadcaster {
        StreamUserType::Broadcaster
    } else if pending.is_moderator {
        StreamUserType::Moderator
    } else if pending.is_subscriber {
        StreamUserType::Subscriber
    } else {
        StreamUserType::Normal
    }
}

pub(crate) fn require_ruler_or_staff(
    simulation: &WorldSimulation,
    pending: &PendingChatCommand,
) -> Result<(), String> {
    if simulation.is_ruler(&pending.actor_id) {
        Ok(())
    } else {
        require_staff(pending)
            .map_err(|_| "this command is restricted to the Ruler or staff".to_owned())
    }
}

pub(crate) fn require_world_view_operator(
    simulation: &WorldSimulation,
    pending: &PendingChatCommand,
    user_type: StreamUserType,
) -> Result<(), String> {
    if user_type == StreamUserType::GameMaster {
        Ok(())
    } else {
        require_ruler_or_staff(simulation, pending)
    }
}

pub(crate) fn resolve_ruler_vote_option(
    simulation: &WorldSimulation,
    requested: &StableId,
) -> Option<StableId> {
    let vote = simulation.ruler_vote.as_ref()?;
    if vote.kind == RulerVoteKind::KeepRuler {
        return matches!(requested.as_str(), "yes" | "no").then(|| requested.clone());
    }
    if simulation.actors.contains_key(requested) {
        return Some(requested.clone());
    }
    let prefixed = prefixed_id(requested, "twitch:");
    if let Some(actor) = prefixed.filter(|actor| simulation.actors.contains_key(actor)) {
        return Some(actor);
    }
    let normalized = requested.as_str().replace('_', " ");
    simulation
        .actors
        .values()
        .filter(|actor| actor.role.as_str() != "role:enemy" && actor.alive)
        .find(|actor| {
            actor
                .login_name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(requested.as_str()))
                || actor
                    .display_name
                    .as_deref()
                    .is_some_and(|name| name.replace('_', " ").eq_ignore_ascii_case(&normalized))
        })
        .map(|actor| actor.id.clone())
}

pub(crate) fn resolve_community_vote_option(requested: &StableId) -> Option<bool> {
    match requested.as_str() {
        "yes" | "y" | "1" => Some(true),
        "no" | "n" | "2" => Some(false),
        _ => None,
    }
}

pub(crate) fn recruited_actor_ids(simulation: &WorldSimulation) -> Vec<StableId> {
    simulation
        .actors
        .keys()
        .filter(|id| is_recruited_actor_id(id) && !is_guardhouse_defender_id(id))
        .cloned()
        .collect()
}

pub(crate) fn capacity_recruited_actor_ids(simulation: &WorldSimulation) -> Vec<StableId> {
    recruited_actor_ids(simulation)
        .into_iter()
        .filter(|id| !is_guardhouse_defender_id(id))
        .collect()
}

pub(crate) fn is_guardhouse_defender_id(id: &StableId) -> bool {
    id.as_str().starts_with("npc:guardhouse_")
}

pub(crate) fn guardhouse_defender_id(building: &StableId) -> StableId {
    StableId::new(format!("npc:guardhouse_{}", building.as_str()))
        .expect("runtime building IDs form valid guardhouse defender IDs")
}

pub(crate) fn guardhouse_for_defender<'a>(
    simulation: &'a WorldSimulation,
    actor: &StableId,
) -> Option<&'a BuildingState> {
    let building = actor.as_str().strip_prefix("npc:guardhouse_")?;
    let building = StableId::new(building).ok()?;
    simulation
        .buildings
        .get(&building)
        .filter(|building| building.archetype.as_str() == "archetype:building:guardhouse")
}

pub(crate) fn is_recruited_actor_id(id: &StableId) -> bool {
    id.as_str().starts_with("npc:recruit_")
        || id.as_str().starts_with("npc:starting_")
        || id.as_str().starts_with("npc:guardhouse_")
}

pub(crate) fn is_level_capped_npc_id(id: &StableId) -> bool {
    is_recruited_actor_id(id) || id.as_str().starts_with("actor:viewer_")
}

pub(crate) fn actor_role_level_cap(content: &ContentCatalog, id: &StableId) -> u16 {
    if is_level_capped_npc_id(id) {
        content.progression.maximum_recruit_role_level
    } else {
        content.progression.maximum_role_level
    }
}

pub(crate) fn normalize_recruit_role_progression(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
) -> usize {
    let maximum_level = content.progression.maximum_recruit_role_level;
    let mut adjusted = 0;
    for actor in simulation
        .actors
        .values_mut()
        .filter(|actor| is_level_capped_npc_id(&actor.id))
    {
        for progress in actor.role_progression.values_mut() {
            if progress.level > maximum_level
                || (progress.level == maximum_level && progress.experience > 0)
            {
                progress.level = maximum_level;
                progress.experience = 0;
                adjusted += 1;
            }
        }
    }
    adjusted
}

pub(crate) fn enforce_recruit_role_level_cap(
    content: Res<RuntimeContent>,
    mut simulation: ResMut<SimulationRuntime>,
) {
    let adjusted = normalize_recruit_role_progression(&content.0, &mut simulation.0);
    if adjusted > 0 {
        warn!(
            adjusted,
            maximum_level = content.0.progression.maximum_recruit_role_level,
            "clamped recruit role progression"
        );
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn sync_guardhouse_defenders(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    world: Res<WorldRuntime>,
    render: Res<RenderAssets>,
    mut simulation: ResMut<SimulationRuntime>,
    agents: Query<(Entity, &Agent)>,
) {
    let guardhouses = simulation
        .0
        .buildings
        .values()
        .filter(|building| {
            building.complete
                && building.health > 0
                && building.archetype.as_str() == "archetype:building:guardhouse"
        })
        .map(|building| (building.id.clone(), building.position))
        .collect::<Vec<_>>();
    let live_ids = guardhouses
        .iter()
        .map(|(building, _)| guardhouse_defender_id(building))
        .collect::<BTreeSet<_>>();
    let orphaned = simulation
        .0
        .actors
        .keys()
        .filter(|id| is_guardhouse_defender_id(id) && !live_ids.contains(*id))
        .cloned()
        .collect::<Vec<_>>();
    for actor in orphaned {
        simulation.0.actors.remove(&actor);
        if let Some((entity, _)) = agents.iter().find(|(_, agent)| agent.id == actor) {
            commands.entity(entity).try_despawn();
        }
    }

    let defender = StableId::new("role:defender").expect("static defender role ID");
    let actor_archetype =
        archetype_id_by_source(&content.0, ArchetypeKind::Player, "Player_Character.prefab")
            .unwrap_or_else(|| {
                StableId::new("archetype:viewer").expect("static viewer archetype ID")
            });
    let base_scale = Vec3::new(
        config.0.world.cell_size * 0.3,
        config.0.world.cell_size * 0.55,
        config.0.world.cell_size * 0.3,
    );
    for (guardhouse, origin) in guardhouses {
        let actor_id = guardhouse_defender_id(&guardhouse);
        if !simulation.0.actors.contains_key(&actor_id) {
            let position = nearest_walkable(&world.generated, origin).unwrap_or(origin);
            if !simulation.0.join_player(actor_id.clone(), position) {
                continue;
            }
        }
        if simulation
            .0
            .actors
            .get(&actor_id)
            .is_some_and(|actor| actor.role != defender)
        {
            let _ = simulation.0.assign_role(&actor_id, defender.clone());
        }
        let Some(actor) = simulation.0.actors.get_mut(&actor_id) else {
            continue;
        };
        actor.station = Some(guardhouse.clone());
        actor.archetype = Some(actor_archetype.clone());
        actor.display_name = Some("Guardhouse Defender".to_owned());
        actor.login_name = None;
        if agents.iter().any(|(_, agent)| agent.id == actor_id) {
            continue;
        }
        let position = actor.position;
        let target = deterministic_wander_target(&world.generated, &actor_id, position);
        let world_position = grid_to_world_on_surface(position, &config.0, &world.generated);
        commands.spawn((
            WorldEntity,
            GridLocation(position),
            Agent {
                id: actor_id,
                kind: ActorKind::Player,
                archetype: actor_archetype.clone(),
                goal: AgentGoal::Wander,
                spawn: position,
                origin: position,
                navigation_position: placement_to_navigation_centre(position),
                path: Vec::new(),
                path_index: 0,
                target,
                action_cooldown_seconds: 0.0,
                action_started: false,
                repath_remaining_seconds: 0.0,
                health_regen_accumulator: 0.0,
                wander_sequence: 0,
                previous_wander_origin: None,
            },
            AgentLocomotion::default(),
            AgentAnimation {
                base_scale,
                ..default()
            },
            Mesh3d(render.actor_lod.clone()),
            MeshMaterial3d(actor_material(&render, &ActorKind::Player, false)),
            Transform::from_translation(world_position + Vec3::Y * base_scale.y * 0.5)
                .with_scale(base_scale),
        ));
    }
}

pub(crate) fn spawn_numbered_world_labels(commands: &mut Commands, targets: &[StableId]) {
    for (index, target) in targets.iter().enumerate() {
        commands.spawn((
            WorldEntity,
            TemporaryWorldLabel {
                target: target.clone(),
                remaining_seconds: UNITY_NUMBERED_LABEL_SECONDS,
            },
            Text::new((index + 1).to_string()),
            TextFont {
                font_size: FontSize::Px(22.0),
                ..default()
            },
            TextLayout::justify(Justify::Center),
            TextColor(Color::srgb(0.91, 0.89, 0.81)),
            GlobalZIndex(19),
            Node {
                position_type: PositionType::Absolute,
                width: px(40),
                ..default()
            },
        ));
    }
}

pub(crate) fn recruit_id(simulation: &WorldSimulation, index: u16) -> Option<StableId> {
    capacity_recruited_actor_ids(simulation)
        .get(usize::from(index.saturating_sub(1)))
        .cloned()
}

pub(crate) fn role_capacity(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    role: &StableId,
) -> Option<u32> {
    let definition = content.roles.get(role)?;
    if !definition.has_user_limit {
        return None;
    }
    Some(
        simulation
            .buildings
            .values()
            .filter(|state| state.complete)
            .filter_map(|state| {
                building_def_for_archetype(content, &state.archetype)
                    .map(|definition| (definition, state.level))
            })
            .flat_map(|(building, level)| {
                building
                    .role_slots
                    .iter()
                    .filter(move |slots| slots.role == *role)
                    .map(move |slots| {
                        u32::from(slots.base_amount).saturating_add(
                            u32::from(slots.increment_amount)
                                .saturating_mul(u32::from(level.saturating_sub(1))),
                        )
                    })
            })
            .fold(u32::from(definition.base_max_users), u32::saturating_add),
    )
}

pub(crate) fn reassign_roles_without_prerequisite_buildings(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
) -> Vec<StableId> {
    let defender = StableId::new("role:defender").expect("static defender role ID");
    let affected = simulation
        .actors
        .values()
        .filter(|actor| {
            !matches!(
                actor.role.as_str(),
                "role:defender" | "role:enemy" | "role:ruler"
            ) && !is_guardhouse_defender_id(&actor.id)
                && role_capacity(content, simulation, &actor.role) == Some(0)
        })
        .map(|actor| actor.id.clone())
        .collect::<Vec<_>>();
    for actor in &affected {
        let _ = simulation.assign_role(actor, defender.clone());
    }
    affected
}

pub(crate) fn role_is_available(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    role: &StableId,
    excluding: Option<&StableId>,
) -> bool {
    if !simulation.role_limits_enabled {
        return true;
    }
    role_capacity(content, simulation, role).is_none_or(|capacity| {
        let used = simulation
            .actors
            .values()
            .filter(|actor| {
                actor.role == *role
                    && excluding != Some(&actor.id)
                    && !is_guardhouse_defender_id(&actor.id)
            })
            .count();
        used < usize::try_from(capacity).unwrap_or(usize::MAX)
    })
}

pub(crate) fn resolve_player_id(
    simulation: &WorldSimulation,
    requested: &StableId,
) -> Option<StableId> {
    if simulation.actors.contains_key(requested) {
        return Some(requested.clone());
    }
    let prefixed = prefixed_id(requested, "twitch:");
    if let Some(actor) = prefixed.filter(|actor| simulation.actors.contains_key(actor)) {
        return Some(actor);
    }
    let normalized = requested.as_str().replace('_', " ");
    simulation
        .actors
        .values()
        .find(|actor| {
            actor
                .login_name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(requested.as_str()))
                || actor
                    .display_name
                    .as_deref()
                    .is_some_and(|name| name.replace('_', " ").eq_ignore_ascii_case(&normalized))
        })
        .map(|actor| actor.id.clone())
}

pub(crate) fn resolve_auto_camera_follow_target(
    simulation: &WorldSimulation,
    caller: &StableId,
    requested: Option<&StableId>,
) -> Result<StableId, String> {
    let target = requested.map_or_else(
        || Some(caller.clone()),
        |requested| resolve_player_id(simulation, requested),
    );
    let target = target.ok_or_else(|| {
        format!(
            "unknown player {}",
            requested.map_or("me", StableId::as_str)
        )
    })?;
    let actor = simulation
        .actors
        .get(&target)
        .filter(|actor| is_stream_player_actor(&actor.id))
        .ok_or_else(|| "the follow camera can target only player citizens".to_owned())?;
    if !actor.alive {
        return Err(format!("{} is not currently alive", actor.id));
    }
    Ok(target)
}

pub(crate) fn auto_camera_is_managed(controller: &TownCameraControllerRuntime) -> bool {
    controller.seconds_since_acknowledgement >= AUTO_CAMERA_IDLE_SECONDS
        && controller.auto_shot != AutoCameraShot::Inactive
}

pub(crate) fn command_interrupts_auto_camera(command: &ChatCommand) -> bool {
    !matches!(
        command,
        ChatCommand::Follow(_) | ChatCommand::FocusBuilding { .. } | ChatCommand::Vote(_)
    )
}

pub(crate) fn command_refreshes_building_focus(command: &ChatCommand) -> bool {
    matches!(
        command,
        ChatCommand::Build(_)
            | ChatCommand::MoveBuilding(_)
            | ChatCommand::CenterBuilding
            | ChatCommand::BeginBuildingLine
            | ChatCommand::EndBuildingLine
            | ChatCommand::ConfirmBuilding
    )
}

pub(crate) fn command_refreshes_world_diagnostic(command: &ChatCommand) -> bool {
    matches!(
        command,
        ChatCommand::Build(_)
            | ChatCommand::MoveBuilding(_)
            | ChatCommand::CenterBuilding
            | ChatCommand::BeginBuildingLine
            | ChatCommand::EndBuildingLine
            | ChatCommand::ConfirmBuilding
            | ChatCommand::CancelBuilding
            | ChatCommand::Camera(_)
            | ChatCommand::ResetCamera
            | ChatCommand::Follow(_)
            | ChatCommand::FocusBuilding { .. }
    )
}

pub(crate) fn is_world_diagnostic_timer_operator(
    simulation: &WorldSimulation,
    actor: &StableId,
    user_type: StreamUserType,
) -> bool {
    simulation.is_ruler(actor)
        || matches!(
            user_type,
            StreamUserType::GameMaster | StreamUserType::Broadcaster
        )
}

pub(crate) fn item_info(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    requested: &StableId,
    instance: Option<u16>,
) -> Result<String, String> {
    if instance.is_none()
        && let Some(player) = simulation.actors.values().find(|actor| {
            is_stream_player_actor(&actor.id)
                && (actor.id == *requested
                    || actor
                        .login_name
                        .as_deref()
                        .is_some_and(|name| name.eq_ignore_ascii_case(requested.as_str()))
                    || actor.display_name.as_deref().is_some_and(|name| {
                        name.replace('_', " ")
                            .eq_ignore_ascii_case(&requested.as_str().replace('_', " "))
                    }))
        })
    {
        let levels = content
            .roles
            .iter()
            .filter(|(id, _)| !matches!(id.as_str(), "role:ruler" | "role:enemy"))
            .map(|(id, role)| {
                let level = player
                    .role_progression
                    .get(id)
                    .copied()
                    .unwrap_or_default()
                    .level;
                format!("{} {level}", role.display_name)
            })
            .collect::<Vec<_>>()
            .join(", ");
        let name = player.display_name.as_deref().unwrap_or(player.id.as_str());
        return Ok(format!(
            "{name}: current {}, health {}/{}, role levels — {levels}",
            player.role, player.health, player.max_health
        ));
    }
    if let Some(role_id) = prefixed_id(requested, "role:")
        && let Some(role) = content.roles.get(&role_id)
    {
        let capacity = role_capacity(content, simulation, &role_id)
            .map_or_else(|| "unlimited".to_owned(), |value| value.to_string());
        let assigned = simulation
            .actors
            .values()
            .filter(|actor| actor.role == role_id && !is_guardhouse_defender_id(&actor.id))
            .count();
        return Ok(format!(
            "{}: action {}, health {}, carry {}, slots {assigned}/{capacity}",
            role.display_name, role.base_action_amount, role.base_health, role.base_carry_capacity
        ));
    }
    if let Ok(building_id) = building_definition_id(content, requested)
        && let Some(building) = content.buildings.get(&building_id)
    {
        let instances = building_instance_ids(content, simulation, &building_id);
        if let Some(instance) = instance {
            let state = instances
                .get(usize::from(instance.saturating_sub(1)))
                .and_then(|runtime_id| simulation.buildings.get(runtime_id))
                .ok_or_else(|| {
                    format!("{} BID {instance} does not exist", building.display_name)
                })?;
            let max_health = building_max_health(content, state);
            let max_level = maximum_building_level(content, simulation, &building_id);
            let status = if state.complete {
                "complete"
            } else {
                "under construction"
            };
            let next_upgrade = if !building.can_level || state.level >= max_level {
                "maximum level".to_owned()
            } else {
                let cost = building_upgrade_cost(
                    content,
                    simulation,
                    &building_id,
                    building,
                    state.level,
                    state.level.saturating_add(1),
                )
                .into_iter()
                .map(|(resource, amount)| format!("{resource}={amount}"))
                .collect::<Vec<_>>()
                .join(", ");
                format!("next upgrade {cost}")
            };
            return Ok(format!(
                "{} BID {instance}: level {}/{max_level}, health {}/{max_health}, {status}, at {},{}, {next_upgrade}",
                building.display_name,
                state.level,
                state.health,
                state.position.x,
                state.position.z
            ));
        }
        let count = instances.len();
        let cost = building
            .cost
            .iter()
            .map(|(resource, amount)| format!("{resource}={amount}"))
            .collect::<Vec<_>>()
            .join(", ");
        return Ok(format!(
            "{}: count {count}, footprint {}x{}, cost {cost}",
            building.display_name, building.footprint[0], building.footprint[1]
        ));
    }
    if let Some(technology_id) = resolve_technology_id(content, requested) {
        let technology = &content.technology.nodes[&technology_id];
        let status = if simulation.unlocked_technology.contains(&technology_id) {
            "unlocked"
        } else if technology.unavailable {
            "unavailable"
        } else {
            "locked"
        };
        return Ok(format!(
            "{} ({status}): {}",
            technology.display_name, technology.description
        ));
    }
    if let Some(resource_id) = prefixed_id(requested, "resource:")
        && let Some(amount) = simulation.town_resources.get(&resource_id)
    {
        return Ok(format!("{resource_id}: town amount {amount}"));
    }
    if let Some(player_id) = resolve_player_id(simulation, requested) {
        let actor = &simulation.actors[&player_id];
        return Ok(format!(
            "{player_id}: {}, health {}/{}, at {},{}",
            actor.role, actor.health, actor.max_health, actor.position.x, actor.position.z
        ));
    }
    Err(format!("unknown item {requested}"))
}

pub(crate) fn compatible_station_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    config: &GameConfig,
    actor: &ActorState,
) -> Vec<StableId> {
    let Some(role) = content.roles.get(&actor.role) else {
        return Vec::new();
    };
    let town_hall = StableId::new("building:townhall").expect("static ID");
    let mut stations: Vec<_> = std::iter::once(town_hall)
        .chain(
            simulation
                .buildings
                .keys()
                .filter(|id| id.as_str() != "building:townhall")
                .cloned(),
        )
        .filter(|id| {
            station_candidate(content, simulation, config, id).is_some_and(|station| {
                station_matches_role(station.definition, role)
                    && station_supports_role_targets(station.definition, role)
            })
        })
        .collect();
    stations.sort();
    stations
}

#[cfg(test)]
pub(crate) fn compatible_target_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    actor: &ActorState,
) -> Vec<StableId> {
    let station_targets = immediate_station_target_runtime(content, simulation, world, config);
    compatible_target_ids_with_station_runtime(
        content,
        simulation,
        world,
        config,
        &station_targets,
        actor,
    )
}

pub(crate) fn compatible_target_ids_with_station_runtime(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    station_targets: &StationTargetRuntime,
    actor: &ActorState,
) -> Vec<StableId> {
    let Some(role) = content.roles.get(&actor.role) else {
        return Vec::new();
    };
    let fallback_station =
        best_station_id(content, simulation, config, &actor.role, actor.position);
    let Some(station) = assigned_station(content, simulation, config, actor).or_else(|| {
        fallback_station
            .as_ref()
            .and_then(|id| station_candidate(content, simulation, config, id))
    }) else {
        return Vec::new();
    };
    let mut kinds = role.target_kinds.iter().collect::<Vec<_>>();
    kinds.sort();
    let mut targets = Vec::new();
    for kind in kinds {
        if !station.definition.targets_all && !station.definition.target_kinds.contains(kind) {
            continue;
        }
        let cached = cached_station_targets(station_targets, station, kind)
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut candidates = station_target_candidates(content, simulation, world, actor, kind);
        candidates.retain(|(_, id)| cached.contains(id));
        candidates.sort_by_key(|(_, id)| id.clone());
        targets.extend(candidates.into_iter().map(|(_, id)| id));
    }
    targets
}

pub(crate) fn station_target_candidates(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    actor: &ActorState,
    kind: &StableId,
) -> Vec<(GridPos, StableId)> {
    if matches!(
        kind.as_str(),
        "target:tree" | "target:ore" | "target:bush" | "target:fish"
    ) {
        // Generated worlds contain thousands of resources. Filter their source
        // records in one pass instead of cloning a catalog and then linearly
        // rescanning the world for every candidate.
        return world
            .resources
            .iter()
            .filter(|resource| {
                resource.amount > 0
                    && resource.target_kind == *kind
                    && actor_accepts_resource(content, actor, resource)
            })
            .map(|resource| (resource.position, resource.id.clone()))
            .collect();
    }

    let mut candidates = station_target_catalog_candidates(content, simulation, world, kind);
    if matches!(
        kind.as_str(),
        "target:player" | "target:injured_player" | "target:dead_player"
    ) {
        candidates.retain(|(_, id)| *id != actor.id);
    }
    candidates
}

pub(crate) fn station_target_catalog_candidates(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    kind: &StableId,
) -> Vec<(GridPos, StableId)> {
    match kind.as_str() {
        "target:tree" | "target:ore" | "target:bush" | "target:fish" => world
            .resources
            .iter()
            .filter(|resource| resource.amount > 0 && resource.target_kind == *kind)
            .map(|resource| (resource.position, resource.id.clone()))
            .collect(),
        "target:farm" => simulation
            .buildings
            .values()
            .filter(|building| is_farm_resource_building(content, building))
            .map(|building| (building_visual_grid(content, building), building.id.clone()))
            .collect(),
        "target:enemy" | "target:boss" => simulation
            .actors
            .values()
            .filter(|target| target.alive && target.role.as_str() == "role:enemy")
            .filter(|target| {
                let is_boss = actor_archetype(content, target)
                    .and_then(|archetype| archetype.enemy.as_ref())
                    .is_some_and(|enemy| enemy.enemy_type.as_str().ends_with("_boss"));
                is_boss == (kind.as_str() == "target:boss")
            })
            .map(|target| (target.position, target.id.clone()))
            .collect(),
        "target:player" => simulation
            .actors
            .values()
            .filter(|target| {
                target.alive
                    && target.role.as_str() != "role:enemy"
                    && target.health == target.max_health
            })
            .map(|target| (target.position, target.id.clone()))
            .collect(),
        "target:injured_player" => simulation
            .actors
            .values()
            .filter(|target| {
                target.alive
                    && target.role.as_str() != "role:enemy"
                    && target.health < target.max_health
            })
            .map(|target| (target.position, target.id.clone()))
            .collect(),
        "target:dead_player" => simulation
            .actors
            .values()
            .filter(|target| !target.alive && target.role.as_str() != "role:enemy")
            .map(|target| (target.position, target.id.clone()))
            .collect(),
        "target:construction" => simulation
            .buildings
            .values()
            .filter(|building| !building.complete && building.health > 0)
            .map(|building| (building_visual_grid(content, building), building.id.clone()))
            .collect(),
        "target:damaged_building" => simulation
            .buildings
            .values()
            .filter(|building| {
                building.complete
                    && building.health > 0
                    && building.health < building_max_health(content, building)
            })
            .map(|building| (building_visual_grid(content, building), building.id.clone()))
            .collect(),
        "target:building" => simulation
            .buildings
            .values()
            .filter(|building| {
                building.complete
                    && building.health == building_max_health(content, building)
                    && !is_farm_resource_building(content, building)
            })
            .map(|building| (building_visual_grid(content, building), building.id.clone()))
            .collect(),
        _ => Vec::new(),
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn recruit_npcs(
    commands: &mut Commands,
    config: &GameConfig,
    content: &ContentCatalog,
    world: &GeneratedWorld,
    render: &RenderAssets,
    simulation: &mut WorldSimulation,
    role: &StableId,
    requested_amount: u16,
) -> Result<String, String> {
    let recruit_resource = StableId::new("resource:recruit").expect("static resource ID");
    let current = simulation
        .town_resources
        .get(&recruit_resource)
        .copied()
        .unwrap_or_default();
    let capacity = resource_storage_capacity(config, content, simulation, &recruit_resource);
    let available = capacity.saturating_sub(current);
    let amount = u32::from(requested_amount).min(available);
    if amount == 0 {
        return Err("the town has no recruit capacity".to_owned());
    }
    if !content.roles.contains_key(role) {
        return Err(format!("unknown role {role}"));
    }
    let current_in_role = simulation
        .actors
        .values()
        .filter(|actor| actor.role == *role && !is_guardhouse_defender_id(&actor.id))
        .count();
    let available_role_slots = role_capacity(content, simulation, role).map_or(usize::MAX, |max| {
        usize::try_from(max)
            .unwrap_or(usize::MAX)
            .saturating_sub(current_in_role)
    });
    let amount = amount.min(u32::try_from(available_role_slots).unwrap_or(u32::MAX));
    if amount == 0 {
        return Err(format!("the {role} role is full"));
    }
    let archetype =
        archetype_id_by_source(content, ArchetypeKind::Player, "Player_Character.prefab")
            .unwrap_or_else(|| StableId::new("archetype:viewer").expect("static ID"));
    let center = GridPos {
        x: world.navigation.width() / 2,
        z: world.navigation.height() / 2,
    };
    let base_scale = Vec3::new(
        config.world.cell_size * 0.3,
        config.world.cell_size * 0.55,
        config.world.cell_size * 0.3,
    );
    let mut spawned = 0_u32;
    for _ in 0..amount {
        let mut sequence = u64::try_from(simulation.actors.len()).unwrap_or(u64::MAX);
        let id = loop {
            let candidate =
                StableId::new(format!("npc:recruit_{sequence:08}")).expect("runtime recruit ID");
            if !simulation.actors.contains_key(&candidate) {
                break candidate;
            }
            sequence = sequence
                .checked_add(1)
                .ok_or_else(|| "runtime recruit identifier space exhausted".to_owned())?;
        };
        let desired = GridPos {
            x: center
                .x
                .saturating_add(u16::try_from(sequence % 9).unwrap_or_default()),
            z: center
                .z
                .saturating_add(u16::try_from((sequence / 9) % 9).unwrap_or_default()),
        };
        let position = nearest_walkable(world, desired)
            .or_else(|| nearest_walkable(world, center))
            .ok_or_else(|| "no walkable recruit spawn is available".to_owned())?;
        if !simulation.join_player(id.clone(), position) {
            continue;
        }
        simulation
            .assign_role(&id, role.clone())
            .map_err(|error| error.to_string())?;
        if let Some(actor) = simulation.actors.get_mut(&id) {
            actor.archetype = Some(archetype.clone());
            actor.display_name = Some(format!("Recruit {}", sequence + 1));
            actor.login_name = None;
        }
        let target = deterministic_wander_target(world, &id, position);
        let world_position = grid_to_world_on_surface(position, config, world);
        commands.spawn((
            WorldEntity,
            GridLocation(position),
            Agent {
                id,
                kind: ActorKind::Player,
                archetype: archetype.clone(),
                goal: AgentGoal::Wander,
                spawn: position,
                origin: position,
                navigation_position: placement_to_navigation_centre(position),
                path: Vec::new(),
                path_index: 0,
                target,
                action_cooldown_seconds: 0.0,
                action_started: false,
                repath_remaining_seconds: 0.0,
                health_regen_accumulator: 0.0,
                wander_sequence: 0,
                previous_wander_origin: None,
            },
            AgentLocomotion::default(),
            AgentAnimation {
                base_scale,
                ..default()
            },
            Mesh3d(render.actor_lod.clone()),
            MeshMaterial3d(actor_material(render, &ActorKind::Player, false)),
            Transform::from_translation(world_position + Vec3::Y * base_scale.y * 0.5)
                .with_scale(base_scale),
        ));
        spawned = spawned.saturating_add(1);
    }
    *simulation
        .town_resources
        .entry(recruit_resource)
        .or_default() = current.saturating_add(spawned);
    Ok(format!("recruited {spawned} {role}"))
}

pub(crate) fn send_command_feedback(connection: &TwitchConnection, message: String) {
    if let Some(transport) = &connection.transport {
        let _ = transport.send(TwitchControl::SendBotMessage(message));
    }
}

pub(crate) fn unity_outbound_reply(
    command: &ChatCommand,
    succeeded: bool,
    message: &str,
    display_name: &str,
) -> Option<String> {
    if !succeeded {
        let message = message
            .strip_prefix("command rejected: ")
            .unwrap_or(message);
        return Some(if message.contains("join before") {
            format!("{display_name}: You need to create a character first with !join")
        } else {
            format!("{display_name}: {message}")
        });
    }
    match command {
        ChatCommand::Join if message == "welcome to Stream Town" => Some(format!(
            "{display_name} Welcome to the game, your character was successfully created!"
        )),
        ChatCommand::Join => Some(format!(
            "{display_name} Character already registered into the game!"
        )),
        ChatCommand::SelectRole(_) => Some(format!("{display_name}  Role switched successfully!")),
        ChatCommand::Role
        | ChatCommand::Health
        | ChatCommand::Level(_)
        | ChatCommand::Experience
        | ChatCommand::Pets
        | ChatCommand::Pet(None)
        | ChatCommand::RecruitCount
        | ChatCommand::RecruitInfo(_)
        | ChatCommand::RecruitRole { .. }
        | ChatCommand::DismissRecruit(_)
        | ChatCommand::Resign
        | ChatCommand::LevelBuilding { .. }
        | ChatCommand::LevelAll { .. }
        | ChatCommand::RemoveBuilding { .. } => Some(format!("{display_name} {message}")),
        ChatCommand::Station(Some(_)) => Some(format!("{display_name}: Station Switched!")),
        ChatCommand::Target(Some(_)) => Some(format!("{display_name}: Target Switched!")),
        ChatCommand::Customize { kind, .. } => Some(format!(
            "{display_name}: {}",
            match kind {
                CustomizationKind::Hair => "Hair Style Changed!",
                CustomizationKind::Eyes => "Eye Style Changed!",
                CustomizationKind::FacialHair => "Facial Hair Style Changed!",
                CustomizationKind::Body => "Body Type Changed!",
                CustomizationKind::HairColor => "Hair Color Changed!",
                CustomizationKind::EyeColor => "Eye Color Changed!",
            }
        )),
        ChatCommand::Pet(Some(_)) => Some(format!("{display_name} pet switched!")),
        ChatCommand::Buy { .. } | ChatCommand::Sell { .. } => {
            Some(format!("{display_name} : {message}"))
        }
        ChatCommand::Discord
        | ChatCommand::Help
        | ChatCommand::Roles
        | ChatCommand::TownStats
        | ChatCommand::Population
        | ChatCommand::Info { .. }
        | ChatCommand::ToggleBuildCosts
        | ChatCommand::ToggleRoleLimits => Some(message.to_owned()),
        // Unity performs these successfully without writing another chat line.
        ChatCommand::Build(_)
        | ChatCommand::MoveBuilding(_)
        | ChatCommand::CenterBuilding
        | ChatCommand::ConfirmBuilding
        | ChatCommand::CancelBuilding
        | ChatCommand::Unstuck
        | ChatCommand::Ping
        | ChatCommand::Camera(_)
        | ChatCommand::ResetCamera
        | ChatCommand::ModRole { .. }
        | ChatCommand::StartRulerVote
        | ChatCommand::Station(None)
        | ChatCommand::Target(None)
        | ChatCommand::RecruitIds
        | ChatCommand::AddResource { .. }
        | ChatCommand::KillPlayer(_)
        | ChatCommand::GameMasterRevive(_)
        | ChatCommand::GiveExperience { .. }
        | ChatCommand::GiveExperienceAll(_)
        | ChatCommand::LevelUpPlayer { .. }
        | ChatCommand::QueueEvent(_)
        | ChatCommand::GivePet { .. }
        | ChatCommand::StopEvent
        | ChatCommand::CompleteObjective
        | ChatCommand::RandomTechnology
        | ChatCommand::TechnologyVote
        | ChatCommand::GameEventAction
        | ChatCommand::UnlockAllTechnology
        | ChatCommand::UnlockAgeTwo
        | ChatCommand::TriggerEvent(_)
        | ChatCommand::Praise => None,
        _ => Some(format!("{display_name}: {message}")),
    }
}

pub(crate) fn twitch_pascal_case(value: &str) -> String {
    value
        .split(['_', '-', ' '])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut characters = part.chars();
            characters.next().map_or_else(String::new, |first| {
                first.to_uppercase().chain(characters).collect()
            })
        })
        .collect()
}

pub(crate) fn resource_cost_display_name(resource: &StableId) -> String {
    twitch_pascal_case(resource.as_str().trim_start_matches("resource:"))
}

pub(crate) fn ordered_resource_costs(cost: &BTreeMap<StableId, u32>) -> Vec<(&StableId, u32)> {
    let order = ["wood", "ore", "food", "gold", "recruit"];
    let mut resources = cost.iter().collect::<Vec<_>>();
    resources.sort_by_key(|(resource, _)| {
        order
            .iter()
            .position(|name| resource.as_str().trim_start_matches("resource:") == *name)
            .unwrap_or(order.len())
    });
    resources
        .into_iter()
        .map(|(resource, amount)| (resource, *amount))
        .collect()
}

pub(crate) fn format_resource_costs(cost: &BTreeMap<StableId, u32>) -> String {
    if cost.is_empty() {
        return "free".to_owned();
    }
    ordered_resource_costs(cost)
        .into_iter()
        .map(|(resource, amount)| format!("{} {amount}", resource_cost_display_name(resource)))
        .collect::<Vec<_>>()
        .join(" | ")
}

pub(crate) fn building_shortage_message(
    simulation: &WorldSimulation,
    display_name: &str,
    cost: &BTreeMap<StableId, u32>,
) -> Option<String> {
    let shortages = ordered_resource_costs(cost)
        .into_iter()
        .filter_map(|(resource, required)| {
            let available = simulation
                .town_resources
                .get(resource)
                .copied()
                .unwrap_or_default();
            (available < required).then(|| {
                format!(
                    "{} {} more (have {available}/{required})",
                    resource_cost_display_name(resource),
                    required - available,
                )
            })
        })
        .collect::<Vec<_>>();
    (!shortages.is_empty()).then(|| {
        format!(
            "cannot afford {display_name}; need {}",
            shortages.join(", ")
        )
    })
}

pub(crate) fn building_cost_summary(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    requested: &StableId,
) -> Result<String, String> {
    let building_id = building_definition_id(content, requested)?;
    let definition = &content.buildings[&building_id];
    let cost = building_construction_cost(content, simulation, &building_id, definition);
    Ok(format!(
        "{} build cost: {} | current maximum level: {}",
        definition.display_name,
        format_resource_costs(&cost),
        maximum_building_level(content, simulation, &building_id),
    ))
}

pub(crate) fn shift_grid_position(
    position: GridPos,
    actions: &[BuildingAction],
    world: &GeneratedWorld,
) -> (GridPos, i32) {
    shift_grid_position_with_bounds(
        position,
        actions,
        world.navigation.width(),
        world.navigation.height(),
    )
}

pub(crate) fn shift_grid_position_with_bounds(
    position: GridPos,
    actions: &[BuildingAction],
    width: u16,
    height: u16,
) -> (GridPos, i32) {
    let mut x = i32::from(position.x);
    let mut z = i32::from(position.z);
    let mut rotation = 0_i32;
    for action in actions {
        match action.direction {
            // The authored gameplay camera looks toward +X, so command names
            // describe the viewer's on-screen directions rather than raw grid
            // axes: screen up/down is +/-X and screen left/right is -/+Z.
            BuildingDirection::Up => x = x.saturating_add(action.amount),
            BuildingDirection::Down => x = x.saturating_sub(action.amount),
            BuildingDirection::Left => z = z.saturating_sub(action.amount),
            BuildingDirection::Right => z = z.saturating_add(action.amount),
            BuildingDirection::Rotate => rotation = rotation.saturating_add(action.amount),
        }
    }
    let max_x = i32::from(width.saturating_sub(1));
    let max_z = i32::from(height.saturating_sub(1));
    (
        GridPos {
            x: u16::try_from(x.clamp(0, max_x)).expect("grid x is clamped"),
            z: u16::try_from(z.clamp(0, max_z)).expect("grid z is clamped"),
        },
        rotation,
    )
}

pub(crate) fn move_path_placement(
    placement: &mut BuildingPlacement,
    actions: &[BuildingAction],
    world: &GeneratedWorld,
    fine_navigation: Option<&stream_town_domain::NavGrid>,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> i32 {
    let width = world
        .navigation
        .width()
        .saturating_mul(NAVIGATION_SUBDIVISIONS);
    let height = world
        .navigation
        .height()
        .saturating_mul(NAVIGATION_SUBDIVISIONS);
    let mut cursor = placement
        .navigation_position
        .unwrap_or_else(|| placement_to_navigation_centre(placement.position));
    let mut rotation = 0_i32;
    for action in actions {
        if action.direction == BuildingDirection::Rotate {
            rotation = rotation.saturating_add(action.amount);
            continue;
        }
        let step = BuildingAction {
            direction: action.direction,
            amount: action.amount.signum(),
        };
        for _ in 0..action.amount.unsigned_abs() {
            let next = shift_grid_position_with_bounds(cursor, &[step], width, height).0;
            if next == cursor {
                continue;
            }
            cursor = next;
        }
    }
    set_path_placement_cursor(placement, cursor, fine_navigation, content, simulation);
    rotation
}

pub(crate) fn set_path_placement_cursor(
    placement: &mut BuildingPlacement,
    cursor: GridPos,
    fine_navigation: Option<&stream_town_domain::NavGrid>,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) {
    placement.navigation_position = Some(cursor);
    placement.position = navigation_to_placement(cursor);
    if let Some(start) = placement.line_start {
        placement.path_cells = fine_navigation
            .and_then(|navigation| {
                routed_path_cells(navigation, content, simulation, start, cursor)
            })
            // Retain both endpoints so the preview clearly communicates a
            // failed route; path_route_is_complete keeps confirmation red.
            .unwrap_or_else(|| vec![start, cursor]);
    }
}

pub(crate) fn centre_building_placement_at_world_position(
    placement: &mut BuildingPlacement,
    world_position: Vec3,
    config: &GameConfig,
    fine_navigation: Option<&stream_town_domain::NavGrid>,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> Result<GridPos, String> {
    if is_path_building(&placement.building) {
        let cursor = world_to_navigation(world_position, config)
            .ok_or_else(|| "the centre ray landed outside the fine pathfinding grid".to_owned())?;
        set_path_placement_cursor(placement, cursor, fine_navigation, content, simulation);
        Ok(cursor)
    } else {
        let cursor = world_to_grid(world_position, config)
            .ok_or_else(|| "the centre ray landed outside the building grid".to_owned())?;
        placement.position = cursor;
        Ok(cursor)
    }
}

pub(crate) fn routed_path_cells(
    navigation: &stream_town_domain::NavGrid,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    start: GridPos,
    goal: GridPos,
) -> Option<Vec<GridPos>> {
    let gate_cells = completed_player_gate_navigation_cells(content, simulation);
    navigation
        .find_path_with_exceptions_and_costs(start, goal, &gate_cells, 10, 14, |position, cost| {
            path_construction_step_cost(position, cost)
        })
        .ok()
}

pub(crate) fn path_construction_step_cost(position: GridPos, base_cost: u32) -> u32 {
    let centre_distance = u32::from(position.x % NAVIGATION_SUBDIVISIONS).abs_diff(1)
        + u32::from(position.z % NAVIGATION_SUBDIVISIONS).abs_diff(1);
    base_cost.saturating_add(centre_distance.saturating_mul(PATH_CENTRE_THIRD_STEP_PENALTY))
}

pub(crate) fn building_definition_id(
    content: &ContentCatalog,
    requested: &StableId,
) -> Result<StableId, String> {
    if let Some(id) =
        prefixed_id(requested, "building:").filter(|id| content.buildings.contains_key(id))
    {
        return Ok(id);
    }
    let normalized = normalize_building_command_name(requested.as_str());
    content
        .buildings
        .iter()
        .find(|(id, definition)| {
            normalize_building_command_name(id.as_str().trim_start_matches("building:"))
                == normalized
                || normalize_building_command_name(&definition.display_name) == normalized
        })
        .map(|(id, _)| id.clone())
        .ok_or_else(|| format!("unknown building {requested}"))
}

pub(crate) fn building_placement_request(
    content: &ContentCatalog,
    requested: &StableId,
) -> Result<(StableId, bool), String> {
    let thick_path =
        normalize_building_command_name(requested.as_str().trim_start_matches("building:"))
            == "thickpath";
    if thick_path {
        Ok((
            StableId::new("building:path").expect("static path ID"),
            true,
        ))
    } else {
        building_definition_id(content, requested).map(|building| (building, false))
    }
}

pub(crate) fn normalize_building_command_name(value: &str) -> String {
    value
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect()
}

pub(crate) fn building_command_name(definition: &stream_town_domain::BuildingDef) -> String {
    twitch_pascal_case(&definition.display_name.replace([' ', '-', '_'], "_"))
}

pub(crate) fn numbered_building_instance_id(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    requested: &StableId,
    index: u16,
) -> Result<(StableId, StableId), String> {
    let building_id = building_definition_id(content, requested)?;
    let definition = &content.buildings[&building_id];
    let instances = building_instance_ids(content, simulation, &building_id);
    let runtime_id = instances
        .get(usize::from(index.saturating_sub(1)))
        .cloned()
        .ok_or_else(|| {
            format!(
                "{} BID {index} does not exist; use !bid {}",
                definition.display_name,
                building_command_name(definition)
            )
        })?;
    Ok((building_id, runtime_id))
}

pub(crate) fn building_instance_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building_id: &StableId,
) -> Vec<StableId> {
    let archetype = &content.buildings[building_id].archetype;
    let mut instances = simulation
        .buildings
        .values()
        .filter(|building| building.archetype == *archetype)
        .map(|building| building.id.clone())
        .collect::<Vec<_>>();
    instances.sort();
    instances
}

pub(crate) fn constructed_building_count(simulation: &WorldSimulation) -> usize {
    simulation
        .buildings
        .keys()
        .filter(|id| id.as_str() != "building:townhall")
        .count()
}

pub(crate) fn hud_building_count(content: &ContentCatalog, simulation: &WorldSimulation) -> usize {
    simulation
        .buildings
        .values()
        .filter(|state| {
            content
                .buildings
                .iter()
                .find(|(_, definition)| definition.archetype == state.archetype)
                .is_some_and(|(id, definition)| {
                    id.as_str() != "building:townhall"
                        && id.as_str() != "building:wall"
                        && id.as_str() != "building:path"
                        && definition.projectile_shooter.is_none()
                })
        })
        .count()
}

pub(crate) fn upgrade_building_instance(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    building_id: &StableId,
    runtime_id: &StableId,
    levels: u16,
) -> Result<u16, String> {
    let definition = &content.buildings[building_id];
    if !definition.can_level {
        return Err(format!("{} cannot be upgraded", definition.display_name));
    }
    let state = simulation
        .buildings
        .get(runtime_id)
        .ok_or_else(|| format!("building instance {runtime_id} does not exist"))?;
    let current_level = state.level;
    let archetype = state.archetype.clone();
    let max_level = maximum_building_level(content, simulation, building_id);
    let target_level = current_level.saturating_add(levels.max(1)).min(max_level);
    let cost = if simulation.building_costs_enabled {
        building_upgrade_cost(
            content,
            simulation,
            building_id,
            definition,
            current_level,
            target_level,
        )
    } else {
        BTreeMap::new()
    };
    let upgraded_max_health = building_max_health_at_level(content, &archetype, target_level);
    simulation
        .upgrade_building(
            runtime_id,
            max_level,
            target_level,
            upgraded_max_health,
            &cost,
        )
        .map_err(|error| error.to_string())
}

pub(crate) fn rotate_building_instance(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    runtime_id: &StableId,
    quarter_turns: i32,
) -> Result<(String, i32), String> {
    let state = simulation
        .buildings
        .get(runtime_id)
        .ok_or_else(|| format!("unknown building BID {runtime_id}"))?;
    let (definition_id, definition) = content
        .buildings
        .iter()
        .find(|(_, definition)| definition.archetype == state.archetype)
        .ok_or_else(|| format!("BID {runtime_id} has no building definition"))?;
    if definition.footprint[0] != definition.footprint[1] {
        return Err(format!(
            "{} has a non-square footprint and cannot be rotated after placement",
            definition.display_name
        ));
    }
    if matches!(definition_id.as_str(), "building:wall" | "building:gate") {
        return Err(format!(
            "{} orientation is controlled by its neighboring tiles",
            definition.display_name
        ));
    }
    let name = definition.display_name.clone();
    let state = simulation
        .buildings
        .get_mut(runtime_id)
        .expect("building was validated");
    state.rotation_quarter_turns = state.rotation_quarter_turns.saturating_add(quarter_turns);
    Ok((name, state.rotation_quarter_turns.rem_euclid(4) * 90))
}

pub(crate) fn sell_town_resource(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    resource: &StableId,
    amount: u32,
) -> Result<String, String> {
    simulation
        .sell_resource(resource, amount)
        .map(|(sold, gold)| {
            let _ = simulation.record_objective_event(
                &content.objectives,
                &ObjectiveEvent::ResourceGained {
                    resource: StableId::new("resource:gold").expect("static stable ID"),
                    amount: gold,
                },
            );
            let _ = simulation.record_objective_event(
                &content.objectives,
                &ObjectiveEvent::ResourceSold {
                    resource: resource.clone(),
                    amount: sold,
                },
            );
            format!("sold {sold} {resource} for {gold} gold")
        })
        .map_err(|error| error.to_string())
}

pub(crate) fn buy_town_resource(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    resource: StableId,
    amount: u32,
) -> Result<String, String> {
    let capacity = resource_storage_capacity(config, content, simulation, &resource);
    simulation
        .buy_resource(resource.clone(), amount, capacity)
        .map(|(bought, gold)| {
            let _ = simulation.record_objective_event(
                &content.objectives,
                &ObjectiveEvent::TradeGoldSpent { amount: gold },
            );
            let _ = simulation.record_objective_event(
                &content.objectives,
                &ObjectiveEvent::ResourceGained {
                    resource: resource.clone(),
                    amount: bought,
                },
            );
            let _ = simulation.record_objective_event(
                &content.objectives,
                &ObjectiveEvent::ResourceBought {
                    resource: resource.clone(),
                    amount: bought,
                },
            );
            format!("bought {bought} {resource} for {gold} gold")
        })
        .map_err(|error| error.to_string())
}
