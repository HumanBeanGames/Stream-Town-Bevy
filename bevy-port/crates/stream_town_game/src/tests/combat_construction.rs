#[test]
fn combat_goal_damages_kills_and_respawns() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let (defender_position, enemy_position) = (0..world.navigation.height())
        .find_map(|z| {
            (0..world.navigation.width().saturating_sub(1)).find_map(|x| {
                let defender = GridPos { x, z };
                let enemy = GridPos { x: x + 1, z };
                (world.navigation.is_walkable(defender) && world.navigation.is_walkable(enemy))
                    .then_some((defender, enemy))
            })
        })
        .expect("generated world has adjacent combat cells");
    let defender = StableId::new("npc:defender_test").unwrap();
    let enemy = StableId::new("actor:enemy_test").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(defender.clone(), defender_position));
    assert!(simulation.join_player(enemy.clone(), enemy_position));
    simulation
        .assign_role(&defender, StableId::new("role:defender").unwrap())
        .unwrap();
    simulation
        .assign_role(&enemy, StableId::new("role:enemy").unwrap())
        .unwrap();
    let (goal, target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &defender,
        defender_position,
    );
    assert_eq!(goal, AgentGoal::Attack(enemy.clone()));
    assert_eq!(target, defender_position);
    for _ in 0..100 {
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &defender,
            &goal,
            defender_position,
        );
    }
    assert!(!simulation.actors[&enemy].alive);
    assert_eq!(simulation.actors[&enemy].health, 0);
    simulation.respawn_actor(&enemy, enemy_position).unwrap();
    assert!(simulation.actors[&enemy].alive);
    assert_eq!(simulation.actors[&enemy].health, 100);
    let expected_cooldown =
        effective_role_stats(&content, &simulation, &simulation.actors[&defender])
            .map(|stats| milli_units_as_f32(stats.action_milliseconds))
            .unwrap();
    assert!(
        (action_cooldown(&content, &simulation, &defender, &goal) - expected_cooldown).abs()
            <= f32::EPSILON
    );
}

#[test]
fn every_combat_role_paths_to_strikes_and_kills_an_enemy() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world_with_content(&config.world, &content);
    let (start, target) = (0..world.navigation.height())
        .find_map(|z| {
            (0..world.navigation.width()).find_map(|x| {
                let start = GridPos { x, z };
                if !world.navigation.is_walkable(start) {
                    return None;
                }
                (7..=12).find_map(|offset| {
                    let target = GridPos {
                        x: x.checked_add(offset)?,
                        z,
                    };
                    (target.x < world.navigation.width() && world.navigation.is_walkable(target))
                        .then(|| world.navigation.find_path(start, target).ok())
                        .flatten()
                        .filter(|path| path.len() > 2)
                        .map(|_| (start, target))
                })
            })
        })
        .expect("generated world has a combat route");
    let goblin =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();

    for role_name in [
        "role:defender",
        "role:necromancer",
        "role:paladin",
        "role:ranger",
        "role:ruler",
        "role:soldier",
        "role:wizard",
    ] {
        let role = StableId::new(role_name).unwrap();
        let attacker =
            StableId::new(format!("npc:pathing_{}", role_name.replace(':', "_"))).unwrap();
        let enemy = StableId::new(format!("actor:target_{}", role_name.replace(':', "_"))).unwrap();
        let mut simulation = WorldSimulation::new(world.seed);
        assert!(simulation.join_player(attacker.clone(), start));
        simulation.assign_role(&attacker, role).unwrap();
        let damage = effective_role_stats(&content, &simulation, &simulation.actors[&attacker])
            .unwrap()
            .action_amount;
        assert!(damage > 0, "{role_name} has no attack damage");
        assert!(simulation.spawn_enemy(
            enemy.clone(),
            goblin.clone(),
            target,
            i32::try_from(damage).unwrap(),
        ));

        let (goal, destination) =
            next_agent_goal(&simulation, &world, &config, &content, &attacker, start);
        assert_eq!(goal, AgentGoal::Attack(enemy.clone()), "{role_name}");
        assert_ne!(destination, start, "{role_name} never requested a path");
        let routed = try_agent_path(
            &world.navigation,
            &content,
            &simulation,
            &ActorKind::Player,
            start,
            destination,
        )
        .unwrap_or_else(|| panic!("{role_name} could not route to its enemy"));
        assert!(routed.len() > 1, "{role_name} produced an empty route");
        assert_eq!(routed.last(), Some(&destination), "{role_name}");
        simulation.actors.get_mut(&attacker).unwrap().position = destination;

        let presentation = complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &attacker,
            &goal,
            destination,
        )
        .unwrap_or_else(|| panic!("{role_name} did not execute its attack"));
        if let ActionPresentation::Projectile(spawn) = presentation {
            let projectile = CombatProjectile {
                source: spawn.source,
                target: spawn.target,
                damage: spawn.damage,
                speed_cells_per_second: spawn.speed_cells_per_second,
                visual: spawn.visual,
                trail_cooldown_seconds: 0.0,
                remaining_seconds: PROJECTILE_MAX_LIFETIME_SECONDS,
            };
            resolve_combat_projectile_impact(&config, &mut simulation, &content, &projectile)
                .unwrap();
        }
        assert!(!simulation.actors[&enemy].alive, "{role_name} did not kill");
        assert_eq!(simulation.actors[&enemy].health, 0, "{role_name}");
    }
}

#[test]
fn enemy_attacks_can_kill_citizens() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world_with_content(&config.world, &content);
    let (enemy_position, citizen_position) = (0..world.navigation.height())
        .find_map(|z| {
            (0..world.navigation.width().saturating_sub(1)).find_map(|x| {
                let enemy = GridPos { x, z };
                let citizen = GridPos { x: x + 1, z };
                (world.navigation.is_walkable(enemy) && world.navigation.is_walkable(citizen))
                    .then_some((enemy, citizen))
            })
        })
        .unwrap();
    let blargul =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Blargul.prefab").unwrap();
    let damage = content.archetypes[&blargul]
        .enemy
        .as_ref()
        .unwrap()
        .action_amount;
    let enemy = StableId::new("actor:citizen_killer").unwrap();
    let citizen = StableId::new("npc:citizen_victim").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.spawn_enemy(enemy.clone(), blargul, enemy_position, 100));
    assert!(simulation.join_player(citizen.clone(), citizen_position));
    simulation.actors.get_mut(&citizen).unwrap().health = i32::try_from(damage).unwrap();
    simulation.actors.get_mut(&citizen).unwrap().max_health = i32::try_from(damage).unwrap();

    let (goal, destination) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &enemy,
        enemy_position,
    );
    assert_eq!(goal, AgentGoal::Attack(citizen.clone()));
    assert_eq!(destination, enemy_position);
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &enemy,
            &goal,
            enemy_position,
        )
        .is_some()
    );
    assert!(!simulation.actors[&citizen].alive);
    assert_eq!(simulation.actors[&citizen].health, 0);
    simulation.actors.get_mut(&citizen).unwrap().display_name = Some("Ada".to_owned());
    assert_eq!(
        citizen_death_announcement(&simulation.actors[&citizen]),
        "Ada has died! They will respawn at the Town Hall in 10 minutes."
    );
}

#[test]
fn tower_projectile_selects_hits_and_kills_an_enemy() {
    let config = GameConfig::default();
    let content = embedded_content();
    let tower_definition_id = StableId::new("building:tower").unwrap();
    let tower_definition = &content.buildings[&tower_definition_id];
    let shooter = tower_definition.projectile_shooter.as_ref().unwrap();
    let tower = StableId::new("building:tower_projectile_test").unwrap();
    let enemy = StableId::new("actor:tower_target").unwrap();
    let goblin =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();
    let mut simulation = WorldSimulation::new(42);
    simulation.buildings.insert(
        tower.clone(),
        BuildingState {
            id: tower.clone(),
            archetype: tower_definition.archetype.clone(),
            position: GridPos { x: 20, z: 20 },
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(&content, tower_definition)).unwrap(),
            complete: true,
        },
    );
    assert!(simulation.spawn_enemy(
        enemy.clone(),
        goblin,
        GridPos { x: 23, z: 20 },
        i32::try_from(shooter.damage).unwrap(),
    ));

    let (spawn, cooldown) = tower_projectile_for_building(&content, &simulation, &tower).unwrap();
    assert!(matches!(spawn.source, ProjectileSource::Building(ref id) if id == &tower));
    assert_eq!(spawn.target, enemy);
    assert_eq!(spawn.damage, shooter.damage);
    assert!((spawn.speed_cells_per_second - 15.0).abs() <= f32::EPSILON);
    assert!((cooldown - 3.0).abs() <= f32::EPSILON);
    let projectile = CombatProjectile {
        source: spawn.source,
        target: spawn.target,
        damage: spawn.damage,
        speed_cells_per_second: spawn.speed_cells_per_second,
        visual: spawn.visual,
        trail_cooldown_seconds: 0.0,
        remaining_seconds: PROJECTILE_MAX_LIFETIME_SECONDS,
    };
    assert!(
        resolve_combat_projectile_impact(&config, &mut simulation, &content, &projectile,).unwrap()
    );
    assert!(!simulation.actors[&enemy].alive);
    assert_eq!(simulation.actors[&enemy].health, 0);
}

#[test]
fn shipping_world_seeds_authored_enemy_camps_deterministically() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut first_world = generate_world(&config.world);
    let mut first = WorldSimulation::new(first_world.seed);
    ensure_town_hall_state(&content, &config, &mut first);
    let town_hall = &first.buildings[&StableId::new("building:townhall").unwrap()];
    let town_hall_definition = building_def_for_archetype(&content, &town_hall.archetype).unwrap();
    first_world
        .navigation
        .set_blocked(
            building_navigation_region(
                town_hall.position,
                town_hall_definition,
                town_hall.rotation_quarter_turns,
                &first_world,
            )
            .unwrap(),
            true,
        )
        .unwrap();
    let expected = content
        .enemy_camp_generation
        .iter()
        .map(|layer| usize::from(layer.maximum_camps))
        .sum::<usize>();
    let spawned = seed_generated_enemy_camps(&config, &content, &mut first_world, &mut first);
    assert!(spawned > 0 && spawned <= expected);
    assert_eq!(first.enemy_camps.len(), spawned);

    let mut centres = Vec::new();
    for camp in first.enemy_camps.values() {
        let archetype = &content.archetypes[&camp.archetype];
        let region =
            enemy_camp_navigation_region(camp.position, archetype.footprint, &first_world).unwrap();
        assert!(camp.spawn_remaining_seconds.abs() <= f64::EPSILON);
        assert!(!first_world.navigation.is_walkable(region.min));
        let camp_spawner = archetype.enemy_spawner.as_ref().unwrap();
        for serial in 0..camp_spawner.spawn_offsets_milli_cells.len() as u64 {
            let spawn = enemy_spawn_position(
                &first_world,
                &content,
                &first,
                None,
                camp,
                camp_spawner,
                archetype.footprint,
                serial,
            );
            assert!(first_world.navigation.is_walkable(spawn));
            assert!(!region_contains_grid_position(region, spawn));
            let (_, approach) =
                nearest_reachable_building_to_town_hall(&content, &first, &first_world, spawn)
                    .expect("camp spawn can reach a town building");
            assert!(first_world.navigation.find_path(spawn, approach).is_ok());
        }
        let centre = GridPos {
            x: camp.position.x + archetype.footprint[0] / 2,
            z: camp.position.z + archetype.footprint[1] / 2,
        };
        let layer = content
            .enemy_camp_generation
            .iter()
            .find(|layer| layer.camp_archetype == camp.archetype)
            .unwrap();
        for other in &centres {
            let distance = u128::from(grid_distance_squared(*other, centre)) * 1_000_000;
            let minimum = u128::from(layer.minimum_distance_between_camps_milli_cells);
            assert!(distance >= minimum * minimum);
        }
        centres.push(centre);
    }

    let mut second_world = generate_world(&config.world);
    let mut second = WorldSimulation::new(second_world.seed);
    ensure_town_hall_state(&content, &config, &mut second);
    second_world
        .navigation
        .set_blocked(
            building_navigation_region(
                second.buildings[&StableId::new("building:townhall").unwrap()].position,
                town_hall_definition,
                0,
                &second_world,
            )
            .unwrap(),
            true,
        )
        .unwrap();
    seed_generated_enemy_camps(&config, &content, &mut second_world, &mut second);
    assert_eq!(second.enemy_camps, first.enemy_camps);
}

#[test]
fn enemies_advance_on_town_hall_while_defenders_acquire_them() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let mut simulation = WorldSimulation::new(world.seed);
    ensure_town_hall_state(&content, &config, &mut simulation);
    let town_hall = StableId::new("building:townhall").unwrap();
    let hall_position = building_visual_grid(&content, &simulation.buildings[&town_hall]);
    let defender_position = nearest_walkable(&world, hall_position).unwrap();
    let hall_state = &simulation.buildings[&town_hall];
    let hall_definition = building_def_for_archetype(&content, &hall_state.archetype).unwrap();
    let enemy_position = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .find(|position| {
            world.navigation.is_walkable(*position)
                && grid_distance_squared(*position, hall_position) >= 100
                && within_player_target_search_region(*position, defender_position)
                && reachable_building_approach(
                    &world,
                    hall_state.position,
                    rotated_footprint(hall_definition.footprint, hall_state.rotation_quarter_turns),
                    *position,
                )
                .is_some()
        })
        .expect("test enemy can reach the Town Hall from outside attack range");
    let enemy_id = StableId::new("actor:town_assault_test").unwrap();
    let defender_id = StableId::new("npc:town_defender_test").unwrap();
    let goblin =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();
    let health = content.archetypes[&goblin]
        .health
        .as_ref()
        .unwrap()
        .max_health;
    assert!(simulation.spawn_enemy(
        enemy_id.clone(),
        goblin,
        enemy_position,
        i32::try_from(health).unwrap(),
    ));
    assert!(simulation.join_player(defender_id.clone(), defender_position));
    simulation
        .assign_role(&defender_id, StableId::new("role:defender").unwrap())
        .unwrap();

    let enemy_goal = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &enemy_id,
        enemy_position,
    );
    assert_eq!(enemy_goal.0, AgentGoal::AttackBuilding(town_hall));
    assert_ne!(enemy_goal.1, enemy_position);
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &defender_id,
            defender_position,
        )
        .0,
        AgentGoal::Attack(enemy_id),
    );
}

#[test]
fn sealed_town_hall_redirects_enemies_to_the_nearest_reachable_defense() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = GeneratedWorld {
        seed: 42,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(32, 9, vec![false; 32 * 9], vec![100; 32 * 9])
            .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    world
        .navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: GridPos { x: 20, z: 0 },
                max: GridPos { x: 20, z: 8 },
            },
            true,
        )
        .unwrap();
    let town_hall_definition = &content.buildings[&StableId::new("building:townhall").unwrap()];
    let wall_definition = &content.buildings[&StableId::new("building:wall").unwrap()];
    let path_definition = &content.buildings[&StableId::new("building:path").unwrap()];
    let town_hall = StableId::new("building:townhall").unwrap();
    let wall = StableId::new("building:wall_barrier").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.buildings.insert(
        town_hall.clone(),
        BuildingState {
            id: town_hall,
            archetype: town_hall_definition.archetype.clone(),
            position: GridPos { x: 24, z: 3 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 1_000,
            complete: true,
        },
    );
    simulation.buildings.insert(
        wall.clone(),
        BuildingState {
            id: wall.clone(),
            archetype: wall_definition.archetype.clone(),
            position: GridPos { x: 20, z: 4 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 1_000,
            complete: true,
        },
    );
    let path = StableId::new("building:path_decoy").unwrap();
    simulation.buildings.insert(
        path.clone(),
        BuildingState {
            id: path.clone(),
            archetype: path_definition.archetype.clone(),
            position: GridPos { x: 5, z: 4 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 1_000,
            complete: true,
        },
    );
    assert!(!enemy_can_attack_building(
        &content,
        &simulation.buildings[&path]
    ));
    let enemy = StableId::new("actor:sealed_town_attacker").unwrap();
    let goblin =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();
    assert!(simulation.spawn_enemy(enemy.clone(), goblin, GridPos { x: 2, z: 4 }, 100));

    let (goal, approach) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &enemy,
        GridPos { x: 2, z: 4 },
    );
    assert_eq!(goal, AgentGoal::AttackBuilding(wall));
    assert!(
        world
            .navigation
            .find_path(GridPos { x: 2, z: 4 }, approach)
            .is_ok()
    );
}

#[test]
fn coarse_building_occupancy_preserves_authored_placement_footprints() {
    let content = embedded_content();
    let world = GeneratedWorld {
        seed: 1,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(
            24,
            24,
            vec![false; 24 * 24],
            vec![100; 24 * 24],
        )
        .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    let origin = GridPos { x: 8, z: 8 };
    let marketplace = &content.buildings[&StableId::new("building:marketplace").unwrap()];
    let civilian = building_navigation_region(origin, marketplace, 0, &world).unwrap();
    assert_eq!(civilian.min, origin);
    assert_eq!(civilian.max, GridPos { x: 11, z: 11 });

    let stonemason = &content.buildings[&StableId::new("building:stonemason").unwrap()];
    let drop_off = building_navigation_region(origin, stonemason, 0, &world).unwrap();
    assert_eq!(drop_off.min, origin);
    assert_eq!(drop_off.max, GridPos { x: 9, z: 9 });

    let barracks = &content.buildings[&StableId::new("building:barracks").unwrap()];
    let military = building_navigation_region(origin, barracks, 0, &world).unwrap();
    assert_eq!(military.min, origin);
    assert_eq!(military.max, GridPos { x: 10, z: 10 });

    let camp = enemy_camp_navigation_region(origin, [7, 6], &world).unwrap();
    assert_eq!(camp.min, GridPos { x: 10, z: 9 });
    assert_eq!(camp.max, GridPos { x: 12, z: 11 });
}

#[test]
fn fine_navigation_decouples_physical_obstructions_from_placement_cells() {
    let mut config = GameConfig::default();
    config.world.width = 24;
    config.world.height = 24;
    let content = embedded_content();
    assert_eq!(
        content.buildings[&StableId::new("building:foodstorage").unwrap()]
            .navigation_footprint_thirds,
        Some([5, 4])
    );
    assert_eq!(
        content.buildings[&StableId::new("building:orestorage").unwrap()]
            .navigation_footprint_thirds,
        Some([4, 4])
    );
    assert_eq!(
        content.buildings[&StableId::new("building:woodstorage").unwrap()]
            .navigation_footprint_thirds,
        Some([5, 4])
    );
    assert_eq!(
        content.buildings[&StableId::new("building:windmill").unwrap()].navigation_footprint_thirds,
        Some([7, 7])
    );
    let house = &content.buildings[&StableId::new("building:house").unwrap()];
    assert_eq!(house.footprint, [2, 3]);
    assert_eq!(house.navigation_footprint_thirds, Some([7, 7]));
    let tower = &content.buildings[&StableId::new("building:tower").unwrap()];
    assert_eq!(tower.footprint, [1, 1]);
    assert_eq!(tower.navigation_footprint_thirds, Some([4, 4]));
    let mut world = GeneratedWorld {
        seed: 7,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(
            24,
            24,
            vec![false; 24 * 24],
            vec![100; 24 * 24],
        )
        .unwrap(),
        resources: vec![stream_town_domain::GeneratedResource {
            id: StableId::new("resource:fine-grid-test").unwrap(),
            kind: StableId::new("resource:wood").unwrap(),
            target_kind: StableId::new("target:tree").unwrap(),
            position: GridPos { x: 8, z: 8 },
            offset_milli_cells: [0, 0],
            generation_occupancy: [0, 0],
            amount: 100,
        }],
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    // Fine navigation is rebuilt from terrain plus authored physical
    // footprints, not from the coarse placement occupancy.
    world
        .navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: GridPos { x: 8, z: 8 },
                max: GridPos { x: 8, z: 8 },
            },
            true,
        )
        .unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    let mut add_building = |runtime_id: &str, building_id: &str, position: GridPos| {
        let definition = &content.buildings[&StableId::new(building_id).unwrap()];
        let id = StableId::new(runtime_id).unwrap();
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id,
                archetype: definition.archetype.clone(),
                position,
                rotation_quarter_turns: 0,
                level: 1,
                health: 100,
                complete: true,
            },
        );
    };
    add_building(
        "building:runtime_house",
        "building:house",
        GridPos { x: 2, z: 2 },
    );
    add_building(
        "building:runtime_storage",
        "building:orestorage",
        GridPos { x: 2, z: 10 },
    );
    add_building(
        "building:runtime_tower",
        "building:tower",
        GridPos { x: 10, z: 2 },
    );
    add_building(
        "building:runtime_wall_a",
        "building:wall",
        GridPos { x: 15, z: 5 },
    );
    add_building(
        "building:runtime_wall_b",
        "building:wall",
        GridPos { x: 16, z: 5 },
    );
    add_building(
        "building:runtime_gate",
        "building:gate",
        GridPos { x: 17, z: 5 },
    );

    let navigation = build_fine_navigation(&config, &content, &simulation, &world).unwrap();
    assert_eq!((navigation.width(), navigation.height()), (72, 72));

    for (runtime_id, building_id) in [
        ("building:runtime_house", "building:house"),
        ("building:runtime_storage", "building:orestorage"),
        ("building:runtime_tower", "building:tower"),
    ] {
        let runtime_id = StableId::new(runtime_id).unwrap();
        let building_id = StableId::new(building_id).unwrap();
        let state = &simulation.buildings[&runtime_id];
        let definition = &content.buildings[&building_id];
        let physical =
            building_fine_navigation_cells(&content, &simulation, state, &building_id, definition);
        assert!(
            physical.iter().all(|cell| !navigation.is_walkable(*cell)),
            "{building_id} physical footprint must be blocked"
        );
        let placement =
            building_fine_placement_cells(&content, &simulation, state, &building_id, definition);
        assert!(
            placement
                .iter()
                .any(|cell| { !physical.contains(cell) && navigation.is_walkable(*cell) })
        );
    }
    // A resource still blocks only its central fine cell.
    assert!(!navigation.is_walkable(GridPos { x: 25, z: 25 }));
    assert!(navigation.is_walkable(GridPos { x: 24, z: 25 }));

    let gate_cells = completed_player_gate_navigation_cells(&content, &simulation);
    assert_eq!(gate_cells.len(), 3);
    assert!(
        gate_cells
            .iter()
            .all(|position| !navigation.is_walkable(*position))
    );
    let wall_a = linear_navigation_cells(
        &content,
        &simulation,
        &simulation.buildings[&StableId::new("building:runtime_wall_a").unwrap()],
        &StableId::new("building:wall").unwrap(),
    );
    let wall_b = linear_navigation_cells(
        &content,
        &simulation,
        &simulation.buildings[&StableId::new("building:runtime_wall_b").unwrap()],
        &StableId::new("building:wall").unwrap(),
    );
    assert!(wall_a.contains(&GridPos { x: 47, z: 16 }));
    assert!(wall_b.contains(&GridPos { x: 48, z: 16 }));
}

#[test]
fn diagnostic_views_use_live_navigation_and_authored_placement_footprints() {
    let mut config = GameConfig::default();
    config.world.width = 10;
    config.world.height = 10;
    let content = embedded_content();
    let resource_position = GridPos { x: 1, z: 1 };
    let world = GeneratedWorld {
        seed: 17,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(10, 10, vec![false; 100], vec![100; 100])
            .unwrap(),
        resources: vec![stream_town_domain::GeneratedResource {
            id: StableId::new("resource:diagnostic-tree").unwrap(),
            kind: StableId::new("resource:wood").unwrap(),
            target_kind: StableId::new("target:tree").unwrap(),
            position: resource_position,
            offset_milli_cells: [0, 0],
            generation_occupancy: [0, 0],
            amount: 100,
        }],
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    let house_id = StableId::new("building:house").unwrap();
    let house = &content.buildings[&house_id];
    let runtime_id = StableId::new("building:diagnostic-house").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id,
            archetype: house.archetype.clone(),
            position: GridPos { x: 2, z: 3 },
            rotation_quarter_turns: 1,
            level: 1,
            health: 100,
            complete: true,
        },
    );

    let footprint_cells = floorplan_diagnostic_cells(&content, &simulation, &world);
    let mut expected = fine_cells_for_coarse_cell(resource_position)
        .into_iter()
        .collect::<BTreeSet<_>>();
    let runtime = &simulation.buildings[&StableId::new("building:diagnostic-house").unwrap()];
    expected.extend(building_fine_placement_cells(
        &content,
        &simulation,
        runtime,
        &house_id,
        house,
    ));
    assert_eq!(footprint_cells, expected);

    let navigation = build_fine_navigation(&config, &content, &simulation, &world).unwrap();
    let blocked = (0..navigation.height())
        .flat_map(|z| (0..navigation.width()).map(move |x| GridPos { x, z }))
        .filter(|cell| !navigation.is_walkable(*cell))
        .count();
    let pathfinding_mesh = world_diagnostic_overlay_mesh(
        WorldDiagnosticMode::Pathfinding,
        &config,
        &content,
        &simulation,
        &world,
        &navigation,
    )
    .unwrap();
    let floorplan_mesh = world_diagnostic_overlay_mesh(
        WorldDiagnosticMode::Floorplan,
        &config,
        &content,
        &simulation,
        &world,
        &navigation,
    )
    .unwrap();
    assert_eq!(
        pathfinding_mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .len(),
        (usize::from(navigation.width()) + 1) * (usize::from(navigation.height()) + 1)
    );
    assert_eq!(pathfinding_mesh.indices().unwrap().len(), blocked * 6);
    assert_eq!(
        floorplan_mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .len(),
        footprint_cells.len() * 4
    );

    let mut runtime = WorldDiagnosticRuntime::default();
    runtime.activate(WorldDiagnosticMode::Floorplan);
    assert_eq!(runtime.mode, Some(WorldDiagnosticMode::Floorplan));
    assert!((runtime.remaining_seconds - WORLD_DIAGNOSTIC_VIEW_SECONDS).abs() <= f32::EPSILON);
    runtime.remaining_seconds = 0.25;
    runtime.refresh_if_active();
    assert!((runtime.remaining_seconds - WORLD_DIAGNOSTIC_VIEW_SECONDS).abs() <= f32::EPSILON);
    assert!(command_refreshes_world_diagnostic(&ChatCommand::Build(
        StableId::new("thickpath").unwrap()
    )));
    assert!(command_refreshes_world_diagnostic(&ChatCommand::Follow(
        None
    )));
    assert!(!command_refreshes_world_diagnostic(&ChatCommand::Health));

    let mut app = App::new();
    app.insert_resource(RuntimeConfig(config))
        .insert_resource(RuntimeContent(content))
        .insert_resource(SimulationRuntime(simulation))
        .insert_resource(WorldRuntime {
            generated: world,
        })
        .insert_resource(FineNavigationRuntime {
            grid: Some(navigation),
            applied_signature: 0,
        })
        .insert_resource(runtime)
        .init_resource::<Time>()
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .add_systems(Update, sync_world_diagnostic_view);
    let building_entity = app
        .world_mut()
        .spawn((
            RuntimeBuilding {
                id: StableId::new("building:visibility-test").unwrap(),
            },
            Visibility::Inherited,
        ))
        .id();
    app.update();
    assert_eq!(
        app.world().get::<Visibility>(building_entity),
        Some(&Visibility::Hidden)
    );
    assert!(
        app.world()
            .get::<WorldDiagnosticVisibilityBackup>(building_entity)
            .is_some()
    );
    let mut overlay_query = app.world_mut().query::<&WorldDiagnosticOverlay>();
    assert_eq!(overlay_query.iter(app.world()).count(), 1);

    app.world_mut()
        .resource_mut::<WorldDiagnosticRuntime>()
        .mode = None;
    app.update();
    assert_eq!(
        app.world().get::<Visibility>(building_entity),
        Some(&Visibility::Inherited)
    );
    assert!(
        app.world()
            .get::<WorldDiagnosticVisibilityBackup>(building_entity)
            .is_none()
    );
    let mut overlay_query = app.world_mut().query::<&WorldDiagnosticOverlay>();
    assert_eq!(overlay_query.iter(app.world()).count(), 0);
}

#[test]
fn sustained_camp_encounter_reaches_combat_and_is_deterministic() {
    fn run_encounter() -> (u32, i32, i32, i32, u32, u32, bool, bool) {
        let config = GameConfig::default();
        let content = embedded_content();
        let mut world = generate_world(&config.world);
        let mut simulation = WorldSimulation::new(world.seed);
        ensure_town_hall_state(&content, &config, &mut simulation);
        let town_hall = StableId::new("building:townhall").unwrap();
        let hall_state = &simulation.buildings[&town_hall];
        let hall_definition = building_def_for_archetype(&content, &hall_state.archetype).unwrap();
        world
            .navigation
            .set_blocked(
                building_navigation_region(
                    hall_state.position,
                    hall_definition,
                    hall_state.rotation_quarter_turns,
                    &world,
                )
                .unwrap(),
                true,
            )
            .unwrap();
        assert!(seed_generated_enemy_camps(&config, &content, &mut world, &mut simulation) > 0);

        let camp = simulation.enemy_camps.values().next().unwrap().clone();
        let camp_archetype = &content.archetypes[&camp.archetype];
        let spawner = camp_archetype.enemy_spawner.as_ref().unwrap();
        let enemy_archetype =
            archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();
        let enemy_position = enemy_spawn_position(
            &world,
            &content,
            &simulation,
            None,
            &camp,
            spawner,
            camp_archetype.footprint,
            0,
        );
        let enemy_id = StableId::new("actor:sustained_enemy").unwrap();
        let enemy_health = content.archetypes[&enemy_archetype]
            .health
            .as_ref()
            .unwrap()
            .max_health;
        let expected_kill_reward = content.archetypes[&enemy_archetype]
            .enemy
            .as_ref()
            .unwrap()
            .kill_reward
            .amount;
        assert!(simulation.spawn_enemy(
            enemy_id.clone(),
            enemy_archetype,
            enemy_position,
            i32::try_from(enemy_health).unwrap(),
        ));

        let initial_enemy_goal = next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &enemy_id,
            enemy_position,
        );
        assert_eq!(
            initial_enemy_goal.0,
            AgentGoal::AttackBuilding(town_hall.clone())
        );
        let initial_enemy_path = try_agent_path(
            &world.navigation,
            &content,
            &simulation,
            &ActorKind::Enemy,
            enemy_position,
            initial_enemy_goal.1,
        )
        .expect("generated camp has a route to the active Town Hall");
        assert!(initial_enemy_path.len() > 1);
        // Put the defender on the verified assault route after proving the
        // no-local-target Town Hall goal. This preserves the complete
        // target-handoff/combat path while keeping the regression fast.
        let defender_position =
            initial_enemy_path[initial_enemy_path.len().saturating_sub(1).min(8)];
        let defender_id = StableId::new("npc:sustained_defender").unwrap();
        assert!(simulation.join_player(defender_id.clone(), defender_position));
        simulation
            .assign_role(&defender_id, StableId::new("role:defender").unwrap())
            .unwrap();
        let defender_damage =
            effective_role_stats(&content, &simulation, &simulation.actors[&defender_id])
                .unwrap()
                .action_amount;
        let encounter_health = i32::try_from(defender_damage.saturating_mul(3)).unwrap();
        let enemy = simulation.actors.get_mut(&enemy_id).unwrap();
        enemy.health = encounter_health;
        enemy.max_health = encounter_health;

        let mut cooldowns = BTreeMap::<StableId, f32>::new();
        let mut movement_budget = BTreeMap::<StableId, f32>::new();
        let mut combat_started = false;
        let mut defender_acquired_enemy = false;
        let mut elapsed_ticks = 0_u32;
        let delta_seconds = 0.25_f32;
        for tick in 1..=600_u32 {
            elapsed_ticks = tick;
            let actor_ids = simulation.actors.keys().cloned().collect::<Vec<_>>();
            for actor_id in actor_ids {
                if !simulation
                    .actors
                    .get(&actor_id)
                    .is_some_and(|actor| actor.alive)
                {
                    continue;
                }
                cooldowns
                    .entry(actor_id.clone())
                    .and_modify(|remaining| {
                        *remaining = (*remaining - delta_seconds).max(0.0);
                    })
                    .or_insert(0.0);
                let speed = actor_movement_speed(&config, &content, &simulation, &actor_id);
                movement_budget
                    .entry(actor_id.clone())
                    .and_modify(|budget| *budget += speed * delta_seconds)
                    .or_insert(speed * delta_seconds);

                loop {
                    let current = simulation.actors[&actor_id].position;
                    let (goal, target) =
                        next_agent_goal(&simulation, &world, &config, &content, &actor_id, current);
                    if actor_id == defender_id && goal == AgentGoal::Attack(enemy_id.clone()) {
                        defender_acquired_enemy = true;
                    }
                    if target == current {
                        if cooldowns[&actor_id] <= f32::EPSILON {
                            let presentation = complete_agent_goal(
                                &mut simulation,
                                &mut world,
                                &config,
                                &content,
                                &actor_id,
                                &goal,
                                current,
                            );
                            if matches!(goal, AgentGoal::Attack(_) | AgentGoal::AttackBuilding(_))
                                && presentation.is_some()
                            {
                                combat_started = true;
                            }
                            if let Some(ActionPresentation::Projectile(projectile)) = presentation {
                                let _ = apply_combat_damage(
                                    &config,
                                    &mut simulation,
                                    &content,
                                    match projectile.source {
                                        ProjectileSource::Actor(ref source) => Some(source),
                                        ProjectileSource::Building(_) => None,
                                    },
                                    &projectile.target,
                                    projectile.damage,
                                );
                            }
                            cooldowns.insert(
                                actor_id.clone(),
                                action_cooldown(&content, &simulation, &actor_id, &goal),
                            );
                        }
                        break;
                    }
                    if movement_budget[&actor_id] < 1.0 {
                        break;
                    }
                    let kind = if actor_id == enemy_id {
                        ActorKind::Enemy
                    } else {
                        ActorKind::Player
                    };
                    let Some(path) = try_agent_path(
                        &world.navigation,
                        &content,
                        &simulation,
                        &kind,
                        current,
                        target,
                    ) else {
                        break;
                    };
                    let Some(next) = path.get(1).copied() else {
                        break;
                    };
                    simulation.actors.get_mut(&actor_id).unwrap().position = next;
                    *movement_budget.get_mut(&actor_id).unwrap() -= 1.0;
                }
            }
            if !simulation.actors[&enemy_id].alive
                || !simulation.actors[&defender_id].alive
                || !simulation.buildings.contains_key(&town_hall)
            {
                break;
            }
        }

        (
            elapsed_ticks,
            simulation.actors[&enemy_id].health,
            simulation.actors[&defender_id].health,
            simulation
                .buildings
                .get(&town_hall)
                .map_or(0, |building| building.health),
            simulation
                .town_resources
                .get(&StableId::new("resource:gold").unwrap())
                .copied()
                .unwrap_or_default(),
            expected_kill_reward,
            combat_started,
            defender_acquired_enemy,
        )
    }

    let first = run_encounter();
    let second = run_encounter();
    assert_eq!(second, first);
    assert!(first.6, "the routed encounter never reached an attack");
    assert!(first.7, "the defender never acquired the approaching enemy");
    assert_eq!(first.1, 0, "the defender did not defeat the enemy");
    assert_eq!(
        first.4, first.5,
        "the authored player kill reward did not fire once"
    );
}

#[test]
fn every_camp_enemy_has_a_renderable_animated_model_contract() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    for layer in &content.enemy_camp_generation {
        let spawner = content.archetypes[&layer.camp_archetype]
            .enemy_spawner
            .as_ref()
            .unwrap();
        for weighted in &spawner.weighted_enemies {
            let archetype = &content.archetypes[&weighted.enemy_archetype];
            let scene = runtime_archetype_scene(archetype).expect("enemy model scene");
            assert!(
                Path::new(&scene.asset_path)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
            );
            assert!(archetype.enemy_models.is_some());
            assert!(
                converted_animation_spec(archetype, &presentation).is_some()
                    || native_animation_request(archetype, &scene, &presentation).is_some(),
                "{} lacks a usable animation contract",
                archetype.source_path
            );
        }
    }
}

#[test]
fn authored_enemies_drive_damage_range_cadence_and_weighted_spawning() {
    let config = GameConfig::default();
    let mut content = embedded_content();
    let mut world = generate_world(&config.world);
    let blargul_archetype =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Blargul.prefab").unwrap();
    let blargul = &content.archetypes[&blargul_archetype].enemy;
    let blargul = blargul.as_ref().unwrap();
    assert_eq!(blargul.action_amount, 5);
    assert_eq!(blargul.action_milliseconds, 3_000);
    assert_eq!(blargul.action_range_milli_cells, 5_000);

    let enemy_id = StableId::new("actor:enemy_authored_test").unwrap();
    let player_id = StableId::new("actor:player_authored_test").unwrap();
    let enemy_position = GridPos { x: 30, z: 30 };
    // Unity's shipped attack state compares squared distance against a
    // linear range term plus the target bounds, giving Blargul a little
    // over two logical cells of effective reach.
    let player_position = GridPos { x: 32, z: 30 };
    let mut simulation = WorldSimulation::new(world.seed);
    let gold = StableId::new("resource:gold").unwrap();
    simulation.town_resources.insert(gold.clone(), 0);
    assert!(simulation.spawn_enemy(enemy_id.clone(), blargul_archetype, enemy_position, 5,));
    assert!(simulation.join_player(player_id.clone(), player_position));
    let defender = StableId::new("role:defender").unwrap();
    simulation
        .assign_role(&player_id, defender.clone())
        .unwrap();
    content
        .roles
        .get_mut(&defender)
        .unwrap()
        .base_damage_reduction_percent = 90;
    assert_eq!(
        effective_role_stats(&content, &simulation, &simulation.actors[&player_id])
            .unwrap()
            .damage_reduction_percent,
        90,
        "the converted role stat remains inspectable"
    );
    let (goal, target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &enemy_id,
        enemy_position,
    );
    assert_eq!(goal, AgentGoal::Attack(player_id.clone()));
    assert_eq!(target, enemy_position);
    let presentation = complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &enemy_id,
        &goal,
        enemy_position,
    );
    assert!(matches!(
        presentation,
        Some(ActionPresentation::Impact {
            target,
            visual: CombatVisualKind::Physical,
        }) if target == player_position
    ));
    assert_eq!(
        simulation.actors[&player_id].health, 95,
        "Unity computes defense but its reachable attack helpers pass raw authored damage"
    );
    assert!((action_cooldown(&content, &simulation, &enemy_id, &goal) - 3.0).abs() <= f32::EPSILON);
    assert!(apply_combat_damage(&config, &mut simulation, &content, None, &enemy_id, 5).unwrap());
    assert_eq!(
        simulation.town_resources[&gold], 0,
        "Unity tower/environment damage does not count as a player kill"
    );
    simulation.respawn_actor(&enemy_id, enemy_position).unwrap();
    assert!(
        apply_combat_damage(
            &config,
            &mut simulation,
            &content,
            Some(&player_id),
            &enemy_id,
            5,
        )
        .unwrap()
    );
    assert_eq!(simulation.town_resources[&gold], 50);

    let camp = content
        .archetypes
        .values()
        .find_map(|archetype| archetype.enemy_spawner.as_ref())
        .unwrap();
    let samples: BTreeSet<_> = (0..1_000)
        .map(|serial| weighted_enemy_archetype(camp, 42, serial))
        .collect();
    assert_eq!(samples.len(), 3);
}

#[test]
fn enemy_sensor_range_and_retaliation_follow_each_authored_prefab() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let goblin_archetype =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();
    let goblin = content.archetypes[&goblin_archetype]
        .enemy
        .as_ref()
        .unwrap();
    assert_eq!(goblin.target_search_range_milli_cells, 4_000);
    assert!(goblin.attack_attacker);

    let enemy_id = StableId::new("actor:sensor_goblin").unwrap();
    let attacker_id = StableId::new("npc:sensor_attacker").unwrap();
    let enemy_position = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .find(|position| world.navigation.is_walkable(*position))
        .expect("generated world has a walkable enemy position");
    let attacker_position = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .find(|position| {
            world.navigation.is_walkable(*position)
                && grid_distance_squared(*position, enemy_position) > 16
                && grid_distance_squared(*position, enemy_position) <= 36
                && enemy_navigation_can_reach(&world, enemy_position, *position)
        })
        .expect("generated world has a connected retaliation target outside sensor range");
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.spawn_enemy(enemy_id.clone(), goblin_archetype, enemy_position, 10,));
    assert!(simulation.join_player(attacker_id.clone(), attacker_position));

    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &enemy_id,
            enemy_position,
        )
        .0,
        AgentGoal::Wander,
    );
    assert!(
        !apply_combat_damage(
            &config,
            &mut simulation,
            &content,
            Some(&attacker_id),
            &enemy_id,
            1,
        )
        .unwrap()
    );
    assert_eq!(
        simulation.actors[&enemy_id].preferred_target,
        Some(attacker_id.clone()),
    );
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &enemy_id,
            enemy_position,
        ),
        (AgentGoal::Attack(attacker_id), attacker_position),
    );

    let serialized = ron::to_string(&simulation).unwrap();
    let reloaded: WorldSimulation = ron::from_str(&serialized).unwrap();
    assert_eq!(reloaded, simulation);
}

#[test]
fn battering_ram_targets_and_damages_buildings_from_authored_mask() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let battering_ram = archetype_id_by_source(
        &content,
        ArchetypeKind::Enemy,
        "Enemy_Goblin_BatteringRam.prefab",
    )
    .unwrap();
    let enemy_health = content.archetypes[&battering_ram]
        .health
        .as_ref()
        .and_then(|health| i32::try_from(health.max_health).ok())
        .expect("battering ram has valid authored health");
    let enemy_id = StableId::new("actor:battering_ram_test").unwrap();
    let building_id = StableId::new("building:ram_target").unwrap();
    let house = &content.buildings[&StableId::new("building:house").unwrap()];
    let building_position = find_building_site(
        &world,
        GridPos {
            x: config.world.width / 2,
            z: config.world.height / 2,
        },
        house.footprint,
    )
    .expect("generated world has a house site");
    let enemy_position = building_approach(
        &world,
        building_position,
        house.footprint,
        building_position,
    )
    .expect("house site has an enemy approach");
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.spawn_enemy(
        enemy_id.clone(),
        battering_ram.clone(),
        enemy_position,
        enemy_health,
    ));
    simulation.buildings.insert(
        building_id.clone(),
        BuildingState {
            id: building_id.clone(),
            archetype: house.archetype.clone(),
            position: building_position,
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );
    let (goal, target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &enemy_id,
        enemy_position,
    );
    assert_eq!(goal, AgentGoal::AttackBuilding(building_id.clone()));
    assert_eq!(target, enemy_position);
    let presentation = complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &enemy_id,
        &goal,
        enemy_position,
    );
    assert!(matches!(
        presentation,
        Some(ActionPresentation::BuildingWork {
            target,
            sparks: true,
        }) if target == building_visual_grid(&content, &simulation.buildings[&building_id])
    ));
    assert_eq!(
        simulation.buildings[&building_id].health,
        BUILDING_MAX_HEALTH - 20
    );
    let footprint = content.buildings[&StableId::new("building:house").unwrap()].footprint;
    let region = building_region(building_position, footprint, &world).unwrap();
    world.navigation.set_blocked(region, true).unwrap();
    simulation.buildings.get_mut(&building_id).unwrap().health = 20;
    let presentation = complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &enemy_id,
        &goal,
        enemy_position,
    );
    assert!(matches!(
        presentation,
        Some(ActionPresentation::BuildingDestroyed {
            building,
            target: _,
        }) if building == building_id
    ));
    assert!(!simulation.buildings.contains_key(&building_id));
    assert!(world.navigation.is_walkable(region.min));
}

#[test]
fn priest_prioritizes_and_heals_the_nearest_injured_player() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let priest_position = GridPos { x: 32, z: 32 };
    let patient_position = GridPos { x: 33, z: 32 };
    let priest = StableId::new("npc:priest_test").unwrap();
    let patient = StableId::new("npc:patient_test").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(priest.clone(), priest_position));
    assert!(simulation.join_player(patient.clone(), patient_position));
    simulation
        .assign_role(&priest, StableId::new("role:priest").unwrap())
        .unwrap();
    simulation.damage_actor(&patient, 20).unwrap();

    let (goal, target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &priest,
        priest_position,
    );
    assert_eq!(goal, AgentGoal::Heal(patient.clone()));
    assert_eq!(target, priest_position);
    let presentation = complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &priest,
        &goal,
        priest_position,
    );
    assert!(matches!(
        presentation,
        Some(ActionPresentation::Healing { source, target })
            if source == priest && target == patient
    ));
    assert_eq!(simulation.actors[&patient].health, 82);
}

#[test]
fn engaged_priest_keeps_healing_through_the_twenty_five_percent_range_band() {
    let content = embedded_content();
    let priest = StableId::new("npc:priest_hysteresis").unwrap();
    let patient = StableId::new("npc:patient_hysteresis").unwrap();
    let origin = GridPos { x: 20, z: 20 };
    let mut simulation = WorldSimulation::new(1);
    assert!(simulation.join_player(priest.clone(), origin));
    assert!(simulation.join_player(patient.clone(), origin));
    simulation
        .assign_role(&priest, StableId::new("role:priest").unwrap())
        .unwrap();
    simulation.damage_actor(&patient, 40).unwrap();
    let edge = (1..20)
        .map(|offset| GridPos {
            x: origin.x + offset,
            z: origin.z,
        })
        .find(|position| {
            simulation.actors.get_mut(&patient).unwrap().position = *position;
            !within_actor_heal_range(
                &content,
                &simulation,
                &simulation.actors[&priest],
                &simulation.actors[&patient],
                origin,
            ) && within_actor_heal_hysteresis_range(
                &content,
                &simulation,
                &simulation.actors[&priest],
                &simulation.actors[&patient],
                origin,
            )
        })
        .expect("the expanded range has at least one whole-cell sample");
    simulation.actors.get_mut(&patient).unwrap().position = edge;
    assert_eq!(
        retained_healing_goal(
            &AgentGoal::Heal(patient.clone()),
            &content,
            &simulation,
            &priest,
            origin,
        ),
        Some((AgentGoal::Heal(patient), origin))
    );
}

#[test]
fn citizen_retreat_uses_entry_and_full_health_exit_hysteresis() {
    let actor = StableId::new("npc:retreat_test").unwrap();
    let mut simulation = WorldSimulation::new(1);
    assert!(simulation.join_player(actor.clone(), GridPos { x: 1, z: 1 }));
    let state = simulation.actors.get_mut(&actor).unwrap();
    state.max_health = 100;
    state.health = 24;
    assert!(citizen_should_retreat(state, false));
    state.health = 99;
    assert!(citizen_should_retreat(state, true));
    state.health = 100;
    assert!(!citizen_should_retreat(state, true));
    state.health = 25;
    assert!(!citizen_should_retreat(state, false));
}

#[test]
fn saved_recruits_are_normalized_to_level_one_without_capping_players() {
    let content = embedded_content();
    let recruit = StableId::new("npc:recruit_level_cap").unwrap();
    let initial_viewer = StableId::new("actor:viewer_0005").unwrap();
    let guard = StableId::new("npc:guardhouse_building:runtime_test").unwrap();
    let player = StableId::new("twitch:player_level_cap").unwrap();
    let role = StableId::new("role:defender").unwrap();
    let mut simulation = WorldSimulation::new(1);
    assert!(simulation.join_player(recruit.clone(), GridPos { x: 1, z: 1 }));
    assert!(simulation.join_player(initial_viewer.clone(), GridPos { x: 1, z: 2 }));
    assert!(simulation.join_player(guard.clone(), GridPos { x: 1, z: 3 }));
    assert!(simulation.join_player(player.clone(), GridPos { x: 2, z: 1 }));
    for actor in [&recruit, &initial_viewer, &guard, &player] {
        simulation
            .actors
            .get_mut(actor)
            .unwrap()
            .role_progression
            .insert(
                role.clone(),
                stream_town_domain::RoleProgress {
                    level: 42,
                    experience: 12_345,
                },
            );
    }

    assert_eq!(
        normalize_recruit_role_progression(&content, &mut simulation),
        3
    );
    assert_eq!(simulation.actors[&recruit].role_progression[&role].level, 1);
    assert_eq!(
        simulation.actors[&recruit].role_progression[&role].experience,
        0
    );
    for actor in [&initial_viewer, &guard] {
        assert_eq!(simulation.actors[actor].role_progression[&role].level, 1);
        assert_eq!(
            simulation.actors[actor].role_progression[&role].experience,
            0
        );
    }
    assert_eq!(simulation.actors[&player].role_progression[&role].level, 42);
    assert_eq!(
        simulation.actors[&player].role_progression[&role].experience,
        12_345
    );
}

#[test]
fn healing_effect_curves_preserve_authored_lifetimes_and_channel_keys() {
    let presentation = embedded_presentation();
    let burst_duration = healing_effect_duration(&presentation, HealingEffectKind::Burst);
    let channel_duration = healing_effect_duration(&presentation, HealingEffectKind::Channel);
    let revive_duration = healing_effect_duration(&presentation, HealingEffectKind::Revive);
    assert!((burst_duration - 1.2).abs() < f32::EPSILON);
    assert!((channel_duration - 5.0).abs() < f32::EPSILON);
    assert!((revive_duration - 1.2).abs() < f32::EPSILON);
    let burst = healing_burst_effect(&presentation).1;
    let channel = healing_channel_effect(&presentation).1;
    assert_eq!(burst.plus_burst_count, 100);
    assert_eq!(burst.plus_capacity, 8);
    assert_eq!(burst.disc_burst_count, 1);
    assert_eq!(channel.particle_capacity, 32);
    assert!((channel.emission_rate_per_second - 16.0).abs() < f32::EPSILON);

    let channel_start = healing_effect_sample(
        &presentation,
        HealingEffectKind::Channel,
        0.0,
        channel_duration,
    );
    let channel_first_key = healing_effect_sample(
        &presentation,
        HealingEffectKind::Channel,
        1.5,
        channel_duration,
    );
    let channel_peak = healing_effect_sample(
        &presentation,
        HealingEffectKind::Channel,
        3.0,
        channel_duration,
    );
    let channel_end = healing_effect_sample(
        &presentation,
        HealingEffectKind::Channel,
        5.0,
        channel_duration,
    );
    assert!(channel_start.ring_scale.abs() < f32::EPSILON);
    assert!((channel_first_key.ring_scale - 0.289_276_9).abs() < f32::EPSILON);
    assert!((channel_peak.ring_scale - 1.0).abs() < f32::EPSILON);
    assert!(channel_end.ring_scale.abs() < f32::EPSILON);

    let burst_midpoint =
        healing_effect_sample(&presentation, HealingEffectKind::Burst, 0.6, burst_duration);
    let revive_midpoint = healing_effect_sample(
        &presentation,
        HealingEffectKind::Revive,
        0.6,
        revive_duration,
    );
    assert!(burst_midpoint.ring_scale > 0.0);
    assert!(burst_midpoint.mote_scale > 0.0);
    assert!(revive_midpoint.ring_scale > burst_midpoint.ring_scale);
    assert!(revive_midpoint.rise > burst_midpoint.rise);
}

#[test]
fn healing_ring_mesh_is_a_closed_top_facing_annulus() {
    let mesh = healing_ring_mesh(48);
    assert_eq!(mesh.count_vertices(), 96);
    assert_eq!(mesh.indices().unwrap().len(), 288);
    assert!(mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_some());
    assert!(mesh.attribute(Mesh::ATTRIBUTE_UV_0).is_some());
}

#[test]
fn ranged_roles_emit_projectiles_instead_of_instant_damage() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let ranger_position = GridPos { x: 32, z: 32 };
    let enemy_position = GridPos { x: 34, z: 32 };
    let ranger = StableId::new("npc:ranger_test").unwrap();
    let enemy = StableId::new("actor:enemy_ranged_test").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(ranger.clone(), ranger_position));
    assert!(simulation.join_player(enemy.clone(), enemy_position));
    simulation
        .assign_role(&ranger, StableId::new("role:ranger").unwrap())
        .unwrap();
    simulation
        .assign_role(&enemy, StableId::new("role:enemy").unwrap())
        .unwrap();
    let (goal, _) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &ranger,
        ranger_position,
    );
    let presentation = complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &ranger,
        &goal,
        ranger_position,
    )
    .unwrap();
    let ActionPresentation::Projectile(projectile) = presentation else {
        panic!("ranged role must emit a projectile");
    };
    assert_eq!(projectile.target, enemy);
    assert_eq!(projectile.visual, CombatVisualKind::Arrow);
    assert_eq!(simulation.actors[&projectile.target].health, 100);
}

#[test]
fn combat_visuals_follow_authored_role_identity_and_vfx_constants() {
    assert_eq!(
        actor_combat_visual(&StableId::new("role:ranger").unwrap()),
        CombatVisualKind::Arrow
    );
    assert_eq!(
        actor_combat_visual(&StableId::new("role:wizard").unwrap()),
        CombatVisualKind::Fireball
    );
    assert_eq!(
        actor_combat_visual(&StableId::new("role:necromancer").unwrap()),
        CombatVisualKind::Necrotic
    );
    assert_eq!(
        actor_combat_visual(&StableId::new("role:defender").unwrap()),
        CombatVisualKind::Physical
    );
    assert!((CHARACTER_HIT_SECONDS - 0.25).abs() < f32::EPSILON);
    assert!((TOWER_TRAIL_SECONDS - 2.0).abs() < f32::EPSILON);
    assert!((TOWER_TRAIL_WIDTH - 0.1).abs() < f32::EPSILON);
    assert!((FIREBALL_SIZE - 0.4).abs() < f32::EPSILON);
    assert!((FIREBALL_TRAIL_SIZE - 0.3).abs() < f32::EPSILON);
}

#[test]
fn builders_can_work_from_every_planned_corner_approach() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let builder_id = StableId::new("npc:corner_builder").unwrap();
    let building_position = (1..world.navigation.height().saturating_sub(1))
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
    let builder_position = GridPos {
        x: building_position.x - 1,
        z: building_position.z - 1,
    };
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(builder_id.clone(), builder_position));
    simulation
        .assign_role(&builder_id, StableId::new("role:builder").unwrap())
        .unwrap();

    for (index, building_name) in ["path", "wall"].into_iter().enumerate() {
        let definition =
            &content.buildings[&StableId::new(format!("building:{building_name}")).unwrap()];
        let runtime_id = StableId::new(format!("building:corner_work_{index}")).unwrap();
        simulation.buildings.insert(
            runtime_id.clone(),
            BuildingState {
                id: runtime_id.clone(),
                archetype: definition.archetype.clone(),
                position: building_position,
                rotation_quarter_turns: 0,
                level: 1,
                health: 3,
                complete: false,
            },
        );
        assert!(
            building_approaches(
                &world,
                building_position,
                definition.footprint,
                builder_position,
            )
            .contains(&builder_position)
        );
        assert!(within_building_work_range(
            &content,
            &simulation,
            &simulation.actors[&builder_id],
            &simulation.buildings[&runtime_id],
            builder_position,
        ));
        assert!(
            complete_agent_goal(
                &mut simulation,
                &mut world,
                &config,
                &content,
                &builder_id,
                &AgentGoal::Construct(runtime_id.clone()),
                builder_position,
            )
            .is_some()
        );
        assert_eq!(simulation.buildings[&runtime_id].health, 4);
        simulation.buildings.remove(&runtime_id);
    }
}

#[test]
fn builder_selection_uses_an_alternate_approach_after_path_rejection() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let building_position = (1..world.navigation.height().saturating_sub(1))
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
    let builder_position = GridPos {
        x: building_position.x - 1,
        z: building_position.z - 1,
    };
    let builder_id = StableId::new("npc:alternate_approach_builder").unwrap();
    let building_id = StableId::new("building:alternate_approach").unwrap();
    let definition = &content.buildings[&StableId::new("building:path").unwrap()];
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(builder_id.clone(), builder_position));
    simulation
        .assign_role(&builder_id, StableId::new("role:builder").unwrap())
        .unwrap();
    simulation.buildings.insert(
        building_id.clone(),
        BuildingState {
            id: building_id.clone(),
            archetype: definition.archetype.clone(),
            position: building_position,
            rotation_quarter_turns: 0,
            level: 1,
            health: 3,
            complete: false,
        },
    );

    let choose = |approach_reservations: &BTreeMap<GridPos, StableId>| {
        next_agent_goal_with_station_runtime(
            &simulation,
            &world,
            None,
            &config,
            &content,
            &StationTargetRuntime::default(),
            &builder_id,
            builder_position,
            placement_to_navigation_centre(builder_position),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            approach_reservations,
        )
    };
    let (first_goal, first_approach) = choose(&BTreeMap::new());
    assert_eq!(first_goal, AgentGoal::Construct(building_id.clone()));

    let rejected = BTreeMap::from([(
        first_approach,
        StableId::new("system:unreachable-planning-target").unwrap(),
    )]);
    let (retry_goal, retry_approach) = choose(&rejected);
    assert_eq!(retry_goal, AgentGoal::Construct(building_id));
    assert_ne!(retry_approach, first_approach);
}

#[test]
fn builder_completes_and_upgrades_authored_construction() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let building_id = StableId::new("building:house").unwrap();
    let definition = &content.buildings[&building_id];
    let position = find_building_site(
        &world,
        GridPos {
            x: config.world.width / 2,
            z: config.world.height / 2,
        },
        definition.footprint,
    )
    .unwrap();
    let runtime_id = StableId::new("building:test_house").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    for resource in definition.cost.keys() {
        simulation
            .town_resources
            .insert(resource.clone(), 1_000_000);
    }
    simulation
        .construct(
            runtime_id.clone(),
            definition.archetype.clone(),
            position,
            building_base_max_health(&content, definition),
            &definition.cost,
        )
        .unwrap();
    assert!(!simulation.buildings[&runtime_id].complete);
    assert_eq!(
        building_construction_stage(
            simulation.buildings[&runtime_id].health,
            building_max_health(&content, &simulation.buildings[&runtime_id]),
            simulation.buildings[&runtime_id].complete,
        ),
        0
    );
    let region = building_region(position, definition.footprint, &world).unwrap();
    world.navigation.set_blocked(region, true).unwrap();
    let builder_position =
        building_approach(&world, position, definition.footprint, position).unwrap();
    let builder = StableId::new("npc:builder_test").unwrap();
    assert!(simulation.join_player(builder.clone(), builder_position));
    simulation
        .assign_role(&builder, StableId::new("role:builder").unwrap())
        .unwrap();
    let (goal, target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &builder,
        builder_position,
    );
    assert_eq!(goal, AgentGoal::Construct(runtime_id.clone()));
    assert_eq!(target, builder_position);
    for _ in 0..450 {
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &builder,
            &goal,
            builder_position,
        );
    }
    assert!(simulation.buildings[&runtime_id].complete);
    assert_eq!(
        simulation.buildings[&runtime_id].health,
        building_max_health(&content, &simulation.buildings[&runtime_id])
    );
    assert_eq!(
        building_construction_stage(
            simulation.buildings[&runtime_id].health,
            building_max_health(&content, &simulation.buildings[&runtime_id]),
            simulation.buildings[&runtime_id].complete,
        ),
        3
    );

    simulation.damage_building(&runtime_id, 25).unwrap();
    let damaged_health = simulation.buildings[&runtime_id].health;
    let (repair_goal, repair_target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &builder,
        builder_position,
    );
    assert_eq!(repair_goal, AgentGoal::Construct(runtime_id.clone()));
    assert_eq!(repair_target, builder_position);
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &builder,
            &repair_goal,
            builder_position,
        )
        .is_some()
    );
    assert!(simulation.buildings[&runtime_id].health > damaged_health);
    while simulation.buildings[&runtime_id].health
        < building_max_health(&content, &simulation.buildings[&runtime_id])
    {
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &builder,
            &repair_goal,
            builder_position,
        );
    }

    let (technology, authored_cap) = content
        .technology
        .nodes
        .iter()
        .find_map(|(technology, node)| {
            node.building_level_caps
                .get(&building_id)
                .copied()
                .map(|cap| (technology.clone(), cap))
        })
        .expect("Unity technology graph contains a House level-cap effect");
    simulation.unlocked_technology.insert(technology);
    let max_level = maximum_building_level(&content, &simulation, &building_id);
    assert_eq!(max_level, authored_cap);
    let upgrade_cost = building_upgrade_cost(&content, &simulation, &building_id, definition, 1, 2);
    simulation.damage_building(&runtime_id, 7).unwrap();
    assert_eq!(
        simulation
            .upgrade_building(
                &runtime_id,
                max_level,
                2,
                building_max_health_at_level(&content, &definition.archetype, 2),
                &upgrade_cost,
            )
            .unwrap(),
        2
    );
    assert_eq!(
        simulation.buildings[&runtime_id].health,
        (building_max_health(&content, &simulation.buildings[&runtime_id]) + 9) / 10
    );
    assert!(!simulation.buildings[&runtime_id].complete);
    while !simulation.buildings[&runtime_id].complete {
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &builder,
            &repair_goal,
            builder_position,
        );
    }
}

#[test]
fn completed_technology_goal_unlocks_and_pays_for_a_building_upgrade() {
    let content = embedded_content();
    let mut simulation = WorldSimulation::new(42);
    simulation.unlocked_technology.extend(
        content
            .technology
            .nodes
            .iter()
            .filter(|(_, node)| node.initially_unlocked)
            .map(|(id, _)| id.clone()),
    );
    let (technology, node, building_id) = content
        .technology
        .nodes
        .iter()
        .find_map(|(technology, node)| {
            if node.initially_unlocked {
                return None;
            }
            node.building_level_caps
                .iter()
                .find(|(building, cap)| {
                    **cap >= 2
                        && content.buildings[*building].can_level
                        && !node.objectives.is_empty()
                })
                .map(|(building, _)| (technology.clone(), node.clone(), building.clone()))
        })
        .expect("converted technology contains an objective-backed building level unlock");
    simulation
        .unlocked_technology
        .extend(node.prerequisites.iter().cloned());
    assert!(simulation.start_technology_goal(
        technology.clone(),
        &node.objectives,
        &content.objectives,
        MAX_TOWN_GOALS,
    ));
    assert!(!simulation.unlocked_technology.contains(&technology));

    for objective_id in &node.objectives {
        let objective = &content.objectives[objective_id];
        let event = match objective.kind {
            ObjectiveKind::Build => {
                ObjectiveEvent::BuildingBuilt(objective.building.clone().unwrap())
            }
            ObjectiveKind::BuildAny => ObjectiveEvent::BuildingBuilt(building_id.clone()),
            ObjectiveKind::Collect => ObjectiveEvent::ResourceGathered {
                resource: objective.resource.clone().unwrap(),
                amount: objective.required_amount,
            },
            ObjectiveKind::EarnPerHour => ObjectiveEvent::ResourceGained {
                resource: objective.resource.clone().unwrap(),
                amount: objective.required_amount,
            },
            ObjectiveKind::Kill => ObjectiveEvent::EnemyKilled(objective.enemy.clone().unwrap()),
            ObjectiveKind::KillAny => {
                ObjectiveEvent::EnemyKilled(StableId::new("enemy:technology_test").unwrap())
            }
            ObjectiveKind::Sell => ObjectiveEvent::ResourceSold {
                resource: objective.resource.clone().unwrap(),
                amount: objective.required_amount,
            },
            ObjectiveKind::SellAny => ObjectiveEvent::ResourceSold {
                resource: StableId::new("resource:technology_test").unwrap(),
                amount: objective.required_amount,
            },
            ObjectiveKind::Buy => ObjectiveEvent::ResourceBought {
                resource: objective.resource.clone().unwrap(),
                amount: objective.required_amount,
            },
            ObjectiveKind::BuyAny => ObjectiveEvent::ResourceBought {
                resource: StableId::new("resource:technology_test").unwrap(),
                amount: objective.required_amount,
            },
        };
        let repetitions = match objective.kind {
            ObjectiveKind::Build
            | ObjectiveKind::BuildAny
            | ObjectiveKind::Kill
            | ObjectiveKind::KillAny => objective.required_amount,
            ObjectiveKind::Collect
            | ObjectiveKind::EarnPerHour
            | ObjectiveKind::Sell
            | ObjectiveKind::SellAny
            | ObjectiveKind::Buy
            | ObjectiveKind::BuyAny => 1,
        };
        for _ in 0..repetitions {
            let _ = simulation.record_objective_event(&content.objectives, &event);
        }
    }
    assert!(simulation.unlocked_technology.contains(&technology));
    assert!(simulation.active_goals.is_empty());
    assert!(maximum_building_level(&content, &simulation, &building_id) >= 2);

    let definition = &content.buildings[&building_id];
    let runtime_id = StableId::new("building:technology_upgrade_test").unwrap();
    for resource in definition.cost.keys() {
        simulation
            .town_resources
            .insert(resource.clone(), 1_000_000);
    }
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id.clone(),
            archetype: definition.archetype.clone(),
            position: GridPos { x: 10, z: 10 },
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(&content, definition)).unwrap(),
            complete: true,
        },
    );
    let expected_cost =
        building_upgrade_cost(&content, &simulation, &building_id, definition, 1, 2);
    let before = simulation.town_resources.clone();
    assert_eq!(
        upgrade_building_instance(&content, &mut simulation, &building_id, &runtime_id, 1).unwrap(),
        2
    );
    for (resource, amount) in expected_cost {
        assert_eq!(
            simulation.town_resources[&resource],
            before[&resource] - amount
        );
    }
}

#[test]
fn selling_through_the_runtime_command_path_produces_spendable_gold() {
    let config = GameConfig::default();
    let content = embedded_content();
    let wood = StableId::new("resource:wood").unwrap();
    let ore = StableId::new("resource:ore").unwrap();
    let gold = StableId::new("resource:gold").unwrap();
    let mut simulation = WorldSimulation::new(config.world.seed);
    simulation.town_resources.insert(wood.clone(), 100);
    simulation.town_resources.insert(gold.clone(), 0);

    let ChatCommand::Sell { amount, resource } = "!sell 100 wood".parse().unwrap() else {
        panic!("sell command did not use the trade parser path");
    };
    let resource = prefixed_id(&resource, "resource:").unwrap();
    assert_eq!(
        sell_town_resource(&content, &mut simulation, &resource, amount).unwrap(),
        "sold 100 resource:wood for 13 gold"
    );
    assert_eq!(simulation.town_resources[&wood], 0);
    assert_eq!(simulation.town_resources[&gold], 13);

    let ChatCommand::Buy { amount, resource } = "!buy 24 ore".parse().unwrap() else {
        panic!("buy command did not use the trade parser path");
    };
    let resource = prefixed_id(&resource, "resource:").unwrap();
    assert_eq!(
        buy_town_resource(&config, &content, &mut simulation, resource, amount).unwrap(),
        "bought 24 resource:ore for 10 gold"
    );
    assert_eq!(simulation.town_resources[&ore], 24);
    assert_eq!(simulation.town_resources[&gold], 3);
}

#[test]
fn final_construction_tick_and_builder_repair_preserve_experience_contract() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let building_id = StableId::new("building:house").unwrap();
    let definition = &content.buildings[&building_id];
    let runtime_id = StableId::new("building:final_tick_house").unwrap();
    let position = GridPos {
        x: config.world.width / 2,
        z: config.world.height / 2,
    };
    let max_health = building_base_max_health(&content, definition);
    let mut simulation = WorldSimulation::new(world.seed);
    simulation
        .construct(
            runtime_id.clone(),
            definition.archetype.clone(),
            position,
            max_health,
            &BTreeMap::new(),
        )
        .unwrap();
    let building = simulation.buildings.get_mut(&runtime_id).unwrap();
    building.health = i32::try_from(max_health).unwrap() - 1;
    building.complete = false;

    let builder = StableId::new("npc:final_tick_builder").unwrap();
    assert!(simulation.join_player(builder.clone(), position));
    simulation
        .assign_role(&builder, StableId::new("role:builder").unwrap())
        .unwrap();
    complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &builder,
        &AgentGoal::Construct(runtime_id.clone()),
        position,
    );

    assert!(simulation.buildings[&runtime_id].complete);
    assert_eq!(
        role_progress(&simulation.actors[&builder]).experience,
        0,
        "Unity applies the last build amount, then exits without OnActionSuccess"
    );

    simulation.damage_building(&runtime_id, 1).unwrap();
    let damaged_health = simulation.buildings[&runtime_id].health;
    let experience_before_repair = role_progress(&simulation.actors[&builder]).experience;
    let (goal, _) = next_agent_goal(&simulation, &world, &config, &content, &builder, position);
    assert_eq!(goal, AgentGoal::Construct(runtime_id.clone()));
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &builder,
            &AgentGoal::Construct(runtime_id.clone()),
            position,
        )
        .is_some()
    );
    assert!(simulation.buildings[&runtime_id].health > damaged_health);
    assert_eq!(
        role_progress(&simulation.actors[&builder]).experience,
        experience_before_repair,
        "repairs do not award construction experience"
    );
}

#[test]
fn authored_building_nodes_follow_construction_age_and_storage_fill() {
    let config = GameConfig::default();
    let content = embedded_content();
    let building_id = StableId::new("building:foodstorage").unwrap();
    let definition = &content.buildings[&building_id];
    assert_eq!(definition.model_handlers.len(), 2);
    assert_eq!(definition.storage_models.len(), 2);
    let runtime_id = StableId::new("building:model_test").unwrap();
    let mut simulation = WorldSimulation::new(42);
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id.clone(),
            archetype: definition.archetype.clone(),
            position: GridPos { x: 10, z: 10 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 0,
            complete: false,
        },
    );
    let age_one = &definition.model_handlers[0];
    let visible = |simulation: &WorldSimulation, name: &str, age| {
        building_node_visibility(
            &building_id,
            definition,
            &simulation.buildings[&runtime_id],
            age,
            name,
            &config,
            &content,
            simulation,
        )
    };
    assert_eq!(
        visible(&simulation, &age_one.construction_stages[0], 1),
        Some(true)
    );
    assert_eq!(visible(&simulation, &age_one.full_model, 1), Some(false));
    let max_health = building_max_health(&content, &simulation.buildings[&runtime_id]);
    simulation.buildings.get_mut(&runtime_id).unwrap().health = max_health / 2;
    assert_eq!(
        visible(&simulation, &age_one.construction_stages[1], 1),
        Some(true)
    );
    simulation.buildings.get_mut(&runtime_id).unwrap().health = max_health * 4 / 5;
    assert_eq!(
        visible(&simulation, &age_one.construction_stages[2], 1),
        Some(true)
    );

    let state = simulation.buildings.get_mut(&runtime_id).unwrap();
    state.health = max_health;
    state.complete = true;
    assert_eq!(visible(&simulation, &age_one.full_model, 1), Some(true));
    assert_eq!(
        visible(&simulation, &definition.model_handlers[1].full_model, 1),
        Some(false)
    );

    let storage = &definition.storage_models[0];
    let capacity = resource_storage_capacity(&config, &content, &simulation, &storage.resource);
    assert_eq!(visible(&simulation, &storage.empty_model, 1), Some(true));
    simulation
        .town_resources
        .insert(storage.resource.clone(), capacity.div_ceil(2));
    assert_eq!(
        visible(&simulation, &storage.half_full_model, 1),
        Some(true)
    );
    simulation
        .town_resources
        .insert(storage.resource.clone(), capacity);
    assert_eq!(visible(&simulation, &storage.full_model, 1), Some(true));
}

#[test]
fn windmill_rotation_uses_converted_mesh_axes_and_unity_speed() {
    let content = embedded_content();
    let windmill = &content.buildings[&StableId::new("building:windmill").unwrap()];
    let rotors = &content.archetypes[&windmill.archetype].rotating_nodes;
    assert_eq!(rotors.len(), 2);
    assert_eq!(rotors[0].age, Some(1));
    assert_eq!(rotors[0].node, "Age01_Windmill_Blades");
    assert!(Vec3::from_array(rotors[0].axis).abs_diff_eq(Vec3::Y, f32::EPSILON));
    assert!((rotors[0].degrees_per_second - 35.0).abs() < f32::EPSILON);
    assert_eq!(rotors[1].age, Some(2));
    assert_eq!(rotors[1].node, "Age02_Windmill_Blades");
    assert!(Vec3::from_array(rotors[1].axis).abs_diff_eq(Vec3::Z, f32::EPSILON));
    assert!((rotors[1].degrees_per_second - 35.0).abs() < f32::EPSILON);

    let mut age_one = Transform::default();
    apply_authored_local_rotation(&mut age_one, Vec3::Y, 35.0_f32.to_radians(), 1.0);
    assert!(
        age_one
            .rotation
            .abs_diff_eq(Quat::from_rotation_y(35.0_f32.to_radians()), 1e-6)
    );

    let mut age_two = Transform::default();
    apply_authored_local_rotation(&mut age_two, Vec3::Z, 35.0_f32.to_radians(), 0.5);
    assert!(
        age_two
            .rotation
            .abs_diff_eq(Quat::from_rotation_z(17.5_f32.to_radians()), 1e-6)
    );
}

#[test]
fn loading_icon_rotation_uses_the_authored_prefab_contract() {
    let content = embedded_content();
    let rotation = loading_icon_rotation(&content).unwrap();
    assert_eq!(rotation.hierarchy_path, LOADING_ICON_HIERARCHY_PATH);
    assert_eq!(rotation.age, None);
    assert!(Vec3::from_array(rotation.axis).abs_diff_eq(Vec3::Z, f32::EPSILON));
    assert!((rotation.degrees_per_second - 500.0).abs() < f32::EPSILON);
    assert!(
        content
            .archetypes
            .values()
            .flat_map(|archetype| &archetype.rotating_nodes)
            .count()
            == 3
    );
    let mut transform = UiTransform::IDENTITY;
    apply_loading_icon_rotation(&mut transform, 500.0_f32.to_radians(), 0.1);
    assert!((transform.rotation.as_degrees() + 50.0).abs() < 1e-5);
    let mut loading = WorldLoadingRuntime::default();
    assert!(!advance_loading_runtime(&mut loading, 0.1));
    assert_eq!(loading.phase, WorldLoadingPhase::Presenting);
    loading.phase = WorldLoadingPhase::Loading;
    loading.progress = 0.5;
    assert!(!advance_loading_runtime(&mut loading, 0.1));
    assert!((loading.progress - 0.5).abs() < f32::EPSILON);
    loading.phase = WorldLoadingPhase::Complete;
    loading.progress = 1.0;
    loading.completion_remaining_seconds = 0.5;
    assert!(!advance_loading_runtime(&mut loading, 0.49));
    assert!(advance_loading_runtime(&mut loading, 0.02));
    let presentation = embedded_presentation();
    for source_path in [
        LOADING_SCREEN_TEXTURE_PATH,
        LOADING_OVERLAY_TEXTURE_PATH,
        LOADING_ICON_TEXTURE_PATH,
    ] {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path)
        );
    }
}

#[test]
fn loading_progress_is_recursively_derived_from_real_work() {
    assert!(loading_display_percent(0.0).abs() < f32::EPSILON);
    assert!((loading_display_percent(0.999) - 99.9).abs() < f32::EPSILON);
    assert!((loading_display_percent(1.0) - 100.0).abs() < f32::EPSILON);

    let mut work = WorldLoadingWork::default();
    assert!(work.progress().abs() < f32::EPSILON);
    work.mark_cover_complete();
    assert!((work.progress() - 0.2).abs() < f32::EPSILON);

    work.world_assets = LoadingWork::count(5, 10);
    work.generation_completed
        .extend(WorldGenerationStage::ALL.into_iter().take(2));
    let expected_world_input = (0.5 + 0.4) * 0.5;
    let expected_root = (1.0 + expected_world_input) / 5.0;
    assert!((work.progress() - expected_root).abs() < 1e-6);

    work.world_assets = LoadingWork::count(10, 10);
    work.generation_completed.extend(WorldGenerationStage::ALL);
    work.terrain_entities = LoadingWork::count(2, 2);
    work.resource_entities = LoadingWork::count(300, 300);
    work.foliage_entities = LoadingWork::count(900, 900);
    work.actor_entities = LoadingWork::count(5, 5);
    work.gameplay_setup = LoadingWork::boolean(true);
    work.scene_roots = LoadingWork::count(205, 205);
    work.scene_stable_frames = LoadingWork::count(5, 5);
    work.material_overrides = LoadingWork::count(420, 420);
    work.animation_receivers = LoadingWork::count(5, 5);
    work.lighting_receivers = LoadingWork::count(5, 5);
    work.gpu_images = LoadingWork::count(132, 133);
    work.gpu_meshes = LoadingWork::count(252, 253);
    work.gpu_materials = LoadingWork::count(32, 33);
    work.gpu_pipelines = LoadingWork::boolean(false);
    work.selection_draw = LoadingWork::boolean(true);
    work.gpu_stable_frames = LoadingWork::count(5, 6);
    let incomplete = work.progress();
    assert!(incomplete < 1.0);
    assert!(loading_display_percent(incomplete) < 100.0);

    work.gpu_images = LoadingWork::count(133, 133);
    work.gpu_meshes = LoadingWork::count(253, 253);
    work.gpu_materials = LoadingWork::count(33, 33);
    work.gpu_pipelines = LoadingWork::boolean(true);
    work.gpu_stable_frames = LoadingWork::count(6, 6);
    assert!((work.progress() - 1.0).abs() < f32::EPSILON);
    assert!((loading_display_percent(work.progress()) - 100.0).abs() < f32::EPSILON);
}

#[test]
fn autostart_loading_handoff_stays_covered_and_uses_one_progress_range() {
    let loading = MenuLoadingRuntime {
        started_at: Instant::now(),
        destination: BootDestination::WorldLoading,
        progress: 1.0,
        status: String::new(),
        substatus: String::new(),
        asset_handles: Vec::new(),
        loaded_assets: 0,
        failed_assets: 0,
        ready_presented_frames: 3,
    };
    assert!((boot_loading_display_progress(&loading) - 0.2).abs() < f32::EPSILON);
    assert!(next_state_targets_world_loading(&NextState::Pending(
        GameState::WorldLoading
    )));
    assert!(next_state_targets_world_loading(&NextState::PendingIfNeq(
        GameState::WorldLoading
    )));
    assert!(!next_state_targets_world_loading(&NextState::Pending(
        GameState::MainMenu
    )));
    assert!(!next_state_targets_world_loading(&NextState::Unchanged));
}
