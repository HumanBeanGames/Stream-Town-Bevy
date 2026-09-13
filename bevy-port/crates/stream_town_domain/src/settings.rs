use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const CURRENT_PLAYER_SETTINGS_SCHEMA: u32 = 5;
const LEGACY_BEVY_EXPOSURE_OFFSET_EV: f32 = 0.5;
const PREVIOUS_INGAME_BRIGHTNESS_OFFSET_EV: f32 = 1.0;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum DisplayMode {
    Windowed,
    Borderless,
    #[default]
    Fullscreen,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum PostProcessAntiAliasing {
    None,
    Fxaa,
    #[default]
    Smaa,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum NameDisplayMode {
    #[default]
    None,
    #[serde(alias = "ModeratorsAndSubscribers")]
    StaffAndSubscribers,
    AllPlayers,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum BuildingHealthDisplayMode {
    None,
    #[default]
    DamagedOnly,
    Always,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum TimelapseInterval {
    Off,
    TenMinutes,
    #[default]
    OneHour,
    ThreeHours,
    SixHours,
    TwelveHours,
    TwentyFourHours,
}

impl TimelapseInterval {
    #[must_use]
    pub const fn seconds(self) -> Option<u32> {
        match self {
            Self::Off => None,
            Self::TenMinutes => Some(10 * 60),
            Self::OneHour => Some(60 * 60),
            Self::ThreeHours => Some(3 * 60 * 60),
            Self::SixHours => Some(6 * 60 * 60),
            Self::TwelveHours => Some(12 * 60 * 60),
            Self::TwentyFourHours => Some(24 * 60 * 60),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TimelapseSettings {
    pub interval: TimelapseInterval,
    pub dynamic: bool,
}

impl Default for TimelapseSettings {
    fn default() -> Self {
        Self {
            interval: TimelapseInterval::OneHour,
            dynamic: true,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlayerSettings {
    pub schema_version: u32,
    pub video: VideoSettings,
    pub audio: AudioMixSettings,
    pub camera: CameraSettings,
    pub interface: InterfaceSettings,
    pub autosave_minutes: u16,
    #[serde(default)]
    pub timelapse: TimelapseSettings,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct VideoSettings {
    pub display_mode: DisplayMode,
    pub width: u32,
    pub height: u32,
    pub shadows_enabled: bool,
    pub shadow_map_resolution: u16,
    pub ambient_occlusion: bool,
    pub vsync: bool,
    /// `None` preserves Unity's unlimited option.
    pub fps_limit: Option<u16>,
    pub brightness_ev: f32,
    pub gamma: f32,
    pub msaa_samples: u8,
    pub post_process_aa: PostProcessAntiAliasing,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AudioMixSettings {
    pub master: f32,
    pub music: f32,
    pub sound_effects: f32,
    pub ambience: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CameraSettings {
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub keyboard_pan_sensitivity: f32,
    pub edge_scroll_sensitivity: f32,
    pub field_of_view_degrees: u16,
    pub edge_scrolling: bool,
    pub keyboard_movement: bool,
    pub mouse_controls: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InterfaceSettings {
    pub display_names: NameDisplayMode,
    pub display_building_health: BuildingHealthDisplayMode,
    /// Global logical-pixel scale for every shipping Bevy UI surface.
    #[serde(default = "default_ui_scale_percent")]
    pub ui_scale_percent: u16,
    /// Adds persistent text separation and stronger interactive outlines.
    #[serde(default)]
    pub high_contrast: bool,
    /// Suppresses non-essential looping presentation motion.
    #[serde(default)]
    pub reduced_motion: bool,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_PLAYER_SETTINGS_SCHEMA,
            video: VideoSettings {
                display_mode: DisplayMode::Fullscreen,
                width: 1_920,
                height: 1_080,
                shadows_enabled: true,
                shadow_map_resolution: 4_096,
                ambient_occlusion: true,
                vsync: true,
                fps_limit: None,
                brightness_ev: 0.0,
                gamma: 0.0,
                msaa_samples: 8,
                post_process_aa: PostProcessAntiAliasing::Smaa,
            },
            audio: AudioMixSettings {
                master: 1.0,
                music: 1.0,
                sound_effects: 1.0,
                ambience: 1.0,
            },
            camera: CameraSettings {
                pan_sensitivity: 10.0,
                zoom_sensitivity: 10.0,
                keyboard_pan_sensitivity: 10.0,
                edge_scroll_sensitivity: 10.0,
                field_of_view_degrees: 60,
                edge_scrolling: true,
                keyboard_movement: true,
                mouse_controls: true,
            },
            interface: InterfaceSettings {
                display_names: NameDisplayMode::AllPlayers,
                display_building_health: BuildingHealthDisplayMode::DamagedOnly,
                ui_scale_percent: default_ui_scale_percent(),
                high_contrast: false,
                reduced_motion: false,
            },
            // Favor short recovery windows for an unattended broadcast. Longer
            // intervals remain available in the settings menu.
            autosave_minutes: 1,
            timelapse: TimelapseSettings::default(),
        }
    }
}

#[derive(Clone, Debug, Error, PartialEq)]
pub enum PlayerSettingsValidationError {
    #[error("unsupported player-settings schema {0}")]
    Schema(u32),
    #[error("display dimensions must be between 640x480 and 16384x8640")]
    DisplayDimensions,
    #[error("shadow-map resolution must be 256, 512, 1024, 2048, or 4096")]
    ShadowResolution,
    #[error("MSAA samples must be 1, 2, 4, or 8")]
    Msaa,
    #[error("FPS limit must be 24, 30, 60, 120, or 240")]
    FpsLimit,
    #[error("video calibration values must be finite and between -5 and 5")]
    VideoCalibration,
    #[error("audio volumes must be finite values from 0 to 1")]
    AudioVolume,
    #[error("camera sensitivities must be finite values from 0 to 100")]
    CameraSensitivity,
    #[error("field of view must be between 30 and 120 degrees")]
    FieldOfView,
    #[error("UI scale must be between 75% and 150%")]
    UiScale,
    #[error("autosave interval must be 0, 1, 5, 10, 30, or 60 minutes")]
    AutosaveInterval,
}

impl PlayerSettings {
    pub fn upgrade(mut self) -> Result<Self, PlayerSettingsValidationError> {
        if self.schema_version == 1 {
            // The old calibration required users to select +0.5 EV to reach
            // the intended in-game luminance. That appearance is the new
            // neutral baseline, so retain the image while renormalizing the
            // stored user-facing value back to zero.
            self.video.brightness_ev =
                (self.video.brightness_ev - LEGACY_BEVY_EXPOSURE_OFFSET_EV).clamp(-5.0, 5.0);
            self.schema_version = 2;
        }
        if self.schema_version == 2 {
            // Schema 3 only adds serde-defaulted accessibility preferences, so
            // existing players retain the exact presentation they had before.
            self.schema_version = 3;
        }
        if self.schema_version == 3 {
            // The calibrated neutral image moved from the former -1.0 readout
            // to 0.0. Shift stored values in the opposite direction to keep
            // every existing player's rendered image unchanged.
            self.video.brightness_ev =
                (self.video.brightness_ev + PREVIOUS_INGAME_BRIGHTNESS_OFFSET_EV).clamp(-5.0, 5.0);
            self.schema_version = 4;
        }
        if self.schema_version == 4 {
            // Schema 5 adds a serde-defaulted city timelapse policy.
            self.schema_version = CURRENT_PLAYER_SETTINGS_SCHEMA;
        }
        self.validate()?;
        Ok(self)
    }

    pub fn validate(&self) -> Result<(), PlayerSettingsValidationError> {
        if self.schema_version != CURRENT_PLAYER_SETTINGS_SCHEMA {
            return Err(PlayerSettingsValidationError::Schema(self.schema_version));
        }
        if !(640..=16_384).contains(&self.video.width)
            || !(480..=8_640).contains(&self.video.height)
        {
            return Err(PlayerSettingsValidationError::DisplayDimensions);
        }
        if !matches!(
            self.video.shadow_map_resolution,
            256 | 512 | 1_024 | 2_048 | 4_096
        ) {
            return Err(PlayerSettingsValidationError::ShadowResolution);
        }
        if !matches!(self.video.msaa_samples, 1 | 2 | 4 | 8) {
            return Err(PlayerSettingsValidationError::Msaa);
        }
        if self
            .video
            .fps_limit
            .is_some_and(|limit| !matches!(limit, 24 | 30 | 60 | 120 | 240))
        {
            return Err(PlayerSettingsValidationError::FpsLimit);
        }
        if !valid_signed_setting(self.video.brightness_ev)
            || !valid_signed_setting(self.video.gamma)
        {
            return Err(PlayerSettingsValidationError::VideoCalibration);
        }
        if [
            self.audio.master,
            self.audio.music,
            self.audio.sound_effects,
            self.audio.ambience,
        ]
        .into_iter()
        .any(|volume| !volume.is_finite() || !(0.0..=1.0).contains(&volume))
        {
            return Err(PlayerSettingsValidationError::AudioVolume);
        }
        if [
            self.camera.pan_sensitivity,
            self.camera.zoom_sensitivity,
            self.camera.keyboard_pan_sensitivity,
            self.camera.edge_scroll_sensitivity,
        ]
        .into_iter()
        .any(|sensitivity| !sensitivity.is_finite() || !(0.0..=100.0).contains(&sensitivity))
        {
            return Err(PlayerSettingsValidationError::CameraSensitivity);
        }
        if !(30..=120).contains(&self.camera.field_of_view_degrees) {
            return Err(PlayerSettingsValidationError::FieldOfView);
        }
        if !(75..=150).contains(&self.interface.ui_scale_percent) {
            return Err(PlayerSettingsValidationError::UiScale);
        }
        if !matches!(self.autosave_minutes, 0 | 1 | 5 | 10 | 30 | 60) {
            return Err(PlayerSettingsValidationError::AutosaveInterval);
        }
        Ok(())
    }
}

fn valid_signed_setting(value: f32) -> bool {
    value.is_finite() && (-5.0..=5.0).contains(&value)
}

const fn default_ui_scale_percent() -> u16 {
    100
}

#[derive(Debug, Error)]
pub enum PlayerSettingsStoreError {
    #[error("player-settings I/O failed: {0}")]
    Io(#[from] io::Error),
    #[error("player-settings serialization failed: {0}")]
    Serialize(#[from] ron::Error),
    #[error("player-settings deserialization failed: {0}")]
    Deserialize(#[from] ron::error::SpannedError),
    #[error("player settings failed validation: {0}")]
    Validation(#[from] PlayerSettingsValidationError),
}

pub struct PlayerSettingsStore {
    path: PathBuf,
}

impl PlayerSettingsStore {
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[must_use]
    pub fn backup_path(&self) -> PathBuf {
        PathBuf::from(format!("{}.bak", self.path.display()))
    }

    pub fn load(&self) -> Result<PlayerSettings, PlayerSettingsStoreError> {
        match Self::load_path(&self.path) {
            Ok(settings) => Ok(settings),
            Err(primary) => Self::load_path(&self.backup_path()).map_err(|_| primary),
        }
    }

    pub fn write(&self, settings: &PlayerSettings) -> Result<(), PlayerSettingsStoreError> {
        settings.validate()?;
        let encoded = ron::ser::to_string_pretty(settings, ron::ser::PrettyConfig::default())?;
        let parent = self.path.parent().unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent)?;
        let temporary = PathBuf::from(format!("{}.tmp", self.path.display()));
        let backup = self.backup_path();
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&temporary)?;
        file.write_all(encoded.as_bytes())?;
        file.sync_all()?;
        if self.path.is_file() {
            fs::copy(&self.path, &backup)?;
            fs::remove_file(&self.path)?;
        }
        if let Err(error) = fs::rename(&temporary, &self.path) {
            if backup.is_file() && !self.path.exists() {
                let _ = fs::copy(&backup, &self.path);
            }
            return Err(error.into());
        }
        Ok(())
    }

    fn load_path(path: &Path) -> Result<PlayerSettings, PlayerSettingsStoreError> {
        let encoded = fs::read_to_string(path)?;
        let settings: PlayerSettings = ron::from_str(&encoded)?;
        Ok(settings.upgrade()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_valid_and_round_trip() {
        let settings = PlayerSettings::default();
        settings.validate().unwrap();
        let encoded = ron::to_string(&settings).unwrap();
        assert_eq!(ron::from_str::<PlayerSettings>(&encoded).unwrap(), settings);
    }

    #[test]
    fn upgrades_old_bevy_brightness_to_the_new_neutral_baseline() {
        let defaults = PlayerSettings::default();
        let settings = PlayerSettings {
            schema_version: 1,
            video: VideoSettings {
                brightness_ev: 0.5,
                ..defaults.video
            },
            ..defaults
        };
        let upgraded = settings.upgrade().unwrap();
        assert_eq!(upgraded.schema_version, CURRENT_PLAYER_SETTINGS_SCHEMA);
        assert!((upgraded.video.brightness_ev - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn schema_three_minus_one_brightness_becomes_the_new_neutral_zero() {
        let defaults = PlayerSettings::default();
        let settings = PlayerSettings {
            schema_version: 3,
            video: VideoSettings {
                brightness_ev: -1.0,
                ..defaults.video
            },
            ..defaults
        };
        let upgraded = settings.upgrade().unwrap();
        assert_eq!(upgraded.schema_version, CURRENT_PLAYER_SETTINGS_SCHEMA);
        assert!(upgraded.video.brightness_ev.abs() < f32::EPSILON);
    }

    #[test]
    fn schema_two_files_gain_non_disruptive_accessibility_defaults() {
        #[derive(Serialize)]
        struct SchemaTwoInterface {
            display_names: NameDisplayMode,
            display_building_health: BuildingHealthDisplayMode,
        }

        #[derive(Serialize)]
        struct SchemaTwoSettings<'a> {
            schema_version: u32,
            video: &'a VideoSettings,
            audio: &'a AudioMixSettings,
            camera: &'a CameraSettings,
            interface: SchemaTwoInterface,
            autosave_minutes: u16,
        }

        let current = PlayerSettings::default();
        let encoded = ron::to_string(&SchemaTwoSettings {
            schema_version: 2,
            video: &current.video,
            audio: &current.audio,
            camera: &current.camera,
            interface: SchemaTwoInterface {
                display_names: current.interface.display_names,
                display_building_health: current.interface.display_building_health,
            },
            autosave_minutes: current.autosave_minutes,
        })
        .unwrap();

        let decoded = ron::from_str::<PlayerSettings>(&encoded).unwrap();
        assert_eq!(decoded.interface.ui_scale_percent, 100);
        assert!(!decoded.interface.high_contrast);
        assert!(!decoded.interface.reduced_motion);
        assert_eq!(
            decoded.upgrade().unwrap().schema_version,
            CURRENT_PLAYER_SETTINGS_SCHEMA
        );
    }

    #[test]
    fn rejects_ui_scales_outside_the_supported_readable_range() {
        let mut settings = PlayerSettings::default();
        settings.interface.ui_scale_percent = 74;
        assert_eq!(
            settings.validate(),
            Err(PlayerSettingsValidationError::UiScale)
        );
        settings.interface.ui_scale_percent = 151;
        assert_eq!(
            settings.validate(),
            Err(PlayerSettingsValidationError::UiScale)
        );
    }

    #[test]
    fn store_recovers_last_valid_backup() {
        let directory = tempfile::tempdir().unwrap();
        let store = PlayerSettingsStore::new(directory.path().join("settings.ron"));
        let first = PlayerSettings::default();
        store.write(&first).unwrap();
        let mut second = first.clone();
        second.audio.master = 0.25;
        store.write(&second).unwrap();
        fs::write(store.path(), "not ron").unwrap();
        assert_eq!(store.load().unwrap(), first);
    }
}
