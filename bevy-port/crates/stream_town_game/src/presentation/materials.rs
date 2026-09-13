use super::super::*;

pub(crate) fn standard_material(
    material: &MaterialDef,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> StandardMaterial {
    let primary_texture = primary_material_texture_entry(material, presentation);
    let base_color_texture = asset_server.and_then(|asset_server| {
        primary_texture.map(|(_, path)| asset_server.load(path.to_owned()))
    });
    let texture_transform = primary_texture
        .and_then(|(slot, _)| material.texture_transforms.get(slot))
        .copied()
        .unwrap_or_default();
    let alpha_mode = match material.alpha_mode {
        AuthoredAlphaMode::Opaque => AlphaMode::Opaque,
        AuthoredAlphaMode::Mask => AlphaMode::Mask(0.5),
        AuthoredAlphaMode::Blend => AlphaMode::Blend,
    };
    StandardMaterial {
        base_color: Color::srgba(
            material.base_color[0],
            material.base_color[1],
            material.base_color[2],
            material.base_color[3],
        ),
        base_color_texture,
        emissive: LinearRgba::new(
            material.emissive[0],
            material.emissive[1],
            material.emissive[2],
            material.emissive[3],
        ),
        metallic: material.metallic,
        perceptual_roughness: material.perceptual_roughness,
        alpha_mode,
        uv_transform: Affine2::from_scale_angle_translation(
            Vec2::from_array(texture_transform.scale),
            0.0,
            Vec2::from_array(texture_transform.offset),
        ),
        ..default()
    }
}

pub(crate) fn selection_outline_material(mut material: StandardMaterial) -> StandardMaterial {
    material.depth_bias = SELECTION_OUTLINE_DEPTH_BIAS;
    material
}

pub(crate) fn unity_shader_color(value: [f32; 4]) -> Vec4 {
    Color::srgba(value[0], value[1], value[2], value[3])
        .to_linear()
        .to_f32_array()
        .into()
}

pub(crate) fn character_material(
    material: &MaterialDef,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> CharacterMaterial {
    let mut base = standard_material(material, presentation, asset_server);
    base.base_color = Color::WHITE;
    CharacterMaterial {
        base,
        extension: CharacterMaterialExtension {
            parameters: CharacterMaterialUniform {
                albedo_color: Color::srgba(
                    material.base_color[0],
                    material.base_color[1],
                    material.base_color[2],
                    material.base_color[3],
                )
                .to_linear()
                .to_f32_array()
                .into(),
                shadow_controls: Vec4::new(CHARACTER_SHADOW_RECEIVER_NORMAL_OFFSET, 0.0, 0.0, 0.0),
            },
        },
    }
}

pub(crate) fn character_material_from_standard(material: StandardMaterial) -> CharacterMaterial {
    let albedo_color = material.base_color.to_linear().to_f32_array().into();
    CharacterMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            ..material
        },
        extension: CharacterMaterialExtension {
            parameters: CharacterMaterialUniform {
                albedo_color,
                shadow_controls: Vec4::new(CHARACTER_SHADOW_RECEIVER_NORMAL_OFFSET, 0.0, 0.0, 0.0),
            },
        },
    }
}

pub(crate) fn terrain_material(
    presentation: &PresentationCatalog,
    config: &GameConfig,
    asset_server: Option<&AssetServer>,
    traversal_wear_texture: Option<Handle<Image>>,
    path_surface_texture: Option<Handle<Image>>,
) -> TerrainMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == TERRAIN_MATERIAL_PATH);
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let vector = |name: &str, fallback: [f32; 4]| {
        authored
            .and_then(|material| material.custom_vectors.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let texture_uv = vector("_TextureUV", [0.5, 0.5, 0.0, 0.0]);
    // The green channel of Unity's shared noise texture contains a 2x2
    // checker. Its authored UV repeats once per logical cell, which made each
    // visible square half a cell wide. Divide by the logical cell size so each
    // quadrant now spans exactly one complete terrain cell.
    let checker_scale = config.world.cell_size.max(0.000_1).recip();
    let grid_transform = authored
        .and_then(|material| material.texture_transforms.get("_MainTexture"))
        .copied()
        .unwrap_or_default();
    let grid_texture = authored.and_then(|material| {
        asset_server.and_then(|asset_server| {
            material
                .textures
                .get("_MainTexture")
                .and_then(|id| presentation.textures.get(id))
                .map(|texture| asset_server.load(texture.asset_path.clone()))
        })
    });
    TerrainMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            // Unity Terrain.shader hard-codes Smoothness to zero. The generic
            // material catalog value belongs to the source material inspector,
            // not to this Shader Graph output.
            perceptual_roughness: 1.0,
            metallic: 0.0,
            reflectance: 0.0,
            ..default()
        },
        extension: TerrainMaterialExtension {
            parameters: TerrainMaterialUniform {
                sand_color_a: unity_shader_color(vector(
                    "_SandGridColor1",
                    [0.934, 0.773, 0.084, 1.0],
                )),
                sand_color_b: unity_shader_color(vector(
                    "_SandGridColor2",
                    [0.823, 0.681, 0.071, 1.0],
                )),
                grass_color_a: unity_shader_color(vector("_color1", [0.422, 0.498, 0.153, 1.0])),
                grass_color_b: unity_shader_color(vector("_color2", [0.406, 0.471, 0.141, 1.0])),
                season_tint: Vec4::new(1.0, 1.0, 1.0, scalar("_Tint", 0.0)),
                texture_uv_blend_tint: Vec4::new(
                    texture_uv[0] * checker_scale,
                    texture_uv[1] * checker_scale,
                    scalar("_BlendHeight", 1.0),
                    0.0,
                ),
                grid_scale_offset: Vec4::new(
                    grid_transform.scale[0],
                    grid_transform.scale[1],
                    grid_transform.offset[0],
                    grid_transform.offset[1],
                ),
                selection_center_extent: Vec4::ZERO,
                selection_color: Vec4::ZERO,
                traversal_grid: Vec4::new(
                    f32::from(config.world.width),
                    f32::from(config.world.height),
                    config.world.cell_size,
                    if traversal_wear_texture.is_some() {
                        1.0
                    } else {
                        0.0
                    },
                ),
                path_grid: Vec4::new(
                    f32::from(config.world.width.saturating_mul(NAVIGATION_SUBDIVISIONS)),
                    f32::from(config.world.height.saturating_mul(NAVIGATION_SUBDIVISIONS)),
                    config.world.cell_size / f32::from(NAVIGATION_SUBDIVISIONS),
                    1.0,
                ),
                // Alpha controls the maximum contribution over the authored
                // terrain. Keep paths legible without replacing the grass and
                // use a cooler, less red soil than the first wear pass.
                traversal_dirt_color: unity_shader_color(config.terrain.spring.traversal_tint),
                constructed_path_color: unity_shader_color(config.terrain.spring.path_tint),
            },
            grid_texture,
            traversal_wear_texture,
            path_surface_texture,
        },
    }
}

pub(crate) fn traversal_wear_image(width: u16, height: u16) -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width: u32::from(width.max(1)),
            height: u32::from(height.max(1)),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, u8::MAX],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::nearest();
    image
}

pub(crate) fn water_material(
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> WaterMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == WATER_MATERIAL_PATH);
    let vector = |name: &str, fallback: [f32; 4]| {
        authored
            .and_then(|material| material.custom_vectors.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let texture = |slot: &str| {
        authored.and_then(|material| {
            asset_server.and_then(|asset_server| {
                material
                    .textures
                    .get(slot)
                    .and_then(|id| presentation.textures.get(id))
                    .map(|texture| asset_server.load(texture.asset_path.clone()))
            })
        })
    };
    let wind = vector("_windDirection", [1.0, 0.0, 0.0, 0.0]);
    let main_transform = authored
        .and_then(|material| material.texture_transforms.get("_MainTexture"))
        .copied()
        .unwrap_or_default();
    let noise_transform = authored
        .and_then(|material| material.texture_transforms.get("_NoiseTexture"))
        .copied()
        .unwrap_or_default();
    let surface = unity_shader_color(vector("_SurfaceColor", [0.071, 0.867, 0.886, 1.0]));
    WaterMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            // Unity's stylized water shader does not use the physically intense
            // dielectric highlight produced by Bevy at its serialized 0.9
            // smoothness. Preserve the animated surface while suppressing the
            // false HDR mirror that washed the coastline white.
            perceptual_roughness: (1.0 - scalar("_WaterSmoothness", 0.9).clamp(0.0, 1.0)).max(0.72),
            reflectance: 0.0,
            // Gameplay water is visually opaque and must write depth. Leaving
            // it in the transparent pass allowed submerged seaweed/coral and
            // shore-overhanging resource meshes to sort over the surface.
            // The main-menu variant deliberately opts back into blending.
            alpha_mode: AlphaMode::Opaque,
            ..default()
        },
        extension: WaterMaterialExtension {
            parameters: WaterMaterialUniform {
                surface_color: surface,
                deep_color: unity_shader_color(vector("_DeepColor", [0.063, 0.361, 0.565, 1.0])),
                foam_color: unity_shader_color(vector("_FoamColor", [1.0; 4])),
                ice_color: unity_shader_color(vector("_IceColor", [0.8, 0.93, 1.0, 1.0])),
                wind_speed_noise_alpha: Vec4::new(
                    wind[0],
                    wind[1],
                    scalar("_Speed", 0.02),
                    scalar("_WaterNoiseMultiplyer", 0.03),
                ),
                scale_foam_ice: Vec4::new(
                    scalar("_textureSize", 5.0).max(0.01),
                    scalar("_EdgeFoamScale", 3.71),
                    scalar("_FoamAlpha", 0.4).clamp(0.0, 1.0),
                    scalar("_IceStrength", 0.0).clamp(0.0, 1.0),
                ),
                season_tint: water_color_tint(surface, [0.05, 0.29, 0.47, 0.62]),
                main_scale_offset: Vec4::new(
                    main_transform.scale[0],
                    main_transform.scale[1],
                    main_transform.offset[0],
                    main_transform.offset[1],
                ),
                noise_scale_offset: Vec4::new(
                    noise_transform.scale[0],
                    noise_transform.scale[1],
                    noise_transform.offset[0],
                    noise_transform.offset[1],
                ),
                depth_foam_controls: Vec4::new(
                    scalar("_Distance", 10.0).max(0.01),
                    scalar("_EdgePower", 0.8).max(0.01),
                    scalar("_FoamCuttoff", 7.81).max(0.01),
                    scalar("_FoamDepth", 0.94).max(0.01),
                ),
                opacity_controls: Vec4::new(0.86, 0.94, 0.0, 0.0),
            },
            main_texture: texture("_MainTexture"),
            noise_texture: texture("_NoiseTexture"),
        },
    }
}

pub(crate) fn main_menu_water_material(mut material: WaterMaterial) -> WaterMaterial {
    // The menu uses one flat plane, so z/w request a fixed mid-water depth
    // instead of converting broad animated noise into a second blue region.
    // Restore transparent blending at a lower alpha than gameplay water so the
    // shoreline and fish remain legible without the former grey wash.
    material.extension.parameters.opacity_controls = Vec4::new(0.68, 0.78, 1.0, 0.46);
    material.base.alpha_mode = AlphaMode::Blend;
    material
}

pub(crate) fn menu_sky_material() -> MenuSkyMaterial {
    MenuSkyMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            cull_mode: None,
            ..default()
        },
        extension: MenuSkyMaterialExtension {
            parameters: MenuSkyMaterialUniform {
                horizon_color: Vec4::new(0.16, 0.48, 0.82, 1.0),
                zenith_color: Vec4::new(0.015, 0.08, 0.30, 1.0),
                haze_color: Vec4::new(1.0, 0.48, 0.18, 1.0),
                sun_direction_strength: Vec4::new(-0.48, 0.32, -0.81, 0.46),
            },
        },
    }
}

pub(crate) fn building_material(
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> BuildingMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == BUILDING_MATERIAL_PATH);
    let vector = |name: &str, fallback: [f32; 4]| {
        authored
            .and_then(|material| material.custom_vectors.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let texture = authored.and_then(|material| {
        asset_server.and_then(|asset_server| {
            material
                .textures
                .get("_MainTexture")
                .and_then(|id| presentation.textures.get(id))
                .map(|texture| asset_server.load(texture.asset_path.clone()))
        })
    });
    let transform = authored
        .and_then(|material| material.texture_transforms.get("_MainTexture"))
        .copied()
        .unwrap_or_default();
    let roughness_metallic = vector("_RoughnessMetalicValues", [1.0, 0.5, 0.0, 0.0]);
    BuildingMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        },
        extension: BuildingMaterialExtension {
            parameters: BuildingMaterialUniform {
                detail_color: unity_shader_color(vector(
                    "_DetailColor",
                    [0.521_568_7, 0.521_568_7, 0.521_568_7, 1.0],
                )),
                emissive_color: unity_shader_color(vector(
                    "_EmissiveColour",
                    [0.521_568_7, 0.521_568_7, 0.521_568_7, 1.0],
                )),
                ambient_occlusion: Vec4::from_array(vector(
                    "_AmbientOcclusion",
                    [0.2, 0.5, 0.0, 0.0],
                )),
                surface_controls: Vec4::new(
                    scalar("_DetailStrength", 0.0),
                    scalar("_GlassEmission", 2.5),
                    scalar("_EmissionStrength", 2.5),
                    roughness_metallic[0],
                ),
                snow_damage: Vec4::new(
                    scalar("_SnowPower", 0.0),
                    scalar("_SnowNoiseLevels", 0.0),
                    scalar("_DestructionValue", 2.0),
                    roughness_metallic[1],
                ),
                main_scale_offset: Vec4::new(
                    transform.scale[0],
                    transform.scale[1],
                    transform.offset[0],
                    transform.offset[1],
                ),
                tint_color_strength: Vec4::new(1.0, 1.0, 1.0, 0.0),
                // A zero cycle keeps menu and preview materials in
                // full daylight. Runtime instances replace this with a phase
                // aligned cycle that the shader can advance without dirtying
                // every material asset on every dusk/dawn frame.
                time_cycle: Vec4::ZERO,
            },
            main_texture: texture,
        },
    }
}

pub(crate) fn cloud_material(
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> CloudMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == CLOUD_MATERIAL_PATH);
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let texture = authored.and_then(|material| {
        asset_server.and_then(|asset_server| {
            material
                .textures
                .get("_Texture0")
                .and_then(|id| presentation.textures.get(id))
                .map(|texture| asset_server.load(texture.asset_path.clone()))
        })
    });
    let transform = authored
        .and_then(|material| material.texture_transforms.get("_Texture0"))
        .copied()
        .unwrap_or_default();
    CloudMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.5,
            alpha_mode: AlphaMode::Blend,
            ..default()
        },
        extension: CloudMaterialExtension {
            parameters: CloudMaterialUniform {
                noise_controls: Vec4::new(
                    scalar("_Cloud1", 0.01),
                    scalar("_Cloud2", 0.001),
                    scalar("_ColourSS", 0.0),
                    scalar("_ColourCutoff", 0.13),
                ),
                surface_transform: Vec4::new(
                    scalar("_CloudTint", 1.36),
                    scalar("_CloudSurface", 0.58),
                    transform.scale[0],
                    transform.scale[1],
                ),
                filter_controls: Vec4::new(CLOUD_NOISE_ALPHA_MEAN, 0.75, 2.0, 0.0),
            },
            noise_texture: texture,
        },
    }
}

pub(crate) fn godray_material(presentation: &PresentationCatalog) -> GodrayMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == GODRAY_MATERIAL_PATH);
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    GodrayMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            ..default()
        },
        extension: GodrayMaterialExtension {
            parameters: GodrayMaterialUniform {
                emission_alpha: Vec4::new(
                    scalar("_EmissionStrength", 0.06),
                    scalar("_AlphaStrength", 1.64),
                    0.0,
                    0.0,
                ),
            },
        },
    }
}

pub(crate) fn giraffe_material(
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> GiraffeMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == GIRAFFE_MATERIAL_PATH);
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let main_texture = authored.and_then(|material| {
        asset_server.and_then(|asset_server| {
            material
                .textures
                .get("_TextureSample0")
                .and_then(|id| presentation.textures.get(id))
                .map(|texture| asset_server.load(texture.asset_path.clone()))
        })
    });
    let transform = authored
        .and_then(|material| material.texture_transforms.get("_TextureSample0"))
        .copied()
        .unwrap_or_default();
    GiraffeMaterial {
        base: StandardMaterial {
            base_color_texture: main_texture.clone(),
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            metallic: 0.0,
            ..default()
        },
        extension: GiraffeMaterialExtension {
            parameters: GiraffeMaterialUniform {
                animation_controls: Vec4::new(scalar("_NeckHeight", 8.15), 0.9, 2.0, 0.9),
                mask_controls: Vec4::new(0.07, 1.2, 0.8, 2.1),
                // These values are constants in Amplify's generated shader; similarly named
                // serialized material properties are disconnected graph-editor remnants.
                rotation_controls: Vec4::new(0.11, 4.1, 1.8, 0.0),
                main_scale_offset: Vec4::new(
                    transform.scale[0],
                    transform.scale[1],
                    transform.offset[0],
                    transform.offset[1],
                ),
            },
            main_texture,
        },
    }
}

pub(crate) fn bounds_material(
    presentation: &PresentationCatalog,
    color_override: Option<[f32; 3]>,
) -> BoundsMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == BOUNDS_MATERIAL_PATH);
    let authored_color = authored
        .and_then(|material| material.custom_vectors.get("_boundsVisColor"))
        .copied()
        .unwrap_or([0.498_615_2, 1.0, 0.202_830_2, 1.0]);
    let color = color_override.unwrap_or([authored_color[0], authored_color[1], authored_color[2]]);
    let alpha = authored
        .and_then(|material| material.custom_properties.get("_Alpha"))
        .copied()
        .unwrap_or(0.5);
    BoundsMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            alpha_mode: AlphaMode::Blend,
            metallic: 0.0,
            perceptual_roughness: 0.5,
            ..default()
        },
        extension: BoundsMaterialExtension {
            parameters: BoundsMaterialUniform {
                color_alpha: unity_shader_color([color[0], color[1], color[2], alpha]),
            },
        },
    }
}

pub(crate) fn tree_material(
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> TreeMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == TREE_MATERIAL_PATH);
    let vector = |name: &str, fallback: [f32; 4]| {
        authored
            .and_then(|material| material.custom_vectors.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let texture = |slot: &str| {
        authored.and_then(|material| {
            asset_server.and_then(|asset_server| {
                material
                    .textures
                    .get(slot)
                    .and_then(|id| presentation.textures.get(id))
                    .map(|texture| asset_server.load(texture.asset_path.clone()))
            })
        })
    };
    let transform = authored
        .and_then(|material| material.texture_transforms.get("_MainTexture"))
        .copied()
        .unwrap_or_default();
    let direction = vector("_windDirection", [1.0, 0.0, 0.0, 0.0]);
    let smoothness = vector("_WindDetailSmoothness", [0.0, 1.0, 0.0, 0.0]);
    TreeMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            // Env_Tree.glb carries `doubleSided: true`. Preserve that contract
            // when replacing the imported material with this typed material:
            // disabling culling exposes both sides of each leaf card, while
            // `double_sided` makes StandardMaterial correct the back-face
            // normal for lighting instead of shading it inside-out.
            double_sided: true,
            cull_mode: None,
            ..default()
        },
        extension: TreeMaterialExtension {
            parameters: TreeMaterialUniform {
                wind_direction_smoothness: Vec4::new(
                    direction[0],
                    direction[1],
                    smoothness[0],
                    smoothness[1],
                ),
                wind_controls: Vec4::new(
                    scalar("_Sync", 0.7),
                    scalar("_windStrength", 0.79),
                    scalar("_WindDetailStrength", 0.01),
                    scalar("_textureSize", 1.0),
                ),
                season_controls: tree_season_controls(Season::Spring),
                main_scale_offset: Vec4::new(
                    transform.scale[0],
                    transform.scale[1],
                    transform.offset[0],
                    transform.offset[1],
                ),
            },
            main_texture: texture("_MainTexture"),
            noise_texture: texture("_NoiseTexture"),
        },
    }
}

pub(crate) fn menu_tree_material() -> StandardMaterial {
    StandardMaterial {
        // Compensate for the menu's authored -1.5 EV exposure so the stable
        // green silhouette remains readable rather than collapsing to black.
        base_color: Color::srgb(0.42, 0.72, 0.16),
        perceptual_roughness: 1.0,
        // The converted tree GLB deliberately marks its material double-sided.
        // Leaf cards must remain visible from either side, and Bevy uses this
        // flag to flip the back-face normal before ordinary PBR lighting.
        double_sided: true,
        cull_mode: None,
        ..default()
    }
}

pub(crate) fn grass_material(
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> GrassMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == GRASS_MATERIAL_PATH);
    let vector = |name: &str, fallback: [f32; 4]| {
        authored
            .and_then(|material| material.custom_vectors.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let texture = |slot: &str| {
        authored.and_then(|material| {
            asset_server.and_then(|asset_server| {
                material
                    .textures
                    .get(slot)
                    .and_then(|id| presentation.textures.get(id))
                    .map(|texture| asset_server.load(texture.asset_path.clone()))
            })
        })
    };
    let transform = authored
        .and_then(|material| material.texture_transforms.get("_MainTexture"))
        .copied()
        .unwrap_or_default();
    let color = |name: &str, fallback: [f32; 4]| unity_shader_color(vector(name, fallback));
    let direction = vector("_windDirection", [1.0, 0.0, 0.0, 0.0]);
    let smoothness = vector("_WindTextureSmoothStep", [-0.28, 0.66, 0.0, 0.0]);
    let world_strength = vector("_WorldPositionStrength", [0.5, 0.5, 0.0, 0.0]);
    GrassMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        },
        extension: GrassMaterialExtension {
            parameters: GrassMaterialUniform {
                grid_color_1: color("_GridColor1", [0.416_476_4, 0.498_039_22, 0.152_941_2, 1.0]),
                grid_color_2: color(
                    "_GridColor2",
                    [0.419_094_32, 0.470_588_24, 0.141_176_5, 1.0],
                ),
                wind_color: color(
                    "_WindColor",
                    [0.447_058_83, 0.533_333_36, 0.164_705_89, 1.0],
                ),
                wind_direction_smoothness: Vec4::new(
                    direction[0],
                    direction[1],
                    smoothness[0],
                    smoothness[1],
                ),
                wind_controls: Vec4::new(
                    scalar("_CloudCrawlSpeed", 0.1),
                    scalar("_WindScale", 0.0),
                    scalar("_WindStrength", 1.1),
                    scalar("_textureSize", 20.0),
                ),
                surface_controls: Vec4::new(
                    scalar("_ColourBlend", 1.55),
                    scalar("_Spring", 0.0),
                    scalar("_Tint", 0.0),
                    scalar("_VertexSmoothStepMax", 1.0),
                ),
                world_strength_transform: Vec4::new(
                    world_strength[0],
                    world_strength[1],
                    0.001,
                    0.001,
                ),
                main_scale_offset: Vec4::new(
                    transform.scale[0],
                    transform.scale[1],
                    transform.offset[0],
                    transform.offset[1],
                ),
            },
            main_texture: texture("_MainTexture"),
            noise_texture: texture("_NoiseTexture"),
        },
    }
}

pub(crate) fn critter_material(
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> CritterMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == CRITTER_MATERIAL_PATH);
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let main_texture = authored.and_then(|material| {
        asset_server.and_then(|asset_server| {
            material
                .textures
                .get("_MainTexture")
                .and_then(|id| presentation.textures.get(id))
                .map(|texture| asset_server.load(texture.asset_path.clone()))
        })
    });
    let transform = authored
        .and_then(|material| material.texture_transforms.get("_MainTexture"))
        .copied()
        .unwrap_or_default();
    CritterMaterial {
        base: StandardMaterial {
            base_color_texture: main_texture.clone(),
            base_color: Color::WHITE,
            perceptual_roughness: 0.5,
            ..default()
        },
        extension: CritterMaterialExtension {
            parameters: CritterMaterialUniform {
                animation_controls: Vec4::new(
                    scalar("_Speed", 5.5),
                    scalar("_Sync", 3.58),
                    scalar("_Stretch", 0.3),
                    0.0,
                ),
                main_scale_offset: Vec4::new(
                    transform.scale[0],
                    transform.scale[1],
                    transform.offset[0],
                    transform.offset[1],
                ),
            },
            main_texture,
        },
    }
}

pub(crate) fn flag_material(
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> FlagMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == FLAG_MATERIAL_PATH);
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let vector = |name: &str, fallback: [f32; 4]| {
        authored
            .and_then(|material| material.custom_vectors.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let noise_texture = authored.and_then(|material| {
        asset_server.and_then(|asset_server| {
            material
                .textures
                .get("_Misc_Noises_Texture_01")
                .and_then(|id| presentation.textures.get(id))
                .map(|texture| asset_server.load(texture.asset_path.clone()))
        })
    });
    let transform = authored
        .and_then(|material| material.texture_transforms.get("_Misc_Noises_Texture_01"))
        .copied()
        .unwrap_or_default();
    FlagMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.3,
            ..default()
        },
        extension: FlagMaterialExtension {
            parameters: FlagMaterialUniform {
                colour_1: unity_shader_color(vector("_Colour1", [1.0, 0.835_294_1, 0.0, 0.0])),
                colour_2: unity_shader_color(vector(
                    "_Colour2",
                    [1.0, 0.023_529_41, 0.023_529_41, 0.0],
                )),
                controls: Vec4::new(
                    scalar("_NoiseTextureScale", 0.49),
                    scalar("_RotationStrength", 0.27),
                    scalar("_strength", 0.14),
                    scalar("_Metaledge", 0.7),
                ),
                noise_scale_offset: Vec4::new(
                    transform.scale[0],
                    transform.scale[1],
                    transform.offset[0],
                    transform.offset[1],
                ),
            },
            noise_texture,
        },
    }
}

pub(crate) fn primary_material_texture_entry<'a>(
    material: &'a MaterialDef,
    presentation: &'a PresentationCatalog,
) -> Option<(&'a str, &'a str)> {
    const PRIORITY: [&str; 8] = [
        "_BaseMap",
        "_BaseColorMap",
        "_MainTexture",
        "_MainTex",
        "_Texture0",
        "_characterTexture",
        "_BaseColorRGBOutlineWidthA",
        "_BaseColorRGBSmoothnessA",
    ];
    PRIORITY
        .iter()
        .filter_map(|slot| material.textures.get_key_value(*slot))
        .chain(material.textures.iter())
        .find_map(|(slot, id)| {
            presentation
                .textures
                .get(id)
                .map(|texture| (slot.as_str(), texture.asset_path.as_str()))
        })
}

pub(crate) fn presentation_texture_handle(
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
    source_path: &str,
) -> Option<Handle<Image>> {
    let asset_path = presentation
        .textures
        .values()
        .find(|texture| texture.source_path == source_path)?
        .asset_path
        .clone();
    Some(asset_server?.load(asset_path))
}
