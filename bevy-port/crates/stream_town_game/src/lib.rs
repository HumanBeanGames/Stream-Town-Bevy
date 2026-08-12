pub mod twitch;

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result as AnyResult};
use avian3d::prelude::{Collider, PhysicsPlugins, RigidBody, SpatialQuery, SpatialQueryFilter};
use bevy::{
    animation::{
        AnimatedBy, AnimationClip, AnimationTargetId, animated_field,
        graph::{AnimationGraph, AnimationGraphHandle, AnimationNodeIndex, AnimationNodeType},
        prelude::{AnimatableCurve, AnimatableKeyframeCurve},
    },
    asset::{AssetPlugin, RenderAssetUsages},
    audio::{Pitch, Volume},
    camera::ScalingMode,
    color::LinearRgba,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::system::SystemParam,
    gltf::{GltfMaterialName, GltfMeshName},
    math::Affine2,
    mesh::Indices,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, PrimitiveTopology, ShaderType},
    render::view::screenshot::{Screenshot, save_to_disk},
    shader::ShaderRef,
    window::{PresentMode, PrimaryWindow, WindowResolution},
};
use stream_town_domain::{
    ActorCustomization, ActorKind, ActorState, AnimationBlendSelection, AnimationClipDef,
    AnimationControllerRuntime, AnimationLayerBlendMode, AnimationLayerDef,
    AnimationTransformTrack, AnimationTransitionPlayback, ArchetypeDef, ArchetypeKind,
    ArchetypeScene, AvatarMaskDef, BUILDING_MAX_HEALTH, BuildingAction, BuildingDef,
    BuildingDirection, BuildingState, CameraAction, CameraDirection, ChatCommand, ContentCatalog,
    CustomizationKind, EnemyCampState, GameConfig, GeneratedFoliage, GeneratedWorld, GridPos,
    LegacyMigrationMetadata, MaterialAlphaMode as AuthoredAlphaMode, MaterialDef, NativeSaveStore,
    ObjectiveEvent, PresentationCatalog, RoleEquipmentDef, RulerVoteKind, SavedActor,
    SavedTerrainMesh, Season, StableId, StationDef, TownEvent, Weather, WorldSimulation,
    WorldSnapshot, generate_world_with_content,
};

const MAX_TOWN_GOALS: usize = 2;
const PASSIVE_RESOURCE_FIXED_POINT_DENOMINATOR: u128 = 1_000_000_000_000;
const DEFAULT_ACTOR_DETAIL_BUDGET: usize = 16;
const FISH_GOD_REWARD_ID: &str = "5a760033-50b5-4e47-911b-d63993d2860c";
const TERRAIN_SHADER_ASSET_PATH: &str = "shaders/terrain_material.wgsl";
const TERRAIN_MATERIAL_PATH: &str = "Assets/Materials/Environment/Env_Terrain.mat";
const WATER_SHADER_ASSET_PATH: &str = "shaders/water_material.wgsl";
const WATER_MATERIAL_PATH: &str = "Assets/Materials/Environment/Env_Water.mat";
const BUILDING_SHADER_ASSET_PATH: &str = "shaders/building_material.wgsl";
const BUILDING_MATERIAL_PATH: &str = "Assets/Materials/Building_Material.mat";
const CLOUD_SHADER_ASSET_PATH: &str = "shaders/cloud_material.wgsl";
const CLOUD_MATERIAL_PATH: &str = "Assets/Materials/VFX/Clouds.mat";
const TREE_SHADER_ASSET_PATH: &str = "shaders/tree_material.wgsl";
const TREE_MATERIAL_PATH: &str = "Assets/Materials/Environment/Env_Tree.mat";
const FOLIAGE_VISIBILITY_RANGE: f32 = 420.0;
const HEALED_BURST_SECONDS: f32 = 1.2;
const HEALING_CHANNEL_SECONDS: f32 = 5.0;
const CHARACTER_HIT_SECONDS: f32 = 0.25;
const TOWER_TRAIL_SECONDS: f32 = 2.0;
const TOWER_TRAIL_WIDTH: f32 = 0.1;
const FIREBALL_SIZE: f32 = 0.4;
const FIREBALL_TRAIL_SIZE: f32 = 0.3;
const BUILDING_HIT_SECONDS: f32 = 0.5;
const BUILDING_HIT_SMOKE_SPEED: f32 = 3.0;
const BUILDING_HIT_SPARK_SPEED: f32 = 12.0;
const BUILDING_HIT_SMOKE_SIZE: f32 = 0.5;
const BUILDING_HIT_SPARK_SIZE: f32 = 0.25;
const BUILDING_LEVEL_UP_SECONDS: f32 = 1.5;
const BUILDING_LEVEL_UP_ARROW_SIZE: f32 = 0.5;
const BUILDING_LEVEL_UP_TILE_SIZE: f32 = 4.0;
const BUILDING_DAMAGED_RADIUS: f32 = 1.403_639_8;
const BUILDING_DAMAGED_FIRE_AMOUNT: u16 = 128;
const BUILDING_DAMAGED_SMOKE_AMOUNT: u16 = 200;
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
use twitch::{TwitchControl, TwitchEvent, TwitchStatus, TwitchTransport};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Boot,
    MainMenu,
    WorldLoading,
    InGame,
    Credits,
}

#[derive(Resource)]
pub struct RuntimeConfig(pub GameConfig);

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

#[derive(Resource)]
struct SaveRuntime {
    store: NativeSaveStore,
}

#[derive(SystemParam)]
struct LoadRenderParams<'w, 's> {
    presentation: Res<'w, RuntimePresentation>,
    render: Res<'w, RenderAssets>,
    meshes: Option<ResMut<'w, Assets<Mesh>>>,
    asset_server: Option<Res<'w, AssetServer>>,
    asset_root: Res<'w, RuntimeAssetRoot>,
    terrain_surfaces: Query<'w, 's, Entity, With<TerrainSurface>>,
}

#[derive(Resource)]
struct SimulationRuntime(WorldSimulation);

#[derive(Resource, Default)]
struct SessionStats {
    elapsed_seconds: f64,
    paths_completed: u64,
    commands_processed: u64,
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
    position: GridPos,
    rotation_quarter_turns: i32,
}

#[derive(Resource, Default)]
struct BuildingPlacers(BTreeMap<StableId, BuildingPlacement>);

#[derive(SystemParam)]
struct RuntimeCommandQueues<'w> {
    injected: ResMut<'w, InjectedCommands>,
    camera: ResMut<'w, CameraCommandQueue>,
    agent: ResMut<'w, AgentCommandQueue>,
    building: ResMut<'w, BuildingCommandQueue>,
    placers: ResMut<'w, BuildingPlacers>,
}

#[derive(Clone, Debug)]
struct CameraRequest {
    reset: bool,
    actions: Vec<CameraAction>,
}

#[derive(Clone, Debug)]
enum AgentCommand {
    Teleport { actor: StableId, position: GridPos },
    Despawn(StableId),
}

#[derive(Resource)]
struct TwitchConnection {
    transport: Option<TwitchTransport>,
    status: TwitchStatus,
    broadcaster_authorized: bool,
    connect_code: String,
}

impl Default for TwitchConnection {
    fn default() -> Self {
        Self {
            transport: None,
            status: TwitchStatus::Disabled,
            broadcaster_authorized: false,
            connect_code: generate_connect_code(),
        }
    }
}

#[derive(Resource, Default)]
struct SelectedCell(Option<GridPos>);

#[derive(Resource, Default)]
struct EnvironmentPresentation {
    applied: Option<(Season, Weather)>,
}

#[derive(Resource, Default)]
struct BuildingMaterialInstances(BTreeMap<StableId, BuildingMaterialInstance>);

struct BuildingMaterialInstance {
    handle: Handle<BuildingMaterial>,
    applied_health: i32,
    applied_season: Season,
}

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct TerrainMaterialUniform {
    sand_color_a: Vec4,
    sand_color_b: Vec4,
    grass_color_a: Vec4,
    grass_color_b: Vec4,
    season_tint: Vec4,
    texture_uv_blend_tint: Vec4,
    grid_scale_offset: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct TerrainMaterialExtension {
    #[uniform(100)]
    parameters: TerrainMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    grid_texture: Option<Handle<Image>>,
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
    fn vertex_shader() -> ShaderRef {
        TREE_SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        TREE_SHADER_ASSET_PATH.into()
    }
}

type TreeMaterial = ExtendedMaterial<StandardMaterial, TreeMaterialExtension>;

#[derive(Clone)]
enum ResolvedMaterialHandle {
    Standard(Handle<StandardMaterial>),
    Building(Handle<BuildingMaterial>),
    Cloud(Handle<CloudMaterial>),
    Tree(Handle<TreeMaterial>),
}

#[derive(Resource, Default)]
struct RenderAssets {
    cube: Handle<Mesh>,
    actor_lod: Handle<Mesh>,
    cloud_plane: Handle<Mesh>,
    healing_ring: Handle<Mesh>,
    projectile_arrow_scene: Option<Handle<bevy::world_serialization::WorldAsset>>,
    ground: Handle<TerrainMaterial>,
    water: Handle<WaterMaterial>,
    wood: Handle<StandardMaterial>,
    ore: Handle<StandardMaterial>,
    food: Handle<StandardMaterial>,
    building: Handle<StandardMaterial>,
    construction: Handle<StandardMaterial>,
    placement_valid: Handle<StandardMaterial>,
    placement_invalid: Handle<StandardMaterial>,
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
    building_spark: Handle<StandardMaterial>,
    building_fire: Handle<StandardMaterial>,
    building_upgrade: Handle<StandardMaterial>,
    healing_green: Handle<StandardMaterial>,
    healing_gold: Handle<StandardMaterial>,
    authored_building: Handle<BuildingMaterial>,
    clouds: Handle<CloudMaterial>,
    tree: Handle<TreeMaterial>,
    presentation_materials: BTreeMap<StableId, ResolvedMaterialHandle>,
}

#[derive(Component)]
struct StateEntity;

#[derive(Component)]
struct AuthoredCreditsElement {
    target_path: String,
}

#[derive(Component)]
struct CreditsFade;

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
struct TerrainSurface;

#[derive(Component)]
struct Agent {
    id: StableId,
    kind: ActorKind,
    archetype: StableId,
    goal: AgentGoal,
    spawn: GridPos,
    origin: GridPos,
    path: Vec<GridPos>,
    path_index: usize,
    target: GridPos,
    action_cooldown_seconds: f32,
    health_regen_accumulator: f64,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
enum AgentGoal {
    #[default]
    Wander,
    Gather(StableId),
    Deposit,
    Attack(StableId),
    AttackBuilding(StableId),
    Heal(StableId),
    Construct(StableId),
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
struct FoliageVisual;

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct TownHall;

#[derive(Component)]
struct RuntimeBuilding {
    id: StableId,
}

#[derive(Component)]
struct BuildingPlacementVisual {
    owner: StableId,
}

#[derive(Component)]
struct EnemyCamp {
    id: StableId,
}

#[derive(Component)]
struct FishGodPresentation;

#[derive(Component)]
struct FallingFish {
    floor_height: f32,
    top_height: f32,
    fall_speed: f32,
}

#[derive(Component)]
struct TowerShooter {
    building: StableId,
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
        source: GridPos,
        target: GridPos,
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
    origin: Vec3,
    elapsed_seconds: f32,
    base_scale: f32,
}

#[derive(Component)]
struct HealingMoteEffect {
    kind: HealingEffectKind,
    origin: Vec3,
    elapsed_seconds: f32,
    angle_radians: f32,
    phase: f32,
    base_scale: Vec3,
    distance_scale: f32,
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
}

#[derive(Component)]
struct SelectionMarker;

#[derive(Component)]
struct TownCamera;

#[derive(Component)]
struct ActivePetVisual {
    owner: StableId,
    pet: StableId,
}

#[derive(Component)]
struct WeatherParticle {
    kind: Weather,
    seed: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct EnvironmentPalette {
    terrain_tint: [f32; 3],
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

#[derive(Component)]
struct EquipmentNode {
    actor_root: Entity,
    name: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CosmeticNodeKind {
    Eyes,
    Hair,
    FacialHair,
}

#[derive(Component)]
struct CosmeticNode {
    actor_root: Entity,
    kind: CosmeticNodeKind,
    index: u8,
}

#[derive(Component)]
struct CosmeticRenderer {
    actor_root: Entity,
    kind: CosmeticNodeKind,
    base_material: Handle<StandardMaterial>,
    applied_color: Option<u8>,
}

struct CosmeticMaterialVariant {
    base_material: Handle<StandardMaterial>,
    kind: CosmeticNodeKind,
    color: u8,
    material: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
struct CosmeticMaterialCache(Vec<CosmeticMaterialVariant>);

#[derive(Resource, Default)]
struct RoleActionAudioCache(BTreeMap<StableId, Handle<Pitch>>);

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
}

struct ConvertedAnimationLayerDriver {
    display_name: String,
    fallback_state: StableId,
    runtime: AnimationControllerRuntime,
    nodes: BTreeMap<StableId, AnimationNodeIndex>,
    active: Vec<(AnimationNodeIndex, f32)>,
    applied: Vec<(AnimationNodeIndex, f32, f32)>,
    crossfade: Option<ConvertedAnimationCrossfade>,
    state_offset: f32,
    event_elapsed: BTreeMap<StableId, f32>,
}

#[derive(Clone)]
struct ConvertedAnimationLayerTemplate {
    display_name: String,
    fallback_state: StableId,
    nodes: BTreeMap<StableId, AnimationNodeIndex>,
}

struct CachedConvertedAnimation {
    graph: Handle<AnimationGraph>,
    layers: Vec<ConvertedAnimationLayerTemplate>,
    clip_count: usize,
}

#[derive(Resource, Default)]
struct ConvertedAnimationCache(BTreeMap<(StableId, StableId, String), CachedConvertedAnimation>);

#[derive(Clone)]
struct ConvertedAnimationCrossfade {
    source: Vec<(AnimationNodeIndex, f32, f32)>,
    elapsed: f32,
    duration: f32,
}

#[derive(Clone, Debug)]
struct PendingRoleActionAudio {
    actor: StableId,
    clip: StableId,
    display_name: String,
}

#[derive(Component, Clone)]
struct MaterialOverrideSpec {
    fallback: Option<ResolvedMaterialHandle>,
    model_materials: BTreeMap<String, ResolvedMaterialHandle>,
    renderer_materials: Vec<ResolvedRendererMaterialBinding>,
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

pub struct StreamTownGamePlugin;

impl Plugin for StreamTownGamePlugin {
    fn build(&self, app: &mut App) {
        if !app.world().contains_resource::<RuntimeConfig>() {
            app.insert_resource(RuntimeConfig(GameConfig::default()));
        }
        if !app.world().contains_resource::<RuntimeContent>() {
            app.insert_resource(RuntimeContent(embedded_content()));
        }
        if !app.world().contains_resource::<RuntimePresentation>() {
            app.insert_resource(RuntimePresentation(embedded_presentation()));
        }
        if !app.world().contains_resource::<RuntimeAssetRoot>() {
            app.insert_resource(RuntimeAssetRoot(locate_asset_root()));
        }
        app.init_state::<GameState>()
            .init_resource::<SessionStats>()
            .init_resource::<InjectedCommands>()
            .init_resource::<CommandFeedback>()
            .init_resource::<CameraCommandQueue>()
            .init_resource::<AgentCommandQueue>()
            .init_resource::<BuildingCommandQueue>()
            .init_resource::<BuildingPlacers>()
            .init_resource::<TwitchConnection>()
            .init_resource::<SelectedCell>()
            .init_resource::<EnvironmentPresentation>()
            .init_resource::<BuildingMaterialInstances>()
            .init_resource::<CosmeticMaterialCache>()
            .init_resource::<RoleActionAudioCache>()
            .init_resource::<NativeAnimationCache>()
            .init_resource::<ConvertedAnimationCache>()
            .init_resource::<LevelUpPresentation>()
            .insert_resource(SaveRuntime {
                store: NativeSaveStore::new(std::env::var_os("STREAM_TOWN_SAVE_PATH").map_or_else(
                    || PathBuf::from(".stream-town").join("StreamTownSave.stbevy"),
                    PathBuf::from,
                )),
            })
            .add_systems(Startup, (setup_rendering, start_twitch_transport))
            .add_systems(OnEnter(GameState::Boot), finish_boot)
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                upgrade_actor_placeholders.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    poll_twitch_transport,
                    twitch_connection_input,
                    capture_screenshot,
                    report_frame_time_gate,
                ),
            )
            .add_systems(
                Update,
                main_menu_input.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_state_entities)
            .add_systems(OnEnter(GameState::WorldLoading), generate_and_spawn_world)
            .add_systems(OnEnter(GameState::InGame), spawn_level_up_toast)
            .add_systems(
                Update,
                (
                    tag_equipment_nodes,
                    sync_equipment_nodes.after(tag_equipment_nodes),
                    tag_cosmetic_nodes,
                    sync_cosmetic_nodes.after(tag_cosmetic_nodes),
                    tag_cosmetic_renderers.after(apply_material_overrides),
                    sync_cosmetic_materials
                        .after(tag_cosmetic_renderers)
                        .after(apply_material_overrides),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    update_tower_shooters.after(move_agents),
                    move_combat_projectiles.after(update_tower_shooters),
                    animate_combat_effects.after(move_combat_projectiles),
                    repeat_combat_smoke.after(animate_combat_effects),
                    emit_damaged_building_effects.after(move_agents),
                    animate_building_effects.after(emit_damaged_building_effects),
                    repeat_building_smoke.after(animate_building_effects),
                    update_enemy_encounters.after(move_combat_projectiles),
                    sync_fish_god_presentation.after(update_enemy_encounters),
                    animate_falling_fish.after(sync_fish_god_presentation),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    move_agents,
                    sync_resource_nodes.after(move_agents),
                    sync_building_presentation.after(move_agents),
                    animate_agents,
                    resolve_native_animation_requests.after(upgrade_actor_placeholders),
                    attach_native_animations,
                    attach_converted_animations.after(upgrade_actor_placeholders),
                    drive_native_animations,
                    drive_converted_animations.after(move_agents),
                    apply_material_overrides,
                    update_environment_presentation.after(move_agents),
                    animate_weather_particles.after(update_environment_presentation),
                    camera_controls,
                    sync_active_pets,
                    select_grid_cell,
                    game_input,
                    save_input,
                    load_input,
                    drive_level_up_presentation.after(move_agents),
                    update_hud,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                animate_healing_effects
                    .after(move_agents)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    instantiate_building_materials.after(apply_material_overrides),
                    sync_building_material_instances.after(instantiate_building_materials),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    process_injected_commands.after(game_input),
                    apply_agent_commands.after(process_injected_commands),
                    apply_building_commands.after(process_injected_commands),
                    sync_building_placers.after(process_injected_commands),
                    sync_foliage_clearance
                        .after(apply_building_commands)
                        .after(load_input)
                        .after(move_agents),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), cleanup_world)
            .add_systems(OnEnter(GameState::Credits), spawn_credits)
            .add_systems(
                Update,
                (drive_credits_animation, credits_input)
                    .chain()
                    .run_if(in_state(GameState::Credits)),
            )
            .add_systems(OnExit(GameState::Credits), cleanup_state_entities);
    }
}

pub fn run(config: GameConfig) {
    let content = embedded_content();
    let asset_root = locate_asset_root();
    let resolution = WindowResolution::new(config.window.width, config.window.height);
    let title = config.window.title.clone();
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.025, 0.04, 0.055)))
        .insert_resource(RuntimeConfig(config))
        .insert_resource(RuntimeContent(content))
        .insert_resource(RuntimePresentation(embedded_presentation()))
        .insert_resource(RuntimeAssetRoot(asset_root.clone()))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: asset_root.to_string_lossy().into_owned(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title,
                        resolution,
                        present_mode: if std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some()
                        {
                            PresentMode::AutoNoVsync
                        } else {
                            PresentMode::AutoVsync
                        },
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::new(600))
        .add_plugins(MaterialPlugin::<TerrainMaterial>::default())
        .add_plugins(MaterialPlugin::<WaterMaterial>::default())
        .add_plugins(MaterialPlugin::<BuildingMaterial>::default())
        .add_plugins(MaterialPlugin::<CloudMaterial>::default())
        .add_plugins(MaterialPlugin::<TreeMaterial>::default())
        .add_plugins(StreamTownGamePlugin)
        .run();
}

pub fn load_runtime_config() -> AnyResult<GameConfig> {
    let configured = std::env::var_os("STREAM_TOWN_CONFIG").map(PathBuf::from);
    let user_config = PathBuf::from(".stream-town").join("config.ron");
    let encoded = if let Some(path) = configured {
        std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read runtime config {}", path.display()))?
    } else if user_config.is_file() {
        std::fs::read_to_string(&user_config)
            .with_context(|| format!("failed to read runtime config {}", user_config.display()))?
    } else {
        include_str!("../../../assets/config/game.ron").to_owned()
    };
    let config: GameConfig = ron::from_str(&encoded).context("runtime config is invalid RON")?;
    config
        .validate()
        .context("runtime config failed validation")?;
    Ok(config)
}

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
    presentation: Res<RuntimePresentation>,
    asset_server: Option<Res<AssetServer>>,
    meshes: Option<ResMut<Assets<Mesh>>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
    terrain_materials: Option<ResMut<Assets<TerrainMaterial>>>,
    water_materials: Option<ResMut<Assets<WaterMaterial>>>,
    building_materials: Option<ResMut<Assets<BuildingMaterial>>>,
    cloud_materials: Option<ResMut<Assets<CloudMaterial>>>,
    tree_materials: Option<ResMut<Assets<TreeMaterial>>>,
) {
    let (
        Some(mut meshes),
        Some(mut materials),
        Some(mut terrain_materials),
        Some(mut water_materials),
        Some(mut building_materials),
        Some(mut cloud_materials),
        Some(mut tree_materials),
    ) = (
        meshes,
        materials,
        terrain_materials,
        water_materials,
        building_materials,
        cloud_materials,
        tree_materials,
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
    let building_closeup = std::env::var_os("STREAM_TOWN_SMOKE_BUILDING_VFX").is_some();
    let foliage_closeup = std::env::var_os("STREAM_TOWN_SMOKE_FOLIAGE").is_some();
    let shoreline_closeup = std::env::var_os("STREAM_TOWN_SMOKE_SHORELINE").is_some();
    commands.spawn((
        TownCamera,
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: if material_closeup {
                    96.0
                } else if animation_closeup {
                    180.0
                } else if resource_closeup {
                    24.0
                } else if healing_closeup {
                    42.0
                } else if combat_closeup {
                    48.0
                } else if building_closeup {
                    58.0
                } else if foliage_closeup {
                    45.0
                } else if shoreline_closeup {
                    80.0
                } else {
                    520.0
                },
            },
            ..OrthographicProjection::default_3d()
        }),
        AmbientLight {
            color: Color::srgb(0.70, 0.82, 0.92),
            brightness: 90.0,
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
        Transform::from_xyz(360.0, 420.0, 360.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 14_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(250.0, 400.0, 180.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let authored_building =
        building_materials.add(building_material(&presentation.0, asset_server.as_deref()));
    let clouds = cloud_materials.add(cloud_material(&presentation.0, asset_server.as_deref()));
    let tree = tree_materials.add(tree_material(&presentation.0, asset_server.as_deref()));
    let presentation_materials = presentation
        .0
        .materials
        .iter()
        .map(|(id, material)| {
            let resolved = if material.source_path == BUILDING_MATERIAL_PATH {
                ResolvedMaterialHandle::Building(authored_building.clone())
            } else if material.source_path == CLOUD_MATERIAL_PATH {
                ResolvedMaterialHandle::Cloud(clouds.clone())
            } else if material.source_path == TREE_MATERIAL_PATH {
                ResolvedMaterialHandle::Tree(tree.clone())
            } else {
                ResolvedMaterialHandle::Standard(materials.add(standard_material(
                    material,
                    &presentation.0,
                    asset_server.as_deref(),
                )))
            };
            (id.clone(), resolved)
        })
        .collect();
    commands.insert_resource(RenderAssets {
        cube: meshes.add(Cuboid::default()),
        actor_lod: meshes.add(Capsule3d::new(0.42, 1.45)),
        cloud_plane: meshes.add(Plane3d::default().mesh().size(1.0, 1.0)),
        healing_ring: meshes.add(healing_ring_mesh(48)),
        projectile_arrow_scene: asset_server.as_deref().map(|asset_server| {
            asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("migrated/models/Models/Combat/Arrow.glb"),
            )
        }),
        ground: terrain_materials.add(terrain_material(
            &presentation.0,
            &config.0,
            asset_server.as_deref(),
        )),
        water: water_materials.add(water_material(&presentation.0, asset_server.as_deref())),
        wood: materials.add(Color::srgb(0.16, 0.46, 0.18)),
        ore: materials.add(Color::srgb(0.46, 0.50, 0.55)),
        food: materials.add(Color::srgb(0.74, 0.64, 0.18)),
        building: materials.add(Color::srgb(0.42, 0.26, 0.12)),
        construction: materials.add(StandardMaterial {
            base_color: Color::srgb(0.72, 0.51, 0.24),
            perceptual_roughness: 0.88,
            ..default()
        }),
        placement_valid: materials.add(StandardMaterial {
            base_color: Color::srgba(0.18, 0.9, 0.34, 0.46),
            emissive: LinearRgba::new(0.06, 0.8, 0.12, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        placement_invalid: materials.add(StandardMaterial {
            base_color: Color::srgba(0.95, 0.12, 0.08, 0.54),
            emissive: LinearRgba::new(1.0, 0.03, 0.01, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        enemy_idle: materials.add(Color::srgb(0.72, 0.12, 0.12)),
        enemy_moving: materials.add(Color::srgb(1.0, 0.28, 0.22)),
        player_idle: materials.add(Color::srgb(0.35, 0.72, 0.95)),
        player_moving: materials.add(Color::srgb(0.52, 0.86, 1.0)),
        selection: materials.add(StandardMaterial {
            base_color: Color::srgba(0.95, 0.92, 0.18, 0.35),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
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
        healing_gold: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.84, 0.18, 0.86),
            emissive: LinearRgba::new(4.0, 2.4, 0.16, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        authored_building,
        clouds,
        tree,
        presentation_materials,
    });
}

fn standard_material(
    material: &MaterialDef,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> StandardMaterial {
    let primary_texture = primary_material_texture_entry(material, presentation);
    let base_color_texture = asset_server.and_then(|asset_server| {
        primary_texture.map(|(_, path)| asset_server.load(path.to_owned()))
    });
    let texture_transform = primary_texture
        .and_then(|(slot, _)| material.texture_transforms.get(slot))
        .copied()
        .unwrap_or_default();
    let alpha_mode = match material.alpha_mode {
        AuthoredAlphaMode::Opaque => AlphaMode::Opaque,
        AuthoredAlphaMode::Mask => AlphaMode::Mask(0.5),
        AuthoredAlphaMode::Blend => AlphaMode::Blend,
    };
    StandardMaterial {
        base_color: Color::srgba(
            material.base_color[0],
            material.base_color[1],
            material.base_color[2],
            material.base_color[3],
        ),
        base_color_texture,
        emissive: LinearRgba::new(
            material.emissive[0],
            material.emissive[1],
            material.emissive[2],
            material.emissive[3],
        ),
        metallic: material.metallic,
        perceptual_roughness: material.perceptual_roughness,
        alpha_mode,
        uv_transform: Affine2::from_scale_angle_translation(
            Vec2::from_array(texture_transform.scale),
            0.0,
            Vec2::from_array(texture_transform.offset),
        ),
        ..default()
    }
}

fn terrain_material(
    presentation: &PresentationCatalog,
    config: &GameConfig,
    asset_server: Option<&AssetServer>,
) -> TerrainMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == TERRAIN_MATERIAL_PATH);
    let vector = |name: &str, fallback: [f32; 4]| {
        authored
            .and_then(|material| material.custom_vectors.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let texture_uv = vector("_TextureUV", [0.5, 0.5, 0.0, 0.0]);
    let grid_transform = authored
        .and_then(|material| material.texture_transforms.get("_MainTexture"))
        .copied()
        .unwrap_or_default();
    let grid_texture = authored.and_then(|material| {
        asset_server.and_then(|asset_server| {
            material
                .textures
                .get("_MainTexture")
                .and_then(|id| presentation.textures.get(id))
                .map(|texture| asset_server.load(texture.asset_path.clone()))
        })
    });
    TerrainMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: authored.map_or(0.96, |material| material.perceptual_roughness),
            ..default()
        },
        extension: TerrainMaterialExtension {
            parameters: TerrainMaterialUniform {
                sand_color_a: Vec4::from_array(vector(
                    "_SandGridColor1",
                    [0.934, 0.773, 0.084, 1.0],
                )),
                sand_color_b: Vec4::from_array(vector(
                    "_SandGridColor2",
                    [0.823, 0.681, 0.071, 1.0],
                )),
                grass_color_a: Vec4::from_array(vector("_color1", [0.422, 0.498, 0.153, 1.0])),
                grass_color_b: Vec4::from_array(vector("_color2", [0.406, 0.471, 0.141, 1.0])),
                season_tint: Vec4::new(1.0, 1.0, 1.0, scalar("_Tint", 0.0)),
                texture_uv_blend_tint: Vec4::new(
                    texture_uv[0],
                    texture_uv[1],
                    f32::from(config.world.water_level_centimetres) * 0.01,
                    scalar("_BlendHeight", 1.0).max(0.01),
                ),
                grid_scale_offset: Vec4::new(
                    grid_transform.scale[0],
                    grid_transform.scale[1],
                    grid_transform.offset[0],
                    grid_transform.offset[1],
                ),
            },
            grid_texture,
        },
    }
}

fn water_material(
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> WaterMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == WATER_MATERIAL_PATH);
    let vector = |name: &str, fallback: [f32; 4]| {
        authored
            .and_then(|material| material.custom_vectors.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let texture = |slot: &str| {
        authored.and_then(|material| {
            asset_server.and_then(|asset_server| {
                material
                    .textures
                    .get(slot)
                    .and_then(|id| presentation.textures.get(id))
                    .map(|texture| asset_server.load(texture.asset_path.clone()))
            })
        })
    };
    let wind = vector("_windDirection", [1.0, 0.0, 0.0, 0.0]);
    let main_transform = authored
        .and_then(|material| material.texture_transforms.get("_MainTexture"))
        .copied()
        .unwrap_or_default();
    let noise_transform = authored
        .and_then(|material| material.texture_transforms.get("_NoiseTexture"))
        .copied()
        .unwrap_or_default();
    let surface = Vec4::from_array(vector("_SurfaceColor", [0.071, 0.867, 0.886, 1.0]));
    WaterMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0 - scalar("_WaterSmoothness", 0.9).clamp(0.0, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        },
        extension: WaterMaterialExtension {
            parameters: WaterMaterialUniform {
                surface_color: surface,
                deep_color: Vec4::from_array(vector("_DeepColor", [0.063, 0.361, 0.565, 1.0])),
                foam_color: Vec4::from_array(vector("_FoamColor", [1.0; 4])),
                ice_color: Vec4::from_array(vector("_IceColor", [0.8, 0.93, 1.0, 1.0])),
                wind_speed_noise_alpha: Vec4::new(
                    wind[0],
                    wind[1],
                    scalar("_Speed", 0.02),
                    scalar("_WaterNoiseMultiplyer", 0.03),
                ),
                scale_foam_ice: Vec4::new(
                    scalar("_textureSize", 5.0).max(0.01),
                    scalar("_EdgeFoamScale", 3.71),
                    scalar("_FoamAlpha", 0.4).clamp(0.0, 1.0),
                    scalar("_IceStrength", 0.0).clamp(0.0, 1.0),
                ),
                season_tint: water_color_tint(surface, [0.05, 0.29, 0.47, 0.62]),
                main_scale_offset: Vec4::new(
                    main_transform.scale[0],
                    main_transform.scale[1],
                    main_transform.offset[0],
                    main_transform.offset[1],
                ),
                noise_scale_offset: Vec4::new(
                    noise_transform.scale[0],
                    noise_transform.scale[1],
                    noise_transform.offset[0],
                    noise_transform.offset[1],
                ),
                depth_foam_controls: Vec4::new(
                    scalar("_Distance", 10.0).max(0.01),
                    scalar("_EdgePower", 0.8).max(0.01),
                    scalar("_FoamCuttoff", 7.81).max(0.01),
                    scalar("_FoamDepth", 0.94).max(0.01),
                ),
            },
            main_texture: texture("_MainTexture"),
            noise_texture: texture("_NoiseTexture"),
        },
    }
}

fn building_material(
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> BuildingMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == BUILDING_MATERIAL_PATH);
    let vector = |name: &str, fallback: [f32; 4]| {
        authored
            .and_then(|material| material.custom_vectors.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let texture = authored.and_then(|material| {
        asset_server.and_then(|asset_server| {
            material
                .textures
                .get("_MainTexture")
                .and_then(|id| presentation.textures.get(id))
                .map(|texture| asset_server.load(texture.asset_path.clone()))
        })
    });
    let transform = authored
        .and_then(|material| material.texture_transforms.get("_MainTexture"))
        .copied()
        .unwrap_or_default();
    let roughness_metallic = vector("_RoughnessMetalicValues", [1.0, 0.5, 0.0, 0.0]);
    BuildingMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        },
        extension: BuildingMaterialExtension {
            parameters: BuildingMaterialUniform {
                detail_color: Vec4::from_array(vector(
                    "_DetailColor",
                    [0.521_568_7, 0.521_568_7, 0.521_568_7, 1.0],
                )),
                emissive_color: Vec4::from_array(vector(
                    "_EmissiveColour",
                    [0.521_568_7, 0.521_568_7, 0.521_568_7, 1.0],
                )),
                ambient_occlusion: Vec4::from_array(vector(
                    "_AmbientOcclusion",
                    [0.2, 0.5, 0.0, 0.0],
                )),
                surface_controls: Vec4::new(
                    scalar("_DetailStrength", 0.0),
                    scalar("_GlassEmission", 2.5),
                    scalar("_EmissionStrength", 2.5),
                    roughness_metallic[0],
                ),
                snow_damage: Vec4::new(
                    scalar("_SnowPower", 0.0),
                    scalar("_SnowNoiseLevels", 0.0),
                    scalar("_DestructionValue", 2.0),
                    roughness_metallic[1],
                ),
                main_scale_offset: Vec4::new(
                    transform.scale[0],
                    transform.scale[1],
                    transform.offset[0],
                    transform.offset[1],
                ),
            },
            main_texture: texture,
        },
    }
}

fn cloud_material(
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> CloudMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == CLOUD_MATERIAL_PATH);
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let texture = authored.and_then(|material| {
        asset_server.and_then(|asset_server| {
            material
                .textures
                .get("_Texture0")
                .and_then(|id| presentation.textures.get(id))
                .map(|texture| asset_server.load(texture.asset_path.clone()))
        })
    });
    let transform = authored
        .and_then(|material| material.texture_transforms.get("_Texture0"))
        .copied()
        .unwrap_or_default();
    CloudMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.5,
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            ..default()
        },
        extension: CloudMaterialExtension {
            parameters: CloudMaterialUniform {
                noise_controls: Vec4::new(
                    scalar("_Cloud1", 0.01),
                    scalar("_Cloud2", 0.001),
                    scalar("_ColourSS", 0.0),
                    scalar("_ColourCutoff", 0.13),
                ),
                surface_transform: Vec4::new(
                    scalar("_CloudTint", 1.36),
                    scalar("_CloudSurface", 0.58),
                    transform.scale[0],
                    transform.scale[1],
                ),
            },
            noise_texture: texture,
        },
    }
}

fn tree_material(
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> TreeMaterial {
    let authored = presentation
        .materials
        .values()
        .find(|material| material.source_path == TREE_MATERIAL_PATH);
    let vector = |name: &str, fallback: [f32; 4]| {
        authored
            .and_then(|material| material.custom_vectors.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let scalar = |name: &str, fallback: f32| {
        authored
            .and_then(|material| material.custom_properties.get(name))
            .copied()
            .unwrap_or(fallback)
    };
    let texture = |slot: &str| {
        authored.and_then(|material| {
            asset_server.and_then(|asset_server| {
                material
                    .textures
                    .get(slot)
                    .and_then(|id| presentation.textures.get(id))
                    .map(|texture| asset_server.load(texture.asset_path.clone()))
            })
        })
    };
    let transform = authored
        .and_then(|material| material.texture_transforms.get("_MainTexture"))
        .copied()
        .unwrap_or_default();
    let direction = vector("_windDirection", [1.0, 0.0, 0.0, 0.0]);
    let smoothness = vector("_WindDetailSmoothness", [0.0, 1.0, 0.0, 0.0]);
    TreeMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        },
        extension: TreeMaterialExtension {
            parameters: TreeMaterialUniform {
                wind_direction_smoothness: Vec4::new(
                    direction[0],
                    direction[1],
                    smoothness[0],
                    smoothness[1],
                ),
                wind_controls: Vec4::new(
                    scalar("_Sync", 0.7),
                    scalar("_windStrength", 0.79),
                    scalar("_WindDetailStrength", 0.01),
                    scalar("_textureSize", 1.0),
                ),
                season_controls: tree_season_controls(Season::Spring),
                main_scale_offset: Vec4::new(
                    transform.scale[0],
                    transform.scale[1],
                    transform.offset[0],
                    transform.offset[1],
                ),
            },
            main_texture: texture("_MainTexture"),
            noise_texture: texture("_NoiseTexture"),
        },
    }
}

fn primary_material_texture_entry<'a>(
    material: &'a MaterialDef,
    presentation: &'a PresentationCatalog,
) -> Option<(&'a str, &'a str)> {
    const PRIORITY: [&str; 8] = [
        "_BaseMap",
        "_BaseColorMap",
        "_MainTexture",
        "_MainTex",
        "_Texture0",
        "_characterTexture",
        "_BaseColorRGBOutlineWidthA",
        "_BaseColorRGBSmoothnessA",
    ];
    PRIORITY
        .iter()
        .filter_map(|slot| material.textures.get_key_value(*slot))
        .chain(material.textures.iter())
        .find_map(|(slot, id)| {
            presentation
                .textures
                .get(id)
                .map(|texture| (slot.as_str(), texture.asset_path.as_str()))
        })
}

fn finish_boot(mut next_state: ResMut<NextState<GameState>>) {
    info!("Stream Town boot validation complete");
    if std::env::var_os("STREAM_TOWN_AUTOSTART_CREDITS").is_some() {
        next_state.set(GameState::Credits);
    } else if std::env::var_os("STREAM_TOWN_AUTOSTART").is_some() {
        next_state.set(GameState::WorldLoading);
    } else {
        next_state.set(GameState::MainMenu);
    }
}

fn spawn_main_menu(mut commands: Commands, render: Res<RenderAssets>) {
    spawn_cloud_field(&mut commands, &render, 72.0);
    commands.spawn((
        StateEntity,
        Text::new("STREAM TOWN\n\nENTER  Generate Town\nC  Credits\nESC  Quit"),
        TextFont {
            font_size: FontSize::Px(48.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::srgb(0.86, 0.95, 0.84)),
        Node {
            position_type: PositionType::Absolute,
            top: percent(28.0),
            left: percent(32.0),
            ..default()
        },
    ));
}

fn spawn_cloud_field(commands: &mut Commands, render: &RenderAssets, base_height: f32) {
    for layer in 0_u8..21 {
        let height = base_height + f32::from(layer) * 0.85;
        commands.spawn((
            StateEntity,
            Mesh3d(render.cloud_plane.clone()),
            MeshMaterial3d(render.clouds.clone()),
            Transform::from_xyz(0.0, height, 0.0).with_scale(Vec3::new(900.0, 1.0, 900.0)),
        ));
    }
}

fn main_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::WorldLoading);
    } else if keyboard.just_pressed(KeyCode::KeyC) {
        next_state.set(GameState::Credits);
    } else if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

fn debug_start_day() -> Option<u32> {
    std::env::var_os("STREAM_TOWN_DEBUG_DAY")
        .and_then(|value| value.to_str().and_then(|value| value.parse().ok()))
}

fn debug_weather_override() -> Option<Weather> {
    std::env::var_os("STREAM_TOWN_DEBUG_WEATHER")
        .and_then(|value| value.to_str().and_then(parse_weather))
}

fn debug_building_health() -> Option<i32> {
    std::env::var_os("STREAM_TOWN_DEBUG_BUILDING_HEALTH")
        .and_then(|value| value.to_str().and_then(|value| value.parse().ok()))
        .map(|health: i32| health.clamp(0, BUILDING_MAX_HEALTH))
}

fn debug_initial_agents(configured: u16) -> u16 {
    std::env::var_os("STREAM_TOWN_DEBUG_INITIAL_AGENTS")
        .and_then(|value| value.to_str().and_then(|value| value.parse().ok()))
        .map_or(configured, |agents: u16| agents.clamp(1, configured))
}

fn actor_scene_budget() -> usize {
    actor_detail_budget(
        std::env::var("STREAM_TOWN_ACTOR_SCENE_BUDGET")
            .ok()
            .as_deref(),
    )
}

fn actor_detail_budget(value: Option<&str>) -> usize {
    value
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(DEFAULT_ACTOR_DETAIL_BUDGET)
}

fn parse_weather(value: &str) -> Option<Weather> {
    match value.to_ascii_lowercase().as_str() {
        "clear" => Some(Weather::Clear),
        "rain" => Some(Weather::Rain),
        "fog" => Some(Weather::Fog),
        "snow" => Some(Weather::Snow),
        _ => None,
    }
}

fn initial_actor_identity(index: u16) -> (String, Option<&'static str>) {
    match index {
        0 => ("actor:enemy_0000".to_owned(), Some("role:enemy")),
        1 => ("npc:starting_defender".to_owned(), Some("role:defender")),
        2 => ("npc:starting_logger".to_owned(), Some("role:logger")),
        3 => ("npc:starting_miner".to_owned(), Some("role:miner")),
        4 => ("npc:starting_gatherer".to_owned(), Some("role:gatherer")),
        5 => ("npc:starting_builder".to_owned(), Some("role:builder")),
        _ => (format!("actor:viewer_{index:04}"), None),
    }
}

fn generate_and_spawn_world(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    mut selected: ResMut<SelectedCell>,
    mut cameras: Query<&mut Transform, With<TownCamera>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    selected.0 = None;
    let mut generated = generate_world_with_content(&config.0.world, &content.0);
    let centre = GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    };
    let town_hall_position = GridPos {
        x: (centre.x + 4).min(config.0.world.width - 2),
        z: centre.z,
    };
    let town_hall_id = StableId::new("building:townhall").expect("static ID");
    let town_hall_definition = &content.0.buildings[&town_hall_id];
    let town_hall_placement =
        town_hall_placement_position(&config.0, town_hall_definition.footprint);
    if let Ok(mut camera) = cameras.single_mut() {
        *camera = if std::env::var_os("STREAM_TOWN_SMOKE_RESOURCE_CLOSEUP").is_some() {
            let requested_kind = std::env::var("STREAM_TOWN_SMOKE_RESOURCE_KIND")
                .unwrap_or_else(|_| "resource:wood".to_owned());
            let focus = generated
                .resources
                .iter()
                .filter(|resource| resource.kind.as_str() == requested_kind)
                .min_by_key(|resource| {
                    let dx = i32::from(resource.position.x) - i32::from(centre.x);
                    let dz = i32::from(resource.position.z) - i32::from(centre.z);
                    dx * dx + dz * dz
                })
                .map_or(Vec3::ZERO, |resource| {
                    grid_to_world_on_surface(resource.position, &config.0, &generated)
                });
            Transform::from_xyz(focus.x + 12.0, focus.y + 16.0, focus.z + 12.0)
                .looking_at(focus + Vec3::Y * 4.0, Vec3::Y)
        } else if std::env::var_os("STREAM_TOWN_SMOKE_HEALING_VFX").is_some() {
            let focus = grid_to_world_on_surface(centre, &config.0, &generated);
            Transform::from_xyz(focus.x + 28.0, focus.y + 32.0, focus.z + 28.0)
                .looking_at(focus + Vec3::Y * 5.0, Vec3::Y)
        } else if std::env::var_os("STREAM_TOWN_SMOKE_COMBAT_VFX").is_some() {
            let focus = grid_to_world_on_surface(centre, &config.0, &generated);
            Transform::from_xyz(focus.x + 34.0, focus.y + 38.0, focus.z + 34.0)
                .looking_at(focus + Vec3::Y * 4.0, Vec3::Y)
        } else if std::env::var_os("STREAM_TOWN_SMOKE_BUILDING_VFX").is_some() {
            let focus = grid_to_world_on_surface(centre, &config.0, &generated);
            Transform::from_xyz(focus.x + 40.0, focus.y + 42.0, focus.z + 40.0)
                .looking_at(focus + Vec3::Y * 6.0, Vec3::Y)
        } else if std::env::var_os("STREAM_TOWN_SMOKE_FOLIAGE").is_some() {
            let focus = grid_to_world_on_surface(centre, &config.0, &generated);
            Transform::from_xyz(focus.x + 30.0, focus.y + 35.0, focus.z + 30.0)
                .looking_at(focus + Vec3::Y, Vec3::Y)
        } else if std::env::var_os("STREAM_TOWN_SMOKE_SHORELINE").is_some() {
            let focus = shoreline_focus(&generated, &config.0);
            Transform::from_translation(focus + Vec3::new(45.0, 48.0, 45.0))
                .looking_at(focus, Vec3::Y)
        } else if std::env::var_os("STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP").is_some() {
            let focus = initial_actor_position(&generated, town_hall_position, 1)
                .map_or(Vec3::ZERO, |position| {
                    grid_to_world_on_surface(position, &config.0, &generated)
                });
            Transform::from_xyz(focus.x + 110.0, 130.0, focus.z + 110.0)
                .looking_at(Vec3::new(focus.x + 24.0, focus.y, focus.z + 24.0), Vec3::Y)
        } else if std::env::var_os("STREAM_TOWN_SMOKE_CLOSEUP").is_some() {
            let focus = grid_to_world_on_surface(town_hall_position, &config.0, &generated);
            Transform::from_xyz(focus.x + 66.0, 78.0, focus.z + 66.0).looking_at(focus, Vec3::Y)
        } else {
            Transform::from_xyz(360.0, 420.0, 360.0).looking_at(Vec3::ZERO, Vec3::Y)
        };
    }
    let town_hall_region = building_region(
        town_hall_placement,
        town_hall_definition.footprint,
        &generated,
    )
    .expect("the configured Town Hall footprint fits the generated world");
    generated
        .navigation
        .set_blocked(town_hall_region, true)
        .expect("the configured Town Hall footprint updates navigation");

    let world_size = Vec2::new(
        f32::from(config.0.world.width) * config.0.world.cell_size,
        f32::from(config.0.world.height) * config.0.world.cell_size,
    );
    if let Some(meshes) = meshes.as_deref_mut() {
        let terrain_mesh = generated_terrain_mesh(&generated, &config.0);
        let terrain_collider = Collider::trimesh_from_mesh(&terrain_mesh)
            .expect("generated terrain mesh has indexed triangle geometry");
        commands.spawn((
            WorldEntity,
            TerrainSurface,
            Mesh3d(meshes.add(terrain_mesh)),
            MeshMaterial3d(render.ground.clone()),
            terrain_collider,
            RigidBody::Static,
        ));
    } else {
        commands.spawn((
            WorldEntity,
            TerrainSurface,
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.ground.clone()),
            Transform::from_xyz(0.0, -0.15, 0.0).with_scale(Vec3::new(
                world_size.x,
                0.3,
                world_size.y,
            )),
        ));
    }
    if let Some(meshes) = meshes.as_deref_mut() {
        commands.spawn((
            WorldEntity,
            Mesh3d(meshes.add(generated_water_mesh(&generated, &config.0))),
            MeshMaterial3d(render.water.clone()),
        ));
    } else {
        let water_height = f32::from(config.0.world.water_level_centimetres) * 0.01;
        commands.spawn((
            WorldEntity,
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.water.clone()),
            Transform::from_xyz(0.0, water_height - 0.08, 0.0).with_scale(Vec3::new(
                world_size.x,
                0.12,
                world_size.y,
            )),
        ));
    }

    for resource in &generated.resources {
        let position = grid_to_world_on_surface(resource.position, &config.0, &generated);
        spawn_resource_visual(
            &mut commands,
            &content.0,
            &presentation.0,
            &render,
            asset_server.as_deref(),
            &asset_root.0,
            resource,
            position,
            config.0.world.cell_size,
        );
    }
    if let Some(asset_server) = asset_server.as_deref() {
        for foliage in &generated.foliage {
            spawn_foliage_visual(
                &mut commands,
                &content.0,
                &presentation.0,
                &render,
                asset_server,
                &asset_root.0,
                &generated,
                &config.0,
                foliage,
            );
        }
    }

    let hall = grid_to_world_on_surface(town_hall_position, &config.0, &generated);
    let mut hall_entity = commands.spawn((
        WorldEntity,
        TownHall,
        RuntimeBuilding {
            id: town_hall_id.clone(),
        },
        GridLocation(town_hall_position),
        Transform::from_translation(hall),
    ));
    let town_hall = content
        .0
        .buildings
        .get(&town_hall_id)
        .and_then(|building| content.0.archetypes.get(&building.archetype));
    if let Some(scene) = town_hall.and_then(default_archetype_scene).filter(|scene| {
        asset_server.is_some() && converted_asset_exists(&asset_root.0, &scene.asset_path)
    }) {
        hall_entity.insert((
            WorldAssetRoot(
                asset_server
                    .as_deref()
                    .expect("asset server checked above")
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            BuildingPresentation {
                base_translation: hall,
                base_scale: Vec3::splat(config.0.world.cell_size / 2.0),
                base_height_offset: 0.0,
                applied_stage: u8::MAX,
                applied_level: u16::MAX,
                applied_age: 1,
            },
            BuildingDamageEmitter::default(),
            Transform::from_translation(hall)
                .with_scale(Vec3::splat(config.0.world.cell_size / 2.0)),
        ));
        if let Some(material) = town_hall
            .and_then(|archetype| prefab_material_spec(archetype, scene, &presentation.0, &render))
        {
            hall_entity.insert(material);
        }
    } else {
        let footprint = town_hall.map_or([2, 2], |archetype| archetype.footprint);
        let size = Vec3::new(
            f32::from(footprint[0]) * config.0.world.cell_size,
            config.0.world.cell_size * 1.5,
            f32::from(footprint[1]) * config.0.world.cell_size,
        );
        hall_entity.insert((
            BuildingPresentation {
                base_translation: hall + Vec3::Y * size.y * 0.5,
                base_scale: size,
                base_height_offset: size.y * 0.5,
                applied_stage: u8::MAX,
                applied_level: u16::MAX,
                applied_age: 1,
            },
            BuildingDamageEmitter::default(),
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.building.clone()),
            Transform::from_xyz(hall.x, hall.y + size.y * 0.5, hall.z).with_scale(size),
        ));
    }

    commands.spawn((
        WorldEntity,
        SelectionMarker,
        Mesh3d(render.cube.clone()),
        MeshMaterial3d(render.selection.clone()),
        Transform::from_xyz(0.0, 0.12, 0.0).with_scale(Vec3::new(
            config.0.world.cell_size * 0.9,
            0.2,
            config.0.world.cell_size * 0.9,
        )),
        Visibility::Hidden,
    ));

    let mut spawned = 0_u16;
    let mut simulation = WorldSimulation::new(generated.seed);
    ensure_town_hall_state(&content.0, &config.0, &mut simulation);
    if let Some(health) = debug_building_health()
        && let Some(town_hall) = simulation.buildings.get_mut(&town_hall_id)
    {
        town_hall.health = health;
    }
    if std::env::var_os("STREAM_TOWN_SMOKE_BUILDING_VFX").is_some()
        && let Some(town_hall) = simulation.buildings.get_mut(&town_hall_id)
    {
        town_hall.health = BUILDING_MAX_HEALTH / 3;
    }
    simulation.town_resources = config.0.gameplay.starting_town_resources.clone();
    simulation.unlocked_technology.extend(
        content
            .0
            .technology
            .nodes
            .iter()
            .filter(|(_, technology)| technology.initially_unlocked)
            .map(|(id, _)| id.clone()),
    );
    if let Some(day) = debug_start_day() {
        simulation.elapsed_seconds = f64::from(day) * 120.0;
        simulation.tick(0.0);
    }
    if let Some(weather) = debug_weather_override() {
        simulation.weather = weather;
    }
    if std::env::var_os("STREAM_TOWN_DEBUG_AGE_TWO").is_some()
        && let Some((technology, _)) = content
            .0
            .technology
            .nodes
            .iter()
            .find(|(_, technology)| technology.aged_buildings.contains(&town_hall_id))
    {
        simulation.unlocked_technology.insert(technology.clone());
    }
    let initial_agents = debug_initial_agents(config.0.gameplay.initial_agents);
    let spawn_positions =
        connected_actor_positions(&generated, centre, town_hall_position, initial_agents);
    for position in spawn_positions {
        let x = position.x;
        let z = position.z;
        let target = GridPos {
            x: generated.navigation.width() - 1 - x,
            z: generated.navigation.height() - 1 - z,
        };
        let target = nearest_walkable(&generated, target).unwrap_or(centre);
        let world_position = grid_to_world_on_surface(position, &config.0, &generated);
        let (actor_id, initial_role) = initial_actor_identity(spawned);
        let actor_id = StableId::new(actor_id).expect("generated ID");
        let kind = if spawned == 0 {
            ActorKind::Enemy
        } else {
            ActorKind::Player
        };
        simulation.join_player(actor_id.clone(), position);
        if let Some(role) = initial_role {
            let _ = simulation.assign_role(
                &actor_id,
                StableId::new(role).expect("starting role IDs are valid"),
            );
        }
        let authored_archetype = if spawned == 0 {
            archetype_id_by_source(&content.0, ArchetypeKind::Enemy, "Enemy_Goblin.prefab")
        } else {
            archetype_id_by_source(&content.0, ArchetypeKind::Player, "Player_Character.prefab")
        };
        if let Some(actor) = simulation.actors.get_mut(&actor_id) {
            actor.archetype.clone_from(&authored_archetype);
            if spawned == 1 && std::env::var_os("STREAM_TOWN_DEBUG_CARRY").is_some() {
                actor.role = StableId::new("role:logger").expect("static ID");
                actor
                    .inventory
                    .insert(StableId::new("resource:wood").expect("static ID"), 1);
            }
            if let Some(health) = authored_archetype
                .as_ref()
                .and_then(|id| content.0.archetypes.get(id))
                .and_then(|archetype| archetype.health.as_ref())
            {
                actor.max_health = i32::try_from(health.max_health).unwrap_or(i32::MAX);
                actor.health = actor.max_health;
            }
        }
        let real_archetype = (usize::from(spawned) < actor_scene_budget())
            .then(|| {
                authored_archetype
                    .as_ref()
                    .and_then(|id| content.0.archetypes.get(id))
            })
            .flatten();
        let real_scene = real_archetype
            .and_then(default_archetype_scene)
            .filter(|scene| {
                asset_server.is_some() && converted_asset_exists(&asset_root.0, &scene.asset_path)
            });
        let native_animation = real_archetype
            .zip(real_scene)
            .and_then(|(archetype, scene)| {
                native_animation_request(archetype, scene, &presentation.0)
            });
        let converted_animation = native_animation
            .is_none()
            .then(|| {
                real_archetype
                    .and_then(|archetype| converted_animation_spec(archetype, &presentation.0))
            })
            .flatten();
        let base_scale = if real_scene.is_some() {
            Vec3::splat(config.0.world.cell_size / 2.0)
        } else {
            Vec3::new(
                config.0.world.cell_size * 0.3,
                config.0.world.cell_size * 0.55,
                config.0.world.cell_size * 0.3,
            )
        };
        let visual_height = if real_scene.is_some() {
            world_position.y
        } else {
            world_position.y + base_scale.y * 0.5
        };
        let mut entity = commands.spawn((
            WorldEntity,
            GridLocation(position),
            Agent {
                id: actor_id,
                kind: kind.clone(),
                archetype: authored_archetype
                    .unwrap_or_else(|| StableId::new("archetype:viewer").expect("static ID")),
                goal: AgentGoal::Wander,
                spawn: position,
                origin: position,
                path: Vec::new(),
                path_index: 0,
                target,
                action_cooldown_seconds: 0.0,
                health_regen_accumulator: 0.0,
            },
            AgentAnimation {
                base_scale,
                native: native_animation.is_some() || converted_animation.is_some(),
                ..default()
            },
            Transform::from_xyz(world_position.x, visual_height, world_position.z)
                .with_scale(base_scale),
        ));
        if let Some(scene) = real_scene {
            entity.insert(WorldAssetRoot(
                asset_server
                    .as_deref()
                    .expect("asset server checked above")
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ));
            if let Some(native_animation) = native_animation {
                entity.insert(native_animation);
            } else if let Some(converted_animation) = converted_animation {
                entity.insert(converted_animation);
            }
            if let Some(material) = real_archetype.and_then(|archetype| {
                prefab_material_spec(archetype, scene, &presentation.0, &render)
            }) {
                entity.insert(material);
            }
        } else {
            entity.insert((
                Mesh3d(render.actor_lod.clone()),
                MeshMaterial3d(actor_material(&render, &kind, false)),
            ));
        }
        spawned += 1;
        if spawned >= initial_agents {
            break;
        }
    }

    let recruit_resource = StableId::new("resource:recruit").expect("static ID");
    simulation.town_resources.insert(
        recruit_resource,
        u32::try_from(recruited_actor_ids(&simulation).len()).unwrap_or(u32::MAX),
    );

    if let Some((camp_archetype_id, camp_archetype)) = content
        .0
        .archetypes
        .iter()
        .find(|(_, archetype)| archetype.enemy_spawner.is_some())
        && let Some(position) = find_building_site(
            &generated,
            GridPos {
                x: generated.navigation.width().saturating_sub(8),
                z: generated.navigation.height().saturating_sub(8),
            },
            camp_archetype.footprint,
        )
    {
        let camp_id = StableId::new("enemy_camp:0000").expect("static ID");
        if let Some(region) = building_region(position, camp_archetype.footprint, &generated) {
            let _ = generated.navigation.set_blocked(region, true);
        }
        simulation.enemy_camps.insert(
            camp_id.clone(),
            EnemyCampState {
                id: camp_id.clone(),
                archetype: camp_archetype_id.clone(),
                position,
                health: camp_archetype.health.as_ref().map_or(1_000, |health| {
                    i32::try_from(health.max_health).unwrap_or(i32::MAX)
                }),
                spawn_remaining_seconds: 0.0,
                spawned_enemies: BTreeSet::new(),
            },
        );
        spawn_enemy_camp(
            &mut commands,
            &config.0,
            &generated,
            &presentation.0,
            asset_server.as_deref(),
            &asset_root.0,
            &render,
            &camp_id,
            camp_archetype,
            position,
        );
    }

    commands.spawn((
        WorldEntity,
        Hud,
        Text::new(format!(
            "{} agents | world {}\nF5 Save | F9 Load | F12 Capture | J Inject !join | WASD Pan | Q/E Zoom | Click Select | ESC Menu",
            spawned,
            &generated.deterministic_hash[..12]
        )),
        TextFont {
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
    if std::env::var_os("STREAM_TOWN_SMOKE_HEALING_VFX").is_some() {
        let focus = grid_to_world_on_surface(centre, &config.0, &generated);
        let spacing = config.0.world.cell_size * 3.2;
        spawn_healing_effect(
            &mut commands,
            &render,
            focus - Vec3::X * spacing,
            HealingEffectKind::Channel,
            config.0.world.cell_size,
        );
        spawn_healing_effect(
            &mut commands,
            &render,
            focus,
            HealingEffectKind::Burst,
            config.0.world.cell_size,
        );
        spawn_healing_effect(
            &mut commands,
            &render,
            focus + Vec3::X * spacing,
            HealingEffectKind::Revive,
            config.0.world.cell_size,
        );
    }
    if std::env::var_os("STREAM_TOWN_SMOKE_COMBAT_VFX").is_some() {
        let focus = grid_to_world_on_surface(centre, &config.0, &generated)
            + Vec3::Y * config.0.world.cell_size * 0.35;
        spawn_combat_smoke_field(&mut commands, &render, focus, config.0.world.cell_size);
    }
    if std::env::var_os("STREAM_TOWN_SMOKE_BUILDING_VFX").is_some() {
        let focus = grid_to_world_on_surface(centre, &config.0, &generated);
        spawn_building_smoke_field(&mut commands, &render, focus, config.0.world.cell_size);
    }
    commands.insert_resource(WorldRuntime {
        generated,
        legacy_terrain_mesh: None,
        legacy_migration: None,
    });
    commands.insert_resource(SimulationRuntime(simulation));
    commands.insert_resource(EnvironmentPresentation::default());
    next_state.set(GameState::InGame);
}

fn actor_material(
    render: &RenderAssets,
    kind: &ActorKind,
    moving: bool,
) -> Handle<StandardMaterial> {
    if matches!(kind, ActorKind::Enemy) {
        if moving {
            render.enemy_moving.clone()
        } else {
            render.enemy_idle.clone()
        }
    } else if moving {
        render.player_moving.clone()
    } else {
        render.player_idle.clone()
    }
}

fn default_archetype_scene(
    archetype: &stream_town_domain::ArchetypeDef,
) -> Option<&ArchetypeScene> {
    archetype
        .scenes
        .iter()
        .find(|scene| scene.is_default)
        .or_else(|| archetype.scenes.first())
}

fn resource_visual_archetype<'a>(
    content: &'a ContentCatalog,
    resource_kind: &StableId,
) -> Option<&'a ArchetypeDef> {
    let source_suffix = match resource_kind.as_str() {
        "resource:wood" => "Assets/Prefabs/Dummy Assets/Env_Tree.prefab",
        "resource:ore" => "Assets/Prefabs/Resources/Resource_Ore_Base.prefab",
        "resource:food" => "Assets/Prefabs/Resources/Resource_Bush_Base.prefab",
        _ => return None,
    };
    content
        .archetypes
        .values()
        .find(|archetype| archetype.source_path == source_suffix)
}

fn resource_mesh_index(resource: &stream_town_domain::GeneratedResource) -> usize {
    if resource.kind.as_str() == "resource:food" {
        // Unity's production generation settings list the same bush mesh twice.
        return 0;
    }
    usize::from((resource.position.x ^ resource.position.z) & 1)
}

fn resource_visual_scale(cell_size: f32) -> f32 {
    // Blender's GLBs retain the Unity scene root's centimeter conversion. The
    // Bevy grid uses a 12-unit cell where the Unity resource footprint was
    // authored around four units, so scale both conversions explicitly.
    0.01 * (cell_size / 4.0)
}

#[allow(clippy::too_many_arguments)]
fn spawn_resource_visual(
    commands: &mut Commands,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    resource: &stream_town_domain::GeneratedResource,
    position: Vec3,
    cell_size: f32,
) {
    let visual = resource_visual_archetype(content, &resource.kind)
        .and_then(default_archetype_scene)
        .filter(|scene| converted_asset_exists(asset_root, &scene.asset_path));
    let mesh_index = resource_mesh_index(resource);
    let material = visual.and_then(|scene| {
        presentation
            .model_materials
            .get(&scene.source_model)
            .and_then(|materials| materials.get("MainMaterial"))
            .and_then(|id| render.presentation_materials.get(id))
    });
    if let (Some(asset_server), Some(scene), Some(material)) = (asset_server, visual, material) {
        let mesh = asset_server.load(
            GltfAssetLabel::Primitive {
                mesh: mesh_index,
                primitive: 0,
            }
            .from_asset(scene.asset_path.clone()),
        );
        let mut entity = commands.spawn((
            WorldEntity,
            ResourceNode {
                id: resource.id.clone(),
            },
            ResourceVisual { mesh_index },
            GridLocation(resource.position),
            Mesh3d(mesh),
            Transform::from_translation(position)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
                .with_scale(Vec3::splat(resource_visual_scale(cell_size))),
        ));
        match material {
            ResolvedMaterialHandle::Standard(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
            ResolvedMaterialHandle::Building(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
            ResolvedMaterialHandle::Cloud(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
            ResolvedMaterialHandle::Tree(material) => {
                entity.insert(MeshMaterial3d(material.clone()));
            }
        }
        return;
    }

    let material = match resource.kind.as_str() {
        "resource:wood" => render.wood.clone(),
        "resource:ore" => render.ore.clone(),
        _ => render.food.clone(),
    };
    let scale = cell_size * 0.55;
    commands.spawn((
        WorldEntity,
        ResourceNode {
            id: resource.id.clone(),
        },
        GridLocation(resource.position),
        Mesh3d(render.cube.clone()),
        MeshMaterial3d(material),
        Transform::from_xyz(position.x, position.y + scale * 0.5, position.z)
            .with_scale(Vec3::splat(scale)),
    ));
}

#[allow(clippy::too_many_arguments)]
fn spawn_foliage_visual(
    commands: &mut Commands,
    content: &ContentCatalog,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    asset_server: &AssetServer,
    asset_root: &Path,
    world: &GeneratedWorld,
    config: &GameConfig,
    foliage: &GeneratedFoliage,
) {
    let Some(layer) = content
        .foliage
        .iter()
        .find(|layer| layer.id == foliage.layer)
    else {
        return;
    };
    let Some(variant) = layer.variants.get(usize::from(foliage.variant)) else {
        return;
    };
    if !converted_asset_exists(asset_root, &variant.asset_path) {
        return;
    }
    let mesh = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset(variant.asset_path.clone()),
    );
    let centre = grid_to_world_on_surface(foliage.position, config, world);
    let offset = Vec3::new(
        f32::from(foliage.offset_milli_cells[0]) * config.world.cell_size / 1_000.0,
        0.0,
        f32::from(foliage.offset_milli_cells[1]) * config.world.cell_size / 1_000.0,
    );
    // The Blender conversion preserves FBX centimetres just like the resource
    // GLBs, while authored Unity scale is expressed in metres.
    let scale = Vec3::from_array(variant.base_scale)
        * resource_visual_scale(config.world.cell_size)
        * (f32::from(foliage.scale_milli) / 1_000.0);
    let material = presentation
        .model_materials
        .get(&variant.source_model)
        .and_then(|materials| materials.values().next())
        .and_then(|id| render.presentation_materials.get(id));
    let mut entity = commands.spawn((
        WorldEntity,
        FoliageVisual,
        GridLocation(foliage.position),
        Mesh3d(mesh),
        Transform::from_translation(centre + offset)
            .with_rotation(
                Quat::from_rotation_y(f32::from(foliage.yaw_milliradians) / 1_000.0)
                    * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            )
            .with_scale(scale),
        Visibility::Inherited,
        bevy::camera::visibility::VisibilityRange::abrupt(0.0, FOLIAGE_VISIBILITY_RANGE),
        bevy::light::NotShadowCaster,
    ));
    match material {
        Some(ResolvedMaterialHandle::Standard(material)) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
        Some(ResolvedMaterialHandle::Building(material)) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
        Some(ResolvedMaterialHandle::Tree(material)) => {
            entity.insert(MeshMaterial3d(material.clone()));
        }
        Some(ResolvedMaterialHandle::Cloud(_)) | None => {
            entity.insert(MeshMaterial3d(render.food.clone()));
        }
    }
}

fn archetype_scene_for_age(archetype: &ArchetypeDef, age: u8) -> Option<&ArchetypeScene> {
    archetype
        .scenes
        .iter()
        .find(|scene| scene.age == Some(age))
        .or_else(|| default_archetype_scene(archetype))
}

fn archetype_by_source<'a>(
    content: &'a ContentCatalog,
    kind: ArchetypeKind,
    source_suffix: &str,
) -> Option<&'a ArchetypeDef> {
    content
        .archetypes
        .values()
        .find(|archetype| archetype.kind == kind && archetype.source_path.ends_with(source_suffix))
}

fn archetype_id_by_source(
    content: &ContentCatalog,
    kind: ArchetypeKind,
    source_suffix: &str,
) -> Option<StableId> {
    content.archetypes.iter().find_map(|(id, archetype)| {
        (archetype.kind == kind && archetype.source_path.ends_with(source_suffix))
            .then(|| id.clone())
    })
}

fn native_animation_request(
    archetype: &ArchetypeDef,
    scene: &ArchetypeScene,
    presentation: &PresentationCatalog,
) -> Option<NativeAnimationRequest> {
    let binding = presentation.prefab_bindings.get(&archetype.source_guid)?;
    let animation_index = binding.gltf_animation_index?;
    if binding.animated_scene.as_deref() != Some(scene.asset_path.as_str()) {
        return None;
    }
    Some(NativeAnimationRequest {
        asset_path: scene.asset_path.clone(),
        animation_index,
    })
}

fn converted_animation_spec(
    archetype: &ArchetypeDef,
    presentation: &PresentationCatalog,
) -> Option<ConvertedAnimationSpec> {
    let binding = presentation.prefab_bindings.get(&archetype.source_guid)?;
    let controller = presentation.controllers.get(&binding.controller)?;
    let rig_scene = binding.rig_scene.clone()?;
    let (state, locomotion) = controller
        .states
        .iter()
        .find(|(_, state)| state.display_name.eq_ignore_ascii_case("locomotion"))?;
    let converted_motions = locomotion
        .motions
        .iter()
        .filter(|motion| {
            presentation
                .clips
                .get(&motion.clip)
                .is_some_and(|clip| !clip.transform_tracks.is_empty())
        })
        .count();
    (converted_motions == locomotion.motions.len() && converted_motions > 0).then(|| {
        ConvertedAnimationSpec {
            controller: binding.controller.clone(),
            state: state.clone(),
            rig_scene,
        }
    })
}

fn prefab_material_spec(
    archetype: &ArchetypeDef,
    scene: &ArchetypeScene,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
) -> Option<MaterialOverrideSpec> {
    let fallback = presentation
        .prefab_materials
        .get(&archetype.source_guid)
        .into_iter()
        .flatten()
        .find_map(|id| render.presentation_materials.get(id))
        .cloned();
    let model_materials: BTreeMap<_, _> = presentation
        .model_materials
        .get(&scene.source_model)
        .into_iter()
        .flat_map(|materials| materials.iter())
        .filter_map(|(name, id)| {
            render
                .presentation_materials
                .get(id)
                .cloned()
                .map(|material| (name.clone(), material))
        })
        .collect();
    let renderer_materials: Vec<_> = presentation
        .prefab_renderer_materials
        .get(&archetype.source_guid)
        .into_iter()
        .flatten()
        .filter_map(|binding| {
            let materials: BTreeMap<_, _> = binding
                .materials
                .iter()
                .filter_map(|(name, id)| {
                    render
                        .presentation_materials
                        .get(id)
                        .cloned()
                        .map(|material| (name.clone(), material))
                })
                .collect();
            (!materials.is_empty()).then(|| ResolvedRendererMaterialBinding {
                target_path: binding.target_path.clone(),
                materials,
            })
        })
        .collect();
    (fallback.is_some() || !model_materials.is_empty() || !renderer_materials.is_empty()).then_some(
        MaterialOverrideSpec {
            fallback,
            model_materials,
            renderer_materials,
        },
    )
}

fn converted_asset_exists(asset_root: &Path, asset_path: &str) -> bool {
    asset_root.join(asset_path).is_file()
}

fn nearest_walkable(world: &GeneratedWorld, desired: GridPos) -> Option<GridPos> {
    let desired = GridPos {
        x: desired.x.min(world.navigation.width() - 1),
        z: desired.z.min(world.navigation.height() - 1),
    };
    if world.navigation.is_walkable(desired) {
        return Some(desired);
    }
    let limit = world.navigation.width().max(world.navigation.height());
    for radius in 1..limit {
        for z in desired.z.saturating_sub(radius)
            ..=desired
                .z
                .saturating_add(radius)
                .min(world.navigation.height() - 1)
        {
            for x in desired.x.saturating_sub(radius)
                ..=desired
                    .x
                    .saturating_add(radius)
                    .min(world.navigation.width() - 1)
            {
                let candidate = GridPos { x, z };
                if world.navigation.is_walkable(candidate) {
                    return Some(candidate);
                }
            }
        }
    }
    None
}

fn initial_actor_position(
    world: &GeneratedWorld,
    excluded: GridPos,
    actor_index: usize,
) -> Option<GridPos> {
    let mut found = 0;
    for z in 0..world.navigation.height() {
        for x in 0..world.navigation.width() {
            let position = GridPos { x, z };
            if position == excluded || !world.navigation.is_walkable(position) {
                continue;
            }
            if found == actor_index {
                return Some(position);
            }
            found += 1;
        }
    }
    None
}

fn connected_actor_positions(
    world: &GeneratedWorld,
    start: GridPos,
    excluded: GridPos,
    count: u16,
) -> Vec<GridPos> {
    let mut positions = Vec::with_capacity(usize::from(count));
    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::from([start]);
    while let Some(position) = queue.pop_front() {
        if !visited.insert(position) || !world.navigation.is_walkable(position) {
            continue;
        }
        if position != excluded {
            positions.push(position);
            if positions.len() >= usize::from(count) {
                break;
            }
        }
        let neighbors = [
            position
                .x
                .checked_add(1)
                .filter(|x| *x < world.navigation.width())
                .map(|x| GridPos { x, z: position.z }),
            position
                .x
                .checked_sub(1)
                .map(|x| GridPos { x, z: position.z }),
            position
                .z
                .checked_add(1)
                .filter(|z| *z < world.navigation.height())
                .map(|z| GridPos { x: position.x, z }),
            position
                .z
                .checked_sub(1)
                .map(|z| GridPos { x: position.x, z }),
        ];
        queue.extend(neighbors.into_iter().flatten());
    }
    positions
}

fn resource_for_role(content: &ContentCatalog, role: &StableId) -> Option<StableId> {
    content.roles.get(role)?.resource.clone()
}

fn actor_resource_storage_has_room(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
) -> bool {
    let Some(resource) = resource_for_role(content, &actor.role) else {
        return true;
    };
    simulation
        .town_resources
        .get(&resource)
        .copied()
        .unwrap_or_default()
        < resource_storage_capacity(config, content, simulation, &resource)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EffectiveRoleStats {
    level: u16,
    experience: u32,
    required_experience: u32,
    experience_multiplier_per_thousand: u32,
    action_amount: u32,
    action_milliseconds: u32,
    action_range_milli_cells: u32,
    max_health: u32,
    health_regen_milli_per_second: i64,
    damage_reduction_percent: u32,
    movement_speed_milli_cells_per_second: u32,
    carry_capacity: u32,
}

fn leveled_whole_stat(base: u32, per_level_milli: u32, level: u16) -> u32 {
    let increase =
        u64::from(per_level_milli).saturating_mul(u64::from(level.saturating_sub(1))) / 1_000;
    base.saturating_add(u32::try_from(increase).unwrap_or(u32::MAX))
}

fn role_progress(actor: &ActorState) -> stream_town_domain::RoleProgress {
    actor
        .role_progression
        .get(&actor.role)
        .copied()
        .unwrap_or_default()
}

fn effective_role_stats(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
) -> Option<EffectiveRoleStats> {
    let definition = content.roles.get(&actor.role)?;
    let progress = role_progress(actor);
    let levels = u32::from(progress.level.saturating_sub(1));
    let stat = |name: &str| {
        technology_stat_boost_percent(
            content,
            simulation,
            &actor.role,
            &StableId::new(format!("stat:{name}")).expect("static stat IDs are valid"),
        )
    };
    let base_action_milliseconds = definition
        .base_action_milliseconds
        .saturating_sub(
            definition
                .action_milliseconds_reduction_per_level
                .saturating_mul(levels),
        )
        .max(100);
    let base_range = definition.base_action_range_milli_cells.saturating_add(
        definition
            .action_range_milli_cells_per_level
            .saturating_mul(levels),
    );
    let base_movement = definition
        .base_movement_speed_milli_cells_per_second
        .saturating_add(
            definition
                .movement_speed_milli_cells_per_second_per_level
                .saturating_mul(levels),
        );
    let base_defense = leveled_whole_stat(
        u32::try_from(definition.base_damage_reduction_percent.max(0)).unwrap_or_default(),
        definition.damage_reduction_milli_percent_per_level,
        progress.level,
    );
    let base_regen = i64::from(definition.base_health_regen_per_second)
        .saturating_mul(1_000)
        .saturating_add(
            i64::from(definition.health_regen_milli_per_second_per_level)
                .saturating_mul(i64::from(levels)),
        );

    Some(EffectiveRoleStats {
        level: progress.level,
        experience: progress.experience,
        required_experience: stream_town_domain::required_role_experience(progress.level),
        experience_multiplier_per_thousand: definition.experience_multiplier_per_thousand,
        action_amount: percentage_adjusted(
            leveled_whole_stat(
                definition.base_action_amount,
                definition.action_amount_per_level_milli,
                progress.level,
            ),
            stat("action_amount"),
        ),
        action_milliseconds: percentage_reduced(base_action_milliseconds, stat("action_speed"))
            .max(100),
        action_range_milli_cells: percentage_adjusted(base_range, stat("action_range")),
        max_health: percentage_adjusted(
            leveled_whole_stat(
                definition.base_health,
                definition.health_per_level_milli,
                progress.level,
            ),
            stat("health"),
        )
        .max(1),
        health_regen_milli_per_second: percentage_adjusted_i64(base_regen, stat("health_regen")),
        damage_reduction_percent: percentage_adjusted(base_defense, stat("defense")),
        movement_speed_milli_cells_per_second: percentage_adjusted(
            base_movement,
            stat("movement_speed"),
        ),
        carry_capacity: percentage_adjusted(
            leveled_whole_stat(
                definition.base_carry_capacity,
                definition.carry_capacity_per_level_milli,
                progress.level,
            ),
            stat("resource_carry"),
        ),
    })
}

fn role_action_range_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
) -> u16 {
    if let Some(enemy) =
        actor_archetype(content, actor).and_then(|archetype| archetype.enemy.as_ref())
    {
        return u16::try_from(enemy.action_range_milli_cells.div_ceil(1_000)).unwrap_or(u16::MAX);
    }
    effective_role_stats(content, simulation, actor).map_or(1, |stats| {
        u16::try_from(stats.action_range_milli_cells.div_ceil(1_000)).unwrap_or(u16::MAX)
    })
}

fn is_combat_role(role: &StableId) -> bool {
    matches!(
        role.as_str(),
        "role:defender"
            | "role:necromancer"
            | "role:paladin"
            | "role:ranger"
            | "role:ruler"
            | "role:soldier"
            | "role:wizard"
    )
}

fn is_healer_role(role: &StableId) -> bool {
    role.as_str() == "role:priest"
}

fn enemy_targets_kind(content: &ContentCatalog, actor: &ActorState, target: &str) -> bool {
    actor_archetype(content, actor)
        .and_then(|archetype| archetype.enemy.as_ref())
        .is_some_and(|enemy| {
            enemy.targets_all || enemy.target_kinds.iter().any(|id| id.as_str() == target)
        })
}

fn enemy_targets_buildings(content: &ContentCatalog, actor: &ActorState) -> bool {
    [
        "target:building",
        "target:damaged_building",
        "target:construction",
    ]
    .into_iter()
    .any(|target| enemy_targets_kind(content, actor, target))
}

fn is_ranged_role(role: &StableId) -> bool {
    matches!(
        role.as_str(),
        "role:necromancer" | "role:ranger" | "role:wizard"
    )
}

fn actor_combat_visual(role: &StableId) -> CombatVisualKind {
    match role.as_str() {
        "role:wizard" => CombatVisualKind::Fireball,
        "role:necromancer" => CombatVisualKind::Necrotic,
        "role:ranger" => CombatVisualKind::Arrow,
        _ => CombatVisualKind::Physical,
    }
}

fn actor_archetype<'a>(
    content: &'a ContentCatalog,
    actor: &ActorState,
) -> Option<&'a ArchetypeDef> {
    if let Some(archetype) = actor
        .archetype
        .as_ref()
        .and_then(|archetype| content.archetypes.get(archetype))
    {
        return Some(archetype);
    }
    let (kind, source) = if actor.role.as_str() == "role:enemy" {
        (ArchetypeKind::Enemy, "Enemy_Goblin.prefab")
    } else {
        (ArchetypeKind::Player, "Player_Character.prefab")
    };
    archetype_by_source(content, kind, source)
}

fn authored_respawn_milliseconds(content: &ContentCatalog, actor: &ActorState) -> Option<u32> {
    actor_archetype(content, actor)
        .and_then(|archetype| archetype.health.as_ref())
        .and_then(|health| health.revive_milliseconds)
}

fn town_hall_grid_position(config: &GameConfig) -> GridPos {
    GridPos {
        x: (config.world.width / 2 + 4).min(config.world.width - 2),
        z: config.world.height / 2,
    }
}

fn town_hall_placement_position(config: &GameConfig, footprint: [u16; 2]) -> GridPos {
    let centre = town_hall_grid_position(config);
    GridPos {
        x: centre.x.saturating_sub(footprint[0] / 2),
        z: centre.z.saturating_sub(footprint[1] / 2),
    }
}

fn ensure_town_hall_state(
    content: &ContentCatalog,
    config: &GameConfig,
    simulation: &mut WorldSimulation,
) {
    let id = StableId::new("building:townhall").expect("static ID");
    if simulation.buildings.contains_key(&id) {
        return;
    }
    let definition = &content.buildings[&id];
    simulation.buildings.insert(
        id.clone(),
        BuildingState {
            id,
            archetype: definition.archetype.clone(),
            position: town_hall_placement_position(config, definition.footprint),
            rotation_quarter_turns: 0,
            level: 1,
            health: BUILDING_MAX_HEALTH,
            complete: true,
        },
    );
}

#[derive(Clone, Copy)]
struct StationCandidate<'a> {
    id: &'a StableId,
    position: GridPos,
    definition: &'a StationDef,
}

fn ensure_actor_station(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    config: &GameConfig,
    actor_id: &StableId,
) {
    let replacement = simulation.actors.get(actor_id).and_then(|actor| {
        let valid = assigned_station(content, simulation, config, actor).is_some();
        (!valid).then(|| best_station_id(content, simulation, config, &actor.role, actor.position))
    });
    if let Some(station) = replacement
        && let Some(actor) = simulation.actors.get_mut(actor_id)
    {
        actor.station = station;
    }
}

fn station_matches_role(station: &StationDef, role: &stream_town_domain::RoleDef) -> bool {
    station.accepts_all_roles
        || role
            .station_kinds
            .iter()
            .any(|kind| station.accepted_role_kinds.contains(kind))
}

fn station_supports_role_targets(station: &StationDef, role: &stream_town_domain::RoleDef) -> bool {
    station.targets_all
        || role.targets_all
        || role
            .target_kinds
            .iter()
            .any(|kind| station.target_kinds.contains(kind))
}

fn station_candidate<'a>(
    content: &'a ContentCatalog,
    simulation: &'a WorldSimulation,
    config: &GameConfig,
    station_id: &'a StableId,
) -> Option<StationCandidate<'a>> {
    if station_id.as_str() == "building:townhall" {
        let building = content.buildings.get(station_id)?;
        return Some(StationCandidate {
            id: station_id,
            position: town_hall_grid_position(config),
            definition: building.station.as_ref()?,
        });
    }
    let state = simulation
        .buildings
        .get(station_id)
        .filter(|state| state.complete)?;
    let building = building_def_for_archetype(content, &state.archetype)?;
    let footprint = rotated_footprint(building.footprint, state.rotation_quarter_turns);
    Some(StationCandidate {
        id: station_id,
        position: GridPos {
            x: state.position.x.saturating_add(footprint[0] / 2),
            z: state.position.z.saturating_add(footprint[1] / 2),
        },
        definition: building.station.as_ref()?,
    })
}

fn best_station_id(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    config: &GameConfig,
    role_id: &StableId,
    from: GridPos,
) -> Option<StableId> {
    let role = content.roles.get(role_id)?;
    if role.station_kinds.is_empty() {
        return None;
    }
    let town_hall_id = StableId::new("building:townhall").expect("static building ID is valid");
    let town_hall = content.buildings.get(&town_hall_id).and_then(|building| {
        building
            .station
            .as_ref()
            .map(|definition| StationCandidate {
                id: &town_hall_id,
                position: town_hall_grid_position(config),
                definition,
            })
    });
    town_hall
        .into_iter()
        .chain(simulation.buildings.values().filter_map(|state| {
            if state.id.as_str() == "building:townhall" {
                return None;
            }
            if !state.complete {
                return None;
            }
            let building = building_def_for_archetype(content, &state.archetype)?;
            let footprint = rotated_footprint(building.footprint, state.rotation_quarter_turns);
            Some(StationCandidate {
                id: &state.id,
                position: GridPos {
                    x: state.position.x.saturating_add(footprint[0] / 2),
                    z: state.position.z.saturating_add(footprint[1] / 2),
                },
                definition: building.station.as_ref()?,
            })
        }))
        .filter(|station| {
            station_matches_role(station.definition, role)
                && station_supports_role_targets(station.definition, role)
        })
        .min_by_key(|station| {
            (
                station.position.x.abs_diff(from.x) + station.position.z.abs_diff(from.z),
                station.id.clone(),
            )
        })
        .map(|station| station.id.clone())
}

fn assigned_station<'a>(
    content: &'a ContentCatalog,
    simulation: &'a WorldSimulation,
    config: &GameConfig,
    actor: &'a ActorState,
) -> Option<StationCandidate<'a>> {
    actor
        .station
        .as_ref()
        .and_then(|id| station_candidate(content, simulation, config, id))
        .filter(|station| {
            content.roles.get(&actor.role).is_some_and(|role| {
                station_matches_role(station.definition, role)
                    && station_supports_role_targets(station.definition, role)
            })
        })
}

fn station_search_range_cells(station: StationCandidate<'_>) -> u16 {
    u16::try_from(station.definition.search_range_milli_cells.div_ceil(1_000)).unwrap_or(u16::MAX)
}

fn within_station_range(position: GridPos, station: StationCandidate<'_>) -> bool {
    position.x.abs_diff(station.position.x) + position.z.abs_diff(station.position.z)
        <= station_search_range_cells(station)
}

fn building_def_for_archetype<'a>(
    content: &'a ContentCatalog,
    archetype: &StableId,
) -> Option<&'a BuildingDef> {
    content
        .buildings
        .values()
        .find(|building| building.archetype == *archetype)
}

fn building_visual_grid(content: &ContentCatalog, building: &BuildingState) -> GridPos {
    let footprint = building_def_for_archetype(content, &building.archetype)
        .map_or([1, 1], |definition| {
            rotated_footprint(definition.footprint, building.rotation_quarter_turns)
        });
    GridPos {
        x: building.position.x.saturating_add(footprint[0] / 2),
        z: building.position.z.saturating_add(footprint[1] / 2),
    }
}

fn building_approach(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
    from: GridPos,
) -> Option<GridPos> {
    let region = building_region(position, footprint, world)?;
    let min_x = region.min.x.saturating_sub(1);
    let min_z = region.min.z.saturating_sub(1);
    let max_x = region
        .max
        .x
        .saturating_add(1)
        .min(world.navigation.width() - 1);
    let max_z = region
        .max
        .z
        .saturating_add(1)
        .min(world.navigation.height() - 1);
    let mut approaches = Vec::new();
    for z in min_z..=max_z {
        for x in min_x..=max_x {
            let candidate = GridPos { x, z };
            let outside =
                x < region.min.x || x > region.max.x || z < region.min.z || z > region.max.z;
            if outside && world.navigation.is_walkable(candidate) {
                approaches.push(candidate);
            }
        }
    }
    approaches.sort_by_key(|candidate| {
        (
            candidate.x.abs_diff(from.x) + candidate.z.abs_diff(from.z),
            candidate.z,
            candidate.x,
        )
    });
    approaches.into_iter().next()
}

fn next_agent_goal(
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    content: &ContentCatalog,
    actor_id: &StableId,
    current: GridPos,
) -> (AgentGoal, GridPos) {
    let Some(actor) = simulation.actors.get(actor_id) else {
        return (AgentGoal::Wander, mirrored_target(world, current));
    };
    if !actor.alive {
        return (AgentGoal::Wander, current);
    }
    let station = assigned_station(content, simulation, config, actor);
    if let Some(preferred) = actor.preferred_target.as_ref() {
        if let Some(target) = simulation
            .actors
            .get(preferred)
            .filter(|target| target.alive && target.role.as_str() == "role:enemy")
            .filter(|_| is_combat_role(&actor.role))
            .filter(|target| {
                station.is_none_or(|station| within_station_range(target.position, station))
            })
        {
            let distance =
                target.position.x.abs_diff(current.x) + target.position.z.abs_diff(current.z);
            return (
                AgentGoal::Attack(target.id.clone()),
                if distance <= role_action_range_cells(content, simulation, actor) {
                    current
                } else {
                    target.position
                },
            );
        }
        if let Some(target) = simulation
            .actors
            .get(preferred)
            .filter(|target| {
                target.alive
                    && target.role.as_str() != "role:enemy"
                    && target.health < target.max_health
            })
            .filter(|_| is_healer_role(&actor.role))
            .filter(|target| {
                station.is_none_or(|station| within_station_range(target.position, station))
            })
        {
            let distance =
                target.position.x.abs_diff(current.x) + target.position.z.abs_diff(current.z);
            return (
                AgentGoal::Heal(target.id.clone()),
                if distance <= role_action_range_cells(content, simulation, actor) {
                    current
                } else {
                    target.position
                },
            );
        }
        if let Some(resource) = world
            .resources
            .iter()
            .find(|resource| resource.id == *preferred && resource.amount > 0)
            .filter(|resource| {
                resource_for_role(content, &actor.role).as_ref() == Some(&resource.kind)
            })
            .filter(|resource| {
                station.is_none_or(|station| within_station_range(resource.position, station))
            })
        {
            return (AgentGoal::Gather(resource.id.clone()), resource.position);
        }
        if let Some(building) = simulation
            .buildings
            .get(preferred)
            .filter(|building| {
                actor.role.as_str() == "role:builder"
                    && (!building.complete || building.health < BUILDING_MAX_HEALTH)
            })
            .filter(|building| {
                station.is_none_or(|station| within_station_range(building.position, station))
            })
            && let Some(definition) = building_def_for_archetype(content, &building.archetype)
            && let Some(approach) = building_approach(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                current,
            )
        {
            return (AgentGoal::Construct(building.id.clone()), approach);
        }
    }
    if is_healer_role(&actor.role) {
        let mut candidates: Vec<_> = simulation
            .actors
            .values()
            .filter(|target| {
                target.id != actor.id
                    && target.alive
                    && target.role.as_str() != "role:enemy"
                    && target.health < target.max_health
            })
            .filter(|target| {
                station.is_none_or(|station| within_station_range(target.position, station))
            })
            .collect();
        if let Some(station) = station {
            candidates.sort_by_key(|target| {
                (
                    target.position.x.abs_diff(station.position.x)
                        + target.position.z.abs_diff(station.position.z),
                    target.id.clone(),
                )
            });
            candidates.truncate(usize::from(station.definition.max_targets));
        }
        if let Some(target) = candidates.into_iter().min_by_key(|target| {
            (
                target.position.x.abs_diff(current.x) + target.position.z.abs_diff(current.z),
                target.id.clone(),
            )
        }) {
            let distance =
                target.position.x.abs_diff(current.x) + target.position.z.abs_diff(current.z);
            return (
                AgentGoal::Heal(target.id.clone()),
                if distance <= role_action_range_cells(content, simulation, actor) {
                    current
                } else {
                    target.position
                },
            );
        }
    }
    if actor.role.as_str() == "role:enemy" {
        let player_target = enemy_targets_kind(content, actor, "target:player")
            .then(|| {
                simulation
                    .actors
                    .values()
                    .filter(|target| target.alive && target.role.as_str() != "role:enemy")
                    .map(|target| {
                        (
                            target.position.x.abs_diff(current.x)
                                + target.position.z.abs_diff(current.z),
                            target.id.clone(),
                            target.position,
                        )
                    })
                    .min_by_key(|(distance, id, _)| (*distance, id.clone()))
            })
            .flatten();
        let building_target = enemy_targets_buildings(content, actor)
            .then(|| {
                simulation
                    .buildings
                    .values()
                    .filter(|building| building.health > 0)
                    .filter_map(|building| {
                        let definition = building_def_for_archetype(content, &building.archetype)?;
                        let approach = building_approach(
                            world,
                            building.position,
                            rotated_footprint(
                                definition.footprint,
                                building.rotation_quarter_turns,
                            ),
                            current,
                        )?;
                        Some((
                            approach.x.abs_diff(current.x) + approach.z.abs_diff(current.z),
                            building.id.clone(),
                            approach,
                        ))
                    })
                    .min_by_key(|(distance, id, _)| (*distance, id.clone()))
            })
            .flatten();
        let action_range = role_action_range_cells(content, simulation, actor);
        match (player_target, building_target) {
            (Some((player_distance, player, position)), Some((building_distance, _, _)))
                if player_distance <= building_distance =>
            {
                return (
                    AgentGoal::Attack(player),
                    if player_distance <= action_range {
                        current
                    } else {
                        position
                    },
                );
            }
            (_, Some((distance, building, approach))) => {
                return (
                    AgentGoal::AttackBuilding(building),
                    if distance <= action_range {
                        current
                    } else {
                        approach
                    },
                );
            }
            (Some((distance, player, position)), None) => {
                return (
                    AgentGoal::Attack(player),
                    if distance <= action_range {
                        current
                    } else {
                        position
                    },
                );
            }
            (None, None) => {}
        }
    }
    let combat_target = if is_combat_role(&actor.role) {
        let mut candidates: Vec<_> = simulation
            .actors
            .values()
            .filter(|target| target.alive && target.role.as_str() == "role:enemy")
            .filter(|target| {
                station.is_none_or(|station| within_station_range(target.position, station))
            })
            .collect();
        if let Some(station) = station {
            candidates.sort_by_key(|target| {
                (
                    target.position.x.abs_diff(station.position.x)
                        + target.position.z.abs_diff(station.position.z),
                    target.id.clone(),
                )
            });
            candidates.truncate(usize::from(station.definition.max_targets));
        }
        candidates.into_iter().min_by_key(|target| {
            (
                target.position.x.abs_diff(current.x) + target.position.z.abs_diff(current.z),
                target.id.clone(),
            )
        })
    } else {
        None
    };
    if let Some(target) = combat_target {
        let distance =
            target.position.x.abs_diff(current.x) + target.position.z.abs_diff(current.z);
        let destination = if distance <= role_action_range_cells(content, simulation, actor) {
            current
        } else {
            target.position
        };
        return (AgentGoal::Attack(target.id.clone()), destination);
    }
    if actor.role.as_str() == "role:builder" {
        let mut candidates: Vec<_> = simulation
            .buildings
            .values()
            .filter(|building| !building.complete || building.health < BUILDING_MAX_HEALTH)
            .filter(|building| {
                station.is_none_or(|station| within_station_range(building.position, station))
            })
            .filter_map(|building| {
                let definition = building_def_for_archetype(content, &building.archetype)?;
                let approach = building_approach(
                    world,
                    building.position,
                    rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                    current,
                )?;
                Some((
                    station.map_or(0, |station| {
                        building.position.x.abs_diff(station.position.x)
                            + building.position.z.abs_diff(station.position.z)
                    }),
                    approach.x.abs_diff(current.x) + approach.z.abs_diff(current.z),
                    building.id.clone(),
                    approach,
                ))
            })
            .collect();
        if let Some(station) = station {
            candidates.sort_by_key(|(station_distance, _, id, _)| (*station_distance, id.clone()));
            candidates.truncate(usize::from(station.definition.max_targets));
        }
        let construction = candidates
            .into_iter()
            .min_by_key(|(_, distance, id, _)| (*distance, id.clone()));
        if let Some((_, _, building, approach)) = construction {
            return (AgentGoal::Construct(building), approach);
        }
    }
    if !actor_resource_storage_has_room(config, content, simulation, actor) {
        return (AgentGoal::Wander, mirrored_target(world, current));
    }
    let carried = actor
        .inventory
        .values()
        .copied()
        .fold(0_u32, u32::saturating_add);
    let carry_capacity = effective_role_stats(content, simulation, actor)
        .map(|stats| stats.carry_capacity)
        .filter(|capacity| *capacity > 0)
        .unwrap_or(25);
    if carried >= carry_capacity {
        let destination = station.map_or_else(
            || town_hall_grid_position(config),
            |station| station.position,
        );
        let target = nearest_walkable(world, destination).unwrap_or(current);
        return (AgentGoal::Deposit, target);
    }
    let Some(resource_kind) = resource_for_role(content, &actor.role) else {
        return (AgentGoal::Wander, mirrored_target(world, current));
    };
    let mut resources: Vec<_> = world
        .resources
        .iter()
        .filter(|resource| resource.kind == resource_kind && resource.amount > 0)
        .filter(|resource| {
            station.is_none_or(|station| within_station_range(resource.position, station))
        })
        .collect();
    if let Some(station) = station {
        resources.sort_by_key(|resource| {
            (
                resource.position.x.abs_diff(station.position.x)
                    + resource.position.z.abs_diff(station.position.z),
                resource.position.z,
                resource.position.x,
            )
        });
        resources.truncate(usize::from(station.definition.max_targets));
    }
    let resource = resources.into_iter().min_by_key(|resource| {
        (
            resource.position.x.abs_diff(current.x) + resource.position.z.abs_diff(current.z),
            resource.position.z,
            resource.position.x,
        )
    });
    resource.map_or_else(
        || (AgentGoal::Wander, mirrored_target(world, current)),
        |resource| (AgentGoal::Gather(resource.id.clone()), resource.position),
    )
}

fn complete_agent_goal(
    simulation: &mut WorldSimulation,
    world: &mut GeneratedWorld,
    config: &GameConfig,
    content: &ContentCatalog,
    actor_id: &StableId,
    goal: &AgentGoal,
    current: GridPos,
) -> Option<ActionPresentation> {
    if !simulation.actors.contains_key(actor_id) {
        return None;
    }
    let stats = simulation
        .actors
        .get(actor_id)
        .and_then(|actor| effective_role_stats(content, simulation, actor));
    let action_amount = simulation
        .actors
        .get(actor_id)
        .and_then(|actor| actor_archetype(content, actor))
        .and_then(|archetype| archetype.enemy.as_ref())
        .map_or_else(
            || stats.map_or(1, |stats| stats.action_amount),
            |enemy| enemy.action_amount,
        );
    let mut action_presentation = None;
    let action_succeeded = match goal {
        AgentGoal::Gather(resource_id) => {
            let actor = simulation.actors.get(actor_id)?;
            if !actor_resource_storage_has_room(config, content, simulation, actor) {
                return None;
            }
            let gathering_pet = simulation.actors.get(actor_id).and_then(|actor| {
                actor.id.as_str().starts_with("twitch:").then_some(())?;
                match actor.role.as_str() {
                    "role:gatherer" => StableId::new("pet:giraffe").ok(),
                    "role:fisher" => StableId::new("pet:duck").ok(),
                    "role:logger" => StableId::new("pet:butterfly").ok(),
                    _ => None,
                }
            });
            let resource = world
                .resources
                .iter_mut()
                .find(|resource| resource.id == *resource_id && resource.amount > 0)?;
            let amount = resource.amount.min(action_amount);
            resource.amount -= amount;
            let resource_kind = resource.kind.clone();
            if let Err(error) = simulation.gather(actor_id, resource_kind, amount) {
                warn!(actor = %actor_id, %error, "resource gather action failed");
                resource.amount = resource.amount.saturating_add(amount);
                false
            } else {
                if amount > 0
                    && let Some(pet) = gathering_pet
                    && simulation
                        .try_unlock_gathering_pet(actor_id, pet.clone())
                        .unwrap_or(false)
                {
                    info!(actor = %actor_id, %pet, "unlocked gathering pet");
                }
                amount > 0
            }
        }
        AgentGoal::Deposit => {
            let capacities = simulation
                .actors
                .get(actor_id)
                .into_iter()
                .flat_map(|actor| actor.inventory.keys())
                .map(|resource| {
                    (
                        resource.clone(),
                        resource_storage_capacity(config, content, simulation, resource),
                    )
                })
                .collect();
            let resources_before = simulation.town_resources.clone();
            match simulation.deposit_all_with_capacities(actor_id, &capacities) {
                Ok(amount) => {
                    if amount > 0 {
                        let gained: Vec<_> = simulation
                            .town_resources
                            .iter()
                            .filter_map(|(resource, current)| {
                                let previous =
                                    resources_before.get(resource).copied().unwrap_or_default();
                                current
                                    .saturating_sub(previous)
                                    .gt(&0)
                                    .then(|| (resource.clone(), current.saturating_sub(previous)))
                            })
                            .collect();
                        for (resource, amount) in gained {
                            let _ = simulation.record_objective_event(
                                &content.objectives,
                                &ObjectiveEvent::ResourceGained { resource, amount },
                            );
                        }
                    }
                    amount > 0
                }
                Err(error) => {
                    warn!(actor = %actor_id, %error, "resource deposit action failed");
                    false
                }
            }
        }
        AgentGoal::Attack(target_id) => {
            let attacker = simulation.actors.get(actor_id)?;
            let target = simulation.actors.get(target_id)?;
            let visual = actor_combat_visual(&attacker.role);
            let target_position = target.position;
            if !attacker.alive
                || !target.alive
                || target.position.x.abs_diff(current.x) + target.position.z.abs_diff(current.z)
                    > role_action_range_cells(content, simulation, attacker)
            {
                return None;
            }
            let damage = action_amount;
            if is_ranged_role(&attacker.role) {
                action_presentation = Some(ActionPresentation::Projectile(ProjectileSpawn {
                    source: ProjectileSource::Actor(actor_id.clone()),
                    target: target_id.clone(),
                    damage,
                    speed_cells_per_second: 12.0,
                    visual,
                }));
                damage > 0
            } else {
                let defense = effective_role_stats(content, simulation, target)
                    .map_or(0, |stats| stats.damage_reduction_percent);
                let damage = percentage_reduced(damage, i32::try_from(defense).unwrap_or(i32::MAX));
                match apply_combat_damage(simulation, content, target_id, damage) {
                    Ok(_) => {
                        if damage > 0 {
                            action_presentation = Some(ActionPresentation::Impact {
                                target: target_position,
                                visual,
                            });
                            true
                        } else {
                            false
                        }
                    }
                    Err(error) => {
                        warn!(actor = %actor_id, target = %target_id, %error, "combat action failed");
                        false
                    }
                }
            }
        }
        AgentGoal::AttackBuilding(building_id) => {
            let attacker = simulation.actors.get(actor_id)?;
            let building = simulation.buildings.get(building_id)?;
            if !attacker.alive || building.health <= 0 {
                return None;
            }
            let building_position = building_visual_grid(content, building);
            let building_origin = building.position;
            let building_footprint =
                building_def_for_archetype(content, &building.archetype).map(|definition| {
                    rotated_footprint(definition.footprint, building.rotation_quarter_turns)
                });
            match simulation.damage_building(building_id, action_amount) {
                Ok(remaining) if action_amount > 0 => {
                    if remaining == 0 {
                        if let Some(footprint) = building_footprint
                            && let Some(region) = building_region(building_origin, footprint, world)
                        {
                            let _ = world.navigation.set_blocked(region, false);
                        }
                        simulation.buildings.remove(building_id);
                        for actor in simulation.actors.values_mut() {
                            if actor.station.as_ref() == Some(building_id)
                                || actor.preferred_target.as_ref() == Some(building_id)
                            {
                                actor.station = None;
                                actor.preferred_target = None;
                            }
                        }
                        action_presentation = Some(ActionPresentation::BuildingDestroyed {
                            building: building_id.clone(),
                            target: building_position,
                        });
                    } else {
                        action_presentation = Some(ActionPresentation::BuildingWork {
                            target: building_position,
                            sparks: true,
                        });
                    }
                    true
                }
                Ok(_) => false,
                Err(error) => {
                    warn!(actor = %actor_id, building = %building_id, %error, "building attack failed");
                    false
                }
            }
        }
        AgentGoal::Heal(target_id) => {
            let healer = simulation.actors.get(actor_id)?;
            let target = simulation.actors.get(target_id)?;
            let target_position = target.position;
            if !healer.alive
                || !target.alive
                || target.role.as_str() == "role:enemy"
                || target.health >= target.max_health
                || target.position.x.abs_diff(current.x) + target.position.z.abs_diff(current.z)
                    > role_action_range_cells(content, simulation, healer)
            {
                return None;
            }
            match simulation.heal_actor(target_id, action_amount) {
                Ok(restored) => {
                    if restored > 0 {
                        action_presentation = Some(ActionPresentation::Healing {
                            source: current,
                            target: target_position,
                        });
                        true
                    } else {
                        false
                    }
                }
                Err(error) => {
                    warn!(actor = %actor_id, target = %target_id, %error, "healing action failed");
                    false
                }
            }
        }
        AgentGoal::Construct(building_id) => {
            let was_incomplete = simulation
                .buildings
                .get(building_id)
                .is_some_and(|building| !building.complete);
            let building_position = simulation
                .buildings
                .get(building_id)
                .map(|building| building_visual_grid(content, building));
            let archetype = simulation
                .buildings
                .get(building_id)
                .map(|building| building.archetype.clone());
            let result = if was_incomplete {
                simulation.work_on_building(building_id, action_amount)
            } else {
                simulation
                    .repair_building(building_id, action_amount)
                    .map(|restored| restored > 0)
            };
            match result {
                Ok(complete) => {
                    if was_incomplete
                        && complete
                        && let Some(building) = archetype.as_ref().and_then(|archetype| {
                            content.buildings.iter().find_map(|(id, definition)| {
                                (definition.archetype == *archetype).then_some(id.clone())
                            })
                        })
                    {
                        let _ = simulation.record_objective_event(
                            &content.objectives,
                            &ObjectiveEvent::BuildingBuilt(building),
                        );
                    }
                    let succeeded = action_amount > 0 && (was_incomplete || complete);
                    if succeeded && let Some(target) = building_position {
                        action_presentation = Some(ActionPresentation::BuildingWork {
                            target,
                            sparks: was_incomplete,
                        });
                    }
                    succeeded
                }
                Err(error) => {
                    warn!(actor = %actor_id, building = %building_id, %error, "construction action failed");
                    false
                }
            }
        }
        AgentGoal::Wander => false,
    };
    if action_succeeded
        && let Some(stats) = stats
        && let Ok(levels_gained) = simulation.grant_role_experience(
            actor_id,
            action_amount,
            stats.experience_multiplier_per_thousand,
        )
        && levels_gained > 0
    {
        let max_health = simulation
            .actors
            .get(actor_id)
            .and_then(|actor| effective_role_stats(content, simulation, actor))
            .map_or(1, |stats| stats.max_health);
        if let Some(actor) = simulation.actors.get_mut(actor_id) {
            actor.max_health = i32::try_from(max_health).unwrap_or(i32::MAX);
            actor.health = actor.max_health;
        }
    }
    action_presentation
}

fn apply_combat_damage(
    simulation: &mut WorldSimulation,
    content: &ContentCatalog,
    target_id: &StableId,
    damage: u32,
) -> Result<bool, stream_town_domain::SimulationError> {
    if std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some() {
        return Ok(false);
    }
    let enemy_type = simulation
        .actors
        .get(target_id)
        .filter(|target| target.role.as_str() == "role:enemy")
        .and_then(|target| actor_archetype(content, target))
        .and_then(|archetype| archetype.enemy.as_ref())
        .map(|enemy| enemy.enemy_type.clone());
    let killed = simulation.damage_actor(target_id, damage)?;
    if killed && let Some(enemy_type) = enemy_type {
        let _ = simulation.record_objective_event(
            &content.objectives,
            &ObjectiveEvent::EnemyKilled(enemy_type),
        );
    }
    Ok(killed)
}

fn spawn_combat_projectile(
    commands: &mut Commands,
    render: &RenderAssets,
    config: &GameConfig,
    origin: Vec3,
    projectile: ProjectileSpawn,
) {
    let scale = config.world.cell_size * 0.14;
    let material = match projectile.visual {
        CombatVisualKind::Necrotic => render.projectile_necrotic.clone(),
        CombatVisualKind::Arrow => render.projectile_arrow.clone(),
        CombatVisualKind::Physical | CombatVisualKind::Fireball => render.projectile.clone(),
    };
    let mut entity = commands.spawn((
        WorldEntity,
        CombatProjectile {
            source: projectile.source,
            target: projectile.target,
            damage: projectile.damage,
            speed_cells_per_second: projectile.speed_cells_per_second,
            visual: projectile.visual,
            trail_cooldown_seconds: 0.0,
        },
        Transform::from_translation(origin + Vec3::Y * config.world.cell_size * 0.35)
            .with_scale(Vec3::splat(scale)),
    ));
    if projectile.visual == CombatVisualKind::Arrow
        && let Some(scene) = render.projectile_arrow_scene.clone()
    {
        entity.insert(WorldAssetRoot(scene));
    } else {
        entity.insert((Mesh3d(render.cube.clone()), MeshMaterial3d(material)));
    }
}

fn combat_material(render: &RenderAssets, visual: CombatVisualKind) -> Handle<StandardMaterial> {
    match visual {
        CombatVisualKind::Physical => render.impact_physical.clone(),
        CombatVisualKind::Arrow => render.projectile_arrow.clone(),
        CombatVisualKind::Fireball => render.projectile.clone(),
        CombatVisualKind::Necrotic => render.projectile_necrotic.clone(),
    }
}

fn spawn_combat_smoke_field(
    commands: &mut Commands,
    render: &RenderAssets,
    focus: Vec3,
    cell_size: f32,
) {
    let spacing = cell_size * 2.4;
    for (offset, visual) in [
        (-1.5_f32, CombatVisualKind::Physical),
        (-0.5, CombatVisualKind::Arrow),
        (0.5, CombatVisualKind::Fireball),
        (1.5, CombatVisualKind::Necrotic),
    ] {
        let origin = focus + Vec3::X * offset * spacing;
        spawn_combat_impact(commands, render, origin, visual, cell_size);
        for trail_index in 0..7_u16 {
            spawn_combat_trail(
                commands,
                render,
                origin + Vec3::Z * f32::from(trail_index) * cell_size * 0.32,
                Vec3::Z,
                visual,
                cell_size,
            );
        }
    }
}

fn spawn_combat_smoke_arrow(
    commands: &mut Commands,
    render: &RenderAssets,
    focus: Vec3,
    cell_size: f32,
) {
    let origin = focus + Vec3::new(-cell_size * 6.0, cell_size * 0.45, -cell_size * 2.8);
    let target = focus + Vec3::new(cell_size * 5.0, cell_size * 0.45, -cell_size * 2.8);
    let mut transform =
        Transform::from_translation(origin).with_scale(Vec3::splat(cell_size * 0.14));
    transform.look_to(target - origin, Vec3::Y);
    let mut entity = commands.spawn((
        WorldEntity,
        CombatProjectile {
            source: ProjectileSource::Building(
                StableId::new("building:townhall").expect("static stable ID"),
            ),
            target: StableId::new("actor:enemy_0000").expect("static stable ID"),
            damage: 0,
            speed_cells_per_second: 0.0,
            visual: CombatVisualKind::Arrow,
            trail_cooldown_seconds: f32::MAX,
        },
        transform,
    ));
    if let Some(scene) = render.projectile_arrow_scene.clone() {
        entity.insert(WorldAssetRoot(scene));
    } else {
        entity.insert((
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.projectile_arrow.clone()),
        ));
    }
}

fn repeat_combat_smoke(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    mut cooldown_seconds: Local<f32>,
    mut arrow_spawned: Local<bool>,
) {
    if std::env::var_os("STREAM_TOWN_SMOKE_COMBAT_VFX").is_none() {
        return;
    }
    *cooldown_seconds -= time.delta_secs();
    if *cooldown_seconds > 0.0 {
        return;
    }
    let centre = GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    };
    let focus = grid_to_world_on_surface(centre, &config.0, &world.generated)
        + Vec3::Y * config.0.world.cell_size * 0.35;
    spawn_combat_smoke_field(&mut commands, &render, focus, config.0.world.cell_size);
    if !*arrow_spawned {
        spawn_combat_smoke_arrow(&mut commands, &render, focus, config.0.world.cell_size);
        *arrow_spawned = true;
    }
    *cooldown_seconds = 0.72;
}

fn spawn_combat_trail(
    commands: &mut Commands,
    render: &RenderAssets,
    position: Vec3,
    direction: Vec3,
    visual: CombatVisualKind,
    cell_size: f32,
) {
    let (duration_seconds, scale) = match visual {
        CombatVisualKind::Arrow => (
            TOWER_TRAIL_SECONDS,
            Vec3::new(TOWER_TRAIL_WIDTH * 0.35, TOWER_TRAIL_WIDTH * 0.35, 0.22),
        ),
        CombatVisualKind::Fireball => (
            0.6,
            Vec3::new(FIREBALL_TRAIL_SIZE * 0.27, FIREBALL_TRAIL_SIZE * 0.27, 0.16),
        ),
        CombatVisualKind::Necrotic => (0.75, Vec3::new(0.07, 0.07, 0.14)),
        CombatVisualKind::Physical => return,
    };
    let base_scale = scale * cell_size;
    let mut transform = Transform::from_translation(position).with_scale(base_scale);
    transform.look_to(direction.normalize_or_zero(), Vec3::Y);
    commands.spawn((
        WorldEntity,
        CombatTrailSegment {
            elapsed_seconds: 0.0,
            duration_seconds,
            base_scale,
        },
        Mesh3d(render.cube.clone()),
        MeshMaterial3d(combat_material(render, visual)),
        transform,
    ));
}

fn spawn_combat_impact(
    commands: &mut Commands,
    render: &RenderAssets,
    origin: Vec3,
    visual: CombatVisualKind,
    cell_size: f32,
) {
    let (particle_count, duration_seconds, speed) = match visual {
        CombatVisualKind::Physical | CombatVisualKind::Arrow => {
            (8_u16, CHARACTER_HIT_SECONDS, 1.25)
        }
        CombatVisualKind::Fireball => (14, 0.55, 2.0),
        CombatVisualKind::Necrotic => (12, 0.65, 1.5),
    };
    for index in 0..particle_count {
        let phase = f32::from(index) / f32::from(particle_count);
        let angle = phase * std::f32::consts::TAU;
        let vertical = 0.35 + f32::from(index % 3) * 0.24;
        let velocity = Vec3::new(angle.cos(), vertical, angle.sin()) * speed * cell_size;
        let base_scale = Vec3::splat(
            cell_size
                * if visual == CombatVisualKind::Fireball {
                    FIREBALL_SIZE * 0.275
                } else {
                    0.075
                },
        );
        commands.spawn((
            WorldEntity,
            CombatImpactParticle {
                elapsed_seconds: 0.0,
                duration_seconds,
                origin,
                velocity,
                base_scale,
            },
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(combat_material(render, visual)),
            Transform::from_translation(origin).with_scale(base_scale),
        ));
    }
}

fn animate_combat_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut trails: Query<(Entity, &mut CombatTrailSegment, &mut Transform)>,
    mut impacts: Query<
        (Entity, &mut CombatImpactParticle, &mut Transform),
        Without<CombatTrailSegment>,
    >,
) {
    for (entity, mut trail, mut transform) in &mut trails {
        trail.elapsed_seconds += time.delta_secs();
        let progress = trail.elapsed_seconds / trail.duration_seconds;
        if progress >= 1.0 {
            commands.entity(entity).despawn();
            continue;
        }
        transform.scale = trail.base_scale * (1.0 - progress);
    }
    for (entity, mut impact, mut transform) in &mut impacts {
        impact.elapsed_seconds += time.delta_secs();
        let progress = impact.elapsed_seconds / impact.duration_seconds;
        if progress >= 1.0 {
            commands.entity(entity).despawn();
            continue;
        }
        transform.translation = impact.origin
            + impact.velocity * impact.elapsed_seconds
            + Vec3::NEG_Y * 0.5 * 9.8 * impact.elapsed_seconds.powi(2);
        transform.rotation *= Quat::from_rotation_y(time.delta_secs() * 5.0);
        transform.scale = impact.base_scale * (1.0 - progress).sqrt();
    }
}

fn building_effect_material(
    render: &RenderAssets,
    kind: BuildingEffectKind,
) -> Handle<StandardMaterial> {
    match kind {
        BuildingEffectKind::WorkSmoke | BuildingEffectKind::DamageSmoke => {
            render.building_smoke.clone()
        }
        BuildingEffectKind::WorkSpark => render.building_spark.clone(),
        BuildingEffectKind::DamageFire => render.building_fire.clone(),
        BuildingEffectKind::LevelArrow => render.building_upgrade.clone(),
    }
}

fn spawn_building_particle(
    commands: &mut Commands,
    render: &RenderAssets,
    kind: BuildingEffectKind,
    origin: Vec3,
    velocity: Vec3,
    base_scale: Vec3,
    duration_seconds: f32,
    phase: f32,
) {
    commands.spawn((
        WorldEntity,
        BuildingEffectParticle {
            kind,
            elapsed_seconds: 0.0,
            duration_seconds,
            origin,
            velocity,
            base_scale,
            phase,
        },
        Mesh3d(render.cube.clone()),
        MeshMaterial3d(building_effect_material(render, kind)),
        bevy::light::NotShadowCaster,
        bevy::light::NotShadowReceiver,
        Transform::from_translation(origin).with_scale(Vec3::ZERO),
    ));
}

fn spawn_building_work_effect(
    commands: &mut Commands,
    render: &RenderAssets,
    origin: Vec3,
    sparks: bool,
    cell_size: f32,
) {
    for index in 0..5_u16 {
        let phase = f32::from(index) / 5.0;
        let angle = phase * std::f32::consts::TAU + 0.35;
        let particle_origin =
            origin + Vec3::new(angle.cos(), 0.16 + phase * 0.24, angle.sin()) * cell_size * 0.34;
        spawn_building_particle(
            commands,
            render,
            BuildingEffectKind::WorkSmoke,
            particle_origin,
            Vec3::new(
                angle.cos() * 0.22,
                BUILDING_HIT_SMOKE_SPEED,
                angle.sin() * 0.22,
            ) * cell_size,
            Vec3::splat(BUILDING_HIT_SMOKE_SIZE * cell_size * 0.34),
            BUILDING_HIT_SECONDS,
            phase,
        );
    }
    if !sparks {
        return;
    }
    for index in 0..8_u16 {
        let phase = f32::from(index) / 8.0;
        let angle = phase * std::f32::consts::TAU;
        spawn_building_particle(
            commands,
            render,
            BuildingEffectKind::WorkSpark,
            origin + Vec3::Y * cell_size * 0.35,
            Vec3::new(angle.cos(), 0.28 + f32::from(index % 3) * 0.12, angle.sin())
                * BUILDING_HIT_SPARK_SPEED
                * cell_size
                * 0.16,
            Vec3::new(0.045, BUILDING_HIT_SPARK_SIZE, 0.045) * cell_size,
            BUILDING_HIT_SECONDS,
            phase,
        );
    }
}

fn spawn_building_level_up_effect(
    commands: &mut Commands,
    render: &RenderAssets,
    origin: Vec3,
    cell_size: f32,
) {
    let radius = BUILDING_LEVEL_UP_TILE_SIZE * cell_size * 0.38;
    for index in 0..8_u16 {
        let phase = f32::from(index) / 8.0;
        let angle = phase * std::f32::consts::TAU;
        let arrow_origin = origin + Vec3::new(angle.cos() * radius, 0.1, angle.sin() * radius);
        spawn_building_particle(
            commands,
            render,
            BuildingEffectKind::LevelArrow,
            arrow_origin,
            Vec3::Y * cell_size * (1.15 + phase * 0.55),
            Vec3::new(0.12, BUILDING_LEVEL_UP_ARROW_SIZE, 0.12) * cell_size,
            BUILDING_LEVEL_UP_SECONDS,
            phase,
        );
    }
}

fn building_damage_intensity(health: i32, complete: bool) -> f32 {
    if !complete || health <= 0 {
        return 0.0;
    }
    let health_ratio = building_damage_value(health);
    ((0.65 - health_ratio) / 0.65).clamp(0.0, 1.0)
}

fn emit_damaged_building_effects(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    simulation: Res<SimulationRuntime>,
    render: Res<RenderAssets>,
    mut buildings: Query<(
        &RuntimeBuilding,
        &BuildingPresentation,
        &mut BuildingDamageEmitter,
    )>,
) {
    for (runtime, presentation, mut emitter) in &mut buildings {
        let Some(building) = simulation.0.buildings.get(&runtime.id) else {
            continue;
        };
        let intensity = building_damage_intensity(building.health, building.complete);
        if intensity <= f32::EPSILON {
            emitter.cooldown_seconds = 0.0;
            continue;
        }
        emitter.cooldown_seconds -= time.delta_secs();
        if emitter.cooldown_seconds > 0.0 {
            continue;
        }
        let cell_size = config.0.world.cell_size;
        let phase =
            f32::from(u16::try_from(emitter.sequence % 16).expect("sequence is bounded")) / 16.0;
        let angle = phase * std::f32::consts::TAU * 2.618_034;
        let radial = BUILDING_DAMAGED_RADIUS * cell_size * (0.18 + 0.38 * intensity);
        let origin = presentation.base_translation
            + Vec3::new(angle.cos() * radial, cell_size * 0.44, angle.sin() * radial);
        spawn_building_particle(
            &mut commands,
            &render,
            BuildingEffectKind::DamageSmoke,
            origin,
            Vec3::new(
                angle.cos() * 0.16,
                0.78 + intensity * 0.52,
                angle.sin() * 0.16,
            ) * cell_size,
            Vec3::splat(cell_size * (0.22 + intensity * 0.18)),
            1.55,
            phase,
        );
        if emitter.sequence.is_multiple_of(2) {
            spawn_building_particle(
                &mut commands,
                &render,
                BuildingEffectKind::DamageFire,
                origin,
                Vec3::Y * cell_size * (0.45 + intensity * 0.35),
                Vec3::new(0.16, 0.4, 0.16) * cell_size * (0.65 + intensity * 0.45),
                0.72,
                phase,
            );
        }
        emitter.sequence = emitter.sequence.wrapping_add(1);
        let authored_density =
            f32::from(BUILDING_DAMAGED_FIRE_AMOUNT) / f32::from(BUILDING_DAMAGED_SMOKE_AMOUNT);
        emitter.cooldown_seconds = (0.22 - intensity * 0.12) / authored_density.max(0.1);
    }
}

fn animate_building_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut effects: Query<(Entity, &mut BuildingEffectParticle, &mut Transform)>,
) {
    for (entity, mut effect, mut transform) in &mut effects {
        effect.elapsed_seconds += time.delta_secs();
        let progress = effect.elapsed_seconds / effect.duration_seconds;
        if progress >= 1.0 {
            commands.entity(entity).despawn();
            continue;
        }
        let envelope = (std::f32::consts::PI * progress).sin().max(0.0);
        match effect.kind {
            BuildingEffectKind::WorkSmoke => {
                transform.translation = effect.origin + effect.velocity * effect.elapsed_seconds;
                transform.scale = effect.base_scale * envelope.sqrt() * (0.72 + progress * 0.8);
                transform.rotation *= Quat::from_rotation_y(time.delta_secs() * 1.6);
            }
            BuildingEffectKind::WorkSpark => {
                transform.translation = effect.origin
                    + effect.velocity * effect.elapsed_seconds
                    + Vec3::NEG_Y * 0.5 * 9.8 * effect.elapsed_seconds.powi(2);
                transform.scale = effect.base_scale * (1.0 - progress);
                transform.rotation *= Quat::from_rotation_z(time.delta_secs() * 9.0);
            }
            BuildingEffectKind::LevelArrow => {
                transform.translation = effect.origin + effect.velocity * effect.elapsed_seconds;
                transform.scale = effect.base_scale * envelope.sqrt();
                transform.rotation =
                    Quat::from_rotation_y(effect.phase * std::f32::consts::TAU + progress * 0.35);
            }
            BuildingEffectKind::DamageSmoke => {
                transform.translation = effect.origin + effect.velocity * effect.elapsed_seconds;
                transform.scale = effect.base_scale * envelope.sqrt() * (0.6 + progress * 1.45);
                transform.rotation *= Quat::from_rotation_y(time.delta_secs() * 0.8);
            }
            BuildingEffectKind::DamageFire => {
                transform.translation = effect.origin + effect.velocity * effect.elapsed_seconds;
                let flicker = 0.78 + (progress * 31.0 + effect.phase * 13.0).sin().abs() * 0.32;
                transform.scale = effect.base_scale * envelope * flicker;
                transform.rotation *= Quat::from_rotation_y(time.delta_secs() * 4.0);
            }
        }
    }
}

fn spawn_building_smoke_field(
    commands: &mut Commands,
    render: &RenderAssets,
    focus: Vec3,
    cell_size: f32,
) {
    let spacing = cell_size * 3.4;
    spawn_building_work_effect(
        commands,
        render,
        focus - Vec3::X * spacing,
        false,
        cell_size,
    );
    spawn_building_work_effect(commands, render, focus, true, cell_size);
    spawn_building_level_up_effect(commands, render, focus + Vec3::X * spacing, cell_size);
    for sequence in 0..10_u16 {
        let phase = f32::from(sequence) / 10.0;
        let angle = phase * std::f32::consts::TAU;
        let origin = focus
            + Vec3::Z * spacing
            + Vec3::new(angle.cos(), 0.4, angle.sin()) * BUILDING_DAMAGED_RADIUS * cell_size;
        spawn_building_particle(
            commands,
            render,
            if sequence.is_multiple_of(3) {
                BuildingEffectKind::DamageFire
            } else {
                BuildingEffectKind::DamageSmoke
            },
            origin,
            Vec3::Y * cell_size * if sequence.is_multiple_of(3) { 0.7 } else { 1.0 },
            Vec3::splat(cell_size * 0.32),
            if sequence.is_multiple_of(3) {
                0.72
            } else {
                1.55
            },
            phase,
        );
    }
}

fn repeat_building_smoke(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    mut cooldown_seconds: Local<f32>,
) {
    if std::env::var_os("STREAM_TOWN_SMOKE_BUILDING_VFX").is_none() {
        return;
    }
    *cooldown_seconds -= time.delta_secs();
    if *cooldown_seconds > 0.0 {
        return;
    }
    let centre = GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    };
    let focus = grid_to_world_on_surface(centre, &config.0, &world.generated);
    spawn_building_smoke_field(&mut commands, &render, focus, config.0.world.cell_size);
    *cooldown_seconds = 0.82;
}

fn healing_effect_duration(kind: HealingEffectKind) -> f32 {
    match kind {
        HealingEffectKind::Channel => HEALING_CHANNEL_SECONDS,
        HealingEffectKind::Burst | HealingEffectKind::Revive => HEALED_BURST_SECONDS,
    }
}

fn healing_effect_sample(kind: HealingEffectKind, elapsed_seconds: f32) -> HealingEffectSample {
    let duration = healing_effect_duration(kind);
    let progress = (elapsed_seconds / duration).clamp(0.0, 1.0);
    let envelope = (std::f32::consts::PI * progress).sin().max(0.0);
    let channel_size = if elapsed_seconds <= 1.5 {
        0.289 * elapsed_seconds / 1.5
    } else if elapsed_seconds <= 3.0 {
        0.289 + (1.0 - 0.289) * (elapsed_seconds - 1.5) / 1.5
    } else {
        (1.0 - (elapsed_seconds - 3.0) / 2.0).max(0.0)
    };
    match kind {
        HealingEffectKind::Channel => HealingEffectSample {
            ring_scale: channel_size,
            mote_scale: envelope.sqrt(),
            radial_distance: 0.28 + progress * 0.58,
            rise: 0.18 + progress * 1.9,
            rotation_radians: progress * std::f32::consts::TAU * 1.5,
        },
        HealingEffectKind::Burst => HealingEffectSample {
            ring_scale: envelope.sqrt() * (0.35 + progress * 1.85),
            mote_scale: envelope,
            radial_distance: 0.22 + progress * 1.35,
            rise: 0.2 + progress * 2.4,
            rotation_radians: progress * std::f32::consts::TAU,
        },
        HealingEffectKind::Revive => HealingEffectSample {
            ring_scale: envelope.sqrt() * (0.5 + progress * 2.55),
            mote_scale: envelope,
            radial_distance: 0.3 + progress * 1.75,
            rise: 0.25 + progress * 3.1,
            rotation_radians: progress * std::f32::consts::TAU * 1.25,
        },
    }
}

fn spawn_healing_effect(
    commands: &mut Commands,
    render: &RenderAssets,
    origin: Vec3,
    kind: HealingEffectKind,
    cell_size: f32,
) {
    let material = if kind == HealingEffectKind::Revive {
        render.healing_gold.clone()
    } else {
        render.healing_green.clone()
    };
    let base_scale = match kind {
        HealingEffectKind::Burst => cell_size * 0.7,
        HealingEffectKind::Channel | HealingEffectKind::Revive => cell_size * 0.85,
    };
    commands.spawn((
        WorldEntity,
        HealingRingEffect {
            kind,
            origin,
            elapsed_seconds: 0.0,
            base_scale,
        },
        Mesh3d(render.healing_ring.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(origin + Vec3::Y * 0.08).with_scale(Vec3::ZERO),
    ));

    let mote_count: u16 = match kind {
        HealingEffectKind::Channel => 8,
        HealingEffectKind::Burst => 7,
        HealingEffectKind::Revive => 12,
    };
    for mote_index in 0..mote_count {
        let phase = f32::from(mote_index) / f32::from(mote_count);
        let angle_radians = phase * std::f32::consts::TAU;
        for base_scale in [
            Vec3::new(cell_size * 0.055, cell_size * 0.2, cell_size * 0.05),
            Vec3::new(cell_size * 0.2, cell_size * 0.055, cell_size * 0.05),
        ] {
            commands.spawn((
                WorldEntity,
                HealingMoteEffect {
                    kind,
                    origin,
                    elapsed_seconds: -phase * 0.22,
                    angle_radians,
                    phase,
                    base_scale,
                    distance_scale: cell_size,
                },
                Mesh3d(render.cube.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(origin).with_scale(Vec3::ZERO),
            ));
        }
    }
}

fn animate_healing_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut rings: Query<(Entity, &mut HealingRingEffect, &mut Transform)>,
    mut motes: Query<(Entity, &mut HealingMoteEffect, &mut Transform), Without<HealingRingEffect>>,
) {
    for (entity, mut effect, mut transform) in &mut rings {
        effect.elapsed_seconds += time.delta_secs();
        if effect.elapsed_seconds >= healing_effect_duration(effect.kind) {
            commands.entity(entity).despawn();
            continue;
        }
        let sample = healing_effect_sample(effect.kind, effect.elapsed_seconds);
        transform.translation = effect.origin + Vec3::Y * 0.08;
        transform.rotation = Quat::from_rotation_y(sample.rotation_radians * 0.2);
        transform.scale = Vec3::splat(effect.base_scale * sample.ring_scale);
    }
    for (entity, mut effect, mut transform) in &mut motes {
        effect.elapsed_seconds += time.delta_secs();
        if effect.elapsed_seconds >= healing_effect_duration(effect.kind) {
            commands.entity(entity).despawn();
            continue;
        }
        let sample = healing_effect_sample(effect.kind, effect.elapsed_seconds.max(0.0));
        let angle =
            effect.angle_radians + sample.rotation_radians + effect.phase * std::f32::consts::PI;
        let radius = sample.radial_distance * effect.distance_scale;
        transform.translation = effect.origin
            + Vec3::new(
                angle.cos() * radius,
                sample.rise * effect.distance_scale,
                angle.sin() * radius,
            );
        transform.rotation = Quat::from_rotation_y(-angle * 0.35);
        transform.scale = effect.base_scale * sample.mote_scale;
    }
}

fn move_combat_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    render: Res<RenderAssets>,
    mut simulation: ResMut<SimulationRuntime>,
    actors: Query<(&Agent, &Transform), Without<CombatProjectile>>,
    mut projectiles: Query<(Entity, &mut CombatProjectile, &mut Transform), Without<Agent>>,
) {
    let positions: BTreeMap<_, _> = actors
        .iter()
        .map(|(agent, transform)| (agent.id.clone(), transform.translation))
        .collect();
    for (entity, mut projectile, mut transform) in &mut projectiles {
        let source_valid = match &projectile.source {
            ProjectileSource::Actor(actor) => simulation
                .0
                .actors
                .get(actor)
                .is_some_and(|actor| actor.alive),
            ProjectileSource::Building(building) => simulation
                .0
                .buildings
                .get(building)
                .is_some_and(|building| building.complete),
        };
        let Some(target) = simulation.0.actors.get(&projectile.target) else {
            commands.entity(entity).despawn();
            continue;
        };
        let Some(target_position) = positions.get(&projectile.target).copied() else {
            commands.entity(entity).despawn();
            continue;
        };
        if !source_valid || !target.alive {
            commands.entity(entity).despawn();
            continue;
        }
        let target_position = target_position + Vec3::Y * config.0.world.cell_size * 0.35;
        let delta = target_position - transform.translation;
        let step = projectile.speed_cells_per_second * config.0.world.cell_size * time.delta_secs();
        if delta.length_squared() <= step.max(0.1).powi(2) {
            let defense = simulation
                .0
                .actors
                .get(&projectile.target)
                .and_then(|target| effective_role_stats(&content.0, &simulation.0, target))
                .map_or(0, |stats| stats.damage_reduction_percent);
            let damage = percentage_reduced(
                projectile.damage,
                i32::try_from(defense).unwrap_or(i32::MAX),
            );
            if let Err(error) =
                apply_combat_damage(&mut simulation.0, &content.0, &projectile.target, damage)
            {
                warn!(target = %projectile.target, %error, "projectile impact failed");
            }
            spawn_combat_impact(
                &mut commands,
                &render,
                target_position,
                projectile.visual,
                config.0.world.cell_size,
            );
            commands.entity(entity).despawn();
        } else {
            transform.translation += delta.normalize_or_zero() * step;
            transform.look_to(delta.normalize_or_zero(), Vec3::Y);
            projectile.trail_cooldown_seconds -= time.delta_secs();
            if projectile.trail_cooldown_seconds <= 0.0 {
                spawn_combat_trail(
                    &mut commands,
                    &render,
                    transform.translation,
                    delta,
                    projectile.visual,
                    config.0.world.cell_size,
                );
                projectile.trail_cooldown_seconds = 0.045;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn update_enemy_encounters(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    mut simulation: ResMut<SimulationRuntime>,
    agents: Query<(Entity, &Agent)>,
) {
    let dead_enemies: Vec<_> = agents
        .iter()
        .filter(|(_, agent)| agent.kind == ActorKind::Enemy)
        .filter(|(_, agent)| {
            !simulation
                .0
                .actors
                .get(&agent.id)
                .is_some_and(|actor| actor.alive)
        })
        .map(|(entity, agent)| (entity, agent.id.clone()))
        .collect();
    for (entity, enemy) in dead_enemies {
        commands.entity(entity).despawn();
        simulation.0.actors.remove(&enemy);
        for camp in simulation.0.enemy_camps.values_mut() {
            camp.spawned_enemies.remove(&enemy);
        }
        if let Some(raid) = &mut simulation.0.active_raid {
            raid.tracked_enemies.remove(&enemy);
        }
    }

    if simulation.0.active_event == Some(TownEvent::EnemyRaid)
        && simulation.0.active_raid.is_none()
        && let (Some(enemy), Some(boss)) = (
            archetype_id_by_source(&content.0, ArchetypeKind::Enemy, "Enemy_Minotaur.prefab"),
            archetype_id_by_source(
                &content.0,
                ArchetypeKind::Enemy,
                "Enemy_MinotaurBoss.prefab",
            ),
        )
    {
        let _ = simulation.0.start_raid(5, 50, enemy, boss);
    }

    let next_wave = simulation.0.active_raid.as_ref().and_then(|raid| {
        raid.tracked_enemies.is_empty().then(|| {
            if raid.current_wave >= raid.total_waves {
                None
            } else {
                let final_wave = raid.current_wave + 1 == raid.total_waves;
                Some((
                    if final_wave {
                        raid.boss_archetype.clone()
                    } else {
                        raid.enemy_archetype.clone()
                    },
                    if final_wave { 1 } else { raid.enemies_per_wave },
                    final_wave,
                ))
            }
        })
    });
    if matches!(next_wave, Some(None)) {
        simulation.0.finish_raid();
    } else if let Some(Some((archetype, count, final_wave))) = next_wave {
        let camp = simulation.0.enemy_camps.values().next().cloned();
        let spawner = camp.as_ref().and_then(|camp| {
            content
                .0
                .archetypes
                .get(&camp.archetype)
                .and_then(|archetype| archetype.enemy_spawner.clone())
        });
        if let (Some(camp), Some(spawner)) = (camp, spawner) {
            let mut wave_members = BTreeSet::new();
            for _ in 0..count {
                let serial = simulation.0.next_enemy_serial;
                let position = enemy_spawn_position(&world.generated, &camp, &spawner, serial);
                if let Some(enemy) = spawn_runtime_enemy(
                    &mut commands,
                    &config.0,
                    &world.generated,
                    &content.0,
                    &render,
                    &mut simulation.0,
                    archetype.clone(),
                    position,
                ) {
                    if final_wave {
                        let player_count = simulation
                            .0
                            .actors
                            .values()
                            .filter(|actor| actor.role.as_str() != "role:enemy")
                            .count();
                        let boss_health =
                            i32::try_from(50_usize.saturating_mul(player_count).max(1_000))
                                .unwrap_or(i32::MAX);
                        if let Some(actor) = simulation.0.actors.get_mut(&enemy) {
                            actor.health = boss_health;
                            actor.max_health = boss_health;
                        }
                    }
                    wave_members.insert(enemy);
                }
            }
            if let Some(raid) = &mut simulation.0.active_raid {
                raid.current_wave = raid.current_wave.saturating_add(1);
                raid.tracked_enemies = wave_members;
            }
        } else {
            simulation.0.finish_raid();
        }
        return;
    }

    if simulation.0.active_raid.is_some() || simulation.0.elapsed_seconds.rem_euclid(120.0) < 80.0 {
        return;
    }
    let player_count = simulation
        .0
        .actors
        .values()
        .filter(|actor| actor.role.as_str() != "role:enemy")
        .count();
    let camp_ids: Vec<_> = simulation.0.enemy_camps.keys().cloned().collect();
    for camp_id in camp_ids {
        let Some(mut camp) = simulation.0.enemy_camps.get(&camp_id).cloned() else {
            continue;
        };
        let Some(spawner) = content
            .0
            .archetypes
            .get(&camp.archetype)
            .and_then(|archetype| archetype.enemy_spawner.as_ref())
        else {
            continue;
        };
        camp.spawn_remaining_seconds =
            (camp.spawn_remaining_seconds - time.delta_secs_f64()).max(0.0);
        let day_cap = usize::try_from(simulation.0.day)
            .unwrap_or(usize::MAX)
            .saturating_add(player_count / 10)
            .clamp(
                usize::from(spawner.min_total_enemies),
                usize::from(spawner.max_total_enemies),
            );
        if camp.spawned_enemies.len() < day_cap && camp.spawn_remaining_seconds <= f64::EPSILON {
            let serial = simulation.0.next_enemy_serial;
            let archetype = weighted_enemy_archetype(spawner, simulation.0.world_seed, serial);
            let position = enemy_spawn_position(&world.generated, &camp, spawner, serial);
            if let Some(enemy) = spawn_runtime_enemy(
                &mut commands,
                &config.0,
                &world.generated,
                &content.0,
                &render,
                &mut simulation.0,
                archetype,
                position,
            ) {
                camp.spawned_enemies.insert(enemy);
            }
            camp.spawn_remaining_seconds = f64::from(spawner.spawn_milliseconds) / 1_000.0;
        }
        simulation.0.enemy_camps.insert(camp_id, camp);
    }
}

#[allow(clippy::too_many_arguments)]
fn sync_fish_god_presentation(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    simulation: Res<SimulationRuntime>,
    existing: Query<Entity, With<FishGodPresentation>>,
) {
    let active = simulation.0.fish_god.is_some();
    if !active {
        for entity in &existing {
            commands.entity(entity).despawn();
        }
        return;
    }
    if !existing.is_empty() {
        return;
    }
    let spawn = nearest_walkable(
        &world.generated,
        GridPos {
            x: config.0.world.width / 2,
            z: config.0.world.height / 2 + 6,
        },
    )
    .unwrap_or(GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    });
    let position = grid_to_world_on_surface(spawn, &config.0, &world.generated);
    let fish_god = archetype_by_source(&content.0, ArchetypeKind::Other, "Event_FishGod.prefab")
        .or_else(|| {
            content
                .0
                .archetypes
                .values()
                .find(|archetype| archetype.source_path.ends_with("Event_FishGod.prefab"))
        });
    let mut entity = commands.spawn((
        WorldEntity,
        FishGodPresentation,
        Transform::from_translation(position),
    ));
    if let Some((archetype, scene)) = fish_god
        .and_then(|archetype| default_archetype_scene(archetype).map(|scene| (archetype, scene)))
        .filter(|(_, scene)| {
            asset_server.is_some() && converted_asset_exists(&asset_root.0, &scene.asset_path)
        })
    {
        entity.insert((
            WorldAssetRoot(
                asset_server
                    .as_deref()
                    .expect("asset server checked above")
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            Transform::from_translation(position)
                .with_scale(Vec3::splat(config.0.world.cell_size / 2.0)),
        ));
        if let Some(material) = prefab_material_spec(archetype, scene, &presentation.0, &render) {
            entity.insert(material);
        }
    } else {
        let scale = config.0.world.cell_size * 1.5;
        entity.insert((
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.food.clone()),
            Transform::from_translation(position + Vec3::Y * scale * 0.5)
                .with_scale(Vec3::splat(scale)),
        ));
    }
    for index in 0..48_u32 {
        let x = f32::from(u16::try_from(index % 8).unwrap_or(0)) - 3.5;
        let z = f32::from(u16::try_from(index / 8).unwrap_or(0)) - 2.5;
        let top_height = position.y
            + config.0.world.cell_size * (3.0 + f32::from(u16::try_from(index % 5).unwrap_or(0)));
        let floor_height = position.y + config.0.world.cell_size * 0.15;
        commands.spawn((
            WorldEntity,
            FishGodPresentation,
            FallingFish {
                floor_height,
                top_height,
                fall_speed: config.0.world.cell_size
                    * (2.0 + f32::from(u16::try_from(index % 3).unwrap_or(0)) * 0.4),
            },
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.food.clone()),
            Transform::from_xyz(
                position.x + x * config.0.world.cell_size * 1.4,
                top_height,
                position.z + z * config.0.world.cell_size * 1.4,
            )
            .with_scale(Vec3::new(
                config.0.world.cell_size * 0.25,
                config.0.world.cell_size * 0.12,
                config.0.world.cell_size * 0.5,
            )),
        ));
    }
}

fn animate_falling_fish(time: Res<Time>, mut fish: Query<(&FallingFish, &mut Transform)>) {
    for (fish, mut transform) in &mut fish {
        transform.translation.y -= fish.fall_speed * time.delta_secs();
        if transform.translation.y <= fish.floor_height {
            transform.translation.y = fish.top_height;
        }
        transform.rotate_y(time.delta_secs() * 2.5);
    }
}

fn update_tower_shooters(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    render: Res<RenderAssets>,
    simulation: Res<SimulationRuntime>,
    mut towers: Query<(&RuntimeBuilding, &Transform, &mut TowerShooter)>,
) {
    for (runtime, transform, mut tower) in &mut towers {
        tower.cooldown_seconds = (tower.cooldown_seconds - time.delta_secs()).max(0.0);
        let Some(state) = simulation.0.buildings.get(&runtime.id) else {
            continue;
        };
        if !state.complete || tower.cooldown_seconds > f32::EPSILON {
            continue;
        }
        let Some((definition, shooter)) = building_def_for_archetype(&content.0, &state.archetype)
            .and_then(|definition| {
                definition
                    .projectile_shooter
                    .as_ref()
                    .map(|shooter| (definition, shooter))
            })
        else {
            continue;
        };
        let footprint = rotated_footprint(definition.footprint, state.rotation_quarter_turns);
        let centre = GridPos {
            x: state.position.x.saturating_add(footprint[0] / 2),
            z: state.position.z.saturating_add(footprint[1] / 2),
        };
        let range_cells = shooter.range_milli_cells.div_ceil(1_000);
        let Some(target) = simulation
            .0
            .actors
            .values()
            .filter(|actor| actor.alive && actor.role.as_str() == "role:enemy")
            .filter(|actor| {
                u32::from(actor.position.x.abs_diff(centre.x))
                    + u32::from(actor.position.z.abs_diff(centre.z))
                    <= range_cells
            })
            .min_by_key(|actor| {
                (
                    actor.position.x.abs_diff(centre.x) + actor.position.z.abs_diff(centre.z),
                    actor.id.clone(),
                )
            })
        else {
            continue;
        };
        spawn_combat_projectile(
            &mut commands,
            &render,
            &config.0,
            transform.translation + Vec3::Y * config.0.world.cell_size * 0.5,
            ProjectileSpawn {
                source: ProjectileSource::Building(tower.building.clone()),
                target: target.id.clone(),
                damage: shooter.damage,
                speed_cells_per_second: milli_units_as_f32(shooter.movement_milli_cells_per_second),
                visual: CombatVisualKind::Arrow,
            },
        );
        tower.cooldown_seconds = milli_units_as_f32(shooter.fire_milliseconds);
    }
}

fn action_cooldown(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &StableId,
    goal: &AgentGoal,
) -> f32 {
    let fallback = match goal {
        AgentGoal::Attack(_) | AgentGoal::AttackBuilding(_) | AgentGoal::Heal(_) => 1.0,
        AgentGoal::Construct(_) => 0.5,
        AgentGoal::Gather(_) => 0.75,
        AgentGoal::Deposit => 0.25,
        AgentGoal::Wander => 0.0,
    };
    let Some(actor) = simulation.actors.get(actor) else {
        return fallback;
    };
    if matches!(goal, AgentGoal::Attack(_) | AgentGoal::AttackBuilding(_))
        && let Some(enemy) =
            actor_archetype(content, actor).and_then(|archetype| archetype.enemy.as_ref())
    {
        return milli_units_as_f32(enemy.action_milliseconds).max(0.1);
    }
    let base = if matches!(
        goal,
        AgentGoal::Attack(_)
            | AgentGoal::AttackBuilding(_)
            | AgentGoal::Construct(_)
            | AgentGoal::Gather(_)
            | AgentGoal::Heal(_)
    ) {
        effective_role_stats(content, simulation, actor).map_or(fallback, |stats| {
            milli_units_as_f32(stats.action_milliseconds).max(0.1)
        })
    } else {
        fallback
    };
    if base <= f32::EPSILON {
        0.0
    } else {
        base.max(0.1)
    }
}

fn move_agents(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    render: Res<RenderAssets>,
    mut world: ResMut<WorldRuntime>,
    mut simulation: ResMut<SimulationRuntime>,
    mut stats: ResMut<SessionStats>,
    mut agents: Query<(
        &mut Agent,
        &mut GridLocation,
        &AgentAnimation,
        &mut Transform,
    )>,
    buildings: Query<(Entity, &RuntimeBuilding)>,
) {
    stats.elapsed_seconds += time.delta_secs_f64();
    simulation.0.tick(time.delta_secs());
    apply_passive_building_income(&config.0, &content.0, &mut simulation.0, time.delta());
    if let Some(event) = simulation.0.take_next_queued_event() {
        match event {
            TownEvent::FishGod => {
                let _ = simulation.0.start_fish_god(true);
            }
            other => simulation.0.trigger_event(other),
        }
    }
    if let Some(technology) = simulation
        .0
        .active_vote
        .as_ref()
        .filter(|vote| vote.remaining_seconds <= f32::EPSILON)
        .map(|vote| vote.technology.clone())
    {
        let objectives = content
            .0
            .technology
            .nodes
            .get(&technology)
            .map_or(&[][..], |node| node.objectives.as_slice());
        let _ =
            simulation
                .0
                .resolve_technology_vote(objectives, &content.0.objectives, MAX_TOWN_GOALS);
    }
    if let Some(weather) = debug_weather_override() {
        simulation.0.weather = weather;
    }
    for (mut agent, mut location, animation, mut transform) in &mut agents {
        agent.action_cooldown_seconds =
            (agent.action_cooldown_seconds - time.delta_secs()).max(0.0);
        if let Some((role_stats, regeneration_requires_food)) = simulation
            .0
            .actors
            .get(&agent.id)
            .filter(|actor| actor.alive)
            .and_then(|actor| {
                effective_role_stats(&content.0, &simulation.0, actor).map(|stats| {
                    let requires_food = actor_archetype(&content.0, actor)
                        .and_then(|archetype| archetype.health.as_ref())
                        .is_some_and(|health| health.regeneration_requires_food);
                    (stats, requires_food)
                })
            })
        {
            let desired_max = i32::try_from(role_stats.max_health)
                .unwrap_or(i32::MAX)
                .max(1);
            let regen_milli = i32::try_from(
                role_stats
                    .health_regen_milli_per_second
                    .clamp(0, i64::from(i32::MAX)),
            )
            .expect("clamped regeneration fits i32");
            let regen_per_second = f64::from(regen_milli) / 1_000.0;
            agent.health_regen_accumulator += regen_per_second * time.delta_secs_f64();
            let mut regenerated = 0_i32;
            while agent.health_regen_accumulator >= 1.0 && regenerated < desired_max {
                agent.health_regen_accumulator -= 1.0;
                regenerated += 1;
            }
            if regeneration_requires_food && regenerated > 0 {
                let food = StableId::new("resource:food").expect("static stable ID");
                let required = u32::try_from(regenerated).expect("regeneration is non-negative");
                let available = simulation
                    .0
                    .town_resources
                    .get(&food)
                    .copied()
                    .unwrap_or_default();
                if available >= required {
                    simulation
                        .0
                        .town_resources
                        .insert(food, available - required);
                } else {
                    regenerated = 0;
                }
            }
            if let Some(actor) = simulation.0.actors.get_mut(&agent.id) {
                let increase = desired_max.saturating_sub(actor.max_health).max(0);
                actor.max_health = desired_max;
                actor.health = actor
                    .health
                    .saturating_add(increase)
                    .saturating_add(regenerated)
                    .min(desired_max);
            }
        }
        let alive = simulation
            .0
            .actors
            .get(&agent.id)
            .is_some_and(|actor| actor.alive);
        if alive {
            if let Some(actor_position) = simulation
                .0
                .actors
                .get(&agent.id)
                .map(|actor| actor.position)
                && actor_position != location.0
            {
                let mut world_position =
                    grid_to_world_on_surface(actor_position, &config.0, &world.generated);
                if !animation.native {
                    world_position.y += animation.base_scale.y * 0.5;
                }
                transform.translation = world_position;
                location.0 = actor_position;
                agent.origin = actor_position;
                agent.target = mirrored_target(&world.generated, actor_position);
                agent.path.clear();
                agent.path_index = 0;
                agent.action_cooldown_seconds = 0.0;
            }
        } else {
            agent.path.clear();
            agent.goal = AgentGoal::Wander;
            agent.health_regen_accumulator = 0.0;
            let remaining = simulation
                .0
                .actors
                .get(&agent.id)
                .and_then(|actor| actor.respawn_remaining_seconds);
            if remaining.is_none() {
                let Some(duration) = simulation
                    .0
                    .actors
                    .get(&agent.id)
                    .and_then(|actor| authored_respawn_milliseconds(&content.0, actor))
                else {
                    continue;
                };
                let _ = simulation
                    .0
                    .schedule_respawn(&agent.id, f64::from(duration) / 1_000.0);
                continue;
            }
            if remaining.is_some_and(|remaining| remaining > f64::EPSILON) {
                continue;
            }
            let spawn = nearest_walkable(&world.generated, agent.spawn).unwrap_or(agent.spawn);
            if simulation.0.respawn_actor(&agent.id, spawn).is_err() {
                continue;
            }
            let mut world_position = grid_to_world_on_surface(spawn, &config.0, &world.generated);
            if !animation.native {
                world_position.y += animation.base_scale.y * 0.5;
            }
            transform.translation = world_position;
            location.0 = spawn;
            agent.origin = spawn;
            agent.target = mirrored_target(&world.generated, spawn);
            agent.action_cooldown_seconds = 0.0;
            spawn_healing_effect(
                &mut commands,
                &render,
                grid_to_world_on_surface(spawn, &config.0, &world.generated),
                HealingEffectKind::Revive,
                config.0.world.cell_size,
            );
        }
        ensure_actor_station(&content.0, &mut simulation.0, &config.0, &agent.id);
        if agent.path.is_empty() || agent.path_index >= agent.path.len() {
            if !agent.path.is_empty() {
                stats.paths_completed += 1;
                if location.0 == agent.target && agent.action_cooldown_seconds <= f32::EPSILON {
                    if let Some(presentation) = complete_agent_goal(
                        &mut simulation.0,
                        &mut world.generated,
                        &config.0,
                        &content.0,
                        &agent.id,
                        &agent.goal,
                        location.0,
                    ) {
                        match presentation {
                            ActionPresentation::Projectile(projectile) => {
                                spawn_combat_projectile(
                                    &mut commands,
                                    &render,
                                    &config.0,
                                    transform.translation,
                                    projectile,
                                );
                            }
                            ActionPresentation::Impact { target, visual } => {
                                spawn_combat_impact(
                                    &mut commands,
                                    &render,
                                    grid_to_world_on_surface(target, &config.0, &world.generated)
                                        + Vec3::Y * config.0.world.cell_size * 0.35,
                                    visual,
                                    config.0.world.cell_size,
                                );
                            }
                            ActionPresentation::Healing { source, target } => {
                                let source =
                                    grid_to_world_on_surface(source, &config.0, &world.generated);
                                let target =
                                    grid_to_world_on_surface(target, &config.0, &world.generated);
                                spawn_healing_effect(
                                    &mut commands,
                                    &render,
                                    source,
                                    HealingEffectKind::Channel,
                                    config.0.world.cell_size,
                                );
                                spawn_healing_effect(
                                    &mut commands,
                                    &render,
                                    target,
                                    HealingEffectKind::Burst,
                                    config.0.world.cell_size,
                                );
                            }
                            ActionPresentation::BuildingWork { target, sparks } => {
                                spawn_building_work_effect(
                                    &mut commands,
                                    &render,
                                    grid_to_world_on_surface(target, &config.0, &world.generated),
                                    sparks,
                                    config.0.world.cell_size,
                                );
                            }
                            ActionPresentation::BuildingDestroyed { building, target } => {
                                let origin =
                                    grid_to_world_on_surface(target, &config.0, &world.generated);
                                spawn_building_work_effect(
                                    &mut commands,
                                    &render,
                                    origin,
                                    true,
                                    config.0.world.cell_size * 1.5,
                                );
                                if let Some((entity, _)) =
                                    buildings.iter().find(|(_, runtime)| runtime.id == building)
                                {
                                    commands.entity(entity).despawn();
                                }
                            }
                        }
                    }
                    agent.action_cooldown_seconds =
                        action_cooldown(&content.0, &simulation.0, &agent.id, &agent.goal);
                }
            }
            agent.origin = location.0;
            let (goal, target) = next_agent_goal(
                &simulation.0,
                &world.generated,
                &config.0,
                &content.0,
                &agent.id,
                location.0,
            );
            agent.goal = goal;
            agent.target = target;
            agent.path = world
                .generated
                .navigation
                .find_path(location.0, agent.target)
                .unwrap_or_else(|_| vec![location.0]);
            agent.path_index = usize::from(agent.path.len() > 1);
        }
        let Some(next) = agent.path.get(agent.path_index).copied() else {
            continue;
        };
        let mut target = grid_to_world_on_surface(next, &config.0, &world.generated);
        if !animation.native {
            target.y += animation.base_scale.y * 0.5;
        }
        let distance = target - transform.translation;
        let speed = simulation.0.actors.get(&agent.id).map_or(
            config.0.gameplay.agent_speed_cells_per_second,
            |actor| {
                effective_role_stats(&content.0, &simulation.0, actor)
                    .map_or(config.0.gameplay.agent_speed_cells_per_second, |stats| {
                        milli_units_as_f32(stats.movement_speed_milli_cells_per_second)
                    })
            },
        );
        let step = speed * config.0.world.cell_size * time.delta_secs();
        if distance.length_squared() <= step * step {
            transform.translation = target;
            location.0 = next;
            agent.path_index += 1;
            if let Some(actor) = simulation.0.actors.get_mut(&agent.id) {
                actor.position = next;
            }
        } else {
            transform.translation += distance.normalize_or_zero() * step;
        }
    }
}

fn sync_resource_nodes(
    world: Res<WorldRuntime>,
    mut resources: Query<(&ResourceNode, &mut Visibility)>,
) {
    if !world.is_changed() {
        return;
    }
    for (node, mut visibility) in &mut resources {
        let available = world
            .generated
            .resources
            .iter()
            .find(|resource| resource.id == node.id)
            .is_some_and(|resource| resource.amount > 0);
        *visibility = if available {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

fn building_construction_stage(health: i32, complete: bool) -> u8 {
    if complete || health >= BUILDING_MAX_HEALTH {
        3
    } else if health > BUILDING_MAX_HEALTH * 66 / 100 {
        2
    } else {
        u8::from(health > BUILDING_MAX_HEALTH * 33 / 100)
    }
}

#[allow(clippy::type_complexity)]
fn sync_building_presentation(
    mut commands: Commands,
    simulation: Res<SimulationRuntime>,
    content: Res<RuntimeContent>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    mut buildings: Query<(
        &RuntimeBuilding,
        &mut BuildingPresentation,
        &mut Transform,
        Option<&mut MeshMaterial3d<StandardMaterial>>,
        Option<&mut WorldAssetRoot>,
    )>,
) {
    for (runtime, mut presentation, mut transform, material, world_asset) in &mut buildings {
        let Some(state) = simulation.0.buildings.get(&runtime.id) else {
            continue;
        };
        let building = content
            .0
            .buildings
            .iter()
            .find(|(_, building)| building.archetype == state.archetype);
        let age = building.map_or(1, |(building_id, _)| {
            building_age(&content.0, &simulation.0, building_id)
        });
        let construction_stage = building_construction_stage(state.health, state.complete);
        if presentation.applied_stage == construction_stage
            && presentation.applied_level == state.level
            && presentation.applied_age == age
        {
            continue;
        }
        if presentation.applied_level != u16::MAX && state.level > presentation.applied_level {
            spawn_building_level_up_effect(
                &mut commands,
                &render,
                presentation.base_translation,
                config.0.world.cell_size,
            );
        }
        if presentation.applied_age != age {
            if let (Some((_, building)), Some(mut world_asset), Some(asset_server)) =
                (building, world_asset, asset_server.as_deref())
            {
                let archetype = &content.0.archetypes[&building.archetype];
                if let Some(scene) = archetype_scene_for_age(archetype, age)
                    .filter(|scene| converted_asset_exists(&asset_root.0, &scene.asset_path))
                {
                    world_asset.0 = asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone()));
                }
            }
            presentation.applied_age = age;
        }
        let is_town_hall = runtime.id.as_str() == "building:townhall";
        let stage_scale = match construction_stage {
            0 => 0.35,
            1 => 0.55,
            2 => 0.75,
            _ => 1.0,
        };
        let level_scale = if is_town_hall {
            1.0
        } else {
            1.0 + f32::from(state.level.saturating_sub(1)) * 0.05
        };
        let scale = stage_scale * level_scale;
        transform.scale = presentation.base_scale * scale;
        transform.translation = presentation.base_translation
            - Vec3::Y * presentation.base_height_offset * (1.0 - stage_scale);
        if let Some(mut material) = material {
            material.0 = if state.complete {
                render.building.clone()
            } else {
                render.construction.clone()
            };
        }
        presentation.applied_stage = construction_stage;
        presentation.applied_level = state.level;
    }
}

#[allow(clippy::too_many_arguments)]
fn update_environment_presentation(
    mut commands: Commands,
    simulation: Res<SimulationRuntime>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    mut presentation: ResMut<EnvironmentPresentation>,
    mut clear_color: Option<ResMut<ClearColor>>,
    mut terrain_materials: Option<ResMut<Assets<TerrainMaterial>>>,
    mut water_materials: Option<ResMut<Assets<WaterMaterial>>>,
    mut building_materials: Option<ResMut<Assets<BuildingMaterial>>>,
    mut tree_materials: Option<ResMut<Assets<TreeMaterial>>>,
    mut cameras: Query<(&mut DistanceFog, &mut AmbientLight), With<TownCamera>>,
    mut lights: Query<&mut DirectionalLight>,
    particles: Query<Entity, With<WeatherParticle>>,
) {
    let environment = (simulation.0.season, simulation.0.weather);
    if presentation.applied == Some(environment) {
        return;
    }
    let palette = environment_palette(environment.0, environment.1);
    if let Some(clear_color) = clear_color.as_deref_mut() {
        clear_color.0 = Color::srgb(
            palette.clear_color[0],
            palette.clear_color[1],
            palette.clear_color[2],
        );
    }
    if let Some(terrain_materials) = terrain_materials.as_deref_mut()
        && let Some(mut ground) = terrain_materials.get_mut(&render.ground)
    {
        ground.extension.parameters.season_tint = Vec4::new(
            palette.terrain_tint[0],
            palette.terrain_tint[1],
            palette.terrain_tint[2],
            ground.extension.parameters.season_tint.w,
        );
    }
    if let Some(water_materials) = water_materials.as_deref_mut()
        && let Some(mut water) = water_materials.get_mut(&render.water)
    {
        let surface = water.extension.parameters.surface_color;
        water.extension.parameters.season_tint = water_color_tint(surface, palette.water_color);
        water.extension.parameters.scale_foam_ice.w = water_ice_strength(environment.0);
    }
    if let Some(building_materials) = building_materials.as_deref_mut()
        && let Some(mut building) = building_materials.get_mut(&render.authored_building)
    {
        let snow = building_snow_strength(environment.0);
        building.extension.parameters.snow_damage.x = snow;
        building.extension.parameters.snow_damage.y = snow;
    }
    if let Some(tree_materials) = tree_materials.as_deref_mut()
        && let Some(mut tree) = tree_materials.get_mut(&render.tree)
    {
        tree.extension.parameters.season_controls = tree_season_controls(environment.0);
    }
    for (mut fog, mut ambient) in &mut cameras {
        fog.color = Color::srgba(
            palette.fog_color[0],
            palette.fog_color[1],
            palette.fog_color[2],
            palette.fog_color[3],
        );
        fog.falloff = FogFalloff::Linear {
            start: palette.fog_start,
            end: palette.fog_end,
        };
        ambient.color = Color::srgb(
            palette.ambient_color[0],
            palette.ambient_color[1],
            palette.ambient_color[2],
        );
        ambient.brightness = palette.ambient_brightness;
    }
    for mut light in &mut lights {
        light.color = Color::srgb(
            palette.sun_color[0],
            palette.sun_color[1],
            palette.sun_color[2],
        );
        light.illuminance = palette.sun_illuminance;
    }
    for entity in &particles {
        commands.entity(entity).despawn();
    }
    spawn_weather_particles(
        &mut commands,
        &config.0,
        &render,
        simulation.0.world_seed,
        environment.1,
        palette.particle_count,
    );
    info!(
        season = ?environment.0,
        weather = ?environment.1,
        particles = palette.particle_count,
        "environment presentation updated"
    );
    presentation.applied = Some(environment);
}

fn spawn_weather_particles(
    commands: &mut Commands,
    config: &GameConfig,
    render: &RenderAssets,
    world_seed: u64,
    weather: Weather,
    count: u16,
) {
    let (material, scale) = match weather {
        Weather::Rain => (
            render.rain.clone(),
            Vec3::new(0.18, config.world.cell_size * 0.48, 0.18),
        ),
        Weather::Snow => (render.snow.clone(), Vec3::splat(0.62)),
        Weather::Clear | Weather::Fog => return,
    };
    let span_x = f32::from(config.world.width) * config.world.cell_size;
    let span_z = f32::from(config.world.height) * config.world.cell_size;
    for index in 0..count {
        let seed = weather_particle_seed(world_seed, index);
        let x = (unit_from_seed(seed) - 0.5) * span_x;
        let z = (unit_from_seed(seed.rotate_left(13)) - 0.5) * span_z;
        let y = 18.0 + unit_from_seed(seed.rotate_left(23)) * 88.0;
        commands.spawn((
            WorldEntity,
            WeatherParticle {
                kind: weather,
                seed,
            },
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(x, y, z).with_scale(scale),
        ));
    }
}

fn animate_weather_particles(
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    mut particles: Query<(&WeatherParticle, &mut Transform)>,
) {
    let half_x = f32::from(config.0.world.width) * config.0.world.cell_size * 0.5;
    let half_z = f32::from(config.0.world.height) * config.0.world.cell_size * 0.5;
    for (particle, mut transform) in &mut particles {
        let speed = if particle.kind == Weather::Rain {
            52.0
        } else {
            9.0
        };
        transform.translation.y -= speed * time.delta_secs();
        if particle.kind == Weather::Snow {
            let seed_phase = f32::from(
                u16::try_from(particle.seed & 0xff).expect("masked weather seed fits u16"),
            );
            transform.translation.x +=
                (time.elapsed_secs() * 0.72 + seed_phase).sin() * time.delta_secs() * 1.8;
        }
        if transform.translation.y < -2.0 {
            transform.translation.y = 94.0 + unit_from_seed(particle.seed.rotate_left(7)) * 24.0;
        }
        if transform.translation.x < -half_x {
            transform.translation.x += half_x * 2.0;
        } else if transform.translation.x > half_x {
            transform.translation.x -= half_x * 2.0;
        }
        if transform.translation.z < -half_z {
            transform.translation.z += half_z * 2.0;
        } else if transform.translation.z > half_z {
            transform.translation.z -= half_z * 2.0;
        }
    }
}

fn environment_palette(season: Season, weather: Weather) -> EnvironmentPalette {
    let (terrain_tint, water_color, clear_color, sun_color, ambient_color) = match season {
        Season::Spring => (
            [0.94, 1.0, 0.92],
            [0.05, 0.29, 0.47, 0.62],
            [0.025, 0.04, 0.055],
            [1.0, 0.95, 0.84],
            [0.70, 0.82, 0.92],
        ),
        Season::Summer => (
            [1.0, 0.96, 0.78],
            [0.03, 0.34, 0.54, 0.62],
            [0.035, 0.045, 0.05],
            [1.0, 0.88, 0.68],
            [0.82, 0.78, 0.68],
        ),
        Season::Autumn => (
            [1.0, 0.64, 0.30],
            [0.08, 0.25, 0.36, 0.65],
            [0.055, 0.035, 0.035],
            [1.0, 0.72, 0.50],
            [0.82, 0.62, 0.52],
        ),
        Season::Winter => (
            [0.76, 0.88, 1.0],
            [0.10, 0.27, 0.40, 0.68],
            [0.032, 0.044, 0.060],
            [0.78, 0.88, 1.0],
            [0.68, 0.78, 0.92],
        ),
    };
    let (sun_illuminance, ambient_brightness, fog_color, fog_start, fog_end, particle_count) =
        match weather {
            Weather::Clear => (14_000.0, 90.0, [0.58, 0.72, 0.78, 0.08], 560.0, 940.0, 0),
            Weather::Rain => (7_500.0, 72.0, [0.32, 0.42, 0.50, 0.32], 240.0, 650.0, 180),
            Weather::Fog => (5_500.0, 80.0, [0.62, 0.68, 0.69, 0.72], 70.0, 390.0, 0),
            Weather::Snow => (9_000.0, 105.0, [0.76, 0.84, 0.90, 0.42], 170.0, 590.0, 150),
        };
    EnvironmentPalette {
        terrain_tint,
        water_color,
        clear_color,
        sun_color,
        sun_illuminance,
        ambient_color,
        ambient_brightness,
        fog_color,
        fog_start,
        fog_end,
        particle_count,
    }
}

fn water_ice_strength(season: Season) -> f32 {
    if season == Season::Winter { 1.0 } else { 0.0 }
}

fn water_color_tint(surface: Vec4, target: [f32; 4]) -> Vec4 {
    Vec4::new(
        target[0] / surface.x.max(0.1),
        target[1] / surface.y.max(0.1),
        target[2] / surface.z.max(0.1),
        target[3],
    )
}

fn building_snow_strength(season: Season) -> f32 {
    if season == Season::Winter { 1.0 } else { 0.0 }
}

fn tree_season_controls(season: Season) -> Vec4 {
    match season {
        Season::Spring => Vec4::new(0.0, 0.0, 0.1, 0.0),
        Season::Summer => Vec4::ZERO,
        Season::Autumn => Vec4::new(0.3, 0.0, 0.0, 0.0),
        Season::Winter => Vec4::new(0.0, 0.5, 0.0, 0.0),
    }
}

fn weather_particle_seed(world_seed: u64, index: u16) -> u32 {
    let mut value = world_seed
        ^ u64::from(index)
            .wrapping_add(1)
            .wrapping_mul(0x9E37_79B9_7F4A_7C15);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^= value >> 27;
    u32::try_from(value & u64::from(u32::MAX)).expect("masked weather seed fits u32")
}

fn unit_from_seed(seed: u32) -> f32 {
    let fraction = u16::try_from(seed & u32::from(u16::MAX)).expect("masked seed fits u16");
    f32::from(fraction) / f32::from(u16::MAX)
}

fn animate_agents(
    time: Res<Time>,
    render: Res<RenderAssets>,
    mut agents: Query<(
        &Agent,
        &mut AgentAnimation,
        Option<&mut MeshMaterial3d<StandardMaterial>>,
        &mut Transform,
    )>,
) {
    for (agent, mut animation, material, mut transform) in &mut agents {
        let moving = agent_is_moving(agent);
        let next_state = if moving {
            MovementAnimationState::Moving
        } else {
            MovementAnimationState::Idle
        };
        if animation.state != next_state {
            animation.state = next_state;
            animation.phase = 0.0;
        }
        animation.phase += time.delta_secs() * if moving { 9.0 } else { 2.0 };
        if animation.native {
            transform.scale = animation.base_scale;
            continue;
        }
        let pulse = if moving {
            1.0 + animation.phase.sin().abs() * 0.16
        } else {
            1.0
        };
        transform.scale = animation.base_scale * Vec3::new(1.0, pulse, 1.0);
        if let Some(mut material) = material {
            material.0 = actor_material(&render, &agent.kind, moving);
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn upgrade_actor_placeholders(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    world: Res<WorldRuntime>,
    mut placeholders: Query<
        (
            Entity,
            &Agent,
            &GridLocation,
            &mut AgentAnimation,
            &mut Transform,
        ),
        With<Mesh3d>,
    >,
    detailed: Query<(), (With<Agent>, With<WorldAssetRoot>)>,
) {
    let Some(asset_server) = asset_server else {
        return;
    };
    let mut remaining = actor_scene_budget().saturating_sub(detailed.iter().count());
    for (entity, agent, location, mut animation, mut transform) in &mut placeholders {
        if remaining == 0 {
            break;
        }
        let Some(archetype) = content.0.archetypes.get(&agent.archetype) else {
            continue;
        };
        let Some(scene) = default_archetype_scene(archetype)
            .filter(|scene| converted_asset_exists(&asset_root.0, &scene.asset_path))
        else {
            continue;
        };
        let native = native_animation_request(archetype, scene, &presentation.0);
        let converted = native
            .is_none()
            .then(|| converted_animation_spec(archetype, &presentation.0))
            .flatten();
        let base_scale = Vec3::splat(config.0.world.cell_size / 2.0);
        animation.base_scale = base_scale;
        animation.native = native.is_some() || converted.is_some();
        *transform = Transform::from_translation(grid_to_world_on_surface(
            location.0,
            &config.0,
            &world.generated,
        ))
        .with_scale(base_scale);
        let mut actor = commands.entity(entity);
        actor
            .remove::<Mesh3d>()
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .insert(WorldAssetRoot(asset_server.load(
                GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone()),
            )));
        if let Some(native) = native {
            actor.insert(native);
        } else if let Some(converted) = converted {
            actor.insert(converted);
        }
        if let Some(material) = prefab_material_spec(archetype, scene, &presentation.0, &render) {
            actor.insert(material);
        }
        remaining -= 1;
    }
}

fn resolve_native_animation_requests(
    mut commands: Commands,
    asset_server: Option<Res<AssetServer>>,
    animation_graphs: Option<ResMut<Assets<AnimationGraph>>>,
    mut cache: ResMut<NativeAnimationCache>,
    requests: Query<(Entity, &NativeAnimationRequest), Without<NativeAnimationSpec>>,
) {
    let (Some(asset_server), Some(mut animation_graphs)) = (asset_server, animation_graphs) else {
        return;
    };
    for (entity, request) in &requests {
        let key = (request.asset_path.clone(), request.animation_index);
        let spec = cache.0.entry(key).or_insert_with(|| {
            let (graph, node) = AnimationGraph::from_clip(
                asset_server.load(
                    GltfAssetLabel::Animation(
                        usize::try_from(request.animation_index)
                            .expect("animation index fits the current platform"),
                    )
                    .from_asset(request.asset_path.clone()),
                ),
            );
            NativeAnimationSpec {
                graph: animation_graphs.add(graph),
                idle: node,
                moving: node,
            }
        });
        commands
            .entity(entity)
            .insert(spec.clone())
            .remove::<NativeAnimationRequest>();
    }
}

fn attach_native_animations(
    mut commands: Commands,
    specs: Query<&NativeAnimationSpec>,
    parents: Query<&ChildOf>,
    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
) {
    for (entity, mut player) in &mut players {
        let mut ancestor = entity;
        for _ in 0..64 {
            if let Ok(spec) = specs.get(ancestor) {
                commands.entity(entity).insert((
                    AnimationGraphHandle(spec.graph.clone()),
                    ActorAnimationDriver {
                        actor_root: ancestor,
                        idle: spec.idle,
                        moving: spec.moving,
                        current: MovementAnimationState::Idle,
                    },
                ));
                player.play(spec.idle).repeat();
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
    }
}

fn attach_converted_animations(
    mut commands: Commands,
    presentation: Res<RuntimePresentation>,
    animation_clips: Option<ResMut<Assets<AnimationClip>>>,
    animation_graphs: Option<ResMut<Assets<AnimationGraph>>>,
    mut cache: ResMut<ConvertedAnimationCache>,
    specs: Query<(Entity, &ConvertedAnimationSpec), Without<ConvertedAnimationApplied>>,
    children: Query<&Children>,
    names: Query<&Name>,
    transforms: Query<&Transform>,
    applied: Query<(), With<ConvertedAnimationDriver>>,
) {
    let (Some(mut animation_clips), Some(mut animation_graphs)) =
        (animation_clips, animation_graphs)
    else {
        return;
    };
    let animation_budget = std::env::var("STREAM_TOWN_ANIMATION_BUDGET")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(DEFAULT_ACTOR_DETAIL_BUDGET);
    let mut remaining = animation_budget.saturating_sub(applied.iter().count());
    for (actor_root, spec) in &specs {
        if remaining == 0 {
            break;
        }
        let Some(controller) = presentation.0.controllers.get(&spec.controller) else {
            continue;
        };
        let Some(state) = controller.states.get(&spec.state) else {
            continue;
        };
        let Some(root_clip) = state
            .motions
            .iter()
            .find_map(|motion| presentation.0.clips.get(&motion.clip))
        else {
            continue;
        };
        let Some(root_name) = animation_root_name(root_clip) else {
            continue;
        };
        let Some(animation_root) = find_named_descendant(actor_root, root_name, &children, &names)
        else {
            continue;
        };
        let targets = collect_animation_targets(animation_root, &children, &names, &transforms);
        let cache_key = (
            spec.controller.clone(),
            spec.state.clone(),
            spec.rig_scene.clone(),
        );
        if !cache.0.contains_key(&cache_key) {
            let Some(cached) = build_converted_animation(
                controller,
                spec,
                &presentation.0,
                &targets,
                &mut animation_clips,
                &mut animation_graphs,
            ) else {
                continue;
            };
            cache.0.insert(cache_key.clone(), cached);
        }
        let cached = cache
            .0
            .get(&cache_key)
            .expect("converted animation cache was populated");
        let layers = cached
            .layers
            .iter()
            .filter_map(|template| {
                AnimationControllerRuntime::in_state(controller, template.fallback_state.clone())
                    .ok()
                    .map(|runtime| ConvertedAnimationLayerDriver {
                        display_name: template.display_name.clone(),
                        fallback_state: template.fallback_state.clone(),
                        runtime,
                        nodes: template.nodes.clone(),
                        active: Vec::new(),
                        applied: Vec::new(),
                        crossfade: None,
                        state_offset: 0.0,
                        event_elapsed: BTreeMap::new(),
                    })
            })
            .collect();
        for (path, (entity, _)) in &targets {
            commands.entity(*entity).insert((
                path.split('/').collect::<AnimationTargetId>(),
                AnimatedBy(animation_root),
            ));
        }
        commands.entity(animation_root).insert((
            AnimationPlayer::default(),
            AnimationGraphHandle(cached.graph.clone()),
            ConvertedAnimationDriver {
                actor_root,
                controller: spec.controller.clone(),
                layers,
                last_alive: None,
                active_action: None,
            },
        ));
        commands
            .entity(actor_root)
            .insert(ConvertedAnimationApplied);
        remaining -= 1;
        info!(
            actor = ?actor_root,
            controller = %spec.controller,
            state = %spec.state,
            clips = cached.clip_count,
            layers = controller.layers.len().max(1),
            targets = targets.len(),
            "attached translated Unity animation controller"
        );
    }
}

fn build_converted_animation(
    controller: &stream_town_domain::AnimationControllerDef,
    spec: &ConvertedAnimationSpec,
    presentation: &PresentationCatalog,
    targets: &BTreeMap<String, (Entity, Transform)>,
    animation_clips: &mut Assets<AnimationClip>,
    animation_graphs: &mut Assets<AnimationGraph>,
) -> Option<CachedConvertedAnimation> {
    let mut clip_ids = BTreeSet::new();
    let mut converted = Vec::new();
    for motion in controller
        .states
        .values()
        .flat_map(|state| state.motions.iter())
    {
        if !clip_ids.insert(motion.clip.clone()) {
            continue;
        }
        let source = presentation.clips.get(&motion.clip)?;
        let clip = retargeted_animation_clip(source, targets)?;
        converted.push((motion.clip.clone(), animation_clips.add(clip)));
    }
    if converted.is_empty() {
        return None;
    }
    let mut graph = AnimationGraph::new();
    let composition = graph.add_additive_blend(1.0, graph.root);
    let layer_specs: Vec<_> = controller
        .layers
        .iter()
        .enumerate()
        .filter_map(|layer| {
            let (layer_index, layer) = layer;
            let machine = controller.state_machines.get(&layer.state_machine)?;
            let state = machine.default_state.clone()?;
            Some((layer_index, layer.clone(), state))
        })
        .collect();
    let layer_specs = if layer_specs.is_empty() {
        vec![(
            0,
            AnimationLayerDef {
                display_name: "Base Layer".to_owned(),
                state_machine: StableId::new("animation_state_machine:fallback:base")
                    .expect("fallback animation state-machine ID is valid"),
                blend_mode: AnimationLayerBlendMode::Override,
                default_weight: 0.0,
                avatar_mask: None,
            },
            spec.state.clone(),
        )]
    } else {
        layer_specs
    };
    let mut layers = Vec::new();
    for (layer_index, layer, state) in layer_specs {
        let mask = register_avatar_mask(
            &mut graph,
            u32::try_from(layer_index).expect("Animator layer count fits a Bevy mask"),
            layer
                .avatar_mask
                .as_ref()
                .and_then(|mask| presentation.avatar_masks.get(mask)),
            targets,
        );
        let parent = add_animation_layer_branch(
            &mut graph,
            layer.blend_mode,
            layer.effective_weight(layer_index),
            mask,
            composition,
        );
        let nodes = converted
            .iter()
            .filter(|(clip, _)| state_layer_owns_clip(controller, &state, clip))
            .map(|(clip, handle)| (clip.clone(), graph.add_clip(handle.clone(), 1.0, parent)))
            .collect();
        layers.push(ConvertedAnimationLayerTemplate {
            display_name: layer.display_name,
            fallback_state: state,
            nodes,
        });
    }
    if layers.is_empty() {
        return None;
    }
    debug_assert!(matches!(
        graph.graph[composition].node_type,
        AnimationNodeType::Add
    ));
    Some(CachedConvertedAnimation {
        graph: animation_graphs.add(graph),
        layers,
        clip_count: converted.len(),
    })
}

fn add_animation_layer_branch(
    graph: &mut AnimationGraph,
    blend_mode: AnimationLayerBlendMode,
    weight: f32,
    mask: u64,
    parent: AnimationNodeIndex,
) -> AnimationNodeIndex {
    match (blend_mode, mask) {
        (AnimationLayerBlendMode::Override, 0) => graph.add_blend(weight, parent),
        (AnimationLayerBlendMode::Override, mask) => {
            graph.add_blend_with_mask(mask, weight, parent)
        }
        (AnimationLayerBlendMode::Additive, 0) => graph.add_additive_blend(weight, parent),
        (AnimationLayerBlendMode::Additive, mask) => {
            graph.add_additive_blend_with_mask(mask, weight, parent)
        }
    }
}

fn register_avatar_mask(
    graph: &mut AnimationGraph,
    mask_group: u32,
    authored: Option<&AvatarMaskDef>,
    targets: &BTreeMap<String, (Entity, Transform)>,
) -> u64 {
    let Some(authored) = authored else {
        return 0;
    };
    let mut excluded = false;
    for path in targets.keys() {
        if authored
            .transform_weights
            .get(path)
            .is_some_and(|weight| weight.abs() < f32::EPSILON)
        {
            graph.add_target_to_mask_group(path.split('/').collect(), mask_group);
            excluded = true;
        }
    }
    if excluded { 1_u64 << mask_group } else { 0 }
}

fn state_layer_owns_clip(
    controller: &stream_town_domain::AnimationControllerDef,
    layer_state: &StableId,
    clip: &StableId,
) -> bool {
    let Some(root) = controller.layers.iter().find_map(|layer| {
        controller
            .state_machines
            .get(&layer.state_machine)
            .and_then(|machine| {
                (machine.default_state.as_ref() == Some(layer_state))
                    .then_some(&layer.state_machine)
            })
    }) else {
        return true;
    };
    state_machine_state_ids(controller, root).any(|state| {
        controller.states[state]
            .motions
            .iter()
            .any(|motion| &motion.clip == clip)
    })
}

fn state_machine_state_ids<'a>(
    controller: &'a stream_town_domain::AnimationControllerDef,
    root: &'a StableId,
) -> impl Iterator<Item = &'a StableId> {
    let mut pending = vec![root];
    let mut states = Vec::new();
    let mut visited = BTreeSet::new();
    while let Some(machine_id) = pending.pop() {
        if !visited.insert(machine_id) {
            continue;
        }
        let Some(machine) = controller.state_machines.get(machine_id) else {
            continue;
        };
        states.extend(&machine.states);
        pending.extend(&machine.child_state_machines);
    }
    states.into_iter()
}

fn animation_root_name(clip: &AnimationClipDef) -> Option<&str> {
    clip.transform_tracks
        .iter()
        .filter(|track| track.target_path != "$root")
        .find_map(|track| track.target_path.split('/').next())
}

fn find_named_descendant(
    root: Entity,
    target: &str,
    children: &Query<&Children>,
    names: &Query<&Name>,
) -> Option<Entity> {
    let mut pending = vec![root];
    while let Some(entity) = pending.pop() {
        if names.get(entity).is_ok_and(|name| name.as_str() == target) {
            return Some(entity);
        }
        if let Ok(entity_children) = children.get(entity) {
            pending.extend(entity_children.iter().rev());
        }
    }
    None
}

fn equipment_node_names(content: &ContentCatalog) -> BTreeSet<String> {
    content
        .roles
        .values()
        .filter_map(|role| role.equipment.as_ref())
        .flat_map(|equipment| {
            equipment
                .body_nodes
                .iter()
                .cloned()
                .chain(equipment.left_hand_node.iter().cloned())
                .chain(equipment.right_hand_node.iter().cloned())
                .chain(equipment.helmet_node.iter().cloned())
        })
        .collect()
}

#[allow(clippy::type_complexity)]
fn tag_equipment_nodes(
    mut commands: Commands,
    content: Res<RuntimeContent>,
    agents: Query<Entity, With<Agent>>,
    parents: Query<&ChildOf>,
    nodes: Query<(Entity, &Name), (Without<EquipmentNode>, Without<Agent>)>,
) {
    let names = equipment_node_names(&content.0);
    for (entity, name) in &nodes {
        if !names.contains(name.as_str()) {
            continue;
        }
        let mut ancestor = entity;
        for _ in 0..64 {
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
            if agents.contains(ancestor) {
                commands.entity(entity).insert(EquipmentNode {
                    actor_root: ancestor,
                    name: name.as_str().to_owned(),
                });
                commands.entity(ancestor).insert(AgentEquipmentPresentation);
                break;
            }
        }
    }
}

fn equipment_node_visible(
    equipment: &RoleEquipmentDef,
    body_type: u8,
    name: &str,
    carrying: bool,
) -> bool {
    equipment.body_nodes[usize::from(body_type).min(equipment.body_nodes.len() - 1)] == name
        || equipment.right_hand_node.as_deref() == Some(name)
        || equipment.helmet_node.as_deref() == Some(name)
        || (equipment.left_hand_node.as_deref() == Some(name)
            && (equipment.left_hand_permanent || carrying))
}

fn sync_equipment_nodes(
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    agents: Query<&Agent, With<AgentEquipmentPresentation>>,
    mut nodes: Query<(&EquipmentNode, &mut Visibility)>,
) {
    for (node, mut visibility) in &mut nodes {
        let Ok(agent) = agents.get(node.actor_root) else {
            continue;
        };
        let Some(actor) = simulation.0.actors.get(&agent.id) else {
            continue;
        };
        let equipment = content
            .0
            .roles
            .get(&actor.role)
            .and_then(|role| role.equipment.as_ref());
        let carrying = actor.inventory.values().any(|amount| *amount > 0);
        let visible = equipment.is_some_and(|equipment| {
            equipment_node_visible(
                equipment,
                actor.customization.body_type,
                &node.name,
                carrying,
            )
        });
        *visibility = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

fn cosmetic_node(name: &str) -> Option<(CosmeticNodeKind, u8)> {
    EYE_NODES
        .iter()
        .position(|candidate| *candidate == name)
        .map(|index| {
            (
                CosmeticNodeKind::Eyes,
                u8::try_from(index).expect("eye index fits"),
            )
        })
        .or_else(|| {
            HAIR_NODES
                .iter()
                .position(|candidate| *candidate == name)
                .map(|index| {
                    (
                        CosmeticNodeKind::Hair,
                        u8::try_from(index).expect("hair index fits"),
                    )
                })
        })
        .or_else(|| {
            FACIAL_HAIR_NODES
                .iter()
                .position(|candidate| *candidate == name)
                .map(|index| {
                    (
                        CosmeticNodeKind::FacialHair,
                        u8::try_from(index).expect("facial-hair index fits"),
                    )
                })
        })
}

#[allow(clippy::type_complexity)]
fn tag_cosmetic_nodes(
    mut commands: Commands,
    agents: Query<Entity, With<Agent>>,
    parents: Query<&ChildOf>,
    nodes: Query<(Entity, &Name), (Without<CosmeticNode>, Without<Agent>)>,
) {
    for (entity, name) in &nodes {
        let Some((kind, index)) = cosmetic_node(name.as_str()) else {
            continue;
        };
        let mut ancestor = entity;
        for _ in 0..64 {
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
            if agents.contains(ancestor) {
                commands.entity(entity).insert(CosmeticNode {
                    actor_root: ancestor,
                    kind,
                    index,
                });
                break;
            }
        }
    }
}

fn cosmetic_node_visible(
    customization: ActorCustomization,
    kind: CosmeticNodeKind,
    index: u8,
    helmet_equipped: bool,
) -> bool {
    match kind {
        CosmeticNodeKind::Eyes => customization.eyes == index,
        CosmeticNodeKind::Hair => customization.hair == index && !helmet_equipped,
        CosmeticNodeKind::FacialHair => customization.facial_hair == index,
    }
}

fn sync_cosmetic_nodes(
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    agents: Query<&Agent>,
    mut nodes: Query<(&CosmeticNode, &mut Visibility)>,
) {
    for (node, mut visibility) in &mut nodes {
        let Ok(agent) = agents.get(node.actor_root) else {
            continue;
        };
        let Some(actor) = simulation.0.actors.get(&agent.id) else {
            continue;
        };
        let helmet_equipped = content
            .0
            .roles
            .get(&actor.role)
            .and_then(|role| role.equipment.as_ref())
            .and_then(|equipment| equipment.helmet_node.as_ref())
            .is_some();
        *visibility =
            if cosmetic_node_visible(actor.customization, node.kind, node.index, helmet_equipped) {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
    }
}

fn tag_cosmetic_renderers(
    mut commands: Commands,
    parents: Query<&ChildOf>,
    names: Query<&Name>,
    agents: Query<Entity, With<Agent>>,
    renderers: Query<
        (Entity, &MeshMaterial3d<StandardMaterial>),
        Added<MeshMaterial3d<StandardMaterial>>,
    >,
) {
    for (entity, material) in &renderers {
        let mut ancestor = entity;
        let mut cosmetic_kind = None;
        for _ in 0..64 {
            cosmetic_kind = cosmetic_kind.or_else(|| {
                names
                    .get(ancestor)
                    .ok()
                    .and_then(|name| cosmetic_node(name.as_str()))
                    .map(|(kind, _)| kind)
            });
            if let (true, Some(kind)) = (agents.contains(ancestor), cosmetic_kind) {
                commands.entity(entity).insert(CosmeticRenderer {
                    actor_root: ancestor,
                    kind,
                    base_material: material.0.clone(),
                    applied_color: None,
                });
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
    }
}

fn cosmetic_color(customization: ActorCustomization, kind: CosmeticNodeKind) -> (u8, [f32; 3]) {
    match kind {
        CosmeticNodeKind::Eyes => {
            let index = usize::from(customization.eye_color).min(EYE_COLORS.len() - 1);
            (
                u8::try_from(index).expect("eye color index fits"),
                EYE_COLORS[index],
            )
        }
        CosmeticNodeKind::Hair | CosmeticNodeKind::FacialHair => {
            let index = usize::from(customization.hair_color).min(HAIR_COLORS.len() - 1);
            (
                u8::try_from(index).expect("hair color index fits"),
                HAIR_COLORS[index],
            )
        }
    }
}

fn sync_cosmetic_materials(
    simulation: Res<SimulationRuntime>,
    agents: Query<&Agent>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut cache: ResMut<CosmeticMaterialCache>,
    mut renderers: Query<(&mut CosmeticRenderer, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Some(mut materials) = materials else {
        return;
    };
    for (mut cosmetic, mut renderer_material) in &mut renderers {
        let Ok(agent) = agents.get(cosmetic.actor_root) else {
            continue;
        };
        let Some(actor) = simulation.0.actors.get(&agent.id) else {
            continue;
        };
        let (color_index, color) = cosmetic_color(actor.customization, cosmetic.kind);
        if cosmetic.applied_color == Some(color_index) {
            continue;
        }
        let material = if let Some(variant) = cache.0.iter().find(|variant| {
            variant.base_material == cosmetic.base_material
                && variant.kind == cosmetic.kind
                && variant.color == color_index
        }) {
            variant.material.clone()
        } else {
            let Some(mut material) = materials.get(&cosmetic.base_material).cloned() else {
                continue;
            };
            material.base_color = Color::srgb(color[0], color[1], color[2]);
            let handle = materials.add(material);
            cache.0.push(CosmeticMaterialVariant {
                base_material: cosmetic.base_material.clone(),
                kind: cosmetic.kind,
                color: color_index,
                material: handle.clone(),
            });
            handle
        };
        renderer_material.0 = material;
        cosmetic.applied_color = Some(color_index);
    }
}

fn collect_animation_targets(
    root: Entity,
    children: &Query<&Children>,
    names: &Query<&Name>,
    transforms: &Query<&Transform>,
) -> BTreeMap<String, (Entity, Transform)> {
    let Some(root_name) = names.get(root).ok().map(Name::as_str) else {
        return BTreeMap::new();
    };
    let mut targets = BTreeMap::new();
    let mut pending = vec![(root, root_name.to_owned())];
    while let Some((entity, path)) = pending.pop() {
        if let Ok(transform) = transforms.get(entity) {
            targets.insert(path.clone(), (entity, *transform));
        }
        if let Ok(entity_children) = children.get(entity) {
            for child in entity_children.iter().rev() {
                if let Ok(name) = names.get(child) {
                    pending.push((child, format!("{path}/{}", name.as_str())));
                }
            }
        }
    }
    targets
}

fn retargeted_animation_clip(
    source: &AnimationClipDef,
    targets: &BTreeMap<String, (Entity, Transform)>,
) -> Option<AnimationClip> {
    let mut clip = AnimationClip::default();
    for track in &source.transform_tracks {
        let Some((_, rest)) = targets.get(&track.target_path) else {
            continue;
        };
        let target = track.target_path.split('/').collect::<AnimationTargetId>();
        add_translation_curve(&mut clip, target, track, rest);
        add_rotation_curve(&mut clip, target, track, rest);
        add_scale_curve(&mut clip, target, track, rest);
    }
    if clip.curves().is_empty() {
        return None;
    }
    clip.set_duration(clip.duration().max(source.duration_seconds));
    Some(clip)
}

fn add_translation_curve(
    clip: &mut AnimationClip,
    target: AnimationTargetId,
    track: &AnimationTransformTrack,
    rest: &Transform,
) {
    if track.translation.is_empty() {
        return;
    }
    let unity_reference = track.reference_translation.map_or_else(
        || Vec3::from_array(track.translation[0].value),
        Vec3::from_array,
    );
    let unity_rotation = track
        .reference_rotation
        .map_or(Quat::IDENTITY, normalized_quat);
    let basis = rest.rotation * unity_rotation.inverse();
    let unit_scale = if unity_reference.length_squared() > 1.0e-8 {
        rest.translation.length() / unity_reference.length()
    } else {
        1.0
    };
    let keys = ensure_two_keyframes(
        track.translation.iter().map(|key| {
            let delta = Vec3::from_array(key.value) - unity_reference;
            (key.time, rest.translation + basis * delta * unit_scale)
        }),
        clip.duration(),
    );
    if let Ok(curve) = AnimatableKeyframeCurve::new(keys) {
        clip.add_curve_to_target(
            target,
            AnimatableCurve::new(animated_field!(Transform::translation), curve),
        );
    }
}

fn add_rotation_curve(
    clip: &mut AnimationClip,
    target: AnimationTargetId,
    track: &AnimationTransformTrack,
    rest: &Transform,
) {
    let keys = if track.rotation.is_empty() {
        let Some(first) = track.euler_degrees.first() else {
            return;
        };
        let reference = unity_euler(first.value);
        ensure_two_keyframes(
            track.euler_degrees.iter().map(|key| {
                let delta = reference.inverse() * unity_euler(key.value);
                (key.time, (rest.rotation * delta).normalize())
            }),
            clip.duration(),
        )
    } else {
        let reference = track
            .reference_rotation
            .map_or_else(|| normalized_quat(track.rotation[0].value), normalized_quat);
        ensure_two_keyframes(
            track.rotation.iter().map(|key| {
                let delta = reference.inverse() * normalized_quat(key.value);
                (key.time, (rest.rotation * delta).normalize())
            }),
            clip.duration(),
        )
    };
    if let Ok(curve) = AnimatableKeyframeCurve::new(keys) {
        clip.add_curve_to_target(
            target,
            AnimatableCurve::new(animated_field!(Transform::rotation), curve),
        );
    }
}

fn add_scale_curve(
    clip: &mut AnimationClip,
    target: AnimationTargetId,
    track: &AnimationTransformTrack,
    rest: &Transform,
) {
    if track.scale.is_empty() {
        return;
    }
    let unity_reference = track
        .reference_scale
        .map_or_else(|| Vec3::from_array(track.scale[0].value), Vec3::from_array);
    let keys = ensure_two_keyframes(
        track.scale.iter().map(|key| {
            let value = Vec3::from_array(key.value);
            let ratio = Vec3::new(
                safe_ratio(value.x, unity_reference.x),
                safe_ratio(value.y, unity_reference.y),
                safe_ratio(value.z, unity_reference.z),
            );
            (key.time, rest.scale * ratio)
        }),
        clip.duration(),
    );
    if let Ok(curve) = AnimatableKeyframeCurve::new(keys) {
        clip.add_curve_to_target(
            target,
            AnimatableCurve::new(animated_field!(Transform::scale), curve),
        );
    }
}

fn ensure_two_keyframes<T: Clone>(
    keyframes: impl Iterator<Item = (f32, T)>,
    duration: f32,
) -> Vec<(f32, T)> {
    let mut keyframes: Vec<_> = keyframes.collect();
    if keyframes.len() == 1 {
        let (time, value) = keyframes[0].clone();
        keyframes.push((duration.max(time + 1.0 / 60.0), value));
    }
    keyframes
}

fn normalized_quat(value: [f32; 4]) -> Quat {
    let value = Quat::from_array(value);
    if value.length_squared() > 1.0e-8 {
        value.normalize()
    } else {
        Quat::IDENTITY
    }
}

fn unity_euler(value: [f32; 3]) -> Quat {
    Quat::from_euler(
        EulerRot::ZXY,
        value[2].to_radians(),
        value[0].to_radians(),
        value[1].to_radians(),
    )
}

fn safe_ratio(value: f32, reference: f32) -> f32 {
    if reference.abs() > 1.0e-6 {
        value / reference
    } else {
        1.0
    }
}

fn drive_native_animations(
    agents: Query<&Agent>,
    mut players: Query<(&mut AnimationPlayer, &mut ActorAnimationDriver)>,
) {
    for (mut player, mut driver) in &mut players {
        let Ok(agent) = agents.get(driver.actor_root) else {
            continue;
        };
        let next = if !agent.path.is_empty() && agent.path_index < agent.path.len() {
            MovementAnimationState::Moving
        } else {
            MovementAnimationState::Idle
        };
        if next == driver.current {
            continue;
        }
        driver.current = next;
        let node = match next {
            MovementAnimationState::Idle => driver.idle,
            MovementAnimationState::Moving => driver.moving,
        };
        player.stop_all().play(node).repeat();
        info!(
            actor = ?driver.actor_root,
            state = ?next,
            "switched native animation state"
        );
    }
}

fn drive_converted_animations(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    simulation: Res<SimulationRuntime>,
    agents: Query<&Agent>,
    mut players: Query<(&mut AnimationPlayer, &mut ConvertedAnimationDriver)>,
    mut audio_cache: ResMut<RoleActionAudioCache>,
    mut procedural_pitches: Option<ResMut<Assets<Pitch>>>,
) {
    let mut audio_cues = Vec::new();
    for (mut player, mut driver) in &mut players {
        let Ok(agent) = agents.get(driver.actor_root) else {
            continue;
        };
        let Some(controller) = presentation.0.controllers.get(&driver.controller) else {
            continue;
        };
        let moving = !agent.path.is_empty() && agent.path_index < agent.path.len();
        let move_speed = if moving {
            (config.0.gameplay.agent_speed_cells_per_second / 5.0).clamp(0.0, 1.0)
        } else {
            0.0
        };
        for layer in &mut driver.layers {
            let _ = layer.runtime.set_float("Move Speed", move_speed);
        }

        if let Some(alive) = simulation.0.actors.get(&agent.id).map(|actor| actor.alive) {
            if let Some(previous) = driver.last_alive
                && alive != previous
            {
                let trigger = if alive { "Revive" } else { "Death" };
                for layer in &mut driver.layers {
                    let _ = layer.runtime.set_trigger(trigger);
                }
            }
            driver.last_alive = Some(alive);
        }
        if let Some(actor) = simulation.0.actors.get(&agent.id) {
            let action = agent_action_animation(&content.0, agent, actor);
            if driver.active_action.as_deref() != action.as_deref() {
                if let Some(previous) = driver.active_action.take() {
                    for layer in &mut driver.layers {
                        let _ = layer.runtime.reset_trigger(&previous);
                    }
                }
                for layer in &mut driver.layers {
                    let _ = layer.runtime.set_boolean("Action", action.is_some());
                }
                if let Some(action) = &action {
                    let variants = content
                        .0
                        .roles
                        .get(&actor.role)
                        .map_or(1, |role| role.action_animation_variants.max(1));
                    let index = deterministic_animation_variant(&agent.id, action, variants);
                    let speed = action_animation_speed(&content.0, &simulation.0, actor);
                    for layer in &mut driver.layers {
                        let _ = layer.runtime.set_trigger(action);
                        let _ = layer
                            .runtime
                            .set_integer("AnimationIndex", i32::from(index));
                        let _ = layer.runtime.set_float("ActionSpeed", speed);
                    }
                }
                driver.active_action = action;
            }
            let carrying = actor.inventory.values().any(|amount| *amount > 0);
            let carry_kind = content
                .0
                .roles
                .get(&actor.role)
                .and_then(|role| role.equipment.as_ref())
                .and_then(|equipment| equipment.carry_animation.as_deref());
            for layer in &mut driver.layers {
                let _ = layer
                    .runtime
                    .set_boolean("CarryWood", carrying && carry_kind == Some("Carry Wood"));
                let _ = layer
                    .runtime
                    .set_boolean("CarryHip", carrying && carry_kind == Some("Carry Hip"));
            }
        }

        let actor_root = driver.actor_root;
        let mut combined = Vec::new();
        let mut restarts = Vec::new();
        for layer in &mut driver.layers {
            let source_selection = layer.runtime.motion_selection(controller).ok().flatten();
            let source_speed = layer
                .runtime
                .state_speed(controller)
                .unwrap_or(1.0)
                .abs()
                .max(f32::EPSILON);
            let source_duration = source_selection.as_ref().map_or(0.0, |selection| {
                animation_selection_duration(selection, &presentation.0) / source_speed
            });
            let normalized_time = if controller.states[layer.runtime.current_state()]
                .motions
                .is_empty()
            {
                1.0
            } else {
                current_normalized_time(&player, layer, &presentation.0)
            };
            let transition = layer
                .runtime
                .evaluate_transitions(controller, normalized_time)
                .ok();
            let transition_playback = layer.runtime.take_transition_playback();
            if matches!(
                transition,
                Some(stream_town_domain::AnimationTransitionOutcome::Exited)
            ) {
                let fallback_state = layer.fallback_state.clone();
                let _ = layer.runtime.enter_state(controller, fallback_state);
            }
            if let Some(stream_town_domain::AnimationTransitionOutcome::Entered(state)) =
                &transition
            {
                info!(
                    actor = ?actor_root,
                    layer = %layer.display_name,
                    state = %state,
                    "translated animation controller entered state"
                );
            }
            if let Some(playback) = transition_playback {
                begin_animation_crossfade(layer, playback, source_duration);
            }

            let Ok(Some(selection)) = layer.runtime.motion_selection(controller) else {
                layer.active.clear();
                layer.applied.clear();
                layer.crossfade = None;
                layer.event_elapsed.clear();
                continue;
            };
            let desired = animation_nodes_for_selection(&selection, &layer.nodes);
            if transition_playback.is_some() {
                for (node, _) in &desired {
                    let offset_seconds =
                        layer
                            .nodes
                            .iter()
                            .find_map(|(clip, candidate)| {
                                (candidate == node).then(|| {
                                    presentation.0.clips.get(clip).map_or(0.0, |clip| {
                                        clip.duration_seconds * layer.state_offset
                                    })
                                })
                            })
                            .unwrap_or(0.0);
                    restarts.push((*node, offset_seconds));
                }
            }
            collect_animation_audio_events(
                &player,
                layer,
                &selection,
                &presentation.0,
                &agent.id,
                &mut audio_cues,
            );
            let state_speed = layer.runtime.state_speed(controller).unwrap_or(1.0);
            let changed = !same_animation_blend(&layer.active, &desired);
            if changed {
                info!(
                    actor = ?actor_root,
                    layer = %layer.display_name,
                    state = %layer.runtime.current_state(),
                    primary = %selection.first.clip,
                    primary_weight = selection.first.weight,
                    secondary = selection.second.as_ref().map(|motion| motion.clip.as_str()),
                    secondary_weight = selection.second.as_ref().map(|motion| motion.weight),
                    "applied translated animation blend"
                );
            }
            let destination: Vec<_> = desired
                .iter()
                .map(|(node, weight)| (*node, *weight, state_speed))
                .collect();
            let applied =
                advance_animation_crossfade(&mut layer.crossfade, &destination, time.delta_secs());
            combined.extend(applied.iter().copied());
            layer.active = desired;
            layer.applied = applied;
        }
        apply_animation_blend(&mut player, &combined, &restarts);
    }
    if let Some(pitches) = procedural_pitches.as_mut() {
        for cue in audio_cues {
            let frequency = procedural_role_action_frequency(&cue.display_name, &cue.clip);
            let source = audio_cache
                .0
                .entry(cue.clip.clone())
                .or_insert_with(|| pitches.add(Pitch::new(frequency, Duration::from_millis(85))))
                .clone();
            commands.spawn((
                Name::new(format!(
                    "Role action audio: {} ({})",
                    cue.display_name, cue.actor
                )),
                AudioPlayer(source),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.08)),
            ));
        }
    }
}

fn collect_animation_audio_events(
    player: &AnimationPlayer,
    layer: &mut ConvertedAnimationLayerDriver,
    selection: &AnimationBlendSelection,
    presentation: &PresentationCatalog,
    actor: &StableId,
    output: &mut Vec<PendingRoleActionAudio>,
) {
    let mut selected = BTreeSet::new();
    for motion in std::iter::once(&selection.first).chain(selection.second.as_ref()) {
        if motion.weight <= f32::EPSILON {
            continue;
        }
        selected.insert(motion.clip.clone());
        let Some(node) = layer.nodes.get(&motion.clip) else {
            continue;
        };
        let Some(active) = player.animation(*node) else {
            layer.event_elapsed.remove(&motion.clip);
            continue;
        };
        let current =
            active.elapsed() + clip_event_offset_seconds(layer, &motion.clip, presentation);
        let previous = layer.event_elapsed.insert(motion.clip.clone(), current);
        let Some(clip) = presentation.clips.get(&motion.clip) else {
            continue;
        };
        for event in &clip.events {
            if event.function_name != "PlayRoleActionAudio" {
                continue;
            }
            let occurrences =
                animation_event_occurrences(event.time, clip.duration_seconds, previous, current);
            for _ in 0..occurrences {
                output.push(PendingRoleActionAudio {
                    actor: actor.clone(),
                    clip: motion.clip.clone(),
                    display_name: clip.display_name.clone(),
                });
            }
        }
    }
    layer
        .event_elapsed
        .retain(|clip, _| selected.contains(clip));
}

fn clip_event_offset_seconds(
    layer: &ConvertedAnimationLayerDriver,
    clip: &StableId,
    presentation: &PresentationCatalog,
) -> f32 {
    presentation
        .clips
        .get(clip)
        .map_or(0.0, |clip| clip.duration_seconds * layer.state_offset)
}

fn animation_event_occurrences(
    event_time: f32,
    duration: f32,
    previous_elapsed: Option<f32>,
    current_elapsed: f32,
) -> u32 {
    if !event_time.is_finite()
        || !duration.is_finite()
        || !current_elapsed.is_finite()
        || event_time < 0.0
        || duration <= f32::EPSILON
        || event_time > duration
    {
        return 0;
    }
    let previous = previous_elapsed
        .filter(|previous| previous.is_finite() && *previous <= current_elapsed)
        .unwrap_or(-f32::EPSILON);
    // The finite, non-negative cycle indices are intentionally quantized from
    // continuous clip time; saturating float-to-integer casts protect corrupt
    // or extremely long-running clocks without changing event boundaries.
    #[allow(clippy::cast_possible_truncation)]
    let first_cycle = (((previous - event_time) / duration).floor() as i64 + 1).max(0);
    #[allow(clippy::cast_possible_truncation)]
    let last_cycle = ((current_elapsed - event_time) / duration).floor() as i64;
    if last_cycle < first_cycle {
        0
    } else {
        u32::try_from(last_cycle - first_cycle + 1).unwrap_or(u32::MAX)
    }
}

fn procedural_role_action_frequency(display_name: &str, clip: &StableId) -> f32 {
    let name = display_name.to_ascii_lowercase();
    if name.contains("bow") {
        493.88
    } else if name.contains("heal") || name.contains("pray") {
        659.25
    } else if name.contains("build") || name.contains("hammer") {
        246.94
    } else if name.contains("mine") || name.contains("mining") {
        185.00
    } else if name.contains("wood") || name.contains("cut") || name.contains("axe") {
        220.00
    } else if name.contains("sword") || name.contains("attack") {
        146.83
    } else {
        let hash = clip.as_str().bytes().fold(2_166_136_261_u32, |hash, byte| {
            hash.wrapping_mul(16_777_619) ^ u32::from(byte)
        });
        let semitone = u8::try_from(hash % 18).expect("modulo 18 fits u8");
        196.0 * 2.0_f32.powf(f32::from(semitone) / 12.0)
    }
}

fn agent_action_animation(
    content: &ContentCatalog,
    agent: &Agent,
    actor: &ActorState,
) -> Option<String> {
    let acting =
        !agent_is_moving(agent) && agent.action_cooldown_seconds > f32::EPSILON && actor.alive;
    if !acting {
        return None;
    }
    match agent.goal {
        AgentGoal::Gather(_)
        | AgentGoal::Construct(_)
        | AgentGoal::Attack(_)
        | AgentGoal::AttackBuilding(_)
        | AgentGoal::Heal(_) => content
            .roles
            .get(&actor.role)
            .map(|role| role.action_animation.clone()),
        AgentGoal::Deposit | AgentGoal::Wander => None,
    }
}

fn agent_is_moving(agent: &Agent) -> bool {
    agent
        .path
        .get(agent.path_index)
        .is_some_and(|next| *next != agent.origin)
}

fn action_animation_speed(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
) -> f32 {
    let action_seconds = actor_archetype(content, actor)
        .and_then(|archetype| archetype.enemy.as_ref())
        .map_or_else(
            || {
                effective_role_stats(content, simulation, actor)
                    .map_or(1.0, |stats| milli_units_as_f32(stats.action_milliseconds))
            },
            |enemy| milli_units_as_f32(enemy.action_milliseconds),
        );
    (3.0 - action_seconds * 2.0).max(1.0)
}

fn deterministic_animation_variant(actor: &StableId, action: &str, variants: u8) -> u8 {
    if variants <= 1 {
        return 0;
    }
    let hash = actor
        .as_str()
        .bytes()
        .chain(action.bytes())
        .fold(2_166_136_261_u32, |hash, byte| {
            hash.wrapping_mul(16_777_619) ^ u32::from(byte)
        });
    u8::try_from(hash % u32::from(variants)).expect("variant modulo fits u8")
}

fn current_normalized_time(
    player: &AnimationPlayer,
    layer: &ConvertedAnimationLayerDriver,
    presentation: &PresentationCatalog,
) -> f32 {
    player
        .playing_animations()
        .filter_map(|(node, animation)| {
            let clip = layer
                .nodes
                .iter()
                .find_map(|(clip, candidate)| (candidate == node).then_some(clip))?;
            let duration = presentation.clips.get(clip)?.duration_seconds;
            (duration > f32::EPSILON).then_some(animation.elapsed() / duration + layer.state_offset)
        })
        .fold(0.0, f32::max)
}

fn animation_nodes_for_selection(
    selection: &AnimationBlendSelection,
    nodes: &BTreeMap<StableId, AnimationNodeIndex>,
) -> Vec<(AnimationNodeIndex, f32)> {
    let mut desired = Vec::with_capacity(2);
    if let Some(node) = nodes.get(&selection.first.clip)
        && selection.first.weight > f32::EPSILON
    {
        desired.push((*node, selection.first.weight));
    }
    if let Some(second) = &selection.second
        && let Some(node) = nodes.get(&second.clip)
        && second.weight > f32::EPSILON
    {
        desired.push((*node, second.weight));
    }
    desired
}

fn begin_animation_crossfade(
    layer: &mut ConvertedAnimationLayerDriver,
    playback: AnimationTransitionPlayback,
    source_duration: f32,
) {
    let duration = if playback.fixed_duration {
        playback.duration
    } else {
        playback.duration * source_duration
    };
    layer.state_offset = playback.destination_offset;
    layer.crossfade = (duration > f32::EPSILON && !layer.applied.is_empty()).then(|| {
        ConvertedAnimationCrossfade {
            source: layer.applied.clone(),
            elapsed: 0.0,
            duration,
        }
    });
}

fn animation_selection_duration(
    selection: &AnimationBlendSelection,
    presentation: &PresentationCatalog,
) -> f32 {
    let (weighted_duration, total_weight) = std::iter::once(&selection.first)
        .chain(selection.second.as_ref())
        .filter_map(|motion| {
            presentation
                .clips
                .get(&motion.clip)
                .map(|clip| (clip.duration_seconds * motion.weight, motion.weight))
        })
        .fold((0.0, 0.0), |(duration, weight), motion| {
            (duration + motion.0, weight + motion.1)
        });
    if total_weight > f32::EPSILON {
        weighted_duration / total_weight
    } else {
        0.0
    }
}

fn advance_animation_crossfade(
    crossfade: &mut Option<ConvertedAnimationCrossfade>,
    destination: &[(AnimationNodeIndex, f32, f32)],
    delta_seconds: f32,
) -> Vec<(AnimationNodeIndex, f32, f32)> {
    let Some(active) = crossfade.as_mut() else {
        return destination.to_vec();
    };
    active.elapsed = (active.elapsed + delta_seconds.max(0.0)).min(active.duration);
    let progress = (active.elapsed / active.duration).clamp(0.0, 1.0);
    let mut output = Vec::with_capacity(active.source.len() + destination.len());
    for &(node, weight, speed) in &active.source {
        merge_animation_weight(&mut output, node, weight * (1.0 - progress), speed);
    }
    for &(node, weight, speed) in destination {
        merge_animation_weight(&mut output, node, weight * progress, speed);
    }
    if active.elapsed >= active.duration {
        *crossfade = None;
    }
    output
}

fn merge_animation_weight(
    output: &mut Vec<(AnimationNodeIndex, f32, f32)>,
    node: AnimationNodeIndex,
    weight: f32,
    speed: f32,
) {
    if weight <= f32::EPSILON {
        return;
    }
    if let Some((_, existing_weight, existing_speed)) = output
        .iter_mut()
        .find(|(candidate, _, _)| *candidate == node)
    {
        let total = *existing_weight + weight;
        *existing_speed = (*existing_speed * *existing_weight + speed * weight) / total;
        *existing_weight = total;
    } else {
        output.push((node, weight, speed));
    }
}

fn apply_animation_blend(
    player: &mut AnimationPlayer,
    desired: &[(AnimationNodeIndex, f32, f32)],
    restarts: &[(AnimationNodeIndex, f32)],
) {
    let playing: Vec<_> = player.playing_animations().map(|(node, _)| *node).collect();
    for node in playing {
        if !desired.iter().any(|(desired, _, _)| *desired == node) {
            player.stop(node);
        }
    }
    for (node, weight, speed) in desired {
        let animation =
            if let Some((_, offset)) = restarts.iter().find(|(restart, _)| restart == node) {
                player.start(*node).set_seek_time(*offset)
            } else {
                player.play(*node)
            };
        animation.repeat().set_weight(*weight).set_speed(*speed);
    }
}

fn same_animation_blend(
    left: &[(AnimationNodeIndex, f32)],
    right: &[(AnimationNodeIndex, f32)],
) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|((left_node, left_weight), (right_node, right_weight))| {
                left_node == right_node && (left_weight - right_weight).abs() <= f32::EPSILON
            })
}

#[allow(clippy::type_complexity)]
fn apply_material_overrides(
    mut commands: Commands,
    specs: Query<&MaterialOverrideSpec>,
    parents: Query<&ChildOf>,
    names: Query<&Name>,
    mut renderers: Query<
        (
            Entity,
            &mut MeshMaterial3d<StandardMaterial>,
            Option<&GltfMeshName>,
            Option<&GltfMaterialName>,
        ),
        Without<MaterialOverrideApplied>,
    >,
) {
    for (entity, mut material, mesh_name, material_name) in &mut renderers {
        let mut ancestor = entity;
        let mut path = Vec::new();
        for _ in 0..64 {
            if let Ok(spec) = specs.get(ancestor) {
                let hierarchy_path = path.iter().rev().cloned().collect::<Vec<_>>().join("/");
                let authored = resolved_renderer_material(
                    spec,
                    &hierarchy_path,
                    mesh_name.map(|name| name.0.as_str()),
                    material_name.map(|name| name.0.as_str()),
                );
                if let Some(authored) = authored {
                    match authored {
                        ResolvedMaterialHandle::Standard(authored) => {
                            material.0 = authored.clone();
                        }
                        ResolvedMaterialHandle::Building(authored) => {
                            commands
                                .entity(entity)
                                .remove::<MeshMaterial3d<StandardMaterial>>()
                                .insert(MeshMaterial3d(authored.clone()));
                        }
                        ResolvedMaterialHandle::Cloud(authored) => {
                            commands
                                .entity(entity)
                                .remove::<MeshMaterial3d<StandardMaterial>>()
                                .insert(MeshMaterial3d(authored.clone()));
                        }
                        ResolvedMaterialHandle::Tree(authored) => {
                            commands
                                .entity(entity)
                                .remove::<MeshMaterial3d<StandardMaterial>>()
                                .insert(MeshMaterial3d(authored.clone()));
                        }
                    }
                }
                commands.entity(entity).insert(MaterialOverrideApplied);
                break;
            }
            if let Ok(name) = names.get(ancestor) {
                path.push(name.as_str().to_owned());
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
    }
}

fn resolved_renderer_material<'a>(
    spec: &'a MaterialOverrideSpec,
    hierarchy_path: &str,
    mesh_name: Option<&str>,
    material_name: Option<&str>,
) -> Option<&'a ResolvedMaterialHandle> {
    // Bevy puts each glTF primitive on a material-bearing child below its named
    // mesh node. Strip that primitive child before comparing the hierarchy to
    // the Unity renderer path, while retaining the mesh name as the stable
    // fallback for Blender-added scene roots.
    let node_path = if mesh_name.is_some() {
        hierarchy_path
            .rsplit_once('/')
            .map_or(hierarchy_path, |(path, _)| path)
    } else {
        hierarchy_path
    };
    let target_name = mesh_name
        .or_else(|| node_path.rsplit('/').next())
        .unwrap_or_default();
    let full_path_matches = |binding: &&ResolvedRendererMaterialBinding| {
        binding.target_path == node_path
            || node_path.ends_with(&format!("/{}", binding.target_path))
            || binding.target_path.ends_with(&format!("/{node_path}"))
    };
    let name_matches = |binding: &&ResolvedRendererMaterialBinding| {
        binding.target_path.rsplit('/').next() == Some(target_name)
    };
    let exact_path = material_name
        .and_then(|name| {
            spec.renderer_materials
                .iter()
                .filter(full_path_matches)
                .find(|binding| binding.materials.contains_key(name))
        })
        .or_else(|| spec.renderer_materials.iter().find(full_path_matches));
    let exact = exact_path.or_else(|| {
        material_name
            .and_then(|name| {
                spec.renderer_materials
                    .iter()
                    .filter(name_matches)
                    .find(|binding| binding.materials.contains_key(name))
            })
            .or_else(|| spec.renderer_materials.iter().find(name_matches))
    });
    material_name
        .and_then(|name| exact.and_then(|binding| binding.materials.get(name)))
        .or_else(|| material_name.and_then(|name| spec.model_materials.get(name)))
        .or_else(|| exact.and_then(|binding| binding.materials.values().next()))
        .or(spec.fallback.as_ref())
}

fn instantiate_building_materials(
    mut commands: Commands,
    simulation: Res<SimulationRuntime>,
    parents: Query<&ChildOf>,
    buildings: Query<&RuntimeBuilding>,
    mut instances: ResMut<BuildingMaterialInstances>,
    mut materials: Option<ResMut<Assets<BuildingMaterial>>>,
    renderers: Query<
        (Entity, &MeshMaterial3d<BuildingMaterial>),
        Without<BuildingMaterialInstanced>,
    >,
) {
    let Some(materials) = materials.as_deref_mut() else {
        return;
    };
    for (entity, source) in &renderers {
        let mut ancestor = entity;
        let mut building = None;
        for _ in 0..64 {
            if let Ok(runtime) = buildings.get(ancestor) {
                building = Some(runtime.id.clone());
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
        let Some(building) = building else {
            commands.entity(entity).insert(BuildingMaterialInstanced);
            continue;
        };
        let handle = if let Some(instance) = instances.0.get(&building) {
            instance.handle.clone()
        } else {
            let Some(mut material) = materials.get(&source.0).cloned() else {
                continue;
            };
            let health = simulation
                .0
                .buildings
                .get(&building)
                .map_or(BUILDING_MAX_HEALTH, |state| state.health);
            let season = simulation.0.season;
            material.extension.parameters.snow_damage.x = building_snow_strength(season);
            material.extension.parameters.snow_damage.y = building_snow_strength(season);
            material.extension.parameters.snow_damage.z = building_damage_value(health);
            let handle = materials.add(material);
            instances.0.insert(
                building,
                BuildingMaterialInstance {
                    handle: handle.clone(),
                    applied_health: health,
                    applied_season: season,
                },
            );
            handle
        };
        commands
            .entity(entity)
            .insert((MeshMaterial3d(handle), BuildingMaterialInstanced));
    }
}

fn sync_building_material_instances(
    simulation: Res<SimulationRuntime>,
    mut instances: ResMut<BuildingMaterialInstances>,
    mut materials: Option<ResMut<Assets<BuildingMaterial>>>,
) {
    let Some(materials) = materials.as_deref_mut() else {
        return;
    };
    let removed: Vec<_> = instances
        .0
        .iter()
        .filter(|(id, _)| !simulation.0.buildings.contains_key(*id))
        .map(|(id, instance)| (id.clone(), instance.handle.id()))
        .collect();
    for (id, handle) in removed {
        materials.remove(handle);
        instances.0.remove(&id);
    }
    for (id, instance) in &mut instances.0 {
        let Some(building) = simulation.0.buildings.get(id) else {
            continue;
        };
        if instance.applied_health == building.health
            && instance.applied_season == simulation.0.season
        {
            continue;
        }
        let Some(mut material) = materials.get_mut(&instance.handle) else {
            continue;
        };
        let snow = building_snow_strength(simulation.0.season);
        material.extension.parameters.snow_damage.x = snow;
        material.extension.parameters.snow_damage.y = snow;
        material.extension.parameters.snow_damage.z = building_damage_value(building.health);
        instance.applied_health = building.health;
        instance.applied_season = simulation.0.season;
    }
}

fn building_damage_value(health: i32) -> f32 {
    f32::from(u16::try_from(health.clamp(0, BUILDING_MAX_HEALTH)).unwrap_or_default())
        / f32::from(u16::try_from(BUILDING_MAX_HEALTH).expect("building health fits u16"))
}

fn camera_controls(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut requests: ResMut<CameraCommandQueue>,
    mut cameras: Query<(&mut Transform, &mut Projection), With<TownCamera>>,
) {
    let Ok((mut transform, mut projection)) = cameras.single_mut() else {
        return;
    };
    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if direction != Vec2::ZERO {
        let speed = 420.0 * time.delta_secs();
        let direction = direction.normalize();
        transform.translation += Vec3::new(direction.x, 0.0, direction.y) * speed;
    }
    let zoom_factor = if keyboard.pressed(KeyCode::KeyQ) {
        1.0 + time.delta_secs()
    } else if keyboard.pressed(KeyCode::KeyE) {
        1.0 - time.delta_secs() * 0.65
    } else {
        1.0
    };
    if let Projection::Orthographic(orthographic) = &mut *projection {
        orthographic.scale = (orthographic.scale * zoom_factor).clamp(0.35, 4.0);
    }
    if let Some(request) = requests.0.pop_front() {
        if request.reset {
            *transform = default_town_camera_transform();
            if let Projection::Orthographic(orthographic) = &mut *projection {
                orthographic.scale = 1.0;
            }
        } else {
            for action in request.actions {
                let amount = i16::try_from(action.amount.clamp(-100, 100)).map_or(0.0, f32::from);
                match action.direction {
                    CameraDirection::Up => transform.translation.z += amount * 12.0,
                    CameraDirection::Down => transform.translation.z -= amount * 12.0,
                    CameraDirection::Left => transform.translation.x -= amount * 12.0,
                    CameraDirection::Right => transform.translation.x += amount * 12.0,
                    CameraDirection::In | CameraDirection::Out => {
                        if let Projection::Orthographic(orthographic) = &mut *projection {
                            let signed = if action.direction == CameraDirection::In {
                                -amount
                            } else {
                                amount
                            };
                            orthographic.scale =
                                (orthographic.scale * 1.12_f32.powf(signed)).clamp(0.35, 4.0);
                        }
                    }
                }
            }
        }
    }
}

fn pet_scene<'a>(archetype: &'a ArchetypeDef, pet: &StableId) -> Option<&'a ArchetypeScene> {
    let suffix = match pet.as_str() {
        "pet:red_panda" => "Pet_RedPanda.fbx",
        "pet:giraffe" => "Pet_TallBoi.fbx",
        "pet:duck" => "Pet_Duck.fbx",
        "pet:butterfly" => "Pet_Butterfly.fbx",
        "pet:fish_god" => "Critter_Fish3.fbx",
        _ => return None,
    };
    archetype
        .scenes
        .iter()
        .find(|scene| scene.source_model.ends_with(suffix))
}

#[allow(clippy::too_many_arguments)]
fn sync_active_pets(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    owners: Query<(&Agent, &Transform), Without<ActivePetVisual>>,
    mut visuals: Query<(Entity, &ActivePetVisual, &mut Transform), Without<Agent>>,
) {
    let desired: BTreeMap<_, _> = simulation
        .0
        .actors
        .values()
        .filter_map(|actor| {
            actor
                .active_pet
                .as_ref()
                .map(|pet| (actor.id.clone(), pet.clone()))
        })
        .collect();
    let owner_positions: BTreeMap<_, _> = owners
        .iter()
        .map(|(agent, transform)| (agent.id.clone(), transform.translation))
        .collect();
    let mut existing = BTreeSet::new();
    for (entity, visual, mut transform) in &mut visuals {
        let key = (visual.owner.clone(), visual.pet.clone());
        existing.insert(key.clone());
        let Some(position) = owner_positions.get(&visual.owner) else {
            commands.entity(entity).despawn();
            continue;
        };
        if desired.get(&visual.owner) == Some(&visual.pet) {
            let offset = Vec3::new(
                config.0.world.cell_size * 0.55,
                0.0,
                config.0.world.cell_size * 0.4,
            );
            transform.translation = *position + offset;
        } else {
            commands.entity(entity).despawn();
        }
    }
    let Some(server) = asset_server.as_deref() else {
        return;
    };
    let Some(archetype) = content
        .0
        .archetypes
        .values()
        .find(|archetype| archetype.source_path.ends_with("Prefabs/Pets/Pet.prefab"))
    else {
        return;
    };
    for (owner, pet) in desired {
        if existing.contains(&(owner.clone(), pet.clone())) {
            continue;
        }
        let Some(owner_position) = owner_positions.get(&owner) else {
            continue;
        };
        let Some(scene) = pet_scene(archetype, &pet)
            .filter(|scene| converted_asset_exists(&asset_root.0, &scene.asset_path))
        else {
            continue;
        };
        let offset = Vec3::new(
            config.0.world.cell_size * 0.55,
            0.0,
            config.0.world.cell_size * 0.4,
        );
        commands.spawn((
            WorldEntity,
            ActivePetVisual { owner, pet },
            WorldAssetRoot(
                server.load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            Transform::from_translation(*owner_position + offset)
                .with_scale(Vec3::splat(config.0.world.cell_size * 0.28)),
        ));
    }
}

fn default_town_camera_transform() -> Transform {
    Transform::from_xyz(360.0, 420.0, 360.0).looking_at(Vec3::ZERO, Vec3::Y)
}

fn apply_agent_commands(
    mut commands: Commands,
    mut queue: ResMut<AgentCommandQueue>,
    config: Res<RuntimeConfig>,
    world: Res<WorldRuntime>,
    mut agents: Query<(
        Entity,
        &mut Agent,
        &mut GridLocation,
        &AgentAnimation,
        &mut Transform,
    )>,
) {
    while let Some(command) = queue.0.pop_front() {
        match command {
            AgentCommand::Teleport { actor, position } => {
                if let Some((_, mut agent, mut location, animation, mut transform)) = agents
                    .iter_mut()
                    .find(|(_, agent, _, _, _)| agent.id == actor)
                {
                    let mut world_position =
                        grid_to_world_on_surface(position, &config.0, &world.generated);
                    if !animation.native {
                        world_position.y += animation.base_scale.y * 0.5;
                    }
                    location.0 = position;
                    agent.spawn = position;
                    agent.origin = position;
                    agent.path.clear();
                    agent.path_index = 0;
                    agent.target = position;
                    agent.goal = AgentGoal::Wander;
                    transform.translation = world_position;
                }
            }
            AgentCommand::Despawn(actor) => {
                if let Some((entity, _, _, _, _)) = agents
                    .iter_mut()
                    .find(|(_, agent, _, _, _)| agent.id == actor)
                {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn apply_building_commands(
    mut commands: Commands,
    mut queue: ResMut<BuildingCommandQueue>,
    buildings: Query<(Entity, &RuntimeBuilding)>,
) {
    while let Some(command) = queue.0.pop_front() {
        match command {
            BuildingRuntimeCommand::Despawn(building) => {
                if let Some((entity, _)) =
                    buildings.iter().find(|(_, runtime)| runtime.id == building)
                {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn sync_building_placers(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    world: Res<WorldRuntime>,
    placers: Res<BuildingPlacers>,
    render: Res<RenderAssets>,
    mut visuals: Query<(
        Entity,
        &BuildingPlacementVisual,
        &mut Transform,
        &mut MeshMaterial3d<StandardMaterial>,
    )>,
) {
    for (entity, visual, mut transform, mut material) in &mut visuals {
        let Some(placement) = placers.0.get(&visual.owner) else {
            commands.entity(entity).despawn();
            continue;
        };
        let Some(definition) = content.0.buildings.get(&placement.building) else {
            commands.entity(entity).despawn();
            continue;
        };
        update_placer_visual(
            &config.0,
            &world.generated,
            &render,
            placement,
            definition,
            &mut transform,
            &mut material,
        );
    }
    for (owner, placement) in &placers.0 {
        if visuals
            .iter()
            .any(|(_, visual, _, _)| visual.owner == *owner)
        {
            continue;
        }
        let Some(definition) = content.0.buildings.get(&placement.building) else {
            continue;
        };
        let mut transform = Transform::default();
        let mut material = MeshMaterial3d(render.placement_valid.clone());
        update_placer_visual(
            &config.0,
            &world.generated,
            &render,
            placement,
            definition,
            &mut transform,
            &mut material,
        );
        commands.spawn((
            WorldEntity,
            BuildingPlacementVisual {
                owner: owner.clone(),
            },
            Mesh3d(render.cube.clone()),
            material,
            transform,
        ));
    }
}

fn update_placer_visual(
    config: &GameConfig,
    world: &GeneratedWorld,
    render: &RenderAssets,
    placement: &BuildingPlacement,
    definition: &BuildingDef,
    transform: &mut Transform,
    material: &mut MeshMaterial3d<StandardMaterial>,
) {
    let effective = rotated_footprint(definition.footprint, placement.rotation_quarter_turns);
    let centre = GridPos {
        x: placement.position.x.saturating_add(effective[0] / 2),
        z: placement.position.z.saturating_add(effective[1] / 2),
    };
    let size = Vec3::new(
        f32::from(definition.footprint[0]) * config.world.cell_size * 0.9,
        config.world.cell_size * 0.3,
        f32::from(definition.footprint[1]) * config.world.cell_size * 0.9,
    );
    transform.translation =
        grid_to_world_on_surface(centre, config, world) + Vec3::Y * size.y * 0.5;
    transform.scale = size;
    transform.rotation = quarter_turn_rotation(placement.rotation_quarter_turns);
    material.0 = if building_site_is_available(world, placement.position, effective) {
        render.placement_valid.clone()
    } else {
        render.placement_invalid.clone()
    };
}

fn select_grid_cell(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<TownCamera>>,
    spatial: Option<SpatialQuery>,
    config: Res<RuntimeConfig>,
    world: Res<WorldRuntime>,
    mut selected: ResMut<SelectedCell>,
    mut markers: Query<(&mut Transform, &mut Visibility), With<SelectionMarker>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(spatial) = spatial else {
        return;
    };
    let (Ok(window), Ok((camera, camera_transform))) = (windows.single(), cameras.single()) else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor) else {
        return;
    };
    let Some(hit) = spatial.cast_ray(
        ray.origin,
        ray.direction,
        2_000.0,
        true,
        &SpatialQueryFilter::default(),
    ) else {
        return;
    };
    let world_position = ray.get_point(hit.distance);
    let Some(cell) = world_to_grid(world_position, &config.0) else {
        return;
    };
    selected.0 = Some(cell);
    if let Ok((mut transform, mut visibility)) = markers.single_mut() {
        let marker_position = grid_to_world_on_surface(cell, &config.0, &world.generated);
        transform.translation = marker_position + Vec3::Y * 0.12;
        *visibility = Visibility::Visible;
    }
}

fn game_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut injected: ResMut<InjectedCommands>,
    mut injected_debug_commands: Local<bool>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
    if keyboard.just_pressed(KeyCode::KeyJ) {
        injected.0.push_back(PendingChatCommand {
            actor_id: StableId::new("twitch:debug_viewer").expect("static ID"),
            login_name: "debug_viewer".to_owned(),
            display_name: "debug_viewer".to_owned(),
            command: "!join".parse().expect("static chat command"),
            is_broadcaster: true,
            is_moderator: true,
            is_subscriber: true,
            origin: CommandOrigin::LocalDebug,
        });
    }
    if !*injected_debug_commands {
        *injected_debug_commands = true;
        if let Some(commands) = std::env::var_os("STREAM_TOWN_DEBUG_COMMANDS") {
            for command in commands.to_string_lossy().split(';') {
                match command.trim().parse() {
                    Ok(command) => injected.0.push_back(PendingChatCommand {
                        actor_id: StableId::new("twitch:debug_viewer").expect("static ID"),
                        login_name: "debug_viewer".to_owned(),
                        display_name: "debug_viewer".to_owned(),
                        command,
                        is_broadcaster: true,
                        is_moderator: true,
                        is_subscriber: true,
                        origin: CommandOrigin::LocalDebug,
                    }),
                    Err(error) => warn!(command, %error, "ignored invalid debug command"),
                }
            }
        }
    }
}

fn generate_connect_code() -> String {
    let subsecond = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.subsec_nanos());
    format!("{:06}", 100_000 + subsecond % 900_000)
}

fn start_twitch_transport(config: Res<RuntimeConfig>, mut connection: ResMut<TwitchConnection>) {
    if !config.0.twitch.enabled {
        connection.status = TwitchStatus::Disabled;
        return;
    }
    connection.broadcaster_authorized = !config.0.twitch.require_broadcaster_connect;
    connection.status = TwitchStatus::Authorizing;
    match TwitchTransport::start(config.0.twitch.clone()) {
        Ok(transport) => connection.transport = Some(transport),
        Err(error) => connection.status = TwitchStatus::Error(error.to_string()),
    }
}

fn twitch_connection_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<RuntimeConfig>,
    mut connection: ResMut<TwitchConnection>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        if let Some(transport) = connection.transport.take() {
            let _ = transport.send(TwitchControl::Disconnect);
        }
        connection.status = TwitchStatus::Disconnected;
    } else if keyboard.just_pressed(KeyCode::F2) && config.0.twitch.enabled {
        connection.transport = None;
        connection.broadcaster_authorized = !config.0.twitch.require_broadcaster_connect;
        connection.connect_code = generate_connect_code();
        connection.status = TwitchStatus::Authorizing;
        match TwitchTransport::start(config.0.twitch.clone()) {
            Ok(transport) => connection.transport = Some(transport),
            Err(error) => connection.status = TwitchStatus::Error(error.to_string()),
        }
    }
}

fn poll_twitch_transport(
    mut connection: ResMut<TwitchConnection>,
    mut injected: ResMut<InjectedCommands>,
) {
    let events: Vec<_> = connection
        .transport
        .as_ref()
        .into_iter()
        .flat_map(|transport| std::iter::from_fn(|| transport.try_recv()))
        .collect();
    for event in events {
        handle_twitch_event(event, &mut connection, &mut injected);
    }
}

fn handle_twitch_event(
    event: TwitchEvent,
    connection: &mut TwitchConnection,
    injected: &mut InjectedCommands,
) {
    match event {
        TwitchEvent::Status(status) => {
            match &status {
                TwitchStatus::Connected => info!("Twitch IRC connected"),
                TwitchStatus::Error(error) => error!(%error, "Twitch transport error"),
                _ => info!(?status, "Twitch connection state changed"),
            }
            connection.status = status;
        }
        TwitchEvent::Chat(message) => {
            if !connection.broadcaster_authorized {
                let mut parts = message.message.split_whitespace();
                let is_connect = parts
                    .next()
                    .is_some_and(|part| part.eq_ignore_ascii_case("!connect"));
                let valid_code = parts
                    .next()
                    .is_some_and(|code| code == connection.connect_code);
                if message.is_broadcaster && is_connect && valid_code {
                    connection.broadcaster_authorized = true;
                    info!(broadcaster = %message.login, "Twitch broadcaster authorized this session");
                    if let Some(transport) = &connection.transport {
                        let _ = transport.send(TwitchControl::SendMessage(
                            "Stream Town chat commands are now enabled.".to_owned(),
                        ));
                    }
                }
                return;
            }
            if message.custom_reward_id.as_deref() == Some(FISH_GOD_REWARD_ID) {
                injected.0.push_back(PendingChatCommand {
                    actor_id: message.actor_id,
                    login_name: message.login,
                    display_name: message.display_name,
                    command: ChatCommand::Praise,
                    is_broadcaster: message.is_broadcaster,
                    is_moderator: message.is_moderator,
                    is_subscriber: message.is_subscriber,
                    origin: CommandOrigin::Twitch,
                });
                return;
            }
            match message.message.parse::<ChatCommand>() {
                Ok(command) => injected.0.push_back(PendingChatCommand {
                    actor_id: message.actor_id,
                    login_name: message.login,
                    display_name: message.display_name,
                    command,
                    is_broadcaster: message.is_broadcaster,
                    is_moderator: message.is_moderator,
                    is_subscriber: message.is_subscriber,
                    origin: CommandOrigin::Twitch,
                }),
                Err(parse_error) => {
                    debug!(user = %message.login, %parse_error, "ignored invalid Twitch command");
                }
            }
        }
    }
}

fn save_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    save: Res<SaveRuntime>,
    world: Res<WorldRuntime>,
    stats: Res<SessionStats>,
    simulation: Res<SimulationRuntime>,
    agents: Query<(&Agent, &GridLocation)>,
) {
    if !keyboard.just_pressed(KeyCode::F5) {
        return;
    }
    let snapshot = snapshot_world(&world, &stats, &simulation, &agents);
    match save.store.write(&snapshot) {
        Ok(()) => info!(path = %save.store.path().display(), "native save written"),
        Err(error) => error!(%error, "native save failed"),
    }
}

fn load_input(
    mut ecs: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    save: Res<SaveRuntime>,
    mut world: ResMut<WorldRuntime>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    mut load_render: LoadRenderParams,
    mut placers: ResMut<BuildingPlacers>,
    mut stats: ResMut<SessionStats>,
    mut simulation: ResMut<SimulationRuntime>,
    mut agents: Query<(
        Entity,
        &mut Agent,
        &mut GridLocation,
        &AgentAnimation,
        &mut Transform,
    )>,
    runtime_buildings: Query<(Entity, &RuntimeBuilding), Without<TownHall>>,
    enemy_camps: Query<(Entity, &EnemyCamp)>,
    mut automatic_complete: Local<bool>,
) {
    let automatic = !*automatic_complete && std::env::var_os("STREAM_TOWN_AUTO_LOAD").is_some();
    if !keyboard.just_pressed(KeyCode::F9) && !automatic {
        return;
    }
    *automatic_complete = true;
    let mut snapshot = match save.store.load() {
        Ok(snapshot) => snapshot,
        Err(error) => {
            error!(%error, "native load failed");
            return;
        }
    };
    placers.0.clear();
    if snapshot.world_seed != world.generated.seed
        || snapshot.generator_version != world.generated.generator_version
        || snapshot.world_hash != world.generated.deterministic_hash
    {
        error!(
            saved_seed = snapshot.world_seed,
            runtime_seed = world.generated.seed,
            "native save world identity does not match the loaded world"
        );
        return;
    }

    let mut restored_world = generate_world_with_content(&config.0.world, &content.0);
    if !snapshot.resource_nodes.is_empty() {
        for resource in &mut restored_world.resources {
            if let Some(remaining) = snapshot.resource_nodes.get(&resource.id) {
                resource.amount = *remaining;
            }
        }
    }
    let terrain_replacement = if let Some(meshes) = load_render.meshes.as_mut() {
        let mesh = match snapshot.legacy_terrain_mesh.as_ref() {
            Some(saved) => match retained_terrain_mesh(saved) {
                Ok(mesh) => mesh,
                Err(error) => {
                    error!(%error, "native save retained terrain could not be reconstructed");
                    return;
                }
            },
            None => generated_terrain_mesh(&restored_world, &config.0),
        };
        let Some(collider) = Collider::trimesh_from_mesh(&mesh) else {
            error!("native save terrain does not produce a valid triangle collider");
            return;
        };
        Some((meshes.add(mesh), collider))
    } else {
        if snapshot.legacy_terrain_mesh.is_some() {
            error!("native save contains retained terrain but rendering assets are unavailable");
            return;
        }
        None
    };
    ensure_town_hall_state(&content.0, &config.0, &mut snapshot.simulation);
    for (entity, building) in &runtime_buildings {
        debug!(building = %building.id, "despawning runtime building before native load");
        ecs.entity(entity).despawn();
    }
    for (entity, camp) in &enemy_camps {
        debug!(camp = %camp.id, "despawning enemy camp before native load");
        ecs.entity(entity).despawn();
    }
    for saved in snapshot.simulation.buildings.values() {
        let Some((building_id, building)) = content
            .0
            .buildings
            .iter()
            .find(|(_, building)| building.archetype == saved.archetype)
        else {
            error!(
                building = %saved.id,
                archetype = %saved.archetype,
                "native save references an unknown building archetype"
            );
            return;
        };
        let footprint = rotated_footprint(building.footprint, saved.rotation_quarter_turns);
        let Some(region) = building_region(saved.position, footprint, &restored_world) else {
            error!(building = %saved.id, "native save building lies outside the world");
            return;
        };
        if let Err(error) = restored_world.navigation.set_blocked(region, true) {
            error!(building = %saved.id, %error, "native save building could not update navigation");
            return;
        }
        if saved.id.as_str() == "building:townhall" {
            continue;
        }
        spawn_runtime_building(
            &mut ecs,
            &config.0,
            &restored_world,
            &load_render.presentation.0,
            load_render.asset_server.as_deref(),
            &load_render.asset_root.0,
            &load_render.render,
            saved,
            building,
            &content.0.archetypes[&building.archetype],
            saved.position,
            building.footprint,
            building_age(&content.0, &snapshot.simulation, building_id),
        );
    }
    for camp in snapshot.simulation.enemy_camps.values() {
        let Some(archetype) = content.0.archetypes.get(&camp.archetype) else {
            error!(camp = %camp.id, archetype = %camp.archetype, "native save references an unknown enemy camp");
            return;
        };
        let Some(region) = building_region(camp.position, archetype.footprint, &restored_world)
        else {
            error!(camp = %camp.id, "native save enemy camp lies outside the world");
            return;
        };
        if let Err(error) = restored_world.navigation.set_blocked(region, true) {
            error!(camp = %camp.id, %error, "native save enemy camp could not update navigation");
            return;
        }
        spawn_enemy_camp(
            &mut ecs,
            &config.0,
            &restored_world,
            &load_render.presentation.0,
            load_render.asset_server.as_deref(),
            &load_render.asset_root.0,
            &load_render.render,
            &camp.id,
            archetype,
            camp.position,
        );
    }
    if let Some((mesh, collider)) = terrain_replacement {
        for entity in &load_render.terrain_surfaces {
            ecs.entity(entity).despawn();
        }
        ecs.spawn((
            WorldEntity,
            TerrainSurface,
            Mesh3d(mesh),
            MeshMaterial3d(load_render.render.ground.clone()),
            collider,
            RigidBody::Static,
        ));
    }
    world.generated = restored_world;
    world
        .legacy_terrain_mesh
        .clone_from(&snapshot.legacy_terrain_mesh);
    world
        .legacy_migration
        .clone_from(&snapshot.legacy_migration);

    let saved_by_id: BTreeMap<StableId, SavedActor> = snapshot
        .actors
        .iter()
        .filter(|actor| matches!(actor.kind, ActorKind::Player | ActorKind::Enemy))
        .cloned()
        .map(|actor| (actor.id.clone(), actor))
        .collect();
    let mut restored_ids = BTreeSet::new();
    for (entity, mut agent, mut location, animation, mut transform) in &mut agents {
        let Some(saved) = saved_by_id.get(&agent.id) else {
            ecs.entity(entity).despawn();
            continue;
        };
        let position =
            nearest_walkable(&world.generated, saved.grid_position).unwrap_or(saved.grid_position);
        let mut world_position = grid_to_world_on_surface(position, &config.0, &world.generated);
        if !animation.native {
            world_position.y += animation.base_scale.y * 0.5;
        }
        agent.kind = saved.kind.clone();
        agent.archetype = saved.archetype.clone();
        agent.goal = AgentGoal::Wander;
        agent.spawn = position;
        agent.origin = position;
        agent.path.clear();
        agent.path_index = 0;
        agent.target = mirrored_target(&world.generated, position);
        agent.action_cooldown_seconds = 0.0;
        location.0 = position;
        transform.translation = world_position;
        restored_ids.insert(saved.id.clone());
    }

    for saved in saved_by_id.values() {
        if restored_ids.contains(&saved.id) {
            continue;
        }
        let position =
            nearest_walkable(&world.generated, saved.grid_position).unwrap_or(saved.grid_position);
        let world_position = grid_to_world_on_surface(position, &config.0, &world.generated);
        let base_scale = Vec3::new(
            config.0.world.cell_size * 0.3,
            config.0.world.cell_size * 0.55,
            config.0.world.cell_size * 0.3,
        );
        ecs.spawn((
            WorldEntity,
            GridLocation(position),
            Agent {
                id: saved.id.clone(),
                kind: saved.kind.clone(),
                archetype: saved.archetype.clone(),
                goal: AgentGoal::Wander,
                spawn: position,
                origin: position,
                path: Vec::new(),
                path_index: 0,
                target: mirrored_target(&world.generated, position),
                action_cooldown_seconds: 0.0,
                health_regen_accumulator: 0.0,
            },
            AgentAnimation {
                base_scale,
                ..default()
            },
            Mesh3d(load_render.render.actor_lod.clone()),
            MeshMaterial3d(actor_material(&load_render.render, &saved.kind, false)),
            Transform::from_xyz(
                world_position.x,
                world_position.y + base_scale.y * 0.5,
                world_position.z,
            )
            .with_scale(base_scale),
        ));
    }
    stats.elapsed_seconds = Duration::from_secs(snapshot.elapsed_seconds).as_secs_f64();
    stats.paths_completed = 0;
    simulation.0 = snapshot.simulation;
    info!(
        path = %save.store.path().display(),
        retained_terrain = snapshot.legacy_terrain_mesh.is_some(),
        terrain_vertices = snapshot
            .legacy_terrain_mesh
            .as_ref()
            .map_or(0, |mesh| mesh.vertices.len()),
        "native save loaded and applied"
    );
}

fn capture_screenshot(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut elapsed: Local<f32>,
    mut automatic_complete: Local<bool>,
    mut counter: Local<u32>,
    mut exit_delay: Local<Option<f32>>,
    mut exit: MessageWriter<AppExit>,
) {
    if let Some(remaining) = exit_delay.as_mut() {
        *remaining -= time.delta_secs();
        if *remaining <= 0.0 {
            exit.write(AppExit::Success);
            *exit_delay = None;
        }
    }
    *elapsed += time.delta_secs();
    let automatic_delay = std::env::var("STREAM_TOWN_SCREENSHOT_DELAY")
        .ok()
        .and_then(|value| value.parse::<f32>().ok())
        .filter(|value| value.is_finite() && *value >= 0.25)
        .unwrap_or(3.0);
    let automatic_path = if !*automatic_complete && *elapsed >= automatic_delay {
        std::env::var_os("STREAM_TOWN_SCREENSHOT").map(PathBuf::from)
    } else {
        None
    };
    let path = if keyboard.just_pressed(KeyCode::F12) {
        let directory = PathBuf::from(".stream-town").join("screenshots");
        if let Err(error) = std::fs::create_dir_all(&directory) {
            error!(%error, path = %directory.display(), "failed to create screenshot directory");
            return;
        }
        let path = directory.join(format!("stream-town-{:04}.png", *counter));
        *counter += 1;
        Some(path)
    } else {
        automatic_path
    };
    let Some(path) = path else {
        return;
    };
    *automatic_complete = true;
    info!(path = %path.display(), "capturing frame");
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(path));
    if std::env::var_os("STREAM_TOWN_EXIT_AFTER_SCREENSHOT").is_some() {
        *exit_delay = Some(1.0);
    }
}

fn report_frame_time_gate(
    time: Res<Time>,
    diagnostics: Option<ResMut<DiagnosticsStore>>,
    mut warmed_up: Local<bool>,
    mut reported: Local<bool>,
) {
    if *reported || std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_none() {
        return;
    }
    let Some(mut diagnostics) = diagnostics else {
        return;
    };
    let warmup = std::env::var("STREAM_TOWN_FRAME_TIME_WARMUP")
        .ok()
        .and_then(|value| value.parse::<f32>().ok())
        .filter(|value| value.is_finite() && *value >= 1.0)
        .unwrap_or(10.0);
    let sample_seconds = std::env::var("STREAM_TOWN_FRAME_TIME_SAMPLE_SECONDS")
        .ok()
        .and_then(|value| value.parse::<f32>().ok())
        .filter(|value| value.is_finite() && *value >= 1.0)
        .unwrap_or(10.0);
    if !*warmed_up && time.elapsed_secs() >= warmup {
        if let Some(frame_time) = diagnostics.get_mut(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
            frame_time.clear_history();
        }
        *warmed_up = true;
        return;
    }
    if !*warmed_up || time.elapsed_secs() < warmup + sample_seconds {
        return;
    }
    let Some(frame_time) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) else {
        return;
    };
    let mut values: Vec<_> = frame_time
        .values()
        .copied()
        .filter(|value| value.is_finite())
        .collect();
    if values.is_empty() {
        return;
    }
    values.sort_by(f64::total_cmp);
    let p95_index = (values.len() * 95).div_ceil(100).saturating_sub(1);
    let p95_ms = values[p95_index];
    let sample_count = u32::try_from(values.len()).unwrap_or(u32::MAX);
    let average_ms = values.iter().sum::<f64>() / f64::from(sample_count);
    warn!(
        samples = values.len(),
        average_ms,
        p95_ms,
        budget_ms = 16.7,
        passed = p95_ms < 16.7,
        "steady-state frame-time gate"
    );
    *reported = true;
}

fn mirrored_target(world: &GeneratedWorld, position: GridPos) -> GridPos {
    let desired = GridPos {
        x: world.navigation.width() - 1 - position.x,
        z: world.navigation.height() - 1 - position.z,
    };
    nearest_walkable(world, desired).unwrap_or(position)
}

fn prefixed_id(requested: &StableId, prefix: &str) -> Option<StableId> {
    if requested.as_str().starts_with(prefix) {
        Some(requested.clone())
    } else {
        StableId::new(format!("{prefix}{}", requested.as_str())).ok()
    }
}

fn normalized_content_name(value: &str) -> String {
    value
        .chars()
        .filter_map(|character| {
            if character.is_ascii_alphanumeric() {
                Some(character.to_ascii_lowercase())
            } else if character == ' ' || character == '-' || character == '_' {
                Some('_')
            } else {
                None
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_owned()
}

fn resolve_technology_id(content: &ContentCatalog, requested: &StableId) -> Option<StableId> {
    if content.technology.nodes.contains_key(requested) {
        return Some(requested.clone());
    }
    let requested_name = normalized_content_name(requested.as_str());
    content
        .technology
        .nodes
        .iter()
        .find(|(_, technology)| normalized_content_name(&technology.display_name) == requested_name)
        .map(|(id, _)| id.clone())
}

fn eligible_technology_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> Vec<StableId> {
    content
        .technology
        .nodes
        .iter()
        .filter(|(id, node)| {
            !node.unavailable
                && !simulation.unlocked_technology.contains(*id)
                && !simulation
                    .active_goals
                    .iter()
                    .any(|goal| goal.technology == **id)
                && simulation
                    .active_vote
                    .as_ref()
                    .is_none_or(|vote| vote.technology != **id)
                && node
                    .prerequisites
                    .iter()
                    .all(|required| simulation.unlocked_technology.contains(required))
        })
        .map(|(id, _)| id.clone())
        .collect()
}

fn unlock_reachable_technologies(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    age_one_only: bool,
) -> usize {
    let mut unlocked = 0;
    loop {
        let available = eligible_technology_ids(content, simulation)
            .into_iter()
            .filter(|id| {
                !age_one_only
                    || content.technology.nodes[id]
                        .age
                        .trim()
                        .eq_ignore_ascii_case("Age 1")
            })
            .collect::<Vec<_>>();
        if available.is_empty() {
            break;
        }
        unlocked += available.len();
        simulation.unlocked_technology.extend(available);
    }
    unlocked
}

fn maximum_building_level(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
) -> u16 {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .filter_map(|technology| technology.building_level_caps.get(building).copied())
        .max()
        .unwrap_or(1)
}

fn building_is_unlocked(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
) -> bool {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .any(|technology| technology.unlocked_buildings.contains(building))
}

fn technology_stat_boost_percent(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    role: &StableId,
    stat: &StableId,
) -> i32 {
    if role.as_str() == "role:enemy" {
        return 0;
    }
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .fold(0_i32, |total, technology| {
            total
                .saturating_add(
                    technology
                        .global_stat_boost_percent
                        .get(stat)
                        .copied()
                        .unwrap_or_default(),
                )
                .saturating_add(
                    technology
                        .role_stat_boost_percent
                        .get(role)
                        .and_then(|stats| stats.get(stat))
                        .copied()
                        .unwrap_or_default(),
                )
        })
}

fn building_cost_reduction_percent(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
) -> i32 {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .fold(0_i32, |total, technology| {
            total
                .saturating_add(technology.global_building_cost_reduction_percent)
                .saturating_add(
                    technology
                        .building_cost_reduction_percent
                        .get(building)
                        .copied()
                        .unwrap_or_default(),
                )
        })
}

fn storage_boost_percent(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    resource: &StableId,
) -> i32 {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .filter_map(|technology| technology.storage_boost_percent.get(resource).copied())
        .fold(0_i32, i32::saturating_add)
}

fn building_age(content: &ContentCatalog, simulation: &WorldSimulation, building: &StableId) -> u8 {
    if simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .any(|technology| technology.aged_buildings.contains(building))
    {
        2
    } else {
        1
    }
}

fn percentage_adjusted(base: u32, percent: i32) -> u32 {
    let base = i64::from(base);
    let adjusted = base.saturating_add(base.saturating_mul(i64::from(percent)) / 100);
    u32::try_from(adjusted.clamp(0, i64::from(u32::MAX))).unwrap_or(u32::MAX)
}

fn percentage_reduced(base: u32, reduction_percent: i32) -> u32 {
    percentage_adjusted(base, reduction_percent.saturating_neg())
}

fn percentage_adjusted_i64(base: i64, percent: i32) -> i64 {
    base.saturating_add(base.saturating_mul(i64::from(percent)) / 100)
}

fn milli_units_as_f32(value: u32) -> f32 {
    f32::from(u16::try_from(value.min(u32::from(u16::MAX))).expect("clamped milli-unit fits u16"))
        / 1_000.0
}

fn resource_storage_capacity(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    resource: &StableId,
) -> u32 {
    let Some(base) = config
        .gameplay
        .base_town_resource_capacity
        .get(resource)
        .copied()
    else {
        return u32::MAX;
    };
    let boost = storage_boost_percent(content, simulation, resource);
    simulation
        .buildings
        .values()
        .filter(|building| building.complete)
        .filter_map(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            Some((definition, building.level))
        })
        .flat_map(|(definition, level)| {
            definition
                .storage
                .iter()
                .filter(move |storage| storage.resource == *resource)
                .map(move |storage| {
                    let amount = if level <= 1 {
                        storage.base_amount
                    } else {
                        let scaled_level = u64::from(level)
                            .saturating_mul(u64::from(storage.level_multiplier_per_thousand))
                            / 1_000;
                        u32::try_from(
                            u64::from(storage.increment_amount).saturating_mul(scaled_level),
                        )
                        .unwrap_or(u32::MAX)
                    };
                    percentage_adjusted(amount, boost)
                })
        })
        .fold(base, u32::saturating_add)
}

fn apply_passive_building_income(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    delta: Duration,
) {
    let mut rates = BTreeMap::<(StableId, StableId), u64>::new();
    for building in simulation
        .buildings
        .values()
        .filter(|building| building.complete)
    {
        let Some(definition) = building_def_for_archetype(content, &building.archetype) else {
            continue;
        };
        let completed_levels = u64::from(building.level.saturating_sub(1));
        for income in &definition.passive_resources {
            let rate = u64::from(income.base_milli_per_second).saturating_add(
                u64::from(income.increment_milli_per_level)
                    .saturating_mul(completed_levels)
                    .saturating_mul(u64::from(income.level_event_repetitions)),
            );
            let entry = rates
                .entry((building.id.clone(), income.resource.clone()))
                .or_default();
            *entry = entry.saturating_add(rate);
        }
    }

    let mut active = BTreeMap::<StableId, BTreeSet<StableId>>::new();
    for (building, resource) in rates.keys() {
        active
            .entry(building.clone())
            .or_default()
            .insert(resource.clone());
    }
    simulation
        .passive_resource_accumulators
        .retain(|building, resources| {
            let Some(active_resources) = active.get(building) else {
                return false;
            };
            resources.retain(|resource, _| active_resources.contains(resource));
            !resources.is_empty()
        });

    let delta_nanos = delta.as_nanos();
    if delta_nanos == 0 {
        return;
    }
    for ((building, resource), rate) in rates {
        let previous = simulation
            .passive_resource_accumulators
            .entry(building)
            .or_default()
            .entry(resource.clone())
            .or_default();
        let accumulated =
            u128::from(*previous).saturating_add(u128::from(rate).saturating_mul(delta_nanos));
        // Unity uses a strict `> 1` threshold, so exactly one accumulated unit waits
        // until the next positive update rather than being emitted immediately.
        let generated = if accumulated > PASSIVE_RESOURCE_FIXED_POINT_DENOMINATOR {
            accumulated / PASSIVE_RESOURCE_FIXED_POINT_DENOMINATOR
        } else {
            0
        };
        *previous = u64::try_from(
            accumulated
                .saturating_sub(generated.saturating_mul(PASSIVE_RESOURCE_FIXED_POINT_DENOMINATOR)),
        )
        .expect("passive resource remainder fits u64");
        if generated == 0 {
            continue;
        }
        let generated = u32::try_from(generated).unwrap_or(u32::MAX);
        let capacity = resource_storage_capacity(config, content, simulation, &resource);
        let current = simulation
            .town_resources
            .get(&resource)
            .copied()
            .unwrap_or_default();
        simulation.town_resources.insert(
            resource.clone(),
            current.saturating_add(generated).min(capacity),
        );
        let _ = simulation.record_objective_event(
            &content.objectives,
            &ObjectiveEvent::ResourceGained {
                resource,
                amount: generated,
            },
        );
    }
}

fn building_construction_cost(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building_id: &StableId,
    building: &BuildingDef,
) -> BTreeMap<StableId, u32> {
    let reduction = building_cost_reduction_percent(content, simulation, building_id);
    building
        .cost
        .iter()
        .map(|(resource, cost)| (resource.clone(), percentage_reduced(*cost, reduction)))
        .collect()
}

fn building_upgrade_cost(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building_id: &StableId,
    building: &BuildingDef,
    current_level: u16,
) -> BTreeMap<StableId, u32> {
    let level_squared = u64::from(current_level).saturating_pow(2);
    let reduction = building_cost_reduction_percent(content, simulation, building_id);
    building
        .level_cost
        .iter()
        .map(|(resource, base)| {
            let scaled = u64::from(*base)
                .saturating_mul(level_squared)
                .saturating_mul(u64::from(building.level_cost_multiplier_per_thousand))
                / 1_000;
            (
                resource.clone(),
                percentage_reduced(u32::try_from(scaled).unwrap_or(u32::MAX), reduction),
            )
        })
        .collect()
}

fn town_event_from_id(requested: &StableId) -> Option<TownEvent> {
    match requested.as_str().trim_start_matches("event:") {
        "festival" => Some(TownEvent::Festival),
        "raid" | "enemy_raid" => Some(TownEvent::EnemyRaid),
        "harsh_weather" | "weather" => Some(TownEvent::HarshWeather),
        "resource_boom" | "wood_boom" => Some(TownEvent::ResourceBoom(
            StableId::new("resource:wood").expect("static ID"),
        )),
        "fish_god" | "fishgod" => Some(TownEvent::FishGod),
        _ => None,
    }
}

fn building_region(
    position: GridPos,
    footprint: [u16; 2],
    world: &GeneratedWorld,
) -> Option<stream_town_domain::DirtyRegion> {
    let max_x = position.x.checked_add(footprint[0].checked_sub(1)?)?;
    let max_z = position.z.checked_add(footprint[1].checked_sub(1)?)?;
    if max_x >= world.navigation.width() || max_z >= world.navigation.height() {
        return None;
    }
    Some(stream_town_domain::DirtyRegion {
        min: position,
        max: GridPos { x: max_x, z: max_z },
    })
}

fn rotated_footprint(footprint: [u16; 2], rotation_quarter_turns: i32) -> [u16; 2] {
    if rotation_quarter_turns.rem_euclid(2) == 0 {
        footprint
    } else {
        [footprint[1], footprint[0]]
    }
}

fn foliage_clearance_regions(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
) -> Vec<stream_town_domain::DirtyRegion> {
    let building_regions = simulation.buildings.values().filter_map(|building| {
        let definition = building_def_for_archetype(content, &building.archetype)?;
        building_region(
            building.position,
            rotated_footprint(definition.footprint, building.rotation_quarter_turns),
            world,
        )
    });
    let camp_regions = simulation.enemy_camps.values().filter_map(|camp| {
        let archetype = content.archetypes.get(&camp.archetype)?;
        building_region(camp.position, archetype.footprint, world)
    });
    building_regions.chain(camp_regions).collect()
}

fn region_contains_grid_position(
    region: stream_town_domain::DirtyRegion,
    position: GridPos,
) -> bool {
    (region.min.x..=region.max.x).contains(&position.x)
        && (region.min.z..=region.max.z).contains(&position.z)
}

/// Mirrors Unity's placement-time foliage clearing without destroying the
/// deterministic generated instances. Deriving visibility from current
/// structural occupancy also restores foliage when a building is removed and
/// recomputes the correct result after loading a different save.
fn sync_foliage_clearance(
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    mut foliage: Query<(&GridLocation, &mut Visibility), With<FoliageVisual>>,
) {
    let regions = foliage_clearance_regions(&content.0, &simulation.0, &world.generated);
    for (location, mut visibility) in &mut foliage {
        let should_be_hidden = regions
            .iter()
            .any(|region| region_contains_grid_position(*region, location.0));
        if should_be_hidden && !matches!(*visibility, Visibility::Hidden) {
            *visibility = Visibility::Hidden;
        } else if !should_be_hidden && matches!(*visibility, Visibility::Hidden) {
            *visibility = Visibility::Inherited;
        }
    }
}

fn quarter_turn_rotation(rotation_quarter_turns: i32) -> Quat {
    let normalized = i16::try_from(rotation_quarter_turns.rem_euclid(4))
        .expect("normalized quarter turn fits i16");
    Quat::from_rotation_y(-f32::from(normalized) * std::f32::consts::FRAC_PI_2)
}

fn building_site_is_available(
    world: &GeneratedWorld,
    position: GridPos,
    footprint: [u16; 2],
) -> bool {
    let Some(region) = building_region(position, footprint, world) else {
        return false;
    };
    (region.min.z..=region.max.z).all(|cell_z| {
        (region.min.x..=region.max.x).all(|cell_x| {
            world.navigation.is_walkable(GridPos {
                x: cell_x,
                z: cell_z,
            })
        })
    }) && !world.resources.iter().any(|resource| {
        resource.amount > 0
            && (region.min.x..=region.max.x).contains(&resource.position.x)
            && (region.min.z..=region.max.z).contains(&resource.position.z)
    })
}

fn find_building_site(
    world: &GeneratedWorld,
    near: GridPos,
    footprint: [u16; 2],
) -> Option<GridPos> {
    let mut candidates = Vec::new();
    for z in 0..world.navigation.height() {
        for x in 0..world.navigation.width() {
            let position = GridPos { x, z };
            if building_site_is_available(world, position, footprint) {
                candidates.push(position);
            }
        }
    }
    candidates.sort_by_key(|position| {
        (
            position.x.abs_diff(near.x) + position.z.abs_diff(near.z),
            position.z,
            position.x,
        )
    });
    candidates.into_iter().next()
}

#[allow(clippy::too_many_arguments)]
fn spawn_enemy_camp(
    commands: &mut Commands,
    config: &GameConfig,
    world: &GeneratedWorld,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    render: &RenderAssets,
    id: &StableId,
    archetype: &ArchetypeDef,
    position: GridPos,
) {
    let centre = GridPos {
        x: position.x + archetype.footprint[0] / 2,
        z: position.z + archetype.footprint[1] / 2,
    };
    let world_position = grid_to_world_on_surface(centre, config, world);
    let mut entity = commands.spawn((
        WorldEntity,
        EnemyCamp { id: id.clone() },
        GridLocation(position),
        Transform::from_translation(world_position),
    ));
    if let Some(scene) = default_archetype_scene(archetype).filter(|scene| {
        asset_server.is_some() && converted_asset_exists(asset_root, &scene.asset_path)
    }) {
        entity.insert((
            WorldAssetRoot(
                asset_server
                    .expect("asset server checked above")
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            Transform::from_translation(world_position)
                .with_scale(Vec3::splat(config.world.cell_size / 2.0)),
        ));
        if let Some(material) = prefab_material_spec(archetype, scene, presentation, render) {
            entity.insert(material);
        }
    } else {
        let size = Vec3::new(
            f32::from(archetype.footprint[0]) * config.world.cell_size * 0.8,
            config.world.cell_size * 1.2,
            f32::from(archetype.footprint[1]) * config.world.cell_size * 0.8,
        );
        entity.insert((
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.building.clone()),
            Transform::from_translation(world_position + Vec3::Y * size.y * 0.5).with_scale(size),
        ));
    }
}

fn enemy_spawn_position(
    world: &GeneratedWorld,
    camp: &EnemyCampState,
    spawner: &stream_town_domain::EnemySpawnerDef,
    serial: u64,
) -> GridPos {
    let offset = spawner.spawn_offsets_milli_cells
        [usize::try_from(serial % spawner.spawn_offsets_milli_cells.len() as u64).unwrap_or(0)];
    let offset_cells = |value: i32| -> i32 {
        if value >= 0 {
            (value + 500) / 1_000
        } else {
            (value - 500) / 1_000
        }
    };
    let x = i64::from(camp.position.x) + i64::from(offset_cells(offset[0]));
    let z = i64::from(camp.position.z) + i64::from(offset_cells(offset[1]));
    let desired = GridPos {
        x: u16::try_from(x.clamp(0, i64::from(world.navigation.width() - 1))).unwrap_or(0),
        z: u16::try_from(z.clamp(0, i64::from(world.navigation.height() - 1))).unwrap_or(0),
    };
    nearest_walkable(world, desired)
        .unwrap_or_else(|| nearest_walkable(world, camp.position).unwrap_or(camp.position))
}

fn weighted_enemy_archetype(
    spawner: &stream_town_domain::EnemySpawnerDef,
    seed: u64,
    serial: u64,
) -> StableId {
    let total = spawner
        .weighted_enemies
        .iter()
        .map(|entry| u64::from(entry.weight_milli))
        .sum::<u64>();
    let mut mixed = seed.wrapping_add(serial.wrapping_mul(0x9e37_79b9_7f4a_7c15));
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    let mut value = (mixed ^ (mixed >> 31)) % total;
    for entry in &spawner.weighted_enemies {
        if value < u64::from(entry.weight_milli) {
            return entry.enemy_archetype.clone();
        }
        value -= u64::from(entry.weight_milli);
    }
    spawner.weighted_enemies[0].enemy_archetype.clone()
}

#[allow(clippy::too_many_arguments)]
fn spawn_runtime_enemy(
    commands: &mut Commands,
    config: &GameConfig,
    world: &GeneratedWorld,
    content: &ContentCatalog,
    render: &RenderAssets,
    simulation: &mut WorldSimulation,
    archetype_id: StableId,
    position: GridPos,
) -> Option<StableId> {
    let archetype = content.archetypes.get(&archetype_id)?;
    let enemy = archetype.enemy.as_ref()?;
    let base_health = archetype.health.as_ref()?.max_health;
    let player_count = simulation
        .actors
        .values()
        .filter(|actor| actor.role.as_str() != "role:enemy")
        .count();
    let additional = u64::from(enemy.additional_health_milli_per_player)
        .saturating_mul(u64::try_from(player_count).unwrap_or(u64::MAX))
        / 1_000;
    let max_health =
        u32::try_from(u64::from(base_health).saturating_add(additional)).unwrap_or(u32::MAX);
    let id = loop {
        let serial = simulation.next_enemy_serial;
        simulation.next_enemy_serial = simulation.next_enemy_serial.saturating_add(1);
        let id =
            StableId::new(format!("actor:enemy_{serial:08}")).expect("runtime enemy IDs are valid");
        if !simulation.actors.contains_key(&id) {
            break id;
        }
    };
    if !simulation.spawn_enemy(
        id.clone(),
        archetype_id.clone(),
        position,
        i32::try_from(max_health).unwrap_or(i32::MAX),
    ) {
        return None;
    }
    let world_position = grid_to_world_on_surface(position, config, world);
    let base_scale = Vec3::new(
        config.world.cell_size * 0.3,
        config.world.cell_size * 0.55,
        config.world.cell_size * 0.3,
    );
    commands.spawn((
        WorldEntity,
        GridLocation(position),
        Agent {
            id: id.clone(),
            kind: ActorKind::Enemy,
            archetype: archetype_id,
            goal: AgentGoal::Wander,
            spawn: position,
            origin: position,
            path: Vec::new(),
            path_index: 0,
            target: mirrored_target(world, position),
            action_cooldown_seconds: 0.0,
            health_regen_accumulator: 0.0,
        },
        AgentAnimation {
            base_scale,
            ..default()
        },
        Mesh3d(render.actor_lod.clone()),
        MeshMaterial3d(render.enemy_idle.clone()),
        Transform::from_xyz(
            world_position.x,
            world_position.y + base_scale.y * 0.5,
            world_position.z,
        )
        .with_scale(base_scale),
    ));
    Some(id)
}

fn runtime_building_id(simulation: &WorldSimulation) -> StableId {
    for sequence in constructed_building_count(simulation)..usize::MAX {
        let candidate = StableId::new(format!("building:runtime_{sequence:08}"))
            .expect("runtime building IDs are valid");
        if !simulation.buildings.contains_key(&candidate) {
            return candidate;
        }
    }
    unreachable!("runtime building identifier space exhausted")
}

fn spawn_runtime_building(
    commands: &mut Commands,
    config: &GameConfig,
    world: &GeneratedWorld,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
    render: &RenderAssets,
    building: &BuildingState,
    definition: &BuildingDef,
    archetype: &ArchetypeDef,
    position: GridPos,
    footprint: [u16; 2],
    age: u8,
) {
    let occupied_footprint = rotated_footprint(footprint, building.rotation_quarter_turns);
    let centre = GridPos {
        x: position.x + occupied_footprint[0] / 2,
        z: position.z + occupied_footprint[1] / 2,
    };
    let world_position = grid_to_world_on_surface(centre, config, world);
    let rotation = quarter_turn_rotation(building.rotation_quarter_turns);
    let mut entity = commands.spawn((
        WorldEntity,
        RuntimeBuilding {
            id: building.id.clone(),
        },
        GridLocation(position),
        Transform::from_translation(world_position).with_rotation(rotation),
    ));
    if definition.projectile_shooter.is_some() {
        entity.insert(TowerShooter {
            building: building.id.clone(),
            cooldown_seconds: definition
                .projectile_shooter
                .as_ref()
                .map_or(0.0, |shooter| milli_units_as_f32(shooter.fire_milliseconds)),
        });
    }
    if let Some(scene) = archetype_scene_for_age(archetype, age).filter(|scene| {
        asset_server.is_some() && converted_asset_exists(asset_root, &scene.asset_path)
    }) {
        let base_scale = Vec3::splat(config.world.cell_size / 2.0);
        entity.insert((
            WorldAssetRoot(
                asset_server
                    .expect("asset server checked above")
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            BuildingPresentation {
                base_translation: world_position,
                base_scale,
                base_height_offset: 0.0,
                applied_stage: u8::MAX,
                applied_level: u16::MAX,
                applied_age: age,
            },
            BuildingDamageEmitter::default(),
            Transform::from_translation(world_position)
                .with_rotation(rotation)
                .with_scale(base_scale),
        ));
        if let Some(material) = prefab_material_spec(archetype, scene, presentation, render) {
            entity.insert(material);
        }
    } else {
        let size = Vec3::new(
            f32::from(footprint[0]) * config.world.cell_size * 0.88,
            config.world.cell_size * 1.25,
            f32::from(footprint[1]) * config.world.cell_size * 0.88,
        );
        let base_translation = world_position + Vec3::Y * size.y * 0.5;
        entity.insert((
            BuildingPresentation {
                base_translation,
                base_scale: size,
                base_height_offset: size.y * 0.5,
                applied_stage: u8::MAX,
                applied_level: u16::MAX,
                applied_age: age,
            },
            BuildingDamageEmitter::default(),
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(if building.complete {
                render.building.clone()
            } else {
                render.construction.clone()
            }),
            Transform::from_translation(base_translation)
                .with_rotation(rotation)
                .with_scale(size),
        ));
    }
}

fn require_staff(pending: &PendingChatCommand) -> Result<(), String> {
    if pending.is_broadcaster || pending.is_moderator {
        Ok(())
    } else {
        Err("this command requires broadcaster or moderator permission".to_owned())
    }
}

fn require_game_master(config: &GameConfig, pending: &PendingChatCommand) -> Result<(), String> {
    if pending.origin == CommandOrigin::LocalDebug {
        return Ok(());
    }
    let raw_user_id = pending
        .actor_id
        .as_str()
        .strip_prefix("twitch:")
        .unwrap_or_else(|| pending.actor_id.as_str());
    if config.twitch.game_master_ids.contains(raw_user_id) {
        Ok(())
    } else {
        Err("this command requires an explicitly configured game-master Twitch user ID".to_owned())
    }
}

fn require_ruler_or_staff(
    simulation: &WorldSimulation,
    pending: &PendingChatCommand,
) -> Result<(), String> {
    if simulation.is_ruler(&pending.actor_id) {
        Ok(())
    } else {
        require_staff(pending)
            .map_err(|_| "this command is restricted to the Ruler or staff".to_owned())
    }
}

fn resolve_ruler_vote_option(
    simulation: &WorldSimulation,
    requested: &StableId,
) -> Option<StableId> {
    let vote = simulation.ruler_vote.as_ref()?;
    if vote.kind == RulerVoteKind::KeepRuler {
        return matches!(requested.as_str(), "yes" | "no").then(|| requested.clone());
    }
    if simulation.actors.contains_key(requested) {
        return Some(requested.clone());
    }
    let prefixed = prefixed_id(requested, "twitch:");
    if let Some(actor) = prefixed.filter(|actor| simulation.actors.contains_key(actor)) {
        return Some(actor);
    }
    let normalized = requested.as_str().replace('_', " ");
    simulation
        .actors
        .values()
        .filter(|actor| actor.role.as_str() != "role:enemy" && actor.alive)
        .find(|actor| {
            actor
                .login_name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(requested.as_str()))
                || actor
                    .display_name
                    .as_deref()
                    .is_some_and(|name| name.replace('_', " ").eq_ignore_ascii_case(&normalized))
        })
        .map(|actor| actor.id.clone())
}

fn recruited_actor_ids(simulation: &WorldSimulation) -> Vec<StableId> {
    simulation
        .actors
        .keys()
        .filter(|id| {
            id.as_str().starts_with("npc:recruit_") || id.as_str().starts_with("npc:starting_")
        })
        .cloned()
        .collect()
}

fn recruit_id(simulation: &WorldSimulation, index: u16) -> Option<StableId> {
    recruited_actor_ids(simulation)
        .get(usize::from(index.saturating_sub(1)))
        .cloned()
}

fn role_capacity(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    role: &StableId,
) -> Option<u32> {
    let definition = content.roles.get(role)?;
    if !definition.has_user_limit {
        return None;
    }
    Some(
        simulation
            .buildings
            .values()
            .filter(|state| state.complete)
            .filter_map(|state| {
                building_def_for_archetype(content, &state.archetype)
                    .map(|definition| (definition, state.level))
            })
            .flat_map(|(building, level)| {
                building
                    .role_slots
                    .iter()
                    .filter(move |slots| slots.role == *role)
                    .map(move |slots| {
                        u32::from(slots.base_amount).saturating_add(
                            u32::from(slots.increment_amount)
                                .saturating_mul(u32::from(level.saturating_sub(1))),
                        )
                    })
            })
            .fold(u32::from(definition.base_max_users), u32::saturating_add),
    )
}

fn role_is_available(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    role: &StableId,
    excluding: Option<&StableId>,
) -> bool {
    if !simulation.role_limits_enabled {
        return true;
    }
    role_capacity(content, simulation, role).is_none_or(|capacity| {
        let used = simulation
            .actors
            .values()
            .filter(|actor| actor.role == *role && excluding != Some(&actor.id))
            .count();
        used < usize::try_from(capacity).unwrap_or(usize::MAX)
    })
}

fn resolve_player_id(simulation: &WorldSimulation, requested: &StableId) -> Option<StableId> {
    if simulation.actors.contains_key(requested) {
        return Some(requested.clone());
    }
    let prefixed = prefixed_id(requested, "twitch:");
    if let Some(actor) = prefixed.filter(|actor| simulation.actors.contains_key(actor)) {
        return Some(actor);
    }
    let normalized = requested.as_str().replace('_', " ");
    simulation
        .actors
        .values()
        .find(|actor| {
            actor
                .login_name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(requested.as_str()))
                || actor
                    .display_name
                    .as_deref()
                    .is_some_and(|name| name.replace('_', " ").eq_ignore_ascii_case(&normalized))
        })
        .map(|actor| actor.id.clone())
}

fn item_info(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    requested: &StableId,
) -> Result<String, String> {
    if let Some(role_id) = prefixed_id(requested, "role:")
        && let Some(role) = content.roles.get(&role_id)
    {
        let capacity = role_capacity(content, simulation, &role_id)
            .map_or_else(|| "unlimited".to_owned(), |value| value.to_string());
        let assigned = simulation
            .actors
            .values()
            .filter(|actor| actor.role == role_id)
            .count();
        return Ok(format!(
            "{}: action {}, health {}, carry {}, slots {assigned}/{capacity}",
            role.display_name, role.base_action_amount, role.base_health, role.base_carry_capacity
        ));
    }
    if let Some(building_id) = prefixed_id(requested, "building:")
        && let Some(building) = content.buildings.get(&building_id)
    {
        let count = simulation
            .buildings
            .values()
            .filter(|state| state.archetype == building.archetype)
            .count();
        let cost = building
            .cost
            .iter()
            .map(|(resource, amount)| format!("{resource}={amount}"))
            .collect::<Vec<_>>()
            .join(", ");
        return Ok(format!(
            "{}: count {count}, footprint {}x{}, cost {cost}",
            building.display_name, building.footprint[0], building.footprint[1]
        ));
    }
    if let Some(technology_id) = resolve_technology_id(content, requested) {
        let technology = &content.technology.nodes[&technology_id];
        let status = if simulation.unlocked_technology.contains(&technology_id) {
            "unlocked"
        } else if technology.unavailable {
            "unavailable"
        } else {
            "locked"
        };
        return Ok(format!(
            "{} ({status}): {}",
            technology.display_name, technology.description
        ));
    }
    if let Some(resource_id) = prefixed_id(requested, "resource:")
        && let Some(amount) = simulation.town_resources.get(&resource_id)
    {
        return Ok(format!("{resource_id}: town amount {amount}"));
    }
    if let Some(player_id) = resolve_player_id(simulation, requested) {
        let actor = &simulation.actors[&player_id];
        return Ok(format!(
            "{player_id}: {}, health {}/{}, at {},{}",
            actor.role, actor.health, actor.max_health, actor.position.x, actor.position.z
        ));
    }
    Err(format!("unknown item {requested}"))
}

fn compatible_station_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    config: &GameConfig,
    actor: &ActorState,
) -> Vec<StableId> {
    let Some(role) = content.roles.get(&actor.role) else {
        return Vec::new();
    };
    let town_hall = StableId::new("building:townhall").expect("static ID");
    let mut stations: Vec<_> = std::iter::once(town_hall)
        .chain(
            simulation
                .buildings
                .keys()
                .filter(|id| id.as_str() != "building:townhall")
                .cloned(),
        )
        .filter(|id| {
            station_candidate(content, simulation, config, id).is_some_and(|station| {
                station_matches_role(station.definition, role)
                    && station_supports_role_targets(station.definition, role)
            })
        })
        .collect();
    stations.sort();
    stations
}

fn compatible_target_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    actor: &ActorState,
) -> Vec<StableId> {
    let Some(role) = content.roles.get(&actor.role) else {
        return Vec::new();
    };
    let mut targets = Vec::new();
    let accepts =
        |kind: &str| role.targets_all || role.target_kinds.iter().any(|id| id.as_str() == kind);
    if accepts("target:tree") {
        targets.extend(
            world
                .resources
                .iter()
                .filter(|resource| resource.amount > 0 && resource.kind.as_str() == "resource:wood")
                .map(|resource| resource.id.clone()),
        );
    }
    if accepts("target:ore") {
        targets.extend(
            world
                .resources
                .iter()
                .filter(|resource| resource.amount > 0 && resource.kind.as_str() == "resource:ore")
                .map(|resource| resource.id.clone()),
        );
    }
    if ["target:bush", "target:farm", "target:fish"]
        .iter()
        .any(|kind| accepts(kind))
    {
        targets.extend(
            world
                .resources
                .iter()
                .filter(|resource| resource.amount > 0 && resource.kind.as_str() == "resource:food")
                .map(|resource| resource.id.clone()),
        );
    }
    if accepts("target:enemy") {
        targets.extend(
            simulation
                .actors
                .values()
                .filter(|target| target.alive && target.role.as_str() == "role:enemy")
                .map(|target| target.id.clone()),
        );
    }
    if accepts("target:injured_player") {
        targets.extend(
            simulation
                .actors
                .values()
                .filter(|target| {
                    target.id != actor.id
                        && target.alive
                        && target.role.as_str() != "role:enemy"
                        && target.health < target.max_health
                })
                .map(|target| target.id.clone()),
        );
    }
    if accepts("target:construction") || accepts("target:damaged_building") {
        targets.extend(
            simulation
                .buildings
                .values()
                .filter(|building| {
                    (!building.complete && accepts("target:construction"))
                        || (building.complete
                            && building.health < BUILDING_MAX_HEALTH
                            && accepts("target:damaged_building"))
                })
                .map(|building| building.id.clone()),
        );
    }
    targets.sort();
    targets.dedup();
    targets
}

#[allow(clippy::too_many_arguments)]
fn recruit_npcs(
    commands: &mut Commands,
    config: &GameConfig,
    content: &ContentCatalog,
    world: &GeneratedWorld,
    render: &RenderAssets,
    simulation: &mut WorldSimulation,
    role: &StableId,
    requested_amount: u16,
) -> Result<String, String> {
    let recruit_resource = StableId::new("resource:recruit").expect("static resource ID");
    let current = simulation
        .town_resources
        .get(&recruit_resource)
        .copied()
        .unwrap_or_default();
    let capacity = resource_storage_capacity(config, content, simulation, &recruit_resource);
    let available = capacity.saturating_sub(current);
    let amount = u32::from(requested_amount).min(available);
    if amount == 0 {
        return Err("the town has no recruit capacity".to_owned());
    }
    if !content.roles.contains_key(role) {
        return Err(format!("unknown role {role}"));
    }
    let current_in_role = simulation
        .actors
        .values()
        .filter(|actor| actor.role == *role)
        .count();
    let available_role_slots = role_capacity(content, simulation, role).map_or(usize::MAX, |max| {
        usize::try_from(max)
            .unwrap_or(usize::MAX)
            .saturating_sub(current_in_role)
    });
    let amount = amount.min(u32::try_from(available_role_slots).unwrap_or(u32::MAX));
    if amount == 0 {
        return Err(format!("the {role} role is full"));
    }
    let archetype =
        archetype_id_by_source(content, ArchetypeKind::Player, "Player_Character.prefab")
            .unwrap_or_else(|| StableId::new("archetype:viewer").expect("static ID"));
    let center = GridPos {
        x: world.navigation.width() / 2,
        z: world.navigation.height() / 2,
    };
    let base_scale = Vec3::new(
        config.world.cell_size * 0.3,
        config.world.cell_size * 0.55,
        config.world.cell_size * 0.3,
    );
    let mut spawned = 0_u32;
    for _ in 0..amount {
        let mut sequence = u64::try_from(simulation.actors.len()).unwrap_or(u64::MAX);
        let id = loop {
            let candidate =
                StableId::new(format!("npc:recruit_{sequence:08}")).expect("runtime recruit ID");
            if !simulation.actors.contains_key(&candidate) {
                break candidate;
            }
            sequence = sequence
                .checked_add(1)
                .ok_or_else(|| "runtime recruit identifier space exhausted".to_owned())?;
        };
        let desired = GridPos {
            x: center
                .x
                .saturating_add(u16::try_from(sequence % 9).unwrap_or_default()),
            z: center
                .z
                .saturating_add(u16::try_from((sequence / 9) % 9).unwrap_or_default()),
        };
        let position = nearest_walkable(world, desired)
            .or_else(|| nearest_walkable(world, center))
            .ok_or_else(|| "no walkable recruit spawn is available".to_owned())?;
        if !simulation.join_player(id.clone(), position) {
            continue;
        }
        simulation
            .assign_role(&id, role.clone())
            .map_err(|error| error.to_string())?;
        if let Some(actor) = simulation.actors.get_mut(&id) {
            actor.archetype = Some(archetype.clone());
            actor.display_name = Some(format!("Recruit {}", sequence + 1));
            actor.login_name = None;
        }
        let target = mirrored_target(world, position);
        let world_position = grid_to_world_on_surface(position, config, world);
        commands.spawn((
            WorldEntity,
            GridLocation(position),
            Agent {
                id,
                kind: ActorKind::Player,
                archetype: archetype.clone(),
                goal: AgentGoal::Wander,
                spawn: position,
                origin: position,
                path: Vec::new(),
                path_index: 0,
                target,
                action_cooldown_seconds: 0.0,
                health_regen_accumulator: 0.0,
            },
            AgentAnimation {
                base_scale,
                ..default()
            },
            Mesh3d(render.actor_lod.clone()),
            MeshMaterial3d(actor_material(render, &ActorKind::Player, false)),
            Transform::from_translation(world_position + Vec3::Y * base_scale.y * 0.5)
                .with_scale(base_scale),
        ));
        spawned = spawned.saturating_add(1);
    }
    *simulation
        .town_resources
        .entry(recruit_resource)
        .or_default() = current.saturating_add(spawned);
    Ok(format!("recruited {spawned} {role}"))
}

fn send_command_feedback(connection: &TwitchConnection, display_name: &str, message: &str) {
    if let Some(transport) = &connection.transport {
        let _ = transport.send(TwitchControl::SendMessage(format!(
            "@{display_name} {message}"
        )));
    }
}

fn can_afford(simulation: &WorldSimulation, cost: &BTreeMap<StableId, u32>) -> bool {
    cost.iter().all(|(resource, required)| {
        simulation
            .town_resources
            .get(resource)
            .copied()
            .unwrap_or_default()
            >= *required
    })
}

fn shift_grid_position(
    position: GridPos,
    actions: &[BuildingAction],
    world: &GeneratedWorld,
) -> (GridPos, i32) {
    let mut x = i32::from(position.x);
    let mut z = i32::from(position.z);
    let mut rotation = 0_i32;
    for action in actions {
        match action.direction {
            BuildingDirection::Up => z = z.saturating_add(action.amount),
            BuildingDirection::Down => z = z.saturating_sub(action.amount),
            BuildingDirection::Left => x = x.saturating_sub(action.amount),
            BuildingDirection::Right => x = x.saturating_add(action.amount),
            BuildingDirection::Rotate => rotation = rotation.saturating_add(action.amount),
        }
    }
    let max_x = i32::from(world.navigation.width().saturating_sub(1));
    let max_z = i32::from(world.navigation.height().saturating_sub(1));
    (
        GridPos {
            x: u16::try_from(x.clamp(0, max_x)).expect("grid x is clamped"),
            z: u16::try_from(z.clamp(0, max_z)).expect("grid z is clamped"),
        },
        rotation,
    )
}

fn building_definition_id(
    content: &ContentCatalog,
    requested: &StableId,
) -> Result<StableId, String> {
    prefixed_id(requested, "building:")
        .filter(|id| content.buildings.contains_key(id))
        .ok_or_else(|| format!("unknown building {requested}"))
}

fn building_instance_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building_id: &StableId,
) -> Vec<StableId> {
    let archetype = &content.buildings[building_id].archetype;
    simulation
        .buildings
        .values()
        .filter(|building| building.archetype == *archetype)
        .map(|building| building.id.clone())
        .collect()
}

fn constructed_building_count(simulation: &WorldSimulation) -> usize {
    simulation
        .buildings
        .keys()
        .filter(|id| id.as_str() != "building:townhall")
        .count()
}

fn upgrade_building_instance(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    building_id: &StableId,
    runtime_id: &StableId,
) -> Result<u16, String> {
    let definition = &content.buildings[building_id];
    if !definition.can_level {
        return Err(format!("{} cannot be upgraded", definition.display_name));
    }
    let state = simulation
        .buildings
        .get(runtime_id)
        .ok_or_else(|| format!("building instance {runtime_id} does not exist"))?;
    let cost = if simulation.building_costs_enabled {
        building_upgrade_cost(content, simulation, building_id, definition, state.level)
    } else {
        BTreeMap::new()
    };
    let max_level = maximum_building_level(content, simulation, building_id);
    simulation
        .upgrade_building(runtime_id, max_level, &cost)
        .map_err(|error| error.to_string())
}

#[allow(clippy::too_many_arguments)]
fn process_injected_commands(
    mut ecs: Commands,
    mut queues: RuntimeCommandQueues,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    save: Res<SaveRuntime>,
    selected: Res<SelectedCell>,
    connection: Res<TwitchConnection>,
    mut feedback: ResMut<CommandFeedback>,
    mut world: ResMut<WorldRuntime>,
    mut stats: ResMut<SessionStats>,
    mut simulation: ResMut<SimulationRuntime>,
    agents: Query<(&Agent, &GridLocation)>,
) {
    while let Some(pending) = queues.injected.0.pop_front() {
        let actor_id = pending.actor_id.clone();
        let command = pending.command.clone();
        if let Some(actor) = simulation.0.actors.get_mut(&actor_id) {
            actor.display_name = Some(pending.display_name.clone());
            actor.login_name = Some(pending.login_name.clone());
            if pending.is_subscriber {
                let red_panda = StableId::new("pet:red_panda").expect("static pet ID");
                actor.unlocked_pets.insert(red_panda.clone());
                actor.active_pet.get_or_insert(red_panda);
            }
        }
        let result = (|| -> Result<String, String> {
            match &command {
            ChatCommand::Join => {
                if simulation.0.actors.contains_key(&actor_id) {
                    Ok("you are already in town".to_owned())
                } else {
                    let desired = GridPos {
                        x: world.generated.navigation.width() / 2,
                        z: world.generated.navigation.height() / 2,
                    };
                    if let Some(position) = nearest_walkable(&world.generated, desired) {
                        let target = nearest_walkable(
                            &world.generated,
                            GridPos {
                                x: position.x.saturating_sub(8),
                                z: position.z.saturating_sub(8),
                            },
                        )
                        .unwrap_or(position);
                        let world_position =
                            grid_to_world_on_surface(position, &config.0, &world.generated);
                        simulation.0.join_player(actor_id.clone(), position);
                        let player_archetype = archetype_id_by_source(
                            &content.0,
                            ArchetypeKind::Player,
                            "Player_Character.prefab",
                        )
                        .unwrap_or_else(|| {
                            StableId::new("archetype:viewer").expect("static ID")
                        });
                        if let Some(actor) = simulation.0.actors.get_mut(&actor_id) {
                            actor.archetype = Some(player_archetype.clone());
                            actor.display_name = Some(pending.display_name.clone());
                            actor.login_name = Some(pending.login_name.clone());
                            if pending.is_subscriber {
                                let red_panda =
                                    StableId::new("pet:red_panda").expect("static pet ID");
                                actor.unlocked_pets.insert(red_panda.clone());
                                actor.active_pet.get_or_insert(red_panda);
                            }
                        }
                        let base_scale = Vec3::new(
                            config.0.world.cell_size * 0.3,
                            config.0.world.cell_size * 0.55,
                            config.0.world.cell_size * 0.3,
                        );
                        ecs.spawn((
                            WorldEntity,
                            GridLocation(position),
                            Agent {
                                id: actor_id.clone(),
                                kind: ActorKind::Player,
                                archetype: player_archetype,
                                goal: AgentGoal::Wander,
                                spawn: position,
                                origin: position,
                                path: Vec::new(),
                                path_index: 0,
                                target,
                                action_cooldown_seconds: 0.0,
                                health_regen_accumulator: 0.0,
                            },
                            AgentAnimation {
                                base_scale,
                                ..default()
                            },
                            Mesh3d(render.actor_lod.clone()),
                            MeshMaterial3d(actor_material(&render, &ActorKind::Player, false)),
                            Transform::from_xyz(
                                world_position.x,
                                world_position.y + base_scale.y * 0.5,
                                world_position.z,
                            )
                            .with_scale(base_scale),
                        ));
                        Ok("welcome to Stream Town".to_owned())
                    } else {
                        Err("no walkable join position is available".to_owned())
                    }
                }
            }
            ChatCommand::SelectRole(role) => {
                let role = prefixed_id(role, "role:")
                    .filter(|role| content.0.roles.contains_key(role))
                    .ok_or_else(|| format!("unknown role {}", role.as_str()));
                role.and_then(|role| {
                    if role.as_str() == "role:ruler" && !simulation.0.is_ruler(&actor_id) {
                        return Err("the Ruler role is assigned by election".to_owned());
                    }
                    if !role_is_available(&content.0, &simulation.0, &role, Some(&actor_id)) {
                        return Err(format!("the {role} role is full"));
                    }
                    simulation
                        .0
                        .assign_role(&actor_id, role.clone())
                        .map(|()| format!("role changed to {role}"))
                        .map_err(|error| error.to_string())
                })
            }
            ChatCommand::Role => simulation
                .0
                .actors
                .get(&actor_id)
                .ok_or_else(|| "join before checking your role".to_owned())
                .map(|actor| {
                    let progress = actor.role_progression.get(&actor.role).copied().unwrap_or_default();
                    format!("you are a level {} {}", progress.level, actor.role)
                }),
            ChatCommand::Health => simulation
                .0
                .actors
                .get(&actor_id)
                .ok_or_else(|| "join before checking health".to_owned())
                .map(|actor| format!("health: {}/{}", actor.health, actor.max_health)),
            ChatCommand::Buildings => {
                let names = content
                    .0
                    .buildings
                    .iter()
                    .filter(|(id, building)| {
                        building.placeable && building_is_unlocked(&content.0, &simulation.0, id)
                    })
                    .map(|(_, building)| building.display_name.as_str())
                    .collect::<Vec<_>>();
                Ok(format!("unlocked buildings: {}", names.join(", ")))
            }
            ChatCommand::BuildingIds(requested) => {
                let building_id = prefixed_id(requested, "building:")
                    .filter(|id| content.0.buildings.contains_key(id))
                    .ok_or_else(|| format!("unknown building {requested}"))?;
                let definition = &content.0.buildings[&building_id];
                let ids = simulation
                    .0
                    .buildings
                    .values()
                    .filter(|building| building.archetype == definition.archetype)
                    .enumerate()
                    .map(|(index, building)| format!("{}={}", index + 1, building.id))
                    .collect::<Vec<_>>();
                Ok(if ids.is_empty() {
                    format!("no {} buildings", definition.display_name)
                } else {
                    format!("{} IDs: {}", definition.display_name, ids.join(", "))
                })
            }
            ChatCommand::Build(requested) => {
                let building_id = building_definition_id(&content.0, requested);
                building_id.and_then(|building_id| {
                    let building = &content.0.buildings[&building_id];
                    if queues.placers.0.contains_key(&actor_id) {
                        return Err("already placing a building; use !confirm or !cancel".to_owned());
                    }
                    if !building.placeable {
                        return Err(format!("{} cannot be player-placed", building.display_name));
                    }
                    if !building_is_unlocked(&content.0, &simulation.0, &building_id) {
                        return Err(format!("{} is not unlocked", building.display_name));
                    }
                    let actor = simulation
                        .0
                        .actors
                        .get(&actor_id)
                        .ok_or_else(|| "join before building".to_owned())?;
                    let cost = building_construction_cost(
                        &content.0,
                        &simulation.0,
                        &building_id,
                        building,
                    );
                    if simulation.0.building_costs_enabled
                        && !can_afford(&simulation.0, &cost)
                    {
                        return Err(format!("cannot afford {}", building.display_name));
                    }
                    let rotation = actor.building_rotation_quarter_turns;
                    let near = actor
                        .last_building_position
                        .or(selected.0)
                        .unwrap_or(actor.position);
                    queues.placers.0.insert(
                        actor_id.clone(),
                        BuildingPlacement {
                            building: building_id,
                            position: near,
                            rotation_quarter_turns: rotation,
                        },
                    );
                    Ok(format!(
                        "placing {} at {},{}; use !move, !rotate, !confirm, or !cancel",
                        building.display_name, near.x, near.z
                    ))
                })
            }
            ChatCommand::MoveBuilding(actions) => {
                let placement = queues
                    .placers
                    .0
                    .get_mut(&actor_id)
                    .ok_or_else(|| "not in building placement mode".to_owned())?;
                let (position, rotation_delta) =
                    shift_grid_position(placement.position, actions, &world.generated);
                placement.position = position;
                placement.rotation_quarter_turns = placement
                    .rotation_quarter_turns
                    .saturating_add(rotation_delta);
                if let Some(actor) = simulation.0.actors.get_mut(&actor_id) {
                    actor.building_rotation_quarter_turns = actor
                        .building_rotation_quarter_turns
                        .saturating_add(rotation_delta);
                }
                let definition = &content.0.buildings[&placement.building];
                let footprint = rotated_footprint(
                    definition.footprint,
                    placement.rotation_quarter_turns,
                );
                let validity = if building_site_is_available(
                    &world.generated,
                    placement.position,
                    footprint,
                ) {
                    "valid"
                } else {
                    "blocked"
                };
                Ok(format!(
                    "{} placer moved to {},{} at {} degrees ({validity})",
                    definition.display_name,
                    placement.position.x,
                    placement.position.z,
                    placement.rotation_quarter_turns.rem_euclid(4) * 90
                ))
            }
            ChatCommand::ConfirmBuilding => {
                let placement = queues
                    .placers
                    .0
                    .get(&actor_id)
                    .cloned()
                    .ok_or_else(|| "not in building placement mode".to_owned())?;
                let building = &content.0.buildings[&placement.building];
                if !building.placeable
                    || !building_is_unlocked(&content.0, &simulation.0, &placement.building)
                {
                    return Err(format!("{} can no longer be placed", building.display_name));
                }
                let footprint =
                    rotated_footprint(building.footprint, placement.rotation_quarter_turns);
                if !building_site_is_available(
                    &world.generated,
                    placement.position,
                    footprint,
                ) {
                    return Err("building placement is blocked or outside the world".to_owned());
                }
                let cost = if simulation.0.building_costs_enabled {
                    building_construction_cost(
                        &content.0,
                        &simulation.0,
                        &placement.building,
                        building,
                    )
                } else {
                    BTreeMap::new()
                };
                let runtime_id = runtime_building_id(&simulation.0);
                let region = building_region(placement.position, footprint, &world.generated)
                    .ok_or_else(|| "building placement is outside the world".to_owned())?;
                simulation
                    .0
                    .construct_rotated(
                        runtime_id.clone(),
                        building.archetype.clone(),
                        placement.position,
                        placement.rotation_quarter_turns,
                        &cost,
                    )
                    .map_err(|error| error.to_string())?;
                world
                    .generated
                    .navigation
                    .set_blocked(region, true)
                    .map_err(|error| error.to_string())?;
                if let Some(actor) = simulation.0.actors.get_mut(&actor_id) {
                    actor.last_building_position = Some(placement.position);
                }
                queues.placers.0.remove(&actor_id);
                spawn_runtime_building(
                    &mut ecs,
                    &config.0,
                    &world.generated,
                    &presentation.0,
                    asset_server.as_deref(),
                    &asset_root.0,
                    &render,
                    &simulation.0.buildings[&runtime_id],
                    building,
                    &content.0.archetypes[&building.archetype],
                    placement.position,
                    building.footprint,
                    building_age(&content.0, &simulation.0, &placement.building),
                );
                Ok(format!("placed {} construction", building.display_name))
            }
            ChatCommand::CancelBuilding => {
                queues
                    .placers
                    .0
                    .remove(&actor_id)
                    .map(|placement| {
                        format!(
                            "cancelled {} placement",
                            content.0.buildings[&placement.building].display_name
                        )
                    })
                    .ok_or_else(|| "not in building placement mode".to_owned())
            }
            ChatCommand::Upgrade(requested) => {
                let building_id = building_definition_id(&content.0, requested);
                building_id.and_then(|building_id| {
                    let definition = &content.0.buildings[&building_id];
                    let candidate = simulation
                        .0
                        .buildings
                        .values()
                        .filter(|building| {
                            building.archetype == definition.archetype && building.complete
                        })
                        .min_by_key(|building| (building.level, building.id.clone()))
                        .map(|building| building.id.clone())
                        .ok_or_else(|| {
                            format!("no completed {} is available", definition.display_name)
                        })?;
                    upgrade_building_instance(
                        &content.0,
                        &mut simulation.0,
                        &building_id,
                        &candidate,
                    )
                        .map(|level| format!("upgraded {} to level {level}", definition.display_name))
                })
            }
            ChatCommand::Level(requested) => {
                let role = prefixed_id(requested, "role:")
                    .filter(|role| content.0.roles.contains_key(role))
                    .ok_or_else(|| format!("unknown role {requested}"))?;
                let actor = simulation
                    .0
                    .actors
                    .get(&actor_id)
                    .ok_or_else(|| "join before checking role progression".to_owned())?;
                let progress = actor
                    .role_progression
                    .get(&role)
                    .copied()
                    .ok_or_else(|| format!("you have no progression for {role}"))?;
                Ok(format!(
                    "{role} level {}, experience {}/{}",
                    progress.level,
                    progress.experience,
                    stream_town_domain::required_role_experience(progress.level)
                ))
            }
            ChatCommand::LevelBuilding {
                building,
                index,
                iterations,
            } => {
                let building_id = building_definition_id(&content.0, building)?;
                let definition = &content.0.buildings[&building_id];
                let instances = building_instance_ids(&content.0, &simulation.0, &building_id);
                let runtime_id = instances
                    .get(usize::from(index.saturating_sub(1)))
                    .cloned()
                    .ok_or_else(|| format!("{} building ID {index} does not exist", definition.display_name))?;
                let mut successful = 0_u16;
                let mut last_error = None;
                for _ in 0..*iterations {
                    match upgrade_building_instance(
                        &content.0,
                        &mut simulation.0,
                        &building_id,
                        &runtime_id,
                    ) {
                        Ok(_) => successful = successful.saturating_add(1),
                        Err(error) => {
                            last_error = Some(error);
                            break;
                        }
                    }
                }
                if successful == 0 {
                    Err(last_error.unwrap_or_else(|| "building could not be leveled".to_owned()))
                } else {
                    let level = simulation.0.buildings[&runtime_id].level;
                    Ok(format!(
                        "leveled {} ID {index} {successful} time(s) to level {level}",
                        definition.display_name
                    ))
                }
            }
            ChatCommand::LevelAll {
                building,
                target_level,
            } => {
                let building_id = building_definition_id(&content.0, building)?;
                let definition = &content.0.buildings[&building_id];
                let mut successful = 0_u32;
                loop {
                    let mut instances = building_instance_ids(&content.0, &simulation.0, &building_id);
                    instances.sort_by_key(|runtime_id| {
                        (simulation.0.buildings[runtime_id].level, runtime_id.clone())
                    });
                    let mut advanced = false;
                    for runtime_id in instances {
                        if simulation.0.buildings[&runtime_id].level >= *target_level {
                            continue;
                        }
                        if upgrade_building_instance(
                            &content.0,
                            &mut simulation.0,
                            &building_id,
                            &runtime_id,
                        )
                        .is_ok()
                        {
                            successful = successful.saturating_add(1);
                            advanced = true;
                        }
                    }
                    if !advanced {
                        break;
                    }
                }
                if successful == 0 {
                    Err(format!(
                        "no {} buildings could be leveled toward level {target_level}",
                        definition.display_name
                    ))
                } else {
                    Ok(format!(
                        "leveled {} buildings {successful} time(s) toward level {target_level}",
                        definition.display_name
                    ))
                }
            }
            ChatCommand::RemoveBuilding { building, index } => {
                let building_id = building_definition_id(&content.0, building)?;
                if building_id.as_str() == "building:townhall" {
                    return Err("the Town Hall cannot be removed".to_owned());
                }
                let definition = &content.0.buildings[&building_id];
                let instances = building_instance_ids(&content.0, &simulation.0, &building_id);
                let runtime_id = instances
                    .get(usize::from(index.saturating_sub(1)))
                    .cloned()
                    .ok_or_else(|| format!("{} building ID {index} does not exist", definition.display_name))?;
                let removed = simulation
                    .0
                    .buildings
                    .remove(&runtime_id)
                    .expect("building instance was validated");
                let footprint = rotated_footprint(
                    definition.footprint,
                    removed.rotation_quarter_turns,
                );
                let region = building_region(removed.position, footprint, &world.generated)
                    .ok_or_else(|| "removed building lies outside the world".to_owned())?;
                world
                    .generated
                    .navigation
                    .set_blocked(region, false)
                    .map_err(|error| error.to_string())?;
                for actor in simulation.0.actors.values_mut() {
                    if actor.station.as_ref() == Some(&runtime_id) {
                        actor.station = None;
                    }
                    if actor.preferred_target.as_ref() == Some(&runtime_id) {
                        actor.preferred_target = None;
                    }
                }
                queues
                    .building
                    .0
                    .push_back(BuildingRuntimeCommand::Despawn(runtime_id));
                Ok(format!("removed {} building ID {index}", definition.display_name))
            }
            ChatCommand::Sell { amount, resource } => {
                require_ruler_or_staff(&simulation.0, &pending)?;
                let resource = prefixed_id(resource, "resource:")
                    .ok_or_else(|| format!("invalid resource {}", resource.as_str()));
                resource.and_then(|resource| {
                    simulation
                        .0
                        .sell_resource(&resource, *amount)
                        .map(|(sold, gold)| {
                            let _ = simulation.0.record_objective_event(
                                &content.0.objectives,
                                &ObjectiveEvent::ResourceGained {
                                    resource: StableId::new("resource:gold")
                                        .expect("static stable ID"),
                                    amount: gold,
                                },
                            );
                            let _ = simulation.0.record_objective_event(
                                &content.0.objectives,
                                &ObjectiveEvent::ResourceSold {
                                    resource: resource.clone(),
                                    amount: sold,
                                },
                            );
                            format!("sold {sold} {resource} for {gold} gold")
                        })
                        .map_err(|error| error.to_string())
                })
            }
            ChatCommand::Buy { amount, resource } => {
                require_ruler_or_staff(&simulation.0, &pending)?;
                let resource = prefixed_id(resource, "resource:")
                    .ok_or_else(|| format!("invalid resource {}", resource.as_str()));
                resource.and_then(|resource| {
                    let capacity =
                        resource_storage_capacity(&config.0, &content.0, &simulation.0, &resource);
                    simulation
                        .0
                        .buy_resource(resource.clone(), *amount, capacity)
                        .map(|(bought, gold)| {
                            let _ = simulation.0.record_objective_event(
                                &content.0.objectives,
                                &ObjectiveEvent::ResourceGained {
                                    resource: resource.clone(),
                                    amount: bought,
                                },
                            );
                            let _ = simulation.0.record_objective_event(
                                &content.0.objectives,
                                &ObjectiveEvent::ResourceBought {
                                    resource: resource.clone(),
                                    amount: bought,
                                },
                            );
                            format!("bought {bought} {resource} for {gold} gold")
                        })
                        .map_err(|error| error.to_string())
                })
            }
            ChatCommand::Vote(requested) => {
                if simulation.0.ruler_vote.is_some() {
                    let option = resolve_ruler_vote_option(&simulation.0, requested)
                        .ok_or_else(|| format!("unknown ruler candidate {}", requested.as_str()));
                    option.and_then(|option| {
                        simulation
                            .0
                            .cast_ruler_vote(&actor_id, option.clone())
                            .map(|()| {
                                let label = simulation
                                    .0
                                    .actors
                                    .get(&option)
                                    .and_then(|actor| actor.display_name.as_deref())
                                    .unwrap_or(option.as_str());
                                format!("ruler vote for {label} accepted")
                            })
                            .map_err(|error| error.to_string())
                    })
                } else {
                let technology = resolve_technology_id(&content.0, requested)
                    .ok_or_else(|| format!("unknown technology {}", requested.as_str()));
                technology.and_then(|technology| {
                    let node = &content.0.technology.nodes[&technology];
                    if node.unavailable {
                        return Err(format!("{} is unavailable", node.display_name));
                    }
                    if simulation.0.active_goals.len() >= MAX_TOWN_GOALS {
                        return Err("the town already has the maximum active goals".to_owned());
                    }
                    if simulation.0.unlocked_technology.contains(&technology) {
                        return Err(format!("{} is already unlocked", node.display_name));
                    }
                    if let Some(prerequisite) = node
                        .prerequisites
                        .iter()
                        .find(|required| !simulation.0.unlocked_technology.contains(*required))
                    {
                        return Err(format!("missing prerequisite {prerequisite}"));
                    }
                    if simulation
                        .0
                        .active_vote
                        .as_ref()
                        .is_some_and(|vote| vote.technology != technology)
                    {
                        return Err("another technology vote is active".to_owned());
                    }
                if simulation.0.active_vote.is_none() {
                    let _ = simulation.0.start_technology_vote(technology.clone(), 30.0);
                }
                    simulation
                        .0
                        .cast_vote(&actor_id, true)
                        .map(|()| format!("voted for {}", node.display_name))
                        .map_err(|error| error.to_string())
                })
                }
            }
            ChatCommand::Recruit { role, amount } => {
                require_ruler_or_staff(&simulation.0, &pending)?;
                let role = prefixed_id(role, "role:")
                    .filter(|role| content.0.roles.contains_key(role))
                    .ok_or_else(|| format!("unknown role {}", role.as_str()))?;
                if role.as_str() == "role:ruler" {
                    Err("Ruler cannot be recruited".to_owned())
                } else {
                    recruit_npcs(
                        &mut ecs,
                        &config.0,
                        &content.0,
                        &world.generated,
                        &render,
                        &mut simulation.0,
                        &role,
                        *amount,
                    )
                }
            }
            ChatCommand::RecruitCount => {
                require_ruler_or_staff(&simulation.0, &pending)?;
                let recruits = recruited_actor_ids(&simulation.0).len();
                Ok(format!("the town has {recruits} recruited NPCs"))
            }
            ChatCommand::RecruitIds => {
                require_ruler_or_staff(&simulation.0, &pending)?;
                let recruits = recruited_actor_ids(&simulation.0)
                    .iter()
                    .enumerate()
                    .map(|(index, id)| format!("{}={id}", index + 1))
                    .collect::<Vec<_>>();
                Ok(if recruits.is_empty() {
                    "the town has no recruited NPCs".to_owned()
                } else {
                    format!("recruit IDs: {}", recruits.join(", "))
                })
            }
            ChatCommand::RecruitInfo(index) => {
                require_ruler_or_staff(&simulation.0, &pending)?;
                let id = recruit_id(&simulation.0, *index)
                    .ok_or_else(|| format!("unknown recruit ID {index}"))?;
                let recruit = &simulation.0.actors[&id];
                let progress = recruit
                    .role_progression
                    .get(&recruit.role)
                    .copied()
                    .unwrap_or_default();
                Ok(format!(
                    "recruit {index}: {id}, {}, health {}/{}, level {}, experience {}",
                    recruit.role, recruit.health, recruit.max_health, progress.level, progress.experience
                ))
            }
            ChatCommand::RecruitRole { recruit, role } => {
                require_ruler_or_staff(&simulation.0, &pending)?;
                let id = recruit_id(&simulation.0, *recruit)
                    .ok_or_else(|| format!("unknown recruit ID {recruit}"))?;
                let role = prefixed_id(role, "role:")
                    .filter(|role| content.0.roles.contains_key(role))
                    .ok_or_else(|| format!("unknown role {role}"))?;
                if role.as_str() == "role:ruler" {
                    return Err("Ruler cannot be assigned to a recruit".to_owned());
                }
                if !role_is_available(&content.0, &simulation.0, &role, Some(&id)) {
                    return Err(format!("the {role} role is full"));
                }
                simulation
                    .0
                    .assign_role(&id, role.clone())
                    .map_err(|error| error.to_string())?;
                Ok(format!("recruit {recruit} changed to {role}"))
            }
            ChatCommand::DismissRecruit(index) => {
                require_ruler_or_staff(&simulation.0, &pending)?;
                let id = recruit_id(&simulation.0, *index)
                    .ok_or_else(|| format!("unknown recruit ID {index}"))?;
                simulation.0.actors.remove(&id);
                let resource = StableId::new("resource:recruit").expect("static ID");
                let current = simulation.0.town_resources.get(&resource).copied().unwrap_or_default();
                simulation.0.town_resources.insert(resource, current.saturating_sub(1));
                queues.agent.0.push_back(AgentCommand::Despawn(id));
                Ok(format!("dismissed recruit {index}"))
            }
            ChatCommand::StartRulerVote => {
                require_staff(&pending)?;
                let kind = if simulation.0.current_ruler.is_some() {
                    RulerVoteKind::KeepRuler
                } else {
                    RulerVoteKind::NewRuler
                };
                simulation
                    .0
                    .start_ruler_vote(kind)
                    .map(|()| "ruler vote started".to_owned())
                    .map_err(|error| error.to_string())
            }
            ChatCommand::Resign => simulation
                .0
                .resign_ruler(&actor_id)
                .map(|()| "you resigned; a new ruler vote started".to_owned())
                .map_err(|error| error.to_string()),
            ChatCommand::Station(index) => {
                let actor = simulation
                    .0
                    .actors
                    .get(&actor_id)
                    .ok_or_else(|| "join before selecting a station".to_owned())?;
                let stations = compatible_station_ids(&content.0, &simulation.0, &config.0, actor);
                if let Some(index) = index {
                    let station = stations
                        .get(usize::from(index.saturating_sub(1)))
                        .cloned()
                        .ok_or_else(|| format!("unknown station ID {index}"))?;
                    simulation.0.actors.get_mut(&actor_id).expect("validated actor").station =
                        Some(station.clone());
                    Ok(format!("station changed to {station}"))
                } else {
                    Ok(if stations.is_empty() {
                        "no compatible stations are available".to_owned()
                    } else {
                        format!(
                            "station IDs: {}",
                            stations
                                .iter()
                                .enumerate()
                                .map(|(index, id)| format!("{}={id}", index + 1))
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    })
                }
            }
            ChatCommand::Target(index) => {
                let actor = simulation
                    .0
                    .actors
                    .get(&actor_id)
                    .ok_or_else(|| "join before selecting a target".to_owned())?;
                let targets = compatible_target_ids(
                    &content.0,
                    &simulation.0,
                    &world.generated,
                    actor,
                );
                if let Some(index) = index {
                    let target = targets
                        .get(usize::from(index.saturating_sub(1)))
                        .cloned()
                        .ok_or_else(|| format!("unknown target ID {index}"))?;
                    simulation
                        .0
                        .actors
                        .get_mut(&actor_id)
                        .expect("validated actor")
                        .preferred_target = Some(target.clone());
                    Ok(format!("target changed to {target}"))
                } else {
                    Ok(if targets.is_empty() {
                        "no compatible targets are available".to_owned()
                    } else {
                        format!(
                            "target IDs: {}",
                            targets
                                .iter()
                                .enumerate()
                                .map(|(index, id)| format!("{}={id}", index + 1))
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    })
                }
            }
            ChatCommand::Unstuck => {
                let spawn = nearest_walkable(&world.generated, town_hall_grid_position(&config.0))
                    .ok_or_else(|| "the Town Hall has no walkable spawn cell".to_owned())?;
                let actor = simulation
                    .0
                    .actors
                    .get_mut(&actor_id)
                    .ok_or_else(|| "join before using !stuck".to_owned())?;
                actor.position = spawn;
                actor.preferred_target = None;
                queues.agent.0.push_back(AgentCommand::Teleport {
                    actor: actor_id.clone(),
                    position: spawn,
                });
                Ok("returned to the Town Hall".to_owned())
            }
            ChatCommand::Ping => {
                let position = simulation
                    .0
                    .actors
                    .get(&actor_id)
                    .map(|actor| actor.position)
                    .ok_or_else(|| "join before using !ping".to_owned())?;
                Ok(format!("you are at grid {},{}", position.x, position.z))
            }
            ChatCommand::Customize { kind, index } => {
                let actor = simulation
                    .0
                    .actors
                    .get_mut(&actor_id)
                    .ok_or_else(|| "join before customizing your character".to_owned())?;
                let adjusted = index.saturating_sub(1);
                let (name, maximum, field) = match kind {
                    CustomizationKind::Hair => ("hair", 7, &mut actor.customization.hair),
                    CustomizationKind::Eyes => ("eyes", 10, &mut actor.customization.eyes),
                    CustomizationKind::FacialHair => {
                        ("facial hair", 2, &mut actor.customization.facial_hair)
                    }
                    CustomizationKind::Body => ("body", 3, &mut actor.customization.body_type),
                    CustomizationKind::HairColor => {
                        ("hair color", 6, &mut actor.customization.hair_color)
                    }
                    CustomizationKind::EyeColor => {
                        ("eye color", 5, &mut actor.customization.eye_color)
                    }
                };
                if *index > maximum {
                    return Err(format!("{name} index must be between 1 and {maximum}"));
                }
                *field = adjusted;
                Ok(format!("{name} changed to {index}"))
            }
            ChatCommand::Pets | ChatCommand::Pet(None) => {
                let actor = simulation
                    .0
                    .actors
                    .get(&actor_id)
                    .ok_or_else(|| "join before checking pets".to_owned())?;
                Ok(if actor.unlocked_pets.is_empty() {
                    "you have no unlocked pets".to_owned()
                } else {
                    format!(
                        "pets: {}; active: {}",
                        actor
                            .unlocked_pets
                            .iter()
                            .map(StableId::as_str)
                            .collect::<Vec<_>>()
                            .join(", "),
                        actor.active_pet.as_ref().map_or("none", StableId::as_str)
                    )
                })
            }
            ChatCommand::Pet(Some(requested)) => {
                let requested = prefixed_id(requested, "pet:")
                    .ok_or_else(|| format!("invalid pet {requested}"))?;
                let actor = simulation
                    .0
                    .actors
                    .get_mut(&actor_id)
                    .ok_or_else(|| "join before selecting a pet".to_owned())?;
                if requested.as_str() == "pet:none" {
                    actor.active_pet = None;
                    Ok("pet deactivated".to_owned())
                } else if actor.unlocked_pets.contains(&requested) {
                    actor.active_pet = Some(requested.clone());
                    Ok(format!("active pet changed to {requested}"))
                } else {
                    Err(format!("{requested} is not unlocked"))
                }
            }
            ChatCommand::Camera(actions) => {
                require_ruler_or_staff(&simulation.0, &pending)?;
                queues.camera.0.push_back(CameraRequest {
                    reset: false,
                    actions: actions.clone(),
                });
                Ok("camera request queued".to_owned())
            }
            ChatCommand::ResetCamera => {
                require_ruler_or_staff(&simulation.0, &pending)?;
                queues.camera.0.push_back(CameraRequest {
                    reset: true,
                    actions: Vec::new(),
                });
                Ok("camera reset queued".to_owned())
            }
            ChatCommand::ModRole { player, role } => {
                require_staff(&pending)?;
                let player = resolve_player_id(&simulation.0, player)
                    .ok_or_else(|| format!("unknown player {player}"))?;
                let role = prefixed_id(role, "role:")
                    .filter(|role| content.0.roles.contains_key(role))
                    .ok_or_else(|| format!("unknown role {role}"))?;
                if role.as_str() == "role:ruler" && !simulation.0.is_ruler(&player) {
                    return Err("the Ruler role is assigned by election".to_owned());
                }
                if !role_is_available(&content.0, &simulation.0, &role, Some(&player)) {
                    return Err(format!("the {role} role is full"));
                }
                simulation.0.assign_role(&player, role.clone()).map_err(|error| error.to_string())?;
                Ok(format!("changed {player} to {role}"))
            }
            ChatCommand::Roles => {
                let roles = content
                    .0
                    .roles
                    .iter()
                    .filter(|(id, _)| id.as_str() != "role:ruler")
                    .filter(|(id, _)| role_is_available(&content.0, &simulation.0, id, None))
                    .map(|(_, role)| role.display_name.as_str())
                    .collect::<Vec<_>>();
                Ok(format!("available roles: {}", roles.join(", ")))
            }
            ChatCommand::TownStats => Ok(format!(
                "town: {} players, {} recruits, {} buildings, day {}, {:?}/{:?}, resources {}",
                simulation
                    .0
                    .actors
                    .values()
                    .filter(|actor| actor.id.as_str().starts_with("twitch:"))
                    .count(),
                recruited_actor_ids(&simulation.0).len(),
                simulation.0.buildings.len(),
                simulation.0.day,
                simulation.0.season,
                simulation.0.weather,
                simulation
                    .0
                    .town_resources
                    .iter()
                    .map(|(resource, amount)| format!("{resource}={amount}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            ChatCommand::Discord => {
                Ok("Stream Town Discord: https://discord.gg/By4jvks".to_owned())
            }
            ChatCommand::Info(requested) => item_info(&content.0, &simulation.0, requested),
            ChatCommand::ToggleBuildCosts => {
                require_game_master(&config.0, &pending)?;
                let enabled = simulation.0.toggle_building_costs();
                Ok(format!("Buildings Cost Resources: {enabled}"))
            }
            ChatCommand::ToggleRoleLimits => {
                require_game_master(&config.0, &pending)?;
                let enabled = simulation.0.toggle_role_limits();
                Ok(format!("Player Role Limits: {enabled}"))
            }
            ChatCommand::AddResource { resource, amount } => {
                require_game_master(&config.0, &pending)?;
                let resource = prefixed_id(resource, "resource:")
                    .filter(|resource| {
                        matches!(
                            resource.as_str(),
                            "resource:wood"
                                | "resource:ore"
                                | "resource:food"
                                | "resource:gold"
                                | "resource:recruit"
                        )
                    })
                    .ok_or_else(|| format!("unknown town resource {resource}"))?;
                let total = simulation.0.adjust_town_resource(resource.clone(), *amount);
                Ok(format!("{resource} changed by {amount}; total {total}"))
            }
            ChatCommand::KillPlayer(requested) => {
                require_game_master(&config.0, &pending)?;
                let target = resolve_player_id(&simulation.0, requested)
                    .ok_or_else(|| format!("unknown player {requested}"))?;
                simulation
                    .0
                    .damage_actor(&target, u32::MAX)
                    .map_err(|error| error.to_string())?;
                Ok(format!("killed {target}"))
            }
            ChatCommand::GameMasterRevive(requested) => {
                require_game_master(&config.0, &pending)?;
                let target = resolve_player_id(&simulation.0, requested)
                    .ok_or_else(|| format!("unknown player {requested}"))?;
                let position = simulation.0.actors[&target].position;
                let spawn = nearest_walkable(&world.generated, position).unwrap_or(position);
                simulation
                    .0
                    .respawn_actor(&target, spawn)
                    .map_err(|error| error.to_string())?;
                queues.agent.0.push_back(AgentCommand::Teleport {
                    actor: target.clone(),
                    position: spawn,
                });
                spawn_healing_effect(
                    &mut ecs,
                    &render,
                    grid_to_world_on_surface(spawn, &config.0, &world.generated),
                    HealingEffectKind::Revive,
                    config.0.world.cell_size,
                );
                Ok(format!("revived {target} without a food cost"))
            }
            ChatCommand::GiveExperience { player, amount } => {
                require_game_master(&config.0, &pending)?;
                let target = resolve_player_id(&simulation.0, player)
                    .ok_or_else(|| format!("unknown player {player}"))?;
                let multiplier = simulation
                    .0
                    .actors
                    .get(&target)
                    .and_then(|actor| content.0.roles.get(&actor.role))
                    .map_or(1_000, |role| role.experience_multiplier_per_thousand);
                let levels = simulation
                    .0
                    .grant_role_experience(&target, *amount, multiplier)
                    .map_err(|error| error.to_string())?;
                Ok(format!("gave {target} {amount} experience; {levels} levels gained"))
            }
            ChatCommand::GiveExperienceAll(amount) => {
                require_game_master(&config.0, &pending)?;
                let players = simulation
                    .0
                    .actors
                    .values()
                    .filter(|actor| actor.role.as_str() != "role:enemy")
                    .map(|actor| actor.id.clone())
                    .collect::<Vec<_>>();
                for player in &players {
                    let multiplier = simulation
                        .0
                        .actors
                        .get(player)
                        .and_then(|actor| content.0.roles.get(&actor.role))
                        .map_or(1_000, |role| role.experience_multiplier_per_thousand);
                    simulation
                        .0
                        .grant_role_experience(player, *amount, multiplier)
                        .map_err(|error| error.to_string())?;
                }
                Ok(format!("gave {} players {amount} experience", players.len()))
            }
            ChatCommand::LevelUpPlayer { player, amount } => {
                require_game_master(&config.0, &pending)?;
                let target = resolve_player_id(&simulation.0, player)
                    .ok_or_else(|| format!("unknown player {player}"))?;
                let gained = simulation
                    .0
                    .grant_role_levels(&target, *amount)
                    .map_err(|error| error.to_string())?;
                Ok(format!("leveled {target} by {gained}"))
            }
            ChatCommand::GivePet { player, pet } => {
                require_game_master(&config.0, &pending)?;
                let target = resolve_player_id(&simulation.0, player)
                    .ok_or_else(|| format!("unknown player {player}"))?;
                let pet = prefixed_id(pet, "pet:")
                    .filter(|pet| {
                        matches!(
                            pet.as_str(),
                            "pet:none"
                                | "pet:redpanda"
                                | "pet:red_panda"
                                | "pet:fishgod"
                                | "pet:fish_god"
                                | "pet:giraffe"
                                | "pet:duck"
                                | "pet:butterfly"
                        )
                    })
                    .map(|pet| match pet.as_str() {
                        "pet:redpanda" => StableId::new("pet:red_panda").expect("static ID"),
                        "pet:fishgod" => StableId::new("pet:fish_god").expect("static ID"),
                        _ => pet,
                    })
                    .ok_or_else(|| format!("unknown pet {pet}"))?;
                simulation
                    .0
                    .unlock_pet(&target, pet.clone())
                    .map_err(|error| error.to_string())?;
                Ok(format!("unlocked {pet} for {target}"))
            }
            ChatCommand::QueueEvent(requested) => {
                require_game_master(&config.0, &pending)?;
                let event = match requested.as_str().trim_start_matches("event:") {
                    "fishgod" | "fish_god" => TownEvent::FishGod,
                    "monsterraid" | "monster_raid" | "raid" => TownEvent::EnemyRaid,
                    _ => return Err(format!("unsupported Unity queue event {requested}")),
                };
                if !simulation.0.queue_event(event.clone()) {
                    return Err(format!("{event:?} is already active or queued"));
                }
                Ok(format!("queued {event:?}"))
            }
            ChatCommand::StopEvent => {
                require_game_master(&config.0, &pending)?;
                if simulation.0.active_event.is_none() {
                    return Err("there is no active event".to_owned());
                }
                for enemy in simulation.0.stop_active_event() {
                    simulation.0.actors.remove(&enemy);
                    queues.agent.0.push_back(AgentCommand::Despawn(enemy));
                }
                Ok("stopped the current event".to_owned())
            }
            ChatCommand::CompleteObjective => {
                require_game_master(&config.0, &pending)?;
                simulation
                    .0
                    .force_complete_first_goal()
                    .map(|technology| format!("completed the goal for {technology}"))
                    .ok_or_else(|| "there is no active technology goal".to_owned())
            }
            ChatCommand::RandomTechnology => {
                require_game_master(&config.0, &pending)?;
                if simulation.0.active_goals.len() >= MAX_TOWN_GOALS {
                    return Err("the town already has the maximum active goals".to_owned());
                }
                let technology = eligible_technology_ids(&content.0, &simulation.0)
                    .into_iter()
                    .next()
                    .ok_or_else(|| "there are no eligible technologies".to_owned())?;
                let node = &content.0.technology.nodes[&technology];
                if !simulation.0.start_technology_goal(
                    technology.clone(),
                    &node.objectives,
                    &content.0.objectives,
                    MAX_TOWN_GOALS,
                ) {
                    return Err(format!("could not start {}", node.display_name));
                }
                Ok(format!("started technology goal {}", node.display_name))
            }
            ChatCommand::TechnologyVote => {
                require_game_master(&config.0, &pending)?;
                let technology = eligible_technology_ids(&content.0, &simulation.0)
                    .into_iter()
                    .next()
                    .ok_or_else(|| "there are no eligible technologies".to_owned())?;
                let name = content.0.technology.nodes[&technology].display_name.clone();
                simulation
                    .0
                    .start_technology_vote(technology, 60.0)
                    .map_err(|error| error.to_string())?;
                Ok(format!("started a 60-second technology vote for {name}"))
            }
            ChatCommand::GameEventAction => {
                require_game_master(&config.0, &pending)?;
                if simulation.0.fish_god.is_none() {
                    return Err("the current event has no game-master action".to_owned());
                }
                simulation
                    .0
                    .action_fish_god()
                    .map(|completed| {
                        if completed {
                            "completed the Fish God event action".to_owned()
                        } else {
                            let event = simulation.0.fish_god.as_ref().expect("event remains active");
                            format!(
                                "Fish God action {}/{}",
                                event.praises_given, event.praises_required
                            )
                        }
                    })
                    .map_err(|error| error.to_string())
            }
            ChatCommand::UnlockAllTechnology => {
                require_game_master(&config.0, &pending)?;
                let count = unlock_reachable_technologies(&content.0, &mut simulation.0, false);
                Ok(format!("unlocked {count} reachable technologies"))
            }
            ChatCommand::UnlockAgeTwo => {
                require_game_master(&config.0, &pending)?;
                let count = unlock_reachable_technologies(&content.0, &mut simulation.0, true);
                Ok(format!("unlocked {count} Age 1 technologies"))
            }
            ChatCommand::ResetId { kind, value } => {
                require_game_master(&config.0, &pending)?;
                if kind.as_str() != "building" {
                    return Err("Unity resetid supports only building IDs".to_owned());
                }
                let building = building_definition_id(&content.0, value)?;
                let count = building_instance_ids(&content.0, &simulation.0, &building).len();
                Ok(format!(
                    "{count} {} instances already use stable IDs; no counter reset was required",
                    content.0.buildings[&building].display_name
                ))
            }
            ChatCommand::TriggerEvent(event) => {
                require_staff(&pending)?;
                town_event_from_id(event)
                    .ok_or_else(|| format!("unknown event {}", event.as_str()))
                    .and_then(|event| {
                        if event == TownEvent::EnemyRaid {
                            if simulation.0.active_raid.is_some() {
                                return Err("a raid is already active".to_owned());
                            }
                            let enemy = archetype_id_by_source(
                                &content.0,
                                ArchetypeKind::Enemy,
                                "Enemy_Minotaur.prefab",
                            )
                            .ok_or_else(|| "raid enemy archetype is unavailable".to_owned())?;
                            let boss = archetype_id_by_source(
                                &content.0,
                                ArchetypeKind::Enemy,
                                "Enemy_MinotaurBoss.prefab",
                            )
                            .ok_or_else(|| "raid boss archetype is unavailable".to_owned())?;
                            if !simulation.0.start_raid(5, 50, enemy, boss) {
                                return Err("raid settings are invalid".to_owned());
                            }
                        } else if event == TownEvent::FishGod {
                            if !simulation.0.start_fish_god(true) {
                                return Err("another event is active".to_owned());
                            }
                        } else {
                            simulation.0.trigger_event(event);
                        }
                        Ok("event started".to_owned())
                    })
            }
            ChatCommand::Revive(requested) => {
                let self_revive = requested.is_none();
                let target_id = requested
                    .as_ref()
                    .map_or_else(|| Some(actor_id.clone()), |requested| prefixed_id(requested, "twitch:"))
                    .ok_or_else(|| "invalid revive target".to_owned());
                target_id.and_then(|target_id| {
                    if !self_revive {
                        let role = simulation
                            .0
                            .actors
                            .get(&actor_id)
                            .map(|actor| actor.role.as_str())
                            .ok_or_else(|| "join before reviving another player".to_owned())?;
                        if !matches!(role, "role:priest" | "role:paladin") {
                            return Err(
                                "only a Priest or Paladin can revive another player".to_owned(),
                            );
                        }
                        if target_id == actor_id {
                            return Err("use !revive without a target to revive yourself".to_owned());
                        }
                    }
                    let position = simulation
                        .0
                        .actors
                        .get(&target_id)
                        .map(|actor| actor.position)
                        .ok_or_else(|| format!("unknown player {target_id}"))?;
                    let spawn = nearest_walkable(&world.generated, position).unwrap_or(position);
                    let maximum_health = simulation
                        .0
                        .actors
                        .get(&target_id)
                        .map_or(0, |actor| {
                            u32::try_from(actor.max_health.max(0)).unwrap_or(u32::MAX)
                        });
                    simulation
                        .0
                        .revive_actor_with_food_cost(
                            &target_id,
                            spawn,
                            if self_revive { 400 } else { 200 },
                        )
                        .map_err(|error| error.to_string())?;
                    spawn_healing_effect(
                        &mut ecs,
                        &render,
                        grid_to_world_on_surface(spawn, &config.0, &world.generated),
                        HealingEffectKind::Revive,
                        config.0.world.cell_size,
                    );
                    if !self_revive {
                        let experience_multiplier = content
                            .0
                            .roles
                            .get(&simulation.0.actors[&actor_id].role)
                            .map_or(1_000, |role| role.experience_multiplier_per_thousand);
                        let _ = simulation.0.grant_role_experience(
                            &actor_id,
                            maximum_health,
                            experience_multiplier,
                        );
                    }
                    Ok(format!("revived {target_id}"))
                })
            }
            ChatCommand::Praise => {
                if !simulation.0.actors.contains_key(&actor_id) {
                    Err("join before praising the Fish God".to_owned())
                } else if simulation.0.fish_god.is_none() {
                    if simulation.0.start_fish_god(false) {
                        simulation
                            .0
                            .praise_fish_god(&actor_id)
                            .map(|_| "the Fish God answered; praise accepted (1/20)".to_owned())
                            .map_err(|error| error.to_string())
                    } else if simulation.0.active_event.is_some() {
                        Err("another event is active".to_owned())
                    } else {
                        Ok("the Fish God did not answer this praise".to_owned())
                    }
                } else {
                    simulation
                        .0
                        .praise_fish_god(&actor_id)
                        .map(|completed| {
                            if completed {
                                "the Fish God was pleased: the town received 1,000 food".to_owned()
                            } else {
                                let event = simulation
                                    .0
                                    .fish_god
                                    .as_ref()
                                    .expect("incomplete praise retains event");
                                format!(
                                    "Fish God praise {}/{}",
                                    event.praises_given, event.praises_required
                                )
                            }
                        })
                        .map_err(|error| error.to_string())
                }
            }
            ChatCommand::Experience => simulation
                .0
                .actors
                .get(&actor_id)
                .ok_or_else(|| "join before checking experience".to_owned())
                .and_then(|actor| {
                    effective_role_stats(&content.0, &simulation.0, actor)
                        .map(|stats| {
                            format!(
                                "{} level {}/99, experience {}/{}",
                                actor.role,
                                stats.level,
                                stats.experience,
                                stats.required_experience
                            )
                        })
                        .ok_or_else(|| format!("{} has no authored progression", actor.role))
                }),
            ChatCommand::Save => {
                require_ruler_or_staff(&simulation.0, &pending)?;
                let snapshot = snapshot_world(&world, &stats, &simulation, &agents);
                save.store
                    .write(&snapshot)
                    .map(|()| "town saved".to_owned())
                    .map_err(|error| format!("save failed: {error}"))
            }
            ChatCommand::Help => Ok(
                "commands: !join, !role [role], !roles, !health, !experience/!level [role], !station [id], !target [id], !stuck, !ping, !pets/!pet [pet], !hair/!eyes/!facialhair/!body/!haircolor/!eyecolor <id>, !build <type>, !move/!up/!down/!left/!right/!rotate, !confirm/!accept/!cancel, !buildings/!bid, !level <building> <id> [times], !levelall <building> <level>, !remove <building> <id>, !upgrade, !info, !townstats, !stdiscord, !buy/!sell (Ruler), !recruit/!recruits/!rid/!rinfo/!rrole/!rdismiss (Ruler), !cam/!resetcam (Ruler), !modrole/!rulervote/!event (staff), !resign, !revive, !praise, !vote, !save, !help; GM: !tbuildcosts/!trolelimits/!addresource/!kill/!grevive/!givexp/!givexpall/!levelup/!givepet/!qevent/!stopevent/!cobj/!randtech/!techvote/!gaction/!unlockall/!unlockage2/!resetid"
                    .to_owned(),
            ),
            }
        })();
        let message = match result {
            Ok(message) => message,
            Err(error) => format!("command rejected: {error}"),
        };
        feedback.0 = format!("{}: {message}", pending.display_name);
        send_command_feedback(&connection, &pending.display_name, &message);
        info!(user = %pending.display_name, ?command, result = %message, "processed Twitch command");
        stats.commands_processed += 1;
    }
}

fn update_hud(
    stats: Res<SessionStats>,
    twitch: Res<TwitchConnection>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    feedback: Res<CommandFeedback>,
    agents: Query<&Agent>,
    mut hud: Single<&mut Text, With<Hud>>,
) {
    if !stats.is_changed()
        && !twitch.is_changed()
        && !feedback.is_changed()
        && !simulation.is_changed()
    {
        return;
    }
    let first_id = agents
        .iter()
        .next()
        .map_or("none", |agent| agent.id.as_str());
    let gathering = agents
        .iter()
        .filter(|agent| matches!(agent.goal, AgentGoal::Gather(_)))
        .count();
    let depositing = agents
        .iter()
        .filter(|agent| agent.goal == AgentGoal::Deposit)
        .count();
    let attacking = agents
        .iter()
        .filter(|agent| {
            matches!(
                agent.goal,
                AgentGoal::Attack(_) | AgentGoal::AttackBuilding(_)
            )
        })
        .count();
    let healing = agents
        .iter()
        .filter(|agent| matches!(agent.goal, AgentGoal::Heal(_)))
        .count();
    let constructing = agents
        .iter()
        .filter(|agent| matches!(agent.goal, AgentGoal::Construct(_)))
        .count();
    let incomplete_buildings = simulation
        .0
        .buildings
        .values()
        .filter(|building| !building.complete)
        .count();
    let building_levels = simulation
        .0
        .buildings
        .values()
        .map(|building| u64::from(building.level))
        .sum::<u64>();
    let dead = simulation
        .0
        .actors
        .values()
        .filter(|actor| !actor.alive)
        .count();
    hud.0 = format!(
        "{} agents | {:.0}s | {} routes | workers {gathering} gather/{depositing} deposit/{constructing} build | buildings {incomplete_buildings} construction/{building_levels} levels | combat {attacking} attack/{healing} heal/{dead} dead | {} commands | {:?} / {:?} | Twitch: {}\nResources F:{} G:{} O:{} W:{} R:{} | Goals: {} | Event: {} | Governance: {} | {}\nF1 Twitch Off | F2 Twitch On | F5 Save | F9 Load | F12 Capture | J Inject !join | WASD Pan | Q/E Zoom | Click Select | ESC Menu | first {first_id}",
        agents.iter().len(),
        stats.elapsed_seconds,
        stats.paths_completed,
        stats.commands_processed,
        simulation.0.season,
        simulation.0.weather,
        twitch_status_text(&twitch),
        town_resource_amount(&simulation.0, "resource:food"),
        town_resource_amount(&simulation.0, "resource:gold"),
        town_resource_amount(&simulation.0, "resource:ore"),
        town_resource_amount(&simulation.0, "resource:wood"),
        town_resource_amount(&simulation.0, "resource:recruit"),
        town_goal_status(&content.0, &simulation.0),
        active_event_text(&simulation.0),
        ruler_status(&simulation.0),
        feedback.0,
    );
}

fn town_goal_status(content: &ContentCatalog, simulation: &WorldSimulation) -> String {
    let Some(goal) = simulation.active_goals.first() else {
        return "none".to_owned();
    };
    let name = content
        .technology
        .nodes
        .get(&goal.technology)
        .map_or(goal.technology.as_str(), |technology| {
            technology.display_name.as_str()
        });
    let progress = goal
        .objectives
        .iter()
        .filter_map(|progress| {
            content
                .objectives
                .get(&progress.objective)
                .map(|definition| {
                    format!(
                        "{:?} {}/{}",
                        definition.kind, progress.amount, progress.required_amount
                    )
                })
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("{name} [{progress}]")
}

fn town_resource_amount(simulation: &WorldSimulation, resource: &str) -> u32 {
    StableId::new(resource)
        .ok()
        .and_then(|resource| simulation.town_resources.get(&resource).copied())
        .unwrap_or_default()
}

fn active_event_text(simulation: &WorldSimulation) -> String {
    if let Some(raid) = &simulation.active_raid {
        format!(
            "raid wave {}/{} ({} enemies)",
            raid.current_wave,
            raid.total_waves,
            raid.tracked_enemies.len()
        )
    } else if let Some(event) = &simulation.fish_god {
        format!(
            "Fish God {}/{} ({:.0}s)",
            event.praises_given, event.praises_required, event.remaining_seconds
        )
    } else {
        simulation
            .active_event
            .as_ref()
            .map_or_else(|| "none".to_owned(), |event| format!("{event:?}"))
    }
}

fn ruler_status(simulation: &WorldSimulation) -> String {
    if let Some(vote) = &simulation.ruler_vote {
        let options = vote
            .option_order
            .iter()
            .map(|option| {
                let count = vote
                    .votes
                    .values()
                    .filter(|selected| *selected == option)
                    .count();
                let label = simulation
                    .actors
                    .get(option)
                    .and_then(|actor| actor.display_name.as_deref())
                    .unwrap_or(option.as_str());
                format!("{label} {count}")
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "{:?} {:.0}s [{}]",
            vote.kind, vote.remaining_seconds, options
        )
    } else if let Some(ruler) = &simulation.current_ruler {
        let name = simulation
            .actors
            .get(ruler)
            .and_then(|actor| actor.display_name.as_deref())
            .unwrap_or(ruler.as_str());
        format!(
            "Ruler {name}; retention vote in {:.0}s",
            simulation.ruler_vote_cooldown_seconds
        )
    } else {
        format!(
            "no ruler; election in {:.0}s",
            simulation.ruler_vote_cooldown_seconds
        )
    }
}

fn twitch_status_text(connection: &TwitchConnection) -> String {
    if matches!(connection.status, TwitchStatus::Connected) && !connection.broadcaster_authorized {
        format!("awaiting broadcaster !connect {}", connection.connect_code)
    } else {
        match &connection.status {
            TwitchStatus::Error(error) => format!("error: {error}"),
            status => format!("{status:?}"),
        }
    }
}

fn snapshot_world(
    world: &WorldRuntime,
    stats: &SessionStats,
    simulation: &SimulationRuntime,
    agents: &Query<(&Agent, &GridLocation)>,
) -> WorldSnapshot {
    WorldSnapshot {
        schema_version: 2,
        world_seed: world.generated.seed,
        generator_version: world.generated.generator_version,
        world_hash: world.generated.deterministic_hash.clone(),
        elapsed_seconds: Duration::from_secs_f64(stats.elapsed_seconds.max(0.0)).as_secs(),
        actors: agents
            .iter()
            .map(|(agent, location)| SavedActor {
                id: agent.id.clone(),
                kind: agent.kind.clone(),
                archetype: agent.archetype.clone(),
                grid_position: location.0,
                height_centimetres: world
                    .generated
                    .navigation
                    .height_at(location.0)
                    .unwrap_or_default(),
                health: simulation
                    .0
                    .actors
                    .get(&agent.id)
                    .map_or(100, |actor| actor.health),
            })
            .collect(),
        simulation: simulation.0.clone(),
        resource_nodes: world
            .generated
            .resources
            .iter()
            .map(|resource| (resource.id.clone(), resource.amount))
            .collect(),
        legacy_terrain_mesh: world.legacy_terrain_mesh.clone(),
        legacy_migration: world.legacy_migration.clone(),
    }
}

fn spawn_credits(mut commands: Commands, render: Res<RenderAssets>) {
    commands.insert_resource(CreditsTimeline::default());
    spawn_cloud_field(&mut commands, &render, 55.0);
    commands.spawn((
        StateEntity,
        Text::new("STREAM TOWN"),
        TextFont {
            font_size: FontSize::Px(54.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::srgb(0.86, 0.95, 0.84)),
        GlobalZIndex(10),
        Node {
            position_type: PositionType::Absolute,
            top: percent(12.0),
            left: percent(37.0),
            ..default()
        },
    ));
    for (target_path, text) in [
        ("CreatedBy_Canvas", "Created By\nHuman Bean Games"),
        ("ProjectLead_Canvas", "Project Lead\nJayden Hunter"),
        (
            "3DArt_Canvas",
            "Lead Artist\nBen Lindridge\n\n3D Artists\nAaron Hunter | Bella Kovac | William Trimble",
        ),
        (
            "OtherArt_Canvas",
            "Producer and Game Design\nAndrew Nardi | Benjamin Babicka | Jayden Hunter",
        ),
        (
            "AudioAndSpecialThanks_Canvas",
            "Composer\nLeah Crimmins\n\nAudio Design\nFrancisco Inigo Garde | Andrew Nardi\n\nSpecial Thanks\nStream Town community",
        ),
        (
            "Progammer_Canvas",
            "Lead Programmer\nJayden Hunter\n\nProgrammers\nDeclan Doller | Benjamin Babicka",
        ),
        (
            "Design_Canvas",
            "Animation, Technical Art, UI and VFX\nAaron Hunter | Lam Nguyen | Ben Lindridge | Benjamin Babicka | Andrew Nardi",
        ),
        ("VFX_FireWorks", "✦   ✧   ✦"),
        ("VFX_FireWorks (1)", "✧   ✦   ✧"),
    ] {
        commands.spawn((
            StateEntity,
            AuthoredCreditsElement {
                target_path: target_path.into(),
            },
            Text::new(text),
            TextFont {
                font_size: FontSize::Px(if target_path.starts_with("VFX_") {
                    64.0
                } else {
                    32.0
                }),
                ..default()
            },
            TextLayout::justify(Justify::Center),
            TextColor(Color::srgb(0.86, 0.95, 0.84)),
            GlobalZIndex(10),
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                top: percent(if target_path.starts_with("VFX_") {
                    68.0
                } else {
                    31.0
                }),
                left: percent(if target_path == "VFX_FireWorks" {
                    18.0
                } else if target_path == "VFX_FireWorks (1)" {
                    67.0
                } else {
                    20.0
                }),
                width: percent(if target_path.starts_with("VFX_") {
                    18.0
                } else {
                    60.0
                }),
                ..default()
            },
        ));
    }
    commands.spawn((
        StateEntity,
        Text::new("ESC  Main Menu"),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::srgb(0.65, 0.72, 0.66)),
        GlobalZIndex(10),
        Node {
            position_type: PositionType::Absolute,
            bottom: percent(5.0),
            left: percent(45.0),
            ..default()
        },
    ));
    commands.spawn((
        StateEntity,
        CreditsFade,
        GlobalZIndex(20),
        BackgroundColor(Color::BLACK),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100.0),
            height: percent(100.0),
            ..default()
        },
    ));
}

fn drive_credits_animation(
    time: Res<Time>,
    presentation: Res<RuntimePresentation>,
    mut timeline: ResMut<CreditsTimeline>,
    mut elements: Query<(&AuthoredCreditsElement, &mut Visibility)>,
    mut fades: Query<&mut BackgroundColor, With<CreditsFade>>,
) {
    timeline.elapsed_seconds = (timeline.elapsed_seconds + time.delta_secs()).min(65.5);
    for (element, mut visibility) in &mut elements {
        let clip = if element.target_path.starts_with("VFX_") {
            "CreditsFireworks"
        } else {
            "CreditsPopups"
        };
        let active = animation_property_value(
            &presentation.0,
            clip,
            &element.target_path,
            "m_IsActive",
            timeline.elapsed_seconds,
        )
        .is_some_and(|value| value >= 0.5);
        *visibility = if active {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    let alpha = animation_property_value(
        &presentation.0,
        "EndCredits",
        "Panel",
        "m_Color.a",
        timeline.elapsed_seconds,
    )
    .unwrap_or(0.0)
    .clamp(0.0, 1.0);
    for mut background in &mut fades {
        background.0.set_alpha(alpha);
    }
}

fn animation_property_value(
    presentation: &PresentationCatalog,
    clip_name: &str,
    target_path: &str,
    attribute: &str,
    time: f32,
) -> Option<f32> {
    presentation
        .clips
        .values()
        .find(|clip| clip.display_name == clip_name)?
        .property_curves
        .iter()
        .find(|curve| curve.target_path == target_path && curve.attribute == attribute)?
        .sample(time)
}

fn spawn_level_up_toast(mut commands: Commands, simulation: Res<SimulationRuntime>) {
    let actor_levels = simulation
        .0
        .actors
        .iter()
        .map(|(id, actor)| (id.clone(), role_progress(actor).level))
        .collect();
    commands.insert_resource(LevelUpPresentation {
        actor_levels,
        elapsed_seconds: None,
    });
    commands.spawn((
        WorldEntity,
        LevelUpToast,
        Text::new("LEVEL UP"),
        TextFont {
            font_size: FontSize::Px(32.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::WHITE.with_alpha(0.0)),
        Visibility::Hidden,
        Node {
            position_type: PositionType::Absolute,
            top: percent(18.0),
            left: percent(44.0),
            ..default()
        },
    ));
}

fn drive_level_up_presentation(
    time: Res<Time>,
    presentation: Res<RuntimePresentation>,
    simulation: Res<SimulationRuntime>,
    mut state: ResMut<LevelUpPresentation>,
    mut toast: Query<(&mut Text, &mut TextColor, &mut Visibility, &mut Node), With<LevelUpToast>>,
) {
    let mut leveled_actor = None;
    for (id, actor) in &simulation.0.actors {
        let level = role_progress(actor).level;
        if state
            .actor_levels
            .insert(id.clone(), level)
            .is_some_and(|previous| level > previous)
        {
            leveled_actor = Some((id.clone(), level));
        }
    }
    if leveled_actor.is_some() {
        state.elapsed_seconds = Some(0.0);
    }
    let Ok((mut text, mut color, mut visibility, mut node)) = toast.single_mut() else {
        return;
    };
    if let Some((actor, level)) = leveled_actor {
        **text = format!("LEVEL UP\n{actor} | {level}");
    }
    let Some(elapsed) = state.elapsed_seconds.as_mut() else {
        *visibility = Visibility::Hidden;
        return;
    };
    *elapsed += time.delta_secs();
    if *elapsed > 1.0 {
        state.elapsed_seconds = None;
        *visibility = Visibility::Hidden;
        return;
    }
    let y = animation_property_value(
        &presentation.0,
        "LevelUp_Animation",
        "LevelUpText",
        "m_AnchoredPosition.y",
        *elapsed,
    )
    .unwrap_or(0.0);
    let alpha = animation_property_value(
        &presentation.0,
        "LevelUp_Animation",
        "LevelUpText",
        "m_fontColor.a",
        *elapsed,
    )
    .unwrap_or(1.0)
    .clamp(0.0, 1.0);
    node.top = percent(18.0 - y);
    color.0.set_alpha(alpha);
    *visibility = Visibility::Inherited;
}

fn credits_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

fn cleanup_state_entities(mut commands: Commands, entities: Query<Entity, With<StateEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn cleanup_world(mut commands: Commands, entities: Query<Entity, With<WorldEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<WorldRuntime>();
    commands.remove_resource::<SimulationRuntime>();
    commands.insert_resource(BuildingPlacers::default());
    commands.insert_resource(BuildingCommandQueue::default());
}

fn grid_to_world(position: GridPos, config: &GameConfig) -> Vec3 {
    Vec3::new(
        (f32::from(position.x) - f32::from(config.world.width) * 0.5) * config.world.cell_size,
        0.0,
        (f32::from(position.z) - f32::from(config.world.height) * 0.5) * config.world.cell_size,
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
    let best = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .filter(|position| !world.navigation.is_walkable(*position))
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
            .any(|neighbor| world.navigation.is_walkable(neighbor))
        })
        .min_by_key(|position| position.x.abs_diff(centre.x) + position.z.abs_diff(centre.z))
        .unwrap_or(centre);
    let water_height = f32::from(config.world.water_level_centimetres) * 0.01;
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

fn generated_terrain_mesh(world: &GeneratedWorld, config: &GameConfig) -> Mesh {
    let width = world.navigation.width();
    let height = world.navigation.height();
    let columns = u32::from(width) + 1;
    let mut positions = Vec::with_capacity(usize::from(width + 1) * usize::from(height + 1));
    let mut colors = Vec::with_capacity(positions.capacity());
    let mut uvs = Vec::with_capacity(positions.capacity());
    for z in 0..=height {
        for x in 0..=width {
            let elevation = terrain_corner_height(world, x, z);
            positions.push([
                (f32::from(x) - f32::from(width) * 0.5) * config.world.cell_size,
                elevation,
                (f32::from(z) - f32::from(height) * 0.5) * config.world.cell_size,
            ]);
            colors.push(terrain_vertex_color(elevation, config));
            uvs.push([
                f32::from(x) / f32::from(width),
                f32::from(z) / f32::from(height),
            ]);
        }
    }

    let mut indices = Vec::with_capacity(usize::from(width) * usize::from(height) * 6);
    for z in 0..u32::from(height) {
        for x in 0..u32::from(width) {
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

fn generated_water_mesh(world: &GeneratedWorld, config: &GameConfig) -> Mesh {
    const OCEAN_PADDING_CELLS: u16 = 8;
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
                (f32::from(x) - f32::from(OCEAN_PADDING_CELLS) - f32::from(width) * 0.5)
                    * config.world.cell_size,
                water_height + 0.05,
                (f32::from(z) - f32::from(OCEAN_PADDING_CELLS) - f32::from(height) * 0.5)
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
    let min_x = corner_x.saturating_sub(1);
    let max_x = corner_x.min(world.navigation.width() - 1);
    let min_z = corner_z.saturating_sub(1);
    let max_z = corner_z.min(world.navigation.height() - 1);
    let mut total = 0.0_f32;
    let mut samples = 0_u16;
    for z in min_z..=max_z {
        for x in min_x..=max_x {
            total += f32::from(
                world
                    .navigation
                    .height_at(GridPos { x, z })
                    .unwrap_or_default(),
            );
            samples += 1;
        }
    }
    let average = total / f32::from(samples.max(1));
    average * 0.01
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
    let x = (position.x / config.world.cell_size + f32::from(config.world.width) * 0.5).floor();
    let z = (position.z / config.world.cell_size + f32::from(config.world.height) * 0.5).floor();
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

#[cfg(test)]
mod tests {
    use super::*;
    use stream_town_domain::generate_world;

    #[test]
    fn production_crowd_lod_budget_matches_measured_gpu_gate() {
        assert_eq!(actor_detail_budget(None), 16);
        assert_eq!(actor_detail_budget(Some("24")), 24);
        assert_eq!(actor_detail_budget(Some("invalid")), 16);
    }

    #[test]
    fn structural_footprints_clear_and_restore_foliage_visibility() {
        let config = GameConfig::default();
        let content = embedded_content();
        let generated = generate_world(&config.world);
        let building_id = StableId::new("building:foliage_test").unwrap();
        let house = &content.buildings[&StableId::new("building:house").unwrap()];
        let building_position = GridPos { x: 18, z: 18 };
        let rotation_quarter_turns = 1;
        let building_footprint = rotated_footprint(house.footprint, rotation_quarter_turns);
        let building_foliage = GridPos {
            x: building_position.x + building_footprint[0] - 1,
            z: building_position.z + building_footprint[1] - 1,
        };
        let (camp_archetype_id, camp_archetype) = content
            .archetypes
            .iter()
            .find(|(_, archetype)| archetype.enemy_spawner.is_some())
            .unwrap();
        let camp_id = StableId::new("enemy_camp:foliage_test").unwrap();
        let camp_position = GridPos { x: 42, z: 42 };
        let camp_foliage = GridPos {
            x: camp_position.x + camp_archetype.footprint[0] - 1,
            z: camp_position.z + camp_archetype.footprint[1] - 1,
        };
        let outside_foliage = GridPos { x: 2, z: 2 };
        let mut simulation = WorldSimulation::new(generated.seed);
        simulation.buildings.insert(
            building_id.clone(),
            BuildingState {
                id: building_id.clone(),
                archetype: house.archetype.clone(),
                position: building_position,
                rotation_quarter_turns,
                level: 1,
                health: BUILDING_MAX_HEALTH,
                complete: true,
            },
        );
        simulation.enemy_camps.insert(
            camp_id.clone(),
            EnemyCampState {
                id: camp_id.clone(),
                archetype: camp_archetype_id.clone(),
                position: camp_position,
                health: 100,
                spawn_remaining_seconds: 0.0,
                spawned_enemies: BTreeSet::new(),
            },
        );

        let mut app = App::new();
        app.insert_resource(RuntimeContent(content));
        app.insert_resource(SimulationRuntime(simulation));
        app.insert_resource(WorldRuntime {
            generated,
            legacy_terrain_mesh: None,
            legacy_migration: None,
        });
        app.add_systems(Update, sync_foliage_clearance);
        let building_entity = app
            .world_mut()
            .spawn((
                FoliageVisual,
                GridLocation(building_foliage),
                Visibility::Inherited,
            ))
            .id();
        let camp_entity = app
            .world_mut()
            .spawn((
                FoliageVisual,
                GridLocation(camp_foliage),
                Visibility::Inherited,
            ))
            .id();
        let outside_entity = app
            .world_mut()
            .spawn((
                FoliageVisual,
                GridLocation(outside_foliage),
                Visibility::Inherited,
            ))
            .id();

        app.update();
        assert_eq!(
            app.world().get::<Visibility>(building_entity),
            Some(&Visibility::Hidden)
        );
        assert_eq!(
            app.world().get::<Visibility>(camp_entity),
            Some(&Visibility::Hidden)
        );
        assert_eq!(
            app.world().get::<Visibility>(outside_entity),
            Some(&Visibility::Inherited)
        );

        {
            let mut simulation = app.world_mut().resource_mut::<SimulationRuntime>();
            simulation.0.buildings.remove(&building_id);
        }
        app.update();
        assert_eq!(
            app.world().get::<Visibility>(building_entity),
            Some(&Visibility::Inherited)
        );
        assert_eq!(
            app.world().get::<Visibility>(camp_entity),
            Some(&Visibility::Hidden)
        );

        app.world_mut()
            .resource_mut::<SimulationRuntime>()
            .0
            .enemy_camps
            .remove(&camp_id);
        app.update();
        assert_eq!(
            app.world().get::<Visibility>(camp_entity),
            Some(&Visibility::Inherited)
        );
    }

    #[test]
    fn workers_choose_nearest_compatible_station_and_reassign() {
        let config = GameConfig::default();
        let content = embedded_content();
        let logger = StableId::new("role:logger").unwrap();
        let actor_id = StableId::new("npc:station_test").unwrap();
        let station_id = StableId::new("building:runtime_station").unwrap();
        let lumbermill = &content.buildings[&StableId::new("building:lumbermill").unwrap()];
        let position = GridPos { x: 10, z: 10 };
        let mut simulation = WorldSimulation::new(config.world.seed);
        assert!(simulation.join_player(actor_id.clone(), position));
        simulation.assign_role(&actor_id, logger).unwrap();
        simulation.buildings.insert(
            station_id.clone(),
            BuildingState {
                id: station_id.clone(),
                archetype: lumbermill.archetype.clone(),
                position: GridPos { x: 11, z: 10 },
                rotation_quarter_turns: 0,
                level: 1,
                health: BUILDING_MAX_HEALTH,
                complete: true,
            },
        );

        ensure_actor_station(&content, &mut simulation, &config, &actor_id);
        assert_eq!(
            simulation.actors[&actor_id].station,
            Some(station_id.clone())
        );

        simulation.buildings.get_mut(&station_id).unwrap().complete = false;
        ensure_actor_station(&content, &mut simulation, &config, &actor_id);
        assert_eq!(
            simulation.actors[&actor_id]
                .station
                .as_ref()
                .map(StableId::as_str),
            Some("building:townhall")
        );
    }

    #[test]
    fn equipment_visibility_matches_role_and_carry_state() {
        let content = embedded_content();
        let logger = content.roles[&StableId::new("role:logger").unwrap()]
            .equipment
            .as_ref()
            .unwrap();
        assert!(equipment_node_visible(logger, 0, "Body_Logger_Slim", false));
        assert!(equipment_node_visible(
            logger,
            2,
            "Body_Logger_Feminine",
            false
        ));
        assert!(equipment_node_visible(
            logger,
            0,
            "RHand_LoggerToolAxe",
            false
        ));
        assert!(!equipment_node_visible(
            logger,
            0,
            "LHand_LoggerCarryWood",
            false
        ));
        assert!(equipment_node_visible(
            logger,
            0,
            "LHand_LoggerCarryWood",
            true
        ));

        let defender = content.roles[&StableId::new("role:defender").unwrap()]
            .equipment
            .as_ref()
            .unwrap();
        assert!(equipment_node_visible(
            defender,
            0,
            "LHand_DefenderToolShield",
            false
        ));
        assert!(equipment_node_visible(
            defender,
            0,
            "Helmet_Defender",
            false
        ));
        assert!(!equipment_node_visible(
            defender,
            0,
            "Body_Logger_Slim",
            false
        ));
    }

    #[test]
    fn town_hall_is_a_stable_authoritative_building_and_ages_with_technology() {
        let config = GameConfig::default();
        let content = embedded_content();
        let town_hall_id = StableId::new("building:townhall").unwrap();
        let definition = &content.buildings[&town_hall_id];
        let mut simulation = WorldSimulation::new(config.world.seed);

        ensure_town_hall_state(&content, &config, &mut simulation);
        let town_hall = &simulation.buildings[&town_hall_id];
        assert_eq!(town_hall.id, town_hall_id);
        assert_eq!(town_hall.archetype, definition.archetype);
        assert_eq!(
            town_hall.position,
            town_hall_placement_position(&config, definition.footprint)
        );
        assert_eq!(town_hall.level, 1);
        assert_eq!(town_hall.health, BUILDING_MAX_HEALTH);
        assert!(town_hall.complete);
        assert_eq!(constructed_building_count(&simulation), 0);
        assert_eq!(building_age(&content, &simulation, &town_hall_id), 1);

        let age_technology = content
            .technology
            .nodes
            .iter()
            .find(|(_, technology)| technology.aged_buildings.contains(&town_hall_id))
            .map(|(id, _)| id.clone())
            .expect("converted technology contains the Town Hall age unlock");
        simulation.unlocked_technology.insert(age_technology);
        assert_eq!(building_age(&content, &simulation, &town_hall_id), 2);
        assert_eq!(
            archetype_scene_for_age(&content.archetypes[&definition.archetype], 2)
                .and_then(|scene| scene.age),
            Some(2)
        );

        let saved = simulation.buildings[&town_hall_id].clone();
        ensure_town_hall_state(&content, &config, &mut simulation);
        assert_eq!(simulation.buildings.len(), 1);
        assert_eq!(simulation.buildings[&town_hall_id], saved);
    }

    #[test]
    fn live_agent_goals_select_authored_action_animation_contracts() {
        let content = embedded_content();
        let actor_id = StableId::new("npc:animation_test").unwrap();
        let mut simulation = WorldSimulation::new(42);
        assert!(simulation.join_player(actor_id.clone(), GridPos { x: 10, z: 10 }));
        let agent_for = |goal| Agent {
            id: actor_id.clone(),
            kind: ActorKind::Player,
            archetype: StableId::new("archetype:viewer").unwrap(),
            goal,
            spawn: GridPos { x: 10, z: 10 },
            origin: GridPos { x: 10, z: 10 },
            path: vec![GridPos { x: 10, z: 10 }],
            path_index: 1,
            target: GridPos { x: 10, z: 10 },
            action_cooldown_seconds: 0.75,
            health_regen_accumulator: 0.0,
        };

        for (role, goal, expected) in [
            (
                "role:logger",
                AgentGoal::Gather(StableId::new("resource:test").unwrap()),
                "WoodCutting",
            ),
            (
                "role:builder",
                AgentGoal::Construct(StableId::new("building:test").unwrap()),
                "Build",
            ),
            (
                "role:ranger",
                AgentGoal::Attack(StableId::new("actor:enemy").unwrap()),
                "BowShoot",
            ),
            (
                "role:priest",
                AgentGoal::Heal(StableId::new("actor:injured").unwrap()),
                "Heal",
            ),
        ] {
            simulation
                .assign_role(&actor_id, StableId::new(role).unwrap())
                .unwrap();
            assert_eq!(
                agent_action_animation(&content, &agent_for(goal), &simulation.actors[&actor_id])
                    .as_deref(),
                Some(expected)
            );
        }

        let mut moving = agent_for(AgentGoal::Gather(StableId::new("resource:test").unwrap()));
        moving.path_index = 0;
        moving.path[0] = GridPos { x: 11, z: 10 };
        assert_eq!(
            agent_action_animation(&content, &moving, &simulation.actors[&actor_id]),
            None
        );
        assert_eq!(deterministic_animation_variant(&actor_id, "BowShoot", 1), 0);
        assert_eq!(
            deterministic_animation_variant(&actor_id, "BowShoot", 4),
            deterministic_animation_variant(&actor_id, "BowShoot", 4)
        );
    }

    #[test]
    fn animation_events_fire_once_per_elapsed_clip_cycle() {
        assert_eq!(animation_event_occurrences(0.25, 1.0, None, 0.24), 0);
        assert_eq!(animation_event_occurrences(0.25, 1.0, None, 0.25), 1);
        assert_eq!(animation_event_occurrences(0.25, 1.0, Some(0.25), 1.24), 0);
        assert_eq!(animation_event_occurrences(0.25, 1.0, Some(1.24), 1.25), 1);
        assert_eq!(animation_event_occurrences(0.25, 1.0, Some(0.10), 3.10), 3);
        assert_eq!(animation_event_occurrences(0.0, 1.0, None, 0.0), 1);
        assert_eq!(animation_event_occurrences(0.25, 0.0, None, 1.0), 0);
    }

    #[test]
    fn converted_role_audio_events_have_deterministic_procedural_cues() {
        let presentation = embedded_presentation();
        let clips: Vec<_> = presentation
            .clips
            .iter()
            .filter(|(_, clip)| {
                clip.events
                    .iter()
                    .any(|event| event.function_name == "PlayRoleActionAudio")
            })
            .collect();
        assert_eq!(clips.len(), 10);
        for (id, clip) in clips {
            let frequency = procedural_role_action_frequency(&clip.display_name, id);
            assert!(frequency.is_finite());
            assert!((140.0..=660.0).contains(&frequency));
            assert_eq!(
                frequency.to_bits(),
                procedural_role_action_frequency(&clip.display_name, id).to_bits()
            );
        }
    }

    #[test]
    fn converted_state_crossfade_preserves_weights_and_finishes_at_destination() {
        let source = AnimationNodeIndex::new(1);
        let destination = AnimationNodeIndex::new(2);
        let mut crossfade = Some(ConvertedAnimationCrossfade {
            source: vec![(source, 1.0, 1.0)],
            elapsed: 0.0,
            duration: 0.25,
        });
        let desired = vec![(destination, 1.0, 2.0)];
        let half = advance_animation_crossfade(&mut crossfade, &desired, 0.125);
        assert_eq!(half.len(), 2);
        assert!((half[0].1 - 0.5).abs() < f32::EPSILON);
        assert!((half[1].1 - 0.5).abs() < f32::EPSILON);
        let finished = advance_animation_crossfade(&mut crossfade, &desired, 0.125);
        assert_eq!(finished, desired);
        assert!(crossfade.is_none());
    }

    #[test]
    fn converted_crossfade_uses_fixed_or_normalized_authored_duration() {
        let source = AnimationNodeIndex::new(1);
        let presentation = embedded_presentation();
        let controller = presentation.controllers.values().next().unwrap();
        let mut layer = ConvertedAnimationLayerDriver {
            display_name: "Base".into(),
            fallback_state: StableId::new("state:fallback").unwrap(),
            runtime: AnimationControllerRuntime::in_state(
                controller,
                controller.default_states[0].clone(),
            )
            .unwrap(),
            nodes: BTreeMap::new(),
            active: Vec::new(),
            applied: vec![(source, 1.0, 1.0)],
            crossfade: None,
            state_offset: 0.0,
            event_elapsed: BTreeMap::new(),
        };
        begin_animation_crossfade(
            &mut layer,
            AnimationTransitionPlayback {
                duration: 0.25,
                fixed_duration: false,
                destination_offset: 0.2,
            },
            2.0,
        );
        assert!((layer.crossfade.as_ref().unwrap().duration - 0.5).abs() < f32::EPSILON);
        assert!((layer.state_offset - 0.2).abs() < f32::EPSILON);
        begin_animation_crossfade(
            &mut layer,
            AnimationTransitionPlayback {
                duration: 0.25,
                fixed_duration: true,
                destination_offset: 0.0,
            },
            2.0,
        );
        assert!((layer.crossfade.as_ref().unwrap().duration - 0.25).abs() < f32::EPSILON);
    }

    #[test]
    fn cosmetic_nodes_preserve_unity_order_and_visibility_rules() {
        for (index, name) in EYE_NODES.iter().enumerate() {
            assert_eq!(
                cosmetic_node(name),
                Some((CosmeticNodeKind::Eyes, u8::try_from(index).unwrap()))
            );
        }
        for (index, name) in HAIR_NODES.iter().enumerate() {
            assert_eq!(
                cosmetic_node(name),
                Some((CosmeticNodeKind::Hair, u8::try_from(index).unwrap()))
            );
        }
        for (index, name) in FACIAL_HAIR_NODES.iter().enumerate() {
            assert_eq!(
                cosmetic_node(name),
                Some((CosmeticNodeKind::FacialHair, u8::try_from(index).unwrap()))
            );
        }
        assert_eq!(cosmetic_node("Helmet_Defender"), None);

        let customization = ActorCustomization {
            eyes: 3,
            hair: 6,
            facial_hair: 1,
            hair_color: 4,
            eye_color: 1,
            body_type: 2,
        };
        assert!(cosmetic_node_visible(
            customization,
            CosmeticNodeKind::Eyes,
            3,
            false
        ));
        assert!(!cosmetic_node_visible(
            customization,
            CosmeticNodeKind::Eyes,
            2,
            false
        ));
        assert!(cosmetic_node_visible(
            customization,
            CosmeticNodeKind::Hair,
            6,
            false
        ));
        assert!(!cosmetic_node_visible(
            customization,
            CosmeticNodeKind::Hair,
            6,
            true
        ));
        assert!(cosmetic_node_visible(
            customization,
            CosmeticNodeKind::FacialHair,
            1,
            true
        ));
        assert_eq!(
            cosmetic_color(customization, CosmeticNodeKind::Hair),
            (4, HAIR_COLORS[4])
        );
        assert_eq!(
            cosmetic_color(customization, CosmeticNodeKind::FacialHair),
            (4, HAIR_COLORS[4])
        );
        assert_eq!(
            cosmetic_color(customization, CosmeticNodeKind::Eyes),
            (1, EYE_COLORS[1])
        );
    }

    #[test]
    fn role_driven_resource_loop_depletes_and_deposits() {
        let config = GameConfig::default();
        let content = embedded_content();
        let mut world = generate_world(&config.world);
        let resource = world
            .resources
            .first()
            .expect("default world contains resources")
            .clone();
        let role = match resource.kind.as_str() {
            "resource:wood" => "role:logger",
            "resource:ore" => "role:miner",
            _ => "role:gatherer",
        };
        let actor_id = StableId::new("npc:test_worker").unwrap();
        let mut simulation = WorldSimulation::new(world.seed);
        assert!(simulation.join_player(actor_id.clone(), resource.position));
        simulation
            .assign_role(&actor_id, StableId::new(role).unwrap())
            .unwrap();
        let starting_amount = resource.amount;
        let gather = AgentGoal::Gather(resource.id.clone());
        for _ in 0..10 {
            complete_agent_goal(
                &mut simulation,
                &mut world,
                &config,
                &content,
                &actor_id,
                &gather,
                resource.position,
            );
        }
        assert_eq!(
            world
                .resources
                .iter()
                .find(|candidate| candidate.id == resource.id)
                .unwrap()
                .amount,
            starting_amount - 10
        );
        assert_eq!(simulation.actors[&actor_id].inventory[&resource.kind], 10);
        assert_eq!(role_progress(&simulation.actors[&actor_id]).experience, 10);
        let (goal, _) = next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &actor_id,
            resource.position,
        );
        assert_eq!(goal, AgentGoal::Deposit);
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &AgentGoal::Deposit,
            resource.position,
        );
        assert!(simulation.actors[&actor_id].inventory.is_empty());
        assert_eq!(simulation.town_resources[&resource.kind], 10);
    }

    #[test]
    fn full_town_storage_pauses_gathering_and_preserves_carried_overflow() {
        let config = GameConfig::default();
        let content = embedded_content();
        let mut world = generate_world(&config.world);
        let resource = world
            .resources
            .first()
            .expect("default world contains resources")
            .clone();
        let role = match resource.kind.as_str() {
            "resource:wood" => "role:logger",
            "resource:ore" => "role:miner",
            _ => "role:gatherer",
        };
        let actor_id = StableId::new("npc:storage_backpressure_test").unwrap();
        let mut simulation = WorldSimulation::new(world.seed);
        assert!(simulation.join_player(actor_id.clone(), resource.position));
        simulation
            .assign_role(&actor_id, StableId::new(role).unwrap())
            .unwrap();
        simulation
            .actors
            .get_mut(&actor_id)
            .unwrap()
            .inventory
            .insert(resource.kind.clone(), 10);
        let capacity = resource_storage_capacity(&config, &content, &simulation, &resource.kind);
        simulation
            .town_resources
            .insert(resource.kind.clone(), capacity);
        let resource_amount = resource.amount;

        let (goal, _) = next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &actor_id,
            resource.position,
        );
        assert_eq!(goal, AgentGoal::Wander);
        assert!(
            complete_agent_goal(
                &mut simulation,
                &mut world,
                &config,
                &content,
                &actor_id,
                &AgentGoal::Gather(resource.id.clone()),
                resource.position,
            )
            .is_none()
        );
        assert_eq!(
            world
                .resources
                .iter()
                .find(|candidate| candidate.id == resource.id)
                .unwrap()
                .amount,
            resource_amount
        );
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &AgentGoal::Deposit,
            resource.position,
        );
        assert_eq!(simulation.actors[&actor_id].inventory[&resource.kind], 10);

        simulation
            .town_resources
            .insert(resource.kind.clone(), capacity - 1);
        let (goal, _) = next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &actor_id,
            resource.position,
        );
        assert_eq!(goal, AgentGoal::Deposit);
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &goal,
            resource.position,
        );
        assert_eq!(simulation.town_resources[&resource.kind], capacity);
        assert_eq!(simulation.actors[&actor_id].inventory[&resource.kind], 9);

        simulation
            .town_resources
            .insert(resource.kind.clone(), capacity - 10);
        complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &actor_id,
            &AgentGoal::Deposit,
            resource.position,
        );
        assert!(simulation.actors[&actor_id].inventory.is_empty());
        let (goal, _) = next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &actor_id,
            resource.position,
        );
        assert!(matches!(goal, AgentGoal::Gather(_)));
    }

    #[test]
    fn marketplace_passive_income_is_level_scaled_and_save_stable() {
        let config = GameConfig::default();
        let content = embedded_content();
        let marketplace = &content.buildings[&StableId::new("building:marketplace").unwrap()];
        let building_id = StableId::new("building:runtime_marketplace").unwrap();
        let gold = StableId::new("resource:gold").unwrap();
        let mut simulation = WorldSimulation::new(config.world.seed);
        simulation.town_resources.insert(gold.clone(), 0);
        simulation.buildings.insert(
            building_id.clone(),
            BuildingState {
                id: building_id.clone(),
                archetype: marketplace.archetype.clone(),
                position: GridPos { x: 8, z: 8 },
                rotation_quarter_turns: 0,
                level: 1,
                health: BUILDING_MAX_HEALTH,
                complete: true,
            },
        );

        apply_passive_building_income(&config, &content, &mut simulation, Duration::from_secs(2));
        assert_eq!(simulation.town_resources[&gold], 0);
        apply_passive_building_income(&config, &content, &mut simulation, Duration::from_nanos(1));
        assert_eq!(simulation.town_resources[&gold], 1);

        let encoded = ron::to_string(&simulation).unwrap();
        let mut restored: WorldSimulation = ron::from_str(&encoded).unwrap();
        assert_eq!(
            restored.passive_resource_accumulators,
            simulation.passive_resource_accumulators
        );
        restored.buildings.get_mut(&building_id).unwrap().level = 2;
        apply_passive_building_income(&config, &content, &mut restored, Duration::from_secs(1));
        assert_eq!(restored.town_resources[&gold], 2);

        restored.buildings.get_mut(&building_id).unwrap().complete = false;
        apply_passive_building_income(&config, &content, &mut restored, Duration::from_secs(10));
        assert_eq!(restored.town_resources[&gold], 2);
        assert!(restored.passive_resource_accumulators.is_empty());
    }

    #[test]
    fn combat_goal_damages_kills_and_respawns() {
        let config = GameConfig::default();
        let content = embedded_content();
        let mut world = generate_world(&config.world);
        let defender_position = GridPos { x: 32, z: 32 };
        let enemy_position = nearest_walkable(&world, GridPos { x: 33, z: 32 }).unwrap();
        let defender = StableId::new("npc:defender_test").unwrap();
        let enemy = StableId::new("actor:enemy_test").unwrap();
        let mut simulation = WorldSimulation::new(world.seed);
        assert!(simulation.join_player(defender.clone(), defender_position));
        assert!(simulation.join_player(enemy.clone(), enemy_position));
        simulation
            .assign_role(&defender, StableId::new("role:defender").unwrap())
            .unwrap();
        simulation
            .assign_role(&enemy, StableId::new("role:enemy").unwrap())
            .unwrap();
        let (goal, target) = next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &defender,
            defender_position,
        );
        assert_eq!(goal, AgentGoal::Attack(enemy.clone()));
        assert_eq!(target, defender_position);
        for _ in 0..100 {
            complete_agent_goal(
                &mut simulation,
                &mut world,
                &config,
                &content,
                &defender,
                &goal,
                defender_position,
            );
        }
        assert!(!simulation.actors[&enemy].alive);
        assert_eq!(simulation.actors[&enemy].health, 0);
        simulation.respawn_actor(&enemy, enemy_position).unwrap();
        assert!(simulation.actors[&enemy].alive);
        assert_eq!(simulation.actors[&enemy].health, 100);
        let expected_cooldown =
            effective_role_stats(&content, &simulation, &simulation.actors[&defender])
                .map(|stats| milli_units_as_f32(stats.action_milliseconds))
                .unwrap();
        assert!(
            (action_cooldown(&content, &simulation, &defender, &goal) - expected_cooldown).abs()
                <= f32::EPSILON
        );
    }

    #[test]
    fn authored_enemies_drive_damage_range_cadence_and_weighted_spawning() {
        let config = GameConfig::default();
        let content = embedded_content();
        let mut world = generate_world(&config.world);
        let blargul_archetype =
            archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Blargul.prefab").unwrap();
        let blargul = &content.archetypes[&blargul_archetype].enemy;
        let blargul = blargul.as_ref().unwrap();
        assert_eq!(blargul.action_amount, 5);
        assert_eq!(blargul.action_milliseconds, 3_000);
        assert_eq!(blargul.action_range_milli_cells, 10_000);

        let enemy_id = StableId::new("actor:enemy_authored_test").unwrap();
        let player_id = StableId::new("actor:player_authored_test").unwrap();
        let enemy_position = GridPos { x: 30, z: 30 };
        let player_position = GridPos { x: 38, z: 30 };
        let mut simulation = WorldSimulation::new(world.seed);
        assert!(simulation.spawn_enemy(enemy_id.clone(), blargul_archetype, enemy_position, 5,));
        assert!(simulation.join_player(player_id.clone(), player_position));
        let (goal, target) = next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &enemy_id,
            enemy_position,
        );
        assert_eq!(goal, AgentGoal::Attack(player_id.clone()));
        assert_eq!(target, enemy_position);
        let presentation = complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &enemy_id,
            &goal,
            enemy_position,
        );
        assert!(matches!(
            presentation,
            Some(ActionPresentation::Impact {
                target,
                visual: CombatVisualKind::Physical,
            }) if target == player_position
        ));
        assert_eq!(simulation.actors[&player_id].health, 95);
        assert!(
            (action_cooldown(&content, &simulation, &enemy_id, &goal) - 3.0).abs() <= f32::EPSILON
        );

        let camp = content
            .archetypes
            .values()
            .find_map(|archetype| archetype.enemy_spawner.as_ref())
            .unwrap();
        let samples: BTreeSet<_> = (0..1_000)
            .map(|serial| weighted_enemy_archetype(camp, 42, serial))
            .collect();
        assert_eq!(samples.len(), 3);
    }

    #[test]
    fn battering_ram_targets_and_damages_buildings_from_authored_mask() {
        let config = GameConfig::default();
        let content = embedded_content();
        let mut world = generate_world(&config.world);
        let battering_ram = archetype_id_by_source(
            &content,
            ArchetypeKind::Enemy,
            "Enemy_Goblin_BatteringRam.prefab",
        )
        .unwrap();
        let enemy_health = content.archetypes[&battering_ram]
            .health
            .as_ref()
            .and_then(|health| i32::try_from(health.max_health).ok())
            .expect("battering ram has valid authored health");
        let enemy_id = StableId::new("actor:battering_ram_test").unwrap();
        let building_id = StableId::new("building:ram_target").unwrap();
        let enemy_position = GridPos { x: 22, z: 22 };
        let building_position = GridPos { x: 23, z: 22 };
        let mut simulation = WorldSimulation::new(world.seed);
        assert!(simulation.spawn_enemy(
            enemy_id.clone(),
            battering_ram.clone(),
            enemy_position,
            enemy_health,
        ));
        simulation.buildings.insert(
            building_id.clone(),
            BuildingState {
                id: building_id.clone(),
                archetype: content.buildings[&StableId::new("building:house").unwrap()]
                    .archetype
                    .clone(),
                position: building_position,
                rotation_quarter_turns: 0,
                level: 1,
                health: BUILDING_MAX_HEALTH,
                complete: true,
            },
        );
        let (goal, target) = next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &enemy_id,
            enemy_position,
        );
        assert_eq!(goal, AgentGoal::AttackBuilding(building_id.clone()));
        assert_eq!(target, enemy_position);
        let presentation = complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &enemy_id,
            &goal,
            enemy_position,
        );
        assert!(matches!(
            presentation,
            Some(ActionPresentation::BuildingWork {
                target,
                sparks: true,
            }) if target == building_visual_grid(&content, &simulation.buildings[&building_id])
        ));
        assert_eq!(
            simulation.buildings[&building_id].health,
            BUILDING_MAX_HEALTH - 20
        );
        let footprint = content.buildings[&StableId::new("building:house").unwrap()].footprint;
        let region = building_region(building_position, footprint, &world).unwrap();
        world.navigation.set_blocked(region, true).unwrap();
        simulation.buildings.get_mut(&building_id).unwrap().health = 20;
        let presentation = complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &enemy_id,
            &goal,
            enemy_position,
        );
        assert!(matches!(
            presentation,
            Some(ActionPresentation::BuildingDestroyed {
                building,
                target: _,
            }) if building == building_id
        ));
        assert!(!simulation.buildings.contains_key(&building_id));
        assert!(world.navigation.is_walkable(region.min));
    }

    #[test]
    fn priest_prioritizes_and_heals_the_nearest_injured_player() {
        let config = GameConfig::default();
        let content = embedded_content();
        let mut world = generate_world(&config.world);
        let priest_position = GridPos { x: 32, z: 32 };
        let patient_position = GridPos { x: 33, z: 32 };
        let priest = StableId::new("npc:priest_test").unwrap();
        let patient = StableId::new("npc:patient_test").unwrap();
        let mut simulation = WorldSimulation::new(world.seed);
        assert!(simulation.join_player(priest.clone(), priest_position));
        assert!(simulation.join_player(patient.clone(), patient_position));
        simulation
            .assign_role(&priest, StableId::new("role:priest").unwrap())
            .unwrap();
        simulation.damage_actor(&patient, 20).unwrap();

        let (goal, target) = next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &priest,
            priest_position,
        );
        assert_eq!(goal, AgentGoal::Heal(patient.clone()));
        assert_eq!(target, priest_position);
        let presentation = complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &priest,
            &goal,
            priest_position,
        );
        assert!(matches!(
            presentation,
            Some(ActionPresentation::Healing { source, target })
                if source == priest_position && target == patient_position
        ));
        assert_eq!(simulation.actors[&patient].health, 82);
    }

    #[test]
    fn healing_effect_curves_preserve_authored_lifetimes_and_channel_keys() {
        assert!((healing_effect_duration(HealingEffectKind::Burst) - 1.2).abs() < f32::EPSILON);
        assert!((healing_effect_duration(HealingEffectKind::Channel) - 5.0).abs() < f32::EPSILON);
        assert!((healing_effect_duration(HealingEffectKind::Revive) - 1.2).abs() < f32::EPSILON);

        let channel_start = healing_effect_sample(HealingEffectKind::Channel, 0.0);
        let channel_first_key = healing_effect_sample(HealingEffectKind::Channel, 1.5);
        let channel_peak = healing_effect_sample(HealingEffectKind::Channel, 3.0);
        let channel_end = healing_effect_sample(HealingEffectKind::Channel, 5.0);
        assert!(channel_start.ring_scale.abs() < f32::EPSILON);
        assert!((channel_first_key.ring_scale - 0.289).abs() < f32::EPSILON);
        assert!((channel_peak.ring_scale - 1.0).abs() < f32::EPSILON);
        assert!(channel_end.ring_scale.abs() < f32::EPSILON);

        let burst_midpoint = healing_effect_sample(HealingEffectKind::Burst, 0.6);
        let revive_midpoint = healing_effect_sample(HealingEffectKind::Revive, 0.6);
        assert!(burst_midpoint.ring_scale > 0.0);
        assert!(burst_midpoint.mote_scale > 0.0);
        assert!(revive_midpoint.ring_scale > burst_midpoint.ring_scale);
        assert!(revive_midpoint.rise > burst_midpoint.rise);
    }

    #[test]
    fn healing_ring_mesh_is_a_closed_top_facing_annulus() {
        let mesh = healing_ring_mesh(48);
        assert_eq!(mesh.count_vertices(), 96);
        assert_eq!(mesh.indices().unwrap().len(), 288);
        assert!(mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_UV_0).is_some());
    }

    #[test]
    fn ranged_roles_emit_projectiles_instead_of_instant_damage() {
        let config = GameConfig::default();
        let content = embedded_content();
        let mut world = generate_world(&config.world);
        let ranger_position = GridPos { x: 32, z: 32 };
        let enemy_position = GridPos { x: 38, z: 32 };
        let ranger = StableId::new("npc:ranger_test").unwrap();
        let enemy = StableId::new("actor:enemy_ranged_test").unwrap();
        let mut simulation = WorldSimulation::new(world.seed);
        assert!(simulation.join_player(ranger.clone(), ranger_position));
        assert!(simulation.join_player(enemy.clone(), enemy_position));
        simulation
            .assign_role(&ranger, StableId::new("role:ranger").unwrap())
            .unwrap();
        simulation
            .assign_role(&enemy, StableId::new("role:enemy").unwrap())
            .unwrap();
        let (goal, _) = next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &ranger,
            ranger_position,
        );
        let presentation = complete_agent_goal(
            &mut simulation,
            &mut world,
            &config,
            &content,
            &ranger,
            &goal,
            ranger_position,
        )
        .unwrap();
        let ActionPresentation::Projectile(projectile) = presentation else {
            panic!("ranged role must emit a projectile");
        };
        assert_eq!(projectile.target, enemy);
        assert_eq!(projectile.visual, CombatVisualKind::Arrow);
        assert_eq!(simulation.actors[&projectile.target].health, 100);
    }

    #[test]
    fn combat_visuals_follow_authored_role_identity_and_vfx_constants() {
        assert_eq!(
            actor_combat_visual(&StableId::new("role:ranger").unwrap()),
            CombatVisualKind::Arrow
        );
        assert_eq!(
            actor_combat_visual(&StableId::new("role:wizard").unwrap()),
            CombatVisualKind::Fireball
        );
        assert_eq!(
            actor_combat_visual(&StableId::new("role:necromancer").unwrap()),
            CombatVisualKind::Necrotic
        );
        assert_eq!(
            actor_combat_visual(&StableId::new("role:defender").unwrap()),
            CombatVisualKind::Physical
        );
        assert!((CHARACTER_HIT_SECONDS - 0.25).abs() < f32::EPSILON);
        assert!((TOWER_TRAIL_SECONDS - 2.0).abs() < f32::EPSILON);
        assert!((TOWER_TRAIL_WIDTH - 0.1).abs() < f32::EPSILON);
        assert!((FIREBALL_SIZE - 0.4).abs() < f32::EPSILON);
        assert!((FIREBALL_TRAIL_SIZE - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn builder_completes_and_upgrades_authored_construction() {
        let config = GameConfig::default();
        let content = embedded_content();
        let mut world = generate_world(&config.world);
        let building_id = StableId::new("building:house").unwrap();
        let definition = &content.buildings[&building_id];
        let position = find_building_site(
            &world,
            GridPos {
                x: config.world.width / 2,
                z: config.world.height / 2,
            },
            definition.footprint,
        )
        .unwrap();
        let runtime_id = StableId::new("building:test_house").unwrap();
        let mut simulation = WorldSimulation::new(world.seed);
        for resource in definition.cost.keys().chain(definition.level_cost.keys()) {
            simulation
                .town_resources
                .insert(resource.clone(), 1_000_000);
        }
        simulation
            .construct(
                runtime_id.clone(),
                definition.archetype.clone(),
                position,
                &definition.cost,
            )
            .unwrap();
        assert!(!simulation.buildings[&runtime_id].complete);
        assert_eq!(
            building_construction_stage(
                simulation.buildings[&runtime_id].health,
                simulation.buildings[&runtime_id].complete,
            ),
            0
        );
        let region = building_region(position, definition.footprint, &world).unwrap();
        world.navigation.set_blocked(region, true).unwrap();
        let builder_position =
            building_approach(&world, position, definition.footprint, position).unwrap();
        let builder = StableId::new("npc:builder_test").unwrap();
        assert!(simulation.join_player(builder.clone(), builder_position));
        simulation
            .assign_role(&builder, StableId::new("role:builder").unwrap())
            .unwrap();
        let (goal, target) = next_agent_goal(
            &simulation,
            &world,
            &config,
            &content,
            &builder,
            builder_position,
        );
        assert_eq!(goal, AgentGoal::Construct(runtime_id.clone()));
        assert_eq!(target, builder_position);
        for _ in 0..450 {
            complete_agent_goal(
                &mut simulation,
                &mut world,
                &config,
                &content,
                &builder,
                &goal,
                builder_position,
            );
        }
        assert!(simulation.buildings[&runtime_id].complete);
        assert_eq!(
            simulation.buildings[&runtime_id].health,
            BUILDING_MAX_HEALTH
        );
        assert_eq!(
            building_construction_stage(
                simulation.buildings[&runtime_id].health,
                simulation.buildings[&runtime_id].complete,
            ),
            3
        );

        let (technology, authored_cap) = content
            .technology
            .nodes
            .iter()
            .find_map(|(technology, node)| {
                node.building_level_caps
                    .get(&building_id)
                    .copied()
                    .map(|cap| (technology.clone(), cap))
            })
            .expect("Unity technology graph contains a House level-cap effect");
        simulation.unlocked_technology.insert(technology);
        let max_level = maximum_building_level(&content, &simulation, &building_id);
        assert_eq!(max_level, authored_cap);
        let upgrade_cost =
            building_upgrade_cost(&content, &simulation, &building_id, definition, 1);
        assert_eq!(
            simulation
                .upgrade_building(&runtime_id, max_level, &upgrade_cost)
                .unwrap(),
            2
        );
    }

    #[test]
    fn technology_effects_authoritatively_gate_buildings() {
        let content = embedded_content();
        let mut simulation = WorldSimulation::new(42);
        simulation.unlocked_technology.extend(
            content
                .technology
                .nodes
                .iter()
                .filter(|(_, technology)| technology.initially_unlocked)
                .map(|(id, _)| id.clone()),
        );
        let lumbermill = StableId::new("building:lumbermill").unwrap();
        assert!(building_is_unlocked(&content, &simulation, &lumbermill));
        let (technology, locked_building) = content
            .technology
            .nodes
            .iter()
            .filter(|(technology, _)| !simulation.unlocked_technology.contains(*technology))
            .find_map(|(technology, node)| {
                node.unlocked_buildings
                    .iter()
                    .find(|building| {
                        content.buildings[*building].placeable
                            && !building_is_unlocked(&content, &simulation, building)
                    })
                    .map(|building| (technology.clone(), building.clone()))
            })
            .expect("Unity technology graph contains a locked placeable building");
        assert!(!building_is_unlocked(
            &content,
            &simulation,
            &locked_building
        ));
        simulation.unlocked_technology.insert(technology);
        assert!(building_is_unlocked(
            &content,
            &simulation,
            &locked_building
        ));
    }

    #[test]
    fn converted_technology_modifiers_change_runtime_rules() {
        let config = GameConfig::default();
        let content = embedded_content();
        let mut simulation = WorldSimulation::new(42);

        let (cost_technology, building_id, reduction) = content
            .technology
            .nodes
            .iter()
            .find_map(|(technology, node)| {
                node.building_cost_reduction_percent
                    .iter()
                    .next()
                    .map(|(building, amount)| (technology.clone(), building.clone(), *amount))
            })
            .expect("Unity technology graph contains building cost reductions");
        let definition = &content.buildings[&building_id];
        simulation.unlocked_technology.insert(cost_technology);
        assert_eq!(
            building_cost_reduction_percent(&content, &simulation, &building_id),
            reduction
        );
        let adjusted = building_construction_cost(&content, &simulation, &building_id, definition);
        for (resource, base) in &definition.cost {
            assert_eq!(adjusted[resource], percentage_reduced(*base, reduction));
        }

        simulation.unlocked_technology.clear();
        let (storage_technology, resource, boost) = content
            .technology
            .nodes
            .iter()
            .find_map(|(technology, node)| {
                node.storage_boost_percent
                    .iter()
                    .next()
                    .map(|(resource, amount)| (technology.clone(), resource.clone(), *amount))
            })
            .expect("Unity technology graph contains storage boosts");
        let storage_definition = content
            .buildings
            .values()
            .find(|building| {
                building
                    .storage
                    .iter()
                    .any(|storage| storage.resource == resource)
            })
            .expect("converted catalog contains the boosted storage building");
        let storage = storage_definition
            .storage
            .iter()
            .find(|storage| storage.resource == resource)
            .unwrap();
        let runtime_storage = StableId::new("building:test_storage").unwrap();
        simulation.buildings.insert(
            runtime_storage.clone(),
            BuildingState {
                id: runtime_storage,
                archetype: storage_definition.archetype.clone(),
                position: GridPos { x: 1, z: 1 },
                rotation_quarter_turns: 0,
                level: 1,
                health: BUILDING_MAX_HEALTH,
                complete: true,
            },
        );
        simulation.unlocked_technology.insert(storage_technology);
        let base = config.gameplay.base_town_resource_capacity[&resource];
        assert_eq!(
            resource_storage_capacity(&config, &content, &simulation, &resource),
            base + percentage_adjusted(storage.base_amount, boost)
        );

        simulation.unlocked_technology.clear();
        let (stat_technology, role, stat, boost) = content
            .technology
            .nodes
            .iter()
            .find_map(|(technology, node)| {
                node.role_stat_boost_percent
                    .iter()
                    .find_map(|(role, stats)| {
                        stats.iter().next().map(|(stat, amount)| {
                            (technology.clone(), role.clone(), stat.clone(), *amount)
                        })
                    })
            })
            .expect("Unity technology graph contains role stat boosts");
        simulation.unlocked_technology.insert(stat_technology);
        assert_eq!(
            technology_stat_boost_percent(&content, &simulation, &role, &stat),
            boost
        );

        simulation.unlocked_technology.clear();
        let (age_technology, aged_building) = content
            .technology
            .nodes
            .iter()
            .find_map(|(technology, node)| {
                node.aged_buildings
                    .iter()
                    .next()
                    .map(|building| (technology.clone(), building.clone()))
            })
            .expect("Unity technology graph contains building age upgrades");
        assert_eq!(building_age(&content, &simulation, &aged_building), 1);
        simulation.unlocked_technology.insert(age_technology);
        assert_eq!(building_age(&content, &simulation, &aged_building), 2);
        let archetype = &content.archetypes[&content.buildings[&aged_building].archetype];
        assert_eq!(archetype_scene_for_age(archetype, 2).unwrap().age, Some(2));
    }

    #[test]
    fn environment_palette_covers_every_season_and_weather() {
        assert_eq!(parse_weather("SNOW"), Some(Weather::Snow));
        assert_eq!(parse_weather("unknown"), None);
        let seasons = [
            Season::Spring,
            Season::Summer,
            Season::Autumn,
            Season::Winter,
        ];
        let weather = [Weather::Clear, Weather::Rain, Weather::Fog, Weather::Snow];
        for season in seasons {
            for weather in weather {
                let palette = environment_palette(season, weather);
                assert!(palette.fog_start >= 0.0);
                assert!(palette.fog_end > palette.fog_start);
                assert!(palette.sun_illuminance > 0.0);
                assert!(palette.ambient_brightness > 0.0);
            }
        }
        assert_eq!(
            environment_palette(Season::Spring, Weather::Clear).particle_count,
            0
        );
        assert_eq!(
            environment_palette(Season::Spring, Weather::Rain).particle_count,
            180
        );
        assert_eq!(
            environment_palette(Season::Winter, Weather::Snow).particle_count,
            150
        );
        let spring = environment_palette(Season::Spring, Weather::Clear).terrain_tint;
        let winter = environment_palette(Season::Winter, Weather::Clear).terrain_tint;
        assert!(
            spring
                .iter()
                .zip(winter)
                .any(|(spring, winter)| (spring - winter).abs() > 0.1)
        );
        assert_eq!(weather_particle_seed(42, 7), weather_particle_seed(42, 7));
        assert_ne!(weather_particle_seed(42, 7), weather_particle_seed(42, 8));
    }

    #[test]
    fn building_damage_value_matches_unity_health_percentage() {
        assert!((building_damage_value(BUILDING_MAX_HEALTH) - 1.0).abs() < f32::EPSILON);
        assert!((building_damage_value(BUILDING_MAX_HEALTH / 2) - 0.5).abs() < f32::EPSILON);
        assert!((building_damage_value(0) - 0.0).abs() < f32::EPSILON);
        assert!((building_damage_value(-100) - 0.0).abs() < f32::EPSILON);
        assert!((building_damage_value(BUILDING_MAX_HEALTH * 2) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn building_effects_preserve_authored_vfx_contracts() {
        assert!((BUILDING_HIT_SECONDS - 0.5).abs() < f32::EPSILON);
        assert!((BUILDING_HIT_SMOKE_SPEED - 3.0).abs() < f32::EPSILON);
        assert!((BUILDING_HIT_SPARK_SPEED - 12.0).abs() < f32::EPSILON);
        assert!((BUILDING_HIT_SMOKE_SIZE - 0.5).abs() < f32::EPSILON);
        assert!((BUILDING_HIT_SPARK_SIZE - 0.25).abs() < f32::EPSILON);
        assert!((BUILDING_LEVEL_UP_SECONDS - 1.5).abs() < f32::EPSILON);
        assert!((BUILDING_LEVEL_UP_ARROW_SIZE - 0.5).abs() < f32::EPSILON);
        assert!((BUILDING_LEVEL_UP_TILE_SIZE - 4.0).abs() < f32::EPSILON);
        assert!((BUILDING_DAMAGED_RADIUS - 1.403_639_8).abs() < f32::EPSILON);
        assert_eq!(BUILDING_DAMAGED_FIRE_AMOUNT, 128);
        assert_eq!(BUILDING_DAMAGED_SMOKE_AMOUNT, 200);
        assert!(building_damage_intensity(BUILDING_MAX_HEALTH, true).abs() < f32::EPSILON);
        assert!(building_damage_intensity(BUILDING_MAX_HEALTH / 2, false).abs() < f32::EPSILON);
        assert!(building_damage_intensity(BUILDING_MAX_HEALTH / 2, true) > 0.0);
        assert!(building_damage_intensity(0, true).abs() < f32::EPSILON);
    }

    #[test]
    fn production_resource_kinds_resolve_converted_visuals() {
        let content = embedded_content();
        let cases = [
            (
                "resource:wood",
                "Assets/Prefabs/Dummy Assets/Env_Tree.prefab",
                "Env_Tree.glb",
            ),
            (
                "resource:ore",
                "Assets/Prefabs/Resources/Resource_Ore_Base.prefab",
                "Env_Ore.glb",
            ),
            (
                "resource:food",
                "Assets/Prefabs/Resources/Resource_Bush_Base.prefab",
                "Env_Bush.glb",
            ),
        ];
        for (kind, expected_source, expected_asset) in cases {
            let kind = StableId::new(kind).unwrap();
            let archetype = resource_visual_archetype(&content, &kind).unwrap();
            assert_eq!(archetype.source_path, expected_source);
            let scene = default_archetype_scene(archetype).unwrap();
            assert!(scene.asset_path.ends_with(expected_asset));
        }
        assert!(
            resource_visual_archetype(&content, &StableId::new("resource:fish").unwrap()).is_none()
        );
    }

    #[test]
    fn production_resource_glbs_expose_unity_masks_as_color_zero() {
        for name in ["Env_Tree.glb", "Env_Ore.glb", "Env_Bush.glb"] {
            let path = locate_asset_root()
                .join("migrated/models/Models/Resources")
                .join(name);
            let bytes = std::fs::read(&path).unwrap();
            let json_length = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
            let json = &bytes[20..20 + json_length];
            let json_end = json
                .iter()
                .rposition(|byte| !byte.is_ascii_whitespace() && *byte != 0)
                .unwrap()
                + 1;
            let document: serde_json::Value = serde_json::from_slice(&json[..json_end]).unwrap();
            for primitive in document["meshes"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|mesh| mesh["primitives"].as_array().unwrap())
            {
                let attributes = primitive["attributes"].as_object().unwrap();
                assert!(attributes.contains_key("COLOR_0"), "{name} lacks COLOR_0");
                assert!(
                    attributes
                        .keys()
                        .all(|key| !key.starts_with("COLOR_") || key == "COLOR_0"),
                    "{name} contains an unsupported secondary color semantic"
                );
            }
        }
    }

    #[test]
    fn resource_variants_and_scale_are_deterministic() {
        let resource = |kind: &str, x, z| stream_town_domain::GeneratedResource {
            id: StableId::new(format!("resource:{x}:{z}")).unwrap(),
            kind: StableId::new(kind).unwrap(),
            position: GridPos { x, z },
            amount: 100,
        };
        assert_eq!(resource_mesh_index(&resource("resource:wood", 2, 4)), 0);
        assert_eq!(resource_mesh_index(&resource("resource:wood", 3, 4)), 1);
        assert_eq!(resource_mesh_index(&resource("resource:ore", 9, 6)), 1);
        assert_eq!(resource_mesh_index(&resource("resource:food", 9, 6)), 0);
        assert!((resource_visual_scale(12.0) - 0.03).abs() < f32::EPSILON);
    }

    #[test]
    fn tree_seasons_match_unity_material_targets() {
        assert_eq!(
            tree_season_controls(Season::Spring),
            Vec4::new(0.0, 0.0, 0.1, 0.0)
        );
        assert_eq!(tree_season_controls(Season::Summer), Vec4::ZERO);
        assert_eq!(
            tree_season_controls(Season::Autumn),
            Vec4::new(0.3, 0.0, 0.0, 0.0)
        );
        assert_eq!(
            tree_season_controls(Season::Winter),
            Vec4::new(0.0, 0.5, 0.0, 0.0)
        );
    }

    #[test]
    fn generated_terrain_mesh_matches_navigation_grid() {
        let config = GameConfig::default();
        let world = generate_world(&config.world);
        let mesh = generated_terrain_mesh(&world, &config);
        assert_eq!(mesh.count_vertices(), 65 * 65);
        assert_eq!(mesh.indices().unwrap().len(), 64 * 64 * 6);
        assert_eq!(
            mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap().len(),
            mesh.count_vertices()
        );
        assert_eq!(
            mesh.attribute(Mesh::ATTRIBUTE_COLOR).unwrap().len(),
            mesh.count_vertices()
        );
        let centre = GridPos {
            x: config.world.width / 2,
            z: config.world.height / 2,
        };
        assert!(
            (grid_to_world_on_surface(centre, &config, &world).y - terrain_height(&world, centre))
                .abs()
                <= f32::EPSILON
        );
        let water = generated_water_mesh(&world, &config);
        assert_eq!(water.count_vertices(), 81 * 81);
        assert_eq!(water.indices().unwrap().len(), 80 * 80 * 6);
        let bevy::mesh::VertexAttributeValues::Float32x4(depth_colors) =
            water.attribute(Mesh::ATTRIBUTE_COLOR).unwrap()
        else {
            panic!("water depth must use float vertex colors");
        };
        assert!(
            depth_colors
                .iter()
                .all(|color| (0.0..=1.0).contains(&color[0]))
        );
        assert!(depth_colors.iter().any(|color| color[0] == 0.0));
        assert!(depth_colors.iter().any(|color| color[0] > 0.0));
    }

    #[test]
    fn retained_terrain_mesh_reconstructs_source_geometry() {
        let saved = SavedTerrainMesh {
            vertices: vec![[-4.0, 0.0, 3.0], [4.0, 0.0, 3.0], [0.0, 2.0, -5.0]],
            triangle_indices: vec![0, 1, 2],
            uvs: Vec::new(),
            uses_32_bit_indices: false,
        };
        let mesh = retained_terrain_mesh(&saved).unwrap();
        assert_eq!(mesh.count_vertices(), saved.vertices.len());
        assert_eq!(mesh.indices().unwrap().len(), saved.triangle_indices.len());
        assert_eq!(
            mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap().len(),
            saved.vertices.len()
        );
        assert_eq!(
            mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap().len(),
            saved.vertices.len()
        );
        assert!(Collider::trimesh_from_mesh(&mesh).is_some());
    }

    #[test]
    fn embedded_config_supports_vertical_slice_scale() {
        let config = GameConfig::default();
        assert!(config.gameplay.initial_agents >= 300);
        let world = generate_world(&config.world);
        assert_eq!(world.navigation.width(), config.world.width);
    }

    #[test]
    fn embedded_unity_content_catalog_is_valid() {
        let content = embedded_content();
        assert_eq!(content.archetypes.len(), 215);
        assert_eq!(content.foliage.len(), 4);
        assert_eq!(
            content
                .foliage
                .iter()
                .map(|layer| layer.variants.len())
                .sum::<usize>(),
            21
        );
        assert_eq!(content.buildings.len(), 26);
        assert_eq!(content.roles.len(), 15);
        assert_eq!(content.technology.nodes.len(), 363);
        assert_eq!(content.technology.groups.len(), 20);
        let logger = &content.roles[&StableId::new("role:logger").unwrap()];
        assert_eq!(logger.base_action_amount, 1);
        assert_eq!(logger.experience_multiplier_per_thousand, 1_000);
        assert_eq!(logger.action_amount_per_level_milli, 250);
        assert_eq!(logger.base_action_milliseconds, 1_000);
        assert_eq!(logger.action_milliseconds_reduction_per_level, 5);
        assert_eq!(logger.base_carry_capacity, 10);
        assert_eq!(logger.carry_capacity_per_level_milli, 2_000);
        assert_eq!(
            logger.resource.as_ref().map(StableId::as_str),
            Some("resource:wood")
        );
        let ranger = &content.roles[&StableId::new("role:ranger").unwrap()];
        assert_eq!(ranger.base_action_range_milli_cells, 12_000);
    }

    #[test]
    fn authored_level_curves_drive_effective_role_stats() {
        let content = embedded_content();
        let actor_id = StableId::new("npc:leveled_logger").unwrap();
        let role = StableId::new("role:logger").unwrap();
        let mut simulation = WorldSimulation::new(9);
        assert!(simulation.join_player(actor_id.clone(), GridPos { x: 1, z: 1 }));
        simulation.assign_role(&actor_id, role.clone()).unwrap();
        simulation
            .actors
            .get_mut(&actor_id)
            .unwrap()
            .role_progression
            .insert(
                role,
                stream_town_domain::RoleProgress {
                    level: 5,
                    experience: 7,
                },
            );

        let stats =
            effective_role_stats(&content, &simulation, &simulation.actors[&actor_id]).unwrap();
        assert_eq!(stats.level, 5);
        assert_eq!(stats.experience, 7);
        assert_eq!(stats.action_amount, 2);
        assert_eq!(stats.action_milliseconds, 980);
        assert_eq!(stats.movement_speed_milli_cells_per_second, 3_200);
        assert_eq!(stats.carry_capacity, 18);
    }

    #[test]
    fn renderer_material_resolution_prefers_exact_slots_then_model_then_fallback() {
        let mut materials = Assets::<StandardMaterial>::default();
        let fallback = materials.add(StandardMaterial::default());
        let game = materials.add(StandardMaterial::default());
        let skin = materials.add(StandardMaterial::default());
        let wrong_same_name = materials.add(StandardMaterial::default());
        let override_material = materials.add(StandardMaterial::default());
        let mut building_materials = Assets::<BuildingMaterial>::default();
        let building = building_materials.add(building_material(&embedded_presentation(), None));
        let mut cloud_materials = Assets::<CloudMaterial>::default();
        let cloud = cloud_materials.add(cloud_material(&embedded_presentation(), None));
        let spec = MaterialOverrideSpec {
            fallback: Some(ResolvedMaterialHandle::Standard(fallback.clone())),
            model_materials: BTreeMap::from([
                (
                    "GameMaterial".into(),
                    ResolvedMaterialHandle::Standard(game.clone()),
                ),
                (
                    "SkinMaterial".into(),
                    ResolvedMaterialHandle::Standard(skin.clone()),
                ),
                (
                    "BuildingMaterial".into(),
                    ResolvedMaterialHandle::Building(building.clone()),
                ),
                (
                    "CloudMaterial".into(),
                    ResolvedMaterialHandle::Cloud(cloud.clone()),
                ),
            ]),
            renderer_materials: vec![
                ResolvedRendererMaterialBinding {
                    target_path: "Other/Body_Blacksmith_Bulk".into(),
                    materials: BTreeMap::from([(
                        "GameMaterial".into(),
                        ResolvedMaterialHandle::Standard(wrong_same_name),
                    )]),
                },
                ResolvedRendererMaterialBinding {
                    target_path: "PlayerChar_TPose/Body_Mesh/Body_Blacksmith_Bulk".into(),
                    materials: BTreeMap::from([(
                        "GameMaterial".into(),
                        ResolvedMaterialHandle::Standard(override_material.clone()),
                    )]),
                },
            ],
        };

        let exact = resolved_renderer_material(
            &spec,
            "Scene/PlayerChar_TPose/Body_Mesh/Body_Blacksmith_Bulk/Body_Blacksmith_Bulk.GameMaterial",
            Some("Body_Blacksmith_Bulk"),
            Some("GameMaterial"),
        )
        .unwrap();
        assert!(matches!(
            exact,
            ResolvedMaterialHandle::Standard(material) if material.id() == override_material.id()
        ));

        let model = resolved_renderer_material(
            &spec,
            "Scene/PlayerChar_TPose/Body_Mesh/Body_Blacksmith_Bulk/Body_Blacksmith_Bulk.SkinMaterial",
            Some("Body_Blacksmith_Bulk"),
            Some("SkinMaterial"),
        )
        .unwrap();
        assert!(matches!(
            model,
            ResolvedMaterialHandle::Standard(material) if material.id() == skin.id()
        ));

        let inherited = resolved_renderer_material(
            &spec,
            "Scene/Unrelated/Unrelated.GameMaterial",
            Some("Unrelated"),
            Some("GameMaterial"),
        )
        .unwrap();
        assert!(matches!(
            inherited,
            ResolvedMaterialHandle::Standard(material) if material.id() == game.id()
        ));

        let typed_building = resolved_renderer_material(
            &spec,
            "Scene/TownHall/TownHall.BuildingMaterial",
            Some("TownHall"),
            Some("BuildingMaterial"),
        )
        .unwrap();
        assert!(matches!(
            typed_building,
            ResolvedMaterialHandle::Building(material) if material.id() == building.id()
        ));

        let typed_cloud = resolved_renderer_material(
            &spec,
            "Scene/Clouds/Clouds.CloudMaterial",
            Some("Clouds"),
            Some("CloudMaterial"),
        )
        .unwrap();
        assert!(matches!(
            typed_cloud,
            ResolvedMaterialHandle::Cloud(material) if material.id() == cloud.id()
        ));

        let final_fallback =
            resolved_renderer_material(&spec, "Scene/Unrelated", None, Some("Unmapped")).unwrap();
        assert!(matches!(
            final_fallback,
            ResolvedMaterialHandle::Standard(material) if material.id() == fallback.id()
        ));
    }

    #[test]
    fn embedded_presentation_binds_native_and_converted_animation_paths() {
        let content = embedded_content();
        let presentation = embedded_presentation();
        assert_eq!(presentation.schema_version, 11);
        assert_eq!(presentation.textures.len(), 133);
        assert_eq!(presentation.materials.len(), 33);
        assert_eq!(
            presentation
                .materials
                .values()
                .map(|material| material.custom_vectors.len())
                .sum::<usize>(),
            141
        );
        assert_eq!(
            presentation
                .materials
                .values()
                .map(|material| material.texture_transforms.len())
                .sum::<usize>(),
            32
        );
        let terrain = terrain_material(&presentation, &GameConfig::default(), None);
        assert!(terrain.extension.grid_texture.is_none());
        assert!((terrain.extension.parameters.texture_uv_blend_tint.z - -1.8).abs() < f32::EPSILON);
        assert!((terrain.extension.parameters.texture_uv_blend_tint.w - 1.0).abs() < f32::EPSILON);
        assert_eq!(
            terrain.extension.parameters.grid_scale_offset,
            Vec4::new(1.0, 1.0, 0.0, 0.0)
        );
        assert!(
            terrain
                .extension
                .parameters
                .sand_color_a
                .to_array()
                .into_iter()
                .zip([1.0, 0.827_731, 0.088_235_21, 0.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        let water = water_material(&presentation, None);
        assert!(water.extension.main_texture.is_none());
        assert!(water.extension.noise_texture.is_none());
        assert!(
            water
                .extension
                .parameters
                .surface_color
                .to_array()
                .into_iter()
                .zip([0.0, 0.764_705_9, 1.0, 1.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert!((water.extension.parameters.wind_speed_noise_alpha.z - 0.02).abs() < f32::EPSILON);
        assert!((water.extension.parameters.scale_foam_ice.y - 3.71).abs() < f32::EPSILON);
        assert_eq!(
            water.extension.parameters.depth_foam_controls,
            Vec4::new(10.0, 0.8, 7.81, 0.94)
        );
        assert_eq!(
            water_color_tint(
                Vec4::new(0.0, 0.764_705_9, 1.0, 1.0),
                [0.05, 0.29, 0.47, 0.62]
            ),
            Vec4::new(0.5, 0.29 / 0.764_705_9, 0.47, 0.62)
        );
        assert_eq!(
            water.extension.parameters.main_scale_offset,
            Vec4::new(1.0, 1.0, 0.0, 0.0)
        );
        let building = building_material(&presentation, None);
        assert!(building.extension.main_texture.is_none());
        assert_eq!(
            building.extension.parameters.detail_color,
            Vec4::new(0.521_568_5, 0.521_568_5, 0.521_568_5, 1.0)
        );
        assert_eq!(
            building.extension.parameters.ambient_occlusion,
            Vec4::new(0.4, 1.74, 0.0, 0.0)
        );
        assert_eq!(
            building.extension.parameters.main_scale_offset,
            Vec4::new(1.0, 1.0, 0.0, 0.0)
        );
        assert!((building.extension.parameters.snow_damage.z - 1.787).abs() < f32::EPSILON);
        let clouds = cloud_material(&presentation, None);
        assert!(clouds.extension.noise_texture.is_none());
        assert_eq!(
            clouds.extension.parameters.noise_controls,
            Vec4::new(0.005, 20.0, 0.0, 0.09)
        );
        assert_eq!(
            clouds.extension.parameters.surface_transform,
            Vec4::new(200.0, 1.4, 1.0, 1.0)
        );
        let water_definition = presentation
            .materials
            .values()
            .find(|material| material.source_path == WATER_MATERIAL_PATH)
            .unwrap();
        assert!(
            water_definition.texture_transforms["_WaterNormal"]
                .scale
                .into_iter()
                .zip([81.2, 200.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert!((water_ice_strength(Season::Spring) - 0.0).abs() < f32::EPSILON);
        assert!((water_ice_strength(Season::Winter) - 1.0).abs() < f32::EPSILON);
        assert_eq!(presentation.controllers.len(), 31);
        assert_eq!(
            presentation
                .model_materials
                .values()
                .map(BTreeMap::len)
                .sum::<usize>(),
            241
        );
        assert_eq!(
            presentation
                .prefab_renderer_materials
                .values()
                .map(Vec::len)
                .sum::<usize>(),
            903
        );
        assert_eq!(
            presentation
                .prefab_renderer_materials
                .values()
                .flatten()
                .map(|renderer| renderer.materials.len())
                .sum::<usize>(),
            912
        );
        assert_eq!(
            presentation
                .clips
                .values()
                .map(|clip| clip.property_curves.len())
                .sum::<usize>(),
            110
        );
        assert_eq!(
            presentation
                .clips
                .values()
                .map(|clip| clip.events.len())
                .sum::<usize>(),
            10
        );
        assert_eq!(
            animation_property_value(
                &presentation,
                "CreditsPopups",
                "CreatedBy_Canvas",
                "m_IsActive",
                8.0,
            ),
            Some(1.0)
        );
        assert_eq!(
            animation_property_value(
                &presentation,
                "CreditsPopups",
                "CreatedBy_Canvas",
                "m_IsActive",
                8.5,
            ),
            Some(0.0)
        );
        let character_materials = presentation
            .model_materials
            .get("Assets/Models/Characters/Characters.fbx")
            .unwrap();
        assert_eq!(
            character_materials["GameMaterial"].as_str(),
            "material:fa4085ce3ea4d394bb4e587376c58cbd"
        );
        assert_eq!(
            character_materials["SkinMaterial"].as_str(),
            "material:304fcfe47809be14ab680e64084f8494"
        );
        let archetype =
            archetype_by_source(&content, ArchetypeKind::Enemy, "Enemy_Goblin.prefab").unwrap();
        let scene = default_archetype_scene(archetype).unwrap();
        let binding = presentation
            .prefab_bindings
            .get(&archetype.source_guid)
            .unwrap();
        assert_eq!(
            binding.animated_scene.as_deref(),
            Some(scene.asset_path.as_str())
        );
        assert_eq!(binding.gltf_animation_index, Some(0));
        let materials = presentation
            .prefab_materials
            .get(&archetype.source_guid)
            .unwrap();
        assert!(!materials.is_empty());
        assert!(
            materials
                .iter()
                .all(|material| presentation.materials.contains_key(material))
        );

        let player =
            archetype_by_source(&content, ArchetypeKind::Player, "Player_Character.prefab")
                .unwrap();
        let player_scene = default_archetype_scene(player).unwrap();
        let player_binding = presentation
            .prefab_bindings
            .get(&player.source_guid)
            .unwrap();
        assert_eq!(
            player_binding.rig_scene.as_deref(),
            Some(player_scene.asset_path.as_str())
        );
        let spec = converted_animation_spec(player, &presentation).unwrap();
        let controller = presentation.controllers.get(&spec.controller).unwrap();
        assert_eq!(controller.state_machines.len(), 4);
        assert_eq!(controller.layers.len(), 2);
        assert_eq!(controller.layers[0].display_name, "Base Layer");
        assert_eq!(controller.layers[1].display_name, "Top");
        assert!((controller.layers[0].effective_weight(0) - 1.0).abs() < f32::EPSILON);
        assert!(controller.layers[1].effective_weight(1).abs() < f32::EPSILON);
        let base_mask = controller.layers[0]
            .avatar_mask
            .as_ref()
            .and_then(|mask| presentation.avatar_masks.get(mask))
            .unwrap();
        assert_eq!(base_mask.display_name, "Player_All");
        assert!(
            base_mask
                .transform_weights
                .values()
                .all(|weight| (*weight - 1.0).abs() < f32::EPSILON)
        );
        for (role_id, role) in &content.roles {
            let parameter = controller
                .parameters
                .iter()
                .find(|parameter| parameter.name == role.action_animation)
                .unwrap_or_else(|| {
                    panic!(
                        "{role_id} action {} is missing from Character.controller",
                        role.action_animation
                    )
                });
            let expected_kind = if role.action_animation == "Action" {
                stream_town_domain::AnimationParameterKind::Boolean
            } else {
                stream_town_domain::AnimationParameterKind::Trigger
            };
            assert_eq!(parameter.kind, expected_kind, "{role_id}");
        }
        let state = controller.states.get(&spec.state).unwrap();
        assert_eq!(state.blend_parameter.as_deref(), Some("Move Speed"));
        assert_eq!(state.motions.len(), 3);
        let idle = presentation.clips.get(&state.motions[0].clip).unwrap();
        assert!(!idle.transform_tracks.is_empty());
        let mut runtime = AnimationControllerRuntime::in_state(controller, spec.state).unwrap();
        runtime.set_float("Move Speed", 0.25).unwrap();
        let selection = runtime.motion_selection(controller).unwrap().unwrap();
        assert!(selection.second.is_some());
        runtime.set_boolean("Action", true).unwrap();
        runtime.set_trigger("SpearAttack").unwrap();
        let transition = runtime.evaluate_transitions(controller, 0.0).unwrap();
        let stream_town_domain::AnimationTransitionOutcome::Entered(action_state) = transition
        else {
            panic!("authored SpearAttack trigger did not enter an action state");
        };
        assert_eq!(controller.states[&action_state].display_name, "SpearAttack");
        assert_eq!(
            controller.states[&action_state].speed_parameter.as_deref(),
            Some("ActionSpeed")
        );
        runtime.set_float("ActionSpeed", 2.0).unwrap();
        assert!((runtime.state_speed(controller).unwrap() - 2.0).abs() < f32::EPSILON);
        runtime.set_boolean("Action", false).unwrap();
        let exit = runtime.evaluate_transitions(controller, 0.0).unwrap();
        let stream_town_domain::AnimationTransitionOutcome::Entered(locomotion_state) = exit else {
            panic!("authored action exit did not return through the parent state machine");
        };
        assert_eq!(
            controller.states[&locomotion_state].display_name,
            "Locomotion"
        );
        let top = controller
            .layers
            .iter()
            .find(|layer| layer.display_name == "Top")
            .unwrap();
        let top_state = controller.state_machines[&top.state_machine]
            .default_state
            .clone()
            .unwrap();
        let mut top_runtime = AnimationControllerRuntime::in_state(controller, top_state).unwrap();
        top_runtime.set_boolean("CarryWood", true).unwrap();
        let carry_transition = top_runtime.evaluate_transitions(controller, 1.0).unwrap();
        let stream_town_domain::AnimationTransitionOutcome::Entered(carry_state) = carry_transition
        else {
            panic!("Top layer did not enter Carry from authored CarryWood parameter");
        };
        assert_eq!(controller.states[&carry_state].display_name, "Carry");
        assert!(state_layer_owns_clip(
            controller,
            top_runtime.current_state(),
            &controller.states[&carry_state].motions[0].clip,
        ));
        assert_eq!(
            presentation
                .clips
                .values()
                .filter(|clip| !clip.transform_tracks.is_empty())
                .count(),
            57
        );
        let targets: BTreeMap<_, _> = idle
            .transform_tracks
            .iter()
            .map(|track| {
                (
                    track.target_path.clone(),
                    (
                        Entity::PLACEHOLDER,
                        Transform {
                            translation: track
                                .reference_translation
                                .map_or(Vec3::ZERO, Vec3::from_array),
                            rotation: track
                                .reference_rotation
                                .map_or(Quat::IDENTITY, normalized_quat),
                            scale: track.reference_scale.map_or(Vec3::ONE, Vec3::from_array),
                        },
                    ),
                )
            })
            .collect();
        let retargeted = retargeted_animation_clip(idle, &targets).unwrap();
        assert!(!retargeted.curves().is_empty());
        assert!(retargeted.duration() >= idle.duration_seconds);
    }

    #[test]
    fn authored_layer_weight_and_mask_configure_bevy_graph_branch() {
        let mut graph = AnimationGraph::new();
        let composition = graph.add_additive_blend(1.0, graph.root);
        let path = "CharacterArmature/Body/UpperArm_R";
        let target = path.split('/').collect::<AnimationTargetId>();
        let targets =
            BTreeMap::from([(path.to_owned(), (Entity::PLACEHOLDER, Transform::IDENTITY))]);
        let authored = AvatarMaskDef {
            display_name: "Left Arm".into(),
            source_guid: "a".repeat(32),
            source_path: "Assets/LeftArm.mask".into(),
            humanoid_body_mask_hex: "01000000".into(),
            transform_weights: BTreeMap::from([(path.to_owned(), 0.0)]),
        };
        let mask = register_avatar_mask(&mut graph, 3, Some(&authored), &targets);
        let branch = add_animation_layer_branch(
            &mut graph,
            AnimationLayerBlendMode::Additive,
            0.25,
            mask,
            composition,
        );
        assert_eq!(mask, 1 << 3);
        assert_eq!(graph.mask_groups[&target], 1 << 3);
        assert!(matches!(
            graph.graph[branch].node_type,
            AnimationNodeType::Add
        ));
        assert!((graph.graph[branch].weight - 0.25).abs() < f32::EPSILON);
        assert_eq!(graph.graph[branch].mask, 1 << 3);
    }

    #[test]
    fn broadcaster_gate_precedes_twitch_command_dispatch() {
        let viewer = |message: &str, is_broadcaster| {
            TwitchEvent::Chat(twitch::TwitchChatEnvelope {
                actor_id: StableId::new("twitch:42").unwrap(),
                user_id: "42".to_owned(),
                login: "viewer".to_owned(),
                display_name: "Viewer".to_owned(),
                message: message.to_owned(),
                is_broadcaster,
                is_moderator: false,
                is_subscriber: false,
                custom_reward_id: None,
            })
        };
        let mut connection = TwitchConnection {
            connect_code: "123456".to_owned(),
            ..default()
        };
        let mut commands = InjectedCommands::default();

        handle_twitch_event(viewer("!join", false), &mut connection, &mut commands);
        assert!(commands.0.is_empty());
        handle_twitch_event(
            viewer("!connect 123456", true),
            &mut connection,
            &mut commands,
        );
        assert!(connection.broadcaster_authorized);
        handle_twitch_event(viewer("!join", false), &mut connection, &mut commands);
        let dispatched = commands.0.pop_front().unwrap();
        assert_eq!(dispatched.actor_id, StableId::new("twitch:42").unwrap());
        assert_eq!(dispatched.command, ChatCommand::Join);
    }

    #[test]
    fn fish_god_channel_reward_dispatches_praise_without_command_text() {
        let mut connection = TwitchConnection {
            broadcaster_authorized: true,
            ..default()
        };
        let mut commands = InjectedCommands::default();
        handle_twitch_event(
            TwitchEvent::Chat(twitch::TwitchChatEnvelope {
                actor_id: StableId::new("twitch:fish").unwrap(),
                user_id: "fish".to_owned(),
                login: "fishfriend".to_owned(),
                display_name: "FishFriend".to_owned(),
                message: "Praise!".to_owned(),
                is_broadcaster: false,
                is_moderator: false,
                is_subscriber: false,
                custom_reward_id: Some(FISH_GOD_REWARD_ID.to_owned()),
            }),
            &mut connection,
            &mut commands,
        );
        assert_eq!(commands.0.pop_front().unwrap().command, ChatCommand::Praise);
    }

    #[test]
    fn ruler_permissions_and_login_candidate_resolution_are_stable() {
        let ruler = StableId::new("twitch:100").unwrap();
        let viewer = StableId::new("twitch:200").unwrap();
        let mut simulation = WorldSimulation::new(7);
        assert!(simulation.join_player(ruler.clone(), GridPos { x: 1, z: 1 }));
        assert!(simulation.join_player(viewer.clone(), GridPos { x: 2, z: 1 }));
        simulation.actors.get_mut(&ruler).unwrap().login_name = Some("the_ruler".to_owned());
        simulation.actors.get_mut(&ruler).unwrap().display_name = Some("The Ruler".to_owned());
        simulation.set_ruler(ruler.clone()).unwrap();
        simulation
            .start_ruler_vote(RulerVoteKind::KeepRuler)
            .unwrap();

        let ordinary = PendingChatCommand {
            actor_id: viewer,
            login_name: "viewer".to_owned(),
            display_name: "Viewer".to_owned(),
            command: ChatCommand::RecruitCount,
            is_broadcaster: false,
            is_moderator: false,
            is_subscriber: false,
            origin: CommandOrigin::Twitch,
        };
        assert!(require_ruler_or_staff(&simulation, &ordinary).is_err());
        let ruler_command = PendingChatCommand {
            actor_id: ruler,
            login_name: "the_ruler".to_owned(),
            display_name: "The Ruler".to_owned(),
            command: ChatCommand::RecruitCount,
            is_broadcaster: false,
            is_moderator: false,
            is_subscriber: false,
            origin: CommandOrigin::Twitch,
        };
        assert!(require_ruler_or_staff(&simulation, &ruler_command).is_ok());
        assert_eq!(
            resolve_ruler_vote_option(&simulation, &StableId::new("yes").unwrap()),
            Some(StableId::new("yes").unwrap())
        );
        simulation.ruler_vote = None;
        simulation
            .start_ruler_vote(RulerVoteKind::NewRuler)
            .unwrap();
        assert_eq!(
            resolve_ruler_vote_option(&simulation, &StableId::new("the_ruler").unwrap()),
            Some(StableId::new("twitch:100").unwrap())
        );
    }

    #[test]
    fn game_master_permission_is_explicit_and_local_debug_bypasses_it() {
        let mut config = GameConfig::default();
        let mut command = PendingChatCommand {
            actor_id: StableId::new("twitch:12345").unwrap(),
            login_name: "configured_gm".to_owned(),
            display_name: "Configured GM".to_owned(),
            command: ChatCommand::ToggleBuildCosts,
            is_broadcaster: true,
            is_moderator: true,
            is_subscriber: false,
            origin: CommandOrigin::Twitch,
        };
        assert!(require_staff(&command).is_ok());
        assert!(require_game_master(&config, &command).is_err());
        config.twitch.game_master_ids.insert("12345".to_owned());
        assert!(require_game_master(&config, &command).is_ok());
        config.twitch.game_master_ids.clear();
        command.is_broadcaster = false;
        command.is_moderator = false;
        command.origin = CommandOrigin::LocalDebug;
        assert!(require_game_master(&config, &command).is_ok());
    }

    #[test]
    fn headless_vertical_slice_spawns_three_hundred_agents() {
        let config = GameConfig::default();
        let expected = usize::from(config.gameplay.initial_agents);
        let save_directory = tempfile::tempdir().unwrap();
        let save_path = save_directory.path().join("command-save.stbevy");
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            bevy::state::app::StatesPlugin,
            bevy::input::InputPlugin,
        ))
        .insert_resource(RuntimeConfig(config))
        .add_plugins(StreamTownGamePlugin);
        app.insert_resource(SaveRuntime {
            store: NativeSaveStore::new(&save_path),
        });

        app.update();
        app.update();
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::WorldLoading);
        app.update();
        app.update();

        let actual = app
            .world_mut()
            .query_filtered::<Entity, With<Agent>>()
            .iter(app.world())
            .count();
        assert_eq!(actual, expected);
        assert!(app.world().contains_resource::<WorldRuntime>());

        let enemies = app
            .world_mut()
            .query::<&Agent>()
            .iter(app.world())
            .filter(|agent| agent.kind == ActorKind::Enemy)
            .count();
        assert_eq!(enemies, 1);

        app.world_mut()
            .resource_mut::<InjectedCommands>()
            .0
            .push_back(PendingChatCommand {
                actor_id: StableId::new("twitch:debug_viewer").unwrap(),
                login_name: "debug_viewer".to_owned(),
                display_name: "debug_viewer".to_owned(),
                command: "!join".parse().unwrap(),
                is_broadcaster: true,
                is_moderator: true,
                is_subscriber: true,
                origin: CommandOrigin::LocalDebug,
            });
        app.update();
        let joined_count = app
            .world_mut()
            .query_filtered::<Entity, With<Agent>>()
            .iter(app.world())
            .count();
        assert_eq!(joined_count, expected + 1);
        assert!(
            app.world()
                .resource::<SimulationRuntime>()
                .0
                .actors
                .contains_key(&StableId::new("twitch:debug_viewer").unwrap())
        );
        assert_eq!(
            app.world().resource::<SimulationRuntime>().0.actors
                [&StableId::new("twitch:debug_viewer").unwrap()]
                .active_pet
                .as_ref()
                .map(StableId::as_str),
            Some("pet:red_panda")
        );

        let gm_commands = [
            ChatCommand::ToggleBuildCosts,
            ChatCommand::ToggleRoleLimits,
            ChatCommand::AddResource {
                resource: StableId::new("wood").unwrap(),
                amount: 123,
            },
            ChatCommand::GivePet {
                player: StableId::new("debug_viewer").unwrap(),
                pet: StableId::new("duck").unwrap(),
            },
            ChatCommand::LevelUpPlayer {
                player: StableId::new("debug_viewer").unwrap(),
                amount: 2,
            },
            ChatCommand::QueueEvent(StableId::new("fishgod").unwrap()),
        ];
        for command in gm_commands {
            app.world_mut()
                .resource_mut::<InjectedCommands>()
                .0
                .push_back(PendingChatCommand {
                    actor_id: StableId::new("twitch:debug_viewer").unwrap(),
                    login_name: "debug_viewer".to_owned(),
                    display_name: "debug_viewer".to_owned(),
                    command,
                    is_broadcaster: false,
                    is_moderator: false,
                    is_subscriber: true,
                    origin: CommandOrigin::LocalDebug,
                });
        }
        app.update();
        {
            let simulation = &app.world().resource::<SimulationRuntime>().0;
            assert!(!simulation.building_costs_enabled);
            assert!(!simulation.role_limits_enabled);
            assert_eq!(town_resource_amount(simulation, "resource:wood"), 5_123);
            assert!(
                simulation.actors[&StableId::new("twitch:debug_viewer").unwrap()]
                    .unlocked_pets
                    .contains(&StableId::new("pet:duck").unwrap())
            );
            assert_eq!(
                simulation.queued_events.len() + usize::from(simulation.fish_god.is_some()),
                1
            );
        }
        app.update();
        assert!(
            app.world()
                .resource::<SimulationRuntime>()
                .0
                .fish_god
                .is_some()
        );
        for command in [
            ChatCommand::GameEventAction,
            ChatCommand::StopEvent,
            ChatCommand::AddResource {
                resource: StableId::new("wood").unwrap(),
                amount: -123,
            },
            ChatCommand::ToggleBuildCosts,
            ChatCommand::ToggleRoleLimits,
        ] {
            app.world_mut()
                .resource_mut::<InjectedCommands>()
                .0
                .push_back(PendingChatCommand {
                    actor_id: StableId::new("twitch:debug_viewer").unwrap(),
                    login_name: "debug_viewer".to_owned(),
                    display_name: "debug_viewer".to_owned(),
                    command,
                    is_broadcaster: false,
                    is_moderator: false,
                    is_subscriber: true,
                    origin: CommandOrigin::LocalDebug,
                });
        }
        app.update();
        {
            let simulation = &app.world().resource::<SimulationRuntime>().0;
            assert!(simulation.building_costs_enabled);
            assert!(simulation.role_limits_enabled);
            assert_eq!(town_resource_amount(simulation, "resource:wood"), 5_000);
            assert!(simulation.fish_god.is_none());
            assert!(simulation.active_event.is_none());
        }

        let eligible_technology = {
            let content = &app.world().resource::<RuntimeContent>().0;
            let simulation = &app.world().resource::<SimulationRuntime>().0;
            content
                .technology
                .nodes
                .iter()
                .find(|(id, node)| {
                    !simulation.unlocked_technology.contains(*id)
                        && !node.unavailable
                        && node
                            .prerequisites
                            .iter()
                            .all(|required| simulation.unlocked_technology.contains(required))
                })
                .map(|(id, _)| id.clone())
                .expect("converted catalog has a vote-eligible technology")
        };
        let available_building = {
            let content = &app.world().resource::<RuntimeContent>().0;
            let simulation = &app.world().resource::<SimulationRuntime>().0;
            content
                .buildings
                .iter()
                .find(|(id, building)| {
                    building.placeable && building_is_unlocked(content, simulation, id)
                })
                .map(|(id, building)| {
                    (
                        id.clone(),
                        building.clone(),
                        building_construction_cost(content, simulation, id, building),
                    )
                })
                .expect("converted initial technology unlocks a placeable building")
        };
        let actor_id = StableId::new("twitch:debug_viewer").unwrap();
        {
            let position = app.world().resource::<SimulationRuntime>().0.actors[&actor_id].position;
            let site = find_building_site(
                &app.world().resource::<WorldRuntime>().generated,
                position,
                available_building.1.footprint,
            )
            .expect("vertical slice has a building site");
            app.world_mut()
                .resource_mut::<SimulationRuntime>()
                .0
                .actors
                .get_mut(&actor_id)
                .unwrap()
                .last_building_position = Some(site);
        }
        let commands = [
            ChatCommand::SelectRole(StableId::new("builder").unwrap()),
            ChatCommand::Experience,
            ChatCommand::Build(available_building.0.clone()),
            ChatCommand::MoveBuilding(vec![BuildingAction {
                direction: BuildingDirection::Rotate,
                amount: 4,
            }]),
            ChatCommand::ConfirmBuilding,
            ChatCommand::DismissRecruit(5),
            ChatCommand::DismissRecruit(4),
            ChatCommand::Recruit {
                role: StableId::new("miner").unwrap(),
                amount: 2,
            },
            ChatCommand::Role,
            ChatCommand::Health,
            ChatCommand::Target(None),
            ChatCommand::Station(None),
            ChatCommand::Customize {
                kind: CustomizationKind::Body,
                index: 3,
            },
            ChatCommand::Camera(vec![CameraAction {
                direction: CameraDirection::In,
                amount: 1,
            }]),
            ChatCommand::Vote(eligible_technology.clone()),
            ChatCommand::TriggerEvent(StableId::new("festival").unwrap()),
            ChatCommand::Save,
            ChatCommand::Help,
        ];
        for command in commands {
            app.world_mut()
                .resource_mut::<InjectedCommands>()
                .0
                .push_back(PendingChatCommand {
                    actor_id: actor_id.clone(),
                    login_name: "debug_viewer".to_owned(),
                    display_name: "debug_viewer".to_owned(),
                    command,
                    is_broadcaster: true,
                    is_moderator: true,
                    is_subscriber: true,
                    origin: CommandOrigin::LocalDebug,
                });
        }
        app.update();

        let (placed_building, saved_building_id, food_before_revive) = {
            let simulation = &app.world().resource::<SimulationRuntime>().0;
            assert_eq!(simulation.actors[&actor_id].role.as_str(), "role:builder");
            assert_eq!(
                simulation
                    .actors
                    .keys()
                    .filter(|id| id.as_str().starts_with("npc:recruit_"))
                    .count(),
                2
            );
            assert_eq!(recruited_actor_ids(simulation).len(), 5);
            assert_eq!(town_resource_amount(simulation, "resource:recruit"), 5);
            assert_eq!(simulation.actors[&actor_id].customization.body_type, 2);
            assert_eq!(
                simulation.actors[&actor_id].building_rotation_quarter_turns,
                4
            );
            assert!(
                simulation.actors[&actor_id]
                    .last_building_position
                    .is_some()
            );
            assert_eq!(simulation.buildings.len(), 2);
            let placed_building = simulation
                .buildings
                .values()
                .find(|building| building.id.as_str() != "building:townhall")
                .unwrap()
                .clone();
            assert!(!placed_building.complete);
            assert_eq!(placed_building.health, BUILDING_MAX_HEALTH / 10);
            assert_eq!(placed_building.rotation_quarter_turns, 4);
            for resource in [
                "resource:food",
                "resource:gold",
                "resource:ore",
                "resource:wood",
            ] {
                let resource_id = StableId::new(resource).unwrap();
                assert_eq!(
                    town_resource_amount(simulation, resource),
                    5_000 - available_building.2[&resource_id]
                );
            }
            assert_eq!(
                simulation.active_vote.as_ref().map(|vote| &vote.technology),
                Some(&eligible_technology)
            );
            assert_eq!(simulation.active_event, Some(TownEvent::Festival));
            let saved_building_id = placed_building.id.clone();
            (
                placed_building,
                saved_building_id,
                town_resource_amount(simulation, "resource:food"),
            )
        };
        let mut runtime_building_ids: Vec<_> = app
            .world_mut()
            .query::<&RuntimeBuilding>()
            .iter(app.world())
            .map(|building| building.id.clone())
            .collect();
        runtime_building_ids.sort();
        let mut expected_runtime_buildings = vec![
            StableId::new("building:townhall").unwrap(),
            saved_building_id.clone(),
        ];
        expected_runtime_buildings.sort();
        assert_eq!(runtime_building_ids, expected_runtime_buildings);
        assert!(save_path.is_file());
        let saved = NativeSaveStore::new(&save_path).load().unwrap();
        assert_eq!(
            saved.simulation.buildings[&saved_building_id],
            placed_building
        );
        assert!(
            app.world()
                .resource::<CommandFeedback>()
                .0
                .contains("commands: !join")
        );
        app.world_mut()
            .resource_mut::<SimulationRuntime>()
            .0
            .damage_actor(&actor_id, u32::MAX)
            .unwrap();
        app.world_mut()
            .resource_mut::<InjectedCommands>()
            .0
            .push_back(PendingChatCommand {
                actor_id: actor_id.clone(),
                login_name: "debug_viewer".to_owned(),
                display_name: "debug_viewer".to_owned(),
                command: ChatCommand::Revive(None),
                is_broadcaster: true,
                is_moderator: true,
                is_subscriber: true,
                origin: CommandOrigin::LocalDebug,
            });
        app.update();
        let simulation = &app.world().resource::<SimulationRuntime>().0;
        assert!(simulation.actors[&actor_id].alive);
        assert_eq!(
            town_resource_amount(simulation, "resource:food"),
            food_before_revive - 400
        );
        assert!(
            app.world()
                .resource::<CommandFeedback>()
                .0
                .contains("revived twitch:debug_viewer")
        );
        let revival_rings = app
            .world_mut()
            .query::<&HealingRingEffect>()
            .iter(app.world())
            .filter(|effect| effect.kind == HealingEffectKind::Revive)
            .count();
        let revival_mote_bars = app
            .world_mut()
            .query::<&HealingMoteEffect>()
            .iter(app.world())
            .filter(|effect| effect.kind == HealingEffectKind::Revive)
            .count();
        assert_eq!(revival_rings, 1);
        assert_eq!(revival_mote_bars, 24);

        {
            let technology_ids = app
                .world()
                .resource::<RuntimeContent>()
                .0
                .technology
                .nodes
                .keys()
                .cloned()
                .collect::<Vec<_>>();
            let mut simulation = app.world_mut().resource_mut::<SimulationRuntime>();
            let building = simulation.0.buildings.get_mut(&saved_building_id).unwrap();
            building.complete = true;
            building.health = BUILDING_MAX_HEALTH;
            simulation.0.unlocked_technology.extend(technology_ids);
            for amount in simulation.0.town_resources.values_mut() {
                *amount = 1_000_000;
            }
        }
        for command in [
            ChatCommand::LevelBuilding {
                building: available_building.0.clone(),
                index: 1,
                iterations: 2,
            },
            ChatCommand::LevelAll {
                building: available_building.0.clone(),
                target_level: 4,
            },
        ] {
            app.world_mut()
                .resource_mut::<InjectedCommands>()
                .0
                .push_back(PendingChatCommand {
                    actor_id: actor_id.clone(),
                    login_name: "debug_viewer".to_owned(),
                    display_name: "debug_viewer".to_owned(),
                    command,
                    is_broadcaster: false,
                    is_moderator: false,
                    is_subscriber: true,
                    origin: CommandOrigin::LocalDebug,
                });
        }
        app.update();
        assert!(
            app.world().resource::<SimulationRuntime>().0.buildings[&saved_building_id].level >= 2
        );
        let removed_position =
            app.world().resource::<SimulationRuntime>().0.buildings[&saved_building_id].position;
        app.world_mut()
            .resource_mut::<InjectedCommands>()
            .0
            .push_back(PendingChatCommand {
                actor_id,
                login_name: "debug_viewer".to_owned(),
                display_name: "debug_viewer".to_owned(),
                command: ChatCommand::RemoveBuilding {
                    building: available_building.0,
                    index: 1,
                },
                is_broadcaster: false,
                is_moderator: false,
                is_subscriber: true,
                origin: CommandOrigin::LocalDebug,
            });
        app.update();
        assert_eq!(
            app.world()
                .resource::<SimulationRuntime>()
                .0
                .buildings
                .keys()
                .map(StableId::as_str)
                .collect::<Vec<_>>(),
            vec!["building:townhall"]
        );
        assert!(
            app.world()
                .resource::<WorldRuntime>()
                .generated
                .navigation
                .is_walkable(removed_position)
        );
        assert_eq!(
            app.world_mut()
                .query_filtered::<&RuntimeBuilding, Without<TownHall>>()
                .iter(app.world())
                .count(),
            0
        );
    }

    #[test]
    fn world_grid_projection_round_trips() {
        let config = GameConfig::default();
        for cell in [
            GridPos { x: 0, z: 0 },
            GridPos { x: 31, z: 47 },
            GridPos { x: 63, z: 63 },
        ] {
            assert_eq!(
                world_to_grid(grid_to_world(cell, &config), &config),
                Some(cell)
            );
        }
    }

    #[test]
    fn rotated_footprints_and_building_moves_are_deterministic() {
        assert_eq!(rotated_footprint([2, 5], 0), [2, 5]);
        assert_eq!(rotated_footprint([2, 5], 1), [5, 2]);
        assert_eq!(rotated_footprint([2, 5], -1), [5, 2]);
        let world = generate_world(&GameConfig::default().world);
        let (position, rotation) = shift_grid_position(
            GridPos { x: 1, z: 1 },
            &[
                BuildingAction {
                    direction: BuildingDirection::Left,
                    amount: 4,
                },
                BuildingAction {
                    direction: BuildingDirection::Up,
                    amount: 3,
                },
                BuildingAction {
                    direction: BuildingDirection::Rotate,
                    amount: -2,
                },
            ],
            &world,
        );
        assert_eq!(position, GridPos { x: 0, z: 4 });
        assert_eq!(rotation, -2);
    }
}
