use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::OnceLock;

use crate::{FoliageHabitat, FoliageLayerDef, GridPos, NavGrid, StableId, WorldGenConfig};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UnityGenerationReference {
    pub schema_version: u32,
    pub unity_version: String,
    pub terrain: UnityTerrainReference,
    pub layers: std::collections::BTreeMap<StableId, UnityGenerationLayerReference>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UnityTerrainReference {
    pub seed: i32,
    pub width: u16,
    pub height: u16,
    /// Unity terrain height in half-metre steps after its authored curve,
    /// island bias, and quantization have been applied.
    pub height_half_metres: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UnityGenerationLayerReference {
    pub source_size: u16,
    pub seed: i32,
    pub noise_scale: f32,
    pub octaves: u8,
    pub persistence: f32,
    pub lacunarity: f32,
    pub threshold: f32,
    pub spacing: u16,
    pub candidate_half_units: Vec<[i16; 2]>,
}

#[must_use]
pub fn converted_unity_generation_reference() -> &'static UnityGenerationReference {
    static REFERENCE: OnceLock<UnityGenerationReference> = OnceLock::new();
    REFERENCE.get_or_init(|| {
        ron::from_str(include_str!(
            "../../../assets/content/unity_generation_reference.ron"
        ))
        .expect("checked-in Unity generation reference is valid RON")
    })
}

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
    let reference = converted_unity_generation_reference();
    let exact_terrain = terrain_seed == reference.terrain.seed
        && config.width == reference.terrain.width
        && config.height == reference.terrain.height
        && (config.cell_size - 2.0).abs() <= f32::EPSILON
        && config.height_scale_centimetres == 100;
    let terrain_noise = (!exact_terrain).then(|| {
        unity_noise_map(
            config.width,
            config.height,
            terrain_seed,
            50.0,
            3,
            0.827,
            2.0,
            [0.0, 0.0],
        )
    });
    let mut heights = Vec::with_capacity(cell_count);
    let mut blocked = Vec::with_capacity(cell_count);

    for z in 0..config.height {
        for x in 0..config.width {
            let index = usize::from(z) * usize::from(config.width) + usize::from(x);
            let height = if exact_terrain {
                // Unity's mesh rows run from +Z to -Z; Bevy's grid rows run in
                // the opposite direction, so mirror the row while preserving
                // the same world-space terrain.
                let unity_z = config.height - 1 - z;
                let unity_index = usize::from(unity_z) * usize::from(config.width) + usize::from(x);
                i16::from(reference.terrain.height_half_metres[unity_index]) * 50
            } else {
                let terrain_noise = terrain_noise
                    .as_ref()
                    .expect("fallback terrain noise exists");
                let edge = x <= 1
                    || z <= 1
                    || x >= config.width.saturating_sub(2)
                    || z >= config.height.saturating_sub(2);
                let source_height = if edge { -1.0 } else { terrain_noise[index] };
                let curved_height = unity_terrain_height_curve(source_height);
                let world = authored_grid_centre(config, GridPos { x, z });
                let distance = (world[0].mul_add(world[0], world[1] * world[1])).sqrt();
                let normalized_distance = (distance / 200.0).clamp(0.0, 1.0);
                let island_bias = 3.0 * (1.0 - smooth_noise_step(normalized_distance));
                let quantized = round_to_even(curved_height * island_bias / 0.5) * 0.5;
                let centimetres = quantized * f32::from(config.height_scale_centimetres);
                unity_clamped_i16(centimetres)
            };
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
    let reference = converted_unity_generation_reference();
    for (layer_index, layer) in AUTHORED_RESOURCE_LAYERS.iter().enumerate() {
        let layer_id = StableId::new(layer.kind).expect("authored resource layer ID");
        let reference_layer = &reference.layers[&layer_id];
        let candidates = if resource_layer_matches_reference(layer, reference_layer) {
            reference_layer.candidate_half_units.clone()
        } else {
            generate_candidate_mask(
                300,
                layer.seed,
                layer.noise_scale,
                layer.octaves,
                layer.persistence,
                layer.lacunarity,
                layer.threshold,
                layer.spacing,
                layer.kind == "resource:wood",
            )
        };
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

fn resource_layer_matches_reference(
    layer: &AuthoredResourceLayer,
    reference: &UnityGenerationLayerReference,
) -> bool {
    reference.source_size == 300
        && reference.seed == layer.seed
        && exact_f32(reference.noise_scale, layer.noise_scale)
        && reference.octaves == layer.octaves
        && exact_f32(reference.persistence, layer.persistence)
        && exact_f32(reference.lacunarity, layer.lacunarity)
        && exact_f32(reference.threshold, layer.threshold)
        && reference.spacing == layer.spacing
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
        let reference = converted_unity_generation_reference();
        let candidates = reference
            .layers
            .get(&layer.id)
            .filter(|reference| foliage_layer_matches_reference(layer, reference))
            .map_or_else(
                || {
                    generate_candidate_mask(
                        layer.source_size,
                        layer.seed,
                        layer.noise_scale,
                        layer.octaves,
                        layer.persistence,
                        layer.lacunarity,
                        layer.spawn_threshold,
                        layer.spacing,
                        false,
                    )
                },
                |reference| reference.candidate_half_units.clone(),
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

fn foliage_layer_matches_reference(
    layer: &FoliageLayerDef,
    reference: &UnityGenerationLayerReference,
) -> bool {
    reference.source_size == layer.source_size
        && reference.seed == layer.seed
        && exact_f32(reference.noise_scale, layer.noise_scale)
        && reference.octaves == layer.octaves
        && exact_f32(reference.persistence, layer.persistence)
        && exact_f32(reference.lacunarity, layer.lacunarity)
        && exact_f32(reference.threshold, layer.spawn_threshold)
        && reference.spacing == layer.spacing
}

fn exact_f32(left: f32, right: f32) -> bool {
    left.to_bits() == right.to_bits()
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
    let minimum = f64::from(size) * 0.5;
    let additional = f64::from(size) * 2.0;
    [
        (minimum + random.next_double() * additional) as f32,
        (minimum + random.next_double() * additional) as f32,
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

#[allow(clippy::cast_precision_loss, clippy::too_many_arguments)]
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
            let mut frequency = 1.0;
            let mut noise_height = 0.0;
            for octave_offset in &octave_offsets {
                let sample_x = (f32::from(x) - half_width) / scale * frequency + octave_offset[0];
                let sample_z = (f32::from(z) - half_height) / scale * frequency + octave_offset[1];
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
    let range = maximum - minimum;
    for value in &mut values {
        *value = if range.abs() <= f32::EPSILON {
            0.0
        } else {
            ((*value - minimum) / range).clamp(0.0, 1.0)
        };
    }
    values
}

// Unity's native Mathf.PerlinNoise implementation is not exposed to managed
// code. This is the same fixed-permutation improved-Perlin family, while the
// surrounding octave offsets, f32 sampling, normalization, curves and authored
// settings mirror Assets/Scripts/Utils/Noise.cs exactly.
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
    let xi = (x.floor() as i32 & 255) as usize;
    let zi = (z.floor() as i32 & 255) as usize;
    let xf = x - x.floor();
    let zf = z - z.floor();
    let fade = |value: f32| value * value * value * (value * (value * 6.0 - 15.0) + 10.0);
    let hash = |first: usize, second: usize| {
        let inner = usize::from(PERMUTATION[first & 255]);
        PERMUTATION[(inner + second) & 255]
    };
    let gradient = |hash: u8, x: f32, z: f32| match hash & 7 {
        0 => x + z,
        1 => -x + z,
        2 => x - z,
        3 => -x - z,
        4 => x,
        5 => -x,
        6 => z,
        _ => -z,
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
    (lerp(lower, upper, v) * 0.5 + 0.5).clamp(0.0, 1.0)
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
        // seed. The checked-in Unity generation reference comes from the editor
        // algorithm and authored settings, not from save placements.
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
