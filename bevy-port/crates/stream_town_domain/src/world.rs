use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{FoliageHabitat, FoliageLayerDef, GridPos, NavGrid, StableId, WorldGenConfig};

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

#[must_use]
pub fn generate_world(config: &WorldGenConfig) -> GeneratedWorld {
    generate_world_from_layers(config, &[])
}

#[must_use]
pub fn generate_world_with_content(
    config: &WorldGenConfig,
    content: &crate::ContentCatalog,
) -> GeneratedWorld {
    generate_world_from_layers(config, &content.foliage)
}

fn generate_world_from_layers(
    config: &WorldGenConfig,
    foliage_layers: &[FoliageLayerDef],
) -> GeneratedWorld {
    const GENERATOR_VERSION: u32 = 6;
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

    let mut resources = generate_authored_resources(config, &heights);

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
    generate_shoreline_fish(config, &navigation, &mut resources);
    let foliage = generate_foliage(config, &navigation, &resources, foliage_layers);
    // Decorative foliage is regenerated from authored content and deliberately
    // excluded from native-save compatibility. The saved world fingerprint
    // continues to describe terrain, navigation, and gameplay resources only.
    let deterministic_hash = hash_world(config.seed, GENERATOR_VERSION, &navigation, &resources);
    GeneratedWorld {
        seed: config.seed,
        generator_version: GENERATOR_VERSION,
        navigation,
        resources,
        foliage,
        deterministic_hash,
    }
}

#[derive(Clone, Copy)]
struct AuthoredResourceLayer {
    seed: i32,
    noise_scale: f32,
    octaves: u8,
    persistence: f32,
    lacunarity: f32,
    threshold: f32,
    spacing: u16,
    kind: &'static str,
    target_kind: &'static str,
}

const AUTHORED_RESOURCE_LAYERS: [AuthoredResourceLayer; 3] = [
    AuthoredResourceLayer {
        seed: -1_165_233_549,
        noise_scale: 17.0,
        octaves: 6,
        persistence: 0.452,
        lacunarity: 22.47,
        threshold: 0.6,
        spacing: 2,
        kind: "resource:wood",
        target_kind: "target:tree",
    },
    AuthoredResourceLayer {
        seed: -1_165_233_548,
        noise_scale: 7.0,
        octaves: 1,
        persistence: 1.0,
        lacunarity: 0.0,
        threshold: 0.85,
        spacing: 1,
        kind: "resource:ore",
        target_kind: "target:ore",
    },
    AuthoredResourceLayer {
        seed: -1_165_233_547,
        noise_scale: 7.0,
        octaves: 2,
        persistence: 1.0,
        lacunarity: 0.0,
        threshold: 0.85,
        spacing: 1,
        kind: "resource:food",
        target_kind: "target:bush",
    },
];

fn generate_authored_resources(config: &WorldGenConfig, heights: &[i16]) -> Vec<GeneratedResource> {
    let mut resources = Vec::new();
    // Unity shares one HashSet<(int, int)> across all resource and foliage
    // layers. Each layer computes those keys using its own spacing; preserving
    // that slightly unusual behavior is required for placement parity.
    let mut occupied = std::collections::BTreeSet::<(i32, i32)>::new();
    for (layer_index, layer) in AUTHORED_RESOURCE_LAYERS.iter().enumerate() {
        let candidates = generate_candidate_mask(
            300,
            layer.seed,
            layer.noise_scale,
            layer.octaves,
            layer.persistence,
            layer.lacunarity,
            layer.threshold,
            layer.spacing,
            layer.kind == "resource:wood",
        );
        for candidate in candidates {
            let resource_world_x = f32::from(candidate[0]) * 0.5;
            let resource_world_z = f32::from(candidate[1]) * 0.5;
            let wood_offset = if layer.kind == "resource:wood" {
                0.5
            } else {
                0.0
            };
            let world_x = resource_world_x - wood_offset;
            let world_z = resource_world_z - wood_offset;
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
            let Some(terrain_position) = authored_world_to_grid(config, world_x, world_z) else {
                continue;
            };
            let index = usize::from(terrain_position.z) * usize::from(config.width)
                + usize::from(terrain_position.x);
            if heights[index] <= 0 {
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
                kind: StableId::new(layer.kind).expect("authored resource kind ID"),
                target_kind: StableId::new(layer.target_kind).expect("authored target kind ID"),
                position,
                offset_milli_cells,
                generation_occupancy: [
                    i16::try_from(occupancy.0).expect("authored occupancy x fits i16"),
                    i16::try_from(occupancy.1).expect("authored occupancy z fits i16"),
                ],
                amount: 100,
            });
        }
    }
    resources
}

fn generate_shoreline_fish(
    config: &WorldGenConfig,
    navigation: &NavGrid,
    resources: &mut Vec<GeneratedResource>,
) {
    const FISH_SEED_SALT: u64 = 0x4649_5348_5F53_484F;
    let density = config
        .resource_density_per_thousand
        .saturating_mul(4)
        .min(1_000);
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
            let random = cell_hash(config.seed ^ FISH_SEED_SALT, x, z);
            let roll = u16::try_from(random % 1_000).expect("modulo 1000");
            if roll >= density {
                continue;
            }
            resources.push(GeneratedResource {
                id: StableId::new(format!("resource:fish:{x}:{z}")).expect("generated stable ID"),
                kind: StableId::new("resource:food").expect("static stable ID"),
                target_kind: StableId::new("target:fish").expect("static stable ID"),
                position,
                offset_milli_cells: [0, 0],
                generation_occupancy: [
                    i16::try_from(x).expect("world x fits i16"),
                    i16::try_from(z).expect("world z fits i16"),
                ],
                amount: 100,
            });
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
        .map(|resource| {
            (
                i32::from(resource.generation_occupancy[0]),
                i32::from(resource.generation_occupancy[1]),
            )
        })
        .collect();
    for (layer_index, layer) in layers.iter().enumerate() {
        if layer.variants.is_empty() {
            continue;
        }
        let candidates = generate_candidate_mask(
            layer.source_size,
            layer.seed,
            layer.noise_scale,
            layer.octaves,
            layer.persistence,
            layer.lacunarity,
            layer.spawn_threshold,
            layer.spacing,
            false,
        );
        for candidate in candidates {
            let world_x = f32::from(candidate[0]) * 0.5;
            let world_z = f32::from(candidate[1]) * 0.5;
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
            let Some(position) = authored_world_to_grid(config, world_x, world_z) else {
                continue;
            };
            let terrain_height = navigation.height_at(position).unwrap_or_default();
            let habitat_matches = match layer.habitat {
                FoliageHabitat::Land => terrain_height > 0,
                FoliageHabitat::Underwater => terrain_height <= -50,
            };
            if !habitat_matches {
                continue;
            }
            let centre_world = authored_grid_centre(config, position);
            let offset_milli_cells = [
                unity_clamped_i16((world_x - centre_world[0]) / config.cell_size * 1_000.0),
                unity_clamped_i16((world_z - centre_world[1]) / config.cell_size * 1_000.0),
            ];
            let variant = u16::try_from(unity_instance_index(
                world_x,
                world_z,
                0x31_C4_D2_u32 ^ stable_string_hash(layer.id.as_str()),
                layer.variants.len(),
            ))
            .expect("foliage variant count fits u16");
            let quarter_turn = unity_instance_index(
                world_x,
                world_z,
                0x7B_29_F3_u32 ^ stable_string_hash(layer.id.as_str()),
                4,
            );
            let yaw_milliradians =
                u16::try_from(quarter_turn * 1_571).expect("four authored quarter turns fit u16");
            foliage.push(GeneratedFoliage {
                id: StableId::new(format!("foliage:{layer_index}:{source_x}:{source_z}"))
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
) -> Vec<[i16; 2]> {
    let offset = positive_noise_offset(seed, source_size);
    let noise = unity_noise_map(
        source_size,
        source_size,
        seed,
        noise_scale,
        octaves,
        persistence,
        lacunarity,
        offset,
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

#[allow(clippy::cast_possible_truncation)]
fn unity_instance_index(world_x: f32, world_z: f32, salt: u32, count: usize) -> usize {
    if count == 0 {
        return 0;
    }
    let x = ((world_x * 1_000.0).round() as i32).cast_unsigned();
    let z = ((world_z * 1_000.0).round() as i32).cast_unsigned();
    let hash = fnv_mix(fnv_mix(fnv_mix(2_166_136_261, x), z), salt);
    usize::try_from(hash).expect("u32 instance hash fits the target platform") % count
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
    fn changing_seed_changes_world_hash() {
        let mut config = GameConfig::default().world;
        let first = generate_world(&config).deterministic_hash;
        config.seed += 1;
        assert_ne!(first, generate_world(&config).deterministic_hash);
    }

    #[test]
    fn generated_resources_preserve_unity_target_types_and_reachable_fish() {
        let config = GameConfig::default().world;
        let world = generate_world(&config);
        assert_eq!(world.generator_version, 6);
        assert_ne!(legacy_v1_world_hash(&world), world.deterministic_hash);
        assert_ne!(legacy_v2_world_hash(&world), world.deterministic_hash);
        assert_ne!(legacy_v3_world_hash(&world), world.deterministic_hash);
        assert!(
            world
                .resources
                .iter()
                .all(|resource| resource.amount == 100)
        );
        for resource in &world.resources {
            match resource.kind.as_str() {
                "resource:wood" => assert_eq!(resource.target_kind.as_str(), "target:tree"),
                "resource:ore" => assert_eq!(resource.target_kind.as_str(), "target:ore"),
                "resource:food" => assert!(matches!(
                    resource.target_kind.as_str(),
                    "target:bush" | "target:fish"
                )),
                kind => panic!("unexpected generated resource kind {kind}"),
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
        for foliage in &first.foliage {
            assert!(first.navigation.height_at(foliage.position).unwrap() > 0);
        }
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
    fn generated_instance_counts_match_the_sanitized_unity_save_oracle() {
        // StreamTownSave.stsave is never loaded by the runtime. These are the
        // non-personal generation counts and horizontal fingerprints exported
        // from it by `stream_town_migrate export-world-oracle` for the recorded
        // seed. They are expected values inside this test only. The generator
        // above neither reads the save nor receives any of its placements.
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
                ("resource:food", 92),
                ("resource:ore", 300),
                ("resource:wood", 2_928),
            ])
        );
        assert_eq!(
            resource_horizontal_hash(&world, "resource:wood", &config),
            "8eeeb2da50572b867aee1cde8f4d6575c78eab02bd35e4d35d54aa845771e42f"
        );
        assert_eq!(
            resource_horizontal_hash(&world, "resource:ore", &config),
            "d5b138a1cf7c3d1bc60c218b0781da7282e7c8cd9ac06f417ce0562a2603ac52"
        );
        assert_eq!(
            resource_horizontal_hash(&world, "resource:food", &config),
            "b0d29635cb29143bb83d447444169866339e3cbec569c80746f25b59f36a7c63"
        );
        assert_eq!(
            foliage_counts,
            std::collections::BTreeMap::from([
                // The live save contains 16,203 because seven of these 16,210
                // generated grass instances are beneath its player-built
                // Lumbermill at (-8, -1). That post-generation removal must not
                // be baked into Bevy's generator.
                ("foliage:land:0", 16_210),
                ("foliage:land:1", 371),
            ])
        );
        assert_eq!(
            foliage_horizontal_hash(&world, "foliage:land:1", &config),
            "a23679cfeeea54bbd5cdbfe2f7aa6a4f63e28b22e64fe84891a081be320a8ade"
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
