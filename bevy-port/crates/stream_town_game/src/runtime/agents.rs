use super::super::*;

type MovableAgentQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        Entity,
        &'static mut Agent,
        &'static mut GridLocation,
        &'static AgentAnimation,
        &'static mut Transform,
        Option<&'static PlayerRigAxisCorrected>,
    ),
>;

pub(crate) fn move_agents(
    mut commands: Commands,
    cadence: Res<AgentSimulationCadence>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    authored_presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    station_targets: Res<StationTargetRuntime>,
    enemy_navigation: Res<EnemyNavigationRuntime>,
    mut world: ResMut<WorldRuntime>,
    mut simulation: ResMut<SimulationRuntime>,
    mut movement_stats: MovementStats,
    mut render_stats: ResMut<WorldRenderStats>,
    mut recovery: AgentRecoveryRuntime,
    mut regeneration: ResMut<RegenerationRoleRuntime>,
    mut agents: MovableAgentQuery,
    buildings: Query<(Entity, &RuntimeBuilding)>,
) {
    let delta = cadence.delta();
    movement_stats.session.elapsed_seconds += delta.as_secs_f64();
    regeneration.elapsed_seconds += delta.as_secs_f64();
    accelerate_forester_regeneration_cooldowns(
        &simulation.0,
        &world.generated,
        &mut regeneration,
        delta.as_secs_f64(),
    );
    simulation
        .0
        .tick(delta.as_secs_f32(), config.0.time.seconds_per_day);
    if let Some(elapsed) =
        take_passive_building_income_delta(&mut movement_stats.passive_income_elapsed, delta)
    {
        apply_passive_building_income(&config.0, &content.0, &mut simulation.0, elapsed);
    }
    if let Some(technology) = simulation
        .0
        .active_vote
        .as_ref()
        .filter(|vote| vote.remaining_seconds <= f32::EPSILON)
        .map(technology_vote_leader)
    {
        let objectives = content
            .0
            .technology
            .nodes
            .get(&technology)
            .map_or(&[][..], |node| node.objectives.as_slice());
        let _ =
            simulation
                .0
                .resolve_technology_vote(objectives, &content.0.objectives, MAX_TOWN_GOALS);
    }
    if let Some(weather) = debug_weather_override() {
        simulation.0.weather = weather;
    }
    let Some(navigation) = movement_stats.fine_navigation.grid.as_ref() else {
        return;
    };
    for worker in regeneration.workers.values_mut() {
        worker.planting_target = None;
    }
    for (_, agent, _, _, _, _) in &agents {
        if let Some(target) = goal_planting_target(&agent.goal) {
            regeneration
                .workers
                .entry(agent.id.clone())
                .or_default()
                .planting_target = Some(target);
        }
    }
    let mut resource_reservations = BTreeMap::new();
    let mut target_assignment_counts: BTreeMap<StableId, u32> = BTreeMap::new();
    let mut occupied_approaches = simulation
        .0
        .actors
        .values()
        .filter(|actor| actor.alive)
        .fold(
            BTreeMap::<GridPos, StableId>::new(),
            |mut occupied, actor| {
                occupied
                    .entry(actor.position)
                    .and_modify(|owner| {
                        if actor.id < *owner {
                            owner.clone_from(&actor.id);
                        }
                    })
                    .or_insert_with(|| actor.id.clone());
                occupied
            },
        );
    let mut approach_reservations = BTreeMap::<GridPos, StableId>::new();
    let mut idle_approach_reservations = BTreeMap::<GridPos, StableId>::new();
    for (_, agent, _, _, _, _) in &agents {
        if let Some(target) = goal_reservation(&agent.goal)
            && goal_reservation_is_valid(
                &simulation.0,
                &world.generated,
                &content.0,
                &agent.id,
                &agent.goal,
            )
        {
            resource_reservations
                .entry(target.clone())
                .and_modify(|owner| {
                    if agent.id < *owner {
                        owner.clone_from(&agent.id);
                    }
                })
                .or_insert_with(|| agent.id.clone());
        }
        if let Some(target) = goal_assignment_target(&agent.goal)
            && goal_reservation_is_valid(
                &simulation.0,
                &world.generated,
                &content.0,
                &agent.id,
                &agent.goal,
            )
        {
            target_assignment_counts
                .entry(target.clone())
                .and_modify(|count| *count = count.saturating_add(1))
                .or_insert(1_u32);
        }
        if goal_reserves_approach(&agent.goal) {
            let reservations = if goal_uses_idle_approach_reservation(&agent.goal) {
                &mut idle_approach_reservations
            } else {
                &mut approach_reservations
            };
            reservations
                .entry(agent.target)
                .and_modify(|owner| {
                    if agent.id < *owner {
                        owner.clone_from(&agent.id);
                    }
                })
                .or_insert_with(|| agent.id.clone());
        }
    }
    let mut agent_order: Vec<_> = agents
        .iter()
        .map(|(entity, agent, _, _, _, _)| (agent.id.clone(), entity))
        .collect();
    agent_order.sort_by(|(left, _), (right, _)| left.cmp(right));
    recovery
        .path_failures
        .0
        .retain(|actor, _| simulation.0.actors.contains_key(actor));
    recovery
        .retreating
        .0
        .retain(|actor| simulation.0.actors.contains_key(actor));
    let predictive_agents = agents
        .iter()
        .filter_map(|(_, agent, _, _, transform, _)| {
            let next = agent.path.get(agent.path_index).copied()?;
            let position = Vec2::new(transform.translation.x, transform.translation.z);
            let target = navigation_to_world_on_surface(next, &config.0, &world.generated);
            let direction = Vec2::new(target.x - position.x, target.z - position.y);
            let speed = actor_movement_speed_on_path(
                &config.0,
                &content.0,
                &simulation.0,
                &movement_stats.path_surfaces,
                &agent.id,
                agent.navigation_position,
            ) * config.0.world.cell_size;
            Some((
                agent.id.clone(),
                position,
                direction.normalize_or_zero() * speed,
            ))
        })
        .collect::<Vec<_>>();
    let predictive_factors = predictive_speed_factors(
        &predictive_agents,
        config.0.world.cell_size * CROWD_PREDICTION_RADIUS_CELLS,
        CROWD_PREDICTION_HORIZON_SECONDS,
        CROWD_MINIMUM_YIELD_SPEED,
    );
    render_stats.crowd_yielding_agents = predictive_factors
        .values()
        .filter(|factor| **factor < 1.0 - f32::EPSILON)
        .count();
    for (_, entity) in agent_order {
        let Ok((_, mut agent, mut location, animation, mut transform, axis_corrected)) =
            agents.get_mut(entity)
        else {
            continue;
        };
        agent.action_cooldown_seconds =
            (agent.action_cooldown_seconds - delta.as_secs_f32()).max(0.0);
        agent.repath_remaining_seconds =
            (agent.repath_remaining_seconds - delta.as_secs_f32()).max(0.0);
        if let Some((role_stats, regeneration_requires_food)) = simulation
            .0
            .actors
            .get(&agent.id)
            .filter(|actor| actor.alive)
            .and_then(|actor| {
                effective_role_stats(&content.0, &simulation.0, actor).map(|stats| {
                    let requires_food = actor_archetype(&content.0, actor)
                        .and_then(|archetype| archetype.health.as_ref())
                        .is_some_and(|health| health.regeneration_requires_food);
                    (stats, requires_food)
                })
            })
        {
            let desired_max = i32::try_from(role_stats.max_health)
                .unwrap_or(i32::MAX)
                .max(1);
            let regen_milli = i32::try_from(
                role_stats
                    .health_regen_milli_per_second
                    .clamp(0, i64::from(i32::MAX)),
            )
            .expect("clamped regeneration fits i32");
            let regen_per_second = f64::from(regen_milli) / 1_000.0;
            agent.health_regen_accumulator += regen_per_second * delta.as_secs_f64();
            let mut regenerated = 0_i32;
            while agent.health_regen_accumulator >= 1.0 && regenerated < desired_max {
                agent.health_regen_accumulator -= 1.0;
                regenerated += 1;
            }
            if regeneration_requires_food && regenerated > 0 {
                let food = StableId::new("resource:food").expect("static stable ID");
                let required = u32::try_from(regenerated).expect("regeneration is non-negative");
                let available = simulation
                    .0
                    .town_resources
                    .get(&food)
                    .copied()
                    .unwrap_or_default();
                if available >= required {
                    simulation
                        .0
                        .town_resources
                        .insert(food, available - required);
                } else {
                    regenerated = 0;
                }
            }
            if let Some(actor) = simulation.0.actors.get_mut(&agent.id) {
                let increase = desired_max.saturating_sub(actor.max_health).max(0);
                actor.max_health = desired_max;
                actor.health = actor
                    .health
                    .saturating_add(increase)
                    .saturating_add(regenerated)
                    .min(desired_max);
            }
        }
        let was_retreating = recovery.retreating.0.contains(&agent.id);
        let is_retreating = simulation
            .0
            .actors
            .get(&agent.id)
            .is_some_and(|actor| citizen_should_retreat(actor, was_retreating));
        if is_retreating {
            recovery.retreating.0.insert(agent.id.clone());
        } else {
            recovery.retreating.0.remove(&agent.id);
        }
        if is_retreating != was_retreating {
            agent.goal = AgentGoal::Wander;
            agent.path.clear();
            agent.path_index = 0;
            agent.action_started = false;
            agent.action_cooldown_seconds = 0.0;
            agent.repath_remaining_seconds = 0.0;
            agent.previous_wander_origin = None;
        }
        let alive = simulation
            .0
            .actors
            .get(&agent.id)
            .is_some_and(|actor| actor.alive);
        if alive {
            if let Some(actor_position) = simulation
                .0
                .actors
                .get(&agent.id)
                .map(|actor| actor.position)
                && actor_position != location.0
            {
                let mut world_position =
                    grid_to_world_on_surface(actor_position, &config.0, &world.generated);
                if !animation.native {
                    world_position.y += animation.base_scale.y * 0.5;
                }
                transform.translation = world_position;
                location.0 = actor_position;
                agent.origin = actor_position;
                agent.navigation_position = placement_to_navigation_centre(actor_position);
                agent.target =
                    deterministic_wander_target(&world.generated, &agent.id, actor_position);
                agent.path.clear();
                agent.path_index = 0;
                agent.action_cooldown_seconds = 0.0;
                agent.action_started = false;
                agent.repath_remaining_seconds = 0.0;
                agent.previous_wander_origin = None;
            }
        } else {
            agent.path.clear();
            agent.goal = AgentGoal::Wander;
            agent.action_started = false;
            agent.action_cooldown_seconds = 0.0;
            agent.repath_remaining_seconds = 0.0;
            agent.health_regen_accumulator = 0.0;
            if agent.kind == ActorKind::Enemy {
                continue;
            }
            let remaining = simulation
                .0
                .actors
                .get(&agent.id)
                .and_then(|actor| actor.respawn_remaining_seconds);
            if remaining.is_none() {
                let _ = simulation
                    .0
                    .schedule_respawn(&agent.id, AUTOMATIC_PLAYER_RESPAWN_SECONDS);
                continue;
            }
            if remaining.is_some_and(|remaining| remaining > f64::EPSILON) {
                continue;
            }
            let town_hall = restored_town_hall_position(&content.0, &simulation.0, &config.0);
            let desired_navigation = placement_to_navigation_centre(town_hall);
            let spawn_navigation = fine_navigation_recovery_position(
                navigation,
                desired_navigation,
                &agent.id,
                &occupied_approaches,
            )
            .unwrap_or(desired_navigation);
            let spawn = navigation_to_placement(spawn_navigation);
            if simulation
                .0
                .respawn_actor_with_level_penalty(&agent.id, spawn)
                .is_err()
            {
                continue;
            }
            occupied_approaches.insert(spawn, agent.id.clone());
            let mut world_position =
                navigation_to_world_on_surface(spawn_navigation, &config.0, &world.generated);
            if !animation.native {
                world_position.y += animation.base_scale.y * 0.5;
            }
            transform.translation = world_position;
            location.0 = spawn;
            agent.origin = spawn;
            agent.navigation_position = spawn_navigation;
            agent.target = deterministic_wander_target(&world.generated, &agent.id, spawn);
            agent.action_cooldown_seconds = 0.0;
            agent.action_started = false;
            agent.repath_remaining_seconds = 0.0;
            agent.previous_wander_origin = None;
            spawn_healing_effect(
                &mut commands,
                &authored_presentation.0,
                &render,
                grid_to_world_on_surface(spawn, &config.0, &world.generated),
                HealingEffectKind::Revive,
                config.0.world.cell_size,
                Some(agent.id.clone()),
            );
        }
        if std::env::var_os("STREAM_TOWN_SMOKE_GATE").is_some()
            && agent.id.as_str() == "npc:starting_defender"
        {
            agent.path.clear();
            agent.path_index = 0;
            agent.origin = location.0;
            agent.target = location.0;
            continue;
        }
        ensure_actor_station(&content.0, &mut simulation.0, &config.0, &agent.id);
        if !is_retreating
            && agent_is_moving(&agent)
            && agent.repath_remaining_seconds <= f32::EPSILON
            && matches!(
                agent.goal,
                AgentGoal::Attack(_) | AgentGoal::AttackBuilding(_) | AgentGoal::Heal(_)
            )
        {
            let (updated_goal, updated_target) = retained_healing_goal(
                &agent.goal,
                &content.0,
                &simulation.0,
                &agent.id,
                location.0,
            )
            .unwrap_or_else(|| {
                next_agent_goal_with_station_runtime(
                    &simulation.0,
                    &world.generated,
                    enemy_navigation.field.as_ref(),
                    &config.0,
                    &content.0,
                    &station_targets,
                    &agent.id,
                    location.0,
                    agent.navigation_position,
                    &resource_reservations,
                    &target_assignment_counts,
                    &occupied_approaches,
                    &approach_reservations,
                )
            });
            if updated_goal != agent.goal || updated_target != agent.target {
                agent.path.clear();
                agent.path_index = 0;
                agent.action_started = false;
                agent.action_cooldown_seconds = 0.0;
            }
            agent.repath_remaining_seconds = config.0.gameplay.repath_interval_seconds.max(0.1);
        }
        let idle_replan_delay = idle_wander_replan_delay(&agent.kind, &agent.goal);
        if agent.path.is_empty()
            && idle_replan_delay.is_some()
            && agent.repath_remaining_seconds > f32::EPSILON
        {
            recovery.path_failures.clear_movement(&agent.id);
            continue;
        }
        if agent.path.is_empty() || agent.path_index >= agent.path.len() {
            let completed_idle_wander = !agent.path.is_empty() && idle_replan_delay.is_some();
            if !agent.path.is_empty() {
                let completing_started_action = agent.action_started;
                if !completing_started_action {
                    movement_stats.session.paths_completed += 1;
                }
                let action_phase = if location.0 == agent.target {
                    let delay = if completing_started_action {
                        0.0
                    } else {
                        action_cooldown(&content.0, &simulation.0, &agent.id, &agent.goal)
                    };
                    arrived_action_phase(&mut agent, delay)
                } else {
                    ArrivedActionPhase::NonAction
                };
                if matches!(
                    action_phase,
                    ArrivedActionPhase::Started | ArrivedActionPhase::Waiting
                ) {
                    // Unity enters the action state, plays its animation, and
                    // invokes DoAction only after the authored timer.
                    // Keep turning throughout that timer. Previously the early
                    // continue froze construction actors after only the single
                    // arrival-frame rotation step.
                    rotate_agent_toward_action(
                        &mut transform,
                        &agent,
                        &content.0,
                        &simulation.0,
                        &world.generated,
                        &config.0,
                        delta.as_secs_f32(),
                        axis_corrected.is_some(),
                    );
                    continue;
                }
                if action_phase == ArrivedActionPhase::Complete {
                    if let Some(presentation) = complete_agent_goal_with_regeneration(
                        &mut simulation.0,
                        &mut world.generated,
                        &config.0,
                        &content.0,
                        &mut regeneration,
                        &agent.id,
                        &agent.goal,
                        location.0,
                    ) {
                        match presentation {
                            ActionPresentation::Projectile(projectile) => {
                                spawn_combat_projectile(
                                    &mut commands,
                                    &render,
                                    &config.0,
                                    transform.translation,
                                    projectile,
                                );
                            }
                            ActionPresentation::Impact { target, visual } => {
                                spawn_combat_impact(
                                    &mut commands,
                                    &render,
                                    grid_to_world_on_surface(target, &config.0, &world.generated)
                                        + Vec3::Y * config.0.world.cell_size * 0.35,
                                    visual,
                                    config.0.world.cell_size,
                                );
                            }
                            ActionPresentation::Healing { source, target } => {
                                let source_position = simulation
                                    .0
                                    .actors
                                    .get(&source)
                                    .map_or(location.0, |actor| actor.position);
                                let target_position = simulation
                                    .0
                                    .actors
                                    .get(&target)
                                    .map_or(location.0, |actor| actor.position);
                                spawn_healing_effect(
                                    &mut commands,
                                    &authored_presentation.0,
                                    &render,
                                    grid_to_world_on_surface(
                                        source_position,
                                        &config.0,
                                        &world.generated,
                                    ),
                                    HealingEffectKind::Channel,
                                    config.0.world.cell_size,
                                    Some(source),
                                );
                                spawn_healing_effect(
                                    &mut commands,
                                    &authored_presentation.0,
                                    &render,
                                    grid_to_world_on_surface(
                                        target_position,
                                        &config.0,
                                        &world.generated,
                                    ),
                                    HealingEffectKind::Burst,
                                    config.0.world.cell_size,
                                    Some(target),
                                );
                            }
                            ActionPresentation::BuildingWork { target, sparks } => {
                                spawn_building_work_effect(
                                    &mut commands,
                                    &render,
                                    grid_to_world_on_surface(target, &config.0, &world.generated),
                                    sparks,
                                    config.0.world.cell_size,
                                );
                            }
                            ActionPresentation::BuildingDestroyed { building, target } => {
                                let origin =
                                    grid_to_world_on_surface(target, &config.0, &world.generated);
                                spawn_building_work_effect(
                                    &mut commands,
                                    &render,
                                    origin,
                                    true,
                                    config.0.world.cell_size * 1.5,
                                );
                                if let Some((entity, _)) =
                                    buildings.iter().find(|(_, runtime)| runtime.id == building)
                                {
                                    commands.entity(entity).try_despawn();
                                }
                            }
                        }
                    }
                    agent.action_started = false;
                    agent.action_cooldown_seconds = 0.0;
                }
            }
            agent.origin = location.0;
            if let Some(reserved) = goal_reservation(&agent.goal)
                && resource_reservations.get(reserved) == Some(&agent.id)
            {
                resource_reservations.remove(reserved);
            }
            if let Some(target) = goal_assignment_target(&agent.goal)
                && let Some(count) = target_assignment_counts.get_mut(target)
            {
                *count = count.saturating_sub(1);
                if *count == 0 {
                    target_assignment_counts.remove(target);
                }
            }
            if goal_reserves_approach(&agent.goal) {
                let reservations = if goal_uses_idle_approach_reservation(&agent.goal) {
                    &mut idle_approach_reservations
                } else {
                    &mut approach_reservations
                };
                if reservations.get(&agent.target) == Some(&agent.id) {
                    reservations.remove(&agent.target);
                }
            }
            if completed_idle_wander {
                // Unity waits three seconds at an idle destination before sampling
                // another point. Replanning immediately made trapped or fully
                // reserved citizens run a complete fine-grid search every frame.
                agent.path.clear();
                agent.path_index = 0;
                agent.repath_remaining_seconds =
                    idle_replan_delay.expect("completed citizen wander has an idle delay");
                recovery.path_failures.clear_movement(&agent.id);
                continue;
            }
            let mut planning_reservations = resource_reservations.clone();
            let mut planning_approach_reservations = approach_reservations.clone();
            let unreachable_owner =
                StableId::new("system:unreachable-planning-target").expect("static stable ID");
            let mut rejected_resources = 0_usize;
            let mut rejected_approaches = 0_usize;
            let maximum_rejected_approaches = usize::from(world.generated.navigation.width())
                .saturating_mul(usize::from(world.generated.navigation.height()))
                .max(1);
            let (goal, target, mut planned_path) = loop {
                if is_retreating {
                    break (AgentGoal::Wander, location.0, None);
                }
                let (candidate_goal, candidate_target) = retained_healing_goal(
                    &agent.goal,
                    &content.0,
                    &simulation.0,
                    &agent.id,
                    location.0,
                )
                .or_else(|| {
                    regeneration_agent_goal_with_reservations(
                        &content.0,
                        &simulation.0,
                        &world.generated,
                        &mut regeneration,
                        &agent.id,
                        location.0,
                        &approach_reservations,
                    )
                })
                .unwrap_or_else(|| {
                    next_agent_goal_with_station_runtime(
                        &simulation.0,
                        &world.generated,
                        enemy_navigation.field.as_ref(),
                        &config.0,
                        &content.0,
                        &station_targets,
                        &agent.id,
                        location.0,
                        agent.navigation_position,
                        &planning_reservations,
                        &target_assignment_counts,
                        &occupied_approaches,
                        &planning_approach_reservations,
                    )
                });
                if candidate_goal == AgentGoal::Wander {
                    break (candidate_goal, candidate_target, None);
                }
                if let Some(path) = try_fine_agent_path_for_goal(
                    navigation,
                    &content.0,
                    &simulation.0,
                    &movement_stats.path_surfaces,
                    enemy_navigation.field.as_ref(),
                    &agent.kind,
                    &candidate_goal,
                    agent.navigation_position,
                    candidate_target,
                ) {
                    break (candidate_goal, candidate_target, Some(path));
                }
                match &candidate_goal {
                    AgentGoal::Gather(resource) => {
                        debug!(
                            actor = %agent.id,
                            resource = %resource,
                            ?candidate_target,
                            "resource target has no valid path; trying another compatible target"
                        );
                        planning_reservations.insert(resource.clone(), unreachable_owner.clone());
                        rejected_resources += 1;
                        if rejected_resources >= world.generated.resources.len().max(1) {
                            break (AgentGoal::Wander, location.0, None);
                        }
                    }
                    AgentGoal::Construct(building) => {
                        debug!(
                            actor = %agent.id,
                            building = %building,
                            ?candidate_target,
                            "construction approach has no valid path; trying another approach or building"
                        );
                        planning_approach_reservations
                            .insert(candidate_target, unreachable_owner.clone());
                        rejected_approaches += 1;
                        if rejected_approaches >= maximum_rejected_approaches {
                            break (AgentGoal::Wander, location.0, None);
                        }
                    }
                    _ => {
                        debug!(
                            actor = %agent.id,
                            ?candidate_goal,
                            ?candidate_target,
                            "agent target has no valid path; retrying with idle movement"
                        );
                        break (AgentGoal::Wander, location.0, None);
                    }
                }
            };
            let target = if goal == AgentGoal::Wander {
                if is_retreating {
                    let town_hall =
                        restored_town_hall_position(&content.0, &simulation.0, &config.0);
                    if grid_distance_squared(location.0, town_hall)
                        > u64::from(RETREAT_TOWN_HALL_RADIUS_CELLS).pow(2)
                    {
                        agent.previous_wander_origin = None;
                        town_hall_wait_target(
                            &content.0,
                            &simulation.0,
                            &world.generated,
                            &config.0,
                            location.0,
                        )
                    } else {
                        let anchor =
                            nearest_walkable(&world.generated, town_hall).unwrap_or(town_hall);
                        let target = deterministic_wander_target_step(
                            &world.generated,
                            &agent.id,
                            anchor,
                            location.0,
                            agent.wander_sequence,
                            agent.previous_wander_origin,
                            |candidate| {
                                approach_reservation_available(
                                    &approach_reservations,
                                    &agent.id,
                                    candidate,
                                ) && approach_reservation_available(
                                    &idle_approach_reservations,
                                    &agent.id,
                                    candidate,
                                ) && occupied_approaches
                                    .get(&candidate)
                                    .is_none_or(|owner| owner == &agent.id)
                                    && try_fine_agent_path_for_goal(
                                        navigation,
                                        &content.0,
                                        &simulation.0,
                                        &movement_stats.path_surfaces,
                                        enemy_navigation.field.as_ref(),
                                        &agent.kind,
                                        &AgentGoal::Wander,
                                        agent.navigation_position,
                                        candidate,
                                    )
                                    .is_some()
                            },
                        );
                        agent.wander_sequence = agent.wander_sequence.wrapping_add(1);
                        agent.previous_wander_origin = Some(location.0);
                        target
                    }
                } else {
                    let anchor = actor_idle_anchor(
                        &content.0,
                        &simulation.0,
                        &config.0,
                        &regeneration,
                        &agent.id,
                        location.0,
                    );
                    let target = deterministic_wander_target_step(
                        &world.generated,
                        &agent.id,
                        anchor,
                        location.0,
                        agent.wander_sequence,
                        agent.previous_wander_origin,
                        |candidate| {
                            approach_reservation_available(
                                &approach_reservations,
                                &agent.id,
                                candidate,
                            ) && approach_reservation_available(
                                &idle_approach_reservations,
                                &agent.id,
                                candidate,
                            ) && occupied_approaches
                                .get(&candidate)
                                .is_none_or(|owner| owner == &agent.id)
                                && try_fine_agent_path_for_goal(
                                    navigation,
                                    &content.0,
                                    &simulation.0,
                                    &movement_stats.path_surfaces,
                                    enemy_navigation.field.as_ref(),
                                    &agent.kind,
                                    &AgentGoal::Wander,
                                    agent.navigation_position,
                                    candidate,
                                )
                                .is_some()
                        },
                    );
                    agent.wander_sequence = agent.wander_sequence.wrapping_add(1);
                    agent.previous_wander_origin = Some(location.0);
                    target
                }
            } else {
                agent.previous_wander_origin = None;
                target
            };
            if planned_path.is_none() {
                planned_path = try_fine_agent_path_for_goal(
                    navigation,
                    &content.0,
                    &simulation.0,
                    &movement_stats.path_surfaces,
                    enemy_navigation.field.as_ref(),
                    &agent.kind,
                    &goal,
                    agent.navigation_position,
                    target,
                );
            }
            if planned_path.is_some() {
                recovery.path_failures.clear_path_failure(&agent.id);
            } else if recovery
                .path_failures
                .record_failure(&agent.id, delta.as_secs_f32())
            {
                return_agent_to_town_hall(
                    &content.0,
                    &mut simulation.0,
                    &config.0,
                    &world.generated,
                    navigation,
                    &mut occupied_approaches,
                    &mut agent,
                    &mut location,
                    animation,
                    &mut transform,
                );
                recovery.path_failures.clear(&agent.id);
                warn!(actor = %agent.id, "automatically returned an endlessly unpathable actor to the Town Hall");
                continue;
            }
            agent.goal = goal;
            agent.action_started = false;
            agent.action_cooldown_seconds = 0.0;
            if let Some(reserved) = goal_reservation(&agent.goal) {
                resource_reservations.insert(reserved.clone(), agent.id.clone());
            }
            if let Some(target) = goal_assignment_target(&agent.goal) {
                target_assignment_counts
                    .entry(target.clone())
                    .and_modify(|count| *count = count.saturating_add(1))
                    .or_insert(1_u32);
            }
            if goal_reserves_approach(&agent.goal) {
                let reservations = if goal_uses_idle_approach_reservation(&agent.goal) {
                    &mut idle_approach_reservations
                } else {
                    &mut approach_reservations
                };
                reservations.insert(target, agent.id.clone());
            }
            agent.target = target;
            agent.path = planned_path.unwrap_or_else(|| vec![agent.navigation_position]);
            agent.path_index = usize::from(agent.path.len() > 1);
            if matches!(agent.goal, AgentGoal::Construct(_)) {
                debug!(
                    actor = %agent.id,
                    ?agent.goal,
                    ?agent.target,
                    path_steps = agent.path.len(),
                    "builder construction route planned"
                );
            }
        }
        let Some(next) = agent.path.get(agent.path_index).copied() else {
            recovery.path_failures.clear_movement(&agent.id);
            continue;
        };
        let mut target =
            agent_path_world_target(next, &agent, &content.0, &world.generated, &config.0);
        if !animation.native {
            target.y += animation.base_scale.y * 0.5;
        }
        let distance = target - transform.translation;
        let speed = actor_movement_speed_on_path(
            &config.0,
            &content.0,
            &simulation.0,
            &movement_stats.path_surfaces,
            &agent.id,
            agent.navigation_position,
        ) * predictive_factors.get(&agent.id).copied().unwrap_or(1.0);
        let step = speed * config.0.world.cell_size * delta.as_secs_f32();
        if distance.length_squared() <= step * step {
            transform.translation = target;
            let previous_placement = location.0;
            agent.navigation_position = next;
            let next_placement = navigation_to_placement(next);
            record_completed_cell_traversal(
                &mut movement_stats.traversal_wear,
                &config.0.terrain,
                &agent.kind,
                previous_placement,
                next_placement,
            );
            location.0 = next_placement;
            agent.path_index += 1;
            if let Some(actor) = simulation.0.actors.get_mut(&agent.id) {
                actor.position = next_placement;
            }
        } else {
            transform.translation += distance.normalize_or_zero() * step;
        }
        if recovery.path_failures.record_movement(
            &agent.id,
            Vec2::new(transform.translation.x, transform.translation.z),
            agent.path_index,
            agent.target,
            delta.as_secs_f32(),
            config.0.world.cell_size * 0.05,
        ) {
            // A stalled renderer/locomotion sample does not mean the actor is
            // globally unpathable. Snap back to the last confirmed fine-grid
            // waypoint and force a local replan; reserve the Town Hall return
            // for five seconds of actual pathfinding failure above.
            let recovered_navigation = agent.navigation_position;
            let recovered_placement = navigation_to_placement(recovered_navigation);
            let mut recovered_world =
                navigation_to_world_on_surface(recovered_navigation, &config.0, &world.generated);
            if !animation.native {
                recovered_world.y += animation.base_scale.y * 0.5;
            }
            transform.translation = recovered_world;
            location.0 = recovered_placement;
            if let Some(actor) = simulation.0.actors.get_mut(&agent.id) {
                actor.position = recovered_placement;
            }
            agent.origin = recovered_placement;
            agent.path.clear();
            agent.path_index = 0;
            agent.action_started = false;
            agent.action_cooldown_seconds = 0.0;
            agent.repath_remaining_seconds = 0.0;
            agent.previous_wander_origin = None;
            recovery.path_failures.clear(&agent.id);
            warn!(actor = %agent.id, "locally reset a stalled actor route without moving it to the Town Hall");
            continue;
        }
        let action_facing = !agent_is_moving(&agent)
            && rotate_agent_toward_action(
                &mut transform,
                &agent,
                &content.0,
                &simulation.0,
                &world.generated,
                &config.0,
                delta.as_secs_f32(),
                axis_corrected.is_some(),
            );
        if !action_facing && let Some(facing_grid) = agent.path.get(agent.path_index).copied() {
            let facing_target =
                navigation_to_world_on_surface(facing_grid, &config.0, &world.generated);
            rotate_agent_toward(
                &mut transform,
                facing_target,
                delta.as_secs_f32(),
                false,
                axis_corrected.is_some(),
            );
        }
    }
}

pub(crate) fn start_queued_town_event(mut simulation: ResMut<SimulationRuntime>) {
    let Some(event) = simulation.0.take_next_queued_event() else {
        return;
    };
    match event {
        TownEvent::FishGod => {
            let _ = simulation.0.start_fish_god(true);
        }
        other => simulation.0.trigger_event(other),
    }
}

pub(crate) fn record_completed_cell_traversal(
    traversal_wear: &mut TraversalWearRuntime,
    settings: &stream_town_domain::TerrainAppearanceConfig,
    kind: &ActorKind,
    previous: GridPos,
    next: GridPos,
) {
    if *kind == ActorKind::Player && previous != next {
        traversal_wear.record(next, settings);
    }
}

pub(crate) fn reset_crowd_separation(
    mut runtime: ResMut<CrowdSeparationRuntime>,
    mut agents: Query<(Entity, &mut Transform), With<Agent>>,
) {
    for (entity, mut transform) in &mut agents {
        if let Some(offset) = runtime.applied_offsets.remove(&entity) {
            transform.translation -= offset;
        }
    }
    runtime.applied_offsets.clear();
}

pub(crate) fn apply_crowd_separation(
    config: Res<RuntimeConfig>,
    mut runtime: ResMut<CrowdSeparationRuntime>,
    mut stats: ResMut<WorldRenderStats>,
    mut agents: Query<(Entity, &Agent, &mut Transform)>,
) {
    let mut ordered = agents
        .iter()
        .map(|(entity, agent, transform)| {
            (
                agent.id.clone(),
                entity,
                Vec2::new(transform.translation.x, transform.translation.z),
            )
        })
        .collect::<Vec<_>>();
    ordered.sort_by(|left, right| left.0.cmp(&right.0));
    let positions = ordered
        .iter()
        .map(|(id, _, position)| (id.clone(), *position))
        .collect::<Vec<_>>();
    let radius = config.0.world.cell_size * CROWD_SEPARATION_RADIUS_CELLS;
    let max_offset = config.0.world.cell_size * CROWD_SEPARATION_MAX_CELLS;
    let offsets = crowd_separation_offsets(&positions, radius, max_offset);
    stats.crowd_adjusted_agents = 0;
    for ((_, entity, _), offset) in ordered.into_iter().zip(offsets) {
        if offset.length_squared() <= f32::EPSILON {
            continue;
        }
        let offset = Vec3::new(offset.x, 0.0, offset.y);
        if let Ok((_, _, mut transform)) = agents.get_mut(entity) {
            transform.translation += offset;
            runtime.applied_offsets.insert(entity, offset);
            stats.crowd_adjusted_agents += 1;
        }
    }
}

pub(crate) fn update_agent_locomotion(
    cadence: Res<AgentSimulationCadence>,
    config: Res<RuntimeConfig>,
    mut agents: Query<(&Agent, &Transform, &mut AgentLocomotion)>,
) {
    let delta_seconds = cadence.delta().as_secs_f32().max(f32::EPSILON);
    let cell_size = config.0.world.cell_size.max(f32::EPSILON);
    let authored_speed = (config.0.gameplay.agent_speed_cells_per_second
        / LOCOMOTION_REFERENCE_SPEED_CELLS_PER_SECOND)
        .clamp(0.0, 1.0);
    for (agent, transform, mut locomotion) in &mut agents {
        update_agent_locomotion_sample(
            &mut locomotion,
            transform.translation,
            delta_seconds,
            cell_size,
            authored_speed,
            agent_is_moving(agent),
        );
    }
}

pub(crate) fn update_agent_locomotion_sample(
    locomotion: &mut AgentLocomotion,
    position: Vec3,
    delta_seconds: f32,
    cell_size: f32,
    authored_speed: f32,
    initially_moving: bool,
) {
    if !locomotion.initialized {
        locomotion.previous_position = position;
        locomotion.initialized = true;
        if initially_moving {
            locomotion.normalized_speed = authored_speed;
            locomotion.stop_grace_seconds = LOCOMOTION_STOP_GRACE_SECONDS;
        }
        return;
    }

    let displacement = Vec2::new(
        position.x - locomotion.previous_position.x,
        position.z - locomotion.previous_position.z,
    )
    .length();
    locomotion.previous_position = position;
    let measured_speed =
        displacement / cell_size.max(f32::EPSILON) / delta_seconds.max(f32::EPSILON);
    if measured_speed > 0.05 {
        // The original AnimationHandler normalizes AIPath.velocity by a fixed
        // run speed. Use measured displacement only as the moving/stopped
        // signal; keeping the authored normalized speed stable prevents crowd
        // separation from chattering the walk/run blend weights.
        locomotion.normalized_speed = authored_speed;
        locomotion.stop_grace_seconds = LOCOMOTION_STOP_GRACE_SECONDS;
    } else if locomotion.stop_grace_seconds > 0.0 {
        // Unity drives this blend from AIPath.velocity, which does not emit a
        // one-frame zero while replacing a completed route. Retain the last
        // measured value briefly so a path handoff cannot stop and restart the
        // looping locomotion node.
        locomotion.stop_grace_seconds = (locomotion.stop_grace_seconds - delta_seconds).max(0.0);
    } else {
        locomotion.normalized_speed = 0.0;
    }
}

pub(crate) fn crowd_separation_offsets(
    agents: &[(StableId, Vec2)],
    radius: f32,
    max_offset: f32,
) -> Vec<Vec2> {
    if agents.len() < 2 || radius <= f32::EPSILON || max_offset <= f32::EPSILON {
        return vec![Vec2::ZERO; agents.len()];
    }
    let mut buckets: BTreeMap<(i32, i32), Vec<usize>> = BTreeMap::new();
    for (index, (_, position)) in agents.iter().enumerate() {
        let bucket = crowd_spatial_bucket(*position, radius);
        buckets.entry(bucket).or_default().push(index);
    }
    let mut offsets = vec![Vec2::ZERO; agents.len()];
    for (index, (id, position)) in agents.iter().enumerate() {
        let bucket = crowd_spatial_bucket(*position, radius);
        for bucket_z in bucket.1 - 1..=bucket.1 + 1 {
            for bucket_x in bucket.0 - 1..=bucket.0 + 1 {
                let Some(neighbours) = buckets.get(&(bucket_x, bucket_z)) else {
                    continue;
                };
                for &other_index in neighbours.iter().filter(|&&other| other > index) {
                    let (other_id, other_position) = &agents[other_index];
                    let delta = *position - *other_position;
                    let distance = delta.length();
                    if distance >= radius {
                        continue;
                    }
                    let direction = if distance > 0.001 {
                        delta / distance
                    } else {
                        deterministic_overlap_direction(id, other_id)
                    };
                    let strength = (1.0 - distance / radius) * max_offset * 0.5;
                    let separation = direction * strength;
                    offsets[index] += separation;
                    offsets[other_index] -= separation;
                }
            }
        }
    }
    for offset in &mut offsets {
        *offset = offset.clamp_length_max(max_offset);
    }
    offsets
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) fn crowd_spatial_bucket(position: Vec2, radius: f32) -> (i32, i32) {
    // Runtime world coordinates are bounded to a few thousand units; the cast
    // cannot approach i32 limits and intentionally discards the fractional cell.
    (
        (position.x / radius).floor() as i32,
        (position.y / radius).floor() as i32,
    )
}

pub(crate) fn deterministic_overlap_direction(left: &StableId, right: &StableId) -> Vec2 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in left
        .as_str()
        .bytes()
        .chain([0xff])
        .chain(right.as_str().bytes())
    {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    let milliradians = u16::try_from(hash % 6_283).expect("bounded angle fits u16");
    let angle = f32::from(milliradians) / 1_000.0;
    Vec2::from_angle(angle)
}

#[derive(Default)]
pub(crate) struct ResourceNodeSyncState {
    initialized: bool,
    enemy_camp_signature: u64,
    resource_visibility_signature: u64,
}

pub(crate) fn fold_visibility_signature(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash = (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

pub(crate) fn enemy_camp_visibility_signature(simulation: &WorldSimulation) -> u64 {
    simulation
        .enemy_camps
        .values()
        .fold(0xcbf2_9ce4_8422_2325_u64, |hash, camp| {
            let hash = fold_visibility_signature(hash, stable_id_hash(&camp.archetype));
            fold_visibility_signature(
                hash,
                u64::from(camp.position.x) | (u64::from(camp.position.z) << 16),
            )
        })
}

pub(crate) fn resource_visibility_signature(world: &GeneratedWorld) -> u64 {
    world
        .resources
        .iter()
        .filter(|resource| resource.amount > 0)
        .fold(
            fold_visibility_signature(
                0xcbf2_9ce4_8422_2325_u64,
                u64::try_from(world.resources.len()).unwrap_or(u64::MAX),
            ),
            |hash, resource| {
                let hash = fold_visibility_signature(hash, stable_id_hash(&resource.id));
                fold_visibility_signature(
                    hash,
                    u64::from(resource.position.x) | (u64::from(resource.position.z) << 16),
                )
            },
        )
}

pub(crate) fn sync_resource_nodes(
    mut commands: Commands,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    mut sync: Local<ResourceNodeSyncState>,
    mut resources: Query<(
        Entity,
        &ResourceNode,
        &GridLocation,
        Option<&PendingSurfaceGrounding>,
        &mut Visibility,
    )>,
) {
    // Resource amounts change for every gather action, but presentation changes
    // only when a node crosses zero, a node is added, or a camp footprint moves.
    // Avoid rebuilding live-ID sets and rescanning thousands of ECS entities for
    // ordinary positive-to-positive amount changes.
    let enemy_camp_signature = enemy_camp_visibility_signature(&simulation.0);
    let resource_visibility_signature = if sync.initialized && !world.is_changed() {
        sync.resource_visibility_signature
    } else {
        resource_visibility_signature(&world.generated)
    };
    if sync.initialized
        && sync.enemy_camp_signature == enemy_camp_signature
        && sync.resource_visibility_signature == resource_visibility_signature
    {
        return;
    }
    sync.initialized = true;
    sync.enemy_camp_signature = enemy_camp_signature;
    sync.resource_visibility_signature = resource_visibility_signature;
    let camp_regions = simulation
        .0
        .enemy_camps
        .values()
        .filter_map(|camp| {
            let archetype = content.0.archetypes.get(&camp.archetype)?;
            building_region(camp.position, archetype.footprint, &world.generated)
        })
        .collect::<Vec<_>>();
    let live_ids = world
        .generated
        .resources
        .iter()
        .filter(|resource| resource.amount > 0)
        .map(|resource| resource.id.as_str())
        .collect::<HashSet<_>>();
    let mut spawned_ids = HashSet::with_capacity(resources.iter().len());
    for (entity, node, location, pending_grounding, mut visibility) in &mut resources {
        if !live_ids.contains(node.id.as_str()) {
            commands.entity(entity).try_despawn();
            continue;
        }
        spawned_ids.insert(node.id.clone());
        let available = !camp_regions
            .iter()
            .any(|region| region_contains_grid_position(*region, location.0));
        let desired = if available && pending_grounding.is_none() {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != desired {
            *visibility = desired;
        }
    }
    for resource in world.generated.resources.iter().filter(|resource| {
        live_ids.contains(resource.id.as_str())
            && !spawned_ids.contains(&resource.id)
            && !camp_regions
                .iter()
                .any(|region| region_contains_grid_position(*region, resource.position))
    }) {
        let position = generated_resource_world_position(resource, &config.0, &world.generated);
        spawn_resource_visual(
            &mut commands,
            &content.0,
            &presentation.0,
            &render,
            asset_server.as_deref(),
            &asset_root.0,
            resource,
            position,
            &config.0,
            &world.generated,
        );
    }
}

pub(crate) fn building_construction_stage(health: i32, max_health: i32, complete: bool) -> u8 {
    if complete || health >= max_health {
        3
    } else if health > max_health * 66 / 100 {
        2
    } else {
        u8::from(health > max_health * 33 / 100)
    }
}

#[derive(Default)]
pub(crate) struct BuildingVisualSyncState {
    initialized: bool,
    signature: u64,
}

#[derive(Resource, Default)]
pub(crate) struct BuildingVisualSignatures {
    structure: u64,
    model_visibility: u64,
}

pub(crate) fn building_visual_structure_signature(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> u64 {
    // A town can contain hundreds of instances of the same building archetype.
    // Cache by archetype and level because visual age is gated by both the
    // town's unlocked age and the individual instance's upgrade level.
    let archetype_buildings = content
        .buildings
        .iter()
        .map(|(building_id, definition)| (definition.archetype.as_str(), building_id))
        .collect::<HashMap<_, _>>();
    let mut archetype_ages = HashMap::<(&str, u16), u8>::new();
    let mut signature = fold_visibility_signature(
        0xcbf2_9ce4_8422_2325_u64,
        u64::try_from(simulation.buildings.len()).unwrap_or(u64::MAX),
    );
    for building in simulation.buildings.values() {
        signature = fold_visibility_signature(signature, stable_id_hash(&building.id));
        signature = fold_visibility_signature(signature, stable_id_hash(&building.archetype));
        signature = fold_visibility_signature(
            signature,
            u64::from(building.position.x) | (u64::from(building.position.z) << 16),
        );
        signature = fold_visibility_signature(
            signature,
            u64::from(u32::from_ne_bytes(
                building.rotation_quarter_turns.to_ne_bytes(),
            )),
        );
        signature = fold_visibility_signature(signature, u64::from(building.level));
        signature = fold_visibility_signature(
            signature,
            u64::from(building_construction_stage(
                building.health,
                building_max_health(content, building),
                building.complete,
            )),
        );
        signature = fold_visibility_signature(signature, u64::from(building.complete));
        let key = (building.archetype.as_str(), building.level);
        let age = *archetype_ages.entry(key).or_insert_with(|| {
            archetype_buildings.get(key.0).map_or(1, |building_id| {
                building_age(content, simulation, building_id, key.1)
            })
        });
        signature = fold_visibility_signature(signature, u64::from(age));
    }
    signature
}

#[cfg(test)]
pub(crate) fn building_model_visibility_signature(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> u64 {
    building_model_visibility_signature_from_structure(
        config,
        content,
        simulation,
        building_visual_structure_signature(content, simulation),
    )
}

pub(crate) fn building_model_visibility_signature_from_structure(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    mut signature: u64,
) -> u64 {
    let storage_resources = content
        .buildings
        .values()
        .flat_map(|building| building.storage_models.iter())
        .map(|model| model.resource.clone())
        .collect::<BTreeSet<_>>();
    for resource in storage_resources {
        let amount = simulation
            .town_resources
            .get(&resource)
            .copied()
            .unwrap_or_default();
        let capacity = resource_storage_capacity(config, content, simulation, &resource);
        let fill_stage = if amount >= capacity {
            2_u64
        } else {
            u64::from(u64::from(amount).saturating_mul(2) >= u64::from(capacity))
        };
        signature = fold_visibility_signature(signature, stable_id_hash(&resource));
        signature = fold_visibility_signature(signature, fill_stage);
    }
    signature
}

pub(crate) fn refresh_building_visual_signatures(
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    mut signatures: ResMut<BuildingVisualSignatures>,
) {
    let structure = building_visual_structure_signature(&content.0, &simulation.0);
    let model_visibility = building_model_visibility_signature_from_structure(
        &config.0,
        &content.0,
        &simulation.0,
        structure,
    );
    if signatures.structure != structure || signatures.model_visibility != model_visibility {
        signatures.structure = structure;
        signatures.model_visibility = model_visibility;
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn sync_building_presentation(
    mut commands: Commands,
    simulation: Res<SimulationRuntime>,
    content: Res<RuntimeContent>,
    authored_presentation: Res<RuntimePresentation>,
    config: Res<RuntimeConfig>,
    signatures: Res<BuildingVisualSignatures>,
    render: Res<RenderAssets>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    added_buildings: Query<(), Added<RuntimeBuilding>>,
    mut sync: Local<BuildingVisualSyncState>,
    mut buildings: Query<(
        Entity,
        &RuntimeBuilding,
        &mut BuildingPresentation,
        &mut Transform,
        Option<&mut MeshMaterial3d<StandardMaterial>>,
        Option<&mut WorldAssetRoot>,
    )>,
) {
    let signature = signatures.structure;
    if sync.initialized && sync.signature == signature && added_buildings.is_empty() {
        return;
    }
    sync.initialized = true;
    sync.signature = signature;
    for (entity, runtime, mut presentation, mut transform, material, world_asset) in &mut buildings
    {
        let Some(state) = simulation.0.buildings.get(&runtime.id) else {
            continue;
        };
        let building = content
            .0
            .buildings
            .iter()
            .find(|(_, building)| building.archetype == state.archetype);
        let age = building.map_or(1, |(building_id, _)| {
            building_age(&content.0, &simulation.0, building_id, state.level)
        });
        let desired_scene = building.and_then(|(building_id, definition)| {
            let archetype = &content.0.archetypes[&definition.archetype];
            building_scene_for_state(
                archetype,
                definition,
                building_id,
                state,
                age,
                &content.0,
                &simulation.0,
            )
            .filter(|scene| converted_asset_exists(&asset_root.0, &scene.asset_path))
        });
        let desired_scene_path = desired_scene.map(|scene| scene.asset_path.as_str());
        let construction_stage = building_construction_stage(
            state.health,
            building_max_health(&content.0, state),
            state.complete,
        );
        if presentation.applied_stage == construction_stage
            && presentation.applied_level == state.level
            && presentation.applied_age == age
            && presentation.applied_scene.as_deref() == desired_scene_path
        {
            continue;
        }
        if presentation.applied_level != u16::MAX && state.level > presentation.applied_level {
            spawn_building_level_up_effect(
                &mut commands,
                &render,
                presentation.base_translation,
                config.0.world.cell_size,
            );
        }
        let has_authored_model = world_asset.is_some();
        if presentation.applied_scene.as_deref() != desired_scene_path {
            if let (Some((_, building)), Some(scene), Some(mut world_asset), Some(asset_server)) = (
                building,
                desired_scene,
                world_asset,
                asset_server.as_deref(),
            ) {
                let archetype = &content.0.archetypes[&building.archetype];
                world_asset.0 = asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone()));
                if let Some(material) = building_prefab_material_spec(
                    archetype,
                    scene,
                    &authored_presentation.0,
                    &render,
                ) {
                    commands.entity(entity).insert(material);
                }
            }
            presentation.applied_scene = desired_scene_path.map(str::to_owned);
        }
        if presentation.applied_age != age {
            presentation.applied_age = age;
        }
        let is_town_hall = runtime.id.as_str() == "building:townhall";
        let stage_scale = if has_authored_model {
            1.0
        } else {
            match construction_stage {
                0 => 0.35,
                1 => 0.55,
                2 => 0.75,
                _ => 1.0,
            }
        };
        let level_scale = if is_town_hall {
            1.0
        } else {
            1.0 + f32::from(state.level.saturating_sub(1)) * 0.05
        };
        let scale = stage_scale * level_scale;
        transform.scale = presentation.base_scale * scale;
        transform.translation = presentation.base_translation
            - Vec3::Y * presentation.base_height_offset * (1.0 - stage_scale);
        if let Some(mut material) = material {
            material.0 = if state.complete {
                render.building.clone()
            } else {
                render.construction.clone()
            };
        }
        presentation.applied_stage = construction_stage;
        presentation.applied_level = state.level;
    }
}

pub(crate) fn sync_chimney_smoke_emitters(
    mut commands: Commands,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    simulation: Res<SimulationRuntime>,
    signatures: Res<BuildingVisualSignatures>,
    added_buildings: Query<(), Added<RuntimeBuilding>>,
    mut sync: Local<BuildingVisualSyncState>,
    mut buildings: Query<(Entity, &RuntimeBuilding, Option<&mut ChimneySmokeEmitters>)>,
) {
    let signature = signatures.structure;
    if sync.initialized && sync.signature == signature && added_buildings.is_empty() {
        return;
    }
    sync.initialized = true;
    sync.signature = signature;
    for (entity, runtime, existing) in &mut buildings {
        let Some(state) = simulation.0.buildings.get(&runtime.id) else {
            continue;
        };
        let Some((building_id, definition)) = content
            .0
            .buildings
            .iter()
            .find(|(_, building)| building.archetype == state.archetype)
        else {
            continue;
        };
        let archetype = &content.0.archetypes[&definition.archetype];
        let age = building_age(&content.0, &simulation.0, building_id, state.level);
        let matches = existing.as_ref().is_some_and(|emitters| {
            emitters.prefab_guid == archetype.source_guid && emitters.age == age
        });
        if matches {
            continue;
        }
        let emitters = presentation
            .0
            .prefab_chimney_emitters
            .get(&archetype.source_guid)
            .into_iter()
            .flatten()
            .filter(|binding| binding.age == age)
            .map(|binding| ChimneySmokeEmitterRuntime {
                effect: binding.effect.clone(),
                local_position: Vec3::from_array(binding.local_position),
                emission_accumulator: 0.0,
                sequence: 0,
            })
            .collect::<Vec<_>>();
        if emitters.is_empty() {
            commands.entity(entity).remove::<ChimneySmokeEmitters>();
        } else {
            commands.entity(entity).insert(ChimneySmokeEmitters {
                prefab_guid: archetype.source_guid.clone(),
                age,
                emitters,
            });
        }
    }
}

pub(crate) fn chimney_particle_seed(
    building: &StableId,
    emitter: usize,
    sequence: u32,
    salt: u32,
) -> u32 {
    building
        .as_str()
        .bytes()
        .chain(
            u64::try_from(emitter)
                .expect("chimney-emitter index fits u64")
                .to_le_bytes(),
        )
        .chain(sequence.to_le_bytes())
        .chain(salt.to_le_bytes())
        .fold(2_166_136_261_u32, |hash, byte| {
            hash.wrapping_mul(16_777_619) ^ u32::from(byte)
        })
}

pub(crate) fn chimney_emitter_world_position(
    transform: &GlobalTransform,
    local_position: Vec3,
) -> Vec3 {
    transform.transform_point(local_position)
}

pub(crate) fn chimney_emitter_world_scale(transform: &GlobalTransform) -> f32 {
    let (scale, _, _) = transform.to_scale_rotation_translation();
    (scale.x.abs() + scale.y.abs() + scale.z.abs()) / 3.0
}

pub(crate) fn spawn_chimney_smoke_particle(
    commands: &mut Commands,
    render: &RenderAssets,
    effect_id: &StableId,
    effect: &ChimneySmokeDef,
    origin: Vec3,
    building: &StableId,
    emitter_index: usize,
    sequence: u32,
    world_scale: f32,
    building_entity: Entity,
) {
    let angle = deterministic_unit(chimney_particle_seed(building, emitter_index, sequence, 11))
        * std::f32::consts::TAU;
    let particle_scale = if effect.world_space { world_scale } else { 1.0 };
    let radius = effect.cone_radius
        * particle_scale
        * deterministic_unit(chimney_particle_seed(building, emitter_index, sequence, 13)).sqrt();
    let direction_angle = effect.cone_angle_degrees.to_radians()
        * deterministic_unit(chimney_particle_seed(building, emitter_index, sequence, 17));
    let horizontal = Vec3::new(angle.cos(), 0.0, angle.sin());
    let direction = Vec3::Y * direction_angle.cos() + horizontal * direction_angle.sin();
    let color_variant =
        usize::from(chimney_particle_seed(building, emitter_index, sequence, 19) & 1 != 0);
    let Some(materials) = render.chimney_smoke.get(effect_id) else {
        return;
    };
    let base_scale = Vec3::splat(effect.start_size * particle_scale);
    let mut particle = commands.spawn((
        WorldEntity,
        ChimneySmokeParticle {
            effect: effect_id.clone(),
            elapsed_seconds: 0.0,
            duration_seconds: effect.lifetime_seconds,
            origin: origin + horizontal * radius,
            velocity: direction * effect.start_speed * particle_scale,
            base_scale,
            size_over_lifetime: effect.size_over_lifetime,
            color_variant,
        },
        Mesh3d(render.chimney_particle.clone()),
        MeshMaterial3d(materials[color_variant][0].clone()),
        bevy::light::NotShadowCaster,
        bevy::light::NotShadowReceiver,
        Transform::from_translation(origin + horizontal * radius)
            .with_scale(base_scale * effect.size_over_lifetime[0]),
    ));
    if !effect.world_space {
        particle.insert(ChildOf(building_entity));
    }
}

pub(crate) fn chimney_emission_count(
    accumulator: &mut f32,
    rate_per_second: f32,
    delta_seconds: f32,
    max_particles: u16,
) -> u16 {
    *accumulator += rate_per_second * delta_seconds;
    let mut count = 0_u16;
    while *accumulator >= 1.0 && count < max_particles {
        *accumulator -= 1.0;
        count += 1;
    }
    count
}

pub(crate) fn emit_chimney_smoke(
    mut commands: Commands,
    time: Res<Time>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    simulation: Res<SimulationRuntime>,
    mut buildings: Query<(
        Entity,
        &RuntimeBuilding,
        &GlobalTransform,
        &mut ChimneySmokeEmitters,
    )>,
) {
    for (building_entity, runtime, transform, mut emitters) in &mut buildings {
        let Some(building) = simulation.0.buildings.get(&runtime.id) else {
            continue;
        };
        if !building.complete || building.health <= 0 {
            for emitter in &mut emitters.emitters {
                emitter.emission_accumulator = 0.0;
            }
            continue;
        }
        for (emitter_index, emitter) in emitters.emitters.iter_mut().enumerate() {
            let Some(effect) = presentation.0.chimney_smoke_effects.get(&emitter.effect) else {
                continue;
            };
            let count = chimney_emission_count(
                &mut emitter.emission_accumulator,
                effect.emission_rate_per_second,
                time.delta_secs(),
                effect.max_particles,
            );
            let origin = if effect.world_space {
                chimney_emitter_world_position(transform, emitter.local_position)
            } else {
                emitter.local_position
            };
            let world_scale = chimney_emitter_world_scale(transform);
            for _ in 0..count {
                spawn_chimney_smoke_particle(
                    &mut commands,
                    &render,
                    &emitter.effect,
                    effect,
                    origin,
                    &runtime.id,
                    emitter_index,
                    emitter.sequence,
                    world_scale,
                    building_entity,
                );
                emitter.sequence = emitter.sequence.wrapping_add(1);
            }
        }
    }
}

pub(crate) fn chimney_particle_scale(effect: &ChimneySmokeParticle, progress: f32) -> Vec3 {
    let size = effect.size_over_lifetime[0]
        + (effect.size_over_lifetime[1] - effect.size_over_lifetime[0]) * progress;
    effect.base_scale * size
}

pub(crate) fn chimney_alpha_progress(step: usize) -> f32 {
    match step {
        0 => 0.0,
        1 => 1.0 / 7.0,
        2 => 2.0 / 7.0,
        3 => 3.0 / 7.0,
        4 => 4.0 / 7.0,
        5 => 5.0 / 7.0,
        6 => 6.0 / 7.0,
        _ => 1.0,
    }
}

pub(crate) fn chimney_alpha_step(progress: f32) -> usize {
    match progress.clamp(0.0, 1.0) {
        value if value < 1.0 / 8.0 => 0,
        value if value < 2.0 / 8.0 => 1,
        value if value < 3.0 / 8.0 => 2,
        value if value < 4.0 / 8.0 => 3,
        value if value < 5.0 / 8.0 => 4,
        value if value < 6.0 / 8.0 => 5,
        value if value < 7.0 / 8.0 => 6,
        _ => 7,
    }
}

pub(crate) fn animate_chimney_smoke_particles(
    mut commands: Commands,
    time: Res<Time>,
    render: Res<RenderAssets>,
    mut particles: Query<(
        Entity,
        &mut ChimneySmokeParticle,
        &mut Transform,
        &mut MeshMaterial3d<StandardMaterial>,
    )>,
) {
    for (entity, mut particle, mut transform, mut material) in &mut particles {
        particle.elapsed_seconds += time.delta_secs();
        let progress = particle.elapsed_seconds / particle.duration_seconds;
        if progress >= 1.0 {
            commands.entity(entity).try_despawn();
            continue;
        }
        transform.translation = particle.origin + particle.velocity * particle.elapsed_seconds;
        transform.scale = chimney_particle_scale(&particle, progress);
        if let Some(variants) = render.chimney_smoke.get(&particle.effect) {
            material.0 = variants[particle.color_variant][chimney_alpha_step(progress)].clone();
        }
        let rotation_speed = if particle.color_variant == 0 {
            0.35
        } else {
            0.55
        };
        transform.rotation *= Quat::from_rotation_y(time.delta_secs() * rotation_speed);
    }
}

pub(crate) fn building_model_node_names(content: &ContentCatalog) -> BTreeSet<String> {
    content
        .buildings
        .values()
        .flat_map(|building| {
            building
                .model_handlers
                .iter()
                .flat_map(|model| {
                    std::iter::once(model.full_model.clone())
                        .chain(model.construction_stages.iter().cloned())
                        .chain(model.upgrades.iter().cloned())
                        .chain(model.other_models.iter().cloned())
                })
                .chain(building.storage_models.iter().flat_map(|model| {
                    [
                        model.empty_model.clone(),
                        model.half_full_model.clone(),
                        model.full_model.clone(),
                    ]
                }))
        })
        .collect()
}

#[allow(clippy::type_complexity)]
pub(crate) fn tag_building_model_nodes(
    mut commands: Commands,
    content: Res<RuntimeContent>,
    buildings: Query<Entity, With<RuntimeBuilding>>,
    parents: Query<&ChildOf>,
    nodes: Query<
        (Entity, &Name),
        (
            Without<BuildingModelNodeProcessed>,
            Without<RuntimeBuilding>,
        ),
    >,
) {
    if nodes.is_empty() {
        return;
    }
    let names = building_model_node_names(&content.0);
    for (entity, name) in &nodes {
        if !names.contains(name.as_str()) {
            commands
                .entity(entity)
                .try_insert(BuildingModelNodeProcessed);
            continue;
        }
        let mut ancestor = entity;
        let mut traversed_parent = false;
        for _ in 0..64 {
            let Ok(parent) = parents.get(ancestor) else {
                if traversed_parent {
                    commands
                        .entity(entity)
                        .try_insert(BuildingModelNodeProcessed);
                }
                break;
            };
            traversed_parent = true;
            ancestor = parent.parent();
            if buildings.contains(ancestor) {
                commands.entity(entity).try_insert(BuildingModelNode {
                    building_root: ancestor,
                    name: name.as_str().to_owned(),
                });
                commands
                    .entity(entity)
                    .try_insert(BuildingModelNodeProcessed);
                break;
            }
        }
    }
}

pub(crate) fn authored_rotating_node_names(content: &ContentCatalog) -> BTreeSet<String> {
    content
        .archetypes
        .values()
        .flat_map(|archetype| {
            archetype
                .rotating_nodes
                .iter()
                .map(|rotating| rotating.node.clone())
        })
        .collect()
}

#[allow(clippy::type_complexity)]
pub(crate) fn tag_authored_rotating_nodes(
    mut commands: Commands,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    buildings: Query<&RuntimeBuilding>,
    parents: Query<&ChildOf>,
    nodes: Query<
        (Entity, &Name),
        (
            Without<AuthoredRotatingNodeProcessed>,
            Without<RuntimeBuilding>,
        ),
    >,
) {
    if nodes.is_empty() {
        return;
    }
    let names = authored_rotating_node_names(&content.0);
    for (entity, name) in &nodes {
        if !names.contains(name.as_str()) {
            commands
                .entity(entity)
                .insert(AuthoredRotatingNodeProcessed);
            continue;
        }
        let mut ancestor = entity;
        let mut traversed_parent = false;
        for _ in 0..64 {
            let Ok(parent) = parents.get(ancestor) else {
                if traversed_parent {
                    commands
                        .entity(entity)
                        .insert(AuthoredRotatingNodeProcessed);
                }
                break;
            };
            traversed_parent = true;
            ancestor = parent.parent();
            let Ok(runtime) = buildings.get(ancestor) else {
                continue;
            };
            let Some(state) = simulation.0.buildings.get(&runtime.id) else {
                break;
            };
            let Some(archetype) = content
                .0
                .buildings
                .values()
                .find(|building| building.archetype == state.archetype)
                .and_then(|building| content.0.archetypes.get(&building.archetype))
            else {
                break;
            };
            if let Some(rotating) = archetype
                .rotating_nodes
                .iter()
                .find(|rotating| rotating.node == name.as_str())
            {
                commands.entity(entity).insert(AuthoredRotatingNode {
                    building_root: ancestor,
                    age: rotating.age.expect("building rotors have validated ages"),
                    axis: Vec3::from_array(rotating.axis),
                    radians_per_second: rotating.degrees_per_second.to_radians(),
                });
            }
            commands
                .entity(entity)
                .insert(AuthoredRotatingNodeProcessed);
            break;
        }
    }
}

pub(crate) fn apply_authored_local_rotation(
    transform: &mut Transform,
    axis: Vec3,
    radians_per_second: f32,
    delta_seconds: f32,
) {
    let euler = axis * radians_per_second * delta_seconds;
    // Unity's `Quaternion.Euler` contract applies Z, then X, then Y.
    transform.rotation *= Quat::from_euler(EulerRot::ZXY, euler.z, euler.x, euler.y);
}

pub(crate) fn rotate_authored_building_nodes(
    time: Res<Time>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    buildings: Query<&RuntimeBuilding>,
    mut nodes: Query<(&AuthoredRotatingNode, &mut Transform)>,
) {
    for (node, mut transform) in &mut nodes {
        let Ok(runtime) = buildings.get(node.building_root) else {
            continue;
        };
        let Some(state) = simulation.0.buildings.get(&runtime.id) else {
            continue;
        };
        let Some((building_id, _)) = content
            .0
            .buildings
            .iter()
            .find(|(_, building)| building.archetype == state.archetype)
        else {
            continue;
        };
        if !state.complete
            || building_age(&content.0, &simulation.0, building_id, state.level) != node.age
        {
            continue;
        }
        apply_authored_local_rotation(
            &mut transform,
            node.axis,
            node.radians_per_second,
            time.delta_secs(),
        );
    }
}

pub(crate) fn model_contains_node(model: &BuildingModelDef, name: &str) -> bool {
    model.full_model == name
        || model.construction_stages.iter().any(|node| node == name)
        || model.upgrades.iter().any(|node| node == name)
        || model.other_models.iter().any(|node| node == name)
}

pub(crate) fn model_node_visible(
    model: &BuildingModelDef,
    construction_stage: u8,
    name: &str,
) -> bool {
    if construction_stage < 3 {
        return model.construction_stages[usize::from(construction_stage)] == name;
    }
    model.full_model == name
        || model.upgrades.iter().any(|node| node == name)
        || model.other_models.iter().any(|node| node == name)
}

pub(crate) fn tiled_neighbor_value(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    state: &BuildingState,
) -> u8 {
    simulation
        .buildings
        .values()
        .filter(|other| other.id != state.id)
        .filter(|other| {
            content.buildings.iter().any(|(id, definition)| {
                definition.archetype == other.archetype
                    && matches!(id.as_str(), "building:wall" | "building:gate")
            })
        })
        .fold(0_u8, |value, other| {
            value
                | match (
                    i32::from(other.position.x) - i32::from(state.position.x),
                    i32::from(other.position.z) - i32::from(state.position.z),
                ) {
                    // The Blender conversion mirrors the authored FBX local X
                    // basis. Feed the Unity tiler its reflected east/west bits
                    // so corner and T-junction geometry still faces its actual
                    // Bevy-world neighbours.
                    (1, 0) => 8,
                    (-1, 0) => 2,
                    (0, 1) => 16,
                    (0, -1) => 4,
                    _ => 0,
                }
        })
}

pub(crate) fn wall_tiling(tile_value: u8) -> (usize, i32) {
    match tile_value {
        0 | 2 | 8 | 10 => (0, 1),
        4 | 16 | 20 => (0, 0),
        18 => (1, 0),
        6 => (1, 1),
        12 => (1, 2),
        24 => (1, 3),
        22 => (3, 0),
        14 => (3, 1),
        28 => (3, 2),
        26 => (3, 3),
        _ => (2, 0),
    }
}

pub(crate) fn gate_tiling(tile_value: u8) -> i32 {
    match tile_value {
        0 | 2 | 6 | 8 | 10 | 14 => 1,
        12 | 28 => 2,
        24 | 26 => 3,
        _ => 0,
    }
}

pub(crate) fn storage_node_visible(
    model: &StorageModelDef,
    amount: u32,
    capacity: u32,
    name: &str,
) -> bool {
    if amount >= capacity {
        model.full_model == name
    } else if u64::from(amount).saturating_mul(2) >= u64::from(capacity) {
        model.half_full_model == name
    } else {
        model.empty_model == name
    }
}

pub(crate) fn building_node_visibility(
    building_id: &StableId,
    definition: &BuildingDef,
    state: &BuildingState,
    age: u8,
    name: &str,
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> Option<bool> {
    let model_index = if building_id.as_str() == "building:wall" {
        wall_tiling(tiled_neighbor_value(content, simulation, state)).0
    } else {
        0
    };
    let active_model = definition
        .model_handlers
        .iter()
        .filter(|model| model.age == age)
        .nth(model_index)
        .or_else(|| {
            definition
                .model_handlers
                .iter()
                .rev()
                .find(|model| model.age == age)
        })
        .or_else(|| {
            let fallback_age = definition
                .model_handlers
                .iter()
                .filter(|model| model.age <= age)
                .map(|model| model.age)
                .max()?;
            definition
                .model_handlers
                .iter()
                .filter(|model| model.age == fallback_age)
                .nth(model_index)
                .or_else(|| {
                    definition
                        .model_handlers
                        .iter()
                        .rev()
                        .find(|model| model.age == fallback_age)
                })
        });
    if let Some(model) = definition
        .model_handlers
        .iter()
        .find(|model| model_contains_node(model, name))
    {
        return Some(
            active_model.is_some_and(|active| std::ptr::eq(active, model))
                && model_node_visible(
                    model,
                    building_construction_stage(
                        state.health,
                        building_max_health(content, state),
                        state.complete,
                    ),
                    name,
                ),
        );
    }
    if let Some(model) = definition.storage_models.iter().find(|model| {
        model.empty_model == name || model.half_full_model == name || model.full_model == name
    }) {
        if !state.complete || model.age != age {
            return Some(false);
        }
        let amount = simulation
            .town_resources
            .get(&model.resource)
            .copied()
            .unwrap_or_default();
        let capacity = resource_storage_capacity(config, content, simulation, &model.resource);
        return Some(storage_node_visible(model, amount, capacity, name));
    }
    None
}

pub(crate) fn sync_building_model_nodes(
    content: Res<RuntimeContent>,
    config: Res<RuntimeConfig>,
    simulation: Res<SimulationRuntime>,
    signatures: Res<BuildingVisualSignatures>,
    buildings: Query<&RuntimeBuilding>,
    added_nodes: Query<(), Added<BuildingModelNode>>,
    mut sync: Local<BuildingVisualSyncState>,
    mut nodes: Query<(&BuildingModelNode, &mut Visibility)>,
) {
    let signature = signatures.model_visibility;
    if sync.initialized && sync.signature == signature && added_nodes.is_empty() {
        return;
    }
    sync.initialized = true;
    sync.signature = signature;
    for (node, mut visibility) in &mut nodes {
        let Ok(runtime) = buildings.get(node.building_root) else {
            continue;
        };
        let Some(state) = simulation.0.buildings.get(&runtime.id) else {
            continue;
        };
        let Some((building_id, definition)) = content
            .0
            .buildings
            .iter()
            .find(|(_, building)| building.archetype == state.archetype)
        else {
            continue;
        };
        let age = building_age(&content.0, &simulation.0, building_id, state.level);
        let Some(visible) = building_node_visibility(
            building_id,
            definition,
            state,
            age,
            &node.name,
            &config.0,
            &content.0,
            &simulation.0,
        ) else {
            continue;
        };
        let desired = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != desired {
            *visibility = desired;
        }
    }
}

pub(crate) fn sync_tiled_building_rotation(
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    signatures: Res<BuildingVisualSignatures>,
    added_buildings: Query<(), Added<RuntimeBuilding>>,
    mut sync: Local<BuildingVisualSyncState>,
    mut buildings: Query<(&RuntimeBuilding, &mut Transform)>,
) {
    let signature = signatures.structure;
    if sync.initialized && sync.signature == signature && added_buildings.is_empty() {
        return;
    }
    sync.initialized = true;
    sync.signature = signature;
    for (runtime, mut transform) in &mut buildings {
        let Some(state) = simulation.0.buildings.get(&runtime.id) else {
            continue;
        };
        let Some((building_id, _)) = content
            .0
            .buildings
            .iter()
            .find(|(_, building)| building.archetype == state.archetype)
        else {
            continue;
        };
        let quarter_turns = match building_id.as_str() {
            "building:wall" => {
                wall_tiling(tiled_neighbor_value(&content.0, &simulation.0, state)).1
            }
            "building:gate" => gate_tiling(tiled_neighbor_value(&content.0, &simulation.0, state)),
            _ => state.rotation_quarter_turns,
        };
        let desired = quarter_turn_rotation(quarter_turns);
        if transform.rotation.angle_between(desired) > 1.0e-6 {
            transform.rotation = desired;
        }
    }
}
