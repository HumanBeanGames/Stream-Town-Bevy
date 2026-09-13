pub(crate) fn send_command_feedback(connection: &TwitchConnection, message: String) {
    if let Some(transport) = &connection.transport {
        let _ = transport.send(TwitchControl::SendBotMessage(message));
    }
}

pub(crate) fn unity_outbound_reply(
    command: &ChatCommand,
    succeeded: bool,
    message: &str,
    display_name: &str,
) -> Option<String> {
    if !succeeded {
        let message = message
            .strip_prefix("command rejected: ")
            .unwrap_or(message);
        return Some(if message.contains("join before") {
            format!("{display_name}: You need to create a character first with !join")
        } else {
            format!("{display_name}: {message}")
        });
    }
    match command {
        ChatCommand::Join if message == "welcome to Stream Town" => Some(format!(
            "{display_name} Welcome to the game, your character was successfully created!"
        )),
        ChatCommand::Join => Some(format!(
            "{display_name} Character already registered into the game!"
        )),
        ChatCommand::SelectRole(_) => Some(format!("{display_name}  Role switched successfully!")),
        ChatCommand::Role
        | ChatCommand::Health
        | ChatCommand::Level(_)
        | ChatCommand::Experience
        | ChatCommand::Pets
        | ChatCommand::Pet(None)
        | ChatCommand::RecruitCount
        | ChatCommand::RecruitInfo(_)
        | ChatCommand::RecruitRole { .. }
        | ChatCommand::DismissRecruit(_)
        | ChatCommand::Resign
        | ChatCommand::LevelBuilding { .. }
        | ChatCommand::LevelAll { .. }
        | ChatCommand::RemoveBuilding { .. } => Some(format!("{display_name} {message}")),
        ChatCommand::Station(Some(_)) => Some(format!("{display_name}: Station Switched!")),
        ChatCommand::Target(Some(_)) => Some(format!("{display_name}: Target Switched!")),
        ChatCommand::Customize { kind, .. } => Some(format!(
            "{display_name}: {}",
            match kind {
                CustomizationKind::Hair => "Hair Style Changed!",
                CustomizationKind::Eyes => "Eye Style Changed!",
                CustomizationKind::FacialHair => "Facial Hair Style Changed!",
                CustomizationKind::Body => "Body Type Changed!",
                CustomizationKind::HairColor => "Hair Color Changed!",
                CustomizationKind::EyeColor => "Eye Color Changed!",
            }
        )),
        ChatCommand::Pet(Some(_)) => Some(format!("{display_name} pet switched!")),
        ChatCommand::Buy { .. } | ChatCommand::Sell { .. } => {
            Some(format!("{display_name} : {message}"))
        }
        ChatCommand::Discord
        | ChatCommand::Help
        | ChatCommand::Roles
        | ChatCommand::TownStats
        | ChatCommand::Population
        | ChatCommand::Info { .. }
        | ChatCommand::ToggleBuildCosts
        | ChatCommand::ToggleRoleLimits => Some(message.to_owned()),
        // Unity performs these successfully without writing another chat line.
        ChatCommand::Build(_)
        | ChatCommand::MoveBuilding(_)
        | ChatCommand::CenterBuilding
        | ChatCommand::ConfirmBuilding
        | ChatCommand::CancelBuilding
        | ChatCommand::Unstuck
        | ChatCommand::Ping
        | ChatCommand::Camera(_)
        | ChatCommand::ResetCamera
        | ChatCommand::ModRole { .. }
        | ChatCommand::StartRulerVote
        | ChatCommand::Station(None)
        | ChatCommand::Target(None)
        | ChatCommand::RecruitIds
        | ChatCommand::AddResource { .. }
        | ChatCommand::KillPlayer(_)
        | ChatCommand::GameMasterRevive(_)
        | ChatCommand::GiveExperience { .. }
        | ChatCommand::GiveExperienceAll(_)
        | ChatCommand::LevelUpPlayer { .. }
        | ChatCommand::QueueEvent(_)
        | ChatCommand::GivePet { .. }
        | ChatCommand::StopEvent
        | ChatCommand::CompleteObjective
        | ChatCommand::RandomTechnology
        | ChatCommand::TechnologyVote
        | ChatCommand::GameEventAction
        | ChatCommand::UnlockAllTechnology
        | ChatCommand::UnlockAgeTwo
        | ChatCommand::TriggerEvent(_)
        | ChatCommand::Praise => None,
        _ => Some(format!("{display_name}: {message}")),
    }
}

pub(crate) fn twitch_pascal_case(value: &str) -> String {
    value
        .split(['_', '-', ' '])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut characters = part.chars();
            characters.next().map_or_else(String::new, |first| {
                first.to_uppercase().chain(characters).collect()
            })
        })
        .collect()
}

pub(crate) fn resource_cost_display_name(resource: &StableId) -> String {
    twitch_pascal_case(resource.as_str().trim_start_matches("resource:"))
}

pub(crate) fn ordered_resource_costs(cost: &BTreeMap<StableId, u32>) -> Vec<(&StableId, u32)> {
    let order = ["wood", "ore", "food", "gold", "recruit"];
    let mut resources = cost.iter().collect::<Vec<_>>();
    resources.sort_by_key(|(resource, _)| {
        order
            .iter()
            .position(|name| resource.as_str().trim_start_matches("resource:") == *name)
            .unwrap_or(order.len())
    });
    resources
        .into_iter()
        .map(|(resource, amount)| (resource, *amount))
        .collect()
}

pub(crate) fn format_resource_costs(cost: &BTreeMap<StableId, u32>) -> String {
    if cost.is_empty() {
        return "free".to_owned();
    }
    ordered_resource_costs(cost)
        .into_iter()
        .map(|(resource, amount)| format!("{} {amount}", resource_cost_display_name(resource)))
        .collect::<Vec<_>>()
        .join(" | ")
}

pub(crate) fn building_shortage_message(
    simulation: &WorldSimulation,
    display_name: &str,
    cost: &BTreeMap<StableId, u32>,
) -> Option<String> {
    let shortages = ordered_resource_costs(cost)
        .into_iter()
        .filter_map(|(resource, required)| {
            let available = simulation
                .town_resources
                .get(resource)
                .copied()
                .unwrap_or_default();
            (available < required).then(|| {
                format!(
                    "{} {} more (have {available}/{required})",
                    resource_cost_display_name(resource),
                    required - available,
                )
            })
        })
        .collect::<Vec<_>>();
    (!shortages.is_empty()).then(|| {
        format!(
            "cannot afford {display_name}; need {}",
            shortages.join(", ")
        )
    })
}

pub(crate) fn building_cost_summary(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    requested: &StableId,
) -> Result<String, String> {
    let building_id = building_definition_id(content, requested)?;
    let definition = &content.buildings[&building_id];
    let cost = building_construction_cost(content, simulation, &building_id, definition);
    Ok(format!(
        "{} build cost: {} | current maximum level: {}",
        definition.display_name,
        format_resource_costs(&cost),
        maximum_building_level(content, simulation, &building_id),
    ))
}

pub(crate) fn shift_grid_position(
    position: GridPos,
    actions: &[BuildingAction],
    world: &GeneratedWorld,
) -> (GridPos, i32) {
    shift_grid_position_with_bounds(
        position,
        actions,
        world.navigation.width(),
        world.navigation.height(),
    )
}

pub(crate) fn shift_grid_position_with_bounds(
    position: GridPos,
    actions: &[BuildingAction],
    width: u16,
    height: u16,
) -> (GridPos, i32) {
    let mut x = i32::from(position.x);
    let mut z = i32::from(position.z);
    let mut rotation = 0_i32;
    for action in actions {
        match action.direction {
            // The authored gameplay camera looks toward +X, so command names
            // describe the viewer's on-screen directions rather than raw grid
            // axes: screen up/down is +/-X and screen left/right is -/+Z.
            BuildingDirection::Up => x = x.saturating_add(action.amount),
            BuildingDirection::Down => x = x.saturating_sub(action.amount),
            BuildingDirection::Left => z = z.saturating_sub(action.amount),
            BuildingDirection::Right => z = z.saturating_add(action.amount),
            BuildingDirection::Rotate => rotation = rotation.saturating_add(action.amount),
        }
    }
    let max_x = i32::from(width.saturating_sub(1));
    let max_z = i32::from(height.saturating_sub(1));
    (
        GridPos {
            x: u16::try_from(x.clamp(0, max_x)).expect("grid x is clamped"),
            z: u16::try_from(z.clamp(0, max_z)).expect("grid z is clamped"),
        },
        rotation,
    )
}

pub(crate) fn move_path_placement(
    placement: &mut BuildingPlacement,
    actions: &[BuildingAction],
    world: &GeneratedWorld,
    fine_navigation: Option<&stream_town_domain::NavGrid>,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> i32 {
    let width = world
        .navigation
        .width()
        .saturating_mul(NAVIGATION_SUBDIVISIONS);
    let height = world
        .navigation
        .height()
        .saturating_mul(NAVIGATION_SUBDIVISIONS);
    let mut cursor = placement
        .navigation_position
        .unwrap_or_else(|| placement_to_navigation_centre(placement.position));
    let mut rotation = 0_i32;
    for action in actions {
        if action.direction == BuildingDirection::Rotate {
            rotation = rotation.saturating_add(action.amount);
            continue;
        }
        let step = BuildingAction {
            direction: action.direction,
            amount: action.amount.signum(),
        };
        for _ in 0..action.amount.unsigned_abs() {
            let next = shift_grid_position_with_bounds(cursor, &[step], width, height).0;
            if next == cursor {
                continue;
            }
            cursor = next;
        }
    }
    set_path_placement_cursor(placement, cursor, fine_navigation, content, simulation);
    rotation
}

pub(crate) fn set_path_placement_cursor(
    placement: &mut BuildingPlacement,
    cursor: GridPos,
    fine_navigation: Option<&stream_town_domain::NavGrid>,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) {
    placement.navigation_position = Some(cursor);
    placement.position = navigation_to_placement(cursor);
    if let Some(start) = placement.line_start {
        placement.path_cells = fine_navigation
            .and_then(|navigation| {
                routed_path_cells(navigation, content, simulation, start, cursor)
            })
            // Retain both endpoints so the preview clearly communicates a
            // failed route; path_route_is_complete keeps confirmation red.
            .unwrap_or_else(|| vec![start, cursor]);
    }
}

pub(crate) fn centre_building_placement_at_world_position(
    placement: &mut BuildingPlacement,
    world_position: Vec3,
    config: &GameConfig,
    fine_navigation: Option<&stream_town_domain::NavGrid>,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> Result<GridPos, String> {
    if is_path_building(&placement.building) {
        let cursor = world_to_navigation(world_position, config)
            .ok_or_else(|| "the centre ray landed outside the fine pathfinding grid".to_owned())?;
        set_path_placement_cursor(placement, cursor, fine_navigation, content, simulation);
        Ok(cursor)
    } else {
        let cursor = world_to_grid(world_position, config)
            .ok_or_else(|| "the centre ray landed outside the building grid".to_owned())?;
        placement.position = cursor;
        Ok(cursor)
    }
}

pub(crate) fn routed_path_cells(
    navigation: &stream_town_domain::NavGrid,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    start: GridPos,
    goal: GridPos,
) -> Option<Vec<GridPos>> {
    let gate_cells = completed_player_gate_navigation_cells(content, simulation);
    navigation
        .find_path_with_exceptions_and_costs(start, goal, &gate_cells, 10, 14, |position, cost| {
            path_construction_step_cost(position, cost)
        })
        .ok()
}

pub(crate) fn path_construction_step_cost(position: GridPos, base_cost: u32) -> u32 {
    let centre_distance = u32::from(position.x % NAVIGATION_SUBDIVISIONS).abs_diff(1)
        + u32::from(position.z % NAVIGATION_SUBDIVISIONS).abs_diff(1);
    base_cost.saturating_add(centre_distance.saturating_mul(PATH_CENTRE_THIRD_STEP_PENALTY))
}

pub(crate) fn building_definition_id(
    content: &ContentCatalog,
    requested: &StableId,
) -> Result<StableId, String> {
    if let Some(id) =
        prefixed_id(requested, "building:").filter(|id| content.buildings.contains_key(id))
    {
        return Ok(id);
    }
    let normalized = normalize_building_command_name(requested.as_str());
    content
        .buildings
        .iter()
        .find(|(id, definition)| {
            normalize_building_command_name(id.as_str().trim_start_matches("building:"))
                == normalized
                || normalize_building_command_name(&definition.display_name) == normalized
        })
        .map(|(id, _)| id.clone())
        .ok_or_else(|| format!("unknown building {requested}"))
}

pub(crate) fn building_placement_request(
    content: &ContentCatalog,
    requested: &StableId,
) -> Result<(StableId, bool), String> {
    let thick_path =
        normalize_building_command_name(requested.as_str().trim_start_matches("building:"))
            == "thickpath";
    if thick_path {
        Ok((
            StableId::new("building:path").expect("static path ID"),
            true,
        ))
    } else {
        building_definition_id(content, requested).map(|building| (building, false))
    }
}

pub(crate) fn normalize_building_command_name(value: &str) -> String {
    value
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect()
}

pub(crate) fn building_command_name(definition: &stream_town_domain::BuildingDef) -> String {
    twitch_pascal_case(&definition.display_name.replace([' ', '-', '_'], "_"))
}

pub(crate) fn numbered_building_instance_id(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    requested: &StableId,
    index: u16,
) -> Result<(StableId, StableId), String> {
    let building_id = building_definition_id(content, requested)?;
    let definition = &content.buildings[&building_id];
    let instances = building_instance_ids(content, simulation, &building_id);
    let runtime_id = instances
        .get(usize::from(index.saturating_sub(1)))
        .cloned()
        .ok_or_else(|| {
            format!(
                "{} BID {index} does not exist; use !bid {}",
                definition.display_name,
                building_command_name(definition)
            )
        })?;
    Ok((building_id, runtime_id))
}

pub(crate) fn building_instance_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building_id: &StableId,
) -> Vec<StableId> {
    let archetype = &content.buildings[building_id].archetype;
    let mut instances = simulation
        .buildings
        .values()
        .filter(|building| building.archetype == *archetype)
        .map(|building| building.id.clone())
        .collect::<Vec<_>>();
    instances.sort();
    instances
}

pub(crate) fn constructed_building_count(simulation: &WorldSimulation) -> usize {
    simulation
        .buildings
        .keys()
        .filter(|id| id.as_str() != "building:townhall")
        .count()
}

pub(crate) fn hud_building_count(content: &ContentCatalog, simulation: &WorldSimulation) -> usize {
    simulation
        .buildings
        .values()
        .filter(|state| {
            content
                .buildings
                .iter()
                .find(|(_, definition)| definition.archetype == state.archetype)
                .is_some_and(|(id, definition)| {
                    id.as_str() != "building:townhall"
                        && id.as_str() != "building:wall"
                        && id.as_str() != "building:path"
                        && definition.projectile_shooter.is_none()
                })
        })
        .count()
}

pub(crate) fn upgrade_building_instance(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    building_id: &StableId,
    runtime_id: &StableId,
    levels: u16,
) -> Result<u16, String> {
    let definition = &content.buildings[building_id];
    if !definition.can_level {
        return Err(format!("{} cannot be upgraded", definition.display_name));
    }
    let state = simulation
        .buildings
        .get(runtime_id)
        .ok_or_else(|| format!("building instance {runtime_id} does not exist"))?;
    let current_level = state.level;
    let archetype = state.archetype.clone();
    let max_level = maximum_building_level(content, simulation, building_id);
    let target_level = current_level.saturating_add(levels.max(1)).min(max_level);
    let cost = if simulation.building_costs_enabled {
        building_upgrade_cost(
            content,
            simulation,
            building_id,
            definition,
            current_level,
            target_level,
        )
    } else {
        BTreeMap::new()
    };
    let upgraded_max_health = building_max_health_at_level(content, &archetype, target_level);
    simulation
        .upgrade_building(
            runtime_id,
            max_level,
            target_level,
            upgraded_max_health,
            &cost,
        )
        .map_err(|error| error.to_string())
}

pub(crate) fn rotate_building_instance(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    runtime_id: &StableId,
    quarter_turns: i32,
) -> Result<(String, i32), String> {
    let state = simulation
        .buildings
        .get(runtime_id)
        .ok_or_else(|| format!("unknown building BID {runtime_id}"))?;
    let (definition_id, definition) = content
        .buildings
        .iter()
        .find(|(_, definition)| definition.archetype == state.archetype)
        .ok_or_else(|| format!("BID {runtime_id} has no building definition"))?;
    if definition.footprint[0] != definition.footprint[1] {
        return Err(format!(
            "{} has a non-square footprint and cannot be rotated after placement",
            definition.display_name
        ));
    }
    if matches!(definition_id.as_str(), "building:wall" | "building:gate") {
        return Err(format!(
            "{} orientation is controlled by its neighboring tiles",
            definition.display_name
        ));
    }
    let name = definition.display_name.clone();
    let state = simulation
        .buildings
        .get_mut(runtime_id)
        .expect("building was validated");
    state.rotation_quarter_turns = state.rotation_quarter_turns.saturating_add(quarter_turns);
    Ok((name, state.rotation_quarter_turns.rem_euclid(4) * 90))
}

pub(crate) fn sell_town_resource(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    resource: &StableId,
    amount: u32,
) -> Result<String, String> {
    simulation
        .sell_resource(resource, amount)
        .map(|(sold, gold)| {
            let _ = simulation.record_objective_event(
                &content.objectives,
                &ObjectiveEvent::ResourceGained {
                    resource: StableId::new("resource:gold").expect("static stable ID"),
                    amount: gold,
                },
            );
            let _ = simulation.record_objective_event(
                &content.objectives,
                &ObjectiveEvent::ResourceSold {
                    resource: resource.clone(),
                    amount: sold,
                },
            );
            format!("sold {sold} {resource} for {gold} gold")
        })
        .map_err(|error| error.to_string())
}

pub(crate) fn buy_town_resource(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    resource: StableId,
    amount: u32,
) -> Result<String, String> {
    let capacity = resource_storage_capacity(config, content, simulation, &resource);
    simulation
        .buy_resource(resource.clone(), amount, capacity)
        .map(|(bought, gold)| {
            let _ = simulation.record_objective_event(
                &content.objectives,
                &ObjectiveEvent::TradeGoldSpent { amount: gold },
            );
            let _ = simulation.record_objective_event(
                &content.objectives,
                &ObjectiveEvent::ResourceGained {
                    resource: resource.clone(),
                    amount: bought,
                },
            );
            let _ = simulation.record_objective_event(
                &content.objectives,
                &ObjectiveEvent::ResourceBought {
                    resource: resource.clone(),
                    amount: bought,
                },
            );
            format!("bought {bought} {resource} for {gold} gold")
        })
        .map_err(|error| error.to_string())
}
