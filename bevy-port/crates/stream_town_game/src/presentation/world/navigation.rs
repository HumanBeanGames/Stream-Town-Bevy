pub(crate) fn best_tower_target(
    simulation: &WorldSimulation,
    centre: GridPos,
    range_milli_cells: u32,
) -> Option<&ActorState> {
    simulation
        .actors
        .values()
        .filter(|actor| actor.alive && actor.role.as_str() == "role:enemy")
        .filter(|actor| within_milli_cell_range(actor.position, centre, range_milli_cells))
        .min_by_key(|actor| {
            (
                grid_distance_squared(actor.position, centre),
                actor.id.clone(),
            )
        })
}

pub(crate) fn tower_projectile_for_building(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building_id: &StableId,
) -> Option<(ProjectileSpawn, f32)> {
    let state = simulation
        .buildings
        .get(building_id)
        .filter(|building| building.complete && building.health > 0)?;
    let definition = building_def_for_archetype(content, &state.archetype)?;
    let shooter = definition.projectile_shooter.as_ref()?;
    let footprint = rotated_footprint(definition.footprint, state.rotation_quarter_turns);
    let centre = GridPos {
        x: state.position.x.saturating_add(footprint[0] / 2),
        z: state.position.z.saturating_add(footprint[1] / 2),
    };
    let target = best_tower_target(simulation, centre, shooter.range_milli_cells)?;
    Some((
        ProjectileSpawn {
            source: ProjectileSource::Building(building_id.clone()),
            target: target.id.clone(),
            damage: leveled_percentage_stat(
                shooter.damage,
                shooter.damage_bonus_per_level_per_thousand,
                state.level,
            ),
            speed_cells_per_second: milli_units_as_f32(shooter.movement_milli_cells_per_second),
            visual: CombatVisualKind::Arrow,
        },
        milli_units_as_f32(shooter.fire_milliseconds),
    ))
}

pub(crate) fn update_tower_shooters(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    render: Res<RenderAssets>,
    simulation: Res<SimulationRuntime>,
    mut towers: Query<(&RuntimeBuilding, &Transform, &mut TowerShooter)>,
) {
    for (runtime, transform, mut tower) in &mut towers {
        tower.cooldown_seconds = (tower.cooldown_seconds - time.delta_secs()).max(0.0);
        if tower.cooldown_seconds > f32::EPSILON {
            continue;
        }
        let Some((projectile, cooldown_seconds)) =
            tower_projectile_for_building(&content.0, &simulation.0, &runtime.id)
        else {
            continue;
        };
        spawn_combat_projectile(
            &mut commands,
            &render,
            &config.0,
            transform.translation + Vec3::Y * config.0.world.cell_size * 0.5,
            projectile,
        );
        tower.cooldown_seconds = cooldown_seconds;
    }
}

pub(crate) fn action_cooldown(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &StableId,
    goal: &AgentGoal,
) -> f32 {
    let fallback = match goal {
        AgentGoal::Attack(_)
        | AgentGoal::AttackBuilding(_)
        | AgentGoal::Heal(_)
        | AgentGoal::PlantTree(_)
        | AgentGoal::Prospect { .. }
        | AgentGoal::PlantBush(_) => 1.0,
        AgentGoal::Construct(_) => 0.5,
        AgentGoal::VisitRegenerationStation(_) => 0.1,
        AgentGoal::Gather(_) | AgentGoal::HarvestFarm(_) => 0.75,
        // `PlayerInventory.DepositResources` waits 2.5 seconds before its
        // transfer callback. Keep the complete authored station cadence.
        AgentGoal::Deposit => 2.5,
        AgentGoal::WaitForStorage | AgentGoal::Wander => 0.0,
    };
    let Some(actor) = simulation.actors.get(actor) else {
        return fallback;
    };
    if matches!(goal, AgentGoal::Attack(_) | AgentGoal::AttackBuilding(_))
        && let Some(enemy) =
            actor_archetype(content, actor).and_then(|archetype| archetype.enemy.as_ref())
    {
        return milli_units_as_f32(enemy.action_milliseconds).max(0.1);
    }
    let base = if matches!(
        goal,
        AgentGoal::Attack(_)
            | AgentGoal::AttackBuilding(_)
            | AgentGoal::Construct(_)
            | AgentGoal::Gather(_)
            | AgentGoal::HarvestFarm(_)
            | AgentGoal::Heal(_)
            | AgentGoal::PlantTree(_)
            | AgentGoal::Prospect { .. }
            | AgentGoal::PlantBush(_)
    ) {
        effective_role_stats(content, simulation, actor).map_or(fallback, |stats| {
            milli_units_as_f32(stats.action_milliseconds).max(0.1)
        })
    } else {
        fallback
    };
    let mut rate_multiplier = if matches!(goal, AgentGoal::PlantTree(_) | AgentGoal::PlantBush(_)) {
        1.0
    } else {
        simulation.active_community_event.map_or(1.0, |event| {
            community_event_role_rate_multiplier(event, &actor.role)
        })
    };
    if simulation.active_community_event == Some(CommunityEvent::EconomicBoom)
        && matches!(goal, AgentGoal::Gather(_) | AgentGoal::HarvestFarm(_))
    {
        rate_multiplier *= 1.1;
    }
    let base = base / rate_multiplier.max(f32::EPSILON);
    if base <= f32::EPSILON {
        0.0
    } else {
        base.max(0.1)
    }
}

pub(crate) fn idle_wander_replan_delay(kind: &ActorKind, goal: &AgentGoal) -> Option<f32> {
    (*kind == ActorKind::Player && *goal == AgentGoal::Wander)
        .then_some(UNITY_IDLE_WANDER_INTERVAL_SECONDS)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ArrivedActionPhase {
    NonAction,
    Started,
    Waiting,
    Complete,
}

pub(crate) fn arrived_action_phase(agent: &mut Agent, authored_delay: f32) -> ArrivedActionPhase {
    if !agent.action_started {
        if authored_delay <= f32::EPSILON {
            return ArrivedActionPhase::NonAction;
        }
        agent.action_started = true;
        agent.action_cooldown_seconds = authored_delay;
        return ArrivedActionPhase::Started;
    }
    if agent.action_cooldown_seconds > f32::EPSILON {
        ArrivedActionPhase::Waiting
    } else {
        ArrivedActionPhase::Complete
    }
}

pub(crate) fn completed_player_gate_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> HashSet<GridPos> {
    let Some(gate) = content
        .buildings
        .get(&StableId::new("building:gate").expect("static ID"))
    else {
        return HashSet::new();
    };
    simulation
        .buildings
        .values()
        .filter(|building| building.complete && building.archetype == gate.archetype)
        .flat_map(|building| {
            let footprint = rotated_footprint(gate.footprint, building.rotation_quarter_turns);
            (0..footprint[1]).flat_map(move |z| {
                (0..footprint[0]).map(move |x| GridPos {
                    x: building.position.x.saturating_add(x),
                    z: building.position.z.saturating_add(z),
                })
            })
        })
        .collect()
}

pub(crate) fn completed_player_gate_navigation_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> HashSet<GridPos> {
    let gate_id = StableId::new("building:gate").expect("static ID");
    let Some(gate) = content.buildings.get(&gate_id) else {
        return HashSet::new();
    };
    simulation
        .buildings
        .values()
        .filter(|building| building.complete && building.archetype == gate.archetype)
        .flat_map(|building| {
            linear_navigation_cells(content, simulation, building, &gate_id).into_iter()
        })
        .collect()
}

#[cfg(test)]
pub(crate) fn agent_path(
    navigation: &stream_town_domain::NavGrid,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    kind: &ActorKind,
    start: GridPos,
    goal: GridPos,
) -> Vec<GridPos> {
    try_agent_path(navigation, content, simulation, kind, start, goal)
        .unwrap_or_else(|| vec![start])
}

pub(crate) fn try_agent_path(
    navigation: &stream_town_domain::NavGrid,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    kind: &ActorKind,
    start: GridPos,
    goal: GridPos,
) -> Option<Vec<GridPos>> {
    let path = if *kind == ActorKind::Player {
        navigation.find_path_with_exceptions(
            start,
            goal,
            &completed_player_gate_cells(content, simulation),
        )
    } else {
        navigation.find_path(start, goal)
    };
    path.ok()
}

pub(crate) fn fine_navigation_goal_candidates(placement: GridPos, start: GridPos) -> Vec<GridPos> {
    let base = GridPos {
        x: placement.x.saturating_mul(NAVIGATION_SUBDIVISIONS),
        z: placement.z.saturating_mul(NAVIGATION_SUBDIVISIONS),
    };
    let centre = placement_to_navigation_centre(placement);
    let mut candidates = (0..NAVIGATION_SUBDIVISIONS)
        .flat_map(|z| {
            (0..NAVIGATION_SUBDIVISIONS).map(move |x| GridPos {
                x: base.x.saturating_add(x),
                z: base.z.saturating_add(z),
            })
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|candidate| {
        (
            u8::from(*candidate != centre),
            candidate.x.abs_diff(start.x) + candidate.z.abs_diff(start.z),
            candidate.x.abs_diff(centre.x) + candidate.z.abs_diff(centre.z),
            candidate.z,
            candidate.x,
        )
    });
    candidates
}

pub(crate) fn try_fine_agent_path(
    navigation: &stream_town_domain::NavGrid,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    paths: &PathSurfaceRuntime,
    kind: &ActorKind,
    start: GridPos,
    placement_goal: GridPos,
) -> Option<Vec<GridPos>> {
    let mut exceptions = if *kind == ActorKind::Player {
        completed_player_gate_navigation_cells(content, simulation)
    } else {
        HashSet::new()
    };
    // A topology change can leave an actor standing in a newly blocked fine
    // cell. Let it leave that cell instead of waiting for the unstuck timeout.
    if navigation.contains(start) && !navigation.is_walkable(start) {
        exceptions.insert(start);
    }
    // Candidates are ordered centre-first. A walkable centre is only a
    // preference, not a requirement: walls can disconnect that third while a
    // side third in the same coarse approach cell remains reachable.
    let goals = fine_navigation_goal_candidates(placement_goal, start)
        .into_iter()
        .filter(|goal| {
            navigation.contains(*goal)
                && (navigation.is_walkable(*goal) || exceptions.contains(goal))
        })
        .collect::<Vec<_>>();
    if *kind == ActorKind::Player {
        let maximum_path_level = paths.levels.values().copied().max().unwrap_or_default();
        goals.into_iter().find_map(|goal| {
            navigation
                .find_path_with_exceptions_and_costs(
                    start,
                    goal,
                    &exceptions,
                    path_route_step_cost(10, maximum_path_level),
                    path_route_step_cost(14, maximum_path_level),
                    |position, base_cost| {
                        path_route_step_cost(
                            base_cost,
                            paths.levels.get(&position).copied().unwrap_or_default(),
                        )
                    },
                )
                .ok()
        })
    } else {
        goals.into_iter().find_map(|goal| {
            navigation
                .find_path_with_exceptions(start, goal, &exceptions)
                .ok()
        })
    }
}

pub(crate) fn try_fine_agent_path_for_goal(
    navigation: &stream_town_domain::NavGrid,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    paths: &PathSurfaceRuntime,
    enemy_navigation: Option<&EnemyNavigationField>,
    kind: &ActorKind,
    goal_kind: &AgentGoal,
    start: GridPos,
    placement_goal: GridPos,
) -> Option<Vec<GridPos>> {
    let goal = placement_to_navigation_centre(placement_goal);
    if *kind == ActorKind::Enemy
        && let AgentGoal::AttackBuilding(building) = goal_kind
        && let Some(path) = enemy_navigation.and_then(|field| field.path_to(start, building, goal))
    {
        return Some(path);
    }
    if *kind == ActorKind::Enemy
        && let Some(path) =
            enemy_navigation.and_then(|field| field.hierarchical_path(navigation, start, goal))
    {
        return Some(path);
    }
    try_fine_agent_path(
        navigation,
        content,
        simulation,
        paths,
        kind,
        start,
        placement_goal,
    )
}

pub(crate) fn agent_action_facing_grid(
    goal: &AgentGoal,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
) -> Option<GridPos> {
    match goal {
        AgentGoal::Gather(resource) => world
            .resources
            .iter()
            .find(|candidate| candidate.id == *resource && candidate.amount > 0)
            .map(|resource| resource.position),
        AgentGoal::HarvestFarm(building)
        | AgentGoal::AttackBuilding(building)
        | AgentGoal::Construct(building)
        | AgentGoal::VisitRegenerationStation(building) => simulation
            .buildings
            .get(building)
            .map(|building| building_visual_grid(content, building)),
        AgentGoal::Attack(actor) | AgentGoal::Heal(actor) => {
            simulation.actors.get(actor).map(|actor| actor.position)
        }
        AgentGoal::PlantTree(position) | AgentGoal::PlantBush(position) => Some(*position),
        AgentGoal::Prospect { cell, .. } => Some(*cell),
        AgentGoal::Deposit | AgentGoal::WaitForStorage | AgentGoal::Wander => None,
    }
}

pub(crate) fn rotate_agent_toward(
    transform: &mut Transform,
    target: Vec3,
    delta_seconds: f32,
    snap: bool,
    correct_player_axis: bool,
) {
    let direction = Vec3::new(
        target.x - transform.translation.x,
        0.0,
        target.z - transform.translation.z,
    );
    if direction.length_squared() <= f32::EPSILON {
        return;
    }
    // Unity character and enemy meshes visibly face local +Z. Bevy's
    // Transform::looking_to aligns local -Z, which made every actor walk and
    // act backwards even though transform.forward() appeared correct.
    let mut target_rotation = Quat::from_rotation_arc(Vec3::Z, direction.normalize());
    if correct_player_axis {
        target_rotation *= Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    }
    transform.rotation = if snap {
        target_rotation
    } else {
        transform.rotation.slerp(
            target_rotation,
            (delta_seconds * AGENT_ROTATION_SPEED).clamp(0.0, 1.0),
        )
    };
}

pub(crate) fn rotate_agent_toward_action(
    transform: &mut Transform,
    agent: &Agent,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    delta_seconds: f32,
    correct_player_axis: bool,
) -> bool {
    let Some(facing_grid) = agent_action_facing_grid(&agent.goal, content, simulation, world)
    else {
        return false;
    };
    let facing_target = if let AgentGoal::Gather(resource_id) = &agent.goal {
        world
            .resources
            .iter()
            .find(|resource| resource.id == *resource_id && resource.amount > 0)
            .map_or_else(
                || grid_to_world_on_surface(facing_grid, config, world),
                |resource| {
                    let position = generated_resource_world_position(resource, config, world);
                    resource_visual_archetype(content, &resource.kind).map_or(
                        position,
                        |archetype| {
                            centred_resource_visual_position(
                                position,
                                archetype,
                                config.world.cell_size,
                            )
                        },
                    )
                },
            )
    } else {
        grid_to_world_on_surface(facing_grid, config, world)
    };
    rotate_agent_toward(
        transform,
        facing_target,
        delta_seconds,
        matches!(agent.goal, AgentGoal::Gather(_)),
        correct_player_axis,
    );
    true
}

pub(crate) fn agent_path_world_target(
    next: GridPos,
    agent: &Agent,
    content: &ContentCatalog,
    world: &GeneratedWorld,
    config: &GameConfig,
) -> Vec3 {
    let mut target = navigation_to_world_on_surface(next, config, world);
    if next != placement_to_navigation_centre(agent.target) {
        return target;
    }
    let AgentGoal::Gather(resource_id) = &agent.goal else {
        return target;
    };
    let Some(resource) = world
        .resources
        .iter()
        .find(|resource| resource.id == *resource_id && resource.amount > 0)
    else {
        return target;
    };
    let visual = generated_resource_world_position(resource, config, world);
    let visual = resource_visual_archetype(content, &resource.kind).map_or(visual, |archetype| {
        centred_resource_visual_position(visual, archetype, config.world.cell_size)
    });
    let toward_resource = Vec2::new(visual.x - target.x, visual.z - target.z);
    let offset = toward_resource.normalize_or_zero()
        * (config.world.cell_size * 0.42).min(toward_resource.length() * 0.5);
    target.x += offset.x;
    target.z += offset.y;
    target
}

pub(crate) fn actor_movement_speed(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor_id: &StableId,
) -> f32 {
    simulation
        .actors
        .get(actor_id)
        .map_or(config.gameplay.agent_speed_cells_per_second, |actor| {
            effective_role_stats(content, simulation, actor)
                .map_or(config.gameplay.agent_speed_cells_per_second, |stats| {
                    milli_units_as_f32(stats.movement_speed_milli_cells_per_second)
                })
        })
}

pub(crate) fn actor_movement_speed_on_path(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    paths: &PathSurfaceRuntime,
    actor_id: &StableId,
    navigation_position: GridPos,
) -> f32 {
    let speed = actor_movement_speed(config, content, simulation, actor_id);
    let Some(_actor) = simulation
        .actors
        .get(actor_id)
        .filter(|actor| actor.role.as_str() != "role:enemy")
    else {
        return speed;
    };
    speed
        * (1.0
            + f32::from(
                paths
                    .levels
                    .get(&navigation_position)
                    .copied()
                    .unwrap_or_default(),
            ) * 0.05)
}

pub(crate) fn path_route_step_cost(base_cost: u32, path_level: u16) -> u32 {
    let speed_milli = 1_000_u32.saturating_add(u32::from(path_level).saturating_mul(50));
    base_cost
        .saturating_mul(100_000)
        .saturating_add(speed_milli.saturating_sub(1))
        / speed_milli
}

pub(crate) fn predictive_speed_factors(
    agents: &[(StableId, Vec2, Vec2)],
    radius: f32,
    horizon_seconds: f32,
    minimum_factor: f32,
) -> BTreeMap<StableId, f32> {
    let mut factors = agents
        .iter()
        .map(|(id, _, _)| (id.clone(), 1.0_f32))
        .collect::<BTreeMap<_, _>>();
    if radius <= f32::EPSILON || horizon_seconds <= f32::EPSILON {
        return factors;
    }
    for left_index in 0..agents.len() {
        let (left_id, left_position, left_velocity) = &agents[left_index];
        for (right_id, right_position, right_velocity) in &agents[left_index + 1..] {
            let relative_position = *right_position - *left_position;
            let relative_velocity = *left_velocity - *right_velocity;
            let relative_speed_squared = relative_velocity.length_squared();
            if relative_speed_squared <= f32::EPSILON {
                continue;
            }
            let approach = relative_position.dot(relative_velocity);
            if approach <= 0.0 {
                continue;
            }
            let closest_seconds = (approach / relative_speed_squared).min(horizon_seconds);
            let closest_offset = relative_position - relative_velocity * closest_seconds;
            let closest_distance = closest_offset.length();
            if closest_distance >= radius {
                continue;
            }
            let time_urgency = 1.0 - closest_seconds / horizon_seconds;
            let distance_urgency = 1.0 - closest_distance / radius;
            let factor = (1.0 - time_urgency * distance_urgency * (1.0 - minimum_factor))
                .clamp(minimum_factor, 1.0);

            // A following agent yields to the one already ahead. At crossings
            // and head-on encounters, the stable actor ID is the deterministic
            // right-of-way tie-breaker, preventing reciprocal stop/start.
            let same_direction = left_velocity.dot(*right_velocity) > 0.0;
            let left_is_behind =
                same_direction && relative_position.dot(left_velocity.normalize_or_zero()) > 0.0;
            let right_is_behind = same_direction
                && (-relative_position).dot(right_velocity.normalize_or_zero()) > 0.0;
            let yielding = if left_is_behind && !right_is_behind {
                left_id
            } else if right_is_behind && !left_is_behind {
                right_id
            } else if left_id > right_id {
                left_id
            } else {
                right_id
            };
            factors
                .entry(yielding.clone())
                .and_modify(|current| *current = current.min(factor));
        }
    }
    factors
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn return_agent_to_town_hall(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    config: &GameConfig,
    world: &GeneratedWorld,
    navigation: &stream_town_domain::NavGrid,
    occupied: &mut BTreeMap<GridPos, StableId>,
    agent: &mut Agent,
    location: &mut GridLocation,
    animation: &AgentAnimation,
    transform: &mut Transform,
) {
    let town_hall = restored_town_hall_position(content, simulation, config);
    let desired_navigation = placement_to_navigation_centre(town_hall);
    let spawn_navigation =
        fine_navigation_recovery_position(navigation, desired_navigation, &agent.id, occupied)
            .unwrap_or(desired_navigation);
    let spawn = navigation_to_placement(spawn_navigation);
    if occupied.get(&location.0) == Some(&agent.id) {
        occupied.remove(&location.0);
    }
    occupied.insert(spawn, agent.id.clone());
    if let Some(actor) = simulation.actors.get_mut(&agent.id) {
        actor.position = spawn;
        actor.preferred_target = None;
    }
    let mut world_position = navigation_to_world_on_surface(spawn_navigation, config, world);
    if !animation.native {
        world_position.y += animation.base_scale.y * 0.5;
    }
    transform.translation = world_position;
    location.0 = spawn;
    agent.origin = spawn;
    agent.navigation_position = spawn_navigation;
    agent.target = spawn;
    agent.goal = AgentGoal::Wander;
    agent.path.clear();
    agent.path_index = 0;
    agent.action_started = false;
    agent.action_cooldown_seconds = 0.0;
    agent.repath_remaining_seconds = 0.0;
    agent.previous_wander_origin = None;
}

pub(crate) fn navigation_component_reaches_minimum(
    navigation: &stream_town_domain::NavGrid,
    start: GridPos,
    minimum_cells: usize,
) -> bool {
    if !navigation.is_walkable(start) {
        return false;
    }
    let mut visited = HashSet::from([start]);
    let mut queue = VecDeque::from([start]);
    while let Some(position) = queue.pop_front() {
        if visited.len() >= minimum_cells {
            return true;
        }
        for (neighbour, _) in navigation
            .walkable_neighbours(position)
            .into_iter()
            .flatten()
        {
            if visited.insert(neighbour) {
                queue.push_back(neighbour);
            }
        }
    }
    visited.len() >= minimum_cells
}

pub(crate) fn fine_navigation_recovery_position(
    navigation: &stream_town_domain::NavGrid,
    desired: GridPos,
    actor: &StableId,
    occupied: &BTreeMap<GridPos, StableId>,
) -> Option<GridPos> {
    const MINIMUM_SAFE_COMPONENT_CELLS: usize = 64;
    if navigation.width() == 0 || navigation.height() == 0 {
        return None;
    }
    let desired = GridPos {
        x: desired.x.min(navigation.width() - 1),
        z: desired.z.min(navigation.height() - 1),
    };
    let index = |position: GridPos| {
        usize::from(position.z) * usize::from(navigation.width()) + usize::from(position.x)
    };
    let mut visited =
        vec![false; usize::from(navigation.width()) * usize::from(navigation.height())];
    visited[index(desired)] = true;
    let mut queue = VecDeque::from([desired]);
    let mut fallback = None;
    while let Some(position) = queue.pop_front() {
        let placement = navigation_to_placement(position);
        let available = navigation.is_walkable(position)
            && occupied.get(&placement).is_none_or(|owner| owner == actor);
        if available {
            fallback.get_or_insert(position);
            if navigation_component_reaches_minimum(
                navigation,
                position,
                MINIMUM_SAFE_COMPONENT_CELLS,
            ) {
                return Some(position);
            }
        }
        for neighbour in [
            position
                .x
                .checked_sub(1)
                .map(|x| GridPos { x, z: position.z }),
            position
                .x
                .checked_add(1)
                .filter(|x| *x < navigation.width())
                .map(|x| GridPos { x, z: position.z }),
            position
                .z
                .checked_sub(1)
                .map(|z| GridPos { x: position.x, z }),
            position
                .z
                .checked_add(1)
                .filter(|z| *z < navigation.height())
                .map(|z| GridPos { x: position.x, z }),
        ]
        .into_iter()
        .flatten()
        {
            let neighbour_index = index(neighbour);
            if !visited[neighbour_index] {
                visited[neighbour_index] = true;
                queue.push_back(neighbour);
            }
        }
    }
    fallback
}
