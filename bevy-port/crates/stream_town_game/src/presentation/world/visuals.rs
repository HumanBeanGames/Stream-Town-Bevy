pub(crate) fn actor_material(
    render: &RenderAssets,
    kind: &ActorKind,
    moving: bool,
) -> Handle<StandardMaterial> {
    if matches!(kind, ActorKind::Enemy) {
        if moving {
            render.enemy_moving.clone()
        } else {
            render.enemy_idle.clone()
        }
    } else if moving {
        render.player_moving.clone()
    } else {
        render.player_idle.clone()
    }
}

pub(crate) fn default_archetype_scene(
    archetype: &stream_town_domain::ArchetypeDef,
) -> Option<&ArchetypeScene> {
    archetype
        .scenes
        .iter()
        .find(|scene| scene.is_default)
        .or_else(|| archetype.scenes.first())
}

pub(crate) fn runtime_archetype_scene(archetype: &ArchetypeDef) -> Option<ArchetypeScene> {
    let mut scene = default_archetype_scene(archetype)?.clone();
    if archetype.kind == ArchetypeKind::Player {
        // PlayerChar_TPose.glb contains nine independently exported skin rigs,
        // while the animation FBX contains the one authoritative armature and
        // the same renderer variants. Playing Unity curves on the former makes
        // hands and heads orbit incompatible pivots. Use the animation model's
        // own mesh + skeleton so embedded and manual clips share one rest pose.
        PLAYER_ANIMATED_SOURCE_MODEL.clone_into(&mut scene.source_model);
        PLAYER_ANIMATED_MODEL_PATH.clone_into(&mut scene.asset_path);
    }
    Some(scene)
}

pub(crate) fn resource_visual_archetype<'a>(
    content: &'a ContentCatalog,
    resource_kind: &StableId,
) -> Option<&'a ArchetypeDef> {
    let source_suffix = match resource_kind.as_str() {
        "resource:wood" => "Assets/Prefabs/Dummy Assets/Env_Tree.prefab",
        "resource:ore" => "Assets/Prefabs/Resources/Resource_Ore_Base.prefab",
        "resource:food" => "Assets/Prefabs/Resources/Resource_Bush_Base.prefab",
        _ => return None,
    };
    content
        .archetypes
        .values()
        .find(|archetype| archetype.source_path == source_suffix)
}

pub(crate) fn generated_resource_world_position(
    resource: &stream_town_domain::GeneratedResource,
    config: &GameConfig,
    world: &GeneratedWorld,
) -> Vec3 {
    let mut position = grid_to_world_on_surface(resource.position, config, world);
    let offset = locational_visual_offset(
        world.seed,
        &resource.id,
        resource.position,
        resource.offset_milli_cells,
        config.world.cell_size,
    );
    position.x += offset.x;
    position.z += offset.y;
    // The visual offset can cross a sloped terrain triangle. Sampling before
    // applying it left large trees and bushes hovering at their cell centre's
    // elevation even though their visible X/Z position had moved elsewhere.
    position.y = terrain_surface_height_at_world(world, config, position.x, position.z)
        .unwrap_or(position.y);
    position
}

pub(crate) fn locational_visual_offset(
    _world_seed: u64,
    _id: &StableId,
    _position: GridPos,
    source_offset_milli_cells: [i16; 2],
    cell_size: f32,
) -> Vec2 {
    // The generator stores the authored final X/Z point relative to its grid
    // cell. Re-randomising that point here made habitat selection inspect one
    // location while the renderer used another, especially around shorelines.
    Vec2::new(
        f32::from(source_offset_milli_cells[0]) * cell_size / 1_000.0,
        f32::from(source_offset_milli_cells[1]) * cell_size / 1_000.0,
    )
}

pub(crate) fn resource_mesh_index(
    resource: &stream_town_domain::GeneratedResource,
    config: &GameConfig,
) -> usize {
    if resource.kind.as_str() == "resource:food" {
        // Unity's production generation settings list the same bush mesh twice.
        return 0;
    }
    let mut position = grid_to_world(resource.position, config);
    position.x += f32::from(resource.offset_milli_cells[0]) * config.world.cell_size / 1_000.0;
    position.z += f32::from(resource.offset_milli_cells[1]) * config.world.cell_size / 1_000.0;
    resource_visual_variant(position.x, position.z, &resource.kind, 2)
}

pub(crate) fn resource_visual_scale(cell_size: f32) -> f32 {
    // Primitive-label loads bypass the glTF scene node that carries Blender's
    // centimetre-to-metre 0.01 transform. Restore it here, with the same world
    // scale relationship as the shipping two-unit terrain cells.
    cell_size / 200.0
}

pub(crate) fn centred_resource_visual_position(
    position: Vec3,
    archetype: &ArchetypeDef,
    cell_size: f32,
) -> Vec3 {
    // Generation coordinates are gameplay authority (including Unity's wood
    // half-cell offset). Correct only the prefab's asymmetric mesh pivot so the
    // visible bounds are centred on that authoritative point.
    let source_to_world = cell_size / 2.0;
    position
        + Vec3::new(
            -archetype.bounds.center[0] * source_to_world,
            0.0,
            archetype.bounds.center[2] * source_to_world,
        )
}

pub(crate) fn grounded_resource_visual_position(
    position: Vec3,
    archetype: Option<&ArchetypeDef>,
    config: &GameConfig,
    world: &GeneratedWorld,
) -> Vec3 {
    let mut position = archetype.map_or(position, |archetype| {
        centred_resource_visual_position(position, archetype, config.world.cell_size)
    });
    position.y = terrain_surface_height_at_world(world, config, position.x, position.z)
        .unwrap_or(position.y);
    position
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_resource_visual(
    commands: &mut Commands,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    resource: &stream_town_domain::GeneratedResource,
    position: Vec3,
    config: &GameConfig,
    world: &GeneratedWorld,
) {
    let cell_size = config.world.cell_size;
    if resource.target_kind.as_str() == "target:fish" {
        // Unity's Resource_Fish_Base prefab is an invisible work target in the
        // water. Keep the ECS node for depletion/save state without rendering
        // the bush model used by the shared food resource kind.
        commands.spawn((
            WorldEntity,
            ResourceNode {
                id: resource.id.clone(),
            },
            GridLocation(resource.position),
            Transform::from_translation(position),
            Visibility::Hidden,
        ));
        return;
    }
    let visual_archetype = resource_visual_archetype(content, &resource.kind);
    let visual = visual_archetype
        .and_then(default_archetype_scene)
        .filter(|scene| converted_asset_exists(asset_root, &scene.asset_path));
    let mesh_index = resource_mesh_index(resource, config);
    let material = visual.and_then(|scene| {
        presentation
            .model_materials
            .get(&scene.source_model)
            .and_then(|materials| materials.get("MainMaterial"))
            .and_then(|id| render.presentation_materials.get(id))
    });
    if let (Some(asset_server), Some(scene), Some(material)) = (asset_server, visual, material) {
        let mesh = asset_server.load(
            GltfAssetLabel::Primitive {
                mesh: mesh_index,
                primitive: 0,
            }
            .from_asset(scene.asset_path.clone()),
        );
        let visual_position =
            grounded_resource_visual_position(position, visual_archetype, config, world);
        let mut entity = commands.spawn((
            WorldEntity,
            ResourceNode {
                id: resource.id.clone(),
            },
            ResourceVisual { mesh_index },
            GridLocation(resource.position),
            Mesh3d(mesh),
            RuntimeTerrainGrounding,
            // Every visible generated resource is a land resource. Shoreline
            // fish use the hidden early-return above and the separate fish
            // school presentation, so they are not accidentally culled here.
            SurfaceFoliageHabitat(FoliageHabitat::Land),
            PendingSurfaceGrounding {
                surface_height: visual_position.y,
            },
            Transform::from_translation(visual_position)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
                .with_scale(Vec3::splat(resource_visual_scale(cell_size))),
            Visibility::Hidden,
        ));
        match material {
            ResolvedMaterialHandle::Standard(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
            ResolvedMaterialHandle::Building(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
            ResolvedMaterialHandle::Cloud(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
            ResolvedMaterialHandle::Godray(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
            ResolvedMaterialHandle::Giraffe(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
            ResolvedMaterialHandle::Bounds(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
            ResolvedMaterialHandle::Tree(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
            ResolvedMaterialHandle::Grass(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
            ResolvedMaterialHandle::Critter(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
            ResolvedMaterialHandle::Flag(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
            ResolvedMaterialHandle::Character(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
        }
        return;
    }

    let material = match resource.kind.as_str() {
        "resource:wood" => render.wood.clone(),
        "resource:ore" => render.ore.clone(),
        _ => render.food.clone(),
    };
    let scale = cell_size * 0.55;
    commands.spawn((
        WorldEntity,
        ResourceNode {
            id: resource.id.clone(),
        },
        GridLocation(resource.position),
        Mesh3d(render.cube.clone()),
        MeshMaterial3d(material),
        Transform::from_xyz(position.x, position.y + scale * 0.5, position.z)
            .with_scale(Vec3::splat(scale)),
    ));
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct FoliageBatchKey {
    pub(crate) chunk_x: u16,
    pub(crate) chunk_z: u16,
    pub(crate) layer: StableId,
    pub(crate) variant: u16,
}

pub(crate) fn foliage_batch_key(foliage: &GeneratedFoliage) -> FoliageBatchKey {
    FoliageBatchKey {
        chunk_x: foliage.position.x / FOLIAGE_BATCH_CHUNK_CELLS,
        chunk_z: foliage.position.z / FOLIAGE_BATCH_CHUNK_CELLS,
        layer: foliage.layer.clone(),
        variant: foliage.variant,
    }
}

pub(crate) struct ResolvedFoliageVisual {
    id: StableId,
    source_mesh: Handle<Mesh>,
    material: ResolvedMaterialHandle,
    suppress_self_shadows: bool,
    visibility_end: f32,
    position: GridPos,
    navigation_position: GridPos,
    habitat: FoliageHabitat,
    transform: Transform,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn resolve_foliage_visual(
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: &AssetServer,
    asset_root: &Path,
    world: &GeneratedWorld,
    config: &GameConfig,
    foliage: &GeneratedFoliage,
) -> Option<ResolvedFoliageVisual> {
    // Gameplay water is an opaque depth-writing surface. The converted
    // underwater seaweed/coral meshes are taller than the shallow authored
    // water column and visibly protrude through it as floating grass/flowers.
    // They provide no valid underwater view, so do not create those renderers.
    if foliage.habitat == FoliageHabitat::Underwater {
        return None;
    }
    let layer = content
        .foliage
        .iter()
        .find(|layer| layer.id == foliage.layer)?;
    let variant = layer.variants.get(usize::from(foliage.variant))?;
    if !converted_asset_exists(asset_root, &variant.asset_path) {
        return None;
    }
    let source_mesh = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset(variant.asset_path.clone()),
    );
    let ground_position = resolved_foliage_ground_position(world, config, foliage)?;
    let navigation_position = world_to_navigation(ground_position, config)?;
    // This is also a primitive-label load, so `resource_visual_scale` restores
    // the glTF scene node's centimetre conversion before authored BaseScale.
    let scale = Vec3::from_array(variant.base_scale)
        * resource_visual_scale(config.world.cell_size)
        * (f32::from(foliage.scale_milli) / 1_000.0);
    let visibility_end = foliage_visibility_distance(scale);
    let mapped_material = foliage_layer_material(layer, render).or_else(|| {
        presentation
            .model_materials
            .get(&variant.source_model)
            .and_then(|materials| materials.values().next())
            .and_then(|id| render.presentation_materials.get(id))
    });
    let suppress_self_shadows = mapped_material.is_some_and(material_needs_self_shadow_suppression);
    let material = match mapped_material {
        Some(
            material @ (ResolvedMaterialHandle::Standard(_)
            | ResolvedMaterialHandle::Building(_)
            | ResolvedMaterialHandle::Tree(_)
            | ResolvedMaterialHandle::Grass(_)
            | ResolvedMaterialHandle::Critter(_)
            | ResolvedMaterialHandle::Flag(_)
            | ResolvedMaterialHandle::Character(_)),
        ) => material.clone(),
        Some(
            ResolvedMaterialHandle::Cloud(_)
            | ResolvedMaterialHandle::Godray(_)
            | ResolvedMaterialHandle::Giraffe(_)
            | ResolvedMaterialHandle::Bounds(_),
        )
        | None => ResolvedMaterialHandle::Standard(render.food.clone()),
    };
    Some(ResolvedFoliageVisual {
        id: foliage.id.clone(),
        source_mesh,
        material,
        suppress_self_shadows,
        visibility_end,
        position: foliage.position,
        navigation_position,
        habitat: foliage.habitat,
        transform: Transform::from_translation(ground_position)
            .with_rotation(
                Quat::from_rotation_y(f32::from(foliage.yaw_milliradians) / 1_000.0)
                    * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            )
            .with_scale(scale),
    })
}

pub(crate) fn insert_world_material(
    entity: &mut EntityCommands<'_>,
    material: &ResolvedMaterialHandle,
) {
    match material {
        ResolvedMaterialHandle::Standard(material) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
        ResolvedMaterialHandle::Building(material) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
        ResolvedMaterialHandle::Cloud(material) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
        ResolvedMaterialHandle::Godray(material) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
        ResolvedMaterialHandle::Giraffe(material) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
        ResolvedMaterialHandle::Bounds(material) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
        ResolvedMaterialHandle::Tree(material) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
        ResolvedMaterialHandle::Grass(material) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
        ResolvedMaterialHandle::Critter(material) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
        ResolvedMaterialHandle::Flag(material) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
        ResolvedMaterialHandle::Character(material) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
    }
}

pub(crate) fn spawn_foliage_visual(
    commands: &mut Commands,
    batch: FoliageBatchKey,
    visual: ResolvedFoliageVisual,
) {
    let mut entity = commands.spawn((
        WorldEntity,
        FoliageVisual(visual.id),
        FoliageRenderBatch(batch),
        GridLocation(visual.position),
        FoliageNavigationLocation(visual.navigation_position),
        Mesh3d(visual.source_mesh),
        RuntimeTerrainGrounding,
        SurfaceFoliageHabitat(visual.habitat),
        PendingSurfaceGrounding {
            surface_height: visual.transform.translation.y,
        },
        visual.transform,
        // Do not render a mesh at its source pivot while Bevy is still
        // calculating its bounds. The grounding pass reveals it only after
        // its lowest transformed point has been placed on the terrain.
        Visibility::Hidden,
        bevy::camera::visibility::VisibilityRange {
            start_margin: 0.0..0.0,
            end_margin: (visual.visibility_end - FOLIAGE_VISIBILITY_FADE)..visual.visibility_end,
            use_aabb: true,
        },
    ));
    insert_world_material(&mut entity, &visual.material);
    if visual.suppress_self_shadows {
        entity.insert(bevy::light::NotShadowReceiver);
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn ground_loaded_surface_visuals(
    mut commands: Commands,
    config: Option<Res<RuntimeConfig>>,
    world: Option<Res<WorldRuntime>>,
    meshes: Option<Res<Assets<Mesh>>>,
    mut foliage: Query<(
        Entity,
        &PendingSurfaceGrounding,
        &Aabb,
        Option<&Mesh3d>,
        Option<&RuntimeTerrainGrounding>,
        Option<&SurfaceFoliageHabitat>,
        &mut Transform,
        &mut Visibility,
    )>,
) {
    for (entity, pending, bounds, mesh, runtime_terrain, habitat, mut transform, mut visibility) in
        &mut foliage
    {
        let loaded_mesh = mesh.and_then(|mesh| meshes.as_deref()?.get(&mesh.0));
        let terrain_context = config.as_deref().zip(world.as_deref());
        // Runtime foliage can acquire its AABB one frame before WorldRuntime is
        // installed. Revealing it then used to permanently bypass the shore
        // test. Missing terrain or CPU geometry now means "still loading".
        if runtime_terrain.is_some() && (loaded_mesh.is_none() || terrain_context.is_none()) {
            continue;
        }
        let vertical_extent = loaded_mesh
            .and_then(|mesh| {
                transformed_mesh_vertical_extent(mesh, transform.rotation, transform.scale)
            })
            .unwrap_or_else(|| {
                let minimum = transformed_bounds_minimum_y(
                    Vec3::from(bounds.center),
                    Vec3::from(bounds.half_extents),
                    transform.rotation,
                    transform.scale,
                );
                let maximum = -transformed_bounds_minimum_y(
                    -Vec3::from(bounds.center),
                    Vec3::from(bounds.half_extents),
                    transform.rotation,
                    transform.scale,
                );
                (minimum, maximum)
            });
        let minimum_y = vertical_extent.0;
        if minimum_y.is_finite() {
            transform.translation.y = loaded_mesh
                .zip(terrain_context)
                .and_then(|(mesh, (config, world))| {
                    terrain_conforming_translation_y(
                        mesh,
                        &transform,
                        vertical_extent,
                        &config.0,
                        &world.generated,
                    )
                })
                .unwrap_or(pending.surface_height - minimum_y);
            if let (Some(mesh), Some(habitat), Some((config, world))) =
                (loaded_mesh, habitat, terrain_context)
                && !foliage_mesh_fits_habitat(
                    mesh,
                    &transform,
                    vertical_extent,
                    habitat.0,
                    &config.0,
                    &world.generated,
                )
            {
                commands.entity(entity).try_despawn();
                continue;
            }
            *visibility = Visibility::Inherited;
            commands
                .entity(entity)
                .try_remove::<PendingSurfaceGrounding>();
        }
    }
}

pub(crate) fn transformed_mesh_vertical_extent(
    mesh: &Mesh,
    rotation: Quat,
    scale: Vec3,
) -> Option<(f32, f32)> {
    let VertexAttributeValues::Float32x3(positions) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?
    else {
        return None;
    };
    positions.iter().fold(None, |extent, position| {
        let y = (rotation * (Vec3::from_array(*position) * scale)).y;
        if !y.is_finite() {
            return extent;
        }
        Some(extent.map_or((y, y), |(minimum, maximum): (f32, f32)| {
            (minimum.min(y), maximum.max(y))
        }))
    })
}

pub(crate) fn terrain_conforming_translation_y(
    mesh: &Mesh,
    transform: &Transform,
    (minimum_y, maximum_y): (f32, f32),
    config: &GameConfig,
    world: &GeneratedWorld,
) -> Option<f32> {
    let VertexAttributeValues::Float32x3(positions) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?
    else {
        return None;
    };
    let base_band = ((maximum_y - minimum_y) * 0.02).max(0.001);
    let mut translation_y = f32::INFINITY;
    let mut sampled = false;
    for position in positions {
        let offset = transform.rotation * (Vec3::from_array(*position) * transform.scale);
        if !offset.is_finite() || offset.y > minimum_y + base_band {
            continue;
        }
        let surface = terrain_surface_height_at_world(
            world,
            config,
            transform.translation.x + offset.x,
            transform.translation.z + offset.z,
        )?;
        translation_y = translation_y.min(surface - offset.y);
        sampled = true;
    }
    sampled.then_some({
        // A slight embed removes sub-pixel daylight beneath rigid grass cards
        // on a sloped triangle without visibly burying the authored base.
        translation_y - config.world.cell_size * 0.0025
    })
}

pub(crate) fn foliage_mesh_fits_habitat(
    mesh: &Mesh,
    transform: &Transform,
    (_minimum_y, maximum_y): (f32, f32),
    habitat: FoliageHabitat,
    config: &GameConfig,
    world: &GeneratedWorld,
) -> bool {
    let visible_water_height = visible_water_surface_height(&config.world);
    if habitat == FoliageHabitat::Underwater {
        return transform.translation.y + maximum_y < visible_water_height;
    }
    let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    else {
        return true;
    };
    // A land visual is rejected when any part of its rendered mesh projects
    // over water. Checking only the bottom band was insufficient for flowers,
    // grass cards, and leaning trees: their stems could be rooted on the final
    // strip of land while most of the visible mesh hung over the water.
    let mesh_is_dry = positions.iter().all(|position| {
        let offset = transform.rotation * (Vec3::from_array(*position) * transform.scale);
        terrain_surface_height_at_world(
            world,
            config,
            transform.translation.x + offset.x,
            transform.translation.z + offset.z,
        )
        .is_some_and(|height| height > visible_water_height)
    });
    if !mesh_is_dry {
        return false;
    }

    // Keep a full cell of dry terrain around the pivot. The generated water
    // boundary cuts across averaged terrain triangles, while narrow imported
    // grass/flower cards do not necessarily have a vertex at the point where
    // their pixels cross that boundary. A small deterministic shoreline buffer
    // closes that sampling gap and also keeps large resource canopies dry.
    let clearance = config.world.cell_size;
    [-clearance, 0.0, clearance].into_iter().all(|offset_x| {
        [-clearance, 0.0, clearance].into_iter().all(|offset_z| {
            terrain_surface_height_at_world(
                world,
                config,
                transform.translation.x + offset_x,
                transform.translation.z + offset_z,
            )
            .is_some_and(|height| height > visible_water_height)
        })
    })
}

pub(crate) fn transformed_bounds_minimum_y(
    center: Vec3,
    half_extents: Vec3,
    rotation: Quat,
    scale: Vec3,
) -> f32 {
    let mut minimum_y = f32::INFINITY;
    for x in [-1.0, 1.0] {
        for y in [-1.0, 1.0] {
            for z in [-1.0, 1.0] {
                let corner = center + half_extents * Vec3::new(x, y, z);
                minimum_y = minimum_y.min((rotation * (corner * scale)).y);
            }
        }
    }
    minimum_y
}

pub(crate) fn foliage_visibility_distance(scale: Vec3) -> f32 {
    (FOLIAGE_VISIBILITY_MIN_RANGE + scale.max_element().max(0.0) * 20.0)
        .clamp(FOLIAGE_VISIBILITY_MIN_RANGE, FOLIAGE_VISIBILITY_MAX_RANGE)
}

pub(crate) fn archetype_scene_for_age(
    archetype: &ArchetypeDef,
    age: u8,
) -> Option<&ArchetypeScene> {
    archetype
        .scenes
        .iter()
        .find(|scene| scene.age == Some(age))
        .or_else(|| {
            archetype
                .scenes
                .iter()
                .filter(|scene| scene.age.is_none())
                .min_by_key(|scene| !scene.is_default)
        })
        .or_else(|| {
            archetype
                .scenes
                .iter()
                .filter(|scene| scene.age.is_some_and(|scene_age| scene_age <= age))
                .max_by_key(|scene| scene.age)
        })
}

pub(crate) fn building_scene_for_state<'a>(
    archetype: &'a ArchetypeDef,
    definition: &BuildingDef,
    building_id: &StableId,
    state: &BuildingState,
    age: u8,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> Option<&'a ArchetypeScene> {
    if building_id.as_str() != "building:wall" {
        return archetype_scene_for_age(archetype, age);
    }
    let model_index = wall_tiling(tiled_neighbor_value(content, simulation, state)).0;
    let model = definition
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
        })?;
    let model_stem = model.full_model.strip_suffix("_Base")?;
    archetype
        .scenes
        .iter()
        .find(|scene| {
            scene.age == Some(age)
                && Path::new(&scene.source_model)
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .is_some_and(|stem| stem.eq_ignore_ascii_case(model_stem))
        })
        .or_else(|| archetype_scene_for_age(archetype, age))
}

pub(crate) fn archetype_by_source<'a>(
    content: &'a ContentCatalog,
    kind: ArchetypeKind,
    source_suffix: &str,
) -> Option<&'a ArchetypeDef> {
    content
        .archetypes
        .values()
        .find(|archetype| archetype.kind == kind && archetype.source_path.ends_with(source_suffix))
}

pub(crate) fn archetype_id_by_source(
    content: &ContentCatalog,
    kind: ArchetypeKind,
    source_suffix: &str,
) -> Option<StableId> {
    content.archetypes.iter().find_map(|(id, archetype)| {
        (archetype.kind == kind && archetype.source_path.ends_with(source_suffix))
            .then(|| id.clone())
    })
}

pub(crate) fn native_animation_request(
    archetype: &ArchetypeDef,
    scene: &ArchetypeScene,
    presentation: &PresentationCatalog,
) -> Option<NativeAnimationRequest> {
    let binding = presentation.prefab_bindings.get(&archetype.source_guid)?;
    let animation_index = binding.gltf_animation_index?;
    if binding.animated_scene.as_deref() != Some(scene.asset_path.as_str()) {
        return None;
    }
    Some(NativeAnimationRequest {
        asset_path: scene.asset_path.clone(),
        animation_index,
    })
}

pub(crate) fn converted_animation_spec(
    archetype: &ArchetypeDef,
    presentation: &PresentationCatalog,
) -> Option<ConvertedAnimationSpec> {
    let binding = presentation.prefab_bindings.get(&archetype.source_guid)?;
    let controller = presentation.controllers.get(&binding.controller)?;
    let rig_scene = binding.rig_scene.clone()?;
    let (state, _) = controller
        .states
        .iter()
        .find(|(_, state)| state.display_name.eq_ignore_ascii_case("locomotion"))?;
    let motions: Vec<_> = controller
        .states
        .values()
        .flat_map(|state| &state.motions)
        .collect();
    let converted_motions = motions
        .iter()
        .filter(|motion| {
            presentation.clips.get(&motion.clip).is_some_and(|clip| {
                !clip.transform_tracks.is_empty()
                    || clip.converted_asset_path.is_some() && clip.gltf_animation_index.is_some()
            })
        })
        .count();
    (converted_motions == motions.len() && converted_motions > 0).then(|| ConvertedAnimationSpec {
        controller: binding.controller.clone(),
        state: state.clone(),
        rig_scene,
    })
}

pub(crate) fn complete_converted_animation_spec(
    archetype: &ArchetypeDef,
    scene: &ArchetypeScene,
    presentation: &PresentationCatalog,
) -> Option<ConvertedAnimationSpec> {
    let binding = presentation.prefab_bindings.get(&archetype.source_guid)?;
    let controller = presentation.controllers.get(&binding.controller)?;
    let state = controller.default_states.first()?.clone();
    let motions = controller
        .states
        .values()
        .flat_map(|state| &state.motions)
        .collect::<Vec<_>>();
    (!motions.is_empty()
        && motions.iter().all(|motion| {
            presentation.clips.get(&motion.clip).is_some_and(|clip| {
                !clip.transform_tracks.is_empty()
                    || clip.converted_asset_path.as_deref() == Some(scene.asset_path.as_str())
                        && clip.gltf_animation_index.is_some()
            })
        }))
    .then(|| ConvertedAnimationSpec {
        controller: binding.controller.clone(),
        state,
        rig_scene: scene.asset_path.clone(),
    })
}

pub(crate) fn pet_animation_spec(
    pet: &StableId,
    scene: &ArchetypeScene,
    presentation: &PresentationCatalog,
) -> Option<ConvertedAnimationSpec> {
    let controller = match pet.as_str() {
        "pet:giraffe" => "controller:7a24ad00bb657d1439fa86e0c2eb6b12",
        "pet:red_panda" => "controller:7b5536b35afda8d41930f35e0e51215a",
        "pet:duck" => "controller:bda358ac13f989345a8f46fd230f9826",
        "pet:butterfly" => "controller:a1529f2d1d90b8b4198e099687f1bbea",
        // Fish God and unknown IDs have no Animator/controller mapping.
        _ => return None,
    };
    let controller = StableId::new(controller).expect("authored pet controller ID is valid");
    let definition = presentation.controllers.get(&controller)?;
    let state = definition
        .states
        .iter()
        .find(|(_, state)| state.display_name.eq_ignore_ascii_case("special"))
        .map(|(id, _)| id.clone())?;
    definition
        .states
        .values()
        .flat_map(|state| &state.motions)
        .all(|motion| {
            presentation
                .clips
                .get(&motion.clip)
                .is_some_and(|clip| !clip.transform_tracks.is_empty())
        })
        .then(|| ConvertedAnimationSpec {
            controller,
            state,
            rig_scene: scene.asset_path.clone(),
        })
}

pub(crate) fn prefab_material_spec(
    archetype: &ArchetypeDef,
    scene: &ArchetypeScene,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
) -> Option<MaterialOverrideSpec> {
    let fallback = presentation
        .prefab_materials
        .get(&archetype.source_guid)
        .into_iter()
        .flatten()
        .find_map(|id| render.presentation_materials.get(id))
        .cloned();
    let model_materials: BTreeMap<_, _> = presentation
        .model_materials
        .get(&scene.source_model)
        .into_iter()
        .flat_map(|materials| materials.iter())
        .filter_map(|(name, id)| {
            render
                .presentation_materials
                .get(id)
                .cloned()
                .map(|material| (name.clone(), material))
        })
        .collect();
    let renderer_materials: Vec<_> = presentation
        .prefab_renderer_materials
        .get(&archetype.source_guid)
        .into_iter()
        .flatten()
        .filter_map(|binding| {
            let materials: BTreeMap<_, _> = binding
                .materials
                .iter()
                .filter_map(|(name, id)| {
                    render
                        .presentation_materials
                        .get(id)
                        .cloned()
                        .map(|material| (name.clone(), material))
                })
                .collect();
            (!materials.is_empty()).then(|| ResolvedRendererMaterialBinding {
                target_path: binding.target_path.clone(),
                materials,
            })
        })
        .collect();
    (fallback.is_some() || !model_materials.is_empty() || !renderer_materials.is_empty()).then_some(
        MaterialOverrideSpec {
            fallback,
            model_materials,
            renderer_materials,
            suppress_self_shadows: false,
        },
    )
}

pub(crate) fn archetype_needs_self_shadow_suppression(archetype: &ArchetypeDef) -> bool {
    archetype.source_path == "Assets/Prefabs/Buildings/Building_Gate.prefab"
}

pub(crate) fn building_prefab_material_spec(
    archetype: &ArchetypeDef,
    scene: &ArchetypeScene,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
) -> Option<MaterialOverrideSpec> {
    let mut spec = render
        .regeneration_buildings
        .get(&archetype.source_guid)
        .map(|material| MaterialOverrideSpec {
            fallback: Some(ResolvedMaterialHandle::Standard(material.clone())),
            model_materials: BTreeMap::new(),
            renderer_materials: Vec::new(),
            suppress_self_shadows: false,
        })
        .or_else(|| prefab_material_spec(archetype, scene, presentation, render))?;
    spec.suppress_self_shadows |= archetype_needs_self_shadow_suppression(archetype);
    Some(spec)
}

pub(crate) fn converted_asset_exists(asset_root: &Path, asset_path: &str) -> bool {
    asset_root.join(asset_path).is_file()
}
