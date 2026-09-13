#[test]
fn workers_choose_nearest_compatible_station_and_reassign() {
    let config = GameConfig::default();
    let content = embedded_content();
    let logger = StableId::new("role:logger").unwrap();
    let actor_id = StableId::new("npc:station_test").unwrap();
    let station_id = StableId::new("building:runtime_station").unwrap();
    let lumbermill = &content.buildings[&StableId::new("building:lumbermill").unwrap()];
    let position = GridPos { x: 10, z: 10 };
    let mut simulation = WorldSimulation::new(config.world.seed);
    assert!(simulation.join_player(actor_id.clone(), position));
    simulation.assign_role(&actor_id, logger).unwrap();
    simulation.actors.get_mut(&actor_id).unwrap().station =
        Some(StableId::new("building:townhall").unwrap());
    simulation.buildings.insert(
        station_id.clone(),
        BuildingState {
            id: station_id.clone(),
            archetype: lumbermill.archetype.clone(),
            position: GridPos { x: 16, z: 10 },
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );

    ensure_actor_station(&content, &mut simulation, &config, &actor_id);
    assert_eq!(
        simulation.actors[&actor_id].station,
        Some(station_id.clone())
    );

    let closer_station = StableId::new("building:closer_runtime_station").unwrap();
    simulation.buildings.insert(
        closer_station.clone(),
        BuildingState {
            id: closer_station.clone(),
            archetype: lumbermill.archetype.clone(),
            position: GridPos { x: 11, z: 10 },
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );
    ensure_actor_station(&content, &mut simulation, &config, &actor_id);
    assert_eq!(
        simulation.actors[&actor_id].station,
        Some(closer_station.clone())
    );

    simulation
        .buildings
        .get_mut(&closer_station)
        .unwrap()
        .complete = false;
    ensure_actor_station(&content, &mut simulation, &config, &actor_id);
    assert_eq!(
        simulation.actors[&actor_id].station,
        Some(station_id.clone())
    );

    simulation.buildings.get_mut(&station_id).unwrap().complete = false;
    ensure_actor_station(&content, &mut simulation, &config, &actor_id);
    assert_eq!(
        simulation.actors[&actor_id]
            .station
            .as_ref()
            .map(StableId::as_str),
        Some("building:townhall")
    );
}

#[test]
fn resource_workers_deposit_at_their_authored_processing_stations() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world_with_content(&config.world, &content);
    let centre = nearest_walkable(
        &world,
        GridPos {
            x: world.navigation.width() / 2,
            z: world.navigation.height() / 2,
        },
    )
    .unwrap();

    for (serial, building_id, role_id, resource_id) in [
        (0, "building:stonemason", "role:miner", "resource:ore"),
        (1, "building:lumbermill", "role:logger", "resource:wood"),
        (2, "building:windmill", "role:gatherer", "resource:food"),
    ] {
        let building_definition = &content.buildings[&StableId::new(building_id).unwrap()];
        let station_id = StableId::new(format!("building:deposit_station_{serial}")).unwrap();
        let actor_id = StableId::new(format!("npc:deposit_worker_{serial}")).unwrap();
        let resource = StableId::new(resource_id).unwrap();
        let mut simulation = WorldSimulation::new(world.seed);
        simulation.buildings.insert(
            station_id.clone(),
            BuildingState {
                id: station_id.clone(),
                archetype: building_definition.archetype.clone(),
                position: centre,
                rotation_quarter_turns: 0,
                level: 1,
                health: building_base_max_health(&content, building_definition)
                    .try_into()
                    .unwrap(),
                complete: true,
            },
        );
        assert!(simulation.join_player(actor_id.clone(), centre));
        simulation
            .assign_role(&actor_id, StableId::new(role_id).unwrap())
            .unwrap();
        simulation.actors.get_mut(&actor_id).unwrap().station = Some(station_id.clone());
        let carry_capacity =
            effective_role_stats(&content, &simulation, &simulation.actors[&actor_id])
                .unwrap()
                .carry_capacity;
        simulation
            .gather(&actor_id, resource.clone(), carry_capacity)
            .unwrap();

        let (goal, target) =
            next_agent_goal(&simulation, &world, &config, &content, &actor_id, centre);
        assert_eq!(
            goal,
            AgentGoal::Deposit,
            "{role_id} did not choose its station"
        );
        let station = assigned_station(
            &content,
            &simulation,
            &config,
            &simulation.actors[&actor_id],
        )
        .unwrap();
        assert_eq!(station.id, &station_id);
        assert_eq!(
            Some(target),
            station_deposit_approach(&content, &simulation, &world, station, centre)
        );
        assert!(world.navigation.find_path(centre, target).is_ok());

        assert!(
            complete_agent_goal(
                &mut simulation,
                &mut world,
                &config,
                &content,
                &actor_id,
                &goal,
                target,
            )
            .is_none(),
            "deposit has no visual presentation"
        );
        assert_eq!(simulation.town_resources[&resource], carry_capacity);
        assert!(
            !simulation.actors[&actor_id]
                .inventory
                .contains_key(&resource)
        );
    }
}

#[test]
fn partially_loaded_miner_promotes_from_town_hall_and_deposits_when_ore_is_exhausted() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world_with_content(&config.world, &content);
    let actor_id = StableId::new("npc:partial_miner_station_regression").unwrap();
    let station_id = StableId::new("building:partial_miner_stonemason").unwrap();
    let stonemason = &content.buildings[&StableId::new("building:stonemason").unwrap()];
    let position = nearest_walkable(
        &world,
        GridPos {
            x: world.navigation.width() / 2,
            z: world.navigation.height() / 2,
        },
    )
    .unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    ensure_town_hall_state(&content, &config, &mut simulation);
    simulation.buildings.insert(
        station_id.clone(),
        BuildingState {
            id: station_id.clone(),
            archetype: stonemason.archetype.clone(),
            position,
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(&content, stonemason)).unwrap(),
            complete: true,
        },
    );
    assert!(simulation.join_player(actor_id.clone(), position));
    simulation
        .assign_role(&actor_id, StableId::new("role:miner").unwrap())
        .unwrap();
    simulation.actors.get_mut(&actor_id).unwrap().station =
        Some(StableId::new("building:townhall").unwrap());
    simulation
        .gather(&actor_id, StableId::new("resource:ore").unwrap(), 6)
        .unwrap();
    for resource in &mut world.resources {
        if resource.kind.as_str() == "resource:ore" {
            resource.amount = 0;
        }
    }

    ensure_actor_station(&content, &mut simulation, &config, &actor_id);
    assert_eq!(
        simulation.actors[&actor_id].station,
        Some(station_id.clone())
    );
    let (goal, target) =
        next_agent_goal(&simulation, &world, &config, &content, &actor_id, position);
    assert_eq!(goal, AgentGoal::Deposit);
    let station = assigned_station(
        &content,
        &simulation,
        &config,
        &simulation.actors[&actor_id],
    )
    .unwrap();
    assert_eq!(station.id, &station_id);
    assert_eq!(
        Some(target),
        station_deposit_approach(&content, &simulation, &world, station, position)
    );
}

#[test]
fn builders_reserve_distinct_unoccupied_construction_approaches() {
    let world = generate_world(&GameConfig::default().world);
    let origin = GridPos { x: 95, z: 95 };
    let first_builder = StableId::new("npc:builder_approach_first").unwrap();
    let second_builder = StableId::new("npc:builder_approach_second").unwrap();
    let occupied = BTreeMap::new();
    let first = unoccupied_building_approach(
        &world,
        origin,
        [3, 3],
        GridPos { x: 90, z: 95 },
        &first_builder,
        &occupied,
        &BTreeMap::new(),
    )
    .unwrap();
    let reservations = BTreeMap::from([(first, first_builder)]);
    let second = unoccupied_building_approach(
        &world,
        origin,
        [3, 3],
        GridPos { x: 90, z: 95 },
        &second_builder,
        &occupied,
        &reservations,
    )
    .unwrap();
    assert_ne!(second, first);
}

#[test]
fn movement_goals_reserve_their_approach_cells() {
    assert!(goal_reserves_approach(&AgentGoal::Deposit));
    assert!(goal_reserves_approach(&AgentGoal::HarvestFarm(
        StableId::new("building:farm_reservation").unwrap()
    )));
    assert!(goal_reserves_approach(&AgentGoal::Construct(
        StableId::new("building:construction_reservation").unwrap()
    )));
    assert!(goal_reserves_approach(&AgentGoal::PlantTree(GridPos {
        x: 4,
        z: 5,
    })));
    assert!(goal_reserves_approach(&AgentGoal::Wander));
    assert!(goal_uses_idle_approach_reservation(&AgentGoal::Wander));
    assert!(!goal_uses_idle_approach_reservation(&AgentGoal::Construct(
        StableId::new("building:construction_work_reservation").unwrap()
    )));
}

#[test]
fn construction_can_try_fine_space_inside_a_blocked_coarse_approach() {
    let mut world = generate_world(&GameConfig::default().world);
    let position = (1..world.navigation.height().saturating_sub(1))
        .flat_map(|z| {
            (1..world.navigation.width().saturating_sub(1)).map(move |x| GridPos { x, z })
        })
        .find(|position| {
            (position.z - 1..=position.z + 1).all(|z| {
                (position.x - 1..=position.x + 1)
                    .all(|x| world.navigation.is_walkable(GridPos { x, z }))
            })
        })
        .expect("generated world should contain a walkable 3x3 area");
    let blocked_approach = GridPos {
        x: position.x - 1,
        z: position.z,
    };
    world
        .navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: blocked_approach,
                max: blocked_approach,
            },
            true,
        )
        .unwrap();
    let actor = StableId::new("npc:fine_construction_approach").unwrap();

    assert!(
        !building_approaches(&world, position, [1, 1], blocked_approach)
            .contains(&blocked_approach)
    );
    assert_eq!(
        unoccupied_construction_approach(
            &world,
            position,
            [1, 1],
            blocked_approach,
            &actor,
            &BTreeMap::new(),
            &BTreeMap::new(),
        ),
        Some(blocked_approach)
    );
}

#[test]
fn floorplan_owner_label_names_the_player_and_building() {
    let content = embedded_content();
    let owner = StableId::new("player:floorplan_owner").unwrap();
    let mut simulation = WorldSimulation::new(7);
    assert!(simulation.join_player(owner.clone(), GridPos { x: 4, z: 5 }));
    simulation.actors.get_mut(&owner).unwrap().display_name = Some("Aidan".to_owned());
    let definition = &content.buildings[&StableId::new("building:stonemason").unwrap()];

    assert_eq!(
        building_placement_overlay_text(&simulation, definition, &owner),
        "Aidan · Stonemason floorplan"
    );
}

#[test]
fn station_search_matches_unity_broad_phase_and_distance_ranking() {
    let content = embedded_content();
    let station_id = StableId::new("building:lumbermill").unwrap();
    let definition = content.buildings[&station_id].station.as_ref().unwrap();
    let station = StationCandidate {
        id: &station_id,
        position: GridPos { x: 100, z: 100 },
        definition,
    };
    let range = station_search_range_cells(station);

    // Unity's cell-space broad phase queries an axis-aligned rectangle,
    // so a diagonal target at both authored extents remains eligible.
    assert!(within_station_search_region(
        GridPos {
            x: station.position.x + range,
            z: station.position.z + range,
        },
        station,
    ));
    assert!(!within_station_search_region(
        GridPos {
            x: station.position.x + range + 1,
            z: station.position.z,
        },
        station,
    ));

    // Unity then ranks cached targets using Vector3.SqrMagnitude. The
    // diagonal target is closer even though its Manhattan sum is larger.
    let axis = GridPos { x: 100, z: 106 };
    let diagonal = GridPos { x: 104, z: 104 };
    assert!(
        grid_distance_squared(diagonal, station.position)
            < grid_distance_squared(axis, station.position)
    );
    assert!(
        diagonal.x.abs_diff(station.position.x) + diagonal.z.abs_diff(station.position.z)
            > axis.x.abs_diff(station.position.x) + axis.z.abs_diff(station.position.z)
    );
}

#[test]
fn shipping_station_target_policies_match_unity_target_settings() {
    let content = embedded_content();
    let update = ["target:tree", "target:ore", "target:bush", "target:farm"];
    let clear = [
        "target:player",
        "target:fish",
        "target:enemy",
        "target:boss",
        "target:building",
        "target:damaged_building",
        "target:construction",
        "target:injured_player",
        "target:dead_player",
    ];
    assert_eq!(content.station_target_update_modes.len(), 13);
    for kind in update {
        assert_eq!(
            content.station_target_update_modes[&StableId::new(kind).unwrap()],
            StationUpdateMode::Update
        );
    }
    for kind in clear {
        assert_eq!(
            content.station_target_update_modes[&StableId::new(kind).unwrap()],
            StationUpdateMode::Clear
        );
    }
}

#[test]
fn resource_visibility_signature_ignores_non_depleting_amount_updates() {
    let mut world = generate_world(&GameConfig::default().world);
    world.resources = vec![stream_town_domain::GeneratedResource {
        id: StableId::new("resource:visibility-a").unwrap(),
        kind: StableId::new("resource:ore").unwrap(),
        target_kind: StableId::new("target:ore").unwrap(),
        position: GridPos { x: 4, z: 7 },
        offset_milli_cells: [0, 0],
        generation_occupancy: [0, 0],
        amount: 100,
    }];
    let initial = resource_visibility_signature(&world);

    world.resources[0].amount = 99;
    assert_eq!(resource_visibility_signature(&world), initial);

    world.resources[0].amount = 0;
    let depleted = resource_visibility_signature(&world);
    assert_ne!(depleted, initial);

    world.resources.push(stream_town_domain::GeneratedResource {
        id: StableId::new("resource:visibility-b").unwrap(),
        position: GridPos { x: 8, z: 9 },
        amount: 100,
        ..world.resources[0].clone()
    });
    assert_ne!(resource_visibility_signature(&world), depleted);
}

#[test]
fn station_update_mode_retains_targets_while_clear_mode_rebuilds_nearest_first() {
    let config = GameConfig::default();
    let mut content = embedded_content();
    let station_id = StableId::new("building:townhall").unwrap();
    content
        .buildings
        .get_mut(&station_id)
        .unwrap()
        .station
        .as_mut()
        .unwrap()
        .max_targets = 1;
    let simulation = WorldSimulation::new(config.world.seed);
    let mut world = generate_world(&config.world);
    world.resources.clear();
    let station = station_candidate(&content, &simulation, &config, &station_id).unwrap();
    let retained = stream_town_domain::GeneratedResource {
        id: StableId::new("resource:cache_retained").unwrap(),
        kind: StableId::new("resource:wood").unwrap(),
        target_kind: StableId::new("target:tree").unwrap(),
        position: GridPos {
            x: station.position.x.saturating_add(2),
            z: station.position.z,
        },
        offset_milli_cells: [0, 0],
        generation_occupancy: [0, 0],
        amount: 100,
    };
    world.resources.push(retained.clone());
    let mut runtime = StationTargetRuntime::default();
    let mut resource_index = StationResourceTargetIndex::default();
    populate_station_target_cache(
        &mut runtime,
        &mut resource_index,
        &content,
        &simulation,
        &world,
        &config,
        &station_id,
    );
    let tree = StableId::new("target:tree").unwrap();
    assert_eq!(
        runtime.stations[&station_id].targets[&tree],
        std::slice::from_ref(&retained.id)
    );

    let nearer = stream_town_domain::GeneratedResource {
        id: StableId::new("resource:cache_nearer").unwrap(),
        position: GridPos {
            x: station.position.x.saturating_add(1),
            z: station.position.z,
        },
        ..retained.clone()
    };
    world.resources.push(nearer.clone());
    populate_station_target_cache(
        &mut runtime,
        &mut resource_index,
        &content,
        &simulation,
        &world,
        &config,
        &station_id,
    );
    assert_eq!(
        runtime.stations[&station_id].targets[&tree],
        std::slice::from_ref(&retained.id),
        "Unity Update mode keeps valid cached targets"
    );

    content
        .station_target_update_modes
        .insert(tree.clone(), StationUpdateMode::Clear);
    populate_station_target_cache(
        &mut runtime,
        &mut resource_index,
        &content,
        &simulation,
        &world,
        &config,
        &station_id,
    );
    assert_eq!(
        runtime.stations[&station_id].targets[&tree],
        [nearer.id],
        "Unity Clear mode replaces the bounded cache from nearest to farthest"
    );
}

#[test]
fn deposit_cooldown_matches_unity_inventory_transfer_delay() {
    let config = GameConfig::default();
    let content = embedded_content();
    let actor_id = StableId::new("npc:deposit_timing").unwrap();
    let mut simulation = WorldSimulation::new(config.world.seed);
    assert!(simulation.join_player(actor_id.clone(), GridPos { x: 1, z: 1 }));
    assert!(
        (action_cooldown(&content, &simulation, &actor_id, &AgentGoal::Deposit) - 2.5).abs()
            <= f32::EPSILON
    );

    let position = GridPos { x: 1, z: 1 };
    let mut agent = Agent {
        id: actor_id,
        kind: ActorKind::Player,
        archetype: StableId::new("archetype:deposit_timing").unwrap(),
        goal: AgentGoal::Deposit,
        spawn: position,
        origin: position,
        navigation_position: placement_to_navigation_centre(position),
        path: vec![position],
        path_index: 1,
        target: position,
        action_cooldown_seconds: 0.0,
        action_started: false,
        repath_remaining_seconds: 0.0,
        health_regen_accumulator: 0.0,
        wander_sequence: 0,
        previous_wander_origin: None,
    };
    assert_eq!(
        arrived_action_phase(&mut agent, 2.5),
        ArrivedActionPhase::Started
    );
    assert!(agent.action_started);
    assert!((agent.action_cooldown_seconds - 2.5).abs() <= f32::EPSILON);
    agent.action_cooldown_seconds = 0.1;
    assert_eq!(
        arrived_action_phase(&mut agent, 0.0),
        ArrivedActionPhase::Waiting
    );
    agent.action_cooldown_seconds = 0.0;
    assert_eq!(
        arrived_action_phase(&mut agent, 0.0),
        ArrivedActionPhase::Complete
    );
}

#[test]
fn resource_workers_only_receive_targets_from_their_assigned_station_catalog() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    world.resources.clear();
    let close = stream_town_domain::GeneratedResource {
        id: StableId::new("resource:station_close").unwrap(),
        kind: StableId::new("resource:wood").unwrap(),
        target_kind: StableId::new("target:tree").unwrap(),
        position: GridPos { x: 5, z: 5 },
        offset_milli_cells: [0, 0],
        generation_occupancy: [5, 5],
        amount: 100,
    };
    let far = stream_town_domain::GeneratedResource {
        id: StableId::new("resource:station_far").unwrap(),
        kind: StableId::new("resource:wood").unwrap(),
        target_kind: StableId::new("target:tree").unwrap(),
        position: GridPos { x: 63, z: 63 },
        offset_milli_cells: [0, 0],
        generation_occupancy: [63, 63],
        amount: 100,
    };
    for resource in [&close, &far] {
        world
            .navigation
            .set_blocked(
                stream_town_domain::DirtyRegion {
                    min: resource.position,
                    max: resource.position,
                },
                false,
            )
            .unwrap();
    }
    world.resources.extend([close.clone(), far.clone()]);

    let lumbermill = &content.buildings[&StableId::new("building:lumbermill").unwrap()];
    let station_id = StableId::new("building:station_catalog_lumbermill").unwrap();
    let actor_id = StableId::new("npc:station_catalog_logger").unwrap();
    let current = GridPos { x: 4, z: 5 };
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.buildings.insert(
        station_id.clone(),
        BuildingState {
            id: station_id.clone(),
            archetype: lumbermill.archetype.clone(),
            position: GridPos { x: 4, z: 4 },
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(&content, lumbermill)).unwrap(),
            complete: true,
        },
    );
    assert!(simulation.join_player(actor_id.clone(), current));
    simulation
        .assign_role(&actor_id, StableId::new("role:logger").unwrap())
        .unwrap();
    simulation.actors.get_mut(&actor_id).unwrap().station = Some(station_id);

    assert_eq!(
        compatible_target_ids(
            &content,
            &simulation,
            &world,
            &config,
            &simulation.actors[&actor_id],
        ),
        vec![close.id.clone()],
    );
    assert_eq!(
        next_agent_goal(&simulation, &world, &config, &content, &actor_id, current).0,
        AgentGoal::Gather(close.id),
    );
}

#[test]
fn target_score_uses_authored_euclidean_distance_and_assignment_penalty() {
    let scoring = TargetingScoreDef {
        assignment_penalty_milli: 10_000_000,
        distance_penalty_milli_per_cell: 100,
    };
    assert_eq!(
        target_score_milli(GridPos { x: 3, z: 4 }, GridPos { x: 0, z: 0 }, 2, &scoring,),
        20_000_500,
    );
}

#[test]
fn action_ranges_and_tower_acquisition_are_euclidean() {
    let centre = GridPos { x: 20, z: 20 };
    let diagonal = GridPos { x: 23, z: 24 };
    assert!(within_milli_cell_range(diagonal, centre, 5_000));
    assert!(!within_milli_cell_range(diagonal, centre, 4_999));

    let axis_id = StableId::new("actor:tower_axis_enemy").unwrap();
    let diagonal_id = StableId::new("actor:tower_diagonal_enemy").unwrap();
    let mut simulation = WorldSimulation::new(1);
    assert!(simulation.join_player(axis_id.clone(), GridPos { x: 26, z: 20 }));
    assert!(simulation.join_player(diagonal_id.clone(), GridPos { x: 24, z: 24 }));
    for actor in [&axis_id, &diagonal_id] {
        simulation
            .assign_role(actor, StableId::new("role:enemy").unwrap())
            .unwrap();
    }
    assert_eq!(
        best_tower_target(&simulation, centre, 10_000).map(|actor| &actor.id),
        Some(&diagonal_id),
    );
    assert!(best_tower_target(&simulation, centre, 5_000).is_none());
}

#[test]
fn authored_target_sizes_drive_unity_action_reach_formulas() {
    let content = embedded_content();
    let player_archetype =
        archetype_id_by_source(&content, ArchetypeKind::Player, "Player_Character.prefab").unwrap();
    let blargul_archetype =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Blargul.prefab").unwrap();
    assert_eq!(
        content.archetypes[&player_archetype].target_size_milli_cells,
        750
    );
    assert_eq!(
        content.archetypes[&blargul_archetype].target_size_milli_cells,
        500
    );

    let defender_id = StableId::new("npc:target_size_defender").unwrap();
    let enemy_id = StableId::new("actor:target_size_blargul").unwrap();
    let mut combat = WorldSimulation::new(1);
    assert!(combat.join_player(defender_id.clone(), GridPos { x: 20, z: 20 }));
    combat
        .assign_role(&defender_id, StableId::new("role:defender").unwrap())
        .unwrap();
    combat.actors.get_mut(&defender_id).unwrap().archetype = Some(player_archetype.clone());
    assert!(combat.spawn_enemy(
        enemy_id.clone(),
        blargul_archetype,
        GridPos { x: 21, z: 20 },
        100,
    ));
    let defender = &combat.actors[&defender_id];
    let enemy = &combat.actors[&enemy_id];
    assert!(within_actor_attack_range(
        &content,
        &combat,
        defender,
        enemy,
        GridPos { x: 20, z: 20 },
    ));
    assert!(!within_actor_attack_range(
        &content,
        &combat,
        defender,
        enemy,
        GridPos { x: 20, z: 19 },
    ));

    combat.actors.get_mut(&defender_id).unwrap().position = GridPos { x: 22, z: 20 };
    let defender = &combat.actors[&defender_id];
    let enemy = &combat.actors[&enemy_id];
    assert!(within_actor_attack_range(
        &content,
        &combat,
        enemy,
        defender,
        GridPos { x: 20, z: 20 },
    ));
    assert!(!within_actor_attack_range(
        &content,
        &combat,
        enemy,
        defender,
        GridPos { x: 19, z: 20 },
    ));

    let priest_id = StableId::new("npc:target_size_priest").unwrap();
    assert!(combat.join_player(priest_id.clone(), GridPos { x: 20, z: 20 }));
    combat
        .assign_role(&priest_id, StableId::new("role:priest").unwrap())
        .unwrap();
    let priest = &combat.actors[&priest_id];
    assert!(within_actor_heal_range(
        &content,
        &combat,
        priest,
        &combat.actors[&defender_id],
        GridPos { x: 20, z: 20 },
    ));
    assert!(!within_actor_heal_range(
        &content,
        &combat,
        priest,
        &combat.actors[&defender_id],
        GridPos { x: 19, z: 20 },
    ));

    let builder_id = StableId::new("npc:target_size_builder").unwrap();
    assert!(combat.join_player(builder_id.clone(), GridPos { x: 20, z: 20 }));
    combat
        .assign_role(&builder_id, StableId::new("role:builder").unwrap())
        .unwrap();
    let house = &content.buildings[&StableId::new("building:house").unwrap()];
    assert_eq!(
        content.archetypes[&house.archetype].target_size_milli_cells,
        2_000
    );
    let mut building = BuildingState {
        id: StableId::new("building:target_size_house").unwrap(),
        archetype: house.archetype.clone(),
        position: GridPos { x: 22, z: 19 },
        rotation_quarter_turns: 0,
        level: 1,
        health: BUILDING_MAX_HEALTH,
        complete: false,
    };
    assert!(within_building_work_range(
        &content,
        &combat,
        &combat.actors[&builder_id],
        &building,
        GridPos { x: 20, z: 20 },
    ));
    building.position.x += 1;
    assert!(!within_building_work_range(
        &content,
        &combat,
        &combat.actors[&builder_id],
        &building,
        GridPos { x: 20, z: 20 },
    ));
}

#[test]
fn combat_and_healing_bypass_station_target_caches() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let tower_id = StableId::new("building:target_sensor_tower").unwrap();
    let tower = &content.buildings[&StableId::new("building:tower").unwrap()];
    let tower_state = BuildingState {
        id: tower_id.clone(),
        archetype: tower.archetype.clone(),
        position: GridPos { x: 1, z: 1 },
        rotation_quarter_turns: 0,
        level: 1,
        health: BUILDING_MAX_HEALTH,
        complete: true,
    };
    let current = GridPos { x: 30, z: 30 };

    let defender = StableId::new("npc:target_sensor_defender").unwrap();
    let axis_enemy = StableId::new("actor:target_sensor_axis_enemy").unwrap();
    let diagonal_enemy = StableId::new("actor:target_sensor_diagonal_enemy").unwrap();
    let mut combat = WorldSimulation::new(world.seed);
    assert!(combat.join_player(defender.clone(), current));
    assert!(combat.join_player(axis_enemy.clone(), GridPos { x: 30, z: 36 }));
    assert!(combat.join_player(diagonal_enemy.clone(), GridPos { x: 34, z: 34 }));
    combat
        .assign_role(&defender, StableId::new("role:defender").unwrap())
        .unwrap();
    for enemy in [&axis_enemy, &diagonal_enemy] {
        combat
            .assign_role(enemy, StableId::new("role:enemy").unwrap())
            .unwrap();
    }
    combat
        .buildings
        .insert(tower_id.clone(), tower_state.clone());
    combat.actors.get_mut(&defender).unwrap().station = Some(tower_id.clone());
    assert!(assigned_station(&content, &combat, &config, &combat.actors[&defender]).is_some());
    assert_eq!(
        next_agent_goal(&combat, &world, &config, &content, &defender, current).0,
        AgentGoal::Attack(diagonal_enemy)
    );

    let priest = StableId::new("npc:target_sensor_priest").unwrap();
    let patient = StableId::new("npc:target_sensor_patient").unwrap();
    let patient_position = GridPos { x: 35, z: 35 };
    let mut healing = WorldSimulation::new(world.seed);
    assert!(healing.join_player(priest.clone(), current));
    assert!(healing.join_player(patient.clone(), patient_position));
    healing
        .assign_role(&priest, StableId::new("role:priest").unwrap())
        .unwrap();
    healing.damage_actor(&patient, 20).unwrap();
    healing.buildings.insert(tower_id.clone(), tower_state);
    healing.actors.get_mut(&priest).unwrap().station = Some(tower_id);
    assert!(assigned_station(&content, &healing, &config, &healing.actors[&priest]).is_some());
    assert_eq!(
        next_agent_goal(&healing, &world, &config, &content, &priest, current).0,
        AgentGoal::Heal(patient)
    );
}

#[test]
fn defenders_prioritize_nearby_threats_then_their_defence_anchor() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let town_hall = town_hall_grid_position(&config);
    let defender_position = GridPos {
        x: town_hall.x.saturating_add(20),
        z: town_hall.z,
    };
    let close_to_defender = GridPos {
        x: defender_position.x.saturating_sub(1),
        z: defender_position.z,
    };
    let close_to_town = GridPos {
        x: town_hall.x.saturating_add(5),
        z: town_hall.z,
    };
    let fighter = StableId::new("npc:defender_priority").unwrap();
    let near_actor = StableId::new("enemy:near_actor").unwrap();
    let near_town = StableId::new("enemy:near_town").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(fighter.clone(), defender_position));
    assert!(simulation.join_player(near_actor.clone(), close_to_defender));
    assert!(simulation.join_player(near_town.clone(), close_to_town));
    for enemy in [&near_actor, &near_town] {
        simulation
            .assign_role(enemy, StableId::new("role:enemy").unwrap())
            .unwrap();
    }
    simulation
        .assign_role(&fighter, StableId::new("role:defender").unwrap())
        .unwrap();
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &fighter,
            defender_position,
        )
        .0,
        AgentGoal::Attack(near_actor.clone())
    );

    simulation.actors.get_mut(&near_actor).unwrap().alive = false;
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &fighter,
            defender_position,
        )
        .0,
        AgentGoal::Attack(near_town.clone())
    );
    simulation.actors.get_mut(&near_actor).unwrap().alive = true;

    simulation
        .assign_role(&fighter, StableId::new("role:ranger").unwrap())
        .unwrap();
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &fighter,
            defender_position,
        )
        .0,
        AgentGoal::Attack(near_actor)
    );
}

#[test]
fn equipment_visibility_matches_role_and_carry_state() {
    let content = embedded_content();
    let logger = content.roles[&StableId::new("role:logger").unwrap()]
        .equipment
        .as_ref()
        .unwrap();
    assert!(equipment_node_visible(logger, 0, "Body_Logger_Slim", false));
    assert!(equipment_node_visible(
        logger,
        2,
        "Body_Logger_Feminine",
        false
    ));
    assert!(equipment_node_visible(
        logger,
        0,
        "RHand_LoggerToolAxe_Starter",
        false
    ));
    assert!(!equipment_node_visible(
        logger,
        0,
        "LHand_LoggerCarryWood",
        false
    ));
    assert!(equipment_node_visible(
        logger,
        0,
        "LHand_LoggerCarryWood",
        true
    ));
    assert!(!carried_resource_visible(
        MovementAnimationState::Idle,
        true
    ));
    assert!(carried_resource_visible(
        MovementAnimationState::Moving,
        true
    ));
    assert!(!carried_resource_visible(
        MovementAnimationState::Moving,
        false
    ));

    let defender = content.roles[&StableId::new("role:defender").unwrap()]
        .equipment
        .as_ref()
        .unwrap();
    assert!(equipment_node_visible(
        defender,
        0,
        "LHand_DefenderToolShield_Starter",
        false
    ));
    assert!(equipment_node_visible(
        defender,
        0,
        "Helmet_Defender",
        false
    ));
    assert!(!equipment_node_visible(
        defender,
        0,
        "Body_Logger_Slim",
        false
    ));
    let equipment_names = equipment_node_names(&content);
    assert!(equipment_names.contains(canonical_equipment_node_name(
        "RHand_NecromancerToolScepter_Starter"
    )));
    assert_eq!(
        canonical_equipment_node_name("Helmet_Ruler"),
        "Helmet_Ruler"
    );
    assert!(player_equipment_slot_node("Body_Default_Slim"));
    assert!(player_equipment_slot_node("Back_CommanderBanner"));
    assert!(player_equipment_slot_node("RHand_LoggerToolAxe_Starter"));
    assert!(!player_equipment_slot_node("Head_Box"));
    assert!(!equipment_node_visible(
        defender,
        0,
        "Body_Default_Slim",
        false
    ));
}

#[test]
fn enemy_model_selection_is_stable_and_preserves_weapon_contracts() {
    let content = embedded_content();
    let skeleton =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Skeleton.prefab").unwrap();
    let models = content.archetypes[&skeleton].enemy_models.as_ref().unwrap();
    assert!(models.base_models.is_empty());
    assert_eq!(models.permanent_models.len(), 2);
    assert_eq!(models.optional_models.len(), 12);
    assert_eq!(models.weapons.len(), 3);

    let actor_id = StableId::new("actor:model_selection_skeleton").unwrap();
    let selection = enemy_model_selection(&actor_id, models);
    assert_eq!(selection, enemy_model_selection(&actor_id, models));
    assert!(selection.weapon.is_some_and(|index| index < 3));
    assert!(
        models
            .permanent_models
            .iter()
            .all(|name| { enemy_model_node_visible(models, &selection, name) })
    );
    for (index, weapon) in models.weapons.iter().enumerate() {
        assert_eq!(
            enemy_model_node_visible(models, &selection, &weapon.main_model),
            selection.weapon == Some(index)
        );
    }

    let mut simulation = WorldSimulation::new(42);
    assert!(simulation.spawn_enemy(actor_id.clone(), skeleton, GridPos { x: 10, z: 10 }, 100,));
    let contract =
        enemy_animation_contract(&content, &simulation.actors[&actor_id], &actor_id).unwrap();
    let weapon = &models.weapons[selection.weapon.unwrap()];
    assert_eq!(contract.action_animation, weapon.action_animation);
    assert_eq!(
        contract.action_animation_variants,
        weapon.action_animation_variants
    );
    assert_eq!(contract.run_animation, weapon.run_animation);
}

#[test]
fn shipping_enemies_use_authored_rigs_and_translated_controllers() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let expected = [
        ("Enemy_Blargul.prefab", "Blargul", "Blargul.glb"),
        ("Enemy_Goblin.prefab", "Goblin", "Goblin.glb"),
        (
            "Enemy_Goblin_BatteringRam.prefab",
            "BatteringRam",
            "Goblin_BatteringRam.glb",
        ),
        ("Enemy_Goblin_Boss.prefab", "GoblinBoss", "GoblinBoss.glb"),
        ("Enemy_Minotaur.prefab", "Minotaur", "Minotaur.glb"),
        ("Enemy_MinotaurBoss.prefab", "Minotaur", "Minotaur.glb"),
        (
            "Enemy_NecroSlasher.prefab",
            "NecroSlasher",
            "NecroSlasher.glb",
        ),
        (
            "Enemy_NecroStalker.prefab",
            "NecroStalker",
            "NecroStalker.glb",
        ),
        ("Enemy_Skeleton.prefab", "Skeleton", "Skeleton.glb"),
    ];
    for (source, controller_name, rig_suffix) in expected {
        let archetype = archetype_by_source(&content, ArchetypeKind::Enemy, source).unwrap();
        let scene = default_archetype_scene(archetype).unwrap();
        assert!(scene.asset_path.ends_with(rig_suffix), "{source}");
        let binding = &presentation.prefab_bindings[&archetype.source_guid];
        assert_eq!(
            presentation.controllers[&binding.controller].display_name, controller_name,
            "{source}"
        );
        assert_eq!(
            binding.rig_scene.as_deref(),
            Some(scene.asset_path.as_str()),
            "{source}"
        );
        assert!(
            converted_animation_spec(archetype, &presentation).is_some(),
            "{source} does not have a complete translated locomotion contract: {:?}",
            presentation.controllers[&binding.controller]
                .states
                .values()
                .filter(|state| state.display_name.eq_ignore_ascii_case("locomotion"))
                .flat_map(|state| &state.motions)
                .map(|motion| (
                    presentation.clips[&motion.clip].display_name.clone(),
                    presentation.clips[&motion.clip].transform_tracks.len(),
                    presentation.clips[&motion.clip].rig_asset_path.clone(),
                ))
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn building_level_research_is_contiguous_and_distributed_across_ages() {
    let content = embedded_content();
    let age_count = content
        .technology
        .nodes
        .values()
        .map(|node| authored_technology_age(&node.age))
        .max()
        .unwrap();

    for building in content.buildings.keys() {
        let researched_levels = content
            .technology
            .nodes
            .values()
            .filter_map(|node| {
                node.building_level_caps
                    .get(building)
                    .copied()
                    .filter(|level| *level >= 2)
            })
            .collect::<Vec<_>>();
        let Some(maximum) = researched_levels.iter().max().copied() else {
            continue;
        };
        let minimum_age = content
            .technology
            .nodes
            .values()
            .filter(|node| node.unlocked_buildings.contains(building))
            .map(|node| authored_technology_age(&node.age))
            .max()
            .unwrap_or(1);
        for level in 2..=maximum {
            let matches = content
                .technology
                .nodes
                .values()
                .filter(|node| node.building_level_caps.get(building) == Some(&level))
                .collect::<Vec<_>>();
            assert_eq!(
                matches.len(),
                1,
                "{building} must have exactly one level-{level} research"
            );
            let distributed = u32::from(level.saturating_sub(1))
                .saturating_mul(u32::from(age_count))
                / u32::from(maximum)
                + 1;
            let expected_age = u8::try_from(distributed)
                .unwrap_or(age_count)
                .clamp(minimum_age, age_count);
            assert_eq!(
                authored_technology_age(&matches[0].age),
                expected_age,
                "{building} level {level} is assigned to the wrong age"
            );
        }
    }
}

#[test]
fn age_two_research_requires_level_two_town_hall_and_has_no_tech_count_gate() {
    let config = GameConfig::default();
    let content = embedded_content();
    let town_hall = StableId::new("building:townhall").unwrap();
    let (age_up_id, age_up) = content
        .technology
        .nodes
        .iter()
        .find(|(_, node)| node.aged_buildings.contains(&town_hall))
        .expect("dedicated town age-up research exists");
    assert_eq!(age_up.display_name, "Advance to Age 2");
    assert!(age_up.building_level_caps.is_empty());
    assert_eq!(age_up.prerequisites.len(), 1);
    let level_two = &age_up.prerequisites[0];
    assert_eq!(
        content.technology.nodes[level_two]
            .building_level_caps
            .get(&town_hall),
        Some(&2)
    );

    let mut simulation = WorldSimulation::new(config.world.seed);
    ensure_town_hall_state(&content, &config, &mut simulation);
    simulation.unlocked_technology.insert(level_two.clone());
    assert!(
        !eligible_technology_ids(&content, &simulation).contains(age_up_id),
        "research alone must not replace the physical Town Hall upgrade"
    );

    simulation.buildings.get_mut(&town_hall).unwrap().level = 2;
    assert!(
        eligible_technology_ids(&content, &simulation).contains(age_up_id),
        "Age 2 must become eligible immediately without a 50-tech requirement"
    );

    let (age_two_id, age_two) = content
        .technology
        .nodes
        .iter()
        .find(|(_, node)| {
            authored_technology_age(&node.age) == 2
                && !node.aged_buildings.contains(&town_hall)
                && !node.prerequisites.is_empty()
        })
        .expect("ordinary Age 2 research exists");
    simulation
        .unlocked_technology
        .extend(age_two.prerequisites.iter().cloned());
    assert!(
        !eligible_technology_ids(&content, &simulation).contains(age_two_id),
        "ordinary Age 2 research must remain hidden while the town is Age 1"
    );
    simulation.unlocked_technology.insert(age_up_id.clone());
    assert!(eligible_technology_ids(&content, &simulation).contains(age_two_id));
}

#[test]
fn town_hall_is_a_stable_authoritative_building_and_ages_with_technology() {
    let config = GameConfig::default();
    let content = embedded_content();
    let town_hall_id = StableId::new("building:townhall").unwrap();
    let definition = &content.buildings[&town_hall_id];
    let mut simulation = WorldSimulation::new(config.world.seed);

    ensure_town_hall_state(&content, &config, &mut simulation);
    let town_hall = &simulation.buildings[&town_hall_id];
    assert_eq!(town_hall.id, town_hall_id);
    assert_eq!(town_hall.archetype, definition.archetype);
    assert_eq!(
        town_hall.position,
        town_hall_placement_position(&config, definition.footprint)
    );
    assert_eq!(town_hall.level, 1);
    assert_eq!(town_hall.health, building_max_health(&content, town_hall));
    assert_eq!(town_hall.health, 250);
    assert!(town_hall.complete);
    assert_eq!(constructed_building_count(&simulation), 0);
    assert_eq!(building_age(&content, &simulation, &town_hall_id, 1), 1);

    let age_technology = content
        .technology
        .nodes
        .iter()
        .find(|(_, technology)| technology.aged_buildings.contains(&town_hall_id))
        .map(|(id, _)| id.clone())
        .expect("converted technology contains the Town Hall age unlock");
    simulation.unlocked_technology.insert(age_technology);
    assert_eq!(building_age(&content, &simulation, &town_hall_id, 1), 2);
    assert_eq!(
        archetype_scene_for_age(&content.archetypes[&definition.archetype], 2)
            .and_then(|scene| scene.age),
        Some(2)
    );

    let saved = simulation.buildings[&town_hall_id].clone();
    ensure_town_hall_state(&content, &config, &mut simulation);
    assert_eq!(simulation.buildings.len(), 1);
    assert_eq!(simulation.buildings[&town_hall_id], saved);
}

#[test]
fn live_agent_goals_select_authored_action_animation_contracts() {
    let content = embedded_content();
    let actor_id = StableId::new("npc:animation_test").unwrap();
    let mut simulation = WorldSimulation::new(42);
    assert!(simulation.join_player(actor_id.clone(), GridPos { x: 10, z: 10 }));
    let agent_for = |goal| Agent {
        id: actor_id.clone(),
        kind: ActorKind::Player,
        archetype: StableId::new("archetype:viewer").unwrap(),
        goal,
        spawn: GridPos { x: 10, z: 10 },
        origin: GridPos { x: 10, z: 10 },
        navigation_position: placement_to_navigation_centre(GridPos { x: 10, z: 10 }),
        path: vec![GridPos { x: 10, z: 10 }],
        path_index: 1,
        target: GridPos { x: 10, z: 10 },
        action_cooldown_seconds: 0.75,
        action_started: true,
        repath_remaining_seconds: 0.0,
        health_regen_accumulator: 0.0,
        wander_sequence: 0,
        previous_wander_origin: None,
    };

    for (role, goal, expected) in [
        (
            "role:logger",
            AgentGoal::Gather(StableId::new("resource:test").unwrap()),
            "WoodCutting",
        ),
        (
            "role:miner",
            AgentGoal::Gather(StableId::new("resource:test").unwrap()),
            "Mining",
        ),
        (
            "role:builder",
            AgentGoal::Construct(StableId::new("building:test").unwrap()),
            "Build",
        ),
        (
            "role:ranger",
            AgentGoal::Attack(StableId::new("actor:enemy").unwrap()),
            "BowShoot",
        ),
        (
            "role:priest",
            AgentGoal::Heal(StableId::new("actor:injured").unwrap()),
            "Heal",
        ),
    ] {
        simulation
            .assign_role(&actor_id, StableId::new(role).unwrap())
            .unwrap();
        assert_eq!(
            agent_action_animation(&content, &agent_for(goal), &simulation.actors[&actor_id])
                .as_deref(),
            Some(expected)
        );
    }

    let mut moving = agent_for(AgentGoal::Gather(StableId::new("resource:test").unwrap()));
    moving.path_index = 0;
    moving.path[0] = GridPos { x: 11, z: 10 };
    assert_eq!(
        agent_action_animation(&content, &moving, &simulation.actors[&actor_id]),
        None
    );
    simulation
        .assign_role(&actor_id, StableId::new("role:miner").unwrap())
        .unwrap();
    let mut between_mining_ticks =
        agent_for(AgentGoal::Gather(StableId::new("resource:test").unwrap()));
    between_mining_ticks.action_cooldown_seconds = 0.0;
    between_mining_ticks.action_started = false;
    assert_eq!(
        agent_action_animation(
            &content,
            &between_mining_ticks,
            &simulation.actors[&actor_id]
        )
        .as_deref(),
        Some("Mining"),
        "the Unity action state must survive Bevy's per-tick replanning gap"
    );
    assert_eq!(deterministic_animation_variant(&actor_id, "BowShoot", 1), 0);
    assert_eq!(
        deterministic_animation_variant(&actor_id, "BowShoot", 4),
        deterministic_animation_variant(&actor_id, "BowShoot", 4)
    );
}
