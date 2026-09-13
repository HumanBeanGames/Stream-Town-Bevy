#[test]
fn reveal_grace_frames_begin_only_after_scene_readiness() {
    assert!(!asset_root_collection_ready(0, 0, true));
    assert!(asset_root_collection_ready(0, 0, false));
    assert!(!asset_root_collection_ready(3, 2, true));
    assert!(asset_root_collection_ready(3, 3, true));

    let mut starting_frame = None;
    assert_eq!(
        rendered_frames_since_ready(&mut starting_frame, false, 100),
        0
    );
    assert_eq!(starting_frame, None);

    assert_eq!(
        rendered_frames_since_ready(&mut starting_frame, true, 110),
        0
    );
    assert_eq!(starting_frame, Some(110));
    assert_eq!(
        rendered_frames_since_ready(&mut starting_frame, true, 115),
        5
    );

    assert_eq!(
        rendered_frames_since_ready(&mut starting_frame, false, 120),
        0
    );
    assert_eq!(starting_frame, None);
    assert_eq!(
        rendered_frames_since_ready(&mut starting_frame, true, 130),
        0
    );
    assert_eq!(starting_frame, Some(130));
}

#[test]
fn gpu_reveal_requires_every_asset_pipeline_and_selection_draw() {
    let mut snapshot = GpuReadinessSnapshot {
        epoch: 4,
        expected_images: 12,
        ready_images: 12,
        expected_meshes: 7,
        ready_meshes: 7,
        expected_materials: 9,
        ready_materials: 9,
        selection_expected: true,
        ..default()
    };
    assert!(!snapshot.is_ready());
    snapshot.selection_draw_ready = true;
    assert!(snapshot.is_ready());
    snapshot.pending_pipelines = 1;
    assert!(!snapshot.is_ready());
    snapshot.pending_pipelines = 0;
    snapshot.failed_pipelines = 1;
    assert!(!snapshot.is_ready());
    snapshot.failed_pipelines = 0;
    snapshot.ready_images -= 1;
    assert!(!snapshot.is_ready());
}

#[test]
fn cold_world_transition_requires_a_presented_gpu_ready_loading_cover() {
    assert!(!loading_cover_ready(false, 3, 3, 0, true, 3));
    assert!(!loading_cover_ready(true, 2, 3, 0, true, 3));
    assert!(!loading_cover_ready(true, 3, 3, 1, true, 3));
    assert!(!loading_cover_ready(true, 3, 3, 0, false, 3));
    assert!(!loading_cover_ready(true, 3, 3, 0, true, 2));
    assert!(loading_cover_ready(true, 3, 3, 0, true, 3));
}

#[test]
fn object_selection_outline_keeps_its_positive_depth_bias() {
    let material = selection_outline_material(StandardMaterial::default());
    assert!((material.depth_bias - SELECTION_OUTLINE_DEPTH_BIAS).abs() < f32::EPSILON);
    assert!(material.depth_bias > 0.0);
}

#[test]
fn wall_and_gate_tiling_match_unity_tile_value_tables() {
    assert_eq!(wall_tiling(0), (0, 1));
    assert_eq!(wall_tiling(20), (0, 0));
    assert_eq!(wall_tiling(6), (1, 1));
    assert_eq!(wall_tiling(12), (1, 2));
    assert_eq!(wall_tiling(24), (1, 3));
    assert_eq!(wall_tiling(22), (3, 0));
    assert_eq!(wall_tiling(14), (3, 1));
    assert_eq!(wall_tiling(28), (3, 2));
    assert_eq!(wall_tiling(26), (3, 3));
    assert_eq!(wall_tiling(30), (2, 0));
    assert_eq!(gate_tiling(2), 1);
    assert_eq!(gate_tiling(12), 2);
    assert_eq!(gate_tiling(24), 3);
    assert_eq!(gate_tiling(18), 0);
}

#[test]
fn wall_adjacency_selects_the_matching_converted_scene() {
    let content = embedded_content();
    let building_id = StableId::new("building:wall").unwrap();
    let definition = &content.buildings[&building_id];
    let archetype = &content.archetypes[&definition.archetype];
    let wall = StableId::new("building:wall_test").unwrap();
    let neighbor = StableId::new("building:wall_neighbor").unwrap();
    let state = BuildingState {
        id: wall.clone(),
        archetype: definition.archetype.clone(),
        position: GridPos { x: 10, z: 10 },
        rotation_quarter_turns: 0,
        level: 1,
        health: BUILDING_MAX_HEALTH,
        complete: true,
    };
    let mut simulation = WorldSimulation::new(42);
    simulation.buildings.insert(wall, state.clone());
    let straight = building_scene_for_state(
        archetype,
        definition,
        &building_id,
        &state,
        1,
        &content,
        &simulation,
    )
    .unwrap();
    assert!(straight.source_model.ends_with("Age01_Wall_Straight.fbx"));

    simulation.buildings.insert(
        neighbor.clone(),
        BuildingState {
            id: neighbor,
            position: GridPos { x: 11, z: 10 },
            ..state.clone()
        },
    );
    let north = StableId::new("building:wall_north").unwrap();
    simulation.buildings.insert(
        north.clone(),
        BuildingState {
            id: north,
            position: GridPos { x: 10, z: 11 },
            ..state.clone()
        },
    );
    let corner = building_scene_for_state(
        archetype,
        definition,
        &building_id,
        &state,
        1,
        &content,
        &simulation,
    )
    .unwrap();
    assert!(corner.source_model.ends_with("Age01_Wall_Corner.fbx"));
    let converted_mask = tiled_neighbor_value(&content, &simulation, &state);
    assert_eq!(converted_mask, 24);
    assert_eq!(
        wall_tiling(converted_mask),
        (1, 3),
        "the reflected GLB corner rotates toward +X/+Z neighbours"
    );
}

#[test]
fn wall_visual_upgrade_waits_for_town_age_then_applies() {
    let content = embedded_content();
    let wall_id = StableId::new("building:wall").unwrap();
    let town_hall_id = StableId::new("building:townhall").unwrap();
    let definition = &content.buildings[&wall_id];
    let archetype = &content.archetypes[&definition.archetype];
    let state = BuildingState {
        id: StableId::new("building:wall_age_cap_test").unwrap(),
        archetype: definition.archetype.clone(),
        position: GridPos { x: 10, z: 10 },
        rotation_quarter_turns: 0,
        level: 1,
        health: BUILDING_MAX_HEALTH,
        complete: true,
    };
    let age_technology_for = |building: &StableId| {
        content
            .technology
            .nodes
            .iter()
            .find(|(_, node)| node.aged_buildings.contains(building))
            .map_or_else(
                || panic!("missing age technology for {building}"),
                |(technology, _)| technology.clone(),
            )
    };
    let mut simulation = WorldSimulation::new(42);
    simulation.buildings.insert(state.id.clone(), state.clone());
    simulation
        .unlocked_technology
        .insert(age_technology_for(&wall_id));

    let capped_age = building_age(&content, &simulation, &wall_id, state.level);
    assert_eq!(capped_age, 1, "the wall upgrade must remain pending");
    let wood_scene = building_scene_for_state(
        archetype,
        definition,
        &wall_id,
        &state,
        capped_age,
        &content,
        &simulation,
    )
    .unwrap();
    assert!(wood_scene.source_model.ends_with("Age01_Wall_Straight.fbx"));
    let stone_model = definition
        .model_handlers
        .iter()
        .find(|model| model.age == 2)
        .expect("wall has an age-two model");
    assert_eq!(
        building_node_visibility(
            &wall_id,
            definition,
            &state,
            capped_age,
            &stone_model.full_model,
            &GameConfig::default(),
            &content,
            &simulation,
        ),
        Some(false),
        "age-two wall nodes must stay hidden while the town is age one"
    );

    simulation
        .unlocked_technology
        .insert(age_technology_for(&town_hall_id));
    let unlocked_age = building_age(&content, &simulation, &wall_id, state.level);
    assert_eq!(
        unlocked_age, 1,
        "town age alone must not upgrade an individual level-one wall"
    );
    let mut upgraded_state = state.clone();
    upgraded_state.level = 11;
    simulation
        .buildings
        .insert(upgraded_state.id.clone(), upgraded_state.clone());
    let unlocked_age = building_age(&content, &simulation, &wall_id, upgraded_state.level);
    assert_eq!(
        unlocked_age, 2,
        "level eleven unlocks the age-two wall model"
    );
    let stone_scene = building_scene_for_state(
        archetype,
        definition,
        &wall_id,
        &upgraded_state,
        unlocked_age,
        &content,
        &simulation,
    )
    .unwrap();
    assert!(
        stone_scene
            .source_model
            .ends_with("Age02_Wall_Straight.fbx")
    );
}

#[test]
fn converted_wall_corners_face_all_four_world_quadrants() {
    let content = embedded_content();
    let definition = &content.buildings[&StableId::new("building:wall").unwrap()];
    let centre_id = StableId::new("building:wall_centre").unwrap();
    let centre = BuildingState {
        id: centre_id.clone(),
        archetype: definition.archetype.clone(),
        position: GridPos { x: 10, z: 10 },
        rotation_quarter_turns: 0,
        level: 1,
        health: BUILDING_MAX_HEALTH,
        complete: true,
    };
    for (suffix, offsets, expected_mask, expected_turns) in [
        ("east_north", [(1, 0), (0, 1)], 24, 3),
        ("east_south", [(1, 0), (0, -1)], 12, 2),
        ("west_south", [(-1, 0), (0, -1)], 6, 1),
        ("west_north", [(-1, 0), (0, 1)], 18, 0),
    ] {
        let mut simulation = WorldSimulation::new(42);
        simulation
            .buildings
            .insert(centre_id.clone(), centre.clone());
        for (index, (x, z)) in offsets.into_iter().enumerate() {
            let id = StableId::new(format!("building:wall_{suffix}_{index}")).unwrap();
            simulation.buildings.insert(
                id.clone(),
                BuildingState {
                    id,
                    position: GridPos {
                        x: u16::try_from(10 + x).unwrap(),
                        z: u16::try_from(10 + z).unwrap(),
                    },
                    ..centre.clone()
                },
            );
        }
        let mask = tiled_neighbor_value(&content, &simulation, &centre);
        assert_eq!(mask, expected_mask, "wrong converted mask for {suffix}");
        assert_eq!(
            wall_tiling(mask),
            (1, expected_turns),
            "wrong corner rotation for {suffix}"
        );
    }
}

#[test]
fn completed_gates_open_player_routes_but_remain_blocked_for_enemies() {
    let content = embedded_content();
    let gate_id = StableId::new("building:gate").unwrap();
    let definition = &content.buildings[&gate_id];
    let runtime_id = StableId::new("building:gate_test").unwrap();
    let mut simulation = WorldSimulation::new(42);
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id,
            archetype: definition.archetype.clone(),
            position: GridPos { x: 2, z: 1 },
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );
    let mut navigation =
        stream_town_domain::NavGrid::new(5, 3, vec![false; 15], vec![0; 15]).unwrap();
    navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: GridPos { x: 2, z: 0 },
                max: GridPos { x: 2, z: 2 },
            },
            true,
        )
        .unwrap();
    let start = GridPos { x: 0, z: 1 };
    let goal = GridPos { x: 4, z: 1 };
    let player_path = agent_path(
        &navigation,
        &content,
        &simulation,
        &ActorKind::Player,
        start,
        goal,
    );
    assert!(player_path.contains(&GridPos { x: 2, z: 1 }));
    assert_eq!(player_path.last(), Some(&goal));
    assert_eq!(
        agent_path(
            &navigation,
            &content,
            &simulation,
            &ActorKind::Enemy,
            start,
            goal,
        ),
        vec![start]
    );
    simulation.buildings.values_mut().next().unwrap().complete = false;
    assert_eq!(
        agent_path(
            &navigation,
            &content,
            &simulation,
            &ActorKind::Player,
            start,
            goal,
        ),
        vec![start]
    );
}

#[test]
fn gate_animation_contract_resolves_both_shipping_ages() {
    let presentation = embedded_presentation();
    let wood = gate_animation_contract(1, &presentation).unwrap();
    assert_eq!(
        wood.controller.as_str(),
        "controller:20be80b0e10b1af40a0fc064abd6a19b"
    );
    assert_eq!(wood.model_root_name, "Age01_Gate01");
    assert_eq!(wood.animation_root_name, "Age01_Gate01");
    assert!(
        wood.open
            .asset_path
            .ends_with("Age01_Gate_Animation_Open01.glb")
    );
    assert!(
        wood.close
            .asset_path
            .ends_with("Age01_Gate_Animation_Closing01.glb")
    );
    assert_eq!(wood.open.animation_index, 0);
    assert_eq!(wood.close.animation_index, 0);
    assert!((wood.open.transition_seconds - 0.25).abs() <= f32::EPSILON);
    assert!((wood.close.transition_seconds - 0.25).abs() <= f32::EPSILON);

    let stone = gate_animation_contract(2, &presentation).unwrap();
    assert_eq!(
        stone.controller.as_str(),
        "controller:2ddf69683616b2e43b31be96ca9624d5"
    );
    assert_eq!(stone.model_root_name, "Armature");
    assert_eq!(stone.animation_root_name, "Age02_Gate");
    assert!(
        stone
            .open
            .asset_path
            .ends_with("Age02_Gate_Animation_Open01.glb")
    );
    assert!(
        stone
            .close
            .asset_path
            .ends_with("Age02_Gate_Animation_Closing01.glb")
    );
    assert_eq!(stone.open.animation_index, 0);
    assert_eq!(stone.close.animation_index, 0);
}

#[test]
fn gate_trigger_matches_the_authored_four_unit_box() {
    let gate = Vec3::new(10.0, 5.0, -4.0);
    assert!(gate_trigger_contains(
        gate,
        Vec3::new(12.0, 100.0, -6.0),
        2.0
    ));
    assert!(!gate_trigger_contains(
        gate,
        Vec3::new(12.001, 5.0, -4.0),
        2.0
    ));
    assert!(!gate_trigger_contains(
        gate,
        Vec3::new(10.0, 5.0, -6.001),
        2.0
    ));
}

#[test]
fn ping_pointer_preserves_shipping_curve_duration_and_mesh_anchor() {
    for (elapsed, expected) in [(0.0, 0.0), (2.0, 0.7), (4.0, 0.4), (6.0, 0.7), (8.0, 0.0)] {
        assert!((ping_pointer_scale(elapsed) - expected).abs() < 1.0e-6);
    }
    let actor_position = Vec3::new(12.0, 7.0, -3.0);
    let actor_height = 6.0;
    let base_scale = 12.0 / PING_POINTER_MODEL_HEIGHT;
    let transform = ping_pointer_transform(actor_position, actor_height, base_scale, 2.0);
    let mesh_bottom = transform.translation.y + PING_POINTER_MODEL_MIN_Y * transform.scale.y;
    assert!((mesh_bottom - (actor_position.y + actor_height)).abs() < 1.0e-5);
    assert!((transform.translation.x - actor_position.x).abs() < f32::EPSILON);
    assert!((transform.translation.z - actor_position.z).abs() < f32::EPSILON);
}

#[test]
fn ping_pointer_uses_converted_shipping_model_and_material() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let archetype = content
        .archetypes
        .get(&StableId::new("archetype:prefab:3934bb254cb808a4f9f465f5a1f3ca48").unwrap())
        .unwrap();
    let scene = default_archetype_scene(archetype).unwrap();
    assert_eq!(scene.source_model, "Assets/Models/VFX/PointerArrow.fbx");
    assert_eq!(scene.asset_path, PING_POINTER_MODEL_PATH);
    assert!(converted_asset_exists(
        &locate_asset_root(),
        PING_POINTER_MODEL_PATH
    ));
    assert_eq!(
        presentation.model_materials[&scene.source_model]["Material.001"].as_str(),
        PING_POINTER_MATERIAL_ID
    );
    let material = &presentation.materials
        [&StableId::new(PING_POINTER_MATERIAL_ID).expect("static pointer material ID")];
    assert_eq!(material.source_path, "Assets/Materials/VFX/VFX_Pointer.mat");
    assert!(
        material
            .base_color
            .into_iter()
            .zip([0.858_490_6, 0.028_346_404, 0.028_346_404, 1.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert!(
        material
            .emissive
            .into_iter()
            .zip([0.890_021_1, 0.0, 0.0, 1.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
}

#[test]
fn fish_god_uses_complete_shipping_entry_idle_exit_controller() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let archetype = content
        .archetypes
        .values()
        .find(|archetype| archetype.source_path.ends_with("Event_FishGod.prefab"))
        .unwrap();
    let scene = default_archetype_scene(archetype).unwrap();
    let spec = complete_converted_animation_spec(archetype, scene, &presentation).unwrap();
    assert_eq!(
        spec.controller.as_str(),
        "controller:c11f40d5660648f46b67cfaaa37761e5"
    );
    assert_eq!(
        scene.asset_path,
        "shipping/models/Models/Events/FishGod.glb"
    );
    let controller = &presentation.controllers[&spec.controller];
    assert_eq!(controller.display_name, "FishGod");
    assert_eq!(controller.states[&spec.state].display_name, "Entry");
    let states = controller
        .states
        .values()
        .map(|state| state.display_name.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(states, BTreeSet::from(["Entry", "Exit", "Idle"]));
    assert!(controller.parameters.iter().any(|parameter| {
        parameter.name == "Exit"
            && parameter.kind == stream_town_domain::AnimationParameterKind::Trigger
    }));
    let clips = controller
        .states
        .values()
        .flat_map(|state| &state.motions)
        .map(|motion| &presentation.clips[&motion.clip])
        .collect::<Vec<_>>();
    assert_eq!(clips.len(), 3);
    assert!(clips.iter().all(|clip| {
        clip.converted_asset_path.as_deref() == Some(scene.asset_path.as_str())
            && clip.gltf_animation_index.is_some()
    }));
    let idle = controller
        .states
        .iter()
        .find(|(_, state)| state.display_name == "Idle")
        .map(|(id, _)| id.clone())
        .unwrap();
    let exit = controller
        .states
        .iter()
        .find(|(_, state)| state.display_name == "Exit")
        .map(|(id, _)| id.clone())
        .unwrap();
    let mut runtime = AnimationControllerRuntime::in_state(controller, idle).unwrap();
    runtime.set_trigger("Exit").unwrap();
    assert_eq!(
        runtime.evaluate_transitions(controller, 0.91).unwrap(),
        stream_town_domain::AnimationTransitionOutcome::Entered(exit)
    );
    let playback = runtime.take_transition_playback().unwrap();
    assert!(playback.fixed_duration);
    assert!((playback.duration - 0.25).abs() < f32::EPSILON);
    assert!((FISH_GOD_EXIT_DELAY_SECONDS - 2.5).abs() < f32::EPSILON);
}

#[test]
fn fish_god_rain_uses_converted_authored_mesh_particle_contract() {
    let presentation = embedded_presentation();
    let (id, effect) = raining_fish_effect(&presentation).unwrap();
    assert_eq!(
        id.as_str(),
        "particle_effect:220b9325efc3ade41a3298ab568345ef"
    );
    assert_eq!(
        effect.model_asset_path,
        "shipping/models/Models/Critters/Critter_Fish3.glb"
    );
    assert_eq!(effect.material.as_str(), CRITTER_MATERIAL_ID);
    assert!((effect.emission_rate_per_second - 500.0).abs() < f32::EPSILON);
    assert!((effect.lifetime_seconds - 15.0).abs() < f32::EPSILON);
    assert_eq!(effect.max_particles, 5_000);
    assert!(
        effect
            .emitter_position
            .into_iter()
            .zip([0.0, 46.2, 0.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert!(
        effect
            .shape_scale
            .into_iter()
            .zip([300.0, 300.0, 5.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert!((effect.collision_bounce - 0.5).abs() < f32::EPSILON);
    assert!((effect.collision_lifetime_loss - 0.25).abs() < f32::EPSILON);
    assert!(effect.prewarm && effect.world_space);
    assert!(RAINING_FISH_RENDER_BUDGET < usize::from(effect.max_particles));
    assert_eq!(effect.size_multiplier(1.0), Some(0.0));
}

#[test]
fn falling_fish_sampling_is_repeatable_and_sequence_sensitive() {
    let first = (0..64)
        .map(|sequence| falling_fish_seed(42, sequence, 3))
        .collect::<Vec<_>>();
    let repeated = (0..64)
        .map(|sequence| falling_fish_seed(42, sequence, 3))
        .collect::<Vec<_>>();
    assert_eq!(first, repeated);
    assert_eq!(first.iter().copied().collect::<BTreeSet<_>>().len(), 64);
    assert_ne!(falling_fish_seed(42, 7, 3), falling_fish_seed(43, 7, 3));
    assert_ne!(falling_fish_seed(42, 7, 3), falling_fish_seed(42, 7, 4));
}

#[test]
fn falling_fish_uses_authored_gravity_terrain_bounce_and_lifetime_loss() {
    let config = GameConfig::default();
    let world = generate_world(&config.world);
    let presentation = embedded_presentation();
    let (_, effect) = raining_fish_effect(&presentation).unwrap();
    let cell = GridPos {
        x: config.world.width / 2,
        z: config.world.height / 2,
    };
    let floor = terrain_height(&world, cell);
    let mut fish = FallingFish {
        effect: StableId::new("particle_effect:test").unwrap(),
        sequence: 7,
        velocity: Vec3::new(0.0, -12.0, 0.0),
        angular_velocity: Vec3::ZERO,
        age_seconds: 2.0,
        lifetime_seconds: effect.lifetime_seconds,
        base_scale: 0.01,
        collision_count: 0,
    };
    let mut transform = Transform::from_translation(
        grid_to_world_on_surface(cell, &config, &world) + Vec3::Y * 0.01,
    );
    advance_falling_fish(
        effect,
        &mut fish,
        &mut transform,
        world.seed,
        7,
        &config,
        &world,
        0.05,
    );
    assert!((transform.translation.y - floor).abs() < f32::EPSILON);
    assert!(fish.velocity.y > 0.0);
    assert_eq!(fish.collision_count, 1);
    let expected_age = 2.05 + effect.lifetime_seconds * effect.collision_lifetime_loss;
    assert!((fish.age_seconds - expected_age).abs() < 0.000_01);
    let expected_scale = effect
        .size_multiplier(fish.age_seconds / fish.lifetime_seconds)
        .unwrap()
        * fish.base_scale;
    assert!((transform.scale.x - expected_scale).abs() < f32::EPSILON);
}

#[test]
fn save_restore_keeps_players_on_completed_gate_cells_only() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world_with_content(&config.world, &content);
    let gate_id = StableId::new("building:gate").unwrap();
    let gate = &content.buildings[&gate_id];
    let position = find_building_site(
        &world,
        GridPos {
            x: config.world.width / 2,
            z: config.world.height / 2,
        },
        gate.footprint,
    )
    .unwrap();
    let region = building_region(position, gate.footprint, &world).unwrap();
    world.navigation.set_blocked(region, true).unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.buildings.insert(
        StableId::new("building:gate_restore_test").unwrap(),
        BuildingState {
            id: StableId::new("building:gate_restore_test").unwrap(),
            archetype: gate.archetype.clone(),
            position,
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );

    assert_eq!(
        restored_actor_position(&world, &content, &simulation, &ActorKind::Player, position,),
        Some(position)
    );
    assert_ne!(
        restored_actor_position(&world, &content, &simulation, &ActorKind::Enemy, position,),
        Some(position)
    );
    simulation.buildings.values_mut().next().unwrap().complete = false;
    assert_ne!(
        restored_actor_position(&world, &content, &simulation, &ActorKind::Player, position,),
        Some(position)
    );
}

#[test]
fn technology_effects_authoritatively_gate_buildings() {
    let content = embedded_content();
    let mut simulation = WorldSimulation::new(42);
    simulation.unlocked_technology.extend(
        content
            .technology
            .nodes
            .iter()
            .filter(|(_, technology)| technology.initially_unlocked)
            .map(|(id, _)| id.clone()),
    );
    let lumbermill = StableId::new("building:lumbermill").unwrap();
    assert!(building_is_unlocked(&content, &simulation, &lumbermill));
    let (technology, locked_building) = content
        .technology
        .nodes
        .iter()
        .filter(|(technology, _)| !simulation.unlocked_technology.contains(*technology))
        .find_map(|(technology, node)| {
            node.unlocked_buildings
                .iter()
                .find(|building| {
                    content.buildings[*building].placeable
                        && !building_is_unlocked(&content, &simulation, building)
                })
                .map(|building| (technology.clone(), building.clone()))
        })
        .expect("Unity technology graph contains a locked placeable building");
    assert!(!building_is_unlocked(
        &content,
        &simulation,
        &locked_building
    ));
    simulation.unlocked_technology.insert(technology);
    assert!(building_is_unlocked(
        &content,
        &simulation,
        &locked_building
    ));
}

#[test]
fn converted_technology_modifiers_change_runtime_rules() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut simulation = WorldSimulation::new(42);

    let (cost_technology, building_id, reduction) = content
        .technology
        .nodes
        .iter()
        .find_map(|(technology, node)| {
            node.building_cost_reduction_percent
                .iter()
                .next()
                .map(|(building, amount)| (technology.clone(), building.clone(), *amount))
        })
        .expect("Unity technology graph contains building cost reductions");
    let definition = &content.buildings[&building_id];
    simulation.unlocked_technology.insert(cost_technology);
    assert_eq!(
        building_cost_reduction_percent(&content, &simulation, &building_id),
        reduction
    );
    let adjusted = building_construction_cost(&content, &simulation, &building_id, definition);
    for (resource, base) in &definition.cost {
        assert_eq!(adjusted[resource], percentage_reduced(*base, reduction));
    }

    simulation.unlocked_technology.clear();
    let (storage_technology, resource, boost) = content
        .technology
        .nodes
        .iter()
        .find_map(|(technology, node)| {
            node.storage_boost_percent
                .iter()
                .next()
                .map(|(resource, amount)| (technology.clone(), resource.clone(), *amount))
        })
        .expect("Unity technology graph contains storage boosts");
    let storage_definition = content
        .buildings
        .values()
        .find(|building| {
            building
                .storage
                .iter()
                .any(|storage| storage.resource == resource)
        })
        .expect("converted catalog contains the boosted storage building");
    let storage = storage_definition
        .storage
        .iter()
        .find(|storage| storage.resource == resource)
        .unwrap();
    let runtime_storage = StableId::new("building:test_storage").unwrap();
    simulation.buildings.insert(
        runtime_storage.clone(),
        BuildingState {
            id: runtime_storage,
            archetype: storage_definition.archetype.clone(),
            position: GridPos { x: 1, z: 1 },
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );
    simulation.unlocked_technology.insert(storage_technology);
    let base = config.gameplay.base_town_resource_capacity[&resource];
    assert_eq!(
        resource_storage_capacity(&config, &content, &simulation, &resource),
        base + percentage_adjusted(storage.base_amount, boost)
    );

    simulation.unlocked_technology.clear();
    let (stat_technology, role, stat, boost) = content
        .technology
        .nodes
        .iter()
        .find_map(|(technology, node)| {
            node.role_stat_boost_percent
                .iter()
                .find_map(|(role, stats)| {
                    stats.iter().next().map(|(stat, amount)| {
                        (technology.clone(), role.clone(), stat.clone(), *amount)
                    })
                })
        })
        .expect("Unity technology graph contains role stat boosts");
    simulation.unlocked_technology.insert(stat_technology);
    assert_eq!(
        technology_stat_boost_percent(&content, &simulation, &role, &stat),
        boost
    );

    simulation.unlocked_technology.clear();
    let (global_technology, global_stat, retained_value) = content
        .technology
        .nodes
        .iter()
        .find_map(|(technology, node)| {
            node.global_stat_boost_percent
                .iter()
                .next()
                .map(|(stat, amount)| (technology.clone(), stat.clone(), *amount))
        })
        .expect("Unity technology graph retains global stat boost data");
    assert!(retained_value > 0);
    simulation.unlocked_technology.insert(global_technology);
    assert_eq!(
        technology_stat_boost_percent(
            &content,
            &simulation,
            &StableId::new("role:logger").unwrap(),
            &global_stat,
        ),
        0,
        "shipping Unity stores global technology stats but PlayerRoleData never consumes them"
    );

    simulation.unlocked_technology.clear();
    let town_hall = StableId::new("building:townhall").unwrap();
    let ore_storage = StableId::new("building:orestorage").unwrap();
    let food_storage = StableId::new("building:foodstorage").unwrap();
    let age_technology_for = |building: &StableId| {
        content
            .technology
            .nodes
            .iter()
            .find(|(_, node)| node.aged_buildings.contains(building))
            .map_or_else(
                || panic!("missing age technology for {building}"),
                |(technology, _)| technology.clone(),
            )
    };
    let town_age_technology = age_technology_for(&town_hall);
    let ore_age_technology = age_technology_for(&ore_storage);
    let ore_archetype = &content.archetypes[&content.buildings[&ore_storage].archetype];

    assert_eq!(building_age(&content, &simulation, &ore_storage, 100), 1);
    simulation.unlocked_technology.insert(ore_age_technology);
    assert_eq!(
        building_age(&content, &simulation, &ore_storage, 100),
        1,
        "a building upgrade remains pending until the town reaches that age"
    );
    simulation.unlocked_technology.insert(town_age_technology);
    assert_eq!(building_age(&content, &simulation, &town_hall, 100), 2);
    assert_eq!(
        building_age(&content, &simulation, &ore_storage, 100),
        2,
        "unlocking the town age applies pending building visual upgrades"
    );
    assert_eq!(
        building_age(&content, &simulation, &food_storage, 100),
        1,
        "the town age does not guarantee every building visual upgrade"
    );
    assert_eq!(
        archetype_scene_for_age(ore_archetype, 2).unwrap().age,
        Some(2)
    );
}

#[test]
fn construction_costs_scale_by_existing_copies_except_fixed_buildings() {
    let content = embedded_content();
    let mut simulation = WorldSimulation::new(42);
    let house_id = StableId::new("building:house").unwrap();
    let house = &content.buildings[&house_id];
    let base = building_construction_cost(&content, &simulation, &house_id, house);
    let existing_id = StableId::new("building:existing-house").unwrap();
    simulation.buildings.insert(
        existing_id.clone(),
        BuildingState {
            id: existing_id,
            archetype: house.archetype.clone(),
            position: GridPos { x: 1, z: 1 },
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );
    let next = building_construction_cost(&content, &simulation, &house_id, house);
    for (resource, amount) in base {
        assert_eq!(next[&resource], amount.saturating_mul(2));
    }

    for fixed_name in ["building:path", "building:streetlight"] {
        let fixed_id = StableId::new(fixed_name).unwrap();
        let fixed = &content.buildings[&fixed_id];
        let before = building_construction_cost(&content, &simulation, &fixed_id, fixed);
        for index in 0..3_u16 {
            let id = StableId::new(format!("building:fixed-{index}-{fixed_name}")).unwrap();
            simulation.buildings.insert(
                id.clone(),
                BuildingState {
                    id,
                    archetype: fixed.archetype.clone(),
                    position: GridPos { x: index, z: index },
                    rotation_quarter_turns: 0,
                    level: 1,
                    health: BUILDING_MAX_HEALTH,
                    complete: true,
                },
            );
        }
        assert_eq!(
            building_construction_cost(&content, &simulation, &fixed_id, fixed),
            before
        );
    }
}

#[test]
fn authored_building_and_role_balance_is_explicit_and_complete() {
    let content = embedded_content();
    let half_scaling = BTreeSet::from([
        "building:foodstorage",
        "building:orestorage",
        "building:woodstorage",
        "building:tower",
        "building:necrotower",
        "building:wizardtower",
        "building:guardhouse",
        "building:gate",
    ]);
    for (id, building) in &content.buildings {
        let expected_construction_scaling = match id.as_str() {
            "building:path" | "building:streetlight" => 0,
            "building:wall" => 50,
            name if half_scaling.contains(name) => 500,
            _ => 1_000,
        };
        assert_eq!(
            building.construction_cost_multiplier_per_thousand, expected_construction_scaling,
            "unexpected construction scaling for {id}"
        );
        assert_eq!(building.upgrade_cost_per_target_level_per_thousand, 500);
        let expected_health = if building.can_level {
            match id.as_str() {
                "building:wall" => 500,
                "building:house" => 300,
                _ => 200,
            }
        } else {
            0
        };
        assert_eq!(
            building.health_bonus_per_level_per_thousand, expected_health,
            "unexpected health scaling for {id}"
        );
    }

    let recruit = StableId::new("resource:recruit").unwrap();
    let house = &content.buildings[&StableId::new("building:house").unwrap()];
    let population = house
        .storage
        .iter()
        .find(|storage| storage.resource == recruit)
        .unwrap();
    assert_eq!(
        (population.base_amount, population.increment_amount),
        (1, 0)
    );
    assert_eq!(house.global_gather_rate_bonus_per_level_per_thousand, 5);

    for storage_name in ["foodstorage", "orestorage", "woodstorage"] {
        let building =
            &content.buildings[&StableId::new(format!("building:{storage_name}")).unwrap()];
        assert!(
            building
                .storage
                .iter()
                .any(|storage| storage.base_amount == 10_000 && storage.increment_amount == 5_000)
        );
    }
    let shooters = content
        .buildings
        .values()
        .filter_map(|building| building.projectile_shooter.as_ref())
        .collect::<Vec<_>>();
    assert!(!shooters.is_empty());
    for shooter in shooters {
        assert_eq!(shooter.damage_bonus_per_level_per_thousand, 200);
    }
    for building in content
        .buildings
        .values()
        .filter(|building| building.station.is_some())
    {
        assert_eq!(
            building
                .station
                .as_ref()
                .unwrap()
                .experience_bonus_per_level_per_thousand,
            50
        );
    }

    let military_roles = BTreeSet::from([
        "role:defender",
        "role:necromancer",
        "role:paladin",
        "role:ranger",
        "role:soldier",
        "role:wizard",
    ]);
    for (id, role) in &content.roles {
        assert_eq!(role.movement_speed_milli_cells_per_second_per_level, 0);
        assert_eq!(
            role.health_bonus_per_level_per_thousand,
            if military_roles.contains(id.as_str()) {
                10
            } else {
                5
            }
        );
    }
    assert_eq!(content.progression.maximum_role_level, 1_000);
    assert_eq!(content.progression.level_announcement_interval, 10);
}

#[test]
fn authored_research_ladders_grow_by_one_and_a_half() {
    let content = embedded_content();
    let mut ladders = BTreeMap::<(String, String), BTreeMap<u16, u64>>::new();
    let mut referenced = BTreeSet::new();
    for node in content.technology.nodes.values() {
        referenced.extend(node.objectives.iter().cloned());
    }
    for (id, objective) in &content.objectives {
        let parts = id.as_str().split(':').collect::<Vec<_>>();
        if parts.len() < 6
            || parts[0] != "objective"
            || parts[1] != "balance"
            || !matches!(parts[2], "building" | "role")
        {
            continue;
        }
        let rung = parts[4].parse::<u16>().unwrap();
        *ladders
            .entry((parts[2].to_owned(), parts[3].to_owned()))
            .or_default()
            .entry(rung)
            .or_default() += u64::from(objective.required_amount);
        assert!(
            referenced.contains(id),
            "balanced objective {id} is orphaned"
        );
    }
    assert!(!ladders.is_empty());
    for ((kind, name), rungs) in ladders {
        assert!(rungs.len() > 1, "{kind} {name} has no research ladder");
        let amounts = rungs.values().copied().collect::<Vec<_>>();
        for pair in amounts.windows(2) {
            assert_eq!(
                pair[1],
                pair[0].saturating_mul(3).div_ceil(2),
                "{kind} {name} research is not exponential"
            );
        }
    }

    let nursery = &content.technology.nodes[&StableId::new("tech:native_nursery").unwrap()];
    assert_eq!(nursery.objectives.len(), 1);
    assert_eq!(
        content.objectives[&nursery.objectives[0]].required_amount,
        25_000
    );
}

#[test]
fn multi_level_upgrade_charges_each_authored_target_rung_atomically() {
    let content = embedded_content();
    let building_id = StableId::new("building:house").unwrap();
    let definition = &content.buildings[&building_id];
    let runtime_id = StableId::new("building:multi_upgrade_house").unwrap();
    let mut simulation = WorldSimulation::new(42);
    simulation.unlocked_technology.extend(
        content
            .technology
            .nodes
            .iter()
            .filter(|(_, node)| node.building_level_caps.contains_key(&building_id))
            .map(|(id, _)| id.clone()),
    );
    assert!(maximum_building_level(&content, &simulation, &building_id) >= 4);
    for resource in definition.cost.keys() {
        simulation
            .town_resources
            .insert(resource.clone(), 1_000_000);
    }
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id.clone(),
            archetype: definition.archetype.clone(),
            position: GridPos { x: 1, z: 1 },
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(&content, definition)).unwrap(),
            complete: true,
        },
    );
    let expected = building_upgrade_cost(&content, &simulation, &building_id, definition, 1, 4);
    for (resource, base) in &definition.cost {
        assert_eq!(expected[resource], base.saturating_mul(3));
    }
    let before = simulation.town_resources.clone();
    assert_eq!(
        upgrade_building_instance(&content, &mut simulation, &building_id, &runtime_id, 3).unwrap(),
        4
    );
    assert_eq!(simulation.buildings[&runtime_id].level, 4);
    for (resource, amount) in expected {
        assert_eq!(
            simulation.town_resources[&resource],
            before[&resource] - amount
        );
    }

    let mut odd_cost = definition.clone();
    odd_cost.cost = BTreeMap::from([(StableId::new("resource:wood").unwrap(), 1)]);
    assert_eq!(
        building_upgrade_cost(&content, &simulation, &building_id, &odd_cost, 1, 4)
            [&StableId::new("resource:wood").unwrap()],
        4,
        "each target-level price must be rounded independently"
    );
}

#[test]
fn authored_building_level_effects_drive_runtime_stats() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut simulation = WorldSimulation::new(42);
    let add_building = |simulation: &mut WorldSimulation, name: &str, runtime: &str, level: u16| {
        let definition = &content.buildings[&StableId::new(format!("building:{name}")).unwrap()];
        let id = StableId::new(format!("building:{runtime}")).unwrap();
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id: id.clone(),
                archetype: definition.archetype.clone(),
                position: GridPos { x: 10, z: 10 },
                rotation_quarter_turns: 0,
                level,
                health: i32::try_from(building_max_health_at_level(
                    &content,
                    &definition.archetype,
                    level,
                ))
                .unwrap(),
                complete: true,
            },
        );
        id
    };

    let food_storage = add_building(&mut simulation, "foodstorage", "effect_storage", 3);
    let food = StableId::new("resource:food").unwrap();
    assert_eq!(
        resource_storage_capacity(&config, &content, &simulation, &food),
        config.gameplay.base_town_resource_capacity[&food] + 20_000
    );
    assert!(simulation.buildings.contains_key(&food_storage));

    add_building(&mut simulation, "house", "effect_house", 3);
    let station = add_building(&mut simulation, "lumbermill", "effect_lumbermill", 3);
    let actor_id = StableId::new("twitch:building_effect_logger").unwrap();
    let logger = StableId::new("role:logger").unwrap();
    assert!(simulation.join_player(actor_id.clone(), GridPos { x: 1, z: 1 }));
    simulation.assign_role(&actor_id, logger.clone()).unwrap();
    simulation.actors.get_mut(&actor_id).unwrap().station = Some(station);
    let entry_stats =
        effective_role_stats(&content, &simulation, &simulation.actors[&actor_id]).unwrap();
    assert_eq!(entry_stats.action_milliseconds, 990);
    assert_eq!(entry_stats.experience_multiplier_per_thousand, 1_100);
    simulation
        .actors
        .get_mut(&actor_id)
        .unwrap()
        .role_progression
        .insert(
            logger.clone(),
            stream_town_domain::RoleProgress {
                level: 101,
                experience: 0,
            },
        );
    let logger_stats =
        effective_role_stats(&content, &simulation, &simulation.actors[&actor_id]).unwrap();
    assert_eq!(
        logger_stats.max_health,
        leveled_percentage_stat(
            content.roles[&logger].base_health,
            content.roles[&logger].health_bonus_per_level_per_thousand,
            101,
        )
    );

    let defender = StableId::new("role:defender").unwrap();
    simulation.assign_role(&actor_id, defender.clone()).unwrap();
    simulation
        .actors
        .get_mut(&actor_id)
        .unwrap()
        .role_progression
        .insert(
            defender.clone(),
            stream_town_domain::RoleProgress {
                level: 101,
                experience: 0,
            },
        );
    let defender_stats =
        effective_role_stats(&content, &simulation, &simulation.actors[&actor_id]).unwrap();
    assert_eq!(
        defender_stats.max_health,
        content.roles[&defender].base_health.saturating_mul(2)
    );

    let tower = add_building(&mut simulation, "tower", "effect_tower", 6);
    let tower_definition = &content.buildings[&StableId::new("building:tower").unwrap()];
    let shooter = tower_definition.projectile_shooter.as_ref().unwrap();
    let enemy_archetype = content
        .archetypes
        .iter()
        .find(|(_, archetype)| archetype.kind == ArchetypeKind::Enemy && archetype.enemy.is_some())
        .map(|(id, _)| id.clone())
        .unwrap();
    let enemy = StableId::new("actor:building_effect_enemy").unwrap();
    assert!(simulation.spawn_enemy(enemy, enemy_archetype.clone(), GridPos { x: 11, z: 11 }, 5));
    let (projectile, _) = tower_projectile_for_building(&content, &simulation, &tower).unwrap();
    assert_eq!(
        projectile.damage,
        leveled_percentage_stat(shooter.damage, 200, 6)
    );

    add_building(&mut simulation, "guardhouse", "effect_guardhouse", 3);
    simulation
        .actors
        .get_mut(&actor_id)
        .unwrap()
        .role_progression
        .insert(
            defender,
            stream_town_domain::RoleProgress {
                level: 1,
                experience: 0,
            },
        );
    let kill_target = StableId::new("actor:building_effect_kill_target").unwrap();
    assert!(simulation.spawn_enemy(
        kill_target.clone(),
        enemy_archetype,
        GridPos { x: 1, z: 2 },
        1,
    ));
    assert!(!is_level_capped_npc_id(&actor_id));
    assert_eq!(simulation.actors[&actor_id].role.as_str(), "role:defender");
    assert!(military_kill_experience_bonus_per_thousand(&content, &simulation) > 0);
    assert!(
        apply_combat_damage(
            &config,
            &mut simulation,
            &content,
            Some(&actor_id),
            &kill_target,
            1,
        )
        .unwrap()
    );
    let progress = role_progress(&simulation.actors[&actor_id]);
    assert!(progress.level > 1 || progress.experience > 0);
}

#[test]
fn every_building_visual_is_capped_by_town_age() {
    let content = embedded_content();
    let town_hall = StableId::new("building:townhall").unwrap();
    let town_age_technology = content
        .technology
        .nodes
        .iter()
        .find(|(_, node)| node.aged_buildings.contains(&town_hall))
        .map(|(technology, _)| technology.clone())
        .expect("converted technology contains the town age unlock");
    let mut simulation = WorldSimulation::new(42);
    let mut pending_buildings = BTreeSet::new();
    for (technology, node) in &content.technology.nodes {
        if node.aged_buildings.contains(&town_hall) {
            continue;
        }
        if !node.aged_buildings.is_empty() {
            simulation.unlocked_technology.insert(technology.clone());
            pending_buildings.extend(node.aged_buildings.iter().cloned());
        }
    }

    for (building_id, definition) in &content.buildings {
        let capped_age = building_age(&content, &simulation, building_id, 100);
        assert_eq!(
            capped_age, 1,
            "{building_id} escaped the town's age-one visual cap"
        );
        if let Some(scene) =
            archetype_scene_for_age(&content.archetypes[&definition.archetype], capped_age)
        {
            assert!(
                scene.age.is_none_or(|scene_age| scene_age <= capped_age),
                "{building_id} selected future scene {}",
                scene.asset_path
            );
        }
        let state = BuildingState {
            id: StableId::new(format!("building:age_cap_{}", building_id.as_str())).unwrap(),
            archetype: definition.archetype.clone(),
            position: GridPos { x: 0, z: 0 },
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        };
        for future_model in definition
            .model_handlers
            .iter()
            .filter(|model| model.age > capped_age)
        {
            assert_eq!(
                building_node_visibility(
                    building_id,
                    definition,
                    &state,
                    capped_age,
                    &future_model.full_model,
                    &GameConfig::default(),
                    &content,
                    &simulation,
                ),
                Some(false),
                "{building_id} exposed a future-age model node"
            );
        }
        for future_storage in definition
            .storage_models
            .iter()
            .filter(|model| model.age > capped_age)
        {
            assert_eq!(
                building_node_visibility(
                    building_id,
                    definition,
                    &state,
                    capped_age,
                    &future_storage.full_model,
                    &GameConfig::default(),
                    &content,
                    &simulation,
                ),
                Some(false),
                "{building_id} exposed a future-age storage node"
            );
        }
    }

    simulation.unlocked_technology.insert(town_age_technology);
    for (building_id, definition) in &content.buildings {
        let unlocked_age = building_age(&content, &simulation, building_id, 100);
        let expected_age = if building_id == &town_hall || pending_buildings.contains(building_id) {
            2
        } else {
            1
        };
        assert_eq!(
            unlocked_age, expected_age,
            "{building_id} did not preserve its own pending-upgrade state"
        );
        if let Some(scene) =
            archetype_scene_for_age(&content.archetypes[&definition.archetype], unlocked_age)
        {
            assert!(
                scene.age.is_none_or(|scene_age| scene_age <= unlocked_age),
                "{building_id} selected future scene {}",
                scene.asset_path
            );
        }
    }
}

#[test]
fn building_visual_signatures_change_only_for_visible_state_transitions() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut simulation = WorldSimulation::new(42);
    let ore_storage = StableId::new("building:orestorage").unwrap();
    let ore_resource = StableId::new("resource:ore").unwrap();
    let runtime_id = StableId::new("building:test_ore_storage").unwrap();
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id.clone(),
            archetype: content.buildings[&ore_storage].archetype.clone(),
            position: GridPos { x: 1, z: 1 },
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );

    let structure = building_visual_structure_signature(&content, &simulation);
    simulation.buildings.get_mut(&runtime_id).unwrap().complete = false;
    assert_ne!(
        building_visual_structure_signature(&content, &simulation),
        structure,
        "completion must switch construction materials even at maximum health"
    );
    simulation.buildings.get_mut(&runtime_id).unwrap().complete = true;
    simulation.buildings.get_mut(&runtime_id).unwrap().health /= 2;
    assert_eq!(
        building_visual_structure_signature(&content, &simulation),
        structure,
        "damage does not change the model of an already completed building"
    );
    simulation.buildings.get_mut(&runtime_id).unwrap().level += 1;
    assert_ne!(
        building_visual_structure_signature(&content, &simulation),
        structure,
        "level changes must invalidate building presentation"
    );

    let capacity = resource_storage_capacity(&config, &content, &simulation, &ore_resource);
    simulation
        .town_resources
        .insert(ore_resource.clone(), capacity / 4);
    let low_fill = building_model_visibility_signature(&config, &content, &simulation);
    simulation
        .town_resources
        .insert(ore_resource.clone(), capacity / 3);
    assert_eq!(
        building_model_visibility_signature(&config, &content, &simulation),
        low_fill,
        "ordinary resource changes inside one visible fill bucket must not rescan models"
    );
    simulation.town_resources.insert(ore_resource, capacity / 2);
    assert_ne!(
        building_model_visibility_signature(&config, &content, &simulation),
        low_fill,
        "crossing a visible storage fill threshold must invalidate model nodes"
    );
}

#[test]
fn environment_palette_covers_every_season_and_weather() {
    let terrain = stream_town_domain::TerrainAppearanceConfig::default();
    assert_eq!(parse_weather("SNOW"), Some(Weather::Snow));
    assert_eq!(parse_weather("unknown"), None);
    let seasons = [
        Season::Spring,
        Season::Summer,
        Season::Autumn,
        Season::Winter,
    ];
    let weather = [Weather::Clear, Weather::Rain, Weather::Fog, Weather::Snow];
    for season in seasons {
        for weather in weather {
            let palette = environment_palette(season, weather, &terrain);
            assert!(palette.fog_start >= 0.0);
            assert!(palette.fog_end > palette.fog_start);
            assert!(palette.sun_illuminance > 0.0);
            assert!(palette.ambient_brightness > 0.0);
        }
    }
    assert_eq!(
        environment_palette(Season::Spring, Weather::Clear, &terrain).particle_count,
        0
    );
    assert_eq!(
        environment_palette(Season::Spring, Weather::Rain, &terrain).particle_count,
        180
    );
    assert_eq!(
        environment_palette(Season::Winter, Weather::Snow, &terrain).particle_count,
        150
    );
    let spring = environment_palette(Season::Spring, Weather::Clear, &terrain).terrain_tint;
    let winter_palette = environment_palette(Season::Winter, Weather::Clear, &terrain);
    let winter = winter_palette.terrain_tint;
    assert!(
        spring
            .iter()
            .zip(winter)
            .any(|(spring, winter)| (spring - winter).abs() > 0.1)
    );
    assert!((winter_palette.terrain_winter_strength - 1.0).abs() <= f32::EPSILON);
    assert!((winter[0] - winter[1]).abs() < 0.05);
    assert!((winter[1] - winter[2]).abs() < 0.05);
    assert_eq!(weather_particle_seed(42, 7), weather_particle_seed(42, 7));
    assert_ne!(weather_particle_seed(42, 7), weather_particle_seed(42, 8));
}

#[test]
fn building_damage_value_matches_unity_health_percentage() {
    assert!((building_damage_value(25, 25) - 1.0).abs() < f32::EPSILON);
    assert!((building_damage_value(150, 300) - 0.5).abs() < f32::EPSILON);
    assert!((building_damage_value(0, 25) - 0.0).abs() < f32::EPSILON);
    assert!((building_damage_value(-100, 25) - 0.0).abs() < f32::EPSILON);
    assert!((building_damage_value(600, 300) - 1.0).abs() < f32::EPSILON);
}

#[test]
fn building_effects_preserve_authored_vfx_contracts() {
    assert!((BUILDING_HIT_SECONDS - 0.5).abs() < f32::EPSILON);
    assert!((BUILDING_HIT_SMOKE_SPEED - 3.0).abs() < f32::EPSILON);
    assert!((BUILDING_HIT_SPARK_SPEED - 12.0).abs() < f32::EPSILON);
    assert!((BUILDING_HIT_SMOKE_SIZE - 0.5).abs() < f32::EPSILON);
    assert!((BUILDING_HIT_SPARK_SIZE - 0.25).abs() < f32::EPSILON);
    assert!((BUILDING_LEVEL_UP_SECONDS - 1.5).abs() < f32::EPSILON);
    assert!((BUILDING_LEVEL_UP_ARROW_SIZE - 0.5).abs() < f32::EPSILON);
    assert!((BUILDING_LEVEL_UP_TILE_SIZE - 4.0).abs() < f32::EPSILON);
    assert!((BUILDING_DAMAGED_RADIUS - 1.403_639_8).abs() < f32::EPSILON);
    assert_eq!(BUILDING_DAMAGED_FIRE_AMOUNT, 128);
    assert_eq!(BUILDING_DAMAGED_SMOKE_AMOUNT, 200);
    assert!(building_damage_intensity(25, 25, true).abs() < f32::EPSILON);
    assert!(building_damage_intensity(150, 300, false).abs() < f32::EPSILON);
    assert!(building_damage_intensity(150, 300, true) > 0.0);
    assert!(building_damage_intensity(0, 25, true).abs() < f32::EPSILON);
}

#[test]
fn production_resource_kinds_resolve_converted_visuals() {
    let content = embedded_content();
    let cases = [
        (
            "resource:wood",
            "Assets/Prefabs/Dummy Assets/Env_Tree.prefab",
            "Env_Tree.glb",
        ),
        (
            "resource:ore",
            "Assets/Prefabs/Resources/Resource_Ore_Base.prefab",
            "Env_Ore.glb",
        ),
        (
            "resource:food",
            "Assets/Prefabs/Resources/Resource_Bush_Base.prefab",
            "Env_Bush.glb",
        ),
    ];
    for (kind, expected_source, expected_asset) in cases {
        let kind = StableId::new(kind).unwrap();
        let archetype = resource_visual_archetype(&content, &kind).unwrap();
        assert_eq!(archetype.source_path, expected_source);
        let scene = default_archetype_scene(archetype).unwrap();
        assert!(scene.asset_path.ends_with(expected_asset));
    }
    assert!(
        resource_visual_archetype(&content, &StableId::new("resource:fish").unwrap()).is_none()
    );
}

#[test]
fn production_resource_glbs_expose_unity_masks_as_color_zero() {
    for name in ["Env_Tree.glb", "Env_Ore.glb", "Env_Bush.glb"] {
        let path = locate_asset_root()
            .join("shipping/models/Models/Resources")
            .join(name);
        let bytes = std::fs::read(&path).unwrap();
        let json_length = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
        let json = &bytes[20..20 + json_length];
        let json_end = json
            .iter()
            .rposition(|byte| !byte.is_ascii_whitespace() && *byte != 0)
            .unwrap()
            + 1;
        let document: serde_json::Value = serde_json::from_slice(&json[..json_end]).unwrap();
        for primitive in document["meshes"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|mesh| mesh["primitives"].as_array().unwrap())
        {
            let attributes = primitive["attributes"].as_object().unwrap();
            assert!(attributes.contains_key("COLOR_0"), "{name} lacks COLOR_0");
            assert!(
                attributes
                    .keys()
                    .all(|key| !key.starts_with("COLOR_") || key == "COLOR_0"),
                "{name} contains an unsupported secondary color semantic"
            );
        }
    }
}

#[test]
fn resource_variants_and_scale_are_deterministic() {
    let resource = |kind: &str, x, z| stream_town_domain::GeneratedResource {
        id: StableId::new(format!("resource:{x}:{z}")).unwrap(),
        kind: StableId::new(kind).unwrap(),
        target_kind: StableId::new(match kind {
            "resource:wood" => "target:tree",
            "resource:ore" => "target:ore",
            _ => "target:bush",
        })
        .unwrap(),
        position: GridPos { x, z },
        offset_milli_cells: [0, 0],
        generation_occupancy: [i16::try_from(x).unwrap(), i16::try_from(z).unwrap()],
        amount: 100,
    };
    let config = GameConfig::default();
    let wood = resource("resource:wood", 2, 4);
    let wood_variant = resource_mesh_index(&wood, &config);
    assert!(wood_variant < 2);
    assert_eq!(resource_mesh_index(&wood, &config), wood_variant);
    assert!(resource_mesh_index(&resource("resource:ore", 9, 6), &config) < 2);
    assert_eq!(
        resource_mesh_index(&resource("resource:food", 9, 6), &config),
        0
    );
    assert!((resource_visual_scale(2.0) - 0.01).abs() < f32::EPSILON);
    assert!((resource_visual_scale(12.0) - 0.06).abs() < f32::EPSILON);
    let content = embedded_content();
    let tree =
        resource_visual_archetype(&content, &StableId::new("resource:wood").unwrap()).unwrap();
    let centred = centred_resource_visual_position(Vec3::ZERO, tree, 2.0);
    assert!((centred.x + tree.bounds.center[0]).abs() < f32::EPSILON);
    assert!((centred.z - tree.bounds.center[2]).abs() < f32::EPSILON);
}

#[test]
fn baked_menu_tree_variants_override_legacy_checkerboard_indices() {
    let reference: MainMenuSceneReference =
        ron::from_str(include_str!("../../../../assets/content/main_menu_scene.ron")).unwrap();
    let resources = &reference.corrective_bake.unwrap().resources;
    let trees = resources
        .iter()
        .filter(|resource| resource.kind.as_str() == "resource:wood")
        .collect::<Vec<_>>();
    let mut counts = [0_usize; 2];
    let mut corrected = 0_usize;
    for tree in &trees {
        let variant =
            resource_visual_variant(tree.position[0], tree.position[2], &tree.kind, counts.len());
        counts[variant] += 1;
        corrected += usize::from(variant != usize::from(tree.mesh_index));
    }

    assert!(trees.len() > 100);
    assert!(counts.into_iter().all(|count| count > trees.len() / 3));
    assert!(corrected > trees.len() / 3);
}

#[test]
fn tree_seasons_match_unity_material_targets() {
    assert_eq!(
        tree_season_controls(Season::Spring),
        Vec4::new(0.0, 0.0, 0.1, 0.0)
    );
    assert_eq!(tree_season_controls(Season::Summer), Vec4::ZERO);
    assert_eq!(
        tree_season_controls(Season::Autumn),
        Vec4::new(0.3, 0.0, 0.0, 0.0)
    );
    assert_eq!(
        tree_season_controls(Season::Winter),
        Vec4::new(0.0, 0.5, 0.0, 0.0)
    );
    assert!(TreeMaterialExtension::enable_shadows());
    let world_material = tree_material(&embedded_presentation(), None);
    assert!(world_material.base.double_sided);
    assert!(world_material.base.cull_mode.is_none());
    let menu_material = menu_tree_material();
    assert!(menu_material.double_sided);
    assert!(menu_material.cull_mode.is_none());
    assert!(world_material.extension.parameters.wind_controls.y > 0.0);
    let shader = include_str!("../../../../assets/shaders/tree_material.wgsl");
    let prepass = include_str!("../../../../assets/shaders/tree_material_prepass.wgsl");
    let shared_wind = include_str!("../../../../assets/shaders/tree_wind.wgsl");
    assert!(shader.contains("get_world_from_local(in.instance_index)[3].xz"));
    assert!(shader.contains("0.3 * max(1.0 - vertex_color.b, vertex_color.r)"));
    assert!(shader.contains("stream_town_tree_deformed_position"));
    assert!(prepass.contains("stream_town_tree_deformed_position"));
    assert!(!prepass.contains("TreeMaterialUniform"));
    assert!(shared_wind.contains("let wind_strength = 0.79"));
    assert!(shared_wind.contains("let sync = 0.7"));
}

#[test]
fn grass_seasons_match_unity_material_targets() {
    let (spring_1, spring_2, spring_wind, spring_power, spring_tint) =
        grass_season_controls(Season::Spring);
    assert_eq!(
        spring_1,
        unity_shader_color([0.282_352_95, 0.482_352_94, 0.149_019_61, 0.0])
    );
    assert_eq!(
        spring_2,
        unity_shader_color([0.262_745_1, 0.431_372_55, 0.129_411_77, 0.0])
    );
    assert_eq!(
        spring_wind,
        unity_shader_color([0.315_821_56, 0.518, 0.187_516, 0.0])
    );
    assert_eq!(spring_power.to_bits(), 0.1_f32.to_bits());
    assert_eq!(spring_tint.to_bits(), 0.0_f32.to_bits());

    let (winter_1, winter_2, winter_wind, winter_power, winter_tint) =
        grass_season_controls(Season::Winter);
    assert_eq!(
        winter_1,
        unity_shader_color([0.849_056_6, 0.849_056_6, 0.849_056_6, 0.0])
    );
    assert_eq!(
        winter_2,
        unity_shader_color([0.772_549_03, 0.772_549_03, 0.772_549_03, 0.0])
    );
    assert_eq!(winter_wind, unity_shader_color([0.965, 0.965, 0.965, 0.0]));
    assert_eq!(winter_power.to_bits(), 0.0_f32.to_bits());
    assert_eq!(winter_tint.to_bits(), 0.42_f32.to_bits());

    assert_ne!(
        grass_season_controls(Season::Summer),
        grass_season_controls(Season::Autumn)
    );
}

#[test]
fn grass_material_preserves_authored_wind_and_texture_contract() {
    let material = grass_material(&embedded_presentation(), None);
    let parameters = material.extension.parameters;
    assert_eq!(parameters.wind_controls, Vec4::new(0.1, 0.0, 1.1, 20.0));
    assert_eq!(parameters.surface_controls, Vec4::new(1.55, 0.0, 0.0, 1.0));
    assert_eq!(
        parameters.wind_direction_smoothness,
        Vec4::new(1.0, 0.0, -0.28, 0.66)
    );
    assert!(material.extension.main_texture.is_none());
    assert!(material.extension.noise_texture.is_none());
    assert!(GrassMaterialExtension::enable_shadows());
    let prepass = include_str!("../../../../assets/shaders/grass_material_prepass.wgsl");
    assert!(prepass.contains("deformed_grass_position"));
    assert!(prepass.contains("previous_world_position"));
    assert!(prepass.contains("@group(0) @binding(1) var<uniform> globals"));
    assert!(!prepass.contains("@binding(100)"));
}

#[test]
fn critter_material_preserves_authored_vertex_animation_contract() {
    let material = critter_material(&embedded_presentation(), None);
    assert_eq!(
        material.extension.parameters.animation_controls,
        Vec4::new(5.5, 3.58, 0.3, 0.0)
    );
    assert_eq!(
        material.extension.parameters.main_scale_offset,
        Vec4::new(1.0, 1.0, 0.0, 0.0)
    );
    assert!(material.extension.main_texture.is_none());
    assert!(material.base.base_color_texture.is_none());
    assert!(CritterMaterialExtension::enable_shadows());
    let prepass = include_str!("../../../../assets/shaders/critter_material_prepass.wgsl");
    assert!(prepass.contains("deformed_critter_position"));
    assert!(prepass.contains("textureSampleLevel"));
    assert!(prepass.contains("previous_world_position"));
    assert!(prepass.contains("globals.time - globals.delta_time"));
}

#[test]
fn rain_near_camera_is_culled_without_hiding_other_weather() {
    let camera = Vec3::new(2.0, 8.0, -4.0);
    assert!(!weather_particle_visible(
        Weather::Rain,
        camera + Vec3::X * (RAIN_CAMERA_CULL_DISTANCE - 0.1),
        camera,
    ));
    assert!(weather_particle_visible(
        Weather::Rain,
        camera + Vec3::X * RAIN_CAMERA_CULL_DISTANCE,
        camera,
    ));
    assert!(weather_particle_visible(Weather::Snow, camera, camera));
}

#[test]
fn world_labels_stay_below_the_header_and_inside_the_viewport() {
    let viewport = Vec2::new(1_920.0, 1_080.0);
    assert!(!overlay_position_clears_hud(
        viewport,
        Vec2::new(960.0, WORLD_UI_SAFE_TOP + WORLD_UI_OVERLAY_MARGIN - 1.0,),
    ));
    assert!(overlay_position_clears_hud(
        viewport,
        Vec2::new(960.0, WORLD_UI_SAFE_TOP + WORLD_UI_OVERLAY_MARGIN),
    ));
    assert!(!overlay_position_clears_hud(
        viewport,
        Vec2::new(
            960.0,
            1_080.0 - WORLD_UI_SAFE_BOTTOM - WORLD_UI_OVERLAY_MARGIN + 1.0,
        ),
    ));
}

#[test]
fn character_material_preserves_authored_albedo_and_cosmetic_contract() {
    let presentation = embedded_presentation();
    for (path, expected) in [
        (
            "Assets/Materials/Character/Hair.mat",
            [0.018_867_91, 0.018_867_91, 0.018_867_91, 1.0],
        ),
        (
            "Assets/Materials/Character/Eyes.mat",
            [0.132_075_49, 0.097_810_62, 0.097_810_62, 1.0],
        ),
    ] {
        let authored = presentation
            .materials
            .values()
            .find(|material| material.source_path == path)
            .unwrap();
        assert_eq!(
            authored.shader_source.as_deref(),
            Some(CHARACTER_UNITY_SHADER_PATH)
        );
        let material = character_material(authored, &presentation, None);
        let expected = Color::srgba(expected[0], expected[1], expected[2], expected[3])
            .to_linear()
            .to_f32_array();
        assert_eq!(
            material.extension.parameters.albedo_color,
            Vec4::from_array(expected)
        );
        assert_eq!(material.base.base_color, Color::WHITE);
        assert_eq!(material.base.alpha_mode, AlphaMode::Opaque);
        assert_eq!(
            material.extension.parameters.shadow_controls,
            Vec4::new(CHARACTER_SHADOW_RECEIVER_NORMAL_OFFSET, 0.0, 0.0, 0.0)
        );
    }

    let shader = include_str!("../../../../assets/shaders/character_material.wgsl");
    assert!(shader.contains("pbr_input.material.base_color"));
    assert!(shader.contains("character_material.albedo_color"));
    assert!(shader.contains("character_material.shadow_controls.x"));
    assert!(shader.contains("var lighting_input = pbr_input"));
    assert!(shader.contains("apply_pbr_lighting"));

    let source = StandardMaterial {
        base_color: Color::srgb(0.2, 0.4, 0.6),
        ..default()
    };
    let expected = source.base_color.to_linear().to_f32_array();
    let converted = character_material_from_standard(source);
    assert_eq!(
        converted.extension.parameters.albedo_color,
        Vec4::from_array(expected)
    );
    assert_eq!(converted.base.base_color, Color::WHITE);
    assert_eq!(
        converted.extension.parameters.shadow_controls,
        Vec4::new(CHARACTER_SHADOW_RECEIVER_NORMAL_OFFSET, 0.0, 0.0, 0.0)
    );
}

#[test]
fn animated_character_receiver_scope_follows_only_the_player_rig_hierarchy() {
    let mut world = World::new();
    let player_root = world.spawn(PlayerAnimatedRig).id();
    let player_mesh = world.spawn(ChildOf(player_root)).id();
    let unrelated_root = world.spawn_empty().id();
    let unrelated_mesh = world.spawn(ChildOf(unrelated_root)).id();
    let mut state = bevy::ecs::system::SystemState::<(
        Query<&ChildOf>,
        Query<Entity, With<PlayerAnimatedRig>>,
    )>::new(&mut world);
    let (parents, animated_player_rigs) = state.get(&world).unwrap();

    assert!(animated_player_renderer(
        player_mesh,
        &parents,
        &animated_player_rigs
    ));
    assert!(!animated_player_renderer(
        unrelated_mesh,
        &parents,
        &animated_player_rigs
    ));
}

#[test]
fn animated_character_renderers_use_the_character_shadow_receiver_path() {
    let mut app = App::new();
    app.insert_resource(CharacterBaseMaterialCache::default());
    app.add_systems(Update, tag_cosmetic_renderers);
    let player_root = app.world_mut().spawn(PlayerAnimatedRig).id();
    let renderer = app
        .world_mut()
        .spawn(MeshMaterial3d::<CharacterMaterial>(Handle::default()))
        .id();
    app.world_mut().entity_mut(player_root).add_child(renderer);

    app.update();

    assert!(
        app.world()
            .get::<AnimatedCharacterShadowReceiver>(renderer)
            .is_some()
    );
    assert!(
        app.world().get::<NoFrustumCulling>(renderer).is_some(),
        "animated character renderers must keep stable skin-buffer slots as cameras move"
    );
    assert!(
        app.world()
            .get::<bevy::light::NotShadowCaster>(renderer)
            .is_none()
    );
    assert!(
        app.world()
            .get::<bevy::light::NotShadowReceiver>(renderer)
            .is_none()
    );
}

#[test]
fn flag_material_preserves_authored_wind_colour_and_metal_contract() {
    let material = flag_material(&embedded_presentation(), None);
    assert_eq!(
        material.extension.parameters.colour_1,
        unity_shader_color([1.0, 0.835_294_1, 0.0, 0.0])
    );
    assert_eq!(
        material.extension.parameters.colour_2,
        unity_shader_color([1.0, 0.023_529_41, 0.023_529_41, 0.0])
    );
    assert_eq!(
        material.extension.parameters.controls,
        Vec4::new(0.49, 0.27, 0.14, 0.7)
    );
    assert_eq!(
        material.extension.parameters.noise_scale_offset,
        Vec4::new(1.0, 1.0, 0.0, 0.0)
    );
    assert!(material.extension.noise_texture.is_none());

    let shader = include_str!("../../../../assets/shaders/flag_material.wgsl");
    assert!(shader.contains("(1.0 - vertex_color.a)"));
    assert!(shader.contains("flag_material.colour_2"));
    assert!(shader.contains("pbr_input.material.metallic = metal_edge"));
}

#[test]
fn godray_material_preserves_authored_vertex_colour_emission_and_alpha_contract() {
    let material = godray_material(&embedded_presentation());
    assert_eq!(
        material.extension.parameters.emission_alpha,
        Vec4::new(0.06, 1.64, 0.0, 0.0)
    );
    assert_eq!(material.base.alpha_mode, AlphaMode::Blend);
    assert!(material.base.cull_mode.is_none());

    let shader = include_str!("../../../../assets/shaders/godray_material.wgsl");
    assert!(shader.contains("vertex_color.rgb"));
    assert!(shader.contains("vertex_color.a * godray_material.emission_alpha.y"));
    assert!(shader.contains("1.0 + godray_material.emission_alpha.x"));
}

#[test]
fn giraffe_material_preserves_authored_skinned_vertex_animation_contract() {
    let material = giraffe_material(&embedded_presentation(), None);
    let parameters = material.extension.parameters;
    assert_eq!(
        parameters.animation_controls,
        Vec4::new(8.15, 0.9, 2.0, 0.9)
    );
    assert_eq!(parameters.mask_controls, Vec4::new(0.07, 1.2, 0.8, 2.1));
    assert_eq!(parameters.rotation_controls, Vec4::new(0.11, 4.1, 1.8, 0.0));
    assert_eq!(parameters.main_scale_offset, Vec4::new(1.0, 1.0, 0.0, 0.0));
    assert!(material.extension.main_texture.is_none());
    assert!((material.base.perceptual_roughness - 1.0).abs() <= f32::EPSILON);
    assert!(material.base.metallic.abs() <= f32::EPSILON);

    let shader = include_str!("../../../../assets/shaders/giraffe_material.wgsl");
    assert!(shader.contains("skinning::skin_model"));
    assert!(shader.contains("skinning::skin_normals"));
    assert!(shader.contains("smoothstep("));
    assert!(shader.contains("textureSample(main_texture"));
    assert!(!shader.contains("normalize(axis)"));
}

#[test]
fn building_bounds_material_preserves_unity_placement_contract() {
    let presentation = embedded_presentation();
    let authored = bounds_material(&presentation, None);
    assert_eq!(
        authored.extension.parameters.color_alpha,
        unity_shader_color([0.0, 0.867_924_5, 0.137_292_03, 0.568])
    );
    assert_eq!(authored.base.alpha_mode, AlphaMode::Blend);
    let valid = bounds_material(&presentation, Some(BUILDING_PLACEMENT_SUCCESS_COLOR));
    let blocked = bounds_material(&presentation, Some(BUILDING_PLACEMENT_FAIL_COLOR));
    assert_eq!(
        valid.extension.parameters.color_alpha,
        unity_shader_color([
            BUILDING_PLACEMENT_SUCCESS_COLOR[0],
            BUILDING_PLACEMENT_SUCCESS_COLOR[1],
            BUILDING_PLACEMENT_SUCCESS_COLOR[2],
            0.568,
        ])
    );
    assert_eq!(
        blocked.extension.parameters.color_alpha,
        unity_shader_color([
            BUILDING_PLACEMENT_FAIL_COLOR[0],
            BUILDING_PLACEMENT_FAIL_COLOR[1],
            BUILDING_PLACEMENT_FAIL_COLOR[2],
            0.568,
        ])
    );
    let shader = include_str!("../../../../assets/shaders/bounds_material.wgsl");
    assert!(shader.contains("bounds_material.color_alpha"));
    assert!(shader.contains("apply_pbr_lighting"));

    let content = embedded_content();
    let placer = content
        .archetypes
        .values()
        .find(|archetype| {
            archetype
                .source_path
                .ends_with("Prefabs/BuildingPlacer.prefab")
        })
        .unwrap();
    let bounds_id = StableId::new("material:b837afe89d932d14cba0f2178703e9fd").unwrap();
    assert!(presentation.prefab_materials[&placer.source_guid].contains(&bounds_id));
    assert!(
        presentation.prefab_renderer_materials[&placer.source_guid]
            .iter()
            .any(|binding| binding.target_path == "VisualBounds"
                && binding.materials["BoundsVisualizer"] == bounds_id)
    );
}

#[test]
fn selection_outline_preserves_authored_material_texture_and_footprints() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let material_id = StableId::new(SELECTION_MASK_MATERIAL_ID).unwrap();
    let authored = &presentation.materials[&material_id];
    assert_eq!(authored.source_path, "Assets/Materials/SelectionMask.mat");
    assert_eq!(authored.alpha_mode, AuthoredAlphaMode::Mask);
    assert_eq!(
        presentation.textures[&authored.textures["_BaseMap"]].source_path,
        SELECTION_MASK_TEXTURE_PATH
    );
    let material = standard_material(authored, &presentation, None);
    assert_eq!(material.alpha_mode, AlphaMode::Mask(0.5));
    for (actual, expected) in material
        .base_color
        .to_srgba()
        .to_f32_array()
        .into_iter()
        .zip([1.0, 0.983_102, 0.0, 1.0])
    {
        assert!((actual - expected).abs() <= f32::EPSILON);
    }
    assert_eq!(
        material.emissive,
        LinearRgba::new(0.877_358_5, 0.836_833_4, 0.0, 1.0)
    );
    let texture = &presentation.textures[&authored.textures["_BaseMap"]];
    assert!(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../assets")
            .join(&texture.asset_path)
            .is_file()
    );

    let config = GameConfig::default();
    let world = generate_world_with_content(&config.world, &content);
    let mut simulation = WorldSimulation::new(world.seed);
    ensure_town_hall_state(&content, &config, &mut simulation);
    let town_hall_id = StableId::new("building:townhall").unwrap();
    let town_hall = &simulation.buildings[&town_hall_id];
    let footprint = selected_building_footprint(town_hall.position, &content, &simulation)
        .expect("Town Hall origin should resolve its complete footprint");
    let outline = selection_outline_rect(town_hall.position, Some(footprint), false, &config);
    assert_eq!(
        outline.zw(),
        Vec2::new(
            f32::from(footprint.1[0]) * config.world.cell_size * 0.5,
            f32::from(footprint.1[1]) * config.world.cell_size * 0.5,
        )
    );

    let player = selection_outline_rect(town_hall.position, None, true, &config);
    assert_eq!(
        player.zw(),
        Vec2::splat(config.world.cell_size * PLAYER_SELECTION_OUTLINE_SCALE_CELLS * 0.5,)
    );

    let terrain = terrain_material(&presentation, &config, None, None, None);
    assert_eq!(
        terrain.extension.parameters.selection_center_extent,
        Vec4::ZERO
    );
    assert_eq!(terrain.extension.parameters.selection_color, Vec4::ZERO);
}

#[test]
fn selected_building_actions_share_authoritative_upgrade_and_removal_rules() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world_with_content(&config.world, &content);
    let mut simulation = WorldSimulation::new(world.seed);
    ensure_town_hall_state(&content, &config, &mut simulation);
    assert!(
        remove_selected_building(
            &StableId::new("building:townhall").unwrap(),
            &content,
            &mut world,
            &mut simulation,
        )
        .is_err()
    );

    let building_id = StableId::new("building:house").unwrap();
    let definition = &content.buildings[&building_id];
    let position = find_building_site(
        &world,
        GridPos {
            x: config.world.width / 2,
            z: config.world.height / 2,
        },
        definition.footprint,
    )
    .unwrap();
    let runtime_id = StableId::new("building:selected_house").unwrap();
    let region = building_region(position, definition.footprint, &world).unwrap();
    world.navigation.set_blocked(region, true).unwrap();
    simulation.building_costs_enabled = false;
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id.clone(),
            archetype: definition.archetype.clone(),
            position,
            rotation_quarter_turns: 0,
            level: 1,
            health: building_base_max_health(&content, definition)
                .try_into()
                .unwrap(),
            complete: true,
        },
    );
    let selected_cell = GridPos {
        x: position.x.saturating_add(definition.footprint[0] - 1),
        z: position.z.saturating_add(definition.footprint[1] - 1),
    };
    assert_eq!(
        selected_building_id_at_cell(selected_cell, &content, &simulation),
        Some(runtime_id.clone())
    );
    let upgrade =
        upgrade_building_instance(&content, &mut simulation, &building_id, &runtime_id, 1);
    assert!(upgrade.is_err(), "the default technology cap is level one");
    remove_selected_building(&runtime_id, &content, &mut world, &mut simulation).unwrap();
    assert!(!simulation.buildings.contains_key(&runtime_id));
    assert!(world.navigation.is_walkable(position));
}

#[test]
fn building_cost_queries_and_rejections_name_every_missing_resource() {
    let content = embedded_content();
    let simulation = WorldSimulation::new(11);
    let requested = StableId::new("orestorage").unwrap();
    let summary = building_cost_summary(&content, &simulation, &requested).unwrap();
    assert!(summary.contains("build cost:"));
    assert!(summary.contains("maximum level:"));

    let costs = BTreeMap::from([
        (StableId::new("resource:wood").unwrap(), 100),
        (StableId::new("resource:ore").unwrap(), 80),
    ]);
    let message = building_shortage_message(&simulation, "Ore Storage", &costs).unwrap();
    assert!(message.contains("Wood 100 more (have 0/100)"));
    assert!(message.contains("Ore 80 more (have 0/80)"));
    assert!(message.find("Wood 100").unwrap() < message.find("Ore 80").unwrap());
}

#[test]
fn passive_buildings_use_authoritative_income_rates() {
    let content = embedded_content();
    let marketplace_id = StableId::new("building:marketplace").unwrap();
    let marketplace = &content.buildings[&marketplace_id];
    assert_eq!(
        passive_resource_rate_milli_per_second(marketplace, 1),
        BTreeMap::from([(StableId::new("resource:gold").unwrap(), 500)])
    );
    assert_eq!(
        passive_resource_rate_milli_per_second(marketplace, 2),
        BTreeMap::from([(StableId::new("resource:gold").unwrap(), 1_000)])
    );
}
