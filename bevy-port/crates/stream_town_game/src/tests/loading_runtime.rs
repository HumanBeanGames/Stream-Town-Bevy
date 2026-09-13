#[test]
fn gameplay_gate_remains_closed_until_loading_entities_and_runtimes_are_gone() {
    let mut app = App::new();
    app.insert_resource(AutomaticResumeRuntime::default())
        .add_systems(Update, confirm_gameplay_ready);
    let loading_entity = app.world_mut().spawn(LoadingScreenEntity).id();
    app.update();
    assert!(!app.world().contains_resource::<GameplayReady>());

    app.world_mut().despawn(loading_entity);
    app.world_mut()
        .insert_resource(WorldLoadingRuntime::default());
    app.update();
    assert!(!app.world().contains_resource::<GameplayReady>());

    app.world_mut().remove_resource::<WorldLoadingRuntime>();
    app.world_mut()
        .insert_resource(WorldRevealRuntime::default());
    app.update();
    assert!(!app.world().contains_resource::<GameplayReady>());

    app.world_mut().remove_resource::<WorldRevealRuntime>();
    app.update();
    assert!(app.world().contains_resource::<GameplayReady>());
}

#[test]
fn automatic_resume_gate_never_exposes_an_unapplied_fallback_world() {
    let mut app = App::new();
    app.insert_resource(AutomaticResumeRuntime::new(Some(PathBuf::from(
        ".stream-town/saves/Tonyville.stbevy",
    ))))
    .add_systems(Update, confirm_gameplay_ready);

    app.update();
    assert!(!app.world().contains_resource::<GameplayReady>());

    app.world_mut()
        .resource_mut::<AutomaticResumeRuntime>()
        .applied = true;
    app.update();
    assert!(app.world().contains_resource::<GameplayReady>());
}

#[test]
fn returning_to_main_menu_recreates_the_cover_before_scene_construction() {
    let mut app = App::new();
    app.insert_resource(RuntimeConfig(GameConfig::default()))
        .insert_resource(RuntimeContent(embedded_content()))
        .insert_resource(RuntimeAssetRoot(PathBuf::from("unused-test-assets")))
        .add_systems(
            Update,
            (spawn_loading_screen, ensure_main_menu_loading).chain(),
        );

    app.update();

    let mut covers = app
        .world_mut()
        .query_filtered::<Entity, With<LoadingScreenEntity>>();
    assert!(covers.iter(app.world()).count() >= 4);
    assert_eq!(
        app.world().resource::<MenuLoadingRuntime>().destination,
        BootDestination::MainMenu
    );
    assert!(app.world().resource::<MenuLoadingRuntime>().progress.abs() < f32::EPSILON);
}

#[test]
fn gameplay_simulation_set_is_paused_without_the_ready_marker() {
    #[derive(Resource, Default)]
    struct Counter(u32);

    fn count_update(mut counter: ResMut<Counter>) {
        counter.0 += 1;
    }

    let mut app = App::new();
    app.init_resource::<Counter>()
        .configure_sets(
            Update,
            GameplaySimulationSet.run_if(resource_exists::<GameplayReady>),
        )
        .add_systems(Update, count_update.in_set(GameplaySimulationSet));
    app.update();
    assert_eq!(app.world().resource::<Counter>().0, 0);
    app.world_mut().insert_resource(GameplayReady);
    app.update();
    assert_eq!(app.world().resource::<Counter>().0, 1);
    app.world_mut().remove_resource::<GameplayReady>();
    app.update();
    assert_eq!(app.world().resource::<Counter>().0, 1);
}

#[test]
fn stream_only_agent_simulation_accumulates_real_time_at_sixty_hertz() {
    let mut cadence = AgentSimulationCadenceState::default();
    assert!(!advance_agent_simulation_cadence(
        &mut cadence,
        Duration::from_millis(8),
        true,
        true,
    ));
    assert!(advance_agent_simulation_cadence(
        &mut cadence,
        Duration::from_millis(9),
        true,
        true,
    ));
    assert_eq!(cadence.delta, Duration::from_millis(17));
    assert_eq!(cadence.accumulated, Duration::ZERO);

    assert!(advance_agent_simulation_cadence(
        &mut cadence,
        Duration::from_millis(4),
        false,
        true,
    ));
    assert_eq!(cadence.delta, Duration::from_millis(4));
    assert!(!advance_agent_simulation_cadence(
        &mut cadence,
        Duration::from_millis(20),
        true,
        false,
    ));
    assert_eq!(cadence.delta, Duration::ZERO);
}

#[test]
fn stream_only_night_lights_sync_at_output_cadence() {
    let mut cadence = AgentSimulationCadenceState::default();
    for _ in 0..3 {
        assert!(!advance_stream_only_cadence(
            &mut cadence,
            Duration::from_millis(8),
            true,
            true,
            STREAM_ONLY_NIGHT_LIGHT_SYNC_HZ,
        ));
    }
    assert!(advance_stream_only_cadence(
        &mut cadence,
        Duration::from_millis(10),
        true,
        true,
        STREAM_ONLY_NIGHT_LIGHT_SYNC_HZ,
    ));
    assert_eq!(cadence.delta, Duration::from_millis(34));
    assert_eq!(cadence.accumulated, Duration::ZERO);
}

#[test]
fn day_night_cycle_rotates_the_shipping_sun_parent() {
    let day = in_game_sun_transform_for_daylight(1.0);
    let dusk = in_game_sun_transform_for_daylight(0.5);
    let night = in_game_sun_transform_for_daylight(0.0);
    assert!(day.rotation.angle_between(dusk.rotation).to_degrees() > 59.0);
    assert!(dusk.rotation.angle_between(night.rotation).to_degrees() > 59.0);
    assert!((day.rotation.angle_between(night.rotation).to_degrees() - 120.0).abs() < 0.01);
    assert_eq!(day.translation, night.translation);
}

#[test]
fn sunset_presentation_preserves_sub_millisecond_daylight_changes() {
    let config = GameConfig::default();
    let day_seconds = f64::from(config.time.seconds_per_day);
    let transition_seconds = f64::from(config.time.transition_seconds);
    let dusk_midpoint = day_seconds * f64::from(config.time.daylight_per_thousand) / 1_000.0
        - transition_seconds * 0.5;
    let first = config.time.sample(dusk_midpoint).daylight;
    let second = config.time.sample(dusk_midpoint + 0.01).daylight;

    assert!((first - second).abs() < 0.001);
    assert_ne!(daylight_signature(first), daylight_signature(second));
    assert_ne!(
        in_game_sun_transform_for_daylight(first).rotation,
        in_game_sun_transform_for_daylight(second).rotation
    );
}

#[test]
fn building_material_cycle_stays_phase_aligned_without_cpu_asset_updates() {
    let config = GameConfig::default();
    let app_elapsed_seconds = 2_345.25;
    for simulation_elapsed_seconds in [0.0, 2_297.0, 2_347.0, 2_400.0, 3_550.0, 85_440.5] {
        let parameters = building_material_time_cycle(
            &config.time,
            simulation_elapsed_seconds,
            app_elapsed_seconds,
        );
        let app_elapsed = Duration::from_secs_f64(app_elapsed_seconds).as_secs_f32();
        let shader_phase =
            f64::from(app_elapsed + parameters.w).rem_euclid(f64::from(parameters.x));
        let cpu = config.time.sample(simulation_elapsed_seconds).daylight;
        let shader_equivalent = config.time.sample(shader_phase).daylight;
        assert!((cpu - shader_equivalent).abs() < 0.000_1);
    }

    let shader = include_str!("../../../../assets/shaders/building_material.wgsl");
    assert!(shader.contains("view_bindings::globals.time"));
    assert!(shader.contains("(1.0 - building_daylight())"));
}

#[test]
fn building_material_round_robin_covers_every_instance_with_a_bounded_batch() {
    let total = 83;
    let mut cursor = 0;
    let mut visited = vec![false; total];
    for _ in 0..total.div_ceil(MAX_BUILDING_MATERIAL_UPDATES_PER_FRAME) {
        let batch = round_robin_indices(total, cursor)
            .take(MAX_BUILDING_MATERIAL_UPDATES_PER_FRAME)
            .collect::<Vec<_>>();
        for index in &batch {
            visited[*index] = true;
        }
        cursor = (cursor + batch.len()) % total;
    }

    assert!(visited.into_iter().all(|was_visited| was_visited));
}

#[test]
fn player_runtime_uses_the_single_animation_armature() {
    let content = embedded_content();
    let player =
        archetype_by_source(&content, ArchetypeKind::Player, "Player_Character.prefab").unwrap();
    let scene = runtime_archetype_scene(player).unwrap();
    assert_eq!(scene.asset_path, PLAYER_ANIMATED_MODEL_PATH);
    assert_eq!(scene.source_model, PLAYER_ANIMATED_SOURCE_MODEL);
}

#[test]
fn replaced_converted_scene_becomes_eligible_for_animation_rebinding() {
    let mut app = App::new();
    app.add_systems(Update, repair_replaced_converted_animation_instances);
    let controller = StableId::new("controller:test").unwrap();
    let state = StableId::new("animation_state:test").unwrap();
    let stale_root = app
        .world_mut()
        .spawn((
            ConvertedAnimationSpec {
                controller: controller.clone(),
                state: state.clone(),
                rig_scene: "test.glb".to_owned(),
            },
            ConvertedAnimationInstanceReady,
            ConvertedAnimationApplied,
        ))
        .id();
    let live_root = app
        .world_mut()
        .spawn((
            ConvertedAnimationSpec {
                controller: controller.clone(),
                state,
                rig_scene: "test.glb".to_owned(),
            },
            ConvertedAnimationInstanceReady,
            ConvertedAnimationApplied,
        ))
        .id();
    app.world_mut().spawn(ConvertedAnimationDriver {
        actor_root: live_root,
        controller,
        layers: Vec::new(),
        last_alive: None,
        active_action: None,
        transient_carry_visible: false,
    });

    app.update();

    assert!(
        app.world()
            .get::<ConvertedAnimationApplied>(stale_root)
            .is_none(),
        "a discarded scene hierarchy must not permanently suppress rebinding"
    );
    assert!(
        app.world()
            .get::<ConvertedAnimationApplied>(live_root)
            .is_some(),
        "a live animation driver must retain the applied marker"
    );
}

#[test]
fn game_menu_exposes_state_appropriate_actions_and_save_availability() {
    let in_game = game_menu_text(GameState::InGame, 3, false);
    assert!(in_game.contains("STREAM TOWN MENU"));
    assert!(in_game.contains("Load Game  [No save]"));
    assert!(in_game.contains("> Settings"));
    assert!(in_game.contains("Exit Game"));
    assert!(!in_game.contains("Idle Mode"));

    let main_menu = game_menu_text(GameState::MainMenu, 1, true);
    assert!(main_menu.contains("New town"));
    assert!(main_menu.contains("> Load game\n"));
    assert!(main_menu.contains("Credits"));
    assert!(main_menu.contains("Quit"));
}

#[test]
fn shipping_game_menu_preserves_operator_actions_without_legacy_idle_control() {
    let presentation = embedded_presentation();
    for source_path in GAME_MENU_TEXTURE_PATHS {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path),
            "missing Game Menu texture {source_path}"
        );
    }
    let actions = [
        GameMenuAction::SaveGame,
        GameMenuAction::SaveJumpStart,
        GameMenuAction::LoadGame,
        GameMenuAction::Settings,
        GameMenuAction::GoLive,
        GameMenuAction::ExitGame,
        GameMenuAction::Close,
    ];
    assert_eq!(
        actions.map(|action| game_menu_action_label(action, false)),
        [
            "Save Game",
            "Save Jump-Start",
            "Load Game",
            "Settings",
            "Go Live",
            "Exit Game",
            "Close"
        ]
    );
    assert_eq!(
        game_menu_action_label(GameMenuAction::GoLive, true),
        "End Stream"
    );
    assert!(!game_menu_action_enabled(GameMenuAction::LoadGame, false));
    assert!(game_menu_action_enabled(GameMenuAction::LoadGame, true));
    assert!(game_menu_action_enabled(GameMenuAction::Close, false));
}

#[cfg(target_os = "windows")]
#[test]
fn shared_operator_toggle_ends_an_active_stream_before_checking_setup() {
    let mut runtime = direct_broadcast::DirectBroadcastRuntime::default();
    runtime.set_phase_for_test(direct_broadcast::DirectBroadcastPhase::Broadcasting);
    let mut control = direct_broadcast::DirectBroadcastControl::default();
    let mut feedback = String::new();

    toggle_direct_broadcast(
        &GameConfig::default(),
        &SecretsRuntime::default(),
        &TwitchConnection::default(),
        &runtime,
        &mut control,
        &mut feedback,
    );

    assert!(control.stop_requested_for_test());
    assert!(feedback.contains("Ending"));
}

#[test]
fn game_menu_mouse_actions_use_authoritative_requests_and_settings() {
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("game-menu-actions.stbevy");
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
        .init_state::<GameState>()
        .insert_resource(MenuRuntime {
            page: MenuPage::Game,
            ..default()
        })
        .init_resource::<MenuIoRequest>()
        .init_resource::<SecretsRuntime>()
        .init_resource::<TwitchConnection>()
        .insert_resource(RuntimeConfig(GameConfig::default()))
        .insert_resource(RuntimePlayerSettings(PlayerSettings::default()))
        .insert_resource(SaveRuntime {
            store: NativeSaveStore::new(&save_path),
        });
    #[cfg(target_os = "windows")]
    app.init_resource::<direct_broadcast::DirectBroadcastControl>()
        .init_resource::<direct_broadcast::DirectBroadcastRuntime>();
    app.add_systems(Update, game_menu_buttons);
    app.world_mut()
        .spawn((Interaction::Pressed, GameMenuAction::SaveGame));

    app.update();

    assert!(app.world().resource::<MenuIoRequest>().save);
    assert_eq!(app.world().resource::<MenuRuntime>().page, MenuPage::Closed);

    app.world_mut().resource_mut::<MenuIoRequest>().save = false;
    app.world_mut().resource_mut::<MenuRuntime>().page = MenuPage::Game;
    app.world_mut()
        .spawn((Interaction::Pressed, GameMenuAction::Settings));
    app.update();

    assert_eq!(
        app.world().resource::<MenuRuntime>().page,
        MenuPage::Settings
    );
    assert_eq!(
        app.world().resource::<MenuRuntime>().return_page,
        MenuPage::Game
    );
}

#[test]
fn cursor_is_hidden_in_the_town_and_visible_for_escape_menu_children() {
    assert!(!cursor_visible_for_state(GameState::Boot, MenuPage::Closed));
    assert!(!cursor_visible_for_state(
        GameState::WorldLoading,
        MenuPage::Closed
    ));
    assert!(!cursor_visible_for_state(
        GameState::InGame,
        MenuPage::Closed
    ));
    assert!(cursor_visible_for_state(GameState::InGame, MenuPage::Game));
    assert!(cursor_visible_for_state(
        GameState::InGame,
        MenuPage::Settings
    ));
    assert!(cursor_visible_for_state(
        GameState::MainMenu,
        MenuPage::Closed
    ));
    assert!(cursor_visible_for_state(
        GameState::Credits,
        MenuPage::Closed
    ));
}

#[test]
fn town_names_have_stable_seeds_and_windows_safe_independent_save_names() {
    assert_eq!(town_name_seed(" Bean Bay "), town_name_seed("bean bay"));
    assert_ne!(town_name_seed("Bean Bay"), town_name_seed("Bean Vale"));
    assert_eq!(town_name_seed("Bobville"), 14_812_036_045_316_836_008);
    assert_eq!(safe_town_filename("Bean: Bay?"), "Bean_ Bay_");
    assert_eq!(safe_town_filename("CON"), "Town_CON");
}

#[test]
fn patch_resume_path_overrides_fixed_smoke_path_without_hiding_the_town_catalog() {
    let resume = PathBuf::from(".stream-town/saves/Tonyville.stbevy");
    let fixed = PathBuf::from("generated/smoke.stbevy");
    assert_eq!(startup_save_path(Some(resume.clone()), Some(fixed)), resume);

    let catalog = TownSaveCatalogRuntime::from_startup_paths(None, Some(resume));
    assert_eq!(catalog.active_town.as_deref(), Some("Tonyville"));
    assert!(catalog.fixed_path.is_none());
    assert_eq!(
        startup_destination(false, false, true),
        BootDestination::WorldLoading
    );
    assert!(automatic_load_requested(false, false, true));
    assert!(!automatic_load_requested(true, false, true));
}

#[test]
fn fixed_new_town_path_names_the_active_town_before_the_first_save() {
    let fixed = PathBuf::from(".stream-town/saves/Beanville.stbevy");
    let catalog = TownSaveCatalogRuntime::from_startup_paths(Some(fixed), None);
    assert_eq!(catalog.active_town.as_deref(), Some("Beanville"));
}

#[test]
fn automatic_live_launch_enables_twitch_without_reducing_output_quality() {
    let mut config = GameConfig::default();
    assert!(!config.twitch.enabled);
    assert!(!config.twitch.broadcast.enabled);

    bootstrap::configure_automatic_live(&mut config, Some("public-client-id")).unwrap();

    assert!(config.twitch.enabled);
    assert!(config.twitch.broadcast.enabled);
    assert_eq!(config.twitch.client_id, "public-client-id");
    assert_eq!(config.twitch.broadcast.width, 1_920);
    assert_eq!(config.twitch.broadcast.height, 1_080);
    assert_eq!(config.twitch.broadcast.frames_per_second, 30);
    assert_eq!(config.twitch.broadcast.video_bitrate_kbps, 6_000);
}

#[test]
fn automatic_resume_seed_is_loaded_before_world_generation() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("Tonyville.stbevy");
    let mut config = GameConfig::default();
    config.world.seed = 7_004_668_131_361_051_943;
    let generated = generate_world_with_content(&config.world, &embedded_content());
    let snapshot = WorldSnapshot {
        schema_version: CURRENT_WORLD_SNAPSHOT_SCHEMA,
        world_seed: generated.seed,
        generator_version: generated.generator_version,
        world_hash: generated.deterministic_hash,
        elapsed_seconds: 0,
        actors: Vec::new(),
        simulation: WorldSimulation::new(config.world.seed),
        resource_nodes: BTreeMap::new(),
        traversal_wear: BTreeMap::new(),
        legacy_terrain_mesh: None,
        legacy_migration: None,
    };
    NativeSaveStore::new(&path).write(&snapshot).unwrap();

    assert_eq!(
        automatic_resume_world_seed(Some(&path)),
        Some(config.world.seed)
    );
}

#[test]
fn jump_start_saves_are_identifiable_and_never_reuse_an_existing_path() {
    let directory = tempfile::tempdir().unwrap();
    let active = directory.path().join("Bean Bay.stbevy");
    let first = jump_start_snapshot_path(&active);
    assert_eq!(first.file_name().unwrap(), "Bean Bay.jumpstart.stbevy");
    std::fs::write(&first, b"reserved").unwrap();
    let second = jump_start_snapshot_path(&active);
    assert_eq!(second.file_name().unwrap(), "Bean Bay 2.jumpstart.stbevy");
    assert!(is_jump_start_path(&first));
    assert!(is_jump_start_path(&second));
}

#[test]
fn rotating_save_backups_are_not_listed_as_independent_towns() {
    let directory = tempfile::tempdir().unwrap();
    let current = directory.path().join("Bean Bay.stbevy");
    std::fs::write(&current, b"current").unwrap();
    std::fs::write(directory.path().join("Bean Bay.stbevy.bak"), b"backup 1").unwrap();
    std::fs::write(directory.path().join("Bean Bay.stbevy.bak.2"), b"backup 2").unwrap();
    let catalog = TownSaveCatalogRuntime {
        directory: directory.path().to_path_buf(),
        fixed_path: None,
        active_town: None,
    };
    let entries = catalog.entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].path, current);
}

#[test]
fn new_town_dialog_selects_an_independent_save_and_seed_before_consent() {
    let directory = tempfile::tempdir().unwrap();
    let initial = directory.path().join("initial.stbevy");
    let mut menu = MenuRuntime::default();
    open_new_town_dialog(&mut menu);
    let mut app = App::new();
    app.insert_resource(menu)
        .insert_resource(TownSaveCatalogRuntime {
            directory: directory.path().to_path_buf(),
            fixed_path: None,
            active_town: None,
        })
        .insert_resource(SaveRuntime {
            store: NativeSaveStore::new(initial),
        })
        .insert_resource(RuntimeConfig(GameConfig::default()))
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<InputFocus>()
        .add_systems(Update, town_dialog_buttons);
    app.world_mut()
        .spawn((TownNameField, EditableText::new("Bean Bay")));
    app.world_mut()
        .spawn((Interaction::Pressed, TownDialogAction::Create));

    app.update();

    assert_eq!(
        app.world().resource::<MenuRuntime>().page,
        MenuPage::GoLiveConfirmation
    );
    assert_eq!(
        app.world().resource::<RuntimeConfig>().0.world.seed,
        town_name_seed("Bean Bay")
    );
    assert_eq!(
        app.world()
            .resource::<SaveRuntime>()
            .store
            .path()
            .file_name()
            .unwrap(),
        "Bean Bay.stbevy"
    );
}

#[test]
fn night_waves_repeat_every_two_minutes_and_scale_with_citizens() {
    let mut runtime = NightEnemyWaveRuntime::default();
    assert!(night_enemy_wave_due(&mut runtime, 1, 0.0));
    assert_eq!(runtime.wave_index, 1);
    assert!(!night_enemy_wave_due(&mut runtime, 1, 119.0));
    assert!(night_enemy_wave_due(&mut runtime, 1, 1.0));
    assert_eq!(runtime.wave_index, 2);
    assert!(night_enemy_wave_due(&mut runtime, 2, 0.0));
    assert_eq!(runtime.wave_index, 1);
    assert_eq!(raid_enemies_per_wave(1), 2);
    assert_eq!(raid_enemies_per_wave(2), 2);
    assert_eq!(raid_enemies_per_wave(18), 18);
    assert_eq!(raid_enemies_per_wave(25), 25);
    assert_eq!(raid_enemies_per_wave(100), 25);
}

#[test]
fn community_events_apply_their_authored_role_and_invasion_rates() {
    let prospector = StableId::new("role:prospector").unwrap();
    let forester = StableId::new("role:forester").unwrap();
    let tender = StableId::new("role:tender").unwrap();

    assert_eq!(
        community_event_role_rate_multiplier(CommunityEvent::ProspectingBoom, &prospector),
        3.0
    );
    assert_eq!(
        community_event_role_rate_multiplier(CommunityEvent::ProspectingBoom, &forester),
        0.5
    );
    assert_eq!(
        community_event_role_rate_multiplier(CommunityEvent::ReforestationBoom, &forester),
        3.0
    );
    assert_eq!(
        community_event_role_rate_multiplier(CommunityEvent::AgriculturalBoom, &tender),
        3.0
    );
    assert_eq!(
        community_event_from_id(&StableId::new("event:market").unwrap()),
        Some(CommunityEvent::Market)
    );
    assert_eq!(community_event_name(CommunityEvent::Market), "Market");
    assert_eq!(
        event_adjusted_wave_size(2, Some(CommunityEvent::Invasion)),
        3
    );
    assert_eq!(
        event_adjusted_wave_size(25, Some(CommunityEvent::Invasion)),
        38
    );
    assert_eq!(event_adjusted_wave_size(25, None), 25);

    assert_eq!(
        community_adjusted_regeneration_interval_seconds(
            &forester,
            1,
            Some(CommunityEvent::ReforestationBoom),
        ),
        Some(67.0)
    );
    assert_eq!(
        community_adjusted_regeneration_interval_seconds(
            &forester,
            1,
            Some(CommunityEvent::ProspectingBoom),
        ),
        Some(402.0)
    );
    assert_eq!(
        community_adjusted_regeneration_interval_seconds(
            &tender,
            1,
            Some(CommunityEvent::AgriculturalBoom),
        ),
        Some(1_200.0)
    );

    let config = GameConfig::default();
    let content = embedded_content();
    let mut simulation = WorldSimulation::new(config.world.seed);
    let actor_id = StableId::new("twitch:market-worker").unwrap();
    simulation.join_player(actor_id.clone(), GridPos { x: 1, z: 1 });
    simulation.actors.get_mut(&actor_id).unwrap().role = prospector;
    simulation.active_community_event = Some(CommunityEvent::Market);
    let actor = &simulation.actors[&actor_id];
    assert!(actor_resource_storage_has_room(
        &config,
        &content,
        &simulation,
        actor
    ));
}

#[test]
fn fine_placement_border_allows_a_walkable_gap_but_rejects_physical_encroachment() {
    let mut config = GameConfig::default();
    config.world.width = 16;
    config.world.height = 16;
    let content = embedded_content();
    let world = GeneratedWorld {
        seed: 9,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(
            16,
            16,
            vec![false; 16 * 16],
            vec![100; 16 * 16],
        )
        .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    let guardhouse_id = StableId::new("building:guardhouse").unwrap();
    let definition = &content.buildings[&guardhouse_id];
    let existing_id = StableId::new("building:existing-guardhouse").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.buildings.insert(
        existing_id.clone(),
        BuildingState {
            id: existing_id,
            archetype: definition.archetype.clone(),
            position: GridPos { x: 5, z: 5 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 100,
            complete: true,
        },
    );
    let candidate = |x| BuildingPlacement {
        building: guardhouse_id.clone(),
        thick_path: false,
        position: GridPos { x, z: 5 },
        navigation_position: None,
        rotation_quarter_turns: 0,
        line_start: None,
        line_end: None,
        path_cells: Vec::new(),
        inactivity_seconds: 0.0,
    };
    assert!(building_placement_is_available(
        &content,
        &simulation,
        &world,
        &candidate(7),
        definition,
        None,
    ));
    assert!(!building_placement_is_available(
        &content,
        &simulation,
        &world,
        &candidate(6),
        definition,
        None,
    ));
}

#[test]
fn shipping_building_boxes_fit_model_bounds_and_preserve_special_topology() {
    let config = GameConfig::default();
    let content = embedded_content();
    let fine_cell_size = config.world.cell_size / f32::from(NAVIGATION_SUBDIVISIONS);
    for (building_id, definition) in &content.buildings {
        let placement = definition
            .placement_footprint_thirds
            .expect("shipping buildings author an exact placement footprint");
        let navigation = definition
            .navigation_footprint_thirds
            .expect("shipping buildings author an exact navigation footprint");
        match building_id.as_str() {
            "building:path" => {
                assert_eq!(placement, [1, 1]);
                assert_eq!(navigation, [0, 0]);
            }
            "building:wall" | "building:gate" => {
                assert_eq!(placement, navigation);
                assert_eq!(placement, [1, 3]);
            }
            "building:streetlight" => {
                assert_eq!(placement, [1, 1]);
                assert_eq!(navigation, [1, 1]);
            }
            _ => {
                let bounds = content.archetypes[&definition.archetype].bounds;
                assert!(
                    f32::from(navigation[0]) * fine_cell_size + f32::EPSILON >= bounds.size[0],
                    "{} navigation width does not contain model width {}",
                    building_id,
                    bounds.size[0]
                );
                assert!(
                    f32::from(navigation[1]) * fine_cell_size + f32::EPSILON >= bounds.size[2],
                    "{} navigation depth does not contain model depth {}",
                    building_id,
                    bounds.size[2]
                );
                assert!(
                    f32::from(navigation[0].saturating_sub(1)) * fine_cell_size < bounds.size[0],
                    "{building_id} navigation width is not minimal"
                );
                assert!(
                    f32::from(navigation[1].saturating_sub(1)) * fine_cell_size < bounds.size[2],
                    "{building_id} navigation depth is not minimal"
                );
                assert_eq!(placement, navigation.map(|axis| axis.saturating_add(2)));
            }
        }
    }
}

#[test]
fn town_hall_loss_carries_players_and_roles_into_a_real_world_reload() {
    let mut simulation = WorldSimulation::new(42);
    let player = StableId::new("twitch:survivor").unwrap();
    let enemy = StableId::new("actor:enemy_test").unwrap();
    let role = StableId::new("role:forester").unwrap();
    assert!(simulation.join_player(player.clone(), GridPos { x: 2, z: 3 }));
    simulation.assign_role(&player, role.clone()).unwrap();
    assert!(simulation.spawn_enemy(
        enemy,
        StableId::new("archetype:enemy:test").unwrap(),
        GridPos { x: 9, z: 9 },
        10,
    ));
    let mut app = App::new();
    app.insert_resource(SimulationRuntime(simulation))
        .init_resource::<TownRestartRuntime>()
        .add_systems(Update, restart_world_after_town_hall_falls);

    app.update();

    let restart = app.world().resource::<TownRestartRuntime>();
    let retained = restart.retained_players.as_ref().unwrap();
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].id, player);
    assert_eq!(retained[0].role, role);
    assert!(restart.suppress_exit_save);
    assert!(app.world().contains_resource::<WorldLoadingCoverRuntime>());
}

#[test]
fn night_lights_are_prewarmed_reused_and_disabled_during_the_day() {
    let config = GameConfig::default();
    let mut simulation = WorldSimulation::new(config.world.seed);
    simulation.elapsed_seconds = f64::from(config.time.seconds_per_day) * 0.8;
    let player = StableId::new("twitch:light_test").unwrap();
    assert!(simulation.join_player(player.clone(), GridPos { x: 2, z: 2 }));
    let building_id = StableId::new("building:test-torch").unwrap();
    simulation.buildings.insert(
        building_id.clone(),
        BuildingState {
            id: building_id.clone(),
            archetype: StableId::new("archetype:building:torch").unwrap(),
            position: GridPos { x: 3, z: 3 },
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );
    let mut app = App::new();
    app.insert_resource(RuntimeConfig(config.clone()))
        .insert_resource(SimulationRuntime(simulation))
        .add_systems(Update, sync_pooled_night_lights);
    app.world_mut().spawn((
        Agent {
            id: player,
            kind: ActorKind::Player,
            archetype: StableId::new("archetype:player:test").unwrap(),
            goal: AgentGoal::Wander,
            spawn: GridPos { x: 2, z: 2 },
            origin: GridPos { x: 2, z: 2 },
            navigation_position: placement_to_navigation_centre(GridPos { x: 2, z: 2 }),
            path: Vec::new(),
            path_index: 0,
            target: GridPos { x: 2, z: 2 },
            action_cooldown_seconds: 0.0,
            action_started: false,
            repath_remaining_seconds: 0.0,
            health_regen_accumulator: 0.0,
            wander_sequence: 0,
            previous_wander_origin: None,
        },
        GlobalTransform::from_translation(Vec3::new(1.0, 0.0, 1.0)),
    ));
    app.world_mut().spawn((
        RuntimeBuilding {
            id: building_id.clone(),
        },
        GlobalTransform::from_translation(Vec3::new(3.0, 0.0, 3.0)),
    ));
    app.world_mut().spawn((
        CombatProjectile {
            source: ProjectileSource::Building(building_id),
            target: StableId::new("actor:enemy_test").unwrap(),
            damage: 1,
            speed_cells_per_second: 1.0,
            visual: CombatVisualKind::Fireball,
            trail_cooldown_seconds: 0.0,
            remaining_seconds: 1.0,
        },
        GlobalTransform::from_translation(Vec3::new(2.0, 1.0, 2.0)),
    ));

    app.update();

    let slot_count = app
        .world_mut()
        .query::<&NightPointLightPoolSlot>()
        .iter(app.world())
        .count();
    let visible_at_night = app
        .world_mut()
        .query_filtered::<&Visibility, With<NightPointLightPoolSlot>>()
        .iter(app.world())
        .filter(|visibility| **visibility == Visibility::Visible)
        .count();
    assert_eq!(slot_count, NIGHT_LIGHT_POOL_CAPACITY);
    assert_eq!(visible_at_night, 3);
    assert!(
        building_night_light_profile(
            &StableId::new("archetype:building:house").unwrap(),
            config.world.cell_size,
        )
        .is_none()
    );
    assert!(
        building_night_light_profile(
            &StableId::new("archetype:building:streetlight").unwrap(),
            config.world.cell_size,
        )
        .is_some()
    );

    app.world_mut()
        .resource_mut::<SimulationRuntime>()
        .0
        .elapsed_seconds = f64::from(NIGHT_LIGHT_TRANSITION_SECONDS) + 1.0;
    app.update();
    let slot_count_after_day = app
        .world_mut()
        .query::<&NightPointLightPoolSlot>()
        .iter(app.world())
        .count();
    let visible_during_day = app
        .world_mut()
        .query_filtered::<&Visibility, With<NightPointLightPoolSlot>>()
        .iter(app.world())
        .filter(|visibility| **visibility == Visibility::Visible)
        .count();
    assert_eq!(slot_count_after_day, slot_count);
    assert_eq!(visible_during_day, 0);
}

#[test]
fn night_light_budget_keeps_the_nearest_sources() {
    let spec = |distance: f32| NightLightSpec {
        position: Vec3::new(distance, 0.0, 0.0),
        color: Color::WHITE,
        intensity: 1.0,
        range: 1.0,
        transition_delay_seconds: 0.0,
    };
    let mut candidates = vec![(9.0, spec(3.0)), (1.0, spec(1.0)), (4.0, spec(2.0))];
    let mut selected = Vec::new();
    append_nearest_night_lights(&mut selected, &mut candidates, 2);
    assert_eq!(selected.len(), 2);
    assert!((selected[0].position.x - 1.0).abs() < f32::EPSILON);
    assert!((selected[1].position.x - 2.0).abs() < f32::EPSILON);
}

#[test]
fn night_lights_stagger_activation_and_deactivation_over_ten_seconds() {
    let config = GameConfig::default().time;
    let night_start =
        f64::from(config.seconds_per_day) * f64::from(config.daylight_per_thousand) / 1_000.0;
    assert!(!night_light_is_active(&config, night_start + 4.9, 5.0));
    assert!(night_light_is_active(&config, night_start + 5.0, 5.0));
    assert!(night_light_is_active(&config, 4.9, 5.0));
    assert!(!night_light_is_active(&config, 5.0, 5.0));
    assert!(!night_light_sources_are_inactive(
        &config,
        f64::from(NIGHT_LIGHT_TRANSITION_SECONDS) - 0.1,
    ));
    assert!(night_light_sources_are_inactive(
        &config,
        f64::from(NIGHT_LIGHT_TRANSITION_SECONDS),
    ));
    assert!(!night_light_sources_are_inactive(&config, night_start,));
    assert!(night_light_transition_delay(0) >= 0.0);
    assert!(night_light_transition_delay(u64::MAX) <= NIGHT_LIGHT_TRANSITION_SECONDS);
}

#[test]
fn surface_grounding_places_rotated_bounds_on_the_surface() {
    let rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let minimum = transformed_bounds_minimum_y(
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(0.5, 2.0, 0.25),
        rotation,
        Vec3::splat(2.0),
    );
    assert!((minimum + 0.5).abs() < 0.0001);
    let surface = 7.0;
    let translation_y = surface - minimum;
    assert!((translation_y + minimum - surface).abs() < 0.0001);
}

#[test]
fn surface_grounding_uses_real_vertices_instead_of_empty_aabb_corners() {
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            [2.0, 1.0, 0.0],
            [1.0, 2.0, 0.0],
        ],
    );
    let rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_4);
    let exact = transformed_mesh_vertical_extent(&mesh, rotation, Vec3::splat(2.0)).unwrap();
    let conservative = transformed_bounds_minimum_y(
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        rotation,
        Vec3::splat(2.0),
    );
    assert!(exact.0 > conservative + 1.0);
    assert!(exact.1 >= exact.0);
}

#[test]
fn foliage_mesh_habitat_rejects_shore_overhang_and_emergent_underwater_grass() {
    let mut config = GameConfig::default();
    config.world.width = 3;
    config.world.height = 3;
    config.world.cell_size = 2.0;
    config.world.water_level_centimetres = 5;
    let world = GeneratedWorld {
        seed: 1,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(
            3,
            3,
            vec![false; 9],
            vec![100, 100, -100, 100, 100, -100, 100, 100, -100],
        )
        .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // The only base vertex is on dry land, but the upper foliage leans
        // over a submerged cell. The complete rendered footprint must be
        // classified, not just its root/base band.
        vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
    );
    let land = Transform::from_xyz(0.0, 1.0, 0.0);
    assert!(!foliage_mesh_fits_habitat(
        &mesh,
        &land,
        (0.0, 1.0),
        FoliageHabitat::Land,
        &config,
        &world,
    ));
    let compact_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.5, 0.0]],
    );
    assert!(!foliage_mesh_fits_habitat(
        &compact_mesh,
        &land,
        (0.0, 1.0),
        FoliageHabitat::Land,
        &config,
        &world,
    ));
    let underwater = Transform::from_xyz(0.0, -0.5, 0.0);
    assert!(!foliage_mesh_fits_habitat(
        &mesh,
        &underwater,
        (0.0, 1.0),
        FoliageHabitat::Underwater,
        &config,
        &world,
    ));
}

#[test]
fn surface_visual_remains_hidden_until_grounding_is_complete() {
    let mut app = App::new();
    app.add_systems(Update, ground_loaded_surface_visuals);
    let bounds = Aabb::from_min_max(Vec3::new(-0.5, 0.0, -0.25), Vec3::new(0.5, 4.0, 0.25));
    let rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let scale = Vec3::splat(2.0);
    let surface = 7.0;
    let entity = app
        .world_mut()
        .spawn((
            PendingSurfaceGrounding {
                surface_height: surface,
            },
            bounds,
            Transform::from_rotation(rotation).with_scale(scale),
            Visibility::Hidden,
        ))
        .id();

    app.update();

    let grounded = app.world().entity(entity);
    assert!(grounded.get::<PendingSurfaceGrounding>().is_none());
    assert_eq!(grounded.get::<Visibility>(), Some(&Visibility::Inherited));
    let transform = grounded.get::<Transform>().unwrap();
    let minimum = transformed_bounds_minimum_y(
        Vec3::from(bounds.center),
        Vec3::from(bounds.half_extents),
        transform.rotation,
        transform.scale,
    );
    assert!((transform.translation.y + minimum - surface).abs() < 0.0001);
}

#[test]
fn runtime_foliage_waits_for_world_before_habitat_classification() {
    let mut config = GameConfig::default();
    config.world.width = 3;
    config.world.height = 3;
    config.world.cell_size = 2.0;
    config.world.water_level_centimetres = 5;
    let world = GeneratedWorld {
        seed: 1,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(
            3,
            3,
            vec![false; 9],
            vec![100, 100, -100, 100, 100, -100, 100, 100, -100],
        )
        .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
    );
    let bounds = Aabb::from_min_max(Vec3::ZERO, Vec3::new(2.0, 1.0, 0.0));
    let mut app = App::new();
    app.insert_resource(Assets::<Mesh>::default())
        .add_systems(Update, ground_loaded_surface_visuals);
    let mesh = app.world_mut().resource_mut::<Assets<Mesh>>().add(mesh);
    let entity = app
        .world_mut()
        .spawn((
            Mesh3d(mesh),
            RuntimeTerrainGrounding,
            SurfaceFoliageHabitat(FoliageHabitat::Land),
            PendingSurfaceGrounding {
                surface_height: 1.0,
            },
            bounds,
            Transform::from_xyz(0.0, 1.0, 0.0),
            Visibility::Hidden,
        ))
        .id();

    app.update();

    let waiting = app.world().entity(entity);
    assert!(waiting.get::<PendingSurfaceGrounding>().is_some());
    assert_eq!(waiting.get::<Visibility>(), Some(&Visibility::Hidden));

    app.world_mut().insert_resource(RuntimeConfig(config));
    app.world_mut().insert_resource(WorldRuntime {
        generated: world,
    });
    app.update();
    assert!(app.world().get_entity(entity).is_err());
}

#[test]
fn terrain_conforming_grounding_never_leaves_a_base_vertex_in_the_air() {
    let mut config = GameConfig::default();
    config.world.width = 2;
    config.world.height = 2;
    config.world.cell_size = 2.0;
    let world = GeneratedWorld {
        seed: 1,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(2, 2, vec![false; 4], vec![0, 100, 300, 900])
            .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-0.5, 0.0, 0.0], [0.5, 0.0, 0.0], [0.0, 1.0, 0.0]],
    );
    let mut transform = Transform::default();
    let extent =
        transformed_mesh_vertical_extent(&mesh, transform.rotation, transform.scale).unwrap();
    transform.translation.y =
        terrain_conforming_translation_y(&mesh, &transform, extent, &config, &world).unwrap();
    for x in [-0.5, 0.5] {
        let surface = terrain_surface_height_at_world(&world, &config, x, 0.0).unwrap();
        assert!(transform.translation.y <= surface);
    }
}

#[test]
fn foliage_surface_height_matches_the_rendered_terrain_triangles() {
    let mut config = GameConfig::default();
    config.world.width = 2;
    config.world.height = 2;
    config.world.cell_size = 2.0;
    let world = GeneratedWorld {
        seed: 1,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(2, 2, vec![false; 4], vec![0, 100, 300, 900])
            .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };

    for (x, z, expected) in [
        (-1.0, -1.0, 0.0),
        (1.0, -1.0, 0.5),
        (-1.0, 1.0, 1.5),
        (1.0, 1.0, 3.25),
        (-0.5, -0.5, 0.5),
        (0.5, 0.5, 2.125),
    ] {
        let actual = terrain_surface_height_at_world(&world, &config, x, z).unwrap();
        assert!(
            (actual - expected).abs() < 0.0001,
            "surface at ({x}, {z}) was {actual}, expected {expected}"
        );
    }
    assert!(terrain_surface_height_at_world(&world, &config, 1.01, 0.0).is_none());
}

#[test]
fn resource_pivot_offsets_are_resampled_and_bounds_grounded_at_final_xz() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let resource = world
        .resources
        .iter()
        .find(|resource| resource.kind.as_str() == "resource:wood")
        .expect("shipping world has trees");
    let archetype = resource_visual_archetype(&content, &resource.kind).unwrap();
    let generated = generated_resource_world_position(resource, &config, &world);
    let grounded = grounded_resource_visual_position(generated, Some(archetype), &config, &world);
    let expected = terrain_surface_height_at_world(&world, &config, grounded.x, grounded.z)
        .expect("resource pivot remains inside terrain");
    assert!((grounded.y - expected).abs() < 0.0001);
}

#[test]
fn foliage_habitat_is_checked_after_offset_at_the_visible_surface() {
    let mut config = GameConfig::default();
    config.world.width = 3;
    config.world.height = 3;
    config.world.cell_size = 2.0;
    config.world.water_level_centimetres = 5;
    let foliage = |habitat| GeneratedFoliage {
        id: StableId::new(match habitat {
            FoliageHabitat::Land => "foliage:test:land",
            FoliageHabitat::Underwater => "foliage:test:underwater",
        })
        .unwrap(),
        layer: StableId::new("foliage:test").unwrap(),
        habitat,
        position: GridPos { x: 1, z: 1 },
        offset_milli_cells: [0, 0],
        variant: 0,
        yaw_milliradians: 0,
        scale_milli: 1_000,
    };
    let flat_world = |height_centimetres| GeneratedWorld {
        seed: 7,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(
            3,
            3,
            vec![false; 9],
            vec![height_centimetres; 9],
        )
        .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };

    let dry_world = flat_world(100);
    let land =
        resolved_foliage_ground_position(&dry_world, &config, &foliage(FoliageHabitat::Land))
            .unwrap();
    assert!((land.y - 1.0).abs() < 0.0001);
    assert!(
            resolved_foliage_ground_position(
                &dry_world,
                &config,
                &foliage(FoliageHabitat::Underwater),
            )
            .is_none()
        );

    let submerged_world = flat_world(-100);
    let underwater = resolved_foliage_ground_position(
        &submerged_world,
        &config,
        &foliage(FoliageHabitat::Underwater),
    )
    .unwrap();
    assert!((underwater.y + 1.0).abs() < 0.0001);
    assert!(
            resolved_foliage_ground_position(
                &submerged_world,
                &config,
                &foliage(FoliageHabitat::Land),
            )
            .is_none()
        );
}

#[test]
fn player_night_lights_normalize_colour_luminance_then_apply_total_levels() {
    for color in [[255, 0, 0], [0, 255, 0], [0, 0, 255], [255, 209, 143]] {
        let linear = perceptually_normalized_light_color(color)
            .to_linear()
            .to_f32_array();
        let luminance = linear[0] * 0.2126 + linear[1] * 0.7152 + linear[2] * 0.0722;
        assert!((luminance - 1.0).abs() < 0.0001, "{color:?}");
    }

    let mut simulation = WorldSimulation::new(7);
    let actor_id = StableId::new("twitch:level_light").unwrap();
    assert!(simulation.join_player(actor_id.clone(), GridPos { x: 1, z: 1 }));
    let actor = simulation.actors.get_mut(&actor_id).unwrap();
    actor.role_progression.clear();
    assert!((player_night_light_level_multiplier(actor) - 1.0).abs() < f32::EPSILON);
    actor.role_progression.insert(
        StableId::new("role:logger").unwrap(),
        stream_town_domain::RoleProgress {
            level: 25,
            experience: 0,
        },
    );
    actor.role_progression.insert(
        StableId::new("role:miner").unwrap(),
        stream_town_domain::RoleProgress {
            level: 25,
            experience: 0,
        },
    );
    assert!((player_night_light_level_multiplier(actor) - 2.0).abs() < f32::EPSILON);
    actor.role_progression.values_mut().for_each(|progress| {
        progress.level = u16::MAX;
    });
    assert!(player_night_light_level_multiplier(actor) < 3.0);
}
