#[test]
fn traversal_wear_records_only_completed_citizen_cell_entries() {
    let mut wear = TraversalWearRuntime::default();
    let settings = stream_town_domain::TerrainAppearanceConfig::default();
    let start = GridPos { x: 1, z: 2 };
    let next = GridPos { x: 2, z: 2 };
    record_completed_cell_traversal(&mut wear, &settings, &ActorKind::Enemy, start, next);
    record_completed_cell_traversal(&mut wear, &settings, &ActorKind::Player, start, start);
    assert!(wear.cells.is_empty());

    let full_score = traversal_score_for_rate(
        settings.traversal_full_tint_per_minute,
        settings.traversal_half_life_seconds,
    );
    while wear.cells.get(&next).map_or(0.0, |cell| cell.score) < full_score {
        record_completed_cell_traversal(&mut wear, &settings, &ActorKind::Player, start, next);
    }
    assert!((wear.cells[&next].score - full_score).abs() < f32::EPSILON);
    assert!(
        (wear.cells[&next].decay_pause_seconds - settings.traversal_decay_pause_seconds).abs()
            < f32::EPSILON
    );
}

#[test]
fn traversal_wear_fades_from_five_to_fifty_and_decays_by_half_life() {
    let settings = stream_town_domain::TerrainAppearanceConfig::default();
    let fade_start = traversal_score_for_rate(
        settings.traversal_fade_start_per_minute,
        settings.traversal_half_life_seconds,
    );
    let full_tint = traversal_score_for_rate(
        settings.traversal_full_tint_per_minute,
        settings.traversal_half_life_seconds,
    );
    assert!(traversal_wear_fraction(fade_start - 0.01, &settings).abs() < f32::EPSILON);
    assert!(
        (traversal_wear_fraction((fade_start + full_tint) * 0.5, &settings) - 0.5).abs()
            < f32::EPSILON
    );
    assert!((traversal_wear_fraction(full_tint, &settings) - 1.0).abs() < f32::EPSILON);
    assert!((traversal_wear_fraction(full_tint * 2.0, &settings) - 1.0).abs() < f32::EPSILON);

    let position = GridPos { x: 4, z: 5 };
    let pruned = GridPos { x: 5, z: 5 };
    let mut wear = TraversalWearRuntime::default();
    wear.cells.insert(
        position,
        TraversalWearCell {
            score: full_tint,
            decay_pause_seconds: 0.0,
        },
    );
    wear.cells.insert(
        pruned,
        TraversalWearCell {
            score: 0.015,
            decay_pause_seconds: 0.0,
        },
    );
    wear.decay(settings.traversal_half_life_seconds, &settings);
    assert!((wear.cells[&position].score - full_tint * 0.5).abs() < 0.000_1);
    assert!(!wear.cells.contains_key(&pruned));

    let paused = GridPos { x: 6, z: 5 };
    wear.record(paused, &settings);
    wear.decay(settings.traversal_decay_pause_seconds - 1.0, &settings);
    assert!((wear.cells[&paused].score - 1.0).abs() < f32::EPSILON);
    wear.decay(1.0, &settings);
    assert!((wear.cells[&paused].score - 1.0).abs() < f32::EPSILON);
    wear.decay(settings.traversal_half_life_seconds, &settings);
    assert!((wear.cells[&paused].score - 0.5).abs() < 0.000_1);
}

#[test]
fn traversal_wear_texture_maps_scores_to_their_grid_cells() {
    let settings = stream_town_domain::TerrainAppearanceConfig::default();
    let mut image = traversal_wear_image(4, 3);
    let fade_start = traversal_score_for_rate(
        settings.traversal_fade_start_per_minute,
        settings.traversal_half_life_seconds,
    );
    let full_tint = traversal_score_for_rate(
        settings.traversal_full_tint_per_minute,
        settings.traversal_half_life_seconds,
    );
    let half = GridPos { x: 1, z: 2 };
    let full = GridPos { x: 3, z: 0 };
    let cells = HashMap::from([
        (
            half,
            TraversalWearCell {
                score: (fade_start + full_tint) * 0.5,
                decay_pause_seconds: 0.0,
            },
        ),
        (
            full,
            TraversalWearCell {
                score: full_tint,
                decay_pause_seconds: 0.0,
            },
        ),
    ]);
    write_traversal_wear_pixels(&mut image, 4, 3, &cells, &settings);
    let data = image.data.as_ref().expect("wear texture is CPU-backed");
    let half_index = (usize::from(half.z) * 4 + usize::from(half.x)) * 4;
    let full_index = (usize::from(full.z) * 4 + usize::from(full.x)) * 4;
    assert_eq!(data[half_index], 128);
    assert_eq!(data[full_index], u8::MAX);
    assert_eq!(data[0], 0);
}

#[test]
fn traversal_wear_hides_foliage_without_removing_it() {
    let settings = stream_town_domain::TerrainAppearanceConfig::default();
    let fade_start = traversal_score_for_rate(
        settings.traversal_fade_start_per_minute,
        settings.traversal_half_life_seconds,
    );
    assert!(!foliage_should_be_hidden(
        false,
        false,
        false,
        fade_start - 0.01,
        &settings
    ));
    assert!(foliage_should_be_hidden(
        false, false, false, fade_start, &settings
    ));
    assert!(foliage_should_be_hidden(true, false, false, 0.0, &settings));
    assert!(foliage_should_be_hidden(false, true, false, 0.0, &settings));
    assert!(foliage_should_be_hidden(false, false, true, 0.0, &settings));
}

#[test]
fn foliage_clearance_cell_set_matches_structural_regions() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let mut simulation = WorldSimulation::new(world.seed);
    ensure_town_hall_state(&content, &config, &mut simulation);

    let regions = foliage_clearance_regions(&content, &simulation, &world);
    let cells = foliage_clearance_cells(&content, &simulation, &world);
    for z in 0..world.navigation.height() {
        for x in 0..world.navigation.width() {
            let position = GridPos { x, z };
            assert_eq!(
                cells.contains(&position),
                regions
                    .iter()
                    .any(|region| region_contains_grid_position(*region, position)),
                "clearance mismatch at {position:?}"
            );
        }
    }
}

#[test]
fn initial_town_hall_clears_foliage_across_its_exact_placement_footprint() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let mut simulation = WorldSimulation::new(world.seed);
    ensure_town_hall_state(&content, &config, &mut simulation);
    let town_hall_id = StableId::new("building:townhall").unwrap();
    let town_hall = &simulation.buildings[&town_hall_id];
    let definition = &content.buildings[&town_hall_id];
    let placement_cells =
        building_fine_placement_cells(&content, &simulation, town_hall, &town_hall_id, definition);
    let clearance_cells = foliage_clearance_navigation_cells(&content, &simulation, &world);
    let legacy_cells = foliage_clearance_regions(&content, &simulation, &world)
        .into_iter()
        .flat_map(fine_cells_for_coarse_region)
        .collect::<HashSet<_>>();

    assert_eq!(placement_cells.len(), 15 * 9);
    assert!(
        placement_cells
            .iter()
            .all(|position| clearance_cells.contains(position))
    );
    assert!(
        placement_cells
            .iter()
            .any(|position| !legacy_cells.contains(position)),
        "the regression must cover the Town Hall area missed by legacy coarse clearing"
    );
}

#[test]
fn initial_town_hall_removes_seeded_tree_resources_beneath_its_model() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let mut simulation = WorldSimulation::new(world.seed);
    ensure_town_hall_state(&content, &config, &mut simulation);
    let town_hall_id = StableId::new("building:townhall").unwrap();
    let town_hall = &simulation.buildings[&town_hall_id];
    let definition = &content.buildings[&town_hall_id];
    let occupied =
        building_fine_navigation_cells(&content, &simulation, town_hall, &town_hall_id, definition)
            .into_iter()
            .collect::<HashSet<_>>();
    let under_town_hall = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .find(|position| occupied.contains(&placement_to_navigation_centre(*position)))
        .expect("Town Hall contains at least one coarse resource centre");
    world.resources.push(stream_town_domain::GeneratedResource {
        id: StableId::new("resource:tree_under_town_hall").unwrap(),
        kind: StableId::new("resource:wood").unwrap(),
        target_kind: StableId::new("target:tree").unwrap(),
        position: under_town_hall,
        offset_milli_cells: [0, 0],
        generation_occupancy: [0, 0],
        amount: 100,
    });
    let expected_removed = world
        .resources
        .iter()
        .filter(|resource| {
            resource.target_kind.as_str() == "target:tree"
                && occupied.contains(&placement_to_navigation_centre(resource.position))
        })
        .count();

    let removed =
        clear_seeded_trees_under_building(&content, &simulation, &mut world, &town_hall_id);

    assert!(expected_removed > 0);
    assert_eq!(removed, expected_removed);
    assert!(world.resources.iter().all(|resource| {
        resource.target_kind.as_str() != "target:tree"
            || !occupied.contains(&placement_to_navigation_centre(resource.position))
    }));
}

#[test]
fn completed_path_levels_accelerate_citizens_by_five_percent_each() {
    let config = GameConfig::default();
    let content = embedded_content();
    let position = GridPos { x: 8, z: 9 };
    let citizen = StableId::new("twitch:path_speed_test").unwrap();
    let mut simulation = WorldSimulation::new(7);
    assert!(simulation.join_player(citizen.clone(), position));
    let base = actor_movement_speed(&config, &content, &simulation, &citizen);
    let navigation_position = placement_to_navigation_centre(position);
    let paths = PathSurfaceRuntime {
        levels: HashMap::from([(navigation_position, 3)]),
        ..default()
    };
    let accelerated = actor_movement_speed_on_path(
        &config,
        &content,
        &simulation,
        &paths,
        &citizen,
        navigation_position,
    );
    assert!((accelerated - base * 1.15).abs() < 0.000_1);
}

fn audio_acceptance_record(wav: &[u8]) -> serde_json::Value {
    assert_eq!(&wav[0..4], b"RIFF");
    assert_eq!(&wav[8..12], b"WAVE");
    let samples = wav[44..]
        .chunks_exact(2)
        .map(|bytes| i16::from_le_bytes([bytes[0], bytes[1]]))
        .collect::<Vec<_>>();
    let peak = samples
        .iter()
        .map(|sample| i32::from(*sample).abs())
        .max()
        .unwrap_or_default();
    let maximum_step = samples
        .windows(2)
        .map(|pair| (i32::from(pair[1]) - i32::from(pair[0])).abs())
        .max()
        .unwrap_or_default();
    let sum_squares = samples.iter().fold(0_u128, |sum, sample| {
        let sample = i128::from(*sample);
        sum.saturating_add(u128::try_from(sample * sample).expect("sample square is positive"))
    });
    serde_json::json!({
        "sha256": format!("{:x}", Sha256::digest(wav)),
        "bytes": wav.len(),
        "samples": samples.len(),
        "peak": peak,
        "maximum_step": maximum_step,
        "sum_squares": sum_squares.to_string(),
    })
}

fn audio_acceptance_wavs() -> BTreeMap<String, Vec<u8>> {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let mut clips = BTreeMap::new();
    clips.insert(
        "ambience".to_owned(),
        procedural_ambience_wav(PROCEDURAL_AUDIO_SAMPLE_RATE, 24.0),
    );
    for variant in 0..3 {
        clips.insert(
            format!("seagull-{variant}"),
            procedural_seagull_call_wav(variant, PROCEDURAL_AUDIO_SAMPLE_RATE),
        );
    }
    for (role, definition) in &presentation.role_action_audio {
        let display_name = &content.roles[role].action_animation;
        for guid in &definition.clip_guids {
            clips.insert(
                format!("{role}:{guid}"),
                procedural_role_action_wav(role, display_name, guid, PROCEDURAL_AUDIO_SAMPLE_RATE),
            );
        }
    }
    clips
}

fn audio_acceptance_manifest() -> serde_json::Value {
    let clips = audio_acceptance_wavs()
        .into_iter()
        .map(|(name, wav)| (name, format!("{:x}", Sha256::digest(wav))))
        .collect::<BTreeMap<_, _>>();
    serde_json::json!({
        "schema_version": 1,
        "sample_rate": PROCEDURAL_AUDIO_SAMPLE_RATE,
        "clips": clips,
    })
}

#[test]
fn twitch_outbound_replies_preserve_unity_attribution_and_silence() {
    assert_eq!(
        unity_outbound_reply(&ChatCommand::Join, true, "welcome to Stream Town", "Viewer"),
        Some("Viewer Welcome to the game, your character was successfully created!".to_owned())
    );
    assert_eq!(
        unity_outbound_reply(
            &ChatCommand::Station(Some(2)),
            true,
            "station changed to station:mill",
            "Viewer"
        ),
        Some("Viewer: Station Switched!".to_owned())
    );
    assert_eq!(
        unity_outbound_reply(
            &ChatCommand::Health,
            false,
            "command rejected: join before checking health",
            "Viewer"
        ),
        Some("Viewer: You need to create a character first with !join".to_owned())
    );
    assert_eq!(
        unity_outbound_reply(
            &ChatCommand::MoveBuilding(Vec::new()),
            true,
            "building moved",
            "Viewer"
        ),
        None
    );
    assert_eq!(
        unity_outbound_reply(
            &ChatCommand::TownStats,
            true,
            "town: 2 players, 4 buildings",
            "Viewer"
        ),
        Some("town: 2 players, 4 buildings".to_owned())
    );
}

#[test]
fn building_info_accepts_the_source_authored_optional_instance_id() {
    let content = embedded_content();
    let house_id = StableId::new("building:house").unwrap();
    let house = &content.buildings[&house_id];
    let runtime_id = StableId::new("building:runtime_000004").unwrap();
    let mut simulation = WorldSimulation::new(42);
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id.clone(),
            archetype: house.archetype.clone(),
            position: GridPos { x: 21, z: 34 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 80,
            complete: true,
        },
    );

    let details = item_info(&content, &simulation, &house_id, Some(1)).unwrap();

    assert!(details.contains("House BID 1"), "{details}");
    assert!(details.contains("level 1/1"), "{details}");
    assert!(details.contains("health 80/"), "{details}");
    assert!(details.contains("at 21,34"), "{details}");
    assert!(
        item_info(
            &content,
            &simulation,
            &StableId::new("prospectorhut").unwrap(),
            None,
        )
        .unwrap()
        .starts_with("Prospector Hut:"),
        "PascalCase command names must resolve building IDs containing underscores"
    );
}

#[test]
fn player_info_reports_every_authored_profession_level() {
    let content = embedded_content();
    let player = StableId::new("twitch:forestfriend").unwrap();
    let mut simulation = WorldSimulation::new(42);
    assert!(simulation.join_player(player.clone(), GridPos { x: 2, z: 3 }));
    simulation.actors.get_mut(&player).unwrap().display_name = Some("ForestFriend".to_owned());
    simulation
        .actors
        .get_mut(&player)
        .unwrap()
        .role_progression
        .insert(
            StableId::new("role:forester").unwrap(),
            stream_town_domain::RoleProgress {
                level: 7,
                experience: 12,
            },
        );

    let details = item_info(
        &content,
        &simulation,
        &StableId::new("forestfriend").unwrap(),
        None,
    )
    .unwrap();

    assert!(details.starts_with("ForestFriend:"), "{details}");
    assert!(details.contains("Forester 7"), "{details}");
    for role in content
        .roles
        .values()
        .filter(|role| !matches!(role.display_name.as_str(), "Ruler" | "Enemy"))
    {
        assert!(
            details.contains(&role.display_name),
            "missing {} in {details}",
            role.display_name
        );
    }
}

#[test]
fn twitch_pascal_case_matches_unity_enum_names() {
    assert_eq!(twitch_pascal_case("red_panda"), "RedPanda");
    assert_eq!(twitch_pascal_case("fish_god"), "FishGod");
}

#[test]
fn fullscreen_uses_startup_safe_borderless_compatibility() {
    assert_eq!(
        player_window_mode(DisplayMode::Fullscreen, None),
        WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
    );
    assert_eq!(
        player_window_mode(DisplayMode::Borderless, None),
        WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
    );

    let mut world = World::new();
    let monitor = world.spawn_empty().id();
    assert_eq!(
        player_window_mode(DisplayMode::Fullscreen, Some(monitor)),
        WindowMode::BorderlessFullscreen(MonitorSelection::Entity(monitor))
    );
    assert_eq!(
        player_window_mode(DisplayMode::Borderless, Some(monitor)),
        WindowMode::BorderlessFullscreen(MonitorSelection::Entity(monitor))
    );
    assert_eq!(
        startup_window_mode(DisplayMode::Fullscreen, false),
        WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
    );
    assert_eq!(
        startup_window_mode(DisplayMode::Windowed, false),
        WindowMode::Windowed
    );
    assert_eq!(
        startup_window_mode(DisplayMode::Fullscreen, true),
        WindowMode::Windowed
    );
}

#[test]
fn camera_screen_directions_match_the_shipping_unity_mapping() {
    assert_eq!(
        unity_camera_world_direction(Vec2::new(-1.0, 0.0)),
        Vec2::new(0.0, -1.0),
        "left command moves toward -Z"
    );
    assert_eq!(
        unity_camera_world_direction(Vec2::new(1.0, 0.0)),
        Vec2::new(0.0, 1.0),
        "right command moves toward +Z"
    );
    assert_eq!(
        unity_camera_world_direction(Vec2::new(0.0, 1.0)),
        Vec2::new(1.0, 0.0),
        "top edge moves toward +X"
    );
    assert_eq!(
        unity_camera_world_direction(Vec2::new(0.0, -1.0)),
        Vec2::new(-1.0, 0.0),
        "bottom edge moves toward -X"
    );
}

#[test]
fn twitch_camera_horizontal_steps_are_more_granular_than_vertical_steps() {
    let approximately = |left: f32, right: f32| (left - right).abs() <= f32::EPSILON;
    assert!(approximately(
        twitch_camera_pan_distance(CameraDirection::Left),
        TWITCH_CAMERA_HORIZONTAL_PAN_DISTANCE
    ));
    assert!(approximately(
        twitch_camera_pan_distance(CameraDirection::Right),
        TWITCH_CAMERA_HORIZONTAL_PAN_DISTANCE
    ));
    assert!(approximately(
        twitch_camera_pan_distance(CameraDirection::Up),
        TWITCH_CAMERA_VERTICAL_PAN_DISTANCE
    ));
    assert!(approximately(
        TWITCH_CAMERA_HORIZONTAL_PAN_DISTANCE * 2.0,
        TWITCH_CAMERA_VERTICAL_PAN_DISTANCE
    ));
}

#[test]
fn centered_build_cursor_retains_fine_path_position() {
    let config = GameConfig::default();
    let content = embedded_content();
    let simulation = WorldSimulation::new(7);
    let coarse = GridPos { x: 10, z: 12 };
    let coarse_world = Vec3::new(
        (f32::from(coarse.x) - f32::from(config.world.width.saturating_sub(1)) * 0.5)
            * config.world.cell_size,
        0.0,
        (f32::from(coarse.z) - f32::from(config.world.height.saturating_sub(1)) * 0.5)
            * config.world.cell_size,
    );
    let hit = coarse_world + Vec3::X * (config.world.cell_size / 3.0);
    let mut building = BuildingPlacement {
        building: StableId::new("building:house").unwrap(),
        thick_path: false,
        position: GridPos { x: 1, z: 1 },
        navigation_position: None,
        rotation_quarter_turns: 0,
        line_start: None,
        line_end: None,
        path_cells: Vec::new(),
        inactivity_seconds: 9.0,
    };
    let mut path = BuildingPlacement {
        building: StableId::new("building:path").unwrap(),
        navigation_position: Some(GridPos { x: 1, z: 1 }),
        ..building.clone()
    };

    let building_cursor = centre_building_placement_at_world_position(
        &mut building,
        hit,
        &config,
        None,
        &content,
        &simulation,
    )
    .unwrap();
    let path_cursor = centre_building_placement_at_world_position(
        &mut path,
        hit,
        &config,
        None,
        &content,
        &simulation,
    )
    .unwrap();

    assert_eq!(building_cursor, coarse);
    assert_eq!(building.position, coarse);
    assert_eq!(
        path_cursor,
        GridPos {
            x: coarse.x * NAVIGATION_SUBDIVISIONS + 2,
            z: coarse.z * NAVIGATION_SUBDIVISIONS + 1,
        }
    );
    assert_eq!(path.navigation_position, Some(path_cursor));
    assert_eq!(path.position, coarse);
}

#[test]
fn queued_camera_command_executes_without_legacy_idle_gate() {
    let home = Transform::from_xyz(0.0, 120.0, 0.0);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_millis(100));
    let mut app = App::new();
    app.insert_resource(time)
        .insert_resource(RuntimeConfig(GameConfig::default()))
        .insert_resource(RuntimePlayerSettings(PlayerSettings::default()))
        .insert_resource(MenuRuntime::default())
        .init_resource::<CommandAcknowledgementRuntime>()
        .init_resource::<CameraDamageRuntime>()
        .insert_resource(CameraCommandQueue(VecDeque::from([CameraRequest {
            reset: false,
            actions: vec![CameraAction {
                direction: CameraDirection::Up,
                amount: 2,
            }],
            follow: None,
            focus_building: None,
        }])))
        .add_systems(Update, camera_zoom_and_commands);
    let camera = app
        .world_mut()
        .spawn((
            TownCamera,
            home,
            Projection::Perspective(PerspectiveProjection::default()),
            TownCameraControllerRuntime::new(home),
        ))
        .id();

    app.update();

    assert!(app.world().resource::<CameraCommandQueue>().0.is_empty());
    let camera = app.world().entity(camera);
    let transform = camera.get::<Transform>().unwrap();
    let controller = camera.get::<TownCameraControllerRuntime>().unwrap();
    assert!((controller.move_target.x - (home.translation.x + 24.0)).abs() < 0.001);
    assert!(transform.translation.x > home.translation.x);
    assert!(transform.translation.x < controller.move_target.x);
    assert!((transform.translation.z - home.translation.z).abs() < 0.001);
}

#[test]
fn damage_camera_selects_the_damaged_town_target_closest_to_the_town_hall() {
    let content = embedded_content();
    let town_hall = StableId::new("building:townhall").unwrap();
    let house_definition = &content.buildings[&StableId::new("building:house").unwrap()];
    let house = StableId::new("building:runtime_damage_focus").unwrap();
    let citizen = StableId::new("npc:damage_focus").unwrap();
    let mut simulation = WorldSimulation::new(1);
    simulation.buildings.insert(
        town_hall.clone(),
        BuildingState {
            id: town_hall,
            archetype: content.buildings[&StableId::new("building:townhall").unwrap()]
                .archetype
                .clone(),
            position: GridPos { x: 20, z: 20 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 1_000,
            complete: true,
        },
    );
    simulation.buildings.insert(
        house.clone(),
        BuildingState {
            id: house.clone(),
            archetype: house_definition.archetype.clone(),
            position: GridPos { x: 22, z: 20 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 500,
            complete: true,
        },
    );
    assert!(simulation.join_player(citizen.clone(), GridPos { x: 40, z: 40 }));
    let mut app = App::new();
    app.insert_resource(RuntimeContent(content))
        .insert_resource(SimulationRuntime(simulation))
        .init_resource::<CameraDamageRuntime>()
        .add_systems(Update, track_camera_damage_focus);
    app.update();
    {
        let mut simulation = app.world_mut().resource_mut::<SimulationRuntime>();
        simulation.0.damage_actor(&citizen, 10).unwrap();
        simulation.0.damage_building(&house, 10).unwrap();
    }
    app.update();
    let damage = app.world().resource::<CameraDamageRuntime>();
    assert_eq!(
        damage.latest_target,
        Some(CameraFocusTarget::Building(house))
    );
    assert_eq!(damage.sequence, 1);
}

#[test]
fn combat_camera_holds_a_target_for_five_seconds_before_redirecting() {
    let home = Transform::from_xyz(0.0, 30.0, 0.0);
    let first = StableId::new("twitch:combat_camera_first").unwrap();
    let second = StableId::new("twitch:combat_camera_second").unwrap();
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_millis(100));
    let mut app = App::new();
    app.insert_resource(time)
        .insert_resource(RuntimeConfig(GameConfig::default()))
        .insert_resource(RuntimePlayerSettings(PlayerSettings::default()))
        .insert_resource(MenuRuntime::default())
        .init_resource::<CommandAcknowledgementRuntime>()
        .insert_resource(CameraDamageRuntime {
            latest_target: Some(CameraFocusTarget::Citizen(first.clone())),
            sequence: 1,
            ..default()
        })
        .init_resource::<CameraCommandQueue>()
        .add_systems(Update, camera_zoom_and_commands);
    let camera = app
        .world_mut()
        .spawn((
            TownCamera,
            home,
            Projection::Perspective(PerspectiveProjection::default()),
            {
                let mut controller = TownCameraControllerRuntime::new(home);
                controller.auto_shot = AutoCameraShot::Town;
                controller.seconds_since_acknowledgement = AUTO_CAMERA_IDLE_SECONDS;
                controller
            },
        ))
        .id();
    for (id, x) in [(first.clone(), 10.0), (second.clone(), 20.0)] {
        app.world_mut().spawn((
            Agent {
                id,
                kind: ActorKind::Player,
                archetype: StableId::new("archetype:player").unwrap(),
                goal: AgentGoal::Wander,
                spawn: GridPos { x: 10, z: 10 },
                origin: GridPos { x: 10, z: 10 },
                navigation_position: placement_to_navigation_centre(GridPos { x: 10, z: 10 }),
                path: Vec::new(),
                path_index: 0,
                target: GridPos { x: 10, z: 10 },
                action_cooldown_seconds: 0.0,
                action_started: false,
                repath_remaining_seconds: 0.0,
                health_regen_accumulator: 0.0,
                wander_sequence: 0,
                previous_wander_origin: None,
            },
            Transform::from_xyz(x, 1.0, 10.0),
        ));
    }

    app.update();
    let focused = |app: &App| {
        app.world()
            .entity(camera)
            .get::<TownCameraControllerRuntime>()
            .and_then(|controller| controller.temporary_focus.as_ref())
            .map(|focus| focus.target.clone())
    };
    assert_eq!(
        focused(&app),
        Some(CameraFocusTarget::Citizen(first.clone()))
    );

    {
        let mut damage = app.world_mut().resource_mut::<CameraDamageRuntime>();
        damage.latest_target = Some(CameraFocusTarget::Citizen(second.clone()));
        damage.sequence = 2;
    }
    app.update();
    assert_eq!(focused(&app), Some(CameraFocusTarget::Citizen(first)));

    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::from_secs_f32(5.0));
    app.world_mut()
        .resource_mut::<CameraDamageRuntime>()
        .sequence = 3;
    app.update();
    assert_eq!(focused(&app), Some(CameraFocusTarget::Citizen(second)));
}

#[test]
fn inactive_camera_smoothly_returns_to_home_position_and_zoom() {
    let home = Transform::from_xyz(0.0, 30.0, 0.0);
    let current = Transform::from_xyz(24.0, 40.0, 12.0);
    let mut controller = TownCameraControllerRuntime::new(home);
    controller.move_target = current.translation;
    controller.zoom_target_height = current.translation.y;
    controller.seconds_since_acknowledgement = AUTO_CAMERA_IDLE_SECONDS - 0.05;
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_millis(100));
    let mut app = App::new();
    app.insert_resource(time)
        .insert_resource(RuntimeConfig(GameConfig::default()))
        .insert_resource(RuntimePlayerSettings(PlayerSettings::default()))
        .insert_resource(MenuRuntime::default())
        .init_resource::<CommandAcknowledgementRuntime>()
        .init_resource::<CameraDamageRuntime>()
        .init_resource::<CameraCommandQueue>()
        .add_systems(Update, camera_zoom_and_commands);
    let camera = app
        .world_mut()
        .spawn((
            TownCamera,
            current,
            Projection::Perspective(PerspectiveProjection::default()),
            controller,
        ))
        .id();

    app.update();

    let camera = app.world().entity(camera);
    let transform = camera.get::<Transform>().unwrap();
    let controller = camera.get::<TownCameraControllerRuntime>().unwrap();
    assert_eq!(controller.move_target, home.translation);
    assert!((controller.zoom_target_height - home.translation.y).abs() < 0.001);
    assert!(transform.translation.x > home.translation.x);
    assert!(transform.translation.x < current.translation.x);
    assert!(transform.translation.y > home.translation.y);
    assert!(transform.translation.y < current.translation.y);
}

#[test]
fn idle_camera_director_cycles_citizens_and_town_then_yields_to_any_acknowledged_command() {
    let home =
        Transform::from_xyz(-33.5, 33.240_562, 0.0).looking_to(Vec3::new(1.0, -1.0, 0.0), Vec3::Y);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(AUTO_CAMERA_IDLE_SECONDS));
    let mut app = App::new();
    app.insert_resource(time)
        .insert_resource(RuntimeConfig(GameConfig::default()))
        .insert_resource(RuntimePlayerSettings(PlayerSettings::default()))
        .insert_resource(MenuRuntime::default())
        .init_resource::<CommandAcknowledgementRuntime>()
        .init_resource::<CameraDamageRuntime>()
        .init_resource::<CameraCommandQueue>()
        .add_systems(Update, camera_zoom_and_commands);
    let camera = app
        .world_mut()
        .spawn((
            TownCamera,
            home,
            Projection::Perspective(PerspectiveProjection::default()),
            TownCameraControllerRuntime::new(home),
        ))
        .id();
    for (id, position) in [
        ("twitch:auto_camera_a", GridPos { x: 30, z: 30 }),
        ("twitch:auto_camera_b", GridPos { x: 40, z: 42 }),
        ("npc:auto_camera_recruit", GridPos { x: 35, z: 36 }),
    ] {
        app.world_mut().spawn((
            Agent {
                id: StableId::new(id).unwrap(),
                kind: ActorKind::Player,
                archetype: StableId::new("archetype:player").unwrap(),
                goal: AgentGoal::Wander,
                spawn: position,
                origin: position,
                navigation_position: placement_to_navigation_centre(position),
                path: Vec::new(),
                path_index: 0,
                target: position,
                action_cooldown_seconds: 0.0,
                action_started: false,
                repath_remaining_seconds: 0.0,
                health_regen_accumulator: 0.0,
                wander_sequence: 0,
                previous_wander_origin: None,
            },
            Transform::from_xyz(f32::from(position.x), 1.0, f32::from(position.z)),
        ));
    }

    app.update();
    assert_eq!(
        app.world()
            .entity(camera)
            .get::<TownCameraControllerRuntime>()
            .unwrap()
            .auto_shot,
        AutoCameraShot::Town
    );

    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::from_secs_f32(AUTO_CAMERA_TOWN_SHOT_SECONDS));
    app.update();
    let first_citizen = app
        .world()
        .entity(camera)
        .get::<TownCameraControllerRuntime>()
        .unwrap()
        .auto_shot
        .clone();
    assert!(matches!(first_citizen, AutoCameraShot::Citizen(_)));
    assert!(!matches!(
        first_citizen,
        AutoCameraShot::Citizen(ref citizen)
            if citizen.as_str() == "npc:auto_camera_recruit"
    ));

    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::from_secs_f32(AUTO_CAMERA_CITIZEN_SHOT_SECONDS));
    app.update();
    let second_citizen = app
        .world()
        .entity(camera)
        .get::<TownCameraControllerRuntime>()
        .unwrap()
        .auto_shot
        .clone();
    assert!(matches!(second_citizen, AutoCameraShot::Citizen(_)));
    assert!(!matches!(
        second_citizen,
        AutoCameraShot::Citizen(ref citizen)
            if citizen.as_str() == "npc:auto_camera_recruit"
    ));
    assert_ne!(second_citizen, first_citizen);

    for _ in 0..2 {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f32(AUTO_CAMERA_CITIZEN_SHOT_SECONDS));
        app.update();
    }
    assert_eq!(
        app.world()
            .entity(camera)
            .get::<TownCameraControllerRuntime>()
            .unwrap()
            .auto_shot,
        AutoCameraShot::Town
    );

    app.world_mut()
        .resource_mut::<CommandAcknowledgementRuntime>()
        .acknowledge();
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::from_millis(16));
    app.update();
    let controller = app
        .world()
        .entity(camera)
        .get::<TownCameraControllerRuntime>()
        .unwrap();
    assert_eq!(controller.auto_shot, AutoCameraShot::Inactive);
    assert!(controller.seconds_since_acknowledgement <= f32::EPSILON);
    assert_eq!(controller.move_target, home.translation);
    assert!((controller.zoom_target_height - home.translation.y).abs() < f32::EPSILON);
}

#[test]
fn citizen_auto_camera_translation_centres_the_follow_target_at_close_zoom() {
    let home =
        Transform::from_xyz(-33.5, 33.240_562, 0.0).looking_to(Vec3::new(1.0, -1.0, 0.0), Vec3::Y);
    let citizen = Vec3::new(25.0, 2.0, -12.0);
    let translation = auto_camera_citizen_translation(&home, citizen);
    let focus = citizen + Vec3::Y * AUTO_CAMERA_CITIZEN_FOCUS_HEIGHT;
    let forward = home.forward().as_vec3();
    let distance = (focus - translation).dot(forward);
    assert!(distance > 0.0);
    assert!((translation + forward * distance).distance(focus) < 0.000_1);
    assert!((translation.y - 17.0).abs() < 0.000_1);
    assert!(translation.y < home.translation.y);
}

#[test]
fn camera_smoothing_is_frame_rate_independent() {
    let one_step = frame_independent_lerp_factor(UNITY_TOWN_CAMERA_MOVE_SMOOTHNESS, 1.0);
    let sixty_fps = frame_independent_lerp_factor(UNITY_TOWN_CAMERA_MOVE_SMOOTHNESS, 1.0 / 60.0);
    let accumulated = 1.0 - (1.0 - sixty_fps).powi(60);
    assert!((one_step - accumulated).abs() < 0.000_01);
    assert!(one_step > 0.99 && one_step < 1.0);
}

#[test]
fn follow_request_retargets_without_leaving_the_automatic_camera_director() {
    let home =
        Transform::from_xyz(-33.5, 33.240_562, 0.0).looking_to(Vec3::new(1.0, -1.0, 0.0), Vec3::Y);
    let target = StableId::new("twitch:camera_guest").unwrap();
    let position = GridPos { x: 44, z: 38 };
    let mut simulation = WorldSimulation::new(17);
    assert!(simulation.join_player(target.clone(), position));
    let mut controller = TownCameraControllerRuntime::new(home);
    controller.seconds_since_acknowledgement = AUTO_CAMERA_IDLE_SECONDS;
    controller.auto_shot = AutoCameraShot::Town;
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_millis(100));
    let mut app = App::new();
    app.insert_resource(time)
        .insert_resource(RuntimeConfig(GameConfig::default()))
        .insert_resource(RuntimePlayerSettings(PlayerSettings::default()))
        .insert_resource(MenuRuntime::default())
        .insert_resource(SimulationRuntime(simulation))
        .init_resource::<CommandAcknowledgementRuntime>()
        .init_resource::<CameraDamageRuntime>()
        .insert_resource(CameraCommandQueue(VecDeque::from([CameraRequest {
            reset: false,
            actions: Vec::new(),
            follow: Some(target.clone()),
            focus_building: None,
        }])))
        .add_systems(Update, camera_zoom_and_commands);
    let camera = app
        .world_mut()
        .spawn((
            TownCamera,
            home,
            Projection::Perspective(PerspectiveProjection::default()),
            controller,
        ))
        .id();
    app.world_mut().spawn((
        Agent {
            id: target.clone(),
            kind: ActorKind::Player,
            archetype: StableId::new("archetype:player").unwrap(),
            goal: AgentGoal::Wander,
            spawn: position,
            origin: position,
            navigation_position: placement_to_navigation_centre(position),
            path: Vec::new(),
            path_index: 0,
            target: position,
            action_cooldown_seconds: 0.0,
            action_started: false,
            repath_remaining_seconds: 0.0,
            health_regen_accumulator: 0.0,
            wander_sequence: 0,
            previous_wander_origin: None,
        },
        Transform::from_xyz(12.0, 1.0, -8.0),
    ));

    app.update();

    let controller = app
        .world()
        .entity(camera)
        .get::<TownCameraControllerRuntime>()
        .unwrap();
    assert_eq!(
        controller.auto_shot,
        AutoCameraShot::Citizen(target.clone())
    );
    assert!(
        (controller.seconds_since_acknowledgement - AUTO_CAMERA_IDLE_SECONDS).abs() <= f32::EPSILON
    );
    assert!(!command_interrupts_auto_camera(&ChatCommand::Follow(Some(
        target
    ))));
    assert!(!command_interrupts_auto_camera(&ChatCommand::Vote(
        StableId::new("1").unwrap()
    )));
    assert!(!command_interrupts_auto_camera(
        &ChatCommand::FocusBuilding {
            building: StableId::new("tower").unwrap(),
            index: 2,
        }
    ));
    assert!(command_interrupts_auto_camera(&ChatCommand::Health));
}

#[test]
fn follow_target_resolution_accepts_twitch_names_and_rejects_npcs() {
    let caller = StableId::new("twitch:caller").unwrap();
    let viewer = StableId::new("twitch:some_viewer").unwrap();
    let recruit = StableId::new("npc:recruit").unwrap();
    let mut simulation = WorldSimulation::new(9);
    for actor in [&caller, &viewer, &recruit] {
        assert!(simulation.join_player(actor.clone(), GridPos { x: 1, z: 1 }));
    }
    simulation.actors.get_mut(&viewer).unwrap().display_name = Some("Some Viewer".to_owned());
    simulation.actors.get_mut(&viewer).unwrap().login_name = Some("some_viewer".to_owned());

    assert_eq!(
        resolve_auto_camera_follow_target(&simulation, &caller, None).unwrap(),
        caller
    );
    assert_eq!(
        resolve_auto_camera_follow_target(
            &simulation,
            &caller,
            Some(&StableId::new("some_viewer").unwrap()),
        )
        .unwrap(),
        viewer
    );
    assert!(resolve_auto_camera_follow_target(&simulation, &caller, Some(&recruit)).is_err());
}

#[test]
fn foliage_acceptance_camera_holds_then_orbits_zooms_and_returns() {
    let focus = Vec3::new(3.0, 1.0, -7.0);
    let starting =
        Transform::from_translation(focus + Vec3::new(30.0, 35.0, 30.0)).looking_at(focus, Vec3::Y);
    let stationary = foliage_capture_camera(starting, 0.75);
    assert!(stationary.translation.distance(starting.translation) < 0.000_1);
    assert!(stationary.rotation.angle_between(starting.rotation) < 0.000_1);

    let moving = foliage_capture_camera(starting, 2.25);
    assert!(moving.translation.distance(starting.translation) > 5.0);
    assert!(moving.translation.distance(focus) < starting.translation.distance(focus));

    let returned = foliage_capture_camera(starting, 6.0);
    assert!(returned.translation.distance(starting.translation) < 0.000_1);
    assert!(returned.rotation.angle_between(starting.rotation) < 0.000_1);
}

#[test]
fn town_camera_keeps_the_unity_angle_with_the_wider_upper_third_frame() {
    let config = GameConfig::default();
    let focus = Vec3::new(7.0, 2.5, -11.0);
    let transform = unity_town_camera_transform(focus);
    assert!(
        transform.translation.distance(
            focus + UNITY_TOWN_CAMERA_OFFSET - Vec3::X * UNITY_TOWN_CAMERA_FOCUS_BACK_SHIFT,
        ) < 0.000_1
    );
    assert!(
        transform
            .forward()
            .as_vec3()
            .distance(Vec3::new(1.0, -1.0, 0.0).normalize())
            < 0.000_1
    );
    let Projection::Perspective(projection) = town_camera_projection(UNITY_TOWN_CAMERA_FOV_DEGREES)
    else {
        panic!("shipping town camera must remain perspective");
    };
    assert!((projection.fov.to_degrees() - 60.0).abs() < 0.000_1);
    assert!((projection.near - 0.3).abs() < 0.000_1);
    assert!((projection.far - 1_000.0).abs() < 0.000_1);
    let clip = Mat4::perspective_rh(60.0_f32.to_radians(), 16.0 / 9.0, 0.3, 1_000.0)
        * transform.to_matrix().inverse()
        * focus.extend(1.0);
    let top_fraction = (1.0 - clip.y / clip.w) * 0.5;
    assert!(
        (0.29..=0.37).contains(&top_fraction),
        "Town Hall should frame one third from the top, got {top_fraction}"
    );
    let controller = TownCameraControllerRuntime::new(transform);
    assert!((controller.zoom_target_height - transform.translation.y).abs() < f32::EPSILON);
    let constrained = constrain_town_camera_position(
        Vec3::new(-500.0, UNITY_TOWN_CAMERA_MAX_HEIGHT, 500.0),
        &config.world,
    );
    assert_eq!(
        constrained,
        Vec3::new(-260.0, UNITY_TOWN_CAMERA_MAX_HEIGHT, 200.0)
    );
    assert!(
        (camera_ground_focus(
            &unity_town_camera_transform(Vec3::new(-200.0, 0.0, 200.0))
                .with_translation(constrained)
        ) - Vec2::new(-200.0, 200.0))
        .length()
            < 0.001,
        "the maximum-zoom camera must still reach the far island corner"
    );
}

#[test]
fn citizen_idle_wander_replans_only_after_the_unity_pause() {
    assert_eq!(
        idle_wander_replan_delay(&ActorKind::Player, &AgentGoal::Wander),
        Some(3.0)
    );
    assert_eq!(
        idle_wander_replan_delay(
            &ActorKind::Player,
            &AgentGoal::Attack(StableId::new("npc:idle-delay-target").unwrap()),
        ),
        None
    );
    assert_eq!(
        idle_wander_replan_delay(&ActorKind::Enemy, &AgentGoal::Wander),
        None
    );
}

#[test]
fn station_anchored_wander_is_reachable_and_never_immediately_backtracks() {
    let config = GameConfig::default();
    let world = generate_world(&config.world);
    let actor = StableId::new("npc:wander_regression").unwrap();
    let centre = GridPos {
        x: config.world.width / 2,
        z: config.world.height / 2,
    };
    let anchor = nearest_walkable(&world, centre).unwrap();
    let mut current = anchor;
    let mut previous = None;
    let mut visited = BTreeSet::from([current]);
    for sequence in 0..24 {
        let next = deterministic_wander_target_step(
            &world,
            &actor,
            anchor,
            current,
            sequence,
            previous,
            |candidate| world.navigation.find_path(current, candidate).is_ok(),
        );
        assert_ne!(next, current);
        assert_ne!(Some(next), previous);
        assert!(next.x.abs_diff(anchor.x) <= 5);
        assert!(next.z.abs_diff(anchor.z) <= 5);
        assert!(world.navigation.find_path(current, next).is_ok());
        visited.insert(next);
        previous = Some(current);
        current = next;
    }
    assert!(
        visited.len() >= 8,
        "idle wandering must explore its station radius instead of cycling a tiny route"
    );
}

#[test]
fn station_wander_skips_a_coarse_candidate_rejected_by_live_navigation() {
    let config = GameConfig::default();
    let world = generate_world(&config.world);
    let actor = StableId::new("npc:live_wander_filter").unwrap();
    let centre = GridPos {
        x: config.world.width / 2,
        z: config.world.height / 2,
    };
    let anchor = nearest_walkable(&world, centre).unwrap();
    let coarse_choice =
        deterministic_wander_target_step(&world, &actor, anchor, anchor, 0, None, |candidate| {
            world.navigation.find_path(anchor, candidate).is_ok()
        });
    let live_choice =
        deterministic_wander_target_step(&world, &actor, anchor, anchor, 0, None, |candidate| {
            candidate != coarse_choice && world.navigation.find_path(anchor, candidate).is_ok()
        });

    assert_ne!(live_choice, anchor);
    assert_ne!(live_choice, coarse_choice);
    assert!(world.navigation.find_path(anchor, live_choice).is_ok());
}

#[test]
fn generated_visual_offsets_preserve_the_classified_source_point() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world_with_content(&config.world, &content);
    let max_axis_offset = config.world.cell_size * 0.25;
    let offsets = world
        .resources
        .iter()
        .filter(|resource| resource.target_kind.as_str() != "target:fish")
        .take(512)
        .map(|resource| {
            let offset = locational_visual_offset(
                world.seed,
                &resource.id,
                resource.position,
                resource.offset_milli_cells,
                config.world.cell_size,
            );
            assert_eq!(
                offset,
                locational_visual_offset(
                    world.seed,
                    &resource.id,
                    resource.position,
                    resource.offset_milli_cells,
                    config.world.cell_size,
                )
            );
            let expected = Vec2::new(
                f32::from(resource.offset_milli_cells[0]) * config.world.cell_size / 1_000.0,
                f32::from(resource.offset_milli_cells[1]) * config.world.cell_size / 1_000.0,
            );
            assert_eq!(offset, expected);
            assert!(offset.x.abs() <= max_axis_offset + f32::EPSILON);
            assert!(offset.y.abs() <= max_axis_offset + f32::EPSILON);
            (offset.x.to_bits(), offset.y.to_bits())
        })
        .collect::<HashSet<_>>();
    assert!(!offsets.is_empty());
    for resource in world
        .resources
        .iter()
        .filter(|resource| resource.target_kind.as_str() != "target:fish")
        .take(512)
    {
        let position = generated_resource_world_position(resource, &config, &world);
        let rendered_surface =
            terrain_surface_height_at_world(&world, &config, position.x, position.z)
                .expect("generated resource jitter remains on the terrain mesh");
        assert!((position.y - rendered_surface).abs() < 0.000_1);
    }

    let foliage_offsets = world
        .foliage
        .iter()
        .take(512)
        .map(|foliage| {
            let offset = locational_visual_offset(
                world.seed,
                &foliage.id,
                foliage.position,
                foliage.offset_milli_cells,
                config.world.cell_size,
            );
            let foliage_max_axis_offset = config.world.cell_size * 0.5;
            assert!(offset.x.abs() <= foliage_max_axis_offset + f32::EPSILON);
            assert!(offset.y.abs() <= foliage_max_axis_offset + f32::EPSILON);
            let expected = Vec2::new(
                f32::from(foliage.offset_milli_cells[0]) * config.world.cell_size / 1_000.0,
                f32::from(foliage.offset_milli_cells[1]) * config.world.cell_size / 1_000.0,
            );
            assert_eq!(offset, expected);
            (offset.x.to_bits(), offset.y.to_bits())
        })
        .collect::<HashSet<_>>();
    assert!(
        foliage_offsets.len() > 3,
        "source foliage offsets must retain more than one grid location"
    );
    assert!(foliage_offsets.iter().any(|(x, z)| {
        f32::from_bits(*x).abs() > max_axis_offset || f32::from_bits(*z).abs() > max_axis_offset
    }));
}
