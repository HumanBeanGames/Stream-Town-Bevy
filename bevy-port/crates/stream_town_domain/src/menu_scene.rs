use serde::{Deserialize, Serialize};

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
