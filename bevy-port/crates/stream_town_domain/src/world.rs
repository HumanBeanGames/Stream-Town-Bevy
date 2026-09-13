use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    FoliageHabitat, FoliageLayerDef, GridPos, NavGrid, ResourceGenerationHabitat,
    ResourceGenerationLayerDef, StableId, WorldGenConfig, default_resource_generation_layers,
};

/// Seed of the recorded shipping town used to validate deterministic generation.
///
/// Authored resource/foliage offsets describe this reference town. New towns
/// retain those authored Perlin parameters but deterministically translate each
/// layer's noise domain from their own seed.
const UNITY_REFERENCE_WORLD_SEED: u64 = 1_580_290_387;

/// The rendered gameplay water plane sits slightly above the configured level
/// to avoid coplanar terrain artifacts. Habitat decisions must use this final
/// visible surface, not the unshifted configuration value.
pub const VISIBLE_WATER_SURFACE_LIFT_METRES: f32 = 0.20;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GeneratedResource {
    pub id: StableId,
    pub kind: StableId,
    pub target_kind: StableId,
    pub position: GridPos,
    #[serde(default)]
    pub offset_milli_cells: [i16; 2],
    #[serde(default)]
    pub generation_occupancy: [i16; 2],
    pub amount: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GeneratedFoliage {
    pub id: StableId,
    pub layer: StableId,
    pub habitat: FoliageHabitat,
    pub position: GridPos,
    pub offset_milli_cells: [i16; 2],
    pub variant: u16,
    pub yaw_milliradians: u16,
    pub scale_milli: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GeneratedWorld {
    pub seed: u64,
    pub generator_version: u32,
    pub navigation: NavGrid,
    pub resources: Vec<GeneratedResource>,
    pub foliage: Vec<GeneratedFoliage>,
    pub deterministic_hash: String,
}

/// Observable, engine-independent milestones in deterministic world generation.
///
/// These are deliberately semantic stages rather than elapsed-time estimates:
/// a loading frontend can report which real generator work has completed
/// without changing the deterministic algorithm or guessing a percentage.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorldGenerationStage {
    Terrain,
    LandResources,
    NavigationAndFish,
    Foliage,
    Fingerprint,
}

impl WorldGenerationStage {
    pub const ALL: [Self; 5] = [
        Self::Terrain,
        Self::LandResources,
        Self::NavigationAndFish,
        Self::Foliage,
        Self::Fingerprint,
    ];
}

#[must_use]
pub fn generate_world(config: &WorldGenConfig) -> GeneratedWorld {
    generate_world_from_layers(
        config,
        &default_resource_generation_layers(),
        &[],
        &mut |_| {},
    )
}

#[must_use]
pub fn generate_world_with_content(
    config: &WorldGenConfig,
    content: &crate::ContentCatalog,
) -> GeneratedWorld {
    generate_world_from_layers(
        config,
        &content.resource_generation,
        &content.foliage,
        &mut |_| {},
    )
}

/// Generates the same deterministic world while reporting completed stages.
///
/// The observer is diagnostic only: it cannot provide inputs to generation and
/// therefore cannot affect the resulting terrain, resources, foliage, or hash.
pub fn generate_world_with_content_observed(
    config: &WorldGenConfig,
    content: &crate::ContentCatalog,
    mut completed: impl FnMut(WorldGenerationStage),
) -> GeneratedWorld {
    generate_world_from_layers(
        config,
        &content.resource_generation,
        &content.foliage,
        &mut completed,
    )
}

fn generate_world_from_layers(
    config: &WorldGenConfig,
    resource_layers: &[ResourceGenerationLayerDef],
    foliage_layers: &[FoliageLayerDef],
    completed: &mut impl FnMut(WorldGenerationStage),
) -> GeneratedWorld {
    // Version 9 fingerprints final-height terrain/resource classification.
    // Foliage is intentionally absent from the save fingerprint and is rebuilt
    // from the active foliage algorithm whenever a town loads; foliage-only
    // improvements therefore do not invalidate terrain or simulation saves.
    const GENERATOR_VERSION: u32 = 9;
    let cell_count = usize::from(config.width) * usize::from(config.height);
    let terrain_seed = u32::try_from(config.seed & u64::from(u32::MAX))
        .expect("masked terrain seed fits u32")
        .cast_signed();
    let terrain_noise = unity_noise_map(
        config.width,
        config.height,
        terrain_seed,
        50.0,
        3,
        0.827,
        2.0,
        [0.0, 0.0],
    );
    let mut heights = Vec::with_capacity(cell_count);
    let mut blocked = Vec::with_capacity(cell_count);

    for z in 0..config.height {
        for x in 0..config.width {
            // Unity emits noise rows from +Z to -Z, while NavGrid rows run from
            // -Z to +Z. Mirroring the source row reproduces the authored mesh in
            // the same world-space orientation without importing any mesh data.
            let unity_y = config.height - 1 - z;
            let unity_index = usize::from(unity_y) * usize::from(config.width) + usize::from(x);
            let edge = x <= 1
                || unity_y <= 1
                || x >= config.width.saturating_sub(2)
                || unity_y >= config.height.saturating_sub(2);
            let source_height = if edge {
                -1.0
            } else {
                terrain_noise[unity_index]
            };
            let curved_height = unity_terrain_height_curve(source_height);
            let world = authored_grid_centre(config, GridPos { x, z });
            let distance = (world[0].mul_add(world[0], world[1] * world[1])).sqrt();
            let normalized_distance = (distance / 200.0).clamp(0.0, 1.0);
            let island_bias = 3.0 * (1.0 - smooth_noise_step(normalized_distance));
            let quantized = round_to_even(curved_height * island_bias / 0.5) * 0.5;
            let centimetres = quantized * f32::from(config.height_scale_centimetres);
            let height = unity_clamped_i16(centimetres);
            // Unity's A* pass rejects terrain below QuantizationFactor - 0.05.
            let is_blocked = height < 45;
            heights.push(height);
            blocked.push(is_blocked);
        }
    }
    completed(WorldGenerationStage::Terrain);

    let mut resources = generate_authored_resources(config, &heights, resource_layers);
    completed(WorldGenerationStage::LandResources);

    // Keep the town centre navigable and suitable for deterministic actor spawning.
    let spawn = GridPos {
        x: config.width / 2,
        z: config.height / 2,
    };
    let spawn_index = usize::from(spawn.z) * usize::from(config.width) + usize::from(spawn.x);
    blocked[spawn_index] = false;

    for resource in &resources {
        let index = usize::from(resource.position.z) * usize::from(config.width)
            + usize::from(resource.position.x);
        blocked[index] = true;
    }
    // The explicit town spawn remains the sole land-resource occupancy
    // exception, matching the existing centre-safety guarantee.
    blocked[spawn_index] = false;
    let navigation = NavGrid::new(config.width, config.height, blocked, heights)
        .expect("validated world configuration produces a valid grid");
    generate_shoreline_resources(config, &navigation, resource_layers, &mut resources);
    completed(WorldGenerationStage::NavigationAndFish);
    let foliage = generate_foliage(config, &navigation, &resources, foliage_layers);
    completed(WorldGenerationStage::Foliage);
    // Decorative foliage is regenerated from authored content and deliberately
    // excluded from native-save compatibility. The saved world fingerprint
    // continues to describe terrain, navigation, and gameplay resources only.
    let deterministic_hash = hash_world(config.seed, GENERATOR_VERSION, &navigation, &resources);
    completed(WorldGenerationStage::Fingerprint);
    GeneratedWorld {
        seed: config.seed,
        generator_version: GENERATOR_VERSION,
        navigation,
        resources,
        foliage,
        deterministic_hash,
    }
}

fn generate_authored_resources(
    config: &WorldGenConfig,
    heights: &[i16],
    layers: &[ResourceGenerationLayerDef],
) -> Vec<GeneratedResource> {
    let mut resources = Vec::new();
    // Unity shares one HashSet<(int, int)> across all resource and foliage
    // layers. Each layer computes those keys using its own spacing; preserving
    // that slightly unusual behavior is required for placement parity.
    let mut occupied = std::collections::BTreeSet::<(i32, i32)>::new();
    for (layer_index, layer) in layers
        .iter()
        .filter(|layer| layer.habitat == ResourceGenerationHabitat::Land)
        .enumerate()
    {
        let candidates = generate_candidate_mask(
            layer.source_size,
            layer.seed,
            layer.noise_scale,
            layer.octaves,
            layer.persistence,
            layer.lacunarity,
            layer.spawn_threshold,
            layer.spacing,
            layer.half_cell_terrain_offset,
            town_layer_noise_offset(config.seed, &layer.id, layer.seed, layer.source_size),
        );
        for candidate in candidates {
            let resource_world_x = f32::from(candidate[0]) * 0.5;
            let resource_world_z = f32::from(candidate[1]) * 0.5;
            let terrain_offset = if layer.half_cell_terrain_offset {
                0.5
            } else {
                0.0
            };
            let world_x = resource_world_x - terrain_offset;
            let world_z = resource_world_z - terrain_offset;
            let centre = f32::from(layer.spacing) * 0.5;
            let source_z = unity_rounded_i32(world_x - centre);
            let source_x = unity_rounded_i32(world_z - centre);
            let occupancy = (
                unity_floor_i32(world_x / f32::from(layer.spacing.max(1))),
                unity_floor_i32(world_z / f32::from(layer.spacing.max(1))),
            );
            if occupied.contains(&occupancy) {
                continue;
            }
            let Some(surface_height) = terrain_surface_height_from_centimetres(
                config,
                heights,
                resource_world_x,
                resource_world_z,
            ) else {
                continue;
            };
            if surface_height <= visible_water_surface_height(config) {
                continue;
            }
            let Some(position) = authored_world_to_grid(config, resource_world_x, resource_world_z)
            else {
                continue;
            };
            let centre_world = authored_grid_centre(config, position);
            let offset_milli_cells = [
                unity_clamped_i16(
                    (resource_world_x - centre_world[0]) / config.cell_size * 1_000.0,
                ),
                unity_clamped_i16(
                    (resource_world_z - centre_world[1]) / config.cell_size * 1_000.0,
                ),
            ];
            occupied.insert(occupancy);
            resources.push(GeneratedResource {
                id: StableId::new(format!("resource:{layer_index}:{source_x}:{source_z}"))
                    .expect("generated stable resource ID"),
                kind: layer.resource.clone(),
                target_kind: layer.target_kind.clone(),
                position,
                offset_milli_cells,
                generation_occupancy: [
                    i16::try_from(occupancy.0).expect("authored occupancy x fits i16"),
                    i16::try_from(occupancy.1).expect("authored occupancy z fits i16"),
                ],
                amount: layer.amount,
            });
        }
    }
    resources
}

fn generate_shoreline_resources(
    config: &WorldGenConfig,
    navigation: &NavGrid,
    layers: &[ResourceGenerationLayerDef],
    resources: &mut Vec<GeneratedResource>,
) {
    const FISH_SEED_SALT: u64 = 0x4649_5348_5F53_484F;
    for (layer_index, layer) in layers
        .iter()
        .filter(|layer| layer.habitat == ResourceGenerationHabitat::ShorelineWater)
        .enumerate()
    {
        let density = u32::from(config.resource_density_per_thousand)
            .saturating_mul(u32::from(layer.density_multiplier_per_thousand))
            .div_ceil(1_000)
            .min(1_000);
        let seed =
            config.seed ^ FISH_SEED_SALT ^ u64::from(u32::from_ne_bytes(layer.seed.to_ne_bytes()));
        for z in 0..navigation.height() {
            for x in 0..navigation.width() {
                let position = GridPos { x, z };
                if navigation.is_walkable(position)
                    || navigation.height_at(position).unwrap_or_default()
                        > config.water_level_centimetres
                    || shoreline_approaches(navigation, position).next().is_none()
                {
                    continue;
                }
                let random = cell_hash(seed, x, z);
                let roll = u32::try_from(random % 1_000).expect("modulo 1000");
                if roll >= density {
                    continue;
                }
                let id = if layer_index == 0 && layer.target_kind.as_str() == "target:fish" {
                    format!("resource:fish:{x}:{z}")
                } else {
                    format!("resource:water:{layer_index}:{x}:{z}")
                };
                resources.push(GeneratedResource {
                    id: StableId::new(id).expect("generated stable resource ID"),
                    kind: layer.resource.clone(),
                    target_kind: layer.target_kind.clone(),
                    position,
                    offset_milli_cells: [0, 0],
                    generation_occupancy: [
                        i16::try_from(x).expect("world x fits i16"),
                        i16::try_from(z).expect("world z fits i16"),
                    ],
                    amount: layer.amount,
                });
            }
        }
    }
}

pub fn shoreline_approaches(
    navigation: &NavGrid,
    position: GridPos,
) -> impl Iterator<Item = GridPos> + '_ {
    [
        position
            .x
            .checked_add(1)
            .filter(|x| *x < navigation.width())
            .map(|x| GridPos { x, z: position.z }),
        position
            .x
            .checked_sub(1)
            .map(|x| GridPos { x, z: position.z }),
        position
            .z
            .checked_add(1)
            .filter(|z| *z < navigation.height())
            .map(|z| GridPos { x: position.x, z }),
        position
            .z
            .checked_sub(1)
            .map(|z| GridPos { x: position.x, z }),
    ]
    .into_iter()
    .flatten()
    .filter(|candidate| navigation.is_walkable(*candidate))
}

#[must_use]
pub fn legacy_v1_world_hash(world: &GeneratedWorld) -> String {
    let legacy_resources = legacy_variable_resource_amounts(world);
    let resources: Vec<_> = legacy_resources
        .iter()
        .filter(|resource| resource.target_kind.as_str() != "target:fish")
        .collect();
    let mut hasher = Sha256::new();
    hasher.update(world.seed.to_le_bytes());
    hasher.update(1_u32.to_le_bytes());
    let navigation = legacy_resource_navigation(world);
    for z in 0..navigation.height() {
        for x in 0..navigation.width() {
            let position = GridPos { x, z };
            hasher.update(
                navigation
                    .height_at(position)
                    .unwrap_or_default()
                    .to_le_bytes(),
            );
            hasher.update([u8::from(navigation.is_walkable(position))]);
        }
    }
    for resource in resources {
        hasher.update(resource.id.as_str().as_bytes());
        hasher.update(resource.kind.as_str().as_bytes());
        hasher.update(resource.position.x.to_le_bytes());
        hasher.update(resource.position.z.to_le_bytes());
        hasher.update(resource.amount.to_le_bytes());
    }
    hex::encode(hasher.finalize())
}

#[must_use]
pub fn legacy_v2_world_hash(world: &GeneratedWorld) -> String {
    let resources = legacy_variable_resource_amounts(world);
    hash_world(
        world.seed,
        2,
        &legacy_resource_navigation(world),
        &resources,
    )
}

#[must_use]
pub fn legacy_v3_world_hash(world: &GeneratedWorld) -> String {
    hash_world(
        world.seed,
        3,
        &world.navigation,
        &legacy_variable_resource_amounts(world),
    )
}

fn legacy_variable_resource_amounts(world: &GeneratedWorld) -> Vec<GeneratedResource> {
    const FISH_SEED_SALT: u64 = 0x4649_5348_5F53_484F;
    world
        .resources
        .iter()
        .cloned()
        .map(|mut resource| {
            let seed = if resource.target_kind.as_str() == "target:fish" {
                world.seed ^ FISH_SEED_SALT
            } else {
                world.seed
            };
            let random = cell_hash(seed, resource.position.x, resource.position.z);
            resource.amount =
                50 + u32::try_from((random >> 24) % 151).expect("bounded legacy amount");
            resource
        })
        .collect()
}

fn legacy_resource_navigation(world: &GeneratedWorld) -> NavGrid {
    let mut navigation = world.navigation.clone();
    for resource in &world.resources {
        if resource.target_kind.as_str() == "target:fish" {
            continue;
        }
        let region = crate::DirtyRegion {
            min: resource.position,
            max: resource.position,
        };
        navigation
            .set_blocked(region, false)
            .expect("generated resource position is inside navigation");
    }
    navigation
}

fn generate_foliage(
    config: &WorldGenConfig,
    navigation: &NavGrid,
    resources: &[GeneratedResource],
    layers: &[FoliageLayerDef],
) -> Vec<GeneratedFoliage> {
    let mut foliage = Vec::new();
    let mut occupied: std::collections::BTreeSet<_> = resources
        .iter()
        .filter(|resource| resource.target_kind.as_str() != "target:fish")
        .filter_map(|resource| {
            let centre = authored_grid_centre(config, resource.position);
            foliage_third_cell(
                config,
                centre[0] + f32::from(resource.offset_milli_cells[0]) * config.cell_size / 1_000.0,
                centre[1] + f32::from(resource.offset_milli_cells[1]) * config.cell_size / 1_000.0,
            )
        })
        .collect();
    for (layer_index, layer) in layers.iter().enumerate() {
        if layer.variants.is_empty() {
            continue;
        }
        let candidates = generate_foliage_candidate_mask(
            config,
            layer.source_size,
            layer.seed,
            layer.noise_scale,
            layer.octaves,
            layer.persistence,
            layer.lacunarity,
            layer.spawn_threshold,
            layer.spacing,
            town_layer_noise_offset(config.seed, &layer.id, layer.seed, layer.source_size),
        );
        for candidate in candidates {
            let jitter = foliage_candidate_jitter(
                config.seed,
                &layer.id,
                candidate.sample,
                config.cell_size / 3.0,
            );
            let world_x = candidate.world[0] + jitter[0];
            let world_z = candidate.world[1] + jitter[1];
            let Some(occupancy) = foliage_third_cell(config, world_x, world_z) else {
                continue;
            };
            if occupied.contains(&occupancy) {
                continue;
            }
            let Some(position) = authored_world_to_grid(config, world_x, world_z) else {
                continue;
            };
            let Some(terrain_height) =
                navigation_surface_height_at_world(navigation, config, world_x, world_z)
            else {
                continue;
            };
            let habitat_matches = match layer.habitat {
                FoliageHabitat::Land => terrain_height > visible_water_surface_height(config),
                FoliageHabitat::Underwater => terrain_height <= -0.50,
            };
            if !habitat_matches {
                continue;
            }
            let centre_world = authored_grid_centre(config, position);
            let offset_milli_cells = [
                unity_clamped_i16((world_x - centre_world[0]) / config.cell_size * 1_000.0),
                unity_clamped_i16((world_z - centre_world[1]) / config.cell_size * 1_000.0),
            ];
            let variant = u16::try_from(foliage_visual_variant(
                world_x,
                world_z,
                &layer.id,
                layer.variants.len(),
            ))
            .expect("foliage variant count fits u16");
            let yaw_milliradians = foliage_visual_yaw_milliradians(world_x, world_z, &layer.id);
            foliage.push(GeneratedFoliage {
                id: StableId::new(format!(
                    "foliage:{layer_index}:{}:{}",
                    candidate.source[0], candidate.source[1]
                ))
                .expect("generated stable foliage ID"),
                layer: layer.id.clone(),
                habitat: layer.habitat,
                position,
                offset_milli_cells,
                variant,
                yaw_milliradians,
                // Unity uses MeshSettings.BaseScale exactly; there is no
                // per-instance random scale in the shipping generator.
                scale_milli: 1_000,
            });
            occupied.insert(occupancy);
        }
    }
    foliage
}

#[derive(Clone, Copy)]
struct FoliageCandidate {
    /// Stable coordinates in the dense source lattice (x/z follow the legacy
    /// source naming, while `world` preserves its historical axis mapping).
    source: [i32; 2],
    sample: [u16; 2],
    world: [f32; 2],
}

/// Foliage used to sample once per half of a logical terrain cell. Keep the
/// authored physical extent and noise field, but evaluate a three-way logical
/// subdivision so foliage density matches fine navigation and paths.
#[allow(clippy::too_many_arguments)]
fn generate_foliage_candidate_mask(
    config: &WorldGenConfig,
    source_size: u16,
    seed: i32,
    noise_scale: f32,
    octaves: u8,
    persistence: f32,
    lacunarity: f32,
    threshold: f32,
    spacing: u16,
    noise_offset: [f32; 2],
) -> Vec<FoliageCandidate> {
    const LEGACY_SUBDIVISIONS: u16 = 2;
    const FOLIAGE_SUBDIVISIONS: u16 = 3;
    const LEGACY_BORDER_SAMPLES: i32 = 2;

    let dense_size = source_size.saturating_mul(FOLIAGE_SUBDIVISIONS) / LEGACY_SUBDIVISIONS;
    let source_scale = f32::from(FOLIAGE_SUBDIVISIONS) / f32::from(LEGACY_SUBDIVISIONS);
    let noise = unity_noise_map(
        dense_size,
        dense_size,
        seed,
        noise_scale * source_scale,
        octaves,
        persistence,
        lacunarity,
        noise_offset,
    );
    let mut candidates = Vec::new();
    let half = i32::from(dense_size / 2);
    let border =
        LEGACY_BORDER_SAMPLES * i32::from(FOLIAGE_SUBDIVISIONS) / i32::from(LEGACY_SUBDIVISIONS);
    let spacing_step = usize::from(spacing.max(1));
    let third_cell = config.cell_size / f32::from(FOLIAGE_SUBDIVISIONS);
    let centre = f32::from(spacing) * third_cell * 0.5;
    for source_z in (-half + border..half - border).step_by(spacing_step) {
        for source_x in (-half + border..half - border).step_by(spacing_step) {
            let sample_x = u16::try_from(source_x + half).expect("foliage sample x");
            let sample_z = u16::try_from(source_z + half).expect("foliage sample z");
            let index = usize::from(sample_z) * usize::from(dense_size) + usize::from(sample_x);
            if noise[index] < threshold {
                continue;
            }
            candidates.push(FoliageCandidate {
                source: [source_x, source_z],
                sample: [sample_x, sample_z],
                world: [
                    f32::from(i16::try_from(source_z).expect("foliage source z fits i16"))
                        * third_cell
                        + centre,
                    f32::from(i16::try_from(source_x).expect("foliage source x fits i16"))
                        * third_cell
                        + centre,
                ],
            });
        }
    }
    candidates
}

fn foliage_candidate_jitter(
    world_seed: u64,
    layer: &StableId,
    sample: [u16; 2],
    third_cell: f32,
) -> [f32; 2] {
    const MAXIMUM_JITTER_FRACTION: f32 = 0.38;
    let hash = cell_hash(
        world_seed ^ u64::from(stable_string_hash(layer.as_str())),
        sample[0],
        sample[1],
    );
    let signed = |bits: u64| {
        let unit = f32::from(u16::try_from(bits & 0xffff).expect("masked jitter fits u16"))
            / f32::from(u16::MAX);
        (unit * 2.0 - 1.0) * third_cell * MAXIMUM_JITTER_FRACTION
    };
    [signed(hash), signed(hash >> 32)]
}

fn foliage_third_cell(config: &WorldGenConfig, world_x: f32, world_z: f32) -> Option<(i32, i32)> {
    let subdivision = 3.0;
    let half_x = f32::from(config.width.saturating_sub(1)) * 0.5;
    let half_z = f32::from(config.height.saturating_sub(1)) * 0.5;
    let x = unity_rounded_i32((world_x / config.cell_size + half_x) * subdivision + 1.0);
    let z = unity_rounded_i32((world_z / config.cell_size + half_z) * subdivision + 1.0);
    let width = i32::from(config.width) * 3;
    let height = i32::from(config.height) * 3;
    (x >= 0 && z >= 0 && x < width && z < height).then_some((x, z))
}

#[allow(clippy::too_many_arguments)]
fn generate_candidate_mask(
    source_size: u16,
    seed: i32,
    noise_scale: f32,
    octaves: u8,
    persistence: f32,
    lacunarity: f32,
    threshold: f32,
    spacing: u16,
    wood_visual_offset: bool,
    noise_offset: [f32; 2],
) -> Vec<[i16; 2]> {
    let noise = unity_noise_map(
        source_size,
        source_size,
        seed,
        noise_scale,
        octaves,
        persistence,
        lacunarity,
        noise_offset,
    );
    let mut candidates = Vec::new();
    let half = i32::from(source_size / 2);
    let spacing_step = usize::from(spacing.max(1));
    for source_z in (-half + 2..half - 2).step_by(spacing_step) {
        for source_x in (-half + 2..half - 2).step_by(spacing_step) {
            let sample_x = usize::try_from(source_x + half).expect("candidate sample x");
            let sample_z = usize::try_from(source_z + half).expect("candidate sample z");
            if noise[sample_z * usize::from(source_size) + sample_x] < threshold {
                continue;
            }
            let centre = f32::from(spacing) * 0.5;
            let visual_offset = if wood_visual_offset { 0.5 } else { 0.0 };
            let world_x = f32::from(i16::try_from(source_z).expect("candidate z fits i16"))
                + centre
                + visual_offset;
            let world_z = f32::from(i16::try_from(source_x).expect("candidate x fits i16"))
                + centre
                + visual_offset;
            candidates.push([
                unity_clamped_i16(world_x * 2.0),
                unity_clamped_i16(world_z * 2.0),
            ]);
        }
    }
    candidates
}

fn authored_grid_centre(config: &WorldGenConfig, position: GridPos) -> [f32; 2] {
    [
        (f32::from(position.x) - f32::from(config.width.saturating_sub(1)) * 0.5)
            * config.cell_size,
        (f32::from(position.z) - f32::from(config.height.saturating_sub(1)) * 0.5)
            * config.cell_size,
    ]
}

#[must_use]
pub fn visible_water_surface_height(config: &WorldGenConfig) -> f32 {
    f32::from(config.water_level_centimetres) * 0.01 + VISIBLE_WATER_SURFACE_LIFT_METRES
}

fn averaged_terrain_corner_height(
    width: u16,
    height: u16,
    corner_x: u16,
    corner_z: u16,
    height_at: &impl Fn(u16, u16) -> f32,
) -> f32 {
    let min_x = corner_x.saturating_sub(1);
    let max_x = corner_x.min(width - 1);
    let min_z = corner_z.saturating_sub(1);
    let max_z = corner_z.min(height - 1);
    let mut total = 0.0_f32;
    let mut samples = 0_u16;
    for z in min_z..=max_z {
        for x in min_x..=max_x {
            total += height_at(x, z);
            samples += 1;
        }
    }
    total / f32::from(samples.max(1))
}

fn terrain_surface_height_with(
    width: u16,
    height: u16,
    cell_size: f32,
    world_x: f32,
    world_z: f32,
    height_at: impl Fn(u16, u16) -> f32,
) -> Option<f32> {
    if width == 0
        || height == 0
        || !cell_size.is_finite()
        || cell_size <= 0.0
        || !world_x.is_finite()
        || !world_z.is_finite()
    {
        return None;
    }
    let grid_x = world_x / cell_size + f32::from(width.saturating_sub(1)) * 0.5;
    let grid_z = world_z / cell_size + f32::from(height.saturating_sub(1)) * 0.5;
    let max_x = f32::from(width.saturating_sub(1));
    let max_z = f32::from(height.saturating_sub(1));
    if grid_x < 0.0 || grid_z < 0.0 || grid_x > max_x || grid_z > max_z {
        return None;
    }

    let grid_x = grid_x.clamp(0.0, max_x);
    let grid_z = grid_z.clamp(0.0, max_z);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let x0 = grid_x.floor() as u16;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let z0 = grid_z.floor() as u16;
    let x1 = x0.saturating_add(1).min(width - 1);
    let z1 = z0.saturating_add(1).min(height - 1);
    let fraction_x = grid_x - f32::from(x0);
    let fraction_z = grid_z - f32::from(z0);
    let corner = |x, z| averaged_terrain_corner_height(width, height, x, z, &height_at);

    let top_left = corner(x0, z0);
    if x0 == x1 && z0 == z1 {
        return Some(top_left);
    }
    let top_right = corner(x1, z0);
    if z0 == z1 {
        return Some(fraction_x.mul_add(top_right - top_left, top_left));
    }
    let bottom_left = corner(x0, z1);
    if x0 == x1 {
        return Some(fraction_z.mul_add(bottom_left - top_left, top_left));
    }
    let bottom_right = corner(x1, z1);
    if fraction_x + fraction_z <= 1.0 {
        Some(fraction_x.mul_add(
            top_right - top_left,
            fraction_z.mul_add(bottom_left - top_left, top_left),
        ))
    } else {
        Some((1.0 - fraction_z).mul_add(
            top_right - bottom_right,
            (1.0 - fraction_x).mul_add(bottom_left - bottom_right, bottom_right),
        ))
    }
}

fn terrain_surface_height_from_centimetres(
    config: &WorldGenConfig,
    heights: &[i16],
    world_x: f32,
    world_z: f32,
) -> Option<f32> {
    let expected = usize::from(config.width) * usize::from(config.height);
    if heights.len() != expected {
        return None;
    }
    terrain_surface_height_with(
        config.width,
        config.height,
        config.cell_size,
        world_x,
        world_z,
        |x, z| {
            f32::from(heights[usize::from(z) * usize::from(config.width) + usize::from(x)]) * 0.01
        },
    )
}

/// Returns the averaged height used for a generated terrain mesh vertex.
#[must_use]
pub fn navigation_corner_height_metres(navigation: &NavGrid, corner_x: u16, corner_z: u16) -> f32 {
    if navigation.width() == 0 || navigation.height() == 0 {
        return 0.0;
    }
    averaged_terrain_corner_height(
        navigation.width(),
        navigation.height(),
        corner_x,
        corner_z,
        &|x, z| f32::from(navigation.height_at(GridPos { x, z }).unwrap_or_default()) * 0.01,
    )
}

/// Samples the final triangles used by the generated terrain renderer.
///
/// This is the common authority for habitat classification and visual
/// grounding, so neither can accidentally use a pre-smoothed cell height.
#[must_use]
pub fn navigation_surface_height_at_world(
    navigation: &NavGrid,
    config: &WorldGenConfig,
    world_x: f32,
    world_z: f32,
) -> Option<f32> {
    terrain_surface_height_with(
        navigation.width(),
        navigation.height(),
        config.cell_size,
        world_x,
        world_z,
        |x, z| f32::from(navigation.height_at(GridPos { x, z }).unwrap_or_default()) * 0.01,
    )
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn authored_world_to_grid(config: &WorldGenConfig, world_x: f32, world_z: f32) -> Option<GridPos> {
    let half_x = f32::from(config.width.saturating_sub(1)) * 0.5;
    let half_z = f32::from(config.height.saturating_sub(1)) * 0.5;
    let x = (world_x / config.cell_size + half_x).round();
    let z = (world_z / config.cell_size + half_z).round();
    if x < 0.0 || z < 0.0 || x >= f32::from(config.width) || z >= f32::from(config.height) {
        return None;
    }
    Some(GridPos {
        x: x as u16,
        z: z as u16,
    })
}

#[allow(clippy::cast_possible_truncation)]
fn unity_rounded_i32(value: f32) -> i32 {
    value.round() as i32
}

#[allow(clippy::cast_possible_truncation)]
fn unity_floor_i32(value: f32) -> i32 {
    value.floor() as i32
}

#[allow(clippy::cast_possible_truncation)]
fn positive_noise_offset(seed: i32, size: u16) -> [f32; 2] {
    let mut random = SystemRandom::new(seed);
    let minimum = f32::from(size) * 0.5;
    let additional = f32::from(size) * 2.0;
    [
        minimum + random.next_double() as f32 * additional,
        minimum + random.next_double() as f32 * additional,
    ]
}

fn town_layer_noise_offset(
    world_seed: u64,
    layer_id: &StableId,
    authored_seed: i32,
    size: u16,
) -> [f32; 2] {
    let runtime_hash = town_layer_seed_hash(world_seed, layer_id);
    let reference_hash = town_layer_seed_hash(UNITY_REFERENCE_WORLD_SEED, layer_id);
    let authored_bits = u32::from_ne_bytes(authored_seed.to_ne_bytes());
    let offset_seed =
        i32::from_ne_bytes((authored_bits ^ runtime_hash ^ reference_hash).to_ne_bytes());
    positive_noise_offset(offset_seed, size)
}

fn town_layer_seed_hash(world_seed: u64, layer_id: &StableId) -> u32 {
    let mut hasher = Sha256::new();
    hasher.update(b"stream-town-authored-layer-offset-v1");
    hasher.update(world_seed.to_le_bytes());
    hasher.update(layer_id.as_str().as_bytes());
    let digest = hasher.finalize();
    u32::from_le_bytes([digest[0], digest[1], digest[2], digest[3]])
}

// Rust has no checked float-to-integer conversion. Every call mirrors Unity's
// rounded and clamped authored centimetre/offset serialization boundary.
#[allow(clippy::cast_possible_truncation)]
fn unity_clamped_i16(value: f32) -> i16 {
    value
        .round()
        .clamp(f32::from(i16::MIN), f32::from(i16::MAX)) as i16
}

fn smooth_noise_step(value: f32) -> f32 {
    value * value * (3.0 - 2.0 * value)
}

fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    start + (end - start) * amount
}

fn unity_terrain_height_curve(value: f32) -> f32 {
    if (0.0..=1.0).contains(&value) {
        smooth_noise_step(value)
    } else {
        value
    }
}

fn round_to_even(value: f32) -> f32 {
    let floor = value.floor();
    let fraction = value - floor;
    if (fraction - 0.5).abs() > f32::EPSILON {
        value.round()
    } else if (floor * 0.5).fract().abs() <= f32::EPSILON {
        floor
    } else {
        floor + 1.0
    }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::float_cmp,
    clippy::too_many_arguments
)]
fn unity_noise_map(
    width: u16,
    height: u16,
    seed: i32,
    scale: f32,
    octaves: u8,
    persistence: f32,
    lacunarity: f32,
    offset: [f32; 2],
) -> Vec<f32> {
    let mut random = SystemRandom::new(seed);
    let octave_offsets: Vec<_> = (0..octaves)
        .map(|_| {
            [
                random.next_range(-100_000, 100_000) as f32 + offset[0],
                random.next_range(-100_000, 100_000) as f32 + offset[1],
            ]
        })
        .collect();
    let scale = scale.max(0.000_1);
    let half_width = f32::from(width) * 0.5;
    let half_height = f32::from(height) * 0.5;
    let mut values = Vec::with_capacity(usize::from(width) * usize::from(height));
    let mut minimum = f32::MAX;
    let mut maximum = f32::MIN;
    for z in 0..height {
        for x in 0..width {
            let mut amplitude = 1.0;
            let mut frequency = 1.0_f32;
            let mut noise_height = 0.0;
            for octave_offset in &octave_offsets {
                // Mono evaluates this CLI floating-point expression at its
                // internal precision, then rounds at the native Mathf call.
                // The stored frequency local itself rounds to f32 each loop.
                let base_x = (f64::from(x) - f64::from(half_width)) / f64::from(scale);
                let base_z = (f64::from(z) - f64::from(half_height)) / f64::from(scale);
                let sample_x = (base_x * f64::from(frequency) + f64::from(octave_offset[0])) as f32;
                let sample_z = (base_z * f64::from(frequency) + f64::from(octave_offset[1])) as f32;
                noise_height += (perlin_noise(sample_x, sample_z) * 2.0 - 1.0) * amplitude;
                amplitude *= persistence;
                frequency *= lacunarity;
            }
            if noise_height > maximum {
                maximum = noise_height;
            } else if noise_height < minimum {
                minimum = noise_height;
            }
            values.push(noise_height);
        }
    }
    for value in &mut values {
        *value = if maximum == minimum {
            0.0
        } else {
            // Mathf.InverseLerp is another managed CLI floating-point
            // expression: its float arguments are fixed-width, while the
            // subtraction/division is evaluated at Mono's internal precision
            // before the float result crosses the Clamp01 call boundary.
            (((f64::from(*value) - f64::from(minimum)) / (f64::from(maximum) - f64::from(minimum)))
                as f32)
                .clamp(0.0, 1.0)
        };
    }
    values
}

// Algorithmic port of Unity 6000.5's native PerlinNoise::NoiseNormalized:
// absolute coordinates, its duplicated fixed permutation, improved-Perlin
// gradients, and Unity's measured normalization constants. No generated Unity
// terrain, masks, candidates, save positions, or coordinate fixtures are inputs.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn perlin_noise(x: f32, z: f32) -> f32 {
    const PERMUTATION: [u8; 256] = [
        151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30,
        69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94,
        252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171,
        168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60,
        211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1,
        216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
        164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118,
        126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
        213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39,
        253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34,
        242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49,
        192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
        138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
    ];
    let x = x.abs();
    let z = z.abs();
    let xi_integer = x.trunc();
    let zi_integer = z.trunc();
    let xi = (xi_integer as i32 & 255) as usize;
    let zi = (zi_integer as i32 & 255) as usize;
    let xf = x - xi_integer;
    let zf = z - zi_integer;
    let fade = |value: f32| value * value * value * (value * (value * 6.0 - 15.0) + 10.0);
    let hash = |first: usize, second: usize| {
        let inner = usize::from(PERMUTATION[first & 255]);
        let corner = usize::from(PERMUTATION[(inner + second) & 255]);
        PERMUTATION[corner]
    };
    let gradient = |hash: u8, x: f32, z: f32| {
        let hash = hash & 15;
        let first = if hash < 8 { x } else { z };
        let second = if hash < 4 {
            z
        } else if hash == 12 || hash == 14 {
            x
        } else {
            0.0
        };
        (if hash & 1 == 0 { first } else { -first }) + if hash & 2 == 0 { second } else { -second }
    };
    let u = fade(xf);
    let v = fade(zf);
    let lower = lerp(
        gradient(hash(xi, zi), xf, zf),
        gradient(hash(xi + 1, zi), xf - 1.0, zf),
        u,
    );
    let upper = lerp(
        gradient(hash(xi, zi + 1), xf, zf - 1.0),
        gradient(hash(xi + 1, zi + 1), xf - 1.0, zf - 1.0),
        u,
    );
    (lerp(lower, upper, v) + 0.69) / 1.483
}

struct SystemRandom {
    seed_array: [i32; 56],
    inext: usize,
    inextp: usize,
}

impl SystemRandom {
    fn new(seed: i32) -> Self {
        const BIG: i32 = i32::MAX;
        let subtraction = if seed == i32::MIN { BIG } else { seed.abs() };
        let mut mj = 161_803_398 - subtraction;
        if mj < 0 {
            mj += BIG;
        }
        let mut seed_array = [0; 56];
        seed_array[55] = mj;
        let mut mk = 1;
        for i in 1..55 {
            let ii = (21 * i) % 55;
            seed_array[ii] = mk;
            mk = mj - mk;
            if mk < 0 {
                mk += BIG;
            }
            mj = seed_array[ii];
        }
        for _ in 0..4 {
            for i in 1..56 {
                seed_array[i] -= seed_array[1 + (i + 30) % 55];
                if seed_array[i] < 0 {
                    seed_array[i] += BIG;
                }
            }
        }
        Self {
            seed_array,
            inext: 0,
            inextp: 21,
        }
    }

    fn sample(&mut self) -> i32 {
        self.inext += 1;
        if self.inext >= 56 {
            self.inext = 1;
        }
        self.inextp += 1;
        if self.inextp >= 56 {
            self.inextp = 1;
        }
        let mut result = self.seed_array[self.inext] - self.seed_array[self.inextp];
        if result == i32::MAX {
            result -= 1;
        }
        if result < 0 {
            result += i32::MAX;
        }
        self.seed_array[self.inext] = result;
        result
    }

    fn next_double(&mut self) -> f64 {
        f64::from(self.sample()) / f64::from(i32::MAX)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn next_range(&mut self, minimum: i32, maximum: i32) -> i32 {
        (self.next_double() * f64::from(maximum - minimum)) as i32 + minimum
    }
}

fn fnv_mix(mut hash: u32, value: u32) -> u32 {
    for shift in [0, 8, 16, 24] {
        hash = (hash ^ ((value >> shift) & 0xff)).wrapping_mul(16_777_619);
    }
    hash
}

fn stable_string_hash(value: &str) -> u32 {
    value.encode_utf16().fold(2_166_136_261, |hash, character| {
        fnv_mix(hash, u32::from(character))
    })
}

#[must_use]
pub fn foliage_visual_variant(world_x: f32, world_z: f32, layer: &StableId, count: usize) -> usize {
    unity_instance_index(
        world_x,
        world_z,
        0x31_C4_D2_u32 ^ stable_string_hash(layer.as_str()),
        count,
    )
}

#[must_use]
pub fn foliage_visual_yaw_milliradians(world_x: f32, world_z: f32, layer: &StableId) -> u16 {
    let quarter_turn = unity_instance_index(
        world_x,
        world_z,
        0x7B_29_F3_u32 ^ stable_string_hash(layer.as_str()),
        4,
    );
    u16::try_from(quarter_turn * 1_571).expect("four authored quarter turns fit u16")
}

#[must_use]
pub fn resource_visual_variant(world_x: f32, world_z: f32, kind: &StableId, count: usize) -> usize {
    let unity_resource = match kind.as_str() {
        "resource:wood" => 1,
        "resource:ore" => 2,
        "resource:food" => 3,
        "resource:gold" => 4,
        "resource:recruit" => 5,
        _ => 0,
    };
    unity_instance_index(world_x, world_z, 0x13_57_9B_u32 ^ unity_resource, count)
}

#[allow(clippy::cast_possible_truncation)]
fn unity_instance_index(world_x: f32, world_z: f32, salt: u32, count: usize) -> usize {
    if count == 0 {
        return 0;
    }
    let x = ((world_x * 1_000.0).round() as i32).cast_unsigned();
    let z = ((world_z * 1_000.0).round() as i32).cast_unsigned();
    let hash = avalanche_instance_hash(fnv_mix(fnv_mix(fnv_mix(2_166_136_261, x), z), salt));
    usize::try_from(hash).expect("u32 instance hash fits the target platform") % count
}

fn avalanche_instance_hash(mut hash: u32) -> u32 {
    // FNV's low bit mirrors regular coordinate parity, which made two-variant
    // foliage form diagonal checkerboard rows. Preserve the coordinate/salt
    // inputs shared with Unity, then avalanche every bit before modulo.
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x7FEB_352D);
    hash ^= hash >> 15;
    hash = hash.wrapping_mul(0x846C_A68B);
    hash ^ (hash >> 16)
}

fn cell_hash(seed: u64, x: u16, z: u16) -> u64 {
    let mut value = seed
        ^ (u64::from(x).wrapping_mul(0x9E37_79B9_7F4A_7C15))
        ^ (u64::from(z).wrapping_mul(0xBF58_476D_1CE4_E5B9));
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

fn hash_world(
    seed: u64,
    version: u32,
    navigation: &NavGrid,
    resources: &[GeneratedResource],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(seed.to_le_bytes());
    hasher.update(version.to_le_bytes());
    for z in 0..navigation.height() {
        for x in 0..navigation.width() {
            let position = GridPos { x, z };
            hasher.update(
                navigation
                    .height_at(position)
                    .unwrap_or_default()
                    .to_le_bytes(),
            );
            hasher.update([u8::from(navigation.is_walkable(position))]);
        }
    }
    for resource in resources {
        hasher.update(resource.id.as_str().as_bytes());
        hasher.update(resource.kind.as_str().as_bytes());
        hasher.update(resource.target_kind.as_str().as_bytes());
        hasher.update(resource.position.x.to_le_bytes());
        hasher.update(resource.position.z.to_le_bytes());
        if version >= 6 {
            hasher.update(resource.offset_milli_cells[0].to_le_bytes());
            hasher.update(resource.offset_milli_cells[1].to_le_bytes());
        }
        hasher.update(resource.amount.to_le_bytes());
    }
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use crate::{ContentCatalog, GameConfig};

    use super::*;

    #[test]
    #[allow(clippy::excessive_precision)]
    fn unity_native_perlin_port_matches_validation_samples() {
        let samples: [([f32; 2], f32); 14] = [
            ([0.0, 0.0], 0.465_273_08),
            ([0.1, 0.1], 0.521_736_98),
            ([0.5, 0.5], 0.296_695_9),
            ([1.25, -2.75], 0.569_467_96),
            ([-0.25, 0.75], 0.412_921_97),
            ([123.456, 789.012], 0.640_877_66),
            ([-999.75, 431.125], 0.657_504_8),
            ([100_000.125, -99_999.875], 0.281_942_93),
            ([33_797.949_218_75, 87_593.859_375], 0.556_386_23),
            ([24_469.505_859_375, -84_847.585_937_5], 0.458_267_84),
            ([-88_742.234_375, -43_157.328_125], 0.674_914_6),
            ([-41_509.246_093_75, -76_963.335_937_5], 0.464_331_9),
            ([-2_174_468.5, -2_253_224.5], -0.040_458_53),
            ([-50_637_696.0, -50_605_980.0], 0.465_273_08),
        ];
        for ([x, z], expected) in samples {
            assert_eq!(
                perlin_noise(x, z).to_bits(),
                expected.to_bits(),
                "Unity Perlin mismatch at ({x}, {z})"
            );
        }
    }

    #[test]
    #[allow(clippy::cast_precision_loss, clippy::excessive_precision)]
    fn algorithmic_generation_matches_unity_validation_fingerprints() {
        // These are output-only validation fingerprints emitted by Unity. They
        // can fail this test but cannot supply coordinates, terrain, masks, or
        // counts to the generator above.
        assert_eq!(
            positive_noise_offset(-1_165_233_549, 300).map(f32::to_bits),
            [
                549.771_972_656_25_f32.to_bits(),
                710.678_771_972_656_3_f32.to_bits()
            ]
        );
        let outer = positive_noise_offset(-1_165_233_549, 300);
        let mut offset_hasher = Sha256::new();
        let mut octave_random = SystemRandom::new(-1_165_233_549);
        for _ in 0..6 {
            let x = octave_random.next_range(-100_000, 100_000) as f32 + outer[0];
            let z = octave_random.next_range(-100_000, 100_000) as f32 + outer[1];
            offset_hasher.update(x.to_bits().to_le_bytes());
            offset_hasher.update(z.to_bits().to_le_bytes());
        }
        assert_eq!(
            hex::encode(offset_hasher.finalize()),
            "bd28ef8bcfc692dae0df71cee2ac4108d131fb4c0ac61f256cbc98cf96a01335"
        );
        let layers = [
            (
                300,
                -1_165_233_549,
                17.0,
                6,
                0.452,
                22.47,
                0.6,
                2,
                true,
                3_763,
                "b1d0baa74171a590bf1d6ca0972a255a7ee56f929a1fb4f9c21926c4322049fb",
            ),
            (
                300,
                -1_165_233_548,
                7.0,
                1,
                1.0,
                0.0,
                0.85,
                1,
                false,
                393,
                "2ffd55a61a479c2cb65a290e455f18cef63591074e6881af80e6e0052d4d61d1",
            ),
            (
                300,
                -1_165_233_547,
                7.0,
                2,
                1.0,
                0.0,
                0.85,
                1,
                false,
                130,
                "5ea5ddab9e13b56f4ee53181a5d889c958da6c1852609b83d9429a5c38ae5537",
            ),
            (
                300,
                -430_535_522,
                10.0,
                1,
                0.847,
                1.53,
                0.6,
                1,
                false,
                22_341,
                "bf56606b2a300bd2bd4fcd7a8db6dfd997bd6e4b2d1b5596dd827f148de10fb7",
            ),
            (
                300,
                -430_535_523,
                4.0,
                1,
                0.847,
                1.53,
                0.8,
                1,
                false,
                726,
                "138a87651288ae8e057f180033c42fac50fb94adb7136ebced6a169c3ded478a",
            ),
            (
                500,
                -430_535_520,
                6.68,
                1,
                0.8,
                1.53,
                0.7,
                1,
                false,
                21_680,
                "2f965ea209c5f2ef77fa6088253465f3f768bd3dfa2b09f7c6da2f5281e81e94",
            ),
            (
                500,
                -430_535_519,
                6.68,
                1,
                0.8,
                1.53,
                0.7,
                1,
                false,
                21_071,
                "661ae24165c7d685df8fc3c202b9a4191270ab5788d8d34f149af26f93877ffb",
            ),
        ];
        for (
            size,
            seed,
            scale,
            octaves,
            persistence,
            lacunarity,
            threshold,
            spacing,
            wood_offset,
            expected_count,
            expected_hash,
        ) in layers
        {
            let candidates = generate_candidate_mask(
                size,
                seed,
                scale,
                octaves,
                persistence,
                lacunarity,
                threshold,
                spacing,
                wood_offset,
                positive_noise_offset(seed, size),
            );
            assert_eq!(candidates.len(), expected_count, "seed {seed}");
            let mut hasher = Sha256::new();
            for [x, z] in candidates {
                hasher.update((f32::from(x) * 0.5).to_bits().to_le_bytes());
                hasher.update((f32::from(z) * 0.5).to_bits().to_le_bytes());
            }
            assert_eq!(hex::encode(hasher.finalize()), expected_hash, "seed {seed}");
        }

        let config = GameConfig::default().world;
        let world = generate_world(&config);
        let mut hasher = Sha256::new();
        for unity_y in 0..config.height {
            let bevy_z = config.height - 1 - unity_y;
            for x in 0..config.width {
                let height_metres = f32::from(
                    world
                        .navigation
                        .height_at(GridPos { x, z: bevy_z })
                        .unwrap(),
                ) / 100.0;
                hasher.update(height_metres.to_bits().to_le_bytes());
            }
        }
        assert_eq!(
            hex::encode(hasher.finalize()),
            "c4ef1e2b23cd38224676866f5467fe7fc98ad167e227be0013f7b7bbfd254455"
        );
    }

    #[test]
    fn generation_is_deterministic() {
        let config = GameConfig::default().world;
        let first = generate_world(&config);
        let second = generate_world(&config);
        assert_eq!(first.deterministic_hash, second.deterministic_hash);
        assert_eq!(first, second);
    }

    #[test]
    fn two_variant_foliage_hash_does_not_form_coordinate_parity_rows() {
        let layer = StableId::new("foliage:land:0").unwrap();
        let mut counts = [0_usize; 2];
        let mut equal_diagonal_neighbours = 0_usize;
        let mut different_diagonal_neighbours = 0_usize;
        let coordinate = |value: usize| f32::from(u16::try_from(value).unwrap()) * 0.5;
        for z in 0..32 {
            for x in 0..32 {
                let variant = foliage_visual_variant(coordinate(x), coordinate(z), &layer, 2);
                counts[variant] += 1;
                if x > 0 && z > 0 {
                    let diagonal =
                        foliage_visual_variant(coordinate(x - 1), coordinate(z - 1), &layer, 2);
                    if variant == diagonal {
                        equal_diagonal_neighbours += 1;
                    } else {
                        different_diagonal_neighbours += 1;
                    }
                }
            }
        }
        assert!(counts.into_iter().all(|count| (400..=624).contains(&count)));
        assert!(equal_diagonal_neighbours > 250);
        assert!(different_diagonal_neighbours > 250);
    }

    #[test]
    fn two_variant_resource_hash_does_not_form_coordinate_parity_rows() {
        let kind = StableId::new("resource:wood").unwrap();
        let mut counts = [0_usize; 2];
        let mut equal_diagonal_neighbours = 0_usize;
        let mut different_diagonal_neighbours = 0_usize;
        let coordinate = |value: usize| f32::from(u16::try_from(value).unwrap()) * 0.5;
        for z in 0..32 {
            for x in 0..32 {
                let variant = resource_visual_variant(coordinate(x), coordinate(z), &kind, 2);
                counts[variant] += 1;
                if x > 0 && z > 0 {
                    let diagonal =
                        resource_visual_variant(coordinate(x - 1), coordinate(z - 1), &kind, 2);
                    if variant == diagonal {
                        equal_diagonal_neighbours += 1;
                    } else {
                        different_diagonal_neighbours += 1;
                    }
                }
            }
        }
        assert!(counts.into_iter().all(|count| (400..=624).contains(&count)));
        assert!(equal_diagonal_neighbours > 250);
        assert!(different_diagonal_neighbours > 250);
    }

    #[test]
    fn observed_generation_reports_every_real_stage_without_changing_output() {
        let config = GameConfig::default().world;
        let content: ContentCatalog =
            ron::from_str(include_str!("../../../assets/content/catalog.ron")).unwrap();
        let expected = generate_world_with_content(&config, &content);
        let mut completed = Vec::new();
        let observed =
            generate_world_with_content_observed(&config, &content, |stage| completed.push(stage));

        assert_eq!(observed, expected);
        assert_eq!(completed, WorldGenerationStage::ALL);
    }

    #[test]
    fn changing_seed_changes_world_hash() {
        let mut config = GameConfig::default().world;
        let first = generate_world(&config).deterministic_hash;
        config.seed += 1;
        assert_ne!(first, generate_world(&config).deterministic_hash);
    }

    #[test]
    fn town_seed_randomizes_every_authored_perlin_layer_offset() {
        let content: ContentCatalog =
            ron::from_str(include_str!("../../../assets/content/catalog.ron")).unwrap();
        // Stable FNV-1a value produced by the game menu for "Bobville".
        let alternate_seed = 14_812_036_045_316_836_008;

        for layer in &content.resource_generation {
            if layer.habitat != ResourceGenerationHabitat::Land {
                continue;
            }
            let reference = town_layer_noise_offset(
                UNITY_REFERENCE_WORLD_SEED,
                &layer.id,
                layer.seed,
                layer.source_size,
            );
            let alternate =
                town_layer_noise_offset(alternate_seed, &layer.id, layer.seed, layer.source_size);
            assert_eq!(
                reference.map(f32::to_bits),
                positive_noise_offset(layer.seed, layer.source_size).map(f32::to_bits)
            );
            assert_ne!(
                alternate.map(f32::to_bits),
                reference.map(f32::to_bits),
                "layer {} reused its offset",
                layer.id
            );
            assert_eq!(
                alternate.map(f32::to_bits),
                town_layer_noise_offset(alternate_seed, &layer.id, layer.seed, layer.source_size,)
                    .map(f32::to_bits)
            );
        }

        for layer in &content.foliage {
            let reference = town_layer_noise_offset(
                UNITY_REFERENCE_WORLD_SEED,
                &layer.id,
                layer.seed,
                layer.source_size,
            );
            let alternate =
                town_layer_noise_offset(alternate_seed, &layer.id, layer.seed, layer.source_size);
            assert_eq!(
                reference.map(f32::to_bits),
                positive_noise_offset(layer.seed, layer.source_size).map(f32::to_bits)
            );
            assert_ne!(
                alternate.map(f32::to_bits),
                reference.map(f32::to_bits),
                "layer {} reused its offset",
                layer.id
            );
            assert_eq!(
                alternate.map(f32::to_bits),
                town_layer_noise_offset(alternate_seed, &layer.id, layer.seed, layer.source_size,)
                    .map(f32::to_bits)
            );
        }
    }

    #[test]
    fn different_town_seeds_produce_different_resource_and_foliage_layouts() {
        let mut config = GameConfig::default().world;
        let content: ContentCatalog =
            ron::from_str(include_str!("../../../assets/content/catalog.ron")).unwrap();
        let reference = generate_world_with_content(&config, &content);
        // Stable FNV-1a value produced by the game menu for "Bobville".
        config.seed = 14_812_036_045_316_836_008;
        let alternate = generate_world_with_content(&config, &content);
        let repeated = generate_world_with_content(&config, &content);

        assert_eq!(alternate, repeated);
        for target in ["target:tree", "target:ore", "target:bush"] {
            let positions = |world: &GeneratedWorld| {
                world
                    .resources
                    .iter()
                    .filter(|resource| resource.target_kind.as_str() == target)
                    .map(|resource| (resource.position, resource.offset_milli_cells))
                    .collect::<Vec<_>>()
            };
            assert_ne!(
                positions(&reference),
                positions(&alternate),
                "{target} layout did not respond to the town seed"
            );
        }
        assert_ne!(reference.foliage, alternate.foliage);
    }

    #[test]
    fn generated_resources_preserve_unity_target_types_and_reachable_fish() {
        let config = GameConfig::default().world;
        let world = generate_world(&config);
        assert_eq!(world.generator_version, 9);
        assert_ne!(legacy_v1_world_hash(&world), world.deterministic_hash);
        assert_ne!(legacy_v2_world_hash(&world), world.deterministic_hash);
        assert_ne!(legacy_v3_world_hash(&world), world.deterministic_hash);
        for resource in &world.resources {
            match resource.target_kind.as_str() {
                "target:tree" => {
                    assert_eq!(resource.kind.as_str(), "resource:wood");
                    assert_eq!(resource.amount, 500);
                }
                "target:ore" => {
                    assert_eq!(resource.kind.as_str(), "resource:ore");
                    assert_eq!(resource.amount, 10_000);
                }
                "target:bush" => {
                    assert_eq!(resource.kind.as_str(), "resource:food");
                    assert_eq!(resource.amount, 10_000);
                }
                "target:fish" => {
                    assert_eq!(resource.kind.as_str(), "resource:food");
                    assert_eq!(resource.amount, 100);
                }
                kind => panic!("unexpected generated resource target {kind}"),
            }
        }
        let fish: Vec<_> = world
            .resources
            .iter()
            .filter(|resource| resource.target_kind.as_str() == "target:fish")
            .collect();
        assert!(!fish.is_empty());
        for resource in fish {
            assert!(!world.navigation.is_walkable(resource.position));
            assert!(
                shoreline_approaches(&world.navigation, resource.position)
                    .next()
                    .is_some()
            );
        }
        for resource in world
            .resources
            .iter()
            .filter(|resource| resource.target_kind.as_str() != "target:fish")
            .filter(|resource| {
                resource.position
                    != GridPos {
                        x: config.width / 2,
                        z: config.height / 2,
                    }
            })
        {
            assert!(!world.navigation.is_walkable(resource.position));
        }
    }

    #[test]
    fn final_surface_height_overrides_a_raw_land_cell_at_the_waterline() {
        let mut config = GameConfig::default().world;
        config.width = 3;
        config.height = 3;
        config.cell_size = 2.0;
        config.water_level_centimetres = 5;
        let mut heights = vec![-100; 9];
        heights[4] = 100;
        let navigation = NavGrid::new(3, 3, vec![false; 9], heights).unwrap();

        assert!(
            navigation.height_at(GridPos { x: 1, z: 1 }).unwrap() > config.water_level_centimetres
        );
        let final_height =
            navigation_surface_height_at_world(&navigation, &config, 0.0, 0.0).unwrap();
        assert!(final_height <= visible_water_surface_height(&config));
    }

    #[test]
    fn authored_foliage_is_deterministic_and_respects_habitat_and_resources() {
        let config = GameConfig::default().world;
        let mut content: ContentCatalog =
            ron::from_str(include_str!("../../../assets/content/catalog.ron")).unwrap();
        let first = generate_world_with_content(&config, &content);
        let second = generate_world_with_content(&config, &content);
        assert_eq!(first.foliage, second.foliage);
        assert!(!first.foliage.is_empty());
        assert!(
            first
                .foliage
                .iter()
                .any(|foliage| foliage.habitat == FoliageHabitat::Land)
        );
        // The shipping terrain has no cells below the underwater foliage
        // threshold, so its two authored underwater layers correctly emit no
        // instances for this seed.
        assert!(
            first
                .foliage
                .iter()
                .all(|foliage| foliage.habitat == FoliageHabitat::Land)
        );
        let ids: std::collections::BTreeSet<_> =
            first.foliage.iter().map(|foliage| &foliage.id).collect();
        assert_eq!(ids.len(), first.foliage.len());
        let mut occupied_thirds = std::collections::BTreeSet::new();
        let mut visibly_jittered = 0_usize;
        for foliage in &first.foliage {
            let centre = authored_grid_centre(&config, foliage.position);
            let world_x =
                centre[0] + f32::from(foliage.offset_milli_cells[0]) * config.cell_size / 1_000.0;
            let world_z =
                centre[1] + f32::from(foliage.offset_milli_cells[1]) * config.cell_size / 1_000.0;
            let third = foliage_third_cell(&config, world_x, world_z)
                .expect("generated foliage remains inside the third-cell grid");
            assert!(occupied_thirds.insert(third));
            let fine_coordinate = |world: f32, size: u16| {
                (world / config.cell_size + f32::from(size.saturating_sub(1)) * 0.5) * 3.0 + 1.0
            };
            let jitter = [
                (fine_coordinate(world_x, config.width).round()
                    - fine_coordinate(world_x, config.width))
                .abs()
                    * config.cell_size
                    / 3.0,
                (fine_coordinate(world_z, config.height).round()
                    - fine_coordinate(world_z, config.height))
                .abs()
                    * config.cell_size
                    / 3.0,
            ];
            assert!(jitter.into_iter().all(|offset| offset <= 0.26));
            visibly_jittered += usize::from(jitter.into_iter().any(|offset| offset >= 0.03));
            let final_height =
                navigation_surface_height_at_world(&first.navigation, &config, world_x, world_z)
                    .unwrap();
            assert!(final_height > visible_water_surface_height(&config));
        }
        assert!(visibly_jittered * 10 > first.foliage.len() * 9);
        content.foliage[0].noise_scale *= 2.0;
        let altered = generate_world_with_content(&config, &content);
        assert_ne!(first.foliage, altered.foliage);
        content.foliage.truncate(1);
        content.foliage[0].id = StableId::new("foliage:test-custom-layer").unwrap();
        let custom = generate_world_with_content(&config, &content);
        assert!(!custom.foliage.is_empty());
        assert!(
            custom
                .foliage
                .iter()
                .all(|foliage| foliage.layer == content.foliage[0].id)
        );
        content.foliage.clear();
        let without_foliage = generate_world_with_content(&config, &content);
        assert!(without_foliage.foliage.is_empty());
        assert_eq!(first.deterministic_hash, without_foliage.deterministic_hash);
    }

    #[test]
    fn final_surface_filtered_generation_remains_deterministic() {
        // StreamTownSave.stsave is never loaded by the runtime. These are the
        // horizontal fingerprints recorded from the shipping reference for
        // this seed. Final-height habitat
        // filtering deliberately removes source placements which end up under
        // the visible water plane, while remaining deterministic.
        let mut config = GameConfig::default().world;
        config.seed = 1_580_290_387;
        let content: ContentCatalog =
            ron::from_str(include_str!("../../../assets/content/catalog.ron")).unwrap();
        let world = generate_world_with_content(&config, &content);
        let resource_counts = world.resources.iter().fold(
            std::collections::BTreeMap::<&str, usize>::new(),
            |mut counts, resource| {
                if resource.target_kind.as_str() != "target:fish" {
                    *counts.entry(resource.kind.as_str()).or_default() += 1;
                }
                counts
            },
        );
        let foliage_counts = world.foliage.iter().fold(
            std::collections::BTreeMap::<&str, usize>::new(),
            |mut counts, foliage| {
                *counts.entry(foliage.layer.as_str()).or_default() += 1;
                counts
            },
        );
        assert_eq!(
            resource_counts,
            std::collections::BTreeMap::from([
                ("resource:food", 87),
                ("resource:ore", 305),
                ("resource:wood", 2_928),
            ])
        );
        assert_eq!(
            resource_horizontal_hash(&world, "resource:wood", &config),
            "52c5f489d930ac3589d00dc9e2ef3825a969160b95e6d7d120a40e4588b6f7a3"
        );
        assert_eq!(
            resource_horizontal_hash(&world, "resource:ore", &config),
            "154fd994bca9b9abbbb22e51f4aceb9806718407c40d4d5bef1751257e6ebef8"
        );
        assert_eq!(
            resource_horizontal_hash(&world, "resource:food", &config),
            "fdec18e9f4e7017cf654f9872004b98115294537fcab4579505d5cee30dfe714"
        );
        assert_eq!(
            foliage_counts,
            std::collections::BTreeMap::from(
                [("foliage:land:0", 36_797), ("foliage:land:1", 523),]
            )
        );
        assert_eq!(
            foliage_horizontal_hash(&world, "foliage:land:1", &config),
            "ebb449253e77bdeb07fec40d795e44a3c6666c9e75bd53afaf30953fda42820f"
        );
    }

    fn resource_horizontal_hash(
        world: &GeneratedWorld,
        kind: &str,
        config: &WorldGenConfig,
    ) -> String {
        let positions = world
            .resources
            .iter()
            .filter(|resource| {
                resource.kind.as_str() == kind && resource.target_kind.as_str() != "target:fish"
            })
            .map(|resource| {
                let centre = authored_grid_centre(config, resource.position);
                [
                    centre[0]
                        + f32::from(resource.offset_milli_cells[0]) * config.cell_size / 1_000.0,
                    centre[1]
                        + f32::from(resource.offset_milli_cells[1]) * config.cell_size / 1_000.0,
                ]
            });
        horizontal_hash(positions)
    }

    fn foliage_horizontal_hash(
        world: &GeneratedWorld,
        layer: &str,
        config: &WorldGenConfig,
    ) -> String {
        let positions = world
            .foliage
            .iter()
            .filter(|foliage| foliage.layer.as_str() == layer)
            .map(|foliage| {
                let centre = authored_grid_centre(config, foliage.position);
                [
                    centre[0]
                        + f32::from(foliage.offset_milli_cells[0]) * config.cell_size / 1_000.0,
                    centre[1]
                        + f32::from(foliage.offset_milli_cells[1]) * config.cell_size / 1_000.0,
                ]
            });
        horizontal_hash(positions)
    }

    fn horizontal_hash(positions: impl Iterator<Item = [f32; 2]>) -> String {
        let mut positions = positions.collect::<Vec<_>>();
        positions.sort_by(|left, right| {
            left[0]
                .total_cmp(&right[0])
                .then(left[1].total_cmp(&right[1]))
        });
        let mut hasher = Sha256::new();
        for position in positions {
            hasher.update(position[0].to_bits().to_le_bytes());
            hasher.update(position[1].to_bits().to_le_bytes());
        }
        hex::encode(hasher.finalize())
    }
}
