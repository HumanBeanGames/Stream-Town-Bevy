use std::{
    collections::{BTreeMap, BTreeSet, hash_map::DefaultHasher},
    fs::{self, OpenOptions},
    hash::{Hash, Hasher},
    io::Write,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex, mpsc},
    thread,
    time::{Duration, Instant},
};

use anyhow::Context as _;

mod technology_graph;

use bevy::animation::{
    RepeatAnimation,
    graph::{AnimationGraph, AnimationGraphHandle, AnimationNodeIndex},
};
use bevy::math::Affine2;
use bevy::prelude::*;
#[cfg(target_os = "windows")]
use bevy::render::{
    RenderPlugin,
    settings::{Backends, WgpuSettings},
};
use bevy::{
    asset::{AssetPlugin, LoadState},
    camera::{ClearColorConfig, RenderTarget, primitives::Aabb},
    gltf::{GltfAssetLabel, GltfMaterialName, GltfMeshName},
    render::render_resource::TextureFormat,
    transform::TransformSystems,
};
use bevy_egui::{
    EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass, EguiStartupSet,
    EguiTextureHandle, EguiUserTextures, PrimaryEguiContext, egui,
};
use bevy_tidal::{
    NativeAudioRouting, NativeAudioState, NativeAudioStatus, TidalBackendStatus, TidalConfig,
    TidalController, TidalPlugin,
};
use stream_town_domain::{
    AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef,
    AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef,
    AnimationMotionDef, AnimationParameterDef, AnimationParameterKind, AnimationPropertyCurve,
    AnimationQuatKeyframe, AnimationStateDef, AnimationStateMachineDef, AnimationTangent,
    AnimationTransformTrack, AnimationTransitionDef, AnimationVec3Keyframe, ArchetypeBounds,
    ArchetypeDef, ArchetypeKind, ArchetypeScene, BroadcastEncoderPreference, BuildingDef,
    BuildingHealthDisplayMode, BuildingModelDef, ChatCommand, ContentCatalog, DisplayMode,
    EnemyCampGenerationDef, EnemyDef, EnemyModelSetDef, EnemyRunAnimation, EnemySpawnerDef,
    EnemyWeaponModelDef, FoliageHabitat, FoliageLayerDef, GameConfig, GeneratedWorld, GridPos,
    HealthDef, MaterialAlphaMode, MaterialDef, NameDisplayMode, ObjectiveDef, ObjectiveKind,
    PassiveResourceContribution, PetDef, PetModelDef, PlayerSettings, PlayerSettingsStore,
    PostProcessAntiAliasing, PresentationCatalog, ProjectileShooterDef, RendererMaterialBinding,
    ResourceGenerationHabitat, ResourceGenerationLayerDef, ResourceReward, RoleDef,
    RoleEquipmentDef, RoleSlotContribution, RotatingNodeDef, RuntimeConsoleAction,
    RuntimeConsoleRequest, RuntimeConsoleStatus, RuntimeConsoleStore, StableId, StationDef,
    StorageContribution, StorageModelDef, TargetingScoreDef, TechGroup, TechNode,
    TechnologyGraphLayout, TextureDef, TextureTransform, WeightedEnemySpawn,
};
#[cfg(target_os = "windows")]
use stream_town_game::direct_broadcast::{BroadcastPrerequisites, inspect_broadcast_prerequisites};
use stream_town_game::twitch::{
    CredentialVault, DeviceAuthorization, OAuthClient, TokenValidation, TwitchControl, TwitchEvent,
    TwitchStatus, TwitchTransport, TwitchUserIdentity,
};
use stream_town_game::{
    PLAYER_ANIMATED_MODEL_PATH, PLAYER_ANIMATED_SOURCE_MODEL, preview_animation_asset_for_rig,
};
use technology_graph::{TechnologyGraphViewState, show as show_technology_graph};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum ToolTab {
    #[default]
    Authority,
    Assets,
    Buildings,
    Roles,
    Balance,
    Technology,
    Terrain,
    Music,
    World,
    Settings,
    Runtime,
    Twitch,
    Validation,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum AssetEditorSection {
    #[default]
    Models,
    Textures,
    Materials,
    Animations,
}

impl AssetEditorSection {
    const ALL: [Self; 4] = [
        Self::Models,
        Self::Textures,
        Self::Materials,
        Self::Animations,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Models => "Models",
            Self::Textures => "Textures",
            Self::Materials => "Materials",
            Self::Animations => "Animations",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum PreviewRequest {
    Model(String),
    Texture {
        id: StableId,
        fingerprint: u64,
    },
    Material {
        id: StableId,
        fingerprint: u64,
    },
    Animation {
        clip: StableId,
        asset_path: String,
        animation_index: u32,
        fingerprint: u64,
    },
    Role {
        role: StableId,
        asset_path: String,
        animation_index: u32,
        looping: bool,
        visible_nodes: BTreeSet<String>,
        fingerprint: u64,
    },
}

const DEFAULT_PREVIEW_YAW: f32 = 0.0;
const DEFAULT_PREVIEW_PITCH: f32 = 0.18;
const DEFAULT_PREVIEW_DISTANCE: f32 = 10.0;
const AUTHORING_COMBO_MAX_HEIGHT: f32 = 320.0;
const TOOL_MUSIC_PREVIEW_TRACK: u64 = 1;

#[derive(Clone, Debug)]
struct ModelPreviewControls {
    yaw: f32,
    pitch: f32,
    distance: f32,
    pan: Vec3,
    animation_playing: bool,
    animation_looping: bool,
    animation_speed: f32,
    restart_animation: bool,
}

#[derive(Clone, Debug, Default)]
struct GltfMetadata {
    nodes: Vec<String>,
    materials: Vec<String>,
    animations: Vec<(usize, String)>,
}

impl Default for ModelPreviewControls {
    fn default() -> Self {
        Self {
            yaw: DEFAULT_PREVIEW_YAW,
            pitch: DEFAULT_PREVIEW_PITCH,
            distance: DEFAULT_PREVIEW_DISTANCE,
            pan: Vec3::ZERO,
            animation_playing: true,
            animation_looping: true,
            animation_speed: 1.0,
            restart_animation: false,
        }
    }
}

impl ModelPreviewControls {
    fn reset_view(&mut self) {
        self.yaw = DEFAULT_PREVIEW_YAW;
        self.pitch = DEFAULT_PREVIEW_PITCH;
        self.distance = DEFAULT_PREVIEW_DISTANCE;
        self.pan = Vec3::ZERO;
    }
}

impl ToolTab {
    const ALL: [Self; 13] = [
        Self::Authority,
        Self::Assets,
        Self::Buildings,
        Self::Roles,
        Self::Balance,
        Self::Technology,
        Self::Terrain,
        Self::Music,
        Self::World,
        Self::Settings,
        Self::Runtime,
        Self::Twitch,
        Self::Validation,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Authority => "Game Authority",
            Self::Assets => "Models + Assets",
            Self::Buildings => "Buildings",
            Self::Roles => "Roles",
            Self::Balance => "Progression",
            Self::Technology => "Technology",
            Self::Terrain => "Terrain",
            Self::Music => "Music",
            Self::World => "World + Nav",
            Self::Settings => "Player Settings",
            Self::Runtime => "Runtime",
            Self::Twitch => "Twitch",
            Self::Validation => "Validation",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MusicPreviewState {
    Stopped,
    Playing,
}

#[derive(Resource)]
struct ToolState {
    tab: ToolTab,
    command: String,
    status: String,
    config: GameConfig,
    config_path: String,
    player_settings: PlayerSettings,
    catalog: ContentCatalog,
    presentation: PresentationCatalog,
    presentation_path: String,
    asset_section: AssetEditorSection,
    selected_archetype: Option<StableId>,
    selected_archetype_scene: usize,
    discovered_model_assets: Vec<String>,
    discovered_texture_assets: Vec<String>,
    gltf_metadata: BTreeMap<String, GltfMetadata>,
    asset_search: String,
    new_archetype_id: String,
    new_archetype_name: String,
    new_archetype_kind: ArchetypeKind,
    new_archetype_asset: String,
    model_import_source: String,
    model_import_name: String,
    preview_request: Option<PreviewRequest>,
    preview_label: String,
    selected_texture: Option<StableId>,
    selected_material: Option<StableId>,
    selected_clip: Option<StableId>,
    selected_controller: Option<StableId>,
    new_texture_id: String,
    new_texture_name: String,
    new_texture_asset: String,
    texture_import_source: String,
    texture_import_name: String,
    new_material_id: String,
    new_material_name: String,
    new_material_texture_slot: String,
    new_material_property: String,
    new_material_vector: String,
    new_clip_id: String,
    new_clip_name: String,
    new_clip_asset: String,
    new_controller_id: String,
    new_controller_name: String,
    new_controller_state_id: String,
    new_controller_machine_id: String,
    new_animation_parameter: String,
    generated_world: Option<GeneratedWorld>,
    technology_search: String,
    selected_group: Option<StableId>,
    technology_draft: Option<TechnologyDraft>,
    catalog_path: String,
    technology_layout: TechnologyGraphLayout,
    technology_layout_path: String,
    technology_graph_view: TechnologyGraphViewState,
    selected_objective: Option<StableId>,
    objective_draft: Option<ObjectiveDraft>,
    new_objective_id: String,
    selected_building: Option<StableId>,
    building_draft: Option<BuildingDraft>,
    new_building_id: String,
    new_building_name: String,
    selected_role: Option<StableId>,
    role_draft: Option<RoleDraft>,
    new_role_id: String,
    new_role_name: String,
    role_preview_body_type: u8,
    role_preview_carrying: bool,
    role_preview_animation: Option<StableId>,
    role_preview_eye: Option<String>,
    role_preview_hair: Option<String>,
    role_preview_facial_hair: Option<String>,
    selected_foliage: Option<StableId>,
    foliage_draft: Option<FoliageLayerDef>,
    selected_foliage_variant: usize,
    new_foliage_id: String,
    selected_resource_generation: Option<StableId>,
    resource_generation_draft: Option<ResourceGenerationLayerDef>,
    new_resource_generation_id: String,
    new_resource_generation_name: String,
    selected_enemy_camp_generation: Option<StableId>,
    enemy_camp_generation_draft: Option<EnemyCampGenerationDef>,
    new_enemy_camp_generation_id: String,
    world_asset_search: String,
    world_preview_layer: WorldPreviewLayer,
    new_technology_id: String,
    new_technology_name: String,
    new_group_id: String,
    new_group_name: String,
    undo_authoring: Vec<AuthoringSnapshot>,
    redo_authoring: Vec<AuthoringSnapshot>,
    twitch_auth_events: Option<Arc<Mutex<mpsc::Receiver<TwitchToolEvent>>>>,
    twitch_device: Option<DeviceAuthorization>,
    twitch_validation: Option<TokenValidation>,
    twitch_broadcast_validation: Option<TokenValidation>,
    twitch_channel_identity: Option<TwitchUserIdentity>,
    twitch_irc_verified: bool,
    game_master_ids: String,
    game_master_lookup: String,
    fish_god_reward_id: String,
    tool_job_events: Option<Arc<Mutex<mpsc::Receiver<ToolJobEvent>>>>,
    runtime_console: RuntimeConsoleStore,
    runtime_status: Option<RuntimeConsoleStatus>,
    runtime_process: Option<Child>,
    runtime_profile_launch: bool,
    runtime_sequence: u64,
    runtime_actor_id: String,
    runtime_login: String,
    music_preview_intensity: f64,
    music_preview_season: f64,
    music_preview_time_of_day: f64,
    music_preview_population: usize,
    music_preview_building_count: usize,
    music_preview_state: MusicPreviewState,
    music_preview_rendered_score: String,
}

#[derive(Resource)]
struct ModelPreviewRuntime {
    image: Handle<Image>,
    scene_entity: Option<Entity>,
    scene_handle: Option<Handle<WorldAsset>>,
    loaded_request: Option<PreviewRequest>,
    animation_graph: Option<Handle<AnimationGraph>>,
    animation_node: Option<AnimationNodeIndex>,
    animation_started: bool,
    material_overrides: PreviewMaterialOverrides,
    visible_nodes: Option<BTreeSet<String>>,
    camera_target_offset: Vec3,
    framed: bool,
    status: String,
    controls: ModelPreviewControls,
}

#[derive(Default)]
struct PreviewMaterialOverrides {
    fallback: Option<Handle<StandardMaterial>>,
    model_materials: BTreeMap<String, Handle<StandardMaterial>>,
    renderer_materials: Vec<PreviewRendererMaterialBinding>,
}

struct PreviewRendererMaterialBinding {
    target_path: String,
    materials: BTreeMap<String, Handle<StandardMaterial>>,
}

#[derive(Component)]
struct ModelPreviewScene;

#[derive(Component)]
struct PreviewMaterialApplied;

#[derive(Component)]
struct PreviewNodeVisibilityApplied;

#[derive(Component)]
struct ModelPreviewCamera;

#[derive(Clone)]
struct AuthoringSnapshot {
    catalog: ContentCatalog,
    technology_layout: TechnologyGraphLayout,
}

#[derive(Debug)]
enum TwitchToolEvent {
    Device(DeviceAuthorization),
    Authorized(TokenValidation),
    BroadcastAuthorized(TokenValidation),
    Progress(String),
    Diagnostic {
        validation: TokenValidation,
        channel: TwitchUserIdentity,
    },
    BroadcastDiagnostic {
        validation: TokenValidation,
        ingest_count: usize,
        prerequisites: BroadcastPrerequisites,
    },
    GameMasterResolved(TwitchUserIdentity),
    RewardCaptured(String),
    Cleared,
    BroadcastCleared,
    Error(String),
}

#[derive(Debug)]
enum ToolJobEvent {
    Finished(String),
    Error(String),
}

#[derive(Clone)]
struct TechnologyDraft {
    id: StableId,
    value: TechNode,
    prerequisite_search: String,
    unlock_search: String,
    objective_search: String,
    icon_search: String,
}

#[derive(Clone)]
struct RoleDraft {
    id: StableId,
    value: RoleDef,
}

#[derive(Clone)]
struct BuildingDraft {
    id: StableId,
    value: BuildingDef,
}

#[derive(Clone)]
struct ObjectiveDraft {
    id: StableId,
    value: ObjectiveDef,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum WorldPreviewLayer {
    #[default]
    Elevation,
    Navigation,
    LandResources,
    WaterResources,
    Foliage,
    EnemyCamps,
}

impl WorldPreviewLayer {
    const ALL: [Self; 6] = [
        Self::Elevation,
        Self::Navigation,
        Self::LandResources,
        Self::WaterResources,
        Self::Foliage,
        Self::EnemyCamps,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Elevation => "Elevation + water",
            Self::Navigation => "Navigation occupancy",
            Self::LandResources => "Land resources",
            Self::WaterResources => "Water resources",
            Self::Foliage => "Foliage layers",
            Self::EnemyCamps => "Enemy camp ranges",
        }
    }
}

impl Default for ToolState {
    fn default() -> Self {
        let catalog: ContentCatalog =
            ron::from_str(include_str!("../../../assets/content/catalog.ron"))
                .expect("checked-in content catalog must parse");
        catalog
            .validate()
            .expect("checked-in content catalog must validate");
        let technology_layout: TechnologyGraphLayout = ron::from_str(include_str!(
            "../../../assets/content/technology_layout.ron"
        ))
        .expect("checked-in technology layout must parse");
        technology_layout
            .validate(&catalog.technology)
            .expect("checked-in technology layout must validate");
        let selected_group = catalog.technology.groups.keys().next().cloned();
        let selected_objective = catalog.objectives.keys().next().cloned();
        let objective_draft = selected_objective.as_ref().and_then(|id| {
            catalog
                .objectives
                .get(id)
                .cloned()
                .map(|value| ObjectiveDraft {
                    id: id.clone(),
                    value,
                })
        });
        let selected_role = catalog.roles.keys().next().cloned();
        let role_draft = selected_role
            .as_ref()
            .and_then(|id| role_draft(&catalog, id));
        let selected_foliage = catalog.foliage.first().map(|layer| layer.id.clone());
        let selected_resource_generation = catalog
            .resource_generation
            .first()
            .map(|layer| layer.id.clone());
        let selected_enemy_camp_generation = catalog
            .enemy_camp_generation
            .first()
            .map(|layer| layer.id.clone());
        let enemy_camp_generation_draft = selected_enemy_camp_generation.as_ref().and_then(|id| {
            catalog
                .enemy_camp_generation
                .iter()
                .find(|layer| &layer.id == id)
                .cloned()
        });
        let selected_building = catalog.buildings.keys().next().cloned();
        let building_draft = selected_building
            .as_ref()
            .and_then(|id| building_draft(&catalog, id));
        let foliage_draft = selected_foliage.as_ref().and_then(|id| {
            catalog
                .foliage
                .iter()
                .find(|layer| &layer.id == id)
                .cloned()
        });
        let resource_generation_draft = selected_resource_generation.as_ref().and_then(|id| {
            catalog
                .resource_generation
                .iter()
                .find(|layer| &layer.id == id)
                .cloned()
        });
        let presentation: PresentationCatalog =
            ron::from_str(include_str!("../../../assets/content/presentation.ron"))
                .expect("checked-in presentation catalog must parse");
        presentation
            .validate()
            .expect("checked-in presentation catalog must validate");
        let selected_archetype = catalog.archetypes.keys().next().cloned();
        let discovered_model_assets = discover_model_assets();
        let discovered_texture_assets = discover_texture_assets();
        let new_archetype_asset = discovered_model_assets.first().cloned().unwrap_or_default();
        let selected_texture = presentation.textures.keys().next().cloned();
        let selected_material = presentation.materials.keys().next().cloned();
        let selected_clip = presentation.clips.keys().next().cloned();
        let selected_controller = presentation.controllers.keys().next().cloned();
        let new_texture_asset = discovered_texture_assets
            .first()
            .cloned()
            .unwrap_or_default();
        let new_clip_asset = discovered_model_assets.first().cloned().unwrap_or_default();
        let config_path = default_config_path();
        let mut config = load_game_config(config_path.to_string_lossy().as_ref())
            .expect("checked-in game configuration must parse and validate");
        if stream_town_game::runtime_config_path().is_file()
            && let Ok(runtime_config) = stream_town_game::load_runtime_config()
        {
            // The Music tab should open on the score the world actually loaded,
            // while the other authoring sections retain their project values.
            config.music = runtime_config.music;
        }
        let player_settings_store =
            PlayerSettingsStore::new(stream_town_game::player_settings_path());
        let player_settings = player_settings_store.load().unwrap_or_default();
        let game_master_ids = config
            .twitch
            .game_master_ids
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        let game_master_lookup = config.twitch.channel_login.clone();
        let fish_god_reward_id = config.twitch.fish_god_reward_id.clone().unwrap_or_default();
        let role_preview_animation = default_role_preview_animation(&presentation, &catalog);
        Self {
            tab: ToolTab::default(),
            command: "!join".to_owned(),
            status: "Ready. Content edits remain drafts until their catalog validates and saves."
                .to_owned(),
            config,
            config_path: config_path.display().to_string(),
            player_settings,
            catalog,
            presentation,
            presentation_path: default_presentation_path().display().to_string(),
            asset_section: AssetEditorSection::default(),
            selected_archetype,
            selected_archetype_scene: 0,
            discovered_model_assets,
            discovered_texture_assets,
            gltf_metadata: BTreeMap::new(),
            asset_search: String::new(),
            new_archetype_id: "archetype:new".to_owned(),
            new_archetype_name: "New Model".to_owned(),
            new_archetype_kind: ArchetypeKind::Other,
            new_archetype_asset,
            model_import_source: String::new(),
            model_import_name: String::new(),
            preview_request: None,
            preview_label: "No model selected".to_owned(),
            selected_texture,
            selected_material,
            selected_clip,
            selected_controller,
            new_texture_id: "texture:new".to_owned(),
            new_texture_name: "New Texture".to_owned(),
            new_texture_asset,
            texture_import_source: String::new(),
            texture_import_name: String::new(),
            new_material_id: "material:new".to_owned(),
            new_material_name: "New Material".to_owned(),
            new_material_texture_slot: "_BaseMap".to_owned(),
            new_material_property: "_Property".to_owned(),
            new_material_vector: "_Colour".to_owned(),
            new_clip_id: "clip:new".to_owned(),
            new_clip_name: "New Animation".to_owned(),
            new_clip_asset,
            new_controller_id: "animation_controller:new".to_owned(),
            new_controller_name: "New Controller".to_owned(),
            new_controller_state_id: "animation_state:new".to_owned(),
            new_controller_machine_id: "animation_state_machine:new".to_owned(),
            new_animation_parameter: "Parameter".to_owned(),
            generated_world: None,
            technology_search: String::new(),
            selected_group,
            technology_draft: None,
            catalog_path: default_catalog_path().display().to_string(),
            technology_layout,
            technology_layout_path: default_technology_layout_path().display().to_string(),
            technology_graph_view: TechnologyGraphViewState::default(),
            selected_objective,
            objective_draft,
            new_objective_id: "objective:new".to_owned(),
            selected_building,
            building_draft,
            new_building_id: "building:new".to_owned(),
            new_building_name: "New Building".to_owned(),
            selected_role,
            role_draft,
            new_role_id: "role:new".to_owned(),
            new_role_name: "New Role".to_owned(),
            role_preview_body_type: 0,
            role_preview_carrying: false,
            role_preview_animation,
            role_preview_eye: Some("Eyes_Normal".to_owned()),
            role_preview_hair: Some("Hair_Short_Normal".to_owned()),
            role_preview_facial_hair: None,
            selected_foliage,
            foliage_draft,
            selected_foliage_variant: 0,
            new_foliage_id: "foliage:new".to_owned(),
            selected_resource_generation,
            resource_generation_draft,
            new_resource_generation_id: "resource_generation:new".to_owned(),
            new_resource_generation_name: "New Resource Layer".to_owned(),
            selected_enemy_camp_generation,
            enemy_camp_generation_draft,
            new_enemy_camp_generation_id: "enemy_camp_generation:new".to_owned(),
            world_asset_search: String::new(),
            world_preview_layer: WorldPreviewLayer::default(),
            new_technology_id: "technology:new".to_owned(),
            new_technology_name: "New Technology".to_owned(),
            new_group_id: "technology_group:new".to_owned(),
            new_group_name: "New Group".to_owned(),
            undo_authoring: Vec::new(),
            redo_authoring: Vec::new(),
            twitch_auth_events: None,
            twitch_device: None,
            twitch_validation: None,
            twitch_broadcast_validation: None,
            twitch_channel_identity: None,
            twitch_irc_verified: false,
            game_master_ids,
            game_master_lookup,
            fish_god_reward_id,
            tool_job_events: None,
            runtime_console: RuntimeConsoleStore::from_environment(),
            runtime_status: None,
            runtime_process: None,
            runtime_profile_launch: false,
            runtime_sequence: 0,
            runtime_actor_id: "tool:operator".to_owned(),
            runtime_login: "tool_operator".to_owned(),
            music_preview_intensity: 4.0,
            music_preview_season: 0.0,
            music_preview_time_of_day: 0.5,
            music_preview_population: 20,
            music_preview_building_count: 10,
            music_preview_state: MusicPreviewState::Stopped,
            music_preview_rendered_score: String::new(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    if std::env::args().any(|argument| argument == "--validate-authoring") {
        let summary = validate_authoring_assets()?;
        println!("{summary}");
        return Ok(());
    }
    let asset_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    let default_plugins = DefaultPlugins
        .set(AssetPlugin {
            file_path: asset_root.to_string_lossy().into_owned(),
            ..default()
        })
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Stream Town Tools".to_owned(),
                resolution: (1_440, 900).into(),
                ..default()
            }),
            ..default()
        });
    #[cfg(target_os = "windows")]
    let default_plugins = default_plugins.set(RenderPlugin {
        render_creation: tools_wgpu_settings().into(),
        ..default()
    });
    App::new()
        .add_plugins(default_plugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(TidalPlugin {
            config: TidalConfig {
                samples_path: asset_root.join("music").join("samples"),
                cycles_per_second: 0.3125,
                scheduler_lookahead: Duration::from_millis(100),
            },
        })
        .init_resource::<ToolState>()
        .add_systems(
            PreStartup,
            setup_camera.before(EguiStartupSet::InitContexts),
        )
        .add_systems(Startup, setup_model_preview)
        .add_systems(Update, drive_model_preview_animation)
        .add_systems(
            PostUpdate,
            (
                apply_preview_material_overrides,
                apply_preview_node_visibility,
                frame_model_preview,
                update_model_preview_camera,
            )
                .chain()
                .after(TransformSystems::Propagate),
        )
        .add_systems(
            EguiPrimaryContextPass,
            (sync_model_preview, tools_ui).chain(),
        )
        .run();
    Ok(())
}

include!("ui/preview_runtime.rs");
include!("ui/shell.rs");
include!("ui/music_preview.rs");
include!("ui/asset_editors.rs");
include!("ui/content_editor.rs");
include!("ui/building_editor.rs");
include!("ui/role_editor.rs");
include!("ui/progression_editor.rs");
include!("ui/world_editor.rs");
include!("ui/runtime_editor.rs");
include!("ui/twitch_editor.rs");
include!("ui/persistence.rs");
include!("ui/field_editors.rs");
include!("ui/catalog_mutations.rs");
include!("ui/tests.rs");
