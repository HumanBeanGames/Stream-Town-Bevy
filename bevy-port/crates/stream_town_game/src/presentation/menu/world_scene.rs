pub(crate) fn animate_loading_icon(
    time: Res<Time>,
    settings: Option<Res<RuntimePlayerSettings>>,
    mut spinners: Query<(&LoadingIconSpinner, &mut UiTransform)>,
) {
    let reduced_motion = settings.is_some_and(|settings| settings.0.interface.reduced_motion);
    for (spinner, mut transform) in &mut spinners {
        if reduced_motion {
            transform.rotation = Rot2::IDENTITY;
            continue;
        }
        apply_loading_icon_rotation(
            &mut transform,
            spinner.radians_per_second,
            time.delta_secs(),
        );
    }
}

pub(crate) fn apply_loading_icon_rotation(
    transform: &mut UiTransform,
    radians_per_second: f32,
    delta_seconds: f32,
) {
    // Unity applies positive local-Z rotation counterclockwise. Bevy's UI
    // transform documents positive rotation as clockwise, hence the sign.
    transform.rotation =
        Rot2::radians(transform.rotation.as_radians() - radians_per_second * delta_seconds);
}

pub(crate) fn menu_baked_position_visible(
    reference: &MainMenuSceneReference,
    position: Vec3,
) -> bool {
    let camera = Transform::from_translation(Vec3::from_array(reference.camera.position))
        .with_rotation(authored_scene_rotation(reference.camera.rotation));
    let local = camera.rotation.inverse() * (position - camera.translation);
    let depth = -local.z;
    if depth < reference.camera.near || depth > reference.camera.far {
        return false;
    }
    let vertical = depth * (reference.camera.field_of_view_degrees.to_radians() * 0.5).tan();
    // The shipping menu is authored at 16:9. The generous margin retains tall
    // trees and anything just outside the static camera's ground intersection.
    let horizontal = vertical * (16.0 / 9.0);
    local.x.abs() <= horizontal * 1.25 + 12.0 && local.y.abs() <= vertical * 1.45 + 18.0
}

pub(crate) fn insert_main_menu_material(
    entity: &mut EntityCommands<'_>,
    material: &ResolvedMaterialHandle,
    render: &RenderAssets,
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
            let _ = material;
            entity.insert(MeshMaterial3d(render.menu_tree.clone()));
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
    if material_needs_self_shadow_suppression(material) {
        entity.insert(bevy::light::NotShadowReceiver);
    }
}

pub(crate) fn material_needs_self_shadow_suppression(material: &ResolvedMaterialHandle) -> bool {
    // Tree geometry has an exact matching shadow deformation and must receive
    // light/shadows. Keep suppression only for the older animated materials
    // whose shadow pass is not yet sourced from their visible deformation.
    matches!(
        material,
        ResolvedMaterialHandle::Grass(_) | ResolvedMaterialHandle::Critter(_)
    )
}

pub(crate) fn foliage_layer_material<'a>(
    layer: &stream_town_domain::FoliageLayerDef,
    render: &'a RenderAssets,
) -> Option<&'a ResolvedMaterialHandle> {
    render
        .presentation_materials_by_source_path
        .get(&layer.material_source_path)
}

pub(crate) fn main_menu_baked_decoration_indices(
    reference: &MainMenuSceneReference,
) -> (Vec<usize>, Vec<usize>) {
    let bake = reference
        .corrective_bake
        .as_ref()
        .expect("validated main-menu corrective bake");
    let resource_indices = bake
        .resources
        .iter()
        .enumerate()
        .filter(|(_, resource)| {
            menu_baked_position_visible(reference, Vec3::from_array(resource.position))
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let foliage_indices = bake
        .foliage
        .iter()
        .enumerate()
        .filter(|(_, foliage)| {
            menu_baked_position_visible(reference, Vec3::from_array(foliage.position))
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    (resource_indices, foliage_indices)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_main_menu_model_instance(
    commands: &mut Commands,
    reference: &MainMenuSceneReference,
    index: usize,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: &AssetServer,
    asset_root: &Path,
) {
    let instance = &reference.instances[index];
    let Some((archetype_id, archetype, scene)) =
        menu_scene_for_source_model(content, &instance.source_path)
            .filter(|(_, _, scene)| converted_asset_exists(asset_root, &scene.asset_path))
    else {
        warn!(
            source_model = %instance.source_path,
            hierarchy = %instance.hierarchy_path,
            "authored main-menu model has no converted scene"
        );
        return;
    };
    if spawn_completed_main_menu_farm(
        commands,
        instance,
        scene,
        presentation,
        render,
        asset_server,
    ) {
        return;
    }
    let mut entity = commands.spawn((
        StateEntity,
        Name::new(format!(
            "Authored menu instance: {}",
            instance.hierarchy_path
        )),
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
        ),
        Transform::from_translation(Vec3::from_array(instance.position))
            .with_rotation(authored_scene_rotation(instance.rotation))
            .with_scale(Vec3::from_array(instance.scale)),
    ));
    if archetype.kind == ArchetypeKind::Building {
        entity.insert(MainMenuBuildingShadowRoot);
    }
    if let Some(material) = prefab_material_spec(archetype, scene, presentation, render) {
        entity.insert(material);
    }
    if !archetype.rotating_nodes.is_empty() {
        entity.insert(MainMenuRotatingDefinitions(
            archetype.rotating_nodes.clone(),
        ));
    }
    let hidden_nodes = main_menu_hidden_model_node_names(content, archetype_id, scene);
    if !hidden_nodes.is_empty() {
        entity.insert(MainMenuHiddenModelNodes(hidden_nodes));
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_main_menu_baked_resource(
    commands: &mut Commands,
    reference: &MainMenuSceneReference,
    index: usize,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: &AssetServer,
    asset_root: &Path,
) {
    let bake = reference
        .corrective_bake
        .as_ref()
        .expect("validated main-menu corrective bake");
    let resource = &bake.resources[index];
    let position = Vec3::from_array(resource.position);
    let visual_archetype = resource_visual_archetype(content, &resource.kind);
    let visual = visual_archetype
        .and_then(default_archetype_scene)
        .filter(|scene| converted_asset_exists(asset_root, &scene.asset_path));
    let material = visual.and_then(|scene| {
        presentation
            .model_materials
            .get(&scene.source_model)
            .and_then(|materials| materials.get("MainMaterial"))
            .and_then(|id| render.presentation_materials.get(id))
    });
    if let Some(scene) = visual {
        let mesh_index = if resource.kind.as_str() == "resource:food" {
            0
        } else {
            resource_visual_variant(position.x, position.z, &resource.kind, 2)
        };
        let mesh = asset_server.load(
            GltfAssetLabel::Primitive {
                mesh: mesh_index,
                primitive: 0,
            }
            .from_asset(scene.asset_path.clone()),
        );
        let visual_position = visual_archetype.map_or(position, |archetype| {
            centred_resource_visual_position(position, archetype, bake.cell_size)
        });
        let mut entity = commands.spawn((
            StateEntity,
            Name::new(format!("Baked menu resource: {}", resource.id)),
            Mesh3d(mesh),
            Transform::from_translation(visual_position)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
                .with_scale(Vec3::splat(resource_visual_scale(bake.cell_size))),
        ));
        if let Some(material) = material {
            insert_main_menu_material(&mut entity, material, render);
        } else {
            entity.insert(MeshMaterial3d(render.food.clone()));
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_main_menu_baked_foliage(
    commands: &mut Commands,
    reference: &MainMenuSceneReference,
    index: usize,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: &AssetServer,
    asset_root: &Path,
) {
    let bake = reference
        .corrective_bake
        .as_ref()
        .expect("validated main-menu corrective bake");
    let foliage = &bake.foliage[index];
    let position = Vec3::from_array(foliage.position);
    let Some(layer) = content
        .foliage
        .iter()
        .find(|layer| layer.id == foliage.layer)
    else {
        return;
    };
    if layer.habitat == FoliageHabitat::Land && position.y <= bake.water_height {
        return;
    }
    let variant_index =
        foliage_visual_variant(position.x, position.z, &layer.id, layer.variants.len());
    let Some(variant) = layer.variants.get(variant_index) else {
        return;
    };
    if !converted_asset_exists(asset_root, &variant.asset_path) {
        return;
    }
    let material = foliage_layer_material(layer, render).or_else(|| {
        presentation
            .model_materials
            .get(&variant.source_model)
            .and_then(|materials| materials.values().next())
            .and_then(|id| render.presentation_materials.get(id))
    });
    let mesh = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset(variant.asset_path.clone()),
    );
    let scale = Vec3::from_array(variant.base_scale)
        * resource_visual_scale(bake.cell_size)
        * (f32::from(foliage.scale_milli) / 1_000.0);
    let mut entity = commands.spawn((
        StateEntity,
        Name::new(format!("Baked menu foliage: {}", foliage.id)),
        Mesh3d(mesh),
        PendingSurfaceGrounding {
            surface_height: position.y,
        },
        Transform::from_translation(position)
            .with_rotation(
                Quat::from_rotation_y(
                    f32::from(foliage_visual_yaw_milliradians(
                        position.x, position.z, &layer.id,
                    )) / 1_000.0,
                ) * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            )
            .with_scale(scale),
        Visibility::Hidden,
    ));
    if let Some(material) = material {
        insert_main_menu_material(&mut entity, material, render);
    } else {
        entity.insert(MeshMaterial3d(render.food.clone()));
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_main_menu_incrementally(
    mut commands: Commands,
    mut runtime: Option<ResMut<MainMenuSpawnRuntime>>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    asset_root: Res<RuntimeAssetRoot>,
    asset_server: Option<Res<AssetServer>>,
    presented_frames: Res<PresentedRenderFrames>,
) {
    let (Some(runtime), Some(asset_server)) = (runtime.as_deref_mut(), asset_server.as_deref())
    else {
        return;
    };
    let reference = embedded_main_menu_scene();
    runtime.update_count = runtime.update_count.saturating_add(1);
    let frame_started = Instant::now();
    let mut spawned_this_frame = 0;
    while spawned_this_frame < MAIN_MENU_SPAWN_BUDGET_PER_FRAME
        && runtime.phase != MainMenuSpawnPhase::Complete
        && (spawned_this_frame == 0
            || frame_started.elapsed() < LOADING_SCENE_TIME_BUDGET_PER_FRAME)
    {
        let indices = match runtime.phase {
            MainMenuSpawnPhase::Models => &runtime.model_indices,
            MainMenuSpawnPhase::Resources => &runtime.resource_indices,
            MainMenuSpawnPhase::Foliage => &runtime.foliage_indices,
            MainMenuSpawnPhase::Complete => break,
        };
        if runtime.cursor >= indices.len() {
            runtime.cursor = 0;
            runtime.phase = match runtime.phase {
                MainMenuSpawnPhase::Models => MainMenuSpawnPhase::Resources,
                MainMenuSpawnPhase::Resources => MainMenuSpawnPhase::Foliage,
                MainMenuSpawnPhase::Foliage | MainMenuSpawnPhase::Complete => {
                    MainMenuSpawnPhase::Complete
                }
            };
            continue;
        }
        let index = indices[runtime.cursor];
        match runtime.phase {
            MainMenuSpawnPhase::Models => spawn_main_menu_model_instance(
                &mut commands,
                reference,
                index,
                &content.0,
                &presentation.0,
                &render,
                asset_server,
                &asset_root.0,
            ),
            MainMenuSpawnPhase::Resources => spawn_main_menu_baked_resource(
                &mut commands,
                reference,
                index,
                &content.0,
                &presentation.0,
                &render,
                asset_server,
                &asset_root.0,
            ),
            MainMenuSpawnPhase::Foliage => spawn_main_menu_baked_foliage(
                &mut commands,
                reference,
                index,
                &content.0,
                &presentation.0,
                &render,
                asset_server,
                &asset_root.0,
            ),
            MainMenuSpawnPhase::Complete => break,
        }
        runtime.cursor += 1;
        runtime.completed += 1;
        spawned_this_frame += 1;
    }
    if runtime.phase == MainMenuSpawnPhase::Complete {
        info!(
            spawned_entities = runtime.completed,
            scheduled_entities = runtime.total,
            construction_updates = runtime.update_count,
            presented_frames = presented_frames
                .current()
                .saturating_sub(runtime.starting_render_frame),
            "completed frame-budgeted main-menu scene construction"
        );
        commands.remove_resource::<MainMenuSpawnRuntime>();
    }
}

pub(crate) fn tag_main_menu_rotating_nodes(
    mut commands: Commands,
    parents: Query<&ChildOf>,
    definitions: Query<&MainMenuRotatingDefinitions>,
    nodes: Query<(Entity, &Name), Without<MainMenuRotatingNodeProcessed>>,
) {
    if nodes.is_empty() {
        return;
    }
    for (entity, name) in &nodes {
        let mut ancestor = entity;
        let mut traversed_parent = false;
        for _ in 0..64 {
            if let Ok(definitions) = definitions.get(ancestor) {
                if let Some(rotation) = definitions
                    .0
                    .iter()
                    .find(|rotation| rotation.node == name.as_str())
                {
                    commands.entity(entity).insert(MainMenuRotatingNode {
                        axis: Vec3::from_array(rotation.axis),
                        radians_per_second: rotation.degrees_per_second.to_radians(),
                    });
                }
                commands
                    .entity(entity)
                    .insert(MainMenuRotatingNodeProcessed);
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                if traversed_parent {
                    commands
                        .entity(entity)
                        .insert(MainMenuRotatingNodeProcessed);
                }
                break;
            };
            traversed_parent = true;
            ancestor = parent.parent();
        }
    }
}

pub(crate) fn hide_main_menu_inactive_model_nodes(
    mut commands: Commands,
    parents: Query<&ChildOf>,
    definitions: Query<&MainMenuHiddenModelNodes>,
    nodes: Query<(Entity, &Name), Without<MainMenuModelNodeProcessed>>,
) {
    for (entity, name) in &nodes {
        if !definitions
            .iter()
            .any(|definitions| definitions.0.contains(name.as_str()))
        {
            continue;
        }
        let mut ancestor = entity;
        let mut belongs_to_menu_building = false;
        for _ in 0..64 {
            if let Ok(definitions) = definitions.get(ancestor) {
                belongs_to_menu_building = definitions.0.contains(name.as_str());
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
        let mut entity = commands.entity(entity);
        entity.insert(MainMenuModelNodeProcessed);
        if belongs_to_menu_building {
            entity.insert(Visibility::Hidden);
        }
    }
}

pub(crate) fn rotate_main_menu_nodes(
    time: Res<Time>,
    settings: Option<Res<RuntimePlayerSettings>>,
    mut nodes: Query<(&MainMenuRotatingNode, &mut Transform)>,
) {
    if settings.is_some_and(|settings| settings.0.interface.reduced_motion) {
        return;
    }
    for (node, mut transform) in &mut nodes {
        apply_authored_local_rotation(
            &mut transform,
            node.axis,
            node.radians_per_second,
            time.delta_secs(),
        );
    }
}

pub(crate) fn enforce_main_menu_building_shadow_casters(
    mut commands: Commands,
    parents: Query<&ChildOf>,
    roots: Query<(), With<MainMenuBuildingShadowRoot>>,
    renderers: Query<Entity, (With<Mesh3d>, Without<MainMenuBuildingShadowVerified>)>,
) {
    for renderer in &renderers {
        let mut ancestor = renderer;
        let mut belongs_to_building = false;
        for _ in 0..64 {
            if roots.contains(ancestor) {
                belongs_to_building = true;
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
        if belongs_to_building {
            // Imported opaque building renderers should both cast and receive
            // the primary menu sun's shadows. This explicit removal protects
            // them from the VFX/card exclusions used elsewhere in the scene.
            commands
                .entity(renderer)
                .remove::<bevy::light::NotShadowCaster>()
                .insert(MainMenuBuildingShadowVerified);
        }
    }
}

pub(crate) fn spawn_authored_main_menu_clouds(
    commands: &mut Commands,
    render: &RenderAssets,
    mut materials: Option<&mut Assets<StandardMaterial>>,
) {
    let base_material = materials
        .as_deref()
        .and_then(|materials| materials.get(&render.menu_cloud))
        .cloned();
    for layer in 0..MAIN_MENU_CLOUD_COLUMNS * MAIN_MENU_CLOUD_ROWS {
        let material = match (materials.as_deref_mut(), base_material.as_ref()) {
            (Some(materials), Some(base)) => {
                let mut material = base.clone();
                material.base_color.set_alpha(0.0);
                material.alpha_mode = AlphaMode::Blend;
                materials.add(material)
            }
            _ => render.menu_cloud.clone(),
        };
        commands.spawn((
            StateEntity,
            Name::new(format!("Menu cloud prism {layer:02}")),
            MainMenuCloudPrism {
                drift_per_second: Vec3::new(1.15 + small_pattern(layer, 5) * 0.12, 0.0, 0.16),
                wrap_min_x: -135.0,
                wrap_max_x: 330.0,
                fade_delay_seconds: 0.0,
                fade_elapsed_seconds: 0.0,
                entrance_fade_distance: 56.0,
                target_alpha: 0.82,
            },
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(material.clone()),
            main_menu_cloud_prism_transform(layer),
            bevy::light::NotShadowCaster,
            bevy::light::NotShadowReceiver,
            NoFrustumCulling,
        ));
    }
}

pub(crate) fn main_menu_cloud_prism_transform(layer: usize) -> Transform {
    let column = layer % MAIN_MENU_CLOUD_COLUMNS;
    let row = layer / MAIN_MENU_CLOUD_COLUMNS;
    #[allow(clippy::cast_precision_loss)]
    let (column_f, row_f) = (column as f32, row as f32);
    let small =
        |value: usize| f32::from(u8::try_from(value).expect("menu cloud pattern value fits in u8"));
    let position = Vec3::new(
        -105.0 + column_f * 34.0 - row_f * 7.0,
        25.0 + row_f * 5.5 + small((column * 2 + row) % 3) * 2.1,
        42.0 + row_f * 48.0 + small(column % 2) * 7.0,
    );
    let scale = Vec3::new(
        18.0 + small((column + row) % 3) * 7.0,
        3.0 + small((column + row * 2) % 3) * 0.75,
        12.0 + small((column * 2 + row) % 4) * 4.5,
    );
    Transform::from_translation(position).with_scale(scale)
}

pub(crate) fn small_pattern(value: usize, modulo: usize) -> f32 {
    f32::from(u8::try_from(value % modulo).expect("menu cloud pattern value fits u8"))
}

pub(crate) fn animate_main_menu_clouds(
    time: Res<Time>,
    settings: Option<Res<RuntimePlayerSettings>>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut clouds: Query<(
        &mut MainMenuCloudPrism,
        &MeshMaterial3d<StandardMaterial>,
        &mut Transform,
    )>,
) {
    let reduced_motion = settings.is_some_and(|settings| settings.0.interface.reduced_motion);
    for (mut cloud, material, mut transform) in &mut clouds {
        cloud.fade_elapsed_seconds += time.delta_secs();
        if !reduced_motion {
            transform.translation += cloud.drift_per_second * time.delta_secs();
            if transform.translation.x > cloud.wrap_max_x {
                transform.translation.x = cloud.wrap_min_x;
                cloud.fade_elapsed_seconds = 0.0;
            }
        }
        let smooth_fade = main_menu_cloud_fade(&cloud, transform.translation.x);
        if let Some(materials) = materials.as_deref_mut()
            && let Some(mut material) = materials.get_mut(material)
        {
            material
                .base_color
                .set_alpha(cloud.target_alpha * smooth_fade);
        }
    }
}

pub(crate) fn main_menu_cloud_fade(cloud: &MainMenuCloudPrism, position_x: f32) -> f32 {
    let lifetime_fade =
        ((cloud.fade_elapsed_seconds - cloud.fade_delay_seconds) / 1.8).clamp(0.0, 1.0);
    let entrance_fade =
        ((position_x - cloud.wrap_min_x) / cloud.entrance_fade_distance.max(0.001)).clamp(0.0, 1.0);
    let fade = lifetime_fade.min(entrance_fade);
    fade * fade * (3.0 - 2.0 * fade)
}

pub(crate) fn spawn_cloud_field(commands: &mut Commands, render: &RenderAssets, base_height: f32) {
    for layer in 0_u8..21 {
        let height = base_height + f32::from(layer) * 0.85;
        commands.spawn((
            StateEntity,
            Mesh3d(render.cloud_plane.clone()),
            MeshMaterial3d(render.clouds.clone()),
            Transform::from_xyz(0.0, height, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::PI))
                .with_scale(Vec3::new(900.0, 1.0, 900.0)),
            bevy::light::NotShadowCaster,
            bevy::light::NotShadowReceiver,
        ));
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_fish_school_scene(
    commands: &mut Commands,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    scene_path: &str,
    world_seed: u64,
    water_height: f32,
    state_entity: bool,
    menu_reference: Option<&MainMenuSceneReference>,
    world_surface: Option<(&GeneratedWorld, &GameConfig)>,
) {
    let Some(bindings) = presentation.scene_fish_schools.get(scene_path) else {
        return;
    };
    for (binding_index, binding) in bindings.iter().enumerate() {
        let Some(effect) = presentation.fish_school_effects.get(&binding.effect) else {
            continue;
        };
        let count = usize::from(binding.max_particles).min(FISH_SCHOOL_RENDER_BUDGET_PER_BINDING);
        let shape_half_extents = Vec3::from_array(effect.shape_scale) * 0.5;
        let shape_rotation = Quat::from_euler(
            EulerRot::XYZ,
            -effect.shape_rotation_degrees[0].to_radians(),
            -effect.shape_rotation_degrees[1].to_radians(),
            effect.shape_rotation_degrees[2].to_radians(),
        );
        // The converter excludes inactive Unity hierarchies. Active fish systems are world-space,
        // so their authored root and child offsets can be applied without invented depth layers.
        let centre = Vec3::new(
            binding.local_position[0],
            water_height + effect.particle_local_position[1] + binding.local_position[1],
            -binding.local_position[2],
        );
        for sequence in 0..count {
            let seed = fish_school_seed(
                world_seed,
                scene_path,
                binding_index,
                u32::try_from(sequence).expect("fish render budget fits u32"),
            );
            let phase = Vec3::new(
                deterministic_unit(seed) * std::f32::consts::TAU,
                deterministic_unit(seed.rotate_left(11)) * std::f32::consts::TAU,
                deterministic_unit(seed.rotate_left(23)) * std::f32::consts::TAU,
            );
            let size = deterministic_f32_range(effect.start_size, seed.rotate_left(7)) * 0.01;
            let shape_local = Vec3::new(
                deterministic_signed_unit(seed.rotate_left(3)) * shape_half_extents.x,
                deterministic_signed_unit(seed.rotate_left(13)) * shape_half_extents.y,
                -deterministic_signed_unit(seed.rotate_left(29)) * shape_half_extents.z,
            );
            let particle = FishSchoolParticle {
                base_position: centre + shape_rotation * shape_local,
                noise_amplitude: Vec3::from_array(binding.noise_strength)
                    * effect.noise_position_amount,
                phase,
                frequency: effect.noise_frequency,
                scroll_speed: effect.noise_scroll_speed,
                octaves: effect.noise_octaves,
                octave_multiplier: effect.noise_octave_multiplier,
                octave_scale: effect.noise_octave_scale,
                align_to_velocity: effect.align_to_velocity,
            };
            let intersects_land = menu_reference.is_some_and(|reference| {
                main_menu_surface_height_at(
                    reference,
                    particle.base_position.x,
                    particle.base_position.z,
                ) > water_height + 0.05
            }) || world_surface.is_some_and(|(world, config)| {
                fish_school_intersects_generated_land(
                    particle.base_position,
                    world,
                    config,
                    water_height,
                )
            });
            if intersects_land {
                continue;
            }
            let mut transform = fish_school_transform(&particle, 0.0);
            transform.scale = Vec3::splat(size);
            let mut entity = commands.spawn((
                Name::new(format!("{} fish {:03}", binding.hierarchy_path, sequence)),
                particle,
                transform,
                bevy::light::NotShadowCaster,
                bevy::light::NotShadowReceiver,
            ));
            if state_entity {
                entity.insert(StateEntity);
            } else {
                entity.insert(WorldEntity);
            }
            if let Some(mesh) = &render.fish_school_mesh {
                entity.insert((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(render.fish_school_material.clone()),
                ));
            } else {
                entity.insert((
                    Mesh3d(render.cube.clone()),
                    MeshMaterial3d(render.food.clone()),
                ));
            }
        }
    }
}

pub(crate) fn fish_school_intersects_generated_land(
    position: Vec3,
    world: &GeneratedWorld,
    config: &GameConfig,
    water_height: f32,
) -> bool {
    world_to_grid(position, config)
        .is_some_and(|cell| terrain_height(world, cell) > water_height + 0.05)
}

pub(crate) fn animate_fish_school(
    time: Res<Time>,
    settings: Option<Res<RuntimePlayerSettings>>,
    mut fish: Query<(&FishSchoolParticle, &mut Transform)>,
) {
    if settings.is_some_and(|settings| settings.0.interface.reduced_motion) {
        return;
    }
    let elapsed = time.elapsed_secs();
    for (fish, mut transform) in &mut fish {
        let scale = transform.scale;
        *transform = fish_school_transform(fish, elapsed);
        transform.scale = scale;
    }
}

pub(crate) fn main_menu_surface_height_at(
    reference: &MainMenuSceneReference,
    x: f32,
    z: f32,
) -> f32 {
    reference
        .embedded_meshes
        .first()
        .and_then(|mesh| {
            mesh.vertices.iter().min_by(|left, right| {
                let left_distance = (left[0] - x).mul_add(left[0] - x, (left[2] - z).powi(2));
                let right_distance = (right[0] - x).mul_add(right[0] - x, (right[2] - z).powi(2));
                left_distance.total_cmp(&right_distance)
            })
        })
        .map_or(f32::NEG_INFINITY, |vertex| vertex[1])
}

pub(crate) fn fish_school_transform(fish: &FishSchoolParticle, elapsed_seconds: f32) -> Transform {
    let (x, velocity_x) = fish_school_noise_axis(fish, fish.phase.x, elapsed_seconds);
    let (y, velocity_y) = fish_school_noise_axis(fish, fish.phase.y, elapsed_seconds);
    let (z, velocity_z) = fish_school_noise_axis(fish, fish.phase.z, elapsed_seconds);
    let offset = Vec3::new(x, y, z) * fish.noise_amplitude;
    let velocity = Vec3::new(velocity_x, velocity_y, velocity_z) * fish.noise_amplitude;
    let rotation = if fish.align_to_velocity && velocity.length_squared() > 1.0e-8 {
        // Primitive-label loading intentionally bypasses the imported node
        // transform. The raw converted fish points along +Y; rotating -90° on X
        // maps that nose to glTF -Z before -Z is aligned to its velocity.
        Quat::from_rotation_arc(Vec3::NEG_Z, velocity.normalize())
            * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
    } else {
        Quat::IDENTITY
    };
    Transform::from_translation(fish.base_position + offset).with_rotation(rotation)
}

pub(crate) fn fish_school_noise_axis(
    fish: &FishSchoolParticle,
    phase: f32,
    elapsed_seconds: f32,
) -> (f32, f32) {
    let octaves = fish.octaves.max(1);
    let base_rate = fish.frequency.max(0.001) * fish.scroll_speed.max(0.001);
    let mut displacement = 0.0;
    let mut velocity = 0.0;
    let mut amplitude_sum = 0.0;
    for octave in 0..octaves {
        let exponent = i32::from(octave);
        let octave = f32::from(octave);
        let amplitude = fish.octave_multiplier.powi(exponent);
        let rate = base_rate * fish.octave_scale.powi(exponent);
        let angle = phase * (1.0 + octave * 0.37) + octave * 2.399_963_1 + elapsed_seconds * rate;
        displacement += angle.sin() * amplitude;
        velocity += angle.cos() * amplitude * rate;
        amplitude_sum += amplitude.abs();
    }
    if amplitude_sum <= f32::EPSILON {
        (0.0, 0.0)
    } else {
        (displacement / amplitude_sum, velocity / amplitude_sum)
    }
}

pub(crate) fn deterministic_signed_unit(seed: u32) -> f32 {
    deterministic_unit(seed) * 2.0 - 1.0
}

pub(crate) fn fish_school_seed(
    world_seed: u64,
    scene_path: &str,
    binding: usize,
    sequence: u32,
) -> u32 {
    let mut hash = u32::try_from(world_seed & u64::from(u32::MAX)).expect("masked seed fits u32");
    for byte in scene_path.bytes() {
        hash = hash.wrapping_mul(16_777_619) ^ u32::from(byte);
    }
    hash ^ u32::try_from(binding)
        .unwrap_or(u32::MAX)
        .wrapping_mul(0x9E37_79B9)
        ^ sequence.wrapping_mul(0x85EB_CA6B)
}

pub(crate) fn top_bar_texture(render: &RenderAssets, source_path: &str) -> Option<Handle<Image>> {
    render.top_bar_textures.get(source_path).cloned()
}

pub(crate) fn main_menu_texture(render: &RenderAssets, source_path: &str) -> Handle<Image> {
    render
        .main_menu_textures
        .get(source_path)
        .cloned()
        .unwrap_or_default()
}

pub(crate) fn vote_texture(render: &RenderAssets, source_path: &str) -> Handle<Image> {
    render
        .vote_textures
        .get(source_path)
        .cloned()
        .unwrap_or_default()
}

pub(crate) fn objective_texture(render: &RenderAssets, source_path: &str) -> Handle<Image> {
    render
        .objective_textures
        .get(source_path)
        .cloned()
        .unwrap_or_default()
}

pub(crate) fn current_event_texture(render: &RenderAssets, source_path: &str) -> Handle<Image> {
    render
        .current_event_textures
        .get(source_path)
        .cloned()
        .unwrap_or_default()
}

pub(crate) fn authored_ui_image(
    render: &RenderAssets,
    source_path: &str,
    image: Handle<Image>,
) -> ImageNode {
    authored_ui_image_with_corner_scale(render, source_path, image, render.main_ui_scale)
}

pub(crate) fn settings_ui_image(
    render: &RenderAssets,
    source_path: &str,
    image: Handle<Image>,
) -> ImageNode {
    authored_ui_image_with_corner_scale(render, source_path, image, render.settings_ui_scale)
}

pub(crate) fn settings_panel_ui_image(
    render: &RenderAssets,
    source_path: &str,
    image: Handle<Image>,
) -> ImageNode {
    authored_ui_image_with_corner_scale(
        render,
        source_path,
        image,
        render.settings_ui_scale * SETTINGS_PANEL_CORNER_SCALE,
    )
}

pub(crate) fn authored_main_ui_image_with_ppu(
    render: &RenderAssets,
    source_path: &str,
    image: Handle<Image>,
    pixels_per_unit_multiplier: f32,
) -> ImageNode {
    authored_ui_image_with_corner_scale(
        render,
        source_path,
        image,
        render.main_ui_scale / pixels_per_unit_multiplier.max(0.001),
    )
}

pub(crate) fn authored_ui_image_with_corner_scale(
    render: &RenderAssets,
    source_path: &str,
    image: Handle<Image>,
    corner_scale: f32,
) -> ImageNode {
    let mode = render
        .ui_slicers
        .get(source_path)
        .map_or(NodeImageMode::Stretch, |slicer| {
            let mut slicer = slicer.clone();
            // Unity keeps the slice lines in source pixels, then scales only
            // their drawn corners by CanvasScaler / PixelsPerUnitMultiplier.
            // Altering the border itself samples the wrong part of the sprite.
            slicer.max_corner_scale = corner_scale.max(0.001);
            NodeImageMode::Sliced(slicer)
        });
    ImageNode::new(image).with_mode(mode)
}
