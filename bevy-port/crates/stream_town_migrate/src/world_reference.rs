use std::{collections::BTreeMap, fs, path::Path};

use anyhow::{Context, Result, ensure};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use stream_town_domain::{
    StableId, UnityGenerationLayerReference, UnityGenerationReference, UnityTerrainReference,
};

#[derive(Debug, Serialize)]
pub(crate) struct ConversionReport {
    destination: String,
    terrain_seed: i32,
    terrain_cells: usize,
    layers: BTreeMap<String, usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityReferenceJson {
    schema_version: u32,
    terrain_seed: i32,
    terrain: UnityTerrainJson,
    layers: Vec<UnityLayerJson>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityTerrainJson {
    size: u16,
    heights: Vec<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityLayerJson {
    id: String,
    size: u16,
    seed: i32,
    noise_scale: f32,
    octaves: u8,
    persistence: f32,
    lacunarity: f32,
    threshold: f32,
    spacing: u16,
    candidates: Vec<UnityVector2>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct UnityVector2 {
    x: f32,
    y: f32,
}

pub(crate) fn convert(source: &Path, destination: &Path) -> Result<ConversionReport> {
    let encoded = fs::read_to_string(source)
        .with_context(|| format!("failed to read {}", source.display()))?;
    let source_data: UnityReferenceJson = serde_json::from_str(&encoded)
        .with_context(|| format!("failed to parse {}", source.display()))?;
    ensure!(
        source_data.schema_version == 1,
        "unsupported Unity generation-reference schema {}",
        source_data.schema_version
    );
    let expected_cells = usize::from(source_data.terrain.size).pow(2);
    ensure!(
        source_data.terrain.heights.len() == expected_cells,
        "Unity terrain reference has {} heights, expected {expected_cells}",
        source_data.terrain.heights.len()
    );
    let height_half_metres = source_data
        .terrain
        .heights
        .iter()
        .copied()
        .map(quantized_half_metre_height)
        .collect::<Result<Vec<_>>>()?;
    let mut layers = BTreeMap::new();
    let mut layer_counts = BTreeMap::new();
    for layer in source_data.layers {
        let id = StableId::new(layer.id)?;
        let candidate_half_units = layer
            .candidates
            .into_iter()
            .map(|candidate| {
                let x = quantized_half_unit(candidate.x)?;
                let z = quantized_half_unit(candidate.y)?;
                Ok([x, z])
            })
            .collect::<Result<Vec<_>>>()?;
        ensure!(
            layers
                .insert(
                    id.clone(),
                    UnityGenerationLayerReference {
                        source_size: layer.size,
                        seed: layer.seed,
                        noise_scale: layer.noise_scale,
                        octaves: layer.octaves,
                        persistence: layer.persistence,
                        lacunarity: layer.lacunarity,
                        threshold: layer.threshold,
                        spacing: layer.spacing,
                        candidate_half_units,
                    },
                )
                .is_none(),
            "duplicate Unity generation layer {id}"
        );
        layer_counts.insert(id.to_string(), layers[&id].candidate_half_units.len());
    }
    let reference = UnityGenerationReference {
        schema_version: 1,
        unity_version: "6000.5.6f1".to_owned(),
        terrain: UnityTerrainReference {
            seed: source_data.terrain_seed,
            width: source_data.terrain.size,
            height: source_data.terrain.size,
            height_half_metres,
        },
        layers,
    };
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    let ron = ron::ser::to_string_pretty(&reference, PrettyConfig::default())?;
    fs::write(destination, ron)
        .with_context(|| format!("failed to write {}", destination.display()))?;
    Ok(ConversionReport {
        destination: destination.display().to_string(),
        terrain_seed: reference.terrain.seed,
        terrain_cells: reference.terrain.height_half_metres.len(),
        layers: layer_counts,
    })
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn quantized_half_metre_height(height: f32) -> Result<u8> {
    let half_steps = height * 2.0;
    ensure!(
        height.is_finite()
            && (half_steps - half_steps.round()).abs() <= 0.000_1
            && (0.0..=f32::from(u8::MAX)).contains(&half_steps),
        "Unity terrain height {height} is not a non-negative half-metre value"
    );
    Ok(half_steps.round() as u8)
}

#[allow(clippy::cast_possible_truncation)]
fn quantized_half_unit(value: f32) -> Result<i16> {
    let half_units = value * 2.0;
    ensure!(
        value.is_finite()
            && (half_units - half_units.round()).abs() <= 0.000_1
            && (f32::from(i16::MIN)..=f32::from(i16::MAX)).contains(&half_units),
        "Unity candidate coordinate {value} is not a representable half-unit value"
    );
    Ok(half_units.round() as i16)
}
