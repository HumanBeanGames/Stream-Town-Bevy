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
    /// Reachable Unity volume profiles converted into engine-neutral settings.
    #[serde(default)]
    pub post_process_profiles: BTreeMap<StableId, PostProcessProfileDef>,
    /// Global volume bindings keyed by the shipping Unity scene path.
    #[serde(default)]
    pub scene_post_process: BTreeMap<String, Vec<ScenePostProcessBinding>>,
    /// Reachable Unity VFX Graph fireworks converted for the Credits sequence.
    #[serde(default)]
    pub fireworks_effects: BTreeMap<StableId, FireworksVfxDef>,
    /// Fireworks instances keyed by the shipping Unity scene path.
    #[serde(default)]
    pub scene_fireworks: BTreeMap<String, Vec<SceneFireworksBinding>>,
    /// Reachable Unity chimney particle systems converted for building presentation.
    #[serde(default)]
    pub chimney_smoke_effects: BTreeMap<StableId, ChimneySmokeDef>,
    /// Chimney emitters keyed by the source building-prefab GUID.
    #[serde(default)]
    pub prefab_chimney_emitters: BTreeMap<String, Vec<PrefabChimneyEmitterBinding>>,
    /// Reachable Fish God mesh-particle effect converted from Unity's built-in particle system.
    #[serde(default)]
    pub raining_fish_effects: BTreeMap<StableId, RainingFishVfxDef>,
    /// Reachable player-healing channel graphs with prefab-exposed overrides applied.
    #[serde(default)]
    pub healing_channel_effects: BTreeMap<StableId, HealingChannelVfxDef>,
    /// Reachable completed-heal graphs with both authored particle systems retained.
    #[serde(default)]
    pub healing_burst_effects: BTreeMap<StableId, HealingBurstVfxDef>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FireworksVfxDef {
    pub display_name: String,
    pub source_guid: String,
    pub source_path: String,
    pub sparks_speed: f32,
    pub launch_rate_per_second: f32,
    pub rocket_capacity: u16,
    pub rocket_lifetime_seconds: [f32; 2],
    pub rocket_velocity_min: [f32; 3],
    pub rocket_velocity_max: [f32; 3],
    pub burst_lifetime_seconds: [f32; 2],
    pub spark_lifetime_seconds: [f32; 2],
    pub burst_particle_rate: u16,
    pub burst_count: [u16; 2],
    pub burst_delay_seconds: [f32; 2],
    /// HDR linear colors from the exposed Unity `FireworkColour` gradient.
    pub colors: Vec<[f32; 4]>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SceneFireworksBinding {
    pub hierarchy_path: String,
    pub effect: StableId,
    pub local_position: [f32; 3],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChimneySmokeDef {
    pub display_name: String,
    pub source_guid: String,
    pub source_path: String,
    pub duration_seconds: f32,
    pub emission_rate_per_second: f32,
    pub lifetime_seconds: f32,
    pub start_speed: f32,
    pub start_size: f32,
    pub start_color_min: [f32; 4],
    pub start_color_max: [f32; 4],
    pub cone_radius: f32,
    pub cone_angle_degrees: f32,
    pub size_over_lifetime: [f32; 2],
    pub alpha_over_lifetime: [f32; 2],
    pub max_particles: u16,
    /// Unity's particle simulation-space enum (`moveWithTransform == 1` is world space).
    pub world_space: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PrefabChimneyEmitterBinding {
    pub hierarchy_path: String,
    pub effect: StableId,
    pub age: u8,
    pub local_position: [f32; 3],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RainingFishVfxDef {
    pub display_name: String,
    pub source_guid: String,
    pub source_path: String,
    pub model_source: String,
    pub model_asset_path: String,
    pub material: StableId,
    pub duration_seconds: f32,
    pub emission_rate_per_second: f32,
    pub lifetime_seconds: f32,
    pub start_size: [f32; 2],
    pub gravity: f32,
    pub max_particles: u16,
    pub emitter_position: [f32; 3],
    pub shape_scale: [f32; 3],
    pub shape_rotation_degrees: [f32; 3],
    pub size_over_lifetime: Vec<AnimationFloatKeyframe>,
    pub noise_strength: [f32; 3],
    pub noise_frequency: f32,
    pub noise_scroll_speed: f32,
    pub collision_bounce: f32,
    pub collision_lifetime_loss: f32,
    pub world_space: bool,
    pub prewarm: bool,
}

impl RainingFishVfxDef {
    /// Samples the authored Unity size-over-lifetime curve at normalized particle age.
    #[must_use]
    pub fn size_multiplier(&self, normalized_age: f32) -> Option<f32> {
        sample_float_keys(&self.size_over_lifetime, normalized_age.clamp(0.0, 1.0))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct VfxGradientDef {
    pub color_keys: Vec<VfxColorKeyframe>,
    pub alpha_keys: Vec<VfxAlphaKeyframe>,
}

impl VfxGradientDef {
    /// Samples Unity's linear gradient representation in linear HDR color space.
    #[must_use]
    pub fn sample(&self, normalized_time: f32) -> Option<[f32; 4]> {
        let time = normalized_time.clamp(0.0, 1.0);
        let color = sample_vfx_color_keys(&self.color_keys, time)?;
        let alpha = sample_vfx_alpha_keys(&self.alpha_keys, time)?;
        Some([color[0], color[1], color[2], alpha])
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct VfxColorKeyframe {
    pub time: f32,
    /// Unity VFX Graph colors are retained as linear HDR values.
    pub color: [f32; 3],
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct VfxAlphaKeyframe {
    pub time: f32,
    pub alpha: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HealingChannelVfxDef {
    pub display_name: String,
    pub source_guid: String,
    pub source_path: String,
    pub graph_guid: String,
    pub graph_source: String,
    pub particle_capacity: u16,
    pub emission_rate_per_second: f32,
    pub particle_lifetime_seconds: [f32; 2],
    pub exposed_size: f32,
    /// Prefab override curve; unlike particle-age curves, its keys are authored in seconds.
    pub size_over_lifetime: Vec<AnimationFloatKeyframe>,
    pub color: VfxGradientDef,
}

impl HealingChannelVfxDef {
    #[must_use]
    pub fn duration_seconds(&self) -> Option<f32> {
        self.size_over_lifetime.last().map(|key| key.time)
    }

    #[must_use]
    pub fn size_multiplier(&self, elapsed_seconds: f32) -> Option<f32> {
        sample_float_keys(&self.size_over_lifetime, elapsed_seconds.max(0.0))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HealingBurstVfxDef {
    pub display_name: String,
    pub source_guid: String,
    pub source_path: String,
    pub graph_guid: String,
    pub graph_source: String,
    pub duration_seconds: f32,
    pub plus_capacity: u16,
    pub plus_burst_count: u16,
    pub plus_lifetime_seconds: [f32; 2],
    pub plus_size_over_lifetime: Vec<AnimationFloatKeyframe>,
    pub plus_color: VfxGradientDef,
    pub plus_model_source: String,
    pub plus_model_asset_path: String,
    pub disc_capacity: u16,
    pub disc_burst_count: u16,
    pub disc_lifetime_seconds: f32,
    pub disc_size_multiplier: f32,
    pub disc_size_over_lifetime: Vec<AnimationFloatKeyframe>,
    pub disc_color: VfxGradientDef,
    pub disc_texture: StableId,
}

impl HealingBurstVfxDef {
    #[must_use]
    pub fn plus_size_multiplier(&self, normalized_age: f32) -> Option<f32> {
        sample_float_keys(
            &self.plus_size_over_lifetime,
            normalized_age.clamp(0.0, 1.0),
        )
    }

    #[must_use]
    pub fn disc_size_multiplier_at(&self, normalized_age: f32) -> Option<f32> {
        sample_float_keys(
            &self.disc_size_over_lifetime,
            normalized_age.clamp(0.0, 1.0),
        )
        .map(|value| value * self.disc_size_multiplier)
    }
}

fn sample_vfx_color_keys(keys: &[VfxColorKeyframe], time: f32) -> Option<[f32; 3]> {
    let first = keys.first()?;
    if time <= first.time {
        return Some(first.color);
    }
    let last = keys.last()?;
    if time >= last.time {
        return Some(last.color);
    }
    let pair = keys
        .windows(2)
        .find(|pair| time >= pair[0].time && time < pair[1].time)?;
    let progress = (time - pair[0].time) / (pair[1].time - pair[0].time);
    Some(std::array::from_fn(|axis| {
        pair[0].color[axis] + (pair[1].color[axis] - pair[0].color[axis]) * progress
    }))
}

fn sample_vfx_alpha_keys(keys: &[VfxAlphaKeyframe], time: f32) -> Option<f32> {
    let first = keys.first()?;
    if time <= first.time {
        return Some(first.alpha);
    }
    let last = keys.last()?;
    if time >= last.time {
        return Some(last.alpha);
    }
    let pair = keys
        .windows(2)
        .find(|pair| time >= pair[0].time && time < pair[1].time)?;
    let progress = (time - pair[0].time) / (pair[1].time - pair[0].time);
    Some(pair[0].alpha + (pair[1].alpha - pair[0].alpha) * progress)
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PostProcessProfileDef {
    pub display_name: String,
    pub source_guid: String,
    pub source_path: String,
    pub bloom: Option<PostProcessBloomDef>,
    pub vignette: Option<PostProcessVignetteDef>,
    pub motion_blur: Option<PostProcessMotionBlurDef>,
    pub tonemapping: Option<PostProcessTonemapping>,
    pub color_adjustments: Option<PostProcessColorAdjustmentsDef>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct PostProcessBloomDef {
    pub intensity: f32,
    pub threshold: f32,
    pub scatter: f32,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct PostProcessVignetteDef {
    pub color: [f32; 4],
    pub center: [f32; 2],
    pub intensity: f32,
    pub smoothness: f32,
    pub rounded: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct PostProcessMotionBlurDef {
    pub intensity: f32,
    pub quality: u8,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostProcessTonemapping {
    None,
    Neutral,
    Aces,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct PostProcessColorAdjustmentsDef {
    pub post_exposure: f32,
    pub color_filter: [f32; 4],
    pub hue_shift_degrees: f32,
    pub saturation: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ScenePostProcessBinding {
    pub hierarchy_path: String,
    pub profile: StableId,
    pub weight: f32,
    /// Unity's day/night processor drives the night volume opposite daylight.
    #[serde(default)]
    pub inverse_daylight: bool,
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
    /// Unity texture-environment tiling and offset, keyed by shader property.
    #[serde(default)]
    pub texture_transforms: BTreeMap<String, TextureTransform>,
    /// Unity properties that need a custom WGSL material rather than Bevy PBR.
    #[serde(default)]
    pub custom_properties: BTreeMap<String, f32>,
    /// Unity color/vector properties retained for custom WGSL materials.
    #[serde(default)]
    pub custom_vectors: BTreeMap<String, [f32; 4]>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct TextureTransform {
    pub scale: [f32; 2],
    pub offset: [f32; 2],
}

impl Default for TextureTransform {
    fn default() -> Self {
        Self {
            scale: [1.0; 2],
            offset: [0.0; 2],
        }
    }
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
    /// Samples Unity's constant, unweighted Hermite, and weighted Bezier semantics.
    #[must_use]
    pub fn sample(&self, time: f32) -> Option<f32> {
        sample_float_keys(&self.keys, time)
    }
}

fn sample_float_keys(keys: &[AnimationFloatKeyframe], time: f32) -> Option<f32> {
    let first = keys.first()?;
    if time <= first.time {
        return Some(first.value);
    }
    let last = keys.last()?;
    if time >= last.time {
        return Some(last.value);
    }
    let pair = keys
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
    let out_weight = if left.weighted_mode & 2 != 0 {
        left.out_weight
    } else {
        1.0 / 3.0
    };
    let in_weight = if right.weighted_mode & 1 != 0 {
        right.in_weight
    } else {
        1.0 / 3.0
    };
    if left.weighted_mode & 2 != 0 || right.weighted_mode & 1 != 0 {
        if out_weight <= f32::EPSILON && in_weight <= f32::EPSILON {
            return Some(left.value + (right.value - left.value) * t);
        }
        let parameter = solve_weighted_curve_parameter(t, out_weight, in_weight);
        return Some(cubic_bezier(
            left.value,
            left.value + left.out_slope.finite_value() * duration * out_weight,
            right.value - right.in_slope.finite_value() * duration * in_weight,
            right.value,
            parameter,
        ));
    }
    let t2 = t * t;
    let t3 = t2 * t;
    Some(
        (2.0 * t3 - 3.0 * t2 + 1.0) * left.value
            + (t3 - 2.0 * t2 + t) * duration * left.out_slope.finite_value()
            + (-2.0 * t3 + 3.0 * t2) * right.value
            + (t3 - t2) * duration * right.in_slope.finite_value(),
    )
}

fn solve_weighted_curve_parameter(time: f32, out_weight: f32, in_weight: f32) -> f32 {
    let mut low = 0.0;
    let mut high = 1.0;
    for _ in 0..24 {
        let parameter = (low + high) * 0.5;
        let sampled_time = cubic_bezier(0.0, out_weight, 1.0 - in_weight, 1.0, parameter);
        if sampled_time < time {
            low = parameter;
        } else {
            high = parameter;
        }
    }
    (low + high) * 0.5
}

fn cubic_bezier(start: f32, control_a: f32, control_b: f32, end: f32, time: f32) -> f32 {
    let inverse = 1.0 - time;
    inverse * inverse * inverse * start
        + 3.0 * inverse * inverse * time * control_a
        + 3.0 * inverse * time * time * control_b
        + time * time * time * end
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
    #[error("material {material} has an invalid custom shader parameter {property}")]
    InvalidMaterialParameter {
        material: StableId,
        property: String,
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
    #[error("post-process profile {profile} is invalid: {reason}")]
    InvalidPostProcessProfile { profile: StableId, reason: String },
    #[error("scene {scene} references missing post-process profile {profile}")]
    MissingPostProcessProfile { scene: String, profile: StableId },
    #[error("scene {scene} has an invalid post-process binding at {hierarchy_path}: {reason}")]
    InvalidPostProcessBinding {
        scene: String,
        hierarchy_path: String,
        reason: String,
    },
    #[error("fireworks effect {effect} is invalid: {reason}")]
    InvalidFireworksEffect { effect: StableId, reason: String },
    #[error("scene {scene} has an invalid fireworks binding at {hierarchy_path}: {reason}")]
    InvalidFireworksBinding {
        scene: String,
        hierarchy_path: String,
        reason: String,
    },
    #[error("scene {scene} references missing fireworks effect {effect}")]
    MissingFireworksEffect { scene: String, effect: StableId },
    #[error("chimney-smoke effect {effect} is invalid: {reason}")]
    InvalidChimneySmokeEffect { effect: StableId, reason: String },
    #[error("prefab {prefab_guid} references missing chimney-smoke effect {effect}")]
    MissingChimneySmokeEffect {
        prefab_guid: String,
        effect: StableId,
    },
    #[error("prefab {prefab_guid} has an invalid chimney emitter at {hierarchy_path}: {reason}")]
    InvalidChimneyEmitterBinding {
        prefab_guid: String,
        hierarchy_path: String,
        reason: String,
    },
    #[error("raining-fish effect {effect} is invalid: {reason}")]
    InvalidRainingFishEffect { effect: StableId, reason: String },
    #[error("raining-fish effect {effect} references missing material {material}")]
    MissingRainingFishMaterial {
        effect: StableId,
        material: StableId,
    },
    #[error("healing-channel effect {effect} is invalid: {reason}")]
    InvalidHealingChannelEffect { effect: StableId, reason: String },
    #[error("healing-burst effect {effect} is invalid: {reason}")]
    InvalidHealingBurstEffect { effect: StableId, reason: String },
    #[error("healing-burst effect {effect} references missing texture {texture}")]
    MissingHealingBurstTexture { effect: StableId, texture: StableId },
}

impl PresentationCatalog {
    pub fn validate(&self) -> Result<(), PresentationError> {
        for (id, effect) in &self.healing_channel_effects {
            let valid = valid_unity_source(&effect.source_guid, &effect.source_path, "prefab")
                && valid_unity_source(&effect.graph_guid, &effect.graph_source, "vfx")
                && effect.particle_capacity > 0
                && finite_positive(effect.emission_rate_per_second)
                && ordered_positive_range(effect.particle_lifetime_seconds)
                && finite_positive(effect.exposed_size)
                && valid_float_curve(&effect.size_over_lifetime, false)
                && effect
                    .size_over_lifetime
                    .first()
                    .is_some_and(|key| key.time.abs() < f32::EPSILON)
                && effect.duration_seconds().is_some_and(finite_positive)
                && valid_vfx_gradient(&effect.color);
            if !valid {
                return Err(PresentationError::InvalidHealingChannelEffect {
                    effect: id.clone(),
                    reason: "source metadata, emission, lifetime, size curve, and gradient must be portable and valid".into(),
                });
            }
        }
        for (id, effect) in &self.healing_burst_effects {
            if !self.textures.contains_key(&effect.disc_texture) {
                return Err(PresentationError::MissingHealingBurstTexture {
                    effect: id.clone(),
                    texture: effect.disc_texture.clone(),
                });
            }
            let portable_path = |path: &str| {
                !path.contains('\\') && !path.split('/').any(|component| component == "..")
            };
            let valid = valid_unity_source(&effect.source_guid, &effect.source_path, "prefab")
                && valid_unity_source(&effect.graph_guid, &effect.graph_source, "vfx")
                && effect.plus_model_source.starts_with("Assets/")
                && effect.plus_model_asset_path.starts_with("migrated/models/")
                && portable_path(&effect.plus_model_source)
                && portable_path(&effect.plus_model_asset_path)
                && finite_positive(effect.duration_seconds)
                && effect.plus_capacity > 0
                && effect.plus_burst_count > 0
                && ordered_positive_range(effect.plus_lifetime_seconds)
                && valid_float_curve(&effect.plus_size_over_lifetime, true)
                && valid_vfx_gradient(&effect.plus_color)
                && effect.disc_capacity > 0
                && effect.disc_burst_count > 0
                && finite_positive(effect.disc_lifetime_seconds)
                && finite_positive(effect.disc_size_multiplier)
                && valid_float_curve(&effect.disc_size_over_lifetime, true)
                && valid_vfx_gradient(&effect.disc_color);
            if !valid {
                return Err(PresentationError::InvalidHealingBurstEffect {
                    effect: id.clone(),
                    reason: "source metadata, converted dependencies, budgets, lifetimes, curves, and gradients must be portable and valid".into(),
                });
            }
        }
        for (id, effect) in &self.raining_fish_effects {
            if !self.materials.contains_key(&effect.material) {
                return Err(PresentationError::MissingRainingFishMaterial {
                    effect: id.clone(),
                    material: effect.material.clone(),
                });
            }
            let portable_path = |path: &str| {
                !path.contains('\\') && !path.split('/').any(|component| component == "..")
            };
            let source_valid = effect.source_guid.len() == 32
                && effect
                    .source_guid
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit())
                && effect.source_path.starts_with("Assets/")
                && effect.model_source.starts_with("Assets/")
                && effect.model_asset_path.starts_with("migrated/models/")
                && portable_path(&effect.source_path)
                && portable_path(&effect.model_source)
                && portable_path(&effect.model_asset_path);
            let finite_positive = |value: f32| value.is_finite() && value > 0.0;
            let curve_valid = !effect.size_over_lifetime.is_empty()
                && effect
                    .size_over_lifetime
                    .windows(2)
                    .all(|pair| pair[1].time > pair[0].time)
                && effect.size_over_lifetime.iter().all(|key| {
                    key.time.is_finite()
                        && key.value.is_finite()
                        && key.value >= 0.0
                        && key.in_slope.is_finite()
                        && key.out_slope.is_finite()
                        && key.in_weight.is_finite()
                        && key.out_weight.is_finite()
                });
            let values_valid = finite_positive(effect.duration_seconds)
                && finite_positive(effect.emission_rate_per_second)
                && finite_positive(effect.lifetime_seconds)
                && effect.start_size.into_iter().all(finite_positive)
                && effect.start_size[1] >= effect.start_size[0]
                && finite_positive(effect.gravity)
                && effect.max_particles > 0
                && effect
                    .emitter_position
                    .into_iter()
                    .chain(effect.shape_rotation_degrees)
                    .chain(effect.noise_strength)
                    .all(f32::is_finite)
                && effect.shape_scale.into_iter().all(finite_positive)
                && finite_positive(effect.noise_frequency)
                && effect.noise_scroll_speed.is_finite()
                && effect.noise_scroll_speed >= 0.0
                && effect.collision_bounce.is_finite()
                && (0.0..=1.0).contains(&effect.collision_bounce)
                && effect.collision_lifetime_loss.is_finite()
                && (0.0..=1.0).contains(&effect.collision_lifetime_loss)
                && curve_valid;
            if !source_valid || !values_valid {
                return Err(PresentationError::InvalidRainingFishEffect {
                    effect: id.clone(),
                    reason: "source metadata, mesh particle parameters, curve, noise, and collision must be portable and valid".into(),
                });
            }
        }
        for (id, effect) in &self.chimney_smoke_effects {
            let source_valid = effect.source_guid.len() == 32
                && effect
                    .source_guid
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit())
                && effect.source_path.starts_with("Assets/")
                && Path::new(&effect.source_path)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("prefab"))
                && !effect.source_path.contains('\\')
                && !effect.source_path.contains("..");
            let positive = |value: f32| value.is_finite() && value > 0.0;
            let valid = positive(effect.duration_seconds)
                && positive(effect.emission_rate_per_second)
                && positive(effect.lifetime_seconds)
                && positive(effect.start_speed)
                && positive(effect.start_size)
                && effect
                    .start_color_min
                    .into_iter()
                    .chain(effect.start_color_max)
                    .all(|value| value.is_finite() && value >= 0.0)
                && positive(effect.cone_radius)
                && effect.cone_angle_degrees.is_finite()
                && (0.0..=90.0).contains(&effect.cone_angle_degrees)
                && effect
                    .size_over_lifetime
                    .into_iter()
                    .chain(effect.alpha_over_lifetime)
                    .all(|value| value.is_finite() && value >= 0.0)
                && effect.max_particles > 0;
            if !source_valid || !valid {
                return Err(PresentationError::InvalidChimneySmokeEffect {
                    effect: id.clone(),
                    reason: "source metadata and particle parameters must be portable and valid"
                        .into(),
                });
            }
        }
        for (prefab_guid, bindings) in &self.prefab_chimney_emitters {
            let prefab_valid =
                prefab_guid.len() == 32 && prefab_guid.bytes().all(|byte| byte.is_ascii_hexdigit());
            for binding in bindings {
                if !self.chimney_smoke_effects.contains_key(&binding.effect) {
                    return Err(PresentationError::MissingChimneySmokeEffect {
                        prefab_guid: prefab_guid.clone(),
                        effect: binding.effect.clone(),
                    });
                }
                if !prefab_valid
                    || binding.hierarchy_path.trim().is_empty()
                    || binding.hierarchy_path.contains('\\')
                    || binding.age == 0
                    || binding
                        .local_position
                        .into_iter()
                        .any(|value| !value.is_finite())
                {
                    return Err(PresentationError::InvalidChimneyEmitterBinding {
                        prefab_guid: prefab_guid.clone(),
                        hierarchy_path: binding.hierarchy_path.clone(),
                        reason: "prefab GUID, hierarchy path, age, and position must be valid"
                            .into(),
                    });
                }
            }
        }
        for (id, effect) in &self.fireworks_effects {
            let source_valid = effect.source_guid.len() == 32
                && effect
                    .source_guid
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit())
                && effect.source_path.starts_with("Assets/")
                && Path::new(&effect.source_path)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("vfx"))
                && !effect.source_path.contains('\\')
                && !effect.source_path.contains("..");
            let ordered_positive = |range: [f32; 2]| {
                range.into_iter().all(f32::is_finite) && range[0] > 0.0 && range[1] >= range[0]
            };
            let values_valid = effect.sparks_speed.is_finite()
                && effect.sparks_speed > 0.0
                && effect.launch_rate_per_second.is_finite()
                && effect.launch_rate_per_second > 0.0
                && effect.rocket_capacity > 0
                && ordered_positive(effect.rocket_lifetime_seconds)
                && effect.rocket_velocity_min.into_iter().all(f32::is_finite)
                && effect.rocket_velocity_max.into_iter().all(f32::is_finite)
                && effect
                    .rocket_velocity_min
                    .into_iter()
                    .zip(effect.rocket_velocity_max)
                    .all(|(minimum, maximum)| maximum >= minimum)
                && ordered_positive(effect.burst_lifetime_seconds)
                && ordered_positive(effect.spark_lifetime_seconds)
                && effect.burst_particle_rate > 0
                && effect.burst_count[0] > 0
                && effect.burst_count[1] >= effect.burst_count[0]
                && effect.burst_delay_seconds.into_iter().all(f32::is_finite)
                && effect.burst_delay_seconds[0] >= 0.0
                && effect.burst_delay_seconds[1] >= effect.burst_delay_seconds[0]
                && !effect.colors.is_empty()
                && effect
                    .colors
                    .iter()
                    .flatten()
                    .copied()
                    .all(|value| value.is_finite() && value >= 0.0);
            if !source_valid || !values_valid {
                return Err(PresentationError::InvalidFireworksEffect {
                    effect: id.clone(),
                    reason: "source metadata, ranges, rates, and colors must be portable and valid"
                        .into(),
                });
            }
        }
        for (scene, bindings) in &self.scene_fireworks {
            let scene_valid = scene.starts_with("Assets/")
                && Path::new(scene)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("unity"))
                && !scene.contains('\\')
                && !scene.contains("..");
            for binding in bindings {
                if !self.fireworks_effects.contains_key(&binding.effect) {
                    return Err(PresentationError::MissingFireworksEffect {
                        scene: scene.clone(),
                        effect: binding.effect.clone(),
                    });
                }
                if !scene_valid
                    || binding.hierarchy_path.trim().is_empty()
                    || binding.hierarchy_path.contains('\\')
                    || binding
                        .local_position
                        .into_iter()
                        .any(|value| !value.is_finite())
                {
                    return Err(PresentationError::InvalidFireworksBinding {
                        scene: scene.clone(),
                        hierarchy_path: binding.hierarchy_path.clone(),
                        reason:
                            "scene path, hierarchy path, and position must be portable and valid"
                                .into(),
                    });
                }
            }
        }
        for (id, profile) in &self.post_process_profiles {
            let source_valid = profile.source_guid.len() == 32
                && profile
                    .source_guid
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit())
                && profile.source_path.starts_with("Assets/")
                && Path::new(&profile.source_path)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("asset"))
                && !profile.source_path.contains('\\')
                && !profile.source_path.contains("..");
            let values_valid = profile.bloom.is_none_or(|bloom| {
                [bloom.intensity, bloom.threshold, bloom.scatter]
                    .into_iter()
                    .all(f32::is_finite)
                    && bloom.intensity >= 0.0
                    && bloom.threshold >= 0.0
                    && (0.0..=1.0).contains(&bloom.scatter)
            }) && profile.vignette.is_none_or(|vignette| {
                vignette.color.into_iter().all(f32::is_finite)
                    && vignette.center.into_iter().all(f32::is_finite)
                    && vignette.intensity.is_finite()
                    && (0.0..=1.0).contains(&vignette.intensity)
                    && vignette.smoothness.is_finite()
                    && vignette.smoothness > 0.0
            }) && profile.motion_blur.is_none_or(|motion_blur| {
                motion_blur.intensity.is_finite()
                    && (0.0..=1.0).contains(&motion_blur.intensity)
                    && motion_blur.quality > 0
            }) && profile.color_adjustments.is_none_or(|adjustments| {
                adjustments.post_exposure.is_finite()
                    && adjustments.color_filter.into_iter().all(f32::is_finite)
                    && adjustments.hue_shift_degrees.is_finite()
                    && adjustments.saturation.is_finite()
            });
            if !source_valid || !values_valid {
                return Err(PresentationError::InvalidPostProcessProfile {
                    profile: id.clone(),
                    reason: "source metadata and numeric parameters must be portable and finite"
                        .into(),
                });
            }
        }
        for (scene, bindings) in &self.scene_post_process {
            if !scene.starts_with("Assets/")
                || !Path::new(scene)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("unity"))
                || scene.contains('\\')
                || scene.contains("..")
            {
                return Err(PresentationError::InvalidPostProcessBinding {
                    scene: scene.clone(),
                    hierarchy_path: String::new(),
                    reason: "scene path must be a portable Unity asset path".into(),
                });
            }
            for binding in bindings {
                if !self.post_process_profiles.contains_key(&binding.profile) {
                    return Err(PresentationError::MissingPostProcessProfile {
                        scene: scene.clone(),
                        profile: binding.profile.clone(),
                    });
                }
                if binding.hierarchy_path.trim().is_empty()
                    || binding.hierarchy_path.contains('\\')
                    || !binding.weight.is_finite()
                    || !(0.0..=1.0).contains(&binding.weight)
                {
                    return Err(PresentationError::InvalidPostProcessBinding {
                        scene: scene.clone(),
                        hierarchy_path: binding.hierarchy_path.clone(),
                        reason: "hierarchy path and weight must be valid".into(),
                    });
                }
            }
        }
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
            if let Some(property) = material
                .custom_properties
                .iter()
                .find_map(|(property, value)| (!value.is_finite()).then_some(property))
                .or_else(|| {
                    material
                        .custom_vectors
                        .iter()
                        .find_map(|(property, value)| {
                            (!value.iter().all(|component| component.is_finite()))
                                .then_some(property)
                        })
                })
            {
                return Err(PresentationError::InvalidMaterialParameter {
                    material: id.clone(),
                    property: property.clone(),
                });
            }
            if let Some(property) =
                material
                    .texture_transforms
                    .iter()
                    .find_map(|(property, transform)| {
                        (!material.textures.contains_key(property)
                            || !transform.scale.into_iter().all(f32::is_finite)
                            || !transform.offset.into_iter().all(f32::is_finite))
                        .then_some(property)
                    })
            {
                return Err(PresentationError::InvalidMaterialParameter {
                    material: id.clone(),
                    property: property.clone(),
                });
            }
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
                && (0.0..=1.0).contains(&key.in_weight)
                && (0.0..=1.0).contains(&key.out_weight)
                && key.weighted_mode <= 3
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

fn finite_positive(value: f32) -> bool {
    value.is_finite() && value > 0.0
}

fn ordered_positive_range(range: [f32; 2]) -> bool {
    range.into_iter().all(finite_positive) && range[1] >= range[0]
}

fn valid_unity_source(guid: &str, path: &str, extension: &str) -> bool {
    guid.len() == 32
        && guid.bytes().all(|byte| byte.is_ascii_hexdigit())
        && path.starts_with("Assets/")
        && Path::new(path)
            .extension()
            .is_some_and(|candidate| candidate.eq_ignore_ascii_case(extension))
        && !path.contains('\\')
        && !path.split('/').any(|component| component == "..")
}

fn valid_float_curve(keys: &[AnimationFloatKeyframe], normalized: bool) -> bool {
    !keys.is_empty()
        && keys.windows(2).all(|pair| pair[1].time > pair[0].time)
        && keys.iter().all(|key| {
            key.time.is_finite()
                && key.time >= 0.0
                && (!normalized || key.time <= 1.0)
                && key.value.is_finite()
                && key.value >= 0.0
                && key.in_slope.is_finite()
                && key.out_slope.is_finite()
                && key.in_weight.is_finite()
                && key.out_weight.is_finite()
        })
}

fn valid_vfx_gradient(gradient: &VfxGradientDef) -> bool {
    !gradient.color_keys.is_empty()
        && !gradient.alpha_keys.is_empty()
        && gradient
            .color_keys
            .windows(2)
            .all(|pair| pair[1].time > pair[0].time)
        && gradient
            .alpha_keys
            .windows(2)
            .all(|pair| pair[1].time > pair[0].time)
        && gradient.color_keys.iter().all(|key| {
            (0.0..=1.0).contains(&key.time)
                && key
                    .color
                    .into_iter()
                    .all(|value| value.is_finite() && value >= 0.0)
        })
        && gradient.alpha_keys.iter().all(|key| {
            (0.0..=1.0).contains(&key.time)
                && key.alpha.is_finite()
                && (0.0..=1.0).contains(&key.alpha)
        })
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
                    texture_transforms: BTreeMap::new(),
                    custom_properties: BTreeMap::new(),
                    custom_vectors: BTreeMap::new(),
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
    fn property_curve_samples_weighted_bezier_segments() {
        let mut curve = AnimationPropertyCurve {
            target_path: "Panel".into(),
            attribute: "m_Alpha".into(),
            class_id: 225,
            script_guid: None,
            keys: vec![
                AnimationFloatKeyframe {
                    time: 0.0,
                    value: 0.0,
                    in_slope: AnimationTangent::Finite(0.0),
                    out_slope: AnimationTangent::Finite(2.0),
                    tangent_mode: 0,
                    weighted_mode: 2,
                    in_weight: 1.0 / 3.0,
                    out_weight: 0.5,
                },
                AnimationFloatKeyframe {
                    time: 1.0,
                    value: 1.0,
                    in_slope: AnimationTangent::Finite(0.0),
                    out_slope: AnimationTangent::Finite(0.0),
                    tangent_mode: 0,
                    weighted_mode: 1,
                    in_weight: 0.25,
                    out_weight: 1.0 / 3.0,
                },
            ],
        };
        assert!((curve.sample(0.593_75).unwrap() - 0.875).abs() < 0.000_01);

        curve.keys[0].out_weight = 0.0;
        curve.keys[1].in_weight = 0.0;
        assert!((curve.sample(0.25).unwrap() - 0.25).abs() < f32::EPSILON);
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
    fn validates_portable_fireworks_settings() {
        let effect_id = StableId::new("vfx:fireworks").unwrap();
        let effect = FireworksVfxDef {
            display_name: "Fireworks".into(),
            source_guid: "a".repeat(32),
            source_path: "Assets/VFX/fireworks.vfx".into(),
            sparks_speed: 2.0,
            launch_rate_per_second: 16.0,
            rocket_capacity: 8,
            rocket_lifetime_seconds: [0.75, 1.0],
            rocket_velocity_min: [-1.0, 12.0, -1.0],
            rocket_velocity_max: [1.0, 16.0, 1.0],
            burst_lifetime_seconds: [0.5, 1.0],
            spark_lifetime_seconds: [1.0, 2.0],
            burst_particle_rate: 30,
            burst_count: [1, 3],
            burst_delay_seconds: [1.0, 2.0],
            colors: vec![[10.0, 1.0, 0.0, 1.0]],
        };
        let catalog = PresentationCatalog {
            schema_version: 15,
            fireworks_effects: BTreeMap::from([(effect_id.clone(), effect.clone())]),
            scene_fireworks: BTreeMap::from([(
                "Assets/Scenes/Menu/Credits.unity".into(),
                vec![SceneFireworksBinding {
                    hierarchy_path: "VFX_FireWorks".into(),
                    effect: effect_id.clone(),
                    local_position: [-23.94, 12.81, -24.78],
                }],
            )]),
            ..PresentationCatalog::default()
        };
        assert_eq!(catalog.validate(), Ok(()));

        let invalid = PresentationCatalog {
            schema_version: 15,
            fireworks_effects: BTreeMap::from([(
                effect_id,
                FireworksVfxDef {
                    launch_rate_per_second: 0.0,
                    ..effect
                },
            )]),
            ..PresentationCatalog::default()
        };
        assert!(matches!(
            invalid.validate(),
            Err(PresentationError::InvalidFireworksEffect { .. })
        ));
    }

    #[test]
    fn validates_chimney_smoke_and_rejects_dangling_emitters() {
        let effect_id = StableId::new("particle_effect:chimney").unwrap();
        let effect = ChimneySmokeDef {
            display_name: "Chimney Smoke".into(),
            source_guid: "b".repeat(32),
            source_path: "Assets/Prefabs/VFX/Environment/VFX_Chimney_Smoke.prefab".into(),
            duration_seconds: 5.0,
            emission_rate_per_second: 5.0,
            lifetime_seconds: 5.0,
            start_speed: 1.0,
            start_size: 0.25,
            start_color_min: [0.886, 0.886, 0.886, 0.568],
            start_color_max: [0.584, 0.584, 0.584, 0.537],
            cone_radius: 0.11,
            cone_angle_degrees: 6.1,
            size_over_lifetime: [0.513_157_84, 1.0],
            alpha_over_lifetime: [1.0, 0.0],
            max_particles: 1_000,
            world_space: false,
        };
        let binding = PrefabChimneyEmitterBinding {
            hierarchy_path: "Age01_House/Age01_House_Base/VFX_Chimney_Smoke".into(),
            effect: effect_id.clone(),
            age: 1,
            local_position: [-0.373, 2.201, -0.25],
        };
        let catalog = PresentationCatalog {
            schema_version: 16,
            chimney_smoke_effects: BTreeMap::from([(effect_id.clone(), effect)]),
            prefab_chimney_emitters: BTreeMap::from([("a".repeat(32), vec![binding.clone()])]),
            ..PresentationCatalog::default()
        };
        assert_eq!(catalog.validate(), Ok(()));

        let dangling = PresentationCatalog {
            schema_version: 16,
            prefab_chimney_emitters: BTreeMap::from([("a".repeat(32), vec![binding])]),
            ..PresentationCatalog::default()
        };
        assert!(matches!(
            dangling.validate(),
            Err(PresentationError::MissingChimneySmokeEffect { .. })
        ));
    }

    #[test]
    fn validates_raining_fish_effect_and_samples_authored_size_curve() {
        let material = StableId::new("material:fish").unwrap();
        let effect_id = StableId::new("particle_effect:fish").unwrap();
        let effect = RainingFishVfxDef {
            display_name: "VFX_RainingFish".into(),
            source_guid: "a".repeat(32),
            source_path: "Assets/Prefabs/VFX/Environment/VFX_RainingFish.prefab".into(),
            model_source: "Assets/Models/Critters/Critter_Fish3.fbx".into(),
            model_asset_path: "migrated/models/Models/Critters/Critter_Fish3.glb".into(),
            material: material.clone(),
            duration_seconds: 15.0,
            emission_rate_per_second: 500.0,
            lifetime_seconds: 15.0,
            start_size: [0.2, 1.0],
            gravity: 9.8,
            max_particles: 5_000,
            emitter_position: [0.0, 46.2, 0.0],
            shape_scale: [300.0, 300.0, 5.0],
            shape_rotation_degrees: [-90.0, 0.0, 0.0],
            size_over_lifetime: vec![
                AnimationFloatKeyframe {
                    time: 0.0,
                    value: 1.0,
                    in_slope: AnimationTangent::Finite(0.0),
                    out_slope: AnimationTangent::Finite(0.0),
                    tangent_mode: 0,
                    weighted_mode: 0,
                    in_weight: 0.0,
                    out_weight: 0.0,
                },
                AnimationFloatKeyframe {
                    time: 1.0,
                    value: 0.0,
                    in_slope: AnimationTangent::Finite(-2.0),
                    out_slope: AnimationTangent::Finite(-2.0),
                    tangent_mode: 0,
                    weighted_mode: 0,
                    in_weight: 0.0,
                    out_weight: 0.0,
                },
            ],
            noise_strength: [1.0, 0.02, 1.0],
            noise_frequency: 0.13,
            noise_scroll_speed: 0.15,
            collision_bounce: 0.5,
            collision_lifetime_loss: 0.25,
            world_space: true,
            prewarm: true,
        };
        let catalog = PresentationCatalog {
            schema_version: 17,
            materials: BTreeMap::from([(
                material,
                MaterialDef {
                    display_name: "Fish".into(),
                    source_guid: "b".repeat(32),
                    source_path: "Assets/Materials/Fish.mat".into(),
                    shader_source: None,
                    base_color: [1.0; 4],
                    emissive: [0.0; 4],
                    metallic: 0.0,
                    perceptual_roughness: 1.0,
                    alpha_mode: MaterialAlphaMode::Opaque,
                    textures: BTreeMap::new(),
                    texture_transforms: BTreeMap::new(),
                    custom_properties: BTreeMap::new(),
                    custom_vectors: BTreeMap::new(),
                },
            )]),
            raining_fish_effects: BTreeMap::from([(effect_id, effect.clone())]),
            ..PresentationCatalog::default()
        };
        assert_eq!(catalog.validate(), Ok(()));
        assert_eq!(effect.size_multiplier(0.0), Some(1.0));
        assert_eq!(effect.size_multiplier(1.0), Some(0.0));
    }

    #[test]
    fn vfx_gradient_samples_hdr_color_and_independent_alpha() {
        let gradient = VfxGradientDef {
            color_keys: vec![
                VfxColorKeyframe {
                    time: 0.0,
                    color: [2.0, 4.0, 0.0],
                },
                VfxColorKeyframe {
                    time: 1.0,
                    color: [4.0, 2.0, 0.0],
                },
            ],
            alpha_keys: vec![
                VfxAlphaKeyframe {
                    time: 0.0,
                    alpha: 0.0,
                },
                VfxAlphaKeyframe {
                    time: 0.5,
                    alpha: 1.0,
                },
                VfxAlphaKeyframe {
                    time: 1.0,
                    alpha: 0.0,
                },
            ],
        };
        assert_eq!(gradient.sample(0.5), Some([3.0, 3.0, 0.0, 1.0]));
        assert_eq!(gradient.sample(1.0), Some([4.0, 2.0, 0.0, 0.0]));
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
            texture_transforms: BTreeMap::new(),
            custom_properties: BTreeMap::new(),
            custom_vectors: BTreeMap::new(),
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
