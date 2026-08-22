//! A zero-setup, Tidal-inspired procedural music engine for Bevy.
//!
//! [`TidalPlugin`] contains both the pattern scheduler and audio renderer. It
//! does not launch GHCi, TidalCycles, SuperCollider, or any other process.
//! Applications can submit familiar mini-notation through [`TidalController`]
//! and observe scheduled notes through [`TidalEvent`].

mod audio;
mod backend;
mod pattern;

use bevy::prelude::*;
use std::path::PathBuf;

pub use audio::{NativeAudioState, NativeAudioStatus, TidalEvent};
pub use backend::{TidalBackendState, TidalBackendStatus, TidalController};

/// Settings shared by the native pattern scheduler and audio renderer.
#[derive(Resource, Clone, Debug)]
pub struct TidalConfig {
    /// Root containing sample-bank directories.
    pub samples_path: PathBuf,
    /// Tidal-style cycles per second. The traditional default is `0.5`.
    pub cycles_per_second: f64,
    /// How far ahead the native scheduler queues notes.
    pub scheduler_lookahead: std::time::Duration,
}

impl Default for TidalConfig {
    fn default() -> Self {
        Self {
            samples_path: PathBuf::from("samples"),
            cycles_per_second: 0.5,
            scheduler_lookahead: std::time::Duration::from_millis(100),
        }
    }
}

/// Adds the native Rust pattern scheduler and audio renderer.
#[derive(Default)]
pub struct TidalPlugin {
    pub config: TidalConfig,
}

impl Plugin for TidalPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone())
            .add_plugins((audio::NativeAudioPlugin, backend::TidalBackendPlugin));
    }
}
