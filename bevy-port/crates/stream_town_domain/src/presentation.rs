use std::{collections::BTreeMap, path::Path};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::StableId;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PresentationCatalog {
    pub schema_version: u32,
    #[serde(default)]
    pub textures: BTreeMap<StableId, TextureDef>,
    #[serde(default)]
    pub materials: BTreeMap<StableId, MaterialDef>,
    #[serde(default)]
    pub clips: BTreeMap<StableId, AnimationClipDef>,
    #[serde(default)]
    pub controllers: BTreeMap<StableId, AnimationControllerDef>,
    #[serde(default)]
    pub prefab_bindings: BTreeMap<String, PrefabPresentationBinding>,
    /// Effective material dependencies after following nested prefab/model sources.
    #[serde(default)]
    pub prefab_materials: BTreeMap<String, Vec<StableId>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TextureDef {
    pub display_name: String,
    pub source_guid: String,
    pub source_path: String,
    /// Asset-server-relative path below `bevy-port/assets`.
    pub asset_path: String,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterialAlphaMode {
    #[default]
    Opaque,
    Mask,
    Blend,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MaterialDef {
    pub display_name: String,
    pub source_guid: String,
    pub source_path: String,
    pub shader_source: Option<String>,
    pub base_color: [f32; 4],
    pub emissive: [f32; 4],
    pub metallic: f32,
    pub perceptual_roughness: f32,
    pub alpha_mode: MaterialAlphaMode,
    #[serde(default)]
    pub textures: BTreeMap<String, StableId>,
    /// Unity properties that need a custom WGSL material rather than Bevy PBR.
    #[serde(default)]
    pub custom_properties: BTreeMap<String, f32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationClipDef {
    pub display_name: String,
    pub source_guid: String,
    pub source_path: String,
    pub duration_seconds: f32,
    pub sample_rate: f32,
    pub looping: bool,
    /// Source-model rest pose used to retarget Unity-local curves to a GLB rig.
    #[serde(default)]
    pub rig_asset_path: Option<String>,
    /// Engine-neutral Unity transform curves. Property-only clips have no tracks.
    #[serde(default)]
    pub transform_tracks: Vec<AnimationTransformTrack>,
    /// Set only when this exact clip has been converted to a GLB animation.
    pub converted_asset_path: Option<String>,
    pub gltf_animation_index: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationTransformTrack {
    /// Slash-separated path relative to the Unity Animator / Bevy animation root.
    pub target_path: String,
    #[serde(default)]
    pub reference_translation: Option<[f32; 3]>,
    #[serde(default)]
    pub reference_rotation: Option<[f32; 4]>,
    #[serde(default)]
    pub reference_scale: Option<[f32; 3]>,
    #[serde(default)]
    pub translation: Vec<AnimationVec3Keyframe>,
    #[serde(default)]
    pub rotation: Vec<AnimationQuatKeyframe>,
    #[serde(default)]
    pub scale: Vec<AnimationVec3Keyframe>,
    /// Unity's Z-X-Y Euler curves, retained for non-skeletal scene animation.
    #[serde(default)]
    pub euler_degrees: Vec<AnimationVec3Keyframe>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationVec3Keyframe {
    pub time: f32,
    pub value: [f32; 3],
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationQuatKeyframe {
    pub time: f32,
    pub value: [f32; 4],
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimationParameterKind {
    Float,
    Integer,
    Boolean,
    Trigger,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationParameterDef {
    pub name: String,
    pub kind: AnimationParameterKind,
    pub default_float: f32,
    pub default_integer: i32,
    pub default_boolean: bool,
    /// True when a transition referenced a parameter omitted from Unity's list.
    #[serde(default)]
    pub inferred: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationMotionDef {
    pub clip: StableId,
    pub threshold: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationStateDef {
    pub display_name: String,
    pub speed: f32,
    /// Optional Unity Animator float parameter that multiplies this state's speed.
    #[serde(default)]
    pub speed_parameter: Option<String>,
    /// Float parameter driving a Unity 1D blend tree, when this is a blend state.
    #[serde(default)]
    pub blend_parameter: Option<String>,
    pub motions: Vec<AnimationMotionDef>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimationConditionMode {
    If,
    IfNot,
    Greater,
    Less,
    Equals,
    NotEqual,
}

impl TryFrom<u8> for AnimationConditionMode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::If),
            2 => Ok(Self::IfNot),
            3 => Ok(Self::Greater),
            4 => Ok(Self::Less),
            6 => Ok(Self::Equals),
            7 => Ok(Self::NotEqual),
            value => Err(value),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationConditionDef {
    pub parameter: String,
    pub mode: AnimationConditionMode,
    pub threshold: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct AnimationTransitionDef {
    pub source: Option<StableId>,
    pub destination: Option<StableId>,
    #[serde(default)]
    pub source_state_machine: Option<StableId>,
    #[serde(default)]
    pub destination_state_machine: Option<StableId>,
    #[serde(default)]
    pub is_entry: bool,
    #[serde(default)]
    pub is_any_state: bool,
    pub is_exit: bool,
    pub has_exit_time: bool,
    pub exit_time: f32,
    pub duration: f32,
    pub conditions: Vec<AnimationConditionDef>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimationLayerBlendMode {
    #[default]
    Override,
    Additive,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationStateMachineDef {
    pub display_name: String,
    pub states: Vec<StableId>,
    pub child_state_machines: Vec<StableId>,
    pub default_state: Option<StableId>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationLayerDef {
    pub display_name: String,
    pub state_machine: StableId,
    pub blend_mode: AnimationLayerBlendMode,
    pub default_weight: f32,
    #[serde(default)]
    pub avatar_mask_guid: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationControllerDef {
    pub display_name: String,
    pub source_guid: String,
    pub source_path: String,
    pub parameters: Vec<AnimationParameterDef>,
    pub states: BTreeMap<StableId, AnimationStateDef>,
    pub transitions: Vec<AnimationTransitionDef>,
    #[serde(default)]
    pub state_machines: BTreeMap<StableId, AnimationStateMachineDef>,
    #[serde(default)]
    pub layers: Vec<AnimationLayerDef>,
    /// One default per Unity animation layer/state machine when present.
    pub default_states: Vec<StableId>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PrefabPresentationBinding {
    pub source_prefab_path: String,
    pub controller: StableId,
    /// Default converted model scene, including rigs without embedded animations.
    #[serde(default)]
    pub rig_scene: Option<String>,
    /// Converted scene whose embedded animation zero can be played natively.
    pub animated_scene: Option<String>,
    pub gltf_animation_index: Option<u32>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum PresentationError {
    #[error("texture {texture} has invalid asset path {path}")]
    InvalidTexturePath { texture: StableId, path: String },
    #[error("material {material} references missing texture {texture}")]
    MissingTexture {
        material: StableId,
        texture: StableId,
    },
    #[error("animation clip {clip} has invalid converted path {path}")]
    InvalidClipPath { clip: StableId, path: String },
    #[error("animation clip {clip} has invalid transform track {path}: {reason}")]
    InvalidAnimationTrack {
        clip: StableId,
        path: String,
        reason: String,
    },
    #[error("controller {controller} state {state} references missing clip {clip}")]
    MissingClip {
        controller: StableId,
        state: StableId,
        clip: StableId,
    },
    #[error("controller {controller} has missing default state {state}")]
    MissingDefaultState {
        controller: StableId,
        state: StableId,
    },
    #[error("controller {controller} transition references missing state {state}")]
    MissingTransitionState {
        controller: StableId,
        state: StableId,
    },
    #[error("controller {controller} references missing state machine {state_machine}")]
    MissingStateMachine {
        controller: StableId,
        state_machine: StableId,
    },
    #[error(
        "controller {controller} state machine {state_machine} references missing state {state}"
    )]
    MissingStateMachineState {
        controller: StableId,
        state_machine: StableId,
        state: StableId,
    },
    #[error("controller {controller} references missing animation parameter {parameter}")]
    MissingAnimationParameter {
        controller: StableId,
        parameter: String,
    },
    #[error("controller {controller} has invalid animation parameter {parameter}: {reason}")]
    InvalidAnimationParameter {
        controller: StableId,
        parameter: String,
        reason: String,
    },
    #[error("controller {controller} state {state} has invalid blend tree: {reason}")]
    InvalidBlendTree {
        controller: StableId,
        state: StableId,
        reason: String,
    },
    #[error("prefab {prefab_guid} references missing controller {controller}")]
    MissingController {
        prefab_guid: String,
        controller: StableId,
    },
    #[error("prefab {prefab_guid} has invalid animated scene path {path}")]
    InvalidAnimatedScene { prefab_guid: String, path: String },
    #[error("prefab {prefab_guid} references missing material {material}")]
    MissingPrefabMaterial {
        prefab_guid: String,
        material: StableId,
    },
}

impl PresentationCatalog {
    pub fn validate(&self) -> Result<(), PresentationError> {
        for (id, texture) in &self.textures {
            let extension_valid = Path::new(&texture.asset_path)
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| {
                    matches!(
                        extension.to_ascii_lowercase().as_str(),
                        "png" | "tga" | "jpg" | "jpeg"
                    )
                });
            if !texture.asset_path.starts_with("migrated/textures/")
                || texture.asset_path.contains("..")
                || texture.asset_path.contains('\\')
                || !extension_valid
            {
                return Err(PresentationError::InvalidTexturePath {
                    texture: id.clone(),
                    path: texture.asset_path.clone(),
                });
            }
        }
        for (id, material) in &self.materials {
            for texture in material.textures.values() {
                if !self.textures.contains_key(texture) {
                    return Err(PresentationError::MissingTexture {
                        material: id.clone(),
                        texture: texture.clone(),
                    });
                }
            }
        }
        for (id, clip) in &self.clips {
            if let Some(path) = &clip.converted_asset_path
                && (!path.starts_with("migrated/models/")
                    || !is_glb_path(path)
                    || path.contains("..")
                    || path.contains('\\'))
            {
                return Err(PresentationError::InvalidClipPath {
                    clip: id.clone(),
                    path: path.clone(),
                });
            }
            if let Some(path) = &clip.rig_asset_path
                && (!path.starts_with("migrated/models/")
                    || !is_glb_path(path)
                    || path.contains("..")
                    || path.contains('\\'))
            {
                return Err(PresentationError::InvalidClipPath {
                    clip: id.clone(),
                    path: path.clone(),
                });
            }
            for track in &clip.transform_tracks {
                validate_animation_track(id, track)?;
            }
        }
        for (controller_id, controller) in &self.controllers {
            let parameters: BTreeMap<_, _> = controller
                .parameters
                .iter()
                .map(|parameter| (parameter.name.as_str(), parameter))
                .collect();
            for default_state in &controller.default_states {
                if !controller.states.contains_key(default_state) {
                    return Err(PresentationError::MissingDefaultState {
                        controller: controller_id.clone(),
                        state: default_state.clone(),
                    });
                }
            }
            for layer in &controller.layers {
                if !controller.state_machines.contains_key(&layer.state_machine) {
                    return Err(PresentationError::MissingStateMachine {
                        controller: controller_id.clone(),
                        state_machine: layer.state_machine.clone(),
                    });
                }
            }
            for (state_machine_id, state_machine) in &controller.state_machines {
                for state in state_machine
                    .states
                    .iter()
                    .chain(state_machine.default_state.iter())
                {
                    if !controller.states.contains_key(state) {
                        return Err(PresentationError::MissingStateMachineState {
                            controller: controller_id.clone(),
                            state_machine: state_machine_id.clone(),
                            state: state.clone(),
                        });
                    }
                }
                for child in &state_machine.child_state_machines {
                    if !controller.state_machines.contains_key(child) {
                        return Err(PresentationError::MissingStateMachine {
                            controller: controller_id.clone(),
                            state_machine: child.clone(),
                        });
                    }
                }
            }
            for (state_id, state) in &controller.states {
                if let Some(speed_parameter) = &state.speed_parameter {
                    let Some(parameter) = parameters.get(speed_parameter.as_str()) else {
                        return Err(PresentationError::MissingAnimationParameter {
                            controller: controller_id.clone(),
                            parameter: speed_parameter.clone(),
                        });
                    };
                    if parameter.kind != AnimationParameterKind::Float {
                        return Err(PresentationError::InvalidAnimationParameter {
                            controller: controller_id.clone(),
                            parameter: speed_parameter.clone(),
                            reason: "state speed parameters must be floats".into(),
                        });
                    }
                }
                if let Some(blend_parameter) = &state.blend_parameter {
                    let Some(parameter) = parameters.get(blend_parameter.as_str()) else {
                        return Err(PresentationError::MissingAnimationParameter {
                            controller: controller_id.clone(),
                            parameter: blend_parameter.clone(),
                        });
                    };
                    if parameter.kind != AnimationParameterKind::Float
                        || state.motions.iter().any(|motion| {
                            motion
                                .threshold
                                .is_none_or(|threshold| !threshold.is_finite())
                        })
                    {
                        return Err(PresentationError::InvalidBlendTree {
                            controller: controller_id.clone(),
                            state: state_id.clone(),
                            reason:
                                "1D blend trees require a float parameter and finite thresholds"
                                    .into(),
                        });
                    }
                }
                for motion in &state.motions {
                    if !self.clips.contains_key(&motion.clip) {
                        return Err(PresentationError::MissingClip {
                            controller: controller_id.clone(),
                            state: state_id.clone(),
                            clip: motion.clip.clone(),
                        });
                    }
                }
            }
            for transition in &controller.transitions {
                for condition in &transition.conditions {
                    if !parameters.contains_key(condition.parameter.as_str()) {
                        return Err(PresentationError::MissingAnimationParameter {
                            controller: controller_id.clone(),
                            parameter: condition.parameter.clone(),
                        });
                    }
                }
                for state in [transition.source.as_ref(), transition.destination.as_ref()]
                    .into_iter()
                    .flatten()
                {
                    if !controller.states.contains_key(state) {
                        return Err(PresentationError::MissingTransitionState {
                            controller: controller_id.clone(),
                            state: state.clone(),
                        });
                    }
                }
                for state_machine in [
                    transition.source_state_machine.as_ref(),
                    transition.destination_state_machine.as_ref(),
                ]
                .into_iter()
                .flatten()
                {
                    if !controller.state_machines.contains_key(state_machine) {
                        return Err(PresentationError::MissingStateMachine {
                            controller: controller_id.clone(),
                            state_machine: state_machine.clone(),
                        });
                    }
                }
            }
        }
        for (prefab_guid, binding) in &self.prefab_bindings {
            if !self.controllers.contains_key(&binding.controller) {
                return Err(PresentationError::MissingController {
                    prefab_guid: prefab_guid.clone(),
                    controller: binding.controller.clone(),
                });
            }
            if let Some(path) = &binding.animated_scene
                && (!path.starts_with("migrated/models/")
                    || !is_glb_path(path)
                    || path.contains("..")
                    || path.contains('\\'))
            {
                return Err(PresentationError::InvalidAnimatedScene {
                    prefab_guid: prefab_guid.clone(),
                    path: path.clone(),
                });
            }
            if let Some(path) = &binding.rig_scene
                && (!path.starts_with("migrated/models/")
                    || !is_glb_path(path)
                    || path.contains("..")
                    || path.contains('\\'))
            {
                return Err(PresentationError::InvalidAnimatedScene {
                    prefab_guid: prefab_guid.clone(),
                    path: path.clone(),
                });
            }
        }
        for (prefab_guid, materials) in &self.prefab_materials {
            for material in materials {
                if !self.materials.contains_key(material) {
                    return Err(PresentationError::MissingPrefabMaterial {
                        prefab_guid: prefab_guid.clone(),
                        material: material.clone(),
                    });
                }
            }
        }
        Ok(())
    }
}

fn validate_animation_track(
    clip: &StableId,
    track: &AnimationTransformTrack,
) -> Result<(), PresentationError> {
    if track.target_path.is_empty()
        || track.target_path.contains('\\')
        || track
            .target_path
            .split('/')
            .any(|segment| segment.is_empty() || matches!(segment, "." | ".."))
    {
        return Err(PresentationError::InvalidAnimationTrack {
            clip: clip.clone(),
            path: track.target_path.clone(),
            reason: "target path is not portable".into(),
        });
    }
    let references_valid = track
        .reference_translation
        .is_none_or(|value| value.into_iter().all(f32::is_finite))
        && track
            .reference_rotation
            .is_none_or(|value| value.into_iter().all(f32::is_finite))
        && track
            .reference_scale
            .is_none_or(|value| value.into_iter().all(f32::is_finite));
    let curves_valid = valid_vec3_curve(&track.translation)
        && valid_quat_curve(&track.rotation)
        && valid_vec3_curve(&track.scale)
        && valid_vec3_curve(&track.euler_degrees);
    if !references_valid || !curves_valid {
        return Err(PresentationError::InvalidAnimationTrack {
            clip: clip.clone(),
            path: track.target_path.clone(),
            reason: "keyframe times/values must be finite and monotonic".into(),
        });
    }
    if track.translation.is_empty()
        && track.rotation.is_empty()
        && track.scale.is_empty()
        && track.euler_degrees.is_empty()
    {
        return Err(PresentationError::InvalidAnimationTrack {
            clip: clip.clone(),
            path: track.target_path.clone(),
            reason: "track has no transform curves".into(),
        });
    }
    Ok(())
}

fn valid_vec3_curve(curve: &[AnimationVec3Keyframe]) -> bool {
    valid_curve_times(curve.iter().map(|key| key.time))
        && curve
            .iter()
            .all(|key| key.value.into_iter().all(f32::is_finite))
}

fn valid_quat_curve(curve: &[AnimationQuatKeyframe]) -> bool {
    valid_curve_times(curve.iter().map(|key| key.time))
        && curve
            .iter()
            .all(|key| key.value.into_iter().all(f32::is_finite))
}

fn valid_curve_times(times: impl Iterator<Item = f32>) -> bool {
    let mut previous = None;
    for time in times {
        if !time.is_finite() || time < 0.0 || previous.is_some_and(|previous| time < previous) {
            return false;
        }
        previous = Some(time);
    }
    true
}

fn is_glb_path(path: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_dangling_material_texture() {
        let material = StableId::new("material:one").unwrap();
        let missing = StableId::new("texture:missing").unwrap();
        let catalog = PresentationCatalog {
            schema_version: 1,
            materials: BTreeMap::from([(
                material.clone(),
                MaterialDef {
                    display_name: "One".into(),
                    source_guid: "a".repeat(32),
                    source_path: "Assets/One.mat".into(),
                    shader_source: None,
                    base_color: [1.0; 4],
                    emissive: [0.0; 4],
                    metallic: 0.0,
                    perceptual_roughness: 1.0,
                    alpha_mode: MaterialAlphaMode::Opaque,
                    textures: BTreeMap::from([("_MainTex".into(), missing.clone())]),
                    custom_properties: BTreeMap::new(),
                },
            )]),
            ..PresentationCatalog::default()
        };
        assert_eq!(
            catalog.validate(),
            Err(PresentationError::MissingTexture {
                material,
                texture: missing,
            })
        );
    }
}
