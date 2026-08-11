use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const CURRENT_CONFIG_SCHEMA: u32 = 1;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameConfig {
    pub schema_version: u32,
    pub window: WindowConfig,
    pub world: WorldGenConfig,
    pub gameplay: GameplayConfig,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WorldGenConfig {
    pub seed: u64,
    pub width: u16,
    pub height: u16,
    pub cell_size: f32,
    pub height_scale_centimetres: i16,
    pub water_level_centimetres: i16,
    pub resource_density_per_thousand: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameplayConfig {
    pub initial_agents: u16,
    pub agent_speed_cells_per_second: f32,
    pub repath_interval_seconds: f32,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ConfigError {
    #[error("unsupported config schema {0}")]
    Schema(u32),
    #[error("window dimensions must be non-zero")]
    WindowDimensions,
    #[error("world dimensions must be between 8 and 512 cells")]
    WorldDimensions,
    #[error("world cell size must be positive")]
    CellSize,
    #[error("resource density must be at most 1000")]
    ResourceDensity,
    #[error("initial agent count must be between 1 and 5000")]
    AgentCount,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_CONFIG_SCHEMA,
            window: WindowConfig {
                title: "Stream Town — Bevy Migration".to_owned(),
                width: 1_920,
                height: 1_080,
            },
            world: WorldGenConfig {
                seed: 0x5EED_2026,
                width: 64,
                height: 64,
                cell_size: 12.0,
                height_scale_centimetres: 800,
                water_level_centimetres: -180,
                resource_density_per_thousand: 38,
            },
            gameplay: GameplayConfig {
                initial_agents: 300,
                agent_speed_cells_per_second: 4.0,
                repath_interval_seconds: 1.0,
            },
        }
    }
}

impl GameConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.schema_version != CURRENT_CONFIG_SCHEMA {
            return Err(ConfigError::Schema(self.schema_version));
        }
        if self.window.width == 0 || self.window.height == 0 {
            return Err(ConfigError::WindowDimensions);
        }
        if !(8..=512).contains(&self.world.width) || !(8..=512).contains(&self.world.height) {
            return Err(ConfigError::WorldDimensions);
        }
        if !self.world.cell_size.is_finite() || self.world.cell_size <= 0.0 {
            return Err(ConfigError::CellSize);
        }
        if self.world.resource_density_per_thousand > 1_000 {
            return Err(ConfigError::ResourceDensity);
        }
        if !(1..=5_000).contains(&self.gameplay.initial_agents) {
            return Err(ConfigError::AgentCount);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid_and_round_trips_ron() {
        let config = GameConfig::default();
        config.validate().unwrap();
        let encoded = ron::to_string(&config).unwrap();
        assert_eq!(ron::from_str::<GameConfig>(&encoded).unwrap(), config);
    }
}
