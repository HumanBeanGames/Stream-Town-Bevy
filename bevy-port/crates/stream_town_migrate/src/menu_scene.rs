use std::{fs, path::Path};

use anyhow::{Context, Result, ensure};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use stream_town_domain::{
    MainMenuCameraReference, MainMenuEmbeddedMesh, MainMenuModelInstance, MainMenuSceneReference,
};

#[derive(Debug, Serialize)]
pub(crate) struct ConversionReport {
    destination: String,
    model_instances: usize,
    embedded_meshes: usize,
    embedded_vertices: usize,
    embedded_triangles: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityMenuReference {
    schema_version: u32,
    source_scene: String,
    camera: UnityCamera,
    instances: Vec<UnityInstance>,
    embedded_meshes: Vec<UnityMesh>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityCamera {
    position: UnityVec3,
    rotation: UnityQuat,
    orthographic: bool,
    orthographic_size: f32,
    field_of_view: f32,
    near: f32,
    far: f32,
    background: UnityColor,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityInstance {
    hierarchy_path: String,
    source_guid: String,
    source_path: String,
    position: UnityVec3,
    rotation: UnityQuat,
    scale: UnityVec3,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityMesh {
    hierarchy_path: String,
    vertices: Vec<UnityVec3>,
    normals: Vec<UnityVec3>,
    uv: Vec<UnityVec2>,
    triangles: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct UnityVec2 {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct UnityVec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct UnityQuat {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct UnityColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

pub(crate) fn convert(source: &Path, destination: &Path) -> Result<ConversionReport> {
    let encoded = fs::read_to_string(source)
        .with_context(|| format!("failed to read {}", source.display()))?;
    let source: UnityMenuReference = serde_json::from_str(&encoded)
        .with_context(|| format!("failed to parse {}", source.display()))?;
    ensure!(
        source.schema_version == 1,
        "unsupported main-menu reference schema"
    );
    let camera = MainMenuCameraReference {
        position: vec3(source.camera.position)?,
        rotation: quat(source.camera.rotation)?,
        orthographic: source.camera.orthographic,
        orthographic_size: finite(source.camera.orthographic_size)?,
        field_of_view_degrees: finite(source.camera.field_of_view)?,
        near: finite(source.camera.near)?,
        far: finite(source.camera.far)?,
        background: [
            finite(source.camera.background.r)?,
            finite(source.camera.background.g)?,
            finite(source.camera.background.b)?,
            finite(source.camera.background.a)?,
        ],
    };
    ensure!(
        camera.near > 0.0 && camera.far > camera.near,
        "invalid menu camera clip planes"
    );
    let instances = source
        .instances
        .into_iter()
        .map(|instance| {
            ensure!(
                !instance.hierarchy_path.trim().is_empty(),
                "empty menu hierarchy path"
            );
            ensure!(instance.source_guid.len() == 32, "invalid menu source GUID");
            ensure!(
                instance.source_path.starts_with("Assets/"),
                "invalid menu source path"
            );
            Ok(MainMenuModelInstance {
                hierarchy_path: instance.hierarchy_path,
                source_guid: instance.source_guid,
                source_path: instance.source_path.replace('\\', "/"),
                position: vec3(instance.position)?,
                rotation: quat(instance.rotation)?,
                scale: vec3(instance.scale)?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let embedded_meshes =
        source
            .embedded_meshes
            .into_iter()
            .map(|mesh| {
                ensure!(
                    mesh.vertices.len() == mesh.normals.len(),
                    "menu mesh normal count mismatch"
                );
                ensure!(
                    mesh.vertices.len() == mesh.uv.len(),
                    "menu mesh UV count mismatch"
                );
                ensure!(
                    mesh.triangles.len() % 3 == 0,
                    "menu mesh triangle list is malformed"
                );
                ensure!(
                    mesh.triangles.iter().all(|index| usize::try_from(*index)
                        .is_ok_and(|index| index < mesh.vertices.len())),
                    "menu mesh index is out of bounds"
                );
                Ok(MainMenuEmbeddedMesh {
                    hierarchy_path: mesh.hierarchy_path,
                    vertices: mesh.vertices.into_iter().map(vec3).collect::<Result<_>>()?,
                    normals: mesh.normals.into_iter().map(vec3).collect::<Result<_>>()?,
                    uv: mesh
                        .uv
                        .into_iter()
                        .map(|value| Ok([finite(value.x)?, finite(value.y)?]))
                        .collect::<Result<_>>()?,
                    triangles: mesh.triangles,
                })
            })
            .collect::<Result<Vec<_>>>()?;
    let reference = MainMenuSceneReference {
        schema_version: 1,
        source_scene: source.source_scene,
        camera,
        instances,
        embedded_meshes,
    };
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(
        destination,
        ron::ser::to_string_pretty(&reference, PrettyConfig::default())?,
    )
    .with_context(|| format!("failed to write {}", destination.display()))?;
    Ok(ConversionReport {
        destination: destination.display().to_string(),
        model_instances: reference.instances.len(),
        embedded_meshes: reference.embedded_meshes.len(),
        embedded_vertices: reference
            .embedded_meshes
            .iter()
            .map(|mesh| mesh.vertices.len())
            .sum(),
        embedded_triangles: reference
            .embedded_meshes
            .iter()
            .map(|mesh| mesh.triangles.len() / 3)
            .sum(),
    })
}

fn vec3(value: UnityVec3) -> Result<[f32; 3]> {
    Ok([finite(value.x)?, finite(value.y)?, finite(value.z)?])
}

fn quat(value: UnityQuat) -> Result<[f32; 4]> {
    Ok([
        finite(value.x)?,
        finite(value.y)?,
        finite(value.z)?,
        finite(value.w)?,
    ])
}

fn finite(value: f32) -> Result<f32> {
    ensure!(
        value.is_finite(),
        "main-menu reference contains a non-finite number"
    );
    Ok(value)
}
