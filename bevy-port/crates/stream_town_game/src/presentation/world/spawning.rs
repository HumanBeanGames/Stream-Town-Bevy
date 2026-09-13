pub(crate) fn generate_and_spawn_world(
    mut commands: Commands,
    mut loading: ResMut<WorldLoadingRuntime>,
    mut restart: ResMut<TownRestartRuntime>,
    mut io: ResMut<MenuIoRequest>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    mut selected: ResMut<SelectedCell>,
    mut placers: ResMut<BuildingPlacers>,
    mut agent_commands: ResMut<AgentCommandQueue>,
    mut render_stats: ResMut<WorldRenderStats>,
    mut cameras: Query<(&mut Transform, &mut TownCameraControllerRuntime), With<TownCamera>>,
) {
    if loading.phase != WorldLoadingPhase::Spawning {
        return;
    }
    if loading.spawn_runtime.is_none() {
        let construction_starting_render_frame = loading
            .presented_frames
            .as_ref()
            .map_or(0, PresentedRenderFrames::current);
        selected.0 = None;
        commands.insert_resource(SelectedActor::default());
        commands.insert_resource(RegenerationRoleRuntime::default());
        commands.insert_resource(TraversalWearRuntime::default());
        commands.insert_resource(PathSurfaceRuntime::default());
        *render_stats = WorldRenderStats::default();
        let Some(prepared) = loading.prepared_world.as_mut() else {
            loading.phase = WorldLoadingPhase::Loading;
            "Waiting for deterministic terrain".clone_into(&mut loading.status);
            "The generation task has not completed yet".clone_into(&mut loading.substatus);
            return;
        };
        let generated = &mut prepared.world;
        let centre = GridPos {
            x: config.0.world.width / 2,
            z: config.0.world.height / 2,
        };
        let town_hall_position = GridPos {
            x: (centre.x + 4).min(config.0.world.width - 2),
            z: centre.z,
        };
        let town_hall_id = StableId::new("building:townhall").expect("static ID");
        let town_hall_definition = &content.0.buildings[&town_hall_id];
        let mut initial_simulation = WorldSimulation::new(generated.seed);
        ensure_town_hall_state(&content.0, &config.0, &mut initial_simulation);
        let cleared_trees = clear_seeded_trees_under_building(
            &content.0,
            &initial_simulation,
            generated,
            &town_hall_id,
        );
        if cleared_trees > 0 {
            info!(
                cleared_trees,
                "cleared seeded trees beneath the initial Town Hall"
            );
        }
        let town_hall_placement =
            town_hall_placement_position(&config.0, town_hall_definition.footprint);
        let town_hall_focus = grid_to_world_on_surface(town_hall_position, &config.0, generated);
        if let Ok((mut camera, mut controller)) = cameras.single_mut() {
            let transform = if std::env::var_os("STREAM_TOWN_SMOKE_FISH_SCHOOL").is_some() {
                let water_height = f32::from(config.0.world.water_level_centimetres) * 0.01;
                let focus = Vec3::new(145.0, water_height - 2.0, 0.0);
                Transform::from_translation(focus + Vec3::new(0.0, 45.0, 0.01))
                    .looking_at(focus, Vec3::Z)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_ROLE_AUDIO").is_some() {
                let focus = initial_actor_position(generated, town_hall_position, 1)
                    .map_or(Vec3::ZERO, |position| {
                        grid_to_world_on_surface(position, &config.0, generated)
                    });
                Transform::from_translation(focus + Vec3::new(7.0, 7.0, 7.0))
                    .looking_at(focus + Vec3::Y * 2.0, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_RESOURCE_CLOSEUP").is_some() {
                let requested_kind = std::env::var("STREAM_TOWN_SMOKE_RESOURCE_KIND")
                    .unwrap_or_else(|_| "resource:wood".to_owned());
                let focus = generated
                    .resources
                    .iter()
                    .filter(|resource| resource.kind.as_str() == requested_kind)
                    .min_by_key(|resource| {
                        let dx = i32::from(resource.position.x) - i32::from(centre.x);
                        let dz = i32::from(resource.position.z) - i32::from(centre.z);
                        dx * dx + dz * dz
                    })
                    .map_or(Vec3::ZERO, |resource| {
                        generated_resource_world_position(resource, &config.0, generated)
                    });
                Transform::from_xyz(focus.x + 5.0, focus.y + 6.0, focus.z + 5.0)
                    .looking_at(focus + Vec3::Y * 1.5, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_HEALING_VFX").is_some() {
                let focus = grid_to_world_on_surface(centre, &config.0, generated);
                Transform::from_xyz(focus.x + 28.0, focus.y + 32.0, focus.z + 28.0)
                    .looking_at(focus + Vec3::Y * 5.0, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_COMBAT_VFX").is_some() {
                let focus = grid_to_world_on_surface(centre, &config.0, generated);
                Transform::from_xyz(focus.x + 34.0, focus.y + 38.0, focus.z + 34.0)
                    .looking_at(focus + Vec3::Y * 4.0, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_BUILDING_VFX").is_some()
                || std::env::var_os("STREAM_TOWN_SMOKE_CHIMNEY").is_some()
            {
                let focus = grid_to_world_on_surface(centre, &config.0, generated);
                Transform::from_xyz(focus.x + 40.0, focus.y + 42.0, focus.z + 40.0)
                    .looking_at(focus + Vec3::Y * 6.0, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_PING").is_some() {
                let focus = initial_actor_position(generated, town_hall_position, 0)
                    .map_or(Vec3::ZERO, |position| {
                        grid_to_world_on_surface(position, &config.0, generated)
                    });
                Transform::from_xyz(focus.x + 14.0, focus.y + 13.0, focus.z + 14.0)
                    .looking_at(focus + Vec3::Y * 3.0, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_GATE").is_some() {
                let focus = grid_to_world_on_surface(centre, &config.0, generated);
                Transform::from_xyz(focus.x + 12.0, focus.y + 10.0, focus.z + 12.0)
                    .looking_at(focus + Vec3::Y * 1.5, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_FOLIAGE").is_some() {
                let focus = grid_to_world_on_surface(centre, &config.0, generated);
                Transform::from_xyz(focus.x + 30.0, focus.y + 35.0, focus.z + 30.0)
                    .looking_at(focus + Vec3::Y, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_SHORELINE").is_some() {
                shoreline_camera_transform(generated, &config.0)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP").is_some() {
                let focus = initial_actor_position(generated, town_hall_position, 0)
                    .map_or(Vec3::ZERO, |position| {
                        grid_to_world_on_surface(position, &config.0, generated)
                    });
                Transform::from_xyz(focus.x + 7.0, focus.y + 6.0, focus.z + 7.0)
                    .looking_at(focus + Vec3::Y * 1.6, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_OVERLAYS").is_some() {
                let focus = grid_to_world_on_surface(town_hall_position, &config.0, generated);
                Transform::from_xyz(focus.x + 74.0, focus.y + 88.0, focus.z + 74.0)
                    .looking_at(focus + Vec3::Y * 5.0, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_SEAGULL").is_some() {
                let (start, end) = deterministic_seagull_leg(generated.seed, 0);
                let focus = start.lerp(end, 0.1);
                Transform::from_translation(focus + Vec3::new(42.0, 32.0, 42.0))
                    .looking_at(focus, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_FLAG").is_some() {
                let focus = grid_to_world_on_surface(centre, &config.0, generated);
                Transform::from_translation(focus + Vec3::new(30.0, 34.0, 30.0))
                    .looking_at(focus + Vec3::Y * 7.0, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_GODRAY").is_some() {
                let focus = grid_to_world_on_surface(centre, &config.0, generated);
                Transform::from_translation(focus + Vec3::new(52.0, 66.0, 52.0))
                    .looking_at(focus + Vec3::Y * 22.0, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_PET").is_some() {
                let focus = initial_actor_position(generated, town_hall_position, 1)
                    .map_or(Vec3::ZERO, |position| {
                        grid_to_world_on_surface(position, &config.0, generated)
                    });
                Transform::from_translation(focus + Vec3::new(22.0, 20.0, 22.0))
                    .looking_at(focus + Vec3::Y * 3.0, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_GIRAFFE").is_some() {
                let focus = grid_to_world_on_surface(centre, &config.0, generated);
                Transform::from_translation(focus + Vec3::new(24.0, 24.0, 24.0))
                    .looking_at(focus + Vec3::Y * 6.0, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_PLACEMENT").is_some() {
                let focus = grid_to_world_on_surface(town_hall_position, &config.0, generated);
                Transform::from_translation(focus + Vec3::new(32.0, 38.0, 32.0))
                    .looking_at(focus, Vec3::Y)
            } else if std::env::var_os("STREAM_TOWN_SMOKE_CLOSEUP").is_some() {
                let focus = grid_to_world_on_surface(town_hall_position, &config.0, generated);
                Transform::from_xyz(focus.x + 66.0, 78.0, focus.z + 66.0).looking_at(focus, Vec3::Y)
            } else {
                unity_town_camera_transform(town_hall_focus)
            };
            *camera = transform;
            controller.set_home(transform);
        }
        let town_hall_region =
            building_navigation_region(town_hall_placement, town_hall_definition, 0, generated)
                .expect("the configured Town Hall footprint fits the generated world");
        generated
            .navigation
            .set_blocked(town_hall_region, true)
            .expect("the configured Town Hall footprint updates navigation");

        if std::env::var_os("STREAM_TOWN_SMOKE_PLACEMENT").is_some() {
            let building = StableId::new("building:house").expect("static building ID");
            if let Some(definition) = content.0.buildings.get(&building)
                && let Some(valid_position) = find_building_site(
                    generated,
                    GridPos {
                        x: town_hall_placement
                            .x
                            .saturating_add(town_hall_definition.footprint[0] + 2)
                            .min(generated.navigation.width() - 1),
                        z: town_hall_placement.z.saturating_sub(6),
                    },
                    definition.footprint,
                )
            {
                placers.0.insert(
                    StableId::new("actor:smoke_placement_valid").expect("static actor ID"),
                    BuildingPlacement {
                        building: building.clone(),
                        thick_path: false,
                        position: valid_position,
                        navigation_position: None,
                        rotation_quarter_turns: 0,
                        line_start: None,
                        line_end: None,
                        path_cells: Vec::new(),
                        inactivity_seconds: 0.0,
                    },
                );
                placers.0.insert(
                    StableId::new("actor:smoke_placement_blocked").expect("static actor ID"),
                    BuildingPlacement {
                        building,
                        thick_path: false,
                        position: town_hall_placement,
                        navigation_position: None,
                        rotation_quarter_turns: 1,
                        line_start: None,
                        line_end: None,
                        path_cells: Vec::new(),
                        inactivity_seconds: 0.0,
                    },
                );
            }
        }

        let world_size = Vec2::new(
            f32::from(config.0.world.width) * config.0.world.cell_size,
            f32::from(config.0.world.height) * config.0.world.cell_size,
        );
        if let Some(meshes) = meshes.as_deref_mut() {
            // The worker task normally supplies all three CPU-heavy products. The
            // fallback keeps reduced test apps valid when they install Assets<Mesh>
            // without an AssetServer/compute pool.
            let terrain_mesh = prepared
                .terrain_mesh
                .take()
                .unwrap_or_else(|| generated_terrain_mesh(generated, &config.0));
            let terrain_collider = prepared.terrain_collider.take().unwrap_or_else(|| {
                let collision_mesh = generated_terrain_chunk_mesh(
                    generated,
                    &config.0,
                    0,
                    0,
                    generated.navigation.width(),
                    generated.navigation.height(),
                    1,
                    false,
                );
                Collider::trimesh_from_mesh(&collision_mesh)
                    .expect("generated terrain has indexed triangle geometry")
            });
            let terrain = meshes.add(terrain_mesh);
            commands.spawn((
                WorldEntity,
                TerrainSurface,
                Name::new("Terrain"),
                Mesh3d(terrain),
                MeshMaterial3d(render.ground.clone()),
                terrain_collider,
                RigidBody::Static,
            ));
            render_stats.terrain_high_chunks = 1;
        } else {
            commands.spawn((
                WorldEntity,
                TerrainSurface,
                Mesh3d(render.cube.clone()),
                MeshMaterial3d(render.ground.clone()),
                Transform::from_xyz(0.0, -0.15, 0.0).with_scale(Vec3::new(
                    world_size.x,
                    0.3,
                    world_size.y,
                )),
            ));
        }
        if let Some(meshes) = meshes.as_deref_mut() {
            let water_mesh = prepared
                .water_mesh
                .take()
                .unwrap_or_else(|| generated_water_mesh(generated, &config.0));
            commands.spawn((
                WorldEntity,
                WaterSurface,
                Mesh3d(meshes.add(water_mesh)),
                MeshMaterial3d(render.water.clone()),
            ));
        } else {
            let water_height = f32::from(config.0.world.water_level_centimetres) * 0.01;
            commands.spawn((
                WorldEntity,
                WaterSurface,
                Mesh3d(render.cube.clone()),
                MeshMaterial3d(render.water.clone()),
                Transform::from_xyz(0.0, water_height - 0.08, 0.0).with_scale(Vec3::new(
                    world_size.x,
                    0.12,
                    world_size.y,
                )),
            ));
        }
        spawn_fish_school_scene(
            &mut commands,
            &presentation.0,
            &render,
            WORLD_SCENE_PATH,
            generated.seed,
            f32::from(config.0.world.water_level_centimetres) * 0.01,
            false,
            None,
            Some((generated, &config.0)),
        );

        let isolate_animation = std::env::var_os("STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP").is_some();
        let foliage_rendering_enabled = std::env::var("STREAM_TOWN_BENCHMARK_FOLIAGE")
            .map_or(true, |value| value.trim() != "0");
        let resource_total = if isolate_animation {
            0
        } else {
            generated.resources.len()
        };
        let foliage_total =
            if isolate_animation || !foliage_rendering_enabled || asset_server.is_none() {
                0
            } else {
                generated.foliage.len()
            };
        loading.spawn_runtime = Some(WorldSpawnRuntime {
            phase: WorldSpawnPhase::Resources,
            resource_cursor: 0,
            foliage_cursor: 0,
            resource_total,
            foliage_total,
            foliage_gpu_batches: BTreeSet::new(),
            foliage_spatial_groups: BTreeSet::new(),
            update_count: 0,
            starting_render_frame: construction_starting_render_frame,
        });
        loading.work.terrain_entities = LoadingWork::count(2, 2);
        loading.work.resource_entities = LoadingWork::count(0, resource_total);
        loading.work.foliage_entities = LoadingWork::count(0, foliage_total);
        loading.progress = loading.work.progress();
        "Populating town resources".clone_into(&mut loading.status);
        loading.substatus = format!("0 / {resource_total} resource entities");
        return;
    }

    let mut spawn_runtime = loading
        .spawn_runtime
        .take()
        .expect("world spawn runtime checked above");
    spawn_runtime.update_count = spawn_runtime.update_count.saturating_add(1);
    let Some(prepared) = loading.prepared_world.as_ref() else {
        loading.spawn_runtime = Some(spawn_runtime);
        loading.phase = WorldLoadingPhase::Loading;
        "Waiting for deterministic terrain".clone_into(&mut loading.status);
        return;
    };
    let generated = &prepared.world;
    let isolate_animation = std::env::var_os("STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP").is_some();
    if spawn_runtime.phase == WorldSpawnPhase::Resources {
        let start = spawn_runtime.resource_cursor;
        let frame_started = Instant::now();
        let mut end = start;
        while end < spawn_runtime.resource_total
            && end.saturating_sub(start) < WORLD_RESOURCE_SPAWN_BUDGET_PER_FRAME
            && (end == start || frame_started.elapsed() < LOADING_SCENE_TIME_BUDGET_PER_FRAME)
        {
            if !isolate_animation {
                let resource = &generated.resources[end];
                if resource.amount > 0 {
                    let position =
                        generated_resource_world_position(resource, &config.0, generated);
                    spawn_resource_visual(
                        &mut commands,
                        &content.0,
                        &presentation.0,
                        &render,
                        asset_server.as_deref(),
                        &asset_root.0,
                        resource,
                        position,
                        &config.0,
                        generated,
                    );
                }
            }
            end += 1;
        }
        spawn_runtime.resource_cursor = end;
        loading.work.resource_entities =
            LoadingWork::count(spawn_runtime.resource_cursor, spawn_runtime.resource_total);
        loading.progress = loading.work.progress();
        "Populating town resources".clone_into(&mut loading.status);
        loading.substatus = format!(
            "{} / {} resource entities",
            spawn_runtime.resource_cursor, spawn_runtime.resource_total
        );
        if spawn_runtime.resource_cursor >= spawn_runtime.resource_total {
            spawn_runtime.phase = WorldSpawnPhase::Foliage;
        }
        loading.spawn_runtime = Some(spawn_runtime);
        return;
    }
    if spawn_runtime.phase == WorldSpawnPhase::Foliage {
        let start = spawn_runtime.foliage_cursor;
        let frame_started = Instant::now();
        let mut end = start;
        while end < spawn_runtime.foliage_total
            && end.saturating_sub(start) < WORLD_FOLIAGE_SPAWN_BUDGET_PER_FRAME
            && (end == start || frame_started.elapsed() < LOADING_SCENE_TIME_BUDGET_PER_FRAME)
        {
            if let Some(asset_server) = asset_server.as_deref() {
                let foliage = &generated.foliage[end];
                let Some(visual) = resolve_foliage_visual(
                    &content.0,
                    &presentation.0,
                    &render,
                    asset_server,
                    &asset_root.0,
                    generated,
                    &config.0,
                    foliage,
                ) else {
                    end += 1;
                    continue;
                };
                let key = foliage_batch_key(foliage);
                spawn_runtime
                    .foliage_gpu_batches
                    .insert((foliage.layer.clone(), foliage.variant));
                spawn_runtime.foliage_spatial_groups.insert(key.clone());
                spawn_foliage_visual(&mut commands, key, visual);
                render_stats.foliage_instances += 1;
                render_stats.foliage_visible_instances += 1;
            }
            end += 1;
        }
        spawn_runtime.foliage_cursor = end;
        loading.work.foliage_entities =
            LoadingWork::count(spawn_runtime.foliage_cursor, spawn_runtime.foliage_total);
        loading.progress = loading.work.progress();
        "Planting town foliage".clone_into(&mut loading.status);
        loading.substatus = format!(
            "{} / {} foliage entities",
            spawn_runtime.foliage_cursor, spawn_runtime.foliage_total
        );
        if spawn_runtime.foliage_cursor < spawn_runtime.foliage_total {
            loading.spawn_runtime = Some(spawn_runtime);
            return;
        }
        // Matching Mesh3d/material handles are automatically instanced by
        // Bevy's opaque and shadow render phases. These deterministic spatial
        // groups are retained for auditing and clearance/streaming telemetry;
        // they do not duplicate source geometry.
        render_stats.foliage_batches = spawn_runtime.foliage_gpu_batches.len();
        render_stats.foliage_spatial_groups = spawn_runtime.foliage_spatial_groups.len();
        spawn_runtime.phase = WorldSpawnPhase::Gameplay;
        loading.spawn_runtime = Some(spawn_runtime);
        return;
    }

    let Some(prepared) = loading.prepared_world.take() else {
        loading.phase = WorldLoadingPhase::Loading;
        return;
    };
    info!(
        resources = spawn_runtime.resource_total,
        foliage = spawn_runtime.foliage_total,
        construction_updates = spawn_runtime.update_count,
        presented_frames = loading.presented_frames.as_ref().map_or(0, |frames| {
            frames
                .current()
                .saturating_sub(spawn_runtime.starting_render_frame)
        }),
        "completed frame-budgeted world scene construction"
    );
    let mut generated = prepared.world;
    let centre = GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    };
    let town_hall_position = GridPos {
        x: (centre.x + 4).min(config.0.world.width - 2),
        z: centre.z,
    };
    let town_hall_id = StableId::new("building:townhall").expect("static ID");
    let town_hall_definition = &content.0.buildings[&town_hall_id];
    let town_hall_placement =
        town_hall_placement_position(&config.0, town_hall_definition.footprint);
    let town_hall_focus = grid_to_world_on_surface(town_hall_position, &config.0, &generated);
    let hall = town_hall_focus;
    let mut hall_entity = commands.spawn((
        WorldEntity,
        TownHall,
        RuntimeBuilding {
            id: town_hall_id.clone(),
        },
        GridLocation(town_hall_placement),
        Transform::from_translation(hall),
    ));
    let town_hall = content
        .0
        .buildings
        .get(&town_hall_id)
        .and_then(|building| content.0.archetypes.get(&building.archetype));
    if let Some(scene) = town_hall.and_then(default_archetype_scene).filter(|scene| {
        !isolate_animation
            && asset_server.is_some()
            && converted_asset_exists(&asset_root.0, &scene.asset_path)
    }) {
        hall_entity.insert((
            WorldAssetRoot(
                asset_server
                    .as_deref()
                    .expect("asset server checked above")
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            BuildingPresentation {
                base_translation: hall,
                base_scale: Vec3::splat(config.0.world.cell_size / 2.0),
                base_height_offset: 0.0,
                applied_stage: u8::MAX,
                applied_level: u16::MAX,
                applied_age: 1,
                applied_scene: Some(scene.asset_path.clone()),
            },
            BuildingDamageEmitter::default(),
            Transform::from_translation(hall)
                .with_scale(Vec3::splat(config.0.world.cell_size / 2.0)),
        ));
        if let Some(material) = town_hall
            .and_then(|archetype| prefab_material_spec(archetype, scene, &presentation.0, &render))
        {
            hall_entity.insert(material);
        }
    } else if !isolate_animation {
        let footprint = town_hall.map_or([2, 2], |archetype| archetype.footprint);
        let size = Vec3::new(
            f32::from(footprint[0]) * config.0.world.cell_size,
            config.0.world.cell_size * 1.5,
            f32::from(footprint[1]) * config.0.world.cell_size,
        );
        hall_entity.insert((
            BuildingPresentation {
                base_translation: hall + Vec3::Y * size.y * 0.5,
                base_scale: size,
                base_height_offset: size.y * 0.5,
                applied_stage: u8::MAX,
                applied_level: u16::MAX,
                applied_age: 1,
                applied_scene: None,
            },
            BuildingDamageEmitter::default(),
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.building.clone()),
            Transform::from_xyz(hall.x, hall.y + size.y * 0.5, hall.z).with_scale(size),
        ));
    }

    let mut spawned = 0_u16;
    let mut simulation = WorldSimulation::new(generated.seed);
    ensure_town_hall_state(&content.0, &config.0, &mut simulation);
    let seeded_enemy_camps =
        seed_generated_enemy_camps(&config.0, &content.0, &mut generated, &mut simulation);
    for camp in simulation.enemy_camps.values() {
        let archetype = &content.0.archetypes[&camp.archetype];
        spawn_enemy_camp(
            &mut commands,
            &config.0,
            &generated,
            &presentation.0,
            asset_server.as_deref(),
            &asset_root.0,
            &render,
            &camp.id,
            archetype,
            camp.position,
        );
    }
    info!(
        camps = seeded_enemy_camps,
        "seeded deterministic Unity-authored enemy camps"
    );
    if let Some(health) = debug_building_health(building_max_health(
        &content.0,
        &simulation.buildings[&town_hall_id],
    )) && let Some(town_hall) = simulation.buildings.get_mut(&town_hall_id)
    {
        town_hall.health = health;
    }
    if std::env::var_os("STREAM_TOWN_SMOKE_BUILDING_VFX").is_some()
        && let Some(town_hall) = simulation.buildings.get_mut(&town_hall_id)
    {
        town_hall.health = building_max_health(&content.0, town_hall) / 3;
    }
    simulation.town_resources = config.0.gameplay.starting_town_resources.clone();
    simulation.unlocked_technology.extend(
        content
            .0
            .technology
            .nodes
            .iter()
            .filter(|(_, technology)| technology.initially_unlocked)
            .map(|(id, _)| id.clone()),
    );
    if let Ok(smoke_vote) = std::env::var("STREAM_TOWN_SMOKE_VOTE") {
        match smoke_vote.to_ascii_lowercase().as_str() {
            "technology" | "tech" => {
                let options = technology_ballot_options(&content.0, &simulation);
                if !options.is_empty() {
                    let _ = simulation
                        .start_technology_ballot(options, TECHNOLOGY_VOTE_DURATION_SECONDS);
                }
            }
            "ruler" | "election" => {
                let _ = simulation.start_ruler_vote(RulerVoteKind::NewRuler);
            }
            "retention" | "keep" => {
                simulation.current_ruler =
                    Some(StableId::new("npc:starting_defender").expect("static smoke ruler ID"));
                let _ = simulation.start_ruler_vote(RulerVoteKind::KeepRuler);
            }
            _ => {}
        }
    }
    if std::env::var_os("STREAM_TOWN_SMOKE_GOAL").is_some()
        && let Some(technology) = eligible_technology_ids(&content.0, &simulation)
            .into_iter()
            .find(|technology| !content.0.technology.nodes[technology].objectives.is_empty())
    {
        let node = &content.0.technology.nodes[&technology];
        let _ = simulation.start_technology_goal(
            technology,
            &node.objectives,
            &content.0.objectives,
            MAX_TOWN_GOALS,
        );
    }
    if let Ok(smoke_event) = std::env::var("STREAM_TOWN_SMOKE_EVENT") {
        match smoke_event.to_ascii_lowercase().as_str() {
            "fish" | "fishgod" => {
                let _ = simulation.start_fish_god(true);
                if let Some(event) = &mut simulation.fish_god {
                    event.praises_given = 7;
                }
            }
            "raid" => {
                if let (Some(enemy), Some(boss)) = (
                    archetype_id_by_source(
                        &content.0,
                        ArchetypeKind::Enemy,
                        "Enemy_Minotaur.prefab",
                    ),
                    archetype_id_by_source(
                        &content.0,
                        ArchetypeKind::Enemy,
                        "Enemy_MinotaurBoss.prefab",
                    ),
                ) {
                    let _ = simulation.start_raid(5, 50, enemy, boss);
                }
            }
            _ => {}
        }
    }
    if let Some(day) = debug_start_day() {
        simulation.elapsed_seconds = f64::from(day) * f64::from(config.0.time.seconds_per_day);
        simulation.tick(0.0, config.0.time.seconds_per_day);
    }
    if std::env::var_os("STREAM_TOWN_SMOKE_ENCOUNTER").is_some() {
        simulation.elapsed_seconds = f64::from(config.0.time.seconds_per_day)
            * f64::from(config.0.time.daylight_per_thousand)
            / 1_000.0
            + 1.0;
        simulation.tick(0.0, config.0.time.seconds_per_day);
    }
    if let Some(weather) = debug_weather_override() {
        simulation.weather = weather;
    }
    if std::env::var_os("STREAM_TOWN_DEBUG_AGE_TWO").is_some() {
        let mut buildings = vec![town_hall_id.clone()];
        if std::env::var_os("STREAM_TOWN_SMOKE_GATE").is_some() {
            buildings.push(StableId::new("building:gate").expect("static gate ID"));
        }
        for building in buildings {
            if let Some((technology, _)) = content
                .0
                .technology
                .nodes
                .iter()
                .find(|(_, technology)| technology.aged_buildings.contains(&building))
            {
                simulation.unlocked_technology.insert(technology.clone());
            }
        }
    }
    let retained_players = restart.retained_players.clone();
    let initial_agents = retained_players.as_ref().map_or_else(
        || runtime_initial_agents(config.0.gameplay.initial_agents),
        |players| u16::try_from(players.len()).unwrap_or(u16::MAX),
    );
    let smoke_pet = debug_smoke_pet();
    let spawn_positions =
        connected_actor_positions(&generated, centre, town_hall_position, initial_agents);
    let smoke_gate_position = std::env::var_os("STREAM_TOWN_SMOKE_GATE")
        .is_some()
        .then(|| spawn_positions.first().copied())
        .flatten();
    for position in spawn_positions {
        let world_position = grid_to_world_on_surface(position, &config.0, &generated);
        if spawned == 0
            && std::env::var_os("STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP").is_some()
            && let Ok((mut camera, mut controller)) = cameras.single_mut()
        {
            let transform = Transform::from_xyz(
                world_position.x + 7.0,
                world_position.y + 6.0,
                world_position.z + 7.0,
            )
            .looking_at(world_position + Vec3::Y * 1.6, Vec3::Y);
            *camera = transform;
            controller.set_home(transform);
        }
        let retained = retained_players
            .as_ref()
            .and_then(|players| players.get(usize::from(spawned)));
        let (actor_id, initial_role) = retained.map_or_else(
            || {
                let (actor_id, role) = initial_actor_identity(spawned);
                (
                    StableId::new(actor_id).expect("generated ID"),
                    role.map(|role| StableId::new(role).expect("starting role IDs are valid")),
                )
            },
            |actor| (actor.id.clone(), Some(actor.role.clone())),
        );
        let target = deterministic_wander_target(&generated, &actor_id, position);
        let kind = ActorKind::Player;
        simulation.join_player(actor_id.clone(), position);
        if let Some(role) = initial_role {
            let _ = simulation.assign_role(&actor_id, role);
        }
        let authored_archetype = retained
            .and_then(|actor| actor.archetype.clone())
            .or_else(|| {
                archetype_id_by_source(&content.0, ArchetypeKind::Player, "Player_Character.prefab")
            });
        if let Some(actor) = simulation.actors.get_mut(&actor_id) {
            actor.archetype.clone_from(&authored_archetype);
            if let Some(retained) = retained {
                actor.display_name.clone_from(&retained.display_name);
                actor.login_name.clone_from(&retained.login_name);
                actor.user_type = retained.user_type;
                actor.customization = retained.customization;
                actor.unlocked_pets.clone_from(&retained.unlocked_pets);
                actor.active_pet.clone_from(&retained.active_pet);
                actor.role_progression.clear();
                actor.role_progression.insert(
                    actor.role.clone(),
                    stream_town_domain::RoleProgress::default(),
                );
            }
            if spawned == 0
                && let Some(pet) = &smoke_pet
            {
                actor.unlocked_pets.insert(pet.clone());
                actor.active_pet = Some(pet.clone());
            }
            if spawned == 1 && std::env::var_os("STREAM_TOWN_DEBUG_CARRY").is_some() {
                actor.role = StableId::new("role:logger").expect("static ID");
                actor
                    .inventory
                    .insert(StableId::new("resource:wood").expect("static ID"), 1);
            }
            if let Some(health) = authored_archetype
                .as_ref()
                .and_then(|id| content.0.archetypes.get(id))
                .and_then(|archetype| archetype.health.as_ref())
            {
                actor.max_health = i32::try_from(health.max_health).unwrap_or(i32::MAX);
                actor.health = actor.max_health;
            }
        }
        let real_archetype = (usize::from(spawned) < actor_scene_budget())
            .then(|| {
                authored_archetype
                    .as_ref()
                    .and_then(|id| content.0.archetypes.get(id))
            })
            .flatten();
        let real_scene = real_archetype
            .and_then(runtime_archetype_scene)
            .filter(|scene| {
                asset_server.is_some() && converted_asset_exists(&asset_root.0, &scene.asset_path)
            });
        // Prefer the complete translated Animator controller. A single native
        // imported clip is only a fallback when the authored controller cannot
        // be represented for this rig.
        let converted_animation = std::env::var_os("STREAM_TOWN_SMOKE_STATIC_RIG")
            .is_none()
            .then(|| {
                real_archetype
                    .and_then(|archetype| converted_animation_spec(archetype, &presentation.0))
                    .map(|mut spec| {
                        if let Some(scene) = &real_scene {
                            spec.rig_scene.clone_from(&scene.asset_path);
                        }
                        spec
                    })
            })
            .flatten();
        let native_animation = converted_animation
            .is_none()
            .then(|| {
                real_archetype
                    .zip(real_scene.as_ref())
                    .and_then(|(archetype, scene)| {
                        native_animation_request(archetype, scene, &presentation.0)
                    })
            })
            .flatten();
        let base_scale = if real_scene.is_some() {
            Vec3::splat(config.0.world.cell_size / 2.0)
        } else {
            Vec3::new(
                config.0.world.cell_size * 0.3,
                config.0.world.cell_size * 0.55,
                config.0.world.cell_size * 0.3,
            )
        };
        let visual_height = if real_scene.is_some() {
            world_position.y
        } else {
            world_position.y + base_scale.y * 0.5
        };
        let mut entity = commands.spawn((
            WorldEntity,
            GridLocation(position),
            Agent {
                id: actor_id,
                kind: kind.clone(),
                archetype: authored_archetype
                    .unwrap_or_else(|| StableId::new("archetype:viewer").expect("static ID")),
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
                native: native_animation.is_some() || converted_animation.is_some(),
                ..default()
            },
            Transform::from_xyz(world_position.x, visual_height, world_position.z)
                .with_scale(base_scale),
        ));
        if let Some(scene) = real_scene {
            entity.insert(WorldAssetRoot(
                asset_server
                    .as_deref()
                    .expect("asset server checked above")
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ));
            if let Some(native_animation) = native_animation {
                entity.insert(native_animation);
            } else if let Some(converted_animation) = converted_animation {
                entity.insert(converted_animation);
            }
            if let Some(material) = real_archetype.and_then(|archetype| {
                prefab_material_spec(archetype, &scene, &presentation.0, &render)
            }) {
                entity.insert(material);
            }
            if scene.asset_path != PLAYER_ANIMATED_MODEL_PATH
                && real_archetype.is_some_and(|archetype| archetype.kind == ArchetypeKind::Player)
            {
                entity.insert(PlayerRigAxisCorrectionRequired);
            } else if scene.asset_path == PLAYER_ANIMATED_MODEL_PATH {
                // Tag the converted shipping armature so every visible surface
                // receives the character-compatible shadow path.
                entity.insert(PlayerAnimatedRig);
            }
        } else {
            entity.insert((
                Mesh3d(render.actor_lod.clone()),
                MeshMaterial3d(actor_material(&render, &kind, false)),
            ));
        }
        spawned += 1;
        if spawned >= initial_agents {
            break;
        }
    }

    if std::env::var_os("STREAM_TOWN_SMOKE_PING").is_some() && spawned > 0 {
        agent_commands.0.push_back(AgentCommand::Ping(
            StableId::new("npc:starting_defender").expect("static smoke actor ID"),
        ));
    }

    if let Some(position) = smoke_gate_position {
        let gate_id = StableId::new("building:gate").expect("static gate ID");
        let definition = &content.0.buildings[&gate_id];
        let archetype = &content.0.archetypes[&definition.archetype];
        let runtime_id = StableId::new("building:smoke_gate").expect("static smoke gate ID");
        let state = BuildingState {
            id: runtime_id.clone(),
            archetype: definition.archetype.clone(),
            position,
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(&content.0, definition))
                .unwrap_or(i32::MAX),
            complete: true,
        };
        if let Some(region) = building_navigation_region(
            position,
            definition,
            state.rotation_quarter_turns,
            &generated,
        ) {
            generated
                .navigation
                .set_blocked(region, true)
                .expect("smoke gate footprint updates navigation");
        }
        let age = building_age(&content.0, &simulation, &gate_id, state.level);
        spawn_runtime_building(
            &mut commands,
            &config.0,
            &generated,
            &presentation.0,
            asset_server.as_deref(),
            &asset_root.0,
            &render,
            &state,
            definition,
            archetype,
            position,
            definition.footprint,
            age,
        );
        simulation.buildings.insert(runtime_id, state);
    }

    if std::env::var_os("STREAM_TOWN_SMOKE_CHIMNEY").is_some() {
        let house_id = StableId::new("building:house").expect("static house ID");
        let definition = &content.0.buildings[&house_id];
        let archetype = &content.0.archetypes[&definition.archetype];
        if let Some(position) = find_building_site(&generated, centre, definition.footprint) {
            let runtime_id = StableId::new("building:smoke_chimney").expect("static smoke ID");
            let state = BuildingState {
                id: runtime_id.clone(),
                archetype: definition.archetype.clone(),
                position,
                rotation_quarter_turns: 0,
                level: 1,
                health: i32::try_from(building_base_max_health(&content.0, definition))
                    .unwrap_or(i32::MAX),
                complete: true,
            };
            spawn_runtime_building(
                &mut commands,
                &config.0,
                &generated,
                &presentation.0,
                asset_server.as_deref(),
                &asset_root.0,
                &render,
                &state,
                definition,
                archetype,
                position,
                definition.footprint,
                building_age(&content.0, &simulation, &house_id, state.level),
            );
            simulation.buildings.insert(runtime_id, state);
        }
    }

    let recruit_resource = StableId::new("resource:recruit").expect("static ID");
    simulation.town_resources.insert(
        recruit_resource,
        u32::try_from(capacity_recruited_actor_ids(&simulation).len()).unwrap_or(u32::MAX),
    );

    spawn_hud(
        &mut commands,
        &render,
        spawned,
        &generated.deterministic_hash,
    );
    if std::env::var_os("STREAM_TOWN_SMOKE_HEALING_VFX").is_some() {
        let focus = grid_to_world_on_surface(centre, &config.0, &generated);
        let spacing = config.0.world.cell_size * 3.2;
        spawn_healing_effect(
            &mut commands,
            &presentation.0,
            &render,
            focus - Vec3::X * spacing,
            HealingEffectKind::Channel,
            config.0.world.cell_size,
            None,
        );
        spawn_healing_effect(
            &mut commands,
            &presentation.0,
            &render,
            focus,
            HealingEffectKind::Burst,
            config.0.world.cell_size,
            None,
        );
        spawn_healing_effect(
            &mut commands,
            &presentation.0,
            &render,
            focus + Vec3::X * spacing,
            HealingEffectKind::Revive,
            config.0.world.cell_size,
            None,
        );
    }
    if std::env::var_os("STREAM_TOWN_SMOKE_COMBAT_VFX").is_some() {
        let focus = grid_to_world_on_surface(centre, &config.0, &generated)
            + Vec3::Y * config.0.world.cell_size * 0.35;
        spawn_combat_smoke_field(&mut commands, &render, focus, config.0.world.cell_size);
    }
    if std::env::var_os("STREAM_TOWN_SMOKE_BUILDING_VFX").is_some() {
        let focus = grid_to_world_on_surface(centre, &config.0, &generated);
        spawn_building_smoke_field(&mut commands, &render, focus, config.0.world.cell_size);
    }
    if std::env::var_os("STREAM_TOWN_SMOKE_FLAG").is_some()
        && let Some(asset_server) = asset_server.as_deref()
    {
        let focus = grid_to_world_on_surface(centre, &config.0, &generated);
        spawn_flag_smoke_castle(
            &mut commands,
            &content.0,
            &presentation.0,
            &render,
            asset_server,
            &asset_root.0,
            focus,
            config.0.world.cell_size,
        );
    }
    if std::env::var_os("STREAM_TOWN_SMOKE_GODRAY").is_some()
        && let Some(asset_server) = asset_server.as_deref()
    {
        let focus = grid_to_world_on_surface(centre, &config.0, &generated);
        spawn_godray_smoke_tower(
            &mut commands,
            &content.0,
            &presentation.0,
            &render,
            asset_server,
            &asset_root.0,
            focus,
            config.0.world.cell_size,
        );
    }
    if std::env::var_os("STREAM_TOWN_SMOKE_GIRAFFE").is_some()
        && let Some(asset_server) = asset_server.as_deref()
    {
        let focus = grid_to_world_on_surface(centre, &config.0, &generated);
        spawn_giraffe_smoke_pet(
            &mut commands,
            &content.0,
            &presentation.0,
            &render,
            asset_server,
            &asset_root.0,
            focus,
        );
    }
    spawn_seagull(
        &mut commands,
        &render,
        asset_server.as_deref(),
        &asset_root.0,
        generated.seed,
    );
    if std::env::var_os("STREAM_TOWN_SMOKE_SELECTION").is_some() {
        selected.0 = Some(town_hall_placement);
    }
    let resource_entity_count = if isolate_animation {
        0
    } else {
        generated.resources.len()
    };
    let foliage_entity_count = render_stats.foliage_instances;
    commands.insert_resource(WorldRuntime {
        generated,
    });
    commands.insert_resource(SimulationRuntime(simulation));
    if retained_players.is_some() {
        restart.retained_players = None;
        restart.suppress_exit_save = false;
        io.save = true;
    }
    commands.insert_resource(EnvironmentPresentation::default());
    loading.phase = WorldLoadingPhase::Complete;
    loading.work.terrain_entities = LoadingWork::count(2, 2);
    loading.work.resource_entities =
        LoadingWork::count(resource_entity_count, resource_entity_count);
    loading.work.foliage_entities = LoadingWork::count(foliage_entity_count, foliage_entity_count);
    loading.work.actor_entities = LoadingWork::count(usize::from(spawned), usize::from(spawned));
    loading.work.gameplay_setup = LoadingWork::boolean(true);
    loading.progress = loading.work.progress();
    "Finalizing town presentation".clone_into(&mut loading.status);
    "Waiting for initial scene roots to instantiate".clone_into(&mut loading.substatus);
    loading.completion_remaining_seconds = Duration::from_millis(u64::from(
        content.0.loading_screen.completion_hold_milliseconds,
    ))
    .as_secs_f32();
}
