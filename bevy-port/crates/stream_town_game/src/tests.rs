#![allow(clippy::float_cmp)]

use super::bootstrap::{
    is_transient_surface_configuration_error, player_window_mode, startup_window_mode,
};
use super::runtime::timelapse::{
    CityTimelapseRuntime, draw_timelapse_label, next_timelapse_frame_index,
    timelapse_frame_is_blank,
};
use super::*;
use sha2::{Digest, Sha256};
use stream_town_domain::generate_world;

include!("tests/world_foundations.rs");
include!("tests/loading_runtime.rs");
include!("tests/menu_accessibility.rs");
include!("tests/stations_agents.rs");
include!("tests/animation_audio.rs");
include!("tests/resources.rs");
include!("tests/combat_construction.rs");
include!("tests/loading_visuals.rs");
include!("tests/placement_visuals.rs");
include!("tests/actors_environment.rs");
include!("tests/regeneration.rs");
include!("tests/animation_twitch.rs");
include!("tests/persistence_runtime.rs");
include!("tests/paths.rs");
