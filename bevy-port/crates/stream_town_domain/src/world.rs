use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{GridPos, NavGrid, StableId, WorldGenConfig};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GeneratedResource {
    pub id: StableId,
    pub kind: StableId,
    pub position: GridPos,
    pub amount: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GeneratedWorld {
    pub seed: u64,
    pub generator_version: u32,
    pub navigation: NavGrid,
    pub resources: Vec<GeneratedResource>,
    pub deterministic_hash: String,
}

#[must_use]
pub fn generate_world(config: &WorldGenConfig) -> GeneratedWorld {
    const GENERATOR_VERSION: u32 = 1;
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
                let kind = match (random >> 10) % 3 {
                    0 => "resource:wood",
                    1 => "resource:ore",
                    _ => "resource:food",
                };
                resources.push(GeneratedResource {
                    id: StableId::new(format!("resource:{x}:{z}")).expect("generated stable ID"),
                    kind: StableId::new(kind).expect("static stable ID"),
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

    let navigation = NavGrid::new(config.width, config.height, blocked, heights)
        .expect("validated world configuration produces a valid grid");
    let deterministic_hash = hash_world(config.seed, GENERATOR_VERSION, &navigation, &resources);
    GeneratedWorld {
        seed: config.seed,
        generator_version: GENERATOR_VERSION,
        navigation,
        resources,
        deterministic_hash,
    }
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
        hasher.update(resource.position.x.to_le_bytes());
        hasher.update(resource.position.z.to_le_bytes());
        hasher.update(resource.amount.to_le_bytes());
    }
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use crate::GameConfig;

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
}
