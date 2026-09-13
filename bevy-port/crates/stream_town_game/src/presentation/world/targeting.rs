pub(crate) fn cached_station_targets<'a>(
    runtime: &'a StationTargetRuntime,
    station: StationCandidate<'_>,
    kind: &StableId,
) -> &'a [StableId] {
    runtime
        .stations
        .get(station.id)
        .and_then(|cache| cache.targets.get(kind))
        .map(Vec::as_slice)
        .unwrap_or_default()
}

pub(crate) fn within_player_target_search_region(position: GridPos, player: GridPos) -> bool {
    position.x.abs_diff(player.x) <= PLAYER_TARGET_SEARCH_RANGE_CELLS
        && position.z.abs_diff(player.z) <= PLAYER_TARGET_SEARCH_RANGE_CELLS
}

pub(crate) fn building_def_for_archetype<'a>(
    content: &'a ContentCatalog,
    archetype: &StableId,
) -> Option<&'a BuildingDef> {
    content
        .buildings
        .values()
        .find(|building| building.archetype == *archetype)
}

pub(crate) fn is_farm_resource_building(
    content: &ContentCatalog,
    building: &BuildingState,
) -> bool {
    building.complete
        && content
            .buildings
            .get(&StableId::new("building:farm").expect("static building ID"))
            .is_some_and(|farm| farm.archetype == building.archetype)
}

pub(crate) fn resource_approach(
    world: &GeneratedWorld,
    resource: &stream_town_domain::GeneratedResource,
    from: GridPos,
) -> Option<GridPos> {
    if world.navigation.is_walkable(resource.position) {
        return Some(resource.position);
    }
    if resource.target_kind.as_str() == "target:fish" {
        return stream_town_domain::shoreline_approaches(&world.navigation, resource.position)
            .min_by_key(|candidate| {
                (
                    candidate.x.abs_diff(from.x) + candidate.z.abs_diff(from.z),
                    candidate.z,
                    candidate.x,
                )
            });
    }
    let radius = 1_u16;
    let min_x = resource.position.x.saturating_sub(radius);
    let max_x = resource
        .position
        .x
        .saturating_add(radius)
        .min(world.navigation.width() - 1);
    let min_z = resource.position.z.saturating_sub(radius);
    let max_z = resource
        .position
        .z
        .saturating_add(radius)
        .min(world.navigation.height() - 1);
    let mut approaches = Vec::new();
    for z in min_z..=max_z {
        for x in min_x..=max_x {
            let candidate = GridPos { x, z };
            if candidate != resource.position && world.navigation.is_walkable(candidate) {
                approaches.push(candidate);
            }
        }
    }
    approaches.sort_by_key(|candidate| {
        (
            candidate.x.abs_diff(from.x) + candidate.z.abs_diff(from.z),
            candidate.x.abs_diff(resource.position.x) + candidate.z.abs_diff(resource.position.z),
            candidate.z,
            candidate.x,
        )
    });
    approaches
        .into_iter()
        .next()
        .or_else(|| nearest_walkable(world, resource.position))
}

pub(crate) fn is_current_building_approach(
    world: &GeneratedWorld,
    content: &ContentCatalog,
    building: &BuildingState,
    current: GridPos,
) -> bool {
    building_def_for_archetype(content, &building.archetype).and_then(|definition| {
        building_approach(
            world,
            building.position,
            rotated_footprint(definition.footprint, building.rotation_quarter_turns),
            current,
        )
    }) == Some(current)
}

pub(crate) fn goal_reservation(goal: &AgentGoal) -> Option<&StableId> {
    match goal {
        // Unity's zero-assignment claim is part of the data-driven generated
        // resource pipeline. Targetable farm holders continue to use their
        // station's authored max-target capacity.
        AgentGoal::Gather(target) => Some(target),
        _ => None,
    }
}

pub(crate) fn goal_assignment_target(goal: &AgentGoal) -> Option<&StableId> {
    match goal {
        AgentGoal::HarvestFarm(target) => Some(target),
        _ => None,
    }
}

pub(crate) fn goal_planting_target(goal: &AgentGoal) -> Option<GridPos> {
    match goal {
        AgentGoal::PlantTree(target) | AgentGoal::PlantBush(target) => Some(*target),
        _ => None,
    }
}

pub(crate) fn goal_reserves_approach(goal: &AgentGoal) -> bool {
    matches!(
        goal,
        AgentGoal::HarvestFarm(_)
            | AgentGoal::Construct(_)
            | AgentGoal::Deposit
            | AgentGoal::VisitRegenerationStation(_)
            | AgentGoal::PlantTree(_)
            | AgentGoal::Prospect { .. }
            | AgentGoal::PlantBush(_)
            | AgentGoal::Wander
    )
}

pub(crate) fn goal_uses_idle_approach_reservation(goal: &AgentGoal) -> bool {
    matches!(goal, AgentGoal::Wander)
}

pub(crate) fn approach_reservation_available(
    reservations: &BTreeMap<GridPos, StableId>,
    actor: &StableId,
    target: GridPos,
) -> bool {
    reservations.get(&target).is_none_or(|owner| owner == actor)
}

pub(crate) fn reservation_available(
    reservations: &BTreeMap<StableId, StableId>,
    actor: &StableId,
    target: &StableId,
) -> bool {
    reservations.get(target).is_none_or(|owner| owner == actor)
}

pub(crate) fn goal_reservation_is_valid(
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    content: &ContentCatalog,
    actor_id: &StableId,
    goal: &AgentGoal,
) -> bool {
    let Some(actor) = simulation.actors.get(actor_id).filter(|actor| actor.alive) else {
        return false;
    };
    match goal {
        AgentGoal::Gather(resource_id) => world
            .resources
            .iter()
            .find(|resource| resource.id == *resource_id && resource.amount > 0)
            .is_some_and(|resource| actor_accepts_resource(content, actor, resource)),
        AgentGoal::HarvestFarm(building_id) => {
            content.roles.get(&actor.role).is_some_and(|role| {
                role_accepts_target(
                    role,
                    &StableId::new("target:farm").expect("static target ID"),
                )
            }) && simulation
                .buildings
                .get(building_id)
                .is_some_and(|building| is_farm_resource_building(content, building))
        }
        _ => false,
    }
}

pub(crate) fn building_max_health_at_level(
    content: &ContentCatalog,
    archetype: &StableId,
    level: u16,
) -> u32 {
    let base_health = content
        .archetypes
        .get(archetype)
        .and_then(|definition| definition.health.as_ref())
        .expect("validated building archetypes have authored health")
        .max_health;
    let definition = building_def_for_archetype(content, archetype)
        .expect("validated building archetypes have a building definition");
    leveled_percentage_stat(
        base_health,
        definition.health_bonus_per_level_per_thousand,
        level,
    )
}

pub(crate) fn building_max_health(content: &ContentCatalog, building: &BuildingState) -> i32 {
    i32::try_from(building_max_health_at_level(
        content,
        &building.archetype,
        building.level,
    ))
    .unwrap_or(i32::MAX)
}

pub(crate) fn building_base_max_health(content: &ContentCatalog, definition: &BuildingDef) -> u32 {
    building_max_health_at_level(content, &definition.archetype, 1)
}

pub(crate) fn normalize_building_health(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
) {
    for building in simulation.buildings.values_mut() {
        let Some(definition) = building_def_for_archetype(content, &building.archetype) else {
            continue;
        };
        let Some(base_health) = content
            .archetypes
            .get(&building.archetype)
            .and_then(|archetype| archetype.health.as_ref())
            .map(|health| health.max_health)
        else {
            continue;
        };
        let max_health = i32::try_from(leveled_percentage_stat(
            base_health,
            definition.health_bonus_per_level_per_thousand,
            building.level,
        ))
        .unwrap_or(i32::MAX);
        building.health = building.health.clamp(0, max_health);
        if !building.complete && building.health >= max_health {
            building.complete = true;
        }
    }
}

pub(crate) fn building_visual_grid(content: &ContentCatalog, building: &BuildingState) -> GridPos {
    let footprint = building_def_for_archetype(content, &building.archetype)
        .map_or([1, 1], |definition| {
            rotated_footprint(definition.footprint, building.rotation_quarter_turns)
        });
    GridPos {
        x: building.position.x.saturating_add(footprint[0] / 2),
        z: building.position.z.saturating_add(footprint[1] / 2),
    }
}

pub(crate) fn building_approach(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
    from: GridPos,
) -> Option<GridPos> {
    building_approaches(world, position, footprint, from)
        .into_iter()
        .next()
}

pub(crate) fn building_approaches(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
    from: GridPos,
) -> Vec<GridPos> {
    building_perimeter_candidates(world, position, footprint, from)
        .into_iter()
        .filter(|candidate| world.navigation.is_walkable(*candidate))
        .collect()
}

pub(crate) fn building_perimeter_candidates(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
    from: GridPos,
) -> Vec<GridPos> {
    let Some(region) = building_region(position, footprint, world) else {
        return Vec::new();
    };
    let min_x = region.min.x.saturating_sub(1);
    let min_z = region.min.z.saturating_sub(1);
    let max_x = region
        .max
        .x
        .saturating_add(1)
        .min(world.navigation.width() - 1);
    let max_z = region
        .max
        .z
        .saturating_add(1)
        .min(world.navigation.height() - 1);
    let mut approaches = Vec::new();
    for z in min_z..=max_z {
        for x in min_x..=max_x {
            let candidate = GridPos { x, z };
            let outside =
                x < region.min.x || x > region.max.x || z < region.min.z || z > region.max.z;
            if outside {
                approaches.push(candidate);
            }
        }
    }
    approaches.sort_by_key(|candidate| {
        (
            candidate.x.abs_diff(from.x) + candidate.z.abs_diff(from.z),
            candidate.z,
            candidate.x,
        )
    });
    approaches
}

pub(crate) fn unoccupied_building_approach(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
    from: GridPos,
    actor: &StableId,
    occupied: &BTreeMap<GridPos, StableId>,
    reservations: &BTreeMap<GridPos, StableId>,
) -> Option<GridPos> {
    building_approaches(world, position, footprint, from)
        .into_iter()
        .find(|candidate| {
            occupied.get(candidate).is_none_or(|owner| owner == actor)
                && reservations
                    .get(candidate)
                    .is_none_or(|owner| owner == actor)
        })
}
pub(crate) fn unoccupied_construction_approach(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
    from: GridPos,
    actor: &StableId,
    occupied: &BTreeMap<GridPos, StableId>,
    reservations: &BTreeMap<GridPos, StableId>,
) -> Option<GridPos> {
    // Coarse navigation marks a whole placement cell blocked when any building
    // occupies it. Construction uses the fine thirds grid, where part of that
    // same cell can still be open and reachable around a dense floorplan.
    building_perimeter_candidates(world, position, footprint, from)
        .into_iter()
        .find(|candidate| {
            occupied.get(candidate).is_none_or(|owner| owner == actor)
                && reservations
                    .get(candidate)
                    .is_none_or(|owner| owner == actor)
        })
}

#[cfg(test)]
pub(crate) fn next_agent_goal(
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    content: &ContentCatalog,
    actor_id: &StableId,
    current: GridPos,
) -> (AgentGoal, GridPos) {
    let station_targets = immediate_station_target_runtime(content, simulation, world, config);
    next_agent_goal_with_station_runtime(
        simulation,
        world,
        None,
        config,
        content,
        &station_targets,
        actor_id,
        current,
        placement_to_navigation_centre(current),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
    )
}

#[cfg(test)]
pub(crate) fn next_agent_goal_with_reservations(
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    content: &ContentCatalog,
    actor_id: &StableId,
    current: GridPos,
    reservations: &BTreeMap<StableId, StableId>,
    target_assignments: &BTreeMap<StableId, u32>,
) -> (AgentGoal, GridPos) {
    let station_targets = immediate_station_target_runtime(content, simulation, world, config);
    next_agent_goal_with_station_runtime(
        simulation,
        world,
        None,
        config,
        content,
        &station_targets,
        actor_id,
        current,
        placement_to_navigation_centre(current),
        reservations,
        target_assignments,
        &BTreeMap::new(),
        &BTreeMap::new(),
    )
}

pub(crate) fn town_hall_wait_target(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    current: GridPos,
) -> GridPos {
    let town_hall = StableId::new("building:townhall").expect("static building ID");
    simulation
        .buildings
        .get(&town_hall)
        .and_then(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            building_approach(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                current,
            )
        })
        .or_else(|| {
            nearest_walkable(
                world,
                restored_town_hall_position(content, simulation, config),
            )
        })
        .unwrap_or(current)
}

pub(crate) fn nearest_reachable_building_to_town_hall(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    current: GridPos,
) -> Option<(StableId, GridPos)> {
    let town_hall = StableId::new("building:townhall").expect("static building ID");
    let town_hall_position = simulation
        .buildings
        .get(&town_hall)
        .map_or(current, |building| building_visual_grid(content, building));
    let mut candidates = simulation
        .buildings
        .values()
        .filter(|building| building.health > 0)
        .filter_map(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            if !building_blocks_navigation(definition) {
                return None;
            }
            Some((
                grid_distance_squared(building_visual_grid(content, building), town_hall_position),
                building.id.clone(),
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
            ))
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|(town_hall_distance, id, _, _)| (*town_hall_distance, id.clone()));
    candidates
        .into_iter()
        .find_map(|(_, id, position, footprint)| {
            reachable_building_approach(world, position, footprint, current)
                .map(|approach| (id, approach))
        })
}

pub(crate) fn enemy_navigation_can_reach(
    world: &GeneratedWorld,
    current: GridPos,
    goal: GridPos,
) -> bool {
    let exceptions = if world.navigation.is_walkable(current) {
        HashSet::new()
    } else {
        HashSet::from([current])
    };
    world
        .navigation
        .find_path_with_exceptions(current, goal, &exceptions)
        .is_ok()
}

pub(crate) fn reachable_building_approach(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
    current: GridPos,
) -> Option<GridPos> {
    building_approaches(world, position, footprint, current)
        .into_iter()
        .find(|approach| enemy_navigation_can_reach(world, current, *approach))
}

pub(crate) fn enemy_route_buildings(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    navigation: &stream_town_domain::NavGrid,
) -> Vec<EnemyRouteBuilding> {
    let town_hall = StableId::new("building:townhall").expect("static building ID");
    let town_hall_position = simulation
        .buildings
        .get(&town_hall)
        .map_or(GridPos { x: 0, z: 0 }, |building| {
            building_visual_grid(content, building)
        });
    simulation
        .buildings
        .values()
        .filter(|building| building.health > 0)
        .filter_map(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            if !building_blocks_navigation(definition) {
                return None;
            }
            let approaches = building_approaches(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                town_hall_position,
            )
            .into_iter()
            .map(placement_to_navigation_centre)
            .filter(|approach| navigation.is_walkable(*approach))
            .collect::<Vec<_>>();
            (!approaches.is_empty()).then(|| EnemyRouteBuilding {
                id: building.id.clone(),
                town_hall_distance: grid_distance_squared(
                    building_visual_grid(content, building),
                    town_hall_position,
                ),
                approaches,
            })
        })
        .collect()
}

pub(crate) fn enemy_navigation_signature(
    navigation: &stream_town_domain::NavGrid,
    buildings: &[EnemyRouteBuilding],
) -> u64 {
    fn fold_u64(mut hash: u64, value: u64) -> u64 {
        for byte in value.to_le_bytes() {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    let mut hash = fold_u64(0xcbf2_9ce4_8422_2325, u64::from(navigation.width()));
    hash = fold_u64(hash, u64::from(navigation.height()));
    hash = fold_u64(hash, navigation.topology_signature());
    for building in buildings {
        hash = fold_u64(hash, stable_id_hash(&building.id));
        hash = fold_u64(hash, building.town_hall_distance);
        for approach in &building.approaches {
            hash = fold_u64(hash, u64::from(approach.x) | (u64::from(approach.z) << 16));
        }
    }
    hash
}

pub(crate) fn grid_octile_distance(left: GridPos, right: GridPos) -> u32 {
    let x = u32::from(left.x.abs_diff(right.x));
    let z = u32::from(left.z.abs_diff(right.z));
    let diagonal = x.min(z);
    diagonal * 14 + (x.max(z) - diagonal) * 10
}

pub(crate) fn build_enemy_navigation_field(
    signature: u64,
    navigation: stream_town_domain::NavGrid,
    buildings: Vec<EnemyRouteBuilding>,
) -> EnemyNavigationField {
    const BLOCKED_COMPONENT: u32 = u32::MAX;

    let width = navigation.width();
    let height = navigation.height();
    let cell_count = usize::from(width) * usize::from(height);
    let index =
        |position: GridPos| usize::from(position.z) * usize::from(width) + usize::from(position.x);
    let mut component_by_cell = vec![BLOCKED_COMPONENT; cell_count];
    let mut component_count = 0_u32;
    for z in 0..height {
        for x in 0..width {
            let start = GridPos { x, z };
            if !navigation.is_walkable(start)
                || component_by_cell[index(start)] != BLOCKED_COMPONENT
            {
                continue;
            }
            let component = component_count;
            component_count = component_count.saturating_add(1);
            component_by_cell[index(start)] = component;
            let mut queue = VecDeque::from([start]);
            while let Some(current) = queue.pop_front() {
                for (neighbour, _) in navigation
                    .walkable_neighbours(current)
                    .into_iter()
                    .flatten()
                {
                    let neighbour_index = index(neighbour);
                    if component_by_cell[neighbour_index] == BLOCKED_COMPONENT {
                        component_by_cell[neighbour_index] = component;
                        queue.push_back(neighbour);
                    }
                }
            }
        }
    }

    let component_count = usize::try_from(component_count).unwrap_or(0);
    let mut selected = vec![None::<(u64, StableId)>; component_count];
    for building in &buildings {
        for approach in &building.approaches {
            let component = component_by_cell[index(*approach)];
            if component == BLOCKED_COMPONENT {
                continue;
            }
            let candidate = (building.town_hall_distance, building.id.clone());
            let selected = &mut selected[usize::try_from(component).expect("component fits usize")];
            if selected.as_ref().is_none_or(|current| candidate < *current) {
                *selected = Some(candidate);
            }
        }
    }
    let target_by_component = selected
        .iter()
        .map(|candidate| candidate.as_ref().map(|(_, id)| id.clone()))
        .collect::<Vec<_>>();
    let mut next_by_cell = vec![None; cell_count];
    let mut goal_by_cell = vec![None; cell_count];
    let mut route_costs = vec![u32::MAX; cell_count];
    let mut queue = BinaryHeap::new();
    for building in &buildings {
        for approach in &building.approaches {
            let approach_index = index(*approach);
            let component = component_by_cell[approach_index];
            if component == BLOCKED_COMPONENT
                || target_by_component[usize::try_from(component).expect("component fits usize")]
                    .as_ref()
                    != Some(&building.id)
                || goal_by_cell[approach_index].is_some()
            {
                continue;
            }
            goal_by_cell[approach_index] = Some(*approach);
            route_costs[approach_index] = 0;
            queue.push(EnemyPathOpenNode {
                position: *approach,
                estimated_total: 0,
                cost: 0,
            });
        }
    }
    while let Some(current) = queue.pop() {
        let current_index = index(current.position);
        if current.cost > route_costs[current_index] {
            continue;
        }
        let component = component_by_cell[current_index];
        let goal = goal_by_cell[current_index].expect("queued route cells have a goal");
        for (neighbour, step_cost) in navigation
            .walkable_neighbours(current.position)
            .into_iter()
            .flatten()
        {
            let neighbour_index = index(neighbour);
            if component_by_cell[neighbour_index] != component {
                continue;
            }
            let next_cost = current.cost.saturating_add(step_cost);
            if next_cost < route_costs[neighbour_index] {
                route_costs[neighbour_index] = next_cost;
                next_by_cell[neighbour_index] = Some(current.position);
                goal_by_cell[neighbour_index] = Some(goal);
                queue.push(EnemyPathOpenNode {
                    position: neighbour,
                    estimated_total: next_cost,
                    cost: next_cost,
                });
            }
        }
    }

    let mut cluster_edge_sets = BTreeMap::<EnemyClusterNode, BTreeSet<EnemyClusterNode>>::new();
    for z in 0..height {
        for x in 0..width {
            let current = GridPos { x, z };
            let current_index = index(current);
            let component = component_by_cell[current_index];
            if component == BLOCKED_COMPONENT {
                continue;
            }
            let current_cluster = EnemyClusterNode {
                x: x / ENEMY_NAVIGATION_CLUSTER_SIZE,
                z: z / ENEMY_NAVIGATION_CLUSTER_SIZE,
                component,
            };
            cluster_edge_sets.entry(current_cluster).or_default();
            for (neighbour, _) in navigation
                .walkable_neighbours(current)
                .into_iter()
                .flatten()
            {
                let neighbour_component = component_by_cell[index(neighbour)];
                if neighbour_component != component {
                    continue;
                }
                let neighbour_cluster = EnemyClusterNode {
                    x: neighbour.x / ENEMY_NAVIGATION_CLUSTER_SIZE,
                    z: neighbour.z / ENEMY_NAVIGATION_CLUSTER_SIZE,
                    component,
                };
                if neighbour_cluster != current_cluster {
                    cluster_edge_sets
                        .entry(current_cluster)
                        .or_default()
                        .insert(neighbour_cluster);
                    cluster_edge_sets
                        .entry(neighbour_cluster)
                        .or_default()
                        .insert(current_cluster);
                }
            }
        }
    }
    let cluster_edges = cluster_edge_sets
        .into_iter()
        .map(|(cluster, neighbours)| (cluster, neighbours.into_iter().collect()))
        .collect();

    EnemyNavigationField {
        signature,
        width,
        component_by_cell,
        target_by_component,
        next_by_cell,
        goal_by_cell,
        cluster_edges,
    }
}

pub(crate) fn refresh_enemy_navigation(
    world: Res<WorldRuntime>,
    simulation: Res<SimulationRuntime>,
    content: Res<RuntimeContent>,
    fine_navigation: Res<FineNavigationRuntime>,
    mut runtime: ResMut<EnemyNavigationRuntime>,
) {
    let Some(navigation) = fine_navigation.grid.as_ref() else {
        return;
    };
    let buildings = enemy_route_buildings(&content.0, &simulation.0, &world.generated, navigation);
    let signature = enemy_navigation_signature(navigation, &buildings);
    let completed = runtime
        .task
        .as_mut()
        .and_then(|pending| block_on(poll_once(&mut pending.task)));
    if let Some(field) = completed {
        runtime.task = None;
        if field.signature == signature {
            runtime.field = Some(field);
        }
    }
    if runtime
        .field
        .as_ref()
        .is_some_and(|field| field.signature == signature)
        || runtime
            .task
            .as_ref()
            .is_some_and(|pending| pending.signature == signature)
    {
        return;
    }
    if runtime.task.is_some() {
        return;
    }

    let navigation = navigation.clone();
    if runtime.field.is_none() {
        runtime.field = Some(build_enemy_navigation_field(
            signature, navigation, buildings,
        ));
    } else if let Some(pool) = AsyncComputeTaskPool::try_get() {
        runtime.task = Some(EnemyNavigationTask {
            signature,
            task: pool.spawn(async move {
                build_enemy_navigation_field(signature, navigation, buildings)
            }),
        });
    } else {
        runtime.field = Some(build_enemy_navigation_field(
            signature, navigation, buildings,
        ));
    }
}
