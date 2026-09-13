pub(crate) fn complete_agent_goal_with_regeneration(
    simulation: &mut WorldSimulation,
    world: &mut GeneratedWorld,
    config: &GameConfig,
    content: &ContentCatalog,
    regeneration: &mut RegenerationRoleRuntime,
    actor_id: &StableId,
    goal: &AgentGoal,
    current: GridPos,
) -> Option<ActionPresentation> {
    if !simulation.actors.contains_key(actor_id) {
        return None;
    }
    let stats = simulation
        .actors
        .get(actor_id)
        .and_then(|actor| effective_role_stats(content, simulation, actor));
    let action_amount = simulation
        .actors
        .get(actor_id)
        .and_then(|actor| actor_archetype(content, actor))
        .and_then(|archetype| archetype.enemy.as_ref())
        .map_or_else(
            || stats.map_or(1, |stats| stats.action_amount),
            |enemy| enemy.action_amount,
        );
    let mut action_presentation = None;
    let action_succeeded = match goal {
        AgentGoal::VisitRegenerationStation(building_id) => {
            let actor = simulation.actors.get(actor_id)?;
            if !matches!(actor.role.as_str(), "role:forester" | "role:tender") {
                return None;
            }
            let station = simulation.buildings.get(building_id)?;
            if !station.complete
                || !building_def_for_archetype(content, &station.archetype).is_some_and(
                    |definition| {
                        definition
                            .role_slots
                            .iter()
                            .any(|slot| slot.role == actor.role)
                    },
                )
                || !is_current_building_approach(world, content, station, current)
            {
                return None;
            }
            let worker = regeneration.workers.entry(actor_id.clone()).or_default();
            worker.last_station = Some(building_id.clone());
            worker.station_visit_required = false;
            if let Some(actor) = simulation.actors.get_mut(actor_id) {
                actor.station = Some(building_id.clone());
            }
            true
        }
        AgentGoal::Gather(resource_id) => {
            let actor = simulation.actors.get(actor_id)?;
            if !actor_resource_storage_has_room(config, content, simulation, actor) {
                return None;
            }
            if !world
                .resources
                .iter()
                .find(|resource| resource.id == *resource_id && resource.amount > 0)
                .is_some_and(|resource| {
                    actor_accepts_resource(content, actor, resource)
                        && resource_approach(world, resource, current) == Some(current)
                })
            {
                return None;
            }
            let remaining_carry = actor_remaining_carry_capacity(content, simulation, actor);
            if remaining_carry == 0 {
                return None;
            }
            let gathering_pet = simulation.actors.get(actor_id).and_then(|actor| {
                actor.id.as_str().starts_with("twitch:").then_some(())?;
                match actor.role.as_str() {
                    "role:gatherer" => StableId::new("pet:giraffe").ok(),
                    "role:fisher" => StableId::new("pet:duck").ok(),
                    "role:logger" => StableId::new("pet:butterfly").ok(),
                    _ => None,
                }
            });
            let resource = world
                .resources
                .iter_mut()
                .find(|resource| resource.id == *resource_id && resource.amount > 0)?;
            let amount = resource.amount.min(action_amount);
            let stored_amount = amount.min(remaining_carry);
            resource.amount -= amount;
            let resource_kind = resource.kind.clone();
            let cleared_tree = (resource.amount == 0
                && resource.target_kind.as_str() == "target:tree")
                .then_some(resource.position);
            let cleared_position = (resource.amount == 0
                && resource.target_kind.as_str() != "target:fish")
                .then_some(resource.position);
            if let Err(error) = simulation.gather(actor_id, resource_kind, stored_amount) {
                warn!(actor = %actor_id, %error, "resource gather action failed");
                resource.amount = resource.amount.saturating_add(amount);
                false
            } else {
                if let Some(position) = cleared_position
                    && !resource_cell_has_active_generation_occupant(&world.resources, position)
                {
                    let _ = world.navigation.set_blocked(
                        stream_town_domain::DirtyRegion {
                            min: position,
                            max: position,
                        },
                        false,
                    );
                }
                if let Some(position) = cleared_tree {
                    regeneration.recently_fallen_trees.push_back(position);
                    while regeneration.recently_fallen_trees.len() > 128 {
                        regeneration.recently_fallen_trees.pop_front();
                    }
                }
                if amount > 0
                    && let Some(pet) = gathering_pet
                    && simulation
                        .try_unlock_gathering_pet(actor_id, pet.clone())
                        .unwrap_or(false)
                {
                    info!(actor = %actor_id, %pet, "unlocked gathering pet");
                }
                amount > 0
            }
        }
        AgentGoal::HarvestFarm(building_id) => {
            let actor = simulation.actors.get(actor_id)?;
            if !actor_resource_storage_has_room(config, content, simulation, actor)
                || !content.roles.get(&actor.role).is_some_and(|role| {
                    role.resource
                        .as_ref()
                        .is_some_and(|resource| resource.as_str() == "resource:food")
                        && role_accepts_target(
                            role,
                            &StableId::new("target:farm").expect("static target ID"),
                        )
                })
                || !simulation
                    .buildings
                    .get(building_id)
                    .is_some_and(|building| {
                        is_farm_resource_building(content, building)
                            && is_current_building_approach(world, content, building, current)
                    })
            {
                return None;
            }
            let remaining_carry = actor_remaining_carry_capacity(content, simulation, actor);
            if remaining_carry == 0 {
                return None;
            }
            let amount = action_amount.min(remaining_carry);
            let food = StableId::new("resource:food").expect("static resource ID");
            match simulation.gather(actor_id, food, amount) {
                Ok(()) => amount > 0,
                Err(error) => {
                    warn!(actor = %actor_id, building = %building_id, %error, "farm harvest action failed");
                    false
                }
            }
        }
        AgentGoal::Deposit => {
            let resource = simulation
                .actors
                .get(actor_id)
                .and_then(|actor| resource_for_role(content, &actor.role))?;
            let capacity = resource_storage_capacity(config, content, simulation, &resource);
            match simulation.deposit_resource_with_market(actor_id, &resource, capacity) {
                Ok((deposited, traded, gold)) => {
                    let gathered = deposited.saturating_add(traded);
                    if gathered > 0 {
                        let _ = simulation.record_objective_event(
                            &content.objectives,
                            &ObjectiveEvent::ResourceGathered {
                                resource: resource.clone(),
                                amount: gathered,
                            },
                        );
                    }
                    if traded > 0 {
                        let _ = simulation.record_objective_event(
                            &content.objectives,
                            &ObjectiveEvent::ResourceSold {
                                resource,
                                amount: traded,
                            },
                        );
                    }
                    if gold > 0 {
                        let _ = simulation.record_objective_event(
                            &content.objectives,
                            &ObjectiveEvent::ResourceGained {
                                resource: StableId::new("resource:gold").expect("static stable ID"),
                                amount: gold,
                            },
                        );
                    }
                    gathered > 0
                }
                Err(error) => {
                    warn!(actor = %actor_id, %error, "resource deposit action failed");
                    false
                }
            }
        }
        AgentGoal::Attack(target_id) => {
            let attacker = simulation.actors.get(actor_id)?;
            let target = simulation.actors.get(target_id)?;
            let visual = actor_combat_visual(&attacker.role);
            let target_position = target.position;
            if !attacker.alive
                || !target.alive
                || !within_actor_attack_range(content, simulation, attacker, target, current)
            {
                return None;
            }
            let damage = action_amount;
            if is_ranged_role(&attacker.role) {
                action_presentation = Some(ActionPresentation::Projectile(ProjectileSpawn {
                    source: ProjectileSource::Actor(actor_id.clone()),
                    target: target_id.clone(),
                    damage,
                    speed_cells_per_second: 12.0,
                    visual,
                }));
                damage > 0
            } else {
                match apply_combat_damage(
                    config,
                    simulation,
                    content,
                    Some(actor_id),
                    target_id,
                    damage,
                ) {
                    Ok(_) => {
                        if damage > 0 {
                            action_presentation = Some(ActionPresentation::Impact {
                                target: target_position,
                                visual,
                            });
                            true
                        } else {
                            false
                        }
                    }
                    Err(error) => {
                        warn!(actor = %actor_id, target = %target_id, %error, "combat action failed");
                        false
                    }
                }
            }
        }
        AgentGoal::AttackBuilding(building_id) => {
            let attacker = simulation.actors.get(actor_id)?;
            let building = simulation.buildings.get(building_id)?;
            if !enemy_can_attack_building(content, building) {
                return None;
            }
            if !attacker.alive
                || building.health <= 0
                || !within_enemy_building_attack_range(
                    content, simulation, attacker, building, current,
                )
            {
                return None;
            }
            let building_position = building_visual_grid(content, building);
            let building_origin = building.position;
            let building_rotation = building.rotation_quarter_turns;
            let building_definition =
                building_def_for_archetype(content, &building.archetype).cloned();
            match simulation.damage_building(building_id, action_amount) {
                Ok(remaining) if action_amount > 0 => {
                    if remaining == 0 {
                        if let Some(definition) = building_definition
                            .as_ref()
                            .filter(|definition| building_blocks_navigation(definition))
                            && let Some(region) = building_navigation_region(
                                building_origin,
                                definition,
                                building_rotation,
                                world,
                            )
                        {
                            let _ = world.navigation.set_blocked(region, false);
                        }
                        simulation.buildings.remove(building_id);
                        simulation.path_navigation_positions.remove(building_id);
                        simulation.building_night_light_colors.remove(building_id);
                        for actor in simulation.actors.values_mut() {
                            if actor.station.as_ref() == Some(building_id)
                                || actor.preferred_target.as_ref() == Some(building_id)
                            {
                                actor.station = None;
                                actor.preferred_target = None;
                            }
                        }
                        let reassigned =
                            reassign_roles_without_prerequisite_buildings(content, simulation);
                        if !reassigned.is_empty() {
                            info!(actors = ?reassigned, "reassigned citizens whose last role prerequisite was destroyed");
                        }
                        action_presentation = Some(ActionPresentation::BuildingDestroyed {
                            building: building_id.clone(),
                            target: building_position,
                        });
                    } else {
                        action_presentation = Some(ActionPresentation::BuildingWork {
                            target: building_position,
                            sparks: true,
                        });
                    }
                    true
                }
                Ok(_) => false,
                Err(error) => {
                    warn!(actor = %actor_id, building = %building_id, %error, "building attack failed");
                    false
                }
            }
        }
        AgentGoal::Heal(target_id) => {
            let healer = simulation.actors.get(actor_id)?;
            let target = simulation.actors.get(target_id)?;
            if !healer.alive
                || !target.alive
                || target.role.as_str() == "role:enemy"
                || target.health >= target.max_health
                || !within_actor_heal_hysteresis_range(content, simulation, healer, target, current)
            {
                return None;
            }
            match simulation.heal_actor(target_id, action_amount) {
                Ok(restored) => {
                    if restored > 0 {
                        action_presentation = Some(ActionPresentation::Healing {
                            source: actor_id.clone(),
                            target: target_id.clone(),
                        });
                        true
                    } else {
                        false
                    }
                }
                Err(error) => {
                    warn!(actor = %actor_id, target = %target_id, %error, "healing action failed");
                    false
                }
            }
        }
        AgentGoal::Construct(building_id) => {
            let builder = simulation.actors.get(actor_id)?;
            let building_before = simulation.buildings.get(building_id)?;
            let was_complete = building_before.complete;
            let needs_work = if was_complete {
                building_before.health < building_max_health(content, building_before)
            } else {
                true
            };
            let within_range =
                within_building_work_range(content, simulation, builder, building_before, current);
            if !needs_work || !within_range {
                debug!(
                    actor = %actor_id,
                    building = %building_id,
                    ?current,
                    needs_work,
                    within_range,
                    "builder action was rejected at completion"
                );
                return None;
            }
            let building_position = simulation
                .buildings
                .get(building_id)
                .map(|building| building_visual_grid(content, building));
            let archetype = simulation
                .buildings
                .get(building_id)
                .map(|building| building.archetype.clone());
            let max_health = simulation
                .buildings
                .get(building_id)
                .map_or(BUILDING_MAX_HEALTH, |building| {
                    building_max_health(content, building)
                });
            let max_health = u32::try_from(max_health).unwrap_or(u32::MAX);
            let result = if was_complete {
                simulation
                    .repair_building(building_id, action_amount, max_health)
                    .map(|restored| (false, restored > 0))
            } else {
                simulation
                    .work_on_building(building_id, action_amount, max_health)
                    .map(|complete| (complete, action_amount > 0))
            };
            match result {
                Ok((complete, succeeded)) => {
                    debug!(
                        actor = %actor_id,
                        building = %building_id,
                        action_amount,
                        was_complete,
                        complete,
                        succeeded,
                        "builder action applied"
                    );
                    if !was_complete
                        && complete
                        && let Some(building) = archetype.as_ref().and_then(|archetype| {
                            content.buildings.iter().find_map(|(id, definition)| {
                                (definition.archetype == *archetype).then_some(id.clone())
                            })
                        })
                    {
                        let _ = simulation.record_objective_event(
                            &content.objectives,
                            &ObjectiveEvent::BuildingBuilt(building),
                        );
                    }
                    if succeeded && let Some(target) = building_position {
                        action_presentation = Some(ActionPresentation::BuildingWork {
                            target,
                            sparks: true,
                        });
                    }
                    succeeded
                }
                Err(error) => {
                    warn!(actor = %actor_id, building = %building_id, %error, "construction action failed");
                    false
                }
            }
        }
        AgentGoal::PlantTree(_) | AgentGoal::Prospect { .. } | AgentGoal::PlantBush(_) => {
            complete_regeneration_goal(simulation, world, content, regeneration, actor_id, goal)
        }
        AgentGoal::WaitForStorage | AgentGoal::Wander => false,
    };
    let grants_role_experience = action_succeeded
        && match goal {
            AgentGoal::Gather(_)
            | AgentGoal::HarvestFarm(_)
            | AgentGoal::Attack(_)
            | AgentGoal::Heal(_)
            | AgentGoal::PlantTree(_)
            | AgentGoal::Prospect { .. }
            | AgentGoal::PlantBush(_) => true,
            // Unity's Builder action applies its final construction tick, then
            // returns false after the Construction target flag disappears.
            // Completed-building repair and deposits are not PlayerAction
            // successes and therefore do not award role experience.
            AgentGoal::Construct(building) => simulation
                .buildings
                .get(building)
                .is_some_and(|building| !building.complete),
            AgentGoal::AttackBuilding(_)
            | AgentGoal::Deposit
            | AgentGoal::VisitRegenerationStation(_)
            | AgentGoal::WaitForStorage
            | AgentGoal::Wander => false,
        };
    if grants_role_experience
        && let Some(stats) = stats
        && let Ok(levels_gained) = simulation.grant_role_experience_capped(
            actor_id,
            action_amount,
            stats.experience_multiplier_per_thousand,
            actor_role_level_cap(content, actor_id),
            content.progression.role_experience_curve_level_span,
            content.progression.role_experience_curve_maximum,
        )
        && levels_gained > 0
    {
        let max_health = simulation
            .actors
            .get(actor_id)
            .and_then(|actor| effective_role_stats(content, simulation, actor))
            .map_or(1, |stats| stats.max_health);
        if let Some(actor) = simulation.actors.get_mut(actor_id) {
            actor.max_health = i32::try_from(max_health).unwrap_or(i32::MAX);
            actor.health = actor.max_health;
        }
    }
    action_presentation
}
