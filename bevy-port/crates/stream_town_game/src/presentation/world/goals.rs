#[allow(clippy::too_many_arguments)]
pub(crate) fn next_agent_goal_with_station_runtime(
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    enemy_navigation: Option<&EnemyNavigationField>,
    config: &GameConfig,
    content: &ContentCatalog,
    station_targets: &StationTargetRuntime,
    actor_id: &StableId,
    current: GridPos,
    current_navigation: GridPos,
    reservations: &BTreeMap<StableId, StableId>,
    target_assignments: &BTreeMap<StableId, u32>,
    occupied_approaches: &BTreeMap<GridPos, StableId>,
    approach_reservations: &BTreeMap<GridPos, StableId>,
) -> (AgentGoal, GridPos) {
    let Some(actor) = simulation.actors.get(actor_id) else {
        return (
            AgentGoal::Wander,
            deterministic_wander_target(world, actor_id, current),
        );
    };
    if !actor.alive {
        return (AgentGoal::Wander, current);
    }
    let fallback_station =
        best_station_id(content, simulation, config, &actor.role, actor.position);
    let station = assigned_station(content, simulation, config, actor).or_else(|| {
        fallback_station
            .as_ref()
            .and_then(|id| station_candidate(content, simulation, config, id))
    });
    if let Some(preferred) = actor.preferred_target.as_ref() {
        if actor.role.as_str() == "role:enemy" {
            if let Some(target) = simulation
                .actors
                .get(preferred)
                .filter(|target| target.alive && target.role.as_str() != "role:enemy")
                .filter(|target| {
                    enemy_targets_kind(content, actor, "target:player")
                        || (target.health < target.max_health
                            && enemy_targets_kind(content, actor, "target:injured_player"))
                })
            {
                return (
                    AgentGoal::Attack(target.id.clone()),
                    if within_actor_attack_range(content, simulation, actor, target, current) {
                        current
                    } else {
                        target.position
                    },
                );
            }
            if let Some(building) = simulation
                .buildings
                .get(preferred)
                .filter(|building| building.health > 0)
                .filter(|building| enemy_can_attack_building(content, building))
                .filter(|building| {
                    enemy_targets_kind(content, actor, "target:building")
                        || (!building.complete
                            && enemy_targets_kind(content, actor, "target:construction"))
                        || (building.complete
                            && building.health < building_max_health(content, building)
                            && enemy_targets_kind(content, actor, "target:damaged_building"))
                })
                && let Some(definition) = building_def_for_archetype(content, &building.archetype)
                && let Some(approach) = building_approach(
                    world,
                    building.position,
                    rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                    current,
                )
            {
                return (
                    AgentGoal::AttackBuilding(building.id.clone()),
                    if within_enemy_building_attack_range(
                        content, simulation, actor, building, current,
                    ) {
                        current
                    } else {
                        approach
                    },
                );
            }
        }
        if let Some(target) = simulation
            .actors
            .get(preferred)
            .filter(|target| target.alive && target.role.as_str() == "role:enemy")
            .filter(|_| is_combat_role(&actor.role))
        {
            return (
                AgentGoal::Attack(target.id.clone()),
                if within_actor_attack_range(content, simulation, actor, target, current) {
                    current
                } else {
                    target.position
                },
            );
        }
        if let Some(target) = simulation
            .actors
            .get(preferred)
            .filter(|target| {
                target.alive
                    && target.role.as_str() != "role:enemy"
                    && target.health < target.max_health
            })
            .filter(|_| is_healer_role(&actor.role))
        {
            return (
                AgentGoal::Heal(target.id.clone()),
                if within_actor_heal_range(content, simulation, actor, target, current) {
                    current
                } else {
                    target.position
                },
            );
        }
        if let Some(resource) = world
            .resources
            .iter()
            .find(|resource| resource.id == *preferred && resource.amount > 0)
            .filter(|resource| {
                station.is_some_and(|station| {
                    cached_station_targets(station_targets, station, &resource.target_kind)
                        .contains(&resource.id)
                })
            })
            .filter(|resource| reservation_available(reservations, actor_id, &resource.id))
            .filter(|resource| actor_accepts_resource(content, actor, resource))
            .filter(|_| actor_remaining_carry_capacity(content, simulation, actor) > 0)
            .filter(|_| actor_resource_storage_has_room(config, content, simulation, actor))
            && let Some(approach) = resource_approach(world, resource, current)
        {
            return (AgentGoal::Gather(resource.id.clone()), approach);
        }
        if let Some(building) = simulation
            .buildings
            .get(preferred)
            .filter(|building| is_farm_resource_building(content, building))
            .filter(|building| {
                let farm = StableId::new("target:farm").expect("static target ID");
                station.is_some_and(|station| {
                    cached_station_targets(station_targets, station, &farm).contains(&building.id)
                })
            })
            .filter(|building| reservation_available(reservations, actor_id, &building.id))
            .filter(|_| {
                content.roles.get(&actor.role).is_some_and(|role| {
                    role_accepts_target(
                        role,
                        &StableId::new("target:farm").expect("static target ID"),
                    )
                })
            })
            .filter(|_| actor_remaining_carry_capacity(content, simulation, actor) > 0)
            .filter(|_| actor_resource_storage_has_room(config, content, simulation, actor))
            && let Some(definition) = building_def_for_archetype(content, &building.archetype)
            && let Some(approach) = unoccupied_building_approach(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                current,
                actor_id,
                occupied_approaches,
                approach_reservations,
            )
        {
            return (AgentGoal::HarvestFarm(building.id.clone()), approach);
        }
        if let Some(building) = simulation.buildings.get(preferred).filter(|building| {
            actor.role.as_str() == "role:builder"
                && (!building.complete || building.health < building_max_health(content, building))
        }) && let Some(definition) = building_def_for_archetype(content, &building.archetype)
            && let Some(approach) = unoccupied_construction_approach(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                current,
                actor_id,
                occupied_approaches,
                approach_reservations,
            )
        {
            return (AgentGoal::Construct(building.id.clone()), approach);
        }
    }
    if is_healer_role(&actor.role) {
        let candidates: Vec<_> = simulation
            .actors
            .values()
            .filter(|target| {
                target.id != actor.id
                    && target.alive
                    && target.role.as_str() != "role:enemy"
                    && target.health < target.max_health
            })
            .filter(|target| within_player_target_search_region(target.position, current))
            .collect();
        if let Some(target) = candidates.into_iter().min_by_key(|target| {
            (
                grid_distance_squared(target.position, current),
                target.id.clone(),
            )
        }) {
            return (
                AgentGoal::Heal(target.id.clone()),
                if within_actor_heal_range(content, simulation, actor, target, current) {
                    current
                } else {
                    target.position
                },
            );
        }
    }
    if actor.role.as_str() == "role:enemy" {
        let player_target = enemy_targets_kind(content, actor, "target:player")
            .then(|| {
                simulation
                    .actors
                    .values()
                    .filter(|target| target.alive && target.role.as_str() != "role:enemy")
                    .filter(|target| {
                        within_enemy_target_search(content, actor, current, target.position)
                    })
                    .map(|target| {
                        (
                            grid_distance_squared(target.position, current),
                            target.id.clone(),
                            target.position,
                        )
                    })
                    .min_by_key(|(distance, id, _)| (*distance, id.clone()))
            })
            .flatten();
        let building_target = enemy_targets_buildings(content, actor)
            .then(|| {
                simulation
                    .buildings
                    .values()
                    .filter(|building| building.health > 0)
                    .filter(|building| enemy_can_attack_building(content, building))
                    .filter(|building| {
                        within_enemy_target_search(
                            content,
                            actor,
                            current,
                            building_visual_grid(content, building),
                        )
                    })
                    .filter_map(|building| {
                        let definition = building_def_for_archetype(content, &building.archetype)?;
                        let approach = building_approach(
                            world,
                            building.position,
                            rotated_footprint(
                                definition.footprint,
                                building.rotation_quarter_turns,
                            ),
                            current,
                        )?;
                        Some((
                            grid_distance_squared(building_visual_grid(content, building), current),
                            building.id.clone(),
                            approach,
                        ))
                    })
                    .min_by_key(|(distance, id, _)| (*distance, id.clone()))
            })
            .flatten();
        match (player_target, building_target) {
            (Some((player_distance, player, position)), Some((building_distance, _, _)))
                if player_distance <= building_distance =>
            {
                let target = &simulation.actors[&player];
                return (
                    AgentGoal::Attack(player),
                    if within_actor_attack_range(content, simulation, actor, target, current) {
                        current
                    } else {
                        position
                    },
                );
            }
            (_, Some((_, building, approach))) => {
                let target = &simulation.buildings[&building];
                return (
                    AgentGoal::AttackBuilding(building),
                    if within_enemy_building_attack_range(
                        content, simulation, actor, target, current,
                    ) {
                        current
                    } else {
                        approach
                    },
                );
            }
            (Some((_, player, position)), None) => {
                let target = &simulation.actors[&player];
                return (
                    AgentGoal::Attack(player),
                    if within_actor_attack_range(content, simulation, actor, target, current) {
                        current
                    } else {
                        position
                    },
                );
            }
            (None, None) => {}
        }
        if enemy_targets_buildings(content, actor) {
            // Advance on the Town Hall when it is reachable. If fortifications
            // seal it off, attack the reachable building closest to it instead
            // of repeatedly requesting an impossible A* path and idling.
            let route = enemy_navigation
                .and_then(|navigation| navigation.destination(current_navigation))
                .and_then(|(building, approach)| {
                    simulation
                        .buildings
                        .get(building)
                        .filter(|building| building.health > 0)
                        .map(|building| (building.id.clone(), navigation_to_placement(approach)))
                })
                .or_else(|| {
                    if enemy_navigation.is_none() {
                        nearest_reachable_building_to_town_hall(content, simulation, world, current)
                    } else {
                        None
                    }
                });
            if let Some((building_id, approach)) = route {
                let building = &simulation.buildings[&building_id];
                return (
                    AgentGoal::AttackBuilding(building_id),
                    if within_enemy_building_attack_range(
                        content, simulation, actor, building, current,
                    ) {
                        current
                    } else {
                        approach
                    },
                );
            }
        }
    }
    let combat_target = if is_combat_role(&actor.role) {
        let defender_anchor = (actor.role.as_str() == "role:defender").then(|| {
            guardhouse_for_defender(simulation, &actor.id).map_or_else(
                || restored_town_hall_position(content, simulation, config),
                |building| building_visual_grid(content, building),
            )
        });
        simulation
            .actors
            .values()
            .filter(|target| target.alive && target.role.as_str() == "role:enemy")
            .filter(|target| within_player_target_search_region(target.position, current))
            .min_by_key(|target| {
                let actor_distance = grid_distance_squared(target.position, current);
                let nearby = defender_anchor.is_some()
                    && actor_distance <= u64::from(DEFENDER_LOCAL_PRIORITY_RADIUS_CELLS).pow(2);
                (
                    u8::from(defender_anchor.is_some() && !nearby),
                    if nearby {
                        actor_distance
                    } else {
                        defender_anchor.map_or(actor_distance, |anchor| {
                            grid_distance_squared(target.position, anchor)
                        })
                    },
                    actor_distance,
                    target.id.clone(),
                )
            })
    } else {
        None
    };
    if let Some(target) = combat_target {
        let destination = if within_actor_attack_range(content, simulation, actor, target, current)
        {
            current
        } else {
            target.position
        };
        return (AgentGoal::Attack(target.id.clone()), destination);
    }
    if actor.role.as_str() == "role:builder" {
        let candidates: Vec<_> = simulation
            .buildings
            .values()
            .filter(|building| {
                !building.complete || building.health < building_max_health(content, building)
            })
            .filter(|building| {
                within_player_target_search_region(building_visual_grid(content, building), current)
            })
            .filter_map(|building| {
                let definition = building_def_for_archetype(content, &building.archetype)?;
                let approach = unoccupied_construction_approach(
                    world,
                    building.position,
                    rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                    current,
                    actor_id,
                    occupied_approaches,
                    approach_reservations,
                )?;
                Some((
                    grid_distance_squared(building_visual_grid(content, building), current),
                    building.id.clone(),
                    approach,
                ))
            })
            .collect();
        let construction = candidates
            .into_iter()
            .min_by_key(|(distance, id, _)| (*distance, id.clone()));
        if let Some((_, building, approach)) = construction {
            return (AgentGoal::Construct(building), approach);
        }
    }
    if !actor_resource_storage_has_room(config, content, simulation, actor) {
        let carrying_role_resource = resource_for_role(content, &actor.role)
            .is_some_and(|kind| actor.inventory.get(&kind).copied().unwrap_or_default() > 0);
        if carrying_role_resource && actor_remaining_carry_capacity(content, simulation, actor) == 0
        {
            return (
                AgentGoal::WaitForStorage,
                town_hall_wait_target(content, simulation, world, config, current),
            );
        }
        return (
            AgentGoal::Wander,
            deterministic_wander_target(world, actor_id, current),
        );
    }
    if actor_remaining_carry_capacity(content, simulation, actor) == 0 {
        let destination = station.map_or_else(
            || {
                nearest_walkable(
                    world,
                    restored_town_hall_position(content, simulation, config),
                )
                .unwrap_or(current)
            },
            |station| {
                unoccupied_station_deposit_approach(
                    content,
                    simulation,
                    world,
                    station,
                    current,
                    actor_id,
                    occupied_approaches,
                    approach_reservations,
                )
                .unwrap_or(current)
            },
        );
        return (AgentGoal::Deposit, destination);
    }
    let Some(resource_kind) = resource_for_role(content, &actor.role) else {
        return (
            AgentGoal::Wander,
            deterministic_wander_target(world, actor_id, current),
        );
    };
    let Some(station) = station else {
        return (
            AgentGoal::Wander,
            deterministic_wander_target(world, actor_id, current),
        );
    };
    if content.roles.get(&actor.role).is_some_and(|role| {
        role_accepts_target(
            role,
            &StableId::new("target:farm").expect("static target ID"),
        )
    }) {
        let farm_kind = StableId::new("target:farm").expect("static target ID");
        let cached_farms = cached_station_targets(station_targets, station, &farm_kind)
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut farms: Vec<_> = simulation
            .buildings
            .values()
            .filter(|building| is_farm_resource_building(content, building))
            .filter(|building| cached_farms.contains(&building.id))
            .filter(|building| reservation_available(reservations, actor_id, &building.id))
            .filter_map(|building| {
                let definition = building_def_for_archetype(content, &building.archetype)?;
                let approach = unoccupied_building_approach(
                    world,
                    building.position,
                    rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                    current,
                    actor_id,
                    occupied_approaches,
                    approach_reservations,
                )?;
                Some((
                    grid_distance_squared(
                        building_visual_grid(content, building),
                        station.position,
                    ),
                    definition.targeting.as_ref().map_or_else(
                        || {
                            u128::from(grid_distance_squared(
                                building_visual_grid(content, building),
                                current,
                            ))
                        },
                        |scoring| {
                            target_score_milli(
                                building_visual_grid(content, building),
                                current,
                                target_assignments
                                    .get(&building.id)
                                    .copied()
                                    .unwrap_or_default(),
                                scoring,
                            )
                        },
                    ),
                    building.id.clone(),
                    approach,
                ))
            })
            .collect();
        farms.sort_by_key(|(station_distance, _, id, _)| (*station_distance, id.clone()));
        farms.sort_by_key(|(_, score, id, _)| (*score, id.clone()));
        if let Some((_, _, building, approach)) = farms.into_iter().next() {
            return (AgentGoal::HarvestFarm(building), approach);
        }
    }
    let role = content
        .roles
        .get(&actor.role)
        .expect("live actor role is validated");
    let resource_target_kind =
        role.target_kinds
            .iter()
            .find(|kind| {
                world.resources.iter().any(|resource| {
                    resource.kind == resource_kind && resource.target_kind == **kind
                })
            })
            .cloned();
    let cached_resources = resource_target_kind
        .as_ref()
        .map(|kind| {
            cached_station_targets(station_targets, station, kind)
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let mut resources: Vec<_> = world
        .resources
        .iter()
        .filter(|resource| {
            resource.kind == resource_kind
                && resource.amount > 0
                && actor_accepts_resource(content, actor, resource)
                && cached_resources.contains(&resource.id)
        })
        .collect();
    resources.sort_by_key(|resource| {
        (
            grid_distance_squared(resource.position, current),
            resource.position.z,
            resource.position.x,
            resource.id.clone(),
        )
    });
    if let Some(goal) = resources
        .into_iter()
        .filter(|resource| reservation_available(reservations, actor_id, &resource.id))
        .find_map(|resource| {
            resource_approach(world, resource, current)
                .map(|approach| (AgentGoal::Gather(resource.id.clone()), approach))
        })
    {
        return goal;
    }
    if actor
        .inventory
        .get(&resource_kind)
        .copied()
        .unwrap_or_default()
        > 0
    {
        let destination = unoccupied_station_deposit_approach(
            content,
            simulation,
            world,
            station,
            current,
            actor_id,
            occupied_approaches,
            approach_reservations,
        )
        .unwrap_or(current);
        return (AgentGoal::Deposit, destination);
    }
    (
        AgentGoal::Wander,
        deterministic_wander_target(world, actor_id, current),
    )
}

pub(crate) fn regeneration_role_interval_seconds(role: &StableId, level: u16) -> Option<f64> {
    let progress = f64::from(level.saturating_sub(1).min(99)) / 99.0;
    match role.as_str() {
        "role:forester" => Some((300.0 + (20.0 - 300.0) * progress) * FORESTER_INTERVAL_MULTIPLIER),
        "role:tender" => {
            Some((1_800.0 + (360.0 - 1_800.0) * progress) * TENDER_INTERVAL_MULTIPLIER)
        }
        _ => None,
    }
}

pub(crate) fn community_adjusted_regeneration_interval_seconds(
    role: &StableId,
    level: u16,
    event: Option<CommunityEvent>,
) -> Option<f64> {
    regeneration_role_interval_seconds(role, level).map(|seconds| {
        seconds
            / f64::from(event.map_or(1.0, |event| {
                community_event_role_rate_multiplier(event, role)
            }))
    })
}

pub(crate) fn prospector_discovery_denominator(level: u16) -> u64 {
    let progress = u64::from(level.saturating_sub(1).min(99));
    4_000_u64.saturating_sub(3_600 * progress / 99).max(400)
}

pub(crate) fn regeneration_role_level(actor: &ActorState) -> u16 {
    actor
        .role_progression
        .get(&actor.role)
        .map_or(1, |progress| progress.level.max(1))
}

pub(crate) fn regeneration_hut<'a>(
    content: &'a ContentCatalog,
    simulation: &'a WorldSimulation,
    role: &StableId,
    from: GridPos,
) -> Option<&'a BuildingState> {
    simulation
        .buildings
        .values()
        .filter(|building| building.complete)
        .filter(|building| {
            building_def_for_archetype(content, &building.archetype).is_some_and(|definition| {
                definition.role_slots.iter().any(|slot| slot.role == *role)
            })
        })
        .min_by_key(|building| {
            (
                grid_distance_squared(building.position, from),
                building.id.clone(),
            )
        })
}

pub(crate) fn offset_grid(
    position: GridPos,
    x: i32,
    z: i32,
    world: &GeneratedWorld,
) -> Option<GridPos> {
    let x = i32::from(position.x).checked_add(x)?;
    let z = i32::from(position.z).checked_add(z)?;
    (x >= 0
        && z >= 0
        && x < i32::from(world.navigation.width())
        && z < i32::from(world.navigation.height()))
    .then(|| GridPos {
        x: u16::try_from(x).expect("checked world x fits u16"),
        z: u16::try_from(z).expect("checked world z fits u16"),
    })
}

pub(crate) fn active_resource_at(world: &GeneratedWorld, position: GridPos) -> bool {
    world
        .resources
        .iter()
        .any(|resource| resource.amount > 0 && resource.position == position)
}

pub(crate) fn nearby_active_resource_count(
    world: &GeneratedWorld,
    position: GridPos,
    target_kind: &str,
    radius: u16,
) -> u32 {
    let radius_squared = u64::from(radius) * u64::from(radius);
    u32::try_from(
        world
            .resources
            .iter()
            .filter(|resource| {
                resource.amount > 0
                    && resource.target_kind.as_str() == target_kind
                    && grid_distance_squared(resource.position, position) <= radius_squared
            })
            .count(),
    )
    .unwrap_or(u32::MAX)
}

pub(crate) fn accelerate_forester_regeneration_cooldowns(
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    runtime: &mut RegenerationRoleRuntime,
    delta_seconds: f64,
) {
    if delta_seconds <= 0.0 {
        return;
    }
    for actor in simulation
        .actors
        .values()
        .filter(|actor| actor.alive && actor.role.as_str() == "role:forester")
    {
        let Some(worker) = runtime.workers.get_mut(&actor.id) else {
            continue;
        };
        if !worker.initialized || worker.next_ready_seconds <= runtime.elapsed_seconds {
            continue;
        }
        let nearby_trees = nearby_active_resource_count(
            world,
            actor.position,
            "target:tree",
            FORESTER_TREE_TIMER_RADIUS_CELLS,
        );
        let bonus_seconds =
            delta_seconds * f64::from(nearby_trees) * FORESTER_TREE_TIMER_BONUS_PER_TREE;
        worker.next_ready_seconds =
            (worker.next_ready_seconds - bonus_seconds).max(runtime.elapsed_seconds);
    }
}

pub(crate) fn local_resource_saturation_roll(
    runtime: &mut RegenerationRoleRuntime,
    world: &GeneratedWorld,
    actor: &StableId,
    position: GridPos,
    target_kind: &str,
    sequence: u32,
    base_denominator: u64,
    salt: u64,
) -> (u64, u64, u32) {
    let observed = nearby_active_resource_count(
        world,
        position,
        target_kind,
        LOCAL_RESOURCE_SATURATION_RADIUS_CELLS,
    );
    let target = StableId::new(target_kind).expect("static saturation target ID");
    let worker = runtime.workers.entry(actor.clone()).or_default();
    if worker.resource_saturation_target.as_ref() != Some(&target) {
        worker.resource_saturation_target = Some(target);
        worker.resource_saturation_debuff = 0;
    }
    let decayed = worker
        .resource_saturation_debuff
        .saturating_sub(LOCAL_RESOURCE_SATURATION_DECAY_PER_CHECK);
    worker.resource_saturation_debuff = decayed.max(observed);
    let denominator = base_denominator
        .max(1)
        .saturating_mul(u64::from(worker.resource_saturation_debuff).saturating_add(1));
    let roll = seagull_hash(
        world.seed ^ stable_id_hash(actor),
        u64::from(sequence),
        salt,
    );
    (roll, denominator, worker.resource_saturation_debuff)
}

pub(crate) fn squared_distance_from_region(
    position: GridPos,
    region: stream_town_domain::DirtyRegion,
) -> u64 {
    let dx = if position.x < region.min.x {
        region.min.x - position.x
    } else {
        position.x.saturating_sub(region.max.x)
    };
    let dz = if position.z < region.min.z {
        region.min.z - position.z
    } else {
        position.z.saturating_sub(region.max.z)
    };
    u64::from(dx) * u64::from(dx) + u64::from(dz) * u64::from(dz)
}

pub(crate) fn cell_is_clear_of_buildings(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    position: GridPos,
    minimum_distance: u16,
) -> bool {
    let minimum_squared = u64::from(minimum_distance) * u64::from(minimum_distance);
    simulation.buildings.values().all(|building| {
        let Some(definition) = building_def_for_archetype(content, &building.archetype) else {
            return true;
        };
        let footprint = rotated_footprint(definition.footprint, building.rotation_quarter_turns);
        building_region(building.position, footprint, world)
            .is_none_or(|region| squared_distance_from_region(position, region) >= minimum_squared)
    })
}

pub(crate) struct RegenerationSpatialIndex {
    pub(crate) resources: HashSet<GridPos>,
    pub(crate) trees: HashSet<GridPos>,
    pub(crate) actors: HashSet<GridPos>,
    pub(crate) buildings: HashSet<GridPos>,
}

impl RegenerationSpatialIndex {
    pub(crate) fn new(
        content: &ContentCatalog,
        world: &GeneratedWorld,
        simulation: &WorldSimulation,
    ) -> Self {
        Self {
            resources: world
                .resources
                .iter()
                .filter(|resource| resource.amount > 0)
                .map(|resource| resource.position)
                .collect(),
            trees: world
                .resources
                .iter()
                .filter(|resource| {
                    resource.amount > 0 && resource.target_kind.as_str() == "target:tree"
                })
                .map(|resource| resource.position)
                .collect(),
            actors: simulation
                .actors
                .values()
                .filter(|actor| actor.alive)
                .map(|actor| actor.position)
                .collect(),
            buildings: simulation
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
                .flat_map(|region| {
                    (region.min.z..=region.max.z).flat_map(move |z| {
                        (region.min.x..=region.max.x).map(move |x| GridPos { x, z })
                    })
                })
                .collect(),
        }
    }
}

pub(crate) fn offset_grid_unbounded(position: GridPos, x: i32, z: i32) -> Option<GridPos> {
    let x = i32::from(position.x).checked_add(x)?;
    let z = i32::from(position.z).checked_add(z)?;
    (x >= 0 && z >= 0 && x <= i32::from(u16::MAX) && z <= i32::from(u16::MAX)).then(|| GridPos {
        x: u16::try_from(x).expect("checked grid x fits u16"),
        z: u16::try_from(z).expect("checked grid z fits u16"),
    })
}

pub(crate) fn nearest_resource_distance_capped(
    resources: &HashSet<GridPos>,
    position: GridPos,
    maximum_squared: u64,
) -> u64 {
    let radius = i32::try_from(maximum_squared.isqrt()).unwrap_or(i32::MAX);
    let mut nearest = maximum_squared;
    for z in -radius..=radius {
        for x in -radius..=radius {
            let distance = u64::try_from(i64::from(x) * i64::from(x) + i64::from(z) * i64::from(z))
                .unwrap_or(u64::MAX);
            if distance >= nearest {
                continue;
            }
            if offset_grid_unbounded(position, x, z).is_some_and(|cell| resources.contains(&cell)) {
                nearest = distance;
            }
        }
    }
    nearest
}

pub(crate) fn valid_regeneration_cell_indexed(
    world: &GeneratedWorld,
    spatial: &RegenerationSpatialIndex,
    position: GridPos,
) -> bool {
    world.navigation.is_walkable(position)
        && !spatial.resources.contains(&position)
        && !spatial.actors.contains(&position)
        && !spatial.buildings.contains(&position)
}

pub(crate) fn valid_regeneration_cell(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    position: GridPos,
) -> bool {
    valid_regeneration_cell_indexed(
        world,
        &RegenerationSpatialIndex::new(content, world, simulation),
        position,
    )
}

#[cfg(test)]
pub(crate) fn planting_approach(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    plant: GridPos,
    from: GridPos,
) -> Option<GridPos> {
    let actor = StableId::new("system:unreserved-planter").expect("static stable ID");
    planting_approach_with_reservations(
        content,
        simulation,
        world,
        plant,
        from,
        &actor,
        &BTreeMap::new(),
    )
}

pub(crate) fn planting_approach_with_reservations(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    plant: GridPos,
    from: GridPos,
    actor: &StableId,
    approach_reservations: &BTreeMap<GridPos, StableId>,
) -> Option<GridPos> {
    let mut candidates = (-1..=1)
        .flat_map(|z| (-1..=1).map(move |x| (x, z)))
        .filter(|(x, z)| *x != 0 || *z != 0)
        .filter_map(|(x, z)| offset_grid(plant, x, z, world))
        .filter(|candidate| world.navigation.is_walkable(*candidate))
        .filter(|candidate| {
            approach_reservation_available(approach_reservations, actor, *candidate)
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|candidate| {
        (
            grid_distance_squared(*candidate, from),
            candidate.z,
            candidate.x,
        )
    });
    candidates.into_iter().find(|candidate| {
        try_agent_path(
            &world.navigation,
            content,
            simulation,
            &ActorKind::Player,
            from,
            *candidate,
        )
        .is_some()
    })
}

pub(crate) fn stable_id_hash(value: &StableId) -> u64 {
    value
        .as_str()
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
        })
}

pub(crate) fn initial_enemy_repath_delay(config: &GameConfig, actor: &StableId) -> f32 {
    let interval = config.gameplay.repath_interval_seconds.max(0.1);
    let phase =
        f32::from(u16::try_from(stable_id_hash(actor) % 1_000).unwrap_or_default()) / 1_000.0;
    interval * phase
}

pub(crate) fn regenerated_resource_offset(id: &StableId, position: GridPos) -> [i16; 2] {
    let hash = seagull_hash(
        stable_id_hash(id),
        u64::from(position.x) | (u64::from(position.z) << 16),
        0x5245_4752_4F57,
    );
    let component =
        |bits: u64| i16::try_from(bits % 501).expect("hashed resource offset fits i16") - 250;
    [component(hash), component(hash >> 32)]
}

pub(crate) fn forester_uses_random_fallback(
    world_seed: u64,
    actor: &StableId,
    sequence: u32,
) -> bool {
    seagull_hash(
        world_seed ^ stable_id_hash(actor),
        u64::from(sequence),
        0x464F_5245_5354_524E,
    )
    .is_multiple_of(FORESTER_RANDOM_FALLBACK_DENOMINATOR)
}

pub(crate) fn forester_direction_from_nursery(nursery: GridPos, position: GridPos) -> (i8, i8) {
    (
        (i32::from(position.x) - i32::from(nursery.x)).signum() as i8,
        (i32::from(position.z) - i32::from(nursery.z)).signum() as i8,
    )
}

pub(crate) fn grid_distance_cells(left: GridPos, right: GridPos) -> f64 {
    f64::from(left.x.abs_diff(right.x)).hypot(f64::from(left.z.abs_diff(right.z)))
}

pub(crate) fn forester_nursery_distance_utility(nursery: GridPos, position: GridPos) -> f64 {
    let distance = grid_distance_cells(nursery, position);
    if distance <= FORESTER_PREFERRED_NURSERY_DISTANCE_CELLS {
        distance / FORESTER_PREFERRED_NURSERY_DISTANCE_CELLS
    } else {
        (1.0 - (distance - FORESTER_PREFERRED_NURSERY_DISTANCE_CELLS)
            / (f64::from(FORESTER_UTILITY_RADIUS_CELLS)
                - FORESTER_PREFERRED_NURSERY_DISTANCE_CELLS))
            .max(0.0)
    }
}

pub(crate) fn forester_worker_distance_utility(from: GridPos, position: GridPos) -> f64 {
    let distance = grid_distance_cells(from, position);
    (1.0 - distance / (f64::from(FORESTER_UTILITY_RADIUS_CELLS) * 2.0)).max(0.0)
}

pub(crate) fn forester_candidate_utility(
    spatial: &RegenerationSpatialIndex,
    runtime: &RegenerationRoleRuntime,
    reserved_planting_cells: &BTreeSet<GridPos>,
    nursery: GridPos,
    nearest_nursery: GridPos,
    candidate: GridPos,
) -> f64 {
    let maximum_tree_distance_squared =
        u64::try_from(FORESTER_UTILITY_RADIUS_CELLS.pow(2)).expect("positive utility radius");
    let nearest_tree =
        nearest_resource_distance_capped(&spatial.trees, candidate, maximum_tree_distance_squared);
    let tree_proximity = if nearest_tree >= maximum_tree_distance_squared {
        0.0
    } else {
        let nearest_tree =
            u32::try_from(nearest_tree).expect("the capped Forester tree-search distance fits u32");
        1.0 / (1.0 + f64::from(nearest_tree).sqrt())
    };
    let fallen_proximity = runtime
        .recently_fallen_trees
        .iter()
        .rev()
        .take(64)
        .enumerate()
        .map(|(age_rank, fallen)| {
            let recency = 1.0 - f64::from(u32::try_from(age_rank).unwrap_or(64)) / 64.0;
            recency / (1.0 + grid_distance_cells(*fallen, candidate))
        })
        .fold(0.0_f64, f64::max);
    let direction = forester_direction_from_nursery(nursery, candidate);
    let placed_direction_penalty = runtime
        .recent_tree_plantings
        .iter()
        .filter(|planting| {
            planting.nursery == nursery
                && forester_direction_from_nursery(nursery, planting.position) == direction
        })
        .map(|planting| {
            let age = (runtime.elapsed_seconds - planting.planted_at_seconds).max(0.0);
            (1.0 - age / FORESTER_DIRECTION_MEMORY_SECONDS).clamp(0.0, 1.0)
        })
        .sum::<f64>()
        .min(2.0);
    // Treat another Forester's current destination like a just-planted tree.
    // This spreads simultaneous workers before either has had time to finish.
    let reserved_direction_penalty = reserved_planting_cells
        .iter()
        .filter(|position| forester_direction_from_nursery(nursery, **position) == direction)
        .count();
    let reserved_direction_penalty = f64::from(
        u32::try_from(reserved_direction_penalty).expect("reserved Forester destinations fit u32"),
    );

    forester_nursery_distance_utility(nearest_nursery, candidate) * 1_000.0
        + tree_proximity * 600.0
        + fallen_proximity * 1_200.0
        - placed_direction_penalty * 1_400.0
        - reserved_direction_penalty * 2_000.0
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn forester_selection_utility(
    spatial: &RegenerationSpatialIndex,
    runtime: &RegenerationRoleRuntime,
    reserved_planting_cells: &BTreeSet<GridPos>,
    nursery: GridPos,
    nearest_nursery: GridPos,
    from: GridPos,
    world_seed: u64,
    actor: &StableId,
    sequence: u32,
    candidate: GridPos,
) -> f64 {
    let worker_proximity = forester_worker_distance_utility(from, candidate);
    let coordinate =
        u64::from(candidate.x) | (u64::from(candidate.z) << 16) | (u64::from(sequence) << 32);
    let random = f64::from(
        u32::try_from(
            seagull_hash(
                world_seed ^ stable_id_hash(actor),
                coordinate,
                0x464F_5245_5354_5554,
            ) >> 32,
        )
        .expect("upper hash half fits u32"),
    ) / f64::from(u32::MAX);

    forester_candidate_utility(
        spatial,
        runtime,
        reserved_planting_cells,
        nursery,
        nearest_nursery,
        candidate,
    ) + worker_proximity * 800.0
        + random * 1_000.0
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn forester_planting_cell(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    runtime: &RegenerationRoleRuntime,
    actor: &StableId,
    sequence: u32,
    reserved_planting_cells: &BTreeSet<GridPos>,
    approach_reservations: &BTreeMap<GridPos, StableId>,
    hut: GridPos,
    from: GridPos,
) -> Option<(GridPos, GridPos)> {
    let spatial = RegenerationSpatialIndex::new(content, world, simulation);
    if forester_uses_random_fallback(world.seed, actor, sequence)
        && let Some(result) = forester_random_planting_cell(
            content,
            simulation,
            world,
            &spatial,
            actor,
            sequence,
            reserved_planting_cells,
            approach_reservations,
            hut,
            from,
        )
    {
        return Some(result);
    }
    let radius_squared =
        u64::try_from(FORESTER_UTILITY_RADIUS_CELLS.pow(2)).expect("positive utility radius");
    let nursery_role = StableId::new("role:forester").expect("static Forester role ID");
    let nursery_positions = simulation
        .buildings
        .values()
        .filter(|building| building.complete)
        .filter(|building| {
            building_def_for_archetype(content, &building.archetype).is_some_and(|definition| {
                definition
                    .role_slots
                    .iter()
                    .any(|slot| slot.role == nursery_role)
            })
        })
        .map(|building| building.position)
        .collect::<Vec<_>>();
    let mut candidates = (-FORESTER_UTILITY_RADIUS_CELLS..=FORESTER_UTILITY_RADIUS_CELLS)
        .flat_map(|z| {
            (-FORESTER_UTILITY_RADIUS_CELLS..=FORESTER_UTILITY_RADIUS_CELLS).map(move |x| (x, z))
        })
        .filter_map(|(x, z)| offset_grid(hut, x, z, world))
        .filter(|candidate| grid_distance_squared(hut, *candidate) <= radius_squared)
        .filter(|candidate| valid_regeneration_cell_indexed(world, &spatial, *candidate))
        .filter(|candidate| !reserved_planting_cells.contains(candidate))
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        let nearest_nursery = |candidate: GridPos| {
            nursery_positions
                .iter()
                .copied()
                .min_by_key(|position| grid_distance_squared(*position, candidate))
                .unwrap_or(hut)
        };
        let left_utility = forester_selection_utility(
            &spatial,
            runtime,
            reserved_planting_cells,
            hut,
            nearest_nursery(*left),
            from,
            world.seed,
            actor,
            sequence,
            *left,
        );
        let right_utility = forester_selection_utility(
            &spatial,
            runtime,
            reserved_planting_cells,
            hut,
            nearest_nursery(*right),
            from,
            world.seed,
            actor,
            sequence,
            *right,
        );
        right_utility
            .total_cmp(&left_utility)
            .then_with(|| (left.z, left.x).cmp(&(right.z, right.x)))
    });
    if let Some(result) = candidates.into_iter().find_map(|candidate| {
        planting_approach_with_reservations(
            content,
            simulation,
            world,
            candidate,
            from,
            actor,
            approach_reservations,
        )
        .map(|approach| (candidate, approach))
    }) {
        return Some(result);
    }

    forester_random_planting_cell(
        content,
        simulation,
        world,
        &spatial,
        actor,
        sequence,
        reserved_planting_cells,
        approach_reservations,
        hut,
        from,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn forester_random_planting_cell(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    spatial: &RegenerationSpatialIndex,
    actor: &StableId,
    sequence: u32,
    reserved_planting_cells: &BTreeSet<GridPos>,
    approach_reservations: &BTreeMap<GridPos, StableId>,
    hut: GridPos,
    from: GridPos,
) -> Option<(GridPos, GridPos)> {
    let radius = 20_i32;
    let mut fallback = (-radius..=radius)
        .flat_map(|z| (-radius..=radius).map(move |x| (x, z)))
        .filter_map(|(x, z)| offset_grid(hut, x, z, world))
        .filter(|candidate| valid_regeneration_cell_indexed(world, spatial, *candidate))
        .filter(|candidate| !reserved_planting_cells.contains(candidate))
        .filter(|candidate| cell_is_clear_of_buildings(content, simulation, world, *candidate, 3))
        .collect::<Vec<_>>();
    fallback.sort_by_key(|candidate| {
        seagull_hash(
            world.seed ^ stable_id_hash(actor),
            u64::from(candidate.x) | (u64::from(candidate.z) << 16) | (u64::from(sequence) << 32),
            0x4641_4C4C_4241_434B,
        )
    });
    fallback.into_iter().find_map(|candidate| {
        planting_approach_with_reservations(
            content,
            simulation,
            world,
            candidate,
            from,
            actor,
            approach_reservations,
        )
        .map(|approach| (candidate, approach))
    })
}

pub(crate) fn tender_planting_cell(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    actor: &StableId,
    sequence: u32,
    reserved: &BTreeSet<GridPos>,
    approach_reservations: &BTreeMap<GridPos, StableId>,
    hut: GridPos,
    from: GridPos,
) -> Option<(GridPos, GridPos)> {
    let spatial = RegenerationSpatialIndex::new(content, world, simulation);
    let radius = 24_i32;
    let mut fields = (-radius..=radius)
        .flat_map(|z| (-radius..=radius).map(move |x| (x, z)))
        .filter_map(|(x, z)| offset_grid(hut, x, z, world))
        .filter(|candidate| valid_regeneration_cell_indexed(world, &spatial, *candidate))
        .filter(|candidate| cell_is_clear_of_buildings(content, simulation, world, *candidate, 10))
        .filter(|candidate| {
            let nearest_tree = nearest_resource_distance_capped(&spatial.trees, *candidate, 37);
            (9..=36).contains(&nearest_tree)
        })
        .filter(|candidate| !reserved.contains(candidate))
        .collect::<Vec<_>>();
    fields.sort_by_cached_key(|candidate| {
        let coordinate = u64::from(candidate.x) | (u64::from(candidate.z) << 16);
        seagull_hash(
            world.seed ^ stable_id_hash(actor),
            coordinate ^ (u64::from(sequence) << 32),
            0x5445_4E44_4552,
        )
    });
    fields.into_iter().find_map(|candidate| {
        planting_approach_with_reservations(
            content,
            simulation,
            world,
            candidate,
            from,
            actor,
            approach_reservations,
        )
        .map(|approach| (candidate, approach))
    })
}

pub(crate) fn prospector_spiral_cells(world: &GeneratedWorld, hut: GridPos) -> Vec<GridPos> {
    let mut cells = Vec::new();
    for radius in 5_i32..=20 {
        for z in 0..=radius {
            if let Some(cell) = offset_grid(hut, radius, z, world) {
                cells.push(cell);
            }
        }
        for x in (-radius..radius).rev() {
            if let Some(cell) = offset_grid(hut, x, radius, world) {
                cells.push(cell);
            }
        }
        for z in (-radius..radius).rev() {
            if let Some(cell) = offset_grid(hut, -radius, z, world) {
                cells.push(cell);
            }
        }
        for x in (-radius + 1)..=radius {
            if let Some(cell) = offset_grid(hut, x, -radius, world) {
                cells.push(cell);
            }
        }
        for z in (-radius + 1)..0 {
            if let Some(cell) = offset_grid(hut, radius, z, world) {
                cells.push(cell);
            }
        }
    }
    cells.retain(|cell| world.navigation.is_walkable(*cell));
    cells
}

#[cfg(test)]
pub(crate) fn regeneration_agent_goal(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    runtime: &mut RegenerationRoleRuntime,
    actor_id: &StableId,
    current: GridPos,
) -> Option<(AgentGoal, GridPos)> {
    regeneration_agent_goal_with_reservations(
        content,
        simulation,
        world,
        runtime,
        actor_id,
        current,
        &BTreeMap::new(),
    )
}

pub(crate) fn regeneration_agent_goal_with_reservations(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    runtime: &mut RegenerationRoleRuntime,
    actor_id: &StableId,
    current: GridPos,
    approach_reservations: &BTreeMap<GridPos, StableId>,
) -> Option<(AgentGoal, GridPos)> {
    let actor = simulation.actors.get(actor_id)?;
    if !matches!(
        actor.role.as_str(),
        "role:forester" | "role:prospector" | "role:tender"
    ) {
        return None;
    }
    let hut = regeneration_hut(content, simulation, &actor.role, current)?;
    let hut_id = hut.id.clone();
    let hut_position = hut.position;
    let hut_footprint = building_def_for_archetype(content, &hut.archetype)
        .map(|definition| rotated_footprint(definition.footprint, hut.rotation_quarter_turns))?;
    let level = regeneration_role_level(actor);
    if let Some(interval) = community_adjusted_regeneration_interval_seconds(
        &actor.role,
        level,
        simulation.active_community_event,
    ) {
        let worker = runtime.workers.entry(actor_id.clone()).or_default();
        if !worker.initialized {
            worker.initialized = true;
            worker.next_ready_seconds = runtime.elapsed_seconds + interval;
            worker.station_visit_required = true;
            return None;
        }
        if runtime.elapsed_seconds < worker.next_ready_seconds {
            return None;
        }
        // A mature town may have no valid planting cells. Bound the next
        // expensive spatial search even when target selection or path planning
        // fails; successful work replaces this with the full authored interval.
        worker.next_ready_seconds = runtime.elapsed_seconds + REGENERATION_TARGET_RETRY_SECONDS;
    }
    if matches!(actor.role.as_str(), "role:forester" | "role:tender") {
        let worker = runtime.workers.entry(actor_id.clone()).or_default();
        if worker.last_station.as_ref() != Some(&hut_id) {
            worker.station_visit_required = true;
        }
        if worker.station_visit_required {
            return building_approaches(world, hut_position, hut_footprint, current)
                .into_iter()
                .find(|approach| {
                    approach_reservation_available(approach_reservations, actor_id, *approach)
                })
                .map(|approach| {
                    (
                        AgentGoal::VisitRegenerationStation(hut_id.clone()),
                        approach,
                    )
                });
        }
    }
    let reserved_planting_cells = runtime
        .workers
        .iter()
        .filter(|(worker_id, _)| *worker_id != actor_id)
        .filter_map(|(_, worker)| worker.planting_target)
        .collect::<BTreeSet<_>>();
    match actor.role.as_str() {
        "role:forester" => {
            let sequence = {
                let worker = runtime.workers.entry(actor_id.clone()).or_default();
                let sequence = worker.planting_sequence;
                worker.planting_sequence = worker.planting_sequence.wrapping_add(1);
                sequence
            };
            let selected = forester_planting_cell(
                content,
                simulation,
                world,
                runtime,
                actor_id,
                sequence,
                &reserved_planting_cells,
                approach_reservations,
                hut_position,
                current,
            );
            if let Some((plant, approach)) = selected {
                runtime
                    .workers
                    .entry(actor_id.clone())
                    .or_default()
                    .planting_target = Some(plant);
                Some((AgentGoal::PlantTree(plant), approach))
            } else {
                None
            }
        }
        "role:tender" => {
            let sequence = {
                let worker = runtime.workers.entry(actor_id.clone()).or_default();
                let sequence = worker.planting_sequence;
                worker.planting_sequence = worker.planting_sequence.wrapping_add(1);
                sequence
            };
            let (roll, denominator, _) = local_resource_saturation_roll(
                runtime,
                world,
                actor_id,
                current,
                "target:bush",
                sequence,
                1,
                0x5445_4E44_4552_4348,
            );
            if !roll.is_multiple_of(denominator) {
                if let Some(interval) = community_adjusted_regeneration_interval_seconds(
                    &actor.role,
                    level,
                    simulation.active_community_event,
                ) {
                    runtime
                        .workers
                        .entry(actor_id.clone())
                        .or_default()
                        .next_ready_seconds = runtime.elapsed_seconds + interval;
                    runtime
                        .workers
                        .entry(actor_id.clone())
                        .or_default()
                        .station_visit_required = true;
                }
                return None;
            }
            let selected = tender_planting_cell(
                content,
                simulation,
                world,
                actor_id,
                sequence,
                &reserved_planting_cells,
                approach_reservations,
                hut_position,
                current,
            );
            if let Some((plant, approach)) = selected {
                let worker = runtime.workers.entry(actor_id.clone()).or_default();
                worker.planting_target = Some(plant);
                Some((AgentGoal::PlantBush(plant), approach))
            } else {
                None
            }
        }
        "role:prospector" => {
            let cells = prospector_spiral_cells(world, hut_position);
            if cells.is_empty() {
                return None;
            }
            let worker = runtime.workers.entry(actor_id.clone()).or_default();
            worker.initialized = true;
            let sequence = worker.prospector_step;
            worker.prospector_step = worker.prospector_step.wrapping_add(1);
            let cell = cells[usize::try_from(sequence).unwrap_or_default() % cells.len()];
            Some((AgentGoal::Prospect { cell, sequence }, cell))
        }
        _ => None,
    }
}

pub(crate) fn resource_amount_for_target(content: &ContentCatalog, target: &str) -> u32 {
    content
        .resource_generation
        .iter()
        .find(|layer| layer.target_kind.as_str() == target)
        .map_or(1, |layer| layer.amount)
}

pub(crate) fn spawn_regenerated_resource(
    runtime: &mut RegenerationRoleRuntime,
    world: &mut GeneratedWorld,
    content: &ContentCatalog,
    kind: &str,
    target: &str,
    position: GridPos,
) -> bool {
    if active_resource_at(world, position) || !world.navigation.is_walkable(position) {
        return false;
    }
    let id = loop {
        let serial = runtime.next_resource_serial;
        runtime.next_resource_serial = runtime.next_resource_serial.wrapping_add(1);
        let candidate = StableId::new(format!(
            "resource:regrown_{}_{}_{}_{}_{serial:08x}",
            target.trim_start_matches("target:"),
            world.seed,
            position.x,
            position.z,
        ))
        .expect("regrown resource ID is valid");
        if !world
            .resources
            .iter()
            .any(|resource| resource.id == candidate)
        {
            break candidate;
        }
    };
    let offset_milli_cells = regenerated_resource_offset(&id, position);
    world.resources.push(stream_town_domain::GeneratedResource {
        id,
        kind: StableId::new(kind).expect("static regenerated resource kind"),
        target_kind: StableId::new(target).expect("static regenerated target kind"),
        position,
        offset_milli_cells,
        generation_occupancy: [0, 0],
        amount: resource_amount_for_target(content, target),
    });
    let _ = world.navigation.set_blocked(
        stream_town_domain::DirtyRegion {
            min: position,
            max: position,
        },
        true,
    );
    true
}

pub(crate) fn restored_regenerated_resource(
    id: &StableId,
    amount: u32,
) -> Option<stream_town_domain::GeneratedResource> {
    let mut parts = id.as_str().strip_prefix("resource:regrown_")?.split('_');
    let target_name = parts.next()?;
    let _seed = parts.next()?.parse::<u64>().ok()?;
    let x = parts.next()?.parse::<u16>().ok()?;
    let z = parts.next()?.parse::<u16>().ok()?;
    let _serial = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    let (kind, target_kind) = match target_name {
        "tree" => ("resource:wood", "target:tree"),
        "ore" => ("resource:ore", "target:ore"),
        "bush" => ("resource:food", "target:bush"),
        _ => return None,
    };
    let position = GridPos { x, z };
    Some(stream_town_domain::GeneratedResource {
        id: id.clone(),
        kind: StableId::new(kind).expect("static regenerated resource kind"),
        target_kind: StableId::new(target_kind).expect("static regenerated target kind"),
        position,
        offset_milli_cells: regenerated_resource_offset(id, position),
        generation_occupancy: [0, 0],
        amount,
    })
}

pub(crate) fn complete_regeneration_goal(
    simulation: &mut WorldSimulation,
    world: &mut GeneratedWorld,
    content: &ContentCatalog,
    runtime: &mut RegenerationRoleRuntime,
    actor_id: &StableId,
    goal: &AgentGoal,
) -> bool {
    let Some(actor) = simulation.actors.get(actor_id) else {
        return false;
    };
    let level = regeneration_role_level(actor);
    match goal {
        AgentGoal::PlantTree(position) => {
            let planted = valid_regeneration_cell(content, simulation, world, *position)
                && spawn_regenerated_resource(
                    runtime,
                    world,
                    content,
                    "resource:wood",
                    "target:tree",
                    *position,
                );
            if planted
                && let Some(nursery) =
                    regeneration_hut(content, simulation, &actor.role, actor.position)
            {
                runtime.recent_tree_plantings.push_back(RecentTreePlanting {
                    nursery: nursery.position,
                    position: *position,
                    planted_at_seconds: runtime.elapsed_seconds,
                });
                while runtime.recent_tree_plantings.len() > 128 {
                    runtime.recent_tree_plantings.pop_front();
                }
            }
            if let Some(interval) = community_adjusted_regeneration_interval_seconds(
                &actor.role,
                level,
                simulation.active_community_event,
            ) {
                let worker = runtime.workers.entry(actor_id.clone()).or_default();
                worker.next_ready_seconds = runtime.elapsed_seconds + interval;
                worker.planting_target = None;
                worker.station_visit_required = true;
            }
            planted
        }
        AgentGoal::PlantBush(position) => {
            let planted = valid_regeneration_cell(content, simulation, world, *position)
                && spawn_regenerated_resource(
                    runtime,
                    world,
                    content,
                    "resource:food",
                    "target:bush",
                    *position,
                );
            if let Some(interval) = community_adjusted_regeneration_interval_seconds(
                &actor.role,
                level,
                simulation.active_community_event,
            ) {
                let worker = runtime.workers.entry(actor_id.clone()).or_default();
                worker.next_ready_seconds = runtime.elapsed_seconds + interval;
                worker.planting_target = None;
                worker.station_visit_required = true;
            }
            planted
        }
        AgentGoal::Prospect { cell, sequence } => {
            let (roll, denominator, _) = local_resource_saturation_roll(
                runtime,
                world,
                actor_id,
                actor.position,
                "target:ore",
                *sequence,
                prospector_discovery_denominator(level),
                0x5052_4F53_5045_4354,
            );
            if !roll.is_multiple_of(denominator) {
                return true;
            }
            let count = 3 + usize::try_from((roll >> 32) % 3).expect("ore count fits usize");
            let offsets = [
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
                (1, 1),
                (-1, 1),
                (-1, -1),
                (1, -1),
                (2, 0),
                (0, 2),
                (-2, 0),
                (0, -2),
            ];
            let mut spawned = 0;
            for (x, z) in offsets {
                let Some(position) = offset_grid(*cell, x, z, world) else {
                    continue;
                };
                if valid_regeneration_cell(content, simulation, world, position)
                    && spawn_regenerated_resource(
                        runtime,
                        world,
                        content,
                        "resource:ore",
                        "target:ore",
                        position,
                    )
                {
                    spawned += 1;
                    if spawned >= count {
                        break;
                    }
                }
            }
            true
        }
        _ => false,
    }
}
