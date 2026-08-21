use serde::{Deserialize, Serialize};

use crate::StableId;

/// Authored Unity main-menu scene content converted into Bevy's right-handed,
/// Y-up coordinate system. Positions and normals use -Z forward, rotations are
/// already reflected, and embedded triangle winding is counter-clockwise.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MainMenuSceneReference {
    pub schema_version: u32,
    pub source_scene: String,
    pub camera: MainMenuCameraReference,
    pub instances: Vec<MainMenuModelInstance>,
    pub embedded_meshes: Vec<MainMenuEmbeddedMesh>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corrective_bake: Option<MainMenuCorrectiveBake>,
}

/// Deterministic, once-off presentation data baked from the shipping world
/// generator. Runtime menu loading only deserializes this data; it never runs
/// terrain, resource, or foliage generation on the main thread.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MainMenuCorrectiveBake {
    pub version: u32,
    pub seed: u64,
    pub generator_version: u32,
    pub generator_hash: String,
    pub cell_size: f32,
    pub water_height: f32,
    pub water_extent: f32,
    pub ocean_floor_height: f32,
    pub terrain_extent: f32,
    pub resources: Vec<MainMenuResourceVisual>,
    pub foliage: Vec<MainMenuFoliageVisual>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MainMenuResourceVisual {
    pub id: StableId,
    pub kind: StableId,
    pub target_kind: StableId,
    pub position: [f32; 3],
    pub mesh_index: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MainMenuFoliageVisual {
    pub id: StableId,
    pub layer: StableId,
    pub variant: u16,
    pub position: [f32; 3],
    pub yaw_milliradians: u16,
    pub scale_milli: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MainMenuCameraReference {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub orthographic: bool,
    pub orthographic_size: f32,
    pub field_of_view_degrees: f32,
    pub near: f32,
    pub far: f32,
    pub background: [f32; 4],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MainMenuModelInstance {
    pub hierarchy_path: String,
    pub source_guid: String,
    pub source_path: String,
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MainMenuEmbeddedMesh {
    pub hierarchy_path: String,
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uv: Vec<[f32; 2]>,
    pub triangles: Vec<u32>,
}
