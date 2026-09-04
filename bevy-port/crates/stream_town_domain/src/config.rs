use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::StableId;

pub const CURRENT_CONFIG_SCHEMA: u32 = 8;
pub const SHIPPING_FISH_GOD_REWARD_ID: &str = "5a760033-50b5-4e47-911b-d63993d2860c";
pub const SHIPPING_SECONDS_PER_DAY: u32 = 3_600;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameConfig {
    pub schema_version: u32,
    pub window: WindowConfig,
    pub world: WorldGenConfig,
    #[serde(default)]
    pub time: TimeCycleConfig,
    #[serde(default)]
    pub terrain: TerrainAppearanceConfig,
    #[serde(default)]
    pub music: AdaptiveMusicConfig,
    pub gameplay: GameplayConfig,
    pub twitch: TwitchConfig,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct SeasonalTerrainPalette {
    /// Linear multiplier applied to the authored sand/grass terrain shader.
    pub base_color: [f32; 4],
    /// Constructed-path tint; alpha is the maximum blend over the terrain.
    pub path_tint: [f32; 4],
    /// Traversal-wear tint; alpha is the maximum blend over the authored terrain.
    pub traversal_tint: [f32; 4],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TerrainAppearanceConfig {
    pub spring: SeasonalTerrainPalette,
    pub summer: SeasonalTerrainPalette,
    pub autumn: SeasonalTerrainPalette,
    pub winter: SeasonalTerrainPalette,
    /// Sustained citizen cell crossings per minute at which wear first becomes visible.
    pub traversal_fade_start_per_minute: f32,
    /// Sustained citizen cell crossings per minute at which the tint reaches full strength.
    pub traversal_full_tint_per_minute: f32,
    pub traversal_half_life_seconds: f32,
    pub traversal_decay_pause_seconds: f32,
    pub traversal_prune_score: f32,
}

impl Default for TerrainAppearanceConfig {
    fn default() -> Self {
        let path_tint = [0.42, 0.44, 0.46, 0.92];
        let traversal_tint = [0.30, 0.245, 0.14, 0.50];
        Self {
            spring: SeasonalTerrainPalette {
                base_color: [0.86, 1.14, 0.84, 1.0],
                path_tint,
                traversal_tint,
            },
            summer: SeasonalTerrainPalette {
                base_color: [1.0, 0.96, 0.78, 1.0],
                path_tint,
                traversal_tint,
            },
            autumn: SeasonalTerrainPalette {
                base_color: [1.0, 0.64, 0.30, 1.0],
                path_tint,
                traversal_tint,
            },
            winter: SeasonalTerrainPalette {
                base_color: [0.76, 0.88, 1.0, 1.0],
                path_tint: [0.50, 0.53, 0.56, 0.92],
                traversal_tint: [0.34, 0.32, 0.27, 0.45],
            },
            traversal_fade_start_per_minute: 5.0,
            traversal_full_tint_per_minute: 50.0,
            traversal_half_life_seconds: 120.0 * 60.0,
            traversal_decay_pause_seconds: 5.0,
            traversal_prune_score: 0.01,
        }
    }
}

/// Authorable mapping from live town state into the Bevy Tidal composition.
/// The score template can use both its existing generated-voice placeholders
/// and `${intensity}`, `${season}`, `${time_of_day}`, `${population}`,
/// `${building_count}`, or `${energy}`.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AdaptiveMusicConfig {
    pub intensity_smoothing_seconds: f32,
    pub maximum_energy: f32,
    pub cycles_per_minute_base: f32,
    pub cycles_per_minute_per_energy: f32,
    pub intensity_weight: f32,
    pub season_weight: f32,
    pub time_of_day_weight: f32,
    pub population_weight: f32,
    pub building_count_weight: f32,
    pub score_template: String,
}

impl Default for AdaptiveMusicConfig {
    fn default() -> Self {
        Self {
            intensity_smoothing_seconds: 5.0,
            maximum_energy: 12.0,
            cycles_per_minute_base: 75.0,
            cycles_per_minute_per_energy: 5.0,
            intensity_weight: 1.0,
            season_weight: 0.0,
            time_of_day_weight: 0.0,
            population_weight: 0.0,
            building_count_weight: 0.0,
            score_template: concat!(
                "stack\n",
                "  [ sound \"${kick}\" # lpf ${kick_brightness} # gain ${kick_gain}\n",
                "  , struct \"${root_pattern}\" $ ${roots}\n",
                "      # sound \"superpiano\"\n",
                "      # legato 0.9\n",
                "      # attack ${melody_attack}\n",
                "      # decay 0.02\n",
                "      # release 0.035\n",
                "      # lpf ${melody_brightness}\n",
                "      # gain ${melody_gain}\n",
                "      # room 0.08\n",
                "      # roomsize 1\n",
                "  , struct \"${chord_pattern}\" $ ${chords}\n",
                "      # sound \"superpiano\"\n",
                "      # attack ${chord_attack}\n",
                "      # release 0.4\n",
                "      # lpf ${chord_brightness}\n",
                "      # gain ${chord_gain}\n",
                "      # room 0.20\n",
                "      # roomsize 2\n",
                "  , sound \"${hats}\" # lpf ${hat_brightness} # gain ${hat_gain}\n",
                "  ]",
            )
            .to_owned(),
        }
    }
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
    /// Legacy schema field retained for configuration compatibility. Chat
    /// commands are enabled automatically after the authorized bot connects.
    pub require_broadcaster_connect: bool,
    /// Numeric Twitch user IDs authorized for Unity-compatible game-master commands.
    /// This intentionally does not inherit broadcaster or moderator privileges.
    #[serde(default)]
    pub game_master_ids: BTreeSet<String>,
    /// Optional channel-point reward UUID that dispatches the Fish God praise action.
    /// The ordinary `!praise` command remains available when this is `None`.
    #[serde(default = "shipping_fish_god_reward_id")]
    pub fish_god_reward_id: Option<String>,
    /// Direct, in-process Twitch broadcast settings. The stream key is fetched
    /// at runtime with a separately authorized broadcaster token and is never
    /// serialized into this configuration.
    #[serde(default)]
    pub broadcast: BroadcastConfig,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BroadcastEncoderPreference {
    Auto,
    Nvidia,
    Intel,
    Amd,
    MediaFoundation,
    OpenH264,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BroadcastRenderMode {
    StreamOnly,
    Headed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BroadcastConfig {
    /// Broadcast remains independently opt-in from chat connectivity.
    pub enabled: bool,
    /// Renders directly into an offscreen broadcast target and replaces the
    /// local game view with a lightweight operator dashboard while live.
    #[serde(default = "broadcast_render_mode_default")]
    pub render_mode: BroadcastRenderMode,
    /// Legacy schema field retained for configuration compatibility. The game
    /// always starts offline and requires an explicit operator Go Live action.
    pub start_on_launch: bool,
    pub width: u16,
    pub height: u16,
    pub frames_per_second: u8,
    pub video_bitrate_kbps: u32,
    pub audio_bitrate_kbps: u16,
    pub encoder: BroadcastEncoderPreference,
    /// Optional Twitch ingest name substring. Empty selects Twitch's first
    /// recommended endpoint.
    pub ingest: String,
    /// Appends Twitch's bandwidth-test flag. This never goes live, but still
    /// consumes network bandwidth and therefore requires an explicit setting.
    pub bandwidth_test: bool,
}

impl Default for BroadcastConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            render_mode: BroadcastRenderMode::StreamOnly,
            start_on_launch: false,
            width: 1_280,
            height: 720,
            frames_per_second: 30,
            video_bitrate_kbps: 6_000,
            audio_bitrate_kbps: 160,
            encoder: BroadcastEncoderPreference::Auto,
            ingest: String::new(),
            bandwidth_test: false,
        }
    }
}

const fn broadcast_render_mode_default() -> BroadcastRenderMode {
    BroadcastRenderMode::StreamOnly
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
    #[error("terrain appearance and traversal-wear settings are invalid")]
    TerrainAppearance,
    #[error("adaptive music settings are invalid")]
    AdaptiveMusic,
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
    #[error("the Twitch channel-point reward ID must be a UUID or omitted")]
    TwitchRewardId,
    #[error("direct Twitch broadcast dimensions must be even and between 320x180 and 1920x1080")]
    TwitchBroadcastDimensions,
    #[error("direct Twitch broadcast frame rate must be 30 or 60 FPS")]
    TwitchBroadcastFrameRate,
    #[error("direct Twitch broadcast video bitrate must be between 500 and 6000 Kbps")]
    TwitchBroadcastVideoBitrate,
    #[error("direct Twitch broadcast audio bitrate must be between 64 and 160 Kbps")]
    TwitchBroadcastAudioBitrate,
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
                // Recorded shipping-town seed. The Unity save is used only by
                // offline parity tests; runtime generation starts from this seed.
                seed: 1_580_290_387,
                // D_TerrainGenSettings.Size (50) is multiplied by the shipping
                // D_WorldGenScaleSettings x/y scale (4), and each authored
                // voxel cell is two Unity units wide.
                width: 200,
                height: 200,
                cell_size: 2.0,
                height_scale_centimetres: 100,
                water_level_centimetres: 5,
                resource_density_per_thousand: 38,
            },
            time: TimeCycleConfig::default(),
            terrain: TerrainAppearanceConfig::default(),
            music: AdaptiveMusicConfig::default(),
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
                require_broadcaster_connect: false,
                game_master_ids: BTreeSet::new(),
                fish_god_reward_id: shipping_fish_god_reward_id(),
                broadcast: BroadcastConfig::default(),
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
    /// Schema 5 used the Bevy prototype's oversized 12-metre cells; untouched
    /// copies are migrated to the spatial scale authored by the Unity project.
    /// Schema 7 adds a configurable Channel Points reward ID; schema-6 files
    /// receive the shipping Unity reward ID through the serde default. Schema
    /// 8 adds direct broadcasting, disabled by default for existing installs.
    pub fn upgrade(mut self) -> Result<Self, ConfigError> {
        if self.schema_version == 4 {
            self.schema_version = 5;
        }
        if self.schema_version == 5 {
            if self.world.width == 64
                && self.world.height == 64
                && (self.world.cell_size - 12.0).abs() <= f32::EPSILON
                && self.world.height_scale_centimetres == 800
                && self.world.water_level_centimetres == -180
                && self.world.resource_density_per_thousand == 38
            {
                self.world.width = 200;
                self.world.height = 200;
                self.world.cell_size = 2.0;
                self.world.height_scale_centimetres = 100;
                self.world.water_level_centimetres = 5;
            }
            self.schema_version = CURRENT_CONFIG_SCHEMA;
        }
        if self.schema_version == 6 {
            self.schema_version = 7;
        }
        if self.schema_version == 7 {
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
        let terrain_palettes = [
            self.terrain.spring,
            self.terrain.summer,
            self.terrain.autumn,
            self.terrain.winter,
        ];
        if terrain_palettes.iter().any(|palette| {
            palette
                .base_color
                .iter()
                .chain(&palette.path_tint)
                .chain(&palette.traversal_tint)
                .any(|component| !component.is_finite() || *component < 0.0 || *component > 4.0)
        }) || !self.terrain.traversal_fade_start_per_minute.is_finite()
            || !self.terrain.traversal_full_tint_per_minute.is_finite()
            || self.terrain.traversal_fade_start_per_minute < 0.0
            || self.terrain.traversal_full_tint_per_minute
                <= self.terrain.traversal_fade_start_per_minute
            || self.terrain.traversal_full_tint_per_minute > 10_000.0
            || !self.terrain.traversal_half_life_seconds.is_finite()
            || self.terrain.traversal_half_life_seconds <= 0.0
            || !self.terrain.traversal_decay_pause_seconds.is_finite()
            || self.terrain.traversal_decay_pause_seconds < 0.0
            || !self.terrain.traversal_prune_score.is_finite()
            || self.terrain.traversal_prune_score < 0.0
        {
            return Err(ConfigError::TerrainAppearance);
        }
        let music = &self.music;
        if !music.intensity_smoothing_seconds.is_finite()
            || music.intensity_smoothing_seconds <= 0.0
            || !music.maximum_energy.is_finite()
            || music.maximum_energy <= 0.0
            || !music.cycles_per_minute_base.is_finite()
            || music.cycles_per_minute_base <= 0.0
            || !music.cycles_per_minute_per_energy.is_finite()
            || [
                music.intensity_weight,
                music.season_weight,
                music.time_of_day_weight,
                music.population_weight,
                music.building_count_weight,
            ]
            .iter()
            .any(|weight| !weight.is_finite())
            || music.score_template.trim().is_empty()
        {
            return Err(ConfigError::AdaptiveMusic);
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
        if (self.twitch.enabled || self.twitch.broadcast.enabled)
            && self.twitch.client_id.trim().is_empty()
        {
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
        if self
            .twitch
            .fish_god_reward_id
            .as_deref()
            .is_some_and(|id| !valid_uuid(id))
        {
            return Err(ConfigError::TwitchRewardId);
        }
        let broadcast = &self.twitch.broadcast;
        if broadcast.width < 320
            || broadcast.width > 1_920
            || broadcast.height < 180
            || broadcast.height > 1_080
            || !broadcast.width.is_multiple_of(2)
            || !broadcast.height.is_multiple_of(2)
        {
            return Err(ConfigError::TwitchBroadcastDimensions);
        }
        if !matches!(broadcast.frames_per_second, 30 | 60) {
            return Err(ConfigError::TwitchBroadcastFrameRate);
        }
        if !(500..=6_000).contains(&broadcast.video_bitrate_kbps) {
            return Err(ConfigError::TwitchBroadcastVideoBitrate);
        }
        if !(64..=160).contains(&broadcast.audio_bitrate_kbps) {
            return Err(ConfigError::TwitchBroadcastAudioBitrate);
        }
        Ok(())
    }
}

// Serde's field-default callback must return the field's `Option<String>` type.
#[allow(clippy::unnecessary_wraps)]
fn shipping_fish_god_reward_id() -> Option<String> {
    Some(SHIPPING_FISH_GOD_REWARD_ID.to_owned())
}

fn valid_uuid(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => byte == b'-',
            _ => byte.is_ascii_hexdigit(),
        })
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
        assert!((config.terrain.traversal_half_life_seconds - 7_200.0).abs() < f32::EPSILON);
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
    fn schema_five_prototype_world_scale_upgrades_to_unity_scale() {
        let mut config = GameConfig {
            schema_version: 5,
            ..GameConfig::default()
        };
        config.world.width = 64;
        config.world.height = 64;
        config.world.cell_size = 12.0;
        config.world.height_scale_centimetres = 800;
        config.world.water_level_centimetres = -180;
        let upgraded = config.upgrade().unwrap();
        assert_eq!(upgraded.schema_version, CURRENT_CONFIG_SCHEMA);
        assert_eq!(upgraded.world.width, 200);
        assert_eq!(upgraded.world.height, 200);
        assert!((upgraded.world.cell_size - 2.0).abs() <= f32::EPSILON);
        assert_eq!(upgraded.world.height_scale_centimetres, 100);
        assert_eq!(upgraded.world.water_level_centimetres, 5);
    }

    #[test]
    fn schema_five_custom_world_scale_is_preserved() {
        let mut config = GameConfig {
            schema_version: 5,
            ..GameConfig::default()
        };
        config.world.width = 96;
        let upgraded = config.upgrade().unwrap();
        assert_eq!(upgraded.schema_version, CURRENT_CONFIG_SCHEMA);
        assert_eq!(upgraded.world.width, 96);
        assert!((upgraded.world.cell_size - 2.0).abs() <= f32::EPSILON);
    }

    #[test]
    fn schema_six_config_upgrades_with_shipping_reward_id() {
        #[derive(Serialize)]
        struct SchemaSixTwitch<'a> {
            enabled: bool,
            client_id: &'a str,
            bot_login: &'a str,
            channel_login: &'a str,
            require_broadcaster_connect: bool,
            game_master_ids: &'a BTreeSet<String>,
        }

        #[derive(Serialize)]
        struct SchemaSixConfig<'a> {
            schema_version: u32,
            window: &'a WindowConfig,
            world: &'a WorldGenConfig,
            time: &'a TimeCycleConfig,
            gameplay: &'a GameplayConfig,
            twitch: SchemaSixTwitch<'a>,
        }

        let config = GameConfig::default();
        let encoded = ron::to_string(&SchemaSixConfig {
            schema_version: 6,
            window: &config.window,
            world: &config.world,
            time: &config.time,
            gameplay: &config.gameplay,
            twitch: SchemaSixTwitch {
                enabled: config.twitch.enabled,
                client_id: &config.twitch.client_id,
                bot_login: &config.twitch.bot_login,
                channel_login: &config.twitch.channel_login,
                require_broadcaster_connect: config.twitch.require_broadcaster_connect,
                game_master_ids: &config.twitch.game_master_ids,
            },
        })
        .unwrap();
        let upgraded = ron::from_str::<GameConfig>(&encoded)
            .unwrap()
            .upgrade()
            .unwrap();
        assert_eq!(upgraded.schema_version, CURRENT_CONFIG_SCHEMA);
        assert_eq!(
            upgraded.twitch.fish_god_reward_id.as_deref(),
            Some(SHIPPING_FISH_GOD_REWARD_ID)
        );
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
        config.twitch.game_master_ids.clear();
        config.twitch.fish_god_reward_id = Some("not-a-reward-uuid".to_owned());
        assert_eq!(config.validate(), Err(ConfigError::TwitchRewardId));
        config.twitch.fish_god_reward_id = None;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn schema_seven_config_gains_disabled_direct_broadcast_defaults() {
        #[derive(Serialize)]
        struct SchemaSevenTwitch<'a> {
            enabled: bool,
            client_id: &'a str,
            bot_login: &'a str,
            channel_login: &'a str,
            require_broadcaster_connect: bool,
            game_master_ids: &'a BTreeSet<String>,
            fish_god_reward_id: &'a Option<String>,
        }

        #[derive(Serialize)]
        struct SchemaSevenConfig<'a> {
            schema_version: u32,
            window: &'a WindowConfig,
            world: &'a WorldGenConfig,
            time: &'a TimeCycleConfig,
            gameplay: &'a GameplayConfig,
            twitch: SchemaSevenTwitch<'a>,
        }

        let config = GameConfig::default();
        let encoded = ron::to_string(&SchemaSevenConfig {
            schema_version: 7,
            window: &config.window,
            world: &config.world,
            time: &config.time,
            gameplay: &config.gameplay,
            twitch: SchemaSevenTwitch {
                enabled: config.twitch.enabled,
                client_id: &config.twitch.client_id,
                bot_login: &config.twitch.bot_login,
                channel_login: &config.twitch.channel_login,
                require_broadcaster_connect: config.twitch.require_broadcaster_connect,
                game_master_ids: &config.twitch.game_master_ids,
                fish_god_reward_id: &config.twitch.fish_god_reward_id,
            },
        })
        .unwrap();
        let upgraded = ron::from_str::<GameConfig>(&encoded)
            .unwrap()
            .upgrade()
            .unwrap();
        assert_eq!(upgraded.schema_version, CURRENT_CONFIG_SCHEMA);
        assert_eq!(upgraded.twitch.broadcast, BroadcastConfig::default());
    }

    #[test]
    fn direct_broadcast_settings_are_strictly_validated() {
        let mut config = GameConfig::default();
        config.twitch.broadcast.enabled = true;
        assert_eq!(config.validate(), Err(ConfigError::TwitchClientId));
        config.twitch.client_id = "public-client-id".to_owned();
        assert!(config.validate().is_ok());
        config.twitch.broadcast.width = 1_279;
        assert_eq!(
            config.validate(),
            Err(ConfigError::TwitchBroadcastDimensions)
        );
        config.twitch.broadcast.width = 1_280;
        config.twitch.broadcast.frames_per_second = 24;
        assert_eq!(
            config.validate(),
            Err(ConfigError::TwitchBroadcastFrameRate)
        );
    }
}
