use super::super::*;

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
        legacy_terrain_mesh: None,
        legacy_migration: None,
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

pub(crate) fn nearest_walkable(world: &GeneratedWorld, desired: GridPos) -> Option<GridPos> {
    let desired = GridPos {
        x: desired.x.min(world.navigation.width() - 1),
        z: desired.z.min(world.navigation.height() - 1),
    };
    if world.navigation.is_walkable(desired) {
        return Some(desired);
    }
    let limit = world.navigation.width().max(world.navigation.height());
    for radius in 1..limit {
        for z in desired.z.saturating_sub(radius)
            ..=desired
                .z
                .saturating_add(radius)
                .min(world.navigation.height() - 1)
        {
            for x in desired.x.saturating_sub(radius)
                ..=desired
                    .x
                    .saturating_add(radius)
                    .min(world.navigation.width() - 1)
            {
                let candidate = GridPos { x, z };
                if world.navigation.is_walkable(candidate) {
                    return Some(candidate);
                }
            }
        }
    }
    None
}

pub(crate) fn restored_actor_position(
    world: &GeneratedWorld,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    kind: &ActorKind,
    desired: GridPos,
) -> Option<GridPos> {
    let desired = GridPos {
        x: desired.x.min(world.navigation.width() - 1),
        z: desired.z.min(world.navigation.height() - 1),
    };
    if *kind == ActorKind::Player
        && completed_player_gate_cells(content, simulation).contains(&desired)
    {
        Some(desired)
    } else {
        nearest_walkable(world, desired)
    }
}

pub(crate) fn initial_actor_position(
    world: &GeneratedWorld,
    excluded: GridPos,
    actor_index: usize,
) -> Option<GridPos> {
    let mut found = 0;
    for z in 0..world.navigation.height() {
        for x in 0..world.navigation.width() {
            let position = GridPos { x, z };
            if position == excluded || !world.navigation.is_walkable(position) {
                continue;
            }
            if found == actor_index {
                return Some(position);
            }
            found += 1;
        }
    }
    None
}

pub(crate) fn connected_actor_positions(
    world: &GeneratedWorld,
    start: GridPos,
    excluded: GridPos,
    count: u16,
) -> Vec<GridPos> {
    let mut positions = Vec::with_capacity(usize::from(count));
    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::from([start]);
    while let Some(position) = queue.pop_front() {
        if !visited.insert(position) || !world.navigation.is_walkable(position) {
            continue;
        }
        if position != excluded {
            positions.push(position);
            if positions.len() >= usize::from(count) {
                break;
            }
        }
        queue.extend(
            world
                .navigation
                .walkable_neighbours(position)
                .into_iter()
                .flatten()
                .map(|(neighbour, _)| neighbour),
        );
    }
    positions
}

pub(crate) fn resource_for_role(content: &ContentCatalog, role: &StableId) -> Option<StableId> {
    content.roles.get(role)?.resource.clone()
}

pub(crate) fn actor_resource_storage_has_room(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
) -> bool {
    if simulation.active_community_event == Some(CommunityEvent::Market) {
        return true;
    }
    let Some(resource) = resource_for_role(content, &actor.role) else {
        return true;
    };
    simulation
        .town_resources
        .get(&resource)
        .copied()
        .unwrap_or_default()
        < resource_storage_capacity(config, content, simulation, &resource)
}

pub(crate) fn actor_remaining_carry_capacity(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
) -> u32 {
    let carried = resource_for_role(content, &actor.role)
        .and_then(|resource| actor.inventory.get(&resource).copied())
        .unwrap_or_default();
    let capacity = effective_role_stats(content, simulation, actor)
        .map(|stats| stats.carry_capacity)
        .filter(|capacity| *capacity > 0)
        .unwrap_or(25);
    capacity.saturating_sub(carried)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct EffectiveRoleStats {
    pub(crate) level: u16,
    pub(crate) experience: u32,
    pub(crate) required_experience: u32,
    pub(crate) experience_multiplier_per_thousand: u32,
    pub(crate) action_amount: u32,
    pub(crate) action_milliseconds: u32,
    pub(crate) action_range_milli_cells: u32,
    pub(crate) max_health: u32,
    pub(crate) health_regen_milli_per_second: i64,
    pub(crate) damage_reduction_percent: u32,
    pub(crate) movement_speed_milli_cells_per_second: u32,
    pub(crate) carry_capacity: u32,
}

pub(crate) fn leveled_whole_stat(base: u32, per_level_milli: u32, level: u16) -> u32 {
    let increase =
        u64::from(per_level_milli).saturating_mul(u64::from(level.saturating_sub(1))) / 1_000;
    base.saturating_add(u32::try_from(increase).unwrap_or(u32::MAX))
}

pub(crate) fn leveled_percentage_stat(base: u32, per_level_per_thousand: u32, level: u16) -> u32 {
    let increase = u64::from(base)
        .saturating_mul(u64::from(per_level_per_thousand))
        .saturating_mul(u64::from(level.saturating_sub(1)))
        / 1_000;
    base.saturating_add(u32::try_from(increase).unwrap_or(u32::MAX))
}

pub(crate) fn building_level_bonus_per_thousand(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
    bonus: impl Fn(&BuildingDef) -> u32,
) -> u32 {
    actor
        .station
        .as_ref()
        .and_then(|station| simulation.buildings.get(station))
        .filter(|station| station.complete && station.health > 0)
        .and_then(|station| {
            let definition = building_def_for_archetype(content, &station.archetype)?;
            definition.station.as_ref()?;
            Some(bonus(definition).saturating_mul(u32::from(station.level.saturating_sub(1))))
        })
        .unwrap_or_default()
}

pub(crate) fn global_gather_rate_bonus_per_thousand(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> u32 {
    simulation
        .buildings
        .values()
        .filter(|building| building.complete && building.health > 0)
        .filter_map(|building| {
            building_def_for_archetype(content, &building.archetype).map(|definition| {
                definition
                    .global_gather_rate_bonus_per_level_per_thousand
                    .saturating_mul(u32::from(building.level.saturating_sub(1)))
            })
        })
        .fold(0_u32, u32::saturating_add)
}

pub(crate) fn military_kill_experience_bonus_per_thousand(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> u32 {
    simulation
        .buildings
        .values()
        .filter(|building| building.complete && building.health > 0)
        .filter_map(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            Some(
                definition
                    .kill_experience_bonus_per_level_per_thousand
                    .saturating_mul(u32::from(building.level.saturating_sub(1))),
            )
        })
        .fold(0_u32, u32::saturating_add)
}

pub(crate) fn role_progress(actor: &ActorState) -> stream_town_domain::RoleProgress {
    actor
        .role_progression
        .get(&actor.role)
        .copied()
        .unwrap_or_default()
}

pub(crate) fn effective_role_stats(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
) -> Option<EffectiveRoleStats> {
    let definition = content.roles.get(&actor.role)?;
    let progress = role_progress(actor);
    let levels = u32::from(progress.level.saturating_sub(1));
    let stat = |name: &str| {
        technology_stat_boost_percent(
            content,
            simulation,
            &actor.role,
            &StableId::new(format!("stat:{name}")).expect("static stat IDs are valid"),
        )
    };
    let base_action_milliseconds = definition
        .base_action_milliseconds
        .saturating_sub(
            definition
                .action_milliseconds_reduction_per_level
                .saturating_mul(levels),
        )
        .max(content.progression.minimum_action_milliseconds);
    // Unity retains ActionRangePerLevel in RoleData, but PlayerRoleData gates it
    // behind `_ranged`; the shipping constructor never sets that TODO field.
    // Keep the converted value available to tools without activating dead data.
    let base_range = definition.base_action_range_milli_cells;
    let base_movement = definition
        .base_movement_speed_milli_cells_per_second
        .saturating_add(
            definition
                .movement_speed_milli_cells_per_second_per_level
                .saturating_mul(levels),
        );
    let base_defense = leveled_whole_stat(
        u32::try_from(definition.base_damage_reduction_percent.max(0)).unwrap_or_default(),
        definition.damage_reduction_milli_percent_per_level,
        progress.level,
    );
    let base_regen = i64::from(definition.base_health_regen_per_second)
        .saturating_mul(1_000)
        .saturating_add(
            i64::from(definition.health_regen_milli_per_second_per_level)
                .saturating_mul(i64::from(levels)),
        );
    let max_health = percentage_adjusted(
        leveled_percentage_stat(
            definition.base_health,
            definition.health_bonus_per_level_per_thousand,
            progress.level,
        ),
        stat("health"),
    )
    .max(1);
    let max_health = if is_guardhouse_defender_id(&actor.id) {
        (max_health / 2).max(1)
    } else {
        max_health
    };

    let station_experience_bonus =
        building_level_bonus_per_thousand(content, simulation, actor, |building| {
            building
                .station
                .as_ref()
                .map_or(0, |station| station.experience_bonus_per_level_per_thousand)
        });
    let experience_multiplier_per_thousand = u32::try_from(
        u64::from(definition.experience_multiplier_per_thousand).saturating_mul(u64::from(
            1_000_u32.saturating_add(station_experience_bonus),
        )) / 1_000,
    )
    .unwrap_or(u32::MAX);
    let gather_rate_bonus = definition.resource.as_ref().map_or(0, |_| {
        global_gather_rate_bonus_per_thousand(content, simulation)
    });
    let gathered_action_milliseconds = u32::try_from(
        u64::from(base_action_milliseconds).saturating_mul(1_000)
            / u64::from(1_000_u32.saturating_add(gather_rate_bonus)),
    )
    .unwrap_or(u32::MAX)
    .max(content.progression.minimum_action_milliseconds);

    Some(EffectiveRoleStats {
        level: progress.level,
        experience: progress.experience,
        required_experience: stream_town_domain::required_role_experience(
            progress.level,
            content.progression.role_experience_curve_level_span,
            content.progression.role_experience_curve_maximum,
        ),
        experience_multiplier_per_thousand,
        action_amount: percentage_adjusted(
            leveled_whole_stat(
                definition.base_action_amount,
                definition.action_amount_per_level_milli,
                progress.level,
            ),
            stat("action_amount"),
        ),
        action_milliseconds: percentage_reduced(gathered_action_milliseconds, stat("action_speed"))
            .max(content.progression.minimum_action_milliseconds),
        action_range_milli_cells: percentage_adjusted(base_range, stat("action_range")),
        max_health,
        health_regen_milli_per_second: percentage_adjusted_i64(base_regen, stat("health_regen")),
        damage_reduction_percent: percentage_adjusted(base_defense, stat("defense")),
        movement_speed_milli_cells_per_second: percentage_adjusted(
            base_movement,
            stat("movement_speed"),
        ),
        carry_capacity: percentage_adjusted(
            leveled_whole_stat(
                definition.base_carry_capacity,
                definition.carry_capacity_per_level_milli,
                progress.level,
            ),
            stat("resource_carry"),
        ),
    })
}

pub(crate) fn role_action_range_milli_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
) -> u32 {
    if let Some(enemy) =
        actor_archetype(content, actor).and_then(|archetype| archetype.enemy.as_ref())
    {
        return enemy.action_range_milli_cells;
    }
    effective_role_stats(content, simulation, actor)
        .map_or(1_000, |stats| stats.action_range_milli_cells)
}

pub(crate) fn within_milli_cell_range(
    left: GridPos,
    right: GridPos,
    range_milli_cells: u32,
) -> bool {
    grid_distance_squared_milli_cells(left, right)
        <= u128::from(range_milli_cells).saturating_mul(u128::from(range_milli_cells))
}

pub(crate) fn grid_distance_squared_milli_cells(left: GridPos, right: GridPos) -> u128 {
    u128::from(grid_distance_squared(left, right)).saturating_mul(1_000_000)
}

pub(crate) fn actor_target_size_milli_cells(content: &ContentCatalog, actor: &ActorState) -> u32 {
    actor_archetype(content, actor).map_or(0, |archetype| archetype.target_size_milli_cells)
}

pub(crate) fn building_target_size_milli_cells(
    content: &ContentCatalog,
    building: &BuildingState,
) -> u32 {
    content
        .archetypes
        .get(&building.archetype)
        .map_or(0, |archetype| archetype.target_size_milli_cells)
}

pub(crate) fn within_actor_attack_range(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    attacker: &ActorState,
    target: &ActorState,
    current: GridPos,
) -> bool {
    let range = u128::from(role_action_range_milli_cells(content, simulation, attacker));
    let target_size = u128::from(actor_target_size_milli_cells(content, target));
    // Unity's shipping action states compare squared world distance against
    // `(range * 2[.5]) + SizeSqr` (rather than squaring the range). After
    // converting two world units to one cell, the coefficients are 1.0 for
    // enemies and 1.25 for players.
    let range_term = if attacker.role.as_str() == "role:enemy" {
        range.saturating_mul(1_000)
    } else {
        range.saturating_mul(1_250)
    };
    grid_distance_squared_milli_cells(current, target.position)
        <= range_term.saturating_add(target_size.saturating_mul(target_size))
}

pub(crate) fn within_actor_heal_range(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    healer: &ActorState,
    target: &ActorState,
    current: GridPos,
) -> bool {
    let range = u128::from(role_action_range_milli_cells(content, simulation, healer));
    let target_size = u128::from(actor_target_size_milli_cells(content, target));
    grid_distance_squared_milli_cells(current, target.position)
        <= range
            .saturating_mul(1_000)
            .saturating_add(target_size.saturating_mul(target_size))
}

pub(crate) fn within_actor_heal_hysteresis_range(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    healer: &ActorState,
    target: &ActorState,
    current: GridPos,
) -> bool {
    let range = u128::from(role_action_range_milli_cells(content, simulation, healer));
    let target_size = u128::from(actor_target_size_milli_cells(content, target));
    let normal_squared = range
        .saturating_mul(1_000)
        .saturating_add(target_size.saturating_mul(target_size));
    grid_distance_squared_milli_cells(current, target.position).saturating_mul(10_000)
        <= normal_squared
            .saturating_mul(HEALING_HYSTERESIS_PERCENT)
            .saturating_mul(HEALING_HYSTERESIS_PERCENT)
}

pub(crate) fn retained_healing_goal(
    goal: &AgentGoal,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor_id: &StableId,
    current: GridPos,
) -> Option<(AgentGoal, GridPos)> {
    let AgentGoal::Heal(target_id) = goal else {
        return None;
    };
    let healer = simulation.actors.get(actor_id)?;
    let target = simulation.actors.get(target_id)?;
    (healer.alive
        && target.alive
        && target.role.as_str() != "role:enemy"
        && target.health < target.max_health
        && within_actor_heal_hysteresis_range(content, simulation, healer, target, current))
    .then(|| (AgentGoal::Heal(target_id.clone()), current))
}

pub(crate) fn citizen_should_retreat(actor: &ActorState, was_retreating: bool) -> bool {
    if !actor.alive || actor.role.as_str() == "role:enemy" || actor.max_health <= 0 {
        return false;
    }
    if was_retreating {
        actor.health < actor.max_health
    } else {
        i64::from(actor.health.max(0)).saturating_mul(100)
            < i64::from(actor.max_health).saturating_mul(i64::from(RETREAT_HEALTH_PERCENT))
    }
}

pub(crate) fn within_enemy_building_attack_range(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    attacker: &ActorState,
    building: &BuildingState,
    current: GridPos,
) -> bool {
    let range = u128::from(role_action_range_milli_cells(content, simulation, attacker));
    let target_size = u128::from(building_target_size_milli_cells(content, building));
    grid_distance_squared_milli_cells(current, building_visual_grid(content, building))
        <= range
            .saturating_mul(1_000)
            .saturating_add(target_size.saturating_mul(target_size))
}

pub(crate) fn within_building_work_range(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    builder: &ActorState,
    building: &BuildingState,
    current: GridPos,
) -> bool {
    if building_def_for_archetype(content, &building.archetype).is_some_and(|definition| {
        let footprint = rotated_footprint(definition.footprint, building.rotation_quarter_turns);
        let maximum = GridPos {
            x: building
                .position
                .x
                .saturating_add(footprint[0].saturating_sub(1)),
            z: building
                .position
                .z
                .saturating_add(footprint[1].saturating_sub(1)),
        };
        let dx = if current.x < building.position.x {
            building.position.x - current.x
        } else {
            current.x.saturating_sub(maximum.x)
        };
        let dz = if current.z < building.position.z {
            building.position.z - current.z
        } else {
            current.z.saturating_sub(maximum.z)
        };
        (dx != 0 || dz != 0) && dx <= 1 && dz <= 1
    }) {
        // Every cell offered by `building_approaches` must also be a legal
        // action cell. Previously corner approaches to one-cell paths and
        // walls were planned successfully, but rejected here after the build
        // animation, leaving builders in a permanent no-progress loop.
        return true;
    }
    let range = u128::from(role_action_range_milli_cells(content, simulation, builder));
    let target_size = u128::from(building_target_size_milli_cells(content, building));
    let maximum = range
        .saturating_mul(5)
        .div_ceil(2)
        .saturating_add(target_size);
    grid_distance_squared_milli_cells(current, building_visual_grid(content, building))
        <= maximum.saturating_mul(maximum)
}

pub(crate) fn is_combat_role(role: &StableId) -> bool {
    matches!(
        role.as_str(),
        "role:defender"
            | "role:necromancer"
            | "role:paladin"
            | "role:ranger"
            | "role:ruler"
            | "role:soldier"
            | "role:wizard"
    )
}

pub(crate) fn is_healer_role(role: &StableId) -> bool {
    role.as_str() == "role:priest"
}

pub(crate) fn enemy_targets_kind(
    content: &ContentCatalog,
    actor: &ActorState,
    target: &str,
) -> bool {
    actor_archetype(content, actor)
        .and_then(|archetype| archetype.enemy.as_ref())
        .is_some_and(|enemy| {
            enemy.targets_all || enemy.target_kinds.iter().any(|id| id.as_str() == target)
        })
}

pub(crate) fn enemy_targets_buildings(content: &ContentCatalog, actor: &ActorState) -> bool {
    [
        "target:building",
        "target:damaged_building",
        "target:construction",
    ]
    .into_iter()
    .any(|target| enemy_targets_kind(content, actor, target))
}

pub(crate) fn enemy_target_search_range_milli_cells(
    content: &ContentCatalog,
    actor: &ActorState,
) -> Option<u32> {
    actor_archetype(content, actor)
        .and_then(|archetype| archetype.enemy.as_ref())
        .map(|enemy| enemy.target_search_range_milli_cells)
}

pub(crate) fn enemy_attacks_attacker(content: &ContentCatalog, actor: &ActorState) -> bool {
    actor_archetype(content, actor)
        .and_then(|archetype| archetype.enemy.as_ref())
        .is_some_and(|enemy| enemy.attack_attacker)
}

pub(crate) fn within_enemy_target_search(
    content: &ContentCatalog,
    actor: &ActorState,
    current: GridPos,
    target: GridPos,
) -> bool {
    enemy_target_search_range_milli_cells(content, actor)
        .is_some_and(|range| within_milli_cell_range(current, target, range))
}

pub(crate) fn is_ranged_role(role: &StableId) -> bool {
    matches!(
        role.as_str(),
        "role:necromancer" | "role:ranger" | "role:wizard"
    )
}

pub(crate) fn actor_combat_visual(role: &StableId) -> CombatVisualKind {
    match role.as_str() {
        "role:wizard" => CombatVisualKind::Fireball,
        "role:necromancer" => CombatVisualKind::Necrotic,
        "role:ranger" => CombatVisualKind::Arrow,
        _ => CombatVisualKind::Physical,
    }
}

pub(crate) fn actor_archetype<'a>(
    content: &'a ContentCatalog,
    actor: &ActorState,
) -> Option<&'a ArchetypeDef> {
    if let Some(archetype) = actor
        .archetype
        .as_ref()
        .and_then(|archetype| content.archetypes.get(archetype))
    {
        return Some(archetype);
    }
    let (kind, source) = if actor.role.as_str() == "role:enemy" {
        (ArchetypeKind::Enemy, "Enemy_Goblin.prefab")
    } else {
        (ArchetypeKind::Player, "Player_Character.prefab")
    };
    archetype_by_source(content, kind, source)
}

pub(crate) fn town_hall_grid_position(config: &GameConfig) -> GridPos {
    GridPos {
        x: (config.world.width / 2 + 4).min(config.world.width - 2),
        z: config.world.height / 2,
    }
}

pub(crate) fn town_hall_placement_position(config: &GameConfig, footprint: [u16; 2]) -> GridPos {
    let centre = town_hall_grid_position(config);
    GridPos {
        x: centre.x.saturating_sub(footprint[0] / 2),
        z: centre.z.saturating_sub(footprint[1] / 2),
    }
}

pub(crate) fn ensure_town_hall_state(
    content: &ContentCatalog,
    config: &GameConfig,
    simulation: &mut WorldSimulation,
) {
    let id = StableId::new("building:townhall").expect("static ID");
    if simulation.buildings.contains_key(&id) {
        return;
    }
    let definition = &content.buildings[&id];
    simulation.buildings.insert(
        id.clone(),
        BuildingState {
            id,
            archetype: definition.archetype.clone(),
            position: town_hall_placement_position(config, definition.footprint),
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(content, definition))
                .unwrap_or(i32::MAX),
            complete: true,
        },
    );
}

#[derive(Clone, Copy)]
pub(crate) struct StationCandidate<'a> {
    pub(crate) id: &'a StableId,
    pub(crate) position: GridPos,
    pub(crate) definition: &'a StationDef,
}

pub(crate) fn ensure_actor_station(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    config: &GameConfig,
    actor_id: &StableId,
) {
    let replacement = simulation.actors.get(actor_id).and_then(|actor| {
        let best = best_station_id(content, simulation, config, &actor.role, actor.position);
        (actor.station != best).then_some(best)
    });
    if let Some(station) = replacement
        && let Some(actor) = simulation.actors.get_mut(actor_id)
    {
        actor.station = station;
    }
}

pub(crate) fn station_matches_role(
    station: &StationDef,
    role: &stream_town_domain::RoleDef,
) -> bool {
    station.accepts_all_roles
        || role
            .station_kinds
            .iter()
            .any(|kind| station.accepted_role_kinds.contains(kind))
}

pub(crate) fn station_supports_role_targets(
    station: &StationDef,
    role: &stream_town_domain::RoleDef,
) -> bool {
    station.targets_all
        || role.targets_all
        || role
            .target_kinds
            .iter()
            .any(|kind| station.target_kinds.contains(kind))
}

pub(crate) fn role_accepts_target(
    role: &stream_town_domain::RoleDef,
    target_kind: &StableId,
) -> bool {
    role.targets_all || role.target_kinds.contains(target_kind)
}

pub(crate) fn actor_accepts_resource(
    content: &ContentCatalog,
    actor: &ActorState,
    resource: &stream_town_domain::GeneratedResource,
) -> bool {
    content.roles.get(&actor.role).is_some_and(|role| {
        role.resource.as_ref() == Some(&resource.kind)
            && role_accepts_target(role, &resource.target_kind)
    })
}

pub(crate) fn station_candidate<'a>(
    content: &'a ContentCatalog,
    simulation: &'a WorldSimulation,
    config: &GameConfig,
    station_id: &'a StableId,
) -> Option<StationCandidate<'a>> {
    let Some(state) = simulation.buildings.get(station_id) else {
        if station_id.as_str() != "building:townhall" {
            return None;
        }
        let building = content.buildings.get(station_id)?;
        return Some(StationCandidate {
            id: station_id,
            position: town_hall_grid_position(config),
            definition: building.station.as_ref()?,
        });
    };
    let state = state.complete.then_some(state)?;
    let building = building_def_for_archetype(content, &state.archetype)?;
    let footprint = rotated_footprint(building.footprint, state.rotation_quarter_turns);
    Some(StationCandidate {
        id: station_id,
        position: GridPos {
            x: state.position.x.saturating_add(footprint[0] / 2),
            z: state.position.z.saturating_add(footprint[1] / 2),
        },
        definition: building.station.as_ref()?,
    })
}

pub(crate) fn best_station_id(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    config: &GameConfig,
    role_id: &StableId,
    from: GridPos,
) -> Option<StableId> {
    let role = content.roles.get(role_id)?;
    if role.station_kinds.is_empty() {
        return None;
    }
    let town_hall = StableId::new("building:townhall").expect("static building ID is valid");
    std::iter::once(&town_hall)
        .chain(
            simulation
                .buildings
                .keys()
                .filter(|id| id.as_str() != "building:townhall"),
        )
        .filter_map(|id| station_candidate(content, simulation, config, id))
        .filter(|station| {
            station_matches_role(station.definition, role)
                && station_supports_role_targets(station.definition, role)
        })
        .min_by_key(|station| {
            (
                grid_distance_squared(station.position, from),
                station.id.clone(),
            )
        })
        .map(|station| station.id.clone())
}

pub(crate) fn restored_town_hall_position(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    config: &GameConfig,
) -> GridPos {
    let town_hall = StableId::new("building:townhall").expect("static building ID is valid");
    station_candidate(content, simulation, config, &town_hall).map_or_else(
        || town_hall_grid_position(config),
        |station| station.position,
    )
}

pub(crate) fn assigned_station<'a>(
    content: &'a ContentCatalog,
    simulation: &'a WorldSimulation,
    config: &GameConfig,
    actor: &'a ActorState,
) -> Option<StationCandidate<'a>> {
    actor
        .station
        .as_ref()
        .and_then(|id| station_candidate(content, simulation, config, id))
        .filter(|station| {
            content.roles.get(&actor.role).is_some_and(|role| {
                station_matches_role(station.definition, role)
                    && station_supports_role_targets(station.definition, role)
            })
        })
}

#[cfg(test)]
pub(crate) fn station_deposit_approach(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    station: StationCandidate<'_>,
    current: GridPos,
) -> Option<GridPos> {
    simulation
        .buildings
        .get(station.id)
        .and_then(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            building_approach(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                current,
            )
        })
        .or_else(|| nearest_walkable(world, station.position))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn unoccupied_station_deposit_approach(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    station: StationCandidate<'_>,
    current: GridPos,
    actor: &StableId,
    occupied: &BTreeMap<GridPos, StableId>,
    reservations: &BTreeMap<GridPos, StableId>,
) -> Option<GridPos> {
    simulation
        .buildings
        .get(station.id)
        .and_then(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            unoccupied_building_approach(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                current,
                actor,
                occupied,
                reservations,
            )
        })
        .or_else(|| nearest_walkable(world, station.position))
}

pub(crate) fn station_search_range_cells(station: StationCandidate<'_>) -> u16 {
    u16::try_from(station.definition.search_range_milli_cells.div_ceil(1_000)).unwrap_or(u16::MAX)
}

pub(crate) fn grid_distance_squared(left: GridPos, right: GridPos) -> u64 {
    let x = u64::from(left.x.abs_diff(right.x));
    let z = u64::from(left.z.abs_diff(right.z));
    x * x + z * z
}

pub(crate) fn target_score_milli(
    target: GridPos,
    actor: GridPos,
    assigned_count: u32,
    scoring: &TargetingScoreDef,
) -> u128 {
    // Keep Unity's Euclidean `Vector3.Distance` ordering without introducing
    // platform-dependent floating-point decisions into the simulation. The
    // square root is evaluated in thousandths of a logical grid cell.
    let distance_milli_cells = grid_distance_squared(target, actor)
        .saturating_mul(1_000_000)
        .isqrt();
    u128::from(distance_milli_cells)
        .saturating_mul(u128::from(scoring.distance_penalty_milli_per_cell))
        .div_ceil(1_000)
        .saturating_add(
            u128::from(assigned_count).saturating_mul(u128::from(scoring.assignment_penalty_milli)),
        )
}

pub(crate) fn within_station_search_region(
    position: GridPos,
    station: StationCandidate<'_>,
) -> bool {
    let range = station_search_range_cells(station);
    position.x.abs_diff(station.position.x) <= range
        && position.z.abs_diff(station.position.z) <= range
}

pub(crate) fn station_target_is_reachable(
    world: &GeneratedWorld,
    station: StationCandidate<'_>,
    target_kind: &StableId,
    target: GridPos,
) -> bool {
    // Unity intentionally skips its path check for fish, which are approached
    // from the shoreline rather than from the target's water node.
    if target_kind.as_str() == "target:fish" {
        return true;
    }
    let Some(start) = nearest_walkable(world, station.position) else {
        return false;
    };
    let Some(goal) = nearest_walkable(world, target) else {
        return false;
    };
    world.navigation.find_path(start, goal).is_ok()
}

pub(crate) fn stable_station_timer_offset(station: &StableId, update_milliseconds: u32) -> f64 {
    // Unity starts every station at a random point inside its authored refresh
    // interval. A stable FNV-1a offset preserves that stagger without allowing
    // platform RNG or frame order to alter target selection.
    let hash = station
        .as_str()
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3)
        });
    f64::from(u32::try_from(hash % u64::from(update_milliseconds)).unwrap_or_default())
}

pub(crate) fn active_station_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    config: &GameConfig,
) -> Vec<StableId> {
    let town_hall = StableId::new("building:townhall").expect("static building ID");
    let mut stations = BTreeSet::from([town_hall]);
    stations.extend(
        simulation.buildings.keys().filter_map(|id| {
            station_candidate(content, simulation, config, id).map(|_| id.clone())
        }),
    );
    stations.into_iter().collect()
}

pub(crate) fn station_target_position(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    resource_index: &StationResourceTargetIndex,
    kind: &StableId,
    target: &StableId,
) -> Option<GridPos> {
    match kind.as_str() {
        "target:tree" | "target:ore" | "target:bush" | "target:fish" => resource_index
            .by_id
            .get(target)
            .and_then(|index| world.resources.get(*index))
            .filter(|resource| {
                resource.id == *target && resource.amount > 0 && resource.target_kind == *kind
            })
            .map(|resource| resource.position),
        "target:farm" => simulation
            .buildings
            .get(target)
            .filter(|building| is_farm_resource_building(content, building))
            .map(|building| building_visual_grid(content, building)),
        "target:enemy" | "target:boss" => simulation
            .actors
            .get(target)
            .filter(|actor| actor.alive && actor.role.as_str() == "role:enemy")
            .filter(|actor| {
                let is_boss = actor_archetype(content, actor)
                    .and_then(|archetype| archetype.enemy.as_ref())
                    .is_some_and(|enemy| enemy.enemy_type.as_str().ends_with("_boss"));
                is_boss == (kind.as_str() == "target:boss")
            })
            .map(|actor| actor.position),
        "target:player" => simulation
            .actors
            .get(target)
            .filter(|actor| {
                actor.alive
                    && actor.role.as_str() != "role:enemy"
                    && actor.health == actor.max_health
            })
            .map(|actor| actor.position),
        "target:injured_player" => simulation
            .actors
            .get(target)
            .filter(|actor| {
                actor.alive
                    && actor.role.as_str() != "role:enemy"
                    && actor.health < actor.max_health
            })
            .map(|actor| actor.position),
        "target:dead_player" => simulation
            .actors
            .get(target)
            .filter(|actor| !actor.alive && actor.role.as_str() != "role:enemy")
            .map(|actor| actor.position),
        "target:construction" => simulation
            .buildings
            .get(target)
            .filter(|building| !building.complete && building.health > 0)
            .map(|building| building_visual_grid(content, building)),
        "target:damaged_building" => simulation
            .buildings
            .get(target)
            .filter(|building| {
                building.complete
                    && building.health > 0
                    && building.health < building_max_health(content, building)
            })
            .map(|building| building_visual_grid(content, building)),
        "target:building" => simulation
            .buildings
            .get(target)
            .filter(|building| {
                building.complete
                    && building.health == building_max_health(content, building)
                    && !is_farm_resource_building(content, building)
            })
            .map(|building| building_visual_grid(content, building)),
        _ => None,
    }
}

pub(crate) fn populate_station_target_cache(
    runtime: &mut StationTargetRuntime,
    resource_index: &mut StationResourceTargetIndex,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    station_id: &StableId,
) {
    resource_index.sync(world);
    let Some(station) = station_candidate(content, simulation, config, station_id) else {
        runtime.stations.remove(station_id);
        return;
    };
    let target_kinds = if station.definition.targets_all {
        content
            .station_target_update_modes
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>()
    } else {
        station.definition.target_kinds.clone()
    };
    let cache = runtime.stations.entry(station_id.clone()).or_default();
    cache.targets.retain(|kind, _| target_kinds.contains(kind));
    for kind in target_kinds {
        let mut candidates = if matches!(
            kind.as_str(),
            "target:tree" | "target:ore" | "target:bush" | "target:fish"
        ) {
            resource_index
                .by_kind
                .get(&kind)
                .into_iter()
                .flatten()
                .filter_map(|index| world.resources.get(*index))
                .filter(|resource| resource.amount > 0 && resource.target_kind == kind)
                .map(|resource| (resource.position, resource.id.clone()))
                .collect()
        } else {
            station_target_catalog_candidates(content, simulation, world, &kind)
        };
        candidates.retain(|(position, _)| within_station_search_region(*position, station));
        candidates.sort_by_key(|(position, id)| {
            (
                grid_distance_squared(*position, station.position),
                id.clone(),
            )
        });
        let available = candidates
            .iter()
            .map(|(_, id)| id.clone())
            .collect::<BTreeSet<_>>();
        let previous = cache.targets.get(&kind).cloned().unwrap_or_default();
        let mut next = match content.station_target_update_modes.get(&kind) {
            Some(StationUpdateMode::Update) => previous
                .iter()
                .filter(|target| available.contains(*target))
                .cloned()
                .collect::<Vec<_>>(),
            Some(StationUpdateMode::Clear) | None => Vec::new(),
        };
        for (_, target) in candidates {
            if next.len() >= usize::from(station.definition.max_targets) {
                break;
            }
            if !next.contains(&target) {
                next.push(target);
            }
        }
        for target in next.iter().filter(|target| !previous.contains(*target)) {
            let queued = (kind.clone(), target.clone());
            if !cache.reachability_queue.contains(&queued) {
                cache.reachability_queue.push_back(queued);
            }
        }
        cache.targets.insert(kind, next);
    }
}

#[cfg(test)]
pub(crate) fn immediate_station_target_runtime(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
) -> StationTargetRuntime {
    let mut runtime = StationTargetRuntime::default();
    let mut resource_index = StationResourceTargetIndex::default();
    for station_id in active_station_ids(content, simulation, config) {
        if let Some(station) = station_candidate(content, simulation, config, &station_id) {
            runtime.stations.insert(
                station_id.clone(),
                CachedStationTargets {
                    refresh_elapsed_milliseconds: stable_station_timer_offset(
                        &station_id,
                        station.definition.update_milliseconds,
                    ),
                    ..default()
                },
            );
            populate_station_target_cache(
                &mut runtime,
                &mut resource_index,
                content,
                simulation,
                world,
                config,
                &station_id,
            );
        }
    }
    let station_ids = runtime.stations.keys().cloned().collect::<Vec<_>>();
    for station_id in station_ids {
        let Some(station) = station_candidate(content, simulation, config, &station_id) else {
            continue;
        };
        let cached = runtime
            .stations
            .get(&station_id)
            .map(|cache| cache.targets.clone())
            .unwrap_or_default();
        for (kind, targets) in cached {
            let reachable = targets
                .into_iter()
                .filter(|target| {
                    station_target_position(
                        content,
                        simulation,
                        world,
                        &resource_index,
                        &kind,
                        target,
                    )
                    .is_some_and(|position| {
                        station_target_is_reachable(world, station, &kind, position)
                    })
                })
                .collect();
            if let Some(cache) = runtime.stations.get_mut(&station_id) {
                cache.targets.insert(kind, reachable);
            }
        }
    }
    runtime
}

pub(crate) fn refresh_station_target_runtime(
    cadence: Res<AgentSimulationCadence>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    config: Res<RuntimeConfig>,
    mut runtime: ResMut<StationTargetRuntime>,
    mut resource_index: ResMut<StationResourceTargetIndex>,
) {
    resource_index.sync(&world.generated);
    let active = active_station_ids(&content.0, &simulation.0, &config.0);
    let active_set = active.iter().cloned().collect::<BTreeSet<_>>();
    runtime.stations.retain(|id, _| active_set.contains(id));
    runtime.refresh_queue.retain(|id| active_set.contains(id));

    for station_id in &active {
        if runtime.stations.contains_key(station_id) {
            continue;
        }
        let Some(station) = station_candidate(&content.0, &simulation.0, &config.0, station_id)
        else {
            continue;
        };
        runtime.stations.insert(
            station_id.clone(),
            CachedStationTargets {
                refresh_elapsed_milliseconds: stable_station_timer_offset(
                    station_id,
                    station.definition.update_milliseconds,
                ),
                ..default()
            },
        );
        populate_station_target_cache(
            &mut runtime,
            &mut resource_index,
            &content.0,
            &simulation.0,
            &world.generated,
            &config.0,
            station_id,
        );
    }

    let delta_milliseconds = cadence.delta().as_secs_f64() * 1_000.0;
    let mut due = Vec::new();
    for station_id in &active {
        let Some(station) = station_candidate(&content.0, &simulation.0, &config.0, station_id)
        else {
            continue;
        };
        let cache = runtime
            .stations
            .get_mut(station_id)
            .expect("active stations are initialized above");
        cache.refresh_elapsed_milliseconds += delta_milliseconds;
        cache.target_check_elapsed_milliseconds += delta_milliseconds;
        if cache.refresh_elapsed_milliseconds >= f64::from(station.definition.update_milliseconds) {
            cache.refresh_elapsed_milliseconds -= f64::from(station.definition.update_milliseconds);
            due.push(station_id.clone());
        }
    }
    for station_id in due {
        if !runtime.refresh_queue.contains(&station_id) {
            runtime.refresh_queue.push_back(station_id);
        }
    }

    // Unity's StationProcessor repopulates at most one queued station per
    // rendered frame, even when several authored timers expire together.
    if let Some(station_id) = runtime.refresh_queue.pop_front() {
        populate_station_target_cache(
            &mut runtime,
            &mut resource_index,
            &content.0,
            &simulation.0,
            &world.generated,
            &config.0,
            &station_id,
        );
    }

    for station_id in active {
        let check = runtime.stations.get_mut(&station_id).and_then(|cache| {
            if cache.target_check_elapsed_milliseconds < STATION_TARGET_CHECK_MILLISECONDS {
                return None;
            }
            cache.target_check_elapsed_milliseconds -= STATION_TARGET_CHECK_MILLISECONDS;
            cache.reachability_queue.pop_front()
        });
        let Some((kind, target)) = check else {
            continue;
        };
        let reachable = station_candidate(&content.0, &simulation.0, &config.0, &station_id)
            .and_then(|station| {
                station_target_position(
                    &content.0,
                    &simulation.0,
                    &world.generated,
                    &resource_index,
                    &kind,
                    &target,
                )
                .map(|position| {
                    station_target_is_reachable(&world.generated, station, &kind, position)
                })
            })
            .unwrap_or(false);
        let cache = runtime
            .stations
            .get_mut(&station_id)
            .expect("active station cache exists");
        if reachable {
            cache.reachability_queue.push_back((kind, target));
        } else if let Some(targets) = cache.targets.get_mut(&kind) {
            targets.retain(|candidate| candidate != &target);
        }
    }
}

pub(crate) fn cached_station_targets<'a>(
    runtime: &'a StationTargetRuntime,
    station: StationCandidate<'_>,
    kind: &StableId,
) -> &'a [StableId] {
    runtime
        .stations
        .get(station.id)
        .and_then(|cache| cache.targets.get(kind))
        .map(Vec::as_slice)
        .unwrap_or_default()
}

pub(crate) fn within_player_target_search_region(position: GridPos, player: GridPos) -> bool {
    position.x.abs_diff(player.x) <= PLAYER_TARGET_SEARCH_RANGE_CELLS
        && position.z.abs_diff(player.z) <= PLAYER_TARGET_SEARCH_RANGE_CELLS
}

pub(crate) fn building_def_for_archetype<'a>(
    content: &'a ContentCatalog,
    archetype: &StableId,
) -> Option<&'a BuildingDef> {
    content
        .buildings
        .values()
        .find(|building| building.archetype == *archetype)
}

pub(crate) fn is_farm_resource_building(
    content: &ContentCatalog,
    building: &BuildingState,
) -> bool {
    building.complete
        && content
            .buildings
            .get(&StableId::new("building:farm").expect("static building ID"))
            .is_some_and(|farm| farm.archetype == building.archetype)
}

pub(crate) fn resource_approach(
    world: &GeneratedWorld,
    resource: &stream_town_domain::GeneratedResource,
    from: GridPos,
) -> Option<GridPos> {
    if world.navigation.is_walkable(resource.position) {
        return Some(resource.position);
    }
    if resource.target_kind.as_str() == "target:fish" {
        return stream_town_domain::shoreline_approaches(&world.navigation, resource.position)
            .min_by_key(|candidate| {
                (
                    candidate.x.abs_diff(from.x) + candidate.z.abs_diff(from.z),
                    candidate.z,
                    candidate.x,
                )
            });
    }
    let radius = 1_u16;
    let min_x = resource.position.x.saturating_sub(radius);
    let max_x = resource
        .position
        .x
        .saturating_add(radius)
        .min(world.navigation.width() - 1);
    let min_z = resource.position.z.saturating_sub(radius);
    let max_z = resource
        .position
        .z
        .saturating_add(radius)
        .min(world.navigation.height() - 1);
    let mut approaches = Vec::new();
    for z in min_z..=max_z {
        for x in min_x..=max_x {
            let candidate = GridPos { x, z };
            if candidate != resource.position && world.navigation.is_walkable(candidate) {
                approaches.push(candidate);
            }
        }
    }
    approaches.sort_by_key(|candidate| {
        (
            candidate.x.abs_diff(from.x) + candidate.z.abs_diff(from.z),
            candidate.x.abs_diff(resource.position.x) + candidate.z.abs_diff(resource.position.z),
            candidate.z,
            candidate.x,
        )
    });
    approaches
        .into_iter()
        .next()
        .or_else(|| nearest_walkable(world, resource.position))
}

pub(crate) fn is_current_building_approach(
    world: &GeneratedWorld,
    content: &ContentCatalog,
    building: &BuildingState,
    current: GridPos,
) -> bool {
    building_def_for_archetype(content, &building.archetype).and_then(|definition| {
        building_approach(
            world,
            building.position,
            rotated_footprint(definition.footprint, building.rotation_quarter_turns),
            current,
        )
    }) == Some(current)
}

pub(crate) fn goal_reservation(goal: &AgentGoal) -> Option<&StableId> {
    match goal {
        // Unity's zero-assignment claim is part of the data-driven generated
        // resource pipeline. Targetable farm holders continue to use their
        // station's authored max-target capacity.
        AgentGoal::Gather(target) => Some(target),
        _ => None,
    }
}

pub(crate) fn goal_assignment_target(goal: &AgentGoal) -> Option<&StableId> {
    match goal {
        AgentGoal::HarvestFarm(target) => Some(target),
        _ => None,
    }
}

pub(crate) fn goal_planting_target(goal: &AgentGoal) -> Option<GridPos> {
    match goal {
        AgentGoal::PlantTree(target) | AgentGoal::PlantBush(target) => Some(*target),
        _ => None,
    }
}

pub(crate) fn goal_reserves_approach(goal: &AgentGoal) -> bool {
    matches!(
        goal,
        AgentGoal::HarvestFarm(_)
            | AgentGoal::Construct(_)
            | AgentGoal::Deposit
            | AgentGoal::VisitRegenerationStation(_)
            | AgentGoal::PlantTree(_)
            | AgentGoal::Prospect { .. }
            | AgentGoal::PlantBush(_)
            | AgentGoal::Wander
    )
}

pub(crate) fn goal_uses_idle_approach_reservation(goal: &AgentGoal) -> bool {
    matches!(goal, AgentGoal::Wander)
}

pub(crate) fn approach_reservation_available(
    reservations: &BTreeMap<GridPos, StableId>,
    actor: &StableId,
    target: GridPos,
) -> bool {
    reservations.get(&target).is_none_or(|owner| owner == actor)
}

pub(crate) fn reservation_available(
    reservations: &BTreeMap<StableId, StableId>,
    actor: &StableId,
    target: &StableId,
) -> bool {
    reservations.get(target).is_none_or(|owner| owner == actor)
}

pub(crate) fn goal_reservation_is_valid(
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    content: &ContentCatalog,
    actor_id: &StableId,
    goal: &AgentGoal,
) -> bool {
    let Some(actor) = simulation.actors.get(actor_id).filter(|actor| actor.alive) else {
        return false;
    };
    match goal {
        AgentGoal::Gather(resource_id) => world
            .resources
            .iter()
            .find(|resource| resource.id == *resource_id && resource.amount > 0)
            .is_some_and(|resource| actor_accepts_resource(content, actor, resource)),
        AgentGoal::HarvestFarm(building_id) => {
            content.roles.get(&actor.role).is_some_and(|role| {
                role_accepts_target(
                    role,
                    &StableId::new("target:farm").expect("static target ID"),
                )
            }) && simulation
                .buildings
                .get(building_id)
                .is_some_and(|building| is_farm_resource_building(content, building))
        }
        _ => false,
    }
}

pub(crate) fn building_max_health_at_level(
    content: &ContentCatalog,
    archetype: &StableId,
    level: u16,
) -> u32 {
    let base_health = content
        .archetypes
        .get(archetype)
        .and_then(|definition| definition.health.as_ref())
        .expect("validated building archetypes have authored health")
        .max_health;
    let definition = building_def_for_archetype(content, archetype)
        .expect("validated building archetypes have a building definition");
    leveled_percentage_stat(
        base_health,
        definition.health_bonus_per_level_per_thousand,
        level,
    )
}

pub(crate) fn building_max_health(content: &ContentCatalog, building: &BuildingState) -> i32 {
    i32::try_from(building_max_health_at_level(
        content,
        &building.archetype,
        building.level,
    ))
    .unwrap_or(i32::MAX)
}

pub(crate) fn building_base_max_health(content: &ContentCatalog, definition: &BuildingDef) -> u32 {
    building_max_health_at_level(content, &definition.archetype, 1)
}

pub(crate) fn normalize_building_health(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
) {
    for building in simulation.buildings.values_mut() {
        let Some(definition) = building_def_for_archetype(content, &building.archetype) else {
            continue;
        };
        let Some(base_health) = content
            .archetypes
            .get(&building.archetype)
            .and_then(|archetype| archetype.health.as_ref())
            .map(|health| health.max_health)
        else {
            continue;
        };
        let max_health = i32::try_from(leveled_percentage_stat(
            base_health,
            definition.health_bonus_per_level_per_thousand,
            building.level,
        ))
        .unwrap_or(i32::MAX);
        building.health = building.health.clamp(0, max_health);
        if !building.complete && building.health >= max_health {
            building.complete = true;
        }
    }
}

pub(crate) fn building_visual_grid(content: &ContentCatalog, building: &BuildingState) -> GridPos {
    let footprint = building_def_for_archetype(content, &building.archetype)
        .map_or([1, 1], |definition| {
            rotated_footprint(definition.footprint, building.rotation_quarter_turns)
        });
    GridPos {
        x: building.position.x.saturating_add(footprint[0] / 2),
        z: building.position.z.saturating_add(footprint[1] / 2),
    }
}

pub(crate) fn building_approach(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
    from: GridPos,
) -> Option<GridPos> {
    building_approaches(world, position, footprint, from)
        .into_iter()
        .next()
}

pub(crate) fn building_approaches(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
    from: GridPos,
) -> Vec<GridPos> {
    building_perimeter_candidates(world, position, footprint, from)
        .into_iter()
        .filter(|candidate| world.navigation.is_walkable(*candidate))
        .collect()
}

pub(crate) fn building_perimeter_candidates(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
    from: GridPos,
) -> Vec<GridPos> {
    let Some(region) = building_region(position, footprint, world) else {
        return Vec::new();
    };
    let min_x = region.min.x.saturating_sub(1);
    let min_z = region.min.z.saturating_sub(1);
    let max_x = region
        .max
        .x
        .saturating_add(1)
        .min(world.navigation.width() - 1);
    let max_z = region
        .max
        .z
        .saturating_add(1)
        .min(world.navigation.height() - 1);
    let mut approaches = Vec::new();
    for z in min_z..=max_z {
        for x in min_x..=max_x {
            let candidate = GridPos { x, z };
            let outside =
                x < region.min.x || x > region.max.x || z < region.min.z || z > region.max.z;
            if outside {
                approaches.push(candidate);
            }
        }
    }
    approaches.sort_by_key(|candidate| {
        (
            candidate.x.abs_diff(from.x) + candidate.z.abs_diff(from.z),
            candidate.z,
            candidate.x,
        )
    });
    approaches
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn unoccupied_building_approach(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
    from: GridPos,
    actor: &StableId,
    occupied: &BTreeMap<GridPos, StableId>,
    reservations: &BTreeMap<GridPos, StableId>,
) -> Option<GridPos> {
    building_approaches(world, position, footprint, from)
        .into_iter()
        .find(|candidate| {
            occupied.get(candidate).is_none_or(|owner| owner == actor)
                && reservations
                    .get(candidate)
                    .is_none_or(|owner| owner == actor)
        })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn unoccupied_construction_approach(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
    from: GridPos,
    actor: &StableId,
    occupied: &BTreeMap<GridPos, StableId>,
    reservations: &BTreeMap<GridPos, StableId>,
) -> Option<GridPos> {
    // Coarse navigation marks a whole placement cell blocked when any building
    // occupies it. Construction uses the fine thirds grid, where part of that
    // same cell can still be open and reachable around a dense floorplan.
    building_perimeter_candidates(world, position, footprint, from)
        .into_iter()
        .find(|candidate| {
            occupied.get(candidate).is_none_or(|owner| owner == actor)
                && reservations
                    .get(candidate)
                    .is_none_or(|owner| owner == actor)
        })
}

#[cfg(test)]
pub(crate) fn next_agent_goal(
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    content: &ContentCatalog,
    actor_id: &StableId,
    current: GridPos,
) -> (AgentGoal, GridPos) {
    let station_targets = immediate_station_target_runtime(content, simulation, world, config);
    next_agent_goal_with_station_runtime(
        simulation,
        world,
        None,
        config,
        content,
        &station_targets,
        actor_id,
        current,
        placement_to_navigation_centre(current),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
    )
}

#[allow(clippy::too_many_arguments)]
#[cfg(test)]
pub(crate) fn next_agent_goal_with_reservations(
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    content: &ContentCatalog,
    actor_id: &StableId,
    current: GridPos,
    reservations: &BTreeMap<StableId, StableId>,
    target_assignments: &BTreeMap<StableId, u32>,
) -> (AgentGoal, GridPos) {
    let station_targets = immediate_station_target_runtime(content, simulation, world, config);
    next_agent_goal_with_station_runtime(
        simulation,
        world,
        None,
        config,
        content,
        &station_targets,
        actor_id,
        current,
        placement_to_navigation_centre(current),
        reservations,
        target_assignments,
        &BTreeMap::new(),
        &BTreeMap::new(),
    )
}

pub(crate) fn town_hall_wait_target(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    current: GridPos,
) -> GridPos {
    let town_hall = StableId::new("building:townhall").expect("static building ID");
    simulation
        .buildings
        .get(&town_hall)
        .and_then(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            building_approach(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                current,
            )
        })
        .or_else(|| {
            nearest_walkable(
                world,
                restored_town_hall_position(content, simulation, config),
            )
        })
        .unwrap_or(current)
}

pub(crate) fn nearest_reachable_building_to_town_hall(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    current: GridPos,
) -> Option<(StableId, GridPos)> {
    let town_hall = StableId::new("building:townhall").expect("static building ID");
    let town_hall_position = simulation
        .buildings
        .get(&town_hall)
        .map_or(current, |building| building_visual_grid(content, building));
    let mut candidates = simulation
        .buildings
        .values()
        .filter(|building| building.health > 0)
        .filter_map(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            if !building_blocks_navigation(definition) {
                return None;
            }
            Some((
                grid_distance_squared(building_visual_grid(content, building), town_hall_position),
                building.id.clone(),
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
            ))
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|(town_hall_distance, id, _, _)| (*town_hall_distance, id.clone()));
    candidates
        .into_iter()
        .find_map(|(_, id, position, footprint)| {
            reachable_building_approach(world, position, footprint, current)
                .map(|approach| (id, approach))
        })
}

pub(crate) fn enemy_navigation_can_reach(
    world: &GeneratedWorld,
    current: GridPos,
    goal: GridPos,
) -> bool {
    let exceptions = if world.navigation.is_walkable(current) {
        HashSet::new()
    } else {
        HashSet::from([current])
    };
    world
        .navigation
        .find_path_with_exceptions(current, goal, &exceptions)
        .is_ok()
}

pub(crate) fn reachable_building_approach(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
    current: GridPos,
) -> Option<GridPos> {
    building_approaches(world, position, footprint, current)
        .into_iter()
        .find(|approach| enemy_navigation_can_reach(world, current, *approach))
}

pub(crate) fn enemy_route_buildings(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    navigation: &stream_town_domain::NavGrid,
) -> Vec<EnemyRouteBuilding> {
    let town_hall = StableId::new("building:townhall").expect("static building ID");
    let town_hall_position = simulation
        .buildings
        .get(&town_hall)
        .map_or(GridPos { x: 0, z: 0 }, |building| {
            building_visual_grid(content, building)
        });
    simulation
        .buildings
        .values()
        .filter(|building| building.health > 0)
        .filter_map(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            if !building_blocks_navigation(definition) {
                return None;
            }
            let approaches = building_approaches(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                town_hall_position,
            )
            .into_iter()
            .map(placement_to_navigation_centre)
            .filter(|approach| navigation.is_walkable(*approach))
            .collect::<Vec<_>>();
            (!approaches.is_empty()).then(|| EnemyRouteBuilding {
                id: building.id.clone(),
                town_hall_distance: grid_distance_squared(
                    building_visual_grid(content, building),
                    town_hall_position,
                ),
                approaches,
            })
        })
        .collect()
}

pub(crate) fn enemy_navigation_signature(
    navigation: &stream_town_domain::NavGrid,
    buildings: &[EnemyRouteBuilding],
) -> u64 {
    fn fold_u64(mut hash: u64, value: u64) -> u64 {
        for byte in value.to_le_bytes() {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    let mut hash = fold_u64(0xcbf2_9ce4_8422_2325, u64::from(navigation.width()));
    hash = fold_u64(hash, u64::from(navigation.height()));
    hash = fold_u64(hash, navigation.topology_signature());
    for building in buildings {
        hash = fold_u64(hash, stable_id_hash(&building.id));
        hash = fold_u64(hash, building.town_hall_distance);
        for approach in &building.approaches {
            hash = fold_u64(hash, u64::from(approach.x) | (u64::from(approach.z) << 16));
        }
    }
    hash
}

pub(crate) fn grid_octile_distance(left: GridPos, right: GridPos) -> u32 {
    let x = u32::from(left.x.abs_diff(right.x));
    let z = u32::from(left.z.abs_diff(right.z));
    let diagonal = x.min(z);
    diagonal * 14 + (x.max(z) - diagonal) * 10
}

pub(crate) fn build_enemy_navigation_field(
    signature: u64,
    navigation: stream_town_domain::NavGrid,
    buildings: Vec<EnemyRouteBuilding>,
) -> EnemyNavigationField {
    const BLOCKED_COMPONENT: u32 = u32::MAX;

    let width = navigation.width();
    let height = navigation.height();
    let cell_count = usize::from(width) * usize::from(height);
    let index =
        |position: GridPos| usize::from(position.z) * usize::from(width) + usize::from(position.x);
    let mut component_by_cell = vec![BLOCKED_COMPONENT; cell_count];
    let mut component_count = 0_u32;
    for z in 0..height {
        for x in 0..width {
            let start = GridPos { x, z };
            if !navigation.is_walkable(start)
                || component_by_cell[index(start)] != BLOCKED_COMPONENT
            {
                continue;
            }
            let component = component_count;
            component_count = component_count.saturating_add(1);
            component_by_cell[index(start)] = component;
            let mut queue = VecDeque::from([start]);
            while let Some(current) = queue.pop_front() {
                for (neighbour, _) in navigation
                    .walkable_neighbours(current)
                    .into_iter()
                    .flatten()
                {
                    let neighbour_index = index(neighbour);
                    if component_by_cell[neighbour_index] == BLOCKED_COMPONENT {
                        component_by_cell[neighbour_index] = component;
                        queue.push_back(neighbour);
                    }
                }
            }
        }
    }

    let component_count = usize::try_from(component_count).unwrap_or(0);
    let mut selected = vec![None::<(u64, StableId)>; component_count];
    for building in &buildings {
        for approach in &building.approaches {
            let component = component_by_cell[index(*approach)];
            if component == BLOCKED_COMPONENT {
                continue;
            }
            let candidate = (building.town_hall_distance, building.id.clone());
            let selected = &mut selected[usize::try_from(component).expect("component fits usize")];
            if selected.as_ref().is_none_or(|current| candidate < *current) {
                *selected = Some(candidate);
            }
        }
    }
    let target_by_component = selected
        .iter()
        .map(|candidate| candidate.as_ref().map(|(_, id)| id.clone()))
        .collect::<Vec<_>>();
    let mut next_by_cell = vec![None; cell_count];
    let mut goal_by_cell = vec![None; cell_count];
    let mut route_costs = vec![u32::MAX; cell_count];
    let mut queue = BinaryHeap::new();
    for building in &buildings {
        for approach in &building.approaches {
            let approach_index = index(*approach);
            let component = component_by_cell[approach_index];
            if component == BLOCKED_COMPONENT
                || target_by_component[usize::try_from(component).expect("component fits usize")]
                    .as_ref()
                    != Some(&building.id)
                || goal_by_cell[approach_index].is_some()
            {
                continue;
            }
            goal_by_cell[approach_index] = Some(*approach);
            route_costs[approach_index] = 0;
            queue.push(EnemyPathOpenNode {
                position: *approach,
                estimated_total: 0,
                cost: 0,
            });
        }
    }
    while let Some(current) = queue.pop() {
        let current_index = index(current.position);
        if current.cost > route_costs[current_index] {
            continue;
        }
        let component = component_by_cell[current_index];
        let goal = goal_by_cell[current_index].expect("queued route cells have a goal");
        for (neighbour, step_cost) in navigation
            .walkable_neighbours(current.position)
            .into_iter()
            .flatten()
        {
            let neighbour_index = index(neighbour);
            if component_by_cell[neighbour_index] != component {
                continue;
            }
            let next_cost = current.cost.saturating_add(step_cost);
            if next_cost < route_costs[neighbour_index] {
                route_costs[neighbour_index] = next_cost;
                next_by_cell[neighbour_index] = Some(current.position);
                goal_by_cell[neighbour_index] = Some(goal);
                queue.push(EnemyPathOpenNode {
                    position: neighbour,
                    estimated_total: next_cost,
                    cost: next_cost,
                });
            }
        }
    }

    let mut cluster_edge_sets = BTreeMap::<EnemyClusterNode, BTreeSet<EnemyClusterNode>>::new();
    for z in 0..height {
        for x in 0..width {
            let current = GridPos { x, z };
            let current_index = index(current);
            let component = component_by_cell[current_index];
            if component == BLOCKED_COMPONENT {
                continue;
            }
            let current_cluster = EnemyClusterNode {
                x: x / ENEMY_NAVIGATION_CLUSTER_SIZE,
                z: z / ENEMY_NAVIGATION_CLUSTER_SIZE,
                component,
            };
            cluster_edge_sets.entry(current_cluster).or_default();
            for (neighbour, _) in navigation
                .walkable_neighbours(current)
                .into_iter()
                .flatten()
            {
                let neighbour_component = component_by_cell[index(neighbour)];
                if neighbour_component != component {
                    continue;
                }
                let neighbour_cluster = EnemyClusterNode {
                    x: neighbour.x / ENEMY_NAVIGATION_CLUSTER_SIZE,
                    z: neighbour.z / ENEMY_NAVIGATION_CLUSTER_SIZE,
                    component,
                };
                if neighbour_cluster != current_cluster {
                    cluster_edge_sets
                        .entry(current_cluster)
                        .or_default()
                        .insert(neighbour_cluster);
                    cluster_edge_sets
                        .entry(neighbour_cluster)
                        .or_default()
                        .insert(current_cluster);
                }
            }
        }
    }
    let cluster_edges = cluster_edge_sets
        .into_iter()
        .map(|(cluster, neighbours)| (cluster, neighbours.into_iter().collect()))
        .collect();

    EnemyNavigationField {
        signature,
        width,
        component_by_cell,
        target_by_component,
        next_by_cell,
        goal_by_cell,
        cluster_edges,
    }
}

pub(crate) fn refresh_enemy_navigation(
    world: Res<WorldRuntime>,
    simulation: Res<SimulationRuntime>,
    content: Res<RuntimeContent>,
    fine_navigation: Res<FineNavigationRuntime>,
    mut runtime: ResMut<EnemyNavigationRuntime>,
) {
    let Some(navigation) = fine_navigation.grid.as_ref() else {
        return;
    };
    let buildings = enemy_route_buildings(&content.0, &simulation.0, &world.generated, navigation);
    let signature = enemy_navigation_signature(navigation, &buildings);
    let completed = runtime
        .task
        .as_mut()
        .and_then(|pending| block_on(poll_once(&mut pending.task)));
    if let Some(field) = completed {
        runtime.task = None;
        if field.signature == signature {
            runtime.field = Some(field);
        }
    }
    if runtime
        .field
        .as_ref()
        .is_some_and(|field| field.signature == signature)
        || runtime
            .task
            .as_ref()
            .is_some_and(|pending| pending.signature == signature)
    {
        return;
    }
    if runtime.task.is_some() {
        return;
    }

    let navigation = navigation.clone();
    if runtime.field.is_none() {
        runtime.field = Some(build_enemy_navigation_field(
            signature, navigation, buildings,
        ));
    } else if let Some(pool) = AsyncComputeTaskPool::try_get() {
        runtime.task = Some(EnemyNavigationTask {
            signature,
            task: pool.spawn(async move {
                build_enemy_navigation_field(signature, navigation, buildings)
            }),
        });
    } else {
        runtime.field = Some(build_enemy_navigation_field(
            signature, navigation, buildings,
        ));
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn next_agent_goal_with_station_runtime(
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    enemy_navigation: Option<&EnemyNavigationField>,
    config: &GameConfig,
    content: &ContentCatalog,
    station_targets: &StationTargetRuntime,
    actor_id: &StableId,
    current: GridPos,
    current_navigation: GridPos,
    reservations: &BTreeMap<StableId, StableId>,
    target_assignments: &BTreeMap<StableId, u32>,
    occupied_approaches: &BTreeMap<GridPos, StableId>,
    approach_reservations: &BTreeMap<GridPos, StableId>,
) -> (AgentGoal, GridPos) {
    let Some(actor) = simulation.actors.get(actor_id) else {
        return (
            AgentGoal::Wander,
            deterministic_wander_target(world, actor_id, current),
        );
    };
    if !actor.alive {
        return (AgentGoal::Wander, current);
    }
    let fallback_station =
        best_station_id(content, simulation, config, &actor.role, actor.position);
    let station = assigned_station(content, simulation, config, actor).or_else(|| {
        fallback_station
            .as_ref()
            .and_then(|id| station_candidate(content, simulation, config, id))
    });
    if let Some(preferred) = actor.preferred_target.as_ref() {
        if actor.role.as_str() == "role:enemy" {
            if let Some(target) = simulation
                .actors
                .get(preferred)
                .filter(|target| target.alive && target.role.as_str() != "role:enemy")
                .filter(|target| {
                    enemy_targets_kind(content, actor, "target:player")
                        || (target.health < target.max_health
                            && enemy_targets_kind(content, actor, "target:injured_player"))
                })
            {
                return (
                    AgentGoal::Attack(target.id.clone()),
                    if within_actor_attack_range(content, simulation, actor, target, current) {
                        current
                    } else {
                        target.position
                    },
                );
            }
            if let Some(building) = simulation
                .buildings
                .get(preferred)
                .filter(|building| building.health > 0)
                .filter(|building| enemy_can_attack_building(content, building))
                .filter(|building| {
                    enemy_targets_kind(content, actor, "target:building")
                        || (!building.complete
                            && enemy_targets_kind(content, actor, "target:construction"))
                        || (building.complete
                            && building.health < building_max_health(content, building)
                            && enemy_targets_kind(content, actor, "target:damaged_building"))
                })
                && let Some(definition) = building_def_for_archetype(content, &building.archetype)
                && let Some(approach) = building_approach(
                    world,
                    building.position,
                    rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                    current,
                )
            {
                return (
                    AgentGoal::AttackBuilding(building.id.clone()),
                    if within_enemy_building_attack_range(
                        content, simulation, actor, building, current,
                    ) {
                        current
                    } else {
                        approach
                    },
                );
            }
        }
        if let Some(target) = simulation
            .actors
            .get(preferred)
            .filter(|target| target.alive && target.role.as_str() == "role:enemy")
            .filter(|_| is_combat_role(&actor.role))
        {
            return (
                AgentGoal::Attack(target.id.clone()),
                if within_actor_attack_range(content, simulation, actor, target, current) {
                    current
                } else {
                    target.position
                },
            );
        }
        if let Some(target) = simulation
            .actors
            .get(preferred)
            .filter(|target| {
                target.alive
                    && target.role.as_str() != "role:enemy"
                    && target.health < target.max_health
            })
            .filter(|_| is_healer_role(&actor.role))
        {
            return (
                AgentGoal::Heal(target.id.clone()),
                if within_actor_heal_range(content, simulation, actor, target, current) {
                    current
                } else {
                    target.position
                },
            );
        }
        if let Some(resource) = world
            .resources
            .iter()
            .find(|resource| resource.id == *preferred && resource.amount > 0)
            .filter(|resource| {
                station.is_some_and(|station| {
                    cached_station_targets(station_targets, station, &resource.target_kind)
                        .contains(&resource.id)
                })
            })
            .filter(|resource| reservation_available(reservations, actor_id, &resource.id))
            .filter(|resource| actor_accepts_resource(content, actor, resource))
            .filter(|_| actor_remaining_carry_capacity(content, simulation, actor) > 0)
            .filter(|_| actor_resource_storage_has_room(config, content, simulation, actor))
            && let Some(approach) = resource_approach(world, resource, current)
        {
            return (AgentGoal::Gather(resource.id.clone()), approach);
        }
        if let Some(building) = simulation
            .buildings
            .get(preferred)
            .filter(|building| is_farm_resource_building(content, building))
            .filter(|building| {
                let farm = StableId::new("target:farm").expect("static target ID");
                station.is_some_and(|station| {
                    cached_station_targets(station_targets, station, &farm).contains(&building.id)
                })
            })
            .filter(|building| reservation_available(reservations, actor_id, &building.id))
            .filter(|_| {
                content.roles.get(&actor.role).is_some_and(|role| {
                    role_accepts_target(
                        role,
                        &StableId::new("target:farm").expect("static target ID"),
                    )
                })
            })
            .filter(|_| actor_remaining_carry_capacity(content, simulation, actor) > 0)
            .filter(|_| actor_resource_storage_has_room(config, content, simulation, actor))
            && let Some(definition) = building_def_for_archetype(content, &building.archetype)
            && let Some(approach) = unoccupied_building_approach(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                current,
                actor_id,
                occupied_approaches,
                approach_reservations,
            )
        {
            return (AgentGoal::HarvestFarm(building.id.clone()), approach);
        }
        if let Some(building) = simulation.buildings.get(preferred).filter(|building| {
            actor.role.as_str() == "role:builder"
                && (!building.complete || building.health < building_max_health(content, building))
        }) && let Some(definition) = building_def_for_archetype(content, &building.archetype)
            && let Some(approach) = unoccupied_construction_approach(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                current,
                actor_id,
                occupied_approaches,
                approach_reservations,
            )
        {
            return (AgentGoal::Construct(building.id.clone()), approach);
        }
    }
    if is_healer_role(&actor.role) {
        let candidates: Vec<_> = simulation
            .actors
            .values()
            .filter(|target| {
                target.id != actor.id
                    && target.alive
                    && target.role.as_str() != "role:enemy"
                    && target.health < target.max_health
            })
            .filter(|target| within_player_target_search_region(target.position, current))
            .collect();
        if let Some(target) = candidates.into_iter().min_by_key(|target| {
            (
                grid_distance_squared(target.position, current),
                target.id.clone(),
            )
        }) {
            return (
                AgentGoal::Heal(target.id.clone()),
                if within_actor_heal_range(content, simulation, actor, target, current) {
                    current
                } else {
                    target.position
                },
            );
        }
    }
    if actor.role.as_str() == "role:enemy" {
        let player_target = enemy_targets_kind(content, actor, "target:player")
            .then(|| {
                simulation
                    .actors
                    .values()
                    .filter(|target| target.alive && target.role.as_str() != "role:enemy")
                    .filter(|target| {
                        within_enemy_target_search(content, actor, current, target.position)
                    })
                    .map(|target| {
                        (
                            grid_distance_squared(target.position, current),
                            target.id.clone(),
                            target.position,
                        )
                    })
                    .min_by_key(|(distance, id, _)| (*distance, id.clone()))
            })
            .flatten();
        let building_target = enemy_targets_buildings(content, actor)
            .then(|| {
                simulation
                    .buildings
                    .values()
                    .filter(|building| building.health > 0)
                    .filter(|building| enemy_can_attack_building(content, building))
                    .filter(|building| {
                        within_enemy_target_search(
                            content,
                            actor,
                            current,
                            building_visual_grid(content, building),
                        )
                    })
                    .filter_map(|building| {
                        let definition = building_def_for_archetype(content, &building.archetype)?;
                        let approach = building_approach(
                            world,
                            building.position,
                            rotated_footprint(
                                definition.footprint,
                                building.rotation_quarter_turns,
                            ),
                            current,
                        )?;
                        Some((
                            grid_distance_squared(building_visual_grid(content, building), current),
                            building.id.clone(),
                            approach,
                        ))
                    })
                    .min_by_key(|(distance, id, _)| (*distance, id.clone()))
            })
            .flatten();
        match (player_target, building_target) {
            (Some((player_distance, player, position)), Some((building_distance, _, _)))
                if player_distance <= building_distance =>
            {
                let target = &simulation.actors[&player];
                return (
                    AgentGoal::Attack(player),
                    if within_actor_attack_range(content, simulation, actor, target, current) {
                        current
                    } else {
                        position
                    },
                );
            }
            (_, Some((_, building, approach))) => {
                let target = &simulation.buildings[&building];
                return (
                    AgentGoal::AttackBuilding(building),
                    if within_enemy_building_attack_range(
                        content, simulation, actor, target, current,
                    ) {
                        current
                    } else {
                        approach
                    },
                );
            }
            (Some((_, player, position)), None) => {
                let target = &simulation.actors[&player];
                return (
                    AgentGoal::Attack(player),
                    if within_actor_attack_range(content, simulation, actor, target, current) {
                        current
                    } else {
                        position
                    },
                );
            }
            (None, None) => {}
        }
        if enemy_targets_buildings(content, actor) {
            // Advance on the Town Hall when it is reachable. If fortifications
            // seal it off, attack the reachable building closest to it instead
            // of repeatedly requesting an impossible A* path and idling.
            let route = enemy_navigation
                .and_then(|navigation| navigation.destination(current_navigation))
                .and_then(|(building, approach)| {
                    simulation
                        .buildings
                        .get(building)
                        .filter(|building| building.health > 0)
                        .map(|building| (building.id.clone(), navigation_to_placement(approach)))
                })
                .or_else(|| {
                    if enemy_navigation.is_none() {
                        nearest_reachable_building_to_town_hall(content, simulation, world, current)
                    } else {
                        None
                    }
                });
            if let Some((building_id, approach)) = route {
                let building = &simulation.buildings[&building_id];
                return (
                    AgentGoal::AttackBuilding(building_id),
                    if within_enemy_building_attack_range(
                        content, simulation, actor, building, current,
                    ) {
                        current
                    } else {
                        approach
                    },
                );
            }
        }
    }
    let combat_target = if is_combat_role(&actor.role) {
        let defender_anchor = (actor.role.as_str() == "role:defender").then(|| {
            guardhouse_for_defender(simulation, &actor.id).map_or_else(
                || restored_town_hall_position(content, simulation, config),
                |building| building_visual_grid(content, building),
            )
        });
        simulation
            .actors
            .values()
            .filter(|target| target.alive && target.role.as_str() == "role:enemy")
            .filter(|target| within_player_target_search_region(target.position, current))
            .min_by_key(|target| {
                let actor_distance = grid_distance_squared(target.position, current);
                let nearby = defender_anchor.is_some()
                    && actor_distance <= u64::from(DEFENDER_LOCAL_PRIORITY_RADIUS_CELLS).pow(2);
                (
                    u8::from(defender_anchor.is_some() && !nearby),
                    if nearby {
                        actor_distance
                    } else {
                        defender_anchor.map_or(actor_distance, |anchor| {
                            grid_distance_squared(target.position, anchor)
                        })
                    },
                    actor_distance,
                    target.id.clone(),
                )
            })
    } else {
        None
    };
    if let Some(target) = combat_target {
        let destination = if within_actor_attack_range(content, simulation, actor, target, current)
        {
            current
        } else {
            target.position
        };
        return (AgentGoal::Attack(target.id.clone()), destination);
    }
    if actor.role.as_str() == "role:builder" {
        let candidates: Vec<_> = simulation
            .buildings
            .values()
            .filter(|building| {
                !building.complete || building.health < building_max_health(content, building)
            })
            .filter(|building| {
                within_player_target_search_region(building_visual_grid(content, building), current)
            })
            .filter_map(|building| {
                let definition = building_def_for_archetype(content, &building.archetype)?;
                let approach = unoccupied_construction_approach(
                    world,
                    building.position,
                    rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                    current,
                    actor_id,
                    occupied_approaches,
                    approach_reservations,
                )?;
                Some((
                    grid_distance_squared(building_visual_grid(content, building), current),
                    building.id.clone(),
                    approach,
                ))
            })
            .collect();
        let construction = candidates
            .into_iter()
            .min_by_key(|(distance, id, _)| (*distance, id.clone()));
        if let Some((_, building, approach)) = construction {
            return (AgentGoal::Construct(building), approach);
        }
    }
    if !actor_resource_storage_has_room(config, content, simulation, actor) {
        let carrying_role_resource = resource_for_role(content, &actor.role)
            .is_some_and(|kind| actor.inventory.get(&kind).copied().unwrap_or_default() > 0);
        if carrying_role_resource && actor_remaining_carry_capacity(content, simulation, actor) == 0
        {
            return (
                AgentGoal::WaitForStorage,
                town_hall_wait_target(content, simulation, world, config, current),
            );
        }
        return (
            AgentGoal::Wander,
            deterministic_wander_target(world, actor_id, current),
        );
    }
    if actor_remaining_carry_capacity(content, simulation, actor) == 0 {
        let destination = station.map_or_else(
            || {
                nearest_walkable(
                    world,
                    restored_town_hall_position(content, simulation, config),
                )
                .unwrap_or(current)
            },
            |station| {
                unoccupied_station_deposit_approach(
                    content,
                    simulation,
                    world,
                    station,
                    current,
                    actor_id,
                    occupied_approaches,
                    approach_reservations,
                )
                .unwrap_or(current)
            },
        );
        return (AgentGoal::Deposit, destination);
    }
    let Some(resource_kind) = resource_for_role(content, &actor.role) else {
        return (
            AgentGoal::Wander,
            deterministic_wander_target(world, actor_id, current),
        );
    };
    let Some(station) = station else {
        return (
            AgentGoal::Wander,
            deterministic_wander_target(world, actor_id, current),
        );
    };
    if content.roles.get(&actor.role).is_some_and(|role| {
        role_accepts_target(
            role,
            &StableId::new("target:farm").expect("static target ID"),
        )
    }) {
        let farm_kind = StableId::new("target:farm").expect("static target ID");
        let cached_farms = cached_station_targets(station_targets, station, &farm_kind)
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut farms: Vec<_> = simulation
            .buildings
            .values()
            .filter(|building| is_farm_resource_building(content, building))
            .filter(|building| cached_farms.contains(&building.id))
            .filter(|building| reservation_available(reservations, actor_id, &building.id))
            .filter_map(|building| {
                let definition = building_def_for_archetype(content, &building.archetype)?;
                let approach = unoccupied_building_approach(
                    world,
                    building.position,
                    rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                    current,
                    actor_id,
                    occupied_approaches,
                    approach_reservations,
                )?;
                Some((
                    grid_distance_squared(
                        building_visual_grid(content, building),
                        station.position,
                    ),
                    definition.targeting.as_ref().map_or_else(
                        || {
                            u128::from(grid_distance_squared(
                                building_visual_grid(content, building),
                                current,
                            ))
                        },
                        |scoring| {
                            target_score_milli(
                                building_visual_grid(content, building),
                                current,
                                target_assignments
                                    .get(&building.id)
                                    .copied()
                                    .unwrap_or_default(),
                                scoring,
                            )
                        },
                    ),
                    building.id.clone(),
                    approach,
                ))
            })
            .collect();
        farms.sort_by_key(|(station_distance, _, id, _)| (*station_distance, id.clone()));
        farms.sort_by_key(|(_, score, id, _)| (*score, id.clone()));
        if let Some((_, _, building, approach)) = farms.into_iter().next() {
            return (AgentGoal::HarvestFarm(building), approach);
        }
    }
    let role = content
        .roles
        .get(&actor.role)
        .expect("live actor role is validated");
    let resource_target_kind =
        role.target_kinds
            .iter()
            .find(|kind| {
                world.resources.iter().any(|resource| {
                    resource.kind == resource_kind && resource.target_kind == **kind
                })
            })
            .cloned();
    let cached_resources = resource_target_kind
        .as_ref()
        .map(|kind| {
            cached_station_targets(station_targets, station, kind)
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let mut resources: Vec<_> = world
        .resources
        .iter()
        .filter(|resource| {
            resource.kind == resource_kind
                && resource.amount > 0
                && actor_accepts_resource(content, actor, resource)
                && cached_resources.contains(&resource.id)
        })
        .collect();
    resources.sort_by_key(|resource| {
        (
            grid_distance_squared(resource.position, current),
            resource.position.z,
            resource.position.x,
            resource.id.clone(),
        )
    });
    if let Some(goal) = resources
        .into_iter()
        .filter(|resource| reservation_available(reservations, actor_id, &resource.id))
        .find_map(|resource| {
            resource_approach(world, resource, current)
                .map(|approach| (AgentGoal::Gather(resource.id.clone()), approach))
        })
    {
        return goal;
    }
    if actor
        .inventory
        .get(&resource_kind)
        .copied()
        .unwrap_or_default()
        > 0
    {
        let destination = unoccupied_station_deposit_approach(
            content,
            simulation,
            world,
            station,
            current,
            actor_id,
            occupied_approaches,
            approach_reservations,
        )
        .unwrap_or(current);
        return (AgentGoal::Deposit, destination);
    }
    (
        AgentGoal::Wander,
        deterministic_wander_target(world, actor_id, current),
    )
}

pub(crate) fn regeneration_role_interval_seconds(role: &StableId, level: u16) -> Option<f64> {
    let progress = f64::from(level.saturating_sub(1).min(99)) / 99.0;
    match role.as_str() {
        "role:forester" => Some((300.0 + (20.0 - 300.0) * progress) * FORESTER_INTERVAL_MULTIPLIER),
        "role:tender" => {
            Some((1_800.0 + (360.0 - 1_800.0) * progress) * TENDER_INTERVAL_MULTIPLIER)
        }
        _ => None,
    }
}

pub(crate) fn community_adjusted_regeneration_interval_seconds(
    role: &StableId,
    level: u16,
    event: Option<CommunityEvent>,
) -> Option<f64> {
    regeneration_role_interval_seconds(role, level).map(|seconds| {
        seconds
            / f64::from(event.map_or(1.0, |event| {
                community_event_role_rate_multiplier(event, role)
            }))
    })
}

pub(crate) fn prospector_discovery_denominator(level: u16) -> u64 {
    let progress = u64::from(level.saturating_sub(1).min(99));
    4_000_u64.saturating_sub(3_600 * progress / 99).max(400)
}

pub(crate) fn regeneration_role_level(actor: &ActorState) -> u16 {
    actor
        .role_progression
        .get(&actor.role)
        .map_or(1, |progress| progress.level.max(1))
}

pub(crate) fn regeneration_hut<'a>(
    content: &'a ContentCatalog,
    simulation: &'a WorldSimulation,
    role: &StableId,
    from: GridPos,
) -> Option<&'a BuildingState> {
    simulation
        .buildings
        .values()
        .filter(|building| building.complete)
        .filter(|building| {
            building_def_for_archetype(content, &building.archetype).is_some_and(|definition| {
                definition.role_slots.iter().any(|slot| slot.role == *role)
            })
        })
        .min_by_key(|building| {
            (
                grid_distance_squared(building.position, from),
                building.id.clone(),
            )
        })
}

pub(crate) fn offset_grid(
    position: GridPos,
    x: i32,
    z: i32,
    world: &GeneratedWorld,
) -> Option<GridPos> {
    let x = i32::from(position.x).checked_add(x)?;
    let z = i32::from(position.z).checked_add(z)?;
    (x >= 0
        && z >= 0
        && x < i32::from(world.navigation.width())
        && z < i32::from(world.navigation.height()))
    .then(|| GridPos {
        x: u16::try_from(x).expect("checked world x fits u16"),
        z: u16::try_from(z).expect("checked world z fits u16"),
    })
}

pub(crate) fn active_resource_at(world: &GeneratedWorld, position: GridPos) -> bool {
    world
        .resources
        .iter()
        .any(|resource| resource.amount > 0 && resource.position == position)
}

pub(crate) fn nearby_active_resource_count(
    world: &GeneratedWorld,
    position: GridPos,
    target_kind: &str,
    radius: u16,
) -> u32 {
    let radius_squared = u64::from(radius) * u64::from(radius);
    u32::try_from(
        world
            .resources
            .iter()
            .filter(|resource| {
                resource.amount > 0
                    && resource.target_kind.as_str() == target_kind
                    && grid_distance_squared(resource.position, position) <= radius_squared
            })
            .count(),
    )
    .unwrap_or(u32::MAX)
}

pub(crate) fn accelerate_forester_regeneration_cooldowns(
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    runtime: &mut RegenerationRoleRuntime,
    delta_seconds: f64,
) {
    if delta_seconds <= 0.0 {
        return;
    }
    for actor in simulation
        .actors
        .values()
        .filter(|actor| actor.alive && actor.role.as_str() == "role:forester")
    {
        let Some(worker) = runtime.workers.get_mut(&actor.id) else {
            continue;
        };
        if !worker.initialized || worker.next_ready_seconds <= runtime.elapsed_seconds {
            continue;
        }
        let nearby_trees = nearby_active_resource_count(
            world,
            actor.position,
            "target:tree",
            FORESTER_TREE_TIMER_RADIUS_CELLS,
        );
        let bonus_seconds =
            delta_seconds * f64::from(nearby_trees) * FORESTER_TREE_TIMER_BONUS_PER_TREE;
        worker.next_ready_seconds =
            (worker.next_ready_seconds - bonus_seconds).max(runtime.elapsed_seconds);
    }
}

pub(crate) fn local_resource_saturation_roll(
    runtime: &mut RegenerationRoleRuntime,
    world: &GeneratedWorld,
    actor: &StableId,
    position: GridPos,
    target_kind: &str,
    sequence: u32,
    base_denominator: u64,
    salt: u64,
) -> (u64, u64, u32) {
    let observed = nearby_active_resource_count(
        world,
        position,
        target_kind,
        LOCAL_RESOURCE_SATURATION_RADIUS_CELLS,
    );
    let target = StableId::new(target_kind).expect("static saturation target ID");
    let worker = runtime.workers.entry(actor.clone()).or_default();
    if worker.resource_saturation_target.as_ref() != Some(&target) {
        worker.resource_saturation_target = Some(target);
        worker.resource_saturation_debuff = 0;
    }
    let decayed = worker
        .resource_saturation_debuff
        .saturating_sub(LOCAL_RESOURCE_SATURATION_DECAY_PER_CHECK);
    worker.resource_saturation_debuff = decayed.max(observed);
    let denominator = base_denominator
        .max(1)
        .saturating_mul(u64::from(worker.resource_saturation_debuff).saturating_add(1));
    let roll = seagull_hash(
        world.seed ^ stable_id_hash(actor),
        u64::from(sequence),
        salt,
    );
    (roll, denominator, worker.resource_saturation_debuff)
}

pub(crate) fn squared_distance_from_region(
    position: GridPos,
    region: stream_town_domain::DirtyRegion,
) -> u64 {
    let dx = if position.x < region.min.x {
        region.min.x - position.x
    } else {
        position.x.saturating_sub(region.max.x)
    };
    let dz = if position.z < region.min.z {
        region.min.z - position.z
    } else {
        position.z.saturating_sub(region.max.z)
    };
    u64::from(dx) * u64::from(dx) + u64::from(dz) * u64::from(dz)
}

pub(crate) fn cell_is_clear_of_buildings(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    position: GridPos,
    minimum_distance: u16,
) -> bool {
    let minimum_squared = u64::from(minimum_distance) * u64::from(minimum_distance);
    simulation.buildings.values().all(|building| {
        let Some(definition) = building_def_for_archetype(content, &building.archetype) else {
            return true;
        };
        let footprint = rotated_footprint(definition.footprint, building.rotation_quarter_turns);
        building_region(building.position, footprint, world)
            .is_none_or(|region| squared_distance_from_region(position, region) >= minimum_squared)
    })
}

pub(crate) struct RegenerationSpatialIndex {
    pub(crate) resources: HashSet<GridPos>,
    pub(crate) trees: HashSet<GridPos>,
    pub(crate) actors: HashSet<GridPos>,
    pub(crate) buildings: HashSet<GridPos>,
}

impl RegenerationSpatialIndex {
    pub(crate) fn new(
        content: &ContentCatalog,
        world: &GeneratedWorld,
        simulation: &WorldSimulation,
    ) -> Self {
        Self {
            resources: world
                .resources
                .iter()
                .filter(|resource| resource.amount > 0)
                .map(|resource| resource.position)
                .collect(),
            trees: world
                .resources
                .iter()
                .filter(|resource| {
                    resource.amount > 0 && resource.target_kind.as_str() == "target:tree"
                })
                .map(|resource| resource.position)
                .collect(),
            actors: simulation
                .actors
                .values()
                .filter(|actor| actor.alive)
                .map(|actor| actor.position)
                .collect(),
            buildings: simulation
                .buildings
                .values()
                .filter_map(|building| {
                    let definition = building_def_for_archetype(content, &building.archetype)?;
                    building_region(
                        building.position,
                        rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                        world,
                    )
                })
                .flat_map(|region| {
                    (region.min.z..=region.max.z).flat_map(move |z| {
                        (region.min.x..=region.max.x).map(move |x| GridPos { x, z })
                    })
                })
                .collect(),
        }
    }
}

pub(crate) fn offset_grid_unbounded(position: GridPos, x: i32, z: i32) -> Option<GridPos> {
    let x = i32::from(position.x).checked_add(x)?;
    let z = i32::from(position.z).checked_add(z)?;
    (x >= 0 && z >= 0 && x <= i32::from(u16::MAX) && z <= i32::from(u16::MAX)).then(|| GridPos {
        x: u16::try_from(x).expect("checked grid x fits u16"),
        z: u16::try_from(z).expect("checked grid z fits u16"),
    })
}

pub(crate) fn nearest_resource_distance_capped(
    resources: &HashSet<GridPos>,
    position: GridPos,
    maximum_squared: u64,
) -> u64 {
    let radius = i32::try_from(maximum_squared.isqrt()).unwrap_or(i32::MAX);
    let mut nearest = maximum_squared;
    for z in -radius..=radius {
        for x in -radius..=radius {
            let distance = u64::try_from(i64::from(x) * i64::from(x) + i64::from(z) * i64::from(z))
                .unwrap_or(u64::MAX);
            if distance >= nearest {
                continue;
            }
            if offset_grid_unbounded(position, x, z).is_some_and(|cell| resources.contains(&cell)) {
                nearest = distance;
            }
        }
    }
    nearest
}

pub(crate) fn valid_regeneration_cell_indexed(
    world: &GeneratedWorld,
    spatial: &RegenerationSpatialIndex,
    position: GridPos,
) -> bool {
    world.navigation.is_walkable(position)
        && !spatial.resources.contains(&position)
        && !spatial.actors.contains(&position)
        && !spatial.buildings.contains(&position)
}

pub(crate) fn valid_regeneration_cell(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    position: GridPos,
) -> bool {
    valid_regeneration_cell_indexed(
        world,
        &RegenerationSpatialIndex::new(content, world, simulation),
        position,
    )
}

#[cfg(test)]
pub(crate) fn planting_approach(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    plant: GridPos,
    from: GridPos,
) -> Option<GridPos> {
    let actor = StableId::new("system:unreserved-planter").expect("static stable ID");
    planting_approach_with_reservations(
        content,
        simulation,
        world,
        plant,
        from,
        &actor,
        &BTreeMap::new(),
    )
}

pub(crate) fn planting_approach_with_reservations(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    plant: GridPos,
    from: GridPos,
    actor: &StableId,
    approach_reservations: &BTreeMap<GridPos, StableId>,
) -> Option<GridPos> {
    let mut candidates = (-1..=1)
        .flat_map(|z| (-1..=1).map(move |x| (x, z)))
        .filter(|(x, z)| *x != 0 || *z != 0)
        .filter_map(|(x, z)| offset_grid(plant, x, z, world))
        .filter(|candidate| world.navigation.is_walkable(*candidate))
        .filter(|candidate| {
            approach_reservation_available(approach_reservations, actor, *candidate)
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|candidate| {
        (
            grid_distance_squared(*candidate, from),
            candidate.z,
            candidate.x,
        )
    });
    candidates.into_iter().find(|candidate| {
        try_agent_path(
            &world.navigation,
            content,
            simulation,
            &ActorKind::Player,
            from,
            *candidate,
        )
        .is_some()
    })
}

pub(crate) fn stable_id_hash(value: &StableId) -> u64 {
    value
        .as_str()
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
        })
}

pub(crate) fn initial_enemy_repath_delay(config: &GameConfig, actor: &StableId) -> f32 {
    let interval = config.gameplay.repath_interval_seconds.max(0.1);
    let phase =
        f32::from(u16::try_from(stable_id_hash(actor) % 1_000).unwrap_or_default()) / 1_000.0;
    interval * phase
}

pub(crate) fn regenerated_resource_offset(id: &StableId, position: GridPos) -> [i16; 2] {
    let hash = seagull_hash(
        stable_id_hash(id),
        u64::from(position.x) | (u64::from(position.z) << 16),
        0x5245_4752_4F57,
    );
    let component =
        |bits: u64| i16::try_from(bits % 501).expect("hashed resource offset fits i16") - 250;
    [component(hash), component(hash >> 32)]
}

pub(crate) fn forester_uses_random_fallback(
    world_seed: u64,
    actor: &StableId,
    sequence: u32,
) -> bool {
    seagull_hash(
        world_seed ^ stable_id_hash(actor),
        u64::from(sequence),
        0x464F_5245_5354_524E,
    )
    .is_multiple_of(FORESTER_RANDOM_FALLBACK_DENOMINATOR)
}

pub(crate) fn forester_direction_from_nursery(nursery: GridPos, position: GridPos) -> (i8, i8) {
    (
        (i32::from(position.x) - i32::from(nursery.x)).signum() as i8,
        (i32::from(position.z) - i32::from(nursery.z)).signum() as i8,
    )
}

pub(crate) fn grid_distance_cells(left: GridPos, right: GridPos) -> f64 {
    f64::from(left.x.abs_diff(right.x)).hypot(f64::from(left.z.abs_diff(right.z)))
}

pub(crate) fn forester_nursery_distance_utility(nursery: GridPos, position: GridPos) -> f64 {
    let distance = grid_distance_cells(nursery, position);
    if distance <= FORESTER_PREFERRED_NURSERY_DISTANCE_CELLS {
        distance / FORESTER_PREFERRED_NURSERY_DISTANCE_CELLS
    } else {
        (1.0 - (distance - FORESTER_PREFERRED_NURSERY_DISTANCE_CELLS)
            / (f64::from(FORESTER_UTILITY_RADIUS_CELLS)
                - FORESTER_PREFERRED_NURSERY_DISTANCE_CELLS))
            .max(0.0)
    }
}

pub(crate) fn forester_worker_distance_utility(from: GridPos, position: GridPos) -> f64 {
    let distance = grid_distance_cells(from, position);
    (1.0 - distance / (f64::from(FORESTER_UTILITY_RADIUS_CELLS) * 2.0)).max(0.0)
}

pub(crate) fn forester_candidate_utility(
    spatial: &RegenerationSpatialIndex,
    runtime: &RegenerationRoleRuntime,
    reserved_planting_cells: &BTreeSet<GridPos>,
    nursery: GridPos,
    nearest_nursery: GridPos,
    candidate: GridPos,
) -> f64 {
    let maximum_tree_distance_squared =
        u64::try_from(FORESTER_UTILITY_RADIUS_CELLS.pow(2)).expect("positive utility radius");
    let nearest_tree =
        nearest_resource_distance_capped(&spatial.trees, candidate, maximum_tree_distance_squared);
    let tree_proximity = if nearest_tree >= maximum_tree_distance_squared {
        0.0
    } else {
        let nearest_tree =
            u32::try_from(nearest_tree).expect("the capped Forester tree-search distance fits u32");
        1.0 / (1.0 + f64::from(nearest_tree).sqrt())
    };
    let fallen_proximity = runtime
        .recently_fallen_trees
        .iter()
        .rev()
        .take(64)
        .enumerate()
        .map(|(age_rank, fallen)| {
            let recency = 1.0 - f64::from(u32::try_from(age_rank).unwrap_or(64)) / 64.0;
            recency / (1.0 + grid_distance_cells(*fallen, candidate))
        })
        .fold(0.0_f64, f64::max);
    let direction = forester_direction_from_nursery(nursery, candidate);
    let placed_direction_penalty = runtime
        .recent_tree_plantings
        .iter()
        .filter(|planting| {
            planting.nursery == nursery
                && forester_direction_from_nursery(nursery, planting.position) == direction
        })
        .map(|planting| {
            let age = (runtime.elapsed_seconds - planting.planted_at_seconds).max(0.0);
            (1.0 - age / FORESTER_DIRECTION_MEMORY_SECONDS).clamp(0.0, 1.0)
        })
        .sum::<f64>()
        .min(2.0);
    // Treat another Forester's current destination like a just-planted tree.
    // This spreads simultaneous workers before either has had time to finish.
    let reserved_direction_penalty = reserved_planting_cells
        .iter()
        .filter(|position| forester_direction_from_nursery(nursery, **position) == direction)
        .count();
    let reserved_direction_penalty = f64::from(
        u32::try_from(reserved_direction_penalty).expect("reserved Forester destinations fit u32"),
    );

    forester_nursery_distance_utility(nearest_nursery, candidate) * 1_000.0
        + tree_proximity * 600.0
        + fallen_proximity * 1_200.0
        - placed_direction_penalty * 1_400.0
        - reserved_direction_penalty * 2_000.0
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn forester_selection_utility(
    spatial: &RegenerationSpatialIndex,
    runtime: &RegenerationRoleRuntime,
    reserved_planting_cells: &BTreeSet<GridPos>,
    nursery: GridPos,
    nearest_nursery: GridPos,
    from: GridPos,
    world_seed: u64,
    actor: &StableId,
    sequence: u32,
    candidate: GridPos,
) -> f64 {
    let worker_proximity = forester_worker_distance_utility(from, candidate);
    let coordinate =
        u64::from(candidate.x) | (u64::from(candidate.z) << 16) | (u64::from(sequence) << 32);
    let random = f64::from(
        u32::try_from(
            seagull_hash(
                world_seed ^ stable_id_hash(actor),
                coordinate,
                0x464F_5245_5354_5554,
            ) >> 32,
        )
        .expect("upper hash half fits u32"),
    ) / f64::from(u32::MAX);

    forester_candidate_utility(
        spatial,
        runtime,
        reserved_planting_cells,
        nursery,
        nearest_nursery,
        candidate,
    ) + worker_proximity * 800.0
        + random * 1_000.0
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn forester_planting_cell(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    runtime: &RegenerationRoleRuntime,
    actor: &StableId,
    sequence: u32,
    reserved_planting_cells: &BTreeSet<GridPos>,
    approach_reservations: &BTreeMap<GridPos, StableId>,
    hut: GridPos,
    from: GridPos,
) -> Option<(GridPos, GridPos)> {
    let spatial = RegenerationSpatialIndex::new(content, world, simulation);
    if forester_uses_random_fallback(world.seed, actor, sequence)
        && let Some(result) = forester_random_planting_cell(
            content,
            simulation,
            world,
            &spatial,
            actor,
            sequence,
            reserved_planting_cells,
            approach_reservations,
            hut,
            from,
        )
    {
        return Some(result);
    }
    let radius_squared =
        u64::try_from(FORESTER_UTILITY_RADIUS_CELLS.pow(2)).expect("positive utility radius");
    let nursery_role = StableId::new("role:forester").expect("static Forester role ID");
    let nursery_positions = simulation
        .buildings
        .values()
        .filter(|building| building.complete)
        .filter(|building| {
            building_def_for_archetype(content, &building.archetype).is_some_and(|definition| {
                definition
                    .role_slots
                    .iter()
                    .any(|slot| slot.role == nursery_role)
            })
        })
        .map(|building| building.position)
        .collect::<Vec<_>>();
    let mut candidates = (-FORESTER_UTILITY_RADIUS_CELLS..=FORESTER_UTILITY_RADIUS_CELLS)
        .flat_map(|z| {
            (-FORESTER_UTILITY_RADIUS_CELLS..=FORESTER_UTILITY_RADIUS_CELLS).map(move |x| (x, z))
        })
        .filter_map(|(x, z)| offset_grid(hut, x, z, world))
        .filter(|candidate| grid_distance_squared(hut, *candidate) <= radius_squared)
        .filter(|candidate| valid_regeneration_cell_indexed(world, &spatial, *candidate))
        .filter(|candidate| !reserved_planting_cells.contains(candidate))
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        let nearest_nursery = |candidate: GridPos| {
            nursery_positions
                .iter()
                .copied()
                .min_by_key(|position| grid_distance_squared(*position, candidate))
                .unwrap_or(hut)
        };
        let left_utility = forester_selection_utility(
            &spatial,
            runtime,
            reserved_planting_cells,
            hut,
            nearest_nursery(*left),
            from,
            world.seed,
            actor,
            sequence,
            *left,
        );
        let right_utility = forester_selection_utility(
            &spatial,
            runtime,
            reserved_planting_cells,
            hut,
            nearest_nursery(*right),
            from,
            world.seed,
            actor,
            sequence,
            *right,
        );
        right_utility
            .total_cmp(&left_utility)
            .then_with(|| (left.z, left.x).cmp(&(right.z, right.x)))
    });
    if let Some(result) = candidates.into_iter().find_map(|candidate| {
        planting_approach_with_reservations(
            content,
            simulation,
            world,
            candidate,
            from,
            actor,
            approach_reservations,
        )
        .map(|approach| (candidate, approach))
    }) {
        return Some(result);
    }

    forester_random_planting_cell(
        content,
        simulation,
        world,
        &spatial,
        actor,
        sequence,
        reserved_planting_cells,
        approach_reservations,
        hut,
        from,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn forester_random_planting_cell(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    spatial: &RegenerationSpatialIndex,
    actor: &StableId,
    sequence: u32,
    reserved_planting_cells: &BTreeSet<GridPos>,
    approach_reservations: &BTreeMap<GridPos, StableId>,
    hut: GridPos,
    from: GridPos,
) -> Option<(GridPos, GridPos)> {
    let radius = 20_i32;
    let mut fallback = (-radius..=radius)
        .flat_map(|z| (-radius..=radius).map(move |x| (x, z)))
        .filter_map(|(x, z)| offset_grid(hut, x, z, world))
        .filter(|candidate| valid_regeneration_cell_indexed(world, spatial, *candidate))
        .filter(|candidate| !reserved_planting_cells.contains(candidate))
        .filter(|candidate| cell_is_clear_of_buildings(content, simulation, world, *candidate, 3))
        .collect::<Vec<_>>();
    fallback.sort_by_key(|candidate| {
        seagull_hash(
            world.seed ^ stable_id_hash(actor),
            u64::from(candidate.x) | (u64::from(candidate.z) << 16) | (u64::from(sequence) << 32),
            0x4641_4C4C_4241_434B,
        )
    });
    fallback.into_iter().find_map(|candidate| {
        planting_approach_with_reservations(
            content,
            simulation,
            world,
            candidate,
            from,
            actor,
            approach_reservations,
        )
        .map(|approach| (candidate, approach))
    })
}

pub(crate) fn tender_planting_cell(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    actor: &StableId,
    sequence: u32,
    reserved: &BTreeSet<GridPos>,
    approach_reservations: &BTreeMap<GridPos, StableId>,
    hut: GridPos,
    from: GridPos,
) -> Option<(GridPos, GridPos)> {
    let spatial = RegenerationSpatialIndex::new(content, world, simulation);
    let radius = 24_i32;
    let mut fields = (-radius..=radius)
        .flat_map(|z| (-radius..=radius).map(move |x| (x, z)))
        .filter_map(|(x, z)| offset_grid(hut, x, z, world))
        .filter(|candidate| valid_regeneration_cell_indexed(world, &spatial, *candidate))
        .filter(|candidate| cell_is_clear_of_buildings(content, simulation, world, *candidate, 10))
        .filter(|candidate| {
            let nearest_tree = nearest_resource_distance_capped(&spatial.trees, *candidate, 37);
            (9..=36).contains(&nearest_tree)
        })
        .filter(|candidate| !reserved.contains(candidate))
        .collect::<Vec<_>>();
    fields.sort_by_cached_key(|candidate| {
        let coordinate = u64::from(candidate.x) | (u64::from(candidate.z) << 16);
        seagull_hash(
            world.seed ^ stable_id_hash(actor),
            coordinate ^ (u64::from(sequence) << 32),
            0x5445_4E44_4552,
        )
    });
    fields.into_iter().find_map(|candidate| {
        planting_approach_with_reservations(
            content,
            simulation,
            world,
            candidate,
            from,
            actor,
            approach_reservations,
        )
        .map(|approach| (candidate, approach))
    })
}

pub(crate) fn prospector_spiral_cells(world: &GeneratedWorld, hut: GridPos) -> Vec<GridPos> {
    let mut cells = Vec::new();
    for radius in 5_i32..=20 {
        for z in 0..=radius {
            if let Some(cell) = offset_grid(hut, radius, z, world) {
                cells.push(cell);
            }
        }
        for x in (-radius..radius).rev() {
            if let Some(cell) = offset_grid(hut, x, radius, world) {
                cells.push(cell);
            }
        }
        for z in (-radius..radius).rev() {
            if let Some(cell) = offset_grid(hut, -radius, z, world) {
                cells.push(cell);
            }
        }
        for x in (-radius + 1)..=radius {
            if let Some(cell) = offset_grid(hut, x, -radius, world) {
                cells.push(cell);
            }
        }
        for z in (-radius + 1)..0 {
            if let Some(cell) = offset_grid(hut, radius, z, world) {
                cells.push(cell);
            }
        }
    }
    cells.retain(|cell| world.navigation.is_walkable(*cell));
    cells
}

#[cfg(test)]
pub(crate) fn regeneration_agent_goal(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    runtime: &mut RegenerationRoleRuntime,
    actor_id: &StableId,
    current: GridPos,
) -> Option<(AgentGoal, GridPos)> {
    regeneration_agent_goal_with_reservations(
        content,
        simulation,
        world,
        runtime,
        actor_id,
        current,
        &BTreeMap::new(),
    )
}

pub(crate) fn regeneration_agent_goal_with_reservations(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    runtime: &mut RegenerationRoleRuntime,
    actor_id: &StableId,
    current: GridPos,
    approach_reservations: &BTreeMap<GridPos, StableId>,
) -> Option<(AgentGoal, GridPos)> {
    let actor = simulation.actors.get(actor_id)?;
    if !matches!(
        actor.role.as_str(),
        "role:forester" | "role:prospector" | "role:tender"
    ) {
        return None;
    }
    let hut = regeneration_hut(content, simulation, &actor.role, current)?;
    let hut_id = hut.id.clone();
    let hut_position = hut.position;
    let hut_footprint = building_def_for_archetype(content, &hut.archetype)
        .map(|definition| rotated_footprint(definition.footprint, hut.rotation_quarter_turns))?;
    let level = regeneration_role_level(actor);
    if let Some(interval) = community_adjusted_regeneration_interval_seconds(
        &actor.role,
        level,
        simulation.active_community_event,
    ) {
        let worker = runtime.workers.entry(actor_id.clone()).or_default();
        if !worker.initialized {
            worker.initialized = true;
            worker.next_ready_seconds = runtime.elapsed_seconds + interval;
            worker.station_visit_required = true;
            return None;
        }
        if runtime.elapsed_seconds < worker.next_ready_seconds {
            return None;
        }
        // A mature town may have no valid planting cells. Bound the next
        // expensive spatial search even when target selection or path planning
        // fails; successful work replaces this with the full authored interval.
        worker.next_ready_seconds = runtime.elapsed_seconds + REGENERATION_TARGET_RETRY_SECONDS;
    }
    if matches!(actor.role.as_str(), "role:forester" | "role:tender") {
        let worker = runtime.workers.entry(actor_id.clone()).or_default();
        if worker.last_station.as_ref() != Some(&hut_id) {
            worker.station_visit_required = true;
        }
        if worker.station_visit_required {
            return building_approaches(world, hut_position, hut_footprint, current)
                .into_iter()
                .find(|approach| {
                    approach_reservation_available(approach_reservations, actor_id, *approach)
                })
                .map(|approach| {
                    (
                        AgentGoal::VisitRegenerationStation(hut_id.clone()),
                        approach,
                    )
                });
        }
    }
    let reserved_planting_cells = runtime
        .workers
        .iter()
        .filter(|(worker_id, _)| *worker_id != actor_id)
        .filter_map(|(_, worker)| worker.planting_target)
        .collect::<BTreeSet<_>>();
    match actor.role.as_str() {
        "role:forester" => {
            let sequence = {
                let worker = runtime.workers.entry(actor_id.clone()).or_default();
                let sequence = worker.planting_sequence;
                worker.planting_sequence = worker.planting_sequence.wrapping_add(1);
                sequence
            };
            let selected = forester_planting_cell(
                content,
                simulation,
                world,
                runtime,
                actor_id,
                sequence,
                &reserved_planting_cells,
                approach_reservations,
                hut_position,
                current,
            );
            if let Some((plant, approach)) = selected {
                runtime
                    .workers
                    .entry(actor_id.clone())
                    .or_default()
                    .planting_target = Some(plant);
                Some((AgentGoal::PlantTree(plant), approach))
            } else {
                None
            }
        }
        "role:tender" => {
            let sequence = {
                let worker = runtime.workers.entry(actor_id.clone()).or_default();
                let sequence = worker.planting_sequence;
                worker.planting_sequence = worker.planting_sequence.wrapping_add(1);
                sequence
            };
            let (roll, denominator, _) = local_resource_saturation_roll(
                runtime,
                world,
                actor_id,
                current,
                "target:bush",
                sequence,
                1,
                0x5445_4E44_4552_4348,
            );
            if !roll.is_multiple_of(denominator) {
                if let Some(interval) = community_adjusted_regeneration_interval_seconds(
                    &actor.role,
                    level,
                    simulation.active_community_event,
                ) {
                    runtime
                        .workers
                        .entry(actor_id.clone())
                        .or_default()
                        .next_ready_seconds = runtime.elapsed_seconds + interval;
                    runtime
                        .workers
                        .entry(actor_id.clone())
                        .or_default()
                        .station_visit_required = true;
                }
                return None;
            }
            let selected = tender_planting_cell(
                content,
                simulation,
                world,
                actor_id,
                sequence,
                &reserved_planting_cells,
                approach_reservations,
                hut_position,
                current,
            );
            if let Some((plant, approach)) = selected {
                let worker = runtime.workers.entry(actor_id.clone()).or_default();
                worker.planting_target = Some(plant);
                Some((AgentGoal::PlantBush(plant), approach))
            } else {
                None
            }
        }
        "role:prospector" => {
            let cells = prospector_spiral_cells(world, hut_position);
            if cells.is_empty() {
                return None;
            }
            let worker = runtime.workers.entry(actor_id.clone()).or_default();
            worker.initialized = true;
            let sequence = worker.prospector_step;
            worker.prospector_step = worker.prospector_step.wrapping_add(1);
            let cell = cells[usize::try_from(sequence).unwrap_or_default() % cells.len()];
            Some((AgentGoal::Prospect { cell, sequence }, cell))
        }
        _ => None,
    }
}

pub(crate) fn resource_amount_for_target(content: &ContentCatalog, target: &str) -> u32 {
    content
        .resource_generation
        .iter()
        .find(|layer| layer.target_kind.as_str() == target)
        .map_or(1, |layer| layer.amount)
}

pub(crate) fn spawn_regenerated_resource(
    runtime: &mut RegenerationRoleRuntime,
    world: &mut GeneratedWorld,
    content: &ContentCatalog,
    kind: &str,
    target: &str,
    position: GridPos,
) -> bool {
    if active_resource_at(world, position) || !world.navigation.is_walkable(position) {
        return false;
    }
    let id = loop {
        let serial = runtime.next_resource_serial;
        runtime.next_resource_serial = runtime.next_resource_serial.wrapping_add(1);
        let candidate = StableId::new(format!(
            "resource:regrown_{}_{}_{}_{}_{serial:08x}",
            target.trim_start_matches("target:"),
            world.seed,
            position.x,
            position.z,
        ))
        .expect("regrown resource ID is valid");
        if !world
            .resources
            .iter()
            .any(|resource| resource.id == candidate)
        {
            break candidate;
        }
    };
    let offset_milli_cells = regenerated_resource_offset(&id, position);
    world.resources.push(stream_town_domain::GeneratedResource {
        id,
        kind: StableId::new(kind).expect("static regenerated resource kind"),
        target_kind: StableId::new(target).expect("static regenerated target kind"),
        position,
        offset_milli_cells,
        generation_occupancy: [0, 0],
        amount: resource_amount_for_target(content, target),
    });
    let _ = world.navigation.set_blocked(
        stream_town_domain::DirtyRegion {
            min: position,
            max: position,
        },
        true,
    );
    true
}

pub(crate) fn restored_regenerated_resource(
    id: &StableId,
    amount: u32,
) -> Option<stream_town_domain::GeneratedResource> {
    let mut parts = id.as_str().strip_prefix("resource:regrown_")?.split('_');
    let target_name = parts.next()?;
    let _seed = parts.next()?.parse::<u64>().ok()?;
    let x = parts.next()?.parse::<u16>().ok()?;
    let z = parts.next()?.parse::<u16>().ok()?;
    let _serial = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    let (kind, target_kind) = match target_name {
        "tree" => ("resource:wood", "target:tree"),
        "ore" => ("resource:ore", "target:ore"),
        "bush" => ("resource:food", "target:bush"),
        _ => return None,
    };
    let position = GridPos { x, z };
    Some(stream_town_domain::GeneratedResource {
        id: id.clone(),
        kind: StableId::new(kind).expect("static regenerated resource kind"),
        target_kind: StableId::new(target_kind).expect("static regenerated target kind"),
        position,
        offset_milli_cells: regenerated_resource_offset(id, position),
        generation_occupancy: [0, 0],
        amount,
    })
}

pub(crate) fn complete_regeneration_goal(
    simulation: &mut WorldSimulation,
    world: &mut GeneratedWorld,
    content: &ContentCatalog,
    runtime: &mut RegenerationRoleRuntime,
    actor_id: &StableId,
    goal: &AgentGoal,
) -> bool {
    let Some(actor) = simulation.actors.get(actor_id) else {
        return false;
    };
    let level = regeneration_role_level(actor);
    match goal {
        AgentGoal::PlantTree(position) => {
            let planted = valid_regeneration_cell(content, simulation, world, *position)
                && spawn_regenerated_resource(
                    runtime,
                    world,
                    content,
                    "resource:wood",
                    "target:tree",
                    *position,
                );
            if planted
                && let Some(nursery) =
                    regeneration_hut(content, simulation, &actor.role, actor.position)
            {
                runtime.recent_tree_plantings.push_back(RecentTreePlanting {
                    nursery: nursery.position,
                    position: *position,
                    planted_at_seconds: runtime.elapsed_seconds,
                });
                while runtime.recent_tree_plantings.len() > 128 {
                    runtime.recent_tree_plantings.pop_front();
                }
            }
            if let Some(interval) = community_adjusted_regeneration_interval_seconds(
                &actor.role,
                level,
                simulation.active_community_event,
            ) {
                let worker = runtime.workers.entry(actor_id.clone()).or_default();
                worker.next_ready_seconds = runtime.elapsed_seconds + interval;
                worker.planting_target = None;
                worker.station_visit_required = true;
            }
            planted
        }
        AgentGoal::PlantBush(position) => {
            let planted = valid_regeneration_cell(content, simulation, world, *position)
                && spawn_regenerated_resource(
                    runtime,
                    world,
                    content,
                    "resource:food",
                    "target:bush",
                    *position,
                );
            if let Some(interval) = community_adjusted_regeneration_interval_seconds(
                &actor.role,
                level,
                simulation.active_community_event,
            ) {
                let worker = runtime.workers.entry(actor_id.clone()).or_default();
                worker.next_ready_seconds = runtime.elapsed_seconds + interval;
                worker.planting_target = None;
                worker.station_visit_required = true;
            }
            planted
        }
        AgentGoal::Prospect { cell, sequence } => {
            let (roll, denominator, _) = local_resource_saturation_roll(
                runtime,
                world,
                actor_id,
                actor.position,
                "target:ore",
                *sequence,
                prospector_discovery_denominator(level),
                0x5052_4F53_5045_4354,
            );
            if !roll.is_multiple_of(denominator) {
                return true;
            }
            let count = 3 + usize::try_from((roll >> 32) % 3).expect("ore count fits usize");
            let offsets = [
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
                (1, 1),
                (-1, 1),
                (-1, -1),
                (1, -1),
                (2, 0),
                (0, 2),
                (-2, 0),
                (0, -2),
            ];
            let mut spawned = 0;
            for (x, z) in offsets {
                let Some(position) = offset_grid(*cell, x, z, world) else {
                    continue;
                };
                if valid_regeneration_cell(content, simulation, world, position)
                    && spawn_regenerated_resource(
                        runtime,
                        world,
                        content,
                        "resource:ore",
                        "target:ore",
                        position,
                    )
                {
                    spawned += 1;
                    if spawned >= count {
                        break;
                    }
                }
            }
            true
        }
        _ => false,
    }
}

pub(crate) fn complete_agent_goal_with_regeneration(
    simulation: &mut WorldSimulation,
    world: &mut GeneratedWorld,
    config: &GameConfig,
    content: &ContentCatalog,
    regeneration: &mut RegenerationRoleRuntime,
    actor_id: &StableId,
    goal: &AgentGoal,
    current: GridPos,
) -> Option<ActionPresentation> {
    if !simulation.actors.contains_key(actor_id) {
        return None;
    }
    let stats = simulation
        .actors
        .get(actor_id)
        .and_then(|actor| effective_role_stats(content, simulation, actor));
    let action_amount = simulation
        .actors
        .get(actor_id)
        .and_then(|actor| actor_archetype(content, actor))
        .and_then(|archetype| archetype.enemy.as_ref())
        .map_or_else(
            || stats.map_or(1, |stats| stats.action_amount),
            |enemy| enemy.action_amount,
        );
    let mut action_presentation = None;
    let action_succeeded = match goal {
        AgentGoal::VisitRegenerationStation(building_id) => {
            let actor = simulation.actors.get(actor_id)?;
            if !matches!(actor.role.as_str(), "role:forester" | "role:tender") {
                return None;
            }
            let station = simulation.buildings.get(building_id)?;
            if !station.complete
                || !building_def_for_archetype(content, &station.archetype).is_some_and(
                    |definition| {
                        definition
                            .role_slots
                            .iter()
                            .any(|slot| slot.role == actor.role)
                    },
                )
                || !is_current_building_approach(world, content, station, current)
            {
                return None;
            }
            let worker = regeneration.workers.entry(actor_id.clone()).or_default();
            worker.last_station = Some(building_id.clone());
            worker.station_visit_required = false;
            if let Some(actor) = simulation.actors.get_mut(actor_id) {
                actor.station = Some(building_id.clone());
            }
            true
        }
        AgentGoal::Gather(resource_id) => {
            let actor = simulation.actors.get(actor_id)?;
            if !actor_resource_storage_has_room(config, content, simulation, actor) {
                return None;
            }
            if !world
                .resources
                .iter()
                .find(|resource| resource.id == *resource_id && resource.amount > 0)
                .is_some_and(|resource| {
                    actor_accepts_resource(content, actor, resource)
                        && resource_approach(world, resource, current) == Some(current)
                })
            {
                return None;
            }
            let remaining_carry = actor_remaining_carry_capacity(content, simulation, actor);
            if remaining_carry == 0 {
                return None;
            }
            let gathering_pet = simulation.actors.get(actor_id).and_then(|actor| {
                actor.id.as_str().starts_with("twitch:").then_some(())?;
                match actor.role.as_str() {
                    "role:gatherer" => StableId::new("pet:giraffe").ok(),
                    "role:fisher" => StableId::new("pet:duck").ok(),
                    "role:logger" => StableId::new("pet:butterfly").ok(),
                    _ => None,
                }
            });
            let resource = world
                .resources
                .iter_mut()
                .find(|resource| resource.id == *resource_id && resource.amount > 0)?;
            let amount = resource.amount.min(action_amount);
            let stored_amount = amount.min(remaining_carry);
            resource.amount -= amount;
            let resource_kind = resource.kind.clone();
            let cleared_tree = (resource.amount == 0
                && resource.target_kind.as_str() == "target:tree")
                .then_some(resource.position);
            let cleared_position = (resource.amount == 0
                && resource.target_kind.as_str() != "target:fish")
                .then_some(resource.position);
            if let Err(error) = simulation.gather(actor_id, resource_kind, stored_amount) {
                warn!(actor = %actor_id, %error, "resource gather action failed");
                resource.amount = resource.amount.saturating_add(amount);
                false
            } else {
                if let Some(position) = cleared_position
                    && !resource_cell_has_active_generation_occupant(&world.resources, position)
                {
                    let _ = world.navigation.set_blocked(
                        stream_town_domain::DirtyRegion {
                            min: position,
                            max: position,
                        },
                        false,
                    );
                }
                if let Some(position) = cleared_tree {
                    regeneration.recently_fallen_trees.push_back(position);
                    while regeneration.recently_fallen_trees.len() > 128 {
                        regeneration.recently_fallen_trees.pop_front();
                    }
                }
                if amount > 0
                    && let Some(pet) = gathering_pet
                    && simulation
                        .try_unlock_gathering_pet(actor_id, pet.clone())
                        .unwrap_or(false)
                {
                    info!(actor = %actor_id, %pet, "unlocked gathering pet");
                }
                amount > 0
            }
        }
        AgentGoal::HarvestFarm(building_id) => {
            let actor = simulation.actors.get(actor_id)?;
            if !actor_resource_storage_has_room(config, content, simulation, actor)
                || !content.roles.get(&actor.role).is_some_and(|role| {
                    role.resource
                        .as_ref()
                        .is_some_and(|resource| resource.as_str() == "resource:food")
                        && role_accepts_target(
                            role,
                            &StableId::new("target:farm").expect("static target ID"),
                        )
                })
                || !simulation
                    .buildings
                    .get(building_id)
                    .is_some_and(|building| {
                        is_farm_resource_building(content, building)
                            && is_current_building_approach(world, content, building, current)
                    })
            {
                return None;
            }
            let remaining_carry = actor_remaining_carry_capacity(content, simulation, actor);
            if remaining_carry == 0 {
                return None;
            }
            let amount = action_amount.min(remaining_carry);
            let food = StableId::new("resource:food").expect("static resource ID");
            match simulation.gather(actor_id, food, amount) {
                Ok(()) => amount > 0,
                Err(error) => {
                    warn!(actor = %actor_id, building = %building_id, %error, "farm harvest action failed");
                    false
                }
            }
        }
        AgentGoal::Deposit => {
            let resource = simulation
                .actors
                .get(actor_id)
                .and_then(|actor| resource_for_role(content, &actor.role))?;
            let capacity = resource_storage_capacity(config, content, simulation, &resource);
            match simulation.deposit_resource_with_market(actor_id, &resource, capacity) {
                Ok((deposited, traded, gold)) => {
                    let gathered = deposited.saturating_add(traded);
                    if gathered > 0 {
                        let _ = simulation.record_objective_event(
                            &content.objectives,
                            &ObjectiveEvent::ResourceGathered {
                                resource: resource.clone(),
                                amount: gathered,
                            },
                        );
                    }
                    if traded > 0 {
                        let _ = simulation.record_objective_event(
                            &content.objectives,
                            &ObjectiveEvent::ResourceSold {
                                resource,
                                amount: traded,
                            },
                        );
                    }
                    if gold > 0 {
                        let _ = simulation.record_objective_event(
                            &content.objectives,
                            &ObjectiveEvent::ResourceGained {
                                resource: StableId::new("resource:gold").expect("static stable ID"),
                                amount: gold,
                            },
                        );
                    }
                    gathered > 0
                }
                Err(error) => {
                    warn!(actor = %actor_id, %error, "resource deposit action failed");
                    false
                }
            }
        }
        AgentGoal::Attack(target_id) => {
            let attacker = simulation.actors.get(actor_id)?;
            let target = simulation.actors.get(target_id)?;
            let visual = actor_combat_visual(&attacker.role);
            let target_position = target.position;
            if !attacker.alive
                || !target.alive
                || !within_actor_attack_range(content, simulation, attacker, target, current)
            {
                return None;
            }
            let damage = action_amount;
            if is_ranged_role(&attacker.role) {
                action_presentation = Some(ActionPresentation::Projectile(ProjectileSpawn {
                    source: ProjectileSource::Actor(actor_id.clone()),
                    target: target_id.clone(),
                    damage,
                    speed_cells_per_second: 12.0,
                    visual,
                }));
                damage > 0
            } else {
                match apply_combat_damage(
                    config,
                    simulation,
                    content,
                    Some(actor_id),
                    target_id,
                    damage,
                ) {
                    Ok(_) => {
                        if damage > 0 {
                            action_presentation = Some(ActionPresentation::Impact {
                                target: target_position,
                                visual,
                            });
                            true
                        } else {
                            false
                        }
                    }
                    Err(error) => {
                        warn!(actor = %actor_id, target = %target_id, %error, "combat action failed");
                        false
                    }
                }
            }
        }
        AgentGoal::AttackBuilding(building_id) => {
            let attacker = simulation.actors.get(actor_id)?;
            let building = simulation.buildings.get(building_id)?;
            if !enemy_can_attack_building(content, building) {
                return None;
            }
            if !attacker.alive
                || building.health <= 0
                || !within_enemy_building_attack_range(
                    content, simulation, attacker, building, current,
                )
            {
                return None;
            }
            let building_position = building_visual_grid(content, building);
            let building_origin = building.position;
            let building_rotation = building.rotation_quarter_turns;
            let building_definition =
                building_def_for_archetype(content, &building.archetype).cloned();
            match simulation.damage_building(building_id, action_amount) {
                Ok(remaining) if action_amount > 0 => {
                    if remaining == 0 {
                        if let Some(definition) = building_definition
                            .as_ref()
                            .filter(|definition| building_blocks_navigation(definition))
                            && let Some(region) = building_navigation_region(
                                building_origin,
                                definition,
                                building_rotation,
                                world,
                            )
                        {
                            let _ = world.navigation.set_blocked(region, false);
                        }
                        simulation.buildings.remove(building_id);
                        simulation.path_navigation_positions.remove(building_id);
                        simulation.building_night_light_colors.remove(building_id);
                        for actor in simulation.actors.values_mut() {
                            if actor.station.as_ref() == Some(building_id)
                                || actor.preferred_target.as_ref() == Some(building_id)
                            {
                                actor.station = None;
                                actor.preferred_target = None;
                            }
                        }
                        let reassigned =
                            reassign_roles_without_prerequisite_buildings(content, simulation);
                        if !reassigned.is_empty() {
                            info!(actors = ?reassigned, "reassigned citizens whose last role prerequisite was destroyed");
                        }
                        action_presentation = Some(ActionPresentation::BuildingDestroyed {
                            building: building_id.clone(),
                            target: building_position,
                        });
                    } else {
                        action_presentation = Some(ActionPresentation::BuildingWork {
                            target: building_position,
                            sparks: true,
                        });
                    }
                    true
                }
                Ok(_) => false,
                Err(error) => {
                    warn!(actor = %actor_id, building = %building_id, %error, "building attack failed");
                    false
                }
            }
        }
        AgentGoal::Heal(target_id) => {
            let healer = simulation.actors.get(actor_id)?;
            let target = simulation.actors.get(target_id)?;
            if !healer.alive
                || !target.alive
                || target.role.as_str() == "role:enemy"
                || target.health >= target.max_health
                || !within_actor_heal_hysteresis_range(content, simulation, healer, target, current)
            {
                return None;
            }
            match simulation.heal_actor(target_id, action_amount) {
                Ok(restored) => {
                    if restored > 0 {
                        action_presentation = Some(ActionPresentation::Healing {
                            source: actor_id.clone(),
                            target: target_id.clone(),
                        });
                        true
                    } else {
                        false
                    }
                }
                Err(error) => {
                    warn!(actor = %actor_id, target = %target_id, %error, "healing action failed");
                    false
                }
            }
        }
        AgentGoal::Construct(building_id) => {
            let builder = simulation.actors.get(actor_id)?;
            let building_before = simulation.buildings.get(building_id)?;
            let was_complete = building_before.complete;
            let needs_work = if was_complete {
                building_before.health < building_max_health(content, building_before)
            } else {
                true
            };
            let within_range =
                within_building_work_range(content, simulation, builder, building_before, current);
            if !needs_work || !within_range {
                debug!(
                    actor = %actor_id,
                    building = %building_id,
                    ?current,
                    needs_work,
                    within_range,
                    "builder action was rejected at completion"
                );
                return None;
            }
            let building_position = simulation
                .buildings
                .get(building_id)
                .map(|building| building_visual_grid(content, building));
            let archetype = simulation
                .buildings
                .get(building_id)
                .map(|building| building.archetype.clone());
            let max_health = simulation
                .buildings
                .get(building_id)
                .map_or(BUILDING_MAX_HEALTH, |building| {
                    building_max_health(content, building)
                });
            let max_health = u32::try_from(max_health).unwrap_or(u32::MAX);
            let result = if was_complete {
                simulation
                    .repair_building(building_id, action_amount, max_health)
                    .map(|restored| (false, restored > 0))
            } else {
                simulation
                    .work_on_building(building_id, action_amount, max_health)
                    .map(|complete| (complete, action_amount > 0))
            };
            match result {
                Ok((complete, succeeded)) => {
                    debug!(
                        actor = %actor_id,
                        building = %building_id,
                        action_amount,
                        was_complete,
                        complete,
                        succeeded,
                        "builder action applied"
                    );
                    if !was_complete
                        && complete
                        && let Some(building) = archetype.as_ref().and_then(|archetype| {
                            content.buildings.iter().find_map(|(id, definition)| {
                                (definition.archetype == *archetype).then_some(id.clone())
                            })
                        })
                    {
                        let _ = simulation.record_objective_event(
                            &content.objectives,
                            &ObjectiveEvent::BuildingBuilt(building),
                        );
                    }
                    if succeeded && let Some(target) = building_position {
                        action_presentation = Some(ActionPresentation::BuildingWork {
                            target,
                            sparks: true,
                        });
                    }
                    succeeded
                }
                Err(error) => {
                    warn!(actor = %actor_id, building = %building_id, %error, "construction action failed");
                    false
                }
            }
        }
        AgentGoal::PlantTree(_) | AgentGoal::Prospect { .. } | AgentGoal::PlantBush(_) => {
            complete_regeneration_goal(simulation, world, content, regeneration, actor_id, goal)
        }
        AgentGoal::WaitForStorage | AgentGoal::Wander => false,
    };
    let grants_role_experience = action_succeeded
        && match goal {
            AgentGoal::Gather(_)
            | AgentGoal::HarvestFarm(_)
            | AgentGoal::Attack(_)
            | AgentGoal::Heal(_)
            | AgentGoal::PlantTree(_)
            | AgentGoal::Prospect { .. }
            | AgentGoal::PlantBush(_) => true,
            // Unity's Builder action applies its final construction tick, then
            // returns false after the Construction target flag disappears.
            // Completed-building repair and deposits are not PlayerAction
            // successes and therefore do not award role experience.
            AgentGoal::Construct(building) => simulation
                .buildings
                .get(building)
                .is_some_and(|building| !building.complete),
            AgentGoal::AttackBuilding(_)
            | AgentGoal::Deposit
            | AgentGoal::VisitRegenerationStation(_)
            | AgentGoal::WaitForStorage
            | AgentGoal::Wander => false,
        };
    if grants_role_experience
        && let Some(stats) = stats
        && let Ok(levels_gained) = simulation.grant_role_experience_capped(
            actor_id,
            action_amount,
            stats.experience_multiplier_per_thousand,
            actor_role_level_cap(content, actor_id),
            content.progression.role_experience_curve_level_span,
            content.progression.role_experience_curve_maximum,
        )
        && levels_gained > 0
    {
        let max_health = simulation
            .actors
            .get(actor_id)
            .and_then(|actor| effective_role_stats(content, simulation, actor))
            .map_or(1, |stats| stats.max_health);
        if let Some(actor) = simulation.actors.get_mut(actor_id) {
            actor.max_health = i32::try_from(max_health).unwrap_or(i32::MAX);
            actor.health = actor.max_health;
        }
    }
    action_presentation
}

#[cfg(test)]
pub(crate) fn complete_agent_goal(
    simulation: &mut WorldSimulation,
    world: &mut GeneratedWorld,
    config: &GameConfig,
    content: &ContentCatalog,
    actor_id: &StableId,
    goal: &AgentGoal,
    current: GridPos,
) -> Option<ActionPresentation> {
    complete_agent_goal_with_regeneration(
        simulation,
        world,
        config,
        content,
        &mut RegenerationRoleRuntime::default(),
        actor_id,
        goal,
        current,
    )
}

pub(crate) fn resource_cell_has_active_generation_occupant(
    resources: &[stream_town_domain::GeneratedResource],
    position: GridPos,
) -> bool {
    resources.iter().any(|resource| {
        resource.amount > 0
            && resource.target_kind.as_str() != "target:fish"
            && resource.position == position
    })
}

pub(crate) fn upgraded_resource_remaining(
    saved_generator_version: u32,
    resource: &stream_town_domain::GeneratedResource,
    remaining: u32,
) -> u32 {
    if saved_generator_version >= 7 {
        return remaining;
    }
    let multiplier = match resource.target_kind.as_str() {
        "target:tree" => 5,
        "target:ore" | "target:bush" => 100,
        _ => 1,
    };
    remaining.saturating_mul(multiplier)
}

pub(crate) fn apply_combat_damage(
    config: &GameConfig,
    simulation: &mut WorldSimulation,
    content: &ContentCatalog,
    attacker_id: Option<&StableId>,
    target_id: &StableId,
    damage: u32,
) -> Result<bool, stream_town_domain::SimulationError> {
    if std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some() {
        return Ok(false);
    }
    let was_alive = simulation
        .actors
        .get(target_id)
        .is_some_and(|target| target.alive);
    let killed_by_player = attacker_id.is_some_and(|attacker| {
        simulation
            .actors
            .get(attacker)
            .is_some_and(|actor| actor.alive && actor.role.as_str() != "role:enemy")
    });
    let target_before_damage = simulation.actors.get(target_id).cloned();
    let enemy = target_before_damage
        .as_ref()
        .filter(|target| target.role.as_str() == "role:enemy")
        .and_then(|target| actor_archetype(content, target))
        .and_then(|archetype| archetype.enemy.as_ref())
        .cloned();
    let killed = simulation.damage_actor(target_id, damage)?;
    if !killed
        && let (Some(attacker_id), Some(target)) = (attacker_id, target_before_damage.as_ref())
        && enemy_attacks_attacker(content, target)
        && enemy_retaliation_target_is_valid(content, simulation, target, attacker_id)
        && let Some(target) = simulation.actors.get_mut(target_id)
    {
        target.preferred_target = Some(attacker_id.clone());
    }
    if killed
        && was_alive
        && killed_by_player
        && let Some(enemy) = enemy
    {
        let _ = simulation.record_objective_event(
            &content.objectives,
            &ObjectiveEvent::EnemyKilled(enemy.enemy_type),
        );
        let resource = enemy.kill_reward.resource;
        let amount = if simulation.active_community_event == Some(CommunityEvent::Invasion)
            && resource.as_str() == "resource:gold"
        {
            enemy.kill_reward.amount.saturating_mul(2)
        } else {
            enemy.kill_reward.amount
        };
        let capacity = resource_storage_capacity(config, content, simulation, &resource);
        let current = simulation
            .town_resources
            .get(&resource)
            .copied()
            .unwrap_or_default();
        simulation.town_resources.insert(
            resource.clone(),
            current.saturating_add(amount).min(capacity),
        );
        let _ = simulation.record_objective_event(
            &content.objectives,
            &ObjectiveEvent::ResourceGained { resource, amount },
        );
        if let Some(attacker_id) = attacker_id
            && let Some(attacker) = simulation.actors.get(attacker_id)
            && let Some(stats) = effective_role_stats(content, simulation, attacker)
        {
            let building_bonus = military_kill_experience_bonus_per_thousand(content, simulation);
            if building_bonus > 0 {
                let base_experience = u64::from(stats.action_amount)
                    .saturating_mul(u64::from(stats.experience_multiplier_per_thousand))
                    / 1_000;
                let extra = u32::try_from(
                    base_experience
                        .saturating_mul(u64::from(building_bonus))
                        .saturating_add(999)
                        / 1_000,
                )
                .unwrap_or(u32::MAX);
                let _ = simulation.grant_role_experience_capped(
                    attacker_id,
                    extra,
                    1_000,
                    actor_role_level_cap(content, attacker_id),
                    content.progression.role_experience_curve_level_span,
                    content.progression.role_experience_curve_maximum,
                );
            }
        }
    }
    Ok(killed)
}

pub(crate) fn resolve_combat_projectile_impact(
    config: &GameConfig,
    simulation: &mut WorldSimulation,
    content: &ContentCatalog,
    projectile: &CombatProjectile,
) -> Result<bool, stream_town_domain::SimulationError> {
    apply_combat_damage(
        config,
        simulation,
        content,
        match &projectile.source {
            ProjectileSource::Actor(actor) => Some(actor),
            // Unity tower projectiles call TakeDamage with a null Targetable,
            // so they neither provoke retaliation nor count as player kills.
            ProjectileSource::Building(_) => None,
        },
        &projectile.target,
        projectile.damage,
    )
}

pub(crate) fn enemy_retaliation_target_is_valid(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    enemy: &ActorState,
    attacker: &StableId,
) -> bool {
    if let Some(actor) = simulation.actors.get(attacker) {
        return actor.alive
            && actor.role.as_str() != "role:enemy"
            && (enemy_targets_kind(content, enemy, "target:player")
                || (actor.health < actor.max_health
                    && enemy_targets_kind(content, enemy, "target:injured_player")));
    }
    simulation.buildings.get(attacker).is_some_and(|building| {
        building.health > 0
            && (enemy_targets_kind(content, enemy, "target:building")
                || (!building.complete
                    && enemy_targets_kind(content, enemy, "target:construction"))
                || (building.complete
                    && building.health < building_max_health(content, building)
                    && enemy_targets_kind(content, enemy, "target:damaged_building")))
    })
}

pub(crate) fn spawn_combat_projectile(
    commands: &mut Commands,
    render: &RenderAssets,
    config: &GameConfig,
    origin: Vec3,
    projectile: ProjectileSpawn,
) {
    let scale = config.world.cell_size * 0.14;
    let material = match projectile.visual {
        CombatVisualKind::Necrotic => render.projectile_necrotic.clone(),
        CombatVisualKind::Arrow => render.projectile_arrow.clone(),
        CombatVisualKind::Physical | CombatVisualKind::Fireball => render.projectile.clone(),
    };
    let mut entity = commands.spawn((
        WorldEntity,
        CombatProjectile {
            source: projectile.source,
            target: projectile.target,
            damage: projectile.damage,
            speed_cells_per_second: projectile.speed_cells_per_second,
            visual: projectile.visual,
            trail_cooldown_seconds: 0.0,
            remaining_seconds: PROJECTILE_MAX_LIFETIME_SECONDS,
        },
        Transform::from_translation(origin + Vec3::Y * config.world.cell_size * 0.35)
            .with_scale(Vec3::splat(scale)),
    ));
    if projectile.visual == CombatVisualKind::Arrow
        && let Some(scene) = render.projectile_arrow_scene.clone()
    {
        entity.insert(WorldAssetRoot(scene));
    } else {
        entity.insert((Mesh3d(render.cube.clone()), MeshMaterial3d(material)));
    }
}

pub(crate) fn combat_material(
    render: &RenderAssets,
    visual: CombatVisualKind,
) -> Handle<StandardMaterial> {
    match visual {
        CombatVisualKind::Physical => render.impact_physical.clone(),
        CombatVisualKind::Arrow => render.projectile_arrow.clone(),
        CombatVisualKind::Fireball => render.projectile.clone(),
        CombatVisualKind::Necrotic => render.projectile_necrotic.clone(),
    }
}

pub(crate) fn spawn_combat_smoke_field(
    commands: &mut Commands,
    render: &RenderAssets,
    focus: Vec3,
    cell_size: f32,
) {
    let spacing = cell_size * 2.4;
    for (offset, visual) in [
        (-1.5_f32, CombatVisualKind::Physical),
        (-0.5, CombatVisualKind::Arrow),
        (0.5, CombatVisualKind::Fireball),
        (1.5, CombatVisualKind::Necrotic),
    ] {
        let origin = focus + Vec3::X * offset * spacing;
        spawn_combat_impact(commands, render, origin, visual, cell_size);
        for trail_index in 0..7_u16 {
            spawn_combat_trail(
                commands,
                render,
                origin + Vec3::Z * f32::from(trail_index) * cell_size * 0.32,
                Vec3::Z,
                visual,
                cell_size,
            );
        }
    }
}

pub(crate) fn spawn_combat_smoke_arrow(
    commands: &mut Commands,
    render: &RenderAssets,
    focus: Vec3,
    cell_size: f32,
) {
    let origin = focus + Vec3::new(-cell_size * 6.0, cell_size * 0.45, -cell_size * 2.8);
    let target = focus + Vec3::new(cell_size * 5.0, cell_size * 0.45, -cell_size * 2.8);
    let mut transform =
        Transform::from_translation(origin).with_scale(Vec3::splat(cell_size * 0.14));
    transform.look_to(target - origin, Vec3::Y);
    let mut entity = commands.spawn((
        WorldEntity,
        CombatProjectile {
            source: ProjectileSource::Building(
                StableId::new("building:townhall").expect("static stable ID"),
            ),
            target: StableId::new("actor:enemy_0000").expect("static stable ID"),
            damage: 0,
            speed_cells_per_second: 0.0,
            visual: CombatVisualKind::Arrow,
            trail_cooldown_seconds: f32::MAX,
            remaining_seconds: f32::MAX,
        },
        transform,
    ));
    if let Some(scene) = render.projectile_arrow_scene.clone() {
        entity.insert(WorldAssetRoot(scene));
    } else {
        entity.insert((
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.projectile_arrow.clone()),
        ));
    }
}

pub(crate) fn repeat_combat_smoke(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    mut cooldown_seconds: Local<f32>,
    mut arrow_spawned: Local<bool>,
) {
    if std::env::var_os("STREAM_TOWN_SMOKE_COMBAT_VFX").is_none() {
        return;
    }
    *cooldown_seconds -= time.delta_secs();
    if *cooldown_seconds > 0.0 {
        return;
    }
    let centre = GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    };
    let focus = grid_to_world_on_surface(centre, &config.0, &world.generated)
        + Vec3::Y * config.0.world.cell_size * 0.35;
    spawn_combat_smoke_field(&mut commands, &render, focus, config.0.world.cell_size);
    if !*arrow_spawned {
        spawn_combat_smoke_arrow(&mut commands, &render, focus, config.0.world.cell_size);
        *arrow_spawned = true;
    }
    *cooldown_seconds = 0.72;
}

pub(crate) fn spawn_combat_trail(
    commands: &mut Commands,
    render: &RenderAssets,
    position: Vec3,
    direction: Vec3,
    visual: CombatVisualKind,
    cell_size: f32,
) {
    let (duration_seconds, scale) = match visual {
        CombatVisualKind::Arrow => (
            TOWER_TRAIL_SECONDS,
            Vec3::new(TOWER_TRAIL_WIDTH * 0.35, TOWER_TRAIL_WIDTH * 0.35, 0.22),
        ),
        CombatVisualKind::Fireball => (
            0.6,
            Vec3::new(FIREBALL_TRAIL_SIZE * 0.27, FIREBALL_TRAIL_SIZE * 0.27, 0.16),
        ),
        CombatVisualKind::Necrotic => (0.75, Vec3::new(0.07, 0.07, 0.14)),
        CombatVisualKind::Physical => return,
    };
    let base_scale = scale * cell_size;
    let mut transform = Transform::from_translation(position).with_scale(base_scale);
    transform.look_to(direction.normalize_or_zero(), Vec3::Y);
    commands.spawn((
        WorldEntity,
        CombatTrailSegment {
            elapsed_seconds: 0.0,
            duration_seconds,
            base_scale,
        },
        Mesh3d(render.cube.clone()),
        MeshMaterial3d(combat_material(render, visual)),
        transform,
    ));
}

pub(crate) fn spawn_combat_impact(
    commands: &mut Commands,
    render: &RenderAssets,
    origin: Vec3,
    visual: CombatVisualKind,
    cell_size: f32,
) {
    let (particle_count, duration_seconds, speed) = match visual {
        CombatVisualKind::Physical | CombatVisualKind::Arrow => {
            (8_u16, CHARACTER_HIT_SECONDS, 1.25)
        }
        CombatVisualKind::Fireball => (14, 0.55, 2.0),
        CombatVisualKind::Necrotic => (12, 0.65, 1.5),
    };
    for index in 0..particle_count {
        let phase = f32::from(index) / f32::from(particle_count);
        let angle = phase * std::f32::consts::TAU;
        let vertical = 0.35 + f32::from(index % 3) * 0.24;
        let velocity = Vec3::new(angle.cos(), vertical, angle.sin()) * speed * cell_size;
        let base_scale = Vec3::splat(
            cell_size
                * if visual == CombatVisualKind::Fireball {
                    FIREBALL_SIZE * 0.275
                } else {
                    0.075
                },
        );
        commands.spawn((
            WorldEntity,
            CombatImpactParticle {
                elapsed_seconds: 0.0,
                duration_seconds,
                origin,
                velocity,
                base_scale,
            },
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(combat_material(render, visual)),
            Transform::from_translation(origin).with_scale(base_scale),
        ));
    }
}

pub(crate) fn animate_combat_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut trails: Query<(Entity, &mut CombatTrailSegment, &mut Transform)>,
    mut impacts: Query<
        (Entity, &mut CombatImpactParticle, &mut Transform),
        Without<CombatTrailSegment>,
    >,
) {
    for (entity, mut trail, mut transform) in &mut trails {
        trail.elapsed_seconds += time.delta_secs();
        let progress = trail.elapsed_seconds / trail.duration_seconds;
        if progress >= 1.0 {
            commands.entity(entity).try_despawn();
            continue;
        }
        transform.scale = trail.base_scale * (1.0 - progress);
    }
    for (entity, mut impact, mut transform) in &mut impacts {
        impact.elapsed_seconds += time.delta_secs();
        let progress = impact.elapsed_seconds / impact.duration_seconds;
        if progress >= 1.0 {
            commands.entity(entity).try_despawn();
            continue;
        }
        transform.translation = impact.origin
            + impact.velocity * impact.elapsed_seconds
            + Vec3::NEG_Y * 0.5 * 9.8 * impact.elapsed_seconds.powi(2);
        transform.rotation *= Quat::from_rotation_y(time.delta_secs() * 5.0);
        transform.scale = impact.base_scale * (1.0 - progress).sqrt();
    }
}

pub(crate) fn building_effect_material(
    render: &RenderAssets,
    kind: BuildingEffectKind,
) -> Handle<StandardMaterial> {
    match kind {
        BuildingEffectKind::WorkSmoke | BuildingEffectKind::DamageSmoke => {
            render.building_smoke.clone()
        }
        BuildingEffectKind::WorkSpark => render.building_spark.clone(),
        BuildingEffectKind::DamageFire => render.building_fire.clone(),
        BuildingEffectKind::LevelArrow => render.building_upgrade.clone(),
    }
}

pub(crate) fn spawn_building_particle(
    commands: &mut Commands,
    render: &RenderAssets,
    kind: BuildingEffectKind,
    origin: Vec3,
    velocity: Vec3,
    base_scale: Vec3,
    duration_seconds: f32,
    phase: f32,
) {
    commands.spawn((
        WorldEntity,
        BuildingEffectParticle {
            kind,
            elapsed_seconds: 0.0,
            duration_seconds,
            origin,
            velocity,
            base_scale,
            phase,
        },
        Mesh3d(render.cube.clone()),
        MeshMaterial3d(building_effect_material(render, kind)),
        bevy::light::NotShadowCaster,
        bevy::light::NotShadowReceiver,
        Transform::from_translation(origin).with_scale(Vec3::ZERO),
    ));
}

pub(crate) fn spawn_building_work_effect(
    commands: &mut Commands,
    render: &RenderAssets,
    origin: Vec3,
    sparks: bool,
    cell_size: f32,
) {
    for index in 0..5_u16 {
        let phase = f32::from(index) / 5.0;
        let angle = phase * std::f32::consts::TAU + 0.35;
        let particle_origin =
            origin + Vec3::new(angle.cos(), 0.16 + phase * 0.24, angle.sin()) * cell_size * 0.34;
        spawn_building_particle(
            commands,
            render,
            BuildingEffectKind::WorkSmoke,
            particle_origin,
            Vec3::new(
                angle.cos() * 0.22,
                BUILDING_HIT_SMOKE_SPEED,
                angle.sin() * 0.22,
            ) * cell_size,
            Vec3::splat(BUILDING_HIT_SMOKE_SIZE * cell_size * 0.34),
            BUILDING_HIT_SECONDS,
            phase,
        );
    }
    if !sparks {
        return;
    }
    for index in 0..8_u16 {
        let phase = f32::from(index) / 8.0;
        let angle = phase * std::f32::consts::TAU;
        spawn_building_particle(
            commands,
            render,
            BuildingEffectKind::WorkSpark,
            origin + Vec3::Y * cell_size * 0.35,
            Vec3::new(angle.cos(), 0.28 + f32::from(index % 3) * 0.12, angle.sin())
                * BUILDING_HIT_SPARK_SPEED
                * cell_size
                * 0.16,
            Vec3::new(0.045, BUILDING_HIT_SPARK_SIZE, 0.045) * cell_size,
            BUILDING_HIT_SECONDS,
            phase,
        );
    }
}

pub(crate) fn spawn_building_level_up_effect(
    commands: &mut Commands,
    render: &RenderAssets,
    origin: Vec3,
    cell_size: f32,
) {
    let radius = BUILDING_LEVEL_UP_TILE_SIZE * cell_size * 0.38;
    for index in 0..8_u16 {
        let phase = f32::from(index) / 8.0;
        let angle = phase * std::f32::consts::TAU;
        let arrow_origin = origin + Vec3::new(angle.cos() * radius, 0.1, angle.sin() * radius);
        spawn_building_particle(
            commands,
            render,
            BuildingEffectKind::LevelArrow,
            arrow_origin,
            Vec3::Y * cell_size * (1.15 + phase * 0.55),
            Vec3::new(0.12, BUILDING_LEVEL_UP_ARROW_SIZE, 0.12) * cell_size,
            BUILDING_LEVEL_UP_SECONDS,
            phase,
        );
    }
}

pub(crate) fn building_damage_intensity(health: i32, max_health: i32, complete: bool) -> f32 {
    if !complete || health <= 0 {
        return 0.0;
    }
    let health_ratio = building_damage_value(health, max_health);
    ((0.65 - health_ratio) / 0.65).clamp(0.0, 1.0)
}

pub(crate) fn emit_damaged_building_effects(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    render: Res<RenderAssets>,
    mut buildings: Query<(
        &RuntimeBuilding,
        &BuildingPresentation,
        &mut BuildingDamageEmitter,
    )>,
) {
    for (runtime, presentation, mut emitter) in &mut buildings {
        let Some(building) = simulation.0.buildings.get(&runtime.id) else {
            continue;
        };
        let intensity = building_damage_intensity(
            building.health,
            building_max_health(&content.0, building),
            building.complete,
        );
        if intensity <= f32::EPSILON {
            emitter.cooldown_seconds = 0.0;
            continue;
        }
        emitter.cooldown_seconds -= time.delta_secs();
        if emitter.cooldown_seconds > 0.0 {
            continue;
        }
        let cell_size = config.0.world.cell_size;
        let phase =
            f32::from(u16::try_from(emitter.sequence % 16).expect("sequence is bounded")) / 16.0;
        let angle = phase * std::f32::consts::TAU * 2.618_034;
        let radial = BUILDING_DAMAGED_RADIUS * cell_size * (0.18 + 0.38 * intensity);
        let origin = presentation.base_translation
            + Vec3::new(angle.cos() * radial, cell_size * 0.44, angle.sin() * radial);
        spawn_building_particle(
            &mut commands,
            &render,
            BuildingEffectKind::DamageSmoke,
            origin,
            Vec3::new(
                angle.cos() * 0.16,
                0.78 + intensity * 0.52,
                angle.sin() * 0.16,
            ) * cell_size,
            Vec3::splat(cell_size * (0.22 + intensity * 0.18)),
            1.55,
            phase,
        );
        if emitter.sequence.is_multiple_of(2) {
            spawn_building_particle(
                &mut commands,
                &render,
                BuildingEffectKind::DamageFire,
                origin,
                Vec3::Y * cell_size * (0.45 + intensity * 0.35),
                Vec3::new(0.16, 0.4, 0.16) * cell_size * (0.65 + intensity * 0.45),
                0.72,
                phase,
            );
        }
        emitter.sequence = emitter.sequence.wrapping_add(1);
        let authored_density =
            f32::from(BUILDING_DAMAGED_FIRE_AMOUNT) / f32::from(BUILDING_DAMAGED_SMOKE_AMOUNT);
        emitter.cooldown_seconds = (0.22 - intensity * 0.12) / authored_density.max(0.1);
    }
}

pub(crate) fn animate_building_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut effects: Query<(Entity, &mut BuildingEffectParticle, &mut Transform)>,
) {
    for (entity, mut effect, mut transform) in &mut effects {
        effect.elapsed_seconds += time.delta_secs();
        let progress = effect.elapsed_seconds / effect.duration_seconds;
        if progress >= 1.0 {
            commands.entity(entity).try_despawn();
            continue;
        }
        let envelope = (std::f32::consts::PI * progress).sin().max(0.0);
        match effect.kind {
            BuildingEffectKind::WorkSmoke => {
                transform.translation = effect.origin + effect.velocity * effect.elapsed_seconds;
                transform.scale = effect.base_scale * envelope.sqrt() * (0.72 + progress * 0.8);
                transform.rotation *= Quat::from_rotation_y(time.delta_secs() * 1.6);
            }
            BuildingEffectKind::WorkSpark => {
                transform.translation = effect.origin
                    + effect.velocity * effect.elapsed_seconds
                    + Vec3::NEG_Y * 0.5 * 9.8 * effect.elapsed_seconds.powi(2);
                transform.scale = effect.base_scale * (1.0 - progress);
                transform.rotation *= Quat::from_rotation_z(time.delta_secs() * 9.0);
            }
            BuildingEffectKind::LevelArrow => {
                transform.translation = effect.origin + effect.velocity * effect.elapsed_seconds;
                transform.scale = effect.base_scale * envelope.sqrt();
                transform.rotation =
                    Quat::from_rotation_y(effect.phase * std::f32::consts::TAU + progress * 0.35);
            }
            BuildingEffectKind::DamageSmoke => {
                transform.translation = effect.origin + effect.velocity * effect.elapsed_seconds;
                transform.scale = effect.base_scale * envelope.sqrt() * (0.6 + progress * 1.45);
                transform.rotation *= Quat::from_rotation_y(time.delta_secs() * 0.8);
            }
            BuildingEffectKind::DamageFire => {
                transform.translation = effect.origin + effect.velocity * effect.elapsed_seconds;
                let flicker = 0.78 + (progress * 31.0 + effect.phase * 13.0).sin().abs() * 0.32;
                transform.scale = effect.base_scale * envelope * flicker;
                transform.rotation *= Quat::from_rotation_y(time.delta_secs() * 4.0);
            }
        }
    }
}

pub(crate) fn spawn_building_smoke_field(
    commands: &mut Commands,
    render: &RenderAssets,
    focus: Vec3,
    cell_size: f32,
) {
    let spacing = cell_size * 3.4;
    spawn_building_work_effect(
        commands,
        render,
        focus - Vec3::X * spacing,
        false,
        cell_size,
    );
    spawn_building_work_effect(commands, render, focus, true, cell_size);
    spawn_building_level_up_effect(commands, render, focus + Vec3::X * spacing, cell_size);
    for sequence in 0..10_u16 {
        let phase = f32::from(sequence) / 10.0;
        let angle = phase * std::f32::consts::TAU;
        let origin = focus
            + Vec3::Z * spacing
            + Vec3::new(angle.cos(), 0.4, angle.sin()) * BUILDING_DAMAGED_RADIUS * cell_size;
        spawn_building_particle(
            commands,
            render,
            if sequence.is_multiple_of(3) {
                BuildingEffectKind::DamageFire
            } else {
                BuildingEffectKind::DamageSmoke
            },
            origin,
            Vec3::Y * cell_size * if sequence.is_multiple_of(3) { 0.7 } else { 1.0 },
            Vec3::splat(cell_size * 0.32),
            if sequence.is_multiple_of(3) {
                0.72
            } else {
                1.55
            },
            phase,
        );
    }
}

pub(crate) fn repeat_building_smoke(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    mut cooldown_seconds: Local<f32>,
) {
    if std::env::var_os("STREAM_TOWN_SMOKE_BUILDING_VFX").is_none() {
        return;
    }
    *cooldown_seconds -= time.delta_secs();
    if *cooldown_seconds > 0.0 {
        return;
    }
    let centre = GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    };
    let focus = grid_to_world_on_surface(centre, &config.0, &world.generated);
    spawn_building_smoke_field(&mut commands, &render, focus, config.0.world.cell_size);
    *cooldown_seconds = 0.82;
}

pub(crate) fn healing_channel_effect(
    presentation: &PresentationCatalog,
) -> (&StableId, &HealingChannelVfxDef) {
    presentation
        .healing_channel_effects
        .iter()
        .next()
        .expect("validated presentation contains the healing-channel graph")
}

pub(crate) fn healing_burst_effect(
    presentation: &PresentationCatalog,
) -> (&StableId, &HealingBurstVfxDef) {
    presentation
        .healing_burst_effects
        .iter()
        .next()
        .expect("validated presentation contains the completed-heal graph")
}

pub(crate) fn healing_material(color: [f32; 4], alpha_scale: f32) -> StandardMaterial {
    // Unity's VFX gradients are authored in HDR linear colour. Feeding values
    // such as green=16.9 directly into Bevy bloom makes a character-sized cue
    // look screen-sized. Preserve the hue and authored HDR intent while
    // bounding the emission to the same range as the rest of the port's VFX.
    let peak = color[0].max(color[1]).max(color[2]).max(1.0);
    let normalized = [color[0] / peak, color[1] / peak, color[2] / peak];
    let emissive_gain = peak.min(HEALING_EMISSIVE_MAX_COMPONENT);
    StandardMaterial {
        base_color: Color::linear_rgba(
            normalized[0],
            normalized[1],
            normalized[2],
            color[3] * alpha_scale,
        ),
        emissive: LinearRgba::new(
            normalized[0] * emissive_gain,
            normalized[1] * emissive_gain,
            normalized[2] * emissive_gain,
            1.0,
        ),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    }
}

pub(crate) fn healing_gradient_materials(
    materials: &mut Assets<StandardMaterial>,
    gradient: &VfxGradientDef,
) -> Vec<Handle<StandardMaterial>> {
    (0..CHIMNEY_ALPHA_STEPS)
        .filter_map(|step| {
            gradient
                .sample(chimney_alpha_progress(step))
                .map(|color| materials.add(healing_material(color, 1.0)))
        })
        .collect()
}

pub(crate) fn gradient_material(
    materials: &[Handle<StandardMaterial>],
    normalized_age: f32,
) -> Handle<StandardMaterial> {
    let last = materials.len().saturating_sub(1);
    let index = (0..=last)
        .rev()
        .find(|index| {
            let threshold = if last == 0 {
                0.0
            } else {
                f32::from(u16::try_from(*index).unwrap_or(u16::MAX))
                    / f32::from(u16::try_from(last).unwrap_or(u16::MAX))
            };
            normalized_age.clamp(0.0, 1.0) >= threshold
        })
        .unwrap_or_default();
    materials[index].clone()
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub(crate) fn f32_to_u16_saturating(value: f32) -> u16 {
    value.round().clamp(0.0, f32::from(u16::MAX)) as u16
}

pub(crate) fn healing_effect_duration(
    presentation: &PresentationCatalog,
    kind: HealingEffectKind,
) -> f32 {
    match kind {
        HealingEffectKind::Channel => healing_channel_effect(presentation)
            .1
            .duration_seconds()
            .expect("validated channel curve has a final key"),
        HealingEffectKind::Burst | HealingEffectKind::Revive => {
            healing_burst_effect(presentation).1.duration_seconds
        }
    }
}

pub(crate) fn healing_effect_sample(
    presentation: &PresentationCatalog,
    kind: HealingEffectKind,
    elapsed_seconds: f32,
    duration_seconds: f32,
) -> HealingEffectSample {
    let duration = duration_seconds.max(f32::EPSILON);
    let progress = (elapsed_seconds / duration).clamp(0.0, 1.0);
    let envelope = (std::f32::consts::PI * progress).sin().max(0.0);
    let channel_size = healing_channel_effect(presentation)
        .1
        .size_multiplier(elapsed_seconds)
        .unwrap_or_default();
    let disc_size = healing_burst_effect(presentation)
        .1
        .disc_size_multiplier_at(progress)
        .unwrap_or_default();
    match kind {
        HealingEffectKind::Channel => HealingEffectSample {
            ring_scale: channel_size,
            mote_scale: envelope.sqrt(),
            radial_distance: 0.1 + progress * 0.2,
            rise: 0.08 + progress * 0.48,
            rotation_radians: progress * std::f32::consts::TAU * 1.5,
        },
        HealingEffectKind::Burst => HealingEffectSample {
            ring_scale: disc_size,
            mote_scale: envelope,
            radial_distance: 0.12 + progress * 0.32,
            rise: 0.1 + progress * 0.6,
            rotation_radians: progress * std::f32::consts::TAU,
        },
        HealingEffectKind::Revive => HealingEffectSample {
            ring_scale: disc_size * 1.15,
            mote_scale: envelope,
            radial_distance: 0.14 + progress * 0.42,
            rise: 0.12 + progress * 0.78,
            rotation_radians: progress * std::f32::consts::TAU * 1.25,
        },
    }
}

pub(crate) fn spawn_healing_effect(
    commands: &mut Commands,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    origin: Vec3,
    kind: HealingEffectKind,
    cell_size: f32,
    follow_target: Option<StableId>,
) {
    // Unity-authored VFX sizes are already world-space values. The initial
    // port multiplied them by Bevy's two-metre grid cell a second time, which
    // made the green heal/revive burst look like a monster-sized screen flash.
    let world_scale = (cell_size / UNITY_AUTHORED_GRID_CELL_SIZE).clamp(0.25, 4.0);
    // The on-target heal flash is deliberately much smaller than the caster
    // channel and the ten-minute revive burst. It remains attached to the
    // healed actor without washing over nearby characters or the camera.
    let target_scale = if kind == HealingEffectKind::Burst {
        HEALING_TARGET_EFFECT_SCALE
    } else {
        1.0
    };
    let duration_seconds = healing_effect_duration(presentation, kind);
    let (effect, material, base_scale) = match kind {
        HealingEffectKind::Channel => {
            let (effect, definition) = healing_channel_effect(presentation);
            (
                effect.clone(),
                render
                    .healing_channel
                    .get(effect)
                    .cloned()
                    .unwrap_or_else(|| render.healing_green.clone()),
                world_scale * definition.exposed_size * HEALING_CHANNEL_RING_SCALE,
            )
        }
        HealingEffectKind::Burst | HealingEffectKind::Revive => {
            let (effect, _) = healing_burst_effect(presentation);
            (
                effect.clone(),
                render
                    .healing_disc_materials
                    .get(effect)
                    .and_then(|materials| materials.first())
                    .cloned()
                    .unwrap_or_else(|| render.healing_green.clone()),
                world_scale * HEALING_BURST_RING_SCALE * target_scale,
            )
        }
    };
    commands.spawn((
        WorldEntity,
        HealingRingEffect {
            kind,
            effect: effect.clone(),
            origin,
            elapsed_seconds: 0.0,
            duration_seconds,
            base_scale,
            follow_target: follow_target.clone(),
        },
        Mesh3d(render.healing_ring.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(origin + Vec3::Y * 0.08).with_scale(Vec3::ZERO),
    ));

    let (mote_count, mote_size_multiplier): (u16, f32) = match kind {
        HealingEffectKind::Channel => {
            let definition = healing_channel_effect(presentation).1;
            let average_lifetime = (definition.particle_lifetime_seconds[0]
                + definition.particle_lifetime_seconds[1])
                * 0.5;
            (
                f32_to_u16_saturating(definition.emission_rate_per_second * average_lifetime)
                    .min(definition.particle_capacity),
                1.2,
            )
        }
        HealingEffectKind::Burst | HealingEffectKind::Revive => {
            let definition = healing_burst_effect(presentation).1;
            (
                definition.plus_burst_count.min(definition.plus_capacity),
                if kind == HealingEffectKind::Revive {
                    1.25
                } else {
                    1.0
                },
            )
        }
    };
    for mote_index in 0..mote_count {
        let phase = f32::from(mote_index) / f32::from(mote_count);
        let angle_radians = phase * std::f32::consts::TAU;
        let (mesh, material) = if kind == HealingEffectKind::Channel {
            (render.chimney_particle.clone(), material.clone())
        } else {
            (
                render
                    .healing_plus
                    .clone()
                    .unwrap_or_else(|| render.cube.clone()),
                render
                    .healing_plus_materials
                    .get(&effect)
                    .and_then(|materials| materials.first())
                    .cloned()
                    .unwrap_or_else(|| render.healing_green.clone()),
            )
        };
        commands.spawn((
            WorldEntity,
            HealingMoteEffect {
                kind,
                effect: effect.clone(),
                origin,
                elapsed_seconds: -phase * 0.22,
                duration_seconds,
                angle_radians,
                phase,
                base_scale: Vec3::splat(world_scale * HEALING_MOTE_SCALE * target_scale),
                distance_scale: world_scale * target_scale,
                size_multiplier: mote_size_multiplier,
                follow_target: follow_target.clone(),
            },
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(origin).with_scale(Vec3::ZERO),
            bevy::light::NotShadowCaster,
            bevy::light::NotShadowReceiver,
        ));
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn animate_healing_effects(
    mut commands: Commands,
    time: Res<Time>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    actors: Query<(&Agent, &Transform), (Without<HealingRingEffect>, Without<HealingMoteEffect>)>,
    mut rings: Query<(
        Entity,
        &mut HealingRingEffect,
        &mut Transform,
        &mut MeshMaterial3d<StandardMaterial>,
    )>,
    mut motes: Query<
        (
            Entity,
            &mut HealingMoteEffect,
            &mut Transform,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<HealingRingEffect>,
    >,
) {
    let actor_positions: BTreeMap<_, _> = actors
        .iter()
        .map(|(agent, transform)| (agent.id.clone(), transform.translation))
        .collect();
    for (entity, mut effect, mut transform, mut material) in &mut rings {
        effect.elapsed_seconds += time.delta_secs();
        if effect.elapsed_seconds >= effect.duration_seconds {
            commands.entity(entity).try_despawn();
            continue;
        }
        if let Some(target) = effect.follow_target.as_ref()
            && let Some(position) = actor_positions.get(target)
        {
            effect.origin = *position;
        }
        let sample = healing_effect_sample(
            &presentation.0,
            effect.kind,
            effect.elapsed_seconds,
            effect.duration_seconds,
        );
        transform.translation = effect.origin + Vec3::Y * 0.08;
        transform.rotation = Quat::from_rotation_y(sample.rotation_radians * 0.2);
        transform.scale = Vec3::splat(effect.base_scale * sample.ring_scale);
        if effect.kind != HealingEffectKind::Channel
            && let Some(materials) = render.healing_disc_materials.get(&effect.effect)
        {
            material.0 =
                gradient_material(materials, effect.elapsed_seconds / effect.duration_seconds);
        }
    }
    for (entity, mut effect, mut transform, mut material) in &mut motes {
        effect.elapsed_seconds += time.delta_secs();
        if effect.elapsed_seconds >= effect.duration_seconds {
            commands.entity(entity).try_despawn();
            continue;
        }
        if let Some(target) = effect.follow_target.as_ref()
            && let Some(position) = actor_positions.get(target)
        {
            effect.origin = *position;
        }
        let sample = healing_effect_sample(
            &presentation.0,
            effect.kind,
            effect.elapsed_seconds.max(0.0),
            effect.duration_seconds,
        );
        let angle =
            effect.angle_radians + sample.rotation_radians + effect.phase * std::f32::consts::PI;
        let radius = sample.radial_distance * effect.distance_scale;
        transform.translation = effect.origin
            + Vec3::new(
                angle.cos() * radius,
                sample.rise * effect.distance_scale,
                angle.sin() * radius,
            );
        transform.rotation = Quat::from_rotation_y(-angle * 0.35);
        let particle_age =
            (effect.elapsed_seconds.max(0.0) / effect.duration_seconds).clamp(0.0, 1.0);
        let authored_scale = if effect.kind == HealingEffectKind::Channel {
            sample.mote_scale
        } else {
            healing_burst_effect(&presentation.0)
                .1
                .plus_size_multiplier(particle_age)
                .unwrap_or_default()
        };
        transform.scale = effect.base_scale * authored_scale * effect.size_multiplier;
        if effect.kind != HealingEffectKind::Channel
            && let Some(materials) = render.healing_plus_materials.get(&effect.effect)
        {
            material.0 = gradient_material(materials, particle_age);
        }
    }
}

pub(crate) fn move_combat_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    render: Res<RenderAssets>,
    mut simulation: ResMut<SimulationRuntime>,
    actors: Query<(&Agent, &Transform), Without<CombatProjectile>>,
    mut projectiles: Query<(Entity, &mut CombatProjectile, &mut Transform), Without<Agent>>,
) {
    let positions: BTreeMap<_, _> = actors
        .iter()
        .map(|(agent, transform)| (agent.id.clone(), transform.translation))
        .collect();
    for (entity, mut projectile, mut transform) in &mut projectiles {
        projectile.remaining_seconds -= time.delta_secs();
        if projectile.remaining_seconds <= 0.0 {
            commands.entity(entity).try_despawn();
            continue;
        }
        let source_valid = match &projectile.source {
            ProjectileSource::Actor(actor) => simulation
                .0
                .actors
                .get(actor)
                .is_some_and(|actor| actor.alive),
            ProjectileSource::Building(building) => simulation
                .0
                .buildings
                .get(building)
                .is_some_and(|building| building.complete),
        };
        let Some(target) = simulation.0.actors.get(&projectile.target) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let Some(target_position) = positions.get(&projectile.target).copied() else {
            commands.entity(entity).try_despawn();
            continue;
        };
        if !source_valid || !target.alive {
            commands.entity(entity).try_despawn();
            continue;
        }
        let target_position = target_position + Vec3::Y * config.0.world.cell_size * 0.35;
        let delta = target_position - transform.translation;
        let step = projectile.speed_cells_per_second * config.0.world.cell_size * time.delta_secs();
        if delta.length_squared() <= step.max(0.1).powi(2) {
            if let Err(error) = resolve_combat_projectile_impact(
                &config.0,
                &mut simulation.0,
                &content.0,
                &projectile,
            ) {
                warn!(target = %projectile.target, %error, "projectile impact failed");
            }
            spawn_combat_impact(
                &mut commands,
                &render,
                target_position,
                projectile.visual,
                config.0.world.cell_size,
            );
            commands.entity(entity).try_despawn();
        } else {
            transform.translation += delta.normalize_or_zero() * step;
            transform.look_to(delta.normalize_or_zero(), Vec3::Y);
            projectile.trail_cooldown_seconds -= time.delta_secs();
            if projectile.trail_cooldown_seconds <= 0.0 {
                spawn_combat_trail(
                    &mut commands,
                    &render,
                    transform.translation,
                    delta,
                    projectile.visual,
                    config.0.world.cell_size,
                );
                projectile.trail_cooldown_seconds = 0.045;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn update_enemy_encounters(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    enemy_navigation: Res<EnemyNavigationRuntime>,
    mut simulation: ResMut<SimulationRuntime>,
    mut night_waves: ResMut<NightEnemyWaveRuntime>,
    agents: Query<(Entity, &Agent)>,
) {
    let dead_enemies: Vec<_> = agents
        .iter()
        .filter(|(_, agent)| agent.kind == ActorKind::Enemy)
        .filter(|(_, agent)| {
            !simulation
                .0
                .actors
                .get(&agent.id)
                .is_some_and(|actor| actor.alive)
        })
        .map(|(entity, agent)| (entity, agent.id.clone()))
        .collect();
    for (entity, enemy) in dead_enemies {
        commands.entity(entity).try_despawn();
        simulation.0.actors.remove(&enemy);
        for camp in simulation.0.enemy_camps.values_mut() {
            camp.spawned_enemies.remove(&enemy);
        }
        if let Some(raid) = &mut simulation.0.active_raid {
            raid.tracked_enemies.remove(&enemy);
        }
    }

    if !night_waves.pending_spawns.is_empty() {
        night_waves.spawn_remaining_seconds =
            (night_waves.spawn_remaining_seconds - time.delta_secs_f64()).max(0.0);
        if night_waves.spawn_remaining_seconds <= f64::EPSILON
            && let Some(pending) = night_waves.pending_spawns.pop_front()
        {
            let spawn_source = simulation
                .0
                .enemy_camps
                .get(&pending.camp)
                .and_then(|camp| {
                    let camp_archetype = content.0.archetypes.get(&camp.archetype)?;
                    Some((
                        camp.clone(),
                        camp_archetype.enemy_spawner.clone()?,
                        camp_archetype.footprint,
                    ))
                });
            if let Some((camp, spawner, footprint)) = spawn_source {
                let position = enemy_spawn_position(
                    &world.generated,
                    &content.0,
                    &simulation.0,
                    enemy_navigation.field.as_ref(),
                    &camp,
                    &spawner,
                    footprint,
                    pending.serial,
                );
                if let Some(enemy) = spawn_runtime_enemy(
                    &mut commands,
                    &config.0,
                    &world.generated,
                    &content.0,
                    &presentation.0,
                    asset_server.as_deref(),
                    &asset_root.0,
                    &render,
                    &mut simulation.0,
                    pending.archetype,
                    position,
                ) {
                    if pending.final_raid_wave {
                        let player_count = simulation_player_count(&simulation.0);
                        let boss_health =
                            i32::try_from(50_usize.saturating_mul(player_count).max(1_000))
                                .unwrap_or(i32::MAX);
                        if let Some(actor) = simulation.0.actors.get_mut(&enemy) {
                            actor.health = boss_health;
                            actor.max_health = boss_health;
                        }
                    }
                    if let Some(camp) = simulation.0.enemy_camps.get_mut(&pending.camp) {
                        camp.spawned_enemies.insert(enemy.clone());
                    }
                    if let Some(raid) = &mut simulation.0.active_raid {
                        raid.tracked_enemies.insert(enemy);
                    }
                }
            }
            night_waves.spawn_remaining_seconds = night_waves.spawn_interval_seconds;
        }
        return;
    }

    if simulation.0.active_event == Some(TownEvent::EnemyRaid)
        && simulation.0.active_raid.is_none()
        && let (Some(enemy), Some(boss)) = (
            archetype_id_by_source(&content.0, ArchetypeKind::Enemy, "Enemy_Minotaur.prefab"),
            archetype_id_by_source(
                &content.0,
                ArchetypeKind::Enemy,
                "Enemy_MinotaurBoss.prefab",
            ),
        )
    {
        let player_count = simulation_player_count(&simulation.0);
        let wave_size = event_adjusted_wave_size(
            raid_enemies_per_wave(player_count),
            simulation.0.active_community_event,
        );
        let _ = simulation.0.start_raid(5, wave_size, enemy, boss);
    }

    let next_wave = simulation.0.active_raid.as_ref().and_then(|raid| {
        raid.tracked_enemies.is_empty().then(|| {
            if raid.current_wave >= raid.total_waves {
                None
            } else {
                let final_wave = raid.current_wave + 1 == raid.total_waves;
                Some((
                    if final_wave {
                        raid.boss_archetype.clone()
                    } else {
                        raid.enemy_archetype.clone()
                    },
                    if final_wave { 1 } else { raid.enemies_per_wave },
                    final_wave,
                ))
            }
        })
    });
    if matches!(next_wave, Some(None)) {
        simulation.0.finish_raid();
    } else if let Some(Some((archetype, count, final_wave))) = next_wave {
        let camps = simulation
            .0
            .enemy_camps
            .iter()
            .filter(|(_, camp)| {
                content
                    .0
                    .archetypes
                    .get(&camp.archetype)
                    .is_some_and(|archetype| archetype.enemy_spawner.is_some())
            })
            .map(|(id, _)| id.clone())
            .collect::<Vec<_>>();
        if camps.is_empty() {
            simulation.0.finish_raid();
        } else {
            for offset in 0..u64::from(count) {
                let serial = simulation.0.next_enemy_serial.saturating_add(offset);
                // Unity chooses a random eligible camp for every raid member.
                // Use the same per-enemy distribution with a stable hash so
                // fixed seeds and replays remain deterministic in Bevy.
                let camp_index = usize::try_from(
                    generated_enemy_camp_hash(simulation.0.world_seed, &archetype, serial)
                        % u64::try_from(camps.len()).expect("camp count fits u64"),
                )
                .expect("raid camp index fits usize");
                night_waves.pending_spawns.push_back(PendingEnemySpawn {
                    camp: camps[camp_index].clone(),
                    archetype: archetype.clone(),
                    serial,
                    final_raid_wave: final_wave,
                });
            }
            night_waves.spawn_interval_seconds =
                ENEMY_WAVE_SPAWN_SPREAD_SECONDS / f64::from(count.max(1));
            night_waves.spawn_remaining_seconds = 0.0;
            if let Some(raid) = &mut simulation.0.active_raid {
                raid.current_wave = raid.current_wave.saturating_add(1);
                raid.tracked_enemies.clear();
            }
        }
        return;
    }

    if simulation.0.active_raid.is_some() {
        return;
    }
    if config
        .0
        .time
        .sample(simulation.0.elapsed_seconds)
        .is_daytime
    {
        *night_waves = NightEnemyWaveRuntime::default();
        return;
    }
    if !night_enemy_wave_due(&mut night_waves, simulation.0.day, time.delta_secs_f64()) {
        return;
    }
    let player_count = simulation_player_count(&simulation.0);
    let camps = simulation
        .0
        .enemy_camps
        .iter()
        .filter_map(|(camp_id, camp)| {
            let archetype = content.0.archetypes.get(&camp.archetype)?;
            Some((
                camp_id.clone(),
                camp.clone(),
                archetype.enemy_spawner.clone()?,
                archetype.footprint,
            ))
        })
        .collect::<Vec<_>>();
    if camps.is_empty() {
        return;
    }
    let wave_archetype = StableId::new("enemy:night_wave").expect("static stable ID");
    let wave_count = event_adjusted_wave_size(
        raid_enemies_per_wave(player_count),
        simulation.0.active_community_event,
    );
    for offset in 0..u64::from(wave_count) {
        let serial = simulation.0.next_enemy_serial.saturating_add(offset);
        let camp_index = usize::try_from(
            generated_enemy_camp_hash(simulation.0.world_seed, &wave_archetype, serial)
                % u64::try_from(camps.len()).expect("camp count fits u64"),
        )
        .expect("night-wave camp index fits usize");
        let (camp_id, _, spawner, _) = &camps[camp_index];
        let archetype = weighted_enemy_archetype(spawner, simulation.0.world_seed, serial);
        night_waves.pending_spawns.push_back(PendingEnemySpawn {
            camp: camp_id.clone(),
            archetype,
            serial,
            final_raid_wave: false,
        });
    }
    night_waves.spawn_interval_seconds =
        ENEMY_WAVE_SPAWN_SPREAD_SECONDS / f64::from(wave_count.max(1));
    night_waves.spawn_remaining_seconds = 0.0;
}

pub(crate) fn night_enemy_wave_due(
    runtime: &mut NightEnemyWaveRuntime,
    day: u32,
    delta_seconds: f64,
) -> bool {
    if runtime.night_day != Some(day) {
        runtime.night_day = Some(day);
        runtime.remaining_seconds = 0.0;
        runtime.wave_index = 0;
    }
    runtime.remaining_seconds = (runtime.remaining_seconds - delta_seconds.max(0.0)).max(0.0);
    if runtime.remaining_seconds > f64::EPSILON {
        return false;
    }
    runtime.remaining_seconds = NIGHT_ENEMY_WAVE_INTERVAL_SECONDS;
    runtime.wave_index = runtime.wave_index.saturating_add(1);
    true
}

pub(crate) fn simulation_player_count(simulation: &WorldSimulation) -> usize {
    simulation
        .actors
        .values()
        .filter(|actor| {
            actor.role.as_str() != "role:enemy" && !is_guardhouse_defender_id(&actor.id)
        })
        .count()
}

pub(crate) fn raid_enemies_per_wave(player_count: usize) -> u16 {
    u16::try_from(player_count.clamp(2, 25)).unwrap_or(25)
}

pub(crate) fn restart_world_after_town_hall_falls(
    mut commands: Commands,
    simulation: Res<SimulationRuntime>,
    mut restart: ResMut<TownRestartRuntime>,
) {
    let town_hall = StableId::new("building:townhall").expect("static stable ID");
    if simulation.0.buildings.contains_key(&town_hall) || restart.retained_players.is_some() {
        return;
    }
    let mut retained_players = simulation
        .0
        .actors
        .values()
        .filter(|actor| {
            actor.role.as_str() != "role:enemy" && !is_guardhouse_defender_id(&actor.id)
        })
        .cloned()
        .collect::<Vec<_>>();
    if let (Some(ruler), Some(previous_role)) = (
        simulation.0.current_ruler.as_ref(),
        simulation.0.ruler_previous_role.as_ref(),
    ) && let Some(actor) = retained_players.iter_mut().find(|actor| &actor.id == ruler)
    {
        actor.role.clone_from(previous_role);
    }
    restart.retained_players = Some(retained_players);
    restart.suppress_exit_save = true;
    warn!("Town Hall fell; rebuilding the town while retaining player identities and roles");
    queue_world_loading(&mut commands);
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn sync_fish_god_presentation(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    simulation: Res<SimulationRuntime>,
    existing: Query<
        (Entity, Option<&FishGodAnimation>, Option<&FishGodExitTimer>),
        With<FishGodPresentation>,
    >,
) {
    let active = simulation.0.fish_god.is_some();
    if !active {
        for (entity, animation, exit_timer) in &existing {
            if animation.is_some() {
                if exit_timer.is_none() {
                    commands.entity(entity).insert(FishGodExitTimer {
                        remaining_seconds: FISH_GOD_EXIT_DELAY_SECONDS,
                    });
                }
            } else {
                commands.entity(entity).try_despawn();
            }
        }
        return;
    }
    if existing.iter().any(|(_, animation, _)| animation.is_some()) {
        return;
    }
    let spawn = nearest_walkable(
        &world.generated,
        GridPos {
            x: config.0.world.width / 2,
            z: config.0.world.height / 2 + 6,
        },
    )
    .unwrap_or(GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    });
    let position = grid_to_world_on_surface(spawn, &config.0, &world.generated);
    let fish_god = archetype_by_source(&content.0, ArchetypeKind::Other, "Event_FishGod.prefab")
        .or_else(|| {
            content
                .0
                .archetypes
                .values()
                .find(|archetype| archetype.source_path.ends_with("Event_FishGod.prefab"))
        });
    let mut entity = commands.spawn((
        WorldEntity,
        FishGodPresentation,
        FishGodAnimation,
        Transform::from_translation(position),
    ));
    if let Some((archetype, scene)) = fish_god
        .and_then(|archetype| default_archetype_scene(archetype).map(|scene| (archetype, scene)))
        .filter(|(_, scene)| {
            asset_server.is_some() && converted_asset_exists(&asset_root.0, &scene.asset_path)
        })
    {
        entity.insert((
            WorldAssetRoot(
                asset_server
                    .as_deref()
                    .expect("asset server checked above")
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            Transform::from_translation(position)
                .with_scale(Vec3::splat(config.0.world.cell_size / 2.0)),
        ));
        if let Some(material) = prefab_material_spec(archetype, scene, &presentation.0, &render) {
            entity.insert(material);
        }
        if let Some(animation) =
            complete_converted_animation_spec(archetype, scene, &presentation.0)
        {
            entity.insert(animation);
        }
    } else {
        let scale = config.0.world.cell_size * 1.5;
        entity.insert((
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.food.clone()),
            Transform::from_translation(position + Vec3::Y * scale * 0.5)
                .with_scale(Vec3::splat(scale)),
        ));
    }
    if let Some((effect, _)) = raining_fish_effect(&presentation.0) {
        commands.spawn((
            Name::new("VFX_RainingFish (Unity parity)"),
            WorldEntity,
            FishGodPresentation,
            FallingFishEmitter {
                effect: effect.clone(),
                emission_remainder: 0.0,
                sequence: 0,
            },
            Transform::default(),
        ));
    }
}

pub(crate) fn raining_fish_effect(
    presentation: &PresentationCatalog,
) -> Option<(&StableId, &RainingFishVfxDef)> {
    presentation
        .raining_fish_effects
        .iter()
        .find(|(_, effect)| effect.source_path.ends_with(RAINING_FISH_PREFAB_SUFFIX))
}

pub(crate) fn emit_falling_fish(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    world: Res<WorldRuntime>,
    mut emitters: Query<&mut FallingFishEmitter>,
    existing: Query<(), With<FallingFish>>,
) {
    let mut visible_count = existing.iter().count();
    for mut emitter in &mut emitters {
        let Some(effect) = presentation.0.raining_fish_effects.get(&emitter.effect) else {
            continue;
        };
        let available = RAINING_FISH_RENDER_BUDGET.saturating_sub(visible_count);
        if available == 0 {
            continue;
        }
        let prewarm_count = if emitter.sequence == 0 && effect.prewarm {
            available
        } else {
            0
        };
        let render_budget =
            u16::try_from(RAINING_FISH_RENDER_BUDGET).expect("raining-fish render budget fits u16");
        let representative_rate = effect.emission_rate_per_second * f32::from(render_budget)
            / f32::from(effect.max_particles);
        emitter.emission_remainder += representative_rate * time.delta_secs();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let emitted = emitter.emission_remainder.floor() as usize;
        emitter.emission_remainder -=
            f32::from(u16::try_from(emitted).expect("bounded raining-fish emission fits u16"));
        let spawn_count = prewarm_count.max(emitted.min(available));
        for index in 0..spawn_count {
            let sequence = emitter.sequence;
            emitter.sequence = emitter.sequence.wrapping_add(1);
            let initial_age = if prewarm_count > 0 {
                effect.lifetime_seconds
                    * f32::from(u16::try_from(index).expect("render budget index fits u16"))
                    / f32::from(
                        u16::try_from(prewarm_count).expect("raining-fish render budget fits u16"),
                    )
            } else {
                0.0
            };
            spawn_falling_fish(
                &mut commands,
                effect,
                sequence,
                world.generated.seed,
                initial_age,
                &config.0,
                &world.generated,
                &render,
                asset_server.as_deref(),
                &asset_root.0,
            );
        }
        visible_count += spawn_count;
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_falling_fish(
    commands: &mut Commands,
    effect: &RainingFishVfxDef,
    sequence: u32,
    world_seed: u64,
    initial_age: f32,
    config: &GameConfig,
    world: &GeneratedWorld,
    render: &RenderAssets,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
) {
    let seed = falling_fish_seed(world_seed, sequence, 0);
    let local_offset = Vec3::new(
        (deterministic_unit(falling_fish_seed(world_seed, sequence, 1)) - 0.5)
            * effect.shape_scale[0],
        (deterministic_unit(falling_fish_seed(world_seed, sequence, 2)) - 0.5)
            * effect.shape_scale[1],
        (deterministic_unit(falling_fish_seed(world_seed, sequence, 3)) - 0.5)
            * effect.shape_scale[2],
    );
    let shape_rotation = Quat::from_euler(
        EulerRot::XYZ,
        effect.shape_rotation_degrees[0].to_radians(),
        effect.shape_rotation_degrees[1].to_radians(),
        effect.shape_rotation_degrees[2].to_radians(),
    );
    let offset = shape_rotation * local_offset;
    let size = deterministic_f32_range(
        effect.start_size,
        falling_fish_seed(world_seed, sequence, 4),
    );
    let mut particle = FallingFish {
        effect: StableId::new(format!("particle_effect:{}", effect.source_guid))
            .expect("converted effect GUID forms a stable ID"),
        sequence,
        velocity: Vec3::ZERO,
        angular_velocity: Vec3::new(
            1.2 + deterministic_unit(seed.rotate_left(7)) * 2.8,
            0.8 + deterministic_unit(seed.rotate_left(13)) * 2.2,
            1.0 + deterministic_unit(seed.rotate_left(19)) * 2.4,
        ),
        age_seconds: 0.0,
        lifetime_seconds: effect.lifetime_seconds,
        base_scale: size * 0.01,
        collision_count: 0,
    };
    let mut transform =
        Transform::from_translation(Vec3::from_array(effect.emitter_position) + offset)
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                deterministic_unit(seed.rotate_left(3)) * std::f32::consts::TAU,
                deterministic_unit(seed.rotate_left(11)) * std::f32::consts::TAU,
                deterministic_unit(seed.rotate_left(23)) * std::f32::consts::TAU,
            ))
            .with_scale(Vec3::splat(particle.base_scale));
    if initial_age > f32::EPSILON {
        let mut remaining = initial_age;
        while remaining > f32::EPSILON && particle.age_seconds < particle.lifetime_seconds {
            let step = remaining.min(0.05);
            advance_falling_fish(
                effect,
                &mut particle,
                &mut transform,
                world_seed,
                sequence,
                config,
                world,
                step,
            );
            remaining -= step;
        }
    }
    let mut entity = commands.spawn((
        Name::new("Raining fish"),
        WorldEntity,
        FishGodPresentation,
        particle,
        transform,
        bevy::light::NotShadowCaster,
        bevy::light::NotShadowReceiver,
    ));
    let authored =
        asset_server.filter(|_| converted_asset_exists(asset_root, &effect.model_asset_path));
    let critter = render
        .presentation_materials
        .get(&effect.material)
        .and_then(|material| match material {
            ResolvedMaterialHandle::Critter(material) => Some(material.clone()),
            _ => None,
        });
    if let (Some(server), Some(material)) = (authored, critter) {
        entity.insert((
            Mesh3d(
                server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset(effect.model_asset_path.clone()),
                ),
            ),
            MeshMaterial3d(material),
        ));
    } else {
        entity.insert((
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.food.clone()),
        ));
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn advance_falling_fish(
    effect: &RainingFishVfxDef,
    fish: &mut FallingFish,
    transform: &mut Transform,
    world_seed: u64,
    sequence: u32,
    config: &GameConfig,
    world: &GeneratedWorld,
    delta_seconds: f32,
) {
    fish.age_seconds += delta_seconds;
    let phase = fish.age_seconds * effect.noise_scroll_speed * std::f32::consts::TAU;
    let x_phase = phase
        + deterministic_unit(falling_fish_seed(world_seed, sequence, 5)) * std::f32::consts::TAU;
    let z_phase = phase
        + deterministic_unit(falling_fish_seed(world_seed, sequence, 6)) * std::f32::consts::TAU;
    fish.velocity.x +=
        (x_phase * effect.noise_frequency).sin() * effect.noise_strength[0] * delta_seconds;
    fish.velocity.z +=
        (z_phase * effect.noise_frequency).cos() * effect.noise_strength[2] * delta_seconds;
    fish.velocity.y +=
        (x_phase * effect.noise_frequency).cos() * effect.noise_strength[1] * delta_seconds;
    fish.velocity.y -= effect.gravity * 9.81 * delta_seconds;
    transform.translation += fish.velocity * delta_seconds;
    transform.rotate_local_x(fish.angular_velocity.x * delta_seconds);
    transform.rotate_local_y(fish.angular_velocity.y * delta_seconds);
    transform.rotate_local_z(fish.angular_velocity.z * delta_seconds);
    if let Some(cell) = world_to_grid(transform.translation, config) {
        let floor = terrain_height(world, cell);
        if transform.translation.y <= floor && fish.velocity.y < 0.0 {
            transform.translation.y = floor;
            fish.velocity.y = -fish.velocity.y * effect.collision_bounce;
            fish.age_seconds += fish.lifetime_seconds * effect.collision_lifetime_loss;
            fish.collision_count = fish.collision_count.saturating_add(1);
        }
    }
    let life = (fish.age_seconds / fish.lifetime_seconds).clamp(0.0, 1.0);
    let scale = effect.size_multiplier(life).unwrap_or(1.0) * fish.base_scale;
    transform.scale = Vec3::splat(scale.max(0.0));
}

pub(crate) fn animate_falling_fish(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    presentation: Res<RuntimePresentation>,
    world: Res<WorldRuntime>,
    mut fish: Query<(Entity, &mut FallingFish, &mut Transform)>,
) {
    for (entity, mut fish, mut transform) in &mut fish {
        let Some(effect) = presentation.0.raining_fish_effects.get(&fish.effect) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let sequence = fish.sequence;
        advance_falling_fish(
            effect,
            &mut fish,
            &mut transform,
            world.generated.seed,
            sequence,
            &config.0,
            &world.generated,
            time.delta_secs(),
        );
        if fish.age_seconds >= fish.lifetime_seconds {
            commands.entity(entity).try_despawn();
        }
    }
}

pub(crate) fn falling_fish_seed(world_seed: u64, sequence: u32, salt: u32) -> u32 {
    let low = u32::try_from(world_seed & u64::from(u32::MAX)).expect("masked seed fits u32");
    low.wrapping_add(sequence.wrapping_mul(0x9E37_79B9))
        .wrapping_add(salt.wrapping_mul(0x85EB_CA6B))
}

pub(crate) fn drive_fish_god_exit(
    mut commands: Commands,
    time: Res<Time>,
    parents: Query<&ChildOf>,
    mut roots: Query<
        (
            Entity,
            &mut FishGodExitTimer,
            Option<&FishGodExitTriggerSent>,
        ),
        With<FishGodAnimation>,
    >,
    mut drivers: Query<&mut ConvertedAnimationDriver>,
) {
    for (entity, mut exit, trigger_already_sent) in &mut roots {
        let mut trigger_sent = false;
        for mut driver in &mut drivers {
            if trigger_already_sent.is_some() {
                break;
            }
            let mut ancestor = driver.actor_root;
            let mut belongs_to_fish_god = ancestor == entity;
            for _ in 0..64 {
                if belongs_to_fish_god {
                    break;
                }
                let Ok(parent) = parents.get(ancestor) else {
                    break;
                };
                ancestor = parent.parent();
                belongs_to_fish_god = ancestor == entity;
            }
            if !belongs_to_fish_god {
                continue;
            }
            for layer in &mut driver.layers {
                let _ = layer.runtime.set_trigger("Exit");
            }
            trigger_sent = true;
        }
        if trigger_sent {
            commands.entity(entity).insert(FishGodExitTriggerSent);
        }
        exit.remaining_seconds -= time.delta_secs();
        if exit.remaining_seconds <= 0.0 {
            commands.entity(entity).try_despawn();
        }
    }
}

pub(crate) fn best_tower_target(
    simulation: &WorldSimulation,
    centre: GridPos,
    range_milli_cells: u32,
) -> Option<&ActorState> {
    simulation
        .actors
        .values()
        .filter(|actor| actor.alive && actor.role.as_str() == "role:enemy")
        .filter(|actor| within_milli_cell_range(actor.position, centre, range_milli_cells))
        .min_by_key(|actor| {
            (
                grid_distance_squared(actor.position, centre),
                actor.id.clone(),
            )
        })
}

pub(crate) fn tower_projectile_for_building(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building_id: &StableId,
) -> Option<(ProjectileSpawn, f32)> {
    let state = simulation
        .buildings
        .get(building_id)
        .filter(|building| building.complete && building.health > 0)?;
    let definition = building_def_for_archetype(content, &state.archetype)?;
    let shooter = definition.projectile_shooter.as_ref()?;
    let footprint = rotated_footprint(definition.footprint, state.rotation_quarter_turns);
    let centre = GridPos {
        x: state.position.x.saturating_add(footprint[0] / 2),
        z: state.position.z.saturating_add(footprint[1] / 2),
    };
    let target = best_tower_target(simulation, centre, shooter.range_milli_cells)?;
    Some((
        ProjectileSpawn {
            source: ProjectileSource::Building(building_id.clone()),
            target: target.id.clone(),
            damage: leveled_percentage_stat(
                shooter.damage,
                shooter.damage_bonus_per_level_per_thousand,
                state.level,
            ),
            speed_cells_per_second: milli_units_as_f32(shooter.movement_milli_cells_per_second),
            visual: CombatVisualKind::Arrow,
        },
        milli_units_as_f32(shooter.fire_milliseconds),
    ))
}

pub(crate) fn update_tower_shooters(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    render: Res<RenderAssets>,
    simulation: Res<SimulationRuntime>,
    mut towers: Query<(&RuntimeBuilding, &Transform, &mut TowerShooter)>,
) {
    for (runtime, transform, mut tower) in &mut towers {
        tower.cooldown_seconds = (tower.cooldown_seconds - time.delta_secs()).max(0.0);
        if tower.cooldown_seconds > f32::EPSILON {
            continue;
        }
        let Some((projectile, cooldown_seconds)) =
            tower_projectile_for_building(&content.0, &simulation.0, &runtime.id)
        else {
            continue;
        };
        spawn_combat_projectile(
            &mut commands,
            &render,
            &config.0,
            transform.translation + Vec3::Y * config.0.world.cell_size * 0.5,
            projectile,
        );
        tower.cooldown_seconds = cooldown_seconds;
    }
}

pub(crate) fn action_cooldown(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &StableId,
    goal: &AgentGoal,
) -> f32 {
    let fallback = match goal {
        AgentGoal::Attack(_)
        | AgentGoal::AttackBuilding(_)
        | AgentGoal::Heal(_)
        | AgentGoal::PlantTree(_)
        | AgentGoal::Prospect { .. }
        | AgentGoal::PlantBush(_) => 1.0,
        AgentGoal::Construct(_) => 0.5,
        AgentGoal::VisitRegenerationStation(_) => 0.1,
        AgentGoal::Gather(_) | AgentGoal::HarvestFarm(_) => 0.75,
        // `PlayerInventory.DepositResources` waits 2.5 seconds before its
        // transfer callback. Keep the complete authored station cadence.
        AgentGoal::Deposit => 2.5,
        AgentGoal::WaitForStorage | AgentGoal::Wander => 0.0,
    };
    let Some(actor) = simulation.actors.get(actor) else {
        return fallback;
    };
    if matches!(goal, AgentGoal::Attack(_) | AgentGoal::AttackBuilding(_))
        && let Some(enemy) =
            actor_archetype(content, actor).and_then(|archetype| archetype.enemy.as_ref())
    {
        return milli_units_as_f32(enemy.action_milliseconds).max(0.1);
    }
    let base = if matches!(
        goal,
        AgentGoal::Attack(_)
            | AgentGoal::AttackBuilding(_)
            | AgentGoal::Construct(_)
            | AgentGoal::Gather(_)
            | AgentGoal::HarvestFarm(_)
            | AgentGoal::Heal(_)
            | AgentGoal::PlantTree(_)
            | AgentGoal::Prospect { .. }
            | AgentGoal::PlantBush(_)
    ) {
        effective_role_stats(content, simulation, actor).map_or(fallback, |stats| {
            milli_units_as_f32(stats.action_milliseconds).max(0.1)
        })
    } else {
        fallback
    };
    let mut rate_multiplier = if matches!(goal, AgentGoal::PlantTree(_) | AgentGoal::PlantBush(_)) {
        1.0
    } else {
        simulation.active_community_event.map_or(1.0, |event| {
            community_event_role_rate_multiplier(event, &actor.role)
        })
    };
    if simulation.active_community_event == Some(CommunityEvent::EconomicBoom)
        && matches!(goal, AgentGoal::Gather(_) | AgentGoal::HarvestFarm(_))
    {
        rate_multiplier *= 1.1;
    }
    let base = base / rate_multiplier.max(f32::EPSILON);
    if base <= f32::EPSILON {
        0.0
    } else {
        base.max(0.1)
    }
}

pub(crate) fn idle_wander_replan_delay(kind: &ActorKind, goal: &AgentGoal) -> Option<f32> {
    (*kind == ActorKind::Player && *goal == AgentGoal::Wander)
        .then_some(UNITY_IDLE_WANDER_INTERVAL_SECONDS)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ArrivedActionPhase {
    NonAction,
    Started,
    Waiting,
    Complete,
}

pub(crate) fn arrived_action_phase(agent: &mut Agent, authored_delay: f32) -> ArrivedActionPhase {
    if !agent.action_started {
        if authored_delay <= f32::EPSILON {
            return ArrivedActionPhase::NonAction;
        }
        agent.action_started = true;
        agent.action_cooldown_seconds = authored_delay;
        return ArrivedActionPhase::Started;
    }
    if agent.action_cooldown_seconds > f32::EPSILON {
        ArrivedActionPhase::Waiting
    } else {
        ArrivedActionPhase::Complete
    }
}

pub(crate) fn completed_player_gate_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> HashSet<GridPos> {
    let Some(gate) = content
        .buildings
        .get(&StableId::new("building:gate").expect("static ID"))
    else {
        return HashSet::new();
    };
    simulation
        .buildings
        .values()
        .filter(|building| building.complete && building.archetype == gate.archetype)
        .flat_map(|building| {
            let footprint = rotated_footprint(gate.footprint, building.rotation_quarter_turns);
            (0..footprint[1]).flat_map(move |z| {
                (0..footprint[0]).map(move |x| GridPos {
                    x: building.position.x.saturating_add(x),
                    z: building.position.z.saturating_add(z),
                })
            })
        })
        .collect()
}

pub(crate) fn completed_player_gate_navigation_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> HashSet<GridPos> {
    let gate_id = StableId::new("building:gate").expect("static ID");
    let Some(gate) = content.buildings.get(&gate_id) else {
        return HashSet::new();
    };
    simulation
        .buildings
        .values()
        .filter(|building| building.complete && building.archetype == gate.archetype)
        .flat_map(|building| {
            linear_navigation_cells(content, simulation, building, &gate_id).into_iter()
        })
        .collect()
}

#[cfg(test)]
pub(crate) fn agent_path(
    navigation: &stream_town_domain::NavGrid,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    kind: &ActorKind,
    start: GridPos,
    goal: GridPos,
) -> Vec<GridPos> {
    try_agent_path(navigation, content, simulation, kind, start, goal)
        .unwrap_or_else(|| vec![start])
}

pub(crate) fn try_agent_path(
    navigation: &stream_town_domain::NavGrid,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    kind: &ActorKind,
    start: GridPos,
    goal: GridPos,
) -> Option<Vec<GridPos>> {
    let path = if *kind == ActorKind::Player {
        navigation.find_path_with_exceptions(
            start,
            goal,
            &completed_player_gate_cells(content, simulation),
        )
    } else {
        navigation.find_path(start, goal)
    };
    path.ok()
}

pub(crate) fn fine_navigation_goal_candidates(placement: GridPos, start: GridPos) -> Vec<GridPos> {
    let base = GridPos {
        x: placement.x.saturating_mul(NAVIGATION_SUBDIVISIONS),
        z: placement.z.saturating_mul(NAVIGATION_SUBDIVISIONS),
    };
    let centre = placement_to_navigation_centre(placement);
    let mut candidates = (0..NAVIGATION_SUBDIVISIONS)
        .flat_map(|z| {
            (0..NAVIGATION_SUBDIVISIONS).map(move |x| GridPos {
                x: base.x.saturating_add(x),
                z: base.z.saturating_add(z),
            })
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|candidate| {
        (
            u8::from(*candidate != centre),
            candidate.x.abs_diff(start.x) + candidate.z.abs_diff(start.z),
            candidate.x.abs_diff(centre.x) + candidate.z.abs_diff(centre.z),
            candidate.z,
            candidate.x,
        )
    });
    candidates
}

pub(crate) fn try_fine_agent_path(
    navigation: &stream_town_domain::NavGrid,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    paths: &PathSurfaceRuntime,
    kind: &ActorKind,
    start: GridPos,
    placement_goal: GridPos,
) -> Option<Vec<GridPos>> {
    let mut exceptions = if *kind == ActorKind::Player {
        completed_player_gate_navigation_cells(content, simulation)
    } else {
        HashSet::new()
    };
    // A topology change can leave an actor standing in a newly blocked fine
    // cell. Let it leave that cell instead of waiting for the unstuck timeout.
    if navigation.contains(start) && !navigation.is_walkable(start) {
        exceptions.insert(start);
    }
    // Candidates are ordered centre-first. A walkable centre is only a
    // preference, not a requirement: walls can disconnect that third while a
    // side third in the same coarse approach cell remains reachable.
    let goals = fine_navigation_goal_candidates(placement_goal, start)
        .into_iter()
        .filter(|goal| {
            navigation.contains(*goal)
                && (navigation.is_walkable(*goal) || exceptions.contains(goal))
        })
        .collect::<Vec<_>>();
    if *kind == ActorKind::Player {
        let maximum_path_level = paths.levels.values().copied().max().unwrap_or_default();
        goals.into_iter().find_map(|goal| {
            navigation
                .find_path_with_exceptions_and_costs(
                    start,
                    goal,
                    &exceptions,
                    path_route_step_cost(10, maximum_path_level),
                    path_route_step_cost(14, maximum_path_level),
                    |position, base_cost| {
                        path_route_step_cost(
                            base_cost,
                            paths.levels.get(&position).copied().unwrap_or_default(),
                        )
                    },
                )
                .ok()
        })
    } else {
        goals.into_iter().find_map(|goal| {
            navigation
                .find_path_with_exceptions(start, goal, &exceptions)
                .ok()
        })
    }
}

pub(crate) fn try_fine_agent_path_for_goal(
    navigation: &stream_town_domain::NavGrid,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    paths: &PathSurfaceRuntime,
    enemy_navigation: Option<&EnemyNavigationField>,
    kind: &ActorKind,
    goal_kind: &AgentGoal,
    start: GridPos,
    placement_goal: GridPos,
) -> Option<Vec<GridPos>> {
    let goal = placement_to_navigation_centre(placement_goal);
    if *kind == ActorKind::Enemy
        && let AgentGoal::AttackBuilding(building) = goal_kind
        && let Some(path) = enemy_navigation.and_then(|field| field.path_to(start, building, goal))
    {
        return Some(path);
    }
    if *kind == ActorKind::Enemy
        && let Some(path) =
            enemy_navigation.and_then(|field| field.hierarchical_path(navigation, start, goal))
    {
        return Some(path);
    }
    try_fine_agent_path(
        navigation,
        content,
        simulation,
        paths,
        kind,
        start,
        placement_goal,
    )
}

pub(crate) fn agent_action_facing_grid(
    goal: &AgentGoal,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
) -> Option<GridPos> {
    match goal {
        AgentGoal::Gather(resource) => world
            .resources
            .iter()
            .find(|candidate| candidate.id == *resource && candidate.amount > 0)
            .map(|resource| resource.position),
        AgentGoal::HarvestFarm(building)
        | AgentGoal::AttackBuilding(building)
        | AgentGoal::Construct(building)
        | AgentGoal::VisitRegenerationStation(building) => simulation
            .buildings
            .get(building)
            .map(|building| building_visual_grid(content, building)),
        AgentGoal::Attack(actor) | AgentGoal::Heal(actor) => {
            simulation.actors.get(actor).map(|actor| actor.position)
        }
        AgentGoal::PlantTree(position) | AgentGoal::PlantBush(position) => Some(*position),
        AgentGoal::Prospect { cell, .. } => Some(*cell),
        AgentGoal::Deposit | AgentGoal::WaitForStorage | AgentGoal::Wander => None,
    }
}

pub(crate) fn rotate_agent_toward(
    transform: &mut Transform,
    target: Vec3,
    delta_seconds: f32,
    snap: bool,
    correct_player_axis: bool,
) {
    let direction = Vec3::new(
        target.x - transform.translation.x,
        0.0,
        target.z - transform.translation.z,
    );
    if direction.length_squared() <= f32::EPSILON {
        return;
    }
    // Unity character and enemy meshes visibly face local +Z. Bevy's
    // Transform::looking_to aligns local -Z, which made every actor walk and
    // act backwards even though transform.forward() appeared correct.
    let mut target_rotation = Quat::from_rotation_arc(Vec3::Z, direction.normalize());
    if correct_player_axis {
        target_rotation *= Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    }
    transform.rotation = if snap {
        target_rotation
    } else {
        transform.rotation.slerp(
            target_rotation,
            (delta_seconds * AGENT_ROTATION_SPEED).clamp(0.0, 1.0),
        )
    };
}

pub(crate) fn rotate_agent_toward_action(
    transform: &mut Transform,
    agent: &Agent,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    delta_seconds: f32,
    correct_player_axis: bool,
) -> bool {
    let Some(facing_grid) = agent_action_facing_grid(&agent.goal, content, simulation, world)
    else {
        return false;
    };
    let facing_target = if let AgentGoal::Gather(resource_id) = &agent.goal {
        world
            .resources
            .iter()
            .find(|resource| resource.id == *resource_id && resource.amount > 0)
            .map_or_else(
                || grid_to_world_on_surface(facing_grid, config, world),
                |resource| {
                    let position = generated_resource_world_position(resource, config, world);
                    resource_visual_archetype(content, &resource.kind).map_or(
                        position,
                        |archetype| {
                            centred_resource_visual_position(
                                position,
                                archetype,
                                config.world.cell_size,
                            )
                        },
                    )
                },
            )
    } else {
        grid_to_world_on_surface(facing_grid, config, world)
    };
    rotate_agent_toward(
        transform,
        facing_target,
        delta_seconds,
        matches!(agent.goal, AgentGoal::Gather(_)),
        correct_player_axis,
    );
    true
}

pub(crate) fn agent_path_world_target(
    next: GridPos,
    agent: &Agent,
    content: &ContentCatalog,
    world: &GeneratedWorld,
    config: &GameConfig,
) -> Vec3 {
    let mut target = navigation_to_world_on_surface(next, config, world);
    if next != placement_to_navigation_centre(agent.target) {
        return target;
    }
    let AgentGoal::Gather(resource_id) = &agent.goal else {
        return target;
    };
    let Some(resource) = world
        .resources
        .iter()
        .find(|resource| resource.id == *resource_id && resource.amount > 0)
    else {
        return target;
    };
    let visual = generated_resource_world_position(resource, config, world);
    let visual = resource_visual_archetype(content, &resource.kind).map_or(visual, |archetype| {
        centred_resource_visual_position(visual, archetype, config.world.cell_size)
    });
    let toward_resource = Vec2::new(visual.x - target.x, visual.z - target.z);
    let offset = toward_resource.normalize_or_zero()
        * (config.world.cell_size * 0.42).min(toward_resource.length() * 0.5);
    target.x += offset.x;
    target.z += offset.y;
    target
}

pub(crate) fn actor_movement_speed(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor_id: &StableId,
) -> f32 {
    simulation
        .actors
        .get(actor_id)
        .map_or(config.gameplay.agent_speed_cells_per_second, |actor| {
            effective_role_stats(content, simulation, actor)
                .map_or(config.gameplay.agent_speed_cells_per_second, |stats| {
                    milli_units_as_f32(stats.movement_speed_milli_cells_per_second)
                })
        })
}

pub(crate) fn actor_movement_speed_on_path(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    paths: &PathSurfaceRuntime,
    actor_id: &StableId,
    navigation_position: GridPos,
) -> f32 {
    let speed = actor_movement_speed(config, content, simulation, actor_id);
    let Some(_actor) = simulation
        .actors
        .get(actor_id)
        .filter(|actor| actor.role.as_str() != "role:enemy")
    else {
        return speed;
    };
    speed
        * (1.0
            + f32::from(
                paths
                    .levels
                    .get(&navigation_position)
                    .copied()
                    .unwrap_or_default(),
            ) * 0.05)
}

pub(crate) fn path_route_step_cost(base_cost: u32, path_level: u16) -> u32 {
    let speed_milli = 1_000_u32.saturating_add(u32::from(path_level).saturating_mul(50));
    base_cost
        .saturating_mul(100_000)
        .saturating_add(speed_milli.saturating_sub(1))
        / speed_milli
}

pub(crate) fn predictive_speed_factors(
    agents: &[(StableId, Vec2, Vec2)],
    radius: f32,
    horizon_seconds: f32,
    minimum_factor: f32,
) -> BTreeMap<StableId, f32> {
    let mut factors = agents
        .iter()
        .map(|(id, _, _)| (id.clone(), 1.0_f32))
        .collect::<BTreeMap<_, _>>();
    if radius <= f32::EPSILON || horizon_seconds <= f32::EPSILON {
        return factors;
    }
    for left_index in 0..agents.len() {
        let (left_id, left_position, left_velocity) = &agents[left_index];
        for (right_id, right_position, right_velocity) in &agents[left_index + 1..] {
            let relative_position = *right_position - *left_position;
            let relative_velocity = *left_velocity - *right_velocity;
            let relative_speed_squared = relative_velocity.length_squared();
            if relative_speed_squared <= f32::EPSILON {
                continue;
            }
            let approach = relative_position.dot(relative_velocity);
            if approach <= 0.0 {
                continue;
            }
            let closest_seconds = (approach / relative_speed_squared).min(horizon_seconds);
            let closest_offset = relative_position - relative_velocity * closest_seconds;
            let closest_distance = closest_offset.length();
            if closest_distance >= radius {
                continue;
            }
            let time_urgency = 1.0 - closest_seconds / horizon_seconds;
            let distance_urgency = 1.0 - closest_distance / radius;
            let factor = (1.0 - time_urgency * distance_urgency * (1.0 - minimum_factor))
                .clamp(minimum_factor, 1.0);

            // A following agent yields to the one already ahead. At crossings
            // and head-on encounters, the stable actor ID is the deterministic
            // right-of-way tie-breaker, preventing reciprocal stop/start.
            let same_direction = left_velocity.dot(*right_velocity) > 0.0;
            let left_is_behind =
                same_direction && relative_position.dot(left_velocity.normalize_or_zero()) > 0.0;
            let right_is_behind = same_direction
                && (-relative_position).dot(right_velocity.normalize_or_zero()) > 0.0;
            let yielding = if left_is_behind && !right_is_behind {
                left_id
            } else if right_is_behind && !left_is_behind {
                right_id
            } else if left_id > right_id {
                left_id
            } else {
                right_id
            };
            factors
                .entry(yielding.clone())
                .and_modify(|current| *current = current.min(factor));
        }
    }
    factors
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn return_agent_to_town_hall(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    config: &GameConfig,
    world: &GeneratedWorld,
    navigation: &stream_town_domain::NavGrid,
    occupied: &mut BTreeMap<GridPos, StableId>,
    agent: &mut Agent,
    location: &mut GridLocation,
    animation: &AgentAnimation,
    transform: &mut Transform,
) {
    let town_hall = restored_town_hall_position(content, simulation, config);
    let desired_navigation = placement_to_navigation_centre(town_hall);
    let spawn_navigation =
        fine_navigation_recovery_position(navigation, desired_navigation, &agent.id, occupied)
            .unwrap_or(desired_navigation);
    let spawn = navigation_to_placement(spawn_navigation);
    if occupied.get(&location.0) == Some(&agent.id) {
        occupied.remove(&location.0);
    }
    occupied.insert(spawn, agent.id.clone());
    if let Some(actor) = simulation.actors.get_mut(&agent.id) {
        actor.position = spawn;
        actor.preferred_target = None;
    }
    let mut world_position = navigation_to_world_on_surface(spawn_navigation, config, world);
    if !animation.native {
        world_position.y += animation.base_scale.y * 0.5;
    }
    transform.translation = world_position;
    location.0 = spawn;
    agent.origin = spawn;
    agent.navigation_position = spawn_navigation;
    agent.target = spawn;
    agent.goal = AgentGoal::Wander;
    agent.path.clear();
    agent.path_index = 0;
    agent.action_started = false;
    agent.action_cooldown_seconds = 0.0;
    agent.repath_remaining_seconds = 0.0;
    agent.previous_wander_origin = None;
}

pub(crate) fn navigation_component_reaches_minimum(
    navigation: &stream_town_domain::NavGrid,
    start: GridPos,
    minimum_cells: usize,
) -> bool {
    if !navigation.is_walkable(start) {
        return false;
    }
    let mut visited = HashSet::from([start]);
    let mut queue = VecDeque::from([start]);
    while let Some(position) = queue.pop_front() {
        if visited.len() >= minimum_cells {
            return true;
        }
        for (neighbour, _) in navigation
            .walkable_neighbours(position)
            .into_iter()
            .flatten()
        {
            if visited.insert(neighbour) {
                queue.push_back(neighbour);
            }
        }
    }
    visited.len() >= minimum_cells
}

pub(crate) fn fine_navigation_recovery_position(
    navigation: &stream_town_domain::NavGrid,
    desired: GridPos,
    actor: &StableId,
    occupied: &BTreeMap<GridPos, StableId>,
) -> Option<GridPos> {
    const MINIMUM_SAFE_COMPONENT_CELLS: usize = 64;
    if navigation.width() == 0 || navigation.height() == 0 {
        return None;
    }
    let desired = GridPos {
        x: desired.x.min(navigation.width() - 1),
        z: desired.z.min(navigation.height() - 1),
    };
    let index = |position: GridPos| {
        usize::from(position.z) * usize::from(navigation.width()) + usize::from(position.x)
    };
    let mut visited =
        vec![false; usize::from(navigation.width()) * usize::from(navigation.height())];
    visited[index(desired)] = true;
    let mut queue = VecDeque::from([desired]);
    let mut fallback = None;
    while let Some(position) = queue.pop_front() {
        let placement = navigation_to_placement(position);
        let available = navigation.is_walkable(position)
            && occupied.get(&placement).is_none_or(|owner| owner == actor);
        if available {
            fallback.get_or_insert(position);
            if navigation_component_reaches_minimum(
                navigation,
                position,
                MINIMUM_SAFE_COMPONENT_CELLS,
            ) {
                return Some(position);
            }
        }
        for neighbour in [
            position
                .x
                .checked_sub(1)
                .map(|x| GridPos { x, z: position.z }),
            position
                .x
                .checked_add(1)
                .filter(|x| *x < navigation.width())
                .map(|x| GridPos { x, z: position.z }),
            position
                .z
                .checked_sub(1)
                .map(|z| GridPos { x: position.x, z }),
            position
                .z
                .checked_add(1)
                .filter(|z| *z < navigation.height())
                .map(|z| GridPos { x: position.x, z }),
        ]
        .into_iter()
        .flatten()
        {
            let neighbour_index = index(neighbour);
            if !visited[neighbour_index] {
                visited[neighbour_index] = true;
                queue.push_back(neighbour);
            }
        }
    }
    fallback
}
