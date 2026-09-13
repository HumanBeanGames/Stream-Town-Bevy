pub(crate) fn building_region(
    position: GridPos,
    footprint: [u16; 2],
    world: &GeneratedWorld,
) -> Option<stream_town_domain::DirtyRegion> {
    let max_x = position.x.checked_add(footprint[0].checked_sub(1)?)?;
    let max_z = position.z.checked_add(footprint[1].checked_sub(1)?)?;
    if max_x >= world.navigation.width() || max_z >= world.navigation.height() {
        return None;
    }
    Some(stream_town_domain::DirtyRegion {
        min: position,
        max: GridPos { x: max_x, z: max_z },
    })
}

pub(crate) fn rotated_footprint(footprint: [u16; 2], rotation_quarter_turns: i32) -> [u16; 2] {
    if rotation_quarter_turns.rem_euclid(2) == 0 {
        footprint
    } else {
        [footprint[1], footprint[0]]
    }
}

pub(crate) fn building_navigation_region(
    position: GridPos,
    definition: &BuildingDef,
    rotation_quarter_turns: i32,
    world: &GeneratedWorld,
) -> Option<stream_town_domain::DirtyRegion> {
    // The coarse grid remains the authoritative placement and exclusion map.
    // Physical movement uses `FineNavigationRuntime`, where the independently
    // authored footprint can be inset without weakening placement validation.
    building_region(
        position,
        rotated_footprint(definition.footprint, rotation_quarter_turns),
        world,
    )
}

pub(crate) fn placement_to_navigation_centre(position: GridPos) -> GridPos {
    GridPos {
        x: position
            .x
            .saturating_mul(NAVIGATION_SUBDIVISIONS)
            .saturating_add(NAVIGATION_SUBDIVISIONS / 2),
        z: position
            .z
            .saturating_mul(NAVIGATION_SUBDIVISIONS)
            .saturating_add(NAVIGATION_SUBDIVISIONS / 2),
    }
}

pub(crate) fn navigation_to_placement(position: GridPos) -> GridPos {
    GridPos {
        x: position.x / NAVIGATION_SUBDIVISIONS,
        z: position.z / NAVIGATION_SUBDIVISIONS,
    }
}

pub(crate) fn navigation_to_world_on_surface(
    position: GridPos,
    config: &GameConfig,
    world: &GeneratedWorld,
) -> Vec3 {
    let half = f32::from(NAVIGATION_SUBDIVISIONS / 2);
    let subdivision = f32::from(NAVIGATION_SUBDIVISIONS);
    let logical_x = (f32::from(position.x) - half) / subdivision;
    let logical_z = (f32::from(position.z) - half) / subdivision;
    let mut result = Vec3::new(
        (logical_x - f32::from(config.world.width.saturating_sub(1)) * 0.5)
            * config.world.cell_size,
        0.0,
        (logical_z - f32::from(config.world.height.saturating_sub(1)) * 0.5)
            * config.world.cell_size,
    );
    result.y = terrain_surface_height_at_world(world, config, result.x, result.z)
        .unwrap_or_else(|| terrain_height(world, navigation_to_placement(position)));
    result
}

pub(crate) fn default_navigation_footprint_thirds(placement: [u16; 2]) -> [u16; 2] {
    placement.map(|axis| {
        axis.saturating_mul(NAVIGATION_SUBDIVISIONS)
            .saturating_sub(2)
            .max(1)
    })
}

pub(crate) fn linear_neighbour_value(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    state: &BuildingState,
    building_id: &StableId,
) -> u8 {
    if is_path_building(building_id) {
        let Some(path_archetype) = content
            .buildings
            .get(building_id)
            .map(|definition| &definition.archetype)
        else {
            return 0;
        };
        let centre = simulation
            .path_navigation_positions
            .get(&state.id)
            .copied()
            .unwrap_or_else(|| placement_to_navigation_centre(state.position));
        return simulation
            .buildings
            .values()
            .filter(|other| other.id != state.id && other.archetype == *path_archetype)
            .fold(0_u8, |value, other| {
                let other = simulation
                    .path_navigation_positions
                    .get(&other.id)
                    .copied()
                    .unwrap_or_else(|| placement_to_navigation_centre(other.position));
                let delta_x = i32::from(other.x) - i32::from(centre.x);
                let delta_z = i32::from(other.z) - i32::from(centre.z);
                let connection = match (delta_x, delta_z) {
                    (1..=3, 0) => 8,
                    (-3..=-1, 0) => 2,
                    (0, 1..=3) => 16,
                    (0, -3..=-1) => 4,
                    _ => 0,
                };
                value | connection
            });
    }
    let connects = |other: &BuildingState| {
        content.buildings.iter().any(|(id, definition)| {
            definition.archetype == other.archetype
                && if building_id.as_str() == "building:path" {
                    id.as_str() == "building:path"
                } else {
                    matches!(id.as_str(), "building:wall" | "building:gate")
                }
        })
    };
    simulation
        .buildings
        .values()
        .filter(|other| other.id != state.id && connects(other))
        .fold(0_u8, |value, other| {
            value
                | match (
                    i32::from(other.position.x) - i32::from(state.position.x),
                    i32::from(other.position.z) - i32::from(state.position.z),
                ) {
                    (1, 0) => 8,
                    (-1, 0) => 2,
                    (0, 1) => 16,
                    (0, -1) => 4,
                    _ => 0,
                }
        })
}

pub(crate) fn linear_navigation_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    state: &BuildingState,
    building_id: &StableId,
) -> Vec<GridPos> {
    let path = is_path_building(building_id);
    let centre = if path {
        simulation
            .path_navigation_positions
            .get(&state.id)
            .copied()
            .unwrap_or_else(|| placement_to_navigation_centre(state.position))
    } else {
        placement_to_navigation_centre(state.position)
    };
    let mut connections = linear_neighbour_value(content, simulation, state, building_id);
    if path && connections == 0 {
        return vec![centre];
    }
    let horizontal = connections & (2 | 8) != 0;
    let vertical = connections & (4 | 16) != 0;
    if horizontal ^ vertical {
        // A line endpoint still spans its full placement cell, avoiding a one-third
        // gap between a wall/gate and the next non-linear obstruction.
        connections |= if horizontal { 2 | 8 } else { 4 | 16 };
    } else if !path && connections == 0 {
        // An isolated segment has no neighbours from which to infer its axis,
        // so retain the direction selected by the placement command.
        connections = if state.rotation_quarter_turns.rem_euclid(2) == 0 {
            4 | 16
        } else {
            2 | 8
        };
    }
    let mut cells = vec![centre];
    if connections & 2 != 0 {
        cells.push(GridPos {
            x: centre.x.saturating_sub(1),
            z: centre.z,
        });
    }
    if connections & 8 != 0 {
        cells.push(GridPos {
            x: centre.x.saturating_add(1),
            z: centre.z,
        });
    }
    if connections & 4 != 0 {
        cells.push(GridPos {
            x: centre.x,
            z: centre.z.saturating_sub(1),
        });
    }
    if connections & 16 != 0 {
        cells.push(GridPos {
            x: centre.x,
            z: centre.z.saturating_add(1),
        });
    }
    cells.sort_unstable();
    cells.dedup();
    cells
}

pub(crate) fn rectangular_navigation_cells(
    position: GridPos,
    placement: [u16; 2],
    navigation: [u16; 2],
    rotation_quarter_turns: i32,
) -> Vec<GridPos> {
    let placement = rotated_footprint(placement, rotation_quarter_turns);
    let navigation = rotated_footprint(navigation, rotation_quarter_turns);
    let full = placement.map(|axis| axis.saturating_mul(NAVIGATION_SUBDIVISIONS));
    let offset = [
        full[0].saturating_sub(navigation[0]) / 2,
        full[1].saturating_sub(navigation[1]) / 2,
    ];
    let origin = GridPos {
        x: position
            .x
            .saturating_mul(NAVIGATION_SUBDIVISIONS)
            .saturating_add(offset[0]),
        z: position
            .z
            .saturating_mul(NAVIGATION_SUBDIVISIONS)
            .saturating_add(offset[1]),
    };
    (0..navigation[1])
        .flat_map(|z| {
            (0..navigation[0]).map(move |x| GridPos {
                x: origin.x.saturating_add(x),
                z: origin.z.saturating_add(z),
            })
        })
        .collect()
}

pub(crate) fn rectangular_building_fine_cells(
    position: GridPos,
    model_footprint: [u16; 2],
    fine_footprint: [u16; 2],
    rotation_quarter_turns: i32,
) -> Vec<GridPos> {
    let model_footprint = rotated_footprint(model_footprint, rotation_quarter_turns);
    let fine_footprint = rotated_footprint(fine_footprint, rotation_quarter_turns);
    if fine_footprint[0] == 0 || fine_footprint[1] == 0 {
        return Vec::new();
    }
    let visual = GridPos {
        x: position.x.saturating_add(model_footprint[0] / 2),
        z: position.z.saturating_add(model_footprint[1] / 2),
    };
    let centre = placement_to_navigation_centre(visual);
    let origin_x = i32::from(centre.x) - i32::from(fine_footprint[0].saturating_sub(1) / 2);
    let origin_z = i32::from(centre.z) - i32::from(fine_footprint[1].saturating_sub(1) / 2);
    (0..fine_footprint[1])
        .flat_map(|z| {
            (0..fine_footprint[0]).filter_map(move |x| {
                Some(GridPos {
                    x: u16::try_from(origin_x + i32::from(x)).ok()?,
                    z: u16::try_from(origin_z + i32::from(z)).ok()?,
                })
            })
        })
        .collect()
}

pub(crate) fn fine_cells_for_coarse_cell(position: GridPos) -> Vec<GridPos> {
    let origin = GridPos {
        x: position.x.saturating_mul(NAVIGATION_SUBDIVISIONS),
        z: position.z.saturating_mul(NAVIGATION_SUBDIVISIONS),
    };
    (0..NAVIGATION_SUBDIVISIONS)
        .flat_map(|z| {
            (0..NAVIGATION_SUBDIVISIONS).map(move |x| GridPos {
                x: origin.x.saturating_add(x),
                z: origin.z.saturating_add(z),
            })
        })
        .collect()
}

pub(crate) fn fine_cells_for_coarse_region(
    region: stream_town_domain::DirtyRegion,
) -> Vec<GridPos> {
    (region.min.z..=region.max.z)
        .flat_map(|z| {
            (region.min.x..=region.max.x)
                .flat_map(move |x| fine_cells_for_coarse_cell(GridPos { x, z }))
        })
        .collect()
}

pub(crate) fn building_fine_placement_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    state: &BuildingState,
    building_id: &StableId,
    definition: &BuildingDef,
) -> Vec<GridPos> {
    if is_path_building(building_id) {
        return simulation
            .path_navigation_positions
            .get(&state.id)
            .copied()
            .into_iter()
            .collect();
    }
    if matches!(building_id.as_str(), "building:wall" | "building:gate") {
        return linear_navigation_cells(content, simulation, state, building_id);
    }
    rectangular_building_fine_cells(
        state.position,
        definition.footprint,
        definition
            .placement_footprint_thirds
            .unwrap_or_else(|| definition.footprint.map(|axis| axis.saturating_mul(3))),
        state.rotation_quarter_turns,
    )
}

pub(crate) fn building_fine_navigation_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    state: &BuildingState,
    building_id: &StableId,
    definition: &BuildingDef,
) -> Vec<GridPos> {
    if is_path_building(building_id) {
        return Vec::new();
    }
    if matches!(building_id.as_str(), "building:wall" | "building:gate") {
        return linear_navigation_cells(content, simulation, state, building_id);
    }
    let navigation = definition
        .navigation_footprint_thirds
        .unwrap_or_else(|| default_navigation_footprint_thirds(definition.footprint));
    rectangular_building_fine_cells(
        state.position,
        definition.footprint,
        navigation,
        state.rotation_quarter_turns,
    )
}

pub(crate) fn fine_navigation_signature(
    world: &GeneratedWorld,
    simulation: &WorldSimulation,
) -> u64 {
    let mut signature = world.navigation.topology_signature() ^ 0x6a09_e667_f3bc_c909;
    for building in simulation.buildings.values().filter(|building| {
        !simulation
            .path_navigation_positions
            .contains_key(&building.id)
    }) {
        signature = signature
            .wrapping_mul(0x0000_0100_0000_01b3)
            .wrapping_add(stable_id_hash(&building.id))
            .wrapping_add(u64::from(building.position.x) << 16)
            .wrapping_add(u64::from(building.position.z))
            .wrapping_add(
                u64::from(u32::from_ne_bytes(
                    building.rotation_quarter_turns.to_ne_bytes(),
                )) << 32,
            );
    }
    signature
}

pub(crate) fn build_fine_navigation(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
) -> Result<stream_town_domain::NavGrid, stream_town_domain::NavigationError> {
    let width = config
        .world
        .width
        .checked_mul(NAVIGATION_SUBDIVISIONS)
        .ok_or(stream_town_domain::NavigationError::BufferSize)?;
    let height = config
        .world
        .height
        .checked_mul(NAVIGATION_SUBDIVISIONS)
        .ok_or(stream_town_domain::NavigationError::BufferSize)?;
    let mut blocked = Vec::with_capacity(usize::from(width) * usize::from(height));
    let mut heights = Vec::with_capacity(blocked.capacity());
    for z in 0..height {
        for x in 0..width {
            let placement = navigation_to_placement(GridPos { x, z });
            let terrain_height = world.navigation.height_at(placement).unwrap_or_default();
            heights.push(terrain_height);
            blocked.push(terrain_height < NAVIGATION_TERRAIN_MIN_HEIGHT_CENTIMETRES);
        }
    }
    let index = |position: GridPos| {
        (position.x < width && position.z < height)
            .then(|| usize::from(position.z) * usize::from(width) + usize::from(position.x))
    };
    let spawn = GridPos {
        x: config.world.width / 2,
        z: config.world.height / 2,
    };
    for z in 0..NAVIGATION_SUBDIVISIONS {
        for x in 0..NAVIGATION_SUBDIVISIONS {
            let position = GridPos {
                x: spawn
                    .x
                    .saturating_mul(NAVIGATION_SUBDIVISIONS)
                    .saturating_add(x),
                z: spawn
                    .z
                    .saturating_mul(NAVIGATION_SUBDIVISIONS)
                    .saturating_add(z),
            };
            if let Some(index) = index(position) {
                blocked[index] = false;
            }
        }
    }
    for resource in world
        .resources
        .iter()
        .filter(|resource| resource.amount > 0 && resource.target_kind.as_str() != "target:fish")
    {
        if let Some(index) = index(placement_to_navigation_centre(resource.position)) {
            blocked[index] = true;
        }
    }
    for state in simulation.buildings.values() {
        let Some((building_id, definition)) = content
            .buildings
            .iter()
            .find(|(_, definition)| definition.archetype == state.archetype)
        else {
            continue;
        };
        if !building_blocks_navigation(definition) {
            continue;
        }
        for position in
            building_fine_navigation_cells(content, simulation, state, building_id, definition)
        {
            if let Some(index) = index(position) {
                blocked[index] = true;
            }
        }
    }
    for camp in simulation.enemy_camps.values() {
        let Some(archetype) = content.archetypes.get(&camp.archetype) else {
            continue;
        };
        let footprint = [archetype.footprint[0].min(3), archetype.footprint[1].min(3)];
        for position in rectangular_navigation_cells(
            camp.position,
            footprint,
            footprint.map(|axis| axis.saturating_mul(NAVIGATION_SUBDIVISIONS)),
            0,
        ) {
            if let Some(index) = index(position) {
                blocked[index] = true;
            }
        }
    }
    stream_town_domain::NavGrid::new(width, height, blocked, heights)
}

pub(crate) fn sync_fine_navigation(
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    mut runtime: ResMut<FineNavigationRuntime>,
) {
    let signature = fine_navigation_signature(&world.generated, &simulation.0);
    if runtime.grid.is_some() && runtime.applied_signature == signature {
        return;
    }
    match build_fine_navigation(&config.0, &content.0, &simulation.0, &world.generated) {
        Ok(grid) => {
            runtime.grid = Some(grid);
            runtime.applied_signature = signature;
        }
        Err(error) => error!(%error, "failed to build the navigation-only third-cell grid"),
    }
}

pub(crate) fn floorplan_diagnostic_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
) -> BTreeSet<GridPos> {
    let mut cells = world
        .resources
        .iter()
        .filter(|resource| resource.amount > 0)
        .flat_map(|resource| fine_cells_for_coarse_cell(resource.position))
        .collect::<BTreeSet<_>>();
    for building in simulation.buildings.values() {
        let Some((building_id, definition)) = content
            .buildings
            .iter()
            .find(|(_, definition)| definition.archetype == building.archetype)
        else {
            continue;
        };
        cells.extend(building_fine_placement_cells(
            content,
            simulation,
            building,
            building_id,
            definition,
        ));
    }
    for camp in simulation.enemy_camps.values() {
        let Some(archetype) = content.archetypes.get(&camp.archetype) else {
            continue;
        };
        if let Some(region) = building_region(camp.position, archetype.footprint, world) {
            cells.extend(fine_cells_for_coarse_region(region));
        }
    }
    cells
}

pub(crate) fn append_world_diagnostic_quad(
    corners: [Vec3; 4],
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
) {
    let base = u32::try_from(positions.len()).expect("diagnostic overlay vertex count fits u32");
    positions.extend(corners.map(|corner| corner.to_array()));
    normals.extend_from_slice(&[[0.0, 1.0, 0.0]; 4]);
    uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
    indices.extend_from_slice(&[base, base + 3, base + 1, base + 1, base + 3, base + 2]);
}

pub(crate) fn diagnostic_navigation_surface_quad(
    cell: GridPos,
    size: f32,
    config: &GameConfig,
    world: &GeneratedWorld,
    water_height: f32,
) -> [Vec3; 4] {
    let centre = navigation_to_world_on_surface(cell, config, world);
    let half = size * 0.5;
    [
        Vec2::new(centre.x - half, centre.z - half),
        Vec2::new(centre.x + half, centre.z - half),
        Vec2::new(centre.x + half, centre.z + half),
        Vec2::new(centre.x - half, centre.z + half),
    ]
    .map(|point| {
        let surface = terrain_surface_height_at_world(world, config, point.x, point.y)
            .unwrap_or(centre.y)
            .max(water_height);
        Vec3::new(point.x, surface + WORLD_DIAGNOSTIC_OVERLAY_LIFT, point.y)
    })
}

pub(crate) fn world_diagnostic_overlay_mesh(
    mode: WorldDiagnosticMode,
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    fine_navigation: &stream_town_domain::NavGrid,
) -> Option<Mesh> {
    let water_height = f32::from(config.world.water_level_centimetres) * 0.01;
    if mode == WorldDiagnosticMode::Pathfinding {
        return pathfinding_diagnostic_overlay_mesh(config, world, fine_navigation, water_height);
    }
    let estimated_cells = world.resources.len() + simulation.buildings.len() * 4;
    let mut positions = Vec::with_capacity(estimated_cells.saturating_mul(4));
    let mut normals = Vec::with_capacity(positions.capacity());
    let mut uvs = Vec::with_capacity(positions.capacity());
    let mut indices = Vec::with_capacity(estimated_cells.saturating_mul(6));
    let fine_tile_size =
        config.world.cell_size / f32::from(NAVIGATION_SUBDIVISIONS) * WORLD_DIAGNOSTIC_TILE_SCALE;
    for cell in floorplan_diagnostic_cells(content, simulation, world) {
        append_world_diagnostic_quad(
            diagnostic_navigation_surface_quad(cell, fine_tile_size, config, world, water_height),
            &mut positions,
            &mut normals,
            &mut uvs,
            &mut indices,
        );
    }
    if positions.is_empty() {
        return None;
    }
    Some(
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices)),
    )
}

pub(crate) fn pathfinding_diagnostic_overlay_mesh(
    config: &GameConfig,
    world: &GeneratedWorld,
    navigation: &stream_town_domain::NavGrid,
    water_height: f32,
) -> Option<Mesh> {
    let width = navigation.width();
    let height = navigation.height();
    let vertex_width = usize::from(width) + 1;
    let vertex_height = usize::from(height) + 1;
    let vertex_count = vertex_width.saturating_mul(vertex_height);
    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = Vec::with_capacity(vertex_count);
    let mut uvs = Vec::with_capacity(vertex_count);
    let subdivision = f32::from(NAVIGATION_SUBDIVISIONS);
    let fine_half_offset = subdivision * 0.5;
    let coarse_half_x = f32::from(config.world.width.saturating_sub(1)) * 0.5;
    let coarse_half_z = f32::from(config.world.height.saturating_sub(1)) * 0.5;
    let terrain_limit_x = coarse_half_x * config.world.cell_size;
    let terrain_limit_z = coarse_half_z * config.world.cell_size;
    for z in 0..=height {
        for x in 0..=width {
            let logical_x = (f32::from(x) - fine_half_offset) / subdivision;
            let logical_z = (f32::from(z) - fine_half_offset) / subdivision;
            let world_x = (logical_x - coarse_half_x) * config.world.cell_size;
            let world_z = (logical_z - coarse_half_z) * config.world.cell_size;
            let surface = terrain_surface_height_at_world(
                world,
                config,
                world_x.clamp(-terrain_limit_x, terrain_limit_x),
                world_z.clamp(-terrain_limit_z, terrain_limit_z),
            )
            .unwrap_or(water_height);
            positions.push([
                world_x,
                surface.max(water_height) + WORLD_DIAGNOSTIC_OVERLAY_LIFT,
                world_z,
            ]);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([
                f32::from(x) / f32::from(width.max(1)),
                f32::from(z) / f32::from(height.max(1)),
            ]);
        }
    }
    let mut indices = Vec::new();
    for z in 0..height {
        for x in 0..width {
            if navigation.is_walkable(GridPos { x, z }) {
                continue;
            }
            let top_left = u32::try_from(usize::from(z) * vertex_width + usize::from(x))
                .expect("diagnostic overlay vertex count fits u32");
            let bottom_left = top_left
                .checked_add(u32::try_from(vertex_width).expect("diagnostic row width fits u32"))
                .expect("diagnostic overlay vertex count fits u32");
            indices.extend_from_slice(&[
                top_left,
                bottom_left,
                top_left + 1,
                top_left + 1,
                bottom_left,
                bottom_left + 1,
            ]);
        }
    }
    if indices.is_empty() {
        return None;
    }
    Some(
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices)),
    )
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn sync_world_diagnostic_view(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    fine_navigation: Res<FineNavigationRuntime>,
    mut runtime: ResMut<WorldDiagnosticRuntime>,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut visual_roots: Query<
        (
            Entity,
            &mut Visibility,
            Option<&WorldDiagnosticVisibilityBackup>,
        ),
        Or<(
            With<RuntimeBuilding>,
            With<ResourceNode>,
            With<EnemyCamp>,
            With<BuildingPlacementGhost>,
            With<BuildingHealthOverlay>,
        )>,
    >,
    overlays: Query<(Entity, &WorldDiagnosticOverlay)>,
) {
    if runtime.mode.is_some() {
        runtime.remaining_seconds = (runtime.remaining_seconds - time.delta_secs()).max(0.0);
        if runtime.remaining_seconds <= f32::EPSILON {
            runtime.mode = None;
        }
    }

    let Some(mode) = runtime.mode else {
        for (entity, mut visibility, backup) in &mut visual_roots {
            let Some(backup) = backup else {
                continue;
            };
            *visibility = backup.0;
            commands
                .entity(entity)
                .remove::<WorldDiagnosticVisibilityBackup>();
        }
        for (entity, _) in &overlays {
            commands.entity(entity).try_despawn();
        }
        return;
    };

    for (entity, mut visibility, backup) in &mut visual_roots {
        if backup.is_none() {
            commands
                .entity(entity)
                .insert(WorldDiagnosticVisibilityBackup(*visibility));
        }
        *visibility = Visibility::Hidden;
    }
    if overlays.iter().any(|(_, overlay)| overlay.mode == mode) {
        return;
    }
    for (entity, _) in &overlays {
        commands.entity(entity).try_despawn();
    }
    let (Some(meshes), Some(materials), Some(fine_navigation)) = (
        meshes.as_deref_mut(),
        materials.as_deref_mut(),
        fine_navigation.grid.as_ref(),
    ) else {
        return;
    };
    let Some(mesh) = world_diagnostic_overlay_mesh(
        mode,
        &config.0,
        &content.0,
        &simulation.0,
        &world.generated,
        fine_navigation,
    ) else {
        return;
    };
    let material = runtime.black_material.clone().unwrap_or_else(|| {
        let material = materials.add(StandardMaterial {
            base_color: Color::BLACK,
            unlit: true,
            depth_bias: WORLD_DIAGNOSTIC_DEPTH_BIAS,
            ..default()
        });
        runtime.black_material = Some(material.clone());
        material
    });
    commands.spawn((
        WorldEntity,
        WorldDiagnosticOverlay { mode },
        Name::new(match mode {
            WorldDiagnosticMode::Pathfinding => "Pathfinding Accessibility Overlay",
            WorldDiagnosticMode::Floorplan => "Placement Footprint Overlay",
        }),
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(material),
        NotShadowCaster,
        NoFrustumCulling,
    ));
}

pub(crate) fn enemy_camp_navigation_region(
    position: GridPos,
    visual: [u16; 2],
    world: &GeneratedWorld,
) -> Option<stream_town_domain::DirtyRegion> {
    let footprint = [visual[0].min(3), visual[1].min(3)];
    let offset = [
        visual[0].saturating_sub(footprint[0]) / 2,
        visual[1].saturating_sub(footprint[1]) / 2,
    ];
    building_region(
        GridPos {
            x: position.x.checked_add(offset[0])?,
            z: position.z.checked_add(offset[1])?,
        },
        footprint,
        world,
    )
}

pub(crate) fn foliage_clearance_regions(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
) -> Vec<stream_town_domain::DirtyRegion> {
    let building_regions = simulation.buildings.values().filter_map(|building| {
        let definition = building_def_for_archetype(content, &building.archetype)?;
        if definition.archetype.as_str() == "archetype:building:path" {
            return None;
        }
        building_region(
            building.position,
            rotated_footprint(definition.footprint, building.rotation_quarter_turns),
            world,
        )
    });
    let camp_regions = simulation.enemy_camps.values().filter_map(|camp| {
        let archetype = content.archetypes.get(&camp.archetype)?;
        building_region(camp.position, archetype.footprint, world)
    });
    building_regions.chain(camp_regions).collect()
}

pub(crate) fn foliage_clearance_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
) -> HashSet<GridPos> {
    foliage_clearance_regions(content, simulation, world)
        .into_iter()
        .flat_map(|region| {
            (region.min.z..=region.max.z)
                .flat_map(move |z| (region.min.x..=region.max.x).map(move |x| GridPos { x, z }))
        })
        .collect()
}

pub(crate) fn foliage_clearance_navigation_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
) -> HashSet<GridPos> {
    let mut cells = HashSet::new();
    for building in simulation.buildings.values() {
        let Some((building_id, definition)) = content
            .buildings
            .iter()
            .find(|(_, definition)| definition.archetype == building.archetype)
        else {
            continue;
        };
        if is_path_building(building_id) {
            continue;
        }
        cells.extend(building_fine_placement_cells(
            content,
            simulation,
            building,
            building_id,
            definition,
        ));
    }
    for camp in simulation.enemy_camps.values() {
        let Some(archetype) = content.archetypes.get(&camp.archetype) else {
            continue;
        };
        if let Some(region) = building_region(camp.position, archetype.footprint, world) {
            cells.extend(fine_cells_for_coarse_region(region));
        }
    }
    cells
}

pub(crate) fn clear_seeded_trees_under_building(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &mut GeneratedWorld,
    building_id: &StableId,
) -> usize {
    let Some(building) = simulation.buildings.get(building_id) else {
        return 0;
    };
    let Some((definition_id, definition)) = content
        .buildings
        .iter()
        .find(|(_, definition)| definition.archetype == building.archetype)
    else {
        return 0;
    };
    let occupied =
        building_fine_navigation_cells(content, simulation, building, definition_id, definition)
            .into_iter()
            .collect::<HashSet<_>>();
    let mut removed_positions = Vec::new();
    world.resources.retain(|resource| {
        let remove = resource.target_kind.as_str() == "target:tree"
            && occupied.contains(&placement_to_navigation_centre(resource.position));
        if remove {
            removed_positions.push(resource.position);
        }
        !remove
    });
    for position in &removed_positions {
        if !resource_cell_has_active_generation_occupant(&world.resources, *position) {
            let _ = world.navigation.set_blocked(
                stream_town_domain::DirtyRegion {
                    min: *position,
                    max: *position,
                },
                false,
            );
        }
    }
    removed_positions.len()
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct FoliageClearanceInputSignature {
    structures: u64,
    paths: u64,
    wear: u64,
}

#[derive(Default)]
pub(crate) struct FoliageClearanceSyncState {
    initialized: bool,
    signature: FoliageClearanceInputSignature,
}

pub(crate) fn foliage_clearance_input_signature(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    traversal_wear: &TraversalWearRuntime,
    paths: Option<&PathSurfaceRuntime>,
    terrain: &stream_town_domain::TerrainAppearanceConfig,
) -> FoliageClearanceInputSignature {
    fn fold_u64(mut hash: u64, value: u64) -> u64 {
        for byte in value.to_le_bytes() {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    let mut structures = fold_u64(
        0xcbf2_9ce4_8422_2325_u64,
        u64::from(world.navigation.width()) | (u64::from(world.navigation.height()) << 16),
    );
    for building in simulation.buildings.values() {
        let Some(definition) = building_def_for_archetype(content, &building.archetype) else {
            continue;
        };
        if definition.archetype.as_str() == "archetype:building:path" {
            continue;
        }
        structures = fold_u64(structures, stable_id_hash(&building.id));
        structures = fold_u64(
            structures,
            u64::from(building.position.x) | (u64::from(building.position.z) << 16),
        );
        structures = fold_u64(
            structures,
            u64::from(u32::from_ne_bytes(
                building.rotation_quarter_turns.to_ne_bytes(),
            )),
        );
        structures = fold_u64(
            structures,
            u64::from(definition.footprint[0]) | (u64::from(definition.footprint[1]) << 16),
        );
        let placement = definition
            .placement_footprint_thirds
            .unwrap_or_else(|| definition.footprint.map(|axis| axis.saturating_mul(3)));
        structures = fold_u64(
            structures,
            u64::from(placement[0]) | (u64::from(placement[1]) << 16),
        );
    }
    for camp in simulation.enemy_camps.values() {
        let Some(archetype) = content.archetypes.get(&camp.archetype) else {
            continue;
        };
        structures = fold_u64(structures, stable_id_hash(&camp.id));
        structures = fold_u64(
            structures,
            u64::from(camp.position.x) | (u64::from(camp.position.z) << 16),
        );
        structures = fold_u64(
            structures,
            u64::from(archetype.footprint[0]) | (u64::from(archetype.footprint[1]) << 16),
        );
    }

    // Hash the set of cells whose wear currently hides foliage. HashMap order is
    // intentionally neutralized so an unchanged set cannot invalidate the scan.
    let wear_threshold = traversal_score_for_rate(
        terrain.traversal_fade_start_per_minute,
        terrain.traversal_half_life_seconds,
    );
    let mut wear_xor = u64::from(wear_threshold.to_bits());
    let mut wear_sum = 0_u64;
    let mut wear_count = 0_u64;
    for (position, cell) in &traversal_wear.cells {
        if cell.score < wear_threshold {
            continue;
        }
        let cell_hash = fold_u64(
            0xcbf2_9ce4_8422_2325_u64,
            u64::from(position.x) | (u64::from(position.z) << 16),
        );
        wear_xor ^= cell_hash;
        wear_sum = wear_sum.wrapping_add(cell_hash.rotate_left(23));
        wear_count += 1;
    }
    let wear = fold_u64(fold_u64(wear_xor, wear_sum), wear_count);
    let paths = paths.map_or(0, |paths| {
        paths.applied_signature ^ u64::from(paths.initialized)
    });

    FoliageClearanceInputSignature {
        structures,
        paths,
        wear,
    }
}

pub(crate) fn region_contains_grid_position(
    region: stream_town_domain::DirtyRegion,
    position: GridPos,
) -> bool {
    (region.min.x..=region.max.x).contains(&position.x)
        && (region.min.z..=region.max.z).contains(&position.z)
}

/// Mirrors Unity's placement-time foliage clearing without destroying the
/// deterministic generated instances. Deriving visibility from current
/// structural occupancy also restores foliage when a building is removed and
/// recomputes the correct result after loading a different save.
pub(crate) fn sync_foliage_clearance(
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    traversal_wear: Res<TraversalWearRuntime>,
    paths: Option<Res<PathSurfaceRuntime>>,
    added_foliage: Query<(), Added<FoliageVisual>>,
    added_grounding: Query<(), (With<FoliageVisual>, Added<PendingSurfaceGrounding>)>,
    mut removed_foliage: RemovedComponents<FoliageVisual>,
    mut removed_grounding: RemovedComponents<PendingSurfaceGrounding>,
    mut stats: ResMut<WorldRenderStats>,
    mut sync: Local<FoliageClearanceSyncState>,
    mut foliage: FoliageClearanceQuery,
) {
    let signature = foliage_clearance_input_signature(
        &content.0,
        &simulation.0,
        &world.generated,
        &traversal_wear,
        paths.as_deref(),
        &config.0.terrain,
    );
    let membership_changed = !added_foliage.is_empty()
        || !added_grounding.is_empty()
        || removed_foliage.read().next().is_some()
        || removed_grounding.read().next().is_some();
    if sync.initialized && sync.signature == signature && !membership_changed {
        return;
    }
    sync.initialized = true;
    sync.signature = signature;

    // A generated town contains many more foliage entities than occupied
    // structural cells. Expanding the small set of building/camp rectangles
    // once avoids testing every foliage entity against every rectangle. The
    // input signature above also avoids repeating this full scan on frames where
    // no structure, path, wear threshold, grounding, or foliage membership changed.
    let structural_cells = foliage_clearance_cells(&content.0, &simulation.0, &world.generated);
    let structural_navigation_cells =
        foliage_clearance_navigation_cells(&content.0, &simulation.0, &world.generated);
    let mut visible_instances = 0;
    for (location, navigation_location, batch, pending_grounding, mut visibility) in &mut foliage {
        if let Some(batch) = batch {
            debug_assert_eq!(batch.0.chunk_x, location.0.x / FOLIAGE_BATCH_CHUNK_CELLS);
            debug_assert_eq!(batch.0.chunk_z, location.0.z / FOLIAGE_BATCH_CHUNK_CELLS);
        }
        let structure_hidden = navigation_location.map_or_else(
            || structural_cells.contains(&location.0),
            |location| structural_navigation_cells.contains(&location.0),
        );
        let path_hidden = navigation_location.is_some_and(|location| {
            paths
                .as_deref()
                .is_some_and(|paths| paths.levels.contains_key(&location.0))
        });
        let wear_score = traversal_wear
            .cells
            .get(&location.0)
            .map(|cell| cell.score)
            .unwrap_or_default();
        let should_be_hidden = foliage_should_be_hidden(
            structure_hidden,
            path_hidden,
            pending_grounding.is_some(),
            wear_score,
            &config.0.terrain,
        );
        if should_be_hidden && !matches!(*visibility, Visibility::Hidden) {
            *visibility = Visibility::Hidden;
        } else if !should_be_hidden && matches!(*visibility, Visibility::Hidden) {
            *visibility = Visibility::Inherited;
        }
        if !should_be_hidden {
            visible_instances += 1;
        }
    }
    stats.foliage_visible_instances = visible_instances;
}

pub(crate) fn foliage_should_be_hidden(
    structure_hidden: bool,
    path_hidden: bool,
    pending_grounding: bool,
    wear_score: f32,
    settings: &stream_town_domain::TerrainAppearanceConfig,
) -> bool {
    structure_hidden
        || path_hidden
        || pending_grounding
        || wear_score
            >= traversal_score_for_rate(
                settings.traversal_fade_start_per_minute,
                settings.traversal_half_life_seconds,
            )
}

pub(crate) fn decay_traversal_wear(
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    mut traversal_wear: ResMut<TraversalWearRuntime>,
) {
    traversal_wear.decay(time.delta_secs(), &config.0.terrain);
}

pub(crate) fn sync_traversal_wear_texture(
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    mut traversal_wear: ResMut<TraversalWearRuntime>,
    images: Option<ResMut<Assets<Image>>>,
    terrain_materials: Option<ResMut<Assets<TerrainMaterial>>>,
) {
    if !traversal_wear.texture_dirty {
        return;
    }
    let (Some(mut images), Some(mut terrain_materials)) = (images, terrain_materials) else {
        return;
    };
    let width = config.0.world.width.max(1);
    let height = config.0.world.height.max(1);
    let Some(mut image) = images.get_mut(&render.traversal_wear) else {
        return;
    };
    if image.width() != u32::from(width) || image.height() != u32::from(height) {
        *image = traversal_wear_image(width, height);
    }
    write_traversal_wear_pixels(
        &mut image,
        width,
        height,
        &traversal_wear.cells,
        &config.0.terrain,
    );
    if let Some(mut material) = terrain_materials.get_mut(&render.ground) {
        material.extension.parameters.traversal_grid = Vec4::new(
            f32::from(width),
            f32::from(height),
            config.0.world.cell_size,
            1.0,
        );
    }
    traversal_wear.texture_dirty = false;
}

pub(crate) fn path_surface_signature(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    width: u16,
    height: u16,
) -> u64 {
    let path_archetype = content
        .buildings
        .get(&StableId::new("building:path").expect("static path ID"))
        .map(|definition| &definition.archetype);
    let mut signature = u64::from(width) << 48 | u64::from(height) << 32;
    for building in simulation.buildings.values().filter(|building| {
        building.complete
            && path_archetype.is_some_and(|archetype| *archetype == building.archetype)
    }) {
        let fine_position = simulation.path_navigation_positions.get(&building.id);
        signature = signature
            .wrapping_mul(0x0000_0100_0000_01b3)
            .wrapping_add(stable_id_hash(&building.id))
            .wrapping_add(u64::from(building.position.x) << 16)
            .wrapping_add(u64::from(building.position.z))
            .wrapping_add(fine_position.map_or(0, |position| {
                (u64::from(position.x) << 24) | (u64::from(position.z) << 8)
            }))
            .wrapping_add(
                u64::from(u32::from_ne_bytes(
                    building.rotation_quarter_turns.to_ne_bytes(),
                )) << 8,
            )
            .wrapping_add(u64::from(building.level) << 40);
    }
    signature
}

pub(crate) fn sync_path_surface_texture(
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    render: Res<RenderAssets>,
    mut runtime: ResMut<PathSurfaceRuntime>,
    images: Option<ResMut<Assets<Image>>>,
    mut terrain_materials: Option<ResMut<Assets<TerrainMaterial>>>,
) {
    let width = config
        .0
        .world
        .width
        .saturating_mul(NAVIGATION_SUBDIVISIONS)
        .max(1);
    let height = config
        .0
        .world
        .height
        .saturating_mul(NAVIGATION_SUBDIVISIONS)
        .max(1);
    let signature = path_surface_signature(&content.0, &simulation.0, width, height);
    if runtime.initialized && runtime.applied_signature == signature {
        return;
    }
    let Some(mut images) = images else {
        return;
    };
    let Some(mut image) = images.get_mut(&render.path_surface) else {
        return;
    };
    if image.width() != u32::from(width) || image.height() != u32::from(height) {
        *image = traversal_wear_image(width, height);
    }
    let path_id = StableId::new("building:path").expect("static path ID");
    let path_archetype = content
        .0
        .buildings
        .get(&path_id)
        .map(|definition| &definition.archetype);
    let Some(data) = image.data.as_mut() else {
        return;
    };
    for pixel in data.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0, 0, 0, u8::MAX]);
    }
    runtime.levels.clear();
    for building in simulation.0.buildings.values().filter(|building| {
        building.complete
            && path_archetype.is_some_and(|archetype| *archetype == building.archetype)
    }) {
        for position in linear_navigation_cells(&content.0, &simulation.0, building, &path_id) {
            if position.x >= width || position.z >= height {
                continue;
            }
            let index =
                (usize::from(position.z) * usize::from(width) + usize::from(position.x)) * 4;
            if let Some(level) = data.get_mut(index) {
                *level = u8::try_from(building.level.min(u16::from(u8::MAX))).unwrap_or(u8::MAX);
            }
            runtime
                .levels
                .entry(position)
                .and_modify(|level| *level = (*level).max(building.level))
                .or_insert(building.level);
        }
    }
    if let Some(materials) = terrain_materials.as_mut()
        && let Some(mut material) = materials.get_mut(&render.ground)
    {
        material.extension.parameters.path_grid = Vec4::new(
            f32::from(width),
            f32::from(height),
            config.0.world.cell_size / f32::from(NAVIGATION_SUBDIVISIONS),
            1.0,
        );
    }
    runtime.applied_signature = signature;
    runtime.initialized = true;
}

pub(crate) fn write_traversal_wear_pixels(
    image: &mut Image,
    width: u16,
    height: u16,
    cells: &HashMap<GridPos, TraversalWearCell>,
    settings: &stream_town_domain::TerrainAppearanceConfig,
) {
    let Some(data) = image.data.as_mut() else {
        return;
    };
    for pixel in data.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0, 0, 0, u8::MAX]);
    }
    for (position, cell) in cells {
        if position.x >= width || position.z >= height {
            continue;
        }
        let index = (usize::from(position.z) * usize::from(width) + usize::from(position.x)) * 4;
        let wear = traversal_wear_byte(cell.score, settings);
        if let Some(red) = data.get_mut(index) {
            *red = wear;
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub(crate) fn traversal_wear_byte(
    score: f32,
    settings: &stream_town_domain::TerrainAppearanceConfig,
) -> u8 {
    // The fraction is clamped to [0, 1], so the rounded value is always in
    // the complete u8 range before this intentional texture conversion.
    (traversal_wear_fraction(score, settings) * f32::from(u8::MAX)).round() as u8
}

pub(crate) fn quarter_turn_rotation(rotation_quarter_turns: i32) -> Quat {
    let normalized = i16::try_from(rotation_quarter_turns.rem_euclid(4))
        .expect("normalized quarter turn fits i16");
    Quat::from_rotation_y(-f32::from(normalized) * std::f32::consts::FRAC_PI_2)
}

pub(crate) fn building_site_is_available(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
) -> bool {
    let Some(region) = building_region(position, footprint, world) else {
        return false;
    };
    (region.min.z..=region.max.z).all(|cell_z| {
        (region.min.x..=region.max.x).all(|cell_x| {
            world.navigation.is_walkable(GridPos {
                x: cell_x,
                z: cell_z,
            })
        })
    }) && !world.resources.iter().any(|resource| {
        resource.amount > 0
            && (region.min.x..=region.max.x).contains(&resource.position.x)
            && (region.min.z..=region.max.z).contains(&resource.position.z)
    })
}

#[cfg(test)]
pub(crate) fn regions_overlap(
    left: stream_town_domain::DirtyRegion,
    right: stream_town_domain::DirtyRegion,
) -> bool {
    left.min.x <= right.max.x
        && left.max.x >= right.min.x
        && left.min.z <= right.max.z
        && left.max.z >= right.min.z
}

#[cfg(test)]
pub(crate) fn building_site_is_available_for_simulation(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
) -> bool {
    if !building_site_is_available(world, position, footprint) {
        return false;
    }
    let Some(candidate) = building_region(position, footprint, world) else {
        return false;
    };
    let overlaps_building = simulation.buildings.values().any(|building| {
        let Some(definition) = building_def_for_archetype(content, &building.archetype) else {
            return false;
        };
        // Paths belong to the fine navigation grid and never reserve their
        // containing coarse floorplan cell. A later building may therefore
        // coexist with a path running through its inset pedestrian margin.
        if definition.archetype.as_str() == "archetype:building:path" {
            return false;
        }
        building_region(
            building.position,
            rotated_footprint(definition.footprint, building.rotation_quarter_turns),
            world,
        )
        .is_some_and(|occupied| regions_overlap(candidate, occupied))
    });
    let overlaps_camp = simulation.enemy_camps.values().any(|camp| {
        content
            .archetypes
            .get(&camp.archetype)
            .and_then(|archetype| building_region(camp.position, archetype.footprint, world))
            .is_some_and(|occupied| regions_overlap(candidate, occupied))
    });
    !overlaps_building && !overlaps_camp
}
