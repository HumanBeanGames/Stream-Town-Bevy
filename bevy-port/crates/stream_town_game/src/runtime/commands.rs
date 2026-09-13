use super::super::*;

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_injected_commands(
    mut ecs: Commands,
    mut queues: RuntimeCommandQueues,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    station_targets: Res<StationTargetRuntime>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    command_save: CommandSaveRuntime,
    mut view: CommandWorldViewRuntime,
    mut response: CommandResponseRuntime,
    mut world: ResMut<WorldRuntime>,
    mut stats: ResMut<SessionStats>,
    mut simulation: ResMut<SimulationRuntime>,
    mut timelapse_builds: MessageWriter<runtime::timelapse::CityTimelapseBuildConfirmed>,
) {
    while let Some(pending) = queues.injected.0.pop_front() {
        let actor_id = pending.actor_id.clone();
        let command = pending.command.clone();
        let user_type = pending_stream_user_type(&config.0, &pending);
        if let Some(actor) = simulation.0.actors.get_mut(&actor_id) {
            actor.display_name = Some(pending.display_name.clone());
            actor.login_name = Some(pending.login_name.clone());
            actor.user_type = user_type;
            if matches!(
                user_type,
                StreamUserType::Subscriber | StreamUserType::GameMaster
            ) {
                let red_panda = StableId::new("pet:red_panda").expect("static pet ID");
                actor.unlocked_pets.insert(red_panda.clone());
                actor.active_pet.get_or_insert(red_panda);
            }
        }
        let result = (|| -> Result<String, String> {
            match &command {
                ChatCommand::Join => {
                    if simulation.0.actors.contains_key(&actor_id) {
                        Ok("you are already in town".to_owned())
                    } else {
                        let desired = GridPos {
                            x: world.generated.navigation.width() / 2,
                            z: world.generated.navigation.height() / 2,
                        };
                        if let Some(position) = nearest_walkable(&world.generated, desired) {
                            let target = nearest_walkable(
                                &world.generated,
                                GridPos {
                                    x: position.x.saturating_sub(8),
                                    z: position.z.saturating_sub(8),
                                },
                            )
                            .unwrap_or(position);
                            let world_position =
                                grid_to_world_on_surface(position, &config.0, &world.generated);
                            simulation.0.join_player(actor_id.clone(), position);
                            let player_archetype = archetype_id_by_source(
                                &content.0,
                                ArchetypeKind::Player,
                                "Player_Character.prefab",
                            )
                            .unwrap_or_else(|| {
                                StableId::new("archetype:viewer").expect("static ID")
                            });
                            let name_color =
                                preset_player_name_color(simulation.0.world_seed, &actor_id);
                            if let Some(actor) = simulation.0.actors.get_mut(&actor_id) {
                                actor.archetype = Some(player_archetype.clone());
                                actor.display_name = Some(pending.display_name.clone());
                                actor.login_name = Some(pending.login_name.clone());
                                actor.user_type = user_type;
                                actor.customization.name_color = Some(name_color);
                                if matches!(
                                    user_type,
                                    StreamUserType::Subscriber | StreamUserType::GameMaster
                                ) {
                                    let red_panda =
                                        StableId::new("pet:red_panda").expect("static pet ID");
                                    actor.unlocked_pets.insert(red_panda.clone());
                                    actor.active_pet.get_or_insert(red_panda);
                                }
                            }
                            let base_scale = Vec3::new(
                                config.0.world.cell_size * 0.3,
                                config.0.world.cell_size * 0.55,
                                config.0.world.cell_size * 0.3,
                            );
                            ecs.spawn((
                                WorldEntity,
                                GridLocation(position),
                                Agent {
                                    id: actor_id.clone(),
                                    kind: ActorKind::Player,
                                    archetype: player_archetype,
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
                                Transform::from_xyz(
                                    world_position.x,
                                    world_position.y + base_scale.y * 0.5,
                                    world_position.z,
                                )
                                .with_scale(base_scale),
                            ));
                            Ok("welcome to Stream Town".to_owned())
                        } else {
                            Err("no walkable join position is available".to_owned())
                        }
                    }
                }
                ChatCommand::SelectRole(role) => {
                    let role = prefixed_id(role, "role:")
                        .filter(|role| content.0.roles.contains_key(role))
                        .ok_or_else(|| format!("unknown role {}", role.as_str()));
                    role.and_then(|role| {
                        if role.as_str() == "role:ruler" && !simulation.0.is_ruler(&actor_id) {
                            return Err("the Ruler role is assigned by election".to_owned());
                        }
                        if !role_is_available(&content.0, &simulation.0, &role, Some(&actor_id)) {
                            return Err(format!("the {role} role is full"));
                        }
                        simulation
                            .0
                            .assign_role(&actor_id, role.clone())
                            .map(|()| format!("role changed to {role}"))
                            .map_err(|error| error.to_string())
                    })
                }
                ChatCommand::Role => simulation
                    .0
                    .actors
                    .get(&actor_id)
                    .ok_or_else(|| "join before checking your role".to_owned())
                    .map(|actor| {
                        let progress = actor
                            .role_progression
                            .get(&actor.role)
                            .copied()
                            .unwrap_or_default();
                        let role = content
                            .0
                            .roles
                            .get(&actor.role)
                            .map_or(actor.role.as_str(), |role| role.display_name.as_str());
                        format!("you are currently a level {} {role}", progress.level)
                    }),
                ChatCommand::Health => simulation
                    .0
                    .actors
                    .get(&actor_id)
                    .ok_or_else(|| "join before checking health".to_owned())
                    .map(|actor| {
                        format!("your health is: ({}/{})", actor.health, actor.max_health)
                    }),
                ChatCommand::Buildings => {
                    let names = content
                        .0
                        .buildings
                        .iter()
                        .filter(|(id, building)| {
                            building.placeable
                                && building_is_unlocked(&content.0, &simulation.0, id)
                        })
                        .map(|(_, building)| building_command_name(building))
                        .collect::<Vec<_>>();
                    Ok(format!("unlocked buildings: {}", names.join(", ")))
                }
                ChatCommand::BuildingCost(requested) => {
                    building_cost_summary(&content.0, &simulation.0, requested)
                }
                ChatCommand::BuildingIds(requested) => {
                    let building_id = building_definition_id(&content.0, requested)?;
                    let definition = &content.0.buildings[&building_id];
                    let instances = building_instance_ids(&content.0, &simulation.0, &building_id);
                    let ids = instances
                        .iter()
                        .enumerate()
                        .map(|(index, _)| (index + 1).to_string())
                        .collect::<Vec<_>>();
                    spawn_numbered_world_labels(&mut ecs, &instances);
                    Ok(if ids.is_empty() {
                        format!("no {} buildings", definition.display_name)
                    } else {
                        format!("{} BIDs: {}", definition.display_name, ids.join(", "))
                    })
                }
                ChatCommand::Build(requested) => {
                    building_placement_request(&content.0, requested).and_then(
                        |(building_id, thick_path)| {
                        let building = &content.0.buildings[&building_id];
                        if queues.placers.0.contains_key(&actor_id) {
                            return Err(
                                "already placing a building; use !confirm or !cancel".to_owned()
                            );
                        }
                        if !building.placeable {
                            return Err(format!(
                                "{} cannot be player-placed",
                                building.display_name
                            ));
                        }
                        if !building_is_unlocked(&content.0, &simulation.0, &building_id) {
                            return Err(format!("{} is not unlocked", building.display_name));
                        }
                        let actor = simulation
                            .0
                            .actors
                            .get(&actor_id)
                            .ok_or_else(|| "join before building".to_owned())?;
                        let cost = building_construction_cost(
                            &content.0,
                            &simulation.0,
                            &building_id,
                            building,
                        );
                        if !is_line_building(&building_id)
                            && simulation.0.building_costs_enabled
                            && let Some(message) = building_shortage_message(
                                &simulation.0,
                                &building.display_name,
                                &cost,
                            )
                        {
                            return Err(message);
                        }
                        let rotation = actor.building_rotation_quarter_turns;
                        let near = actor
                            .last_building_position
                            .or(view.selected.0)
                            .unwrap_or(actor.position);
                        let line_building = is_line_building(&building_id);
                        let path_building = is_path_building(&building_id);
                        let path_navigation_position =
                            path_building.then(|| placement_to_navigation_centre(near));
                        queues.placers.0.insert(
                            actor_id.clone(),
                            BuildingPlacement {
                                building: building_id,
                                thick_path,
                                position: near,
                                navigation_position: path_navigation_position,
                                rotation_quarter_turns: rotation,
                                line_start: None,
                                line_end: None,
                                path_cells: Vec::new(),
                                inactivity_seconds: 0.0,
                            },
                        );
                        Ok(if path_building {
                            let fine = path_navigation_position.expect("path has fine cursor");
                            format!(
                                "placing {}{} at fine pathfinding cell {},{}; move to the first point and use !beginplace",
                                if thick_path { "thick " } else { "" },
                                building.display_name,
                                fine.x,
                                fine.z
                            )
                        } else if line_building {
                            format!(
                                "placing {} at {},{}; move to the first endpoint and use !beginplace",
                                building.display_name, near.x, near.z
                            )
                        } else {
                            format!(
                                "placing {} at {},{}; use !move, !rotate, !confirm, or !cancel",
                                building.display_name, near.x, near.z
                            )
                        })
                        },
                    )
                }
                ChatCommand::MoveBuilding(actions) => {
                    let placement = queues
                        .placers
                        .0
                        .get_mut(&actor_id)
                        .ok_or_else(|| "not in building placement mode".to_owned())?;
                    let rotation_delta = if is_path_building(&placement.building) {
                        move_path_placement(
                            placement,
                            actions,
                            &world.generated,
                            view.fine_navigation.grid.as_ref(),
                            &content.0,
                            &simulation.0,
                        )
                    } else {
                        let (position, rotation_delta) =
                            shift_grid_position(placement.position, actions, &world.generated);
                        placement.position = position;
                        rotation_delta
                    };
                    placement.inactivity_seconds = 0.0;
                    placement.rotation_quarter_turns = placement
                        .rotation_quarter_turns
                        .saturating_add(rotation_delta);
                    if let Some(actor) = simulation.0.actors.get_mut(&actor_id) {
                        actor.building_rotation_quarter_turns = actor
                            .building_rotation_quarter_turns
                            .saturating_add(rotation_delta);
                    }
                    let definition = &content.0.buildings[&placement.building];
                    let cursor = placement.navigation_position.unwrap_or(placement.position);
                    let validity = if building_placement_is_available(
                        &content.0,
                        &simulation.0,
                        &world.generated,
                        placement,
                        definition,
                        view.fine_navigation.grid.as_ref(),
                    ) {
                        "valid"
                    } else {
                        "blocked"
                    };
                    Ok(format!(
                        "{} placer moved to {},{} at {} degrees ({validity})",
                        definition.display_name,
                        cursor.x,
                        cursor.z,
                        placement.rotation_quarter_turns.rem_euclid(4) * 90
                    ))
                }
                ChatCommand::CenterBuilding => {
                    if !queues.placers.0.contains_key(&actor_id) {
                        return Err("not in building placement mode".to_owned());
                    }
                    let (camera, camera_transform, _) = view
                        .cameras
                        .single()
                        .map_err(|_| "the broadcast camera is unavailable".to_owned())?;
                    let spatial = view
                        .spatial
                        .as_ref()
                        .ok_or_else(|| "town raycasting is not ready".to_owned())?;
                    let world_position = centre_screen_raycast_world_position(
                        camera,
                        camera_transform,
                        spatial,
                    )?;
                    let placement = queues
                        .placers
                        .0
                        .get_mut(&actor_id)
                        .expect("active placement was checked");
                    let cursor = centre_building_placement_at_world_position(
                        placement,
                        world_position,
                        &config.0,
                        view.fine_navigation.grid.as_ref(),
                        &content.0,
                        &simulation.0,
                    )?;
                    placement.inactivity_seconds = 0.0;
                    let definition = &content.0.buildings[&placement.building];
                    let validity = if building_placement_is_available(
                        &content.0,
                        &simulation.0,
                        &world.generated,
                        placement,
                        definition,
                        view.fine_navigation.grid.as_ref(),
                    ) {
                        "valid"
                    } else {
                        "blocked"
                    };
                    Ok(format!(
                        "{} placer centered at {},{} ({validity})",
                        definition.display_name, cursor.x, cursor.z
                    ))
                }
                ChatCommand::BeginBuildingLine => {
                    let placement = queues
                        .placers
                        .0
                        .get_mut(&actor_id)
                        .ok_or_else(|| "not in building placement mode".to_owned())?;
                    if !is_line_building(&placement.building) {
                        return Err(
                            "!beginplace is only used for wall or path placement".to_owned()
                        );
                    }
                    let line_name = content.0.buildings[&placement.building]
                        .display_name
                        .clone();
                    let cursor = placement.navigation_position.unwrap_or(placement.position);
                    placement.line_start = Some(cursor);
                    placement.line_end = None;
                    placement.path_cells = if is_path_building(&placement.building) {
                        vec![cursor]
                    } else {
                        Vec::new()
                    };
                    placement.inactivity_seconds = 0.0;
                    Ok(format!(
                        "{line_name} start set at {},{}; move to the other endpoint and use !confirm",
                        cursor.x, cursor.z,
                    ))
                }
                ChatCommand::EndBuildingLine => {
                    let placement = queues
                        .placers
                        .0
                        .get_mut(&actor_id)
                        .ok_or_else(|| "not in building placement mode".to_owned())?;
                    if !is_line_building(&placement.building) {
                        return Err("!endplace is only used for wall or path placement".to_owned());
                    }
                    let line_name = content.0.buildings[&placement.building]
                        .display_name
                        .clone();
                    let start = placement.line_start.ok_or_else(|| {
                        format!("set the first {line_name} endpoint with !beginplace")
                    })?;
                    placement.line_end = None;
                    let cursor = placement.navigation_position.unwrap_or(placement.position);
                    let cells = if is_path_building(&placement.building) {
                        placement.path_cells.clone()
                    } else {
                        wall_line_cells(start, cursor)?
                    };
                    placement.line_end = Some(cursor);
                    placement.inactivity_seconds = 0.0;
                    Ok(format!(
                        "{line_name} endpoint set at {},{} ({} sections); !endplace is optional, review the ghost then use !confirm",
                        cursor.x,
                        cursor.z,
                        cells.len()
                    ))
                }
                ChatCommand::ConfirmBuilding => {
                    let placement = queues
                        .placers
                        .0
                        .get(&actor_id)
                        .cloned()
                        .ok_or_else(|| "not in building placement mode".to_owned())?;
                    let building = &content.0.buildings[&placement.building];
                    if !building.placeable
                        || !building_is_unlocked(&content.0, &simulation.0, &placement.building)
                    {
                        return Err(format!("{} can no longer be placed", building.display_name));
                    }
                    let path_building = is_path_building(&placement.building);
                    let attempted_path_cells = if path_building {
                        placement_visual_cells(&placement).len()
                    } else {
                        0
                    };
                    let cells = if path_building {
                        let navigation = view
                            .fine_navigation
                            .grid
                            .as_ref()
                            .ok_or_else(|| "pathfinding grid is not ready".to_owned())?;
                        confirmed_path_cells(&content.0, &simulation.0, &placement, navigation)
                    } else if is_line_building(&placement.building) {
                        placement.line_start.map_or_else(
                            || Ok(vec![placement.position]),
                            |start| wall_line_cells(start, placement.position),
                        )?
                    } else {
                        vec![placement.position]
                    };
                    if path_building && cells.is_empty() {
                        return Err("the selected route is already fully paved".to_owned());
                    }
                    if !building_placement_is_available(
                        &content.0,
                        &simulation.0,
                        &world.generated,
                        &placement,
                        building,
                        view.fine_navigation.grid.as_ref(),
                    ) {
                        return Err("building placement is blocked or outside the world".to_owned());
                    }
                    let unit_cost = if simulation.0.building_costs_enabled {
                        building_construction_cost(
                            &content.0,
                            &simulation.0,
                            &placement.building,
                            building,
                        )
                    } else {
                        BTreeMap::new()
                    };
                    let cost = scaled_building_cost(&unit_cost, cells.len());
                    if let Some(message) =
                        building_shortage_message(&simulation.0, &building.display_name, &cost)
                    {
                        return Err(message);
                    }
                    let sections = cells
                        .iter()
                        .map(|cell| {
                            let placement_cell = if path_building {
                                navigation_to_placement(*cell)
                            } else {
                                *cell
                            };
                            let region = if path_building {
                                None
                            } else {
                                Some(
                                    building_navigation_region(
                                        placement_cell,
                                        building,
                                        placement.rotation_quarter_turns,
                                        &world.generated,
                                    )
                                    .ok_or_else(|| {
                                        "building placement is outside the world".to_owned()
                                    })?,
                                )
                            };
                            Ok((*cell, placement_cell, region))
                        })
                        .collect::<Result<Vec<_>, String>>()?;
                    let covered_regions = sections
                        .iter()
                        .filter_map(|(_, _, region)| *region)
                        .collect::<Vec<_>>();
                    let removed_paths = remove_paths_covered_by_regions(
                        &content.0,
                        &mut world.generated,
                        &mut simulation.0,
                        &covered_regions,
                    )?;
                    let removed_path_count = removed_paths.len();
                    for runtime_id in removed_paths {
                        queues
                            .building
                            .0
                            .push_back(BuildingRuntimeCommand::Despawn(runtime_id));
                    }
                    let mut first = true;
                    let no_cost = BTreeMap::new();
                    for (cell, placement_cell, region) in sections {
                        let runtime_id = runtime_building_id(&simulation.0);
                        let section_cost = if first { &cost } else { &no_cost };
                        simulation
                            .0
                            .construct_rotated(
                                runtime_id.clone(),
                                building.archetype.clone(),
                                placement_cell,
                                placement.rotation_quarter_turns,
                                building_base_max_health(&content.0, building),
                                section_cost,
                            )
                            .map_err(|error| error.to_string())?;
                        if path_building {
                            simulation
                                .0
                                .path_navigation_positions
                                .insert(runtime_id.clone(), cell);
                        }
                        first = false;
                        if building_blocks_navigation(building)
                            && let Some(region) = region
                        {
                            world
                                .generated
                                .navigation
                                .set_blocked(region, true)
                                .map_err(|error| error.to_string())?;
                        }
                        spawn_runtime_building(
                            &mut ecs,
                            &config.0,
                            &world.generated,
                            &presentation.0,
                            asset_server.as_deref(),
                            &asset_root.0,
                            &render,
                            &simulation.0.buildings[&runtime_id],
                            building,
                            &content.0.archetypes[&building.archetype],
                            placement_cell,
                            building.footprint,
                            building_age(
                                &content.0,
                                &simulation.0,
                                &placement.building,
                                simulation.0.buildings[&runtime_id].level,
                            ),
                        );
                    }
                    if let Some(actor) = simulation.0.actors.get_mut(&actor_id) {
                        actor.last_building_position = Some(placement.position);
                    }
                    queues.placers.0.remove(&actor_id);
                    let skipped_path_cells = attempted_path_cells.saturating_sub(cells.len());
                    let mut message = if placement.thick_path && skipped_path_cells > 0 {
                        format!(
                            "placed {} {} sections for construction; {} blocked or existing supplemental sections were skipped",
                            cells.len(),
                            building.display_name,
                            skipped_path_cells
                        )
                    } else if cells.len() == 1 {
                        format!("placed {} construction", building.display_name)
                    } else {
                        format!(
                            "placed {} {} sections for construction",
                            cells.len(),
                            building.display_name
                        )
                    };
                    if removed_path_count > 0 {
                        message.push_str("; removed ");
                        message.push_str(&removed_path_count.to_string());
                        message.push_str(" covered path section");
                        if removed_path_count != 1 {
                            message.push('s');
                        }
                    }
                    Ok(message)
                }
                ChatCommand::CancelBuilding => queues
                    .placers
                    .0
                    .remove(&actor_id)
                    .map(|placement| {
                        format!(
                            "cancelled {} placement",
                            content.0.buildings[&placement.building].display_name
                        )
                    })
                    .ok_or_else(|| "not in building placement mode".to_owned()),
                ChatCommand::Upgrade {
                    building,
                    index,
                    levels,
                } => {
                    let (building_id, runtime_id) =
                        numbered_building_instance_id(&content.0, &simulation.0, building, *index)?;
                    let name = content.0.buildings[&building_id].display_name.clone();
                    upgrade_building_instance(
                        &content.0,
                        &mut simulation.0,
                        &building_id,
                        &runtime_id,
                        *levels,
                    )
                    .map(|level| format!("started {name} BID {index} level {level} construction"))
                }
                ChatCommand::RotateBuilding {
                    building,
                    index,
                    quarter_turns,
                } => {
                    let (_, runtime_id) =
                        numbered_building_instance_id(&content.0, &simulation.0, building, *index)?;
                    let (name, degrees) = rotate_building_instance(
                        &content.0,
                        &mut simulation.0,
                        &runtime_id,
                        *quarter_turns,
                    )?;
                    Ok(format!("rotated {name} BID {index} to {degrees} degrees"))
                }
                ChatCommand::Level(requested) => {
                    let role = prefixed_id(requested, "role:")
                        .filter(|role| content.0.roles.contains_key(role))
                        .ok_or_else(|| format!("unknown role {requested}"))?;
                    let actor = simulation
                        .0
                        .actors
                        .get(&actor_id)
                        .ok_or_else(|| "join before checking role progression".to_owned())?;
                    let progress = actor
                        .role_progression
                        .get(&role)
                        .copied()
                        .ok_or_else(|| format!("you have no progression for {role}"))?;
                    let role_name = content.0.roles[&role].display_name.as_str();
                    Ok(format!(
                        "you are a level ({}/{}) {role_name}. Current Exp: ({}/{}).",
                        progress.level,
                        content.0.progression.maximum_role_level,
                        progress.experience,
                        stream_town_domain::required_role_experience(
                            progress.level,
                            content.0.progression.role_experience_curve_level_span,
                            content.0.progression.role_experience_curve_maximum,
                        )
                    ))
                }
                ChatCommand::LevelBuilding {
                    building,
                    index,
                    iterations,
                } => {
                    let building_id = building_definition_id(&content.0, building)?;
                    let definition = &content.0.buildings[&building_id];
                    let instances = building_instance_ids(&content.0, &simulation.0, &building_id);
                    let runtime_id = instances
                        .get(usize::from(index.saturating_sub(1)))
                        .cloned()
                        .ok_or_else(|| {
                            format!(
                                "{} building ID {index} does not exist",
                                definition.display_name
                            )
                        })?;
                    let before = simulation.0.buildings[&runtime_id].level;
                    let after = upgrade_building_instance(
                        &content.0,
                        &mut simulation.0,
                        &building_id,
                        &runtime_id,
                        *iterations,
                    )?;
                    Ok(format!(
                        "Successfully Leveled Building {} {}",
                        after.saturating_sub(before),
                        if after.saturating_sub(before) > 1 { "Times" } else { "Time" }
                    ))
                }
                ChatCommand::LevelAll {
                    building,
                    target_level,
                } => {
                    let building_id = building_definition_id(&content.0, building)?;
                    let definition = &content.0.buildings[&building_id];
                    let instances = building_instance_ids(&content.0, &simulation.0, &building_id);
                    let mut successful = 0_u32;
                    for runtime_id in instances {
                        let current = simulation.0.buildings[&runtime_id].level;
                        if current >= *target_level {
                            continue;
                        }
                        if upgrade_building_instance(
                            &content.0,
                            &mut simulation.0,
                            &building_id,
                            &runtime_id,
                            target_level.saturating_sub(current),
                        )
                        .is_ok()
                        {
                            successful = successful.saturating_add(1);
                        }
                    }
                    if successful == 0 {
                        Err(format!(
                            "no {} buildings could be leveled toward level {target_level}",
                            definition.display_name
                        ))
                    } else {
                        Ok(format!("Successfully leveled {successful} times!"))
                    }
                }
                ChatCommand::RemoveBuilding { building, index } => {
                    if !simulation.0.is_ruler(&actor_id) {
                        return Err("only the current Ruler can remove buildings".to_owned());
                    }
                    let building_id = building_definition_id(&content.0, building)?;
                    if building_id.as_str() == "building:townhall" {
                        return Err("the Town Hall cannot be removed".to_owned());
                    }
                    let definition = &content.0.buildings[&building_id];
                    let instances = building_instance_ids(&content.0, &simulation.0, &building_id);
                    let runtime_id = instances
                        .get(usize::from(index.saturating_sub(1)))
                        .cloned()
                        .ok_or_else(|| {
                            format!(
                                "{} building ID {index} does not exist",
                                definition.display_name
                            )
                        })?;
                    remove_selected_building(
                        &runtime_id,
                        &content.0,
                        &mut world.generated,
                        &mut simulation.0,
                    )?;
                    queues
                        .building
                        .0
                        .push_back(BuildingRuntimeCommand::Despawn(runtime_id));
                    Ok("Successfully Removed Building".to_owned())
                }
                ChatCommand::Sell { amount, resource } => {
                    require_ruler_or_staff(&simulation.0, &pending)?;
                    let resource = prefixed_id(resource, "resource:")
                        .ok_or_else(|| format!("invalid resource {}", resource.as_str()));
                    resource.and_then(|resource| {
                        sell_town_resource(&content.0, &mut simulation.0, &resource, *amount)
                    })
                }
                ChatCommand::Buy { amount, resource } => {
                    require_ruler_or_staff(&simulation.0, &pending)?;
                    let resource = prefixed_id(resource, "resource:")
                        .ok_or_else(|| format!("invalid resource {}", resource.as_str()));
                    resource.and_then(|resource| {
                        buy_town_resource(
                            &config.0,
                            &content.0,
                            &mut simulation.0,
                            resource,
                            *amount,
                        )
                    })
                }
                ChatCommand::Vote(requested) => {
                    if simulation.0.community_vote.is_some() {
                        resolve_community_vote_option(requested)
                            .ok_or_else(|| "event voting uses !vote yes or !vote no".to_owned())
                            .and_then(|approved| {
                                simulation
                                    .0
                                    .cast_community_vote(&actor_id, approved)
                                    .map(|()| {
                                        format!(
                                            "event vote for {} accepted",
                                            if approved { "yes" } else { "no" }
                                        )
                                    })
                                    .map_err(|error| error.to_string())
                            })
                    } else if requested.as_str().parse::<usize>().is_ok()
                        && simulation.0.active_vote.is_some()
                    {
                        let technology = resolve_active_technology_vote_option(
                            &content.0,
                            &simulation.0,
                            requested,
                        )
                        .ok_or_else(|| {
                            format!("unknown technology vote option {}", requested.as_str())
                        });
                        technology.and_then(|technology| {
                            let name = content.0.technology.nodes[&technology].display_name.clone();
                            simulation
                                .0
                                .cast_technology_vote(&actor_id, technology)
                                .map(|()| format!("voted for {name}"))
                                .map_err(|error| error.to_string())
                        })
                    } else if simulation.0.ruler_vote.is_some() {
                        let option = resolve_ruler_vote_option(&simulation.0, requested)
                            .ok_or_else(|| {
                                format!("unknown ruler candidate {}", requested.as_str())
                            });
                        option.and_then(|option| {
                            simulation
                                .0
                                .cast_ruler_vote(&actor_id, option.clone())
                                .map(|()| {
                                    let label = simulation
                                        .0
                                        .actors
                                        .get(&option)
                                        .and_then(|actor| actor.display_name.as_deref())
                                        .unwrap_or(option.as_str());
                                    format!("ruler vote for {label} accepted")
                                })
                                .map_err(|error| error.to_string())
                        })
                    } else if simulation.0.active_vote.is_some() {
                        Err("technology voting uses !vote 1, !vote 2, or !vote 3".to_owned())
                    } else {
                        Err("there is no active vote".to_owned())
                    }
                }
                ChatCommand::Recruit { role, amount } => {
                    require_ruler_or_staff(&simulation.0, &pending)?;
                    let role = prefixed_id(role, "role:")
                        .filter(|role| content.0.roles.contains_key(role))
                        .ok_or_else(|| format!("unknown role {}", role.as_str()))?;
                    if role.as_str() == "role:ruler" {
                        Err("Ruler cannot be recruited".to_owned())
                    } else {
                        recruit_npcs(
                            &mut ecs,
                            &config.0,
                            &content.0,
                            &world.generated,
                            &render,
                            &mut simulation.0,
                            &role,
                            *amount,
                        )
                    }
                }
                ChatCommand::RecruitCount => {
                    require_ruler_or_staff(&simulation.0, &pending)?;
                    let recruits = capacity_recruited_actor_ids(&simulation.0).len();
                    Ok(format!("The town has {recruits} recruits!"))
                }
                ChatCommand::RecruitIds => {
                    require_ruler_or_staff(&simulation.0, &pending)?;
                    let recruit_ids = capacity_recruited_actor_ids(&simulation.0);
                    let recruits = recruit_ids
                        .iter()
                        .enumerate()
                        .map(|(index, id)| format!("{}={id}", index + 1))
                        .collect::<Vec<_>>();
                    spawn_numbered_world_labels(&mut ecs, &recruit_ids);
                    Ok(if recruits.is_empty() {
                        "the town has no recruited NPCs".to_owned()
                    } else {
                        format!("recruit IDs: {}", recruits.join(", "))
                    })
                }
                ChatCommand::RecruitInfo(index) => {
                    require_ruler_or_staff(&simulation.0, &pending)?;
                    let id = recruit_id(&simulation.0, *index)
                        .ok_or_else(|| format!("unknown recruit ID {index}"))?;
                    let recruit = &simulation.0.actors[&id];
                    let progress = recruit
                        .role_progression
                        .get(&recruit.role)
                        .copied()
                        .unwrap_or_default();
                    let role = content
                        .0
                        .roles
                        .get(&recruit.role)
                        .map_or(recruit.role.as_str(), |role| role.display_name.as_str());
                    Ok(format!(
                        "----- Recruit {index} | Current role {role} |  Health: {} / {} |  Level: {} / {} |  Experience: {} / {}",
                        recruit.health,
                        recruit.max_health,
                        progress.level,
                        content.0.progression.maximum_recruit_role_level,
                        progress.experience,
                        stream_town_domain::required_role_experience(
                            progress.level,
                            content.0.progression.role_experience_curve_level_span,
                            content.0.progression.role_experience_curve_maximum,
                        )
                    ))
                }
                ChatCommand::RecruitRole { recruit, role } => {
                    require_ruler_or_staff(&simulation.0, &pending)?;
                    let id = recruit_id(&simulation.0, *recruit)
                        .ok_or_else(|| format!("unknown recruit ID {recruit}"))?;
                    if is_guardhouse_defender_id(&id) {
                        return Err("guardhouse defenders are permanently assigned".to_owned());
                    }
                    let role = prefixed_id(role, "role:")
                        .filter(|role| content.0.roles.contains_key(role))
                        .ok_or_else(|| format!("unknown role {role}"))?;
                    if role.as_str() == "role:ruler" {
                        return Err("Ruler cannot be assigned to a recruit".to_owned());
                    }
                    if !role_is_available(&content.0, &simulation.0, &role, Some(&id)) {
                        return Err(format!("the {role} role is full"));
                    }
                    simulation
                        .0
                        .assign_role(&id, role.clone())
                        .map_err(|error| error.to_string())?;
                    let role_name = content.0.roles[&role].display_name.as_str();
                    Ok(format!(
                        "Successfully changed recruit {recruit} to {role_name}!"
                    ))
                }
                ChatCommand::DismissRecruit(index) => {
                    require_ruler_or_staff(&simulation.0, &pending)?;
                    let id = recruit_id(&simulation.0, *index)
                        .ok_or_else(|| format!("unknown recruit ID {index}"))?;
                    if is_guardhouse_defender_id(&id) {
                        return Err(
                            "guardhouse defenders leave only with their guardhouse".to_owned()
                        );
                    }
                    simulation.0.actors.remove(&id);
                    let resource = StableId::new("resource:recruit").expect("static ID");
                    let current = simulation
                        .0
                        .town_resources
                        .get(&resource)
                        .copied()
                        .unwrap_or_default();
                    simulation
                        .0
                        .town_resources
                        .insert(resource, current.saturating_sub(1));
                    queues.agent.0.push_back(AgentCommand::Despawn(id));
                    Ok(format!("Successfully Dismissed recruit {index}!"))
                }
                ChatCommand::StartRulerVote => {
                    require_staff(&pending)?;
                    let kind = if simulation.0.current_ruler.is_some() {
                        RulerVoteKind::KeepRuler
                    } else {
                        RulerVoteKind::NewRuler
                    };
                    simulation
                        .0
                        .start_ruler_vote(kind)
                        .map(|()| "ruler vote started".to_owned())
                        .map_err(|error| error.to_string())
                }
                ChatCommand::Resign => simulation
                    .0
                    .resign_ruler(&actor_id)
                    .map(|()| "you have been succesfully resigned!".to_owned())
                    .map_err(|error| error.to_string()),
                ChatCommand::RequestEvent(requested) => {
                    let event = community_event_from_id(requested).ok_or_else(|| {
                        format!(
                            "unknown event {}; use prospecting, reforestation, agricultural, rebalance, awakening, economic, or invasion",
                            requested.as_str()
                        )
                    })?;
                    simulation
                        .0
                        .request_community_event(event)
                        .map_err(|error| error.to_string())?;
                    Ok(if simulation.0.community_vote.is_some() {
                        format!(
                            "{} is now up for a 2-minute vote; use !vote yes or !vote no",
                            community_event_name(event)
                        )
                    } else {
                        format!(
                            "{} is queued behind the current Ruler vote",
                            community_event_name(event)
                        )
                    })
                }
                ChatCommand::Station(index) => {
                    let actor = simulation
                        .0
                        .actors
                        .get(&actor_id)
                        .ok_or_else(|| "join before selecting a station".to_owned())?;
                    let stations =
                        compatible_station_ids(&content.0, &simulation.0, &config.0, actor);
                    if let Some(index) = index {
                        let station = stations
                            .get(usize::from(index.saturating_sub(1)))
                            .cloned()
                            .ok_or_else(|| format!("unknown station ID {index}"))?;
                        simulation
                            .0
                            .actors
                            .get_mut(&actor_id)
                            .expect("validated actor")
                            .station = Some(station.clone());
                        Ok(format!("station changed to {station}"))
                    } else {
                        spawn_numbered_world_labels(&mut ecs, &stations);
                        Ok(if stations.is_empty() {
                            "no compatible stations are available".to_owned()
                        } else {
                            format!(
                                "station IDs: {}",
                                stations
                                    .iter()
                                    .enumerate()
                                    .map(|(index, id)| format!("{}={id}", index + 1))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            )
                        })
                    }
                }
                ChatCommand::Target(index) => {
                    let actor = simulation
                        .0
                        .actors
                        .get(&actor_id)
                        .ok_or_else(|| "join before selecting a target".to_owned())?;
                    let targets = compatible_target_ids_with_station_runtime(
                        &content.0,
                        &simulation.0,
                        &world.generated,
                        &config.0,
                        &station_targets,
                        actor,
                    );
                    if let Some(index) = index {
                        let target = targets
                            .get(usize::from(index.saturating_sub(1)))
                            .cloned()
                            .ok_or_else(|| format!("unknown target ID {index}"))?;
                        simulation
                            .0
                            .actors
                            .get_mut(&actor_id)
                            .expect("validated actor")
                            .preferred_target = Some(target.clone());
                        Ok(format!("target changed to {target}"))
                    } else {
                        spawn_numbered_world_labels(&mut ecs, &targets);
                        Ok(if targets.is_empty() {
                            "no compatible targets are available".to_owned()
                        } else {
                            format!(
                                "target IDs: {}",
                                targets
                                    .iter()
                                    .enumerate()
                                    .map(|(index, id)| format!("{}={id}", index + 1))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            )
                        })
                    }
                }
                ChatCommand::Unstuck => {
                    let town_hall =
                        restored_town_hall_position(&content.0, &simulation.0, &config.0);
                    let spawn = nearest_walkable(&world.generated, town_hall)
                        .ok_or_else(|| "the Town Hall has no walkable spawn cell".to_owned())?;
                    let actor = simulation
                        .0
                        .actors
                        .get_mut(&actor_id)
                        .ok_or_else(|| "join before using !stuck".to_owned())?;
                    actor.position = spawn;
                    actor.preferred_target = None;
                    queues.agent.0.push_back(AgentCommand::Teleport {
                        actor: actor_id.clone(),
                        position: spawn,
                    });
                    Ok("returned to the Town Hall".to_owned())
                }
                ChatCommand::Ping => {
                    let position = simulation
                        .0
                        .actors
                        .get(&actor_id)
                        .map(|actor| actor.position)
                        .ok_or_else(|| "join before using !ping".to_owned())?;
                    queues
                        .agent
                        .0
                        .push_back(AgentCommand::Ping(actor_id.clone()));
                    Ok(format!("pinged at grid {},{}", position.x, position.z))
                }
                ChatCommand::Customize { kind, index } => {
                    let actor = simulation
                        .0
                        .actors
                        .get_mut(&actor_id)
                        .ok_or_else(|| "join before customizing your character".to_owned())?;
                    let adjusted = index.saturating_sub(1);
                    let (name, maximum, field) = match kind {
                        CustomizationKind::Hair => ("hair", 7, &mut actor.customization.hair),
                        CustomizationKind::Eyes => ("eyes", 10, &mut actor.customization.eyes),
                        CustomizationKind::FacialHair => {
                            ("facial hair", 2, &mut actor.customization.facial_hair)
                        }
                        CustomizationKind::Body => ("body", 3, &mut actor.customization.body_type),
                        CustomizationKind::HairColor => {
                            ("hair color", 6, &mut actor.customization.hair_color)
                        }
                        CustomizationKind::EyeColor => {
                            ("eye color", 5, &mut actor.customization.eye_color)
                        }
                    };
                    if *index > maximum {
                        return Err(format!("{name} index must be between 1 and {maximum}"));
                    }
                    *field = adjusted;
                    Ok(format!("{name} changed to {index}"))
                }
                ChatCommand::SetNightLight(color) => {
                    let actor = simulation
                        .0
                        .actors
                        .get_mut(&actor_id)
                        .ok_or_else(|| "join before customizing your night light".to_owned())?;
                    actor.customization.night_light_color = Some(*color);
                    Ok(format!("night light changed to {}", format_rgb(*color)))
                }
                ChatCommand::SetNameColor(color) => {
                    let actor = simulation
                        .0
                        .actors
                        .get_mut(&actor_id)
                        .ok_or_else(|| "join before customizing your name colour".to_owned())?;
                    actor.customization.name_color = Some(*color);
                    Ok(format!("name colour changed to {}", format_rgb(*color)))
                }
                ChatCommand::SetBuildingNightLight {
                    building,
                    index,
                    color,
                } => {
                    if !simulation.0.is_ruler(&actor_id) {
                        return Err("this command is restricted to the current Ruler".to_owned());
                    }
                    let (_, runtime_id) =
                        numbered_building_instance_id(&content.0, &simulation.0, building, *index)?;
                    simulation
                        .0
                        .building_night_light_colors
                        .insert(runtime_id, *color);
                    Ok(format!(
                        "building {} BID {index} night light changed to {}",
                        building_command_name(
                            &content.0.buildings[&building_definition_id(&content.0, building,)?]
                        ),
                        format_rgb(*color)
                    ))
                }
                ChatCommand::Pets | ChatCommand::Pet(_) => {
                    Err("pet commands are not implemented yet".to_owned())
                }
                ChatCommand::Camera(actions) => {
                    require_world_view_operator(&simulation.0, &pending, user_type)?;
                    queues.camera.0.push_back(CameraRequest {
                        reset: false,
                        actions: actions.clone(),
                        follow: None,
                        focus_building: None,
                    });
                    Ok("camera request queued".to_owned())
                }
                ChatCommand::ResetCamera => {
                    require_world_view_operator(&simulation.0, &pending, user_type)?;
                    queues.camera.0.push_back(CameraRequest {
                        reset: true,
                        actions: Vec::new(),
                        follow: None,
                        focus_building: None,
                    });
                    Ok("camera reset queued".to_owned())
                }
                ChatCommand::Follow(requested) => {
                    let (_, _, controller) = view
                        .cameras
                        .single()
                        .map_err(|_| "the town camera is unavailable".to_owned())?;
                    if !auto_camera_is_managed(controller) {
                        return Err(
                            "!follow is available only while the automatic camera director is active"
                                .to_owned(),
                        );
                    }
                    let target = resolve_auto_camera_follow_target(
                        &simulation.0,
                        &actor_id,
                        requested.as_ref(),
                    )?;
                    let name = simulation.0.actors[&target]
                        .display_name
                        .as_deref()
                        .unwrap_or(target.as_str())
                        .to_owned();
                    queues.camera.0.push_back(CameraRequest {
                        reset: false,
                        actions: Vec::new(),
                        follow: Some(target),
                        focus_building: None,
                    });
                    Ok(format!("automatic camera is now following {name}"))
                }
                ChatCommand::FocusBuilding { building, index } => {
                    let (_, _, controller) = view
                        .cameras
                        .single()
                        .map_err(|_| "the town camera is unavailable".to_owned())?;
                    if !auto_camera_is_managed(controller)
                        && !queues.placers.0.contains_key(&actor_id)
                    {
                        return Err(
                            "!focus is available only during automatic camera direction or building placement"
                                .to_owned(),
                        );
                    }
                    let (definition, runtime_id) =
                        numbered_building_instance_id(&content.0, &simulation.0, building, *index)?;
                    let name = content.0.buildings[&definition].display_name.clone();
                    queues.camera.0.push_back(CameraRequest {
                        reset: false,
                        actions: Vec::new(),
                        follow: None,
                        focus_building: Some(runtime_id),
                    });
                    Ok(format!(
                        "camera focused on {name} BID {index} for 15 seconds"
                    ))
                }
                ChatCommand::PathfindingView => {
                    require_world_view_operator(&simulation.0, &pending, user_type)?;
                    if view.fine_navigation.grid.is_none() {
                        return Err("the live pathfinding grid is not ready yet".to_owned());
                    }
                    view.diagnostic.activate(WorldDiagnosticMode::Pathfinding);
                    Ok("pathfinding accessibility view enabled for 10 seconds".to_owned())
                }
                ChatCommand::FloorplanView => {
                    require_world_view_operator(&simulation.0, &pending, user_type)?;
                    view.diagnostic.activate(WorldDiagnosticMode::Floorplan);
                    Ok("placement footprint view enabled for 10 seconds".to_owned())
                }
                ChatCommand::ModRole { player, role } => {
                    require_staff(&pending)?;
                    let player = resolve_player_id(&simulation.0, player)
                        .ok_or_else(|| format!("unknown player {player}"))?;
                    let role = prefixed_id(role, "role:")
                        .filter(|role| content.0.roles.contains_key(role))
                        .ok_or_else(|| format!("unknown role {role}"))?;
                    if role.as_str() == "role:ruler" && !simulation.0.is_ruler(&player) {
                        return Err("the Ruler role is assigned by election".to_owned());
                    }
                    if !role_is_available(&content.0, &simulation.0, &role, Some(&player)) {
                        return Err(format!("the {role} role is full"));
                    }
                    simulation
                        .0
                        .assign_role(&player, role.clone())
                        .map_err(|error| error.to_string())?;
                    Ok(format!("changed {player} to {role}"))
                }
                ChatCommand::Roles => {
                    let roles = content
                        .0
                        .roles
                        .iter()
                        .filter(|(id, _)| id.as_str() != "role:ruler")
                        .filter(|(id, _)| role_is_available(&content.0, &simulation.0, id, None))
                        .map(|(_, role)| role.display_name.as_str())
                        .collect::<Vec<_>>();
                    let role_list = if roles.is_empty() {
                        "none currently available".to_owned()
                    } else {
                        roles.join(", ")
                    };
                    Ok(format!(
                        "Available roles: {role_list}. Use !role <role> after joining; use !info <role> for details."
                    ))
                }
                ChatCommand::TownStats => Ok(format!(
                    "town: {} players, {} recruits, {} buildings, day {}, {:?}/{:?}, resources {}",
                    simulation
                        .0
                        .actors
                        .values()
                        .filter(|actor| actor.id.as_str().starts_with("twitch:"))
                        .count(),
                    capacity_recruited_actor_ids(&simulation.0).len(),
                    hud_building_count(&content.0, &simulation.0),
                    simulation.0.day,
                    simulation.0.season,
                    simulation.0.weather,
                    simulation
                        .0
                        .town_resources
                        .iter()
                        .map(|(resource, amount)| format!("{resource}={amount}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )),
                ChatCommand::Population => {
                    let recruits = capacity_recruited_actor_ids(&simulation.0)
                        .into_iter()
                        .collect::<BTreeSet<_>>();
                    let populations = content
                        .0
                        .roles
                        .iter()
                        .filter(|(id, _)| id.as_str() != "role:ruler")
                        .filter(|(id, role)| {
                            !role.has_user_limit
                                || role_capacity(&content.0, &simulation.0, id)
                                    .is_some_and(|capacity| capacity > 0)
                        })
                        .map(|(id, role)| {
                            let players = simulation
                                .0
                                .actors
                                .values()
                                .filter(|actor| {
                                    actor.alive
                                        && actor.role == *id
                                        && is_stream_player_actor(&actor.id)
                                })
                                .count();
                            let npc_recruits = simulation
                                .0
                                .actors
                                .values()
                                .filter(|actor| {
                                    actor.alive && actor.role == *id && recruits.contains(&actor.id)
                                })
                                .count();
                            format!("{} P{players}/R{npc_recruits}", role.display_name)
                        })
                        .collect::<Vec<_>>();
                    Ok(format!(
                        "Population by role (P=players, R=recruits) — {}",
                        populations.join("; ")
                    ))
                }
                ChatCommand::Discord => {
                    Ok("Stream Town Discord: https://discord.gg/By4jvks".to_owned())
                }
                ChatCommand::Info { item, instance } => {
                    item_info(&content.0, &simulation.0, item, *instance)
                }
                ChatCommand::ToggleBuildCosts => {
                    require_game_master(&config.0, &pending)?;
                    let enabled = simulation.0.toggle_building_costs();
                    Ok(format!(
                        "Buildings Cost Resources: {}",
                        if enabled { "True" } else { "False" }
                    ))
                }
                ChatCommand::ToggleRoleLimits => {
                    require_game_master(&config.0, &pending)?;
                    let enabled = simulation.0.toggle_role_limits();
                    Ok(format!(
                        "Player Role Limits: {}",
                        if enabled { "True" } else { "False" }
                    ))
                }
                ChatCommand::AddResource { resource, amount } => {
                    require_game_master(&config.0, &pending)?;
                    let resource = prefixed_id(resource, "resource:")
                        .filter(|resource| {
                            matches!(
                                resource.as_str(),
                                "resource:wood"
                                    | "resource:ore"
                                    | "resource:food"
                                    | "resource:gold"
                                    | "resource:recruit"
                            )
                        })
                        .ok_or_else(|| format!("unknown town resource {resource}"))?;
                    let total = simulation.0.adjust_town_resource(resource.clone(), *amount);
                    Ok(format!("{resource} changed by {amount}; total {total}"))
                }
                ChatCommand::KillPlayer(requested) => {
                    require_game_master(&config.0, &pending)?;
                    let target = resolve_player_id(&simulation.0, requested)
                        .ok_or_else(|| format!("unknown player {requested}"))?;
                    simulation
                        .0
                        .damage_actor(&target, u32::MAX)
                        .map_err(|error| error.to_string())?;
                    Ok(format!("killed {target}"))
                }
                ChatCommand::GameMasterRevive(requested) => {
                    require_game_master(&config.0, &pending)?;
                    let target = resolve_player_id(&simulation.0, requested)
                        .ok_or_else(|| format!("unknown player {requested}"))?;
                    let position = simulation.0.actors[&target].position;
                    let spawn = nearest_walkable(&world.generated, position).unwrap_or(position);
                    simulation
                        .0
                        .respawn_actor(&target, spawn)
                        .map_err(|error| error.to_string())?;
                    queues.agent.0.push_back(AgentCommand::Teleport {
                        actor: target.clone(),
                        position: spawn,
                    });
                    spawn_healing_effect(
                        &mut ecs,
                        &presentation.0,
                        &render,
                        grid_to_world_on_surface(spawn, &config.0, &world.generated),
                        HealingEffectKind::Revive,
                        config.0.world.cell_size,
                        Some(target.clone()),
                    );
                    Ok(format!("revived {target} without a food cost"))
                }
                ChatCommand::GiveExperience { player, amount } => {
                    require_game_master(&config.0, &pending)?;
                    let target = resolve_player_id(&simulation.0, player)
                        .ok_or_else(|| format!("unknown player {player}"))?;
                    let multiplier = simulation
                        .0
                        .actors
                        .get(&target)
                        .and_then(|actor| content.0.roles.get(&actor.role))
                        .map_or(1_000, |role| role.experience_multiplier_per_thousand);
                    let levels = simulation
                        .0
                        .grant_role_experience_capped(
                            &target,
                            *amount,
                            multiplier,
                            actor_role_level_cap(&content.0, &target),
                            content.0.progression.role_experience_curve_level_span,
                            content.0.progression.role_experience_curve_maximum,
                        )
                        .map_err(|error| error.to_string())?;
                    Ok(format!(
                        "gave {target} {amount} experience; {levels} levels gained"
                    ))
                }
                ChatCommand::GiveExperienceAll(amount) => {
                    require_game_master(&config.0, &pending)?;
                    let players = simulation
                        .0
                        .actors
                        .values()
                        .filter(|actor| actor.role.as_str() != "role:enemy")
                        .map(|actor| actor.id.clone())
                        .collect::<Vec<_>>();
                    for player in &players {
                        let multiplier = simulation
                            .0
                            .actors
                            .get(player)
                            .and_then(|actor| content.0.roles.get(&actor.role))
                            .map_or(1_000, |role| role.experience_multiplier_per_thousand);
                        simulation
                            .0
                            .grant_role_experience_capped(
                                player,
                                *amount,
                                multiplier,
                                actor_role_level_cap(&content.0, player),
                                content.0.progression.role_experience_curve_level_span,
                                content.0.progression.role_experience_curve_maximum,
                            )
                            .map_err(|error| error.to_string())?;
                    }
                    Ok(format!(
                        "gave {} players {amount} experience",
                        players.len()
                    ))
                }
                ChatCommand::LevelUpPlayer { player, amount } => {
                    require_game_master(&config.0, &pending)?;
                    let target = resolve_player_id(&simulation.0, player)
                        .ok_or_else(|| format!("unknown player {player}"))?;
                    let gained = simulation
                        .0
                        .grant_role_levels_capped(
                            &target,
                            *amount,
                            actor_role_level_cap(&content.0, &target),
                        )
                        .map_err(|error| error.to_string())?;
                    Ok(format!("leveled {target} by {gained}"))
                }
                ChatCommand::GivePet { player, pet } => {
                    require_game_master(&config.0, &pending)?;
                    let _ = (player, pet);
                    Err("pet commands are not implemented yet".to_owned())
                }
                ChatCommand::QueueEvent(requested) => {
                    require_game_master(&config.0, &pending)?;
                    let event = match requested.as_str().trim_start_matches("event:") {
                        "fishgod" | "fish_god" => TownEvent::FishGod,
                        "monsterraid" | "monster_raid" | "raid" => TownEvent::EnemyRaid,
                        _ => return Err(format!("unsupported Unity queue event {requested}")),
                    };
                    if !simulation.0.queue_event(event.clone()) {
                        return Err(format!("{event:?} is already active or queued"));
                    }
                    Ok(format!("queued {event:?}"))
                }
                ChatCommand::StopEvent => {
                    require_game_master(&config.0, &pending)?;
                    if simulation.0.active_event.is_none() {
                        return Err("there is no active event".to_owned());
                    }
                    for enemy in simulation.0.stop_active_event() {
                        simulation.0.actors.remove(&enemy);
                        queues.agent.0.push_back(AgentCommand::Despawn(enemy));
                    }
                    Ok("stopped the current event".to_owned())
                }
                ChatCommand::CompleteObjective => {
                    require_game_master(&config.0, &pending)?;
                    simulation
                        .0
                        .force_complete_first_goal()
                        .map(|technology| format!("completed the goal for {technology}"))
                        .ok_or_else(|| "there is no active technology goal".to_owned())
                }
                ChatCommand::RandomTechnology => {
                    require_game_master(&config.0, &pending)?;
                    if simulation.0.active_goals.len() >= MAX_TOWN_GOALS {
                        return Err("the town already has the maximum active goals".to_owned());
                    }
                    let technology = eligible_technology_ids(&content.0, &simulation.0)
                        .into_iter()
                        .next()
                        .ok_or_else(|| "there are no eligible technologies".to_owned())?;
                    let node = &content.0.technology.nodes[&technology];
                    if !simulation.0.start_technology_goal(
                        technology.clone(),
                        &node.objectives,
                        &content.0.objectives,
                        MAX_TOWN_GOALS,
                    ) {
                        return Err(format!("could not start {}", node.display_name));
                    }
                    Ok(format!("started technology goal {}", node.display_name))
                }
                ChatCommand::TechnologyVote => {
                    require_game_master(&config.0, &pending)?;
                    let options = technology_ballot_options(&content.0, &simulation.0);
                    if options.is_empty() {
                        return Err("there are no eligible technologies".to_owned());
                    }
                    let option_count = options.len();
                    simulation
                        .0
                        .start_technology_ballot(options, TECHNOLOGY_VOTE_DURATION_SECONDS)
                        .map_err(|error| error.to_string())?;
                    Ok(format!(
                        "started a 60-second technology vote with {option_count} options"
                    ))
                }
                ChatCommand::GameEventAction => {
                    require_game_master(&config.0, &pending)?;
                    if simulation.0.fish_god.is_none() {
                        return Err("the current event has no game-master action".to_owned());
                    }
                    simulation
                        .0
                        .action_fish_god()
                        .map(|completed| {
                            if completed {
                                "completed the Fish God event action".to_owned()
                            } else {
                                let event = simulation
                                    .0
                                    .fish_god
                                    .as_ref()
                                    .expect("event remains active");
                                format!(
                                    "Fish God action {}/{}",
                                    event.praises_given, event.praises_required
                                )
                            }
                        })
                        .map_err(|error| error.to_string())
                }
                ChatCommand::UnlockAllTechnology => {
                    require_game_master(&config.0, &pending)?;
                    let count = unlock_reachable_technologies(&content.0, &mut simulation.0, false);
                    Ok(format!("unlocked {count} reachable technologies"))
                }
                ChatCommand::UnlockAgeTwo => {
                    require_game_master(&config.0, &pending)?;
                    let count = unlock_reachable_technologies(&content.0, &mut simulation.0, true);
                    Ok(format!("unlocked {count} Age 1 technologies"))
                }
                ChatCommand::ResetId { kind, value } => {
                    require_game_master(&config.0, &pending)?;
                    if kind.as_str() != "building" {
                        return Err("Unity resetid supports only building IDs".to_owned());
                    }
                    let building = building_definition_id(&content.0, value)?;
                    let count = building_instance_ids(&content.0, &simulation.0, &building).len();
                    Ok(format!(
                        "{count} {} instances already use stable IDs; no counter reset was required",
                        content.0.buildings[&building].display_name
                    ))
                }
                ChatCommand::TriggerEvent(event) => {
                    require_staff(&pending)?;
                    town_event_from_id(event)
                        .ok_or_else(|| format!("unknown event {}", event.as_str()))
                        .and_then(|event| {
                            if event == TownEvent::EnemyRaid {
                                if simulation.0.active_raid.is_some() {
                                    return Err("a raid is already active".to_owned());
                                }
                                let enemy = archetype_id_by_source(
                                    &content.0,
                                    ArchetypeKind::Enemy,
                                    "Enemy_Minotaur.prefab",
                                )
                                .ok_or_else(|| "raid enemy archetype is unavailable".to_owned())?;
                                let boss = archetype_id_by_source(
                                    &content.0,
                                    ArchetypeKind::Enemy,
                                    "Enemy_MinotaurBoss.prefab",
                                )
                                .ok_or_else(|| "raid boss archetype is unavailable".to_owned())?;
                                let enemies_per_wave = event_adjusted_wave_size(
                                    raid_enemies_per_wave(simulation_player_count(&simulation.0)),
                                    simulation.0.active_community_event,
                                );
                                if !simulation.0.start_raid(5, enemies_per_wave, enemy, boss) {
                                    return Err("raid settings are invalid".to_owned());
                                }
                            } else if event == TownEvent::FishGod {
                                if !simulation.0.start_fish_god(true) {
                                    return Err("another event is active".to_owned());
                                }
                            } else {
                                simulation.0.trigger_event(event);
                            }
                            Ok("event started".to_owned())
                        })
                }
                ChatCommand::Revive(requested) => {
                    let self_revive = requested.is_none();
                    let target_id = requested
                        .as_ref()
                        .map_or_else(
                            || Some(actor_id.clone()),
                            |requested| prefixed_id(requested, "twitch:"),
                        )
                        .ok_or_else(|| "invalid revive target".to_owned());
                    target_id.and_then(|target_id| {
                        if !self_revive {
                            let role = simulation
                                .0
                                .actors
                                .get(&actor_id)
                                .map(|actor| actor.role.as_str())
                                .ok_or_else(|| "join before reviving another player".to_owned())?;
                            if !matches!(role, "role:priest" | "role:paladin") {
                                return Err(
                                    "only a Priest or Paladin can revive another player".to_owned()
                                );
                            }
                            if target_id == actor_id {
                                return Err(
                                    "use !revive without a target to revive yourself".to_owned()
                                );
                            }
                        }
                        let position = simulation
                            .0
                            .actors
                            .get(&target_id)
                            .map(|actor| actor.position)
                            .ok_or_else(|| format!("unknown player {target_id}"))?;
                        let spawn =
                            nearest_walkable(&world.generated, position).unwrap_or(position);
                        let maximum_health =
                            simulation.0.actors.get(&target_id).map_or(0, |actor| {
                                u32::try_from(actor.max_health.max(0)).unwrap_or(u32::MAX)
                            });
                        simulation
                            .0
                            .revive_actor_with_food_cost(
                                &target_id,
                                spawn,
                                if self_revive { 400 } else { 200 },
                            )
                            .map_err(|error| error.to_string())?;
                        spawn_healing_effect(
                            &mut ecs,
                            &presentation.0,
                            &render,
                            grid_to_world_on_surface(spawn, &config.0, &world.generated),
                            HealingEffectKind::Revive,
                            config.0.world.cell_size,
                            Some(target_id.clone()),
                        );
                        if !self_revive {
                            let experience_multiplier = content
                                .0
                                .roles
                                .get(&simulation.0.actors[&actor_id].role)
                                .map_or(1_000, |role| role.experience_multiplier_per_thousand);
                            let _ = simulation.0.grant_role_experience_capped(
                                &actor_id,
                                maximum_health,
                                experience_multiplier,
                                actor_role_level_cap(&content.0, &actor_id),
                                content.0.progression.role_experience_curve_level_span,
                                content.0.progression.role_experience_curve_maximum,
                            );
                        }
                        Ok(format!("revived {target_id}"))
                    })
                }
                ChatCommand::Praise => Err("!praise is not implemented yet".to_owned()),
                ChatCommand::Experience => simulation
                    .0
                    .actors
                    .get(&actor_id)
                    .ok_or_else(|| "join before checking experience".to_owned())
                    .and_then(|actor| {
                        effective_role_stats(&content.0, &simulation.0, actor)
                            .map(|stats| {
                                let role = content
                                    .0
                                    .roles
                                    .get(&actor.role)
                                    .map_or(actor.role.as_str(), |role| role.display_name.as_str());
                                format!(
                                    "you are a level ({}/100) {role}. Current Exp: ({}/{}).",
                                    stats.level, stats.experience, stats.required_experience
                                )
                            })
                            .ok_or_else(|| format!("{} has no authored progression", actor.role))
                    }),
                ChatCommand::Save => {
                    require_ruler_or_staff(&simulation.0, &pending)?;
                    let snapshot =
                        snapshot_world(&world, &stats, &simulation, &command_save.traversal_wear);
                    command_save
                        .save
                        .store
                        .write(&snapshot)
                        .map(|()| "town saved".to_owned())
                        .map_err(|error| format!("save failed: {error}"))
                }
                ChatCommand::Help => Ok(format!(
                    "All Stream Town commands: {TWITCH_COMMAND_HELP_URL}"
                )),
            }
        })();
        let succeeded = result.is_ok();
        if succeeded && matches!(command, ChatCommand::ConfirmBuilding) {
            timelapse_builds.write(runtime::timelapse::CityTimelapseBuildConfirmed);
        }
        if succeeded
            && command_refreshes_world_diagnostic(&command)
            && is_world_diagnostic_timer_operator(&simulation.0, &actor_id, user_type)
        {
            view.diagnostic.refresh_if_active();
        }
        let refreshed_building_focus = succeeded
            .then(|| {
                command_refreshes_building_focus(&command)
                    .then(|| {
                        view.cameras.single().ok().and_then(|(_, _, controller)| {
                            controller.temporary_focus.as_ref().and_then(|focus| {
                                let CameraFocusTarget::Building(building) = &focus.target else {
                                    return None;
                                };
                                Some(building.clone())
                            })
                        })
                    })
                    .flatten()
            })
            .flatten();
        if let Some(building) = refreshed_building_focus.as_ref() {
            queues.camera.0.push_back(CameraRequest {
                reset: false,
                actions: Vec::new(),
                follow: None,
                focus_building: Some(building.clone()),
            });
        }
        let message = match result {
            Ok(message) => message,
            Err(error) => format!("command rejected: {error}"),
        };
        response.feedback.0 = format!("{}: {message}", pending.display_name);
        if command_interrupts_auto_camera(&command) && refreshed_building_focus.is_none() {
            response.acknowledgements.acknowledge();
        }
        if let Some(outbound) =
            unity_outbound_reply(&command, succeeded, &message, &pending.display_name)
        {
            send_command_feedback(&response.connection, outbound);
        }
        info!(user = %pending.display_name, ?command, result = %message, "processed Twitch command");
        stats.commands_processed += 1;
    }
}

pub(crate) fn format_rgb(color: [u8; 3]) -> String {
    format!("#{:02X}{:02X}{:02X}", color[0], color[1], color[2])
}

pub(crate) fn update_hud(
    stats: Res<SessionStats>,
    twitch: Res<TwitchConnection>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    feedback: Res<CommandFeedback>,
    agents: Query<&Agent>,
    mut hud: Single<&mut Text, With<Hud>>,
    mut metrics: HudMetricTextQuery,
    mut maximums: HudMetricMaximumTextQuery,
    mut technology_texts: HudTechnologyTextQuery,
    mut technology_progress: Single<
        &mut Node,
        (With<HudTechnologyProgressFill>, Without<SeasonMeter>),
    >,
    mut season_meters: Query<&mut Node, (With<SeasonMeter>, Without<HudTechnologyProgressFill>)>,
) {
    if !stats.is_changed()
        && !twitch.is_changed()
        && !feedback.is_changed()
        && !simulation.is_changed()
    {
        return;
    }
    let first_id = agents
        .iter()
        .next()
        .map_or("none", |agent| agent.id.as_str());
    let gathering = agents
        .iter()
        .filter(|agent| matches!(agent.goal, AgentGoal::Gather(_) | AgentGoal::HarvestFarm(_)))
        .count();
    let depositing = agents
        .iter()
        .filter(|agent| agent.goal == AgentGoal::Deposit)
        .count();
    let attacking = agents
        .iter()
        .filter(|agent| {
            matches!(
                agent.goal,
                AgentGoal::Attack(_) | AgentGoal::AttackBuilding(_)
            )
        })
        .count();
    let healing = agents
        .iter()
        .filter(|agent| matches!(agent.goal, AgentGoal::Heal(_)))
        .count();
    let constructing = agents
        .iter()
        .filter(|agent| matches!(agent.goal, AgentGoal::Construct(_)))
        .count();
    let incomplete_buildings = simulation
        .0
        .buildings
        .values()
        .filter(|building| !building.complete)
        .count();
    let building_levels = simulation
        .0
        .buildings
        .values()
        .map(|building| u64::from(building.level))
        .sum::<u64>();
    let dead = simulation
        .0
        .actors
        .values()
        .filter(|actor| !actor.alive)
        .count();
    let time_cycle = config.0.time.sample(simulation.0.elapsed_seconds);
    let day_phase = if time_cycle.is_daytime {
        "Day"
    } else {
        "Night"
    };
    for (metric, mut text) in &mut metrics {
        text.0 = match metric {
            HudMetric::Food => town_resource_amount(&simulation.0, "resource:food").to_string(),
            HudMetric::Gold => town_resource_amount(&simulation.0, "resource:gold").to_string(),
            HudMetric::Ore => town_resource_amount(&simulation.0, "resource:ore").to_string(),
            HudMetric::Wood => town_resource_amount(&simulation.0, "resource:wood").to_string(),
            HudMetric::Players => agents
                .iter()
                .filter(|agent| {
                    agent.kind == ActorKind::Player && is_stream_player_actor(&agent.id)
                })
                .count()
                .to_string(),
            HudMetric::Npcs => agents
                .iter()
                .filter(|agent| {
                    agent.kind == ActorKind::Player
                        && !is_stream_player_actor(&agent.id)
                        && !is_guardhouse_defender_id(&agent.id)
                })
                .count()
                .to_string(),
            HudMetric::Buildings => hud_building_count(&content.0, &simulation.0).to_string(),
            HudMetric::PlayTime => hud_play_time(stats.elapsed_seconds),
        };
    }
    for (maximum, mut text) in &mut maximums {
        let capacity = match maximum.0 {
            HudMetric::Food => Some(resource_storage_capacity(
                &config.0,
                &content.0,
                &simulation.0,
                &StableId::new("resource:food").expect("static stable ID"),
            )),
            HudMetric::Gold => Some(resource_storage_capacity(
                &config.0,
                &content.0,
                &simulation.0,
                &StableId::new("resource:gold").expect("static stable ID"),
            )),
            HudMetric::Ore => Some(resource_storage_capacity(
                &config.0,
                &content.0,
                &simulation.0,
                &StableId::new("resource:ore").expect("static stable ID"),
            )),
            HudMetric::Wood => Some(resource_storage_capacity(
                &config.0,
                &content.0,
                &simulation.0,
                &StableId::new("resource:wood").expect("static stable ID"),
            )),
            HudMetric::Npcs => Some(resource_storage_capacity(
                &config.0,
                &content.0,
                &simulation.0,
                &StableId::new("resource:recruit").expect("static stable ID"),
            )),
            HudMetric::Players | HudMetric::Buildings | HudMetric::PlayTime => None,
        };
        text.0 = capacity.map_or_else(String::new, |capacity| {
            if capacity == u32::MAX {
                "/∞".to_owned()
            } else {
                format!("/{capacity}")
            }
        });
    }
    let technology_summary = hud_technology_summary(&content.0, &simulation.0);
    for (kind, mut text) in &mut technology_texts {
        text.0 = match kind {
            HudTechnologyTextKind::Title => technology_summary.title.clone(),
            HudTechnologyTextKind::Requirement => technology_summary.requirement.clone(),
        };
    }
    technology_progress.width = percent(technology_summary.progress * 100.0);
    let season_progress =
        hud_season_meter_percent(simulation.0.elapsed_seconds, config.0.time.seconds_per_day);
    for mut meter in &mut season_meters {
        meter.left = percent(season_progress);
    }
    hud.0 = format!(
        "Day {} ({day_phase}) | {} routes | workers {gathering}/{depositing}/{constructing} | construction {incomplete_buildings}, levels {building_levels} | combat {attacking}/{healing}/{dead} | {} commands | {:?}/{:?} | Twitch {}\nRecruit {} | Goals {} | Event {} | Governance {} | {}\nText commands active | !help lists commands | ESC menu | F12 capture | first {first_id}",
        simulation.0.day,
        stats.paths_completed,
        stats.commands_processed,
        simulation.0.season,
        simulation.0.weather,
        twitch_status_text(&twitch),
        town_resource_amount(&simulation.0, "resource:recruit"),
        town_goal_status(&content.0, &simulation.0),
        active_event_text(&simulation.0),
        ruler_status(&simulation.0),
        feedback.0,
    );
}

pub(crate) fn hud_play_time(elapsed_seconds: f64) -> String {
    let total = std::time::Duration::from_secs_f64(elapsed_seconds.max(0.0)).as_secs();
    format!("{:02}:{:02}", total / 60, total % 60)
}

pub(crate) fn hud_season_meter_percent(elapsed_seconds: f64, seconds_per_day: u32) -> f32 {
    let year_seconds = f64::from(seconds_per_day.max(1))
        * f64::from(DAYS_PER_SEASON)
        * f64::from(SEASONS_PER_YEAR);
    let progress = elapsed_seconds.max(0.0).rem_euclid(year_seconds) / year_seconds;
    (Duration::from_secs_f64(progress).as_secs_f32() * 96.0).clamp(0.0, 96.0)
}

pub(crate) fn town_goal_status(content: &ContentCatalog, simulation: &WorldSimulation) -> String {
    let Some(goal) = simulation.active_goals.first() else {
        return "none".to_owned();
    };
    let name = content
        .technology
        .nodes
        .get(&goal.technology)
        .map_or(goal.technology.as_str(), |technology| {
            technology.display_name.as_str()
        });
    let progress = goal
        .objectives
        .iter()
        .filter_map(|progress| {
            content
                .objectives
                .get(&progress.objective)
                .map(|definition| {
                    format!(
                        "{:?} {}/{}",
                        definition.kind, progress.amount, progress.required_amount
                    )
                })
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("{name} [{progress}]")
}

#[derive(Debug, PartialEq)]
pub(crate) struct HudTechnologySummary {
    pub(crate) title: String,
    pub(crate) requirement: String,
    pub(crate) progress: f32,
}

pub(crate) fn hud_technology_summary(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> HudTechnologySummary {
    let Some(goal) = simulation.active_goals.first() else {
        return if simulation.active_vote.is_some() {
            HudTechnologySummary {
                title: "NEXT TECHNOLOGY".to_owned(),
                requirement: "Vote in chat now".to_owned(),
                progress: 0.0,
            }
        } else {
            HudTechnologySummary {
                title: "TECHNOLOGY".to_owned(),
                requirement: "Ballot pending".to_owned(),
                progress: 0.0,
            }
        };
    };
    let technology = content.technology.nodes.get(&goal.technology).map_or_else(
        || title_case(goal.technology.as_str().trim_start_matches("tech:")),
        |technology| compact_technology_label(&technology.display_name).replace('\n', " "),
    );
    let objective = goal
        .objectives
        .iter()
        .find(|objective| objective.amount < objective.required_amount)
        .or_else(|| goal.objectives.first());
    let Some(objective) = objective else {
        return HudTechnologySummary {
            title: fit_hud_label(&technology, 34),
            requirement: "Complete".to_owned(),
            progress: 1.0,
        };
    };
    let label = content
        .objectives
        .get(&objective.objective)
        .map_or_else(|| "Complete objective".to_owned(), objective_display_label);
    HudTechnologySummary {
        title: fit_hud_label(&technology, 34),
        requirement: format!(
            "{} {}/{}",
            fit_hud_label(&label, 26),
            objective.amount.min(objective.required_amount),
            objective.required_amount
        ),
        progress: objective_progress_ratio(objective.amount, objective.required_amount),
    }
}

pub(crate) fn fit_hud_label(label: &str, max_chars: usize) -> String {
    if label.chars().count() <= max_chars {
        return label.to_owned();
    }
    label
        .chars()
        .take(max_chars.saturating_sub(1))
        .chain(std::iter::once('…'))
        .collect()
}

pub(crate) fn town_resource_amount(simulation: &WorldSimulation, resource: &str) -> u32 {
    StableId::new(resource)
        .ok()
        .and_then(|resource| simulation.town_resources.get(&resource).copied())
        .unwrap_or_default()
}

pub(crate) fn active_event_text(simulation: &WorldSimulation) -> String {
    if let Some(raid) = &simulation.active_raid {
        format!(
            "raid wave {}/{} ({} enemies)",
            raid.current_wave,
            raid.total_waves,
            raid.tracked_enemies.len()
        )
    } else if let Some(event) = &simulation.fish_god {
        format!(
            "Fish God {}/{} ({:.0}s)",
            event.praises_given, event.praises_required, event.remaining_seconds
        )
    } else {
        simulation
            .active_event
            .as_ref()
            .map_or_else(|| "none".to_owned(), |event| format!("{event:?}"))
    }
}

pub(crate) fn ruler_status(simulation: &WorldSimulation) -> String {
    if let Some(vote) = &simulation.ruler_vote {
        let options = vote
            .option_order
            .iter()
            .map(|option| {
                let count = vote
                    .votes
                    .values()
                    .filter(|selected| *selected == option)
                    .count();
                let label = simulation
                    .actors
                    .get(option)
                    .and_then(|actor| actor.display_name.as_deref())
                    .unwrap_or(option.as_str());
                format!("{label} {count}")
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "{:?} {:.0}s [{}]",
            vote.kind, vote.remaining_seconds, options
        )
    } else if let Some(ruler) = &simulation.current_ruler {
        let name = simulation
            .actors
            .get(ruler)
            .and_then(|actor| actor.display_name.as_deref())
            .unwrap_or(ruler.as_str());
        format!(
            "Ruler {name}; retention vote in {:.0}s",
            simulation.ruler_vote_cooldown_seconds
        )
    } else {
        format!(
            "no ruler; election in {:.0}s",
            simulation.ruler_vote_cooldown_seconds
        )
    }
}

pub(crate) fn twitch_status_text(connection: &TwitchConnection) -> String {
    match &connection.status {
        TwitchStatus::Error(error) => format!("error: {error}"),
        status => format!("{status:?}"),
    }
}
