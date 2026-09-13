pub(crate) fn startup_destination(credits: bool, autostart: bool, resume: bool) -> BootDestination {
    if credits {
        BootDestination::Credits
    } else if autostart || resume {
        BootDestination::WorldLoading
    } else {
        BootDestination::MainMenu
    }
}

pub(crate) fn begin_menu_loading(
    mut commands: Commands,
    content: Res<RuntimeContent>,
    render: Option<Res<RenderAssets>>,
    asset_root: Res<RuntimeAssetRoot>,
    asset_server: Option<Res<AssetServer>>,
    automatic_resume: Res<AutomaticResumeRuntime>,
) {
    let destination = startup_destination(
        std::env::var_os("STREAM_TOWN_AUTOSTART_CREDITS").is_some(),
        std::env::var_os("STREAM_TOWN_AUTOSTART").is_some(),
        automatic_resume.path.is_some(),
    );
    commands.insert_resource(menu_loading_runtime(
        destination,
        &content.0,
        render.as_deref(),
        &asset_root.0,
        asset_server.as_deref(),
    ));
}

pub(crate) fn ensure_main_menu_loading(
    mut commands: Commands,
    existing: Option<Res<MenuLoadingRuntime>>,
    content: Res<RuntimeContent>,
    render: Option<Res<RenderAssets>>,
    asset_root: Res<RuntimeAssetRoot>,
    asset_server: Option<Res<AssetServer>>,
) {
    if existing.is_some() {
        return;
    }
    commands.insert_resource(menu_loading_runtime(
        BootDestination::MainMenu,
        &content.0,
        render.as_deref(),
        &asset_root.0,
        asset_server.as_deref(),
    ));
}

pub(crate) fn menu_loading_runtime(
    destination: BootDestination,
    content: &ContentCatalog,
    render: Option<&RenderAssets>,
    asset_root: &Path,
    asset_server: Option<&AssetServer>,
) -> MenuLoadingRuntime {
    let started_at = Instant::now();
    let mut asset_handles = if destination == BootDestination::MainMenu {
        asset_server.map_or_else(Vec::new, |server| {
            main_menu_preload_paths(content, asset_root)
                .into_iter()
                .map(|path| {
                    // Preload the exact scene sub-asset instantiated by
                    // WorldAssetRoot. Loading only the untyped GLB container
                    // reported 100% while Bevy still had to create Scene(0)
                    // after entering MainMenu, producing a long second wait.
                    let scene: Handle<bevy::world_serialization::WorldAsset> =
                        server.load(GltfAssetLabel::Scene(0).from_asset(path));
                    scene.untyped()
                })
                .collect()
        })
    } else {
        Vec::new()
    };
    // The loading screen is itself authored content. Keep Boot alive until its
    // background, overlay, and spinner have decoded and have had rendered
    // frames; otherwise a fast startup can transition while only the solid
    // fallback layer is visible.
    if let Some(render) = render {
        asset_handles.extend(
            [
                render.loading_screen.as_ref(),
                render.loading_overlay.as_ref(),
                render.loading_icon.as_ref(),
            ]
            .into_iter()
            .flatten()
            .cloned()
            .map(Handle::untyped),
        );
        asset_handles.extend(
            [render.ui_font.as_ref(), render.ui_display_font.as_ref()]
                .into_iter()
                .flatten()
                .cloned()
                .map(Handle::untyped),
        );
    }
    let asset_count = asset_handles.len();
    let (status, substatus) = match destination {
        BootDestination::MainMenu => (
            "Loading main menu".to_owned(),
            format!("0 / {asset_count} assets"),
        ),
        BootDestination::WorldLoading => (
            "Starting town loading".to_owned(),
            "Preparing the renderer".to_owned(),
        ),
        BootDestination::Credits => (
            "Loading credits".to_owned(),
            format!("0 / {asset_count} assets"),
        ),
    };
    MenuLoadingRuntime {
        started_at,
        destination,
        progress: 0.0,
        status,
        substatus,
        asset_handles,
        loaded_assets: 0,
        failed_assets: 0,
        ready_presented_frames: 0,
    }
}

pub(crate) fn main_menu_preload_paths(
    content: &ContentCatalog,
    asset_root: &Path,
) -> BTreeSet<String> {
    let reference = embedded_main_menu_scene();
    let mut paths = BTreeSet::new();
    for instance in &reference.instances {
        if let Some((_, _, scene)) = menu_scene_for_source_model(content, &instance.source_path)
            .filter(|(_, _, scene)| converted_asset_exists(asset_root, &scene.asset_path))
        {
            paths.insert(scene.asset_path.clone());
        }
    }
    if let Some(bake) = &reference.corrective_bake {
        for resource in &bake.resources {
            if !menu_baked_position_visible(reference, Vec3::from_array(resource.position)) {
                continue;
            }
            if let Some(scene) = resource_visual_archetype(content, &resource.kind)
                .and_then(default_archetype_scene)
                .filter(|scene| converted_asset_exists(asset_root, &scene.asset_path))
            {
                paths.insert(scene.asset_path.clone());
            }
        }
        for foliage in &bake.foliage {
            if !menu_baked_position_visible(reference, Vec3::from_array(foliage.position)) {
                continue;
            }
            if let Some(variant) = content
                .foliage
                .iter()
                .find(|layer| layer.id == foliage.layer)
                .and_then(|layer| {
                    let position = Vec3::from_array(foliage.position);
                    layer.variants.get(foliage_visual_variant(
                        position.x,
                        position.z,
                        &layer.id,
                        layer.variants.len(),
                    ))
                })
                .filter(|variant| converted_asset_exists(asset_root, &variant.asset_path))
            {
                paths.insert(variant.asset_path.clone());
            }
        }
    }
    paths
}

pub(crate) fn loaded_asset_counts(
    asset_server: Option<&AssetServer>,
    handles: &[UntypedHandle],
) -> (usize, usize) {
    let Some(asset_server) = asset_server else {
        return (handles.len(), 0);
    };
    let mut loaded = 0;
    let mut failed = 0;
    for handle in handles {
        if asset_server.is_loaded_with_dependencies(handle.id()) {
            loaded += 1;
        } else if matches!(
            asset_server.get_load_state(handle.id()),
            Some(LoadState::Failed(_))
        ) || asset_server
            .get_recursive_dependency_load_state(handle.id())
            .is_some_and(|state| state.is_failed())
        {
            failed += 1;
        }
    }
    (loaded, failed)
}

pub(crate) fn loading_fraction(completed: usize, total: usize) -> f32 {
    if total == 0 {
        return 1.0;
    }
    let completed = u16::try_from(completed).expect("preload manifest count fits u16");
    let total = u16::try_from(total).expect("preload manifest count fits u16");
    f32::from(completed) / f32::from(total)
}

pub(crate) fn world_generation_stage_label(stage: WorldGenerationStage) -> &'static str {
    match stage {
        WorldGenerationStage::Terrain => "terrain",
        WorldGenerationStage::LandResources => "land resources",
        WorldGenerationStage::NavigationAndFish => "navigation and shoreline fish",
        WorldGenerationStage::Foliage => "foliage",
        WorldGenerationStage::Fingerprint => "world fingerprint",
    }
}

pub(crate) fn loading_display_percent(progress: f32) -> f32 {
    // Truncate only the textual precision. The fill itself uses the exact
    // recursive fraction, and there is no synthetic ceiling or reserved band.
    // Because every unfinished leaf is below one, 100.00 can only be produced
    // when the complete work tree is actually complete.
    (progress.clamp(0.0, 1.0) * 10_000.0).floor() / 100.0
}

pub(crate) fn loading_percent_text(progress_percent: f32) -> String {
    if progress_percent >= 100.0 {
        "100%".to_owned()
    } else {
        format!("{progress_percent:.2}%")
    }
}

pub(crate) fn main_menu_loading_progress(
    loaded_assets: usize,
    total_assets: usize,
    constructed_entities: LoadingWork,
    ready_roots: LoadingWork,
    rendered_frames: LoadingWork,
    stable_updates: LoadingWork,
) -> f32 {
    LoadingWorkNode::group([
        LoadingWorkNode::leaf(LoadingWork::count(loaded_assets, total_assets)),
        LoadingWorkNode::leaf(constructed_entities),
        LoadingWorkNode::leaf(ready_roots),
        LoadingWorkNode::group([
            LoadingWorkNode::leaf(rendered_frames),
            LoadingWorkNode::leaf(stable_updates),
        ]),
    ])
    .fraction()
}

pub(crate) fn poll_menu_loading(
    mut loading: ResMut<MenuLoadingRuntime>,
    asset_server: Option<Res<AssetServer>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (loaded, failed) = loaded_asset_counts(asset_server.as_deref(), &loading.asset_handles);
    if failed > loading.failed_assets {
        error!(
            failed_assets = failed,
            "Boot preload contains failed assets"
        );
    }
    loading.loaded_assets = loaded;
    loading.failed_assets = failed;
    let total = loading.asset_handles.len();
    let progress = match loading.destination {
        BootDestination::MainMenu => main_menu_loading_progress(
            loaded,
            total,
            LoadingWork::Pending,
            LoadingWork::Pending,
            LoadingWork::Pending,
            LoadingWork::Pending,
        ),
        BootDestination::WorldLoading => boot_loading_display_progress(&loading),
        BootDestination::Credits => loading_fraction(loaded, total),
    };
    loading.progress = progress;
    let ready = loaded == total && failed == 0;
    match (loading.destination, ready) {
        (BootDestination::MainMenu, false) => "Loading main menu",
        (BootDestination::MainMenu, true) => "Main menu ready",
        (BootDestination::WorldLoading, false) => "Starting town loading",
        (BootDestination::WorldLoading, true) => "Renderer ready",
        (BootDestination::Credits, false) => "Loading credits",
        (BootDestination::Credits, true) => "Credits ready",
    }
    .clone_into(&mut loading.status);
    loading.substatus = if loading.destination == BootDestination::WorldLoading {
        if ready {
            "Continuing directly into town preparation".to_owned()
        } else {
            "Preparing the renderer".to_owned()
        }
    } else if failed == 0 {
        format!("{loaded} / {total} assets")
    } else {
        format!("{loaded} loaded, {failed} unavailable, {total} total")
    };
    if ready {
        loading.ready_presented_frames = loading.ready_presented_frames.saturating_add(1);
    } else {
        loading.ready_presented_frames = 0;
    }
    let required_presented_frames = if asset_server.is_some() { 3 } else { 1 };
    if ready && loading.ready_presented_frames >= required_presented_frames {
        info!(
            loaded_assets = loaded,
            failed_assets = failed,
            elapsed_seconds = loading.started_at.elapsed().as_secs_f64(),
            "Stream Town boot loading complete"
        );
        next_state.set(match loading.destination {
            BootDestination::MainMenu => GameState::MainMenu,
            BootDestination::WorldLoading => GameState::WorldLoading,
            BootDestination::Credits => GameState::Credits,
        });
    }
}

pub(crate) fn in_rendered_game_state(state: Res<State<GameState>>) -> bool {
    matches!(
        state.get(),
        GameState::MainMenu | GameState::WorldLoading | GameState::InGame
    )
}

#[cfg(test)]
pub(crate) fn next_state_targets_world_loading(next_state: &NextState<GameState>) -> bool {
    matches!(
        next_state,
        NextState::Pending(GameState::WorldLoading)
            | NextState::PendingIfNeq(GameState::WorldLoading)
    )
}

pub(crate) fn queue_world_loading(commands: &mut Commands) {
    commands.insert_resource(WorldLoadingCoverRuntime::default());
}

pub(crate) fn smoke_start_new_game_after_menu_reveal(
    time: Res<Time>,
    reveal: Option<Res<MenuRevealRuntime>>,
    cover: Option<Res<WorldLoadingCoverRuntime>>,
    mut menu: ResMut<MenuRuntime>,
    mut buttons: Query<(&GoLiveConfirmationAction, &mut Interaction)>,
    mut stage: Local<u8>,
    mut ready_seconds: Local<f32>,
) {
    if *stage >= 2 || std::env::var_os("STREAM_TOWN_SMOKE_NEW_GAME_TRANSITION").is_none() {
        return;
    }
    if *stage == 1 {
        if let Some((_, mut interaction)) = buttons
            .iter_mut()
            .find(|(action, _)| **action == GoLiveConfirmationAction::Yes)
        {
            *interaction = Interaction::Pressed;
            *stage = 2;
        }
        return;
    }
    if reveal.is_some() || cover.is_some() {
        *ready_seconds = 0.0;
        return;
    }
    *ready_seconds += time.delta_secs();
    let delay_seconds = std::env::var("STREAM_TOWN_SMOKE_NEW_GAME_DELAY_SECONDS")
        .ok()
        .and_then(|value| value.parse::<f32>().ok())
        .filter(|value| value.is_finite() && *value >= 0.0)
        .unwrap_or(0.5);
    if *ready_seconds < delay_seconds {
        return;
    }
    request_go_live_confirmation(&mut menu, PendingTownStart::NewGame);
    *stage = 1;
}

pub(crate) fn world_loading_cover_requested(cover: Option<Res<WorldLoadingCoverRuntime>>) -> bool {
    cover.is_some_and(|cover| !cover.transition_queued)
}

pub(crate) fn loading_screen_asset_handles(render: &RenderAssets) -> Vec<UntypedHandle> {
    [
        render.loading_screen.as_ref(),
        render.loading_overlay.as_ref(),
        render.loading_icon.as_ref(),
    ]
    .into_iter()
    .flatten()
    .cloned()
    .map(Handle::untyped)
    .collect()
}

pub(crate) fn begin_world_loading_cover(
    mut cover: ResMut<WorldLoadingCoverRuntime>,
    presented_frames: Res<PresentedRenderFrames>,
    render: Res<RenderAssets>,
    gpu_readiness: Res<GpuReadinessProbe>,
) {
    if cover.initialized {
        return;
    }
    let expected_images = loading_screen_asset_handles(&render)
        .into_iter()
        .filter_map(|handle| handle.id().try_typed::<Image>().ok())
        .collect();
    gpu_readiness.begin_world(expected_images, BTreeSet::new(), BTreeSet::new());
    cover.starting_render_frame = presented_frames.current();
    cover.initialized = true;
}

pub(crate) fn loading_cover_ready(
    overlay_present: bool,
    loaded_assets: usize,
    total_assets: usize,
    failed_assets: usize,
    gpu_ready: bool,
    rendered_frames: u64,
) -> bool {
    overlay_present
        && loaded_assets == total_assets
        && failed_assets == 0
        && gpu_ready
        && rendered_frames >= 3
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn advance_world_loading_cover(
    presented_frames: Res<PresentedRenderFrames>,
    render: Res<RenderAssets>,
    gpu_readiness: Res<GpuReadinessProbe>,
    asset_server: Option<Res<AssetServer>>,
    mut cover: ResMut<WorldLoadingCoverRuntime>,
    mut next_state: ResMut<NextState<GameState>>,
    loading_entities: Query<(), With<LoadingScreenEntity>>,
    mut status: Query<&mut Text, With<LoadingStatusText>>,
    mut substatus: LoadingSubstatusQuery,
    mut percent_text: LoadingPercentQuery,
    mut fills: Query<(&mut Node, &mut AccessibilityNode), With<LoadingProgressFill>>,
) {
    if !cover.initialized || cover.transition_queued {
        return;
    }
    cover.fallback_updates = cover.fallback_updates.saturating_add(1);
    let presented_render_frame = if presented_frames.render_schedule_available {
        presented_frames.current()
    } else {
        u64::from(cover.fallback_updates)
    };
    let rendered_frames = presented_render_frame.saturating_sub(cover.starting_render_frame);
    let handles = loading_screen_asset_handles(&render);
    let (loaded, failed) = loaded_asset_counts(asset_server.as_deref(), &handles);
    let gpu = gpu_readiness.snapshot();
    let gpu_ready = !presented_frames.render_schedule_available || gpu.is_ready();
    let ready = loading_cover_ready(
        !loading_entities.is_empty(),
        loaded,
        handles.len(),
        failed,
        gpu_ready,
        rendered_frames,
    );
    cover.ready_updates = if ready {
        cover.ready_updates.saturating_add(1)
    } else {
        0
    };

    let mut work = WorldLoadingWork {
        cover_entities: LoadingWork::boolean(!loading_entities.is_empty()),
        cover_artwork: LoadingWork::count(loaded, handles.len()),
        cover_presented_frames: LoadingWork::count(
            usize::try_from(rendered_frames.min(3)).expect("cover frame count fits usize"),
            3,
        ),
        ..default()
    };
    if presented_frames.render_schedule_available {
        if gpu.epoch != 0 {
            work.cover_gpu_images = LoadingWork::count(gpu.ready_images, gpu.expected_images);
            work.cover_pipelines =
                LoadingWork::boolean(gpu.pending_pipelines == 0 && gpu.failed_pipelines == 0);
        }
    } else {
        work.cover_gpu_images = LoadingWork::boolean(true);
        work.cover_pipelines = LoadingWork::boolean(true);
    }
    let recursive_progress = work.progress();
    let display_progress = loading_display_percent(recursive_progress);
    if let Ok(mut text) = status.single_mut() {
        **text = if ready {
            "Loading screen ready".to_owned()
        } else {
            "Preparing town loading".to_owned()
        };
    }
    if let Ok(mut text) = substatus.single_mut() {
        **text = format!(
            "Loading artwork {loaded}/{}; GPU images {}/{}; {} pipelines",
            handles.len(),
            gpu.ready_images,
            gpu.expected_images,
            gpu.pending_pipelines,
        );
    }
    if let Ok(mut text) = percent_text.single_mut() {
        **text = loading_percent_text(display_progress);
    }
    if let Ok((mut fill, mut accessible)) = fills.single_mut() {
        set_loading_progress(&mut fill, &mut accessible, recursive_progress * 100.0);
    }

    if cover.ready_updates < 3 {
        return;
    }
    cover.transition_queued = true;
    info!(
        rendered_frames,
        gpu_images = gpu.ready_images,
        pending_pipelines = gpu.pending_pipelines,
        elapsed_seconds = cover.started_at.elapsed().as_secs_f64(),
        "Stream Town loading cover presented before world transition"
    );
    next_state.set(GameState::WorldLoading);
}

pub(crate) fn embedded_main_menu_scene() -> &'static MainMenuSceneReference {
    static REFERENCE: OnceLock<MainMenuSceneReference> = OnceLock::new();
    REFERENCE.get_or_init(|| {
        let reference: MainMenuSceneReference = ron::from_str(include_str!(
            "../../../../../assets/content/main_menu_scene.ron"
        ))
        .expect("checked-in authored main-menu scene must parse");
        assert_eq!(
            reference.schema_version, 3,
            "checked-in authored main-menu scene schema must be supported"
        );
        assert!(
            reference.corrective_bake.is_some(),
            "checked-in authored main-menu scene must contain the corrective bake"
        );
        reference
    })
}

pub(crate) fn menu_scene_for_source_model<'a>(
    content: &'a ContentCatalog,
    source_model: &str,
) -> Option<(&'a StableId, &'a ArchetypeDef, &'a ArchetypeScene)> {
    content
        .archetypes
        .iter()
        .flat_map(|(id, archetype)| {
            archetype
                .scenes
                .iter()
                .map(move |scene| (id, archetype, scene))
        })
        .filter(|(_, _, scene)| scene.source_model == source_model)
        .min_by_key(|(id, _, scene)| (!scene.is_default, scene.asset_path.as_str(), id.as_str()))
}

pub(crate) fn main_menu_hidden_model_node_names(
    content: &ContentCatalog,
    archetype_id: &StableId,
    scene: &ArchetypeScene,
) -> BTreeSet<String> {
    let buildings = content
        .buildings
        .values()
        .filter(|building| building.archetype == *archetype_id)
        .collect::<Vec<_>>();
    let mut controlled = BTreeSet::new();
    let mut visible = BTreeSet::new();
    for building in buildings {
        for model in &building.model_handlers {
            controlled.insert(model.full_model.clone());
            controlled.extend(model.construction_stages.iter().cloned());
            controlled.extend(model.upgrades.iter().cloned());
            controlled.extend(model.other_models.iter().cloned());
            if scene.age.is_none_or(|age| model.age == age) {
                visible.insert(model.full_model.clone());
                visible.extend(model.upgrades.iter().cloned());
                visible.extend(model.other_models.iter().cloned());
            }
        }
        for model in &building.storage_models {
            controlled.insert(model.empty_model.clone());
            controlled.insert(model.half_full_model.clone());
            controlled.insert(model.full_model.clone());
            if scene.age.is_none_or(|age| model.age == age) {
                visible.insert(model.full_model.clone());
            }
        }
    }
    if scene.source_model.ends_with("/Age02_Farm.fbx") {
        // Farm growth meshes are controlled by its resource holder in Unity,
        // not BuildingResourceModelHandler, so they are absent from the generic
        // storage-model catalog. The menu depicts completed farms: keep Full
        // and prevent Seeds/Half/Full from occupying the same surface.
        controlled.extend([
            "Age02_Farm_Seeds".to_owned(),
            "Age02_Farm_Half".to_owned(),
            "Age02_Farm_Full".to_owned(),
        ]);
        visible.insert("Age02_Farm_Full".to_owned());
    }
    controlled.difference(&visible).cloned().collect()
}

pub(crate) fn spawn_completed_main_menu_farm(
    commands: &mut Commands,
    instance: &stream_town_domain::MainMenuModelInstance,
    scene: &ArchetypeScene,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: &AssetServer,
) -> bool {
    if !scene.asset_path.ends_with("/Age02_Farm.glb") {
        return false;
    }
    let material = presentation
        .model_materials
        .get(&scene.source_model)
        .and_then(|materials| materials.get("MainMaterial"))
        .and_then(|id| render.presentation_materials.get(id));
    let root_rotation = authored_scene_rotation(instance.rotation);
    let root_scale = Vec3::from_array(instance.scale);
    for (mesh_index, node_name) in [(0, "Age02_Farm_Base"), (1, "Age02_Farm_Full")] {
        let mesh = asset_server.load(
            GltfAssetLabel::Primitive {
                mesh: mesh_index,
                primitive: 0,
            }
            .from_asset(scene.asset_path.clone()),
        );
        let mut entity = commands.spawn((
            StateEntity,
            MainMenuBuildingShadowRoot,
            Name::new(format!("Direct menu farm: {node_name}")),
            Mesh3d(mesh),
            Transform::from_translation(Vec3::from_array(instance.position))
                .with_rotation(root_rotation * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
                .with_scale(root_scale * 0.01),
        ));
        if let Some(material) = material {
            insert_main_menu_material(&mut entity, material, render);
        } else {
            entity.insert(MeshMaterial3d(render.authored_building.clone()));
        }
    }
    true
}

pub(crate) fn authored_scene_rotation(rotation: [f32; 4]) -> Quat {
    // Main-menu reference schema 2 stores rotations in Bevy's right-handed space.
    Quat::from_array(rotation).normalize()
}

pub(crate) fn authored_main_menu_mesh(
    reference: &stream_town_domain::MainMenuEmbeddedMesh,
    water_height: f32,
) -> Mesh {
    let (positions, normals, uv) = clipped_main_menu_geometry(reference, water_height);
    let vertex_count = u32::try_from(positions.len()).expect("menu shoreline mesh fits u32");
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
    .with_inserted_indices(Indices::U32((0..vertex_count).collect()))
}

#[derive(Clone, Copy)]
struct MainMenuClipVertex {
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
}

type MainMenuMeshGeometry = (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>);

pub(crate) fn clipped_main_menu_geometry(
    reference: &stream_town_domain::MainMenuEmbeddedMesh,
    water_height: f32,
) -> MainMenuMeshGeometry {
    let source_vertex = |index: u32| {
        let index = usize::try_from(index).expect("validated main-menu mesh index");
        MainMenuClipVertex {
            position: Vec3::from_array(reference.vertices[index]),
            normal: Vec3::from_array(reference.normals[index]),
            uv: Vec2::from_array(reference.uv[index]),
        }
    };
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uv = Vec::new();
    for triangle in reference.triangles.chunks_exact(3) {
        let source = [
            source_vertex(triangle[0]),
            source_vertex(triangle[1]),
            source_vertex(triangle[2]),
        ];
        let mut polygon = Vec::with_capacity(4);
        let mut previous = source[2];
        let mut previous_inside = previous.position.y >= water_height;
        for current in source {
            let current_inside = current.position.y >= water_height;
            if current_inside != previous_inside {
                let progress = (water_height - previous.position.y)
                    / (current.position.y - previous.position.y);
                let mut position = previous.position.lerp(current.position, progress);
                position.y = water_height;
                polygon.push(MainMenuClipVertex {
                    position,
                    normal: previous
                        .normal
                        .lerp(current.normal, progress)
                        .normalize_or(Vec3::Y),
                    uv: previous.uv.lerp(current.uv, progress),
                });
            }
            if current_inside {
                polygon.push(current);
            }
            previous = current;
            previous_inside = current_inside;
        }
        for corner in 1..polygon.len().saturating_sub(1) {
            for vertex in [polygon[0], polygon[corner], polygon[corner + 1]] {
                positions.push(vertex.position.to_array());
                normals.push(vertex.normal.to_array());
                uv.push(vertex.uv.to_array());
            }
        }
    }
    (positions, normals, uv)
}

pub(crate) fn apply_authored_main_menu_camera(
    reference: &MainMenuSceneReference,
    cameras: &mut TownCameraMutQuery,
) {
    let camera_reference = &reference.camera;
    for (mut camera, mut projection, mut transform, mut ambient) in cameras.iter_mut() {
        camera.clear_color = bevy::camera::ClearColorConfig::Custom(Color::srgba(
            camera_reference.background[0],
            camera_reference.background[1],
            camera_reference.background[2],
            camera_reference.background[3],
        ));
        *projection = if camera_reference.orthographic {
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: camera_reference.orthographic_size * 2.0,
                },
                near: camera_reference.near,
                far: camera_reference.far,
                ..OrthographicProjection::default_3d()
            })
        } else {
            Projection::Perspective(PerspectiveProjection {
                fov: camera_reference.field_of_view_degrees.to_radians(),
                near: camera_reference.near,
                far: camera_reference.far,
                ..default()
            })
        };
        *transform = Transform::from_translation(Vec3::from_array(camera_reference.position))
            .with_rotation(authored_scene_rotation(camera_reference.rotation));
        // The Unity camera renders against its sky-lit environment. The town
        // runtime's lower ambient level is balanced for an overhead sun and
        // leaves the authored side-on menu city nearly black.
        ambient.color = Color::srgb(0.212, 0.227, 0.259);
        ambient.brightness = 150.0;
    }
}

pub(crate) fn restore_town_camera_for_world(
    settings: Res<RuntimePlayerSettings>,
    mut cameras: TownCameraMutQuery,
    mut controllers: Query<&mut TownCameraControllerRuntime, With<TownCamera>>,
    mut sun: TownSunMutQuery,
) {
    for (mut camera, mut projection, mut transform, mut ambient) in &mut cameras {
        camera.clear_color = bevy::camera::ClearColorConfig::Default;
        *projection = town_camera_projection(f32::from(settings.0.camera.field_of_view_degrees));
        *transform = default_town_camera_transform();
        ambient.color = Color::srgb(0.70, 0.82, 0.92);
        ambient.brightness = in_game_ambient_brightness(90.0);
    }
    if let Ok(mut controller) = controllers.single_mut() {
        controller.set_home(default_town_camera_transform());
    }
    if let Ok((mut light, mut transform)) = sun.single_mut() {
        light.color = Color::WHITE;
        light.illuminance = 14_000.0;
        light.shadow_maps_enabled = true;
        *transform = in_game_sun_transform();
    }
}
