//! Engine-independent Stream Town data, deterministic simulation, and persistence.

pub mod command;
pub mod config;
pub mod content;
pub mod id;
pub mod navigation;
pub mod save;
pub mod simulation;
pub mod world;

pub use command::{ChatCommand, CommandParseError};
pub use config::{GameConfig, GameplayConfig, WindowConfig, WorldGenConfig};
pub use content::{
    ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue,
    BuildingDef, ContentCatalog, RoleDef, TechGroup, TechNode, TechTree,
};
pub use id::{StableId, StableIdError};
pub use navigation::{DirtyRegion, GridPos, NavGrid, NavigationError};
pub use save::{
    ActorKind, LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, NativeSaveError,
    NativeSaveStore, SavedActor, SavedTerrainMesh, WorldSnapshot, inspect_legacy_save,
};
pub use simulation::{
    ActorState, BuildingState, Season, SimulationError, TechVote, TownEvent, Weather,
    WorldSimulation,
};
pub use world::{GeneratedResource, GeneratedWorld, generate_world};
