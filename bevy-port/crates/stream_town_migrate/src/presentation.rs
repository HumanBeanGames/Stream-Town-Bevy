use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use stream_town_domain::{
    AnimationClipDef, AnimationConditionDef, AnimationControllerDef, AnimationMotionDef,
    AnimationParameterDef, AnimationParameterKind, AnimationQuatKeyframe, AnimationStateDef,
    AnimationTransformTrack, AnimationTransitionDef, AnimationVec3Keyframe, MaterialAlphaMode,
    MaterialDef, PrefabPresentationBinding, PresentationCatalog, StableId, TextureDef,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PresentationConversionReport {
    pub schema_version: u32,
    pub textures: usize,
    pub texture_bytes: u64,
    pub materials: usize,
    pub custom_shader_materials: usize,
    pub material_prefab_bindings: usize,
    pub material_slots: usize,
    pub clips: usize,
    pub converted_clips: usize,
    pub missing_clip_sources: usize,
    pub controllers: usize,
    pub controller_states: usize,
    pub controller_transitions: usize,
    pub prefab_bindings: usize,
    pub native_animation_bindings: usize,
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
    path: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityGameObject {
    #[serde(default)]
    components: Vec<UnityComponent>,
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
    threshold: Option<f32>,
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

    let (textures, texture_bytes) = convert_textures(&export, &root, out_dir)?;
    let materials = convert_materials(&export, &assets_by_path)?;
    let prefab_materials = convert_prefab_materials(&export, &assets_by_path, &materials);
    let mut clips = convert_clips(&export, &root)?;
    let controllers = convert_controllers(&export, &root, &assets_by_guid, &mut clips)?;
    let prefab_bindings = convert_prefab_bindings(&export, &assets_by_path, &controllers);
    assign_clip_rigs_and_reference_poses(
        &export,
        &assets_by_path,
        &controllers,
        &prefab_bindings,
        &mut clips,
    );
    let catalog = PresentationCatalog {
        schema_version: 2,
        textures,
        materials,
        clips,
        controllers,
        prefab_bindings,
        prefab_materials,
    };
    catalog
        .validate()
        .context("converted presentation catalog is invalid")?;

    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create output directory {}", out_dir.display()))?;
    let catalog_path = out_dir.join("presentation.ron");
    let report_path = out_dir.join("presentation-report.ron");
    let report = PresentationConversionReport {
        schema_version: 2,
        textures: catalog.textures.len(),
        texture_bytes,
        materials: catalog.materials.len(),
        custom_shader_materials: catalog
            .materials
            .values()
            .filter(|material| !material.custom_properties.is_empty())
            .count(),
        material_prefab_bindings: catalog.prefab_materials.len(),
        material_slots: catalog.prefab_materials.values().map(Vec::len).sum(),
        clips: catalog.clips.len(),
        converted_clips: catalog
            .clips
            .values()
            .filter(|clip| clip.converted_asset_path.is_some() || !clip.transform_tracks.is_empty())
            .count(),
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
        prefab_bindings: catalog.prefab_bindings.len(),
        native_animation_bindings: catalog
            .prefab_bindings
            .values()
            .filter(|binding| binding.gltf_animation_index.is_some())
            .count(),
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
            },
        );
    }
    Ok((textures, total_bytes))
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
                custom_properties,
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
        let transform_tracks = if Path::new(&asset.path)
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("anim"))
        {
            let source = unity_root.join(&asset.path);
            let contents = fs::read_to_string(&source)
                .with_context(|| format!("failed to read animation clip {}", source.display()))?;
            parse_transform_tracks(&contents)
                .with_context(|| format!("failed to parse animation clip {}", source.display()))?
        } else {
            Vec::new()
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
                converted_asset_path: None,
                gltf_animation_index: None,
            },
        );
    }
    Ok(clips)
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
        let controller = parse_controller(asset, &contents, assets_by_guid, clips)?;
        controllers.insert(controller_id(&asset.guid)?, controller);
    }
    Ok(controllers)
}

fn parse_controller(
    asset: &UnityAsset,
    contents: &str,
    assets_by_guid: &BTreeMap<&str, &UnityAsset>,
    clips: &mut BTreeMap<StableId, AnimationClipDef>,
) -> Result<AnimationControllerDef> {
    let documents = parse_yaml_documents(contents)?;
    let blend_motions: BTreeMap<i64, Vec<ParsedMotion>> = documents
        .iter()
        .filter(|document| document.class_id == 206)
        .map(|document| (document.file_id, parse_motions(&document.lines)))
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

    let mut transition_sources = BTreeMap::new();
    let mut states = BTreeMap::new();
    for document in state_documents {
        let state_id = state_ids[&document.file_id].clone();
        for transition in parse_reference_list(&document.lines, "m_Transitions:") {
            transition_sources.insert(transition, state_id.clone());
        }
        let motions = parse_state_motions(document, &blend_motions);
        let mut converted_motions = Vec::new();
        for motion in motions {
            let clip = clip_id(&motion.guid)?;
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
                motions: converted_motions,
            },
        );
    }

    let transitions = documents
        .iter()
        .filter(|document| matches!(document.class_id, 1101 | 1109))
        .filter_map(|document| {
            let destination_file = reference_id(&document.lines, "m_DstState:").unwrap_or(0);
            let source = transition_sources.get(&document.file_id).cloned();
            let destination = state_ids.get(&destination_file).cloned();
            let is_exit = scalar_bool(&document.lines, "m_IsExit:").unwrap_or(false);
            (source.is_some() || destination.is_some() || is_exit).then(|| AnimationTransitionDef {
                source,
                destination,
                is_exit,
                has_exit_time: scalar_bool(&document.lines, "m_HasExitTime:").unwrap_or(false),
                exit_time: scalar_f32(&document.lines, "m_ExitTime:").unwrap_or(0.0),
                duration: scalar_f32(&document.lines, "m_TransitionDuration:").unwrap_or(0.0),
                conditions: parse_conditions(&document.lines),
            })
        })
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
    let parameters = documents
        .iter()
        .find(|document| document.class_id == 91)
        .map_or_else(Vec::new, |document| parse_parameters(&document.lines));
    Ok(AnimationControllerDef {
        display_name: asset.name.clone(),
        source_guid: asset.guid.clone(),
        source_path: asset.path.clone(),
        parameters,
        states,
        transitions,
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
        let Some(controller) = dependency_paths
            .iter()
            .find_map(|path| controller_paths.get(path.as_str()).copied())
        else {
            continue;
        };
        let animated_model = dependency_paths.iter().find_map(|path| {
            let model = assets_by_path.get(path.as_str())?;
            (model.kind == "model" && model_has_animation(model)).then_some(path.as_str())
        });
        let rig_model = animated_model.or_else(|| {
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
                file_id: file_id.parse()?,
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
    blend_motions: &BTreeMap<i64, Vec<ParsedMotion>>,
) -> Vec<ParsedMotion> {
    let Some(line) = state
        .lines
        .iter()
        .find(|line| line.trim_start().starts_with("m_Motion:"))
    else {
        return Vec::new();
    };
    if let Some(guid) = reference_guid(line) {
        return vec![ParsedMotion {
            guid: guid.to_owned(),
            threshold: None,
        }];
    }
    inline_file_id(line)
        .and_then(|file_id| blend_motions.get(&file_id).cloned())
        .unwrap_or_default()
}

fn parse_motions(lines: &[String]) -> Vec<ParsedMotion> {
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
            threshold,
        });
    }
    motions
}

fn parse_conditions(lines: &[String]) -> Vec<AnimationConditionDef> {
    let mut conditions = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let Some(mode) = line
            .trim()
            .strip_prefix("- m_ConditionMode: ")
            .and_then(|value| value.parse().ok())
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
        conditions.push(AnimationConditionDef {
            parameter,
            mode,
            threshold,
        });
    }
    conditions
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
        });
        index += 1;
    }
    parameters
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

fn glb_asset_path(source_model: &str) -> String {
    let relative = source_model.strip_prefix("Assets/").unwrap_or(source_model);
    let stem = relative
        .rsplit_once('.')
        .map_or(relative, |(stem, _extension)| stem);
    format!("migrated/models/{stem}.glb")
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

fn controller_id(guid: &str) -> Result<StableId> {
    StableId::new(format!("controller:{guid}")).map_err(Into::into)
}

fn animation_state_id(controller_guid: &str, file_id: i64) -> Result<StableId> {
    StableId::new(format!("animation_state:{controller_guid}:{file_id}")).map_err(Into::into)
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
    fn parses_unity_controller_states_transitions_and_parameters() {
        let yaml = r"%YAML 1.1
--- !u!1102 &10
AnimatorState:
  m_Name: Idle
  m_Speed: 1
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
  m_ExitTime: 0.8
  m_HasExitTime: 1
--- !u!1107 &30
AnimatorStateMachine:
  m_DefaultState: {fileID: 10}
--- !u!91 &40
AnimatorController:
  m_AnimatorParameters:
  - m_Name: Moving
    m_Type: 4
    m_DefaultFloat: 0
    m_DefaultInt: 0
    m_DefaultBool: 1
  m_AnimatorLayers: []
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
        };
        let assets = BTreeMap::from([(clip_asset.guid.as_str(), &clip_asset)]);
        let mut clips = BTreeMap::new();
        let controller = parse_controller(&asset, yaml, &assets, &mut clips).unwrap();
        assert_eq!(controller.states.len(), 1);
        assert_eq!(controller.transitions.len(), 1);
        assert_eq!(controller.parameters.len(), 1);
        assert_eq!(controller.default_states.len(), 1);
        assert_eq!(clips.len(), 1);
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
            ],
            dependencies: Vec::new(),
            game_object: None,
        };
        assert_eq!(
            named_values(&asset, "m_SavedProperties.m_Floats.Array.data[")["_Metallic"],
            Value::from(0.5)
        );
    }
}
