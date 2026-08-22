use std::{collections::BTreeMap, fs, path::Path};

use anyhow::{Context, Result, ensure};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use stream_town_domain::{
    ArchetypeKind, ContentCatalog, GameConfig, GridPos, MainMenuCameraReference,
    MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance,
    MainMenuResourceVisual, MainMenuSceneReference, generate_world_with_content,
};

const MAIN_MENU_TERRAIN_HEIGHT_MULTIPLIER: f32 = 3.0;

#[derive(Debug, Serialize)]
pub(crate) struct ConversionReport {
    destination: String,
    model_instances: usize,
    embedded_meshes: usize,
    embedded_vertices: usize,
    embedded_triangles: usize,
}

#[derive(Debug, Serialize)]
pub(crate) struct BakeReport {
    destination: String,
    seed: u64,
    generator_hash: String,
    adjusted_vertices: usize,
    flattened_foundations: usize,
    resources: usize,
    foliage: usize,
    baked_hash: String,
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
        position: right_handed_vec3(source.camera.position)?,
        rotation: right_handed_quat(source.camera.rotation)?,
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
                position: right_handed_vec3(instance.position)?,
                rotation: right_handed_quat(instance.rotation)?,
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
                    vertices: mesh
                        .vertices
                        .into_iter()
                        .map(right_handed_vec3)
                        .collect::<Result<_>>()?,
                    normals: mesh
                        .normals
                        .into_iter()
                        .map(right_handed_vec3)
                        .collect::<Result<_>>()?,
                    uv: mesh
                        .uv
                        .into_iter()
                        .map(|value| Ok([finite(value.x)?, finite(value.y)?]))
                        .collect::<Result<_>>()?,
                    triangles: right_handed_triangles(mesh.triangles),
                })
            })
            .collect::<Result<Vec<_>>>()?;
    let reference = MainMenuSceneReference {
        schema_version: 2,
        source_scene: source.source_scene,
        camera,
        instances,
        embedded_meshes,
        corrective_bake: None,
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

pub(crate) fn bake(
    source: &Path,
    config_path: &Path,
    content_path: &Path,
    destination: &Path,
) -> Result<BakeReport> {
    let encoded = fs::read_to_string(source)
        .with_context(|| format!("failed to read {}", source.display()))?;
    let mut reference: MainMenuSceneReference =
        ron::from_str(&encoded).with_context(|| format!("failed to parse {}", source.display()))?;
    ensure!(
        reference.schema_version == 2 && reference.corrective_bake.is_none(),
        "main-menu corrective baking requires an unbaked schema-2 reference"
    );
    ensure!(
        reference.embedded_meshes.len() == 1,
        "main-menu corrective baking requires exactly one embedded terrain mesh"
    );
    let config: GameConfig = ron::from_str(
        &fs::read_to_string(config_path)
            .with_context(|| format!("failed to read {}", config_path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", config_path.display()))?;
    let content: ContentCatalog = ron::from_str(
        &fs::read_to_string(content_path)
            .with_context(|| format!("failed to read {}", content_path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", content_path.display()))?;
    config.validate()?;
    content.validate()?;

    let generated = generate_world_with_content(&config.world, &content);
    let mesh = &mut reference.embedded_meshes[0];
    let source_vertices = mesh.vertices.clone();
    let mesh_bounds = horizontal_bounds(&source_vertices)?;
    for (vertex, source_vertex) in mesh.vertices.iter_mut().zip(&source_vertices) {
        let generated_height =
            generated_height_at(&generated, &config, source_vertex[0], source_vertex[2]).max(0.0);
        // The authored menu mesh has three shoreline levels: -7m ocean,
        // -3.5m beach shelf, and 0m land. Weighting ordinary generated height
        // by that existing shoreline mask preserves its silhouette.
        let shoreline_weight = ((source_vertex[1] + 7.0) / 7.0).clamp(0.0, 1.0);
        // The authored menu camera views the town almost side-on. Ordinary
        // gameplay relief (0.5m steps) reads as flat from that distance, so
        // amplify only the baked presentation height. Squaring the shoreline
        // mask keeps the -3.5m beach shelf submerged instead of lifting it out
        // of the ocean as land relief is exaggerated.
        vertex[1] = source_vertex[1]
            + generated_height * shoreline_weight.powi(2) * MAIN_MENU_TERRAIN_HEIGHT_MULTIPLIER;
    }

    let foundations = building_foundations(&reference.instances, &content, &config);
    let foundation_heights = flatten_foundations(mesh, &foundations);
    for instance in &mut reference.instances {
        let root = menu_building_root(&instance.hierarchy_path);
        if let Some(height) = foundation_heights.get(root) {
            instance.position[1] = *height;
        }
    }
    recompute_normals(mesh);

    let water_height = -0.65;
    let resources = generated
        .resources
        .iter()
        .filter(|resource| resource.target_kind.as_str() != "target:fish")
        .filter_map(|resource| {
            let [x, z] =
                generated_visual_position(resource.position, resource.offset_milli_cells, &config);
            decoration_position(mesh, mesh_bounds, &foundations, x, z, water_height).map(
                |position| MainMenuResourceVisual {
                    id: resource.id.clone(),
                    kind: resource.kind.clone(),
                    target_kind: resource.target_kind.clone(),
                    position,
                    mesh_index: if resource.kind.as_str() == "resource:food" {
                        0
                    } else {
                        (resource.position.x ^ resource.position.z) & 1
                    },
                },
            )
        })
        .collect::<Vec<_>>();
    let foliage = generated
        .foliage
        .iter()
        .filter_map(|foliage| {
            let [x, z] =
                generated_visual_position(foliage.position, foliage.offset_milli_cells, &config);
            decoration_position(mesh, mesh_bounds, &foundations, x, z, water_height).map(
                |position| MainMenuFoliageVisual {
                    id: foliage.id.clone(),
                    layer: foliage.layer.clone(),
                    variant: foliage.variant,
                    position,
                    yaw_milliradians: foliage.yaw_milliradians,
                    scale_milli: foliage.scale_milli,
                },
            )
        })
        .collect::<Vec<_>>();
    let resources_count = resources.len();
    let foliage_count = foliage.len();
    let adjusted_vertices = mesh.vertices.len();
    reference.schema_version = 3;
    reference.corrective_bake = Some(MainMenuCorrectiveBake {
        version: 3,
        seed: generated.seed,
        generator_version: generated.generator_version,
        generator_hash: generated.deterministic_hash.clone(),
        cell_size: config.world.cell_size,
        terrain_height_multiplier: MAIN_MENU_TERRAIN_HEIGHT_MULTIPLIER,
        water_height,
        water_extent: 620.0,
        ocean_floor_height: -7.05,
        terrain_extent: 620.0,
        resources,
        foliage,
    });
    let baked = ron::ser::to_string_pretty(&reference, PrettyConfig::default())?;
    let baked_hash = hex::encode(Sha256::digest(baked.as_bytes()));
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(destination, baked)
        .with_context(|| format!("failed to write {}", destination.display()))?;
    Ok(BakeReport {
        destination: destination.display().to_string(),
        seed: generated.seed,
        generator_hash: generated.deterministic_hash,
        adjusted_vertices,
        flattened_foundations: foundations.len(),
        resources: resources_count,
        foliage: foliage_count,
        baked_hash,
    })
}

#[derive(Clone, Debug)]
struct BuildingFoundation {
    hierarchy_root: String,
    centre: [f32; 2],
    half_extents: [f32; 2],
}

fn building_foundations(
    instances: &[MainMenuModelInstance],
    content: &ContentCatalog,
    config: &GameConfig,
) -> Vec<BuildingFoundation> {
    let mut foundations = BTreeMap::new();
    for instance in instances {
        let Some(archetype) = content.archetypes.values().find(|archetype| {
            archetype.kind == ArchetypeKind::Building
                && archetype
                    .scenes
                    .iter()
                    .any(|scene| scene.source_model == instance.source_path)
        }) else {
            continue;
        };
        let root = menu_building_root(&instance.hierarchy_path).to_owned();
        let padding = config.world.cell_size;
        foundations
            .entry(root.clone())
            .or_insert(BuildingFoundation {
                hierarchy_root: root,
                centre: [instance.position[0], instance.position[2]],
                half_extents: [
                    (f32::from(archetype.footprint[0]) * config.world.cell_size * 0.5).max(padding),
                    (f32::from(archetype.footprint[1]) * config.world.cell_size * 0.5).max(padding),
                ],
            });
    }
    foundations.into_values().collect()
}

fn menu_building_root(path: &str) -> &str {
    path.rsplit_once('/').map_or(path, |(root, _)| root)
}

fn flatten_foundations(
    mesh: &mut MainMenuEmbeddedMesh,
    foundations: &[BuildingFoundation],
) -> BTreeMap<String, f32> {
    // Sample every building from the same untouched generated surface. Sampling
    // the mutable mesh after each flatten lets adjacent farms/walls propagate
    // one plateau across the town and erases the generator's height variation.
    let generated_surface = mesh.clone();
    let foundation_heights = foundations
        .iter()
        .map(|foundation| {
            (
                foundation.hierarchy_root.clone(),
                sample_menu_height(
                    &generated_surface,
                    foundation.centre[0],
                    foundation.centre[1],
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for foundation in foundations {
        let height = foundation_heights[&foundation.hierarchy_root];
        for vertex in &mut mesh.vertices {
            if (vertex[0] - foundation.centre[0]).abs() <= foundation.half_extents[0]
                && (vertex[2] - foundation.centre[1]).abs() <= foundation.half_extents[1]
            {
                vertex[1] = height;
            }
        }
    }
    foundation_heights
}

fn horizontal_bounds(vertices: &[[f32; 3]]) -> Result<[f32; 4]> {
    let first = vertices.first().context("menu terrain mesh is empty")?;
    Ok(vertices.iter().fold(
        [first[0], first[0], first[2], first[2]],
        |[min_x, max_x, min_z, max_z], vertex| {
            [
                min_x.min(vertex[0]),
                max_x.max(vertex[0]),
                min_z.min(vertex[2]),
                max_z.max(vertex[2]),
            ]
        },
    ))
}

fn generated_height_at(
    generated: &stream_town_domain::GeneratedWorld,
    config: &GameConfig,
    world_x: f32,
    world_z: f32,
) -> f32 {
    let half_x = f32::from(config.world.width.saturating_sub(1)) * 0.5;
    let half_z = f32::from(config.world.height.saturating_sub(1)) * 0.5;
    let x = (world_x / config.world.cell_size + half_x).round();
    let z = (world_z / config.world.cell_size + half_z).round();
    if x < 0.0
        || z < 0.0
        || x >= f32::from(config.world.width)
        || z >= f32::from(config.world.height)
    {
        return 0.0;
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let position = GridPos {
        x: x as u16,
        z: z as u16,
    };
    f32::from(generated.navigation.height_at(position).unwrap_or_default()) / 100.0
}

fn generated_visual_position(
    position: GridPos,
    offset_milli_cells: [i16; 2],
    config: &GameConfig,
) -> [f32; 2] {
    let x = (f32::from(position.x) - f32::from(config.world.width.saturating_sub(1)) * 0.5)
        * config.world.cell_size
        + f32::from(offset_milli_cells[0]) * config.world.cell_size / 1_000.0;
    let z = (f32::from(position.z) - f32::from(config.world.height.saturating_sub(1)) * 0.5)
        * config.world.cell_size
        + f32::from(offset_milli_cells[1]) * config.world.cell_size / 1_000.0;
    [x, z]
}

fn decoration_position(
    mesh: &MainMenuEmbeddedMesh,
    bounds: [f32; 4],
    foundations: &[BuildingFoundation],
    x: f32,
    z: f32,
    water_height: f32,
) -> Option<[f32; 3]> {
    if x < bounds[0] || x > bounds[1] || z < bounds[2] || z > bounds[3] {
        return None;
    }
    if foundations.iter().any(|foundation| {
        (x - foundation.centre[0]).abs() <= foundation.half_extents[0]
            && (z - foundation.centre[1]).abs() <= foundation.half_extents[1]
    }) {
        return None;
    }
    let y = sample_menu_height(mesh, x, z);
    (y > water_height + 0.1).then_some([x, y, z])
}

fn sample_menu_height(mesh: &MainMenuEmbeddedMesh, x: f32, z: f32) -> f32 {
    mesh.vertices
        .iter()
        .min_by(|left, right| {
            let left_distance = (left[0] - x).mul_add(left[0] - x, (left[2] - z).powi(2));
            let right_distance = (right[0] - x).mul_add(right[0] - x, (right[2] - z).powi(2));
            left_distance.total_cmp(&right_distance)
        })
        .map_or(0.0, |vertex| vertex[1])
}

fn recompute_normals(mesh: &mut MainMenuEmbeddedMesh) {
    let mut normals = vec![[0.0_f32; 3]; mesh.vertices.len()];
    for triangle in mesh.triangles.chunks_exact(3) {
        let [a, b, c] = [
            mesh.vertices[triangle[0] as usize],
            mesh.vertices[triangle[1] as usize],
            mesh.vertices[triangle[2] as usize],
        ];
        let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
        let normal = [
            ab[1].mul_add(ac[2], -ab[2] * ac[1]),
            ab[2].mul_add(ac[0], -ab[0] * ac[2]),
            ab[0].mul_add(ac[1], -ab[1] * ac[0]),
        ];
        for index in triangle {
            let target = &mut normals[*index as usize];
            target[0] += normal[0];
            target[1] += normal[1];
            target[2] += normal[2];
        }
    }
    for normal in &mut normals {
        let length = normal[0]
            .mul_add(
                normal[0],
                normal[1].mul_add(normal[1], normal[2] * normal[2]),
            )
            .sqrt();
        if length > f32::EPSILON {
            for channel in normal {
                *channel /= length;
            }
        } else {
            *normal = [0.0, 1.0, 0.0];
        }
    }
    mesh.normals = normals;
}

fn vec3(value: UnityVec3) -> Result<[f32; 3]> {
    Ok([finite(value.x)?, finite(value.y)?, finite(value.z)?])
}

fn right_handed_vec3(value: UnityVec3) -> Result<[f32; 3]> {
    Ok([finite(value.x)?, finite(value.y)?, -finite(value.z)?])
}

fn right_handed_quat(value: UnityQuat) -> Result<[f32; 4]> {
    Ok([
        -finite(value.x)?,
        -finite(value.y)?,
        finite(value.z)?,
        finite(value.w)?,
    ])
}

fn right_handed_triangles(mut triangles: Vec<u32>) -> Vec<u32> {
    for triangle in triangles.chunks_exact_mut(3) {
        triangle.swap(1, 2);
    }
    triangles
}

fn finite(value: f32) -> Result<f32> {
    ensure!(
        value.is_finite(),
        "main-menu reference contains a non-finite number"
    );
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unity_scene_data_is_fully_reflected_into_bevy_space() {
        assert!(
            right_handed_vec3(UnityVec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            })
            .unwrap()
            .into_iter()
            .zip([1.0, 2.0, -3.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert!(
            right_handed_quat(UnityQuat {
                x: 0.1,
                y: 0.2,
                z: 0.3,
                w: 0.4,
            })
            .unwrap()
            .into_iter()
            .zip([-0.1, -0.2, 0.3, 0.4])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert_eq!(
            right_handed_triangles(vec![0, 1, 2, 4, 5, 6]),
            vec![0, 2, 1, 4, 6, 5]
        );
    }

    #[test]
    fn adjacent_foundations_sample_the_unmodified_generated_surface() {
        let mut mesh = MainMenuEmbeddedMesh {
            hierarchy_path: "Terrain".to_owned(),
            vertices: vec![
                [0.0, 0.0, 0.0],
                [2.0, 1.0, 0.0],
                [4.0, 2.0, 0.0],
                [6.0, 3.0, 0.0],
            ],
            normals: vec![[0.0, 1.0, 0.0]; 4],
            uv: vec![[0.0, 0.0]; 4],
            triangles: Vec::new(),
        };
        let foundations = vec![
            BuildingFoundation {
                hierarchy_root: "first".to_owned(),
                centre: [0.0, 0.0],
                half_extents: [2.0, 1.0],
            },
            BuildingFoundation {
                hierarchy_root: "second".to_owned(),
                centre: [4.0, 0.0],
                half_extents: [2.0, 1.0],
            },
        ];

        let heights = flatten_foundations(&mut mesh, &foundations);

        assert!(heights["first"].abs() < f32::EPSILON);
        assert!((heights["second"] - 2.0).abs() < f32::EPSILON);
        assert!(mesh.vertices[0][1].abs() < f32::EPSILON);
        assert!((mesh.vertices[3][1] - 2.0).abs() < f32::EPSILON);
    }
}
