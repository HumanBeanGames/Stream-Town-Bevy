use super::super::*;

pub(crate) fn sync_world_render_lod(
    cameras: Query<&Transform, With<TownCamera>>,
    mut chunks: Query<(&mut TerrainChunkLod, &mut Mesh3d)>,
    mut stats: ResMut<WorldRenderStats>,
) {
    if chunks.is_empty() {
        // Runtime-generated worlds now use one continuous authored-style mesh.
        // Keep the legacy report fields meaningful for existing tooling.
        stats.terrain_high_chunks = 1;
        stats.terrain_medium_chunks = 0;
        stats.terrain_low_chunks = 0;
        return;
    }
    let Ok(camera) = cameras.single() else {
        return;
    };
    let focus = camera_ground_focus(camera);
    stats.terrain_high_chunks = 0;
    stats.terrain_medium_chunks = 0;
    stats.terrain_low_chunks = 0;
    for (mut chunk, mut mesh) in &mut chunks {
        let desired = terrain_lod_for_distance(chunk.current, chunk.centre.distance(focus));
        if desired != chunk.current {
            mesh.0 = match desired {
                TerrainLodLevel::High => chunk.high.clone(),
                TerrainLodLevel::Medium => chunk.medium.clone(),
                TerrainLodLevel::Low => chunk.low.clone(),
            };
            chunk.current = desired;
        }
        match chunk.current {
            TerrainLodLevel::High => stats.terrain_high_chunks += 1,
            TerrainLodLevel::Medium => stats.terrain_medium_chunks += 1,
            TerrainLodLevel::Low => stats.terrain_low_chunks += 1,
        }
    }
}

pub(crate) fn pet_model<'a>(
    archetype: &'a ArchetypeDef,
    pet: &StableId,
) -> Option<(&'a ArchetypeScene, &'a PetModelDef)> {
    let model = archetype.pet.as_ref()?.models.get(pet)?;
    let scene = archetype
        .scenes
        .iter()
        .find(|scene| scene.source_model == model.source_model)?;
    Some((scene, model))
}

pub(crate) fn pet_follow_step(
    transform: &mut Transform,
    owner_position: Vec3,
    pet: &PetDef,
    delta_seconds: f32,
) -> f32 {
    let direction = owner_position - transform.translation;
    let distance_squared = direction.length_squared();
    if distance_squared <= f32::EPSILON {
        return 0.0;
    }
    let closest_distance_squared = pet.closest_distance.powi(2);
    let max_distance_squared = pet.max_distance.powi(2);
    let movement_speed = (pet.min_move_speed
        + (distance_squared - closest_distance_squared)
            * (pet.max_move_speed - pet.min_move_speed)
            / (max_distance_squared - closest_distance_squared))
        .clamp(pet.min_move_speed, pet.max_move_speed);
    let distance = distance_squared.sqrt();
    // Unity's Pet.Update uses world-space speed directly and does not clamp an
    // overshooting frame to the remaining distance.
    let world_step = movement_speed * delta_seconds;
    transform.translation += direction / distance * world_step;

    let horizontal = Vec3::new(direction.x, 0.0, direction.z);
    if horizontal.length_squared() > f32::EPSILON {
        // The imported pet meshes face local +Z, matching Unity's Transform.forward.
        let target_rotation = Quat::from_rotation_arc(Vec3::Z, horizontal.normalize());
        transform.rotation = transform.rotation.slerp(
            target_rotation,
            (delta_seconds * pet.rotation_radians_per_second).clamp(0.0, 1.0),
        );
    }
    movement_speed
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn sync_active_pets(
    mut commands: Commands,
    time: Res<Time>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    simulation: Res<SimulationRuntime>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    owners: Query<(&Agent, &Transform), Without<ActivePetVisual>>,
    mut visuals: Query<(Entity, &mut ActivePetVisual, &mut Transform), Without<Agent>>,
) {
    let desired: BTreeMap<_, _> = simulation
        .0
        .actors
        .values()
        .filter_map(|actor| {
            actor
                .active_pet
                .as_ref()
                .map(|pet| (actor.id.clone(), pet.clone()))
        })
        .collect();
    let owner_positions: BTreeMap<_, _> = owners
        .iter()
        .map(|(agent, transform)| (agent.id.clone(), transform.translation))
        .collect();
    let Some(archetype) = content
        .0
        .archetypes
        .values()
        .find(|archetype| archetype.source_path.ends_with("Prefabs/Pets/Pet.prefab"))
    else {
        return;
    };
    let Some(pet_definition) = archetype.pet.as_ref() else {
        return;
    };
    let mut existing = BTreeSet::new();
    for (entity, mut visual, mut transform) in &mut visuals {
        let key = (visual.owner.clone(), visual.pet.clone());
        existing.insert(key.clone());
        let Some(position) = owner_positions.get(&visual.owner) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        if desired.get(&visual.owner) == Some(&visual.pet) {
            visual.movement_speed =
                pet_follow_step(&mut transform, *position, pet_definition, time.delta_secs());
        } else {
            commands.entity(entity).try_despawn();
        }
    }
    let Some(server) = asset_server.as_deref() else {
        return;
    };
    for (owner, pet) in desired {
        if existing.contains(&(owner.clone(), pet.clone())) {
            continue;
        }
        let Some(owner_position) = owner_positions.get(&owner) else {
            continue;
        };
        let Some((scene, model)) = pet_model(archetype, &pet)
            .filter(|(scene, _)| converted_asset_exists(&asset_root.0, &scene.asset_path))
        else {
            continue;
        };
        let spawn_position = if std::env::var_os("STREAM_TOWN_SMOKE_PET").is_some() {
            *owner_position + Vec3::X * pet_definition.max_distance
        } else {
            *owner_position
        };
        let mut visual = commands.spawn((
            WorldEntity,
            ActivePetVisual {
                owner,
                pet: pet.clone(),
                movement_speed: 0.0,
            },
            Transform::from_translation(spawn_position),
        ));
        if let Some(animation) = pet_animation_spec(&pet, scene, &presentation.0) {
            visual.insert(animation);
        }
        if let Some(materials) = prefab_material_spec(archetype, scene, &presentation.0, &render) {
            visual.insert(materials);
        }
        visual.with_children(|root| {
            root.spawn((
                Name::new(format!("{} model", pet.as_str())),
                WorldAssetRoot(
                    server.load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
                ),
                Transform::from_translation(Vec3::from_array(model.local_position))
                    .with_rotation(Quat::from_array(model.local_rotation).normalize())
                    .with_scale(Vec3::from_array(model.local_scale)),
            ));
        });
    }
}

pub(crate) fn default_town_camera_transform() -> Transform {
    unity_town_camera_transform(Vec3::ZERO)
}

pub(crate) fn in_game_sun_transform() -> Transform {
    // The shipping camera views the town from -X. The earlier +X sun direction
    // lit the backs of camera-facing actors and buildings; moving it to the
    // camera side preserves the elevated authored look while lighting the
    // readable front faces.
    Transform::from_xyz(-250.0, 400.0, 180.0).looking_at(Vec3::ZERO, Vec3::Y)
}

pub(crate) fn in_game_sun_transform_for_daylight(daylight: f32) -> Transform {
    let base = in_game_sun_transform();
    // Unity rotates the directional light's parent from Y=0 degrees at day to
    // Y=-120 degrees at night during the authored transition.
    let parent_rotation =
        Quat::from_rotation_y((-120.0_f32 * (1.0 - daylight.clamp(0.0, 1.0))).to_radians());
    Transform::from_translation(base.translation).with_rotation(parent_rotation * base.rotation)
}

pub(crate) const fn in_game_ambient_brightness(authored: f32) -> f32 {
    authored * IN_GAME_AMBIENT_LIGHT_MULTIPLIER
}

pub(crate) fn unity_town_camera_transform(focus: Vec3) -> Transform {
    // Retain Unity's side-on 45-degree view, with a wider shipping frame. The
    // backward shift keeps the Town Hall around the upper third instead of
    // pinning it to screen centre, leaving the playable foreground visible.
    Transform::from_translation(
        focus + UNITY_TOWN_CAMERA_OFFSET - Vec3::X * UNITY_TOWN_CAMERA_FOCUS_BACK_SHIFT,
    )
    .looking_to(Vec3::new(1.0, -1.0, 0.0).normalize(), Vec3::Y)
}

pub(crate) fn town_camera_projection(field_of_view_degrees: f32) -> Projection {
    Projection::Perspective(PerspectiveProjection {
        fov: field_of_view_degrees.to_radians(),
        near: UNITY_TOWN_CAMERA_NEAR,
        far: UNITY_TOWN_CAMERA_FAR,
        ..default()
    })
}

pub(crate) fn ping_pointer_scale(elapsed_seconds: f32) -> f32 {
    let normalized = (elapsed_seconds / PING_POINTER_DURATION_SECONDS).clamp(0.0, 1.0);
    // Exact unweighted Unity AnimationCurve keys from VFX_PointerArrow.prefab:
    // (time, value, in tangent, out tangent). Unity's weighted mode is disabled.
    let keys = [
        (0.0, 0.0, 0.0, 0.0),
        (0.25, 0.7, -0.809_009_2, -0.809_009_2),
        (0.5, 0.4, 0.000_833_988_2, 0.000_833_988_2),
        (0.75, 0.7, 0.820_234_3, 0.820_234_3),
        (1.0, 0.0, 0.0, 0.0),
    ];
    let pair = keys
        .windows(2)
        .find(|pair| normalized >= pair[0].0 && normalized <= pair[1].0)
        .unwrap_or_else(|| keys.last_chunk::<2>().expect("pointer curve has two keys"));
    let left = pair[0];
    let right = pair[1];
    let progress = (normalized - left.0) / (right.0 - left.0);
    let progress_squared = progress * progress;
    let progress_cubed = progress_squared * progress;
    let duration = right.0 - left.0;
    (2.0 * progress_cubed - 3.0 * progress_squared + 1.0) * left.1
        + (progress_cubed - 2.0 * progress_squared + progress) * duration * left.3
        + (-2.0 * progress_cubed + 3.0 * progress_squared) * right.1
        + (progress_cubed - progress_squared) * duration * right.2
}

pub(crate) fn ping_pointer_transform(
    actor_position: Vec3,
    actor_height: f32,
    base_scale: f32,
    elapsed_seconds: f32,
) -> Transform {
    let scale = base_scale * ping_pointer_scale(elapsed_seconds);
    Transform::from_translation(
        actor_position + Vec3::Y * (actor_height - PING_POINTER_MODEL_MIN_Y * scale),
    )
    .with_scale(Vec3::splat(scale))
}

pub(crate) fn spawn_ping_pointer(
    commands: &mut Commands,
    render: &RenderAssets,
    asset_server: Option<&AssetServer>,
    actor: StableId,
    position: Vec3,
    actor_height: f32,
    cell_size: f32,
) {
    let base_scale = cell_size / PING_POINTER_MODEL_HEIGHT;
    let mut pointer = commands.spawn((
        WorldEntity,
        PingPointer {
            actor,
            elapsed_seconds: 0.0,
            base_scale,
        },
        ping_pointer_transform(position, actor_height, base_scale, 0.0),
    ));
    if let Some(asset_server) = asset_server {
        pointer.insert(WorldAssetRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(PING_POINTER_MODEL_PATH.to_owned()),
        )));
        if let Some(material) = standalone_material_override(render, PING_POINTER_MATERIAL_ID) {
            pointer.insert(material);
        }
    } else {
        pointer.insert((
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.projectile.clone()),
        ));
    }
}

pub(crate) fn apply_agent_commands(
    mut commands: Commands,
    mut queue: ResMut<AgentCommandQueue>,
    config: Res<RuntimeConfig>,
    world: Res<WorldRuntime>,
    render: Res<RenderAssets>,
    asset_server: Option<Res<AssetServer>>,
    mut agents: Query<(
        Entity,
        &mut Agent,
        &mut GridLocation,
        &AgentAnimation,
        &mut Transform,
    )>,
    pointers: Query<(Entity, &PingPointer)>,
) {
    let mut pointed_actors = pointers
        .iter()
        .map(|(_, pointer)| pointer.actor.clone())
        .collect::<BTreeSet<_>>();
    while let Some(command) = queue.0.pop_front() {
        match command {
            AgentCommand::Teleport { actor, position } => {
                if let Some((_, mut agent, mut location, animation, mut transform)) = agents
                    .iter_mut()
                    .find(|(_, agent, _, _, _)| agent.id == actor)
                {
                    let mut world_position =
                        grid_to_world_on_surface(position, &config.0, &world.generated);
                    if !animation.native {
                        world_position.y += animation.base_scale.y * 0.5;
                    }
                    location.0 = position;
                    agent.spawn = position;
                    agent.origin = position;
                    agent.path.clear();
                    agent.path_index = 0;
                    agent.target = position;
                    agent.goal = AgentGoal::Wander;
                    agent.action_started = false;
                    agent.action_cooldown_seconds = 0.0;
                    transform.translation = world_position;
                }
            }
            AgentCommand::Ping(actor) => {
                if !pointed_actors.insert(actor.clone()) {
                    continue;
                }
                if let Some((_, _, _, animation, transform)) = agents
                    .iter_mut()
                    .find(|(_, agent, _, _, _)| agent.id == actor)
                {
                    let actor_height = if animation.native {
                        config.0.world.cell_size * 1.5
                    } else {
                        animation.base_scale.y * 1.5
                    };
                    spawn_ping_pointer(
                        &mut commands,
                        &render,
                        asset_server.as_deref(),
                        actor,
                        transform.translation,
                        actor_height,
                        config.0.world.cell_size,
                    );
                } else {
                    pointed_actors.remove(&actor);
                }
            }
            AgentCommand::Despawn(actor) => {
                if let Some((entity, _, _, _, _)) = agents
                    .iter_mut()
                    .find(|(_, agent, _, _, _)| agent.id == actor)
                {
                    commands.entity(entity).try_despawn();
                }
            }
        }
    }
}

pub(crate) fn animate_ping_pointers(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    simulation: Res<SimulationRuntime>,
    agents: Query<(&Agent, &AgentAnimation, &Transform)>,
    mut pointers: Query<(Entity, &mut PingPointer, &mut Transform), Without<Agent>>,
) {
    for (entity, mut pointer, mut transform) in &mut pointers {
        pointer.elapsed_seconds += time.delta_secs();
        let Some((_, animation, actor_transform)) = agents
            .iter()
            .find(|(agent, _, _)| agent.id == pointer.actor)
        else {
            commands.entity(entity).try_despawn();
            continue;
        };
        if pointer.elapsed_seconds >= PING_POINTER_DURATION_SECONDS
            || !simulation
                .0
                .actors
                .get(&pointer.actor)
                .is_some_and(|actor| actor.alive)
        {
            commands.entity(entity).try_despawn();
            continue;
        }
        let actor_height = if animation.native {
            config.0.world.cell_size * 1.5
        } else {
            animation.base_scale.y * 1.5
        };
        *transform = ping_pointer_transform(
            actor_transform.translation,
            actor_height,
            pointer.base_scale,
            pointer.elapsed_seconds,
        );
    }
}

pub(crate) fn apply_building_commands(
    mut commands: Commands,
    mut queue: ResMut<BuildingCommandQueue>,
    buildings: Query<(Entity, &RuntimeBuilding)>,
) {
    while let Some(command) = queue.0.pop_front() {
        match command {
            BuildingRuntimeCommand::Despawn(building) => {
                if let Some((entity, _)) =
                    buildings.iter().find(|(_, runtime)| runtime.id == building)
                {
                    commands.entity(entity).try_despawn();
                }
            }
        }
    }
}

pub(crate) fn is_line_building(building: &StableId) -> bool {
    matches!(building.as_str(), "building:wall" | "building:path")
}

pub(crate) fn is_path_building(building: &StableId) -> bool {
    building.as_str() == "building:path"
}

pub(crate) fn building_blocks_navigation(definition: &BuildingDef) -> bool {
    definition.archetype.as_str() != "archetype:building:path"
}

pub(crate) fn enemy_can_attack_building(
    content: &ContentCatalog,
    building: &BuildingState,
) -> bool {
    building_def_for_archetype(content, &building.archetype).is_some_and(building_blocks_navigation)
}

pub(crate) fn wall_line_cells(start: GridPos, end: GridPos) -> Result<Vec<GridPos>, String> {
    if start.x != end.x && start.z != end.z {
        return Err("line endpoints must be orthogonal (same row or same column)".to_owned());
    }
    if start.x == end.x {
        let (minimum, maximum) = if start.z <= end.z {
            (start.z, end.z)
        } else {
            (end.z, start.z)
        };
        Ok((minimum..=maximum)
            .map(|z| GridPos { x: start.x, z })
            .collect())
    } else {
        let (minimum, maximum) = if start.x <= end.x {
            (start.x, end.x)
        } else {
            (end.x, start.x)
        };
        Ok((minimum..=maximum)
            .map(|x| GridPos { x, z: start.z })
            .collect())
    }
}

pub(crate) fn path_centre_cells(placement: &BuildingPlacement) -> Vec<GridPos> {
    debug_assert!(is_path_building(&placement.building));
    if placement.line_start.is_some() && !placement.path_cells.is_empty() {
        placement.path_cells.clone()
    } else {
        vec![
            placement
                .navigation_position
                .unwrap_or_else(|| placement_to_navigation_centre(placement.position)),
        ]
    }
}

pub(crate) fn offset_navigation_cell(cell: GridPos, x: i32, z: i32) -> Option<GridPos> {
    Some(GridPos {
        x: u16::try_from(i32::from(cell.x).checked_add(x)?).ok()?,
        z: u16::try_from(i32::from(cell.z).checked_add(z)?).ok()?,
    })
}

pub(crate) fn thick_path_side_cells(placement: &BuildingPlacement) -> Vec<GridPos> {
    if !placement.thick_path || !is_path_building(&placement.building) {
        return Vec::new();
    }
    let centre = path_centre_cells(placement);
    let mut seen = centre.iter().copied().collect::<HashSet<_>>();
    let mut sides = Vec::with_capacity(centre.len().saturating_mul(2));
    for (index, cell) in centre.iter().copied().enumerate() {
        let previous = index
            .checked_sub(1)
            .and_then(|index| centre.get(index))
            .copied()
            .unwrap_or(cell);
        let next = centre.get(index + 1).copied().unwrap_or(cell);
        let mut tangent_x = (i32::from(next.x) - i32::from(previous.x)).signum();
        let mut tangent_z = (i32::from(next.z) - i32::from(previous.z)).signum();
        if tangent_x == 0 && tangent_z == 0 {
            if placement.rotation_quarter_turns.rem_euclid(2) == 0 {
                tangent_z = 1;
            } else {
                tangent_x = 1;
            }
        }
        let normal = (-tangent_z, tangent_x);
        for multiplier in [-1, 1] {
            let Some(side) =
                offset_navigation_cell(cell, normal.0 * multiplier, normal.1 * multiplier)
            else {
                continue;
            };
            if seen.insert(side) {
                sides.push(side);
            }
        }
    }
    // A diagonal centre step crosses both orthogonal neighbours. Without these
    // bridge cells, adjacent 3x1 bands only touch at a corner and leave a 2x1
    // notch in the middle of every diagonal thick-path segment.
    for step in centre.windows(2) {
        if step[0].x.abs_diff(step[1].x) != 1 || step[0].z.abs_diff(step[1].z) != 1 {
            continue;
        }
        for bridge in [
            GridPos {
                x: step[0].x,
                z: step[1].z,
            },
            GridPos {
                x: step[1].x,
                z: step[0].z,
            },
        ] {
            if seen.insert(bridge) {
                sides.push(bridge);
            }
        }
    }
    sides
}

pub(crate) fn placement_visual_cells(placement: &BuildingPlacement) -> Vec<GridPos> {
    if is_path_building(&placement.building) {
        let mut cells = path_centre_cells(placement);
        cells.extend(thick_path_side_cells(placement));
        return cells;
    }
    if is_line_building(&placement.building)
        && let Some(start) = placement.line_start
    {
        return wall_line_cells(start, placement.position)
            .unwrap_or_else(|_| vec![start, placement.position]);
    }
    vec![placement.position]
}

pub(crate) fn path_route_is_complete(placement: &BuildingPlacement) -> bool {
    let Some(start) = placement.line_start else {
        return true;
    };
    let cursor = placement
        .navigation_position
        .unwrap_or_else(|| placement_to_navigation_centre(placement.position));
    placement.path_cells.first() == Some(&start)
        && placement.path_cells.last() == Some(&cursor)
        && placement.path_cells.windows(2).all(|step| {
            let dx = step[0].x.abs_diff(step[1].x);
            let dz = step[0].z.abs_diff(step[1].z);
            dx <= 1 && dz <= 1 && (dx != 0 || dz != 0)
        })
}

pub(crate) fn candidate_building_fine_cells(
    placement: &BuildingPlacement,
    definition: &BuildingDef,
    fine_footprint: [u16; 2],
) -> Vec<GridPos> {
    if matches!(
        placement.building.as_str(),
        "building:wall" | "building:gate"
    ) {
        let rotation = placement
            .line_start
            .map_or(placement.rotation_quarter_turns, |start| {
                i32::from(start.x != placement.position.x)
            });
        let mut cells = placement_visual_cells(placement)
            .into_iter()
            .flat_map(|position| {
                rectangular_building_fine_cells(
                    position,
                    definition.footprint,
                    fine_footprint,
                    rotation,
                )
            })
            .collect::<Vec<_>>();
        cells.sort_unstable();
        cells.dedup();
        return cells;
    }
    rectangular_building_fine_cells(
        placement.position,
        definition.footprint,
        fine_footprint,
        placement.rotation_quarter_turns,
    )
}

pub(crate) fn placement_at_cell(placement: &BuildingPlacement, cell: GridPos) -> BuildingPlacement {
    let path = is_path_building(&placement.building);
    BuildingPlacement {
        building: placement.building.clone(),
        thick_path: placement.thick_path,
        position: if path {
            navigation_to_placement(cell)
        } else {
            cell
        },
        navigation_position: path.then_some(cell),
        rotation_quarter_turns: placement.rotation_quarter_turns,
        line_start: None,
        line_end: None,
        path_cells: Vec::new(),
        inactivity_seconds: placement.inactivity_seconds,
    }
}

pub(crate) fn building_placement_is_available(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    placement: &BuildingPlacement,
    definition: &BuildingDef,
    fine_navigation: Option<&stream_town_domain::NavGrid>,
) -> bool {
    if is_path_building(&placement.building) {
        let Some(navigation) = fine_navigation else {
            return false;
        };
        let gate_cells = completed_player_gate_navigation_cells(content, simulation);
        return path_route_is_complete(placement)
            && path_centre_cells(placement)
                .into_iter()
                .all(|cell| path_navigation_cell_is_traversable(navigation, &gate_cells, cell));
    }
    {
        let candidate_placement = candidate_building_fine_cells(
            placement,
            definition,
            definition
                .placement_footprint_thirds
                .unwrap_or_else(|| definition.footprint.map(|axis| axis.saturating_mul(3))),
        );
        let candidate_navigation = candidate_building_fine_cells(
            placement,
            definition,
            definition
                .navigation_footprint_thirds
                .unwrap_or_else(|| default_navigation_footprint_thirds(definition.footprint)),
        );
        let expected = definition
            .placement_footprint_thirds
            .unwrap_or_else(|| definition.footprint.map(|axis| axis.saturating_mul(3)));
        let expected = rotated_footprint(expected, placement.rotation_quarter_turns);
        let sections = if matches!(
            placement.building.as_str(),
            "building:wall" | "building:gate"
        ) {
            placement_visual_cells(placement).len()
        } else {
            1
        };
        if candidate_placement.len()
            != usize::from(expected[0]) * usize::from(expected[1]) * sections
        {
            return false;
        }
        let width = world
            .navigation
            .width()
            .saturating_mul(NAVIGATION_SUBDIVISIONS);
        let height = world
            .navigation
            .height()
            .saturating_mul(NAVIGATION_SUBDIVISIONS);
        if candidate_placement.iter().any(|cell| {
            cell.x >= width
                || cell.z >= height
                || world
                    .navigation
                    .height_at(navigation_to_placement(*cell))
                    .is_none_or(|height| height < NAVIGATION_TERRAIN_MIN_HEIGHT_CENTIMETRES)
        }) {
            return false;
        }
        let candidate_placement = candidate_placement.into_iter().collect::<HashSet<_>>();
        let candidate_navigation = candidate_navigation.into_iter().collect::<HashSet<_>>();
        if world.resources.iter().any(|resource| {
            resource.amount > 0
                && fine_cells_for_coarse_cell(resource.position)
                    .into_iter()
                    .any(|cell| candidate_placement.contains(&cell))
        }) {
            return false;
        }
        if simulation.buildings.values().any(|building| {
            let Some((building_id, existing_definition)) = content
                .buildings
                .iter()
                .find(|(_, definition)| definition.archetype == building.archetype)
            else {
                return false;
            };
            if is_path_building(building_id) {
                return false;
            }
            let existing_placement = building_fine_placement_cells(
                content,
                simulation,
                building,
                building_id,
                existing_definition,
            );
            let existing_navigation = building_fine_navigation_cells(
                content,
                simulation,
                building,
                building_id,
                existing_definition,
            );
            existing_navigation
                .into_iter()
                .any(|cell| candidate_placement.contains(&cell))
                || existing_placement
                    .into_iter()
                    .any(|cell| candidate_navigation.contains(&cell))
        }) {
            return false;
        }
        !simulation.enemy_camps.values().any(|camp| {
            let Some(archetype) = content.archetypes.get(&camp.archetype) else {
                return false;
            };
            let Some(region) = building_region(camp.position, archetype.footprint, world) else {
                return true;
            };
            fine_cells_for_coarse_region(region)
                .into_iter()
                .any(|cell| candidate_placement.contains(&cell))
        })
    }
}

pub(crate) fn path_navigation_cell_is_available(
    navigation: &stream_town_domain::NavGrid,
    simulation: &WorldSimulation,
    gate_cells: &HashSet<GridPos>,
    cell: GridPos,
) -> bool {
    path_navigation_cell_is_traversable(navigation, gate_cells, cell)
        && !simulation
            .path_navigation_positions
            .values()
            .any(|occupied| *occupied == cell)
}

pub(crate) fn path_navigation_cell_is_traversable(
    navigation: &stream_town_domain::NavGrid,
    gate_cells: &HashSet<GridPos>,
    cell: GridPos,
) -> bool {
    navigation.contains(cell) && (navigation.is_walkable(cell) || gate_cells.contains(&cell))
}

pub(crate) fn confirmed_path_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    placement: &BuildingPlacement,
    navigation: &stream_town_domain::NavGrid,
) -> Vec<GridPos> {
    let gate_cells = completed_player_gate_navigation_cells(content, simulation);
    let mut cells = path_centre_cells(placement)
        .into_iter()
        .filter(|cell| {
            path_navigation_cell_is_available(navigation, simulation, &gate_cells, *cell)
        })
        .collect::<Vec<_>>();
    if !placement.thick_path {
        return cells;
    }
    cells.extend(thick_path_side_cells(placement).into_iter().filter(|cell| {
        path_navigation_cell_is_available(navigation, simulation, &gate_cells, *cell)
    }));
    cells
}

pub(crate) fn building_placement_visual_cell_is_available(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    placement: &BuildingPlacement,
    definition: &BuildingDef,
    fine_navigation: Option<&stream_town_domain::NavGrid>,
    cell: GridPos,
) -> bool {
    let route_available = building_placement_is_available(
        content,
        simulation,
        world,
        placement,
        definition,
        fine_navigation,
    );
    if !route_available
        || !is_path_building(&placement.building)
        || !placement.thick_path
        || path_centre_cells(placement).contains(&cell)
    {
        return route_available;
    }
    let Some(navigation) = fine_navigation else {
        return false;
    };
    path_navigation_cell_is_available(
        navigation,
        simulation,
        &completed_player_gate_navigation_cells(content, simulation),
        cell,
    )
}

pub(crate) fn scaled_building_cost(
    cost: &BTreeMap<StableId, u32>,
    multiplier: usize,
) -> BTreeMap<StableId, u32> {
    let multiplier = u32::try_from(multiplier).unwrap_or(u32::MAX);
    cost.iter()
        .map(|(resource, amount)| (resource.clone(), amount.saturating_mul(multiplier)))
        .collect()
}

#[allow(clippy::type_complexity)]
pub(crate) fn sync_building_placers(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    world: Res<WorldRuntime>,
    fine_navigation: Res<FineNavigationRuntime>,
    placers: Res<BuildingPlacers>,
    render: Res<RenderAssets>,
    mut visuals: Query<
        (
            Entity,
            &BuildingPlacementVisual,
            &mut Transform,
            &mut MeshMaterial3d<BoundsMaterial>,
        ),
        Without<BuildingPlacementGhost>,
    >,
    mut ghosts: Query<
        (Entity, &BuildingPlacementGhost, &mut Transform),
        Without<BuildingPlacementVisual>,
    >,
) {
    let mut active_visuals = BTreeSet::new();
    for (entity, visual, mut transform, mut material) in &mut visuals {
        let Some(placement) = placers.0.get(&visual.owner) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let Some(definition) = content.0.buildings.get(&placement.building) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        if !placement_visual_cells(placement).contains(&visual.cell) {
            commands.entity(entity).try_despawn();
            continue;
        }
        let cell_placement = placement_at_cell(placement, visual.cell);
        let available = building_placement_visual_cell_is_available(
            &content.0,
            &simulation.0,
            &world.generated,
            placement,
            definition,
            fine_navigation.grid.as_ref(),
            visual.cell,
        );
        update_placer_visual(
            &config.0,
            &world.generated,
            &render,
            &cell_placement,
            definition,
            available,
            &mut transform,
            &mut material,
        );
        active_visuals.insert((visual.owner.clone(), visual.cell));
    }
    for (owner, placement) in &placers.0 {
        let Some(definition) = content.0.buildings.get(&placement.building) else {
            continue;
        };
        for cell in placement_visual_cells(placement) {
            if active_visuals.contains(&(owner.clone(), cell)) {
                continue;
            }
            let cell_placement = placement_at_cell(placement, cell);
            let mut transform = Transform::default();
            let mut material = MeshMaterial3d(render.placement_valid.clone());
            let available = building_placement_visual_cell_is_available(
                &content.0,
                &simulation.0,
                &world.generated,
                placement,
                definition,
                fine_navigation.grid.as_ref(),
                cell,
            );
            update_placer_visual(
                &config.0,
                &world.generated,
                &render,
                &cell_placement,
                definition,
                available,
                &mut transform,
                &mut material,
            );
            commands.spawn((
                WorldEntity,
                BuildingPlacementVisual {
                    owner: owner.clone(),
                    cell,
                },
                Mesh3d(render.cube.clone()),
                material,
                transform,
            ));
        }
    }

    let mut active_ghosts = BTreeSet::new();
    for (entity, ghost, mut transform) in &mut ghosts {
        let Some(placement) = placers.0.get(&ghost.owner) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let Some(definition) = content.0.buildings.get(&placement.building) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let age = building_age(&content.0, &simulation.0, &placement.building, 1);
        let scene = content
            .0
            .archetypes
            .get(&definition.archetype)
            .and_then(|archetype| archetype_scene_for_age(archetype, age));
        if ghost.building != placement.building
            || scene.is_none_or(|scene| scene.asset_path != ghost.scene_asset_path)
            || !placement_visual_cells(placement).contains(&ghost.cell)
        {
            commands.entity(entity).try_despawn();
            continue;
        }
        let cell_placement = placement_at_cell(placement, ghost.cell);
        update_placer_ghost_transform(
            &config.0,
            &world.generated,
            &cell_placement,
            definition,
            &mut transform,
        );
        active_ghosts.insert((ghost.owner.clone(), ghost.cell));
    }
    let Some(asset_server) = asset_server.as_deref() else {
        return;
    };
    for (owner, placement) in &placers.0 {
        let Some(definition) = content.0.buildings.get(&placement.building) else {
            continue;
        };
        let Some(archetype) = content.0.archetypes.get(&definition.archetype) else {
            continue;
        };
        let age = building_age(&content.0, &simulation.0, &placement.building, 1);
        let Some(scene) = archetype_scene_for_age(archetype, age)
            .filter(|scene| converted_asset_exists(&asset_root.0, &scene.asset_path))
        else {
            continue;
        };
        for cell in placement_visual_cells(placement) {
            if active_ghosts.contains(&(owner.clone(), cell)) {
                continue;
            }
            let cell_placement = placement_at_cell(placement, cell);
            let mut transform = Transform::default();
            update_placer_ghost_transform(
                &config.0,
                &world.generated,
                &cell_placement,
                definition,
                &mut transform,
            );
            commands.spawn((
                WorldEntity,
                BuildingPlacementGhost {
                    owner: owner.clone(),
                    cell,
                    building: placement.building.clone(),
                    scene_asset_path: scene.asset_path.clone(),
                },
                BuildingPlacementGhostHiddenNodes(main_menu_hidden_model_node_names(
                    &content.0,
                    &definition.archetype,
                    scene,
                )),
                WorldAssetRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
                ),
                transform,
            ));
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn apply_building_placement_ghosts(
    mut commands: Commands,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    fine_navigation: Res<FineNavigationRuntime>,
    placers: Res<BuildingPlacers>,
    render: Res<RenderAssets>,
    parents: Query<&ChildOf>,
    roots: Query<(&BuildingPlacementGhost, &BuildingPlacementGhostHiddenNodes)>,
    renderers: Query<
        Entity,
        (
            With<Mesh3d>,
            Without<BuildingPlacementGhostMesh>,
            Without<BuildingPlacementGhost>,
        ),
    >,
    mut ghost_materials: Query<(
        &BuildingPlacementGhostMesh,
        &mut MeshMaterial3d<BoundsMaterial>,
    )>,
    nodes: Query<
        (Entity, &Name),
        (
            Without<BuildingPlacementGhostNodeProcessed>,
            Without<BuildingPlacementGhost>,
        ),
    >,
) {
    for entity in &renderers {
        let mut ancestor = entity;
        let mut owner = None;
        for _ in 0..64 {
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
            if let Ok((ghost, _)) = roots.get(ancestor) {
                owner = Some(ghost.owner.clone());
                break;
            }
        }
        let Some(owner) = owner else {
            continue;
        };
        let material = placement_ghost_material(
            &content.0,
            &simulation.0,
            &world.generated,
            fine_navigation.grid.as_ref(),
            &placers.0,
            &render,
            &owner,
        );
        commands
            .entity(entity)
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .insert((
                BuildingPlacementGhostMesh { owner },
                MeshMaterial3d(material),
            ));
    }
    for (mesh, mut material) in &mut ghost_materials {
        material.0 = placement_ghost_material(
            &content.0,
            &simulation.0,
            &world.generated,
            fine_navigation.grid.as_ref(),
            &placers.0,
            &render,
            &mesh.owner,
        );
    }
    for (entity, name) in &nodes {
        let mut ancestor = entity;
        let mut hidden = None;
        for _ in 0..64 {
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
            if let Ok((_, hidden_nodes)) = roots.get(ancestor) {
                hidden = Some(hidden_nodes.0.contains(name.as_str()));
                break;
            }
        }
        let Some(hidden) = hidden else {
            continue;
        };
        let mut entity = commands.entity(entity);
        entity.insert(BuildingPlacementGhostNodeProcessed);
        if hidden {
            entity.insert(Visibility::Hidden);
        }
    }
}

pub(crate) fn placement_ghost_material(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    fine_navigation: Option<&stream_town_domain::NavGrid>,
    placers: &BTreeMap<StableId, BuildingPlacement>,
    render: &RenderAssets,
    owner: &StableId,
) -> Handle<BoundsMaterial> {
    let available = placers.get(owner).is_some_and(|placement| {
        content
            .buildings
            .get(&placement.building)
            .is_some_and(|definition| {
                building_placement_is_available(
                    content,
                    simulation,
                    world,
                    placement,
                    definition,
                    fine_navigation,
                )
            })
    });
    if available {
        render.placement_valid.clone()
    } else {
        render.placement_invalid.clone()
    }
}

pub(crate) fn update_placer_ghost_transform(
    config: &GameConfig,
    world: &GeneratedWorld,
    placement: &BuildingPlacement,
    definition: &BuildingDef,
    transform: &mut Transform,
) {
    if is_path_building(&placement.building) {
        let position = placement
            .navigation_position
            .unwrap_or_else(|| placement_to_navigation_centre(placement.position));
        transform.translation = navigation_to_world_on_surface(position, config, world);
        transform.rotation = quarter_turn_rotation(placement.rotation_quarter_turns);
        transform.scale =
            Vec3::splat(config.world.cell_size / f32::from(NAVIGATION_SUBDIVISIONS) / 2.0);
        return;
    }
    let effective = rotated_footprint(definition.footprint, placement.rotation_quarter_turns);
    let centre = GridPos {
        x: placement.position.x.saturating_add(effective[0] / 2),
        z: placement.position.z.saturating_add(effective[1] / 2),
    };
    transform.translation = grid_to_world_on_surface(centre, config, world);
    transform.rotation = quarter_turn_rotation(placement.rotation_quarter_turns);
    transform.scale = Vec3::splat(config.world.cell_size / 2.0);
}

pub(crate) fn building_placement_overlay_text(
    simulation: &WorldSimulation,
    definition: &BuildingDef,
    owner: &StableId,
) -> String {
    let owner_name = simulation
        .actors
        .get(owner)
        .and_then(|actor| {
            actor
                .display_name
                .as_deref()
                .or(actor.login_name.as_deref())
        })
        .unwrap_or_else(|| owner.as_str());
    format!("{owner_name} · {} floorplan", definition.display_name)
}

pub(crate) fn building_placement_overlay_world_position(
    placement: &BuildingPlacement,
    definition: &BuildingDef,
    config: &GameConfig,
    world: &GeneratedWorld,
) -> Vec3 {
    let cells = placement_visual_cells(placement);
    let first = cells.first().copied().unwrap_or(placement.position);
    let last = cells.last().copied().unwrap_or(placement.position);
    if is_path_building(&placement.building) {
        return navigation_to_world_on_surface(
            GridPos {
                x: first.x.saturating_add(last.x).saturating_div(2),
                z: first.z.saturating_add(last.z).saturating_div(2),
            },
            config,
            world,
        );
    }
    let footprint = rotated_footprint(definition.footprint, placement.rotation_quarter_turns);
    grid_to_world_on_surface(
        GridPos {
            x: first
                .x
                .saturating_add(last.x)
                .saturating_div(2)
                .saturating_add(footprint[0] / 2),
            z: first
                .z
                .saturating_add(last.z)
                .saturating_div(2)
                .saturating_add(footprint[1] / 2),
        },
        config,
        world,
    )
}

#[allow(clippy::type_complexity)]
pub(crate) fn sync_building_placement_overlays(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    placers: Res<BuildingPlacers>,
    cameras: Query<(&Camera, &GlobalTransform), With<TownCamera>>,
    mut overlays: Query<(
        Entity,
        &BuildingPlacementOwnerOverlay,
        &mut Text,
        &mut Node,
        &mut Visibility,
    )>,
) {
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };
    let mut existing = BTreeSet::new();
    for (entity, overlay, mut text, mut node, mut visibility) in &mut overlays {
        existing.insert(overlay.owner.clone());
        let Some(placement) = placers.0.get(&overlay.owner) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let Some(definition) = content.0.buildings.get(&placement.building) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let world_position = building_placement_overlay_world_position(
            placement,
            definition,
            &config.0,
            &world.generated,
        ) + Vec3::Y * config.0.world.cell_size * 0.35;
        let Some(screen) = overlay_viewport_position(camera, camera_transform, world_position)
        else {
            *visibility = Visibility::Hidden;
            continue;
        };
        **text = building_placement_overlay_text(&simulation.0, definition, &overlay.owner);
        node.left = px(screen.x - 120.0);
        node.top = px(screen.y - 34.0);
        *visibility = Visibility::Visible;
    }
    for (owner, placement) in &placers.0 {
        if existing.contains(owner) {
            continue;
        }
        let Some(definition) = content.0.buildings.get(&placement.building) else {
            continue;
        };
        let world_position = building_placement_overlay_world_position(
            placement,
            definition,
            &config.0,
            &world.generated,
        ) + Vec3::Y * config.0.world.cell_size * 0.35;
        let Some(screen) = overlay_viewport_position(camera, camera_transform, world_position)
        else {
            continue;
        };
        commands.spawn((
            WorldEntity,
            BuildingPlacementOwnerOverlay {
                owner: owner.clone(),
            },
            Text::new(building_placement_overlay_text(
                &simulation.0,
                definition,
                owner,
            )),
            TextFont {
                font_size: FontSize::Px(16.0),
                ..default()
            },
            TextLayout::new(Justify::Center, LineBreak::NoWrap),
            TextColor(Color::srgb(0.98, 0.94, 0.78)),
            TextShadow {
                offset: Vec2::splat(1.5),
                color: Color::linear_rgba(0.0, 0.0, 0.0, 0.98),
            },
            Pickable::IGNORE,
            GlobalZIndex(20),
            Node {
                position_type: PositionType::Absolute,
                left: px(screen.x - 120.0),
                top: px(screen.y - 34.0),
                width: px(240),
                ..default()
            },
        ));
    }
}

pub(crate) fn update_placer_visual(
    config: &GameConfig,
    world: &GeneratedWorld,
    render: &RenderAssets,
    placement: &BuildingPlacement,
    definition: &BuildingDef,
    available: bool,
    transform: &mut Transform,
    material: &mut MeshMaterial3d<BoundsMaterial>,
) {
    if is_path_building(&placement.building) {
        let position = placement
            .navigation_position
            .unwrap_or_else(|| placement_to_navigation_centre(placement.position));
        let side = config.world.cell_size / f32::from(NAVIGATION_SUBDIVISIONS) * 0.9;
        let footprint_size = Vec3::new(side, config.world.cell_size * 0.02, side);
        transform.translation = navigation_to_world_on_surface(position, config, world)
            + Vec3::Y * footprint_size.y * 0.5;
        transform.scale = footprint_size;
        transform.rotation = Quat::IDENTITY;
        material.0 = if available {
            render.placement_valid.clone()
        } else {
            render.placement_invalid.clone()
        };
        return;
    }
    let fine = definition
        .placement_footprint_thirds
        .unwrap_or_else(|| definition.footprint.map(|axis| axis.saturating_mul(3)));
    let legacy = rotated_footprint(definition.footprint, placement.rotation_quarter_turns);
    let effective = rotated_footprint(fine, placement.rotation_quarter_turns);
    let centre = GridPos {
        x: placement.position.x.saturating_add(legacy[0] / 2),
        z: placement.position.z.saturating_add(legacy[1] / 2),
    };
    let fine_cell_size = config.world.cell_size / f32::from(NAVIGATION_SUBDIVISIONS);
    let size = Vec3::new(
        f32::from(effective[0]) * fine_cell_size * 0.9,
        config.world.cell_size * 0.02,
        f32::from(effective[1]) * fine_cell_size * 0.9,
    );
    transform.translation =
        grid_to_world_on_surface(centre, config, world) + Vec3::Y * size.y * 0.5;
    transform.scale = size;
    transform.rotation = Quat::IDENTITY;
    material.0 = if available {
        render.placement_valid.clone()
    } else {
        render.placement_invalid.clone()
    };
}

pub(crate) fn select_grid_cell(
    mouse: Res<ButtonInput<MouseButton>>,
    ui_buttons: Query<&Interaction, With<Button>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<TownCamera>>,
    spatial: Option<SpatialQuery>,
    config: Res<RuntimeConfig>,
    agents: Query<(&Agent, &GridLocation, &GlobalTransform)>,
    mut selected: ResMut<SelectedCell>,
    mut selected_actor: ResMut<SelectedActor>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    if pointer_is_over_button(ui_buttons.iter()) {
        return;
    }
    let Some(spatial) = spatial else {
        return;
    };
    let (Ok(window), Ok((camera, camera_transform))) = (windows.single(), cameras.single()) else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor) else {
        return;
    };
    let terrain_hit = spatial.cast_ray(
        ray.origin,
        ray.direction,
        2_000.0,
        true,
        &SpatialQueryFilter::default(),
    );
    let terrain_distance = terrain_hit.as_ref().map_or(2_000.0, |hit| hit.distance);
    let actor_radius = config.0.world.cell_size * 0.75;
    let actor_height = config.0.world.cell_size * 0.75;
    if let Some((agent, location, _)) = agents
        .iter()
        .filter_map(|(agent, location, transform)| {
            let centre = transform.translation() + Vec3::Y * actor_height;
            ray_sphere_distance(ray.origin, *ray.direction, centre, actor_radius)
                .filter(|distance| *distance <= terrain_distance)
                .map(|distance| (agent, location, distance))
        })
        .min_by(|(_, _, left), (_, _, right)| left.total_cmp(right))
    {
        selected_actor.0 = Some(agent.id.clone());
        selected.0 = Some(location.0);
        return;
    }
    let Some(hit) = terrain_hit else {
        return;
    };
    let world_position = ray.get_point(hit.distance);
    let Some(cell) = world_to_grid(world_position, &config.0) else {
        return;
    };
    selected_actor.0 = None;
    selected.0 = Some(cell);
}

pub(crate) fn centre_screen_raycast_world_position(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    spatial: &SpatialQuery,
) -> Result<Vec3, String> {
    let viewport = camera
        .logical_viewport_rect()
        .ok_or_else(|| "the broadcast camera viewport is not ready".to_owned())?;
    let ray = camera
        .viewport_to_world(camera_transform, viewport.center())
        .map_err(|_| "the broadcast camera centre ray is unavailable".to_owned())?;
    let hit = spatial
        .cast_ray(
            ray.origin,
            ray.direction,
            BUILD_CURSOR_RAY_DISTANCE,
            true,
            &SpatialQueryFilter::default(),
        )
        .ok_or_else(|| "the centre of the broadcast view does not hit the town".to_owned())?;
    Ok(ray.get_point(hit.distance))
}

pub(crate) fn ray_sphere_distance(
    origin: Vec3,
    direction: Vec3,
    centre: Vec3,
    radius: f32,
) -> Option<f32> {
    let to_centre = centre - origin;
    let projected = to_centre.dot(direction);
    if projected < 0.0 {
        return None;
    }
    let closest_squared = to_centre.length_squared() - projected * projected;
    let radius_squared = radius * radius;
    if closest_squared > radius_squared {
        return None;
    }
    Some((projected - (radius_squared - closest_squared).sqrt()).max(0.0))
}

pub(crate) fn remove_selected_building(
    runtime_id: &StableId,
    content: &ContentCatalog,
    world: &mut GeneratedWorld,
    simulation: &mut WorldSimulation,
) -> Result<(), String> {
    if runtime_id.as_str() == "building:townhall" {
        return Err("the Town Hall cannot be removed".to_owned());
    }
    let removed = simulation
        .buildings
        .get(runtime_id)
        .cloned()
        .ok_or_else(|| format!("building instance {runtime_id} does not exist"))?;
    let definition = building_def_for_archetype(content, &removed.archetype)
        .ok_or_else(|| format!("unknown building archetype {}", removed.archetype))?;
    let region = building_navigation_region(
        removed.position,
        definition,
        removed.rotation_quarter_turns,
        world,
    )
    .ok_or_else(|| "removed building lies outside the world".to_owned())?;
    if building_blocks_navigation(definition) {
        world
            .navigation
            .set_blocked(region, false)
            .map_err(|error| error.to_string())?;
    }
    simulation.buildings.remove(runtime_id);
    simulation.path_navigation_positions.remove(runtime_id);
    simulation.building_night_light_colors.remove(runtime_id);
    for actor in simulation.actors.values_mut() {
        if actor.station.as_ref() == Some(runtime_id) {
            actor.station = None;
        }
        if actor.preferred_target.as_ref() == Some(runtime_id) {
            actor.preferred_target = None;
        }
    }
    let reassigned = reassign_roles_without_prerequisite_buildings(content, simulation);
    if !reassigned.is_empty() {
        info!(actors = ?reassigned, "reassigned citizens whose last role prerequisite was removed");
    }
    Ok(())
}

pub(crate) fn path_building_ids_covered_by_regions(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    regions: &[stream_town_domain::DirtyRegion],
) -> Vec<StableId> {
    let Some(path_archetype) = content
        .buildings
        .get(&StableId::new("building:path").expect("static path ID"))
        .map(|definition| &definition.archetype)
    else {
        return Vec::new();
    };
    simulation
        .buildings
        .values()
        .filter(|building| building.archetype == *path_archetype)
        .filter(|building| {
            let position = simulation
                .path_navigation_positions
                .get(&building.id)
                .copied()
                .map_or(building.position, navigation_to_placement);
            regions.iter().any(|region| {
                (region.min.x..=region.max.x).contains(&position.x)
                    && (region.min.z..=region.max.z).contains(&position.z)
            })
        })
        .map(|building| building.id.clone())
        .collect()
}

pub(crate) fn remove_paths_covered_by_regions(
    content: &ContentCatalog,
    world: &mut GeneratedWorld,
    simulation: &mut WorldSimulation,
    regions: &[stream_town_domain::DirtyRegion],
) -> Result<Vec<StableId>, String> {
    let covered = path_building_ids_covered_by_regions(content, simulation, regions);
    for runtime_id in &covered {
        remove_selected_building(runtime_id, content, world, simulation)?;
    }
    Ok(covered)
}

pub(crate) fn passive_resource_rate_milli_per_second(
    definition: &BuildingDef,
    level: u16,
) -> BTreeMap<StableId, u64> {
    let completed_levels = u64::from(level.saturating_sub(1));
    definition.passive_resources.iter().fold(
        BTreeMap::<StableId, u64>::new(),
        |mut rates, income| {
            let rate = u64::from(income.base_milli_per_second).saturating_add(
                u64::from(income.increment_milli_per_level)
                    .saturating_mul(completed_levels)
                    .saturating_mul(u64::from(income.level_event_repetitions)),
            );
            let total = rates.entry(income.resource.clone()).or_default();
            *total = total.saturating_add(rate);
            rates
        },
    )
}

pub(crate) fn sync_selection_outline(
    selected: Res<SelectedCell>,
    selected_actor: Res<SelectedActor>,
    content: Res<RuntimeContent>,
    config: Res<RuntimeConfig>,
    simulation: Res<SimulationRuntime>,
    render: Res<RenderAssets>,
    agents: Query<(&Agent, &GlobalTransform)>,
    terrain_materials: Option<ResMut<Assets<TerrainMaterial>>>,
) {
    let Some(mut terrain_materials) = terrain_materials else {
        // Reduced/headless apps intentionally omit render asset storage.
        return;
    };
    let selection_center_extent = selected.0.map_or(Vec4::ZERO, |cell| {
        if let Some(actor_id) = selected_actor.0.as_ref()
            && let Some((_, actor_transform)) =
                agents.iter().find(|(agent, _)| &agent.id == actor_id)
        {
            let position = actor_transform.translation();
            let half_extent = config.0.world.cell_size * PLAYER_SELECTION_OUTLINE_SCALE_CELLS * 0.5;
            Vec4::new(position.x, position.z, half_extent, half_extent)
        } else {
            let footprint = selected_structural_footprint(cell, &content.0, &simulation.0);
            selection_outline_rect(cell, footprint, false, &config.0)
        }
    });
    let selection_color = if selection_center_extent.z > 0.0 {
        SELECTION_OUTLINE_COLOR
    } else {
        Vec4::ZERO
    };
    let Some(current) = terrain_materials.get(&render.ground) else {
        return;
    };
    if current.extension.parameters.selection_center_extent == selection_center_extent
        && current.extension.parameters.selection_color == selection_color
    {
        return;
    }
    let Some(mut ground) = terrain_materials.get_mut(&render.ground) else {
        return;
    };
    ground.extension.parameters.selection_center_extent = selection_center_extent;
    ground.extension.parameters.selection_color = selection_color;
}

pub(crate) fn selected_building_footprint(
    cell: GridPos,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> Option<(GridPos, [u16; 2])> {
    simulation.buildings.values().find_map(|building| {
        let definition = content
            .buildings
            .values()
            .find(|definition| definition.archetype == building.archetype)?;
        let footprint = rotated_footprint(definition.footprint, building.rotation_quarter_turns);
        let inside = cell.x >= building.position.x
            && cell.z >= building.position.z
            && cell.x < building.position.x.saturating_add(footprint[0])
            && cell.z < building.position.z.saturating_add(footprint[1]);
        inside.then_some((building.position, footprint))
    })
}

#[allow(dead_code)] // Retained for programmatic object selection and future automatic camera focus.
pub(crate) fn selected_building_id_at_cell(
    cell: GridPos,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> Option<StableId> {
    let (origin, _) = selected_building_footprint(cell, content, simulation)?;
    simulation
        .buildings
        .values()
        .find(|building| building.position == origin)
        .map(|building| building.id.clone())
}

pub(crate) fn selected_enemy_camp_at_cell<'a>(
    cell: GridPos,
    content: &'a ContentCatalog,
    simulation: &'a WorldSimulation,
) -> Option<(&'a EnemyCampState, &'a ArchetypeDef)> {
    simulation.enemy_camps.values().find_map(|camp| {
        let archetype = content.archetypes.get(&camp.archetype)?;
        let inside = cell.x >= camp.position.x
            && cell.z >= camp.position.z
            && cell.x < camp.position.x.saturating_add(archetype.footprint[0])
            && cell.z < camp.position.z.saturating_add(archetype.footprint[1]);
        inside.then_some((camp, archetype))
    })
}

pub(crate) fn selected_structural_footprint(
    cell: GridPos,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> Option<(GridPos, [u16; 2])> {
    selected_building_footprint(cell, content, simulation).or_else(|| {
        selected_enemy_camp_at_cell(cell, content, simulation)
            .map(|(camp, archetype)| (camp.position, archetype.footprint))
    })
}

pub(crate) fn selection_outline_rect(
    cell: GridPos,
    building: Option<(GridPos, [u16; 2])>,
    player_selected: bool,
    config: &GameConfig,
) -> Vec4 {
    let (origin, footprint, scale_factor) = building.map_or_else(
        || {
            (
                cell,
                [1, 1],
                if player_selected {
                    PLAYER_SELECTION_OUTLINE_SCALE_CELLS
                } else {
                    0.9
                },
            )
        },
        |(origin, footprint)| (origin, footprint, 1.0),
    );
    let last = GridPos {
        x: origin.x.saturating_add(footprint[0].saturating_sub(1)),
        z: origin.z.saturating_add(footprint[1].saturating_sub(1)),
    };
    let first_world = grid_to_world(origin, config);
    let last_world = grid_to_world(last, config);
    let centre = (first_world + last_world) * 0.5;
    Vec4::new(
        centre.x,
        centre.z,
        f32::from(footprint[0]) * config.world.cell_size * scale_factor * 0.5,
        f32::from(footprint[1]) * config.world.cell_size * scale_factor * 0.5,
    )
}

pub(crate) fn update_selection_panel(
    selected: Res<SelectedCell>,
    selected_actor: Res<SelectedActor>,
    content: Res<RuntimeContent>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    simulation: Res<SimulationRuntime>,
    mut panels: SelectionPanelTextQuery,
    mut tracks: SelectionPanelTrackQuery,
    mut sliders: Query<
        (
            &SelectionPanelSlider,
            &mut Node,
            &mut ImageNode,
            &mut BackgroundColor,
        ),
        With<SelectionPanelSlider>,
    >,
) {
    if !selected.is_changed()
        && !selected_actor.is_changed()
        && !simulation.is_changed()
        && !world.is_changed()
    {
        return;
    }
    let Some(cell) = selected.0 else {
        for (_, mut visibility) in &mut panels {
            *visibility = Visibility::Hidden;
        }
        return;
    };
    let details = selected_actor
        .0
        .as_ref()
        .and_then(|actor| simulation.0.actors.get(actor))
        .map(|actor| selection_panel_actor_details(actor, &content.0))
        .or_else(|| selection_panel_details(cell, &content.0, &world.generated, &simulation.0));
    let Some(details) = details else {
        for (_, mut visibility) in &mut panels {
            *visibility = Visibility::Hidden;
        }
        return;
    };
    for (mut text, mut visibility) in &mut panels {
        text.0.clone_from(&details.description);
        *visibility = Visibility::Visible;
    }
    for (track, mut visibility) in &mut tracks {
        *visibility = if details.progress(track.0).is_some() {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    for (slider, mut node, mut image, mut color) in &mut sliders {
        let Some(progress) = details.progress(slider.0) else {
            continue;
        };
        let low_health = slider.0 == SelectionPanelBar::Health && progress <= 0.35;
        let source_path = if low_health {
            SELECTION_PANEL_TEXTURE_PATHS[2]
        } else {
            SELECTION_PANEL_TEXTURE_PATHS[1]
        };
        node.width = percent((progress * 100.0).clamp(0.0, 100.0));
        if let Some(handle) = render.selection_panel_textures.get(source_path) {
            image.image = handle.clone();
        }
        color.0 = if low_health {
            Color::srgb(0.9, 0.14, 0.1)
        } else {
            Color::srgb(0.2, 0.8, 0.24)
        };
    }
}

pub(crate) fn vote_timer_text(remaining_seconds: f32) -> String {
    let total = Duration::from_secs_f32(remaining_seconds.max(0.0).ceil()).as_secs();
    format!("{:02}:{:02}", total / 60, total % 60)
}

#[cfg(test)]
pub(crate) fn technology_vote_tally(simulation: &WorldSimulation) -> Option<(usize, usize, f32)> {
    let vote = simulation.active_vote.as_ref()?;
    let approvals = if vote.options.len() > 1 || !vote.option_votes.is_empty() {
        technology_vote_option_tally(vote, &vote.technology)
    } else {
        vote.votes.values().filter(|approve| **approve).count()
    };
    let total = vote.votes.len() + vote.option_votes.len();
    let ratio = vote_ratio(approvals, total);
    Some((approvals, total, ratio))
}

pub(crate) fn vote_ratio(votes: usize, total: usize) -> f32 {
    if total == 0 {
        0.0
    } else {
        let votes = u16::try_from(votes).unwrap_or(u16::MAX);
        let total = u16::try_from(total).unwrap_or(u16::MAX);
        f32::from(votes) / f32::from(total)
    }
}

pub(crate) fn technology_vote_options(vote: &stream_town_domain::TechVote) -> Vec<&StableId> {
    if vote.options.is_empty() {
        vec![&vote.technology]
    } else {
        vote.options.iter().collect()
    }
}

pub(crate) fn technology_vote_requirements(
    content: &ContentCatalog,
    vote: &stream_town_domain::TechVote,
    technology: &StableId,
) -> String {
    let Some(technology) = content.technology.nodes.get(technology) else {
        return "Requirements unavailable".to_owned();
    };
    let requirements = technology
        .objectives
        .iter()
        .filter_map(|objective| content.objectives.get(objective))
        .map(|objective| {
            let gold_adjustment = if objective.kind == ObjectiveKind::Collect
                && objective
                    .resource
                    .as_ref()
                    .is_some_and(|resource| resource.as_str() == "resource:gold")
            {
                vote.gold_spent_during_vote
            } else {
                0
            };
            format!(
                "{} ×{}",
                objective_display_label(objective),
                objective.required_amount.saturating_add(gold_adjustment)
            )
        })
        .collect::<Vec<_>>();
    if requirements.is_empty() {
        "No additional requirements".to_owned()
    } else {
        requirements.join("\n")
    }
}

pub(crate) fn technology_vote_option_tally(
    vote: &stream_town_domain::TechVote,
    technology: &StableId,
) -> usize {
    if vote.options.len() <= 1 && vote.option_votes.is_empty() {
        return vote.votes.values().filter(|approve| **approve).count();
    }
    vote.option_votes
        .values()
        .filter(|option| *option == technology)
        .count()
}

pub(crate) fn technology_vote_leader(vote: &stream_town_domain::TechVote) -> StableId {
    technology_vote_options(vote)
        .into_iter()
        .cloned()
        .fold(None::<(StableId, usize)>, |winner, option| {
            let tally = technology_vote_option_tally(vote, &option);
            match winner {
                Some((_, best_tally)) if best_tally >= tally => winner,
                _ => Some((option, tally)),
            }
        })
        .map_or_else(|| vote.technology.clone(), |(winner, _)| winner)
}

pub(crate) fn ruler_vote_option_lines(simulation: &WorldSimulation) -> Vec<String> {
    let Some(vote) = simulation.ruler_vote.as_ref() else {
        return Vec::new();
    };
    let mut options = vote
        .option_order
        .iter()
        .enumerate()
        .map(|(order, option)| {
            let count = vote
                .votes
                .values()
                .filter(|selected| *selected == option)
                .count();
            (option, count, order)
        })
        .collect::<Vec<_>>();
    options.sort_by_key(|(_, count, order)| (std::cmp::Reverse(*count), *order));
    let mut lines = options
        .into_iter()
        .take(3)
        .map(|(option, count, _)| {
            let label = simulation
                .actors
                .get(option)
                .and_then(|actor| actor.display_name.as_deref())
                .unwrap_or(option.as_str());
            format!("{label}  ({count})")
        })
        .collect::<Vec<_>>();
    if lines.is_empty() {
        lines = match vote.kind {
            RulerVoteKind::NewRuler => Vec::new(),
            RulerVoteKind::KeepRuler => {
                vec!["yes  (0)".to_owned(), "no  (0)".to_owned()]
            }
        };
    }
    lines
}

pub(crate) fn community_vote_option_lines(simulation: &WorldSimulation) -> Vec<String> {
    let Some(vote) = simulation.community_vote.as_ref() else {
        return Vec::new();
    };
    let yes = vote.votes.values().filter(|approved| **approved).count();
    let no = vote.votes.len().saturating_sub(yes);
    vec![format!("yes  ({yes})"), format!("no  ({no})")]
}

pub(crate) fn community_vote_event(proposal: &CommunityVoteProposal) -> CommunityEvent {
    match proposal {
        CommunityVoteProposal::Event(event) => *event,
    }
}

pub(crate) fn ruler_vote_option_font_size(line: &str) -> f32 {
    let characters = u16::try_from(line.chars().count()).unwrap_or(u16::MAX);
    (320.0 / f32::from(characters.max(1))).clamp(11.0, 17.0)
}

pub(crate) fn ruler_vote_result_announcement(
    previous_kind: RulerVoteKind,
    ruler_before_vote: Option<&StableId>,
    simulation: &WorldSimulation,
) -> Option<String> {
    let actor_label = |id: &StableId| {
        simulation
            .actors
            .get(id)
            .and_then(|actor| actor.display_name.as_deref())
            .unwrap_or(id.as_str())
            .to_owned()
    };
    match previous_kind {
        RulerVoteKind::NewRuler => simulation.current_ruler.as_ref().map(|ruler| {
            format!(
                "{} has been elected Ruler of Stream Town!",
                actor_label(ruler)
            )
        }),
        RulerVoteKind::KeepRuler if simulation.current_ruler.as_ref() == ruler_before_vote => {
            simulation.current_ruler.as_ref().map(|ruler| {
                format!(
                    "Chat has chosen to keep {} as Ruler of Stream Town!",
                    actor_label(ruler)
                )
            })
        }
        RulerVoteKind::KeepRuler => ruler_before_vote.map(|ruler| {
            format!(
                "{} was not retained as Ruler. Nominations are now open.",
                actor_label(ruler)
            )
        }),
    }
}

pub(crate) fn announce_ruler_vote_result(
    simulation: Res<SimulationRuntime>,
    mut runtime: ResMut<RulerVoteAnnouncementRuntime>,
    connection: Res<TwitchConnection>,
) {
    let active_kind = simulation.0.ruler_vote.as_ref().map(|vote| vote.kind);
    if !runtime.initialized {
        runtime.initialized = true;
        runtime.active_kind = active_kind;
        runtime.ruler_before_vote = active_kind.and(simulation.0.current_ruler.clone());
        return;
    }
    if let Some(previous_kind) = runtime.active_kind
        && active_kind != Some(previous_kind)
        && let Some(message) = ruler_vote_result_announcement(
            previous_kind,
            runtime.ruler_before_vote.as_ref(),
            &simulation.0,
        )
    {
        info!(announcement = %message, "ruler vote result announced");
        if let Some(transport) = &connection.transport {
            let _ = transport.send(TwitchControl::SendBotMessage(message));
        }
    }
    if active_kind != runtime.active_kind {
        runtime.active_kind = active_kind;
        runtime.ruler_before_vote = active_kind.and(simulation.0.current_ruler.clone());
    }
}

pub(crate) fn announce_community_vote_results(
    mut simulation: ResMut<SimulationRuntime>,
    connection: Res<TwitchConnection>,
) {
    while let Some(outcome) = simulation.0.pending_community_vote_outcomes.pop_front() {
        let event = community_vote_event(&outcome.proposal);
        let message = if outcome.approved {
            format!(
                "The vote passed: {} is now the active town event!",
                community_event_name(event)
            )
        } else {
            format!(
                "The vote did not pass. The current town event is unchanged (proposal: {}).",
                community_event_name(event)
            )
        };
        info!(announcement = %message, "community event vote result announced");
        send_command_feedback(&connection, message);
    }
}

pub(crate) fn citizen_death_announcement(actor: &stream_town_domain::ActorState) -> String {
    let label = actor
        .display_name
        .as_deref()
        .or(actor.login_name.as_deref())
        .unwrap_or(actor.id.as_str());
    format!("{label} has died! They will respawn at the Town Hall in 10 minutes.")
}

pub(crate) fn announce_citizen_deaths(
    simulation: Res<SimulationRuntime>,
    connection: Res<TwitchConnection>,
    mut runtime: ResMut<CitizenDeathAnnouncementRuntime>,
) {
    let currently_dead = simulation
        .0
        .actors
        .values()
        .filter(|actor| !actor.alive && actor.role.as_str() != "role:enemy")
        .map(|actor| actor.id.clone())
        .collect::<BTreeSet<_>>();
    if !runtime.initialized {
        runtime.initialized = true;
        runtime.dead = currently_dead;
        return;
    }
    for actor_id in currently_dead.difference(&runtime.dead) {
        let Some(actor) = simulation.0.actors.get(actor_id) else {
            continue;
        };
        let message = citizen_death_announcement(actor);
        info!(actor = %actor.id, announcement = %message, "citizen death announced");
        send_command_feedback(&connection, message);
    }
    runtime.dead = currently_dead;
}

#[cfg(test)]
pub(crate) fn ruler_vote_option_text(simulation: &WorldSimulation) -> String {
    ruler_vote_option_lines(simulation).join("\n")
}

pub(crate) fn update_vote_panels(
    mut commands: Commands,
    simulation: Res<SimulationRuntime>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    asset_server: Option<Res<AssetServer>>,
    mut panels: Query<(&VotePanelKind, &mut Visibility), Without<TechnologyVoteOptionRow>>,
    mut texts: Query<(&VoteTextKind, &mut Text)>,
    mut fills: Query<(&VoteFillKind, &mut Node)>,
    mut technology_icons: TechnologyVoteIconQuery,
    mut technology_rows: TechnologyVoteRowQuery,
    mut technology_depth_badges: TechnologyVoteDepthBadgeQuery,
    ruler_options: Query<(Entity, Option<&Children>), With<RulerOptionsContainer>>,
) {
    if !simulation.is_changed() {
        return;
    }
    let technology_visible = simulation.0.active_vote.is_some();
    let ruler_visible = simulation.0.ruler_vote.is_some() || simulation.0.community_vote.is_some();
    for (kind, mut visibility) in &mut panels {
        *visibility = match kind {
            VotePanelKind::Technology if technology_visible => Visibility::Visible,
            VotePanelKind::Ruler if ruler_visible => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }

    if let Some(vote) = &simulation.0.active_vote {
        let options = technology_vote_options(vote);
        let total = vote.votes.len() + vote.option_votes.len();
        let mut row_tops = [TECHNOLOGY_VOTE_FIRST_ROW_TOP; TECHNOLOGY_VOTE_OPTION_COUNT];
        let mut next_row_top = TECHNOLOGY_VOTE_FIRST_ROW_TOP;
        for (index, technology) in options
            .iter()
            .take(TECHNOLOGY_VOTE_OPTION_COUNT)
            .enumerate()
        {
            row_tops[index] = next_row_top;
            let requirement_lines = content
                .0
                .technology
                .nodes
                .get(technology)
                .map_or(1, |node| node.objectives.len().max(1));
            next_row_top += technology_vote_row_advance(requirement_lines);
        }
        for (row, mut visibility, mut node) in &mut technology_rows {
            *visibility = if options.get(usize::from(row.0)).is_some() {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
            if let Some(top) = row_tops.get(usize::from(row.0)) {
                node.top = px(*top);
            }
        }
        for (badge, mut visibility) in &mut technology_depth_badges {
            *visibility = if options.get(usize::from(badge.0)).is_some()
                && !technology_vote_depth_tag(options.len(), badge.0).is_empty()
            {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
        for (kind, mut text) in &mut texts {
            text.0 = match kind {
                VoteTextKind::TechnologyTitle => "TECHNOLOGY VOTE".to_owned(),
                VoteTextKind::TechnologyTimer => vote_timer_text(vote.remaining_seconds),
                VoteTextKind::TechnologyOptionTitle(index) => options
                    .get(usize::from(*index))
                    .map(|technology| {
                        let label = content.0.technology.nodes.get(*technology).map_or_else(
                            || technology.to_string(),
                            |node| compact_technology_label(&node.display_name).replace('\n', " "),
                        );
                        format!("{}. {label}", index + 1)
                    })
                    .unwrap_or_default(),
                VoteTextKind::TechnologyOptionDepthTag(index) => {
                    technology_vote_depth_tag(options.len(), *index).to_owned()
                }
                VoteTextKind::TechnologyOptionRequirements(index) => options
                    .get(usize::from(*index))
                    .map(|technology| technology_vote_requirements(&content.0, vote, technology))
                    .unwrap_or_default(),
                _ => continue,
            };
        }
        for (kind, mut node) in &mut fills {
            match kind {
                VoteFillKind::TechnologyTimer => {
                    node.width = percent(
                        (vote.remaining_seconds / TECHNOLOGY_VOTE_DURATION_SECONDS * 100.0)
                            .clamp(0.0, 100.0),
                    );
                }
                VoteFillKind::TechnologyOption(index) => {
                    let ratio = options.get(usize::from(*index)).map_or(0.0, |technology| {
                        vote_ratio(technology_vote_option_tally(vote, technology), total)
                    });
                    node.width = percent((ratio * 100.0).clamp(0.0, 100.0));
                }
                VoteFillKind::RulerTimer => {}
            }
        }
        for (marker, mut icon) in &mut technology_icons {
            let Some(technology) = options.get(usize::from(marker.0)) else {
                icon.image = Handle::default();
                continue;
            };
            if let Some(icon_path) = content
                .0
                .technology
                .nodes
                .get(*technology)
                .map(|node| node.icon_path.as_str())
                && let Some(handle) =
                    presentation_texture_handle(&presentation.0, asset_server.as_deref(), icon_path)
            {
                icon.image = handle;
            }
        }
    } else {
        // Explicitly collapse every technology child as well as the parent.
        // This avoids stale absolute-positioned slider fills surviving the
        // frame where a completed ballot is removed from the simulation.
        for (_, mut visibility, _) in &mut technology_rows {
            *visibility = Visibility::Hidden;
        }
        for (_, mut visibility) in &mut technology_depth_badges {
            *visibility = Visibility::Hidden;
        }
        for (kind, mut node) in &mut fills {
            if matches!(
                kind,
                VoteFillKind::TechnologyTimer | VoteFillKind::TechnologyOption(_)
            ) {
                node.width = percent(0.0);
            }
        }
        for (kind, mut text) in &mut texts {
            if matches!(
                kind,
                VoteTextKind::TechnologyTitle
                    | VoteTextKind::TechnologyTimer
                    | VoteTextKind::TechnologyOptionTitle(_)
                    | VoteTextKind::TechnologyOptionDepthTag(_)
                    | VoteTextKind::TechnologyOptionRequirements(_)
            ) {
                text.0.clear();
            }
        }
        for (_, mut icon) in &mut technology_icons {
            icon.image = Handle::default();
        }
    }

    if simulation.0.ruler_vote.is_some() || simulation.0.community_vote.is_some() {
        let (title, description, remaining_seconds, lines) =
            if let Some(vote) = &simulation.0.ruler_vote {
                (
                    "VOTE FOR RULER".to_owned(),
                    match vote.kind {
                        RulerVoteKind::NewRuler => "Who should be Ruler?".to_owned(),
                        RulerVoteKind::KeepRuler => "Keep the current Ruler?".to_owned(),
                    },
                    vote.remaining_seconds,
                    ruler_vote_option_lines(&simulation.0),
                )
            } else {
                let vote = simulation
                    .0
                    .community_vote
                    .as_ref()
                    .expect("governance panel requires an active vote");
                let event = community_vote_event(&vote.proposal);
                (
                    "EVENT VOTE".to_owned(),
                    format!(
                        "Activate {}?  !vote yes / !vote no",
                        community_event_name(event)
                    ),
                    vote.remaining_seconds,
                    community_vote_option_lines(&simulation.0),
                )
            };
        for (kind, mut text) in &mut texts {
            text.0 = match kind {
                VoteTextKind::RulerTitle => title.clone(),
                VoteTextKind::RulerDescription => description.clone(),
                VoteTextKind::RulerTimer => vote_timer_text(remaining_seconds),
                _ => continue,
            };
        }
        if let Ok((container, children)) = ruler_options.single() {
            if let Some(children) = children {
                for child in children {
                    commands.entity(*child).try_despawn();
                }
            }
            commands.entity(container).with_children(|options| {
                if lines.is_empty() {
                    options.spawn((
                        Text::new("Waiting for nominations..."),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::WHITE),
                        Node {
                            width: percent(100.0),
                            margin: UiRect::top(px(12)),
                            ..default()
                        },
                    ));
                }
                for line in lines {
                    let font_size = ruler_vote_option_font_size(&line);
                    options
                        .spawn((
                            authored_ui_image(
                                &render,
                                VOTE_TEXTURE_PATHS[4],
                                vote_texture(&render, VOTE_TEXTURE_PATHS[4]),
                            ),
                            Node {
                                width: percent(100.0),
                                height: px(42),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                overflow: Overflow::clip(),
                                ..default()
                            },
                        ))
                        .with_children(|row| {
                            row.spawn((
                                Text::new(line),
                                TextFont {
                                    font_size: FontSize::Px(font_size),
                                    ..default()
                                },
                                TextLayout::no_wrap().with_justify(Justify::Center),
                                TextColor(Color::BLACK),
                                Pickable::IGNORE,
                                Node {
                                    width: percent(100.0),
                                    overflow: Overflow::clip(),
                                    ..default()
                                },
                            ));
                        });
                }
            });
        }
        for (kind, mut node) in &mut fills {
            if *kind == VoteFillKind::RulerTimer {
                node.width = percent((remaining_seconds / 120.0 * 100.0).clamp(0.0, 100.0));
            }
        }
    }
}

pub(crate) fn content_label(id: Option<&StableId>, prefix: &str, fallback: &str) -> String {
    id.map_or_else(
        || fallback.to_owned(),
        |id| {
            title_case(&format_camel_words(
                &id.as_str().trim_start_matches(prefix).replace('_', " "),
            ))
        },
    )
}

pub(crate) fn objective_progress_ratio(amount: u32, required: u32) -> f32 {
    if required == 0 {
        return 1.0;
    }
    let permille = amount
        .min(required)
        .saturating_mul(1_000)
        .checked_div(required)
        .unwrap_or_default()
        .min(1_000);
    f32::from(u16::try_from(permille).expect("bounded permille fits u16")) / 1_000.0
}

pub(crate) fn objective_display_label(definition: &stream_town_domain::ObjectiveDef) -> String {
    match definition.kind {
        ObjectiveKind::Build => format!(
            "Build {}",
            content_label(definition.building.as_ref(), "building:", "Building")
        ),
        ObjectiveKind::BuildAny => "Build Buildings".to_owned(),
        ObjectiveKind::Collect => format!(
            "Gather {}",
            content_label(definition.resource.as_ref(), "resource:", "Resources")
        ),
        ObjectiveKind::Kill => format!(
            "Kill {}",
            content_label(definition.enemy.as_ref(), "enemy:", "Enemy")
        ),
        ObjectiveKind::KillAny => "Kill Enemies".to_owned(),
        ObjectiveKind::EarnPerHour => format!(
            "Earn {}/Hour",
            content_label(definition.resource.as_ref(), "resource:", "Resources")
        ),
        ObjectiveKind::Sell => format!(
            "Sell {}",
            content_label(definition.resource.as_ref(), "resource:", "Resources")
        ),
        ObjectiveKind::SellAny => "Sell Resources".to_owned(),
        ObjectiveKind::Buy => format!(
            "Buy {}",
            content_label(definition.resource.as_ref(), "resource:", "Resources")
        ),
        ObjectiveKind::BuyAny => "Buy Resources".to_owned(),
    }
}

pub(crate) fn current_event_panel_state(
    simulation: &WorldSimulation,
) -> Option<(String, String, String, f32)> {
    if let Some(event) = &simulation.fish_god {
        return Some((
            "Fish God".to_owned(),
            "Praise the Fish God!  Type !praise".to_owned(),
            format!("{}  /  {}", event.praises_given, event.praises_required),
            objective_progress_ratio(
                u32::from(event.praises_given),
                u32::from(event.praises_required),
            ),
        ));
    }
    simulation
        .active_raid
        .as_ref()
        .map(|raid| {
            let completed_waves = if raid.tracked_enemies.is_empty() {
                raid.current_wave
            } else {
                raid.current_wave.saturating_sub(1)
            };
            (
                "Raid".to_owned(),
                format!("Enemies In Wave: {}", raid.tracked_enemies.len()),
                format!(
                    "Waves Completed: {}/{}",
                    completed_waves.min(raid.total_waves),
                    raid.total_waves
                ),
                objective_progress_ratio(u32::from(completed_waves), u32::from(raid.total_waves)),
            )
        })
        .or_else(|| {
            simulation.active_community_event.map(|event| {
                (
                    community_event_name(event).to_owned(),
                    community_event_description(event).to_owned(),
                    String::new(),
                    1.0,
                )
            })
        })
}

pub(crate) fn update_current_event_panel(
    simulation: Res<SimulationRuntime>,
    mut panels: Query<&mut Visibility, With<CurrentEventPanel>>,
    mut texts: Query<(&CurrentEventText, &mut Text)>,
    mut fills: Query<&mut Node, With<CurrentEventFill>>,
) {
    if !simulation.is_changed() {
        return;
    }
    let Some((title, description, progress_text, progress)) =
        current_event_panel_state(&simulation.0)
    else {
        for mut visibility in &mut panels {
            *visibility = Visibility::Hidden;
        }
        return;
    };
    for mut visibility in &mut panels {
        *visibility = Visibility::Visible;
    }
    for (kind, mut text) in &mut texts {
        text.0 = match kind {
            CurrentEventText::Title => title.clone(),
            CurrentEventText::Description => description.clone(),
            CurrentEventText::Progress => progress_text.clone(),
        };
    }
    for mut node in &mut fills {
        node.width = percent((progress * 100.0).clamp(0.0, 100.0));
    }
}

pub(crate) fn selection_panel_details(
    cell: GridPos,
    content: &ContentCatalog,
    world: &GeneratedWorld,
    simulation: &WorldSimulation,
) -> Option<SelectionPanelDetails> {
    if let Some(actor) = simulation
        .actors
        .values()
        .find(|actor| actor.position == cell)
    {
        return Some(selection_panel_actor_details(actor, content));
    }
    if let Some((building, definition)) = simulation.buildings.values().find_map(|building| {
        let definition = content
            .buildings
            .values()
            .find(|definition| definition.archetype == building.archetype)?;
        let footprint = rotated_footprint(definition.footprint, building.rotation_quarter_turns);
        let inside = cell.x >= building.position.x
            && cell.z >= building.position.z
            && cell.x < building.position.x.saturating_add(footprint[0])
            && cell.z < building.position.z.saturating_add(footprint[1]);
        inside.then_some((building, definition))
    }) {
        let maximum = building_max_health(content, building).max(1);
        let progress = building_health_fraction(building.health, maximum);
        let state = if building.complete && definition.can_level {
            let building_id = content.buildings.iter().find_map(|(id, candidate)| {
                (candidate.archetype == building.archetype).then(|| id.clone())
            });
            let maximum_level = building_id.as_ref().map_or(building.level, |building_id| {
                maximum_building_level(content, simulation, building_id)
            });
            format!("Level {} / {maximum_level}", building.level)
        } else if building.complete {
            "Complete".to_owned()
        } else {
            "Under construction".to_owned()
        };
        return Some(SelectionPanelDetails {
            description: format!(
                "{}\n{state}\nHealth {}/{}  |  Cell {},{}",
                definition.display_name,
                building.health.max(0),
                maximum,
                cell.x,
                cell.z
            ),
            health_progress: Some(progress),
            experience_progress: None,
        });
    }
    if let Some((camp, archetype)) = selected_enemy_camp_at_cell(cell, content, simulation) {
        let maximum = archetype
            .health
            .as_ref()
            .map_or(BUILDING_MAX_HEALTH, |health| {
                i32::try_from(health.max_health).unwrap_or(i32::MAX)
            })
            .max(1);
        return Some(SelectionPanelDetails {
            description: format!(
                "Enemy Camp\nHealth {}/{}  |  Cell {},{}",
                camp.health.max(0),
                maximum,
                cell.x,
                cell.z
            ),
            health_progress: Some(building_health_fraction(camp.health, maximum)),
            experience_progress: None,
        });
    }
    world
        .resources
        .iter()
        .find(|resource| resource.position == cell && resource.amount > 0)
        .map(|resource| {
            let kind = resource.kind.as_str().trim_start_matches("resource:");
            SelectionPanelDetails {
                description: format!(
                    "{} node\n{} remaining  |  Cell {},{}",
                    title_case(kind),
                    resource.amount,
                    cell.x,
                    cell.z
                ),
                health_progress: None,
                experience_progress: None,
            }
        })
}

pub(crate) fn selection_panel_actor_details(
    actor: &ActorState,
    content: &ContentCatalog,
) -> SelectionPanelDetails {
    let maximum = actor.max_health.max(1);
    let progress = building_health_fraction(actor.health, maximum);
    if actor.role.as_str() == "role:enemy" {
        let enemy_name = actor
            .archetype
            .as_ref()
            .and_then(|id| content.archetypes.get(id))
            .and_then(|archetype| archetype.enemy.as_ref())
            .map_or_else(
                || "Enemy".to_owned(),
                |enemy| content_label(Some(&enemy.enemy_type), "enemy:", "Enemy"),
            );
        return SelectionPanelDetails {
            description: format!(
                "{enemy_name}\nHealth {}/{}  |  Cell {},{}",
                actor.health.max(0),
                maximum,
                actor.position.x,
                actor.position.z
            ),
            health_progress: Some(progress),
            experience_progress: None,
        };
    }
    let name = actor.display_name.as_deref().unwrap_or(actor.id.as_str());
    let role = content
        .roles
        .get(&actor.role)
        .map_or(actor.role.as_str(), |role| role.display_name.as_str());
    let role_progress = role_progress(actor);
    let required_experience = stream_town_domain::required_role_experience(
        role_progress.level,
        content.progression.role_experience_curve_level_span,
        content.progression.role_experience_curve_maximum,
    );
    let experience_progress = if role_progress.level >= content.progression.maximum_role_level {
        1.0
    } else {
        objective_progress_ratio(role_progress.experience, required_experience)
    };
    SelectionPanelDetails {
        description: format!(
            "{name}\n{role}  |  Level {}/{}\nHealth {}/{}  |  XP {}/{}  |  Cell {},{}",
            role_progress.level,
            content.progression.maximum_role_level,
            actor.health.max(0),
            maximum,
            role_progress.experience,
            required_experience,
            actor.position.x,
            actor.position.z
        ),
        health_progress: Some(progress),
        experience_progress: Some(experience_progress),
    }
}

pub(crate) fn title_case(value: &str) -> String {
    let mut characters = value.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + characters.as_str()
    })
}

pub(crate) fn pointer_is_over_button<'a>(
    mut interactions: impl Iterator<Item = &'a Interaction>,
) -> bool {
    interactions.any(|interaction| *interaction != Interaction::None)
}

pub(crate) fn compact_technology_label(display_name: &str) -> String {
    for (prefix, suffix) in [("Unlock", "Unlock"), ("Level", "Max Lv"), ("Upgrade", "Up")] {
        let Some(remainder) = display_name.strip_prefix(prefix) else {
            continue;
        };
        let digit_start = remainder
            .find(|character: char| character.is_ascii_digit())
            .unwrap_or(remainder.len());
        let digit_end = remainder[digit_start..]
            .find(|character: char| !character.is_ascii_digit())
            .map_or(remainder.len(), |offset| digit_start + offset);
        let amount = &remainder[digit_start..digit_end];
        let subject = format_camel_words(&format!(
            "{}{}",
            &remainder[..digit_start],
            &remainder[digit_end..]
        ));
        return if amount.is_empty() {
            format!("{subject}\n{suffix}")
        } else {
            format!("{subject}\n{suffix} {amount}")
        };
    }
    format_camel_words(display_name)
}

pub(crate) fn format_camel_words(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 4);
    for (index, character) in value.chars().enumerate() {
        if index > 0 && character.is_ascii_uppercase() {
            output.push(' ');
        }
        output.push(character);
    }
    output
}
