pub(crate) fn require_staff(pending: &PendingChatCommand) -> Result<(), String> {
    if pending.is_broadcaster || pending.is_moderator {
        Ok(())
    } else {
        Err("this command requires broadcaster or moderator permission".to_owned())
    }
}

pub(crate) fn require_game_master(
    config: &GameConfig,
    pending: &PendingChatCommand,
) -> Result<(), String> {
    if pending.origin == CommandOrigin::LocalDebug {
        return Ok(());
    }
    let raw_user_id = pending
        .actor_id
        .as_str()
        .strip_prefix("twitch:")
        .unwrap_or_else(|| pending.actor_id.as_str());
    if config.twitch.game_master_ids.contains(raw_user_id) {
        Ok(())
    } else {
        Err("this command requires an explicitly configured game-master Twitch user ID".to_owned())
    }
}

pub(crate) fn pending_stream_user_type(
    config: &GameConfig,
    pending: &PendingChatCommand,
) -> StreamUserType {
    let raw_user_id = pending
        .actor_id
        .as_str()
        .strip_prefix("twitch:")
        .unwrap_or_else(|| pending.actor_id.as_str());
    if pending.origin == CommandOrigin::LocalDebug
        || config.twitch.game_master_ids.contains(raw_user_id)
    {
        StreamUserType::GameMaster
    } else if pending.is_broadcaster {
        StreamUserType::Broadcaster
    } else if pending.is_moderator {
        StreamUserType::Moderator
    } else if pending.is_subscriber {
        StreamUserType::Subscriber
    } else {
        StreamUserType::Normal
    }
}

pub(crate) fn require_ruler_or_staff(
    simulation: &WorldSimulation,
    pending: &PendingChatCommand,
) -> Result<(), String> {
    if simulation.is_ruler(&pending.actor_id) {
        Ok(())
    } else {
        require_staff(pending)
            .map_err(|_| "this command is restricted to the Ruler or staff".to_owned())
    }
}

pub(crate) fn require_world_view_operator(
    simulation: &WorldSimulation,
    pending: &PendingChatCommand,
    user_type: StreamUserType,
) -> Result<(), String> {
    if user_type == StreamUserType::GameMaster {
        Ok(())
    } else {
        require_ruler_or_staff(simulation, pending)
    }
}

pub(crate) fn resolve_ruler_vote_option(
    simulation: &WorldSimulation,
    requested: &StableId,
) -> Option<StableId> {
    let vote = simulation.ruler_vote.as_ref()?;
    if vote.kind == RulerVoteKind::KeepRuler {
        return matches!(requested.as_str(), "yes" | "no").then(|| requested.clone());
    }
    if simulation.actors.contains_key(requested) {
        return Some(requested.clone());
    }
    let prefixed = prefixed_id(requested, "twitch:");
    if let Some(actor) = prefixed.filter(|actor| simulation.actors.contains_key(actor)) {
        return Some(actor);
    }
    let normalized = requested.as_str().replace('_', " ");
    simulation
        .actors
        .values()
        .filter(|actor| actor.role.as_str() != "role:enemy" && actor.alive)
        .find(|actor| {
            actor
                .login_name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(requested.as_str()))
                || actor
                    .display_name
                    .as_deref()
                    .is_some_and(|name| name.replace('_', " ").eq_ignore_ascii_case(&normalized))
        })
        .map(|actor| actor.id.clone())
}

pub(crate) fn resolve_community_vote_option(requested: &StableId) -> Option<bool> {
    match requested.as_str() {
        "yes" | "y" | "1" => Some(true),
        "no" | "n" | "2" => Some(false),
        _ => None,
    }
}

pub(crate) fn recruited_actor_ids(simulation: &WorldSimulation) -> Vec<StableId> {
    simulation
        .actors
        .keys()
        .filter(|id| is_recruited_actor_id(id) && !is_guardhouse_defender_id(id))
        .cloned()
        .collect()
}

pub(crate) fn capacity_recruited_actor_ids(simulation: &WorldSimulation) -> Vec<StableId> {
    recruited_actor_ids(simulation)
        .into_iter()
        .filter(|id| !is_guardhouse_defender_id(id))
        .collect()
}

pub(crate) fn is_guardhouse_defender_id(id: &StableId) -> bool {
    id.as_str().starts_with("npc:guardhouse_")
}

pub(crate) fn guardhouse_defender_id(building: &StableId) -> StableId {
    StableId::new(format!("npc:guardhouse_{}", building.as_str()))
        .expect("runtime building IDs form valid guardhouse defender IDs")
}

pub(crate) fn guardhouse_for_defender<'a>(
    simulation: &'a WorldSimulation,
    actor: &StableId,
) -> Option<&'a BuildingState> {
    let building = actor.as_str().strip_prefix("npc:guardhouse_")?;
    let building = StableId::new(building).ok()?;
    simulation
        .buildings
        .get(&building)
        .filter(|building| building.archetype.as_str() == "archetype:building:guardhouse")
}

pub(crate) fn is_recruited_actor_id(id: &StableId) -> bool {
    id.as_str().starts_with("npc:recruit_")
        || id.as_str().starts_with("npc:starting_")
        || id.as_str().starts_with("npc:guardhouse_")
}

pub(crate) fn is_level_capped_npc_id(id: &StableId) -> bool {
    is_recruited_actor_id(id) || id.as_str().starts_with("actor:viewer_")
}

pub(crate) fn actor_role_level_cap(content: &ContentCatalog, id: &StableId) -> u16 {
    if is_level_capped_npc_id(id) {
        content.progression.maximum_recruit_role_level
    } else {
        content.progression.maximum_role_level
    }
}

pub(crate) fn normalize_recruit_role_progression(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
) -> usize {
    let maximum_level = content.progression.maximum_recruit_role_level;
    let mut adjusted = 0;
    for actor in simulation
        .actors
        .values_mut()
        .filter(|actor| is_level_capped_npc_id(&actor.id))
    {
        for progress in actor.role_progression.values_mut() {
            if progress.level > maximum_level
                || (progress.level == maximum_level && progress.experience > 0)
            {
                progress.level = maximum_level;
                progress.experience = 0;
                adjusted += 1;
            }
        }
    }
    adjusted
}

pub(crate) fn enforce_recruit_role_level_cap(
    content: Res<RuntimeContent>,
    mut simulation: ResMut<SimulationRuntime>,
) {
    let adjusted = normalize_recruit_role_progression(&content.0, &mut simulation.0);
    if adjusted > 0 {
        warn!(
            adjusted,
            maximum_level = content.0.progression.maximum_recruit_role_level,
            "clamped recruit role progression"
        );
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn sync_guardhouse_defenders(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    world: Res<WorldRuntime>,
    render: Res<RenderAssets>,
    mut simulation: ResMut<SimulationRuntime>,
    agents: Query<(Entity, &Agent)>,
) {
    let guardhouses = simulation
        .0
        .buildings
        .values()
        .filter(|building| {
            building.complete
                && building.health > 0
                && building.archetype.as_str() == "archetype:building:guardhouse"
        })
        .map(|building| (building.id.clone(), building.position))
        .collect::<Vec<_>>();
    let live_ids = guardhouses
        .iter()
        .map(|(building, _)| guardhouse_defender_id(building))
        .collect::<BTreeSet<_>>();
    let orphaned = simulation
        .0
        .actors
        .keys()
        .filter(|id| is_guardhouse_defender_id(id) && !live_ids.contains(*id))
        .cloned()
        .collect::<Vec<_>>();
    for actor in orphaned {
        simulation.0.actors.remove(&actor);
        if let Some((entity, _)) = agents.iter().find(|(_, agent)| agent.id == actor) {
            commands.entity(entity).try_despawn();
        }
    }

    let defender = StableId::new("role:defender").expect("static defender role ID");
    let actor_archetype =
        archetype_id_by_source(&content.0, ArchetypeKind::Player, "Player_Character.prefab")
            .unwrap_or_else(|| {
                StableId::new("archetype:viewer").expect("static viewer archetype ID")
            });
    let base_scale = Vec3::new(
        config.0.world.cell_size * 0.3,
        config.0.world.cell_size * 0.55,
        config.0.world.cell_size * 0.3,
    );
    for (guardhouse, origin) in guardhouses {
        let actor_id = guardhouse_defender_id(&guardhouse);
        if !simulation.0.actors.contains_key(&actor_id) {
            let position = nearest_walkable(&world.generated, origin).unwrap_or(origin);
            if !simulation.0.join_player(actor_id.clone(), position) {
                continue;
            }
        }
        if simulation
            .0
            .actors
            .get(&actor_id)
            .is_some_and(|actor| actor.role != defender)
        {
            let _ = simulation.0.assign_role(&actor_id, defender.clone());
        }
        let Some(actor) = simulation.0.actors.get_mut(&actor_id) else {
            continue;
        };
        actor.station = Some(guardhouse.clone());
        actor.archetype = Some(actor_archetype.clone());
        actor.display_name = Some("Guardhouse Defender".to_owned());
        actor.login_name = None;
        if agents.iter().any(|(_, agent)| agent.id == actor_id) {
            continue;
        }
        let position = actor.position;
        let target = deterministic_wander_target(&world.generated, &actor_id, position);
        let world_position = grid_to_world_on_surface(position, &config.0, &world.generated);
        commands.spawn((
            WorldEntity,
            GridLocation(position),
            Agent {
                id: actor_id,
                kind: ActorKind::Player,
                archetype: actor_archetype.clone(),
                goal: AgentGoal::Wander,
                spawn: position,
                origin: position,
                navigation_position: placement_to_navigation_centre(position),
                path: Vec::new(),
                path_index: 0,
                target,
                action_cooldown_seconds: 0.0,
                action_started: false,
                repath_remaining_seconds: 0.0,
                health_regen_accumulator: 0.0,
                wander_sequence: 0,
                previous_wander_origin: None,
            },
            AgentLocomotion::default(),
            AgentAnimation {
                base_scale,
                ..default()
            },
            Mesh3d(render.actor_lod.clone()),
            MeshMaterial3d(actor_material(&render, &ActorKind::Player, false)),
            Transform::from_translation(world_position + Vec3::Y * base_scale.y * 0.5)
                .with_scale(base_scale),
        ));
    }
}

pub(crate) fn spawn_numbered_world_labels(commands: &mut Commands, targets: &[StableId]) {
    for (index, target) in targets.iter().enumerate() {
        commands.spawn((
            WorldEntity,
            TemporaryWorldLabel {
                target: target.clone(),
                remaining_seconds: UNITY_NUMBERED_LABEL_SECONDS,
            },
            Text::new((index + 1).to_string()),
            TextFont {
                font_size: FontSize::Px(22.0),
                ..default()
            },
            TextLayout::justify(Justify::Center),
            TextColor(Color::srgb(0.91, 0.89, 0.81)),
            GlobalZIndex(19),
            Node {
                position_type: PositionType::Absolute,
                width: px(40),
                ..default()
            },
        ));
    }
}

pub(crate) fn recruit_id(simulation: &WorldSimulation, index: u16) -> Option<StableId> {
    capacity_recruited_actor_ids(simulation)
        .get(usize::from(index.saturating_sub(1)))
        .cloned()
}

pub(crate) fn role_capacity(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    role: &StableId,
) -> Option<u32> {
    let definition = content.roles.get(role)?;
    if !definition.has_user_limit {
        return None;
    }
    Some(
        simulation
            .buildings
            .values()
            .filter(|state| state.complete)
            .filter_map(|state| {
                building_def_for_archetype(content, &state.archetype)
                    .map(|definition| (definition, state.level))
            })
            .flat_map(|(building, level)| {
                building
                    .role_slots
                    .iter()
                    .filter(move |slots| slots.role == *role)
                    .map(move |slots| {
                        u32::from(slots.base_amount).saturating_add(
                            u32::from(slots.increment_amount)
                                .saturating_mul(u32::from(level.saturating_sub(1))),
                        )
                    })
            })
            .fold(u32::from(definition.base_max_users), u32::saturating_add),
    )
}

pub(crate) fn reassign_roles_without_prerequisite_buildings(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
) -> Vec<StableId> {
    let defender = StableId::new("role:defender").expect("static defender role ID");
    let affected = simulation
        .actors
        .values()
        .filter(|actor| {
            !matches!(
                actor.role.as_str(),
                "role:defender" | "role:enemy" | "role:ruler"
            ) && !is_guardhouse_defender_id(&actor.id)
                && role_capacity(content, simulation, &actor.role) == Some(0)
        })
        .map(|actor| actor.id.clone())
        .collect::<Vec<_>>();
    for actor in &affected {
        let _ = simulation.assign_role(actor, defender.clone());
    }
    affected
}

pub(crate) fn role_is_available(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    role: &StableId,
    excluding: Option<&StableId>,
) -> bool {
    if !simulation.role_limits_enabled {
        return true;
    }
    role_capacity(content, simulation, role).is_none_or(|capacity| {
        let used = simulation
            .actors
            .values()
            .filter(|actor| {
                actor.role == *role
                    && excluding != Some(&actor.id)
                    && !is_guardhouse_defender_id(&actor.id)
            })
            .count();
        used < usize::try_from(capacity).unwrap_or(usize::MAX)
    })
}

pub(crate) fn resolve_player_id(
    simulation: &WorldSimulation,
    requested: &StableId,
) -> Option<StableId> {
    if simulation.actors.contains_key(requested) {
        return Some(requested.clone());
    }
    let prefixed = prefixed_id(requested, "twitch:");
    if let Some(actor) = prefixed.filter(|actor| simulation.actors.contains_key(actor)) {
        return Some(actor);
    }
    let normalized = requested.as_str().replace('_', " ");
    simulation
        .actors
        .values()
        .find(|actor| {
            actor
                .login_name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(requested.as_str()))
                || actor
                    .display_name
                    .as_deref()
                    .is_some_and(|name| name.replace('_', " ").eq_ignore_ascii_case(&normalized))
        })
        .map(|actor| actor.id.clone())
}

pub(crate) fn resolve_auto_camera_follow_target(
    simulation: &WorldSimulation,
    caller: &StableId,
    requested: Option<&StableId>,
) -> Result<StableId, String> {
    let target = requested.map_or_else(
        || Some(caller.clone()),
        |requested| resolve_player_id(simulation, requested),
    );
    let target = target.ok_or_else(|| {
        format!(
            "unknown player {}",
            requested.map_or("me", StableId::as_str)
        )
    })?;
    let actor = simulation
        .actors
        .get(&target)
        .filter(|actor| is_stream_player_actor(&actor.id))
        .ok_or_else(|| "the follow camera can target only player citizens".to_owned())?;
    if !actor.alive {
        return Err(format!("{} is not currently alive", actor.id));
    }
    Ok(target)
}

pub(crate) fn auto_camera_is_managed(controller: &TownCameraControllerRuntime) -> bool {
    controller.seconds_since_acknowledgement >= AUTO_CAMERA_IDLE_SECONDS
        && controller.auto_shot != AutoCameraShot::Inactive
}

pub(crate) fn command_interrupts_auto_camera(command: &ChatCommand) -> bool {
    !matches!(
        command,
        ChatCommand::Follow(_) | ChatCommand::FocusBuilding { .. } | ChatCommand::Vote(_)
    )
}

pub(crate) fn command_refreshes_building_focus(command: &ChatCommand) -> bool {
    matches!(
        command,
        ChatCommand::Build(_)
            | ChatCommand::MoveBuilding(_)
            | ChatCommand::CenterBuilding
            | ChatCommand::BeginBuildingLine
            | ChatCommand::EndBuildingLine
            | ChatCommand::ConfirmBuilding
    )
}

pub(crate) fn command_refreshes_world_diagnostic(command: &ChatCommand) -> bool {
    matches!(
        command,
        ChatCommand::Build(_)
            | ChatCommand::MoveBuilding(_)
            | ChatCommand::CenterBuilding
            | ChatCommand::BeginBuildingLine
            | ChatCommand::EndBuildingLine
            | ChatCommand::ConfirmBuilding
            | ChatCommand::CancelBuilding
            | ChatCommand::Camera(_)
            | ChatCommand::ResetCamera
            | ChatCommand::Follow(_)
            | ChatCommand::FocusBuilding { .. }
    )
}

pub(crate) fn is_world_diagnostic_timer_operator(
    simulation: &WorldSimulation,
    actor: &StableId,
    user_type: StreamUserType,
) -> bool {
    simulation.is_ruler(actor)
        || matches!(
            user_type,
            StreamUserType::GameMaster | StreamUserType::Broadcaster
        )
}

pub(crate) fn item_info(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    requested: &StableId,
    instance: Option<u16>,
) -> Result<String, String> {
    if instance.is_none()
        && let Some(player) = simulation.actors.values().find(|actor| {
            is_stream_player_actor(&actor.id)
                && (actor.id == *requested
                    || actor
                        .login_name
                        .as_deref()
                        .is_some_and(|name| name.eq_ignore_ascii_case(requested.as_str()))
                    || actor.display_name.as_deref().is_some_and(|name| {
                        name.replace('_', " ")
                            .eq_ignore_ascii_case(&requested.as_str().replace('_', " "))
                    }))
        })
    {
        let levels = content
            .roles
            .iter()
            .filter(|(id, _)| !matches!(id.as_str(), "role:ruler" | "role:enemy"))
            .map(|(id, role)| {
                let level = player
                    .role_progression
                    .get(id)
                    .copied()
                    .unwrap_or_default()
                    .level;
                format!("{} {level}", role.display_name)
            })
            .collect::<Vec<_>>()
            .join(", ");
        let name = player.display_name.as_deref().unwrap_or(player.id.as_str());
        return Ok(format!(
            "{name}: current {}, health {}/{}, role levels — {levels}",
            player.role, player.health, player.max_health
        ));
    }
    if let Some(role_id) = prefixed_id(requested, "role:")
        && let Some(role) = content.roles.get(&role_id)
    {
        let capacity = role_capacity(content, simulation, &role_id)
            .map_or_else(|| "unlimited".to_owned(), |value| value.to_string());
        let assigned = simulation
            .actors
            .values()
            .filter(|actor| actor.role == role_id && !is_guardhouse_defender_id(&actor.id))
            .count();
        return Ok(format!(
            "{}: action {}, health {}, carry {}, slots {assigned}/{capacity}",
            role.display_name, role.base_action_amount, role.base_health, role.base_carry_capacity
        ));
    }
    if let Ok(building_id) = building_definition_id(content, requested)
        && let Some(building) = content.buildings.get(&building_id)
    {
        let instances = building_instance_ids(content, simulation, &building_id);
        if let Some(instance) = instance {
            let state = instances
                .get(usize::from(instance.saturating_sub(1)))
                .and_then(|runtime_id| simulation.buildings.get(runtime_id))
                .ok_or_else(|| {
                    format!("{} BID {instance} does not exist", building.display_name)
                })?;
            let max_health = building_max_health(content, state);
            let max_level = maximum_building_level(content, simulation, &building_id);
            let status = if state.complete {
                "complete"
            } else {
                "under construction"
            };
            let next_upgrade = if !building.can_level || state.level >= max_level {
                "maximum level".to_owned()
            } else {
                let cost = building_upgrade_cost(
                    content,
                    simulation,
                    &building_id,
                    building,
                    state.level,
                    state.level.saturating_add(1),
                )
                .into_iter()
                .map(|(resource, amount)| format!("{resource}={amount}"))
                .collect::<Vec<_>>()
                .join(", ");
                format!("next upgrade {cost}")
            };
            return Ok(format!(
                "{} BID {instance}: level {}/{max_level}, health {}/{max_health}, {status}, at {},{}, {next_upgrade}",
                building.display_name,
                state.level,
                state.health,
                state.position.x,
                state.position.z
            ));
        }
        let count = instances.len();
        let cost = building
            .cost
            .iter()
            .map(|(resource, amount)| format!("{resource}={amount}"))
            .collect::<Vec<_>>()
            .join(", ");
        return Ok(format!(
            "{}: count {count}, footprint {}x{}, cost {cost}",
            building.display_name, building.footprint[0], building.footprint[1]
        ));
    }
    if let Some(technology_id) = resolve_technology_id(content, requested) {
        let technology = &content.technology.nodes[&technology_id];
        let status = if simulation.unlocked_technology.contains(&technology_id) {
            "unlocked"
        } else if technology.unavailable {
            "unavailable"
        } else {
            "locked"
        };
        return Ok(format!(
            "{} ({status}): {}",
            technology.display_name, technology.description
        ));
    }
    if let Some(resource_id) = prefixed_id(requested, "resource:")
        && let Some(amount) = simulation.town_resources.get(&resource_id)
    {
        return Ok(format!("{resource_id}: town amount {amount}"));
    }
    if let Some(player_id) = resolve_player_id(simulation, requested) {
        let actor = &simulation.actors[&player_id];
        return Ok(format!(
            "{player_id}: {}, health {}/{}, at {},{}",
            actor.role, actor.health, actor.max_health, actor.position.x, actor.position.z
        ));
    }
    Err(format!("unknown item {requested}"))
}

pub(crate) fn compatible_station_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    config: &GameConfig,
    actor: &ActorState,
) -> Vec<StableId> {
    let Some(role) = content.roles.get(&actor.role) else {
        return Vec::new();
    };
    let town_hall = StableId::new("building:townhall").expect("static ID");
    let mut stations: Vec<_> = std::iter::once(town_hall)
        .chain(
            simulation
                .buildings
                .keys()
                .filter(|id| id.as_str() != "building:townhall")
                .cloned(),
        )
        .filter(|id| {
            station_candidate(content, simulation, config, id).is_some_and(|station| {
                station_matches_role(station.definition, role)
                    && station_supports_role_targets(station.definition, role)
            })
        })
        .collect();
    stations.sort();
    stations
}

#[cfg(test)]
pub(crate) fn compatible_target_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    actor: &ActorState,
) -> Vec<StableId> {
    let station_targets = immediate_station_target_runtime(content, simulation, world, config);
    compatible_target_ids_with_station_runtime(
        content,
        simulation,
        world,
        config,
        &station_targets,
        actor,
    )
}

pub(crate) fn compatible_target_ids_with_station_runtime(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    station_targets: &StationTargetRuntime,
    actor: &ActorState,
) -> Vec<StableId> {
    let Some(role) = content.roles.get(&actor.role) else {
        return Vec::new();
    };
    let fallback_station =
        best_station_id(content, simulation, config, &actor.role, actor.position);
    let Some(station) = assigned_station(content, simulation, config, actor).or_else(|| {
        fallback_station
            .as_ref()
            .and_then(|id| station_candidate(content, simulation, config, id))
    }) else {
        return Vec::new();
    };
    let mut kinds = role.target_kinds.iter().collect::<Vec<_>>();
    kinds.sort();
    let mut targets = Vec::new();
    for kind in kinds {
        if !station.definition.targets_all && !station.definition.target_kinds.contains(kind) {
            continue;
        }
        let cached = cached_station_targets(station_targets, station, kind)
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut candidates = station_target_candidates(content, simulation, world, actor, kind);
        candidates.retain(|(_, id)| cached.contains(id));
        candidates.sort_by_key(|(_, id)| id.clone());
        targets.extend(candidates.into_iter().map(|(_, id)| id));
    }
    targets
}

pub(crate) fn station_target_candidates(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    actor: &ActorState,
    kind: &StableId,
) -> Vec<(GridPos, StableId)> {
    if matches!(
        kind.as_str(),
        "target:tree" | "target:ore" | "target:bush" | "target:fish"
    ) {
        // Generated worlds contain thousands of resources. Filter their source
        // records in one pass instead of cloning a catalog and then linearly
        // rescanning the world for every candidate.
        return world
            .resources
            .iter()
            .filter(|resource| {
                resource.amount > 0
                    && resource.target_kind == *kind
                    && actor_accepts_resource(content, actor, resource)
            })
            .map(|resource| (resource.position, resource.id.clone()))
            .collect();
    }

    let mut candidates = station_target_catalog_candidates(content, simulation, world, kind);
    if matches!(
        kind.as_str(),
        "target:player" | "target:injured_player" | "target:dead_player"
    ) {
        candidates.retain(|(_, id)| *id != actor.id);
    }
    candidates
}

pub(crate) fn station_target_catalog_candidates(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    kind: &StableId,
) -> Vec<(GridPos, StableId)> {
    match kind.as_str() {
        "target:tree" | "target:ore" | "target:bush" | "target:fish" => world
            .resources
            .iter()
            .filter(|resource| resource.amount > 0 && resource.target_kind == *kind)
            .map(|resource| (resource.position, resource.id.clone()))
            .collect(),
        "target:farm" => simulation
            .buildings
            .values()
            .filter(|building| is_farm_resource_building(content, building))
            .map(|building| (building_visual_grid(content, building), building.id.clone()))
            .collect(),
        "target:enemy" | "target:boss" => simulation
            .actors
            .values()
            .filter(|target| target.alive && target.role.as_str() == "role:enemy")
            .filter(|target| {
                let is_boss = actor_archetype(content, target)
                    .and_then(|archetype| archetype.enemy.as_ref())
                    .is_some_and(|enemy| enemy.enemy_type.as_str().ends_with("_boss"));
                is_boss == (kind.as_str() == "target:boss")
            })
            .map(|target| (target.position, target.id.clone()))
            .collect(),
        "target:player" => simulation
            .actors
            .values()
            .filter(|target| {
                target.alive
                    && target.role.as_str() != "role:enemy"
                    && target.health == target.max_health
            })
            .map(|target| (target.position, target.id.clone()))
            .collect(),
        "target:injured_player" => simulation
            .actors
            .values()
            .filter(|target| {
                target.alive
                    && target.role.as_str() != "role:enemy"
                    && target.health < target.max_health
            })
            .map(|target| (target.position, target.id.clone()))
            .collect(),
        "target:dead_player" => simulation
            .actors
            .values()
            .filter(|target| !target.alive && target.role.as_str() != "role:enemy")
            .map(|target| (target.position, target.id.clone()))
            .collect(),
        "target:construction" => simulation
            .buildings
            .values()
            .filter(|building| !building.complete && building.health > 0)
            .map(|building| (building_visual_grid(content, building), building.id.clone()))
            .collect(),
        "target:damaged_building" => simulation
            .buildings
            .values()
            .filter(|building| {
                building.complete
                    && building.health > 0
                    && building.health < building_max_health(content, building)
            })
            .map(|building| (building_visual_grid(content, building), building.id.clone()))
            .collect(),
        "target:building" => simulation
            .buildings
            .values()
            .filter(|building| {
                building.complete
                    && building.health == building_max_health(content, building)
                    && !is_farm_resource_building(content, building)
            })
            .map(|building| (building_visual_grid(content, building), building.id.clone()))
            .collect(),
        _ => Vec::new(),
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn recruit_npcs(
    commands: &mut Commands,
    config: &GameConfig,
    content: &ContentCatalog,
    world: &GeneratedWorld,
    render: &RenderAssets,
    simulation: &mut WorldSimulation,
    role: &StableId,
    requested_amount: u16,
) -> Result<String, String> {
    let recruit_resource = StableId::new("resource:recruit").expect("static resource ID");
    let current = simulation
        .town_resources
        .get(&recruit_resource)
        .copied()
        .unwrap_or_default();
    let capacity = resource_storage_capacity(config, content, simulation, &recruit_resource);
    let available = capacity.saturating_sub(current);
    let amount = u32::from(requested_amount).min(available);
    if amount == 0 {
        return Err("the town has no recruit capacity".to_owned());
    }
    if !content.roles.contains_key(role) {
        return Err(format!("unknown role {role}"));
    }
    let current_in_role = simulation
        .actors
        .values()
        .filter(|actor| actor.role == *role && !is_guardhouse_defender_id(&actor.id))
        .count();
    let available_role_slots = role_capacity(content, simulation, role).map_or(usize::MAX, |max| {
        usize::try_from(max)
            .unwrap_or(usize::MAX)
            .saturating_sub(current_in_role)
    });
    let amount = amount.min(u32::try_from(available_role_slots).unwrap_or(u32::MAX));
    if amount == 0 {
        return Err(format!("the {role} role is full"));
    }
    let archetype =
        archetype_id_by_source(content, ArchetypeKind::Player, "Player_Character.prefab")
            .unwrap_or_else(|| StableId::new("archetype:viewer").expect("static ID"));
    let center = GridPos {
        x: world.navigation.width() / 2,
        z: world.navigation.height() / 2,
    };
    let base_scale = Vec3::new(
        config.world.cell_size * 0.3,
        config.world.cell_size * 0.55,
        config.world.cell_size * 0.3,
    );
    let mut spawned = 0_u32;
    for _ in 0..amount {
        let mut sequence = u64::try_from(simulation.actors.len()).unwrap_or(u64::MAX);
        let id = loop {
            let candidate =
                StableId::new(format!("npc:recruit_{sequence:08}")).expect("runtime recruit ID");
            if !simulation.actors.contains_key(&candidate) {
                break candidate;
            }
            sequence = sequence
                .checked_add(1)
                .ok_or_else(|| "runtime recruit identifier space exhausted".to_owned())?;
        };
        let desired = GridPos {
            x: center
                .x
                .saturating_add(u16::try_from(sequence % 9).unwrap_or_default()),
            z: center
                .z
                .saturating_add(u16::try_from((sequence / 9) % 9).unwrap_or_default()),
        };
        let position = nearest_walkable(world, desired)
            .or_else(|| nearest_walkable(world, center))
            .ok_or_else(|| "no walkable recruit spawn is available".to_owned())?;
        if !simulation.join_player(id.clone(), position) {
            continue;
        }
        simulation
            .assign_role(&id, role.clone())
            .map_err(|error| error.to_string())?;
        if let Some(actor) = simulation.actors.get_mut(&id) {
            actor.archetype = Some(archetype.clone());
            actor.display_name = Some(format!("Recruit {}", sequence + 1));
            actor.login_name = None;
        }
        let target = deterministic_wander_target(world, &id, position);
        let world_position = grid_to_world_on_surface(position, config, world);
        commands.spawn((
            WorldEntity,
            GridLocation(position),
            Agent {
                id,
                kind: ActorKind::Player,
                archetype: archetype.clone(),
                goal: AgentGoal::Wander,
                spawn: position,
                origin: position,
                navigation_position: placement_to_navigation_centre(position),
                path: Vec::new(),
                path_index: 0,
                target,
                action_cooldown_seconds: 0.0,
                action_started: false,
                repath_remaining_seconds: 0.0,
                health_regen_accumulator: 0.0,
                wander_sequence: 0,
                previous_wander_origin: None,
            },
            AgentLocomotion::default(),
            AgentAnimation {
                base_scale,
                ..default()
            },
            Mesh3d(render.actor_lod.clone()),
            MeshMaterial3d(actor_material(render, &ActorKind::Player, false)),
            Transform::from_translation(world_position + Vec3::Y * base_scale.y * 0.5)
                .with_scale(base_scale),
        ));
        spawned = spawned.saturating_add(1);
    }
    *simulation
        .town_resources
        .entry(recruit_resource)
        .or_default() = current.saturating_add(spawned);
    Ok(format!("recruited {spawned} {role}"))
}
