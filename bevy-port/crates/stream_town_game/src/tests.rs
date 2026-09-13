#![allow(clippy::float_cmp)]

use super::bootstrap::{
    is_transient_surface_configuration_error, player_window_mode, startup_window_mode,
};
use super::runtime::timelapse::{
    CityTimelapseRuntime, draw_timelapse_label, next_timelapse_frame_index,
    timelapse_frame_is_blank,
};
use super::*;
use sha2::{Digest, Sha256};
use stream_town_domain::generate_world;

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

#[test]
fn gameplay_gate_remains_closed_until_loading_entities_and_runtimes_are_gone() {
    let mut app = App::new();
    app.insert_resource(AutomaticResumeRuntime::default())
        .add_systems(Update, confirm_gameplay_ready);
    let loading_entity = app.world_mut().spawn(LoadingScreenEntity).id();
    app.update();
    assert!(!app.world().contains_resource::<GameplayReady>());

    app.world_mut().despawn(loading_entity);
    app.world_mut()
        .insert_resource(WorldLoadingRuntime::default());
    app.update();
    assert!(!app.world().contains_resource::<GameplayReady>());

    app.world_mut().remove_resource::<WorldLoadingRuntime>();
    app.world_mut()
        .insert_resource(WorldRevealRuntime::default());
    app.update();
    assert!(!app.world().contains_resource::<GameplayReady>());

    app.world_mut().remove_resource::<WorldRevealRuntime>();
    app.update();
    assert!(app.world().contains_resource::<GameplayReady>());
}

#[test]
fn automatic_resume_gate_never_exposes_an_unapplied_fallback_world() {
    let mut app = App::new();
    app.insert_resource(AutomaticResumeRuntime::new(Some(PathBuf::from(
        ".stream-town/saves/Tonyville.stbevy",
    ))))
    .add_systems(Update, confirm_gameplay_ready);

    app.update();
    assert!(!app.world().contains_resource::<GameplayReady>());

    app.world_mut()
        .resource_mut::<AutomaticResumeRuntime>()
        .applied = true;
    app.update();
    assert!(app.world().contains_resource::<GameplayReady>());
}

#[test]
fn returning_to_main_menu_recreates_the_cover_before_scene_construction() {
    let mut app = App::new();
    app.insert_resource(RuntimeConfig(GameConfig::default()))
        .insert_resource(RuntimeContent(embedded_content()))
        .insert_resource(RuntimeAssetRoot(PathBuf::from("unused-test-assets")))
        .add_systems(
            Update,
            (spawn_loading_screen, ensure_main_menu_loading).chain(),
        );

    app.update();

    let mut covers = app
        .world_mut()
        .query_filtered::<Entity, With<LoadingScreenEntity>>();
    assert!(covers.iter(app.world()).count() >= 4);
    assert_eq!(
        app.world().resource::<MenuLoadingRuntime>().destination,
        BootDestination::MainMenu
    );
    assert!(app.world().resource::<MenuLoadingRuntime>().progress.abs() < f32::EPSILON);
}

#[test]
fn gameplay_simulation_set_is_paused_without_the_ready_marker() {
    #[derive(Resource, Default)]
    struct Counter(u32);

    fn count_update(mut counter: ResMut<Counter>) {
        counter.0 += 1;
    }

    let mut app = App::new();
    app.init_resource::<Counter>()
        .configure_sets(
            Update,
            GameplaySimulationSet.run_if(resource_exists::<GameplayReady>),
        )
        .add_systems(Update, count_update.in_set(GameplaySimulationSet));
    app.update();
    assert_eq!(app.world().resource::<Counter>().0, 0);
    app.world_mut().insert_resource(GameplayReady);
    app.update();
    assert_eq!(app.world().resource::<Counter>().0, 1);
    app.world_mut().remove_resource::<GameplayReady>();
    app.update();
    assert_eq!(app.world().resource::<Counter>().0, 1);
}

#[test]
fn stream_only_agent_simulation_accumulates_real_time_at_sixty_hertz() {
    let mut cadence = AgentSimulationCadenceState::default();
    assert!(!advance_agent_simulation_cadence(
        &mut cadence,
        Duration::from_millis(8),
        true,
        true,
    ));
    assert!(advance_agent_simulation_cadence(
        &mut cadence,
        Duration::from_millis(9),
        true,
        true,
    ));
    assert_eq!(cadence.delta, Duration::from_millis(17));
    assert_eq!(cadence.accumulated, Duration::ZERO);

    assert!(advance_agent_simulation_cadence(
        &mut cadence,
        Duration::from_millis(4),
        false,
        true,
    ));
    assert_eq!(cadence.delta, Duration::from_millis(4));
    assert!(!advance_agent_simulation_cadence(
        &mut cadence,
        Duration::from_millis(20),
        true,
        false,
    ));
    assert_eq!(cadence.delta, Duration::ZERO);
}

#[test]
fn stream_only_night_lights_sync_at_output_cadence() {
    let mut cadence = AgentSimulationCadenceState::default();
    for _ in 0..3 {
        assert!(!advance_stream_only_cadence(
            &mut cadence,
            Duration::from_millis(8),
            true,
            true,
            STREAM_ONLY_NIGHT_LIGHT_SYNC_HZ,
        ));
    }
    assert!(advance_stream_only_cadence(
        &mut cadence,
        Duration::from_millis(10),
        true,
        true,
        STREAM_ONLY_NIGHT_LIGHT_SYNC_HZ,
    ));
    assert_eq!(cadence.delta, Duration::from_millis(34));
    assert_eq!(cadence.accumulated, Duration::ZERO);
}

#[test]
fn day_night_cycle_rotates_the_shipping_sun_parent() {
    let day = in_game_sun_transform_for_daylight(1.0);
    let dusk = in_game_sun_transform_for_daylight(0.5);
    let night = in_game_sun_transform_for_daylight(0.0);
    assert!(day.rotation.angle_between(dusk.rotation).to_degrees() > 59.0);
    assert!(dusk.rotation.angle_between(night.rotation).to_degrees() > 59.0);
    assert!((day.rotation.angle_between(night.rotation).to_degrees() - 120.0).abs() < 0.01);
    assert_eq!(day.translation, night.translation);
}

#[test]
fn sunset_presentation_preserves_sub_millisecond_daylight_changes() {
    let config = GameConfig::default();
    let day_seconds = f64::from(config.time.seconds_per_day);
    let transition_seconds = f64::from(config.time.transition_seconds);
    let dusk_midpoint = day_seconds * f64::from(config.time.daylight_per_thousand) / 1_000.0
        - transition_seconds * 0.5;
    let first = config.time.sample(dusk_midpoint).daylight;
    let second = config.time.sample(dusk_midpoint + 0.01).daylight;

    assert!((first - second).abs() < 0.001);
    assert_ne!(daylight_signature(first), daylight_signature(second));
    assert_ne!(
        in_game_sun_transform_for_daylight(first).rotation,
        in_game_sun_transform_for_daylight(second).rotation
    );
}

#[test]
fn building_material_cycle_stays_phase_aligned_without_cpu_asset_updates() {
    let config = GameConfig::default();
    let app_elapsed_seconds = 2_345.25;
    for simulation_elapsed_seconds in [0.0, 2_297.0, 2_347.0, 2_400.0, 3_550.0, 85_440.5] {
        let parameters = building_material_time_cycle(
            &config.time,
            simulation_elapsed_seconds,
            app_elapsed_seconds,
        );
        let app_elapsed = Duration::from_secs_f64(app_elapsed_seconds).as_secs_f32();
        let shader_phase =
            f64::from(app_elapsed + parameters.w).rem_euclid(f64::from(parameters.x));
        let cpu = config.time.sample(simulation_elapsed_seconds).daylight;
        let shader_equivalent = config.time.sample(shader_phase).daylight;
        assert!((cpu - shader_equivalent).abs() < 0.000_1);
    }

    let shader = include_str!("../../../assets/shaders/building_material.wgsl");
    assert!(shader.contains("view_bindings::globals.time"));
    assert!(shader.contains("(1.0 - building_daylight())"));
}

#[test]
fn building_material_round_robin_covers_every_instance_with_a_bounded_batch() {
    let total = 83;
    let mut cursor = 0;
    let mut visited = vec![false; total];
    for _ in 0..total.div_ceil(MAX_BUILDING_MATERIAL_UPDATES_PER_FRAME) {
        let batch = round_robin_indices(total, cursor)
            .take(MAX_BUILDING_MATERIAL_UPDATES_PER_FRAME)
            .collect::<Vec<_>>();
        for index in &batch {
            visited[*index] = true;
        }
        cursor = (cursor + batch.len()) % total;
    }

    assert!(visited.into_iter().all(|was_visited| was_visited));
}

#[test]
fn player_runtime_uses_the_single_animation_armature() {
    let content = embedded_content();
    let player =
        archetype_by_source(&content, ArchetypeKind::Player, "Player_Character.prefab").unwrap();
    let scene = runtime_archetype_scene(player).unwrap();
    assert_eq!(scene.asset_path, PLAYER_ANIMATED_MODEL_PATH);
    assert_eq!(scene.source_model, PLAYER_ANIMATED_SOURCE_MODEL);
}

#[test]
fn replaced_converted_scene_becomes_eligible_for_animation_rebinding() {
    let mut app = App::new();
    app.add_systems(Update, repair_replaced_converted_animation_instances);
    let controller = StableId::new("controller:test").unwrap();
    let state = StableId::new("animation_state:test").unwrap();
    let stale_root = app
        .world_mut()
        .spawn((
            ConvertedAnimationSpec {
                controller: controller.clone(),
                state: state.clone(),
                rig_scene: "test.glb".to_owned(),
            },
            ConvertedAnimationInstanceReady,
            ConvertedAnimationApplied,
        ))
        .id();
    let live_root = app
        .world_mut()
        .spawn((
            ConvertedAnimationSpec {
                controller: controller.clone(),
                state,
                rig_scene: "test.glb".to_owned(),
            },
            ConvertedAnimationInstanceReady,
            ConvertedAnimationApplied,
        ))
        .id();
    app.world_mut().spawn(ConvertedAnimationDriver {
        actor_root: live_root,
        controller,
        layers: Vec::new(),
        last_alive: None,
        active_action: None,
        transient_carry_visible: false,
    });

    app.update();

    assert!(
        app.world()
            .get::<ConvertedAnimationApplied>(stale_root)
            .is_none(),
        "a discarded scene hierarchy must not permanently suppress rebinding"
    );
    assert!(
        app.world()
            .get::<ConvertedAnimationApplied>(live_root)
            .is_some(),
        "a live animation driver must retain the applied marker"
    );
}

#[test]
fn game_menu_exposes_state_appropriate_actions_and_save_availability() {
    let in_game = game_menu_text(GameState::InGame, 3, false);
    assert!(in_game.contains("STREAM TOWN MENU"));
    assert!(in_game.contains("Load Game  [No save]"));
    assert!(in_game.contains("> Settings"));
    assert!(in_game.contains("Exit Game"));
    assert!(!in_game.contains("Idle Mode"));

    let main_menu = game_menu_text(GameState::MainMenu, 1, true);
    assert!(main_menu.contains("New town"));
    assert!(main_menu.contains("> Load game\n"));
    assert!(main_menu.contains("Credits"));
    assert!(main_menu.contains("Quit"));
}

#[test]
fn shipping_game_menu_preserves_operator_actions_without_legacy_idle_control() {
    let presentation = embedded_presentation();
    for source_path in GAME_MENU_TEXTURE_PATHS {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path),
            "missing Game Menu texture {source_path}"
        );
    }
    let actions = [
        GameMenuAction::SaveGame,
        GameMenuAction::SaveJumpStart,
        GameMenuAction::LoadGame,
        GameMenuAction::Settings,
        GameMenuAction::GoLive,
        GameMenuAction::ExitGame,
        GameMenuAction::Close,
    ];
    assert_eq!(
        actions.map(|action| game_menu_action_label(action, false)),
        [
            "Save Game",
            "Save Jump-Start",
            "Load Game",
            "Settings",
            "Go Live",
            "Exit Game",
            "Close"
        ]
    );
    assert_eq!(
        game_menu_action_label(GameMenuAction::GoLive, true),
        "End Stream"
    );
    assert!(!game_menu_action_enabled(GameMenuAction::LoadGame, false));
    assert!(game_menu_action_enabled(GameMenuAction::LoadGame, true));
    assert!(game_menu_action_enabled(GameMenuAction::Close, false));
}

#[cfg(target_os = "windows")]
#[test]
fn shared_operator_toggle_ends_an_active_stream_before_checking_setup() {
    let mut runtime = direct_broadcast::DirectBroadcastRuntime::default();
    runtime.set_phase_for_test(direct_broadcast::DirectBroadcastPhase::Broadcasting);
    let mut control = direct_broadcast::DirectBroadcastControl::default();
    let mut feedback = String::new();

    toggle_direct_broadcast(
        &GameConfig::default(),
        &SecretsRuntime::default(),
        &TwitchConnection::default(),
        &runtime,
        &mut control,
        &mut feedback,
    );

    assert!(control.stop_requested_for_test());
    assert!(feedback.contains("Ending"));
}

#[test]
fn game_menu_mouse_actions_use_authoritative_requests_and_settings() {
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("game-menu-actions.stbevy");
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
        .init_state::<GameState>()
        .insert_resource(MenuRuntime {
            page: MenuPage::Game,
            ..default()
        })
        .init_resource::<MenuIoRequest>()
        .init_resource::<SecretsRuntime>()
        .init_resource::<TwitchConnection>()
        .insert_resource(RuntimeConfig(GameConfig::default()))
        .insert_resource(RuntimePlayerSettings(PlayerSettings::default()))
        .insert_resource(SaveRuntime {
            store: NativeSaveStore::new(&save_path),
        });
    #[cfg(target_os = "windows")]
    app.init_resource::<direct_broadcast::DirectBroadcastControl>()
        .init_resource::<direct_broadcast::DirectBroadcastRuntime>();
    app.add_systems(Update, game_menu_buttons);
    app.world_mut()
        .spawn((Interaction::Pressed, GameMenuAction::SaveGame));

    app.update();

    assert!(app.world().resource::<MenuIoRequest>().save);
    assert_eq!(app.world().resource::<MenuRuntime>().page, MenuPage::Closed);

    app.world_mut().resource_mut::<MenuIoRequest>().save = false;
    app.world_mut().resource_mut::<MenuRuntime>().page = MenuPage::Game;
    app.world_mut()
        .spawn((Interaction::Pressed, GameMenuAction::Settings));
    app.update();

    assert_eq!(
        app.world().resource::<MenuRuntime>().page,
        MenuPage::Settings
    );
    assert_eq!(
        app.world().resource::<MenuRuntime>().return_page,
        MenuPage::Game
    );
}

#[test]
fn cursor_is_hidden_in_the_town_and_visible_for_escape_menu_children() {
    assert!(!cursor_visible_for_state(GameState::Boot, MenuPage::Closed));
    assert!(!cursor_visible_for_state(
        GameState::WorldLoading,
        MenuPage::Closed
    ));
    assert!(!cursor_visible_for_state(
        GameState::InGame,
        MenuPage::Closed
    ));
    assert!(cursor_visible_for_state(GameState::InGame, MenuPage::Game));
    assert!(cursor_visible_for_state(
        GameState::InGame,
        MenuPage::Settings
    ));
    assert!(cursor_visible_for_state(
        GameState::MainMenu,
        MenuPage::Closed
    ));
    assert!(cursor_visible_for_state(
        GameState::Credits,
        MenuPage::Closed
    ));
}

#[test]
fn town_names_have_stable_seeds_and_windows_safe_independent_save_names() {
    assert_eq!(town_name_seed(" Bean Bay "), town_name_seed("bean bay"));
    assert_ne!(town_name_seed("Bean Bay"), town_name_seed("Bean Vale"));
    assert_eq!(town_name_seed("Bobville"), 14_812_036_045_316_836_008);
    assert_eq!(safe_town_filename("Bean: Bay?"), "Bean_ Bay_");
    assert_eq!(safe_town_filename("CON"), "Town_CON");
}

#[test]
fn patch_resume_path_overrides_fixed_smoke_path_without_hiding_the_town_catalog() {
    let resume = PathBuf::from(".stream-town/saves/Tonyville.stbevy");
    let fixed = PathBuf::from("generated/smoke.stbevy");
    assert_eq!(startup_save_path(Some(resume.clone()), Some(fixed)), resume);

    let catalog = TownSaveCatalogRuntime::from_startup_paths(None, Some(resume));
    assert_eq!(catalog.active_town.as_deref(), Some("Tonyville"));
    assert!(catalog.fixed_path.is_none());
    assert_eq!(
        startup_destination(false, false, true),
        BootDestination::WorldLoading
    );
    assert!(automatic_load_requested(false, false, true));
    assert!(!automatic_load_requested(true, false, true));
}

#[test]
fn fixed_new_town_path_names_the_active_town_before_the_first_save() {
    let fixed = PathBuf::from(".stream-town/saves/Beanville.stbevy");
    let catalog = TownSaveCatalogRuntime::from_startup_paths(Some(fixed), None);
    assert_eq!(catalog.active_town.as_deref(), Some("Beanville"));
}

#[test]
fn automatic_resume_seed_is_loaded_before_world_generation() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("Tonyville.stbevy");
    let mut config = GameConfig::default();
    config.world.seed = 7_004_668_131_361_051_943;
    let generated = generate_world_with_content(&config.world, &embedded_content());
    let snapshot = WorldSnapshot {
        schema_version: CURRENT_WORLD_SNAPSHOT_SCHEMA,
        world_seed: generated.seed,
        generator_version: generated.generator_version,
        world_hash: generated.deterministic_hash,
        elapsed_seconds: 0,
        actors: Vec::new(),
        simulation: WorldSimulation::new(config.world.seed),
        resource_nodes: BTreeMap::new(),
        traversal_wear: BTreeMap::new(),
        legacy_terrain_mesh: None,
        legacy_migration: None,
    };
    NativeSaveStore::new(&path).write(&snapshot).unwrap();

    assert_eq!(
        automatic_resume_world_seed(Some(&path)),
        Some(config.world.seed)
    );
}

#[test]
fn jump_start_saves_are_identifiable_and_never_reuse_an_existing_path() {
    let directory = tempfile::tempdir().unwrap();
    let active = directory.path().join("Bean Bay.stbevy");
    let first = jump_start_snapshot_path(&active);
    assert_eq!(first.file_name().unwrap(), "Bean Bay.jumpstart.stbevy");
    std::fs::write(&first, b"reserved").unwrap();
    let second = jump_start_snapshot_path(&active);
    assert_eq!(second.file_name().unwrap(), "Bean Bay 2.jumpstart.stbevy");
    assert!(is_jump_start_path(&first));
    assert!(is_jump_start_path(&second));
}

#[test]
fn rotating_save_backups_are_not_listed_as_independent_towns() {
    let directory = tempfile::tempdir().unwrap();
    let current = directory.path().join("Bean Bay.stbevy");
    std::fs::write(&current, b"current").unwrap();
    std::fs::write(directory.path().join("Bean Bay.stbevy.bak"), b"backup 1").unwrap();
    std::fs::write(directory.path().join("Bean Bay.stbevy.bak.2"), b"backup 2").unwrap();
    let catalog = TownSaveCatalogRuntime {
        directory: directory.path().to_path_buf(),
        legacy_path: directory.path().join("legacy.stbevy"),
        fixed_path: None,
        active_town: None,
    };
    let entries = catalog.entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].path, current);
}

#[test]
fn new_town_dialog_selects_an_independent_save_and_seed_before_consent() {
    let directory = tempfile::tempdir().unwrap();
    let initial = directory.path().join("legacy.stbevy");
    let mut menu = MenuRuntime::default();
    open_new_town_dialog(&mut menu);
    let mut app = App::new();
    app.insert_resource(menu)
        .insert_resource(TownSaveCatalogRuntime {
            directory: directory.path().to_path_buf(),
            legacy_path: initial.clone(),
            fixed_path: None,
            active_town: None,
        })
        .insert_resource(SaveRuntime {
            store: NativeSaveStore::new(initial),
        })
        .insert_resource(RuntimeConfig(GameConfig::default()))
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<InputFocus>()
        .add_systems(Update, town_dialog_buttons);
    app.world_mut()
        .spawn((TownNameField, EditableText::new("Bean Bay")));
    app.world_mut()
        .spawn((Interaction::Pressed, TownDialogAction::Create));

    app.update();

    assert_eq!(
        app.world().resource::<MenuRuntime>().page,
        MenuPage::GoLiveConfirmation
    );
    assert_eq!(
        app.world().resource::<RuntimeConfig>().0.world.seed,
        town_name_seed("Bean Bay")
    );
    assert_eq!(
        app.world()
            .resource::<SaveRuntime>()
            .store
            .path()
            .file_name()
            .unwrap(),
        "Bean Bay.stbevy"
    );
}

#[test]
fn night_waves_repeat_every_two_minutes_and_scale_with_citizens() {
    let mut runtime = NightEnemyWaveRuntime::default();
    assert!(night_enemy_wave_due(&mut runtime, 1, 0.0));
    assert_eq!(runtime.wave_index, 1);
    assert!(!night_enemy_wave_due(&mut runtime, 1, 119.0));
    assert!(night_enemy_wave_due(&mut runtime, 1, 1.0));
    assert_eq!(runtime.wave_index, 2);
    assert!(night_enemy_wave_due(&mut runtime, 2, 0.0));
    assert_eq!(runtime.wave_index, 1);
    assert_eq!(raid_enemies_per_wave(1), 2);
    assert_eq!(raid_enemies_per_wave(2), 2);
    assert_eq!(raid_enemies_per_wave(18), 18);
    assert_eq!(raid_enemies_per_wave(25), 25);
    assert_eq!(raid_enemies_per_wave(100), 25);
}

#[test]
fn community_events_apply_their_authored_role_and_invasion_rates() {
    let prospector = StableId::new("role:prospector").unwrap();
    let forester = StableId::new("role:forester").unwrap();
    let tender = StableId::new("role:tender").unwrap();

    assert_eq!(
        community_event_role_rate_multiplier(CommunityEvent::ProspectingBoom, &prospector),
        3.0
    );
    assert_eq!(
        community_event_role_rate_multiplier(CommunityEvent::ProspectingBoom, &forester),
        0.5
    );
    assert_eq!(
        community_event_role_rate_multiplier(CommunityEvent::ReforestationBoom, &forester),
        3.0
    );
    assert_eq!(
        community_event_role_rate_multiplier(CommunityEvent::AgriculturalBoom, &tender),
        3.0
    );
    assert_eq!(
        community_event_from_id(&StableId::new("event:market").unwrap()),
        Some(CommunityEvent::Market)
    );
    assert_eq!(community_event_name(CommunityEvent::Market), "Market");
    assert_eq!(
        event_adjusted_wave_size(2, Some(CommunityEvent::Invasion)),
        3
    );
    assert_eq!(
        event_adjusted_wave_size(25, Some(CommunityEvent::Invasion)),
        38
    );
    assert_eq!(event_adjusted_wave_size(25, None), 25);

    assert_eq!(
        community_adjusted_regeneration_interval_seconds(
            &forester,
            1,
            Some(CommunityEvent::ReforestationBoom),
        ),
        Some(67.0)
    );
    assert_eq!(
        community_adjusted_regeneration_interval_seconds(
            &forester,
            1,
            Some(CommunityEvent::ProspectingBoom),
        ),
        Some(402.0)
    );
    assert_eq!(
        community_adjusted_regeneration_interval_seconds(
            &tender,
            1,
            Some(CommunityEvent::AgriculturalBoom),
        ),
        Some(1_200.0)
    );

    let config = GameConfig::default();
    let content = embedded_content();
    let mut simulation = WorldSimulation::new(config.world.seed);
    let actor_id = StableId::new("twitch:market-worker").unwrap();
    simulation.join_player(actor_id.clone(), GridPos { x: 1, z: 1 });
    simulation.actors.get_mut(&actor_id).unwrap().role = prospector;
    simulation.active_community_event = Some(CommunityEvent::Market);
    let actor = &simulation.actors[&actor_id];
    assert!(actor_resource_storage_has_room(
        &config,
        &content,
        &simulation,
        actor
    ));
}

#[test]
fn fine_placement_border_allows_a_walkable_gap_but_rejects_physical_encroachment() {
    let mut config = GameConfig::default();
    config.world.width = 16;
    config.world.height = 16;
    let content = embedded_content();
    let world = GeneratedWorld {
        seed: 9,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(
            16,
            16,
            vec![false; 16 * 16],
            vec![100; 16 * 16],
        )
        .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    let guardhouse_id = StableId::new("building:guardhouse").unwrap();
    let definition = &content.buildings[&guardhouse_id];
    let existing_id = StableId::new("building:existing-guardhouse").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.buildings.insert(
        existing_id.clone(),
        BuildingState {
            id: existing_id,
            archetype: definition.archetype.clone(),
            position: GridPos { x: 5, z: 5 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 100,
            complete: true,
        },
    );
    let candidate = |x| BuildingPlacement {
        building: guardhouse_id.clone(),
        thick_path: false,
        position: GridPos { x, z: 5 },
        navigation_position: None,
        rotation_quarter_turns: 0,
        line_start: None,
        line_end: None,
        path_cells: Vec::new(),
        inactivity_seconds: 0.0,
    };
    assert!(building_placement_is_available(
        &content,
        &simulation,
        &world,
        &candidate(7),
        definition,
        None,
    ));
    assert!(!building_placement_is_available(
        &content,
        &simulation,
        &world,
        &candidate(6),
        definition,
        None,
    ));
}

#[test]
fn shipping_building_boxes_fit_model_bounds_and_preserve_special_topology() {
    let config = GameConfig::default();
    let content = embedded_content();
    let fine_cell_size = config.world.cell_size / f32::from(NAVIGATION_SUBDIVISIONS);
    for (building_id, definition) in &content.buildings {
        let placement = definition
            .placement_footprint_thirds
            .expect("shipping buildings author an exact placement footprint");
        let navigation = definition
            .navigation_footprint_thirds
            .expect("shipping buildings author an exact navigation footprint");
        match building_id.as_str() {
            "building:path" => {
                assert_eq!(placement, [1, 1]);
                assert_eq!(navigation, [0, 0]);
            }
            "building:wall" | "building:gate" => {
                assert_eq!(placement, navigation);
                assert_eq!(placement, [1, 3]);
            }
            "building:streetlight" => {
                assert_eq!(placement, [1, 1]);
                assert_eq!(navigation, [1, 1]);
            }
            _ => {
                let bounds = content.archetypes[&definition.archetype].bounds;
                assert!(
                    f32::from(navigation[0]) * fine_cell_size + f32::EPSILON >= bounds.size[0],
                    "{} navigation width does not contain model width {}",
                    building_id,
                    bounds.size[0]
                );
                assert!(
                    f32::from(navigation[1]) * fine_cell_size + f32::EPSILON >= bounds.size[2],
                    "{} navigation depth does not contain model depth {}",
                    building_id,
                    bounds.size[2]
                );
                assert!(
                    f32::from(navigation[0].saturating_sub(1)) * fine_cell_size < bounds.size[0],
                    "{building_id} navigation width is not minimal"
                );
                assert!(
                    f32::from(navigation[1].saturating_sub(1)) * fine_cell_size < bounds.size[2],
                    "{building_id} navigation depth is not minimal"
                );
                assert_eq!(placement, navigation.map(|axis| axis.saturating_add(2)));
            }
        }
    }
}

#[test]
fn town_hall_loss_carries_players_and_roles_into_a_real_world_reload() {
    let mut simulation = WorldSimulation::new(42);
    let player = StableId::new("twitch:survivor").unwrap();
    let enemy = StableId::new("actor:enemy_test").unwrap();
    let role = StableId::new("role:forester").unwrap();
    assert!(simulation.join_player(player.clone(), GridPos { x: 2, z: 3 }));
    simulation.assign_role(&player, role.clone()).unwrap();
    assert!(simulation.spawn_enemy(
        enemy,
        StableId::new("archetype:enemy:test").unwrap(),
        GridPos { x: 9, z: 9 },
        10,
    ));
    let mut app = App::new();
    app.insert_resource(SimulationRuntime(simulation))
        .init_resource::<TownRestartRuntime>()
        .add_systems(Update, restart_world_after_town_hall_falls);

    app.update();

    let restart = app.world().resource::<TownRestartRuntime>();
    let retained = restart.retained_players.as_ref().unwrap();
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].id, player);
    assert_eq!(retained[0].role, role);
    assert!(restart.suppress_exit_save);
    assert!(app.world().contains_resource::<WorldLoadingCoverRuntime>());
}

#[test]
fn night_lights_are_prewarmed_reused_and_disabled_during_the_day() {
    let config = GameConfig::default();
    let mut simulation = WorldSimulation::new(config.world.seed);
    simulation.elapsed_seconds = f64::from(config.time.seconds_per_day) * 0.8;
    let player = StableId::new("twitch:light_test").unwrap();
    assert!(simulation.join_player(player.clone(), GridPos { x: 2, z: 2 }));
    let building_id = StableId::new("building:test-torch").unwrap();
    simulation.buildings.insert(
        building_id.clone(),
        BuildingState {
            id: building_id.clone(),
            archetype: StableId::new("archetype:building:torch").unwrap(),
            position: GridPos { x: 3, z: 3 },
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );
    let mut app = App::new();
    app.insert_resource(RuntimeConfig(config.clone()))
        .insert_resource(SimulationRuntime(simulation))
        .add_systems(Update, sync_pooled_night_lights);
    app.world_mut().spawn((
        Agent {
            id: player,
            kind: ActorKind::Player,
            archetype: StableId::new("archetype:player:test").unwrap(),
            goal: AgentGoal::Wander,
            spawn: GridPos { x: 2, z: 2 },
            origin: GridPos { x: 2, z: 2 },
            navigation_position: placement_to_navigation_centre(GridPos { x: 2, z: 2 }),
            path: Vec::new(),
            path_index: 0,
            target: GridPos { x: 2, z: 2 },
            action_cooldown_seconds: 0.0,
            action_started: false,
            repath_remaining_seconds: 0.0,
            health_regen_accumulator: 0.0,
            wander_sequence: 0,
            previous_wander_origin: None,
        },
        GlobalTransform::from_translation(Vec3::new(1.0, 0.0, 1.0)),
    ));
    app.world_mut().spawn((
        RuntimeBuilding {
            id: building_id.clone(),
        },
        GlobalTransform::from_translation(Vec3::new(3.0, 0.0, 3.0)),
    ));
    app.world_mut().spawn((
        CombatProjectile {
            source: ProjectileSource::Building(building_id),
            target: StableId::new("actor:enemy_test").unwrap(),
            damage: 1,
            speed_cells_per_second: 1.0,
            visual: CombatVisualKind::Fireball,
            trail_cooldown_seconds: 0.0,
            remaining_seconds: 1.0,
        },
        GlobalTransform::from_translation(Vec3::new(2.0, 1.0, 2.0)),
    ));

    app.update();

    let slot_count = app
        .world_mut()
        .query::<&NightPointLightPoolSlot>()
        .iter(app.world())
        .count();
    let visible_at_night = app
        .world_mut()
        .query_filtered::<&Visibility, With<NightPointLightPoolSlot>>()
        .iter(app.world())
        .filter(|visibility| **visibility == Visibility::Visible)
        .count();
    assert_eq!(slot_count, NIGHT_LIGHT_POOL_CAPACITY);
    assert_eq!(visible_at_night, 3);
    assert!(
        building_night_light_profile(
            &StableId::new("archetype:building:house").unwrap(),
            config.world.cell_size,
        )
        .is_none()
    );
    assert!(
        building_night_light_profile(
            &StableId::new("archetype:building:streetlight").unwrap(),
            config.world.cell_size,
        )
        .is_some()
    );

    app.world_mut()
        .resource_mut::<SimulationRuntime>()
        .0
        .elapsed_seconds = f64::from(NIGHT_LIGHT_TRANSITION_SECONDS) + 1.0;
    app.update();
    let slot_count_after_day = app
        .world_mut()
        .query::<&NightPointLightPoolSlot>()
        .iter(app.world())
        .count();
    let visible_during_day = app
        .world_mut()
        .query_filtered::<&Visibility, With<NightPointLightPoolSlot>>()
        .iter(app.world())
        .filter(|visibility| **visibility == Visibility::Visible)
        .count();
    assert_eq!(slot_count_after_day, slot_count);
    assert_eq!(visible_during_day, 0);
}

#[test]
fn night_light_budget_keeps_the_nearest_sources() {
    let spec = |distance: f32| NightLightSpec {
        position: Vec3::new(distance, 0.0, 0.0),
        color: Color::WHITE,
        intensity: 1.0,
        range: 1.0,
        transition_delay_seconds: 0.0,
    };
    let mut candidates = vec![(9.0, spec(3.0)), (1.0, spec(1.0)), (4.0, spec(2.0))];
    let mut selected = Vec::new();
    append_nearest_night_lights(&mut selected, &mut candidates, 2);
    assert_eq!(selected.len(), 2);
    assert!((selected[0].position.x - 1.0).abs() < f32::EPSILON);
    assert!((selected[1].position.x - 2.0).abs() < f32::EPSILON);
}

#[test]
fn night_lights_stagger_activation_and_deactivation_over_ten_seconds() {
    let config = GameConfig::default().time;
    let night_start =
        f64::from(config.seconds_per_day) * f64::from(config.daylight_per_thousand) / 1_000.0;
    assert!(!night_light_is_active(&config, night_start + 4.9, 5.0));
    assert!(night_light_is_active(&config, night_start + 5.0, 5.0));
    assert!(night_light_is_active(&config, 4.9, 5.0));
    assert!(!night_light_is_active(&config, 5.0, 5.0));
    assert!(!night_light_sources_are_inactive(
        &config,
        f64::from(NIGHT_LIGHT_TRANSITION_SECONDS) - 0.1,
    ));
    assert!(night_light_sources_are_inactive(
        &config,
        f64::from(NIGHT_LIGHT_TRANSITION_SECONDS),
    ));
    assert!(!night_light_sources_are_inactive(&config, night_start,));
    assert!(night_light_transition_delay(0) >= 0.0);
    assert!(night_light_transition_delay(u64::MAX) <= NIGHT_LIGHT_TRANSITION_SECONDS);
}

#[test]
fn surface_grounding_places_rotated_bounds_on_the_surface() {
    let rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let minimum = transformed_bounds_minimum_y(
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(0.5, 2.0, 0.25),
        rotation,
        Vec3::splat(2.0),
    );
    assert!((minimum + 0.5).abs() < 0.0001);
    let surface = 7.0;
    let translation_y = surface - minimum;
    assert!((translation_y + minimum - surface).abs() < 0.0001);
}

#[test]
fn surface_grounding_uses_real_vertices_instead_of_empty_aabb_corners() {
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            [2.0, 1.0, 0.0],
            [1.0, 2.0, 0.0],
        ],
    );
    let rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_4);
    let exact = transformed_mesh_vertical_extent(&mesh, rotation, Vec3::splat(2.0)).unwrap();
    let conservative = transformed_bounds_minimum_y(
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        rotation,
        Vec3::splat(2.0),
    );
    assert!(exact.0 > conservative + 1.0);
    assert!(exact.1 >= exact.0);
}

#[test]
fn foliage_mesh_habitat_rejects_shore_overhang_and_emergent_underwater_grass() {
    let mut config = GameConfig::default();
    config.world.width = 3;
    config.world.height = 3;
    config.world.cell_size = 2.0;
    config.world.water_level_centimetres = 5;
    let world = GeneratedWorld {
        seed: 1,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(
            3,
            3,
            vec![false; 9],
            vec![100, 100, -100, 100, 100, -100, 100, 100, -100],
        )
        .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // The only base vertex is on dry land, but the upper foliage leans
        // over a submerged cell. The complete rendered footprint must be
        // classified, not just its root/base band.
        vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
    );
    let land = Transform::from_xyz(0.0, 1.0, 0.0);
    assert!(!foliage_mesh_fits_habitat(
        &mesh,
        &land,
        (0.0, 1.0),
        FoliageHabitat::Land,
        &config,
        &world,
    ));
    let compact_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.5, 0.0]],
    );
    assert!(!foliage_mesh_fits_habitat(
        &compact_mesh,
        &land,
        (0.0, 1.0),
        FoliageHabitat::Land,
        &config,
        &world,
    ));
    let underwater = Transform::from_xyz(0.0, -0.5, 0.0);
    assert!(!foliage_mesh_fits_habitat(
        &mesh,
        &underwater,
        (0.0, 1.0),
        FoliageHabitat::Underwater,
        &config,
        &world,
    ));
}

#[test]
fn surface_visual_remains_hidden_until_grounding_is_complete() {
    let mut app = App::new();
    app.add_systems(Update, ground_loaded_surface_visuals);
    let bounds = Aabb::from_min_max(Vec3::new(-0.5, 0.0, -0.25), Vec3::new(0.5, 4.0, 0.25));
    let rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let scale = Vec3::splat(2.0);
    let surface = 7.0;
    let entity = app
        .world_mut()
        .spawn((
            PendingSurfaceGrounding {
                surface_height: surface,
            },
            bounds,
            Transform::from_rotation(rotation).with_scale(scale),
            Visibility::Hidden,
        ))
        .id();

    app.update();

    let grounded = app.world().entity(entity);
    assert!(grounded.get::<PendingSurfaceGrounding>().is_none());
    assert_eq!(grounded.get::<Visibility>(), Some(&Visibility::Inherited));
    let transform = grounded.get::<Transform>().unwrap();
    let minimum = transformed_bounds_minimum_y(
        Vec3::from(bounds.center),
        Vec3::from(bounds.half_extents),
        transform.rotation,
        transform.scale,
    );
    assert!((transform.translation.y + minimum - surface).abs() < 0.0001);
}

#[test]
fn runtime_foliage_waits_for_world_before_habitat_classification() {
    let mut config = GameConfig::default();
    config.world.width = 3;
    config.world.height = 3;
    config.world.cell_size = 2.0;
    config.world.water_level_centimetres = 5;
    let world = GeneratedWorld {
        seed: 1,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(
            3,
            3,
            vec![false; 9],
            vec![100, 100, -100, 100, 100, -100, 100, 100, -100],
        )
        .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
    );
    let bounds = Aabb::from_min_max(Vec3::ZERO, Vec3::new(2.0, 1.0, 0.0));
    let mut app = App::new();
    app.insert_resource(Assets::<Mesh>::default())
        .add_systems(Update, ground_loaded_surface_visuals);
    let mesh = app.world_mut().resource_mut::<Assets<Mesh>>().add(mesh);
    let entity = app
        .world_mut()
        .spawn((
            Mesh3d(mesh),
            RuntimeTerrainGrounding,
            SurfaceFoliageHabitat(FoliageHabitat::Land),
            PendingSurfaceGrounding {
                surface_height: 1.0,
            },
            bounds,
            Transform::from_xyz(0.0, 1.0, 0.0),
            Visibility::Hidden,
        ))
        .id();

    app.update();

    let waiting = app.world().entity(entity);
    assert!(waiting.get::<PendingSurfaceGrounding>().is_some());
    assert_eq!(waiting.get::<Visibility>(), Some(&Visibility::Hidden));

    app.world_mut().insert_resource(RuntimeConfig(config));
    app.world_mut().insert_resource(WorldRuntime {
        generated: world,
        legacy_terrain_mesh: None,
        legacy_migration: None,
    });
    app.update();
    assert!(app.world().get_entity(entity).is_err());
}

#[test]
fn terrain_conforming_grounding_never_leaves_a_base_vertex_in_the_air() {
    let mut config = GameConfig::default();
    config.world.width = 2;
    config.world.height = 2;
    config.world.cell_size = 2.0;
    let world = GeneratedWorld {
        seed: 1,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(2, 2, vec![false; 4], vec![0, 100, 300, 900])
            .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-0.5, 0.0, 0.0], [0.5, 0.0, 0.0], [0.0, 1.0, 0.0]],
    );
    let mut transform = Transform::default();
    let extent =
        transformed_mesh_vertical_extent(&mesh, transform.rotation, transform.scale).unwrap();
    transform.translation.y =
        terrain_conforming_translation_y(&mesh, &transform, extent, &config, &world).unwrap();
    for x in [-0.5, 0.5] {
        let surface = terrain_surface_height_at_world(&world, &config, x, 0.0).unwrap();
        assert!(transform.translation.y <= surface);
    }
}

#[test]
fn foliage_surface_height_matches_the_rendered_terrain_triangles() {
    let mut config = GameConfig::default();
    config.world.width = 2;
    config.world.height = 2;
    config.world.cell_size = 2.0;
    let world = GeneratedWorld {
        seed: 1,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(2, 2, vec![false; 4], vec![0, 100, 300, 900])
            .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };

    for (x, z, expected) in [
        (-1.0, -1.0, 0.0),
        (1.0, -1.0, 0.5),
        (-1.0, 1.0, 1.5),
        (1.0, 1.0, 3.25),
        (-0.5, -0.5, 0.5),
        (0.5, 0.5, 2.125),
    ] {
        let actual = terrain_surface_height_at_world(&world, &config, x, z).unwrap();
        assert!(
            (actual - expected).abs() < 0.0001,
            "surface at ({x}, {z}) was {actual}, expected {expected}"
        );
    }
    assert!(terrain_surface_height_at_world(&world, &config, 1.01, 0.0).is_none());
}

#[test]
fn resource_pivot_offsets_are_resampled_and_bounds_grounded_at_final_xz() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let resource = world
        .resources
        .iter()
        .find(|resource| resource.kind.as_str() == "resource:wood")
        .expect("shipping world has trees");
    let archetype = resource_visual_archetype(&content, &resource.kind).unwrap();
    let generated = generated_resource_world_position(resource, &config, &world);
    let grounded = grounded_resource_visual_position(generated, Some(archetype), &config, &world);
    let expected = terrain_surface_height_at_world(&world, &config, grounded.x, grounded.z)
        .expect("resource pivot remains inside terrain");
    assert!((grounded.y - expected).abs() < 0.0001);
}

#[test]
fn foliage_habitat_is_checked_after_offset_at_the_visible_surface() {
    let mut config = GameConfig::default();
    config.world.width = 3;
    config.world.height = 3;
    config.world.cell_size = 2.0;
    config.world.water_level_centimetres = 5;
    let foliage = |habitat| GeneratedFoliage {
        id: StableId::new(match habitat {
            FoliageHabitat::Land => "foliage:test:land",
            FoliageHabitat::Underwater => "foliage:test:underwater",
        })
        .unwrap(),
        layer: StableId::new("foliage:test").unwrap(),
        habitat,
        position: GridPos { x: 1, z: 1 },
        offset_milli_cells: [0, 0],
        variant: 0,
        yaw_milliradians: 0,
        scale_milli: 1_000,
    };
    let flat_world = |height_centimetres| GeneratedWorld {
        seed: 7,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(
            3,
            3,
            vec![false; 9],
            vec![height_centimetres; 9],
        )
        .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };

    let dry_world = flat_world(100);
    let land =
        resolved_foliage_ground_position(&dry_world, &config, &foliage(FoliageHabitat::Land))
            .unwrap();
    assert!((land.y - 1.0).abs() < 0.0001);
    assert!(
            resolved_foliage_ground_position(
                &dry_world,
                &config,
                &foliage(FoliageHabitat::Underwater),
            )
            .is_none()
        );

    let submerged_world = flat_world(-100);
    let underwater = resolved_foliage_ground_position(
        &submerged_world,
        &config,
        &foliage(FoliageHabitat::Underwater),
    )
    .unwrap();
    assert!((underwater.y + 1.0).abs() < 0.0001);
    assert!(
            resolved_foliage_ground_position(
                &submerged_world,
                &config,
                &foliage(FoliageHabitat::Land),
            )
            .is_none()
        );
}

#[test]
fn player_night_lights_normalize_colour_luminance_then_apply_total_levels() {
    for color in [[255, 0, 0], [0, 255, 0], [0, 0, 255], [255, 209, 143]] {
        let linear = perceptually_normalized_light_color(color)
            .to_linear()
            .to_f32_array();
        let luminance = linear[0] * 0.2126 + linear[1] * 0.7152 + linear[2] * 0.0722;
        assert!((luminance - 1.0).abs() < 0.0001, "{color:?}");
    }

    let mut simulation = WorldSimulation::new(7);
    let actor_id = StableId::new("twitch:level_light").unwrap();
    assert!(simulation.join_player(actor_id.clone(), GridPos { x: 1, z: 1 }));
    let actor = simulation.actors.get_mut(&actor_id).unwrap();
    actor.role_progression.clear();
    assert!((player_night_light_level_multiplier(actor) - 1.0).abs() < f32::EPSILON);
    actor.role_progression.insert(
        StableId::new("role:logger").unwrap(),
        stream_town_domain::RoleProgress {
            level: 25,
            experience: 0,
        },
    );
    actor.role_progression.insert(
        StableId::new("role:miner").unwrap(),
        stream_town_domain::RoleProgress {
            level: 25,
            experience: 0,
        },
    );
    assert!((player_night_light_level_multiplier(actor) - 2.0).abs() < f32::EPSILON);
    actor.role_progression.values_mut().for_each(|progress| {
        progress.level = u16::MAX;
    });
    assert!(player_night_light_level_multiplier(actor) < 3.0);
}

#[test]
fn shipping_main_menu_preserves_art_order_and_load_availability() {
    let presentation = embedded_presentation();
    let content = embedded_content();
    let menu_scene = embedded_main_menu_scene();
    for source_path in MAIN_MENU_TEXTURE_PATHS {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path),
            "missing Main Menu texture {source_path}"
        );
    }
    let actions = [
        MainMenuAction::NewGame,
        MainMenuAction::LoadGame,
        MainMenuAction::Settings,
        MainMenuAction::Secrets,
        MainMenuAction::Credits,
        MainMenuAction::Quit,
    ];
    assert_eq!(
        actions.map(main_menu_action_label),
        [
            "New Game",
            "Load Game",
            "Settings",
            "Secrets",
            "Credits",
            "Quit"
        ]
    );
    assert!(!main_menu_action_enabled(MainMenuAction::LoadGame, false));
    assert!(main_menu_action_enabled(MainMenuAction::LoadGame, true));
    assert!(main_menu_action_enabled(MainMenuAction::NewGame, false));
    assert_eq!(menu_scene.schema_version, 3);
    let corrective_bake = menu_scene.corrective_bake.as_ref().unwrap();
    assert_eq!(corrective_bake.version, 3);
    assert!((corrective_bake.terrain_height_multiplier - 3.0).abs() < f32::EPSILON);
    assert_eq!(menu_scene.source_scene, MAIN_MENU_SCENE_PATH);
    assert!(
        menu_scene
            .camera
            .position
            .into_iter()
            .zip([134.97, 8.5, 133.799_99])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert!(
        menu_scene
            .camera
            .rotation
            .into_iter()
            .zip([0.117_483_57, -0.617_187, -0.095_625_274, -0.772_096_93])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert_eq!(menu_scene.instances.len(), 285);
    assert_eq!(menu_scene.embedded_meshes.len(), 1);
    assert_eq!(menu_scene.embedded_meshes[0].vertices.len(), 4_900);
    assert_eq!(menu_scene.embedded_meshes[0].triangles.len() / 3, 9_522);
    assert!(
        menu_scene
            .instances
            .iter()
            .map(|instance| instance.position[1].to_bits())
            .collect::<BTreeSet<_>>()
            .len()
            >= 4,
        "the corrective bake must retain visible generated height terraces"
    );
    let (minimum_height, maximum_height) = menu_scene.instances.iter().fold(
        (f32::INFINITY, f32::NEG_INFINITY),
        |(minimum, maximum), instance| {
            (
                minimum.min(instance.position[1]),
                maximum.max(instance.position[1]),
            )
        },
    );
    assert!(
        maximum_height - minimum_height >= 4.5,
        "the side-on menu bake must visibly amplify generated relief"
    );
    let fountain = menu_scene
        .instances
        .iter()
        .find(|instance| instance.hierarchy_path.contains("Building_Fountain"))
        .unwrap();
    let camera_transform =
        Transform::from_translation(Vec3::from_array(menu_scene.camera.position))
            .with_rotation(authored_scene_rotation(menu_scene.camera.rotation));
    let clip = Mat4::perspective_rh(
        menu_scene.camera.field_of_view_degrees.to_radians(),
        16.0 / 9.0,
        menu_scene.camera.near,
        menu_scene.camera.far,
    ) * camera_transform.to_matrix().inverse()
        * Vec3::from_array(fountain.position).extend(1.0);
    assert!(clip.w > 0.0);
    assert!(
        clip.x / clip.w > 0.0,
        "the authored town must compose on the right side of the menu"
    );
    let asset_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    for instance in &menu_scene.instances {
        let (_, _, scene) = menu_scene_for_source_model(&content, &instance.source_path)
            .unwrap_or_else(|| panic!("unmapped menu model {}", instance.source_path));
        assert!(
            asset_root.join(&scene.asset_path).is_file(),
            "missing converted menu model {}",
            scene.asset_path
        );
    }
}

#[test]
fn secrets_flow_is_sensitive_and_requires_an_explicit_answer() {
    assert!(menu_page_is_sensitive(MenuPage::SecretsDisclaimer));
    assert!(menu_page_is_sensitive(MenuPage::Secrets));
    assert!(!menu_page_is_sensitive(MenuPage::Closed));
    assert!(SECRETS_DISCLAIMER.contains("not presently streaming"));
    assert!(SECRETS_DISCLAIMER.contains("any and all responsibility"));
    assert!(SECRETS_DISCLAIMER.contains("liability"));
    assert!(!SECRETS_DISCLAIMER.contains("Human Bean"));
    assert_eq!(
        secrets_action_label(SecretsAction::DisclaimerNo, &GameConfig::default()),
        "No — go back"
    );
    assert_eq!(
        secrets_action_label(SecretsAction::DisclaimerYes, &GameConfig::default()),
        "Yes — continue"
    );
}

#[test]
fn secrets_save_restarts_only_connections_affected_by_visible_fields() {
    let original = GameConfig::default().twitch;
    assert_eq!(
        secrets_restart_requirements(&original, &original),
        (false, false)
    );

    let mut bot_changed = original.clone();
    bot_changed.bot_login = "another_bot".to_owned();
    assert_eq!(
        secrets_restart_requirements(&original, &bot_changed),
        (true, false)
    );

    let mut channel_changed = original.clone();
    channel_changed.channel_login = "another_channel".to_owned();
    assert_eq!(
        secrets_restart_requirements(&original, &channel_changed),
        (true, true)
    );

    let mut client_changed = original.clone();
    client_changed.client_id = "another-client".to_owned();
    assert_eq!(
        secrets_restart_requirements(&original, &client_changed),
        (true, true)
    );
}

#[test]
fn secrets_bot_status_reports_automatic_command_connection() {
    let mut config = GameConfig::default();
    config.twitch.enabled = false;
    let connection = TwitchConnection::default();
    let (status, tone) =
        bot_connection_status(&config, &connection, &SecretsCredentialState::Stored);
    assert!(status.contains("authorized"));
    assert!(status.contains("disabled"));
    assert_eq!(tone, SecretsStatusTone::Inactive);

    config.twitch.enabled = true;
    let connection = TwitchConnection {
        status: TwitchStatus::Connected,
        ..default()
    };
    let (status, tone) =
        bot_connection_status(&config, &connection, &SecretsCredentialState::Stored);
    assert!(status.contains("Connected automatically"));
    assert!(status.contains("commands are enabled"));
    assert_eq!(tone, SecretsStatusTone::Good);
}

#[test]
fn town_start_requires_both_grants_and_a_connected_bot() {
    let mut config = GameConfig::default();
    config.twitch.enabled = true;
    config.twitch.broadcast.enabled = true;
    config.twitch.client_id = "public-client-id".to_owned();
    config.twitch.bot_login = "humanbeanbot".to_owned();
    config.twitch.channel_login = "humanbeangames".to_owned();
    let mut secrets = SecretsRuntime {
        bot_credential: SecretsCredentialState::Stored,
        broadcaster_credential: SecretsCredentialState::Stored,
        ..default()
    };
    let mut connection = TwitchConnection {
        status: TwitchStatus::Connected,
        moderation_status: TwitchModerationStatus::Ready,
        ..default()
    };
    assert!(twitch_accounts_connected(&config, &secrets, &connection));

    secrets.broadcaster_credential = SecretsCredentialState::Missing;
    assert!(!twitch_accounts_connected(&config, &secrets, &connection));
    secrets.broadcaster_credential = SecretsCredentialState::Stored;
    connection.status = TwitchStatus::Connecting;
    assert!(!twitch_accounts_connected(&config, &secrets, &connection));
    connection.status = TwitchStatus::Connected;
    connection.moderation_status =
        TwitchModerationStatus::Error("missing moderator:manage:banned_users".to_owned());
    assert!(!twitch_accounts_connected(&config, &secrets, &connection));

    let mut menu = MenuRuntime::default();
    open_twitch_setup_required(&mut menu);
    assert_eq!(menu.page, MenuPage::SecretsDisclaimer);
    assert!(menu.feedback.contains("Connect both Twitch accounts"));
}

#[cfg(target_os = "windows")]
#[test]
fn secrets_broadcast_status_distinguishes_test_from_live_output() {
    use direct_broadcast::{DirectBroadcastPhase, DirectBroadcastSnapshot};

    let mut config = GameConfig::default();
    config.twitch.broadcast.enabled = true;
    config.twitch.broadcast.bandwidth_test = true;
    let mut snapshot = DirectBroadcastSnapshot {
        phase: DirectBroadcastPhase::BandwidthTesting,
        encoder: Some("h264_mf".to_owned()),
        encoder_rejections: Vec::new(),
        ingest: Some("US East".to_owned()),
        captured_video_frames: 12,
        encoded_video_frames: 10,
        dropped_video_frames: 2,
        encoded_audio_frames: 20,
        dropped_audio_frames: 1,
        replaced_video_frames: 1,
        skipped_video_frames: 0,
        audio_queue_depth: 0,
        audio_queue_high_water: 2,
        captured_video_fps: 29.5,
        encoded_video_fps: 30.0,
        average_capture_ms: 8.0,
        maximum_capture_ms: 12.0,
        average_encode_ms: 2.0,
        maximum_encode_ms: 4.0,
        average_mux_write_ms: 0.5,
        maximum_mux_write_ms: 1.0,
    };
    let (status, tone) = broadcast_connection_status(
        &config,
        &SecretsCredentialState::Stored,
        &TwitchModerationStatus::Ready,
        &snapshot,
    );
    assert!(status.contains("BANDWIDTH TEST — not publicly live"));
    assert!(status.contains("29.5 captured / 30.0 output FPS"));
    assert!(status.contains("2 video / 1 audio drops"));
    assert_eq!(tone, SecretsStatusTone::Good);

    config.twitch.broadcast.bandwidth_test = false;
    snapshot.phase = DirectBroadcastPhase::Broadcasting;
    let (status, tone) = broadcast_connection_status(
        &config,
        &SecretsCredentialState::Stored,
        &TwitchModerationStatus::Ready,
        &snapshot,
    );
    assert!(status.contains("● LIVE"));
    assert_eq!(tone, SecretsStatusTone::Good);
}

#[cfg(target_os = "windows")]
#[test]
fn broadcaster_control_failure_is_never_reported_as_a_bot_error() {
    use direct_broadcast::DirectBroadcastRuntime;

    let mut config = GameConfig::default();
    config.twitch.enabled = true;
    config.twitch.broadcast.enabled = true;
    config.twitch.bot_login = "humanbeanbot".to_owned();
    config.twitch.channel_login = "humanbeangames".to_owned();
    let connection = TwitchConnection {
        status: TwitchStatus::Connected,
        moderation_status: TwitchModerationStatus::Error(
            "stored Twitch token is missing scope moderator:manage:banned_users".to_owned(),
        ),
        ..default()
    };

    let (bot_status, bot_tone) =
        bot_connection_status(&config, &connection, &SecretsCredentialState::Stored);
    assert!(bot_status.contains("Connected automatically"));
    assert!(!bot_status.contains("moderation"));
    assert_eq!(bot_tone, SecretsStatusTone::Good);

    let (broadcaster_status, broadcaster_tone) = broadcast_connection_status(
        &config,
        &SecretsCredentialState::Stored,
        &connection.moderation_status,
        &DirectBroadcastRuntime::default().snapshot(),
    );
    assert!(broadcaster_status.contains("Broadcaster authorization error"));
    assert!(broadcaster_status.contains("Authorize stream account"));
    assert!(!broadcaster_status.contains("Bot connection error"));
    assert_eq!(broadcaster_tone, SecretsStatusTone::Error);
}

#[test]
fn twitch_verification_opens_only_the_https_url_returned_by_twitch() {
    let mut opened = String::new();
    open_twitch_verification_uri_with("https://www.twitch.tv/activate", |uri| {
        uri.clone_into(&mut opened);
        Ok(())
    })
    .expect("HTTPS Twitch verification URL should open");
    assert_eq!(opened, "https://www.twitch.tv/activate");

    let mut insecure_opened = false;
    let error = open_twitch_verification_uri_with("http://www.twitch.tv/activate", |_| {
        insecure_opened = true;
        Ok(())
    })
    .expect_err("non-HTTPS verification URL should be rejected");
    assert!(!insecure_opened);
    assert!(error.to_string().contains("non-HTTPS"));

    let error = open_twitch_verification_uri_with("https://example.com/activate", |_| Ok(()))
        .expect_err("non-Twitch verification URL should be rejected");
    assert!(error.to_string().contains("unexpected host"));
}

#[test]
fn accessibility_navigation_preserves_editable_text_focus() {
    assert!(!accessibility_should_clear_focus(false, true));
    assert!(!accessibility_should_clear_focus(true, false));
    assert!(accessibility_should_clear_focus(false, false));

    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(GameConfig::default()))
    .add_plugins(StreamTownGamePlugin);
    let field = app
        .world_mut()
        .spawn(EditableText::new("public-client-id"))
        .id();
    let tabbable_button = app.world_mut().spawn(TabIndex(1)).id();
    app.world_mut()
        .resource_mut::<InputFocus>()
        .set(field, FocusCause::Pressed);

    app.update();

    assert_eq!(app.world().resource::<InputFocus>().get(), Some(field));
    app.world_mut()
        .resource_mut::<InputFocus>()
        .set(tabbable_button, FocusCause::Navigated);
    app.update();
    assert_eq!(
        app.world().resource::<InputFocus>().get(),
        Some(tabbable_button)
    );
}

#[test]
fn scene_exposure_uses_approved_neutral_baselines_and_prism_clouds() {
    let settings = PlayerSettings::default();
    let menu = color_grading_for_state(&settings, &[], GameState::MainMenu);
    let world = color_grading_for_state(&settings, &[], GameState::InGame);
    assert!((menu.global.exposure - MAIN_MENU_BASELINE_EXPOSURE_EV).abs() < f32::EPSILON);
    assert!((world.global.exposure - IN_GAME_BASELINE_EXPOSURE_EV).abs() < f32::EPSILON);
    let prism_count = MAIN_MENU_CLOUD_COLUMNS * MAIN_MENU_CLOUD_ROWS;
    let prisms = (0..prism_count)
        .map(main_menu_cloud_prism_transform)
        .collect::<Vec<_>>();
    assert!(prisms.iter().all(|cloud| cloud.scale.min_element() > 2.0));
    assert!(
        prisms
            .iter()
            .map(|cloud| cloud.translation.to_array().map(f32::to_bits))
            .collect::<BTreeSet<_>>()
            .len()
            == prism_count,
        "menu cloud prisms must not overlap exactly and z-fight"
    );
    let (minimum_x, maximum_x) = prisms.iter().fold(
        (f32::INFINITY, f32::NEG_INFINITY),
        |(minimum, maximum), cloud| {
            (
                minimum.min(cloud.translation.x),
                maximum.max(cloud.translation.x),
            )
        },
    );
    assert!(maximum_x - minimum_x > 300.0);

    let sky = include_str!("../../../assets/shaders/menu_sky_material.wgsl");
    assert!(sky.contains("horizon_color"));
    assert!(sky.contains("zenith_color"));
    assert!(sky.contains("sun_disc"));
    assert!(sky.contains("high_wisps"));
    let water = include_str!("../../../assets/shaders/water_material.wgsl");
    assert!(water.contains("menu_wave"));
    assert!(water.contains("ripple * 0.28"));
    assert!(water.contains("broad_ripple"));
}

#[test]
fn main_menu_schedules_every_visible_baked_decoration() {
    let reference = embedded_main_menu_scene();
    let (resource_indices, foliage_indices) = main_menu_baked_decoration_indices(reference);
    let bake = reference.corrective_bake.as_ref().unwrap();
    let visible_resource_count = bake
        .resources
        .iter()
        .filter(|resource| {
            menu_baked_position_visible(reference, Vec3::from_array(resource.position))
        })
        .count();
    let visible_foliage_count = bake
        .foliage
        .iter()
        .filter(|foliage| {
            menu_baked_position_visible(reference, Vec3::from_array(foliage.position))
        })
        .count();
    assert_eq!(resource_indices.len(), visible_resource_count);
    assert_eq!(foliage_indices.len(), visible_foliage_count);
    let visible_layers = foliage_indices
        .iter()
        .map(|index| bake.foliage[*index].layer.as_str())
        .collect::<BTreeSet<_>>();
    assert!(visible_layers.contains("foliage:land:0"));
    assert!(visible_layers.contains("foliage:land:1"));
    let model_count = reference
        .instances
        .iter()
        .filter(|instance| {
            menu_baked_position_visible(reference, Vec3::from_array(instance.position))
        })
        .count();
    let scheduled = model_count + resource_indices.len() + foliage_indices.len();
    let construction_updates = scheduled.div_ceil(MAIN_MENU_SPAWN_BUDGET_PER_FRAME);
    assert!(construction_updates > 1);

    let mut spinner = UiTransform::IDENTITY;
    for _ in 0..construction_updates {
        let before = spinner.rotation;
        apply_loading_icon_rotation(&mut spinner, 500.0_f32.to_radians(), 1.0 / 60.0);
        assert_ne!(spinner.rotation, before);
    }
}

#[test]
fn main_menu_shoreline_is_clipped_to_water_without_diagonal_holes() {
    let mesh = stream_town_domain::MainMenuEmbeddedMesh {
        hierarchy_path: "Terrain".to_owned(),
        vertices: vec![
            [-1.0, 1.0, 0.0],
            [1.0, -2.0, -1.0],
            [1.0, -2.0, 1.0],
            [-1.0, 1.0, 2.0],
        ],
        normals: vec![[0.0, 1.0, 0.0]; 4],
        uv: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
        triangles: vec![0, 1, 2, 0, 2, 3],
    };

    let (positions, normals, uv) = clipped_main_menu_geometry(&mesh, 0.0);

    assert_eq!(positions.len(), 9);
    assert_eq!(normals.len(), positions.len());
    assert_eq!(uv.len(), positions.len());
    assert!(positions.iter().all(|position| position[1] >= 0.0));
    assert!(
        positions
            .iter()
            .filter(|position| position[1].abs() < f32::EPSILON)
            .count()
            >= 4
    );
}

#[test]
fn menu_buildings_hide_construction_stages_but_keep_completed_models() {
    let content = embedded_content();
    let archetype = content
        .archetypes
        .get(&StableId::new("archetype:building:farm").unwrap())
        .unwrap();
    let scene = archetype.scenes.first().unwrap();
    let hidden = main_menu_hidden_model_node_names(
        &content,
        &StableId::new("archetype:building:farm").unwrap(),
        scene,
    );
    assert!(hidden.contains("Age02_Farm_Stage01"));
    assert!(hidden.contains("Age02_Farm_Seeds"));
    assert!(hidden.contains("Age02_Farm_Half"));
    assert!(!hidden.contains("Age02_Farm_Base"));
    assert!(!hidden.contains("Age02_Farm_Full"));

    let mut app = App::new();
    app.add_systems(Update, hide_main_menu_inactive_model_nodes);
    let root = app
        .world_mut()
        .spawn((MainMenuHiddenModelNodes(hidden), Name::new("Farm root")))
        .id();
    let stage = app
        .world_mut()
        .spawn((Name::new("Age02_Farm_Stage01"), Visibility::Inherited))
        .id();
    app.world_mut().entity_mut(root).add_child(stage);

    app.update();

    assert_eq!(
        *app.world().get::<Visibility>(stage).unwrap(),
        Visibility::Hidden
    );
    assert!(
        app.world()
            .get::<MainMenuModelNodeProcessed>(stage)
            .is_some()
    );
}

#[test]
fn menu_building_renderers_explicitly_keep_shadow_casting() {
    let mut app = App::new();
    app.add_systems(Update, enforce_main_menu_building_shadow_casters);
    let root = app.world_mut().spawn(MainMenuBuildingShadowRoot).id();
    let renderer = app
        .world_mut()
        .spawn((Mesh3d(Handle::default()), bevy::light::NotShadowCaster))
        .id();
    app.world_mut().entity_mut(root).add_child(renderer);

    app.update();

    assert!(
        app.world()
            .get::<bevy::light::NotShadowCaster>(renderer)
            .is_none()
    );
    assert!(
        app.world()
            .get::<MainMenuBuildingShadowVerified>(renderer)
            .is_some()
    );
}

#[test]
fn synchronized_tree_shadows_receive_light_while_older_card_materials_do_not() {
    assert!(!material_needs_self_shadow_suppression(
        &ResolvedMaterialHandle::Tree(Handle::default())
    ));
    assert!(material_needs_self_shadow_suppression(
        &ResolvedMaterialHandle::Grass(Handle::default())
    ));
    assert!(material_needs_self_shadow_suppression(
        &ResolvedMaterialHandle::Critter(Handle::default())
    ));
    assert!(!material_needs_self_shadow_suppression(
        &ResolvedMaterialHandle::Standard(Handle::default())
    ));
    let content = embedded_content();
    let gate =
        &content.archetypes[&content.buildings[&StableId::new("building:gate").unwrap()].archetype];
    let house = &content.archetypes
        [&content.buildings[&StableId::new("building:house").unwrap()].archetype];
    assert!(archetype_needs_self_shadow_suppression(gate));
    assert!(!archetype_needs_self_shadow_suppression(house));
}

#[test]
fn gate_shadow_suppression_reaches_every_material_overridden_renderer() {
    let mut app = App::new();
    app.add_systems(Update, apply_material_overrides);
    let root = app
        .world_mut()
        .spawn(MaterialOverrideSpec {
            fallback: Some(ResolvedMaterialHandle::Standard(Handle::default())),
            model_materials: BTreeMap::new(),
            renderer_materials: Vec::new(),
            suppress_self_shadows: true,
        })
        .id();
    let renderer = app
        .world_mut()
        .spawn(MeshMaterial3d::<StandardMaterial>(Handle::default()))
        .id();
    app.world_mut().entity_mut(root).add_child(renderer);

    app.update();

    assert!(
        app.world()
            .get::<bevy::light::NotShadowCaster>(renderer)
            .is_some()
    );
    assert!(
        app.world()
            .get::<bevy::light::NotShadowReceiver>(renderer)
            .is_some()
    );
}

#[test]
fn chimney_smoke_keeps_the_standard_material_required_by_its_expiry_system() {
    let mut app = App::new();
    app.add_systems(Update, apply_material_overrides);
    let root = app
        .world_mut()
        .spawn(MaterialOverrideSpec {
            fallback: Some(ResolvedMaterialHandle::Building(Handle::default())),
            model_materials: BTreeMap::new(),
            renderer_materials: Vec::new(),
            suppress_self_shadows: false,
        })
        .id();
    let particle = app
        .world_mut()
        .spawn((
            ChimneySmokeParticle {
                effect: StableId::new("effect:test:chimney").unwrap(),
                elapsed_seconds: 0.0,
                duration_seconds: 1.0,
                origin: Vec3::ZERO,
                velocity: Vec3::Y,
                base_scale: Vec3::ONE,
                size_over_lifetime: [1.0, 1.0],
                color_variant: 0,
            },
            Mesh3d(Handle::default()),
            MeshMaterial3d::<StandardMaterial>(Handle::default()),
        ))
        .id();
    app.world_mut().entity_mut(root).add_child(particle);

    app.update();

    assert!(
        app.world()
            .get::<MeshMaterial3d<StandardMaterial>>(particle)
            .is_some()
    );
    assert!(
        app.world()
            .get::<MeshMaterial3d<BuildingMaterial>>(particle)
            .is_none()
    );
    assert!(
        app.world()
            .get::<MaterialOverrideApplied>(particle)
            .is_none()
    );
}

#[test]
fn main_menu_mouse_actions_use_the_shipping_state_and_settings_paths() {
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("menu-actions.stbevy");
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
        .init_state::<GameState>()
        .init_resource::<MenuRuntime>()
        .init_resource::<MenuIoRequest>()
        .init_resource::<SecretsRuntime>()
        .init_resource::<TwitchConnection>()
        .insert_resource(RuntimeConfig(GameConfig::default()))
        .insert_resource(RuntimePlayerSettings(PlayerSettings::default()))
        .insert_resource(SaveRuntime {
            store: NativeSaveStore::new(&save_path),
        })
        .insert_resource(TownSaveCatalogRuntime {
            directory: save_directory.path().to_path_buf(),
            legacy_path: save_path.clone(),
            fixed_path: Some(save_path),
            active_town: None,
        })
        .add_systems(Update, main_menu_buttons);
    app.world_mut()
        .spawn((Interaction::Pressed, MainMenuAction::Settings));

    app.update();

    assert_eq!(
        app.world().resource::<MenuRuntime>().page,
        MenuPage::Settings
    );
    assert_eq!(
        app.world().resource::<MenuRuntime>().return_page,
        MenuPage::Closed
    );
}

#[cfg(target_os = "windows")]
#[test]
fn town_start_consent_prepares_stream_and_loading_only_after_yes() {
    let mut app = App::new();
    let mut menu = MenuRuntime::default();
    let source = PathBuf::from("protected.jumpstart.stbevy");
    request_go_live_confirmation(
        &mut menu,
        PendingTownStart::LoadGame {
            source: source.clone(),
        },
    );
    app.insert_resource(menu)
        .init_resource::<MenuIoRequest>()
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<direct_broadcast::DirectBroadcastControl>()
        .add_systems(Update, go_live_confirmation_buttons);

    app.world_mut()
        .spawn((Interaction::Pressed, GoLiveConfirmationAction::Yes));
    app.update();

    let menu = app.world().resource::<MenuRuntime>();
    assert_eq!(menu.page, MenuPage::Closed);
    assert_eq!(menu.pending_town_start, None);
    assert!(app.world().resource::<MenuIoRequest>().load);
    assert_eq!(
        app.world().resource::<MenuIoRequest>().load_source,
        Some(source)
    );
    assert!(app.world().contains_resource::<WorldLoadingCoverRuntime>());
    assert!(
        app.world()
            .resource::<direct_broadcast::DirectBroadcastControl>()
            .restart_requested_for_test()
    );
}

#[test]
fn main_menu_clouds_fade_from_the_wrap_edge_instead_of_popping() {
    let mut cloud = MainMenuCloudPrism {
        drift_per_second: Vec3::X,
        wrap_min_x: -100.0,
        wrap_max_x: 100.0,
        fade_delay_seconds: 0.0,
        fade_elapsed_seconds: 0.0,
        entrance_fade_distance: 40.0,
        target_alpha: 0.82,
    };
    assert!(main_menu_cloud_fade(&cloud, cloud.wrap_min_x).abs() <= f32::EPSILON);

    cloud.fade_elapsed_seconds = 1.8;
    assert!(main_menu_cloud_fade(&cloud, cloud.wrap_min_x).abs() <= f32::EPSILON);
    let halfway = main_menu_cloud_fade(&cloud, cloud.wrap_min_x + 20.0);
    assert!(halfway > 0.0 && halfway < 1.0);
    assert!(
        (main_menu_cloud_fade(&cloud, cloud.wrap_min_x + cloud.entrance_fade_distance) - 1.0).abs()
            <= f32::EPSILON
    );
}

#[test]
fn screen_reader_click_uses_the_same_shipping_menu_action() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(GameConfig::default()))
    .add_plugins(StreamTownGamePlugin);
    app.update();
    app.update();
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::MainMenu
    );

    let settings_button = app
        .world_mut()
        .query::<(Entity, &MainMenuAction)>()
        .iter(app.world())
        .find_map(|(entity, action)| (*action == MainMenuAction::Settings).then_some(entity))
        .expect("shipping settings button");
    app.world_mut().entity_mut(settings_button).insert((
        AccessibilityNode(AccessibleNode::new(Role::Button)),
        Visibility::Visible,
        InheritedVisibility::VISIBLE,
    ));
    app.update();
    let accessible = app
        .world()
        .get::<AccessibilityNode>(settings_button)
        .expect("settings button semantics");
    assert_eq!(accessible.label(), Some("Settings"));
    assert!(accessible.supports_action(AccessAction::Click));

    app.world_mut()
        .resource_mut::<Messages<AccessibilityActionRequest>>()
        .write(AccessibilityActionRequest(accesskit::ActionRequest {
            action: AccessAction::Click,
            target_tree: accesskit::TreeId::ROOT,
            target_node: accesskit::NodeId(settings_button.to_bits()),
            data: None,
        }));
    app.update();

    assert_eq!(
        app.world().resource::<MenuRuntime>().page,
        MenuPage::Settings
    );
}

#[test]
fn settings_value_edits_do_not_rebuild_the_other_readouts() {
    let twitch = TwitchConnection::default();
    let mut menu = MenuRuntime {
        page: MenuPage::Settings,
        settings_tab: SettingsTab::Audio,
        selected: 12,
        ..default()
    };
    let signature = settings_rows_signature(&menu, &twitch, false);
    menu.draft.audio.music = 0.25;
    menu.selected = 13;
    assert_eq!(settings_rows_signature(&menu, &twitch, false), signature);

    menu.settings_tab = SettingsTab::Gameplay;
    assert_ne!(settings_rows_signature(&menu, &twitch, false), signature);
    assert_ne!(settings_rows_signature(&menu, &twitch, true), signature);
}

#[test]
fn settings_menu_edits_a_complete_valid_draft() {
    let original = PlayerSettings::default();
    let mut draft = original.clone();
    let original_streaming = BroadcastConfig::default();
    let mut streaming = original_streaming.clone();

    for (index, direction) in [
        (0, 1),
        (1, 1),
        (5, -1),
        (8, 1),
        (11, -1),
        (15, 1),
        (16, 1),
        (17, 1),
        (19, 1),
        (20, 1),
        (21, 1),
        (22, 1),
        (23, 1),
        (24, 1),
        (26, 1),
        (27, 1),
        (28, 1),
        (29, 1),
        (30, 1),
        (31, 1),
    ] {
        adjust_settings_menu(&mut draft, &mut streaming, index, direction);
    }

    assert_ne!(draft, original);
    assert_eq!(draft.video.display_mode, DisplayMode::Windowed);
    assert_eq!((draft.video.width, draft.video.height), (2_560, 1_440));
    assert_eq!(draft.video.shadow_map_resolution, 2_048);
    assert_eq!(draft.video.post_process_aa, PostProcessAntiAliasing::None);
    assert!((draft.audio.master - 0.95).abs() < f32::EPSILON);
    assert!((draft.camera.zoom_sensitivity - 11.0).abs() < f32::EPSILON);
    assert_eq!(draft.camera.field_of_view_degrees, 65);
    assert_eq!(draft.interface.display_names, NameDisplayMode::None);
    assert_eq!(draft.autosave_minutes, 5);
    assert_eq!(draft.timelapse.interval, TimelapseInterval::ThreeHours);
    assert!(!draft.timelapse.dynamic);
    assert_eq!(draft.interface.ui_scale_percent, 110);
    assert!(draft.interface.high_contrast);
    assert!(draft.interface.reduced_motion);
    draft.validate().unwrap();
    assert_ne!(streaming, original_streaming);
    assert_eq!((streaming.width, streaming.height), (1_920, 1_080));
    assert_eq!(streaming.frames_per_second, 60);
    assert_eq!(streaming.video_bitrate_kbps, 500);
    assert_eq!(streaming.audio_bitrate_kbps, 64);
    assert_eq!(streaming.encoder, BroadcastEncoderPreference::Nvidia);
    assert!(streaming.bandwidth_test);
    let mut config = GameConfig::default();
    config.twitch.broadcast = streaming.clone();
    config.validate().unwrap();

    let text = settings_menu_text(
        &draft,
        &streaming,
        SETTINGS_MENU_ITEM_COUNT - 1,
        "draft feedback",
    );
    assert!(text.contains("> Cancel changes"));
    assert!(text.contains("Resolution: 2560 x 1440"));
    assert!(text.contains("Stream output: 1920 x 1080"));
    assert!(text.contains("draft feedback"));
}

#[test]
fn timelapse_defaults_and_dynamic_build_acceleration_are_stable() {
    let settings = PlayerSettings::default();
    assert_eq!(settings.timelapse.interval, TimelapseInterval::OneHour);
    assert!(settings.timelapse.dynamic);

    let mut runtime = CityTimelapseRuntime::default();
    runtime.sync_settings(settings.timelapse.interval, settings.timelapse.dynamic);
    assert_eq!(runtime.remaining_seconds, Some(3_600.0));
    runtime.activate_town("Tonyville");
    assert_eq!(runtime.remaining_seconds, Some(0.0));
    runtime.remaining_seconds = Some(3_600.0);
    runtime.activate_town("Tonyville");
    assert_eq!(runtime.remaining_seconds, Some(3_600.0));
    runtime.confirmed_build(settings.timelapse.interval, true);
    assert_eq!(runtime.remaining_seconds, Some(3_000.0));
    runtime.confirmed_build(settings.timelapse.interval, false);
    assert_eq!(runtime.remaining_seconds, Some(3_000.0));
    runtime.sync_settings(TimelapseInterval::TenMinutes, true);
    assert_eq!(runtime.remaining_seconds, Some(600.0));
    runtime.sync_settings(TimelapseInterval::Off, true);
    assert_eq!(runtime.remaining_seconds, None);
    runtime.activate_town("Another Town");
    assert_eq!(runtime.remaining_seconds, None);
}

#[test]
fn timelapse_frame_names_advance_and_video_labels_are_burned_in() {
    let directory = tempfile::tempdir().unwrap();
    assert_eq!(next_timelapse_frame_index(directory.path()), 1);
    std::fs::write(directory.path().join("frame-00000001.png"), b"frame").unwrap();
    std::fs::write(directory.path().join("unrelated.png"), b"frame").unwrap();
    assert_eq!(next_timelapse_frame_index(directory.path()), 2);

    let mut frame = image::RgbImage::new(320, 180);
    let unchanged_top = *frame.get_pixel(0, 0);
    draw_timelapse_label(&mut frame, "1 HOUR - DYNAMIC");
    assert_eq!(*frame.get_pixel(0, 0), unchanged_top);
    assert!(frame.pixels().any(|pixel| *pixel != image::Rgb([0, 0, 0])));
}

#[test]
fn timelapse_rejects_only_uniform_black_capture_failures() {
    let mut frame = image::RgbImage::new(320, 180);
    assert!(timelapse_frame_is_blank(&frame));

    frame.put_pixel(160, 90, image::Rgb([0, 0, 1]));
    assert!(!timelapse_frame_is_blank(&frame));
}

#[test]
fn shipping_settings_menu_preserves_art_tabs_and_supported_runtime_fields() {
    let presentation = embedded_presentation();
    assert!(
        presentation
            .textures
            .values()
            .any(|texture| texture.source_path == SETTINGS_BACKGROUND_TEXTURE_PATH),
        "missing authored Settings background texture"
    );
    assert_eq!(
        SETTINGS_TABS.map(settings_tab_label),
        [
            "Video",
            "Audio",
            "Gameplay",
            "Accessibility",
            "Streaming",
            "Connection"
        ]
    );
    let editable = SETTINGS_TABS
        .into_iter()
        .flat_map(settings_tab_indices)
        .copied()
        .collect::<Vec<_>>();
    assert_eq!(editable, (0..33).collect::<Vec<_>>());
    for index in editable {
        let (label, value) = settings_value_label(
            &PlayerSettings::default(),
            &BroadcastConfig::default(),
            index,
        );
        assert!(!label.is_empty());
        assert!(!value.is_empty());
        assert_eq!(
            settings_tab_for_index(index).unwrap(),
            match index {
                0..=10 => SettingsTab::Video,
                11..=14 => SettingsTab::Audio,
                15..=21 => SettingsTab::Gameplay,
                22..=24 => SettingsTab::Accessibility,
                _ => SettingsTab::Streaming,
            }
        );
        assert_eq!(
            settings_value_enabled(index, true),
            !settings_index_is_streaming(index)
        );
        assert!(settings_value_enabled(index, false));
    }
    assert_eq!(
        cycle_settings_tab(SettingsTab::Connection, true),
        SettingsTab::Video
    );
    assert_eq!(
        cycle_settings_tab(SettingsTab::Video, false),
        SettingsTab::Connection
    );
}

#[test]
fn accessibility_motion_preferences_preserve_authored_parameters() {
    let tree = Vec4::new(0.16, 0.7, 1.4, 0.25);
    let grass = Vec4::new(0.3, 0.9, 1.8, 0.2);
    let water = Vec4::new(0.14, 0.08, 0.065, 0.72);

    assert_eq!(reduced_tree_wind(tree, false), tree);
    assert_eq!(reduced_grass_wind(grass, false), grass);
    assert_eq!(reduced_water_wind(water, false), water);
    assert_eq!(
        reduced_tree_wind(tree, true),
        Vec4::new(0.16, 0.0, 0.0, 0.25)
    );
    assert_eq!(
        reduced_grass_wind(grass, true),
        Vec4::new(0.0, 0.9, 0.0, 0.2)
    );
    assert_eq!(
        reduced_water_wind(water, true),
        Vec4::new(0.0, 0.0, -0.065, 0.72)
    );
}

#[test]
fn loading_progress_exposes_accesskit_range_and_live_value() {
    let mut node = Node {
        width: percent(0),
        ..default()
    };
    let mut accessible = loading_progress_accessibility_node();
    assert_eq!(accessible.role(), Role::ProgressIndicator);
    assert_eq!(accessible.min_numeric_value(), Some(0.0));
    assert_eq!(accessible.max_numeric_value(), Some(100.0));

    set_loading_progress(&mut node, &mut accessible, 47.6);

    assert_eq!(node.width, percent(47.6));
    assert!((accessible.numeric_value().expect("progress value") - 47.6).abs() < 1e-5);
    assert_eq!(accessible.value(), Some("47.60%"));
}

#[test]
fn accessibility_scope_keeps_modal_navigation_inside_the_open_surface() {
    let mut menu = MenuRuntime::default();
    assert!(accessibility_scope_active(
        AccessibleButtonScope::MainMenu,
        GameState::MainMenu,
        &menu
    ));
    assert!(!accessibility_scope_active(
        AccessibleButtonScope::GameMenu,
        GameState::MainMenu,
        &menu
    ));

    menu.page = MenuPage::Settings;
    assert!(accessibility_scope_active(
        AccessibleButtonScope::Settings,
        GameState::MainMenu,
        &menu
    ));
    assert!(!accessibility_scope_active(
        AccessibleButtonScope::MainMenu,
        GameState::MainMenu,
        &menu
    ));
    menu.confirm_settings_close = true;
    assert!(accessibility_scope_active(
        AccessibleButtonScope::SettingsConfirm,
        GameState::MainMenu,
        &menu
    ));
    assert!(!accessibility_scope_active(
        AccessibleButtonScope::Settings,
        GameState::MainMenu,
        &menu
    ));
}

#[test]
fn shipping_ui_preserves_unity_nine_slice_borders() {
    let presentation = embedded_presentation();
    let border = |source_path: &str| {
        presentation
            .textures
            .values()
            .find(|texture| texture.source_path == source_path)
            .and_then(|texture| texture.sprite_border)
    };
    assert_eq!(border(MAIN_MENU_TEXTURE_PATHS[0]), Some([15.0; 4]));
    assert_eq!(border(SETTINGS_BACKGROUND_TEXTURE_PATH), Some([82.0; 4]));
    assert_eq!(border(VOTE_TEXTURE_PATHS[0]), Some([158.0; 4]));
    assert_eq!(border(VOTE_TEXTURE_PATHS[4]), Some([39.0; 4]));
    assert_eq!(border(CURRENT_EVENT_TEXTURE_PATHS[0]), Some([64.0; 4]));
    assert_eq!(border(OBJECTIVE_TEXTURE_PATHS[1]), Some([31.0; 4]));
    let asset_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    assert!(asset_root.join(UI_FONT_ASSET_PATH).is_file());
    assert!(asset_root.join(UI_DISPLAY_FONT_ASSET_PATH).is_file());

    let settings_slicer = presentation
        .textures
        .values()
        .find(|texture| texture.source_path == SETTINGS_BACKGROUND_TEXTURE_PATH)
        .and_then(|texture| texture.sprite_border)
        .map(|border| TextureSlicer {
            border: BorderRect::from(border),
            center_scale_mode: default(),
            sides_scale_mode: default(),
            max_corner_scale: 1.0,
        })
        .expect("the authored settings background is a nine-slice sprite");
    let mut render = RenderAssets {
        settings_ui_scale: 1.0,
        ..default()
    };
    render
        .ui_slicers
        .insert(SETTINGS_BACKGROUND_TEXTURE_PATH.to_owned(), settings_slicer);
    let image = settings_ui_image(&render, SETTINGS_BACKGROUND_TEXTURE_PATH, Handle::default());
    let NodeImageMode::Sliced(slicer) = image.image_mode else {
        panic!("settings menu must render through Bevy's nine-slice mode");
    };
    assert_eq!(slicer.border, BorderRect::all(82.0));
    assert!((slicer.max_corner_scale - 1.0).abs() < f32::EPSILON);

    let panel_image =
        settings_panel_ui_image(&render, SETTINGS_BACKGROUND_TEXTURE_PATH, Handle::default());
    let NodeImageMode::Sliced(panel_slicer) = panel_image.image_mode else {
        panic!("settings panel surface must retain the authored nine-slice");
    };
    assert_eq!(panel_slicer.border, BorderRect::all(82.0));
    assert!((panel_slicer.max_corner_scale - SETTINGS_PANEL_CORNER_SCALE).abs() < f32::EPSILON);

    for (source_path, expected_border) in [
        (VOTE_TEXTURE_PATHS[0], 158.0),
        (VOTE_TEXTURE_PATHS[4], 39.0),
    ] {
        render.ui_slicers.insert(
            source_path.to_owned(),
            TextureSlicer {
                border: BorderRect::all(expected_border),
                center_scale_mode: default(),
                sides_scale_mode: default(),
                max_corner_scale: 1.0,
            },
        );
        let vote_image = authored_ui_image(&render, source_path, Handle::default());
        let NodeImageMode::Sliced(vote_slicer) = vote_image.image_mode else {
            panic!("vote surfaces must render through Bevy's nine-slice mode");
        };
        assert_eq!(vote_slicer.border, BorderRect::all(expected_border));
    }
}

#[test]
fn settings_pointer_controls_switch_tabs_and_adjust_values() {
    let mut app = App::new();
    app.insert_resource(MenuRuntime {
        page: MenuPage::Settings,
        ..default()
    })
    .add_systems(
        Update,
        (settings_tab_buttons, settings_value_buttons).chain(),
    );
    #[cfg(target_os = "windows")]
    app.init_resource::<direct_broadcast::DirectBroadcastRuntime>();
    app.world_mut()
        .spawn((Interaction::Pressed, SettingsTabButton(SettingsTab::Audio)));

    app.update();

    let menu = app.world().resource::<MenuRuntime>();
    assert_eq!(menu.settings_tab, SettingsTab::Audio);
    assert_eq!(menu.selected, 11);

    app.world_mut().spawn((
        Interaction::Pressed,
        SettingsValueButton {
            index: 11,
            direction: -1,
        },
    ));
    app.update();

    let menu = app.world().resource::<MenuRuntime>();
    assert!((menu.draft.audio.master - 0.95).abs() < f32::EPSILON);
    assert_eq!(menu.selected, 11);
}

#[cfg(target_os = "windows")]
#[test]
fn streaming_pointer_controls_unlock_only_after_the_session_stops() {
    let mut app = App::new();
    app.insert_resource(MenuRuntime {
        page: MenuPage::Settings,
        settings_tab: SettingsTab::Streaming,
        selected: SETTINGS_STREAMING_FIRST_INDEX,
        ..default()
    })
    .init_resource::<direct_broadcast::DirectBroadcastRuntime>()
    .add_systems(Update, settings_value_buttons);
    app.world_mut()
        .resource_mut::<direct_broadcast::DirectBroadcastRuntime>()
        .set_phase_for_test(direct_broadcast::DirectBroadcastPhase::Broadcasting);
    let original = app
        .world()
        .resource::<MenuRuntime>()
        .streaming_draft
        .video_bitrate_kbps;
    app.world_mut().spawn((
        Interaction::Pressed,
        SettingsValueButton {
            index: 28,
            direction: 1,
        },
    ));

    app.update();

    let menu = app.world().resource::<MenuRuntime>();
    assert_eq!(menu.streaming_draft.video_bitrate_kbps, original);
    assert!(menu.feedback.contains("End the stream"));

    app.world_mut()
        .resource_mut::<direct_broadcast::DirectBroadcastRuntime>()
        .set_phase_for_test(direct_broadcast::DirectBroadcastPhase::Stopped);
    app.world_mut().spawn((
        Interaction::Pressed,
        SettingsValueButton {
            index: 28,
            direction: 1,
        },
    ));
    app.update();

    let menu = app.world().resource::<MenuRuntime>();
    assert_ne!(menu.streaming_draft.video_bitrate_kbps, original);
    assert!(menu.feedback.is_empty());
}

#[test]
fn settings_back_confirms_dirty_drafts_and_discard_restores_runtime() {
    let runtime = PlayerSettings::default();
    let streaming = BroadcastConfig::default();
    let mut menu = MenuRuntime {
        page: MenuPage::Settings,
        return_page: MenuPage::Game,
        draft: runtime.clone(),
        ..default()
    };
    request_settings_close(&mut menu, &runtime, &streaming);
    assert_eq!(menu.page, MenuPage::Game);
    assert!(!menu.confirm_settings_close);

    menu.page = MenuPage::Settings;
    menu.return_page = MenuPage::Game;
    menu.draft.audio.master = 0.25;
    request_settings_close(&mut menu, &runtime, &streaming);
    assert_eq!(menu.page, MenuPage::Settings);
    assert!(menu.confirm_settings_close);
    assert_eq!(menu.selected, 0);

    let mut app = App::new();
    app.insert_resource(menu)
        .insert_resource(RuntimePlayerSettings(runtime.clone()))
        .insert_resource(RuntimeConfig(GameConfig::default()))
        .add_systems(Update, settings_action_buttons);
    #[cfg(target_os = "windows")]
    app.init_resource::<direct_broadcast::DirectBroadcastRuntime>();
    app.world_mut()
        .spawn((Interaction::Pressed, SettingsAction::ConfirmDiscard));
    app.update();

    let menu = app.world().resource::<MenuRuntime>();
    assert_eq!(menu.page, MenuPage::Game);
    assert_eq!(menu.draft, runtime);
    assert!(!menu.confirm_settings_close);
}

#[test]
fn ssao_disables_incompatible_msaa_without_losing_the_preference() {
    let mut settings = PlayerSettings::default();
    settings.video.msaa_samples = 8;
    settings.video.ambient_occlusion = true;
    assert_eq!(player_msaa(&settings), Msaa::Off);
    assert_eq!(settings.video.msaa_samples, 8);

    settings.video.ambient_occlusion = false;
    assert_eq!(player_msaa(&settings), Msaa::Sample8);
}

#[test]
fn in_game_menu_overlay_cleanup_does_not_remove_world_entities() {
    let mut app = App::new();
    app.add_systems(Update, cleanup_menu_overlay);
    let overlay = app.world_mut().spawn(MenuOverlay).id();
    let game_menu = app.world_mut().spawn(GameMenuRoot).id();
    let world_entity = app.world_mut().spawn(WorldEntity).id();

    app.update();

    assert!(app.world().get_entity(overlay).is_err());
    assert!(app.world().get_entity(game_menu).is_err());
    assert!(app.world().get_entity(world_entity).is_ok());
}

#[test]
fn state_cleanup_is_idempotent_for_nested_marked_entities() {
    let mut app = App::new();
    app.add_systems(Update, cleanup_state_entities);
    let parent = app.world_mut().spawn(StateEntity).id();
    let child = app.world_mut().spawn(StateEntity).id();
    app.world_mut().entity_mut(parent).add_child(child);

    app.update();

    assert!(app.world().get_entity(parent).is_err());
    assert!(app.world().get_entity(child).is_err());
}

#[test]
fn accessibility_tagging_tolerates_ui_removed_before_deferred_annotation() {
    let mut app = App::new();
    app.add_systems(
        Update,
        (
            cleanup_state_entities.before(tag_accessible_text),
            tag_accessible_text,
        ),
    );
    let text = app
        .world_mut()
        .spawn((StateEntity, Text::new("Transient menu label")))
        .id();

    app.update();

    assert!(app.world().get_entity(text).is_err());
}

#[test]
fn runtime_and_benchmark_crowd_budgets_are_separate() {
    assert_eq!(actor_detail_budget(None, false), 6_400);
    assert_eq!(actor_detail_budget(None, true), 16);
    assert_eq!(actor_detail_budget(Some("24"), true), 24);
    assert_eq!(actor_detail_budget(Some("invalid"), false), 6_400);
}

#[test]
fn structural_footprints_clear_and_restore_foliage_visibility() {
    let config = GameConfig::default();
    let content = embedded_content();
    let generated = generate_world(&config.world);
    let building_id = StableId::new("building:foliage_test").unwrap();
    let house = &content.buildings[&StableId::new("building:house").unwrap()];
    let building_position = GridPos { x: 18, z: 18 };
    let rotation_quarter_turns = 1;
    let building_footprint = rotated_footprint(house.footprint, rotation_quarter_turns);
    let building_foliage = GridPos {
        x: building_position.x + building_footprint[0] - 1,
        z: building_position.z + building_footprint[1] - 1,
    };
    let (camp_archetype_id, camp_archetype) = content
        .archetypes
        .iter()
        .find(|(_, archetype)| archetype.enemy_spawner.is_some())
        .unwrap();
    let camp_id = StableId::new("enemy_camp:foliage_test").unwrap();
    let camp_position = GridPos { x: 42, z: 42 };
    let camp_foliage = GridPos {
        x: camp_position.x + camp_archetype.footprint[0] - 1,
        z: camp_position.z + camp_archetype.footprint[1] - 1,
    };
    let outside_foliage = GridPos { x: 2, z: 2 };
    let mut simulation = WorldSimulation::new(generated.seed);
    simulation.buildings.insert(
        building_id.clone(),
        BuildingState {
            id: building_id.clone(),
            archetype: house.archetype.clone(),
            position: building_position,
            rotation_quarter_turns,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );
    simulation.enemy_camps.insert(
        camp_id.clone(),
        EnemyCampState {
            id: camp_id.clone(),
            archetype: camp_archetype_id.clone(),
            position: camp_position,
            health: 100,
            spawn_remaining_seconds: 0.0,
            spawned_enemies: BTreeSet::new(),
        },
    );

    let mut app = App::new();
    app.insert_resource(RuntimeConfig(config));
    app.insert_resource(RuntimeContent(content));
    app.insert_resource(SimulationRuntime(simulation));
    app.insert_resource(WorldRuntime {
        generated,
        legacy_terrain_mesh: None,
        legacy_migration: None,
    });
    app.init_resource::<WorldRenderStats>();
    app.init_resource::<TraversalWearRuntime>();
    app.add_systems(Update, sync_foliage_clearance);
    let building_entity = app
        .world_mut()
        .spawn((
            FoliageVisual(StableId::new("foliage:test:building").unwrap()),
            GridLocation(building_foliage),
            Visibility::Inherited,
        ))
        .id();
    let camp_entity = app
        .world_mut()
        .spawn((
            FoliageVisual(StableId::new("foliage:test:camp").unwrap()),
            GridLocation(camp_foliage),
            Visibility::Inherited,
        ))
        .id();
    let outside_entity = app
        .world_mut()
        .spawn((
            FoliageVisual(StableId::new("foliage:test:outside").unwrap()),
            GridLocation(outside_foliage),
            Visibility::Inherited,
        ))
        .id();
    app.update();
    assert_eq!(
        app.world().get::<Visibility>(building_entity),
        Some(&Visibility::Hidden)
    );
    assert_eq!(
        app.world().get::<Visibility>(camp_entity),
        Some(&Visibility::Hidden)
    );
    assert_eq!(
        app.world().get::<Visibility>(outside_entity),
        Some(&Visibility::Inherited)
    );

    {
        let mut simulation = app.world_mut().resource_mut::<SimulationRuntime>();
        simulation.0.buildings.remove(&building_id);
    }
    app.update();
    assert_eq!(
        app.world().get::<Visibility>(building_entity),
        Some(&Visibility::Inherited)
    );
    assert_eq!(
        app.world().get::<Visibility>(camp_entity),
        Some(&Visibility::Hidden)
    );

    app.world_mut()
        .resource_mut::<SimulationRuntime>()
        .0
        .enemy_camps
        .remove(&camp_id);
    app.update();
    assert_eq!(
        app.world().get::<Visibility>(camp_entity),
        Some(&Visibility::Inherited)
    );
}

#[test]
fn workers_choose_nearest_compatible_station_and_reassign() {
    let config = GameConfig::default();
    let content = embedded_content();
    let logger = StableId::new("role:logger").unwrap();
    let actor_id = StableId::new("npc:station_test").unwrap();
    let station_id = StableId::new("building:runtime_station").unwrap();
    let lumbermill = &content.buildings[&StableId::new("building:lumbermill").unwrap()];
    let position = GridPos { x: 10, z: 10 };
    let mut simulation = WorldSimulation::new(config.world.seed);
    assert!(simulation.join_player(actor_id.clone(), position));
    simulation.assign_role(&actor_id, logger).unwrap();
    simulation.actors.get_mut(&actor_id).unwrap().station =
        Some(StableId::new("building:townhall").unwrap());
    simulation.buildings.insert(
        station_id.clone(),
        BuildingState {
            id: station_id.clone(),
            archetype: lumbermill.archetype.clone(),
            position: GridPos { x: 16, z: 10 },
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );

    ensure_actor_station(&content, &mut simulation, &config, &actor_id);
    assert_eq!(
        simulation.actors[&actor_id].station,
        Some(station_id.clone())
    );

    let closer_station = StableId::new("building:closer_runtime_station").unwrap();
    simulation.buildings.insert(
        closer_station.clone(),
        BuildingState {
            id: closer_station.clone(),
            archetype: lumbermill.archetype.clone(),
            position: GridPos { x: 11, z: 10 },
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );
    ensure_actor_station(&content, &mut simulation, &config, &actor_id);
    assert_eq!(
        simulation.actors[&actor_id].station,
        Some(closer_station.clone())
    );

    simulation
        .buildings
        .get_mut(&closer_station)
        .unwrap()
        .complete = false;
    ensure_actor_station(&content, &mut simulation, &config, &actor_id);
    assert_eq!(
        simulation.actors[&actor_id].station,
        Some(station_id.clone())
    );

    simulation.buildings.get_mut(&station_id).unwrap().complete = false;
    ensure_actor_station(&content, &mut simulation, &config, &actor_id);
    assert_eq!(
        simulation.actors[&actor_id]
            .station
            .as_ref()
            .map(StableId::as_str),
        Some("building:townhall")
    );
}

#[test]
fn resource_workers_deposit_at_their_authored_processing_stations() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world_with_content(&config.world, &content);
    let centre = nearest_walkable(
        &world,
        GridPos {
            x: world.navigation.width() / 2,
            z: world.navigation.height() / 2,
        },
    )
    .unwrap();

    for (serial, building_id, role_id, resource_id) in [
        (0, "building:stonemason", "role:miner", "resource:ore"),
        (1, "building:lumbermill", "role:logger", "resource:wood"),
        (2, "building:windmill", "role:gatherer", "resource:food"),
    ] {
        let building_definition = &content.buildings[&StableId::new(building_id).unwrap()];
        let station_id = StableId::new(format!("building:deposit_station_{serial}")).unwrap();
        let actor_id = StableId::new(format!("npc:deposit_worker_{serial}")).unwrap();
        let resource = StableId::new(resource_id).unwrap();
        let mut simulation = WorldSimulation::new(world.seed);
        simulation.buildings.insert(
            station_id.clone(),
            BuildingState {
                id: station_id.clone(),
                archetype: building_definition.archetype.clone(),
                position: centre,
                rotation_quarter_turns: 0,
                level: 1,
                health: building_base_max_health(&content, building_definition)
                    .try_into()
                    .unwrap(),
                complete: true,
            },
        );
        assert!(simulation.join_player(actor_id.clone(), centre));
        simulation
            .assign_role(&actor_id, StableId::new(role_id).unwrap())
            .unwrap();
        simulation.actors.get_mut(&actor_id).unwrap().station = Some(station_id.clone());
        let carry_capacity =
            effective_role_stats(&content, &simulation, &simulation.actors[&actor_id])
                .unwrap()
                .carry_capacity;
        simulation
            .gather(&actor_id, resource.clone(), carry_capacity)
            .unwrap();

        let (goal, target) =
            next_agent_goal(&simulation, &world, &config, &content, &actor_id, centre);
        assert_eq!(
            goal,
            AgentGoal::Deposit,
            "{role_id} did not choose its station"
        );
        let station = assigned_station(
            &content,
            &simulation,
            &config,
            &simulation.actors[&actor_id],
        )
        .unwrap();
        assert_eq!(station.id, &station_id);
        assert_eq!(
            Some(target),
            station_deposit_approach(&content, &simulation, &world, station, centre)
        );
        assert!(world.navigation.find_path(centre, target).is_ok());

        assert!(
            complete_agent_goal(
                &mut simulation,
                &mut world,
                &config,
                &content,
                &actor_id,
                &goal,
                target,
            )
            .is_none(),
            "deposit has no visual presentation"
        );
        assert_eq!(simulation.town_resources[&resource], carry_capacity);
        assert!(
            !simulation.actors[&actor_id]
                .inventory
                .contains_key(&resource)
        );
    }
}

#[test]
fn partially_loaded_miner_promotes_from_town_hall_and_deposits_when_ore_is_exhausted() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world_with_content(&config.world, &content);
    let actor_id = StableId::new("npc:partial_miner_station_regression").unwrap();
    let station_id = StableId::new("building:partial_miner_stonemason").unwrap();
    let stonemason = &content.buildings[&StableId::new("building:stonemason").unwrap()];
    let position = nearest_walkable(
        &world,
        GridPos {
            x: world.navigation.width() / 2,
            z: world.navigation.height() / 2,
        },
    )
    .unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    ensure_town_hall_state(&content, &config, &mut simulation);
    simulation.buildings.insert(
        station_id.clone(),
        BuildingState {
            id: station_id.clone(),
            archetype: stonemason.archetype.clone(),
            position,
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(&content, stonemason)).unwrap(),
            complete: true,
        },
    );
    assert!(simulation.join_player(actor_id.clone(), position));
    simulation
        .assign_role(&actor_id, StableId::new("role:miner").unwrap())
        .unwrap();
    simulation.actors.get_mut(&actor_id).unwrap().station =
        Some(StableId::new("building:townhall").unwrap());
    simulation
        .gather(&actor_id, StableId::new("resource:ore").unwrap(), 6)
        .unwrap();
    for resource in &mut world.resources {
        if resource.kind.as_str() == "resource:ore" {
            resource.amount = 0;
        }
    }

    ensure_actor_station(&content, &mut simulation, &config, &actor_id);
    assert_eq!(
        simulation.actors[&actor_id].station,
        Some(station_id.clone())
    );
    let (goal, target) =
        next_agent_goal(&simulation, &world, &config, &content, &actor_id, position);
    assert_eq!(goal, AgentGoal::Deposit);
    let station = assigned_station(
        &content,
        &simulation,
        &config,
        &simulation.actors[&actor_id],
    )
    .unwrap();
    assert_eq!(station.id, &station_id);
    assert_eq!(
        Some(target),
        station_deposit_approach(&content, &simulation, &world, station, position)
    );
}

#[test]
fn builders_reserve_distinct_unoccupied_construction_approaches() {
    let world = generate_world(&GameConfig::default().world);
    let origin = GridPos { x: 95, z: 95 };
    let first_builder = StableId::new("npc:builder_approach_first").unwrap();
    let second_builder = StableId::new("npc:builder_approach_second").unwrap();
    let occupied = BTreeMap::new();
    let first = unoccupied_building_approach(
        &world,
        origin,
        [3, 3],
        GridPos { x: 90, z: 95 },
        &first_builder,
        &occupied,
        &BTreeMap::new(),
    )
    .unwrap();
    let reservations = BTreeMap::from([(first, first_builder)]);
    let second = unoccupied_building_approach(
        &world,
        origin,
        [3, 3],
        GridPos { x: 90, z: 95 },
        &second_builder,
        &occupied,
        &reservations,
    )
    .unwrap();
    assert_ne!(second, first);
}

#[test]
fn movement_goals_reserve_their_approach_cells() {
    assert!(goal_reserves_approach(&AgentGoal::Deposit));
    assert!(goal_reserves_approach(&AgentGoal::HarvestFarm(
        StableId::new("building:farm_reservation").unwrap()
    )));
    assert!(goal_reserves_approach(&AgentGoal::Construct(
        StableId::new("building:construction_reservation").unwrap()
    )));
    assert!(goal_reserves_approach(&AgentGoal::PlantTree(GridPos {
        x: 4,
        z: 5,
    })));
    assert!(goal_reserves_approach(&AgentGoal::Wander));
    assert!(goal_uses_idle_approach_reservation(&AgentGoal::Wander));
    assert!(!goal_uses_idle_approach_reservation(&AgentGoal::Construct(
        StableId::new("building:construction_work_reservation").unwrap()
    )));
}

#[test]
fn construction_can_try_fine_space_inside_a_blocked_coarse_approach() {
    let mut world = generate_world(&GameConfig::default().world);
    let position = (1..world.navigation.height().saturating_sub(1))
        .flat_map(|z| {
            (1..world.navigation.width().saturating_sub(1)).map(move |x| GridPos { x, z })
        })
        .find(|position| {
            (position.z - 1..=position.z + 1).all(|z| {
                (position.x - 1..=position.x + 1)
                    .all(|x| world.navigation.is_walkable(GridPos { x, z }))
            })
        })
        .expect("generated world should contain a walkable 3x3 area");
    let blocked_approach = GridPos {
        x: position.x - 1,
        z: position.z,
    };
    world
        .navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: blocked_approach,
                max: blocked_approach,
            },
            true,
        )
        .unwrap();
    let actor = StableId::new("npc:fine_construction_approach").unwrap();

    assert!(
        !building_approaches(&world, position, [1, 1], blocked_approach)
            .contains(&blocked_approach)
    );
    assert_eq!(
        unoccupied_construction_approach(
            &world,
            position,
            [1, 1],
            blocked_approach,
            &actor,
            &BTreeMap::new(),
            &BTreeMap::new(),
        ),
        Some(blocked_approach)
    );
}

#[test]
fn floorplan_owner_label_names_the_player_and_building() {
    let content = embedded_content();
    let owner = StableId::new("player:floorplan_owner").unwrap();
    let mut simulation = WorldSimulation::new(7);
    assert!(simulation.join_player(owner.clone(), GridPos { x: 4, z: 5 }));
    simulation.actors.get_mut(&owner).unwrap().display_name = Some("Aidan".to_owned());
    let definition = &content.buildings[&StableId::new("building:stonemason").unwrap()];

    assert_eq!(
        building_placement_overlay_text(&simulation, definition, &owner),
        "Aidan · Stonemason floorplan"
    );
}

#[test]
fn station_search_matches_unity_broad_phase_and_distance_ranking() {
    let content = embedded_content();
    let station_id = StableId::new("building:lumbermill").unwrap();
    let definition = content.buildings[&station_id].station.as_ref().unwrap();
    let station = StationCandidate {
        id: &station_id,
        position: GridPos { x: 100, z: 100 },
        definition,
    };
    let range = station_search_range_cells(station);

    // Unity's cell-space broad phase queries an axis-aligned rectangle,
    // so a diagonal target at both authored extents remains eligible.
    assert!(within_station_search_region(
        GridPos {
            x: station.position.x + range,
            z: station.position.z + range,
        },
        station,
    ));
    assert!(!within_station_search_region(
        GridPos {
            x: station.position.x + range + 1,
            z: station.position.z,
        },
        station,
    ));

    // Unity then ranks cached targets using Vector3.SqrMagnitude. The
    // diagonal target is closer even though its Manhattan sum is larger.
    let axis = GridPos { x: 100, z: 106 };
    let diagonal = GridPos { x: 104, z: 104 };
    assert!(
        grid_distance_squared(diagonal, station.position)
            < grid_distance_squared(axis, station.position)
    );
    assert!(
        diagonal.x.abs_diff(station.position.x) + diagonal.z.abs_diff(station.position.z)
            > axis.x.abs_diff(station.position.x) + axis.z.abs_diff(station.position.z)
    );
}

#[test]
fn shipping_station_target_policies_match_unity_target_settings() {
    let content = embedded_content();
    let update = ["target:tree", "target:ore", "target:bush", "target:farm"];
    let clear = [
        "target:player",
        "target:fish",
        "target:enemy",
        "target:boss",
        "target:building",
        "target:damaged_building",
        "target:construction",
        "target:injured_player",
        "target:dead_player",
    ];
    assert_eq!(content.station_target_update_modes.len(), 13);
    for kind in update {
        assert_eq!(
            content.station_target_update_modes[&StableId::new(kind).unwrap()],
            StationUpdateMode::Update
        );
    }
    for kind in clear {
        assert_eq!(
            content.station_target_update_modes[&StableId::new(kind).unwrap()],
            StationUpdateMode::Clear
        );
    }
}

#[test]
fn resource_visibility_signature_ignores_non_depleting_amount_updates() {
    let mut world = generate_world(&GameConfig::default().world);
    world.resources = vec![stream_town_domain::GeneratedResource {
        id: StableId::new("resource:visibility-a").unwrap(),
        kind: StableId::new("resource:ore").unwrap(),
        target_kind: StableId::new("target:ore").unwrap(),
        position: GridPos { x: 4, z: 7 },
        offset_milli_cells: [0, 0],
        generation_occupancy: [0, 0],
        amount: 100,
    }];
    let initial = resource_visibility_signature(&world);

    world.resources[0].amount = 99;
    assert_eq!(resource_visibility_signature(&world), initial);

    world.resources[0].amount = 0;
    let depleted = resource_visibility_signature(&world);
    assert_ne!(depleted, initial);

    world.resources.push(stream_town_domain::GeneratedResource {
        id: StableId::new("resource:visibility-b").unwrap(),
        position: GridPos { x: 8, z: 9 },
        amount: 100,
        ..world.resources[0].clone()
    });
    assert_ne!(resource_visibility_signature(&world), depleted);
}

#[test]
fn station_update_mode_retains_targets_while_clear_mode_rebuilds_nearest_first() {
    let config = GameConfig::default();
    let mut content = embedded_content();
    let station_id = StableId::new("building:townhall").unwrap();
    content
        .buildings
        .get_mut(&station_id)
        .unwrap()
        .station
        .as_mut()
        .unwrap()
        .max_targets = 1;
    let simulation = WorldSimulation::new(config.world.seed);
    let mut world = generate_world(&config.world);
    world.resources.clear();
    let station = station_candidate(&content, &simulation, &config, &station_id).unwrap();
    let retained = stream_town_domain::GeneratedResource {
        id: StableId::new("resource:cache_retained").unwrap(),
        kind: StableId::new("resource:wood").unwrap(),
        target_kind: StableId::new("target:tree").unwrap(),
        position: GridPos {
            x: station.position.x.saturating_add(2),
            z: station.position.z,
        },
        offset_milli_cells: [0, 0],
        generation_occupancy: [0, 0],
        amount: 100,
    };
    world.resources.push(retained.clone());
    let mut runtime = StationTargetRuntime::default();
    let mut resource_index = StationResourceTargetIndex::default();
    populate_station_target_cache(
        &mut runtime,
        &mut resource_index,
        &content,
        &simulation,
        &world,
        &config,
        &station_id,
    );
    let tree = StableId::new("target:tree").unwrap();
    assert_eq!(
        runtime.stations[&station_id].targets[&tree],
        std::slice::from_ref(&retained.id)
    );

    let nearer = stream_town_domain::GeneratedResource {
        id: StableId::new("resource:cache_nearer").unwrap(),
        position: GridPos {
            x: station.position.x.saturating_add(1),
            z: station.position.z,
        },
        ..retained.clone()
    };
    world.resources.push(nearer.clone());
    populate_station_target_cache(
        &mut runtime,
        &mut resource_index,
        &content,
        &simulation,
        &world,
        &config,
        &station_id,
    );
    assert_eq!(
        runtime.stations[&station_id].targets[&tree],
        std::slice::from_ref(&retained.id),
        "Unity Update mode keeps valid cached targets"
    );

    content
        .station_target_update_modes
        .insert(tree.clone(), StationUpdateMode::Clear);
    populate_station_target_cache(
        &mut runtime,
        &mut resource_index,
        &content,
        &simulation,
        &world,
        &config,
        &station_id,
    );
    assert_eq!(
        runtime.stations[&station_id].targets[&tree],
        [nearer.id],
        "Unity Clear mode replaces the bounded cache from nearest to farthest"
    );
}

#[test]
fn deposit_cooldown_matches_unity_inventory_transfer_delay() {
    let config = GameConfig::default();
    let content = embedded_content();
    let actor_id = StableId::new("npc:deposit_timing").unwrap();
    let mut simulation = WorldSimulation::new(config.world.seed);
    assert!(simulation.join_player(actor_id.clone(), GridPos { x: 1, z: 1 }));
    assert!(
        (action_cooldown(&content, &simulation, &actor_id, &AgentGoal::Deposit) - 2.5).abs()
            <= f32::EPSILON
    );

    let position = GridPos { x: 1, z: 1 };
    let mut agent = Agent {
        id: actor_id,
        kind: ActorKind::Player,
        archetype: StableId::new("archetype:deposit_timing").unwrap(),
        goal: AgentGoal::Deposit,
        spawn: position,
        origin: position,
        navigation_position: placement_to_navigation_centre(position),
        path: vec![position],
        path_index: 1,
        target: position,
        action_cooldown_seconds: 0.0,
        action_started: false,
        repath_remaining_seconds: 0.0,
        health_regen_accumulator: 0.0,
        wander_sequence: 0,
        previous_wander_origin: None,
    };
    assert_eq!(
        arrived_action_phase(&mut agent, 2.5),
        ArrivedActionPhase::Started
    );
    assert!(agent.action_started);
    assert!((agent.action_cooldown_seconds - 2.5).abs() <= f32::EPSILON);
    agent.action_cooldown_seconds = 0.1;
    assert_eq!(
        arrived_action_phase(&mut agent, 0.0),
        ArrivedActionPhase::Waiting
    );
    agent.action_cooldown_seconds = 0.0;
    assert_eq!(
        arrived_action_phase(&mut agent, 0.0),
        ArrivedActionPhase::Complete
    );
}

#[test]
fn resource_workers_only_receive_targets_from_their_assigned_station_catalog() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    world.resources.clear();
    let close = stream_town_domain::GeneratedResource {
        id: StableId::new("resource:station_close").unwrap(),
        kind: StableId::new("resource:wood").unwrap(),
        target_kind: StableId::new("target:tree").unwrap(),
        position: GridPos { x: 5, z: 5 },
        offset_milli_cells: [0, 0],
        generation_occupancy: [5, 5],
        amount: 100,
    };
    let far = stream_town_domain::GeneratedResource {
        id: StableId::new("resource:station_far").unwrap(),
        kind: StableId::new("resource:wood").unwrap(),
        target_kind: StableId::new("target:tree").unwrap(),
        position: GridPos { x: 63, z: 63 },
        offset_milli_cells: [0, 0],
        generation_occupancy: [63, 63],
        amount: 100,
    };
    for resource in [&close, &far] {
        world
            .navigation
            .set_blocked(
                stream_town_domain::DirtyRegion {
                    min: resource.position,
                    max: resource.position,
                },
                false,
            )
            .unwrap();
    }
    world.resources.extend([close.clone(), far.clone()]);

    let lumbermill = &content.buildings[&StableId::new("building:lumbermill").unwrap()];
    let station_id = StableId::new("building:station_catalog_lumbermill").unwrap();
    let actor_id = StableId::new("npc:station_catalog_logger").unwrap();
    let current = GridPos { x: 4, z: 5 };
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.buildings.insert(
        station_id.clone(),
        BuildingState {
            id: station_id.clone(),
            archetype: lumbermill.archetype.clone(),
            position: GridPos { x: 4, z: 4 },
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(&content, lumbermill)).unwrap(),
            complete: true,
        },
    );
    assert!(simulation.join_player(actor_id.clone(), current));
    simulation
        .assign_role(&actor_id, StableId::new("role:logger").unwrap())
        .unwrap();
    simulation.actors.get_mut(&actor_id).unwrap().station = Some(station_id);

    assert_eq!(
        compatible_target_ids(
            &content,
            &simulation,
            &world,
            &config,
            &simulation.actors[&actor_id],
        ),
        vec![close.id.clone()],
    );
    assert_eq!(
        next_agent_goal(&simulation, &world, &config, &content, &actor_id, current).0,
        AgentGoal::Gather(close.id),
    );
}

#[test]
fn target_score_uses_authored_euclidean_distance_and_assignment_penalty() {
    let scoring = TargetingScoreDef {
        assignment_penalty_milli: 10_000_000,
        distance_penalty_milli_per_cell: 100,
    };
    assert_eq!(
        target_score_milli(GridPos { x: 3, z: 4 }, GridPos { x: 0, z: 0 }, 2, &scoring,),
        20_000_500,
    );
}

#[test]
fn action_ranges_and_tower_acquisition_are_euclidean() {
    let centre = GridPos { x: 20, z: 20 };
    let diagonal = GridPos { x: 23, z: 24 };
    assert!(within_milli_cell_range(diagonal, centre, 5_000));
    assert!(!within_milli_cell_range(diagonal, centre, 4_999));

    let axis_id = StableId::new("actor:tower_axis_enemy").unwrap();
    let diagonal_id = StableId::new("actor:tower_diagonal_enemy").unwrap();
    let mut simulation = WorldSimulation::new(1);
    assert!(simulation.join_player(axis_id.clone(), GridPos { x: 26, z: 20 }));
    assert!(simulation.join_player(diagonal_id.clone(), GridPos { x: 24, z: 24 }));
    for actor in [&axis_id, &diagonal_id] {
        simulation
            .assign_role(actor, StableId::new("role:enemy").unwrap())
            .unwrap();
    }
    assert_eq!(
        best_tower_target(&simulation, centre, 10_000).map(|actor| &actor.id),
        Some(&diagonal_id),
    );
    assert!(best_tower_target(&simulation, centre, 5_000).is_none());
}

#[test]
fn authored_target_sizes_drive_unity_action_reach_formulas() {
    let content = embedded_content();
    let player_archetype =
        archetype_id_by_source(&content, ArchetypeKind::Player, "Player_Character.prefab").unwrap();
    let blargul_archetype =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Blargul.prefab").unwrap();
    assert_eq!(
        content.archetypes[&player_archetype].target_size_milli_cells,
        750
    );
    assert_eq!(
        content.archetypes[&blargul_archetype].target_size_milli_cells,
        500
    );

    let defender_id = StableId::new("npc:target_size_defender").unwrap();
    let enemy_id = StableId::new("actor:target_size_blargul").unwrap();
    let mut combat = WorldSimulation::new(1);
    assert!(combat.join_player(defender_id.clone(), GridPos { x: 20, z: 20 }));
    combat
        .assign_role(&defender_id, StableId::new("role:defender").unwrap())
        .unwrap();
    combat.actors.get_mut(&defender_id).unwrap().archetype = Some(player_archetype.clone());
    assert!(combat.spawn_enemy(
        enemy_id.clone(),
        blargul_archetype,
        GridPos { x: 21, z: 20 },
        100,
    ));
    let defender = &combat.actors[&defender_id];
    let enemy = &combat.actors[&enemy_id];
    assert!(within_actor_attack_range(
        &content,
        &combat,
        defender,
        enemy,
        GridPos { x: 20, z: 20 },
    ));
    assert!(!within_actor_attack_range(
        &content,
        &combat,
        defender,
        enemy,
        GridPos { x: 20, z: 19 },
    ));

    combat.actors.get_mut(&defender_id).unwrap().position = GridPos { x: 22, z: 20 };
    let defender = &combat.actors[&defender_id];
    let enemy = &combat.actors[&enemy_id];
    assert!(within_actor_attack_range(
        &content,
        &combat,
        enemy,
        defender,
        GridPos { x: 20, z: 20 },
    ));
    assert!(!within_actor_attack_range(
        &content,
        &combat,
        enemy,
        defender,
        GridPos { x: 19, z: 20 },
    ));

    let priest_id = StableId::new("npc:target_size_priest").unwrap();
    assert!(combat.join_player(priest_id.clone(), GridPos { x: 20, z: 20 }));
    combat
        .assign_role(&priest_id, StableId::new("role:priest").unwrap())
        .unwrap();
    let priest = &combat.actors[&priest_id];
    assert!(within_actor_heal_range(
        &content,
        &combat,
        priest,
        &combat.actors[&defender_id],
        GridPos { x: 20, z: 20 },
    ));
    assert!(!within_actor_heal_range(
        &content,
        &combat,
        priest,
        &combat.actors[&defender_id],
        GridPos { x: 19, z: 20 },
    ));

    let builder_id = StableId::new("npc:target_size_builder").unwrap();
    assert!(combat.join_player(builder_id.clone(), GridPos { x: 20, z: 20 }));
    combat
        .assign_role(&builder_id, StableId::new("role:builder").unwrap())
        .unwrap();
    let house = &content.buildings[&StableId::new("building:house").unwrap()];
    assert_eq!(
        content.archetypes[&house.archetype].target_size_milli_cells,
        2_000
    );
    let mut building = BuildingState {
        id: StableId::new("building:target_size_house").unwrap(),
        archetype: house.archetype.clone(),
        position: GridPos { x: 22, z: 19 },
        rotation_quarter_turns: 0,
        level: 1,
        health: BUILDING_MAX_HEALTH,
        complete: false,
    };
    assert!(within_building_work_range(
        &content,
        &combat,
        &combat.actors[&builder_id],
        &building,
        GridPos { x: 20, z: 20 },
    ));
    building.position.x += 1;
    assert!(!within_building_work_range(
        &content,
        &combat,
        &combat.actors[&builder_id],
        &building,
        GridPos { x: 20, z: 20 },
    ));
}

#[test]
fn combat_and_healing_bypass_station_target_caches() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let tower_id = StableId::new("building:target_sensor_tower").unwrap();
    let tower = &content.buildings[&StableId::new("building:tower").unwrap()];
    let tower_state = BuildingState {
        id: tower_id.clone(),
        archetype: tower.archetype.clone(),
        position: GridPos { x: 1, z: 1 },
        rotation_quarter_turns: 0,
        level: 1,
        health: BUILDING_MAX_HEALTH,
        complete: true,
    };
    let current = GridPos { x: 30, z: 30 };

    let defender = StableId::new("npc:target_sensor_defender").unwrap();
    let axis_enemy = StableId::new("actor:target_sensor_axis_enemy").unwrap();
    let diagonal_enemy = StableId::new("actor:target_sensor_diagonal_enemy").unwrap();
    let mut combat = WorldSimulation::new(world.seed);
    assert!(combat.join_player(defender.clone(), current));
    assert!(combat.join_player(axis_enemy.clone(), GridPos { x: 30, z: 36 }));
    assert!(combat.join_player(diagonal_enemy.clone(), GridPos { x: 34, z: 34 }));
    combat
        .assign_role(&defender, StableId::new("role:defender").unwrap())
        .unwrap();
    for enemy in [&axis_enemy, &diagonal_enemy] {
        combat
            .assign_role(enemy, StableId::new("role:enemy").unwrap())
            .unwrap();
    }
    combat
        .buildings
        .insert(tower_id.clone(), tower_state.clone());
    combat.actors.get_mut(&defender).unwrap().station = Some(tower_id.clone());
    assert!(assigned_station(&content, &combat, &config, &combat.actors[&defender]).is_some());
    assert_eq!(
        next_agent_goal(&combat, &world, &config, &content, &defender, current).0,
        AgentGoal::Attack(diagonal_enemy)
    );

    let priest = StableId::new("npc:target_sensor_priest").unwrap();
    let patient = StableId::new("npc:target_sensor_patient").unwrap();
    let patient_position = GridPos { x: 35, z: 35 };
    let mut healing = WorldSimulation::new(world.seed);
    assert!(healing.join_player(priest.clone(), current));
    assert!(healing.join_player(patient.clone(), patient_position));
    healing
        .assign_role(&priest, StableId::new("role:priest").unwrap())
        .unwrap();
    healing.damage_actor(&patient, 20).unwrap();
    healing.buildings.insert(tower_id.clone(), tower_state);
    healing.actors.get_mut(&priest).unwrap().station = Some(tower_id);
    assert!(assigned_station(&content, &healing, &config, &healing.actors[&priest]).is_some());
    assert_eq!(
        next_agent_goal(&healing, &world, &config, &content, &priest, current).0,
        AgentGoal::Heal(patient)
    );
}

#[test]
fn defenders_prioritize_nearby_threats_then_their_defence_anchor() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let town_hall = town_hall_grid_position(&config);
    let defender_position = GridPos {
        x: town_hall.x.saturating_add(20),
        z: town_hall.z,
    };
    let close_to_defender = GridPos {
        x: defender_position.x.saturating_sub(1),
        z: defender_position.z,
    };
    let close_to_town = GridPos {
        x: town_hall.x.saturating_add(5),
        z: town_hall.z,
    };
    let fighter = StableId::new("npc:defender_priority").unwrap();
    let near_actor = StableId::new("enemy:near_actor").unwrap();
    let near_town = StableId::new("enemy:near_town").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(fighter.clone(), defender_position));
    assert!(simulation.join_player(near_actor.clone(), close_to_defender));
    assert!(simulation.join_player(near_town.clone(), close_to_town));
    for enemy in [&near_actor, &near_town] {
        simulation
            .assign_role(enemy, StableId::new("role:enemy").unwrap())
            .unwrap();
    }
    simulation
        .assign_role(&fighter, StableId::new("role:defender").unwrap())
        .unwrap();
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &fighter,
            defender_position,
        )
        .0,
        AgentGoal::Attack(near_actor.clone())
    );

    simulation.actors.get_mut(&near_actor).unwrap().alive = false;
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &fighter,
            defender_position,
        )
        .0,
        AgentGoal::Attack(near_town.clone())
    );
    simulation.actors.get_mut(&near_actor).unwrap().alive = true;

    simulation
        .assign_role(&fighter, StableId::new("role:ranger").unwrap())
        .unwrap();
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &fighter,
            defender_position,
        )
        .0,
        AgentGoal::Attack(near_actor)
    );
}

#[test]
fn equipment_visibility_matches_role_and_carry_state() {
    let content = embedded_content();
    let logger = content.roles[&StableId::new("role:logger").unwrap()]
        .equipment
        .as_ref()
        .unwrap();
    assert!(equipment_node_visible(logger, 0, "Body_Logger_Slim", false));
    assert!(equipment_node_visible(
        logger,
        2,
        "Body_Logger_Feminine",
        false
    ));
    assert!(equipment_node_visible(
        logger,
        0,
        "RHand_LoggerToolAxe_Starter",
        false
    ));
    assert!(!equipment_node_visible(
        logger,
        0,
        "LHand_LoggerCarryWood",
        false
    ));
    assert!(equipment_node_visible(
        logger,
        0,
        "LHand_LoggerCarryWood",
        true
    ));
    assert!(!carried_resource_visible(
        MovementAnimationState::Idle,
        true
    ));
    assert!(carried_resource_visible(
        MovementAnimationState::Moving,
        true
    ));
    assert!(!carried_resource_visible(
        MovementAnimationState::Moving,
        false
    ));

    let defender = content.roles[&StableId::new("role:defender").unwrap()]
        .equipment
        .as_ref()
        .unwrap();
    assert!(equipment_node_visible(
        defender,
        0,
        "LHand_DefenderToolShield_Starter",
        false
    ));
    assert!(equipment_node_visible(
        defender,
        0,
        "Helmet_Defender",
        false
    ));
    assert!(!equipment_node_visible(
        defender,
        0,
        "Body_Logger_Slim",
        false
    ));
    let equipment_names = equipment_node_names(&content);
    assert!(equipment_names.contains(canonical_equipment_node_name(
        "RHand_NecromancerToolScepter_Starter"
    )));
    assert_eq!(
        canonical_equipment_node_name("Helmet_Ruler"),
        "Helmet_Ruler"
    );
    assert!(player_equipment_slot_node("Body_Default_Slim"));
    assert!(player_equipment_slot_node("Back_CommanderBanner"));
    assert!(player_equipment_slot_node("RHand_LoggerToolAxe_Starter"));
    assert!(!player_equipment_slot_node("Head_Box"));
    assert!(!equipment_node_visible(
        defender,
        0,
        "Body_Default_Slim",
        false
    ));
}

#[test]
fn enemy_model_selection_is_stable_and_preserves_weapon_contracts() {
    let content = embedded_content();
    let skeleton =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Skeleton.prefab").unwrap();
    let models = content.archetypes[&skeleton].enemy_models.as_ref().unwrap();
    assert!(models.base_models.is_empty());
    assert_eq!(models.permanent_models.len(), 2);
    assert_eq!(models.optional_models.len(), 12);
    assert_eq!(models.weapons.len(), 3);

    let actor_id = StableId::new("actor:model_selection_skeleton").unwrap();
    let selection = enemy_model_selection(&actor_id, models);
    assert_eq!(selection, enemy_model_selection(&actor_id, models));
    assert!(selection.weapon.is_some_and(|index| index < 3));
    assert!(
        models
            .permanent_models
            .iter()
            .all(|name| { enemy_model_node_visible(models, &selection, name) })
    );
    for (index, weapon) in models.weapons.iter().enumerate() {
        assert_eq!(
            enemy_model_node_visible(models, &selection, &weapon.main_model),
            selection.weapon == Some(index)
        );
    }

    let mut simulation = WorldSimulation::new(42);
    assert!(simulation.spawn_enemy(actor_id.clone(), skeleton, GridPos { x: 10, z: 10 }, 100,));
    let contract =
        enemy_animation_contract(&content, &simulation.actors[&actor_id], &actor_id).unwrap();
    let weapon = &models.weapons[selection.weapon.unwrap()];
    assert_eq!(contract.action_animation, weapon.action_animation);
    assert_eq!(
        contract.action_animation_variants,
        weapon.action_animation_variants
    );
    assert_eq!(contract.run_animation, weapon.run_animation);
}

#[test]
fn shipping_enemies_use_authored_rigs_and_translated_controllers() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let expected = [
        ("Enemy_Blargul.prefab", "Blargul", "Blargul.glb"),
        ("Enemy_Goblin.prefab", "Goblin", "Goblin.glb"),
        (
            "Enemy_Goblin_BatteringRam.prefab",
            "BatteringRam",
            "Goblin_BatteringRam.glb",
        ),
        ("Enemy_Goblin_Boss.prefab", "GoblinBoss", "GoblinBoss.glb"),
        ("Enemy_Minotaur.prefab", "Minotaur", "Minotaur.glb"),
        ("Enemy_MinotaurBoss.prefab", "Minotaur", "Minotaur.glb"),
        (
            "Enemy_NecroSlasher.prefab",
            "NecroSlasher",
            "NecroSlasher.glb",
        ),
        (
            "Enemy_NecroStalker.prefab",
            "NecroStalker",
            "NecroStalker.glb",
        ),
        ("Enemy_Skeleton.prefab", "Skeleton", "Skeleton.glb"),
    ];
    for (source, controller_name, rig_suffix) in expected {
        let archetype = archetype_by_source(&content, ArchetypeKind::Enemy, source).unwrap();
        let scene = default_archetype_scene(archetype).unwrap();
        assert!(scene.asset_path.ends_with(rig_suffix), "{source}");
        let binding = &presentation.prefab_bindings[&archetype.source_guid];
        assert_eq!(
            presentation.controllers[&binding.controller].display_name, controller_name,
            "{source}"
        );
        assert_eq!(
            binding.rig_scene.as_deref(),
            Some(scene.asset_path.as_str()),
            "{source}"
        );
        assert!(
            converted_animation_spec(archetype, &presentation).is_some(),
            "{source} does not have a complete translated locomotion contract: {:?}",
            presentation.controllers[&binding.controller]
                .states
                .values()
                .filter(|state| state.display_name.eq_ignore_ascii_case("locomotion"))
                .flat_map(|state| &state.motions)
                .map(|motion| (
                    presentation.clips[&motion.clip].display_name.clone(),
                    presentation.clips[&motion.clip].transform_tracks.len(),
                    presentation.clips[&motion.clip].rig_asset_path.clone(),
                ))
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn building_level_research_is_contiguous_and_distributed_across_ages() {
    let content = embedded_content();
    let age_count = content
        .technology
        .nodes
        .values()
        .map(|node| authored_technology_age(&node.age))
        .max()
        .unwrap();

    for building in content.buildings.keys() {
        let researched_levels = content
            .technology
            .nodes
            .values()
            .filter_map(|node| {
                node.building_level_caps
                    .get(building)
                    .copied()
                    .filter(|level| *level >= 2)
            })
            .collect::<Vec<_>>();
        let Some(maximum) = researched_levels.iter().max().copied() else {
            continue;
        };
        let minimum_age = content
            .technology
            .nodes
            .values()
            .filter(|node| node.unlocked_buildings.contains(building))
            .map(|node| authored_technology_age(&node.age))
            .max()
            .unwrap_or(1);
        for level in 2..=maximum {
            let matches = content
                .technology
                .nodes
                .values()
                .filter(|node| node.building_level_caps.get(building) == Some(&level))
                .collect::<Vec<_>>();
            assert_eq!(
                matches.len(),
                1,
                "{building} must have exactly one level-{level} research"
            );
            let distributed = u32::from(level.saturating_sub(1))
                .saturating_mul(u32::from(age_count))
                / u32::from(maximum)
                + 1;
            let expected_age = u8::try_from(distributed)
                .unwrap_or(age_count)
                .clamp(minimum_age, age_count);
            assert_eq!(
                authored_technology_age(&matches[0].age),
                expected_age,
                "{building} level {level} is assigned to the wrong age"
            );
        }
    }
}

#[test]
fn age_two_research_requires_level_two_town_hall_and_has_no_tech_count_gate() {
    let config = GameConfig::default();
    let content = embedded_content();
    let town_hall = StableId::new("building:townhall").unwrap();
    let (age_up_id, age_up) = content
        .technology
        .nodes
        .iter()
        .find(|(_, node)| node.aged_buildings.contains(&town_hall))
        .expect("dedicated town age-up research exists");
    assert_eq!(age_up.display_name, "Advance to Age 2");
    assert!(age_up.building_level_caps.is_empty());
    assert_eq!(age_up.prerequisites.len(), 1);
    let level_two = &age_up.prerequisites[0];
    assert_eq!(
        content.technology.nodes[level_two]
            .building_level_caps
            .get(&town_hall),
        Some(&2)
    );

    let mut simulation = WorldSimulation::new(config.world.seed);
    ensure_town_hall_state(&content, &config, &mut simulation);
    simulation.unlocked_technology.insert(level_two.clone());
    assert!(
        !eligible_technology_ids(&content, &simulation).contains(age_up_id),
        "research alone must not replace the physical Town Hall upgrade"
    );

    simulation.buildings.get_mut(&town_hall).unwrap().level = 2;
    assert!(
        eligible_technology_ids(&content, &simulation).contains(age_up_id),
        "Age 2 must become eligible immediately without a 50-tech requirement"
    );

    let (age_two_id, age_two) = content
        .technology
        .nodes
        .iter()
        .find(|(_, node)| {
            authored_technology_age(&node.age) == 2
                && !node.aged_buildings.contains(&town_hall)
                && !node.prerequisites.is_empty()
        })
        .expect("ordinary Age 2 research exists");
    simulation
        .unlocked_technology
        .extend(age_two.prerequisites.iter().cloned());
    assert!(
        !eligible_technology_ids(&content, &simulation).contains(age_two_id),
        "ordinary Age 2 research must remain hidden while the town is Age 1"
    );
    simulation.unlocked_technology.insert(age_up_id.clone());
    assert!(eligible_technology_ids(&content, &simulation).contains(age_two_id));
}

#[test]
fn town_hall_is_a_stable_authoritative_building_and_ages_with_technology() {
    let config = GameConfig::default();
    let content = embedded_content();
    let town_hall_id = StableId::new("building:townhall").unwrap();
    let definition = &content.buildings[&town_hall_id];
    let mut simulation = WorldSimulation::new(config.world.seed);

    ensure_town_hall_state(&content, &config, &mut simulation);
    let town_hall = &simulation.buildings[&town_hall_id];
    assert_eq!(town_hall.id, town_hall_id);
    assert_eq!(town_hall.archetype, definition.archetype);
    assert_eq!(
        town_hall.position,
        town_hall_placement_position(&config, definition.footprint)
    );
    assert_eq!(town_hall.level, 1);
    assert_eq!(town_hall.health, building_max_health(&content, town_hall));
    assert_eq!(town_hall.health, 250);
    assert!(town_hall.complete);
    assert_eq!(constructed_building_count(&simulation), 0);
    assert_eq!(building_age(&content, &simulation, &town_hall_id, 1), 1);

    let age_technology = content
        .technology
        .nodes
        .iter()
        .find(|(_, technology)| technology.aged_buildings.contains(&town_hall_id))
        .map(|(id, _)| id.clone())
        .expect("converted technology contains the Town Hall age unlock");
    simulation.unlocked_technology.insert(age_technology);
    assert_eq!(building_age(&content, &simulation, &town_hall_id, 1), 2);
    assert_eq!(
        archetype_scene_for_age(&content.archetypes[&definition.archetype], 2)
            .and_then(|scene| scene.age),
        Some(2)
    );

    let saved = simulation.buildings[&town_hall_id].clone();
    ensure_town_hall_state(&content, &config, &mut simulation);
    assert_eq!(simulation.buildings.len(), 1);
    assert_eq!(simulation.buildings[&town_hall_id], saved);
}

#[test]
fn live_agent_goals_select_authored_action_animation_contracts() {
    let content = embedded_content();
    let actor_id = StableId::new("npc:animation_test").unwrap();
    let mut simulation = WorldSimulation::new(42);
    assert!(simulation.join_player(actor_id.clone(), GridPos { x: 10, z: 10 }));
    let agent_for = |goal| Agent {
        id: actor_id.clone(),
        kind: ActorKind::Player,
        archetype: StableId::new("archetype:viewer").unwrap(),
        goal,
        spawn: GridPos { x: 10, z: 10 },
        origin: GridPos { x: 10, z: 10 },
        navigation_position: placement_to_navigation_centre(GridPos { x: 10, z: 10 }),
        path: vec![GridPos { x: 10, z: 10 }],
        path_index: 1,
        target: GridPos { x: 10, z: 10 },
        action_cooldown_seconds: 0.75,
        action_started: true,
        repath_remaining_seconds: 0.0,
        health_regen_accumulator: 0.0,
        wander_sequence: 0,
        previous_wander_origin: None,
    };

    for (role, goal, expected) in [
        (
            "role:logger",
            AgentGoal::Gather(StableId::new("resource:test").unwrap()),
            "WoodCutting",
        ),
        (
            "role:miner",
            AgentGoal::Gather(StableId::new("resource:test").unwrap()),
            "Mining",
        ),
        (
            "role:builder",
            AgentGoal::Construct(StableId::new("building:test").unwrap()),
            "Build",
        ),
        (
            "role:ranger",
            AgentGoal::Attack(StableId::new("actor:enemy").unwrap()),
            "BowShoot",
        ),
        (
            "role:priest",
            AgentGoal::Heal(StableId::new("actor:injured").unwrap()),
            "Heal",
        ),
    ] {
        simulation
            .assign_role(&actor_id, StableId::new(role).unwrap())
            .unwrap();
        assert_eq!(
            agent_action_animation(&content, &agent_for(goal), &simulation.actors[&actor_id])
                .as_deref(),
            Some(expected)
        );
    }

    let mut moving = agent_for(AgentGoal::Gather(StableId::new("resource:test").unwrap()));
    moving.path_index = 0;
    moving.path[0] = GridPos { x: 11, z: 10 };
    assert_eq!(
        agent_action_animation(&content, &moving, &simulation.actors[&actor_id]),
        None
    );
    simulation
        .assign_role(&actor_id, StableId::new("role:miner").unwrap())
        .unwrap();
    let mut between_mining_ticks =
        agent_for(AgentGoal::Gather(StableId::new("resource:test").unwrap()));
    between_mining_ticks.action_cooldown_seconds = 0.0;
    between_mining_ticks.action_started = false;
    assert_eq!(
        agent_action_animation(
            &content,
            &between_mining_ticks,
            &simulation.actors[&actor_id]
        )
        .as_deref(),
        Some("Mining"),
        "the Unity action state must survive Bevy's per-tick replanning gap"
    );
    assert_eq!(deterministic_animation_variant(&actor_id, "BowShoot", 1), 0);
    assert_eq!(
        deterministic_animation_variant(&actor_id, "BowShoot", 4),
        deterministic_animation_variant(&actor_id, "BowShoot", 4)
    );
}

#[test]
fn animation_events_fire_once_per_elapsed_clip_cycle() {
    assert_eq!(animation_event_occurrences(0.25, 1.0, None, 0.24), 0);
    assert_eq!(animation_event_occurrences(0.25, 1.0, None, 0.25), 1);
    assert_eq!(animation_event_occurrences(0.25, 1.0, Some(0.25), 1.24), 0);
    assert_eq!(animation_event_occurrences(0.25, 1.0, Some(1.24), 1.25), 1);
    assert_eq!(animation_event_occurrences(0.25, 1.0, Some(0.10), 3.10), 3);
    assert_eq!(animation_event_occurrences(0.0, 1.0, None, 0.0), 1);
    assert_eq!(animation_event_occurrences(0.25, 0.0, None, 1.0), 0);
}

#[test]
fn fishing_carry_events_toggle_only_inside_the_authored_catch_window() {
    let mut visible = false;
    assert!(apply_transient_carry_event("ToggleOn", 0, &mut visible));
    assert!(!visible);
    assert!(apply_transient_carry_event("ToggleOn", 1, &mut visible));
    assert!(visible);
    assert!(apply_transient_carry_event("ToggleOff", 1, &mut visible));
    assert!(!visible);
    assert!(!apply_transient_carry_event(
        "PlayRoleActionAudio",
        1,
        &mut visible
    ));
}

#[test]
fn locomotion_velocity_grace_prevents_one_frame_run_restart() {
    let delta_seconds = 1.0 / 60.0;
    let cell_size = 2.0;
    let frame_step = 4.0 * cell_size * delta_seconds;
    let mut locomotion = AgentLocomotion::default();
    update_agent_locomotion_sample(
        &mut locomotion,
        Vec3::ZERO,
        delta_seconds,
        cell_size,
        0.8,
        true,
    );
    update_agent_locomotion_sample(
        &mut locomotion,
        Vec3::X * frame_step,
        delta_seconds,
        cell_size,
        0.8,
        false,
    );
    assert!((locomotion.normalized_speed - 0.8).abs() < 1.0e-5);

    // A route handoff can produce one stationary simulation frame. It must
    // not stop the active run node and restart it at time zero.
    update_agent_locomotion_sample(
        &mut locomotion,
        Vec3::X * frame_step,
        delta_seconds,
        cell_size,
        0.8,
        false,
    );
    assert!((locomotion.normalized_speed - 0.8).abs() < 1.0e-5);
    update_agent_locomotion_sample(
        &mut locomotion,
        Vec3::X * frame_step * 2.0,
        delta_seconds,
        cell_size,
        0.8,
        false,
    );
    assert!((locomotion.normalized_speed - 0.8).abs() < 1.0e-5);

    for _ in 0..10 {
        update_agent_locomotion_sample(
            &mut locomotion,
            Vec3::X * frame_step * 2.0,
            delta_seconds,
            cell_size,
            0.8,
            false,
        );
    }
    assert!(locomotion.normalized_speed.abs() < f32::EPSILON);
}

#[test]
fn converted_role_audio_events_use_source_guided_spatial_procedural_cues() {
    let presentation = embedded_presentation();
    let clips: Vec<_> = presentation
        .clips
        .iter()
        .filter(|(_, clip)| {
            clip.events
                .iter()
                .any(|event| event.function_name == "PlayRoleActionAudio")
        })
        .collect();
    assert_eq!(clips.len(), 10);
    assert_eq!(presentation.role_action_audio.len(), 14);
    assert_eq!(
        presentation
            .role_action_audio
            .values()
            .map(|audio| audio.clip_guids.len())
            .sum::<usize>(),
        35
    );
    let actor = StableId::new("actor:audio-test").unwrap();
    let role = StableId::new("role:miner").unwrap();
    let (clip_id, clip) = clips[0];
    let guid = role_action_audio_variant(&presentation, &actor, &role, clip_id).unwrap();
    assert_eq!(
        guid,
        role_action_audio_variant(&presentation, &actor, &role, clip_id).unwrap()
    );
    let wav = procedural_role_action_wav(
        &role,
        &clip.display_name,
        guid,
        PROCEDURAL_AUDIO_SAMPLE_RATE,
    );
    assert_eq!(&wav[0..4], b"RIFF");
    assert_eq!(&wav[8..12], b"WAVE");
    assert_eq!(
        wav,
        procedural_role_action_wav(
            &role,
            &clip.display_name,
            guid,
            PROCEDURAL_AUDIO_SAMPLE_RATE,
        )
    );
    let samples: Vec<_> = wav[44..]
        .chunks_exact(2)
        .map(|bytes| i16::from_le_bytes([bytes[0], bytes[1]]))
        .collect();
    assert!(samples.iter().any(|sample| sample.abs() > 512));
    assert!(i32::from(*samples.last().unwrap()).abs() < 256);
    assert!(
        samples
            .windows(2)
            .map(|pair| (i32::from(pair[1]) - i32::from(pair[0])).abs())
            .max()
            .unwrap_or_default()
            < 12_000,
        "role cue contains a static-like discontinuity"
    );
}

#[test]
fn fish_school_uses_authored_spawn_volume_noise_and_velocity_alignment() {
    let presentation = embedded_presentation();
    let effect = presentation.fish_school_effects.values().next().unwrap();
    assert!(
        effect
            .shape_scale
            .into_iter()
            .zip([300.0, 300.0, 5.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert_eq!(
        presentation
            .scene_fish_schools
            .values()
            .map(Vec::len)
            .sum::<usize>(),
        2
    );
    assert!(effect.start_speed.abs() < f32::EPSILON);
    assert!(
        effect
            .shape_rotation_degrees
            .into_iter()
            .zip([-90.0, 0.0, 0.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert!((effect.noise_scroll_speed - 1.0).abs() < f32::EPSILON);
    assert!((effect.noise_position_amount - 2.0).abs() < f32::EPSILON);
    assert_eq!(effect.noise_octaves, 2);
    assert!(effect.align_to_velocity);
    assert_eq!(
        presentation.scene_fish_schools[WORLD_SCENE_PATH]
            .iter()
            .map(|binding| binding.hierarchy_path.as_str())
            .collect::<Vec<_>>(),
        ["Fish"]
    );
    let particle = FishSchoolParticle {
        base_position: Vec3::new(2.0, -2.0, 3.0),
        noise_amplitude: Vec3::new(20.0, 0.04, 20.0),
        phase: Vec3::new(0.4, 1.7, 2.8),
        frequency: 0.22,
        scroll_speed: 1.0,
        octaves: 2,
        octave_multiplier: 1.0,
        octave_scale: 2.0,
        align_to_velocity: true,
    };
    for elapsed in [0.0, 1.0, 30.0, 120.0] {
        let transform = fish_school_transform(&particle, elapsed);
        assert_eq!(
            transform,
            fish_school_transform(&particle, elapsed),
            "fish transform must be deterministic"
        );
        let offset = transform.translation - particle.base_position;
        assert!(offset.x.abs() <= particle.noise_amplitude.x);
        assert!(offset.y.abs() <= particle.noise_amplitude.y);
        assert!(offset.z.abs() <= particle.noise_amplitude.z);
        let next = fish_school_transform(&particle, elapsed + 0.001);
        let movement = (next.translation - transform.translation).normalize();
        // Primitive extraction bypasses the imported node; its raw +Y nose
        // must still face the instantaneous velocity after correction.
        let model_forward = transform.rotation * Vec3::Y;
        assert!(model_forward.dot(movement) > 0.999);
    }
    let one_second = fish_school_transform(&particle, 1.0).translation
        - fish_school_transform(&particle, 0.0).translation;
    assert!(
        one_second.length() < 15.0,
        "fish movement must remain smooth"
    );

    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world_with_content(&config.world, &content);
    let water_height = f32::from(config.world.water_level_centimetres) * 0.01;
    let land = (0..config.world.height)
        .flat_map(|z| (0..config.world.width).map(move |x| GridPos { x, z }))
        .find(|position| terrain_height(&world, *position) > water_height + 0.05)
        .expect("generated world contains land");
    assert!(fish_school_intersects_generated_land(
        grid_to_world(land, &config),
        &world,
        &config,
        water_height,
    ));
}

#[test]
fn procedural_ambience_wav_is_valid_deterministic_and_seamless() {
    let ambience = procedural_ambience_wav(8_000, 1.0);
    assert_eq!(&ambience[0..4], b"RIFF");
    assert_eq!(&ambience[8..12], b"WAVE");
    assert_eq!(ambience.len(), 44 + 8_000 * 2);
    assert_eq!(ambience, procedural_ambience_wav(8_000, 1.0));
    let ambience_samples: Vec<_> = ambience[44..]
        .chunks_exact(2)
        .map(|bytes| i16::from_le_bytes([bytes[0], bytes[1]]))
        .collect();
    let peak = ambience_samples
        .iter()
        .map(|sample| i32::from(*sample).abs())
        .max()
        .unwrap_or_default();
    let maximum_step = ambience_samples
        .windows(2)
        .map(|samples| (i32::from(samples[1]) - i32::from(samples[0])).abs())
        .max()
        .unwrap_or_default();
    let loop_step = (i32::from(ambience_samples[0])
        - i32::from(*ambience_samples.last().expect("ambience has samples")))
    .abs();
    assert!(peak > 512, "ambience must remain audible");
    assert!(maximum_step < 128, "ambience contains static-like jumps");
    assert!(loop_step < 128, "ambience loop contains an audible seam");
}

#[test]
fn seagull_flight_preserves_shipping_prefab_bounds_and_is_deterministic() {
    for serial in 0..64 {
        let (start, end) = deterministic_seagull_leg(0x5eed_2026, serial);
        assert_eq!(start.y.to_bits(), SEAGULL_HEIGHT.to_bits());
        assert_eq!(end.y.to_bits(), SEAGULL_HEIGHT.to_bits());
        assert_eq!(start.x.to_bits(), (-end.x).to_bits());
        assert_eq!(start.z.to_bits(), (-end.z).to_bits());
        assert!(start.x.abs() <= 202.5);
        assert!(start.z.abs() <= 202.5);
        assert_eq!((start, end), deterministic_seagull_leg(0x5eed_2026, serial));
    }

    let position = Vec3::new(-12.0, SEAGULL_HEIGHT, 8.0);
    let target = Vec3::new(15.0, SEAGULL_HEIGHT, -21.0);
    let flight = seagull_flight_transform(position, target);
    let model = seagull_model_transform();
    let visible_forward = flight.rotation * model.rotation * Vec3::X;
    assert!(
        visible_forward.dot((target - position).normalize()) > 0.999,
        "the visible bird, not merely its parent entity, must face its flight direction"
    );
    assert!((model.rotation * Vec3::X).dot(Vec3::Z) > 0.999);
}

#[test]
fn seagull_calls_preserve_unity_cadence_variants_and_rolloff() {
    let mut variants = BTreeSet::new();
    for serial in 0..64 {
        let wait = deterministic_seagull_call_wait(17, serial);
        assert!((1.0..=5.0).contains(&wait));
        assert_eq!(
            wait.to_bits(),
            deterministic_seagull_call_wait(17, serial).to_bits()
        );
        variants.insert(deterministic_seagull_call_variant(17, serial));
    }
    assert_eq!(variants, BTreeSet::from([0, 1, 2]));
    assert_eq!(unity_seagull_rolloff(0.0).to_bits(), 1.0_f32.to_bits());
    assert_eq!(unity_seagull_rolloff(5.0).to_bits(), 0.5_f32.to_bits());
    assert_eq!(unity_seagull_rolloff(50.0).to_bits(), 0.0_f32.to_bits());
    assert_eq!(unity_seagull_rolloff(500.0).to_bits(), 0.0_f32.to_bits());
}

#[test]
fn procedural_seagull_calls_are_valid_distinct_wav_sources() {
    let calls: Vec<_> = (0..3)
        .map(|variant| procedural_seagull_call_wav(variant, 8_000))
        .collect();
    assert!(calls.iter().all(|call| &call[0..4] == b"RIFF"));
    assert!(calls.iter().all(|call| &call[8..12] == b"WAVE"));
    assert_ne!(calls[0], calls[1]);
    assert_ne!(calls[1], calls[2]);
    assert_eq!(calls[0], procedural_seagull_call_wav(0, 8_000));
}

#[test]
fn procedural_audio_matches_curated_acceptance_baseline() {
    let actual = audio_acceptance_manifest();
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/acceptance/audio-baseline.json");
    let expected: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(actual, expected);
    let clips = audio_acceptance_wavs();
    assert_eq!(clips.len(), 39);
    for (name, wav) in clips {
        let record = audio_acceptance_record(&wav);
        assert!(
            record["peak"].as_i64().unwrap() > 512,
            "{name} is inaudible"
        );
        let maximum_step = record["maximum_step"].as_i64().unwrap();
        let limit = if name == "ambience" { 128 } else { 12_000 };
        assert!(
            maximum_step < limit,
            "{name} contains a static-like discontinuity"
        );
    }
}

#[test]
fn shoreline_acceptance_camera_frames_water_and_land_from_the_ocean() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world_with_content(&config.world, &content);
    let focus = shoreline_focus(&world, &config);
    let centre = GridPos {
        x: world.navigation.width() / 2,
        z: world.navigation.height() / 2,
    };
    let centre_world = grid_to_world_on_surface(centre, &config, &world);
    let inward = Vec3::new(centre_world.x - focus.x, 0.0, centre_world.z - focus.z).normalize();
    let boundary_world = focus - inward * config.world.cell_size * 2.5;
    let boundary = world_to_grid(boundary_world, &config).expect("shoreline is on the grid");
    let water_height = f32::from(config.world.water_level_centimetres) * 0.01;
    assert!(terrain_height(&world, boundary) <= water_height);
    assert!(
        [
            boundary
                .x
                .checked_sub(1)
                .map(|x| GridPos { x, z: boundary.z }),
            (boundary.x + 1 < world.navigation.width()).then_some(GridPos {
                x: boundary.x + 1,
                z: boundary.z,
            }),
            boundary
                .z
                .checked_sub(1)
                .map(|z| GridPos { x: boundary.x, z }),
            (boundary.z + 1 < world.navigation.height()).then_some(GridPos {
                x: boundary.x,
                z: boundary.z + 1,
            }),
        ]
        .into_iter()
        .flatten()
        .any(|neighbor| terrain_height(&world, neighbor) > water_height),
        "acceptance focus must be derived from the generated water/land boundary"
    );

    let camera = shoreline_camera_transform(&world, &config);
    let horizontal_camera_offset = Vec3::new(
        camera.translation.x - focus.x,
        0.0,
        camera.translation.z - focus.z,
    );
    assert!(horizontal_camera_offset.dot(inward) < -20.0);
    let to_focus = (focus + Vec3::Y - camera.translation).normalize();
    assert!(camera.forward().dot(to_focus) > 0.999);
}

#[test]
fn converted_state_crossfade_preserves_weights_and_finishes_at_destination() {
    let source = AnimationNodeIndex::new(1);
    let destination = AnimationNodeIndex::new(2);
    let mut crossfade = Some(ConvertedAnimationCrossfade {
        source: vec![ConvertedAnimationPlayback {
            node: source,
            weight: 1.0,
            speed: 1.0,
            looping: true,
        }],
        elapsed: 0.0,
        duration: 0.25,
    });
    let desired = vec![ConvertedAnimationPlayback {
        node: destination,
        weight: 1.0,
        speed: 2.0,
        looping: false,
    }];
    let half = advance_animation_crossfade(&mut crossfade, &desired, 0.125);
    assert_eq!(half.len(), 2);
    assert!((half[0].weight - 0.5).abs() < f32::EPSILON);
    assert!((half[1].weight - 0.5).abs() < f32::EPSILON);
    let finished = advance_animation_crossfade(&mut crossfade, &desired, 0.125);
    assert_eq!(finished, desired);
    assert!(crossfade.is_none());
}

#[test]
fn converted_playback_preserves_authored_loop_modes() {
    let looping = AnimationNodeIndex::new(1);
    let one_shot = AnimationNodeIndex::new(2);
    let mut player = AnimationPlayer::default();
    apply_animation_blend(
        &mut player,
        &[
            ConvertedAnimationPlayback {
                node: looping,
                weight: 0.75,
                speed: 1.0,
                looping: true,
            },
            ConvertedAnimationPlayback {
                node: one_shot,
                weight: 0.25,
                speed: 1.0,
                looping: false,
            },
        ],
        &[],
    );

    assert_eq!(
        player.animation(looping).unwrap().repeat_mode(),
        RepeatAnimation::Forever
    );
    assert_eq!(
        player.animation(one_shot).unwrap().repeat_mode(),
        RepeatAnimation::Never
    );
}

#[test]
fn in_game_sun_lights_camera_facing_surfaces_with_authored_probe_compensation() {
    let sun = in_game_sun_transform();
    let camera = default_town_camera_transform();
    assert!(sun.translation.x < 0.0);
    assert!(sun.forward().dot(*camera.forward()) > 0.7);
    assert!((in_game_ambient_brightness(90.0) - 117.0).abs() < 0.001);
}

#[test]
fn converted_crossfade_uses_fixed_or_normalized_authored_duration() {
    let source = AnimationNodeIndex::new(1);
    let presentation = embedded_presentation();
    let controller = presentation.controllers.values().next().unwrap();
    let mut layer = ConvertedAnimationLayerDriver {
        fallback_state: StableId::new("state:fallback").unwrap(),
        runtime: AnimationControllerRuntime::in_state(
            controller,
            controller.default_states[0].clone(),
        )
        .unwrap(),
        nodes: BTreeMap::new(),
        active: Vec::new(),
        applied: vec![ConvertedAnimationPlayback {
            node: source,
            weight: 1.0,
            speed: 1.0,
            looping: true,
        }],
        crossfade: None,
        state_offset: 0.0,
        event_elapsed: BTreeMap::new(),
    };
    begin_animation_crossfade(
        &mut layer,
        AnimationTransitionPlayback {
            duration: 0.25,
            fixed_duration: false,
            destination_offset: 0.2,
        },
        2.0,
    );
    assert!((layer.crossfade.as_ref().unwrap().duration - 0.5).abs() < f32::EPSILON);
    assert!((layer.state_offset - 0.2).abs() < f32::EPSILON);
    begin_animation_crossfade(
        &mut layer,
        AnimationTransitionPlayback {
            duration: 0.25,
            fixed_duration: true,
            destination_offset: 0.0,
        },
        2.0,
    );
    assert!((layer.crossfade.as_ref().unwrap().duration - 0.25).abs() < f32::EPSILON);
}

#[test]
fn cosmetic_nodes_preserve_unity_order_and_visibility_rules() {
    for (index, name) in EYE_NODES.iter().enumerate() {
        assert_eq!(
            cosmetic_node(name),
            Some((CosmeticNodeKind::Eyes, u8::try_from(index).unwrap()))
        );
    }
    for (index, name) in HAIR_NODES.iter().enumerate() {
        assert_eq!(
            cosmetic_node(name),
            Some((CosmeticNodeKind::Hair, u8::try_from(index).unwrap()))
        );
    }
    for (index, name) in FACIAL_HAIR_NODES.iter().enumerate() {
        assert_eq!(
            cosmetic_node(name),
            Some((CosmeticNodeKind::FacialHair, u8::try_from(index).unwrap()))
        );
    }
    assert_eq!(cosmetic_node("Helmet_Defender"), None);

    let customization = ActorCustomization {
        eyes: 3,
        hair: 6,
        facial_hair: 1,
        hair_color: 4,
        eye_color: 1,
        skin_color: 3,
        body_type: 2,
        ..ActorCustomization::default()
    };
    assert!(cosmetic_node_visible(
        customization,
        CosmeticNodeKind::Eyes,
        3,
        false
    ));
    assert!(!cosmetic_node_visible(
        customization,
        CosmeticNodeKind::Eyes,
        2,
        false
    ));
    assert!(cosmetic_node_visible(
        customization,
        CosmeticNodeKind::Hair,
        6,
        false
    ));
    assert!(!cosmetic_node_visible(
        customization,
        CosmeticNodeKind::Hair,
        6,
        true
    ));
    assert!(cosmetic_node_visible(
        customization,
        CosmeticNodeKind::FacialHair,
        1,
        true
    ));
    assert_eq!(
        cosmetic_color(customization, CosmeticNodeKind::Hair),
        (4, HAIR_COLORS[4])
    );
    assert_eq!(
        cosmetic_color(customization, CosmeticNodeKind::FacialHair),
        (4, HAIR_COLORS[4])
    );
    assert_eq!(
        cosmetic_color(customization, CosmeticNodeKind::Eyes),
        (1, EYE_COLORS[1])
    );
    assert_eq!(
        cosmetic_color(customization, CosmeticNodeKind::Skin),
        (3, SKIN_COLORS[3])
    );
}

#[test]
fn role_driven_resource_loop_depletes_and_deposits() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let resource = world
        .resources
        .first()
        .expect("default world contains resources")
        .clone();
    let role = match resource.kind.as_str() {
        "resource:wood" => "role:logger",
        "resource:ore" => "role:miner",
        _ => "role:gatherer",
    };
    let actor_id = StableId::new("npc:test_worker").unwrap();
    let approach = resource_approach(&world, &resource, resource.position).unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), approach));
    simulation
        .assign_role(&actor_id, StableId::new(role).unwrap())
        .unwrap();
    let starting_amount = resource.amount;
    let gather = AgentGoal::Gather(resource.id.clone());
    for _ in 0..10 {
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &gather,
            approach,
        );
    }
    assert_eq!(
        world
            .resources
            .iter()
            .find(|candidate| candidate.id == resource.id)
            .unwrap()
            .amount,
        starting_amount - 10
    );
    assert_eq!(simulation.actors[&actor_id].inventory[&resource.kind], 10);
    assert_eq!(role_progress(&simulation.actors[&actor_id]).experience, 10);
    let (goal, _) = next_agent_goal(&simulation, &world, &config, &content, &actor_id, approach);
    assert_eq!(goal, AgentGoal::Deposit);
    complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &actor_id,
        &AgentGoal::Deposit,
        approach,
    );
    assert!(simulation.actors[&actor_id].inventory.is_empty());
    assert_eq!(simulation.town_resources[&resource.kind], 10);
    assert_eq!(
        role_progress(&simulation.actors[&actor_id]).experience,
        10,
        "Unity's standalone deposit state does not award role experience"
    );
}

#[test]
fn gathering_clamps_inventory_after_the_full_authored_action() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let wood = StableId::new("resource:wood").unwrap();
    let resource = world
        .resources
        .iter()
        .find(|resource| resource.kind == wood && resource.amount >= 2)
        .unwrap()
        .clone();
    let approach = resource_approach(&world, &resource, resource.position).unwrap();
    let actor_id = StableId::new("npc:partial_capacity_logger").unwrap();
    let logger = StableId::new("role:logger").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), approach));
    simulation.assign_role(&actor_id, logger.clone()).unwrap();
    simulation
        .actors
        .get_mut(&actor_id)
        .unwrap()
        .role_progression
        .insert(
            logger,
            stream_town_domain::RoleProgress {
                level: 5,
                experience: 0,
            },
        );
    simulation.gather(&actor_id, wood.clone(), 17).unwrap();
    let starting_resource = resource.amount;

    complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &actor_id,
        &AgentGoal::Gather(resource.id.clone()),
        approach,
    );

    assert_eq!(simulation.actors[&actor_id].inventory[&wood], 18);
    assert_eq!(
        world
            .resources
            .iter()
            .find(|candidate| candidate.id == resource.id)
            .unwrap()
            .amount,
        starting_resource - 3,
        "Unity takes the full action amount before PlayerInventory clamps the stored result"
    );
    assert_eq!(
        role_progress(&simulation.actors[&actor_id]).experience,
        3,
        "Unity awards BaseActionAmount rather than only the amount that fit"
    );
}

#[test]
fn role_switch_keeps_independent_inventory_capacity_and_deposits_current_resource_only() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let actor_id = StableId::new("npc:role_switch_inventory").unwrap();
    let logger = StableId::new("role:logger").unwrap();
    let miner = StableId::new("role:miner").unwrap();
    let wood = StableId::new("resource:wood").unwrap();
    let ore = StableId::new("resource:ore").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), GridPos { x: 1, z: 1 }));
    simulation.assign_role(&actor_id, logger.clone()).unwrap();
    simulation.gather(&actor_id, wood.clone(), 50).unwrap();
    simulation.assign_role(&actor_id, miner.clone()).unwrap();

    let miner_capacity = effective_role_stats(&content, &simulation, &simulation.actors[&actor_id])
        .unwrap()
        .carry_capacity;
    assert_eq!(
        actor_remaining_carry_capacity(&content, &simulation, &simulation.actors[&actor_id]),
        miner_capacity,
        "wood retained from the logger role must not consume ore capacity"
    );
    assert!(!actor_carries_role_resource(
        &content,
        &simulation.actors[&actor_id]
    ));

    simulation.gather(&actor_id, ore.clone(), 3).unwrap();
    assert_eq!(
        actor_remaining_carry_capacity(&content, &simulation, &simulation.actors[&actor_id]),
        miner_capacity.saturating_sub(3)
    );
    assert!(actor_carries_role_resource(
        &content,
        &simulation.actors[&actor_id]
    ));

    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &AgentGoal::Deposit,
            GridPos { x: 1, z: 1 },
        )
        .is_none()
    );
    assert_eq!(simulation.town_resources[&ore], 3);
    assert_eq!(simulation.actors[&actor_id].inventory[&wood], 50);
    assert!(!simulation.actors[&actor_id].inventory.contains_key(&ore));
    assert!(!actor_carries_role_resource(
        &content,
        &simulation.actors[&actor_id]
    ));

    simulation.assign_role(&actor_id, logger).unwrap();
    assert!(actor_carries_role_resource(
        &content,
        &simulation.actors[&actor_id]
    ));
}

#[test]
fn native_world_compatibility_accepts_current_and_checksum_valid_prior_generators() {
    let world = generate_world(&GameConfig::default().world);
    assert_eq!(
        native_world_compatibility(
            world.seed,
            world.generator_version,
            &world.deterministic_hash,
            &world,
        ),
        Some(NativeWorldCompatibility::Current)
    );
    assert_eq!(
        native_world_compatibility(
            world.seed,
            1,
            &stream_town_domain::legacy_v1_world_hash(&world),
            &world,
        ),
        Some(NativeWorldCompatibility::UpgradeV1)
    );
    assert_eq!(
        native_world_compatibility(
            world.seed,
            2,
            &stream_town_domain::legacy_v2_world_hash(&world),
            &world,
        ),
        Some(NativeWorldCompatibility::UpgradeV2)
    );
    assert_eq!(
        native_world_compatibility(
            world.seed,
            3,
            &stream_town_domain::legacy_v3_world_hash(&world),
            &world,
        ),
        Some(NativeWorldCompatibility::UpgradeV3)
    );
    assert_eq!(
        native_world_compatibility(world.seed, 1, "corrupt", &world),
        None
    );
    assert_eq!(
        native_world_compatibility(world.seed, 4, "validated-v4-fingerprint", &world),
        Some(NativeWorldCompatibility::RegeneratePrior)
    );
    assert_eq!(
        native_world_compatibility(world.seed, 5, "validated-v5-fingerprint", &world),
        Some(NativeWorldCompatibility::RegeneratePrior)
    );
    assert_eq!(
        native_world_compatibility(world.seed, 6, "validated-v6-fingerprint", &world),
        Some(NativeWorldCompatibility::RegeneratePrior)
    );
    assert_eq!(
        native_world_compatibility(
            world.seed.wrapping_add(1),
            world.generator_version,
            &world.deterministic_hash,
            &world,
        ),
        None
    );
}

#[test]
fn prior_native_resource_stock_scales_once_for_long_lived_land_nodes() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world_with_content(&config.world, &content);
    for resource in &world.resources {
        let expected = match resource.target_kind.as_str() {
            "target:tree" => 500,
            "target:ore" | "target:bush" => 10_000,
            "target:fish" => 100,
            kind => panic!("unexpected resource target {kind}"),
        };
        assert_eq!(
            upgraded_resource_remaining(6, resource, 100),
            expected,
            "{} should be upgraded from a version-6 save",
            resource.target_kind
        );
        assert_eq!(
            upgraded_resource_remaining(7, resource, expected),
            expected,
            "{} must not be upgraded twice",
            resource.target_kind
        );
        assert_eq!(upgraded_resource_remaining(6, resource, 0), 0);
    }
}

#[test]
fn food_roles_only_select_their_authored_target_types() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let bush = world
        .resources
        .iter()
        .find(|resource| resource.target_kind.as_str() == "target:bush")
        .unwrap();
    let fish = world
        .resources
        .iter()
        .find(|resource| resource.target_kind.as_str() == "target:fish")
        .unwrap();

    for (serial, role, expected_target, rejected_target) in [
        (0, "role:gatherer", bush, fish),
        (1, "role:fisher", fish, bush),
    ] {
        let actor_id = StableId::new(format!("npc:typed_food_{serial}")).unwrap();
        let current = resource_approach(&world, expected_target, expected_target.position)
            .expect("generated resource has an approach");
        let mut simulation = WorldSimulation::new(world.seed);
        assert!(simulation.join_player(actor_id.clone(), current));
        simulation
            .assign_role(&actor_id, StableId::new(role).unwrap())
            .unwrap();
        let (goal, _) = next_agent_goal(&simulation, &world, &config, &content, &actor_id, current);
        let compatible = compatible_target_ids(
            &content,
            &simulation,
            &world,
            &config,
            &simulation.actors[&actor_id],
        );
        let AgentGoal::Gather(selected) = goal else {
            panic!("food worker did not select a generated resource");
        };
        assert!(compatible.contains(&selected));
        assert_eq!(
            world
                .resources
                .iter()
                .find(|resource| resource.id == selected)
                .map(|resource| &resource.target_kind),
            Some(&expected_target.target_kind),
        );
        assert!(!compatible.contains(&rejected_target.id));
    }
}

#[test]
fn generated_resource_reservations_are_exclusive_and_fail_over() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let starting_resource = world
        .resources
        .iter()
        .find(|resource| resource.target_kind.as_str() == "target:tree")
        .unwrap();
    let current = starting_resource.position;
    let first = StableId::new("npc:reservation_first").unwrap();
    let second = StableId::new("npc:reservation_second").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    for actor in [&first, &second] {
        assert!(simulation.join_player(actor.clone(), current));
        simulation
            .assign_role(actor, StableId::new("role:logger").unwrap())
            .unwrap();
    }
    let AgentGoal::Gather(claimed) = next_agent_goal_with_reservations(
        &simulation,
        &world,
        &config,
        &content,
        &first,
        current,
        &BTreeMap::new(),
        &BTreeMap::new(),
    )
    .0
    else {
        panic!("logger did not select a station-catalog resource");
    };
    let reservations = BTreeMap::from([(claimed.clone(), first.clone())]);
    assert_eq!(
        next_agent_goal_with_reservations(
            &simulation,
            &world,
            &config,
            &content,
            &first,
            current,
            &reservations,
            &BTreeMap::new(),
        )
        .0,
        AgentGoal::Gather(claimed.clone())
    );
    let second_goal = next_agent_goal_with_reservations(
        &simulation,
        &world,
        &config,
        &content,
        &second,
        current,
        &reservations,
        &BTreeMap::new(),
    )
    .0;
    assert!(matches!(second_goal, AgentGoal::Gather(ref id) if id != &claimed));
    assert!(reservation_available(&BTreeMap::new(), &second, &claimed));
}

#[test]
fn unreachable_resource_route_is_rejected_before_alternate_target_retry() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let mut template = world
        .resources
        .iter()
        .find(|resource| resource.target_kind.as_str() == "target:tree")
        .unwrap()
        .clone();
    let mut simulation = WorldSimulation::new(world.seed);
    let current = restored_town_hall_position(&content, &simulation, &config);
    let bad_position = GridPos {
        x: current.x + 2,
        z: current.z,
    };
    let good_position = GridPos {
        x: current.x + 5,
        z: current.z,
    };
    world.navigation = stream_town_domain::NavGrid::new(
        config.world.width,
        config.world.height,
        vec![false; usize::from(config.world.width) * usize::from(config.world.height)],
        vec![0; usize::from(config.world.width) * usize::from(config.world.height)],
    )
    .unwrap();
    world
        .navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: GridPos {
                    x: bad_position.x - 1,
                    z: bad_position.z - 1,
                },
                max: GridPos {
                    x: bad_position.x + 1,
                    z: bad_position.z + 1,
                },
            },
            true,
        )
        .unwrap();
    world
        .navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: bad_position,
                max: bad_position,
            },
            false,
        )
        .unwrap();
    template.id = StableId::new("resource:unreachable-tree").unwrap();
    template.position = bad_position;
    template.amount = 100;
    let mut reachable = template.clone();
    reachable.id = StableId::new("resource:reachable-tree").unwrap();
    reachable.position = good_position;
    world.resources = vec![template.clone(), reachable.clone()];

    let actor_id = StableId::new("npc:path-retry-logger").unwrap();
    assert!(simulation.join_player(actor_id.clone(), current));
    simulation
        .assign_role(&actor_id, StableId::new("role:logger").unwrap())
        .unwrap();
    let town_hall = StableId::new("building:townhall").unwrap();
    let tree_kind = StableId::new("target:tree").unwrap();
    let station_targets = StationTargetRuntime {
        stations: BTreeMap::from([(
            town_hall,
            CachedStationTargets {
                targets: BTreeMap::from([(
                    tree_kind,
                    vec![template.id.clone(), reachable.id.clone()],
                )]),
                ..default()
            },
        )]),
        ..default()
    };
    let (first_goal, first_target) = next_agent_goal_with_station_runtime(
        &simulation,
        &world,
        None,
        &config,
        &content,
        &station_targets,
        &actor_id,
        current,
        placement_to_navigation_centre(current),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
    );
    assert_eq!(first_goal, AgentGoal::Gather(template.id.clone()));
    assert!(
        try_agent_path(
            &world.navigation,
            &content,
            &simulation,
            &ActorKind::Player,
            current,
            first_target,
        )
        .is_none()
    );
    let retry_reservations = BTreeMap::from([(
        template.id,
        StableId::new("system:unreachable-resource").unwrap(),
    )]);
    let (retry_goal, retry_target) = next_agent_goal_with_station_runtime(
        &simulation,
        &world,
        None,
        &config,
        &content,
        &station_targets,
        &actor_id,
        current,
        placement_to_navigation_centre(current),
        &retry_reservations,
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
    );
    assert_eq!(retry_goal, AgentGoal::Gather(reachable.id));
    assert!(
        try_agent_path(
            &world.navigation,
            &content,
            &simulation,
            &ActorKind::Player,
            current,
            retry_target,
        )
        .is_some()
    );
}

#[test]
fn generated_resource_claims_respect_the_assigned_stations_authored_catalog_cap() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    world.resources.clear();
    let current = restored_town_hall_position(&content, &WorldSimulation::new(world.seed), &config);
    for offset in 1..=31_u16 {
        world.resources.push(stream_town_domain::GeneratedResource {
            id: StableId::new(format!("resource:target_window_{offset:02}")).unwrap(),
            kind: StableId::new("resource:wood").unwrap(),
            target_kind: StableId::new("target:tree").unwrap(),
            position: GridPos {
                x: current.x + offset,
                z: current.z,
            },
            offset_milli_cells: [0, 0],
            generation_occupancy: [0, 0],
            amount: 100,
        });
    }
    let actor_id = StableId::new("npc:target_window_logger").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), current));
    simulation
        .assign_role(&actor_id, StableId::new("role:logger").unwrap())
        .unwrap();
    let reservations: BTreeMap<_, _> = world
        .resources
        .iter()
        .take(30)
        .map(|resource| {
            (
                resource.id.clone(),
                StableId::new(format!(
                    "npc:claim_{}",
                    resource.id.as_str().replace(':', "_")
                ))
                .unwrap(),
            )
        })
        .collect();

    assert!(matches!(
        next_agent_goal_with_reservations(
            &simulation,
            &world,
            &config,
            &content,
            &actor_id,
            current,
            &reservations,
            &BTreeMap::new(),
        )
        .0,
        AgentGoal::Wander
    ));
    assert!(world.resources[30].id.as_str().ends_with("31"));
}

#[test]
fn stale_resource_reservations_release_after_role_change_or_depletion() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let claimed = world
        .resources
        .iter()
        .find(|resource| resource.target_kind.as_str() == "target:tree")
        .unwrap()
        .clone();
    let actor_id = StableId::new("npc:stale_reservation").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), claimed.position));
    simulation
        .assign_role(&actor_id, StableId::new("role:logger").unwrap())
        .unwrap();
    let goal = AgentGoal::Gather(claimed.id.clone());
    assert!(goal_reservation_is_valid(
        &simulation,
        &world,
        &content,
        &actor_id,
        &goal,
    ));
    simulation
        .assign_role(&actor_id, StableId::new("role:miner").unwrap())
        .unwrap();
    assert!(!goal_reservation_is_valid(
        &simulation,
        &world,
        &content,
        &actor_id,
        &goal,
    ));
    simulation
        .assign_role(&actor_id, StableId::new("role:logger").unwrap())
        .unwrap();
    world
        .resources
        .iter_mut()
        .find(|resource| resource.id == claimed.id)
        .unwrap()
        .amount = 0;
    assert!(!goal_reservation_is_valid(
        &simulation,
        &world,
        &content,
        &actor_id,
        &goal,
    ));
}

#[test]
fn land_resource_occupancy_requires_an_edge_action_and_clears_after_last_depletion() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let resource = world
        .resources
        .iter()
        .find(|resource| {
            resource.target_kind.as_str() == "target:tree"
                && !world.navigation.is_walkable(resource.position)
                && world
                    .resources
                    .iter()
                    .filter(|candidate| {
                        candidate.target_kind.as_str() != "target:fish"
                            && candidate.position == resource.position
                    })
                    .count()
                    == 1
        })
        .unwrap()
        .clone();
    let mut overlapping_resource = resource.clone();
    overlapping_resource.id = StableId::new("resource:overlapping_tree").unwrap();
    overlapping_resource.amount = 1;
    world.resources.push(overlapping_resource.clone());
    let approach = resource_approach(&world, &resource, resource.position).unwrap();
    let actor_id = StableId::new("npc:resource_edge_test").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), approach));
    simulation
        .assign_role(&actor_id, StableId::new("role:logger").unwrap())
        .unwrap();
    world
        .resources
        .iter_mut()
        .find(|candidate| candidate.id == resource.id)
        .unwrap()
        .amount = 1;
    let goal = AgentGoal::Gather(resource.id.clone());
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &goal,
            resource.position,
        )
        .is_none()
    );
    assert_eq!(
        world
            .resources
            .iter()
            .find(|candidate| candidate.id == resource.id)
            .unwrap()
            .amount,
        1
    );
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &goal,
            approach,
        )
        .is_none()
    );
    assert!(!world.navigation.is_walkable(resource.position));
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &AgentGoal::Gather(overlapping_resource.id),
            approach,
        )
        .is_none()
    );
    assert!(world.navigation.is_walkable(resource.position));
}

#[test]
fn farmer_harvests_unlimited_food_from_completed_farm() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let farm = &content.buildings[&StableId::new("building:farm").unwrap()];
    let position = find_building_site(
        &world,
        GridPos {
            x: config.world.width / 2,
            z: config.world.height / 2,
        },
        farm.footprint,
    )
    .unwrap();
    let farm_id = StableId::new("building:test_farm").unwrap();
    let region = building_region(position, farm.footprint, &world).unwrap();
    world.navigation.set_blocked(region, true).unwrap();
    let current = building_approach(&world, position, farm.footprint, position).unwrap();
    let actor_id = StableId::new("npc:farmer_test").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.buildings.insert(
        farm_id.clone(),
        BuildingState {
            id: farm_id.clone(),
            archetype: farm.archetype.clone(),
            position,
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(&content, farm)).unwrap(),
            complete: true,
        },
    );
    assert!(simulation.join_player(actor_id.clone(), current));
    simulation
        .assign_role(&actor_id, StableId::new("role:farmer").unwrap())
        .unwrap();

    let (goal, target) =
        next_agent_goal(&simulation, &world, &config, &content, &actor_id, current);
    assert_eq!(goal, AgentGoal::HarvestFarm(farm_id.clone()));
    assert_eq!(target, current);
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &goal,
            current,
        )
        .is_none()
    );
    let food = StableId::new("resource:food").unwrap();
    assert_eq!(simulation.actors[&actor_id].inventory[&food], 1);
    assert_eq!(role_progress(&simulation.actors[&actor_id]).experience, 1);
    assert!(
        compatible_target_ids(
            &content,
            &simulation,
            &world,
            &config,
            &simulation.actors[&actor_id],
        )
        .contains(&farm_id)
    );
}

#[test]
fn authored_assignment_penalty_spreads_farmers_across_farms() {
    let config = GameConfig::default();
    let content = embedded_content();
    let farm = &content.buildings[&StableId::new("building:farm").unwrap()];
    assert_eq!(
        farm.targeting,
        Some(TargetingScoreDef {
            assignment_penalty_milli: 10_000_000,
            distance_penalty_milli_per_cell: 100,
        })
    );
    let mut world = generate_world(&config.world);
    let first_position =
        find_building_site(&world, GridPos { x: 16, z: 16 }, farm.footprint).unwrap();
    world
        .navigation
        .set_blocked(
            building_region(first_position, farm.footprint, &world).unwrap(),
            true,
        )
        .unwrap();
    let second_position =
        find_building_site(&world, GridPos { x: 48, z: 48 }, farm.footprint).unwrap();
    world
        .navigation
        .set_blocked(
            building_region(second_position, farm.footprint, &world).unwrap(),
            true,
        )
        .unwrap();
    let first_id = StableId::new("building:assignment_farm_a").unwrap();
    let second_id = StableId::new("building:assignment_farm_b").unwrap();
    let actor_id = StableId::new("npc:assignment_farmer").unwrap();
    let current =
        building_approach(&world, first_position, farm.footprint, first_position).unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    for (id, position) in [
        (first_id.clone(), first_position),
        (second_id.clone(), second_position),
    ] {
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id,
                archetype: farm.archetype.clone(),
                position,
                rotation_quarter_turns: 0,
                level: 1,
                health: i32::try_from(building_base_max_health(&content, farm)).unwrap(),
                complete: true,
            },
        );
    }
    assert!(simulation.join_player(actor_id.clone(), current));
    simulation
        .assign_role(&actor_id, StableId::new("role:farmer").unwrap())
        .unwrap();

    assert_eq!(
        next_agent_goal_with_reservations(
            &simulation,
            &world,
            &config,
            &content,
            &actor_id,
            current,
            &BTreeMap::new(),
            &BTreeMap::new(),
        )
        .0,
        AgentGoal::HarvestFarm(first_id.clone()),
    );
    assert_eq!(
        next_agent_goal_with_reservations(
            &simulation,
            &world,
            &config,
            &content,
            &actor_id,
            current,
            &BTreeMap::new(),
            &BTreeMap::from([(first_id, 1)]),
        )
        .0,
        AgentGoal::HarvestFarm(second_id),
    );
}

#[test]
fn full_town_storage_pauses_gathering_and_preserves_carried_overflow() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let resource = world
        .resources
        .first()
        .expect("default world contains resources")
        .clone();
    let role = match resource.kind.as_str() {
        "resource:wood" => "role:logger",
        "resource:ore" => "role:miner",
        _ => "role:gatherer",
    };
    let actor_id = StableId::new("npc:storage_backpressure_test").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(actor_id.clone(), resource.position));
    simulation
        .assign_role(&actor_id, StableId::new(role).unwrap())
        .unwrap();
    simulation
        .actors
        .get_mut(&actor_id)
        .unwrap()
        .inventory
        .insert(resource.kind.clone(), 10);
    let capacity = resource_storage_capacity(&config, &content, &simulation, &resource.kind);
    simulation
        .town_resources
        .insert(resource.kind.clone(), capacity);
    let resource_amount = resource.amount;

    let (goal, target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &actor_id,
        resource.position,
    );
    assert_eq!(goal, AgentGoal::WaitForStorage);
    assert_eq!(
        target,
        town_hall_wait_target(&content, &simulation, &world, &config, resource.position,)
    );
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &AgentGoal::Gather(resource.id.clone()),
            resource.position,
        )
        .is_none()
    );
    assert_eq!(
        world
            .resources
            .iter()
            .find(|candidate| candidate.id == resource.id)
            .unwrap()
            .amount,
        resource_amount
    );
    complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &actor_id,
        &AgentGoal::Deposit,
        resource.position,
    );
    assert_eq!(simulation.actors[&actor_id].inventory[&resource.kind], 10);

    simulation
        .town_resources
        .insert(resource.kind.clone(), capacity - 1);
    let (goal, _) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &actor_id,
        resource.position,
    );
    assert_eq!(goal, AgentGoal::Deposit);
    complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &actor_id,
        &goal,
        resource.position,
    );
    assert_eq!(simulation.town_resources[&resource.kind], capacity);
    assert_eq!(simulation.actors[&actor_id].inventory[&resource.kind], 9);

    simulation
        .town_resources
        .insert(resource.kind.clone(), capacity - 10);
    complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &actor_id,
        &AgentGoal::Deposit,
        resource.position,
    );
    assert!(simulation.actors[&actor_id].inventory.is_empty());
    let (goal, _) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &actor_id,
        resource.position,
    );
    assert!(matches!(goal, AgentGoal::Gather(_)));
}

#[test]
fn marketplace_passive_income_is_level_scaled_and_save_stable() {
    let config = GameConfig::default();
    let content = embedded_content();
    let marketplace = &content.buildings[&StableId::new("building:marketplace").unwrap()];
    let building_id = StableId::new("building:runtime_marketplace").unwrap();
    let gold = StableId::new("resource:gold").unwrap();
    let mut simulation = WorldSimulation::new(config.world.seed);
    simulation.town_resources.insert(gold.clone(), 0);
    simulation.buildings.insert(
        building_id.clone(),
        BuildingState {
            id: building_id.clone(),
            archetype: marketplace.archetype.clone(),
            position: GridPos { x: 8, z: 8 },
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );

    apply_passive_building_income(&config, &content, &mut simulation, Duration::from_secs(2));
    assert_eq!(simulation.town_resources[&gold], 0);
    apply_passive_building_income(&config, &content, &mut simulation, Duration::from_nanos(1));
    assert_eq!(simulation.town_resources[&gold], 1);

    let encoded = ron::to_string(&simulation).unwrap();
    let mut restored: WorldSimulation = ron::from_str(&encoded).unwrap();
    assert_eq!(
        restored.passive_resource_accumulators,
        simulation.passive_resource_accumulators
    );
    restored.buildings.get_mut(&building_id).unwrap().level = 2;
    apply_passive_building_income(&config, &content, &mut restored, Duration::from_secs(1));
    assert_eq!(restored.town_resources[&gold], 2);

    restored.buildings.get_mut(&building_id).unwrap().complete = false;
    apply_passive_building_income(&config, &content, &mut restored, Duration::from_secs(10));
    assert_eq!(restored.town_resources[&gold], 2);
    assert!(restored.passive_resource_accumulators.is_empty());
}

#[test]
fn passive_income_batches_exact_elapsed_time_at_four_hertz() {
    let mut accumulated = Duration::ZERO;
    assert_eq!(
        take_passive_building_income_delta(&mut accumulated, Duration::from_millis(100)),
        None
    );
    assert_eq!(
        take_passive_building_income_delta(&mut accumulated, Duration::from_millis(149)),
        None
    );
    assert_eq!(
        take_passive_building_income_delta(&mut accumulated, Duration::from_millis(1)),
        Some(Duration::from_millis(250))
    );
    assert_eq!(accumulated, Duration::ZERO);
    assert_eq!(
        take_passive_building_income_delta(&mut accumulated, Duration::from_millis(275)),
        Some(Duration::from_millis(275))
    );
    assert_eq!(accumulated, Duration::ZERO);
}

#[test]
fn combat_goal_damages_kills_and_respawns() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let (defender_position, enemy_position) = (0..world.navigation.height())
        .find_map(|z| {
            (0..world.navigation.width().saturating_sub(1)).find_map(|x| {
                let defender = GridPos { x, z };
                let enemy = GridPos { x: x + 1, z };
                (world.navigation.is_walkable(defender) && world.navigation.is_walkable(enemy))
                    .then_some((defender, enemy))
            })
        })
        .expect("generated world has adjacent combat cells");
    let defender = StableId::new("npc:defender_test").unwrap();
    let enemy = StableId::new("actor:enemy_test").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(defender.clone(), defender_position));
    assert!(simulation.join_player(enemy.clone(), enemy_position));
    simulation
        .assign_role(&defender, StableId::new("role:defender").unwrap())
        .unwrap();
    simulation
        .assign_role(&enemy, StableId::new("role:enemy").unwrap())
        .unwrap();
    let (goal, target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &defender,
        defender_position,
    );
    assert_eq!(goal, AgentGoal::Attack(enemy.clone()));
    assert_eq!(target, defender_position);
    for _ in 0..100 {
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &defender,
            &goal,
            defender_position,
        );
    }
    assert!(!simulation.actors[&enemy].alive);
    assert_eq!(simulation.actors[&enemy].health, 0);
    simulation.respawn_actor(&enemy, enemy_position).unwrap();
    assert!(simulation.actors[&enemy].alive);
    assert_eq!(simulation.actors[&enemy].health, 100);
    let expected_cooldown =
        effective_role_stats(&content, &simulation, &simulation.actors[&defender])
            .map(|stats| milli_units_as_f32(stats.action_milliseconds))
            .unwrap();
    assert!(
        (action_cooldown(&content, &simulation, &defender, &goal) - expected_cooldown).abs()
            <= f32::EPSILON
    );
}

#[test]
fn every_combat_role_paths_to_strikes_and_kills_an_enemy() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world_with_content(&config.world, &content);
    let (start, target) = (0..world.navigation.height())
        .find_map(|z| {
            (0..world.navigation.width()).find_map(|x| {
                let start = GridPos { x, z };
                if !world.navigation.is_walkable(start) {
                    return None;
                }
                (7..=12).find_map(|offset| {
                    let target = GridPos {
                        x: x.checked_add(offset)?,
                        z,
                    };
                    (target.x < world.navigation.width() && world.navigation.is_walkable(target))
                        .then(|| world.navigation.find_path(start, target).ok())
                        .flatten()
                        .filter(|path| path.len() > 2)
                        .map(|_| (start, target))
                })
            })
        })
        .expect("generated world has a combat route");
    let goblin =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();

    for role_name in [
        "role:defender",
        "role:necromancer",
        "role:paladin",
        "role:ranger",
        "role:ruler",
        "role:soldier",
        "role:wizard",
    ] {
        let role = StableId::new(role_name).unwrap();
        let attacker =
            StableId::new(format!("npc:pathing_{}", role_name.replace(':', "_"))).unwrap();
        let enemy = StableId::new(format!("actor:target_{}", role_name.replace(':', "_"))).unwrap();
        let mut simulation = WorldSimulation::new(world.seed);
        assert!(simulation.join_player(attacker.clone(), start));
        simulation.assign_role(&attacker, role).unwrap();
        let damage = effective_role_stats(&content, &simulation, &simulation.actors[&attacker])
            .unwrap()
            .action_amount;
        assert!(damage > 0, "{role_name} has no attack damage");
        assert!(simulation.spawn_enemy(
            enemy.clone(),
            goblin.clone(),
            target,
            i32::try_from(damage).unwrap(),
        ));

        let (goal, destination) =
            next_agent_goal(&simulation, &world, &config, &content, &attacker, start);
        assert_eq!(goal, AgentGoal::Attack(enemy.clone()), "{role_name}");
        assert_ne!(destination, start, "{role_name} never requested a path");
        let routed = try_agent_path(
            &world.navigation,
            &content,
            &simulation,
            &ActorKind::Player,
            start,
            destination,
        )
        .unwrap_or_else(|| panic!("{role_name} could not route to its enemy"));
        assert!(routed.len() > 1, "{role_name} produced an empty route");
        assert_eq!(routed.last(), Some(&destination), "{role_name}");
        simulation.actors.get_mut(&attacker).unwrap().position = destination;

        let presentation = complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &attacker,
            &goal,
            destination,
        )
        .unwrap_or_else(|| panic!("{role_name} did not execute its attack"));
        if let ActionPresentation::Projectile(spawn) = presentation {
            let projectile = CombatProjectile {
                source: spawn.source,
                target: spawn.target,
                damage: spawn.damage,
                speed_cells_per_second: spawn.speed_cells_per_second,
                visual: spawn.visual,
                trail_cooldown_seconds: 0.0,
                remaining_seconds: PROJECTILE_MAX_LIFETIME_SECONDS,
            };
            resolve_combat_projectile_impact(&config, &mut simulation, &content, &projectile)
                .unwrap();
        }
        assert!(!simulation.actors[&enemy].alive, "{role_name} did not kill");
        assert_eq!(simulation.actors[&enemy].health, 0, "{role_name}");
    }
}

#[test]
fn enemy_attacks_can_kill_citizens() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world_with_content(&config.world, &content);
    let (enemy_position, citizen_position) = (0..world.navigation.height())
        .find_map(|z| {
            (0..world.navigation.width().saturating_sub(1)).find_map(|x| {
                let enemy = GridPos { x, z };
                let citizen = GridPos { x: x + 1, z };
                (world.navigation.is_walkable(enemy) && world.navigation.is_walkable(citizen))
                    .then_some((enemy, citizen))
            })
        })
        .unwrap();
    let blargul =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Blargul.prefab").unwrap();
    let damage = content.archetypes[&blargul]
        .enemy
        .as_ref()
        .unwrap()
        .action_amount;
    let enemy = StableId::new("actor:citizen_killer").unwrap();
    let citizen = StableId::new("npc:citizen_victim").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.spawn_enemy(enemy.clone(), blargul, enemy_position, 100));
    assert!(simulation.join_player(citizen.clone(), citizen_position));
    simulation.actors.get_mut(&citizen).unwrap().health = i32::try_from(damage).unwrap();
    simulation.actors.get_mut(&citizen).unwrap().max_health = i32::try_from(damage).unwrap();

    let (goal, destination) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &enemy,
        enemy_position,
    );
    assert_eq!(goal, AgentGoal::Attack(citizen.clone()));
    assert_eq!(destination, enemy_position);
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &enemy,
            &goal,
            enemy_position,
        )
        .is_some()
    );
    assert!(!simulation.actors[&citizen].alive);
    assert_eq!(simulation.actors[&citizen].health, 0);
    simulation.actors.get_mut(&citizen).unwrap().display_name = Some("Ada".to_owned());
    assert_eq!(
        citizen_death_announcement(&simulation.actors[&citizen]),
        "Ada has died! They will respawn at the Town Hall in 10 minutes."
    );
}

#[test]
fn tower_projectile_selects_hits_and_kills_an_enemy() {
    let config = GameConfig::default();
    let content = embedded_content();
    let tower_definition_id = StableId::new("building:tower").unwrap();
    let tower_definition = &content.buildings[&tower_definition_id];
    let shooter = tower_definition.projectile_shooter.as_ref().unwrap();
    let tower = StableId::new("building:tower_projectile_test").unwrap();
    let enemy = StableId::new("actor:tower_target").unwrap();
    let goblin =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();
    let mut simulation = WorldSimulation::new(42);
    simulation.buildings.insert(
        tower.clone(),
        BuildingState {
            id: tower.clone(),
            archetype: tower_definition.archetype.clone(),
            position: GridPos { x: 20, z: 20 },
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(&content, tower_definition)).unwrap(),
            complete: true,
        },
    );
    assert!(simulation.spawn_enemy(
        enemy.clone(),
        goblin,
        GridPos { x: 23, z: 20 },
        i32::try_from(shooter.damage).unwrap(),
    ));

    let (spawn, cooldown) = tower_projectile_for_building(&content, &simulation, &tower).unwrap();
    assert!(matches!(spawn.source, ProjectileSource::Building(ref id) if id == &tower));
    assert_eq!(spawn.target, enemy);
    assert_eq!(spawn.damage, shooter.damage);
    assert!((spawn.speed_cells_per_second - 15.0).abs() <= f32::EPSILON);
    assert!((cooldown - 3.0).abs() <= f32::EPSILON);
    let projectile = CombatProjectile {
        source: spawn.source,
        target: spawn.target,
        damage: spawn.damage,
        speed_cells_per_second: spawn.speed_cells_per_second,
        visual: spawn.visual,
        trail_cooldown_seconds: 0.0,
        remaining_seconds: PROJECTILE_MAX_LIFETIME_SECONDS,
    };
    assert!(
        resolve_combat_projectile_impact(&config, &mut simulation, &content, &projectile,).unwrap()
    );
    assert!(!simulation.actors[&enemy].alive);
    assert_eq!(simulation.actors[&enemy].health, 0);
}

#[test]
fn shipping_world_seeds_authored_enemy_camps_deterministically() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut first_world = generate_world(&config.world);
    let mut first = WorldSimulation::new(first_world.seed);
    ensure_town_hall_state(&content, &config, &mut first);
    let town_hall = &first.buildings[&StableId::new("building:townhall").unwrap()];
    let town_hall_definition = building_def_for_archetype(&content, &town_hall.archetype).unwrap();
    first_world
        .navigation
        .set_blocked(
            building_navigation_region(
                town_hall.position,
                town_hall_definition,
                town_hall.rotation_quarter_turns,
                &first_world,
            )
            .unwrap(),
            true,
        )
        .unwrap();
    let expected = content
        .enemy_camp_generation
        .iter()
        .map(|layer| usize::from(layer.maximum_camps))
        .sum::<usize>();
    let spawned = seed_generated_enemy_camps(&config, &content, &mut first_world, &mut first);
    assert!(spawned > 0 && spawned <= expected);
    assert_eq!(first.enemy_camps.len(), spawned);

    let mut centres = Vec::new();
    for camp in first.enemy_camps.values() {
        let archetype = &content.archetypes[&camp.archetype];
        let region =
            enemy_camp_navigation_region(camp.position, archetype.footprint, &first_world).unwrap();
        assert!(camp.spawn_remaining_seconds.abs() <= f64::EPSILON);
        assert!(!first_world.navigation.is_walkable(region.min));
        let camp_spawner = archetype.enemy_spawner.as_ref().unwrap();
        for serial in 0..camp_spawner.spawn_offsets_milli_cells.len() as u64 {
            let spawn = enemy_spawn_position(
                &first_world,
                &content,
                &first,
                None,
                camp,
                camp_spawner,
                archetype.footprint,
                serial,
            );
            assert!(first_world.navigation.is_walkable(spawn));
            assert!(!region_contains_grid_position(region, spawn));
            let (_, approach) =
                nearest_reachable_building_to_town_hall(&content, &first, &first_world, spawn)
                    .expect("camp spawn can reach a town building");
            assert!(first_world.navigation.find_path(spawn, approach).is_ok());
        }
        let centre = GridPos {
            x: camp.position.x + archetype.footprint[0] / 2,
            z: camp.position.z + archetype.footprint[1] / 2,
        };
        let layer = content
            .enemy_camp_generation
            .iter()
            .find(|layer| layer.camp_archetype == camp.archetype)
            .unwrap();
        for other in &centres {
            let distance = u128::from(grid_distance_squared(*other, centre)) * 1_000_000;
            let minimum = u128::from(layer.minimum_distance_between_camps_milli_cells);
            assert!(distance >= minimum * minimum);
        }
        centres.push(centre);
    }

    let mut second_world = generate_world(&config.world);
    let mut second = WorldSimulation::new(second_world.seed);
    ensure_town_hall_state(&content, &config, &mut second);
    second_world
        .navigation
        .set_blocked(
            building_navigation_region(
                second.buildings[&StableId::new("building:townhall").unwrap()].position,
                town_hall_definition,
                0,
                &second_world,
            )
            .unwrap(),
            true,
        )
        .unwrap();
    seed_generated_enemy_camps(&config, &content, &mut second_world, &mut second);
    assert_eq!(second.enemy_camps, first.enemy_camps);
}

#[test]
fn enemies_advance_on_town_hall_while_defenders_acquire_them() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let mut simulation = WorldSimulation::new(world.seed);
    ensure_town_hall_state(&content, &config, &mut simulation);
    let town_hall = StableId::new("building:townhall").unwrap();
    let hall_position = building_visual_grid(&content, &simulation.buildings[&town_hall]);
    let defender_position = nearest_walkable(&world, hall_position).unwrap();
    let hall_state = &simulation.buildings[&town_hall];
    let hall_definition = building_def_for_archetype(&content, &hall_state.archetype).unwrap();
    let enemy_position = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .find(|position| {
            world.navigation.is_walkable(*position)
                && grid_distance_squared(*position, hall_position) >= 100
                && within_player_target_search_region(*position, defender_position)
                && reachable_building_approach(
                    &world,
                    hall_state.position,
                    rotated_footprint(hall_definition.footprint, hall_state.rotation_quarter_turns),
                    *position,
                )
                .is_some()
        })
        .expect("test enemy can reach the Town Hall from outside attack range");
    let enemy_id = StableId::new("actor:town_assault_test").unwrap();
    let defender_id = StableId::new("npc:town_defender_test").unwrap();
    let goblin =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();
    let health = content.archetypes[&goblin]
        .health
        .as_ref()
        .unwrap()
        .max_health;
    assert!(simulation.spawn_enemy(
        enemy_id.clone(),
        goblin,
        enemy_position,
        i32::try_from(health).unwrap(),
    ));
    assert!(simulation.join_player(defender_id.clone(), defender_position));
    simulation
        .assign_role(&defender_id, StableId::new("role:defender").unwrap())
        .unwrap();

    let enemy_goal = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &enemy_id,
        enemy_position,
    );
    assert_eq!(enemy_goal.0, AgentGoal::AttackBuilding(town_hall));
    assert_ne!(enemy_goal.1, enemy_position);
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &defender_id,
            defender_position,
        )
        .0,
        AgentGoal::Attack(enemy_id),
    );
}

#[test]
fn sealed_town_hall_redirects_enemies_to_the_nearest_reachable_defense() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = GeneratedWorld {
        seed: 42,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(32, 9, vec![false; 32 * 9], vec![100; 32 * 9])
            .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    world
        .navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: GridPos { x: 20, z: 0 },
                max: GridPos { x: 20, z: 8 },
            },
            true,
        )
        .unwrap();
    let town_hall_definition = &content.buildings[&StableId::new("building:townhall").unwrap()];
    let wall_definition = &content.buildings[&StableId::new("building:wall").unwrap()];
    let path_definition = &content.buildings[&StableId::new("building:path").unwrap()];
    let town_hall = StableId::new("building:townhall").unwrap();
    let wall = StableId::new("building:wall_barrier").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.buildings.insert(
        town_hall.clone(),
        BuildingState {
            id: town_hall,
            archetype: town_hall_definition.archetype.clone(),
            position: GridPos { x: 24, z: 3 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 1_000,
            complete: true,
        },
    );
    simulation.buildings.insert(
        wall.clone(),
        BuildingState {
            id: wall.clone(),
            archetype: wall_definition.archetype.clone(),
            position: GridPos { x: 20, z: 4 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 1_000,
            complete: true,
        },
    );
    let path = StableId::new("building:path_decoy").unwrap();
    simulation.buildings.insert(
        path.clone(),
        BuildingState {
            id: path.clone(),
            archetype: path_definition.archetype.clone(),
            position: GridPos { x: 5, z: 4 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 1_000,
            complete: true,
        },
    );
    assert!(!enemy_can_attack_building(
        &content,
        &simulation.buildings[&path]
    ));
    let enemy = StableId::new("actor:sealed_town_attacker").unwrap();
    let goblin =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();
    assert!(simulation.spawn_enemy(enemy.clone(), goblin, GridPos { x: 2, z: 4 }, 100));

    let (goal, approach) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &enemy,
        GridPos { x: 2, z: 4 },
    );
    assert_eq!(goal, AgentGoal::AttackBuilding(wall));
    assert!(
        world
            .navigation
            .find_path(GridPos { x: 2, z: 4 }, approach)
            .is_ok()
    );
}

#[test]
fn coarse_building_occupancy_preserves_authored_placement_footprints() {
    let content = embedded_content();
    let world = GeneratedWorld {
        seed: 1,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(
            24,
            24,
            vec![false; 24 * 24],
            vec![100; 24 * 24],
        )
        .unwrap(),
        resources: Vec::new(),
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    let origin = GridPos { x: 8, z: 8 };
    let marketplace = &content.buildings[&StableId::new("building:marketplace").unwrap()];
    let civilian = building_navigation_region(origin, marketplace, 0, &world).unwrap();
    assert_eq!(civilian.min, origin);
    assert_eq!(civilian.max, GridPos { x: 11, z: 11 });

    let stonemason = &content.buildings[&StableId::new("building:stonemason").unwrap()];
    let drop_off = building_navigation_region(origin, stonemason, 0, &world).unwrap();
    assert_eq!(drop_off.min, origin);
    assert_eq!(drop_off.max, GridPos { x: 9, z: 9 });

    let barracks = &content.buildings[&StableId::new("building:barracks").unwrap()];
    let military = building_navigation_region(origin, barracks, 0, &world).unwrap();
    assert_eq!(military.min, origin);
    assert_eq!(military.max, GridPos { x: 10, z: 10 });

    let camp = enemy_camp_navigation_region(origin, [7, 6], &world).unwrap();
    assert_eq!(camp.min, GridPos { x: 10, z: 9 });
    assert_eq!(camp.max, GridPos { x: 12, z: 11 });
}

#[test]
fn fine_navigation_decouples_physical_obstructions_from_placement_cells() {
    let mut config = GameConfig::default();
    config.world.width = 24;
    config.world.height = 24;
    let content = embedded_content();
    assert_eq!(
        content.buildings[&StableId::new("building:foodstorage").unwrap()]
            .navigation_footprint_thirds,
        Some([5, 4])
    );
    assert_eq!(
        content.buildings[&StableId::new("building:orestorage").unwrap()]
            .navigation_footprint_thirds,
        Some([4, 4])
    );
    assert_eq!(
        content.buildings[&StableId::new("building:woodstorage").unwrap()]
            .navigation_footprint_thirds,
        Some([5, 4])
    );
    assert_eq!(
        content.buildings[&StableId::new("building:windmill").unwrap()].navigation_footprint_thirds,
        Some([7, 7])
    );
    let house = &content.buildings[&StableId::new("building:house").unwrap()];
    assert_eq!(house.footprint, [2, 3]);
    assert_eq!(house.navigation_footprint_thirds, Some([7, 7]));
    let tower = &content.buildings[&StableId::new("building:tower").unwrap()];
    assert_eq!(tower.footprint, [1, 1]);
    assert_eq!(tower.navigation_footprint_thirds, Some([4, 4]));
    let mut world = GeneratedWorld {
        seed: 7,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(
            24,
            24,
            vec![false; 24 * 24],
            vec![100; 24 * 24],
        )
        .unwrap(),
        resources: vec![stream_town_domain::GeneratedResource {
            id: StableId::new("resource:fine-grid-test").unwrap(),
            kind: StableId::new("resource:wood").unwrap(),
            target_kind: StableId::new("target:tree").unwrap(),
            position: GridPos { x: 8, z: 8 },
            offset_milli_cells: [0, 0],
            generation_occupancy: [0, 0],
            amount: 100,
        }],
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    // Fine navigation is rebuilt from terrain plus authored physical
    // footprints, not from the coarse placement occupancy.
    world
        .navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: GridPos { x: 8, z: 8 },
                max: GridPos { x: 8, z: 8 },
            },
            true,
        )
        .unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    let mut add_building = |runtime_id: &str, building_id: &str, position: GridPos| {
        let definition = &content.buildings[&StableId::new(building_id).unwrap()];
        let id = StableId::new(runtime_id).unwrap();
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id,
                archetype: definition.archetype.clone(),
                position,
                rotation_quarter_turns: 0,
                level: 1,
                health: 100,
                complete: true,
            },
        );
    };
    add_building(
        "building:runtime_house",
        "building:house",
        GridPos { x: 2, z: 2 },
    );
    add_building(
        "building:runtime_storage",
        "building:orestorage",
        GridPos { x: 2, z: 10 },
    );
    add_building(
        "building:runtime_tower",
        "building:tower",
        GridPos { x: 10, z: 2 },
    );
    add_building(
        "building:runtime_wall_a",
        "building:wall",
        GridPos { x: 15, z: 5 },
    );
    add_building(
        "building:runtime_wall_b",
        "building:wall",
        GridPos { x: 16, z: 5 },
    );
    add_building(
        "building:runtime_gate",
        "building:gate",
        GridPos { x: 17, z: 5 },
    );

    let navigation = build_fine_navigation(&config, &content, &simulation, &world).unwrap();
    assert_eq!((navigation.width(), navigation.height()), (72, 72));

    for (runtime_id, building_id) in [
        ("building:runtime_house", "building:house"),
        ("building:runtime_storage", "building:orestorage"),
        ("building:runtime_tower", "building:tower"),
    ] {
        let runtime_id = StableId::new(runtime_id).unwrap();
        let building_id = StableId::new(building_id).unwrap();
        let state = &simulation.buildings[&runtime_id];
        let definition = &content.buildings[&building_id];
        let physical =
            building_fine_navigation_cells(&content, &simulation, state, &building_id, definition);
        assert!(
            physical.iter().all(|cell| !navigation.is_walkable(*cell)),
            "{building_id} physical footprint must be blocked"
        );
        let placement =
            building_fine_placement_cells(&content, &simulation, state, &building_id, definition);
        assert!(
            placement
                .iter()
                .any(|cell| { !physical.contains(cell) && navigation.is_walkable(*cell) })
        );
    }
    // A resource still blocks only its central fine cell.
    assert!(!navigation.is_walkable(GridPos { x: 25, z: 25 }));
    assert!(navigation.is_walkable(GridPos { x: 24, z: 25 }));

    let gate_cells = completed_player_gate_navigation_cells(&content, &simulation);
    assert_eq!(gate_cells.len(), 3);
    assert!(
        gate_cells
            .iter()
            .all(|position| !navigation.is_walkable(*position))
    );
    let wall_a = linear_navigation_cells(
        &content,
        &simulation,
        &simulation.buildings[&StableId::new("building:runtime_wall_a").unwrap()],
        &StableId::new("building:wall").unwrap(),
    );
    let wall_b = linear_navigation_cells(
        &content,
        &simulation,
        &simulation.buildings[&StableId::new("building:runtime_wall_b").unwrap()],
        &StableId::new("building:wall").unwrap(),
    );
    assert!(wall_a.contains(&GridPos { x: 47, z: 16 }));
    assert!(wall_b.contains(&GridPos { x: 48, z: 16 }));
}

#[test]
fn diagnostic_views_use_live_navigation_and_authored_placement_footprints() {
    let mut config = GameConfig::default();
    config.world.width = 10;
    config.world.height = 10;
    let content = embedded_content();
    let resource_position = GridPos { x: 1, z: 1 };
    let world = GeneratedWorld {
        seed: 17,
        generator_version: 1,
        navigation: stream_town_domain::NavGrid::new(10, 10, vec![false; 100], vec![100; 100])
            .unwrap(),
        resources: vec![stream_town_domain::GeneratedResource {
            id: StableId::new("resource:diagnostic-tree").unwrap(),
            kind: StableId::new("resource:wood").unwrap(),
            target_kind: StableId::new("target:tree").unwrap(),
            position: resource_position,
            offset_milli_cells: [0, 0],
            generation_occupancy: [0, 0],
            amount: 100,
        }],
        foliage: Vec::new(),
        deterministic_hash: String::new(),
    };
    let house_id = StableId::new("building:house").unwrap();
    let house = &content.buildings[&house_id];
    let runtime_id = StableId::new("building:diagnostic-house").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id,
            archetype: house.archetype.clone(),
            position: GridPos { x: 2, z: 3 },
            rotation_quarter_turns: 1,
            level: 1,
            health: 100,
            complete: true,
        },
    );

    let footprint_cells = floorplan_diagnostic_cells(&content, &simulation, &world);
    let mut expected = fine_cells_for_coarse_cell(resource_position)
        .into_iter()
        .collect::<BTreeSet<_>>();
    let runtime = &simulation.buildings[&StableId::new("building:diagnostic-house").unwrap()];
    expected.extend(building_fine_placement_cells(
        &content,
        &simulation,
        runtime,
        &house_id,
        house,
    ));
    assert_eq!(footprint_cells, expected);

    let navigation = build_fine_navigation(&config, &content, &simulation, &world).unwrap();
    let blocked = (0..navigation.height())
        .flat_map(|z| (0..navigation.width()).map(move |x| GridPos { x, z }))
        .filter(|cell| !navigation.is_walkable(*cell))
        .count();
    let pathfinding_mesh = world_diagnostic_overlay_mesh(
        WorldDiagnosticMode::Pathfinding,
        &config,
        &content,
        &simulation,
        &world,
        &navigation,
    )
    .unwrap();
    let floorplan_mesh = world_diagnostic_overlay_mesh(
        WorldDiagnosticMode::Floorplan,
        &config,
        &content,
        &simulation,
        &world,
        &navigation,
    )
    .unwrap();
    assert_eq!(
        pathfinding_mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .len(),
        (usize::from(navigation.width()) + 1) * (usize::from(navigation.height()) + 1)
    );
    assert_eq!(pathfinding_mesh.indices().unwrap().len(), blocked * 6);
    assert_eq!(
        floorplan_mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .len(),
        footprint_cells.len() * 4
    );

    let mut runtime = WorldDiagnosticRuntime::default();
    runtime.activate(WorldDiagnosticMode::Floorplan);
    assert_eq!(runtime.mode, Some(WorldDiagnosticMode::Floorplan));
    assert!((runtime.remaining_seconds - WORLD_DIAGNOSTIC_VIEW_SECONDS).abs() <= f32::EPSILON);
    runtime.remaining_seconds = 0.25;
    runtime.refresh_if_active();
    assert!((runtime.remaining_seconds - WORLD_DIAGNOSTIC_VIEW_SECONDS).abs() <= f32::EPSILON);
    assert!(command_refreshes_world_diagnostic(&ChatCommand::Build(
        StableId::new("thickpath").unwrap()
    )));
    assert!(command_refreshes_world_diagnostic(&ChatCommand::Follow(
        None
    )));
    assert!(!command_refreshes_world_diagnostic(&ChatCommand::Health));

    let mut app = App::new();
    app.insert_resource(RuntimeConfig(config))
        .insert_resource(RuntimeContent(content))
        .insert_resource(SimulationRuntime(simulation))
        .insert_resource(WorldRuntime {
            generated: world,
            legacy_terrain_mesh: None,
            legacy_migration: None,
        })
        .insert_resource(FineNavigationRuntime {
            grid: Some(navigation),
            applied_signature: 0,
        })
        .insert_resource(runtime)
        .init_resource::<Time>()
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .add_systems(Update, sync_world_diagnostic_view);
    let building_entity = app
        .world_mut()
        .spawn((
            RuntimeBuilding {
                id: StableId::new("building:visibility-test").unwrap(),
            },
            Visibility::Inherited,
        ))
        .id();
    app.update();
    assert_eq!(
        app.world().get::<Visibility>(building_entity),
        Some(&Visibility::Hidden)
    );
    assert!(
        app.world()
            .get::<WorldDiagnosticVisibilityBackup>(building_entity)
            .is_some()
    );
    let mut overlay_query = app.world_mut().query::<&WorldDiagnosticOverlay>();
    assert_eq!(overlay_query.iter(app.world()).count(), 1);

    app.world_mut()
        .resource_mut::<WorldDiagnosticRuntime>()
        .mode = None;
    app.update();
    assert_eq!(
        app.world().get::<Visibility>(building_entity),
        Some(&Visibility::Inherited)
    );
    assert!(
        app.world()
            .get::<WorldDiagnosticVisibilityBackup>(building_entity)
            .is_none()
    );
    let mut overlay_query = app.world_mut().query::<&WorldDiagnosticOverlay>();
    assert_eq!(overlay_query.iter(app.world()).count(), 0);
}

#[test]
fn sustained_camp_encounter_reaches_combat_and_is_deterministic() {
    fn run_encounter() -> (u32, i32, i32, i32, u32, u32, bool, bool) {
        let config = GameConfig::default();
        let content = embedded_content();
        let mut world = generate_world(&config.world);
        let mut simulation = WorldSimulation::new(world.seed);
        ensure_town_hall_state(&content, &config, &mut simulation);
        let town_hall = StableId::new("building:townhall").unwrap();
        let hall_state = &simulation.buildings[&town_hall];
        let hall_definition = building_def_for_archetype(&content, &hall_state.archetype).unwrap();
        world
            .navigation
            .set_blocked(
                building_navigation_region(
                    hall_state.position,
                    hall_definition,
                    hall_state.rotation_quarter_turns,
                    &world,
                )
                .unwrap(),
                true,
            )
            .unwrap();
        assert!(seed_generated_enemy_camps(&config, &content, &mut world, &mut simulation) > 0);

        let camp = simulation.enemy_camps.values().next().unwrap().clone();
        let camp_archetype = &content.archetypes[&camp.archetype];
        let spawner = camp_archetype.enemy_spawner.as_ref().unwrap();
        let enemy_archetype =
            archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();
        let enemy_position = enemy_spawn_position(
            &world,
            &content,
            &simulation,
            None,
            &camp,
            spawner,
            camp_archetype.footprint,
            0,
        );
        let enemy_id = StableId::new("actor:sustained_enemy").unwrap();
        let enemy_health = content.archetypes[&enemy_archetype]
            .health
            .as_ref()
            .unwrap()
            .max_health;
        let expected_kill_reward = content.archetypes[&enemy_archetype]
            .enemy
            .as_ref()
            .unwrap()
            .kill_reward
            .amount;
        assert!(simulation.spawn_enemy(
            enemy_id.clone(),
            enemy_archetype,
            enemy_position,
            i32::try_from(enemy_health).unwrap(),
        ));

        let initial_enemy_goal = next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &enemy_id,
            enemy_position,
        );
        assert_eq!(
            initial_enemy_goal.0,
            AgentGoal::AttackBuilding(town_hall.clone())
        );
        let initial_enemy_path = try_agent_path(
            &world.navigation,
            &content,
            &simulation,
            &ActorKind::Enemy,
            enemy_position,
            initial_enemy_goal.1,
        )
        .expect("generated camp has a route to the active Town Hall");
        assert!(initial_enemy_path.len() > 1);
        // Put the defender on the verified assault route after proving the
        // no-local-target Town Hall goal. This preserves the complete
        // target-handoff/combat path while keeping the regression fast.
        let defender_position =
            initial_enemy_path[initial_enemy_path.len().saturating_sub(1).min(8)];
        let defender_id = StableId::new("npc:sustained_defender").unwrap();
        assert!(simulation.join_player(defender_id.clone(), defender_position));
        simulation
            .assign_role(&defender_id, StableId::new("role:defender").unwrap())
            .unwrap();
        let defender_damage =
            effective_role_stats(&content, &simulation, &simulation.actors[&defender_id])
                .unwrap()
                .action_amount;
        let encounter_health = i32::try_from(defender_damage.saturating_mul(3)).unwrap();
        let enemy = simulation.actors.get_mut(&enemy_id).unwrap();
        enemy.health = encounter_health;
        enemy.max_health = encounter_health;

        let mut cooldowns = BTreeMap::<StableId, f32>::new();
        let mut movement_budget = BTreeMap::<StableId, f32>::new();
        let mut combat_started = false;
        let mut defender_acquired_enemy = false;
        let mut elapsed_ticks = 0_u32;
        let delta_seconds = 0.25_f32;
        for tick in 1..=600_u32 {
            elapsed_ticks = tick;
            let actor_ids = simulation.actors.keys().cloned().collect::<Vec<_>>();
            for actor_id in actor_ids {
                if !simulation
                    .actors
                    .get(&actor_id)
                    .is_some_and(|actor| actor.alive)
                {
                    continue;
                }
                cooldowns
                    .entry(actor_id.clone())
                    .and_modify(|remaining| {
                        *remaining = (*remaining - delta_seconds).max(0.0);
                    })
                    .or_insert(0.0);
                let speed = actor_movement_speed(&config, &content, &simulation, &actor_id);
                movement_budget
                    .entry(actor_id.clone())
                    .and_modify(|budget| *budget += speed * delta_seconds)
                    .or_insert(speed * delta_seconds);

                loop {
                    let current = simulation.actors[&actor_id].position;
                    let (goal, target) =
                        next_agent_goal(&simulation, &world, &config, &content, &actor_id, current);
                    if actor_id == defender_id && goal == AgentGoal::Attack(enemy_id.clone()) {
                        defender_acquired_enemy = true;
                    }
                    if target == current {
                        if cooldowns[&actor_id] <= f32::EPSILON {
                            let presentation = complete_agent_goal(
                                &mut simulation,
                                &mut world,
                                &config,
                                &content,
                                &actor_id,
                                &goal,
                                current,
                            );
                            if matches!(goal, AgentGoal::Attack(_) | AgentGoal::AttackBuilding(_))
                                && presentation.is_some()
                            {
                                combat_started = true;
                            }
                            if let Some(ActionPresentation::Projectile(projectile)) = presentation {
                                let _ = apply_combat_damage(
                                    &config,
                                    &mut simulation,
                                    &content,
                                    match projectile.source {
                                        ProjectileSource::Actor(ref source) => Some(source),
                                        ProjectileSource::Building(_) => None,
                                    },
                                    &projectile.target,
                                    projectile.damage,
                                );
                            }
                            cooldowns.insert(
                                actor_id.clone(),
                                action_cooldown(&content, &simulation, &actor_id, &goal),
                            );
                        }
                        break;
                    }
                    if movement_budget[&actor_id] < 1.0 {
                        break;
                    }
                    let kind = if actor_id == enemy_id {
                        ActorKind::Enemy
                    } else {
                        ActorKind::Player
                    };
                    let Some(path) = try_agent_path(
                        &world.navigation,
                        &content,
                        &simulation,
                        &kind,
                        current,
                        target,
                    ) else {
                        break;
                    };
                    let Some(next) = path.get(1).copied() else {
                        break;
                    };
                    simulation.actors.get_mut(&actor_id).unwrap().position = next;
                    *movement_budget.get_mut(&actor_id).unwrap() -= 1.0;
                }
            }
            if !simulation.actors[&enemy_id].alive
                || !simulation.actors[&defender_id].alive
                || !simulation.buildings.contains_key(&town_hall)
            {
                break;
            }
        }

        (
            elapsed_ticks,
            simulation.actors[&enemy_id].health,
            simulation.actors[&defender_id].health,
            simulation
                .buildings
                .get(&town_hall)
                .map_or(0, |building| building.health),
            simulation
                .town_resources
                .get(&StableId::new("resource:gold").unwrap())
                .copied()
                .unwrap_or_default(),
            expected_kill_reward,
            combat_started,
            defender_acquired_enemy,
        )
    }

    let first = run_encounter();
    let second = run_encounter();
    assert_eq!(second, first);
    assert!(first.6, "the routed encounter never reached an attack");
    assert!(first.7, "the defender never acquired the approaching enemy");
    assert_eq!(first.1, 0, "the defender did not defeat the enemy");
    assert_eq!(
        first.4, first.5,
        "the authored player kill reward did not fire once"
    );
}

#[test]
fn every_camp_enemy_has_a_renderable_animated_model_contract() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    for layer in &content.enemy_camp_generation {
        let spawner = content.archetypes[&layer.camp_archetype]
            .enemy_spawner
            .as_ref()
            .unwrap();
        for weighted in &spawner.weighted_enemies {
            let archetype = &content.archetypes[&weighted.enemy_archetype];
            let scene = runtime_archetype_scene(archetype).expect("enemy model scene");
            assert!(
                Path::new(&scene.asset_path)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
            );
            assert!(archetype.enemy_models.is_some());
            assert!(
                converted_animation_spec(archetype, &presentation).is_some()
                    || native_animation_request(archetype, &scene, &presentation).is_some(),
                "{} lacks a usable animation contract",
                archetype.source_path
            );
        }
    }
}

#[test]
fn authored_enemies_drive_damage_range_cadence_and_weighted_spawning() {
    let config = GameConfig::default();
    let mut content = embedded_content();
    let mut world = generate_world(&config.world);
    let blargul_archetype =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Blargul.prefab").unwrap();
    let blargul = &content.archetypes[&blargul_archetype].enemy;
    let blargul = blargul.as_ref().unwrap();
    assert_eq!(blargul.action_amount, 5);
    assert_eq!(blargul.action_milliseconds, 3_000);
    assert_eq!(blargul.action_range_milli_cells, 5_000);

    let enemy_id = StableId::new("actor:enemy_authored_test").unwrap();
    let player_id = StableId::new("actor:player_authored_test").unwrap();
    let enemy_position = GridPos { x: 30, z: 30 };
    // Unity's shipped attack state compares squared distance against a
    // linear range term plus the target bounds, giving Blargul a little
    // over two logical cells of effective reach.
    let player_position = GridPos { x: 32, z: 30 };
    let mut simulation = WorldSimulation::new(world.seed);
    let gold = StableId::new("resource:gold").unwrap();
    simulation.town_resources.insert(gold.clone(), 0);
    assert!(simulation.spawn_enemy(enemy_id.clone(), blargul_archetype, enemy_position, 5,));
    assert!(simulation.join_player(player_id.clone(), player_position));
    let defender = StableId::new("role:defender").unwrap();
    simulation
        .assign_role(&player_id, defender.clone())
        .unwrap();
    content
        .roles
        .get_mut(&defender)
        .unwrap()
        .base_damage_reduction_percent = 90;
    assert_eq!(
        effective_role_stats(&content, &simulation, &simulation.actors[&player_id])
            .unwrap()
            .damage_reduction_percent,
        90,
        "the converted role stat remains inspectable"
    );
    let (goal, target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &enemy_id,
        enemy_position,
    );
    assert_eq!(goal, AgentGoal::Attack(player_id.clone()));
    assert_eq!(target, enemy_position);
    let presentation = complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &enemy_id,
        &goal,
        enemy_position,
    );
    assert!(matches!(
        presentation,
        Some(ActionPresentation::Impact {
            target,
            visual: CombatVisualKind::Physical,
        }) if target == player_position
    ));
    assert_eq!(
        simulation.actors[&player_id].health, 95,
        "Unity computes defense but its reachable attack helpers pass raw authored damage"
    );
    assert!((action_cooldown(&content, &simulation, &enemy_id, &goal) - 3.0).abs() <= f32::EPSILON);
    assert!(apply_combat_damage(&config, &mut simulation, &content, None, &enemy_id, 5).unwrap());
    assert_eq!(
        simulation.town_resources[&gold], 0,
        "Unity tower/environment damage does not count as a player kill"
    );
    simulation.respawn_actor(&enemy_id, enemy_position).unwrap();
    assert!(
        apply_combat_damage(
            &config,
            &mut simulation,
            &content,
            Some(&player_id),
            &enemy_id,
            5,
        )
        .unwrap()
    );
    assert_eq!(simulation.town_resources[&gold], 50);

    let camp = content
        .archetypes
        .values()
        .find_map(|archetype| archetype.enemy_spawner.as_ref())
        .unwrap();
    let samples: BTreeSet<_> = (0..1_000)
        .map(|serial| weighted_enemy_archetype(camp, 42, serial))
        .collect();
    assert_eq!(samples.len(), 3);
}

#[test]
fn enemy_sensor_range_and_retaliation_follow_each_authored_prefab() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let goblin_archetype =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();
    let goblin = content.archetypes[&goblin_archetype]
        .enemy
        .as_ref()
        .unwrap();
    assert_eq!(goblin.target_search_range_milli_cells, 4_000);
    assert!(goblin.attack_attacker);

    let enemy_id = StableId::new("actor:sensor_goblin").unwrap();
    let attacker_id = StableId::new("npc:sensor_attacker").unwrap();
    let enemy_position = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .find(|position| world.navigation.is_walkable(*position))
        .expect("generated world has a walkable enemy position");
    let attacker_position = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .find(|position| {
            world.navigation.is_walkable(*position)
                && grid_distance_squared(*position, enemy_position) > 16
                && grid_distance_squared(*position, enemy_position) <= 36
                && enemy_navigation_can_reach(&world, enemy_position, *position)
        })
        .expect("generated world has a connected retaliation target outside sensor range");
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.spawn_enemy(enemy_id.clone(), goblin_archetype, enemy_position, 10,));
    assert!(simulation.join_player(attacker_id.clone(), attacker_position));

    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &enemy_id,
            enemy_position,
        )
        .0,
        AgentGoal::Wander,
    );
    assert!(
        !apply_combat_damage(
            &config,
            &mut simulation,
            &content,
            Some(&attacker_id),
            &enemy_id,
            1,
        )
        .unwrap()
    );
    assert_eq!(
        simulation.actors[&enemy_id].preferred_target,
        Some(attacker_id.clone()),
    );
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &enemy_id,
            enemy_position,
        ),
        (AgentGoal::Attack(attacker_id), attacker_position),
    );

    let serialized = ron::to_string(&simulation).unwrap();
    let reloaded: WorldSimulation = ron::from_str(&serialized).unwrap();
    assert_eq!(reloaded, simulation);
}

#[test]
fn battering_ram_targets_and_damages_buildings_from_authored_mask() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let battering_ram = archetype_id_by_source(
        &content,
        ArchetypeKind::Enemy,
        "Enemy_Goblin_BatteringRam.prefab",
    )
    .unwrap();
    let enemy_health = content.archetypes[&battering_ram]
        .health
        .as_ref()
        .and_then(|health| i32::try_from(health.max_health).ok())
        .expect("battering ram has valid authored health");
    let enemy_id = StableId::new("actor:battering_ram_test").unwrap();
    let building_id = StableId::new("building:ram_target").unwrap();
    let house = &content.buildings[&StableId::new("building:house").unwrap()];
    let building_position = find_building_site(
        &world,
        GridPos {
            x: config.world.width / 2,
            z: config.world.height / 2,
        },
        house.footprint,
    )
    .expect("generated world has a house site");
    let enemy_position = building_approach(
        &world,
        building_position,
        house.footprint,
        building_position,
    )
    .expect("house site has an enemy approach");
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.spawn_enemy(
        enemy_id.clone(),
        battering_ram.clone(),
        enemy_position,
        enemy_health,
    ));
    simulation.buildings.insert(
        building_id.clone(),
        BuildingState {
            id: building_id.clone(),
            archetype: house.archetype.clone(),
            position: building_position,
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );
    let (goal, target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &enemy_id,
        enemy_position,
    );
    assert_eq!(goal, AgentGoal::AttackBuilding(building_id.clone()));
    assert_eq!(target, enemy_position);
    let presentation = complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &enemy_id,
        &goal,
        enemy_position,
    );
    assert!(matches!(
        presentation,
        Some(ActionPresentation::BuildingWork {
            target,
            sparks: true,
        }) if target == building_visual_grid(&content, &simulation.buildings[&building_id])
    ));
    assert_eq!(
        simulation.buildings[&building_id].health,
        BUILDING_MAX_HEALTH - 20
    );
    let footprint = content.buildings[&StableId::new("building:house").unwrap()].footprint;
    let region = building_region(building_position, footprint, &world).unwrap();
    world.navigation.set_blocked(region, true).unwrap();
    simulation.buildings.get_mut(&building_id).unwrap().health = 20;
    let presentation = complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &enemy_id,
        &goal,
        enemy_position,
    );
    assert!(matches!(
        presentation,
        Some(ActionPresentation::BuildingDestroyed {
            building,
            target: _,
        }) if building == building_id
    ));
    assert!(!simulation.buildings.contains_key(&building_id));
    assert!(world.navigation.is_walkable(region.min));
}

#[test]
fn priest_prioritizes_and_heals_the_nearest_injured_player() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let priest_position = GridPos { x: 32, z: 32 };
    let patient_position = GridPos { x: 33, z: 32 };
    let priest = StableId::new("npc:priest_test").unwrap();
    let patient = StableId::new("npc:patient_test").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(priest.clone(), priest_position));
    assert!(simulation.join_player(patient.clone(), patient_position));
    simulation
        .assign_role(&priest, StableId::new("role:priest").unwrap())
        .unwrap();
    simulation.damage_actor(&patient, 20).unwrap();

    let (goal, target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &priest,
        priest_position,
    );
    assert_eq!(goal, AgentGoal::Heal(patient.clone()));
    assert_eq!(target, priest_position);
    let presentation = complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &priest,
        &goal,
        priest_position,
    );
    assert!(matches!(
        presentation,
        Some(ActionPresentation::Healing { source, target })
            if source == priest && target == patient
    ));
    assert_eq!(simulation.actors[&patient].health, 82);
}

#[test]
fn engaged_priest_keeps_healing_through_the_twenty_five_percent_range_band() {
    let content = embedded_content();
    let priest = StableId::new("npc:priest_hysteresis").unwrap();
    let patient = StableId::new("npc:patient_hysteresis").unwrap();
    let origin = GridPos { x: 20, z: 20 };
    let mut simulation = WorldSimulation::new(1);
    assert!(simulation.join_player(priest.clone(), origin));
    assert!(simulation.join_player(patient.clone(), origin));
    simulation
        .assign_role(&priest, StableId::new("role:priest").unwrap())
        .unwrap();
    simulation.damage_actor(&patient, 40).unwrap();
    let edge = (1..20)
        .map(|offset| GridPos {
            x: origin.x + offset,
            z: origin.z,
        })
        .find(|position| {
            simulation.actors.get_mut(&patient).unwrap().position = *position;
            !within_actor_heal_range(
                &content,
                &simulation,
                &simulation.actors[&priest],
                &simulation.actors[&patient],
                origin,
            ) && within_actor_heal_hysteresis_range(
                &content,
                &simulation,
                &simulation.actors[&priest],
                &simulation.actors[&patient],
                origin,
            )
        })
        .expect("the expanded range has at least one whole-cell sample");
    simulation.actors.get_mut(&patient).unwrap().position = edge;
    assert_eq!(
        retained_healing_goal(
            &AgentGoal::Heal(patient.clone()),
            &content,
            &simulation,
            &priest,
            origin,
        ),
        Some((AgentGoal::Heal(patient), origin))
    );
}

#[test]
fn citizen_retreat_uses_entry_and_full_health_exit_hysteresis() {
    let actor = StableId::new("npc:retreat_test").unwrap();
    let mut simulation = WorldSimulation::new(1);
    assert!(simulation.join_player(actor.clone(), GridPos { x: 1, z: 1 }));
    let state = simulation.actors.get_mut(&actor).unwrap();
    state.max_health = 100;
    state.health = 24;
    assert!(citizen_should_retreat(state, false));
    state.health = 99;
    assert!(citizen_should_retreat(state, true));
    state.health = 100;
    assert!(!citizen_should_retreat(state, true));
    state.health = 25;
    assert!(!citizen_should_retreat(state, false));
}

#[test]
fn saved_recruits_are_normalized_to_level_one_without_capping_players() {
    let content = embedded_content();
    let recruit = StableId::new("npc:recruit_level_cap").unwrap();
    let initial_viewer = StableId::new("actor:viewer_0005").unwrap();
    let guard = StableId::new("npc:guardhouse_building:runtime_test").unwrap();
    let player = StableId::new("twitch:player_level_cap").unwrap();
    let role = StableId::new("role:defender").unwrap();
    let mut simulation = WorldSimulation::new(1);
    assert!(simulation.join_player(recruit.clone(), GridPos { x: 1, z: 1 }));
    assert!(simulation.join_player(initial_viewer.clone(), GridPos { x: 1, z: 2 }));
    assert!(simulation.join_player(guard.clone(), GridPos { x: 1, z: 3 }));
    assert!(simulation.join_player(player.clone(), GridPos { x: 2, z: 1 }));
    for actor in [&recruit, &initial_viewer, &guard, &player] {
        simulation
            .actors
            .get_mut(actor)
            .unwrap()
            .role_progression
            .insert(
                role.clone(),
                stream_town_domain::RoleProgress {
                    level: 42,
                    experience: 12_345,
                },
            );
    }

    assert_eq!(
        normalize_recruit_role_progression(&content, &mut simulation),
        3
    );
    assert_eq!(simulation.actors[&recruit].role_progression[&role].level, 1);
    assert_eq!(
        simulation.actors[&recruit].role_progression[&role].experience,
        0
    );
    for actor in [&initial_viewer, &guard] {
        assert_eq!(simulation.actors[actor].role_progression[&role].level, 1);
        assert_eq!(
            simulation.actors[actor].role_progression[&role].experience,
            0
        );
    }
    assert_eq!(simulation.actors[&player].role_progression[&role].level, 42);
    assert_eq!(
        simulation.actors[&player].role_progression[&role].experience,
        12_345
    );
}

#[test]
fn healing_effect_curves_preserve_authored_lifetimes_and_channel_keys() {
    let presentation = embedded_presentation();
    let burst_duration = healing_effect_duration(&presentation, HealingEffectKind::Burst);
    let channel_duration = healing_effect_duration(&presentation, HealingEffectKind::Channel);
    let revive_duration = healing_effect_duration(&presentation, HealingEffectKind::Revive);
    assert!((burst_duration - 1.2).abs() < f32::EPSILON);
    assert!((channel_duration - 5.0).abs() < f32::EPSILON);
    assert!((revive_duration - 1.2).abs() < f32::EPSILON);
    let burst = healing_burst_effect(&presentation).1;
    let channel = healing_channel_effect(&presentation).1;
    assert_eq!(burst.plus_burst_count, 100);
    assert_eq!(burst.plus_capacity, 8);
    assert_eq!(burst.disc_burst_count, 1);
    assert_eq!(channel.particle_capacity, 32);
    assert!((channel.emission_rate_per_second - 16.0).abs() < f32::EPSILON);

    let channel_start = healing_effect_sample(
        &presentation,
        HealingEffectKind::Channel,
        0.0,
        channel_duration,
    );
    let channel_first_key = healing_effect_sample(
        &presentation,
        HealingEffectKind::Channel,
        1.5,
        channel_duration,
    );
    let channel_peak = healing_effect_sample(
        &presentation,
        HealingEffectKind::Channel,
        3.0,
        channel_duration,
    );
    let channel_end = healing_effect_sample(
        &presentation,
        HealingEffectKind::Channel,
        5.0,
        channel_duration,
    );
    assert!(channel_start.ring_scale.abs() < f32::EPSILON);
    assert!((channel_first_key.ring_scale - 0.289_276_9).abs() < f32::EPSILON);
    assert!((channel_peak.ring_scale - 1.0).abs() < f32::EPSILON);
    assert!(channel_end.ring_scale.abs() < f32::EPSILON);

    let burst_midpoint =
        healing_effect_sample(&presentation, HealingEffectKind::Burst, 0.6, burst_duration);
    let revive_midpoint = healing_effect_sample(
        &presentation,
        HealingEffectKind::Revive,
        0.6,
        revive_duration,
    );
    assert!(burst_midpoint.ring_scale > 0.0);
    assert!(burst_midpoint.mote_scale > 0.0);
    assert!(revive_midpoint.ring_scale > burst_midpoint.ring_scale);
    assert!(revive_midpoint.rise > burst_midpoint.rise);
}

#[test]
fn healing_ring_mesh_is_a_closed_top_facing_annulus() {
    let mesh = healing_ring_mesh(48);
    assert_eq!(mesh.count_vertices(), 96);
    assert_eq!(mesh.indices().unwrap().len(), 288);
    assert!(mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_some());
    assert!(mesh.attribute(Mesh::ATTRIBUTE_UV_0).is_some());
}

#[test]
fn ranged_roles_emit_projectiles_instead_of_instant_damage() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let ranger_position = GridPos { x: 32, z: 32 };
    let enemy_position = GridPos { x: 34, z: 32 };
    let ranger = StableId::new("npc:ranger_test").unwrap();
    let enemy = StableId::new("actor:enemy_ranged_test").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(ranger.clone(), ranger_position));
    assert!(simulation.join_player(enemy.clone(), enemy_position));
    simulation
        .assign_role(&ranger, StableId::new("role:ranger").unwrap())
        .unwrap();
    simulation
        .assign_role(&enemy, StableId::new("role:enemy").unwrap())
        .unwrap();
    let (goal, _) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &ranger,
        ranger_position,
    );
    let presentation = complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &ranger,
        &goal,
        ranger_position,
    )
    .unwrap();
    let ActionPresentation::Projectile(projectile) = presentation else {
        panic!("ranged role must emit a projectile");
    };
    assert_eq!(projectile.target, enemy);
    assert_eq!(projectile.visual, CombatVisualKind::Arrow);
    assert_eq!(simulation.actors[&projectile.target].health, 100);
}

#[test]
fn combat_visuals_follow_authored_role_identity_and_vfx_constants() {
    assert_eq!(
        actor_combat_visual(&StableId::new("role:ranger").unwrap()),
        CombatVisualKind::Arrow
    );
    assert_eq!(
        actor_combat_visual(&StableId::new("role:wizard").unwrap()),
        CombatVisualKind::Fireball
    );
    assert_eq!(
        actor_combat_visual(&StableId::new("role:necromancer").unwrap()),
        CombatVisualKind::Necrotic
    );
    assert_eq!(
        actor_combat_visual(&StableId::new("role:defender").unwrap()),
        CombatVisualKind::Physical
    );
    assert!((CHARACTER_HIT_SECONDS - 0.25).abs() < f32::EPSILON);
    assert!((TOWER_TRAIL_SECONDS - 2.0).abs() < f32::EPSILON);
    assert!((TOWER_TRAIL_WIDTH - 0.1).abs() < f32::EPSILON);
    assert!((FIREBALL_SIZE - 0.4).abs() < f32::EPSILON);
    assert!((FIREBALL_TRAIL_SIZE - 0.3).abs() < f32::EPSILON);
}

#[test]
fn builders_can_work_from_every_planned_corner_approach() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let builder_id = StableId::new("npc:corner_builder").unwrap();
    let building_position = (1..world.navigation.height().saturating_sub(1))
        .flat_map(|z| {
            (1..world.navigation.width().saturating_sub(1)).map(move |x| GridPos { x, z })
        })
        .find(|position| {
            (position.z - 1..=position.z + 1).all(|z| {
                (position.x - 1..=position.x + 1)
                    .all(|x| world.navigation.is_walkable(GridPos { x, z }))
            })
        })
        .expect("generated world should contain a walkable 3x3 area");
    let builder_position = GridPos {
        x: building_position.x - 1,
        z: building_position.z - 1,
    };
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(builder_id.clone(), builder_position));
    simulation
        .assign_role(&builder_id, StableId::new("role:builder").unwrap())
        .unwrap();

    for (index, building_name) in ["path", "wall"].into_iter().enumerate() {
        let definition =
            &content.buildings[&StableId::new(format!("building:{building_name}")).unwrap()];
        let runtime_id = StableId::new(format!("building:corner_work_{index}")).unwrap();
        simulation.buildings.insert(
            runtime_id.clone(),
            BuildingState {
                id: runtime_id.clone(),
                archetype: definition.archetype.clone(),
                position: building_position,
                rotation_quarter_turns: 0,
                level: 1,
                health: 3,
                complete: false,
            },
        );
        assert!(
            building_approaches(
                &world,
                building_position,
                definition.footprint,
                builder_position,
            )
            .contains(&builder_position)
        );
        assert!(within_building_work_range(
            &content,
            &simulation,
            &simulation.actors[&builder_id],
            &simulation.buildings[&runtime_id],
            builder_position,
        ));
        assert!(
            complete_agent_goal(
                &mut simulation,
                &mut world,
                &config,
                &content,
                &builder_id,
                &AgentGoal::Construct(runtime_id.clone()),
                builder_position,
            )
            .is_some()
        );
        assert_eq!(simulation.buildings[&runtime_id].health, 4);
        simulation.buildings.remove(&runtime_id);
    }
}

#[test]
fn builder_selection_uses_an_alternate_approach_after_path_rejection() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let building_position = (1..world.navigation.height().saturating_sub(1))
        .flat_map(|z| {
            (1..world.navigation.width().saturating_sub(1)).map(move |x| GridPos { x, z })
        })
        .find(|position| {
            (position.z - 1..=position.z + 1).all(|z| {
                (position.x - 1..=position.x + 1)
                    .all(|x| world.navigation.is_walkable(GridPos { x, z }))
            })
        })
        .expect("generated world should contain a walkable 3x3 area");
    let builder_position = GridPos {
        x: building_position.x - 1,
        z: building_position.z - 1,
    };
    let builder_id = StableId::new("npc:alternate_approach_builder").unwrap();
    let building_id = StableId::new("building:alternate_approach").unwrap();
    let definition = &content.buildings[&StableId::new("building:path").unwrap()];
    let mut simulation = WorldSimulation::new(world.seed);
    assert!(simulation.join_player(builder_id.clone(), builder_position));
    simulation
        .assign_role(&builder_id, StableId::new("role:builder").unwrap())
        .unwrap();
    simulation.buildings.insert(
        building_id.clone(),
        BuildingState {
            id: building_id.clone(),
            archetype: definition.archetype.clone(),
            position: building_position,
            rotation_quarter_turns: 0,
            level: 1,
            health: 3,
            complete: false,
        },
    );

    let choose = |approach_reservations: &BTreeMap<GridPos, StableId>| {
        next_agent_goal_with_station_runtime(
            &simulation,
            &world,
            None,
            &config,
            &content,
            &StationTargetRuntime::default(),
            &builder_id,
            builder_position,
            placement_to_navigation_centre(builder_position),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            approach_reservations,
        )
    };
    let (first_goal, first_approach) = choose(&BTreeMap::new());
    assert_eq!(first_goal, AgentGoal::Construct(building_id.clone()));

    let rejected = BTreeMap::from([(
        first_approach,
        StableId::new("system:unreachable-planning-target").unwrap(),
    )]);
    let (retry_goal, retry_approach) = choose(&rejected);
    assert_eq!(retry_goal, AgentGoal::Construct(building_id));
    assert_ne!(retry_approach, first_approach);
}

#[test]
fn builder_completes_and_upgrades_authored_construction() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
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
    let runtime_id = StableId::new("building:test_house").unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    for resource in definition.cost.keys() {
        simulation
            .town_resources
            .insert(resource.clone(), 1_000_000);
    }
    simulation
        .construct(
            runtime_id.clone(),
            definition.archetype.clone(),
            position,
            building_base_max_health(&content, definition),
            &definition.cost,
        )
        .unwrap();
    assert!(!simulation.buildings[&runtime_id].complete);
    assert_eq!(
        building_construction_stage(
            simulation.buildings[&runtime_id].health,
            building_max_health(&content, &simulation.buildings[&runtime_id]),
            simulation.buildings[&runtime_id].complete,
        ),
        0
    );
    let region = building_region(position, definition.footprint, &world).unwrap();
    world.navigation.set_blocked(region, true).unwrap();
    let builder_position =
        building_approach(&world, position, definition.footprint, position).unwrap();
    let builder = StableId::new("npc:builder_test").unwrap();
    assert!(simulation.join_player(builder.clone(), builder_position));
    simulation
        .assign_role(&builder, StableId::new("role:builder").unwrap())
        .unwrap();
    let (goal, target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &builder,
        builder_position,
    );
    assert_eq!(goal, AgentGoal::Construct(runtime_id.clone()));
    assert_eq!(target, builder_position);
    for _ in 0..450 {
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &builder,
            &goal,
            builder_position,
        );
    }
    assert!(simulation.buildings[&runtime_id].complete);
    assert_eq!(
        simulation.buildings[&runtime_id].health,
        building_max_health(&content, &simulation.buildings[&runtime_id])
    );
    assert_eq!(
        building_construction_stage(
            simulation.buildings[&runtime_id].health,
            building_max_health(&content, &simulation.buildings[&runtime_id]),
            simulation.buildings[&runtime_id].complete,
        ),
        3
    );

    simulation.damage_building(&runtime_id, 25).unwrap();
    let damaged_health = simulation.buildings[&runtime_id].health;
    let (repair_goal, repair_target) = next_agent_goal(
        &simulation,
        &world,
        &config,
        &content,
        &builder,
        builder_position,
    );
    assert_eq!(repair_goal, AgentGoal::Construct(runtime_id.clone()));
    assert_eq!(repair_target, builder_position);
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &builder,
            &repair_goal,
            builder_position,
        )
        .is_some()
    );
    assert!(simulation.buildings[&runtime_id].health > damaged_health);
    while simulation.buildings[&runtime_id].health
        < building_max_health(&content, &simulation.buildings[&runtime_id])
    {
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &builder,
            &repair_goal,
            builder_position,
        );
    }

    let (technology, authored_cap) = content
        .technology
        .nodes
        .iter()
        .find_map(|(technology, node)| {
            node.building_level_caps
                .get(&building_id)
                .copied()
                .map(|cap| (technology.clone(), cap))
        })
        .expect("Unity technology graph contains a House level-cap effect");
    simulation.unlocked_technology.insert(technology);
    let max_level = maximum_building_level(&content, &simulation, &building_id);
    assert_eq!(max_level, authored_cap);
    let upgrade_cost = building_upgrade_cost(&content, &simulation, &building_id, definition, 1, 2);
    simulation.damage_building(&runtime_id, 7).unwrap();
    assert_eq!(
        simulation
            .upgrade_building(
                &runtime_id,
                max_level,
                2,
                building_max_health_at_level(&content, &definition.archetype, 2),
                &upgrade_cost,
            )
            .unwrap(),
        2
    );
    assert_eq!(
        simulation.buildings[&runtime_id].health,
        (building_max_health(&content, &simulation.buildings[&runtime_id]) + 9) / 10
    );
    assert!(!simulation.buildings[&runtime_id].complete);
    while !simulation.buildings[&runtime_id].complete {
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &builder,
            &repair_goal,
            builder_position,
        );
    }
}

#[test]
fn completed_technology_goal_unlocks_and_pays_for_a_building_upgrade() {
    let content = embedded_content();
    let mut simulation = WorldSimulation::new(42);
    simulation.unlocked_technology.extend(
        content
            .technology
            .nodes
            .iter()
            .filter(|(_, node)| node.initially_unlocked)
            .map(|(id, _)| id.clone()),
    );
    let (technology, node, building_id) = content
        .technology
        .nodes
        .iter()
        .find_map(|(technology, node)| {
            if node.initially_unlocked {
                return None;
            }
            node.building_level_caps
                .iter()
                .find(|(building, cap)| {
                    **cap >= 2
                        && content.buildings[*building].can_level
                        && !node.objectives.is_empty()
                })
                .map(|(building, _)| (technology.clone(), node.clone(), building.clone()))
        })
        .expect("converted technology contains an objective-backed building level unlock");
    simulation
        .unlocked_technology
        .extend(node.prerequisites.iter().cloned());
    assert!(simulation.start_technology_goal(
        technology.clone(),
        &node.objectives,
        &content.objectives,
        MAX_TOWN_GOALS,
    ));
    assert!(!simulation.unlocked_technology.contains(&technology));

    for objective_id in &node.objectives {
        let objective = &content.objectives[objective_id];
        let event = match objective.kind {
            ObjectiveKind::Build => {
                ObjectiveEvent::BuildingBuilt(objective.building.clone().unwrap())
            }
            ObjectiveKind::BuildAny => ObjectiveEvent::BuildingBuilt(building_id.clone()),
            ObjectiveKind::Collect => ObjectiveEvent::ResourceGathered {
                resource: objective.resource.clone().unwrap(),
                amount: objective.required_amount,
            },
            ObjectiveKind::EarnPerHour => ObjectiveEvent::ResourceGained {
                resource: objective.resource.clone().unwrap(),
                amount: objective.required_amount,
            },
            ObjectiveKind::Kill => ObjectiveEvent::EnemyKilled(objective.enemy.clone().unwrap()),
            ObjectiveKind::KillAny => {
                ObjectiveEvent::EnemyKilled(StableId::new("enemy:technology_test").unwrap())
            }
            ObjectiveKind::Sell => ObjectiveEvent::ResourceSold {
                resource: objective.resource.clone().unwrap(),
                amount: objective.required_amount,
            },
            ObjectiveKind::SellAny => ObjectiveEvent::ResourceSold {
                resource: StableId::new("resource:technology_test").unwrap(),
                amount: objective.required_amount,
            },
            ObjectiveKind::Buy => ObjectiveEvent::ResourceBought {
                resource: objective.resource.clone().unwrap(),
                amount: objective.required_amount,
            },
            ObjectiveKind::BuyAny => ObjectiveEvent::ResourceBought {
                resource: StableId::new("resource:technology_test").unwrap(),
                amount: objective.required_amount,
            },
        };
        let repetitions = match objective.kind {
            ObjectiveKind::Build
            | ObjectiveKind::BuildAny
            | ObjectiveKind::Kill
            | ObjectiveKind::KillAny => objective.required_amount,
            ObjectiveKind::Collect
            | ObjectiveKind::EarnPerHour
            | ObjectiveKind::Sell
            | ObjectiveKind::SellAny
            | ObjectiveKind::Buy
            | ObjectiveKind::BuyAny => 1,
        };
        for _ in 0..repetitions {
            let _ = simulation.record_objective_event(&content.objectives, &event);
        }
    }
    assert!(simulation.unlocked_technology.contains(&technology));
    assert!(simulation.active_goals.is_empty());
    assert!(maximum_building_level(&content, &simulation, &building_id) >= 2);

    let definition = &content.buildings[&building_id];
    let runtime_id = StableId::new("building:technology_upgrade_test").unwrap();
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
            position: GridPos { x: 10, z: 10 },
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(&content, definition)).unwrap(),
            complete: true,
        },
    );
    let expected_cost =
        building_upgrade_cost(&content, &simulation, &building_id, definition, 1, 2);
    let before = simulation.town_resources.clone();
    assert_eq!(
        upgrade_building_instance(&content, &mut simulation, &building_id, &runtime_id, 1).unwrap(),
        2
    );
    for (resource, amount) in expected_cost {
        assert_eq!(
            simulation.town_resources[&resource],
            before[&resource] - amount
        );
    }
}

#[test]
fn selling_through_the_runtime_command_path_produces_spendable_gold() {
    let config = GameConfig::default();
    let content = embedded_content();
    let wood = StableId::new("resource:wood").unwrap();
    let ore = StableId::new("resource:ore").unwrap();
    let gold = StableId::new("resource:gold").unwrap();
    let mut simulation = WorldSimulation::new(config.world.seed);
    simulation.town_resources.insert(wood.clone(), 100);
    simulation.town_resources.insert(gold.clone(), 0);

    let ChatCommand::Sell { amount, resource } = "!sell 100 wood".parse().unwrap() else {
        panic!("sell command did not use the trade parser path");
    };
    let resource = prefixed_id(&resource, "resource:").unwrap();
    assert_eq!(
        sell_town_resource(&content, &mut simulation, &resource, amount).unwrap(),
        "sold 100 resource:wood for 13 gold"
    );
    assert_eq!(simulation.town_resources[&wood], 0);
    assert_eq!(simulation.town_resources[&gold], 13);

    let ChatCommand::Buy { amount, resource } = "!buy 24 ore".parse().unwrap() else {
        panic!("buy command did not use the trade parser path");
    };
    let resource = prefixed_id(&resource, "resource:").unwrap();
    assert_eq!(
        buy_town_resource(&config, &content, &mut simulation, resource, amount).unwrap(),
        "bought 24 resource:ore for 10 gold"
    );
    assert_eq!(simulation.town_resources[&ore], 24);
    assert_eq!(simulation.town_resources[&gold], 3);
}

#[test]
fn final_construction_tick_and_builder_repair_preserve_experience_contract() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let building_id = StableId::new("building:house").unwrap();
    let definition = &content.buildings[&building_id];
    let runtime_id = StableId::new("building:final_tick_house").unwrap();
    let position = GridPos {
        x: config.world.width / 2,
        z: config.world.height / 2,
    };
    let max_health = building_base_max_health(&content, definition);
    let mut simulation = WorldSimulation::new(world.seed);
    simulation
        .construct(
            runtime_id.clone(),
            definition.archetype.clone(),
            position,
            max_health,
            &BTreeMap::new(),
        )
        .unwrap();
    let building = simulation.buildings.get_mut(&runtime_id).unwrap();
    building.health = i32::try_from(max_health).unwrap() - 1;
    building.complete = false;

    let builder = StableId::new("npc:final_tick_builder").unwrap();
    assert!(simulation.join_player(builder.clone(), position));
    simulation
        .assign_role(&builder, StableId::new("role:builder").unwrap())
        .unwrap();
    complete_agent_goal(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &builder,
        &AgentGoal::Construct(runtime_id.clone()),
        position,
    );

    assert!(simulation.buildings[&runtime_id].complete);
    assert_eq!(
        role_progress(&simulation.actors[&builder]).experience,
        0,
        "Unity applies the last build amount, then exits without OnActionSuccess"
    );

    simulation.damage_building(&runtime_id, 1).unwrap();
    let damaged_health = simulation.buildings[&runtime_id].health;
    let experience_before_repair = role_progress(&simulation.actors[&builder]).experience;
    let (goal, _) = next_agent_goal(&simulation, &world, &config, &content, &builder, position);
    assert_eq!(goal, AgentGoal::Construct(runtime_id.clone()));
    assert!(
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &builder,
            &AgentGoal::Construct(runtime_id.clone()),
            position,
        )
        .is_some()
    );
    assert!(simulation.buildings[&runtime_id].health > damaged_health);
    assert_eq!(
        role_progress(&simulation.actors[&builder]).experience,
        experience_before_repair,
        "repairs do not award construction experience"
    );
}

#[test]
fn authored_building_nodes_follow_construction_age_and_storage_fill() {
    let config = GameConfig::default();
    let content = embedded_content();
    let building_id = StableId::new("building:foodstorage").unwrap();
    let definition = &content.buildings[&building_id];
    assert_eq!(definition.model_handlers.len(), 2);
    assert_eq!(definition.storage_models.len(), 2);
    let runtime_id = StableId::new("building:model_test").unwrap();
    let mut simulation = WorldSimulation::new(42);
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id.clone(),
            archetype: definition.archetype.clone(),
            position: GridPos { x: 10, z: 10 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 0,
            complete: false,
        },
    );
    let age_one = &definition.model_handlers[0];
    let visible = |simulation: &WorldSimulation, name: &str, age| {
        building_node_visibility(
            &building_id,
            definition,
            &simulation.buildings[&runtime_id],
            age,
            name,
            &config,
            &content,
            simulation,
        )
    };
    assert_eq!(
        visible(&simulation, &age_one.construction_stages[0], 1),
        Some(true)
    );
    assert_eq!(visible(&simulation, &age_one.full_model, 1), Some(false));
    let max_health = building_max_health(&content, &simulation.buildings[&runtime_id]);
    simulation.buildings.get_mut(&runtime_id).unwrap().health = max_health / 2;
    assert_eq!(
        visible(&simulation, &age_one.construction_stages[1], 1),
        Some(true)
    );
    simulation.buildings.get_mut(&runtime_id).unwrap().health = max_health * 4 / 5;
    assert_eq!(
        visible(&simulation, &age_one.construction_stages[2], 1),
        Some(true)
    );

    let state = simulation.buildings.get_mut(&runtime_id).unwrap();
    state.health = max_health;
    state.complete = true;
    assert_eq!(visible(&simulation, &age_one.full_model, 1), Some(true));
    assert_eq!(
        visible(&simulation, &definition.model_handlers[1].full_model, 1),
        Some(false)
    );

    let storage = &definition.storage_models[0];
    let capacity = resource_storage_capacity(&config, &content, &simulation, &storage.resource);
    assert_eq!(visible(&simulation, &storage.empty_model, 1), Some(true));
    simulation
        .town_resources
        .insert(storage.resource.clone(), capacity.div_ceil(2));
    assert_eq!(
        visible(&simulation, &storage.half_full_model, 1),
        Some(true)
    );
    simulation
        .town_resources
        .insert(storage.resource.clone(), capacity);
    assert_eq!(visible(&simulation, &storage.full_model, 1), Some(true));
}

#[test]
fn windmill_rotation_uses_converted_mesh_axes_and_unity_speed() {
    let content = embedded_content();
    let windmill = &content.buildings[&StableId::new("building:windmill").unwrap()];
    let rotors = &content.archetypes[&windmill.archetype].rotating_nodes;
    assert_eq!(rotors.len(), 2);
    assert_eq!(rotors[0].age, Some(1));
    assert_eq!(rotors[0].node, "Age01_Windmill_Blades");
    assert!(Vec3::from_array(rotors[0].axis).abs_diff_eq(Vec3::Y, f32::EPSILON));
    assert!((rotors[0].degrees_per_second - 35.0).abs() < f32::EPSILON);
    assert_eq!(rotors[1].age, Some(2));
    assert_eq!(rotors[1].node, "Age02_Windmill_Blades");
    assert!(Vec3::from_array(rotors[1].axis).abs_diff_eq(Vec3::Z, f32::EPSILON));
    assert!((rotors[1].degrees_per_second - 35.0).abs() < f32::EPSILON);

    let mut age_one = Transform::default();
    apply_authored_local_rotation(&mut age_one, Vec3::Y, 35.0_f32.to_radians(), 1.0);
    assert!(
        age_one
            .rotation
            .abs_diff_eq(Quat::from_rotation_y(35.0_f32.to_radians()), 1e-6)
    );

    let mut age_two = Transform::default();
    apply_authored_local_rotation(&mut age_two, Vec3::Z, 35.0_f32.to_radians(), 0.5);
    assert!(
        age_two
            .rotation
            .abs_diff_eq(Quat::from_rotation_z(17.5_f32.to_radians()), 1e-6)
    );
}

#[test]
fn loading_icon_rotation_uses_the_authored_prefab_contract() {
    let content = embedded_content();
    let rotation = loading_icon_rotation(&content).unwrap();
    assert_eq!(rotation.hierarchy_path, LOADING_ICON_HIERARCHY_PATH);
    assert_eq!(rotation.age, None);
    assert!(Vec3::from_array(rotation.axis).abs_diff_eq(Vec3::Z, f32::EPSILON));
    assert!((rotation.degrees_per_second - 500.0).abs() < f32::EPSILON);
    assert!(
        content
            .archetypes
            .values()
            .flat_map(|archetype| &archetype.rotating_nodes)
            .count()
            == 3
    );
    let mut transform = UiTransform::IDENTITY;
    apply_loading_icon_rotation(&mut transform, 500.0_f32.to_radians(), 0.1);
    assert!((transform.rotation.as_degrees() + 50.0).abs() < 1e-5);
    let mut loading = WorldLoadingRuntime::default();
    assert!(!advance_loading_runtime(&mut loading, 0.1));
    assert_eq!(loading.phase, WorldLoadingPhase::Presenting);
    loading.phase = WorldLoadingPhase::Loading;
    loading.progress = 0.5;
    assert!(!advance_loading_runtime(&mut loading, 0.1));
    assert!((loading.progress - 0.5).abs() < f32::EPSILON);
    loading.phase = WorldLoadingPhase::Complete;
    loading.progress = 1.0;
    loading.completion_remaining_seconds = 0.5;
    assert!(!advance_loading_runtime(&mut loading, 0.49));
    assert!(advance_loading_runtime(&mut loading, 0.02));
    let presentation = embedded_presentation();
    for source_path in [
        LOADING_SCREEN_TEXTURE_PATH,
        LOADING_OVERLAY_TEXTURE_PATH,
        LOADING_ICON_TEXTURE_PATH,
    ] {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path)
        );
    }
}

#[test]
fn loading_progress_is_recursively_derived_from_real_work() {
    assert!(loading_display_percent(0.0).abs() < f32::EPSILON);
    assert!((loading_display_percent(0.999) - 99.9).abs() < f32::EPSILON);
    assert!((loading_display_percent(1.0) - 100.0).abs() < f32::EPSILON);

    let mut work = WorldLoadingWork::default();
    assert!(work.progress().abs() < f32::EPSILON);
    work.mark_cover_complete();
    assert!((work.progress() - 0.2).abs() < f32::EPSILON);

    work.world_assets = LoadingWork::count(5, 10);
    work.generation_completed
        .extend(WorldGenerationStage::ALL.into_iter().take(2));
    let expected_world_input = (0.5 + 0.4) * 0.5;
    let expected_root = (1.0 + expected_world_input) / 5.0;
    assert!((work.progress() - expected_root).abs() < 1e-6);

    work.world_assets = LoadingWork::count(10, 10);
    work.generation_completed.extend(WorldGenerationStage::ALL);
    work.terrain_entities = LoadingWork::count(2, 2);
    work.resource_entities = LoadingWork::count(300, 300);
    work.foliage_entities = LoadingWork::count(900, 900);
    work.actor_entities = LoadingWork::count(5, 5);
    work.gameplay_setup = LoadingWork::boolean(true);
    work.scene_roots = LoadingWork::count(205, 205);
    work.scene_stable_frames = LoadingWork::count(5, 5);
    work.material_overrides = LoadingWork::count(420, 420);
    work.animation_receivers = LoadingWork::count(5, 5);
    work.lighting_receivers = LoadingWork::count(5, 5);
    work.gpu_images = LoadingWork::count(132, 133);
    work.gpu_meshes = LoadingWork::count(252, 253);
    work.gpu_materials = LoadingWork::count(32, 33);
    work.gpu_pipelines = LoadingWork::boolean(false);
    work.selection_draw = LoadingWork::boolean(true);
    work.gpu_stable_frames = LoadingWork::count(5, 6);
    let incomplete = work.progress();
    assert!(incomplete < 1.0);
    assert!(loading_display_percent(incomplete) < 100.0);

    work.gpu_images = LoadingWork::count(133, 133);
    work.gpu_meshes = LoadingWork::count(253, 253);
    work.gpu_materials = LoadingWork::count(33, 33);
    work.gpu_pipelines = LoadingWork::boolean(true);
    work.gpu_stable_frames = LoadingWork::count(6, 6);
    assert!((work.progress() - 1.0).abs() < f32::EPSILON);
    assert!((loading_display_percent(work.progress()) - 100.0).abs() < f32::EPSILON);
}

#[test]
fn autostart_loading_handoff_stays_covered_and_uses_one_progress_range() {
    let loading = MenuLoadingRuntime {
        started_at: Instant::now(),
        destination: BootDestination::WorldLoading,
        progress: 1.0,
        status: String::new(),
        substatus: String::new(),
        asset_handles: Vec::new(),
        loaded_assets: 0,
        failed_assets: 0,
        ready_presented_frames: 3,
    };
    assert!((boot_loading_display_progress(&loading) - 0.2).abs() < f32::EPSILON);
    assert!(next_state_targets_world_loading(&NextState::Pending(
        GameState::WorldLoading
    )));
    assert!(next_state_targets_world_loading(&NextState::PendingIfNeq(
        GameState::WorldLoading
    )));
    assert!(!next_state_targets_world_loading(&NextState::Pending(
        GameState::MainMenu
    )));
    assert!(!next_state_targets_world_loading(&NextState::Unchanged));
}

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
        "migrated/models/Models/Events/FishGod.glb"
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
        "migrated/models/Models/Critters/Critter_Fish3.glb"
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
            .join("migrated/models/Models/Resources")
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
        ron::from_str(include_str!("../../../assets/content/main_menu_scene.ron")).unwrap();
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
    let shader = include_str!("../../../assets/shaders/tree_material.wgsl");
    let prepass = include_str!("../../../assets/shaders/tree_material_prepass.wgsl");
    let shared_wind = include_str!("../../../assets/shaders/tree_wind.wgsl");
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
    let prepass = include_str!("../../../assets/shaders/grass_material_prepass.wgsl");
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
    let prepass = include_str!("../../../assets/shaders/critter_material_prepass.wgsl");
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

    let shader = include_str!("../../../assets/shaders/character_material.wgsl");
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

    let shader = include_str!("../../../assets/shaders/flag_material.wgsl");
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

    let shader = include_str!("../../../assets/shaders/godray_material.wgsl");
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

    let shader = include_str!("../../../assets/shaders/giraffe_material.wgsl");
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
    let shader = include_str!("../../../assets/shaders/bounds_material.wgsl");
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

#[test]
fn agent_facing_matches_unity_rotation_and_action_targets() {
    let mut smooth = Transform::default();
    rotate_agent_toward(&mut smooth, Vec3::X * 4.0, 0.1, false, false);
    let smooth_visible_forward = smooth.rotation * Vec3::Z;
    assert!(smooth_visible_forward.dot(Vec3::X) > 0.7);
    assert!(smooth_visible_forward.dot(Vec3::X) < 0.999);

    let mut snapped = Transform::default();
    rotate_agent_toward(&mut snapped, Vec3::X * 4.0, 0.1, true, false);
    assert!((snapped.rotation * Vec3::Z).dot(Vec3::X) > 0.999);

    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world_with_content(&config.world, &content);
    let mut simulation = WorldSimulation::new(world.seed);
    let actor_id = StableId::new("npc:facing_actor").unwrap();
    let target_id = StableId::new("npc:facing_target").unwrap();
    assert!(simulation.join_player(actor_id, GridPos { x: 4, z: 4 }));
    assert!(simulation.join_player(target_id.clone(), GridPos { x: 9, z: 7 }));
    assert_eq!(
        agent_action_facing_grid(&AgentGoal::Heal(target_id), &content, &simulation, &world,),
        Some(GridPos { x: 9, z: 7 })
    );

    let resource = world
        .resources
        .iter()
        .find(|resource| resource.amount > 0)
        .unwrap();
    assert_eq!(
        agent_action_facing_grid(
            &AgentGoal::Gather(resource.id.clone()),
            &content,
            &simulation,
            &world,
        ),
        Some(resource.position)
    );

    let building_id = StableId::new("building:facing_construction").unwrap();
    let building_definition = &content.buildings[&StableId::new("building:townhall").unwrap()];
    simulation.buildings.insert(
        building_id.clone(),
        BuildingState {
            id: building_id.clone(),
            archetype: building_definition.archetype.clone(),
            position: GridPos { x: 12, z: 4 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 1,
            complete: false,
        },
    );
    let origin = GridPos { x: 4, z: 4 };
    let agent = Agent {
        id: StableId::new("npc:waiting_builder").unwrap(),
        kind: ActorKind::Player,
        archetype: StableId::new("archetype:waiting_builder").unwrap(),
        goal: AgentGoal::Construct(building_id),
        spawn: origin,
        origin,
        navigation_position: placement_to_navigation_centre(origin),
        path: vec![origin],
        path_index: 1,
        target: origin,
        action_cooldown_seconds: 1.0,
        action_started: true,
        repath_remaining_seconds: 0.0,
        health_regen_accumulator: 0.0,
        wander_sequence: 0,
        previous_wander_origin: None,
    };
    let mut transform =
        Transform::from_translation(grid_to_world_on_surface(origin, &config, &world));
    for _ in 0..10 {
        assert!(rotate_agent_toward_action(
            &mut transform,
            &agent,
            &content,
            &simulation,
            &world,
            &config,
            0.05,
            false,
        ));
    }
    let target = grid_to_world_on_surface(
        agent_action_facing_grid(&agent.goal, &content, &simulation, &world).unwrap(),
        &config,
        &world,
    );
    let expected = (target - transform.translation)
        .with_y(0.0)
        .normalize_or_zero();
    assert!((transform.rotation * Vec3::Z).dot(expected) > 0.99);
}

#[test]
fn necromancer_tower_godray_binding_and_converted_vertex_colours_are_packaged() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let archetype = &content.archetypes[&StableId::new("archetype:building:necrotower").unwrap()];
    let scene = archetype
        .scenes
        .iter()
        .find(|scene| scene.age == Some(2))
        .unwrap();
    let binding = presentation.prefab_renderer_materials[&archetype.source_guid]
        .iter()
        .find(|binding| binding.target_path.ends_with("Env_Godrays_08"))
        .unwrap();
    let material_id = &binding.materials["VFX_Godrays"];
    assert_eq!(
        presentation.materials[material_id].source_path,
        GODRAY_MATERIAL_PATH
    );

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
    let godray_material_index = document["materials"]
        .as_array()
        .unwrap()
        .iter()
        .position(|material| material["name"].as_str() == Some("God_rays"))
        .expect("converted necromancer tower retains the God_rays material");
    let ray_primitive = document["meshes"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|mesh| mesh["primitives"].as_array().unwrap())
        .find(|primitive| primitive["material"].as_u64() == Some(godray_material_index as u64))
        .expect("converted necromancer tower contains its God_rays primitive");
    assert!(ray_primitive["attributes"].get("COLOR_0").is_some());
}

#[test]
fn main_menu_logo_resolves_the_shipping_drop_shadow_sprite() {
    let presentation = embedded_presentation();
    let logo = presentation
        .textures
        .values()
        .find(|texture| texture.source_path == GAME_LOGO_TEXTURE_PATH)
        .expect("shipping main-menu logo is packaged");
    assert_eq!(
        logo.asset_path,
        "migrated/textures/Sprites/Miscellaneous/Game_Logo_DropShadow.png"
    );
    assert!((GAME_LOGO_ASPECT_RATIO - 1.669_111_7).abs() < 0.000_001);
    assert!(presentation_texture_handle(&presentation, None, GAME_LOGO_TEXTURE_PATH).is_none());
}

#[test]
fn shipping_top_bar_assets_and_metric_formatting_are_complete() {
    let presentation = embedded_presentation();
    for source_path in TOP_BAR_TEXTURE_PATHS {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path),
            "missing top-bar texture {source_path}"
        );
    }
    assert_eq!(hud_play_time(0.0), "00:00");
    assert_eq!(hud_play_time(125.9), "02:05");
    let seconds_per_day = 120;
    assert!(hud_season_meter_percent(0.0, seconds_per_day).abs() <= f32::EPSILON);
    assert!(
        (hud_season_meter_percent(
            f64::from(seconds_per_day * DAYS_PER_SEASON),
            seconds_per_day,
        ) - 24.0)
            .abs()
            <= f32::EPSILON
    );
    assert!(
        hud_season_meter_percent(
            f64::from(seconds_per_day * DAYS_PER_SEASON * SEASONS_PER_YEAR),
            seconds_per_day,
        )
        .abs()
            <= f32::EPSILON
    );
    assert_eq!(
        season_visual_blend(
            f64::from(seconds_per_day * DAYS_PER_SEASON),
            seconds_per_day,
            Season::Summer,
        ),
        (Season::Spring, Season::Summer, 0.0)
    );
    let (_, _, halfway) = season_visual_blend(
        f64::from(seconds_per_day * DAYS_PER_SEASON) + SEASON_TRANSITION_SECONDS * 0.5,
        seconds_per_day,
        Season::Summer,
    );
    assert!((halfway - 0.5).abs() < 0.001);

    let content = embedded_content();
    let mut simulation = WorldSimulation::new(17);
    assert_eq!(
        hud_technology_summary(&content, &simulation),
        HudTechnologySummary {
            title: "TECHNOLOGY".to_owned(),
            requirement: "Ballot pending".to_owned(),
            progress: 0.0,
        }
    );
    let (technology_id, technology) = content
        .technology
        .nodes
        .iter()
        .find(|(_, technology)| !technology.objectives.is_empty())
        .expect("shipping technology contains an objective");
    simulation
        .start_technology_vote(technology_id.clone(), TECHNOLOGY_VOTE_DURATION_SECONDS)
        .unwrap();
    assert_eq!(
        hud_technology_summary(&content, &simulation),
        HudTechnologySummary {
            title: "NEXT TECHNOLOGY".to_owned(),
            requirement: "Vote in chat now".to_owned(),
            progress: 0.0,
        }
    );
    simulation.active_vote = None;
    assert!(simulation.start_technology_goal(
        technology_id.clone(),
        &technology.objectives,
        &content.objectives,
        MAX_TOWN_GOALS,
    ));
    let objective = hud_technology_summary(&content, &simulation);
    assert!(objective.requirement.contains("0/"));
    assert!(objective.progress.abs() <= f32::EPSILON);
}

#[test]
fn level_two_technologies_are_the_first_max_level_caps_after_unlocked_roots() {
    let content = embedded_content();
    for (display_name, building) in [
        ("Level2Stonemason", "building:stonemason"),
        ("Level2TownHall", "building:townhall"),
    ] {
        let building = StableId::new(building).unwrap();
        let technology = content
            .technology
            .nodes
            .values()
            .find(|technology| technology.display_name == display_name)
            .expect("shipping max-level technology");
        assert_eq!(technology.building_level_caps.get(&building), Some(&2));
        assert_eq!(technology.prerequisites.len(), 1);
        assert!(content.technology.nodes[&technology.prerequisites[0]].initially_unlocked);
    }
    assert_eq!(
        compact_technology_label("Level2Stonemason").replace('\n', " "),
        "Stonemason Max Lv 2"
    );
}

#[test]
fn shipping_hud_has_no_clickable_gameplay_controls() {
    fn spawn_test_hud(mut commands: Commands, render: Res<RenderAssets>) {
        spawn_hud(
            &mut commands,
            &render,
            3,
            "0123456789abcdef0123456789abcdef",
        );
    }

    let mut render = RenderAssets::default();
    render.ui_slicers.insert(
        VOTE_TEXTURE_PATHS[0].to_owned(),
        TextureSlicer {
            border: BorderRect::all(158.0),
            center_scale_mode: default(),
            sides_scale_mode: default(),
            max_corner_scale: 1.0,
        },
    );
    let mut app = App::new();
    app.insert_resource(render)
        .add_systems(Startup, spawn_test_hud);
    app.update();

    let mut buttons = app
        .world_mut()
        .query_filtered::<Entity, (With<Button>, With<WorldEntity>)>();
    assert_eq!(buttons.iter(app.world()).count(), 0);
    let mut primary_huds = app.world_mut().query::<&Hud>();
    assert_eq!(primary_huds.iter(app.world()).count(), 1);
    let mut command_guidance = app.world_mut().query::<&HudCommandGuidance>();
    assert_eq!(command_guidance.iter(app.world()).count(), 1);
    assert!(
        !app.world()
            .contains_resource::<PointerObjectSelectionEnabled>()
    );
    let mut strips = app
        .world_mut()
        .query_filtered::<&Node, Or<(With<HudResourceStrip>, With<HudStatsStrip>)>>();
    let strips = strips.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(strips.len(), 2);
    for strip in strips {
        assert_eq!(strip.top, percent(0.0));
        assert_eq!(strip.height, percent(100.0));
    }
    let mut rows = app
        .world_mut()
        .query_filtered::<&Node, With<HudMetricRow>>();
    let rows = rows.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(rows.len(), 8);
    assert!(rows.iter().all(|row| row.align_items == AlignItems::Center));
    assert!(rows.iter().all(|row| row.height == px(44)));
    let mut metrics = app.world_mut().query::<&HudMetric>();
    let metrics = metrics.iter(app.world()).copied().collect::<Vec<_>>();
    assert!(metrics.contains(&HudMetric::Players));
    assert!(metrics.contains(&HudMetric::Npcs));
    let mut maximums = app.world_mut().query::<&HudMetricMaximum>();
    let maximums = maximums
        .iter(app.world())
        .map(|maximum| maximum.0)
        .collect::<Vec<_>>();
    assert_eq!(maximums.len(), 5);
    assert!(maximums.contains(&HudMetric::Food));
    assert!(maximums.contains(&HudMetric::Gold));
    assert!(maximums.contains(&HudMetric::Ore));
    assert!(maximums.contains(&HudMetric::Wood));
    assert!(maximums.contains(&HudMetric::Npcs));
    let resource_strip = app
        .world_mut()
        .query_filtered::<&Node, With<HudResourceStrip>>()
        .single(app.world())
        .unwrap();
    assert_eq!(resource_strip.left, px(18));
    let event_panel = app
        .world_mut()
        .query_filtered::<&Node, With<CurrentEventPanel>>()
        .single(app.world())
        .unwrap();
    assert_eq!(event_panel.top, px(76));
    assert_eq!(event_panel.left, px(18));
    assert_eq!(event_panel.width, px(290));
    assert_eq!(event_panel.height, px(104));
    let (objective_panel, objective_panel_image) = app
        .world_mut()
        .query_filtered::<(&Node, Option<&ImageNode>), With<HudTechnologyObjectivePanel>>()
        .single(app.world())
        .unwrap();
    assert_eq!(objective_panel.width, percent(45.0));
    assert_eq!(objective_panel.overflow, Overflow::clip());
    assert!(objective_panel_image.is_none());
    let top_bar_image = app
        .world_mut()
        .query_filtered::<&ImageNode, With<HudTopBar>>()
        .single(app.world())
        .unwrap();
    assert!(matches!(top_bar_image.image_mode, NodeImageMode::Sliced(_)));
    let mut technology_texts = app.world_mut().query::<(&HudTechnologyTextKind, &Text)>();
    let technology_texts = technology_texts.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(technology_texts.len(), 2);
    assert!(
        technology_texts.iter().any(|(kind, text)| {
            **kind == HudTechnologyTextKind::Title && text.0 == "TECHNOLOGY"
        })
    );
    assert!(technology_texts.iter().any(|(kind, text)| {
        **kind == HudTechnologyTextKind::Requirement && text.0 == "Ballot pending"
    }));
}

#[test]
fn shipping_vote_panels_preserve_art_timers_and_deterministic_tallies() {
    let presentation = embedded_presentation();
    for source_path in VOTE_TEXTURE_PATHS
        .into_iter()
        .chain(std::iter::once(RULER_VOTE_TIMER_UNFILLED_PATH))
    {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path),
            "missing vote texture {source_path}"
        );
    }
    assert_eq!(vote_timer_text(0.0), "00:00");
    assert_eq!(vote_timer_text(30.01), "00:31");
    assert_eq!(vote_timer_text(120.0), "02:00");
    assert!((TECHNOLOGY_VOTE_DURATION_SECONDS - 60.0).abs() <= f32::EPSILON);

    let mut simulation = WorldSimulation::new(17);
    let first = StableId::new("viewer:first").unwrap();
    let second = StableId::new("viewer:second").unwrap();
    simulation.join_player(first.clone(), GridPos { x: 1, z: 1 });
    simulation.join_player(second.clone(), GridPos { x: 2, z: 2 });
    simulation
        .start_technology_vote(StableId::new("tech:test_vote").unwrap(), 30.0)
        .unwrap();
    simulation.cast_vote(&first, true).unwrap();
    simulation.cast_vote(&second, false).unwrap();
    assert_eq!(technology_vote_tally(&simulation), Some((1, 2, 0.5)));

    simulation.active_vote = None;
    simulation
        .start_ruler_vote(RulerVoteKind::NewRuler)
        .unwrap();
    simulation.cast_ruler_vote(&first, second.clone()).unwrap();
    simulation.actors.get_mut(&second).unwrap().display_name = Some("Second Viewer".to_owned());
    assert_eq!(ruler_vote_option_text(&simulation), "Second Viewer  (1)");
    assert!((ruler_vote_option_font_size("Amy  (1)") - 17.0).abs() < f32::EPSILON);
    assert!(ruler_vote_option_font_size("A Very Long Twitch Display Name  (12)") < 17.0);
}

#[test]
fn generated_world_starts_numbered_technology_vote_after_unity_delay() {
    fn spawn_test_vote_panels(mut commands: Commands, render: Res<RenderAssets>) {
        spawn_vote_panels(&mut commands, &render);
    }

    let content = embedded_content();
    let mut simulation = WorldSimulation::new(17);
    simulation.unlocked_technology.extend(
        content
            .technology
            .nodes
            .iter()
            .filter(|(_, technology)| technology.initially_unlocked)
            .map(|(id, _)| id.clone()),
    );
    let seconds_per_day = GameConfig::default().time.seconds_per_day;
    simulation.tick(19.0, seconds_per_day);
    assert!(simulation.active_vote.is_none());
    assert_eq!(simulation.technology_vote_cooldown_seconds, Some(1.0));
    simulation.tick(1.0, seconds_per_day);
    let eligible_before_vote = eligible_technology_ids(&content, &simulation);
    let depths = technology_depths(&content);
    let deepest_available = eligible_before_vote
        .iter()
        .filter_map(|technology| depths.get(technology))
        .max()
        .copied()
        .unwrap();
    let shallowest_available = eligible_before_vote
        .iter()
        .filter_map(|technology| depths.get(technology))
        .min()
        .copied()
        .unwrap();
    simulation
        .start_ruler_vote(RulerVoteKind::NewRuler)
        .expect("ruler ballot may overlap the scheduled technology ballot");

    let mut app = App::new();
    app.insert_resource(RuntimeContent(content.clone()))
        .insert_resource(SimulationRuntime(simulation))
        .insert_resource(RuntimePresentation(embedded_presentation()))
        .insert_resource(RenderAssets::default())
        .add_systems(Startup, spawn_test_vote_panels)
        .add_systems(
            Update,
            (start_scheduled_technology_vote, update_vote_panels).chain(),
        );
    app.update();

    let vote = app
        .world()
        .resource::<SimulationRuntime>()
        .0
        .active_vote
        .as_ref()
        .expect("the generated-world technology ballot starts after Unity's delay");
    assert!(
        app.world()
            .resource::<SimulationRuntime>()
            .0
            .ruler_vote
            .is_some(),
        "starting the technology ballot must not dismiss the ruler ballot"
    );
    assert_eq!(vote.options.len(), TECHNOLOGY_VOTE_OPTION_COUNT);
    assert_eq!(vote.technology, vote.options[0]);
    assert_eq!(depths[&vote.options[0]], deepest_available);
    assert_eq!(depths[&vote.options[1]], shallowest_available);
    assert_eq!(
        vote.options.iter().collect::<BTreeSet<_>>().len(),
        vote.options.len()
    );
    assert!(vote.option_votes.is_empty());
    assert!((vote.remaining_seconds - TECHNOLOGY_VOTE_DURATION_SECONDS).abs() <= f32::EPSILON);
    let vote_options = vote.options.clone();
    let announcement = technology_vote_announcement(&content, vote);
    assert!(announcement.starts_with("Technology vote started! 1:"));
    for index in 1..=TECHNOLOGY_VOTE_OPTION_COUNT {
        assert!(announcement.contains(&format!("!vote {index}")));
    }
    let mut vote_tags = app.world_mut().query::<(&VoteTextKind, &Text)>();
    let vote_tags = vote_tags
        .iter(app.world())
        .filter_map(|(kind, text)| {
            let VoteTextKind::TechnologyOptionDepthTag(index) = kind else {
                return None;
            };
            Some((*index, text.0.clone()))
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(vote_tags.get(&0).map(String::as_str), Some("Specialized"));
    assert_eq!(vote_tags.get(&1).map(String::as_str), Some("Fundamental"));
    assert_eq!(vote_tags.get(&2).map(String::as_str), Some(""));
    let mut depth_badges = app.world_mut().query::<(
        &TechnologyVoteDepthBadge,
        &Visibility,
        &Node,
        &BackgroundColor,
    )>();
    let depth_badges = depth_badges.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(depth_badges.len(), TECHNOLOGY_VOTE_OPTION_COUNT);
    for (badge, visibility, node, background) in depth_badges {
        assert_eq!(node.left, px(-78));
        assert_eq!(node.top, px(4));
        assert_eq!(node.width, px(80));
        assert_eq!(node.height, px(20));
        assert_eq!(node.border_radius, BorderRadius::all(px(10)));
        assert_eq!(node.align_items, AlignItems::Center);
        assert_eq!(node.justify_content, JustifyContent::Center);
        assert_eq!(background.0, Color::srgb(0.96, 0.76, 0.20));
        assert_eq!(
            *visibility,
            if badge.0 < 2 {
                Visibility::Visible
            } else {
                Visibility::Hidden
            }
        );
    }
    let mut panels = app
        .world_mut()
        .query_filtered::<(&VotePanelKind, &Visibility, &Node), Without<TechnologyVoteOptionRow>>();
    assert!(panels.iter(app.world()).any(|(kind, visibility, node)| {
        *kind == VotePanelKind::Technology
            && *visibility == Visibility::Visible
            && node.width == px(260)
            && node.height == px(TECHNOLOGY_VOTE_PANEL_HEIGHT)
    }));
    let mut rows = app
        .world_mut()
        .query::<(&TechnologyVoteOptionRow, &Visibility, &Node)>();
    assert_eq!(
        rows.iter(app.world())
            .filter(|(_, visibility, _)| **visibility == Visibility::Visible)
            .count(),
        TECHNOLOGY_VOTE_OPTION_COUNT
    );
    let mut positioned_rows = rows.iter(app.world()).collect::<Vec<_>>();
    positioned_rows.sort_by_key(|(row, _, _)| row.0);
    let mut expected_top = TECHNOLOGY_VOTE_FIRST_ROW_TOP;
    for (row, _, node) in positioned_rows {
        assert_eq!(node.top, px(expected_top));
        let requirement_lines = vote_options
            .get(usize::from(row.0))
            .and_then(|technology| content.technology.nodes.get(technology))
            .map_or(1, |technology| technology.objectives.len().max(1));
        expected_top += technology_vote_row_advance(requirement_lines);
    }
    let mut vote_texts = app.world_mut().query::<(&VoteTextKind, &Text)>();
    let requirements = vote_texts
        .iter(app.world())
        .filter_map(|(kind, text)| {
            let VoteTextKind::TechnologyOptionRequirements(index) = kind else {
                return None;
            };
            Some((*index, text.0.as_str()))
        })
        .collect::<Vec<_>>();
    assert_eq!(requirements.len(), TECHNOLOGY_VOTE_OPTION_COUNT);
    for (index, text) in requirements {
        let objective_count = vote_options
            .get(usize::from(index))
            .and_then(|technology| content.technology.nodes.get(technology))
            .map_or(0, |technology| technology.objectives.len());
        assert_eq!(text.lines().count(), objective_count.max(1));
        assert!(!text.contains("Requires:"));
        assert!(!text.contains('•'));
    }
    let mut all_text = app.world_mut().query::<&Text>();
    let all_text = all_text.iter(app.world()).collect::<Vec<_>>();
    assert!(all_text.iter().any(|text| text.0 == "TECHNOLOGY VOTE"));
    assert!(
        all_text
            .iter()
            .all(|text| { !text.0.contains("Vote through chat") && !text.0.contains("!vote") })
    );
    let mut vote_fonts = app.world_mut().query::<(&VoteTextKind, &TextFont)>();
    for (kind, font) in vote_fonts.iter(app.world()) {
        let expected = match kind {
            VoteTextKind::TechnologyTitle => Some(18.0),
            VoteTextKind::TechnologyOptionTitle(_) => Some(13.5),
            VoteTextKind::TechnologyOptionRequirements(_) => Some(11.0),
            VoteTextKind::TechnologyTimer => Some(16.0),
            _ => None,
        };
        if let Some(expected) = expected {
            assert_eq!(font.font_size, FontSize::Px(expected));
        }
    }
    let mut vote_text_nodes = app.world_mut().query::<(&VoteTextKind, &Node)>();
    for (kind, node) in vote_text_nodes.iter(app.world()) {
        match kind {
            VoteTextKind::TechnologyTitle => {
                assert_eq!(node.top, px(TECHNOLOGY_VOTE_TITLE_TOP));
            }
            VoteTextKind::TechnologyTimer => {
                assert_eq!(node.width, Val::Auto);
                assert_eq!(node.top, Val::Auto);
            }
            _ => {}
        }
    }
    let mut vote_text_colors = app.world_mut().query::<(&VoteTextKind, &TextColor)>();
    let timer_color = vote_text_colors
        .iter(app.world())
        .find_map(|(kind, color)| (*kind == VoteTextKind::TechnologyTimer).then_some(color.0))
        .expect("technology timer text has a colour");
    assert_eq!(timer_color, Color::srgb(0.06, 0.08, 0.16));
    let mut vote_text_transforms = app.world_mut().query::<(&VoteTextKind, &UiTransform)>();
    let timer_transform = vote_text_transforms
        .iter(app.world())
        .find_map(|(kind, transform)| {
            (*kind == VoteTextKind::TechnologyTimer).then_some(*transform)
        })
        .expect("technology timer text has a transform");
    assert_eq!(
        timer_transform.translation,
        Val2::new(px(0), px(TECHNOLOGY_VOTE_TIMER_TEXT_OFFSET_Y))
    );
    let mut timer_tracks = app
        .world_mut()
        .query_filtered::<&Node, With<TechnologyVoteTimerTrack>>();
    let timer_tracks = timer_tracks.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(timer_tracks.len(), 1);
    assert_eq!(timer_tracks[0].top, px(0));
    assert_eq!(timer_tracks[0].left, px(0));
    assert_eq!(timer_tracks[0].right, px(0));
    assert_eq!(timer_tracks[0].bottom, Val::Auto);
    let mut timer_labels = app
        .world_mut()
        .query_filtered::<&Node, With<TechnologyVoteTimerLabel>>();
    let timer_labels = timer_labels.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(timer_labels.len(), 1);
    assert_eq!(timer_labels[0].top, px(0));
    assert_eq!(timer_labels[0].left, px(0));
    assert_eq!(timer_labels[0].right, px(0));
    assert_eq!(timer_labels[0].height, px(20));
    assert_eq!(timer_labels[0].align_items, AlignItems::Center);
    assert_eq!(timer_labels[0].justify_content, JustifyContent::Center);
    let mut timer_groups = app
        .world_mut()
        .query_filtered::<&Node, With<TechnologyVoteTimerGroup>>();
    let timer_groups = timer_groups.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(timer_groups.len(), 1);
    assert_eq!(timer_groups[0].top, px(TECHNOLOGY_VOTE_TIMER_GROUP_TOP));
    assert_eq!(timer_groups[0].left, px(0));
    assert_eq!(timer_groups[0].right, px(0));
    assert_eq!(timer_groups[0].height, px(24));
    assert_eq!(timer_groups[0].align_items, AlignItems::Center);
    assert_eq!(timer_groups[0].justify_content, JustifyContent::Center);
    let mut timer_bars = app
        .world_mut()
        .query_filtered::<&Node, With<TechnologyVoteTimerBar>>();
    let timer_bars = timer_bars.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(timer_bars.len(), 1);
    assert_eq!(timer_bars[0].width, px(TECHNOLOGY_VOTE_TIMER_BAR_WIDTH));
    assert_eq!(timer_bars[0].height, px(20));
    let mut timer_icons = app
        .world_mut()
        .query_filtered::<&Node, With<TechnologyVoteTimerIcon>>();
    let timer_icons = timer_icons.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(timer_icons.len(), 1);
    assert_eq!(timer_icons[0].width, px(24));
    assert_eq!(timer_icons[0].height, px(24));
    let mut timer_gaps = app
        .world_mut()
        .query_filtered::<&Node, With<TechnologyVoteTimerGap>>();
    let timer_gaps = timer_gaps.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(timer_gaps.len(), 1);
    assert_eq!(timer_gaps[0].width, px(TECHNOLOGY_VOTE_TIMER_GAP));
    let mut title_bars = app.world_mut().query::<(&TechnologyVoteTitleBar, &Node)>();
    let title_bars = title_bars.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(title_bars.len(), TECHNOLOGY_VOTE_OPTION_COUNT);
    for (_, node) in title_bars {
        assert_eq!(node.top, px(TECHNOLOGY_VOTE_LABEL_OFFSET_Y));
        assert_eq!(node.height, px(28));
        assert_eq!(node.align_items, AlignItems::Center);
        assert_eq!(node.justify_content, JustifyContent::Center);
    }
    let mut vote_icons = app.world_mut().query::<(&TechnologyVoteIcon, &Node)>();
    let vote_icons = vote_icons.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(vote_icons.len(), TECHNOLOGY_VOTE_OPTION_COUNT);
    assert!(
        vote_icons
            .iter()
            .all(|(_, node)| node.top == px(TECHNOLOGY_VOTE_ICON_TOP))
    );

    {
        let mut simulation = app.world_mut().resource_mut::<SimulationRuntime>();
        simulation.0.active_vote = None;
        simulation.0.technology_vote_cooldown_seconds = Some(999.0);
    }
    app.update();
    let mut rows = app
        .world_mut()
        .query::<(&TechnologyVoteOptionRow, &Visibility)>();
    assert!(
        rows.iter(app.world())
            .all(|(_, visibility)| *visibility == Visibility::Hidden)
    );
    let mut fills = app.world_mut().query::<(&VoteFillKind, &Node)>();
    assert!(fills.iter(app.world()).all(|(kind, node)| {
        !matches!(
            kind,
            VoteFillKind::TechnologyTimer | VoteFillKind::TechnologyOption(_)
        ) || node.width == percent(0.0)
    }));
}

#[test]
fn actor_nameplates_identify_twitch_players_without_tagging_npcs() {
    assert!(is_stream_player_actor(
        &StableId::new("twitch:human_bean").unwrap()
    ));
    assert!(!is_stream_player_actor(
        &StableId::new("npc:starting_defender").unwrap()
    ));
    assert!(!is_stream_player_actor(
        &StableId::new("npc:recruit_1").unwrap()
    ));
    assert_eq!(
        PlayerSettings::default().interface.display_names,
        NameDisplayMode::AllPlayers
    );
    let health_bottom = ACTOR_HEALTH_OVERLAY_TOP_PX + 10.0;
    assert!(health_bottom < ACTOR_NAME_OVERLAY_TOP_PX);
}

#[test]
fn ruler_vote_results_produce_the_chat_announcement_that_tick_discards() {
    let mut simulation = WorldSimulation::new(17);
    let winner = StableId::new("viewer:winner").unwrap();
    simulation.join_player(winner.clone(), GridPos { x: 1, z: 1 });
    simulation.actors.get_mut(&winner).unwrap().display_name = Some("Town Hero".to_owned());
    simulation.current_ruler = Some(winner.clone());
    assert_eq!(
        ruler_vote_result_announcement(RulerVoteKind::NewRuler, None, &simulation),
        Some("Town Hero has been elected Ruler of Stream Town!".to_owned())
    );
    assert_eq!(
        ruler_vote_result_announcement(RulerVoteKind::KeepRuler, Some(&winner), &simulation,),
        Some("Chat has chosen to keep Town Hero as Ruler of Stream Town!".to_owned())
    );
    simulation.current_ruler = None;
    assert_eq!(
        ruler_vote_result_announcement(RulerVoteKind::KeepRuler, Some(&winner), &simulation,),
        Some("Town Hero was not retained as Ruler. Nominations are now open.".to_owned())
    );
}

#[test]
fn shipping_top_bar_technology_progress_uses_objective_art_and_unity_labels() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    for source_path in OBJECTIVE_TEXTURE_PATHS {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path),
            "missing objective texture {source_path}"
        );
    }
    let (technology, node) = content
        .technology
        .nodes
        .iter()
        .find(|(_, technology)| !technology.objectives.is_empty())
        .expect("shipping technology graph has objective-backed goals");
    let objective = &content.objectives[&node.objectives[0]];
    let label = objective_display_label(objective);
    assert!(!label.is_empty());
    assert!(!label.contains("resource:"));
    assert!(!label.contains("building:"));
    assert!(!label.contains("enemy:"));
    assert!((objective_progress_ratio(500, 1_000) - 0.5).abs() <= f32::EPSILON);
    assert!((objective_progress_ratio(70_000, 100_000) - 0.7).abs() <= f32::EPSILON);
    assert!((objective_progress_ratio(2, 1) - 1.0).abs() <= f32::EPSILON);
    assert!((objective_progress_ratio(0, 0) - 1.0).abs() <= f32::EPSILON);

    let mut simulation = WorldSimulation::new(17);
    assert!(simulation.start_technology_goal(
        technology.clone(),
        &node.objectives,
        &content.objectives,
        MAX_TOWN_GOALS,
    ));
    let summary = hud_technology_summary(&content, &simulation);
    assert_ne!(summary.title, "TECHNOLOGY");
    assert!(summary.requirement.contains("0/"));
    assert!(summary.progress.abs() <= f32::EPSILON);
}

#[test]
fn shipping_current_event_panel_maps_encounters_and_omits_redundant_active_label() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    for source_path in CURRENT_EVENT_TEXTURE_PATHS {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path),
            "missing current-event texture {source_path}"
        );
    }
    let mut simulation = WorldSimulation::new(17);
    assert!(simulation.start_fish_god(true));
    simulation.fish_god.as_mut().unwrap().praises_given = 7;
    let fish = current_event_panel_state(&simulation).unwrap();
    assert_eq!(fish.0, "Fish God");
    assert_eq!(fish.2, "7  /  20");
    assert!((fish.3 - 0.35).abs() <= f32::EPSILON);

    simulation.stop_active_event();
    let enemy =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Minotaur.prefab").unwrap();
    let boss = archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_MinotaurBoss.prefab")
        .unwrap();
    assert!(simulation.start_raid(5, 50, enemy, boss));
    simulation.active_raid.as_mut().unwrap().current_wave = 2;
    let raid = current_event_panel_state(&simulation).unwrap();
    assert_eq!(raid.0, "Raid");
    assert_eq!(raid.2, "Waves Completed: 2/5");
    assert!((raid.3 - 0.4).abs() <= f32::EPSILON);
    simulation
        .active_raid
        .as_mut()
        .unwrap()
        .tracked_enemies
        .insert(StableId::new("enemy:wave").unwrap());
    let active_wave = current_event_panel_state(&simulation).unwrap();
    assert_eq!(active_wave.1, "Enemies In Wave: 1");
    assert_eq!(active_wave.2, "Waves Completed: 1/5");
    assert!((active_wave.3 - 0.2).abs() <= f32::EPSILON);
    simulation.finish_raid();
    assert!(current_event_panel_state(&simulation).is_none());
    simulation.active_community_event = Some(CommunityEvent::Market);
    let market = current_event_panel_state(&simulation).unwrap();
    assert_eq!(market.0, "Market");
    assert!(market.2.is_empty());
    assert!((market.3 - 1.0).abs() <= f32::EPSILON);
}

#[test]
fn selection_panel_uses_packaged_sliders_and_authoritative_actor_state() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    for source_path in SELECTION_PANEL_TEXTURE_PATHS {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path),
            "missing selection slider texture {source_path}"
        );
    }
    let position = GridPos { x: 4, z: 7 };
    let actor_id = StableId::new("actor:selection_test").expect("test ID");
    let mut simulation = WorldSimulation::new(17);
    simulation.join_player(actor_id.clone(), position);
    let actor = simulation.actors.get_mut(&actor_id).expect("joined actor");
    actor.display_name = Some("Selection Test".to_owned());
    actor.health = 60;
    actor.max_health = 120;
    let world = generate_world(&GameConfig::default().world);

    let details = selection_panel_details(position, &content, &world, &simulation)
        .expect("actor cell should resolve");
    assert!(details.description.contains("Selection Test"));
    assert!(details.description.contains("Level 1/1000"));
    assert!(details.description.contains("Health 60/120"));
    assert_eq!(details.health_progress, Some(0.5));
    assert_eq!(details.experience_progress, Some(0.0));
    assert_eq!(title_case("wood"), "Wood");

    let resource = world
        .resources
        .iter()
        .find(|resource| resource.amount > 0)
        .expect("generated resource");
    let resource_details = selection_panel_details(
        resource.position,
        &content,
        &world,
        &WorldSimulation::new(17),
    )
    .expect("resource cell should resolve");
    assert!(resource_details.description.contains("remaining"));
    assert_eq!(resource_details.health_progress, None);
    assert_eq!(resource_details.experience_progress, None);

    let enemy_archetype =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Minotaur.prefab")
            .expect("converted enemy archetype");
    let enemy_position = GridPos { x: 12, z: 14 };
    assert!(simulation.spawn_enemy(
        StableId::new("actor:selection_enemy").unwrap(),
        enemy_archetype,
        enemy_position,
        25,
    ));
    let enemy_details = selection_panel_details(enemy_position, &content, &world, &simulation)
        .expect("enemy cell should resolve");
    assert!(enemy_details.description.starts_with("Minotaur\n"));
    assert!(enemy_details.description.contains("Health 25/25"));
    assert_eq!(enemy_details.health_progress, Some(1.0));
    assert_eq!(enemy_details.experience_progress, None);
}

#[test]
fn selection_panel_and_outline_resolve_the_full_enemy_camp_footprint() {
    let content = embedded_content();
    let world = generate_world(&GameConfig::default().world);
    let (camp_archetype_id, camp_archetype) = content
        .archetypes
        .iter()
        .find(|(_, archetype)| archetype.enemy_spawner.is_some())
        .expect("converted enemy camp archetype");
    let camp_id = StableId::new("enemy_camp:selection_test").unwrap();
    let origin = GridPos { x: 10, z: 12 };
    let selected_cell = GridPos {
        x: origin.x + camp_archetype.footprint[0] - 1,
        z: origin.z + camp_archetype.footprint[1] - 1,
    };
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.enemy_camps.insert(
        camp_id.clone(),
        EnemyCampState {
            id: camp_id,
            archetype: camp_archetype_id.clone(),
            position: origin,
            health: 400,
            spawn_remaining_seconds: 2.0,
            spawned_enemies: BTreeSet::new(),
        },
    );

    assert_eq!(
        selected_enemy_camp_at_cell(selected_cell, &content, &simulation).map(|(camp, _)| &camp.id),
        simulation.enemy_camps.values().next().map(|camp| &camp.id)
    );
    assert_eq!(
        selected_structural_footprint(selected_cell, &content, &simulation),
        Some((origin, camp_archetype.footprint))
    );
    let details = selection_panel_details(selected_cell, &content, &world, &simulation)
        .expect("camp footprint should resolve");
    assert!(details.description.starts_with("Enemy Camp\n"));
    assert!(details.description.contains("Health 400/1000"));
    assert_eq!(details.health_progress, Some(0.4));
    assert_eq!(details.experience_progress, None);
}

#[test]
fn standalone_seagull_resolves_the_authored_critter_material() {
    let id = StableId::new(CRITTER_MATERIAL_ID).expect("critter material id is valid");
    let authored = Handle::<CritterMaterial>::default();
    let mut render = RenderAssets::default();
    render
        .presentation_materials
        .insert(id, ResolvedMaterialHandle::Critter(authored.clone()));

    let resolved = standalone_material_override(&render, CRITTER_MATERIAL_ID)
        .expect("standalone converted scene should resolve its material");
    assert!(resolved.model_materials.is_empty());
    assert!(resolved.renderer_materials.is_empty());
    let Some(ResolvedMaterialHandle::Critter(handle)) = resolved.fallback else {
        panic!("standalone seagull must receive a critter material override");
    };
    assert_eq!(handle, authored);
}

#[test]
fn generated_terrain_mesh_matches_navigation_grid() {
    let config = GameConfig::default();
    let world = generate_world(&config.world);
    let mesh = generated_terrain_mesh(&world, &config);
    let face_count =
        usize::from(config.world.width * 2 - 1) * usize::from(config.world.height * 2 - 1);
    assert_eq!(mesh.count_vertices(), face_count * 4);
    assert_eq!(mesh.indices().unwrap().len(), face_count * 6);
    assert_eq!(
        mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap().len(),
        mesh.count_vertices()
    );
    assert_eq!(
        mesh.attribute(Mesh::ATTRIBUTE_COLOR).unwrap().len(),
        mesh.count_vertices()
    );
    let bevy::mesh::VertexAttributeValues::Float32x3(normals) =
        mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap()
    else {
        panic!("terrain normals must use float triples");
    };
    assert!(
        normals.iter().all(|normal| normal[1] >= -f32::EPSILON),
        "reflected row-connector winding must not point terrain faces downward"
    );
    let centre = GridPos {
        x: config.world.width / 2,
        z: config.world.height / 2,
    };
    assert!(
        (grid_to_world_on_surface(centre, &config, &world).y - terrain_height(&world, centre))
            .abs()
            <= f32::EPSILON
    );
    let water = generated_water_mesh(&world, &config);
    let padded_width = usize::from(config.world.width + OCEAN_PADDING_CELLS * 2);
    let padded_height = usize::from(config.world.height + OCEAN_PADDING_CELLS * 2);
    assert_eq!(
        water.count_vertices(),
        (padded_width + 1) * (padded_height + 1)
    );
    assert_eq!(
        water.indices().unwrap().len(),
        padded_width * padded_height * 6
    );
    let bevy::mesh::VertexAttributeValues::Float32x4(depth_colors) =
        water.attribute(Mesh::ATTRIBUTE_COLOR).unwrap()
    else {
        panic!("water depth must use float vertex colors");
    };
    assert!(
        depth_colors
            .iter()
            .all(|color| (0.0..=1.0).contains(&color[0]))
    );
    assert!(depth_colors.iter().any(|color| color[0] == 0.0));
    assert!(depth_colors.iter().any(|color| color[0] > 0.0));
}

#[test]
fn generated_terrain_chunks_cover_the_grid_with_watertight_seams() {
    let mut config = GameConfig::default();
    config.world.width = 64;
    config.world.height = 64;
    let world = generate_world(&config.world);
    let chunks = generated_terrain_chunks(&world, &config);
    assert_eq!(chunks.len(), 16);
    assert_eq!(
        chunks
            .iter()
            .map(|chunk| chunk.high.indices().unwrap().len())
            .sum::<usize>(),
        64 * 64 * 6
    );
    assert!(chunks.iter().all(|chunk| {
        chunk.high.count_vertices() == 17 * 17
            && Collider::trimesh_from_mesh(&chunk.high).is_some()
            && chunk.medium.indices().unwrap().len() < chunk.high.indices().unwrap().len()
            && chunk.low.indices().unwrap().len() < chunk.medium.indices().unwrap().len()
    }));

    let positions = |chunk_x: u16, chunk_z: u16| {
        let mesh = &chunks
            .iter()
            .find(|chunk| chunk.chunk_x == chunk_x && chunk.chunk_z == chunk_z)
            .unwrap()
            .high;
        let bevy::mesh::VertexAttributeValues::Float32x3(positions) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
        else {
            panic!("terrain positions must use float triples");
        };
        positions.clone()
    };
    let left = positions(0, 0);
    let right = positions(1, 0);
    for row in 0..=usize::from(TERRAIN_CHUNK_CELLS) {
        let left_seam = left[row * 17 + usize::from(TERRAIN_CHUNK_CELLS)];
        let right_seam = right[row * 17];
        assert!(
            left_seam
                .into_iter()
                .zip(right_seam)
                .all(|(left, right)| { (left - right).abs() <= f32::EPSILON })
        );
    }

    let mut uneven = config.clone();
    uneven.world.width = 35;
    uneven.world.height = 19;
    let uneven_world = generate_world(&uneven.world);
    let uneven_chunks = generated_terrain_chunks(&uneven_world, &uneven);
    assert_eq!(uneven_chunks.len(), 6);
    assert_eq!(
        uneven_chunks
            .iter()
            .map(|chunk| chunk.high.indices().unwrap().len())
            .sum::<usize>(),
        35 * 19 * 6
    );
}

#[test]
fn terrain_lod_is_hysteretic_and_keeps_partial_chunk_boundaries() {
    assert_eq!(
        terrain_lod_for_distance(TerrainLodLevel::High, TERRAIN_HIGH_DETAIL_RADIUS),
        TerrainLodLevel::High
    );
    assert_eq!(
        terrain_lod_for_distance(
            TerrainLodLevel::High,
            TERRAIN_HIGH_DETAIL_RADIUS + TERRAIN_LOD_HYSTERESIS + 1.0,
        ),
        TerrainLodLevel::Medium
    );
    assert_eq!(
        terrain_lod_for_distance(TerrainLodLevel::Medium, TERRAIN_HIGH_DETAIL_RADIUS),
        TerrainLodLevel::Medium
    );
    assert_eq!(
        terrain_lod_for_distance(
            TerrainLodLevel::Medium,
            TERRAIN_HIGH_DETAIL_RADIUS - TERRAIN_LOD_HYSTERESIS - 1.0,
        ),
        TerrainLodLevel::High
    );
    assert_eq!(terrain_lod_samples(15, 4), vec![0, 4, 8, 12, 15]);
    assert_eq!(terrain_lod_samples(16, 4), vec![0, 4, 8, 12, 16]);

    let focus = camera_ground_focus(&default_town_camera_transform());
    let authored_focus = Vec2::new(
        UNITY_TOWN_CAMERA_OFFSET.x - UNITY_TOWN_CAMERA_FOCUS_BACK_SHIFT
            + UNITY_TOWN_CAMERA_OFFSET.y,
        UNITY_TOWN_CAMERA_OFFSET.z,
    );
    assert!(focus.distance(authored_focus) < 0.000_1);
}

#[test]
fn foliage_ranges_scale_without_exceeding_the_streaming_budget() {
    assert!(
        (foliage_visibility_distance(Vec3::ZERO) - FOLIAGE_VISIBILITY_MIN_RANGE).abs()
            <= f32::EPSILON
    );
    assert!(
        (foliage_visibility_distance(Vec3::splat(100.0)) - FOLIAGE_VISIBILITY_MAX_RANGE).abs()
            <= f32::EPSILON
    );
    assert!(
        foliage_visibility_distance(Vec3::splat(8.0))
            > foliage_visibility_distance(Vec3::splat(2.0))
    );
}

#[test]
fn shipping_foliage_batch_membership_is_deterministic_and_compact() {
    let config = GameConfig::default();
    let content = embedded_content();
    let first = generate_world_with_content(&config.world, &content);
    let second = generate_world_with_content(&config.world, &content);
    let first_keys = first
        .foliage
        .iter()
        .map(foliage_batch_key)
        .collect::<BTreeSet<_>>();
    let second_keys = second
        .foliage
        .iter()
        .map(foliage_batch_key)
        .collect::<BTreeSet<_>>();
    let gpu_batches = first
        .foliage
        .iter()
        .map(|foliage| (foliage.layer.clone(), foliage.variant))
        .collect::<BTreeSet<_>>();

    assert_eq!(first.deterministic_hash, second.deterministic_hash);
    assert_eq!(first_keys, second_keys);
    assert_eq!(first.foliage.len(), 37_320);
    assert_eq!(first_keys.len(), 293);
    assert_eq!(gpu_batches.len(), 12);
    assert!(first_keys.len() * 8 < first.foliage.len());
    assert!(first_keys.iter().all(|key| {
        key.chunk_x <= config.world.width.saturating_sub(1) / FOLIAGE_BATCH_CHUNK_CELLS
            && key.chunk_z <= config.world.height.saturating_sub(1) / FOLIAGE_BATCH_CHUNK_CELLS
    }));
}

#[test]
fn crowd_separation_is_deterministic_bounded_and_balanced() {
    let agents = vec![
        (StableId::new("actor:alpha").unwrap(), Vec2::new(10.0, 10.0)),
        (StableId::new("actor:beta").unwrap(), Vec2::new(10.0, 10.0)),
        (StableId::new("actor:far").unwrap(), Vec2::new(50.0, 50.0)),
    ];
    let first = crowd_separation_offsets(&agents, 5.0, 2.0);
    let second = crowd_separation_offsets(&agents, 5.0, 2.0);
    assert_eq!(first, second);
    assert!(first[0].length() > 0.0);
    assert!((first[0] + first[1]).length() < 0.001);
    assert_eq!(first[2], Vec2::ZERO);
    assert!(first.iter().all(|offset| offset.length() <= 2.0 + 0.001));
}

#[test]
fn predictive_crowd_yielding_is_deterministic_and_preserves_right_of_way() {
    let alpha = StableId::new("actor:alpha").unwrap();
    let beta = StableId::new("actor:beta").unwrap();
    let far = StableId::new("actor:far").unwrap();
    let agents = vec![
        (alpha.clone(), Vec2::new(-0.5, 0.0), Vec2::X),
        (beta.clone(), Vec2::new(0.5, 0.0), Vec2::NEG_X),
        (far.clone(), Vec2::new(0.0, 8.0), Vec2::X),
    ];
    let first = predictive_speed_factors(&agents, 0.5, 1.0, 0.18);
    let second = predictive_speed_factors(&agents, 0.5, 1.0, 0.18);
    assert_eq!(first, second);
    assert!((first[&alpha] - 1.0).abs() <= f32::EPSILON);
    assert!(first[&beta] < 1.0 && first[&beta] >= 0.18);
    assert!((first[&far] - 1.0).abs() <= f32::EPSILON);

    let follower = StableId::new("actor:follower").unwrap();
    let leader = StableId::new("actor:leader").unwrap();
    let following = predictive_speed_factors(
        &[
            (follower.clone(), Vec2::ZERO, Vec2::X),
            (leader.clone(), Vec2::new(0.3, 0.0), Vec2::X * 0.5),
        ],
        0.25,
        1.0,
        0.18,
    );
    assert!(following[&follower] < 1.0);
    assert!((following[&leader] - 1.0).abs() <= f32::EPSILON);
}

#[test]
fn retained_terrain_mesh_reconstructs_source_geometry() {
    let saved = SavedTerrainMesh {
        vertices: vec![[-4.0, 0.0, 3.0], [4.0, 0.0, 3.0], [0.0, 2.0, -5.0]],
        triangle_indices: vec![0, 1, 2],
        uvs: Vec::new(),
        uses_32_bit_indices: false,
    };
    let mesh = retained_terrain_mesh(&saved).unwrap();
    assert_eq!(mesh.count_vertices(), saved.vertices.len());
    assert_eq!(mesh.indices().unwrap().len(), saved.triangle_indices.len());
    assert_eq!(
        mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap().len(),
        saved.vertices.len()
    );
    assert_eq!(
        mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap().len(),
        saved.vertices.len()
    );
    assert!(Collider::trimesh_from_mesh(&mesh).is_some());
}

#[test]
fn embedded_config_matches_shipping_starting_roster() {
    let config = GameConfig::default();
    assert_eq!(config.gameplay.initial_agents, 5);
    let world = generate_world(&config.world);
    assert_eq!(world.navigation.width(), config.world.width);
}

#[test]
fn initial_actor_identities_match_unity_starting_npc_roles() {
    let expected = [
        ("npc:starting_defender", Some("role:defender")),
        ("npc:starting_logger", Some("role:logger")),
        ("npc:starting_miner", Some("role:miner")),
        ("npc:starting_gatherer", Some("role:gatherer")),
        ("npc:starting_builder", Some("role:builder")),
    ];
    for (index, (expected_id, expected_role)) in expected.into_iter().enumerate() {
        let (id, role) = initial_actor_identity(u16::try_from(index).unwrap());
        assert_eq!(id, expected_id);
        assert_eq!(role, expected_role);
    }
    assert_eq!(
        initial_actor_identity(5),
        ("actor:viewer_0005".to_owned(), None)
    );
}

#[test]
fn embedded_unity_content_catalog_is_valid() {
    let content = embedded_content();
    assert_eq!(content.archetypes.len(), 221);
    assert_eq!(content.foliage.len(), 4);
    assert_eq!(
        content
            .foliage
            .iter()
            .map(|layer| layer.variants.len())
            .sum::<usize>(),
        21
    );
    assert_eq!(content.buildings.len(), 32);
    assert_eq!(content.roles.len(), 18);
    assert_eq!(content.technology.nodes.len(), 598);
    assert_eq!(content.technology.groups.len(), 22);
    let guardhouse_id = StableId::new("building:guardhouse").unwrap();
    let guardhouse = &content.buildings[&guardhouse_id];
    let tower = &content.buildings[&StableId::new("building:tower").unwrap()];
    assert_eq!(guardhouse.footprint, tower.footprint);
    assert_eq!(guardhouse.placement_footprint_thirds, Some([6, 6]));
    assert_eq!(guardhouse.navigation_footprint_thirds, Some([4, 4]));
    assert!(guardhouse.projectile_shooter.is_none());
    for resource in ["food", "gold", "ore", "wood"] {
        let resource = StableId::new(format!("resource:{resource}")).unwrap();
        assert_eq!(guardhouse.cost[&resource], tower.cost[&resource] * 3);
    }
    let guardhouse_tech =
        &content.technology.nodes[&StableId::new("tech:native_guardhouse").unwrap()];
    assert_eq!(guardhouse_tech.age, "Age 1");
    assert!(!guardhouse_tech.initially_unlocked);
    assert!(guardhouse_tech.unlocked_buildings.contains(&guardhouse_id));
    assert_eq!(
        content.objectives[&StableId::new("objective:native_guardhouse:0").unwrap()]
            .required_amount,
        30_000
    );
    let path = &content.buildings[&StableId::new("building:path").unwrap()];
    assert_eq!(path.display_name, "Path");
    assert_eq!(path.footprint, [1, 1]);
    assert!(path.placeable && path.can_level && path.model_handlers.is_empty());
    assert_eq!(path.cost[&StableId::new("resource:wood").unwrap()], 150);
    assert_eq!(path.cost[&StableId::new("resource:ore").unwrap()], 130);
    assert_eq!(path.construction_cost_multiplier_per_thousand, 0);
    let streetlight_id = StableId::new("building:streetlight").unwrap();
    let streetlight = &content.buildings[&streetlight_id];
    assert_eq!(streetlight.placement_footprint_thirds, Some([1, 1]));
    assert_eq!(streetlight.navigation_footprint_thirds, Some([1, 1]));
    assert_eq!(streetlight.construction_cost_multiplier_per_thousand, 0);
    assert!(
        content.technology.nodes[&StableId::new("tech:native_path_unlock").unwrap()]
            .unlocked_buildings
            .contains(&streetlight_id)
    );
    assert_eq!(
        content.technology.nodes[&StableId::new("tech:native_path_level13").unwrap()]
            .building_level_caps[&StableId::new("building:path").unwrap()],
        13
    );
    let logger = &content.roles[&StableId::new("role:logger").unwrap()];
    assert_eq!(logger.base_action_amount, 1);
    assert_eq!(logger.experience_multiplier_per_thousand, 1_000);
    assert_eq!(logger.action_amount_per_level_milli, 500);
    assert_eq!(logger.base_action_milliseconds, 1_000);
    assert_eq!(logger.action_milliseconds_reduction_per_level, 5);
    assert_eq!(logger.base_carry_capacity, 10);
    assert_eq!(logger.carry_capacity_per_level_milli, 2_000);
    assert_eq!(
        logger.resource.as_ref().map(StableId::as_str),
        Some("resource:wood")
    );
    let ranger = &content.roles[&StableId::new("role:ranger").unwrap()];
    assert_eq!(ranger.base_action_range_milli_cells, 6_000);
    let monastery_id = StableId::new("building:monastery").unwrap();
    let monastery = &content.buildings[&monastery_id];
    let monastery_archetype = &content.archetypes[&monastery.archetype];
    assert!(
        default_archetype_scene(monastery_archetype)
            .unwrap()
            .asset_path
            .ends_with("Buildings/Archive/Monastery.glb")
    );
    assert_eq!(monastery.model_handlers.len(), 1);
    assert_eq!(monastery.model_handlers[0].age, 1);
    assert_eq!(monastery.model_handlers[0].full_model, "Monastery");
}

fn regeneration_role_fixture(
    role_name: &str,
    building_name: &str,
) -> (
    GameConfig,
    ContentCatalog,
    GeneratedWorld,
    WorldSimulation,
    RegenerationRoleRuntime,
    StableId,
    GridPos,
) {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world_with_content(&config.world, &content);
    let role = StableId::new(format!("role:{role_name}")).unwrap();
    let building_id = StableId::new(format!("building:{building_name}")).unwrap();
    let definition = &content.buildings[&building_id];
    let hut = find_building_site(
        &world,
        GridPos {
            x: config.world.width / 2,
            z: config.world.height / 2,
        },
        definition.footprint,
    )
    .expect("regeneration test hut has a valid site");
    let runtime_id = StableId::new(format!("building:test_{building_name}")).unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id.clone(),
            archetype: definition.archetype.clone(),
            position: hut,
            rotation_quarter_turns: 0,
            level: 1,
            health: 50,
            complete: true,
        },
    );
    let region = building_region(hut, definition.footprint, &world).unwrap();
    world.navigation.set_blocked(region, true).unwrap();
    let actor_position = nearest_walkable(&world, hut).unwrap();
    let actor = StableId::new(format!("twitch:test_{role_name}")).unwrap();
    assert!(simulation.join_player(actor.clone(), actor_position));
    simulation.assign_role(&actor, role.clone()).unwrap();
    simulation
        .actors
        .get_mut(&actor)
        .unwrap()
        .role_progression
        .insert(
            role,
            stream_town_domain::RoleProgress {
                level: 100,
                experience: 0,
            },
        );
    let mut runtime = RegenerationRoleRuntime {
        elapsed_seconds: 1_000.0,
        ..default()
    };
    runtime.workers.insert(
        actor.clone(),
        RegenerationWorkerState {
            initialized: true,
            next_ready_seconds: 0.0,
            station_visit_required: false,
            last_station: Some(runtime_id),
            prospector_step: 0,
            ..default()
        },
    );
    (config, content, world, simulation, runtime, actor, hut)
}

#[test]
fn planting_roles_visit_their_station_before_each_ready_cycle() {
    let (config, content, mut world, mut simulation, mut runtime, actor, _) =
        regeneration_role_fixture("forester", "nursery");
    let from = simulation.actors[&actor].position;
    let worker = runtime.workers.get_mut(&actor).unwrap();
    worker.station_visit_required = true;
    worker.last_station = None;

    let (visit_goal, approach) =
        regeneration_agent_goal(&content, &simulation, &world, &mut runtime, &actor, from)
            .expect("ready forester first visits the nursery");
    let AgentGoal::VisitRegenerationStation(station_id) = &visit_goal else {
        panic!("forester skipped the nursery before planting");
    };
    let _ = complete_agent_goal_with_regeneration(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &mut runtime,
        &actor,
        &visit_goal,
        approach,
    );
    assert_eq!(
        runtime.workers[&actor].last_station.as_ref(),
        Some(station_id)
    );
    assert!(!runtime.workers[&actor].station_visit_required);
    assert_eq!(
        actor_idle_anchor(&content, &simulation, &config, &runtime, &actor, approach),
        simulation.buildings[station_id].position,
        "idle wandering remains centered on the last visited nursery"
    );
    assert!(!runtime.workers[&actor].station_visit_required);
}

#[test]
fn losing_the_last_role_building_immediately_reassigns_citizens_to_defender() {
    let (_, content, _, mut simulation, _, actor, _) =
        regeneration_role_fixture("forester", "nursery");
    let first_id = simulation.buildings.keys().next().unwrap().clone();
    let mut second = simulation.buildings[&first_id].clone();
    second.id = StableId::new("building:test_nursery_second").unwrap();
    simulation
        .buildings
        .insert(second.id.clone(), second.clone());

    simulation.buildings.remove(&first_id);
    assert!(reassign_roles_without_prerequisite_buildings(&content, &mut simulation).is_empty());
    assert_eq!(simulation.actors[&actor].role.as_str(), "role:forester");

    simulation.buildings.remove(&second.id);
    assert_eq!(
        reassign_roles_without_prerequisite_buildings(&content, &mut simulation),
        vec![actor.clone()]
    );
    assert_eq!(simulation.actors[&actor].role.as_str(), "role:defender");
}

#[test]
fn regeneration_roles_and_buildings_execute_their_authored_tasks() {
    let (_, content, mut world, mut simulation, mut runtime, actor, hut) =
        regeneration_role_fixture("forester", "nursery");
    let from = simulation.actors[&actor].position;
    let fallen = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .find(|candidate| {
            grid_distance_squared(*candidate, hut) >= 100
                && valid_regeneration_cell(&content, &simulation, &world, *candidate)
                && planting_approach(&content, &simulation, &world, *candidate, from).is_some()
        })
        .expect("test world has a reachable former tree site");
    runtime.recently_fallen_trees.push_back(fallen);
    let trees_before = world
        .resources
        .iter()
        .filter(|resource| resource.amount > 0 && resource.target_kind.as_str() == "target:tree")
        .count();
    let (goal, _) =
        regeneration_agent_goal(&content, &simulation, &world, &mut runtime, &actor, from)
            .expect("ready forester chooses a planting task");
    assert!(matches!(goal, AgentGoal::PlantTree(_)));
    assert!(complete_regeneration_goal(
        &mut simulation,
        &mut world,
        &content,
        &mut runtime,
        &actor,
        &goal,
    ));
    assert_eq!(
            world
                .resources
                .iter()
                .filter(|resource| resource.amount > 0
                    && resource.target_kind.as_str() == "target:tree")
                .count(),
            trees_before + 1
        );
    let planted_tree = world
        .resources
        .iter()
        .find(|resource| resource.id.as_str().starts_with("resource:regrown_tree_"))
        .expect("forester creates a persistable regenerated tree");
    let restored_tree = restored_regenerated_resource(&planted_tree.id, planted_tree.amount)
        .expect("regenerated resource ID restores from a native save");
    assert_eq!(restored_tree, *planted_tree);
    assert!(
        planted_tree
            .offset_milli_cells
            .iter()
            .all(|offset| (-250..=250).contains(offset)),
        "regrown resources retain the central-half-cell visual offset contract"
    );
    assert!(
        (regeneration_role_interval_seconds(&StableId::new("role:forester").unwrap(), 100,)
            .unwrap()
            - 13.4)
            .abs()
            < f64::EPSILON
    );
    assert_eq!(
        regeneration_role_interval_seconds(&StableId::new("role:forester").unwrap(), 1),
        Some(201.0)
    );
    let tender = StableId::new("role:tender").unwrap();
    assert_eq!(
        regeneration_role_interval_seconds(&tender, 1),
        Some(3_600.0)
    );
    assert_eq!(
        regeneration_role_interval_seconds(&tender, 100),
        Some(720.0)
    );
    assert_eq!(prospector_discovery_denominator(1), 4_000);
    assert_eq!(prospector_discovery_denominator(100), 400);

    let (_, content, mut world, mut simulation, mut runtime, actor, hut) =
        regeneration_role_fixture("prospector", "prospector_hut");
    let from = simulation.actors[&actor].position;
    let (spiral_goal, _) =
        regeneration_agent_goal(&content, &simulation, &world, &mut runtime, &actor, from)
            .expect("prospector starts the authored spiral");
    let AgentGoal::Prospect { cell, .. } = spiral_goal else {
        panic!("prospector did not receive a prospect goal");
    };
    let radius = cell.x.abs_diff(hut.x).max(cell.z.abs_diff(hut.z));
    assert!((5..=20).contains(&radius));
    let nearby_ores = nearby_active_resource_count(
        &world,
        simulation.actors[&actor].position,
        "target:ore",
        LOCAL_RESOURCE_SATURATION_RADIUS_CELLS,
    );
    let saturated_denominator = prospector_discovery_denominator(100)
        .saturating_mul(u64::from(nearby_ores).saturating_add(1));
    let successful_sequence = (0..1_000_000_u32)
        .find(|sequence| {
            seagull_hash(
                world.seed ^ stable_id_hash(&actor),
                u64::from(*sequence),
                0x5052_4F53_5045_4354,
            )
            .is_multiple_of(saturated_denominator)
        })
        .expect("deterministic prospect sequence contains a discovery");
    let ore_before = world
        .resources
        .iter()
        .filter(|resource| resource.amount > 0 && resource.target_kind.as_str() == "target:ore")
        .count();
    assert!(complete_regeneration_goal(
        &mut simulation,
        &mut world,
        &content,
        &mut runtime,
        &actor,
        &AgentGoal::Prospect {
            cell,
            sequence: successful_sequence,
        },
    ));
    let ore_added = world
        .resources
        .iter()
        .filter(|resource| resource.amount > 0 && resource.target_kind.as_str() == "target:ore")
        .count()
        - ore_before;
    assert!((3..=5).contains(&ore_added));

    let (_, content, mut world, mut simulation, mut runtime, actor, hut) =
        regeneration_role_fixture("tender", "greenhouse");
    let mut tree = world
        .resources
        .iter()
        .find(|resource| resource.target_kind.as_str() == "target:tree")
        .cloned()
        .expect("generated fixture contains a tree template");
    let cleared = world
        .resources
        .iter_mut()
        .filter(|resource| resource.target_kind.as_str() != "target:fish")
        .map(|resource| {
            resource.amount = 0;
            resource.position
        })
        .collect::<Vec<_>>();
    for position in cleared {
        world
            .navigation
            .set_blocked(
                stream_town_domain::DirtyRegion {
                    min: position,
                    max: position,
                },
                false,
            )
            .unwrap();
    }
    let tree_position = (-18_i32..=18)
        .flat_map(|z| (-18_i32..=18).map(move |x| (x, z)))
        .filter(|(x, z)| x.abs().max(z.abs()) >= 12)
        .filter_map(|(x, z)| offset_grid(hut, x, z, &world))
        .find(|position| {
            world.navigation.is_walkable(*position)
                && cell_is_clear_of_buildings(&content, &simulation, &world, *position, 10)
        })
        .expect("fixture has a clear tree position near the greenhouse");
    tree.position = tree_position;
    tree.amount = 100;
    world.resources.push(tree);
    world
        .navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: tree_position,
                max: tree_position,
            },
            true,
        )
        .unwrap();
    let from = simulation.actors[&actor].position;
    let (goal, _) =
        regeneration_agent_goal(&content, &simulation, &world, &mut runtime, &actor, from)
            .expect("ready tender chooses a field planting task");
    let AgentGoal::PlantBush(field) = goal else {
        panic!("tender did not receive a bush planting goal");
    };
    let second_actor = StableId::new("twitch:test_tender_second").unwrap();
    let tender_role = StableId::new("role:tender").unwrap();
    assert!(simulation.join_player(second_actor.clone(), from));
    simulation
        .assign_role(&second_actor, tender_role.clone())
        .unwrap();
    simulation
        .actors
        .get_mut(&second_actor)
        .unwrap()
        .role_progression
        .insert(
            tender_role,
            stream_town_domain::RoleProgress {
                level: 100,
                experience: 0,
            },
        );
    runtime.workers.insert(
        second_actor.clone(),
        RegenerationWorkerState {
            initialized: true,
            next_ready_seconds: 0.0,
            station_visit_required: false,
            last_station: regeneration_hut(
                &content,
                &simulation,
                &StableId::new("role:tender").unwrap(),
                from,
            )
            .map(|building| building.id.clone()),
            ..default()
        },
    );
    let (second_goal, _) = regeneration_agent_goal(
        &content,
        &simulation,
        &world,
        &mut runtime,
        &second_actor,
        from,
    )
    .expect("a second ready tender chooses a field planting task");
    let AgentGoal::PlantBush(second_field) = second_goal else {
        panic!("second tender did not receive a bush planting goal");
    };
    assert_ne!(
        second_field, field,
        "simultaneous tenders reserve distinct planting destinations"
    );
    assert!(cell_is_clear_of_buildings(
        &content,
        &simulation,
        &world,
        field,
        10,
    ));
    let tree_distance = grid_distance_squared(field, tree_position);
    assert!(
        (9..=36).contains(&tree_distance),
        "tender berries must be between three and six cells from a tree"
    );

    // A destination may have been empty when selected and become occupied
    // while the tender walks there. Completion must revalidate the live
    // actor occupancy instead of growing a blocking bush under a citizen.
    simulation.actors.get_mut(&second_actor).unwrap().position = field;
    assert!(!complete_regeneration_goal(
        &mut simulation,
        &mut world,
        &content,
        &mut runtime,
        &actor,
        &AgentGoal::PlantBush(field),
    ));
    simulation.actors.get_mut(&second_actor).unwrap().position = from;
    assert!(complete_regeneration_goal(
        &mut simulation,
        &mut world,
        &content,
        &mut runtime,
        &actor,
        &AgentGoal::PlantBush(field),
    ));
    assert_eq!(
            world
                .resources
                .iter()
                .filter(|resource| resource.amount > 0
                    && resource.target_kind.as_str() == "target:bush")
                .count(),
            1
        );

    for (technology, building, forbidden_resource) in [
        ("tech:native_nursery", "building:nursery", "resource:wood"),
        (
            "tech:native_prospector_hut",
            "building:prospector_hut",
            "resource:ore",
        ),
        (
            "tech:native_greenhouse",
            "building:greenhouse",
            "resource:food",
        ),
    ] {
        let node = &content.technology.nodes[&StableId::new(technology).unwrap()];
        assert!(node.prerequisites.is_empty());
        assert!(!node.initially_unlocked);
        assert!(
            node.unlocked_buildings
                .contains(&StableId::new(building).unwrap())
        );
        assert!(node.objectives.iter().all(|objective| {
            content.objectives[objective]
                .resource
                .as_ref()
                .map(StableId::as_str)
                != Some(forbidden_resource)
        }));
    }
}

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

#[test]
fn root_exit_into_the_active_fallback_does_not_restart_its_loop() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let player =
        archetype_by_source(&content, ArchetypeKind::Player, "Player_Character.prefab").unwrap();
    let spec = converted_animation_spec(player, &presentation).unwrap();
    let controller = presentation.controllers.get(&spec.controller).unwrap();
    let mut runtime = AnimationControllerRuntime::in_state(controller, spec.state.clone()).unwrap();
    let playback = AnimationTransitionPlayback {
        duration: 0.1,
        fixed_duration: true,
        destination_offset: 0.0,
    };

    let resolved = resolve_animation_exit_fallback(
        &mut runtime,
        controller,
        &spec.state,
        Some(&stream_town_domain::AnimationTransitionOutcome::Exited),
        Some(playback),
    );

    assert_eq!(resolved, None);
    assert_eq!(runtime.current_state(), &spec.state);
}

#[test]
fn player_controller_prefers_skin_compatible_native_clips() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let player =
        archetype_by_source(&content, ArchetypeKind::Player, "Player_Character.prefab").unwrap();
    let spec = converted_animation_spec(player, &presentation).unwrap();
    let controller = presentation.controllers.get(&spec.controller).unwrap();
    let requests: BTreeMap<_, _> = controller
        .states
        .values()
        .flat_map(|state| &state.motions)
        .map(|motion| {
            let source = presentation.clips.get(&motion.clip).unwrap();
            let request = converted_clip_request(source, PLAYER_ANIMATED_MODEL_PATH, &presentation)
                .unwrap_or_else(|| {
                    panic!(
                        "{} has no native clip compatible with the visible player skin",
                        source.display_name
                    )
                });
            let ConvertedClipRequest::Asset { path, index } = request else {
                panic!(
                    "{} selected standalone retargeting instead of the visible rig",
                    source.display_name
                );
            };
            assert_eq!(path, PLAYER_ANIMATED_MODEL_PATH);
            (source.display_name.clone(), index)
        })
        .collect();
    let expected: BTreeMap<_, _> = [
        ("PlayerChar_Idle_01", 13),
        ("PlayerChar_Walk_01", 23),
        ("PlayerChar_Run_01", 18),
        ("PlayerChar_Logging_05", 24),
        ("PlayerChar_SpearAttack_01", 20),
        ("PlayerChar_Building_01", 4),
        ("PlayerChar_Farming_01", 7),
        ("PlayerChar_DEATH_01", 6),
        ("PlayerChar_Healing", 11),
        ("PlayerChar_HammerAttack_01", 10),
        ("PlayerChar_BowShoot_01", 3),
        ("PlayerChar_LoggerCarryWood_01", 1),
        ("PlayerChar_Gathering_01", 9),
        ("PlayerChar_Summon", 5),
        ("PlayerChar_Fishing_01", 8),
        ("PlayerChar_Mining_02", 17),
        ("PlayerChar_SwordAttack_01", 16),
        ("PlayerChar_CarryItems_01", 2),
        ("PlayerChar_Magic", 5),
        ("CharacterStaffMagicAttack", 22),
    ]
    .into_iter()
    .map(|(name, index)| (name.to_owned(), index))
    .collect();
    assert_eq!(requests, expected);

    let run = presentation
        .clips
        .values()
        .find(|clip| clip.display_name == "PlayerChar_Run_01")
        .unwrap();
    assert!(
        !run.transform_tracks.is_empty(),
        "the source-selection regression requires competing standalone curves"
    );
    assert_eq!(
        converted_clip_request(run, PLAYER_ANIMATED_MODEL_PATH, &presentation),
        Some(ConvertedClipRequest::Asset {
            path: PLAYER_ANIMATED_MODEL_PATH.to_owned(),
            index: 18,
        })
    );
}

#[test]
fn authored_player_run_clip_loop_pose_correction_closes_the_seam() {
    let presentation = embedded_presentation();
    let run = presentation
        .clips
        .values()
        .find(|clip| clip.display_name == "PlayerChar_Run_01")
        .unwrap();
    assert!(run.looping);
    let mut raw_max_rotation = 0.0_f32;
    let mut max_translation = 0.0_f32;
    let mut max_rotation = 0.0_f32;
    let mut max_scale = 0.0_f32;
    for track in &run.transform_tracks {
        let mut translation: Vec<_> = track
            .translation
            .iter()
            .map(|key| (key.time, Vec3::from_array(key.value)))
            .collect();
        close_translation_loop(&mut translation, run.duration_seconds);
        if let (Some((_, first)), Some((_, last))) = (translation.first(), translation.last()) {
            max_translation = max_translation.max(first.distance(*last));
            assert!(
                translation.last().unwrap().0 + 1.0e-6 >= run.duration_seconds,
                "{} translation ended before the clip boundary",
                track.target_path
            );
        }
        if let (Some(first), Some(last)) = (track.rotation.first(), track.rotation.last()) {
            raw_max_rotation = raw_max_rotation.max(
                Quat::from_array(first.value)
                    .normalize()
                    .angle_between(Quat::from_array(last.value).normalize()),
            );
        }
        let mut rotation: Vec<_> = track
            .rotation
            .iter()
            .map(|key| (key.time, normalized_quat(key.value)))
            .collect();
        close_rotation_loop(&mut rotation, run.duration_seconds);
        if let (Some((_, first)), Some((_, last))) = (rotation.first(), rotation.last()) {
            max_rotation = max_rotation.max(first.angle_between(*last));
            assert!(
                rotation.last().unwrap().0 + 1.0e-6 >= run.duration_seconds,
                "{} rotation ended before the clip boundary",
                track.target_path
            );
        }
        let mut scale: Vec<_> = track
            .scale
            .iter()
            .map(|key| (key.time, Vec3::from_array(key.value)))
            .collect();
        close_scale_loop(&mut scale, run.duration_seconds);
        if let (Some((_, first)), Some((_, last))) = (scale.first(), scale.last()) {
            max_scale = max_scale.max(first.distance(*last));
        }
    }
    assert!(
        raw_max_rotation > 0.5,
        "fixture no longer exercises its known run-pose seam: {raw_max_rotation}"
    );
    assert!(
        max_translation <= 1.0e-5,
        "corrected translation seam {max_translation}"
    );
    assert!(
        max_rotation <= 1.0e-3,
        "corrected rotation seam {max_rotation}"
    );
    assert!(max_scale <= 1.0e-5, "corrected scale seam {max_scale}");
}

#[test]
fn loop_pose_correction_preserves_the_first_pose_and_closes_the_last() {
    let first_rotation = Quat::from_rotation_x(-0.25) * Quat::from_rotation_z(0.1);
    let middle_rotation = Quat::from_rotation_x(0.4);
    let last_rotation = Quat::from_rotation_x(0.7) * Quat::from_rotation_z(-0.2);
    let mut rotations = vec![
        (0.0, first_rotation),
        (0.5, middle_rotation),
        (1.0, last_rotation),
    ];
    let mut translations = vec![
        (0.0, Vec3::new(1.0, 2.0, 3.0)),
        (0.5, Vec3::new(2.0, 3.0, 4.0)),
        (1.0, Vec3::new(3.0, 4.0, 5.0)),
    ];
    close_rotation_loop(&mut rotations, 1.0);
    close_translation_loop(&mut translations, 1.0);

    assert!(rotations[0].1.angle_between(first_rotation) <= 1.0e-6);
    assert!(rotations[2].1.angle_between(first_rotation) <= 1.0e-6);
    assert_eq!(translations[0].1, Vec3::new(1.0, 2.0, 3.0));
    assert!(translations[2].1.distance(translations[0].1) <= 1.0e-6);
    assert!(rotations[1].1.is_finite());
    assert!(translations[1].1.is_finite());
}

#[test]
fn authored_layer_weight_and_mask_configure_bevy_graph_branch() {
    let mut graph = AnimationGraph::new();
    let composition = add_animation_composition(&mut graph);
    assert!(matches!(
        graph.graph[composition].node_type,
        AnimationNodeType::Blend
    ));
    let path = "CharacterArmature/Body/UpperArm_R";
    let target = path.split('/').collect::<AnimationTargetId>();
    let targets = BTreeMap::from([(path.to_owned(), (Entity::PLACEHOLDER, Transform::IDENTITY))]);
    let authored = AvatarMaskDef {
        display_name: "Left Arm".into(),
        source_guid: "a".repeat(32),
        source_path: "Assets/LeftArm.mask".into(),
        humanoid_body_mask_hex: "01000000".into(),
        transform_weights: BTreeMap::from([(path.to_owned(), 0.0)]),
    };
    let mask = register_avatar_mask(&mut graph, 3, Some(&authored), &targets);
    let branch = add_animation_layer_branch(
        &mut graph,
        AnimationLayerBlendMode::Additive,
        0.25,
        mask,
        composition,
    );
    assert_eq!(mask, 1 << 3);
    assert_eq!(graph.mask_groups[&target], 1 << 3);
    assert!(matches!(
        graph.graph[branch].node_type,
        AnimationNodeType::Add
    ));
    assert!((graph.graph[branch].weight - 0.25).abs() < f32::EPSILON);
    assert_eq!(graph.graph[branch].mask, 1 << 3);
}

#[test]
fn connected_bot_dispatches_twitch_commands_without_a_chat_gate() {
    let viewer = |message: &str, is_broadcaster| {
        TwitchEvent::Chat(twitch::TwitchChatEnvelope {
            message_id: None,
            actor_id: StableId::new("twitch:42").unwrap(),
            user_id: "42".to_owned(),
            login: "viewer".to_owned(),
            display_name: "Viewer".to_owned(),
            message: message.to_owned(),
            is_broadcaster,
            is_moderator: false,
            is_subscriber: false,
            custom_reward_id: None,
        })
    };
    let mut connection = TwitchConnection::default();
    let mut commands = InjectedCommands::default();

    handle_twitch_event(viewer("!join", false), &mut connection, &mut commands);
    let dispatched = commands.0.pop_front().unwrap();
    assert_eq!(dispatched.actor_id, StableId::new("twitch:42").unwrap());
    assert_eq!(dispatched.command, ChatCommand::Join);
}

#[test]
fn broadcaster_moderation_state_changes_do_not_overwrite_bot_connection_state() {
    let mut connection = TwitchConnection {
        status: TwitchStatus::Connected,
        moderation_status: TwitchModerationStatus::Error(
            "old broadcaster authorization failed".to_owned(),
        ),
        ..default()
    };
    let mut commands = InjectedCommands::default();

    handle_twitch_event(
        TwitchEvent::ModerationStatus(TwitchModerationStatus::Authorizing),
        &mut connection,
        &mut commands,
    );
    assert_eq!(connection.status, TwitchStatus::Connected);
    assert_eq!(
        connection.moderation_status,
        TwitchModerationStatus::Authorizing
    );

    handle_twitch_event(
        TwitchEvent::ModerationStatus(TwitchModerationStatus::Ready),
        &mut connection,
        &mut commands,
    );
    assert_eq!(connection.status, TwitchStatus::Connected);
    assert_eq!(connection.moderation_status, TwitchModerationStatus::Ready);
}

#[test]
fn operator_chat_history_is_bounded_and_keeps_the_newest_messages() {
    let mut chat = OperatorChatRuntime::default();
    for index in 0..(OPERATOR_CHAT_HISTORY_CAPACITY + 5) {
        chat.push(OperatorChatLine {
            line_id: 0,
            user_id: index.to_string(),
            login: format!("viewer{index}"),
            display_name: format!("Viewer {index}"),
            message: format!("message {index}"),
            badges: OperatorChatBadges {
                broadcaster: false,
                moderator: false,
                subscriber: false,
            },
            is_system: false,
        });
    }
    assert_eq!(chat.lines.len(), OPERATOR_CHAT_HISTORY_CAPACITY);
    assert_eq!(chat.lines.front().unwrap().message, "message 5");
    assert_eq!(
        chat.lines.back().unwrap().message,
        format!("message {}", OPERATOR_CHAT_HISTORY_CAPACITY + 4)
    );
}

#[test]
fn operator_chat_deduplicates_the_broadcaster_echo_by_twitch_message_id() {
    let message = twitch::TwitchChatEnvelope {
        message_id: Some("outbound-message-id".to_owned()),
        actor_id: StableId::new("twitch:7").unwrap(),
        user_id: "7".to_owned(),
        login: "streamer".to_owned(),
        display_name: "Streamer".to_owned(),
        message: "Hello chat".to_owned(),
        is_broadcaster: true,
        is_moderator: true,
        is_subscriber: true,
        custom_reward_id: None,
    };
    let mut chat = OperatorChatRuntime::default();

    assert!(chat.push_chat(&message));
    assert!(!chat.push_chat(&message));
    assert_eq!(chat.lines.len(), 1);
    let line = chat.lines.back().unwrap();
    assert!(line.badges.broadcaster);
    assert!(line.badges.moderator);
    assert!(line.badges.subscriber);
}

#[test]
fn operator_chat_scroll_stays_anchored_when_new_messages_arrive() {
    let mut chat = OperatorChatRuntime::default();
    for index in 0..12 {
        chat.push_system(format!("message {index}"));
    }
    chat.scroll_older(2, 8);
    let before = chat
        .visible_lines(8)
        .into_iter()
        .map(|line| line.message.clone())
        .collect::<Vec<_>>();

    chat.push_system("new message");
    let after = chat
        .visible_lines(8)
        .into_iter()
        .map(|line| line.message.clone())
        .collect::<Vec<_>>();

    assert_eq!(before, after);
    chat.scroll_newer(usize::MAX);
    assert_eq!(chat.visible_lines(8).last().unwrap().message, "new message");
}

#[test]
fn invalid_command_feedback_only_targets_command_attempts() {
    assert_eq!(invalid_twitch_command_reply("hello town"), None);
    assert_eq!(invalid_twitch_command_reply("   hello town"), None);
    assert_eq!(
        invalid_twitch_command_reply("!not-a-command"),
        Some("Invalid Command! Type !help for the list of commands!")
    );
    assert_eq!(
        invalid_twitch_command_reply("  !cam sideways"),
        Some("Invalid Command! Type !help for the list of commands!")
    );
}

#[test]
fn help_points_to_the_versioned_native_command_reference() {
    assert!(TWITCH_COMMAND_HELP_URL.starts_with("https://github.com/HumanBeanGames/"));
    assert!(TWITCH_COMMAND_HELP_URL.ends_with("/TWITCH_COMMANDS.md"));
    let reference = include_str!("../../../../TWITCH_COMMANDS.md");
    for command in [
        "join", "build", "confirm", "cancel", "role", "event", "yes", "no", "cam", "health", "help",
    ] {
        assert!(
            reference.contains(&format!("`!{command}")),
            "public !help reference omits native command !{command}"
        );
    }
}

#[test]
fn fish_god_channel_reward_dispatches_praise_without_command_text() {
    let mut connection = TwitchConnection {
        fish_god_reward_id: Some(stream_town_domain::SHIPPING_FISH_GOD_REWARD_ID.to_owned()),
        ..default()
    };
    let mut commands = InjectedCommands::default();
    handle_twitch_event(
        TwitchEvent::Chat(twitch::TwitchChatEnvelope {
            message_id: None,
            actor_id: StableId::new("twitch:fish").unwrap(),
            user_id: "fish".to_owned(),
            login: "fishfriend".to_owned(),
            display_name: "FishFriend".to_owned(),
            message: "Praise!".to_owned(),
            is_broadcaster: false,
            is_moderator: false,
            is_subscriber: false,
            custom_reward_id: Some(stream_town_domain::SHIPPING_FISH_GOD_REWARD_ID.to_owned()),
        }),
        &mut connection,
        &mut commands,
    );
    assert_eq!(commands.0.pop_front().unwrap().command, ChatCommand::Praise);
}

#[test]
fn unconfigured_or_different_channel_reward_does_not_dispatch_praise() {
    let mut connection = TwitchConnection {
        fish_god_reward_id: None,
        ..default()
    };
    let mut commands = InjectedCommands::default();
    handle_twitch_event(
        TwitchEvent::Chat(twitch::TwitchChatEnvelope {
            message_id: None,
            actor_id: StableId::new("twitch:fish").unwrap(),
            user_id: "fish".to_owned(),
            login: "fishfriend".to_owned(),
            display_name: "FishFriend".to_owned(),
            message: "Praise!".to_owned(),
            is_broadcaster: false,
            is_moderator: false,
            is_subscriber: false,
            custom_reward_id: Some("00000000-0000-0000-0000-000000000000".to_owned()),
        }),
        &mut connection,
        &mut commands,
    );
    assert!(commands.0.is_empty());
}

#[test]
fn ruler_permissions_and_login_candidate_resolution_are_stable() {
    let ruler = StableId::new("twitch:100").unwrap();
    let viewer = StableId::new("twitch:200").unwrap();
    let mut simulation = WorldSimulation::new(7);
    assert!(simulation.join_player(ruler.clone(), GridPos { x: 1, z: 1 }));
    assert!(simulation.join_player(viewer.clone(), GridPos { x: 2, z: 1 }));
    simulation.actors.get_mut(&ruler).unwrap().login_name = Some("the_ruler".to_owned());
    simulation.actors.get_mut(&ruler).unwrap().display_name = Some("The Ruler".to_owned());
    simulation.set_ruler(ruler.clone()).unwrap();
    simulation
        .start_ruler_vote(RulerVoteKind::KeepRuler)
        .unwrap();

    let ordinary = PendingChatCommand {
        actor_id: viewer,
        login_name: "viewer".to_owned(),
        display_name: "Viewer".to_owned(),
        command: ChatCommand::RecruitCount,
        is_broadcaster: false,
        is_moderator: false,
        is_subscriber: false,
        origin: CommandOrigin::Twitch,
    };
    assert!(require_ruler_or_staff(&simulation, &ordinary).is_err());
    let ruler_command = PendingChatCommand {
        actor_id: ruler,
        login_name: "the_ruler".to_owned(),
        display_name: "The Ruler".to_owned(),
        command: ChatCommand::RecruitCount,
        is_broadcaster: false,
        is_moderator: false,
        is_subscriber: false,
        origin: CommandOrigin::Twitch,
    };
    assert!(require_ruler_or_staff(&simulation, &ruler_command).is_ok());
    assert_eq!(
        resolve_ruler_vote_option(&simulation, &StableId::new("yes").unwrap()),
        Some(StableId::new("yes").unwrap())
    );
    simulation.ruler_vote = None;
    simulation
        .start_ruler_vote(RulerVoteKind::NewRuler)
        .unwrap();
    assert_eq!(
        resolve_ruler_vote_option(&simulation, &StableId::new("the_ruler").unwrap()),
        Some(StableId::new("twitch:100").unwrap())
    );
}

#[test]
fn game_master_permission_is_explicit_and_local_debug_bypasses_it() {
    let mut config = GameConfig::default();
    let mut command = PendingChatCommand {
        actor_id: StableId::new("twitch:12345").unwrap(),
        login_name: "configured_gm".to_owned(),
        display_name: "Configured GM".to_owned(),
        command: ChatCommand::ToggleBuildCosts,
        is_broadcaster: true,
        is_moderator: true,
        is_subscriber: false,
        origin: CommandOrigin::Twitch,
    };
    assert!(require_staff(&command).is_ok());
    assert!(require_game_master(&config, &command).is_err());
    config.twitch.game_master_ids.insert("12345".to_owned());
    assert!(require_game_master(&config, &command).is_ok());
    let user_type = pending_stream_user_type(&config, &command);
    assert_eq!(user_type, StreamUserType::GameMaster);
    let simulation = WorldSimulation::new(1);
    assert!(require_world_view_operator(&simulation, &command, user_type).is_ok());
    assert!(is_world_diagnostic_timer_operator(
        &simulation,
        &command.actor_id,
        user_type,
    ));
    config.twitch.game_master_ids.clear();
    command.is_broadcaster = false;
    command.is_moderator = false;
    command.origin = CommandOrigin::LocalDebug;
    assert!(require_game_master(&config, &command).is_ok());
}

#[test]
fn unity_name_and_health_overlay_visibility_rules_are_preserved() {
    assert!(!should_show_actor_name(
        NameDisplayMode::None,
        StreamUserType::GameMaster
    ));
    assert!(should_show_actor_name(
        NameDisplayMode::StaffAndSubscribers,
        StreamUserType::Subscriber
    ));
    assert!(should_show_actor_name(
        NameDisplayMode::StaffAndSubscribers,
        StreamUserType::Moderator
    ));
    assert!(!should_show_actor_name(
        NameDisplayMode::StaffAndSubscribers,
        StreamUserType::Normal
    ));
    assert!(should_show_actor_name(
        NameDisplayMode::AllPlayers,
        StreamUserType::Normal
    ));

    assert!(!should_show_building_health(
        BuildingHealthDisplayMode::None,
        25,
        100
    ));
    assert!(!should_show_building_health(
        BuildingHealthDisplayMode::DamagedOnly,
        100,
        100
    ));
    assert!(should_show_building_health(
        BuildingHealthDisplayMode::DamagedOnly,
        99,
        100
    ));
    assert!(should_show_building_health(
        BuildingHealthDisplayMode::Always,
        100,
        100
    ));
    assert!((building_health_fraction(25, 100) - 0.25).abs() < f32::EPSILON);
}

#[test]
fn path_health_bars_use_fine_positions_and_one_fifth_width() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let mut simulation = WorldSimulation::new(world.seed);
    let fallback = grid_to_world_on_surface(GridPos { x: 1, z: 1 }, &config, &world);
    let mut resolved = Vec::new();
    for (index, position) in [
        GridPos { x: 3, z: 4 },
        GridPos { x: 4, z: 4 },
        GridPos { x: 5, z: 4 },
    ]
    .into_iter()
    .enumerate()
    {
        let id = StableId::new(format!("building:path_health_{index}")).unwrap();
        simulation
            .path_navigation_positions
            .insert(id.clone(), position);
        resolved.push(building_health_overlay_world_position(
            &RuntimeBuilding { id },
            fallback,
            &simulation,
            &config,
            &world,
        ));
    }
    assert_ne!(resolved[0], resolved[1]);
    assert_ne!(resolved[1], resolved[2]);
    assert_ne!(resolved[0], resolved[2]);

    let path_definition = &content.buildings[&StableId::new("building:path").unwrap()];
    let path = BuildingState {
        id: StableId::new("building:path_health_width").unwrap(),
        archetype: path_definition.archetype.clone(),
        position: GridPos { x: 1, z: 1 },
        rotation_quarter_turns: 0,
        level: 1,
        health: 100,
        complete: true,
    };
    let house_definition = &content.buildings[&StableId::new("building:house").unwrap()];
    let house = BuildingState {
        id: StableId::new("building:house_health_width").unwrap(),
        archetype: house_definition.archetype.clone(),
        position: GridPos { x: 1, z: 1 },
        rotation_quarter_turns: 0,
        level: 1,
        health: 100,
        complete: true,
    };
    assert!(
        (building_health_overlay_width_px(&content, &house) - BUILDING_HEALTH_OVERLAY_WIDTH_PX)
            .abs()
            < f32::EPSILON
    );
    assert!(
        (building_health_overlay_width_px(&content, &path) - PATH_HEALTH_OVERLAY_WIDTH_PX).abs()
            < f32::EPSILON
    );
    assert!(
        (building_health_overlay_width_px(&content, &path) * 5.0
            - building_health_overlay_width_px(&content, &house))
        .abs()
            < f32::EPSILON
    );
}

#[test]
fn paths_use_their_authored_health_and_catalog_level_bonus() {
    let content = embedded_content();
    let definition = &content.buildings[&StableId::new("building:path").unwrap()];
    let authored = content.archetypes[&definition.archetype]
        .health
        .as_ref()
        .expect("paths have authored health");
    let mut path = BuildingState {
        id: StableId::new("building:path_health_test").unwrap(),
        archetype: definition.archetype.clone(),
        position: GridPos { x: 1, z: 1 },
        rotation_quarter_turns: 0,
        level: 1,
        health: 1,
        complete: false,
    };

    assert_eq!(
        building_base_max_health(&content, definition),
        authored.max_health
    );
    assert_eq!(
        u32::try_from(building_max_health(&content, &path)).unwrap(),
        authored.max_health
    );
    path.level = 2;
    assert_eq!(
        u32::try_from(building_max_health(&content, &path)).unwrap(),
        leveled_percentage_stat(
            authored.max_health,
            definition.health_bonus_per_level_per_thousand,
            2,
        )
    );
}

#[test]
fn player_health_bar_uses_authored_damage_and_hide_contract() {
    let content = embedded_content();
    let player = content
        .archetypes
        .values()
        .find(|archetype| archetype.source_path.ends_with("Player_Character.prefab"))
        .unwrap();
    assert_eq!(player.health_bar_hide_milliseconds, Some(3_000));
    assert!(
        player
            .component_types
            .iter()
            .any(|component| component == "Units.UnitHealthBar")
    );

    let mut overlay = ActorHealthOverlay {
        actor: StableId::new("twitch:health_bar_test").unwrap(),
        last_health: 100,
        was_damaged: false,
        hide_remaining_seconds: 0.0,
    };
    assert!(update_actor_health_overlay_timer(
        &mut overlay,
        75,
        100,
        3.0,
        0.1
    ));
    assert!(update_actor_health_overlay_timer(
        &mut overlay,
        100,
        100,
        3.0,
        1.0
    ));
    assert!(update_actor_health_overlay_timer(
        &mut overlay,
        100,
        100,
        3.0,
        1.9
    ));
    assert!(!update_actor_health_overlay_timer(
        &mut overlay,
        100,
        100,
        3.0,
        0.2
    ));
    assert!(!overlay.was_damaged);
}

#[test]
fn numbered_world_labels_preserve_unitys_fifteen_second_lifetime() {
    let mut label = TemporaryWorldLabel {
        target: StableId::new("npc:starting_builder").unwrap(),
        remaining_seconds: UNITY_NUMBERED_LABEL_SECONDS,
    };
    assert!(temporary_world_label_is_live(&mut label, 14.9));
    assert!((label.remaining_seconds - 0.1).abs() < 1e-5);
    assert!(!temporary_world_label_is_live(&mut label, 0.2));
    assert!(label.remaining_seconds.abs() < f32::EPSILON);
}

#[test]
fn twitch_user_type_uses_unity_privilege_priority() {
    let mut config = GameConfig::default();
    let mut command = PendingChatCommand {
        actor_id: StableId::new("twitch:12345").unwrap(),
        login_name: "staff".to_owned(),
        display_name: "Staff".to_owned(),
        command: ChatCommand::Join,
        is_broadcaster: true,
        is_moderator: true,
        is_subscriber: true,
        origin: CommandOrigin::Twitch,
    };
    assert_eq!(
        pending_stream_user_type(&config, &command),
        StreamUserType::Broadcaster
    );
    command.is_broadcaster = false;
    assert_eq!(
        pending_stream_user_type(&config, &command),
        StreamUserType::Moderator
    );
    config.twitch.game_master_ids.insert("12345".to_owned());
    assert_eq!(
        pending_stream_user_type(&config, &command),
        StreamUserType::GameMaster
    );
}

fn enter_headless_world(app: &mut App) {
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::WorldLoading);
    app.update();
    app.update();
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        Duration::from_millis(250),
    ));
    for _ in 0..1_024 {
        app.update();
        if *app.world().resource::<State<GameState>>().get() == GameState::InGame
            && app.world().contains_resource::<GameplayReady>()
        {
            break;
        }
        std::thread::yield_now();
    }
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        Duration::ZERO,
    ));
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::InGame
    );
    assert!(
        app.world().contains_resource::<GameplayReady>(),
        "headless world entry must wait for the same overlay-retirement gate as production"
    );
}

#[test]
fn world_loading_transition_reuses_the_already_rendered_cover_entities() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(GameConfig::default()))
    .add_plugins(StreamTownGamePlugin);
    app.update();

    let before = app
        .world_mut()
        .query_filtered::<Entity, With<LoadingScreenEntity>>()
        .iter(app.world())
        .collect::<BTreeSet<_>>();
    assert!(!before.is_empty());
    let loading_camera_before = {
        let world = app.world_mut();
        let mut query = world.query_filtered::<(Entity, &Camera), With<LoadingUiCamera>>();
        let (entity, camera) = query
            .single(world)
            .expect("exactly one persistent loading UI camera");
        assert_eq!(camera.order, 1_000);
        assert!(matches!(
            camera.clear_color,
            bevy::camera::ClearColorConfig::None
        ));
        entity
    };
    assert!(
        app.world()
            .get::<StateEntity>(loading_camera_before)
            .is_none(),
        "the loading camera must survive scene-state cleanup"
    );
    let targeted_roots_before = {
        let world = app.world_mut();
        let mut query = world
            .query_filtered::<&UiTargetCamera, (With<LoadingScreenEntity>, Without<ChildOf>)>();
        query.iter(world).map(|target| target.0).collect::<Vec<_>>()
    };
    assert!(!targeted_roots_before.is_empty());
    assert!(
        targeted_roots_before
            .iter()
            .all(|target| *target == loading_camera_before),
        "every loading overlay root must use the stable UI-only camera"
    );

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::WorldLoading);
    app.update();

    let after = app
        .world_mut()
        .query_filtered::<Entity, With<LoadingScreenEntity>>()
        .iter(app.world())
        .collect::<BTreeSet<_>>();
    assert_eq!(before, after);
    let loading_camera_after = app
        .world_mut()
        .query_filtered::<Entity, With<LoadingUiCamera>>()
        .single(app.world())
        .expect("the persistent loading UI camera survives WorldLoading");
    assert_eq!(loading_camera_before, loading_camera_after);
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::WorldLoading
    );
}

#[test]
fn headless_launch_through_credits_round_trip_covers_shipping_states() {
    let config = GameConfig::default();
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("launch-through-credits.stbevy");
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(config))
    .add_plugins(StreamTownGamePlugin);
    app.insert_resource(SaveRuntime {
        store: NativeSaveStore::new(&save_path),
    });

    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::Boot
    );
    app.update();
    app.update();
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::MainMenu
    );
    assert!(
        app.world_mut()
            .query_filtered::<Entity, With<StateEntity>>()
            .iter(app.world())
            .count()
            > 0
    );
    let main_menu_actions = app
        .world_mut()
        .query::<&MainMenuAction>()
        .iter(app.world())
        .copied()
        .collect::<Vec<_>>();
    assert_eq!(main_menu_actions.len(), 6);
    assert!(main_menu_actions.contains(&MainMenuAction::LoadGame));
    assert!(main_menu_actions.contains(&MainMenuAction::Secrets));

    enter_headless_world(&mut app);
    for _ in 0..16 {
        app.update();
    }
    assert!(
        app.world_mut()
            .query_filtered::<Entity, With<LoadingScreenEntity>>()
            .iter(app.world())
            .next()
            .is_none(),
        "the loading cover must retire after the reveal gate"
    );
    assert!(
        app.world_mut()
            .query_filtered::<Entity, With<LoadingUiCamera>>()
            .iter(app.world())
            .next()
            .is_none(),
        "the no-clear loading camera must retire with its cover"
    );
    let viewer = StableId::new("twitch:acceptance_viewer").unwrap();
    app.world_mut()
        .resource_mut::<InjectedCommands>()
        .0
        .push_back(PendingChatCommand {
            actor_id: viewer.clone(),
            login_name: "acceptance_viewer".to_owned(),
            display_name: "Acceptance Viewer".to_owned(),
            command: ChatCommand::Join,
            is_broadcaster: false,
            is_moderator: false,
            is_subscriber: true,
            origin: CommandOrigin::LocalDebug,
        });
    app.update();
    assert!(
        app.world()
            .resource::<SimulationRuntime>()
            .0
            .actors
            .contains_key(&viewer)
    );

    app.world_mut().resource_mut::<MenuIoRequest>().save = true;
    app.update();
    let saved = NativeSaveStore::new(&save_path).load().unwrap();
    assert!(saved.simulation.actors.contains_key(&viewer));

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::MainMenu);
    app.update();
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::MainMenu
    );
    assert!(!app.world().contains_resource::<WorldRuntime>());
    assert!(!app.world().contains_resource::<SimulationRuntime>());

    app.world_mut().resource_mut::<MenuIoRequest>().load = true;
    enter_headless_world(&mut app);
    app.update();
    assert!(
        app.world()
            .resource::<SimulationRuntime>()
            .0
            .actors
            .contains_key(&viewer)
    );
    assert!(
        app.world()
            .resource::<RuntimeConsoleRuntime>()
            .last_result
            .starts_with("Loaded ")
    );

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::MainMenu);
    app.update();
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Credits);
    app.update();
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::Credits
    );
    assert!(app.world().contains_resource::<CreditsTimeline>());
    assert!(
        app.world_mut()
            .query::<&AuthoredCreditsElement>()
            .iter(app.world())
            .count()
            > 0
    );

    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        Duration::from_mins(1),
    ));
    app.update();
    assert!(
        app.world_mut()
            .query::<&CreditsFireworksEmitter>()
            .iter(app.world())
            .count()
            >= 2
    );
    let skip_button = app
        .world_mut()
        .query_filtered::<Entity, With<CreditsSkipButton>>()
        .single(app.world())
        .unwrap();
    app.world_mut()
        .entity_mut(skip_button)
        .insert(Interaction::Pressed);
    app.update();
    app.update();
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::MainMenu
    );
    assert!(!app.world().contains_resource::<CreditsTimeline>());
    assert_eq!(
        app.world_mut()
            .query::<&AuthoredCreditsElement>()
            .iter(app.world())
            .count(),
        0
    );
}

#[test]
fn headless_new_town_matches_shipping_starting_roster() {
    let config = GameConfig::default();
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("shipping-start.stbevy");
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(config))
    .add_plugins(StreamTownGamePlugin);
    app.insert_resource(SaveRuntime {
        store: NativeSaveStore::new(&save_path),
    });

    app.update();
    app.update();
    enter_headless_world(&mut app);

    {
        let simulation = &app.world().resource::<SimulationRuntime>().0;
        assert_eq!(simulation.actors.len(), 5);
        assert!(!simulation.enemy_camps.is_empty());
        assert!(simulation.enemy_camps.values().all(|camp| {
            camp.spawned_enemies.is_empty() && camp.spawn_remaining_seconds.abs() <= f64::EPSILON
        }));
        assert!(
            simulation
                .actors
                .values()
                .all(|actor| actor.role.as_str() != "role:enemy")
        );
        assert_eq!(recruited_actor_ids(simulation).len(), 5);
        for (index, (expected_id, expected_role)) in [
            ("npc:starting_defender", "role:defender"),
            ("npc:starting_logger", "role:logger"),
            ("npc:starting_miner", "role:miner"),
            ("npc:starting_gatherer", "role:gatherer"),
            ("npc:starting_builder", "role:builder"),
        ]
        .into_iter()
        .enumerate()
        {
            let actor_id = StableId::new(expected_id).unwrap();
            assert_eq!(
                simulation
                    .actors
                    .get(&actor_id)
                    .map(|actor| actor.role.as_str()),
                Some(expected_role),
                "starting actor {index}"
            );
        }
    }

    let agents: Vec<_> = app
        .world_mut()
        .query::<&Agent>()
        .iter(app.world())
        .map(|agent| (agent.id.clone(), agent.kind.clone()))
        .collect();
    assert_eq!(agents.len(), 5);
    assert!(agents.iter().all(|(_, kind)| *kind == ActorKind::Player));
}

#[test]
fn starting_logger_and_miner_recover_from_bad_targets_and_gather_resources() {
    let config = GameConfig::default();
    let save_directory = tempfile::tempdir().unwrap();
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(config))
    .add_plugins(StreamTownGamePlugin);
    app.insert_resource(SaveRuntime {
        store: NativeSaveStore::new(save_directory.path().join("active-workers.stbevy")),
    });
    app.update();
    app.update();
    enter_headless_world(&mut app);

    let logger = StableId::new("npc:starting_logger").unwrap();
    let miner = StableId::new("npc:starting_miner").unwrap();
    let wood = StableId::new("resource:wood").unwrap();
    let ore = StableId::new("resource:ore").unwrap();
    let resource_total = |world: &GeneratedWorld, kind: &StableId| {
        world
            .resources
            .iter()
            .filter(|resource| resource.kind == *kind)
            .map(|resource| u64::from(resource.amount))
            .sum::<u64>()
    };
    let initial_wood = resource_total(&app.world().resource::<WorldRuntime>().generated, &wood);
    let initial_ore = resource_total(&app.world().resource::<WorldRuntime>().generated, &ore);
    let mut logger_positions = BTreeSet::new();
    let mut miner_positions = BTreeSet::new();
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        Duration::from_millis(250),
    ));
    for _ in 0..720 {
        app.update();
        let simulation = &app.world().resource::<SimulationRuntime>().0;
        logger_positions.insert(simulation.actors[&logger].position);
        miner_positions.insert(simulation.actors[&miner].position);
        let world = &app.world().resource::<WorldRuntime>().generated;
        if resource_total(world, &wood) < initial_wood && resource_total(world, &ore) < initial_ore
        {
            break;
        }
    }
    let world = &app.world().resource::<WorldRuntime>().generated;
    assert!(
        resource_total(world, &wood) < initial_wood,
        "the starting logger must eventually gather wood"
    );
    assert!(
        resource_total(world, &ore) < initial_ore,
        "the starting miner must eventually gather ore"
    );
    assert!(
        logger_positions.len() > 2,
        "logger remained effectively AFK"
    );
    assert!(miner_positions.len() > 2, "miner remained effectively AFK");
}

#[test]
fn native_load_moves_town_hall_visual_to_saved_footprint() {
    let config = GameConfig::default();
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("moved-town-hall.stbevy");
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(config.clone()))
    .add_plugins(StreamTownGamePlugin);
    app.insert_resource(SaveRuntime {
        store: NativeSaveStore::new(&save_path),
    });

    app.update();
    app.update();
    enter_headless_world(&mut app);
    app.world_mut().resource_mut::<MenuIoRequest>().save = true;
    app.update();

    let store = NativeSaveStore::new(&save_path);
    let mut snapshot = store.load().unwrap();
    let town_hall_id = StableId::new("building:townhall").unwrap();
    let saved_position = GridPos { x: 6, z: 8 };
    snapshot
        .simulation
        .buildings
        .get_mut(&town_hall_id)
        .unwrap()
        .position = saved_position;
    let relocated_actor_id = snapshot.actors[0].id.clone();
    snapshot.actors[0].grid_position = saved_position;
    snapshot
        .simulation
        .actors
        .get_mut(&relocated_actor_id)
        .unwrap()
        .position = saved_position;
    store.write(&snapshot).unwrap();

    app.world_mut().resource_mut::<MenuIoRequest>().load = true;
    app.update();

    let content = &app.world().resource::<RuntimeContent>().0;
    let footprint = content.buildings[&town_hall_id].footprint;
    let centre = GridPos {
        x: saved_position.x + footprint[0] / 2,
        z: saved_position.z + footprint[1] / 2,
    };
    let expected_surface = grid_to_world_on_surface(
        centre,
        &config,
        &app.world().resource::<WorldRuntime>().generated,
    );
    let (location, presentation, transform) = app
        .world_mut()
        .query_filtered::<(&GridLocation, &BuildingPresentation, &Transform), With<TownHall>>()
        .single(app.world())
        .unwrap();
    assert_eq!(location.0, saved_position);
    assert_eq!(transform.translation.xz(), expected_surface.xz());
    assert_eq!(transform.translation, presentation.base_translation);
    assert_eq!(
        app.world().resource::<SimulationRuntime>().0.buildings[&town_hall_id].position,
        saved_position
    );
    let restored_station = station_candidate(
        &app.world().resource::<RuntimeContent>().0,
        &app.world().resource::<SimulationRuntime>().0,
        &config,
        &town_hall_id,
    )
    .unwrap();
    assert_eq!(restored_station.position, centre);
    assert_eq!(
        restored_town_hall_position(
            &app.world().resource::<RuntimeContent>().0,
            &app.world().resource::<SimulationRuntime>().0,
            &config,
        ),
        centre
    );
    assert_ne!(centre, town_hall_grid_position(&config));
    let (relocated_ecs_position, relocated_navigation_position) = app
        .world_mut()
        .query::<(&Agent, &GridLocation)>()
        .iter(app.world())
        .find(|(agent, _)| agent.id == relocated_actor_id)
        .map(|(agent, location)| (location.0, agent.navigation_position))
        .unwrap();
    assert_ne!(relocated_ecs_position, saved_position);
    assert_eq!(
        relocated_navigation_position,
        placement_to_navigation_centre(relocated_ecs_position)
    );
    assert_eq!(
        app.world().resource::<SimulationRuntime>().0.actors[&relocated_actor_id].position,
        relocated_ecs_position
    );
    let region = building_region(
        saved_position,
        footprint,
        &app.world().resource::<WorldRuntime>().generated,
    )
    .unwrap();
    for z in region.min.z..=region.max.z {
        for x in region.min.x..=region.max.x {
            assert!(
                !app.world()
                    .resource::<WorldRuntime>()
                    .generated
                    .navigation
                    .is_walkable(GridPos { x, z })
            );
        }
    }
}

#[test]
fn native_load_restores_saved_world_seed_after_runtime_config_changes() {
    let config = GameConfig::default();
    let saved_seed = config.world.seed;
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("saved-seed.stbevy");
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(config))
    .add_plugins(StreamTownGamePlugin);
    app.insert_resource(SaveRuntime {
        store: NativeSaveStore::new(&save_path),
    });

    app.update();
    app.update();
    enter_headless_world(&mut app);
    app.world_mut().resource_mut::<MenuIoRequest>().save = true;
    app.update();

    let saved = NativeSaveStore::new(&save_path).load().unwrap();
    let saved_wood = town_resource_amount(&saved.simulation, "resource:wood");
    app.world_mut().resource_mut::<RuntimeConfig>().0.world.seed = saved_seed.wrapping_add(1);
    app.world_mut()
        .resource_mut::<SimulationRuntime>()
        .0
        .town_resources
        .insert(StableId::new("resource:wood").unwrap(), 0);
    app.world_mut().resource_mut::<MenuIoRequest>().load = true;
    app.update();

    assert_eq!(
        app.world().resource::<WorldRuntime>().generated.seed,
        saved_seed
    );
    assert_eq!(
        app.world().resource::<RuntimeConfig>().0.world.seed,
        saved_seed
    );
    assert_eq!(
        app.world().resource::<SimulationRuntime>().0.world_seed,
        saved_seed
    );
    assert_eq!(
        town_resource_amount(
            &app.world().resource::<SimulationRuntime>().0,
            "resource:wood"
        ),
        saved_wood
    );
    assert_eq!(
        app.world().resource::<RuntimeConsoleRuntime>().last_result,
        format!("Loaded {}", save_path.display())
    );
}

#[test]
fn native_load_preflight_preserves_live_town_on_invalid_building() {
    let config = GameConfig::default();
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("invalid-building.stbevy");
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(config))
    .add_plugins(StreamTownGamePlugin);
    app.insert_resource(SaveRuntime {
        store: NativeSaveStore::new(&save_path),
    });

    app.update();
    app.update();
    enter_headless_world(&mut app);
    app.world_mut().resource_mut::<MenuIoRequest>().save = true;
    app.update();

    let store = NativeSaveStore::new(&save_path);
    let mut snapshot = store.load().unwrap();
    let invalid_id = StableId::new("building:invalid_save_fixture").unwrap();
    snapshot.simulation.buildings.insert(
        invalid_id.clone(),
        BuildingState {
            id: invalid_id,
            archetype: StableId::new("archetype:building:missing").unwrap(),
            position: GridPos { x: 2, z: 2 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 100,
            complete: true,
        },
    );
    store.write(&snapshot).unwrap();

    let live_wood = 73;
    app.world_mut()
        .resource_mut::<SimulationRuntime>()
        .0
        .town_resources
        .insert(StableId::new("resource:wood").unwrap(), live_wood);
    let placer_owner = StableId::new("debug:preflight_owner").unwrap();
    app.world_mut().resource_mut::<BuildingPlacers>().0.insert(
        placer_owner.clone(),
        BuildingPlacement {
            building: StableId::new("building:wall").unwrap(),
            thick_path: false,
            position: GridPos { x: 3, z: 3 },
            navigation_position: None,
            rotation_quarter_turns: 0,
            line_start: None,
            line_end: None,
            path_cells: Vec::new(),
            inactivity_seconds: 0.0,
        },
    );
    let live_actor_ids = app
        .world_mut()
        .query::<&Agent>()
        .iter(app.world())
        .map(|agent| agent.id.clone())
        .collect::<BTreeSet<_>>();
    let live_world_hash = app
        .world()
        .resource::<WorldRuntime>()
        .generated
        .deterministic_hash
        .clone();

    app.world_mut().resource_mut::<MenuIoRequest>().load = true;
    app.update();

    let actor_ids_after = app
        .world_mut()
        .query::<&Agent>()
        .iter(app.world())
        .map(|agent| agent.id.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(actor_ids_after, live_actor_ids);
    assert_eq!(
        app.world()
            .resource::<WorldRuntime>()
            .generated
            .deterministic_hash,
        live_world_hash
    );
    assert_eq!(
        town_resource_amount(
            &app.world().resource::<SimulationRuntime>().0,
            "resource:wood"
        ),
        live_wood
    );
    assert!(
        app.world()
            .resource::<BuildingPlacers>()
            .0
            .contains_key(&placer_owner)
    );
    assert!(
        app.world()
            .resource::<RuntimeConsoleRuntime>()
            .last_result
            .contains("references unknown archetype")
    );
}

#[test]
fn headless_vertical_slice_spawns_three_hundred_agents() {
    let mut config = GameConfig::default();
    config.gameplay.initial_agents = 300;
    let expected = usize::from(config.gameplay.initial_agents);
    let save_directory = tempfile::tempdir().unwrap();
    let save_path = save_directory.path().join("command-save.stbevy");
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
    ))
    .insert_resource(RuntimeConfig(config))
    .add_plugins(StreamTownGamePlugin);
    app.insert_resource(SaveRuntime {
        store: NativeSaveStore::new(&save_path),
    });

    app.update();
    app.update();
    enter_headless_world(&mut app);

    let actual = app
        .world_mut()
        .query_filtered::<Entity, With<Agent>>()
        .iter(app.world())
        .count();
    assert_eq!(actual, expected);
    assert!(app.world().contains_resource::<WorldRuntime>());

    let enemies = app
        .world_mut()
        .query::<&Agent>()
        .iter(app.world())
        .filter(|agent| agent.kind == ActorKind::Enemy)
        .count();
    assert_eq!(enemies, 0);
    assert!(
        !app.world()
            .resource::<SimulationRuntime>()
            .0
            .enemy_camps
            .is_empty()
    );

    app.world_mut()
        .resource_mut::<InjectedCommands>()
        .0
        .push_back(PendingChatCommand {
            actor_id: StableId::new("twitch:debug_viewer").unwrap(),
            login_name: "debug_viewer".to_owned(),
            display_name: "debug_viewer".to_owned(),
            command: "!join".parse().unwrap(),
            is_broadcaster: true,
            is_moderator: true,
            is_subscriber: true,
            origin: CommandOrigin::LocalDebug,
        });
    app.update();
    let joined_count = app
        .world_mut()
        .query_filtered::<Entity, With<Agent>>()
        .iter(app.world())
        .count();
    assert_eq!(joined_count, expected + 1);
    assert!(
        app.world()
            .resource::<SimulationRuntime>()
            .0
            .actors
            .contains_key(&StableId::new("twitch:debug_viewer").unwrap())
    );
    assert_eq!(
        app.world().resource::<SimulationRuntime>().0.actors
            [&StableId::new("twitch:debug_viewer").unwrap()]
            .active_pet
            .as_ref()
            .map(StableId::as_str),
        Some("pet:red_panda")
    );

    let gm_commands = [
        ChatCommand::ToggleBuildCosts,
        ChatCommand::ToggleRoleLimits,
        ChatCommand::AddResource {
            resource: StableId::new("wood").unwrap(),
            amount: 123,
        },
        ChatCommand::GivePet {
            player: StableId::new("debug_viewer").unwrap(),
            pet: StableId::new("duck").unwrap(),
        },
        ChatCommand::LevelUpPlayer {
            player: StableId::new("debug_viewer").unwrap(),
            amount: 2,
        },
        ChatCommand::QueueEvent(StableId::new("fishgod").unwrap()),
    ];
    for command in gm_commands {
        app.world_mut()
            .resource_mut::<InjectedCommands>()
            .0
            .push_back(PendingChatCommand {
                actor_id: StableId::new("twitch:debug_viewer").unwrap(),
                login_name: "debug_viewer".to_owned(),
                display_name: "debug_viewer".to_owned(),
                command,
                is_broadcaster: false,
                is_moderator: false,
                is_subscriber: true,
                origin: CommandOrigin::LocalDebug,
            });
    }
    app.update();
    {
        let simulation = &app.world().resource::<SimulationRuntime>().0;
        assert!(!simulation.building_costs_enabled);
        assert!(!simulation.role_limits_enabled);
        assert_eq!(town_resource_amount(simulation, "resource:wood"), 5_123);
        assert!(
            !simulation.actors[&StableId::new("twitch:debug_viewer").unwrap()]
                .unlocked_pets
                .contains(&StableId::new("pet:duck").unwrap()),
            "recognized pet commands must remain explicit no-ops until pets are implemented"
        );
        assert_eq!(
            simulation.queued_events.len() + usize::from(simulation.fish_god.is_some()),
            1
        );
    }
    app.update();
    let simulation = &app.world().resource::<SimulationRuntime>().0;
    assert!(
        simulation.fish_god.is_some(),
        "queued={:?}, active={:?}, game_state={:?}",
        simulation.queued_events,
        simulation.active_event,
        app.world().resource::<State<GameState>>().get(),
    );
    for command in [
        ChatCommand::GameEventAction,
        ChatCommand::StopEvent,
        ChatCommand::AddResource {
            resource: StableId::new("wood").unwrap(),
            amount: -123,
        },
        ChatCommand::ToggleBuildCosts,
        ChatCommand::ToggleRoleLimits,
    ] {
        app.world_mut()
            .resource_mut::<InjectedCommands>()
            .0
            .push_back(PendingChatCommand {
                actor_id: StableId::new("twitch:debug_viewer").unwrap(),
                login_name: "debug_viewer".to_owned(),
                display_name: "debug_viewer".to_owned(),
                command,
                is_broadcaster: false,
                is_moderator: false,
                is_subscriber: true,
                origin: CommandOrigin::LocalDebug,
            });
    }
    app.update();
    {
        let simulation = &app.world().resource::<SimulationRuntime>().0;
        assert!(simulation.building_costs_enabled);
        assert!(simulation.role_limits_enabled);
        assert_eq!(town_resource_amount(simulation, "resource:wood"), 5_000);
        assert!(simulation.fish_god.is_none());
        assert!(simulation.active_event.is_none());
    }

    let available_building = {
        let content = &app.world().resource::<RuntimeContent>().0;
        let simulation = &app.world().resource::<SimulationRuntime>().0;
        content
            .buildings
            .iter()
            .find(|(id, building)| {
                building.placeable && building_is_unlocked(content, simulation, id)
            })
            .map(|(id, building)| {
                (
                    id.clone(),
                    building.clone(),
                    building_construction_cost(content, simulation, id, building),
                )
            })
            .expect("converted initial technology unlocks a placeable building")
    };
    let actor_id = StableId::new("twitch:debug_viewer").unwrap();
    {
        let site = {
            let simulation = &app.world().resource::<SimulationRuntime>().0;
            let world = &app.world().resource::<WorldRuntime>().generated;
            let content = &app.world().resource::<RuntimeContent>().0;
            let near = simulation.actors[&actor_id].position;
            let mut candidates = Vec::new();
            for z in 0..world.navigation.height() {
                for x in 0..world.navigation.width() {
                    let position = GridPos { x, z };
                    let placement = BuildingPlacement {
                        building: available_building.0.clone(),
                        thick_path: false,
                        position,
                        navigation_position: None,
                        rotation_quarter_turns: 0,
                        line_start: None,
                        line_end: None,
                        path_cells: Vec::new(),
                        inactivity_seconds: 0.0,
                    };
                    if building_placement_is_available(
                        content,
                        simulation,
                        world,
                        &placement,
                        &available_building.1,
                        None,
                    ) {
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
            candidates
                .into_iter()
                .next()
                .expect("vertical slice has an exact fine-grid building site")
        };
        app.world_mut()
            .resource_mut::<SimulationRuntime>()
            .0
            .actors
            .get_mut(&actor_id)
            .unwrap()
            .last_building_position = Some(site);
    }
    let commands = [
        ChatCommand::SelectRole(StableId::new("builder").unwrap()),
        ChatCommand::Experience,
        ChatCommand::Build(available_building.0.clone()),
        ChatCommand::MoveBuilding(vec![BuildingAction {
            direction: BuildingDirection::Rotate,
            amount: 4,
        }]),
        ChatCommand::ConfirmBuilding,
        ChatCommand::DismissRecruit(5),
        ChatCommand::DismissRecruit(4),
        ChatCommand::Recruit {
            role: StableId::new("miner").unwrap(),
            amount: 2,
        },
        ChatCommand::Role,
        ChatCommand::Health,
        ChatCommand::Target(None),
        ChatCommand::Station(None),
        ChatCommand::Customize {
            kind: CustomizationKind::Body,
            index: 3,
        },
        ChatCommand::Camera(vec![CameraAction {
            direction: CameraDirection::In,
            amount: 1,
        }]),
        ChatCommand::TechnologyVote,
        ChatCommand::Vote(StableId::new("1").unwrap()),
        ChatCommand::TriggerEvent(StableId::new("festival").unwrap()),
        ChatCommand::Ping,
        ChatCommand::Ping,
        ChatCommand::Save,
        ChatCommand::Help,
    ];
    for command in commands {
        app.world_mut()
            .resource_mut::<InjectedCommands>()
            .0
            .push_back(PendingChatCommand {
                actor_id: actor_id.clone(),
                login_name: "debug_viewer".to_owned(),
                display_name: "debug_viewer".to_owned(),
                command,
                is_broadcaster: true,
                is_moderator: true,
                is_subscriber: true,
                origin: CommandOrigin::LocalDebug,
            });
    }
    app.update();

    let pointer_actors = app
        .world_mut()
        .query::<&PingPointer>()
        .iter(app.world())
        .map(|pointer| pointer.actor.clone())
        .collect::<Vec<_>>();
    assert_eq!(pointer_actors, vec![actor_id.clone()]);
    {
        let mut query = app.world_mut().query::<&mut PingPointer>();
        query
            .single_mut(app.world_mut())
            .expect("deduplicated pointer exists")
            .elapsed_seconds = PING_POINTER_DURATION_SECONDS;
    }
    app.update();
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<PingPointer>>()
            .iter(app.world())
            .count(),
        0
    );

    let (placed_building, saved_building_id, food_before_revive) = {
        let simulation = &app.world().resource::<SimulationRuntime>().0;
        assert_eq!(simulation.actors[&actor_id].role.as_str(), "role:builder");
        assert_eq!(
            simulation
                .actors
                .keys()
                .filter(|id| id.as_str().starts_with("npc:recruit_"))
                .count(),
            2
        );
        assert_eq!(recruited_actor_ids(simulation).len(), 5);
        assert_eq!(town_resource_amount(simulation, "resource:recruit"), 5);
        assert_eq!(simulation.actors[&actor_id].customization.body_type, 2);
        assert_eq!(
            simulation.actors[&actor_id].building_rotation_quarter_turns,
            4
        );
        assert!(
            simulation.actors[&actor_id]
                .last_building_position
                .is_some()
        );
        assert_eq!(simulation.buildings.len(), 2);
        let placed_building = simulation
            .buildings
            .values()
            .find(|building| building.id.as_str() != "building:townhall")
            .unwrap()
            .clone();
        assert!(!placed_building.complete);
        let base_max_health = building_base_max_health(
            &app.world().resource::<RuntimeContent>().0,
            &available_building.1,
        );
        assert_eq!(
            placed_building.health,
            i32::try_from(base_max_health.div_ceil(10)).unwrap()
        );
        assert_eq!(placed_building.rotation_quarter_turns, 4);
        for resource in [
            "resource:food",
            "resource:gold",
            "resource:ore",
            "resource:wood",
        ] {
            let resource_id = StableId::new(resource).unwrap();
            assert_eq!(
                town_resource_amount(simulation, resource),
                5_000 - available_building.2[&resource_id]
            );
        }
        let vote = simulation
            .active_vote
            .as_ref()
            .expect("technology vote active");
        assert_eq!(vote.options.len(), TECHNOLOGY_VOTE_OPTION_COUNT);
        assert_eq!(vote.option_votes.get(&actor_id), vote.options.first());
        assert_eq!(simulation.active_event, Some(TownEvent::Festival));
        let saved_building_id = placed_building.id.clone();
        (
            placed_building,
            saved_building_id,
            town_resource_amount(simulation, "resource:food"),
        )
    };
    let mut runtime_building_ids: Vec<_> = app
        .world_mut()
        .query::<&RuntimeBuilding>()
        .iter(app.world())
        .map(|building| building.id.clone())
        .collect();
    runtime_building_ids.sort();
    let mut expected_runtime_buildings = vec![
        StableId::new("building:townhall").unwrap(),
        saved_building_id.clone(),
    ];
    expected_runtime_buildings.sort();
    assert_eq!(runtime_building_ids, expected_runtime_buildings);
    assert!(
        save_path.is_file(),
        "{}",
        app.world().resource::<RuntimeConsoleRuntime>().last_result
    );
    let saved = NativeSaveStore::new(&save_path).load().unwrap();
    assert_eq!(
        saved
            .actors
            .iter()
            .map(|actor| actor.id.clone())
            .collect::<BTreeSet<_>>(),
        saved
            .simulation
            .actors
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>()
    );
    assert_eq!(
        saved.simulation.buildings[&saved_building_id],
        placed_building
    );
    assert!(
        app.world()
            .resource::<CommandFeedback>()
            .0
            .contains(TWITCH_COMMAND_HELP_URL)
    );
    app.world_mut()
        .resource_mut::<SimulationRuntime>()
        .0
        .damage_actor(&actor_id, u32::MAX)
        .unwrap();
    app.world_mut()
        .resource_mut::<InjectedCommands>()
        .0
        .push_back(PendingChatCommand {
            actor_id: actor_id.clone(),
            login_name: "debug_viewer".to_owned(),
            display_name: "debug_viewer".to_owned(),
            command: ChatCommand::Revive(None),
            is_broadcaster: true,
            is_moderator: true,
            is_subscriber: true,
            origin: CommandOrigin::LocalDebug,
        });
    app.update();
    let simulation = &app.world().resource::<SimulationRuntime>().0;
    assert!(simulation.actors[&actor_id].alive);
    assert_eq!(
        town_resource_amount(simulation, "resource:food"),
        food_before_revive - 400
    );
    assert!(
        app.world()
            .resource::<CommandFeedback>()
            .0
            .contains("revived twitch:debug_viewer")
    );
    let revival_rings = app
        .world_mut()
        .query::<&HealingRingEffect>()
        .iter(app.world())
        .filter(|effect| effect.kind == HealingEffectKind::Revive)
        .count();
    let revival_mote_bars = app
        .world_mut()
        .query::<&HealingMoteEffect>()
        .iter(app.world())
        .filter(|effect| effect.kind == HealingEffectKind::Revive)
        .count();
    assert_eq!(revival_rings, 1);
    assert_eq!(revival_mote_bars, 8);

    {
        let technology_ids = app
            .world()
            .resource::<RuntimeContent>()
            .0
            .technology
            .nodes
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        let max_health = {
            let content = &app.world().resource::<RuntimeContent>().0;
            let simulation = &app.world().resource::<SimulationRuntime>().0;
            building_max_health(content, &simulation.buildings[&saved_building_id])
        };
        let mut simulation = app.world_mut().resource_mut::<SimulationRuntime>();
        let building = simulation.0.buildings.get_mut(&saved_building_id).unwrap();
        building.complete = true;
        building.health = max_health;
        simulation.0.unlocked_technology.extend(technology_ids);
        for amount in simulation.0.town_resources.values_mut() {
            *amount = 1_000_000;
        }
    }
    for command in [
        ChatCommand::LevelBuilding {
            building: available_building.0.clone(),
            index: 1,
            iterations: 2,
        },
        ChatCommand::LevelAll {
            building: available_building.0.clone(),
            target_level: 4,
        },
    ] {
        app.world_mut()
            .resource_mut::<InjectedCommands>()
            .0
            .push_back(PendingChatCommand {
                actor_id: actor_id.clone(),
                login_name: "debug_viewer".to_owned(),
                display_name: "debug_viewer".to_owned(),
                command,
                is_broadcaster: false,
                is_moderator: false,
                is_subscriber: true,
                origin: CommandOrigin::LocalDebug,
            });
    }
    app.update();
    assert!(app.world().resource::<SimulationRuntime>().0.buildings[&saved_building_id].level >= 2);
    let removed_position =
        app.world().resource::<SimulationRuntime>().0.buildings[&saved_building_id].position;

    // Staff privileges must not bypass the Ruler-only removal rule.
    app.world_mut()
        .resource_mut::<InjectedCommands>()
        .0
        .push_back(PendingChatCommand {
            actor_id: actor_id.clone(),
            login_name: "debug_viewer".to_owned(),
            display_name: "debug_viewer".to_owned(),
            command: ChatCommand::RemoveBuilding {
                building: available_building.0.clone(),
                index: 1,
            },
            is_broadcaster: true,
            is_moderator: true,
            is_subscriber: true,
            origin: CommandOrigin::LocalDebug,
        });
    app.update();
    assert!(
        app.world()
            .resource::<SimulationRuntime>()
            .0
            .buildings
            .contains_key(&saved_building_id)
    );

    app.world_mut()
        .resource_mut::<SimulationRuntime>()
        .0
        .current_ruler = Some(actor_id.clone());
    app.world_mut()
        .resource_mut::<InjectedCommands>()
        .0
        .push_back(PendingChatCommand {
            actor_id,
            login_name: "debug_viewer".to_owned(),
            display_name: "debug_viewer".to_owned(),
            command: ChatCommand::RemoveBuilding {
                building: available_building.0,
                index: 1,
            },
            is_broadcaster: false,
            is_moderator: false,
            is_subscriber: true,
            origin: CommandOrigin::LocalDebug,
        });
    app.update();
    assert_eq!(
        app.world()
            .resource::<SimulationRuntime>()
            .0
            .buildings
            .keys()
            .map(StableId::as_str)
            .collect::<Vec<_>>(),
        vec!["building:townhall"]
    );
    assert!(
        app.world()
            .resource::<WorldRuntime>()
            .generated
            .navigation
            .is_walkable(removed_position)
    );
    assert_eq!(
        app.world_mut()
            .query_filtered::<&RuntimeBuilding, Without<TownHall>>()
            .iter(app.world())
            .count(),
        0
    );
}

#[test]
fn world_grid_projection_round_trips() {
    let config = GameConfig::default();
    for cell in [
        GridPos { x: 0, z: 0 },
        GridPos { x: 31, z: 47 },
        GridPos { x: 63, z: 63 },
    ] {
        assert_eq!(
            world_to_grid(grid_to_world(cell, &config), &config),
            Some(cell)
        );
    }
}

#[test]
fn credits_fireworks_use_authored_activation_and_deterministic_emission() {
    let presentation = embedded_presentation();
    assert!(!credits_fireworks_active(
        &presentation,
        "VFX_FireWorks",
        55.99
    ));
    assert!(credits_fireworks_active(
        &presentation,
        "VFX_FireWorks",
        56.0
    ));
    assert!(
        (credits_fireworks_start(&presentation, "VFX_FireWorks (1)") - 56.0).abs() < f32::EPSILON
    );
    assert_eq!(expected_firework_launches(56.0, 56.0, 16.0), 1);
    assert_eq!(expected_firework_launches(56.5, 56.0, 16.0), 9);
    assert_eq!(expected_firework_launches(55.9, 56.0, 16.0), 0);

    let seed = firework_seed(1, 42, 31);
    assert_eq!(seed, firework_seed(1, 42, 31));
    assert_ne!(seed, firework_seed(0, 42, 31));
    assert!((0.0..=1.0).contains(&deterministic_unit(seed)));
    assert!((3..=7).contains(&deterministic_u16_range([3, 7], seed)));
}

#[test]
fn credits_fireworks_spawn_live_rockets_bursts_and_sparks() {
    let presentation = embedded_presentation();
    let effect_id = presentation
        .fireworks_effects
        .keys()
        .next()
        .unwrap()
        .clone();
    let rocket_lifetime = presentation.fireworks_effects[&effect_id].rocket_lifetime_seconds[1];
    let mut world = World::new();
    world.insert_resource(Time::<()>::default());
    world.insert_resource(RuntimePresentation(presentation));
    world.insert_resource(CreditsTimeline {
        elapsed_seconds: 56.0,
    });
    world.spawn(CreditsFireworksEmitter {
        target_path: "VFX_FireWorks".into(),
        effect: effect_id.clone(),
        emitter_index: 0,
        origin_percent: Vec2::new(25.0, 92.0),
        next_launch_index: 0,
        active_rockets: 0,
    });

    let mut schedule = Schedule::default();
    schedule.add_systems(update_credits_fireworks);
    schedule.run(&mut world);
    assert_eq!(
        world
            .query::<&CreditsFireworkParticle>()
            .iter(&world)
            .filter(|particle| particle.kind == CreditsFireworkParticleKind::Rocket)
            .count(),
        1
    );

    let particle_entity = {
        let mut query = world.query::<(Entity, &CreditsFireworkParticle)>();
        query
            .iter(&world)
            .find(|(_, particle)| particle.kind == CreditsFireworkParticleKind::Rocket)
            .map(|(entity, _)| entity)
            .unwrap()
    };
    world
        .entity_mut(particle_entity)
        .get_mut::<CreditsFireworkParticle>()
        .unwrap()
        .age_seconds = rocket_lifetime;
    schedule.run(&mut world);
    assert!(world.query::<&CreditsFireworkBurst>().iter(&world).count() >= 1);
    assert!(
        world
            .query::<&CreditsFireworkParticle>()
            .iter(&world)
            .any(|particle| particle.kind == CreditsFireworkParticleKind::BurstFlash)
    );

    for mut burst in world
        .query::<&mut CreditsFireworkBurst>()
        .iter_mut(&mut world)
    {
        burst.delay_seconds = 0.0;
    }
    schedule.run(&mut world);
    assert!(
        world
            .query::<&CreditsFireworkParticle>()
            .iter(&world)
            .any(|particle| particle.kind == CreditsFireworkParticleKind::Spark)
    );
}

#[test]
fn embedded_chimney_smoke_preserves_authored_emitters_and_parameters() {
    let presentation = embedded_presentation();
    assert_eq!(presentation.chimney_smoke_effects.len(), 1);
    assert_eq!(
        presentation
            .prefab_chimney_emitters
            .values()
            .map(Vec::len)
            .sum::<usize>(),
        7
    );
    let effect = presentation.chimney_smoke_effects.values().next().unwrap();
    assert!((effect.emission_rate_per_second - 5.0).abs() < f32::EPSILON);
    assert!((effect.lifetime_seconds - 5.0).abs() < f32::EPSILON);
    assert!((effect.start_speed - 1.0).abs() < f32::EPSILON);
    assert!((effect.start_size - 0.25).abs() < f32::EPSILON);
    assert!((effect.cone_radius - 0.11).abs() < f32::EPSILON);
    assert!((effect.cone_angle_degrees - 6.1).abs() < f32::EPSILON);
    assert!(
        effect
            .alpha_over_lifetime
            .into_iter()
            .zip([1.0, 0.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert!(!effect.world_space);

    let house = presentation
        .prefab_chimney_emitters
        .get("5939659079433a24db8e34e97f888f7d")
        .unwrap();
    assert_eq!(house.len(), 2);
    assert_eq!(house[0].age, 1);
    assert!(
        house[0]
            .local_position
            .into_iter()
            .zip([-0.373_000_14, 2.201, -0.25])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert_eq!(house[1].age, 2);
}

#[test]
fn chimney_emission_and_world_transform_are_deterministic() {
    let mut accumulator = 0.0;
    assert_eq!(chimney_emission_count(&mut accumulator, 5.0, 0.1, 1_000), 0);
    assert!((accumulator - 0.5).abs() < f32::EPSILON);
    assert_eq!(chimney_emission_count(&mut accumulator, 5.0, 0.1, 1_000), 1);
    assert!(accumulator.abs() < f32::EPSILON);
    assert_eq!(chimney_emission_count(&mut accumulator, 5.0, 1.0, 3), 3);
    assert!((accumulator - 2.0).abs() < f32::EPSILON);

    let transform = GlobalTransform::from(
        Transform::from_xyz(10.0, 20.0, 30.0)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::splat(2.0)),
    );
    let position = chimney_emitter_world_position(&transform, Vec3::new(1.0, 2.0, 3.0));
    assert!(position.abs_diff_eq(Vec3::new(16.0, 24.0, 28.0), 0.000_01));
    assert!((chimney_emitter_world_scale(&transform) - 2.0).abs() < f32::EPSILON);

    let building = StableId::new("building:runtime_00000001").unwrap();
    assert_eq!(
        chimney_particle_seed(&building, 2, 41, 11),
        chimney_particle_seed(&building, 2, 41, 11)
    );
    assert_ne!(
        chimney_particle_seed(&building, 2, 41, 11),
        chimney_particle_seed(&building, 2, 42, 11)
    );
    assert_eq!(chimney_alpha_step(0.0), 0);
    assert_eq!(chimney_alpha_step(0.5), 4);
    assert_eq!(chimney_alpha_step(1.0), 7);
}

#[test]
fn rotated_footprints_and_building_moves_are_deterministic() {
    assert_eq!(rotated_footprint([2, 5], 0), [2, 5]);
    assert_eq!(rotated_footprint([2, 5], 1), [5, 2]);
    assert_eq!(rotated_footprint([2, 5], -1), [5, 2]);
    let world = generate_world(&GameConfig::default().world);
    let (position, rotation) = shift_grid_position(
        GridPos { x: 1, z: 1 },
        &[
            BuildingAction {
                direction: BuildingDirection::Left,
                amount: 4,
            },
            BuildingAction {
                direction: BuildingDirection::Up,
                amount: 3,
            },
            BuildingAction {
                direction: BuildingDirection::Rotate,
                amount: -2,
            },
        ],
        &world,
    );
    assert_eq!(position, GridPos { x: 4, z: 0 });
    assert_eq!(rotation, -2);

    let content = embedded_content();
    let house = &content.buildings[&StableId::new("building:house").unwrap()];
    assert_eq!(house.footprint, [2, 3]);
    assert_eq!(rotated_footprint(house.footprint, 1), [3, 2]);
    let square = &content.buildings[&StableId::new("building:stonemason").unwrap()];
    assert_eq!(square.footprint, [2, 2]);
    let runtime_id = StableId::new("building:rotation_test").unwrap();
    let mut simulation = WorldSimulation::new(1);
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id.clone(),
            archetype: square.archetype.clone(),
            position: GridPos { x: 8, z: 8 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 100,
            complete: true,
        },
    );
    assert_eq!(
        rotate_building_instance(&content, &mut simulation, &runtime_id, -1),
        Ok((square.display_name.clone(), 270))
    );
    assert_eq!(simulation.buildings[&runtime_id].rotation_quarter_turns, -1);
}

#[test]
fn building_direction_commands_follow_the_visible_town_axes() {
    let world = generate_world(&GameConfig::default().world);
    let origin = GridPos { x: 10, z: 10 };
    for (direction, expected) in [
        (BuildingDirection::Up, GridPos { x: 11, z: 10 }),
        (BuildingDirection::Down, GridPos { x: 9, z: 10 }),
        (BuildingDirection::Left, GridPos { x: 10, z: 9 }),
        (BuildingDirection::Right, GridPos { x: 10, z: 11 }),
    ] {
        let (actual, rotation) = shift_grid_position(
            origin,
            &[BuildingAction {
                direction,
                amount: 1,
            }],
            &world,
        );
        assert_eq!(actual, expected, "{direction:?}");
        assert_eq!(rotation, 0);
    }
}

#[test]
fn wall_lines_require_orthogonal_endpoints_and_include_both_ends() {
    let horizontal =
        wall_line_cells(GridPos { x: 8, z: 5 }, GridPos { x: 4, z: 5 }).expect("horizontal line");
    assert_eq!(horizontal.len(), 5);
    assert_eq!(horizontal.first(), Some(&GridPos { x: 4, z: 5 }));
    assert_eq!(horizontal.last(), Some(&GridPos { x: 8, z: 5 }));
    let vertical =
        wall_line_cells(GridPos { x: 3, z: 2 }, GridPos { x: 3, z: 6 }).expect("vertical line");
    assert_eq!(vertical.len(), 5);
    assert!(wall_line_cells(GridPos { x: 2, z: 2 }, GridPos { x: 3, z: 3 }).is_err());

    let wood = StableId::new("resource:wood").unwrap();
    let cost = BTreeMap::from([(wood.clone(), 7)]);
    assert_eq!(scaled_building_cost(&cost, horizontal.len())[&wood], 35);

    let mut placement = BuildingPlacement {
        building: StableId::new("building:wall").unwrap(),
        thick_path: false,
        position: GridPos { x: 10, z: 10 },
        navigation_position: None,
        rotation_quarter_turns: 0,
        line_start: None,
        line_end: None,
        path_cells: Vec::new(),
        inactivity_seconds: 0.0,
    };
    assert_eq!(placement_visual_cells(&placement), vec![placement.position]);
    placement.line_start = Some(placement.position);
    placement.position = GridPos { x: 10, z: 14 };
    assert_eq!(placement_visual_cells(&placement).len(), 5);
}

#[test]
fn building_placement_over_paths_removes_every_covered_path_section() {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world(&config.world);
    let house = &content.buildings[&StableId::new("building:house").unwrap()];
    let footprint = rotated_footprint(house.footprint, 0);
    let position = find_building_site(&world, town_hall_grid_position(&config), footprint)
        .expect("generated world has a house site");
    let region = building_region(position, footprint, &world).expect("house region");
    let outside = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .find(|cell| {
            !(region.min.x..=region.max.x).contains(&cell.x)
                || !(region.min.z..=region.max.z).contains(&cell.z)
        })
        .expect("world has a cell outside the house region");
    let path_archetype = content.buildings
        [&StableId::new("building:path").expect("static path ID")]
        .archetype
        .clone();
    let mut simulation = WorldSimulation::new(world.seed);
    let covered = [
        ("building:covered_path_a", region.min),
        ("building:covered_path_b", region.max),
    ];
    let outside_id = StableId::new("building:outside_path").unwrap();
    for (id, cell) in covered
        .iter()
        .copied()
        .chain([(outside_id.as_str(), outside)])
    {
        let id = StableId::new(id).unwrap();
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id: id.clone(),
                archetype: path_archetype.clone(),
                position: cell,
                rotation_quarter_turns: 0,
                level: 1,
                health: 100,
                complete: true,
            },
        );
        simulation
            .path_navigation_positions
            .insert(id, placement_to_navigation_centre(cell));
    }

    assert!(building_site_is_available_for_simulation(
        &content,
        &simulation,
        &world,
        position,
        footprint,
    ));
    let removed =
        remove_paths_covered_by_regions(&content, &mut world, &mut simulation, &[region]).unwrap();
    assert_eq!(
        removed.into_iter().collect::<BTreeSet<_>>(),
        covered
            .into_iter()
            .map(|(id, _)| StableId::new(id).unwrap())
            .collect()
    );
    for (id, _) in covered {
        let id = StableId::new(id).unwrap();
        assert!(!simulation.buildings.contains_key(&id));
        assert!(!simulation.path_navigation_positions.contains_key(&id));
    }
    assert!(simulation.buildings.contains_key(&outside_id));
    assert!(
        simulation
            .path_navigation_positions
            .contains_key(&outside_id)
    );
}

#[test]
fn paths_route_diagonally_on_single_fine_navigation_cells() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let simulation = WorldSimulation::new(world.seed);
    let origin = placement_to_navigation_centre(town_hall_grid_position(&config));
    let mut placement = BuildingPlacement {
        building: StableId::new("building:path").unwrap(),
        thick_path: false,
        position: navigation_to_placement(origin),
        navigation_position: Some(origin),
        rotation_quarter_turns: 0,
        line_start: Some(origin),
        line_end: None,
        path_cells: vec![origin],
        inactivity_seconds: 0.0,
    };
    let navigation = build_fine_navigation(&config, &content, &simulation, &world).unwrap();
    move_path_placement(
        &mut placement,
        &[
            BuildingAction {
                direction: BuildingDirection::Right,
                amount: 2,
            },
            BuildingAction {
                direction: BuildingDirection::Up,
                amount: 2,
            },
            BuildingAction {
                direction: BuildingDirection::Left,
                amount: 1,
            },
        ],
        &world,
        Some(&navigation),
        &content,
        &simulation,
    );

    let cells = placement_visual_cells(&placement);
    assert_eq!(cells.first(), Some(&origin));
    assert_eq!(cells.last(), placement.navigation_position.as_ref());
    assert!(
        cells
            .windows(2)
            .all(|pair| pair[0].x.abs_diff(pair[1].x) <= 1 && pair[0].z.abs_diff(pair[1].z) <= 1)
    );
    assert!(
        cells.windows(2).any(|pair| {
            pair[0].x.abs_diff(pair[1].x) == 1 && pair[0].z.abs_diff(pair[1].z) == 1
        })
    );
    assert!(cells.iter().any(|cell| cell.x != origin.x));
    assert!(cells.iter().any(|cell| cell.z != origin.z));
    assert_eq!(
        content.buildings[&StableId::new("building:path").unwrap()].placement_footprint_thirds,
        Some([1, 1])
    );
    assert_eq!(
        content.buildings[&StableId::new("building:path").unwrap()].navigation_footprint_thirds,
        Some([0, 0])
    );

    assert!(building_placement_is_available(
        &content,
        &simulation,
        &world,
        &placement,
        &content.buildings[&StableId::new("building:path").unwrap()],
        Some(&navigation),
    ));

    let mut occupied = simulation;
    occupied.path_navigation_positions.insert(
        StableId::new("building:runtime_path_occupied").unwrap(),
        cells[2],
    );
    assert!(building_placement_is_available(
        &content,
        &occupied,
        &world,
        &placement,
        &content.buildings[&StableId::new("building:path").unwrap()],
        Some(&navigation),
    ));
    assert!(
        !confirmed_path_cells(&content, &occupied, &placement, &navigation).contains(&cells[2])
    );
}

#[test]
fn thick_paths_keep_the_centre_route_and_skip_each_blocked_flank_independently() {
    let content = embedded_content();
    let mut navigation =
        stream_town_domain::NavGrid::new(6, 4, vec![false; 24], vec![0; 24]).unwrap();
    navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: GridPos { x: 2, z: 0 },
                max: GridPos { x: 2, z: 0 },
            },
            true,
        )
        .unwrap();
    let mut simulation = WorldSimulation::new(1);
    simulation.path_navigation_positions.insert(
        StableId::new("building:occupied_thick_flank").unwrap(),
        GridPos { x: 3, z: 2 },
    );
    let placement = BuildingPlacement {
        building: StableId::new("building:path").unwrap(),
        thick_path: true,
        position: GridPos { x: 0, z: 0 },
        navigation_position: Some(GridPos { x: 3, z: 1 }),
        rotation_quarter_turns: 1,
        line_start: Some(GridPos { x: 1, z: 1 }),
        line_end: None,
        path_cells: vec![
            GridPos { x: 1, z: 1 },
            GridPos { x: 2, z: 1 },
            GridPos { x: 3, z: 1 },
        ],
        inactivity_seconds: 0.0,
    };
    let world = generate_world(&GameConfig::default().world);
    assert!(building_placement_is_available(
        &content,
        &simulation,
        &world,
        &placement,
        &content.buildings[&placement.building],
        Some(&navigation),
    ));
    assert_eq!(placement_visual_cells(&placement).len(), 9);
    let confirmed = confirmed_path_cells(&content, &simulation, &placement, &navigation);
    assert_eq!(confirmed.len(), 7);
    assert!(
        placement
            .path_cells
            .iter()
            .all(|cell| confirmed.contains(cell))
    );
    assert!(!confirmed.contains(&GridPos { x: 2, z: 0 }));
    assert!(!confirmed.contains(&GridPos { x: 3, z: 2 }));
}

#[test]
fn diagonal_thick_paths_fill_both_inner_bridge_cells() {
    let content = embedded_content();
    let navigation = stream_town_domain::NavGrid::new(5, 5, vec![false; 25], vec![0; 25]).unwrap();
    let simulation = WorldSimulation::new(1);
    let placement = BuildingPlacement {
        building: StableId::new("building:path").unwrap(),
        thick_path: true,
        position: GridPos { x: 0, z: 0 },
        navigation_position: Some(GridPos { x: 2, z: 2 }),
        rotation_quarter_turns: 0,
        line_start: Some(GridPos { x: 1, z: 1 }),
        line_end: None,
        path_cells: vec![GridPos { x: 1, z: 1 }, GridPos { x: 2, z: 2 }],
        inactivity_seconds: 0.0,
    };

    let visual = placement_visual_cells(&placement);
    assert_eq!(visual.len(), 8);
    assert!(visual.contains(&GridPos { x: 1, z: 2 }));
    assert!(visual.contains(&GridPos { x: 2, z: 1 }));
    assert_eq!(
        confirmed_path_cells(&content, &simulation, &placement, &navigation),
        visual
    );
}

#[test]
fn path_construction_prefers_cell_centres_without_forbidding_side_thirds() {
    assert_eq!(path_construction_step_cost(GridPos { x: 4, z: 7 }, 10), 10);
    assert_eq!(path_construction_step_cost(GridPos { x: 5, z: 7 }, 10), 12);
    assert_eq!(path_construction_step_cost(GridPos { x: 5, z: 8 }, 14), 18);
}

#[test]
fn agents_use_an_open_side_third_when_the_approach_centre_is_blocked() {
    let content = embedded_content();
    let simulation = WorldSimulation::new(1);
    let start = GridPos { x: 0, z: 4 };
    let placement_goal = GridPos { x: 1, z: 1 };
    let centre = placement_to_navigation_centre(placement_goal);
    let mut navigation =
        stream_town_domain::NavGrid::new(9, 9, vec![true; 81], vec![0; 81]).unwrap();
    navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: start,
                max: GridPos { x: 3, z: 4 },
            },
            false,
        )
        .unwrap();

    let path = try_fine_agent_path(
        &navigation,
        &content,
        &simulation,
        &PathSurfaceRuntime::default(),
        &ActorKind::Player,
        start,
        placement_goal,
    )
    .expect("an open third of the coarse approach cell should remain reachable");

    assert_eq!(path.last(), Some(&GridPos { x: 3, z: 4 }));
    assert_ne!(path.last(), Some(&centre));
    assert_eq!(
        navigation_to_placement(*path.last().unwrap()),
        placement_goal
    );
}

#[test]
fn agents_try_reachable_side_thirds_when_a_walkable_centre_is_disconnected() {
    let content = embedded_content();
    let simulation = WorldSimulation::new(1);
    let start = GridPos { x: 0, z: 1 };
    let placement_goal = GridPos { x: 1, z: 0 };
    let centre = placement_to_navigation_centre(placement_goal);
    let mut navigation =
        stream_town_domain::NavGrid::new(6, 3, vec![false; 18], vec![0; 18]).unwrap();
    for blocked in [
        GridPos { x: 3, z: 1 },
        GridPos { x: 4, z: 0 },
        GridPos { x: 4, z: 2 },
        GridPos { x: 5, z: 1 },
    ] {
        navigation
            .set_blocked(
                stream_town_domain::DirtyRegion {
                    min: blocked,
                    max: blocked,
                },
                true,
            )
            .unwrap();
    }
    assert!(navigation.is_walkable(centre));
    assert!(navigation.find_path(start, centre).is_err());

    let path = try_fine_agent_path(
        &navigation,
        &content,
        &simulation,
        &PathSurfaceRuntime::default(),
        &ActorKind::Player,
        start,
        placement_goal,
    )
    .expect("a reachable side third must remain a valid approach");

    assert_ne!(path.last(), Some(&centre));
    assert_eq!(
        navigation_to_placement(*path.last().unwrap()),
        placement_goal
    );
}

#[test]
fn paths_can_route_through_the_three_completed_gate_door_cells() {
    let content = embedded_content();
    let gate_id = StableId::new("building:gate").unwrap();
    let gate = &content.buildings[&gate_id];
    let runtime_id = StableId::new("building:gate_path_test").unwrap();
    let mut simulation = WorldSimulation::new(1);
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id,
            archetype: gate.archetype.clone(),
            position: GridPos { x: 1, z: 1 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 100,
            complete: true,
        },
    );
    let gate_cells = completed_player_gate_navigation_cells(&content, &simulation);
    assert_eq!(gate_cells.len(), 3);
    let mut navigation =
        stream_town_domain::NavGrid::new(9, 9, vec![false; 81], vec![0; 81]).unwrap();
    for cell in &gate_cells {
        navigation
            .set_blocked(
                stream_town_domain::DirtyRegion {
                    min: *cell,
                    max: *cell,
                },
                true,
            )
            .unwrap();
    }
    let mut ordered = gate_cells.iter().copied().collect::<Vec<_>>();
    ordered.sort_unstable();
    let route =
        routed_path_cells(&navigation, &content, &simulation, ordered[0], ordered[2]).unwrap();
    assert!(ordered.iter().all(|cell| route.contains(cell)));
}

#[test]
fn legacy_paths_migrate_once_from_coarse_cells_without_overlap() {
    let mut simulation = WorldSimulation::new(9);
    for suffix in ["a", "b"] {
        let id = StableId::new(format!("building:path_{suffix}")).unwrap();
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id,
                archetype: StableId::new("archetype:building:path").unwrap(),
                position: GridPos { x: 4, z: 7 },
                rotation_quarter_turns: 0,
                level: 1,
                health: 100,
                complete: true,
            },
        );
    }

    assert_eq!(migrate_legacy_path_navigation_positions(&mut simulation), 2);
    let positions = simulation
        .path_navigation_positions
        .values()
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(positions.len(), 2);
    assert!(positions.contains(&GridPos { x: 13, z: 22 }));
    assert!(
        positions
            .iter()
            .all(|position| (12..=14).contains(&position.x) && (21..=23).contains(&position.z))
    );
    assert_eq!(migrate_legacy_path_navigation_positions(&mut simulation), 0);
}

#[test]
fn legacy_path_centres_expand_into_straight_and_corner_connectors() {
    let content = embedded_content();
    let path_definition = &content.buildings[&StableId::new("building:path").unwrap()];
    let mut simulation = WorldSimulation::new(10);
    let mut add_path = |name: &str, coarse: GridPos, fine: GridPos| {
        let id = StableId::new(format!("building:{name}")).unwrap();
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id: id.clone(),
                archetype: path_definition.archetype.clone(),
                position: coarse,
                rotation_quarter_turns: 0,
                level: 1,
                health: 100,
                complete: true,
            },
        );
        simulation.path_navigation_positions.insert(id, fine);
    };
    add_path(
        "path_corner",
        GridPos { x: 3, z: 3 },
        GridPos { x: 10, z: 10 },
    );
    add_path(
        "path_right",
        GridPos { x: 4, z: 3 },
        GridPos { x: 13, z: 10 },
    );
    add_path("path_up", GridPos { x: 3, z: 4 }, GridPos { x: 10, z: 13 });
    let path_id = StableId::new("building:path").unwrap();
    let corner_id = StableId::new("building:path_corner").unwrap();
    let corner = linear_navigation_cells(
        &content,
        &simulation,
        &simulation.buildings[&corner_id],
        &path_id,
    );
    assert_eq!(
        corner,
        vec![
            GridPos { x: 10, z: 10 },
            GridPos { x: 10, z: 11 },
            GridPos { x: 11, z: 10 },
        ]
    );
    let right_id = StableId::new("building:path_right").unwrap();
    assert_eq!(
        linear_navigation_cells(
            &content,
            &simulation,
            &simulation.buildings[&right_id],
            &path_id,
        ),
        vec![
            GridPos { x: 12, z: 10 },
            GridPos { x: 13, z: 10 },
            GridPos { x: 14, z: 10 },
        ]
    );
}

#[test]
fn path_route_cost_matches_the_five_percent_per_level_speed_bonus() {
    assert_eq!(path_route_step_cost(10, 0), 1_000);
    assert_eq!(path_route_step_cost(10, 1), 953);
    assert!(path_route_step_cost(10, 10) < path_route_step_cost(10, 1));
}

#[test]
fn routed_paths_traverse_existing_paths_without_rebuilding_them() {
    let navigation = stream_town_domain::NavGrid::new(5, 1, vec![false; 5], vec![0; 5]).unwrap();
    let mut simulation = WorldSimulation::new(1);
    let occupied = GridPos { x: 2, z: 0 };
    simulation
        .path_navigation_positions
        .insert(StableId::new("building:existing_path").unwrap(), occupied);

    let route = routed_path_cells(
        &navigation,
        &embedded_content(),
        &simulation,
        GridPos { x: 0, z: 0 },
        GridPos { x: 4, z: 0 },
    )
    .unwrap();
    assert!(route.contains(&occupied));

    let placement = BuildingPlacement {
        building: StableId::new("building:path").unwrap(),
        thick_path: false,
        position: GridPos { x: 1, z: 0 },
        navigation_position: Some(GridPos { x: 4, z: 0 }),
        rotation_quarter_turns: 0,
        line_start: Some(GridPos { x: 0, z: 0 }),
        line_end: None,
        path_cells: route,
        inactivity_seconds: 0.0,
    };
    let confirmed = confirmed_path_cells(&embedded_content(), &simulation, &placement, &navigation);
    assert!(!confirmed.contains(&occupied));
    assert_eq!(confirmed.len(), 4);
}

#[test]
fn enemies_receive_health_bars_without_a_player_only_authored_timeout() {
    let content = embedded_content();
    let enemy_archetype =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab")
            .expect("shipping Goblin archetype");
    let enemy = StableId::new("actor:enemy_health_bar").unwrap();
    let mut simulation = WorldSimulation::new(1);
    assert!(simulation.spawn_enemy(enemy.clone(), enemy_archetype, GridPos { x: 1, z: 1 }, 100,));
    assert_eq!(
        actor_health_bar_hide_seconds(&content, &simulation.actors[&enemy]),
        Some(3.0)
    );
    assert_eq!(
        actor_health_fill_color(&simulation.actors[&enemy]),
        Color::srgb(0.58, 0.16, 0.86)
    );
}

#[test]
fn guardhouse_defenders_round_trip_to_their_station_without_using_recruit_capacity() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world(&config.world);
    let guardhouse = &content.buildings[&StableId::new("building:guardhouse").unwrap()];
    let building_id = StableId::new("building:runtime_guardhouse_test").unwrap();
    let mut simulation = WorldSimulation::new(1);
    simulation.buildings.insert(
        building_id.clone(),
        BuildingState {
            id: building_id.clone(),
            archetype: guardhouse.archetype.clone(),
            position: GridPos { x: 4, z: 5 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 150,
            complete: true,
        },
    );
    let defender = guardhouse_defender_id(&building_id);
    let defender_position = GridPos { x: 20, z: 20 };
    assert!(simulation.join_player(defender.clone(), defender_position));
    simulation
        .assign_role(&defender, StableId::new("role:defender").unwrap())
        .unwrap();
    let mut recruited_defender = simulation.actors[&defender].clone();
    recruited_defender.id = StableId::new("npc:recruit_health_reference").unwrap();
    let guardhouse_health =
        effective_role_stats(&content, &simulation, &simulation.actors[&defender])
            .unwrap()
            .max_health;
    let recruited_health = effective_role_stats(&content, &simulation, &recruited_defender)
        .unwrap()
        .max_health;
    assert_eq!(guardhouse_health, recruited_health / 2);
    assert_eq!(
        guardhouse_for_defender(&simulation, &defender).unwrap().id,
        building_id
    );
    assert!(recruited_actor_ids(&simulation).is_empty());
    assert!(capacity_recruited_actor_ids(&simulation).is_empty());
    assert_eq!(simulation_player_count(&simulation), 0);

    let nearby = StableId::new("enemy:guard_nearby").unwrap();
    let near_guardhouse = StableId::new("enemy:near_guardhouse").unwrap();
    assert!(simulation.join_player(nearby.clone(), GridPos { x: 19, z: 20 }));
    assert!(simulation.join_player(near_guardhouse.clone(), GridPos { x: 5, z: 5 }));
    for enemy in [&nearby, &near_guardhouse] {
        simulation
            .assign_role(enemy, StableId::new("role:enemy").unwrap())
            .unwrap();
    }
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &guardhouse_defender_id(&building_id),
            defender_position,
        )
        .0,
        AgentGoal::Attack(nearby.clone())
    );
    simulation.actors.get_mut(&nearby).unwrap().alive = false;
    assert_eq!(
        next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &guardhouse_defender_id(&building_id),
            defender_position,
        )
        .0,
        AgentGoal::Attack(near_guardhouse)
    );
}

#[test]
fn endless_path_failures_trigger_only_after_five_seconds() {
    let actor = StableId::new("actor:path_failure").unwrap();
    let mut failures = PathFailureRuntime::default();
    assert!(!failures.record_failure(&actor, 4.99));
    assert!(failures.record_failure(&actor, 0.01));
    failures.clear(&actor);
    assert!(!failures.record_failure(&actor, 0.01));
}

#[test]
fn valid_route_without_world_progress_triggers_unstuck() {
    let actor = StableId::new("actor:movement_failure").unwrap();
    let target = GridPos { x: 8, z: 8 };
    let mut failures = PathFailureRuntime::default();
    assert!(!failures.record_movement(&actor, Vec2::ZERO, 1, target, 0.0, 0.1));
    assert!(!failures.record_movement(&actor, Vec2::ZERO, 1, target, 4.99, 0.1));
    assert!(failures.record_movement(&actor, Vec2::ZERO, 1, target, 0.01, 0.1));

    // Meaningful displacement resets the stall window even if the actor is
    // still working toward the same path waypoint.
    assert!(!failures.record_movement(&actor, Vec2::X, 1, target, 4.99, 0.1));
    assert!(!failures.record_movement(&actor, Vec2::X, 1, target, 0.0, 0.1));
}

#[test]
fn unstuck_recovery_rejects_a_walkable_but_isolated_fine_cell() {
    let actor = StableId::new("actor:fine_recovery").unwrap();
    let desired = GridPos { x: 10, z: 10 };
    let mut navigation =
        stream_town_domain::NavGrid::new(20, 20, vec![false; 400], vec![0; 400]).unwrap();
    for blocked in [
        GridPos { x: 9, z: 9 },
        GridPos { x: 10, z: 9 },
        GridPos { x: 11, z: 9 },
        GridPos { x: 9, z: 10 },
        GridPos { x: 11, z: 10 },
        GridPos { x: 9, z: 11 },
        GridPos { x: 10, z: 11 },
        GridPos { x: 11, z: 11 },
    ] {
        navigation
            .set_blocked(
                stream_town_domain::DirtyRegion {
                    min: blocked,
                    max: blocked,
                },
                true,
            )
            .unwrap();
    }
    assert!(navigation.is_walkable(desired));
    assert!(!navigation_component_reaches_minimum(
        &navigation,
        desired,
        64
    ));

    let recovered =
        fine_navigation_recovery_position(&navigation, desired, &actor, &BTreeMap::new()).unwrap();

    assert_ne!(recovered, desired);
    assert!(navigation_component_reaches_minimum(
        &navigation,
        recovered,
        64
    ));
}

#[test]
fn player_name_presets_are_deterministic_and_catalogued() {
    let actor = StableId::new("twitch:colour_test").unwrap();
    let color = preset_player_name_color(44, &actor);
    assert_eq!(color, preset_player_name_color(44, &actor));
    assert!(PLAYER_NAME_COLOR_PRESETS.contains(&color));
}

#[test]
fn healing_effect_extent_stays_within_character_scale() {
    let presentation = embedded_presentation();
    for kind in [
        HealingEffectKind::Channel,
        HealingEffectKind::Burst,
        HealingEffectKind::Revive,
    ] {
        let duration = healing_effect_duration(&presentation, kind);
        for step in 0_u16..=100 {
            let sample = healing_effect_sample(
                &presentation,
                kind,
                duration * f32::from(step) / 100.0,
                duration,
            );
            let target_scale = if kind == HealingEffectKind::Burst {
                HEALING_TARGET_EFFECT_SCALE
            } else {
                1.0
            };
            let ring_base_scale = match kind {
                HealingEffectKind::Channel => {
                    healing_channel_effect(&presentation).1.exposed_size
                        * HEALING_CHANNEL_RING_SCALE
                }
                HealingEffectKind::Burst | HealingEffectKind::Revive => HEALING_BURST_RING_SCALE,
            } * target_scale;
            let ring_world_radius = ring_base_scale * sample.ring_scale;
            let mote_transform_scale = if kind == HealingEffectKind::Channel {
                HEALING_MOTE_SCALE * sample.mote_scale * 1.2
            } else {
                HEALING_MOTE_SCALE
                    * healing_burst_effect(&presentation)
                        .1
                        .plus_size_multiplier(f32::from(step) / 100.0)
                        .unwrap_or_default()
                    * if kind == HealingEffectKind::Revive {
                        1.25
                    } else {
                        1.0
                    }
            };
            // VFX_Plus.glb has a six-metre maximum imported extent; the
            // channel uses the unit-diameter procedural sphere. Their
            // final geometry, not merely their transforms, stays hand-sized.
            let mote_mesh_extent = if kind == HealingEffectKind::Channel {
                1.0
            } else {
                6.000_001
            };
            let mote_world_extent = mote_transform_scale * mote_mesh_extent * target_scale;
            assert!(ring_world_radius <= 0.56 + f32::EPSILON, "{kind:?}");
            assert!(mote_world_extent <= 0.14 + f32::EPSILON, "{kind:?}");
            assert!(
                sample.radial_distance * target_scale <= 0.56 + f32::EPSILON,
                "{kind:?}"
            );
            assert!(sample.rise <= 0.9 + f32::EPSILON, "{kind:?}");
        }
    }
    assert!((HEALING_TARGET_EFFECT_SCALE - 1.0 / 8.0).abs() < f32::EPSILON);
    let material = healing_material([0.266_204_9, 16.948_38, 0.0, 1.0], 1.0);
    let emissive = material.emissive.to_f32_array();
    assert!(
        emissive[..3]
            .iter()
            .all(|component| *component <= HEALING_EMISSIVE_MAX_COMPONENT)
    );
}

#[test]
fn hud_building_count_excludes_town_hall_walls_paths_and_towers() {
    let content = embedded_content();
    let mut simulation = WorldSimulation::new(1);
    for (runtime, definition_id) in [
        ("building:townhall", "building:townhall"),
        ("building:test_wall", "building:wall"),
        ("building:test_path", "building:path"),
        ("building:test_tower", "building:tower"),
        ("building:test_house", "building:house"),
    ] {
        let definition = &content.buildings[&StableId::new(definition_id).unwrap()];
        let id = StableId::new(runtime).unwrap();
        simulation.buildings.insert(
            id.clone(),
            BuildingState {
                id,
                archetype: definition.archetype.clone(),
                position: GridPos { x: 4, z: 4 },
                rotation_quarter_turns: 0,
                level: 1,
                health: 100,
                complete: true,
            },
        );
    }
    assert_eq!(hud_building_count(&content, &simulation), 1);
}

#[test]
fn transient_dx12_surface_configuration_errors_are_recoverable() {
    let error = RenderError {
        ty: ErrorType::Validation,
        description: "In Surface::configure\n  Invalid surface".to_owned(),
        source: None,
    };
    assert!(is_transient_surface_configuration_error(&error));
    let unrelated = RenderError {
        ty: ErrorType::Validation,
        description: "some other validation error".to_owned(),
        source: None,
    };
    assert!(!is_transient_surface_configuration_error(&unrelated));
}

#[test]
fn water_shader_uses_scene_lighting() {
    let shader = include_str!("../../../assets/shaders/water_material.wgsl");
    assert!(shader.contains("apply_pbr_lighting(pbr_input)"));
    assert!(!shader.contains("keep it unlit"));
}

#[test]
fn enemy_flow_field_routes_each_component_to_its_nearest_town_target() {
    let width = 16_u16;
    let height = 8_u16;
    let mut blocked = vec![false; usize::from(width) * usize::from(height)];
    for z in 0..height {
        blocked[usize::from(z) * usize::from(width) + 7] = true;
    }
    let navigation = stream_town_domain::NavGrid::new(
        width,
        height,
        blocked,
        vec![0; usize::from(width) * usize::from(height)],
    )
    .unwrap();
    let wall = StableId::new("building:test_wall").unwrap();
    let town_hall = StableId::new("building:townhall").unwrap();
    let field = build_enemy_navigation_field(
        42,
        navigation.clone(),
        vec![
            EnemyRouteBuilding {
                id: town_hall.clone(),
                town_hall_distance: 0,
                approaches: vec![GridPos { x: 12, z: 4 }],
            },
            EnemyRouteBuilding {
                id: wall.clone(),
                town_hall_distance: 10,
                approaches: vec![GridPos { x: 6, z: 4 }, GridPos { x: 8, z: 4 }],
            },
        ],
    );

    let left = GridPos { x: 1, z: 1 };
    let right = GridPos { x: 14, z: 1 };
    let (left_target, left_goal) = field.destination(left).unwrap();
    let (right_target, right_goal) = field.destination(right).unwrap();
    assert_eq!(left_target, &wall);
    assert_eq!(left_goal, GridPos { x: 6, z: 4 });
    assert_eq!(right_target, &town_hall);
    assert_eq!(right_goal, GridPos { x: 12, z: 4 });
    let route = field.path_to(left, &wall, left_goal).unwrap();
    assert_eq!(route.first(), Some(&left));
    assert_eq!(route.last(), Some(&left_goal));
    assert!(
        route
            .iter()
            .all(|position| navigation.is_walkable(*position))
    );
}

#[test]
fn hierarchical_enemy_path_refines_a_valid_cluster_corridor() {
    let width = 24_u16;
    let height = 24_u16;
    let mut blocked = vec![false; usize::from(width) * usize::from(height)];
    for z in 0..height {
        if z != 5 {
            blocked[usize::from(z) * usize::from(width) + 8] = true;
        }
        if z != 18 {
            blocked[usize::from(z) * usize::from(width) + 16] = true;
        }
    }
    let navigation = stream_town_domain::NavGrid::new(
        width,
        height,
        blocked,
        vec![0; usize::from(width) * usize::from(height)],
    )
    .unwrap();
    let field = build_enemy_navigation_field(7, navigation.clone(), Vec::new());
    let start = GridPos { x: 2, z: 2 };
    let goal = GridPos { x: 22, z: 22 };
    let route = field.hierarchical_path(&navigation, start, goal).unwrap();
    assert_eq!(route.first(), Some(&start));
    assert_eq!(route.last(), Some(&goal));
    assert!(
        route
            .iter()
            .all(|position| navigation.is_walkable(*position))
    );
    assert!(route.windows(2).all(|cells| {
        let x = cells[0].x.abs_diff(cells[1].x);
        let z = cells[0].z.abs_diff(cells[1].z);
        x <= 1 && z <= 1 && x + z > 0
    }));
    assert!(route.windows(2).any(|cells| {
        cells[0].x.abs_diff(cells[1].x) == 1 && cells[0].z.abs_diff(cells[1].z) == 1
    }));
}
