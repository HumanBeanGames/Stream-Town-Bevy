#[test]
fn building_placement_over_paths_removes_every_covered_path_section() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let house = &content.buildings[&StableId::new("building:house").unwrap()];
    let footprint = rotated_footprint(house.footprint, 0);
    let position = find_building_site(&world, town_hall_grid_position(&config), footprint)
        .expect("generated world has a house site");
    let region = building_region(position, footprint, &world).expect("house region");
    let outside = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .find(|cell| {
            !(region.min.x..=region.max.x).contains(&cell.x)
                || !(region.min.z..=region.max.z).contains(&cell.z)
        })
        .expect("world has a cell outside the house region");
    let path_archetype = content.buildings
        [&StableId::new("building:path").expect("static path ID")]
        .archetype
        .clone();
    let mut simulation = WorldSimulation::new(world.seed);
    let covered = [
        ("building:covered_path_a", region.min),
        ("building:covered_path_b", region.max),
    ];
    let outside_id = StableId::new("building:outside_path").unwrap();
    for (id, cell) in covered
        .iter()
        .copied()
        .chain([(outside_id.as_str(), outside)])
    {
        let id = StableId::new(id).unwrap();
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id: id.clone(),
                archetype: path_archetype.clone(),
                position: cell,
                rotation_quarter_turns: 0,
                level: 1,
                health: 100,
                complete: true,
            },
        );
        simulation
            .path_navigation_positions
            .insert(id, placement_to_navigation_centre(cell));
    }

    assert!(building_site_is_available_for_simulation(
        &content,
        &simulation,
        &world,
        position,
        footprint,
    ));
    let removed =
        remove_paths_covered_by_regions(&content, &mut world, &mut simulation, &[region]).unwrap();
    assert_eq!(
        removed.into_iter().collect::<BTreeSet<_>>(),
        covered
            .into_iter()
            .map(|(id, _)| StableId::new(id).unwrap())
            .collect()
    );
    for (id, _) in covered {
        let id = StableId::new(id).unwrap();
        assert!(!simulation.buildings.contains_key(&id));
        assert!(!simulation.path_navigation_positions.contains_key(&id));
    }
    assert!(simulation.buildings.contains_key(&outside_id));
    assert!(
        simulation
            .path_navigation_positions
            .contains_key(&outside_id)
    );
}

#[test]
fn paths_route_diagonally_on_single_fine_navigation_cells() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let simulation = WorldSimulation::new(world.seed);
    let origin = placement_to_navigation_centre(town_hall_grid_position(&config));
    let mut placement = BuildingPlacement {
        building: StableId::new("building:path").unwrap(),
        thick_path: false,
        position: navigation_to_placement(origin),
        navigation_position: Some(origin),
        rotation_quarter_turns: 0,
        line_start: Some(origin),
        line_end: None,
        path_cells: vec![origin],
        inactivity_seconds: 0.0,
    };
    let navigation = build_fine_navigation(&config, &content, &simulation, &world).unwrap();
    move_path_placement(
        &mut placement,
        &[
            BuildingAction {
                direction: BuildingDirection::Right,
                amount: 2,
            },
            BuildingAction {
                direction: BuildingDirection::Up,
                amount: 2,
            },
            BuildingAction {
                direction: BuildingDirection::Left,
                amount: 1,
            },
        ],
        &world,
        Some(&navigation),
        &content,
        &simulation,
    );

    let cells = placement_visual_cells(&placement);
    assert_eq!(cells.first(), Some(&origin));
    assert_eq!(cells.last(), placement.navigation_position.as_ref());
    assert!(
        cells
            .windows(2)
            .all(|pair| pair[0].x.abs_diff(pair[1].x) <= 1 && pair[0].z.abs_diff(pair[1].z) <= 1)
    );
    assert!(
        cells.windows(2).any(|pair| {
            pair[0].x.abs_diff(pair[1].x) == 1 && pair[0].z.abs_diff(pair[1].z) == 1
        })
    );
    assert!(cells.iter().any(|cell| cell.x != origin.x));
    assert!(cells.iter().any(|cell| cell.z != origin.z));
    assert_eq!(
        content.buildings[&StableId::new("building:path").unwrap()].placement_footprint_thirds,
        Some([1, 1])
    );
    assert_eq!(
        content.buildings[&StableId::new("building:path").unwrap()].navigation_footprint_thirds,
        Some([0, 0])
    );

    assert!(building_placement_is_available(
        &content,
        &simulation,
        &world,
        &placement,
        &content.buildings[&StableId::new("building:path").unwrap()],
        Some(&navigation),
    ));

    let mut occupied = simulation;
    occupied.path_navigation_positions.insert(
        StableId::new("building:runtime_path_occupied").unwrap(),
        cells[2],
    );
    assert!(building_placement_is_available(
        &content,
        &occupied,
        &world,
        &placement,
        &content.buildings[&StableId::new("building:path").unwrap()],
        Some(&navigation),
    ));
    assert!(
        !confirmed_path_cells(&content, &occupied, &placement, &navigation).contains(&cells[2])
    );
}

#[test]
fn thick_paths_keep_the_centre_route_and_skip_each_blocked_flank_independently() {
    let content = embedded_content();
    let mut navigation =
        stream_town_domain::NavGrid::new(6, 4, vec![false; 24], vec![0; 24]).unwrap();
    navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: GridPos { x: 2, z: 0 },
                max: GridPos { x: 2, z: 0 },
            },
            true,
        )
        .unwrap();
    let mut simulation = WorldSimulation::new(1);
    simulation.path_navigation_positions.insert(
        StableId::new("building:occupied_thick_flank").unwrap(),
        GridPos { x: 3, z: 2 },
    );
    let placement = BuildingPlacement {
        building: StableId::new("building:path").unwrap(),
        thick_path: true,
        position: GridPos { x: 0, z: 0 },
        navigation_position: Some(GridPos { x: 3, z: 1 }),
        rotation_quarter_turns: 1,
        line_start: Some(GridPos { x: 1, z: 1 }),
        line_end: None,
        path_cells: vec![
            GridPos { x: 1, z: 1 },
            GridPos { x: 2, z: 1 },
            GridPos { x: 3, z: 1 },
        ],
        inactivity_seconds: 0.0,
    };
    let world = generate_world(&GameConfig::default().world);
    assert!(building_placement_is_available(
        &content,
        &simulation,
        &world,
        &placement,
        &content.buildings[&placement.building],
        Some(&navigation),
    ));
    assert_eq!(placement_visual_cells(&placement).len(), 9);
    let confirmed = confirmed_path_cells(&content, &simulation, &placement, &navigation);
    assert_eq!(confirmed.len(), 7);
    assert!(
        placement
            .path_cells
            .iter()
            .all(|cell| confirmed.contains(cell))
    );
    assert!(!confirmed.contains(&GridPos { x: 2, z: 0 }));
    assert!(!confirmed.contains(&GridPos { x: 3, z: 2 }));
}

#[test]
fn diagonal_thick_paths_fill_both_inner_bridge_cells() {
    let content = embedded_content();
    let navigation = stream_town_domain::NavGrid::new(5, 5, vec![false; 25], vec![0; 25]).unwrap();
    let simulation = WorldSimulation::new(1);
    let placement = BuildingPlacement {
        building: StableId::new("building:path").unwrap(),
        thick_path: true,
        position: GridPos { x: 0, z: 0 },
        navigation_position: Some(GridPos { x: 2, z: 2 }),
        rotation_quarter_turns: 0,
        line_start: Some(GridPos { x: 1, z: 1 }),
        line_end: None,
        path_cells: vec![GridPos { x: 1, z: 1 }, GridPos { x: 2, z: 2 }],
        inactivity_seconds: 0.0,
    };

    let visual = placement_visual_cells(&placement);
    assert_eq!(visual.len(), 8);
    assert!(visual.contains(&GridPos { x: 1, z: 2 }));
    assert!(visual.contains(&GridPos { x: 2, z: 1 }));
    assert_eq!(
        confirmed_path_cells(&content, &simulation, &placement, &navigation),
        visual
    );
}

#[test]
fn path_construction_prefers_cell_centres_without_forbidding_side_thirds() {
    assert_eq!(path_construction_step_cost(GridPos { x: 4, z: 7 }, 10), 10);
    assert_eq!(path_construction_step_cost(GridPos { x: 5, z: 7 }, 10), 12);
    assert_eq!(path_construction_step_cost(GridPos { x: 5, z: 8 }, 14), 18);
}

#[test]
fn agents_use_an_open_side_third_when_the_approach_centre_is_blocked() {
    let content = embedded_content();
    let simulation = WorldSimulation::new(1);
    let start = GridPos { x: 0, z: 4 };
    let placement_goal = GridPos { x: 1, z: 1 };
    let centre = placement_to_navigation_centre(placement_goal);
    let mut navigation =
        stream_town_domain::NavGrid::new(9, 9, vec![true; 81], vec![0; 81]).unwrap();
    navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: start,
                max: GridPos { x: 3, z: 4 },
            },
            false,
        )
        .unwrap();

    let path = try_fine_agent_path(
        &navigation,
        &content,
        &simulation,
        &PathSurfaceRuntime::default(),
        &ActorKind::Player,
        start,
        placement_goal,
    )
    .expect("an open third of the coarse approach cell should remain reachable");

    assert_eq!(path.last(), Some(&GridPos { x: 3, z: 4 }));
    assert_ne!(path.last(), Some(&centre));
    assert_eq!(
        navigation_to_placement(*path.last().unwrap()),
        placement_goal
    );
}

#[test]
fn agents_try_reachable_side_thirds_when_a_walkable_centre_is_disconnected() {
    let content = embedded_content();
    let simulation = WorldSimulation::new(1);
    let start = GridPos { x: 0, z: 1 };
    let placement_goal = GridPos { x: 1, z: 0 };
    let centre = placement_to_navigation_centre(placement_goal);
    let mut navigation =
        stream_town_domain::NavGrid::new(6, 3, vec![false; 18], vec![0; 18]).unwrap();
    for blocked in [
        GridPos { x: 3, z: 1 },
        GridPos { x: 4, z: 0 },
        GridPos { x: 4, z: 2 },
        GridPos { x: 5, z: 1 },
    ] {
        navigation
            .set_blocked(
                stream_town_domain::DirtyRegion {
                    min: blocked,
                    max: blocked,
                },
                true,
            )
            .unwrap();
    }
    assert!(navigation.is_walkable(centre));
    assert!(navigation.find_path(start, centre).is_err());

    let path = try_fine_agent_path(
        &navigation,
        &content,
        &simulation,
        &PathSurfaceRuntime::default(),
        &ActorKind::Player,
        start,
        placement_goal,
    )
    .expect("a reachable side third must remain a valid approach");

    assert_ne!(path.last(), Some(&centre));
    assert_eq!(
        navigation_to_placement(*path.last().unwrap()),
        placement_goal
    );
}

#[test]
fn paths_can_route_through_the_three_completed_gate_door_cells() {
    let content = embedded_content();
    let gate_id = StableId::new("building:gate").unwrap();
    let gate = &content.buildings[&gate_id];
    let runtime_id = StableId::new("building:gate_path_test").unwrap();
    let mut simulation = WorldSimulation::new(1);
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id,
            archetype: gate.archetype.clone(),
            position: GridPos { x: 1, z: 1 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 100,
            complete: true,
        },
    );
    let gate_cells = completed_player_gate_navigation_cells(&content, &simulation);
    assert_eq!(gate_cells.len(), 3);
    let mut navigation =
        stream_town_domain::NavGrid::new(9, 9, vec![false; 81], vec![0; 81]).unwrap();
    for cell in &gate_cells {
        navigation
            .set_blocked(
                stream_town_domain::DirtyRegion {
                    min: *cell,
                    max: *cell,
                },
                true,
            )
            .unwrap();
    }
    let mut ordered = gate_cells.iter().copied().collect::<Vec<_>>();
    ordered.sort_unstable();
    let route =
        routed_path_cells(&navigation, &content, &simulation, ordered[0], ordered[2]).unwrap();
    assert!(ordered.iter().all(|cell| route.contains(cell)));
}

#[test]
fn coarse_paths_migrate_once_from_coarse_cells_without_overlap() {
    let mut simulation = WorldSimulation::new(9);
    for suffix in ["a", "b"] {
        let id = StableId::new(format!("building:path_{suffix}")).unwrap();
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id,
                archetype: StableId::new("archetype:building:path").unwrap(),
                position: GridPos { x: 4, z: 7 },
                rotation_quarter_turns: 0,
                level: 1,
                health: 100,
                complete: true,
            },
        );
    }

    assert_eq!(upgrade_path_navigation_positions(&mut simulation), 2);
    let positions = simulation
        .path_navigation_positions
        .values()
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(positions.len(), 2);
    assert!(positions.contains(&GridPos { x: 13, z: 22 }));
    assert!(
        positions
            .iter()
            .all(|position| (12..=14).contains(&position.x) && (21..=23).contains(&position.z))
    );
    assert_eq!(upgrade_path_navigation_positions(&mut simulation), 0);
}

#[test]
fn legacy_path_centres_expand_into_straight_and_corner_connectors() {
    let content = embedded_content();
    let path_definition = &content.buildings[&StableId::new("building:path").unwrap()];
    let mut simulation = WorldSimulation::new(10);
    let mut add_path = |name: &str, coarse: GridPos, fine: GridPos| {
        let id = StableId::new(format!("building:{name}")).unwrap();
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id: id.clone(),
                archetype: path_definition.archetype.clone(),
                position: coarse,
                rotation_quarter_turns: 0,
                level: 1,
                health: 100,
                complete: true,
            },
        );
        simulation.path_navigation_positions.insert(id, fine);
    };
    add_path(
        "path_corner",
        GridPos { x: 3, z: 3 },
        GridPos { x: 10, z: 10 },
    );
    add_path(
        "path_right",
        GridPos { x: 4, z: 3 },
        GridPos { x: 13, z: 10 },
    );
    add_path("path_up", GridPos { x: 3, z: 4 }, GridPos { x: 10, z: 13 });
    let path_id = StableId::new("building:path").unwrap();
    let corner_id = StableId::new("building:path_corner").unwrap();
    let corner = linear_navigation_cells(
        &content,
        &simulation,
        &simulation.buildings[&corner_id],
        &path_id,
    );
    assert_eq!(
        corner,
        vec![
            GridPos { x: 10, z: 10 },
            GridPos { x: 10, z: 11 },
            GridPos { x: 11, z: 10 },
        ]
    );
    let right_id = StableId::new("building:path_right").unwrap();
    assert_eq!(
        linear_navigation_cells(
            &content,
            &simulation,
            &simulation.buildings[&right_id],
            &path_id,
        ),
        vec![
            GridPos { x: 12, z: 10 },
            GridPos { x: 13, z: 10 },
            GridPos { x: 14, z: 10 },
        ]
    );
}

#[test]
fn path_route_cost_matches_the_five_percent_per_level_speed_bonus() {
    assert_eq!(path_route_step_cost(10, 0), 1_000);
    assert_eq!(path_route_step_cost(10, 1), 953);
    assert!(path_route_step_cost(10, 10) < path_route_step_cost(10, 1));
}

#[test]
fn routed_paths_traverse_existing_paths_without_rebuilding_them() {
    let navigation = stream_town_domain::NavGrid::new(5, 1, vec![false; 5], vec![0; 5]).unwrap();
    let mut simulation = WorldSimulation::new(1);
    let occupied = GridPos { x: 2, z: 0 };
    simulation
        .path_navigation_positions
        .insert(StableId::new("building:existing_path").unwrap(), occupied);

    let route = routed_path_cells(
        &navigation,
        &embedded_content(),
        &simulation,
        GridPos { x: 0, z: 0 },
        GridPos { x: 4, z: 0 },
    )
    .unwrap();
    assert!(route.contains(&occupied));

    let placement = BuildingPlacement {
        building: StableId::new("building:path").unwrap(),
        thick_path: false,
        position: GridPos { x: 1, z: 0 },
        navigation_position: Some(GridPos { x: 4, z: 0 }),
        rotation_quarter_turns: 0,
        line_start: Some(GridPos { x: 0, z: 0 }),
        line_end: None,
        path_cells: route,
        inactivity_seconds: 0.0,
    };
    let confirmed = confirmed_path_cells(&embedded_content(), &simulation, &placement, &navigation);
    assert!(!confirmed.contains(&occupied));
    assert_eq!(confirmed.len(), 4);
}

#[test]
fn enemies_receive_health_bars_without_a_player_only_authored_timeout() {
    let content = embedded_content();
    let enemy_archetype =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab")
            .expect("shipping Goblin archetype");
    let enemy = StableId::new("actor:enemy_health_bar").unwrap();
    let mut simulation = WorldSimulation::new(1);
    assert!(simulation.spawn_enemy(enemy.clone(), enemy_archetype, GridPos { x: 1, z: 1 }, 100,));
    assert_eq!(
        actor_health_bar_hide_seconds(&content, &simulation.actors[&enemy]),
        Some(3.0)
    );
    assert_eq!(
        actor_health_fill_color(&simulation.actors[&enemy]),
        Color::srgb(0.58, 0.16, 0.86)
    );
}

#[test]
fn guardhouse_defenders_round_trip_to_their_station_without_using_recruit_capacity() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let guardhouse = &content.buildings[&StableId::new("building:guardhouse").unwrap()];
    let building_id = StableId::new("building:runtime_guardhouse_test").unwrap();
    let mut simulation = WorldSimulation::new(1);
    simulation.buildings.insert(
        building_id.clone(),
        BuildingState {
            id: building_id.clone(),
            archetype: guardhouse.archetype.clone(),
            position: GridPos { x: 4, z: 5 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 150,
            complete: true,
        },
    );
    let defender = guardhouse_defender_id(&building_id);
    let defender_position = GridPos { x: 20, z: 20 };
    assert!(simulation.join_player(defender.clone(), defender_position));
    simulation
        .assign_role(&defender, StableId::new("role:defender").unwrap())
        .unwrap();
    let mut recruited_defender = simulation.actors[&defender].clone();
    recruited_defender.id = StableId::new("npc:recruit_health_reference").unwrap();
    let guardhouse_health =
        effective_role_stats(&content, &simulation, &simulation.actors[&defender])
            .unwrap()
            .max_health;
    let recruited_health = effective_role_stats(&content, &simulation, &recruited_defender)
        .unwrap()
        .max_health;
    assert_eq!(guardhouse_health, recruited_health / 2);
    assert_eq!(
        guardhouse_for_defender(&simulation, &defender).unwrap().id,
        building_id
    );
    assert!(recruited_actor_ids(&simulation).is_empty());
    assert!(capacity_recruited_actor_ids(&simulation).is_empty());
    assert_eq!(simulation_player_count(&simulation), 0);

    let nearby = StableId::new("enemy:guard_nearby").unwrap();
    let near_guardhouse = StableId::new("enemy:near_guardhouse").unwrap();
    assert!(simulation.join_player(nearby.clone(), GridPos { x: 19, z: 20 }));
    assert!(simulation.join_player(near_guardhouse.clone(), GridPos { x: 5, z: 5 }));
    for enemy in [&nearby, &near_guardhouse] {
        simulation
            .assign_role(enemy, StableId::new("role:enemy").unwrap())
            .unwrap();
    }
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &guardhouse_defender_id(&building_id),
            defender_position,
        )
        .0,
        AgentGoal::Attack(nearby.clone())
    );
    simulation.actors.get_mut(&nearby).unwrap().alive = false;
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &guardhouse_defender_id(&building_id),
            defender_position,
        )
        .0,
        AgentGoal::Attack(near_guardhouse)
    );
}

#[test]
fn endless_path_failures_trigger_only_after_five_seconds() {
    let actor = StableId::new("actor:path_failure").unwrap();
    let mut failures = PathFailureRuntime::default();
    assert!(!failures.record_failure(&actor, 4.99));
    assert!(failures.record_failure(&actor, 0.01));
    failures.clear(&actor);
    assert!(!failures.record_failure(&actor, 0.01));
}

#[test]
fn valid_route_without_world_progress_triggers_unstuck() {
    let actor = StableId::new("actor:movement_failure").unwrap();
    let target = GridPos { x: 8, z: 8 };
    let mut failures = PathFailureRuntime::default();
    assert!(!failures.record_movement(&actor, Vec2::ZERO, 1, target, 0.0, 0.1));
    assert!(!failures.record_movement(&actor, Vec2::ZERO, 1, target, 4.99, 0.1));
    assert!(failures.record_movement(&actor, Vec2::ZERO, 1, target, 0.01, 0.1));

    // Meaningful displacement resets the stall window even if the actor is
    // still working toward the same path waypoint.
    assert!(!failures.record_movement(&actor, Vec2::X, 1, target, 4.99, 0.1));
    assert!(!failures.record_movement(&actor, Vec2::X, 1, target, 0.0, 0.1));
}

#[test]
fn unstuck_recovery_rejects_a_walkable_but_isolated_fine_cell() {
    let actor = StableId::new("actor:fine_recovery").unwrap();
    let desired = GridPos { x: 10, z: 10 };
    let mut navigation =
        stream_town_domain::NavGrid::new(20, 20, vec![false; 400], vec![0; 400]).unwrap();
    for blocked in [
        GridPos { x: 9, z: 9 },
        GridPos { x: 10, z: 9 },
        GridPos { x: 11, z: 9 },
        GridPos { x: 9, z: 10 },
        GridPos { x: 11, z: 10 },
        GridPos { x: 9, z: 11 },
        GridPos { x: 10, z: 11 },
        GridPos { x: 11, z: 11 },
    ] {
        navigation
            .set_blocked(
                stream_town_domain::DirtyRegion {
                    min: blocked,
                    max: blocked,
                },
                true,
            )
            .unwrap();
    }
    assert!(navigation.is_walkable(desired));
    assert!(!navigation_component_reaches_minimum(
        &navigation,
        desired,
        64
    ));

    let recovered =
        fine_navigation_recovery_position(&navigation, desired, &actor, &BTreeMap::new()).unwrap();

    assert_ne!(recovered, desired);
    assert!(navigation_component_reaches_minimum(
        &navigation,
        recovered,
        64
    ));
}

#[test]
fn player_name_presets_are_deterministic_and_catalogued() {
    let actor = StableId::new("twitch:colour_test").unwrap();
    let color = preset_player_name_color(44, &actor);
    assert_eq!(color, preset_player_name_color(44, &actor));
    assert!(PLAYER_NAME_COLOR_PRESETS.contains(&color));
}

#[test]
fn healing_effect_extent_stays_within_character_scale() {
    let presentation = embedded_presentation();
    for kind in [
        HealingEffectKind::Channel,
        HealingEffectKind::Burst,
        HealingEffectKind::Revive,
    ] {
        let duration = healing_effect_duration(&presentation, kind);
        for step in 0_u16..=100 {
            let sample = healing_effect_sample(
                &presentation,
                kind,
                duration * f32::from(step) / 100.0,
                duration,
            );
            let target_scale = if kind == HealingEffectKind::Burst {
                HEALING_TARGET_EFFECT_SCALE
            } else {
                1.0
            };
            let ring_base_scale = match kind {
                HealingEffectKind::Channel => {
                    healing_channel_effect(&presentation).1.exposed_size
                        * HEALING_CHANNEL_RING_SCALE
                }
                HealingEffectKind::Burst | HealingEffectKind::Revive => HEALING_BURST_RING_SCALE,
            } * target_scale;
            let ring_world_radius = ring_base_scale * sample.ring_scale;
            let mote_transform_scale = if kind == HealingEffectKind::Channel {
                HEALING_MOTE_SCALE * sample.mote_scale * 1.2
            } else {
                HEALING_MOTE_SCALE
                    * healing_burst_effect(&presentation)
                        .1
                        .plus_size_multiplier(f32::from(step) / 100.0)
                        .unwrap_or_default()
                    * if kind == HealingEffectKind::Revive {
                        1.25
                    } else {
                        1.0
                    }
            };
            // VFX_Plus.glb has a six-metre maximum imported extent; the
            // channel uses the unit-diameter procedural sphere. Their
            // final geometry, not merely their transforms, stays hand-sized.
            let mote_mesh_extent = if kind == HealingEffectKind::Channel {
                1.0
            } else {
                6.000_001
            };
            let mote_world_extent = mote_transform_scale * mote_mesh_extent * target_scale;
            assert!(ring_world_radius <= 0.56 + f32::EPSILON, "{kind:?}");
            assert!(mote_world_extent <= 0.14 + f32::EPSILON, "{kind:?}");
            assert!(
                sample.radial_distance * target_scale <= 0.56 + f32::EPSILON,
                "{kind:?}"
            );
            assert!(sample.rise <= 0.9 + f32::EPSILON, "{kind:?}");
        }
    }
    assert!((HEALING_TARGET_EFFECT_SCALE - 1.0 / 8.0).abs() < f32::EPSILON);
    let material = healing_material([0.266_204_9, 16.948_38, 0.0, 1.0], 1.0);
    let emissive = material.emissive.to_f32_array();
    assert!(
        emissive[..3]
            .iter()
            .all(|component| *component <= HEALING_EMISSIVE_MAX_COMPONENT)
    );
}

#[test]
fn hud_building_count_excludes_town_hall_walls_paths_and_towers() {
    let content = embedded_content();
    let mut simulation = WorldSimulation::new(1);
    for (runtime, definition_id) in [
        ("building:townhall", "building:townhall"),
        ("building:test_wall", "building:wall"),
        ("building:test_path", "building:path"),
        ("building:test_tower", "building:tower"),
        ("building:test_house", "building:house"),
    ] {
        let definition = &content.buildings[&StableId::new(definition_id).unwrap()];
        let id = StableId::new(runtime).unwrap();
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id,
                archetype: definition.archetype.clone(),
                position: GridPos { x: 4, z: 4 },
                rotation_quarter_turns: 0,
                level: 1,
                health: 100,
                complete: true,
            },
        );
    }
    assert_eq!(hud_building_count(&content, &simulation), 1);
}

#[test]
fn transient_dx12_surface_configuration_errors_are_recoverable() {
    let error = RenderError {
        ty: ErrorType::Validation,
        description: "In Surface::configure\n  Invalid surface".to_owned(),
        source: None,
    };
    assert!(is_transient_surface_configuration_error(&error));
    let unrelated = RenderError {
        ty: ErrorType::Validation,
        description: "some other validation error".to_owned(),
        source: None,
    };
    assert!(!is_transient_surface_configuration_error(&unrelated));
}

#[test]
fn water_shader_uses_scene_lighting() {
    let shader = include_str!("../../../../assets/shaders/water_material.wgsl");
    assert!(shader.contains("apply_pbr_lighting(pbr_input)"));
    assert!(!shader.contains("keep it unlit"));
}

#[test]
fn enemy_flow_field_routes_each_component_to_its_nearest_town_target() {
    let width = 16_u16;
    let height = 8_u16;
    let mut blocked = vec![false; usize::from(width) * usize::from(height)];
    for z in 0..height {
        blocked[usize::from(z) * usize::from(width) + 7] = true;
    }
    let navigation = stream_town_domain::NavGrid::new(
        width,
        height,
        blocked,
        vec![0; usize::from(width) * usize::from(height)],
    )
    .unwrap();
    let wall = StableId::new("building:test_wall").unwrap();
    let town_hall = StableId::new("building:townhall").unwrap();
    let field = build_enemy_navigation_field(
        42,
        navigation.clone(),
        vec![
            EnemyRouteBuilding {
                id: town_hall.clone(),
                town_hall_distance: 0,
                approaches: vec![GridPos { x: 12, z: 4 }],
            },
            EnemyRouteBuilding {
                id: wall.clone(),
                town_hall_distance: 10,
                approaches: vec![GridPos { x: 6, z: 4 }, GridPos { x: 8, z: 4 }],
            },
        ],
    );

    let left = GridPos { x: 1, z: 1 };
    let right = GridPos { x: 14, z: 1 };
    let (left_target, left_goal) = field.destination(left).unwrap();
    let (right_target, right_goal) = field.destination(right).unwrap();
    assert_eq!(left_target, &wall);
    assert_eq!(left_goal, GridPos { x: 6, z: 4 });
    assert_eq!(right_target, &town_hall);
    assert_eq!(right_goal, GridPos { x: 12, z: 4 });
    let route = field.path_to(left, &wall, left_goal).unwrap();
    assert_eq!(route.first(), Some(&left));
    assert_eq!(route.last(), Some(&left_goal));
    assert!(
        route
            .iter()
            .all(|position| navigation.is_walkable(*position))
    );
}

#[test]
fn hierarchical_enemy_path_refines_a_valid_cluster_corridor() {
    let width = 24_u16;
    let height = 24_u16;
    let mut blocked = vec![false; usize::from(width) * usize::from(height)];
    for z in 0..height {
        if z != 5 {
            blocked[usize::from(z) * usize::from(width) + 8] = true;
        }
        if z != 18 {
            blocked[usize::from(z) * usize::from(width) + 16] = true;
        }
    }
    let navigation = stream_town_domain::NavGrid::new(
        width,
        height,
        blocked,
        vec![0; usize::from(width) * usize::from(height)],
    )
    .unwrap();
    let field = build_enemy_navigation_field(7, navigation.clone(), Vec::new());
    let start = GridPos { x: 2, z: 2 };
    let goal = GridPos { x: 22, z: 22 };
    let route = field.hierarchical_path(&navigation, start, goal).unwrap();
    assert_eq!(route.first(), Some(&start));
    assert_eq!(route.last(), Some(&goal));
    assert!(
        route
            .iter()
            .all(|position| navigation.is_walkable(*position))
    );
    assert!(route.windows(2).all(|cells| {
        let x = cells[0].x.abs_diff(cells[1].x);
        let z = cells[0].z.abs_diff(cells[1].z);
        x <= 1 && z <= 1 && x + z > 0
    }));
    assert!(route.windows(2).any(|cells| {
        cells[0].x.abs_diff(cells[1].x) == 1 && cells[0].z.abs_diff(cells[1].z) == 1
    }));
}
