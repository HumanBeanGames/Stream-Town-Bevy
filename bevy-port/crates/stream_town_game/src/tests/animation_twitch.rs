#[test]
fn root_exit_into_the_active_fallback_does_not_restart_its_loop() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let player =
        archetype_by_source(&content, ArchetypeKind::Player, "Player_Character.prefab").unwrap();
    let spec = converted_animation_spec(player, &presentation).unwrap();
    let controller = presentation.controllers.get(&spec.controller).unwrap();
    let mut runtime = AnimationControllerRuntime::in_state(controller, spec.state.clone()).unwrap();
    let playback = AnimationTransitionPlayback {
        duration: 0.1,
        fixed_duration: true,
        destination_offset: 0.0,
    };

    let resolved = resolve_animation_exit_fallback(
        &mut runtime,
        controller,
        &spec.state,
        Some(&stream_town_domain::AnimationTransitionOutcome::Exited),
        Some(playback),
    );

    assert_eq!(resolved, None);
    assert_eq!(runtime.current_state(), &spec.state);
}

#[test]
fn player_controller_prefers_skin_compatible_native_clips() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let player =
        archetype_by_source(&content, ArchetypeKind::Player, "Player_Character.prefab").unwrap();
    let spec = converted_animation_spec(player, &presentation).unwrap();
    let controller = presentation.controllers.get(&spec.controller).unwrap();
    let requests: BTreeMap<_, _> = controller
        .states
        .values()
        .flat_map(|state| &state.motions)
        .map(|motion| {
            let source = presentation.clips.get(&motion.clip).unwrap();
            let request = converted_clip_request(source, PLAYER_ANIMATED_MODEL_PATH, &presentation)
                .unwrap_or_else(|| {
                    panic!(
                        "{} has no native clip compatible with the visible player skin",
                        source.display_name
                    )
                });
            let ConvertedClipRequest::Asset { path, index } = request else {
                panic!(
                    "{} selected standalone retargeting instead of the visible rig",
                    source.display_name
                );
            };
            assert_eq!(path, PLAYER_ANIMATED_MODEL_PATH);
            (source.display_name.clone(), index)
        })
        .collect();
    let expected: BTreeMap<_, _> = [
        ("PlayerChar_Idle_01", 13),
        ("PlayerChar_Walk_01", 23),
        ("PlayerChar_Run_01", 18),
        ("PlayerChar_Logging_05", 24),
        ("PlayerChar_SpearAttack_01", 20),
        ("PlayerChar_Building_01", 4),
        ("PlayerChar_Farming_01", 7),
        ("PlayerChar_DEATH_01", 6),
        ("PlayerChar_Healing", 11),
        ("PlayerChar_HammerAttack_01", 10),
        ("PlayerChar_BowShoot_01", 3),
        ("PlayerChar_LoggerCarryWood_01", 1),
        ("PlayerChar_Gathering_01", 9),
        ("PlayerChar_Summon", 5),
        ("PlayerChar_Fishing_01", 8),
        ("PlayerChar_Mining_02", 17),
        ("PlayerChar_SwordAttack_01", 16),
        ("PlayerChar_CarryItems_01", 2),
        ("PlayerChar_Magic", 5),
        ("CharacterStaffMagicAttack", 22),
    ]
    .into_iter()
    .map(|(name, index)| (name.to_owned(), index))
    .collect();
    assert_eq!(requests, expected);

    let run = presentation
        .clips
        .values()
        .find(|clip| clip.display_name == "PlayerChar_Run_01")
        .unwrap();
    assert!(
        !run.transform_tracks.is_empty(),
        "the source-selection regression requires competing standalone curves"
    );
    assert_eq!(
        converted_clip_request(run, PLAYER_ANIMATED_MODEL_PATH, &presentation),
        Some(ConvertedClipRequest::Asset {
            path: PLAYER_ANIMATED_MODEL_PATH.to_owned(),
            index: 18,
        })
    );
}

#[test]
fn authored_player_run_clip_loop_pose_correction_closes_the_seam() {
    let presentation = embedded_presentation();
    let run = presentation
        .clips
        .values()
        .find(|clip| clip.display_name == "PlayerChar_Run_01")
        .unwrap();
    assert!(run.looping);
    let mut raw_max_rotation = 0.0_f32;
    let mut max_translation = 0.0_f32;
    let mut max_rotation = 0.0_f32;
    let mut max_scale = 0.0_f32;
    for track in &run.transform_tracks {
        let mut translation: Vec<_> = track
            .translation
            .iter()
            .map(|key| (key.time, Vec3::from_array(key.value)))
            .collect();
        close_translation_loop(&mut translation, run.duration_seconds);
        if let (Some((_, first)), Some((_, last))) = (translation.first(), translation.last()) {
            max_translation = max_translation.max(first.distance(*last));
            assert!(
                translation.last().unwrap().0 + 1.0e-6 >= run.duration_seconds,
                "{} translation ended before the clip boundary",
                track.target_path
            );
        }
        if let (Some(first), Some(last)) = (track.rotation.first(), track.rotation.last()) {
            raw_max_rotation = raw_max_rotation.max(
                Quat::from_array(first.value)
                    .normalize()
                    .angle_between(Quat::from_array(last.value).normalize()),
            );
        }
        let mut rotation: Vec<_> = track
            .rotation
            .iter()
            .map(|key| (key.time, normalized_quat(key.value)))
            .collect();
        close_rotation_loop(&mut rotation, run.duration_seconds);
        if let (Some((_, first)), Some((_, last))) = (rotation.first(), rotation.last()) {
            max_rotation = max_rotation.max(first.angle_between(*last));
            assert!(
                rotation.last().unwrap().0 + 1.0e-6 >= run.duration_seconds,
                "{} rotation ended before the clip boundary",
                track.target_path
            );
        }
        let mut scale: Vec<_> = track
            .scale
            .iter()
            .map(|key| (key.time, Vec3::from_array(key.value)))
            .collect();
        close_scale_loop(&mut scale, run.duration_seconds);
        if let (Some((_, first)), Some((_, last))) = (scale.first(), scale.last()) {
            max_scale = max_scale.max(first.distance(*last));
        }
    }
    assert!(
        raw_max_rotation > 0.5,
        "fixture no longer exercises its known run-pose seam: {raw_max_rotation}"
    );
    assert!(
        max_translation <= 1.0e-5,
        "corrected translation seam {max_translation}"
    );
    assert!(
        max_rotation <= 1.0e-3,
        "corrected rotation seam {max_rotation}"
    );
    assert!(max_scale <= 1.0e-5, "corrected scale seam {max_scale}");
}

#[test]
fn loop_pose_correction_preserves_the_first_pose_and_closes_the_last() {
    let first_rotation = Quat::from_rotation_x(-0.25) * Quat::from_rotation_z(0.1);
    let middle_rotation = Quat::from_rotation_x(0.4);
    let last_rotation = Quat::from_rotation_x(0.7) * Quat::from_rotation_z(-0.2);
    let mut rotations = vec![
        (0.0, first_rotation),
        (0.5, middle_rotation),
        (1.0, last_rotation),
    ];
    let mut translations = vec![
        (0.0, Vec3::new(1.0, 2.0, 3.0)),
        (0.5, Vec3::new(2.0, 3.0, 4.0)),
        (1.0, Vec3::new(3.0, 4.0, 5.0)),
    ];
    close_rotation_loop(&mut rotations, 1.0);
    close_translation_loop(&mut translations, 1.0);

    assert!(rotations[0].1.angle_between(first_rotation) <= 1.0e-6);
    assert!(rotations[2].1.angle_between(first_rotation) <= 1.0e-6);
    assert_eq!(translations[0].1, Vec3::new(1.0, 2.0, 3.0));
    assert!(translations[2].1.distance(translations[0].1) <= 1.0e-6);
    assert!(rotations[1].1.is_finite());
    assert!(translations[1].1.is_finite());
}

#[test]
fn authored_layer_weight_and_mask_configure_bevy_graph_branch() {
    let mut graph = AnimationGraph::new();
    let composition = add_animation_composition(&mut graph);
    assert!(matches!(
        graph.graph[composition].node_type,
        AnimationNodeType::Blend
    ));
    let path = "CharacterArmature/Body/UpperArm_R";
    let target = path.split('/').collect::<AnimationTargetId>();
    let targets = BTreeMap::from([(path.to_owned(), (Entity::PLACEHOLDER, Transform::IDENTITY))]);
    let authored = AvatarMaskDef {
        display_name: "Left Arm".into(),
        source_guid: "a".repeat(32),
        source_path: "Assets/LeftArm.mask".into(),
        humanoid_body_mask_hex: "01000000".into(),
        transform_weights: BTreeMap::from([(path.to_owned(), 0.0)]),
    };
    let mask = register_avatar_mask(&mut graph, 3, Some(&authored), &targets);
    let branch = add_animation_layer_branch(
        &mut graph,
        AnimationLayerBlendMode::Additive,
        0.25,
        mask,
        composition,
    );
    assert_eq!(mask, 1 << 3);
    assert_eq!(graph.mask_groups[&target], 1 << 3);
    assert!(matches!(
        graph.graph[branch].node_type,
        AnimationNodeType::Add
    ));
    assert!((graph.graph[branch].weight - 0.25).abs() < f32::EPSILON);
    assert_eq!(graph.graph[branch].mask, 1 << 3);
}

#[test]
fn connected_bot_dispatches_twitch_commands_without_a_chat_gate() {
    let viewer = |message: &str, is_broadcaster| {
        TwitchEvent::Chat(twitch::TwitchChatEnvelope {
            message_id: None,
            actor_id: StableId::new("twitch:42").unwrap(),
            user_id: "42".to_owned(),
            login: "viewer".to_owned(),
            display_name: "Viewer".to_owned(),
            message: message.to_owned(),
            is_broadcaster,
            is_moderator: false,
            is_subscriber: false,
            custom_reward_id: None,
        })
    };
    let mut connection = TwitchConnection::default();
    let mut commands = InjectedCommands::default();

    handle_twitch_event(viewer("!join", false), &mut connection, &mut commands);
    let dispatched = commands.0.pop_front().unwrap();
    assert_eq!(dispatched.actor_id, StableId::new("twitch:42").unwrap());
    assert_eq!(dispatched.command, ChatCommand::Join);
}

#[test]
fn broadcaster_moderation_state_changes_do_not_overwrite_bot_connection_state() {
    let mut connection = TwitchConnection {
        status: TwitchStatus::Connected,
        moderation_status: TwitchModerationStatus::Error(
            "old broadcaster authorization failed".to_owned(),
        ),
        ..default()
    };
    let mut commands = InjectedCommands::default();

    handle_twitch_event(
        TwitchEvent::ModerationStatus(TwitchModerationStatus::Authorizing),
        &mut connection,
        &mut commands,
    );
    assert_eq!(connection.status, TwitchStatus::Connected);
    assert_eq!(
        connection.moderation_status,
        TwitchModerationStatus::Authorizing
    );

    handle_twitch_event(
        TwitchEvent::ModerationStatus(TwitchModerationStatus::Ready),
        &mut connection,
        &mut commands,
    );
    assert_eq!(connection.status, TwitchStatus::Connected);
    assert_eq!(connection.moderation_status, TwitchModerationStatus::Ready);
}

#[test]
fn operator_chat_history_is_bounded_and_keeps_the_newest_messages() {
    let mut chat = OperatorChatRuntime::default();
    for index in 0..(OPERATOR_CHAT_HISTORY_CAPACITY + 5) {
        chat.push(OperatorChatLine {
            line_id: 0,
            user_id: index.to_string(),
            login: format!("viewer{index}"),
            display_name: format!("Viewer {index}"),
            message: format!("message {index}"),
            badges: OperatorChatBadges {
                broadcaster: false,
                moderator: false,
                subscriber: false,
            },
            is_system: false,
        });
    }
    assert_eq!(chat.lines.len(), OPERATOR_CHAT_HISTORY_CAPACITY);
    assert_eq!(chat.lines.front().unwrap().message, "message 5");
    assert_eq!(
        chat.lines.back().unwrap().message,
        format!("message {}", OPERATOR_CHAT_HISTORY_CAPACITY + 4)
    );
}

#[test]
fn operator_chat_deduplicates_the_broadcaster_echo_by_twitch_message_id() {
    let message = twitch::TwitchChatEnvelope {
        message_id: Some("outbound-message-id".to_owned()),
        actor_id: StableId::new("twitch:7").unwrap(),
        user_id: "7".to_owned(),
        login: "streamer".to_owned(),
        display_name: "Streamer".to_owned(),
        message: "Hello chat".to_owned(),
        is_broadcaster: true,
        is_moderator: true,
        is_subscriber: true,
        custom_reward_id: None,
    };
    let mut chat = OperatorChatRuntime::default();

    assert!(chat.push_chat(&message));
    assert!(!chat.push_chat(&message));
    assert_eq!(chat.lines.len(), 1);
    let line = chat.lines.back().unwrap();
    assert!(line.badges.broadcaster);
    assert!(line.badges.moderator);
    assert!(line.badges.subscriber);
}

#[test]
fn operator_chat_scroll_stays_anchored_when_new_messages_arrive() {
    let mut chat = OperatorChatRuntime::default();
    for index in 0..12 {
        chat.push_system(format!("message {index}"));
    }
    chat.scroll_older(2, 8);
    let before = chat
        .visible_lines(8)
        .into_iter()
        .map(|line| line.message.clone())
        .collect::<Vec<_>>();

    chat.push_system("new message");
    let after = chat
        .visible_lines(8)
        .into_iter()
        .map(|line| line.message.clone())
        .collect::<Vec<_>>();

    assert_eq!(before, after);
    chat.scroll_newer(usize::MAX);
    assert_eq!(chat.visible_lines(8).last().unwrap().message, "new message");
}

#[test]
fn invalid_command_feedback_only_targets_command_attempts() {
    assert_eq!(invalid_twitch_command_reply("hello town"), None);
    assert_eq!(invalid_twitch_command_reply("   hello town"), None);
    assert_eq!(
        invalid_twitch_command_reply("!not-a-command"),
        Some("Invalid Command! Type !help for the list of commands!")
    );
    assert_eq!(
        invalid_twitch_command_reply("  !cam sideways"),
        Some("Invalid Command! Type !help for the list of commands!")
    );
}

#[test]
fn help_points_to_the_versioned_native_command_reference() {
    assert!(TWITCH_COMMAND_HELP_URL.starts_with("https://github.com/HumanBeanGames/"));
    assert!(TWITCH_COMMAND_HELP_URL.ends_with("/TWITCH_COMMANDS.md"));
    let reference = include_str!("../../../../../TWITCH_COMMANDS.md");
    for command in [
        "join", "build", "confirm", "cancel", "role", "event", "yes", "no", "cam", "health", "help",
    ] {
        assert!(
            reference.contains(&format!("`!{command}")),
            "public !help reference omits native command !{command}"
        );
    }
}

#[test]
fn fish_god_channel_reward_dispatches_praise_without_command_text() {
    let mut connection = TwitchConnection {
        fish_god_reward_id: Some(stream_town_domain::SHIPPING_FISH_GOD_REWARD_ID.to_owned()),
        ..default()
    };
    let mut commands = InjectedCommands::default();
    handle_twitch_event(
        TwitchEvent::Chat(twitch::TwitchChatEnvelope {
            message_id: None,
            actor_id: StableId::new("twitch:fish").unwrap(),
            user_id: "fish".to_owned(),
            login: "fishfriend".to_owned(),
            display_name: "FishFriend".to_owned(),
            message: "Praise!".to_owned(),
            is_broadcaster: false,
            is_moderator: false,
            is_subscriber: false,
            custom_reward_id: Some(stream_town_domain::SHIPPING_FISH_GOD_REWARD_ID.to_owned()),
        }),
        &mut connection,
        &mut commands,
    );
    assert_eq!(commands.0.pop_front().unwrap().command, ChatCommand::Praise);
}

#[test]
fn unconfigured_or_different_channel_reward_does_not_dispatch_praise() {
    let mut connection = TwitchConnection {
        fish_god_reward_id: None,
        ..default()
    };
    let mut commands = InjectedCommands::default();
    handle_twitch_event(
        TwitchEvent::Chat(twitch::TwitchChatEnvelope {
            message_id: None,
            actor_id: StableId::new("twitch:fish").unwrap(),
            user_id: "fish".to_owned(),
            login: "fishfriend".to_owned(),
            display_name: "FishFriend".to_owned(),
            message: "Praise!".to_owned(),
            is_broadcaster: false,
            is_moderator: false,
            is_subscriber: false,
            custom_reward_id: Some("00000000-0000-0000-0000-000000000000".to_owned()),
        }),
        &mut connection,
        &mut commands,
    );
    assert!(commands.0.is_empty());
}

#[test]
fn ruler_permissions_and_login_candidate_resolution_are_stable() {
    let ruler = StableId::new("twitch:100").unwrap();
    let viewer = StableId::new("twitch:200").unwrap();
    let mut simulation = WorldSimulation::new(7);
    assert!(simulation.join_player(ruler.clone(), GridPos { x: 1, z: 1 }));
    assert!(simulation.join_player(viewer.clone(), GridPos { x: 2, z: 1 }));
    simulation.actors.get_mut(&ruler).unwrap().login_name = Some("the_ruler".to_owned());
    simulation.actors.get_mut(&ruler).unwrap().display_name = Some("The Ruler".to_owned());
    simulation.set_ruler(ruler.clone()).unwrap();
    simulation
        .start_ruler_vote(RulerVoteKind::KeepRuler)
        .unwrap();

    let ordinary = PendingChatCommand {
        actor_id: viewer,
        login_name: "viewer".to_owned(),
        display_name: "Viewer".to_owned(),
        command: ChatCommand::RecruitCount,
        is_broadcaster: false,
        is_moderator: false,
        is_subscriber: false,
        origin: CommandOrigin::Twitch,
    };
    assert!(require_ruler_or_staff(&simulation, &ordinary).is_err());
    let ruler_command = PendingChatCommand {
        actor_id: ruler,
        login_name: "the_ruler".to_owned(),
        display_name: "The Ruler".to_owned(),
        command: ChatCommand::RecruitCount,
        is_broadcaster: false,
        is_moderator: false,
        is_subscriber: false,
        origin: CommandOrigin::Twitch,
    };
    assert!(require_ruler_or_staff(&simulation, &ruler_command).is_ok());
    assert_eq!(
        resolve_ruler_vote_option(&simulation, &StableId::new("yes").unwrap()),
        Some(StableId::new("yes").unwrap())
    );
    simulation.ruler_vote = None;
    simulation
        .start_ruler_vote(RulerVoteKind::NewRuler)
        .unwrap();
    assert_eq!(
        resolve_ruler_vote_option(&simulation, &StableId::new("the_ruler").unwrap()),
        Some(StableId::new("twitch:100").unwrap())
    );
}

#[test]
fn game_master_permission_is_explicit_and_local_debug_bypasses_it() {
    let mut config = GameConfig::default();
    let mut command = PendingChatCommand {
        actor_id: StableId::new("twitch:12345").unwrap(),
        login_name: "configured_gm".to_owned(),
        display_name: "Configured GM".to_owned(),
        command: ChatCommand::ToggleBuildCosts,
        is_broadcaster: true,
        is_moderator: true,
        is_subscriber: false,
        origin: CommandOrigin::Twitch,
    };
    assert!(require_staff(&command).is_ok());
    assert!(require_game_master(&config, &command).is_err());
    config.twitch.game_master_ids.insert("12345".to_owned());
    assert!(require_game_master(&config, &command).is_ok());
    let user_type = pending_stream_user_type(&config, &command);
    assert_eq!(user_type, StreamUserType::GameMaster);
    let simulation = WorldSimulation::new(1);
    assert!(require_world_view_operator(&simulation, &command, user_type).is_ok());
    assert!(is_world_diagnostic_timer_operator(
        &simulation,
        &command.actor_id,
        user_type,
    ));
    config.twitch.game_master_ids.clear();
    command.is_broadcaster = false;
    command.is_moderator = false;
    command.origin = CommandOrigin::LocalDebug;
    assert!(require_game_master(&config, &command).is_ok());
}

#[test]
fn unity_name_and_health_overlay_visibility_rules_are_preserved() {
    assert!(!should_show_actor_name(
        NameDisplayMode::None,
        StreamUserType::GameMaster
    ));
    assert!(should_show_actor_name(
        NameDisplayMode::StaffAndSubscribers,
        StreamUserType::Subscriber
    ));
    assert!(should_show_actor_name(
        NameDisplayMode::StaffAndSubscribers,
        StreamUserType::Moderator
    ));
    assert!(!should_show_actor_name(
        NameDisplayMode::StaffAndSubscribers,
        StreamUserType::Normal
    ));
    assert!(should_show_actor_name(
        NameDisplayMode::AllPlayers,
        StreamUserType::Normal
    ));

    assert!(!should_show_building_health(
        BuildingHealthDisplayMode::None,
        25,
        100
    ));
    assert!(!should_show_building_health(
        BuildingHealthDisplayMode::DamagedOnly,
        100,
        100
    ));
    assert!(should_show_building_health(
        BuildingHealthDisplayMode::DamagedOnly,
        99,
        100
    ));
    assert!(should_show_building_health(
        BuildingHealthDisplayMode::Always,
        100,
        100
    ));
    assert!((building_health_fraction(25, 100) - 0.25).abs() < f32::EPSILON);
}

#[test]
fn path_health_bars_use_fine_positions_and_one_fifth_width() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let mut simulation = WorldSimulation::new(world.seed);
    let fallback = grid_to_world_on_surface(GridPos { x: 1, z: 1 }, &config, &world);
    let mut resolved = Vec::new();
    for (index, position) in [
        GridPos { x: 3, z: 4 },
        GridPos { x: 4, z: 4 },
        GridPos { x: 5, z: 4 },
    ]
    .into_iter()
    .enumerate()
    {
        let id = StableId::new(format!("building:path_health_{index}")).unwrap();
        simulation
            .path_navigation_positions
            .insert(id.clone(), position);
        resolved.push(building_health_overlay_world_position(
            &RuntimeBuilding { id },
            fallback,
            &simulation,
            &config,
            &world,
        ));
    }
    assert_ne!(resolved[0], resolved[1]);
    assert_ne!(resolved[1], resolved[2]);
    assert_ne!(resolved[0], resolved[2]);

    let path_definition = &content.buildings[&StableId::new("building:path").unwrap()];
    let path = BuildingState {
        id: StableId::new("building:path_health_width").unwrap(),
        archetype: path_definition.archetype.clone(),
        position: GridPos { x: 1, z: 1 },
        rotation_quarter_turns: 0,
        level: 1,
        health: 100,
        complete: true,
    };
    let house_definition = &content.buildings[&StableId::new("building:house").unwrap()];
    let house = BuildingState {
        id: StableId::new("building:house_health_width").unwrap(),
        archetype: house_definition.archetype.clone(),
        position: GridPos { x: 1, z: 1 },
        rotation_quarter_turns: 0,
        level: 1,
        health: 100,
        complete: true,
    };
    assert!(
        (building_health_overlay_width_px(&content, &house) - BUILDING_HEALTH_OVERLAY_WIDTH_PX)
            .abs()
            < f32::EPSILON
    );
    assert!(
        (building_health_overlay_width_px(&content, &path) - PATH_HEALTH_OVERLAY_WIDTH_PX).abs()
            < f32::EPSILON
    );
    assert!(
        (building_health_overlay_width_px(&content, &path) * 5.0
            - building_health_overlay_width_px(&content, &house))
        .abs()
            < f32::EPSILON
    );
}

#[test]
fn paths_use_their_authored_health_and_catalog_level_bonus() {
    let content = embedded_content();
    let definition = &content.buildings[&StableId::new("building:path").unwrap()];
    let authored = content.archetypes[&definition.archetype]
        .health
        .as_ref()
        .expect("paths have authored health");
    let mut path = BuildingState {
        id: StableId::new("building:path_health_test").unwrap(),
        archetype: definition.archetype.clone(),
        position: GridPos { x: 1, z: 1 },
        rotation_quarter_turns: 0,
        level: 1,
        health: 1,
        complete: false,
    };

    assert_eq!(
        building_base_max_health(&content, definition),
        authored.max_health
    );
    assert_eq!(
        u32::try_from(building_max_health(&content, &path)).unwrap(),
        authored.max_health
    );
    path.level = 2;
    assert_eq!(
        u32::try_from(building_max_health(&content, &path)).unwrap(),
        leveled_percentage_stat(
            authored.max_health,
            definition.health_bonus_per_level_per_thousand,
            2,
        )
    );
}

#[test]
fn player_health_bar_uses_authored_damage_and_hide_contract() {
    let content = embedded_content();
    let player = content
        .archetypes
        .values()
        .find(|archetype| archetype.source_path.ends_with("Player_Character.prefab"))
        .unwrap();
    assert_eq!(player.health_bar_hide_milliseconds, Some(3_000));
    assert!(
        player
            .component_types
            .iter()
            .any(|component| component == "Units.UnitHealthBar")
    );

    let mut overlay = ActorHealthOverlay {
        actor: StableId::new("twitch:health_bar_test").unwrap(),
        last_health: 100,
        was_damaged: false,
        hide_remaining_seconds: 0.0,
    };
    assert!(update_actor_health_overlay_timer(
        &mut overlay,
        75,
        100,
        3.0,
        0.1
    ));
    assert!(update_actor_health_overlay_timer(
        &mut overlay,
        100,
        100,
        3.0,
        1.0
    ));
    assert!(update_actor_health_overlay_timer(
        &mut overlay,
        100,
        100,
        3.0,
        1.9
    ));
    assert!(!update_actor_health_overlay_timer(
        &mut overlay,
        100,
        100,
        3.0,
        0.2
    ));
    assert!(!overlay.was_damaged);
}

#[test]
fn numbered_world_labels_preserve_unitys_fifteen_second_lifetime() {
    let mut label = TemporaryWorldLabel {
        target: StableId::new("npc:starting_builder").unwrap(),
        remaining_seconds: UNITY_NUMBERED_LABEL_SECONDS,
    };
    assert!(temporary_world_label_is_live(&mut label, 14.9));
    assert!((label.remaining_seconds - 0.1).abs() < 1e-5);
    assert!(!temporary_world_label_is_live(&mut label, 0.2));
    assert!(label.remaining_seconds.abs() < f32::EPSILON);
}

#[test]
fn twitch_user_type_uses_unity_privilege_priority() {
    let mut config = GameConfig::default();
    let mut command = PendingChatCommand {
        actor_id: StableId::new("twitch:12345").unwrap(),
        login_name: "staff".to_owned(),
        display_name: "Staff".to_owned(),
        command: ChatCommand::Join,
        is_broadcaster: true,
        is_moderator: true,
        is_subscriber: true,
        origin: CommandOrigin::Twitch,
    };
    assert_eq!(
        pending_stream_user_type(&config, &command),
        StreamUserType::Broadcaster
    );
    command.is_broadcaster = false;
    assert_eq!(
        pending_stream_user_type(&config, &command),
        StreamUserType::Moderator
    );
    config.twitch.game_master_ids.insert("12345".to_owned());
    assert_eq!(
        pending_stream_user_type(&config, &command),
        StreamUserType::GameMaster
    );
}

fn enter_headless_world(app: &mut App) {
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::WorldLoading);
    app.update();
    app.update();
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        Duration::from_millis(250),
    ));
    for _ in 0..1_024 {
        app.update();
        if *app.world().resource::<State<GameState>>().get() == GameState::InGame
            && app.world().contains_resource::<GameplayReady>()
        {
            break;
        }
        std::thread::yield_now();
    }
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        Duration::ZERO,
    ));
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::InGame
    );
    assert!(
        app.world().contains_resource::<GameplayReady>(),
        "headless world entry must wait for the same overlay-retirement gate as production"
    );
}

#[test]
fn world_loading_transition_reuses_the_already_rendered_cover_entities() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(GameConfig::default()))
    .add_plugins(StreamTownGamePlugin);
    app.update();

    let before = app
        .world_mut()
        .query_filtered::<Entity, With<LoadingScreenEntity>>()
        .iter(app.world())
        .collect::<BTreeSet<_>>();
    assert!(!before.is_empty());
    let loading_camera_before = {
        let world = app.world_mut();
        let mut query = world.query_filtered::<(Entity, &Camera), With<LoadingUiCamera>>();
        let (entity, camera) = query
            .single(world)
            .expect("exactly one persistent loading UI camera");
        assert_eq!(camera.order, 1_000);
        assert!(matches!(
            camera.clear_color,
            bevy::camera::ClearColorConfig::None
        ));
        entity
    };
    assert!(
        app.world()
            .get::<StateEntity>(loading_camera_before)
            .is_none(),
        "the loading camera must survive scene-state cleanup"
    );
    let targeted_roots_before = {
        let world = app.world_mut();
        let mut query = world
            .query_filtered::<&UiTargetCamera, (With<LoadingScreenEntity>, Without<ChildOf>)>();
        query.iter(world).map(|target| target.0).collect::<Vec<_>>()
    };
    assert!(!targeted_roots_before.is_empty());
    assert!(
        targeted_roots_before
            .iter()
            .all(|target| *target == loading_camera_before),
        "every loading overlay root must use the stable UI-only camera"
    );

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::WorldLoading);
    app.update();

    let after = app
        .world_mut()
        .query_filtered::<Entity, With<LoadingScreenEntity>>()
        .iter(app.world())
        .collect::<BTreeSet<_>>();
    assert_eq!(before, after);
    let loading_camera_after = app
        .world_mut()
        .query_filtered::<Entity, With<LoadingUiCamera>>()
        .single(app.world())
        .expect("the persistent loading UI camera survives WorldLoading");
    assert_eq!(loading_camera_before, loading_camera_after);
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::WorldLoading
    );
}

#[test]
fn headless_launch_through_credits_round_trip_covers_shipping_states() {
    let config = GameConfig::default();
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("launch-through-credits.stbevy");
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(config))
    .add_plugins(StreamTownGamePlugin);
    app.insert_resource(SaveRuntime {
        store: NativeSaveStore::new(&save_path),
    });

    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::Boot
    );
    app.update();
    app.update();
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::MainMenu
    );
    assert!(
        app.world_mut()
            .query_filtered::<Entity, With<StateEntity>>()
            .iter(app.world())
            .count()
            > 0
    );
    let main_menu_actions = app
        .world_mut()
        .query::<&MainMenuAction>()
        .iter(app.world())
        .copied()
        .collect::<Vec<_>>();
    assert_eq!(main_menu_actions.len(), 6);
    assert!(main_menu_actions.contains(&MainMenuAction::LoadGame));
    assert!(main_menu_actions.contains(&MainMenuAction::Secrets));

    enter_headless_world(&mut app);
    for _ in 0..16 {
        app.update();
    }
    assert!(
        app.world_mut()
            .query_filtered::<Entity, With<LoadingScreenEntity>>()
            .iter(app.world())
            .next()
            .is_none(),
        "the loading cover must retire after the reveal gate"
    );
    assert!(
        app.world_mut()
            .query_filtered::<Entity, With<LoadingUiCamera>>()
            .iter(app.world())
            .next()
            .is_none(),
        "the no-clear loading camera must retire with its cover"
    );
    let viewer = StableId::new("twitch:acceptance_viewer").unwrap();
    app.world_mut()
        .resource_mut::<InjectedCommands>()
        .0
        .push_back(PendingChatCommand {
            actor_id: viewer.clone(),
            login_name: "acceptance_viewer".to_owned(),
            display_name: "Acceptance Viewer".to_owned(),
            command: ChatCommand::Join,
            is_broadcaster: false,
            is_moderator: false,
            is_subscriber: true,
            origin: CommandOrigin::LocalDebug,
        });
    app.update();
    assert!(
        app.world()
            .resource::<SimulationRuntime>()
            .0
            .actors
            .contains_key(&viewer)
    );

    app.world_mut().resource_mut::<MenuIoRequest>().save = true;
    app.update();
    let saved = NativeSaveStore::new(&save_path).load().unwrap();
    assert!(saved.simulation.actors.contains_key(&viewer));

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::MainMenu);
    app.update();
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::MainMenu
    );
    assert!(!app.world().contains_resource::<WorldRuntime>());
    assert!(!app.world().contains_resource::<SimulationRuntime>());

    app.world_mut().resource_mut::<MenuIoRequest>().load = true;
    enter_headless_world(&mut app);
    app.update();
    assert!(
        app.world()
            .resource::<SimulationRuntime>()
            .0
            .actors
            .contains_key(&viewer)
    );
    assert!(
        app.world()
            .resource::<RuntimeConsoleRuntime>()
            .last_result
            .starts_with("Loaded ")
    );

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::MainMenu);
    app.update();
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Credits);
    app.update();
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::Credits
    );
    assert!(app.world().contains_resource::<CreditsTimeline>());
    assert!(
        app.world_mut()
            .query::<&AuthoredCreditsElement>()
            .iter(app.world())
            .count()
            > 0
    );

    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        Duration::from_mins(1),
    ));
    app.update();
    assert!(
        app.world_mut()
            .query::<&CreditsFireworksEmitter>()
            .iter(app.world())
            .count()
            >= 2
    );
    let skip_button = app
        .world_mut()
        .query_filtered::<Entity, With<CreditsSkipButton>>()
        .single(app.world())
        .unwrap();
    app.world_mut()
        .entity_mut(skip_button)
        .insert(Interaction::Pressed);
    app.update();
    app.update();
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::MainMenu
    );
    assert!(!app.world().contains_resource::<CreditsTimeline>());
    assert_eq!(
        app.world_mut()
            .query::<&AuthoredCreditsElement>()
            .iter(app.world())
            .count(),
        0
    );
}

#[test]
fn headless_new_town_matches_shipping_starting_roster() {
    let config = GameConfig::default();
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("shipping-start.stbevy");
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(config))
    .add_plugins(StreamTownGamePlugin);
    app.insert_resource(SaveRuntime {
        store: NativeSaveStore::new(&save_path),
    });

    app.update();
    app.update();
    enter_headless_world(&mut app);

    {
        let simulation = &app.world().resource::<SimulationRuntime>().0;
        assert_eq!(simulation.actors.len(), 5);
        assert!(!simulation.enemy_camps.is_empty());
        assert!(simulation.enemy_camps.values().all(|camp| {
            camp.spawned_enemies.is_empty() && camp.spawn_remaining_seconds.abs() <= f64::EPSILON
        }));
        assert!(
            simulation
                .actors
                .values()
                .all(|actor| actor.role.as_str() != "role:enemy")
        );
        assert_eq!(recruited_actor_ids(simulation).len(), 5);
        for (index, (expected_id, expected_role)) in [
            ("npc:starting_defender", "role:defender"),
            ("npc:starting_logger", "role:logger"),
            ("npc:starting_miner", "role:miner"),
            ("npc:starting_gatherer", "role:gatherer"),
            ("npc:starting_builder", "role:builder"),
        ]
        .into_iter()
        .enumerate()
        {
            let actor_id = StableId::new(expected_id).unwrap();
            assert_eq!(
                simulation
                    .actors
                    .get(&actor_id)
                    .map(|actor| actor.role.as_str()),
                Some(expected_role),
                "starting actor {index}"
            );
        }
    }

    let agents: Vec<_> = app
        .world_mut()
        .query::<&Agent>()
        .iter(app.world())
        .map(|agent| (agent.id.clone(), agent.kind.clone()))
        .collect();
    assert_eq!(agents.len(), 5);
    assert!(agents.iter().all(|(_, kind)| *kind == ActorKind::Player));
}

#[test]
fn starting_logger_and_miner_recover_from_bad_targets_and_gather_resources() {
    let config = GameConfig::default();
    let save_directory = tempfile::tempdir().unwrap();
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(config))
    .add_plugins(StreamTownGamePlugin);
    app.insert_resource(SaveRuntime {
        store: NativeSaveStore::new(save_directory.path().join("active-workers.stbevy")),
    });
    app.update();
    app.update();
    enter_headless_world(&mut app);

    let logger = StableId::new("npc:starting_logger").unwrap();
    let miner = StableId::new("npc:starting_miner").unwrap();
    let wood = StableId::new("resource:wood").unwrap();
    let ore = StableId::new("resource:ore").unwrap();
    let resource_total = |world: &GeneratedWorld, kind: &StableId| {
        world
            .resources
            .iter()
            .filter(|resource| resource.kind == *kind)
            .map(|resource| u64::from(resource.amount))
            .sum::<u64>()
    };
    let initial_wood = resource_total(&app.world().resource::<WorldRuntime>().generated, &wood);
    let initial_ore = resource_total(&app.world().resource::<WorldRuntime>().generated, &ore);
    let mut logger_positions = BTreeSet::new();
    let mut miner_positions = BTreeSet::new();
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        Duration::from_millis(250),
    ));
    for _ in 0..720 {
        app.update();
        let simulation = &app.world().resource::<SimulationRuntime>().0;
        logger_positions.insert(simulation.actors[&logger].position);
        miner_positions.insert(simulation.actors[&miner].position);
        let world = &app.world().resource::<WorldRuntime>().generated;
        if resource_total(world, &wood) < initial_wood && resource_total(world, &ore) < initial_ore
        {
            break;
        }
    }
    let world = &app.world().resource::<WorldRuntime>().generated;
    assert!(
        resource_total(world, &wood) < initial_wood,
        "the starting logger must eventually gather wood"
    );
    assert!(
        resource_total(world, &ore) < initial_ore,
        "the starting miner must eventually gather ore"
    );
    assert!(
        logger_positions.len() > 2,
        "logger remained effectively AFK"
    );
    assert!(miner_positions.len() > 2, "miner remained effectively AFK");
}
