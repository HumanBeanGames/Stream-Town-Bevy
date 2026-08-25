//! Engine-independent Stream Town data, deterministic simulation, and persistence.

pub mod animation;
pub mod command;
pub mod config;
pub mod content;
pub mod id;
pub mod menu_scene;
pub mod navigation;
pub mod presentation;
pub mod runtime_console;
pub mod save;
pub mod settings;
pub mod simulation;
pub mod technology_layout;
pub mod world;

pub use animation::{
    AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue,
    AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback,
    WeightedAnimationMotion,
};
pub use command::{
    BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand,
    CommandParseError, CustomizationKind, UNITY_CREATE_COMMAND_ALIASES, unity_command_usage,
};
pub use config::{
    BroadcastConfig, BroadcastEncoderPreference, GameConfig, GameplayConfig,
    SHIPPING_FISH_GOD_REWARD_ID, SHIPPING_SECONDS_PER_DAY, TimeCycleConfig, TimeCycleSample,
    TwitchConfig, WindowConfig, WorldGenConfig,
};
pub use content::{
    ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue,
    BuildingDef, BuildingModelDef, CURRENT_CONTENT_SCHEMA, ContentCatalog, EnemyDef,
    EnemyModelSetDef, EnemyRunAnimation, EnemySpawnerDef, EnemyWeaponModelDef, FoliageHabitat,
    FoliageLayerDef, FoliageVariantDef, HealthDef, LoadingScreenDef, ObjectiveDef, ObjectiveKind,
    PassiveResourceContribution, PetDef, PetModelDef, ProjectileShooterDef, ResourceReward,
    RoleDef, RoleEquipmentDef, RoleSlotContribution, RotatingNodeDef, StationDef,
    StationUpdateMode, StorageContribution, StorageModelDef, TargetingScoreDef, TechGroup,
    TechNode, TechTree, WeightedEnemySpawn,
};
pub use id::{StableId, StableIdError};
pub use menu_scene::{
    MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual,
    MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference,
};
pub use navigation::{DirtyRegion, GridPos, NavGrid, NavigationError};
pub use presentation::{
    AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef,
    AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef,
    AnimationMotionDef, AnimationObjectReference, AnimationParameterDef, AnimationParameterKind,
    AnimationPropertyCurve, AnimationQuatKeyframe, AnimationStateDef, AnimationStateMachineDef,
    AnimationTangent, AnimationTransformTrack, AnimationTransitionDef, AnimationVec3Keyframe,
    AvatarMaskDef, ChimneySmokeDef, FireworksVfxDef, FishSchoolVfxDef, HealingBurstVfxDef,
    HealingChannelVfxDef, MaterialAlphaMode, MaterialDef, PostProcessBloomDef,
    PostProcessColorAdjustmentsDef, PostProcessMotionBlurDef, PostProcessProfileDef,
    PostProcessTonemapping, PostProcessVignetteDef, PrefabChimneyEmitterBinding,
    PrefabPresentationBinding, PresentationCatalog, PresentationError, RainingFishVfxDef,
    RendererMaterialBinding, RoleActionAudioDef, SceneFireworksBinding, SceneFishSchoolBinding,
    ScenePostProcessBinding, TextureDef, TextureTransform, VfxAlphaKeyframe, VfxColorKeyframe,
    VfxGradientDef,
};
pub use runtime_console::{
    CURRENT_RUNTIME_CONSOLE_SCHEMA, RuntimeConsoleAction, RuntimeConsoleRequest,
    RuntimeConsoleStatus, RuntimeConsoleStore, RuntimeConsoleStoreError,
    RuntimeConsoleValidationError,
};
pub use save::{
    ActorKind, CURRENT_WORLD_SNAPSHOT_SCHEMA, LegacyMigrationMetadata, LegacySaveInfo,
    LegacySaveKind, NativeSaveError, NativeSaveStore, SavedActor, SavedTerrainMesh,
    SavedTerrainMeshError, WorldSnapshot, inspect_legacy_save,
};
pub use settings::{
    AudioMixSettings, BuildingHealthDisplayMode, CURRENT_PLAYER_SETTINGS_SCHEMA, CameraSettings,
    DisplayMode, InterfaceSettings, NameDisplayMode, PlayerSettings, PlayerSettingsStore,
    PlayerSettingsStoreError, PlayerSettingsValidationError, PostProcessAntiAliasing,
    VideoSettings,
};
pub use simulation::{
    ActorCustomization, ActorState, BUILDING_MAX_HEALTH, BuildingState, CURRENT_SIMULATION_SCHEMA,
    EnemyCampState, FishGodState, MAX_ROLE_LEVEL, ObjectiveEvent, ObjectiveProgress,
    RULER_VOTE_DURATION_SECONDS, RULER_VOTE_INTERVAL_SECONDS, RaidState, RoleProgress,
    RulerVoteKind, RulerVoteState, Season, SimulationError, StreamUserType, TechVote, TownEvent,
    TownGoalState, Weather, WorldSimulation, required_role_experience,
};
pub use technology_layout::{
    CURRENT_TECHNOLOGY_LAYOUT_SCHEMA, GraphPoint, GraphSize, TechnologyGraphLayout,
    TechnologyGroupLayout, TechnologyLayoutError, TechnologyNodeLayout,
};
pub use world::{
    GeneratedFoliage, GeneratedResource, GeneratedWorld, WorldGenerationStage, generate_world,
    generate_world_with_content, generate_world_with_content_observed, legacy_v1_world_hash,
    legacy_v2_world_hash, legacy_v3_world_hash, shoreline_approaches,
};
