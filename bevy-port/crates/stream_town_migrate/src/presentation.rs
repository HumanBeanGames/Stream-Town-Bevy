use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use stream_town_domain::{
    AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef,
    AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef,
    AnimationMotionDef, AnimationObjectReference, AnimationParameterDef, AnimationParameterKind,
    AnimationPropertyCurve, AnimationQuatKeyframe, AnimationStateDef, AnimationStateMachineDef,
    AnimationTangent, AnimationTransformTrack, AnimationTransitionDef, AnimationVec3Keyframe,
    AvatarMaskDef, ChimneySmokeDef, FireworksVfxDef, FishSchoolVfxDef, HealingBurstVfxDef,
    HealingChannelVfxDef, MaterialAlphaMode, MaterialDef, PostProcessBloomDef,
    PostProcessColorAdjustmentsDef, PostProcessMotionBlurDef, PostProcessProfileDef,
    PostProcessTonemapping, PostProcessVignetteDef, PrefabChimneyEmitterBinding,
    PrefabPresentationBinding, PresentationCatalog, RainingFishVfxDef, RendererMaterialBinding,
    RoleActionAudioDef, SceneFireworksBinding, SceneFishSchoolBinding, ScenePostProcessBinding,
    StableId, TextureDef, TextureTransform, VfxAlphaKeyframe, VfxColorKeyframe, VfxGradientDef,
};

const SHIPPING_SCENES: [&str; 4] = [
    "Assets/Scenes/LOADER_INITIAL.unity",
    "Assets/Scenes/Menu/Main_Menu_02.unity",
    "Assets/Scenes/Worlds/World_Town.unity",
    "Assets/Scenes/Menu/Credits.unity",
];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PresentationConversionReport {
    pub schema_version: u32,
    pub fonts: usize,
    pub font_bytes: u64,
    pub textures: usize,
    pub texture_bytes: u64,
    pub materials: usize,
    pub custom_shader_materials: usize,
    pub material_vector_properties: usize,
    pub material_texture_transforms: usize,
    pub material_prefab_bindings: usize,
    pub material_slots: usize,
    pub model_material_bindings: usize,
    pub renderer_material_bindings: usize,
    pub renderer_material_slots: usize,
    pub clips: usize,
    pub converted_clips: usize,
    pub property_curve_clips: usize,
    pub property_curves: usize,
    pub property_curve_keys: usize,
    pub event_clips: usize,
    pub animation_events: usize,
    pub missing_clip_sources: usize,
    pub controllers: usize,
    pub controller_states: usize,
    pub controller_transitions: usize,
    pub controller_state_machines: usize,
    pub controller_layers: usize,
    pub avatar_masks: usize,
    pub avatar_mask_transforms: usize,
    pub disabled_avatar_mask_transforms: usize,
    pub inferred_parameters: usize,
    pub prefab_bindings: usize,
    pub native_animation_bindings: usize,
    pub post_process_profiles: usize,
    pub scene_post_process_bindings: usize,
    pub fireworks_effects: usize,
    pub scene_fireworks_bindings: usize,
    pub chimney_smoke_effects: usize,
    pub prefab_chimney_emitters: usize,
    pub raining_fish_effects: usize,
    pub fish_school_effects: usize,
    pub scene_fish_school_bindings: usize,
    pub role_action_audio_roles: usize,
    pub role_action_audio_variants: usize,
    pub healing_channel_effects: usize,
    pub healing_burst_effects: usize,
    pub outputs: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityExport {
    schema_version: u32,
    assets: Vec<UnityAsset>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityAsset {
    guid: String,
    path: String,
    kind: String,
    name: String,
    #[serde(default)]
    importer_fields: Vec<UnityField>,
    #[serde(default)]
    serialized_fields: Vec<UnityField>,
    #[serde(default)]
    dependencies: Vec<UnityReference>,
    #[serde(default)]
    game_object: Option<UnityGameObject>,
    #[serde(default)]
    scene: Option<UnityScene>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityField {
    path: String,
    value: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityReference {
    #[serde(default)]
    guid: Option<String>,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityGameObject {
    #[serde(default)]
    components: Vec<UnityComponent>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityScene {
    #[serde(default)]
    roots: Vec<UnityGameObject>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityComponent {
    #[serde(default)]
    hierarchy_path: String,
    #[serde(rename = "Type", default)]
    type_name: Option<String>,
    #[serde(default)]
    fields: Vec<UnityField>,
}

#[derive(Debug)]
struct YamlDocument {
    class_id: u32,
    file_id: i64,
    lines: Vec<String>,
}

#[derive(Clone, Debug)]
struct ParsedMotion {
    guid: String,
    local_id: i64,
    threshold: Option<f32>,
}

#[derive(Clone, Debug)]
struct ParsedBlendTree {
    parameter: Option<String>,
    motions: Vec<ParsedMotion>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TransformCurveKind {
    Rotation,
    EulerDegrees,
    Translation,
    Scale,
}

#[derive(Clone, Copy, Debug)]
struct RestTransform {
    translation: [f32; 3],
    rotation: [f32; 4],
    scale: [f32; 3],
}

type PostProcessConversion = (
    BTreeMap<StableId, PostProcessProfileDef>,
    BTreeMap<String, Vec<ScenePostProcessBinding>>,
);

type FireworksConversion = (
    BTreeMap<StableId, FireworksVfxDef>,
    BTreeMap<String, Vec<SceneFireworksBinding>>,
);

type ChimneySmokeConversion = (
    BTreeMap<StableId, ChimneySmokeDef>,
    BTreeMap<String, Vec<PrefabChimneyEmitterBinding>>,
);
type FishSchoolConversion = (
    BTreeMap<StableId, FishSchoolVfxDef>,
    BTreeMap<String, Vec<SceneFishSchoolBinding>>,
);

pub fn convert(
    export_path: &Path,
    unity_root: &Path,
    out_dir: &Path,
) -> Result<PresentationConversionReport> {
    let bytes = fs::read(export_path)
        .with_context(|| format!("failed to read Unity export {}", export_path.display()))?;
    let export: UnityExport = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse Unity export {}", export_path.display()))?;
    if export.schema_version != 1 {
        bail!(
            "unsupported Unity presentation export schema {}",
            export.schema_version
        );
    }
    let root = unity_root
        .canonicalize()
        .with_context(|| format!("Unity root {} does not exist", unity_root.display()))?;
    if !root.join("Assets").is_dir() {
        bail!("{} is not a Unity project root", root.display());
    }

    let assets_by_guid: BTreeMap<_, _> = export
        .assets
        .iter()
        .map(|asset| (asset.guid.as_str(), asset))
        .collect();
    let assets_by_path: BTreeMap<_, _> = export
        .assets
        .iter()
        .map(|asset| (asset.path.as_str(), asset))
        .collect();

    let (fonts, font_bytes) = convert_shipping_fonts(&root, out_dir)?;
    let (textures, texture_bytes) = convert_textures(&export, &root, out_dir)?;
    let materials = convert_materials(&export, &assets_by_path)?;
    let prefab_materials = convert_prefab_materials(&export, &assets_by_path, &materials);
    let model_materials = convert_model_materials(&export, &root, out_dir, &materials);
    let prefab_renderer_materials =
        convert_prefab_renderer_materials(&export, &assets_by_path, &materials, &model_materials);
    let (post_process_profiles, scene_post_process) = convert_post_process(&export, &root)?;
    let (fireworks_effects, scene_fireworks) = convert_fireworks(&export, &root)?;
    let (chimney_smoke_effects, prefab_chimney_emitters) = convert_chimney_smoke(&export, &root)?;
    let raining_fish_effects = convert_raining_fish(&export, &root)?;
    let (fish_school_effects, scene_fish_schools) = convert_fish_schools(&export, &root)?;
    let role_action_audio = convert_role_action_audio(&export, &root)?;
    let (healing_channel_effects, healing_burst_effects) = convert_healing_vfx(&export, &root)?;
    let mut clips = convert_clips(&export, &root)?;
    let embedded_clips = convert_embedded_model_clips(&export, &root, &mut clips)?;
    let avatar_masks = convert_avatar_masks(&export, &root)?;
    let controllers =
        convert_controllers(&export, &root, &assets_by_guid, &embedded_clips, &mut clips)?;
    let prefab_bindings = convert_prefab_bindings(&export, &assets_by_path, &controllers);
    assign_clip_rigs_and_reference_poses(
        &export,
        &assets_by_path,
        &controllers,
        &prefab_bindings,
        &mut clips,
    );
    let catalog = PresentationCatalog {
        schema_version: 21,
        textures,
        materials,
        clips,
        controllers,
        avatar_masks,
        prefab_bindings,
        prefab_materials,
        model_materials,
        prefab_renderer_materials,
        post_process_profiles,
        scene_post_process,
        fireworks_effects,
        scene_fireworks,
        chimney_smoke_effects,
        prefab_chimney_emitters,
        raining_fish_effects,
        fish_school_effects,
        scene_fish_schools,
        role_action_audio,
        healing_channel_effects,
        healing_burst_effects,
    };
    catalog
        .validate()
        .context("converted presentation catalog is invalid")?;

    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create output directory {}", out_dir.display()))?;
    let catalog_path = out_dir.join("presentation.ron");
    let report_path = out_dir.join("presentation-report.ron");
    let report = PresentationConversionReport {
        schema_version: 21,
        fonts,
        font_bytes,
        textures: catalog.textures.len(),
        texture_bytes,
        materials: catalog.materials.len(),
        custom_shader_materials: catalog
            .materials
            .values()
            .filter(|material| {
                !material.custom_properties.is_empty() || !material.custom_vectors.is_empty()
            })
            .count(),
        material_vector_properties: catalog
            .materials
            .values()
            .map(|material| material.custom_vectors.len())
            .sum(),
        material_texture_transforms: catalog
            .materials
            .values()
            .map(|material| material.texture_transforms.len())
            .sum(),
        material_prefab_bindings: catalog.prefab_materials.len(),
        material_slots: catalog.prefab_materials.values().map(Vec::len).sum(),
        model_material_bindings: catalog.model_materials.values().map(BTreeMap::len).sum(),
        renderer_material_bindings: catalog
            .prefab_renderer_materials
            .values()
            .map(Vec::len)
            .sum(),
        renderer_material_slots: catalog
            .prefab_renderer_materials
            .values()
            .flat_map(|renderers| renderers.iter())
            .map(|renderer| renderer.materials.len())
            .sum(),
        clips: catalog.clips.len(),
        converted_clips: catalog
            .clips
            .values()
            .filter(|clip| clip.converted_asset_path.is_some() || !clip.transform_tracks.is_empty())
            .count(),
        property_curve_clips: catalog
            .clips
            .values()
            .filter(|clip| !clip.property_curves.is_empty())
            .count(),
        property_curves: catalog
            .clips
            .values()
            .map(|clip| clip.property_curves.len())
            .sum(),
        property_curve_keys: catalog
            .clips
            .values()
            .flat_map(|clip| &clip.property_curves)
            .map(|curve| curve.keys.len())
            .sum(),
        event_clips: catalog
            .clips
            .values()
            .filter(|clip| !clip.events.is_empty())
            .count(),
        animation_events: catalog.clips.values().map(|clip| clip.events.len()).sum(),
        missing_clip_sources: catalog
            .clips
            .values()
            .filter(|clip| clip.source_path.is_empty())
            .count(),
        controllers: catalog.controllers.len(),
        controller_states: catalog
            .controllers
            .values()
            .map(|controller| controller.states.len())
            .sum(),
        controller_transitions: catalog
            .controllers
            .values()
            .map(|controller| controller.transitions.len())
            .sum(),
        controller_state_machines: catalog
            .controllers
            .values()
            .map(|controller| controller.state_machines.len())
            .sum(),
        controller_layers: catalog
            .controllers
            .values()
            .map(|controller| controller.layers.len())
            .sum(),
        avatar_masks: catalog.avatar_masks.len(),
        avatar_mask_transforms: catalog
            .avatar_masks
            .values()
            .map(|mask| mask.transform_weights.len())
            .sum(),
        disabled_avatar_mask_transforms: catalog
            .avatar_masks
            .values()
            .flat_map(|mask| mask.transform_weights.values())
            .filter(|weight| weight.abs() < f32::EPSILON)
            .count(),
        inferred_parameters: catalog
            .controllers
            .values()
            .flat_map(|controller| &controller.parameters)
            .filter(|parameter| parameter.inferred)
            .count(),
        prefab_bindings: catalog.prefab_bindings.len(),
        native_animation_bindings: catalog
            .prefab_bindings
            .values()
            .filter(|binding| binding.gltf_animation_index.is_some())
            .count(),
        post_process_profiles: catalog.post_process_profiles.len(),
        scene_post_process_bindings: catalog.scene_post_process.values().map(Vec::len).sum(),
        fireworks_effects: catalog.fireworks_effects.len(),
        scene_fireworks_bindings: catalog.scene_fireworks.values().map(Vec::len).sum(),
        chimney_smoke_effects: catalog.chimney_smoke_effects.len(),
        prefab_chimney_emitters: catalog.prefab_chimney_emitters.values().map(Vec::len).sum(),
        raining_fish_effects: catalog.raining_fish_effects.len(),
        fish_school_effects: catalog.fish_school_effects.len(),
        scene_fish_school_bindings: catalog.scene_fish_schools.values().map(Vec::len).sum(),
        role_action_audio_roles: catalog.role_action_audio.len(),
        role_action_audio_variants: catalog
            .role_action_audio
            .values()
            .map(|audio| audio.clip_guids.len())
            .sum(),
        healing_channel_effects: catalog.healing_channel_effects.len(),
        healing_burst_effects: catalog.healing_burst_effects.len(),
        outputs: vec![
            normalized_path(&catalog_path),
            normalized_path(&report_path),
        ],
    };
    write_ron_atomic(&catalog_path, &catalog)?;
    write_ron_atomic(&report_path, &report)?;

    let reloaded: PresentationCatalog = ron::from_str(
        &fs::read_to_string(&catalog_path)
            .with_context(|| format!("failed to reload {}", catalog_path.display()))?,
    )?;
    if reloaded != catalog {
        bail!("generated presentation catalog changed during its RON round trip");
    }
    reloaded.validate()?;
    let reloaded_report: PresentationConversionReport = ron::from_str(
        &fs::read_to_string(&report_path)
            .with_context(|| format!("failed to reload {}", report_path.display()))?,
    )?;
    if reloaded_report != report {
        bail!("generated presentation report changed during its RON round trip");
    }
    Ok(report)
}

fn convert_post_process(export: &UnityExport, unity_root: &Path) -> Result<PostProcessConversion> {
    let assets_by_guid: BTreeMap<_, _> = export
        .assets
        .iter()
        .map(|asset| (asset.guid.as_str(), asset))
        .collect();
    let mut referenced_profiles = BTreeSet::new();
    let mut scene_post_process = BTreeMap::new();
    for scene_path in SHIPPING_SCENES {
        let Some(scene_asset) = export.assets.iter().find(|asset| asset.path == scene_path) else {
            continue;
        };
        let mut bindings = Vec::new();
        for component in scene_asset
            .scene
            .iter()
            .flat_map(|scene| &scene.roots)
            .flat_map(|root| &root.components)
            .filter(|component| {
                component
                    .type_name
                    .as_deref()
                    .is_some_and(|name| name.starts_with("UnityEngine.Rendering.Volume,"))
            })
        {
            if !field_bool(&component.fields, "m_IsGlobal").unwrap_or(false) {
                continue;
            }
            let Some(reference) = field_value(&component.fields, "sharedProfile") else {
                continue;
            };
            let Some(guid) = reference_field(reference, "Guid") else {
                continue;
            };
            let profile = post_process_profile_id(guid)?;
            referenced_profiles.insert(guid.to_owned());
            bindings.push(ScenePostProcessBinding {
                hierarchy_path: component.hierarchy_path.clone(),
                profile,
                weight: field_f32(&component.fields, "weight").unwrap_or(1.0),
                inverse_daylight: scene_path == "Assets/Scenes/Worlds/World_Town.unity"
                    && component
                        .hierarchy_path
                        .to_ascii_lowercase()
                        .contains("night"),
            });
        }
        if !bindings.is_empty() {
            scene_post_process.insert(scene_path.to_owned(), bindings);
        }
    }

    let mut profiles = BTreeMap::new();
    for guid in referenced_profiles {
        let asset = assets_by_guid
            .get(guid.as_str())
            .with_context(|| format!("reachable post-process profile {guid} is missing"))?;
        let source = unity_root.join(&asset.path);
        let contents = fs::read_to_string(&source)
            .with_context(|| format!("failed to read post-process profile {}", source.display()))?;
        let documents = parse_yaml_documents(&contents)?;
        let component = |name: &str| {
            documents.iter().find(|document| {
                document.class_id == 114
                    && scalar(&document.lines, "m_Name:").is_some_and(|value| value == name)
                    && scalar_bool(&document.lines, "active:").unwrap_or(true)
            })
        };
        let bloom = component("Bloom").map(|document| PostProcessBloomDef {
            intensity: volume_parameter_f32(&document.lines, "intensity").unwrap_or(0.0),
            threshold: volume_parameter_f32(&document.lines, "threshold").unwrap_or(1.0),
            scatter: volume_parameter_f32(&document.lines, "scatter").unwrap_or(0.7),
        });
        let vignette = component("Vignette").map(|document| PostProcessVignetteDef {
            color: volume_parameter(&document.lines, "color")
                .map_or([0.0, 0.0, 0.0, 1.0], |value| {
                    inline_color(value, [0.0, 0.0, 0.0, 1.0])
                }),
            center: volume_parameter(&document.lines, "center")
                .map_or([0.5, 0.5], |value| inline_vec2(value, [0.5, 0.5])),
            intensity: volume_parameter_f32(&document.lines, "intensity").unwrap_or(0.0),
            smoothness: volume_parameter_f32(&document.lines, "smoothness").unwrap_or(0.2),
            rounded: volume_parameter_bool(&document.lines, "rounded").unwrap_or(false),
        });
        let motion_blur = component("MotionBlur").map(|document| PostProcessMotionBlurDef {
            intensity: volume_parameter_f32(&document.lines, "intensity").unwrap_or(0.0),
            quality: volume_parameter(&document.lines, "quality")
                .and_then(|value| value.parse().ok())
                .unwrap_or(1),
        });
        let tonemapping = component("Tonemapping").map(|document| {
            match volume_parameter(&document.lines, "mode")
                .and_then(|value| value.parse::<u8>().ok())
                .unwrap_or_default()
            {
                1 => PostProcessTonemapping::Neutral,
                2 => PostProcessTonemapping::Aces,
                _ => PostProcessTonemapping::None,
            }
        });
        let color_adjustments =
            component("ColorAdjustments").map(|document| PostProcessColorAdjustmentsDef {
                post_exposure: volume_parameter_f32(&document.lines, "postExposure").unwrap_or(0.0),
                color_filter: volume_parameter(&document.lines, "colorFilter")
                    .map_or([1.0; 4], |value| inline_color(value, [1.0; 4])),
                hue_shift_degrees: volume_parameter_f32(&document.lines, "hueShift").unwrap_or(0.0),
                saturation: volume_parameter_f32(&document.lines, "saturation").unwrap_or(0.0),
            });
        profiles.insert(
            post_process_profile_id(&guid)?,
            PostProcessProfileDef {
                display_name: asset.name.clone(),
                source_guid: guid,
                source_path: asset.path.clone(),
                bloom,
                vignette,
                motion_blur,
                tonemapping,
                color_adjustments,
            },
        );
    }
    Ok((profiles, scene_post_process))
}

fn convert_fireworks(export: &UnityExport, unity_root: &Path) -> Result<FireworksConversion> {
    let mut effects = BTreeMap::new();
    let mut scene_bindings = BTreeMap::new();
    let Some(asset) = export
        .assets
        .iter()
        .find(|asset| asset.path == "Assets/VFX/vfx_fireworks.vfx")
    else {
        return Ok((effects, scene_bindings));
    };
    let source = unity_root.join(&asset.path);
    let contents = fs::read_to_string(&source)
        .with_context(|| format!("failed to read fireworks graph {}", source.display()))?;
    let documents = parse_yaml_documents(&contents)?;
    let effect_id = fireworks_effect_id(&asset.guid)?;
    let launch_rates = vfx_named_scalar_values(&documents, "Rate");
    let rocket_capacity = documents
        .iter()
        .find_map(|document| scalar(&document.lines, "capacity:")?.parse::<u32>().ok())
        .context("fireworks graph has no rocket capacity")?;
    let rocket_lifetime = vfx_attribute_range(&documents, "lifetime", rocket_capacity)?;
    let rocket_velocity = vfx_attribute_vector_range(&documents, "velocity", rocket_capacity)?;
    let burst_lifetime = vfx_attribute_range(&documents, "lifetime", 12_800)?;
    let spark_lifetime = vfx_attribute_range(&documents, "lifetime", 40_000)?;
    let burst_count = vfx_named_vec2(&documents, "Count")?;
    let burst_delay = vfx_named_vec2(&documents, "Delay")?;
    let colors = vfx_gradient_colors(&contents, "name: FireworkColour")?;
    let sparks_speed = vfx_parameter_scalar(&contents, "name: SparksSpeed")?;
    let effect = FireworksVfxDef {
        display_name: asset.name.clone(),
        source_guid: asset.guid.clone(),
        source_path: asset.path.clone(),
        sparks_speed,
        launch_rate_per_second: *launch_rates
            .first()
            .context("fireworks graph has no launch Rate")?,
        rocket_capacity: u16::try_from(rocket_capacity)
            .context("fireworks rocket capacity is outside u16 range")?,
        rocket_lifetime_seconds: rocket_lifetime,
        rocket_velocity_min: rocket_velocity.0,
        rocket_velocity_max: rocket_velocity.1,
        burst_lifetime_seconds: burst_lifetime,
        spark_lifetime_seconds: spark_lifetime,
        burst_particle_rate: f32_to_u16(
            launch_rates
                .get(1)
                .copied()
                .context("fireworks graph has no burst Rate")?,
            "fireworks burst Rate",
        )?,
        burst_count: [
            f32_to_u16(burst_count[0], "fireworks burst count minimum")?,
            f32_to_u16(burst_count[1], "fireworks burst count maximum")?,
        ],
        burst_delay_seconds: burst_delay,
        colors,
    };
    effects.insert(effect_id.clone(), effect);

    if let Some(scene_asset) = export
        .assets
        .iter()
        .find(|asset| asset.path == "Assets/Scenes/Menu/Credits.unity")
    {
        let bindings = scene_asset
            .scene
            .iter()
            .flat_map(|scene| &scene.roots)
            .flat_map(|root| &root.components)
            .filter(|component| {
                component.hierarchy_path.starts_with("VFX_FireWorks")
                    && component
                        .type_name
                        .as_deref()
                        .is_some_and(|name| name.starts_with("UnityEngine.Transform,"))
            })
            .filter_map(|component| {
                Some(SceneFireworksBinding {
                    hierarchy_path: component.hierarchy_path.clone(),
                    effect: effect_id.clone(),
                    local_position: field_array(&component.fields, "localPosition")?,
                })
            })
            .collect::<Vec<_>>();
        if !bindings.is_empty() {
            scene_bindings.insert(scene_asset.path.clone(), bindings);
        }
    }
    Ok((effects, scene_bindings))
}

fn convert_chimney_smoke(
    export: &UnityExport,
    unity_root: &Path,
) -> Result<ChimneySmokeConversion> {
    const CHIMNEY_PATH: &str = "Assets/Prefabs/VFX/Environment/VFX_Chimney_Smoke.prefab";
    let mut effects = BTreeMap::new();
    let mut prefab_bindings = BTreeMap::new();
    let Some(asset) = export
        .assets
        .iter()
        .find(|asset| asset.path == CHIMNEY_PATH)
    else {
        return Ok((effects, prefab_bindings));
    };
    let source = unity_root.join(CHIMNEY_PATH);
    let contents = fs::read_to_string(&source).with_context(|| {
        format!(
            "failed to read chimney particle prefab {}",
            source.display()
        )
    })?;
    let documents = parse_yaml_documents(&contents)?;
    let particle = documents
        .iter()
        .find(|document| document.class_id == 198)
        .context("chimney prefab has no ParticleSystem")?;
    let initial = yaml_section(&particle.lines, "InitialModule:")?;
    let lifetime = yaml_section(initial, "startLifetime:")?;
    let speed = yaml_section(initial, "startSpeed:")?;
    let color = yaml_section(initial, "startColor:")?;
    let size = yaml_section(initial, "startSize:")?;
    let shape = yaml_section(&particle.lines, "ShapeModule:")?;
    let radius = yaml_section(shape, "radius:")?;
    let emission = yaml_section(&particle.lines, "EmissionModule:")?;
    let rate = yaml_section(emission, "rateOverTime:")?;
    let size_module = yaml_section(&particle.lines, "SizeModule:")?;
    let size_curve = yaml_section(size_module, "curve:")?;
    let size_max_curve = yaml_section(size_curve, "maxCurve:")?;
    let color_module = yaml_section(&particle.lines, "ColorModule:")?;
    let gradient = yaml_section(color_module, "gradient:")?;
    let max_gradient = yaml_section(gradient, "maxGradient:")?;
    let size_values = yaml_keyframe_values(size_max_curve);
    let start_alpha = inline_color(
        scalar(max_gradient, "key0:").context("chimney color curve has no first key")?,
        [1.0; 4],
    )[3];
    let end_alpha = inline_color(
        scalar(max_gradient, "key1:").context("chimney color curve has no last key")?,
        [1.0; 4],
    )[3];
    let effect_id = particle_effect_id(&asset.guid)?;
    effects.insert(
        effect_id.clone(),
        ChimneySmokeDef {
            display_name: asset.name.clone(),
            source_guid: asset.guid.clone(),
            source_path: asset.path.clone(),
            duration_seconds: required_scalar_f32(&particle.lines, "lengthInSec:", "duration")?,
            emission_rate_per_second: required_scalar_f32(rate, "scalar:", "emission rate")?,
            lifetime_seconds: required_scalar_f32(lifetime, "scalar:", "lifetime")?,
            start_speed: required_scalar_f32(speed, "scalar:", "start speed")?,
            start_size: required_scalar_f32(size, "scalar:", "start size")?,
            start_color_min: inline_color(
                scalar(color, "minColor:").context("chimney particle has no minimum color")?,
                [1.0; 4],
            ),
            start_color_max: inline_color(
                scalar(color, "maxColor:").context("chimney particle has no maximum color")?,
                [1.0; 4],
            ),
            cone_radius: required_scalar_f32(radius, "value:", "cone radius")?,
            cone_angle_degrees: required_scalar_f32(shape, "angle:", "cone angle")?,
            size_over_lifetime: [
                *size_values.first().context("chimney size curve is empty")?,
                *size_values.last().context("chimney size curve is empty")?,
            ],
            alpha_over_lifetime: [start_alpha, end_alpha],
            max_particles: scalar(initial, "maxNumParticles:")
                .and_then(|value| value.parse().ok())
                .context("chimney particle has no maximum count")?,
            world_space: scalar(&particle.lines, "moveWithTransform:") == Some("1"),
        },
    );

    for prefab in export.assets.iter().filter(|candidate| {
        candidate.path.starts_with("Assets/Prefabs/Buildings/")
            && candidate
                .dependencies
                .iter()
                .any(|dependency| dependency.guid.as_deref() == Some(asset.guid.as_str()))
    }) {
        let mut bindings = prefab
            .game_object
            .iter()
            .flat_map(|game_object| &game_object.components)
            .filter(|component| {
                component
                    .type_name
                    .as_deref()
                    .is_some_and(|name| name.starts_with("UnityEngine.Transform,"))
                    && component
                        .hierarchy_path
                        .split('/')
                        .next_back()
                        .is_some_and(|name| name.starts_with("VFX_Chimney_Smoke"))
            })
            .map(|component| {
                Ok(PrefabChimneyEmitterBinding {
                    hierarchy_path: component.hierarchy_path.clone(),
                    effect: effect_id.clone(),
                    age: hierarchy_age(&component.hierarchy_path)?,
                    local_position: field_array(&component.fields, "localPosition").with_context(
                        || format!("{} has no local position", component.hierarchy_path),
                    )?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        bindings.sort_by(|left, right| left.hierarchy_path.cmp(&right.hierarchy_path));
        if !bindings.is_empty() {
            prefab_bindings.insert(prefab.guid.clone(), bindings);
        }
    }
    Ok((effects, prefab_bindings))
}

fn convert_raining_fish(
    export: &UnityExport,
    unity_root: &Path,
) -> Result<BTreeMap<StableId, RainingFishVfxDef>> {
    const EFFECT_PATH: &str = "Assets/Prefabs/VFX/Environment/VFX_RainingFish.prefab";
    let mut effects = BTreeMap::new();
    let Some(asset) = export.assets.iter().find(|asset| asset.path == EFFECT_PATH) else {
        return Ok(effects);
    };
    let source = unity_root.join(EFFECT_PATH);
    let contents = fs::read_to_string(&source)
        .with_context(|| format!("failed to read raining-fish prefab {}", source.display()))?;
    let documents = parse_yaml_documents(&contents)?;
    let particle = documents
        .iter()
        .find(|document| document.class_id == 198)
        .context("raining-fish prefab has no ParticleSystem")?;
    let transform = documents
        .iter()
        .find(|document| document.class_id == 4)
        .context("raining-fish prefab has no Transform")?;
    let renderer = documents
        .iter()
        .find(|document| document.class_id == 199)
        .context("raining-fish prefab has no ParticleSystemRenderer")?;
    let initial = yaml_section(&particle.lines, "InitialModule:")?;
    let lifetime = yaml_section(initial, "startLifetime:")?;
    let size = yaml_section(initial, "startSize:")?;
    let gravity = yaml_section(initial, "gravityModifier:")?;
    let shape = yaml_section(&particle.lines, "ShapeModule:")?;
    let emission = yaml_section(&particle.lines, "EmissionModule:")?;
    let rate = yaml_section(emission, "rateOverTime:")?;
    let size_module = yaml_section(&particle.lines, "SizeModule:")?;
    let size_curve = yaml_section(size_module, "curve:")?;
    let size_max_curve = yaml_section(size_curve, "maxCurve:")?;
    let noise = yaml_section(&particle.lines, "NoiseModule:")?;
    let noise_strength = yaml_section(noise, "strength:")?;
    let noise_strength_y = yaml_section(noise, "strengthY:")?;
    let noise_strength_z = yaml_section(noise, "strengthZ:")?;
    let noise_scroll = yaml_section(noise, "scrollSpeed:")?;
    let collision = yaml_section(&particle.lines, "CollisionModule:")?;
    let bounce = yaml_section(collision, "m_Bounce:")?;
    let lifetime_loss = yaml_section(collision, "m_EnergyLossOnCollision:")?;
    let material_guid = renderer
        .lines
        .iter()
        .take_while(|line| !line.trim_start().starts_with("m_Mesh:"))
        .find_map(|line| reference_guid(line))
        .context("raining-fish renderer has no material GUID")?;
    let model_guid = renderer
        .lines
        .iter()
        .find(|line| line.trim_start().starts_with("m_Mesh:"))
        .and_then(|line| reference_guid(line))
        .context("raining-fish renderer has no mesh GUID")?;
    let model_source = export
        .assets
        .iter()
        .find(|candidate| candidate.guid == model_guid)
        .map(|candidate| candidate.path.clone())
        .with_context(|| format!("raining-fish mesh GUID {model_guid} is missing from export"))?;
    let effect_id = particle_effect_id(&asset.guid)?;
    effects.insert(
        effect_id,
        RainingFishVfxDef {
            display_name: asset.name.clone(),
            source_guid: asset.guid.clone(),
            source_path: asset.path.clone(),
            model_asset_path: glb_asset_path(&model_source),
            model_source,
            material: material_id(material_guid)?,
            duration_seconds: required_particle_scalar_f32(
                &particle.lines,
                "lengthInSec:",
                "duration",
            )?,
            emission_rate_per_second: required_particle_scalar_f32(
                rate,
                "scalar:",
                "emission rate",
            )?,
            lifetime_seconds: required_particle_scalar_f32(lifetime, "scalar:", "lifetime")?,
            start_size: [
                required_particle_scalar_f32(size, "minScalar:", "minimum start size")?,
                required_particle_scalar_f32(size, "scalar:", "maximum start size")?,
            ],
            gravity: required_particle_scalar_f32(gravity, "scalar:", "gravity")?,
            max_particles: scalar(initial, "maxNumParticles:")
                .and_then(|value| value.parse().ok())
                .context("raining-fish particle has no maximum count")?,
            emitter_position: inline_vec3(
                scalar(&transform.lines, "m_LocalPosition:")
                    .context("raining-fish transform has no local position")?,
                [0.0; 3],
            ),
            shape_scale: inline_vec3(
                scalar(shape, "m_Scale:").context("raining-fish shape has no scale")?,
                [1.0; 3],
            ),
            shape_rotation_degrees: inline_vec3(
                scalar(shape, "m_Rotation:").context("raining-fish shape has no rotation")?,
                [0.0; 3],
            ),
            size_over_lifetime: yaml_float_keyframes(size_max_curve)?,
            noise_strength: [
                required_particle_scalar_f32(noise_strength, "scalar:", "noise strength X")?,
                required_particle_scalar_f32(noise_strength_y, "scalar:", "noise strength Y")?,
                required_particle_scalar_f32(noise_strength_z, "scalar:", "noise strength Z")?,
            ],
            noise_frequency: required_particle_scalar_f32(noise, "frequency:", "noise frequency")?,
            noise_scroll_speed: required_particle_scalar_f32(
                noise_scroll,
                "scalar:",
                "noise scroll speed",
            )?,
            collision_bounce: required_particle_scalar_f32(bounce, "scalar:", "collision bounce")?,
            collision_lifetime_loss: required_particle_scalar_f32(
                lifetime_loss,
                "scalar:",
                "collision lifetime loss",
            )?,
            world_space: scalar(&particle.lines, "moveWithTransform:") == Some("1"),
            prewarm: scalar(&particle.lines, "prewarm:") == Some("1"),
        },
    );
    Ok(effects)
}

fn convert_fish_schools(export: &UnityExport, unity_root: &Path) -> Result<FishSchoolConversion> {
    const EFFECT_PATH: &str = "Assets/Prefabs/VFX/Environment/Fish.prefab";
    let mut effects = BTreeMap::new();
    let mut scene_bindings = BTreeMap::new();
    let Some(asset) = export.assets.iter().find(|asset| asset.path == EFFECT_PATH) else {
        return Ok((effects, scene_bindings));
    };
    let source = unity_root.join(EFFECT_PATH);
    let contents = fs::read_to_string(&source)
        .with_context(|| format!("failed to read fish-school prefab {}", source.display()))?;
    let documents = parse_yaml_documents(&contents)?;
    let particle = documents
        .iter()
        .find(|document| document.class_id == 198)
        .context("fish-school prefab has no ParticleSystem")?;
    let particle_transform = documents
        .iter()
        .find(|document| {
            document.class_id == 4
                && scalar(&document.lines, "m_Father:")
                    .is_some_and(|father| father != "{fileID: 0}")
        })
        .context("fish-school prefab has no particle Transform")?;
    let renderer = documents
        .iter()
        .find(|document| document.class_id == 199)
        .context("fish-school prefab has no ParticleSystemRenderer")?;
    let initial = yaml_section(&particle.lines, "InitialModule:")?;
    let lifetime = yaml_section(initial, "startLifetime:")?;
    let speed = yaml_section(initial, "startSpeed:")?;
    let size = yaml_section(initial, "startSize:")?;
    let shape = yaml_section(&particle.lines, "ShapeModule:")?;
    let emission = yaml_section(&particle.lines, "EmissionModule:")?;
    let rate = yaml_section(emission, "rateOverTime:")?;
    let noise = yaml_section(&particle.lines, "NoiseModule:")?;
    let strength_x = yaml_section(noise, "strength:")?;
    let strength_y = yaml_section(noise, "strengthY:")?;
    let strength_z = yaml_section(noise, "strengthZ:")?;
    let scroll_speed = yaml_section(noise, "scrollSpeed:")?;
    let position_amount = yaml_section(noise, "positionAmount:")?;
    let material_guid = renderer
        .lines
        .iter()
        .take_while(|line| !line.trim_start().starts_with("m_Mesh:"))
        .find_map(|line| reference_guid(line))
        .context("fish-school renderer has no material GUID")?;
    let model_guid = renderer
        .lines
        .iter()
        .find(|line| line.trim_start().starts_with("m_Mesh:"))
        .and_then(|line| reference_guid(line))
        .context("fish-school renderer has no mesh GUID")?;
    let model_source = export
        .assets
        .iter()
        .find(|candidate| candidate.guid == model_guid)
        .map(|candidate| candidate.path.clone())
        .with_context(|| format!("fish-school mesh GUID {model_guid} is missing from export"))?;
    let effect_id = particle_effect_id(&asset.guid)?;
    let effect = FishSchoolVfxDef {
        display_name: asset.name.clone(),
        source_guid: asset.guid.clone(),
        source_path: asset.path.clone(),
        model_asset_path: glb_asset_path(&model_source),
        model_source,
        material: material_id(material_guid)?,
        duration_seconds: required_particle_scalar_f32(
            &particle.lines,
            "lengthInSec:",
            "duration",
        )?,
        emission_rate_per_second: required_particle_scalar_f32(rate, "scalar:", "emission rate")?,
        lifetime_seconds: required_particle_scalar_f32(lifetime, "scalar:", "lifetime")?,
        start_speed: required_particle_scalar_f32(speed, "scalar:", "start speed")?,
        start_size: [
            required_particle_scalar_f32(size, "minScalar:", "minimum start size")?,
            required_particle_scalar_f32(size, "scalar:", "maximum start size")?,
        ],
        max_particles: scalar(initial, "maxNumParticles:")
            .and_then(|value| value.parse().ok())
            .context("fish-school particle has no maximum count")?,
        particle_local_position: inline_vec3(
            scalar(&particle_transform.lines, "m_LocalPosition:")
                .context("fish-school transform has no local position")?,
            [0.0; 3],
        ),
        shape_scale: inline_vec3(
            scalar(shape, "m_Scale:").context("fish-school shape has no scale")?,
            [1.0; 3],
        ),
        shape_rotation_degrees: inline_vec3(
            scalar(shape, "m_Rotation:").context("fish-school shape has no rotation")?,
            [0.0; 3],
        ),
        noise_strength: [
            required_particle_scalar_f32(strength_x, "scalar:", "noise strength X")?,
            required_particle_scalar_f32(strength_y, "scalar:", "noise strength Y")?,
            required_particle_scalar_f32(strength_z, "scalar:", "noise strength Z")?,
        ],
        noise_frequency: required_particle_scalar_f32(noise, "frequency:", "noise frequency")?,
        noise_scroll_speed: required_particle_scalar_f32(
            scroll_speed,
            "scalar:",
            "noise scroll speed",
        )?,
        noise_position_amount: required_particle_scalar_f32(
            position_amount,
            "scalar:",
            "noise position amount",
        )?,
        noise_octaves: scalar(noise, "octaves:")
            .and_then(|value| value.parse().ok())
            .context("fish-school noise has no octave count")?,
        noise_octave_multiplier: required_particle_scalar_f32(
            noise,
            "octaveMultiplier:",
            "noise octave multiplier",
        )?,
        noise_octave_scale: required_particle_scalar_f32(
            noise,
            "octaveScale:",
            "noise octave scale",
        )?,
        align_to_velocity: scalar(&renderer.lines, "m_RenderAlignment:") == Some("4"),
        world_space: scalar(&particle.lines, "moveWithTransform:") == Some("0"),
        prewarm: scalar(&particle.lines, "prewarm:") == Some("1"),
    };
    effects.insert(effect_id.clone(), effect.clone());

    for scene_path in [
        "Assets/Scenes/Menu/Main_Menu_02.unity",
        "Assets/Scenes/Worlds/World_Town.unity",
    ] {
        let Some(scene_asset) = export.assets.iter().find(|asset| asset.path == scene_path) else {
            continue;
        };
        let scene_contents = fs::read_to_string(unity_root.join(scene_path))?;
        let scene_documents = parse_yaml_documents(&scene_contents)?;
        let mut bindings = Vec::new();
        for document in scene_documents.iter().filter(|document| {
            document.class_id == 1001
                && document.lines.iter().any(|line| {
                    line.trim_start().starts_with("m_SourcePrefab:")
                        && reference_guid(line) == Some(asset.guid.as_str())
                })
                && prefab_instance_is_effectively_active(document, &scene_documents)
        }) {
            let value = |property: &str| prefab_modification_value(&document.lines, property);
            let position = [
                value("m_LocalPosition.x")
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(0.0),
                value("m_LocalPosition.y")
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(0.0),
                value("m_LocalPosition.z")
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(0.0),
            ];
            let mut noise_strength = effect.noise_strength;
            for (index, property) in [
                "NoiseModule.strength.scalar",
                "NoiseModule.strengthY.scalar",
                "NoiseModule.strengthZ.scalar",
            ]
            .into_iter()
            .enumerate()
            {
                if let Some(override_value) = value(property).and_then(|value| value.parse().ok()) {
                    noise_strength[index] = override_value;
                }
            }
            bindings.push(SceneFishSchoolBinding {
                hierarchy_path: value("m_Name").unwrap_or("Fish").to_owned(),
                effect: effect_id.clone(),
                local_position: position,
                max_particles: value("InitialModule.maxNumParticles")
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(effect.max_particles),
                emission_rate_per_second: value("EmissionModule.rateOverTime.scalar")
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(effect.emission_rate_per_second),
                noise_strength,
            });
        }
        bindings.sort_by(|left, right| left.hierarchy_path.cmp(&right.hierarchy_path));
        if !bindings.is_empty() {
            scene_bindings.insert(scene_asset.path.clone(), bindings);
        }
    }
    Ok((effects, scene_bindings))
}

fn prefab_modification_value<'a>(lines: &'a [String], property: &str) -> Option<&'a str> {
    lines
        .iter()
        .enumerate()
        .find(|(_, line)| line.trim() == format!("propertyPath: {property}"))
        .and_then(|(index, _)| {
            lines
                .iter()
                .skip(index + 1)
                .take(2)
                .find_map(|line| line.trim().strip_prefix("value: "))
        })
}

fn prefab_instance_is_effectively_active(
    instance: &YamlDocument,
    documents: &[YamlDocument],
) -> bool {
    if prefab_modification_value(&instance.lines, "m_IsActive") == Some("0") {
        return false;
    }
    let Some(parent) = scalar(&instance.lines, "m_TransformParent:").and_then(inline_file_id)
    else {
        return true;
    };
    transform_hierarchy_is_active(parent, documents, &mut BTreeSet::new())
}

fn transform_hierarchy_is_active(
    transform_id: i64,
    documents: &[YamlDocument],
    visited: &mut BTreeSet<i64>,
) -> bool {
    if transform_id == 0 || !visited.insert(transform_id) {
        return true;
    }
    let Some(transform) = documents
        .iter()
        .find(|document| document.class_id == 4 && document.file_id == transform_id)
    else {
        return true;
    };
    if let Some(game_object_id) = scalar(&transform.lines, "m_GameObject:").and_then(inline_file_id)
        && documents
            .iter()
            .find(|document| document.class_id == 1 && document.file_id == game_object_id)
            .is_some_and(|game_object| scalar(&game_object.lines, "m_IsActive:") == Some("0"))
    {
        return false;
    }
    if let Some(prefab_id) = scalar(&transform.lines, "m_PrefabInstance:").and_then(inline_file_id)
        && prefab_id != 0
        && let Some(prefab) = documents
            .iter()
            .find(|document| document.class_id == 1001 && document.file_id == prefab_id)
    {
        if prefab_modification_value(&prefab.lines, "m_IsActive") == Some("0") {
            return false;
        }
        return scalar(&prefab.lines, "m_TransformParent:")
            .and_then(inline_file_id)
            .is_none_or(|parent| transform_hierarchy_is_active(parent, documents, visited));
    }
    scalar(&transform.lines, "m_Father:")
        .and_then(inline_file_id)
        .is_none_or(|parent| transform_hierarchy_is_active(parent, documents, visited))
}

fn convert_role_action_audio(
    export: &UnityExport,
    unity_root: &Path,
) -> Result<BTreeMap<StableId, RoleActionAudioDef>> {
    const PREFIX: &str = "Assets/Resources/ScriptableObjects/Roles/RoleData_";
    let mut roles = BTreeMap::new();
    for asset in export.assets.iter().filter(|asset| {
        asset.path.starts_with(PREFIX)
            && Path::new(&asset.path)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("asset"))
    }) {
        let contents = fs::read_to_string(unity_root.join(&asset.path))?;
        let Some(action_clips) = contents
            .lines()
            .position(|line| line.trim() == "ActionClips:")
        else {
            continue;
        };
        let clip_guids = contents
            .lines()
            .skip(action_clips + 1)
            .take_while(|line| line.starts_with("  - "))
            .filter_map(reference_guid)
            .map(str::to_owned)
            .collect::<Vec<_>>();
        if clip_guids.is_empty() {
            continue;
        }
        let role_name = asset
            .path
            .strip_prefix(PREFIX)
            .and_then(|name| name.strip_suffix(".asset"))
            .context("role action-audio asset has an invalid path")?;
        let role = StableId::new(format!("role:{}", role_name.to_ascii_lowercase()))?;
        roles.insert(
            role,
            RoleActionAudioDef {
                display_name: role_name.to_owned(),
                source_guid: asset.guid.clone(),
                source_path: asset.path.clone(),
                clip_guids,
            },
        );
    }
    Ok(roles)
}

fn convert_healing_vfx(
    export: &UnityExport,
    unity_root: &Path,
) -> Result<(
    BTreeMap<StableId, HealingChannelVfxDef>,
    BTreeMap<StableId, HealingBurstVfxDef>,
)> {
    const CHANNEL_PREFAB: &str = "Assets/Prefabs/VFX/Player/VFX_Healing_Channeling.prefab";
    const CHANNEL_GRAPH: &str = "Assets/VFX/vfx_channeling.vfx";
    const BURST_PREFAB: &str = "Assets/Prefabs/VFX/Player/VFX_healing.prefab";
    const BURST_GRAPH: &str = "Assets/VFX/vfx_healed.vfx";
    const PLUS_MODEL: &str = "Assets/Models/VFX/VFX_Plus.fbx";
    const DISC_TEXTURE: &str = "Assets/Sprites/VFX/Particle_02.png";

    let mut channels = BTreeMap::new();
    let mut bursts = BTreeMap::new();
    let find = |path: &str| export.assets.iter().find(|asset| asset.path == path);

    if let (Some(prefab), Some(graph)) = (find(CHANNEL_PREFAB), find(CHANNEL_GRAPH)) {
        let prefab_contents = fs::read_to_string(unity_root.join(CHANNEL_PREFAB))?;
        let graph_contents = fs::read_to_string(unity_root.join(CHANNEL_GRAPH))?;
        let documents = parse_yaml_documents(&graph_contents)?;
        let particle_capacity = vfx_capacity(&documents, 32)?;
        let particle_lifetime_seconds = vfx_attribute_range(&documents, "lifetime", 32)?;
        let emission_rate_per_second = *vfx_named_scalar_values(&documents, "Rate")
            .first()
            .context("healing-channel graph has no emission Rate")?;
        channels.insert(
            vfx_effect_id(&prefab.guid)?,
            HealingChannelVfxDef {
                display_name: prefab.name.clone(),
                source_guid: prefab.guid.clone(),
                source_path: prefab.path.clone(),
                graph_guid: graph.guid.clone(),
                graph_source: graph.path.clone(),
                particle_capacity: u16::try_from(particle_capacity)?,
                emission_rate_per_second,
                particle_lifetime_seconds,
                exposed_size: vfx_prefab_scalar(&prefab_contents, "Size")?,
                size_over_lifetime: vfx_prefab_curve(&prefab_contents, "Size overlife")?,
                color: vfx_prefab_gradient(&prefab_contents, "ColourGradient")?,
            },
        );
    }

    if let (Some(prefab), Some(graph), Some(model), Some(texture)) = (
        find(BURST_PREFAB),
        find(BURST_GRAPH),
        find(PLUS_MODEL),
        find(DISC_TEXTURE),
    ) {
        let prefab_contents = fs::read_to_string(unity_root.join(BURST_PREFAB))?;
        let graph_contents = fs::read_to_string(unity_root.join(BURST_GRAPH))?;
        let documents = parse_yaml_documents(&graph_contents)?;
        let plus_capacity = vfx_capacity(&documents, 8)?;
        let disc_capacity = vfx_capacity(&documents, 128)?;
        bursts.insert(
            vfx_effect_id(&prefab.guid)?,
            HealingBurstVfxDef {
                display_name: prefab.name.clone(),
                source_guid: prefab.guid.clone(),
                source_path: prefab.path.clone(),
                graph_guid: graph.guid.clone(),
                graph_source: graph.path.clone(),
                duration_seconds: prefab_disable_lifetime(&prefab_contents)?,
                plus_capacity: u16::try_from(plus_capacity)?,
                plus_burst_count: f32_to_u16(
                    vfx_burst_count(&documents, plus_capacity)?,
                    "healing plus burst count",
                )?,
                plus_lifetime_seconds: vfx_attribute_range(&documents, "lifetime", plus_capacity)?,
                plus_size_over_lifetime: vfx_attribute_curve(&documents, "size", plus_capacity)?,
                plus_color: vfx_prefab_gradient(&prefab_contents, "PlusColour")?,
                plus_model_source: model.path.clone(),
                plus_model_asset_path: glb_asset_path(&model.path),
                disc_capacity: u16::try_from(disc_capacity)?,
                disc_burst_count: f32_to_u16(
                    vfx_burst_count(&documents, disc_capacity)?,
                    "healing disc burst count",
                )?,
                disc_lifetime_seconds: vfx_attribute_values(&documents, "lifetime", disc_capacity)?
                    .first()
                    .copied()
                    .context("healing disc system has no lifetime")?,
                disc_size_multiplier: vfx_attribute_scalar(&documents, "size", disc_capacity, 2)?,
                disc_size_over_lifetime: vfx_attribute_curve(&documents, "size", disc_capacity)?,
                disc_color: vfx_prefab_gradient(&prefab_contents, "DiscColour")?,
                disc_texture: texture_id(&texture.guid)?,
            },
        );
    }

    Ok((channels, bursts))
}

fn yaml_section<'a>(lines: &'a [String], key: &str) -> Result<&'a [String]> {
    let start = lines
        .iter()
        .position(|line| line.trim() == key)
        .with_context(|| format!("YAML has no {key} section"))?;
    let indent = lines[start].len() - lines[start].trim_start().len();
    let end = lines
        .iter()
        .enumerate()
        .skip(start + 1)
        .find(|(_, line)| !line.trim().is_empty() && line.len() - line.trim_start().len() <= indent)
        .map_or(lines.len(), |(index, _)| index);
    Ok(&lines[start + 1..end])
}

fn required_scalar_f32(lines: &[String], key: &str, field: &str) -> Result<f32> {
    scalar_f32(lines, key).with_context(|| format!("chimney particle has no {field}"))
}

fn required_particle_scalar_f32(lines: &[String], key: &str, field: &str) -> Result<f32> {
    scalar_f32(lines, key).with_context(|| format!("particle has no {field}"))
}

fn yaml_keyframe_values(lines: &[String]) -> Vec<f32> {
    lines
        .iter()
        .filter_map(|line| line.trim().strip_prefix("value: "))
        .filter_map(|value| value.parse().ok())
        .collect()
}

fn yaml_float_keyframes(lines: &[String]) -> Result<Vec<AnimationFloatKeyframe>> {
    let mut keys = Vec::new();
    let mut pending = None;
    let finish = |keys: &mut Vec<AnimationFloatKeyframe>, pending: &mut Option<_>| {
        if let Some(key) = pending.take() {
            keys.push(key);
        }
    };
    for line in lines {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("time: ") {
            finish(&mut keys, &mut pending);
            pending = Some(AnimationFloatKeyframe {
                time: value.parse()?,
                value: 0.0,
                in_slope: AnimationTangent::Finite(0.0),
                out_slope: AnimationTangent::Finite(0.0),
                tangent_mode: 0,
                weighted_mode: 0,
                in_weight: 0.0,
                out_weight: 0.0,
            });
        } else if let Some(key) = pending.as_mut() {
            if let Some(value) = trimmed.strip_prefix("value: ") {
                key.value = value.parse()?;
            } else if let Some(value) = trimmed.strip_prefix("inSlope: ") {
                key.in_slope = parse_tangent(value)?;
            } else if let Some(value) = trimmed.strip_prefix("outSlope: ") {
                key.out_slope = parse_tangent(value)?;
            } else if let Some(value) = trimmed.strip_prefix("tangentMode: ") {
                key.tangent_mode = value.parse()?;
            } else if let Some(value) = trimmed.strip_prefix("weightedMode: ") {
                key.weighted_mode = value.parse()?;
            } else if let Some(value) = trimmed.strip_prefix("inWeight: ") {
                key.in_weight = value.parse()?;
            } else if let Some(value) = trimmed.strip_prefix("outWeight: ") {
                key.out_weight = value.parse()?;
            }
        }
    }
    finish(&mut keys, &mut pending);
    Ok(keys)
}

fn hierarchy_age(path: &str) -> Result<u8> {
    path.split('/')
        .find_map(|component| {
            component
                .strip_prefix("Age")
                .and_then(|suffix| suffix.get(..2))
                .and_then(|age| age.parse().ok())
        })
        .with_context(|| format!("chimney hierarchy {path:?} has no age marker"))
}

fn vfx_named_scalar_values(documents: &[YamlDocument], name: &str) -> Vec<f32> {
    documents
        .iter()
        .filter(|document| scalar(&document.lines, "name:") == Some(name))
        .filter_map(|document| scalar_f32(&document.lines, "m_SerializableObject:"))
        .collect()
}

fn vfx_capacity(documents: &[YamlDocument], expected: u32) -> Result<u32> {
    documents
        .iter()
        .filter_map(|document| scalar(&document.lines, "capacity:")?.parse().ok())
        .find(|capacity| *capacity == expected)
        .with_context(|| format!("VFX graph has no particle capacity {expected}"))
}

fn vfx_burst_count(documents: &[YamlDocument], capacity: u32) -> Result<f32> {
    documents
        .iter()
        .filter(|document| scalar(&document.lines, "spawnMode:").is_some())
        .filter(|document| vfx_spawn_capacity(documents, document) == Some(capacity))
        .find_map(|document| {
            yaml_reference_list(&document.lines, "m_InputSlots:")
                .into_iter()
                .filter_map(|id| documents.iter().find(|candidate| candidate.file_id == id))
                .find(|slot| scalar(&slot.lines, "name:") == Some("Count"))
                .and_then(|slot| scalar_f32(&slot.lines, "m_SerializableObject:"))
        })
        .with_context(|| format!("VFX graph has no burst Count for capacity {capacity}"))
}

fn vfx_spawn_capacity(documents: &[YamlDocument], burst: &YamlDocument) -> Option<u32> {
    let spawn_context = reference_id(&burst.lines, "m_Parent:")?;
    let spawn_context = documents
        .iter()
        .find(|document| document.file_id == spawn_context)?;
    let initialize_context = spawn_context.lines.iter().find_map(|line| {
        line.trim_start()
            .strip_prefix("- context:")
            .and_then(|_| inline_file_id(line))
    })?;
    let initialize_context = documents
        .iter()
        .find(|document| document.file_id == initialize_context)?;
    let data = reference_id(&initialize_context.lines, "m_Data:")?;
    let data = documents.iter().find(|document| document.file_id == data)?;
    scalar(&data.lines, "capacity:")?.parse().ok()
}

fn vfx_attribute_documents<'a>(
    documents: &'a [YamlDocument],
    attribute: &str,
    capacity: u32,
) -> impl Iterator<Item = &'a YamlDocument> {
    documents
        .iter()
        .filter(move |document| scalar(&document.lines, "attribute:") == Some(attribute))
        .filter(move |document| vfx_owner_capacity(documents, document) == Some(capacity))
}

fn vfx_attribute_scalar(
    documents: &[YamlDocument],
    attribute: &str,
    capacity: u32,
    composition: u32,
) -> Result<f32> {
    vfx_attribute_documents(documents, attribute, capacity)
        .find(|document| {
            scalar(&document.lines, "Composition:").and_then(|value| value.parse().ok())
                == Some(composition)
        })
        .and_then(|document| {
            yaml_reference_list(&document.lines, "m_InputSlots:")
                .into_iter()
                .filter_map(|id| documents.iter().find(|candidate| candidate.file_id == id))
                .find_map(|slot| scalar_f32(&slot.lines, "m_SerializableObject:"))
        })
        .with_context(|| format!("VFX graph has no {attribute} scalar for capacity {capacity}"))
}

fn vfx_attribute_curve(
    documents: &[YamlDocument],
    attribute: &str,
    capacity: u32,
) -> Result<Vec<AnimationFloatKeyframe>> {
    let encoded = vfx_attribute_documents(documents, attribute, capacity)
        .find_map(|document| {
            yaml_reference_list(&document.lines, "m_InputSlots:")
                .into_iter()
                .filter_map(|id| documents.iter().find(|candidate| candidate.file_id == id))
                .find_map(|slot| {
                    scalar(&slot.lines, "m_SerializableObject:")
                        .filter(|value| value.contains("\"frames\""))
                })
        })
        .with_context(|| format!("VFX graph has no {attribute} curve for capacity {capacity}"))?;
    vfx_json_curve(encoded)
}

fn vfx_prefab_scalar(contents: &str, name: &str) -> Result<f32> {
    let tail = contents
        .split_once(&format!("m_Name: {name}"))
        .with_context(|| format!("VFX prefab has no {name} override"))?
        .0;
    tail.lines()
        .rev()
        .find_map(|line| {
            line.trim()
                .strip_prefix("- m_Value: ")
                .or_else(|| line.trim().strip_prefix("m_Value: "))
        })
        .and_then(|value| value.parse().ok())
        .with_context(|| format!("VFX prefab {name} override has no scalar value"))
}

fn vfx_prefab_curve(contents: &str, name: &str) -> Result<Vec<AnimationFloatKeyframe>> {
    let tail = contents
        .split_once(&format!("m_Name: {name}"))
        .with_context(|| format!("VFX prefab has no {name} override"))?
        .0;
    let curve_start = tail
        .rfind("          m_Curve:")
        .context("VFX prefab curve has no key list")?;
    yaml_float_keyframes(
        &tail[curve_start..]
            .lines()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
    )
}

fn vfx_prefab_gradient(contents: &str, name: &str) -> Result<VfxGradientDef> {
    let before_name = contents
        .split_once(&format!("        m_Name: {name}"))
        .with_context(|| format!("VFX prefab has no {name} gradient override"))?
        .0;
    let start = before_name
        .rfind("          serializedVersion: 2")
        .context("VFX prefab gradient has no serialized value")?;
    let lines = before_name[start..]
        .lines()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let color_count: usize = scalar(&lines, "m_NumColorKeys:")
        .context("VFX gradient has no color-key count")?
        .parse()?;
    let alpha_count: usize = scalar(&lines, "m_NumAlphaKeys:")
        .context("VFX gradient has no alpha-key count")?
        .parse()?;
    let color_keys = (0..color_count)
        .map(|index| {
            let encoded = scalar(&lines, &format!("key{index}:"))
                .with_context(|| format!("VFX gradient has no color key {index}"))?;
            let color = inline_color(encoded, [0.0; 4]);
            let time = scalar_f32(&lines, &format!("ctime{index}:"))
                .with_context(|| format!("VFX gradient has no color time {index}"))?
                / 65_535.0;
            Ok(VfxColorKeyframe {
                time,
                color: [color[0], color[1], color[2]],
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let alpha_keys = (0..alpha_count)
        .map(|index| {
            let alpha = inline_color(
                scalar(&lines, &format!("key{index}:"))
                    .with_context(|| format!("VFX gradient has no alpha key {index}"))?,
                [0.0; 4],
            )[3];
            let time = scalar_f32(&lines, &format!("atime{index}:"))
                .with_context(|| format!("VFX gradient has no alpha time {index}"))?
                / 65_535.0;
            Ok(VfxAlphaKeyframe { time, alpha })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(VfxGradientDef {
        color_keys,
        alpha_keys,
    })
}

fn vfx_json_curve(encoded: &str) -> Result<Vec<AnimationFloatKeyframe>> {
    let value: Value = serde_json::from_str(encoded.trim_matches('\''))?;
    value
        .get("frames")
        .and_then(Value::as_array)
        .context("VFX curve has no frames")?
        .iter()
        .map(|frame| {
            let tangent_mode = frame
                .get("tangentMode")
                .and_then(Value::as_u64)
                .unwrap_or_default();
            Ok(AnimationFloatKeyframe {
                time: json_f32(frame, "time").context("VFX curve key has no time")?,
                value: json_f32(frame, "value").context("VFX curve key has no value")?,
                in_slope: AnimationTangent::Finite(
                    json_f32(frame, "inTangent").context("VFX curve key has no in tangent")?,
                ),
                out_slope: AnimationTangent::Finite(
                    json_f32(frame, "outTangent").context("VFX curve key has no out tangent")?,
                ),
                tangent_mode: u32::try_from(tangent_mode)?,
                weighted_mode: 0,
                in_weight: 0.0,
                out_weight: 0.0,
            })
        })
        .collect()
}

fn prefab_disable_lifetime(contents: &str) -> Result<f32> {
    contents
        .lines()
        .find_map(|line| line.trim().strip_prefix("_lifeTime: "))
        .and_then(|value| value.parse().ok())
        .context("healing prefab has no disable lifetime")
}

fn vfx_named_vec2(documents: &[YamlDocument], name: &str) -> Result<[f32; 2]> {
    let encoded = documents
        .iter()
        .find(|document| scalar(&document.lines, "name:") == Some(name))
        .and_then(|document| scalar(&document.lines, "m_SerializableObject:"))
        .with_context(|| format!("fireworks graph has no {name} property"))?;
    let value: Value = serde_json::from_str(encoded.trim_matches('\''))?;
    Ok([
        json_f32(&value, "x").with_context(|| format!("{name}.x is missing"))?,
        json_f32(&value, "y").with_context(|| format!("{name}.y is missing"))?,
    ])
}

fn vfx_attribute_range(
    documents: &[YamlDocument],
    attribute: &str,
    capacity: u32,
) -> Result<[f32; 2]> {
    let values = vfx_attribute_values(documents, attribute, capacity)?;
    if values.len() != 2 {
        bail!("fireworks {attribute} for capacity {capacity} is not a two-value range");
    }
    Ok([values[0], values[1]])
}

fn vfx_attribute_vector_range(
    documents: &[YamlDocument],
    attribute: &str,
    capacity: u32,
) -> Result<([f32; 3], [f32; 3])> {
    let context = vfx_attribute_document(documents, attribute, capacity)?;
    let input_ids = yaml_reference_list(&context.lines, "m_InputSlots:");
    let values = input_ids
        .iter()
        .filter_map(|id| documents.iter().find(|document| document.file_id == *id))
        .filter_map(|document| scalar(&document.lines, "m_SerializableObject:"))
        .filter_map(|encoded| serde_json::from_str::<Value>(encoded.trim_matches('\'')).ok())
        .filter_map(|value| value.get("vector").cloned())
        .filter_map(|value| {
            Some([
                json_f32(&value, "x")?,
                json_f32(&value, "y")?,
                json_f32(&value, "z")?,
            ])
        })
        .collect::<Vec<_>>();
    if values.len() != 2 {
        bail!("fireworks {attribute} for capacity {capacity} is not a two-vector range");
    }
    Ok((values[0], values[1]))
}

fn vfx_attribute_values(
    documents: &[YamlDocument],
    attribute: &str,
    capacity: u32,
) -> Result<Vec<f32>> {
    let context = vfx_attribute_document(documents, attribute, capacity)?;
    Ok(yaml_reference_list(&context.lines, "m_InputSlots:")
        .iter()
        .filter_map(|id| documents.iter().find(|document| document.file_id == *id))
        .filter_map(|document| scalar_f32(&document.lines, "m_SerializableObject:"))
        .collect())
}

fn vfx_attribute_document<'a>(
    documents: &'a [YamlDocument],
    attribute: &str,
    capacity: u32,
) -> Result<&'a YamlDocument> {
    documents
        .iter()
        .filter(|document| scalar(&document.lines, "attribute:") == Some(attribute))
        .find(|document| vfx_owner_capacity(documents, document) == Some(capacity))
        .with_context(|| format!("fireworks graph has no {attribute} for capacity {capacity}"))
}

fn vfx_owner_capacity(documents: &[YamlDocument], attribute: &YamlDocument) -> Option<u32> {
    let parent = reference_id(&attribute.lines, "m_Parent:")?;
    let context = documents
        .iter()
        .find(|document| document.file_id == parent)?;
    let data = reference_id(&context.lines, "m_Data:")?;
    let data = documents.iter().find(|document| document.file_id == data)?;
    scalar(&data.lines, "capacity:")?.parse().ok()
}

fn yaml_reference_list(lines: &[String], key: &str) -> Vec<i64> {
    let Some(start) = lines.iter().position(|line| line.trim() == key) else {
        return Vec::new();
    };
    lines
        .iter()
        .skip(start + 1)
        .take_while(|line| line.trim_start().starts_with("- {fileID:"))
        .filter_map(|line| inline_file_id(line))
        .collect()
}

fn vfx_parameter_scalar(contents: &str, marker: &str) -> Result<f32> {
    let tail = contents
        .split_once(marker)
        .with_context(|| format!("fireworks graph has no {marker}"))?
        .1;
    tail.lines()
        .find_map(|line| line.trim().strip_prefix("m_SerializableObject: "))
        .and_then(|value| value.trim_matches('\'').parse().ok())
        .with_context(|| format!("fireworks graph {marker} has no scalar default"))
}

fn vfx_gradient_colors(contents: &str, marker: &str) -> Result<Vec<[f32; 4]>> {
    let tail = contents
        .split_once(marker)
        .with_context(|| format!("fireworks graph has no {marker}"))?
        .1;
    let encoded = tail
        .lines()
        .find_map(|line| line.trim().strip_prefix("m_SerializableObject: "))
        .context("fireworks gradient has no serialized value")?;
    let value: Value = serde_json::from_str(encoded.trim_matches('\''))?;
    value
        .get("colorKeys")
        .and_then(Value::as_array)
        .context("fireworks gradient has no color keys")?
        .iter()
        .map(|key| {
            let color = key.get("color").context("gradient key has no color")?;
            Ok([
                json_f32(color, "r").context("gradient color.r is missing")?,
                json_f32(color, "g").context("gradient color.g is missing")?,
                json_f32(color, "b").context("gradient color.b is missing")?,
                json_f32(color, "a").context("gradient color.a is missing")?,
            ])
        })
        .collect()
}

#[allow(clippy::cast_possible_truncation)]
fn json_f32(value: &Value, key: &str) -> Option<f32> {
    value.get(key)?.as_f64().map(|value| value as f32)
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn f32_to_u16(value: f32, label: &str) -> Result<u16> {
    if !value.is_finite() || value < 0.0 || value > f32::from(u16::MAX) {
        bail!("{label} is outside u16 range");
    }
    Ok(value.round() as u16)
}

fn volume_parameter<'a>(lines: &'a [String], name: &str) -> Option<&'a str> {
    let header = format!("{name}:");
    let start = lines.iter().position(|line| line.trim() == header)?;
    let header_indent = lines[start].len() - lines[start].trim_start().len();
    lines
        .iter()
        .skip(start + 1)
        .take_while(|line| {
            line.trim().is_empty() || line.len() - line.trim_start().len() > header_indent
        })
        .find_map(|line| line.trim().strip_prefix("m_Value:").map(str::trim))
}

fn volume_parameter_f32(lines: &[String], name: &str) -> Option<f32> {
    volume_parameter(lines, name)?.parse().ok()
}

fn volume_parameter_bool(lines: &[String], name: &str) -> Option<bool> {
    match volume_parameter(lines, name)? {
        "0" | "false" | "False" => Some(false),
        "1" | "true" | "True" => Some(true),
        _ => None,
    }
}

fn inline_color(value: &str, fallback: [f32; 4]) -> [f32; 4] {
    [
        inline_component(value, "r").unwrap_or(fallback[0]),
        inline_component(value, "g").unwrap_or(fallback[1]),
        inline_component(value, "b").unwrap_or(fallback[2]),
        inline_component(value, "a").unwrap_or(fallback[3]),
    ]
}

fn inline_vec2(value: &str, fallback: [f32; 2]) -> [f32; 2] {
    [
        inline_component(value, "x").unwrap_or(fallback[0]),
        inline_component(value, "y").unwrap_or(fallback[1]),
    ]
}

fn inline_vec3(value: &str, fallback: [f32; 3]) -> [f32; 3] {
    [
        inline_component(value, "x").unwrap_or(fallback[0]),
        inline_component(value, "y").unwrap_or(fallback[1]),
        inline_component(value, "z").unwrap_or(fallback[2]),
    ]
}

fn inline_component(value: &str, component: &str) -> Option<f32> {
    value
        .trim_matches(|character| matches!(character, '{' | '}'))
        .split(',')
        .find_map(|entry| entry.trim().strip_prefix(&format!("{component}: ")))
        .and_then(|value| value.parse().ok())
}

fn reference_field<'a>(value: &'a Value, name: &str) -> Option<&'a str> {
    value.as_object()?.get(name)?.as_str()
}

fn post_process_profile_id(guid: &str) -> Result<StableId> {
    StableId::new(format!("post_process_profile:{guid}")).map_err(Into::into)
}

fn fireworks_effect_id(guid: &str) -> Result<StableId> {
    StableId::new(format!("vfx:{guid}")).map_err(Into::into)
}

fn vfx_effect_id(guid: &str) -> Result<StableId> {
    StableId::new(format!("vfx:{guid}")).map_err(Into::into)
}

fn particle_effect_id(guid: &str) -> Result<StableId> {
    StableId::new(format!("particle_effect:{guid}")).map_err(Into::into)
}

fn convert_textures(
    export: &UnityExport,
    unity_root: &Path,
    out_dir: &Path,
) -> Result<(BTreeMap<StableId, TextureDef>, u64)> {
    let assets_root = out_dir
        .parent()
        .context("presentation output must be inside an assets directory")?;
    let mut textures = BTreeMap::new();
    let mut total_bytes = 0_u64;
    for asset in &export.assets {
        if asset.kind != "asset" || !is_texture_path(&asset.path) {
            continue;
        }
        let relative = asset.path.strip_prefix("Assets/").unwrap_or(&asset.path);
        let asset_path = format!("migrated/textures/{relative}");
        let source = unity_root.join(&asset.path);
        let destination = assets_root.join(&asset_path);
        let parent = destination
            .parent()
            .context("generated texture has no parent directory")?;
        fs::create_dir_all(parent)?;
        let source_bytes = fs::read(&source)
            .with_context(|| format!("failed to read texture {}", source.display()))?;
        let needs_copy = fs::read(&destination).map_or(true, |current| current != source_bytes);
        if needs_copy {
            fs::write(&destination, &source_bytes)
                .with_context(|| format!("failed to write texture {}", destination.display()))?;
        }
        total_bytes = total_bytes.saturating_add(u64::try_from(source_bytes.len())?);
        textures.insert(
            texture_id(&asset.guid)?,
            TextureDef {
                display_name: asset.name.clone(),
                source_guid: asset.guid.clone(),
                source_path: asset.path.clone(),
                asset_path,
                sprite_border: unity_sprite_border(&source)?,
            },
        );
    }
    Ok((textures, total_bytes))
}

fn convert_shipping_fonts(unity_root: &Path, out_dir: &Path) -> Result<(usize, u64)> {
    const SHIPPING_FONTS: [&str; 2] = [
        "Assets/Fonts/Rubik-Bold.ttf",
        "Assets/Fonts/Luckiest Guy.ttf",
    ];
    let assets_root = out_dir
        .parent()
        .context("presentation output must be inside an assets directory")?;
    let destination_root = assets_root.join("migrated/fonts");
    fs::create_dir_all(&destination_root)?;
    let mut total_bytes = 0_u64;
    for source_path in SHIPPING_FONTS {
        let source = unity_root.join(source_path);
        let file_name = source
            .file_name()
            .context("shipping font has no file name")?;
        let destination = destination_root.join(file_name);
        let source_bytes = fs::read(&source)
            .with_context(|| format!("failed to read shipping font {}", source.display()))?;
        if fs::read(&destination).map_or(true, |current| current != source_bytes) {
            fs::write(&destination, &source_bytes).with_context(|| {
                format!("failed to write shipping font {}", destination.display())
            })?;
        }
        total_bytes = total_bytes.saturating_add(u64::try_from(source_bytes.len())?);
    }
    Ok((SHIPPING_FONTS.len(), total_bytes))
}

fn unity_sprite_border(texture_path: &Path) -> Result<Option<[f32; 4]>> {
    let file_name = texture_path
        .file_name()
        .and_then(|name| name.to_str())
        .context("Unity texture path has no portable file name")?;
    let meta_path = texture_path.with_file_name(format!("{file_name}.meta"));
    let encoded = match fs::read_to_string(&meta_path) {
        Ok(encoded) => encoded,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error).with_context(|| {
                format!("failed to read texture metadata {}", meta_path.display())
            });
        }
    };
    let Some(mapping) = encoded
        .lines()
        .find_map(|line| line.trim().strip_prefix("spriteBorder:").map(str::trim))
    else {
        return Ok(None);
    };
    let mapping = mapping
        .strip_prefix('{')
        .and_then(|mapping| mapping.strip_suffix('}'))
        .context("Unity spriteBorder must be an inline mapping")?;
    let mut components = BTreeMap::new();
    for component in mapping.split(',') {
        let (name, value) = component
            .split_once(':')
            .context("Unity spriteBorder component is missing a value")?;
        components.insert(
            name.trim(),
            value
                .trim()
                .parse::<f32>()
                .context("Unity spriteBorder component is not numeric")?,
        );
    }
    let component = |name| {
        components
            .get(name)
            .copied()
            .with_context(|| format!("Unity spriteBorder is missing {name}"))
    };
    // Unity serializes x=left, y=bottom, z=right, w=top. The neutral
    // presentation schema follows Bevy's left, right, top, bottom order.
    let border = [
        component("x")?,
        component("z")?,
        component("w")?,
        component("y")?,
    ];
    if border.iter().all(|value| value.abs() <= f32::EPSILON) {
        Ok(None)
    } else {
        Ok(Some(border))
    }
}

#[allow(clippy::cast_possible_truncation)]
fn convert_materials(
    export: &UnityExport,
    assets_by_path: &BTreeMap<&str, &UnityAsset>,
) -> Result<BTreeMap<StableId, MaterialDef>> {
    let mut materials = BTreeMap::new();
    for asset in export
        .assets
        .iter()
        .filter(|asset| asset.kind == "material")
    {
        let colors = named_values(asset, "m_SavedProperties.m_Colors.Array.data[");
        let floats = named_values(asset, "m_SavedProperties.m_Floats.Array.data[");
        let texture_values = named_values(asset, "m_SavedProperties.m_TexEnvs.Array.data[");
        let base_color = color_value(
            colors
                .get("_BaseColor")
                .or_else(|| colors.get("_Color"))
                .or_else(|| colors.get("_albedoColor")),
            [1.0; 4],
        );
        let emissive = color_value(
            colors
                .get("_EmissionColor")
                .or_else(|| colors.get("_EmissiveColour")),
            [0.0; 4],
        );
        let metallic = float_value(&floats, &["_Metallic", "_Metal"]).clamp(0.0, 1.0);
        let smoothness = float_value(&floats, &["_Smoothness", "_Glossiness"]).clamp(0.0, 1.0);
        let transparent_surface = float_value(&floats, &["_Surface", "_Mode"]) >= 1.0;
        let alpha_test = float_value(&floats, &["_AlphaClip"]) >= 0.5
            || asset.serialized_fields.iter().any(|field| {
                field.path.starts_with("m_ValidKeywords.Array.data[")
                    && field.value.as_str() == Some("_ALPHATEST_ON")
            });
        let alpha_mode = if base_color[3] < 0.999 || transparent_surface {
            MaterialAlphaMode::Blend
        } else if alpha_test {
            MaterialAlphaMode::Mask
        } else {
            MaterialAlphaMode::Opaque
        };
        let mut textures = BTreeMap::new();
        let mut texture_transforms = BTreeMap::new();
        for (slot, value) in texture_values {
            let Some(path) = value
                .get("m_Texture")
                .or_else(|| value.get("Texture"))
                .and_then(reference_path)
            else {
                continue;
            };
            if !is_texture_path(path) {
                continue;
            }
            let texture_asset = assets_by_path
                .get(path)
                .with_context(|| format!("{} references unexported texture {path}", asset.path))?;
            let transform = TextureTransform {
                scale: vec2_value(value.get("m_Scale"), [1.0; 2]),
                offset: vec2_value(value.get("m_Offset"), [0.0; 2]),
            };
            texture_transforms.insert(slot.clone(), transform);
            textures.insert(slot, texture_id(&texture_asset.guid)?);
        }
        let custom_properties = floats
            .into_iter()
            .filter_map(|(name, value)| {
                (!matches!(
                    name.as_str(),
                    "_Metallic"
                        | "_Metal"
                        | "_Smoothness"
                        | "_Glossiness"
                        | "_AlphaCutoff"
                        | "_Cutoff"
                ))
                .then(|| value.as_f64().map(|value| (name, value as f32)))
                .flatten()
            })
            .collect();
        let custom_vectors = colors
            .into_iter()
            .filter(|(name, _)| {
                !matches!(
                    name.as_str(),
                    "_BaseColor" | "_Color" | "_albedoColor" | "_EmissionColor" | "_EmissiveColour"
                )
            })
            .map(|(name, value)| (name, color_value(Some(&value), [0.0; 4])))
            .collect();
        let shader_source = asset
            .dependencies
            .iter()
            .filter_map(|dependency| dependency.path.as_deref())
            .find(|path| path.ends_with(".shader") || path.ends_with(".shadergraph"))
            .map(str::to_owned);
        materials.insert(
            material_id(&asset.guid)?,
            MaterialDef {
                display_name: asset.name.clone(),
                source_guid: asset.guid.clone(),
                source_path: asset.path.clone(),
                shader_source,
                base_color,
                emissive,
                metallic,
                perceptual_roughness: 1.0 - smoothness,
                alpha_mode,
                textures,
                texture_transforms,
                custom_properties,
                custom_vectors,
            },
        );
    }
    Ok(materials)
}

fn convert_clips(
    export: &UnityExport,
    unity_root: &Path,
) -> Result<BTreeMap<StableId, AnimationClipDef>> {
    let mut clips = BTreeMap::new();
    for asset in export
        .assets
        .iter()
        .filter(|asset| asset.kind == "animation_clip")
    {
        let start = field_f32(
            &asset.serialized_fields,
            "m_AnimationClipSettings.m_StartTime",
        )
        .unwrap_or(0.0);
        let stop = field_f32(
            &asset.serialized_fields,
            "m_AnimationClipSettings.m_StopTime",
        )
        .unwrap_or(start);
        let (transform_tracks, property_curves, events) = if Path::new(&asset.path)
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("anim"))
        {
            let source = unity_root.join(&asset.path);
            let contents = fs::read_to_string(&source)
                .with_context(|| format!("failed to read animation clip {}", source.display()))?;
            (
                parse_transform_tracks(&contents).with_context(|| {
                    format!("failed to parse animation clip {}", source.display())
                })?,
                parse_property_curves(&contents).with_context(|| {
                    format!("failed to parse property curves {}", source.display())
                })?,
                parse_animation_events(&contents).with_context(|| {
                    format!("failed to parse animation events {}", source.display())
                })?,
            )
        } else {
            (Vec::new(), Vec::new(), Vec::new())
        };
        clips.insert(
            clip_id(&asset.guid)?,
            AnimationClipDef {
                display_name: asset.name.clone(),
                source_guid: asset.guid.clone(),
                source_path: asset.path.clone(),
                duration_seconds: (stop - start).max(0.0),
                sample_rate: field_f32(&asset.serialized_fields, "m_SampleRate").unwrap_or(0.0),
                looping: field_bool(
                    &asset.serialized_fields,
                    "m_AnimationClipSettings.m_LoopTime",
                )
                .unwrap_or(false),
                rig_asset_path: None,
                transform_tracks,
                property_curves,
                events,
                converted_asset_path: None,
                gltf_animation_index: None,
            },
        );
    }
    Ok(clips)
}

fn convert_embedded_model_clips(
    export: &UnityExport,
    unity_root: &Path,
    clips: &mut BTreeMap<StableId, AnimationClipDef>,
) -> Result<BTreeMap<(String, i64), StableId>> {
    let mut embedded = BTreeMap::new();
    for model in export.assets.iter().filter(|asset| asset.kind == "model") {
        let configured_count = usize::try_from(
            field_u64(&model.importer_fields, "m_ClipAnimations.Array.size").unwrap_or(0),
        )
        .context("Unity model clip count does not fit the current platform")?;
        let imported_count = usize::try_from(
            field_u64(&model.importer_fields, "m_ImportedTakeInfos.Array.size").unwrap_or(0),
        )
        .context("Unity imported take count does not fit the current platform")?;
        let take_count = configured_count.max(imported_count);
        if take_count == 0 {
            continue;
        }
        let meta_path = unity_root.join(format!("{}.meta", model.path));
        let meta = fs::read_to_string(&meta_path)
            .with_context(|| format!("failed to read model metadata {}", meta_path.display()))?;
        let local_ids = parse_model_clip_local_ids(&meta);
        let animation_names = glb_animation_names(
            &unity_root
                .join("bevy-port/assets")
                .join(glb_asset_path(&model.path)),
        )?;
        // Several Unity importers retain stale clip configuration after the
        // corresponding FBX takes were removed. Only publish clips that the
        // reproducible GLB conversion can actually load.
        if animation_names.is_empty() {
            continue;
        }
        for index in 0..take_count {
            let take_prefix = format!("m_ImportedTakeInfos.Array.data[{index}]");
            let clip_prefix = format!("m_ClipAnimations.Array.data[{index}]");
            let Some(name) = field_str(&model.importer_fields, &format!("{clip_prefix}.name"))
                .or_else(|| {
                    field_str(
                        &model.importer_fields,
                        &format!("{take_prefix}.defaultClipName"),
                    )
                })
            else {
                continue;
            };
            let take_name = field_str(&model.importer_fields, &format!("{clip_prefix}.takeName"))
                .or_else(|| {
                    field_str(
                        &model.importer_fields,
                        &format!("{take_prefix}.defaultClipName"),
                    )
                })
                .unwrap_or(name);
            let local_id = local_ids.get(name).copied().with_context(|| {
                format!(
                    "embedded clip {name:?} in {} has no Unity local ID",
                    model.path
                )
            })?;
            let Some(animation_index) = animation_names
                .iter()
                .position(|animation| animation_take_name(animation) == take_name)
            else {
                continue;
            };
            let start = field_f32(&model.importer_fields, &format!("{take_prefix}.startTime"))
                .or_else(|| {
                    field_f32(&model.importer_fields, &format!("{clip_prefix}.firstFrame"))
                        .zip(field_f32(
                            &model.importer_fields,
                            &format!("{take_prefix}.sampleRate"),
                        ))
                        .map(|(frame, sample_rate)| frame / sample_rate.max(f32::EPSILON))
                })
                .unwrap_or(0.0);
            let stop = field_f32(&model.importer_fields, &format!("{take_prefix}.stopTime"))
                .or_else(|| {
                    field_f32(&model.importer_fields, &format!("{clip_prefix}.lastFrame"))
                        .zip(field_f32(
                            &model.importer_fields,
                            &format!("{take_prefix}.sampleRate"),
                        ))
                        .map(|(frame, sample_rate)| frame / sample_rate.max(f32::EPSILON))
                })
                .unwrap_or(start);
            let sample_rate =
                field_f32(&model.importer_fields, &format!("{take_prefix}.sampleRate"))
                    .unwrap_or(0.0);
            let looping = field_bool(&model.importer_fields, &format!("{clip_prefix}.loopTime"))
                .unwrap_or(false);
            let id = embedded_clip_id(&model.guid, local_id)?;
            clips.insert(
                id.clone(),
                AnimationClipDef {
                    display_name: name.to_owned(),
                    source_guid: model.guid.clone(),
                    source_path: model.path.clone(),
                    duration_seconds: (stop - start).max(0.0),
                    sample_rate,
                    looping,
                    rig_asset_path: Some(glb_asset_path(&model.path)),
                    transform_tracks: Vec::new(),
                    property_curves: Vec::new(),
                    events: Vec::new(),
                    converted_asset_path: Some(glb_asset_path(&model.path)),
                    gltf_animation_index: Some(
                        u32::try_from(animation_index)
                            .context("GLB animation index does not fit in u32")?,
                    ),
                },
            );
            embedded.insert((model.guid.clone(), local_id), id);
        }
    }
    Ok(embedded)
}

fn parse_model_clip_local_ids(contents: &str) -> BTreeMap<String, i64> {
    let mut result = BTreeMap::new();
    let mut local_id = None;
    for line in contents.lines() {
        if line.trim() == "externalObjects:" {
            break;
        }
        if let Some(value) = line.trim().strip_prefix("74: ") {
            local_id = value.parse().ok();
        } else if let Some(name) = line.trim().strip_prefix("second: ")
            && let Some(local_id) = local_id.take()
        {
            result.insert(name.trim_matches(['\'', '"']).to_owned(), local_id);
        }
    }
    result
}

fn parse_transform_tracks(contents: &str) -> Result<Vec<AnimationTransformTrack>> {
    let mut tracks = BTreeMap::<String, AnimationTransformTrack>::new();
    let mut kind = None;
    let mut times = Vec::<f32>::new();
    let mut vec3_values = Vec::<[f32; 3]>::new();
    let mut quat_values = Vec::<[f32; 4]>::new();
    let mut pending_time = None;

    for line in contents.lines() {
        if line.starts_with("  m_") {
            kind = match line.trim() {
                "m_RotationCurves:" => Some(TransformCurveKind::Rotation),
                "m_EulerCurves:" => Some(TransformCurveKind::EulerDegrees),
                "m_PositionCurves:" => Some(TransformCurveKind::Translation),
                "m_ScaleCurves:" => Some(TransformCurveKind::Scale),
                _ => None,
            };
            continue;
        }
        let Some(curve_kind) = kind else {
            continue;
        };
        if line == "  - curve:" {
            times.clear();
            vec3_values.clear();
            quat_values.clear();
            pending_time = None;
            continue;
        }
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("time: ") {
            pending_time = Some(value.parse::<f32>()?);
            continue;
        }
        if let Some(value) = trimmed.strip_prefix("value: ") {
            let Some(time) = pending_time.take() else {
                continue;
            };
            times.push(time.max(0.0));
            match curve_kind {
                TransformCurveKind::Rotation => quat_values.push(parse_inline_array(value)?),
                TransformCurveKind::EulerDegrees
                | TransformCurveKind::Translation
                | TransformCurveKind::Scale => vec3_values.push(parse_inline_array(value)?),
            }
            continue;
        }
        let Some(path) = line.strip_prefix("    path:") else {
            continue;
        };
        let path = path.trim();
        let target_path = if path.is_empty() { "$root" } else { path };
        let track =
            tracks
                .entry(target_path.to_owned())
                .or_insert_with(|| AnimationTransformTrack {
                    target_path: target_path.to_owned(),
                    reference_translation: None,
                    reference_rotation: None,
                    reference_scale: None,
                    translation: Vec::new(),
                    rotation: Vec::new(),
                    scale: Vec::new(),
                    euler_degrees: Vec::new(),
                });
        match curve_kind {
            TransformCurveKind::Rotation => {
                track.rotation.extend(
                    times
                        .iter()
                        .copied()
                        .zip(quat_values.drain(..))
                        .map(|(time, value)| AnimationQuatKeyframe { time, value }),
                );
            }
            TransformCurveKind::EulerDegrees => {
                append_vec3_keys(&mut track.euler_degrees, &times, vec3_values.drain(..));
            }
            TransformCurveKind::Translation => {
                append_vec3_keys(&mut track.translation, &times, vec3_values.drain(..));
            }
            TransformCurveKind::Scale => {
                append_vec3_keys(&mut track.scale, &times, vec3_values.drain(..));
            }
        }
        times.clear();
        pending_time = None;
    }
    Ok(tracks
        .into_values()
        .filter(|track| {
            !track.translation.is_empty()
                || !track.rotation.is_empty()
                || !track.scale.is_empty()
                || !track.euler_degrees.is_empty()
        })
        .collect())
}

fn parse_property_curves(contents: &str) -> Result<Vec<AnimationPropertyCurve>> {
    let mut curves = Vec::new();
    let mut active = false;
    let mut target_path = String::new();
    let mut attribute = String::new();
    let mut class_id = 0;
    let mut script_guid = None;
    let mut keys = Vec::new();
    let mut pending_key: Option<AnimationFloatKeyframe> = None;

    let finish_key = |keys: &mut Vec<AnimationFloatKeyframe>, pending: &mut Option<_>| {
        if let Some(key) = pending.take() {
            keys.push(key);
        }
    };
    let finish_curve = |curves: &mut Vec<AnimationPropertyCurve>,
                        target_path: &mut String,
                        attribute: &mut String,
                        class_id: &mut i32,
                        script_guid: &mut Option<String>,
                        keys: &mut Vec<AnimationFloatKeyframe>| {
        if !attribute.is_empty() && !keys.is_empty() {
            curves.push(AnimationPropertyCurve {
                target_path: std::mem::take(target_path),
                attribute: std::mem::take(attribute),
                class_id: *class_id,
                script_guid: script_guid.take(),
                keys: std::mem::take(keys),
            });
        }
        target_path.clear();
        attribute.clear();
        *class_id = 0;
        *script_guid = None;
        keys.clear();
    };

    for line in contents.lines() {
        if line == "  m_FloatCurves:" {
            active = true;
            continue;
        }
        if active && line.starts_with("  m_") {
            finish_key(&mut keys, &mut pending_key);
            finish_curve(
                &mut curves,
                &mut target_path,
                &mut attribute,
                &mut class_id,
                &mut script_guid,
                &mut keys,
            );
            break;
        }
        if !active {
            continue;
        }
        if line == "  - curve:" {
            finish_key(&mut keys, &mut pending_key);
            finish_curve(
                &mut curves,
                &mut target_path,
                &mut attribute,
                &mut class_id,
                &mut script_guid,
                &mut keys,
            );
            continue;
        }
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("time: ") {
            finish_key(&mut keys, &mut pending_key);
            pending_key = Some(AnimationFloatKeyframe {
                time: value.parse()?,
                value: 0.0,
                in_slope: AnimationTangent::Finite(0.0),
                out_slope: AnimationTangent::Finite(0.0),
                tangent_mode: 0,
                weighted_mode: 0,
                in_weight: 0.0,
                out_weight: 0.0,
            });
        } else if let Some(value) = trimmed.strip_prefix("value: ")
            && let Some(key) = pending_key.as_mut()
        {
            key.value = value.parse()?;
        } else if let Some(value) = trimmed.strip_prefix("inSlope: ")
            && let Some(key) = pending_key.as_mut()
        {
            key.in_slope = parse_tangent(value)?;
        } else if let Some(value) = trimmed.strip_prefix("outSlope: ")
            && let Some(key) = pending_key.as_mut()
        {
            key.out_slope = parse_tangent(value)?;
        } else if let Some(value) = trimmed.strip_prefix("tangentMode: ")
            && let Some(key) = pending_key.as_mut()
        {
            key.tangent_mode = value.parse()?;
        } else if let Some(value) = trimmed.strip_prefix("weightedMode: ")
            && let Some(key) = pending_key.as_mut()
        {
            key.weighted_mode = value.parse()?;
        } else if let Some(value) = trimmed.strip_prefix("inWeight: ")
            && let Some(key) = pending_key.as_mut()
        {
            key.in_weight = value.parse()?;
        } else if let Some(value) = trimmed.strip_prefix("outWeight: ")
            && let Some(key) = pending_key.as_mut()
        {
            key.out_weight = value.parse()?;
        } else if let Some(value) = line.strip_prefix("    attribute: ") {
            finish_key(&mut keys, &mut pending_key);
            attribute = unity_scalar(value);
        } else if let Some(value) = line.strip_prefix("    path:") {
            target_path = unity_scalar(value);
        } else if let Some(value) = line.strip_prefix("    classID: ") {
            class_id = value.parse()?;
        } else if let Some(value) = line.strip_prefix("    script: ") {
            script_guid = reference_guid(value).map(str::to_owned);
        }
    }
    Ok(curves)
}

fn parse_tangent(value: &str) -> Result<AnimationTangent> {
    match value.trim() {
        "Infinity" | "+Infinity" => Ok(AnimationTangent::PositiveInfinity),
        "-Infinity" => Ok(AnimationTangent::NegativeInfinity),
        value => Ok(AnimationTangent::Finite(value.parse()?)),
    }
}

fn parse_animation_events(contents: &str) -> Result<Vec<AnimationEventDef>> {
    let mut events = Vec::new();
    let mut active = false;
    let mut pending: Option<AnimationEventDef> = None;
    for line in contents.lines() {
        if line == "  m_Events: []" {
            return Ok(Vec::new());
        }
        if line == "  m_Events:" {
            active = true;
            continue;
        }
        if !active {
            continue;
        }
        if let Some(value) = line.strip_prefix("  - time: ") {
            if let Some(event) = pending.take() {
                events.push(event);
            }
            pending = Some(AnimationEventDef {
                time: value.parse()?,
                function_name: String::new(),
                string_parameter: String::new(),
                object_reference: None,
                float_parameter: 0.0,
                int_parameter: 0,
                message_options: 0,
            });
            continue;
        }
        let Some(event) = pending.as_mut() else {
            continue;
        };
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("functionName: ") {
            event.function_name = unity_scalar(value);
        } else if let Some(value) = trimmed.strip_prefix("data:") {
            event.string_parameter = unity_scalar(value);
        } else if let Some(value) = trimmed.strip_prefix("objectReferenceParameter: ") {
            event.object_reference = parse_object_reference(value);
        } else if let Some(value) = trimmed.strip_prefix("floatParameter: ") {
            event.float_parameter = value.parse()?;
        } else if let Some(value) = trimmed.strip_prefix("intParameter: ") {
            event.int_parameter = value.parse()?;
        } else if let Some(value) = trimmed.strip_prefix("messageOptions: ") {
            event.message_options = value.parse()?;
        }
    }
    if let Some(event) = pending {
        events.push(event);
    }
    Ok(events)
}

fn parse_object_reference(value: &str) -> Option<AnimationObjectReference> {
    let file_id = inline_mapping_value(value, "fileID")?.parse().ok()?;
    (file_id != 0).then(|| AnimationObjectReference {
        file_id,
        guid: inline_mapping_value(value, "guid").map(str::to_owned),
        type_id: inline_mapping_value(value, "type").and_then(|value| value.parse().ok()),
    })
}

fn inline_mapping_value<'a>(value: &'a str, key: &str) -> Option<&'a str> {
    value
        .trim()
        .strip_prefix('{')?
        .strip_suffix('}')?
        .split(',')
        .filter_map(|entry| entry.trim().split_once(':'))
        .find_map(|(entry_key, value)| (entry_key.trim() == key).then(|| value.trim()))
}

fn unity_scalar(value: &str) -> String {
    let value = value.trim();
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value)
        .to_owned()
}

fn append_vec3_keys(
    destination: &mut Vec<AnimationVec3Keyframe>,
    times: &[f32],
    values: impl Iterator<Item = [f32; 3]>,
) {
    destination.extend(
        times
            .iter()
            .copied()
            .zip(values)
            .map(|(time, value)| AnimationVec3Keyframe { time, value }),
    );
}

fn parse_inline_array<const N: usize>(value: &str) -> Result<[f32; N]> {
    let body = value
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .context("Unity vector value is not an inline mapping")?;
    let parsed: Vec<f32> = body
        .split(',')
        .map(|component| {
            component
                .split_once(':')
                .context("Unity vector component has no colon")?
                .1
                .trim()
                .parse::<f32>()
                .map_err(Into::into)
        })
        .collect::<Result<_>>()?;
    parsed.try_into().map_err(|values: Vec<f32>| {
        anyhow::anyhow!("expected {N} components, got {}", values.len())
    })
}

fn convert_controllers(
    export: &UnityExport,
    unity_root: &Path,
    assets_by_guid: &BTreeMap<&str, &UnityAsset>,
    embedded_clips: &BTreeMap<(String, i64), StableId>,
    clips: &mut BTreeMap<StableId, AnimationClipDef>,
) -> Result<BTreeMap<StableId, AnimationControllerDef>> {
    let mut controllers = BTreeMap::new();
    for asset in export
        .assets
        .iter()
        .filter(|asset| asset.kind == "animator_controller")
    {
        let source = unity_root.join(&asset.path);
        let contents = fs::read_to_string(&source)
            .with_context(|| format!("failed to read controller {}", source.display()))?;
        let controller = parse_controller(asset, &contents, assets_by_guid, embedded_clips, clips)?;
        controllers.insert(controller_id(&asset.guid)?, controller);
    }
    Ok(controllers)
}

fn convert_avatar_masks(
    export: &UnityExport,
    unity_root: &Path,
) -> Result<BTreeMap<StableId, AvatarMaskDef>> {
    let mut sources = BTreeMap::<String, (String, String)>::new();
    for asset in export
        .assets
        .iter()
        .filter(|asset| is_avatar_mask_path(&asset.path))
    {
        sources.insert(asset.guid.clone(), (asset.path.clone(), asset.name.clone()));
    }
    for dependency in export
        .assets
        .iter()
        .flat_map(|asset| asset.dependencies.iter())
    {
        let Some((guid, path)) = dependency.guid.as_deref().zip(dependency.path.as_deref()) else {
            continue;
        };
        if is_avatar_mask_path(path) {
            let display_name = Path::new(path)
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("AvatarMask")
                .to_owned();
            sources
                .entry(guid.to_owned())
                .or_insert_with(|| (path.to_owned(), display_name));
        }
    }

    sources
        .into_iter()
        .map(|(guid, (path, fallback_name))| {
            let source = unity_root.join(&path);
            let contents = fs::read_to_string(&source)
                .with_context(|| format!("failed to read avatar mask {}", source.display()))?;
            let mask = parse_avatar_mask(&guid, &path, &fallback_name, &contents)?;
            Ok((avatar_mask_id(&guid)?, mask))
        })
        .collect()
}

fn parse_avatar_mask(
    guid: &str,
    source_path: &str,
    fallback_name: &str,
    contents: &str,
) -> Result<AvatarMaskDef> {
    let documents = parse_yaml_documents(contents)?;
    let document = documents
        .iter()
        .find(|document| document.class_id == 319)
        .with_context(|| format!("{source_path} has no Unity AvatarMask document"))?;
    let mut transform_weights = BTreeMap::new();
    let mut index = 0;
    while index < document.lines.len() {
        let Some(path) = document.lines[index]
            .trim_start()
            .strip_prefix("- m_Path:")
            .map(str::trim)
        else {
            index += 1;
            continue;
        };
        let weight = document.lines[index + 1..]
            .iter()
            .take_while(|line| !line.trim_start().starts_with("- m_Path:"))
            .find_map(|line| line.trim().strip_prefix("m_Weight:").map(str::trim))
            .with_context(|| format!("avatar mask {source_path} path {path:?} has no weight"))?
            .parse::<f32>()
            .with_context(|| {
                format!("avatar mask {source_path} path {path:?} has invalid weight")
            })?;
        transform_weights.insert(path.to_owned(), weight);
        index += 1;
    }
    Ok(AvatarMaskDef {
        display_name: scalar(&document.lines, "m_Name:")
            .unwrap_or(fallback_name)
            .to_owned(),
        source_guid: guid.to_owned(),
        source_path: source_path.to_owned(),
        humanoid_body_mask_hex: scalar(&document.lines, "m_Mask:")
            .unwrap_or_default()
            .to_owned(),
        transform_weights,
    })
}

fn parse_controller(
    asset: &UnityAsset,
    contents: &str,
    assets_by_guid: &BTreeMap<&str, &UnityAsset>,
    embedded_clips: &BTreeMap<(String, i64), StableId>,
    clips: &mut BTreeMap<StableId, AnimationClipDef>,
) -> Result<AnimationControllerDef> {
    let documents = parse_yaml_documents(contents)?;
    let blend_trees: BTreeMap<i64, ParsedBlendTree> = documents
        .iter()
        .filter(|document| document.class_id == 206)
        .map(|document| (document.file_id, parse_blend_tree(&document.lines)))
        .collect();
    let state_documents: Vec<_> = documents
        .iter()
        .filter(|document| document.class_id == 1102)
        .collect();
    let state_ids: BTreeMap<_, _> = state_documents
        .iter()
        .map(|document| {
            Ok((
                document.file_id,
                animation_state_id(&asset.guid, document.file_id)?,
            ))
        })
        .collect::<Result<_>>()?;
    let state_machine_documents: BTreeMap<_, _> = documents
        .iter()
        .filter(|document| document.class_id == 1107)
        .map(|document| (document.file_id, document))
        .collect();
    let state_machine_ids: BTreeMap<_, _> = state_machine_documents
        .keys()
        .map(|file_id| Ok((*file_id, animation_state_machine_id(&asset.guid, *file_id)?)))
        .collect::<Result<_>>()?;
    let state_owners: BTreeMap<_, _> = state_machine_documents
        .iter()
        .flat_map(|(machine, document)| {
            parse_child_references(&document.lines, "m_ChildStates:", "m_State:")
                .into_iter()
                .map(move |state| (state, *machine))
        })
        .collect();
    let any_state_transitions: BTreeMap<_, _> = state_machine_documents
        .iter()
        .flat_map(|(machine, document)| {
            parse_reference_list(&document.lines, "m_AnyStateTransitions:")
                .into_iter()
                .map(move |transition| (transition, *machine))
        })
        .collect();
    let entry_transition_sources: BTreeMap<_, _> = state_machine_documents
        .iter()
        .flat_map(|(machine, document)| {
            parse_reference_list(&document.lines, "m_EntryTransitions:")
                .into_iter()
                .map(move |transition| (transition, *machine))
        })
        .collect();
    let state_machine_transition_sources: BTreeMap<_, _> = state_machine_documents
        .values()
        .flat_map(|document| parse_state_machine_transition_sources(&document.lines))
        .collect();

    let mut transition_sources = BTreeMap::new();
    let mut states = BTreeMap::new();
    for document in state_documents {
        let state_id = state_ids[&document.file_id].clone();
        for transition in parse_reference_list(&document.lines, "m_Transitions:") {
            transition_sources.insert(transition, state_id.clone());
        }
        let parsed_motion = parse_state_motions(document, &blend_trees);
        let mut converted_motions = Vec::new();
        for motion in parsed_motion.motions {
            let clip = embedded_clips
                .get(&(motion.guid.clone(), motion.local_id))
                .cloned()
                .unwrap_or(clip_id(&motion.guid)?);
            if !clips.contains_key(&clip) {
                let source_asset = assets_by_guid.get(motion.guid.as_str());
                clips.insert(
                    clip.clone(),
                    AnimationClipDef {
                        display_name: source_asset.map_or_else(
                            || format!("Missing clip {}", motion.guid),
                            |asset| asset.name.clone(),
                        ),
                        source_guid: motion.guid.clone(),
                        source_path: source_asset
                            .map_or_else(String::new, |asset| asset.path.clone()),
                        duration_seconds: 0.0,
                        sample_rate: 0.0,
                        looping: true,
                        rig_asset_path: None,
                        transform_tracks: Vec::new(),
                        property_curves: Vec::new(),
                        events: Vec::new(),
                        converted_asset_path: None,
                        gltf_animation_index: None,
                    },
                );
            }
            converted_motions.push(AnimationMotionDef {
                clip,
                threshold: motion.threshold,
            });
        }
        states.insert(
            state_id,
            AnimationStateDef {
                display_name: scalar(&document.lines, "m_Name:")
                    .unwrap_or("Unnamed")
                    .to_owned(),
                speed: scalar_f32(&document.lines, "m_Speed:").unwrap_or(1.0),
                speed_parameter: scalar_bool(&document.lines, "m_SpeedParameterActive:")
                    .unwrap_or(false)
                    .then(|| scalar(&document.lines, "m_SpeedParameter:").unwrap_or_default())
                    .filter(|parameter| !parameter.is_empty())
                    .map(str::to_owned),
                blend_parameter: parsed_motion.parameter,
                motions: converted_motions,
            },
        );
    }

    let transitions: Vec<_> = documents
        .iter()
        .filter(|document| matches!(document.class_id, 1101 | 1109))
        .map(|document| {
            let destination_file = reference_id(&document.lines, "m_DstState:").unwrap_or(0);
            let source = transition_sources.get(&document.file_id).cloned();
            let destination = state_ids.get(&destination_file).cloned();
            let source_machine_file = entry_transition_sources
                .get(&document.file_id)
                .copied()
                .or_else(|| {
                    state_machine_transition_sources
                        .get(&document.file_id)
                        .copied()
                })
                .or_else(|| any_state_transitions.get(&document.file_id).copied())
                .or_else(|| {
                    source
                        .as_ref()
                        .and_then(|source| {
                            state_ids
                                .iter()
                                .find_map(|(file, id)| (id == source).then_some(*file))
                        })
                        .and_then(|state| state_owners.get(&state).copied())
                });
            let destination_machine_file =
                reference_id(&document.lines, "m_DstStateMachine:").filter(|file| *file != 0);
            let is_exit = scalar_bool(&document.lines, "m_IsExit:").unwrap_or(false);
            if source.is_none()
                && destination.is_none()
                && destination_machine_file.is_none()
                && !is_exit
            {
                return Ok(None);
            }
            Ok(Some(AnimationTransitionDef {
                source,
                destination,
                source_state_machine: source_machine_file
                    .and_then(|file| state_machine_ids.get(&file).cloned()),
                destination_state_machine: destination_machine_file
                    .and_then(|file| state_machine_ids.get(&file).cloned()),
                is_entry: entry_transition_sources.contains_key(&document.file_id),
                is_any_state: any_state_transitions.contains_key(&document.file_id),
                is_exit,
                has_exit_time: scalar_bool(&document.lines, "m_HasExitTime:").unwrap_or(false),
                exit_time: scalar_f32(&document.lines, "m_ExitTime:").unwrap_or(0.0),
                duration: scalar_f32(&document.lines, "m_TransitionDuration:").unwrap_or(0.0),
                fixed_duration: scalar_bool(&document.lines, "m_HasFixedDuration:").unwrap_or(true),
                offset: scalar_f32(&document.lines, "m_TransitionOffset:").unwrap_or(0.0),
                conditions: parse_conditions(&document.lines)?,
            }))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();
    let mut default_states: Vec<_> = documents
        .iter()
        .filter(|document| document.class_id == 1107)
        .filter_map(|document| reference_id(&document.lines, "m_DefaultState:"))
        .filter(|file_id| *file_id != 0)
        .filter_map(|file_id| state_ids.get(&file_id).cloned())
        .collect();
    default_states.sort();
    default_states.dedup();
    let state_machines = state_machine_documents
        .iter()
        .map(|(file_id, document)| {
            let id = state_machine_ids[file_id].clone();
            let states = parse_child_references(&document.lines, "m_ChildStates:", "m_State:")
                .into_iter()
                .filter_map(|state| state_ids.get(&state).cloned())
                .collect();
            let child_state_machines =
                parse_child_references(&document.lines, "m_ChildStateMachines:", "m_StateMachine:")
                    .into_iter()
                    .filter_map(|machine| state_machine_ids.get(&machine).cloned())
                    .collect();
            let default_state = reference_id(&document.lines, "m_DefaultState:")
                .and_then(|state| state_ids.get(&state).cloned());
            (
                id,
                AnimationStateMachineDef {
                    display_name: scalar(&document.lines, "m_Name:")
                        .unwrap_or("Unnamed")
                        .to_owned(),
                    states,
                    child_state_machines,
                    default_state,
                },
            )
        })
        .collect();
    let layers = documents
        .iter()
        .find(|document| document.class_id == 91)
        .map_or_else(Vec::new, |document| {
            parse_layers(&document.lines, &state_machine_ids)
        });
    let mut parameters = documents
        .iter()
        .find(|document| document.class_id == 91)
        .map_or_else(Vec::new, |document| parse_parameters(&document.lines));
    infer_missing_parameters(&states, &transitions, &mut parameters);
    Ok(AnimationControllerDef {
        display_name: asset.name.clone(),
        source_guid: asset.guid.clone(),
        source_path: asset.path.clone(),
        parameters,
        states,
        transitions,
        state_machines,
        layers,
        default_states,
    })
}

fn convert_prefab_bindings(
    export: &UnityExport,
    assets_by_path: &BTreeMap<&str, &UnityAsset>,
    controllers: &BTreeMap<StableId, AnimationControllerDef>,
) -> BTreeMap<String, PrefabPresentationBinding> {
    let controller_paths: BTreeMap<_, _> = controllers
        .iter()
        .map(|(id, controller)| (controller.source_path.as_str(), id))
        .collect();
    let mut bindings = BTreeMap::new();
    for prefab in export.assets.iter().filter(|asset| asset.kind == "prefab") {
        let mut dependency_paths = BTreeSet::new();
        collect_prefab_dependencies(
            prefab,
            assets_by_path,
            &mut BTreeSet::new(),
            &mut dependency_paths,
        );
        let authored_controller = animator_reference_path(prefab, "runtimeAnimatorController");
        let Some(controller) = authored_controller
            .and_then(|path| controller_paths.get(path).copied())
            .or_else(|| {
                dependency_paths
                    .iter()
                    .find_map(|path| controller_paths.get(path.as_str()).copied())
            })
        else {
            continue;
        };
        let preferred_model = preferred_animator_model(prefab, &dependency_paths, assets_by_path);
        let animated_model = preferred_model
            .filter(|path| {
                assets_by_path
                    .get(*path)
                    .is_some_and(|model| model_has_animation(model))
            })
            .or_else(|| {
                dependency_paths.iter().find_map(|path| {
                    let model = assets_by_path.get(path.as_str())?;
                    (model.kind == "model" && model_has_animation(model)).then_some(path.as_str())
                })
            });
        let rig_model = preferred_model.or(animated_model).or_else(|| {
            dependency_paths.iter().find_map(|path| {
                assets_by_path
                    .get(path.as_str())
                    .is_some_and(|asset| asset.kind == "model")
                    .then_some(path.as_str())
            })
        });
        bindings.insert(
            prefab.guid.clone(),
            PrefabPresentationBinding {
                source_prefab_path: prefab.path.clone(),
                controller: controller.clone(),
                rig_scene: rig_model.map(glb_asset_path),
                animated_scene: animated_model.map(glb_asset_path),
                gltf_animation_index: animated_model.map(|_| 0),
            },
        );
    }
    bindings
}

fn animator_component(prefab: &UnityAsset) -> Option<&UnityComponent> {
    prefab
        .game_object
        .as_ref()?
        .components
        .iter()
        .find(|component| {
            component
                .type_name
                .as_deref()
                .is_some_and(|name| name.starts_with("UnityEngine.Animator,"))
        })
}

fn animator_reference_path<'a>(prefab: &'a UnityAsset, field_path: &str) -> Option<&'a str> {
    animator_component(prefab)?
        .fields
        .iter()
        .find(|field| field.path == field_path)
        .and_then(|field| reference_path(&field.value))
}

fn preferred_animator_model<'a>(
    prefab: &UnityAsset,
    dependency_paths: &'a BTreeSet<String>,
    assets_by_path: &BTreeMap<&str, &UnityAsset>,
) -> Option<&'a str> {
    let animator_name = animator_component(prefab)?
        .hierarchy_path
        .rsplit('/')
        .next()
        .filter(|name| !name.is_empty())?;
    dependency_paths.iter().find_map(|path| {
        let model = assets_by_path.get(path.as_str())?;
        if model.kind != "model" {
            return None;
        }
        let stem = Path::new(path).file_stem()?.to_str()?;
        (stem.eq_ignore_ascii_case(animator_name) || model.name.eq_ignore_ascii_case(animator_name))
            .then_some(path.as_str())
    })
}

fn assign_clip_rigs_and_reference_poses(
    export: &UnityExport,
    assets_by_path: &BTreeMap<&str, &UnityAsset>,
    controllers: &BTreeMap<StableId, AnimationControllerDef>,
    bindings: &BTreeMap<String, PrefabPresentationBinding>,
    clips: &mut BTreeMap<StableId, AnimationClipDef>,
) {
    let mut controller_models = BTreeMap::<StableId, BTreeSet<String>>::new();
    for binding in bindings.values() {
        let Some(prefab) = assets_by_path.get(binding.source_prefab_path.as_str()) else {
            continue;
        };
        let mut dependencies = BTreeSet::new();
        collect_prefab_dependencies(
            prefab,
            assets_by_path,
            &mut BTreeSet::new(),
            &mut dependencies,
        );
        let models = controller_models
            .entry(binding.controller.clone())
            .or_default();
        models.extend(dependencies.into_iter().filter(|path| {
            assets_by_path
                .get(path.as_str())
                .is_some_and(|asset| asset.kind == "model")
        }));
    }

    let mut candidate_models = BTreeMap::<StableId, BTreeSet<String>>::new();
    for (controller_id, controller) in controllers {
        let Some(models) = controller_models.get(controller_id) else {
            continue;
        };
        for motion in controller
            .states
            .values()
            .flat_map(|state| state.motions.iter())
        {
            candidate_models
                .entry(motion.clip.clone())
                .or_default()
                .extend(models.iter().cloned());
        }
    }

    let rest_poses: BTreeMap<_, _> = export
        .assets
        .iter()
        .filter(|asset| asset.kind == "model")
        .map(|asset| (asset.path.as_str(), model_rest_pose(asset)))
        .collect();
    for (clip_id, clip) in clips {
        if clip.transform_tracks.is_empty() {
            continue;
        }
        let Some(models) = candidate_models.get(clip_id) else {
            continue;
        };
        let mut best: Option<(&str, &BTreeMap<String, RestTransform>, usize)> = None;
        for model in models {
            let Some(pose) = rest_poses.get(model.as_str()) else {
                continue;
            };
            let score = clip
                .transform_tracks
                .iter()
                .filter(|track| pose.contains_key(track.target_path.as_str()))
                .count();
            if score == 0 {
                continue;
            }
            if best.is_none_or(|(best_path, _, best_score)| {
                score > best_score || (score == best_score && model.as_str() < best_path)
            }) {
                best = Some((model.as_str(), pose, score));
            }
        }
        let Some((model, pose, _)) = best else {
            continue;
        };
        clip.rig_asset_path = Some(glb_asset_path(model));
        for track in &mut clip.transform_tracks {
            let Some(rest) = pose.get(track.target_path.as_str()) else {
                continue;
            };
            track.reference_translation = Some(rest.translation);
            track.reference_rotation = Some(rest.rotation);
            track.reference_scale = Some(rest.scale);
        }
    }
}

fn model_rest_pose(asset: &UnityAsset) -> BTreeMap<String, RestTransform> {
    asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .filter(|component| {
            component
                .type_name
                .as_deref()
                .is_some_and(|name| name.starts_with("UnityEngine.Transform"))
        })
        .filter_map(|component| {
            let target_path = if component.hierarchy_path.is_empty() {
                "$root".to_owned()
            } else {
                component.hierarchy_path.clone()
            };
            Some((
                target_path,
                RestTransform {
                    translation: field_array(&component.fields, "localPosition")?,
                    rotation: field_array(&component.fields, "localRotation")?,
                    scale: field_array(&component.fields, "localScale")?,
                },
            ))
        })
        .collect()
}

fn field_array<const N: usize>(fields: &[UnityField], path: &str) -> Option<[f32; N]> {
    let value = &fields.iter().find(|field| field.path == path)?.value;
    let keys = ["x", "y", "z", "w"];
    keys[..N]
        .iter()
        .map(|key| value.get(key)?.to_string().parse::<f32>().ok())
        .collect::<Option<Vec<_>>>()?
        .try_into()
        .ok()
}

fn convert_prefab_materials(
    export: &UnityExport,
    assets_by_path: &BTreeMap<&str, &UnityAsset>,
    materials: &BTreeMap<StableId, MaterialDef>,
) -> BTreeMap<String, Vec<StableId>> {
    let material_paths: BTreeMap<_, _> = materials
        .iter()
        .map(|(id, material)| (material.source_path.as_str(), id))
        .collect();
    let mut bindings = BTreeMap::new();
    for prefab in export.assets.iter().filter(|asset| asset.kind == "prefab") {
        let mut dependency_paths = BTreeSet::new();
        collect_prefab_dependencies(
            prefab,
            assets_by_path,
            &mut BTreeSet::new(),
            &mut dependency_paths,
        );
        let material_ids: Vec<_> = dependency_paths
            .iter()
            .filter_map(|path| material_paths.get(path.as_str()).copied().cloned())
            .collect();
        if !material_ids.is_empty() {
            bindings.insert(prefab.guid.clone(), material_ids);
        }
    }
    bindings
}

fn convert_model_materials(
    export: &UnityExport,
    unity_root: &Path,
    out_dir: &Path,
    materials: &BTreeMap<StableId, MaterialDef>,
) -> BTreeMap<String, BTreeMap<String, StableId>> {
    let material_guids: BTreeMap<_, _> = materials
        .iter()
        .map(|(id, material)| (material.source_guid.as_str(), id))
        .collect();
    let mut bindings = BTreeMap::new();
    for model in export.assets.iter().filter(|asset| asset.kind == "model") {
        let meta_path = unity_root.join(format!("{}.meta", model.path));
        let Ok(contents) = fs::read_to_string(&meta_path) else {
            continue;
        };
        let names: BTreeMap<_, _> = model
            .importer_fields
            .iter()
            .filter_map(|field| {
                let index = array_index(&field.path, "m_Materials.Array.data[")?;
                field
                    .path
                    .ends_with("].name")
                    .then(|| field.value.as_str().map(|name| (index, name.to_owned())))
                    .flatten()
            })
            .collect();
        let mut mapped = parse_model_material_remaps(&contents, &material_guids);
        let converted_names = out_dir
            .parent()
            .map(|assets_root| assets_root.join(glb_asset_path(&model.path)))
            .map_or_else(BTreeSet::new, |path| glb_material_names(&path));
        if !converted_names.is_empty() {
            mapped.retain(|name, _| converted_names.contains(name));
        }
        // Models without explicit external remaps still retain their importer names;
        // match these to the effective renderer material names where Unity resolved one.
        for (index, name) in names {
            if mapped.contains_key(&name) {
                continue;
            }
            let material = model
                .game_object
                .as_ref()
                .into_iter()
                .flat_map(|game_object| &game_object.components)
                .filter(|component| is_renderer_component(component))
                .flat_map(|component| renderer_material_references(component).into_iter())
                .nth(index)
                .and_then(|reference| reference.guid)
                .and_then(|guid| material_guids.get(guid.as_str()));
            if let Some(material) = material {
                mapped.insert(name, (*material).clone());
            }
        }
        if !mapped.is_empty() {
            bindings.insert(model.path.clone(), mapped);
        }
    }
    bindings
}

fn parse_model_material_remaps(
    contents: &str,
    material_guids: &BTreeMap<&str, &StableId>,
) -> BTreeMap<String, StableId> {
    let mut mapped = BTreeMap::new();
    let mut active_name = None;
    for line in contents.lines() {
        if let Some(name) = line.trim().strip_prefix("name:") {
            active_name = Some(name.trim().to_owned());
        } else if line.trim_start().starts_with("second:")
            && let Some((name, guid)) = active_name.take().zip(reference_guid(line))
            && let Some(material) = material_guids.get(guid)
        {
            mapped.insert(name, (*material).clone());
        }
    }
    mapped
}

fn glb_material_names(path: &Path) -> BTreeSet<String> {
    let Ok(bytes) = fs::read(path) else {
        return BTreeSet::new();
    };
    let Some(length) = bytes
        .get(12..16)
        .and_then(|bytes| <[u8; 4]>::try_from(bytes).ok())
        .map(u32::from_le_bytes)
        .and_then(|length| usize::try_from(length).ok())
    else {
        return BTreeSet::new();
    };
    let Some(document) = bytes.get(20..20_usize.saturating_add(length)) else {
        return BTreeSet::new();
    };
    serde_json::from_slice::<Value>(document)
        .ok()
        .and_then(|document| document.get("materials").and_then(Value::as_array).cloned())
        .into_iter()
        .flatten()
        .filter_map(|material| {
            material
                .get("name")
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
        .collect()
}

fn convert_prefab_renderer_materials(
    export: &UnityExport,
    assets_by_path: &BTreeMap<&str, &UnityAsset>,
    materials: &BTreeMap<StableId, MaterialDef>,
    model_materials: &BTreeMap<String, BTreeMap<String, StableId>>,
) -> BTreeMap<String, Vec<RendererMaterialBinding>> {
    let material_guids: BTreeMap<_, _> = materials
        .iter()
        .map(|(id, material)| (material.source_guid.as_str(), id))
        .collect();
    let mut bindings = BTreeMap::new();
    for prefab in export.assets.iter().filter(|asset| asset.kind == "prefab") {
        let embedded_by_material = prefab
            .dependencies
            .iter()
            .filter_map(|dependency| dependency.path.as_deref())
            .filter_map(|path| assets_by_path.get(path).copied())
            .filter(|asset| asset.kind == "model")
            .filter_map(|model| model_materials.get(&model.path))
            .flat_map(|model| model.iter())
            .fold(
                BTreeMap::<StableId, Vec<String>>::new(),
                |mut result, (name, id)| {
                    result.entry(id.clone()).or_default().push(name.clone());
                    result
                },
            );
        let mut renderers = BTreeMap::<String, BTreeMap<String, StableId>>::new();
        for component in prefab
            .game_object
            .as_ref()
            .into_iter()
            .flat_map(|game_object| &game_object.components)
            .filter(|component| is_renderer_component(component))
        {
            let references = renderer_material_references(component);
            let mut converted = BTreeMap::new();
            for (slot, reference) in references.iter().enumerate() {
                let Some(material) = reference
                    .guid
                    .as_deref()
                    .and_then(|guid| material_guids.get(guid))
                    .copied()
                else {
                    continue;
                };
                let embedded_name = embedded_by_material
                    .get(material)
                    .and_then(|names| names.get(slot).or_else(|| names.first()))
                    .cloned()
                    .or_else(|| reference.name.clone())
                    .unwrap_or_else(|| format!("slot:{slot}"));
                converted.insert(embedded_name, material.clone());
            }
            if !converted.is_empty() {
                renderers
                    .entry(component.hierarchy_path.clone())
                    .or_default()
                    .extend(converted);
            }
        }
        if !renderers.is_empty() {
            bindings.insert(
                prefab.guid.clone(),
                renderers
                    .into_iter()
                    .map(|(target_path, materials)| RendererMaterialBinding {
                        target_path,
                        materials,
                    })
                    .collect(),
            );
        }
    }
    bindings
}

fn is_renderer_component(component: &UnityComponent) -> bool {
    component.type_name.as_deref().is_some_and(|name| {
        name.starts_with("UnityEngine.MeshRenderer,")
            || name.starts_with("UnityEngine.SkinnedMeshRenderer,")
    })
}

fn renderer_material_references(component: &UnityComponent) -> Vec<UnityReference> {
    component
        .fields
        .iter()
        .find(|field| field.path == "sharedMaterials")
        .and_then(|field| serde_json::from_value(field.value.clone()).ok())
        .unwrap_or_default()
}

fn collect_prefab_dependencies(
    asset: &UnityAsset,
    assets_by_path: &BTreeMap<&str, &UnityAsset>,
    visited: &mut BTreeSet<String>,
    dependencies: &mut BTreeSet<String>,
) {
    if !visited.insert(asset.path.clone()) {
        return;
    }
    for dependency in &asset.dependencies {
        let Some(path) = dependency.path.as_deref() else {
            continue;
        };
        dependencies.insert(path.to_owned());
        if let Some(dependency_asset) = assets_by_path.get(path)
            && matches!(dependency_asset.kind.as_str(), "prefab" | "model")
        {
            collect_prefab_dependencies(dependency_asset, assets_by_path, visited, dependencies);
        }
    }
}

fn parse_yaml_documents(contents: &str) -> Result<Vec<YamlDocument>> {
    let mut documents = Vec::new();
    let mut current: Option<YamlDocument> = None;
    for line in contents.lines() {
        if let Some(header) = line.strip_prefix("--- !u!") {
            if let Some(document) = current.take() {
                documents.push(document);
            }
            let (class_id, file_id) = header
                .split_once(" &")
                .with_context(|| format!("invalid Unity YAML header {line}"))?;
            current = Some(YamlDocument {
                class_id: class_id.parse()?,
                file_id: file_id
                    .split_whitespace()
                    .next()
                    .context("Unity YAML header has no file ID")?
                    .parse()?,
                lines: Vec::new(),
            });
        } else if let Some(document) = &mut current {
            document.lines.push(line.to_owned());
        }
    }
    if let Some(document) = current {
        documents.push(document);
    }
    Ok(documents)
}

fn parse_state_motions(
    state: &YamlDocument,
    blend_trees: &BTreeMap<i64, ParsedBlendTree>,
) -> ParsedBlendTree {
    let Some(line) = state
        .lines
        .iter()
        .find(|line| line.trim_start().starts_with("m_Motion:"))
    else {
        return ParsedBlendTree {
            parameter: None,
            motions: Vec::new(),
        };
    };
    if let Some(guid) = reference_guid(line) {
        return ParsedBlendTree {
            parameter: None,
            motions: vec![ParsedMotion {
                guid: guid.to_owned(),
                local_id: inline_file_id(line).unwrap_or_default(),
                threshold: None,
            }],
        };
    }
    inline_file_id(line)
        .and_then(|file_id| blend_trees.get(&file_id).cloned())
        .unwrap_or(ParsedBlendTree {
            parameter: None,
            motions: Vec::new(),
        })
}

fn parse_blend_tree(lines: &[String]) -> ParsedBlendTree {
    let mut motions = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if !line.trim_start().starts_with("m_Motion:") {
            continue;
        }
        let Some(guid) = reference_guid(line) else {
            continue;
        };
        let threshold = lines
            .iter()
            .skip(index + 1)
            .take(4)
            .find_map(|line| line.trim().strip_prefix("m_Threshold: "))
            .and_then(|value| value.parse().ok());
        motions.push(ParsedMotion {
            guid: guid.to_owned(),
            local_id: inline_file_id(line).unwrap_or_default(),
            threshold,
        });
    }
    ParsedBlendTree {
        parameter: scalar(lines, "m_BlendParameter:").map(str::to_owned),
        motions,
    }
}

fn parse_conditions(lines: &[String]) -> Result<Vec<AnimationConditionDef>> {
    let mut conditions = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let Some(mode) = line
            .trim()
            .strip_prefix("- m_ConditionMode: ")
            .and_then(|value| value.parse::<u8>().ok())
        else {
            continue;
        };
        let parameter = lines
            .get(index + 1)
            .and_then(|line| line.trim().strip_prefix("m_ConditionEvent: "))
            .unwrap_or_default()
            .to_owned();
        let threshold = lines
            .get(index + 2)
            .and_then(|line| line.trim().strip_prefix("m_EventTreshold: "))
            .and_then(|value| value.parse().ok())
            .unwrap_or(0.0);
        let mode = AnimationConditionMode::try_from(mode)
            .map_err(|mode| anyhow::anyhow!("unsupported Unity animation condition mode {mode}"))?;
        conditions.push(AnimationConditionDef {
            parameter,
            mode,
            threshold,
        });
    }
    Ok(conditions)
}

fn parse_parameters(lines: &[String]) -> Vec<AnimationParameterDef> {
    let start = lines
        .iter()
        .position(|line| line.trim() == "m_AnimatorParameters:");
    let end = lines
        .iter()
        .position(|line| line.trim().starts_with("m_AnimatorLayers:"));
    let Some((start, end)) = start.zip(end) else {
        return Vec::new();
    };
    let mut parameters = Vec::new();
    let mut index = start + 1;
    while index < end {
        let Some(name) = lines[index].trim().strip_prefix("- m_Name: ") else {
            index += 1;
            continue;
        };
        let window = &lines[index + 1..end.min(index + 7)];
        let unity_type = scalar_u8(window, "m_Type:").unwrap_or(1);
        let kind = match unity_type {
            3 => AnimationParameterKind::Integer,
            4 => AnimationParameterKind::Boolean,
            9 => AnimationParameterKind::Trigger,
            _ => AnimationParameterKind::Float,
        };
        parameters.push(AnimationParameterDef {
            name: name.to_owned(),
            kind,
            default_float: scalar_f32(window, "m_DefaultFloat:").unwrap_or(0.0),
            default_integer: scalar_i32(window, "m_DefaultInt:").unwrap_or(0),
            default_boolean: scalar_bool(window, "m_DefaultBool:").unwrap_or(false),
            inferred: false,
        });
        index += 1;
    }
    parameters
}

fn infer_missing_parameters(
    states: &BTreeMap<StableId, AnimationStateDef>,
    transitions: &[AnimationTransitionDef],
    parameters: &mut Vec<AnimationParameterDef>,
) {
    let mut known: BTreeSet<_> = parameters
        .iter()
        .map(|parameter| parameter.name.clone())
        .collect();
    let mut inferred = BTreeMap::<String, AnimationParameterKind>::new();
    for parameter in states
        .values()
        .filter_map(|state| state.blend_parameter.as_ref())
    {
        if !known.contains(parameter) {
            inferred.insert(parameter.clone(), AnimationParameterKind::Float);
        }
    }
    for condition in transitions
        .iter()
        .flat_map(|transition| &transition.conditions)
    {
        if known.contains(&condition.parameter) {
            continue;
        }
        let kind = match condition.mode {
            AnimationConditionMode::If | AnimationConditionMode::IfNot => {
                AnimationParameterKind::Boolean
            }
            AnimationConditionMode::Equals | AnimationConditionMode::NotEqual => {
                AnimationParameterKind::Integer
            }
            AnimationConditionMode::Greater | AnimationConditionMode::Less => {
                AnimationParameterKind::Float
            }
        };
        inferred.entry(condition.parameter.clone()).or_insert(kind);
    }
    for (name, kind) in inferred {
        known.insert(name.clone());
        parameters.push(AnimationParameterDef {
            name,
            kind,
            default_float: 0.0,
            default_integer: 0,
            default_boolean: false,
            inferred: true,
        });
    }
}

fn parse_reference_list(lines: &[String], header: &str) -> Vec<i64> {
    let Some(start) = lines.iter().position(|line| line.trim() == header) else {
        return Vec::new();
    };
    lines
        .iter()
        .skip(start + 1)
        .take_while(|line| line.starts_with("  - "))
        .filter_map(|line| inline_file_id(line))
        .collect()
}

fn parse_child_references(lines: &[String], header: &str, key: &str) -> Vec<i64> {
    let Some(start) = lines.iter().position(|line| line.trim() == header) else {
        return Vec::new();
    };
    lines
        .iter()
        .skip(start + 1)
        .take_while(|line| !line.starts_with("  m_") || line.trim_start().starts_with(key))
        .filter(|line| line.trim_start().starts_with(key))
        .filter_map(|line| inline_file_id(line))
        .collect()
}

fn parse_state_machine_transition_sources(lines: &[String]) -> Vec<(i64, i64)> {
    let Some(start) = lines
        .iter()
        .position(|line| line.trim() == "m_StateMachineTransitions:")
    else {
        return Vec::new();
    };
    let mut source = None;
    let mut transitions = Vec::new();
    for line in lines
        .iter()
        .skip(start + 1)
        .take_while(|line| !line.starts_with("  m_"))
    {
        if line.trim_start().starts_with("- first:") {
            source = inline_file_id(line);
        } else if line.trim_start().starts_with("- {fileID:")
            && let Some((transition, source)) = inline_file_id(line).zip(source)
        {
            transitions.push((transition, source));
        }
    }
    transitions
}

fn parse_layers(
    lines: &[String],
    state_machine_ids: &BTreeMap<i64, StableId>,
) -> Vec<AnimationLayerDef> {
    let Some(start) = lines
        .iter()
        .position(|line| line.trim() == "m_AnimatorLayers:")
    else {
        return Vec::new();
    };
    let mut layers = Vec::new();
    let mut index = start + 1;
    while index < lines.len() {
        if !lines[index]
            .trim_start()
            .starts_with("- serializedVersion:")
        {
            index += 1;
            continue;
        }
        let end = lines[index + 1..]
            .iter()
            .position(|line| line.trim_start().starts_with("- serializedVersion:"))
            .map_or(lines.len(), |offset| index + 1 + offset);
        let block = &lines[index..end];
        let Some(machine) = reference_id(block, "m_StateMachine:")
            .and_then(|file| state_machine_ids.get(&file).cloned())
        else {
            index = end;
            continue;
        };
        layers.push(AnimationLayerDef {
            display_name: scalar(block, "m_Name:").unwrap_or("Unnamed").to_owned(),
            state_machine: machine,
            blend_mode: match scalar_i32(block, "m_BlendingMode:").unwrap_or(0) {
                1 => AnimationLayerBlendMode::Additive,
                _ => AnimationLayerBlendMode::Override,
            },
            default_weight: scalar_f32(block, "m_DefaultWeight:").unwrap_or(1.0),
            avatar_mask: block.iter().find_map(|line| {
                line.trim_start()
                    .starts_with("m_Mask:")
                    .then(|| reference_guid(line).and_then(|guid| avatar_mask_id(guid).ok()))
                    .flatten()
            }),
        });
        index = end;
    }
    layers
}

fn named_values(asset: &UnityAsset, prefix: &str) -> BTreeMap<String, Value> {
    let mut names = BTreeMap::<usize, String>::new();
    let mut values = BTreeMap::<usize, Value>::new();
    for field in &asset.serialized_fields {
        let Some(index) = array_index(&field.path, prefix) else {
            continue;
        };
        let suffix = field.path.split_once(']').map(|(_, suffix)| suffix);
        match suffix {
            Some(".first") => {
                if let Some(name) = field.value.as_str() {
                    names.insert(index, name.to_owned());
                }
            }
            Some(".second") => {
                values.insert(index, field.value.clone());
            }
            Some(".second.m_Texture") => {
                values
                    .entry(index)
                    .or_insert_with(|| Value::Object(serde_json::Map::default()))
                    .as_object_mut()
                    .expect("generated object")
                    .insert("m_Texture".to_owned(), field.value.clone());
            }
            Some(".second.m_Scale") => {
                values
                    .entry(index)
                    .or_insert_with(|| Value::Object(serde_json::Map::default()))
                    .as_object_mut()
                    .expect("generated object")
                    .insert("m_Scale".to_owned(), field.value.clone());
            }
            Some(".second.m_Offset") => {
                values
                    .entry(index)
                    .or_insert_with(|| Value::Object(serde_json::Map::default()))
                    .as_object_mut()
                    .expect("generated object")
                    .insert("m_Offset".to_owned(), field.value.clone());
            }
            _ => {}
        }
    }
    names
        .into_iter()
        .filter_map(|(index, name)| values.remove(&index).map(|value| (name, value)))
        .collect()
}

fn array_index(path: &str, prefix: &str) -> Option<usize> {
    path.strip_prefix(prefix)?.split_once(']')?.0.parse().ok()
}

#[allow(clippy::cast_possible_truncation)]
fn color_value(value: Option<&Value>, fallback: [f32; 4]) -> [f32; 4] {
    let Some(value) = value.and_then(Value::as_object) else {
        return fallback;
    };
    let component = |name: &str, fallback: f32| {
        value
            .get(name)
            .and_then(Value::as_f64)
            .map_or(fallback, |value| value as f32)
    };
    [
        component("r", fallback[0]),
        component("g", fallback[1]),
        component("b", fallback[2]),
        component("a", fallback[3]),
    ]
}

#[allow(clippy::cast_possible_truncation)]
fn vec2_value(value: Option<&Value>, fallback: [f32; 2]) -> [f32; 2] {
    let Some(value) = value.and_then(Value::as_object) else {
        return fallback;
    };
    let component = |name: &str, fallback: f32| {
        value
            .get(name)
            .and_then(Value::as_f64)
            .map_or(fallback, |value| value as f32)
    };
    [component("x", fallback[0]), component("y", fallback[1])]
}

#[allow(clippy::cast_possible_truncation)]
fn float_value(values: &BTreeMap<String, Value>, names: &[&str]) -> f32 {
    names
        .iter()
        .find_map(|name| values.get(*name).and_then(Value::as_f64))
        .map_or(0.0, |value| value as f32)
}

fn reference_path(value: &Value) -> Option<&str> {
    value
        .as_object()
        .and_then(|reference| reference.get("Path"))
        .and_then(Value::as_str)
}

fn scalar<'a>(lines: &'a [String], key: &str) -> Option<&'a str> {
    lines
        .iter()
        .find_map(|line| line.trim().strip_prefix(key).map(str::trim))
}

fn scalar_f32(lines: &[String], key: &str) -> Option<f32> {
    scalar(lines, key)?.parse().ok()
}

fn scalar_i32(lines: &[String], key: &str) -> Option<i32> {
    scalar(lines, key)?.parse().ok()
}

fn scalar_u8(lines: &[String], key: &str) -> Option<u8> {
    scalar(lines, key)?.parse().ok()
}

fn scalar_bool(lines: &[String], key: &str) -> Option<bool> {
    match scalar(lines, key)? {
        "0" | "false" | "False" => Some(false),
        "1" | "true" | "True" => Some(true),
        _ => None,
    }
}

fn reference_id(lines: &[String], key: &str) -> Option<i64> {
    let line = lines
        .iter()
        .find(|line| line.trim_start().starts_with(key))?;
    inline_file_id(line)
}

fn inline_file_id(line: &str) -> Option<i64> {
    let value = line.split("fileID: ").nth(1)?;
    value
        .chars()
        .take_while(|character| character.is_ascii_digit() || *character == '-')
        .collect::<String>()
        .parse()
        .ok()
}

fn field_value<'a>(fields: &'a [UnityField], path: &str) -> Option<&'a Value> {
    fields
        .iter()
        .find(|field| field.path == path)
        .map(|field| &field.value)
}

fn field_str<'a>(fields: &'a [UnityField], path: &str) -> Option<&'a str> {
    field_value(fields, path)?.as_str()
}

fn field_u64(fields: &[UnityField], path: &str) -> Option<u64> {
    field_value(fields, path)?.as_u64()
}

fn reference_guid(line: &str) -> Option<&str> {
    let value = line.split("guid: ").nth(1)?;
    let guid = value.split(',').next()?.trim();
    (guid.len() == 32 && guid.bytes().all(|byte| byte.is_ascii_hexdigit())).then_some(guid)
}

#[allow(clippy::cast_possible_truncation)]
fn field_f32(fields: &[UnityField], path: &str) -> Option<f32> {
    fields
        .iter()
        .find(|field| field.path == path)?
        .value
        .as_f64()
        .map(|value| value as f32)
}

fn field_bool(fields: &[UnityField], path: &str) -> Option<bool> {
    fields
        .iter()
        .find(|field| field.path == path)?
        .value
        .as_bool()
}

fn model_has_animation(asset: &UnityAsset) -> bool {
    asset.importer_fields.iter().any(|field| {
        field.path == "m_ClipAnimations.Array.size"
            && field.value.as_u64().is_some_and(|count| count > 0)
    })
}

fn is_texture_path(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "png" | "tga" | "jpg" | "jpeg"
            )
        })
}

fn is_avatar_mask_path(path: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("mask"))
}

fn glb_asset_path(source_model: &str) -> String {
    let relative = source_model.strip_prefix("Assets/").unwrap_or(source_model);
    let stem = relative
        .rsplit_once('.')
        .map_or(relative, |(stem, _extension)| stem);
    format!("migrated/models/{stem}.glb")
}

fn glb_animation_names(path: &Path) -> Result<Vec<String>> {
    let payload = fs::read(path)
        .with_context(|| format!("failed to read converted model {}", path.display()))?;
    if payload.len() < 20 || &payload[..4] != b"glTF" {
        bail!("converted model {} is not a GLB file", path.display());
    }
    let json_length = usize::try_from(u32::from_le_bytes(
        payload[12..16].try_into().expect("four bytes"),
    ))
    .context("GLB JSON length does not fit the current platform")?;
    if u32::from_le_bytes(payload[16..20].try_into().expect("four bytes")) != 0x4E4F_534A
        || 20 + json_length > payload.len()
    {
        bail!(
            "converted model {} has an invalid JSON chunk",
            path.display()
        );
    }
    let document: Value = serde_json::from_slice(&payload[20..20 + json_length])
        .with_context(|| format!("failed to parse converted model {}", path.display()))?;
    Ok(document
        .get("animations")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|animation| animation.get("name").and_then(Value::as_str))
        .map(str::to_owned)
        .collect())
}

fn animation_take_name(name: &str) -> &str {
    let mut parts = name.split('|');
    let first = parts.next().unwrap_or(name);
    parts.next().unwrap_or(first)
}

fn texture_id(guid: &str) -> Result<StableId> {
    StableId::new(format!("texture:{guid}")).map_err(Into::into)
}

fn material_id(guid: &str) -> Result<StableId> {
    StableId::new(format!("material:{guid}")).map_err(Into::into)
}

fn clip_id(guid: &str) -> Result<StableId> {
    StableId::new(format!("clip:{guid}")).map_err(Into::into)
}

fn embedded_clip_id(guid: &str, local_id: i64) -> Result<StableId> {
    StableId::new(format!("clip:{guid}:{local_id}")).map_err(Into::into)
}

fn controller_id(guid: &str) -> Result<StableId> {
    StableId::new(format!("controller:{guid}")).map_err(Into::into)
}

fn avatar_mask_id(guid: &str) -> Result<StableId> {
    StableId::new(format!("avatar_mask:{guid}")).map_err(Into::into)
}

fn animation_state_id(controller_guid: &str, file_id: i64) -> Result<StableId> {
    StableId::new(format!("animation_state:{controller_guid}:{file_id}")).map_err(Into::into)
}

fn animation_state_machine_id(controller_guid: &str, file_id: i64) -> Result<StableId> {
    StableId::new(format!(
        "animation_state_machine:{controller_guid}:{file_id}"
    ))
    .map_err(Into::into)
}

fn normalized_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn write_ron_atomic<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let temporary = path.with_extension("ron.tmp");
    let encoded = ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::default())?;
    fs::write(&temporary, encoded)
        .with_context(|| format!("failed to write temporary file {}", temporary.display()))?;
    if path.exists() {
        fs::remove_file(path)
            .with_context(|| format!("failed to replace generated file {}", path.display()))?;
    }
    fs::rename(&temporary, path)
        .with_context(|| format!("failed to publish generated file {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_unity_sprite_border_to_bevy_edge_order() {
        let directory = tempfile::tempdir().unwrap();
        let texture = directory.path().join("panel.png");
        fs::write(&texture, []).unwrap();
        fs::write(
            directory.path().join("panel.png.meta"),
            "TextureImporter:\n  spriteBorder: {x: 11, y: 44, z: 22, w: 33}\n",
        )
        .unwrap();
        assert_eq!(
            unity_sprite_border(&texture).unwrap(),
            Some([11.0, 22.0, 33.0, 44.0])
        );
    }

    #[test]
    fn omits_zero_unity_sprite_borders() {
        let directory = tempfile::tempdir().unwrap();
        let texture = directory.path().join("plain.png");
        fs::write(&texture, []).unwrap();
        fs::write(
            directory.path().join("plain.png.meta"),
            "TextureImporter:\n  spriteBorder: {x: 0, y: 0, z: 0, w: 0}\n",
        )
        .unwrap();
        assert_eq!(unity_sprite_border(&texture).unwrap(), None);
    }

    #[test]
    fn copies_shipping_ui_fonts_into_the_bevy_asset_tree() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let directory = tempfile::tempdir().unwrap();
        let content = directory.path().join("assets/content");
        let (count, bytes) = convert_shipping_fonts(&root, &content).unwrap();
        assert_eq!(count, 2);
        assert!(bytes > 200_000);
        assert!(
            directory
                .path()
                .join("assets/migrated/fonts/Rubik-Bold.ttf")
                .is_file()
        );
        assert!(
            directory
                .path()
                .join("assets/migrated/fonts/Luckiest Guy.ttf")
                .is_file()
        );
    }

    fn fixture_asset(guid: &str, path: &str, kind: &str, name: &str) -> UnityAsset {
        UnityAsset {
            guid: guid.to_owned(),
            path: path.to_owned(),
            kind: kind.to_owned(),
            name: name.to_owned(),
            importer_fields: Vec::new(),
            serialized_fields: Vec::new(),
            dependencies: Vec::new(),
            game_object: None,
            scene: None,
        }
    }

    fn tracked_unity_asset(root: &Path, path: &str, kind: &str) -> UnityAsset {
        let meta_path = root.join(format!("{path}.meta"));
        let meta = fs::read_to_string(&meta_path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", meta_path.display()));
        let guid = meta
            .lines()
            .find_map(|line| line.strip_prefix("guid: "))
            .unwrap_or_else(|| panic!("{} has no Unity GUID", meta_path.display()));
        let name = Path::new(path)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_else(|| panic!("{path} has no portable file stem"));
        fixture_asset(guid, path, kind, name)
    }

    fn tracked_unity_export(
        root: &Path,
        assets: impl IntoIterator<Item = (&'static str, &'static str)>,
    ) -> UnityExport {
        UnityExport {
            schema_version: 1,
            assets: assets
                .into_iter()
                .map(|(path, kind)| tracked_unity_asset(root, path, kind))
                .collect(),
        }
    }

    #[test]
    fn parses_authored_volume_parameters() {
        let documents = parse_yaml_documents(
            r"%YAML 1.1
--- !u!114 &1
MonoBehaviour:
  m_Name: Bloom
  active: 1
  threshold:
    m_OverrideState: 1
    m_Value: 1.5
  intensity:
    m_OverrideState: 1
    m_Value: 1
  tint:
    m_OverrideState: 0
    m_Value: {r: 0.25, g: 0.5, b: 1, a: 1}
",
        )
        .unwrap();
        let lines = &documents[0].lines;
        assert_eq!(volume_parameter_f32(lines, "threshold"), Some(1.5));
        assert_eq!(volume_parameter_f32(lines, "intensity"), Some(1.0));
        assert!(
            inline_color(volume_parameter(lines, "tint").unwrap(), [0.0; 4])
                .into_iter()
                .zip([0.25, 0.5, 1.0, 1.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
    }

    #[test]
    fn parses_authored_fireworks_graph_parameters() {
        let contents = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../Assets/VFX/vfx_fireworks.vfx"),
        )
        .unwrap();
        let documents = parse_yaml_documents(&contents).unwrap();
        assert_eq!(
            vfx_named_scalar_values(&documents, "Rate"),
            [16.0, 30.0, 1_000.0]
        );
        assert!(
            vfx_attribute_range(&documents, "lifetime", 8)
                .unwrap()
                .into_iter()
                .zip([0.75, 1.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert_eq!(
            vfx_attribute_vector_range(&documents, "velocity", 8).unwrap(),
            ([-1.0, 12.0, -1.0], [1.0, 16.0, 1.0])
        );
        assert!(
            vfx_named_vec2(&documents, "Count")
                .unwrap()
                .into_iter()
                .zip([1.0, 3.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert!(
            vfx_named_vec2(&documents, "Delay")
                .unwrap()
                .into_iter()
                .zip([1.0, 2.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert!(
            (vfx_parameter_scalar(&contents, "name: SparksSpeed").unwrap() - 2.0).abs()
                < f32::EPSILON
        );
        let colors = vfx_gradient_colors(&contents, "name: FireworkColour").unwrap();
        assert_eq!(colors.len(), 8);
        assert!((colors[2][0] - 42.722_507).abs() < 0.000_01);
    }

    #[test]
    fn parses_authored_chimney_particle_sections() {
        let contents = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../Assets/Prefabs/VFX/Environment/VFX_Chimney_Smoke.prefab"),
        )
        .unwrap();
        let documents = parse_yaml_documents(&contents).unwrap();
        let particle = documents
            .iter()
            .find(|document| document.class_id == 198)
            .unwrap();
        let initial = yaml_section(&particle.lines, "InitialModule:").unwrap();
        let lifetime = yaml_section(initial, "startLifetime:").unwrap();
        let shape = yaml_section(&particle.lines, "ShapeModule:").unwrap();
        let emission = yaml_section(&particle.lines, "EmissionModule:").unwrap();
        let rate = yaml_section(emission, "rateOverTime:").unwrap();
        let size = yaml_section(&particle.lines, "SizeModule:").unwrap();
        let curve = yaml_section(size, "curve:").unwrap();
        let max_curve = yaml_section(curve, "maxCurve:").unwrap();
        assert_eq!(scalar_f32(&particle.lines, "lengthInSec:"), Some(5.0));
        assert_eq!(scalar_f32(lifetime, "scalar:"), Some(5.0));
        assert_eq!(scalar_f32(rate, "scalar:"), Some(5.0));
        assert_eq!(scalar_f32(shape, "angle:"), Some(6.1));
        assert_eq!(yaml_keyframe_values(max_curve), [0.513_157_84, 1.0]);
        assert_eq!(
            hierarchy_age("Age02_Forge/Base/VFX_Chimney_Smoke").unwrap(),
            2
        );
    }

    #[test]
    fn parses_authored_raining_fish_particle_sections() {
        let contents = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../Assets/Prefabs/VFX/Environment/VFX_RainingFish.prefab"),
        )
        .unwrap();
        let documents = parse_yaml_documents(&contents).unwrap();
        let particle = documents
            .iter()
            .find(|document| document.class_id == 198)
            .unwrap();
        let initial = yaml_section(&particle.lines, "InitialModule:").unwrap();
        let lifetime = yaml_section(initial, "startLifetime:").unwrap();
        let size = yaml_section(initial, "startSize:").unwrap();
        let shape = yaml_section(&particle.lines, "ShapeModule:").unwrap();
        let emission = yaml_section(&particle.lines, "EmissionModule:").unwrap();
        let rate = yaml_section(emission, "rateOverTime:").unwrap();
        let size_module = yaml_section(&particle.lines, "SizeModule:").unwrap();
        let curve = yaml_section(size_module, "curve:").unwrap();
        let max_curve = yaml_section(curve, "maxCurve:").unwrap();
        let keys = yaml_float_keyframes(max_curve).unwrap();
        assert_eq!(scalar_f32(&particle.lines, "lengthInSec:"), Some(15.0));
        assert_eq!(scalar_f32(lifetime, "scalar:"), Some(15.0));
        assert_eq!(scalar_f32(size, "minScalar:"), Some(0.2));
        assert_eq!(scalar_f32(size, "scalar:"), Some(1.0));
        assert_eq!(scalar_f32(rate, "scalar:"), Some(500.0));
        assert_eq!(scalar_f32(initial, "maxNumParticles:"), Some(5_000.0));
        assert!(
            inline_vec3(scalar(shape, "m_Scale:").unwrap(), [0.0; 3])
                .into_iter()
                .zip([300.0, 300.0, 5.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert_eq!(keys.len(), 3);
        assert!((keys[1].time - 0.876_676_3).abs() < f32::EPSILON);
        assert!((keys[1].value - 0.860_819_2).abs() < f32::EPSILON);
        assert_eq!(keys.last().map(|key| key.value), Some(0.0));
    }

    #[test]
    fn converts_shipping_fish_schools_and_role_audio_references() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let mut export = tracked_unity_export(
            &root,
            [
                ("Assets/Prefabs/VFX/Environment/Fish.prefab", "prefab"),
                ("Assets/Models/Critters/Critter_Fish3.fbx", "model"),
                ("Assets/Scenes/Menu/Main_Menu_02.unity", "scene"),
                ("Assets/Scenes/Worlds/World_Town.unity", "scene"),
            ],
        );
        let role_root = root.join("Assets/Resources/ScriptableObjects/Roles");
        let mut role_assets = fs::read_dir(&role_root)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", role_root.display()))
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.file_stem()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with("RoleData_"))
                    && path
                        .extension()
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("asset"))
            })
            .map(|path| normalized_path(path.strip_prefix(&root).unwrap()))
            .map(|path| tracked_unity_asset(&root, &path, "scriptable_object"))
            .collect::<Vec<_>>();
        role_assets.sort_by(|left, right| left.path.cmp(&right.path));
        export.assets.extend(role_assets);
        let (effects, bindings) = convert_fish_schools(&export, &root).unwrap();
        let effect = effects.values().next().unwrap();
        assert_eq!(effects.len(), 1);
        assert_eq!(effect.max_particles, 2_000);
        assert!(
            effect
                .shape_scale
                .into_iter()
                .zip([300.0, 300.0, 5.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert!(
            effect
                .particle_local_position
                .into_iter()
                .zip([0.0, -2.0, 0.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert!((effect.noise_frequency - 0.22).abs() < f32::EPSILON);
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
        assert!((effect.noise_octave_multiplier - 1.0).abs() < f32::EPSILON);
        assert!((effect.noise_octave_scale - 2.0).abs() < f32::EPSILON);
        assert!(effect.align_to_velocity);
        assert!(effect.world_space && effect.prewarm);
        assert_eq!(bindings.values().map(Vec::len).sum::<usize>(), 2);
        assert_eq!(
            bindings["Assets/Scenes/Menu/Main_Menu_02.unity"][0].max_particles,
            800
        );
        let town = &bindings["Assets/Scenes/Worlds/World_Town.unity"];
        assert_eq!(town.len(), 1);
        assert_eq!(town[0].hierarchy_path, "Fish");
        assert!(
            town[0]
                .local_position
                .into_iter()
                .all(|value| value.abs() < f32::EPSILON)
        );
        assert!(
            town[0]
                .noise_strength
                .into_iter()
                .zip([10.0, 0.02, 10.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );

        let roles = convert_role_action_audio(&export, &root).unwrap();
        assert_eq!(roles.len(), 14);
        assert_eq!(
            roles
                .values()
                .map(|role| role.clip_guids.len())
                .sum::<usize>(),
            35
        );
        assert_eq!(
            roles[&StableId::new("role:miner").unwrap()]
                .clip_guids
                .len(),
            5
        );
    }

    #[test]
    fn converts_authored_healing_graphs_and_prefab_overrides() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let export = tracked_unity_export(
            &root,
            [
                (
                    "Assets/Prefabs/VFX/Player/VFX_Healing_Channeling.prefab",
                    "prefab",
                ),
                ("Assets/VFX/vfx_channeling.vfx", "visual_effect_graph"),
                ("Assets/Prefabs/VFX/Player/VFX_healing.prefab", "prefab"),
                ("Assets/VFX/vfx_healed.vfx", "visual_effect_graph"),
                ("Assets/Models/VFX/VFX_Plus.fbx", "model"),
                ("Assets/Sprites/VFX/Particle_02.png", "texture"),
            ],
        );
        let (channels, bursts) = convert_healing_vfx(&export, &root).unwrap();
        let channel = channels.values().next().unwrap();
        assert_eq!(channel.particle_capacity, 32);
        assert!((channel.emission_rate_per_second - 16.0).abs() < f32::EPSILON);
        assert!(
            channel
                .particle_lifetime_seconds
                .into_iter()
                .zip([1.0, 3.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert!((channel.exposed_size - 10.0).abs() < f32::EPSILON);
        assert_eq!(channel.size_over_lifetime.len(), 4);
        assert_eq!(channel.duration_seconds(), Some(5.0));
        assert!((channel.color.color_keys[0].color[1] - 16.948_38).abs() < 0.000_01);

        let burst = bursts.values().next().unwrap();
        assert!((burst.duration_seconds - 1.2).abs() < f32::EPSILON);
        assert_eq!((burst.plus_capacity, burst.plus_burst_count), (8, 100));
        assert!(
            burst
                .plus_lifetime_seconds
                .into_iter()
                .zip([0.5, 1.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert_eq!((burst.disc_capacity, burst.disc_burst_count), (128, 1));
        assert!((burst.disc_lifetime_seconds - 1.0).abs() < f32::EPSILON);
        assert_eq!(burst.plus_size_over_lifetime.len(), 3);
        assert_eq!(burst.disc_size_over_lifetime.len(), 2);
        assert_eq!(
            burst.plus_model_asset_path,
            "migrated/models/Models/VFX/VFX_Plus.glb"
        );
        assert_eq!(
            burst.disc_texture.as_str(),
            "texture:d66db54b89d0e934398d6e933169a6c5"
        );
    }

    #[test]
    fn prefab_binding_prefers_authored_animator_controller_and_rig() {
        let minotaur_model_path = "Assets/Models/Enemies/Minotaur.fbx";
        let goblin_model_path = "Assets/Models/Enemies/Goblin.fbx";
        let controller_path = "Assets/Animation Controllers/Minotaur.controller";
        let mut prefab = fixture_asset("prefab", "Assets/Enemy.prefab", "prefab", "Enemy");
        prefab.dependencies = vec![
            UnityReference {
                guid: None,
                path: Some(goblin_model_path.to_owned()),
                name: None,
            },
            UnityReference {
                guid: None,
                path: Some(minotaur_model_path.to_owned()),
                name: None,
            },
            UnityReference {
                guid: None,
                path: Some(controller_path.to_owned()),
                name: None,
            },
        ];
        prefab.game_object = Some(UnityGameObject {
            components: vec![UnityComponent {
                hierarchy_path: "Model_Minotaur/Minotaur".to_owned(),
                type_name: Some("UnityEngine.Animator, UnityEngine.AnimationModule".to_owned()),
                fields: vec![UnityField {
                    path: "runtimeAnimatorController".to_owned(),
                    value: serde_json::json!({"Path": controller_path}),
                }],
            }],
        });
        let goblin = fixture_asset("goblin", goblin_model_path, "model", "Goblin");
        let minotaur = fixture_asset("minotaur", minotaur_model_path, "model", "Minotaur");
        let export = UnityExport {
            schema_version: 1,
            assets: vec![prefab, goblin, minotaur],
        };
        let by_path = export
            .assets
            .iter()
            .map(|asset| (asset.path.as_str(), asset))
            .collect();
        let controller_id = StableId::new("controller:minotaur").unwrap();
        let controllers = BTreeMap::from([(
            controller_id.clone(),
            AnimationControllerDef {
                display_name: "Minotaur".to_owned(),
                source_guid: "controller".to_owned(),
                source_path: controller_path.to_owned(),
                parameters: Vec::new(),
                states: BTreeMap::new(),
                transitions: Vec::new(),
                state_machines: BTreeMap::new(),
                layers: Vec::new(),
                default_states: Vec::new(),
            },
        )]);
        let binding = &convert_prefab_bindings(&export, &by_path, &controllers)["prefab"];
        assert_eq!(binding.controller, controller_id);
        assert_eq!(
            binding.rig_scene.as_deref(),
            Some("migrated/models/Models/Enemies/Minotaur.glb")
        );
    }

    #[test]
    fn parses_unity_transform_curves_without_editor_types() {
        let yaml = r"%YAML 1.1
--- !u!74 &7400000
AnimationClip:
  m_RotationCurves:
  - curve:
      m_Curve:
      - serializedVersion: 3
        time: 0
        value: {x: 0, y: 0, z: 0, w: 1}
      - serializedVersion: 3
        time: 1
        value: {x: 0, y: 0.5, z: 0, w: 0.8660254}
    path: pelvis
  m_EulerCurves: []
  m_PositionCurves:
  - curve:
      m_Curve:
      - serializedVersion: 3
        time: 0
        value: {x: 0, y: 1, z: 0}
      - serializedVersion: 3
        time: 1
        value: {x: 1, y: 1, z: 0}
    path: pelvis
  m_ScaleCurves: []
  m_FloatCurves: []
";
        let tracks = parse_transform_tracks(yaml).unwrap();
        assert_eq!(tracks.len(), 1);
        assert_eq!(tracks[0].target_path, "pelvis");
        assert_eq!(tracks[0].rotation.len(), 2);
        assert_eq!(tracks[0].translation.len(), 2);
        assert!(tracks[0].scale.is_empty());
    }

    #[test]
    fn parses_property_curves_and_animation_events_without_unity_types() {
        let yaml = r"%YAML 1.1
AnimationClip:
  m_RotationCurves: []
  m_FloatCurves:
  - curve:
      serializedVersion: 2
      m_Curve:
      - serializedVersion: 3
        time: 0
        value: 1
        inSlope: Infinity
        outSlope: Infinity
        tangentMode: 103
        weightedMode: 0
        inWeight: 0
        outWeight: 0
      - serializedVersion: 3
        time: 2
        value: 0
        inSlope: Infinity
        outSlope: Infinity
        tangentMode: 103
        weightedMode: 0
        inWeight: 0
        outWeight: 0
    attribute: m_IsActive
    path: Credits/CreatedBy
    classID: 1
    script: {fileID: 0}
  m_PPtrCurves: []
  m_Events:
  - time: 0.5
    functionName: PlayRoleActionAudio
    data: gather
    objectReferenceParameter: {fileID: 0}
    floatParameter: 1.25
    intParameter: 3
    messageOptions: 0
";
        let curves = parse_property_curves(yaml).unwrap();
        assert_eq!(curves.len(), 1);
        assert_eq!(curves[0].target_path, "Credits/CreatedBy");
        assert_eq!(curves[0].attribute, "m_IsActive");
        assert_eq!(curves[0].keys.len(), 2);
        assert_eq!(curves[0].sample(1.0), Some(1.0));

        let events = parse_animation_events(yaml).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].function_name, "PlayRoleActionAudio");
        assert_eq!(events[0].string_parameter, "gather");
        assert!((events[0].float_parameter - 1.25).abs() < f32::EPSILON);
        assert_eq!(events[0].int_parameter, 3);
        assert!(events[0].object_reference.is_none());
    }

    #[test]
    fn parses_unity_controller_states_transitions_and_parameters() {
        let yaml = r"%YAML 1.1
--- !u!1102 &10
AnimatorState:
  m_Name: Idle
  m_Speed: 1
  m_SpeedParameterActive: 1
  m_SpeedParameter: AnimationSpeed
  m_Transitions:
  - {fileID: 20}
  m_Motion: {fileID: 7400000, guid: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, type: 2}
--- !u!1101 &20
AnimatorStateTransition:
  m_Conditions:
  - m_ConditionMode: 1
    m_ConditionEvent: Moving
    m_EventTreshold: 0.5
  m_DstState: {fileID: 10}
  m_IsExit: 0
  m_TransitionDuration: 0.2
  m_TransitionOffset: 0.125
  m_ExitTime: 0.8
  m_HasExitTime: 1
  m_HasFixedDuration: 0
--- !u!1107 &30
AnimatorStateMachine:
  m_Name: Base Layer
  m_ChildStates:
  - serializedVersion: 1
    m_State: {fileID: 10}
  m_ChildStateMachines: []
  m_AnyStateTransitions: []
  m_EntryTransitions: []
  m_StateMachineTransitions: {}
  m_DefaultState: {fileID: 10}
--- !u!91 &40
AnimatorController:
  m_AnimatorParameters:
  - m_Name: Moving
    m_Type: 4
    m_DefaultFloat: 0
    m_DefaultInt: 0
    m_DefaultBool: 1
  - m_Name: AnimationSpeed
    m_Type: 1
    m_DefaultFloat: 1
    m_DefaultInt: 0
    m_DefaultBool: 0
  m_AnimatorLayers:
  - serializedVersion: 5
    m_Name: Base Layer
    m_StateMachine: {fileID: 30}
    m_Mask: {fileID: 0}
    m_BlendingMode: 0
    m_DefaultWeight: 0
";
        let asset = UnityAsset {
            guid: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".into(),
            path: "Assets/Test.controller".into(),
            kind: "animator_controller".into(),
            name: "Test".into(),
            importer_fields: Vec::new(),
            serialized_fields: Vec::new(),
            dependencies: Vec::new(),
            game_object: None,
            scene: None,
        };
        let clip_asset = UnityAsset {
            guid: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
            path: "Assets/Test.anim".into(),
            kind: "animation_clip".into(),
            name: "Idle".into(),
            importer_fields: Vec::new(),
            serialized_fields: Vec::new(),
            dependencies: Vec::new(),
            game_object: None,
            scene: None,
        };
        let assets = BTreeMap::from([(clip_asset.guid.as_str(), &clip_asset)]);
        let mut clips = BTreeMap::new();
        let controller =
            parse_controller(&asset, yaml, &assets, &BTreeMap::new(), &mut clips).unwrap();
        assert_eq!(controller.states.len(), 1);
        assert_eq!(controller.transitions.len(), 1);
        assert!((controller.transitions[0].duration - 0.2).abs() < f32::EPSILON);
        assert!(!controller.transitions[0].fixed_duration);
        assert!((controller.transitions[0].offset - 0.125).abs() < f32::EPSILON);
        assert_eq!(controller.parameters.len(), 2);
        assert_eq!(controller.default_states.len(), 1);
        assert_eq!(controller.state_machines.len(), 1);
        assert_eq!(controller.layers.len(), 1);
        assert_eq!(controller.layers[0].display_name, "Base Layer");
        assert_eq!(
            controller.layers[0].blend_mode,
            AnimationLayerBlendMode::Override
        );
        let machine = controller.state_machines.values().next().unwrap();
        assert_eq!(machine.display_name, "Base Layer");
        assert_eq!(machine.states.len(), 1);
        assert_eq!(
            machine.default_state.as_ref(),
            controller.default_states.first()
        );
        let state = controller.states.values().next().unwrap();
        assert_eq!(state.speed_parameter.as_deref(), Some("AnimationSpeed"));
        assert_eq!(clips.len(), 1);
    }

    #[test]
    fn parses_binary_unity_avatar_mask_paths() {
        let yaml = r"%YAML 1.1
--- !u!319 &31900000
AvatarMask:
  m_Name: Arms
  m_Mask: 01000000
  m_Elements:
  - m_Path:
    m_Weight: 1
  - m_Path: CharacterArmature/Body
    m_Weight: 0
  - m_Path: CharacterArmature/Body/UpperArm_L
    m_Weight: 1
";
        let mask = parse_avatar_mask(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "Assets/Arms.mask",
            "Fallback",
            yaml,
        )
        .unwrap();
        assert_eq!(mask.display_name, "Arms");
        assert_eq!(mask.humanoid_body_mask_hex, "01000000");
        assert_eq!(mask.transform_weights.len(), 3);
        assert!(mask.transform_weights["CharacterArmature/Body"].abs() < f32::EPSILON);
        assert!(
            (mask.transform_weights["CharacterArmature/Body/UpperArm_L"] - 1.0).abs()
                < f32::EPSILON
        );
    }

    #[test]
    fn extracts_indexed_material_properties() {
        let asset = UnityAsset {
            guid: "a".repeat(32),
            path: "Assets/Test.mat".into(),
            kind: "material".into(),
            name: "Test".into(),
            importer_fields: Vec::new(),
            serialized_fields: vec![
                UnityField {
                    path: "m_SavedProperties.m_Floats.Array.data[0].first".into(),
                    value: Value::String("_Metallic".into()),
                },
                UnityField {
                    path: "m_SavedProperties.m_Floats.Array.data[0].second".into(),
                    value: Value::from(0.5),
                },
                UnityField {
                    path: "m_SavedProperties.m_Colors.Array.data[0].first".into(),
                    value: Value::String("_TerrainTint".into()),
                },
                UnityField {
                    path: "m_SavedProperties.m_Colors.Array.data[0].second".into(),
                    value: serde_json::json!({"r": 0.1, "g": 0.2, "b": 0.3, "a": 0.4}),
                },
                UnityField {
                    path: "m_SavedProperties.m_TexEnvs.Array.data[0].first".into(),
                    value: Value::String("_MainTex".into()),
                },
                UnityField {
                    path: "m_SavedProperties.m_TexEnvs.Array.data[0].second.m_Scale".into(),
                    value: serde_json::json!({"x": 2.0, "y": 3.0}),
                },
                UnityField {
                    path: "m_SavedProperties.m_TexEnvs.Array.data[0].second.m_Offset".into(),
                    value: serde_json::json!({"x": 0.25, "y": 0.5}),
                },
            ],
            dependencies: Vec::new(),
            game_object: None,
            scene: None,
        };
        assert_eq!(
            named_values(&asset, "m_SavedProperties.m_Floats.Array.data[")["_Metallic"],
            Value::from(0.5)
        );
        let color = color_value(
            named_values(&asset, "m_SavedProperties.m_Colors.Array.data[").get("_TerrainTint"),
            [0.0; 4],
        );
        assert!(
            color
                .into_iter()
                .zip([0.1, 0.2, 0.3, 0.4])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        let texture_values = named_values(&asset, "m_SavedProperties.m_TexEnvs.Array.data[");
        assert!(
            vec2_value(texture_values["_MainTex"].get("m_Scale"), [1.0; 2])
                .into_iter()
                .zip([2.0, 3.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert!(
            vec2_value(texture_values["_MainTex"].get("m_Offset"), [0.0; 2])
                .into_iter()
                .zip([0.25, 0.5])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
    }

    #[test]
    fn parses_model_external_material_remaps_by_embedded_name() {
        let game = StableId::new("material:game").unwrap();
        let skin = StableId::new("material:skin").unwrap();
        let material_guids = BTreeMap::from([
            ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", &game),
            ("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", &skin),
        ]);
        let yaml = r"externalObjects:
  - first:
      type: UnityEngine:Material
      name: GameMaterial
    second: {fileID: 2100000, guid: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, type: 2}
  - first:
      type: UnityEngine:Material
      name: SkinMaterial
    second: {fileID: 2100000, guid: bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb, type: 2}
  - first:
      type: UnityEngine:Material
      name: UnreachableMaterial
    second: {fileID: 2100000, guid: cccccccccccccccccccccccccccccccc, type: 2}
";

        let mapped = parse_model_material_remaps(yaml, &material_guids);
        assert_eq!(mapped.len(), 2);
        assert_eq!(mapped["GameMaterial"], game);
        assert_eq!(mapped["SkinMaterial"], skin);
    }

    #[test]
    fn parses_embedded_model_clip_ids_and_blender_take_names() {
        let metadata = "ModelImporter:\n  internalIDToNameTable:\n  - first:\n      74: -42\n    second: Attack\n  - first:\n      74: 84\n    second: Idle\n  externalObjects:\n";
        assert_eq!(
            parse_model_clip_local_ids(metadata),
            BTreeMap::from([("Attack".to_owned(), -42), ("Idle".to_owned(), 84)])
        );
        assert_eq!(animation_take_name("Armature|Attack"), "Attack");
        assert_eq!(
            animation_take_name("Gate|Gate_Closing|BaseLayer"),
            "Gate_Closing"
        );
    }

    #[test]
    fn renderer_material_references_preserve_unity_slot_order() {
        let component = UnityComponent {
            hierarchy_path: "Root/Renderer".into(),
            type_name: Some("UnityEngine.MeshRenderer, UnityEngine.CoreModule".into()),
            fields: vec![UnityField {
                path: "sharedMaterials".into(),
                value: serde_json::json!([
                    {
                        "Guid": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                        "Path": "Assets/Materials/Game.mat",
                        "Name": "GameMaterial"
                    },
                    {
                        "Guid": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
                        "Path": "Assets/Materials/Skin.mat",
                        "Name": "SkinMaterial"
                    }
                ]),
            }],
        };

        let references = renderer_material_references(&component);
        assert_eq!(references.len(), 2);
        assert_eq!(references[0].name.as_deref(), Some("GameMaterial"));
        assert_eq!(
            references[1].guid.as_deref(),
            Some("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
        );
    }
}
