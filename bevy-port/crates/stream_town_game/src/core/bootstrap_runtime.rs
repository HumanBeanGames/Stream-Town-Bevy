pub use app::StreamTownGamePlugin;
pub use presentation::animation::preview_animation_asset_for_rig;

pub use bootstrap::{
    adaptive_music_preview_program, load_player_settings, load_runtime_config,
    player_settings_path, run, runtime_config_path, save_runtime_config,
};

fn locate_asset_root() -> PathBuf {
    let configured = std::env::var_os("STREAM_TOWN_ASSET_ROOT").map(PathBuf::from);
    let current = std::env::current_dir().ok().map(|path| path.join("assets"));
    let executable = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|path| path.join("assets")));
    let development = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    configured
        .into_iter()
        .chain(current)
        .chain(executable)
        .chain([development])
        .find(|path| path.is_dir())
        .unwrap_or_else(|| PathBuf::from("assets"))
}

fn embedded_content() -> ContentCatalog {
    let content: ContentCatalog =
        ron::from_str(include_str!("../../../../assets/content/catalog.ron"))
            .expect("checked-in Stream Town content catalog must parse");
    content
        .validate()
        .expect("checked-in Stream Town content catalog must validate");
    content
}

fn embedded_presentation() -> PresentationCatalog {
    let presentation: PresentationCatalog =
        ron::from_str(include_str!("../../../../assets/content/presentation.ron"))
            .expect("checked-in Stream Town presentation catalog must parse");
    presentation
        .validate()
        .expect("checked-in Stream Town presentation catalog must validate");
    presentation
}

fn setup_rendering(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    settings: Res<RuntimePlayerSettings>,
    presentation: Res<RuntimePresentation>,
    asset_server: Option<Res<AssetServer>>,
    core_assets: CoreRenderAssets,
    terrain_materials: Option<ResMut<Assets<TerrainMaterial>>>,
    water_materials: Option<ResMut<Assets<WaterMaterial>>>,
    building_materials: Option<ResMut<Assets<BuildingMaterial>>>,
    scene_materials: SceneMaterialAssets,
    specialty_materials: SpecialtyMaterialAssets,
    tree_materials: Option<ResMut<Assets<TreeMaterial>>>,
    grass_materials: Option<ResMut<Assets<GrassMaterial>>>,
    critter_materials: Option<ResMut<Assets<CritterMaterial>>>,
    flag_materials: Option<ResMut<Assets<FlagMaterial>>>,
) {
    let (
        CoreRenderAssets {
            images: Some(mut images),
            meshes: Some(mut meshes),
            materials: Some(mut materials),
        },
        Some(mut terrain_materials),
        Some(mut water_materials),
        Some(mut building_materials),
        SceneMaterialAssets {
            cloud: Some(mut cloud_materials),
            menu_sky: Some(mut menu_sky_materials),
            godray: Some(mut godray_materials),
        },
        SpecialtyMaterialAssets {
            giraffe: Some(mut giraffe_materials),
            bounds: Some(mut bounds_materials),
            character: Some(mut character_materials),
        },
        Some(mut tree_materials),
        Some(mut grass_materials),
        Some(mut critter_materials),
        Some(mut flag_materials),
    ) = (
        core_assets,
        terrain_materials,
        water_materials,
        building_materials,
        scene_materials,
        specialty_materials,
        tree_materials,
        grass_materials,
        critter_materials,
        flag_materials,
    )
    else {
        commands.insert_resource(RenderAssets::default());
        return;
    };
    let material_closeup = std::env::var_os("STREAM_TOWN_SMOKE_CLOSEUP").is_some();
    let animation_closeup = std::env::var_os("STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP").is_some();
    let resource_closeup = std::env::var_os("STREAM_TOWN_SMOKE_RESOURCE_CLOSEUP").is_some();
    let healing_closeup = std::env::var_os("STREAM_TOWN_SMOKE_HEALING_VFX").is_some();
    let combat_closeup = std::env::var_os("STREAM_TOWN_SMOKE_COMBAT_VFX").is_some();
    let building_closeup = std::env::var_os("STREAM_TOWN_SMOKE_BUILDING_VFX").is_some()
        || std::env::var_os("STREAM_TOWN_SMOKE_CHIMNEY").is_some();
    let ping_closeup = std::env::var_os("STREAM_TOWN_SMOKE_PING").is_some();
    let foliage_closeup = std::env::var_os("STREAM_TOWN_SMOKE_FOLIAGE").is_some();
    let shoreline_closeup = std::env::var_os("STREAM_TOWN_SMOKE_SHORELINE").is_some();
    let overlay_closeup = std::env::var_os("STREAM_TOWN_SMOKE_OVERLAYS").is_some();
    let seagull_closeup = std::env::var_os("STREAM_TOWN_SMOKE_SEAGULL").is_some();
    let flag_closeup = std::env::var_os("STREAM_TOWN_SMOKE_FLAG").is_some();
    let godray_closeup = std::env::var_os("STREAM_TOWN_SMOKE_GODRAY").is_some();
    let giraffe_closeup = std::env::var_os("STREAM_TOWN_SMOKE_GIRAFFE").is_some();
    let live_pet_closeup = std::env::var_os("STREAM_TOWN_SMOKE_PET").is_some();
    let placement_closeup = std::env::var_os("STREAM_TOWN_SMOKE_PLACEMENT").is_some();
    let fish_school_closeup = std::env::var_os("STREAM_TOWN_SMOKE_FISH_SCHOOL").is_some();
    let role_audio_closeup = std::env::var_os("STREAM_TOWN_SMOKE_ROLE_AUDIO").is_some();
    let smoke_viewport_height = if material_closeup {
        Some(96.0)
    } else if animation_closeup {
        Some(6.0)
    } else if resource_closeup {
        Some(24.0)
    } else if healing_closeup {
        Some(42.0)
    } else if combat_closeup {
        Some(48.0)
    } else if building_closeup {
        Some(58.0)
    } else if ping_closeup {
        Some(30.0)
    } else if foliage_closeup {
        Some(45.0)
    } else if shoreline_closeup {
        Some(48.0)
    } else if overlay_closeup {
        Some(105.0)
    } else if seagull_closeup {
        Some(80.0)
    } else if flag_closeup {
        Some(52.0)
    } else if godray_closeup {
        Some(86.0)
    } else if giraffe_closeup {
        Some(42.0)
    } else if live_pet_closeup {
        Some(10.0)
    } else if placement_closeup {
        Some(52.0)
    } else if fish_school_closeup {
        Some(42.0)
    } else if role_audio_closeup {
        Some(30.0)
    } else {
        None
    };
    let initial_camera_transform = default_town_camera_transform();
    let initial_projection = smoke_viewport_height.map_or_else(
        || town_camera_projection(UNITY_TOWN_CAMERA_FOV_DEGREES),
        |viewport_height| {
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical { viewport_height },
                near: -2_000.0,
                far: 2_000.0,
                ..OrthographicProjection::default_3d()
            })
        },
    );
    commands.spawn((
        TownCamera,
        TownCameraControllerRuntime::new(initial_camera_transform),
        Camera3d::default(),
        // The shipping camera is orthographic and all local lights occupy a
        // shallow top-down depth band. Spending Bevy's default 24 slices on Z
        // leaves very coarse screen tiles, causing each night light to be
        // evaluated across far more fragments than it can affect.
        ClusterConfig::FixedZ {
            total: 4_096,
            z_slices: 1,
            z_config: ClusterZConfig {
                first_slice_depth: 5.0,
                far_z_mode: ClusterFarZMode::Constant(4_000.0),
            },
            dynamic_resizing: true,
        },
        IsDefaultUiCamera,
        SpatialListener::new(0.2),
        initial_projection,
        AmbientLight {
            color: Color::srgb(0.70, 0.82, 0.92),
            brightness: in_game_ambient_brightness(90.0),
            ..default()
        },
        DistanceFog {
            color: Color::srgba(0.58, 0.72, 0.78, 0.10),
            falloff: FogFalloff::Linear {
                start: 560.0,
                end: 940.0,
            },
            ..default()
        },
        initial_camera_transform,
    ));
    commands.spawn((
        TownSun,
        DirectionalLight {
            illuminance: 14_000.0,
            shadow_maps_enabled: true,
            // Converted low-poly assets use small, centimetre-derived scene
            // transforms. A little more bias prevents coplanar faces from
            // intermittently shadowing themselves at town-camera distances.
            shadow_depth_bias: 0.04,
            shadow_normal_bias: 2.5,
            ..default()
        },
        in_game_sun_transform(),
    ));
    let authored_building =
        building_materials.add(building_material(&presentation.0, asset_server.as_deref()));
    let clouds = cloud_materials.add(cloud_material(&presentation.0, asset_server.as_deref()));
    let menu_sky = menu_sky_materials.add(menu_sky_material());
    let menu_cloud = materials.add(StandardMaterial {
        base_color: Color::srgb(0.96, 0.98, 1.0),
        perceptual_roughness: 1.0,
        unlit: true,
        ..default()
    });
    let menu_ocean_floor = materials.add(StandardMaterial {
        base_color: Color::srgb(0.025, 0.20, 0.30),
        perceptual_roughness: 1.0,
        unlit: true,
        ..default()
    });
    let godrays = godray_materials.add(godray_material(&presentation.0));
    let giraffe = giraffe_materials.add(giraffe_material(&presentation.0, asset_server.as_deref()));
    let authored_bounds = bounds_materials.add(bounds_material(&presentation.0, None));
    let placement_valid = bounds_materials.add(bounds_material(
        &presentation.0,
        Some(BUILDING_PLACEMENT_SUCCESS_COLOR),
    ));
    let placement_invalid = bounds_materials.add(bounds_material(
        &presentation.0,
        Some(BUILDING_PLACEMENT_FAIL_COLOR),
    ));
    let tree_material = tree_material(&presentation.0, asset_server.as_deref());
    let tree_wind = tree_material.extension.parameters.wind_controls;
    let tree = tree_materials.add(tree_material);
    // Menu foliage keeps a dedicated solid-colour material so the shared atlas
    // cannot leak blue, but remains lit so it responds to the authored sun and
    // receives the same ordinary world shadows as gameplay foliage.
    let menu_tree = materials.add(menu_tree_material());
    let grass_material = grass_material(&presentation.0, asset_server.as_deref());
    let grass_wind = grass_material.extension.parameters.wind_controls;
    let grass = grass_materials.add(grass_material);
    let critter = critter_materials.add(critter_material(&presentation.0, asset_server.as_deref()));
    let flag = flag_materials.add(flag_material(&presentation.0, asset_server.as_deref()));
    let game_logo = presentation_texture_handle(
        &presentation.0,
        asset_server.as_deref(),
        GAME_LOGO_TEXTURE_PATH,
    );
    let loading_screen = presentation_texture_handle(
        &presentation.0,
        asset_server.as_deref(),
        LOADING_SCREEN_TEXTURE_PATH,
    );
    let loading_overlay = presentation_texture_handle(
        &presentation.0,
        asset_server.as_deref(),
        LOADING_OVERLAY_TEXTURE_PATH,
    );
    let loading_icon = presentation_texture_handle(
        &presentation.0,
        asset_server.as_deref(),
        LOADING_ICON_TEXTURE_PATH,
    );
    let ui_font = asset_server
        .as_deref()
        .map(|asset_server| asset_server.load(UI_FONT_ASSET_PATH));
    let ui_display_font = asset_server
        .as_deref()
        .map(|asset_server| asset_server.load(UI_DISPLAY_FONT_ASSET_PATH));
    let main_menu_textures = MAIN_MENU_TEXTURE_PATHS
        .iter()
        .chain(GAME_MENU_TEXTURE_PATHS.iter())
        .chain(std::iter::once(&SETTINGS_BACKGROUND_TEXTURE_PATH))
        .filter_map(|source_path| {
            presentation_texture_handle(&presentation.0, asset_server.as_deref(), source_path)
                .map(|handle| ((*source_path).to_owned(), handle))
        })
        .collect();
    let top_bar_textures = TOP_BAR_TEXTURE_PATHS
        .iter()
        .filter_map(|source_path| {
            presentation_texture_handle(&presentation.0, asset_server.as_deref(), source_path)
                .map(|handle| ((*source_path).to_owned(), handle))
        })
        .collect();
    let selection_panel_textures = SELECTION_PANEL_TEXTURE_PATHS
        .iter()
        .filter_map(|source_path| {
            presentation_texture_handle(&presentation.0, asset_server.as_deref(), source_path)
                .map(|handle| ((*source_path).to_owned(), handle))
        })
        .collect();
    let vote_textures = VOTE_TEXTURE_PATHS
        .iter()
        .copied()
        .chain(std::iter::once(RULER_VOTE_TIMER_UNFILLED_PATH))
        .filter_map(|source_path| {
            presentation_texture_handle(&presentation.0, asset_server.as_deref(), source_path)
                .map(|handle| (source_path.to_owned(), handle))
        })
        .collect();
    let objective_textures = OBJECTIVE_TEXTURE_PATHS
        .iter()
        .filter_map(|source_path| {
            presentation_texture_handle(&presentation.0, asset_server.as_deref(), source_path)
                .map(|handle| ((*source_path).to_owned(), handle))
        })
        .collect();
    let current_event_textures = CURRENT_EVENT_TEXTURE_PATHS
        .iter()
        .filter_map(|source_path| {
            presentation_texture_handle(&presentation.0, asset_server.as_deref(), source_path)
                .map(|handle| ((*source_path).to_owned(), handle))
        })
        .collect();
    let ui_slicers = presentation
        .0
        .textures
        .values()
        .filter_map(|texture| {
            texture.sprite_border.map(|border| {
                (
                    texture.source_path.clone(),
                    TextureSlicer {
                        border: BorderRect::from(border),
                        center_scale_mode: default(),
                        sides_scale_mode: default(),
                        max_corner_scale: 1.0,
                    },
                )
            })
        })
        .collect();
    let presentation_materials: BTreeMap<StableId, ResolvedMaterialHandle> = presentation
        .0
        .materials
        .iter()
        .map(|(id, material)| {
            let resolved = if material.source_path == BUILDING_MATERIAL_PATH {
                ResolvedMaterialHandle::Building(authored_building.clone())
            } else if material.source_path == CLOUD_MATERIAL_PATH {
                ResolvedMaterialHandle::Cloud(clouds.clone())
            } else if material.source_path == GODRAY_MATERIAL_PATH {
                ResolvedMaterialHandle::Godray(godrays.clone())
            } else if material.source_path == GIRAFFE_MATERIAL_PATH {
                ResolvedMaterialHandle::Giraffe(giraffe.clone())
            } else if material.source_path == BOUNDS_MATERIAL_PATH {
                ResolvedMaterialHandle::Bounds(authored_bounds.clone())
            } else if material.source_path == TREE_MATERIAL_PATH {
                ResolvedMaterialHandle::Tree(tree.clone())
            } else if material.source_path == GRASS_MATERIAL_PATH {
                ResolvedMaterialHandle::Grass(grass.clone())
            } else if material.source_path == CRITTER_MATERIAL_PATH {
                ResolvedMaterialHandle::Critter(critter.clone())
            } else if material.source_path == FLAG_MATERIAL_PATH {
                ResolvedMaterialHandle::Flag(flag.clone())
            } else if material.shader_source.as_deref() == Some(CHARACTER_UNITY_SHADER_PATH) {
                ResolvedMaterialHandle::Character(character_materials.add(character_material(
                    material,
                    &presentation.0,
                    asset_server.as_deref(),
                )))
            } else {
                let standard =
                    standard_material(material, &presentation.0, asset_server.as_deref());
                let standard = if id.as_str() == SELECTION_MASK_MATERIAL_ID {
                    selection_outline_material(standard)
                } else {
                    standard
                };
                ResolvedMaterialHandle::Standard(materials.add(standard))
            };
            (id.clone(), resolved)
        })
        .collect();
    let presentation_materials_by_source_path = presentation
        .0
        .materials
        .iter()
        .filter_map(|(id, material)| {
            presentation_materials
                .get(id)
                .map(|resolved| (material.source_path.clone(), resolved.clone()))
        })
        .collect();
    let selection = StableId::new(SELECTION_MASK_MATERIAL_ID)
        .ok()
        .and_then(|id| presentation_materials.get(&id))
        .and_then(|material| match material {
            ResolvedMaterialHandle::Standard(material) => Some(material.clone()),
            _ => None,
        })
        .unwrap_or_else(|| {
            materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.983_102, 0.0),
                emissive: LinearRgba::new(0.877_358_5, 0.836_833_4, 0.0, 1.0),
                alpha_mode: AlphaMode::Mask(0.5),
                depth_bias: SELECTION_OUTLINE_DEPTH_BIAS,
                ..default()
            })
        });
    let chimney_smoke = presentation
        .0
        .chimney_smoke_effects
        .iter()
        .map(|(id, effect)| {
            let mut gradient = |color: [f32; 4]| {
                (0..CHIMNEY_ALPHA_STEPS)
                    .map(|step| {
                        let progress = chimney_alpha_progress(step);
                        let alpha = effect.alpha_over_lifetime[0]
                            + (effect.alpha_over_lifetime[1] - effect.alpha_over_lifetime[0])
                                * progress;
                        materials.add(StandardMaterial {
                            base_color: Color::linear_rgba(
                                color[0],
                                color[1],
                                color[2],
                                color[3] * alpha,
                            ),
                            alpha_mode: AlphaMode::Blend,
                            perceptual_roughness: 0.5,
                            ..default()
                        })
                    })
                    .collect()
            };
            (
                id.clone(),
                [
                    gradient(effect.start_color_min),
                    gradient(effect.start_color_max),
                ],
            )
        })
        .collect();
    let healing_channel = presentation
        .0
        .healing_channel_effects
        .iter()
        .filter_map(|(id, effect)| {
            effect
                .color
                .sample(0.06)
                .map(|color| (id.clone(), materials.add(healing_material(color, 0.72))))
        })
        .collect();
    let healing_plus_materials = presentation
        .0
        .healing_burst_effects
        .iter()
        .map(|(id, effect)| {
            (
                id.clone(),
                healing_gradient_materials(&mut materials, &effect.plus_color),
            )
        })
        .collect();
    let healing_disc_materials = presentation
        .0
        .healing_burst_effects
        .iter()
        .map(|(id, effect)| {
            (
                id.clone(),
                healing_gradient_materials(&mut materials, &effect.disc_color),
            )
        })
        .collect();
    let healing_plus = presentation
        .0
        .healing_burst_effects
        .values()
        .next()
        .and_then(|effect| {
            asset_server.as_deref().map(|server| {
                server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset(effect.plus_model_asset_path.clone()),
                )
            })
        });
    let fish_school_mesh = presentation
        .0
        .fish_school_effects
        .values()
        .find(|effect| effect.source_path.ends_with(FISH_SCHOOL_PREFAB_SUFFIX))
        .and_then(|effect| {
            asset_server.as_deref().map(|server| {
                server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset(effect.model_asset_path.clone()),
                )
            })
        });
    let water = water_material(&presentation.0, asset_server.as_deref());
    let water_wind = water.extension.parameters.wind_speed_noise_alpha;
    let menu_water = main_menu_water_material(water.clone());
    commands.insert_resource(AccessibilityMotionDefaults {
        tree: tree_wind,
        grass: grass_wind,
        water: water_wind,
    });
    let window_height =
        f32::from(u16::try_from(settings.0.video.height.max(1)).unwrap_or(u16::MAX));
    let traversal_wear = images.add(traversal_wear_image(
        config.0.world.width,
        config.0.world.height,
    ));
    let path_surface = images.add(traversal_wear_image(
        config.0.world.width,
        config.0.world.height,
    ));
    commands.insert_resource(RenderAssets {
        cube: meshes.add(Cuboid::default()),
        chimney_particle: meshes.add(Sphere::new(0.5).mesh().ico(1).expect("valid icosphere")),
        actor_lod: meshes.add(Capsule3d::new(0.42, 1.45)),
        menu_sky_mesh: meshes.add(
            Sphere::new(1.0)
                .mesh()
                .ico(4)
                .expect("valid main-menu sky sphere"),
        ),
        cloud_plane: meshes.add(Plane3d::default().mesh().size(1.0, 1.0)),
        healing_ring: meshes.add(healing_ring_mesh(48)),
        healing_plus,
        fish_school_mesh,
        fish_school_material: critter,
        projectile_arrow_scene: asset_server.as_deref().map(|asset_server| {
            asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("shipping/models/Models/Combat/Arrow.glb"),
            )
        }),
        ground: terrain_materials.add(terrain_material(
            &presentation.0,
            &config.0,
            asset_server.as_deref(),
            Some(traversal_wear.clone()),
            Some(path_surface.clone()),
        )),
        traversal_wear,
        path_surface,
        water: water_materials.add(water),
        menu_water: water_materials.add(menu_water),
        menu_sky,
        wood: materials.add(Color::srgb(0.16, 0.46, 0.18)),
        ore: materials.add(Color::srgb(0.46, 0.50, 0.55)),
        food: materials.add(Color::srgb(0.74, 0.64, 0.18)),
        building: materials.add(Color::srgb(0.42, 0.26, 0.12)),
        construction: materials.add(StandardMaterial {
            base_color: Color::srgb(0.72, 0.51, 0.24),
            perceptual_roughness: 0.88,
            ..default()
        }),
        streetlight_lamp: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.72, 0.24),
            emissive: LinearRgba::new(8.0, 3.0, 0.35, 1.0),
            perceptual_roughness: 0.35,
            ..default()
        }),
        regeneration_buildings: BTreeMap::from([
            (
                "11111111111111111111111111111111".to_owned(),
                materials.add(StandardMaterial {
                    base_color: Color::srgb(0.08, 0.30, 0.12),
                    perceptual_roughness: 0.82,
                    ..default()
                }),
            ),
            (
                "22222222222222222222222222222222".to_owned(),
                materials.add(StandardMaterial {
                    base_color: Color::srgb(0.30, 0.32, 0.35),
                    perceptual_roughness: 0.86,
                    ..default()
                }),
            ),
            (
                "33333333333333333333333333333333".to_owned(),
                materials.add(StandardMaterial {
                    base_color: Color::srgb(0.20, 0.08, 0.25),
                    perceptual_roughness: 0.82,
                    ..default()
                }),
            ),
        ]),
        placement_valid,
        placement_invalid,
        enemy_idle: materials.add(Color::srgb(0.72, 0.12, 0.12)),
        enemy_moving: materials.add(Color::srgb(1.0, 0.28, 0.22)),
        player_idle: materials.add(Color::srgb(0.35, 0.72, 0.95)),
        player_moving: materials.add(Color::srgb(0.52, 0.86, 1.0)),
        selection,
        rain: materials.add(StandardMaterial {
            base_color: Color::srgba(0.36, 0.66, 0.95, 0.62),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        snow: materials.add(StandardMaterial {
            base_color: Color::srgba(0.94, 0.98, 1.0, 0.92),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        projectile: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.58, 0.12),
            emissive: LinearRgba::new(3.5, 1.1, 0.08, 1.0),
            unlit: true,
            ..default()
        }),
        projectile_arrow: materials.add(StandardMaterial {
            base_color: Color::srgba(0.25, 0.25, 0.25, 0.62),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        projectile_necrotic: materials.add(StandardMaterial {
            base_color: Color::srgba(0.48, 0.08, 1.0, 0.84),
            emissive: LinearRgba::new(1.2, 0.03, 4.2, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        impact_physical: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 1.0, 1.0, 0.66),
            emissive: LinearRgba::new(1.2, 1.2, 1.2, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        building_smoke: materials.add(StandardMaterial {
            base_color: Color::srgba(0.21, 0.21, 0.21, 0.69),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        chimney_smoke,
        building_spark: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.34, 0.02, 0.96),
            emissive: LinearRgba::new(7.5, 0.48, 0.01, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        building_fire: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.18, 0.01, 0.86),
            emissive: LinearRgba::new(8.5, 0.32, 0.01, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        building_upgrade: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.66, 0.05, 0.9),
            emissive: LinearRgba::new(7.0, 2.1, 0.05, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        healing_green: materials.add(StandardMaterial {
            base_color: Color::srgba(0.18, 1.0, 0.12, 0.74),
            emissive: LinearRgba::new(0.28, 3.5, 0.14, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        healing_channel,
        healing_plus_materials,
        healing_disc_materials,
        authored_building,
        clouds,
        menu_cloud,
        menu_ocean_floor,
        tree,
        menu_tree,
        grass,
        game_logo,
        loading_screen,
        loading_overlay,
        loading_icon,
        ui_font,
        ui_display_font,
        main_menu_textures,
        top_bar_textures,
        selection_panel_textures,
        vote_textures,
        objective_textures,
        current_event_textures,
        ui_slicers,
        main_ui_scale: window_height / UNITY_MAIN_UI_REFERENCE_HEIGHT,
        settings_ui_scale: window_height / UNITY_SETTINGS_UI_REFERENCE_HEIGHT,
        presentation_materials,
        presentation_materials_by_source_path,
    });
}

fn apply_player_settings(
    settings: Res<RuntimePlayerSettings>,
    mut commands: Commands,
    mut cameras: Query<(Entity, &mut Projection), With<TownCamera>>,
    mut lights: Query<&mut DirectionalLight>,
    mut shadow_map: Option<ResMut<DirectionalLightShadowMap>>,
    mut winit: Option<ResMut<WinitSettings>>,
) {
    let benchmarking = std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some();
    let msaa = player_msaa(&settings.0);
    for (entity, mut projection) in &mut cameras {
        if let Projection::Perspective(perspective) = &mut *projection {
            perspective.fov = f32::from(settings.0.camera.field_of_view_degrees).to_radians();
        }
        let mut entity_commands = commands.entity(entity);
        entity_commands.insert(msaa);
        entity_commands.remove::<Fxaa>();
        entity_commands.remove::<Smaa>();
        match settings.0.video.post_process_aa {
            PostProcessAntiAliasing::None => {}
            PostProcessAntiAliasing::Fxaa => {
                entity_commands.insert(Fxaa::default());
            }
            PostProcessAntiAliasing::Smaa => {
                entity_commands.insert(Smaa::default());
            }
        }
        if settings.0.video.ambient_occlusion {
            entity_commands.insert(ScreenSpaceAmbientOcclusion::default());
        } else {
            entity_commands.remove::<ScreenSpaceAmbientOcclusion>();
        }
    }
    for mut light in &mut lights {
        light.shadow_maps_enabled = settings.0.video.shadows_enabled;
    }
    if let Some(shadow_map) = shadow_map.as_deref_mut() {
        shadow_map.size = usize::from(settings.0.video.shadow_map_resolution);
    }
    if let Some(winit) = winit.as_deref_mut() {
        winit.focused_mode = if benchmarking {
            UpdateMode::Continuous
        } else {
            settings
                .0
                .video
                .fps_limit
                .map_or(UpdateMode::Continuous, |limit| {
                    UpdateMode::reactive(Duration::from_secs_f64(1.0 / f64::from(limit)))
                })
        };
        if animation_binding_diagnostics_enabled() {
            // Automated GPU smoke windows cannot take focus away from the
            // desktop host. Keep only this explicit diagnostic path updating
            // continuously so elapsed/joint samples represent real frames.
            winit.unfocused_mode = UpdateMode::Continuous;
        }
    }
}

fn sync_primary_window_settings(
    settings: Res<RuntimePlayerSettings>,
    mut windows: Query<(&mut Window, Option<&OnMonitor>), With<PrimaryWindow>>,
) {
    let Ok((mut window, current_monitor)) = windows.single_mut() else {
        return;
    };
    // Direct-broadcast mode deliberately hides the swapchain window. Resizing
    // that hidden surface races DX12 capture on some drivers, so defer changes
    // until the local window becomes visible again.
    if !window.visible {
        return;
    }
    let benchmarking = std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some();
    let monitor = current_monitor.map(|monitor| monitor.0);
    let desired_mode = if benchmarking {
        WindowMode::Windowed
    } else {
        bootstrap::player_window_mode(settings.0.video.display_mode, monitor)
    };
    let desired_present_mode = if benchmarking {
        PresentMode::Immediate
    } else if settings.0.video.vsync {
        PresentMode::AutoVsync
    } else {
        PresentMode::AutoNoVsync
    };
    let desired_width = f32::from(u16::try_from(settings.0.video.width).unwrap_or(u16::MAX));
    let desired_height = f32::from(u16::try_from(settings.0.video.height).unwrap_or(u16::MAX));
    if window.mode != desired_mode {
        window.mode = desired_mode;
    }
    if window.present_mode != desired_present_mode {
        window.present_mode = desired_present_mode;
    }
    if (window.resolution.width() - desired_width).abs() > 0.5
        || (window.resolution.height() - desired_height).abs() > 0.5
    {
        window.resolution.set(desired_width, desired_height);
    }
}

fn apply_authored_ui_fonts(
    render: Res<RenderAssets>,
    mut texts: Query<(&mut TextFont, Option<&UiDisplayFont>), Added<TextFont>>,
) {
    for (mut text_font, display) in &mut texts {
        let authored = if display.is_some() {
            render.ui_display_font.as_ref()
        } else {
            render.ui_font.as_ref()
        };
        if let Some(authored) = authored {
            text_font.font = FontSource::Handle(authored.clone());
        }
    }
}

fn sync_authored_post_processing(
    mut commands: Commands,
    state: Res<State<GameState>>,
    settings: Res<RuntimePlayerSettings>,
    config: Res<RuntimeConfig>,
    catalog: Res<RuntimePresentation>,
    simulation: Option<Res<SimulationRuntime>>,
    environment: Res<EnvironmentPresentation>,
    mut runtime: ResMut<PostProcessPresentation>,
    mut cameras: TownPostProcessQuery,
) {
    let Ok((camera, color_grading, has_hdr, has_bloom, has_vignette, has_motion_blur, tonemapping)) =
        cameras.single_mut()
    else {
        return;
    };
    let daylight = if *state.get() == GameState::InGame {
        simulation.as_deref().map_or_else(
            || environment.daylight_bits.map_or(1.0, f32::from_bits),
            |simulation| {
                config
                    .0
                    .time
                    .sample(simulation.0.elapsed_seconds)
                    .daylight
                    .clamp(0.0, 1.0)
            },
        )
    } else {
        1.0
    };
    let daylight_bits = daylight_signature(daylight);
    let signature = (
        *state.get(),
        daylight_bits,
        i32::from_ne_bytes(settings.0.video.brightness_ev.to_ne_bytes()),
        i32::from_ne_bytes(settings.0.video.gamma.to_ne_bytes()),
    );
    if runtime.applied == Some(signature) {
        return;
    }
    let static_stack_changed = runtime
        .applied
        .is_none_or(|(applied_state, _, _, _)| applied_state != *state.get());

    let scene_path = match state.get() {
        GameState::InGame => Some(WORLD_SCENE_PATH),
        GameState::Credits => Some(CREDITS_SCENE_PATH),
        GameState::Boot | GameState::MainMenu | GameState::WorldLoading => None,
    };
    let stack = scene_path.map_or_else(Vec::new, |scene| {
        authored_post_process_stack(&catalog.0, scene, daylight)
    });
    let primary = stack
        .iter()
        .find_map(|(profile, weight)| (*weight > f32::EPSILON).then_some(*profile));
    let mut entity = commands.entity(camera);
    let next_color_grading = color_grading_for_state(&settings.0, &stack, *state.get());
    if let Some(mut color_grading) = color_grading {
        *color_grading = next_color_grading;
    } else {
        entity.insert(next_color_grading);
    }
    if primary.is_some() {
        if !has_hdr {
            entity.insert(Hdr);
        }
    } else if has_hdr {
        entity.remove::<Hdr>();
    }

    if let Some(bloom) = primary.and_then(|profile| profile.bloom) {
        if !has_bloom || static_stack_changed {
            entity.insert(Bloom {
                // Unity URP's authored intensity is not numerically equivalent to
                // Bevy's full-resolution additive blend. Mapping it onto Bevy's
                // natural, energy-conserving preset prevents white shoreline and
                // water highlights from flooding the entire HDR buffer.
                intensity: (bloom.intensity * 0.15).clamp(0.0, 0.35),
                low_frequency_boost: bloom.scatter,
                prefilter: BloomPrefilter {
                    threshold: bloom.threshold,
                    threshold_softness: 0.2,
                },
                composite_mode: BloomCompositeMode::EnergyConserving,
                ..Bloom::NATURAL
            });
        }
    } else if has_bloom {
        entity.remove::<Bloom>();
    }
    if let Some(vignette) = primary.and_then(|profile| profile.vignette) {
        if !has_vignette || static_stack_changed {
            entity.insert(Vignette {
                intensity: vignette.intensity,
                radius: 0.75,
                smoothness: vignette.smoothness,
                roundness: if vignette.rounded { 1.0 } else { 0.0 },
                center: Vec2::from_array(vignette.center),
                edge_compensation: 1.0,
                color: Color::srgba(
                    vignette.color[0],
                    vignette.color[1],
                    vignette.color[2],
                    vignette.color[3],
                ),
            });
        }
    } else if has_vignette {
        entity.remove::<Vignette>();
    }
    if motion_blur_supported()
        && let Some(motion_blur) = primary.and_then(|profile| profile.motion_blur)
    {
        if !has_motion_blur || static_stack_changed {
            entity.insert(MotionBlur {
                shutter_angle: motion_blur.intensity,
                samples: u32::from(motion_blur.quality),
            });
        }
    } else if has_motion_blur {
        entity.remove::<MotionBlur>();
    }
    let next_tonemapping = match primary.and_then(|profile| profile.tonemapping) {
        Some(PostProcessTonemapping::Aces) => Tonemapping::AcesFitted,
        Some(PostProcessTonemapping::Neutral) => Tonemapping::SomewhatBoringDisplayTransform,
        Some(PostProcessTonemapping::None) | None => Tonemapping::None,
    };
    if let Some(mut tonemapping) = tonemapping {
        if *tonemapping != next_tonemapping {
            *tonemapping = next_tonemapping;
        }
    } else {
        entity.insert(next_tonemapping);
    }
    runtime.applied = Some(signature);
}

const fn motion_blur_supported() -> bool {
    // Bevy 0.19's motion-blur shader contains a varying loop that the
    // self-contained Windows FXC path cannot compile. Other platforms retain
    // the authored effect; Windows keeps the remainder of the post stack.
    !cfg!(target_os = "windows")
}

fn authored_post_process_stack<'a>(
    catalog: &'a PresentationCatalog,
    scene_path: &str,
    daylight: f32,
) -> Vec<(&'a PostProcessProfileDef, f32)> {
    catalog
        .scene_post_process
        .get(scene_path)
        .into_iter()
        .flatten()
        .filter_map(|binding| {
            catalog
                .post_process_profiles
                .get(&binding.profile)
                .map(|profile| {
                    let weight = if binding.inverse_daylight {
                        binding.weight * (1.0 - daylight.clamp(0.0, 1.0))
                    } else {
                        binding.weight
                    };
                    (profile, weight.clamp(0.0, 1.0))
                })
        })
        .collect()
}

fn authored_color_grading(
    settings: &PlayerSettings,
    stack: &[(&PostProcessProfileDef, f32)],
) -> ColorGrading {
    let mut exposure = 0.0;
    let mut hue_degrees = 0.0;
    let mut saturation = 0.0;
    for (profile, weight) in stack {
        let Some(adjustments) = profile.color_adjustments else {
            continue;
        };
        exposure += (adjustments.post_exposure - exposure) * weight;
        hue_degrees += (adjustments.hue_shift_degrees - hue_degrees) * weight;
        saturation += (adjustments.saturation - saturation) * weight;
    }
    let mut grading = ColorGrading::default();
    grading.global.exposure = settings.video.brightness_ev + exposure;
    grading.global.hue = hue_degrees.to_radians();
    grading.global.post_saturation = (1.0 + saturation / 100.0).max(0.0);
    let gamma = (1.0 + settings.video.gamma * 0.1).clamp(0.5, 1.5);
    grading.shadows.gamma = gamma;
    grading.midtones.gamma = gamma;
    grading.highlights.gamma = gamma;
    grading
}

fn color_grading_for_state(
    settings: &PlayerSettings,
    stack: &[(&PostProcessProfileDef, f32)],
    state: GameState,
) -> ColorGrading {
    let mut grading = authored_color_grading(settings, stack);
    if state == GameState::MainMenu {
        // User testing established that the authored menu reaches its intended
        // luminance at the old -1.5 brightness setting. Make that the scene's
        // neutral baseline so setting 0 is now the correct exposure.
        grading.global.exposure += MAIN_MENU_BASELINE_EXPOSURE_EV;
    } else if state == GameState::InGame {
        // User testing established that the intended world luminance was the
        // old +0.5 setting. Treat it as the authored scene baseline so neutral
        // brightness now produces the approved image.
        grading.global.exposure += IN_GAME_BASELINE_EXPOSURE_EV;
        // Unity URP's ACES output retains slightly more chroma than Bevy's
        // fitted implementation for this low-poly palette. This narrow output
        // compensation restores the authored grass/building separation without
        // changing source material values or deterministic world content.
        grading.global.post_saturation *= IN_GAME_SATURATION_MULTIPLIER;
    }
    grading
}

fn authored_rgb_filter(stack: &[(&PostProcessProfileDef, f32)]) -> [f32; 4] {
    let mut filter = [1.0_f32; 4];
    for (profile, weight) in stack {
        let Some(adjustments) = profile.color_adjustments else {
            continue;
        };
        for (current, target) in filter.iter_mut().zip(adjustments.color_filter) {
            *current += (target - *current) * weight;
        }
    }
    filter
}

fn player_msaa(settings: &PlayerSettings) -> Msaa {
    // Bevy 0.19's SSAO render node requires multisampling to be disabled.
    // Retain the authored MSAA preference and apply it whenever SSAO is off.
    if settings.video.ambient_occlusion {
        Msaa::Off
    } else {
        Msaa::from_samples(u32::from(settings.video.msaa_samples))
    }
}

fn setup_world_audio_sources(
    audio_sources: Option<ResMut<Assets<AudioSource>>>,
    mut world_audio: ResMut<WorldAudioRuntime>,
) {
    let Some(mut audio_sources) = audio_sources else {
        return;
    };
    let ambience_wav: Arc<[u8]> =
        procedural_ambience_wav(PROCEDURAL_AUDIO_SAMPLE_RATE, 24.0).into();
    let ambience = audio_sources.add(AudioSource {
        bytes: ambience_wav.clone(),
    });
    let seagull_call_wavs = (0_u8..3)
        .map(|variant| {
            Arc::<[u8]>::from(procedural_seagull_call_wav(
                variant,
                PROCEDURAL_AUDIO_SAMPLE_RATE,
            ))
        })
        .collect::<Vec<_>>();
    let seagull_calls = seagull_call_wavs
        .iter()
        .map(|wav| audio_sources.add(AudioSource { bytes: wav.clone() }))
        .collect();
    world_audio.ambience = Some(ambience);
    world_audio.ambience_wav = Some(ambience_wav);
    world_audio.seagull_calls = seagull_calls;
    world_audio.seagull_call_wavs = seagull_call_wavs;
}

fn drive_world_audio(
    mut commands: Commands,
    player_settings: Res<RuntimePlayerSettings>,
    simulation: Option<Res<SimulationRuntime>>,
    world_audio: Res<WorldAudioRuntime>,
    ambience_players: Query<Entity, With<AmbienceAudio>>,
    mut ambience_sinks: Query<&mut AudioSink, With<AmbienceAudio>>,
    #[cfg(target_os = "windows")] native_audio: Res<direct_broadcast::NativeGameAudioRouting>,
) {
    if simulation.is_none() {
        #[cfg(target_os = "windows")]
        native_audio.clear_looping();
        for entity in &ambience_players {
            commands.entity(entity).try_despawn();
        }
        return;
    }
    let gain = AMBIENCE_GAIN * player_settings.0.audio.master * player_settings.0.audio.ambience;
    #[cfg(target_os = "windows")]
    let local_gain = if native_audio.local_monitor_enabled() {
        gain
    } else {
        0.0
    };
    #[cfg(not(target_os = "windows"))]
    let local_gain = gain;
    #[cfg(target_os = "windows")]
    if let Some(wav) = world_audio.ambience_wav.as_deref() {
        native_audio.set_looping_pcm16_wav("world:ambience", wav, gain);
    }
    for mut sink in &mut ambience_sinks {
        sink.set_volume(Volume::Linear(local_gain));
    }
    if ambience_players.is_empty()
        && let Some(source) = world_audio.ambience.clone()
    {
        commands.spawn((
            Name::new("Procedural seasonal ambience"),
            WorldEntity,
            AmbienceAudio,
            AudioPlayer(source),
            PlaybackSettings::LOOP.with_volume(Volume::Linear(local_gain)),
        ));
    }
}

fn spawn_seagull(
    commands: &mut Commands,
    render: &RenderAssets,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    world_seed: u64,
) {
    let (start, end) = deterministic_seagull_leg(world_seed, 0);
    let critter_material = standalone_material_override(render, CRITTER_MATERIAL_ID);
    let mut flight = commands.spawn((
        Name::new("SeagulSpawner (Unity parity)"),
        WorldEntity,
        SeagullFlight {
            start,
            end,
            elapsed_seconds: 0.0,
            leg_serial: 0,
            call_elapsed_seconds: 0.0,
            call_wait_seconds: 0.0,
            call_serial: 0,
            world_seed,
        },
        Visibility::default(),
        seagull_flight_transform(start, end),
    ));
    flight.with_children(|parent| {
        let model_transform = seagull_model_transform();
        if let Some(asset_server) = asset_server
            && converted_asset_exists(asset_root, SEAGULL_MODEL_PATH)
        {
            let mut visual = parent.spawn((
                Name::new("Critter_Seagull_01"),
                WorldAssetRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset(SEAGULL_MODEL_PATH.to_owned())),
                ),
                model_transform,
            ));
            if let Some(critter_material) = &critter_material {
                visual.insert(critter_material.clone());
            }
        } else {
            parent.spawn((
                Name::new("Seagull fallback"),
                Mesh3d(render.cube.clone()),
                MeshMaterial3d(render.food.clone()),
                model_transform.with_scale(Vec3::new(3.0, 0.35, 1.3)),
            ));
        }
    });
}

fn standalone_material_override(
    render: &RenderAssets,
    material_id: &str,
) -> Option<MaterialOverrideSpec> {
    let material = StableId::new(material_id)
        .ok()
        .and_then(|id| render.presentation_materials.get(&id))?
        .clone();
    Some(MaterialOverrideSpec {
        fallback: Some(material),
        model_materials: BTreeMap::new(),
        renderer_materials: Vec::new(),
        suppress_self_shadows: false,
    })
}

#[allow(clippy::too_many_arguments)]
fn spawn_flag_smoke_castle(
    commands: &mut Commands,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: &AssetServer,
    asset_root: &Path,
    position: Vec3,
    cell_size: f32,
) {
    let archetype_id = StableId::new("archetype:building:castle").expect("static archetype ID");
    let Some(archetype) = content.archetypes.get(&archetype_id) else {
        return;
    };
    let Some(scene) = archetype.scenes.iter().find(|scene| scene.age == Some(2)) else {
        return;
    };
    if !converted_asset_exists(asset_root, &scene.asset_path) {
        return;
    }
    let mut castle = commands.spawn((
        Name::new("Flag material smoke castle"),
        WorldEntity,
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
        ),
        Transform::from_translation(position).with_scale(Vec3::splat(cell_size / 2.0)),
    ));
    if let Some(materials) = prefab_material_spec(archetype, scene, presentation, render) {
        castle.insert(materials);
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_godray_smoke_tower(
    commands: &mut Commands,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: &AssetServer,
    asset_root: &Path,
    position: Vec3,
    cell_size: f32,
) {
    let archetype_id = StableId::new("archetype:building:necrotower").expect("static archetype ID");
    let Some(archetype) = content.archetypes.get(&archetype_id) else {
        return;
    };
    let Some(scene) = archetype.scenes.iter().find(|scene| scene.age == Some(2)) else {
        return;
    };
    if !converted_asset_exists(asset_root, &scene.asset_path) {
        return;
    }
    let mut tower = commands.spawn((
        Name::new("Godray material smoke necromancer tower"),
        WorldEntity,
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
        ),
        Transform::from_translation(position).with_scale(Vec3::splat(cell_size / 2.0)),
    ));
    if let Some(materials) = prefab_material_spec(archetype, scene, presentation, render) {
        tower.insert(materials);
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_giraffe_smoke_pet(
    commands: &mut Commands,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: &AssetServer,
    asset_root: &Path,
    position: Vec3,
) {
    let Some(archetype) = content
        .archetypes
        .values()
        .find(|archetype| archetype.source_path.ends_with("Prefabs/Pets/Pet.prefab"))
    else {
        return;
    };
    let pet = StableId::new("pet:giraffe").expect("static pet ID");
    let Some((scene, model)) = pet_model(archetype, &pet) else {
        return;
    };
    if !converted_asset_exists(asset_root, &scene.asset_path) {
        return;
    }
    let mut giraffe = commands.spawn((
        Name::new("Giraffe material smoke pet"),
        WorldEntity,
        Transform::from_translation(position),
    ));
    if let Some(materials) = prefab_material_spec(archetype, scene, presentation, render) {
        giraffe.insert(materials);
    }
    giraffe.with_children(|root| {
        root.spawn((
            WorldAssetRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            Transform::from_translation(Vec3::from_array(model.local_position))
                .with_rotation(Quat::from_array(model.local_rotation).normalize())
                .with_scale(Vec3::from_array(model.local_scale)),
        ));
    });
}

fn drive_seagull_flight(
    mut commands: Commands,
    time: Res<Time>,
    settings: Res<RuntimePlayerSettings>,
    world_audio: Res<WorldAudioRuntime>,
    camera: Query<&GlobalTransform, With<TownCamera>>,
    mut flights: Query<(&mut SeagullFlight, &mut Transform)>,
    #[cfg(target_os = "windows")] native_audio: Res<direct_broadcast::NativeGameAudioRouting>,
) {
    let camera_position = camera.single().ok().map(GlobalTransform::translation);
    for (mut flight, mut transform) in &mut flights {
        let delta = time.delta_secs();
        flight.elapsed_seconds += delta;
        if flight.elapsed_seconds >= SEAGULL_FLIGHT_SECONDS {
            flight.leg_serial = flight.leg_serial.saturating_add(1);
            (flight.start, flight.end) =
                deterministic_seagull_leg(flight.world_seed, flight.leg_serial);
            flight.elapsed_seconds = 0.0;
        }
        let progress = (flight.elapsed_seconds / SEAGULL_FLIGHT_SECONDS).clamp(0.0, 1.0);
        let position = flight.start.lerp(flight.end, progress);
        *transform = seagull_flight_transform(position, flight.end);

        flight.call_elapsed_seconds += delta;
        if flight.call_elapsed_seconds <= flight.call_wait_seconds {
            continue;
        }
        let call_serial = flight.call_serial;
        flight.call_serial = flight.call_serial.saturating_add(1);
        flight.call_elapsed_seconds = 0.0;
        flight.call_wait_seconds = deterministic_seagull_call_wait(flight.world_seed, call_serial);
        if world_audio.seagull_calls.is_empty() {
            continue;
        }
        let variant = deterministic_seagull_call_variant(flight.world_seed, call_serial);
        let source = world_audio.seagull_calls[variant].clone();
        let distance = camera_position.map_or(0.0, |camera| camera.distance(position));
        let gain = SEAGULL_GAIN
            * settings.0.audio.master
            * settings.0.audio.ambience
            * unity_seagull_rolloff(distance);
        #[cfg(target_os = "windows")]
        if let Some(wav) = world_audio.seagull_call_wavs.get(variant) {
            native_audio.play_pcm16_wav(&format!("seagull:{variant}:{call_serial}"), wav, gain);
        }
        #[cfg(target_os = "windows")]
        if !native_audio.local_monitor_enabled() {
            continue;
        }
        commands.spawn((
            Name::new(format!("Seagull call {}", variant + 1)),
            WorldEntity,
            AudioPlayer(source),
            PlaybackSettings::DESPAWN
                .with_volume(Volume::Linear(gain))
                .with_spatial(true)
                .with_spatial_scale(SpatialScale::new(1.0 / SEAGULL_MAX_AUDIO_DISTANCE)),
            Transform::from_translation(position),
        ));
    }
}

fn seagull_flight_transform(position: Vec3, target: Vec3) -> Transform {
    let direction = (target - position).normalize_or_zero();
    let rotation = if direction == Vec3::ZERO {
        Quat::IDENTITY
    } else {
        Quat::from_rotation_arc(Vec3::Z, direction)
    };
    Transform::from_translation(position).with_rotation(rotation)
}

fn seagull_model_transform() -> Transform {
    // The converted flock's visible nose axis is local +X. Unity's prefab uses
    // +90 degrees around Y to align it with Transform.forward, but Unity and
    // glTF/Bevy use opposite handedness for this import turn. Copying the
    // numeric sign maps +X onto -Z and makes the flock fly backward.
    Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2))
        .with_scale(Vec3::splat(5.0))
}

fn deterministic_seagull_leg(seed: u64, serial: u64) -> (Vec3, Vec3) {
    const CENTRES: [Vec2; 4] = [
        Vec2::new(200.0, 0.0),
        Vec2::new(-200.0, 0.0),
        Vec2::new(0.0, -200.0),
        Vec2::new(0.0, 200.0),
    ];
    const AREAS: [Vec2; 4] = [
        Vec2::new(5.0, 400.0),
        Vec2::new(5.0, 400.0),
        Vec2::new(400.0, 5.0),
        Vec2::new(400.0, 5.0),
    ];
    let mut area =
        usize::try_from(seagull_hash(seed, serial, 0) % 4).expect("seagull area index fits usize");
    // The Unity component never updates `_currentArea`, so an initial area-zero
    // roll is always advanced to area one. Preserve that shipping behaviour.
    if area == 0 {
        area = 1;
    }
    let offset = Vec2::new(
        seagull_signed_unit(seed, serial, 1) * AREAS[area].x * 0.5,
        seagull_signed_unit(seed, serial, 2) * AREAS[area].y * 0.5,
    );
    let horizontal = CENTRES[area] + offset;
    let start = Vec3::new(horizontal.x, SEAGULL_HEIGHT, horizontal.y);
    let end = Vec3::new(-horizontal.x, SEAGULL_HEIGHT, -horizontal.y);
    (start, end)
}

fn deterministic_seagull_call_wait(seed: u64, serial: u64) -> f32 {
    1.0 + seagull_unit(seed, serial, 3) * 4.0
}

fn deterministic_seagull_call_variant(seed: u64, serial: u64) -> usize {
    usize::try_from(seagull_hash(seed, serial, 4) % 3).expect("seagull call variant fits usize")
}

fn seagull_signed_unit(seed: u64, serial: u64, salt: u64) -> f32 {
    seagull_unit(seed, serial, salt) * 2.0 - 1.0
}

#[allow(clippy::cast_precision_loss)]
fn seagull_unit(seed: u64, serial: u64, salt: u64) -> f32 {
    let value = seagull_hash(seed, serial, salt) >> 40;
    value as f32 / 16_777_215.0
}

fn seagull_hash(seed: u64, serial: u64, salt: u64) -> u64 {
    let mut mixed = seed
        .wrapping_add(serial.wrapping_mul(0x9e37_79b9_7f4a_7c15))
        .wrapping_add(salt.wrapping_mul(0xd1b5_4a32_d192_ed03));
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    mixed ^ (mixed >> 31)
}

fn unity_seagull_rolloff(distance: f32) -> f32 {
    const CURVE: [(f32, f32); 7] = [
        (0.0, 1.0),
        (0.05, 1.0),
        (0.1, 0.5),
        (0.2, 0.25),
        (0.4, 0.125),
        (0.766_247_6, 0.037_643_433),
        (1.0, 0.0),
    ];
    let normalized = (distance / SEAGULL_MAX_AUDIO_DISTANCE).clamp(0.0, 1.0);
    for segment in CURVE.windows(2) {
        let (start_time, start_value) = segment[0];
        let (end_time, end_value) = segment[1];
        if normalized <= end_time {
            let progress = ((normalized - start_time) / (end_time - start_time)).clamp(0.0, 1.0);
            return start_value + (end_value - start_value) * progress;
        }
    }
    0.0
}

#[allow(clippy::cast_precision_loss)]
fn procedural_ambience_wav(sample_rate: u32, seconds: f32) -> Vec<u8> {
    let duration = seconds.max(1.0 / sample_rate.max(1) as f32);
    synthesize_wav(sample_rate, seconds, move |time, _| {
        // Integer harmonics over the clip duration make the loop seamless. The
        // previous sample-by-sample white-noise layer was technically valid PCM,
        // but sounded like static on headphones and some resamplers.
        let phase = time / duration * std::f32::consts::TAU;
        let gust = (phase * 2.0).sin() * 0.55
            + (phase * 5.0 + 0.8).sin() * 0.28
            + (phase * 11.0 + 2.1).sin() * 0.14;
        let air = (phase * 23.0 + 1.4).sin() * 0.08
            + (phase * 41.0 + 0.3).sin() * 0.05
            + (phase * 67.0 + 2.7).sin() * 0.025;
        (gust * 0.085 + air * 0.04).clamp(-0.12, 0.12)
    })
}

fn procedural_seagull_call_wav(variant: u8, sample_rate: u32) -> Vec<u8> {
    let (duration, start_frequency, frequency_sweep) = match variant % 3 {
        0 => (0.62, 1_080.0, 420.0),
        1 => (0.78, 920.0, 610.0),
        _ => (0.55, 1_240.0, -260.0),
    };
    synthesize_wav(sample_rate, duration, move |time, _| {
        let phase = (time / duration).clamp(0.0, 1.0);
        let envelope = (phase * std::f32::consts::PI).sin().powf(0.65);
        let pulse = (phase * std::f32::consts::TAU * (2.0 + f32::from(variant))).sin() * 0.08;
        let frequency = start_frequency + frequency_sweep * phase + pulse * start_frequency;
        let carrier = (time * std::f32::consts::TAU * frequency).sin();
        let harmonic = (time * std::f32::consts::TAU * frequency * 1.97).sin() * 0.28;
        (carrier + harmonic) * envelope * 0.22
    })
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn synthesize_wav(
    sample_rate: u32,
    seconds: f32,
    mut sample_value: impl FnMut(f32, u32) -> f32,
) -> Vec<u8> {
    let sample_count = (Duration::from_secs_f32(seconds).as_secs_f64() * f64::from(sample_rate))
        .round()
        .clamp(1.0, f64::from(u32::MAX)) as u32;
    let data_bytes = sample_count.saturating_mul(2);
    let mut wav = Vec::with_capacity(usize::try_from(data_bytes).unwrap_or(0) + 44);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&data_bytes.saturating_add(36).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&sample_rate.saturating_mul(2).to_le_bytes());
    wav.extend_from_slice(&2_u16.to_le_bytes());
    wav.extend_from_slice(&16_u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_bytes.to_le_bytes());
    for sample in 0..sample_count {
        let time = sample as f32 / sample_rate.max(1) as f32;
        let value = sample_value(time, sample).clamp(-1.0, 1.0);
        let encoded = (value * f32::from(i16::MAX)).round() as i16;
        wav.extend_from_slice(&encoded.to_le_bytes());
    }
    wav
}

#[allow(clippy::cast_precision_loss)]
fn pseudo_noise(value: u32) -> f32 {
    let mixed = value.wrapping_mul(747_796_405).wrapping_add(2_891_336_453);
    let mixed = ((mixed >> ((mixed >> 28) + 4)) ^ mixed).wrapping_mul(277_803_737);
    let normalized = ((mixed >> 22) ^ mixed) as f32 / u32::MAX as f32;
    normalized * 2.0 - 1.0
}

use presentation::materials::*;

use presentation::menu::*;

use presentation::world::*;

use runtime::agents::*;

use presentation::environment::*;

use presentation::animation::*;

use presentation::rendering::*;

use runtime::control::*;

use runtime::commands::*;

use runtime::persistence::*;

fn cleanup_state_entities(mut commands: Commands, entities: Query<Entity, With<StateEntity>>) {
    for entity in &entities {
        commands.entity(entity).try_despawn();
    }
}
