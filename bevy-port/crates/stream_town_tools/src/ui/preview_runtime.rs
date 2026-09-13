#[cfg(target_os = "windows")]
fn tools_wgpu_settings() -> WgpuSettings {
    // Match the shipping game: the current Windows AMD driver emits invalid
    // Vulkan swapchain transitions, while DX12 is stable and supported.
    WgpuSettings {
        backends: Some(Backends::DX12),
        ..default()
    }
}

fn setup_camera(mut commands: Commands, mut egui_settings: ResMut<EguiGlobalSettings>) {
    egui_settings.auto_create_primary_context = false;
    commands.spawn((Camera2d, PrimaryEguiContext));
}

fn setup_model_preview(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut egui_textures: ResMut<EguiUserTextures>,
) {
    let image = images.add(Image::new_target_texture(
        768,
        512,
        TextureFormat::Bgra8UnormSrgb,
        None,
    ));
    egui_textures.add_image(EguiTextureHandle::Strong(image.clone()));
    commands.spawn((
        ModelPreviewCamera,
        Camera3d::default(),
        Camera {
            order: -1,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.035, 0.05, 0.065)),
            ..default()
        },
        RenderTarget::Image(image.clone().into()),
        AmbientLight {
            color: Color::srgb(0.82, 0.88, 0.96),
            brightness: 260.0,
            ..default()
        },
        Transform::from_xyz(5.5, 4.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 9_000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.insert_resource(ModelPreviewRuntime {
        image,
        scene_entity: None,
        scene_handle: None,
        loaded_request: None,
        animation_graph: None,
        animation_node: None,
        animation_started: false,
        material_overrides: PreviewMaterialOverrides::default(),
        visible_nodes: None,
        camera_target_offset: Vec3::ZERO,
        framed: false,
        status: "Choose a model to render it here".to_owned(),
        controls: ModelPreviewControls::default(),
    });
}

fn sync_model_preview(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<ToolState>,
    mut preview: ResMut<ModelPreviewRuntime>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    if preview.loaded_request == state.preview_request {
        return;
    }
    if let Some(entity) = preview.scene_entity.take() {
        commands.entity(entity).try_despawn();
    }
    preview.scene_handle = None;
    preview.animation_graph = None;
    preview.animation_node = None;
    preview.animation_started = false;
    preview.material_overrides = PreviewMaterialOverrides::default();
    preview.visible_nodes = None;
    preview.camera_target_offset = Vec3::ZERO;
    preview.framed = false;
    preview.loaded_request.clone_from(&state.preview_request);
    let Some(request) = state.preview_request.as_ref() else {
        "Nothing selected for preview".clone_into(&mut preview.status);
        return;
    };
    match request {
        PreviewRequest::Model(asset_path) => {
            preview.material_overrides =
                preview_material_overrides(asset_path, &state, &asset_server, &mut materials);
            let scene_handle =
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(asset_path.clone()));
            let entity = commands
                .spawn((
                    Name::new(format!("Model preview: {}", state.preview_label)),
                    ModelPreviewScene,
                    WorldAssetRoot(scene_handle.clone()),
                    Transform::IDENTITY,
                ))
                .id();
            preview.scene_entity = Some(entity);
            preview.scene_handle = Some(scene_handle);
            preview.status = format!("Loading textured GLB: {asset_path}");
        }
        PreviewRequest::Animation {
            clip,
            asset_path,
            animation_index,
            ..
        } => {
            preview.material_overrides = if asset_path == PLAYER_ANIMATED_MODEL_PATH {
                player_preview_material_overrides(&state, &asset_server, &mut materials)
            } else {
                preview_material_overrides(asset_path, &state, &asset_server, &mut materials)
            };
            let scene_handle =
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(asset_path.clone()));
            let entity = commands
                .spawn((
                    Name::new(format!("Animation preview: {clip}")),
                    ModelPreviewScene,
                    WorldAssetRoot(scene_handle.clone()),
                    Transform::IDENTITY,
                ))
                .id();
            let animation = asset_server.load(
                GltfAssetLabel::Animation(
                    usize::try_from(*animation_index).expect("animation index fits platform"),
                )
                .from_asset(asset_path.clone()),
            );
            let (graph, node) = AnimationGraph::from_clip(animation);
            preview.scene_entity = Some(entity);
            preview.scene_handle = Some(scene_handle);
            preview.animation_graph = Some(animation_graphs.add(graph));
            preview.animation_node = Some(node);
            if let Some(definition) = state.presentation.clips.get(clip) {
                preview.controls.animation_looping = definition.looping;
            }
            preview.status = format!(
                "Loading animation {} from {}#Animation{}",
                state.preview_label, asset_path, animation_index
            );
        }
        PreviewRequest::Role {
            role,
            asset_path,
            animation_index,
            looping,
            visible_nodes,
            ..
        } => {
            preview.material_overrides =
                player_preview_material_overrides(&state, &asset_server, &mut materials);
            preview.visible_nodes = Some(visible_nodes.clone());
            // The imported skinned AABBs describe the rest pose and sit below
            // the head motion. Aim at the visible animated figure's centre.
            preview.camera_target_offset = Vec3::Y * 0.75;
            let scene_handle =
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(asset_path.clone()));
            let entity = commands
                .spawn((
                    Name::new(format!("Role preview: {role}")),
                    ModelPreviewScene,
                    WorldAssetRoot(scene_handle.clone()),
                    Transform::IDENTITY,
                ))
                .id();
            let animation = asset_server.load(
                GltfAssetLabel::Animation(
                    usize::try_from(*animation_index).expect("animation index fits platform"),
                )
                .from_asset(asset_path.clone()),
            );
            let (graph, node) = AnimationGraph::from_clip(animation);
            preview.scene_entity = Some(entity);
            preview.scene_handle = Some(scene_handle);
            preview.animation_graph = Some(animation_graphs.add(graph));
            preview.animation_node = Some(node);
            preview.controls.animation_looping = *looping;
            preview.status = format!(
                "Loading composed role {} with animation #{}",
                state.preview_label, animation_index
            );
        }
        PreviewRequest::Material { id, .. } => {
            let Some(definition) = state.presentation.materials.get(id) else {
                preview.status = format!("Missing material {id}");
                return;
            };
            let material = materials.add(preview_standard_material(
                definition,
                &state.presentation,
                &asset_server,
            ));
            let entity = commands
                .spawn((
                    Name::new(format!("Material preview: {id}")),
                    ModelPreviewScene,
                    Mesh3d(
                        meshes.add(
                            Sphere::new(1.55)
                                .mesh()
                                .ico(5)
                                .expect("valid preview sphere"),
                        ),
                    ),
                    MeshMaterial3d(material),
                    Transform::from_xyz(0.0, 0.25, 0.0),
                ))
                .id();
            preview.scene_entity = Some(entity);
            preview.framed = true;
            preview.status = format!("Live PBR material preview · {}", definition.display_name);
        }
        PreviewRequest::Texture { id, .. } => {
            let Some(definition) = state.presentation.textures.get(id) else {
                preview.status = format!("Missing texture {id}");
                return;
            };
            let material = materials.add(StandardMaterial {
                base_color: Color::WHITE,
                base_color_texture: Some(asset_server.load(definition.asset_path.clone())),
                perceptual_roughness: 0.82,
                cull_mode: None,
                ..default()
            });
            let entity = commands
                .spawn((
                    Name::new(format!("Texture preview: {id}")),
                    ModelPreviewScene,
                    Mesh3d(meshes.add(Cuboid::new(3.8, 2.6, 0.08))),
                    MeshMaterial3d(material),
                    Transform::IDENTITY,
                ))
                .id();
            preview.scene_entity = Some(entity);
            preview.framed = true;
            preview.status = format!("Live texture preview · {}", definition.asset_path);
        }
    }
}

fn preview_standard_material(
    material: &MaterialDef,
    presentation: &PresentationCatalog,
    asset_server: &AssetServer,
) -> StandardMaterial {
    const PRIORITY: [&str; 8] = [
        "_BaseMap",
        "_BaseColorMap",
        "_MainTexture",
        "_MainTex",
        "_Texture0",
        "_characterTexture",
        "_BaseColorRGBOutlineWidthA",
        "_BaseColorRGBSmoothnessA",
    ];
    let primary = PRIORITY
        .iter()
        .filter_map(|slot| material.textures.get_key_value(*slot))
        .chain(material.textures.iter())
        .find_map(|(slot, id)| {
            presentation
                .textures
                .get(id)
                .map(|texture| (slot.as_str(), texture.asset_path.as_str()))
        });
    let transform = primary
        .and_then(|(slot, _)| material.texture_transforms.get(slot))
        .copied()
        .unwrap_or_default();
    StandardMaterial {
        base_color: Color::srgba(
            material.base_color[0],
            material.base_color[1],
            material.base_color[2],
            material.base_color[3],
        ),
        base_color_texture: primary.map(|(_, path)| asset_server.load(path.to_owned())),
        emissive: LinearRgba::new(
            material.emissive[0],
            material.emissive[1],
            material.emissive[2],
            material.emissive[3],
        ),
        metallic: material.metallic,
        perceptual_roughness: material.perceptual_roughness,
        alpha_mode: match material.alpha_mode {
            MaterialAlphaMode::Opaque => AlphaMode::Opaque,
            MaterialAlphaMode::Mask => AlphaMode::Mask(0.5),
            MaterialAlphaMode::Blend => AlphaMode::Blend,
        },
        uv_transform: Affine2::from_scale_angle_translation(
            Vec2::from_array(transform.scale),
            0.0,
            Vec2::from_array(transform.offset),
        ),
        cull_mode: None,
        ..default()
    }
}

fn preview_material_overrides(
    asset_path: &str,
    state: &ToolState,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
) -> PreviewMaterialOverrides {
    let selected = state
        .selected_archetype
        .as_ref()
        .and_then(|id| state.catalog.archetypes.get(id))
        .filter(|archetype| {
            archetype
                .scenes
                .iter()
                .any(|scene| scene.asset_path == asset_path)
        });
    let archetype = selected.or_else(|| {
        state.catalog.archetypes.values().find(|archetype| {
            archetype
                .scenes
                .iter()
                .any(|scene| scene.asset_path == asset_path)
        })
    });
    let Some(archetype) = archetype else {
        return PreviewMaterialOverrides::default();
    };
    let Some(scene) = archetype
        .scenes
        .iter()
        .find(|scene| scene.asset_path == asset_path)
    else {
        return PreviewMaterialOverrides::default();
    };
    let make_material = |id: &StableId, materials: &mut Assets<StandardMaterial>| {
        state.presentation.materials.get(id).map(|definition| {
            materials.add(preview_standard_material(
                definition,
                &state.presentation,
                asset_server,
            ))
        })
    };
    let fallback = state
        .presentation
        .prefab_materials
        .get(&archetype.source_guid)
        .into_iter()
        .flatten()
        .find_map(|id| make_material(id, materials));
    let model_materials = state
        .presentation
        .model_materials
        .get(&scene.source_model)
        .into_iter()
        .flat_map(|bindings| bindings.iter())
        .filter_map(|(name, id)| {
            make_material(id, materials).map(|material| (name.clone(), material))
        })
        .collect();
    let renderer_materials = state
        .presentation
        .prefab_renderer_materials
        .get(&archetype.source_guid)
        .into_iter()
        .flatten()
        .filter_map(|binding| {
            let bound = binding
                .materials
                .iter()
                .filter_map(|(name, id)| {
                    make_material(id, materials).map(|material| (name.clone(), material))
                })
                .collect::<BTreeMap<_, _>>();
            (!bound.is_empty()).then(|| PreviewRendererMaterialBinding {
                target_path: binding.target_path.clone(),
                materials: bound,
            })
        })
        .collect();
    PreviewMaterialOverrides {
        fallback,
        model_materials,
        renderer_materials,
    }
}

fn player_preview_material_overrides(
    state: &ToolState,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
) -> PreviewMaterialOverrides {
    let Some(archetype) = state
        .catalog
        .archetypes
        .values()
        .find(|archetype| archetype.source_path.ends_with("Player_Character.prefab"))
    else {
        return PreviewMaterialOverrides::default();
    };
    let make_material = |id: &StableId, materials: &mut Assets<StandardMaterial>| {
        state.presentation.materials.get(id).map(|definition| {
            materials.add(preview_standard_material(
                definition,
                &state.presentation,
                asset_server,
            ))
        })
    };
    let fallback = state
        .presentation
        .prefab_materials
        .get(&archetype.source_guid)
        .into_iter()
        .flatten()
        .find_map(|id| make_material(id, materials));
    let model_materials = state
        .presentation
        .model_materials
        .get(PLAYER_ANIMATED_SOURCE_MODEL)
        .into_iter()
        .flat_map(|bindings| bindings.iter())
        .filter_map(|(name, id)| {
            make_material(id, materials).map(|material| (name.clone(), material))
        })
        .collect();
    let renderer_materials = state
        .presentation
        .prefab_renderer_materials
        .get(&archetype.source_guid)
        .into_iter()
        .flatten()
        .filter_map(|binding| {
            let bound = binding
                .materials
                .iter()
                .filter_map(|(name, id)| {
                    make_material(id, materials).map(|material| (name.clone(), material))
                })
                .collect::<BTreeMap<_, _>>();
            (!bound.is_empty()).then(|| PreviewRendererMaterialBinding {
                target_path: binding.target_path.clone(),
                materials: bound,
            })
        })
        .collect();
    PreviewMaterialOverrides {
        fallback,
        model_materials,
        renderer_materials,
    }
}

#[allow(clippy::type_complexity)]
fn apply_preview_material_overrides(
    mut commands: Commands,
    preview: Res<ModelPreviewRuntime>,
    parents: Query<&ChildOf>,
    names: Query<&Name>,
    roots: Query<(), With<ModelPreviewScene>>,
    mut renderers: Query<
        (
            Entity,
            &mut MeshMaterial3d<StandardMaterial>,
            Option<&GltfMeshName>,
            Option<&GltfMaterialName>,
        ),
        Without<PreviewMaterialApplied>,
    >,
) {
    for (entity, mut material, mesh_name, material_name) in &mut renderers {
        let mut ancestor = entity;
        let mut path = Vec::new();
        let mut belongs_to_preview = false;
        for _ in 0..64 {
            if roots.contains(ancestor) {
                belongs_to_preview = true;
                break;
            }
            if let Ok(name) = names.get(ancestor) {
                path.push(name.as_str().to_owned());
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
        if !belongs_to_preview {
            continue;
        }
        let hierarchy_path = path.iter().rev().cloned().collect::<Vec<_>>().join("/");
        if let Some(authored) = resolved_preview_material(
            &preview.material_overrides,
            &hierarchy_path,
            mesh_name.map(|name| name.0.as_str()),
            material_name.map(|name| name.0.as_str()),
        ) {
            material.0 = authored.clone();
        }
        commands.entity(entity).insert(PreviewMaterialApplied);
    }
}

fn canonical_preview_node_name(name: &str) -> &str {
    name.strip_suffix("_Starter").unwrap_or(name)
}

fn player_preview_controlled_node(name: &str) -> bool {
    const COSMETIC_PREFIXES: [&str; 3] = ["Eyes_", "Hair_", "FacialHair_"];
    const EQUIPMENT_PREFIXES: [&str; 5] = ["Body_", "Back_", "LHand_", "RHand_", "Helmet_"];
    let name = canonical_preview_node_name(name);
    COSMETIC_PREFIXES
        .iter()
        .chain(EQUIPMENT_PREFIXES.iter())
        .any(|prefix| name.starts_with(prefix))
}

fn apply_preview_node_visibility(
    mut commands: Commands,
    preview: Res<ModelPreviewRuntime>,
    parents: Query<&ChildOf>,
    roots: Query<(), With<ModelPreviewScene>>,
    mut nodes: Query<(Entity, &Name, &mut Visibility), Without<PreviewNodeVisibilityApplied>>,
) {
    let Some(visible_nodes) = preview.visible_nodes.as_ref() else {
        return;
    };
    for (entity, name, mut visibility) in &mut nodes {
        let mut ancestor = entity;
        let mut belongs_to_preview = roots.contains(entity);
        for _ in 0..64 {
            if belongs_to_preview {
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
            belongs_to_preview = roots.contains(ancestor);
        }
        if !belongs_to_preview {
            continue;
        }
        let canonical = canonical_preview_node_name(name.as_str());
        if player_preview_controlled_node(canonical) {
            *visibility = if visible_nodes.contains(canonical) {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
        commands.entity(entity).insert(PreviewNodeVisibilityApplied);
    }
}

fn resolved_preview_material<'a>(
    overrides: &'a PreviewMaterialOverrides,
    hierarchy_path: &str,
    mesh_name: Option<&str>,
    material_name: Option<&str>,
) -> Option<&'a Handle<StandardMaterial>> {
    let node_path = if mesh_name.is_some() {
        hierarchy_path
            .rsplit_once('/')
            .map_or(hierarchy_path, |(path, _)| path)
    } else {
        hierarchy_path
    };
    let target_name = mesh_name
        .or_else(|| node_path.rsplit('/').next())
        .unwrap_or_default();
    let full_path_matches = |binding: &&PreviewRendererMaterialBinding| {
        binding.target_path == node_path
            || node_path.ends_with(&format!("/{}", binding.target_path))
            || binding.target_path.ends_with(&format!("/{node_path}"))
    };
    let name_matches = |binding: &&PreviewRendererMaterialBinding| {
        binding.target_path.rsplit('/').next() == Some(target_name)
    };
    let exact_path = material_name
        .and_then(|name| {
            overrides
                .renderer_materials
                .iter()
                .filter(full_path_matches)
                .find(|binding| binding.materials.contains_key(name))
        })
        .or_else(|| overrides.renderer_materials.iter().find(full_path_matches));
    let exact = exact_path.or_else(|| {
        material_name
            .and_then(|name| {
                overrides
                    .renderer_materials
                    .iter()
                    .filter(name_matches)
                    .find(|binding| binding.materials.contains_key(name))
            })
            .or_else(|| overrides.renderer_materials.iter().find(name_matches))
    });
    material_name
        .and_then(|name| exact.and_then(|binding| binding.materials.get(name)))
        .or_else(|| material_name.and_then(|name| overrides.model_materials.get(name)))
        .or_else(|| exact.and_then(|binding| binding.materials.values().next()))
        .or(overrides.fallback.as_ref())
}

fn frame_model_preview(
    asset_server: Res<AssetServer>,
    mut preview: ResMut<ModelPreviewRuntime>,
    children: Query<&Children>,
    bounds: Query<(&GlobalTransform, &Aabb)>,
    mut roots: Query<&mut Transform, With<ModelPreviewScene>>,
) {
    if preview.framed {
        return;
    }
    let Some(root) = preview.scene_entity else {
        return;
    };
    let Some(handle) = preview.scene_handle.as_ref() else {
        return;
    };
    match asset_server.load_state(handle.id()) {
        LoadState::Failed(error) => {
            preview.status = format!("GLB load failed: {error}");
            return;
        }
        LoadState::Loading | LoadState::NotLoaded => return,
        LoadState::Loaded => {}
    }

    let mut stack = children
        .get(root)
        .map_or_else(|_| Vec::new(), |value| value.iter().collect::<Vec<_>>());
    let mut minimum = Vec3::splat(f32::INFINITY);
    let mut maximum = Vec3::splat(f32::NEG_INFINITY);
    let mut mesh_count = 0_usize;
    while let Some(entity) = stack.pop() {
        if let Ok(value) = children.get(entity) {
            stack.extend(value.iter());
        }
        let Ok((global, aabb)) = bounds.get(entity) else {
            continue;
        };
        mesh_count += 1;
        let center = Vec3::from(aabb.center);
        let half = Vec3::from(aabb.half_extents);
        for x in [-1.0, 1.0] {
            for y in [-1.0, 1.0] {
                for z in [-1.0, 1.0] {
                    let point = global.transform_point(center + half * Vec3::new(x, y, z));
                    minimum = minimum.min(point);
                    maximum = maximum.max(point);
                }
            }
        }
    }
    if mesh_count == 0 {
        "GLB loaded; waiting for render meshes".clone_into(&mut preview.status);
        return;
    }
    let size = maximum - minimum;
    let largest = size.max_element().max(0.01);
    let scale = 4.0 / largest;
    let center = (minimum + maximum) * 0.5;
    let Ok(mut transform) = roots.get_mut(root) else {
        return;
    };
    *transform = Transform::from_translation(-center * scale).with_scale(Vec3::splat(scale));
    preview.framed = true;
    let authored_bindings = usize::from(preview.material_overrides.fallback.is_some())
        + preview.material_overrides.model_materials.len()
        + preview
            .material_overrides
            .renderer_materials
            .iter()
            .map(|binding| binding.materials.len())
            .sum::<usize>();
    let appearance = if authored_bindings == 0 {
        "embedded GLB materials".to_owned()
    } else {
        format!("{authored_bindings} authored material binding(s)")
    };
    preview.status = if preview.animation_graph.is_some() {
        format!("Loaded and framed {mesh_count} mesh(es) with {appearance}; attaching animation")
    } else {
        format!("Loaded and framed {mesh_count} mesh(es) with {appearance}")
    };
}

fn update_model_preview_camera(
    preview: Res<ModelPreviewRuntime>,
    mut cameras: Query<&mut Transform, With<ModelPreviewCamera>>,
) {
    if !preview.is_changed() {
        return;
    }
    let controls = &preview.controls;
    let target = preview.camera_target_offset + controls.pan;
    let offset = preview_camera_offset(controls.yaw, controls.pitch, controls.distance);
    for mut transform in &mut cameras {
        *transform = Transform::from_translation(target + offset).looking_at(target, Vec3::Y);
    }
}

fn preview_camera_offset(yaw: f32, pitch: f32, distance: f32) -> Vec3 {
    let horizontal = distance * pitch.cos();
    Vec3::new(
        yaw.sin() * horizontal,
        distance * pitch.sin(),
        yaw.cos() * horizontal,
    )
}

fn preview_pan_delta(yaw: f32, pitch: f32, drag_x: f32, drag_y: f32, scale: f32) -> Vec3 {
    let camera_offset = preview_camera_offset(yaw, pitch, 1.0);
    let forward = (-camera_offset).normalize_or_zero();
    let right = forward.cross(Vec3::Y).normalize_or_zero();
    let up = right.cross(forward).normalize_or_zero();
    -right * drag_x * scale + up * drag_y * scale
}

fn drive_model_preview_animation(
    mut commands: Commands,
    mut preview: ResMut<ModelPreviewRuntime>,
    mut players: Query<(Entity, &mut AnimationPlayer)>,
) {
    let (Some(graph), Some(node)) = (preview.animation_graph.clone(), preview.animation_node)
    else {
        return;
    };
    let mut attached = false;
    let restart = preview.controls.restart_animation;
    for (entity, mut player) in &mut players {
        if !player.is_playing_animation(node) {
            player.play(node);
            commands
                .entity(entity)
                .insert(AnimationGraphHandle(graph.clone()));
        }
        if let Some(active) = player.animation_mut(node) {
            if restart {
                active.rewind();
            }
            active.set_speed(preview.controls.animation_speed);
            active.set_repeat(if preview.controls.animation_looping {
                RepeatAnimation::Forever
            } else {
                RepeatAnimation::Never
            });
            if preview.controls.animation_playing {
                active.resume();
            } else {
                active.pause();
            }
            attached = true;
        }
    }
    if attached && !preview.animation_started {
        preview.animation_started = true;
        "Animation playing on the textured preview rig".clone_into(&mut preview.status);
    }
    if restart {
        preview.controls.restart_animation = false;
    }
}
