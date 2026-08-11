use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::StableId;

pub const CURRENT_CONFIG_SCHEMA: u32 = 4;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameConfig {
    pub schema_version: u32,
    pub window: WindowConfig,
    pub world: WorldGenConfig,
    pub gameplay: GameplayConfig,
    pub twitch: TwitchConfig,
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
    pub starting_town_resources: BTreeMap<StableId, u32>,
    /// Resources omitted from this map are intentionally unbounded.
    pub base_town_resource_capacity: BTreeMap<StableId, u32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TwitchConfig {
    /// Twitch stays opt-in so tests, tools, and offline play never make network requests.
    pub enabled: bool,
    /// Public-client identifier. Tokens and refresh tokens are stored in the OS vault.
    pub client_id: String,
    pub bot_login: String,
    pub channel_login: String,
    pub require_broadcaster_connect: bool,
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
    #[error("enabled Twitch integration requires a public client ID")]
    TwitchClientId,
    #[error("Twitch bot and channel logins must be lowercase ASCII names")]
    TwitchLogin,
    #[error("starting resource {resource} exceeds capacity {capacity}")]
    StartingResourceCapacity { resource: StableId, capacity: u32 },
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
                starting_town_resources: BTreeMap::from([
                    (stable_id("resource:food"), 5_000),
                    (stable_id("resource:gold"), 5_000),
                    (stable_id("resource:ore"), 5_000),
                    (stable_id("resource:recruit"), 0),
                    (stable_id("resource:wood"), 5_000),
                ]),
                base_town_resource_capacity: BTreeMap::from([
                    (stable_id("resource:food"), 15_000),
                    (stable_id("resource:ore"), 15_000),
                    (stable_id("resource:recruit"), 5),
                    (stable_id("resource:wood"), 15_000),
                ]),
            },
            twitch: TwitchConfig {
                enabled: false,
                client_id: String::new(),
                bot_login: "humanbeanbot".to_owned(),
                channel_login: "humanbeangames".to_owned(),
                require_broadcaster_connect: true,
            },
        }
    }
}

fn stable_id(value: &str) -> StableId {
    StableId::new(value).expect("built-in configuration IDs are valid")
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
        for (resource, amount) in &self.gameplay.starting_town_resources {
            if let Some(capacity) = self.gameplay.base_town_resource_capacity.get(resource)
                && amount > capacity
            {
                return Err(ConfigError::StartingResourceCapacity {
                    resource: resource.clone(),
                    capacity: *capacity,
                });
            }
        }
        if self.twitch.enabled && self.twitch.client_id.trim().is_empty() {
            return Err(ConfigError::TwitchClientId);
        }
        if !valid_twitch_login(&self.twitch.bot_login)
            || !valid_twitch_login(&self.twitch.channel_login)
        {
            return Err(ConfigError::TwitchLogin);
        }
        Ok(())
    }
}

fn valid_twitch_login(login: &str) -> bool {
    (3..=25).contains(&login.len())
        && login
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
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

    #[test]
    fn enabled_twitch_requires_public_configuration() {
        let mut config = GameConfig::default();
        config.twitch.enabled = true;
        assert_eq!(config.validate(), Err(ConfigError::TwitchClientId));
        config.twitch.client_id = "public-client-id".to_owned();
        assert!(config.validate().is_ok());
        config.twitch.channel_login = "MixedCase".to_owned();
        assert_eq!(config.validate(), Err(ConfigError::TwitchLogin));
    }
}
