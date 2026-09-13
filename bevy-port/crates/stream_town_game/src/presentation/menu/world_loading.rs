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
