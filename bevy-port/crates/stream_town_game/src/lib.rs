#![allow(clippy::wildcard_imports)]

mod app;
mod bootstrap;
#[cfg(target_os = "windows")]
pub mod direct_broadcast;
mod presentation;
#[cfg(all(feature = "stream-profiling", target_os = "windows"))]
pub mod profiling;
mod runtime;
mod tidal_music;
pub mod twitch;

use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex, OnceLock, PoisonError,
        atomic::{AtomicU64, Ordering as AtomicOrdering},
        mpsc,
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use accesskit::{Action as AccessAction, Live, Node as AccessibleNode, Role};
use anyhow::{Context, Result as AnyResult};
use avian3d::prelude::{Collider, PhysicsPlugins, RigidBody, SpatialQuery, SpatialQueryFilter};
#[cfg(target_os = "windows")]
use bevy::render::settings::Backends;
use bevy::{
    a11y::{AccessibilityNode, ActionRequest as AccessibilityActionRequest},
    animation::{
        AnimatedBy, AnimationClip, AnimationTargetId, RepeatAnimation, animated_field,
        graph::{AnimationGraph, AnimationGraphHandle, AnimationNodeIndex, AnimationNodeType},
        prelude::{AnimatableCurve, AnimatableKeyframeCurve},
        transition::AnimationTransitions,
    },
    anti_alias::{fxaa::Fxaa, smaa::Smaa},
    app::AnimationSystems,
    asset::{AssetId, AssetPlugin, LoadState, RenderAssetUsages, UntypedAssetId, UntypedHandle},
    audio::{AudioSink, AudioSinkPlayback, AudioSource, SpatialScale, Volume},
    camera::{Hdr, RenderTarget, ScalingMode},
    camera::{primitives::Aabb, visibility::NoFrustumCulling},
    color::LinearRgba,
    core_pipeline::tonemapping::Tonemapping,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::{query::QueryData, system::SystemParam},
    gltf::{GltfMaterialName, GltfMeshName},
    image::ImageSampler,
    input::mouse::{MouseScrollUnit, MouseWheel},
    input_focus::{
        FocusCause, InputFocus, InputFocusVisible,
        tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin},
    },
    light::{
        DirectionalLightShadowMap, NotShadowCaster,
        cluster::{ClusterConfig, ClusterFarZMode, ClusterZConfig},
    },
    math::Affine2,
    mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes},
    mesh::{Indices, VertexAttributeValues},
    pbr::ScreenSpaceAmbientOcclusion,
    pbr::{ExtendedMaterial, MaterialExtension, PreparedMaterial, RenderMeshInstances},
    post_process::{
        bloom::{Bloom, BloomCompositeMode, BloomPrefilter},
        effect_stack::Vignette,
        motion_blur::MotionBlur,
    },
    prelude::*,
    render::erased_render_asset::ErasedRenderAssets,
    render::error_handler::{ErrorType, RenderError, RenderErrorHandler, RenderErrorPolicy},
    render::mesh::RenderMesh,
    render::render_asset::RenderAssets as GpuRenderAssets,
    render::render_resource::{
        AsBindGroup, CachedPipelineState, Extent3d, PipelineCache, PrimitiveTopology, ShaderType,
        TextureDimension, TextureFormat,
    },
    render::sync_world::MainEntity,
    render::texture::GpuImage,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
    render::view::{ColorGrading, Msaa},
    render::{Render, RenderApp, RenderPlugin, RenderSystems, settings::WgpuSettings},
    shader::ShaderRef,
    sprite::{BorderRect, TextureSlicer},
    tasks::{AsyncComputeTaskPool, Task, block_on, poll_once},
    text::{EditableText, TextCursorStyle},
    transform::TransformSystems,
    ui_widgets::SelectAllOnFocus,
    window::{
        CursorOptions, MonitorSelection, OnMonitor, PresentMode, PrimaryWindow, WindowMode,
        WindowResolution,
    },
    winit::{UpdateMode, WinitSettings},
    world_serialization::{WorldInstance, WorldInstanceReady},
};
use stream_town_domain::{
    ActorCustomization, ActorKind, ActorState, AdaptiveMusicConfig, AnimationBlendSelection,
    AnimationClipDef, AnimationControllerRuntime, AnimationLayerBlendMode, AnimationLayerDef,
    AnimationTransformTrack, AnimationTransitionPlayback, ArchetypeDef, ArchetypeKind,
    ArchetypeScene, AvatarMaskDef, BUILDING_MAX_HEALTH, BroadcastConfig,
    BroadcastEncoderPreference, BroadcastRenderMode, BuildingAction, BuildingDef,
    BuildingDirection, BuildingHealthDisplayMode, BuildingModelDef, BuildingState,
    CURRENT_RUNTIME_CONSOLE_SCHEMA, CURRENT_SIMULATION_SCHEMA, CURRENT_WORLD_SNAPSHOT_SCHEMA,
    CameraAction, CameraDirection, ChatCommand, ChimneySmokeDef, CommunityEvent,
    CommunityVoteProposal, ContentCatalog, CustomizationKind, DAYS_PER_SEASON, DisplayMode,
    EnemyCampGenerationDef, EnemyCampState, EnemyModelSetDef, EnemyRunAnimation, FireworksVfxDef,
    FoliageHabitat, GameConfig, GeneratedFoliage, GeneratedWorld, GridPos, HealingBurstVfxDef,
    HealingChannelVfxDef, LegacyMigrationMetadata, MAX_TRAVERSAL_WEAR_SCORE,
    MainMenuSceneReference, MaterialAlphaMode as AuthoredAlphaMode, MaterialDef, NameDisplayMode,
    NativeSaveStore, ObjectiveEvent, ObjectiveKind, PetDef, PetModelDef, PlayerSettings,
    PlayerSettingsStore, PostProcessAntiAliasing, PostProcessProfileDef, PostProcessTonemapping,
    PresentationCatalog, RainingFishVfxDef, RoleEquipmentDef, RulerVoteKind, RuntimeConsoleAction,
    RuntimeConsoleStatus, RuntimeConsoleStore, SEASON_TRANSITION_SECONDS, SEASONS_PER_YEAR,
    SavedActor, SavedTerrainMesh, Season, StableId, StationDef, StationUpdateMode, StorageModelDef,
    StreamUserType, TargetingScoreDef, TimelapseInterval, TownEvent,
    VISIBLE_WATER_SURFACE_LIFT_METRES, VfxGradientDef, Weather, WorldGenerationStage,
    WorldSimulation, WorldSnapshot, foliage_visual_variant, foliage_visual_yaw_milliradians,
    generate_world_with_content, generate_world_with_content_observed,
    navigation_corner_height_metres, navigation_surface_height_at_world, parse_chat_commands,
    resource_visual_variant, visible_water_surface_height,
};

const MAX_TOWN_GOALS: usize = 2;
const AUTOMATIC_PLAYER_RESPAWN_SECONDS: f64 = 10.0 * 60.0;
const REGENERATION_TARGET_RETRY_SECONDS: f64 = 30.0;
const LOCAL_RESOURCE_SATURATION_RADIUS_CELLS: u16 = 10;
const LOCAL_RESOURCE_SATURATION_DECAY_PER_CHECK: u32 = 1;
const FORESTER_TREE_TIMER_RADIUS_CELLS: u16 = 20;
const FORESTER_TREE_TIMER_BONUS_PER_TREE: f64 = 0.01;
const FORESTER_INTERVAL_MULTIPLIER: f64 = 0.67;
const TENDER_INTERVAL_MULTIPLIER: f64 = 2.0;
const UNITY_IDLE_WANDER_INTERVAL_SECONDS: f32 = 3.0;
const FORESTER_RANDOM_FALLBACK_DENOMINATOR: u64 = 10;
const FORESTER_UTILITY_RADIUS_CELLS: i32 = 20;
const FORESTER_PREFERRED_NURSERY_DISTANCE_CELLS: f64 = 5.0;
const FORESTER_DIRECTION_MEMORY_SECONDS: f64 = 30.0 * 60.0;
const MAX_ACTIVE_CITIZEN_NIGHT_LIGHTS: usize = 32;
const MAX_ACTIVE_BUILDING_NIGHT_LIGHTS: usize = 20;
const MAX_ACTIVE_PROJECTILE_NIGHT_LIGHTS: usize = 12;
const NIGHT_LIGHT_POOL_CAPACITY: usize = MAX_ACTIVE_CITIZEN_NIGHT_LIGHTS
    + MAX_ACTIVE_BUILDING_NIGHT_LIGHTS
    + MAX_ACTIVE_PROJECTILE_NIGHT_LIGHTS;
const MAX_BUILDING_MATERIAL_UPDATES_PER_FRAME: usize = 2;
const NIGHT_LIGHT_TRANSITION_SECONDS: f32 = 10.0;
const PROJECTILE_MAX_LIFETIME_SECONDS: f32 = 12.0;
const UNITY_AUTHORED_GRID_CELL_SIZE: f32 = 2.0;
const TWITCH_COMMAND_HELP_URL: &str = "https://github.com/HumanBeanGames/Stream-Town-Bevy/blob/codex/bevy-migration/TWITCH_COMMANDS.md";
const BUILDING_PLACEMENT_TIMEOUT_SECONDS: f32 = 60.0;
const HEALING_TARGET_EFFECT_SCALE: f32 = 0.125;
const INVALID_TWITCH_COMMAND_REPLY: &str = "Invalid Command! Type !help for the list of commands!";
const TECHNOLOGY_VOTE_DURATION_SECONDS: f32 = 60.0;
const TECHNOLOGY_VOTE_OPTION_COUNT: usize = 3;
const TECHNOLOGY_VOTE_PANEL_HEIGHT: f32 = 400.0;
const TECHNOLOGY_VOTE_TITLE_TOP: f32 = 44.0;
const TECHNOLOGY_VOTE_FIRST_ROW_TOP: f32 = 78.0;
const TECHNOLOGY_VOTE_SINGLE_LINE_ADVANCE: f32 = 76.0;
const TECHNOLOGY_VOTE_EXTRA_LINE_ADVANCE: f32 = 13.0;
const TECHNOLOGY_VOTE_LABEL_OFFSET_Y: f32 = 3.0;
const TECHNOLOGY_VOTE_ICON_TOP: f32 = 35.0;
const TECHNOLOGY_VOTE_TIMER_TOP: f32 = 330.0;
const TECHNOLOGY_VOTE_TIMER_GROUP_TOP: f32 = TECHNOLOGY_VOTE_TIMER_TOP - 2.0;
const TECHNOLOGY_VOTE_TIMER_BAR_WIDTH: f32 = 150.0;
const TECHNOLOGY_VOTE_TIMER_GAP: f32 = 9.0;
const TECHNOLOGY_VOTE_TIMER_TEXT_OFFSET_Y: f32 = 2.0;
const UNITY_NUMBERED_LABEL_SECONDS: f32 = 15.0;
const WORLD_SCENE_PATH: &str = "Assets/Scenes/Worlds/World_Town.unity";
const MAIN_MENU_SCENE_PATH: &str = "Assets/Scenes/Menu/Main_Menu_02.unity";
const CREDITS_SCENE_PATH: &str = "Assets/Scenes/Menu/Credits.unity";
const PASSIVE_RESOURCE_FIXED_POINT_DENOMINATOR: u128 = 1_000_000_000_000;
const NIGHT_ENEMY_WAVE_INTERVAL_SECONDS: f64 = 120.0;
const ENEMY_WAVE_SPAWN_SPREAD_SECONDS: f64 = 10.0;
const CHIMNEY_ALPHA_STEPS: usize = 8;
const TERRAIN_CHUNK_CELLS: u16 = 16;
const OCEAN_PADDING_CELLS: u16 = 128;
const TRAVERSAL_WEAR_DECAY_INTERVAL_SECONDS: f32 = 1.0;
const PASSIVE_BUILDING_INCOME_INTERVAL: Duration = Duration::from_millis(250);
/// Navigation alone is subdivided into thirds; authored placement, saves, terrain
/// generation, resource generation, and gameplay ranges remain in town cells.
const NAVIGATION_SUBDIVISIONS: u16 = 3;
const NAVIGATION_TERRAIN_MIN_HEIGHT_CENTIMETRES: i16 = 45;
const WORLD_DIAGNOSTIC_VIEW_SECONDS: f32 = 10.0;
const WORLD_DIAGNOSTIC_OVERLAY_LIFT: f32 = 0.14;
const WORLD_DIAGNOSTIC_TILE_SCALE: f32 = 0.94;
const DEFENDER_LOCAL_PRIORITY_RADIUS_CELLS: u16 = 5;
const WORLD_DIAGNOSTIC_DEPTH_BIAS: f32 = 64.0;
const TERRAIN_HIGH_DETAIL_RADIUS: f32 = 220.0;
const TERRAIN_MEDIUM_DETAIL_RADIUS: f32 = 440.0;
const TERRAIN_LOD_HYSTERESIS: f32 = 36.0;
// Keep every practical shipping town actor on its authored skinned rig. The
// former 64-actor ceiling was shared by citizens and enemies, so later joins
// could remain capsule placeholders and their animations could be withheld.
// Explicit benchmark mode retains its measured low-detail ceiling.
const DEFAULT_ACTOR_DETAIL_BUDGET: usize = 6_400;
const PERFORMANCE_ACTOR_DETAIL_BUDGET: usize = 16;
const HEALING_CHANNEL_RING_SCALE: f32 = 0.035;
const HEALING_BURST_RING_SCALE: f32 = 0.24;
const HEALING_MOTE_SCALE: f32 = 0.035;
const HEALING_EMISSIVE_MAX_COMPONENT: f32 = 3.5;
// Orthographic framing is retained only for deterministic close-up smoke tests.
// Shipping gameplay uses the perspective camera authored in MainCamera.prefab.
const UNITY_TOWN_CAMERA_FOV_DEGREES: f32 = 60.0;
const UNITY_TOWN_CAMERA_NEAR: f32 = 0.3;
const UNITY_TOWN_CAMERA_FAR: f32 = 1_000.0;
const UNITY_TOWN_CAMERA_OFFSET: Vec3 = Vec3::new(-33.5, 33.240_562, 0.0);
const UNITY_TOWN_CAMERA_FOCUS_BACK_SHIFT: f32 = 16.0;
const UNITY_TOWN_CAMERA_MIN_HEIGHT: f32 = 11.0;
const UNITY_TOWN_CAMERA_MAX_HEIGHT: f32 = 60.0;
const UNITY_TOWN_CAMERA_MOVE_SMOOTHNESS: f32 = 5.0;
const PATH_CENTRE_THIRD_STEP_PENALTY: u32 = 2;
const TWITCH_CAMERA_VERTICAL_PAN_DISTANCE: f32 = 12.0;
const TWITCH_CAMERA_HORIZONTAL_PAN_DISTANCE: f32 = 6.0;
const BUILD_CURSOR_RAY_DISTANCE: f32 = 2_000.0;
const AUTO_CAMERA_IDLE_SECONDS: f32 = 30.0;
const AUTO_CAMERA_TOWN_SHOT_SECONDS: f32 = 12.0;
const AUTO_CAMERA_CITIZEN_SHOT_SECONDS: f32 = 24.0;
const AUTO_CAMERA_CITIZEN_HEIGHT: f32 = 15.0;
const AUTO_CAMERA_CITIZEN_FOCUS_HEIGHT: f32 = 1.4;
const AUTO_CAMERA_BUILDING_HEIGHT: f32 = 22.0;
const AUTO_CAMERA_BUILDING_FOCUS_HEIGHT: f32 = 2.5;
const AUTO_CAMERA_ATTENTION_SECONDS: f32 = 15.0;
const AUTO_CAMERA_COMBAT_REDIRECT_COOLDOWN_SECONDS: f32 = 5.0;
const AUTO_CAMERA_TOWN_SHOT_INTERVAL: u64 = 4;
const RETREAT_HEALTH_PERCENT: i32 = 25;
const RETREAT_TOWN_HALL_RADIUS_CELLS: u16 = 10;
const HEALING_HYSTERESIS_PERCENT: u128 = 125;
const ACTOR_OVERLAY_ANCHOR_HEIGHT_CELLS: f32 = 1.15;
const ACTOR_HEALTH_OVERLAY_TOP_PX: f32 = -11.0;
const ACTOR_NAME_OVERLAY_TOP_PX: f32 = 4.0;
const BUILDING_HEALTH_OVERLAY_WIDTH_PX: f32 = 82.0;
const PATH_HEALTH_OVERLAY_WIDTH_PX: f32 = BUILDING_HEALTH_OVERLAY_WIDTH_PX / 5.0;
const FOLIAGE_CAPTURE_TIMES_SECONDS: [f32; 12] =
    [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0];
pub const PLAYER_ANIMATED_MODEL_PATH: &str = "migrated/models/Models/Characters/Characters.glb";
pub const PLAYER_ANIMATED_SOURCE_MODEL: &str = "Assets/Models/Characters/Characters.fbx";
const PING_POINTER_DURATION_SECONDS: f32 = 8.0;
const PING_POINTER_MODEL_PATH: &str = "migrated/models/Models/VFX/PointerArrow.glb";
const PING_POINTER_MATERIAL_ID: &str = "material:799ef8ce46a71414286fd24c033a98fe";
const PING_POINTER_MODEL_MIN_Y: f32 = 851.829_35;
const PING_POINTER_MODEL_HEIGHT: f32 = 314.468_26;
const PLAYER_NAME_COLOR_PRESETS: [[u8; 3]; 10] = [
    [238, 91, 91],
    [244, 157, 72],
    [239, 213, 83],
    [112, 201, 101],
    [74, 198, 191],
    [83, 163, 238],
    [132, 116, 238],
    [194, 105, 226],
    [232, 109, 171],
    [232, 218, 196],
];
const FISH_GOD_EXIT_DELAY_SECONDS: f32 = 2.5;
const RAINING_FISH_PREFAB_SUFFIX: &str = "VFX/Environment/VFX_RainingFish.prefab";
const RAINING_FISH_RENDER_BUDGET: usize = 320;
const FISH_SCHOOL_PREFAB_SUFFIX: &str = "VFX/Environment/Fish.prefab";
const FISH_SCHOOL_RENDER_BUDGET_PER_BINDING: usize = 160;
const ROLE_ACTION_AUDIO_MAX_DISTANCE: f32 = 20.0;
const SELECTION_MASK_MATERIAL_ID: &str = "material:631afbf4de8b7ad4eabc5e30e0b2c778";
#[cfg(test)]
const SELECTION_MASK_TEXTURE_PATH: &str = "Assets/Textures/SelectionMask.png";
const PLAYER_SELECTION_OUTLINE_SCALE_CELLS: f32 = 1.5 * 1.25 / 2.0;
const SELECTION_OUTLINE_COLOR: Vec4 = Vec4::new(1.0, 0.92, 0.02, 0.92);
const SELECTION_OUTLINE_DEPTH_BIAS: f32 = 8.0;
const WORLD_REVEAL_GPU_STABLE_FRAMES: u64 = 6;
// Building_Gate.prefab uses a 4 x 1 x 4 trigger centred on the gate. Converted
// building models are authored at Unity scale and then scaled by cell_size / 2.
const GATE_TRIGGER_HALF_EXTENT_UNITY_UNITS: f32 = 2.0;
// Player_Character.prefab authors 100 Unity world units for both its target
// and builder-resource sensors. Shipping terrain uses two units per cell.
const PLAYER_TARGET_SEARCH_RANGE_CELLS: u16 = 50;
const STATION_TARGET_CHECK_MILLISECONDS: f64 = 2_000.0;
const BUILDING_PLACEMENT_SUCCESS_COLOR: [f32; 3] = [0.242_250_26, 0.896_226_4, 0.054_957_304];
const BUILDING_PLACEMENT_FAIL_COLOR: [f32; 3] = [0.933_962_3, 0.0, 0.0];
const TERRAIN_SHADER_ASSET_PATH: &str = "shaders/terrain_material.wgsl";
const TERRAIN_MATERIAL_PATH: &str = "Assets/Materials/Environment/Env_Terrain.mat";
const WATER_SHADER_ASSET_PATH: &str = "shaders/water_material.wgsl";
const WATER_MATERIAL_PATH: &str = "Assets/Materials/Environment/Env_Water.mat";
const MAIN_MENU_BASELINE_EXPOSURE_EV: f32 = -1.5;
// Neutral now renders exactly like the former -1.0 brightness setting:
// 0.5 authored baseline + (-1.0) user adjustment = -0.5 EV.
const IN_GAME_BASELINE_EXPOSURE_EV: f32 = -0.5;
// Entity construction must yield back to Bevy regularly so the loading UI can
// be extracted and presented. Asset decoding and deterministic generation run
// on worker pools. A time budget handles models with unequal construction cost,
// while the count ceilings also bound deferred-command application.
const LOADING_SCENE_TIME_BUDGET_PER_FRAME: Duration = Duration::from_millis(3);
const MAIN_MENU_SPAWN_BUDGET_PER_FRAME: usize = 48;
const WORLD_RESOURCE_SPAWN_BUDGET_PER_FRAME: usize = 48;
const WORLD_FOLIAGE_SPAWN_BUDGET_PER_FRAME: usize = 96;
const IN_GAME_AMBIENT_LIGHT_MULTIPLIER: f32 = 1.30;
const IN_GAME_SATURATION_MULTIPLIER: f32 = 1.12;
const BUILDING_SHADER_ASSET_PATH: &str = "shaders/building_material.wgsl";
const BUILDING_MATERIAL_PATH: &str = "Assets/Materials/Building_Material.mat";
const CLOUD_SHADER_ASSET_PATH: &str = "shaders/cloud_material.wgsl";
const CLOUD_MATERIAL_PATH: &str = "Assets/Materials/VFX/Clouds.mat";
// Mean alpha of MainGameTexture_01.tga. Unity's texture importer supplied a
// mip chain for this authored noise atlas; the migrated source TGA does not.
// The cloud shader uses this value as the correctly box-filtered limit when
// its 20x world-space noise layer is too small to resolve on screen.
const CLOUD_NOISE_ALPHA_MEAN: f32 = 67.635_544 / 255.0;
const GODRAY_SHADER_ASSET_PATH: &str = "shaders/godray_material.wgsl";
const GODRAY_MATERIAL_PATH: &str = "Assets/Materials/VFX/VFX_Godrays.mat";
const GIRAFFE_SHADER_ASSET_PATH: &str = "shaders/giraffe_material.wgsl";
const GIRAFFE_MATERIAL_PATH: &str = "Assets/Materials/Character/Giraffe.mat";
const BOUNDS_SHADER_ASSET_PATH: &str = "shaders/bounds_material.wgsl";
const BOUNDS_MATERIAL_PATH: &str = "Assets/Materials/BoundsVisualizer.mat";
const TREE_SHADER_ASSET_PATH: &str = "shaders/tree_material.wgsl";
const TREE_PREPASS_SHADER_ASSET_PATH: &str = "shaders/tree_material_prepass.wgsl";
const TREE_MATERIAL_PATH: &str = "Assets/Materials/Environment/Env_Tree.mat";
const GRASS_SHADER_ASSET_PATH: &str = "shaders/grass_material.wgsl";
const GRASS_PREPASS_SHADER_ASSET_PATH: &str = "shaders/grass_material_prepass.wgsl";
const GRASS_MATERIAL_PATH: &str = "Assets/Materials/Environment/Env_Grass.mat";
const CRITTER_SHADER_ASSET_PATH: &str = "shaders/critter_material.wgsl";
const CRITTER_PREPASS_SHADER_ASSET_PATH: &str = "shaders/critter_material_prepass.wgsl";
const CRITTER_MATERIAL_PATH: &str = "Assets/Materials/Critters/Critters.mat";
const CRITTER_MATERIAL_ID: &str = "material:56cfb478fa4b9e8469bcbbf9cf077701";
const FLAG_SHADER_ASSET_PATH: &str = "shaders/flag_material.wgsl";
const FLAG_MATERIAL_PATH: &str = "Assets/Materials/Prototype/Flags.mat";
const CHARACTER_SHADER_ASSET_PATH: &str = "shaders/character_material.wgsl";
const MENU_SKY_SHADER_ASSET_PATH: &str = "shaders/menu_sky_material.wgsl";
const CHARACTER_UNITY_SHADER_PATH: &str = "Assets/Shaders/LowPolyST.shader";
// Keep the offset local to skinned character receivers. The global directional
// light bias also affects terrain and foliage, whose synchronized shadow paths
// have already been visually validated.
const CHARACTER_SHADOW_RECEIVER_NORMAL_OFFSET: f32 = 0.02;
const GAME_LOGO_TEXTURE_PATH: &str = "Assets/Sprites/Miscellaneous/Game_Logo_DropShadow.png";
const LOADING_SCREEN_TEXTURE_PATH: &str = "Assets/Sprites/LoadingScreen/UI_LoadingScreen.png";
const LOADING_OVERLAY_TEXTURE_PATH: &str = "Assets/Sprites/LoadingScreen/UI_loadingOverlay.png";
const LOADING_ICON_TEXTURE_PATH: &str = "Assets/Sprites/LoadingScreen/UI_LoadingIcon.png";
const UI_FONT_ASSET_PATH: &str = "migrated/fonts/Rubik-Bold.ttf";
const UI_DISPLAY_FONT_ASSET_PATH: &str = "migrated/fonts/Luckiest Guy.ttf";
const LOADING_SCREEN_PREFAB_SUFFIX: &str = "UserInterface/UI_LoadingScreen.prefab";
const LOADING_ICON_HIERARCHY_PATH: &str = "LoadingScreen/LoadingIcon/Anchor/Image";
const GAME_LOGO_ASPECT_RATIO: f32 = 2_048.0 / 1_227.0;
const MAIN_MENU_TEXTURE_PATHS: [&str; 4] = [
    "Assets/Sprites/Buttons/UI_Button_Unpressed.png",
    "Assets/Sprites/Buttons/UI_Button.png",
    "Assets/Sprites/Buttons/UI_Button_Disabled.png",
    "Assets/Sprites/Objectives/UI_Objectives_Background.png",
];
const GAME_MENU_TEXTURE_PATHS: [&str; 3] = [
    "Assets/Sprites/VotingMenu/UI_VotingMenu_Background.png",
    "Assets/Sprites/Miscellaneous/UI_Cross.png",
    "Assets/Sprites/Miscellaneous/UI_Checkmark.png",
];
const SETTINGS_BACKGROUND_TEXTURE_PATH: &str = "Assets/Sprites/Settings/UI_Settings_Background.png";
const SECRETS_DISCLAIMER: &str = "By choosing Yes, you confirm that you are not presently streaming or recording this screen through OBS, Streamlabs, XSplit, browser capture, a capture card, or any other third-party application. The next screen contains sensitive Twitch account setup information. You accept any and all responsibility for keeping that information private and absolve Stream Town, its contributors, and all other parties of liability arising from violating this agreement.";
const SECRETS_PRIVACY_NOTICE: &str = "INTERNAL TWITCH VIDEO IS BLACKED OUT WHILE THIS SCREEN IS OPEN. Third-party capture software cannot be controlled by Stream Town. Never paste a Client Secret or stream key here.";
const SECRETS_INITIAL_FEEDBACK: &str =
    "Enter the public Client ID and both account logins, then authorize each account separately.";
const SETTINGS_PANEL_CORNER_SCALE: f32 = 1.5;
const TOP_BAR_TEXTURE_PATHS: [&str; 10] = [
    "Assets/Sprites/TopBar/UI_TopBar_Background.png",
    "Assets/Sprites/TopBar/UI_TopBar_Resources_Food.png",
    "Assets/Sprites/TopBar/UI_TopBar_Resources_Gold.png",
    "Assets/Sprites/TopBar/UI_TopBar_Resources_Ore.png",
    "Assets/Sprites/TopBar/UI_TopBar_Resources_Wood.png",
    "Assets/Sprites/TopBar/UI_TopBar_Stats_Players.png",
    "Assets/Sprites/TopBar/UI_TopBar_Stats_Buildings.png",
    "Assets/Sprites/TopBar/UI_TopBar_Stats_PlayTime.png",
    "Assets/Sprites/TopBar/UI_TopBar_SeasonGauge.png",
    "Assets/Sprites/TopBar/UI_TopBar_SeasonGauge_Meter.png",
];
const SELECTION_PANEL_TEXTURE_PATHS: [&str; 3] = [
    "Assets/Sprites/SelectionWindow/UI_SelectionWindow_Slider_Unfilled.png",
    "Assets/Sprites/SelectionWindow/UI_SelectionWindow_GreenSlider_Filled.png",
    "Assets/Sprites/SelectionWindow/UI_SelectionWindow_RedSlider_Filled.png",
];
const VOTE_TEXTURE_PATHS: [&str; 9] = [
    "Assets/Sprites/VotingMenu/UI_VotingMenu_Background.png",
    "Assets/Sprites/VotingMenu/UI_VotingMenu_TimerSlider_Filled.png",
    "Assets/Sprites/VotingMenu/UI_VotingMenu_TimerSlider_Unfilled.png",
    "Assets/Sprites/VotingMenu/UI_VotingMenu_Timer_Icon.png",
    "Assets/Sprites/VotingMenu/UI_VotingMenu_VotePrompt.png",
    "Assets/Sprites/VotingMenu/UI_VotingMenu_VoteSlider_Filled.png",
    "Assets/Sprites/VotingMenu/UI_VotingMenu_VoteSlider_Unfilled.png",
    "Assets/Sprites/RulerVotingMenu/UI_RulerVotingMenu_Icon.png",
    "Assets/Sprites/RulerVotingMenu/UI_RulerVotingMenu_TimerSlider_Filled.png",
];
const RULER_VOTE_TIMER_UNFILLED_PATH: &str =
    "Assets/Sprites/RulerVotingMenu/UI_RulerVotingMenu_TimerSlider_Unfilled.png";
const OBJECTIVE_TEXTURE_PATHS: [&str; 3] = [
    "Assets/Sprites/Objectives/UI_Objectives_Background.png",
    "Assets/Sprites/Objectives/UI_Objectives_Slider_Unfilled.png",
    "Assets/Sprites/Objectives/UI_Objectives_Slider_Filled.png",
];
const CURRENT_EVENT_TEXTURE_PATHS: [&str; 3] = [
    "Assets/Sprites/CurrentEvent/UI_CurrentEvent_Background.png",
    "Assets/Sprites/CurrentEvent/UI_CurrentEvent_Slider_Unfilled.png",
    "Assets/Sprites/CurrentEvent/UI_CurrentEvent_Slider_Filled.png",
];
const FOLIAGE_VISIBILITY_MIN_RANGE: f32 = 96.0;
const FOLIAGE_VISIBILITY_MAX_RANGE: f32 = 192.0;
const FOLIAGE_VISIBILITY_FADE: f32 = 18.0;
// Unity groups generated foliage by mesh/material. Bevy performs the matching
// GPU instancing automatically; stable spatial groups make that contract
// auditable without duplicating source geometry or changing generation.
const FOLIAGE_BATCH_CHUNK_CELLS: u16 = 32;
const CROWD_SEPARATION_RADIUS_CELLS: f32 = 0.42;
const CROWD_SEPARATION_MAX_CELLS: f32 = 0.28;
const CROWD_PREDICTION_RADIUS_CELLS: f32 = 0.46;
const CROWD_PREDICTION_HORIZON_SECONDS: f32 = 0.85;
const CROWD_MINIMUM_YIELD_SPEED: f32 = 0.18;
const CHARACTER_HIT_SECONDS: f32 = 0.25;
const TOWER_TRAIL_SECONDS: f32 = 2.0;
const TOWER_TRAIL_WIDTH: f32 = 0.1;
const FIREBALL_SIZE: f32 = 0.4;
const FIREBALL_TRAIL_SIZE: f32 = 0.3;
const BUILDING_HIT_SECONDS: f32 = 0.5;
const BUILDING_HIT_SMOKE_SPEED: f32 = 3.0;
const BUILDING_HIT_SPARK_SPEED: f32 = 12.0;
const SETTINGS_STREAMING_FIRST_INDEX: usize = 25;
const SETTINGS_STREAMING_LAST_INDEX: usize = 32;
const SETTINGS_APPLY_INDEX: usize = 33;
const SETTINGS_DEFAULTS_INDEX: usize = 34;
const SETTINGS_BACK_INDEX: usize = 35;
const SETTINGS_MENU_ITEM_COUNT: usize = 36;
const AMBIENCE_GAIN: f32 = 0.16;
const SEAGULL_GAIN: f32 = 0.28;
const SEAGULL_FLIGHT_SECONDS: f32 = 32.0;
const SEAGULL_HEIGHT: f32 = 15.0;
const SEAGULL_MAX_AUDIO_DISTANCE: f32 = 50.0;
const SEAGULL_MODEL_PATH: &str = "migrated/models/Models/Critters/Critter_Seagull_01.glb";
const RAIN_CAMERA_CULL_DISTANCE: f32 = 7.5;
const WORLD_UI_SAFE_TOP: f32 = 78.0;
const WORLD_UI_SAFE_BOTTOM: f32 = 0.0;
const WORLD_UI_OVERLAY_MARGIN: f32 = 36.0;
const UNITY_MAIN_UI_REFERENCE_HEIGHT: f32 = 2_160.0;
const UNITY_SETTINGS_UI_REFERENCE_HEIGHT: f32 = 1_080.0;
const MAIN_MENU_CLOUD_COLUMNS: usize = 11;
const MAIN_MENU_CLOUD_ROWS: usize = 5;
const PROCEDURAL_AUDIO_SAMPLE_RATE: u32 = 16_000;
const BUILDING_HIT_SMOKE_SIZE: f32 = 0.5;
const BUILDING_HIT_SPARK_SIZE: f32 = 0.25;
const BUILDING_LEVEL_UP_SECONDS: f32 = 1.5;
const BUILDING_LEVEL_UP_ARROW_SIZE: f32 = 0.5;
const BUILDING_LEVEL_UP_TILE_SIZE: f32 = 4.0;
const BUILDING_DAMAGED_RADIUS: f32 = 1.403_639_8;
const BUILDING_DAMAGED_FIRE_AMOUNT: u16 = 128;
const BUILDING_DAMAGED_SMOKE_AMOUNT: u16 = 200;
const AGENT_ROTATION_SPEED: f32 = 5.0;
const LOCOMOTION_REFERENCE_SPEED_CELLS_PER_SECOND: f32 = 5.0;
const LOCOMOTION_STOP_GRACE_SECONDS: f32 = 0.12;
const EYE_NODES: [&str; 10] = [
    "Eyes_Angry",
    "Eyes_Annoyed",
    "Eyes_Cool",
    "Eyes_Happy",
    "Eyes_MissingEye",
    "Eyes_Normal",
    "Eyes_Pain",
    "Eyes_Sad",
    "Eyes_Wink",
    "Eyes_Worried",
];
const HAIR_NODES: [&str; 7] = [
    "Hair_Long_Ponytail",
    "Hair_Long_Sidebraids",
    "Hair_Medium_Side",
    "Hair_Short_Bowlcut",
    "Hair_Short_Normal",
    "Hair_Short_Pushup",
    "Hair_Short_Sideswept",
];
const FACIAL_HAIR_NODES: [&str; 2] = ["FacialHair_Long_Beard", "FacialHair_Medium_Beard"];
const HAIR_COLORS: [[f32; 3]; 6] = [
    [0.019_607_844, 0.019_607_844, 0.019_607_844],
    [0.575_471_7, 0.176_441_8, 0.176_441_8],
    [0.839_622_6, 0.678_662_4, 0.099_012_084],
    [0.5, 0.287_401_7, 0.134_433_95],
    [0.127_714_48, 0.660_377_4, 0.216_843_2],
    [0.129_411_74, 0.599_905_3, 0.658_823_55],
];
const EYE_COLORS: [[f32; 3]; 5] = [
    [0.133_333_34, 0.098_039_22, 0.098_039_22],
    [0.191_883_25, 0.543_265_76, 0.830_188_7],
    [0.402_439_36, 0.726_415_1, 0.085_662_15],
    [0.743_859, 0.792_452_8, 0.790_356_93],
    [0.801_886_8, 0.693_535_1, 0.086_997_17],
];
const SKIN_COLORS: [[f32; 3]; 5] = [
    [0.850_980_4, 0.678_431_4, 0.509_803_95],
    [0.811_320_8, 0.697_127_04, 0.585_528_7],
    [0.556_603_8, 0.387_968_9, 0.223_166_61],
    [0.358_490_6, 0.222_511_4, 0.089_622_65],
    [0.850_980_4, 0.745_124_1, 0.509_803_95],
];
use twitch::{
    CredentialVault, DeviceAuthorization, OAuthClient, TokenValidation, TwitchControl, TwitchEvent,
    TwitchModerationStatus, TwitchStatus, TwitchTransport,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Boot,
    MainMenu,
    WorldLoading,
    InGame,
    Credits,
}

/// Present only after the recursive loading overlay and its dedicated camera
/// have actually left the ECS world. Systems in this set must not advance
/// gameplay, presentation clocks, or world audio without this marker.
#[derive(Resource)]
struct GameplayReady;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
struct GameplaySimulationSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
struct AgentSimulationSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
struct NightLightSyncSet;

const STREAM_ONLY_AGENT_SIMULATION_HZ: u32 = 60;
const STREAM_ONLY_NIGHT_LIGHT_SYNC_HZ: u32 = 30;

#[derive(Resource, Default)]
struct AgentSimulationCadence(AtomicU64);

impl AgentSimulationCadence {
    fn set_delta(&self, delta: Duration) {
        self.0.store(
            u64::try_from(delta.as_nanos()).unwrap_or(u64::MAX),
            AtomicOrdering::Relaxed,
        );
    }

    fn delta(&self) -> Duration {
        Duration::from_nanos(self.0.load(AtomicOrdering::Relaxed))
    }
}

#[derive(Default)]
struct AgentSimulationCadenceState {
    accumulated: Duration,
    delta: Duration,
}

#[derive(Resource)]
pub struct RuntimeConfig(pub GameConfig);

#[derive(Resource, Clone)]
pub struct RuntimePlayerSettings(pub PlayerSettings);

#[derive(Resource)]
struct PlayerSettingsRuntime {
    autosave_elapsed_seconds: f32,
}

#[derive(Resource)]
pub struct RuntimeContent(pub ContentCatalog);

#[derive(Resource)]
pub struct RuntimePresentation(pub PresentationCatalog);

#[derive(Resource)]
struct RuntimeAssetRoot(PathBuf);

#[derive(Resource)]
struct WorldRuntime {
    generated: GeneratedWorld,
    legacy_terrain_mesh: Option<SavedTerrainMesh>,
    legacy_migration: Option<LegacyMigrationMetadata>,
}

#[derive(Resource, Default)]
struct FineNavigationRuntime {
    grid: Option<stream_town_domain::NavGrid>,
    applied_signature: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorldDiagnosticMode {
    Pathfinding,
    Floorplan,
}

#[derive(Resource, Default)]
struct WorldDiagnosticRuntime {
    mode: Option<WorldDiagnosticMode>,
    remaining_seconds: f32,
    black_material: Option<Handle<StandardMaterial>>,
}

impl WorldDiagnosticRuntime {
    fn activate(&mut self, mode: WorldDiagnosticMode) {
        self.mode = Some(mode);
        self.remaining_seconds = WORLD_DIAGNOSTIC_VIEW_SECONDS;
    }

    fn refresh_if_active(&mut self) {
        if self.mode.is_some() {
            self.remaining_seconds = WORLD_DIAGNOSTIC_VIEW_SECONDS;
        }
    }
}

/// Sparse traffic history. A point is added only when a citizen completes a
/// move into a different grid cell, and is discarded once exponential decay
/// makes it visually irrelevant.
#[derive(Resource)]
struct TraversalWearRuntime {
    cells: HashMap<GridPos, TraversalWearCell>,
    decay_elapsed_seconds: f32,
    texture_dirty: bool,
}

#[derive(Resource, Default)]
struct PathSurfaceRuntime {
    applied_signature: u64,
    initialized: bool,
    levels: HashMap<GridPos, u16>,
}

#[derive(Clone, Copy, Debug, Default)]
struct TraversalWearCell {
    score: f32,
    decay_pause_seconds: f32,
}

impl Default for TraversalWearRuntime {
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
            decay_elapsed_seconds: 0.0,
            texture_dirty: true,
        }
    }
}

impl TraversalWearRuntime {
    fn record(
        &mut self,
        position: GridPos,
        settings: &stream_town_domain::TerrainAppearanceConfig,
    ) {
        let cell = self.cells.entry(position).or_default();
        cell.score = (cell.score + 1.0)
            .min(traversal_score_for_rate(
                settings.traversal_full_tint_per_minute,
                settings.traversal_half_life_seconds,
            ))
            .min(MAX_TRAVERSAL_WEAR_SCORE);
        cell.decay_pause_seconds = settings.traversal_decay_pause_seconds;
        self.texture_dirty = true;
    }

    fn decay(
        &mut self,
        delta_seconds: f32,
        settings: &stream_town_domain::TerrainAppearanceConfig,
    ) {
        self.decay_elapsed_seconds += delta_seconds.max(0.0);
        if self.decay_elapsed_seconds < TRAVERSAL_WEAR_DECAY_INTERVAL_SECONDS {
            return;
        }
        if self.cells.is_empty() {
            self.decay_elapsed_seconds = 0.0;
            return;
        }
        let elapsed = std::mem::take(&mut self.decay_elapsed_seconds);
        self.cells.retain(|_, cell| {
            let active_decay_seconds = (elapsed - cell.decay_pause_seconds).max(0.0);
            cell.decay_pause_seconds = (cell.decay_pause_seconds - elapsed).max(0.0);
            if active_decay_seconds > 0.0 {
                cell.score *=
                    0.5_f32.powf(active_decay_seconds / settings.traversal_half_life_seconds);
            }
            cell.score >= settings.traversal_prune_score
        });
        self.texture_dirty = true;
    }

    fn restore(
        &mut self,
        cells: &BTreeMap<GridPos, f32>,
        width: u16,
        height: u16,
        settings: &stream_town_domain::TerrainAppearanceConfig,
    ) {
        self.cells = cells
            .iter()
            .filter(|(position, score)| {
                position.x < width
                    && position.z < height
                    && score.is_finite()
                    && **score >= settings.traversal_prune_score
            })
            .map(|(position, score)| {
                (
                    *position,
                    TraversalWearCell {
                        score: score
                            .min(traversal_score_for_rate(
                                settings.traversal_full_tint_per_minute,
                                settings.traversal_half_life_seconds,
                            ))
                            .min(MAX_TRAVERSAL_WEAR_SCORE),
                        decay_pause_seconds: 0.0,
                    },
                )
            })
            .collect();
        self.decay_elapsed_seconds = 0.0;
        self.texture_dirty = true;
    }

    fn saved_cells(&self) -> BTreeMap<GridPos, f32> {
        self.cells
            .iter()
            .filter(|(_, cell)| cell.score.is_finite() && cell.score > 0.0)
            .map(|(position, cell)| (*position, cell.score.min(MAX_TRAVERSAL_WEAR_SCORE)))
            .collect()
    }
}

fn traversal_wear_fraction(
    score: f32,
    settings: &stream_town_domain::TerrainAppearanceConfig,
) -> f32 {
    let fade_start = traversal_score_for_rate(
        settings.traversal_fade_start_per_minute,
        settings.traversal_half_life_seconds,
    );
    let full_tint = traversal_score_for_rate(
        settings.traversal_full_tint_per_minute,
        settings.traversal_half_life_seconds,
    );
    ((score - fade_start) / (full_tint - fade_start)).clamp(0.0, 1.0)
}

/// Maps an author-facing sustained crossing rate to the equilibrium of the
/// exponentially decaying sparse score: rate / decay constant.
fn traversal_score_for_rate(rate_per_minute: f32, half_life_seconds: f32) -> f32 {
    rate_per_minute.max(0.0) * half_life_seconds.max(f32::EPSILON) / (60.0 * std::f32::consts::LN_2)
}

#[derive(Resource)]
struct SaveRuntime {
    store: NativeSaveStore,
}

/// Holds the startup resume request after process initialization. Keeping the
/// path in a resource makes the loading and broadcast gates independent of any
/// later environment changes, and `applied` prevents a failed resume from ever
/// exposing or streaming the freshly generated fallback world.
#[derive(Resource, Clone, Debug, Default)]
struct AutomaticResumeRuntime {
    path: Option<PathBuf>,
    applied: bool,
}

impl AutomaticResumeRuntime {
    fn new(path: Option<PathBuf>) -> Self {
        Self {
            path,
            applied: false,
        }
    }

    fn pending(&self) -> bool {
        self.path.is_some() && !self.applied
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TownSaveEntry {
    name: String,
    path: PathBuf,
    protected: bool,
}

#[derive(Resource)]
struct TownSaveCatalogRuntime {
    directory: PathBuf,
    legacy_path: PathBuf,
    fixed_path: Option<PathBuf>,
    active_town: Option<String>,
}

fn automatic_resume_save_path() -> Option<PathBuf> {
    std::env::var_os("STREAM_TOWN_AUTO_RESUME_PATH").map(PathBuf::from)
}

fn automatic_resume_world_seed(path: Option<&Path>) -> Option<u64> {
    let path = path?;
    NativeSaveStore::new(path)
        .load()
        .ok()
        .map(|snapshot| snapshot.world_seed)
}

fn startup_save_path(resume_path: Option<PathBuf>, fixed_path: Option<PathBuf>) -> PathBuf {
    resume_path
        .or(fixed_path)
        .unwrap_or_else(|| PathBuf::from(".stream-town").join("StreamTownSave.stbevy"))
}

impl TownSaveCatalogRuntime {
    fn from_startup_paths(fixed_path: Option<PathBuf>, resume_path: Option<PathBuf>) -> Self {
        let active_town = resume_path
            .as_deref()
            .or(fixed_path.as_deref())
            .and_then(|path| {
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(str::to_owned)
            });
        Self {
            directory: PathBuf::from(".stream-town").join("saves"),
            legacy_path: PathBuf::from(".stream-town").join("StreamTownSave.stbevy"),
            fixed_path,
            active_town,
        }
    }

    fn entries(&self) -> Vec<TownSaveEntry> {
        if let Some(path) = &self.fixed_path {
            return path
                .is_file()
                .then(|| town_save_entry(path))
                .into_iter()
                .collect();
        }
        let mut entries = fs::read_dir(&self.directory)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file()
                    && path
                        .extension()
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("stbevy"))
            })
            .map(|path| town_save_entry(&path))
            .collect::<Vec<_>>();
        if self.legacy_path.is_file() && !entries.iter().any(|entry| entry.path == self.legacy_path)
        {
            entries.push(TownSaveEntry {
                name: "Legacy Town".to_owned(),
                path: self.legacy_path.clone(),
                protected: false,
            });
        }
        entries.sort_by(|left, right| {
            left.name
                .to_ascii_lowercase()
                .cmp(&right.name.to_ascii_lowercase())
                .then_with(|| left.path.cmp(&right.path))
        });
        entries
    }

    fn new_town_path(&self, name: &str) -> PathBuf {
        self.fixed_path.clone().unwrap_or_else(|| {
            self.directory
                .join(format!("{}.stbevy", safe_town_filename(name)))
        })
    }
}

#[derive(Resource, Default)]
struct TownRestartRuntime {
    retained_players: Option<Vec<ActorState>>,
    suppress_exit_save: bool,
}

#[derive(SystemParam)]
struct LoadRenderParams<'w, 's> {
    presentation: Res<'w, RuntimePresentation>,
    render: Res<'w, RenderAssets>,
    meshes: Option<ResMut<'w, Assets<Mesh>>>,
    asset_server: Option<Res<'w, AssetServer>>,
    asset_root: Res<'w, RuntimeAssetRoot>,
    terrain_surfaces: Query<'w, 's, Entity, With<TerrainSurface>>,
    water_surfaces: Query<'w, 's, Entity, With<WaterSurface>>,
}

#[derive(SystemParam)]
struct LoadWorldEntities<'w, 's> {
    agents: Query<
        'w,
        's,
        (
            Entity,
            &'static mut Agent,
            &'static mut GridLocation,
            &'static AgentAnimation,
            &'static mut Transform,
        ),
        Without<TownHall>,
    >,
    runtime_buildings: Query<'w, 's, (Entity, &'static RuntimeBuilding), Without<TownHall>>,
    town_halls: Query<
        'w,
        's,
        (
            &'static mut GridLocation,
            &'static mut BuildingPresentation,
            &'static mut Transform,
        ),
        With<TownHall>,
    >,
    enemy_camps: Query<'w, 's, (Entity, &'static EnemyCamp)>,
    surface_visuals: Query<'w, 's, Entity, With<RuntimeTerrainGrounding>>,
    world_fish_schools: Query<'w, 's, Entity, (With<FishSchoolParticle>, With<WorldEntity>)>,
}

#[derive(Resource)]
struct SimulationRuntime(WorldSimulation);

#[derive(Resource, Default)]
struct NightEnemyWaveRuntime {
    night_day: Option<u32>,
    remaining_seconds: f64,
    wave_index: u32,
    pending_spawns: VecDeque<PendingEnemySpawn>,
    spawn_interval_seconds: f64,
    spawn_remaining_seconds: f64,
}

#[derive(Clone, Debug)]
struct PendingEnemySpawn {
    camp: StableId,
    archetype: StableId,
    serial: u64,
    final_raid_wave: bool,
}

#[derive(Clone, Debug)]
struct EnemyRouteBuilding {
    id: StableId,
    town_hall_distance: u64,
    approaches: Vec<GridPos>,
}

#[derive(Clone, Debug)]
struct EnemyNavigationField {
    signature: u64,
    width: u16,
    component_by_cell: Vec<u32>,
    target_by_component: Vec<Option<StableId>>,
    next_by_cell: Vec<Option<GridPos>>,
    goal_by_cell: Vec<Option<GridPos>>,
    cluster_edges: BTreeMap<EnemyClusterNode, Vec<EnemyClusterNode>>,
}

// Preserve the original eight-town-cell HPA* cluster size on the navigation-only
// grid, which has three samples per town cell.
const ENEMY_NAVIGATION_CLUSTER_SIZE: u16 = 8 * NAVIGATION_SUBDIVISIONS;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct EnemyClusterNode {
    x: u16,
    z: u16,
    component: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EnemyPathOpenNode {
    position: GridPos,
    estimated_total: u32,
    cost: u32,
}

impl Ord for EnemyPathOpenNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .estimated_total
            .cmp(&self.estimated_total)
            .then_with(|| other.cost.cmp(&self.cost))
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for EnemyPathOpenNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl EnemyNavigationField {
    fn index(&self, position: GridPos) -> Option<usize> {
        (position.x < self.width)
            .then(|| usize::from(position.z) * usize::from(self.width) + usize::from(position.x))
            .filter(|index| *index < self.component_by_cell.len())
    }

    fn destination(&self, start: GridPos) -> Option<(&StableId, GridPos)> {
        let index = self.index(start)?;
        let component = *self.component_by_cell.get(index)?;
        let component = usize::try_from(component).ok()?;
        Some((
            self.target_by_component.get(component)?.as_ref()?,
            self.goal_by_cell.get(index).copied().flatten()?,
        ))
    }

    fn component_at(&self, position: GridPos) -> Option<u32> {
        let component = *self.component_by_cell.get(self.index(position)?)?;
        (component != u32::MAX).then_some(component)
    }

    fn cluster_node(&self, position: GridPos) -> Option<EnemyClusterNode> {
        Some(EnemyClusterNode {
            x: position.x / ENEMY_NAVIGATION_CLUSTER_SIZE,
            z: position.z / ENEMY_NAVIGATION_CLUSTER_SIZE,
            component: self.component_at(position)?,
        })
    }

    fn cluster_path(&self, start: GridPos, goal: GridPos) -> Option<Vec<EnemyClusterNode>> {
        let start = self.cluster_node(start)?;
        let goal = self.cluster_node(goal)?;
        if start.component != goal.component {
            return None;
        }
        if start == goal {
            return Some(vec![start]);
        }
        let mut frontier = VecDeque::from([start]);
        let mut previous = BTreeMap::<EnemyClusterNode, EnemyClusterNode>::new();
        let mut visited = BTreeSet::from([start]);
        while let Some(current) = frontier.pop_front() {
            for neighbour in self.cluster_edges.get(&current).into_iter().flatten() {
                if !visited.insert(*neighbour) {
                    continue;
                }
                previous.insert(*neighbour, current);
                if *neighbour == goal {
                    let mut route = vec![goal];
                    let mut cursor = goal;
                    while cursor != start {
                        cursor = previous[&cursor];
                        route.push(cursor);
                    }
                    route.reverse();
                    return Some(route);
                }
                frontier.push_back(*neighbour);
            }
        }
        None
    }

    fn hierarchical_path(
        &self,
        navigation: &stream_town_domain::NavGrid,
        start: GridPos,
        goal: GridPos,
    ) -> Option<Vec<GridPos>> {
        let cluster_route = self.cluster_path(start, goal)?;
        let component = self.component_at(start)?;
        let cluster_width = self.width.div_ceil(ENEMY_NAVIGATION_CLUSTER_SIZE);
        let height = u16::try_from(self.component_by_cell.len() / usize::from(self.width)).ok()?;
        let cluster_height = height.div_ceil(ENEMY_NAVIGATION_CLUSTER_SIZE);
        let mut corridor = BTreeSet::new();
        for cluster in cluster_route {
            for dz in -1_i32..=1 {
                for dx in -1_i32..=1 {
                    let x = i32::from(cluster.x) + dx;
                    let z = i32::from(cluster.z) + dz;
                    if x >= 0
                        && z >= 0
                        && x < i32::from(cluster_width)
                        && z < i32::from(cluster_height)
                    {
                        corridor.insert((u16::try_from(x).ok()?, u16::try_from(z).ok()?));
                    }
                }
            }
        }
        self.path_within_cluster_corridor(navigation, start, goal, component, &corridor)
    }

    fn path_within_cluster_corridor(
        &self,
        navigation: &stream_town_domain::NavGrid,
        start: GridPos,
        goal: GridPos,
        component: u32,
        corridor: &BTreeSet<(u16, u16)>,
    ) -> Option<Vec<GridPos>> {
        if start == goal {
            return Some(vec![start]);
        }
        let allowed = |position: GridPos| {
            self.component_at(position) == Some(component)
                && corridor.contains(&(
                    position.x / ENEMY_NAVIGATION_CLUSTER_SIZE,
                    position.z / ENEMY_NAVIGATION_CLUSTER_SIZE,
                ))
        };
        if !allowed(start) || !allowed(goal) {
            return None;
        }
        let mut open = BinaryHeap::from([EnemyPathOpenNode {
            position: start,
            estimated_total: grid_octile_distance(start, goal),
            cost: 0,
        }]);
        let mut previous = HashMap::<GridPos, GridPos>::new();
        let mut costs = HashMap::<GridPos, u32>::from([(start, 0)]);
        while let Some(current) = open.pop() {
            if current.position == goal {
                let mut route = vec![goal];
                let mut cursor = goal;
                while cursor != start {
                    cursor = previous[&cursor];
                    route.push(cursor);
                }
                route.reverse();
                return Some(route);
            }
            if current.cost > costs[&current.position] {
                continue;
            }
            for (neighbour, step_cost) in navigation
                .walkable_neighbours(current.position)
                .into_iter()
                .flatten()
                .filter(|(position, _)| allowed(*position))
            {
                let next_cost = current.cost.saturating_add(step_cost);
                if next_cost < *costs.get(&neighbour).unwrap_or(&u32::MAX) {
                    costs.insert(neighbour, next_cost);
                    previous.insert(neighbour, current.position);
                    open.push(EnemyPathOpenNode {
                        position: neighbour,
                        estimated_total: next_cost
                            .saturating_add(grid_octile_distance(neighbour, goal)),
                        cost: next_cost,
                    });
                }
            }
        }
        None
    }

    fn path_to(&self, start: GridPos, building: &StableId, goal: GridPos) -> Option<Vec<GridPos>> {
        let (field_building, field_goal) = self.destination(start)?;
        if field_building != building || field_goal != goal {
            return None;
        }
        let mut path = vec![start];
        let mut current = start;
        for _ in 0..self.component_by_cell.len() {
            if current == goal {
                return Some(path);
            }
            let next = self.next_by_cell.get(self.index(current)?)?.as_ref()?;
            if *next == current {
                return None;
            }
            current = *next;
            path.push(current);
        }
        None
    }
}

struct EnemyNavigationTask {
    signature: u64,
    task: Task<EnemyNavigationField>,
}

#[derive(Resource, Default)]
struct EnemyNavigationRuntime {
    field: Option<EnemyNavigationField>,
    task: Option<EnemyNavigationTask>,
}

#[derive(Resource, Default)]
struct SessionStats {
    elapsed_seconds: f64,
    paths_completed: u64,
    commands_processed: u64,
}

#[derive(SystemParam)]
struct MovementStats<'w, 's> {
    session: ResMut<'w, SessionStats>,
    traversal_wear: ResMut<'w, TraversalWearRuntime>,
    path_surfaces: Res<'w, PathSurfaceRuntime>,
    fine_navigation: Res<'w, FineNavigationRuntime>,
    passive_income_elapsed: Local<'s, Duration>,
}

fn advance_agent_simulation_cadence(
    cadence: &mut AgentSimulationCadenceState,
    frame_delta: Duration,
    stream_only: bool,
    active: bool,
) -> bool {
    advance_stream_only_cadence(
        cadence,
        frame_delta,
        stream_only,
        active,
        STREAM_ONLY_AGENT_SIMULATION_HZ,
    )
}

fn advance_stream_only_cadence(
    cadence: &mut AgentSimulationCadenceState,
    frame_delta: Duration,
    stream_only: bool,
    active: bool,
    frequency_hz: u32,
) -> bool {
    if !active {
        *cadence = AgentSimulationCadenceState::default();
        return false;
    }
    if !stream_only {
        cadence.accumulated = Duration::ZERO;
        cadence.delta = frame_delta;
        return true;
    }

    cadence.accumulated = cadence.accumulated.saturating_add(frame_delta);
    let interval = Duration::from_secs_f64(1.0 / f64::from(frequency_hz.max(1)));
    if cadence.accumulated < interval {
        return false;
    }
    cadence.delta = std::mem::take(&mut cadence.accumulated);
    true
}

fn night_light_sync_due(
    time: Res<Time>,
    state: Res<State<GameState>>,
    gameplay_ready: Option<Res<GameplayReady>>,
    config: Res<RuntimeConfig>,
    mut local: Local<AgentSimulationCadenceState>,
) -> bool {
    advance_stream_only_cadence(
        &mut local,
        time.delta(),
        config.0.twitch.broadcast.render_mode == BroadcastRenderMode::StreamOnly,
        *state.get() == GameState::InGame && gameplay_ready.is_some(),
        STREAM_ONLY_NIGHT_LIGHT_SYNC_HZ,
    )
}

fn agent_simulation_due(
    time: Res<Time>,
    state: Res<State<GameState>>,
    gameplay_ready: Option<Res<GameplayReady>>,
    config: Res<RuntimeConfig>,
    cadence: Res<AgentSimulationCadence>,
    mut local: Local<AgentSimulationCadenceState>,
) -> bool {
    let due = advance_agent_simulation_cadence(
        &mut local,
        time.delta(),
        config.0.twitch.broadcast.render_mode == BroadcastRenderMode::StreamOnly,
        *state.get() == GameState::InGame && gameplay_ready.is_some(),
    );
    if due {
        cadence.set_delta(local.delta);
    }
    due
}

#[derive(Resource, Default)]
struct WorldRenderStats {
    terrain_high_chunks: usize,
    terrain_medium_chunks: usize,
    terrain_low_chunks: usize,
    foliage_instances: usize,
    foliage_visible_instances: usize,
    foliage_batches: usize,
    foliage_spatial_groups: usize,
    foliage_unbatched_instances: usize,
    crowd_adjusted_agents: usize,
    crowd_yielding_agents: usize,
}

#[derive(Resource, Default)]
struct CrowdSeparationRuntime {
    applied_offsets: HashMap<Entity, Vec3>,
}

#[derive(Clone, Debug, Default)]
struct CachedStationTargets {
    refresh_elapsed_milliseconds: f64,
    target_check_elapsed_milliseconds: f64,
    targets: BTreeMap<StableId, Vec<StableId>>,
    reachability_queue: VecDeque<(StableId, StableId)>,
}

#[derive(Resource, Default)]
struct StationTargetRuntime {
    stations: BTreeMap<StableId, CachedStationTargets>,
    refresh_queue: VecDeque<StableId>,
}

#[derive(Resource, Default)]
struct StationResourceTargetIndex {
    resource_count: usize,
    first_resource: Option<StableId>,
    last_resource: Option<StableId>,
    by_id: HashMap<StableId, usize>,
    by_kind: BTreeMap<StableId, Vec<usize>>,
}

impl StationResourceTargetIndex {
    fn sync(&mut self, world: &GeneratedWorld) {
        let first_resource = world.resources.first().map(|resource| resource.id.clone());
        let last_resource = world.resources.last().map(|resource| resource.id.clone());
        if self.resource_count == world.resources.len()
            && self.first_resource == first_resource
            && self.last_resource == last_resource
        {
            return;
        }
        self.resource_count = world.resources.len();
        self.first_resource = first_resource;
        self.last_resource = last_resource;
        self.by_id.clear();
        self.by_kind.clear();
        self.by_id.reserve(world.resources.len());
        for (index, resource) in world.resources.iter().enumerate() {
            self.by_id.insert(resource.id.clone(), index);
            self.by_kind
                .entry(resource.target_kind.clone())
                .or_default()
                .push(index);
        }
    }
}

#[derive(Clone, Debug)]
struct PendingChatCommand {
    actor_id: StableId,
    login_name: String,
    display_name: String,
    command: ChatCommand,
    is_broadcaster: bool,
    is_moderator: bool,
    is_subscriber: bool,
    origin: CommandOrigin,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CommandOrigin {
    Twitch,
    /// Local tool/keyboard injection mirrors Unity's debug bridge permission bypass.
    LocalDebug,
}

#[derive(Resource, Default)]
struct InjectedCommands(VecDeque<PendingChatCommand>);

#[derive(Resource, Default)]
struct CommandFeedback(String);

#[derive(Resource, Default)]
struct CommandAcknowledgementRuntime {
    sequence: u64,
}

impl CommandAcknowledgementRuntime {
    fn acknowledge(&mut self) {
        self.sequence = self.sequence.saturating_add(1);
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum MenuPage {
    #[default]
    Closed,
    Game,
    GoLiveConfirmation,
    NewTown,
    LoadTown,
    Settings,
    SecretsDisclaimer,
    Secrets,
}

#[derive(Resource, Default)]
pub(crate) struct SensitiveScreenActive(pub bool);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub(crate) struct SensitiveScreenUpdateSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SecretsAuthorizationKind {
    Bot,
    Broadcaster,
}

impl SecretsAuthorizationKind {
    const fn label(self) -> &'static str {
        match self {
            Self::Bot => "bot",
            Self::Broadcaster => "broadcaster",
        }
    }
}

enum SecretsAuthorizationEvent {
    Device {
        kind: SecretsAuthorizationKind,
        authorization: DeviceAuthorization,
    },
    Authorized {
        kind: SecretsAuthorizationKind,
        validation: TokenValidation,
    },
    Error {
        kind: SecretsAuthorizationKind,
        message: String,
    },
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
enum SecretsCredentialState {
    #[default]
    Unknown,
    NotConfigured,
    Missing,
    Stored,
    Error(String),
}

#[derive(Resource, Default)]
struct SecretsRuntime {
    authorization_events: Option<Arc<Mutex<mpsc::Receiver<SecretsAuthorizationEvent>>>>,
    active_authorization: Option<SecretsAuthorizationKind>,
    device: Option<DeviceAuthorization>,
    feedback: String,
    credential_signature: Option<(String, String, String)>,
    bot_credential: SecretsCredentialState,
    broadcaster_credential: SecretsCredentialState,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum SettingsTab {
    #[default]
    Video,
    Audio,
    Gameplay,
    Accessibility,
    Streaming,
    Connection,
}

#[derive(Resource)]
struct MenuRuntime {
    page: MenuPage,
    return_page: MenuPage,
    selected: usize,
    settings_tab: SettingsTab,
    confirm_settings_close: bool,
    draft: PlayerSettings,
    streaming_draft: BroadcastConfig,
    feedback: String,
    pending_town_start: Option<PendingTownStart>,
}

impl Default for MenuRuntime {
    fn default() -> Self {
        Self {
            page: MenuPage::Closed,
            return_page: MenuPage::Closed,
            selected: 0,
            settings_tab: SettingsTab::Video,
            confirm_settings_close: false,
            draft: PlayerSettings::default(),
            streaming_draft: BroadcastConfig::default(),
            feedback: String::new(),
            pending_town_start: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum PendingTownStart {
    NewGame,
    LoadGame { source: PathBuf },
}

#[derive(Resource, Default)]
struct SettingsUiCache {
    signature: String,
}

#[derive(Resource, Default)]
struct AccessibilityRuntime {
    synthetic_pressed: Option<Entity>,
    last_announcement: String,
}

#[derive(Component)]
struct AccessibilityAnnouncement;

#[derive(Component)]
struct AccessibilityHighContrastText;

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum AccessibleButtonScope {
    MainMenu,
    GoLiveConfirmation,
    GameMenu,
    Settings,
    SettingsConfirm,
    Credits,
}

#[derive(Resource, Clone, Copy)]
struct AccessibilityMotionDefaults {
    tree: Vec4,
    grass: Vec4,
    water: Vec4,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
struct AccessibilityActionDispatch;

#[derive(Resource, Default)]
struct MenuIoRequest {
    save: bool,
    save_jump_start: bool,
    load: bool,
    load_source: Option<PathBuf>,
}

#[derive(Resource)]
struct RuntimeConsoleRuntime {
    store: RuntimeConsoleStore,
    enabled: bool,
    last_processed_sequence: u64,
    last_result: String,
    status_elapsed_seconds: f32,
}

impl Default for RuntimeConsoleRuntime {
    fn default() -> Self {
        let store = RuntimeConsoleStore::from_environment();
        let last_processed_sequence = store
            .read_request()
            .ok()
            .flatten()
            .map_or(0, |request| request.sequence);
        Self {
            store,
            enabled: std::env::var_os("STREAM_TOWN_RUNTIME_CONSOLE_DIR").is_some()
                || std::env::var_os("STREAM_TOWN_RUNTIME_CONSOLE").is_some(),
            last_processed_sequence,
            last_result: "Runtime console ready".to_owned(),
            status_elapsed_seconds: 0.0,
        }
    }
}

#[derive(Resource, Default)]
struct RuntimeCaptureRequest(bool);

#[derive(Resource, Default)]
struct CameraCommandQueue(VecDeque<CameraRequest>);

#[derive(Resource, Default)]
struct AgentCommandQueue(VecDeque<AgentCommand>);

#[derive(Resource, Default)]
struct BuildingCommandQueue(VecDeque<BuildingRuntimeCommand>);

#[derive(Clone, Debug)]
enum BuildingRuntimeCommand {
    Despawn(StableId),
}

#[derive(Clone, Debug)]
struct BuildingPlacement {
    building: StableId,
    /// `!build thickpath` still constructs ordinary path entities, but previews
    /// and independently attempts one neighbouring fine cell on each side.
    thick_path: bool,
    /// Coarse placement cell used by all authored buildings and retained as the
    /// containing cell for fine-grid paths.
    position: GridPos,
    /// Paths alone are authored directly on the one-third-cell navigation grid.
    navigation_position: Option<GridPos>,
    rotation_quarter_turns: i32,
    line_start: Option<GridPos>,
    line_end: Option<GridPos>,
    /// Ordered fine-grid cells traced after `!beginplace`, allowing paths to
    /// bend through narrow spaces instead of being constrained to one line.
    path_cells: Vec<GridPos>,
    inactivity_seconds: f32,
}

#[derive(Clone, Copy, Debug)]
struct MovementProgress {
    last_position: Vec2,
    last_path_index: usize,
    target: GridPos,
    stalled_seconds: f32,
}

#[derive(Clone, Copy, Debug, Default)]
struct NavigationFailureState {
    path_failure_seconds: f32,
    movement: Option<MovementProgress>,
}

#[derive(Resource, Default)]
struct PathFailureRuntime(BTreeMap<StableId, NavigationFailureState>);

impl PathFailureRuntime {
    fn record_failure(&mut self, actor: &StableId, delta_seconds: f32) -> bool {
        let state = self.0.entry(actor.clone()).or_default();
        state.path_failure_seconds += delta_seconds.max(0.0);
        state.path_failure_seconds >= 5.0
    }

    fn clear_path_failure(&mut self, actor: &StableId) {
        if let Some(state) = self.0.get_mut(actor) {
            state.path_failure_seconds = 0.0;
        }
    }

    fn record_movement(
        &mut self,
        actor: &StableId,
        position: Vec2,
        path_index: usize,
        target: GridPos,
        delta_seconds: f32,
        minimum_progress: f32,
    ) -> bool {
        let state = self.0.entry(actor.clone()).or_default();
        let Some(progress) = state.movement.as_mut() else {
            state.movement = Some(MovementProgress {
                last_position: position,
                last_path_index: path_index,
                target,
                stalled_seconds: 0.0,
            });
            return false;
        };
        if progress.target != target
            || progress.last_path_index != path_index
            || progress.last_position.distance_squared(position)
                >= minimum_progress * minimum_progress
        {
            *progress = MovementProgress {
                last_position: position,
                last_path_index: path_index,
                target,
                stalled_seconds: 0.0,
            };
            return false;
        }
        progress.stalled_seconds += delta_seconds.max(0.0);
        progress.stalled_seconds >= 5.0
    }

    fn clear_movement(&mut self, actor: &StableId) {
        if let Some(state) = self.0.get_mut(actor) {
            state.movement = None;
        }
    }

    fn clear(&mut self, actor: &StableId) {
        self.0.remove(actor);
    }
}

#[derive(Resource, Default)]
struct SurfaceErrorRuntime {
    last_error: Option<Instant>,
    consecutive_errors: u8,
}

#[derive(Resource, Default)]
struct BuildingPlacers(BTreeMap<StableId, BuildingPlacement>);

fn expire_inactive_building_placements(time: Res<Time>, mut placers: ResMut<BuildingPlacers>) {
    let delta_seconds = time.delta_secs();
    placers
        .0
        .retain(|_, placement| building_placement_remains_active(placement, delta_seconds));
}

fn building_placement_remains_active(
    placement: &mut BuildingPlacement,
    delta_seconds: f32,
) -> bool {
    placement.inactivity_seconds += delta_seconds.max(0.0);
    placement.inactivity_seconds < BUILDING_PLACEMENT_TIMEOUT_SECONDS
}

#[derive(Clone, Debug, Default)]
struct RegenerationWorkerState {
    initialized: bool,
    next_ready_seconds: f64,
    station_visit_required: bool,
    last_station: Option<StableId>,
    prospector_step: u32,
    planting_sequence: u32,
    planting_target: Option<GridPos>,
    resource_saturation_target: Option<StableId>,
    resource_saturation_debuff: u32,
}

#[derive(Clone, Copy, Debug)]
struct RecentTreePlanting {
    nursery: GridPos,
    position: GridPos,
    planted_at_seconds: f64,
}

#[derive(Resource, Default)]
struct RegenerationRoleRuntime {
    elapsed_seconds: f64,
    next_resource_serial: u64,
    workers: BTreeMap<StableId, RegenerationWorkerState>,
    recently_fallen_trees: VecDeque<GridPos>,
    recent_tree_plantings: VecDeque<RecentTreePlanting>,
}

#[derive(SystemParam)]
struct RuntimeCommandQueues<'w> {
    injected: ResMut<'w, InjectedCommands>,
    camera: ResMut<'w, CameraCommandQueue>,
    agent: ResMut<'w, AgentCommandQueue>,
    building: ResMut<'w, BuildingCommandQueue>,
    placers: ResMut<'w, BuildingPlacers>,
}

#[derive(SystemParam)]
struct CommandResponseRuntime<'w> {
    connection: Res<'w, TwitchConnection>,
    feedback: ResMut<'w, CommandFeedback>,
    acknowledgements: ResMut<'w, CommandAcknowledgementRuntime>,
}

#[derive(SystemParam)]
struct CommandSaveRuntime<'w> {
    save: Res<'w, SaveRuntime>,
    traversal_wear: Res<'w, TraversalWearRuntime>,
}

#[derive(SystemParam)]
struct CommandWorldViewRuntime<'w, 's> {
    selected: Res<'w, SelectedCell>,
    cameras: Query<
        'w,
        's,
        (
            &'static Camera,
            &'static GlobalTransform,
            &'static TownCameraControllerRuntime,
        ),
        With<TownCamera>,
    >,
    spatial: Option<SpatialQuery<'w, 's>>,
    fine_navigation: Res<'w, FineNavigationRuntime>,
    diagnostic: ResMut<'w, WorldDiagnosticRuntime>,
}

#[derive(Clone, Debug)]
struct CameraRequest {
    reset: bool,
    actions: Vec<CameraAction>,
    follow: Option<StableId>,
    focus_building: Option<StableId>,
}

#[derive(Clone, Debug)]
enum AgentCommand {
    Teleport { actor: StableId, position: GridPos },
    Ping(StableId),
    Despawn(StableId),
}

#[derive(Resource)]
struct TwitchConnection {
    transport: Option<TwitchTransport>,
    status: TwitchStatus,
    moderation_status: TwitchModerationStatus,
    fish_god_reward_id: Option<String>,
}

const OPERATOR_CHAT_HISTORY_CAPACITY: usize = 200;

#[derive(Clone, Debug, Eq, PartialEq)]
struct OperatorChatBadges {
    broadcaster: bool,
    moderator: bool,
    subscriber: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OperatorChatLine {
    line_id: u64,
    user_id: String,
    login: String,
    display_name: String,
    message: String,
    badges: OperatorChatBadges,
    is_system: bool,
}

#[derive(Resource, Default)]
struct OperatorChatRuntime {
    lines: VecDeque<OperatorChatLine>,
    seen_message_ids: VecDeque<String>,
    next_line_id: u64,
    selected_line: Option<u64>,
    scroll_from_latest: usize,
    draft: String,
    input_focused: bool,
    selected_user: Option<(String, String)>,
    feedback: String,
}

impl OperatorChatRuntime {
    fn push(&mut self, mut line: OperatorChatLine) {
        self.next_line_id = self.next_line_id.wrapping_add(1).max(1);
        line.line_id = self.next_line_id;
        if self.scroll_from_latest > 0 {
            self.scroll_from_latest = self.scroll_from_latest.saturating_add(1);
        }
        self.lines.push_back(line);
        while self.lines.len() > OPERATOR_CHAT_HISTORY_CAPACITY {
            if let Some(removed) = self.lines.pop_front()
                && self.selected_line == Some(removed.line_id)
            {
                self.selected_line = None;
                self.selected_user = None;
            }
        }
    }

    fn push_chat(&mut self, message: &twitch::TwitchChatEnvelope) -> bool {
        if let Some(message_id) = message.message_id.as_ref() {
            if self.seen_message_ids.contains(message_id) {
                return false;
            }
            self.seen_message_ids.push_back(message_id.clone());
            while self.seen_message_ids.len() > OPERATOR_CHAT_HISTORY_CAPACITY {
                self.seen_message_ids.pop_front();
            }
        }
        self.push(OperatorChatLine {
            line_id: 0,
            user_id: message.user_id.clone(),
            login: message.login.clone(),
            display_name: message.display_name.clone(),
            message: message.message.clone(),
            badges: OperatorChatBadges {
                broadcaster: message.is_broadcaster,
                moderator: message.is_moderator,
                subscriber: message.is_subscriber,
            },
            is_system: false,
        });
        true
    }

    fn maximum_scroll(&self, visible_rows: usize) -> usize {
        self.lines.len().saturating_sub(visible_rows)
    }

    fn scroll_older(&mut self, rows: usize, visible_rows: usize) {
        self.scroll_from_latest = self
            .scroll_from_latest
            .saturating_add(rows)
            .min(self.maximum_scroll(visible_rows));
    }

    fn scroll_newer(&mut self, rows: usize) {
        self.scroll_from_latest = self.scroll_from_latest.saturating_sub(rows);
    }

    fn visible_lines(&self, visible_rows: usize) -> Vec<&OperatorChatLine> {
        let end = self.lines.len().saturating_sub(
            self.scroll_from_latest
                .min(self.maximum_scroll(visible_rows)),
        );
        let start = end.saturating_sub(visible_rows);
        self.lines.iter().skip(start).take(end - start).collect()
    }

    fn push_system(&mut self, message: impl Into<String>) {
        self.push(OperatorChatLine {
            line_id: 0,
            user_id: String::new(),
            login: "stream_town".to_owned(),
            display_name: "Stream Town".to_owned(),
            message: message.into(),
            badges: OperatorChatBadges {
                broadcaster: false,
                moderator: false,
                subscriber: false,
            },
            is_system: true,
        });
    }
}

impl Default for TwitchConnection {
    fn default() -> Self {
        Self {
            transport: None,
            status: TwitchStatus::Disabled,
            moderation_status: TwitchModerationStatus::Disabled,
            fish_god_reward_id: None,
        }
    }
}

#[derive(Resource, Default)]
struct SelectedCell(Option<GridPos>);

#[derive(Resource, Default)]
struct SelectedActor(Option<StableId>);

/// Opt-in marker for the retained pointer-to-object resolver. Shipping gameplay
/// never inserts this resource: text commands own interaction, while the
/// selection model stays available for future automatic camera targeting.
#[derive(Resource)]
struct PointerObjectSelectionEnabled;

#[derive(Resource, Default)]
struct EnvironmentPresentation {
    environment: Option<(Season, Weather)>,
    daylight_bits: Option<u32>,
    season_blend_bits: Option<u32>,
}

#[derive(Resource, Default)]
struct PostProcessPresentation {
    applied: Option<(GameState, u32, i32, i32)>,
}

#[derive(Resource, Default)]
struct BuildingMaterialInstances(BTreeMap<StableId, BuildingMaterialInstance>);

#[derive(Resource, Default)]
struct BuildingMaterialUpdateRuntime {
    order: Vec<StableId>,
    next_index: usize,
}

fn round_robin_indices(total: usize, start: usize) -> impl Iterator<Item = usize> {
    (0..total).map(move |offset| (start + offset) % total)
}

struct BuildingMaterialInstance {
    handle: Handle<BuildingMaterial>,
    applied_health: i32,
    applied_season: Season,
    applied_season_blend_bits: u32,
    applied_time_cycle: BuildingTimeCycleSignature,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BuildingTimeCycleSignature {
    seconds_per_day: u32,
    daylight_per_thousand: u16,
    transition_seconds: u32,
    max_building_emission_milli: u16,
}

impl From<&stream_town_domain::TimeCycleConfig> for BuildingTimeCycleSignature {
    fn from(config: &stream_town_domain::TimeCycleConfig) -> Self {
        Self {
            seconds_per_day: config.seconds_per_day,
            daylight_per_thousand: config.daylight_per_thousand,
            transition_seconds: config.transition_seconds,
            max_building_emission_milli: config.max_building_emission_milli,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Reflect, ShaderType)]
struct TerrainMaterialUniform {
    sand_color_a: Vec4,
    sand_color_b: Vec4,
    grass_color_a: Vec4,
    grass_color_b: Vec4,
    season_tint: Vec4,
    texture_uv_blend_tint: Vec4,
    grid_scale_offset: Vec4,
    selection_center_extent: Vec4,
    selection_color: Vec4,
    traversal_grid: Vec4,
    path_grid: Vec4,
    traversal_dirt_color: Vec4,
    constructed_path_color: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct TerrainMaterialExtension {
    #[uniform(100)]
    parameters: TerrainMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    grid_texture: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    traversal_wear_texture: Option<Handle<Image>>,
    #[texture(105)]
    #[sampler(106)]
    path_surface_texture: Option<Handle<Image>>,
}

impl MaterialExtension for TerrainMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        TERRAIN_SHADER_ASSET_PATH.into()
    }
}

type TerrainMaterial = ExtendedMaterial<StandardMaterial, TerrainMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct WaterMaterialUniform {
    surface_color: Vec4,
    deep_color: Vec4,
    foam_color: Vec4,
    ice_color: Vec4,
    wind_speed_noise_alpha: Vec4,
    scale_foam_ice: Vec4,
    season_tint: Vec4,
    main_scale_offset: Vec4,
    noise_scale_offset: Vec4,
    depth_foam_controls: Vec4,
    opacity_controls: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct WaterMaterialExtension {
    #[uniform(100)]
    parameters: WaterMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    main_texture: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    noise_texture: Option<Handle<Image>>,
}

impl MaterialExtension for WaterMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        WATER_SHADER_ASSET_PATH.into()
    }
}

type WaterMaterial = ExtendedMaterial<StandardMaterial, WaterMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct BuildingMaterialUniform {
    detail_color: Vec4,
    emissive_color: Vec4,
    ambient_occlusion: Vec4,
    surface_controls: Vec4,
    snow_damage: Vec4,
    main_scale_offset: Vec4,
    tint_color_strength: Vec4,
    time_cycle: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct BuildingMaterialExtension {
    #[uniform(100)]
    parameters: BuildingMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    main_texture: Option<Handle<Image>>,
}

impl MaterialExtension for BuildingMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        BUILDING_SHADER_ASSET_PATH.into()
    }
}

type BuildingMaterial = ExtendedMaterial<StandardMaterial, BuildingMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct CloudMaterialUniform {
    noise_controls: Vec4,
    surface_transform: Vec4,
    filter_controls: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct CloudMaterialExtension {
    #[uniform(100)]
    parameters: CloudMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    noise_texture: Option<Handle<Image>>,
}

impl MaterialExtension for CloudMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        CLOUD_SHADER_ASSET_PATH.into()
    }
}

type CloudMaterial = ExtendedMaterial<StandardMaterial, CloudMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct MenuSkyMaterialUniform {
    horizon_color: Vec4,
    zenith_color: Vec4,
    haze_color: Vec4,
    sun_direction_strength: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct MenuSkyMaterialExtension {
    #[uniform(100)]
    parameters: MenuSkyMaterialUniform,
}

impl MaterialExtension for MenuSkyMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        MENU_SKY_SHADER_ASSET_PATH.into()
    }
}

type MenuSkyMaterial = ExtendedMaterial<StandardMaterial, MenuSkyMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct GodrayMaterialUniform {
    emission_alpha: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct GodrayMaterialExtension {
    #[uniform(100)]
    parameters: GodrayMaterialUniform,
}

impl MaterialExtension for GodrayMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        GODRAY_SHADER_ASSET_PATH.into()
    }
}

type GodrayMaterial = ExtendedMaterial<StandardMaterial, GodrayMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct GiraffeMaterialUniform {
    animation_controls: Vec4,
    mask_controls: Vec4,
    rotation_controls: Vec4,
    main_scale_offset: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct GiraffeMaterialExtension {
    #[uniform(100)]
    parameters: GiraffeMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    main_texture: Option<Handle<Image>>,
}

impl MaterialExtension for GiraffeMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        GIRAFFE_SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        GIRAFFE_SHADER_ASSET_PATH.into()
    }
}

type GiraffeMaterial = ExtendedMaterial<StandardMaterial, GiraffeMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct BoundsMaterialUniform {
    color_alpha: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct BoundsMaterialExtension {
    #[uniform(100)]
    parameters: BoundsMaterialUniform,
}

impl MaterialExtension for BoundsMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        BOUNDS_SHADER_ASSET_PATH.into()
    }
}

type BoundsMaterial = ExtendedMaterial<StandardMaterial, BoundsMaterialExtension>;

#[derive(SystemParam)]
struct SpecialtyMaterialAssets<'w> {
    giraffe: Option<ResMut<'w, Assets<GiraffeMaterial>>>,
    bounds: Option<ResMut<'w, Assets<BoundsMaterial>>>,
    character: Option<ResMut<'w, Assets<CharacterMaterial>>>,
}

#[derive(SystemParam)]
struct SceneMaterialAssets<'w> {
    cloud: Option<ResMut<'w, Assets<CloudMaterial>>>,
    menu_sky: Option<ResMut<'w, Assets<MenuSkyMaterial>>>,
    godray: Option<ResMut<'w, Assets<GodrayMaterial>>>,
}

#[derive(SystemParam)]
struct CoreRenderAssets<'w> {
    images: Option<ResMut<'w, Assets<Image>>>,
    meshes: Option<ResMut<'w, Assets<Mesh>>>,
    materials: Option<ResMut<'w, Assets<StandardMaterial>>>,
}

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct TreeMaterialUniform {
    wind_direction_smoothness: Vec4,
    wind_controls: Vec4,
    season_controls: Vec4,
    main_scale_offset: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct TreeMaterialExtension {
    #[uniform(100)]
    parameters: TreeMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    main_texture: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    noise_texture: Option<Handle<Image>>,
}

impl MaterialExtension for TreeMaterialExtension {
    fn enable_shadows() -> bool {
        true
    }

    fn vertex_shader() -> ShaderRef {
        TREE_SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        TREE_SHADER_ASSET_PATH.into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        TREE_PREPASS_SHADER_ASSET_PATH.into()
    }
}

type TreeMaterial = ExtendedMaterial<StandardMaterial, TreeMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct GrassMaterialUniform {
    grid_color_1: Vec4,
    grid_color_2: Vec4,
    wind_color: Vec4,
    wind_direction_smoothness: Vec4,
    wind_controls: Vec4,
    surface_controls: Vec4,
    world_strength_transform: Vec4,
    main_scale_offset: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct GrassMaterialExtension {
    #[uniform(100)]
    parameters: GrassMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    main_texture: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    noise_texture: Option<Handle<Image>>,
}

impl MaterialExtension for GrassMaterialExtension {
    fn enable_shadows() -> bool {
        true
    }

    fn vertex_shader() -> ShaderRef {
        GRASS_SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        GRASS_SHADER_ASSET_PATH.into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        GRASS_PREPASS_SHADER_ASSET_PATH.into()
    }
}

type GrassMaterial = ExtendedMaterial<StandardMaterial, GrassMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct CritterMaterialUniform {
    animation_controls: Vec4,
    main_scale_offset: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct CritterMaterialExtension {
    #[uniform(100)]
    parameters: CritterMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    main_texture: Option<Handle<Image>>,
}

impl MaterialExtension for CritterMaterialExtension {
    fn enable_shadows() -> bool {
        true
    }

    fn vertex_shader() -> ShaderRef {
        CRITTER_SHADER_ASSET_PATH.into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        CRITTER_PREPASS_SHADER_ASSET_PATH.into()
    }
}

type CritterMaterial = ExtendedMaterial<StandardMaterial, CritterMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct FlagMaterialUniform {
    colour_1: Vec4,
    colour_2: Vec4,
    controls: Vec4,
    noise_scale_offset: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct FlagMaterialExtension {
    #[uniform(100)]
    parameters: FlagMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    noise_texture: Option<Handle<Image>>,
}

impl MaterialExtension for FlagMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        FLAG_SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        FLAG_SHADER_ASSET_PATH.into()
    }
}

type FlagMaterial = ExtendedMaterial<StandardMaterial, FlagMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct CharacterMaterialUniform {
    albedo_color: Vec4,
    shadow_controls: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct CharacterMaterialExtension {
    #[uniform(100)]
    parameters: CharacterMaterialUniform,
}

impl MaterialExtension for CharacterMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        CHARACTER_SHADER_ASSET_PATH.into()
    }
}

type CharacterMaterial = ExtendedMaterial<StandardMaterial, CharacterMaterialExtension>;

#[derive(Clone)]
enum ResolvedMaterialHandle {
    Standard(Handle<StandardMaterial>),
    Building(Handle<BuildingMaterial>),
    Cloud(Handle<CloudMaterial>),
    Godray(Handle<GodrayMaterial>),
    Giraffe(Handle<GiraffeMaterial>),
    Bounds(Handle<BoundsMaterial>),
    Tree(Handle<TreeMaterial>),
    Grass(Handle<GrassMaterial>),
    Critter(Handle<CritterMaterial>),
    Flag(Handle<FlagMaterial>),
    Character(Handle<CharacterMaterial>),
}

impl ResolvedMaterialHandle {
    fn untyped_id(&self) -> UntypedAssetId {
        match self {
            Self::Standard(handle) => handle.id().untyped(),
            Self::Building(handle) => handle.id().untyped(),
            Self::Cloud(handle) => handle.id().untyped(),
            Self::Godray(handle) => handle.id().untyped(),
            Self::Giraffe(handle) => handle.id().untyped(),
            Self::Bounds(handle) => handle.id().untyped(),
            Self::Tree(handle) => handle.id().untyped(),
            Self::Grass(handle) => handle.id().untyped(),
            Self::Critter(handle) => handle.id().untyped(),
            Self::Flag(handle) => handle.id().untyped(),
            Self::Character(handle) => handle.id().untyped(),
        }
    }
}

#[derive(Resource, Default)]
struct RenderAssets {
    cube: Handle<Mesh>,
    chimney_particle: Handle<Mesh>,
    actor_lod: Handle<Mesh>,
    menu_sky_mesh: Handle<Mesh>,
    cloud_plane: Handle<Mesh>,
    healing_ring: Handle<Mesh>,
    healing_plus: Option<Handle<Mesh>>,
    fish_school_mesh: Option<Handle<Mesh>>,
    fish_school_material: Handle<CritterMaterial>,
    projectile_arrow_scene: Option<Handle<bevy::world_serialization::WorldAsset>>,
    ground: Handle<TerrainMaterial>,
    traversal_wear: Handle<Image>,
    path_surface: Handle<Image>,
    water: Handle<WaterMaterial>,
    menu_water: Handle<WaterMaterial>,
    menu_sky: Handle<MenuSkyMaterial>,
    wood: Handle<StandardMaterial>,
    ore: Handle<StandardMaterial>,
    food: Handle<StandardMaterial>,
    building: Handle<StandardMaterial>,
    construction: Handle<StandardMaterial>,
    streetlight_lamp: Handle<StandardMaterial>,
    regeneration_buildings: BTreeMap<String, Handle<StandardMaterial>>,
    placement_valid: Handle<BoundsMaterial>,
    placement_invalid: Handle<BoundsMaterial>,
    enemy_idle: Handle<StandardMaterial>,
    enemy_moving: Handle<StandardMaterial>,
    player_idle: Handle<StandardMaterial>,
    player_moving: Handle<StandardMaterial>,
    selection: Handle<StandardMaterial>,
    rain: Handle<StandardMaterial>,
    snow: Handle<StandardMaterial>,
    projectile: Handle<StandardMaterial>,
    projectile_arrow: Handle<StandardMaterial>,
    projectile_necrotic: Handle<StandardMaterial>,
    impact_physical: Handle<StandardMaterial>,
    building_smoke: Handle<StandardMaterial>,
    chimney_smoke: BTreeMap<StableId, [Vec<Handle<StandardMaterial>>; 2]>,
    building_spark: Handle<StandardMaterial>,
    building_fire: Handle<StandardMaterial>,
    building_upgrade: Handle<StandardMaterial>,
    healing_green: Handle<StandardMaterial>,
    healing_channel: BTreeMap<StableId, Handle<StandardMaterial>>,
    healing_plus_materials: BTreeMap<StableId, Vec<Handle<StandardMaterial>>>,
    healing_disc_materials: BTreeMap<StableId, Vec<Handle<StandardMaterial>>>,
    authored_building: Handle<BuildingMaterial>,
    clouds: Handle<CloudMaterial>,
    menu_cloud: Handle<StandardMaterial>,
    menu_ocean_floor: Handle<StandardMaterial>,
    tree: Handle<TreeMaterial>,
    menu_tree: Handle<StandardMaterial>,
    grass: Handle<GrassMaterial>,
    game_logo: Option<Handle<Image>>,
    loading_screen: Option<Handle<Image>>,
    loading_overlay: Option<Handle<Image>>,
    loading_icon: Option<Handle<Image>>,
    ui_font: Option<Handle<Font>>,
    ui_display_font: Option<Handle<Font>>,
    main_menu_textures: BTreeMap<String, Handle<Image>>,
    top_bar_textures: BTreeMap<String, Handle<Image>>,
    selection_panel_textures: BTreeMap<String, Handle<Image>>,
    vote_textures: BTreeMap<String, Handle<Image>>,
    objective_textures: BTreeMap<String, Handle<Image>>,
    current_event_textures: BTreeMap<String, Handle<Image>>,
    ui_slicers: BTreeMap<String, TextureSlicer>,
    main_ui_scale: f32,
    settings_ui_scale: f32,
    presentation_materials: BTreeMap<StableId, ResolvedMaterialHandle>,
    presentation_materials_by_source_path: BTreeMap<String, ResolvedMaterialHandle>,
}

impl RenderAssets {
    fn world_gpu_mesh_ids(&self) -> BTreeSet<AssetId<Mesh>> {
        [self.cube.id(), self.actor_lod.id(), self.cloud_plane.id()]
            .into_iter()
            .collect()
    }

    fn world_gpu_material_ids(&self) -> BTreeSet<UntypedAssetId> {
        let mut ids = self
            .presentation_materials
            .values()
            .map(ResolvedMaterialHandle::untyped_id)
            .collect::<BTreeSet<_>>();
        ids.extend([
            self.ground.id().untyped(),
            self.water.id().untyped(),
            self.wood.id().untyped(),
            self.ore.id().untyped(),
            self.food.id().untyped(),
            self.building.id().untyped(),
            self.construction.id().untyped(),
            self.enemy_idle.id().untyped(),
            self.enemy_moving.id().untyped(),
            self.player_idle.id().untyped(),
            self.player_moving.id().untyped(),
            self.selection.id().untyped(),
            self.authored_building.id().untyped(),
            self.tree.id().untyped(),
            self.grass.id().untyped(),
            self.fish_school_material.id().untyped(),
        ]);
        ids
    }
}

#[derive(SystemParam)]
struct ActiveMaterialHandles<'w, 's> {
    standard: Query<'w, 's, &'static MeshMaterial3d<StandardMaterial>>,
    terrain: Query<'w, 's, &'static MeshMaterial3d<TerrainMaterial>>,
    water: Query<'w, 's, &'static MeshMaterial3d<WaterMaterial>>,
    building: Query<'w, 's, &'static MeshMaterial3d<BuildingMaterial>>,
    cloud: Query<'w, 's, &'static MeshMaterial3d<CloudMaterial>>,
    godray: Query<'w, 's, &'static MeshMaterial3d<GodrayMaterial>>,
    giraffe: Query<'w, 's, &'static MeshMaterial3d<GiraffeMaterial>>,
    bounds: Query<'w, 's, &'static MeshMaterial3d<BoundsMaterial>>,
    tree: Query<'w, 's, &'static MeshMaterial3d<TreeMaterial>>,
    grass: Query<'w, 's, &'static MeshMaterial3d<GrassMaterial>>,
    critter: Query<'w, 's, &'static MeshMaterial3d<CritterMaterial>>,
    flag: Query<'w, 's, &'static MeshMaterial3d<FlagMaterial>>,
    character: Query<'w, 's, &'static MeshMaterial3d<CharacterMaterial>>,
}

impl ActiveMaterialHandles<'_, '_> {
    fn ids(&self) -> BTreeSet<UntypedAssetId> {
        let mut ids = BTreeSet::new();
        ids.extend(
            self.standard
                .iter()
                .map(|material| material.0.id().untyped()),
        );
        ids.extend(
            self.terrain
                .iter()
                .map(|material| material.0.id().untyped()),
        );
        ids.extend(self.water.iter().map(|material| material.0.id().untyped()));
        ids.extend(
            self.building
                .iter()
                .map(|material| material.0.id().untyped()),
        );
        ids.extend(self.cloud.iter().map(|material| material.0.id().untyped()));
        ids.extend(self.godray.iter().map(|material| material.0.id().untyped()));
        ids.extend(
            self.giraffe
                .iter()
                .map(|material| material.0.id().untyped()),
        );
        ids.extend(self.bounds.iter().map(|material| material.0.id().untyped()));
        ids.extend(self.tree.iter().map(|material| material.0.id().untyped()));
        ids.extend(self.grass.iter().map(|material| material.0.id().untyped()));
        ids.extend(
            self.critter
                .iter()
                .map(|material| material.0.id().untyped()),
        );
        ids.extend(self.flag.iter().map(|material| material.0.id().untyped()));
        ids.extend(
            self.character
                .iter()
                .map(|material| material.0.id().untyped()),
        );
        ids
    }
}

#[derive(Component)]
struct StateEntity;

#[derive(Component)]
struct UiDisplayFont;

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum MainMenuAction {
    NewGame,
    LoadGame,
    Settings,
    Secrets,
    Credits,
    Quit,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum GoLiveConfirmationAction {
    No,
    Yes,
}

#[derive(Component)]
struct GoLiveConfirmationRoot;

#[derive(Component)]
struct GoLiveConfirmationBody;

#[derive(Component)]
struct TownDialogRoot(MenuPage);

#[derive(Component)]
struct TownNameField;

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum TownDialogAction {
    Create,
    Back,
}

#[derive(Component)]
struct TownLoadChoice {
    name: String,
    path: PathBuf,
    protected: bool,
}

#[derive(Component)]
struct TownDialogFeedback;

#[derive(Component)]
struct TownLoadList;

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum GameMenuAction {
    SaveGame,
    SaveJumpStart,
    LoadGame,
    Settings,
    GoLive,
    ExitGame,
    Close,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum SettingsAction {
    Apply,
    Defaults,
    Back,
    ConfirmApply,
    ConfirmDiscard,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
struct SettingsTabButton(SettingsTab);

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
struct SettingsValueButton {
    index: usize,
    direction: i8,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
struct SettingsValueRow(usize);

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
struct SettingsValueText(usize);

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum SecretsAction {
    DisclaimerYes,
    DisclaimerNo,
    ToggleBot,
    ToggleBroadcast,
    ToggleBandwidthTest,
    Save,
    AuthorizeBot,
    AuthorizeBroadcaster,
    Back,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum SecretsField {
    ClientId,
    BotLogin,
    ChannelLogin,
}

#[derive(Component)]
struct SecretsDynamicLabel(SecretsAction);

#[derive(Component)]
struct SecretsDisclaimerRoot;

#[derive(Component)]
struct SecretsRoot;

#[derive(Component)]
struct SecretsStatusText;

#[derive(Component)]
struct SecretsDeviceText;

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum SecretsConnectionKind {
    Bot,
    Broadcast,
}

#[derive(Component)]
struct SecretsConnectionText(SecretsConnectionKind);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SecretsStatusTone {
    Good,
    Pending,
    Inactive,
    Error,
}

#[derive(Component)]
struct SettingsRoot;

#[derive(Component)]
struct SettingsPanel;

#[derive(Component)]
struct SettingsRows;

#[derive(Component)]
struct SettingsConfirmModal;

#[derive(Component)]
struct SettingsFeedbackText;

#[derive(Component)]
struct GameMenuRoot;

#[derive(Component)]
struct GameMenuActionLabel(GameMenuAction);

#[derive(Component)]
struct LoadingScreenEntity;

#[derive(Component)]
struct LoadingUiCamera;

#[derive(Component)]
struct LoadingIconSpinner {
    radians_per_second: f32,
}

#[derive(Component)]
struct LoadingStatusText;

#[derive(Component)]
struct LoadingSubstatusText;

#[derive(Component)]
struct LoadingPercentText;

#[derive(Component)]
struct LoadingProgressFill;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum WorldLoadingPhase {
    #[default]
    Presenting,
    Loading,
    Spawning,
    Complete,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum LoadingWork {
    #[default]
    Pending,
    Count {
        completed: u64,
        total: u64,
    },
}

impl LoadingWork {
    fn count(completed: usize, total: usize) -> Self {
        Self::Count {
            completed: u64::try_from(completed).expect("loading work count fits u64"),
            total: u64::try_from(total).expect("loading work total fits u64"),
        }
    }

    fn boolean(completed: bool) -> Self {
        Self::Count {
            completed: u64::from(completed),
            total: 1,
        }
    }

    fn fraction(self) -> f32 {
        match self {
            Self::Pending => 0.0,
            Self::Count { total: 0, .. } => 1.0,
            Self::Count { completed, total } => {
                let completed =
                    u16::try_from(completed.min(total)).expect("loading leaf count fits u16");
                let total = u16::try_from(total).expect("loading leaf total fits u16");
                f32::from(completed) / f32::from(total)
            }
        }
    }
}

#[derive(Clone, Debug)]
enum LoadingWorkNode {
    Leaf(LoadingWork),
    Group(Vec<Self>),
}

impl LoadingWorkNode {
    fn leaf(work: LoadingWork) -> Self {
        Self::Leaf(work)
    }

    fn group(children: impl IntoIterator<Item = Self>) -> Self {
        Self::Group(children.into_iter().collect())
    }

    fn fraction(&self) -> f32 {
        match self {
            Self::Leaf(work) => work.fraction(),
            Self::Group(children) if children.is_empty() => 1.0,
            Self::Group(children) => {
                let child_count =
                    u16::try_from(children.len()).expect("loading group size fits u16");
                children.iter().map(Self::fraction).sum::<f32>() / f32::from(child_count)
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct WorldLoadingWork {
    cover_entities: LoadingWork,
    cover_artwork: LoadingWork,
    cover_gpu_images: LoadingWork,
    cover_pipelines: LoadingWork,
    cover_presented_frames: LoadingWork,
    world_assets: LoadingWork,
    generation_completed: BTreeSet<WorldGenerationStage>,
    terrain_entities: LoadingWork,
    resource_entities: LoadingWork,
    foliage_entities: LoadingWork,
    actor_entities: LoadingWork,
    gameplay_setup: LoadingWork,
    scene_roots: LoadingWork,
    scene_stable_frames: LoadingWork,
    material_overrides: LoadingWork,
    animation_receivers: LoadingWork,
    lighting_receivers: LoadingWork,
    gpu_images: LoadingWork,
    gpu_meshes: LoadingWork,
    gpu_materials: LoadingWork,
    gpu_pipelines: LoadingWork,
    selection_draw: LoadingWork,
    gpu_stable_frames: LoadingWork,
}

impl WorldLoadingWork {
    fn mark_cover_complete(&mut self) {
        self.cover_entities = LoadingWork::boolean(true);
        self.cover_artwork = LoadingWork::boolean(true);
        self.cover_gpu_images = LoadingWork::boolean(true);
        self.cover_pipelines = LoadingWork::boolean(true);
        self.cover_presented_frames = LoadingWork::boolean(true);
    }

    fn generation_node(&self) -> LoadingWorkNode {
        LoadingWorkNode::group(WorldGenerationStage::ALL.into_iter().map(|stage| {
            LoadingWorkNode::leaf(LoadingWork::boolean(
                self.generation_completed.contains(&stage),
            ))
        }))
    }

    fn root(&self) -> LoadingWorkNode {
        let leaf = LoadingWorkNode::leaf;
        LoadingWorkNode::group([
            LoadingWorkNode::group([
                leaf(self.cover_entities),
                leaf(self.cover_artwork),
                leaf(self.cover_gpu_images),
                leaf(self.cover_pipelines),
                leaf(self.cover_presented_frames),
            ]),
            LoadingWorkNode::group([leaf(self.world_assets), self.generation_node()]),
            LoadingWorkNode::group([
                LoadingWorkNode::group([
                    leaf(self.terrain_entities),
                    leaf(self.resource_entities),
                    leaf(self.foliage_entities),
                    leaf(self.actor_entities),
                    leaf(self.gameplay_setup),
                ]),
                leaf(self.scene_roots),
                leaf(self.scene_stable_frames),
            ]),
            LoadingWorkNode::group([
                leaf(self.material_overrides),
                leaf(self.animation_receivers),
                leaf(self.lighting_receivers),
            ]),
            LoadingWorkNode::group([
                leaf(self.gpu_images),
                leaf(self.gpu_meshes),
                leaf(self.gpu_materials),
                leaf(self.gpu_pipelines),
                leaf(self.selection_draw),
                leaf(self.gpu_stable_frames),
            ]),
        ])
    }

    fn progress(&self) -> f32 {
        self.root().fraction().clamp(0.0, 1.0)
    }
}

#[derive(Resource, Default)]
struct WorldLoadingRuntime {
    phase: WorldLoadingPhase,
    progress: f32,
    status: String,
    substatus: String,
    asset_handles: Vec<UntypedHandle>,
    loaded_assets: usize,
    failed_assets: usize,
    prepared_world: Option<PreparedWorld>,
    spawn_runtime: Option<WorldSpawnRuntime>,
    presented_frames: Option<PresentedRenderFrames>,
    scene_ready_frames: u8,
    completion_remaining_seconds: f32,
    work: WorldLoadingWork,
}

struct PreparedWorld {
    world: GeneratedWorld,
    terrain_mesh: Option<Mesh>,
    terrain_collider: Option<Collider>,
    water_mesh: Option<Mesh>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorldSpawnPhase {
    Resources,
    Foliage,
    Gameplay,
}

struct WorldSpawnRuntime {
    phase: WorldSpawnPhase,
    resource_cursor: usize,
    foliage_cursor: usize,
    resource_total: usize,
    foliage_total: usize,
    foliage_gpu_batches: BTreeSet<(StableId, u16)>,
    foliage_spatial_groups: BTreeSet<FoliageBatchKey>,
    update_count: u32,
    starting_render_frame: u64,
}

#[derive(Resource)]
struct WorldLoadingCoverRuntime {
    started_at: Instant,
    initialized: bool,
    starting_render_frame: u64,
    fallback_updates: u8,
    ready_updates: u8,
    transition_queued: bool,
}

impl Default for WorldLoadingCoverRuntime {
    fn default() -> Self {
        Self {
            started_at: Instant::now(),
            initialized: false,
            starting_render_frame: 0,
            fallback_updates: 0,
            ready_updates: 0,
            transition_queued: false,
        }
    }
}

#[derive(Resource)]
struct WorldGenerationTask {
    task: Task<PreparedWorld>,
    completed: Arc<Mutex<BTreeSet<WorldGenerationStage>>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BootDestination {
    MainMenu,
    WorldLoading,
    Credits,
}

#[derive(Resource)]
struct MenuLoadingRuntime {
    started_at: Instant,
    destination: BootDestination,
    progress: f32,
    status: String,
    substatus: String,
    asset_handles: Vec<UntypedHandle>,
    loaded_assets: usize,
    failed_assets: usize,
    ready_presented_frames: u8,
}

#[derive(Resource)]
struct MenuRevealRuntime {
    started_at: Instant,
    starting_render_frame: u64,
    ready_starting_render_frame: Option<u64>,
    fallback_updates: u16,
    scene_ready_frames: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MainMenuSpawnPhase {
    Models,
    Resources,
    Foliage,
    Complete,
}

#[derive(Resource)]
struct MainMenuSpawnRuntime {
    phase: MainMenuSpawnPhase,
    model_indices: Vec<usize>,
    resource_indices: Vec<usize>,
    foliage_indices: Vec<usize>,
    cursor: usize,
    completed: usize,
    total: usize,
    update_count: u32,
    starting_render_frame: u64,
}

#[derive(Resource)]
struct WorldRevealRuntime {
    started_at: Instant,
    starting_render_frame: u64,
    ready_starting_render_frame: Option<u64>,
    completion_starting_render_frame: Option<u64>,
    fallback_updates: u16,
    ready_frames: u8,
    simulation_elapsed_seconds: f64,
    session_elapsed_seconds: f64,
}

impl Default for WorldRevealRuntime {
    fn default() -> Self {
        Self {
            started_at: Instant::now(),
            starting_render_frame: 0,
            ready_starting_render_frame: None,
            completion_starting_render_frame: None,
            fallback_updates: 0,
            ready_frames: 0,
            simulation_elapsed_seconds: 0.0,
            session_elapsed_seconds: 0.0,
        }
    }
}

impl Default for MenuRevealRuntime {
    fn default() -> Self {
        Self {
            started_at: Instant::now(),
            starting_render_frame: 0,
            ready_starting_render_frame: None,
            fallback_updates: 0,
            scene_ready_frames: 0,
        }
    }
}

#[derive(Resource, Clone)]
struct PresentedRenderFrames {
    count: Arc<AtomicU64>,
    render_schedule_available: bool,
}

impl PresentedRenderFrames {
    fn new(render_schedule_available: bool) -> Self {
        Self {
            count: Arc::new(AtomicU64::new(0)),
            render_schedule_available,
        }
    }

    fn current(&self) -> u64 {
        self.count.load(AtomicOrdering::Acquire)
    }
}

fn record_presented_render_frame(frames: Res<PresentedRenderFrames>) {
    frames.count.fetch_add(1, AtomicOrdering::Release);
}

#[derive(Clone, Default)]
struct GpuReadinessExpected {
    active: bool,
    epoch: u64,
    images: BTreeSet<AssetId<Image>>,
    meshes: BTreeSet<AssetId<Mesh>>,
    materials: BTreeSet<UntypedAssetId>,
    selection_entity: Option<Entity>,
}

#[derive(Clone, Copy, Debug, Default)]
struct GpuReadinessSnapshot {
    epoch: u64,
    expected_images: usize,
    ready_images: usize,
    expected_meshes: usize,
    ready_meshes: usize,
    expected_materials: usize,
    ready_materials: usize,
    pending_pipelines: usize,
    failed_pipelines: usize,
    selection_expected: bool,
    selection_draw_ready: bool,
}

impl GpuReadinessSnapshot {
    fn is_ready(self) -> bool {
        self.epoch != 0
            && self.ready_images == self.expected_images
            && self.ready_meshes == self.expected_meshes
            && self.ready_materials == self.expected_materials
            && self.pending_pipelines == 0
            && self.failed_pipelines == 0
            && (!self.selection_expected || self.selection_draw_ready)
    }
}

#[derive(Default)]
struct GpuReadinessShared {
    expected: GpuReadinessExpected,
    snapshot: GpuReadinessSnapshot,
}

#[derive(Clone, Resource, Default)]
struct GpuReadinessProbe(Arc<Mutex<GpuReadinessShared>>);

impl GpuReadinessProbe {
    fn begin_world(
        &self,
        images: BTreeSet<AssetId<Image>>,
        meshes: BTreeSet<AssetId<Mesh>>,
        materials: BTreeSet<UntypedAssetId>,
    ) {
        let mut shared = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let epoch = shared.expected.epoch.wrapping_add(1).max(1);
        shared.expected = GpuReadinessExpected {
            active: true,
            epoch,
            images,
            meshes,
            materials,
            selection_entity: None,
        };
        shared.snapshot = GpuReadinessSnapshot::default();
    }

    fn merge_world_content(
        &self,
        meshes: impl IntoIterator<Item = AssetId<Mesh>>,
        materials: impl IntoIterator<Item = UntypedAssetId>,
        selection_entity: Option<Entity>,
    ) {
        let mut shared = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if !shared.expected.active {
            return;
        }
        let mut changed = false;
        for mesh in meshes {
            changed |= shared.expected.meshes.insert(mesh);
        }
        for material in materials {
            changed |= shared.expected.materials.insert(material);
        }
        if selection_entity.is_some() && shared.expected.selection_entity != selection_entity {
            shared.expected.selection_entity = selection_entity;
            changed = true;
        }
        if changed {
            shared.expected.epoch = shared.expected.epoch.wrapping_add(1).max(1);
            shared.snapshot = GpuReadinessSnapshot::default();
        }
    }

    fn snapshot(&self) -> GpuReadinessSnapshot {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .snapshot
    }

    fn clear(&self) {
        let mut shared = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        shared.expected.active = false;
    }
}

fn record_gpu_readiness(
    probe: Res<GpuReadinessProbe>,
    gpu_images: Res<GpuRenderAssets<GpuImage>>,
    gpu_meshes: Res<GpuRenderAssets<RenderMesh>>,
    gpu_materials: Res<ErasedRenderAssets<PreparedMaterial>>,
    pipeline_cache: Res<PipelineCache>,
    render_mesh_instances: Res<RenderMeshInstances>,
) {
    let (expected, selection_previously_ready) = {
        let shared = probe.0.lock().unwrap_or_else(PoisonError::into_inner);
        if !shared.expected.active {
            return;
        }
        (
            shared.expected.clone(),
            shared.snapshot.epoch == shared.expected.epoch && shared.snapshot.selection_draw_ready,
        )
    };
    let waiting_pipelines = pipeline_cache.waiting_pipelines().count();
    let queued_or_creating = pipeline_cache
        .pipelines()
        .filter(|pipeline| {
            matches!(
                pipeline.state,
                CachedPipelineState::Queued | CachedPipelineState::Creating(_)
            )
        })
        .count();
    let snapshot = GpuReadinessSnapshot {
        epoch: expected.epoch,
        expected_images: expected.images.len(),
        ready_images: expected
            .images
            .iter()
            .filter(|id| gpu_images.get(**id).is_some())
            .count(),
        expected_meshes: expected.meshes.len(),
        ready_meshes: expected
            .meshes
            .iter()
            .filter(|id| gpu_meshes.get(**id).is_some())
            .count(),
        expected_materials: expected.materials.len(),
        ready_materials: expected
            .materials
            .iter()
            .filter(|id| gpu_materials.get(**id).is_some())
            .count(),
        pending_pipelines: waiting_pipelines.max(queued_or_creating),
        failed_pipelines: pipeline_cache
            .pipelines()
            .filter(|pipeline| matches!(pipeline.state, CachedPipelineState::Err(_)))
            .count(),
        selection_expected: expected.selection_entity.is_some(),
        selection_draw_ready: selection_previously_ready
            || expected.selection_entity.is_none_or(|entity| {
                render_mesh_instances
                    .render_mesh_queue_data(MainEntity::from(entity))
                    .is_some()
            }),
    };
    let mut shared = probe.0.lock().unwrap_or_else(PoisonError::into_inner);
    if shared.expected.active && shared.expected.epoch == snapshot.epoch {
        shared.snapshot = snapshot;
    }
}

#[derive(Component)]
struct AuthoredCreditsElement {
    target_path: String,
}

#[derive(Component)]
struct CreditsSkipButton;

#[derive(Component)]
struct CreditsFade;

#[derive(Component)]
struct CreditsFireworksEmitter {
    target_path: String,
    effect: StableId,
    emitter_index: u8,
    origin_percent: Vec2,
    next_launch_index: u32,
    active_rockets: u16,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum CreditsFireworkParticleKind {
    Rocket,
    BurstFlash,
    Spark,
}

#[derive(Component)]
struct CreditsFireworkParticle {
    kind: CreditsFireworkParticleKind,
    effect: StableId,
    emitter_index: u8,
    sequence: u32,
    position_percent: Vec2,
    velocity_percent_per_second: Vec2,
    age_seconds: f32,
    lifetime_seconds: f32,
    color_index: usize,
}

#[derive(Component)]
struct CreditsFireworkBurst {
    effect: StableId,
    emitter_index: u8,
    sequence: u32,
    position_percent: Vec2,
    color_index: usize,
    remaining: u16,
    delay_seconds: f32,
}

#[derive(Resource)]
struct CreditsTimeline {
    elapsed_seconds: f32,
}

impl Default for CreditsTimeline {
    fn default() -> Self {
        Self {
            elapsed_seconds: std::env::var("STREAM_TOWN_DEBUG_CREDITS_TIME")
                .ok()
                .and_then(|value| value.parse().ok())
                .filter(|value: &f32| value.is_finite() && *value >= 0.0)
                .unwrap_or(0.0),
        }
    }
}

#[derive(Component)]
struct LevelUpToast;

#[derive(Resource, Default)]
struct LevelUpPresentation {
    actor_levels: BTreeMap<StableId, u16>,
    elapsed_seconds: Option<f32>,
}

#[derive(Component)]
struct WorldEntity;

#[derive(Component)]
struct PingPointer {
    actor: StableId,
    elapsed_seconds: f32,
    base_scale: f32,
}

#[derive(Component)]
struct SeagullFlight {
    start: Vec3,
    end: Vec3,
    elapsed_seconds: f32,
    leg_serial: u64,
    call_elapsed_seconds: f32,
    call_wait_seconds: f32,
    call_serial: u64,
    world_seed: u64,
}

#[derive(Component)]
struct MainMenuCloudPrism {
    drift_per_second: Vec3,
    wrap_min_x: f32,
    wrap_max_x: f32,
    fade_delay_seconds: f32,
    fade_elapsed_seconds: f32,
    entrance_fade_distance: f32,
    target_alpha: f32,
}

#[derive(Component)]
struct TerrainSurface;

#[derive(Component)]
struct WaterSurface;

#[derive(Component)]
struct WorldDiagnosticOverlay {
    mode: WorldDiagnosticMode,
}

#[derive(Component, Clone, Copy)]
struct WorldDiagnosticVisibilityBackup(Visibility);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum TerrainLodLevel {
    #[default]
    High,
    Medium,
    Low,
}

#[derive(Component)]
struct TerrainChunkLod {
    centre: Vec2,
    high: Handle<Mesh>,
    medium: Handle<Mesh>,
    low: Handle<Mesh>,
    current: TerrainLodLevel,
}

#[derive(Component)]
struct Agent {
    id: StableId,
    kind: ActorKind,
    archetype: StableId,
    goal: AgentGoal,
    spawn: GridPos,
    origin: GridPos,
    /// Current location in the navigation-only grid (three units per town cell).
    navigation_position: GridPos,
    path: Vec<GridPos>,
    path_index: usize,
    target: GridPos,
    action_cooldown_seconds: f32,
    action_started: bool,
    repath_remaining_seconds: f32,
    health_regen_accumulator: f64,
    wander_sequence: u64,
    previous_wander_origin: Option<GridPos>,
}

#[derive(Resource, Default)]
struct RulerVoteAnnouncementRuntime {
    initialized: bool,
    active_kind: Option<RulerVoteKind>,
    ruler_before_vote: Option<StableId>,
}

#[derive(Resource, Default)]
struct CitizenDeathAnnouncementRuntime {
    initialized: bool,
    dead: BTreeSet<StableId>,
}

#[derive(Component, Default)]
struct AgentLocomotion {
    previous_position: Vec3,
    normalized_speed: f32,
    stop_grace_seconds: f32,
    initialized: bool,
}

#[derive(Component)]
struct PlayerRigAxisCorrected;

#[derive(Component)]
struct PlayerRigAxisCorrectionRequired;

#[derive(Component)]
struct PlayerAnimatedRig;

/// Marks an animated-player renderer using the character-specific shadow
/// receiver path.
#[derive(Component)]
struct AnimatedCharacterShadowReceiver;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
enum AgentGoal {
    #[default]
    Wander,
    Gather(StableId),
    HarvestFarm(StableId),
    Deposit,
    WaitForStorage,
    Attack(StableId),
    AttackBuilding(StableId),
    Heal(StableId),
    Construct(StableId),
    VisitRegenerationStation(StableId),
    PlantTree(GridPos),
    Prospect {
        cell: GridPos,
        sequence: u32,
    },
    PlantBush(GridPos),
}

#[derive(Component, Clone, Copy)]
struct GridLocation(GridPos);

#[derive(Component)]
struct ResourceNode {
    id: StableId,
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
struct ResourceVisual {
    mesh_index: usize,
}

#[derive(Component)]
struct FoliageVisual(StableId);

#[derive(Component)]
struct PendingSurfaceGrounding {
    surface_height: f32,
}

/// Gameplay visuals must wait for the authoritative terrain runtime before
/// they are grounded or habitat-tested. They are queued while the loading
/// screen is still constructing that runtime.
#[derive(Component)]
struct RuntimeTerrainGrounding;

#[derive(Component, Clone, Copy)]
struct SurfaceFoliageHabitat(FoliageHabitat);

#[derive(Component)]
struct FoliageRenderBatch(FoliageBatchKey);

#[derive(Component, Clone, Copy)]
struct FoliageNavigationLocation(GridPos);

type FoliageClearanceQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static GridLocation,
        Option<&'static FoliageNavigationLocation>,
        Option<&'static FoliageRenderBatch>,
        Option<&'static PendingSurfaceGrounding>,
        &'static mut Visibility,
    ),
    With<FoliageVisual>,
>;

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudCommandGuidance;

#[derive(Component)]
struct HudResourceStrip;

#[derive(Component)]
struct HudStatsStrip;

#[derive(Component)]
struct HudMetricRow;

#[derive(Component)]
struct HudTopBar;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
enum HudTechnologyTextKind {
    Title,
    Requirement,
}

#[derive(Component)]
struct HudTechnologyObjectivePanel;

#[derive(Component)]
struct HudTechnologyProgressFill;

type HudMetricTextQuery<'w, 's> = Query<
    'w,
    's,
    (&'static HudMetric, &'static mut Text),
    (Without<Hud>, Without<HudTechnologyTextKind>),
>;
type HudMetricMaximumTextQuery<'w, 's> = Query<
    'w,
    's,
    (&'static HudMetricMaximum, &'static mut Text),
    (
        Without<Hud>,
        Without<HudMetric>,
        Without<HudTechnologyTextKind>,
    ),
>;
type HudTechnologyTextQuery<'w, 's> = Query<
    'w,
    's,
    (&'static HudTechnologyTextKind, &'static mut Text),
    (Without<Hud>, Without<HudMetric>),
>;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
enum HudMetric {
    Food,
    Gold,
    Ore,
    Wood,
    Players,
    Npcs,
    Buildings,
    PlayTime,
}

impl HudMetric {
    const fn shows_maximum(self) -> bool {
        matches!(
            self,
            Self::Food | Self::Gold | Self::Ore | Self::Wood | Self::Npcs
        )
    }
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
struct HudMetricMaximum(HudMetric);

#[derive(Component)]
struct SeasonMeter;

#[derive(Component)]
struct SelectionPanel;

#[derive(Component)]
struct SelectionPanelSlider(SelectionPanelBar);

#[derive(Component)]
struct SelectionPanelSliderTrack(SelectionPanelBar);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SelectionPanelBar {
    Health,
    Experience,
}

struct SelectionPanelDetails {
    description: String,
    health_progress: Option<f32>,
    experience_progress: Option<f32>,
}

type SelectionPanelTextQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Text, &'static mut Visibility),
    (With<SelectionPanel>, Without<SelectionPanelSliderTrack>),
>;
type SelectionPanelTrackQuery<'w, 's> = Query<
    'w,
    's,
    (&'static SelectionPanelSliderTrack, &'static mut Visibility),
    (With<SelectionPanelSliderTrack>, Without<SelectionPanel>),
>;

impl SelectionPanelDetails {
    const fn progress(&self, bar: SelectionPanelBar) -> Option<f32> {
        match bar {
            SelectionPanelBar::Health => self.health_progress,
            SelectionPanelBar::Experience => self.experience_progress,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
enum VotePanelKind {
    Technology,
    Ruler,
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
enum VoteTextKind {
    TechnologyTitle,
    TechnologyTimer,
    TechnologyOptionTitle(u8),
    TechnologyOptionDepthTag(u8),
    TechnologyOptionRequirements(u8),
    RulerTitle,
    RulerDescription,
    RulerTimer,
}

#[derive(Component)]
struct RulerOptionsContainer;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
enum VoteFillKind {
    TechnologyTimer,
    TechnologyOption(u8),
    RulerTimer,
}

#[derive(Component, Clone, Copy)]
struct TechnologyVoteIcon(u8);

#[derive(Component, Clone, Copy)]
struct TechnologyVoteOptionRow(u8);

#[derive(Component, Clone, Copy)]
struct TechnologyVoteTitleBar;

#[derive(Component, Clone, Copy)]
struct TechnologyVoteDepthBadge(u8);

#[derive(Component)]
struct TechnologyVoteTimerTrack;

#[derive(Component)]
struct TechnologyVoteTimerLabel;

#[derive(Component)]
struct TechnologyVoteTimerGroup;

#[derive(Component)]
struct TechnologyVoteTimerBar;

#[derive(Component)]
struct TechnologyVoteTimerIcon;

#[derive(Component)]
struct TechnologyVoteTimerGap;

#[derive(Component)]
struct CurrentEventPanel;

#[derive(Component, Clone, Copy)]
enum CurrentEventText {
    Title,
    Description,
    Progress,
}

#[derive(Component)]
struct CurrentEventFill;

type TechnologyVoteIconQuery<'w, 's> =
    Query<'w, 's, (&'static TechnologyVoteIcon, &'static mut ImageNode)>;
type TechnologyVoteRowQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static TechnologyVoteOptionRow,
        &'static mut Visibility,
        &'static mut Node,
    ),
    (
        Without<VotePanelKind>,
        Without<VoteFillKind>,
        Without<TechnologyVoteDepthBadge>,
    ),
>;
type TechnologyVoteDepthBadgeQuery<'w, 's> = Query<
    'w,
    's,
    (&'static TechnologyVoteDepthBadge, &'static mut Visibility),
    (Without<VotePanelKind>, Without<TechnologyVoteOptionRow>),
>;
type TownCameraMutQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Camera,
        &'static mut Projection,
        &'static mut Transform,
        &'static mut AmbientLight,
    ),
    (With<TownCamera>, Without<TownSun>),
>;
type TownSunMutQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut DirectionalLight, &'static mut Transform),
    (With<TownSun>, Without<TownCamera>),
>;
type TownPostProcessQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        Option<&'static mut ColorGrading>,
        Has<Hdr>,
        Has<Bloom>,
        Has<Vignette>,
        Has<MotionBlur>,
        Option<&'static mut Tonemapping>,
    ),
    With<TownCamera>,
>;
type LoadingSubstatusQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut Text,
    (
        With<LoadingSubstatusText>,
        Without<LoadingStatusText>,
        Without<LoadingPercentText>,
    ),
>;
type LoadingPercentQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut Text,
    (
        With<LoadingPercentText>,
        Without<LoadingStatusText>,
        Without<LoadingSubstatusText>,
    ),
>;
type LoadingCoverEntityQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, Option<&'static mut Visibility>),
    Or<(With<LoadingScreenEntity>, With<LoadingUiCamera>)>,
>;
type LoadingRetirementQuery<'w, 's> =
    Query<'w, 's, (), Or<(With<LoadingScreenEntity>, With<LoadingUiCamera>)>>;
#[derive(SystemParam)]
struct WorldRevealReadinessQueries<'w, 's> {
    material_specs: Query<'w, 's, (), With<MaterialOverrideSpec>>,
    mesh_overrides: Query<'w, 's, (Entity, Option<&'static MaterialOverrideApplied>), With<Mesh3d>>,
    animated_player_rigs: Query<'w, 's, (), With<PlayerAnimatedRig>>,
    character_receivers: Query<'w, 's, (), With<AnimatedCharacterShadowReceiver>>,
    suppressed_receivers: Query<'w, 's, (), With<bevy::light::NotShadowReceiver>>,
    converted_animation_roots:
        Query<'w, 's, Option<&'static ConvertedAnimationApplied>, With<ConvertedAnimationSpec>>,
    native_animation_roots: Query<'w, 's, Entity, With<NativeAnimationSpec>>,
    native_animation_drivers: Query<'w, 's, &'static ActorAnimationDriver>,
    parents: Query<'w, 's, &'static ChildOf>,
}
type MenuWorldAssetRootQuery<'w, 's> = Query<
    'w,
    's,
    (Option<&'static Children>, Option<&'static WorldInstance>),
    (
        With<StateEntity>,
        With<WorldAssetRoot>,
        Without<WorldEntity>,
    ),
>;
type WorldAssetRootQuery<'w, 's> = Query<
    'w,
    's,
    (Option<&'static Children>, Option<&'static WorldInstance>),
    (With<WorldEntity>, With<WorldAssetRoot>),
>;

#[derive(Component)]
struct MenuOverlay;

type MenuOverlayEntityQuery<'w, 's> = Query<
    'w,
    's,
    Entity,
    Or<(
        With<MenuOverlay>,
        With<GameMenuRoot>,
        With<SettingsRoot>,
        With<SecretsDisclaimerRoot>,
        With<SecretsRoot>,
        With<GoLiveConfirmationRoot>,
        With<TownDialogRoot>,
    )>,
>;

#[derive(Component)]
struct TownHall;

#[derive(Component)]
struct RuntimeBuilding {
    id: StableId,
}

#[derive(Component)]
struct NightPointLightPoolSlot;

#[derive(Component)]
struct ChimneySmokeEmitters {
    prefab_guid: String,
    age: u8,
    emitters: Vec<ChimneySmokeEmitterRuntime>,
}

struct ChimneySmokeEmitterRuntime {
    effect: StableId,
    local_position: Vec3,
    emission_accumulator: f32,
    sequence: u32,
}

#[derive(Component)]
struct ChimneySmokeParticle {
    effect: StableId,
    elapsed_seconds: f32,
    duration_seconds: f32,
    origin: Vec3,
    velocity: Vec3,
    base_scale: Vec3,
    size_over_lifetime: [f32; 2],
    color_variant: usize,
}

#[derive(Component)]
struct ActorNameOverlay {
    actor: StableId,
}

#[derive(Component)]
struct ActorHealthOverlay {
    actor: StableId,
    last_health: i32,
    was_damaged: bool,
    hide_remaining_seconds: f32,
}

#[derive(Component)]
struct ActorHealthFill;

#[derive(Component)]
struct TemporaryWorldLabel {
    target: StableId,
    remaining_seconds: f32,
}

#[derive(Component)]
struct BuildingHealthOverlay {
    building: StableId,
}

#[derive(Component)]
struct BuildingHealthFill;

#[derive(Component)]
struct BuildingPlacementVisual {
    owner: StableId,
    cell: GridPos,
}

#[derive(Component)]
struct BuildingPlacementGhost {
    owner: StableId,
    cell: GridPos,
    building: StableId,
    scene_asset_path: String,
}

#[derive(Component)]
struct BuildingPlacementGhostMesh {
    owner: StableId,
}

#[derive(Component)]
struct BuildingPlacementGhostNodeProcessed;

#[derive(Component)]
struct BuildingPlacementGhostHiddenNodes(BTreeSet<String>);

#[derive(Component)]
struct BuildingPlacementOwnerOverlay {
    owner: StableId,
}

#[derive(Component)]
struct EnemyCamp {
    id: StableId,
}

#[derive(Component)]
struct FishGodPresentation;

#[derive(Component)]
struct FishGodAnimation;

#[derive(Component)]
struct FishGodExitTimer {
    remaining_seconds: f32,
}

#[derive(Component)]
struct FishGodExitTriggerSent;

#[derive(Component)]
struct FallingFish {
    effect: StableId,
    sequence: u32,
    velocity: Vec3,
    angular_velocity: Vec3,
    age_seconds: f32,
    lifetime_seconds: f32,
    base_scale: f32,
    collision_count: u8,
}

#[derive(Component)]
struct FallingFishEmitter {
    effect: StableId,
    emission_remainder: f32,
    sequence: u32,
}

#[derive(Component)]
struct FishSchoolParticle {
    base_position: Vec3,
    noise_amplitude: Vec3,
    phase: Vec3,
    frequency: f32,
    scroll_speed: f32,
    octaves: u8,
    octave_multiplier: f32,
    octave_scale: f32,
    align_to_velocity: bool,
}

#[derive(Component)]
struct TowerShooter {
    cooldown_seconds: f32,
}

#[derive(Component)]
struct CombatProjectile {
    source: ProjectileSource,
    target: StableId,
    damage: u32,
    speed_cells_per_second: f32,
    visual: CombatVisualKind,
    trail_cooldown_seconds: f32,
    remaining_seconds: f32,
}

#[derive(Clone)]
enum ProjectileSource {
    Actor(StableId),
    Building(StableId),
}

#[derive(Clone)]
struct ProjectileSpawn {
    source: ProjectileSource,
    target: StableId,
    damage: u32,
    speed_cells_per_second: f32,
    visual: CombatVisualKind,
}

#[derive(Clone)]
enum ActionPresentation {
    Projectile(ProjectileSpawn),
    Impact {
        target: GridPos,
        visual: CombatVisualKind,
    },
    Healing {
        source: StableId,
        target: StableId,
    },
    BuildingWork {
        target: GridPos,
        sparks: bool,
    },
    BuildingDestroyed {
        building: StableId,
        target: GridPos,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CombatVisualKind {
    Physical,
    Arrow,
    Fireball,
    Necrotic,
}

#[derive(Component)]
struct CombatTrailSegment {
    elapsed_seconds: f32,
    duration_seconds: f32,
    base_scale: Vec3,
}

#[derive(Component)]
struct CombatImpactParticle {
    elapsed_seconds: f32,
    duration_seconds: f32,
    origin: Vec3,
    velocity: Vec3,
    base_scale: Vec3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BuildingEffectKind {
    WorkSmoke,
    WorkSpark,
    LevelArrow,
    DamageSmoke,
    DamageFire,
}

#[derive(Component)]
struct BuildingEffectParticle {
    kind: BuildingEffectKind,
    elapsed_seconds: f32,
    duration_seconds: f32,
    origin: Vec3,
    velocity: Vec3,
    base_scale: Vec3,
    phase: f32,
}

#[derive(Component, Default)]
struct BuildingDamageEmitter {
    cooldown_seconds: f32,
    sequence: u32,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum HealingEffectKind {
    Channel,
    Burst,
    Revive,
}

#[derive(Component)]
struct HealingRingEffect {
    kind: HealingEffectKind,
    effect: StableId,
    origin: Vec3,
    elapsed_seconds: f32,
    duration_seconds: f32,
    base_scale: f32,
    follow_target: Option<StableId>,
}

#[derive(Component)]
struct HealingMoteEffect {
    kind: HealingEffectKind,
    effect: StableId,
    origin: Vec3,
    elapsed_seconds: f32,
    duration_seconds: f32,
    angle_radians: f32,
    phase: f32,
    base_scale: Vec3,
    distance_scale: f32,
    size_multiplier: f32,
    follow_target: Option<StableId>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct HealingEffectSample {
    ring_scale: f32,
    mote_scale: f32,
    radial_distance: f32,
    rise: f32,
    rotation_radians: f32,
}

#[derive(Component)]
struct BuildingPresentation {
    base_translation: Vec3,
    base_scale: Vec3,
    base_height_offset: f32,
    applied_stage: u8,
    applied_level: u16,
    applied_age: u8,
    applied_scene: Option<String>,
}

#[derive(Component)]
struct TownCamera;

#[derive(Component, Clone)]
struct TownCameraControllerRuntime {
    home: Transform,
    move_target: Vec3,
    zoom_target_height: f32,
    seconds_since_acknowledgement: f32,
    observed_command_acknowledgements: u64,
    auto_shot: AutoCameraShot,
    auto_shot_elapsed_seconds: f32,
    auto_sequence: u64,
    temporary_focus: Option<TemporaryCameraFocus>,
    observed_damage_sequence: u64,
    combat_redirect_cooldown_seconds: f32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
enum AutoCameraShot {
    #[default]
    Inactive,
    Town,
    Citizen(StableId),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CameraFocusTarget {
    Citizen(StableId),
    Building(StableId),
}

#[derive(Clone, Debug)]
struct TemporaryCameraFocus {
    target: CameraFocusTarget,
    remaining_seconds: f32,
    return_to_auto_camera: bool,
}

#[derive(Resource, Default)]
struct CameraDamageRuntime {
    actor_health: BTreeMap<StableId, i32>,
    building_health: BTreeMap<StableId, i32>,
    latest_target: Option<CameraFocusTarget>,
    sequence: u64,
    initialized: bool,
}

#[derive(Resource, Default)]
struct RetreatingCitizens(BTreeSet<StableId>);

#[derive(SystemParam)]
struct AgentRecoveryRuntime<'w> {
    path_failures: ResMut<'w, PathFailureRuntime>,
    retreating: ResMut<'w, RetreatingCitizens>,
}

#[derive(Default)]
struct FoliageAcceptanceCapture {
    initialized: bool,
    elapsed_seconds: f32,
    next_capture: usize,
    written_captures: usize,
    capture_cooldown_seconds: f32,
    starting_camera: Option<Transform>,
    output_directory: Option<PathBuf>,
    renderer_count: usize,
    shadow_caster_count: usize,
    shadow_receiver_count: usize,
    pending_grounding_count: usize,
    habitat_violation_count: usize,
    configured_seed: u64,
    generated_seed: u64,
    duplicate_group_count: usize,
    duplicate_groups: Vec<serde_json::Value>,
    frames: Vec<serde_json::Value>,
    completion_delay_seconds: Option<f32>,
}

#[derive(Component)]
struct FoliageAcceptanceScreenshot;

impl TownCameraControllerRuntime {
    fn new(home: Transform) -> Self {
        Self {
            move_target: home.translation,
            zoom_target_height: home.translation.y,
            seconds_since_acknowledgement: 0.0,
            observed_command_acknowledgements: 0,
            auto_shot: AutoCameraShot::Inactive,
            auto_shot_elapsed_seconds: 0.0,
            auto_sequence: 0,
            temporary_focus: None,
            observed_damage_sequence: 0,
            combat_redirect_cooldown_seconds: 0.0,
            home,
        }
    }

    fn set_home(&mut self, home: Transform) {
        self.move_target = home.translation;
        self.zoom_target_height = home.translation.y;
        self.seconds_since_acknowledgement = 0.0;
        self.auto_shot = AutoCameraShot::Inactive;
        self.auto_shot_elapsed_seconds = 0.0;
        self.auto_sequence = 0;
        self.temporary_focus = None;
        self.home = home;
    }

    fn return_home(&mut self) {
        self.move_target = self.home.translation;
        self.zoom_target_height = self.home.translation.y;
    }

    fn cancel_auto_camera(&mut self) {
        self.auto_shot = AutoCameraShot::Inactive;
        self.auto_shot_elapsed_seconds = 0.0;
        self.auto_sequence = 0;
    }
}

#[derive(Component)]
struct ActivePetVisual {
    owner: StableId,
    pet: StableId,
    movement_speed: f32,
}

#[derive(Component)]
struct WeatherParticle {
    kind: Weather,
    seed: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct EnvironmentPalette {
    terrain_tint: [f32; 3],
    terrain_winter_strength: f32,
    water_color: [f32; 4],
    clear_color: [f32; 3],
    sun_color: [f32; 3],
    sun_illuminance: f32,
    ambient_color: [f32; 3],
    ambient_brightness: f32,
    fog_color: [f32; 4],
    fog_start: f32,
    fog_end: f32,
    particle_count: u16,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum MovementAnimationState {
    #[default]
    Idle,
    Moving,
}

#[derive(Component, Default)]
struct AgentAnimation {
    state: MovementAnimationState,
    phase: f32,
    base_scale: Vec3,
    native: bool,
}

#[derive(Component, Default)]
struct AgentEquipmentPresentation;

#[derive(Component, Default)]
struct EquipmentNodeProcessed;

#[derive(Component, Default)]
struct TransientCarryVisibility(bool);

#[derive(Component)]
struct EquipmentNode {
    actor_root: Entity,
    name: String,
}

#[derive(Component, Default)]
struct EnemyModelNodeProcessed;

#[derive(Component)]
struct BuildingModelNode {
    building_root: Entity,
    name: String,
}

#[derive(Component, Default)]
struct BuildingModelNodeProcessed;

#[derive(Component)]
struct AuthoredRotatingNode {
    building_root: Entity,
    age: u8,
    axis: Vec3,
    radians_per_second: f32,
}

#[derive(Component, Default)]
struct AuthoredRotatingNodeProcessed;

#[derive(Component)]
struct MainMenuRotatingDefinitions(Vec<stream_town_domain::RotatingNodeDef>);

#[derive(Component)]
struct MainMenuHiddenModelNodes(BTreeSet<String>);

#[derive(Component)]
struct MainMenuModelNodeProcessed;

/// Marks authored menu buildings whose imported mesh descendants must retain
/// ordinary opaque shadow casting.
#[derive(Component)]
struct MainMenuBuildingShadowRoot;

#[derive(Component)]
struct MainMenuBuildingShadowVerified;

#[derive(Component)]
struct MainMenuRotatingNode {
    axis: Vec3,
    radians_per_second: f32,
}

#[derive(Component, Default)]
struct MainMenuRotatingNodeProcessed;

#[derive(Component)]
struct TownSun;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CosmeticNodeKind {
    Eyes,
    Hair,
    FacialHair,
    Skin,
}

#[derive(Component)]
struct CosmeticNode {
    actor_root: Entity,
    kind: CosmeticNodeKind,
    index: u8,
}

#[derive(Component, Default)]
struct CosmeticNodeProcessed;

#[derive(Component)]
struct CosmeticRenderer {
    actor_root: Entity,
    kind: CosmeticNodeKind,
    base_material: Handle<CharacterMaterial>,
    applied_color: Option<u8>,
}

struct CosmeticMaterialVariant {
    base_material: Handle<CharacterMaterial>,
    kind: CosmeticNodeKind,
    color: u8,
    material: Handle<CharacterMaterial>,
}

#[derive(Resource, Default)]
struct CosmeticMaterialCache(Vec<CosmeticMaterialVariant>);

struct CharacterBaseMaterialVariant {
    source: Handle<StandardMaterial>,
    material: Handle<CharacterMaterial>,
}

#[derive(Resource, Default)]
struct CharacterBaseMaterialCache(Vec<CharacterBaseMaterialVariant>);

#[derive(Resource, Default)]
struct RoleActionAudioCache(BTreeMap<String, CachedRoleActionAudio>);

struct CachedRoleActionAudio {
    source: Handle<AudioSource>,
    wav: Arc<[u8]>,
}

#[derive(Component)]
struct AmbienceAudio;

#[derive(Resource, Default)]
struct WorldAudioRuntime {
    ambience: Option<Handle<AudioSource>>,
    ambience_wav: Option<Arc<[u8]>>,
    seagull_calls: Vec<Handle<AudioSource>>,
    seagull_call_wavs: Vec<Arc<[u8]>>,
}

#[derive(Component, Clone)]
struct NativeAnimationSpec {
    graph: Handle<AnimationGraph>,
    idle: AnimationNodeIndex,
    moving: AnimationNodeIndex,
}

#[derive(Component, Clone)]
struct NativeAnimationRequest {
    asset_path: String,
    animation_index: u32,
}

#[derive(Resource, Default)]
struct NativeAnimationCache(BTreeMap<(String, u32), NativeAnimationSpec>);

#[derive(Component, Clone)]
struct ConvertedAnimationSpec {
    controller: StableId,
    state: StableId,
    rig_scene: String,
}

#[derive(Component)]
struct ConvertedAnimationApplied;

/// Marks a converted rig whose imported scene hierarchy has finished spawning.
///
/// Animation targets attached before `WorldInstanceReady` are descendants of
/// an instance that Bevy may still replace while it finishes loading. Waiting
/// for this marker prevents the animation player from being despawned between
/// `Update` and `PostUpdate` by the scene spawner.
#[derive(Component)]
struct ConvertedAnimationInstanceReady;

#[derive(Component)]
struct ActorAnimationDriver {
    actor_root: Entity,
    idle: AnimationNodeIndex,
    moving: AnimationNodeIndex,
    current: MovementAnimationState,
}

#[derive(Component)]
struct ConvertedAnimationDriver {
    actor_root: Entity,
    controller: StableId,
    layers: Vec<ConvertedAnimationLayerDriver>,
    last_alive: Option<bool>,
    active_action: Option<String>,
    transient_carry_visible: bool,
}

struct ConvertedAnimationLayerDriver {
    fallback_state: StableId,
    runtime: AnimationControllerRuntime,
    nodes: BTreeMap<StableId, AnimationNodeIndex>,
    active: Vec<(AnimationNodeIndex, f32)>,
    applied: Vec<ConvertedAnimationPlayback>,
    crossfade: Option<ConvertedAnimationCrossfade>,
    state_offset: f32,
    event_elapsed: BTreeMap<StableId, f32>,
}

#[derive(Clone)]
struct ConvertedAnimationLayerTemplate {
    fallback_state: StableId,
    nodes: BTreeMap<StableId, AnimationNodeIndex>,
}

struct CachedConvertedAnimation {
    graph: Handle<AnimationGraph>,
    layers: Vec<ConvertedAnimationLayerTemplate>,
}

#[derive(Resource, Default)]
struct ConvertedAnimationCache(BTreeMap<(StableId, StableId, String), CachedConvertedAnimation>);

#[derive(Clone)]
struct GateAnimationClipRequest {
    asset_path: String,
    animation_index: u32,
    speed: f32,
    transition_seconds: f32,
}

struct GateAnimationContract {
    controller: StableId,
    model_root_name: &'static str,
    animation_root_name: &'static str,
    open: GateAnimationClipRequest,
    close: GateAnimationClipRequest,
}

#[derive(Clone)]
struct CachedGateAnimation {
    graph: Handle<AnimationGraph>,
    open: AnimationNodeIndex,
    close: AnimationNodeIndex,
}

#[derive(Resource, Default)]
struct GateAnimationCache(BTreeMap<(String, u32, String, u32), CachedGateAnimation>);

#[derive(Component)]
struct GateAnimationBinding {
    age: u8,
    animation_root: Entity,
}

#[derive(Component)]
struct GateAnimationDriver {
    building_root: Entity,
    open: AnimationNodeIndex,
    close: AnimationNodeIndex,
    open_speed: f32,
    close_speed: f32,
    open_transition_seconds: f32,
    close_transition_seconds: f32,
    is_open: bool,
}

#[derive(Clone)]
struct ConvertedAnimationCrossfade {
    source: Vec<ConvertedAnimationPlayback>,
    elapsed: f32,
    duration: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ConvertedAnimationPlayback {
    node: AnimationNodeIndex,
    weight: f32,
    speed: f32,
    looping: bool,
}

#[derive(Clone, Debug)]
struct PendingRoleActionAudio {
    actor: StableId,
    clip: StableId,
    display_name: String,
    position: Vec3,
}

#[derive(Component, Clone)]
struct MaterialOverrideSpec {
    fallback: Option<ResolvedMaterialHandle>,
    model_materials: BTreeMap<String, ResolvedMaterialHandle>,
    renderer_materials: Vec<ResolvedRendererMaterialBinding>,
    suppress_self_shadows: bool,
}

#[derive(Clone)]
struct ResolvedRendererMaterialBinding {
    target_path: String,
    materials: BTreeMap<String, ResolvedMaterialHandle>,
}

#[derive(Component)]
struct MaterialOverrideApplied;

#[derive(Component)]
struct BuildingMaterialInstanced;

pub use app::StreamTownGamePlugin;
pub use presentation::animation::preview_animation_asset_for_rig;

pub use bootstrap::{
    adaptive_music_preview_program, load_player_settings, load_runtime_config,
    player_settings_path, run, runtime_config_path, save_runtime_config,
};

fn locate_asset_root() -> PathBuf {
    let configured = std::env::var_os("STREAM_TOWN_ASSET_ROOT").map(PathBuf::from);
    let current = std::env::current_dir().ok().map(|path| path.join("assets"));
    let executable = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|path| path.join("assets")));
    let development = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    configured
        .into_iter()
        .chain(current)
        .chain(executable)
        .chain([development])
        .find(|path| path.is_dir())
        .unwrap_or_else(|| PathBuf::from("assets"))
}

fn embedded_content() -> ContentCatalog {
    let content: ContentCatalog =
        ron::from_str(include_str!("../../../assets/content/catalog.ron"))
            .expect("checked-in Stream Town content catalog must parse");
    content
        .validate()
        .expect("checked-in Stream Town content catalog must validate");
    content
}

fn embedded_presentation() -> PresentationCatalog {
    let presentation: PresentationCatalog =
        ron::from_str(include_str!("../../../assets/content/presentation.ron"))
            .expect("checked-in Stream Town presentation catalog must parse");
    presentation
        .validate()
        .expect("checked-in Stream Town presentation catalog must validate");
    presentation
}

fn setup_rendering(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    settings: Res<RuntimePlayerSettings>,
    presentation: Res<RuntimePresentation>,
    asset_server: Option<Res<AssetServer>>,
    core_assets: CoreRenderAssets,
    terrain_materials: Option<ResMut<Assets<TerrainMaterial>>>,
    water_materials: Option<ResMut<Assets<WaterMaterial>>>,
    building_materials: Option<ResMut<Assets<BuildingMaterial>>>,
    scene_materials: SceneMaterialAssets,
    specialty_materials: SpecialtyMaterialAssets,
    tree_materials: Option<ResMut<Assets<TreeMaterial>>>,
    grass_materials: Option<ResMut<Assets<GrassMaterial>>>,
    critter_materials: Option<ResMut<Assets<CritterMaterial>>>,
    flag_materials: Option<ResMut<Assets<FlagMaterial>>>,
) {
    let (
        CoreRenderAssets {
            images: Some(mut images),
            meshes: Some(mut meshes),
            materials: Some(mut materials),
        },
        Some(mut terrain_materials),
        Some(mut water_materials),
        Some(mut building_materials),
        SceneMaterialAssets {
            cloud: Some(mut cloud_materials),
            menu_sky: Some(mut menu_sky_materials),
            godray: Some(mut godray_materials),
        },
        SpecialtyMaterialAssets {
            giraffe: Some(mut giraffe_materials),
            bounds: Some(mut bounds_materials),
            character: Some(mut character_materials),
        },
        Some(mut tree_materials),
        Some(mut grass_materials),
        Some(mut critter_materials),
        Some(mut flag_materials),
    ) = (
        core_assets,
        terrain_materials,
        water_materials,
        building_materials,
        scene_materials,
        specialty_materials,
        tree_materials,
        grass_materials,
        critter_materials,
        flag_materials,
    )
    else {
        commands.insert_resource(RenderAssets::default());
        return;
    };
    let material_closeup = std::env::var_os("STREAM_TOWN_SMOKE_CLOSEUP").is_some();
    let animation_closeup = std::env::var_os("STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP").is_some();
    let resource_closeup = std::env::var_os("STREAM_TOWN_SMOKE_RESOURCE_CLOSEUP").is_some();
    let healing_closeup = std::env::var_os("STREAM_TOWN_SMOKE_HEALING_VFX").is_some();
    let combat_closeup = std::env::var_os("STREAM_TOWN_SMOKE_COMBAT_VFX").is_some();
    let building_closeup = std::env::var_os("STREAM_TOWN_SMOKE_BUILDING_VFX").is_some()
        || std::env::var_os("STREAM_TOWN_SMOKE_CHIMNEY").is_some();
    let ping_closeup = std::env::var_os("STREAM_TOWN_SMOKE_PING").is_some();
    let foliage_closeup = std::env::var_os("STREAM_TOWN_SMOKE_FOLIAGE").is_some();
    let shoreline_closeup = std::env::var_os("STREAM_TOWN_SMOKE_SHORELINE").is_some();
    let overlay_closeup = std::env::var_os("STREAM_TOWN_SMOKE_OVERLAYS").is_some();
    let seagull_closeup = std::env::var_os("STREAM_TOWN_SMOKE_SEAGULL").is_some();
    let flag_closeup = std::env::var_os("STREAM_TOWN_SMOKE_FLAG").is_some();
    let godray_closeup = std::env::var_os("STREAM_TOWN_SMOKE_GODRAY").is_some();
    let giraffe_closeup = std::env::var_os("STREAM_TOWN_SMOKE_GIRAFFE").is_some();
    let live_pet_closeup = std::env::var_os("STREAM_TOWN_SMOKE_PET").is_some();
    let placement_closeup = std::env::var_os("STREAM_TOWN_SMOKE_PLACEMENT").is_some();
    let fish_school_closeup = std::env::var_os("STREAM_TOWN_SMOKE_FISH_SCHOOL").is_some();
    let role_audio_closeup = std::env::var_os("STREAM_TOWN_SMOKE_ROLE_AUDIO").is_some();
    let smoke_viewport_height = if material_closeup {
        Some(96.0)
    } else if animation_closeup {
        Some(6.0)
    } else if resource_closeup {
        Some(24.0)
    } else if healing_closeup {
        Some(42.0)
    } else if combat_closeup {
        Some(48.0)
    } else if building_closeup {
        Some(58.0)
    } else if ping_closeup {
        Some(30.0)
    } else if foliage_closeup {
        Some(45.0)
    } else if shoreline_closeup {
        Some(48.0)
    } else if overlay_closeup {
        Some(105.0)
    } else if seagull_closeup {
        Some(80.0)
    } else if flag_closeup {
        Some(52.0)
    } else if godray_closeup {
        Some(86.0)
    } else if giraffe_closeup {
        Some(42.0)
    } else if live_pet_closeup {
        Some(10.0)
    } else if placement_closeup {
        Some(52.0)
    } else if fish_school_closeup {
        Some(42.0)
    } else if role_audio_closeup {
        Some(30.0)
    } else {
        None
    };
    let initial_camera_transform = default_town_camera_transform();
    let initial_projection = smoke_viewport_height.map_or_else(
        || town_camera_projection(UNITY_TOWN_CAMERA_FOV_DEGREES),
        |viewport_height| {
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical { viewport_height },
                near: -2_000.0,
                far: 2_000.0,
                ..OrthographicProjection::default_3d()
            })
        },
    );
    commands.spawn((
        TownCamera,
        TownCameraControllerRuntime::new(initial_camera_transform),
        Camera3d::default(),
        // The shipping camera is orthographic and all local lights occupy a
        // shallow top-down depth band. Spending Bevy's default 24 slices on Z
        // leaves very coarse screen tiles, causing each night light to be
        // evaluated across far more fragments than it can affect.
        ClusterConfig::FixedZ {
            total: 4_096,
            z_slices: 1,
            z_config: ClusterZConfig {
                first_slice_depth: 5.0,
                far_z_mode: ClusterFarZMode::Constant(4_000.0),
            },
            dynamic_resizing: true,
        },
        IsDefaultUiCamera,
        SpatialListener::new(0.2),
        initial_projection,
        AmbientLight {
            color: Color::srgb(0.70, 0.82, 0.92),
            brightness: in_game_ambient_brightness(90.0),
            ..default()
        },
        DistanceFog {
            color: Color::srgba(0.58, 0.72, 0.78, 0.10),
            falloff: FogFalloff::Linear {
                start: 560.0,
                end: 940.0,
            },
            ..default()
        },
        initial_camera_transform,
    ));
    commands.spawn((
        TownSun,
        DirectionalLight {
            illuminance: 14_000.0,
            shadow_maps_enabled: true,
            // Converted low-poly assets use small, centimetre-derived scene
            // transforms. A little more bias prevents coplanar faces from
            // intermittently shadowing themselves at town-camera distances.
            shadow_depth_bias: 0.04,
            shadow_normal_bias: 2.5,
            ..default()
        },
        in_game_sun_transform(),
    ));
    let authored_building =
        building_materials.add(building_material(&presentation.0, asset_server.as_deref()));
    let clouds = cloud_materials.add(cloud_material(&presentation.0, asset_server.as_deref()));
    let menu_sky = menu_sky_materials.add(menu_sky_material());
    let menu_cloud = materials.add(StandardMaterial {
        base_color: Color::srgb(0.96, 0.98, 1.0),
        perceptual_roughness: 1.0,
        unlit: true,
        ..default()
    });
    let menu_ocean_floor = materials.add(StandardMaterial {
        base_color: Color::srgb(0.025, 0.20, 0.30),
        perceptual_roughness: 1.0,
        unlit: true,
        ..default()
    });
    let godrays = godray_materials.add(godray_material(&presentation.0));
    let giraffe = giraffe_materials.add(giraffe_material(&presentation.0, asset_server.as_deref()));
    let authored_bounds = bounds_materials.add(bounds_material(&presentation.0, None));
    let placement_valid = bounds_materials.add(bounds_material(
        &presentation.0,
        Some(BUILDING_PLACEMENT_SUCCESS_COLOR),
    ));
    let placement_invalid = bounds_materials.add(bounds_material(
        &presentation.0,
        Some(BUILDING_PLACEMENT_FAIL_COLOR),
    ));
    let tree_material = tree_material(&presentation.0, asset_server.as_deref());
    let tree_wind = tree_material.extension.parameters.wind_controls;
    let tree = tree_materials.add(tree_material);
    // Menu foliage keeps a dedicated solid-colour material so the shared atlas
    // cannot leak blue, but remains lit so it responds to the authored sun and
    // receives the same ordinary world shadows as gameplay foliage.
    let menu_tree = materials.add(menu_tree_material());
    let grass_material = grass_material(&presentation.0, asset_server.as_deref());
    let grass_wind = grass_material.extension.parameters.wind_controls;
    let grass = grass_materials.add(grass_material);
    let critter = critter_materials.add(critter_material(&presentation.0, asset_server.as_deref()));
    let flag = flag_materials.add(flag_material(&presentation.0, asset_server.as_deref()));
    let game_logo = presentation_texture_handle(
        &presentation.0,
        asset_server.as_deref(),
        GAME_LOGO_TEXTURE_PATH,
    );
    let loading_screen = presentation_texture_handle(
        &presentation.0,
        asset_server.as_deref(),
        LOADING_SCREEN_TEXTURE_PATH,
    );
    let loading_overlay = presentation_texture_handle(
        &presentation.0,
        asset_server.as_deref(),
        LOADING_OVERLAY_TEXTURE_PATH,
    );
    let loading_icon = presentation_texture_handle(
        &presentation.0,
        asset_server.as_deref(),
        LOADING_ICON_TEXTURE_PATH,
    );
    let ui_font = asset_server
        .as_deref()
        .map(|asset_server| asset_server.load(UI_FONT_ASSET_PATH));
    let ui_display_font = asset_server
        .as_deref()
        .map(|asset_server| asset_server.load(UI_DISPLAY_FONT_ASSET_PATH));
    let main_menu_textures = MAIN_MENU_TEXTURE_PATHS
        .iter()
        .chain(GAME_MENU_TEXTURE_PATHS.iter())
        .chain(std::iter::once(&SETTINGS_BACKGROUND_TEXTURE_PATH))
        .filter_map(|source_path| {
            presentation_texture_handle(&presentation.0, asset_server.as_deref(), source_path)
                .map(|handle| ((*source_path).to_owned(), handle))
        })
        .collect();
    let top_bar_textures = TOP_BAR_TEXTURE_PATHS
        .iter()
        .filter_map(|source_path| {
            presentation_texture_handle(&presentation.0, asset_server.as_deref(), source_path)
                .map(|handle| ((*source_path).to_owned(), handle))
        })
        .collect();
    let selection_panel_textures = SELECTION_PANEL_TEXTURE_PATHS
        .iter()
        .filter_map(|source_path| {
            presentation_texture_handle(&presentation.0, asset_server.as_deref(), source_path)
                .map(|handle| ((*source_path).to_owned(), handle))
        })
        .collect();
    let vote_textures = VOTE_TEXTURE_PATHS
        .iter()
        .copied()
        .chain(std::iter::once(RULER_VOTE_TIMER_UNFILLED_PATH))
        .filter_map(|source_path| {
            presentation_texture_handle(&presentation.0, asset_server.as_deref(), source_path)
                .map(|handle| (source_path.to_owned(), handle))
        })
        .collect();
    let objective_textures = OBJECTIVE_TEXTURE_PATHS
        .iter()
        .filter_map(|source_path| {
            presentation_texture_handle(&presentation.0, asset_server.as_deref(), source_path)
                .map(|handle| ((*source_path).to_owned(), handle))
        })
        .collect();
    let current_event_textures = CURRENT_EVENT_TEXTURE_PATHS
        .iter()
        .filter_map(|source_path| {
            presentation_texture_handle(&presentation.0, asset_server.as_deref(), source_path)
                .map(|handle| ((*source_path).to_owned(), handle))
        })
        .collect();
    let ui_slicers = presentation
        .0
        .textures
        .values()
        .filter_map(|texture| {
            texture.sprite_border.map(|border| {
                (
                    texture.source_path.clone(),
                    TextureSlicer {
                        border: BorderRect::from(border),
                        center_scale_mode: default(),
                        sides_scale_mode: default(),
                        max_corner_scale: 1.0,
                    },
                )
            })
        })
        .collect();
    let presentation_materials: BTreeMap<StableId, ResolvedMaterialHandle> = presentation
        .0
        .materials
        .iter()
        .map(|(id, material)| {
            let resolved = if material.source_path == BUILDING_MATERIAL_PATH {
                ResolvedMaterialHandle::Building(authored_building.clone())
            } else if material.source_path == CLOUD_MATERIAL_PATH {
                ResolvedMaterialHandle::Cloud(clouds.clone())
            } else if material.source_path == GODRAY_MATERIAL_PATH {
                ResolvedMaterialHandle::Godray(godrays.clone())
            } else if material.source_path == GIRAFFE_MATERIAL_PATH {
                ResolvedMaterialHandle::Giraffe(giraffe.clone())
            } else if material.source_path == BOUNDS_MATERIAL_PATH {
                ResolvedMaterialHandle::Bounds(authored_bounds.clone())
            } else if material.source_path == TREE_MATERIAL_PATH {
                ResolvedMaterialHandle::Tree(tree.clone())
            } else if material.source_path == GRASS_MATERIAL_PATH {
                ResolvedMaterialHandle::Grass(grass.clone())
            } else if material.source_path == CRITTER_MATERIAL_PATH {
                ResolvedMaterialHandle::Critter(critter.clone())
            } else if material.source_path == FLAG_MATERIAL_PATH {
                ResolvedMaterialHandle::Flag(flag.clone())
            } else if material.shader_source.as_deref() == Some(CHARACTER_UNITY_SHADER_PATH) {
                ResolvedMaterialHandle::Character(character_materials.add(character_material(
                    material,
                    &presentation.0,
                    asset_server.as_deref(),
                )))
            } else {
                let standard =
                    standard_material(material, &presentation.0, asset_server.as_deref());
                let standard = if id.as_str() == SELECTION_MASK_MATERIAL_ID {
                    selection_outline_material(standard)
                } else {
                    standard
                };
                ResolvedMaterialHandle::Standard(materials.add(standard))
            };
            (id.clone(), resolved)
        })
        .collect();
    let presentation_materials_by_source_path = presentation
        .0
        .materials
        .iter()
        .filter_map(|(id, material)| {
            presentation_materials
                .get(id)
                .map(|resolved| (material.source_path.clone(), resolved.clone()))
        })
        .collect();
    let selection = StableId::new(SELECTION_MASK_MATERIAL_ID)
        .ok()
        .and_then(|id| presentation_materials.get(&id))
        .and_then(|material| match material {
            ResolvedMaterialHandle::Standard(material) => Some(material.clone()),
            _ => None,
        })
        .unwrap_or_else(|| {
            materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.983_102, 0.0),
                emissive: LinearRgba::new(0.877_358_5, 0.836_833_4, 0.0, 1.0),
                alpha_mode: AlphaMode::Mask(0.5),
                depth_bias: SELECTION_OUTLINE_DEPTH_BIAS,
                ..default()
            })
        });
    let chimney_smoke = presentation
        .0
        .chimney_smoke_effects
        .iter()
        .map(|(id, effect)| {
            let mut gradient = |color: [f32; 4]| {
                (0..CHIMNEY_ALPHA_STEPS)
                    .map(|step| {
                        let progress = chimney_alpha_progress(step);
                        let alpha = effect.alpha_over_lifetime[0]
                            + (effect.alpha_over_lifetime[1] - effect.alpha_over_lifetime[0])
                                * progress;
                        materials.add(StandardMaterial {
                            base_color: Color::linear_rgba(
                                color[0],
                                color[1],
                                color[2],
                                color[3] * alpha,
                            ),
                            alpha_mode: AlphaMode::Blend,
                            perceptual_roughness: 0.5,
                            ..default()
                        })
                    })
                    .collect()
            };
            (
                id.clone(),
                [
                    gradient(effect.start_color_min),
                    gradient(effect.start_color_max),
                ],
            )
        })
        .collect();
    let healing_channel = presentation
        .0
        .healing_channel_effects
        .iter()
        .filter_map(|(id, effect)| {
            effect
                .color
                .sample(0.06)
                .map(|color| (id.clone(), materials.add(healing_material(color, 0.72))))
        })
        .collect();
    let healing_plus_materials = presentation
        .0
        .healing_burst_effects
        .iter()
        .map(|(id, effect)| {
            (
                id.clone(),
                healing_gradient_materials(&mut materials, &effect.plus_color),
            )
        })
        .collect();
    let healing_disc_materials = presentation
        .0
        .healing_burst_effects
        .iter()
        .map(|(id, effect)| {
            (
                id.clone(),
                healing_gradient_materials(&mut materials, &effect.disc_color),
            )
        })
        .collect();
    let healing_plus = presentation
        .0
        .healing_burst_effects
        .values()
        .next()
        .and_then(|effect| {
            asset_server.as_deref().map(|server| {
                server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset(effect.plus_model_asset_path.clone()),
                )
            })
        });
    let fish_school_mesh = presentation
        .0
        .fish_school_effects
        .values()
        .find(|effect| effect.source_path.ends_with(FISH_SCHOOL_PREFAB_SUFFIX))
        .and_then(|effect| {
            asset_server.as_deref().map(|server| {
                server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset(effect.model_asset_path.clone()),
                )
            })
        });
    let water = water_material(&presentation.0, asset_server.as_deref());
    let water_wind = water.extension.parameters.wind_speed_noise_alpha;
    let menu_water = main_menu_water_material(water.clone());
    commands.insert_resource(AccessibilityMotionDefaults {
        tree: tree_wind,
        grass: grass_wind,
        water: water_wind,
    });
    let window_height =
        f32::from(u16::try_from(settings.0.video.height.max(1)).unwrap_or(u16::MAX));
    let traversal_wear = images.add(traversal_wear_image(
        config.0.world.width,
        config.0.world.height,
    ));
    let path_surface = images.add(traversal_wear_image(
        config.0.world.width,
        config.0.world.height,
    ));
    commands.insert_resource(RenderAssets {
        cube: meshes.add(Cuboid::default()),
        chimney_particle: meshes.add(Sphere::new(0.5).mesh().ico(1).expect("valid icosphere")),
        actor_lod: meshes.add(Capsule3d::new(0.42, 1.45)),
        menu_sky_mesh: meshes.add(
            Sphere::new(1.0)
                .mesh()
                .ico(4)
                .expect("valid main-menu sky sphere"),
        ),
        cloud_plane: meshes.add(Plane3d::default().mesh().size(1.0, 1.0)),
        healing_ring: meshes.add(healing_ring_mesh(48)),
        healing_plus,
        fish_school_mesh,
        fish_school_material: critter,
        projectile_arrow_scene: asset_server.as_deref().map(|asset_server| {
            asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("migrated/models/Models/Combat/Arrow.glb"),
            )
        }),
        ground: terrain_materials.add(terrain_material(
            &presentation.0,
            &config.0,
            asset_server.as_deref(),
            Some(traversal_wear.clone()),
            Some(path_surface.clone()),
        )),
        traversal_wear,
        path_surface,
        water: water_materials.add(water),
        menu_water: water_materials.add(menu_water),
        menu_sky,
        wood: materials.add(Color::srgb(0.16, 0.46, 0.18)),
        ore: materials.add(Color::srgb(0.46, 0.50, 0.55)),
        food: materials.add(Color::srgb(0.74, 0.64, 0.18)),
        building: materials.add(Color::srgb(0.42, 0.26, 0.12)),
        construction: materials.add(StandardMaterial {
            base_color: Color::srgb(0.72, 0.51, 0.24),
            perceptual_roughness: 0.88,
            ..default()
        }),
        streetlight_lamp: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.72, 0.24),
            emissive: LinearRgba::new(8.0, 3.0, 0.35, 1.0),
            perceptual_roughness: 0.35,
            ..default()
        }),
        regeneration_buildings: BTreeMap::from([
            (
                "11111111111111111111111111111111".to_owned(),
                materials.add(StandardMaterial {
                    base_color: Color::srgb(0.08, 0.30, 0.12),
                    perceptual_roughness: 0.82,
                    ..default()
                }),
            ),
            (
                "22222222222222222222222222222222".to_owned(),
                materials.add(StandardMaterial {
                    base_color: Color::srgb(0.30, 0.32, 0.35),
                    perceptual_roughness: 0.86,
                    ..default()
                }),
            ),
            (
                "33333333333333333333333333333333".to_owned(),
                materials.add(StandardMaterial {
                    base_color: Color::srgb(0.20, 0.08, 0.25),
                    perceptual_roughness: 0.82,
                    ..default()
                }),
            ),
        ]),
        placement_valid,
        placement_invalid,
        enemy_idle: materials.add(Color::srgb(0.72, 0.12, 0.12)),
        enemy_moving: materials.add(Color::srgb(1.0, 0.28, 0.22)),
        player_idle: materials.add(Color::srgb(0.35, 0.72, 0.95)),
        player_moving: materials.add(Color::srgb(0.52, 0.86, 1.0)),
        selection,
        rain: materials.add(StandardMaterial {
            base_color: Color::srgba(0.36, 0.66, 0.95, 0.62),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        snow: materials.add(StandardMaterial {
            base_color: Color::srgba(0.94, 0.98, 1.0, 0.92),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        projectile: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.58, 0.12),
            emissive: LinearRgba::new(3.5, 1.1, 0.08, 1.0),
            unlit: true,
            ..default()
        }),
        projectile_arrow: materials.add(StandardMaterial {
            base_color: Color::srgba(0.25, 0.25, 0.25, 0.62),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        projectile_necrotic: materials.add(StandardMaterial {
            base_color: Color::srgba(0.48, 0.08, 1.0, 0.84),
            emissive: LinearRgba::new(1.2, 0.03, 4.2, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        impact_physical: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 1.0, 1.0, 0.66),
            emissive: LinearRgba::new(1.2, 1.2, 1.2, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        building_smoke: materials.add(StandardMaterial {
            base_color: Color::srgba(0.21, 0.21, 0.21, 0.69),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        chimney_smoke,
        building_spark: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.34, 0.02, 0.96),
            emissive: LinearRgba::new(7.5, 0.48, 0.01, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        building_fire: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.18, 0.01, 0.86),
            emissive: LinearRgba::new(8.5, 0.32, 0.01, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        building_upgrade: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.66, 0.05, 0.9),
            emissive: LinearRgba::new(7.0, 2.1, 0.05, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        healing_green: materials.add(StandardMaterial {
            base_color: Color::srgba(0.18, 1.0, 0.12, 0.74),
            emissive: LinearRgba::new(0.28, 3.5, 0.14, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        healing_channel,
        healing_plus_materials,
        healing_disc_materials,
        authored_building,
        clouds,
        menu_cloud,
        menu_ocean_floor,
        tree,
        menu_tree,
        grass,
        game_logo,
        loading_screen,
        loading_overlay,
        loading_icon,
        ui_font,
        ui_display_font,
        main_menu_textures,
        top_bar_textures,
        selection_panel_textures,
        vote_textures,
        objective_textures,
        current_event_textures,
        ui_slicers,
        main_ui_scale: window_height / UNITY_MAIN_UI_REFERENCE_HEIGHT,
        settings_ui_scale: window_height / UNITY_SETTINGS_UI_REFERENCE_HEIGHT,
        presentation_materials,
        presentation_materials_by_source_path,
    });
}

fn apply_player_settings(
    settings: Res<RuntimePlayerSettings>,
    mut commands: Commands,
    mut cameras: Query<(Entity, &mut Projection), With<TownCamera>>,
    mut lights: Query<&mut DirectionalLight>,
    mut shadow_map: Option<ResMut<DirectionalLightShadowMap>>,
    mut winit: Option<ResMut<WinitSettings>>,
) {
    let benchmarking = std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some();
    let msaa = player_msaa(&settings.0);
    for (entity, mut projection) in &mut cameras {
        if let Projection::Perspective(perspective) = &mut *projection {
            perspective.fov = f32::from(settings.0.camera.field_of_view_degrees).to_radians();
        }
        let mut entity_commands = commands.entity(entity);
        entity_commands.insert(msaa);
        entity_commands.remove::<Fxaa>();
        entity_commands.remove::<Smaa>();
        match settings.0.video.post_process_aa {
            PostProcessAntiAliasing::None => {}
            PostProcessAntiAliasing::Fxaa => {
                entity_commands.insert(Fxaa::default());
            }
            PostProcessAntiAliasing::Smaa => {
                entity_commands.insert(Smaa::default());
            }
        }
        if settings.0.video.ambient_occlusion {
            entity_commands.insert(ScreenSpaceAmbientOcclusion::default());
        } else {
            entity_commands.remove::<ScreenSpaceAmbientOcclusion>();
        }
    }
    for mut light in &mut lights {
        light.shadow_maps_enabled = settings.0.video.shadows_enabled;
    }
    if let Some(shadow_map) = shadow_map.as_deref_mut() {
        shadow_map.size = usize::from(settings.0.video.shadow_map_resolution);
    }
    if let Some(winit) = winit.as_deref_mut() {
        winit.focused_mode = if benchmarking {
            UpdateMode::Continuous
        } else {
            settings
                .0
                .video
                .fps_limit
                .map_or(UpdateMode::Continuous, |limit| {
                    UpdateMode::reactive(Duration::from_secs_f64(1.0 / f64::from(limit)))
                })
        };
        if animation_binding_diagnostics_enabled() {
            // Automated GPU smoke windows cannot take focus away from the
            // desktop host. Keep only this explicit diagnostic path updating
            // continuously so elapsed/joint samples represent real frames.
            winit.unfocused_mode = UpdateMode::Continuous;
        }
    }
}

fn sync_primary_window_settings(
    settings: Res<RuntimePlayerSettings>,
    mut windows: Query<(&mut Window, Option<&OnMonitor>), With<PrimaryWindow>>,
) {
    let Ok((mut window, current_monitor)) = windows.single_mut() else {
        return;
    };
    // Direct-broadcast mode deliberately hides the swapchain window. Resizing
    // that hidden surface races DX12 capture on some drivers, so defer changes
    // until the local window becomes visible again.
    if !window.visible {
        return;
    }
    let benchmarking = std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some();
    let monitor = current_monitor.map(|monitor| monitor.0);
    let desired_mode = if benchmarking {
        WindowMode::Windowed
    } else {
        bootstrap::player_window_mode(settings.0.video.display_mode, monitor)
    };
    let desired_present_mode = if benchmarking {
        PresentMode::Immediate
    } else if settings.0.video.vsync {
        PresentMode::AutoVsync
    } else {
        PresentMode::AutoNoVsync
    };
    let desired_width = f32::from(u16::try_from(settings.0.video.width).unwrap_or(u16::MAX));
    let desired_height = f32::from(u16::try_from(settings.0.video.height).unwrap_or(u16::MAX));
    if window.mode != desired_mode {
        window.mode = desired_mode;
    }
    if window.present_mode != desired_present_mode {
        window.present_mode = desired_present_mode;
    }
    if (window.resolution.width() - desired_width).abs() > 0.5
        || (window.resolution.height() - desired_height).abs() > 0.5
    {
        window.resolution.set(desired_width, desired_height);
    }
}

fn apply_authored_ui_fonts(
    render: Res<RenderAssets>,
    mut texts: Query<(&mut TextFont, Option<&UiDisplayFont>), Added<TextFont>>,
) {
    for (mut text_font, display) in &mut texts {
        let authored = if display.is_some() {
            render.ui_display_font.as_ref()
        } else {
            render.ui_font.as_ref()
        };
        if let Some(authored) = authored {
            text_font.font = FontSource::Handle(authored.clone());
        }
    }
}

fn sync_authored_post_processing(
    mut commands: Commands,
    state: Res<State<GameState>>,
    settings: Res<RuntimePlayerSettings>,
    config: Res<RuntimeConfig>,
    catalog: Res<RuntimePresentation>,
    simulation: Option<Res<SimulationRuntime>>,
    environment: Res<EnvironmentPresentation>,
    mut runtime: ResMut<PostProcessPresentation>,
    mut cameras: TownPostProcessQuery,
) {
    let Ok((camera, color_grading, has_hdr, has_bloom, has_vignette, has_motion_blur, tonemapping)) =
        cameras.single_mut()
    else {
        return;
    };
    let daylight = if *state.get() == GameState::InGame {
        simulation.as_deref().map_or_else(
            || environment.daylight_bits.map_or(1.0, f32::from_bits),
            |simulation| {
                config
                    .0
                    .time
                    .sample(simulation.0.elapsed_seconds)
                    .daylight
                    .clamp(0.0, 1.0)
            },
        )
    } else {
        1.0
    };
    let daylight_bits = daylight_signature(daylight);
    let signature = (
        *state.get(),
        daylight_bits,
        i32::from_ne_bytes(settings.0.video.brightness_ev.to_ne_bytes()),
        i32::from_ne_bytes(settings.0.video.gamma.to_ne_bytes()),
    );
    if runtime.applied == Some(signature) {
        return;
    }
    let static_stack_changed = runtime
        .applied
        .is_none_or(|(applied_state, _, _, _)| applied_state != *state.get());

    let scene_path = match state.get() {
        GameState::InGame => Some(WORLD_SCENE_PATH),
        GameState::Credits => Some(CREDITS_SCENE_PATH),
        GameState::Boot | GameState::MainMenu | GameState::WorldLoading => None,
    };
    let stack = scene_path.map_or_else(Vec::new, |scene| {
        authored_post_process_stack(&catalog.0, scene, daylight)
    });
    let primary = stack
        .iter()
        .find_map(|(profile, weight)| (*weight > f32::EPSILON).then_some(*profile));
    let mut entity = commands.entity(camera);
    let next_color_grading = color_grading_for_state(&settings.0, &stack, *state.get());
    if let Some(mut color_grading) = color_grading {
        *color_grading = next_color_grading;
    } else {
        entity.insert(next_color_grading);
    }
    if primary.is_some() {
        if !has_hdr {
            entity.insert(Hdr);
        }
    } else if has_hdr {
        entity.remove::<Hdr>();
    }

    if let Some(bloom) = primary.and_then(|profile| profile.bloom) {
        if !has_bloom || static_stack_changed {
            entity.insert(Bloom {
                // Unity URP's authored intensity is not numerically equivalent to
                // Bevy's full-resolution additive blend. Mapping it onto Bevy's
                // natural, energy-conserving preset prevents white shoreline and
                // water highlights from flooding the entire HDR buffer.
                intensity: (bloom.intensity * 0.15).clamp(0.0, 0.35),
                low_frequency_boost: bloom.scatter,
                prefilter: BloomPrefilter {
                    threshold: bloom.threshold,
                    threshold_softness: 0.2,
                },
                composite_mode: BloomCompositeMode::EnergyConserving,
                ..Bloom::NATURAL
            });
        }
    } else if has_bloom {
        entity.remove::<Bloom>();
    }
    if let Some(vignette) = primary.and_then(|profile| profile.vignette) {
        if !has_vignette || static_stack_changed {
            entity.insert(Vignette {
                intensity: vignette.intensity,
                radius: 0.75,
                smoothness: vignette.smoothness,
                roundness: if vignette.rounded { 1.0 } else { 0.0 },
                center: Vec2::from_array(vignette.center),
                edge_compensation: 1.0,
                color: Color::srgba(
                    vignette.color[0],
                    vignette.color[1],
                    vignette.color[2],
                    vignette.color[3],
                ),
            });
        }
    } else if has_vignette {
        entity.remove::<Vignette>();
    }
    if motion_blur_supported()
        && let Some(motion_blur) = primary.and_then(|profile| profile.motion_blur)
    {
        if !has_motion_blur || static_stack_changed {
            entity.insert(MotionBlur {
                shutter_angle: motion_blur.intensity,
                samples: u32::from(motion_blur.quality),
            });
        }
    } else if has_motion_blur {
        entity.remove::<MotionBlur>();
    }
    let next_tonemapping = match primary.and_then(|profile| profile.tonemapping) {
        Some(PostProcessTonemapping::Aces) => Tonemapping::AcesFitted,
        Some(PostProcessTonemapping::Neutral) => Tonemapping::SomewhatBoringDisplayTransform,
        Some(PostProcessTonemapping::None) | None => Tonemapping::None,
    };
    if let Some(mut tonemapping) = tonemapping {
        if *tonemapping != next_tonemapping {
            *tonemapping = next_tonemapping;
        }
    } else {
        entity.insert(next_tonemapping);
    }
    runtime.applied = Some(signature);
}

const fn motion_blur_supported() -> bool {
    // Bevy 0.19's motion-blur shader contains a varying loop that the
    // self-contained Windows FXC path cannot compile. Other platforms retain
    // the authored effect; Windows keeps the remainder of the post stack.
    !cfg!(target_os = "windows")
}

fn authored_post_process_stack<'a>(
    catalog: &'a PresentationCatalog,
    scene_path: &str,
    daylight: f32,
) -> Vec<(&'a PostProcessProfileDef, f32)> {
    catalog
        .scene_post_process
        .get(scene_path)
        .into_iter()
        .flatten()
        .filter_map(|binding| {
            catalog
                .post_process_profiles
                .get(&binding.profile)
                .map(|profile| {
                    let weight = if binding.inverse_daylight {
                        binding.weight * (1.0 - daylight.clamp(0.0, 1.0))
                    } else {
                        binding.weight
                    };
                    (profile, weight.clamp(0.0, 1.0))
                })
        })
        .collect()
}

fn authored_color_grading(
    settings: &PlayerSettings,
    stack: &[(&PostProcessProfileDef, f32)],
) -> ColorGrading {
    let mut exposure = 0.0;
    let mut hue_degrees = 0.0;
    let mut saturation = 0.0;
    for (profile, weight) in stack {
        let Some(adjustments) = profile.color_adjustments else {
            continue;
        };
        exposure += (adjustments.post_exposure - exposure) * weight;
        hue_degrees += (adjustments.hue_shift_degrees - hue_degrees) * weight;
        saturation += (adjustments.saturation - saturation) * weight;
    }
    let mut grading = ColorGrading::default();
    grading.global.exposure = settings.video.brightness_ev + exposure;
    grading.global.hue = hue_degrees.to_radians();
    grading.global.post_saturation = (1.0 + saturation / 100.0).max(0.0);
    let gamma = (1.0 + settings.video.gamma * 0.1).clamp(0.5, 1.5);
    grading.shadows.gamma = gamma;
    grading.midtones.gamma = gamma;
    grading.highlights.gamma = gamma;
    grading
}

fn color_grading_for_state(
    settings: &PlayerSettings,
    stack: &[(&PostProcessProfileDef, f32)],
    state: GameState,
) -> ColorGrading {
    let mut grading = authored_color_grading(settings, stack);
    if state == GameState::MainMenu {
        // User testing established that the authored menu reaches its intended
        // luminance at the old -1.5 brightness setting. Make that the scene's
        // neutral baseline so setting 0 is now the correct exposure.
        grading.global.exposure += MAIN_MENU_BASELINE_EXPOSURE_EV;
    } else if state == GameState::InGame {
        // User testing established that the intended world luminance was the
        // old +0.5 setting. Treat it as the authored scene baseline so neutral
        // brightness now produces the approved image.
        grading.global.exposure += IN_GAME_BASELINE_EXPOSURE_EV;
        // Unity URP's ACES output retains slightly more chroma than Bevy's
        // fitted implementation for this low-poly palette. This narrow output
        // compensation restores the authored grass/building separation without
        // changing source material values or deterministic world content.
        grading.global.post_saturation *= IN_GAME_SATURATION_MULTIPLIER;
    }
    grading
}

fn authored_rgb_filter(stack: &[(&PostProcessProfileDef, f32)]) -> [f32; 4] {
    let mut filter = [1.0_f32; 4];
    for (profile, weight) in stack {
        let Some(adjustments) = profile.color_adjustments else {
            continue;
        };
        for (current, target) in filter.iter_mut().zip(adjustments.color_filter) {
            *current += (target - *current) * weight;
        }
    }
    filter
}

fn player_msaa(settings: &PlayerSettings) -> Msaa {
    // Bevy 0.19's SSAO render node requires multisampling to be disabled.
    // Retain the authored MSAA preference and apply it whenever SSAO is off.
    if settings.video.ambient_occlusion {
        Msaa::Off
    } else {
        Msaa::from_samples(u32::from(settings.video.msaa_samples))
    }
}

fn setup_world_audio_sources(
    audio_sources: Option<ResMut<Assets<AudioSource>>>,
    mut world_audio: ResMut<WorldAudioRuntime>,
) {
    let Some(mut audio_sources) = audio_sources else {
        return;
    };
    let ambience_wav: Arc<[u8]> =
        procedural_ambience_wav(PROCEDURAL_AUDIO_SAMPLE_RATE, 24.0).into();
    let ambience = audio_sources.add(AudioSource {
        bytes: ambience_wav.clone(),
    });
    let seagull_call_wavs = (0_u8..3)
        .map(|variant| {
            Arc::<[u8]>::from(procedural_seagull_call_wav(
                variant,
                PROCEDURAL_AUDIO_SAMPLE_RATE,
            ))
        })
        .collect::<Vec<_>>();
    let seagull_calls = seagull_call_wavs
        .iter()
        .map(|wav| audio_sources.add(AudioSource { bytes: wav.clone() }))
        .collect();
    world_audio.ambience = Some(ambience);
    world_audio.ambience_wav = Some(ambience_wav);
    world_audio.seagull_calls = seagull_calls;
    world_audio.seagull_call_wavs = seagull_call_wavs;
}

fn drive_world_audio(
    mut commands: Commands,
    player_settings: Res<RuntimePlayerSettings>,
    simulation: Option<Res<SimulationRuntime>>,
    world_audio: Res<WorldAudioRuntime>,
    ambience_players: Query<Entity, With<AmbienceAudio>>,
    mut ambience_sinks: Query<&mut AudioSink, With<AmbienceAudio>>,
    #[cfg(target_os = "windows")] native_audio: Res<direct_broadcast::NativeGameAudioRouting>,
) {
    if simulation.is_none() {
        #[cfg(target_os = "windows")]
        native_audio.clear_looping();
        for entity in &ambience_players {
            commands.entity(entity).try_despawn();
        }
        return;
    }
    let gain = AMBIENCE_GAIN * player_settings.0.audio.master * player_settings.0.audio.ambience;
    #[cfg(target_os = "windows")]
    let local_gain = if native_audio.local_monitor_enabled() {
        gain
    } else {
        0.0
    };
    #[cfg(not(target_os = "windows"))]
    let local_gain = gain;
    #[cfg(target_os = "windows")]
    if let Some(wav) = world_audio.ambience_wav.as_deref() {
        native_audio.set_looping_pcm16_wav("world:ambience", wav, gain);
    }
    for mut sink in &mut ambience_sinks {
        sink.set_volume(Volume::Linear(local_gain));
    }
    if ambience_players.is_empty()
        && let Some(source) = world_audio.ambience.clone()
    {
        commands.spawn((
            Name::new("Procedural seasonal ambience"),
            WorldEntity,
            AmbienceAudio,
            AudioPlayer(source),
            PlaybackSettings::LOOP.with_volume(Volume::Linear(local_gain)),
        ));
    }
}

fn spawn_seagull(
    commands: &mut Commands,
    render: &RenderAssets,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    world_seed: u64,
) {
    let (start, end) = deterministic_seagull_leg(world_seed, 0);
    let critter_material = standalone_material_override(render, CRITTER_MATERIAL_ID);
    let mut flight = commands.spawn((
        Name::new("SeagulSpawner (Unity parity)"),
        WorldEntity,
        SeagullFlight {
            start,
            end,
            elapsed_seconds: 0.0,
            leg_serial: 0,
            call_elapsed_seconds: 0.0,
            call_wait_seconds: 0.0,
            call_serial: 0,
            world_seed,
        },
        Visibility::default(),
        seagull_flight_transform(start, end),
    ));
    flight.with_children(|parent| {
        let model_transform = seagull_model_transform();
        if let Some(asset_server) = asset_server
            && converted_asset_exists(asset_root, SEAGULL_MODEL_PATH)
        {
            let mut visual = parent.spawn((
                Name::new("Critter_Seagull_01"),
                WorldAssetRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset(SEAGULL_MODEL_PATH.to_owned())),
                ),
                model_transform,
            ));
            if let Some(critter_material) = &critter_material {
                visual.insert(critter_material.clone());
            }
        } else {
            parent.spawn((
                Name::new("Seagull fallback"),
                Mesh3d(render.cube.clone()),
                MeshMaterial3d(render.food.clone()),
                model_transform.with_scale(Vec3::new(3.0, 0.35, 1.3)),
            ));
        }
    });
}

fn standalone_material_override(
    render: &RenderAssets,
    material_id: &str,
) -> Option<MaterialOverrideSpec> {
    let material = StableId::new(material_id)
        .ok()
        .and_then(|id| render.presentation_materials.get(&id))?
        .clone();
    Some(MaterialOverrideSpec {
        fallback: Some(material),
        model_materials: BTreeMap::new(),
        renderer_materials: Vec::new(),
        suppress_self_shadows: false,
    })
}

#[allow(clippy::too_many_arguments)]
fn spawn_flag_smoke_castle(
    commands: &mut Commands,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: &AssetServer,
    asset_root: &Path,
    position: Vec3,
    cell_size: f32,
) {
    let archetype_id = StableId::new("archetype:building:castle").expect("static archetype ID");
    let Some(archetype) = content.archetypes.get(&archetype_id) else {
        return;
    };
    let Some(scene) = archetype.scenes.iter().find(|scene| scene.age == Some(2)) else {
        return;
    };
    if !converted_asset_exists(asset_root, &scene.asset_path) {
        return;
    }
    let mut castle = commands.spawn((
        Name::new("Flag material smoke castle"),
        WorldEntity,
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
        ),
        Transform::from_translation(position).with_scale(Vec3::splat(cell_size / 2.0)),
    ));
    if let Some(materials) = prefab_material_spec(archetype, scene, presentation, render) {
        castle.insert(materials);
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_godray_smoke_tower(
    commands: &mut Commands,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: &AssetServer,
    asset_root: &Path,
    position: Vec3,
    cell_size: f32,
) {
    let archetype_id = StableId::new("archetype:building:necrotower").expect("static archetype ID");
    let Some(archetype) = content.archetypes.get(&archetype_id) else {
        return;
    };
    let Some(scene) = archetype.scenes.iter().find(|scene| scene.age == Some(2)) else {
        return;
    };
    if !converted_asset_exists(asset_root, &scene.asset_path) {
        return;
    }
    let mut tower = commands.spawn((
        Name::new("Godray material smoke necromancer tower"),
        WorldEntity,
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
        ),
        Transform::from_translation(position).with_scale(Vec3::splat(cell_size / 2.0)),
    ));
    if let Some(materials) = prefab_material_spec(archetype, scene, presentation, render) {
        tower.insert(materials);
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_giraffe_smoke_pet(
    commands: &mut Commands,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: &AssetServer,
    asset_root: &Path,
    position: Vec3,
) {
    let Some(archetype) = content
        .archetypes
        .values()
        .find(|archetype| archetype.source_path.ends_with("Prefabs/Pets/Pet.prefab"))
    else {
        return;
    };
    let pet = StableId::new("pet:giraffe").expect("static pet ID");
    let Some((scene, model)) = pet_model(archetype, &pet) else {
        return;
    };
    if !converted_asset_exists(asset_root, &scene.asset_path) {
        return;
    }
    let mut giraffe = commands.spawn((
        Name::new("Giraffe material smoke pet"),
        WorldEntity,
        Transform::from_translation(position),
    ));
    if let Some(materials) = prefab_material_spec(archetype, scene, presentation, render) {
        giraffe.insert(materials);
    }
    giraffe.with_children(|root| {
        root.spawn((
            WorldAssetRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            Transform::from_translation(Vec3::from_array(model.local_position))
                .with_rotation(Quat::from_array(model.local_rotation).normalize())
                .with_scale(Vec3::from_array(model.local_scale)),
        ));
    });
}

fn drive_seagull_flight(
    mut commands: Commands,
    time: Res<Time>,
    settings: Res<RuntimePlayerSettings>,
    world_audio: Res<WorldAudioRuntime>,
    camera: Query<&GlobalTransform, With<TownCamera>>,
    mut flights: Query<(&mut SeagullFlight, &mut Transform)>,
    #[cfg(target_os = "windows")] native_audio: Res<direct_broadcast::NativeGameAudioRouting>,
) {
    let camera_position = camera.single().ok().map(GlobalTransform::translation);
    for (mut flight, mut transform) in &mut flights {
        let delta = time.delta_secs();
        flight.elapsed_seconds += delta;
        if flight.elapsed_seconds >= SEAGULL_FLIGHT_SECONDS {
            flight.leg_serial = flight.leg_serial.saturating_add(1);
            (flight.start, flight.end) =
                deterministic_seagull_leg(flight.world_seed, flight.leg_serial);
            flight.elapsed_seconds = 0.0;
        }
        let progress = (flight.elapsed_seconds / SEAGULL_FLIGHT_SECONDS).clamp(0.0, 1.0);
        let position = flight.start.lerp(flight.end, progress);
        *transform = seagull_flight_transform(position, flight.end);

        flight.call_elapsed_seconds += delta;
        if flight.call_elapsed_seconds <= flight.call_wait_seconds {
            continue;
        }
        let call_serial = flight.call_serial;
        flight.call_serial = flight.call_serial.saturating_add(1);
        flight.call_elapsed_seconds = 0.0;
        flight.call_wait_seconds = deterministic_seagull_call_wait(flight.world_seed, call_serial);
        if world_audio.seagull_calls.is_empty() {
            continue;
        }
        let variant = deterministic_seagull_call_variant(flight.world_seed, call_serial);
        let source = world_audio.seagull_calls[variant].clone();
        let distance = camera_position.map_or(0.0, |camera| camera.distance(position));
        let gain = SEAGULL_GAIN
            * settings.0.audio.master
            * settings.0.audio.ambience
            * unity_seagull_rolloff(distance);
        #[cfg(target_os = "windows")]
        if let Some(wav) = world_audio.seagull_call_wavs.get(variant) {
            native_audio.play_pcm16_wav(&format!("seagull:{variant}:{call_serial}"), wav, gain);
        }
        #[cfg(target_os = "windows")]
        if !native_audio.local_monitor_enabled() {
            continue;
        }
        commands.spawn((
            Name::new(format!("Seagull call {}", variant + 1)),
            WorldEntity,
            AudioPlayer(source),
            PlaybackSettings::DESPAWN
                .with_volume(Volume::Linear(gain))
                .with_spatial(true)
                .with_spatial_scale(SpatialScale::new(1.0 / SEAGULL_MAX_AUDIO_DISTANCE)),
            Transform::from_translation(position),
        ));
    }
}

fn seagull_flight_transform(position: Vec3, target: Vec3) -> Transform {
    let direction = (target - position).normalize_or_zero();
    let rotation = if direction == Vec3::ZERO {
        Quat::IDENTITY
    } else {
        Quat::from_rotation_arc(Vec3::Z, direction)
    };
    Transform::from_translation(position).with_rotation(rotation)
}

fn seagull_model_transform() -> Transform {
    // The converted flock's visible nose axis is local +X. Unity's prefab uses
    // +90 degrees around Y to align it with Transform.forward, but Unity and
    // glTF/Bevy use opposite handedness for this import turn. Copying the
    // numeric sign maps +X onto -Z and makes the flock fly backward.
    Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2))
        .with_scale(Vec3::splat(5.0))
}

fn deterministic_seagull_leg(seed: u64, serial: u64) -> (Vec3, Vec3) {
    const CENTRES: [Vec2; 4] = [
        Vec2::new(200.0, 0.0),
        Vec2::new(-200.0, 0.0),
        Vec2::new(0.0, -200.0),
        Vec2::new(0.0, 200.0),
    ];
    const AREAS: [Vec2; 4] = [
        Vec2::new(5.0, 400.0),
        Vec2::new(5.0, 400.0),
        Vec2::new(400.0, 5.0),
        Vec2::new(400.0, 5.0),
    ];
    let mut area =
        usize::try_from(seagull_hash(seed, serial, 0) % 4).expect("seagull area index fits usize");
    // The Unity component never updates `_currentArea`, so an initial area-zero
    // roll is always advanced to area one. Preserve that shipping behaviour.
    if area == 0 {
        area = 1;
    }
    let offset = Vec2::new(
        seagull_signed_unit(seed, serial, 1) * AREAS[area].x * 0.5,
        seagull_signed_unit(seed, serial, 2) * AREAS[area].y * 0.5,
    );
    let horizontal = CENTRES[area] + offset;
    let start = Vec3::new(horizontal.x, SEAGULL_HEIGHT, horizontal.y);
    let end = Vec3::new(-horizontal.x, SEAGULL_HEIGHT, -horizontal.y);
    (start, end)
}

fn deterministic_seagull_call_wait(seed: u64, serial: u64) -> f32 {
    1.0 + seagull_unit(seed, serial, 3) * 4.0
}

fn deterministic_seagull_call_variant(seed: u64, serial: u64) -> usize {
    usize::try_from(seagull_hash(seed, serial, 4) % 3).expect("seagull call variant fits usize")
}

fn seagull_signed_unit(seed: u64, serial: u64, salt: u64) -> f32 {
    seagull_unit(seed, serial, salt) * 2.0 - 1.0
}

#[allow(clippy::cast_precision_loss)]
fn seagull_unit(seed: u64, serial: u64, salt: u64) -> f32 {
    let value = seagull_hash(seed, serial, salt) >> 40;
    value as f32 / 16_777_215.0
}

fn seagull_hash(seed: u64, serial: u64, salt: u64) -> u64 {
    let mut mixed = seed
        .wrapping_add(serial.wrapping_mul(0x9e37_79b9_7f4a_7c15))
        .wrapping_add(salt.wrapping_mul(0xd1b5_4a32_d192_ed03));
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    mixed ^ (mixed >> 31)
}

fn unity_seagull_rolloff(distance: f32) -> f32 {
    const CURVE: [(f32, f32); 7] = [
        (0.0, 1.0),
        (0.05, 1.0),
        (0.1, 0.5),
        (0.2, 0.25),
        (0.4, 0.125),
        (0.766_247_6, 0.037_643_433),
        (1.0, 0.0),
    ];
    let normalized = (distance / SEAGULL_MAX_AUDIO_DISTANCE).clamp(0.0, 1.0);
    for segment in CURVE.windows(2) {
        let (start_time, start_value) = segment[0];
        let (end_time, end_value) = segment[1];
        if normalized <= end_time {
            let progress = ((normalized - start_time) / (end_time - start_time)).clamp(0.0, 1.0);
            return start_value + (end_value - start_value) * progress;
        }
    }
    0.0
}

#[allow(clippy::cast_precision_loss)]
fn procedural_ambience_wav(sample_rate: u32, seconds: f32) -> Vec<u8> {
    let duration = seconds.max(1.0 / sample_rate.max(1) as f32);
    synthesize_wav(sample_rate, seconds, move |time, _| {
        // Integer harmonics over the clip duration make the loop seamless. The
        // previous sample-by-sample white-noise layer was technically valid PCM,
        // but sounded like static on headphones and some resamplers.
        let phase = time / duration * std::f32::consts::TAU;
        let gust = (phase * 2.0).sin() * 0.55
            + (phase * 5.0 + 0.8).sin() * 0.28
            + (phase * 11.0 + 2.1).sin() * 0.14;
        let air = (phase * 23.0 + 1.4).sin() * 0.08
            + (phase * 41.0 + 0.3).sin() * 0.05
            + (phase * 67.0 + 2.7).sin() * 0.025;
        (gust * 0.085 + air * 0.04).clamp(-0.12, 0.12)
    })
}

fn procedural_seagull_call_wav(variant: u8, sample_rate: u32) -> Vec<u8> {
    let (duration, start_frequency, frequency_sweep) = match variant % 3 {
        0 => (0.62, 1_080.0, 420.0),
        1 => (0.78, 920.0, 610.0),
        _ => (0.55, 1_240.0, -260.0),
    };
    synthesize_wav(sample_rate, duration, move |time, _| {
        let phase = (time / duration).clamp(0.0, 1.0);
        let envelope = (phase * std::f32::consts::PI).sin().powf(0.65);
        let pulse = (phase * std::f32::consts::TAU * (2.0 + f32::from(variant))).sin() * 0.08;
        let frequency = start_frequency + frequency_sweep * phase + pulse * start_frequency;
        let carrier = (time * std::f32::consts::TAU * frequency).sin();
        let harmonic = (time * std::f32::consts::TAU * frequency * 1.97).sin() * 0.28;
        (carrier + harmonic) * envelope * 0.22
    })
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn synthesize_wav(
    sample_rate: u32,
    seconds: f32,
    mut sample_value: impl FnMut(f32, u32) -> f32,
) -> Vec<u8> {
    let sample_count = (Duration::from_secs_f32(seconds).as_secs_f64() * f64::from(sample_rate))
        .round()
        .clamp(1.0, f64::from(u32::MAX)) as u32;
    let data_bytes = sample_count.saturating_mul(2);
    let mut wav = Vec::with_capacity(usize::try_from(data_bytes).unwrap_or(0) + 44);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&data_bytes.saturating_add(36).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&sample_rate.saturating_mul(2).to_le_bytes());
    wav.extend_from_slice(&2_u16.to_le_bytes());
    wav.extend_from_slice(&16_u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_bytes.to_le_bytes());
    for sample in 0..sample_count {
        let time = sample as f32 / sample_rate.max(1) as f32;
        let value = sample_value(time, sample).clamp(-1.0, 1.0);
        let encoded = (value * f32::from(i16::MAX)).round() as i16;
        wav.extend_from_slice(&encoded.to_le_bytes());
    }
    wav
}

#[allow(clippy::cast_precision_loss)]
fn pseudo_noise(value: u32) -> f32 {
    let mixed = value.wrapping_mul(747_796_405).wrapping_add(2_891_336_453);
    let mixed = ((mixed >> ((mixed >> 28) + 4)) ^ mixed).wrapping_mul(277_803_737);
    let normalized = ((mixed >> 22) ^ mixed) as f32 / u32::MAX as f32;
    normalized * 2.0 - 1.0
}

use presentation::materials::*;

use presentation::menu::*;

use presentation::world::*;

use runtime::agents::*;

use presentation::environment::*;

use presentation::animation::*;

use presentation::rendering::*;

use runtime::control::*;

use runtime::commands::*;

use runtime::persistence::*;

fn cleanup_state_entities(mut commands: Commands, entities: Query<Entity, With<StateEntity>>) {
    for entity in &entities {
        commands.entity(entity).try_despawn();
    }
}

fn cleanup_credits(mut commands: Commands) {
    commands.remove_resource::<CreditsTimeline>();
}

fn cleanup_loading_screen(
    mut commands: Commands,
    entities: Query<Entity, With<LoadingScreenEntity>>,
    cameras: Query<Entity, With<LoadingUiCamera>>,
) {
    for entity in &entities {
        commands.entity(entity).try_despawn();
    }
    for entity in &cameras {
        commands.entity(entity).try_despawn();
    }
    clear_loading_runtime(&mut commands);
}

fn cleanup_loading_runtime(mut commands: Commands) {
    clear_loading_runtime(&mut commands);
}

fn clear_loading_runtime(commands: &mut Commands) {
    commands.remove_resource::<GameplayReady>();
    commands.remove_resource::<WorldLoadingRuntime>();
    commands.remove_resource::<WorldGenerationTask>();
    commands.remove_resource::<WorldLoadingCoverRuntime>();
    commands.remove_resource::<MenuLoadingRuntime>();
    commands.remove_resource::<MenuRevealRuntime>();
    commands.remove_resource::<MainMenuSpawnRuntime>();
    commands.remove_resource::<WorldRevealRuntime>();
}

fn cleanup_world(
    mut commands: Commands,
    entities: Query<Entity, With<WorldEntity>>,
    mut building_instances: ResMut<BuildingMaterialInstances>,
    mut building_update_runtime: ResMut<BuildingMaterialUpdateRuntime>,
    mut building_materials: Option<ResMut<Assets<BuildingMaterial>>>,
    mut diagnostic_view: ResMut<WorldDiagnosticRuntime>,
    mut station_targets: ResMut<StationTargetRuntime>,
    mut ruler_announcements: ResMut<RulerVoteAnnouncementRuntime>,
    mut death_announcements: ResMut<CitizenDeathAnnouncementRuntime>,
    mut night_waves: ResMut<NightEnemyWaveRuntime>,
) {
    for entity in &entities {
        commands.entity(entity).try_despawn();
    }
    if let Some(materials) = building_materials.as_deref_mut() {
        for instance in building_instances.0.values() {
            materials.remove(&instance.handle);
        }
    }
    building_instances.0.clear();
    *building_update_runtime = BuildingMaterialUpdateRuntime::default();
    commands.remove_resource::<WorldRuntime>();
    commands.insert_resource(FineNavigationRuntime::default());
    *diagnostic_view = WorldDiagnosticRuntime::default();
    commands.remove_resource::<SimulationRuntime>();
    commands.remove_resource::<WorldRevealRuntime>();
    commands.remove_resource::<GameplayReady>();
    commands.insert_resource(BuildingPlacers::default());
    commands.insert_resource(BuildingCommandQueue::default());
    *station_targets = StationTargetRuntime::default();
    *ruler_announcements = RulerVoteAnnouncementRuntime::default();
    *death_announcements = CitizenDeathAnnouncementRuntime::default();
    *night_waves = NightEnemyWaveRuntime::default();
}

fn cleanup_menu_overlay(mut commands: Commands, overlays: MenuOverlayEntityQuery) {
    for entity in &overlays {
        commands.entity(entity).try_despawn();
    }
}

fn should_show_actor_name(mode: NameDisplayMode, user_type: StreamUserType) -> bool {
    match mode {
        NameDisplayMode::None => false,
        NameDisplayMode::StaffAndSubscribers => user_type.is_staff_or_subscriber(),
        NameDisplayMode::AllPlayers => true,
    }
}

fn is_stream_player_actor(actor_id: &StableId) -> bool {
    actor_id.as_str().starts_with("twitch:")
}

fn stream_user_color(user_type: StreamUserType) -> Color {
    match user_type {
        StreamUserType::GameMaster => Color::srgb(1.0, 0.22, 0.0),
        StreamUserType::Broadcaster => Color::srgb(1.0, 0.12, 0.12),
        StreamUserType::Moderator => Color::srgb(0.2, 1.0, 0.3),
        StreamUserType::Subscriber => Color::srgb(0.48, 0.3, 0.78),
        StreamUserType::Normal => Color::srgb(0.88, 0.87, 0.80),
    }
}

fn actor_name_color(actor: &ActorState) -> Color {
    actor
        .customization
        .name_color
        .map_or_else(|| stream_user_color(actor.user_type), color_from_rgb8)
}

fn should_show_building_health(
    mode: BuildingHealthDisplayMode,
    health: i32,
    max_health: i32,
) -> bool {
    match mode {
        BuildingHealthDisplayMode::None => false,
        BuildingHealthDisplayMode::DamagedOnly => health < max_health,
        BuildingHealthDisplayMode::Always => true,
    }
}

fn overlay_viewport_position(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    world_position: Vec3,
) -> Option<Vec2> {
    let viewport = camera.logical_viewport_size()?;
    camera
        .world_to_viewport(camera_transform, world_position)
        .ok()
        .filter(|position| overlay_position_clears_hud(viewport, *position))
}

fn overlay_position_clears_hud(viewport: Vec2, position: Vec2) -> bool {
    position.x >= 0.0
        && position.y >= WORLD_UI_SAFE_TOP + WORLD_UI_OVERLAY_MARGIN
        && position.x <= viewport.x
        && position.y <= viewport.y - WORLD_UI_SAFE_BOTTOM - WORLD_UI_OVERLAY_MARGIN
}

#[allow(clippy::type_complexity)]
fn sync_actor_name_overlays(
    mut commands: Commands,
    settings: Res<RuntimePlayerSettings>,
    config: Res<RuntimeConfig>,
    simulation: Res<SimulationRuntime>,
    cameras: Query<(&Camera, &GlobalTransform), With<TownCamera>>,
    agents: Query<(&Agent, &GlobalTransform)>,
    mut overlays: Query<
        (
            Entity,
            &ActorNameOverlay,
            &mut Text,
            &mut TextColor,
            &mut Node,
            &mut Visibility,
        ),
        Without<BuildingHealthOverlay>,
    >,
) {
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };
    let actor_positions: BTreeMap<_, _> = agents
        .iter()
        .filter(|(agent, _)| agent.kind == ActorKind::Player && is_stream_player_actor(&agent.id))
        .map(|(agent, transform)| (agent.id.clone(), transform.translation()))
        .collect();
    let mut existing = BTreeSet::new();
    for (entity, overlay, mut text, mut color, mut node, mut visibility) in &mut overlays {
        existing.insert(overlay.actor.clone());
        let Some(actor) = simulation.0.actors.get(&overlay.actor) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let Some(position) = actor_positions.get(&overlay.actor) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let show = actor.alive
            && should_show_actor_name(settings.0.interface.display_names, actor.user_type);
        let screen = show.then(|| {
            overlay_viewport_position(
                camera,
                camera_transform,
                *position + Vec3::Y * config.0.world.cell_size * ACTOR_OVERLAY_ANCHOR_HEIGHT_CELLS,
            )
        });
        let Some(screen) = screen.flatten() else {
            *visibility = Visibility::Hidden;
            continue;
        };
        actor
            .display_name
            .as_deref()
            .or(actor.login_name.as_deref())
            .unwrap_or_else(|| overlay.actor.as_str())
            .clone_into(&mut *text);
        color.0 = actor_name_color(actor);
        node.left = px(screen.x - 80.0);
        node.top = px(screen.y + ACTOR_NAME_OVERLAY_TOP_PX);
        *visibility = Visibility::Visible;
    }
    for (actor_id, position) in actor_positions {
        let Some(actor) = simulation.0.actors.get(&actor_id) else {
            continue;
        };
        if existing.contains(&actor_id)
            || !actor.alive
            || !should_show_actor_name(settings.0.interface.display_names, actor.user_type)
        {
            continue;
        }
        let Some(screen) = overlay_viewport_position(
            camera,
            camera_transform,
            position + Vec3::Y * config.0.world.cell_size * ACTOR_OVERLAY_ANCHOR_HEIGHT_CELLS,
        ) else {
            continue;
        };
        commands.spawn((
            WorldEntity,
            ActorNameOverlay {
                actor: actor_id.clone(),
            },
            Text::new(
                actor
                    .display_name
                    .as_deref()
                    .or(actor.login_name.as_deref())
                    .unwrap_or_else(|| actor_id.as_str()),
            ),
            TextFont {
                font_size: FontSize::Px(15.0),
                ..default()
            },
            TextLayout::justify(Justify::Center),
            TextColor(actor_name_color(actor)),
            TextShadow {
                offset: Vec2::splat(1.5),
                color: Color::linear_rgba(0.0, 0.0, 0.0, 0.98),
            },
            Pickable::IGNORE,
            GlobalZIndex(18),
            Node {
                position_type: PositionType::Absolute,
                left: px(screen.x - 80.0),
                top: px(screen.y + ACTOR_NAME_OVERLAY_TOP_PX),
                width: px(160),
                ..default()
            },
        ));
    }
}

fn actor_health_bar_hide_seconds(content: &ContentCatalog, actor: &ActorState) -> Option<f32> {
    actor_archetype(content, actor)
        .and_then(|archetype| archetype.health_bar_hide_milliseconds)
        .or_else(|| (actor.role.as_str() == "role:enemy").then_some(3_000))
        .map(|milliseconds| Duration::from_millis(u64::from(milliseconds)).as_secs_f32())
}

fn update_actor_health_overlay_timer(
    overlay: &mut ActorHealthOverlay,
    health: i32,
    max_health: i32,
    hide_delay_seconds: f32,
    delta_seconds: f32,
) -> bool {
    if health < overlay.last_health {
        overlay.was_damaged = true;
        overlay.hide_remaining_seconds = hide_delay_seconds;
    } else if health != overlay.last_health && overlay.was_damaged {
        overlay.hide_remaining_seconds = hide_delay_seconds;
    }
    overlay.last_health = health;
    if !overlay.was_damaged {
        return false;
    }
    if health < max_health {
        return true;
    }
    overlay.hide_remaining_seconds =
        (overlay.hide_remaining_seconds - delta_seconds.max(0.0)).max(0.0);
    if overlay.hide_remaining_seconds > 0.0 {
        true
    } else {
        overlay.was_damaged = false;
        false
    }
}

fn actor_health_fill_color(actor: &ActorState) -> Color {
    if actor.role.as_str() == "role:enemy" {
        Color::srgb(0.58, 0.16, 0.86)
    } else {
        Color::srgb(0.85, 0.1, 0.1)
    }
}

#[allow(clippy::type_complexity)]
fn sync_actor_health_overlays(
    mut commands: Commands,
    cadence: Res<AgentSimulationCadence>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    cameras: Query<(&Camera, &GlobalTransform), With<TownCamera>>,
    agents: Query<(&Agent, &GlobalTransform)>,
    mut overlays: Query<(
        Entity,
        &mut ActorHealthOverlay,
        &Children,
        &mut Node,
        &mut Visibility,
    )>,
    mut fills: Query<
        (&mut Node, &mut BackgroundColor),
        (
            With<ActorHealthFill>,
            Without<ActorHealthOverlay>,
            Without<ActorNameOverlay>,
            Without<BuildingHealthOverlay>,
        ),
    >,
) {
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };
    let actor_positions: BTreeMap<_, _> = agents
        .iter()
        .filter(|(agent, _)| matches!(agent.kind, ActorKind::Player | ActorKind::Enemy))
        .map(|(agent, transform)| (agent.id.clone(), transform.translation()))
        .collect();
    let mut existing = BTreeSet::new();
    for (entity, mut overlay, children, mut node, mut visibility) in &mut overlays {
        existing.insert(overlay.actor.clone());
        let Some(actor) = simulation.0.actors.get(&overlay.actor) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let Some(position) = actor_positions.get(&overlay.actor) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let Some(hide_delay) = actor_health_bar_hide_seconds(&content.0, actor) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let show = actor.alive
            && update_actor_health_overlay_timer(
                &mut overlay,
                actor.health,
                actor.max_health.max(1),
                hide_delay,
                cadence.delta().as_secs_f32(),
            );
        let screen = show.then(|| {
            overlay_viewport_position(
                camera,
                camera_transform,
                *position + Vec3::Y * config.0.world.cell_size * ACTOR_OVERLAY_ANCHOR_HEIGHT_CELLS,
            )
        });
        let Some(screen) = screen.flatten() else {
            *visibility = Visibility::Hidden;
            continue;
        };
        node.left = px(screen.x - 45.0);
        node.top = px(screen.y + ACTOR_HEALTH_OVERLAY_TOP_PX);
        *visibility = Visibility::Visible;
        let health_fraction = building_health_fraction(actor.health, actor.max_health.max(1));
        for child in children.iter() {
            if let Ok((mut fill, mut color)) = fills.get_mut(child) {
                fill.width = percent(health_fraction * 100.0);
                color.0 = actor_health_fill_color(actor);
            }
        }
    }
    for (actor_id, position) in actor_positions {
        let Some(actor) = simulation.0.actors.get(&actor_id) else {
            continue;
        };
        let Some(hide_delay) = actor_health_bar_hide_seconds(&content.0, actor) else {
            continue;
        };
        if existing.contains(&actor_id) || !actor.alive || actor.health >= actor.max_health.max(1) {
            continue;
        }
        let Some(screen) = overlay_viewport_position(
            camera,
            camera_transform,
            position + Vec3::Y * config.0.world.cell_size * ACTOR_OVERLAY_ANCHOR_HEIGHT_CELLS,
        ) else {
            continue;
        };
        let health_fraction = building_health_fraction(actor.health, actor.max_health.max(1));
        commands
            .spawn((
                WorldEntity,
                ActorHealthOverlay {
                    actor: actor_id,
                    last_health: actor.health,
                    was_damaged: true,
                    hide_remaining_seconds: hide_delay,
                },
                Node {
                    position_type: PositionType::Absolute,
                    left: px(screen.x - 45.0),
                    top: px(screen.y + ACTOR_HEALTH_OVERLAY_TOP_PX),
                    width: px(90),
                    height: px(10),
                    padding: UiRect::all(px(1)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
                GlobalZIndex(19),
            ))
            .with_children(|parent| {
                parent.spawn((
                    ActorHealthFill,
                    Node {
                        width: percent(health_fraction * 100.0),
                        height: percent(100.0),
                        ..default()
                    },
                    BackgroundColor(actor_health_fill_color(actor)),
                ));
            });
    }
}

fn temporary_world_label_is_live(label: &mut TemporaryWorldLabel, delta_seconds: f32) -> bool {
    label.remaining_seconds = (label.remaining_seconds - delta_seconds.max(0.0)).max(0.0);
    label.remaining_seconds > 0.0
}

#[allow(clippy::type_complexity)]
fn sync_temporary_world_labels(
    mut commands: Commands,
    cadence: Res<AgentSimulationCadence>,
    config: Res<RuntimeConfig>,
    cameras: Query<(&Camera, &GlobalTransform), With<TownCamera>>,
    agents: Query<(&Agent, &GlobalTransform)>,
    buildings: Query<(&RuntimeBuilding, &GlobalTransform)>,
    resources: Query<(&ResourceNode, &GlobalTransform)>,
    mut labels: Query<(Entity, &mut TemporaryWorldLabel, &mut Node, &mut Visibility)>,
) {
    // Numbered labels exist only briefly after a matching chat command. Avoid
    // cloning every resource ID into a new map on ordinary unlabeled frames.
    if labels.is_empty() {
        return;
    }
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };
    let mut target_positions = BTreeMap::new();
    target_positions.extend(
        agents
            .iter()
            .map(|(agent, transform)| (agent.id.clone(), transform.translation())),
    );
    target_positions.extend(
        buildings
            .iter()
            .map(|(building, transform)| (building.id.clone(), transform.translation())),
    );
    target_positions.extend(
        resources
            .iter()
            .map(|(resource, transform)| (resource.id.clone(), transform.translation())),
    );
    for (entity, mut label, mut node, mut visibility) in &mut labels {
        if !temporary_world_label_is_live(&mut label, cadence.delta().as_secs_f32()) {
            commands.entity(entity).try_despawn();
            continue;
        }
        let Some(position) = target_positions.get(&label.target) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let Some(screen) = overlay_viewport_position(
            camera,
            camera_transform,
            *position + Vec3::Y * config.0.world.cell_size * 1.35,
        ) else {
            *visibility = Visibility::Hidden;
            continue;
        };
        node.left = px(screen.x - 20.0);
        node.top = px(screen.y - 12.0);
        *visibility = Visibility::Visible;
    }
}

fn building_health_overlay_world_position(
    building: &RuntimeBuilding,
    fallback: Vec3,
    simulation: &WorldSimulation,
    config: &GameConfig,
    world: &GeneratedWorld,
) -> Vec3 {
    simulation
        .path_navigation_positions
        .get(&building.id)
        .map_or_else(
            || fallback,
            |fine| navigation_to_world_on_surface(*fine, config, world),
        )
}

fn building_health_overlay_width_px(content: &ContentCatalog, building: &BuildingState) -> f32 {
    if building_def_for_archetype(content, &building.archetype)
        .is_some_and(|definition| definition.archetype.as_str() == "archetype:building:path")
    {
        PATH_HEALTH_OVERLAY_WIDTH_PX
    } else {
        BUILDING_HEALTH_OVERLAY_WIDTH_PX
    }
}

#[allow(clippy::type_complexity)]
fn sync_building_health_overlays(
    mut commands: Commands,
    settings: Res<RuntimePlayerSettings>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    cameras: Query<(&Camera, &GlobalTransform), With<TownCamera>>,
    buildings: Query<(&RuntimeBuilding, &GlobalTransform)>,
    mut overlays: Query<
        (
            Entity,
            &BuildingHealthOverlay,
            &Children,
            &mut Node,
            &mut Visibility,
        ),
        Without<ActorNameOverlay>,
    >,
    mut fills: Query<
        (&mut Node, &mut BackgroundColor),
        (
            With<BuildingHealthFill>,
            Without<ActorNameOverlay>,
            Without<BuildingHealthOverlay>,
        ),
    >,
) {
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };
    let building_positions: BTreeMap<_, _> = buildings
        .iter()
        .filter_map(|(building, transform)| {
            let state = simulation.0.buildings.get(&building.id)?;
            let max_health = building_max_health(&content.0, state).max(1);
            if !should_show_building_health(
                settings.0.interface.display_building_health,
                state.health,
                max_health,
            ) {
                return None;
            }
            let position = building_health_overlay_world_position(
                building,
                transform.translation(),
                &simulation.0,
                &config.0,
                &world.generated,
            );
            let width = building_health_overlay_width_px(&content.0, state);
            Some((building.id.clone(), (position, width)))
        })
        .collect();
    let mut existing = BTreeSet::new();
    for (entity, overlay, children, mut node, mut visibility) in &mut overlays {
        existing.insert(overlay.building.clone());
        let Some(building) = simulation.0.buildings.get(&overlay.building) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let max_health = building_max_health(&content.0, building).max(1);
        let health_fraction = building_health_fraction(building.health, max_health);
        let show = should_show_building_health(
            settings.0.interface.display_building_health,
            building.health,
            max_health,
        );
        if !show {
            *visibility = Visibility::Hidden;
            continue;
        }
        let Some((position, width)) = building_positions.get(&overlay.building) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let screen = show.then(|| {
            overlay_viewport_position(
                camera,
                camera_transform,
                *position + Vec3::Y * config.0.world.cell_size * 1.45,
            )
        });
        let Some(screen) = screen.flatten() else {
            *visibility = Visibility::Hidden;
            continue;
        };
        node.left = px(screen.x - *width * 0.5);
        node.top = px(screen.y - 5.0);
        node.width = px(*width);
        *visibility = Visibility::Visible;
        for child in children.iter() {
            if let Ok((mut fill, mut color)) = fills.get_mut(child) {
                fill.width = percent(health_fraction * 100.0);
                color.0 = building_health_color(health_fraction);
            }
        }
    }
    for (building_id, (position, width)) in building_positions {
        let Some(building) = simulation.0.buildings.get(&building_id) else {
            continue;
        };
        let max_health = building_max_health(&content.0, building).max(1);
        if existing.contains(&building_id)
            || !should_show_building_health(
                settings.0.interface.display_building_health,
                building.health,
                max_health,
            )
        {
            continue;
        }
        let Some(screen) = overlay_viewport_position(
            camera,
            camera_transform,
            position + Vec3::Y * config.0.world.cell_size * 1.45,
        ) else {
            continue;
        };
        let health_fraction = building_health_fraction(building.health, max_health);
        commands
            .spawn((
                WorldEntity,
                BuildingHealthOverlay {
                    building: building_id,
                },
                Node {
                    position_type: PositionType::Absolute,
                    left: px(screen.x - width * 0.5),
                    top: px(screen.y - 5.0),
                    width: px(width),
                    height: px(10),
                    border: UiRect::all(px(1)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.02, 0.025, 0.03, 0.9)),
                BorderColor::all(Color::srgba(0.8, 0.86, 0.82, 0.8)),
                GlobalZIndex(19),
            ))
            .with_children(|parent| {
                parent.spawn((
                    BuildingHealthFill,
                    Node {
                        width: percent(health_fraction * 100.0),
                        height: percent(100.0),
                        ..default()
                    },
                    BackgroundColor(building_health_color(health_fraction)),
                ));
            });
    }
}

fn building_health_fraction(health: i32, max_health: i32) -> f32 {
    let thousandths = health
        .max(0)
        .saturating_mul(1_000)
        .checked_div(max_health.max(1))
        .unwrap_or_default()
        .clamp(0, 1_000);
    f32::from(u16::try_from(thousandths).expect("clamped health fraction fits u16")) / 1_000.0
}

fn building_health_color(health_fraction: f32) -> Color {
    Color::srgb(
        1.0 - health_fraction * 0.72,
        0.18 + health_fraction * 0.68,
        0.12,
    )
}

fn grid_to_world(position: GridPos, config: &GameConfig) -> Vec3 {
    Vec3::new(
        (f32::from(position.x) - f32::from(config.world.width.saturating_sub(1)) * 0.5)
            * config.world.cell_size,
        0.0,
        (f32::from(position.z) - f32::from(config.world.height.saturating_sub(1)) * 0.5)
            * config.world.cell_size,
    )
}

fn grid_to_world_on_surface(
    position: GridPos,
    config: &GameConfig,
    world: &GeneratedWorld,
) -> Vec3 {
    let mut position_world = grid_to_world(position, config);
    position_world.y = terrain_height(world, position);
    position_world
}

fn terrain_height(world: &GeneratedWorld, position: GridPos) -> f32 {
    f32::from(world.navigation.height_at(position).unwrap_or_default()) * 0.01
}

fn shoreline_focus(world: &GeneratedWorld, config: &GameConfig) -> Vec3 {
    let centre = GridPos {
        x: world.navigation.width() / 2,
        z: world.navigation.height() / 2,
    };
    let water_height = f32::from(config.world.water_level_centimetres) * 0.01;
    let best = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .filter(|position| terrain_height(world, *position) <= water_height)
        .filter(|position| {
            [
                position
                    .x
                    .checked_sub(1)
                    .map(|x| GridPos { x, z: position.z }),
                (position.x + 1 < world.navigation.width()).then_some(GridPos {
                    x: position.x + 1,
                    z: position.z,
                }),
                position
                    .z
                    .checked_sub(1)
                    .map(|z| GridPos { x: position.x, z }),
                (position.z + 1 < world.navigation.height()).then_some(GridPos {
                    x: position.x,
                    z: position.z + 1,
                }),
            ]
            .into_iter()
            .flatten()
            .any(|neighbor| terrain_height(world, neighbor) > water_height)
        })
        .min_by_key(|position| position.x.abs_diff(centre.x) + position.z.abs_diff(centre.z))
        .unwrap_or(centre);
    let mut boundary = grid_to_world_on_surface(best, config, world);
    boundary.y = water_height;
    let centre_world = grid_to_world_on_surface(centre, config, world);
    let inward = Vec3::new(
        centre_world.x - boundary.x,
        0.0,
        centre_world.z - boundary.z,
    )
    .normalize_or_zero();
    boundary + inward * config.world.cell_size * 2.5
}

fn shoreline_camera_transform(world: &GeneratedWorld, config: &GameConfig) -> Transform {
    let focus = shoreline_focus(world, config);
    let centre = GridPos {
        x: world.navigation.width() / 2,
        z: world.navigation.height() / 2,
    };
    let centre_world = grid_to_world_on_surface(centre, config, world);
    let inward =
        Vec3::new(centre_world.x - focus.x, 0.0, centre_world.z - focus.z).normalize_or_zero();
    let outward = -inward;
    let tangent = Vec3::new(-inward.z, 0.0, inward.x);
    Transform::from_translation(focus + outward * 28.0 + tangent * 8.0 + Vec3::Y * 24.0)
        .looking_at(focus + Vec3::Y, Vec3::Y)
}

struct GeneratedTerrainChunk {
    chunk_x: u16,
    chunk_z: u16,
    centre: Vec2,
    high: Mesh,
    medium: Mesh,
    low: Mesh,
}

fn generated_terrain_chunks(
    world: &GeneratedWorld,
    config: &GameConfig,
) -> Vec<GeneratedTerrainChunk> {
    let width = world.navigation.width();
    let height = world.navigation.height();
    (0..height.div_ceil(TERRAIN_CHUNK_CELLS))
        .flat_map(|chunk_z| {
            (0..width.div_ceil(TERRAIN_CHUNK_CELLS)).map(move |chunk_x| {
                let start_x = chunk_x * TERRAIN_CHUNK_CELLS;
                let start_z = chunk_z * TERRAIN_CHUNK_CELLS;
                let cells_x = (width - start_x).min(TERRAIN_CHUNK_CELLS);
                let cells_z = (height - start_z).min(TERRAIN_CHUNK_CELLS);
                let centre = Vec2::new(
                    (f32::from(start_x) + f32::from(cells_x) * 0.5
                        - f32::from(width.saturating_sub(1)) * 0.5)
                        * config.world.cell_size,
                    (f32::from(start_z) + f32::from(cells_z) * 0.5
                        - f32::from(height.saturating_sub(1)) * 0.5)
                        * config.world.cell_size,
                );
                GeneratedTerrainChunk {
                    chunk_x,
                    chunk_z,
                    centre,
                    high: generated_terrain_chunk_mesh(
                        world, config, start_x, start_z, cells_x, cells_z, 1, false,
                    ),
                    medium: generated_terrain_chunk_mesh(
                        world, config, start_x, start_z, cells_x, cells_z, 2, true,
                    ),
                    low: generated_terrain_chunk_mesh(
                        world, config, start_x, start_z, cells_x, cells_z, 4, true,
                    ),
                }
            })
        })
        .collect()
}

fn generated_terrain_mesh(world: &GeneratedWorld, config: &GameConfig) -> Mesh {
    let width = world.navigation.width();
    let height = world.navigation.height();
    let half = config.world.cell_size * 0.5 * 0.7;
    let estimated_faces = usize::from(width) * usize::from(height) * 4;
    let mut positions = Vec::with_capacity(estimated_faces * 4);
    let mut colors = Vec::with_capacity(estimated_faces * 4);
    let mut uvs = Vec::with_capacity(estimated_faces * 4);
    let mut indices = Vec::with_capacity(estimated_faces * 6);
    let centre = |x: u16, z: u16| {
        Vec2::new(
            (f32::from(x) - f32::from(width.saturating_sub(1)) * 0.5) * config.world.cell_size,
            (f32::from(z) - f32::from(height.saturating_sub(1)) * 0.5) * config.world.cell_size,
        )
    };
    let elevation = |x: u16, z: u16| {
        f32::from(
            world
                .navigation
                .height_at(GridPos { x, z })
                .unwrap_or_default(),
        ) * 0.01
    };
    for z in 0..height {
        for x in 0..width {
            let c = centre(x, z);
            let y = elevation(x, z);
            append_terrain_quad(
                [
                    [c.x - half, y, c.y - half],
                    [c.x + half, y, c.y - half],
                    [c.x + half, y, c.y + half],
                    [c.x - half, y, c.y + half],
                ],
                terrain_vertex_color(y, config),
                [
                    f32::from(x) / f32::from(width),
                    f32::from(z) / f32::from(height),
                ],
                &mut positions,
                &mut colors,
                &mut uvs,
                &mut indices,
            );
            if x > 0 {
                let previous = centre(x - 1, z);
                let previous_y = elevation(x - 1, z);
                append_terrain_quad(
                    [
                        [previous.x + half, previous_y, c.y - half],
                        [c.x - half, y, c.y - half],
                        [c.x - half, y, c.y + half],
                        [previous.x + half, previous_y, c.y + half],
                    ],
                    terrain_vertex_color((y + previous_y) * 0.5, config),
                    [
                        f32::from(x) / f32::from(width),
                        f32::from(z) / f32::from(height),
                    ],
                    &mut positions,
                    &mut colors,
                    &mut uvs,
                    &mut indices,
                );
            }
            if z > 0 {
                let previous = centre(x, z - 1);
                let previous_y = elevation(x, z - 1);
                append_terrain_quad(
                    [
                        [c.x - half, previous_y, previous.y + half],
                        [c.x + half, previous_y, previous.y + half],
                        [c.x + half, y, c.y - half],
                        [c.x - half, y, c.y - half],
                    ],
                    terrain_vertex_color((y + previous_y) * 0.5, config),
                    [
                        f32::from(x) / f32::from(width),
                        f32::from(z) / f32::from(height),
                    ],
                    &mut positions,
                    &mut colors,
                    &mut uvs,
                    &mut indices,
                );
            }
            if x > 0 && z > 0 {
                let upper_left = centre(x - 1, z - 1);
                let upper_right = centre(x, z - 1);
                let lower_left = centre(x - 1, z);
                let heights = [
                    elevation(x - 1, z - 1),
                    elevation(x, z - 1),
                    y,
                    elevation(x - 1, z),
                ];
                append_terrain_quad(
                    [
                        [upper_left.x + half, heights[0], upper_left.y + half],
                        [upper_right.x - half, heights[1], upper_right.y + half],
                        [c.x - half, heights[2], c.y - half],
                        [lower_left.x + half, heights[3], lower_left.y - half],
                    ],
                    terrain_vertex_color(heights.into_iter().sum::<f32>() * 0.25, config),
                    [
                        f32::from(x) / f32::from(width),
                        f32::from(z) / f32::from(height),
                    ],
                    &mut positions,
                    &mut colors,
                    &mut uvs,
                    &mut indices,
                );
            }
        }
    }
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices));
    mesh.compute_smooth_normals();
    mesh
}

#[allow(clippy::too_many_arguments)]
fn append_terrain_quad(
    quad: [[f32; 3]; 4],
    color: [f32; 4],
    uv: [f32; 2],
    positions: &mut Vec<[f32; 3]>,
    colors: &mut Vec<[f32; 4]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
) {
    let base = u32::try_from(positions.len()).expect("terrain vertex count fits u32");
    positions.extend_from_slice(&quad);
    colors.extend_from_slice(&[color; 4]);
    uvs.extend_from_slice(&[uv; 4]);
    indices.extend_from_slice(&[base, base + 3, base + 1, base + 1, base + 3, base + 2]);
}

fn generated_terrain_chunk_mesh(
    world: &GeneratedWorld,
    config: &GameConfig,
    start_x: u16,
    start_z: u16,
    cells_x: u16,
    cells_z: u16,
    detail_step: u16,
    add_skirts: bool,
) -> Mesh {
    let world_width = world.navigation.width();
    let world_height = world.navigation.height();
    let sample_x = terrain_lod_samples(cells_x, detail_step);
    let sample_z = terrain_lod_samples(cells_z, detail_step);
    let columns = u32::try_from(sample_x.len()).expect("terrain LOD column count fits u32");
    let mut positions = Vec::with_capacity(sample_x.len() * sample_z.len());
    let mut colors = Vec::with_capacity(positions.capacity());
    let mut uvs = Vec::with_capacity(positions.capacity());
    for &local_z in &sample_z {
        for &local_x in &sample_x {
            let x = start_x + local_x;
            let z = start_z + local_z;
            let elevation = terrain_corner_height(world, x, z);
            positions.push([
                (f32::from(x) - f32::from(world_width.saturating_sub(1)) * 0.5)
                    * config.world.cell_size,
                elevation,
                (f32::from(z) - f32::from(world_height.saturating_sub(1)) * 0.5)
                    * config.world.cell_size,
            ]);
            colors.push(terrain_vertex_color(elevation, config));
            uvs.push([
                f32::from(x) / f32::from(world_width),
                f32::from(z) / f32::from(world_height),
            ]);
        }
    }

    let mut indices = Vec::with_capacity((sample_x.len() - 1) * (sample_z.len() - 1) * 6);
    for z in 0..u32::try_from(sample_z.len() - 1).expect("terrain rows fit u32") {
        for x in 0..u32::try_from(sample_x.len() - 1).expect("terrain columns fit u32") {
            let top_left = z * columns + x;
            let top_right = top_left + 1;
            let bottom_left = top_left + columns;
            let bottom_right = bottom_left + 1;
            indices.extend_from_slice(&[
                top_left,
                bottom_left,
                top_right,
                top_right,
                bottom_left,
                bottom_right,
            ]);
        }
    }

    if add_skirts {
        let skirt_depth = config.world.cell_size.max(1.0) * 0.75;
        let top = sample_x.iter().copied().map(|x| (x, 0)).collect::<Vec<_>>();
        let bottom = sample_x
            .iter()
            .copied()
            .map(|x| (x, cells_z))
            .collect::<Vec<_>>();
        let left = sample_z.iter().copied().map(|z| (0, z)).collect::<Vec<_>>();
        let right = sample_z
            .iter()
            .copied()
            .map(|z| (cells_x, z))
            .collect::<Vec<_>>();
        for edge in [&top, &bottom, &left, &right] {
            append_terrain_skirt(
                world,
                config,
                start_x,
                start_z,
                edge,
                skirt_depth,
                &mut positions,
                &mut colors,
                &mut uvs,
                &mut indices,
            );
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices));
    mesh.compute_smooth_normals();
    mesh
}

fn terrain_lod_samples(cells: u16, step: u16) -> Vec<u16> {
    let step = step.max(1);
    let mut samples: Vec<_> = (0..=cells).step_by(usize::from(step)).collect();
    if samples.last().copied() != Some(cells) {
        samples.push(cells);
    }
    samples
}

#[allow(clippy::too_many_arguments)]
fn append_terrain_skirt(
    world: &GeneratedWorld,
    config: &GameConfig,
    start_x: u16,
    start_z: u16,
    edge: &[(u16, u16)],
    depth: f32,
    positions: &mut Vec<[f32; 3]>,
    colors: &mut Vec<[f32; 4]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
) {
    let world_width = world.navigation.width();
    let world_height = world.navigation.height();
    for pair in edge.windows(2) {
        let base = u32::try_from(positions.len()).expect("terrain mesh vertex count fits u32");
        for (vertex_index, &(local_x, local_z)) in
            [pair[0], pair[0], pair[1], pair[1]].iter().enumerate()
        {
            let x = start_x + local_x;
            let z = start_z + local_z;
            let elevation = terrain_corner_height(world, x, z);
            let lower = vertex_index % 2 == 1;
            positions.push([
                (f32::from(x) - f32::from(world_width.saturating_sub(1)) * 0.5)
                    * config.world.cell_size,
                elevation - if lower { depth } else { 0.0 },
                (f32::from(z) - f32::from(world_height.saturating_sub(1)) * 0.5)
                    * config.world.cell_size,
            ]);
            colors.push(terrain_vertex_color(elevation, config));
            uvs.push([
                f32::from(x) / f32::from(world_width),
                f32::from(z) / f32::from(world_height),
            ]);
        }
        indices.extend_from_slice(&[
            base,
            base + 1,
            base + 2,
            base + 2,
            base + 1,
            base + 3,
            base,
            base + 2,
            base + 1,
            base + 2,
            base + 3,
            base + 1,
        ]);
    }
}

fn generated_water_mesh(world: &GeneratedWorld, config: &GameConfig) -> Mesh {
    // Unity's Water_Main plane is vastly wider than the authored terrain. The
    // previous eight-cell skirt exposed the clear colour as a black bar at the
    // legal maximum zoom, so retain enough ocean to cover that full view.
    let width = world.navigation.width();
    let height = world.navigation.height();
    let padded_width = width + OCEAN_PADDING_CELLS * 2;
    let padded_height = height + OCEAN_PADDING_CELLS * 2;
    let columns = u32::from(padded_width) + 1;
    let water_height = f32::from(config.world.water_level_centimetres) * 0.01;
    let authored_depth_range = 10.0_f32;
    let mut positions =
        Vec::with_capacity(usize::from(padded_width + 1) * usize::from(padded_height + 1));
    let mut normals = Vec::with_capacity(positions.capacity());
    let mut colors = Vec::with_capacity(positions.capacity());
    let mut uvs = Vec::with_capacity(positions.capacity());
    for z in 0..=padded_height {
        for x in 0..=padded_width {
            let terrain_height = if (OCEAN_PADDING_CELLS..=OCEAN_PADDING_CELLS + width).contains(&x)
                && (OCEAN_PADDING_CELLS..=OCEAN_PADDING_CELLS + height).contains(&z)
            {
                terrain_corner_height(world, x - OCEAN_PADDING_CELLS, z - OCEAN_PADDING_CELLS)
            } else {
                water_height - authored_depth_range
            };
            let depth = (water_height - terrain_height).max(0.0);
            let normalized_depth = (depth / authored_depth_range).clamp(0.0, 1.0);
            positions.push([
                (f32::from(x)
                    - f32::from(OCEAN_PADDING_CELLS)
                    - f32::from(width.saturating_sub(1)) * 0.5)
                    * config.world.cell_size,
                water_height + VISIBLE_WATER_SURFACE_LIFT_METRES,
                (f32::from(z)
                    - f32::from(OCEAN_PADDING_CELLS)
                    - f32::from(height.saturating_sub(1)) * 0.5)
                    * config.world.cell_size,
            ]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push([normalized_depth, 0.0, 0.0, 1.0]);
            uvs.push([
                f32::from(x) / f32::from(padded_width),
                f32::from(z) / f32::from(padded_height),
            ]);
        }
    }

    let mut indices =
        Vec::with_capacity(usize::from(padded_width) * usize::from(padded_height) * 6);
    for z in 0..u32::from(padded_height) {
        for x in 0..u32::from(padded_width) {
            let top_left = z * columns + x;
            let top_right = top_left + 1;
            let bottom_left = top_left + columns;
            let bottom_right = bottom_left + 1;
            indices.extend_from_slice(&[
                top_left,
                bottom_left,
                top_right,
                top_right,
                bottom_left,
                bottom_right,
            ]);
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

fn healing_ring_mesh(segments: u32) -> Mesh {
    let segments = u16::try_from(segments.max(3).min(u32::from(u16::MAX)))
        .expect("clamped ring segment count fits u16");
    let mut positions = Vec::with_capacity(usize::from(segments) * 2);
    let mut normals = Vec::with_capacity(positions.capacity());
    let mut uvs = Vec::with_capacity(positions.capacity());
    for index in 0..segments {
        let angle = f32::from(index) / f32::from(segments) * std::f32::consts::TAU;
        let direction = Vec2::new(angle.cos(), angle.sin());
        for (radius, uv_y) in [(0.84, 0.0), (1.0, 1.0)] {
            positions.push([direction.x * radius, 0.0, direction.y * radius]);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([f32::from(index) / f32::from(segments), uv_y]);
        }
    }
    let mut indices = Vec::with_capacity(usize::from(segments) * 6);
    for index in 0..segments {
        let next = (index + 1) % segments;
        let inner = u32::from(index) * 2;
        let outer = inner + 1;
        let next_inner = u32::from(next) * 2;
        let next_outer = next_inner + 1;
        indices.extend_from_slice(&[inner, next_inner, outer, next_inner, next_outer, outer]);
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

fn retained_terrain_mesh(saved: &SavedTerrainMesh) -> AnyResult<Mesh> {
    saved.validate().context("invalid retained terrain mesh")?;
    let uvs = if saved.uvs.is_empty() {
        generated_terrain_uvs(&saved.vertices)
    } else {
        saved.uvs.clone()
    };
    let indices = if saved.uses_32_bit_indices || saved.vertices.len() > usize::from(u16::MAX) {
        Indices::U32(
            saved
                .triangle_indices
                .iter()
                .map(|index| u32::try_from(*index).expect("validated non-negative mesh index"))
                .collect(),
        )
    } else {
        Indices::U16(
            saved
                .triangle_indices
                .iter()
                .map(|index| u16::try_from(*index).expect("validated 16-bit mesh index"))
                .collect(),
        )
    };
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, saved.vertices.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(indices);
    mesh.compute_smooth_normals();
    Ok(mesh)
}

fn generated_terrain_uvs(vertices: &[[f32; 3]]) -> Vec<[f32; 2]> {
    let (mut min_x, mut max_x, mut min_z, mut max_z) = (
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::INFINITY,
        f32::NEG_INFINITY,
    );
    for vertex in vertices {
        min_x = min_x.min(vertex[0]);
        max_x = max_x.max(vertex[0]);
        min_z = min_z.min(vertex[2]);
        max_z = max_z.max(vertex[2]);
    }
    let span_x = (max_x - min_x).max(f32::EPSILON);
    let span_z = (max_z - min_z).max(f32::EPSILON);
    vertices
        .iter()
        .map(|vertex| [(vertex[0] - min_x) / span_x, (vertex[2] - min_z) / span_z])
        .collect()
}

fn terrain_corner_height(world: &GeneratedWorld, corner_x: u16, corner_z: u16) -> f32 {
    navigation_corner_height_metres(&world.navigation, corner_x, corner_z)
}

/// Samples the same two triangles emitted by `generated_terrain_chunk_mesh` at
/// full detail. Decorative objects must use this rendered surface rather than
/// a navigation cell's raw height: terrain vertices average adjacent cells,
/// and the object's deterministic X/Z offset can move it across a slope.
fn terrain_surface_height_at_world(
    world: &GeneratedWorld,
    config: &GameConfig,
    world_x: f32,
    world_z: f32,
) -> Option<f32> {
    navigation_surface_height_at_world(&world.navigation, &config.world, world_x, world_z)
}

fn resolved_foliage_ground_position(
    world: &GeneratedWorld,
    config: &GameConfig,
    foliage: &GeneratedFoliage,
) -> Option<Vec3> {
    let mut position = grid_to_world(foliage.position, config);
    let offset = locational_visual_offset(
        world.seed,
        &foliage.id,
        foliage.position,
        foliage.offset_milli_cells,
        config.world.cell_size,
    );
    position.x += offset.x;
    position.z += offset.y;
    position.y = terrain_surface_height_at_world(world, config, position.x, position.z)?;

    let visible_water_height = visible_water_surface_height(&config.world);
    match foliage.habitat {
        FoliageHabitat::Land if position.y <= visible_water_height => None,
        FoliageHabitat::Underwater if position.y >= visible_water_height => None,
        FoliageHabitat::Land | FoliageHabitat::Underwater => Some(position),
    }
}

fn terrain_vertex_color(elevation: f32, config: &GameConfig) -> [f32; 4] {
    let water = f32::from(config.world.water_level_centimetres) * 0.01;
    if elevation <= water {
        return [0.12, 0.17, 0.13, 1.0];
    }
    let peak = f32::from(config.world.height_scale_centimetres).max(1.0) * 0.01;
    let normalized = ((elevation - water) / (peak - water).max(0.01)).clamp(0.0, 1.0);
    [
        0.10 + normalized * 0.22,
        0.28 + normalized * 0.25,
        0.12 + normalized * 0.12,
        1.0,
    ]
}

fn world_to_grid(position: Vec3, config: &GameConfig) -> Option<GridPos> {
    let x = (position.x / config.world.cell_size
        + f32::from(config.world.width.saturating_sub(1)) * 0.5)
        .round();
    let z = (position.z / config.world.cell_size
        + f32::from(config.world.height.saturating_sub(1)) * 0.5)
        .round();
    if x < 0.0
        || z < 0.0
        || x >= f32::from(config.world.width)
        || z >= f32::from(config.world.height)
    {
        return None;
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Some(GridPos {
        x: x as u16,
        z: z as u16,
    })
}

fn world_to_navigation(position: Vec3, config: &GameConfig) -> Option<GridPos> {
    let subdivision = f32::from(NAVIGATION_SUBDIVISIONS);
    let half = f32::from(NAVIGATION_SUBDIVISIONS / 2);
    let x = ((position.x / config.world.cell_size
        + f32::from(config.world.width.saturating_sub(1)) * 0.5)
        * subdivision
        + half)
        .round();
    let z = ((position.z / config.world.cell_size
        + f32::from(config.world.height.saturating_sub(1)) * 0.5)
        * subdivision
        + half)
        .round();
    let width = f32::from(config.world.width.saturating_mul(NAVIGATION_SUBDIVISIONS));
    let height = f32::from(config.world.height.saturating_mul(NAVIGATION_SUBDIVISIONS));
    if x < 0.0 || z < 0.0 || x >= width || z >= height {
        return None;
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Some(GridPos {
        x: x as u16,
        z: z as u16,
    })
}

#[cfg(test)]
mod tests;
