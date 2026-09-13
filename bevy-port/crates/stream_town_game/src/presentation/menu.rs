use super::super::*;

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
            "../../../../assets/content/main_menu_scene.ron"
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

pub(crate) fn spawn_main_menu(
    mut commands: Commands,
    presented_frames: Res<PresentedRenderFrames>,
    render: Res<RenderAssets>,
    presentation: Res<RuntimePresentation>,
    asset_server: Option<Res<AssetServer>>,
    meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut cameras: TownCameraMutQuery,
    mut sun: TownSunMutQuery,
) {
    commands.insert_resource(MenuRevealRuntime {
        started_at: Instant::now(),
        starting_render_frame: presented_frames.current(),
        ready_starting_render_frame: None,
        fallback_updates: 0,
        scene_ready_frames: 0,
    });
    let reference = embedded_main_menu_scene();
    let bake = reference
        .corrective_bake
        .as_ref()
        .expect("validated main-menu corrective bake");
    apply_authored_main_menu_camera(reference, &mut cameras);
    commands.spawn((
        StateEntity,
        Name::new("Main-menu gradient sky"),
        Mesh3d(render.menu_sky_mesh.clone()),
        MeshMaterial3d(render.menu_sky.clone()),
        Transform::from_translation(Vec3::from_array(reference.camera.position))
            .with_scale(Vec3::splat(reference.camera.far * 0.82)),
        bevy::light::NotShadowCaster,
        bevy::light::NotShadowReceiver,
    ));
    if let Ok((mut light, mut transform)) = sun.single_mut() {
        light.color = Color::srgb(1.0, 0.956_862_75, 0.839_215_7);
        light.illuminance = 14_000.0;
        light.shadow_maps_enabled = true;
        *transform = Transform::from_rotation(Quat::from_xyzw(
            -0.408_217_88,
            0.234_569_68,
            0.109_381_63,
            0.875_426_1,
        ));
    }
    commands.spawn((
        StateEntity,
        Name::new("Authored menu secondary light"),
        DirectionalLight {
            color: Color::srgb(1.0, 0.956_862_75, 0.839_215_7),
            illuminance: 7_000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_xyzw(
            0.951_251_27,
            0.044_943_463,
            -0.254_886_93,
            0.167_731_26,
        )),
    ));

    if let Some(mut meshes) = meshes {
        for embedded in &reference.embedded_meshes {
            commands.spawn((
                StateEntity,
                Name::new(format!("Authored menu mesh: {}", embedded.hierarchy_path)),
                Mesh3d(meshes.add(authored_main_menu_mesh(embedded, bake.water_height))),
                MeshMaterial3d(render.ground.clone()),
            ));
        }
        // The source prefab's water cube ended exactly at the flat terrain's
        // Y=0 surface. The corrective bake lowers the surface below the beach,
        // expands it past the camera frustum, and uses a plane so only the
        // animated water surface participates in transparent sorting.
        commands.spawn((
            StateEntity,
            Name::new("Baked main-menu ocean floor"),
            Mesh3d(render.cloud_plane.clone()),
            MeshMaterial3d(render.menu_ocean_floor.clone()),
            Transform::from_xyz(0.0, bake.ocean_floor_height, 0.0)
                .with_scale(Vec3::splat(bake.terrain_extent)),
        ));
        commands.spawn((
            StateEntity,
            Name::new("Baked main-menu animated water"),
            Mesh3d(render.cloud_plane.clone()),
            MeshMaterial3d(render.menu_water.clone()),
            Transform::from_xyz(0.0, bake.water_height, 0.0)
                .with_scale(Vec3::splat(bake.water_extent)),
            bevy::light::NotShadowCaster,
            bevy::light::NotShadowReceiver,
        ));
    }

    if asset_server.is_some() {
        let model_indices = reference
            .instances
            .iter()
            .enumerate()
            .filter(|(_, instance)| {
                menu_baked_position_visible(reference, Vec3::from_array(instance.position))
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let (resource_indices, foliage_indices) = main_menu_baked_decoration_indices(reference);
        let total = model_indices.len() + resource_indices.len() + foliage_indices.len();
        let direct_farms = model_indices
            .iter()
            .filter(|index| {
                reference.instances[**index]
                    .source_path
                    .ends_with("/Age02_Farm.fbx")
            })
            .count();
        info!(
            baked_instances = reference.instances.len(),
            rendered_instances = model_indices.len(),
            direct_farms,
            queued_resources = resource_indices.len(),
            queued_foliage = foliage_indices.len(),
            "queued frame-budgeted main-menu scene construction"
        );
        commands.insert_resource(MainMenuSpawnRuntime {
            phase: if model_indices.is_empty() {
                MainMenuSpawnPhase::Resources
            } else {
                MainMenuSpawnPhase::Models
            },
            model_indices,
            resource_indices,
            foliage_indices,
            cursor: 0,
            completed: 0,
            total,
            update_count: 0,
            starting_render_frame: presented_frames.current(),
        });
    }

    spawn_authored_main_menu_clouds(&mut commands, &render, materials.as_deref_mut());
    spawn_fish_school_scene(
        &mut commands,
        &presentation.0,
        &render,
        MAIN_MENU_SCENE_PATH,
        0,
        bake.water_height,
        true,
        Some(reference),
        None,
    );
    commands.spawn((
        StateEntity,
        Name::new("Shipping Main Menu backdrop"),
        ImageNode::new(main_menu_texture(&render, MAIN_MENU_TEXTURE_PATHS[3]))
            .with_color(Color::srgba(0.090_196_08, 0.109_803_93, 0.203_921_59, 0.55))
            .with_mode(NodeImageMode::Stretch),
        GlobalZIndex(8),
        Node {
            position_type: PositionType::Absolute,
            left: px(0),
            top: px(0),
            width: percent(50.0),
            height: percent(100.0),
            ..default()
        },
    ));
    if let Some(game_logo) = &render.game_logo {
        commands.spawn((
            StateEntity,
            Name::new("GameLogo (Unity parity)"),
            ImageNode::new(game_logo.clone()),
            GlobalZIndex(10),
            Node {
                position_type: PositionType::Absolute,
                top: percent(4.9),
                left: px(0),
                width: percent(37.3),
                height: percent(35.1),
                aspect_ratio: Some(GAME_LOGO_ASPECT_RATIO),
                ..default()
            },
        ));
    } else {
        commands.spawn((
            StateEntity,
            Text::new("STREAM TOWN"),
            TextFont {
                font_size: FontSize::Px(64.0),
                ..default()
            },
            TextLayout::justify(Justify::Center),
            TextColor(Color::srgb(0.86, 0.95, 0.84)),
            Node {
                position_type: PositionType::Absolute,
                top: percent(12.0),
                left: percent(6.0),
                ..default()
            },
        ));
    }
    commands
        .spawn((
            StateEntity,
            Name::new("Shipping Main Menu buttons"),
            GlobalZIndex(10),
            Node {
                position_type: PositionType::Absolute,
                left: percent(12.65),
                top: percent(44.1),
                width: percent(12.1),
                height: percent(37.9),
                flex_direction: FlexDirection::Column,
                row_gap: px(20),
                ..default()
            },
        ))
        .with_children(|parent| {
            for action in [
                MainMenuAction::NewGame,
                MainMenuAction::LoadGame,
                MainMenuAction::Settings,
                MainMenuAction::Secrets,
                MainMenuAction::Credits,
                MainMenuAction::Quit,
            ] {
                parent
                    .spawn((
                        action,
                        Button,
                        authored_ui_image(
                            &render,
                            MAIN_MENU_TEXTURE_PATHS[0],
                            main_menu_texture(&render, MAIN_MENU_TEXTURE_PATHS[0]),
                        ),
                        Node {
                            width: percent(100.0),
                            flex_grow: 1.0,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new(main_menu_action_label(action)),
                            TextFont {
                                font_size: FontSize::Px(24.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                            Pickable::IGNORE,
                        ));
                    });
            }
        });
    commands.spawn((
        StateEntity,
        Name::new("Shipping Main Menu copyright"),
        Text::new("© 2022 Panda Belly"),
        TextFont {
            font_size: FontSize::Px(16.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::WHITE),
        GlobalZIndex(10),
        Node {
            position_type: PositionType::Absolute,
            left: percent(11.8),
            top: percent(89.4),
            width: percent(13.8),
            ..default()
        },
    ));
}

pub(crate) const fn main_menu_action_label(action: MainMenuAction) -> &'static str {
    match action {
        MainMenuAction::NewGame => "New Game",
        MainMenuAction::LoadGame => "Load Game",
        MainMenuAction::Settings => "Settings",
        MainMenuAction::Secrets => "Secrets",
        MainMenuAction::Credits => "Credits",
        MainMenuAction::Quit => "Quit",
    }
}

pub(crate) fn loading_icon_rotation(
    content: &ContentCatalog,
) -> Option<&stream_town_domain::RotatingNodeDef> {
    content
        .archetypes
        .values()
        .find(|archetype| {
            archetype
                .source_path
                .ends_with(LOADING_SCREEN_PREFAB_SUFFIX)
        })?
        .rotating_nodes
        .iter()
        .find(|rotating| rotating.hierarchy_path == LOADING_ICON_HIERARCHY_PATH)
}

pub(crate) fn spawn_loading_screen(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    render: Option<Res<RenderAssets>>,
    existing: Query<(), With<LoadingScreenEntity>>,
    loading_cameras: Query<Entity, With<LoadingUiCamera>>,
) {
    // The overlay is deliberately persistent across Boot/MainMenu ->
    // WorldLoading. Recreating already-decoded ImageNodes still costs one render
    // extraction frame; despawning the old scene in that frame exposed the
    // camera clear colour as a black flash.
    if !existing.is_empty() {
        return;
    }
    // The town camera changes projection, HDR, tonemapping, and post-process
    // pipelines between MainMenu, WorldLoading, and InGame. Rendering the
    // loading UI through that camera lets a first-use pipeline rebuild blank
    // the entire window even though the overlay entities remain alive. Keep a
    // separate, stable UI-only camera above the world for the complete loading
    // operation so scene-camera compilation can never interrupt the cover. It
    // is retired with the cover; leaving a no-clear camera on the swapchain can
    // preserve its last SDR frame over the HDR world.
    let loading_camera = loading_cameras.iter().next().unwrap_or_else(|| {
        commands
            .spawn((
                LoadingUiCamera,
                Name::new("Persistent loading UI camera"),
                Camera2d,
                Camera {
                    order: 1_000,
                    clear_color: bevy::camera::ClearColorConfig::None,
                    ..default()
                },
                Msaa::Off,
            ))
            .id()
    });
    // This solid layer is immediately renderable even before the authored PNG
    // finishes its own first asset-server round trip. It prevents the partially
    // instantiated destination scene from showing through during boot.
    commands.spawn((
        LoadingScreenEntity,
        UiTargetCamera(loading_camera),
        BackgroundColor(Color::srgb(0.025, 0.04, 0.055)),
        GlobalZIndex(1_000),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100.0),
            height: percent(100.0),
            ..default()
        },
    ));
    if let Some(background) = render
        .as_deref()
        .and_then(|render| render.loading_screen.as_ref())
    {
        commands.spawn((
            LoadingScreenEntity,
            UiTargetCamera(loading_camera),
            ImageNode::new(background.clone()).with_mode(NodeImageMode::Stretch),
            GlobalZIndex(1_001),
            Node {
                position_type: PositionType::Absolute,
                width: percent(100.0),
                height: percent(100.0),
                ..default()
            },
        ));
    }
    if let Some(overlay) = render
        .as_deref()
        .and_then(|render| render.loading_overlay.as_ref())
    {
        commands.spawn((
            LoadingScreenEntity,
            UiTargetCamera(loading_camera),
            ImageNode::new(overlay.clone()).with_mode(NodeImageMode::Stretch),
            GlobalZIndex(1_002),
            Node {
                position_type: PositionType::Absolute,
                width: percent(100.0),
                height: percent(100.0),
                ..default()
            },
        ));
    }
    commands.spawn((
        LoadingScreenEntity,
        UiTargetCamera(loading_camera),
        LoadingStatusText,
        Text::new("Preparing town..."),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::srgb(0.93, 0.91, 0.78)),
        GlobalZIndex(1_004),
        Node {
            position_type: PositionType::Absolute,
            left: percent(34.0),
            bottom: percent(20.0),
            width: percent(32.0),
            ..default()
        },
    ));
    commands.spawn((
        LoadingScreenEntity,
        UiTargetCamera(loading_camera),
        LoadingSubstatusText,
        Text::new("Discovering dependencies..."),
        TextFont {
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::srgb(0.75, 0.76, 0.78)),
        GlobalZIndex(1_004),
        Node {
            position_type: PositionType::Absolute,
            left: percent(30.0),
            bottom: percent(17.0),
            width: percent(40.0),
            ..default()
        },
    ));
    commands.spawn((
        LoadingScreenEntity,
        UiTargetCamera(loading_camera),
        LoadingPercentText,
        Text::new("0%"),
        TextFont {
            font_size: FontSize::Px(16.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::WHITE),
        GlobalZIndex(1_004),
        Node {
            position_type: PositionType::Absolute,
            left: percent(47.0),
            bottom: percent(15.0),
            width: percent(6.0),
            ..default()
        },
    ));
    commands
        .spawn((
            LoadingScreenEntity,
            UiTargetCamera(loading_camera),
            GlobalZIndex(1_004),
            Node {
                position_type: PositionType::Absolute,
                left: percent(30.0),
                bottom: percent(13.0),
                width: percent(40.0),
                height: px(10),
                padding: UiRect::all(px(1)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.03, 0.03, 0.04, 0.85)),
        ))
        .with_children(|parent| {
            parent.spawn((
                LoadingScreenEntity,
                LoadingProgressFill,
                loading_progress_accessibility_node(),
                Node {
                    width: percent(0.0),
                    height: percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
            ));
        });
    let tooltip = &content.0.loading_screen.tooltips[usize::try_from(config.0.world.seed)
        .unwrap_or_default()
        % content.0.loading_screen.tooltips.len()];
    commands.spawn((
        LoadingScreenEntity,
        UiTargetCamera(loading_camera),
        Text::new(tooltip.clone()),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::srgb(0.82, 0.82, 0.82)),
        GlobalZIndex(1_004),
        Node {
            position_type: PositionType::Absolute,
            left: percent(25.0),
            bottom: percent(4.0),
            width: percent(50.0),
            ..default()
        },
    ));
    let Some(icon) = render
        .as_deref()
        .and_then(|render| render.loading_icon.as_ref())
    else {
        return;
    };
    let rotation = loading_icon_rotation(&content.0)
        .expect("validated content contains the authored loading-icon rotation");
    commands.spawn((
        LoadingScreenEntity,
        UiTargetCamera(loading_camera),
        LoadingIconSpinner {
            radians_per_second: rotation.degrees_per_second.to_radians(),
        },
        ImageNode::new(icon.clone()).with_color(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
        UiTransform::IDENTITY,
        GlobalZIndex(1_004),
        Node {
            position_type: PositionType::Absolute,
            left: percent(45.0),
            bottom: percent(8.0),
            width: percent(10.0),
            aspect_ratio: Some(1.0),
            ..default()
        },
    ));
}

pub(crate) fn world_preload_paths(content: &ContentCatalog, asset_root: &Path) -> BTreeSet<String> {
    let mut paths = content
        .archetypes
        .values()
        .flat_map(|archetype| archetype.scenes.iter())
        .map(|scene| (scene.asset_path.as_str(), scene.asset_path.clone()))
        .chain(
            content
                .foliage
                .iter()
                .flat_map(|layer| &layer.variants)
                .map(|variant| (variant.asset_path.as_str(), variant.asset_path.clone())),
        )
        .filter(|(path, _)| converted_asset_exists(asset_root, path))
        .map(|(_, path)| path)
        .collect::<BTreeSet<_>>();
    for archetype in content.archetypes.values() {
        if let Some(scene) = runtime_archetype_scene(archetype)
            && converted_asset_exists(asset_root, &scene.asset_path)
        {
            paths.insert(scene.asset_path);
        }
    }
    if converted_asset_exists(asset_root, SEAGULL_MODEL_PATH) {
        paths.insert(SEAGULL_MODEL_PATH.to_owned());
    }
    paths
}

pub(crate) fn world_render_preload_handles(
    server: &AssetServer,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
) -> Vec<UntypedHandle> {
    let mut handles = Vec::new();
    for handle in [
        render.loading_screen.as_ref(),
        render.loading_overlay.as_ref(),
        render.loading_icon.as_ref(),
    ]
    .into_iter()
    .flatten()
    {
        handles.push(handle.clone().untyped());
    }
    for handle in [render.ui_font.as_ref(), render.ui_display_font.as_ref()]
        .into_iter()
        .flatten()
    {
        handles.push(handle.clone().untyped());
    }
    for map in [
        &render.top_bar_textures,
        &render.selection_panel_textures,
        &render.vote_textures,
        &render.objective_textures,
        &render.current_event_textures,
        &render.main_menu_textures,
    ] {
        handles.extend(map.values().cloned().map(Handle::untyped));
    }
    // Programmatically created extended materials hold AssetServer image
    // handles, but are not themselves loaded assets with a dependency graph.
    // Explicitly include their converted source textures in readiness.
    let texture_ids = presentation
        .materials
        .values()
        .flat_map(|material| material.textures.values())
        .collect::<BTreeSet<_>>();
    handles.extend(texture_ids.into_iter().filter_map(|id| {
        presentation.textures.get(id).map(|texture| {
            let image: Handle<Image> = server.load(texture.asset_path.clone());
            image.untyped()
        })
    }));
    if let Some(mesh) = &render.fish_school_mesh {
        handles.push(mesh.clone().untyped());
    }
    handles
}

pub(crate) fn prepare_generated_world(
    world: GeneratedWorld,
    config: &GameConfig,
    prepare_render_geometry: bool,
) -> PreparedWorld {
    if !prepare_render_geometry {
        return PreparedWorld {
            world,
            terrain_mesh: None,
            terrain_collider: None,
            water_mesh: None,
        };
    }
    // Mesh topology and trimesh acceleration data are pure CPU work. Building
    // them in the generation task prevents a single long main-thread frame;
    // only insertion into Bevy's asset/ECS worlds remains on the app thread.
    let collision_mesh = generated_terrain_chunk_mesh(
        &world,
        config,
        0,
        0,
        world.navigation.width(),
        world.navigation.height(),
        1,
        false,
    );
    let terrain_collider = Collider::trimesh_from_mesh(&collision_mesh)
        .expect("generated terrain has indexed triangle geometry");
    let terrain_mesh = generated_terrain_mesh(&world, config);
    let water_mesh = generated_water_mesh(&world, config);
    PreparedWorld {
        world,
        terrain_mesh: Some(terrain_mesh),
        terrain_collider: Some(terrain_collider),
        water_mesh: Some(water_mesh),
    }
}

pub(crate) fn begin_world_loading(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    asset_root: Res<RuntimeAssetRoot>,
    asset_server: Option<Res<AssetServer>>,
    gpu_readiness: Res<GpuReadinessProbe>,
    presented_frames: Res<PresentedRenderFrames>,
) {
    let asset_handles = asset_server.as_deref().map_or_else(Vec::new, |server| {
        let mut handles = world_preload_paths(&content.0, &asset_root.0)
            .into_iter()
            .map(|path| {
                // WorldAssetRoot instantiates Scene(0), not the untyped GLB
                // container. Preloading the same sub-asset prevents a false
                // 100% followed by blue/black placeholder frames.
                let scene: Handle<bevy::world_serialization::WorldAsset> =
                    server.load(GltfAssetLabel::Scene(0).from_asset(path));
                scene.untyped()
            })
            .collect::<Vec<_>>();
        handles.extend(world_render_preload_handles(
            server,
            &presentation.0,
            &render,
        ));
        handles
    });
    let expected_images = asset_handles
        .iter()
        .filter_map(|handle| handle.id().try_typed::<Image>().ok())
        .collect();
    gpu_readiness.begin_world(
        expected_images,
        render.world_gpu_mesh_ids(),
        render.world_gpu_material_ids(),
    );
    let asset_count = asset_handles.len();
    let game_config = config.0.clone();
    let content = content.0.clone();
    let completed_generation = Arc::new(Mutex::new(BTreeSet::new()));
    let prepare_render_geometry = asset_server.is_some();
    let prepared_world = if prepare_render_geometry
        && let Some(pool) = AsyncComputeTaskPool::try_get()
    {
        let task_progress = Arc::clone(&completed_generation);
        commands.insert_resource(WorldGenerationTask {
            task: pool.spawn(async move {
                let world =
                    generate_world_with_content_observed(&game_config.world, &content, |stage| {
                        task_progress
                            .lock()
                            .unwrap_or_else(PoisonError::into_inner)
                            .insert(stage);
                    });
                prepare_generated_world(world, &game_config, true)
            }),
            completed: Arc::clone(&completed_generation),
        });
        None
    } else {
        // Headless tests and reduced apps have no asset pipeline to overlap,
        // so keep their deterministic generation synchronous and immediate.
        let world = generate_world_with_content_observed(&game_config.world, &content, |stage| {
            completed_generation
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .insert(stage);
        });
        Some(prepare_generated_world(
            world,
            &game_config,
            prepare_render_geometry,
        ))
    };
    let mut work = WorldLoadingWork::default();
    work.mark_cover_complete();
    work.world_assets = LoadingWork::count(0, asset_count);
    work.generation_completed.clone_from(
        &completed_generation
            .lock()
            .unwrap_or_else(PoisonError::into_inner),
    );
    let progress = work.progress();
    commands.insert_resource(WorldLoadingRuntime {
        phase: WorldLoadingPhase::Presenting,
        progress,
        status: "Preparing new town".to_owned(),
        substatus: format!(
            "0 / {asset_count} presentation assets; terrain {}",
            if prepared_world.is_some() {
                "ready"
            } else {
                "queued"
            }
        ),
        asset_handles,
        loaded_assets: 0,
        failed_assets: 0,
        prepared_world,
        spawn_runtime: None,
        presented_frames: Some(presented_frames.clone()),
        scene_ready_frames: 0,
        completion_remaining_seconds: 0.0,
        work,
    });
}

pub(crate) fn poll_world_loading(
    mut commands: Commands,
    mut loading: ResMut<WorldLoadingRuntime>,
    mut task: Option<ResMut<WorldGenerationTask>>,
    asset_server: Option<Res<AssetServer>>,
) {
    if loading.phase == WorldLoadingPhase::Presenting {
        loading.phase = WorldLoadingPhase::Loading;
    }
    if loading.phase != WorldLoadingPhase::Loading {
        return;
    }
    if let Some(task) = task.as_deref_mut() {
        loading.work.generation_completed.extend(
            task.completed
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .iter()
                .copied(),
        );
        if loading.prepared_world.is_none()
            && let Some(generated) = block_on(poll_once(&mut task.task))
        {
            loading.prepared_world = Some(generated);
            loading
                .work
                .generation_completed
                .extend(WorldGenerationStage::ALL);
            commands.remove_resource::<WorldGenerationTask>();
        }
    }

    let (loaded, failed) = loaded_asset_counts(asset_server.as_deref(), &loading.asset_handles);
    if failed > loading.failed_assets {
        error!(
            failed_assets = failed,
            "World preload contains failed assets"
        );
    }
    loading.loaded_assets = loaded;
    loading.failed_assets = failed;
    let total_assets = loading.asset_handles.len();
    let terrain_ready = loading.prepared_world.is_some();
    loading.work.world_assets = LoadingWork::count(loaded, total_assets);
    if terrain_ready {
        loading
            .work
            .generation_completed
            .extend(WorldGenerationStage::ALL);
    }
    loading.progress = loading.work.progress();
    let generation_completed = loading.work.generation_completed.len();
    let generation_total = WorldGenerationStage::ALL.len();
    loading.status = WorldGenerationStage::ALL
        .into_iter()
        .find(|stage| !loading.work.generation_completed.contains(stage))
        .map_or_else(
            || "Loading town presentation".to_owned(),
            |stage| {
                format!(
                    "Generating deterministic {}",
                    world_generation_stage_label(stage)
                )
            },
        );
    loading.substatus = if failed == 0 {
        format!(
            "{loaded} / {total_assets} presentation assets; generation {generation_completed}/{generation_total} stages"
        )
    } else {
        format!(
            "{loaded} loaded, {failed} unavailable; generation {generation_completed}/{generation_total} stages"
        )
    };
    if loaded == total_assets && failed == 0 && terrain_ready {
        loading.phase = WorldLoadingPhase::Spawning;
        "Building the town scene".clone_into(&mut loading.status);
        "Terrain and presentation assets are ready".clone_into(&mut loading.substatus);
    }
}

pub(crate) fn sync_gpu_world_expectations(
    gpu_readiness: Res<GpuReadinessProbe>,
    active_meshes: Query<&Mesh3d>,
    active_materials: ActiveMaterialHandles,
) {
    gpu_readiness.merge_world_content(
        active_meshes.iter().map(|mesh| mesh.0.id()),
        active_materials.ids(),
        None,
    );
}

pub(crate) fn advance_loading_phase(
    time: Res<Time>,
    mut loading: ResMut<WorldLoadingRuntime>,
    mut next_state: ResMut<NextState<GameState>>,
    world_instance_spawner: Option<Res<WorldInstanceSpawner>>,
    scene_roots: WorldAssetRootQuery,
) {
    if loading.phase == WorldLoadingPhase::Complete {
        let mut root_count = 0;
        let mut ready_count = 0;
        for (children, instance) in &scene_roots {
            root_count += 1;
            if world_asset_root_ready(children, instance, world_instance_spawner.as_deref()) {
                ready_count += 1;
            }
        }
        if asset_root_collection_ready(root_count, ready_count, world_instance_spawner.is_some()) {
            loading.scene_ready_frames = loading.scene_ready_frames.saturating_add(1);
        } else {
            loading.scene_ready_frames = 0;
        }
        loading.work.scene_roots = if root_count == 0 && world_instance_spawner.is_some() {
            LoadingWork::Pending
        } else {
            LoadingWork::count(ready_count, root_count)
        };
        loading.work.scene_stable_frames =
            LoadingWork::count(usize::from(loading.scene_ready_frames.min(5)), 5);
        loading.progress = loading.work.progress();
        if loading.scene_ready_frames < 5 {
            "Finalizing town presentation".clone_into(&mut loading.status);
            loading.substatus = format!("{ready_count} / {root_count} scene roots instantiated");
            return;
        }
        "Scene graph ready".clone_into(&mut loading.status);
        "Preparing materials, animation, and GPU resources".clone_into(&mut loading.substatus);
    }
    if advance_loading_runtime(&mut loading, time.delta_secs()) {
        next_state.set(GameState::InGame);
    }
}

pub(crate) fn advance_loading_runtime(
    loading: &mut WorldLoadingRuntime,
    delta_seconds: f32,
) -> bool {
    let delta_seconds = delta_seconds.max(0.0);
    match loading.phase {
        WorldLoadingPhase::Presenting
        | WorldLoadingPhase::Loading
        | WorldLoadingPhase::Spawning => false,
        WorldLoadingPhase::Complete => {
            loading.completion_remaining_seconds =
                (loading.completion_remaining_seconds - delta_seconds).max(0.0);
            loading.completion_remaining_seconds <= 0.0
        }
    }
}

pub(crate) fn sync_loading_screen_status(
    loading: Res<WorldLoadingRuntime>,
    mut status: Query<&mut Text, With<LoadingStatusText>>,
    mut substatus: LoadingSubstatusQuery,
    mut percent_text: LoadingPercentQuery,
    mut fills: Query<(&mut Node, &mut AccessibilityNode), With<LoadingProgressFill>>,
) {
    let progress_percent = loading.progress.clamp(0.0, 1.0) * 100.0;
    let display_percent = loading_display_percent(loading.progress);
    if let Ok(mut text) = status.single_mut() {
        (**text).clone_from(&loading.status);
    }
    if let Ok(mut text) = substatus.single_mut() {
        (**text).clone_from(&loading.substatus);
    }
    if let Ok(mut text) = percent_text.single_mut() {
        **text = loading_percent_text(display_percent);
    }
    if let Ok((mut fill, mut accessible)) = fills.single_mut() {
        set_loading_progress(&mut fill, &mut accessible, progress_percent);
    }
}

pub(crate) fn sync_boot_loading_screen(
    loading: Option<Res<MenuLoadingRuntime>>,
    world_cover: Option<Res<WorldLoadingCoverRuntime>>,
    mut status: Query<&mut Text, With<LoadingStatusText>>,
    mut substatus: LoadingSubstatusQuery,
    mut percent_text: LoadingPercentQuery,
    mut fills: Query<(&mut Node, &mut AccessibilityNode), With<LoadingProgressFill>>,
) {
    if world_cover.is_some() {
        return;
    }
    let Some(loading) = loading else {
        return;
    };
    let progress = boot_loading_display_progress(&loading);
    let progress_percent = progress.clamp(0.0, 1.0) * 100.0;
    let display_percent = loading_display_percent(progress);
    if let Ok(mut text) = status.single_mut() {
        (**text).clone_from(&loading.status);
    }
    if let Ok(mut text) = substatus.single_mut() {
        (**text).clone_from(&loading.substatus);
    }
    if let Ok(mut text) = percent_text.single_mut() {
        **text = loading_percent_text(display_percent);
    }
    if let Ok((mut fill, mut accessible)) = fills.single_mut() {
        set_loading_progress(&mut fill, &mut accessible, progress_percent);
    }
}

pub(crate) fn loading_progress_accessibility_node() -> AccessibilityNode {
    let mut node = AccessibleNode::new(Role::ProgressIndicator);
    node.set_label("Loading progress");
    node.set_min_numeric_value(0.0);
    node.set_max_numeric_value(100.0);
    node.set_numeric_value(0.0);
    node.set_value("0%");
    AccessibilityNode(node)
}

pub(crate) fn set_loading_progress(
    fill: &mut Node,
    accessible: &mut AccessibilityNode,
    progress_percent: f32,
) {
    let progress_percent = progress_percent.clamp(0.0, 100.0);
    fill.width = percent(progress_percent);
    accessible.set_numeric_value(f64::from(progress_percent));
    accessible.set_value(
        loading_percent_text(loading_display_percent(progress_percent / 100.0)).into_boxed_str(),
    );
}

pub(crate) fn boot_loading_display_progress(loading: &MenuLoadingRuntime) -> f32 {
    if loading.destination != BootDestination::WorldLoading {
        return loading.progress;
    }
    let mut work = WorldLoadingWork {
        cover_entities: LoadingWork::boolean(true),
        cover_artwork: LoadingWork::count(loading.loaded_assets, loading.asset_handles.len()),
        cover_presented_frames: LoadingWork::count(
            usize::from(loading.ready_presented_frames.min(3)),
            3,
        ),
        ..default()
    };
    // Boot has no render-world probe. The same assets are included again in the
    // authoritative world GPU leaves after the state handoff.
    if loading.asset_handles.is_empty() {
        work.cover_gpu_images = LoadingWork::boolean(true);
        work.cover_pipelines = LoadingWork::boolean(true);
    }
    work.progress()
}

pub(crate) fn world_asset_root_ready(
    children: Option<&Children>,
    instance: Option<&WorldInstance>,
    spawner: Option<&WorldInstanceSpawner>,
) -> bool {
    if let Some(spawner) = spawner {
        return instance.is_some_and(|instance| spawner.instance_is_ready(**instance));
    }
    // Reduced/headless apps do not install the world-instance spawner. Keep
    // their deterministic fallback while the shipping renderer uses the
    // authoritative readiness signal above.
    children.is_some_and(|children| !children.is_empty())
}

pub(crate) fn asset_root_collection_ready(
    root_count: usize,
    ready_count: usize,
    world_instance_spawner_available: bool,
) -> bool {
    root_count == ready_count && (root_count > 0 || !world_instance_spawner_available)
}

pub(crate) fn rendered_frames_since_ready(
    starting_frame: &mut Option<u64>,
    ready: bool,
    presented_render_frame: u64,
) -> u64 {
    if !ready {
        *starting_frame = None;
        return 0;
    }
    let starting_frame = *starting_frame.get_or_insert(presented_render_frame);
    presented_render_frame.saturating_sub(starting_frame)
}

pub(crate) fn finish_menu_reveal(
    mut commands: Commands,
    presented_frames: Res<PresentedRenderFrames>,
    world_instance_spawner: Option<Res<WorldInstanceSpawner>>,
    world_cover: Option<Res<WorldLoadingCoverRuntime>>,
    mut reveal: Option<ResMut<MenuRevealRuntime>>,
    mut loading: Option<ResMut<MenuLoadingRuntime>>,
    menu_spawn: Option<Res<MainMenuSpawnRuntime>>,
    mut loading_entities: LoadingCoverEntityQuery,
    scene_roots: MenuWorldAssetRootQuery,
) {
    if world_cover.is_some() {
        return;
    }
    let Some(reveal) = reveal.as_deref_mut() else {
        return;
    };
    reveal.fallback_updates = reveal.fallback_updates.saturating_add(1);
    let presented_render_frame = if presented_frames.render_schedule_available {
        presented_frames.current()
    } else {
        u64::from(reveal.fallback_updates)
    };
    let rendered_frames = presented_render_frame.saturating_sub(reveal.starting_render_frame);
    let mut root_count = 0;
    let mut ready_count = 0;
    for (children, instance) in &scene_roots {
        root_count += 1;
        if world_asset_root_ready(children, instance, world_instance_spawner.as_deref()) {
            ready_count += 1;
        }
    }
    let construction_work = menu_spawn.as_deref().map_or_else(
        || LoadingWork::boolean(true),
        |spawn| LoadingWork::count(spawn.completed, spawn.total),
    );
    let construction_ready = menu_spawn
        .as_deref()
        .is_none_or(|spawn| spawn.phase == MainMenuSpawnPhase::Complete);
    let roots_ready = construction_ready
        && asset_root_collection_ready(root_count, ready_count, world_instance_spawner.is_some());
    if roots_ready {
        reveal.scene_ready_frames = reveal.scene_ready_frames.saturating_add(1);
    } else {
        reveal.scene_ready_frames = 0;
    }
    let ready_rendered_frames = rendered_frames_since_ready(
        &mut reveal.ready_starting_render_frame,
        roots_ready,
        presented_render_frame,
    );
    if let Some(loading) = loading.as_deref_mut() {
        loading.progress = main_menu_loading_progress(
            loading.loaded_assets,
            loading.asset_handles.len(),
            construction_work,
            if root_count == 0 && world_instance_spawner.is_some() {
                LoadingWork::Pending
            } else {
                LoadingWork::count(ready_count, root_count)
            },
            LoadingWork::count(
                usize::try_from(ready_rendered_frames.min(12))
                    .expect("menu reveal frame count fits usize"),
                12,
            ),
            LoadingWork::count(usize::from(reveal.scene_ready_frames.min(8)), 8),
        );
        "Building main menu scene".clone_into(&mut loading.status);
        let (constructed, construction_total) = menu_spawn
            .as_deref()
            .map_or((0, 0), |spawn| (spawn.completed, spawn.total));
        loading.substatus = format!(
            "{constructed} / {construction_total} scene entities; {ready_count} / {root_count} scene roots instantiated"
        );
    }
    if ready_rendered_frames < 12 || reveal.scene_ready_frames < 8 {
        return;
    }
    for (entity, visibility) in &mut loading_entities {
        if let Some(mut visibility) = visibility {
            *visibility = Visibility::Hidden;
        }
        commands.entity(entity).try_despawn();
    }
    info!(
        scene_roots = root_count,
        rendered_frames,
        ready_rendered_frames,
        elapsed_seconds = reveal.started_at.elapsed().as_secs_f64(),
        "Stream Town main-menu scene reveal complete"
    );
    commands.remove_resource::<MenuLoadingRuntime>();
    commands.remove_resource::<MenuRevealRuntime>();
}

pub(crate) fn begin_world_reveal(
    mut commands: Commands,
    presented_frames: Res<PresentedRenderFrames>,
    simulation: Res<SimulationRuntime>,
    session_stats: Res<SessionStats>,
) {
    commands.remove_resource::<GameplayReady>();
    commands.insert_resource(WorldRevealRuntime {
        started_at: Instant::now(),
        starting_render_frame: presented_frames.current(),
        ready_starting_render_frame: None,
        completion_starting_render_frame: None,
        fallback_updates: 0,
        ready_frames: 0,
        simulation_elapsed_seconds: simulation.0.elapsed_seconds,
        session_elapsed_seconds: session_stats.elapsed_seconds,
    });
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn finish_world_reveal(
    mut commands: Commands,
    presented_frames: Res<PresentedRenderFrames>,
    gpu_readiness: Res<GpuReadinessProbe>,
    world_instance_spawner: Option<Res<WorldInstanceSpawner>>,
    mut reveal: Option<ResMut<WorldRevealRuntime>>,
    mut loading: Option<ResMut<WorldLoadingRuntime>>,
    mut loading_entities: LoadingCoverEntityQuery,
    scene_roots: WorldAssetRootQuery,
    readiness: WorldRevealReadinessQueries,
    active_meshes: Query<&Mesh3d>,
    active_materials: ActiveMaterialHandles,
    simulation: Res<SimulationRuntime>,
    session_stats: Res<SessionStats>,
) {
    let Some(reveal) = reveal.as_deref_mut() else {
        return;
    };
    reveal.fallback_updates = reveal.fallback_updates.saturating_add(1);
    let presented_render_frame = if presented_frames.render_schedule_available {
        presented_frames.current()
    } else {
        u64::from(reveal.fallback_updates)
    };
    let rendered_frames = presented_render_frame.saturating_sub(reveal.starting_render_frame);

    let mut root_count = 0;
    let mut ready_roots = 0;
    for (children, instance) in &scene_roots {
        root_count += 1;
        if world_asset_root_ready(children, instance, world_instance_spawner.as_deref()) {
            ready_roots += 1;
        }
    }

    let mut authored_meshes = 0;
    let mut applied_meshes = 0;
    let mut animated_player_meshes = 0;
    let mut ready_character_receivers = 0;
    for (entity, applied) in &readiness.mesh_overrides {
        let mut ancestor = entity;
        let mut has_authored_spec = false;
        let mut has_animated_player_rig = false;
        for _ in 0..64 {
            has_animated_player_rig |= readiness.animated_player_rigs.contains(ancestor);
            if readiness.material_specs.contains(ancestor) {
                has_authored_spec = true;
                break;
            }
            let Ok(parent) = readiness.parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
        if has_authored_spec {
            authored_meshes += 1;
            if applied.is_some() {
                applied_meshes += 1;
            }
        }
        if has_animated_player_rig {
            animated_player_meshes += 1;
            if readiness.character_receivers.contains(entity)
                && !readiness.suppressed_receivers.contains(entity)
            {
                ready_character_receivers += 1;
            }
        }
    }

    let converted_animation_count = readiness.converted_animation_roots.iter().count();
    let converted_animation_ready = readiness.converted_animation_roots.iter().flatten().count();
    let native_animation_count = readiness.native_animation_roots.iter().count();
    let native_animation_ready = readiness
        .native_animation_roots
        .iter()
        .filter(|root| {
            readiness
                .native_animation_drivers
                .iter()
                .any(|driver| driver.actor_root == *root)
        })
        .count();
    let animation_count = converted_animation_count + native_animation_count;
    let ready_animation_count = converted_animation_ready + native_animation_ready;

    gpu_readiness.merge_world_content(
        active_meshes.iter().map(|mesh| mesh.0.id()),
        active_materials.ids(),
        None,
    );
    let gpu = gpu_readiness.snapshot();
    let gpu_ready = !presented_frames.render_schedule_available || gpu.is_ready();
    let roots_ready =
        asset_root_collection_ready(root_count, ready_roots, world_instance_spawner.is_some());
    let materials_ready = authored_meshes == applied_meshes;
    let character_receivers_ready = animated_player_meshes == ready_character_receivers;
    let animation_setup_ready = animation_count == ready_animation_count;
    let world_ready = roots_ready
        && materials_ready
        && animation_setup_ready
        && character_receivers_ready
        && gpu_ready;
    if world_ready {
        reveal.ready_frames = reveal.ready_frames.saturating_add(1);
    } else {
        reveal.ready_frames = 0;
    }
    let ready_rendered_frames = rendered_frames_since_ready(
        &mut reveal.ready_starting_render_frame,
        world_ready,
        presented_render_frame,
    );
    let stable_frames = ready_rendered_frames.min(u64::from(reveal.ready_frames));
    if let Some(loading) = loading.as_deref_mut() {
        loading.work.scene_roots = if root_count == 0 && world_instance_spawner.is_some() {
            LoadingWork::Pending
        } else {
            LoadingWork::count(ready_roots, root_count)
        };
        if roots_ready {
            loading.work.material_overrides = LoadingWork::count(applied_meshes, authored_meshes);
            loading.work.animation_receivers =
                LoadingWork::count(ready_animation_count, animation_count);
            loading.work.lighting_receivers =
                LoadingWork::count(ready_character_receivers, animated_player_meshes);
        } else {
            loading.work.material_overrides = LoadingWork::Pending;
            loading.work.animation_receivers = LoadingWork::Pending;
            loading.work.lighting_receivers = LoadingWork::Pending;
        }
        if roots_ready
            && materials_ready
            && animation_setup_ready
            && character_receivers_ready
            && presented_frames.render_schedule_available
            && gpu.epoch != 0
        {
            loading.work.gpu_images = LoadingWork::count(gpu.ready_images, gpu.expected_images);
            loading.work.gpu_meshes = LoadingWork::count(gpu.ready_meshes, gpu.expected_meshes);
            loading.work.gpu_materials =
                LoadingWork::count(gpu.ready_materials, gpu.expected_materials);
            loading.work.gpu_pipelines =
                LoadingWork::boolean(gpu.pending_pipelines == 0 && gpu.failed_pipelines == 0);
            loading.work.selection_draw =
                LoadingWork::boolean(!gpu.selection_expected || gpu.selection_draw_ready);
        } else if roots_ready
            && materials_ready
            && animation_setup_ready
            && character_receivers_ready
            && !presented_frames.render_schedule_available
        {
            loading.work.gpu_images = LoadingWork::boolean(true);
            loading.work.gpu_meshes = LoadingWork::boolean(true);
            loading.work.gpu_materials = LoadingWork::boolean(true);
            loading.work.gpu_pipelines = LoadingWork::boolean(true);
            loading.work.selection_draw = LoadingWork::boolean(true);
        } else {
            loading.work.gpu_images = LoadingWork::Pending;
            loading.work.gpu_meshes = LoadingWork::Pending;
            loading.work.gpu_materials = LoadingWork::Pending;
            loading.work.gpu_pipelines = LoadingWork::Pending;
            loading.work.selection_draw = LoadingWork::Pending;
        }
        loading.work.gpu_stable_frames = if world_ready {
            LoadingWork::count(
                usize::try_from(stable_frames.min(WORLD_REVEAL_GPU_STABLE_FRAMES))
                    .expect("GPU stable frame count fits usize"),
                usize::try_from(WORLD_REVEAL_GPU_STABLE_FRAMES)
                    .expect("GPU stable frame total fits usize"),
            )
        } else {
            LoadingWork::Pending
        };
        loading.progress = loading.work.progress();
        let status = if !roots_ready {
            "Instantiating town scenes"
        } else if !materials_ready || !animation_setup_ready || !character_receivers_ready {
            "Finalizing models and animation"
        } else if gpu.ready_images != gpu.expected_images
            || gpu.ready_meshes != gpu.expected_meshes
            || gpu.ready_materials != gpu.expected_materials
        {
            "Uploading town presentation"
        } else if gpu.pending_pipelines > 0 || gpu.failed_pipelines > 0 {
            "Compiling render pipelines"
        } else {
            "Validating first rendered frames"
        };
        status.clone_into(&mut loading.status);
        loading.substatus = format!(
            "{ready_roots}/{root_count} scenes; animations {ready_animation_count}/{animation_count}; lighting {ready_character_receivers}/{animated_player_meshes}; GPU images {}/{}, meshes {}/{}, materials {}/{}; {} pipelines",
            gpu.ready_images,
            gpu.expected_images,
            gpu.ready_meshes,
            gpu.expected_meshes,
            gpu.ready_materials,
            gpu.expected_materials,
            gpu.pending_pipelines,
        );
    }

    // Require several frames after the render world has explicitly confirmed
    // every active mesh/material, all preloaded images, and an idle pipeline
    // cache. The selection now shares the already-confirmed terrain material
    // pipeline, so it requires no independent draw instance. This is a
    // readiness proof, not a blind delay.
    if ready_rendered_frames < WORLD_REVEAL_GPU_STABLE_FRAMES
        || u64::from(reveal.ready_frames) < WORLD_REVEAL_GPU_STABLE_FRAMES
    {
        reveal.completion_starting_render_frame = None;
        return;
    }
    let completion_starting_render_frame = *reveal
        .completion_starting_render_frame
        .get_or_insert(presented_render_frame);
    if let Some(loading) = loading.as_deref_mut() {
        debug_assert!(
            (loading.work.progress() - 1.0).abs() < f32::EPSILON,
            "the reveal gate and recursive loading tree must agree"
        );
        loading.progress = loading.work.progress();
        "Town ready".clone_into(&mut loading.status);
        "All scenes, materials, animation, and GPU pipelines are ready"
            .clone_into(&mut loading.substatus);
    }
    // Present one truthful 100% frame. The cover can only retire after the
    // renderer has actually shown that completed state.
    if presented_render_frame.saturating_sub(completion_starting_render_frame) < 1 {
        return;
    }
    debug_assert!(
        (simulation.0.elapsed_seconds - reveal.simulation_elapsed_seconds).abs() < f64::EPSILON,
        "simulation time advanced behind the loading overlay"
    );
    debug_assert!(
        (session_stats.elapsed_seconds - reveal.session_elapsed_seconds).abs() < f64::EPSILON,
        "session time advanced behind the loading overlay"
    );
    for (entity, visibility) in &mut loading_entities {
        if let Some(mut visibility) = visibility {
            *visibility = Visibility::Hidden;
        }
        commands.entity(entity).try_despawn();
    }
    info!(
        scene_roots = root_count,
        authored_meshes,
        animations = animation_count,
        ready_animations = ready_animation_count,
        animated_player_meshes,
        ready_character_receivers,
        gpu_images = gpu.ready_images,
        gpu_meshes = gpu.ready_meshes,
        gpu_materials = gpu.ready_materials,
        pending_pipelines = gpu.pending_pipelines,
        failed_pipelines = gpu.failed_pipelines,
        selection_draw_ready = gpu.selection_draw_ready,
        rendered_frames,
        ready_rendered_frames,
        elapsed_seconds = reveal.started_at.elapsed().as_secs_f64(),
        "Stream Town world reveal complete"
    );
    gpu_readiness.clear();
    commands.remove_resource::<WorldLoadingRuntime>();
    commands.remove_resource::<WorldRevealRuntime>();
}

pub(crate) fn world_reveal_finished(
    reveal: Option<Res<WorldRevealRuntime>>,
    loading: Option<Res<WorldLoadingRuntime>>,
) -> bool {
    reveal.is_none() && loading.is_none()
}

pub(crate) fn enforce_loading_overlay_retired(
    mut commands: Commands,
    reveal: Option<Res<WorldRevealRuntime>>,
    automatic_resume: Res<AutomaticResumeRuntime>,
    mut loading_entities: Query<(Entity, &mut Visibility), With<LoadingScreenEntity>>,
    loading_cameras: Query<Entity, With<LoadingUiCamera>>,
    mut retirement_verified: Local<bool>,
) {
    if reveal.is_some() || automatic_resume.pending() {
        return;
    }
    let mut survivors = 0;
    for (entity, mut visibility) in &mut loading_entities {
        survivors += 1;
        *visibility = Visibility::Hidden;
        commands.entity(entity).try_despawn();
    }
    for entity in &loading_cameras {
        commands.entity(entity).try_despawn();
    }
    if survivors > 0 {
        error!(
            survivors,
            "Loading overlay survived its reveal frame; forcing retirement"
        );
    } else if !*retirement_verified {
        *retirement_verified = true;
        info!("Stream Town loading overlay retirement verified");
    }
}

pub(crate) fn confirm_gameplay_ready(
    mut commands: Commands,
    ready: Option<Res<GameplayReady>>,
    automatic_resume: Res<AutomaticResumeRuntime>,
    reveal: Option<Res<WorldRevealRuntime>>,
    loading: Option<Res<WorldLoadingRuntime>>,
    loading_entities: LoadingRetirementQuery,
    simulation: Option<Res<SimulationRuntime>>,
    stats: Option<Res<SessionStats>>,
) {
    if automatic_resume.pending()
        || ready.is_some()
        || reveal.is_some()
        || loading.is_some()
        || !loading_entities.is_empty()
    {
        return;
    }
    commands.insert_resource(GameplayReady);
    info!(
        simulation_elapsed_seconds = simulation
            .as_ref()
            .map_or(0.0, |runtime| runtime.0.elapsed_seconds),
        session_elapsed_seconds = stats.as_ref().map_or(0.0, |stats| stats.elapsed_seconds),
        "Stream Town gameplay and world audio unpaused after loading overlay retirement"
    );
}

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

pub(crate) fn spawn_vote_track(
    parent: &mut ChildSpawnerCommands,
    render: &RenderAssets,
    fill_kind: VoteFillKind,
    unfilled_path: &str,
    filled_path: &str,
    node: Node,
) {
    let mut track = parent.spawn((
        authored_ui_image(render, unfilled_path, vote_texture(render, unfilled_path)),
        node,
    ));
    if fill_kind == VoteFillKind::TechnologyTimer {
        track.insert(TechnologyVoteTimerTrack);
    }
    track.with_children(|track| {
        track.spawn((
            fill_kind,
            authored_ui_image(render, filled_path, vote_texture(render, filled_path)),
            Node {
                width: percent(100.0),
                height: percent(100.0),
                overflow: Overflow::clip_x(),
                ..default()
            },
        ));
    });
}

pub(crate) fn spawn_technology_vote_option_row(
    panel: &mut ChildSpawnerCommands,
    render: &RenderAssets,
    index: u8,
) {
    let top =
        TECHNOLOGY_VOTE_FIRST_ROW_TOP + f32::from(index) * TECHNOLOGY_VOTE_SINGLE_LINE_ADVANCE;
    panel
        .spawn((
            TechnologyVoteOptionRow(index),
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                top: px(top),
                left: px(32),
                right: px(32),
                height: px(92),
                ..default()
            },
        ))
        .with_children(|row| {
            spawn_vote_track(
                row,
                render,
                VoteFillKind::TechnologyOption(index),
                VOTE_TEXTURE_PATHS[6],
                VOTE_TEXTURE_PATHS[5],
                Node {
                    position_type: PositionType::Absolute,
                    top: px(0),
                    left: px(0),
                    right: px(0),
                    height: px(28),
                    ..default()
                },
            );
            // A flex container centers the label against the bar's real
            // bounds. This remains correct when the display font changes.
            row.spawn((
                TechnologyVoteTitleBar,
                ZIndex(2),
                Pickable::IGNORE,
                Node {
                    position_type: PositionType::Absolute,
                    top: px(TECHNOLOGY_VOTE_LABEL_OFFSET_Y),
                    left: px(6),
                    right: px(6),
                    height: px(28),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::clip(),
                    ..default()
                },
            ))
            .with_children(|title_bar| {
                title_bar.spawn((
                    VoteTextKind::TechnologyOptionTitle(index),
                    UiDisplayFont,
                    Text::new(format!("{}. Technology", index + 1)),
                    TextFont {
                        font_size: FontSize::Px(13.5),
                        ..default()
                    },
                    TextLayout::new(Justify::Center, LineBreak::NoWrap),
                    TextColor(Color::srgb(0.98, 0.94, 0.78)),
                    TextShadow {
                        offset: Vec2::splat(1.0),
                        color: Color::linear_rgba(0.0, 0.0, 0.0, 0.95),
                    },
                    Pickable::IGNORE,
                    Node {
                        width: percent(100),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                ));
            });
            row.spawn((
                TechnologyVoteIcon(index),
                ImageNode::default(),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(2),
                    top: px(TECHNOLOGY_VOTE_ICON_TOP),
                    width: px(30),
                    height: px(30),
                    ..default()
                },
            ));
            row.spawn((
                TechnologyVoteDepthBadge(index),
                Visibility::Hidden,
                ZIndex(3),
                Pickable::IGNORE,
                BackgroundColor(Color::srgb(0.96, 0.76, 0.20)),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(4),
                    left: px(-78),
                    width: px(80),
                    height: px(20),
                    border_radius: BorderRadius::all(px(10)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::clip(),
                    ..default()
                },
            ))
            .with_children(|badge| {
                badge.spawn((
                    VoteTextKind::TechnologyOptionDepthTag(index),
                    UiDisplayFont,
                    Text::new("Fundamental"),
                    TextFont {
                        font_size: FontSize::Px(9.0),
                        ..default()
                    },
                    TextLayout::new(Justify::Center, LineBreak::NoWrap),
                    TextColor(Color::srgb(0.06, 0.08, 0.16)),
                    Pickable::IGNORE,
                ));
            });
            row.spawn((
                VoteTextKind::TechnologyOptionRequirements(index),
                Text::new("Requirements"),
                TextFont {
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextColor(Color::srgb(0.91, 0.89, 0.81)),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(36),
                    left: px(38),
                    right: px(3),
                    height: px(56),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
        });
}

pub(crate) fn technology_vote_row_advance(requirement_lines: usize) -> f32 {
    let extra_lines = u16::try_from(requirement_lines.saturating_sub(1)).unwrap_or(u16::MAX);
    TECHNOLOGY_VOTE_SINGLE_LINE_ADVANCE
        + f32::from(extra_lines) * TECHNOLOGY_VOTE_EXTRA_LINE_ADVANCE
}

pub(crate) fn spawn_vote_panels(commands: &mut Commands, render: &RenderAssets) {
    commands
        .spawn((
            WorldEntity,
            VotePanelKind::Technology,
            Name::new("Technology voting menu"),
            authored_ui_image(
                render,
                VOTE_TEXTURE_PATHS[0],
                vote_texture(render, VOTE_TEXTURE_PATHS[0]),
            ),
            GlobalZIndex(24),
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                top: px(96),
                right: px(18),
                width: px(260),
                height: px(TECHNOLOGY_VOTE_PANEL_HEIGHT),
                ..default()
            },
        ))
        .with_children(|panel| {
            panel.spawn((
                VoteTextKind::TechnologyTitle,
                UiDisplayFont,
                Text::new("TECHNOLOGY VOTE"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.97, 0.88, 0.58)),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(TECHNOLOGY_VOTE_TITLE_TOP),
                    left: px(32),
                    right: px(32),
                    ..default()
                },
            ));
            for index in 0..u8::try_from(TECHNOLOGY_VOTE_OPTION_COUNT)
                .expect("technology vote option count fits u8")
            {
                spawn_technology_vote_option_row(panel, render, index);
            }
            panel
                .spawn((
                    TechnologyVoteTimerGroup,
                    Pickable::IGNORE,
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(0),
                        top: px(TECHNOLOGY_VOTE_TIMER_GROUP_TOP),
                        right: px(0),
                        height: px(24),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                ))
                .with_children(|timer_group| {
                    timer_group.spawn((
                        TechnologyVoteTimerIcon,
                        ImageNode::new(vote_texture(render, VOTE_TEXTURE_PATHS[3])),
                        Pickable::IGNORE,
                        Node {
                            width: px(24),
                            height: px(24),
                            ..default()
                        },
                    ));
                    timer_group.spawn((
                        TechnologyVoteTimerGap,
                        Pickable::IGNORE,
                        Node {
                            width: px(TECHNOLOGY_VOTE_TIMER_GAP),
                            height: px(1),
                            ..default()
                        },
                    ));
                    timer_group
                        .spawn((
                            TechnologyVoteTimerBar,
                            Pickable::IGNORE,
                            Node {
                                width: px(TECHNOLOGY_VOTE_TIMER_BAR_WIDTH),
                                height: px(20),
                                ..default()
                            },
                        ))
                        .with_children(|timer_bar| {
                            spawn_vote_track(
                                timer_bar,
                                render,
                                VoteFillKind::TechnologyTimer,
                                VOTE_TEXTURE_PATHS[2],
                                VOTE_TEXTURE_PATHS[1],
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: px(0),
                                    top: px(0),
                                    right: px(0),
                                    height: px(20),
                                    ..default()
                                },
                            );
                            timer_bar
                                .spawn((
                                    TechnologyVoteTimerLabel,
                                    ZIndex(2),
                                    Pickable::IGNORE,
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: px(0),
                                        top: px(0),
                                        right: px(0),
                                        height: px(20),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        overflow: Overflow::clip(),
                                        ..default()
                                    },
                                ))
                                .with_children(|timer_label| {
                                    timer_label.spawn((
                                        VoteTextKind::TechnologyTimer,
                                        Text::new("00:30"),
                                        TextFont {
                                            font_size: FontSize::Px(16.0),
                                            ..default()
                                        },
                                        TextLayout::new(Justify::Center, LineBreak::NoWrap),
                                        TextColor(Color::srgb(0.06, 0.08, 0.16)),
                                        UiTransform::from_xy(
                                            px(0),
                                            px(TECHNOLOGY_VOTE_TIMER_TEXT_OFFSET_Y),
                                        ),
                                        Pickable::IGNORE,
                                        Node {
                                            overflow: Overflow::clip(),
                                            ..default()
                                        },
                                    ));
                                });
                        });
                });
        });

    commands
        .spawn((
            WorldEntity,
            VotePanelKind::Ruler,
            Name::new("Ruler voting menu"),
            GlobalZIndex(24),
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                right: percent(1.2),
                bottom: percent(0.0),
                width: percent(14.6),
                height: percent(31.4),
                ..default()
            },
        ))
        .with_children(|panel| {
            // Unity extends the sliced panel 156 authored pixels beneath its
            // anchored content rect. Keeping that offset separate prevents the
            // title and options from being scaled along with the decorative frame.
            panel.spawn((
                authored_ui_image(
                    render,
                    VOTE_TEXTURE_PATHS[0],
                    vote_texture(render, VOTE_TEXTURE_PATHS[0]),
                )
                .with_color(Color::srgba(1.0, 1.0, 1.0, 0.95)),
                Pickable::IGNORE,
                Node {
                    position_type: PositionType::Absolute,
                    top: px(0),
                    left: px(0),
                    right: px(0),
                    bottom: px(-156.0 * render.main_ui_scale),
                    ..default()
                },
            ));
            panel.spawn((
                ImageNode::new(vote_texture(render, VOTE_TEXTURE_PATHS[7])),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(21.6),
                    top: percent(10.5),
                    width: percent(9.6),
                    aspect_ratio: Some(1.0),
                    ..default()
                },
            ));
            panel.spawn((
                VoteTextKind::RulerTitle,
                UiDisplayFont,
                Text::new("VOTE FOR RULER"),
                TextFont {
                    // Bevy's converted display font is wider than Unity's
                    // TextMeshPro atlas. Fit the complete authored label rather
                    // than clipping "RULER" at the right edge.
                    font_size: FontSize::Px(17.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(31.2),
                    right: percent(18.8),
                    top: percent(11.4),
                    height: percent(7.5),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
            panel.spawn((
                VoteTextKind::RulerDescription,
                Text::new("Who should be Ruler?"),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.91, 0.89, 0.81)),
                TextShadow {
                    offset: Vec2::splat(1.0),
                    color: Color::linear_rgba(0.0, 0.0, 0.0, 0.9),
                },
                ZIndex(2),
                Node {
                    position_type: PositionType::Absolute,
                    top: percent(18.9),
                    left: percent(10.9),
                    right: percent(11.1),
                    height: percent(10.5),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
            panel.spawn((
                RulerOptionsContainer,
                Name::new("Ruler vote options"),
                Node {
                    position_type: PositionType::Absolute,
                    top: percent(30.6),
                    bottom: percent(22.8),
                    left: percent(17.8),
                    right: percent(17.8),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(3),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
            spawn_vote_track(
                panel,
                render,
                VoteFillKind::RulerTimer,
                RULER_VOTE_TIMER_UNFILLED_PATH,
                VOTE_TEXTURE_PATHS[8],
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(17.9),
                    right: percent(17.9),
                    bottom: percent(10.3),
                    height: percent(7.4),
                    ..default()
                },
            );
            panel.spawn((
                VoteTextKind::RulerTimer,
                Text::new("02:00"),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.91, 0.89, 0.81)),
                TextShadow {
                    offset: Vec2::splat(1.0),
                    color: Color::linear_rgba(0.0, 0.0, 0.0, 0.9),
                },
                ZIndex(2),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(17.9),
                    right: percent(17.9),
                    bottom: percent(10.3),
                    ..default()
                },
            ));
        });
}

pub(crate) fn spawn_current_event_panel(commands: &mut Commands, render: &RenderAssets) {
    commands
        .spawn((
            WorldEntity,
            CurrentEventPanel,
            Name::new("Current town event"),
            authored_ui_image(
                render,
                CURRENT_EVENT_TEXTURE_PATHS[0],
                current_event_texture(render, CURRENT_EVENT_TEXTURE_PATHS[0]),
            ),
            GlobalZIndex(23),
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                top: px(76),
                left: px(18),
                width: px(290),
                height: px(104),
                padding: UiRect::all(px(10)),
                ..default()
            },
        ))
        .with_children(|panel| {
            panel.spawn((
                CurrentEventText::Title,
                UiDisplayFont,
                Text::new("Current Event"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.97, 0.88, 0.58)),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(8),
                    left: px(10),
                    right: px(10),
                    ..default()
                },
            ));
            panel.spawn((
                CurrentEventText::Description,
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(31),
                    left: px(10),
                    right: px(10),
                    ..default()
                },
            ));
            panel
                .spawn((
                    authored_ui_image(
                        render,
                        CURRENT_EVENT_TEXTURE_PATHS[1],
                        current_event_texture(render, CURRENT_EVENT_TEXTURE_PATHS[1]),
                    ),
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(12),
                        right: px(12),
                        bottom: px(10),
                        height: px(20),
                        overflow: Overflow::clip_x(),
                        ..default()
                    },
                ))
                .with_children(|track| {
                    track.spawn((
                        CurrentEventFill,
                        authored_ui_image(
                            render,
                            CURRENT_EVENT_TEXTURE_PATHS[2],
                            current_event_texture(render, CURRENT_EVENT_TEXTURE_PATHS[2]),
                        ),
                        Node {
                            width: percent(0.0),
                            height: percent(100.0),
                            ..default()
                        },
                    ));
                    track.spawn((
                        CurrentEventText::Progress,
                        Text::new(""),
                        TextFont {
                            font_size: FontSize::Px(12.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::srgb(0.91, 0.89, 0.81)),
                        Pickable::IGNORE,
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(4),
                            right: px(4),
                            top: px(1),
                            ..default()
                        },
                    ));
                });
        });
}

pub(crate) fn spawn_hud_metric(
    parent: &mut ChildSpawnerCommands,
    render: &RenderAssets,
    metric: HudMetric,
    source_path: &str,
    width: Val,
) {
    spawn_hud_metric_sized(parent, render, metric, source_path, width, 36.0, 20.0);
}

pub(crate) fn spawn_hud_metric_sized(
    parent: &mut ChildSpawnerCommands,
    render: &RenderAssets,
    metric: HudMetric,
    source_path: &str,
    width: Val,
    icon_size: f32,
    font_size: f32,
) {
    parent
        .spawn((
            HudMetricRow,
            Node {
                width,
                height: px(44),
                flex_shrink: 0.0,
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::clip(),
                ..default()
            },
        ))
        .with_children(|metric_parent| {
            if let Some(icon) = top_bar_texture(render, source_path) {
                metric_parent.spawn((
                    ImageNode::new(icon),
                    Node {
                        width: px(icon_size),
                        height: px(icon_size),
                        flex_shrink: 0.0,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                ));
            }
            metric_parent
                .spawn(Node {
                    flex_grow: 1.0,
                    height: px(40),
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::left(px(5)),
                    overflow: Overflow::clip(),
                    ..default()
                })
                .with_children(|values| {
                    values.spawn((
                        metric,
                        Text::new("0"),
                        TextFont {
                            font_size: FontSize::Px(font_size),
                            ..default()
                        },
                        TextColor(Color::srgb(0.91, 0.89, 0.81)),
                        TextLayout::no_wrap(),
                        Node {
                            height: px(23),
                            flex_shrink: 0.0,
                            overflow: Overflow::clip(),
                            ..default()
                        },
                    ));
                    if metric.shows_maximum() {
                        values.spawn((
                            HudMetricMaximum(metric),
                            Text::new("/0"),
                            TextFont {
                                font_size: FontSize::Px((font_size * 0.55).max(9.0)),
                                ..default()
                            },
                            TextColor(Color::srgb(0.72, 0.72, 0.68)),
                            TextLayout::no_wrap(),
                            Node {
                                height: px(12),
                                flex_shrink: 0.0,
                                overflow: Overflow::clip(),
                                ..default()
                            },
                        ));
                    }
                });
        });
}

pub(crate) fn spawn_hud(
    commands: &mut Commands,
    render: &RenderAssets,
    agents: u16,
    world_hash: &str,
) {
    let complete_art = TOP_BAR_TEXTURE_PATHS
        .iter()
        .all(|path| render.top_bar_textures.contains_key(*path));
    let mut root = commands.spawn((
        WorldEntity,
        HudTopBar,
        Name::new("Shipping top bar"),
        authored_ui_image(
            render,
            VOTE_TEXTURE_PATHS[0],
            vote_texture(render, VOTE_TEXTURE_PATHS[0]),
        ),
        GlobalZIndex(20),
        Node {
            position_type: PositionType::Absolute,
            top: px(0),
            left: px(0),
            width: percent(100.0),
            height: percent(6.2),
            min_height: px(62),
            max_height: px(76),
            ..default()
        },
        BackgroundColor(Color::srgba(0.025, 0.05, 0.035, 0.88)),
    ));
    root.with_children(|parent| {
        parent
            .spawn((
                HudResourceStrip,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(18),
                    top: percent(0.0),
                    width: percent(39.7),
                    height: percent(100.0),
                    align_items: AlignItems::Center,
                    overflow: Overflow::clip(),
                    ..default()
                },
            ))
            .with_children(|resources| {
                for (metric, path) in [
                    (HudMetric::Food, TOP_BAR_TEXTURE_PATHS[1]),
                    (HudMetric::Gold, TOP_BAR_TEXTURE_PATHS[2]),
                    (HudMetric::Ore, TOP_BAR_TEXTURE_PATHS[3]),
                    (HudMetric::Wood, TOP_BAR_TEXTURE_PATHS[4]),
                ] {
                    spawn_hud_metric(resources, render, metric, path, percent(25.0));
                }
            });
        parent
            .spawn((
                HudStatsStrip,
                Node {
                    position_type: PositionType::Absolute,
                    right: px(10),
                    top: percent(0.0),
                    width: percent(40.5),
                    height: percent(100.0),
                    align_items: AlignItems::Center,
                    overflow: Overflow::clip(),
                    ..default()
                },
            ))
            .with_children(|stats| {
                spawn_hud_metric(
                    stats,
                    render,
                    HudMetric::Players,
                    TOP_BAR_TEXTURE_PATHS[5],
                    percent(10.0),
                );
                spawn_hud_metric_sized(
                    stats,
                    render,
                    HudMetric::Npcs,
                    TOP_BAR_TEXTURE_PATHS[5],
                    percent(10.0),
                    27.0,
                    18.0,
                );
                spawn_hud_metric(
                    stats,
                    render,
                    HudMetric::Buildings,
                    TOP_BAR_TEXTURE_PATHS[6],
                    percent(13.0),
                );
                spawn_hud_metric(
                    stats,
                    render,
                    HudMetric::PlayTime,
                    TOP_BAR_TEXTURE_PATHS[7],
                    percent(22.0),
                );
                stats
                    .spawn((
                        HudTechnologyObjectivePanel,
                        Node {
                            width: percent(45.0),
                            height: percent(100.0),
                            align_self: AlignSelf::Center,
                            overflow: Overflow::clip(),
                            ..default()
                        },
                    ))
                    .with_children(|objective| {
                        objective.spawn((
                            HudTechnologyTextKind::Title,
                            UiDisplayFont,
                            Text::new("TECHNOLOGY"),
                            TextFont {
                                font_size: FontSize::Px(10.0),
                                ..default()
                            },
                            TextLayout::no_wrap(),
                            TextColor(Color::srgb(0.97, 0.88, 0.58)),
                            TextShadow {
                                offset: Vec2::splat(1.0),
                                color: Color::linear_rgba(0.0, 0.0, 0.0, 0.9),
                            },
                            Pickable::IGNORE,
                            Node {
                                position_type: PositionType::Absolute,
                                top: px(13),
                                right: px(20),
                                left: px(10),
                                height: px(13),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                        ));
                        objective.spawn((
                            HudTechnologyTextKind::Requirement,
                            Text::new("Ballot pending"),
                            TextFont {
                                font_size: FontSize::Px(8.5),
                                ..default()
                            },
                            TextLayout::no_wrap(),
                            TextColor(Color::srgb(0.91, 0.89, 0.81)),
                            TextShadow {
                                offset: Vec2::splat(1.0),
                                color: Color::linear_rgba(0.0, 0.0, 0.0, 0.85),
                            },
                            Pickable::IGNORE,
                            Node {
                                position_type: PositionType::Absolute,
                                top: px(25),
                                right: px(20),
                                left: px(10),
                                height: px(11),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                        ));
                        objective
                            .spawn((
                                authored_ui_image(
                                    render,
                                    OBJECTIVE_TEXTURE_PATHS[1],
                                    objective_texture(render, OBJECTIVE_TEXTURE_PATHS[1]),
                                ),
                                Pickable::IGNORE,
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: px(10),
                                    right: px(20),
                                    bottom: px(17),
                                    height: px(9),
                                    overflow: Overflow::clip_x(),
                                    ..default()
                                },
                            ))
                            .with_children(|track| {
                                track.spawn((
                                    HudTechnologyProgressFill,
                                    authored_ui_image(
                                        render,
                                        OBJECTIVE_TEXTURE_PATHS[2],
                                        objective_texture(render, OBJECTIVE_TEXTURE_PATHS[2]),
                                    ),
                                    Pickable::IGNORE,
                                    Node {
                                        width: percent(0.0),
                                        height: percent(100.0),
                                        ..default()
                                    },
                                ));
                            });
                    });
            });
        if let (Some(gauge), Some(meter)) = (
            top_bar_texture(render, TOP_BAR_TEXTURE_PATHS[8]),
            top_bar_texture(render, TOP_BAR_TEXTURE_PATHS[9]),
        ) {
            parent
                .spawn((
                    ImageNode::new(gauge),
                    Node {
                        position_type: PositionType::Absolute,
                        left: percent(42.15),
                        top: percent(12.6),
                        width: px(300),
                        height: percent(58.9),
                        ..default()
                    },
                ))
                .with_children(|gauge_parent| {
                    gauge_parent.spawn((
                        SeasonMeter,
                        ImageNode::new(meter),
                        Node {
                            position_type: PositionType::Absolute,
                            top: px(-3),
                            left: percent(0.0),
                            width: px(10),
                            height: px(38),
                            ..default()
                        },
                    ));
                });
        }
    });

    commands.spawn((
        WorldEntity,
        Hud,
        Name::new("Runtime diagnostics"),
        Text::new(if complete_art {
            format!("{agents} agents | world {}", &world_hash[..12])
        } else {
            format!(
                "{agents} agents | world {} | HUD art fallback",
                &world_hash[..12]
            )
        }),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.96, 0.9)),
        GlobalZIndex(20),
        if std::env::var_os("STREAM_TOWN_RUNTIME_DIAGNOSTICS").is_some() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: px(8),
            left: px(12),
            width: px(750),
            ..default()
        },
    ));

    commands.spawn((
        WorldEntity,
        HudCommandGuidance,
        Name::new("Twitch command guidance"),
        UiDisplayFont,
        Text::new("type !join to apply for citizenship\ntype !help for more commands"),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextLayout::new(Justify::Center, LineBreak::WordBoundary),
        TextColor(Color::srgb(0.97, 0.91, 0.7)),
        TextShadow {
            offset: Vec2::new(1.5, 1.5),
            color: Color::linear_rgba(0.0, 0.0, 0.0, 0.9),
        },
        Pickable::IGNORE,
        GlobalZIndex(20),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: percent(25.0),
            width: percent(50.0),
            ..default()
        },
    ));

    let slider_background = render
        .selection_panel_textures
        .get(SELECTION_PANEL_TEXTURE_PATHS[0])
        .cloned();
    let slider_fill = render
        .selection_panel_textures
        .get(SELECTION_PANEL_TEXTURE_PATHS[1])
        .cloned();
    let mut selection_panel = commands.spawn((
        WorldEntity,
        SelectionPanel,
        Name::new("Selection window"),
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(19.0),
            ..default()
        },
        TextColor(Color::WHITE),
        GlobalZIndex(20),
        Visibility::Hidden,
        Node {
            position_type: PositionType::Absolute,
            right: px(20),
            bottom: px(82),
            width: px(460),
            min_height: px(160),
            padding: UiRect {
                left: px(18),
                right: px(18),
                top: px(18),
                bottom: px(60),
            },
            border: UiRect::all(px(2)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.035, 0.075, 0.05, 0.94)),
        BorderColor::all(Color::srgb(0.78, 0.68, 0.24)),
    ));
    selection_panel.with_children(|parent| {
        for (bar, bottom) in [
            (SelectionPanelBar::Health, 14.0),
            (SelectionPanelBar::Experience, 36.0),
        ] {
            let mut track = parent.spawn((
                SelectionPanelSliderTrack(bar),
                Visibility::Hidden,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(18),
                    right: px(18),
                    bottom: px(bottom),
                    height: px(16),
                    overflow: Overflow::clip_x(),
                    ..default()
                },
            ));
            if let Some(background) = slider_background.clone() {
                track.insert(ImageNode::new(background).with_mode(NodeImageMode::Stretch));
            }
            track.with_children(|track_parent| {
                let slider_image = slider_fill
                    .clone()
                    .map_or_else(ImageNode::default, |image| {
                        ImageNode::new(image).with_mode(NodeImageMode::Stretch)
                    });
                track_parent.spawn((
                    SelectionPanelSlider(bar),
                    slider_image,
                    Node {
                        width: percent(100.0),
                        height: percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.8, 0.24)),
                ));
            });
        }
    });

    spawn_vote_panels(commands, render);
    spawn_current_event_panel(commands, render);
}

pub(crate) fn town_save_entry(path: &Path) -> TownSaveEntry {
    let protected = is_jump_start_path(path);
    let stem = path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("Town");
    TownSaveEntry {
        name: if protected {
            format!("{} (Jump-Start)", stem.trim_end_matches(".jumpstart"))
        } else {
            stem.to_owned()
        },
        path: path.to_path_buf(),
        protected,
    }
}

pub(crate) fn is_jump_start_path(path: &Path) -> bool {
    path.file_stem()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".jumpstart"))
}

pub(crate) fn unique_save_path(directory: &Path, preferred_stem: &str) -> PathBuf {
    let preferred_stem = safe_town_filename(preferred_stem);
    let direct = directory.join(format!("{preferred_stem}.stbevy"));
    if !direct.exists() {
        return direct;
    }
    for index in 2_u32.. {
        let candidate = directory.join(format!("{preferred_stem} {index}.stbevy"));
        if !candidate.exists() {
            return candidate;
        }
    }
    unreachable!("the jump-start suffix range is unbounded")
}

pub(crate) fn jump_start_snapshot_path(active: &Path) -> PathBuf {
    let directory = active.parent().unwrap_or_else(|| Path::new("."));
    let stem = safe_town_filename(
        active
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("Town"),
    );
    let direct = directory.join(format!("{stem}.jumpstart.stbevy"));
    if !direct.exists() {
        return direct;
    }
    for index in 2_u32.. {
        let candidate = directory.join(format!("{stem} {index}.jumpstart.stbevy"));
        if !candidate.exists() {
            return candidate;
        }
    }
    unreachable!("the jump-start suffix range is unbounded")
}

pub(crate) fn jump_start_working_path(
    catalog: &TownSaveCatalogRuntime,
    template: &Path,
) -> PathBuf {
    let stem = template
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("Town")
        .trim_end_matches(".jumpstart");
    unique_save_path(&catalog.directory, &format!("{stem} Jump-Start Copy"))
}

pub(crate) fn safe_town_filename(name: &str) -> String {
    let mut safe = name
        .trim()
        .chars()
        .map(|character| {
            if character.is_control()
                || matches!(
                    character,
                    '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
                )
            {
                '_'
            } else {
                character
            }
        })
        .collect::<String>();
    while safe.ends_with(' ') || safe.ends_with('.') {
        safe.pop();
    }
    if safe.is_empty()
        || matches!(
            safe.to_ascii_uppercase().as_str(),
            "CON"
                | "PRN"
                | "AUX"
                | "NUL"
                | "COM1"
                | "COM2"
                | "COM3"
                | "COM4"
                | "COM5"
                | "COM6"
                | "COM7"
                | "COM8"
                | "COM9"
                | "LPT1"
                | "LPT2"
                | "LPT3"
                | "LPT4"
                | "LPT5"
                | "LPT6"
                | "LPT7"
                | "LPT8"
                | "LPT9"
        )
    {
        safe.insert_str(0, "Town_");
    }
    safe
}

pub(crate) fn town_name_seed(name: &str) -> u64 {
    // Stable FNV-1a: unlike DefaultHasher this is identical across processes,
    // platforms, save/load cycles, and future Rust standard-library releases.
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in name.trim().to_lowercase().bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash.max(1)
}

pub(crate) fn open_new_town_dialog(menu: &mut MenuRuntime) {
    menu.page = MenuPage::NewTown;
    menu.return_page = MenuPage::Closed;
    menu.selected = 0;
    menu.feedback.clear();
}

pub(crate) fn open_load_town_dialog(menu: &mut MenuRuntime) {
    menu.page = MenuPage::LoadTown;
    menu.return_page = MenuPage::Closed;
    menu.selected = 0;
    menu.feedback.clear();
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn town_dialog_buttons(
    keyboard: Res<ButtonInput<KeyCode>>,
    actions: Query<(&Interaction, &TownDialogAction), Changed<Interaction>>,
    choices: Query<(&Interaction, &TownLoadChoice), Changed<Interaction>>,
    fields: Query<&EditableText, With<TownNameField>>,
    mut menu: ResMut<MenuRuntime>,
    mut catalog: ResMut<TownSaveCatalogRuntime>,
    mut save: ResMut<SaveRuntime>,
    mut config: ResMut<RuntimeConfig>,
    mut focus: ResMut<InputFocus>,
) {
    if !matches!(menu.page, MenuPage::NewTown | MenuPage::LoadTown) {
        return;
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        menu.page = MenuPage::Closed;
        menu.feedback.clear();
        focus.clear();
        return;
    }
    for (interaction, action) in &actions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            TownDialogAction::Back => {
                menu.page = MenuPage::Closed;
                menu.feedback.clear();
                focus.clear();
            }
            TownDialogAction::Create if menu.page == MenuPage::NewTown => {
                let name = fields
                    .single()
                    .ok()
                    .map(|field| field.value().to_string().trim().to_owned())
                    .unwrap_or_default();
                if name.is_empty() {
                    "Enter a town name first.".clone_into(&mut menu.feedback);
                    continue;
                }
                let path = catalog.new_town_path(&name);
                if path.is_file() {
                    format!("A town named '{name}' already exists. Choose Load Game instead.")
                        .clone_into(&mut menu.feedback);
                    continue;
                }
                save.store = NativeSaveStore::new(path);
                catalog.active_town = Some(name.clone());
                config.0.world.seed = town_name_seed(&name);
                focus.clear();
                request_go_live_confirmation(&mut menu, PendingTownStart::NewGame);
            }
            TownDialogAction::Create => {}
        }
    }
    if menu.page != MenuPage::LoadTown {
        return;
    }
    for (interaction, choice) in &choices {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let store = NativeSaveStore::new(&choice.path);
        match store.load() {
            Ok(snapshot) => {
                config.0.world.seed = snapshot.world_seed;
                if choice.protected {
                    let working_path = jump_start_working_path(&catalog, &choice.path);
                    save.store = NativeSaveStore::new(working_path);
                    catalog.active_town = Some(format!(
                        "{} Copy",
                        choice.name.trim_end_matches(" (Jump-Start)")
                    ));
                } else {
                    save.store = store;
                    catalog.active_town = Some(choice.name.clone());
                }
                focus.clear();
                request_go_live_confirmation(
                    &mut menu,
                    PendingTownStart::LoadGame {
                        source: choice.path.clone(),
                    },
                );
            }
            Err(error) => {
                format!("Could not load '{}': {error}", choice.name).clone_into(&mut menu.feedback);
            }
        }
    }
}

pub(crate) fn update_town_dialog_ui(
    menu: Res<MenuRuntime>,
    mut roots: Query<(&TownDialogRoot, &mut Visibility)>,
    mut feedback: Query<&mut Text, With<TownDialogFeedback>>,
) {
    for (root, mut visibility) in &mut roots {
        *visibility = if menu.page == root.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut text in &mut feedback {
        (**text).clone_from(&menu.feedback);
    }
}

pub(crate) fn scroll_town_load_list(
    menu: Res<MenuRuntime>,
    mut wheel: MessageReader<MouseWheel>,
    mut lists: Query<(&mut ScrollPosition, &ComputedNode), With<TownLoadList>>,
) {
    if menu.page != MenuPage::LoadTown {
        return;
    }
    let delta = wheel.read().fold(0.0, |total, event| {
        total
            + event.y
                * if event.unit == MouseScrollUnit::Line {
                    32.0
                } else {
                    1.0
                }
    });
    if delta.abs() <= f32::EPSILON {
        return;
    }
    for (mut position, computed) in &mut lists {
        let maximum = (computed.content_size().y - computed.size().y).max(0.0)
            * computed.inverse_scale_factor();
        position.y = (position.y - delta).clamp(0.0, maximum);
    }
}

pub(crate) fn main_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<RuntimeConfig>,
    secrets: Res<SecretsRuntime>,
    connection: Res<TwitchConnection>,
    mut menu: ResMut<MenuRuntime>,
    focus: Res<InputFocus>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if menu.page != MenuPage::Closed {
        return;
    }
    if keyboard.just_pressed(KeyCode::Enter) && focus.get().is_none() {
        if twitch_accounts_connected(&config.0, &secrets, &connection) {
            open_new_town_dialog(&mut menu);
        } else {
            open_twitch_setup_required(&mut menu);
        }
    } else if keyboard.just_pressed(KeyCode::KeyC) {
        next_state.set(GameState::Credits);
    } else if keyboard.just_pressed(KeyCode::Escape)
        && !keyboard.pressed(KeyCode::ShiftLeft)
        && !keyboard.pressed(KeyCode::ShiftRight)
    {
        exit.write(AppExit::Success);
    }
}

pub(crate) fn main_menu_action_enabled(action: MainMenuAction, has_save: bool) -> bool {
    action != MainMenuAction::LoadGame || has_save
}

pub(crate) fn main_menu_buttons(
    town_catalog: Res<TownSaveCatalogRuntime>,
    settings: Res<RuntimePlayerSettings>,
    config: Res<RuntimeConfig>,
    secrets: Res<SecretsRuntime>,
    connection: Res<TwitchConnection>,
    mut menu: ResMut<MenuRuntime>,
    buttons: Query<(&Interaction, &MainMenuAction), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if menu.page != MenuPage::Closed {
        return;
    }
    let has_save = !town_catalog.entries().is_empty();
    for (interaction, action) in &buttons {
        if *interaction != Interaction::Pressed || !main_menu_action_enabled(*action, has_save) {
            continue;
        }
        match action {
            MainMenuAction::NewGame => {
                if twitch_accounts_connected(&config.0, &secrets, &connection) {
                    open_new_town_dialog(&mut menu);
                } else {
                    open_twitch_setup_required(&mut menu);
                }
            }
            MainMenuAction::LoadGame => {
                if twitch_accounts_connected(&config.0, &secrets, &connection) {
                    open_load_town_dialog(&mut menu);
                } else {
                    open_twitch_setup_required(&mut menu);
                }
            }
            MainMenuAction::Settings => {
                open_settings_menu(
                    &mut menu,
                    MenuPage::Closed,
                    &settings.0,
                    &config.0.twitch.broadcast,
                );
            }
            MainMenuAction::Secrets => {
                menu.page = MenuPage::SecretsDisclaimer;
                menu.return_page = MenuPage::Closed;
                menu.selected = 0;
                menu.feedback.clear();
            }
            MainMenuAction::Credits => next_state.set(GameState::Credits),
            MainMenuAction::Quit => {
                exit.write(AppExit::Success);
            }
        }
    }
}

pub(crate) fn request_go_live_confirmation(menu: &mut MenuRuntime, start: PendingTownStart) {
    menu.page = MenuPage::GoLiveConfirmation;
    menu.return_page = MenuPage::Closed;
    menu.selected = 1;
    menu.pending_town_start = Some(start);
    menu.feedback.clear();
}

pub(crate) fn cancel_go_live_confirmation(menu: &mut MenuRuntime) {
    menu.page = MenuPage::Closed;
    menu.return_page = MenuPage::Closed;
    menu.selected = 0;
    menu.pending_town_start = None;
    menu.feedback.clear();
}

pub(crate) fn confirm_go_live_and_start_town(
    commands: &mut Commands,
    menu: &mut MenuRuntime,
    io: &mut MenuIoRequest,
    #[cfg(target_os = "windows")] broadcast: &mut direct_broadcast::DirectBroadcastControl,
) {
    let Some(start) = menu.pending_town_start.take() else {
        cancel_go_live_confirmation(menu);
        return;
    };
    match start {
        PendingTownStart::NewGame => {
            io.load = false;
            io.load_source = None;
        }
        PendingTownStart::LoadGame { source } => {
            io.load = true;
            io.load_source = Some(source);
        }
    }
    menu.page = MenuPage::Closed;
    menu.return_page = MenuPage::Closed;
    menu.selected = 0;
    menu.feedback.clear();
    #[cfg(target_os = "windows")]
    broadcast.request_restart();
    queue_world_loading(commands);
}

pub(crate) fn go_live_confirmation_buttons(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut menu: ResMut<MenuRuntime>,
    mut io: ResMut<MenuIoRequest>,
    buttons: Query<(&Interaction, &GoLiveConfirmationAction), Changed<Interaction>>,
    #[cfg(target_os = "windows")] mut broadcast: ResMut<direct_broadcast::DirectBroadcastControl>,
) {
    if menu.page != MenuPage::GoLiveConfirmation {
        return;
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft)
        || keyboard.just_pressed(KeyCode::ArrowRight)
        || keyboard.just_pressed(KeyCode::Tab)
    {
        menu.selected = usize::from(menu.selected == 0);
    }
    let mut action = if keyboard.just_pressed(KeyCode::Escape) {
        Some(GoLiveConfirmationAction::No)
    } else if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        Some(if menu.selected == 0 {
            GoLiveConfirmationAction::No
        } else {
            GoLiveConfirmationAction::Yes
        })
    } else {
        None
    };
    for (interaction, pressed) in &buttons {
        if *interaction == Interaction::Pressed {
            action = Some(*pressed);
        }
    }
    match action {
        Some(GoLiveConfirmationAction::No) => cancel_go_live_confirmation(&mut menu),
        Some(GoLiveConfirmationAction::Yes) => confirm_go_live_and_start_town(
            &mut commands,
            &mut menu,
            &mut io,
            #[cfg(target_os = "windows")]
            &mut broadcast,
        ),
        None => {}
    }
}

pub(crate) fn update_main_menu_buttons(
    town_catalog: Res<TownSaveCatalogRuntime>,
    menu: Res<MenuRuntime>,
    render: Res<RenderAssets>,
    mut buttons: Query<(&Interaction, &MainMenuAction, &mut ImageNode)>,
) {
    let has_save = !town_catalog.entries().is_empty();
    for (interaction, action, mut image) in &mut buttons {
        let enabled = menu.page == MenuPage::Closed && main_menu_action_enabled(*action, has_save);
        let source_path = if !enabled {
            MAIN_MENU_TEXTURE_PATHS[2]
        } else if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            MAIN_MENU_TEXTURE_PATHS[1]
        } else {
            MAIN_MENU_TEXTURE_PATHS[0]
        };
        image.image = main_menu_texture(&render, source_path);
        image.color = if enabled {
            Color::WHITE
        } else {
            Color::srgba(0.78, 0.78, 0.78, 0.5)
        };
    }
}

pub(crate) fn update_go_live_confirmation_ui(
    menu: Res<MenuRuntime>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    mut root: Query<&mut Visibility, With<GoLiveConfirmationRoot>>,
    mut body: Query<&mut Text, With<GoLiveConfirmationBody>>,
    mut buttons: Query<(&Interaction, &GoLiveConfirmationAction, &mut ImageNode)>,
) {
    if let Ok(mut visibility) = root.single_mut() {
        *visibility = if menu.page == MenuPage::GoLiveConfirmation {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if let Ok(mut text) = body.single_mut() {
        if config.0.twitch.broadcast.bandwidth_test {
            "Starting this town will begin Twitch bandwidth-test output once loading is complete. The test is not publicly listed, but it uses the full stream path. Do you agree to start it?"
        } else {
            "Starting this town will begin the internal Twitch stream once loading is complete. Do you agree to go live?"
        }
        .clone_into(&mut **text);
    }
    for (interaction, action, mut image) in &mut buttons {
        let index = usize::from(*action == GoLiveConfirmationAction::Yes);
        let enabled = menu.page == MenuPage::GoLiveConfirmation;
        let highlighted = enabled
            && (menu.selected == index
                || *interaction == Interaction::Hovered
                || *interaction == Interaction::Pressed);
        let source = if !enabled {
            MAIN_MENU_TEXTURE_PATHS[2]
        } else if highlighted {
            MAIN_MENU_TEXTURE_PATHS[1]
        } else {
            MAIN_MENU_TEXTURE_PATHS[0]
        };
        image.image = main_menu_texture(&render, source);
    }
}

pub(crate) const fn game_menu_action_label(
    action: GameMenuAction,
    broadcast_active: bool,
) -> &'static str {
    match action {
        GameMenuAction::SaveGame => "Save Game",
        GameMenuAction::SaveJumpStart => "Save Jump-Start",
        GameMenuAction::LoadGame => "Load Game",
        GameMenuAction::Settings => "Settings",
        GameMenuAction::GoLive if broadcast_active => "End Stream",
        GameMenuAction::GoLive => "Go Live",
        GameMenuAction::ExitGame => "Exit Game",
        GameMenuAction::Close => "Close",
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn toggle_direct_broadcast(
    config: &GameConfig,
    secrets: &SecretsRuntime,
    connection: &TwitchConnection,
    runtime: &direct_broadcast::DirectBroadcastRuntime,
    control: &mut direct_broadcast::DirectBroadcastControl,
    feedback: &mut String,
) {
    if runtime.snapshot().phase.is_active() {
        control.request_stop_and_return_to_main_menu();
        "Ending the internal Twitch stream and returning to Main Menu...".clone_into(feedback);
    } else if twitch_accounts_connected(config, secrets, connection) {
        control.request_restart();
        "Starting the internal Twitch stream...".clone_into(feedback);
    } else {
        "Connect both Twitch accounts from Main Menu → Secrets before going live."
            .clone_into(feedback);
    }
}

pub(crate) fn game_menu_action_enabled(action: GameMenuAction, has_save: bool) -> bool {
    action != GameMenuAction::LoadGame || has_save
}

pub(crate) const fn game_menu_action_index(action: GameMenuAction) -> usize {
    match action {
        GameMenuAction::SaveGame => 0,
        GameMenuAction::SaveJumpStart => 1,
        GameMenuAction::LoadGame => 2,
        GameMenuAction::Settings => 3,
        GameMenuAction::GoLive => 4,
        GameMenuAction::ExitGame => 5,
        GameMenuAction::Close => 6,
    }
}

pub(crate) fn game_menu_buttons(
    save: Res<SaveRuntime>,
    settings: Res<RuntimePlayerSettings>,
    config: Res<RuntimeConfig>,
    secrets: Res<SecretsRuntime>,
    connection: Res<TwitchConnection>,
    mut menu: ResMut<MenuRuntime>,
    mut io: ResMut<MenuIoRequest>,
    buttons: Query<(&Interaction, &GameMenuAction), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    #[cfg(target_os = "windows")] broadcast_runtime: Res<direct_broadcast::DirectBroadcastRuntime>,
    #[cfg(target_os = "windows")] mut broadcast_control: ResMut<
        direct_broadcast::DirectBroadcastControl,
    >,
) {
    if menu.page != MenuPage::Game {
        return;
    }
    let has_save = save.store.path().is_file();
    for (interaction, action) in &buttons {
        if *interaction != Interaction::Pressed || !game_menu_action_enabled(*action, has_save) {
            continue;
        }
        match action {
            GameMenuAction::SaveGame => {
                io.save = true;
                menu.page = MenuPage::Closed;
            }
            GameMenuAction::SaveJumpStart => {
                io.save_jump_start = true;
                menu.page = MenuPage::Closed;
            }
            GameMenuAction::LoadGame => {
                io.load = true;
                io.load_source = None;
                menu.page = MenuPage::Closed;
            }
            GameMenuAction::Settings => {
                open_settings_menu(
                    &mut menu,
                    MenuPage::Game,
                    &settings.0,
                    &config.0.twitch.broadcast,
                );
            }
            GameMenuAction::GoLive => {
                #[cfg(target_os = "windows")]
                toggle_direct_broadcast(
                    &config.0,
                    &secrets,
                    &connection,
                    &broadcast_runtime,
                    &mut broadcast_control,
                    &mut menu.feedback,
                );
                #[cfg(not(target_os = "windows"))]
                "Direct Twitch streaming is available only on Windows."
                    .clone_into(&mut menu.feedback);
            }
            GameMenuAction::ExitGame => {
                menu.page = MenuPage::Closed;
                next_state.set(GameState::MainMenu);
            }
            GameMenuAction::Close => {
                menu.page = MenuPage::Closed;
            }
        }
    }
}

pub(crate) fn setup_accessibility(mut commands: Commands) {
    let mut announcement = AccessibleNode::new(Role::Status);
    announcement.set_label("Stream Town status");
    announcement.set_value("Starting Stream Town");
    announcement.set_live(Live::Polite);
    commands.spawn((
        Name::new("Accessibility announcements"),
        AccessibilityAnnouncement,
        AccessibilityNode(announcement),
    ));
}

#[allow(clippy::type_complexity)]
pub(crate) fn tag_accessible_buttons(
    mut commands: Commands,
    buttons: Query<
        (
            Entity,
            Option<&MainMenuAction>,
            Option<&GoLiveConfirmationAction>,
            Option<&GameMenuAction>,
            Option<&SettingsTabButton>,
            Option<&SettingsValueButton>,
            Option<&SettingsAction>,
            Option<&CreditsSkipButton>,
        ),
        (With<Button>, Without<AccessibleButtonScope>),
    >,
) {
    for (entity, main, go_live, game, tab, value, settings, credits) in &buttons {
        let scope = if main.is_some() {
            Some(AccessibleButtonScope::MainMenu)
        } else if go_live.is_some() {
            Some(AccessibleButtonScope::GoLiveConfirmation)
        } else if game.is_some() {
            Some(AccessibleButtonScope::GameMenu)
        } else if settings.is_some_and(|action| {
            matches!(
                action,
                SettingsAction::ConfirmApply | SettingsAction::ConfirmDiscard
            )
        }) {
            Some(AccessibleButtonScope::SettingsConfirm)
        } else if tab.is_some() || value.is_some() || settings.is_some() {
            Some(AccessibleButtonScope::Settings)
        } else if credits.is_some() {
            Some(AccessibleButtonScope::Credits)
        } else {
            None
        };
        if let Some(scope) = scope {
            commands
                .entity(entity)
                .try_insert((scope, Outline::new(px(0), px(0), Color::NONE)));
        }
    }
}

pub(crate) fn tag_accessible_text(
    mut commands: Commands,
    text: Query<Entity, (Added<Text>, Without<Label>)>,
    parents: Query<&ChildOf>,
    buttons: Query<(), With<Button>>,
) {
    for entity in &text {
        let mut ancestor = entity;
        let mut inside_button = false;
        for _ in 0..16 {
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
            if buttons.contains(ancestor) {
                inside_button = true;
                break;
            }
        }
        if !inside_button {
            commands.entity(entity).try_insert(Label);
        }
    }
}

type DecorativeAccessibilityImageQuery<'w, 's> =
    Query<'w, 's, Entity, (With<ImageNode>, With<AccessibilityNode>, Without<Button>)>;

pub(crate) fn prune_decorative_accessibility_nodes(
    mut commands: Commands,
    images: DecorativeAccessibilityImageQuery,
) {
    for entity in &images {
        commands.entity(entity).try_remove::<AccessibilityNode>();
    }
}

pub(crate) fn accessibility_scope_active(
    scope: AccessibleButtonScope,
    state: GameState,
    menu: &MenuRuntime,
) -> bool {
    match scope {
        AccessibleButtonScope::MainMenu => {
            state == GameState::MainMenu && menu.page == MenuPage::Closed
        }
        AccessibleButtonScope::GoLiveConfirmation => {
            state == GameState::MainMenu && menu.page == MenuPage::GoLiveConfirmation
        }
        AccessibleButtonScope::GameMenu => {
            state == GameState::InGame && menu.page == MenuPage::Game
        }
        AccessibleButtonScope::Settings => {
            menu.page == MenuPage::Settings && !menu.confirm_settings_close
        }
        AccessibleButtonScope::SettingsConfirm => {
            menu.page == MenuPage::Settings && menu.confirm_settings_close
        }
        AccessibleButtonScope::Credits => state == GameState::Credits,
    }
}

pub(crate) fn accessibility_button_enabled(
    main: Option<&MainMenuAction>,
    game: Option<&GameMenuAction>,
    has_save: bool,
) -> bool {
    main.is_none_or(|action| main_menu_action_enabled(*action, has_save))
        && game.is_none_or(|action| game_menu_action_enabled(*action, has_save))
}

pub(crate) fn set_accessibility_label(node: &mut AccessibleNode, label: impl Into<String>) {
    node.set_label(label.into().into_boxed_str());
}

#[derive(QueryData)]
#[query_data(mutable)]
pub(crate) struct AccessibleButtonNodeQuery {
    scope: &'static AccessibleButtonScope,
    node: &'static mut AccessibilityNode,
    visibility: Option<&'static Visibility>,
    inherited_visibility: Option<&'static InheritedVisibility>,
    main: Option<&'static MainMenuAction>,
    game: Option<&'static GameMenuAction>,
    settings_tab: Option<&'static SettingsTabButton>,
    settings_value: Option<&'static SettingsValueButton>,
    settings_action: Option<&'static SettingsAction>,
    credits: Option<&'static CreditsSkipButton>,
}

#[allow(clippy::type_complexity)]
pub(crate) fn enhance_accessible_buttons(
    state: Res<State<GameState>>,
    menu: Res<MenuRuntime>,
    save: Res<SaveRuntime>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
    mut buttons: Query<AccessibleButtonNodeQuery>,
) {
    let has_save = save.store.path().is_file();
    #[cfg(target_os = "windows")]
    let broadcast_active = broadcast.snapshot().phase.is_active();
    #[cfg(not(target_os = "windows"))]
    let broadcast_active = false;
    for mut button in &mut buttons {
        let node = &mut button.node;
        node.add_action(AccessAction::Click);
        node.add_action(AccessAction::Focus);
        node.add_action(AccessAction::Blur);
        let visible = !matches!(button.visibility, Some(Visibility::Hidden))
            && button
                .inherited_visibility
                .is_none_or(|visibility| visibility.get())
            && accessibility_scope_active(*button.scope, *state.get(), &menu);
        if visible {
            node.clear_hidden();
        } else {
            node.set_hidden();
        }
        let enabled = accessibility_button_enabled(button.main, button.game, has_save)
            && button
                .settings_value
                .is_none_or(|value| settings_value_enabled(value.index, broadcast_active));
        if enabled {
            node.clear_disabled();
        } else {
            node.set_disabled();
        }
        if let Some(action) = button.main {
            set_accessibility_label(node, main_menu_action_label(*action));
        } else if let Some(action) = button.game {
            set_accessibility_label(node, game_menu_action_label(*action, broadcast_active));
        } else if let Some(tab) = button.settings_tab {
            node.set_role(Role::Tab);
            set_accessibility_label(node, settings_tab_label(tab.0));
            node.set_selected(tab.0 == menu.settings_tab);
        } else if let Some(value_button) = button.settings_value {
            let (label, value) =
                settings_value_label(&menu.draft, &menu.streaming_draft, value_button.index);
            let direction = if value_button.direction < 0 {
                "Previous"
            } else {
                "Next"
            };
            set_accessibility_label(node, format!("{direction} {label}"));
            node.set_value(value.into_boxed_str());
        } else if let Some(action) = button.settings_action {
            set_accessibility_label(
                node,
                match action {
                    SettingsAction::Apply => "Apply and save settings",
                    SettingsAction::Defaults => "Restore default settings",
                    SettingsAction::Back => "Back from settings",
                    SettingsAction::ConfirmApply => "Apply unsaved changes",
                    SettingsAction::ConfirmDiscard => "Discard unsaved changes",
                },
            );
        } else if button.credits.is_some() {
            set_accessibility_label(node, "Skip credits");
        }
    }
}

#[derive(Clone, Copy)]
struct AccessibilityCandidate {
    entity: Entity,
    position: Vec2,
}

#[allow(clippy::type_complexity)]
pub(crate) fn accessibility_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    menu: Res<MenuRuntime>,
    save: Res<SaveRuntime>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut runtime: ResMut<AccessibilityRuntime>,
    mut action_requests: MessageReader<AccessibilityActionRequest>,
    focusable_widgets: Query<(), Or<(With<EditableText>, With<TabIndex>)>>,
    mut buttons: Query<(
        Entity,
        &mut Interaction,
        &AccessibleButtonScope,
        Option<&Visibility>,
        Option<&InheritedVisibility>,
        Option<&UiGlobalTransform>,
        Option<&MainMenuAction>,
        Option<&GameMenuAction>,
    )>,
) {
    if let Some(previous) = runtime.synthetic_pressed.take()
        && let Ok((_, mut interaction, ..)) = buttons.get_mut(previous)
    {
        *interaction = Interaction::None;
    }

    let has_save = save.store.path().is_file();
    let mut candidates = buttons
        .iter_mut()
        .filter_map(
            |(
                entity,
                interaction,
                scope,
                visibility,
                inherited_visibility,
                transform,
                main,
                game,
            )| {
                let visible = !matches!(visibility, Some(Visibility::Hidden))
                    && inherited_visibility.is_none_or(|visibility| visibility.get())
                    && accessibility_scope_active(*scope, *state.get(), &menu);
                let enabled = accessibility_button_enabled(main, game, has_save);
                (visible && enabled).then(|| {
                    let position = transform.map_or(Vec2::ZERO, |transform| {
                        transform.to_scale_angle_translation().2
                    });
                    if *interaction == Interaction::Pressed {
                        focus.set(entity, FocusCause::Pressed);
                        focus_visible.0 = false;
                    }
                    AccessibilityCandidate { entity, position }
                })
            },
        )
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        left.position
            .y
            .total_cmp(&right.position.y)
            .then_with(|| left.position.x.total_cmp(&right.position.x))
            .then_with(|| left.entity.to_bits().cmp(&right.entity.to_bits()))
    });
    let candidate_entities = candidates
        .iter()
        .map(|candidate| candidate.entity)
        .collect::<HashSet<_>>();
    if focus.get().is_some_and(|entity| {
        accessibility_should_clear_focus(
            candidate_entities.contains(&entity),
            focusable_widgets.contains(entity),
        )
    }) {
        focus.clear();
    }

    let mut requested_click = None;
    for request in action_requests.read() {
        let entity = Entity::from_bits(request.target_node.0);
        if !candidate_entities.contains(&entity) {
            continue;
        }
        match request.action {
            AccessAction::Click => requested_click = Some(entity),
            AccessAction::Focus => {
                focus.set(entity, FocusCause::Navigated);
                focus_visible.0 = true;
            }
            AccessAction::Blur if focus.get() == Some(entity) => focus.clear(),
            _ => {}
        }
    }

    // The shipping Game and Settings menus already have complete arrow-key
    // navigation tied to their highlighted row. Keep that deterministic path;
    // this generic traversal covers the main menu, HUD, votes, and credits.
    if menu.page == MenuPage::Closed && !candidates.is_empty() {
        let backwards = keyboard.pressed(KeyCode::ShiftLeft)
            || keyboard.pressed(KeyCode::ShiftRight)
            || keyboard.just_pressed(KeyCode::ArrowUp)
            || keyboard.just_pressed(KeyCode::ArrowLeft);
        let navigation = keyboard.just_pressed(KeyCode::Tab)
            || (focus_visible.0
                && (keyboard.just_pressed(KeyCode::ArrowUp)
                    || keyboard.just_pressed(KeyCode::ArrowDown)
                    || keyboard.just_pressed(KeyCode::ArrowLeft)
                    || keyboard.just_pressed(KeyCode::ArrowRight)));
        if navigation {
            let current = focus.get().and_then(|entity| {
                candidates
                    .iter()
                    .position(|candidate| candidate.entity == entity)
            });
            let next = match (current, backwards) {
                (Some(index), false) => (index + 1) % candidates.len(),
                (Some(index), true) => index.checked_sub(1).unwrap_or(candidates.len() - 1),
                (None, false) => 0,
                (None, true) => candidates.len() - 1,
            };
            focus.set(candidates[next].entity, FocusCause::Navigated);
            focus_visible.0 = true;
        }
        if (keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space))
            && let Some(entity) = focus.get()
            && candidate_entities.contains(&entity)
        {
            requested_click = Some(entity);
        }
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        focus.clear();
        focus_visible.0 = false;
    }
    if let Some(entity) = requested_click
        && let Ok((_, mut interaction, ..)) = buttons.get_mut(entity)
    {
        *interaction = Interaction::Pressed;
        runtime.synthetic_pressed = Some(entity);
    }
}

pub(crate) const fn accessibility_should_clear_focus(
    is_accessibility_candidate: bool,
    is_focusable_widget: bool,
) -> bool {
    !is_accessibility_candidate && !is_focusable_widget
}

type AccessibilityFocusVisualQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static AccessibleButtonScope,
        Option<&'static Visibility>,
        Option<&'static InheritedVisibility>,
        &'static mut Outline,
    ),
>;

pub(crate) fn sync_accessibility_focus_visuals(
    state: Res<State<GameState>>,
    menu: Res<MenuRuntime>,
    settings: Res<RuntimePlayerSettings>,
    focus: Res<InputFocus>,
    focus_visible: Res<InputFocusVisible>,
    mut buttons: AccessibilityFocusVisualQuery,
) {
    for (entity, scope, visibility, inherited_visibility, mut outline) in &mut buttons {
        let visible = !matches!(visibility, Some(Visibility::Hidden))
            && inherited_visibility.is_none_or(|visibility| visibility.get())
            && accessibility_scope_active(*scope, *state.get(), &menu);
        let focused = visible && focus.get() == Some(entity) && focus_visible.0;
        if focused {
            outline.width = px(4);
            outline.offset = px(2);
            outline.color = Color::srgb(1.0, 0.86, 0.16);
        } else if visible && settings.0.interface.high_contrast {
            outline.width = px(1);
            outline.offset = px(1);
            outline.color = Color::srgba(1.0, 1.0, 1.0, 0.82);
        } else {
            outline.width = px(0);
            outline.offset = px(0);
            outline.color = Color::NONE;
        }
    }
}

pub(crate) fn sync_accessibility_contrast(
    mut commands: Commands,
    settings: Res<RuntimePlayerSettings>,
    text: Query<(Entity, Option<&AccessibilityHighContrastText>), With<Text>>,
) {
    for (entity, applied) in &text {
        if settings.0.interface.high_contrast && applied.is_none() {
            commands.entity(entity).insert((
                AccessibilityHighContrastText,
                TextShadow {
                    offset: Vec2::splat(2.0),
                    color: Color::linear_rgba(0.0, 0.0, 0.0, 0.94),
                },
            ));
        } else if !settings.0.interface.high_contrast && applied.is_some() {
            commands
                .entity(entity)
                .remove::<AccessibilityHighContrastText>()
                .remove::<TextShadow>();
        }
    }
}

pub(crate) fn reduced_tree_wind(defaults: Vec4, reduced: bool) -> Vec4 {
    if reduced {
        Vec4::new(defaults.x, 0.0, 0.0, defaults.w)
    } else {
        defaults
    }
}

pub(crate) fn reduced_grass_wind(defaults: Vec4, reduced: bool) -> Vec4 {
    if reduced {
        Vec4::new(0.0, defaults.y, 0.0, defaults.w)
    } else {
        defaults
    }
}

pub(crate) fn reduced_water_wind(defaults: Vec4, reduced: bool) -> Vec4 {
    if reduced {
        Vec4::new(0.0, 0.0, -defaults.z.abs(), defaults.w)
    } else {
        defaults
    }
}

pub(crate) fn sync_accessibility_preferences(
    settings: Res<RuntimePlayerSettings>,
    mut applied: Local<Option<(u16, bool)>>,
    mut ui_scale: Option<ResMut<UiScale>>,
    defaults: Option<Res<AccessibilityMotionDefaults>>,
    render: Option<Res<RenderAssets>>,
    mut tree_materials: Option<ResMut<Assets<TreeMaterial>>>,
    mut grass_materials: Option<ResMut<Assets<GrassMaterial>>>,
    mut water_materials: Option<ResMut<Assets<WaterMaterial>>>,
) {
    let signature = (
        settings.0.interface.ui_scale_percent,
        settings.0.interface.reduced_motion,
    );
    if *applied == Some(signature) {
        return;
    }
    if let Some(ui_scale) = ui_scale.as_deref_mut() {
        ui_scale.0 = f32::from(settings.0.interface.ui_scale_percent) / 100.0;
    }
    let (Some(defaults), Some(render)) = (defaults, render) else {
        return;
    };
    let reduced = settings.0.interface.reduced_motion;
    if let Some(materials) = tree_materials.as_deref_mut()
        && let Some(mut tree) = materials.get_mut(&render.tree)
    {
        tree.extension.parameters.wind_controls = reduced_tree_wind(defaults.tree, reduced);
    }
    if let Some(materials) = grass_materials.as_deref_mut()
        && let Some(mut grass) = materials.get_mut(&render.grass)
    {
        grass.extension.parameters.wind_controls = reduced_grass_wind(defaults.grass, reduced);
    }
    if let Some(materials) = water_materials.as_deref_mut() {
        let wind = reduced_water_wind(defaults.water, reduced);
        if let Some(mut water) = materials.get_mut(&render.water) {
            water.extension.parameters.wind_speed_noise_alpha = wind;
        }
        if let Some(mut menu_water) = materials.get_mut(&render.menu_water) {
            menu_water.extension.parameters.wind_speed_noise_alpha = wind;
        }
    }
    *applied = Some(signature);
}

pub(crate) fn accessibility_settings_selection(menu: &MenuRuntime) -> String {
    if menu.confirm_settings_close {
        return if menu.selected == 0 {
            "Apply unsaved changes".to_owned()
        } else {
            "Discard unsaved changes".to_owned()
        };
    }
    if menu.selected < SETTINGS_APPLY_INDEX {
        let (label, value) =
            settings_value_label(&menu.draft, &menu.streaming_draft, menu.selected);
        format!("{label}: {value}")
    } else {
        match menu.selected {
            SETTINGS_APPLY_INDEX => "Apply and save settings".to_owned(),
            SETTINGS_DEFAULTS_INDEX => "Restore default settings".to_owned(),
            SETTINGS_BACK_INDEX => "Back from settings".to_owned(),
            _ => "Settings".to_owned(),
        }
    }
}

pub(crate) fn announce_accessibility_state(
    state: Res<State<GameState>>,
    menu: Res<MenuRuntime>,
    loading: Option<Res<WorldLoadingRuntime>>,
    command_feedback: Res<CommandFeedback>,
    mut runtime: ResMut<AccessibilityRuntime>,
    mut announcements: Query<&mut AccessibilityNode, With<AccessibilityAnnouncement>>,
) {
    let message = match *state.get() {
        GameState::Boot => "Loading Stream Town".to_owned(),
        GameState::MainMenu | GameState::InGame if menu.page == MenuPage::Settings => format!(
            "Settings, {} tab. {}",
            settings_tab_label(menu.settings_tab),
            accessibility_settings_selection(&menu)
        ),
        GameState::MainMenu if menu.page == MenuPage::SecretsDisclaimer => {
            "Sensitive Twitch setup disclaimer. Choose Yes to continue or No to return.".to_owned()
        }
        GameState::MainMenu if menu.page == MenuPage::GoLiveConfirmation => {
            "Go-live confirmation. Choose Yes to start the town and begin streaming after loading, or No to remain on the main menu.".to_owned()
        }
        GameState::MainMenu if menu.page == MenuPage::Secrets => {
            "Twitch secrets setup. Internal Twitch video is blacked out. Bot and broadcaster accounts are authorized separately.".to_owned()
        }
        GameState::MainMenu if menu.page == MenuPage::NewTown => {
            "Name your town. The name determines its world seed and save file.".to_owned()
        }
        GameState::MainMenu if menu.page == MenuPage::LoadTown => {
            "Choose a saved town to load.".to_owned()
        }
        GameState::MainMenu => "Main menu. Press Tab to move between actions.".to_owned(),
        GameState::WorldLoading => loading.as_ref().map_or_else(
            || "Loading town".to_owned(),
            |loading| format!("{}. {}", loading.status, loading.substatus),
        ),
        GameState::InGame if menu.page == MenuPage::Game => {
            "Game menu. Use arrow keys and Enter.".to_owned()
        }
        GameState::InGame if !command_feedback.0.is_empty() => command_feedback.0.clone(),
        GameState::InGame => {
            "Town ready. Text commands are active; press Escape for the game menu.".to_owned()
        }
        GameState::Credits => {
            "Credits. Press Escape or activate Skip credits to return.".to_owned()
        }
    };
    if runtime.last_announcement == message {
        return;
    }
    runtime.last_announcement.clone_from(&message);
    if let Ok(mut node) = announcements.single_mut() {
        node.set_value(message.into_boxed_str());
    }
}

pub(crate) fn update_game_menu_controls(
    save: Res<SaveRuntime>,
    menu: Res<MenuRuntime>,
    render: Res<RenderAssets>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
    mut buttons: Query<(&Interaction, &GameMenuAction, &mut ImageNode)>,
    mut labels: Query<(&GameMenuActionLabel, &mut Text)>,
) {
    let has_save = save.store.path().is_file();
    #[cfg(target_os = "windows")]
    let broadcast_active = broadcast.snapshot().phase.is_active();
    #[cfg(not(target_os = "windows"))]
    let broadcast_active = false;
    for (label, mut text) in &mut labels {
        let desired = game_menu_action_label(label.0, broadcast_active);
        if text.0 != desired {
            desired.clone_into(&mut text.0);
        }
    }
    for (interaction, action, mut image) in &mut buttons {
        let enabled = menu.page == MenuPage::Game && game_menu_action_enabled(*action, has_save);
        let selected =
            menu.page == MenuPage::Game && menu.selected == game_menu_action_index(*action);
        let source_path = if !enabled {
            MAIN_MENU_TEXTURE_PATHS[2]
        } else if selected
            || *interaction == Interaction::Hovered
            || *interaction == Interaction::Pressed
        {
            MAIN_MENU_TEXTURE_PATHS[1]
        } else {
            MAIN_MENU_TEXTURE_PATHS[0]
        };
        image.image = main_menu_texture(&render, source_path);
        image.color = if enabled {
            Color::WHITE
        } else {
            Color::srgba(0.78, 0.78, 0.78, 0.5)
        };
    }
}

pub(crate) const SETTINGS_TABS: [SettingsTab; 6] = [
    SettingsTab::Video,
    SettingsTab::Audio,
    SettingsTab::Gameplay,
    SettingsTab::Accessibility,
    SettingsTab::Streaming,
    SettingsTab::Connection,
];

pub(crate) const fn settings_tab_label(tab: SettingsTab) -> &'static str {
    match tab {
        SettingsTab::Video => "Video",
        SettingsTab::Audio => "Audio",
        SettingsTab::Gameplay => "Gameplay",
        SettingsTab::Accessibility => "Accessibility",
        SettingsTab::Streaming => "Streaming",
        SettingsTab::Connection => "Connection",
    }
}

pub(crate) fn settings_tab_indices(tab: SettingsTab) -> &'static [usize] {
    match tab {
        SettingsTab::Video => &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        SettingsTab::Audio => &[11, 12, 13, 14],
        SettingsTab::Gameplay => &[15, 16, 17, 18, 19, 20, 21],
        SettingsTab::Accessibility => &[22, 23, 24],
        SettingsTab::Streaming => &[25, 26, 27, 28, 29, 30, 31, 32],
        SettingsTab::Connection => &[],
    }
}

pub(crate) const fn settings_index_is_streaming(index: usize) -> bool {
    index >= SETTINGS_STREAMING_FIRST_INDEX && index <= SETTINGS_STREAMING_LAST_INDEX
}

pub(crate) const fn settings_value_enabled(index: usize, streaming_locked: bool) -> bool {
    !streaming_locked || !settings_index_is_streaming(index)
}

pub(crate) const fn settings_tab_for_index(index: usize) -> Option<SettingsTab> {
    match index {
        0..=10 => Some(SettingsTab::Video),
        11..=14 => Some(SettingsTab::Audio),
        15..=21 => Some(SettingsTab::Gameplay),
        22..=24 => Some(SettingsTab::Accessibility),
        SETTINGS_STREAMING_FIRST_INDEX..=SETTINGS_STREAMING_LAST_INDEX => {
            Some(SettingsTab::Streaming)
        }
        _ => None,
    }
}

pub(crate) fn cycle_settings_tab(tab: SettingsTab, forward: bool) -> SettingsTab {
    let index = SETTINGS_TABS
        .iter()
        .position(|candidate| *candidate == tab)
        .expect("the active settings tab is part of the shipping tab order");
    let next = if forward {
        (index + 1) % SETTINGS_TABS.len()
    } else {
        index.checked_sub(1).unwrap_or(SETTINGS_TABS.len() - 1)
    };
    SETTINGS_TABS[next]
}

pub(crate) fn settings_value_label(
    settings: &PlayerSettings,
    streaming: &BroadcastConfig,
    index: usize,
) -> (&'static str, String) {
    let video = &settings.video;
    let camera = &settings.camera;
    let interface = &settings.interface;
    match index {
        0 => ("Display Mode", format!("{:?}", video.display_mode)),
        1 => ("Resolution", format!("{} x {}", video.width, video.height)),
        2 => ("VSync", on_off(video.vsync).to_owned()),
        3 => (
            "FPS Limiter",
            video
                .fps_limit
                .map_or("Unlimited".to_owned(), |value| value.to_string()),
        ),
        4 => ("Shadows", on_off(video.shadows_enabled).to_owned()),
        5 => ("Shadow Quality", video.shadow_map_resolution.to_string()),
        6 => (
            "Ambient Occlusion",
            on_off(video.ambient_occlusion).to_owned(),
        ),
        7 => ("MSAA", format!("x{}", video.msaa_samples)),
        8 => ("FXAA / SMAA", format!("{:?}", video.post_process_aa)),
        9 => ("Brightness", format!("{:.1}", video.brightness_ev)),
        10 => ("Gamma", format!("{:.1}", video.gamma)),
        11 => (
            "Master",
            format!("{}%", volume_percent(settings.audio.master)),
        ),
        12 => (
            "Music",
            format!("{}%", volume_percent(settings.audio.music)),
        ),
        13 => (
            "Sound Effects",
            format!("{}%", volume_percent(settings.audio.sound_effects)),
        ),
        14 => (
            "Ambience",
            format!("{}%", volume_percent(settings.audio.ambience)),
        ),
        15 => (
            "Zoom Sensitivity",
            format!("{:.0}", camera.zoom_sensitivity),
        ),
        16 => ("Field of View", camera.field_of_view_degrees.to_string()),
        17 => ("Username Display", format!("{:?}", interface.display_names)),
        18 => (
            "Building Health Display",
            format!("{:?}", interface.display_building_health),
        ),
        19 => (
            "Autosave Time",
            if settings.autosave_minutes == 0 {
                "Disabled".to_owned()
            } else {
                format!("{} Minutes", settings.autosave_minutes)
            },
        ),
        20 => (
            "Timelapse Frequency",
            timelapse_interval_label(settings.timelapse.interval).to_owned(),
        ),
        21 => (
            "Timelapse Dynamic",
            on_off(settings.timelapse.dynamic).to_owned(),
        ),
        22 => ("UI Scale", format!("{}%", interface.ui_scale_percent)),
        23 => ("High Contrast", on_off(interface.high_contrast).to_owned()),
        24 => (
            "Reduced Motion",
            on_off(interface.reduced_motion).to_owned(),
        ),
        25 => ("Direct Streaming", on_off(streaming.enabled).to_owned()),
        26 => (
            "Output Resolution",
            format!("{} x {}", streaming.width, streaming.height),
        ),
        27 => ("Frame Rate", format!("{} FPS", streaming.frames_per_second)),
        28 => (
            "Video Bitrate",
            format!("{} kbps", streaming.video_bitrate_kbps),
        ),
        29 => (
            "Audio Bitrate",
            format!("{} kbps", streaming.audio_bitrate_kbps),
        ),
        30 => ("Encoder", format!("{:?}", streaming.encoder)),
        31 => (
            "Bandwidth Test",
            on_off(streaming.bandwidth_test).to_owned(),
        ),
        32 => ("Render Mode", format!("{:?}", streaming.render_mode)),
        _ => ("Unknown", String::new()),
    }
}

pub(crate) fn spawn_settings_button(
    parent: &mut ChildSpawnerCommands,
    action: SettingsAction,
    label: &'static str,
    render: &RenderAssets,
) {
    parent
        .spawn((
            action,
            Button,
            settings_ui_image(
                render,
                MAIN_MENU_TEXTURE_PATHS[0],
                main_menu_texture(render, MAIN_MENU_TEXTURE_PATHS[0]),
            ),
            Node {
                flex_grow: 1.0,
                height: px(44),
                padding: UiRect::axes(px(18), px(10)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                Pickable::IGNORE,
            ));
        });
}

pub(crate) fn spawn_settings_value_row(
    parent: &mut ChildSpawnerCommands,
    settings: &PlayerSettings,
    streaming: &BroadcastConfig,
    selected: usize,
    index: usize,
    enabled: bool,
    render: &RenderAssets,
) {
    let (label, value) = settings_value_label(settings, streaming, index);
    parent
        .spawn((
            SettingsValueRow(index),
            BackgroundColor(if selected == index && enabled {
                Color::srgb(0.211, 0.240, 0.358)
            } else {
                Color::srgb(0.055, 0.071, 0.141)
            }),
            Node {
                width: percent(100.0),
                height: px(48),
                flex_shrink: 0.0,
                padding: UiRect::axes(px(14), px(6)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                overflow: Overflow::clip(),
                ..default()
            },
        ))
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextColor(if enabled {
                    Color::srgb(0.827, 0.745, 0.498)
                } else {
                    Color::srgb(0.48, 0.48, 0.52)
                }),
                Node {
                    width: percent(43.0),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
            for direction in [-1, 1] {
                if direction == 1 {
                    row.spawn((
                        SettingsValueText(index),
                        Text::new(value.clone()),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(if enabled {
                            Color::srgb(0.90, 0.88, 0.80)
                        } else {
                            Color::srgb(0.56, 0.56, 0.60)
                        }),
                        Pickable::IGNORE,
                        Node {
                            width: percent(30.0),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                    ));
                }
                row.spawn((
                    SettingsValueButton { index, direction },
                    Button,
                    settings_ui_image(
                        render,
                        MAIN_MENU_TEXTURE_PATHS[0],
                        main_menu_texture(render, MAIN_MENU_TEXTURE_PATHS[0]),
                    ),
                    Node {
                        width: px(36),
                        height: px(34),
                        flex_shrink: 0.0,
                        padding: UiRect::axes(px(8), px(6)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(if direction < 0 { "<" } else { ">" }),
                        TextColor(if enabled {
                            Color::WHITE
                        } else {
                            Color::srgb(0.42, 0.42, 0.46)
                        }),
                        Pickable::IGNORE,
                    ));
                });
            }
        });
}

pub(crate) fn settings_tab_buttons(
    mut menu: ResMut<MenuRuntime>,
    buttons: Query<(&Interaction, &SettingsTabButton), Changed<Interaction>>,
) {
    if menu.page != MenuPage::Settings || menu.confirm_settings_close {
        return;
    }
    for (interaction, tab) in &buttons {
        if *interaction == Interaction::Pressed {
            menu.settings_tab = tab.0;
            menu.selected = settings_tab_indices(tab.0)
                .first()
                .copied()
                .unwrap_or(SETTINGS_APPLY_INDEX);
            menu.feedback.clear();
        }
    }
}

pub(crate) fn settings_value_buttons(
    mut menu: ResMut<MenuRuntime>,
    buttons: Query<(&Interaction, &SettingsValueButton), Changed<Interaction>>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
) {
    if menu.page != MenuPage::Settings || menu.confirm_settings_close {
        return;
    }
    for (interaction, button) in &buttons {
        if *interaction == Interaction::Pressed {
            menu.selected = button.index;
            #[cfg(target_os = "windows")]
            let streaming_locked = broadcast.snapshot().phase.is_active();
            #[cfg(not(target_os = "windows"))]
            let streaming_locked = false;
            if !settings_value_enabled(button.index, streaming_locked) {
                "End the stream before changing streaming settings.".clone_into(&mut menu.feedback);
                continue;
            }
            let menu = &mut *menu;
            adjust_settings_menu(
                &mut menu.draft,
                &mut menu.streaming_draft,
                button.index,
                button.direction,
            );
            menu.feedback.clear();
        }
    }
}

pub(crate) fn apply_settings_draft(
    menu: &mut MenuRuntime,
    player_settings: &mut RuntimePlayerSettings,
    config: &mut RuntimeConfig,
    streaming_locked: bool,
) -> bool {
    if let Err(error) = menu.draft.validate() {
        menu.feedback = format!("Settings are invalid: {error}");
        return false;
    }
    if streaming_locked && menu.streaming_draft != config.0.twitch.broadcast {
        "End the stream before applying streaming settings.".clone_into(&mut menu.feedback);
        return false;
    }
    let mut config_draft = config.0.clone();
    config_draft.twitch.broadcast = menu.streaming_draft.clone();
    if let Err(error) = config_draft.validate() {
        menu.feedback = format!("Streaming settings are invalid: {error}");
        return false;
    }
    match PlayerSettingsStore::new(player_settings_path()).write(&menu.draft) {
        Ok(()) => match save_runtime_config(&config_draft) {
            Ok(_) => {
                config.0 = config_draft;
                player_settings.0 = menu.draft.clone();
                "Applied and saved settings".clone_into(&mut menu.feedback);
                true
            }
            Err(error) => {
                menu.feedback = format!("Streaming settings could not be saved: {error}");
                false
            }
        },
        Err(error) => {
            menu.feedback = format!("Settings could not be saved: {error}");
            false
        }
    }
}

pub(crate) fn close_settings_menu(menu: &mut MenuRuntime) {
    let target = menu.return_page;
    menu.page = target;
    if target == MenuPage::Game {
        menu.return_page = MenuPage::Closed;
    }
    menu.selected = 0;
    menu.settings_tab = SettingsTab::Video;
    menu.confirm_settings_close = false;
    menu.feedback.clear();
}

pub(crate) fn request_settings_close(
    menu: &mut MenuRuntime,
    player_settings: &PlayerSettings,
    streaming: &BroadcastConfig,
) {
    if menu.draft == *player_settings && menu.streaming_draft == *streaming {
        close_settings_menu(menu);
    } else {
        menu.confirm_settings_close = true;
        menu.selected = 0;
        menu.feedback.clear();
    }
}

pub(crate) fn settings_action_buttons(
    mut menu: ResMut<MenuRuntime>,
    mut player_settings: ResMut<RuntimePlayerSettings>,
    mut config: ResMut<RuntimeConfig>,
    buttons: Query<(&Interaction, &SettingsAction), Changed<Interaction>>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
) {
    if menu.page != MenuPage::Settings {
        return;
    }
    for (interaction, action) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        #[cfg(target_os = "windows")]
        let streaming_locked = broadcast.snapshot().phase.is_active();
        #[cfg(not(target_os = "windows"))]
        let streaming_locked = false;
        match action {
            SettingsAction::Apply if !menu.confirm_settings_close => {
                apply_settings_draft(
                    &mut menu,
                    &mut player_settings,
                    &mut config,
                    streaming_locked,
                );
            }
            SettingsAction::Defaults if !menu.confirm_settings_close => {
                menu.draft = PlayerSettings::default();
                if !streaming_locked {
                    menu.streaming_draft = BroadcastConfig::default();
                }
                "Restored default settings in this draft".clone_into(&mut menu.feedback);
            }
            SettingsAction::Back if !menu.confirm_settings_close => {
                request_settings_close(&mut menu, &player_settings.0, &config.0.twitch.broadcast);
            }
            SettingsAction::ConfirmApply
                if menu.confirm_settings_close
                    && apply_settings_draft(
                        &mut menu,
                        &mut player_settings,
                        &mut config,
                        streaming_locked,
                    ) =>
            {
                close_settings_menu(&mut menu);
            }
            SettingsAction::ConfirmDiscard if menu.confirm_settings_close => {
                menu.draft = player_settings.0.clone();
                menu.streaming_draft = config.0.twitch.broadcast.clone();
                close_settings_menu(&mut menu);
            }
            _ => {}
        }
    }
}

pub(crate) fn rebuild_settings_rows(
    mut commands: Commands,
    menu: Res<MenuRuntime>,
    twitch: Res<TwitchConnection>,
    render: Res<RenderAssets>,
    mut cache: ResMut<SettingsUiCache>,
    rows: Query<Entity, With<SettingsRows>>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
) {
    #[cfg(target_os = "windows")]
    let streaming_locked = broadcast.snapshot().phase.is_active();
    #[cfg(not(target_os = "windows"))]
    let streaming_locked = false;
    let signature = settings_rows_signature(&menu, &twitch, streaming_locked);
    if cache.signature == signature {
        return;
    }
    cache.signature = signature;
    let Ok(rows) = rows.single() else {
        return;
    };
    commands.entity(rows).despawn_children();
    commands.entity(rows).with_children(|parent| {
        if menu.settings_tab == SettingsTab::Connection {
            parent.spawn((
                Text::new(format!(
                    "CONNECTION\n\nTwitch: {}\n\nOAuth setup and credential storage are managed in stream_town_tools.\nThe game reconnects automatically from the saved public configuration.",
                    twitch_status_text(&twitch)
                )),
                TextFont {
                    font_size: FontSize::Px(19.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::WHITE),
                Node {
                    width: percent(100.0),
                    margin: UiRect::top(percent(8.0)),
                    ..default()
                },
            ));
            return;
        }
        if menu.settings_tab == SettingsTab::Streaming {
            parent.spawn((
                Text::new(if streaming_locked {
                    "STREAMING IS LIVE — END THE STREAM TO CHANGE THESE SETTINGS"
                } else {
                    "Changes are saved for the next stream. Preferred ingest remains in Secrets."
                }),
                TextFont {
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(if streaming_locked {
                    Color::srgb(1.0, 0.54, 0.38)
                } else {
                    Color::srgb(0.76, 0.82, 0.92)
                }),
                Pickable::IGNORE,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0),
                    top: px(0),
                    width: percent(100.0),
                    height: px(30),
                    ..default()
                },
            ));
        }
        let indices = settings_tab_indices(menu.settings_tab);
        let split = indices.len().div_ceil(2);
        for column_indices in [&indices[..split], &indices[split..]] {
            parent
                .spawn(Node {
                    width: percent(48.0),
                    height: percent(100.0),
                    margin: UiRect::top(if menu.settings_tab == SettingsTab::Streaming {
                        px(38)
                    } else {
                        px(0)
                    }),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(8),
                    overflow: Overflow::clip(),
                    ..default()
                })
                .with_children(|column| {
                    for &index in column_indices {
                        spawn_settings_value_row(
                            column,
                            &menu.draft,
                            &menu.streaming_draft,
                            menu.selected,
                            index,
                            settings_value_enabled(index, streaming_locked),
                            &render,
                        );
                    }
                });
        }
    });
}

pub(crate) fn settings_rows_signature(
    menu: &MenuRuntime,
    twitch: &TwitchConnection,
    streaming_locked: bool,
) -> String {
    let connection_status = if menu.settings_tab == SettingsTab::Connection {
        twitch_status_text(twitch)
    } else {
        String::new()
    };
    format!(
        "{:?}:{:?}:{streaming_locked}:{connection_status}",
        menu.page, menu.settings_tab,
    )
}

pub(crate) fn update_settings_value_rows(
    menu: Res<MenuRuntime>,
    mut rows: Query<(&SettingsValueRow, &mut BackgroundColor)>,
    mut values: Query<(&SettingsValueText, &mut Text)>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
) {
    if menu.page != MenuPage::Settings || menu.settings_tab == SettingsTab::Connection {
        return;
    }
    #[cfg(target_os = "windows")]
    let streaming_locked = broadcast.snapshot().phase.is_active();
    #[cfg(not(target_os = "windows"))]
    let streaming_locked = false;
    for (row, mut background) in &mut rows {
        let enabled = settings_value_enabled(row.0, streaming_locked);
        let desired = BackgroundColor(if menu.selected == row.0 && enabled {
            Color::srgb(0.211, 0.240, 0.358)
        } else {
            Color::srgb(0.055, 0.071, 0.141)
        });
        if *background != desired {
            *background = desired;
        }
    }
    for (value_text, mut text) in &mut values {
        let (_, desired) = settings_value_label(&menu.draft, &menu.streaming_draft, value_text.0);
        if text.0 != desired {
            text.0 = desired;
        }
    }
}

pub(crate) fn update_settings_controls(
    menu: Res<MenuRuntime>,
    render: Res<RenderAssets>,
    mut root: Query<&mut Visibility, (With<SettingsRoot>, Without<SettingsConfirmModal>)>,
    mut modal: Query<&mut Visibility, (With<SettingsConfirmModal>, Without<SettingsRoot>)>,
    mut tabs: Query<(&Interaction, &SettingsTabButton, &mut BackgroundColor)>,
    mut actions: Query<
        (&Interaction, &SettingsAction, &mut ImageNode),
        Without<SettingsValueButton>,
    >,
    mut values: Query<
        (&Interaction, &SettingsValueButton, &mut ImageNode),
        Without<SettingsAction>,
    >,
    mut feedback: Query<&mut Text, With<SettingsFeedbackText>>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
) {
    #[cfg(target_os = "windows")]
    let streaming_locked = broadcast.snapshot().phase.is_active();
    #[cfg(not(target_os = "windows"))]
    let streaming_locked = false;
    if let Ok(mut visibility) = root.single_mut() {
        *visibility = if menu.page == MenuPage::Settings {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if let Ok(mut visibility) = modal.single_mut() {
        *visibility = if menu.page == MenuPage::Settings && menu.confirm_settings_close {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for (interaction, tab, mut background) in &mut tabs {
        background.0 = if tab.0 == menu.settings_tab
            || *interaction == Interaction::Hovered
            || *interaction == Interaction::Pressed
        {
            Color::srgba(0.211, 0.240, 0.358, 0.95)
        } else {
            Color::srgba(0.055, 0.071, 0.141, 0.95)
        };
    }
    for (interaction, action, mut image) in &mut actions {
        let modal_action = matches!(
            action,
            SettingsAction::ConfirmApply | SettingsAction::ConfirmDiscard
        );
        let enabled =
            menu.page == MenuPage::Settings && (modal_action == menu.confirm_settings_close);
        let keyboard_selected = match action {
            SettingsAction::Apply => menu.selected == SETTINGS_APPLY_INDEX,
            SettingsAction::Defaults => menu.selected == SETTINGS_DEFAULTS_INDEX,
            SettingsAction::Back => menu.selected == SETTINGS_BACK_INDEX,
            SettingsAction::ConfirmApply => menu.confirm_settings_close && menu.selected == 0,
            SettingsAction::ConfirmDiscard => menu.confirm_settings_close && menu.selected == 1,
        };
        let source_path = if !enabled {
            MAIN_MENU_TEXTURE_PATHS[2]
        } else if keyboard_selected
            || *interaction == Interaction::Hovered
            || *interaction == Interaction::Pressed
        {
            MAIN_MENU_TEXTURE_PATHS[1]
        } else {
            MAIN_MENU_TEXTURE_PATHS[0]
        };
        image.image = main_menu_texture(&render, source_path);
    }
    for (interaction, button, mut image) in &mut values {
        let enabled = settings_value_enabled(button.index, streaming_locked);
        let source_path = if !enabled {
            MAIN_MENU_TEXTURE_PATHS[2]
        } else if menu.selected == button.index
            || *interaction == Interaction::Hovered
            || *interaction == Interaction::Pressed
        {
            MAIN_MENU_TEXTURE_PATHS[1]
        } else {
            MAIN_MENU_TEXTURE_PATHS[0]
        };
        image.image = main_menu_texture(&render, source_path);
        image.color = if enabled {
            Color::WHITE
        } else {
            Color::srgba(0.55, 0.55, 0.58, 0.55)
        };
    }
    if let Ok(mut text) = feedback.single_mut() {
        (**text).clone_from(&menu.feedback);
    }
}

pub(crate) fn spawn_menu_overlay(
    mut commands: Commands,
    state: Res<State<GameState>>,
    mut menu: ResMut<MenuRuntime>,
    config: Res<RuntimeConfig>,
    settings: Res<RuntimePlayerSettings>,
    render: Res<RenderAssets>,
    town_catalog: Res<TownSaveCatalogRuntime>,
    mut settings_ui: ResMut<SettingsUiCache>,
) {
    settings_ui.signature.clear();
    if std::env::var_os("STREAM_TOWN_AUTOSTART_GAME_MENU").is_some()
        && *state.get() == GameState::InGame
    {
        menu.page = MenuPage::Game;
        menu.return_page = MenuPage::Closed;
        menu.selected = 0;
        menu.feedback.clear();
    } else if std::env::var_os("STREAM_TOWN_AUTOSTART_SETTINGS").is_some() {
        open_settings_menu(
            &mut menu,
            MenuPage::Game,
            &settings.0,
            &config.0.twitch.broadcast,
        );
        if std::env::var("STREAM_TOWN_AUTOSTART_SETTINGS_TAB")
            .is_ok_and(|tab| tab.eq_ignore_ascii_case("streaming"))
        {
            menu.settings_tab = SettingsTab::Streaming;
            menu.selected = SETTINGS_STREAMING_FIRST_INDEX;
        }
    } else if std::env::var_os("STREAM_TOWN_AUTOSTART_SECRETS_DISCLAIMER").is_some() {
        menu.page = MenuPage::SecretsDisclaimer;
        menu.return_page = MenuPage::Closed;
        menu.selected = 0;
        menu.feedback.clear();
    }
    commands.spawn((
        StateEntity,
        MenuOverlay,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::srgb(0.92, 0.97, 0.91)),
        Visibility::Hidden,
        GlobalZIndex(100),
        Node {
            position_type: PositionType::Absolute,
            left: percent(5.0),
            top: percent(3.0),
            width: percent(90.0),
            min_height: percent(90.0),
            padding: UiRect::all(px(28)),
            border: UiRect::all(px(2)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.025, 0.045, 0.04, 0.97)),
        BorderColor::all(Color::srgb(0.42, 0.76, 0.52)),
    ));
    commands
        .spawn((
            StateEntity,
            GameMenuRoot,
            Name::new("Shipping in-game menu"),
            Visibility::Hidden,
            GlobalZIndex(100),
            Node {
                position_type: PositionType::Absolute,
                left: percent(37.5),
                top: percent(25.0),
                width: percent(25.0),
                height: percent(50.0),
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                Name::new("Game menu background"),
                authored_main_ui_image_with_ppu(
                    &render,
                    GAME_MENU_TEXTURE_PATHS[0],
                    main_menu_texture(&render, GAME_MENU_TEXTURE_PATHS[0]),
                    1.5,
                ),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(20.35),
                    top: percent(17.08),
                    width: percent(59.45),
                    height: percent(61.62),
                    ..default()
                },
            ));
            root.spawn((
                Name::new("Game menu controls"),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(29.05),
                    top: percent(22.7),
                    width: percent(42.05),
                    height: percent(50.1),
                    flex_direction: FlexDirection::Column,
                    row_gap: percent(2.4),
                    ..default()
                },
            ))
            .with_children(|controls| {
                for action in [
                    GameMenuAction::SaveGame,
                    GameMenuAction::SaveJumpStart,
                    GameMenuAction::LoadGame,
                    GameMenuAction::Settings,
                    GameMenuAction::GoLive,
                    GameMenuAction::ExitGame,
                ] {
                    controls
                        .spawn((
                            action,
                            Button,
                            authored_ui_image(
                                &render,
                                MAIN_MENU_TEXTURE_PATHS[0],
                                main_menu_texture(&render, MAIN_MENU_TEXTURE_PATHS[0]),
                            ),
                            Node {
                                width: percent(100.0),
                                flex_grow: 1.0,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                        ))
                        .with_children(|button| {
                            button.spawn((
                                GameMenuActionLabel(action),
                                Text::new(game_menu_action_label(action, false)),
                                TextFont {
                                    font_size: FontSize::Px(18.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                                Pickable::IGNORE,
                            ));
                        });
                }
            });
            root.spawn((
                GameMenuAction::Close,
                Button,
                Name::new("Game menu close"),
                ImageNode::new(main_menu_texture(&render, MAIN_MENU_TEXTURE_PATHS[1])),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(69.1),
                    top: percent(15.2),
                    width: percent(12.62),
                    height: percent(11.28),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|close| {
                close.spawn((
                    ImageNode::new(main_menu_texture(&render, GAME_MENU_TEXTURE_PATHS[1])),
                    Pickable::IGNORE,
                    Node {
                        width: percent(45.0),
                        height: percent(45.0),
                        ..default()
                    },
                ));
            });
        });
    commands
        .spawn((
            StateEntity,
            SettingsRoot,
            Name::new("Settings modal overlay"),
            BackgroundColor(Color::srgba(0.008, 0.012, 0.025, 0.58)),
            Visibility::Hidden,
            GlobalZIndex(110),
            Node {
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                width: percent(100.0),
                height: percent(100.0),
                padding: UiRect::all(px(24)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|settings_overlay| {
            settings_overlay
                .spawn((
                    SettingsPanel,
                    Name::new("Shipping settings menu"),
                    BackgroundColor(Color::srgb(0.035, 0.046, 0.09)),
                    BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                    Node {
                        width: percent(78.0),
                        height: percent(80.0),
                        max_width: px(1_420),
                        max_height: px(800),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(36)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                ))
                .with_children(|settings_root| {
            settings_root.spawn((
                Name::new("Settings panel nine-slice surface"),
                settings_panel_ui_image(
                    &render,
                    SETTINGS_BACKGROUND_TEXTURE_PATH,
                    main_menu_texture(&render, SETTINGS_BACKGROUND_TEXTURE_PATH),
                ),
                Pickable::IGNORE,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(3),
                    top: px(3),
                    right: px(3),
                    bottom: px(3),
                    ..default()
                },
            ));
            settings_root.spawn((
                UiDisplayFont,
                Text::new("SETTINGS"),
                TextFont {
                    font_size: FontSize::Px(34.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(3.0),
                    top: percent(3.0),
                    width: percent(16.0),
                    height: percent(7.0),
                    ..default()
                },
            ));
            settings_root
                .spawn((
                    Name::new("Settings tab buttons"),
                    Node {
                        position_type: PositionType::Absolute,
                        left: percent(3.0),
                        top: percent(12.0),
                        width: percent(16.0),
                        height: percent(42.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(10),
                        ..default()
                    },
                ))
                .with_children(|tabs| {
                    for tab in SETTINGS_TABS {
                        tabs.spawn((
                            SettingsTabButton(tab),
                            Button,
                            BackgroundColor(Color::srgb(0.055, 0.071, 0.141)),
                            BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                            Node {
                                width: percent(100.0),
                                flex_grow: 1.0,
                                border: UiRect::bottom(px(2)),
                                padding: UiRect::axes(px(18), px(10)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new(settings_tab_label(tab)),
                                TextFont {
                                    font_size: FontSize::Px(19.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                                Pickable::IGNORE,
                            ));
                        });
                    }
                });
            settings_root.spawn((
                SettingsRows,
                Name::new("Settings value rows"),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(23.0),
                    top: percent(8.0),
                    width: percent(72.0),
                    height: percent(82.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: percent(4.0),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
            settings_root.spawn((
                SettingsFeedbackText,
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.92, 0.97, 0.91)),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(23.0),
                    bottom: percent(4.0),
                    width: percent(72.0),
                    height: px(22),
                    ..default()
                },
            ));
            settings_root
                .spawn((
                    Name::new("Settings actions"),
                    Node {
                        position_type: PositionType::Absolute,
                        left: percent(3.0),
                        bottom: percent(7.0),
                        width: percent(16.0),
                        height: percent(27.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(10),
                        ..default()
                    },
                ))
                .with_children(|actions| {
                    spawn_settings_button(actions, SettingsAction::Apply, "Apply", &render);
                    spawn_settings_button(actions, SettingsAction::Defaults, "Defaults", &render);
                    spawn_settings_button(actions, SettingsAction::Back, "Back", &render);
                });
            settings_root
                .spawn((
                    SettingsConfirmModal,
                    Name::new("Confirm settings changes"),
                    Visibility::Hidden,
                    GlobalZIndex(120),
                    BackgroundColor(Color::srgb(0.035, 0.046, 0.09)),
                    BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                    Node {
                        position_type: PositionType::Absolute,
                        left: percent(25.0),
                        top: percent(27.0),
                        width: percent(50.0),
                        height: percent(38.0),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(28)),
                        padding: UiRect::all(percent(5.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                ))
                .with_children(|modal| {
                    modal.spawn((
                        Name::new("Settings confirmation nine-slice surface"),
                        settings_panel_ui_image(
                            &render,
                            SETTINGS_BACKGROUND_TEXTURE_PATH,
                            main_menu_texture(&render, SETTINGS_BACKGROUND_TEXTURE_PATH),
                        ),
                        Pickable::IGNORE,
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(3),
                            top: px(3),
                            right: px(3),
                            bottom: px(3),
                            ..default()
                        },
                    ));
                    modal.spawn((
                        Text::new("CONFIRM CHANGES\n\nYou have unsaved changes.\nDo you want to apply these changes?"),
                        TextFont {
                            font_size: FontSize::Px(19.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::WHITE),
                    ));
                    modal
                        .spawn((
                            Node {
                                width: percent(100.0),
                                height: px(48),
                                column_gap: px(18),
                                ..default()
                            },
                        ))
                        .with_children(|buttons| {
                            spawn_settings_button(
                                buttons,
                                SettingsAction::ConfirmApply,
                                "Yes",
                                &render,
                            );
                            spawn_settings_button(
                                buttons,
                                SettingsAction::ConfirmDiscard,
                                "No",
                                &render,
                            );
                        });
                });
                });
        });
    spawn_secrets_overlays(&mut commands, &render, &config.0);
    spawn_go_live_confirmation(&mut commands, &render);
    spawn_town_dialogs(&mut commands, &render, &town_catalog.entries());
}

pub(crate) fn spawn_town_dialog_button(
    parent: &mut ChildSpawnerCommands,
    action: TownDialogAction,
    label: &str,
    render: &RenderAssets,
    tab_index: i32,
) {
    parent
        .spawn((
            action,
            Button,
            TabIndex(tab_index),
            authored_ui_image(
                render,
                MAIN_MENU_TEXTURE_PATHS[0],
                main_menu_texture(render, MAIN_MENU_TEXTURE_PATHS[0]),
            ),
            Node {
                width: percent(48.0),
                height: px(52),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(Color::srgb(0.827, 0.745, 0.498)),
                Pickable::IGNORE,
            ));
        });
}

pub(crate) fn spawn_town_dialogs(
    commands: &mut Commands,
    render: &RenderAssets,
    saves: &[TownSaveEntry],
) {
    for page in [MenuPage::NewTown, MenuPage::LoadTown] {
        commands
            .spawn((
                StateEntity,
                TownDialogRoot(page),
                Visibility::Hidden,
                GlobalZIndex(185),
                BackgroundColor(Color::srgba(0.004, 0.006, 0.012, 0.88)),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0),
                    top: px(0),
                    width: percent(100.0),
                    height: percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|overlay| {
                overlay
                    .spawn((
                        TabGroup::default(),
                        BackgroundColor(Color::srgb(0.035, 0.046, 0.09)),
                        BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                        Node {
                            width: percent(52.0),
                            max_width: px(840),
                            min_height: px(390),
                            max_height: percent(82.0),
                            border: UiRect::all(px(3)),
                            border_radius: BorderRadius::all(px(34)),
                            padding: UiRect::all(px(42)),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(20),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                    ))
                    .with_children(|panel| {
                        panel.spawn((
                            settings_panel_ui_image(
                                render,
                                SETTINGS_BACKGROUND_TEXTURE_PATH,
                                main_menu_texture(render, SETTINGS_BACKGROUND_TEXTURE_PATH),
                            ),
                            Pickable::IGNORE,
                            Node {
                                position_type: PositionType::Absolute,
                                left: px(3),
                                top: px(3),
                                right: px(3),
                                bottom: px(3),
                                ..default()
                            },
                        ));
                        panel.spawn((
                            UiDisplayFont,
                            Text::new(if page == MenuPage::NewTown {
                                "NAME YOUR TOWN"
                            } else {
                                "LOAD A TOWN"
                            }),
                            TextFont {
                                font_size: FontSize::Px(32.0),
                                ..default()
                            },
                            TextLayout::justify(Justify::Center),
                            TextColor(Color::WHITE),
                            Pickable::IGNORE,
                            Node {
                                width: percent(100.0),
                                ..default()
                            },
                        ));
                        if page == MenuPage::NewTown {
                            panel.spawn((
                                Text::new("The name determines the deterministic world seed and its independent save file."),
                                TextFont {
                                    font_size: FontSize::Px(17.0),
                                    ..default()
                                },
                                TextLayout::justify(Justify::Center),
                                TextColor(Color::srgb(0.84, 0.88, 0.94)),
                                Pickable::IGNORE,
                            ));
                            panel.spawn((
                                TownNameField,
                                EditableText {
                                    max_characters: Some(80),
                                    ..EditableText::new("")
                                },
                                SelectAllOnFocus,
                                TabIndex(0),
                                TextCursorStyle {
                                    color: Color::WHITE,
                                    selected_text_color: Some(Color::BLACK),
                                    ..default()
                                },
                                TextFont {
                                    font_size: FontSize::Px(24.0),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                BackgroundColor(Color::srgb(0.018, 0.026, 0.055)),
                                BorderColor::all(Color::srgb(0.40, 0.49, 0.67)),
                                Node {
                                    width: percent(100.0),
                                    height: px(56),
                                    border: UiRect::all(px(2)),
                                    border_radius: BorderRadius::all(px(8)),
                                    padding: UiRect::axes(px(14), px(10)),
                                    overflow: Overflow::clip_x(),
                                    ..default()
                                },
                            ));
                            panel
                                .spawn(Node {
                                    width: percent(100.0),
                                    height: px(52),
                                    justify_content: JustifyContent::SpaceBetween,
                                    ..default()
                                })
                                .with_children(|buttons| {
                                    spawn_town_dialog_button(
                                        buttons,
                                        TownDialogAction::Back,
                                        "Back",
                                        render,
                                        1,
                                    );
                                    spawn_town_dialog_button(
                                        buttons,
                                        TownDialogAction::Create,
                                        "Create Town",
                                        render,
                                        2,
                                    );
                                });
                        } else {
                            panel
                                .spawn((
                                    TownLoadList,
                                    ScrollPosition::default(),
                                    Node {
                                        width: percent(100.0),
                                        flex_grow: 1.0,
                                        flex_direction: FlexDirection::Column,
                                        row_gap: px(8),
                                        overflow: Overflow::scroll_y(),
                                        ..default()
                                    },
                                ))
                                .with_children(|list| {
                                    if saves.is_empty() {
                                        list.spawn((
                                            Text::new("No saved towns were found."),
                                            TextFont {
                                                font_size: FontSize::Px(19.0),
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    }
                                    for (index, save) in saves.iter().enumerate() {
                                        list.spawn((
                                            TownLoadChoice {
                                                name: save.name.clone(),
                                                path: save.path.clone(),
                                                protected: save.protected,
                                            },
                                            Button,
                                            TabIndex(i32::try_from(index).unwrap_or(i32::MAX)),
                                            authored_ui_image(
                                                render,
                                                MAIN_MENU_TEXTURE_PATHS[0],
                                                main_menu_texture(render, MAIN_MENU_TEXTURE_PATHS[0]),
                                            ),
                                            Node {
                                                width: percent(100.0),
                                                height: px(45),
                                                flex_shrink: 0.0,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                        ))
                                        .with_children(|button| {
                                            button.spawn((
                                                Text::new(save.name.clone()),
                                                TextFont {
                                                    font_size: FontSize::Px(18.0),
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.827, 0.745, 0.498)),
                                                Pickable::IGNORE,
                                            ));
                                        });
                                    }
                                });
                            panel
                                .spawn(Node {
                                    width: percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                })
                                .with_children(|buttons| {
                                    spawn_town_dialog_button(
                                        buttons,
                                        TownDialogAction::Back,
                                        "Back",
                                        render,
                                        i32::try_from(saves.len()).unwrap_or(i32::MAX),
                                    );
                                });
                        }
                        panel.spawn((
                            TownDialogFeedback,
                            Text::new(""),
                            TextFont {
                                font_size: FontSize::Px(15.0),
                                ..default()
                            },
                            TextLayout::justify(Justify::Center),
                            TextColor(Color::srgb(1.0, 0.70, 0.38)),
                            Pickable::IGNORE,
                            Node {
                                width: percent(100.0),
                                min_height: px(20),
                                ..default()
                            },
                        ));
                    });
            });
    }
}

pub(crate) fn spawn_go_live_confirmation(commands: &mut Commands, render: &RenderAssets) {
    commands
        .spawn((
            StateEntity,
            GoLiveConfirmationRoot,
            Name::new("Go-live town-start confirmation"),
            Visibility::Hidden,
            GlobalZIndex(190),
            BackgroundColor(Color::srgba(0.004, 0.006, 0.012, 0.86)),
            Node {
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                width: percent(100.0),
                height: percent(100.0),
                padding: UiRect::all(px(24)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Name::new("Go-live confirmation panel"),
                    TabGroup::default(),
                    Node {
                        width: percent(54.0),
                        max_width: px(860),
                        min_height: px(330),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(30)),
                        padding: UiRect::all(px(42)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(24),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.035, 0.046, 0.09)),
                    BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Name::new("Go-live confirmation nine-slice surface"),
                        settings_panel_ui_image(
                            render,
                            SETTINGS_BACKGROUND_TEXTURE_PATH,
                            main_menu_texture(render, SETTINGS_BACKGROUND_TEXTURE_PATH),
                        ),
                        Pickable::IGNORE,
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(3),
                            top: px(3),
                            right: px(3),
                            bottom: px(3),
                            ..default()
                        },
                    ));
                    panel.spawn((
                        Text::new("GO LIVE WITH THIS TOWN?"),
                        TextFont {
                            font_size: FontSize::Px(32.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::WHITE),
                        Pickable::IGNORE,
                    ));
                    panel.spawn((
                        GoLiveConfirmationBody,
                        Text::new("Starting this town will begin the internal Twitch stream once loading is complete. Do you agree to go live?"),
                        TextFont {
                            font_size: FontSize::Px(21.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            linebreak: LineBreak::WordBoundary,
                        },
                        TextColor(Color::srgb(0.94, 0.94, 0.91)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(92.0),
                            ..default()
                        },
                    ));
                    panel
                        .spawn(Node {
                            width: percent(74.0),
                            height: px(56),
                            column_gap: px(24),
                            ..default()
                        })
                        .with_children(|buttons| {
                            for (action, label, tab_index) in [
                                (GoLiveConfirmationAction::No, "No — stay offline", 0),
                                (GoLiveConfirmationAction::Yes, "Yes — go live", 1),
                            ] {
                                buttons
                                    .spawn((
                                        action,
                                        Button,
                                        TabIndex(tab_index),
                                        authored_ui_image(
                                            render,
                                            MAIN_MENU_TEXTURE_PATHS[0],
                                            main_menu_texture(render, MAIN_MENU_TEXTURE_PATHS[0]),
                                        ),
                                        Node {
                                            flex_grow: 1.0,
                                            height: percent(100.0),
                                            padding: UiRect::horizontal(px(14)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                    ))
                                    .with_child((
                                        Text::new(label),
                                        TextFont {
                                            font_size: FontSize::Px(17.0),
                                            ..default()
                                        },
                                        TextLayout::justify(Justify::Center),
                                        TextColor(Color::srgb(0.827, 0.745, 0.498)),
                                        Pickable::IGNORE,
                                    ));
                            }
                        });
                });
        });
}

pub(crate) fn spawn_secrets_overlays(
    commands: &mut Commands,
    render: &RenderAssets,
    config: &GameConfig,
) {
    commands
        .spawn((
            StateEntity,
            SecretsDisclaimerRoot,
            Name::new("Twitch secrets disclaimer"),
            Visibility::Hidden,
            GlobalZIndex(200),
            BackgroundColor(Color::srgba(0.004, 0.006, 0.012, 0.96)),
            Node {
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                width: percent(100.0),
                height: percent(100.0),
                padding: UiRect::all(px(24)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Name::new("Twitch secrets disclaimer panel"),
                    TabGroup::default(),
                    BackgroundColor(Color::srgb(0.035, 0.046, 0.09)),
                    BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                    Node {
                        width: percent(66.0),
                        max_width: px(1_050),
                        min_height: percent(52.0),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(34)),
                        padding: UiRect::all(px(48)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(22),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Name::new("Twitch secrets disclaimer nine-slice surface"),
                        settings_panel_ui_image(
                            render,
                            SETTINGS_BACKGROUND_TEXTURE_PATH,
                            main_menu_texture(render, SETTINGS_BACKGROUND_TEXTURE_PATH),
                        ),
                        Pickable::IGNORE,
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(3),
                            top: px(3),
                            right: px(3),
                            bottom: px(3),
                            ..default()
                        },
                    ));
                    panel.spawn((
                        Text::new("SENSITIVE TWITCH SETUP"),
                        TextFont {
                            font_size: FontSize::Px(34.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::WHITE),
                        Pickable::IGNORE,
                    ));
                    panel.spawn((
                        Text::new(
                            "Both Twitch accounts must be connected before starting a town. Complete bot and stream authorization in this menu.",
                        ),
                        TextFont {
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            linebreak: LineBreak::WordBoundary,
                        },
                        TextColor(Color::srgb(1.0, 0.78, 0.34)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(92.0),
                            ..default()
                        },
                    ));
                    panel.spawn((
                        Text::new(SECRETS_DISCLAIMER),
                        TextFont {
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            linebreak: LineBreak::WordBoundary,
                        },
                        TextColor(Color::srgb(0.94, 0.94, 0.91)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(92.0),
                            ..default()
                        },
                    ));
                    panel.spawn((
                        Text::new("Do you agree and want to continue?"),
                        TextFont {
                            font_size: FontSize::Px(22.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::srgb(0.94, 0.80, 0.43)),
                        Pickable::IGNORE,
                    ));
                    panel
                        .spawn(Node {
                            width: percent(72.0),
                            height: px(54),
                            column_gap: px(24),
                            ..default()
                        })
                        .with_children(|buttons| {
                            spawn_secrets_button(
                                buttons,
                                SecretsAction::DisclaimerNo,
                                config,
                                render,
                                0,
                            );
                            spawn_secrets_button(
                                buttons,
                                SecretsAction::DisclaimerYes,
                                config,
                                render,
                                1,
                            );
                        });
                });
        });

    commands
        .spawn((
            StateEntity,
            SecretsRoot,
            Name::new("Twitch secrets setup"),
            Visibility::Hidden,
            GlobalZIndex(210),
            BackgroundColor(Color::srgb(0.004, 0.006, 0.012)),
            Node {
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                width: percent(100.0),
                height: percent(100.0),
                padding: UiRect::all(px(22)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Name::new("Twitch secrets setup panel"),
                    TabGroup::default(),
                    BackgroundColor(Color::srgb(0.035, 0.046, 0.09)),
                    BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                    Node {
                        width: percent(88.0),
                        height: percent(92.0),
                        max_width: px(1_520),
                        max_height: px(940),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(36)),
                        padding: UiRect::all(px(38)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(14),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Name::new("Twitch secrets setup nine-slice surface"),
                        settings_panel_ui_image(
                            render,
                            SETTINGS_BACKGROUND_TEXTURE_PATH,
                            main_menu_texture(render, SETTINGS_BACKGROUND_TEXTURE_PATH),
                        ),
                        Pickable::IGNORE,
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(3),
                            top: px(3),
                            right: px(3),
                            bottom: px(3),
                            ..default()
                        },
                    ));
                    panel.spawn((
                        Text::new("TWITCH SECRETS"),
                        TextFont {
                            font_size: FontSize::Px(32.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::WHITE),
                        Pickable::IGNORE,
                    ));
                    panel.spawn((
                        Text::new(SECRETS_PRIVACY_NOTICE),
                        TextFont {
                            font_size: FontSize::Px(17.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::srgb(1.0, 0.72, 0.28)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(100.0),
                            ..default()
                        },
                    ));
                    panel
                        .spawn(Node {
                            width: percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(10),
                            ..default()
                        })
                        .with_children(|fields| {
                            spawn_secrets_field(
                                fields,
                                SecretsField::ClientId,
                                "Twitch application Client ID",
                                &config.twitch.client_id,
                                0,
                            );
                            spawn_secrets_field(
                                fields,
                                SecretsField::BotLogin,
                                "Bot account login",
                                &config.twitch.bot_login,
                                1,
                            );
                            spawn_secrets_field(
                                fields,
                                SecretsField::ChannelLogin,
                                "Broadcaster/channel login",
                                &config.twitch.channel_login,
                                2,
                            );
                        });
                    panel
                        .spawn(Node {
                            width: percent(100.0),
                            flex_grow: 1.0,
                            column_gap: px(20),
                            ..default()
                        })
                        .with_children(|columns| {
                            spawn_secrets_account_column(
                                columns,
                                "CHAT BOT ACCOUNT",
                                "Authorizes chat:read and chat:edit for the bot login above. The token is stored only in the operating-system credential vault.",
                                SecretsConnectionKind::Bot,
                                SecretsAction::ToggleBot,
                                SecretsAction::AuthorizeBot,
                                config,
                                render,
                                3,
                            );
                            spawn_secrets_account_column(
                                columns,
                                "BROADCASTER / STREAM ACCOUNT",
                                "Separately authorizes stream-key access, operator chat output, and moderation for the broadcaster login above. The stream key is never saved; operator messages, timeouts, and bans all come from this streamer account, never the chat bot.",
                                SecretsConnectionKind::Broadcast,
                                SecretsAction::ToggleBroadcast,
                                SecretsAction::AuthorizeBroadcaster,
                                config,
                                render,
                                5,
                            );
                        });
                    panel.spawn((
                        Text::new("Bandwidth test sends the complete encoded stream to Twitch without making the channel live. Use it to verify bitrate and stability, then disable it before a real stream."),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            linebreak: LineBreak::WordBoundary,
                        },
                        TextColor(Color::srgb(0.76, 0.82, 0.92)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(100.0),
                            min_height: px(34),
                            ..default()
                        },
                    ));
                    panel
                        .spawn(Node {
                            width: percent(100.0),
                            min_height: px(46),
                            column_gap: px(14),
                            ..default()
                        })
                        .with_children(|buttons| {
                            spawn_secrets_button(
                                buttons,
                                SecretsAction::ToggleBandwidthTest,
                                config,
                                render,
                                7,
                            );
                            spawn_secrets_button(
                                buttons,
                                SecretsAction::Save,
                                config,
                                render,
                                8,
                            );
                            spawn_secrets_button(
                                buttons,
                                SecretsAction::Back,
                                config,
                                render,
                                9,
                            );
                        });
                    panel.spawn((
                        SecretsDeviceText,
                        Text::new("No Twitch authorization is in progress."),
                        TextFont {
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::srgb(0.66, 0.86, 1.0)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(100.0),
                            min_height: px(48),
                            ..default()
                        },
                    ));
                    panel.spawn((
                        SecretsStatusText,
                        Text::new(SECRETS_INITIAL_FEEDBACK),
                        TextFont {
                            font_size: FontSize::Px(16.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::srgb(0.86, 0.92, 0.88)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(100.0),
                            min_height: px(34),
                            ..default()
                        },
                    ));
                });
        });
}

pub(crate) fn spawn_secrets_field(
    parent: &mut ChildSpawnerCommands,
    field: SecretsField,
    label: &'static str,
    value: &str,
    tab_index: i32,
) {
    parent
        .spawn(Node {
            width: percent(100.0),
            height: px(48),
            column_gap: px(18),
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(17.0),
                    ..default()
                },
                TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                Pickable::IGNORE,
                Node {
                    width: percent(31.0),
                    ..default()
                },
            ));
            row.spawn((
                field,
                EditableText {
                    max_characters: Some(128),
                    ..EditableText::new(value)
                },
                SelectAllOnFocus,
                TabIndex(tab_index),
                TextCursorStyle {
                    color: Color::WHITE,
                    selected_text_color: Some(Color::BLACK),
                    ..default()
                },
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                BackgroundColor(Color::srgb(0.018, 0.026, 0.055)),
                BorderColor::all(Color::srgb(0.40, 0.49, 0.67)),
                Node {
                    width: percent(69.0),
                    height: px(44),
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(px(8)),
                    padding: UiRect::axes(px(12), px(8)),
                    overflow: Overflow::clip_x(),
                    ..default()
                },
            ));
        });
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_secrets_account_column(
    parent: &mut ChildSpawnerCommands,
    heading: &'static str,
    description: &'static str,
    connection: SecretsConnectionKind,
    toggle: SecretsAction,
    authorize: SecretsAction,
    config: &GameConfig,
    render: &RenderAssets,
    tab_index: i32,
) {
    parent
        .spawn((
            BackgroundColor(Color::srgba(0.018, 0.026, 0.055, 0.88)),
            BorderColor::all(Color::srgb(0.32, 0.39, 0.56)),
            Node {
                width: percent(50.0),
                border: UiRect::all(px(2)),
                border_radius: BorderRadius::all(px(14)),
                padding: UiRect::all(px(18)),
                flex_direction: FlexDirection::Column,
                row_gap: px(12),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .with_children(|column| {
            column.spawn((
                Text::new(heading),
                TextFont {
                    font_size: FontSize::Px(21.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            column.spawn((
                Text::new(description),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextLayout {
                    justify: Justify::Center,
                    linebreak: LineBreak::WordBoundary,
                },
                TextColor(Color::srgb(0.82, 0.86, 0.91)),
                Pickable::IGNORE,
            ));
            column.spawn((
                SecretsConnectionText(connection),
                Text::new("● Checking status..."),
                TextFont {
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextLayout {
                    justify: Justify::Center,
                    linebreak: LineBreak::WordBoundary,
                },
                TextColor(Color::srgb(0.88, 0.78, 0.46)),
                Pickable::IGNORE,
                Node {
                    min_height: px(38),
                    ..default()
                },
            ));
            spawn_secrets_button(column, toggle, config, render, tab_index);
            spawn_secrets_button(column, authorize, config, render, tab_index + 1);
        });
}

pub(crate) fn spawn_secrets_button(
    parent: &mut ChildSpawnerCommands,
    action: SecretsAction,
    config: &GameConfig,
    render: &RenderAssets,
    tab_index: i32,
) {
    parent
        .spawn((
            action,
            Button,
            TabIndex(tab_index),
            authored_ui_image(
                render,
                MAIN_MENU_TEXTURE_PATHS[0],
                main_menu_texture(render, MAIN_MENU_TEXTURE_PATHS[0]),
            ),
            Node {
                min_width: px(150),
                flex_grow: 1.0,
                height: px(48),
                padding: UiRect::axes(px(16), px(9)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|button| {
            button.spawn((
                SecretsDynamicLabel(action),
                Text::new(secrets_action_label(action, config)),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                Pickable::IGNORE,
            ));
        });
}

pub(crate) fn secrets_action_label(action: SecretsAction, config: &GameConfig) -> String {
    match action {
        SecretsAction::DisclaimerYes => "Yes — continue".to_owned(),
        SecretsAction::DisclaimerNo => "No — go back".to_owned(),
        SecretsAction::ToggleBot => format!(
            "Chat bot: {}",
            if config.twitch.enabled {
                "Enabled"
            } else {
                "Disabled"
            }
        ),
        SecretsAction::ToggleBroadcast => format!(
            "Direct stream: {}",
            if config.twitch.broadcast.enabled {
                "Enabled"
            } else {
                "Disabled"
            }
        ),
        SecretsAction::ToggleBandwidthTest => format!(
            "Bandwidth test: {}",
            if config.twitch.broadcast.bandwidth_test {
                "Enabled"
            } else {
                "Disabled"
            }
        ),
        SecretsAction::Save => "Save and apply".to_owned(),
        SecretsAction::AuthorizeBot => "Authorize bot account".to_owned(),
        SecretsAction::AuthorizeBroadcaster => "Authorize stream account".to_owned(),
        SecretsAction::Back => "Back".to_owned(),
    }
}

pub(crate) fn secrets_buttons(
    buttons: Query<(&Interaction, &SecretsAction), Changed<Interaction>>,
    fields: Query<(&SecretsField, &EditableText)>,
    mut menu: ResMut<MenuRuntime>,
    mut secrets: ResMut<SecretsRuntime>,
    mut config: ResMut<RuntimeConfig>,
    mut connection: ResMut<TwitchConnection>,
    mut focus: ResMut<InputFocus>,
) {
    for (interaction, action) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            SecretsAction::DisclaimerNo if menu.page == MenuPage::SecretsDisclaimer => {
                menu.page = MenuPage::Closed;
                focus.clear();
            }
            SecretsAction::DisclaimerYes if menu.page == MenuPage::SecretsDisclaimer => {
                menu.page = MenuPage::Secrets;
                "Internal Twitch video is now blacked out.".clone_into(&mut secrets.feedback);
                focus.clear();
            }
            _ if menu.page != MenuPage::Secrets => {}
            SecretsAction::ToggleBot => {
                let enabled = !config.0.twitch.enabled;
                let outcome = save_secrets_fields(&fields, &mut config, |draft| {
                    draft.twitch.enabled = enabled;
                });
                if outcome.is_ok() {
                    restart_twitch_connection(&config.0, &mut connection);
                }
                set_secrets_save_feedback(outcome, &mut secrets, &mut connection);
            }
            SecretsAction::ToggleBroadcast => {
                let enabled = !config.0.twitch.broadcast.enabled;
                let outcome = save_secrets_fields(&fields, &mut config, |draft| {
                    draft.twitch.broadcast.enabled = enabled;
                });
                set_secrets_save_feedback(outcome, &mut secrets, &mut connection);
            }
            SecretsAction::ToggleBandwidthTest => {
                let enabled = !config.0.twitch.broadcast.bandwidth_test;
                let outcome = save_secrets_fields(&fields, &mut config, |draft| {
                    draft.twitch.broadcast.bandwidth_test = enabled;
                });
                set_secrets_save_feedback(outcome, &mut secrets, &mut connection);
            }
            SecretsAction::Save => {
                let previous = config.0.twitch.clone();
                match save_secrets_fields(&fields, &mut config, |_| {}) {
                    Ok(path) => {
                        let (restart_chat, broadcast_settings_changed) =
                            secrets_restart_requirements(&previous, &config.0.twitch);
                        if restart_chat {
                            restart_twitch_connection(&config.0, &mut connection);
                        }
                        secrets.feedback = if restart_chat {
                            format!(
                                "Saved and applied {}. The bot connection is restarting; the stream remains under manual control.",
                                path.display()
                            )
                        } else if broadcast_settings_changed {
                            format!(
                                "Saved and applied {}. Stream settings will be used the next time you choose Go Live.",
                                path.display()
                            )
                        } else {
                            format!(
                                "Saved and applied {}. Active Twitch connections were preserved.",
                                path.display()
                            )
                        };
                    }
                    Err(error) => {
                        set_secrets_save_feedback(Err(error), &mut secrets, &mut connection);
                    }
                }
            }
            SecretsAction::AuthorizeBot | SecretsAction::AuthorizeBroadcaster => {
                if let Some(kind) = secrets.active_authorization {
                    secrets.feedback = format!(
                        "Finish the current {} authorization before starting another.",
                        kind.label()
                    );
                    continue;
                }
                let kind = if *action == SecretsAction::AuthorizeBot {
                    SecretsAuthorizationKind::Bot
                } else {
                    SecretsAuthorizationKind::Broadcaster
                };
                match save_secrets_fields(&fields, &mut config, |_| {}) {
                    Ok(path) => {
                        secrets.feedback = format!(
                            "Saved {}. Requesting a Twitch {} device code...",
                            path.display(),
                            kind.label()
                        );
                        start_secrets_authorization(kind, config.0.twitch.clone(), &mut secrets);
                    }
                    Err(error) => {
                        secrets.feedback = format!("Cannot authorize: {error:#}");
                    }
                }
            }
            SecretsAction::Back => {
                menu.page = MenuPage::Closed;
                focus.clear();
            }
            SecretsAction::DisclaimerNo | SecretsAction::DisclaimerYes => {}
        }
    }
}

pub(crate) fn save_secrets_fields(
    fields: &Query<(&SecretsField, &EditableText)>,
    config: &mut RuntimeConfig,
    mutate: impl FnOnce(&mut GameConfig),
) -> AnyResult<PathBuf> {
    let mut draft = config.0.clone();
    for (field, editable) in fields {
        let value = editable.value().to_string();
        match field {
            SecretsField::ClientId => value.trim().clone_into(&mut draft.twitch.client_id),
            SecretsField::BotLogin => {
                draft.twitch.bot_login = value.trim().to_ascii_lowercase();
            }
            SecretsField::ChannelLogin => {
                draft.twitch.channel_login = value.trim().to_ascii_lowercase();
            }
        }
    }
    mutate(&mut draft);
    let path = save_runtime_config(&draft)?;
    config.0 = draft;
    Ok(path)
}

pub(crate) fn secrets_restart_requirements(
    previous: &stream_town_domain::TwitchConfig,
    current: &stream_town_domain::TwitchConfig,
) -> (bool, bool) {
    let client_changed = previous.client_id != current.client_id;
    let channel_changed = previous.channel_login != current.channel_login;
    let chat_changed = client_changed || channel_changed || previous.bot_login != current.bot_login;
    let broadcast_changed = client_changed || channel_changed;
    (chat_changed, broadcast_changed)
}

pub(crate) fn set_secrets_save_feedback(
    outcome: AnyResult<PathBuf>,
    secrets: &mut SecretsRuntime,
    connection: &mut TwitchConnection,
) {
    match outcome {
        Ok(path) => {
            secrets.feedback = format!("Saved and applied {}", path.display());
        }
        Err(error) => {
            secrets.feedback = format!("Twitch setup was not saved: {error:#}");
            if matches!(connection.status, TwitchStatus::Authorizing) {
                connection.status = TwitchStatus::Disabled;
            }
        }
    }
}

pub(crate) fn restart_twitch_connection(config: &GameConfig, connection: &mut TwitchConnection) {
    if let Some(transport) = connection.transport.take() {
        let _ = transport.send(TwitchControl::Disconnect);
    }
    if !config.twitch.enabled {
        connection.status = TwitchStatus::Disabled;
        connection.moderation_status = TwitchModerationStatus::Disabled;
        return;
    }
    connection
        .fish_god_reward_id
        .clone_from(&config.twitch.fish_god_reward_id);
    connection.status = TwitchStatus::Authorizing;
    connection.moderation_status = TwitchModerationStatus::Authorizing;
    match TwitchTransport::start(config.twitch.clone()) {
        Ok(transport) => connection.transport = Some(transport),
        Err(error) => {
            connection.status = TwitchStatus::Error(error.to_string());
            connection.moderation_status = TwitchModerationStatus::Disabled;
        }
    }
}

pub(crate) fn reload_twitch_moderation(config: &GameConfig, connection: &mut TwitchConnection) {
    connection.moderation_status = TwitchModerationStatus::Authorizing;
    let reload_sent = connection
        .transport
        .as_ref()
        .is_some_and(|transport| transport.send(TwitchControl::ReloadModeration).is_ok());
    if !reload_sent {
        restart_twitch_connection(config, connection);
    }
}

pub(crate) fn start_secrets_authorization(
    kind: SecretsAuthorizationKind,
    config: stream_town_domain::TwitchConfig,
    secrets: &mut SecretsRuntime,
) {
    let (sender, receiver) = mpsc::channel();
    secrets.authorization_events = Some(Arc::new(Mutex::new(receiver)));
    secrets.active_authorization = Some(kind);
    secrets.device = None;
    let spawn = thread::Builder::new()
        .name(format!("stream-town-menu-{}-oauth", kind.label()))
        .spawn(move || {
            let outcome = (|| -> AnyResult<()> {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .context("failed to construct Twitch authorization runtime")?;
                runtime.block_on(async {
                    let oauth = match kind {
                        SecretsAuthorizationKind::Bot => {
                            OAuthClient::new(config.client_id.clone())?
                        }
                        SecretsAuthorizationKind::Broadcaster => {
                            OAuthClient::broadcaster(config.client_id.clone())?
                        }
                    };
                    let authorization = oauth.begin_device_authorization().await?;
                    sender
                        .send(SecretsAuthorizationEvent::Device {
                            kind,
                            authorization: authorization.clone(),
                        })
                        .context("Twitch secrets screen closed")?;
                    let token = oauth.complete_device_authorization(&authorization).await?;
                    let validation = oauth.validate(&token).await?;
                    let expected_login = match kind {
                        SecretsAuthorizationKind::Bot => &config.bot_login,
                        SecretsAuthorizationKind::Broadcaster => &config.channel_login,
                    };
                    anyhow::ensure!(
                        validation.login == *expected_login,
                        "authorized account '{}' does not match configured {} '{}'",
                        validation.login,
                        kind.label(),
                        expected_login
                    );
                    match kind {
                        SecretsAuthorizationKind::Bot => {
                            CredentialVault::new(&config.client_id, &config.bot_login)
                                .save(&token)?;
                        }
                        SecretsAuthorizationKind::Broadcaster => {
                            CredentialVault::broadcaster(&config.client_id, &config.channel_login)
                                .save(&token)?;
                        }
                    }
                    sender
                        .send(SecretsAuthorizationEvent::Authorized { kind, validation })
                        .context("Twitch secrets screen closed")?;
                    Ok(())
                })
            })();
            if let Err(error) = outcome {
                let _ = sender.send(SecretsAuthorizationEvent::Error {
                    kind,
                    message: format!("{error:#}"),
                });
            }
        });
    if let Err(error) = spawn {
        secrets.authorization_events = None;
        secrets.active_authorization = None;
        secrets.feedback = format!(
            "Could not start Twitch {} authorization: {error}",
            kind.label()
        );
    }
}

pub(crate) fn open_twitch_verification_uri(uri: &str) -> AnyResult<()> {
    open_twitch_verification_uri_with(uri, |uri| {
        webbrowser::open(uri).context("the operating system rejected the browser request")
    })
}

pub(crate) fn open_twitch_verification_uri_with(
    uri: &str,
    open: impl FnOnce(&str) -> AnyResult<()>,
) -> AnyResult<()> {
    let parsed = reqwest::Url::parse(uri).context("Twitch returned an invalid verification URL")?;
    anyhow::ensure!(
        parsed.scheme() == "https",
        "Twitch returned a non-HTTPS verification URL"
    );
    let host = parsed.host_str().unwrap_or_default();
    anyhow::ensure!(
        host == "twitch.tv" || host.ends_with(".twitch.tv"),
        "Twitch returned a verification URL for an unexpected host"
    );
    open(parsed.as_str())
}

pub(crate) fn poll_secrets_authorization(
    mut secrets: ResMut<SecretsRuntime>,
    config: Res<RuntimeConfig>,
    mut connection: ResMut<TwitchConnection>,
) {
    let events = secrets
        .authorization_events
        .as_ref()
        .and_then(|receiver| receiver.lock().ok())
        .map(|receiver| receiver.try_iter().collect::<Vec<_>>())
        .unwrap_or_default();
    for event in events {
        match event {
            SecretsAuthorizationEvent::Device {
                kind,
                authorization,
            } => {
                secrets.feedback = match open_twitch_verification_uri(
                    &authorization.verification_uri,
                ) {
                    Ok(()) => format!(
                        "Opened Twitch {} verification in your browser. Enter code {} to approve it.",
                        kind.label(),
                        authorization.user_code
                    ),
                    Err(error) => format!(
                        "Could not open Twitch {} verification automatically: {error:#}. Open {} and enter code {}.",
                        kind.label(),
                        authorization.verification_uri,
                        authorization.user_code
                    ),
                };
                secrets.device = Some(authorization);
            }
            SecretsAuthorizationEvent::Authorized { kind, validation } => {
                secrets.feedback = format!(
                    "Authorized Twitch {} account '{}'. The token is stored in the OS credential vault.",
                    kind.label(),
                    validation.login
                );
                secrets.device = None;
                secrets.active_authorization = None;
                secrets.authorization_events = None;
                match kind {
                    SecretsAuthorizationKind::Bot => {
                        secrets.bot_credential = SecretsCredentialState::Stored;
                        restart_twitch_connection(&config.0, &mut connection);
                    }
                    SecretsAuthorizationKind::Broadcaster => {
                        secrets.broadcaster_credential = SecretsCredentialState::Stored;
                        reload_twitch_moderation(&config.0, &mut connection);
                    }
                }
            }
            SecretsAuthorizationEvent::Error { kind, message } => {
                secrets.feedback =
                    format!("Twitch {} authorization failed: {message}", kind.label());
                secrets.device = None;
                secrets.active_authorization = None;
                secrets.authorization_events = None;
            }
        }
    }
}

pub(crate) fn refresh_secrets_credential_state(secrets: &mut SecretsRuntime, config: &GameConfig) {
    let signature = (
        config.twitch.client_id.clone(),
        config.twitch.bot_login.clone(),
        config.twitch.channel_login.clone(),
    );
    if secrets.credential_signature.as_ref() == Some(&signature) {
        return;
    }
    secrets.bot_credential = probe_secrets_credential(
        !signature.0.trim().is_empty() && !signature.1.trim().is_empty(),
        CredentialVault::new(&signature.0, &signature.1),
    );
    secrets.broadcaster_credential = probe_secrets_credential(
        !signature.0.trim().is_empty() && !signature.2.trim().is_empty(),
        CredentialVault::broadcaster(&signature.0, &signature.2),
    );
    secrets.credential_signature = Some(signature);
}

pub(crate) fn initialize_twitch_account_state(
    config: Res<RuntimeConfig>,
    mut secrets: ResMut<SecretsRuntime>,
) {
    refresh_secrets_credential_state(&mut secrets, &config.0);
}

pub(crate) fn twitch_accounts_connected(
    config: &GameConfig,
    secrets: &SecretsRuntime,
    connection: &TwitchConnection,
) -> bool {
    config.twitch.enabled
        && config.twitch.broadcast.enabled
        && !config.twitch.client_id.trim().is_empty()
        && !config.twitch.bot_login.trim().is_empty()
        && !config.twitch.channel_login.trim().is_empty()
        && secrets.bot_credential == SecretsCredentialState::Stored
        && secrets.broadcaster_credential == SecretsCredentialState::Stored
        && connection.status == TwitchStatus::Connected
        && connection.moderation_status == TwitchModerationStatus::Ready
}

pub(crate) fn open_twitch_setup_required(menu: &mut MenuRuntime) {
    menu.page = MenuPage::SecretsDisclaimer;
    menu.return_page = MenuPage::Closed;
    menu.selected = 0;
    "Connect both Twitch accounts in Secrets before starting a town."
        .clone_into(&mut menu.feedback);
}

pub(crate) fn probe_secrets_credential(
    configured: bool,
    vault: CredentialVault,
) -> SecretsCredentialState {
    if !configured {
        return SecretsCredentialState::NotConfigured;
    }
    match vault.load() {
        Ok(Some(_)) => SecretsCredentialState::Stored,
        Ok(None) => SecretsCredentialState::Missing,
        Err(error) => SecretsCredentialState::Error(format!("{error:#}")),
    }
}

pub(crate) fn bot_connection_status(
    config: &GameConfig,
    connection: &TwitchConnection,
    credential: &SecretsCredentialState,
) -> (String, SecretsStatusTone) {
    if !config.twitch.enabled {
        return match credential {
            SecretsCredentialState::Stored => (
                "● Bot authorized; chat connection is disabled.".to_owned(),
                SecretsStatusTone::Inactive,
            ),
            SecretsCredentialState::Missing => (
                "● Bot is not authorized and chat is disabled.".to_owned(),
                SecretsStatusTone::Inactive,
            ),
            SecretsCredentialState::NotConfigured => (
                "● Enter a Client ID and bot login to configure chat.".to_owned(),
                SecretsStatusTone::Inactive,
            ),
            SecretsCredentialState::Error(error) => (
                format!("● Could not read bot authorization: {error}"),
                SecretsStatusTone::Error,
            ),
            SecretsCredentialState::Unknown => (
                "● Checking bot authorization...".to_owned(),
                SecretsStatusTone::Pending,
            ),
        };
    }
    match &connection.status {
        TwitchStatus::Connected => (
            format!(
                "● Connected automatically as @{} to #{}; commands are enabled.",
                config.twitch.bot_login, config.twitch.channel_login
            ),
            SecretsStatusTone::Good,
        ),
        TwitchStatus::Authorizing => (
            "● Validating the stored bot authorization...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        TwitchStatus::Connecting => (
            format!(
                "● Connecting the bot to #{}...",
                config.twitch.channel_login
            ),
            SecretsStatusTone::Pending,
        ),
        TwitchStatus::Reconnecting => (
            "● Bot connection was interrupted; reconnecting...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        TwitchStatus::Disconnected => (
            "● Bot is disconnected. Use Save and apply to reconnect.".to_owned(),
            SecretsStatusTone::Error,
        ),
        TwitchStatus::Error(error) => (
            format!("● Bot connection error: {error}"),
            SecretsStatusTone::Error,
        ),
        TwitchStatus::Disabled => (
            "● Bot is enabled in settings and waiting to start.".to_owned(),
            SecretsStatusTone::Pending,
        ),
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn broadcast_connection_status(
    config: &GameConfig,
    credential: &SecretsCredentialState,
    moderation: &TwitchModerationStatus,
    snapshot: &direct_broadcast::DirectBroadcastSnapshot,
) -> (String, SecretsStatusTone) {
    use direct_broadcast::DirectBroadcastPhase;

    if !config.twitch.broadcast.enabled {
        return match credential {
            SecretsCredentialState::Stored => (
                "● Broadcaster authorized; direct stream is disabled.".to_owned(),
                SecretsStatusTone::Inactive,
            ),
            SecretsCredentialState::Missing => (
                "● Broadcaster is not authorized and direct stream is disabled.".to_owned(),
                SecretsStatusTone::Inactive,
            ),
            SecretsCredentialState::NotConfigured => (
                "● Enter a Client ID and channel login to configure streaming.".to_owned(),
                SecretsStatusTone::Inactive,
            ),
            SecretsCredentialState::Error(error) => (
                format!("● Could not read broadcaster authorization: {error}"),
                SecretsStatusTone::Error,
            ),
            SecretsCredentialState::Unknown => (
                "● Checking broadcaster authorization...".to_owned(),
                SecretsStatusTone::Pending,
            ),
        };
    }
    match moderation {
        TwitchModerationStatus::Authorizing => {
            return (
                "● Validating broadcaster streaming, operator chat, and moderation authority..."
                    .to_owned(),
                SecretsStatusTone::Pending,
            );
        }
        TwitchModerationStatus::Error(error) => {
            return (
                format!(
                    "● Broadcaster authorization error: {error}. Use Authorize stream account below."
                ),
                SecretsStatusTone::Error,
            );
        }
        TwitchModerationStatus::Disabled => {
            return (
                "● Broadcaster operator controls are waiting for the Twitch connection to start."
                    .to_owned(),
                SecretsStatusTone::Pending,
            );
        }
        TwitchModerationStatus::Ready => {}
    }
    match &snapshot.phase {
        DirectBroadcastPhase::Disabled => (
            "● Direct stream is enabled and waiting to start.".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::WaitingForBroadcasterAuthorization => (
            "● Validating broadcaster authorization and fetching the stream key...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::WaitingForGameplay => (
            "● Stream prepared; it will begin after the in-game loading cover retires.".to_owned(),
            SecretsStatusTone::Good,
        ),
        DirectBroadcastPhase::ResolvingIngest => (
            "● Selecting a Twitch ingest server...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::Connecting => (
            "● Connecting the encoder to Twitch...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::VerifyingTwitch => (
            "● Encoder output reached Twitch; waiting for Twitch to confirm that the channel is publicly live...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::Broadcasting => {
            (
                format!(
                    "● LIVE (confirmed by Twitch); {:.1} captured / {:.1} output FPS (target {}), {} video / {} audio drops via {} ({})",
                    snapshot.captured_video_fps,
                    snapshot.encoded_video_fps,
                    config.twitch.broadcast.frames_per_second,
                    snapshot.dropped_video_frames,
                    snapshot.dropped_audio_frames,
                    snapshot.encoder.as_deref().unwrap_or("encoder pending"),
                    snapshot.ingest.as_deref().unwrap_or("ingest pending")
                ),
                SecretsStatusTone::Good,
            )
        }
        DirectBroadcastPhase::BandwidthTesting => (
            format!(
                "● BANDWIDTH TEST — not publicly live; {:.1} captured / {:.1} output FPS (target {}), {} video / {} audio drops via {} ({})",
                snapshot.captured_video_fps,
                snapshot.encoded_video_fps,
                config.twitch.broadcast.frames_per_second,
                snapshot.dropped_video_frames,
                snapshot.dropped_audio_frames,
                snapshot.encoder.as_deref().unwrap_or("encoder pending"),
                snapshot.ingest.as_deref().unwrap_or("ingest pending")
            ),
            SecretsStatusTone::Good,
        ),
        DirectBroadcastPhase::Reconnecting => (
            "● Twitch stream interrupted; reconnecting automatically...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::Stopping => (
            "● Stopping the previous stream...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::Stopped => (
            "● Stream stopped. Use Restart stream in the operator panel to start it again."
                .to_owned(),
            SecretsStatusTone::Error,
        ),
        DirectBroadcastPhase::Error(error) => {
            (format!("● Stream error: {error}"), SecretsStatusTone::Error)
        }
    }
}

pub(crate) fn secrets_status_color(tone: SecretsStatusTone) -> Color {
    match tone {
        SecretsStatusTone::Good => Color::srgb(0.48, 0.94, 0.58),
        SecretsStatusTone::Pending => Color::srgb(0.96, 0.78, 0.34),
        SecretsStatusTone::Inactive => Color::srgb(0.68, 0.72, 0.78),
        SecretsStatusTone::Error => Color::srgb(1.0, 0.46, 0.42),
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn update_secrets_ui(
    menu: Res<MenuRuntime>,
    config: Res<RuntimeConfig>,
    mut secrets: ResMut<SecretsRuntime>,
    connection: Res<TwitchConnection>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
    render: Res<RenderAssets>,
    mut disclaimer: Query<&mut Visibility, With<SecretsDisclaimerRoot>>,
    mut root: Query<&mut Visibility, (With<SecretsRoot>, Without<SecretsDisclaimerRoot>)>,
    mut status: Query<
        &mut Text,
        (
            With<SecretsStatusText>,
            Without<SecretsDeviceText>,
            Without<SecretsDynamicLabel>,
        ),
    >,
    mut device_text: Query<
        &mut Text,
        (
            With<SecretsDeviceText>,
            Without<SecretsStatusText>,
            Without<SecretsDynamicLabel>,
        ),
    >,
    mut connection_texts: Query<
        (&SecretsConnectionText, &mut Text, &mut TextColor),
        (
            Without<SecretsStatusText>,
            Without<SecretsDeviceText>,
            Without<SecretsDynamicLabel>,
        ),
    >,
    mut labels: Query<(&SecretsDynamicLabel, &mut Text)>,
    mut buttons: Query<(&Interaction, &SecretsAction, &mut ImageNode)>,
) {
    refresh_secrets_credential_state(&mut secrets, &config.0);
    if let Ok(mut visibility) = disclaimer.single_mut() {
        *visibility = if menu.page == MenuPage::SecretsDisclaimer {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if let Ok(mut visibility) = root.single_mut() {
        *visibility = if menu.page == MenuPage::Secrets {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if let Ok(mut text) = status.single_mut() {
        if secrets.feedback.is_empty() {
            SECRETS_INITIAL_FEEDBACK.clone_into(&mut **text);
        } else {
            (**text).clone_from(&secrets.feedback);
        }
    }
    if let Ok(mut text) = device_text.single_mut() {
        **text = secrets.device.as_ref().map_or_else(
            || "No Twitch authorization is in progress.".to_owned(),
            |device| {
                format!(
                    "Open {} and enter code {} (expires in {} minutes).",
                    device.verification_uri,
                    device.user_code,
                    (device.expires_in / 60).max(1)
                )
            },
        );
    }
    for (kind, mut text, mut color) in &mut connection_texts {
        let (status, tone) = match kind.0 {
            SecretsConnectionKind::Bot => {
                bot_connection_status(&config.0, &connection, &secrets.bot_credential)
            }
            SecretsConnectionKind::Broadcast => {
                #[cfg(target_os = "windows")]
                {
                    broadcast_connection_status(
                        &config.0,
                        &secrets.broadcaster_credential,
                        &connection.moderation_status,
                        &broadcast.snapshot(),
                    )
                }
                #[cfg(not(target_os = "windows"))]
                {
                    (
                        "● Direct Twitch streaming is currently available only on Windows."
                            .to_owned(),
                        SecretsStatusTone::Inactive,
                    )
                }
            }
        };
        **text = status;
        color.0 = secrets_status_color(tone);
    }
    for (label, mut text) in &mut labels {
        **text = secrets_action_label(label.0, &config.0);
    }
    for (interaction, action, mut image) in &mut buttons {
        let available = secrets.active_authorization.is_none()
            || !matches!(
                action,
                SecretsAction::AuthorizeBot | SecretsAction::AuthorizeBroadcaster
            );
        let source_path = if !available {
            MAIN_MENU_TEXTURE_PATHS[2]
        } else if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            MAIN_MENU_TEXTURE_PATHS[1]
        } else {
            MAIN_MENU_TEXTURE_PATHS[0]
        };
        image.image = main_menu_texture(&render, source_path);
        image.color = if available {
            Color::WHITE
        } else {
            Color::srgba(0.78, 0.78, 0.78, 0.5)
        };
    }
}

pub(crate) fn sync_sensitive_screen_active(
    menu: Res<MenuRuntime>,
    mut active: ResMut<SensitiveScreenActive>,
) {
    active.0 = menu_page_is_sensitive(menu.page);
}

pub(crate) const fn menu_page_is_sensitive(page: MenuPage) -> bool {
    matches!(page, MenuPage::SecretsDisclaimer | MenuPage::Secrets)
}

pub(crate) fn update_menu_overlay(
    state: Res<State<GameState>>,
    menu: Res<MenuRuntime>,
    save: Res<SaveRuntime>,
    mut overlay: Query<(&mut Text, &mut Visibility), With<MenuOverlay>>,
    mut game_menu: Query<&mut Visibility, (With<GameMenuRoot>, Without<MenuOverlay>)>,
) {
    let Ok((mut text, mut visibility)) = overlay.single_mut() else {
        return;
    };
    let image_game_menu_visible = menu.page == MenuPage::Game && *state.get() == GameState::InGame;
    if let Ok(mut game_menu_visibility) = game_menu.single_mut() {
        *game_menu_visibility = if image_game_menu_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if matches!(
        menu.page,
        MenuPage::Closed
            | MenuPage::GoLiveConfirmation
            | MenuPage::NewTown
            | MenuPage::LoadTown
            | MenuPage::Settings
            | MenuPage::SecretsDisclaimer
            | MenuPage::Secrets
    ) || image_game_menu_visible
    {
        *visibility = Visibility::Hidden;
        return;
    }
    **text = match menu.page {
        MenuPage::Game => game_menu_text(*state.get(), menu.selected, save.store.path().is_file()),
        MenuPage::Settings => settings_menu_text(
            &menu.draft,
            &menu.streaming_draft,
            menu.selected,
            &menu.feedback,
        ),
        MenuPage::Closed
        | MenuPage::GoLiveConfirmation
        | MenuPage::NewTown
        | MenuPage::LoadTown
        | MenuPage::SecretsDisclaimer
        | MenuPage::Secrets => String::new(),
    };
    *visibility = Visibility::Visible;
}

pub(crate) fn sync_cursor_visibility(
    state: Res<State<GameState>>,
    menu: Res<MenuRuntime>,
    mut cursors: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    let visible = cursor_visible_for_state(*state.get(), menu.page);
    for mut cursor in &mut cursors {
        cursor.visible = visible;
    }
}

pub(crate) const fn cursor_visible_for_state(state: GameState, menu_page: MenuPage) -> bool {
    match state {
        GameState::MainMenu | GameState::Credits => true,
        GameState::InGame => !matches!(menu_page, MenuPage::Closed),
        GameState::Boot | GameState::WorldLoading => false,
    }
}

pub(crate) fn game_menu_text(state: GameState, selected: usize, has_save: bool) -> String {
    use std::fmt::Write as _;

    let items: &[(&str, bool)] = if state == GameState::InGame {
        &[
            ("Save Game", true),
            ("Save Jump-Start", true),
            ("Load Game", has_save),
            ("Settings", true),
            ("Go Live", true),
            ("Exit Game", true),
        ]
    } else {
        &[
            ("New town", true),
            ("Load game", has_save),
            ("Settings", true),
            ("Credits", true),
            ("Quit", true),
        ]
    };
    let mut text = String::from("STREAM TOWN MENU\n\n");
    for (index, (label, enabled)) in items.iter().enumerate() {
        let marker = if index == selected { ">" } else { " " };
        let suffix = if *enabled { "" } else { "  [No save]" };
        writeln!(text, "{marker} {label}{suffix}").expect("writing to String cannot fail");
    }
    text.push_str("\nUP/DOWN Select   ENTER Confirm   ESC Resume/Close");
    text
}

pub(crate) fn settings_menu_text(
    settings: &PlayerSettings,
    streaming: &BroadcastConfig,
    selected: usize,
    feedback: &str,
) -> String {
    use std::fmt::Write as _;
    const COLUMN_BREAK: usize = 18;

    let video = &settings.video;
    let camera = &settings.camera;
    let interface = &settings.interface;
    let items = [
        format!("Display mode: {:?}", video.display_mode),
        format!("Resolution: {} x {}", video.width, video.height),
        format!("VSync: {}", on_off(video.vsync)),
        format!(
            "FPS limit: {}",
            video
                .fps_limit
                .map_or("Unlimited".to_owned(), |value| value.to_string())
        ),
        format!("Shadows: {}", on_off(video.shadows_enabled)),
        format!("Shadow map: {}", video.shadow_map_resolution),
        format!("Ambient occlusion: {}", on_off(video.ambient_occlusion)),
        format!("MSAA samples: {}", video.msaa_samples),
        format!("Post-process AA: {:?}", video.post_process_aa),
        format!("Brightness: {:.1}", video.brightness_ev),
        format!("Gamma: {:.1}", video.gamma),
        format!("Master volume: {}%", volume_percent(settings.audio.master)),
        format!("Music volume: {}%", volume_percent(settings.audio.music)),
        format!(
            "Sound effects: {}%",
            volume_percent(settings.audio.sound_effects)
        ),
        format!(
            "Ambience volume: {}%",
            volume_percent(settings.audio.ambience)
        ),
        format!("Zoom sensitivity: {:.0}", camera.zoom_sensitivity),
        format!("Camera field of view: {}", camera.field_of_view_degrees),
        format!("Name display: {:?}", interface.display_names),
        format!("Building health: {:?}", interface.display_building_health),
        format!(
            "Autosave: {}",
            if settings.autosave_minutes == 0 {
                "Off".to_owned()
            } else {
                format!("{} min", settings.autosave_minutes)
            }
        ),
        format!(
            "Timelapse frequency: {}",
            timelapse_interval_label(settings.timelapse.interval)
        ),
        format!("Timelapse dynamic: {}", on_off(settings.timelapse.dynamic)),
        format!("UI scale: {}%", interface.ui_scale_percent),
        format!("High contrast: {}", on_off(interface.high_contrast)),
        format!("Reduced motion: {}", on_off(interface.reduced_motion)),
        format!("Direct streaming: {}", on_off(streaming.enabled)),
        format!("Stream output: {} x {}", streaming.width, streaming.height),
        format!("Stream frame rate: {} FPS", streaming.frames_per_second),
        format!("Video bitrate: {} kbps", streaming.video_bitrate_kbps),
        format!("Audio bitrate: {} kbps", streaming.audio_bitrate_kbps),
        format!("Stream encoder: {:?}", streaming.encoder),
        format!("Bandwidth test: {}", on_off(streaming.bandwidth_test)),
        format!("Render mode: {:?}", streaming.render_mode),
        "Apply and save".to_owned(),
        "Restore defaults".to_owned(),
        "Cancel changes".to_owned(),
    ];
    let mut text = String::from("SETTINGS\n\n");
    for row in 0..COLUMN_BREAK {
        let left = settings_menu_item(&items, row, selected);
        let right = settings_menu_item(&items, row + COLUMN_BREAK, selected);
        writeln!(text, "{left:<44}{right}").expect("writing to String cannot fail");
    }
    text.push_str("\nUP/DOWN Select   LEFT/RIGHT Change   ENTER Confirm   ESC Cancel");
    if !feedback.is_empty() {
        text.push_str("\n\n");
        text.push_str(feedback);
    }
    text
}

pub(crate) fn settings_menu_item(items: &[String], index: usize, selected: usize) -> String {
    items.get(index).map_or_else(String::new, |item| {
        format!("{} {item}", if index == selected { ">" } else { " " })
    })
}

pub(crate) fn on_off(value: bool) -> &'static str {
    if value { "On" } else { "Off" }
}

pub(crate) const fn timelapse_interval_label(interval: TimelapseInterval) -> &'static str {
    match interval {
        TimelapseInterval::Off => "Off",
        TimelapseInterval::TenMinutes => "10 Minutes",
        TimelapseInterval::OneHour => "1 Hour",
        TimelapseInterval::ThreeHours => "3 Hours",
        TimelapseInterval::SixHours => "6 Hours",
        TimelapseInterval::TwelveHours => "12 Hours",
        TimelapseInterval::TwentyFourHours => "24 Hours",
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub(crate) fn volume_percent(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 100.0).round() as u8
}

pub(crate) fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    save: Res<SaveRuntime>,
    town_catalog: Res<TownSaveCatalogRuntime>,
    mut config: ResMut<RuntimeConfig>,
    secrets: Res<SecretsRuntime>,
    connection: Res<TwitchConnection>,
    mut menu: ResMut<MenuRuntime>,
    mut io: ResMut<MenuIoRequest>,
    mut player_settings: ResMut<RuntimePlayerSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
    #[cfg(target_os = "windows")] broadcast_runtime: Res<direct_broadcast::DirectBroadcastRuntime>,
    #[cfg(target_os = "windows")] mut broadcast_control: ResMut<
        direct_broadcast::DirectBroadcastControl,
    >,
) {
    #[cfg(target_os = "windows")]
    let streaming_locked = broadcast_runtime.snapshot().phase.is_active();
    #[cfg(not(target_os = "windows"))]
    let streaming_locked = false;
    let shift_escape = keyboard.just_pressed(KeyCode::Escape)
        && (keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight));
    let open_key = if *state.get() == GameState::InGame {
        false
    } else {
        shift_escape || keyboard.just_pressed(KeyCode::KeyS)
    };
    if menu.page == MenuPage::Closed {
        if open_key {
            menu.page = MenuPage::Game;
            menu.return_page = MenuPage::Closed;
            menu.selected = 0;
            menu.feedback.clear();
        }
        return;
    }
    if matches!(
        menu.page,
        MenuPage::GoLiveConfirmation
            | MenuPage::NewTown
            | MenuPage::LoadTown
            | MenuPage::SecretsDisclaimer
            | MenuPage::Secrets
    ) {
        return;
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        if menu.page == MenuPage::Settings {
            if menu.confirm_settings_close {
                menu.confirm_settings_close = false;
                menu.selected = SETTINGS_BACK_INDEX;
            } else {
                request_settings_close(&mut menu, &player_settings.0, &config.0.twitch.broadcast);
                if menu.confirm_settings_close {
                    menu.selected = 0;
                }
            }
        } else {
            let target = menu.return_page;
            menu.page = target;
            if target == MenuPage::Game {
                menu.return_page = MenuPage::Closed;
            }
            menu.selected = 0;
            menu.feedback.clear();
        }
        return;
    }
    if menu.page == MenuPage::Settings && menu.confirm_settings_close {
        if keyboard.just_pressed(KeyCode::ArrowLeft)
            || keyboard.just_pressed(KeyCode::ArrowRight)
            || keyboard.just_pressed(KeyCode::ArrowUp)
            || keyboard.just_pressed(KeyCode::ArrowDown)
        {
            menu.selected = usize::from(menu.selected == 0);
        }
        if keyboard.just_pressed(KeyCode::Enter) {
            if menu.selected == 0 {
                if apply_settings_draft(
                    &mut menu,
                    &mut player_settings,
                    &mut config,
                    streaming_locked,
                ) {
                    close_settings_menu(&mut menu);
                }
            } else {
                menu.draft = player_settings.0.clone();
                menu.streaming_draft = config.0.twitch.broadcast.clone();
                close_settings_menu(&mut menu);
            }
        }
        return;
    }
    let item_count = if menu.page == MenuPage::Settings {
        SETTINGS_MENU_ITEM_COUNT
    } else if *state.get() == GameState::InGame {
        6
    } else {
        5
    };
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        menu.selected = menu.selected.checked_sub(1).unwrap_or(item_count - 1);
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        menu.selected = (menu.selected + 1) % item_count;
    }
    if menu.page == MenuPage::Settings {
        let previous_tab =
            keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
        if keyboard.just_pressed(KeyCode::Tab) {
            menu.settings_tab = cycle_settings_tab(menu.settings_tab, !previous_tab);
            menu.selected = settings_tab_indices(menu.settings_tab)
                .first()
                .copied()
                .unwrap_or(SETTINGS_APPLY_INDEX);
            menu.feedback.clear();
            return;
        }
        if let Some(tab) = settings_tab_for_index(menu.selected) {
            menu.settings_tab = tab;
        }
        let adjustment = i8::from(keyboard.just_pressed(KeyCode::ArrowRight))
            - i8::from(keyboard.just_pressed(KeyCode::ArrowLeft));
        if adjustment != 0 && menu.selected < SETTINGS_APPLY_INDEX {
            let selected = menu.selected;
            if settings_value_enabled(selected, streaming_locked) {
                let menu = &mut *menu;
                adjust_settings_menu(
                    &mut menu.draft,
                    &mut menu.streaming_draft,
                    selected,
                    adjustment,
                );
                menu.feedback.clear();
            } else {
                "End the stream before changing streaming settings.".clone_into(&mut menu.feedback);
            }
        }
        if !keyboard.just_pressed(KeyCode::Enter) {
            return;
        }
        match menu.selected {
            SETTINGS_APPLY_INDEX => {
                apply_settings_draft(
                    &mut menu,
                    &mut player_settings,
                    &mut config,
                    streaming_locked,
                );
            }
            SETTINGS_DEFAULTS_INDEX => {
                menu.draft = PlayerSettings::default();
                if !streaming_locked {
                    menu.streaming_draft = BroadcastConfig::default();
                }
                "Restored default settings in this draft".clone_into(&mut menu.feedback);
            }
            SETTINGS_BACK_INDEX => {
                request_settings_close(&mut menu, &player_settings.0, &config.0.twitch.broadcast);
                if menu.confirm_settings_close {
                    menu.selected = 0;
                }
            }
            _ => {
                let selected = menu.selected;
                if settings_value_enabled(selected, streaming_locked) {
                    let menu = &mut *menu;
                    adjust_settings_menu(&mut menu.draft, &mut menu.streaming_draft, selected, 1);
                    menu.feedback.clear();
                } else {
                    "End the stream before changing streaming settings."
                        .clone_into(&mut menu.feedback);
                }
            }
        }
        return;
    }
    if !keyboard.just_pressed(KeyCode::Enter) {
        return;
    }
    if *state.get() == GameState::InGame {
        match menu.selected {
            0 => {
                io.save = true;
                "Save requested".clone_into(&mut menu.feedback);
                menu.page = MenuPage::Closed;
            }
            1 => {
                io.save_jump_start = true;
                "Protected jump-start save requested".clone_into(&mut menu.feedback);
                menu.page = MenuPage::Closed;
            }
            2 if save.store.path().is_file() => {
                io.load = true;
                io.load_source = None;
                menu.page = MenuPage::Closed;
            }
            3 => open_settings_menu(
                &mut menu,
                MenuPage::Game,
                &player_settings.0,
                &config.0.twitch.broadcast,
            ),
            4 => {
                #[cfg(target_os = "windows")]
                toggle_direct_broadcast(
                    &config.0,
                    &secrets,
                    &connection,
                    &broadcast_runtime,
                    &mut broadcast_control,
                    &mut menu.feedback,
                );
                #[cfg(not(target_os = "windows"))]
                "Direct Twitch streaming is available only on Windows."
                    .clone_into(&mut menu.feedback);
            }
            5 => {
                menu.page = MenuPage::Closed;
                next_state.set(GameState::MainMenu);
            }
            _ => "No native save exists yet".clone_into(&mut menu.feedback),
        }
    } else {
        match menu.selected {
            0 => {
                if twitch_accounts_connected(&config.0, &secrets, &connection) {
                    open_new_town_dialog(&mut menu);
                } else {
                    open_twitch_setup_required(&mut menu);
                }
            }
            1 if !town_catalog.entries().is_empty() => {
                if twitch_accounts_connected(&config.0, &secrets, &connection) {
                    open_load_town_dialog(&mut menu);
                } else {
                    open_twitch_setup_required(&mut menu);
                }
            }
            2 => open_settings_menu(
                &mut menu,
                MenuPage::Game,
                &player_settings.0,
                &config.0.twitch.broadcast,
            ),
            3 => {
                menu.page = MenuPage::Closed;
                next_state.set(GameState::Credits);
            }
            4 => {
                exit.write(AppExit::Success);
            }
            _ => "No native save exists yet".clone_into(&mut menu.feedback),
        }
    }
}

pub(crate) fn open_settings_menu(
    menu: &mut MenuRuntime,
    return_page: MenuPage,
    settings: &PlayerSettings,
    streaming: &BroadcastConfig,
) {
    menu.page = MenuPage::Settings;
    menu.return_page = return_page;
    menu.selected = 0;
    menu.settings_tab = SettingsTab::Video;
    menu.confirm_settings_close = false;
    menu.draft = settings.clone();
    menu.streaming_draft = streaming.clone();
    menu.feedback.clear();
}

pub(crate) fn adjust_settings_menu(
    settings: &mut PlayerSettings,
    streaming: &mut BroadcastConfig,
    selected: usize,
    direction: i8,
) {
    let increase = direction > 0;
    match selected {
        0 => {
            settings.video.display_mode = match (settings.video.display_mode, increase) {
                (DisplayMode::Windowed, true) | (DisplayMode::Fullscreen, false) => {
                    DisplayMode::Borderless
                }
                (DisplayMode::Borderless, true) | (DisplayMode::Windowed, false) => {
                    DisplayMode::Fullscreen
                }
                (DisplayMode::Borderless, false) | (DisplayMode::Fullscreen, true) => {
                    DisplayMode::Windowed
                }
            };
        }
        1 => {
            const RESOLUTIONS: [(u32, u32); 6] = [
                (1_280, 720),
                (1_600, 900),
                (1_920, 1_080),
                (2_560, 1_440),
                (3_440, 1_440),
                (3_840, 2_160),
            ];
            let resolution = cycle_choice(
                &RESOLUTIONS,
                (settings.video.width, settings.video.height),
                increase,
            );
            (settings.video.width, settings.video.height) = resolution;
        }
        2 => settings.video.vsync = !settings.video.vsync,
        3 => {
            const LIMITS: [Option<u16>; 6] =
                [Some(24), Some(30), Some(60), Some(120), Some(240), None];
            settings.video.fps_limit = cycle_choice(&LIMITS, settings.video.fps_limit, increase);
        }
        4 => settings.video.shadows_enabled = !settings.video.shadows_enabled,
        5 => {
            const RESOLUTIONS: [u16; 5] = [256, 512, 1_024, 2_048, 4_096];
            settings.video.shadow_map_resolution =
                cycle_choice(&RESOLUTIONS, settings.video.shadow_map_resolution, increase);
        }
        6 => settings.video.ambient_occlusion = !settings.video.ambient_occlusion,
        7 => {
            const SAMPLES: [u8; 4] = [1, 2, 4, 8];
            settings.video.msaa_samples =
                cycle_choice(&SAMPLES, settings.video.msaa_samples, increase);
        }
        8 => {
            const MODES: [PostProcessAntiAliasing; 3] = [
                PostProcessAntiAliasing::None,
                PostProcessAntiAliasing::Fxaa,
                PostProcessAntiAliasing::Smaa,
            ];
            settings.video.post_process_aa =
                cycle_choice(&MODES, settings.video.post_process_aa, increase);
        }
        9 => {
            settings.video.brightness_ev =
                step_f32(settings.video.brightness_ev, direction, -5.0, 5.0, 0.5);
        }
        10 => settings.video.gamma = step_f32(settings.video.gamma, direction, -5.0, 5.0, 0.5),
        11 => settings.audio.master = step_f32(settings.audio.master, direction, 0.0, 1.0, 0.05),
        12 => settings.audio.music = step_f32(settings.audio.music, direction, 0.0, 1.0, 0.05),
        13 => {
            settings.audio.sound_effects =
                step_f32(settings.audio.sound_effects, direction, 0.0, 1.0, 0.05);
        }
        14 => {
            settings.audio.ambience = step_f32(settings.audio.ambience, direction, 0.0, 1.0, 0.05);
        }
        15 => {
            settings.camera.zoom_sensitivity =
                step_f32(settings.camera.zoom_sensitivity, direction, 0.0, 100.0, 1.0);
        }
        16 => {
            settings.camera.field_of_view_degrees =
                step_u16(settings.camera.field_of_view_degrees, direction, 30, 120, 5);
        }
        17 => {
            const MODES: [NameDisplayMode; 3] = [
                NameDisplayMode::None,
                NameDisplayMode::StaffAndSubscribers,
                NameDisplayMode::AllPlayers,
            ];
            settings.interface.display_names =
                cycle_choice(&MODES, settings.interface.display_names, increase);
        }
        18 => {
            const MODES: [BuildingHealthDisplayMode; 3] = [
                BuildingHealthDisplayMode::None,
                BuildingHealthDisplayMode::DamagedOnly,
                BuildingHealthDisplayMode::Always,
            ];
            settings.interface.display_building_health =
                cycle_choice(&MODES, settings.interface.display_building_health, increase);
        }
        19 => {
            const MINUTES: [u16; 6] = [0, 1, 5, 10, 30, 60];
            settings.autosave_minutes = cycle_choice(&MINUTES, settings.autosave_minutes, increase);
        }
        20 => {
            const INTERVALS: [TimelapseInterval; 7] = [
                TimelapseInterval::Off,
                TimelapseInterval::TenMinutes,
                TimelapseInterval::OneHour,
                TimelapseInterval::ThreeHours,
                TimelapseInterval::SixHours,
                TimelapseInterval::TwelveHours,
                TimelapseInterval::TwentyFourHours,
            ];
            settings.timelapse.interval =
                cycle_choice(&INTERVALS, settings.timelapse.interval, increase);
        }
        21 => settings.timelapse.dynamic = !settings.timelapse.dynamic,
        22 => {
            const SCALES: [u16; 6] = [75, 90, 100, 110, 125, 150];
            settings.interface.ui_scale_percent =
                cycle_choice(&SCALES, settings.interface.ui_scale_percent, increase);
        }
        23 => settings.interface.high_contrast = !settings.interface.high_contrast,
        24 => settings.interface.reduced_motion = !settings.interface.reduced_motion,
        25 => streaming.enabled = !streaming.enabled,
        26 => {
            const RESOLUTIONS: [(u16, u16); 4] =
                [(640, 360), (854, 480), (1_280, 720), (1_920, 1_080)];
            (streaming.width, streaming.height) =
                cycle_choice(&RESOLUTIONS, (streaming.width, streaming.height), increase);
        }
        27 => {
            const FRAME_RATES: [u8; 2] = [30, 60];
            streaming.frames_per_second =
                cycle_choice(&FRAME_RATES, streaming.frames_per_second, increase);
        }
        28 => {
            const VIDEO_BITRATES: [u32; 7] = [500, 1_000, 2_000, 3_000, 4_500, 5_000, 6_000];
            streaming.video_bitrate_kbps =
                cycle_choice(&VIDEO_BITRATES, streaming.video_bitrate_kbps, increase);
        }
        29 => {
            const AUDIO_BITRATES: [u16; 4] = [64, 96, 128, 160];
            streaming.audio_bitrate_kbps =
                cycle_choice(&AUDIO_BITRATES, streaming.audio_bitrate_kbps, increase);
        }
        30 => {
            const ENCODERS: [BroadcastEncoderPreference; 7] = [
                BroadcastEncoderPreference::Auto,
                BroadcastEncoderPreference::Nvidia,
                BroadcastEncoderPreference::Intel,
                BroadcastEncoderPreference::Amd,
                BroadcastEncoderPreference::MediaFoundation,
                BroadcastEncoderPreference::X264,
                BroadcastEncoderPreference::OpenH264,
            ];
            streaming.encoder = cycle_choice(&ENCODERS, streaming.encoder, increase);
        }
        31 => streaming.bandwidth_test = !streaming.bandwidth_test,
        32 => {
            const MODES: [BroadcastRenderMode; 2] =
                [BroadcastRenderMode::StreamOnly, BroadcastRenderMode::Headed];
            streaming.render_mode = cycle_choice(&MODES, streaming.render_mode, increase);
        }
        _ => {}
    }
}

pub(crate) fn cycle_choice<T: Copy + PartialEq>(choices: &[T], current: T, increase: bool) -> T {
    let current = choices
        .iter()
        .position(|choice| *choice == current)
        .unwrap_or_default();
    let next = if increase {
        (current + 1) % choices.len()
    } else {
        current.checked_sub(1).unwrap_or(choices.len() - 1)
    };
    choices[next]
}

pub(crate) fn step_f32(value: f32, direction: i8, minimum: f32, maximum: f32, step: f32) -> f32 {
    (value + f32::from(direction) * step).clamp(minimum, maximum)
}

pub(crate) fn step_u16(value: u16, direction: i8, minimum: u16, maximum: u16, step: u16) -> u16 {
    let delta = i32::from(direction) * i32::from(step);
    u16::try_from((i32::from(value) + delta).clamp(i32::from(minimum), i32::from(maximum)))
        .expect("clamped camera setting fits u16")
}

pub(crate) fn debug_start_day() -> Option<u32> {
    std::env::var_os("STREAM_TOWN_DEBUG_DAY")
        .and_then(|value| value.to_str().and_then(|value| value.parse().ok()))
}

pub(crate) fn debug_weather_override() -> Option<Weather> {
    std::env::var_os("STREAM_TOWN_DEBUG_WEATHER")
        .and_then(|value| value.to_str().and_then(parse_weather))
}

pub(crate) fn debug_building_health(max_health: i32) -> Option<i32> {
    std::env::var_os("STREAM_TOWN_DEBUG_BUILDING_HEALTH")
        .and_then(|value| value.to_str().and_then(|value| value.parse().ok()))
        .map(|health: i32| health.clamp(0, max_health))
}

pub(crate) fn runtime_initial_agents(configured: u16) -> u16 {
    let configured = if std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some() {
        configured.max(300)
    } else {
        configured
    };
    std::env::var_os("STREAM_TOWN_DEBUG_INITIAL_AGENTS")
        .and_then(|value| value.to_str().and_then(|value| value.parse().ok()))
        .map_or(configured, |agents: u16| agents.clamp(1, 5_000))
}

pub(crate) fn debug_smoke_pet() -> Option<StableId> {
    let value = std::env::var("STREAM_TOWN_SMOKE_PET").ok()?;
    let suffix = match value.trim().to_ascii_lowercase().as_str() {
        "red_panda" | "red-panda" | "panda" => "red_panda",
        "giraffe" => "giraffe",
        "duck" => "duck",
        "butterfly" => "butterfly",
        "fish_god" | "fish-god" | "fishgod" => "fish_god",
        _ => return None,
    };
    StableId::new(format!("pet:{suffix}")).ok()
}

pub(crate) fn actor_scene_budget() -> usize {
    actor_detail_budget(
        std::env::var("STREAM_TOWN_ACTOR_SCENE_BUDGET")
            .ok()
            .as_deref(),
        std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some(),
    )
}

pub(crate) fn actor_detail_budget(value: Option<&str>, benchmarking: bool) -> usize {
    value
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(if benchmarking {
            PERFORMANCE_ACTOR_DETAIL_BUDGET
        } else {
            DEFAULT_ACTOR_DETAIL_BUDGET
        })
}

pub(crate) fn animation_detail_budget() -> usize {
    actor_detail_budget(
        std::env::var("STREAM_TOWN_ANIMATION_BUDGET")
            .ok()
            .as_deref(),
        std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some(),
    )
}

pub(crate) fn parse_weather(value: &str) -> Option<Weather> {
    match value.to_ascii_lowercase().as_str() {
        "clear" => Some(Weather::Clear),
        "rain" => Some(Weather::Rain),
        "fog" => Some(Weather::Fog),
        "snow" => Some(Weather::Snow),
        _ => None,
    }
}

pub(crate) fn initial_actor_identity(index: u16) -> (String, Option<&'static str>) {
    match index {
        0 => ("npc:starting_defender".to_owned(), Some("role:defender")),
        1 => ("npc:starting_logger".to_owned(), Some("role:logger")),
        2 => ("npc:starting_miner".to_owned(), Some("role:miner")),
        3 => ("npc:starting_gatherer".to_owned(), Some("role:gatherer")),
        4 => ("npc:starting_builder".to_owned(), Some("role:builder")),
        _ => (format!("actor:viewer_{index:04}"), None),
    }
}
