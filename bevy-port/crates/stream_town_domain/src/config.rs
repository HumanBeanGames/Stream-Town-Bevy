use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::StableId;

pub const CURRENT_CONFIG_SCHEMA: u32 = 5;
pub const SHIPPING_SECONDS_PER_DAY: u32 = 3_600;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameConfig {
    pub schema_version: u32,
    pub window: WindowConfig,
    pub world: WorldGenConfig,
    #[serde(default)]
    pub time: TimeCycleConfig,
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

/// Shipping time-of-day settings converted from Unity's `D_TimeSettings` and
/// `D_DayAndNightSettings` assets. Milli-units keep the authored values stable
/// across RON, deterministic tests, and rendering backends.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TimeCycleConfig {
    pub seconds_per_day: u32,
    pub daylight_per_thousand: u16,
    pub transition_seconds: u32,
    pub day_light_intensity_milli: u16,
    pub night_light_intensity_milli: u16,
    pub max_building_emission_milli: u16,
}

impl Default for TimeCycleConfig {
    fn default() -> Self {
        Self {
            seconds_per_day: SHIPPING_SECONDS_PER_DAY,
            daylight_per_thousand: 666,
            transition_seconds: 100,
            day_light_intensity_milli: 10_000,
            night_light_intensity_milli: 5_000,
            max_building_emission_milli: 5_000,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeCycleSample {
    /// Matches Unity's `IsDayTime`: dusk remains day until its transition ends,
    /// while dawn remains night until its transition ends.
    pub is_daytime: bool,
    /// Zero at authored night intensity, one at authored day intensity.
    pub daylight: f32,
}

impl TimeCycleConfig {
    #[must_use]
    pub fn sample(&self, elapsed_seconds: f64) -> TimeCycleSample {
        let cycle = f64::from(self.seconds_per_day.max(1));
        let cycle_f32 =
            std::time::Duration::from_secs(u64::from(self.seconds_per_day.max(1))).as_secs_f32();
        let transition =
            std::time::Duration::from_secs(u64::from(self.transition_seconds)).as_secs_f32();
        let daylight_end = cycle_f32 * f32::from(self.daylight_per_thousand) / 1_000.0;
        let day_end = (daylight_end - transition).max(0.0);
        let night_end = (cycle_f32 - transition).max(daylight_end);
        let phase = std::time::Duration::from_secs_f64(elapsed_seconds.max(0.0).rem_euclid(cycle))
            .as_secs_f32();
        if phase < day_end {
            TimeCycleSample {
                is_daytime: true,
                daylight: 1.0,
            }
        } else if phase < daylight_end {
            let progress = (phase - day_end) / transition.max(f32::EPSILON);
            TimeCycleSample {
                is_daytime: true,
                daylight: 1.0 - ease_in_out_cubic(progress),
            }
        } else if phase < night_end {
            TimeCycleSample {
                is_daytime: false,
                daylight: 0.0,
            }
        } else {
            let progress = (phase - night_end) / transition.max(f32::EPSILON);
            TimeCycleSample {
                is_daytime: false,
                daylight: ease_in_out_cubic(progress),
            }
        }
    }
}

fn ease_in_out_cubic(value: f32) -> f32 {
    if value < 0.5 {
        4.0 * value * value * value
    } else {
        1.0 - (-2.0 * value + 2.0).powi(3) / 2.0
    }
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
    /// Numeric Twitch user IDs authorized for Unity-compatible game-master commands.
    /// This intentionally does not inherit broadcaster or moderator privileges.
    #[serde(default)]
    pub game_master_ids: BTreeSet<String>,
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
    #[error("time cycle settings must define positive day and night periods")]
    TimeCycle,
    #[error("initial agent count must be between 1 and 5000")]
    AgentCount,
    #[error("enabled Twitch integration requires a public client ID")]
    TwitchClientId,
    #[error("Twitch bot and channel logins must be lowercase ASCII names")]
    TwitchLogin,
    #[error("game-master Twitch user IDs must contain only ASCII digits")]
    TwitchGameMasterId,
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
            time: TimeCycleConfig::default(),
            gameplay: GameplayConfig {
                initial_agents: 5,
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
                game_master_ids: BTreeSet::new(),
            },
        }
    }
}

fn stable_id(value: &str) -> StableId {
    StableId::new(value).expect("built-in configuration IDs are valid")
}

impl GameConfig {
    /// Upgrades compatible user-owned configuration before strict validation.
    /// Schema 4 predates authored time-of-day settings and receives the shipping
    /// Unity values through `TimeCycleConfig::default` during deserialization.
    pub fn upgrade(mut self) -> Result<Self, ConfigError> {
        if self.schema_version == 4 {
            self.schema_version = CURRENT_CONFIG_SCHEMA;
        }
        self.validate()?;
        Ok(self)
    }

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
        let day_milliseconds = u64::from(self.time.seconds_per_day)
            .saturating_mul(u64::from(self.time.daylight_per_thousand));
        let night_milliseconds = u64::from(self.time.seconds_per_day).saturating_mul(u64::from(
            1_000_u16.saturating_sub(self.time.daylight_per_thousand),
        ));
        let transition_milliseconds = u64::from(self.time.transition_seconds) * 1_000;
        if self.time.seconds_per_day == 0
            || self.time.daylight_per_thousand == 0
            || self.time.daylight_per_thousand >= 1_000
            || transition_milliseconds >= day_milliseconds
            || transition_milliseconds >= night_milliseconds
            || self.time.day_light_intensity_milli == 0
            || self.time.night_light_intensity_milli > self.time.day_light_intensity_milli
        {
            return Err(ConfigError::TimeCycle);
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
        if self
            .twitch
            .game_master_ids
            .iter()
            .any(|id| id.is_empty() || !id.bytes().all(|byte| byte.is_ascii_digit()))
        {
            return Err(ConfigError::TwitchGameMasterId);
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
    fn shipping_time_cycle_matches_unity_day_night_boundaries() {
        let time = GameConfig::default().time;
        assert!((time.sample(0.0).daylight - 1.0).abs() < f32::EPSILON);
        assert!(time.sample(2_300.0).is_daytime);
        assert!(!time.sample(2_400.0).is_daytime);
        assert!(time.sample(3_000.0).daylight.abs() < f32::EPSILON);
        let dawn = time.sample(3_550.0);
        assert!(!dawn.is_daytime);
        assert!((dawn.daylight - 0.5).abs() < f32::EPSILON);
        let next_day = time.sample(3_600.0);
        assert!(next_day.is_daytime);
        assert!((next_day.daylight - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn schema_four_user_config_upgrades_with_shipping_time_defaults() {
        #[derive(Serialize)]
        struct SchemaFourConfig<'a> {
            schema_version: u32,
            window: &'a WindowConfig,
            world: &'a WorldGenConfig,
            gameplay: &'a GameplayConfig,
            twitch: &'a TwitchConfig,
        }

        let config = GameConfig::default();
        let encoded = ron::to_string(&SchemaFourConfig {
            schema_version: 4,
            window: &config.window,
            world: &config.world,
            gameplay: &config.gameplay,
            twitch: &config.twitch,
        })
        .unwrap();
        let upgraded = ron::from_str::<GameConfig>(&encoded)
            .unwrap()
            .upgrade()
            .unwrap();
        assert_eq!(upgraded.schema_version, CURRENT_CONFIG_SCHEMA);
        assert_eq!(upgraded.time, TimeCycleConfig::default());
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
        config.twitch.channel_login = "humanbeangames".to_owned();
        config
            .twitch
            .game_master_ids
            .insert("not-a-user-id".to_owned());
        assert_eq!(config.validate(), Err(ConfigError::TwitchGameMasterId));
    }
}
