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
    AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback,
    WeightedAnimationMotion,
};
pub use command::{
    BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand,
    CommandParseError, CustomizationKind,
};
pub use config::{GameConfig, GameplayConfig, TwitchConfig, WindowConfig, WorldGenConfig};
pub use content::{
    ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue,
    BuildingDef, CURRENT_CONTENT_SCHEMA, ContentCatalog, EnemyDef, EnemySpawnerDef, FoliageHabitat,
    FoliageLayerDef, FoliageVariantDef, HealthDef, ObjectiveDef, ObjectiveKind,
    PassiveResourceContribution, ProjectileShooterDef, ResourceReward, RoleDef, RoleEquipmentDef,
    RoleSlotContribution, StationDef, StorageContribution, TechGroup, TechNode, TechTree,
    WeightedEnemySpawn,
};
pub use id::{StableId, StableIdError};
pub use navigation::{DirtyRegion, GridPos, NavGrid, NavigationError};
pub use presentation::{
    AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef,
    AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef,
    AnimationMotionDef, AnimationObjectReference, AnimationParameterDef, AnimationParameterKind,
    AnimationPropertyCurve, AnimationQuatKeyframe, AnimationStateDef, AnimationStateMachineDef,
    AnimationTangent, AnimationTransformTrack, AnimationTransitionDef, AnimationVec3Keyframe,
    AvatarMaskDef, MaterialAlphaMode, MaterialDef, PrefabPresentationBinding, PresentationCatalog,
    PresentationError, RendererMaterialBinding, TextureDef, TextureTransform,
};
pub use save::{
    ActorKind, LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, NativeSaveError,
    NativeSaveStore, SavedActor, SavedTerrainMesh, SavedTerrainMeshError, WorldSnapshot,
    inspect_legacy_save,
};
pub use simulation::{
    ActorCustomization, ActorState, BUILDING_MAX_HEALTH, BuildingState, EnemyCampState,
    FishGodState, MAX_ROLE_LEVEL, ObjectiveEvent, ObjectiveProgress, RULER_VOTE_DURATION_SECONDS,
    RULER_VOTE_INTERVAL_SECONDS, RaidState, RoleProgress, RulerVoteKind, RulerVoteState, Season,
    SimulationError, TechVote, TownEvent, TownGoalState, Weather, WorldSimulation,
    required_role_experience,
};
pub use world::{
    GeneratedFoliage, GeneratedResource, GeneratedWorld, generate_world,
    generate_world_with_content,
};
