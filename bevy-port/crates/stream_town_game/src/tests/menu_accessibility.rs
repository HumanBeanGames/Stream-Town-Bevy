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

    let sky = include_str!("../../../../assets/shaders/menu_sky_material.wgsl");
    assert!(sky.contains("horizon_color"));
    assert!(sky.contains("zenith_color"));
    assert!(sky.contains("sun_disc"));
    assert!(sky.contains("high_wisps"));
    let water = include_str!("../../../../assets/shaders/water_material.wgsl");
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
    assert_eq!((streaming.width, streaming.height), (640, 360));
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
    assert!(text.contains("Stream output: 640 x 360"));
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
