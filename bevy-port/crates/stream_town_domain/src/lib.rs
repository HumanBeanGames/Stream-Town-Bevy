//! Engine-independent Stream Town data, deterministic simulation, and persistence.

pub mod animation;
pub mod command;
pub mod config;
pub mod content;
pub mod id;
pub mod navigation;
pub mod presentation;
pub mod save;
pub mod simulation;
pub mod world;

pub use animation::{
    AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue,
    AnimationRuntimeError, AnimationTransitionOutcome, WeightedAnimationMotion,
};
pub use command::{ChatCommand, CommandParseError};
pub use config::{GameConfig, GameplayConfig, TwitchConfig, WindowConfig, WorldGenConfig};
pub use content::{
    ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue,
    BuildingDef, CURRENT_CONTENT_SCHEMA, ContentCatalog, ObjectiveDef, ObjectiveKind, RoleDef,
    RoleEquipmentDef, StationDef, StorageContribution, TechGroup, TechNode, TechTree,
};
pub use id::{StableId, StableIdError};
pub use navigation::{DirtyRegion, GridPos, NavGrid, NavigationError};
pub use presentation::{
    AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef,
    AnimationMotionDef, AnimationParameterDef, AnimationParameterKind, AnimationQuatKeyframe,
    AnimationStateDef, AnimationTransformTrack, AnimationTransitionDef, AnimationVec3Keyframe,
    MaterialAlphaMode, MaterialDef, PrefabPresentationBinding, PresentationCatalog,
    PresentationError, TextureDef,
};
pub use save::{
    ActorKind, LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, NativeSaveError,
    NativeSaveStore, SavedActor, SavedTerrainMesh, WorldSnapshot, inspect_legacy_save,
};
pub use simulation::{
    ActorState, BUILDING_MAX_HEALTH, BuildingState, MAX_ROLE_LEVEL, ObjectiveEvent,
    ObjectiveProgress, RoleProgress, Season, SimulationError, TechVote, TownEvent, TownGoalState,
    Weather, WorldSimulation, required_role_experience,
};
pub use world::{GeneratedResource, GeneratedWorld, generate_world};
