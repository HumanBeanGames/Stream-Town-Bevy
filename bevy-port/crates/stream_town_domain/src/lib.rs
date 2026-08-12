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
pub use command::{
    BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand,
    CommandParseError, CustomizationKind,
};
pub use config::{GameConfig, GameplayConfig, TwitchConfig, WindowConfig, WorldGenConfig};
pub use content::{
    ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue,
    BuildingDef, CURRENT_CONTENT_SCHEMA, ContentCatalog, EnemyDef, EnemySpawnerDef, HealthDef,
    ObjectiveDef, ObjectiveKind, ProjectileShooterDef, RoleDef, RoleEquipmentDef,
    RoleSlotContribution, StationDef, StorageContribution, TechGroup, TechNode, TechTree,
    WeightedEnemySpawn,
};
pub use id::{StableId, StableIdError};
pub use navigation::{DirtyRegion, GridPos, NavGrid, NavigationError};
pub use presentation::{
    AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef,
    AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationParameterDef,
    AnimationParameterKind, AnimationQuatKeyframe, AnimationStateDef, AnimationStateMachineDef,
    AnimationTransformTrack, AnimationTransitionDef, AnimationVec3Keyframe, MaterialAlphaMode,
    MaterialDef, PrefabPresentationBinding, PresentationCatalog, PresentationError, TextureDef,
};
pub use save::{
    ActorKind, LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, NativeSaveError,
    NativeSaveStore, SavedActor, SavedTerrainMesh, WorldSnapshot, inspect_legacy_save,
};
pub use simulation::{
    ActorCustomization, ActorState, BUILDING_MAX_HEALTH, BuildingState, EnemyCampState,
    FishGodState, MAX_ROLE_LEVEL, ObjectiveEvent, ObjectiveProgress, RULER_VOTE_DURATION_SECONDS,
    RULER_VOTE_INTERVAL_SECONDS, RaidState, RoleProgress, RulerVoteKind, RulerVoteState, Season,
    SimulationError, TechVote, TownEvent, TownGoalState, Weather, WorldSimulation,
    required_role_experience,
};
pub use world::{GeneratedResource, GeneratedWorld, generate_world};
