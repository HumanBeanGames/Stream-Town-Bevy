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
    /// Set only when this exact clip has been converted to a GLB animation.
    pub converted_asset_path: Option<String>,
    pub gltf_animation_index: Option<u32>,
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
    pub motions: Vec<AnimationMotionDef>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationConditionDef {
    pub parameter: String,
    pub mode: u8,
    pub threshold: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationTransitionDef {
    pub source: Option<StableId>,
    pub destination: Option<StableId>,
    pub is_exit: bool,
    pub has_exit_time: bool,
    pub exit_time: f32,
    pub duration: f32,
    pub conditions: Vec<AnimationConditionDef>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationControllerDef {
    pub display_name: String,
    pub source_guid: String,
    pub source_path: String,
    pub parameters: Vec<AnimationParameterDef>,
    pub states: BTreeMap<StableId, AnimationStateDef>,
    pub transitions: Vec<AnimationTransitionDef>,
    /// One default per Unity animation layer/state machine when present.
    pub default_states: Vec<StableId>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PrefabPresentationBinding {
    pub source_prefab_path: String,
    pub controller: StableId,
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
    #[error("prefab {prefab_guid} references missing controller {controller}")]
    MissingController {
        prefab_guid: String,
        controller: StableId,
    },
    #[error("prefab {prefab_guid} has invalid animated scene path {path}")]
    InvalidAnimatedScene { prefab_guid: String, path: String },
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
        }
        for (controller_id, controller) in &self.controllers {
            for default_state in &controller.default_states {
                if !controller.states.contains_key(default_state) {
                    return Err(PresentationError::MissingDefaultState {
                        controller: controller_id.clone(),
                        state: default_state.clone(),
                    });
                }
            }
            for (state_id, state) in &controller.states {
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
        }
        Ok(())
    }
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
