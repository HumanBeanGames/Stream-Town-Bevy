pub(crate) fn spawn_main_menu(
    mut commands: Commands,
    presented_frames: Res<PresentedRenderFrames>,
    render: Res<RenderAssets>,
    presentation: Res<RuntimePresentation>,
    asset_server: Option<Res<AssetServer>>,
    meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut cameras: TownCameraMutQuery,
    mut sun: TownSunMutQuery,
) {
    commands.insert_resource(MenuRevealRuntime {
        started_at: Instant::now(),
        starting_render_frame: presented_frames.current(),
        ready_starting_render_frame: None,
        fallback_updates: 0,
        scene_ready_frames: 0,
    });
    let reference = embedded_main_menu_scene();
    let bake = reference
        .corrective_bake
        .as_ref()
        .expect("validated main-menu corrective bake");
    apply_authored_main_menu_camera(reference, &mut cameras);
    commands.spawn((
        StateEntity,
        Name::new("Main-menu gradient sky"),
        Mesh3d(render.menu_sky_mesh.clone()),
        MeshMaterial3d(render.menu_sky.clone()),
        Transform::from_translation(Vec3::from_array(reference.camera.position))
            .with_scale(Vec3::splat(reference.camera.far * 0.82)),
        bevy::light::NotShadowCaster,
        bevy::light::NotShadowReceiver,
    ));
    if let Ok((mut light, mut transform)) = sun.single_mut() {
        light.color = Color::srgb(1.0, 0.956_862_75, 0.839_215_7);
        light.illuminance = 14_000.0;
        light.shadow_maps_enabled = true;
        *transform = Transform::from_rotation(Quat::from_xyzw(
            -0.408_217_88,
            0.234_569_68,
            0.109_381_63,
            0.875_426_1,
        ));
    }
    commands.spawn((
        StateEntity,
        Name::new("Authored menu secondary light"),
        DirectionalLight {
            color: Color::srgb(1.0, 0.956_862_75, 0.839_215_7),
            illuminance: 7_000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_xyzw(
            0.951_251_27,
            0.044_943_463,
            -0.254_886_93,
            0.167_731_26,
        )),
    ));

    if let Some(mut meshes) = meshes {
        for embedded in &reference.embedded_meshes {
            commands.spawn((
                StateEntity,
                Name::new(format!("Authored menu mesh: {}", embedded.hierarchy_path)),
                Mesh3d(meshes.add(authored_main_menu_mesh(embedded, bake.water_height))),
                MeshMaterial3d(render.ground.clone()),
            ));
        }
        // The source prefab's water cube ended exactly at the flat terrain's
        // Y=0 surface. The corrective bake lowers the surface below the beach,
        // expands it past the camera frustum, and uses a plane so only the
        // animated water surface participates in transparent sorting.
        commands.spawn((
            StateEntity,
            Name::new("Baked main-menu ocean floor"),
            Mesh3d(render.cloud_plane.clone()),
            MeshMaterial3d(render.menu_ocean_floor.clone()),
            Transform::from_xyz(0.0, bake.ocean_floor_height, 0.0)
                .with_scale(Vec3::splat(bake.terrain_extent)),
        ));
        commands.spawn((
            StateEntity,
            Name::new("Baked main-menu animated water"),
            Mesh3d(render.cloud_plane.clone()),
            MeshMaterial3d(render.menu_water.clone()),
            Transform::from_xyz(0.0, bake.water_height, 0.0)
                .with_scale(Vec3::splat(bake.water_extent)),
            bevy::light::NotShadowCaster,
            bevy::light::NotShadowReceiver,
        ));
    }

    if asset_server.is_some() {
        let model_indices = reference
            .instances
            .iter()
            .enumerate()
            .filter(|(_, instance)| {
                menu_baked_position_visible(reference, Vec3::from_array(instance.position))
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let (resource_indices, foliage_indices) = main_menu_baked_decoration_indices(reference);
        let total = model_indices.len() + resource_indices.len() + foliage_indices.len();
        let direct_farms = model_indices
            .iter()
            .filter(|index| {
                reference.instances[**index]
                    .source_path
                    .ends_with("/Age02_Farm.fbx")
            })
            .count();
        info!(
            baked_instances = reference.instances.len(),
            rendered_instances = model_indices.len(),
            direct_farms,
            queued_resources = resource_indices.len(),
            queued_foliage = foliage_indices.len(),
            "queued frame-budgeted main-menu scene construction"
        );
        commands.insert_resource(MainMenuSpawnRuntime {
            phase: if model_indices.is_empty() {
                MainMenuSpawnPhase::Resources
            } else {
                MainMenuSpawnPhase::Models
            },
            model_indices,
            resource_indices,
            foliage_indices,
            cursor: 0,
            completed: 0,
            total,
            update_count: 0,
            starting_render_frame: presented_frames.current(),
        });
    }

    spawn_authored_main_menu_clouds(&mut commands, &render, materials.as_deref_mut());
    spawn_fish_school_scene(
        &mut commands,
        &presentation.0,
        &render,
        MAIN_MENU_SCENE_PATH,
        0,
        bake.water_height,
        true,
        Some(reference),
        None,
    );
    commands.spawn((
        StateEntity,
        Name::new("Shipping Main Menu backdrop"),
        ImageNode::new(main_menu_texture(&render, MAIN_MENU_TEXTURE_PATHS[3]))
            .with_color(Color::srgba(0.090_196_08, 0.109_803_93, 0.203_921_59, 0.55))
            .with_mode(NodeImageMode::Stretch),
        GlobalZIndex(8),
        Node {
            position_type: PositionType::Absolute,
            left: px(0),
            top: px(0),
            width: percent(50.0),
            height: percent(100.0),
            ..default()
        },
    ));
    if let Some(game_logo) = &render.game_logo {
        commands.spawn((
            StateEntity,
            Name::new("GameLogo (Unity parity)"),
            ImageNode::new(game_logo.clone()),
            GlobalZIndex(10),
            Node {
                position_type: PositionType::Absolute,
                top: percent(4.9),
                left: px(0),
                width: percent(37.3),
                height: percent(35.1),
                aspect_ratio: Some(GAME_LOGO_ASPECT_RATIO),
                ..default()
            },
        ));
    } else {
        commands.spawn((
            StateEntity,
            Text::new("STREAM TOWN"),
            TextFont {
                font_size: FontSize::Px(64.0),
                ..default()
            },
            TextLayout::justify(Justify::Center),
            TextColor(Color::srgb(0.86, 0.95, 0.84)),
            Node {
                position_type: PositionType::Absolute,
                top: percent(12.0),
                left: percent(6.0),
                ..default()
            },
        ));
    }
    commands
        .spawn((
            StateEntity,
            Name::new("Shipping Main Menu buttons"),
            GlobalZIndex(10),
            Node {
                position_type: PositionType::Absolute,
                left: percent(12.65),
                top: percent(44.1),
                width: percent(12.1),
                height: percent(37.9),
                flex_direction: FlexDirection::Column,
                row_gap: px(20),
                ..default()
            },
        ))
        .with_children(|parent| {
            for action in [
                MainMenuAction::NewGame,
                MainMenuAction::LoadGame,
                MainMenuAction::Settings,
                MainMenuAction::Secrets,
                MainMenuAction::Credits,
                MainMenuAction::Quit,
            ] {
                parent
                    .spawn((
                        action,
                        Button,
                        authored_ui_image(
                            &render,
                            MAIN_MENU_TEXTURE_PATHS[0],
                            main_menu_texture(&render, MAIN_MENU_TEXTURE_PATHS[0]),
                        ),
                        Node {
                            width: percent(100.0),
                            flex_grow: 1.0,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new(main_menu_action_label(action)),
                            TextFont {
                                font_size: FontSize::Px(24.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                            Pickable::IGNORE,
                        ));
                    });
            }
        });
    commands.spawn((
        StateEntity,
        Name::new("Shipping Main Menu copyright"),
        Text::new("© 2022 Panda Belly"),
        TextFont {
            font_size: FontSize::Px(16.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::WHITE),
        GlobalZIndex(10),
        Node {
            position_type: PositionType::Absolute,
            left: percent(11.8),
            top: percent(89.4),
            width: percent(13.8),
            ..default()
        },
    ));
}

pub(crate) const fn main_menu_action_label(action: MainMenuAction) -> &'static str {
    match action {
        MainMenuAction::NewGame => "New Game",
        MainMenuAction::LoadGame => "Load Game",
        MainMenuAction::Settings => "Settings",
        MainMenuAction::Secrets => "Secrets",
        MainMenuAction::Credits => "Credits",
        MainMenuAction::Quit => "Quit",
    }
}

pub(crate) fn loading_icon_rotation(
    content: &ContentCatalog,
) -> Option<&stream_town_domain::RotatingNodeDef> {
    content
        .archetypes
        .values()
        .find(|archetype| {
            archetype
                .source_path
                .ends_with(LOADING_SCREEN_PREFAB_SUFFIX)
        })?
        .rotating_nodes
        .iter()
        .find(|rotating| rotating.hierarchy_path == LOADING_ICON_HIERARCHY_PATH)
}

pub(crate) fn spawn_loading_screen(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    render: Option<Res<RenderAssets>>,
    existing: Query<(), With<LoadingScreenEntity>>,
    loading_cameras: Query<Entity, With<LoadingUiCamera>>,
) {
    // The overlay is deliberately persistent across Boot/MainMenu ->
    // WorldLoading. Recreating already-decoded ImageNodes still costs one render
    // extraction frame; despawning the old scene in that frame exposed the
    // camera clear colour as a black flash.
    if !existing.is_empty() {
        return;
    }
    // The town camera changes projection, HDR, tonemapping, and post-process
    // pipelines between MainMenu, WorldLoading, and InGame. Rendering the
    // loading UI through that camera lets a first-use pipeline rebuild blank
    // the entire window even though the overlay entities remain alive. Keep a
    // separate, stable UI-only camera above the world for the complete loading
    // operation so scene-camera compilation can never interrupt the cover. It
    // is retired with the cover; leaving a no-clear camera on the swapchain can
    // preserve its last SDR frame over the HDR world.
    let loading_camera = loading_cameras.iter().next().unwrap_or_else(|| {
        commands
            .spawn((
                LoadingUiCamera,
                Name::new("Persistent loading UI camera"),
                Camera2d,
                Camera {
                    order: 1_000,
                    clear_color: bevy::camera::ClearColorConfig::None,
                    ..default()
                },
                Msaa::Off,
            ))
            .id()
    });
    // This solid layer is immediately renderable even before the authored PNG
    // finishes its own first asset-server round trip. It prevents the partially
    // instantiated destination scene from showing through during boot.
    commands.spawn((
        LoadingScreenEntity,
        UiTargetCamera(loading_camera),
        BackgroundColor(Color::srgb(0.025, 0.04, 0.055)),
        GlobalZIndex(1_000),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100.0),
            height: percent(100.0),
            ..default()
        },
    ));
    if let Some(background) = render
        .as_deref()
        .and_then(|render| render.loading_screen.as_ref())
    {
        commands.spawn((
            LoadingScreenEntity,
            UiTargetCamera(loading_camera),
            ImageNode::new(background.clone()).with_mode(NodeImageMode::Stretch),
            GlobalZIndex(1_001),
            Node {
                position_type: PositionType::Absolute,
                width: percent(100.0),
                height: percent(100.0),
                ..default()
            },
        ));
    }
    if let Some(overlay) = render
        .as_deref()
        .and_then(|render| render.loading_overlay.as_ref())
    {
        commands.spawn((
            LoadingScreenEntity,
            UiTargetCamera(loading_camera),
            ImageNode::new(overlay.clone()).with_mode(NodeImageMode::Stretch),
            GlobalZIndex(1_002),
            Node {
                position_type: PositionType::Absolute,
                width: percent(100.0),
                height: percent(100.0),
                ..default()
            },
        ));
    }
    commands.spawn((
        LoadingScreenEntity,
        UiTargetCamera(loading_camera),
        LoadingStatusText,
        Text::new("Preparing town..."),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::srgb(0.93, 0.91, 0.78)),
        GlobalZIndex(1_004),
        Node {
            position_type: PositionType::Absolute,
            left: percent(34.0),
            bottom: percent(20.0),
            width: percent(32.0),
            ..default()
        },
    ));
    commands.spawn((
        LoadingScreenEntity,
        UiTargetCamera(loading_camera),
        LoadingSubstatusText,
        Text::new("Discovering dependencies..."),
        TextFont {
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::srgb(0.75, 0.76, 0.78)),
        GlobalZIndex(1_004),
        Node {
            position_type: PositionType::Absolute,
            left: percent(30.0),
            bottom: percent(17.0),
            width: percent(40.0),
            ..default()
        },
    ));
    commands.spawn((
        LoadingScreenEntity,
        UiTargetCamera(loading_camera),
        LoadingPercentText,
        Text::new("0%"),
        TextFont {
            font_size: FontSize::Px(16.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::WHITE),
        GlobalZIndex(1_004),
        Node {
            position_type: PositionType::Absolute,
            left: percent(47.0),
            bottom: percent(15.0),
            width: percent(6.0),
            ..default()
        },
    ));
    commands
        .spawn((
            LoadingScreenEntity,
            UiTargetCamera(loading_camera),
            GlobalZIndex(1_004),
            Node {
                position_type: PositionType::Absolute,
                left: percent(30.0),
                bottom: percent(13.0),
                width: percent(40.0),
                height: px(10),
                padding: UiRect::all(px(1)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.03, 0.03, 0.04, 0.85)),
        ))
        .with_children(|parent| {
            parent.spawn((
                LoadingScreenEntity,
                LoadingProgressFill,
                loading_progress_accessibility_node(),
                Node {
                    width: percent(0.0),
                    height: percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
            ));
        });
    let tooltip = &content.0.loading_screen.tooltips[usize::try_from(config.0.world.seed)
        .unwrap_or_default()
        % content.0.loading_screen.tooltips.len()];
    commands.spawn((
        LoadingScreenEntity,
        UiTargetCamera(loading_camera),
        Text::new(tooltip.clone()),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::srgb(0.82, 0.82, 0.82)),
        GlobalZIndex(1_004),
        Node {
            position_type: PositionType::Absolute,
            left: percent(25.0),
            bottom: percent(4.0),
            width: percent(50.0),
            ..default()
        },
    ));
    let Some(icon) = render
        .as_deref()
        .and_then(|render| render.loading_icon.as_ref())
    else {
        return;
    };
    let rotation = loading_icon_rotation(&content.0)
        .expect("validated content contains the authored loading-icon rotation");
    commands.spawn((
        LoadingScreenEntity,
        UiTargetCamera(loading_camera),
        LoadingIconSpinner {
            radians_per_second: rotation.degrees_per_second.to_radians(),
        },
        ImageNode::new(icon.clone()).with_color(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
        UiTransform::IDENTITY,
        GlobalZIndex(1_004),
        Node {
            position_type: PositionType::Absolute,
            left: percent(45.0),
            bottom: percent(8.0),
            width: percent(10.0),
            aspect_ratio: Some(1.0),
            ..default()
        },
    ));
}
