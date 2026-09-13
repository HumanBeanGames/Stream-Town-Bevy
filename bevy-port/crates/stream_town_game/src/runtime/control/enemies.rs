pub(crate) fn find_building_site(
    world: &GeneratedWorld,
    near: GridPos,
    footprint: [u16; 2],
) -> Option<GridPos> {
    let mut candidates = Vec::new();
    for z in 0..world.navigation.height() {
        for x in 0..world.navigation.width() {
            let position = GridPos { x, z };
            if building_site_is_available(world, position, footprint) {
                candidates.push(position);
            }
        }
    }
    candidates.sort_by_key(|position| {
        (
            position.x.abs_diff(near.x) + position.z.abs_diff(near.z),
            position.z,
            position.x,
        )
    });
    candidates.into_iter().next()
}

pub(crate) const ENEMY_CAMP_PLACEMENT_ATTEMPTS: u64 = 500;

pub(crate) fn generated_enemy_camp_hash(seed: u64, layer: &StableId, serial: u64) -> u64 {
    let mut mixed = layer.as_str().bytes().fold(seed, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    });
    mixed = mixed.wrapping_add(serial.wrapping_mul(0x9e37_79b9_7f4a_7c15));
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    mixed ^ (mixed >> 31)
}

pub(crate) fn rounded_milli_cells(value: u32) -> i32 {
    i32::try_from(value.saturating_add(500) / 1_000).unwrap_or(i32::MAX)
}

pub(crate) fn enemy_camp_candidate(
    config: &GameConfig,
    layer: &EnemyCampGenerationDef,
    footprint: [u16; 2],
    camp_index: u16,
    attempt: u64,
) -> Option<(GridPos, GridPos)> {
    let serial = u64::from(camp_index)
        .saturating_mul(ENEMY_CAMP_PLACEMENT_ATTEMPTS)
        .saturating_add(attempt);
    let random = generated_enemy_camp_hash(config.world.seed, &layer.id, serial);
    let sample_axis = |axis: usize, bits: u32| -> i32 {
        let minimum = rounded_milli_cells(layer.minimum_absolute_offset_milli_cells[axis]);
        let maximum = rounded_milli_cells(layer.maximum_absolute_offset_milli_cells[axis]);
        let span = u64::try_from(maximum.saturating_sub(minimum))
            .unwrap_or_default()
            .saturating_add(1);
        let magnitude = minimum.saturating_add(
            i32::try_from((random >> bits) % span).expect("camp offset span fits i32"),
        );
        if (random >> (bits + 15)) & 1 == 0 {
            -magnitude
        } else {
            magnitude
        }
    };
    let map_centre = GridPos {
        x: config.world.width / 2,
        z: config.world.height / 2,
    };
    let centre_x = i32::from(map_centre.x).saturating_add(sample_axis(0, 0));
    let centre_z = i32::from(map_centre.z).saturating_add(sample_axis(1, 32));
    let origin_x = centre_x.saturating_sub(i32::from(footprint[0] / 2));
    let origin_z = centre_z.saturating_sub(i32::from(footprint[1] / 2));
    let origin = GridPos {
        x: u16::try_from(origin_x).ok()?,
        z: u16::try_from(origin_z).ok()?,
    };
    let centre = GridPos {
        x: u16::try_from(centre_x).ok()?,
        z: u16::try_from(centre_z).ok()?,
    };
    Some((origin, centre))
}

pub(crate) fn enemy_camp_region_is_clear(
    world: &GeneratedWorld,
    region: stream_town_domain::DirtyRegion,
    excluded: &[stream_town_domain::DirtyRegion],
) -> bool {
    if excluded.iter().any(|other| {
        region.min.x <= other.max.x
            && region.max.x >= other.min.x
            && region.min.z <= other.max.z
            && region.max.z >= other.min.z
    }) {
        return false;
    }
    (region.min.z..=region.max.z).all(|z| {
        (region.min.x..=region.max.x).all(|x| {
            let position = GridPos { x, z };
            world.navigation.height_at(position).unwrap_or_default() > 0
        })
    })
}

pub(crate) fn enemy_camp_has_town_route(
    world: &GeneratedWorld,
    region: stream_town_domain::DirtyRegion,
    town_reachable: &HashSet<GridPos>,
) -> bool {
    let min_x = region.min.x.saturating_sub(1);
    let min_z = region.min.z.saturating_sub(1);
    let max_x = region
        .max
        .x
        .saturating_add(1)
        .min(world.navigation.width().saturating_sub(1));
    let max_z = region
        .max
        .z
        .saturating_add(1)
        .min(world.navigation.height().saturating_sub(1));
    (min_z..=max_z)
        .flat_map(|z| (min_x..=max_x).map(move |x| GridPos { x, z }))
        .filter(|position| {
            position.x < region.min.x
                || position.x > region.max.x
                || position.z < region.min.z
                || position.z > region.max.z
        })
        .any(|position| town_reachable.contains(&position))
}

pub(crate) fn enemy_camp_town_reachable_cells(
    world: &GeneratedWorld,
    town_hall_approach: GridPos,
) -> HashSet<GridPos> {
    if !world.navigation.is_walkable(town_hall_approach) {
        return HashSet::new();
    }
    let mut reachable = HashSet::from([town_hall_approach]);
    let mut open = VecDeque::from([town_hall_approach]);
    while let Some(position) = open.pop_front() {
        for (neighbour, _) in world
            .navigation
            .walkable_neighbours(position)
            .into_iter()
            .flatten()
        {
            if reachable.insert(neighbour) {
                open.push_back(neighbour);
            }
        }
    }
    reachable
}

pub(crate) fn seed_generated_enemy_camps(
    config: &GameConfig,
    content: &ContentCatalog,
    world: &mut GeneratedWorld,
    simulation: &mut WorldSimulation,
) -> usize {
    if !simulation.enemy_camps.is_empty() {
        return 0;
    }
    let town_hall_centre = restored_town_hall_position(content, simulation, config);
    let town_hall_id = StableId::new("building:townhall").expect("static building ID");
    let town_hall_approach = simulation
        .buildings
        .get(&town_hall_id)
        .and_then(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            building_approach(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                town_hall_centre,
            )
        })
        .or_else(|| nearest_walkable(world, town_hall_centre))
        .unwrap_or(town_hall_centre);
    let mut town_reachable = enemy_camp_town_reachable_cells(world, town_hall_approach);
    let mut excluded = simulation
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
        .collect::<Vec<_>>();
    let mut centres = Vec::<GridPos>::new();
    let mut spawned = 0_usize;
    for layer in &content.enemy_camp_generation {
        let Some(archetype) = content.archetypes.get(&layer.camp_archetype) else {
            continue;
        };
        for camp_index in 0..layer.maximum_camps {
            let placement = (0..ENEMY_CAMP_PLACEMENT_ATTEMPTS).find_map(|attempt| {
                let (origin, centre) =
                    enemy_camp_candidate(config, layer, archetype.footprint, camp_index, attempt)?;
                let visual_region = building_region(origin, archetype.footprint, world)?;
                let navigation_region =
                    enemy_camp_navigation_region(origin, archetype.footprint, world)?;
                let centre_distance = grid_distance_squared(
                    centre,
                    GridPos {
                        x: config.world.width / 2,
                        z: config.world.height / 2,
                    },
                );
                let centre_minimum = u128::from(layer.minimum_distance_from_centre_milli_cells);
                if u128::from(centre_distance).saturating_mul(1_000_000)
                    < centre_minimum.saturating_mul(centre_minimum)
                    || centres.iter().any(|other| {
                        let distance = u128::from(grid_distance_squared(*other, centre))
                            .saturating_mul(1_000_000);
                        let minimum = u128::from(layer.minimum_distance_between_camps_milli_cells);
                        distance < minimum.saturating_mul(minimum)
                    })
                    || !enemy_camp_region_is_clear(world, visual_region, &excluded)
                    || !enemy_camp_has_town_route(world, navigation_region, &town_reachable)
                {
                    return None;
                }
                Some((origin, centre, navigation_region, visual_region))
            });
            let Some((origin, centre, navigation_region, visual_region)) = placement else {
                warn!(
                    layer = %layer.id,
                    camp_index,
                    attempts = ENEMY_CAMP_PLACEMENT_ATTEMPTS,
                    "could not place every authored enemy camp"
                );
                continue;
            };
            let id = StableId::new(format!("enemy_camp:{}:{camp_index:02}", layer.id.as_str()))
                .expect("generated camp IDs are valid");
            let health = archetype.health.as_ref().map_or(1_000, |health| {
                i32::try_from(health.max_health).unwrap_or(i32::MAX)
            });
            simulation.enemy_camps.insert(
                id.clone(),
                EnemyCampState {
                    id,
                    archetype: layer.camp_archetype.clone(),
                    position: origin,
                    health,
                    // EnemySpawner.Awake seeds its elapsed timer to the full
                    // interval, making the first eligible night spawn immediate.
                    spawn_remaining_seconds: 0.0,
                    spawned_enemies: BTreeSet::new(),
                },
            );
            world
                .navigation
                .set_blocked(navigation_region, true)
                .expect("validated enemy camp region updates navigation");
            town_reachable = enemy_camp_town_reachable_cells(world, town_hall_approach);
            centres.push(centre);
            excluded.push(visual_region);
            spawned += 1;
        }
    }
    spawned
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_enemy_camp(
    commands: &mut Commands,
    config: &GameConfig,
    world: &GeneratedWorld,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    render: &RenderAssets,
    id: &StableId,
    archetype: &ArchetypeDef,
    position: GridPos,
) {
    let centre = GridPos {
        x: position.x + archetype.footprint[0] / 2,
        z: position.z + archetype.footprint[1] / 2,
    };
    let world_position = grid_to_world_on_surface(centre, config, world);
    let mut entity = commands.spawn((
        WorldEntity,
        EnemyCamp { id: id.clone() },
        GridLocation(position),
        Transform::from_translation(world_position),
    ));
    if let Some(scene) = default_archetype_scene(archetype).filter(|scene| {
        asset_server.is_some() && converted_asset_exists(asset_root, &scene.asset_path)
    }) {
        entity.insert((
            WorldAssetRoot(
                asset_server
                    .expect("asset server checked above")
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            Transform::from_translation(world_position)
                .with_scale(Vec3::splat(config.world.cell_size / 2.0)),
        ));
        if let Some(material) = prefab_material_spec(archetype, scene, presentation, render) {
            entity.insert(material);
        }
    } else {
        let size = Vec3::new(
            f32::from(archetype.footprint[0]) * config.world.cell_size * 0.8,
            config.world.cell_size * 1.2,
            f32::from(archetype.footprint[1]) * config.world.cell_size * 0.8,
        );
        entity.insert((
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.building.clone()),
            Transform::from_translation(world_position + Vec3::Y * size.y * 0.5).with_scale(size),
        ));
    }
}

pub(crate) fn enemy_spawn_position(
    world: &GeneratedWorld,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    enemy_navigation: Option<&EnemyNavigationField>,
    camp: &EnemyCampState,
    spawner: &stream_town_domain::EnemySpawnerDef,
    footprint: [u16; 2],
    serial: u64,
) -> GridPos {
    let offset = spawner.spawn_offsets_milli_cells
        [usize::try_from(serial % spawner.spawn_offsets_milli_cells.len() as u64).unwrap_or(0)];
    let offset_cells = |value: i32| -> i32 {
        if value >= 0 {
            (value + 500) / 1_000
        } else {
            (value - 500) / 1_000
        }
    };
    let x = i64::from(camp.position.x)
        + i64::from(footprint[0] / 2)
        + i64::from(offset_cells(offset[0]));
    let z = i64::from(camp.position.z)
        + i64::from(footprint[1] / 2)
        + i64::from(offset_cells(offset[1]));
    let desired = GridPos {
        x: u16::try_from(x.clamp(0, i64::from(world.navigation.width() - 1))).unwrap_or(0),
        z: u16::try_from(z.clamp(0, i64::from(world.navigation.height() - 1))).unwrap_or(0),
    };
    let limit = world.navigation.width().max(world.navigation.height());
    for radius in 0..limit {
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
                let reaches_town = enemy_navigation.map_or_else(
                    || {
                        nearest_reachable_building_to_town_hall(
                            content, simulation, world, candidate,
                        )
                        .is_some()
                    },
                    |field| {
                        field
                            .destination(placement_to_navigation_centre(candidate))
                            .is_some()
                    },
                );
                if world.navigation.is_walkable(candidate) && reaches_town {
                    return candidate;
                }
            }
        }
    }
    nearest_walkable(world, desired)
        .unwrap_or_else(|| nearest_walkable(world, camp.position).unwrap_or(camp.position))
}

pub(crate) fn weighted_enemy_archetype(
    spawner: &stream_town_domain::EnemySpawnerDef,
    seed: u64,
    serial: u64,
) -> StableId {
    let total = spawner
        .weighted_enemies
        .iter()
        .map(|entry| u64::from(entry.weight_milli))
        .sum::<u64>();
    let mut mixed = seed.wrapping_add(serial.wrapping_mul(0x9e37_79b9_7f4a_7c15));
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    let mut value = (mixed ^ (mixed >> 31)) % total;
    for entry in &spawner.weighted_enemies {
        if value < u64::from(entry.weight_milli) {
            return entry.enemy_archetype.clone();
        }
        value -= u64::from(entry.weight_milli);
    }
    spawner.weighted_enemies[0].enemy_archetype.clone()
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_runtime_enemy(
    commands: &mut Commands,
    config: &GameConfig,
    world: &GeneratedWorld,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    render: &RenderAssets,
    simulation: &mut WorldSimulation,
    archetype_id: StableId,
    position: GridPos,
) -> Option<StableId> {
    let archetype = content.archetypes.get(&archetype_id)?;
    let enemy = archetype.enemy.as_ref()?;
    let base_health = archetype.health.as_ref()?.max_health;
    let player_count = simulation
        .actors
        .values()
        .filter(|actor| actor.role.as_str() != "role:enemy")
        .count();
    let additional = u64::from(enemy.additional_health_milli_per_player)
        .saturating_mul(u64::try_from(player_count).unwrap_or(u64::MAX))
        / 1_000;
    let max_health =
        u32::try_from(u64::from(base_health).saturating_add(additional)).unwrap_or(u32::MAX);
    let id = loop {
        let serial = simulation.next_enemy_serial;
        simulation.next_enemy_serial = simulation.next_enemy_serial.saturating_add(1);
        let id =
            StableId::new(format!("actor:enemy_{serial:08}")).expect("runtime enemy IDs are valid");
        if !simulation.actors.contains_key(&id) {
            break id;
        }
    };
    if !simulation.spawn_enemy(
        id.clone(),
        archetype_id.clone(),
        position,
        i32::try_from(max_health).unwrap_or(i32::MAX),
    ) {
        return None;
    }
    spawn_runtime_enemy_entity(
        commands,
        config,
        world,
        content,
        presentation,
        asset_server,
        asset_root,
        render,
        &id,
        archetype_id,
        position,
    );
    Some(id)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_runtime_enemy_entity(
    commands: &mut Commands,
    config: &GameConfig,
    world: &GeneratedWorld,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    render: &RenderAssets,
    id: &StableId,
    archetype_id: StableId,
    position: GridPos,
) {
    let Some(archetype) = content.archetypes.get(&archetype_id) else {
        return;
    };
    let world_position = grid_to_world_on_surface(position, config, world);
    let scene = runtime_archetype_scene(archetype).filter(|scene| {
        asset_server.is_some() && converted_asset_exists(asset_root, &scene.asset_path)
    });
    let converted_animation = scene
        .as_ref()
        .and_then(|_| converted_animation_spec(archetype, presentation));
    let native_animation = converted_animation
        .is_none()
        .then(|| {
            scene
                .as_ref()
                .and_then(|scene| native_animation_request(archetype, scene, presentation))
        })
        .flatten();
    let base_scale = if scene.is_some() {
        Vec3::splat(config.world.cell_size / 2.0)
    } else {
        Vec3::new(
            config.world.cell_size * 0.3,
            config.world.cell_size * 0.55,
            config.world.cell_size * 0.3,
        )
    };
    let visual_height = if scene.is_some() {
        world_position.y
    } else {
        world_position.y + base_scale.y * 0.5
    };
    let mut entity = commands.spawn((
        WorldEntity,
        GridLocation(position),
        Agent {
            id: id.clone(),
            kind: ActorKind::Enemy,
            archetype: archetype_id,
            goal: AgentGoal::Wander,
            spawn: position,
            origin: position,
            navigation_position: placement_to_navigation_centre(position),
            path: Vec::new(),
            path_index: 0,
            target: deterministic_wander_target(world, id, position),
            action_cooldown_seconds: 0.0,
            action_started: false,
            repath_remaining_seconds: initial_enemy_repath_delay(config, id),
            health_regen_accumulator: 0.0,
            wander_sequence: 0,
            previous_wander_origin: None,
        },
        AgentLocomotion::default(),
        AgentAnimation {
            base_scale,
            native: converted_animation.is_some() || native_animation.is_some(),
            ..default()
        },
        Transform::from_xyz(world_position.x, visual_height, world_position.z)
            .with_scale(base_scale),
    ));
    if let Some(scene) = scene {
        entity.insert(WorldAssetRoot(
            asset_server
                .expect("asset server checked above")
                .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
        ));
        if let Some(converted_animation) = converted_animation {
            entity.insert(converted_animation);
        } else if let Some(native_animation) = native_animation {
            entity.insert(native_animation);
        }
        if let Some(material) = prefab_material_spec(archetype, &scene, presentation, render) {
            entity.insert(material);
        }
    } else {
        entity.insert((
            Mesh3d(render.actor_lod.clone()),
            MeshMaterial3d(render.enemy_idle.clone()),
        ));
    }
}

pub(crate) fn runtime_building_id(simulation: &WorldSimulation) -> StableId {
    for sequence in constructed_building_count(simulation)..usize::MAX {
        let candidate = StableId::new(format!("building:runtime_{sequence:08}"))
            .expect("runtime building IDs are valid");
        if !simulation.buildings.contains_key(&candidate) {
            return candidate;
        }
    }
    unreachable!("runtime building identifier space exhausted")
}

pub(crate) fn spawn_runtime_building(
    commands: &mut Commands,
    config: &GameConfig,
    world: &GeneratedWorld,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    render: &RenderAssets,
    building: &BuildingState,
    definition: &BuildingDef,
    archetype: &ArchetypeDef,
    position: GridPos,
    footprint: [u16; 2],
    age: u8,
) {
    let occupied_footprint = rotated_footprint(footprint, building.rotation_quarter_turns);
    let centre = GridPos {
        x: position.x + occupied_footprint[0] / 2,
        z: position.z + occupied_footprint[1] / 2,
    };
    let world_position = grid_to_world_on_surface(centre, config, world);
    let rotation = quarter_turn_rotation(building.rotation_quarter_turns);
    let mut entity = commands.spawn((
        WorldEntity,
        RuntimeBuilding {
            id: building.id.clone(),
        },
        GridLocation(position),
        Transform::from_translation(world_position).with_rotation(rotation),
    ));
    if definition.projectile_shooter.is_some() {
        entity.insert(TowerShooter {
            cooldown_seconds: definition
                .projectile_shooter
                .as_ref()
                .map_or(0.0, |shooter| milli_units_as_f32(shooter.fire_milliseconds)),
        });
    }
    if archetype.source_path == "native://buildings/path" {
        return;
    }
    if archetype.source_path == "native://buildings/streetlight" {
        let pole_height = config.world.cell_size * 1.55;
        let pole_width = config.world.cell_size * 0.11;
        let lamp_size = config.world.cell_size * 0.22;
        entity.insert((
            BuildingPresentation {
                base_translation: world_position,
                base_scale: Vec3::ONE,
                base_height_offset: 0.0,
                applied_stage: u8::MAX,
                applied_level: u16::MAX,
                applied_age: age,
                applied_scene: None,
            },
            BuildingDamageEmitter::default(),
        ));
        entity.with_children(|root| {
            root.spawn((
                Name::new("Streetlight wooden pole"),
                Mesh3d(render.cube.clone()),
                MeshMaterial3d(render.building.clone()),
                Transform::from_xyz(0.0, pole_height * 0.5, 0.0).with_scale(Vec3::new(
                    pole_width,
                    pole_height,
                    pole_width,
                )),
            ));
            root.spawn((
                Name::new("Streetlight lamp"),
                Mesh3d(render.cube.clone()),
                MeshMaterial3d(render.streetlight_lamp.clone()),
                Transform::from_xyz(0.0, pole_height + lamp_size * 0.35, 0.0)
                    .with_scale(Vec3::new(lamp_size, lamp_size * 0.65, lamp_size)),
            ));
        });
        return;
    }
    if let Some(scene) = archetype_scene_for_age(archetype, age).filter(|scene| {
        asset_server.is_some() && converted_asset_exists(asset_root, &scene.asset_path)
    }) {
        let base_scale = Vec3::splat(config.world.cell_size / 2.0);
        entity.insert((
            WorldAssetRoot(
                asset_server
                    .expect("asset server checked above")
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            BuildingPresentation {
                base_translation: world_position,
                base_scale,
                base_height_offset: 0.0,
                applied_stage: u8::MAX,
                applied_level: u16::MAX,
                applied_age: age,
                applied_scene: Some(scene.asset_path.clone()),
            },
            BuildingDamageEmitter::default(),
            Transform::from_translation(world_position)
                .with_rotation(rotation)
                .with_scale(base_scale),
        ));
        if let Some(material) =
            building_prefab_material_spec(archetype, scene, presentation, render)
        {
            entity.insert(material);
        }
    } else {
        let size = Vec3::new(
            f32::from(footprint[0]) * config.world.cell_size * 0.88,
            config.world.cell_size * 1.25,
            f32::from(footprint[1]) * config.world.cell_size * 0.88,
        );
        let base_translation = world_position + Vec3::Y * size.y * 0.5;
        entity.insert((
            BuildingPresentation {
                base_translation,
                base_scale: size,
                base_height_offset: size.y * 0.5,
                applied_stage: u8::MAX,
                applied_level: u16::MAX,
                applied_age: age,
                applied_scene: None,
            },
            BuildingDamageEmitter::default(),
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(if building.complete {
                render.building.clone()
            } else {
                render.construction.clone()
            }),
            Transform::from_translation(base_translation)
                .with_rotation(rotation)
                .with_scale(size),
        ));
    }
}
