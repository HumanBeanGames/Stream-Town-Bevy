#[test]
fn placement_visual_switches_typed_bounds_material_for_collision_state() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world_with_content(&config.world, &content);
    let building_id = StableId::new("building:house").unwrap();
    let definition = &content.buildings[&building_id];
    let valid_position = find_building_site(
        &world,
        GridPos {
            x: config.world.width / 2,
            z: config.world.height / 2,
        },
        definition.footprint,
    )
    .unwrap();
    let mut bounds_materials = Assets::<BoundsMaterial>::default();
    let valid_handle = bounds_materials.add(bounds_material(&embedded_presentation(), None));
    let invalid_handle = bounds_materials.add(bounds_material(&embedded_presentation(), None));
    let render = RenderAssets {
        placement_valid: valid_handle.clone(),
        placement_invalid: invalid_handle.clone(),
        ..default()
    };
    let mut transform = Transform::default();
    let mut material = MeshMaterial3d(valid_handle.clone());
    let mut placement = BuildingPlacement {
        building: building_id,
        thick_path: false,
        position: valid_position,
        navigation_position: None,
        rotation_quarter_turns: 0,
        line_start: None,
        line_end: None,
        path_cells: Vec::new(),
        inactivity_seconds: 0.0,
    };
    update_placer_visual(
        &config,
        &world,
        &render,
        &placement,
        definition,
        true,
        &mut transform,
        &mut material,
    );
    assert_eq!(material.0.id(), valid_handle.id());
    assert!((transform.scale.y - config.world.cell_size * 0.02).abs() <= f32::EPSILON);

    placement.position = GridPos {
        x: config.world.width - 1,
        z: config.world.height - 1,
    };
    update_placer_visual(
        &config,
        &world,
        &render,
        &placement,
        definition,
        false,
        &mut transform,
        &mut material,
    );
    assert_eq!(material.0.id(), invalid_handle.id());
}

#[test]
fn placement_ghost_uses_runtime_building_alignment_and_validity_colour() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world_with_content(&config.world, &content);
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
    let owner = StableId::new("twitch:ghost").unwrap();
    let placement = BuildingPlacement {
        building: building_id,
        thick_path: false,
        position,
        navigation_position: None,
        rotation_quarter_turns: 1,
        line_start: None,
        line_end: None,
        path_cells: Vec::new(),
        inactivity_seconds: 0.0,
    };
    let mut transform = Transform::default();
    update_placer_ghost_transform(&config, &world, &placement, definition, &mut transform);
    let effective = rotated_footprint(definition.footprint, 1);
    let centre = GridPos {
        x: position.x + effective[0] / 2,
        z: position.z + effective[1] / 2,
    };
    assert_eq!(
        transform.translation,
        grid_to_world_on_surface(centre, &config, &world)
    );
    assert_eq!(transform.scale, Vec3::splat(config.world.cell_size / 2.0));
    assert_eq!(transform.rotation, quarter_turn_rotation(1));

    let mut bounds_materials = Assets::<BoundsMaterial>::default();
    let valid = bounds_materials.add(bounds_material(&embedded_presentation(), None));
    let invalid = bounds_materials.add(bounds_material(&embedded_presentation(), None));
    let render = RenderAssets {
        placement_valid: valid.clone(),
        placement_invalid: invalid,
        ..default()
    };
    let simulation = WorldSimulation::new(world.seed);
    let placers = BTreeMap::from([(owner.clone(), placement)]);
    assert_eq!(
        placement_ghost_material(
            &content,
            &simulation,
            &world,
            None,
            &placers,
            &render,
            &owner,
        )
        .id(),
        valid.id()
    );
}

#[test]
fn live_giraffe_pet_resolves_typed_material_and_converted_vertex_masks() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let archetype = content
        .archetypes
        .values()
        .find(|archetype| archetype.source_path.ends_with("Prefabs/Pets/Pet.prefab"))
        .unwrap();
    let pet = StableId::new("pet:giraffe").unwrap();
    let (scene, model) = pet_model(archetype, &pet).unwrap();
    assert!(scene.source_model.ends_with("Pet_TallBoi.fbx"));
    assert!(
        model
            .local_position
            .iter()
            .all(|value| value.abs() < f32::EPSILON)
    );
    assert!(
        model
            .local_scale
            .iter()
            .all(|value| (value - 1.0).abs() < f32::EPSILON)
    );
    let material_id = presentation.model_materials[&scene.source_model]["MainMaterial"].clone();
    assert_eq!(
        presentation.materials[&material_id].source_path,
        GIRAFFE_MATERIAL_PATH
    );

    let mut giraffe_materials = Assets::<GiraffeMaterial>::default();
    let handle = giraffe_materials.add(giraffe_material(&presentation, None));
    let mut render = RenderAssets::default();
    render
        .presentation_materials
        .insert(material_id, ResolvedMaterialHandle::Giraffe(handle.clone()));
    let spec = prefab_material_spec(archetype, scene, &presentation, &render)
        .expect("live pet spawn should carry converted material bindings");
    assert!(matches!(
        &spec.model_materials["MainMaterial"],
        ResolvedMaterialHandle::Giraffe(resolved) if resolved.id() == handle.id()
    ));

    let bytes = std::fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../assets")
            .join(&scene.asset_path),
    )
    .unwrap();
    let json_length = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    let json = &bytes[20..20 + json_length];
    let json_end = json
        .iter()
        .rposition(|byte| !byte.is_ascii_whitespace() && *byte != 0)
        .unwrap()
        + 1;
    let document: serde_json::Value = serde_json::from_slice(&json[..json_end]).unwrap();
    assert_eq!(document["skins"].as_array().unwrap().len(), 1);
    let attributes = document["meshes"][0]["primitives"][0]["attributes"]
        .as_object()
        .unwrap();
    for attribute in ["COLOR_0", "JOINTS_0", "WEIGHTS_0"] {
        assert!(
            attributes.contains_key(attribute),
            "giraffe lacks {attribute}"
        );
    }
}

#[test]
fn animated_pets_resolve_their_own_unity_controllers_and_rigs() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let archetype = content
        .archetypes
        .values()
        .find(|archetype| archetype.source_path.ends_with("Prefabs/Pets/Pet.prefab"))
        .unwrap();
    let cases = [
        (
            "pet:giraffe",
            "controller:7a24ad00bb657d1439fa86e0c2eb6b12",
            "Pet_TallBoi.glb",
        ),
        (
            "pet:red_panda",
            "controller:7b5536b35afda8d41930f35e0e51215a",
            "Pet_RedPanda.glb",
        ),
        (
            "pet:duck",
            "controller:bda358ac13f989345a8f46fd230f9826",
            "Pet_Duck.glb",
        ),
        (
            "pet:butterfly",
            "controller:a1529f2d1d90b8b4198e099687f1bbea",
            "Pet_Butterfly.glb",
        ),
    ];
    for (pet, controller, scene_suffix) in cases {
        let pet = StableId::new(pet).unwrap();
        let (scene, _) = pet_model(archetype, &pet).unwrap();
        let spec = pet_animation_spec(&pet, scene, &presentation).unwrap();
        assert_eq!(spec.controller.as_str(), controller);
        assert!(spec.rig_scene.ends_with(scene_suffix));
        let state = &presentation.controllers[&spec.controller].states[&spec.state];
        assert_eq!(state.display_name, "Special");
    }
    let fish_god = StableId::new("pet:fish_god").unwrap();
    assert!(
        pet_animation_spec(
            &fish_god,
            pet_model(archetype, &fish_god).unwrap().0,
            &presentation
        )
        .is_none()
    );
    let pet = archetype.pet.as_ref().unwrap();
    assert_eq!(pet.models.len(), 5);
    assert!(
        pet.models[&fish_god]
            .local_position
            .iter()
            .zip([0.0, 1.403, -0.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert!(pet.models.values().all(|model| {
        model
            .local_scale
            .iter()
            .all(|value| (value - 1.0).abs() < f32::EPSILON)
    }));
}

#[test]
fn pet_follow_uses_unity_distance_remap_and_rotation() {
    let content = embedded_content();
    let pet = content
        .archetypes
        .values()
        .find_map(|archetype| archetype.pet.as_ref())
        .unwrap();
    assert!((pet.closest_distance - 1.0).abs() < f32::EPSILON);
    assert!((pet.max_distance - 5.0).abs() < f32::EPSILON);
    assert!(pet.min_move_speed.abs() < f32::EPSILON);
    assert!((pet.max_move_speed - 10.0).abs() < f32::EPSILON);
    assert!((pet.rotation_radians_per_second - 5.0).abs() < f32::EPSILON);

    let mut near = Transform::from_translation(Vec3::ZERO);
    let stopped = pet_follow_step(&mut near, Vec3::X, pet, 0.5);
    assert!(stopped.abs() <= f32::EPSILON);
    assert_eq!(near.translation, Vec3::ZERO);

    let mut far = Transform::from_translation(Vec3::ZERO);
    let speed = pet_follow_step(&mut far, Vec3::new(5.0, 0.0, 0.0), pet, 0.1);
    assert!((speed - pet.max_move_speed).abs() <= f32::EPSILON);
    assert_eq!(far.translation, Vec3::X);
    // Unity's Quaternion.Slerp uses delta * rotation speed, so a 100 ms
    // update rotates halfway toward the owner rather than snapping.
    assert!((far.rotation * Vec3::Z).dot(Vec3::X) > 0.7);
}
