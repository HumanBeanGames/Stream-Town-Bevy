use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{FoliageHabitat, FoliageLayerDef, GridPos, NavGrid, StableId, WorldGenConfig};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GeneratedResource {
    pub id: StableId,
    pub kind: StableId,
    pub target_kind: StableId,
    pub position: GridPos,
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
    const GENERATOR_VERSION: u32 = 3;
    let cell_count = usize::from(config.width) * usize::from(config.height);
    let mut heights = Vec::with_capacity(cell_count);
    let mut blocked = Vec::with_capacity(cell_count);
    let mut resources = Vec::new();
    let centre_x = i64::from(config.width.saturating_sub(1));
    let centre_z = i64::from(config.height.saturating_sub(1));
    let max_distance = centre_x * centre_x + centre_z * centre_z;

    for z in 0..config.height {
        for x in 0..config.width {
            let position = GridPos { x, z };
            let random = cell_hash(config.seed, x, z);
            let dx = i64::from(x) * 2 - centre_x;
            let dz = i64::from(z) * 2 - centre_z;
            let distance = dx * dx + dz * dz;
            let island =
                i32::try_from((max_distance - distance).max(0) * 1_000 / max_distance.max(1))
                    .expect("normalized island height is between 0 and 1000");
            let noise = i32::try_from((random >> 16) & 0x3ff).expect("ten-bit value") - 512;
            let scaled = (island - 420) * i32::from(config.height_scale_centimetres) / 580
                + noise * i32::from(config.height_scale_centimetres) / 2_048;
            let height = i16::try_from(scaled.clamp(i32::from(i16::MIN), i32::from(i16::MAX)))
                .expect("clamped height");
            let is_blocked = height <= config.water_level_centimetres;
            heights.push(height);
            blocked.push(is_blocked);

            let resource_roll = u16::try_from(random % 1_000).expect("modulo 1000");
            if !is_blocked && resource_roll < config.resource_density_per_thousand {
                let (kind, target_kind) = match (random >> 10) % 3 {
                    0 => ("resource:wood", "target:tree"),
                    1 => ("resource:ore", "target:ore"),
                    _ => ("resource:food", "target:bush"),
                };
                resources.push(GeneratedResource {
                    id: StableId::new(format!("resource:{x}:{z}")).expect("generated stable ID"),
                    kind: StableId::new(kind).expect("static stable ID"),
                    target_kind: StableId::new(target_kind).expect("static stable ID"),
                    position,
                    amount: 50 + u32::try_from((random >> 24) % 151).expect("bounded amount"),
                });
            }
        }
    }

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
                amount: 50 + u32::try_from((random >> 24) % 151).expect("bounded amount"),
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
    let resources: Vec<_> = world
        .resources
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
    hash_world(
        world.seed,
        2,
        &legacy_resource_navigation(world),
        &world.resources,
    )
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
    let mut occupied: std::collections::BTreeSet<_> =
        resources.iter().map(|resource| resource.position).collect();
    for (layer_index, layer) in layers.iter().enumerate() {
        for z in 0..config.height {
            for x in 0..config.width {
                let position = GridPos { x, z };
                if occupied.contains(&position)
                    || x % layer.spacing != 0
                    || z % layer.spacing != 0
                    || (layer.habitat == FoliageHabitat::Land) != navigation.is_walkable(position)
                {
                    continue;
                }
                let random = foliage_hash(config.seed, layer.seed, x, z);
                // Source thresholds are dense because Unity samples one-unit space.
                // The Bevy grid is twelve units, so preserve the pattern while
                // applying an explicit draw-budget density reduction.
                if foliage_noise(config, layer, position) < f64::from(layer.spawn_threshold)
                    || !(random >> 20).is_multiple_of(4)
                {
                    continue;
                }
                let variant = u16::try_from((random >> 24) % layer.variants.len() as u64)
                    .expect("foliage variant count fits u16");
                let offset_milli_cells = [
                    i16::try_from((random >> 8) % 801).expect("bounded foliage offset") - 400,
                    i16::try_from((random >> 16) % 801).expect("bounded foliage offset") - 400,
                ];
                let yaw_milliradians =
                    u16::try_from((random >> 32) % 6_284).expect("bounded foliage yaw");
                let scale_milli =
                    u16::try_from(850 + ((random >> 48) % 301)).expect("bounded foliage scale");
                foliage.push(GeneratedFoliage {
                    id: StableId::new(format!("foliage:{layer_index}:{x}:{z}"))
                        .expect("generated stable foliage ID"),
                    layer: layer.id.clone(),
                    habitat: layer.habitat,
                    position,
                    offset_milli_cells,
                    variant,
                    yaw_milliradians,
                    scale_milli,
                });
                occupied.insert(position);
            }
        }
    }
    foliage
}

fn foliage_noise(config: &WorldGenConfig, layer: &FoliageLayerDef, position: GridPos) -> f64 {
    let source_size = f64::from(layer.source_size);
    let source_x = f64::from(position.x) * source_size / f64::from(config.width)
        - source_size * 0.5
        + f64::from(layer.offset[0]);
    let source_z = f64::from(position.z) * source_size / f64::from(config.height)
        - source_size * 0.5
        + f64::from(layer.offset[1]);
    let mut amplitude = 1.0_f64;
    let mut frequency = 1.0_f64;
    let mut total = 0.0_f64;
    let mut total_amplitude = 0.0_f64;
    for octave in 0..layer.octaves {
        let octave_seed = config.seed
            ^ u64::from(layer.seed.cast_unsigned()).rotate_left(17)
            ^ u64::from(octave).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        total += value_noise(
            octave_seed,
            source_x / f64::from(layer.noise_scale) * frequency,
            source_z / f64::from(layer.noise_scale) * frequency,
        ) * amplitude;
        total_amplitude += amplitude;
        amplitude *= f64::from(layer.persistence);
        frequency *= f64::from(layer.lacunarity);
    }
    total / total_amplitude.max(f64::EPSILON)
}

fn value_noise(seed: u64, x: f64, z: f64) -> f64 {
    let x0 = x.floor();
    let z0 = z.floor();
    let tx = smooth_noise_step(x - x0);
    let tz = smooth_noise_step(z - z0);
    let lower = lerp(
        lattice_noise(seed, x0, z0),
        lattice_noise(seed, x0 + 1.0, z0),
        tx,
    );
    let upper = lerp(
        lattice_noise(seed, x0, z0 + 1.0),
        lattice_noise(seed, x0 + 1.0, z0 + 1.0),
        tx,
    );
    lerp(lower, upper, tz)
}

fn lattice_noise(seed: u64, x: f64, z: f64) -> f64 {
    let mut value = seed
        ^ x.to_bits().wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ z.to_bits().wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^= value >> 31;
    let sample = u32::try_from(value >> 32).expect("upper hash bits fit u32");
    f64::from(sample) / f64::from(u32::MAX)
}

fn smooth_noise_step(value: f64) -> f64 {
    value * value * (3.0 - 2.0 * value)
}

fn lerp(start: f64, end: f64, amount: f64) -> f64 {
    start + (end - start) * amount
}

fn foliage_hash(seed: u64, layer_seed: i32, x: u16, z: u16) -> u64 {
    cell_hash(
        seed ^ u64::from(layer_seed.cast_unsigned()).rotate_left(17),
        x,
        z,
    )
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
        assert_eq!(world.generator_version, 3);
        assert_ne!(legacy_v1_world_hash(&world), world.deterministic_hash);
        assert_ne!(legacy_v2_world_hash(&world), world.deterministic_hash);
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
        assert!(
            first
                .foliage
                .iter()
                .any(|foliage| foliage.habitat == FoliageHabitat::Underwater)
        );
        let resources: std::collections::BTreeSet<_> = first
            .resources
            .iter()
            .map(|resource| resource.position)
            .collect();
        let positions: std::collections::BTreeSet<_> = first
            .foliage
            .iter()
            .map(|foliage| foliage.position)
            .collect();
        assert_eq!(positions.len(), first.foliage.len());
        for foliage in &first.foliage {
            assert!(!resources.contains(&foliage.position));
            assert_eq!(
                first.navigation.is_walkable(foliage.position),
                foliage.habitat == FoliageHabitat::Land
            );
        }
        content.foliage[0].noise_scale *= 2.0;
        let altered = generate_world_with_content(&config, &content);
        assert_ne!(first.foliage, altered.foliage);
        content.foliage.clear();
        let without_foliage = generate_world_with_content(&config, &content);
        assert!(without_foliage.foliage.is_empty());
        assert_eq!(first.deterministic_hash, without_foliage.deterministic_hash);
    }
}
