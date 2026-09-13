#[test]
fn role_driven_resource_loop_depletes_and_deposits() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let resource = world
        .resources
        .first()
        .expect("default world contains resources")
        .clone();
    let role = match resource.kind.as_str() {
        "resource:wood" => "role:logger",
        "resource:ore" => "role:miner",
        _ => "role:gatherer",
    };
    let actor_id = StableId::new("npc:test_worker").unwrap();
    let approach = resource_approach(&world, &resource, resource.position).unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), approach));
    simulation
        .assign_role(&actor_id, StableId::new(role).unwrap())
        .unwrap();
    let starting_amount = resource.amount;
    let gather = AgentGoal::Gather(resource.id.clone());
    for _ in 0..10 {
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &gather,
            approach,
        );
    }
    assert_eq!(
        world
            .resources
            .iter()
            .find(|candidate| candidate.id == resource.id)
            .unwrap()
            .amount,
        starting_amount - 10
    );
    assert_eq!(simulation.actors[&actor_id].inventory[&resource.kind], 10);
    assert_eq!(role_progress(&simulation.actors[&actor_id]).experience, 10);
    let (goal, _) = next_agent_goal(&simulation, &world, &config, &content, &actor_id, approach);
    assert_eq!(goal, AgentGoal::Deposit);
    complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &actor_id,
        &AgentGoal::Deposit,
        approach,
    );
    assert!(simulation.actors[&actor_id].inventory.is_empty());
    assert_eq!(simulation.town_resources[&resource.kind], 10);
    assert_eq!(
        role_progress(&simulation.actors[&actor_id]).experience,
        10,
        "Unity's standalone deposit state does not award role experience"
    );
}

#[test]
fn gathering_clamps_inventory_after_the_full_authored_action() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let wood = StableId::new("resource:wood").unwrap();
    let resource = world
        .resources
        .iter()
        .find(|resource| resource.kind == wood && resource.amount >= 2)
        .unwrap()
        .clone();
    let approach = resource_approach(&world, &resource, resource.position).unwrap();
    let actor_id = StableId::new("npc:partial_capacity_logger").unwrap();
    let logger = StableId::new("role:logger").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), approach));
    simulation.assign_role(&actor_id, logger.clone()).unwrap();
    simulation
        .actors
        .get_mut(&actor_id)
        .unwrap()
        .role_progression
        .insert(
            logger,
            stream_town_domain::RoleProgress {
                level: 5,
                experience: 0,
            },
        );
    simulation.gather(&actor_id, wood.clone(), 17).unwrap();
    let starting_resource = resource.amount;

    complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &actor_id,
        &AgentGoal::Gather(resource.id.clone()),
        approach,
    );

    assert_eq!(simulation.actors[&actor_id].inventory[&wood], 18);
    assert_eq!(
        world
            .resources
            .iter()
            .find(|candidate| candidate.id == resource.id)
            .unwrap()
            .amount,
        starting_resource - 3,
        "Unity takes the full action amount before PlayerInventory clamps the stored result"
    );
    assert_eq!(
        role_progress(&simulation.actors[&actor_id]).experience,
        3,
        "Unity awards BaseActionAmount rather than only the amount that fit"
    );
}

#[test]
fn role_switch_keeps_independent_inventory_capacity_and_deposits_current_resource_only() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let actor_id = StableId::new("npc:role_switch_inventory").unwrap();
    let logger = StableId::new("role:logger").unwrap();
    let miner = StableId::new("role:miner").unwrap();
    let wood = StableId::new("resource:wood").unwrap();
    let ore = StableId::new("resource:ore").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), GridPos { x: 1, z: 1 }));
    simulation.assign_role(&actor_id, logger.clone()).unwrap();
    simulation.gather(&actor_id, wood.clone(), 50).unwrap();
    simulation.assign_role(&actor_id, miner.clone()).unwrap();

    let miner_capacity = effective_role_stats(&content, &simulation, &simulation.actors[&actor_id])
        .unwrap()
        .carry_capacity;
    assert_eq!(
        actor_remaining_carry_capacity(&content, &simulation, &simulation.actors[&actor_id]),
        miner_capacity,
        "wood retained from the logger role must not consume ore capacity"
    );
    assert!(!actor_carries_role_resource(
        &content,
        &simulation.actors[&actor_id]
    ));

    simulation.gather(&actor_id, ore.clone(), 3).unwrap();
    assert_eq!(
        actor_remaining_carry_capacity(&content, &simulation, &simulation.actors[&actor_id]),
        miner_capacity.saturating_sub(3)
    );
    assert!(actor_carries_role_resource(
        &content,
        &simulation.actors[&actor_id]
    ));

    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &AgentGoal::Deposit,
            GridPos { x: 1, z: 1 },
        )
        .is_none()
    );
    assert_eq!(simulation.town_resources[&ore], 3);
    assert_eq!(simulation.actors[&actor_id].inventory[&wood], 50);
    assert!(!simulation.actors[&actor_id].inventory.contains_key(&ore));
    assert!(!actor_carries_role_resource(
        &content,
        &simulation.actors[&actor_id]
    ));

    simulation.assign_role(&actor_id, logger).unwrap();
    assert!(actor_carries_role_resource(
        &content,
        &simulation.actors[&actor_id]
    ));
}

#[test]
fn native_world_compatibility_accepts_current_and_checksum_valid_prior_generators() {
    let world = generate_world(&GameConfig::default().world);
    assert_eq!(
        native_world_compatibility(
            world.seed,
            world.generator_version,
            &world.deterministic_hash,
            &world,
        ),
        Some(NativeWorldCompatibility::Current)
    );
    assert_eq!(
        native_world_compatibility(
            world.seed,
            1,
            &stream_town_domain::legacy_v1_world_hash(&world),
            &world,
        ),
        Some(NativeWorldCompatibility::UpgradeV1)
    );
    assert_eq!(
        native_world_compatibility(
            world.seed,
            2,
            &stream_town_domain::legacy_v2_world_hash(&world),
            &world,
        ),
        Some(NativeWorldCompatibility::UpgradeV2)
    );
    assert_eq!(
        native_world_compatibility(
            world.seed,
            3,
            &stream_town_domain::legacy_v3_world_hash(&world),
            &world,
        ),
        Some(NativeWorldCompatibility::UpgradeV3)
    );
    assert_eq!(
        native_world_compatibility(world.seed, 1, "corrupt", &world),
        None
    );
    assert_eq!(
        native_world_compatibility(world.seed, 4, "validated-v4-fingerprint", &world),
        Some(NativeWorldCompatibility::RegeneratePrior)
    );
    assert_eq!(
        native_world_compatibility(world.seed, 5, "validated-v5-fingerprint", &world),
        Some(NativeWorldCompatibility::RegeneratePrior)
    );
    assert_eq!(
        native_world_compatibility(world.seed, 6, "validated-v6-fingerprint", &world),
        Some(NativeWorldCompatibility::RegeneratePrior)
    );
    assert_eq!(
        native_world_compatibility(
            world.seed.wrapping_add(1),
            world.generator_version,
            &world.deterministic_hash,
            &world,
        ),
        None
    );
}

#[test]
fn prior_native_resource_stock_scales_once_for_long_lived_land_nodes() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world_with_content(&config.world, &content);
    for resource in &world.resources {
        let expected = match resource.target_kind.as_str() {
            "target:tree" => 500,
            "target:ore" | "target:bush" => 10_000,
            "target:fish" => 100,
            kind => panic!("unexpected resource target {kind}"),
        };
        assert_eq!(
            upgraded_resource_remaining(6, resource, 100),
            expected,
            "{} should be upgraded from a version-6 save",
            resource.target_kind
        );
        assert_eq!(
            upgraded_resource_remaining(7, resource, expected),
            expected,
            "{} must not be upgraded twice",
            resource.target_kind
        );
        assert_eq!(upgraded_resource_remaining(6, resource, 0), 0);
    }
}

#[test]
fn food_roles_only_select_their_authored_target_types() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let bush = world
        .resources
        .iter()
        .find(|resource| resource.target_kind.as_str() == "target:bush")
        .unwrap();
    let fish = world
        .resources
        .iter()
        .find(|resource| resource.target_kind.as_str() == "target:fish")
        .unwrap();

    for (serial, role, expected_target, rejected_target) in [
        (0, "role:gatherer", bush, fish),
        (1, "role:fisher", fish, bush),
    ] {
        let actor_id = StableId::new(format!("npc:typed_food_{serial}")).unwrap();
        let current = resource_approach(&world, expected_target, expected_target.position)
            .expect("generated resource has an approach");
        let mut simulation = WorldSimulation::new(world.seed);
        assert!(simulation.join_player(actor_id.clone(), current));
        simulation
            .assign_role(&actor_id, StableId::new(role).unwrap())
            .unwrap();
        let (goal, _) = next_agent_goal(&simulation, &world, &config, &content, &actor_id, current);
        let compatible = compatible_target_ids(
            &content,
            &simulation,
            &world,
            &config,
            &simulation.actors[&actor_id],
        );
        let AgentGoal::Gather(selected) = goal else {
            panic!("food worker did not select a generated resource");
        };
        assert!(compatible.contains(&selected));
        assert_eq!(
            world
                .resources
                .iter()
                .find(|resource| resource.id == selected)
                .map(|resource| &resource.target_kind),
            Some(&expected_target.target_kind),
        );
        assert!(!compatible.contains(&rejected_target.id));
    }
}

#[test]
fn generated_resource_reservations_are_exclusive_and_fail_over() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let starting_resource = world
        .resources
        .iter()
        .find(|resource| resource.target_kind.as_str() == "target:tree")
        .unwrap();
    let current = starting_resource.position;
    let first = StableId::new("npc:reservation_first").unwrap();
    let second = StableId::new("npc:reservation_second").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    for actor in [&first, &second] {
        assert!(simulation.join_player(actor.clone(), current));
        simulation
            .assign_role(actor, StableId::new("role:logger").unwrap())
            .unwrap();
    }
    let AgentGoal::Gather(claimed) = next_agent_goal_with_reservations(
        &simulation,
        &world,
        &config,
        &content,
        &first,
        current,
        &BTreeMap::new(),
        &BTreeMap::new(),
    )
    .0
    else {
        panic!("logger did not select a station-catalog resource");
    };
    let reservations = BTreeMap::from([(claimed.clone(), first.clone())]);
    assert_eq!(
        next_agent_goal_with_reservations(
            &simulation,
            &world,
            &config,
            &content,
            &first,
            current,
            &reservations,
            &BTreeMap::new(),
        )
        .0,
        AgentGoal::Gather(claimed.clone())
    );
    let second_goal = next_agent_goal_with_reservations(
        &simulation,
        &world,
        &config,
        &content,
        &second,
        current,
        &reservations,
        &BTreeMap::new(),
    )
    .0;
    assert!(matches!(second_goal, AgentGoal::Gather(ref id) if id != &claimed));
    assert!(reservation_available(&BTreeMap::new(), &second, &claimed));
}

#[test]
fn unreachable_resource_route_is_rejected_before_alternate_target_retry() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let mut template = world
        .resources
        .iter()
        .find(|resource| resource.target_kind.as_str() == "target:tree")
        .unwrap()
        .clone();
    let mut simulation = WorldSimulation::new(world.seed);
    let current = restored_town_hall_position(&content, &simulation, &config);
    let bad_position = GridPos {
        x: current.x + 2,
        z: current.z,
    };
    let good_position = GridPos {
        x: current.x + 5,
        z: current.z,
    };
    world.navigation = stream_town_domain::NavGrid::new(
        config.world.width,
        config.world.height,
        vec![false; usize::from(config.world.width) * usize::from(config.world.height)],
        vec![0; usize::from(config.world.width) * usize::from(config.world.height)],
    )
    .unwrap();
    world
        .navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: GridPos {
                    x: bad_position.x - 1,
                    z: bad_position.z - 1,
                },
                max: GridPos {
                    x: bad_position.x + 1,
                    z: bad_position.z + 1,
                },
            },
            true,
        )
        .unwrap();
    world
        .navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: bad_position,
                max: bad_position,
            },
            false,
        )
        .unwrap();
    template.id = StableId::new("resource:unreachable-tree").unwrap();
    template.position = bad_position;
    template.amount = 100;
    let mut reachable = template.clone();
    reachable.id = StableId::new("resource:reachable-tree").unwrap();
    reachable.position = good_position;
    world.resources = vec![template.clone(), reachable.clone()];

    let actor_id = StableId::new("npc:path-retry-logger").unwrap();
    assert!(simulation.join_player(actor_id.clone(), current));
    simulation
        .assign_role(&actor_id, StableId::new("role:logger").unwrap())
        .unwrap();
    let town_hall = StableId::new("building:townhall").unwrap();
    let tree_kind = StableId::new("target:tree").unwrap();
    let station_targets = StationTargetRuntime {
        stations: BTreeMap::from([(
            town_hall,
            CachedStationTargets {
                targets: BTreeMap::from([(
                    tree_kind,
                    vec![template.id.clone(), reachable.id.clone()],
                )]),
                ..default()
            },
        )]),
        ..default()
    };
    let (first_goal, first_target) = next_agent_goal_with_station_runtime(
        &simulation,
        &world,
        None,
        &config,
        &content,
        &station_targets,
        &actor_id,
        current,
        placement_to_navigation_centre(current),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
    );
    assert_eq!(first_goal, AgentGoal::Gather(template.id.clone()));
    assert!(
        try_agent_path(
            &world.navigation,
            &content,
            &simulation,
            &ActorKind::Player,
            current,
            first_target,
        )
        .is_none()
    );
    let retry_reservations = BTreeMap::from([(
        template.id,
        StableId::new("system:unreachable-resource").unwrap(),
    )]);
    let (retry_goal, retry_target) = next_agent_goal_with_station_runtime(
        &simulation,
        &world,
        None,
        &config,
        &content,
        &station_targets,
        &actor_id,
        current,
        placement_to_navigation_centre(current),
        &retry_reservations,
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
    );
    assert_eq!(retry_goal, AgentGoal::Gather(reachable.id));
    assert!(
        try_agent_path(
            &world.navigation,
            &content,
            &simulation,
            &ActorKind::Player,
            current,
            retry_target,
        )
        .is_some()
    );
}

#[test]
fn generated_resource_claims_respect_the_assigned_stations_authored_catalog_cap() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    world.resources.clear();
    let current = restored_town_hall_position(&content, &WorldSimulation::new(world.seed), &config);
    for offset in 1..=31_u16 {
        world.resources.push(stream_town_domain::GeneratedResource {
            id: StableId::new(format!("resource:target_window_{offset:02}")).unwrap(),
            kind: StableId::new("resource:wood").unwrap(),
            target_kind: StableId::new("target:tree").unwrap(),
            position: GridPos {
                x: current.x + offset,
                z: current.z,
            },
            offset_milli_cells: [0, 0],
            generation_occupancy: [0, 0],
            amount: 100,
        });
    }
    let actor_id = StableId::new("npc:target_window_logger").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), current));
    simulation
        .assign_role(&actor_id, StableId::new("role:logger").unwrap())
        .unwrap();
    let reservations: BTreeMap<_, _> = world
        .resources
        .iter()
        .take(30)
        .map(|resource| {
            (
                resource.id.clone(),
                StableId::new(format!(
                    "npc:claim_{}",
                    resource.id.as_str().replace(':', "_")
                ))
                .unwrap(),
            )
        })
        .collect();

    assert!(matches!(
        next_agent_goal_with_reservations(
            &simulation,
            &world,
            &config,
            &content,
            &actor_id,
            current,
            &reservations,
            &BTreeMap::new(),
        )
        .0,
        AgentGoal::Wander
    ));
    assert!(world.resources[30].id.as_str().ends_with("31"));
}

#[test]
fn stale_resource_reservations_release_after_role_change_or_depletion() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let claimed = world
        .resources
        .iter()
        .find(|resource| resource.target_kind.as_str() == "target:tree")
        .unwrap()
        .clone();
    let actor_id = StableId::new("npc:stale_reservation").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), claimed.position));
    simulation
        .assign_role(&actor_id, StableId::new("role:logger").unwrap())
        .unwrap();
    let goal = AgentGoal::Gather(claimed.id.clone());
    assert!(goal_reservation_is_valid(
        &simulation,
        &world,
        &content,
        &actor_id,
        &goal,
    ));
    simulation
        .assign_role(&actor_id, StableId::new("role:miner").unwrap())
        .unwrap();
    assert!(!goal_reservation_is_valid(
        &simulation,
        &world,
        &content,
        &actor_id,
        &goal,
    ));
    simulation
        .assign_role(&actor_id, StableId::new("role:logger").unwrap())
        .unwrap();
    world
        .resources
        .iter_mut()
        .find(|resource| resource.id == claimed.id)
        .unwrap()
        .amount = 0;
    assert!(!goal_reservation_is_valid(
        &simulation,
        &world,
        &content,
        &actor_id,
        &goal,
    ));
}

#[test]
fn land_resource_occupancy_requires_an_edge_action_and_clears_after_last_depletion() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let resource = world
        .resources
        .iter()
        .find(|resource| {
            resource.target_kind.as_str() == "target:tree"
                && !world.navigation.is_walkable(resource.position)
                && world
                    .resources
                    .iter()
                    .filter(|candidate| {
                        candidate.target_kind.as_str() != "target:fish"
                            && candidate.position == resource.position
                    })
                    .count()
                    == 1
        })
        .unwrap()
        .clone();
    let mut overlapping_resource = resource.clone();
    overlapping_resource.id = StableId::new("resource:overlapping_tree").unwrap();
    overlapping_resource.amount = 1;
    world.resources.push(overlapping_resource.clone());
    let approach = resource_approach(&world, &resource, resource.position).unwrap();
    let actor_id = StableId::new("npc:resource_edge_test").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), approach));
    simulation
        .assign_role(&actor_id, StableId::new("role:logger").unwrap())
        .unwrap();
    world
        .resources
        .iter_mut()
        .find(|candidate| candidate.id == resource.id)
        .unwrap()
        .amount = 1;
    let goal = AgentGoal::Gather(resource.id.clone());
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &goal,
            resource.position,
        )
        .is_none()
    );
    assert_eq!(
        world
            .resources
            .iter()
            .find(|candidate| candidate.id == resource.id)
            .unwrap()
            .amount,
        1
    );
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &goal,
            approach,
        )
        .is_none()
    );
    assert!(!world.navigation.is_walkable(resource.position));
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &AgentGoal::Gather(overlapping_resource.id),
            approach,
        )
        .is_none()
    );
    assert!(world.navigation.is_walkable(resource.position));
}

#[test]
fn farmer_harvests_unlimited_food_from_completed_farm() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let farm = &content.buildings[&StableId::new("building:farm").unwrap()];
    let position = find_building_site(
        &world,
        GridPos {
            x: config.world.width / 2,
            z: config.world.height / 2,
        },
        farm.footprint,
    )
    .unwrap();
    let farm_id = StableId::new("building:test_farm").unwrap();
    let region = building_region(position, farm.footprint, &world).unwrap();
    world.navigation.set_blocked(region, true).unwrap();
    let current = building_approach(&world, position, farm.footprint, position).unwrap();
    let actor_id = StableId::new("npc:farmer_test").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.buildings.insert(
        farm_id.clone(),
        BuildingState {
            id: farm_id.clone(),
            archetype: farm.archetype.clone(),
            position,
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(&content, farm)).unwrap(),
            complete: true,
        },
    );
    assert!(simulation.join_player(actor_id.clone(), current));
    simulation
        .assign_role(&actor_id, StableId::new("role:farmer").unwrap())
        .unwrap();

    let (goal, target) =
        next_agent_goal(&simulation, &world, &config, &content, &actor_id, current);
    assert_eq!(goal, AgentGoal::HarvestFarm(farm_id.clone()));
    assert_eq!(target, current);
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &goal,
            current,
        )
        .is_none()
    );
    let food = StableId::new("resource:food").unwrap();
    assert_eq!(simulation.actors[&actor_id].inventory[&food], 1);
    assert_eq!(role_progress(&simulation.actors[&actor_id]).experience, 1);
    assert!(
        compatible_target_ids(
            &content,
            &simulation,
            &world,
            &config,
            &simulation.actors[&actor_id],
        )
        .contains(&farm_id)
    );
}

#[test]
fn authored_assignment_penalty_spreads_farmers_across_farms() {
    let config = GameConfig::default();
    let content = embedded_content();
    let farm = &content.buildings[&StableId::new("building:farm").unwrap()];
    assert_eq!(
        farm.targeting,
        Some(TargetingScoreDef {
            assignment_penalty_milli: 10_000_000,
            distance_penalty_milli_per_cell: 100,
        })
    );
    let mut world = generate_world(&config.world);
    let first_position =
        find_building_site(&world, GridPos { x: 16, z: 16 }, farm.footprint).unwrap();
    world
        .navigation
        .set_blocked(
            building_region(first_position, farm.footprint, &world).unwrap(),
            true,
        )
        .unwrap();
    let second_position =
        find_building_site(&world, GridPos { x: 48, z: 48 }, farm.footprint).unwrap();
    world
        .navigation
        .set_blocked(
            building_region(second_position, farm.footprint, &world).unwrap(),
            true,
        )
        .unwrap();
    let first_id = StableId::new("building:assignment_farm_a").unwrap();
    let second_id = StableId::new("building:assignment_farm_b").unwrap();
    let actor_id = StableId::new("npc:assignment_farmer").unwrap();
    let current =
        building_approach(&world, first_position, farm.footprint, first_position).unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    for (id, position) in [
        (first_id.clone(), first_position),
        (second_id.clone(), second_position),
    ] {
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id,
                archetype: farm.archetype.clone(),
                position,
                rotation_quarter_turns: 0,
                level: 1,
                health: i32::try_from(building_base_max_health(&content, farm)).unwrap(),
                complete: true,
            },
        );
    }
    assert!(simulation.join_player(actor_id.clone(), current));
    simulation
        .assign_role(&actor_id, StableId::new("role:farmer").unwrap())
        .unwrap();

    assert_eq!(
        next_agent_goal_with_reservations(
            &simulation,
            &world,
            &config,
            &content,
            &actor_id,
            current,
            &BTreeMap::new(),
            &BTreeMap::new(),
        )
        .0,
        AgentGoal::HarvestFarm(first_id.clone()),
    );
    assert_eq!(
        next_agent_goal_with_reservations(
            &simulation,
            &world,
            &config,
            &content,
            &actor_id,
            current,
            &BTreeMap::new(),
            &BTreeMap::from([(first_id, 1)]),
        )
        .0,
        AgentGoal::HarvestFarm(second_id),
    );
}

#[test]
fn full_town_storage_pauses_gathering_and_preserves_carried_overflow() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let resource = world
        .resources
        .first()
        .expect("default world contains resources")
        .clone();
    let role = match resource.kind.as_str() {
        "resource:wood" => "role:logger",
        "resource:ore" => "role:miner",
        _ => "role:gatherer",
    };
    let actor_id = StableId::new("npc:storage_backpressure_test").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), resource.position));
    simulation
        .assign_role(&actor_id, StableId::new(role).unwrap())
        .unwrap();
    simulation
        .actors
        .get_mut(&actor_id)
        .unwrap()
        .inventory
        .insert(resource.kind.clone(), 10);
    let capacity = resource_storage_capacity(&config, &content, &simulation, &resource.kind);
    simulation
        .town_resources
        .insert(resource.kind.clone(), capacity);
    let resource_amount = resource.amount;

    let (goal, target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &actor_id,
        resource.position,
    );
    assert_eq!(goal, AgentGoal::WaitForStorage);
    assert_eq!(
        target,
        town_hall_wait_target(&content, &simulation, &world, &config, resource.position,)
    );
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &AgentGoal::Gather(resource.id.clone()),
            resource.position,
        )
        .is_none()
    );
    assert_eq!(
        world
            .resources
            .iter()
            .find(|candidate| candidate.id == resource.id)
            .unwrap()
            .amount,
        resource_amount
    );
    complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &actor_id,
        &AgentGoal::Deposit,
        resource.position,
    );
    assert_eq!(simulation.actors[&actor_id].inventory[&resource.kind], 10);

    simulation
        .town_resources
        .insert(resource.kind.clone(), capacity - 1);
    let (goal, _) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &actor_id,
        resource.position,
    );
    assert_eq!(goal, AgentGoal::Deposit);
    complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &actor_id,
        &goal,
        resource.position,
    );
    assert_eq!(simulation.town_resources[&resource.kind], capacity);
    assert_eq!(simulation.actors[&actor_id].inventory[&resource.kind], 9);

    simulation
        .town_resources
        .insert(resource.kind.clone(), capacity - 10);
    complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &actor_id,
        &AgentGoal::Deposit,
        resource.position,
    );
    assert!(simulation.actors[&actor_id].inventory.is_empty());
    let (goal, _) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &actor_id,
        resource.position,
    );
    assert!(matches!(goal, AgentGoal::Gather(_)));
}

#[test]
fn marketplace_passive_income_is_level_scaled_and_save_stable() {
    let config = GameConfig::default();
    let content = embedded_content();
    let marketplace = &content.buildings[&StableId::new("building:marketplace").unwrap()];
    let building_id = StableId::new("building:runtime_marketplace").unwrap();
    let gold = StableId::new("resource:gold").unwrap();
    let mut simulation = WorldSimulation::new(config.world.seed);
    simulation.town_resources.insert(gold.clone(), 0);
    simulation.buildings.insert(
        building_id.clone(),
        BuildingState {
            id: building_id.clone(),
            archetype: marketplace.archetype.clone(),
            position: GridPos { x: 8, z: 8 },
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );

    apply_passive_building_income(&config, &content, &mut simulation, Duration::from_secs(2));
    assert_eq!(simulation.town_resources[&gold], 0);
    apply_passive_building_income(&config, &content, &mut simulation, Duration::from_nanos(1));
    assert_eq!(simulation.town_resources[&gold], 1);

    let encoded = ron::to_string(&simulation).unwrap();
    let mut restored: WorldSimulation = ron::from_str(&encoded).unwrap();
    assert_eq!(
        restored.passive_resource_accumulators,
        simulation.passive_resource_accumulators
    );
    restored.buildings.get_mut(&building_id).unwrap().level = 2;
    apply_passive_building_income(&config, &content, &mut restored, Duration::from_secs(1));
    assert_eq!(restored.town_resources[&gold], 2);

    restored.buildings.get_mut(&building_id).unwrap().complete = false;
    apply_passive_building_income(&config, &content, &mut restored, Duration::from_secs(10));
    assert_eq!(restored.town_resources[&gold], 2);
    assert!(restored.passive_resource_accumulators.is_empty());
}

#[test]
fn passive_income_batches_exact_elapsed_time_at_four_hertz() {
    let mut accumulated = Duration::ZERO;
    assert_eq!(
        take_passive_building_income_delta(&mut accumulated, Duration::from_millis(100)),
        None
    );
    assert_eq!(
        take_passive_building_income_delta(&mut accumulated, Duration::from_millis(149)),
        None
    );
    assert_eq!(
        take_passive_building_income_delta(&mut accumulated, Duration::from_millis(1)),
        Some(Duration::from_millis(250))
    );
    assert_eq!(accumulated, Duration::ZERO);
    assert_eq!(
        take_passive_building_income_delta(&mut accumulated, Duration::from_millis(275)),
        Some(Duration::from_millis(275))
    );
    assert_eq!(accumulated, Duration::ZERO);
}
