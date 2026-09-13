#[test]
fn native_load_moves_town_hall_visual_to_saved_footprint() {
    let config = GameConfig::default();
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("moved-town-hall.stbevy");
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(config.clone()))
    .add_plugins(StreamTownGamePlugin);
    app.insert_resource(SaveRuntime {
        store: NativeSaveStore::new(&save_path),
    });

    app.update();
    app.update();
    enter_headless_world(&mut app);
    app.world_mut().resource_mut::<MenuIoRequest>().save = true;
    app.update();

    let store = NativeSaveStore::new(&save_path);
    let mut snapshot = store.load().unwrap();
    let town_hall_id = StableId::new("building:townhall").unwrap();
    let saved_position = GridPos { x: 6, z: 8 };
    snapshot
        .simulation
        .buildings
        .get_mut(&town_hall_id)
        .unwrap()
        .position = saved_position;
    let relocated_actor_id = snapshot.actors[0].id.clone();
    snapshot.actors[0].grid_position = saved_position;
    snapshot
        .simulation
        .actors
        .get_mut(&relocated_actor_id)
        .unwrap()
        .position = saved_position;
    store.write(&snapshot).unwrap();

    app.world_mut().resource_mut::<MenuIoRequest>().load = true;
    app.update();

    let content = &app.world().resource::<RuntimeContent>().0;
    let footprint = content.buildings[&town_hall_id].footprint;
    let centre = GridPos {
        x: saved_position.x + footprint[0] / 2,
        z: saved_position.z + footprint[1] / 2,
    };
    let expected_surface = grid_to_world_on_surface(
        centre,
        &config,
        &app.world().resource::<WorldRuntime>().generated,
    );
    let (location, presentation, transform) = app
        .world_mut()
        .query_filtered::<(&GridLocation, &BuildingPresentation, &Transform), With<TownHall>>()
        .single(app.world())
        .unwrap();
    assert_eq!(location.0, saved_position);
    assert_eq!(transform.translation.xz(), expected_surface.xz());
    assert_eq!(transform.translation, presentation.base_translation);
    assert_eq!(
        app.world().resource::<SimulationRuntime>().0.buildings[&town_hall_id].position,
        saved_position
    );
    let restored_station = station_candidate(
        &app.world().resource::<RuntimeContent>().0,
        &app.world().resource::<SimulationRuntime>().0,
        &config,
        &town_hall_id,
    )
    .unwrap();
    assert_eq!(restored_station.position, centre);
    assert_eq!(
        restored_town_hall_position(
            &app.world().resource::<RuntimeContent>().0,
            &app.world().resource::<SimulationRuntime>().0,
            &config,
        ),
        centre
    );
    assert_ne!(centre, town_hall_grid_position(&config));
    let (relocated_ecs_position, relocated_navigation_position) = app
        .world_mut()
        .query::<(&Agent, &GridLocation)>()
        .iter(app.world())
        .find(|(agent, _)| agent.id == relocated_actor_id)
        .map(|(agent, location)| (location.0, agent.navigation_position))
        .unwrap();
    assert_ne!(relocated_ecs_position, saved_position);
    assert_eq!(
        relocated_navigation_position,
        placement_to_navigation_centre(relocated_ecs_position)
    );
    assert_eq!(
        app.world().resource::<SimulationRuntime>().0.actors[&relocated_actor_id].position,
        relocated_ecs_position
    );
    let region = building_region(
        saved_position,
        footprint,
        &app.world().resource::<WorldRuntime>().generated,
    )
    .unwrap();
    for z in region.min.z..=region.max.z {
        for x in region.min.x..=region.max.x {
            assert!(
                !app.world()
                    .resource::<WorldRuntime>()
                    .generated
                    .navigation
                    .is_walkable(GridPos { x, z })
            );
        }
    }
}

#[test]
fn native_load_restores_saved_world_seed_after_runtime_config_changes() {
    let config = GameConfig::default();
    let saved_seed = config.world.seed;
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("saved-seed.stbevy");
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
    app.world_mut().resource_mut::<MenuIoRequest>().save = true;
    app.update();

    let saved = NativeSaveStore::new(&save_path).load().unwrap();
    let saved_wood = town_resource_amount(&saved.simulation, "resource:wood");
    app.world_mut().resource_mut::<RuntimeConfig>().0.world.seed = saved_seed.wrapping_add(1);
    app.world_mut()
        .resource_mut::<SimulationRuntime>()
        .0
        .town_resources
        .insert(StableId::new("resource:wood").unwrap(), 0);
    app.world_mut().resource_mut::<MenuIoRequest>().load = true;
    app.update();

    assert_eq!(
        app.world().resource::<WorldRuntime>().generated.seed,
        saved_seed
    );
    assert_eq!(
        app.world().resource::<RuntimeConfig>().0.world.seed,
        saved_seed
    );
    assert_eq!(
        app.world().resource::<SimulationRuntime>().0.world_seed,
        saved_seed
    );
    assert_eq!(
        town_resource_amount(
            &app.world().resource::<SimulationRuntime>().0,
            "resource:wood"
        ),
        saved_wood
    );
    assert_eq!(
        app.world().resource::<RuntimeConsoleRuntime>().last_result,
        format!("Loaded {}", save_path.display())
    );
}

#[test]
fn native_load_preflight_preserves_live_town_on_invalid_building() {
    let config = GameConfig::default();
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("invalid-building.stbevy");
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
    app.world_mut().resource_mut::<MenuIoRequest>().save = true;
    app.update();

    let store = NativeSaveStore::new(&save_path);
    let mut snapshot = store.load().unwrap();
    let invalid_id = StableId::new("building:invalid_save_fixture").unwrap();
    snapshot.simulation.buildings.insert(
        invalid_id.clone(),
        BuildingState {
            id: invalid_id,
            archetype: StableId::new("archetype:building:missing").unwrap(),
            position: GridPos { x: 2, z: 2 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 100,
            complete: true,
        },
    );
    store.write(&snapshot).unwrap();

    let live_wood = 73;
    app.world_mut()
        .resource_mut::<SimulationRuntime>()
        .0
        .town_resources
        .insert(StableId::new("resource:wood").unwrap(), live_wood);
    let placer_owner = StableId::new("debug:preflight_owner").unwrap();
    app.world_mut().resource_mut::<BuildingPlacers>().0.insert(
        placer_owner.clone(),
        BuildingPlacement {
            building: StableId::new("building:wall").unwrap(),
            thick_path: false,
            position: GridPos { x: 3, z: 3 },
            navigation_position: None,
            rotation_quarter_turns: 0,
            line_start: None,
            line_end: None,
            path_cells: Vec::new(),
            inactivity_seconds: 0.0,
        },
    );
    let live_actor_ids = app
        .world_mut()
        .query::<&Agent>()
        .iter(app.world())
        .map(|agent| agent.id.clone())
        .collect::<BTreeSet<_>>();
    let live_world_hash = app
        .world()
        .resource::<WorldRuntime>()
        .generated
        .deterministic_hash
        .clone();

    app.world_mut().resource_mut::<MenuIoRequest>().load = true;
    app.update();

    let actor_ids_after = app
        .world_mut()
        .query::<&Agent>()
        .iter(app.world())
        .map(|agent| agent.id.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(actor_ids_after, live_actor_ids);
    assert_eq!(
        app.world()
            .resource::<WorldRuntime>()
            .generated
            .deterministic_hash,
        live_world_hash
    );
    assert_eq!(
        town_resource_amount(
            &app.world().resource::<SimulationRuntime>().0,
            "resource:wood"
        ),
        live_wood
    );
    assert!(
        app.world()
            .resource::<BuildingPlacers>()
            .0
            .contains_key(&placer_owner)
    );
    assert!(
        app.world()
            .resource::<RuntimeConsoleRuntime>()
            .last_result
            .contains("references unknown archetype")
    );
}

#[test]
fn headless_vertical_slice_spawns_three_hundred_agents() {
    let mut config = GameConfig::default();
    config.gameplay.initial_agents = 300;
    let expected = usize::from(config.gameplay.initial_agents);
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("command-save.stbevy");
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

    let actual = app
        .world_mut()
        .query_filtered::<Entity, With<Agent>>()
        .iter(app.world())
        .count();
    assert_eq!(actual, expected);
    assert!(app.world().contains_resource::<WorldRuntime>());

    let enemies = app
        .world_mut()
        .query::<&Agent>()
        .iter(app.world())
        .filter(|agent| agent.kind == ActorKind::Enemy)
        .count();
    assert_eq!(enemies, 0);
    assert!(
        !app.world()
            .resource::<SimulationRuntime>()
            .0
            .enemy_camps
            .is_empty()
    );

    app.world_mut()
        .resource_mut::<InjectedCommands>()
        .0
        .push_back(PendingChatCommand {
            actor_id: StableId::new("twitch:debug_viewer").unwrap(),
            login_name: "debug_viewer".to_owned(),
            display_name: "debug_viewer".to_owned(),
            command: "!join".parse().unwrap(),
            is_broadcaster: true,
            is_moderator: true,
            is_subscriber: true,
            origin: CommandOrigin::LocalDebug,
        });
    app.update();
    let joined_count = app
        .world_mut()
        .query_filtered::<Entity, With<Agent>>()
        .iter(app.world())
        .count();
    assert_eq!(joined_count, expected + 1);
    assert!(
        app.world()
            .resource::<SimulationRuntime>()
            .0
            .actors
            .contains_key(&StableId::new("twitch:debug_viewer").unwrap())
    );
    assert_eq!(
        app.world().resource::<SimulationRuntime>().0.actors
            [&StableId::new("twitch:debug_viewer").unwrap()]
            .active_pet
            .as_ref()
            .map(StableId::as_str),
        Some("pet:red_panda")
    );

    let gm_commands = [
        ChatCommand::ToggleBuildCosts,
        ChatCommand::ToggleRoleLimits,
        ChatCommand::AddResource {
            resource: StableId::new("wood").unwrap(),
            amount: 123,
        },
        ChatCommand::GivePet {
            player: StableId::new("debug_viewer").unwrap(),
            pet: StableId::new("duck").unwrap(),
        },
        ChatCommand::LevelUpPlayer {
            player: StableId::new("debug_viewer").unwrap(),
            amount: 2,
        },
        ChatCommand::QueueEvent(StableId::new("fishgod").unwrap()),
    ];
    for command in gm_commands {
        app.world_mut()
            .resource_mut::<InjectedCommands>()
            .0
            .push_back(PendingChatCommand {
                actor_id: StableId::new("twitch:debug_viewer").unwrap(),
                login_name: "debug_viewer".to_owned(),
                display_name: "debug_viewer".to_owned(),
                command,
                is_broadcaster: false,
                is_moderator: false,
                is_subscriber: true,
                origin: CommandOrigin::LocalDebug,
            });
    }
    app.update();
    {
        let simulation = &app.world().resource::<SimulationRuntime>().0;
        assert!(!simulation.building_costs_enabled);
        assert!(!simulation.role_limits_enabled);
        assert_eq!(town_resource_amount(simulation, "resource:wood"), 5_123);
        assert!(
            !simulation.actors[&StableId::new("twitch:debug_viewer").unwrap()]
                .unlocked_pets
                .contains(&StableId::new("pet:duck").unwrap()),
            "recognized pet commands must remain explicit no-ops until pets are implemented"
        );
        assert_eq!(
            simulation.queued_events.len() + usize::from(simulation.fish_god.is_some()),
            1
        );
    }
    app.update();
    let simulation = &app.world().resource::<SimulationRuntime>().0;
    assert!(
        simulation.fish_god.is_some(),
        "queued={:?}, active={:?}, game_state={:?}",
        simulation.queued_events,
        simulation.active_event,
        app.world().resource::<State<GameState>>().get(),
    );
    for command in [
        ChatCommand::GameEventAction,
        ChatCommand::StopEvent,
        ChatCommand::AddResource {
            resource: StableId::new("wood").unwrap(),
            amount: -123,
        },
        ChatCommand::ToggleBuildCosts,
        ChatCommand::ToggleRoleLimits,
    ] {
        app.world_mut()
            .resource_mut::<InjectedCommands>()
            .0
            .push_back(PendingChatCommand {
                actor_id: StableId::new("twitch:debug_viewer").unwrap(),
                login_name: "debug_viewer".to_owned(),
                display_name: "debug_viewer".to_owned(),
                command,
                is_broadcaster: false,
                is_moderator: false,
                is_subscriber: true,
                origin: CommandOrigin::LocalDebug,
            });
    }
    app.update();
    {
        let simulation = &app.world().resource::<SimulationRuntime>().0;
        assert!(simulation.building_costs_enabled);
        assert!(simulation.role_limits_enabled);
        assert_eq!(town_resource_amount(simulation, "resource:wood"), 5_000);
        assert!(simulation.fish_god.is_none());
        assert!(simulation.active_event.is_none());
    }

    let available_building = {
        let content = &app.world().resource::<RuntimeContent>().0;
        let simulation = &app.world().resource::<SimulationRuntime>().0;
        content
            .buildings
            .iter()
            .find(|(id, building)| {
                building.placeable && building_is_unlocked(content, simulation, id)
            })
            .map(|(id, building)| {
                (
                    id.clone(),
                    building.clone(),
                    building_construction_cost(content, simulation, id, building),
                )
            })
            .expect("converted initial technology unlocks a placeable building")
    };
    let actor_id = StableId::new("twitch:debug_viewer").unwrap();
    {
        let site = {
            let simulation = &app.world().resource::<SimulationRuntime>().0;
            let world = &app.world().resource::<WorldRuntime>().generated;
            let content = &app.world().resource::<RuntimeContent>().0;
            let near = simulation.actors[&actor_id].position;
            let mut candidates = Vec::new();
            for z in 0..world.navigation.height() {
                for x in 0..world.navigation.width() {
                    let position = GridPos { x, z };
                    let placement = BuildingPlacement {
                        building: available_building.0.clone(),
                        thick_path: false,
                        position,
                        navigation_position: None,
                        rotation_quarter_turns: 0,
                        line_start: None,
                        line_end: None,
                        path_cells: Vec::new(),
                        inactivity_seconds: 0.0,
                    };
                    if building_placement_is_available(
                        content,
                        simulation,
                        world,
                        &placement,
                        &available_building.1,
                        None,
                    ) {
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
            candidates
                .into_iter()
                .next()
                .expect("vertical slice has an exact fine-grid building site")
        };
        app.world_mut()
            .resource_mut::<SimulationRuntime>()
            .0
            .actors
            .get_mut(&actor_id)
            .unwrap()
            .last_building_position = Some(site);
    }
    let commands = [
        ChatCommand::SelectRole(StableId::new("builder").unwrap()),
        ChatCommand::Experience,
        ChatCommand::Build(available_building.0.clone()),
        ChatCommand::MoveBuilding(vec![BuildingAction {
            direction: BuildingDirection::Rotate,
            amount: 4,
        }]),
        ChatCommand::ConfirmBuilding,
        ChatCommand::DismissRecruit(5),
        ChatCommand::DismissRecruit(4),
        ChatCommand::Recruit {
            role: StableId::new("miner").unwrap(),
            amount: 2,
        },
        ChatCommand::Role,
        ChatCommand::Health,
        ChatCommand::Target(None),
        ChatCommand::Station(None),
        ChatCommand::Customize {
            kind: CustomizationKind::Body,
            index: 3,
        },
        ChatCommand::Camera(vec![CameraAction {
            direction: CameraDirection::In,
            amount: 1,
        }]),
        ChatCommand::TechnologyVote,
        ChatCommand::Vote(StableId::new("1").unwrap()),
        ChatCommand::TriggerEvent(StableId::new("festival").unwrap()),
        ChatCommand::Ping,
        ChatCommand::Ping,
        ChatCommand::Save,
        ChatCommand::Help,
    ];
    for command in commands {
        app.world_mut()
            .resource_mut::<InjectedCommands>()
            .0
            .push_back(PendingChatCommand {
                actor_id: actor_id.clone(),
                login_name: "debug_viewer".to_owned(),
                display_name: "debug_viewer".to_owned(),
                command,
                is_broadcaster: true,
                is_moderator: true,
                is_subscriber: true,
                origin: CommandOrigin::LocalDebug,
            });
    }
    app.update();

    let pointer_actors = app
        .world_mut()
        .query::<&PingPointer>()
        .iter(app.world())
        .map(|pointer| pointer.actor.clone())
        .collect::<Vec<_>>();
    assert_eq!(pointer_actors, vec![actor_id.clone()]);
    {
        let mut query = app.world_mut().query::<&mut PingPointer>();
        query
            .single_mut(app.world_mut())
            .expect("deduplicated pointer exists")
            .elapsed_seconds = PING_POINTER_DURATION_SECONDS;
    }
    app.update();
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<PingPointer>>()
            .iter(app.world())
            .count(),
        0
    );

    let (placed_building, saved_building_id, food_before_revive) = {
        let simulation = &app.world().resource::<SimulationRuntime>().0;
        assert_eq!(simulation.actors[&actor_id].role.as_str(), "role:builder");
        assert_eq!(
            simulation
                .actors
                .keys()
                .filter(|id| id.as_str().starts_with("npc:recruit_"))
                .count(),
            2
        );
        assert_eq!(recruited_actor_ids(simulation).len(), 5);
        assert_eq!(town_resource_amount(simulation, "resource:recruit"), 5);
        assert_eq!(simulation.actors[&actor_id].customization.body_type, 2);
        assert_eq!(
            simulation.actors[&actor_id].building_rotation_quarter_turns,
            4
        );
        assert!(
            simulation.actors[&actor_id]
                .last_building_position
                .is_some()
        );
        assert_eq!(simulation.buildings.len(), 2);
        let placed_building = simulation
            .buildings
            .values()
            .find(|building| building.id.as_str() != "building:townhall")
            .unwrap()
            .clone();
        assert!(!placed_building.complete);
        let base_max_health = building_base_max_health(
            &app.world().resource::<RuntimeContent>().0,
            &available_building.1,
        );
        assert_eq!(
            placed_building.health,
            i32::try_from(base_max_health.div_ceil(10)).unwrap()
        );
        assert_eq!(placed_building.rotation_quarter_turns, 4);
        for resource in [
            "resource:food",
            "resource:gold",
            "resource:ore",
            "resource:wood",
        ] {
            let resource_id = StableId::new(resource).unwrap();
            assert_eq!(
                town_resource_amount(simulation, resource),
                5_000 - available_building.2[&resource_id]
            );
        }
        let vote = simulation
            .active_vote
            .as_ref()
            .expect("technology vote active");
        assert_eq!(vote.options.len(), TECHNOLOGY_VOTE_OPTION_COUNT);
        assert_eq!(vote.option_votes.get(&actor_id), vote.options.first());
        assert_eq!(simulation.active_event, Some(TownEvent::Festival));
        let saved_building_id = placed_building.id.clone();
        (
            placed_building,
            saved_building_id,
            town_resource_amount(simulation, "resource:food"),
        )
    };
    let mut runtime_building_ids: Vec<_> = app
        .world_mut()
        .query::<&RuntimeBuilding>()
        .iter(app.world())
        .map(|building| building.id.clone())
        .collect();
    runtime_building_ids.sort();
    let mut expected_runtime_buildings = vec![
        StableId::new("building:townhall").unwrap(),
        saved_building_id.clone(),
    ];
    expected_runtime_buildings.sort();
    assert_eq!(runtime_building_ids, expected_runtime_buildings);
    assert!(
        save_path.is_file(),
        "{}",
        app.world().resource::<RuntimeConsoleRuntime>().last_result
    );
    let saved = NativeSaveStore::new(&save_path).load().unwrap();
    assert_eq!(
        saved
            .actors
            .iter()
            .map(|actor| actor.id.clone())
            .collect::<BTreeSet<_>>(),
        saved
            .simulation
            .actors
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>()
    );
    assert_eq!(
        saved.simulation.buildings[&saved_building_id],
        placed_building
    );
    assert!(
        app.world()
            .resource::<CommandFeedback>()
            .0
            .contains(TWITCH_COMMAND_HELP_URL)
    );
    app.world_mut()
        .resource_mut::<SimulationRuntime>()
        .0
        .damage_actor(&actor_id, u32::MAX)
        .unwrap();
    app.world_mut()
        .resource_mut::<InjectedCommands>()
        .0
        .push_back(PendingChatCommand {
            actor_id: actor_id.clone(),
            login_name: "debug_viewer".to_owned(),
            display_name: "debug_viewer".to_owned(),
            command: ChatCommand::Revive(None),
            is_broadcaster: true,
            is_moderator: true,
            is_subscriber: true,
            origin: CommandOrigin::LocalDebug,
        });
    app.update();
    let simulation = &app.world().resource::<SimulationRuntime>().0;
    assert!(simulation.actors[&actor_id].alive);
    assert_eq!(
        town_resource_amount(simulation, "resource:food"),
        food_before_revive - 400
    );
    assert!(
        app.world()
            .resource::<CommandFeedback>()
            .0
            .contains("revived twitch:debug_viewer")
    );
    let revival_rings = app
        .world_mut()
        .query::<&HealingRingEffect>()
        .iter(app.world())
        .filter(|effect| effect.kind == HealingEffectKind::Revive)
        .count();
    let revival_mote_bars = app
        .world_mut()
        .query::<&HealingMoteEffect>()
        .iter(app.world())
        .filter(|effect| effect.kind == HealingEffectKind::Revive)
        .count();
    assert_eq!(revival_rings, 1);
    assert_eq!(revival_mote_bars, 8);

    {
        let technology_ids = app
            .world()
            .resource::<RuntimeContent>()
            .0
            .technology
            .nodes
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        let max_health = {
            let content = &app.world().resource::<RuntimeContent>().0;
            let simulation = &app.world().resource::<SimulationRuntime>().0;
            building_max_health(content, &simulation.buildings[&saved_building_id])
        };
        let mut simulation = app.world_mut().resource_mut::<SimulationRuntime>();
        let building = simulation.0.buildings.get_mut(&saved_building_id).unwrap();
        building.complete = true;
        building.health = max_health;
        simulation.0.unlocked_technology.extend(technology_ids);
        for amount in simulation.0.town_resources.values_mut() {
            *amount = 1_000_000;
        }
    }
    for command in [
        ChatCommand::LevelBuilding {
            building: available_building.0.clone(),
            index: 1,
            iterations: 2,
        },
        ChatCommand::LevelAll {
            building: available_building.0.clone(),
            target_level: 4,
        },
    ] {
        app.world_mut()
            .resource_mut::<InjectedCommands>()
            .0
            .push_back(PendingChatCommand {
                actor_id: actor_id.clone(),
                login_name: "debug_viewer".to_owned(),
                display_name: "debug_viewer".to_owned(),
                command,
                is_broadcaster: false,
                is_moderator: false,
                is_subscriber: true,
                origin: CommandOrigin::LocalDebug,
            });
    }
    app.update();
    assert!(app.world().resource::<SimulationRuntime>().0.buildings[&saved_building_id].level >= 2);
    let removed_position =
        app.world().resource::<SimulationRuntime>().0.buildings[&saved_building_id].position;

    // Staff privileges must not bypass the Ruler-only removal rule.
    app.world_mut()
        .resource_mut::<InjectedCommands>()
        .0
        .push_back(PendingChatCommand {
            actor_id: actor_id.clone(),
            login_name: "debug_viewer".to_owned(),
            display_name: "debug_viewer".to_owned(),
            command: ChatCommand::RemoveBuilding {
                building: available_building.0.clone(),
                index: 1,
            },
            is_broadcaster: true,
            is_moderator: true,
            is_subscriber: true,
            origin: CommandOrigin::LocalDebug,
        });
    app.update();
    assert!(
        app.world()
            .resource::<SimulationRuntime>()
            .0
            .buildings
            .contains_key(&saved_building_id)
    );

    app.world_mut()
        .resource_mut::<SimulationRuntime>()
        .0
        .current_ruler = Some(actor_id.clone());
    app.world_mut()
        .resource_mut::<InjectedCommands>()
        .0
        .push_back(PendingChatCommand {
            actor_id,
            login_name: "debug_viewer".to_owned(),
            display_name: "debug_viewer".to_owned(),
            command: ChatCommand::RemoveBuilding {
                building: available_building.0,
                index: 1,
            },
            is_broadcaster: false,
            is_moderator: false,
            is_subscriber: true,
            origin: CommandOrigin::LocalDebug,
        });
    app.update();
    assert_eq!(
        app.world()
            .resource::<SimulationRuntime>()
            .0
            .buildings
            .keys()
            .map(StableId::as_str)
            .collect::<Vec<_>>(),
        vec!["building:townhall"]
    );
    assert!(
        app.world()
            .resource::<WorldRuntime>()
            .generated
            .navigation
            .is_walkable(removed_position)
    );
    assert_eq!(
        app.world_mut()
            .query_filtered::<&RuntimeBuilding, Without<TownHall>>()
            .iter(app.world())
            .count(),
        0
    );
}

#[test]
fn world_grid_projection_round_trips() {
    let config = GameConfig::default();
    for cell in [
        GridPos { x: 0, z: 0 },
        GridPos { x: 31, z: 47 },
        GridPos { x: 63, z: 63 },
    ] {
        assert_eq!(
            world_to_grid(grid_to_world(cell, &config), &config),
            Some(cell)
        );
    }
}

#[test]
fn credits_fireworks_use_authored_activation_and_deterministic_emission() {
    let presentation = embedded_presentation();
    assert!(!credits_fireworks_active(
        &presentation,
        "VFX_FireWorks",
        55.99
    ));
    assert!(credits_fireworks_active(
        &presentation,
        "VFX_FireWorks",
        56.0
    ));
    assert!(
        (credits_fireworks_start(&presentation, "VFX_FireWorks (1)") - 56.0).abs() < f32::EPSILON
    );
    assert_eq!(expected_firework_launches(56.0, 56.0, 16.0), 1);
    assert_eq!(expected_firework_launches(56.5, 56.0, 16.0), 9);
    assert_eq!(expected_firework_launches(55.9, 56.0, 16.0), 0);

    let seed = firework_seed(1, 42, 31);
    assert_eq!(seed, firework_seed(1, 42, 31));
    assert_ne!(seed, firework_seed(0, 42, 31));
    assert!((0.0..=1.0).contains(&deterministic_unit(seed)));
    assert!((3..=7).contains(&deterministic_u16_range([3, 7], seed)));
}

#[test]
fn credits_fireworks_spawn_live_rockets_bursts_and_sparks() {
    let presentation = embedded_presentation();
    let effect_id = presentation
        .fireworks_effects
        .keys()
        .next()
        .unwrap()
        .clone();
    let rocket_lifetime = presentation.fireworks_effects[&effect_id].rocket_lifetime_seconds[1];
    let mut world = World::new();
    world.insert_resource(Time::<()>::default());
    world.insert_resource(RuntimePresentation(presentation));
    world.insert_resource(CreditsTimeline {
        elapsed_seconds: 56.0,
    });
    world.spawn(CreditsFireworksEmitter {
        target_path: "VFX_FireWorks".into(),
        effect: effect_id.clone(),
        emitter_index: 0,
        origin_percent: Vec2::new(25.0, 92.0),
        next_launch_index: 0,
        active_rockets: 0,
    });

    let mut schedule = Schedule::default();
    schedule.add_systems(update_credits_fireworks);
    schedule.run(&mut world);
    assert_eq!(
        world
            .query::<&CreditsFireworkParticle>()
            .iter(&world)
            .filter(|particle| particle.kind == CreditsFireworkParticleKind::Rocket)
            .count(),
        1
    );

    let particle_entity = {
        let mut query = world.query::<(Entity, &CreditsFireworkParticle)>();
        query
            .iter(&world)
            .find(|(_, particle)| particle.kind == CreditsFireworkParticleKind::Rocket)
            .map(|(entity, _)| entity)
            .unwrap()
    };
    world
        .entity_mut(particle_entity)
        .get_mut::<CreditsFireworkParticle>()
        .unwrap()
        .age_seconds = rocket_lifetime;
    schedule.run(&mut world);
    assert!(world.query::<&CreditsFireworkBurst>().iter(&world).count() >= 1);
    assert!(
        world
            .query::<&CreditsFireworkParticle>()
            .iter(&world)
            .any(|particle| particle.kind == CreditsFireworkParticleKind::BurstFlash)
    );

    for mut burst in world
        .query::<&mut CreditsFireworkBurst>()
        .iter_mut(&mut world)
    {
        burst.delay_seconds = 0.0;
    }
    schedule.run(&mut world);
    assert!(
        world
            .query::<&CreditsFireworkParticle>()
            .iter(&world)
            .any(|particle| particle.kind == CreditsFireworkParticleKind::Spark)
    );
}

#[test]
fn embedded_chimney_smoke_preserves_authored_emitters_and_parameters() {
    let presentation = embedded_presentation();
    assert_eq!(presentation.chimney_smoke_effects.len(), 1);
    assert_eq!(
        presentation
            .prefab_chimney_emitters
            .values()
            .map(Vec::len)
            .sum::<usize>(),
        7
    );
    let effect = presentation.chimney_smoke_effects.values().next().unwrap();
    assert!((effect.emission_rate_per_second - 5.0).abs() < f32::EPSILON);
    assert!((effect.lifetime_seconds - 5.0).abs() < f32::EPSILON);
    assert!((effect.start_speed - 1.0).abs() < f32::EPSILON);
    assert!((effect.start_size - 0.25).abs() < f32::EPSILON);
    assert!((effect.cone_radius - 0.11).abs() < f32::EPSILON);
    assert!((effect.cone_angle_degrees - 6.1).abs() < f32::EPSILON);
    assert!(
        effect
            .alpha_over_lifetime
            .into_iter()
            .zip([1.0, 0.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert!(!effect.world_space);

    let house = presentation
        .prefab_chimney_emitters
        .get("5939659079433a24db8e34e97f888f7d")
        .unwrap();
    assert_eq!(house.len(), 2);
    assert_eq!(house[0].age, 1);
    assert!(
        house[0]
            .local_position
            .into_iter()
            .zip([-0.373_000_14, 2.201, -0.25])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert_eq!(house[1].age, 2);
}

#[test]
fn chimney_emission_and_world_transform_are_deterministic() {
    let mut accumulator = 0.0;
    assert_eq!(chimney_emission_count(&mut accumulator, 5.0, 0.1, 1_000), 0);
    assert!((accumulator - 0.5).abs() < f32::EPSILON);
    assert_eq!(chimney_emission_count(&mut accumulator, 5.0, 0.1, 1_000), 1);
    assert!(accumulator.abs() < f32::EPSILON);
    assert_eq!(chimney_emission_count(&mut accumulator, 5.0, 1.0, 3), 3);
    assert!((accumulator - 2.0).abs() < f32::EPSILON);

    let transform = GlobalTransform::from(
        Transform::from_xyz(10.0, 20.0, 30.0)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::splat(2.0)),
    );
    let position = chimney_emitter_world_position(&transform, Vec3::new(1.0, 2.0, 3.0));
    assert!(position.abs_diff_eq(Vec3::new(16.0, 24.0, 28.0), 0.000_01));
    assert!((chimney_emitter_world_scale(&transform) - 2.0).abs() < f32::EPSILON);

    let building = StableId::new("building:runtime_00000001").unwrap();
    assert_eq!(
        chimney_particle_seed(&building, 2, 41, 11),
        chimney_particle_seed(&building, 2, 41, 11)
    );
    assert_ne!(
        chimney_particle_seed(&building, 2, 41, 11),
        chimney_particle_seed(&building, 2, 42, 11)
    );
    assert_eq!(chimney_alpha_step(0.0), 0);
    assert_eq!(chimney_alpha_step(0.5), 4);
    assert_eq!(chimney_alpha_step(1.0), 7);
}

#[test]
fn rotated_footprints_and_building_moves_are_deterministic() {
    assert_eq!(rotated_footprint([2, 5], 0), [2, 5]);
    assert_eq!(rotated_footprint([2, 5], 1), [5, 2]);
    assert_eq!(rotated_footprint([2, 5], -1), [5, 2]);
    let world = generate_world(&GameConfig::default().world);
    let (position, rotation) = shift_grid_position(
        GridPos { x: 1, z: 1 },
        &[
            BuildingAction {
                direction: BuildingDirection::Left,
                amount: 4,
            },
            BuildingAction {
                direction: BuildingDirection::Up,
                amount: 3,
            },
            BuildingAction {
                direction: BuildingDirection::Rotate,
                amount: -2,
            },
        ],
        &world,
    );
    assert_eq!(position, GridPos { x: 4, z: 0 });
    assert_eq!(rotation, -2);

    let content = embedded_content();
    let house = &content.buildings[&StableId::new("building:house").unwrap()];
    assert_eq!(house.footprint, [2, 3]);
    assert_eq!(rotated_footprint(house.footprint, 1), [3, 2]);
    let square = &content.buildings[&StableId::new("building:stonemason").unwrap()];
    assert_eq!(square.footprint, [2, 2]);
    let runtime_id = StableId::new("building:rotation_test").unwrap();
    let mut simulation = WorldSimulation::new(1);
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id.clone(),
            archetype: square.archetype.clone(),
            position: GridPos { x: 8, z: 8 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 100,
            complete: true,
        },
    );
    assert_eq!(
        rotate_building_instance(&content, &mut simulation, &runtime_id, -1),
        Ok((square.display_name.clone(), 270))
    );
    assert_eq!(simulation.buildings[&runtime_id].rotation_quarter_turns, -1);
}

#[test]
fn building_direction_commands_follow_the_visible_town_axes() {
    let world = generate_world(&GameConfig::default().world);
    let origin = GridPos { x: 10, z: 10 };
    for (direction, expected) in [
        (BuildingDirection::Up, GridPos { x: 11, z: 10 }),
        (BuildingDirection::Down, GridPos { x: 9, z: 10 }),
        (BuildingDirection::Left, GridPos { x: 10, z: 9 }),
        (BuildingDirection::Right, GridPos { x: 10, z: 11 }),
    ] {
        let (actual, rotation) = shift_grid_position(
            origin,
            &[BuildingAction {
                direction,
                amount: 1,
            }],
            &world,
        );
        assert_eq!(actual, expected, "{direction:?}");
        assert_eq!(rotation, 0);
    }
}

#[test]
fn wall_lines_require_orthogonal_endpoints_and_include_both_ends() {
    let horizontal =
        wall_line_cells(GridPos { x: 8, z: 5 }, GridPos { x: 4, z: 5 }).expect("horizontal line");
    assert_eq!(horizontal.len(), 5);
    assert_eq!(horizontal.first(), Some(&GridPos { x: 4, z: 5 }));
    assert_eq!(horizontal.last(), Some(&GridPos { x: 8, z: 5 }));
    let vertical =
        wall_line_cells(GridPos { x: 3, z: 2 }, GridPos { x: 3, z: 6 }).expect("vertical line");
    assert_eq!(vertical.len(), 5);
    assert!(wall_line_cells(GridPos { x: 2, z: 2 }, GridPos { x: 3, z: 3 }).is_err());

    let wood = StableId::new("resource:wood").unwrap();
    let cost = BTreeMap::from([(wood.clone(), 7)]);
    assert_eq!(scaled_building_cost(&cost, horizontal.len())[&wood], 35);

    let mut placement = BuildingPlacement {
        building: StableId::new("building:wall").unwrap(),
        thick_path: false,
        position: GridPos { x: 10, z: 10 },
        navigation_position: None,
        rotation_quarter_turns: 0,
        line_start: None,
        line_end: None,
        path_cells: Vec::new(),
        inactivity_seconds: 0.0,
    };
    assert_eq!(placement_visual_cells(&placement), vec![placement.position]);
    placement.line_start = Some(placement.position);
    placement.position = GridPos { x: 10, z: 14 };
    assert_eq!(placement_visual_cells(&placement).len(), 5);
}

#[test]
fn persistence_messages_bridge_runtime_intent_to_menu_io() {
    let mut app = App::new();
    app.add_message::<TownPersistenceRequest>()
        .init_resource::<MenuIoRequest>()
        .add_systems(Update, apply_town_persistence_requests);
    let source = PathBuf::from("operator-selected.stbevy");
    {
        let mut messages = app
            .world_mut()
            .resource_mut::<Messages<TownPersistenceRequest>>();
        messages.write(TownPersistenceRequest::Save);
        messages.write(TownPersistenceRequest::SaveJumpStart);
        messages.write(TownPersistenceRequest::Load(Some(source.clone())));
    }

    app.update();

    let io = app.world().resource::<MenuIoRequest>();
    assert!(io.save);
    assert!(io.save_jump_start);
    assert!(io.load);
    assert_eq!(io.load_source.as_ref(), Some(&source));
}
