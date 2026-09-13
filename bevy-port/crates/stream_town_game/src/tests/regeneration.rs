#[test]
fn nearby_trees_accelerate_a_foresters_active_planting_timer() {
    let (_, _, mut world, simulation, mut runtime, actor, _) =
        regeneration_role_fixture("forester", "nursery");
    let position = simulation.actors[&actor].position;
    world.resources.clear();
    for serial in 0..3 {
        world.resources.push(stream_town_domain::GeneratedResource {
            id: StableId::new(format!("resource:test_tree_{serial}")).unwrap(),
            kind: StableId::new("resource:wood").unwrap(),
            target_kind: StableId::new("target:tree").unwrap(),
            position,
            offset_milli_cells: [0, 0],
            generation_occupancy: [0, 0],
            amount: 1,
        });
    }
    runtime.elapsed_seconds = 1_000.0;
    runtime.workers.get_mut(&actor).unwrap().next_ready_seconds = 1_100.0;

    accelerate_forester_regeneration_cooldowns(&simulation, &world, &mut runtime, 10.0);

    assert!(
        (runtime.workers[&actor].next_ready_seconds - 1_099.7).abs() < 1.0e-9,
        "three nearby trees should add three percent to the timer's tick rate"
    );
}

#[test]
fn local_resource_saturation_raises_then_decays_the_check_denominator() {
    let (_, _, mut world, simulation, mut runtime, actor, _) =
        regeneration_role_fixture("prospector", "prospector_hut");
    let origin = simulation.actors[&actor].position;
    let template = world
        .resources
        .iter()
        .find(|resource| resource.target_kind.as_str() == "target:ore")
        .cloned()
        .expect("fixture contains ore");
    world
        .resources
        .iter_mut()
        .for_each(|resource| resource.amount = 0);
    for (index, position) in [
        origin,
        offset_grid(origin, 1, 0, &world).unwrap(),
        offset_grid(origin, 0, 1, &world).unwrap(),
    ]
    .into_iter()
    .enumerate()
    {
        let mut ore = template.clone();
        ore.id = StableId::new(format!("resource:saturation_ore_{index}")).unwrap();
        ore.position = position;
        ore.amount = 1;
        world.resources.push(ore);
    }

    let (_, denominator, debuff) = local_resource_saturation_roll(
        &mut runtime,
        &world,
        &actor,
        origin,
        "target:ore",
        0,
        400,
        1,
    );
    assert_eq!((denominator, debuff), (1_600, 3));

    world
        .resources
        .iter_mut()
        .for_each(|resource| resource.amount = 0);
    for (sequence, expected) in [(1, (1_200, 2)), (2, (800, 1)), (3, (400, 0))] {
        let (_, denominator, debuff) = local_resource_saturation_roll(
            &mut runtime,
            &world,
            &actor,
            origin,
            "target:ore",
            sequence,
            400,
            1,
        );
        assert_eq!((denominator, debuff), expected);
    }

    let (_, denominator, debuff) = local_resource_saturation_roll(
        &mut runtime,
        &world,
        &actor,
        origin,
        "target:bush",
        4,
        1,
        2,
    );
    assert_eq!((denominator, debuff), (1, 0));
}

#[test]
fn simultaneous_foresters_reserve_distinct_planting_cells_and_approaches() {
    let (_, content, world, mut simulation, mut runtime, first_actor, _) =
        regeneration_role_fixture("forester", "nursery");
    let from = simulation.actors[&first_actor].position;
    let second_actor = StableId::new("twitch:test_forester_second").unwrap();
    let forester = StableId::new("role:forester").unwrap();
    assert!(simulation.join_player(second_actor.clone(), from));
    simulation
        .assign_role(&second_actor, forester.clone())
        .unwrap();
    simulation
        .actors
        .get_mut(&second_actor)
        .unwrap()
        .role_progression
        .insert(
            forester,
            stream_town_domain::RoleProgress {
                level: 100,
                experience: 0,
            },
        );
    let visited_station = runtime.workers[&first_actor].last_station.clone();
    runtime.workers.insert(
        second_actor.clone(),
        RegenerationWorkerState {
            initialized: true,
            next_ready_seconds: 0.0,
            station_visit_required: false,
            last_station: visited_station,
            ..default()
        },
    );

    let (first_goal, first_approach) = regeneration_agent_goal_with_reservations(
        &content,
        &simulation,
        &world,
        &mut runtime,
        &first_actor,
        from,
        &BTreeMap::new(),
    )
    .expect("the first forester reserves a planting task");
    let AgentGoal::PlantTree(first_plant) = first_goal else {
        panic!("the first forester did not select a tree");
    };
    let reservations = BTreeMap::from([(first_approach, first_actor.clone())]);
    let (second_goal, second_approach) = regeneration_agent_goal_with_reservations(
        &content,
        &simulation,
        &world,
        &mut runtime,
        &second_actor,
        from,
        &reservations,
    )
    .expect("the second forester finds an independently reachable planting task");
    let AgentGoal::PlantTree(second_plant) = second_goal else {
        panic!("the second forester did not select a tree");
    };

    assert_ne!(first_plant, second_plant);
    assert_ne!(first_approach, second_approach);
    assert_eq!(
        runtime.workers[&first_actor].planting_target,
        Some(first_plant)
    );
    assert_eq!(
        runtime.workers[&second_actor].planting_target,
        Some(second_plant)
    );
}

#[test]
fn forester_utility_balances_nursery_distance_ecology_and_direction_memory() {
    let nursery = GridPos { x: 30, z: 30 };
    let east = GridPos { x: 35, z: 30 };
    let west = GridPos { x: 25, z: 30 };
    assert!(
        forester_nursery_distance_utility(nursery, east)
            > forester_nursery_distance_utility(nursery, nursery)
    );
    assert!(
        forester_nursery_distance_utility(nursery, east)
            > forester_nursery_distance_utility(nursery, GridPos { x: 45, z: 30 })
    );

    let mut spatial = RegenerationSpatialIndex {
        resources: HashSet::new(),
        trees: HashSet::new(),
        actors: HashSet::new(),
        buildings: HashSet::new(),
    };
    spatial.trees.insert(GridPos { x: 36, z: 30 });
    let runtime = RegenerationRoleRuntime::default();
    let east_near_tree =
        forester_candidate_utility(&spatial, &runtime, &BTreeSet::new(), nursery, nursery, east);
    let west_away_from_tree =
        forester_candidate_utility(&spatial, &runtime, &BTreeSet::new(), nursery, nursery, west);
    assert!(east_near_tree > west_away_from_tree);

    spatial.trees.clear();
    let mut runtime = RegenerationRoleRuntime {
        elapsed_seconds: 600.0,
        ..default()
    };
    runtime.recently_fallen_trees.push_back(east);
    assert!(
        forester_candidate_utility(&spatial, &runtime, &BTreeSet::new(), nursery, nursery, east,)
            > forester_candidate_utility(
                &spatial,
                &runtime,
                &BTreeSet::new(),
                nursery,
                nursery,
                west,
            )
    );

    runtime.recently_fallen_trees.clear();
    runtime.recent_tree_plantings.push_back(RecentTreePlanting {
        nursery,
        position: east,
        planted_at_seconds: runtime.elapsed_seconds,
    });
    assert!(
        forester_candidate_utility(&spatial, &runtime, &BTreeSet::new(), nursery, nursery, east,)
            < forester_candidate_utility(
                &spatial,
                &runtime,
                &BTreeSet::new(),
                nursery,
                nursery,
                west,
            )
    );
}

#[test]
fn forester_selection_weights_worker_hut_ecology_and_randomness() {
    let nursery = GridPos { x: 30, z: 30 };
    let near_worker = GridPos { x: 32, z: 30 };
    let far_worker = GridPos { x: 45, z: 30 };
    assert!(
        forester_worker_distance_utility(nursery, near_worker)
            > forester_worker_distance_utility(nursery, far_worker)
    );

    let mut spatial = RegenerationSpatialIndex {
        resources: HashSet::new(),
        trees: HashSet::from([GridPos { x: 36, z: 30 }]),
        actors: HashSet::new(),
        buildings: HashSet::new(),
    };
    let actor = StableId::new("twitch:weighted_forester").unwrap();
    let runtime = RegenerationRoleRuntime::default();
    let east = GridPos { x: 35, z: 30 };
    let west = GridPos { x: 25, z: 30 };
    let score = |sequence, candidate| {
        forester_selection_utility(
            &spatial,
            &runtime,
            &BTreeSet::new(),
            nursery,
            nursery,
            nursery,
            91,
            &actor,
            sequence,
            candidate,
        )
    };
    assert!((0..1_000).any(|sequence| score(sequence, east) > score(sequence, west)));
    assert!((0..1_000).any(|sequence| score(sequence, west) > score(sequence, east)));

    spatial.trees.clear();
    assert!(
        forester_nursery_distance_utility(nursery, east)
            > forester_nursery_distance_utility(nursery, far_worker)
    );
}

#[test]
fn forester_random_fallback_is_one_in_ten_and_clear_of_buildings() {
    let (_, content, world, simulation, runtime, actor, hut) =
        regeneration_role_fixture("forester", "nursery");
    let fallback_checks = (0..10_000_u32)
        .filter(|sequence| forester_uses_random_fallback(world.seed, &actor, *sequence))
        .count();
    assert!((900..=1_100).contains(&fallback_checks));

    let sequence = (0..100_u32)
        .find(|sequence| forester_uses_random_fallback(world.seed, &actor, *sequence))
        .expect("a one-in-ten fallback occurs in the first hundred checks");
    let spatial = RegenerationSpatialIndex::new(&content, &world, &simulation);
    let expected = forester_random_planting_cell(
        &content,
        &simulation,
        &world,
        &spatial,
        &actor,
        sequence,
        &BTreeSet::new(),
        &BTreeMap::new(),
        hut,
        simulation.actors[&actor].position,
    )
    .expect("fixture has a valid random planting cell");
    let selected = forester_planting_cell(
        &content,
        &simulation,
        &world,
        &runtime,
        &actor,
        sequence,
        &BTreeSet::new(),
        &BTreeMap::new(),
        hut,
        simulation.actors[&actor].position,
    )
    .expect("fallback check selects a planting cell");
    assert_eq!(selected, expected);
    assert!(cell_is_clear_of_buildings(
        &content,
        &simulation,
        &world,
        selected.0,
        3,
    ));
}

#[test]
fn failed_regeneration_target_search_is_backed_off() {
    let (_, content, mut world, simulation, mut runtime, actor, _) =
        regeneration_role_fixture("forester", "nursery");
    let from = simulation.actors[&actor].position;
    let blocked_world = stream_town_domain::DirtyRegion {
        min: GridPos { x: 0, z: 0 },
        max: GridPos {
            x: world.navigation.width() - 1,
            z: world.navigation.height() - 1,
        },
    };
    world.navigation.set_blocked(blocked_world, true).unwrap();

    assert!(
        regeneration_agent_goal(&content, &simulation, &world, &mut runtime, &actor, from)
            .is_none()
    );
    let retry_at = runtime.workers[&actor].next_ready_seconds;
    assert!(
        (retry_at - (runtime.elapsed_seconds + REGENERATION_TARGET_RETRY_SECONDS)).abs()
            <= f64::EPSILON
    );

    runtime.elapsed_seconds += 1.0;
    assert!(
        regeneration_agent_goal(&content, &simulation, &world, &mut runtime, &actor, from)
            .is_none()
    );
    assert!((runtime.workers[&actor].next_ready_seconds - retry_at).abs() <= f64::EPSILON);
}

#[test]
fn inactive_building_placements_expire_after_one_minute() {
    let mut placement = BuildingPlacement {
        building: StableId::new("building:house").unwrap(),
        thick_path: false,
        position: GridPos { x: 1, z: 1 },
        navigation_position: None,
        rotation_quarter_turns: 0,
        line_start: None,
        line_end: None,
        path_cells: Vec::new(),
        inactivity_seconds: 0.0,
    };
    assert!(building_placement_remains_active(&mut placement, 59.0));
    assert!(!building_placement_remains_active(&mut placement, 1.0));
    placement.inactivity_seconds = 0.0;
    assert!(building_placement_remains_active(&mut placement, 1.0));
}

#[test]
fn building_command_names_and_numbered_bids_are_stable() {
    let content = embedded_content();
    assert_eq!(
        building_definition_id(&content, &StableId::new("orestorage").unwrap()).unwrap(),
        StableId::new("building:orestorage").unwrap()
    );
    assert_eq!(
        building_placement_request(&content, &StableId::new("thickpath").unwrap()).unwrap(),
        (StableId::new("building:path").unwrap(), true)
    );
    assert!(
        content
            .buildings
            .values()
            .all(|definition| !building_command_name(definition).contains(char::is_whitespace))
    );
    let building_id = StableId::new("building:tower").unwrap();
    let definition = &content.buildings[&building_id];
    let mut simulation = WorldSimulation::new(1);
    for serial in [3, 1, 2] {
        let id = StableId::new(format!("building:runtime_{serial:08}")).unwrap();
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id,
                archetype: definition.archetype.clone(),
                position: GridPos { x: serial, z: 1 },
                rotation_quarter_turns: 0,
                level: 1,
                health: 100,
                complete: true,
            },
        );
    }
    let (_, runtime) =
        numbered_building_instance_id(&content, &simulation, &StableId::new("tower").unwrap(), 2)
            .unwrap();
    assert_eq!(runtime.as_str(), "building:runtime_00000002");
}

#[test]
fn authored_level_curves_drive_effective_role_stats() {
    let content = embedded_content();
    let actor_id = StableId::new("npc:leveled_logger").unwrap();
    let role = StableId::new("role:logger").unwrap();
    let mut simulation = WorldSimulation::new(9);
    assert!(simulation.join_player(actor_id.clone(), GridPos { x: 1, z: 1 }));
    simulation.assign_role(&actor_id, role.clone()).unwrap();
    simulation
        .actors
        .get_mut(&actor_id)
        .unwrap()
        .role_progression
        .insert(
            role,
            stream_town_domain::RoleProgress {
                level: 5,
                experience: 7,
            },
        );

    let stats = effective_role_stats(&content, &simulation, &simulation.actors[&actor_id]).unwrap();
    assert_eq!(stats.level, 5);
    assert_eq!(stats.experience, 7);
    assert_eq!(stats.action_amount, 3);
    assert_eq!(stats.action_milliseconds, 980);
    assert_eq!(stats.movement_speed_milli_cells_per_second, 1_500);
    assert_eq!(stats.carry_capacity, 18);

    let necromancer = StableId::new("role:necromancer").unwrap();
    let necromancer_definition = &content.roles[&necromancer];
    assert!(necromancer_definition.action_range_milli_cells_per_level > 0);
    simulation
        .assign_role(&actor_id, necromancer.clone())
        .unwrap();
    simulation
        .actors
        .get_mut(&actor_id)
        .unwrap()
        .role_progression
        .insert(
            necromancer,
            stream_town_domain::RoleProgress {
                level: 25,
                experience: 0,
            },
        );
    let stats = effective_role_stats(&content, &simulation, &simulation.actors[&actor_id]).unwrap();
    assert_eq!(
        stats.action_range_milli_cells, necromancer_definition.base_action_range_milli_cells,
        "Unity never initializes PlayerRoleData._ranged, so its serialized range curve is dormant"
    );
}

#[test]
fn renderer_material_resolution_prefers_exact_slots_then_model_then_fallback() {
    let mut materials = Assets::<StandardMaterial>::default();
    let fallback = materials.add(StandardMaterial::default());
    let game = materials.add(StandardMaterial::default());
    let skin = materials.add(StandardMaterial::default());
    let wrong_same_name = materials.add(StandardMaterial::default());
    let override_material = materials.add(StandardMaterial::default());
    let mut building_materials = Assets::<BuildingMaterial>::default();
    let building = building_materials.add(building_material(&embedded_presentation(), None));
    let mut cloud_materials = Assets::<CloudMaterial>::default();
    let cloud = cloud_materials.add(cloud_material(&embedded_presentation(), None));
    let mut godray_materials = Assets::<GodrayMaterial>::default();
    let godray = godray_materials.add(godray_material(&embedded_presentation()));
    let mut giraffe_materials = Assets::<GiraffeMaterial>::default();
    let giraffe = giraffe_materials.add(giraffe_material(&embedded_presentation(), None));
    let mut bounds_materials = Assets::<BoundsMaterial>::default();
    let bounds = bounds_materials.add(bounds_material(&embedded_presentation(), None));
    let spec = MaterialOverrideSpec {
        fallback: Some(ResolvedMaterialHandle::Standard(fallback.clone())),
        model_materials: BTreeMap::from([
            (
                "GameMaterial".into(),
                ResolvedMaterialHandle::Standard(game.clone()),
            ),
            (
                "SkinMaterial".into(),
                ResolvedMaterialHandle::Standard(skin.clone()),
            ),
            (
                "BuildingMaterial".into(),
                ResolvedMaterialHandle::Building(building.clone()),
            ),
            (
                "CloudMaterial".into(),
                ResolvedMaterialHandle::Cloud(cloud.clone()),
            ),
            (
                "VFX_Godrays".into(),
                ResolvedMaterialHandle::Godray(godray.clone()),
            ),
            (
                "MainMaterial".into(),
                ResolvedMaterialHandle::Giraffe(giraffe.clone()),
            ),
            (
                "BoundsVisualizer".into(),
                ResolvedMaterialHandle::Bounds(bounds.clone()),
            ),
        ]),
        renderer_materials: vec![
            ResolvedRendererMaterialBinding {
                target_path: "Other/Body_Blacksmith_Bulk".into(),
                materials: BTreeMap::from([(
                    "GameMaterial".into(),
                    ResolvedMaterialHandle::Standard(wrong_same_name),
                )]),
            },
            ResolvedRendererMaterialBinding {
                target_path: "PlayerChar_TPose/Body_Mesh/Body_Blacksmith_Bulk".into(),
                materials: BTreeMap::from([(
                    "GameMaterial".into(),
                    ResolvedMaterialHandle::Standard(override_material.clone()),
                )]),
            },
        ],
        suppress_self_shadows: false,
    };

    let exact = resolved_renderer_material(
        &spec,
        "Scene/PlayerChar_TPose/Body_Mesh/Body_Blacksmith_Bulk/Body_Blacksmith_Bulk.GameMaterial",
        Some("Body_Blacksmith_Bulk"),
        Some("GameMaterial"),
    )
    .unwrap();
    assert!(matches!(
        exact,
        ResolvedMaterialHandle::Standard(material) if material.id() == override_material.id()
    ));

    let model = resolved_renderer_material(
        &spec,
        "Scene/PlayerChar_TPose/Body_Mesh/Body_Blacksmith_Bulk/Body_Blacksmith_Bulk.SkinMaterial",
        Some("Body_Blacksmith_Bulk"),
        Some("SkinMaterial"),
    )
    .unwrap();
    assert!(matches!(
        model,
        ResolvedMaterialHandle::Standard(material) if material.id() == skin.id()
    ));

    let inherited = resolved_renderer_material(
        &spec,
        "Scene/Unrelated/Unrelated.GameMaterial",
        Some("Unrelated"),
        Some("GameMaterial"),
    )
    .unwrap();
    assert!(matches!(
        inherited,
        ResolvedMaterialHandle::Standard(material) if material.id() == game.id()
    ));

    let typed_building = resolved_renderer_material(
        &spec,
        "Scene/TownHall/TownHall.BuildingMaterial",
        Some("TownHall"),
        Some("BuildingMaterial"),
    )
    .unwrap();
    assert!(matches!(
        typed_building,
        ResolvedMaterialHandle::Building(material) if material.id() == building.id()
    ));

    let typed_cloud = resolved_renderer_material(
        &spec,
        "Scene/Clouds/Clouds.CloudMaterial",
        Some("Clouds"),
        Some("CloudMaterial"),
    )
    .unwrap();
    assert!(matches!(
        typed_cloud,
        ResolvedMaterialHandle::Cloud(material) if material.id() == cloud.id()
    ));

    let typed_godray = resolved_renderer_material(
        &spec,
        "Scene/Age02_NecroTower/Env_Godrays_08/Env_Godrays_08.VFX_Godrays",
        Some("Env_Godrays_08"),
        Some("VFX_Godrays"),
    )
    .unwrap();
    assert!(matches!(
        typed_godray,
        ResolvedMaterialHandle::Godray(material) if material.id() == godray.id()
    ));

    let typed_giraffe = resolved_renderer_material(
        &spec,
        "Scene/Pet_TallBoi/Pet_TallBoi.MainMaterial",
        Some("Pet_TallBoi"),
        Some("MainMaterial"),
    )
    .unwrap();
    assert!(matches!(
        typed_giraffe,
        ResolvedMaterialHandle::Giraffe(material) if material.id() == giraffe.id()
    ));

    let typed_bounds = resolved_renderer_material(
        &spec,
        "Scene/VisualBounds/VisualBounds.BoundsVisualizer",
        Some("VisualBounds"),
        Some("BoundsVisualizer"),
    )
    .unwrap();
    assert!(matches!(
        typed_bounds,
        ResolvedMaterialHandle::Bounds(material) if material.id() == bounds.id()
    ));

    let final_fallback =
        resolved_renderer_material(&spec, "Scene/Unrelated", None, Some("Unmapped")).unwrap();
    assert!(matches!(
        final_fallback,
        ResolvedMaterialHandle::Standard(material) if material.id() == fallback.id()
    ));
}

#[test]
fn embedded_presentation_binds_native_and_converted_animation_paths() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    assert_eq!(presentation.schema_version, 21);
    assert_eq!(presentation.textures.len(), 133);
    assert_eq!(presentation.materials.len(), 33);
    assert_eq!(presentation.post_process_profiles.len(), 2);
    assert_eq!(presentation.fireworks_effects.len(), 1);
    assert_eq!(
        presentation
            .clips
            .values()
            .flat_map(|clip| &clip.events)
            .count(),
        12
    );
    let fishing = presentation
        .clips
        .values()
        .find(|clip| {
            clip.source_path == PLAYER_ANIMATED_SOURCE_MODEL
                && clip.display_name == "CharacterFishing"
        })
        .expect("shipping embedded fishing take");
    assert_eq!(
        fishing
            .events
            .iter()
            .map(|event| event.function_name.as_str())
            .collect::<Vec<_>>(),
        ["ToggleOn", "ToggleOff"]
    );
    assert!(fishing.events[0].time > 7.5 && fishing.events[0].time < 7.6);
    assert!(fishing.events[1].time > 13.2 && fishing.events[1].time < 13.3);
    assert_eq!(
        presentation
            .scene_fireworks
            .get(CREDITS_SCENE_PATH)
            .map(Vec::len),
        Some(2)
    );
    assert_eq!(
        presentation
            .scene_post_process
            .values()
            .map(Vec::len)
            .sum::<usize>(),
        3
    );
    let day = authored_post_process_stack(&presentation, WORLD_SCENE_PATH, 1.0);
    let night = authored_post_process_stack(&presentation, WORLD_SCENE_PATH, 0.0);
    assert_eq!(day.len(), 2);
    assert!((day[0].1 - 1.0).abs() < f32::EPSILON);
    assert!(day[1].1.abs() < f32::EPSILON);
    assert!((night[0].1 - 1.0).abs() < f32::EPSILON);
    assert!((night[1].1 - 1.0).abs() < f32::EPSILON);
    let day_grading = authored_color_grading(&PlayerSettings::default(), &day);
    let night_grading = authored_color_grading(&PlayerSettings::default(), &night);
    assert!((day_grading.global.exposure - 1.1).abs() < f32::EPSILON);
    assert!((night_grading.global.exposure - 0.75).abs() < f32::EPSILON);
    assert!(day_grading.global.temperature.abs() < f32::EPSILON);
    assert!(day_grading.global.tint.abs() < f32::EPSILON);
    assert!(night_grading.global.temperature.abs() < f32::EPSILON);
    assert!(night_grading.global.tint.abs() < f32::EPSILON);
    assert!(
        authored_rgb_filter(&day)
            .into_iter()
            .all(|component| (component - 1.0).abs() < f32::EPSILON)
    );
    assert!(
        authored_rgb_filter(&night)
            .into_iter()
            .zip([0.674_528_3, 0.728_030_44, 1.0, 1.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert_eq!(
        presentation
            .materials
            .values()
            .map(|material| material.custom_vectors.len())
            .sum::<usize>(),
        141
    );
    assert_eq!(
        presentation
            .materials
            .values()
            .map(|material| material.texture_transforms.len())
            .sum::<usize>(),
        32
    );
    let terrain = terrain_material(&presentation, &GameConfig::default(), None, None, None);
    assert!(terrain.extension.grid_texture.is_none());
    assert!((terrain.base.perceptual_roughness - 1.0).abs() < f32::EPSILON);
    assert!(terrain.base.metallic.abs() < f32::EPSILON);
    assert!(terrain.base.reflectance.abs() < f32::EPSILON);
    assert!((terrain.extension.parameters.traversal_dirt_color.w - 0.5).abs() < f32::EPSILON);
    assert_eq!(
        terrain.extension.parameters.texture_uv_blend_tint.xy(),
        Vec2::splat(0.25)
    );
    assert!((terrain.extension.parameters.texture_uv_blend_tint.z - 1.0).abs() < f32::EPSILON);
    assert!(terrain.extension.parameters.texture_uv_blend_tint.w.abs() < f32::EPSILON);
    assert_eq!(
        terrain.extension.parameters.grid_scale_offset,
        Vec4::new(1.0, 1.0, 0.0, 0.0)
    );
    assert!(
        terrain
            .extension
            .parameters
            .sand_color_a
            .to_array()
            .into_iter()
            .zip(unity_shader_color([1.0, 0.827_731, 0.088_235_21, 0.0]).to_array())
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    let water = water_material(&presentation, None);
    assert_eq!(water.base.alpha_mode, AlphaMode::Opaque);
    assert!(water.extension.main_texture.is_none());
    assert!(water.extension.noise_texture.is_none());
    assert!(
        water
            .extension
            .parameters
            .surface_color
            .to_array()
            .into_iter()
            .zip(unity_shader_color([0.0, 0.764_705_9, 1.0, 1.0]).to_array())
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert!((water.extension.parameters.wind_speed_noise_alpha.z - 0.02).abs() < f32::EPSILON);
    assert!((water.extension.parameters.scale_foam_ice.y - 3.71).abs() < f32::EPSILON);
    assert_eq!(
        water.extension.parameters.depth_foam_controls,
        Vec4::new(10.0, 0.8, 7.81, 0.94)
    );
    assert_eq!(
        water.extension.parameters.opacity_controls,
        Vec4::new(0.86, 0.94, 0.0, 0.0)
    );
    let menu_water = main_menu_water_material(water.clone());
    assert_eq!(
        menu_water.extension.parameters.opacity_controls,
        Vec4::new(0.68, 0.78, 1.0, 0.46)
    );
    assert_eq!(menu_water.base.alpha_mode, AlphaMode::Blend);
    let surface = unity_shader_color([0.0, 0.764_705_9, 1.0, 1.0]);
    let target = unity_shader_color([0.05, 0.29, 0.47, 0.62]);
    assert_eq!(
        water_color_tint(surface, [0.05, 0.29, 0.47, 0.62]),
        Vec4::new(
            target.x / 0.1,
            target.y / surface.y,
            target.z / surface.z,
            target.w
        )
    );
    assert_eq!(
        water.extension.parameters.main_scale_offset,
        Vec4::new(1.0, 1.0, 0.0, 0.0)
    );
    let building = building_material(&presentation, None);
    assert!(building.extension.main_texture.is_none());
    assert_eq!(
        building.extension.parameters.detail_color,
        unity_shader_color([0.521_568_5, 0.521_568_5, 0.521_568_5, 1.0])
    );
    assert_eq!(
        building.extension.parameters.ambient_occlusion,
        Vec4::new(0.4, 1.74, 0.0, 0.0)
    );
    assert_eq!(
        building.extension.parameters.main_scale_offset,
        Vec4::new(1.0, 1.0, 0.0, 0.0)
    );
    assert!((building.extension.parameters.snow_damage.z - 1.787).abs() < f32::EPSILON);
    let clouds = cloud_material(&presentation, None);
    assert!(clouds.extension.noise_texture.is_none());
    assert_eq!(
        clouds.extension.parameters.noise_controls,
        Vec4::new(0.005, 20.0, 0.0, 0.09)
    );
    assert_eq!(
        clouds.extension.parameters.surface_transform,
        Vec4::new(200.0, 1.4, 1.0, 1.0)
    );
    assert_eq!(
        clouds.extension.parameters.filter_controls,
        Vec4::new(CLOUD_NOISE_ALPHA_MEAN, 0.75, 2.0, 0.0)
    );
    let water_definition = presentation
        .materials
        .values()
        .find(|material| material.source_path == WATER_MATERIAL_PATH)
        .unwrap();
    assert!(
        water_definition.texture_transforms["_WaterNormal"]
            .scale
            .into_iter()
            .zip([81.2, 200.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert!((water_ice_strength(Season::Spring) - 0.0).abs() < f32::EPSILON);
    assert!((water_ice_strength(Season::Winter) - 1.0).abs() < f32::EPSILON);
    assert_eq!(presentation.controllers.len(), 31);
    assert_eq!(
        presentation
            .model_materials
            .values()
            .map(BTreeMap::len)
            .sum::<usize>(),
        241
    );
    assert_eq!(
        presentation
            .prefab_renderer_materials
            .values()
            .map(Vec::len)
            .sum::<usize>(),
        903
    );
    assert_eq!(
        presentation
            .prefab_renderer_materials
            .values()
            .flatten()
            .map(|renderer| renderer.materials.len())
            .sum::<usize>(),
        912
    );
    assert_eq!(
        presentation
            .clips
            .values()
            .map(|clip| clip.property_curves.len())
            .sum::<usize>(),
        110
    );
    assert_eq!(
        presentation
            .clips
            .values()
            .map(|clip| clip.events.len())
            .sum::<usize>(),
        12
    );
    assert_eq!(
        animation_property_value(
            &presentation,
            "CreditsPopups",
            "CreatedBy_Canvas",
            "m_IsActive",
            8.0,
        ),
        Some(1.0)
    );
    assert_eq!(
        animation_property_value(
            &presentation,
            "CreditsPopups",
            "CreatedBy_Canvas",
            "m_IsActive",
            8.5,
        ),
        Some(0.0)
    );
    let character_materials = presentation
        .model_materials
        .get("Assets/Models/Characters/Characters.fbx")
        .unwrap();
    assert_eq!(
        character_materials["GameMaterial"].as_str(),
        "material:fa4085ce3ea4d394bb4e587376c58cbd"
    );
    assert_eq!(
        character_materials["SkinMaterial"].as_str(),
        "material:304fcfe47809be14ab680e64084f8494"
    );
    let archetype =
        archetype_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();
    let scene = default_archetype_scene(archetype).unwrap();
    let binding = presentation
        .prefab_bindings
        .get(&archetype.source_guid)
        .unwrap();
    assert_eq!(
        binding.animated_scene.as_deref(),
        Some(scene.asset_path.as_str())
    );
    assert_eq!(binding.gltf_animation_index, Some(0));
    let materials = presentation
        .prefab_materials
        .get(&archetype.source_guid)
        .unwrap();
    assert!(!materials.is_empty());
    assert!(
        materials
            .iter()
            .all(|material| presentation.materials.contains_key(material))
    );

    let player =
        archetype_by_source(&content, ArchetypeKind::Player, "Player_Character.prefab").unwrap();
    let player_scene = default_archetype_scene(player).unwrap();
    let player_binding = presentation
        .prefab_bindings
        .get(&player.source_guid)
        .unwrap();
    assert_eq!(
        player_binding.rig_scene.as_deref(),
        Some(player_scene.asset_path.as_str())
    );
    let spec = converted_animation_spec(player, &presentation).unwrap();
    let controller = presentation.controllers.get(&spec.controller).unwrap();
    assert_eq!(
        rig_animation_root_name(PLAYER_ANIMATED_MODEL_PATH),
        Some("CharacterArmature")
    );
    assert_eq!(controller.state_machines.len(), 4);
    assert_eq!(controller.layers.len(), 2);
    assert_eq!(controller.layers[0].display_name, "Base Layer");
    assert_eq!(controller.layers[1].display_name, "Top");
    assert!((controller.layers[0].effective_weight(0) - 1.0).abs() < f32::EPSILON);
    assert!(controller.layers[1].effective_weight(1).abs() < f32::EPSILON);
    let base_mask = controller.layers[0]
        .avatar_mask
        .as_ref()
        .and_then(|mask| presentation.avatar_masks.get(mask))
        .unwrap();
    assert_eq!(base_mask.display_name, "Player_All");
    assert!(
        base_mask
            .transform_weights
            .values()
            .all(|weight| (*weight - 1.0).abs() < f32::EPSILON)
    );
    for (role_id, role) in &content.roles {
        let parameter = controller
            .parameters
            .iter()
            .find(|parameter| parameter.name == role.action_animation)
            .unwrap_or_else(|| {
                panic!(
                    "{role_id} action {} is missing from Character.controller",
                    role.action_animation
                )
            });
        let expected_kind = if role.action_animation == "Action" {
            stream_town_domain::AnimationParameterKind::Boolean
        } else {
            stream_town_domain::AnimationParameterKind::Trigger
        };
        assert_eq!(parameter.kind, expected_kind, "{role_id}");
    }
    let state = controller.states.get(&spec.state).unwrap();
    assert_eq!(state.blend_parameter.as_deref(), Some("Move Speed"));
    assert_eq!(state.motions.len(), 3);
    let idle = presentation.clips.get(&state.motions[0].clip).unwrap();
    assert!(!idle.transform_tracks.is_empty());
    let mut runtime = AnimationControllerRuntime::in_state(controller, spec.state).unwrap();
    runtime.set_float("Move Speed", 0.25).unwrap();
    let selection = runtime.motion_selection(controller).unwrap().unwrap();
    assert!(selection.second.is_some());
    runtime.set_boolean("Action", true).unwrap();
    runtime.set_trigger("SpearAttack").unwrap();
    let transition = runtime.evaluate_transitions(controller, 0.0).unwrap();
    let stream_town_domain::AnimationTransitionOutcome::Entered(action_state) = transition else {
        panic!("authored SpearAttack trigger did not enter an action state");
    };
    assert_eq!(controller.states[&action_state].display_name, "SpearAttack");
    assert_eq!(
        controller.states[&action_state].speed_parameter.as_deref(),
        Some("ActionSpeed")
    );
    runtime.set_float("ActionSpeed", 2.0).unwrap();
    assert!((runtime.state_speed(controller).unwrap() - 2.0).abs() < f32::EPSILON);
    runtime.set_boolean("Action", false).unwrap();
    let exit = runtime.evaluate_transitions(controller, 0.0).unwrap();
    let stream_town_domain::AnimationTransitionOutcome::Entered(locomotion_state) = exit else {
        panic!("authored action exit did not return through the parent state machine");
    };
    assert_eq!(
        controller.states[&locomotion_state].display_name,
        "Locomotion"
    );
    let top = controller
        .layers
        .iter()
        .find(|layer| layer.display_name == "Top")
        .unwrap();
    let top_state = controller.state_machines[&top.state_machine]
        .default_state
        .clone()
        .unwrap();
    let mut top_runtime = AnimationControllerRuntime::in_state(controller, top_state).unwrap();
    top_runtime.set_boolean("CarryWood", true).unwrap();
    let carry_transition = top_runtime.evaluate_transitions(controller, 1.0).unwrap();
    let stream_town_domain::AnimationTransitionOutcome::Entered(carry_state) = carry_transition
    else {
        panic!("Top layer did not enter Carry from authored CarryWood parameter");
    };
    assert_eq!(controller.states[&carry_state].display_name, "Carry");
    assert!(state_layer_owns_clip(
        controller,
        top_runtime.current_state(),
        &controller.states[&carry_state].motions[0].clip,
    ));
    assert_eq!(
        presentation
            .clips
            .values()
            .filter(|clip| !clip.transform_tracks.is_empty())
            .count(),
        57
    );
    let targets: BTreeMap<_, _> = idle
        .transform_tracks
        .iter()
        .map(|track| {
            (
                track.target_path.clone(),
                (
                    Entity::PLACEHOLDER,
                    Transform {
                        translation: track
                            .reference_translation
                            .map_or(Vec3::ZERO, Vec3::from_array),
                        rotation: track
                            .reference_rotation
                            .map_or(Quat::IDENTITY, normalized_quat),
                        scale: track.reference_scale.map_or(Vec3::ONE, Vec3::from_array),
                    },
                ),
            )
        })
        .collect();
    let retargeted = retargeted_animation_clip(idle, &targets).unwrap();
    assert!(!retargeted.curves().is_empty());
    assert!(retargeted.duration() >= idle.duration_seconds);
    let prefixed_targets: BTreeMap<_, _> = targets
        .iter()
        .map(|(path, target)| (format!("CharacterArmature/Body/{path}"), *target))
        .collect();
    let prefixed = retargeted_animation_clip(idle, &prefixed_targets).unwrap();
    assert_eq!(prefixed.curves().len(), retargeted.curves().len());
    let consolidated_targets: BTreeMap<_, _> = targets
        .iter()
        .map(|(path, target)| {
            let path = path.strip_prefix("pelvis/").map_or_else(
                || "CharacterArmature".to_owned(),
                |bone| format!("CharacterArmature/{bone}"),
            );
            (path, *target)
        })
        .collect();
    let consolidated = retargeted_animation_clip(idle, &consolidated_targets).unwrap();
    assert_eq!(consolidated.curves().len(), retargeted.curves().len());
    assert!(
        idle.rig_asset_path
            .as_deref()
            .is_some_and(|path| path.contains("/Characters/"))
    );
    assert!(
        idle.transform_tracks
            .iter()
            .any(|track| track.target_path == "pelvis" && !track.translation.is_empty())
    );
    assert!(idle.transform_tracks.iter().any(|track| {
        track.target_path.starts_with("pelvis/") && !track.translation.is_empty()
    }));
}
