use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const CURRENT_PLAYER_SETTINGS_SCHEMA: u32 = 2;
const LEGACY_BEVY_EXPOSURE_OFFSET_EV: f32 = 0.5;

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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlayerSettings {
    pub schema_version: u32,
    pub video: VideoSettings,
    pub audio: AudioMixSettings,
    pub camera: CameraSettings,
    pub interface: InterfaceSettings,
    pub autosave_minutes: u16,
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
                display_names: NameDisplayMode::None,
                display_building_health: BuildingHealthDisplayMode::DamagedOnly,
            },
            // Unity's default index 3 maps through [0, 5, 10, 30, 60].
            autosave_minutes: 30,
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
    #[error("autosave interval must be 0, 5, 10, 30, or 60 minutes")]
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
        if !matches!(self.autosave_minutes, 0 | 5 | 10 | 30 | 60) {
            return Err(PlayerSettingsValidationError::AutosaveInterval);
        }
        Ok(())
    }

    pub fn from_unity_json(encoded: &str) -> Result<Self, serde_json::Error> {
        let legacy: UnitySettingsData = serde_json::from_str(encoded)?;
        let defaults = Self::default();
        let shadow_type = legacy.shadow_type.unwrap_or(2).clamp(0, 2);
        let shadow_resolution = legacy.shadow_resolution.unwrap_or(4).clamp(0, 4);
        let msaa = legacy.anti_aliasing.unwrap_or(3).clamp(0, 3);
        let mut settings = Self {
            video: VideoSettings {
                display_mode: match legacy.display_mode.unwrap_or(2).clamp(0, 2) {
                    0 => DisplayMode::Windowed,
                    1 => DisplayMode::Borderless,
                    _ => DisplayMode::Fullscreen,
                },
                shadows_enabled: shadow_type != 0,
                shadow_map_resolution: [256, 512, 1_024, 2_048, 4_096]
                    [usize::try_from(shadow_resolution).unwrap_or(4)],
                ambient_occlusion: legacy.enabled_ao.unwrap_or(true),
                vsync: legacy.v_sync.unwrap_or(true),
                fps_limit: match legacy.fps_limiter.unwrap_or(5).clamp(0, 5) {
                    0 => Some(24),
                    1 => Some(30),
                    2 => Some(60),
                    3 => Some(120),
                    4 => Some(240),
                    _ => None,
                },
                brightness_ev: legacy.brightness.unwrap_or(0.0).clamp(-5.0, 5.0),
                gamma: legacy.gamma.unwrap_or(0.0).clamp(-5.0, 5.0),
                msaa_samples: [1, 2, 4, 8][usize::try_from(msaa).unwrap_or(3)],
                post_process_aa: match legacy.camera_aa.unwrap_or(2).clamp(0, 2) {
                    1 => PostProcessAntiAliasing::Fxaa,
                    2 => PostProcessAntiAliasing::Smaa,
                    _ => PostProcessAntiAliasing::None,
                },
                ..defaults.video
            },
            audio: AudioMixSettings {
                master: clamp_unit(legacy.master_volume.unwrap_or(1.0)),
                music: clamp_unit(legacy.music_volume.unwrap_or(1.0)),
                sound_effects: clamp_unit(legacy.player_volume.unwrap_or(1.0)),
                ambience: clamp_unit(legacy.environment_volume.unwrap_or(1.0)),
            },
            camera: CameraSettings {
                pan_sensitivity: clamp_sensitivity(legacy.pan_sensitivity.unwrap_or(10.0)),
                zoom_sensitivity: clamp_sensitivity(legacy.zoom_sensitivity.unwrap_or(10.0)),
                keyboard_pan_sensitivity: clamp_sensitivity(
                    legacy.wasd_sensitivity.unwrap_or(10.0),
                ),
                edge_scroll_sensitivity: clamp_sensitivity(
                    legacy.edge_scrolling_sensitivity.unwrap_or(10.0),
                ),
                field_of_view_degrees: u16::try_from(legacy.fov.unwrap_or(60).clamp(30, 120))
                    .unwrap_or(60),
                edge_scrolling: legacy.edge_scrolling.unwrap_or(true),
                keyboard_movement: legacy.keyboard_movement.unwrap_or(true),
                mouse_controls: legacy.mouse_controls.unwrap_or(true),
            },
            interface: InterfaceSettings {
                display_names: match legacy.display_names.unwrap_or(0).clamp(0, 2) {
                    1 => NameDisplayMode::StaffAndSubscribers,
                    2 => NameDisplayMode::AllPlayers,
                    _ => NameDisplayMode::None,
                },
                display_building_health: match legacy
                    .display_building_damage
                    .unwrap_or(1)
                    .clamp(0, 2)
                {
                    0 => BuildingHealthDisplayMode::None,
                    2 => BuildingHealthDisplayMode::Always,
                    _ => BuildingHealthDisplayMode::DamagedOnly,
                },
            },
            autosave_minutes: match legacy.autosave_time.unwrap_or(3).clamp(0, 4) {
                0 => 0,
                1 => 5,
                2 => 10,
                3 => 30,
                _ => 60,
            },
            ..defaults
        };
        settings.schema_version = CURRENT_PLAYER_SETTINGS_SCHEMA;
        Ok(settings)
    }
}

fn valid_signed_setting(value: f32) -> bool {
    value.is_finite() && (-5.0..=5.0).contains(&value)
}

fn clamp_unit(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        1.0
    }
}

fn clamp_sensitivity(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 100.0)
    } else {
        10.0
    }
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

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UnitySettingsData {
    display_mode: Option<i32>,
    shadow_type: Option<i32>,
    shadow_resolution: Option<i32>,
    #[serde(rename = "enabledAO")]
    enabled_ao: Option<bool>,
    v_sync: Option<bool>,
    fps_limiter: Option<i32>,
    brightness: Option<f32>,
    gamma: Option<f32>,
    anti_aliasing: Option<i32>,
    #[serde(rename = "cameraAA")]
    camera_aa: Option<i32>,
    master_volume: Option<f32>,
    music_volume: Option<f32>,
    player_volume: Option<f32>,
    environment_volume: Option<f32>,
    pan_sensitivity: Option<f32>,
    zoom_sensitivity: Option<f32>,
    wasd_sensitivity: Option<f32>,
    edge_scrolling_sensitivity: Option<f32>,
    fov: Option<i32>,
    autosave_time: Option<i32>,
    display_names: Option<i32>,
    display_building_damage: Option<i32>,
    edge_scrolling: Option<bool>,
    keyboard_movement: Option<bool>,
    mouse_controls: Option<bool>,
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
        assert!(upgraded.video.brightness_ev.abs() < f32::EPSILON);
    }

    #[test]
    fn imports_unity_json_indices_and_clamps_values() {
        let settings = PlayerSettings::from_unity_json(
            r#"{
                "displayMode": 1, "shadowType": 0, "shadowResolution": 2,
                "enabledAO": false, "vSync": false, "fpsLimiter": 2,
                "antiAliasing": 1, "cameraAA": 1,
                "masterVolume": 0.5, "playerVolume": 2.0,
                "wasdSensitivity": 17.0, "autosaveTime": 4,
                "displayNames": 2, "displayBuildingDamage": 0
            }"#,
        )
        .unwrap();
        assert_eq!(settings.video.display_mode, DisplayMode::Borderless);
        assert!(!settings.video.shadows_enabled);
        assert_eq!(settings.video.shadow_map_resolution, 1_024);
        assert_eq!(settings.video.fps_limit, Some(60));
        assert_eq!(settings.video.msaa_samples, 2);
        assert_eq!(
            settings.video.post_process_aa,
            PostProcessAntiAliasing::Fxaa
        );
        assert!((settings.audio.master - 0.5).abs() < f32::EPSILON);
        assert!((settings.audio.sound_effects - 1.0).abs() < f32::EPSILON);
        assert_eq!(settings.autosave_minutes, 60);
        assert_eq!(
            settings.interface.display_names,
            NameDisplayMode::AllPlayers
        );
        settings.validate().unwrap();
    }

    #[test]
    fn imports_unity_subscriber_name_display_index() {
        let settings = PlayerSettings::from_unity_json(r#"{"displayNames":1}"#).unwrap();
        assert_eq!(
            settings.interface.display_names,
            NameDisplayMode::StaffAndSubscribers
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
