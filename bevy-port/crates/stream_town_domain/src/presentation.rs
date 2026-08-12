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
    pub avatar_masks: BTreeMap<StableId, AvatarMaskDef>,
    #[serde(default)]
    pub prefab_bindings: BTreeMap<String, PrefabPresentationBinding>,
    /// Effective material dependencies after following nested prefab/model sources.
    #[serde(default)]
    pub prefab_materials: BTreeMap<String, Vec<StableId>>,
    /// Unity model-importer material names mapped to stable authored materials.
    #[serde(default)]
    pub model_materials: BTreeMap<String, BTreeMap<String, StableId>>,
    /// Per-renderer prefab overrides, keyed by the source prefab GUID.
    #[serde(default)]
    pub prefab_renderer_materials: BTreeMap<String, Vec<RendererMaterialBinding>>,
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
    /// Unity component/UI float curves retained independently of skeletal tracks.
    #[serde(default)]
    pub property_curves: Vec<AnimationPropertyCurve>,
    /// Authored Unity `AnimationEvents` in stable time order.
    #[serde(default)]
    pub events: Vec<AnimationEventDef>,
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationPropertyCurve {
    /// Slash-separated path relative to the Animator; empty targets the root.
    pub target_path: String,
    pub attribute: String,
    pub class_id: i32,
    #[serde(default)]
    pub script_guid: Option<String>,
    pub keys: Vec<AnimationFloatKeyframe>,
}

impl AnimationPropertyCurve {
    /// Samples Unity's unweighted Hermite/constant float curve semantics.
    #[must_use]
    pub fn sample(&self, time: f32) -> Option<f32> {
        let first = self.keys.first()?;
        if time <= first.time {
            return Some(first.value);
        }
        let last = self.keys.last()?;
        if time >= last.time {
            return Some(last.value);
        }
        let pair = self
            .keys
            .windows(2)
            .find(|pair| time >= pair[0].time && time < pair[1].time)?;
        let left = &pair[0];
        let right = &pair[1];
        if !left.out_slope.is_finite() || !right.in_slope.is_finite() {
            return Some(left.value);
        }
        let duration = right.time - left.time;
        if duration <= f32::EPSILON {
            return Some(right.value);
        }
        let t = (time - left.time) / duration;
        let t2 = t * t;
        let t3 = t2 * t;
        Some(
            (2.0 * t3 - 3.0 * t2 + 1.0) * left.value
                + (t3 - 2.0 * t2 + t) * duration * left.out_slope.finite_value()
                + (-2.0 * t3 + 3.0 * t2) * right.value
                + (t3 - t2) * duration * right.in_slope.finite_value(),
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationFloatKeyframe {
    pub time: f32,
    pub value: f32,
    pub in_slope: AnimationTangent,
    pub out_slope: AnimationTangent,
    pub tangent_mode: u32,
    pub weighted_mode: u8,
    pub in_weight: f32,
    pub out_weight: f32,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimationTangent {
    Finite(f32),
    PositiveInfinity,
    NegativeInfinity,
}

impl AnimationTangent {
    #[must_use]
    pub const fn is_finite(self) -> bool {
        matches!(self, Self::Finite(_))
    }

    const fn finite_value(self) -> f32 {
        match self {
            Self::Finite(value) => value,
            Self::PositiveInfinity | Self::NegativeInfinity => 0.0,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationEventDef {
    pub time: f32,
    pub function_name: String,
    #[serde(default)]
    pub string_parameter: String,
    #[serde(default)]
    pub object_reference: Option<AnimationObjectReference>,
    pub float_parameter: f32,
    pub int_parameter: i32,
    pub message_options: i32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AnimationObjectReference {
    pub file_id: i64,
    #[serde(default)]
    pub guid: Option<String>,
    #[serde(default)]
    pub type_id: Option<u32>,
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
    /// Whether `duration` is seconds (`true`) or normalized source-state time.
    #[serde(default = "default_transition_fixed_duration")]
    pub fixed_duration: bool,
    /// Normalized start offset in the destination state.
    #[serde(default)]
    pub offset: f32,
    pub conditions: Vec<AnimationConditionDef>,
}

const fn default_transition_fixed_duration() -> bool {
    true
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
    pub avatar_mask: Option<StableId>,
}

impl AnimationLayerDef {
    /// Unity fixes the base layer's runtime influence at one even though the
    /// controller YAML serializes its `m_DefaultWeight` as zero.
    #[must_use]
    pub fn effective_weight(&self, layer_index: usize) -> f32 {
        if layer_index == 0 {
            1.0
        } else {
            self.default_weight
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AvatarMaskDef {
    pub display_name: String,
    pub source_guid: String,
    pub source_path: String,
    /// Unity's humanoid body-part mask, retained losslessly for future humanoid rigs.
    pub humanoid_body_mask_hex: String,
    /// Slash-separated transform paths. An empty path represents the Animator root.
    pub transform_weights: BTreeMap<String, f32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RendererMaterialBinding {
    /// Unity renderer path relative to the prefab root.
    pub target_path: String,
    /// Converted glTF material name to the effective Unity prefab material.
    pub materials: BTreeMap<String, StableId>,
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
    #[error("controller {controller} has invalid transition timing: {reason}")]
    InvalidTransitionTiming {
        controller: StableId,
        reason: String,
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
    #[error("avatar mask {mask} has invalid transform {path}: {reason}")]
    InvalidAvatarMask {
        mask: StableId,
        path: String,
        reason: String,
    },
    #[error("controller {controller} layer {layer} references missing avatar mask {mask}")]
    MissingAvatarMask {
        controller: StableId,
        layer: String,
        mask: StableId,
    },
    #[error("controller {controller} layer {layer} has invalid default weight: {reason}")]
    InvalidAnimationLayerWeight {
        controller: StableId,
        layer: String,
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
    #[error("model {model} material {embedded_name} references missing material {material}")]
    MissingModelMaterial {
        model: String,
        embedded_name: String,
        material: StableId,
    },
    #[error("prefab {prefab_guid} renderer {target_path} has invalid material binding: {reason}")]
    InvalidRendererMaterialBinding {
        prefab_guid: String,
        target_path: String,
        reason: String,
    },
    #[error("animation clip {clip} has invalid property curve {path}/{attribute}: {reason}")]
    InvalidPropertyCurve {
        clip: StableId,
        path: String,
        attribute: String,
        reason: String,
    },
    #[error("animation clip {clip} has invalid event {function_name}: {reason}")]
    InvalidAnimationEvent {
        clip: StableId,
        function_name: String,
        reason: String,
    },
}

impl PresentationCatalog {
    pub fn validate(&self) -> Result<(), PresentationError> {
        for (id, mask) in &self.avatar_masks {
            if mask.source_path.contains('\\')
                || mask.source_path.contains("..")
                || !Path::new(&mask.source_path)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("mask"))
                || mask.humanoid_body_mask_hex.is_empty()
                || mask.humanoid_body_mask_hex.len() % 2 != 0
                || !mask
                    .humanoid_body_mask_hex
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit())
            {
                return Err(PresentationError::InvalidAvatarMask {
                    mask: id.clone(),
                    path: mask.source_path.clone(),
                    reason: "source path and body mask must be portable Unity mask data".into(),
                });
            }
            for (path, weight) in &mask.transform_weights {
                let path_valid = !path.contains('\\')
                    && (path.is_empty()
                        || path
                            .split('/')
                            .all(|segment| !segment.is_empty() && !matches!(segment, "." | "..")));
                if !path_valid || !matches!(*weight, 0.0 | 1.0) {
                    return Err(PresentationError::InvalidAvatarMask {
                        mask: id.clone(),
                        path: path.clone(),
                        reason: "transform paths must be portable and weights must be binary"
                            .into(),
                    });
                }
            }
        }
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
            for curve in &clip.property_curves {
                validate_property_curve(id, curve)?;
            }
            for event in &clip.events {
                let reference_valid = event.object_reference.as_ref().is_none_or(|reference| {
                    reference.file_id >= 0
                        && reference.guid.as_ref().is_none_or(|guid| {
                            guid.len() == 32 && guid.bytes().all(|byte| byte.is_ascii_hexdigit())
                        })
                });
                if !event.time.is_finite()
                    || event.time < 0.0
                    || event.function_name.trim().is_empty()
                    || !event.float_parameter.is_finite()
                    || !reference_valid
                {
                    return Err(PresentationError::InvalidAnimationEvent {
                        clip: id.clone(),
                        function_name: event.function_name.clone(),
                        reason: "event time, function, parameters, and object reference must be portable"
                            .into(),
                    });
                }
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
                if !layer.default_weight.is_finite() || !(0.0..=1.0).contains(&layer.default_weight)
                {
                    return Err(PresentationError::InvalidAnimationLayerWeight {
                        controller: controller_id.clone(),
                        layer: layer.display_name.clone(),
                        reason: "weight must be within 0..=1".into(),
                    });
                }
                if !controller.state_machines.contains_key(&layer.state_machine) {
                    return Err(PresentationError::MissingStateMachine {
                        controller: controller_id.clone(),
                        state_machine: layer.state_machine.clone(),
                    });
                }
                if let Some(mask) = &layer.avatar_mask
                    && !self.avatar_masks.contains_key(mask)
                {
                    return Err(PresentationError::MissingAvatarMask {
                        controller: controller_id.clone(),
                        layer: layer.display_name.clone(),
                        mask: mask.clone(),
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
                if !transition.duration.is_finite()
                    || transition.duration < 0.0
                    || !transition.offset.is_finite()
                    || transition.offset < 0.0
                {
                    return Err(PresentationError::InvalidTransitionTiming {
                        controller: controller_id.clone(),
                        reason: "duration and destination offset must be finite and non-negative"
                            .into(),
                    });
                }
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
        for (model, materials) in &self.model_materials {
            if model.contains('\\')
                || model.contains("..")
                || !Path::new(model)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("fbx"))
            {
                return Err(PresentationError::InvalidRendererMaterialBinding {
                    prefab_guid: "model".into(),
                    target_path: model.clone(),
                    reason: "model path must be a portable FBX source path".into(),
                });
            }
            for (embedded_name, material) in materials {
                if embedded_name.trim().is_empty() || !self.materials.contains_key(material) {
                    return Err(PresentationError::MissingModelMaterial {
                        model: model.clone(),
                        embedded_name: embedded_name.clone(),
                        material: material.clone(),
                    });
                }
            }
        }
        for (prefab_guid, renderers) in &self.prefab_renderer_materials {
            let mut paths = std::collections::BTreeSet::new();
            for renderer in renderers {
                let path_valid = !renderer.target_path.contains('\\')
                    && (renderer.target_path.is_empty()
                        || renderer
                            .target_path
                            .split('/')
                            .all(|segment| !segment.is_empty() && !matches!(segment, "." | "..")));
                if !path_valid
                    || renderer.materials.is_empty()
                    || !paths.insert(&renderer.target_path)
                {
                    return Err(PresentationError::InvalidRendererMaterialBinding {
                        prefab_guid: prefab_guid.clone(),
                        target_path: renderer.target_path.clone(),
                        reason:
                            "renderer paths must be unique/portable and bind at least one material"
                                .into(),
                    });
                }
                for (embedded_name, material) in &renderer.materials {
                    if embedded_name.trim().is_empty() || !self.materials.contains_key(material) {
                        return Err(PresentationError::MissingPrefabMaterial {
                            prefab_guid: prefab_guid.clone(),
                            material: material.clone(),
                        });
                    }
                }
            }
        }
        Ok(())
    }
}

fn validate_property_curve(
    clip: &StableId,
    curve: &AnimationPropertyCurve,
) -> Result<(), PresentationError> {
    let path_valid = !curve.target_path.contains('\\')
        && (curve.target_path.is_empty()
            || curve
                .target_path
                .split('/')
                .all(|segment| !segment.is_empty() && !matches!(segment, "." | "..")));
    let script_valid = curve
        .script_guid
        .as_ref()
        .is_none_or(|guid| guid.len() == 32 && guid.bytes().all(|byte| byte.is_ascii_hexdigit()));
    let tangent_valid = |tangent: AnimationTangent| match tangent {
        AnimationTangent::Finite(value) => value.is_finite(),
        AnimationTangent::PositiveInfinity | AnimationTangent::NegativeInfinity => true,
    };
    let mut previous = None;
    let keys_valid = !curve.keys.is_empty()
        && curve.keys.iter().all(|key| {
            let ordered = previous.is_none_or(|previous| key.time >= previous);
            previous = Some(key.time);
            ordered
                && key.time.is_finite()
                && key.time >= 0.0
                && key.value.is_finite()
                && tangent_valid(key.in_slope)
                && tangent_valid(key.out_slope)
                && key.in_weight.is_finite()
                && key.out_weight.is_finite()
                && key.in_weight >= 0.0
                && key.out_weight >= 0.0
        });
    if !path_valid
        || curve.attribute.trim().is_empty()
        || curve.class_id <= 0
        || !script_valid
        || !keys_valid
    {
        return Err(PresentationError::InvalidPropertyCurve {
            clip: clip.clone(),
            path: curve.target_path.clone(),
            attribute: curve.attribute.clone(),
            reason: "binding metadata and ordered keyframes must be finite/portable".into(),
        });
    }
    Ok(())
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

    #[test]
    fn animator_base_layer_has_fixed_runtime_weight() {
        let layer = AnimationLayerDef {
            display_name: "Base Layer".into(),
            state_machine: StableId::new("animation_state_machine:test:1").unwrap(),
            blend_mode: AnimationLayerBlendMode::Override,
            default_weight: 0.0,
            avatar_mask: None,
        };
        assert!((layer.effective_weight(0) - 1.0).abs() < f32::EPSILON);
        assert!(layer.effective_weight(1).abs() < f32::EPSILON);
    }

    #[test]
    fn property_curve_samples_constant_and_hermite_segments() {
        let constant = AnimationPropertyCurve {
            target_path: "Panel".into(),
            attribute: "m_IsActive".into(),
            class_id: 1,
            script_guid: None,
            keys: vec![
                AnimationFloatKeyframe {
                    time: 0.0,
                    value: 1.0,
                    in_slope: AnimationTangent::PositiveInfinity,
                    out_slope: AnimationTangent::PositiveInfinity,
                    tangent_mode: 103,
                    weighted_mode: 0,
                    in_weight: 0.0,
                    out_weight: 0.0,
                },
                AnimationFloatKeyframe {
                    time: 1.0,
                    value: 0.0,
                    in_slope: AnimationTangent::PositiveInfinity,
                    out_slope: AnimationTangent::PositiveInfinity,
                    tangent_mode: 103,
                    weighted_mode: 0,
                    in_weight: 0.0,
                    out_weight: 0.0,
                },
            ],
        };
        assert_eq!(constant.sample(0.75), Some(1.0));
        assert_eq!(constant.sample(1.0), Some(0.0));

        let mut hermite = constant;
        hermite.keys[0].value = 0.0;
        hermite.keys[0].in_slope = AnimationTangent::Finite(0.0);
        hermite.keys[0].out_slope = AnimationTangent::Finite(0.0);
        hermite.keys[1].value = 1.0;
        hermite.keys[1].in_slope = AnimationTangent::Finite(0.0);
        hermite.keys[1].out_slope = AnimationTangent::Finite(0.0);
        assert!((hermite.sample(0.5).unwrap() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn rejects_dangling_avatar_mask_reference() {
        let controller_id = StableId::new("controller:test").unwrap();
        let machine_id = StableId::new("animation_state_machine:test:1").unwrap();
        let mask_id = StableId::new("avatar_mask:missing").unwrap();
        let catalog = PresentationCatalog {
            schema_version: 6,
            controllers: BTreeMap::from([(
                controller_id.clone(),
                AnimationControllerDef {
                    display_name: "Test".into(),
                    source_guid: "a".repeat(32),
                    source_path: "Assets/Test.controller".into(),
                    parameters: Vec::new(),
                    states: BTreeMap::new(),
                    transitions: Vec::new(),
                    state_machines: BTreeMap::from([(
                        machine_id.clone(),
                        AnimationStateMachineDef {
                            display_name: "Base Layer".into(),
                            states: Vec::new(),
                            child_state_machines: Vec::new(),
                            default_state: None,
                        },
                    )]),
                    layers: vec![AnimationLayerDef {
                        display_name: "Base Layer".into(),
                        state_machine: machine_id,
                        blend_mode: AnimationLayerBlendMode::Override,
                        default_weight: 0.0,
                        avatar_mask: Some(mask_id.clone()),
                    }],
                    default_states: Vec::new(),
                },
            )]),
            ..PresentationCatalog::default()
        };
        assert_eq!(
            catalog.validate(),
            Err(PresentationError::MissingAvatarMask {
                controller: controller_id,
                layer: "Base Layer".into(),
                mask: mask_id,
            })
        );
    }

    #[test]
    fn rejects_invalid_renderer_material_bindings() {
        let material = StableId::new("material:one").unwrap();
        let definition = MaterialDef {
            display_name: "One".into(),
            source_guid: "a".repeat(32),
            source_path: "Assets/One.mat".into(),
            shader_source: None,
            base_color: [1.0; 4],
            emissive: [0.0; 4],
            metallic: 0.0,
            perceptual_roughness: 1.0,
            alpha_mode: MaterialAlphaMode::Opaque,
            textures: BTreeMap::new(),
            custom_properties: BTreeMap::new(),
        };
        let invalid_path = PresentationCatalog {
            schema_version: 7,
            materials: BTreeMap::from([(material.clone(), definition.clone())]),
            prefab_renderer_materials: BTreeMap::from([(
                "prefab".into(),
                vec![RendererMaterialBinding {
                    target_path: "Root/../Renderer".into(),
                    materials: BTreeMap::from([("GameMaterial".into(), material.clone())]),
                }],
            )]),
            ..PresentationCatalog::default()
        };
        assert!(matches!(
            invalid_path.validate(),
            Err(PresentationError::InvalidRendererMaterialBinding { .. })
        ));

        let missing = StableId::new("material:missing").unwrap();
        let dangling = PresentationCatalog {
            schema_version: 7,
            materials: BTreeMap::from([(material, definition)]),
            model_materials: BTreeMap::from([(
                "Assets/Model.fbx".into(),
                BTreeMap::from([("GameMaterial".into(), missing.clone())]),
            )]),
            ..PresentationCatalog::default()
        };
        assert_eq!(
            dangling.validate(),
            Err(PresentationError::MissingModelMaterial {
                model: "Assets/Model.fbx".into(),
                embedded_name: "GameMaterial".into(),
                material: missing,
            })
        );
    }
}
