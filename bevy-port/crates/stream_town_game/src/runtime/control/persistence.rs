pub(crate) fn apply_town_persistence_requests(
    mut requests: MessageReader<TownPersistenceRequest>,
    mut io: ResMut<MenuIoRequest>,
) {
    for request in requests.read() {
        match request {
            TownPersistenceRequest::Save => io.save = true,
            TownPersistenceRequest::SaveJumpStart => io.save_jump_start = true,
            TownPersistenceRequest::Load(source) => {
                io.load = true;
                io.load_source.clone_from(source);
            }
        }
    }
}

pub(crate) fn save_input(
    mut io: ResMut<MenuIoRequest>,
    save: Res<SaveRuntime>,
    world: Res<WorldRuntime>,
    stats: Res<SessionStats>,
    simulation: Res<SimulationRuntime>,
    traversal_wear: Res<TraversalWearRuntime>,
    mut runtime_console: ResMut<RuntimeConsoleRuntime>,
) {
    let save_requested = std::mem::take(&mut io.save);
    let jump_start_requested = std::mem::take(&mut io.save_jump_start);
    if !save_requested && !jump_start_requested {
        return;
    }
    let snapshot = snapshot_world(&world, &stats, &simulation, &traversal_wear);
    let destination = if jump_start_requested {
        NativeSaveStore::new(jump_start_snapshot_path(save.store.path()))
    } else {
        NativeSaveStore::new(save.store.path())
    };
    match destination.write(&snapshot) {
        Ok(()) => {
            runtime_console.last_result = if jump_start_requested {
                format!(
                    "Protected jump-start saved to {}",
                    destination.path().display()
                )
            } else {
                format!("Saved {}", destination.path().display())
            };
            info!(
                path = %destination.path().display(),
                protected = jump_start_requested,
                "native save written"
            );
        }
        Err(error) => {
            runtime_console.last_result = format!("Save failed: {error}");
            error!(%error, "native save failed");
        }
    }
}

pub(crate) fn autosave_game(
    time: Res<Time>,
    mut settings: ResMut<PlayerSettingsRuntime>,
    player_settings: Res<RuntimePlayerSettings>,
    save: Res<SaveRuntime>,
    world: Res<WorldRuntime>,
    stats: Res<SessionStats>,
    simulation: Res<SimulationRuntime>,
    traversal_wear: Res<TraversalWearRuntime>,
) {
    if player_settings.0.autosave_minutes == 0 {
        settings.autosave_elapsed_seconds = 0.0;
        return;
    }
    settings.autosave_elapsed_seconds += time.delta_secs();
    let interval = f32::from(player_settings.0.autosave_minutes) * 60.0;
    if settings.autosave_elapsed_seconds < interval {
        return;
    }
    settings.autosave_elapsed_seconds = settings.autosave_elapsed_seconds.rem_euclid(interval);
    let snapshot = snapshot_world(&world, &stats, &simulation, &traversal_wear);
    match save.store.write(&snapshot) {
        Ok(()) => info!(path = %save.store.path().display(), "autosave written"),
        Err(error) => error!(%error, "autosave failed"),
    }
}

pub(crate) fn save_on_gameplay_exit(
    save: Res<SaveRuntime>,
    restart: Res<TownRestartRuntime>,
    ready: Option<Res<GameplayReady>>,
    world: Option<Res<WorldRuntime>>,
    stats: Option<Res<SessionStats>>,
    simulation: Option<Res<SimulationRuntime>>,
    traversal_wear: Option<Res<TraversalWearRuntime>>,
) {
    if restart.suppress_exit_save || ready.is_none() {
        return;
    }
    let (Some(world), Some(stats), Some(simulation), Some(traversal_wear)) =
        (world, stats, simulation, traversal_wear)
    else {
        return;
    };
    let snapshot = snapshot_world(&world, &stats, &simulation, &traversal_wear);
    match save.store.write(&snapshot) {
        Ok(()) => info!(path = %save.store.path().display(), "gameplay-exit save written"),
        Err(error) => error!(%error, "gameplay-exit save failed"),
    }
}

pub(crate) fn automatic_load_requested(
    automatic_complete: bool,
    compatibility_load: bool,
    resume: bool,
) -> bool {
    !automatic_complete && (compatibility_load || resume)
}

pub(crate) fn upgrade_path_navigation_positions(simulation: &mut WorldSimulation) -> usize {
    const LOCAL_CANDIDATES: [(u16, u16); 9] = [
        (1, 1),
        (1, 0),
        (2, 1),
        (1, 2),
        (0, 1),
        (2, 0),
        (2, 2),
        (0, 2),
        (0, 0),
    ];
    let mut occupied = simulation
        .path_navigation_positions
        .values()
        .copied()
        .collect::<BTreeSet<_>>();
    let coarse_paths = simulation
        .buildings
        .values()
        .filter(|building| building.archetype.as_str() == "archetype:building:path")
        .filter(|building| {
            !simulation
                .path_navigation_positions
                .contains_key(&building.id)
        })
        .map(|building| (building.id.clone(), building.position))
        .collect::<Vec<_>>();
    let mut upgraded = 0;
    for (id, coarse) in coarse_paths {
        let base_x = coarse.x.saturating_mul(NAVIGATION_SUBDIVISIONS);
        let base_z = coarse.z.saturating_mul(NAVIGATION_SUBDIVISIONS);
        let Some(position) = LOCAL_CANDIDATES
            .iter()
            .map(|(x, z)| GridPos {
                x: base_x.saturating_add(*x),
                z: base_z.saturating_add(*z),
            })
            .find(|position| !occupied.contains(position))
        else {
            continue;
        };
        occupied.insert(position);
        simulation.path_navigation_positions.insert(id, position);
        upgraded += 1;
    }
    upgraded
}

pub(crate) fn load_input(
    mut ecs: Commands,
    mut io: ResMut<MenuIoRequest>,
    save: Res<SaveRuntime>,
    mut automatic_resume: ResMut<AutomaticResumeRuntime>,
    mut world: ResMut<WorldRuntime>,
    mut config: ResMut<RuntimeConfig>,
    content: Res<RuntimeContent>,
    mut load_render: LoadRenderParams,
    mut placers: ResMut<BuildingPlacers>,
    mut stats: ResMut<SessionStats>,
    mut simulation: ResMut<SimulationRuntime>,
    mut traversal_wear: ResMut<TraversalWearRuntime>,
    mut selected: ResMut<SelectedCell>,
    mut entities: LoadWorldEntities,
    mut automatic_complete: Local<bool>,
    mut runtime_console: ResMut<RuntimeConsoleRuntime>,
) {
    let automatic = automatic_load_requested(
        *automatic_complete,
        std::env::var_os("STREAM_TOWN_AUTO_LOAD").is_some(),
        automatic_resume.path.is_some(),
    );
    let requested = std::mem::take(&mut io.load);
    let requested_source = io.load_source.take();
    if !automatic && !requested {
        return;
    }
    *automatic_complete = true;
    let source_store = requested_source.as_ref().map_or_else(
        || NativeSaveStore::new(save.store.path()),
        NativeSaveStore::new,
    );
    let source_path = source_store.path().to_path_buf();
    let mut snapshot = match source_store.load() {
        Ok(snapshot) => snapshot,
        Err(error) => {
            runtime_console.last_result = format!("Load failed: {error}");
            error!(%error, "native load failed");
            return;
        }
    };
    let mut persisted_upgrade = snapshot.simulation.schema_version < CURRENT_SIMULATION_SCHEMA;
    selected.0 = None;
    ecs.insert_resource(SelectedActor::default());
    let mut restored_config = config.0.clone();
    restored_config.world.seed = snapshot.world_seed;
    let mut restored_world = generate_world_with_content(&restored_config.world, &content.0);
    let saved_generator_version = snapshot.generator_version;
    let compatibility = native_world_compatibility(
        snapshot.world_seed,
        snapshot.generator_version,
        &snapshot.world_hash,
        &restored_world,
    );
    if compatibility.is_none() {
        runtime_console.last_result = format!(
            "Load failed: saved world {} does not match generator {}",
            snapshot.world_hash, restored_world.deterministic_hash
        );
        error!(
            saved_seed = snapshot.world_seed,
            runtime_seed = world.generated.seed,
            "native save world identity does not match the loaded world"
        );
        return;
    }
    if matches!(
        compatibility,
        Some(
            NativeWorldCompatibility::UpgradeV1
                | NativeWorldCompatibility::UpgradeV2
                | NativeWorldCompatibility::UpgradeV3
                | NativeWorldCompatibility::RegeneratePrior
        )
    ) {
        persisted_upgrade = true;
        info!(
            saved_generator_version = snapshot.generator_version,
            runtime_generator_version = restored_world.generator_version,
            "upgrading native save world fingerprint and proportionally migrating resource stock"
        );
        snapshot.generator_version = restored_world.generator_version;
        snapshot
            .world_hash
            .clone_from(&restored_world.deterministic_hash);
    }

    if !snapshot.resource_nodes.is_empty() {
        let restored_regrowth = snapshot
            .resource_nodes
            .iter()
            .filter(|(id, _)| {
                !restored_world
                    .resources
                    .iter()
                    .any(|resource| resource.id == **id)
            })
            .filter_map(|(id, amount)| restored_regenerated_resource(id, *amount))
            .filter(|resource| {
                resource.position.x < restored_world.navigation.width()
                    && resource.position.z < restored_world.navigation.height()
            })
            .collect::<Vec<_>>();
        restored_world.resources.extend(restored_regrowth);
        let mut depleted_land = Vec::new();
        for resource in &mut restored_world.resources {
            if let Some(remaining) = snapshot.resource_nodes.get(&resource.id) {
                resource.amount =
                    upgraded_resource_remaining(saved_generator_version, resource, *remaining);
                if resource.amount == 0 && resource.target_kind.as_str() != "target:fish" {
                    depleted_land.push(resource.position);
                }
            }
        }
        for position in depleted_land {
            if !resource_cell_has_active_generation_occupant(&restored_world.resources, position) {
                let _ = restored_world.navigation.set_blocked(
                    stream_town_domain::DirtyRegion {
                        min: position,
                        max: position,
                    },
                    false,
                );
            }
        }
        for resource in restored_world.resources.iter().filter(|resource| {
            resource.amount > 0 && resource.id.as_str().starts_with("resource:regrown_")
        }) {
            let _ = restored_world.navigation.set_blocked(
                stream_town_domain::DirtyRegion {
                    min: resource.position,
                    max: resource.position,
                },
                true,
            );
        }
    }
    ensure_town_hall_state(&content.0, &restored_config, &mut snapshot.simulation);
    let town_hall_id = StableId::new("building:townhall").expect("static ID");
    let cleared_trees = clear_seeded_trees_under_building(
        &content.0,
        &snapshot.simulation,
        &mut restored_world,
        &town_hall_id,
    );
    if cleared_trees > 0 {
        persisted_upgrade = true;
        info!(
            cleared_trees,
            "cleared saved seeded trees beneath the Town Hall"
        );
    }
    snapshot
        .simulation
        .upgrade_time_schema(restored_config.time.seconds_per_day);
    let upgraded_paths = upgrade_path_navigation_positions(&mut snapshot.simulation);
    if upgraded_paths > 0 {
        persisted_upgrade = true;
        info!(
            paths = upgraded_paths,
            "upgraded coarse saved paths onto the fine navigation grid"
        );
    }
    normalize_building_health(&content.0, &mut snapshot.simulation);
    let capped_recruit_progress =
        normalize_recruit_role_progression(&content.0, &mut snapshot.simulation);
    if capped_recruit_progress > 0 {
        persisted_upgrade = true;
        info!(
            adjusted_professions = capped_recruit_progress,
            maximum_level = content.0.progression.maximum_recruit_role_level,
            "capped loaded recruit profession levels"
        );
    }
    if entities.town_halls.single_mut().is_err() {
        "Load failed: the persistent Town Hall visual is unavailable"
            .clone_into(&mut runtime_console.last_result);
        error!("the persistent Town Hall visual is unavailable during native load preflight");
        return;
    }
    for saved in snapshot.simulation.buildings.values() {
        let Some((building_id, building)) = content
            .0
            .buildings
            .iter()
            .find(|(_, building)| building.archetype == saved.archetype)
        else {
            runtime_console.last_result = format!(
                "Load failed: building {} references unknown archetype {}",
                saved.id, saved.archetype
            );
            error!(
                building = %saved.id,
                archetype = %saved.archetype,
                "native save references an unknown building archetype"
            );
            return;
        };
        if !content.0.archetypes.contains_key(&building.archetype) {
            runtime_console.last_result = format!(
                "Load failed: building definition {} references unknown prefab archetype {}",
                building_id, building.archetype
            );
            error!(
                building = %saved.id,
                definition = %building_id,
                archetype = %building.archetype,
                "native save building definition references an unknown prefab archetype"
            );
            return;
        }
        let Some(region) = building_navigation_region(
            saved.position,
            building,
            saved.rotation_quarter_turns,
            &restored_world,
        ) else {
            runtime_console.last_result =
                format!("Load failed: building {} lies outside the world", saved.id);
            error!(building = %saved.id, "native save building lies outside the world");
            return;
        };
        if building_blocks_navigation(building)
            && let Err(error) = restored_world.navigation.set_blocked(region, true)
        {
            runtime_console.last_result = format!(
                "Load failed: building {} cannot update navigation",
                saved.id
            );
            error!(building = %saved.id, %error, "native save building could not update navigation");
            return;
        }
    }
    let repaired_enemy_camps = seed_generated_enemy_camps(
        &restored_config,
        &content.0,
        &mut restored_world,
        &mut snapshot.simulation,
    );
    if repaired_enemy_camps > 0 {
        persisted_upgrade = true;
        info!(
            camps = repaired_enemy_camps,
            "repaired native save created before enemy camp generation was enabled"
        );
    }
    for camp in snapshot.simulation.enemy_camps.values() {
        let Some(archetype) = content.0.archetypes.get(&camp.archetype) else {
            runtime_console.last_result = format!(
                "Load failed: enemy camp {} references unknown archetype {}",
                camp.id, camp.archetype
            );
            error!(camp = %camp.id, archetype = %camp.archetype, "native save references an unknown enemy camp");
            return;
        };
        let Some(region) =
            enemy_camp_navigation_region(camp.position, archetype.footprint, &restored_world)
        else {
            runtime_console.last_result =
                format!("Load failed: enemy camp {} lies outside the world", camp.id);
            error!(camp = %camp.id, "native save enemy camp lies outside the world");
            return;
        };
        if let Err(error) = restored_world.navigation.set_blocked(region, true) {
            runtime_console.last_result = format!(
                "Load failed: enemy camp {} cannot update navigation",
                camp.id
            );
            error!(camp = %camp.id, %error, "native save enemy camp could not update navigation");
            return;
        }
    }

    let terrain_replacement = if let Some(meshes) = load_render.meshes.as_mut() {
        let terrain_meshes = generated_terrain_chunks(&restored_world, &restored_config)
            .into_iter()
            .map(|chunk| {
                (
                    format!("TerrainChunk_{}_{}", chunk.chunk_x, chunk.chunk_z),
                    chunk.high,
                    Some((chunk.centre, chunk.medium, chunk.low)),
                )
            })
            .collect::<Vec<_>>();
        let mut replacements = Vec::with_capacity(terrain_meshes.len());
        for (name, mesh, lod_meshes) in terrain_meshes {
            let Some(collider) = Collider::trimesh_from_mesh(&mesh) else {
                "Load failed: saved terrain does not produce a valid collider"
                    .clone_into(&mut runtime_console.last_result);
                error!("native save terrain does not produce a valid triangle collider");
                return;
            };
            let high = meshes.add(mesh);
            let lod = lod_meshes.map(|(centre, medium, low)| TerrainChunkLod {
                centre,
                high: high.clone(),
                medium: meshes.add(medium),
                low: meshes.add(low),
                current: TerrainLodLevel::High,
            });
            replacements.push((name, high, lod, collider));
        }
        Some(replacements)
    } else {
        None
    };
    let water_replacement = load_render
        .meshes
        .as_mut()
        .map(|meshes| meshes.add(generated_water_mesh(&restored_world, &restored_config)));

    placers.0.clear();
    for (entity, building) in &entities.runtime_buildings {
        debug!(building = %building.id, "despawning runtime building before native load");
        ecs.entity(entity).despawn();
    }
    for (entity, camp) in &entities.enemy_camps {
        debug!(camp = %camp.id, "despawning enemy camp before native load");
        ecs.entity(entity).despawn();
    }
    for saved in snapshot.simulation.buildings.values() {
        let (building_id, building) = content
            .0
            .buildings
            .iter()
            .find(|(_, building)| building.archetype == saved.archetype)
            .expect("building references were checked during native load preflight");
        if saved.id.as_str() == "building:townhall" {
            continue;
        }
        spawn_runtime_building(
            &mut ecs,
            &restored_config,
            &restored_world,
            &load_render.presentation.0,
            load_render.asset_server.as_deref(),
            &load_render.asset_root.0,
            &load_render.render,
            saved,
            building,
            content
                .0
                .archetypes
                .get(&building.archetype)
                .expect("building prefab was checked during native load preflight"),
            saved.position,
            building.footprint,
            building_age(&content.0, &snapshot.simulation, building_id, saved.level),
        );
    }
    for camp in snapshot.simulation.enemy_camps.values() {
        let archetype = content
            .0
            .archetypes
            .get(&camp.archetype)
            .expect("enemy camp archetype was checked during native load preflight");
        spawn_enemy_camp(
            &mut ecs,
            &restored_config,
            &restored_world,
            &load_render.presentation.0,
            load_render.asset_server.as_deref(),
            &load_render.asset_root.0,
            &load_render.render,
            &camp.id,
            archetype,
            camp.position,
        );
    }
    if let Some(replacements) = terrain_replacement {
        for entity in &load_render.terrain_surfaces {
            ecs.entity(entity).despawn();
        }
        for (name, mesh, lod, collider) in replacements {
            let mut terrain = ecs.spawn((
                WorldEntity,
                TerrainSurface,
                Name::new(name),
                Mesh3d(mesh),
                MeshMaterial3d(load_render.render.ground.clone()),
                collider,
                RigidBody::Static,
            ));
            if let Some(lod) = lod {
                terrain.insert(lod);
            }
        }
    }
    if let Some(water_mesh) = water_replacement {
        for entity in &load_render.water_surfaces {
            ecs.entity(entity).despawn();
        }
        ecs.spawn((
            WorldEntity,
            WaterSurface,
            Mesh3d(water_mesh),
            MeshMaterial3d(load_render.render.water.clone()),
        ));
    }
    for entity in &entities.world_fish_schools {
        ecs.entity(entity).despawn();
    }
    spawn_fish_school_scene(
        &mut ecs,
        &load_render.presentation.0,
        &load_render.render,
        WORLD_SCENE_PATH,
        restored_world.seed,
        f32::from(restored_config.world.water_level_centimetres) * 0.01,
        false,
        None,
        Some((&restored_world, &restored_config)),
    );
    // Manual/native loads may replace the generated height field after these
    // visuals have already resolved their AABBs. Force every foliage/resource
    // renderer back through the final-height grounding and habitat gate.
    for entity in &entities.surface_visuals {
        ecs.entity(entity).insert((
            PendingSurfaceGrounding {
                surface_height: 0.0,
            },
            Visibility::Hidden,
        ));
    }
    let town_hall_id = StableId::new("building:townhall").expect("static ID");
    let saved_town_hall = &snapshot.simulation.buildings[&town_hall_id];
    let town_hall_definition = &content.0.buildings[&town_hall_id];
    let town_hall_footprint = rotated_footprint(
        town_hall_definition.footprint,
        saved_town_hall.rotation_quarter_turns,
    );
    let town_hall_centre = GridPos {
        x: saved_town_hall.position.x + town_hall_footprint[0] / 2,
        z: saved_town_hall.position.z + town_hall_footprint[1] / 2,
    };
    let town_hall_surface =
        grid_to_world_on_surface(town_hall_centre, &restored_config, &restored_world);
    if let Ok((mut location, mut presentation, mut transform)) = entities.town_halls.single_mut() {
        location.0 = saved_town_hall.position;
        presentation.base_translation =
            town_hall_surface + Vec3::Y * presentation.base_height_offset;
        transform.translation = presentation.base_translation;
        transform.rotation = quarter_turn_rotation(saved_town_hall.rotation_quarter_turns);
    } else {
        error!("the persistent Town Hall visual is unavailable during native load");
        return;
    }
    world.generated = restored_world;

    let saved_by_id: BTreeMap<StableId, SavedActor> = snapshot
        .actors
        .iter()
        .filter(|actor| matches!(actor.kind, ActorKind::Player | ActorKind::Enemy))
        .cloned()
        .map(|actor| (actor.id.clone(), actor))
        .collect();
    let mut restored_ids = BTreeSet::new();
    for (entity, mut agent, mut location, animation, mut transform) in &mut entities.agents {
        let Some(saved) = saved_by_id.get(&agent.id) else {
            ecs.entity(entity).despawn();
            continue;
        };
        let position = restored_actor_position(
            &world.generated,
            &content.0,
            &snapshot.simulation,
            &saved.kind,
            saved.grid_position,
        )
        .unwrap_or(saved.grid_position);
        if let Some(actor) = snapshot.simulation.actors.get_mut(&saved.id) {
            actor.position = position;
        }
        let mut world_position =
            grid_to_world_on_surface(position, &restored_config, &world.generated);
        if !animation.native {
            world_position.y += animation.base_scale.y * 0.5;
        }
        agent.kind = saved.kind.clone();
        agent.archetype = saved.archetype.clone();
        agent.goal = AgentGoal::Wander;
        agent.action_started = false;
        agent.spawn = position;
        agent.origin = position;
        agent.navigation_position = placement_to_navigation_centre(position);
        agent.path.clear();
        agent.path_index = 0;
        agent.target = deterministic_wander_target(&world.generated, &agent.id, position);
        agent.action_cooldown_seconds = 0.0;
        agent.action_started = false;
        agent.repath_remaining_seconds = 0.0;
        agent.wander_sequence = 0;
        agent.previous_wander_origin = None;
        location.0 = position;
        transform.translation = world_position;
        restored_ids.insert(saved.id.clone());
    }

    for saved in saved_by_id.values() {
        if restored_ids.contains(&saved.id) {
            continue;
        }
        let position = restored_actor_position(
            &world.generated,
            &content.0,
            &snapshot.simulation,
            &saved.kind,
            saved.grid_position,
        )
        .unwrap_or(saved.grid_position);
        if let Some(actor) = snapshot.simulation.actors.get_mut(&saved.id) {
            actor.position = position;
        }
        if saved.kind == ActorKind::Enemy {
            spawn_runtime_enemy_entity(
                &mut ecs,
                &restored_config,
                &world.generated,
                &content.0,
                &load_render.presentation.0,
                load_render.asset_server.as_deref(),
                &load_render.asset_root.0,
                &load_render.render,
                &saved.id,
                saved.archetype.clone(),
                position,
            );
            continue;
        }
        let world_position = grid_to_world_on_surface(position, &restored_config, &world.generated);
        let base_scale = Vec3::new(
            restored_config.world.cell_size * 0.3,
            restored_config.world.cell_size * 0.55,
            restored_config.world.cell_size * 0.3,
        );
        ecs.spawn((
            WorldEntity,
            GridLocation(position),
            Agent {
                id: saved.id.clone(),
                kind: saved.kind.clone(),
                archetype: saved.archetype.clone(),
                goal: AgentGoal::Wander,
                spawn: position,
                origin: position,
                navigation_position: placement_to_navigation_centre(position),
                path: Vec::new(),
                path_index: 0,
                target: deterministic_wander_target(&world.generated, &saved.id, position),
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
                ..default()
            },
            Mesh3d(load_render.render.actor_lod.clone()),
            MeshMaterial3d(actor_material(&load_render.render, &saved.kind, false)),
            Transform::from_xyz(
                world_position.x,
                world_position.y + base_scale.y * 0.5,
                world_position.z,
            )
            .with_scale(base_scale),
        ));
    }
    stats.elapsed_seconds = Duration::from_secs(snapshot.elapsed_seconds).as_secs_f64();
    stats.paths_completed = 0;
    traversal_wear.restore(
        &snapshot.traversal_wear,
        world.generated.navigation.width(),
        world.generated.navigation.height(),
        &config.0.terrain,
    );
    let forked_from_template = source_path != save.store.path();
    if forked_from_template || persisted_upgrade {
        match save.store.write(&snapshot) {
            Ok(()) => {
                if forked_from_template {
                    info!(
                        source = %source_path.display(),
                        destination = %save.store.path().display(),
                        "jump-start template forked to writable town save"
                    );
                } else {
                    info!(
                        destination = %save.store.path().display(),
                        "persisted native save schema and content migrations"
                    );
                }
            }
            Err(error) => error!(
                %error,
                destination = %save.store.path().display(),
                "loaded native save but could not persist its migrated town state"
            ),
        }
    }
    simulation.0 = snapshot.simulation;
    config.0.world.seed = snapshot.world_seed;
    if automatic_resume.path.is_some() {
        automatic_resume.applied = true;
    }
    runtime_console.last_result = if forked_from_template {
        format!(
            "Loaded protected {} as {}",
            source_path.display(),
            save.store.path().display()
        )
    } else {
        format!("Loaded {}", source_path.display())
    };
    info!(path = %source_path.display(), "native save loaded and applied");
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NativeWorldCompatibility {
    Current,
    UpgradeV1,
    UpgradeV2,
    UpgradeV3,
    RegeneratePrior,
}

pub(crate) fn native_world_compatibility(
    seed: u64,
    generator_version: u32,
    world_hash: &str,
    world: &GeneratedWorld,
) -> Option<NativeWorldCompatibility> {
    if seed != world.seed {
        return None;
    }
    if generator_version == world.generator_version && world_hash == world.deterministic_hash {
        return Some(NativeWorldCompatibility::Current);
    }
    if generator_version == 1 && world_hash == stream_town_domain::legacy_v1_world_hash(world) {
        return Some(NativeWorldCompatibility::UpgradeV1);
    }
    if generator_version == 2 && world_hash == stream_town_domain::legacy_v2_world_hash(world) {
        return Some(NativeWorldCompatibility::UpgradeV2);
    }
    if generator_version == 3 && world_hash == stream_town_domain::legacy_v3_world_hash(world) {
        return Some(NativeWorldCompatibility::UpgradeV3);
    }
    // NativeSaveStore has already verified the envelope checksum and snapshot
    // schema before this point. Versions 4+ changed terrain/resource topology
    // too substantially to reproduce their old fingerprints from the current
    // generator, so regenerate the seeded world and use the existing stable-ID
    // restoration/relocation path instead of rejecting a valid native save.
    (generator_version >= 4 && generator_version < world.generator_version)
        .then_some(NativeWorldCompatibility::RegeneratePrior)
}
