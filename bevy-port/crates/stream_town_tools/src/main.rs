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
    Migration,
    Authority,
    Assets,
    Buildings,
    Roles,
    Technology,
    Terrain,
    Music,
    World,
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
    const ALL: [Self; 10] = [
        Self::Migration,
        Self::Authority,
        Self::Assets,
        Self::Buildings,
        Self::Roles,
        Self::Technology,
        Self::Terrain,
        Self::Music,
        Self::World,
        Self::Validation,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Migration => "Migration",
            Self::Authority => "Game Authority",
            Self::Assets => "Models + Assets",
            Self::Buildings => "Buildings",
            Self::Roles => "Roles",
            Self::Technology => "Technology",
            Self::Terrain => "Terrain",
            Self::Music => "Music",
            Self::World => "World + Nav",
            Self::Validation => "Validation",
        }
    }
}

#[derive(Resource)]
struct ToolState {
    tab: ToolTab,
    unity_root: String,
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
    role_search: String,
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
    display_name: String,
    description: String,
    age: String,
    tier: i32,
    group: Option<StableId>,
    prerequisites: String,
    unlocks: String,
    objectives: String,
    icon_path: String,
    initially_unlocked: bool,
    unavailable: bool,
    prerequisite_search: String,
    unlock_search: String,
    objective_search: String,
    icon_search: String,
}

#[derive(Clone)]
struct RoleDraft {
    id: StableId,
    value: RoleDef,
    resource: String,
    station_kinds: String,
    target_kinds: String,
    granted_abilities: String,
    has_equipment: bool,
    body_nodes: [String; 3],
    left_hand_node: String,
    right_hand_node: String,
    helmet_node: String,
    carry_animation: String,
    left_hand_permanent: bool,
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
        let config = load_game_config(config_path.to_string_lossy().as_ref())
            .expect("checked-in game configuration must parse and validate");
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
            unity_root: "..".to_owned(),
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
            role_search: String::new(),
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

#[cfg(target_os = "windows")]
fn tools_wgpu_settings() -> WgpuSettings {
    // Match the shipping game: the current Windows AMD driver emits invalid
    // Vulkan swapchain transitions, while DX12 is stable and supported.
    WgpuSettings {
        backends: Some(Backends::DX12),
        ..default()
    }
}

fn setup_camera(mut commands: Commands, mut egui_settings: ResMut<EguiGlobalSettings>) {
    egui_settings.auto_create_primary_context = false;
    commands.spawn((Camera2d, PrimaryEguiContext));
}

fn setup_model_preview(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut egui_textures: ResMut<EguiUserTextures>,
) {
    let image = images.add(Image::new_target_texture(
        768,
        512,
        TextureFormat::Bgra8UnormSrgb,
        None,
    ));
    egui_textures.add_image(EguiTextureHandle::Strong(image.clone()));
    commands.spawn((
        ModelPreviewCamera,
        Camera3d::default(),
        Camera {
            order: -1,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.035, 0.05, 0.065)),
            ..default()
        },
        RenderTarget::Image(image.clone().into()),
        AmbientLight {
            color: Color::srgb(0.82, 0.88, 0.96),
            brightness: 260.0,
            ..default()
        },
        Transform::from_xyz(5.5, 4.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 9_000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.insert_resource(ModelPreviewRuntime {
        image,
        scene_entity: None,
        scene_handle: None,
        loaded_request: None,
        animation_graph: None,
        animation_node: None,
        animation_started: false,
        material_overrides: PreviewMaterialOverrides::default(),
        visible_nodes: None,
        camera_target_offset: Vec3::ZERO,
        framed: false,
        status: "Choose a model to render it here".to_owned(),
        controls: ModelPreviewControls::default(),
    });
}

fn sync_model_preview(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<ToolState>,
    mut preview: ResMut<ModelPreviewRuntime>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    if preview.loaded_request == state.preview_request {
        return;
    }
    if let Some(entity) = preview.scene_entity.take() {
        commands.entity(entity).try_despawn();
    }
    preview.scene_handle = None;
    preview.animation_graph = None;
    preview.animation_node = None;
    preview.animation_started = false;
    preview.material_overrides = PreviewMaterialOverrides::default();
    preview.visible_nodes = None;
    preview.camera_target_offset = Vec3::ZERO;
    preview.framed = false;
    preview.loaded_request.clone_from(&state.preview_request);
    let Some(request) = state.preview_request.as_ref() else {
        "Nothing selected for preview".clone_into(&mut preview.status);
        return;
    };
    match request {
        PreviewRequest::Model(asset_path) => {
            preview.material_overrides =
                preview_material_overrides(asset_path, &state, &asset_server, &mut materials);
            let scene_handle =
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(asset_path.clone()));
            let entity = commands
                .spawn((
                    Name::new(format!("Model preview: {}", state.preview_label)),
                    ModelPreviewScene,
                    WorldAssetRoot(scene_handle.clone()),
                    Transform::IDENTITY,
                ))
                .id();
            preview.scene_entity = Some(entity);
            preview.scene_handle = Some(scene_handle);
            preview.status = format!("Loading textured GLB: {asset_path}");
        }
        PreviewRequest::Animation {
            clip,
            asset_path,
            animation_index,
            ..
        } => {
            preview.material_overrides = if asset_path == PLAYER_ANIMATED_MODEL_PATH {
                player_preview_material_overrides(&state, &asset_server, &mut materials)
            } else {
                preview_material_overrides(asset_path, &state, &asset_server, &mut materials)
            };
            let scene_handle =
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(asset_path.clone()));
            let entity = commands
                .spawn((
                    Name::new(format!("Animation preview: {clip}")),
                    ModelPreviewScene,
                    WorldAssetRoot(scene_handle.clone()),
                    Transform::IDENTITY,
                ))
                .id();
            let animation = asset_server.load(
                GltfAssetLabel::Animation(
                    usize::try_from(*animation_index).expect("animation index fits platform"),
                )
                .from_asset(asset_path.clone()),
            );
            let (graph, node) = AnimationGraph::from_clip(animation);
            preview.scene_entity = Some(entity);
            preview.scene_handle = Some(scene_handle);
            preview.animation_graph = Some(animation_graphs.add(graph));
            preview.animation_node = Some(node);
            if let Some(definition) = state.presentation.clips.get(clip) {
                preview.controls.animation_looping = definition.looping;
            }
            preview.status = format!(
                "Loading animation {} from {}#Animation{}",
                state.preview_label, asset_path, animation_index
            );
        }
        PreviewRequest::Role {
            role,
            asset_path,
            animation_index,
            looping,
            visible_nodes,
            ..
        } => {
            preview.material_overrides =
                player_preview_material_overrides(&state, &asset_server, &mut materials);
            preview.visible_nodes = Some(visible_nodes.clone());
            // The imported skinned AABBs describe the rest pose and sit below
            // the head motion. Aim at the visible animated figure's centre.
            preview.camera_target_offset = Vec3::Y * 0.75;
            let scene_handle =
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(asset_path.clone()));
            let entity = commands
                .spawn((
                    Name::new(format!("Role preview: {role}")),
                    ModelPreviewScene,
                    WorldAssetRoot(scene_handle.clone()),
                    Transform::IDENTITY,
                ))
                .id();
            let animation = asset_server.load(
                GltfAssetLabel::Animation(
                    usize::try_from(*animation_index).expect("animation index fits platform"),
                )
                .from_asset(asset_path.clone()),
            );
            let (graph, node) = AnimationGraph::from_clip(animation);
            preview.scene_entity = Some(entity);
            preview.scene_handle = Some(scene_handle);
            preview.animation_graph = Some(animation_graphs.add(graph));
            preview.animation_node = Some(node);
            preview.controls.animation_looping = *looping;
            preview.status = format!(
                "Loading composed role {} with animation #{}",
                state.preview_label, animation_index
            );
        }
        PreviewRequest::Material { id, .. } => {
            let Some(definition) = state.presentation.materials.get(id) else {
                preview.status = format!("Missing material {id}");
                return;
            };
            let material = materials.add(preview_standard_material(
                definition,
                &state.presentation,
                &asset_server,
            ));
            let entity = commands
                .spawn((
                    Name::new(format!("Material preview: {id}")),
                    ModelPreviewScene,
                    Mesh3d(
                        meshes.add(
                            Sphere::new(1.55)
                                .mesh()
                                .ico(5)
                                .expect("valid preview sphere"),
                        ),
                    ),
                    MeshMaterial3d(material),
                    Transform::from_xyz(0.0, 0.25, 0.0),
                ))
                .id();
            preview.scene_entity = Some(entity);
            preview.framed = true;
            preview.status = format!("Live PBR material preview · {}", definition.display_name);
        }
        PreviewRequest::Texture { id, .. } => {
            let Some(definition) = state.presentation.textures.get(id) else {
                preview.status = format!("Missing texture {id}");
                return;
            };
            let material = materials.add(StandardMaterial {
                base_color: Color::WHITE,
                base_color_texture: Some(asset_server.load(definition.asset_path.clone())),
                perceptual_roughness: 0.82,
                cull_mode: None,
                ..default()
            });
            let entity = commands
                .spawn((
                    Name::new(format!("Texture preview: {id}")),
                    ModelPreviewScene,
                    Mesh3d(meshes.add(Cuboid::new(3.8, 2.6, 0.08))),
                    MeshMaterial3d(material),
                    Transform::IDENTITY,
                ))
                .id();
            preview.scene_entity = Some(entity);
            preview.framed = true;
            preview.status = format!("Live texture preview · {}", definition.asset_path);
        }
    }
}

fn preview_standard_material(
    material: &MaterialDef,
    presentation: &PresentationCatalog,
    asset_server: &AssetServer,
) -> StandardMaterial {
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
    let primary = PRIORITY
        .iter()
        .filter_map(|slot| material.textures.get_key_value(*slot))
        .chain(material.textures.iter())
        .find_map(|(slot, id)| {
            presentation
                .textures
                .get(id)
                .map(|texture| (slot.as_str(), texture.asset_path.as_str()))
        });
    let transform = primary
        .and_then(|(slot, _)| material.texture_transforms.get(slot))
        .copied()
        .unwrap_or_default();
    StandardMaterial {
        base_color: Color::srgba(
            material.base_color[0],
            material.base_color[1],
            material.base_color[2],
            material.base_color[3],
        ),
        base_color_texture: primary.map(|(_, path)| asset_server.load(path.to_owned())),
        emissive: LinearRgba::new(
            material.emissive[0],
            material.emissive[1],
            material.emissive[2],
            material.emissive[3],
        ),
        metallic: material.metallic,
        perceptual_roughness: material.perceptual_roughness,
        alpha_mode: match material.alpha_mode {
            MaterialAlphaMode::Opaque => AlphaMode::Opaque,
            MaterialAlphaMode::Mask => AlphaMode::Mask(0.5),
            MaterialAlphaMode::Blend => AlphaMode::Blend,
        },
        uv_transform: Affine2::from_scale_angle_translation(
            Vec2::from_array(transform.scale),
            0.0,
            Vec2::from_array(transform.offset),
        ),
        cull_mode: None,
        ..default()
    }
}

fn preview_material_overrides(
    asset_path: &str,
    state: &ToolState,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
) -> PreviewMaterialOverrides {
    let selected = state
        .selected_archetype
        .as_ref()
        .and_then(|id| state.catalog.archetypes.get(id))
        .filter(|archetype| {
            archetype
                .scenes
                .iter()
                .any(|scene| scene.asset_path == asset_path)
        });
    let archetype = selected.or_else(|| {
        state.catalog.archetypes.values().find(|archetype| {
            archetype
                .scenes
                .iter()
                .any(|scene| scene.asset_path == asset_path)
        })
    });
    let Some(archetype) = archetype else {
        return PreviewMaterialOverrides::default();
    };
    let Some(scene) = archetype
        .scenes
        .iter()
        .find(|scene| scene.asset_path == asset_path)
    else {
        return PreviewMaterialOverrides::default();
    };
    let make_material = |id: &StableId, materials: &mut Assets<StandardMaterial>| {
        state.presentation.materials.get(id).map(|definition| {
            materials.add(preview_standard_material(
                definition,
                &state.presentation,
                asset_server,
            ))
        })
    };
    let fallback = state
        .presentation
        .prefab_materials
        .get(&archetype.source_guid)
        .into_iter()
        .flatten()
        .find_map(|id| make_material(id, materials));
    let model_materials = state
        .presentation
        .model_materials
        .get(&scene.source_model)
        .into_iter()
        .flat_map(|bindings| bindings.iter())
        .filter_map(|(name, id)| {
            make_material(id, materials).map(|material| (name.clone(), material))
        })
        .collect();
    let renderer_materials = state
        .presentation
        .prefab_renderer_materials
        .get(&archetype.source_guid)
        .into_iter()
        .flatten()
        .filter_map(|binding| {
            let bound = binding
                .materials
                .iter()
                .filter_map(|(name, id)| {
                    make_material(id, materials).map(|material| (name.clone(), material))
                })
                .collect::<BTreeMap<_, _>>();
            (!bound.is_empty()).then(|| PreviewRendererMaterialBinding {
                target_path: binding.target_path.clone(),
                materials: bound,
            })
        })
        .collect();
    PreviewMaterialOverrides {
        fallback,
        model_materials,
        renderer_materials,
    }
}

fn player_preview_material_overrides(
    state: &ToolState,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
) -> PreviewMaterialOverrides {
    let Some(archetype) = state
        .catalog
        .archetypes
        .values()
        .find(|archetype| archetype.source_path.ends_with("Player_Character.prefab"))
    else {
        return PreviewMaterialOverrides::default();
    };
    let make_material = |id: &StableId, materials: &mut Assets<StandardMaterial>| {
        state.presentation.materials.get(id).map(|definition| {
            materials.add(preview_standard_material(
                definition,
                &state.presentation,
                asset_server,
            ))
        })
    };
    let fallback = state
        .presentation
        .prefab_materials
        .get(&archetype.source_guid)
        .into_iter()
        .flatten()
        .find_map(|id| make_material(id, materials));
    let model_materials = state
        .presentation
        .model_materials
        .get(PLAYER_ANIMATED_SOURCE_MODEL)
        .into_iter()
        .flat_map(|bindings| bindings.iter())
        .filter_map(|(name, id)| {
            make_material(id, materials).map(|material| (name.clone(), material))
        })
        .collect();
    let renderer_materials = state
        .presentation
        .prefab_renderer_materials
        .get(&archetype.source_guid)
        .into_iter()
        .flatten()
        .filter_map(|binding| {
            let bound = binding
                .materials
                .iter()
                .filter_map(|(name, id)| {
                    make_material(id, materials).map(|material| (name.clone(), material))
                })
                .collect::<BTreeMap<_, _>>();
            (!bound.is_empty()).then(|| PreviewRendererMaterialBinding {
                target_path: binding.target_path.clone(),
                materials: bound,
            })
        })
        .collect();
    PreviewMaterialOverrides {
        fallback,
        model_materials,
        renderer_materials,
    }
}

#[allow(clippy::type_complexity)]
fn apply_preview_material_overrides(
    mut commands: Commands,
    preview: Res<ModelPreviewRuntime>,
    parents: Query<&ChildOf>,
    names: Query<&Name>,
    roots: Query<(), With<ModelPreviewScene>>,
    mut renderers: Query<
        (
            Entity,
            &mut MeshMaterial3d<StandardMaterial>,
            Option<&GltfMeshName>,
            Option<&GltfMaterialName>,
        ),
        Without<PreviewMaterialApplied>,
    >,
) {
    for (entity, mut material, mesh_name, material_name) in &mut renderers {
        let mut ancestor = entity;
        let mut path = Vec::new();
        let mut belongs_to_preview = false;
        for _ in 0..64 {
            if roots.contains(ancestor) {
                belongs_to_preview = true;
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
        if !belongs_to_preview {
            continue;
        }
        let hierarchy_path = path.iter().rev().cloned().collect::<Vec<_>>().join("/");
        if let Some(authored) = resolved_preview_material(
            &preview.material_overrides,
            &hierarchy_path,
            mesh_name.map(|name| name.0.as_str()),
            material_name.map(|name| name.0.as_str()),
        ) {
            material.0 = authored.clone();
        }
        commands.entity(entity).insert(PreviewMaterialApplied);
    }
}

fn canonical_preview_node_name(name: &str) -> &str {
    name.strip_suffix("_Starter").unwrap_or(name)
}

fn player_preview_controlled_node(name: &str) -> bool {
    const COSMETIC_PREFIXES: [&str; 3] = ["Eyes_", "Hair_", "FacialHair_"];
    const EQUIPMENT_PREFIXES: [&str; 5] = ["Body_", "Back_", "LHand_", "RHand_", "Helmet_"];
    let name = canonical_preview_node_name(name);
    COSMETIC_PREFIXES
        .iter()
        .chain(EQUIPMENT_PREFIXES.iter())
        .any(|prefix| name.starts_with(prefix))
}

fn apply_preview_node_visibility(
    mut commands: Commands,
    preview: Res<ModelPreviewRuntime>,
    parents: Query<&ChildOf>,
    roots: Query<(), With<ModelPreviewScene>>,
    mut nodes: Query<(Entity, &Name, &mut Visibility), Without<PreviewNodeVisibilityApplied>>,
) {
    let Some(visible_nodes) = preview.visible_nodes.as_ref() else {
        return;
    };
    for (entity, name, mut visibility) in &mut nodes {
        let mut ancestor = entity;
        let mut belongs_to_preview = roots.contains(entity);
        for _ in 0..64 {
            if belongs_to_preview {
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
            belongs_to_preview = roots.contains(ancestor);
        }
        if !belongs_to_preview {
            continue;
        }
        let canonical = canonical_preview_node_name(name.as_str());
        if player_preview_controlled_node(canonical) {
            *visibility = if visible_nodes.contains(canonical) {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
        commands.entity(entity).insert(PreviewNodeVisibilityApplied);
    }
}

fn resolved_preview_material<'a>(
    overrides: &'a PreviewMaterialOverrides,
    hierarchy_path: &str,
    mesh_name: Option<&str>,
    material_name: Option<&str>,
) -> Option<&'a Handle<StandardMaterial>> {
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
    let full_path_matches = |binding: &&PreviewRendererMaterialBinding| {
        binding.target_path == node_path
            || node_path.ends_with(&format!("/{}", binding.target_path))
            || binding.target_path.ends_with(&format!("/{node_path}"))
    };
    let name_matches = |binding: &&PreviewRendererMaterialBinding| {
        binding.target_path.rsplit('/').next() == Some(target_name)
    };
    let exact_path = material_name
        .and_then(|name| {
            overrides
                .renderer_materials
                .iter()
                .filter(full_path_matches)
                .find(|binding| binding.materials.contains_key(name))
        })
        .or_else(|| overrides.renderer_materials.iter().find(full_path_matches));
    let exact = exact_path.or_else(|| {
        material_name
            .and_then(|name| {
                overrides
                    .renderer_materials
                    .iter()
                    .filter(name_matches)
                    .find(|binding| binding.materials.contains_key(name))
            })
            .or_else(|| overrides.renderer_materials.iter().find(name_matches))
    });
    material_name
        .and_then(|name| exact.and_then(|binding| binding.materials.get(name)))
        .or_else(|| material_name.and_then(|name| overrides.model_materials.get(name)))
        .or_else(|| exact.and_then(|binding| binding.materials.values().next()))
        .or(overrides.fallback.as_ref())
}

fn frame_model_preview(
    asset_server: Res<AssetServer>,
    mut preview: ResMut<ModelPreviewRuntime>,
    children: Query<&Children>,
    bounds: Query<(&GlobalTransform, &Aabb)>,
    mut roots: Query<&mut Transform, With<ModelPreviewScene>>,
) {
    if preview.framed {
        return;
    }
    let Some(root) = preview.scene_entity else {
        return;
    };
    let Some(handle) = preview.scene_handle.as_ref() else {
        return;
    };
    match asset_server.load_state(handle.id()) {
        LoadState::Failed(error) => {
            preview.status = format!("GLB load failed: {error}");
            return;
        }
        LoadState::Loading | LoadState::NotLoaded => return,
        LoadState::Loaded => {}
    }

    let mut stack = children
        .get(root)
        .map_or_else(|_| Vec::new(), |value| value.iter().collect::<Vec<_>>());
    let mut minimum = Vec3::splat(f32::INFINITY);
    let mut maximum = Vec3::splat(f32::NEG_INFINITY);
    let mut mesh_count = 0_usize;
    while let Some(entity) = stack.pop() {
        if let Ok(value) = children.get(entity) {
            stack.extend(value.iter());
        }
        let Ok((global, aabb)) = bounds.get(entity) else {
            continue;
        };
        mesh_count += 1;
        let center = Vec3::from(aabb.center);
        let half = Vec3::from(aabb.half_extents);
        for x in [-1.0, 1.0] {
            for y in [-1.0, 1.0] {
                for z in [-1.0, 1.0] {
                    let point = global.transform_point(center + half * Vec3::new(x, y, z));
                    minimum = minimum.min(point);
                    maximum = maximum.max(point);
                }
            }
        }
    }
    if mesh_count == 0 {
        "GLB loaded; waiting for render meshes".clone_into(&mut preview.status);
        return;
    }
    let size = maximum - minimum;
    let largest = size.max_element().max(0.01);
    let scale = 4.0 / largest;
    let center = (minimum + maximum) * 0.5;
    let Ok(mut transform) = roots.get_mut(root) else {
        return;
    };
    *transform = Transform::from_translation(-center * scale).with_scale(Vec3::splat(scale));
    preview.framed = true;
    let authored_bindings = usize::from(preview.material_overrides.fallback.is_some())
        + preview.material_overrides.model_materials.len()
        + preview
            .material_overrides
            .renderer_materials
            .iter()
            .map(|binding| binding.materials.len())
            .sum::<usize>();
    let appearance = if authored_bindings == 0 {
        "embedded GLB materials".to_owned()
    } else {
        format!("{authored_bindings} authored material binding(s)")
    };
    preview.status = if preview.animation_graph.is_some() {
        format!("Loaded and framed {mesh_count} mesh(es) with {appearance}; attaching animation")
    } else {
        format!("Loaded and framed {mesh_count} mesh(es) with {appearance}")
    };
}

fn update_model_preview_camera(
    preview: Res<ModelPreviewRuntime>,
    mut cameras: Query<&mut Transform, With<ModelPreviewCamera>>,
) {
    if !preview.is_changed() {
        return;
    }
    let controls = &preview.controls;
    let target = preview.camera_target_offset + controls.pan;
    let offset = preview_camera_offset(controls.yaw, controls.pitch, controls.distance);
    for mut transform in &mut cameras {
        *transform = Transform::from_translation(target + offset).looking_at(target, Vec3::Y);
    }
}

fn preview_camera_offset(yaw: f32, pitch: f32, distance: f32) -> Vec3 {
    let horizontal = distance * pitch.cos();
    Vec3::new(
        yaw.sin() * horizontal,
        distance * pitch.sin(),
        yaw.cos() * horizontal,
    )
}

fn preview_pan_delta(yaw: f32, pitch: f32, drag_x: f32, drag_y: f32, scale: f32) -> Vec3 {
    let camera_offset = preview_camera_offset(yaw, pitch, 1.0);
    let forward = (-camera_offset).normalize_or_zero();
    let right = forward.cross(Vec3::Y).normalize_or_zero();
    let up = right.cross(forward).normalize_or_zero();
    -right * drag_x * scale + up * drag_y * scale
}

fn drive_model_preview_animation(
    mut commands: Commands,
    mut preview: ResMut<ModelPreviewRuntime>,
    mut players: Query<(Entity, &mut AnimationPlayer)>,
) {
    let (Some(graph), Some(node)) = (preview.animation_graph.clone(), preview.animation_node)
    else {
        return;
    };
    let mut attached = false;
    let restart = preview.controls.restart_animation;
    for (entity, mut player) in &mut players {
        if !player.is_playing_animation(node) {
            player.play(node);
            commands
                .entity(entity)
                .insert(AnimationGraphHandle(graph.clone()));
        }
        if let Some(active) = player.animation_mut(node) {
            if restart {
                active.rewind();
            }
            active.set_speed(preview.controls.animation_speed);
            active.set_repeat(if preview.controls.animation_looping {
                RepeatAnimation::Forever
            } else {
                RepeatAnimation::Never
            });
            if preview.controls.animation_playing {
                active.resume();
            } else {
                active.pause();
            }
            attached = true;
        }
    }
    if attached && !preview.animation_started {
        preview.animation_started = true;
        "Animation playing on the textured preview rig".clone_into(&mut preview.status);
    }
    if restart {
        preview.controls.restart_animation = false;
    }
}

fn tools_ui(
    mut contexts: EguiContexts,
    mut state: ResMut<ToolState>,
    mut preview: ResMut<ModelPreviewRuntime>,
) -> Result {
    poll_twitch_tool_events(&mut state);
    poll_tool_job_events(&mut state);
    poll_runtime_console(&mut state);
    update_preview_request(&mut state);
    let preview_texture = contexts.image_id(&preview.image);
    let preview_status = preview.status.clone();
    let context = contexts.ctx_mut()?;
    let mut viewport_ui = egui::Ui::new(
        context.clone(),
        "stream_town_tools".into(),
        egui::UiBuilder::new()
            .layer_id(egui::LayerId::background())
            .max_rect(context.viewport_rect()),
    );
    egui::Panel::top("top").show(&mut viewport_ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.heading("Stream Town Tools");
            ui.separator();
            for tab in ToolTab::ALL {
                if ui.selectable_label(state.tab == tab, tab.label()).clicked() {
                    state.tab = tab;
                }
            }
        });
    });
    egui::Panel::bottom("status").show(&mut viewport_ui, |ui| {
        ui.label(&state.status);
    });
    egui::CentralPanel::default().show(&mut viewport_ui, |ui| match state.tab {
        ToolTab::Migration => migration_tab(ui, &mut state),
        ToolTab::Authority => authority_tab(ui, &mut state),
        ToolTab::Assets => content_tab(
            ui,
            &mut state,
            preview_texture,
            &preview_status,
            &mut preview.controls,
        ),
        ToolTab::Buildings => {
            buildings_tab(
                ui,
                &mut state,
                preview_texture,
                &preview_status,
                &mut preview.controls,
            );
        }
        ToolTab::Roles => roles_tab(
            ui,
            &mut state,
            preview_texture,
            &preview_status,
            &mut preview.controls,
        ),
        ToolTab::Technology => technology_tab(ui, &mut state),
        ToolTab::Terrain => terrain_tab(ui, &mut state),
        ToolTab::Music => music_tab(ui, &mut state),
        ToolTab::World => world_tab(
            ui,
            &mut state,
            preview_texture,
            &preview_status,
            &mut preview.controls,
        ),
        ToolTab::Validation => validation_tab(ui, &mut state),
    });
    Ok(())
}

fn migration_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Unity migration dashboard");
    ui.label("Inventory GUIDs, YAML assets, prefab references, and legacy saves without modifying Unity sources.");
    ui.horizontal(|ui| {
        ui.label("Unity root");
        ui.text_edit_singleline(&mut state.unity_root);
    });
    if ui.button("Prepare inventory command").clicked() {
        state.status = format!(
            "Run: cargo run -p stream_town_migrate -- inventory {:?} --out generated/content-manifest.json",
            state.unity_root
        );
    }
    ui.separator();
    ui.label(format!(
        "Active catalog: {} archetypes, {} buildings, {} roles, {} technologies, {} materials, {} renderer bindings, {} controllers, {} source records",
        state.catalog.archetypes.len(),
        state.catalog.buildings.len(),
        state.catalog.roles.len(),
        state.catalog.technology.nodes.len(),
        state.presentation.materials.len(),
        state
            .presentation
            .prefab_renderer_materials
            .values()
            .map(Vec::len)
            .sum::<usize>(),
        state.presentation.controllers.len(),
        state.catalog.source_records.len()
    ));
    ui.monospace(".\\bevy-port\\scripts\\export-unity.ps1");
    ui.monospace(".\\bevy-port\\scripts\\convert-models.ps1");
    ui.label(
        "Manifest stages: discovered -> referenced -> converted -> manually reviewed -> packaged",
    );
}

fn authority_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Authoritative game settings");
    ui.label(
        "These values drive deterministic world creation and simulation. Save + apply writes both the source-controlled baseline and the runtime configuration used on the next game launch.",
    );
    ui.horizontal_wrapped(|ui| {
        ui.label("Project config");
        ui.text_edit_singleline(&mut state.config_path);
        if ui.button("Reload").clicked() {
            state.status = match load_game_config(&state.config_path) {
                Ok(config) => {
                    state.config = config;
                    sync_twitch_tool_fields(state);
                    state.generated_world = None;
                    "Reloaded and validated authoritative game configuration".to_owned()
                }
                Err(error) => format!("Could not reload game configuration: {error:#}"),
            };
        }
        if ui.button("Validate").clicked() {
            state.status = match state.config.validate() {
                Ok(()) => "Authoritative game configuration is valid".to_owned(),
                Err(error) => format!("Game configuration error: {error}"),
            };
        }
        if ui.button("Save + apply to game").clicked() {
            state.status = match save_and_apply_game_config(&state.config, &state.config_path) {
                Ok((project, runtime)) => format!(
                    "Saved {} and applied {} while preserving local Twitch setup; restart the game to load the changes",
                    project.display(),
                    runtime.display()
                ),
                Err(error) => format!("Could not save and apply game configuration: {error:#}"),
            };
        }
        if ui.button("Save baseline only").clicked() {
            state.status = match save_game_config(&state.config, &state.config_path) {
                Ok(path) => format!(
                    "Saved project baseline to {}; it is not the active runtime override",
                    path.display()
                ),
                Err(error) => format!("Could not save project game configuration: {error:#}"),
            };
        }
        if ui.button("Apply locally only").clicked() {
            state.status = match save_authoring_runtime_config(&state.config) {
                Ok(path) => format!(
                    "Applied authored runtime settings to {} without replacing local Twitch setup; restart the game to load them",
                    path.display()
                ),
                Err(error) => format!("Could not save runtime override: {error:#}"),
            };
        }
    });

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.separator();
        ui.collapsing("Window and boot", |ui| {
            ui.horizontal(|ui| {
                ui.label("Title");
                ui.text_edit_singleline(&mut state.config.window.title);
            });
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut state.config.window.width)
                        .range(640..=16_384)
                        .prefix("Width "),
                );
                ui.add(
                    egui::DragValue::new(&mut state.config.window.height)
                        .range(480..=8_640)
                        .prefix("Height "),
                );
            });
        });
        ui.collapsing("Simulation", |ui| {
            ui.add(
                egui::DragValue::new(&mut state.config.gameplay.initial_agents)
                    .range(1..=5_000)
                    .prefix("Initial actors "),
            );
            ui.add(
                egui::DragValue::new(&mut state.config.gameplay.agent_speed_cells_per_second)
                    .range(0.01..=100.0)
                    .speed(0.05)
                    .prefix("Actor speed ")
                    .suffix(" cells/s"),
            );
            ui.add(
                egui::DragValue::new(&mut state.config.gameplay.repath_interval_seconds)
                    .range(0.01..=60.0)
                    .speed(0.05)
                    .prefix("Repath interval ")
                    .suffix(" s"),
            );
        });
        ui.collapsing("Starting town resources", |ui| {
            let ids: Vec<_> = state
                .config
                .gameplay
                .starting_town_resources
                .keys()
                .cloned()
                .collect();
            egui::Grid::new("authority_starting_resources")
                .striped(true)
                .show(ui, |ui| {
                    ui.strong("Stable ID");
                    ui.strong("Starting amount");
                    ui.end_row();
                    for id in ids {
                        ui.monospace(id.to_string());
                        if let Some(amount) =
                            state.config.gameplay.starting_town_resources.get_mut(&id)
                        {
                            ui.add(egui::DragValue::new(amount).range(0..=u32::MAX));
                        }
                        ui.end_row();
                    }
                });
        });
        ui.collapsing("Base resource capacities", |ui| {
            ui.label("Resources omitted from this table are intentionally unbounded.");
            let ids: Vec<_> = state
                .config
                .gameplay
                .base_town_resource_capacity
                .keys()
                .cloned()
                .collect();
            egui::Grid::new("authority_resource_capacities")
                .striped(true)
                .show(ui, |ui| {
                    ui.strong("Stable ID");
                    ui.strong("Capacity");
                    ui.end_row();
                    for id in ids {
                        ui.monospace(id.to_string());
                        if let Some(amount) = state
                            .config
                            .gameplay
                            .base_town_resource_capacity
                            .get_mut(&id)
                        {
                            ui.add(egui::DragValue::new(amount).range(0..=u32::MAX));
                        }
                        ui.end_row();
                    }
                });
        });
        ui.collapsing("Time, lighting, and emission", |ui| {
            ui.add(
                egui::DragValue::new(&mut state.config.time.seconds_per_day)
                    .range(1..=86_400)
                    .suffix(" seconds/day"),
            );
            ui.add(
                egui::Slider::new(&mut state.config.time.daylight_per_thousand, 1..=999)
                    .text("Daylight fraction (per thousand)"),
            );
            ui.add(
                egui::DragValue::new(&mut state.config.time.transition_seconds)
                    .range(0..=3_600)
                    .suffix(" transition seconds"),
            );
            ui.add(
                egui::DragValue::new(&mut state.config.time.day_light_intensity_milli)
                    .range(1..=65_535)
                    .suffix(" day intensity milli"),
            );
            ui.add(
                egui::DragValue::new(&mut state.config.time.night_light_intensity_milli)
                    .range(0..=65_535)
                    .suffix(" night intensity milli"),
            );
            ui.add(
                egui::DragValue::new(&mut state.config.time.max_building_emission_milli)
                    .range(0..=65_535)
                    .suffix(" max building emission milli"),
            );
        });
        ui.separator();
        match state.config.validate() {
            Ok(()) => ui.colored_label(egui::Color32::LIGHT_GREEN, "Configuration is valid"),
            Err(error) => ui.colored_label(
                egui::Color32::LIGHT_RED,
                format!("Configuration is not saveable: {error}"),
            ),
        };
    });
}

fn authoring_config_save_bar(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.horizontal_wrapped(|ui| {
        if ui.button("Save + apply to game").clicked() {
            state.status = match save_and_apply_game_config(&state.config, &state.config_path) {
                Ok((project, runtime)) => format!(
                    "Saved {} and applied {} while preserving local Twitch setup; restart the game to load the changes",
                    project.display(),
                    runtime.display()
                ),
                Err(error) => format!("Could not save and apply game configuration: {error:#}"),
            };
        }
        if ui.button("Reload").clicked() {
            state.status = match load_game_config(&state.config_path) {
                Ok(config) => {
                    state.config = config;
                    sync_twitch_tool_fields(state);
                    "Reloaded the authoritative game configuration".to_owned()
                }
                Err(error) => format!("Could not reload game configuration: {error:#}"),
            };
        }
        match state.config.validate() {
            Ok(()) => ui.colored_label(egui::Color32::LIGHT_GREEN, "Valid"),
            Err(error) => ui.colored_label(egui::Color32::LIGHT_RED, error.to_string()),
        };
    });
}

fn terrain_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Terrain texture and traversal wear");
    ui.label(
        "Seasonal colours are applied to the authored terrain shader. Traversal is recorded only when a citizen completes a cell crossing.",
    );
    authoring_config_save_bar(ui, state);
    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("seasonal_terrain_palettes")
            .striped(true)
            .show(ui, |ui| {
                ui.strong("Season");
                ui.strong("Terrain base multiplier");
                ui.strong("Built-path tint / opacity");
                ui.strong("Traversal-wear tint / opacity");
                ui.end_row();
                for (name, palette) in [
                    ("Spring", &mut state.config.terrain.spring),
                    ("Summer", &mut state.config.terrain.summer),
                    ("Autumn", &mut state.config.terrain.autumn),
                    ("Winter", &mut state.config.terrain.winter),
                ] {
                    ui.label(name);
                    ui.color_edit_button_rgba_unmultiplied(&mut palette.base_color);
                    ui.color_edit_button_rgba_unmultiplied(&mut palette.path_tint);
                    ui.color_edit_button_rgba_unmultiplied(&mut palette.traversal_tint);
                    ui.end_row();
                }
            });
        ui.separator();
        ui.heading("Traversal response");
        ui.add(
            egui::DragValue::new(&mut state.config.terrain.traversal_fade_start_per_minute)
                .range(0.0..=10_000.0)
                .speed(0.25)
                .prefix("Wear begins at ")
                .suffix(" crossings/minute"),
        );
        ui.add(
            egui::DragValue::new(&mut state.config.terrain.traversal_full_tint_per_minute)
                .range(0.01..=10_000.0)
                .speed(0.5)
                .prefix("Full tint at ")
                .suffix(" crossings/minute"),
        );
        ui.add(
            egui::DragValue::new(&mut state.config.terrain.traversal_half_life_seconds)
                .range(1.0..=604_800.0)
                .speed(10.0)
                .prefix("Half-life ")
                .suffix(" seconds"),
        );
        ui.add(
            egui::DragValue::new(&mut state.config.terrain.traversal_decay_pause_seconds)
                .range(0.0..=3_600.0)
                .speed(0.25)
                .prefix("Decay pause after crossing ")
                .suffix(" seconds"),
        );
        ui.add(
            egui::DragValue::new(&mut state.config.terrain.traversal_prune_score)
                .range(0.0..=100.0)
                .speed(0.001)
                .prefix("Sparse-map prune score "),
        );
    });
}

fn music_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Adaptive Bevy Tidal score");
    ui.label(
        "The five live variables below are available both as weighted energy inputs and as score-template placeholders. Changes take effect after saving and restarting the game.",
    );
    authoring_config_save_bar(ui, state);
    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| {
        let music = &mut state.config.music;
        ui.add(
            egui::DragValue::new(&mut music.intensity_smoothing_seconds)
                .range(0.05..=120.0)
                .speed(0.1)
                .prefix("Enemy intensity smoothing ")
                .suffix(" seconds"),
        );
        ui.add(
            egui::DragValue::new(&mut music.maximum_energy)
                .range(0.1..=1_000.0)
                .speed(0.25)
                .prefix("Maximum energy "),
        );
        ui.horizontal(|ui| {
            ui.add(
                egui::DragValue::new(&mut music.cycles_per_minute_base)
                    .range(1.0..=1_000.0)
                    .prefix("Base CPM "),
            );
            ui.add(
                egui::DragValue::new(&mut music.cycles_per_minute_per_energy)
                    .range(-100.0..=100.0)
                    .speed(0.1)
                    .prefix("CPM / energy "),
            );
        });
        ui.separator();
        egui::Grid::new("adaptive_music_variables")
            .striped(true)
            .show(ui, |ui| {
                ui.strong("Live variable");
                ui.strong("Template token");
                ui.strong("Energy weight");
                ui.end_row();
                for (name, token, weight) in [
                    ("Enemies on screen (smoothed)", "${intensity}", &mut music.intensity_weight),
                    ("Season (0 spring .. 3 winter)", "${season}", &mut music.season_weight),
                    ("Time of day (0 .. 1)", "${time_of_day}", &mut music.time_of_day_weight),
                    ("Living citizen population", "${population}", &mut music.population_weight),
                    ("Completed building count", "${building_count}", &mut music.building_count_weight),
                ] {
                    ui.label(name);
                    ui.monospace(token);
                    ui.add(egui::DragValue::new(weight).speed(0.01));
                    ui.end_row();
                }
                ui.label("Weighted/clamped result");
                ui.monospace("${energy}");
                ui.label("—");
                ui.end_row();
            });
        ui.separator();
        ui.label("Tidal score template");
        ui.add(
            egui::TextEdit::multiline(&mut music.score_template)
                .code_editor()
                .desired_rows(24)
                .desired_width(f32::INFINITY),
        );
        ui.small(
            "Generated voice placeholders such as ${kick}, ${roots}, ${melody_attack}, and ${hat_gain} remain available. Unresolved placeholders are rejected at runtime.",
        );
    });
}

fn player_animation_controller(
    presentation: &PresentationCatalog,
) -> Option<&AnimationControllerDef> {
    let binding = presentation.prefab_bindings.values().find(|binding| {
        binding
            .source_prefab_path
            .ends_with("Player_Character.prefab")
    })?;
    presentation.controllers.get(&binding.controller)
}

fn role_preview_animation_choices(presentation: &PresentationCatalog) -> Vec<(StableId, String)> {
    player_animation_controller(presentation).map_or_else(Vec::new, |controller| {
        controller
            .states
            .iter()
            .filter(|(_, state)| !state.motions.is_empty())
            .map(|(id, state)| (id.clone(), state.display_name.clone()))
            .collect()
    })
}

fn matching_role_animation_state(
    presentation: &PresentationCatalog,
    action_animation: &str,
) -> Option<StableId> {
    let controller = player_animation_controller(presentation)?;
    controller
        .states
        .iter()
        .find(|(_, state)| {
            state.display_name == action_animation
                || state
                    .display_name
                    .strip_suffix(" 0")
                    .is_some_and(|name| name == action_animation)
        })
        .or_else(|| {
            controller
                .states
                .iter()
                .find(|(_, state)| state.display_name == "Locomotion")
        })
        .map(|(id, _)| id.clone())
}

fn default_role_preview_animation(
    presentation: &PresentationCatalog,
    catalog: &ContentCatalog,
) -> Option<StableId> {
    catalog
        .roles
        .values()
        .next()
        .and_then(|role| matching_role_animation_state(presentation, &role.action_animation))
}

fn role_preview_animation_request(
    presentation: &PresentationCatalog,
    state_id: &StableId,
) -> Option<(String, u32, bool)> {
    let state = player_animation_controller(presentation)?
        .states
        .get(state_id)?;
    let motion = state.motions.iter().max_by(|left, right| {
        left.threshold
            .unwrap_or_default()
            .total_cmp(&right.threshold.unwrap_or_default())
    })?;
    let source = presentation.clips.get(&motion.clip)?;
    let (path, index) =
        preview_animation_asset_for_rig(source, PLAYER_ANIMATED_MODEL_PATH, presentation)?;
    Some((path, index, source.looping))
}

fn role_preview_visible_nodes(
    role: &RoleDef,
    body_type: u8,
    carrying: bool,
    eye: Option<&str>,
    hair: Option<&str>,
    facial_hair: Option<&str>,
) -> BTreeSet<String> {
    let mut visible = BTreeSet::new();
    let helmet_equipped = role
        .equipment
        .as_ref()
        .and_then(|equipment| equipment.helmet_node.as_ref())
        .is_some();
    if let Some(equipment) = role.equipment.as_ref() {
        visible.insert(
            equipment.body_nodes[usize::from(body_type).min(equipment.body_nodes.len() - 1)]
                .clone(),
        );
        visible.extend(equipment.right_hand_node.iter().cloned());
        visible.extend(equipment.helmet_node.iter().cloned());
        if equipment.left_hand_permanent || carrying {
            visible.extend(equipment.left_hand_node.iter().cloned());
        }
    } else {
        visible.insert(
            [
                "Body_Default_Slim",
                "Body_Default_Bulk",
                "Body_Default_Feminine",
            ][usize::from(body_type).min(2)]
            .to_owned(),
        );
    }
    visible.extend(eye.map(str::to_owned));
    if !helmet_equipped {
        visible.extend(hair.map(str::to_owned));
    }
    visible.extend(facial_hair.map(str::to_owned));
    visible
        .into_iter()
        .map(|name| canonical_preview_node_name(&name).to_owned())
        .collect()
}

fn update_preview_request(state: &mut ToolState) {
    let requested = match state.tab {
        ToolTab::Assets => match state.asset_section {
            AssetEditorSection::Models => state
                .selected_archetype
                .as_ref()
                .and_then(|id| state.catalog.archetypes.get(id))
                .and_then(|archetype| {
                    let index = state
                        .selected_archetype_scene
                        .min(archetype.scenes.len().saturating_sub(1));
                    archetype.scenes.get(index).map(|scene| {
                        (
                            PreviewRequest::Model(scene.asset_path.clone()),
                            format!("{} · {}", archetype.display_name, scene.asset_path),
                        )
                    })
                }),
            AssetEditorSection::Textures => state.selected_texture.as_ref().and_then(|id| {
                state.presentation.textures.get(id).map(|texture| {
                    (
                        PreviewRequest::Texture {
                            id: id.clone(),
                            fingerprint: debug_fingerprint(texture),
                        },
                        format!("{} · {}", texture.display_name, texture.asset_path),
                    )
                })
            }),
            AssetEditorSection::Materials => state.selected_material.as_ref().and_then(|id| {
                state.presentation.materials.get(id).map(|material| {
                    (
                        PreviewRequest::Material {
                            id: id.clone(),
                            fingerprint: debug_fingerprint(material),
                        },
                        format!("{} · {id}", material.display_name),
                    )
                })
            }),
            AssetEditorSection::Animations => state.selected_clip.as_ref().and_then(|id| {
                let clip = state.presentation.clips.get(id)?;
                let (asset_path, animation_index) = preview_animation_asset_for_rig(
                    clip,
                    PLAYER_ANIMATED_MODEL_PATH,
                    &state.presentation,
                )
                .or_else(|| {
                    Some((
                        clip.converted_asset_path
                            .as_ref()
                            .or(clip.rig_asset_path.as_ref())?
                            .clone(),
                        clip.gltf_animation_index?,
                    ))
                })?;
                Some((
                    PreviewRequest::Animation {
                        clip: id.clone(),
                        asset_path,
                        animation_index,
                        fingerprint: debug_fingerprint(clip),
                    },
                    format!("{} · {id}", clip.display_name),
                ))
            }),
        },
        ToolTab::Buildings => state
            .building_draft
            .as_ref()
            .and_then(|draft| state.catalog.archetypes.get(&draft.value.archetype))
            .and_then(|archetype| {
                archetype
                    .scenes
                    .iter()
                    .find(|scene| scene.is_default)
                    .or_else(|| archetype.scenes.first())
                    .map(|scene| {
                        (
                            PreviewRequest::Model(scene.asset_path.clone()),
                            format!("{} · {}", archetype.display_name, scene.asset_path),
                        )
                    })
            }),
        ToolTab::Roles => state.role_draft.as_ref().and_then(|draft| {
            let animation_state = state.role_preview_animation.clone().or_else(|| {
                matching_role_animation_state(&state.presentation, &draft.value.action_animation)
            })?;
            let (asset_path, animation_index, looping) =
                role_preview_animation_request(&state.presentation, &animation_state)?;
            let visible_nodes = role_preview_visible_nodes(
                &draft.value,
                state.role_preview_body_type,
                state.role_preview_carrying,
                state.role_preview_eye.as_deref(),
                state.role_preview_hair.as_deref(),
                state.role_preview_facial_hair.as_deref(),
            );
            let fingerprint = debug_fingerprint(&(
                &draft.value,
                &animation_state,
                &visible_nodes,
                state.role_preview_body_type,
                state.role_preview_carrying,
            ));
            Some((
                PreviewRequest::Role {
                    role: draft.id.clone(),
                    asset_path,
                    animation_index,
                    looping,
                    visible_nodes,
                    fingerprint,
                },
                format!("{} · composed character", draft.value.display_name),
            ))
        }),
        ToolTab::World
            if matches!(
                state.world_preview_layer,
                WorldPreviewLayer::LandResources | WorldPreviewLayer::WaterResources
            ) =>
        {
            state.resource_generation_draft.as_ref().map(|layer| {
                (
                    PreviewRequest::Model(layer.visual_asset_path.clone()),
                    format!("Resource · {}", layer.display_name),
                )
            })
        }
        ToolTab::World if state.world_preview_layer == WorldPreviewLayer::EnemyCamps => state
            .enemy_camp_generation_draft
            .as_ref()
            .and_then(|layer| state.catalog.archetypes.get(&layer.camp_archetype))
            .and_then(|archetype| {
                archetype
                    .scenes
                    .iter()
                    .find(|scene| scene.is_default)
                    .or_else(|| archetype.scenes.first())
                    .map(|scene| {
                        (
                            PreviewRequest::Model(scene.asset_path.clone()),
                            format!("Enemy camp · {}", archetype.display_name),
                        )
                    })
            }),
        ToolTab::World => state
            .foliage_draft
            .as_ref()
            .and_then(|layer| layer.variants.get(state.selected_foliage_variant))
            .map(|variant| {
                (
                    PreviewRequest::Model(variant.asset_path.clone()),
                    format!("Foliage · {}", variant.asset_path),
                )
            }),
        _ => None,
    };
    if let Some((request, label)) = requested {
        state.preview_request = Some(request);
        state.preview_label = label;
    } else if matches!(
        state.tab,
        ToolTab::Assets | ToolTab::Buildings | ToolTab::Roles | ToolTab::World
    ) {
        state.preview_request = None;
        "Nothing previewable is assigned".clone_into(&mut state.preview_label);
    }
}

fn debug_fingerprint(value: &impl std::fmt::Debug) -> u64 {
    let mut hasher = DefaultHasher::new();
    format!("{value:?}").hash(&mut hasher);
    hasher.finish()
}

fn draw_model_preview(
    ui: &mut egui::Ui,
    texture: Option<egui::TextureId>,
    status: &str,
    desired: egui::Vec2,
    controls: &mut ModelPreviewControls,
    animation_controls: bool,
) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.strong("Live 3D preview");
            if ui.small_button("Reset view").clicked() {
                controls.reset_view();
            }
            ui.small("drag: orbit · Shift/right-drag: pan · wheel: zoom");
        });
        let response = if let Some(texture) = texture {
            ui.add(
                egui::Image::new(egui::load::SizedTexture::new(texture, desired))
                    .sense(egui::Sense::click_and_drag()),
            )
        } else {
            let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click_and_drag());
            ui.painter()
                .rect_filled(rect, 5.0, egui::Color32::from_rgb(9, 14, 19));
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Preview render target unavailable",
                egui::FontId::proportional(13.0),
                egui::Color32::LIGHT_RED,
            );
            response
        };
        if response.double_clicked() {
            controls.reset_view();
        }
        if response.hovered() {
            let (delta, scroll, shift) = ui.input(|input| {
                (
                    input.pointer.delta(),
                    input.smooth_scroll_delta.y,
                    input.modifiers.shift,
                )
            });
            if response.dragged_by(egui::PointerButton::Primary) && !shift {
                controls.yaw = (controls.yaw - delta.x * 0.012).rem_euclid(std::f32::consts::TAU);
                controls.pitch = (controls.pitch + delta.y * 0.01).clamp(-1.35, 1.35);
            }
            if response.dragged_by(egui::PointerButton::Secondary)
                || (response.dragged_by(egui::PointerButton::Primary) && shift)
            {
                let pan_scale = controls.distance * 0.0028;
                controls.pan +=
                    preview_pan_delta(controls.yaw, controls.pitch, delta.x, delta.y, pan_scale);
            }
            if scroll.abs() > f32::EPSILON {
                controls.distance =
                    (controls.distance * (-scroll * 0.0015).exp()).clamp(2.25, 30.0);
            }
        }
        if animation_controls {
            ui.horizontal_wrapped(|ui| {
                if ui
                    .button(if controls.animation_playing {
                        "Pause"
                    } else {
                        "Play"
                    })
                    .clicked()
                {
                    controls.animation_playing = !controls.animation_playing;
                }
                if ui.button("Restart").clicked() {
                    controls.animation_playing = true;
                    controls.restart_animation = true;
                }
                ui.checkbox(&mut controls.animation_looping, "Loop");
                ui.add(
                    egui::Slider::new(&mut controls.animation_speed, 0.05..=3.0)
                        .logarithmic(true)
                        .text("Speed"),
                );
            });
        }
        ui.small(status);
    });
}

fn discover_model_assets() -> Vec<String> {
    fn visit(root: &std::path::Path, current: &std::path::Path, output: &mut Vec<String>) {
        let Ok(entries) = fs::read_dir(current) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, output);
            } else if path
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
                && let Ok(relative) = path.strip_prefix(root)
            {
                output.push(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    let mut output = Vec::new();
    visit(&root, &root, &mut output);
    output.sort();
    output.dedup();
    output
}

fn discover_texture_assets() -> Vec<String> {
    fn visit(root: &std::path::Path, current: &std::path::Path, output: &mut Vec<String>) {
        let Ok(entries) = fs::read_dir(current) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, output);
            } else if path.extension().is_some_and(|extension| {
                matches!(
                    extension.to_string_lossy().to_ascii_lowercase().as_str(),
                    "png" | "tga" | "jpg" | "jpeg"
                )
            }) && let Ok(relative) = path.strip_prefix(root)
            {
                let relative = relative.to_string_lossy().replace('\\', "/");
                if relative.starts_with("migrated/textures/") {
                    output.push(relative);
                }
            }
        }
    }

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    let mut output = Vec::new();
    visit(&root, &root, &mut output);
    output.sort();
    output.dedup();
    output
}

fn inspect_gltf_asset(asset_path: &str) -> Result<GltfMetadata, String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join(asset_path);
    let document = gltf::Gltf::open(&path)
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
    let mut nodes = document
        .nodes()
        .filter_map(|node| node.name().map(ToOwned::to_owned))
        .collect::<Vec<_>>();
    let mut materials = document
        .materials()
        .enumerate()
        .map(|(index, material)| {
            material
                .name()
                .map_or_else(|| format!("Material {index}"), ToOwned::to_owned)
        })
        .collect::<Vec<_>>();
    let animations = document
        .animations()
        .enumerate()
        .map(|(index, animation)| {
            (
                index,
                animation
                    .name()
                    .map_or_else(|| format!("Animation {index}"), ToOwned::to_owned),
            )
        })
        .collect::<Vec<_>>();
    nodes.sort();
    nodes.dedup();
    materials.sort();
    materials.dedup();
    Ok(GltfMetadata {
        nodes,
        materials,
        animations,
    })
}

fn cached_gltf_metadata(state: &mut ToolState, asset_path: &str) -> GltfMetadata {
    if let Some(value) = state.gltf_metadata.get(asset_path) {
        return value.clone();
    }
    match inspect_gltf_asset(asset_path) {
        Ok(value) => {
            state
                .gltf_metadata
                .insert(asset_path.to_owned(), value.clone());
            value
        }
        Err(error) => {
            state.status = error;
            GltfMetadata::default()
        }
    }
}

fn import_model_asset(state: &mut ToolState) -> Result<String, String> {
    let source_text = state.model_import_source.trim();
    if source_text.is_empty() {
        return Err("choose a source .glb file".to_owned());
    }
    let source = fs::canonicalize(source_text)
        .map_err(|error| format!("could not read {source_text}: {error}"))?;
    if !source.is_file()
        || !source
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
    {
        return Err("the source must be an existing .glb file".to_owned());
    }
    let requested_name = state.model_import_name.trim();
    let file_name = if requested_name.is_empty() {
        source
            .file_name()
            .ok_or_else(|| "the source has no filename".to_owned())?
            .to_owned()
    } else {
        let candidate = std::path::Path::new(requested_name);
        if candidate.file_name() != Some(candidate.as_os_str()) {
            return Err("destination name must be a filename, not a path".to_owned());
        }
        candidate.as_os_str().to_owned()
    };
    if !std::path::Path::new(&file_name)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
    {
        return Err("destination name must end in .glb".to_owned());
    }

    let relative = PathBuf::from("user/models").join(&file_name);
    let destination = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join(&relative);
    if destination.exists() {
        return Err(format!(
            "{} already exists; choose another destination name",
            destination.display()
        ));
    }
    let parent = destination
        .parent()
        .ok_or_else(|| "destination has no parent directory".to_owned())?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    fs::copy(&source, &destination)
        .map_err(|error| format!("could not copy model into the project: {error}"))?;

    state.discovered_model_assets = discover_model_assets();
    let relative = relative.to_string_lossy().replace('\\', "/");
    relative.clone_into(&mut state.new_archetype_asset);
    state.model_import_source.clear();
    state.model_import_name.clear();
    Ok(relative)
}

fn import_texture_asset(state: &mut ToolState) -> Result<String, String> {
    let source_text = state.texture_import_source.trim();
    if source_text.is_empty() {
        return Err("choose a source PNG, TGA, JPG, or JPEG file".to_owned());
    }
    let source = fs::canonicalize(source_text)
        .map_err(|error| format!("could not read {source_text}: {error}"))?;
    let extension = source
        .extension()
        .map(|value| value.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    if !source.is_file() || !matches!(extension.as_str(), "png" | "tga" | "jpg" | "jpeg") {
        return Err("the source must be a supported image file".to_owned());
    }
    let requested_name = state.texture_import_name.trim();
    let file_name = if requested_name.is_empty() {
        source
            .file_name()
            .ok_or_else(|| "the source has no filename".to_owned())?
            .to_owned()
    } else {
        let candidate = std::path::Path::new(requested_name);
        if candidate.file_name() != Some(candidate.as_os_str()) {
            return Err("destination name must be a filename, not a path".to_owned());
        }
        candidate.as_os_str().to_owned()
    };
    let output_extension = std::path::Path::new(&file_name)
        .extension()
        .map(|value| value.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    if !matches!(output_extension.as_str(), "png" | "tga" | "jpg" | "jpeg") {
        return Err("destination name must retain a supported image extension".to_owned());
    }
    let relative = PathBuf::from("migrated/textures/user").join(&file_name);
    let destination = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join(&relative);
    if destination.exists() {
        return Err(format!(
            "{} already exists; choose another destination name",
            destination.display()
        ));
    }
    let parent = destination
        .parent()
        .ok_or_else(|| "destination has no parent directory".to_owned())?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    fs::copy(&source, &destination)
        .map_err(|error| format!("could not copy texture into the project: {error}"))?;
    state.discovered_texture_assets = discover_texture_assets();
    let relative = relative.to_string_lossy().replace('\\', "/");
    relative.clone_into(&mut state.new_texture_asset);
    state.texture_import_source.clear();
    state.texture_import_name.clear();
    Ok(relative)
}

fn texture_assets_editor(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    let choices = state
        .presentation
        .textures
        .iter()
        .map(|(id, value)| (id.clone(), value.display_name.clone()))
        .collect::<Vec<_>>();
    ui.horizontal_wrapped(|ui| {
        stable_id_option_choice(ui, "Texture", &mut state.selected_texture, &choices);
        if ui.button("Refresh texture files").clicked() {
            state.discovered_texture_assets = discover_texture_assets();
            state.status = format!(
                "Discovered {} supported textures",
                state.discovered_texture_assets.len()
            );
        }
        if ui
            .add_enabled(
                state.selected_texture.is_some(),
                egui::Button::new("Delete"),
            )
            .clicked()
        {
            state.status = match delete_selected_texture(state) {
                Ok(()) => "Deleted unreferenced texture definition".to_owned(),
                Err(error) => format!("Texture deletion rejected: {error}"),
            };
        }
    });
    draw_model_preview(
        ui,
        preview_texture,
        preview_status,
        egui::vec2(500.0, 333.0),
        preview_controls,
        false,
    );
    ui.collapsing("Import texture file", |ui| {
        ui.label("Copies a supported image into assets/migrated/textures/user without overwriting existing files.");
        ui.horizontal_wrapped(|ui| {
            ui.label("Source file");
            ui.add(
                egui::TextEdit::singleline(&mut state.texture_import_source)
                    .desired_width(400.0)
                    .hint_text("C:\\path\\to\\texture.png"),
            );
            ui.label("Destination name");
            ui.add(
                egui::TextEdit::singleline(&mut state.texture_import_name)
                    .desired_width(220.0)
                    .hint_text("optional_name.png"),
            );
            if ui.button("Import").clicked() {
                state.status = match import_texture_asset(state) {
                    Ok(path) => format!("Imported texture as {path}"),
                    Err(error) => format!("Texture import rejected: {error}"),
                };
            }
        });
    });
    ui.collapsing("Add texture definition", |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label("Stable ID");
            ui.text_edit_singleline(&mut state.new_texture_id);
            ui.label("Name");
            ui.text_edit_singleline(&mut state.new_texture_name);
        });
        let assets = state.discovered_texture_assets.clone();
        searchable_string_choice(
            ui,
            "new_texture_asset",
            "Texture file",
            &mut state.new_texture_asset,
            &assets,
            &mut state.asset_search,
        );
        if ui.button("Add texture").clicked() {
            state.status = match create_texture_definition(state) {
                Ok(()) => "Added texture definition and selected it".to_owned(),
                Err(error) => format!("Texture creation rejected: {error}"),
            };
        }
    });
    if let Some(id) = state.selected_texture.clone()
        && let Some(texture) = state.presentation.textures.get_mut(&id)
    {
        ui.separator();
        ui.heading(&texture.display_name);
        ui.monospace(id.to_string());
        ui.horizontal_wrapped(|ui| {
            ui.label("Display name");
            ui.text_edit_singleline(&mut texture.display_name);
            ui.label("Source GUID");
            ui.text_edit_singleline(&mut texture.source_guid);
        });
        ui.horizontal_wrapped(|ui| {
            ui.label("Source path");
            ui.text_edit_singleline(&mut texture.source_path);
        });
        let assets = state.discovered_texture_assets.clone();
        searchable_string_choice(
            ui,
            "selected_texture_asset",
            "Asset file",
            &mut texture.asset_path,
            &assets,
            &mut state.asset_search,
        );
        let mut nine_slice = texture.sprite_border.is_some();
        if ui
            .checkbox(&mut nine_slice, "Nine-slice sprite border")
            .changed()
        {
            texture.sprite_border = nine_slice.then_some([8.0; 4]);
        }
        if let Some(border) = texture.sprite_border.as_mut() {
            ui.horizontal_wrapped(|ui| {
                for (value, label) in border.iter_mut().zip(["Left", "Right", "Top", "Bottom"]) {
                    ui.add(
                        egui::DragValue::new(value)
                            .range(0.0..=8_192.0)
                            .prefix(format!("{label} ")),
                    );
                }
            });
        }
    }
}

fn create_texture_definition(state: &mut ToolState) -> Result<(), String> {
    let id = StableId::new(state.new_texture_id.trim()).map_err(|error| error.to_string())?;
    if state.presentation.textures.contains_key(&id) {
        return Err(format!("texture {id} already exists"));
    }
    let mut candidate = state.presentation.clone();
    candidate.textures.insert(
        id.clone(),
        TextureDef {
            display_name: state.new_texture_name.trim().to_owned(),
            source_guid: "authored-in-bevy".to_owned(),
            source_path: "authoring://stream-town-tools".to_owned(),
            asset_path: state.new_texture_asset.clone(),
            sprite_border: None,
        },
    );
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_texture = Some(id);
    "texture:new".clone_into(&mut state.new_texture_id);
    "New Texture".clone_into(&mut state.new_texture_name);
    Ok(())
}

fn delete_selected_texture(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_texture
        .clone()
        .ok_or_else(|| "no texture selected".to_owned())?;
    let mut candidate = state.presentation.clone();
    candidate
        .textures
        .remove(&id)
        .ok_or_else(|| format!("missing texture {id}"))?;
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_texture = state.presentation.textures.keys().next().cloned();
    Ok(())
}

fn material_assets_editor(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    let material_choices = state
        .presentation
        .materials
        .iter()
        .map(|(id, value)| (id.clone(), value.display_name.clone()))
        .collect::<Vec<_>>();
    let texture_choices = state
        .presentation
        .textures
        .iter()
        .map(|(id, value)| (id.clone(), value.display_name.clone()))
        .collect::<Vec<_>>();
    ui.horizontal_wrapped(|ui| {
        stable_id_option_choice(
            ui,
            "Material",
            &mut state.selected_material,
            &material_choices,
        );
        ui.label("New stable ID");
        ui.text_edit_singleline(&mut state.new_material_id);
        ui.label("Name");
        ui.text_edit_singleline(&mut state.new_material_name);
        if ui.button("Add material").clicked() {
            state.status = match create_material_definition(state) {
                Ok(()) => "Added material and selected it".to_owned(),
                Err(error) => format!("Material creation rejected: {error}"),
            };
        }
        if ui
            .add_enabled(
                state.selected_material.is_some(),
                egui::Button::new("Delete"),
            )
            .clicked()
        {
            state.status = match delete_selected_material(state) {
                Ok(()) => "Deleted unreferenced material".to_owned(),
                Err(error) => format!("Material deletion rejected: {error}"),
            };
        }
    });
    draw_model_preview(
        ui,
        preview_texture,
        preview_status,
        egui::vec2(500.0, 333.0),
        preview_controls,
        false,
    );
    let Some(id) = state.selected_material.clone() else {
        return;
    };
    let Some(material) = state.presentation.materials.get_mut(&id) else {
        return;
    };
    ui.separator();
    ui.heading(&material.display_name);
    ui.monospace(id.to_string());
    ui.horizontal_wrapped(|ui| {
        ui.label("Display name");
        ui.text_edit_singleline(&mut material.display_name);
        ui.label("Source GUID");
        ui.text_edit_singleline(&mut material.source_guid);
    });
    ui.horizontal_wrapped(|ui| {
        ui.label("Source path");
        ui.text_edit_singleline(&mut material.source_path);
        ui.label("Shader source");
        optional_string_editor(ui, &mut material.shader_source, "Assets/Shaders/...");
    });
    ui.horizontal_wrapped(|ui| {
        ui.label("Base colour");
        ui.color_edit_button_rgba_unmultiplied(&mut material.base_color);
        ui.label("Emissive");
        ui.color_edit_button_rgba_unmultiplied(&mut material.emissive);
        ui.add(egui::Slider::new(&mut material.metallic, 0.0..=1.0).text("Metallic"));
        ui.add(egui::Slider::new(&mut material.perceptual_roughness, 0.0..=1.0).text("Roughness"));
        material_alpha_choice(ui, &mut material.alpha_mode);
    });
    ui.collapsing("Texture slots and UV transforms", |ui| {
        let slots = material.textures.keys().cloned().collect::<Vec<_>>();
        let mut remove = None;
        for slot in slots {
            ui.push_id(("material_texture", &slot), |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.monospace(&slot);
                    if let Some(texture) = material.textures.get_mut(&slot) {
                        stable_id_required_choice(ui, "Texture", texture, &texture_choices);
                    }
                    if ui.small_button("Remove").clicked() {
                        remove = Some(slot.clone());
                    }
                });
                let mut transformed = material.texture_transforms.contains_key(&slot);
                if ui
                    .checkbox(&mut transformed, "Custom tiling/offset")
                    .changed()
                {
                    if transformed {
                        material
                            .texture_transforms
                            .insert(slot.clone(), TextureTransform::default());
                    } else {
                        material.texture_transforms.remove(&slot);
                    }
                }
                if let Some(transform) = material.texture_transforms.get_mut(&slot) {
                    ui.horizontal_wrapped(|ui| {
                        ui.add(egui::DragValue::new(&mut transform.scale[0]).prefix("Tile X "));
                        ui.add(egui::DragValue::new(&mut transform.scale[1]).prefix("Y "));
                        ui.add(egui::DragValue::new(&mut transform.offset[0]).prefix("Offset X "));
                        ui.add(egui::DragValue::new(&mut transform.offset[1]).prefix("Y "));
                    });
                }
            });
        }
        if let Some(slot) = remove {
            material.textures.remove(&slot);
            material.texture_transforms.remove(&slot);
        }
        ui.horizontal_wrapped(|ui| {
            ui.label("New slot");
            ui.text_edit_singleline(&mut state.new_material_texture_slot);
            if ui
                .add_enabled(
                    !texture_choices.is_empty(),
                    egui::Button::new("Add texture slot"),
                )
                .clicked()
                && let Some((texture, _)) = texture_choices.first()
            {
                material
                    .textures
                    .entry(state.new_material_texture_slot.trim().to_owned())
                    .or_insert_with(|| texture.clone());
            }
        });
    });
    ui.collapsing("Custom shader scalar properties", |ui| {
        string_f32_map_editor(
            ui,
            &mut material.custom_properties,
            &mut state.new_material_property,
        );
    });
    ui.collapsing("Custom shader vector/colour properties", |ui| {
        string_vec4_map_editor(
            ui,
            &mut material.custom_vectors,
            &mut state.new_material_vector,
        );
    });
}

fn material_alpha_choice(ui: &mut egui::Ui, value: &mut MaterialAlphaMode) {
    egui::ComboBox::from_id_salt(("material_alpha", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(format!("{value:?}"))
        .show_ui(ui, |ui| {
            ui.selectable_value(value, MaterialAlphaMode::Opaque, "Opaque");
            ui.selectable_value(value, MaterialAlphaMode::Mask, "Alpha mask");
            ui.selectable_value(value, MaterialAlphaMode::Blend, "Transparent blend");
        });
}

fn create_material_definition(state: &mut ToolState) -> Result<(), String> {
    let id = StableId::new(state.new_material_id.trim()).map_err(|error| error.to_string())?;
    if state.presentation.materials.contains_key(&id) {
        return Err(format!("material {id} already exists"));
    }
    let mut candidate = state.presentation.clone();
    candidate.materials.insert(
        id.clone(),
        MaterialDef {
            display_name: state.new_material_name.trim().to_owned(),
            source_guid: "authored-in-bevy".to_owned(),
            source_path: "authoring://stream-town-tools".to_owned(),
            shader_source: None,
            base_color: [1.0; 4],
            emissive: [0.0, 0.0, 0.0, 1.0],
            metallic: 0.0,
            perceptual_roughness: 0.75,
            alpha_mode: MaterialAlphaMode::Opaque,
            textures: BTreeMap::new(),
            texture_transforms: BTreeMap::new(),
            custom_properties: BTreeMap::new(),
            custom_vectors: BTreeMap::new(),
        },
    );
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_material = Some(id);
    "material:new".clone_into(&mut state.new_material_id);
    "New Material".clone_into(&mut state.new_material_name);
    Ok(())
}

fn delete_selected_material(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_material
        .clone()
        .ok_or_else(|| "no material selected".to_owned())?;
    let mut candidate = state.presentation.clone();
    candidate
        .materials
        .remove(&id)
        .ok_or_else(|| format!("missing material {id}"))?;
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_material = state.presentation.materials.keys().next().cloned();
    Ok(())
}

fn string_f32_map_editor(
    ui: &mut egui::Ui,
    values: &mut BTreeMap<String, f32>,
    new_key: &mut String,
) {
    let keys = values.keys().cloned().collect::<Vec<_>>();
    let mut remove = None;
    for key in keys {
        ui.horizontal_wrapped(|ui| {
            ui.monospace(&key);
            if let Some(value) = values.get_mut(&key) {
                ui.add(egui::DragValue::new(value).speed(0.01));
            }
            if ui.small_button("Remove").clicked() {
                remove = Some(key.clone());
            }
        });
    }
    if let Some(key) = remove {
        values.remove(&key);
    }
    ui.horizontal_wrapped(|ui| {
        ui.text_edit_singleline(new_key);
        if ui.button("Add scalar").clicked() && !new_key.trim().is_empty() {
            values.entry(new_key.trim().to_owned()).or_insert(0.0);
        }
    });
}

fn string_vec4_map_editor(
    ui: &mut egui::Ui,
    values: &mut BTreeMap<String, [f32; 4]>,
    new_key: &mut String,
) {
    let keys = values.keys().cloned().collect::<Vec<_>>();
    let mut remove = None;
    for key in keys {
        ui.group(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.monospace(&key);
                if let Some(value) = values.get_mut(&key) {
                    ui.color_edit_button_rgba_unmultiplied(value);
                    for component in value {
                        ui.add(egui::DragValue::new(component).speed(0.01));
                    }
                }
                if ui.small_button("Remove").clicked() {
                    remove = Some(key.clone());
                }
            });
        });
    }
    if let Some(key) = remove {
        values.remove(&key);
    }
    ui.horizontal_wrapped(|ui| {
        ui.text_edit_singleline(new_key);
        if ui.button("Add vector").clicked() && !new_key.trim().is_empty() {
            values.entry(new_key.trim().to_owned()).or_insert([1.0; 4]);
        }
    });
}

fn optional_string_editor(ui: &mut egui::Ui, value: &mut Option<String>, hint: &str) {
    let mut enabled = value.is_some();
    if ui.checkbox(&mut enabled, "Set").changed() {
        *value = enabled.then(String::new);
    }
    if let Some(value) = value {
        ui.add(
            egui::TextEdit::singleline(value)
                .desired_width(280.0)
                .hint_text(hint),
        );
    }
}

fn animation_assets_editor(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    let clip_choices = state
        .presentation
        .clips
        .iter()
        .map(|(id, value)| (id.clone(), value.display_name.clone()))
        .collect::<Vec<_>>();
    let animation_assets = state
        .discovered_model_assets
        .iter()
        .filter(|path| path.starts_with("migrated/models/"))
        .cloned()
        .collect::<Vec<_>>();
    ui.horizontal_wrapped(|ui| {
        stable_id_option_choice(
            ui,
            "Animation clip",
            &mut state.selected_clip,
            &clip_choices,
        );
        ui.label("New stable ID");
        ui.text_edit_singleline(&mut state.new_clip_id);
        ui.label("Name");
        ui.text_edit_singleline(&mut state.new_clip_name);
    });
    searchable_string_choice(
        ui,
        "new_clip_asset",
        "Converted rig/animation GLB",
        &mut state.new_clip_asset,
        &animation_assets,
        &mut state.asset_search,
    );
    ui.horizontal_wrapped(|ui| {
        if ui.button("Add animation clip").clicked() {
            state.status = match create_animation_clip(state) {
                Ok(()) => "Added animation clip and selected it".to_owned(),
                Err(error) => format!("Animation creation rejected: {error}"),
            };
        }
        if ui
            .add_enabled(
                state.selected_clip.is_some(),
                egui::Button::new("Delete clip"),
            )
            .clicked()
        {
            state.status = match delete_selected_animation_clip(state) {
                Ok(()) => "Deleted unreferenced animation clip".to_owned(),
                Err(error) => format!("Animation deletion rejected: {error}"),
            };
        }
    });
    draw_model_preview(
        ui,
        preview_texture,
        preview_status,
        egui::vec2(560.0, 373.0),
        preview_controls,
        true,
    );
    let Some(id) = state.selected_clip.clone() else {
        return;
    };
    let selected_clip_asset = state
        .presentation
        .clips
        .get(&id)
        .and_then(|clip| {
            clip.converted_asset_path
                .as_ref()
                .or(clip.rig_asset_path.as_ref())
        })
        .cloned();
    let clip_metadata = selected_clip_asset
        .as_deref()
        .map(|path| cached_gltf_metadata(state, path))
        .unwrap_or_default();
    let Some(clip) = state.presentation.clips.get_mut(&id) else {
        return;
    };
    ui.separator();
    ui.heading(&clip.display_name);
    ui.monospace(id.to_string());
    ui.horizontal_wrapped(|ui| {
        ui.label("Display name");
        ui.text_edit_singleline(&mut clip.display_name);
        ui.label("Source GUID");
        ui.text_edit_singleline(&mut clip.source_guid);
        ui.checkbox(&mut clip.looping, "Authored looping");
    });
    ui.horizontal_wrapped(|ui| {
        ui.label("Source path");
        ui.text_edit_singleline(&mut clip.source_path);
        ui.add(
            egui::DragValue::new(&mut clip.duration_seconds)
                .range(0.001..=86_400.0)
                .suffix(" seconds"),
        );
        ui.add(
            egui::DragValue::new(&mut clip.sample_rate)
                .range(0.001..=10_000.0)
                .suffix(" Hz"),
        );
    });
    optional_asset_choice(
        ui,
        "clip_rig_asset",
        "Rig GLB",
        &mut clip.rig_asset_path,
        &animation_assets,
        &mut state.asset_search,
    );
    optional_asset_choice(
        ui,
        "clip_converted_asset",
        "Converted animation GLB",
        &mut clip.converted_asset_path,
        &animation_assets,
        &mut state.asset_search,
    );
    let mut has_index = clip.gltf_animation_index.is_some();
    ui.horizontal_wrapped(|ui| {
        if ui
            .checkbox(&mut has_index, "glTF animation index")
            .changed()
        {
            clip.gltf_animation_index = has_index.then_some(0);
        }
        if let Some(index) = clip.gltf_animation_index.as_mut() {
            if clip_metadata.animations.is_empty() {
                ui.add(egui::DragValue::new(index));
            } else {
                egui::ComboBox::from_id_salt("gltf_animation_index")
                    .height(AUTHORING_COMBO_MAX_HEIGHT)
                    .selected_text(
                        clip_metadata
                            .animations
                            .iter()
                            .find(|(candidate, _)| u32::try_from(*candidate).ok() == Some(*index))
                            .map_or_else(|| format!("Animation {index}"), |(_, name)| name.clone()),
                    )
                    .show_ui(ui, |ui| {
                        for (candidate, name) in &clip_metadata.animations {
                            if let Ok(candidate) = u32::try_from(*candidate) {
                                ui.selectable_value(
                                    index,
                                    candidate,
                                    format!("{candidate}: {name}"),
                                );
                            }
                        }
                    });
            }
        }
    });
    animation_transform_tracks_editor(ui, &mut clip.transform_tracks, clip.duration_seconds);
    animation_property_curves_editor(ui, &mut clip.property_curves, clip.duration_seconds);
    animation_events_editor(ui, &mut clip.events, clip.duration_seconds);
    ui.separator();
    animation_controller_editor(ui, state, &clip_choices);
}

fn optional_asset_choice(
    ui: &mut egui::Ui,
    salt: &str,
    label: &str,
    value: &mut Option<String>,
    choices: &[String],
    search: &mut String,
) {
    let mut enabled = value.is_some();
    ui.horizontal_wrapped(|ui| {
        if ui.checkbox(&mut enabled, label).changed() {
            *value = enabled.then(|| choices.first().cloned().unwrap_or_default());
        }
    });
    if let Some(value) = value {
        searchable_string_choice(ui, salt, label, value, choices, search);
    }
}

fn create_animation_clip(state: &mut ToolState) -> Result<(), String> {
    let id = StableId::new(state.new_clip_id.trim()).map_err(|error| error.to_string())?;
    if state.presentation.clips.contains_key(&id) {
        return Err(format!("animation clip {id} already exists"));
    }
    if !state.new_clip_asset.starts_with("migrated/models/") {
        return Err("animation GLBs must live below migrated/models".to_owned());
    }
    let mut candidate = state.presentation.clone();
    candidate.clips.insert(
        id.clone(),
        AnimationClipDef {
            display_name: state.new_clip_name.trim().to_owned(),
            source_guid: "authored-in-bevy".to_owned(),
            source_path: "authoring://stream-town-tools".to_owned(),
            duration_seconds: 1.0,
            sample_rate: 30.0,
            looping: true,
            rig_asset_path: Some(state.new_clip_asset.clone()),
            transform_tracks: Vec::new(),
            property_curves: Vec::new(),
            events: Vec::new(),
            converted_asset_path: Some(state.new_clip_asset.clone()),
            gltf_animation_index: Some(0),
        },
    );
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_clip = Some(id);
    "clip:new".clone_into(&mut state.new_clip_id);
    "New Animation".clone_into(&mut state.new_clip_name);
    Ok(())
}

fn delete_selected_animation_clip(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_clip
        .clone()
        .ok_or_else(|| "no animation clip selected".to_owned())?;
    let mut candidate = state.presentation.clone();
    candidate
        .clips
        .remove(&id)
        .ok_or_else(|| format!("missing animation clip {id}"))?;
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_clip = state.presentation.clips.keys().next().cloned();
    Ok(())
}

fn animation_controller_editor(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    clip_choices: &[(StableId, String)],
) {
    ui.heading("Animation controllers");
    ui.label(
        "Controller parameters, states, blend motions, transitions, state machines, and layers are authored here rather than displayed as migration diagnostics.",
    );
    let controller_choices = state
        .presentation
        .controllers
        .iter()
        .map(|(id, value)| (id.clone(), value.display_name.clone()))
        .collect::<Vec<_>>();
    ui.horizontal_wrapped(|ui| {
        stable_id_option_choice(
            ui,
            "Controller",
            &mut state.selected_controller,
            &controller_choices,
        );
        ui.label("New stable ID");
        ui.text_edit_singleline(&mut state.new_controller_id);
        ui.label("Name");
        ui.text_edit_singleline(&mut state.new_controller_name);
        if ui.button("Add controller").clicked() {
            state.status = match create_animation_controller(state) {
                Ok(()) => "Added animation controller and selected it".to_owned(),
                Err(error) => format!("Controller creation rejected: {error}"),
            };
        }
        if ui
            .add_enabled(
                state.selected_controller.is_some(),
                egui::Button::new("Delete controller"),
            )
            .clicked()
        {
            state.status = match delete_selected_animation_controller(state) {
                Ok(()) => "Deleted unreferenced animation controller".to_owned(),
                Err(error) => format!("Controller deletion rejected: {error}"),
            };
        }
    });
    let Some(id) = state.selected_controller.clone() else {
        return;
    };
    let avatar_mask_choices = state
        .presentation
        .avatar_masks
        .iter()
        .map(|(id, value)| (id.clone(), value.display_name.clone()))
        .collect::<Vec<_>>();
    let Some(controller) = state.presentation.controllers.get_mut(&id) else {
        return;
    };
    ui.group(|ui| {
        ui.heading(&controller.display_name);
        ui.monospace(id.to_string());
        ui.horizontal_wrapped(|ui| {
            ui.label("Display name");
            ui.text_edit_singleline(&mut controller.display_name);
            ui.label("Source GUID");
            ui.text_edit_singleline(&mut controller.source_guid);
            ui.label("Source path");
            ui.text_edit_singleline(&mut controller.source_path);
        });
    });
    animation_parameters_editor(ui, controller, &mut state.new_animation_parameter);
    animation_states_editor(
        ui,
        controller,
        clip_choices,
        &mut state.new_controller_state_id,
    );
    animation_transitions_editor(ui, controller);
    animation_state_machines_editor(ui, controller, &mut state.new_controller_machine_id);
    animation_layers_editor(ui, controller, &avatar_mask_choices);
}

fn create_animation_controller(state: &mut ToolState) -> Result<(), String> {
    let id = StableId::new(state.new_controller_id.trim()).map_err(|error| error.to_string())?;
    if state.presentation.controllers.contains_key(&id) {
        return Err(format!("animation controller {id} already exists"));
    }
    let mut candidate = state.presentation.clone();
    candidate.controllers.insert(
        id.clone(),
        AnimationControllerDef {
            display_name: state.new_controller_name.trim().to_owned(),
            source_guid: "authored-in-bevy".to_owned(),
            source_path: "authoring://stream-town-tools".to_owned(),
            parameters: Vec::new(),
            states: BTreeMap::new(),
            transitions: Vec::new(),
            state_machines: BTreeMap::new(),
            layers: Vec::new(),
            default_states: Vec::new(),
        },
    );
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_controller = Some(id);
    "animation_controller:new".clone_into(&mut state.new_controller_id);
    "New Controller".clone_into(&mut state.new_controller_name);
    Ok(())
}

fn delete_selected_animation_controller(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_controller
        .clone()
        .ok_or_else(|| "no animation controller selected".to_owned())?;
    let mut candidate = state.presentation.clone();
    candidate
        .controllers
        .remove(&id)
        .ok_or_else(|| format!("missing animation controller {id}"))?;
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_controller = state.presentation.controllers.keys().next().cloned();
    Ok(())
}

fn animation_parameters_editor(
    ui: &mut egui::Ui,
    controller: &mut AnimationControllerDef,
    new_name: &mut String,
) {
    ui.collapsing(
        format!("Parameters ({})", controller.parameters.len()),
        |ui| {
            let mut remove = None;
            for (index, parameter) in controller.parameters.iter_mut().enumerate() {
                ui.push_id(("animation_parameter", index), |ui| {
                    ui.group(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Name");
                            ui.text_edit_singleline(&mut parameter.name);
                            animation_parameter_kind_choice(ui, &mut parameter.kind);
                            ui.checkbox(&mut parameter.inferred, "Inferred");
                            if ui.small_button("Remove").clicked() {
                                remove = Some(index);
                            }
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.add(
                                egui::DragValue::new(&mut parameter.default_float).prefix("Float "),
                            );
                            ui.add(
                                egui::DragValue::new(&mut parameter.default_integer)
                                    .prefix("Integer "),
                            );
                            ui.checkbox(&mut parameter.default_boolean, "Boolean default");
                        });
                    });
                });
            }
            if let Some(index) = remove {
                let removed = controller.parameters.remove(index).name;
                for state in controller.states.values_mut() {
                    if state.speed_parameter.as_deref() == Some(removed.as_str()) {
                        state.speed_parameter = None;
                    }
                    if state.blend_parameter.as_deref() == Some(removed.as_str()) {
                        state.blend_parameter = None;
                    }
                }
                for transition in &mut controller.transitions {
                    transition
                        .conditions
                        .retain(|condition| condition.parameter != removed);
                }
            }
            ui.horizontal_wrapped(|ui| {
                ui.text_edit_singleline(new_name);
                if ui.button("Add parameter").clicked() && !new_name.trim().is_empty() {
                    controller.parameters.push(AnimationParameterDef {
                        name: new_name.trim().to_owned(),
                        kind: AnimationParameterKind::Float,
                        default_float: 0.0,
                        default_integer: 0,
                        default_boolean: false,
                        inferred: false,
                    });
                }
            });
        },
    );
}

fn animation_parameter_kind_choice(ui: &mut egui::Ui, value: &mut AnimationParameterKind) {
    egui::ComboBox::from_id_salt(("animation_parameter_kind", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(format!("{value:?}"))
        .show_ui(ui, |ui| {
            for kind in [
                AnimationParameterKind::Float,
                AnimationParameterKind::Integer,
                AnimationParameterKind::Boolean,
                AnimationParameterKind::Trigger,
            ] {
                ui.selectable_value(value, kind, format!("{kind:?}"));
            }
        });
}

fn animation_states_editor(
    ui: &mut egui::Ui,
    controller: &mut AnimationControllerDef,
    clip_choices: &[(StableId, String)],
    new_state_id: &mut String,
) {
    ui.collapsing(format!("States ({})", controller.states.len()), |ui| {
        let state_ids = controller.states.keys().cloned().collect::<Vec<_>>();
        let parameter_names = controller
            .parameters
            .iter()
            .filter(|parameter| parameter.kind == AnimationParameterKind::Float)
            .map(|parameter| parameter.name.clone())
            .collect::<Vec<_>>();
        let mut remove_state = None;
        for state_id in state_ids {
            let Some(state) = controller.states.get_mut(&state_id) else {
                continue;
            };
            ui.push_id(("animation_state", state_id.as_str()), |ui| {
                ui.collapsing(format!("{} · {state_id}", state.display_name), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut state.display_name);
                        ui.add(egui::DragValue::new(&mut state.speed).prefix("Speed "));
                        if ui.small_button("Remove state").clicked() {
                            remove_state = Some(state_id.clone());
                        }
                    });
                    optional_string_choice(
                        ui,
                        "Speed parameter",
                        &mut state.speed_parameter,
                        &parameter_names,
                    );
                    optional_string_choice(
                        ui,
                        "Blend parameter",
                        &mut state.blend_parameter,
                        &parameter_names,
                    );
                    let mut remove_motion = None;
                    for (index, motion) in state.motions.iter_mut().enumerate() {
                        ui.horizontal_wrapped(|ui| {
                            stable_id_required_choice(ui, "Clip", &mut motion.clip, clip_choices);
                            let mut threshold = motion.threshold.is_some();
                            if ui.checkbox(&mut threshold, "Blend threshold").changed() {
                                motion.threshold = threshold.then_some(0.0);
                            }
                            if let Some(value) = motion.threshold.as_mut() {
                                ui.add(egui::DragValue::new(value));
                            }
                            if ui.small_button("Remove motion").clicked() {
                                remove_motion = Some(index);
                            }
                        });
                    }
                    if let Some(index) = remove_motion {
                        state.motions.remove(index);
                    }
                    if ui
                        .add_enabled(!clip_choices.is_empty(), egui::Button::new("Add motion"))
                        .clicked()
                        && let Some((clip, _)) = clip_choices.first()
                    {
                        state.motions.push(AnimationMotionDef {
                            clip: clip.clone(),
                            threshold: None,
                        });
                    }
                });
            });
        }
        if let Some(state_id) = remove_state {
            controller.states.remove(&state_id);
            controller.default_states.retain(|id| id != &state_id);
            for machine in controller.state_machines.values_mut() {
                machine.states.retain(|id| id != &state_id);
                if machine.default_state.as_ref() == Some(&state_id) {
                    machine.default_state = None;
                }
            }
            controller.transitions.retain(|transition| {
                transition.source.as_ref() != Some(&state_id)
                    && transition.destination.as_ref() != Some(&state_id)
            });
        }
        ui.horizontal_wrapped(|ui| {
            ui.label("New state ID");
            ui.text_edit_singleline(new_state_id);
            if ui.button("Add state").clicked()
                && let Ok(id) = StableId::new(new_state_id.trim())
            {
                controller.states.entry(id).or_insert(AnimationStateDef {
                    display_name: "New State".to_owned(),
                    speed: 1.0,
                    speed_parameter: None,
                    blend_parameter: None,
                    motions: Vec::new(),
                });
            }
        });
        let state_choices = controller
            .states
            .iter()
            .map(|(id, state)| (id.clone(), state.display_name.clone()))
            .collect::<Vec<_>>();
        stable_id_vec_choices(
            ui,
            "Controller default states",
            &mut controller.default_states,
            &state_choices,
        );
    });
}

fn animation_transitions_editor(ui: &mut egui::Ui, controller: &mut AnimationControllerDef) {
    ui.collapsing(
        format!("Transitions ({})", controller.transitions.len()),
        |ui| {
            let state_choices = controller
                .states
                .iter()
                .map(|(id, state)| (id.clone(), state.display_name.clone()))
                .collect::<Vec<_>>();
            let machine_choices = controller
                .state_machines
                .iter()
                .map(|(id, machine)| (id.clone(), machine.display_name.clone()))
                .collect::<Vec<_>>();
            let parameter_names = controller
                .parameters
                .iter()
                .map(|parameter| parameter.name.clone())
                .collect::<Vec<_>>();
            let mut remove_transition = None;
            for (index, transition) in controller.transitions.iter_mut().enumerate() {
                ui.push_id(("animation_transition", index), |ui| {
                    ui.group(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.strong(format!("Transition {}", index + 1));
                            stable_id_option_choice(
                                ui,
                                "Source",
                                &mut transition.source,
                                &state_choices,
                            );
                            stable_id_option_choice(
                                ui,
                                "Destination",
                                &mut transition.destination,
                                &state_choices,
                            );
                            if ui.small_button("Remove").clicked() {
                                remove_transition = Some(index);
                            }
                        });
                        ui.horizontal_wrapped(|ui| {
                            stable_id_option_choice(
                                ui,
                                "Source machine",
                                &mut transition.source_state_machine,
                                &machine_choices,
                            );
                            stable_id_option_choice(
                                ui,
                                "Destination machine",
                                &mut transition.destination_state_machine,
                                &machine_choices,
                            );
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.checkbox(&mut transition.is_entry, "Entry");
                            ui.checkbox(&mut transition.is_any_state, "Any state");
                            ui.checkbox(&mut transition.is_exit, "Exit");
                            ui.checkbox(&mut transition.has_exit_time, "Has exit time");
                            ui.checkbox(&mut transition.fixed_duration, "Duration in seconds");
                            ui.add(
                                egui::DragValue::new(&mut transition.exit_time)
                                    .prefix("Exit time "),
                            );
                            ui.add(
                                egui::DragValue::new(&mut transition.duration)
                                    .range(0.0..=10_000.0)
                                    .prefix("Duration "),
                            );
                            ui.add(
                                egui::DragValue::new(&mut transition.offset)
                                    .range(0.0..=10_000.0)
                                    .prefix("Offset "),
                            );
                        });
                        let mut remove_condition = None;
                        for (condition_index, condition) in
                            transition.conditions.iter_mut().enumerate()
                        {
                            ui.horizontal_wrapped(|ui| {
                                string_choice(
                                    ui,
                                    "Parameter",
                                    &mut condition.parameter,
                                    &parameter_names,
                                );
                                animation_condition_mode_choice(ui, &mut condition.mode);
                                ui.add(
                                    egui::DragValue::new(&mut condition.threshold)
                                        .prefix("Threshold "),
                                );
                                if ui.small_button("Remove condition").clicked() {
                                    remove_condition = Some(condition_index);
                                }
                            });
                        }
                        if let Some(index) = remove_condition {
                            transition.conditions.remove(index);
                        }
                        if ui
                            .add_enabled(
                                !parameter_names.is_empty(),
                                egui::Button::new("Add condition"),
                            )
                            .clicked()
                        {
                            transition.conditions.push(AnimationConditionDef {
                                parameter: parameter_names[0].clone(),
                                mode: AnimationConditionMode::Greater,
                                threshold: 0.0,
                            });
                        }
                    });
                });
            }
            if let Some(index) = remove_transition {
                controller.transitions.remove(index);
            }
            if ui
                .add_enabled(
                    !state_choices.is_empty(),
                    egui::Button::new("Add transition"),
                )
                .clicked()
            {
                let state = state_choices[0].0.clone();
                controller.transitions.push(AnimationTransitionDef {
                    source: Some(state.clone()),
                    destination: Some(state),
                    source_state_machine: None,
                    destination_state_machine: None,
                    is_entry: false,
                    is_any_state: false,
                    is_exit: false,
                    has_exit_time: true,
                    exit_time: 1.0,
                    duration: 0.1,
                    fixed_duration: true,
                    offset: 0.0,
                    conditions: Vec::new(),
                });
            }
        },
    );
}

fn animation_condition_mode_choice(ui: &mut egui::Ui, value: &mut AnimationConditionMode) {
    egui::ComboBox::from_id_salt(("animation_condition_mode", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(format!("{value:?}"))
        .show_ui(ui, |ui| {
            for mode in [
                AnimationConditionMode::If,
                AnimationConditionMode::IfNot,
                AnimationConditionMode::Greater,
                AnimationConditionMode::Less,
                AnimationConditionMode::Equals,
                AnimationConditionMode::NotEqual,
            ] {
                ui.selectable_value(value, mode, format!("{mode:?}"));
            }
        });
}

fn animation_state_machines_editor(
    ui: &mut egui::Ui,
    controller: &mut AnimationControllerDef,
    new_machine_id: &mut String,
) {
    ui.collapsing(
        format!("State machines ({})", controller.state_machines.len()),
        |ui| {
            let state_choices = controller
                .states
                .iter()
                .map(|(id, state)| (id.clone(), state.display_name.clone()))
                .collect::<Vec<_>>();
            let machine_choices = controller
                .state_machines
                .iter()
                .map(|(id, machine)| (id.clone(), machine.display_name.clone()))
                .collect::<Vec<_>>();
            let machine_ids = controller
                .state_machines
                .keys()
                .cloned()
                .collect::<Vec<_>>();
            let mut remove_machine = None;
            for id in machine_ids {
                let Some(machine) = controller.state_machines.get_mut(&id) else {
                    continue;
                };
                ui.push_id(("animation_state_machine", id.as_str()), |ui| {
                    ui.collapsing(format!("{} · {id}", machine.display_name), |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Name");
                            ui.text_edit_singleline(&mut machine.display_name);
                            if ui.small_button("Remove machine").clicked() {
                                remove_machine = Some(id.clone());
                            }
                        });
                        stable_id_vec_choices(ui, "States", &mut machine.states, &state_choices);
                        let child_choices = machine_choices
                            .iter()
                            .filter(|(choice, _)| choice != &id)
                            .cloned()
                            .collect::<Vec<_>>();
                        stable_id_vec_choices(
                            ui,
                            "Child state machines",
                            &mut machine.child_state_machines,
                            &child_choices,
                        );
                        stable_id_option_choice(
                            ui,
                            "Default state",
                            &mut machine.default_state,
                            &state_choices,
                        );
                    });
                });
            }
            if let Some(id) = remove_machine {
                controller.state_machines.remove(&id);
                controller.layers.retain(|layer| layer.state_machine != id);
                for machine in controller.state_machines.values_mut() {
                    machine.child_state_machines.retain(|child| child != &id);
                }
                controller.transitions.retain(|transition| {
                    transition.source_state_machine.as_ref() != Some(&id)
                        && transition.destination_state_machine.as_ref() != Some(&id)
                });
            }
            ui.horizontal_wrapped(|ui| {
                ui.label("New machine ID");
                ui.text_edit_singleline(new_machine_id);
                if ui.button("Add state machine").clicked()
                    && let Ok(id) = StableId::new(new_machine_id.trim())
                {
                    controller
                        .state_machines
                        .entry(id)
                        .or_insert(AnimationStateMachineDef {
                            display_name: "New State Machine".to_owned(),
                            states: Vec::new(),
                            child_state_machines: Vec::new(),
                            default_state: None,
                        });
                }
            });
        },
    );
}

fn animation_layers_editor(
    ui: &mut egui::Ui,
    controller: &mut AnimationControllerDef,
    avatar_masks: &[(StableId, String)],
) {
    ui.collapsing(format!("Layers ({})", controller.layers.len()), |ui| {
        let machine_choices = controller
            .state_machines
            .iter()
            .map(|(id, machine)| (id.clone(), machine.display_name.clone()))
            .collect::<Vec<_>>();
        let mut remove = None;
        for (index, layer) in controller.layers.iter_mut().enumerate() {
            ui.push_id(("animation_layer", index), |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut layer.display_name);
                        stable_id_required_choice(
                            ui,
                            "State machine",
                            &mut layer.state_machine,
                            &machine_choices,
                        );
                        animation_layer_blend_choice(ui, &mut layer.blend_mode);
                        ui.add(
                            egui::Slider::new(&mut layer.default_weight, 0.0..=1.0).text("Weight"),
                        );
                        if ui.small_button("Remove").clicked() {
                            remove = Some(index);
                        }
                    });
                    stable_id_option_choice(
                        ui,
                        "Avatar mask",
                        &mut layer.avatar_mask,
                        avatar_masks,
                    );
                });
            });
        }
        if let Some(index) = remove {
            controller.layers.remove(index);
        }
        if ui
            .add_enabled(
                !machine_choices.is_empty(),
                egui::Button::new("Add animation layer"),
            )
            .clicked()
        {
            controller.layers.push(AnimationLayerDef {
                display_name: "New Layer".to_owned(),
                state_machine: machine_choices[0].0.clone(),
                blend_mode: AnimationLayerBlendMode::Override,
                default_weight: 1.0,
                avatar_mask: None,
            });
        }
    });
}

fn animation_layer_blend_choice(ui: &mut egui::Ui, value: &mut AnimationLayerBlendMode) {
    egui::ComboBox::from_id_salt(("animation_layer_blend", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(format!("{value:?}"))
        .show_ui(ui, |ui| {
            ui.selectable_value(value, AnimationLayerBlendMode::Override, "Override");
            ui.selectable_value(value, AnimationLayerBlendMode::Additive, "Additive");
        });
}

fn animation_transform_tracks_editor(
    ui: &mut egui::Ui,
    tracks: &mut Vec<AnimationTransformTrack>,
    duration: f32,
) {
    ui.collapsing(format!("Transform tracks ({})", tracks.len()), |ui| {
        let mut remove = None;
        for (index, track) in tracks.iter_mut().enumerate() {
            ui.push_id(("animation_track", index), |ui| {
                ui.collapsing(
                    if track.target_path.is_empty() {
                        format!("Track {} · root", index + 1)
                    } else {
                        format!("Track {} · {}", index + 1, track.target_path)
                    },
                    |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Hierarchy path");
                            ui.text_edit_singleline(&mut track.target_path);
                            if ui.small_button("Remove track").clicked() {
                                remove = Some(index);
                            }
                        });
                        animation_vec3_keys_editor(
                            ui,
                            "Translation",
                            &mut track.translation,
                            duration,
                            [0.0; 3],
                        );
                        animation_quat_keys_editor(ui, &mut track.rotation, duration);
                        animation_vec3_keys_editor(
                            ui,
                            "Scale",
                            &mut track.scale,
                            duration,
                            [1.0; 3],
                        );
                        animation_vec3_keys_editor(
                            ui,
                            "Euler degrees",
                            &mut track.euler_degrees,
                            duration,
                            [0.0; 3],
                        );
                    },
                );
            });
        }
        if let Some(index) = remove {
            tracks.remove(index);
        }
        if ui.button("Add transform track").clicked() {
            tracks.push(AnimationTransformTrack {
                target_path: String::new(),
                reference_translation: None,
                reference_rotation: None,
                reference_scale: None,
                translation: Vec::new(),
                rotation: Vec::new(),
                scale: Vec::new(),
                euler_degrees: Vec::new(),
            });
        }
    });
}

fn animation_vec3_keys_editor(
    ui: &mut egui::Ui,
    label: &str,
    keys: &mut Vec<AnimationVec3Keyframe>,
    duration: f32,
    default_value: [f32; 3],
) {
    ui.collapsing(format!("{label} keys ({})", keys.len()), |ui| {
        let mut remove = None;
        for (index, key) in keys.iter_mut().enumerate() {
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    egui::DragValue::new(&mut key.time)
                        .range(0.0..=duration.max(0.001))
                        .prefix("t "),
                );
                for (component, axis) in key.value.iter_mut().zip(["x", "y", "z"]) {
                    ui.add(egui::DragValue::new(component).prefix(format!("{axis} ")));
                }
                if ui.small_button("Remove").clicked() {
                    remove = Some(index);
                }
            });
        }
        if let Some(index) = remove {
            keys.remove(index);
        }
        if ui.button(format!("Add {label} key")).clicked() {
            keys.push(AnimationVec3Keyframe {
                time: duration.max(0.0),
                value: default_value,
            });
        }
    });
}

fn animation_quat_keys_editor(
    ui: &mut egui::Ui,
    keys: &mut Vec<AnimationQuatKeyframe>,
    duration: f32,
) {
    ui.collapsing(format!("Rotation keys ({})", keys.len()), |ui| {
        let mut remove = None;
        for (index, key) in keys.iter_mut().enumerate() {
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    egui::DragValue::new(&mut key.time)
                        .range(0.0..=duration.max(0.001))
                        .prefix("t "),
                );
                for (component, axis) in key.value.iter_mut().zip(["x", "y", "z", "w"]) {
                    ui.add(egui::DragValue::new(component).prefix(format!("{axis} ")));
                }
                if ui.small_button("Remove").clicked() {
                    remove = Some(index);
                }
            });
        }
        if let Some(index) = remove {
            keys.remove(index);
        }
        if ui.button("Add rotation key").clicked() {
            keys.push(AnimationQuatKeyframe {
                time: duration.max(0.0),
                value: [0.0, 0.0, 0.0, 1.0],
            });
        }
    });
}

fn animation_property_curves_editor(
    ui: &mut egui::Ui,
    curves: &mut Vec<AnimationPropertyCurve>,
    duration: f32,
) {
    ui.collapsing(format!("Property curves ({})", curves.len()), |ui| {
        let mut remove_curve = None;
        for (index, curve) in curves.iter_mut().enumerate() {
            ui.push_id(("property_curve", index), |ui| {
                ui.collapsing(
                    format!("{} :: {}", curve.target_path, curve.attribute),
                    |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Target");
                            ui.text_edit_singleline(&mut curve.target_path);
                            ui.label("Attribute");
                            ui.text_edit_singleline(&mut curve.attribute);
                            ui.add(egui::DragValue::new(&mut curve.class_id).prefix("Class "));
                            if ui.small_button("Remove curve").clicked() {
                                remove_curve = Some(index);
                            }
                        });
                        let mut remove_key = None;
                        for (key_index, key) in curve.keys.iter_mut().enumerate() {
                            ui.horizontal_wrapped(|ui| {
                                ui.add(
                                    egui::DragValue::new(&mut key.time)
                                        .range(0.0..=duration.max(0.001))
                                        .prefix("t "),
                                );
                                ui.add(egui::DragValue::new(&mut key.value).prefix("value "));
                                if ui.small_button("Remove").clicked() {
                                    remove_key = Some(key_index);
                                }
                            });
                        }
                        if let Some(key_index) = remove_key {
                            curve.keys.remove(key_index);
                        }
                        if ui.button("Add property key").clicked() {
                            curve.keys.push(default_float_key(duration.max(0.0), 0.0));
                        }
                    },
                );
            });
        }
        if let Some(index) = remove_curve {
            curves.remove(index);
        }
        if ui.button("Add property curve").clicked() {
            curves.push(AnimationPropertyCurve {
                target_path: String::new(),
                attribute: "m_Enabled".to_owned(),
                class_id: 1,
                script_guid: None,
                keys: vec![default_float_key(0.0, 0.0)],
            });
        }
    });
}

fn default_float_key(time: f32, value: f32) -> AnimationFloatKeyframe {
    AnimationFloatKeyframe {
        time,
        value,
        in_slope: AnimationTangent::Finite(0.0),
        out_slope: AnimationTangent::Finite(0.0),
        tangent_mode: 0,
        weighted_mode: 0,
        in_weight: 1.0 / 3.0,
        out_weight: 1.0 / 3.0,
    }
}

fn animation_events_editor(ui: &mut egui::Ui, events: &mut Vec<AnimationEventDef>, duration: f32) {
    ui.collapsing(format!("Animation events ({})", events.len()), |ui| {
        let mut remove = None;
        for (index, event) in events.iter_mut().enumerate() {
            ui.push_id(("animation_event", index), |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut event.time)
                                .range(0.0..=duration.max(0.001))
                                .prefix("Time "),
                        );
                        ui.label("Function");
                        ui.text_edit_singleline(&mut event.function_name);
                        if ui.small_button("Remove").clicked() {
                            remove = Some(index);
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.label("String");
                        ui.text_edit_singleline(&mut event.string_parameter);
                        ui.add(egui::DragValue::new(&mut event.float_parameter).prefix("Float "));
                        ui.add(egui::DragValue::new(&mut event.int_parameter).prefix("Int "));
                        ui.add(
                            egui::DragValue::new(&mut event.message_options)
                                .prefix("Message options "),
                        );
                    });
                });
            });
        }
        if let Some(index) = remove {
            events.remove(index);
        }
        if ui.button("Add animation event").clicked() {
            events.push(AnimationEventDef {
                time: duration.max(0.0),
                function_name: "OnAnimationEvent".to_owned(),
                string_parameter: String::new(),
                object_reference: None,
                float_parameter: 0.0,
                int_parameter: 0,
                message_options: 0,
            });
        }
    });
}

fn content_tab(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    egui::ScrollArea::vertical()
        .id_salt("models_assets_editor_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            content_tab_contents(ui, state, preview_texture, preview_status, preview_controls);
        });
}

fn content_tab_contents(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    ui.heading("Models and presentation assets");
    ui.label(
        "Every shipping presentation catalog is editable here: models, imported textures, PBR materials, and animation clips.",
    );
    ui.horizontal_wrapped(|ui| {
        for section in AssetEditorSection::ALL {
            ui.selectable_value(&mut state.asset_section, section, section.label());
        }
        ui.separator();
        if ui.button("Save presentation catalog").clicked() {
            state.status =
                match save_presentation_catalog(&state.presentation, &state.presentation_path) {
                    Ok(path) => format!("Saved presentation catalog to {}", path.display()),
                    Err(error) => format!("Could not save presentation catalog: {error:#}"),
                };
        }
    });
    match state.asset_section {
        AssetEditorSection::Models => {}
        AssetEditorSection::Textures => {
            texture_assets_editor(ui, state, preview_texture, preview_status, preview_controls);
            return;
        }
        AssetEditorSection::Materials => {
            material_assets_editor(ui, state, preview_texture, preview_status, preview_controls);
            return;
        }
        AssetEditorSection::Animations => {
            animation_assets_editor(ui, state, preview_texture, preview_status, preview_controls);
            return;
        }
    }
    let archetype_choices: Vec<_> = state
        .catalog
        .archetypes
        .iter()
        .map(|(id, archetype)| (id.clone(), archetype.display_name.clone()))
        .collect();
    ui.horizontal_wrapped(|ui| {
        stable_id_option_choice(
            ui,
            "Selected model",
            &mut state.selected_archetype,
            &archetype_choices,
        );
        if ui.button("Refresh discovered GLBs").clicked() {
            state.discovered_model_assets = discover_model_assets();
            state.status = format!(
                "Discovered {} GLB assets below bevy-port/assets",
                state.discovered_model_assets.len()
            );
        }
        if ui.button("Save catalog").clicked() {
            state.status = match save_content_catalog(&state.catalog, &state.catalog_path) {
                Ok(path) => format!("Saved model catalog to {}", path.display()),
                Err(error) => format!("Could not save model catalog: {error:#}"),
            };
        }
    });
    draw_model_preview(
        ui,
        preview_texture,
        preview_status,
        egui::vec2(500.0, 333.0),
        preview_controls,
        false,
    );
    ui.collapsing("Import a GLB asset", |ui| {
        ui.label(
            "Copies an existing .glb into assets/user/models. The original is retained and existing destination files are never overwritten.",
        );
        ui.horizontal_wrapped(|ui| {
            ui.label("Source file");
            ui.add(
                egui::TextEdit::singleline(&mut state.model_import_source)
                    .hint_text("C:\\path\\to\\model.glb")
                    .desired_width(420.0),
            );
            ui.label("Destination name (optional)");
            ui.add(
                egui::TextEdit::singleline(&mut state.model_import_name)
                    .hint_text("my_model.glb")
                    .desired_width(220.0),
            );
            if ui.button("Import GLB").clicked() {
                state.status = match import_model_asset(state) {
                    Ok(path) => format!("Imported model asset as {path}"),
                    Err(error) => format!("Model import rejected: {error}"),
                };
            }
        });
    });
    ui.collapsing("Add model archetype", |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label("Stable ID");
            ui.text_edit_singleline(&mut state.new_archetype_id);
            ui.label("Display name");
            ui.text_edit_singleline(&mut state.new_archetype_name);
            archetype_kind_choice(ui, &mut state.new_archetype_kind);
        });
        searchable_string_choice(
            ui,
            "new_archetype_asset",
            "GLB asset",
            &mut state.new_archetype_asset,
            &state.discovered_model_assets,
            &mut state.asset_search,
        );
        if ui.button("Add model archetype").clicked() {
            state.status = match create_model_archetype(state) {
                Ok(()) => "Added a validated model archetype and selected it".to_owned(),
                Err(error) => format!("Model creation rejected: {error}"),
            };
        }
    });

    let selected_archetype = state.selected_archetype.clone();
    if let Some(id) = selected_archetype {
        let model_assets = state.discovered_model_assets.clone();
        let selected_scene = state
            .catalog
            .archetypes
            .get(&id)
            .and_then(|archetype| {
                let index = state
                    .selected_archetype_scene
                    .min(archetype.scenes.len().saturating_sub(1));
                archetype.scenes.get(index)
            })
            .cloned();
        let selected_source_guid = state
            .catalog
            .archetypes
            .get(&id)
            .map(|archetype| archetype.source_guid.clone())
            .unwrap_or_default();
        let selected_metadata = selected_scene
            .as_ref()
            .map(|scene| cached_gltf_metadata(state, &scene.asset_path))
            .unwrap_or_default();
        let resources = resource_choices(&state.catalog);
        let target_kinds = target_kind_choices(&state.catalog);
        let enemy_archetypes = state
            .catalog
            .archetypes
            .iter()
            .filter(|(_, archetype)| archetype.enemy.is_some())
            .map(|(id, archetype)| (id.clone(), archetype.display_name.clone()))
            .collect::<Vec<_>>();
        let enemy_types = labeled_ids(
            state
                .catalog
                .archetypes
                .values()
                .filter_map(|archetype| archetype.enemy.as_ref())
                .map(|enemy| enemy.enemy_type.clone())
                .collect(),
        );
        let enemy_pools = labeled_ids(
            state
                .catalog
                .archetypes
                .values()
                .filter_map(|archetype| archetype.enemy.as_ref())
                .map(|enemy| enemy.pool.clone())
                .collect(),
        );
        let mut model_nodes = building_model_node_choices(&state.catalog);
        model_nodes.extend(selected_metadata.nodes.iter().cloned());
        model_nodes.sort();
        model_nodes.dedup();
        let animation_names = action_animation_choices(&state.catalog);
        let asset_source_models: BTreeMap<_, _> = state
            .catalog
            .archetypes
            .values()
            .flat_map(|archetype| archetype.scenes.iter())
            .map(|scene| (scene.asset_path.clone(), scene.source_model.clone()))
            .collect();
        let mut remove_scene = None;
        let mut make_default = None;
        let mut add_scene = false;
        let mut delete_archetype = false;
        let mut synchronized_footprint = None;
        let asset_search = &mut state.asset_search;
        if let Some(archetype) = state.catalog.archetypes.get_mut(&id) {
            ui.collapsing("Selected archetype and variants", |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Display name");
                    ui.text_edit_singleline(&mut archetype.display_name);
                    archetype_kind_choice(ui, &mut archetype.kind);
                });
                if footprint_editor(
                    ui,
                    &mut archetype.footprint,
                    "Default logical footprint",
                    4_096,
                ) {
                    synchronized_footprint = Some(archetype.footprint);
                }
                ui.collapsing("Bounds, provenance, and components", |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Source GUID");
                        ui.text_edit_singleline(&mut archetype.source_guid);
                        ui.label("Source path");
                        ui.text_edit_singleline(&mut archetype.source_path);
                    });
                    ui.label("Bounds centre / size");
                    ui.horizontal_wrapped(|ui| {
                        for (value, label) in archetype
                            .bounds
                            .center
                            .iter_mut()
                            .zip(["Centre X", "Y", "Z"])
                        {
                            ui.add(egui::DragValue::new(value).prefix(format!("{label} ")));
                        }
                        for (value, label) in
                            archetype.bounds.size.iter_mut().zip(["Size X", "Y", "Z"])
                        {
                            ui.add(
                                egui::DragValue::new(value)
                                    .range(0.001..=100_000.0)
                                    .prefix(format!("{label} ")),
                            );
                        }
                    });
                    free_string_vec_editor(ui, "Component types", &mut archetype.component_types);
                });
                archetype_runtime_editor(
                    ui,
                    archetype,
                    &resources,
                    &target_kinds,
                    &enemy_archetypes,
                    &enemy_types,
                    &enemy_pools,
                    &model_nodes,
                    &animation_names,
                );
                for (index, scene) in archetype.scenes.iter_mut().enumerate() {
                    ui.push_id(("archetype_scene", index), |ui| {
                        ui.group(|ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.strong(format!("Variant {}", index + 1));
                                if ui.radio(scene.is_default, "Default").clicked() {
                                    state.selected_archetype_scene = index;
                                    make_default = Some(index);
                                }
                                ui.add(
                                    egui::DragValue::new(scene.age.get_or_insert(1))
                                        .range(1..=u8::MAX)
                                        .prefix("Age "),
                                );
                                if ui.small_button("No age").clicked() {
                                    scene.age = None;
                                }
                                if ui.small_button("Remove").clicked() {
                                    remove_scene = Some(index);
                                }
                            });
                            if searchable_string_choice(
                                ui,
                                "scene_asset",
                                "GLB asset",
                                &mut scene.asset_path,
                                &model_assets,
                                asset_search,
                            ) {
                                scene.source_model = asset_source_models
                                    .get(&scene.asset_path)
                                    .cloned()
                                    .unwrap_or_else(|| scene.asset_path.clone());
                            }
                        });
                    });
                }
                add_scene = ui.button("Add GLB variant").clicked();
                delete_archetype = ui.button("Delete unreferenced archetype").clicked();
            });
        }
        if let Some(footprint) = synchronized_footprint {
            let updated = synchronize_buildings_for_archetype(&mut state.catalog, &id, footprint);
            if let Some(draft) = state
                .building_draft
                .as_mut()
                .filter(|draft| draft.value.archetype == id)
            {
                draft.value.footprint = footprint;
            }
            state.status = format!(
                "Updated model footprint and synchronized {updated} logical building placement record(s)"
            );
        }
        if let Some(index) = make_default
            && let Some(archetype) = state.catalog.archetypes.get_mut(&id)
        {
            for (scene_index, scene) in archetype.scenes.iter_mut().enumerate() {
                scene.is_default = scene_index == index;
            }
        }
        if let Some(index) = remove_scene {
            state.status = match remove_archetype_scene(state, &id, index) {
                Ok(()) => "Removed model variant".to_owned(),
                Err(error) => format!("Could not remove model variant: {error}"),
            };
        } else if add_scene {
            state.status = match add_archetype_scene(state, &id) {
                Ok(()) => "Added model variant".to_owned(),
                Err(error) => format!("Could not add model variant: {error}"),
            };
        } else if delete_archetype {
            state.status = match delete_model_archetype(state, &id) {
                Ok(()) => "Deleted unreferenced model archetype".to_owned(),
                Err(error) => format!("Could not delete model archetype: {error}"),
            };
        }
        if let Some(scene) = selected_scene {
            gltf_material_bindings_editor(
                ui,
                &mut state.presentation,
                &id,
                &selected_source_guid,
                &scene,
                &selected_metadata,
            );
        }
    }
    ui.horizontal_wrapped(|ui| {
        ui.label(format!("Archetypes: {}", state.catalog.archetypes.len()));
        ui.separator();
        ui.label(format!(
            "GLB variants: {}",
            state
                .catalog
                .archetypes
                .values()
                .map(|archetype| archetype.scenes.len())
                .sum::<usize>()
        ));
        ui.separator();
        ui.label(format!("Materials: {}", state.presentation.materials.len()));
        ui.separator();
        ui.label(format!("Textures: {}", state.presentation.textures.len()));
        ui.separator();
        ui.label(format!(
            "Controllers: {}",
            state.presentation.controllers.len()
        ));
        ui.separator();
        ui.label(format!("Clips: {}", state.presentation.clips.len()));
    });
    ui.collapsing("Complete catalog reference inventory", |ui| {
        ui.collapsing("Prefab archetypes and GLB variants", |ui| {
            for (id, archetype) in &state.catalog.archetypes {
                ui.collapsing(format!("{}  ({id})", archetype.display_name), |ui| {
                    ui.label(format!("Kind: {:?}", archetype.kind));
                    ui.monospace(format!("Unity source: {}", archetype.source_path));
                    for scene in &archetype.scenes {
                        ui.horizontal(|ui| {
                            ui.label(if scene.is_default { "●" } else { "○" });
                            ui.monospace(&scene.asset_path);
                        });
                    }
                    if let Some(bindings) = state
                        .presentation
                        .prefab_renderer_materials
                        .get(&archetype.source_guid)
                    {
                        ui.label(format!("{} renderer binding(s)", bindings.len()));
                    }
                });
            }
        });
        ui.collapsing("Materials and texture bindings", |ui| {
            for (id, material) in &state.presentation.materials {
                ui.collapsing(format!("{}  ({id})", material.display_name), |ui| {
                    let color: egui::Color32 = egui::Rgba::from_rgba_unmultiplied(
                        material.base_color[0].clamp(0.0, 1.0),
                        material.base_color[1].clamp(0.0, 1.0),
                        material.base_color[2].clamp(0.0, 1.0),
                        material.base_color[3].clamp(0.0, 1.0),
                    )
                    .into();
                    ui.colored_label(color, "████  authored base colour");
                    ui.monospace(format!("Unity source: {}", material.source_path));
                    ui.label(format!(
                        "Metallic {:.2} · roughness {:.2} · {:?}",
                        material.metallic, material.perceptual_roughness, material.alpha_mode
                    ));
                    for (slot, texture) in &material.textures {
                        let path = state
                            .presentation
                            .textures
                            .get(texture)
                            .map_or("missing", |texture| texture.asset_path.as_str());
                        ui.monospace(format!("{slot}: {path}"));
                    }
                });
            }
        });
        ui.collapsing("Animation controllers and clips", |ui| {
            for (id, controller) in &state.presentation.controllers {
                ui.collapsing(format!("{}  ({id})", controller.display_name), |ui| {
                    ui.monospace(format!("Unity source: {}", controller.source_path));
                    ui.label(format!(
                        "{} layers · {} states · {} transitions · {} parameters",
                        controller.layers.len(),
                        controller.states.len(),
                        controller.transitions.len(),
                        controller.parameters.len()
                    ));
                    for state_def in controller.states.values() {
                        ui.monospace(format!(
                            "{} · {:.2}x · {} motion(s)",
                            state_def.display_name,
                            state_def.speed,
                            state_def.motions.len()
                        ));
                    }
                });
            }
            ui.separator();
            for (id, clip) in &state.presentation.clips {
                ui.collapsing(format!("{}  ({id})", clip.display_name), |ui| {
                    ui.monospace(format!("Unity source: {}", clip.source_path));
                    ui.label(format!(
                        "{:.3}s · {:.1} Hz · {} tracks · {} events · looping {}",
                        clip.duration_seconds,
                        clip.sample_rate,
                        clip.transform_tracks.len(),
                        clip.events.len(),
                        clip.looping
                    ));
                });
            }
        });
    });
}

fn gltf_material_bindings_editor(
    ui: &mut egui::Ui,
    presentation: &mut PresentationCatalog,
    archetype_id: &StableId,
    prefab_guid: &str,
    scene: &ArchetypeScene,
    metadata: &GltfMetadata,
) {
    let material_choices = presentation
        .materials
        .iter()
        .map(|(id, material)| (id.clone(), material.display_name.clone()))
        .collect::<Vec<_>>();
    ui.collapsing("GLB hierarchy, animations, and material assignments", |ui| {
        ui.label(format!(
            "{} named nodes · {} embedded materials · {} embedded animations",
            metadata.nodes.len(),
            metadata.materials.len(),
            metadata.animations.len()
        ));
        if !metadata.animations.is_empty() {
            ui.horizontal_wrapped(|ui| {
                ui.strong("Animations:");
                for (index, name) in &metadata.animations {
                    ui.monospace(format!("{index}: {name}"));
                }
            });
        }
        ui.label(
            "Model-wide material assignments use names read directly from the GLB; no embedded material name needs to be typed.",
        );
        let bindings = presentation
            .model_materials
            .entry(scene.source_model.clone())
            .or_default();
        for embedded in &metadata.materials {
            ui.push_id(("model_material_binding", embedded), |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.monospace(embedded);
                    let mut selected = bindings.get(embedded).cloned();
                    stable_id_option_choice(
                        ui,
                        "Authored material",
                        &mut selected,
                        &material_choices,
                    );
                    if let Some(material) = selected {
                        bindings.insert(embedded.clone(), material);
                    } else {
                        bindings.remove(embedded);
                    }
                });
            });
        }
        if bindings.is_empty() {
            presentation.model_materials.remove(&scene.source_model);
        }

        ui.separator();
        ui.label(format!(
            "Per-renderer overrides for {} ({archetype_id})",
            scene.asset_path
        ));
        let renderers = presentation
            .prefab_renderer_materials
            .entry(prefab_guid.to_owned())
            .or_default();
        let mut remove_renderer = None;
        for (renderer_index, renderer) in renderers.iter_mut().enumerate() {
            ui.push_id(("renderer_material_binding", renderer_index), |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        string_choice(
                            ui,
                            "Renderer node",
                            &mut renderer.target_path,
                            &metadata.nodes,
                        );
                        if ui.small_button("Remove renderer override").clicked() {
                            remove_renderer = Some(renderer_index);
                        }
                    });
                    let slots = renderer.materials.keys().cloned().collect::<Vec<_>>();
                    let mut remove_slot = None;
                    for embedded in slots {
                        ui.horizontal_wrapped(|ui| {
                            ui.monospace(&embedded);
                            if let Some(material) = renderer.materials.get_mut(&embedded) {
                                stable_id_required_choice(
                                    ui,
                                    "Material",
                                    material,
                                    &material_choices,
                                );
                            }
                            if ui.small_button("Remove slot").clicked() {
                                remove_slot = Some(embedded.clone());
                            }
                        });
                    }
                    if let Some(slot) = remove_slot
                        && renderer.materials.len() > 1
                    {
                        renderer.materials.remove(&slot);
                    }
                    let unused_material = metadata
                        .materials
                        .iter()
                        .find(|name| !renderer.materials.contains_key(*name));
                    if ui
                        .add_enabled(
                            unused_material.is_some() && !material_choices.is_empty(),
                            egui::Button::new("Add renderer material slot"),
                        )
                        .clicked()
                        && let (Some(embedded), Some((material, _))) =
                            (unused_material, material_choices.first())
                    {
                        renderer
                            .materials
                            .insert(embedded.clone(), material.clone());
                    }
                });
            });
        }
        if let Some(index) = remove_renderer {
            renderers.remove(index);
        }
        let unused_node = metadata
            .nodes
            .iter()
            .find(|node| {
                !renderers
                    .iter()
                    .any(|renderer| renderer.target_path == node.as_str())
            });
        if ui
            .add_enabled(
                unused_node.is_some()
                    && !metadata.materials.is_empty()
                    && !material_choices.is_empty(),
                egui::Button::new("Add renderer override"),
            )
            .clicked()
            && let (Some(node), Some(embedded), Some((material, _))) = (
                unused_node,
                metadata.materials.first(),
                material_choices.first(),
            )
        {
            renderers.push(RendererMaterialBinding {
                target_path: node.clone(),
                materials: BTreeMap::from([(embedded.clone(), material.clone())]),
            });
        }
        if renderers.is_empty() {
            presentation.prefab_renderer_materials.remove(prefab_guid);
        }
    });
}

#[allow(dead_code)]
fn legacy_content_tab(ui: &mut egui::Ui, state: &ToolState) {
    let converted_clips = state
        .presentation
        .clips
        .values()
        .filter(|clip| !clip.transform_tracks.is_empty())
        .count();
    let property_curves: usize = state
        .presentation
        .clips
        .values()
        .map(|clip| clip.property_curves.len())
        .sum();
    let animation_events: usize = state
        .presentation
        .clips
        .values()
        .map(|clip| clip.events.len())
        .sum();
    let enemies = state
        .catalog
        .archetypes
        .values()
        .filter(|archetype| archetype.enemy.is_some())
        .count();
    let camps = state
        .catalog
        .archetypes
        .values()
        .filter(|archetype| archetype.enemy_spawner.is_some())
        .count();
    let enemy_model_handlers = state
        .catalog
        .archetypes
        .values()
        .filter(|archetype| archetype.enemy_models.is_some())
        .count();
    ui.heading("Content catalog and stable references");
    ui.label("Versioned RON uses stable IDs; Unity GUIDs remain in typed provenance records.");
    let loading_percent_tenths = state.catalog.loading_screen.progress_milli_per_second;
    ui.label(format!(
        "Loading: {}.{}%/s · {}ms ready hold · {} tooltip(s)",
        loading_percent_tenths / 10,
        loading_percent_tenths % 10,
        state.catalog.loading_screen.completion_hold_milliseconds,
        state.catalog.loading_screen.tooltips.len(),
    ));
    ui.horizontal(|ui| {
        ui.label(format!("Archetypes: {}", state.catalog.archetypes.len()));
        ui.separator();
        ui.label(format!("Buildings: {}", state.catalog.buildings.len()));
        ui.separator();
        ui.label(format!("Roles: {}", state.catalog.roles.len()));
        ui.separator();
        ui.label(format!(
            "Enemies / camp prefabs / generation layers: {enemies} / {camps} / {}",
            state.catalog.enemy_camp_generation.len()
        ));
        ui.separator();
        ui.label(format!("Enemy model sets: {enemy_model_handlers}"));
        ui.separator();
        ui.label(format!(
            "Technology: {}",
            state.catalog.technology.nodes.len()
        ));
        ui.separator();
        ui.label(format!(
            "Provenance: {}",
            state.catalog.source_records.len()
        ));
        ui.separator();
        ui.label(format!(
            "Presentation: {} textures / {} materials / {} renderer bindings / {} clips ({} native transform, {} property curves, {} events) / {} controllers / {} post-process profiles ({} scene bindings) / {} fireworks effects ({} scene emitters) / {} chimney effects ({} prefab emitters) / {} raining-fish effects / {} healing-channel effects / {} healing-burst effects / {} fish-school effects ({} scene bindings) / {} role-audio contracts ({} original clip variants)",
            state.presentation.textures.len(),
            state.presentation.materials.len(),
            state
                .presentation
                .prefab_renderer_materials
                .values()
                .map(Vec::len)
                .sum::<usize>(),
            state.presentation.clips.len(),
            converted_clips,
            property_curves,
            animation_events,
            state.presentation.controllers.len(),
            state.presentation.post_process_profiles.len(),
            state
                .presentation
                .scene_post_process
                .values()
                .map(Vec::len)
                .sum::<usize>(),
            state.presentation.fireworks_effects.len(),
            state
                .presentation
                .scene_fireworks
                .values()
                .map(Vec::len)
                .sum::<usize>(),
            state.presentation.chimney_smoke_effects.len(),
            state
                .presentation
                .prefab_chimney_emitters
                .values()
                .map(Vec::len)
                .sum::<usize>(),
            state.presentation.raining_fish_effects.len(),
            state.presentation.healing_channel_effects.len(),
            state.presentation.healing_burst_effects.len(),
            state.presentation.fish_school_effects.len(),
            state
                .presentation
                .scene_fish_schools
                .values()
                .map(Vec::len)
                .sum::<usize>(),
            state.presentation.role_action_audio.len(),
            state
                .presentation
                .role_action_audio
                .values()
                .map(|audio| audio.clip_guids.len())
                .sum::<usize>()
        ));
    });
    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.collapsing("Enemy camp generation", |ui| {
            for layer in &state.catalog.enemy_camp_generation {
                ui.collapsing(format!("{}  ({})", layer.camp_archetype, layer.id), |ui| {
                    ui.monospace(format!("Unity settings: {}", layer.source_path));
                    ui.label(format!(
                        "Maximum camps: {}; placement attempts per camp: 500",
                        layer.maximum_camps
                    ));
                    ui.label(format!(
                        "Absolute centre offsets: X {:.1}–{:.1} cells, Z {:.1}–{:.1} cells",
                        f64::from(layer.minimum_absolute_offset_milli_cells[0]) / 1_000.0,
                        f64::from(layer.maximum_absolute_offset_milli_cells[0]) / 1_000.0,
                        f64::from(layer.minimum_absolute_offset_milli_cells[1]) / 1_000.0,
                        f64::from(layer.maximum_absolute_offset_milli_cells[1]) / 1_000.0,
                    ));
                    ui.label(format!(
                        "Minimum centre / camp separation: {:.1} / {:.1} cells; camp size {:.1} cells",
                        f64::from(layer.minimum_distance_from_centre_milli_cells) / 1_000.0,
                        f64::from(layer.minimum_distance_between_camps_milli_cells) / 1_000.0,
                        f64::from(layer.camp_size_milli_cells) / 1_000.0,
                    ));
                });
            }
        });
        ui.collapsing("Prefab archetypes", |ui| {
            for (id, archetype) in &state.catalog.archetypes {
                ui.collapsing(format!("{}  ({id})", archetype.display_name), |ui| {
                    if let Some(milliseconds) = archetype.disable_after_milliseconds {
                        ui.monospace(format!(
                            "disables after {:.3}s",
                            f64::from(milliseconds) / 1_000.0
                        ));
                    }
                    if let Some(milliseconds) = archetype.health_bar_hide_milliseconds {
                        ui.monospace(format!(
                            "unit health bar hides {:.3}s after full health",
                            f64::from(milliseconds) / 1_000.0
                        ));
                    }
                    for rotating in &archetype.rotating_nodes {
                        ui.monospace(format!(
                            "rotates {}{}: [{:.1}, {:.1}, {:.1}] at {:.1} deg/s",
                            rotating.hierarchy_path,
                            rotating.age.map_or_else(String::new, |age| format!(" (age {age})")),
                            rotating.axis[0],
                            rotating.axis[1],
                            rotating.axis[2],
                            rotating.degrees_per_second
                        ));
                    }
                    ui.label(format!("Kind: {:?}", archetype.kind));
                    ui.monospace(format!("Unity prefab: {}", archetype.source_path));
                    ui.label(format!(
                        "Footprint: {} x {}; scene variants: {}",
                        archetype.footprint[0],
                        archetype.footprint[1],
                        archetype.scenes.len()
                    ));
                    if archetype.target_size_milli_cells > 0 {
                        ui.label(format!(
                            "Target size: {:.2} cells",
                            f64::from(archetype.target_size_milli_cells) / 1_000.0
                        ));
                    }
                    if let Some(health) = &archetype.health {
                        ui.label(format!(
                            "Health: {} base; +{} per level; {} milli/s regeneration",
                            health.max_health,
                            health.health_gain_per_level,
                            health.regeneration_milli_per_second
                        ));
                    }
                    if let Some(enemy) = &archetype.enemy {
                        ui.label(format!(
                            "Enemy: {} damage every {:.1}s at {:.1} cells; searches {:.1} cells",
                            enemy.action_amount,
                            f64::from(enemy.action_milliseconds) / 1_000.0,
                            f64::from(enemy.action_range_milli_cells) / 1_000.0,
                            f64::from(enemy.target_search_range_milli_cells) / 1_000.0,
                        ));
                        ui.monospace(format!("retaliates when attacked: {}", enemy.attack_attacker));
                        ui.monospace(format!(
                            "player kill reward: {} {}",
                            enemy.kill_reward.amount, enemy.kill_reward.resource
                        ));
                    }
                    if let Some(models) = &archetype.enemy_models {
                        ui.label(format!(
                            "Enemy models: {} base / {} permanent / {} optional / {} weapons",
                            models.base_models.len(),
                            models.permanent_models.len(),
                            models.optional_models.len(),
                            models.weapons.len()
                        ));
                        for weapon in &models.weapons {
                            ui.monospace(format!(
                                "weapon {}: {} x{}; {:?} run; {} off-hand",
                                weapon.main_model,
                                weapon.action_animation,
                                weapon.action_animation_variants,
                                weapon.run_animation,
                                weapon.off_hand_models.len()
                            ));
                        }
                    }
                    for scene in &archetype.scenes {
                        let marker = if scene.is_default {
                            "default"
                        } else {
                            "variant"
                        };
                        ui.monospace(format!("{marker}: {}", scene.asset_path));
                    }
                    if let Some(materials) = state
                        .presentation
                        .prefab_materials
                        .get(&archetype.source_guid)
                    {
                        for material_id in materials {
                            let name = state
                                .presentation
                                .materials
                                .get(material_id)
                                .map_or("missing", |material| material.display_name.as_str());
                            ui.monospace(format!("material: {name} ({material_id})"));
                        }
                    }
                });
            }
        });
        ui.collapsing("Buildings", |ui| {
            for (id, building) in &state.catalog.buildings {
                ui.collapsing(format!("{}  ({id})", building.display_name), |ui| {
                    ui.monospace(format!("Archetype: {}", building.archetype));
                    ui.label(format!(
                        "Footprint: {} x {}",
                        building.footprint[0], building.footprint[1]
                    ));
                    for (resource, amount) in &building.cost {
                        ui.label(format!("{resource}: {amount}"));
                    }
                    if let Some(station) = &building.station {
                        ui.separator();
                        ui.label(format!(
                            "Station: {} role kinds, {} target kinds, {} target slots, {:.1} cells",
                            station.accepted_role_kinds.len(),
                            station.target_kinds.len(),
                            station.max_targets,
                            station.search_range_milli_cells / 1_000
                        ));
                        for kind in &station.accepted_role_kinds {
                            ui.monospace(format!("accepts: {kind}"));
                        }
                        for kind in &station.target_kinds {
                            ui.monospace(format!("target: {kind}"));
                        }
                    }
                    if let Some(targeting) = &building.targeting {
                        ui.separator();
                        ui.label(format!(
                            "Target scoring: +{:.2} per assignment, +{:.2} per cell",
                            f64::from(targeting.assignment_penalty_milli) / 1_000.0,
                            f64::from(targeting.distance_penalty_milli_per_cell) / 1_000.0
                        ));
                    }
                    for income in &building.passive_resources {
                        ui.separator();
                        ui.label(format!(
                            "Passive: +{:.2} {} / second",
                            f64::from(income.base_milli_per_second) / 1_000.0,
                            income.resource
                        ));
                        ui.monospace(format!(
                            "+{:.2} / level x {} authored callbacks",
                            f64::from(income.increment_milli_per_level) / 1_000.0,
                            income.level_event_repetitions
                        ));
                    }
                    if !building.model_handlers.is_empty() {
                        ui.separator();
                        ui.label(format!(
                            "Models: {} construction/upgrade handlers",
                            building.model_handlers.len()
                        ));
                        for model in &building.model_handlers {
                            ui.monospace(format!(
                                "age {}: {} + {} upgrade layers",
                                model.age,
                                model.full_model,
                                model.upgrades.len()
                            ));
                        }
                    }
                    for model in &building.storage_models {
                        ui.monospace(format!(
                            "storage age {} ({}): {} / {} / {}",
                            model.age,
                            model.resource,
                            model.empty_model,
                            model.half_full_model,
                            model.full_model
                        ));
                    }
                    if let Some(shooter) = &building.projectile_shooter {
                        ui.separator();
                        ui.label(format!(
                            "Projectile: {} damage every {:.1}s, {:.1}-cell range, {:.1} cells/s",
                            shooter.damage,
                            f64::from(shooter.fire_milliseconds) / 1_000.0,
                            f64::from(shooter.range_milli_cells) / 1_000.0,
                            f64::from(shooter.movement_milli_cells_per_second) / 1_000.0,
                        ));
                        ui.monospace(format!("Unity pool: {}", shooter.projectile_pool));
                    }
                });
            }
        });
        ui.collapsing("Roles", |ui| {
            for (id, role) in &state.catalog.roles {
                ui.collapsing(format!("{}  ({id})", role.display_name), |ui| {
                    ui.label(format!(
                        "Movement: {} per-thousand",
                        role.movement_speed_multiplier_per_thousand
                    ));
                    for ability in &role.granted_abilities {
                        ui.monospace(ability.to_string());
                    }
                    for station in &role.station_kinds {
                        ui.monospace(format!("station: {station}"));
                    }
                    for target in &role.target_kinds {
                        ui.monospace(format!("target: {target}"));
                    }
                    if let Some(equipment) = &role.equipment {
                        ui.label(format!(
                            "Equipment: body {}; right {}; left {}; helmet {}",
                            equipment.body_nodes[0],
                            equipment.right_hand_node.as_deref().unwrap_or("none"),
                            equipment.left_hand_node.as_deref().unwrap_or("none"),
                            equipment.helmet_node.as_deref().unwrap_or("none"),
                        ));
                        if let Some(animation) = &equipment.carry_animation {
                            ui.label(format!("Carry animation: {animation}"));
                        }
                    }
                });
            }
        });
        ui.collapsing("Materials and texture bindings", |ui| {
            ui.label(format!(
                "{} model material-name bindings; {} renderer paths / {} material slots",
                state
                    .presentation
                    .model_materials
                    .values()
                    .map(BTreeMap::len)
                    .sum::<usize>(),
                state
                    .presentation
                    .prefab_renderer_materials
                    .values()
                    .map(Vec::len)
                    .sum::<usize>(),
                state
                    .presentation
                    .prefab_renderer_materials
                    .values()
                    .flat_map(|renderers| renderers.iter())
                    .map(|renderer| renderer.materials.len())
                    .sum::<usize>()
            ));
            for (id, material) in &state.presentation.materials {
                ui.collapsing(format!("{}  ({id})", material.display_name), |ui| {
                    ui.monospace(format!("Unity material: {}", material.source_path));
                    ui.label(format!(
                        "PBR base {:?}; metallic {:.2}; roughness {:.2}; {:?}",
                        material.base_color,
                        material.metallic,
                        material.perceptual_roughness,
                        material.alpha_mode
                    ));
                    if let Some(shader) = &material.shader_source {
                        ui.monospace(format!("Shader source: {shader}"));
                    }
                    for (slot, texture) in &material.textures {
                        let path = state
                            .presentation
                            .textures
                            .get(texture)
                            .map_or("missing", |texture| texture.asset_path.as_str());
                        ui.monospace(format!("{slot}: {path}"));
                    }
                    ui.label(format!(
                        "{} custom shader properties retained for WGSL porting",
                        material.custom_properties.len() + material.custom_vectors.len()
                    ));
                    ui.label(format!(
                        "{} authored texture transforms",
                        material.texture_transforms.len()
                    ));
                });
            }
        });
        ui.collapsing("Animation controllers", |ui| {
            for (id, controller) in &state.presentation.controllers {
                ui.collapsing(format!("{}  ({id})", controller.display_name), |ui| {
                    ui.monospace(format!("Unity controller: {}", controller.source_path));
                    ui.label(format!(
                        "{} parameters ({} inferred), {} states, {} transitions, {} layers",
                        controller.parameters.len(),
                        controller
                            .parameters
                            .iter()
                            .filter(|parameter| parameter.inferred)
                            .count(),
                        controller.states.len(),
                        controller.transitions.len(),
                        controller.layers.len()
                    ));
                    for (index, layer) in controller.layers.iter().enumerate() {
                        let mask = layer
                            .avatar_mask
                            .as_ref()
                            .and_then(|id| state.presentation.avatar_masks.get(id));
                        ui.monospace(format!(
                            "{}: {:?}, weight {:.2}{}",
                            layer.display_name,
                            layer.blend_mode,
                            layer.effective_weight(index),
                            mask.map_or_else(String::new, |mask| format!(
                                ", mask {} ({} enabled / {} disabled transforms)",
                                mask.display_name,
                                mask.transform_weights
                                    .values()
                                    .filter(|weight| (**weight - 1.0).abs() < f32::EPSILON)
                                    .count(),
                                mask.transform_weights
                                    .values()
                                    .filter(|weight| **weight < f32::EPSILON)
                                    .count()
                            ))
                        ));
                    }
                    for state_def in controller.states.values() {
                        ui.label(format!(
                            "{} (speed {:.2}, {} motions{})",
                            state_def.display_name,
                            state_def.speed,
                            state_def.motions.len(),
                            state_def
                                .blend_parameter
                                .as_ref()
                                .map_or_else(String::new, |parameter| format!(
                                    ", 1D blend: {parameter}"
                                ))
                        ));
                    }
                });
            }
        });
        ui.collapsing("Animation clips", |ui| {
            for (id, clip) in &state.presentation.clips {
                ui.collapsing(format!("{}  ({id})", clip.display_name), |ui| {
                    ui.monospace(format!("Unity clip: {}", clip.source_path));
                    ui.label(format!(
                        "{:.3}s at {:.1} Hz, {} transform tracks, {} property curves, {} events, looping: {}",
                        clip.duration_seconds,
                        clip.sample_rate,
                        clip.transform_tracks.len(),
                        clip.property_curves.len(),
                        clip.events.len(),
                        clip.looping
                    ));
                    for curve in &clip.property_curves {
                        ui.monospace(format!(
                            "{} :: {} (class {}, {} keys)",
                            curve.target_path,
                            curve.attribute,
                            curve.class_id,
                            curve.keys.len()
                        ));
                    }
                    for event in &clip.events {
                        ui.monospace(format!(
                            "event {:.3}s: {}",
                            event.time, event.function_name
                        ));
                    }
                    if let Some(rig) = &clip.rig_asset_path {
                        ui.monospace(format!("Retarget rig: {rig}"));
                    }
                });
            }
        });
    });
}

fn buildings_tab(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    ui.heading("Building authoring");
    ui.label(
        "Create a building from a complete shipping template, choose its model archetype visually, and edit gameplay references without typing stable IDs.",
    );
    ui.horizontal_wrapped(|ui| {
        if ui
            .add_enabled(!state.undo_authoring.is_empty(), egui::Button::new("Undo"))
            .clicked()
        {
            undo_authoring_edit(state);
        }
        if ui
            .add_enabled(!state.redo_authoring.is_empty(), egui::Button::new("Redo"))
            .clicked()
        {
            redo_authoring_edit(state);
        }
        if ui.button("Save catalog").clicked() {
            state.status = match save_content_catalog(&state.catalog, &state.catalog_path) {
                Ok(path) => format!("Saved validated building catalog to {}", path.display()),
                Err(error) => format!("Could not save building catalog: {error:#}"),
            };
        }
    });
    let building_choices: Vec<_> = state
        .catalog
        .buildings
        .iter()
        .map(|(id, building)| (id.clone(), building.display_name.clone()))
        .collect();
    let mut changed = false;
    egui::ComboBox::from_label("Building")
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(
            state
                .selected_building
                .as_ref()
                .and_then(|id| state.catalog.buildings.get(id))
                .map_or("Select building", |building| building.display_name.as_str()),
        )
        .show_ui(ui, |ui| {
            for (id, name) in &building_choices {
                changed |= ui
                    .selectable_value(&mut state.selected_building, Some(id.clone()), name)
                    .changed();
            }
        });
    if changed {
        refresh_building_draft(state);
    }
    ui.horizontal_wrapped(|ui| {
        ui.label("New stable ID");
        ui.text_edit_singleline(&mut state.new_building_id);
        ui.label("Name");
        ui.text_edit_singleline(&mut state.new_building_name);
        if ui
            .add_enabled(
                state.selected_building.is_some(),
                egui::Button::new("Create from selected"),
            )
            .on_hover_text("Copies every model, balance, station, and presentation field")
            .clicked()
        {
            state.status = match duplicate_selected_building(state) {
                Ok(()) => "Created a complete building draft from the selected template".to_owned(),
                Err(error) => format!("Building creation rejected: {error}"),
            };
        }
        if ui
            .add_enabled(
                state.selected_building.is_some(),
                egui::Button::new("Delete"),
            )
            .clicked()
        {
            state.status = match delete_selected_building(state) {
                Ok(()) => "Deleted unreferenced building".to_owned(),
                Err(error) => format!("Building deletion rejected: {error}"),
            };
        }
    });

    let archetype_choices: Vec<_> = state
        .catalog
        .archetypes
        .iter()
        .filter(|(_, archetype)| archetype.kind == ArchetypeKind::Building)
        .map(|(id, archetype)| (id.clone(), archetype.display_name.clone()))
        .collect();
    let resources = resource_choices(&state.catalog);
    let roles: Vec<_> = state
        .catalog
        .roles
        .iter()
        .map(|(id, role)| (id.clone(), role.display_name.clone()))
        .collect();
    let station_kinds = station_kind_choices(&state.catalog);
    let target_kinds = target_kind_choices(&state.catalog);
    let building_asset_path = state
        .building_draft
        .as_ref()
        .and_then(|draft| state.catalog.archetypes.get(&draft.value.archetype))
        .and_then(|archetype| {
            archetype
                .scenes
                .iter()
                .find(|scene| scene.is_default)
                .or_else(|| archetype.scenes.first())
        })
        .map(|scene| scene.asset_path.clone());
    let building_metadata = building_asset_path
        .as_deref()
        .map(|path| cached_gltf_metadata(state, path))
        .unwrap_or_default();
    let mut model_nodes = building_model_node_choices(&state.catalog);
    model_nodes.extend(building_metadata.nodes);
    model_nodes.sort();
    model_nodes.dedup();
    let projectile_pools = projectile_pool_choices(&state.catalog);
    let mut apply = false;
    let mut reset = false;
    if let Some(draft) = state.building_draft.as_mut() {
        ui.separator();
        ui.monospace(draft.id.to_string());
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.columns(2, |columns| {
                columns[0].label("Display name");
                columns[0].text_edit_singleline(&mut draft.value.display_name);
                if stable_id_required_choice(
                    &mut columns[0],
                    "Model archetype",
                    &mut draft.value.archetype,
                    &archetype_choices,
                ) && let Some(archetype) = state.catalog.archetypes.get(&draft.value.archetype)
                {
                    draft.value.footprint = archetype.footprint;
                }
                footprint_editor(
                    &mut columns[0],
                    &mut draft.value.footprint,
                    "Shared logical placement footprint",
                    64,
                );
                let mut custom_navigation = draft.value.navigation_footprint_thirds.is_some();
                if columns[0]
                    .checkbox(
                        &mut custom_navigation,
                        "Override physical navigation footprint",
                    )
                    .changed()
                {
                    draft.value.navigation_footprint_thirds = custom_navigation.then(|| {
                        default_navigation_footprint_thirds(draft.value.footprint)
                    });
                }
                if let Some(navigation) = draft.value.navigation_footprint_thirds.as_mut() {
                    let maximum = draft.value.footprint.map(|axis| axis.saturating_mul(3));
                    columns[0].label("Physical footprint (third-cell navigation units)");
                    columns[0].horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut navigation[0])
                                .range(1..=maximum[0])
                                .prefix("Width "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut navigation[1])
                                .range(1..=maximum[1])
                                .prefix("Depth "),
                        );
                    });
                    navigation[0] = navigation[0].clamp(1, maximum[0]);
                    navigation[1] = navigation[1].clamp(1, maximum[1]);
                    columns[0].small(format!(
                        "Occupies {:.2} × {:.2} placement cells, centered inside the placement/exclusion footprint.",
                        f32::from(navigation[0]) / 3.0,
                        f32::from(navigation[1]) / 3.0,
                    ));
                } else {
                    let inferred = default_navigation_footprint_thirds(draft.value.footprint);
                    columns[0].small(format!(
                        "Default physical footprint: {} × {} third-cells (one unit inset per side).",
                        inferred[0], inferred[1],
                    ));
                }
                columns[0].small(
                    "Changing the model adopts its authored footprint. Applying saves this value to both the building and model archetype atomically.",
                );
                columns[0].horizontal_wrapped(|ui| {
                    ui.checkbox(&mut draft.value.placeable, "Placeable");
                    ui.checkbox(&mut draft.value.can_level, "Can level");
                });
                columns[0].add(
                    egui::DragValue::new(&mut draft.value.level_cost_multiplier_per_thousand)
                        .range(1..=100_000)
                        .prefix("Level cost multiplier /1000 "),
                );
                draw_building_visual(
                    &mut columns[1],
                    &draft.value,
                    &state.catalog,
                    preview_texture,
                    preview_status,
                    preview_controls,
                );
            });
            ui.collapsing("Construction and level costs", |ui| {
                stable_u32_map_editor(ui, "Construction cost", &mut draft.value.cost, &resources);
                stable_u32_map_editor(
                    ui,
                    "Per-level cost",
                    &mut draft.value.level_cost,
                    &resources,
                );
            });
            ui.collapsing("Storage", |ui| {
                let mut remove = None;
                for (index, storage) in draft.value.storage.iter_mut().enumerate() {
                    ui.horizontal_wrapped(|ui| {
                        stable_id_required_choice(
                            ui,
                            "Resource",
                            &mut storage.resource,
                            &resources,
                        );
                        ui.add(egui::DragValue::new(&mut storage.base_amount).prefix("Base "));
                        ui.add(
                            egui::DragValue::new(&mut storage.increment_amount)
                                .prefix("Per level "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut storage.level_multiplier_per_thousand)
                                .prefix("Multiplier /1000 "),
                        );
                        if ui.small_button("Remove").clicked() {
                            remove = Some(index);
                        }
                    });
                }
                if let Some(index) = remove {
                    draft.value.storage.remove(index);
                }
                if ui.button("Add storage contribution").clicked()
                    && let Some((resource, _)) = resources.first()
                {
                    draft.value.storage.push(StorageContribution {
                        resource: resource.clone(),
                        base_amount: 100,
                        increment_amount: 0,
                        level_multiplier_per_thousand: 1_000,
                    });
                }
            });
            ui.collapsing("Role capacity", |ui| {
                let mut remove = None;
                for (index, slot) in draft.value.role_slots.iter_mut().enumerate() {
                    ui.horizontal_wrapped(|ui| {
                        stable_id_required_choice(ui, "Role", &mut slot.role, &roles);
                        ui.add(egui::DragValue::new(&mut slot.base_amount).prefix("Base slots "));
                        ui.add(
                            egui::DragValue::new(&mut slot.increment_amount).prefix("Per level "),
                        );
                        if ui.small_button("Remove").clicked() {
                            remove = Some(index);
                        }
                    });
                }
                if let Some(index) = remove {
                    draft.value.role_slots.remove(index);
                }
                if ui.button("Add role capacity").clicked()
                    && let Some((role, _)) = roles.first()
                {
                    draft.value.role_slots.push(RoleSlotContribution {
                        role: role.clone(),
                        base_amount: 1,
                        increment_amount: 0,
                    });
                }
            });
            ui.collapsing("Passive production", |ui| {
                let mut remove = None;
                for (index, income) in draft.value.passive_resources.iter_mut().enumerate() {
                    ui.horizontal_wrapped(|ui| {
                        stable_id_required_choice(ui, "Resource", &mut income.resource, &resources);
                        ui.add(
                            egui::DragValue::new(&mut income.base_milli_per_second)
                                .prefix("Base milli/s "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut income.increment_milli_per_level)
                                .prefix("Per level "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut income.level_event_repetitions)
                                .prefix("Callbacks "),
                        );
                        if ui.small_button("Remove").clicked() {
                            remove = Some(index);
                        }
                    });
                }
                if let Some(index) = remove {
                    draft.value.passive_resources.remove(index);
                }
                if ui.button("Add passive resource").clicked()
                    && let Some((resource, _)) = resources.first()
                {
                    draft
                        .value
                        .passive_resources
                        .push(PassiveResourceContribution {
                            resource: resource.clone(),
                            base_milli_per_second: 1_000,
                            increment_milli_per_level: 0,
                            level_event_repetitions: 1,
                        });
                }
            });
            ui.collapsing("Station and target selection", |ui| {
                let mut enabled = draft.value.station.is_some();
                if ui
                    .checkbox(&mut enabled, "Acts as a work station")
                    .changed()
                {
                    draft.value.station = enabled.then(|| StationDef {
                        accepts_all_roles: false,
                        accepted_role_kinds: BTreeSet::new(),
                        targets_all: false,
                        target_kinds: BTreeSet::new(),
                        max_targets: 1,
                        update_milliseconds: 1_000,
                        search_range_milli_cells: 10_000,
                    });
                }
                if let Some(station) = draft.value.station.as_mut() {
                    ui.checkbox(&mut station.accepts_all_roles, "Accept all roles");
                    stable_id_set_choices(
                        ui,
                        "Accepted role kinds",
                        &mut station.accepted_role_kinds,
                        &station_kinds,
                    );
                    ui.checkbox(&mut station.targets_all, "Target every kind");
                    stable_id_set_choices(
                        ui,
                        "Target kinds",
                        &mut station.target_kinds,
                        &target_kinds,
                    );
                    ui.horizontal_wrapped(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut station.max_targets)
                                .prefix("Maximum targets "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut station.update_milliseconds)
                                .prefix("Refresh ms "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut station.search_range_milli_cells)
                                .prefix("Range milli-cells "),
                        );
                    });
                }
                let mut targeting = draft.value.targeting.is_some();
                if ui.checkbox(&mut targeting, "Uses target scoring").changed() {
                    draft.value.targeting = targeting.then_some(TargetingScoreDef {
                        assignment_penalty_milli: 1_000,
                        distance_penalty_milli_per_cell: 1_000,
                    });
                }
                if let Some(targeting) = draft.value.targeting.as_mut() {
                    ui.add(
                        egui::DragValue::new(&mut targeting.assignment_penalty_milli)
                            .prefix("Assignment penalty "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut targeting.distance_penalty_milli_per_cell)
                            .prefix("Distance penalty/cell "),
                    );
                }
            });
            ui.collapsing("Projectile attack", |ui| {
                let mut enabled = draft.value.projectile_shooter.is_some();
                if ui.checkbox(&mut enabled, "Shoots projectiles").changed() {
                    let pool = projectile_pools
                        .first()
                        .cloned()
                        .unwrap_or_else(|| "projectile:default".to_owned());
                    draft.value.projectile_shooter = enabled.then_some(ProjectileShooterDef {
                        projectile_pool: pool,
                        movement_milli_cells_per_second: 10_000,
                        damage: 1,
                        range_milli_cells: 10_000,
                        fire_milliseconds: 1_000,
                    });
                }
                if let Some(shooter) = draft.value.projectile_shooter.as_mut() {
                    string_choice(
                        ui,
                        "Projectile pool",
                        &mut shooter.projectile_pool,
                        &projectile_pools,
                    );
                    ui.horizontal_wrapped(|ui| {
                        ui.add(egui::DragValue::new(&mut shooter.damage).prefix("Damage "));
                        ui.add(
                            egui::DragValue::new(&mut shooter.fire_milliseconds)
                                .prefix("Cadence ms "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut shooter.range_milli_cells)
                                .prefix("Range milli-cells "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut shooter.movement_milli_cells_per_second)
                                .prefix("Projectile speed "),
                        );
                    });
                }
            });
            ui.collapsing("Model hierarchy bindings", |ui| {
                let mut remove_handler = None;
                for (index, model) in draft.value.model_handlers.iter_mut().enumerate() {
                    ui.push_id(("building_model_handler", index), |ui| {
                        ui.group(|ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.strong(format!("Model handler {}", index + 1));
                                ui.add(egui::DragValue::new(&mut model.age).prefix("Age "));
                                if ui.small_button("Remove handler").clicked() {
                                    remove_handler = Some(index);
                                }
                            });
                            string_choice(
                                ui,
                                "Complete model",
                                &mut model.full_model,
                                &model_nodes,
                            );
                            for (stage, node) in model.construction_stages.iter_mut().enumerate() {
                                ui.push_id(("construction_stage", stage), |ui| {
                                    string_choice(
                                        ui,
                                        &format!("Construction stage {}", stage + 1),
                                        node,
                                        &model_nodes,
                                    );
                                });
                            }
                            string_vec_choices(
                                ui,
                                "Upgrade layers",
                                &mut model.upgrades,
                                &model_nodes,
                            );
                            string_vec_choices(
                                ui,
                                "Other controlled models",
                                &mut model.other_models,
                                &model_nodes,
                            );
                        })
                    });
                }
                if let Some(index) = remove_handler {
                    draft.value.model_handlers.remove(index);
                }
                if ui.button("Add model handler").clicked()
                    && let Some(first) = model_nodes.first()
                {
                    draft.value.model_handlers.push(BuildingModelDef {
                        age: 1,
                        full_model: first.clone(),
                        construction_stages: [first.clone(), first.clone(), first.clone()],
                        upgrades: Vec::new(),
                        other_models: Vec::new(),
                    });
                }
                ui.separator();
                let mut remove_storage_model = None;
                for (index, model) in draft.value.storage_models.iter_mut().enumerate() {
                    ui.push_id(("building_storage_model", index), |ui| {
                        ui.group(|ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.strong(format!("Storage model {}", index + 1));
                                ui.add(egui::DragValue::new(&mut model.age).prefix("Age "));
                                if ui.small_button("Remove storage model").clicked() {
                                    remove_storage_model = Some(index);
                                }
                            });
                            stable_id_required_choice(
                                ui,
                                "Resource",
                                &mut model.resource,
                                &resources,
                            );
                            string_choice(ui, "Empty", &mut model.empty_model, &model_nodes);
                            string_choice(
                                ui,
                                "Half full",
                                &mut model.half_full_model,
                                &model_nodes,
                            );
                            string_choice(ui, "Full", &mut model.full_model, &model_nodes);
                        })
                    });
                }
                if let Some(index) = remove_storage_model {
                    draft.value.storage_models.remove(index);
                }
                if ui.button("Add storage model").clicked()
                    && let (Some(first), Some((resource, _))) =
                        (model_nodes.first(), resources.first())
                {
                    draft.value.storage_models.push(StorageModelDef {
                        age: 1,
                        resource: resource.clone(),
                        empty_model: first.clone(),
                        half_full_model: first.clone(),
                        full_model: first.clone(),
                    });
                }
            });
            ui.horizontal(|ui| {
                apply = ui.button("Apply validated building").clicked();
                reset = ui.button("Discard draft").clicked();
            });
        });
    } else {
        ui.label("Select a building to edit it.");
    }
    if apply {
        state.status = match apply_building_draft(state) {
            Ok(()) => "Building edit applied; every reference remains valid".to_owned(),
            Err(error) => format!("Building edit rejected: {error}"),
        };
    } else if reset {
        refresh_building_draft(state);
        "Discarded building draft".clone_into(&mut state.status);
    }
}

fn roles_tab(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    ui.heading("Role authoring");
    ui.label(
        "Create roles from a known-good template, then author every runtime reference through catalog-backed choices.",
    );
    ui.horizontal_wrapped(|ui| {
        if ui
            .add_enabled(!state.undo_authoring.is_empty(), egui::Button::new("Undo"))
            .clicked()
        {
            undo_authoring_edit(state);
        }
        if ui
            .add_enabled(!state.redo_authoring.is_empty(), egui::Button::new("Redo"))
            .clicked()
        {
            redo_authoring_edit(state);
        }
        if ui.button("Save catalog").clicked() {
            state.status = match save_content_catalog(&state.catalog, &state.catalog_path) {
                Ok(path) => format!("Saved validated role catalog to {}", path.display()),
                Err(error) => format!("Could not save role catalog: {error:#}"),
            };
        }
    });

    let roles: Vec<_> = state
        .catalog
        .roles
        .iter()
        .map(|(id, role)| (id.clone(), role.display_name.clone()))
        .collect();
    let mut selected_changed = false;
    egui::ComboBox::from_label("Role")
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(
            state
                .selected_role
                .as_ref()
                .and_then(|id| state.catalog.roles.get(id))
                .map_or("Select role", |role| role.display_name.as_str()),
        )
        .show_ui(ui, |ui| {
            for (id, name) in &roles {
                selected_changed |= ui
                    .selectable_value(&mut state.selected_role, Some(id.clone()), name)
                    .changed();
            }
        });
    if selected_changed {
        refresh_role_draft(state);
    }
    ui.horizontal_wrapped(|ui| {
        ui.label("New stable ID");
        ui.text_edit_singleline(&mut state.new_role_id);
        ui.label("Name");
        ui.text_edit_singleline(&mut state.new_role_name);
        if ui
            .add_enabled(
                state.selected_role.is_some(),
                egui::Button::new("Create from selected"),
            )
            .on_hover_text("Copies a validated role as a complete starting template")
            .clicked()
        {
            state.status = match duplicate_selected_role(state) {
                Ok(()) => "Created a complete role draft from the selected template".to_owned(),
                Err(error) => format!("Role creation rejected: {error}"),
            };
        }
        if ui
            .add_enabled(state.selected_role.is_some(), egui::Button::new("Delete"))
            .clicked()
        {
            state.status = match delete_selected_role(state) {
                Ok(()) => "Deleted unreferenced role".to_owned(),
                Err(error) => format!("Role deletion rejected: {error}"),
            };
        }
    });

    let resource_choices = resource_choices(&state.catalog);
    let station_choices = station_kind_choices(&state.catalog);
    let target_choices = target_kind_choices(&state.catalog);
    let ability_choices = ability_choices(&state.catalog);
    let animation_choices = action_animation_choices(&state.catalog);
    let character_metadata =
        cached_gltf_metadata(state, "migrated/models/Models/Characters/Characters.glb");
    let mut equipment_choices = equipment_node_choices(&state.catalog, &state.presentation);
    equipment_choices.extend(character_metadata.nodes);
    equipment_choices.sort();
    equipment_choices.dedup();
    let role_animation_choices = role_preview_animation_choices(&state.presentation);
    let eye_choices = equipment_choices
        .iter()
        .filter(|name| name.starts_with("Eyes_"))
        .cloned()
        .collect::<Vec<_>>();
    let hair_choices = equipment_choices
        .iter()
        .filter(|name| name.starts_with("Hair_"))
        .cloned()
        .collect::<Vec<_>>();
    let facial_hair_choices = equipment_choices
        .iter()
        .filter(|name| name.starts_with("FacialHair_"))
        .cloned()
        .collect::<Vec<_>>();
    let mut apply = false;
    let mut reset = false;
    if let Some(draft) = state.role_draft.as_mut() {
        ui.separator();
        ui.monospace(draft.id.to_string());
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.columns(2, |columns| {
                draw_model_preview(
                    &mut columns[0],
                    preview_texture,
                    preview_status,
                    egui::vec2(440.0, 360.0),
                    preview_controls,
                    true,
                );
                columns[1].group(|ui| {
                    ui.heading("Preview composition");
                    ui.label(
                        "This uses the shipping character rig, the draft role's equipment, and the selected Animator state.",
                    );
                    egui::ComboBox::from_label("Body type")
                        .height(AUTHORING_COMBO_MAX_HEIGHT)
                        .selected_text(match state.role_preview_body_type {
                            1 => "Bulk",
                            2 => "Feminine",
                            _ => "Slim",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut state.role_preview_body_type, 0, "Slim");
                            ui.selectable_value(&mut state.role_preview_body_type, 1, "Bulk");
                            ui.selectable_value(&mut state.role_preview_body_type, 2, "Feminine");
                        });
                    stable_id_option_choice(
                        ui,
                        "Animation state",
                        &mut state.role_preview_animation,
                        &role_animation_choices,
                    );
                    ui.checkbox(
                        &mut state.role_preview_carrying,
                        "Show conditional carried item",
                    );
                    optional_string_choice(
                        ui,
                        "Eyes",
                        &mut state.role_preview_eye,
                        &eye_choices,
                    );
                    optional_string_choice(
                        ui,
                        "Hair",
                        &mut state.role_preview_hair,
                        &hair_choices,
                    );
                    optional_string_choice(
                        ui,
                        "Facial hair",
                        &mut state.role_preview_facial_hair,
                        &facial_hair_choices,
                    );
                    ui.small(
                        "Helmet-equipped roles hide hair, matching runtime composition. Orbit, camera-relative pan, zoom, playback, and looping controls are live.",
                    );
                });
            });
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label("Display name");
                ui.text_edit_singleline(&mut draft.value.display_name);
                string_choice(
                    ui,
                    "Action animation",
                    &mut draft.value.action_animation,
                    &animation_choices,
                );
                ui.add(
                    egui::DragValue::new(&mut draft.value.action_animation_variants)
                        .range(1..=u8::MAX)
                        .prefix("Variants "),
                );
            });
            ui.horizontal_wrapped(|ui| {
                ui.checkbox(&mut draft.value.has_user_limit, "Limit users");
                ui.add(
                    egui::DragValue::new(&mut draft.value.base_max_users).prefix("Base max users "),
                );
                ui.checkbox(
                    &mut draft.value.targets_all,
                    "Targets all matching entities",
                );
            });
            stable_id_option_choice(
                ui,
                "Produced/carried resource",
                &mut draft.value.resource,
                &resource_choices,
            );
            stable_id_set_choices(
                ui,
                "Compatible station kinds",
                &mut draft.value.station_kinds,
                &station_choices,
            );
            stable_id_set_choices(
                ui,
                "Target kinds",
                &mut draft.value.target_kinds,
                &target_choices,
            );
            stable_id_vec_choices(
                ui,
                "Granted abilities",
                &mut draft.value.granted_abilities,
                &ability_choices,
            );

            ui.collapsing("Progression and action balance", |ui| {
                egui::Grid::new("typed_role_balance_grid")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        role_u16(
                            ui,
                            "Movement multiplier (per thousand)",
                            &mut draft.value.movement_speed_multiplier_per_thousand,
                        );
                        role_u32(
                            ui,
                            "Experience multiplier (per thousand)",
                            &mut draft.value.experience_multiplier_per_thousand,
                        );
                        role_u32(
                            ui,
                            "Base action amount",
                            &mut draft.value.base_action_amount,
                        );
                        role_u32(
                            ui,
                            "Action/level (milli)",
                            &mut draft.value.action_amount_per_level_milli,
                        );
                        role_u32(
                            ui,
                            "Base action cadence (ms)",
                            &mut draft.value.base_action_milliseconds,
                        );
                        role_u32(
                            ui,
                            "Cadence reduction/level (ms)",
                            &mut draft.value.action_milliseconds_reduction_per_level,
                        );
                        role_u32(
                            ui,
                            "Base action range (milli-cells)",
                            &mut draft.value.base_action_range_milli_cells,
                        );
                        role_u32(
                            ui,
                            "Action range/level (milli-cells)",
                            &mut draft.value.action_range_milli_cells_per_level,
                        );
                        role_u32(ui, "Base health", &mut draft.value.base_health);
                        role_u32(
                            ui,
                            "Health/level (milli)",
                            &mut draft.value.health_per_level_milli,
                        );
                        role_i32(
                            ui,
                            "Base health regeneration/s",
                            &mut draft.value.base_health_regen_per_second,
                        );
                        role_u32(
                            ui,
                            "Health regeneration/level (milli/s)",
                            &mut draft.value.health_regen_milli_per_second_per_level,
                        );
                        role_i32(
                            ui,
                            "Base damage reduction (%)",
                            &mut draft.value.base_damage_reduction_percent,
                        );
                        role_u32(
                            ui,
                            "Damage reduction/level (milli-%)",
                            &mut draft.value.damage_reduction_milli_percent_per_level,
                        );
                        role_u32(
                            ui,
                            "Base movement (milli-cells/s)",
                            &mut draft.value.base_movement_speed_milli_cells_per_second,
                        );
                        role_u32(
                            ui,
                            "Movement/level (milli-cells/s)",
                            &mut draft.value.movement_speed_milli_cells_per_second_per_level,
                        );
                        role_u32(
                            ui,
                            "Base carry capacity",
                            &mut draft.value.base_carry_capacity,
                        );
                        role_u32(
                            ui,
                            "Carry capacity/level (milli)",
                            &mut draft.value.carry_capacity_per_level_milli,
                        );
                    });
            });

            ui.collapsing("Character model and equipment", |ui| {
                let mut enabled = draft.value.equipment.is_some();
                if ui
                    .checkbox(&mut enabled, "Role has equipment bindings")
                    .changed()
                {
                    if enabled {
                        let first = equipment_choices.first().cloned().unwrap_or_default();
                        draft.value.equipment = Some(RoleEquipmentDef {
                            body_nodes: [first.clone(), first.clone(), first],
                            left_hand_node: None,
                            right_hand_node: None,
                            helmet_node: None,
                            carry_animation: None,
                            left_hand_permanent: false,
                        });
                    } else {
                        draft.value.equipment = None;
                    }
                }
                if let Some(equipment) = draft.value.equipment.as_mut() {
                    for (index, label) in ["Slim body", "Bulk body", "Feminine body"]
                        .into_iter()
                        .enumerate()
                    {
                        string_choice(
                            ui,
                            label,
                            &mut equipment.body_nodes[index],
                            &equipment_choices,
                        );
                    }
                    optional_string_choice(
                        ui,
                        "Left-hand model",
                        &mut equipment.left_hand_node,
                        &equipment_choices,
                    );
                    optional_string_choice(
                        ui,
                        "Right-hand model",
                        &mut equipment.right_hand_node,
                        &equipment_choices,
                    );
                    optional_string_choice(
                        ui,
                        "Helmet model",
                        &mut equipment.helmet_node,
                        &equipment_choices,
                    );
                    optional_string_choice(
                        ui,
                        "Carry animation",
                        &mut equipment.carry_animation,
                        &animation_choices,
                    );
                    ui.checkbox(
                        &mut equipment.left_hand_permanent,
                        "Left-hand item remains visible outside carry actions",
                    );
                }
            });
            ui.horizontal(|ui| {
                apply = ui.button("Apply validated role").clicked();
                reset = ui.button("Discard draft").clicked();
            });
        });
    } else {
        ui.label("Select a role to edit it.");
    }
    if apply {
        state.status = match apply_role_draft(state) {
            Ok(()) => "Role edit applied; every catalog reference remains valid".to_owned(),
            Err(error) => format!("Role edit rejected: {error}"),
        };
    } else if reset {
        refresh_role_draft(state);
        "Discarded role draft".clone_into(&mut state.status);
    }
}

#[allow(dead_code)]
fn legacy_roles_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Role authoring");
    ui.label(
        "Edit authoritative role balance, targeting, station compatibility, animation contracts, and equipment node bindings. Changes remain a draft until the complete catalog validates.",
    );
    ui.horizontal_wrapped(|ui| {
        if ui
            .add_enabled(!state.undo_authoring.is_empty(), egui::Button::new("Undo"))
            .clicked()
        {
            undo_authoring_edit(state);
        }
        if ui
            .add_enabled(!state.redo_authoring.is_empty(), egui::Button::new("Redo"))
            .clicked()
        {
            redo_authoring_edit(state);
        }
        if ui.button("Validate catalog").clicked() {
            state.status = match state.catalog.validate() {
                Ok(()) => format!("Catalog valid: {} roles", state.catalog.roles.len()),
                Err(error) => format!("Catalog error: {error}"),
            };
        }
        if ui.button("Save validated catalog").clicked() {
            state.status = match save_content_catalog(&state.catalog, &state.catalog_path) {
                Ok(path) => format!("Saved validated content catalog to {}", path.display()),
                Err(error) => format!("Could not save content catalog: {error:#}"),
            };
        }
        ui.label("Search");
        ui.text_edit_singleline(&mut state.role_search);
    });
    ui.horizontal(|ui| {
        ui.label("Catalog path");
        ui.text_edit_singleline(&mut state.catalog_path);
        if ui.button("Reload catalog").clicked() {
            state.status = match reload_content_catalog(state) {
                Ok(()) => "Reloaded and validated content catalog".to_owned(),
                Err(error) => format!("Could not reload content catalog: {error:#}"),
            };
        }
    });

    let search = state.role_search.trim().to_ascii_lowercase();
    let choices: Vec<_> = state
        .catalog
        .roles
        .iter()
        .filter(|(id, role)| {
            search.is_empty()
                || id.as_str().contains(&search)
                || role.display_name.to_ascii_lowercase().contains(&search)
        })
        .map(|(id, role)| (id.clone(), role.display_name.clone()))
        .collect();
    let mut selected = None;
    egui::ScrollArea::horizontal()
        .id_salt("role_selector")
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                for (id, name) in &choices {
                    if ui
                        .selectable_label(
                            state.selected_role.as_ref() == Some(id),
                            format!("{name}  ({id})"),
                        )
                        .clicked()
                    {
                        selected = Some(id.clone());
                    }
                }
            });
        });
    if let Some(id) = selected {
        state.selected_role = Some(id.clone());
        state.role_draft = role_draft(&state.catalog, &id);
    }

    ui.horizontal_wrapped(|ui| {
        ui.label("Duplicate as stable ID");
        ui.text_edit_singleline(&mut state.new_role_id);
        ui.label("Name");
        ui.text_edit_singleline(&mut state.new_role_name);
        if ui
            .add_enabled(
                state.selected_role.is_some(),
                egui::Button::new("Duplicate role"),
            )
            .clicked()
        {
            state.status = match duplicate_selected_role(state) {
                Ok(()) => "Duplicated role into a validated catalog draft".to_owned(),
                Err(error) => format!("Role duplication rejected: {error}"),
            };
        }
        if ui
            .add_enabled(
                state.selected_role.is_some(),
                egui::Button::new("Delete role"),
            )
            .clicked()
        {
            state.status = match delete_selected_role(state) {
                Ok(()) => "Deleted unreferenced role".to_owned(),
                Err(error) => format!("Role deletion rejected: {error}"),
            };
        }
    });

    let mut apply = false;
    let mut reset = false;
    if let Some(draft) = state.role_draft.as_mut() {
        ui.separator();
        ui.monospace(draft.id.to_string());
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.collapsing("Identity and behavior", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Display name");
                    ui.text_edit_singleline(&mut draft.value.display_name);
                    ui.label("Action animation");
                    ui.text_edit_singleline(&mut draft.value.action_animation);
                    ui.add(
                        egui::DragValue::new(&mut draft.value.action_animation_variants)
                            .range(1..=u8::MAX)
                            .prefix("Variants "),
                    );
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut draft.value.has_user_limit, "Limit users");
                    ui.add(
                        egui::DragValue::new(&mut draft.value.base_max_users)
                            .prefix("Base max users "),
                    );
                    ui.checkbox(
                        &mut draft.value.targets_all,
                        "Targets all matching entities",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Produced/carried resource (optional stable ID)");
                    ui.text_edit_singleline(&mut draft.resource);
                });
                ui.label("Station kind stable IDs (comma separated)");
                ui.text_edit_singleline(&mut draft.station_kinds);
                ui.label("Target kind stable IDs (comma separated)");
                ui.text_edit_singleline(&mut draft.target_kinds);
                ui.label("Granted ability stable IDs (comma separated)");
                ui.text_edit_singleline(&mut draft.granted_abilities);
            });
            ui.collapsing("Progression and action balance", |ui| {
                egui::Grid::new("role_balance_grid")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        role_u16(
                            ui,
                            "Movement multiplier (per thousand)",
                            &mut draft.value.movement_speed_multiplier_per_thousand,
                        );
                        role_u32(
                            ui,
                            "Experience multiplier (per thousand)",
                            &mut draft.value.experience_multiplier_per_thousand,
                        );
                        role_u32(
                            ui,
                            "Base action amount",
                            &mut draft.value.base_action_amount,
                        );
                        role_u32(
                            ui,
                            "Action/level (milli)",
                            &mut draft.value.action_amount_per_level_milli,
                        );
                        role_u32(
                            ui,
                            "Base action cadence (ms)",
                            &mut draft.value.base_action_milliseconds,
                        );
                        role_u32(
                            ui,
                            "Cadence reduction/level (ms)",
                            &mut draft.value.action_milliseconds_reduction_per_level,
                        );
                        role_u32(
                            ui,
                            "Base action range (milli-cells)",
                            &mut draft.value.base_action_range_milli_cells,
                        );
                        role_u32(
                            ui,
                            "Action range/level (milli-cells)",
                            &mut draft.value.action_range_milli_cells_per_level,
                        );
                        role_u32(ui, "Base health", &mut draft.value.base_health);
                        role_u32(
                            ui,
                            "Health/level (milli)",
                            &mut draft.value.health_per_level_milli,
                        );
                        role_i32(
                            ui,
                            "Base health regeneration/s",
                            &mut draft.value.base_health_regen_per_second,
                        );
                        role_u32(
                            ui,
                            "Health regeneration/level (milli/s)",
                            &mut draft.value.health_regen_milli_per_second_per_level,
                        );
                        role_i32(
                            ui,
                            "Base damage reduction (%)",
                            &mut draft.value.base_damage_reduction_percent,
                        );
                        role_u32(
                            ui,
                            "Damage reduction/level (milli-%)",
                            &mut draft.value.damage_reduction_milli_percent_per_level,
                        );
                        role_u32(
                            ui,
                            "Base movement (milli-cells/s)",
                            &mut draft.value.base_movement_speed_milli_cells_per_second,
                        );
                        role_u32(
                            ui,
                            "Movement/level (milli-cells/s)",
                            &mut draft.value.movement_speed_milli_cells_per_second_per_level,
                        );
                        role_u32(
                            ui,
                            "Base carry capacity",
                            &mut draft.value.base_carry_capacity,
                        );
                        role_u32(
                            ui,
                            "Carry capacity/level (milli)",
                            &mut draft.value.carry_capacity_per_level_milli,
                        );
                    });
            });
            ui.collapsing("Character model and equipment bindings", |ui| {
                ui.checkbox(&mut draft.has_equipment, "Role has equipment bindings");
                if draft.has_equipment {
                    for (index, label) in ["Slim body node", "Bulk body node", "Feminine body node"]
                        .into_iter()
                        .enumerate()
                    {
                        ui.horizontal(|ui| {
                            ui.label(label);
                            ui.text_edit_singleline(&mut draft.body_nodes[index]);
                        });
                    }
                    for (label, value) in [
                        ("Left-hand node", &mut draft.left_hand_node),
                        ("Right-hand node", &mut draft.right_hand_node),
                        ("Helmet node", &mut draft.helmet_node),
                        ("Carry animation", &mut draft.carry_animation),
                    ] {
                        ui.horizontal(|ui| {
                            ui.label(label);
                            ui.text_edit_singleline(value);
                        });
                    }
                    ui.checkbox(
                        &mut draft.left_hand_permanent,
                        "Left-hand item remains visible outside carry actions",
                    );
                }
            });
            ui.horizontal(|ui| {
                apply = ui.button("Apply validated role edit").clicked();
                reset = ui.button("Discard role draft").clicked();
            });
        });
    } else {
        ui.label("Select a role to edit it.");
    }
    if apply {
        state.status = match apply_role_draft(state) {
            Ok(()) => "Role edit applied; all catalog references remain valid".to_owned(),
            Err(error) => format!("Role edit rejected: {error}"),
        };
    } else if reset {
        refresh_role_draft(state);
        "Discarded role draft".clone_into(&mut state.status);
    }
}

fn role_u16(ui: &mut egui::Ui, label: &str, value: &mut u16) {
    ui.label(label);
    ui.add(egui::DragValue::new(value));
    ui.end_row();
}

fn role_u32(ui: &mut egui::Ui, label: &str, value: &mut u32) {
    ui.label(label);
    ui.add(egui::DragValue::new(value));
    ui.end_row();
}

fn role_i32(ui: &mut egui::Ui, label: &str, value: &mut i32) {
    ui.label(label);
    ui.add(egui::DragValue::new(value));
    ui.end_row();
}

fn objective_catalog_editor(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    objective_choices: &[(StableId, String)],
    resources: &[(StableId, String)],
    buildings: &[(StableId, String)],
    enemies: &[(StableId, String)],
) {
    ui.collapsing("Vote requirement catalog", |ui| {
        ui.label(
            "Technology requirements are reusable typed records. Create them here, then attach them to node cards with the searchable requirement menu.",
        );
        let before = state.selected_objective.clone();
        stable_id_option_choice(
            ui,
            "Selected requirement",
            &mut state.selected_objective,
            objective_choices,
        );
        if before != state.selected_objective {
            refresh_objective_draft(state);
        }
        let mut duplicate = false;
        let mut delete = false;
        ui.horizontal_wrapped(|ui| {
            ui.label("New stable ID");
            ui.text_edit_singleline(&mut state.new_objective_id);
            duplicate = ui
                .add_enabled(
                    state.objective_draft.is_some(),
                    egui::Button::new("Duplicate selected requirement"),
                )
                .clicked();
            delete = ui
                .add_enabled(
                    state.objective_draft.is_some(),
                    egui::Button::new("Delete unreferenced requirement"),
                )
                .clicked();
        });
        let mut apply = false;
        let mut discard = false;
        if let Some(draft) = state.objective_draft.as_mut() {
            ui.group(|ui| {
                ui.monospace(draft.id.to_string());
                ui.horizontal_wrapped(|ui| {
                    objective_kind_choice(ui, &mut draft.value.kind);
                    ui.add(
                        egui::DragValue::new(&mut draft.value.required_amount)
                            .range(1..=u32::MAX)
                            .prefix("Required amount "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut draft.value.float_value_milli)
                            .prefix("Float value milli "),
                    );
                });
                stable_id_option_choice(
                    ui,
                    "Resource target",
                    &mut draft.value.resource,
                    resources,
                );
                stable_id_option_choice(
                    ui,
                    "Building target",
                    &mut draft.value.building,
                    buildings,
                );
                stable_id_option_choice(
                    ui,
                    "Enemy target",
                    &mut draft.value.enemy,
                    enemies,
                );
                ui.horizontal(|ui| {
                    apply = ui.button("Apply validated requirement").clicked();
                    discard = ui.button("Discard requirement draft").clicked();
                });
            });
        }
        if duplicate {
            state.status = match duplicate_selected_objective(state) {
                Ok(()) => "Created vote requirement from selected template".to_owned(),
                Err(error) => format!("Requirement creation rejected: {error}"),
            };
        } else if delete {
            state.status = match delete_selected_objective(state) {
                Ok(()) => "Deleted unreferenced vote requirement".to_owned(),
                Err(error) => format!("Requirement deletion rejected: {error}"),
            };
        } else if apply {
            state.status = match apply_objective_draft(state) {
                Ok(()) => "Vote requirement applied and validated".to_owned(),
                Err(error) => format!("Requirement edit rejected: {error}"),
            };
        } else if discard {
            refresh_objective_draft(state);
        }
    });
}

fn objective_kind_choice(ui: &mut egui::Ui, value: &mut ObjectiveKind) {
    egui::ComboBox::from_id_salt(("objective_kind", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(format!("{value:?}"))
        .show_ui(ui, |ui| {
            for kind in [
                ObjectiveKind::Build,
                ObjectiveKind::BuildAny,
                ObjectiveKind::Collect,
                ObjectiveKind::Kill,
                ObjectiveKind::KillAny,
                ObjectiveKind::EarnPerHour,
                ObjectiveKind::Sell,
                ObjectiveKind::SellAny,
                ObjectiveKind::Buy,
                ObjectiveKind::BuyAny,
            ] {
                ui.selectable_value(value, kind, format!("{kind:?}"));
            }
        });
}

fn technology_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.horizontal_wrapped(|ui| {
        ui.heading("Technology graph authoring");
        if ui
            .add_enabled(!state.undo_authoring.is_empty(), egui::Button::new("Undo"))
            .clicked()
        {
            undo_authoring_edit(state);
        }
        if ui
            .add_enabled(!state.redo_authoring.is_empty(), egui::Button::new("Redo"))
            .clicked()
        {
            redo_authoring_edit(state);
        }
        if ui.button("Validate").clicked() {
            let validation = state
                .catalog
                .validate()
                .map_err(|error| error.to_string())
                .and_then(|()| {
                    state
                        .technology_layout
                        .validate(&state.catalog.technology)
                        .map_err(|error| error.to_string())
                });
            state.status = match validation {
                Ok(()) => format!(
                    "Technology graph valid: {} nodes in {} groups",
                    state.catalog.technology.nodes.len(),
                    state.catalog.technology.groups.len()
                ),
                Err(error) => format!("Technology graph error: {error}"),
            };
        }
        if ui.button("Save catalog + layout").clicked() {
            state.status = match save_content_catalog(&state.catalog, &state.catalog_path).and_then(
                |catalog_path| {
                    save_technology_layout(
                        &state.technology_layout,
                        &state.catalog,
                        &state.technology_layout_path,
                    )
                    .map(|layout_path| (catalog_path, layout_path))
                },
            ) {
                Ok((catalog, layout)) => format!(
                    "Saved technology catalog to {} and layout to {}",
                    catalog.display(),
                    layout.display()
                ),
                Err(error) => format!("Could not save technology authoring assets: {error:#}"),
            };
        }
        if ui.button("Auto layout").clicked() {
            let previous = authoring_snapshot(state);
            state.technology_layout = TechnologyGraphLayout::automatic(&state.catalog.technology);
            push_authoring_undo(state, previous);
            state.technology_graph_view.request_fit();
        }
        if ui.button("Fit all").clicked() {
            state.technology_graph_view.request_fit();
        }
        ui.checkbox(&mut state.technology_graph_view.show_minimap, "Minimap");
    });
    ui.label(
        "The graph is the primary workspace. Node cards expose requirements and effects; drag an output socket into another node's input to create a prerequisite.",
    );

    let technology_choices: Vec<_> = state
        .catalog
        .technology
        .nodes
        .iter()
        .map(|(id, node)| (id.clone(), node.display_name.clone()))
        .collect();
    let group_choices: Vec<_> = state
        .catalog
        .technology
        .groups
        .iter()
        .map(|(id, group)| (id.clone(), group.display_name.clone()))
        .collect();
    let building_choices: Vec<_> = state
        .catalog
        .buildings
        .iter()
        .map(|(id, building)| (id.clone(), building.display_name.clone()))
        .collect();
    let role_choices: Vec<_> = state
        .catalog
        .roles
        .iter()
        .map(|(id, role)| (id.clone(), role.display_name.clone()))
        .collect();
    let resources = resource_choices(&state.catalog);
    let stat_options = stat_choices(&state.catalog);
    let objective_choices: Vec<_> = state
        .catalog
        .objectives
        .iter()
        .map(|(id, objective)| {
            (
                id.clone(),
                format!("{:?} ×{}", objective.kind, objective.required_amount),
            )
        })
        .collect();
    let enemy_choices: Vec<_> = state
        .catalog
        .archetypes
        .iter()
        .filter(|(_, archetype)| archetype.enemy.is_some())
        .map(|(id, archetype)| (id.clone(), archetype.display_name.clone()))
        .collect();
    let icon_choices = technology_icon_choices(state);

    objective_catalog_editor(
        ui,
        state,
        &objective_choices,
        &resources,
        &building_choices,
        &enemy_choices,
    );

    ui.horizontal_wrapped(|ui| {
        ui.label("Search");
        ui.text_edit_singleline(&mut state.technology_search);
        ui.label("New group");
        ui.text_edit_singleline(&mut state.new_group_id);
        ui.text_edit_singleline(&mut state.new_group_name);
        if ui.button("Add group").clicked() {
            state.status = match create_technology_group(state) {
                Ok(()) => "Created technology group".to_owned(),
                Err(error) => format!("Group creation rejected: {error}"),
            };
        }
        if ui
            .add_enabled(
                state.selected_group.is_some(),
                egui::Button::new("Delete selected group"),
            )
            .clicked()
        {
            state.status = match delete_selected_technology_group(state) {
                Ok(()) => "Deleted empty technology group".to_owned(),
                Err(error) => format!("Group deletion rejected: {error}"),
            };
        }
    });
    ui.horizontal_wrapped(|ui| {
        stable_id_option_choice(
            ui,
            "New node group",
            &mut state.selected_group,
            &group_choices,
        );
        ui.text_edit_singleline(&mut state.new_technology_id);
        ui.text_edit_singleline(&mut state.new_technology_name);
        if ui
            .add_enabled(
                state.selected_group.is_some(),
                egui::Button::new("Add node"),
            )
            .clicked()
        {
            state.status = match create_technology_node(state) {
                Ok(()) => "Created complete technology node draft".to_owned(),
                Err(error) => format!("Node creation rejected: {error}"),
            };
        }
    });

    let selected_node = state
        .technology_draft
        .as_ref()
        .map(|draft| draft.id.clone());
    let before = state.technology_layout.clone();
    let output = show_technology_graph(
        ui,
        &state.catalog,
        &mut state.technology_layout,
        &mut state.technology_graph_view,
        selected_node.as_ref(),
        state.selected_group.as_ref(),
        &state.technology_search,
    );
    if output.layout_edit_started {
        push_authoring_undo(
            state,
            AuthoringSnapshot {
                catalog: state.catalog.clone(),
                technology_layout: before,
            },
        );
    }
    if let Some(group) = output.selected_group {
        state.selected_group = Some(group);
    }
    if let Some(node) = output.selected_node {
        state.technology_draft = technology_draft(&state.catalog, &node);
    }
    if let Some((source, target)) = output.connection_requested {
        state.status = match connect_technology_nodes(state, &source, &target) {
            Ok(()) => format!("Connected {source} as a prerequisite of {target}"),
            Err(error) => format!("Connection rejected: {error}"),
        };
    }
    if let Some((source, target)) = output.connection_removal_requested {
        state.status = match disconnect_technology_nodes(state, &source, &target) {
            Ok(()) => format!("Removed prerequisite connection {source} → {target}"),
            Err(error) => format!("Connection removal rejected: {error}"),
        };
    }

    let objective_details = state.catalog.objectives.clone();
    let mut apply = false;
    let mut discard = false;
    let mut delete = false;
    if let Some(selected) = state
        .technology_draft
        .as_ref()
        .map(|draft| draft.id.clone())
    {
        egui::Window::new("Technology node editor")
            .id(egui::Id::new(("technology_node_editor", selected.as_str())))
            .default_pos(egui::pos2(
                ui.max_rect().right() - 580.0,
                ui.max_rect().top() + 110.0,
            ))
            .default_size(egui::vec2(550.0, 720.0))
            .resizable(true)
            .show(ui.ctx(), |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("technology_node_editor_scroll")
                    .show(ui, |ui| {
                        let Some(draft) = state.technology_draft.as_mut() else {
                            return;
                        };
                        ui.heading(&draft.value.display_name);
                        ui.monospace(draft.id.to_string());
                        ui.label("Display name");
                        ui.text_edit_singleline(&mut draft.value.display_name);
                        ui.label("Description");
                        ui.text_edit_multiline(&mut draft.value.description);
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Age");
                            ui.text_edit_singleline(&mut draft.value.age);
                            ui.add(egui::DragValue::new(&mut draft.value.tier).prefix("Tier "));
                        });
                        stable_id_option_choice(
                            ui,
                            "Group",
                            &mut draft.value.group,
                            &group_choices,
                        );
                        searchable_string_choice(
                            ui,
                            "technology_icon",
                            "Icon",
                            &mut draft.value.icon_path,
                            &icon_choices,
                            &mut draft.icon_search,
                        );
                        ui.horizontal_wrapped(|ui| {
                            ui.checkbox(&mut draft.value.initially_unlocked, "Initially unlocked");
                            ui.checkbox(&mut draft.value.unavailable, "Unavailable");
                        });

                        let node_choices: Vec<_> = technology_choices
                            .iter()
                            .filter(|(id, _)| id != &draft.id)
                            .cloned()
                            .collect();
                        searchable_stable_id_vec_editor(
                            ui,
                            "prerequisites",
                            "Prerequisites",
                            &mut draft.value.prerequisites,
                            &node_choices,
                            &mut draft.prerequisite_search,
                        );
                        searchable_stable_id_vec_editor(
                            ui,
                            "unlocks",
                            "Explicit unlocks",
                            &mut draft.value.unlocks,
                            &node_choices,
                            &mut draft.unlock_search,
                        );
                        searchable_stable_id_vec_editor(
                            ui,
                            "objectives",
                            "Vote requirements",
                            &mut draft.value.objectives,
                            &objective_choices,
                            &mut draft.objective_search,
                        );
                        for objective_id in &draft.value.objectives {
                            if let Some(objective) = objective_details.get(objective_id) {
                                ui.monospace(format!(
                                    "  {:?} ×{}{}{}{}",
                                    objective.kind,
                                    objective.required_amount,
                                    objective
                                        .resource
                                        .as_ref()
                                        .map_or_else(String::new, |id| format!(" · {id}")),
                                    objective
                                        .building
                                        .as_ref()
                                        .map_or_else(String::new, |id| format!(" · {id}")),
                                    objective
                                        .enemy
                                        .as_ref()
                                        .map_or_else(String::new, |id| format!(" · {id}")),
                                ));
                            }
                        }

                        ui.collapsing("Building effects", |ui| {
                            stable_u16_map_editor(
                                ui,
                                "Maximum levels",
                                &mut draft.value.building_level_caps,
                                &building_choices,
                            );
                            stable_id_set_choices(
                                ui,
                                "Unlock buildings",
                                &mut draft.value.unlocked_buildings,
                                &building_choices,
                            );
                            stable_i32_map_editor(
                                ui,
                                "Cost reductions (%)",
                                &mut draft.value.building_cost_reduction_percent,
                                &building_choices,
                            );
                            ui.add(
                                egui::DragValue::new(
                                    &mut draft.value.global_building_cost_reduction_percent,
                                )
                                .suffix("% global cost reduction"),
                            );
                            stable_id_set_choices(
                                ui,
                                "Age-up buildings",
                                &mut draft.value.aged_buildings,
                                &building_choices,
                            );
                        });
                        ui.collapsing("Storage and stat effects", |ui| {
                            stable_i32_map_editor(
                                ui,
                                "Storage boosts (%)",
                                &mut draft.value.storage_boost_percent,
                                &resources,
                            );
                            stable_i32_map_editor(
                                ui,
                                "Global stat boosts (%)",
                                &mut draft.value.global_stat_boost_percent,
                                &stat_options,
                            );
                            role_stat_map_editor(
                                ui,
                                &mut draft.value.role_stat_boost_percent,
                                &role_choices,
                                &stat_options,
                            );
                        });
                        ui.horizontal_wrapped(|ui| {
                            apply = ui.button("Apply validated node").clicked();
                            discard = ui.button("Discard draft").clicked();
                            delete = ui.button("Delete node").clicked();
                        });
                    });
            });
    }
    if apply {
        state.status = match apply_technology_draft(state) {
            Ok(()) => "Technology node applied; graph and references remain valid".to_owned(),
            Err(error) => format!("Technology edit rejected: {error}"),
        };
    } else if discard {
        refresh_technology_draft(state);
        "Discarded technology draft".clone_into(&mut state.status);
    } else if delete {
        state.status = match delete_selected_technology_node(state) {
            Ok(()) => "Deleted technology node and cleaned graph references".to_owned(),
            Err(error) => format!("Technology deletion rejected: {error}"),
        };
    }
}

fn enemy_camp_generation_editor(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    ui.collapsing("Enemy camp generation layers", |ui| {
        let layer_choices = state
            .catalog
            .enemy_camp_generation
            .iter()
            .map(|layer| (layer.id.clone(), layer.id.to_string()))
            .collect::<Vec<_>>();
        let camp_choices = state
            .catalog
            .archetypes
            .iter()
            .filter(|(_, archetype)| archetype.enemy_spawner.is_some())
            .map(|(id, archetype)| (id.clone(), archetype.display_name.clone()))
            .collect::<Vec<_>>();
        let source_paths = state
            .catalog
            .source_records
            .values()
            .map(|record| record.source_path.clone())
            .chain(
                state
                    .catalog
                    .enemy_camp_generation
                    .iter()
                    .map(|layer| layer.source_path.clone()),
            )
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let before = state.selected_enemy_camp_generation.clone();
        stable_id_option_choice(
            ui,
            "Camp layer",
            &mut state.selected_enemy_camp_generation,
            &layer_choices,
        );
        if before != state.selected_enemy_camp_generation {
            refresh_enemy_camp_generation_draft(state);
            state.world_preview_layer = WorldPreviewLayer::EnemyCamps;
        }
        let mut duplicate = false;
        let mut delete = false;
        ui.horizontal_wrapped(|ui| {
            ui.label("New layer ID");
            ui.text_edit_singleline(&mut state.new_enemy_camp_generation_id);
            duplicate = ui
                .add_enabled(
                    state.enemy_camp_generation_draft.is_some(),
                    egui::Button::new("Duplicate selected camp layer"),
                )
                .clicked();
            delete = ui
                .add_enabled(
                    state.enemy_camp_generation_draft.is_some(),
                    egui::Button::new("Remove selected camp layer"),
                )
                .clicked();
        });
        let mut apply = false;
        let mut discard = false;
        if let Some(layer) = state.enemy_camp_generation_draft.as_mut() {
            ui.group(|ui| {
                ui.monospace(layer.id.to_string());
                stable_id_required_choice(
                    ui,
                    "Camp/spawner archetype",
                    &mut layer.camp_archetype,
                    &camp_choices,
                );
                searchable_string_choice(
                    ui,
                    "enemy_camp_source",
                    "Unity source",
                    &mut layer.source_path,
                    &source_paths,
                    &mut state.world_asset_search,
                );
                ui.horizontal_wrapped(|ui| {
                    ui.label("Minimum absolute offset");
                    for value in &mut layer.minimum_absolute_offset_milli_cells {
                        ui.add(egui::DragValue::new(value).prefix("milli-cells "));
                    }
                    ui.label("Maximum absolute offset");
                    for value in &mut layer.maximum_absolute_offset_milli_cells {
                        ui.add(egui::DragValue::new(value).prefix("milli-cells "));
                    }
                });
                ui.horizontal_wrapped(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut layer.maximum_camps)
                            .range(1..=u16::MAX)
                            .prefix("Maximum camps "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut layer.minimum_distance_from_centre_milli_cells)
                            .prefix("Minimum centre distance "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut layer.minimum_distance_between_camps_milli_cells)
                            .range(1..=u32::MAX)
                            .prefix("Minimum camp spacing "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut layer.camp_size_milli_cells)
                            .range(1..=u32::MAX)
                            .prefix("Camp size "),
                    );
                });
                if state.world_preview_layer == WorldPreviewLayer::EnemyCamps {
                    draw_model_preview(
                        ui,
                        preview_texture,
                        preview_status,
                        egui::vec2(360.0, 240.0),
                        preview_controls,
                        false,
                    );
                } else if ui
                    .button("Preview this camp model and placement range")
                    .clicked()
                {
                    state.world_preview_layer = WorldPreviewLayer::EnemyCamps;
                }
                ui.horizontal(|ui| {
                    apply = ui.button("Apply validated camp layer").clicked();
                    discard = ui.button("Discard camp draft").clicked();
                });
            });
        }
        if duplicate {
            state.status = match duplicate_enemy_camp_generation_layer(state) {
                Ok(()) => "Duplicated enemy camp generation layer".to_owned(),
                Err(error) => format!("Camp layer creation rejected: {error}"),
            };
        } else if delete {
            state.status = match delete_enemy_camp_generation_layer(state) {
                Ok(()) => "Removed enemy camp generation layer".to_owned(),
                Err(error) => format!("Camp layer removal rejected: {error}"),
            };
        } else if apply {
            state.status = match apply_enemy_camp_generation_draft(state) {
                Ok(()) => "Enemy camp generation layer applied and validated".to_owned(),
                Err(error) => format!("Camp generation edit rejected: {error}"),
            };
        } else if discard {
            refresh_enemy_camp_generation_draft(state);
        }
    });
}

#[allow(dead_code)]
fn legacy_technology_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Technology graph editor");
    ui.label(
        "Edit the complete shipping graph directly. Node/group positions are versioned separately from runtime technology data and every mutation participates in the same undo history.",
    );
    ui.horizontal_wrapped(|ui| {
        if ui
            .add_enabled(!state.undo_authoring.is_empty(), egui::Button::new("Undo"))
            .clicked()
        {
            undo_authoring_edit(state);
        }
        if ui
            .add_enabled(!state.redo_authoring.is_empty(), egui::Button::new("Redo"))
            .clicked()
        {
            redo_authoring_edit(state);
        }
        if ui.button("Validate graph").clicked() {
            let validation = state
                .catalog
                .validate()
                .map_err(|error| error.to_string())
                .and_then(|()| {
                    state
                        .technology_layout
                        .validate(&state.catalog.technology)
                        .map_err(|error| error.to_string())
                });
            state.status = match validation {
                Ok(()) => format!(
                    "Technology graph valid: {} nodes in {} groups",
                    state.catalog.technology.nodes.len(),
                    state.catalog.technology.groups.len()
                ),
                Err(error) => format!("Technology graph error: {error}"),
            };
        }
        if ui.button("Save catalog + layout").clicked() {
            state.status = match save_content_catalog(&state.catalog, &state.catalog_path).and_then(
                |catalog_path| {
                    save_technology_layout(
                        &state.technology_layout,
                        &state.catalog,
                        &state.technology_layout_path,
                    )
                    .map(|layout_path| (catalog_path, layout_path))
                },
            ) {
                Ok((catalog_path, layout_path)) => format!(
                    "Saved validated catalog to {} and graph layout to {}",
                    catalog_path.display(),
                    layout_path.display()
                ),
                Err(error) => format!("Could not save technology authoring assets: {error:#}"),
            };
        }
        if ui.button("Auto layout").clicked() {
            let previous = authoring_snapshot(state);
            state.technology_layout = TechnologyGraphLayout::automatic(&state.catalog.technology);
            push_authoring_undo(state, previous);
            state.technology_graph_view.request_fit();
            "Applied deterministic graph layout; use Undo to restore authored positions"
                .clone_into(&mut state.status);
        }
        if ui.button("Fit all").clicked() {
            state.technology_graph_view.request_fit();
        }
        if ui
            .add_enabled(
                state.technology_draft.is_some(),
                egui::Button::new("Focus selection"),
            )
            .clicked()
            && let Some(id) = state
                .technology_draft
                .as_ref()
                .map(|draft| draft.id.clone())
        {
            state.technology_graph_view.request_focus(id);
        }
        ui.checkbox(&mut state.technology_graph_view.show_minimap, "Minimap");
        ui.label("Search");
        ui.text_edit_singleline(&mut state.technology_search);
    });
    ui.horizontal_wrapped(|ui| {
        ui.label("Catalog path");
        ui.text_edit_singleline(&mut state.catalog_path);
    });
    ui.horizontal_wrapped(|ui| {
        ui.label("Graph layout path");
        ui.text_edit_singleline(&mut state.technology_layout_path);
        if ui.button("Reload both").clicked() {
            state.status = match reload_content_catalog(state) {
                Ok(()) => "Reloaded and validated content catalog and graph layout".to_owned(),
                Err(error) => format!("Could not reload technology authoring assets: {error:#}"),
            };
        }
    });

    let groups: Vec<_> = state
        .catalog
        .technology
        .groups
        .iter()
        .map(|(id, group)| (id.clone(), group.display_name.clone()))
        .collect();
    let selected_label = state
        .selected_group
        .as_ref()
        .and_then(|selected| {
            groups
                .iter()
                .find(|(id, _)| id == selected)
                .map(|(_, name)| name.as_str())
        })
        .unwrap_or("Select group");
    egui::ComboBox::from_label("Group")
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(selected_label)
        .show_ui(ui, |ui| {
            for (id, name) in &groups {
                ui.selectable_value(&mut state.selected_group, Some(id.clone()), name);
            }
        });
    ui.horizontal_wrapped(|ui| {
        ui.label("New group ID");
        ui.text_edit_singleline(&mut state.new_group_id);
        ui.label("Name");
        ui.text_edit_singleline(&mut state.new_group_name);
        if ui.button("Create group").clicked() {
            state.status = match create_technology_group(state) {
                Ok(()) => "Created technology group".to_owned(),
                Err(error) => format!("Group creation rejected: {error}"),
            };
        }
        if ui
            .add_enabled(
                state.selected_group.is_some(),
                egui::Button::new("Delete group"),
            )
            .clicked()
        {
            state.status = match delete_selected_technology_group(state) {
                Ok(()) => "Deleted empty technology group".to_owned(),
                Err(error) => format!("Group deletion rejected: {error}"),
            };
        }
    });
    let selected_node = state
        .technology_draft
        .as_ref()
        .map(|draft| draft.id.clone());
    let layout_before = state.technology_layout.clone();
    let graph_output = show_technology_graph(
        ui,
        &state.catalog,
        &mut state.technology_layout,
        &mut state.technology_graph_view,
        selected_node.as_ref(),
        state.selected_group.as_ref(),
        &state.technology_search,
    );
    if graph_output.layout_edit_started {
        let previous = AuthoringSnapshot {
            catalog: state.catalog.clone(),
            technology_layout: layout_before,
        };
        push_authoring_undo(state, previous);
    }
    if let Some(group) = graph_output.selected_group {
        state.selected_group = Some(group);
    }
    if let Some(node) = graph_output.selected_node {
        state.technology_draft = technology_draft(&state.catalog, &node);
    }

    let search = state.technology_search.to_ascii_lowercase();
    let node_choices: Vec<_> = state
        .catalog
        .technology
        .nodes
        .iter()
        .filter_map(|(id, node)| {
            let matches = search.is_empty()
                || node.display_name.to_ascii_lowercase().contains(&search)
                || id.as_str().contains(&search);
            matches.then(|| (id.clone(), node.display_name.clone()))
        })
        .collect();
    let mut selected = None;
    egui::ScrollArea::vertical()
        .max_height(170.0)
        .show(ui, |ui| {
            for (id, name) in &node_choices {
                let is_selected = state
                    .technology_draft
                    .as_ref()
                    .is_some_and(|draft| draft.id == *id);
                if ui
                    .selectable_label(is_selected, format!("{name}  ({id})"))
                    .clicked()
                {
                    selected = Some(id.clone());
                }
            }
        });
    if let Some(selected) = selected {
        state.technology_draft = technology_draft(&state.catalog, &selected);
        state.selected_group = state
            .catalog
            .technology
            .nodes
            .get(&selected)
            .and_then(|node| node.group.clone());
        state.technology_graph_view.request_focus(selected);
    }

    ui.horizontal_wrapped(|ui| {
        ui.label("New node ID");
        ui.text_edit_singleline(&mut state.new_technology_id);
        ui.label("Name");
        ui.text_edit_singleline(&mut state.new_technology_name);
        if ui
            .add_enabled(
                state.selected_group.is_some(),
                egui::Button::new("Create node"),
            )
            .clicked()
        {
            state.status = match create_technology_node(state) {
                Ok(()) => "Created technology node".to_owned(),
                Err(error) => format!("Node creation rejected: {error}"),
            };
        }
        if ui
            .add_enabled(
                state.technology_draft.is_some(),
                egui::Button::new("Delete node"),
            )
            .clicked()
        {
            state.status = match delete_selected_technology_node(state) {
                Ok(()) => "Deleted technology node and cleaned references".to_owned(),
                Err(error) => format!("Node deletion rejected: {error}"),
            };
        }
    });

    let mut apply = false;
    if let Some(draft) = state.technology_draft.as_mut() {
        ui.separator();
        ui.monospace(draft.id.to_string());
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.text_edit_singleline(&mut draft.display_name);
            ui.label("Age");
            ui.text_edit_singleline(&mut draft.age);
            ui.add(egui::DragValue::new(&mut draft.tier).prefix("Tier "));
        });
        egui::ComboBox::from_label("Node group")
            .height(AUTHORING_COMBO_MAX_HEIGHT)
            .selected_text(
                draft
                    .group
                    .as_ref()
                    .and_then(|id| state.catalog.technology.groups.get(id))
                    .map_or("Ungrouped", |group| group.display_name.as_str()),
            )
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut draft.group, None, "Ungrouped");
                for (id, group) in &state.catalog.technology.groups {
                    ui.selectable_value(&mut draft.group, Some(id.clone()), &group.display_name);
                }
            });
        ui.label("Description");
        ui.text_edit_multiline(&mut draft.description);
        if let Some(node) = state.catalog.technology.nodes.get(&draft.id) {
            ui.label(format!("Authored objectives ({})", node.objectives.len()));
            for objective_id in &node.objectives {
                if let Some(objective) = state.catalog.objectives.get(objective_id) {
                    ui.monospace(format!(
                        "{objective_id}: {:?} 0/{}{}{}{}",
                        objective.kind,
                        objective.required_amount,
                        objective
                            .resource
                            .as_ref()
                            .map_or_else(String::new, |id| format!(" resource={id}")),
                        objective
                            .building
                            .as_ref()
                            .map_or_else(String::new, |id| format!(" building={id}")),
                        objective
                            .enemy
                            .as_ref()
                            .map_or_else(String::new, |id| format!(" enemy={id}")),
                    ));
                }
            }
        }
        ui.label("Prerequisite stable IDs (comma separated)");
        ui.text_edit_singleline(&mut draft.prerequisites);
        ui.label("Explicit unlock stable IDs (comma separated)");
        ui.text_edit_singleline(&mut draft.unlocks);
        ui.label("Objective stable IDs (comma separated)");
        ui.text_edit_singleline(&mut draft.objectives);
        ui.horizontal(|ui| {
            ui.label("Icon asset path");
            ui.text_edit_singleline(&mut draft.icon_path);
        });
        ui.horizontal(|ui| {
            ui.checkbox(&mut draft.initially_unlocked, "Initially unlocked");
            ui.checkbox(&mut draft.unavailable, "Unavailable");
            apply = ui.button("Apply validated edit").clicked();
        });
    } else {
        ui.label("Select a technology node to edit its metadata and references.");
    }
    if apply {
        state.status = match apply_technology_draft(state) {
            Ok(()) => "Technology edit applied; graph remains valid".to_owned(),
            Err(error) => format!("Technology edit rejected: {error}"),
        };
    }
}

fn world_tab(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    ui.heading("World-generation lab");
    ui.label(
        "Tune Unity-compatible terrain and foliage generation, then inspect deterministic elevation, occupancy, resource, and foliage previews.",
    );
    ui.horizontal_wrapped(|ui| {
        ui.add(egui::DragValue::new(&mut state.config.world.seed).prefix("Seed "));
        ui.add(egui::Slider::new(&mut state.config.world.width, 8..=512).text("Width"));
        ui.add(egui::Slider::new(&mut state.config.world.height, 8..=512).text("Height"));
        ui.add(
            egui::DragValue::new(&mut state.config.world.cell_size)
                .range(0.1..=100.0)
                .speed(0.1)
                .prefix("Cell size "),
        );
        ui.add(
            egui::DragValue::new(&mut state.config.world.height_scale_centimetres)
                .range(1..=10_000)
                .prefix("Height scale cm "),
        );
        ui.add(
            egui::DragValue::new(&mut state.config.world.water_level_centimetres)
                .prefix("Water level cm "),
        );
        ui.add(
            egui::Slider::new(
                &mut state.config.world.resource_density_per_thousand,
                0..=1_000,
            )
            .text("Resource density / 1000"),
        );
    });
    ui.horizontal_wrapped(|ui| {
        if ui.button("Save + apply world config").clicked() {
            state.status = match save_and_apply_game_config(&state.config, &state.config_path) {
                Ok((project, runtime)) => format!(
                    "Saved {} and applied {} while preserving local Twitch setup; restart the game to load the changes",
                    project.display(),
                    runtime.display()
                ),
                Err(error) => format!("Could not save and apply game configuration: {error:#}"),
            };
        }
        if ui.button("Save foliage catalog").clicked() {
            state.status = match save_content_catalog(&state.catalog, &state.catalog_path) {
                Ok(path) => format!("Saved authored foliage to {}", path.display()),
                Err(error) => format!("Could not save foliage catalog: {error:#}"),
            };
        }
        if ui
            .add_enabled(
                !state.undo_authoring.is_empty(),
                egui::Button::new("Undo content edit"),
            )
            .clicked()
        {
            undo_authoring_edit(state);
        }
        if ui
            .add_enabled(
                !state.redo_authoring.is_empty(),
                egui::Button::new("Redo content edit"),
            )
            .clicked()
        {
            redo_authoring_edit(state);
        }
    });

    let resource_choices = resource_choices(&state.catalog);
    let target_choices = target_kind_choices(&state.catalog);
    let discovered_models = state.discovered_model_assets.clone();
    ui.collapsing("Land and water resource generation layers", |ui| {
        let layers: Vec<_> = state
            .catalog
            .resource_generation
            .iter()
            .map(|layer| (layer.id.clone(), layer.display_name.clone()))
            .collect();
        let mut changed = false;
        stable_id_option_choice(
            ui,
            "Resource layer",
            &mut state.selected_resource_generation,
            &layers,
        );
        if state
            .resource_generation_draft
            .as_ref()
            .map(|draft| &draft.id)
            != state.selected_resource_generation.as_ref()
        {
            changed = true;
        }
        if changed {
            refresh_resource_generation_draft(state);
        }
        ui.horizontal_wrapped(|ui| {
            ui.label("New layer ID");
            ui.text_edit_singleline(&mut state.new_resource_generation_id);
            ui.label("Name");
            ui.text_edit_singleline(&mut state.new_resource_generation_name);
            if ui
                .add_enabled(
                    state.selected_resource_generation.is_some(),
                    egui::Button::new("Duplicate selected layer"),
                )
                .clicked()
            {
                state.status = match duplicate_resource_generation_layer(state) {
                    Ok(()) => "Duplicated resource generation layer".to_owned(),
                    Err(error) => format!("Resource layer creation rejected: {error}"),
                };
            }
            if ui
                .add_enabled(
                    state.selected_resource_generation.is_some(),
                    egui::Button::new("Remove selected layer"),
                )
                .clicked()
            {
                state.status = match delete_resource_generation_layer(state) {
                    Ok(()) => "Removed resource generation layer".to_owned(),
                    Err(error) => format!("Resource layer removal rejected: {error}"),
                };
            }
        });
        let mut apply = false;
        let mut discard = false;
        if let Some(layer) = state.resource_generation_draft.as_mut() {
            ui.group(|ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.monospace(layer.id.to_string());
                    ui.label("Display name");
                    ui.text_edit_singleline(&mut layer.display_name);
                    egui::ComboBox::from_id_salt("resource_generation_habitat")
                        .height(AUTHORING_COMBO_MAX_HEIGHT)
                        .selected_text(format!("{:?}", layer.habitat))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut layer.habitat,
                                ResourceGenerationHabitat::Land,
                                "Land",
                            );
                            ui.selectable_value(
                                &mut layer.habitat,
                                ResourceGenerationHabitat::ShorelineWater,
                                "Shoreline water",
                            );
                        });
                });
                stable_id_required_choice(
                    ui,
                    "Produced resource",
                    &mut layer.resource,
                    &resource_choices,
                );
                stable_id_required_choice(
                    ui,
                    "Target kind",
                    &mut layer.target_kind,
                    &target_choices,
                );
                searchable_string_choice(
                    ui,
                    "resource_visual_asset",
                    "Preview/model GLB",
                    &mut layer.visual_asset_path,
                    &discovered_models,
                    &mut state.world_asset_search,
                );
                ui.horizontal_wrapped(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut layer.amount)
                            .range(1..=u32::MAX)
                            .prefix("Amount "),
                    );
                    ui.add(egui::DragValue::new(&mut layer.seed).prefix("Seed "));
                    ui.add(
                        egui::DragValue::new(&mut layer.density_multiplier_per_thousand)
                            .range(1..=10_000)
                            .prefix("Water density multiplier /1000 "),
                    );
                });
                if layer.habitat == ResourceGenerationHabitat::Land {
                    ui.horizontal_wrapped(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut layer.source_size)
                                .range(1..=4_096)
                                .prefix("Source size "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut layer.noise_scale)
                                .range(0.001..=10_000.0)
                                .prefix("Noise scale "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut layer.octaves)
                                .range(1..=8)
                                .prefix("Octaves "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut layer.persistence)
                                .range(0.0..=1.0)
                                .prefix("Persistence "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut layer.lacunarity)
                                .range(0.0..=32.0)
                                .prefix("Lacunarity "),
                        );
                        ui.add(
                            egui::Slider::new(&mut layer.spawn_threshold, 0.0..=1.0)
                                .text("Threshold"),
                        );
                        ui.add(
                            egui::DragValue::new(&mut layer.spacing)
                                .range(1..=u16::MAX)
                                .prefix("Spacing "),
                        );
                        ui.checkbox(
                            &mut layer.half_cell_terrain_offset,
                            "Half-cell terrain offset",
                        );
                    });
                }
                ui.label(format!("Unity provenance: {}", layer.source_path));
                draw_model_preview(
                    ui,
                    preview_texture,
                    preview_status,
                    egui::vec2(360.0, 240.0),
                    preview_controls,
                    false,
                );
                ui.horizontal(|ui| {
                    apply = ui.button("Apply validated resource layer").clicked();
                    discard = ui.button("Discard resource draft").clicked();
                });
            });
        }
        if apply {
            state.status = match apply_resource_generation_draft(state) {
                Ok(()) => "Resource generation layer applied and validated".to_owned(),
                Err(error) => format!("Resource generation edit rejected: {error}"),
            };
            state.generated_world = None;
        } else if discard {
            refresh_resource_generation_draft(state);
        }
    });

    enemy_camp_generation_editor(ui, state, preview_texture, preview_status, preview_controls);

    ui.collapsing("Authored foliage generation layers", |ui| {
        let layers: Vec<_> = state
            .catalog
            .foliage
            .iter()
            .map(|layer| (layer.id.clone(), layer.source_path.clone()))
            .collect();
        let selected_label = state
            .selected_foliage
            .as_ref()
            .map_or("Select layer", StableId::as_str);
        let mut changed = false;
        egui::ComboBox::from_label("Layer")
            .height(AUTHORING_COMBO_MAX_HEIGHT)
            .selected_text(selected_label)
            .show_ui(ui, |ui| {
                for (id, source) in &layers {
                    changed |= ui
                        .selectable_value(
                            &mut state.selected_foliage,
                            Some(id.clone()),
                            format!("{id} — {source}"),
                        )
                        .changed();
                }
            });
        if changed {
            refresh_foliage_draft(state);
        }
        ui.horizontal_wrapped(|ui| {
            ui.label("New layer ID");
            ui.text_edit_singleline(&mut state.new_foliage_id);
            if ui
                .add_enabled(
                    state.selected_foliage.is_some(),
                    egui::Button::new("Duplicate selected foliage layer"),
                )
                .clicked()
            {
                state.status = match duplicate_foliage_layer(state) {
                    Ok(()) => "Duplicated foliage generation layer".to_owned(),
                    Err(error) => format!("Foliage layer creation rejected: {error}"),
                };
            }
            if ui
                .add_enabled(
                    state.selected_foliage.is_some(),
                    egui::Button::new("Remove selected foliage layer"),
                )
                .clicked()
            {
                state.status = match delete_foliage_layer(state) {
                    Ok(()) => "Removed foliage generation layer".to_owned(),
                    Err(error) => format!("Foliage layer removal rejected: {error}"),
                };
            }
        });
        let source_paths: Vec<_> = state
            .catalog
            .source_records
            .values()
            .map(|record| record.source_path.clone())
            .chain(
                state
                    .catalog
                    .foliage
                    .iter()
                    .map(|layer| layer.source_path.clone()),
            )
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let material_paths: Vec<_> = state
            .presentation
            .materials
            .values()
            .map(|material| material.source_path.clone())
            .chain(
                state
                    .catalog
                    .foliage
                    .iter()
                    .map(|layer| layer.material_source_path.clone()),
            )
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let asset_source_models: BTreeMap<_, _> = state
            .catalog
            .archetypes
            .values()
            .flat_map(|archetype| archetype.scenes.iter())
            .map(|scene| (scene.asset_path.clone(), scene.source_model.clone()))
            .collect();
        let mut apply_foliage = false;
        let mut reset_foliage = false;
        if let Some(layer) = state.foliage_draft.as_mut() {
            ui.horizontal_wrapped(|ui| {
                ui.monospace(layer.id.to_string());
                egui::ComboBox::from_label("Habitat")
                    .height(AUTHORING_COMBO_MAX_HEIGHT)
                    .selected_text(format!("{:?}", layer.habitat))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut layer.habitat, FoliageHabitat::Land, "Land");
                        ui.selectable_value(
                            &mut layer.habitat,
                            FoliageHabitat::Underwater,
                            "Underwater",
                        );
                    });
                ui.add(
                    egui::DragValue::new(&mut layer.source_size)
                        .range(1..=4_096)
                        .prefix("Source size "),
                );
                ui.add(
                    egui::DragValue::new(&mut layer.level_of_detail)
                        .range(0..=6)
                        .prefix("LOD "),
                );
            });
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    egui::DragValue::new(&mut layer.noise_scale)
                        .range(0.001..=10_000.0)
                        .speed(0.1)
                        .prefix("Noise scale "),
                );
                ui.add(
                    egui::DragValue::new(&mut layer.octaves)
                        .range(1..=8)
                        .prefix("Octaves "),
                );
                ui.add(
                    egui::DragValue::new(&mut layer.persistence)
                        .range(0.0..=1.0)
                        .speed(0.01)
                        .prefix("Persistence "),
                );
                ui.add(
                    egui::DragValue::new(&mut layer.lacunarity)
                        .range(0.001..=16.0)
                        .speed(0.05)
                        .prefix("Lacunarity "),
                );
                ui.add(egui::DragValue::new(&mut layer.seed).prefix("Layer seed "));
            });
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    egui::DragValue::new(&mut layer.offset[0])
                        .speed(0.1)
                        .prefix("Offset x "),
                );
                ui.add(
                    egui::DragValue::new(&mut layer.offset[1])
                        .speed(0.1)
                        .prefix("z "),
                );
                ui.add(
                    egui::Slider::new(&mut layer.spawn_threshold, 0.0..=1.0)
                        .text("Spawn threshold"),
                );
                ui.add(
                    egui::DragValue::new(&mut layer.spacing)
                        .range(1..=u16::MAX)
                        .prefix("Spacing "),
                );
            });
            searchable_string_choice(
                ui,
                "foliage_source",
                "Unity source",
                &mut layer.source_path,
                &source_paths,
                &mut state.world_asset_search,
            );
            searchable_string_choice(
                ui,
                "foliage_material",
                "Material source",
                &mut layer.material_source_path,
                &material_paths,
                &mut state.world_asset_search,
            );
            ui.collapsing(format!("Model variants ({})", layer.variants.len()), |ui| {
                let mut remove_variant = None;
                for (index, variant) in layer.variants.iter_mut().enumerate() {
                    ui.push_id(("foliage_variant", index), |ui| {
                        ui.group(|ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.strong(format!("Variant {}", index + 1));
                                ui.selectable_value(
                                    &mut state.selected_foliage_variant,
                                    index,
                                    "Preview",
                                );
                                if ui.small_button("Remove").clicked() {
                                    remove_variant = Some(index);
                                }
                            });
                            if searchable_string_choice(
                                ui,
                                "foliage_variant_asset",
                                "GLB asset",
                                &mut variant.asset_path,
                                &discovered_models,
                                &mut state.world_asset_search,
                            ) {
                                variant.source_model = asset_source_models
                                    .get(&variant.asset_path)
                                    .cloned()
                                    .unwrap_or_else(|| variant.asset_path.clone());
                            }
                            ui.monospace(format!("Source: {}", variant.source_model));
                            ui.horizontal(|ui| {
                                ui.label("Base scale");
                                for axis in &mut variant.base_scale {
                                    ui.add(
                                        egui::DragValue::new(axis)
                                            .range(0.001..=1_000.0)
                                            .speed(0.01),
                                    );
                                }
                            });
                        });
                    });
                }
                if let Some(index) = remove_variant {
                    if layer.variants.len() > 1 {
                        layer.variants.remove(index);
                        state.selected_foliage_variant = state
                            .selected_foliage_variant
                            .min(layer.variants.len().saturating_sub(1));
                    } else {
                        "A foliage layer must retain at least one model variant"
                            .clone_into(&mut state.status);
                    }
                }
                if ui.button("Add model variant").clicked()
                    && let Some(asset_path) = discovered_models.first()
                {
                    layer.variants.push(stream_town_domain::FoliageVariantDef {
                        source_model: asset_source_models
                            .get(asset_path)
                            .cloned()
                            .unwrap_or_else(|| asset_path.clone()),
                        asset_path: asset_path.clone(),
                        base_scale: [1.0, 1.0, 1.0],
                    });
                    state.selected_foliage_variant = layer.variants.len() - 1;
                }
            });
            draw_model_preview(
                ui,
                preview_texture,
                preview_status,
                egui::vec2(360.0, 240.0),
                preview_controls,
                false,
            );
            ui.horizontal(|ui| {
                apply_foliage = ui.button("Apply validated foliage edit").clicked();
                reset_foliage = ui.button("Discard foliage draft").clicked();
            });
        }
        if apply_foliage {
            state.status = match apply_foliage_draft(state) {
                Ok(()) => "Foliage generation edit applied and validated".to_owned(),
                Err(error) => format!("Foliage edit rejected: {error}"),
            };
            state.generated_world = None;
        } else if reset_foliage {
            refresh_foliage_draft(state);
            "Discarded foliage draft".clone_into(&mut state.status);
        }
    });

    ui.separator();
    ui.horizontal_wrapped(|ui| {
        if ui.button("Generate deterministic preview").clicked() {
            state.status = match (state.config.validate(), state.catalog.validate()) {
                (Ok(()), Ok(())) => {
                    let world = stream_town_domain::generate_world_with_content(
                        &state.config.world,
                        &state.catalog,
                    );
                    let status = format!(
                        "Generated {}x{} world with {} resources and {} foliage instances; hash {}",
                        world.navigation.width(),
                        world.navigation.height(),
                        world.resources.len(),
                        world.foliage.len(),
                        &world.deterministic_hash[..16]
                    );
                    state.generated_world = Some(world);
                    status
                }
                (Err(error), _) => format!("World configuration is invalid: {error}"),
                (_, Err(error)) => format!("Foliage catalog is invalid: {error}"),
            };
        }
        egui::ComboBox::from_label("Preview")
            .height(AUTHORING_COMBO_MAX_HEIGHT)
            .selected_text(state.world_preview_layer.label())
            .show_ui(ui, |ui| {
                for layer in WorldPreviewLayer::ALL {
                    ui.selectable_value(&mut state.world_preview_layer, layer, layer.label());
                }
            });
    });
    if let Some(world) = &state.generated_world {
        let walkable = (0..world.navigation.height())
            .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
            .filter(|position| world.navigation.is_walkable(*position))
            .count();
        ui.monospace(format!("Hash: {}", world.deterministic_hash));
        ui.label(format!(
            "Generator v{} · {} walkable / {} total cells · {} resources · {} foliage",
            world.generator_version,
            walkable,
            usize::from(world.navigation.width()) * usize::from(world.navigation.height()),
            world.resources.len(),
            world.foliage.len(),
        ));
        draw_world_preview(
            ui,
            world,
            state.world_preview_layer,
            state.selected_foliage.as_ref(),
            state
                .resource_generation_draft
                .as_ref()
                .map(|layer| &layer.target_kind),
            state.enemy_camp_generation_draft.as_ref(),
            state.config.world.water_level_centimetres,
        );
    }
}

#[allow(dead_code)]
fn settings_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Player settings");
    ui.label("Unity SettingsData parity with validated, atomic RON persistence.");
    let settings_path = stream_town_game::player_settings_path();
    ui.horizontal_wrapped(|ui| {
        ui.monospace(settings_path.display().to_string());
        if ui.button("Reload saved settings").clicked() {
            let store = PlayerSettingsStore::new(settings_path.clone());
            state.status = match store.load() {
                Ok(settings) => {
                    state.player_settings = settings;
                    "Reloaded and validated player settings".to_owned()
                }
                Err(error) => format!("Could not reload player settings: {error}"),
            };
        }
        if ui.button("Validate draft").clicked() {
            state.status = match state.player_settings.validate() {
                Ok(()) => "Player settings are valid".to_owned(),
                Err(error) => format!("Player settings error: {error}"),
            };
        }
    });
    ui.horizontal(|ui| {
        ui.label("Display mode");
        for (mode, label) in [
            (DisplayMode::Windowed, "Windowed"),
            (DisplayMode::Borderless, "Borderless"),
            (DisplayMode::Fullscreen, "Fullscreen"),
        ] {
            ui.selectable_value(&mut state.player_settings.video.display_mode, mode, label);
        }
    });
    ui.horizontal(|ui| {
        ui.add(
            egui::DragValue::new(&mut state.player_settings.video.width)
                .range(640..=16_384)
                .prefix("Width "),
        );
        ui.add(
            egui::DragValue::new(&mut state.player_settings.video.height)
                .range(480..=8_640)
                .prefix("Height "),
        );
    });
    ui.horizontal(|ui| {
        ui.checkbox(&mut state.player_settings.video.vsync, "VSync");
        ui.checkbox(&mut state.player_settings.video.shadows_enabled, "Shadows");
        ui.checkbox(
            &mut state.player_settings.video.ambient_occlusion,
            "Ambient occlusion",
        );
    });
    ui.horizontal(|ui| {
        egui::ComboBox::from_label("MSAA")
            .selected_text(format!("{}x", state.player_settings.video.msaa_samples))
            .show_ui(ui, |ui| {
                for samples in [1, 2, 4, 8] {
                    ui.selectable_value(
                        &mut state.player_settings.video.msaa_samples,
                        samples,
                        format!("{samples}x"),
                    );
                }
            });
        egui::ComboBox::from_label("Post AA")
            .selected_text(format!("{:?}", state.player_settings.video.post_process_aa))
            .show_ui(ui, |ui| {
                for mode in [
                    PostProcessAntiAliasing::None,
                    PostProcessAntiAliasing::Fxaa,
                    PostProcessAntiAliasing::Smaa,
                ] {
                    ui.selectable_value(
                        &mut state.player_settings.video.post_process_aa,
                        mode,
                        format!("{mode:?}"),
                    );
                }
            });
        egui::ComboBox::from_label("Shadow map")
            .selected_text(
                state
                    .player_settings
                    .video
                    .shadow_map_resolution
                    .to_string(),
            )
            .show_ui(ui, |ui| {
                for resolution in [256, 512, 1_024, 2_048, 4_096] {
                    ui.selectable_value(
                        &mut state.player_settings.video.shadow_map_resolution,
                        resolution,
                        resolution.to_string(),
                    );
                }
            });
        let mut fps = state.player_settings.video.fps_limit.unwrap_or_default();
        egui::ComboBox::from_label("FPS limit")
            .selected_text(if fps == 0 {
                "Unlimited".to_owned()
            } else {
                fps.to_string()
            })
            .show_ui(ui, |ui| {
                for (value, label) in [
                    (0, "Unlimited"),
                    (24, "24"),
                    (30, "30"),
                    (60, "60"),
                    (120, "120"),
                    (240, "240"),
                ] {
                    ui.selectable_value(&mut fps, value, label);
                }
            });
        state.player_settings.video.fps_limit = (fps != 0).then_some(fps);
    });
    ui.horizontal(|ui| {
        ui.add(
            egui::Slider::new(&mut state.player_settings.video.brightness_ev, -5.0..=5.0)
                .text("Brightness EV"),
        );
        ui.add(egui::Slider::new(&mut state.player_settings.video.gamma, -5.0..=5.0).text("Gamma"));
    });
    ui.separator();
    ui.label("Audio mix");
    ui.add(egui::Slider::new(&mut state.player_settings.audio.master, 0.0..=1.0).text("Master"));
    ui.add(egui::Slider::new(&mut state.player_settings.audio.music, 0.0..=1.0).text("Music"));
    ui.add(
        egui::Slider::new(&mut state.player_settings.audio.sound_effects, 0.0..=1.0)
            .text("Sound effects"),
    );
    ui.add(
        egui::Slider::new(&mut state.player_settings.audio.ambience, 0.0..=1.0).text("Ambience"),
    );
    ui.separator();
    ui.label("Camera");
    ui.add(
        egui::Slider::new(
            &mut state.player_settings.camera.zoom_sensitivity,
            0.0..=100.0,
        )
        .text("Zoom sensitivity"),
    );
    ui.add(
        egui::Slider::new(
            &mut state.player_settings.camera.field_of_view_degrees,
            30..=120,
        )
        .text("Field of view"),
    );
    egui::ComboBox::from_label("Autosave")
        .selected_text(if state.player_settings.autosave_minutes == 0 {
            "Off".to_owned()
        } else {
            format!("{} minutes", state.player_settings.autosave_minutes)
        })
        .show_ui(ui, |ui| {
            for (minutes, label) in [
                (0, "Off"),
                (1, "1 minute"),
                (5, "5 minutes"),
                (10, "10 minutes"),
                (30, "30 minutes"),
                (60, "60 minutes"),
            ] {
                ui.selectable_value(&mut state.player_settings.autosave_minutes, minutes, label);
            }
        });
    ui.horizontal(|ui| {
        egui::ComboBox::from_label("Names")
            .selected_text(format!(
                "{:?}",
                state.player_settings.interface.display_names
            ))
            .show_ui(ui, |ui| {
                for value in [
                    NameDisplayMode::None,
                    NameDisplayMode::StaffAndSubscribers,
                    NameDisplayMode::AllPlayers,
                ] {
                    ui.selectable_value(
                        &mut state.player_settings.interface.display_names,
                        value,
                        format!("{value:?}"),
                    );
                }
            });
        egui::ComboBox::from_label("Building health")
            .selected_text(format!(
                "{:?}",
                state.player_settings.interface.display_building_health
            ))
            .show_ui(ui, |ui| {
                for value in [
                    BuildingHealthDisplayMode::None,
                    BuildingHealthDisplayMode::DamagedOnly,
                    BuildingHealthDisplayMode::Always,
                ] {
                    ui.selectable_value(
                        &mut state.player_settings.interface.display_building_health,
                        value,
                        format!("{value:?}"),
                    );
                }
            });
    });
    ui.separator();
    ui.heading("Accessibility");
    ui.horizontal(|ui| {
        egui::ComboBox::from_label("UI scale")
            .selected_text(format!(
                "{}%",
                state.player_settings.interface.ui_scale_percent
            ))
            .show_ui(ui, |ui| {
                for percent in [75, 90, 100, 110, 125, 150] {
                    ui.selectable_value(
                        &mut state.player_settings.interface.ui_scale_percent,
                        percent,
                        format!("{percent}%"),
                    );
                }
            });
        ui.checkbox(
            &mut state.player_settings.interface.high_contrast,
            "High contrast",
        );
        ui.checkbox(
            &mut state.player_settings.interface.reduced_motion,
            "Reduced motion",
        );
    });
    ui.horizontal(|ui| {
        if ui.button("Save validated settings").clicked() {
            let store = PlayerSettingsStore::new(settings_path.clone());
            state.status = match store.write(&state.player_settings) {
                Ok(()) => format!("Saved player settings to {}", store.path().display()),
                Err(error) => format!("Could not save player settings: {error}"),
            };
        }
        if ui.button("Restore Unity defaults").clicked() {
            state.player_settings = PlayerSettings::default();
            "Restored Unity-equivalent defaults; save to persist them"
                .clone_into(&mut state.status);
        }
    });
    ui.label("Restart the game after saving to apply window, renderer, and audio changes.");
}

#[allow(dead_code)]
fn runtime_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Runtime developer console");
    let attached = runtime_console_attached(state.runtime_status.as_ref());
    ui.horizontal_wrapped(|ui| {
        ui.label(if attached {
            "Connected"
        } else {
            "Not attached"
        });
        ui.separator();
        ui.checkbox(&mut state.runtime_profile_launch, "Profile launch");
        if ui.button("Launch game").clicked() {
            state.status = match launch_runtime_game(state) {
                Ok(()) => "Launched the Bevy game and waiting for runtime status".to_owned(),
                Err(error) => format!("Could not launch game: {error:#}"),
            };
        }
        if ui.button("Refresh/attach").clicked() {
            poll_runtime_console(state);
            state.status = if runtime_console_attached(state.runtime_status.as_ref()) {
                "Attached to the local Bevy runtime".to_owned()
            } else {
                "No live local runtime status was found".to_owned()
            };
        }
    });
    ui.label(format!(
        "Control channel: {}",
        state.runtime_console.root().display()
    ));
    if let Some(status) = &state.runtime_status {
        egui::Grid::new("runtime_status")
            .num_columns(4)
            .show(ui, |ui| {
                ui.label("PID");
                ui.monospace(status.process_id.to_string());
                ui.label("State");
                ui.monospace(&status.state);
                ui.end_row();
                ui.label("World");
                ui.monospace(status.world_hash.as_deref().unwrap_or("Not loaded"));
                ui.label("Seed");
                ui.monospace(
                    status
                        .world_seed
                        .map_or_else(|| "-".to_owned(), |seed| seed.to_string()),
                );
                ui.end_row();
                ui.label("Actors / buildings");
                ui.monospace(format!(
                    "{} / {}",
                    status.actor_count, status.building_count
                ));
                ui.label("Elapsed / routes");
                ui.monospace(format!(
                    "{:.1}s / {}",
                    status.elapsed_seconds, status.paths_completed
                ));
                ui.end_row();
                ui.label("Frame avg / p95");
                ui.monospace(format_runtime_frame_times(status));
                ui.label("Commands");
                ui.monospace(status.commands_processed.to_string());
                ui.end_row();
                ui.label("Terrain H / M / L");
                ui.monospace(format!(
                    "{} / {} / {}",
                    status.terrain_high_chunks,
                    status.terrain_medium_chunks,
                    status.terrain_low_chunks
                ));
                ui.label("Foliage active / total");
                ui.monospace(format!(
                    "{} / {}",
                    status.foliage_visible_instances, status.foliage_instances
                ));
                ui.end_row();
                ui.label("Foliage GPU / spatial batches");
                ui.monospace(format!(
                    "{} / {}",
                    status.foliage_batches, status.foliage_spatial_groups
                ));
                ui.label("Fallback / separated / yielding");
                ui.monospace(format!(
                    "{} / {} / {}",
                    status.foliage_unbatched_instances,
                    status.crowd_adjusted_agents,
                    status.crowd_yielding_agents
                ));
                ui.end_row();
                ui.label("Twitch");
                ui.monospace(&status.twitch_status);
                ui.label("Direct broadcast");
                ui.monospace(&status.direct_broadcast_status);
                ui.end_row();
                ui.label("Save");
                ui.monospace(if status.save_exists {
                    "Available"
                } else {
                    "Missing"
                });
                ui.label("");
                ui.label("");
                ui.end_row();
            });
        ui.label(format!("Last result: {}", status.last_result));
        if !status.town_resources.is_empty() {
            ui.label(format!(
                "Resources: {}",
                status
                    .town_resources
                    .iter()
                    .map(|(id, amount)| format!("{id}={amount}"))
                    .collect::<Vec<_>>()
                    .join("  ")
            ));
        }
    }
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Stable actor ID");
        ui.text_edit_singleline(&mut state.runtime_actor_id);
        ui.label("Login");
        ui.text_edit_singleline(&mut state.runtime_login);
    });
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut state.command);
        if ui
            .add_enabled(attached, egui::Button::new("Inject command"))
            .clicked()
        {
            state.status = match inject_runtime_command(state) {
                Ok(()) => format!("Injected {} through the local debug channel", state.command),
                Err(error) => format!("Could not inject command: {error:#}"),
            };
        }
    });
    ui.horizontal(|ui| {
        if ui.button("Prepare Fish God start").clicked() {
            "!event fish_god".clone_into(&mut state.command);
        }
        if ui.button("Prepare praise").clicked() {
            "!praise".clone_into(&mut state.command);
        }
        if ui.button("Prepare raid").clicked() {
            "!event raid".clone_into(&mut state.command);
        }
        if ui.button("Prepare ruler vote").clicked() {
            "!rulervote".clone_into(&mut state.command);
        }
        if ui.button("Prepare recruit").clicked() {
            "!recruit miner 1".clone_into(&mut state.command);
        }
    });
    ui.horizontal_wrapped(|ui| {
        for (label, action) in [
            ("Save", RuntimeConsoleAction::Save),
            ("Save Jump-Start", RuntimeConsoleAction::SaveJumpStart),
            ("Load", RuntimeConsoleAction::Load),
            ("Capture frame", RuntimeConsoleAction::CaptureFrame),
            ("Main Menu", RuntimeConsoleAction::ReturnToMainMenu),
            ("Exit game", RuntimeConsoleAction::Exit),
        ] {
            if ui.add_enabled(attached, egui::Button::new(label)).clicked() {
                state.status = match send_runtime_action(state, action) {
                    Ok(()) => format!("Sent runtime action: {label}"),
                    Err(error) => format!("Could not send runtime action: {error:#}"),
                };
            }
        }
    });
    ui.label("Local tool injection uses the explicit Unity debug-bridge bypass; the control directory contains no OAuth credentials.");
}

fn poll_runtime_console(state: &mut ToolState) {
    match state.runtime_console.read_status() {
        Ok(status) => state.runtime_status = status,
        Err(error) => state.status = format!("Runtime status could not be read: {error}"),
    }
    if state
        .runtime_process
        .as_mut()
        .is_some_and(|process| process.try_wait().ok().flatten().is_some())
    {
        state.runtime_process = None;
    }
}

fn runtime_console_attached(status: Option<&RuntimeConsoleStatus>) -> bool {
    let Some(status) = status else {
        return false;
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| {
            u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
        });
    now.saturating_sub(status.updated_unix_millis) < 2_000
}

fn launch_runtime_game(state: &mut ToolState) -> anyhow::Result<()> {
    if state.runtime_process.is_some() {
        anyhow::bail!("the tools app already owns a running game process");
    }
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut command = Command::new("cargo");
    command
        .args(["run", "--release", "-p", "stream_town_game"])
        .current_dir(workspace)
        .env(
            "STREAM_TOWN_RUNTIME_CONSOLE_DIR",
            state.runtime_console.root(),
        )
        .env("STREAM_TOWN_AUTOSTART", "1")
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if state.runtime_profile_launch {
        command
            .env("STREAM_TOWN_REPORT_FRAME_TIME", "1")
            .env("STREAM_TOWN_FRAME_TIME_WARMUP", "5")
            .env("STREAM_TOWN_FRAME_TIME_SAMPLE_SECONDS", "10");
    }
    state.runtime_process = Some(command.spawn()?);
    Ok(())
}

fn inject_runtime_command(state: &mut ToolState) -> anyhow::Result<()> {
    let _: ChatCommand = state.command.parse()?;
    let actor_id = StableId::new(state.runtime_actor_id.clone())?;
    let login_name = state.runtime_login.trim();
    if login_name.is_empty() {
        anyhow::bail!("runtime login cannot be empty");
    }
    send_runtime_action(
        state,
        RuntimeConsoleAction::InjectChat {
            actor_id,
            login_name: login_name.to_owned(),
            display_name: login_name.to_owned(),
            command: state.command.clone(),
            is_broadcaster: true,
            is_moderator: true,
            is_subscriber: true,
        },
    )
}

fn send_runtime_action(state: &mut ToolState, action: RuntimeConsoleAction) -> anyhow::Result<()> {
    let acknowledged = state
        .runtime_status
        .as_ref()
        .map_or(0, |status| status.last_processed_sequence);
    state.runtime_sequence = state.runtime_sequence.max(acknowledged).saturating_add(1);
    state
        .runtime_console
        .write_request(&RuntimeConsoleRequest::new(state.runtime_sequence, action))?;
    Ok(())
}

fn format_runtime_frame_times(status: &RuntimeConsoleStatus) -> String {
    match (status.average_frame_ms, status.p95_frame_ms) {
        (Some(average), Some(p95)) => format!("{average:.2} / {p95:.2} ms"),
        _ => "Collecting".to_owned(),
    }
}

fn parse_game_master_ids(value: &str) -> BTreeSet<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(str::to_owned)
        .collect()
}

fn format_game_master_ids(ids: &BTreeSet<String>) -> String {
    ids.iter().cloned().collect::<Vec<_>>().join(", ")
}

fn sync_twitch_tool_fields(state: &mut ToolState) {
    state.game_master_ids = format_game_master_ids(&state.config.twitch.game_master_ids);
    state.fish_god_reward_id = state
        .config
        .twitch
        .fish_god_reward_id
        .clone()
        .unwrap_or_default();
    state.twitch_channel_identity = None;
    state.twitch_irc_verified = false;
}

#[allow(dead_code)]
fn twitch_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Twitch setup and diagnostics");
    ui.label("Use a Twitch public client with the exact chat:read and chat:edit scopes. OAuth tokens live only in the operating-system credential vault.");
    ui.checkbox(
        &mut state.config.twitch.enabled,
        "Enable Twitch in the game",
    );
    ui.horizontal(|ui| {
        ui.label("Client ID");
        ui.text_edit_singleline(&mut state.config.twitch.client_id);
    });
    ui.horizontal(|ui| {
        ui.label("Bot login");
        ui.text_edit_singleline(&mut state.config.twitch.bot_login);
    });
    ui.horizontal(|ui| {
        ui.label("Channel login");
        ui.text_edit_singleline(&mut state.config.twitch.channel_login);
    });
    let busy = state.twitch_auth_events.is_some();
    ui.horizontal(|ui| {
        ui.label("Game-master Twitch user IDs");
        if ui
            .text_edit_singleline(&mut state.game_master_ids)
            .changed()
        {
            state.config.twitch.game_master_ids = parse_game_master_ids(&state.game_master_ids);
        }
    });
    ui.horizontal(|ui| {
        ui.label("Resolve GM login");
        ui.text_edit_singleline(&mut state.game_master_lookup);
        if ui
            .add_enabled(
                !busy
                    && !state.config.twitch.client_id.trim().is_empty()
                    && !state.game_master_lookup.trim().is_empty(),
                egui::Button::new("Resolve and add ID"),
            )
            .clicked()
        {
            start_twitch_game_master_lookup(state);
        }
    });
    ui.label("GM IDs are explicit numeric Twitch user IDs; broadcaster/moderator status does not grant GM commands.");
    ui.horizontal(|ui| {
        ui.label("Fish God reward ID");
        if ui
            .text_edit_singleline(&mut state.fish_god_reward_id)
            .changed()
        {
            let reward_id = state.fish_god_reward_id.trim();
            state.config.twitch.fish_god_reward_id = if reward_id.is_empty() {
                None
            } else {
                Some(reward_id.to_owned())
            };
        }
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Capture next reward"),
            )
            .on_hover_text("Connect to chat, then redeem the intended Channel Points reward within three minutes")
            .clicked()
        {
            start_twitch_reward_capture(state);
        }
    });
    ui.horizontal(|ui| {
        if ui.button("Save runtime config").clicked() {
            state.status = match save_runtime_config(&state.config) {
                Ok(path) => format!("Saved public runtime configuration to {}", path.display()),
                Err(error) => format!("Could not save runtime configuration: {error:#}"),
            };
        }
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Authorize bot"),
            )
            .clicked()
        {
            start_twitch_authorization(state);
        }
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Run end-to-end diagnostic"),
            )
            .clicked()
        {
            start_twitch_diagnostic(state);
        }
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Forget token"),
            )
            .clicked()
        {
            start_twitch_clear(state);
        }
    });
    if let Some(device) = &state.twitch_device {
        ui.separator();
        ui.heading("Authorization pending");
        ui.label(format!("Enter code: {}", device.user_code));
        ui.hyperlink_to("Open Twitch activation page", &device.verification_uri);
        ui.label(format!(
            "This request expires in {} minutes; polling every {} seconds.",
            device.expires_in.div_ceil(60),
            device.interval
        ));
    }
    if let Some(validation) = &state.twitch_validation {
        ui.separator();
        ui.label(format!(
            "Validated bot '{}' (user {}) for client {}",
            validation.login, validation.user_id, validation.client_id
        ));
        ui.label(format!("Scopes: {}", validation.scopes.join(", ")));
        ui.label(format!(
            "Access token valid for {} seconds",
            validation.expires_in
        ));
    }
    if let Some(channel) = &state.twitch_channel_identity {
        ui.label(format!(
            "Resolved channel '{}' ({}, user {})",
            channel.display_name, channel.login, channel.id
        ));
    }
    if state.twitch_irc_verified {
        ui.colored_label(
            egui::Color32::LIGHT_GREEN,
            "Authenticated IRC connection and channel join verified.",
        );
    }

    ui.separator();
    ui.heading("Direct Twitch broadcast (Windows)");
    ui.label("The game captures its own Bevy render target and process audio, then publishes H.264/AAC through shared LGPL FFmpeg libraries. It does not launch FFmpeg, capture the desktop, or require OBS.");
    ui.checkbox(
        &mut state.config.twitch.broadcast.enabled,
        "Enable direct broadcast",
    );
    ui.label("The game always starts offline. Use Go Live from the main-menu status badge or the in-game Esc menu.");
    ui.horizontal(|ui| {
        ui.label("Output");
        ui.add(
            egui::DragValue::new(&mut state.config.twitch.broadcast.width)
                .range(320..=1_920)
                .suffix(" px"),
        );
        ui.label("×");
        ui.add(
            egui::DragValue::new(&mut state.config.twitch.broadcast.height)
                .range(180..=1_080)
                .suffix(" px"),
        );
        egui::ComboBox::from_id_salt("broadcast-fps")
            .selected_text(format!(
                "{} FPS",
                state.config.twitch.broadcast.frames_per_second
            ))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut state.config.twitch.broadcast.frames_per_second,
                    30,
                    "30 FPS",
                );
                ui.selectable_value(
                    &mut state.config.twitch.broadcast.frames_per_second,
                    60,
                    "60 FPS",
                );
            });
    });
    ui.horizontal(|ui| {
        ui.label("Video bitrate");
        ui.add(
            egui::DragValue::new(&mut state.config.twitch.broadcast.video_bitrate_kbps)
                .range(500..=6_000)
                .suffix(" Kbps"),
        );
        ui.label("Audio bitrate");
        ui.add(
            egui::DragValue::new(&mut state.config.twitch.broadcast.audio_bitrate_kbps)
                .range(64..=160)
                .suffix(" Kbps"),
        );
    });
    ui.horizontal(|ui| {
        ui.label("H.264 encoder");
        egui::ComboBox::from_id_salt("broadcast-encoder")
            .selected_text(broadcast_encoder_label(
                state.config.twitch.broadcast.encoder,
            ))
            .show_ui(ui, |ui| {
                for encoder in [
                    BroadcastEncoderPreference::Auto,
                    BroadcastEncoderPreference::Nvidia,
                    BroadcastEncoderPreference::Intel,
                    BroadcastEncoderPreference::Amd,
                    BroadcastEncoderPreference::MediaFoundation,
                    BroadcastEncoderPreference::OpenH264,
                ] {
                    ui.selectable_value(
                        &mut state.config.twitch.broadcast.encoder,
                        encoder,
                        broadcast_encoder_label(encoder),
                    );
                }
            });
        ui.label("Preferred ingest");
        ui.text_edit_singleline(&mut state.config.twitch.broadcast.ingest)
            .on_hover_text(
                "Optional name substring such as Sydney; empty uses Twitch's default ingest",
            );
    });
    ui.checkbox(
        &mut state.config.twitch.broadcast.bandwidth_test,
        "Bandwidth-test mode (does not go live)",
    );
    if state.config.twitch.broadcast.bandwidth_test {
        ui.colored_label(
            egui::Color32::YELLOW,
            "Bandwidth-test mode still sends the configured bitrate to Twitch. Turn it off before a real broadcast.",
        );
    }
    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Authorize broadcaster"),
            )
            .on_hover_text(
                "Requests only channel:read:stream_key for the configured channel account",
            )
            .clicked()
        {
            start_twitch_broadcast_authorization(state);
        }
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Test broadcast prerequisites"),
            )
            .clicked()
        {
            start_twitch_broadcast_diagnostic(state);
        }
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Forget broadcaster token"),
            )
            .clicked()
        {
            start_twitch_broadcast_clear(state);
        }
    });
    if let Some(validation) = &state.twitch_broadcast_validation {
        ui.colored_label(
            egui::Color32::LIGHT_GREEN,
            format!(
                "Broadcaster '{}' is authorized (user {}, token valid for {} seconds).",
                validation.login, validation.user_id, validation.expires_in
            ),
        );
    }
    ui.colored_label(
        egui::Color32::LIGHT_BLUE,
        "No client secret, OAuth credential, or stream key is stored in repository assets.",
    );
}

const fn broadcast_encoder_label(encoder: BroadcastEncoderPreference) -> &'static str {
    match encoder {
        BroadcastEncoderPreference::Auto => "Automatic",
        BroadcastEncoderPreference::Nvidia => "NVIDIA NVENC",
        BroadcastEncoderPreference::Intel => "Intel Quick Sync",
        BroadcastEncoderPreference::Amd => "AMD AMF",
        BroadcastEncoderPreference::MediaFoundation => "Windows Media Foundation",
        BroadcastEncoderPreference::OpenH264 => "OpenH264 (CPU)",
    }
}

fn poll_twitch_tool_events(state: &mut ToolState) {
    let events: Vec<_> = state
        .twitch_auth_events
        .as_ref()
        .and_then(|receiver| receiver.lock().ok())
        .map(|receiver| receiver.try_iter().collect())
        .unwrap_or_default();
    let mut finished = false;
    for event in events {
        match event {
            TwitchToolEvent::Device(device) => {
                "Twitch is waiting for device authorization".clone_into(&mut state.status);
                state.twitch_device = Some(device);
            }
            TwitchToolEvent::Authorized(validation) => {
                state.status = format!(
                    "Authorized and securely stored Twitch bot '{}'",
                    validation.login
                );
                state.twitch_validation = Some(validation);
                state.twitch_channel_identity = None;
                state.twitch_irc_verified = false;
                state.twitch_device = None;
                finished = true;
            }
            TwitchToolEvent::BroadcastAuthorized(validation) => {
                state.status = format!(
                    "Authorized and securely stored Twitch broadcaster '{}'",
                    validation.login
                );
                state.twitch_broadcast_validation = Some(validation);
                state.twitch_device = None;
                finished = true;
            }
            TwitchToolEvent::Progress(message) => {
                state.status = message;
            }
            TwitchToolEvent::Diagnostic {
                validation,
                channel,
            } => {
                state.status = format!(
                    "Twitch bot '{}' validated and joined channel '{}'",
                    validation.login, channel.login
                );
                state.twitch_validation = Some(validation);
                state.twitch_channel_identity = Some(channel);
                state.twitch_irc_verified = true;
                finished = true;
            }
            TwitchToolEvent::BroadcastDiagnostic {
                validation,
                ingest_count,
                prerequisites,
            } => {
                state.status = format!(
                    "Direct broadcast ready for '{}': {ingest_count} Twitch ingests; selected encoder: {}; rejected before selection: {}; available: {}; process audio: {}",
                    validation.login,
                    prerequisites.selected_encoder,
                    if prerequisites.rejected_encoders.is_empty() {
                        "none".to_owned()
                    } else {
                        prerequisites.rejected_encoders.join(" | ")
                    },
                    prerequisites.available_encoders.join(", "),
                    if prerequisites.process_audio_capture_available {
                        "ready"
                    } else {
                        "unavailable"
                    }
                );
                state.twitch_broadcast_validation = Some(validation);
                state.twitch_device = None;
                finished = true;
            }
            TwitchToolEvent::GameMasterResolved(identity) => {
                state
                    .config
                    .twitch
                    .game_master_ids
                    .insert(identity.id.clone());
                state.game_master_ids =
                    format_game_master_ids(&state.config.twitch.game_master_ids);
                state.status = format!(
                    "Resolved Twitch GM '{}' to numeric user ID {}",
                    identity.login, identity.id
                );
                finished = true;
            }
            TwitchToolEvent::RewardCaptured(reward_id) => {
                state.config.twitch.fish_god_reward_id = Some(reward_id.clone());
                state.fish_god_reward_id.clone_from(&reward_id);
                state.status = format!(
                    "Captured Channel Points reward ID {reward_id}; save the runtime config"
                );
                finished = true;
            }
            TwitchToolEvent::Cleared => {
                "Removed the Twitch token from the OS credential vault"
                    .clone_into(&mut state.status);
                state.twitch_validation = None;
                state.twitch_channel_identity = None;
                state.twitch_irc_verified = false;
                state.twitch_device = None;
                finished = true;
            }
            TwitchToolEvent::BroadcastCleared => {
                "Removed the Twitch broadcaster token from the OS credential vault"
                    .clone_into(&mut state.status);
                state.twitch_broadcast_validation = None;
                state.twitch_device = None;
                finished = true;
            }
            TwitchToolEvent::Error(error) => {
                state.status = format!("Twitch setup failed: {error}");
                state.twitch_device = None;
                state.twitch_irc_verified = false;
                finished = true;
            }
        }
    }
    if finished {
        state.twitch_auth_events = None;
    }
}

fn twitch_event_channel(state: &mut ToolState) -> mpsc::Sender<TwitchToolEvent> {
    let (sender, receiver) = mpsc::channel();
    state.twitch_auth_events = Some(Arc::new(Mutex::new(receiver)));
    sender
}

fn start_twitch_authorization(state: &mut ToolState) {
    state.twitch_device = None;
    state.twitch_validation = None;
    state.twitch_channel_identity = None;
    state.twitch_irc_verified = false;
    "Starting Twitch device authorization...".clone_into(&mut state.status);
    let config = state.config.twitch.clone();
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-oauth".to_owned())
        .spawn(move || {
            let outcome = (|| -> anyhow::Result<()> {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                runtime.block_on(async {
                    let oauth = OAuthClient::new(config.client_id.clone())?;
                    let authorization = oauth.begin_device_authorization().await?;
                    sender
                        .send(TwitchToolEvent::Device(authorization.clone()))
                        .map_err(|_| anyhow::anyhow!("Twitch setup window closed"))?;
                    let token = oauth.complete_device_authorization(&authorization).await?;
                    let validation = oauth.validate(&token).await?;
                    anyhow::ensure!(
                        validation.login == config.bot_login,
                        "authorized account '{}' does not match configured bot '{}'",
                        validation.login,
                        config.bot_login
                    );
                    CredentialVault::new(&config.client_id, &config.bot_login).save(&token)?;
                    sender
                        .send(TwitchToolEvent::Authorized(validation))
                        .map_err(|_| anyhow::anyhow!("Twitch setup window closed"))?;
                    Ok(())
                })
            })();
            if let Err(error) = outcome {
                let _ = sender.send(TwitchToolEvent::Error(format!("{error:#}")));
            }
        });
    if let Err(error) = worker {
        state.status = format!("Could not start Twitch authorization worker: {error}");
        state.twitch_auth_events = None;
    }
}

fn start_twitch_broadcast_authorization(state: &mut ToolState) {
    state.twitch_device = None;
    state.twitch_broadcast_validation = None;
    "Starting Twitch broadcaster authorization...".clone_into(&mut state.status);
    let config = state.config.twitch.clone();
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-broadcast-oauth".to_owned())
        .spawn(move || {
            let outcome = (|| -> anyhow::Result<()> {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                runtime.block_on(async {
                    let oauth = OAuthClient::broadcaster(config.client_id.clone())?;
                    let authorization = oauth.begin_device_authorization().await?;
                    sender
                        .send(TwitchToolEvent::Device(authorization.clone()))
                        .map_err(|_| anyhow::anyhow!("Twitch setup window closed"))?;
                    let token = oauth.complete_device_authorization(&authorization).await?;
                    let validation = oauth.validate(&token).await?;
                    anyhow::ensure!(
                        validation.login == config.channel_login,
                        "authorized account '{}' does not match configured channel '{}'",
                        validation.login,
                        config.channel_login
                    );
                    CredentialVault::broadcaster(&config.client_id, &config.channel_login)
                        .save(&token)?;
                    sender
                        .send(TwitchToolEvent::BroadcastAuthorized(validation))
                        .map_err(|_| anyhow::anyhow!("Twitch setup window closed"))?;
                    Ok(())
                })
            })();
            if let Err(error) = outcome {
                let _ = sender.send(TwitchToolEvent::Error(format!("{error:#}")));
            }
        });
    if let Err(error) = worker {
        state.status = format!("Could not start Twitch broadcaster authorization: {error}");
        state.twitch_auth_events = None;
    }
}

fn start_twitch_broadcast_diagnostic(state: &mut ToolState) {
    "Validating broadcaster authorization, Twitch ingest access, shared FFmpeg, and process audio..."
        .clone_into(&mut state.status);
    let config = state.config.twitch.clone();
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-broadcast-check".to_owned())
        .spawn(move || {
            let outcome =
                (|| -> anyhow::Result<(TokenValidation, usize, BroadcastPrerequisites)> {
                    let runtime = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()?;
                    let (validation, ingest_count) = runtime.block_on(async {
                        let oauth = OAuthClient::broadcaster(config.client_id.clone())?;
                        let vault =
                            CredentialVault::broadcaster(&config.client_id, &config.channel_login);
                        let (token, validation) = oauth.load_validated_token(&vault).await?;
                        anyhow::ensure!(
                            validation.login == config.channel_login,
                            "stored broadcaster token belongs to '{}', expected '{}'",
                            validation.login,
                            config.channel_login
                        );
                        // Confirm Helix can return a non-empty key without exposing or persisting it.
                        let stream_key = oauth.stream_key(&token, &validation.user_id).await?;
                        drop(stream_key);
                        let ingests = oauth.ingests().await?;
                        Ok::<_, anyhow::Error>((validation, ingests.len()))
                    })?;
                    let prerequisites = inspect_broadcast_prerequisites(&config.broadcast)?;
                    anyhow::ensure!(
                        prerequisites.process_audio_capture_available,
                        "Windows process-scoped audio capture is unavailable"
                    );
                    Ok((validation, ingest_count, prerequisites))
                })();
            let event = outcome.map_or_else(
                |error| TwitchToolEvent::Error(format!("{error:#}")),
                |(validation, ingest_count, prerequisites)| TwitchToolEvent::BroadcastDiagnostic {
                    validation,
                    ingest_count,
                    prerequisites,
                },
            );
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not start direct-broadcast diagnostic: {error}");
        state.twitch_auth_events = None;
    }
}

fn start_twitch_diagnostic(state: &mut ToolState) {
    "Validating and refreshing the Twitch token...".clone_into(&mut state.status);
    let config = state.config.twitch.clone();
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-twitch-check".to_owned())
        .spawn(move || {
            let outcome = (|| -> anyhow::Result<(TokenValidation, TwitchUserIdentity)> {
                let vault = CredentialVault::new(&config.client_id, &config.bot_login);
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                let (validation, channel) = runtime.block_on(async {
                    let oauth = OAuthClient::new(config.client_id.clone())?;
                    let (token, validation) = oauth.load_validated_token(&vault).await?;
                    anyhow::ensure!(
                        validation.login == config.bot_login,
                        "stored token belongs to '{}', expected '{}'",
                        validation.login,
                        config.bot_login
                    );
                    let channel = oauth.lookup_user(&token, &config.channel_login).await?;
                    Ok::<_, anyhow::Error>((validation, channel))
                })?;
                sender
                    .send(TwitchToolEvent::Progress(format!(
                        "Token and channel validated; joining #{}...",
                        config.channel_login
                    )))
                    .map_err(|_| anyhow::anyhow!("Twitch setup window closed"))?;
                let transport = TwitchTransport::start(config.clone())?;
                wait_for_twitch_connection(&transport, Duration::from_secs(30))?;
                let _ = transport.send(TwitchControl::Disconnect);
                Ok((validation, channel))
            })();
            let event = outcome.map_or_else(
                |error| TwitchToolEvent::Error(format!("{error:#}")),
                |(validation, channel)| TwitchToolEvent::Diagnostic {
                    validation,
                    channel,
                },
            );
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not start Twitch diagnostic worker: {error}");
        state.twitch_auth_events = None;
    }
}

fn start_twitch_game_master_lookup(state: &mut ToolState) {
    let config = state.config.twitch.clone();
    let login = state.game_master_lookup.trim().to_ascii_lowercase();
    state.status = format!("Resolving Twitch user '{login}'...");
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-twitch-user-lookup".to_owned())
        .spawn(move || {
            let outcome = (|| -> anyhow::Result<TwitchUserIdentity> {
                let vault = CredentialVault::new(&config.client_id, &config.bot_login);
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                runtime.block_on(async {
                    let oauth = OAuthClient::new(config.client_id.clone())?;
                    let (token, validation) = oauth.load_validated_token(&vault).await?;
                    anyhow::ensure!(
                        validation.login == config.bot_login,
                        "stored token belongs to '{}', expected '{}'",
                        validation.login,
                        config.bot_login
                    );
                    oauth.lookup_user(&token, &login).await
                })
            })();
            let event = outcome.map_or_else(
                |error| TwitchToolEvent::Error(format!("{error:#}")),
                TwitchToolEvent::GameMasterResolved,
            );
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not start Twitch user lookup worker: {error}");
        state.twitch_auth_events = None;
    }
}

fn start_twitch_reward_capture(state: &mut ToolState) {
    let config = state.config.twitch.clone();
    "Connecting to Twitch; redeem the intended Channel Points reward after the tool confirms the channel join..."
        .clone_into(&mut state.status);
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-twitch-reward-capture".to_owned())
        .spawn(move || {
            let outcome = (|| -> anyhow::Result<String> {
                let transport = TwitchTransport::start(config)?;
                wait_for_twitch_connection(&transport, Duration::from_secs(30))?;
                sender
                    .send(TwitchToolEvent::Progress(
                        "Connected. Redeem the intended Channel Points reward now (three-minute timeout)."
                            .to_owned(),
                    ))
                    .map_err(|_| anyhow::anyhow!("Twitch setup window closed"))?;
                let reward_id = wait_for_twitch_reward(&transport, Duration::from_mins(3))?;
                let _ = transport.send(TwitchControl::Disconnect);
                Ok(reward_id)
            })();
            let event = outcome.map_or_else(
                |error| TwitchToolEvent::Error(format!("{error:#}")),
                TwitchToolEvent::RewardCaptured,
            );
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not start Twitch reward capture worker: {error}");
        state.twitch_auth_events = None;
    }
}

fn wait_for_twitch_connection(
    transport: &TwitchTransport,
    timeout: Duration,
) -> anyhow::Result<()> {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        match transport.try_recv() {
            Some(TwitchEvent::Status(TwitchStatus::Connected)) => return Ok(()),
            Some(TwitchEvent::Status(TwitchStatus::Error(error))) => anyhow::bail!("{error}"),
            Some(TwitchEvent::Status(TwitchStatus::Disconnected)) => {
                anyhow::bail!("Twitch disconnected before joining the configured channel")
            }
            _ => thread::sleep(Duration::from_millis(50)),
        }
    }
    anyhow::bail!("timed out waiting for Twitch to join the configured channel")
}

fn wait_for_twitch_reward(
    transport: &TwitchTransport,
    timeout: Duration,
) -> anyhow::Result<String> {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        match transport.try_recv() {
            Some(TwitchEvent::Chat(envelope)) => {
                if let Some(reward_id) = envelope.custom_reward_id {
                    return Ok(reward_id);
                }
            }
            Some(TwitchEvent::Status(TwitchStatus::Error(error))) => anyhow::bail!("{error}"),
            Some(TwitchEvent::Status(TwitchStatus::Disconnected)) => {
                anyhow::bail!("Twitch disconnected while waiting for a reward redemption")
            }
            _ => thread::sleep(Duration::from_millis(50)),
        }
    }
    anyhow::bail!("timed out waiting for a Channel Points reward redemption")
}

fn start_twitch_clear(state: &mut ToolState) {
    "Removing Twitch token from the OS credential vault...".clone_into(&mut state.status);
    let config = state.config.twitch.clone();
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-twitch-clear".to_owned())
        .spawn(move || {
            let event = CredentialVault::new(&config.client_id, &config.bot_login)
                .clear()
                .map_or_else(
                    |error| TwitchToolEvent::Error(format!("{error:#}")),
                    |()| TwitchToolEvent::Cleared,
                );
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not start Twitch credential cleanup worker: {error}");
        state.twitch_auth_events = None;
    }
}

fn start_twitch_broadcast_clear(state: &mut ToolState) {
    "Removing Twitch broadcaster token from the OS credential vault..."
        .clone_into(&mut state.status);
    let config = state.config.twitch.clone();
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-broadcast-clear".to_owned())
        .spawn(move || {
            let event = CredentialVault::broadcaster(&config.client_id, &config.channel_login)
                .clear()
                .map_or_else(
                    |error| TwitchToolEvent::Error(format!("{error:#}")),
                    |()| TwitchToolEvent::BroadcastCleared,
                );
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not remove Twitch broadcaster credentials: {error}");
        state.twitch_auth_events = None;
    }
}

fn save_runtime_config(config: &GameConfig) -> anyhow::Result<std::path::PathBuf> {
    stream_town_game::save_runtime_config(config)
}

fn merge_authoring_config_with_runtime(
    authored: &GameConfig,
    existing_runtime: Option<&GameConfig>,
) -> GameConfig {
    let mut applied = authored.clone();
    if let Some(existing_runtime) = existing_runtime {
        // Twitch identities and broadcast preferences are local operator data.
        // The tools app authors simulation/content settings, so applying those
        // settings must never replace setup entered through the game's Secrets
        // and Streaming screens with the repository baseline.
        applied.twitch = existing_runtime.twitch.clone();
    }
    applied
}

fn save_authoring_runtime_config(config: &GameConfig) -> anyhow::Result<PathBuf> {
    let runtime_path = stream_town_game::runtime_config_path();
    let existing_runtime = runtime_path
        .is_file()
        .then(stream_town_game::load_runtime_config)
        .transpose()
        .with_context(|| {
            format!(
                "could not preserve local Twitch setup from {}",
                runtime_path.display()
            )
        })?;
    let applied = merge_authoring_config_with_runtime(config, existing_runtime.as_ref());
    save_runtime_config(&applied)
}

fn save_and_apply_game_config(
    config: &GameConfig,
    project_path: &str,
) -> anyhow::Result<(PathBuf, PathBuf)> {
    let project = save_game_config(config, project_path)?;
    let runtime = save_authoring_runtime_config(config)?;
    Ok((project, runtime))
}

fn validation_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Asset validator and packager");
    if ui.button("Validate configuration and catalog").clicked() {
        state.status = match (
            state.config.validate(),
            state.catalog.validate(),
            state.presentation.validate(),
        ) {
            (Ok(()), Ok(()), Ok(())) => format!(
                "Configuration and catalogs valid: {} semantic records, {} materials, {} controllers",
                state.catalog.source_records.len(),
                state.presentation.materials.len(),
                state.presentation.controllers.len()
            ),
            (Err(error), _, _) => format!("Configuration error: {error}"),
            (_, Err(error), _) => format!("Content catalog error: {error}"),
            (_, _, Err(error)) => format!("Presentation catalog error: {error}"),
        };
    }
    let busy = state.tool_job_events.is_some();
    ui.horizontal(|ui| {
        if ui
            .add_enabled(!busy, egui::Button::new("Run repository validation"))
            .clicked()
        {
            start_xtask_job(state, "Validation", ["validate"]);
        }
        if ui
            .add_enabled(!busy, egui::Button::new("Build Windows release package"))
            .clicked()
        {
            start_xtask_job(state, "Windows package", ["package-windows"]);
        }
    });
    ui.label("Checks include stable IDs, dangling references, technology cycles, GLB hashes/headers, and deterministic baselines.");
    ui.label("Release packaging builds optimized game/tools executables, includes only runtime assets plus GPL attribution, validates the archive, and writes dist/stream-town-windows-x86_64.zip.");
}

fn poll_tool_job_events(state: &mut ToolState) {
    let event = state
        .tool_job_events
        .as_ref()
        .and_then(|receiver| receiver.lock().ok())
        .and_then(|receiver| receiver.try_recv().ok());
    match event {
        Some(ToolJobEvent::Finished(message) | ToolJobEvent::Error(message)) => {
            state.status = message;
            state.tool_job_events = None;
        }
        None => {}
    }
}

fn start_xtask_job<const N: usize>(
    state: &mut ToolState,
    label: &'static str,
    args: [&'static str; N],
) {
    let (sender, receiver) = mpsc::channel();
    state.tool_job_events = Some(Arc::new(Mutex::new(receiver)));
    state.status = format!("{label} is running...");
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let worker = thread::Builder::new()
        .name("stream-town-tools-xtask".to_owned())
        .spawn(move || {
            let outcome = Command::new("cargo")
                .current_dir(&workspace)
                .arg("run")
                .arg("-p")
                .arg("xtask")
                .arg("--")
                .args(args)
                .output();
            let event = match outcome {
                Ok(output) if output.status.success() => {
                    let summary = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .last()
                        .unwrap_or("completed successfully")
                        .to_owned();
                    ToolJobEvent::Finished(format!("{label} complete: {summary}"))
                }
                Ok(output) => {
                    let error = String::from_utf8_lossy(&output.stderr)
                        .lines()
                        .last()
                        .unwrap_or("unknown error")
                        .to_owned();
                    ToolJobEvent::Error(format!("{label} failed: {error}"))
                }
                Err(error) => ToolJobEvent::Error(format!("Could not start {label}: {error}")),
            };
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not start {label} worker: {error}");
        state.tool_job_events = None;
    }
}

fn technology_draft(catalog: &ContentCatalog, id: &StableId) -> Option<TechnologyDraft> {
    let node = catalog.technology.nodes.get(id)?;
    Some(TechnologyDraft {
        id: id.clone(),
        value: node.clone(),
        display_name: node.display_name.clone(),
        description: node.description.clone(),
        age: node.age.clone(),
        tier: node.tier,
        group: node.group.clone(),
        prerequisites: node
            .prerequisites
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        unlocks: node
            .unlocks
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        objectives: node
            .objectives
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        icon_path: node.icon_path.clone(),
        initially_unlocked: node.initially_unlocked,
        unavailable: node.unavailable,
        prerequisite_search: String::new(),
        unlock_search: String::new(),
        objective_search: String::new(),
        icon_search: String::new(),
    })
}

fn default_catalog_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content/catalog.ron")
}

fn default_presentation_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content/presentation.ron")
}

fn default_technology_layout_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content/technology_layout.ron")
}

fn default_config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/config/game.ron")
}

fn validate_authoring_assets() -> anyhow::Result<String> {
    let config = load_game_config(default_config_path().to_string_lossy().as_ref())?;
    let catalog = load_content_catalog(default_catalog_path().to_string_lossy().as_ref())?;
    let technology_layout = load_technology_layout(
        default_technology_layout_path().to_string_lossy().as_ref(),
        &catalog,
    )?;
    let presentation: PresentationCatalog =
        ron::from_str(include_str!("../../../assets/content/presentation.ron"))?;
    presentation.validate()?;
    Ok(format!(
        "Authoring assets valid: schema {}, {} roles, {} foliage layers, {} technologies, {} graph positions, {} presentation records",
        config.schema_version,
        catalog.roles.len(),
        catalog.foliage.len(),
        catalog.technology.nodes.len(),
        technology_layout.nodes.len(),
        presentation.textures.len()
            + presentation.materials.len()
            + presentation.clips.len()
            + presentation.controllers.len(),
    ))
}

fn load_content_catalog(path: &str) -> anyhow::Result<ContentCatalog> {
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("content-catalog path cannot be empty");
    }
    let catalog: ContentCatalog = ron::from_str(&fs::read_to_string(&path)?)?;
    catalog.validate()?;
    Ok(catalog)
}

fn reload_content_catalog(state: &mut ToolState) -> anyhow::Result<()> {
    let catalog = load_content_catalog(&state.catalog_path)?;
    let technology_layout = load_technology_layout(&state.technology_layout_path, &catalog)?;
    state.catalog = catalog;
    state.technology_layout = technology_layout;
    state.undo_authoring.clear();
    state.redo_authoring.clear();
    state.technology_graph_view.request_fit();
    refresh_catalog_drafts(state);
    Ok(())
}

fn load_technology_layout(
    path: &str,
    catalog: &ContentCatalog,
) -> anyhow::Result<TechnologyGraphLayout> {
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("technology-layout path cannot be empty");
    }
    let layout: TechnologyGraphLayout = ron::from_str(&fs::read_to_string(&path)?)?;
    layout.validate(&catalog.technology)?;
    Ok(layout)
}

fn load_game_config(path: &str) -> anyhow::Result<GameConfig> {
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("game-config path cannot be empty");
    }
    let config: GameConfig = ron::from_str(&fs::read_to_string(&path)?)?;
    Ok(config.upgrade()?)
}

fn save_game_config(config: &GameConfig, path: &str) -> anyhow::Result<PathBuf> {
    config.validate()?;
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("game-config path cannot be empty");
    }
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    fs::create_dir_all(parent)?;
    let temporary = PathBuf::from(format!("{}.tmp", path.display()));
    let backup = PathBuf::from(format!("{}.bak", path.display()));
    let encoded =
        ron::ser::to_string_pretty(config, ron::ser::PrettyConfig::new().struct_names(true))?;
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(encoded.as_bytes())?;
    file.sync_all()?;
    if path.is_file() {
        fs::copy(&path, &backup)?;
        fs::remove_file(&path)?;
    }
    if let Err(error) = fs::rename(&temporary, &path) {
        if backup.is_file() && !path.exists() {
            let _ = fs::copy(&backup, &path);
        }
        let _ = fs::remove_file(&temporary);
        return Err(error.into());
    }
    let reloaded = load_game_config(path.to_string_lossy().as_ref())?;
    if reloaded != *config {
        anyhow::bail!("reloaded game configuration does not match the authored configuration");
    }
    Ok(path)
}

fn save_content_catalog(catalog: &ContentCatalog, path: &str) -> anyhow::Result<PathBuf> {
    catalog.validate()?;
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("catalog path cannot be empty");
    }
    let encoded = ron::ser::to_string_pretty(catalog, ron::ser::PrettyConfig::default())?;
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    fs::create_dir_all(parent)?;
    let temporary = PathBuf::from(format!("{}.tmp", path.display()));
    let backup = PathBuf::from(format!("{}.bak", path.display()));
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(encoded.as_bytes())?;
    file.sync_all()?;
    if path.is_file() {
        fs::copy(&path, &backup)?;
        fs::remove_file(&path)?;
    }
    if let Err(error) = fs::rename(&temporary, &path) {
        if backup.is_file() && !path.exists() {
            let _ = fs::copy(&backup, &path);
        }
        let _ = fs::remove_file(&temporary);
        return Err(error.into());
    }
    let reloaded: ContentCatalog = ron::from_str(&fs::read_to_string(&path)?)?;
    reloaded.validate()?;
    if reloaded != *catalog {
        anyhow::bail!("reloaded catalog does not match the authored catalog");
    }
    Ok(path)
}

fn save_presentation_catalog(catalog: &PresentationCatalog, path: &str) -> anyhow::Result<PathBuf> {
    catalog.validate()?;
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("presentation-catalog path cannot be empty");
    }
    let encoded = ron::ser::to_string_pretty(catalog, ron::ser::PrettyConfig::default())?;
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    fs::create_dir_all(parent)?;
    let temporary = PathBuf::from(format!("{}.tmp", path.display()));
    let backup = PathBuf::from(format!("{}.bak", path.display()));
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(encoded.as_bytes())?;
    file.sync_all()?;
    if path.is_file() {
        fs::copy(&path, &backup)?;
        fs::remove_file(&path)?;
    }
    if let Err(error) = fs::rename(&temporary, &path) {
        if backup.is_file() && !path.exists() {
            let _ = fs::copy(&backup, &path);
        }
        let _ = fs::remove_file(&temporary);
        return Err(error.into());
    }
    let reloaded: PresentationCatalog = ron::from_str(&fs::read_to_string(&path)?)?;
    reloaded.validate()?;
    if reloaded != *catalog {
        anyhow::bail!("reloaded presentation catalog does not match the authored catalog");
    }
    Ok(path)
}

fn save_technology_layout(
    layout: &TechnologyGraphLayout,
    catalog: &ContentCatalog,
    path: &str,
) -> anyhow::Result<PathBuf> {
    catalog.validate()?;
    layout.validate(&catalog.technology)?;
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("technology-layout path cannot be empty");
    }
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    fs::create_dir_all(parent)?;
    let temporary = PathBuf::from(format!("{}.tmp", path.display()));
    let backup = PathBuf::from(format!("{}.bak", path.display()));
    let encoded = ron::ser::to_string_pretty(layout, ron::ser::PrettyConfig::default())?;
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(encoded.as_bytes())?;
    file.sync_all()?;
    if path.is_file() {
        fs::copy(&path, &backup)?;
        fs::remove_file(&path)?;
    }
    if let Err(error) = fs::rename(&temporary, &path) {
        if backup.is_file() && !path.exists() {
            let _ = fs::copy(&backup, &path);
        }
        let _ = fs::remove_file(&temporary);
        return Err(error.into());
    }
    let reloaded = load_technology_layout(path.to_string_lossy().as_ref(), catalog)?;
    if reloaded != *layout {
        anyhow::bail!("reloaded technology layout does not match the authored layout");
    }
    Ok(path)
}

fn labeled_ids(ids: BTreeSet<StableId>) -> Vec<(StableId, String)> {
    ids.into_iter()
        .map(|id| {
            let label = id
                .as_str()
                .rsplit_once(':')
                .map_or(id.as_str(), |(_, suffix)| suffix)
                .replace(['_', '-'], " ");
            (id, label)
        })
        .collect()
}

fn resource_choices(catalog: &ContentCatalog) -> Vec<(StableId, String)> {
    let mut ids = BTreeSet::new();
    for role in catalog.roles.values() {
        ids.extend(role.resource.iter().cloned());
    }
    for building in catalog.buildings.values() {
        ids.extend(building.cost.keys().cloned());
        ids.extend(building.level_cost.keys().cloned());
        ids.extend(building.storage.iter().map(|value| value.resource.clone()));
        ids.extend(
            building
                .passive_resources
                .iter()
                .map(|value| value.resource.clone()),
        );
    }
    for archetype in catalog.archetypes.values() {
        if let Some(enemy) = &archetype.enemy {
            ids.insert(enemy.kill_reward.resource.clone());
        }
    }
    ids.extend(
        catalog
            .objectives
            .values()
            .filter_map(|objective| objective.resource.clone()),
    );
    for node in catalog.technology.nodes.values() {
        ids.extend(node.storage_boost_percent.keys().cloned());
    }
    labeled_ids(ids)
}

fn station_kind_choices(catalog: &ContentCatalog) -> Vec<(StableId, String)> {
    let mut ids = BTreeSet::new();
    for role in catalog.roles.values() {
        ids.extend(role.station_kinds.iter().cloned());
    }
    for building in catalog.buildings.values() {
        if let Some(station) = &building.station {
            ids.extend(station.accepted_role_kinds.iter().cloned());
        }
    }
    labeled_ids(ids)
}

fn target_kind_choices(catalog: &ContentCatalog) -> Vec<(StableId, String)> {
    let mut ids = BTreeSet::new();
    for role in catalog.roles.values() {
        ids.extend(role.target_kinds.iter().cloned());
    }
    for building in catalog.buildings.values() {
        if let Some(station) = &building.station {
            ids.extend(station.target_kinds.iter().cloned());
        }
    }
    for archetype in catalog.archetypes.values() {
        if let Some(enemy) = &archetype.enemy {
            ids.extend(enemy.target_kinds.iter().cloned());
        }
    }
    labeled_ids(ids)
}

fn ability_choices(catalog: &ContentCatalog) -> Vec<(StableId, String)> {
    labeled_ids(
        catalog
            .roles
            .values()
            .flat_map(|role| role.granted_abilities.iter().cloned())
            .collect(),
    )
}

fn stat_choices(catalog: &ContentCatalog) -> Vec<(StableId, String)> {
    let mut ids = BTreeSet::new();
    for node in catalog.technology.nodes.values() {
        ids.extend(node.global_stat_boost_percent.keys().cloned());
        for boosts in node.role_stat_boost_percent.values() {
            ids.extend(boosts.keys().cloned());
        }
    }
    labeled_ids(ids)
}

fn action_animation_choices(catalog: &ContentCatalog) -> Vec<String> {
    let mut values = BTreeSet::new();
    for role in catalog.roles.values() {
        values.insert(role.action_animation.clone());
        if let Some(animation) = role
            .equipment
            .as_ref()
            .and_then(|equipment| equipment.carry_animation.clone())
        {
            values.insert(animation);
        }
    }
    for archetype in catalog.archetypes.values() {
        if let Some(models) = &archetype.enemy_models {
            values.extend(
                models
                    .weapons
                    .iter()
                    .map(|weapon| weapon.action_animation.clone()),
            );
        }
    }
    values.retain(|value| !value.trim().is_empty());
    values.into_iter().collect()
}

fn equipment_node_choices(
    catalog: &ContentCatalog,
    presentation: &PresentationCatalog,
) -> Vec<String> {
    let mut values = BTreeSet::new();
    for equipment in catalog
        .roles
        .values()
        .filter_map(|role| role.equipment.as_ref())
    {
        values.extend(equipment.body_nodes.iter().cloned());
        values.extend(equipment.left_hand_node.iter().cloned());
        values.extend(equipment.right_hand_node.iter().cloned());
        values.extend(equipment.helmet_node.iter().cloned());
    }
    values.extend(
        presentation
            .prefab_renderer_materials
            .values()
            .flatten()
            .filter_map(|binding| binding.target_path.rsplit('/').next())
            .filter(|name| {
                name.starts_with("Body_")
                    || name.starts_with("LHand_")
                    || name.starts_with("RHand_")
                    || name.starts_with("Back_")
                    || name.starts_with("Helmet_")
            })
            .map(ToOwned::to_owned),
    );
    values.retain(|value| !value.trim().is_empty());
    values.into_iter().collect()
}

fn building_model_node_choices(catalog: &ContentCatalog) -> Vec<String> {
    let mut values = BTreeSet::new();
    for building in catalog.buildings.values() {
        for model in &building.model_handlers {
            values.insert(model.full_model.clone());
            values.extend(model.construction_stages.iter().cloned());
            values.extend(model.upgrades.iter().cloned());
            values.extend(model.other_models.iter().cloned());
        }
        for model in &building.storage_models {
            values.insert(model.empty_model.clone());
            values.insert(model.half_full_model.clone());
            values.insert(model.full_model.clone());
        }
    }
    for archetype in catalog.archetypes.values() {
        values.extend(
            archetype
                .rotating_nodes
                .iter()
                .map(|rotating| rotating.node.clone()),
        );
    }
    values.retain(|value| !value.trim().is_empty());
    values.into_iter().collect()
}

fn projectile_pool_choices(catalog: &ContentCatalog) -> Vec<String> {
    catalog
        .buildings
        .values()
        .filter_map(|building| building.projectile_shooter.as_ref())
        .map(|shooter| shooter.projectile_pool.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn technology_icon_choices(state: &ToolState) -> Vec<String> {
    let mut values: BTreeSet<_> = state
        .catalog
        .technology
        .nodes
        .values()
        .map(|node| node.icon_path.clone())
        .collect();
    values.extend(
        state
            .presentation
            .textures
            .values()
            .map(|texture| texture.asset_path.clone()),
    );
    values.retain(|value| !value.trim().is_empty());
    values.into_iter().collect()
}

fn archetype_kind_choice(ui: &mut egui::Ui, value: &mut ArchetypeKind) {
    egui::ComboBox::from_id_salt(("archetype_kind", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(format!("{value:?}"))
        .show_ui(ui, |ui| {
            for kind in [
                ArchetypeKind::Building,
                ArchetypeKind::Player,
                ArchetypeKind::Enemy,
                ArchetypeKind::Resource,
                ArchetypeKind::Environment,
                ArchetypeKind::Vfx,
                ArchetypeKind::Ui,
                ArchetypeKind::Other,
            ] {
                ui.selectable_value(value, kind, format!("{kind:?}"));
            }
        });
}

fn searchable_string_choice(
    ui: &mut egui::Ui,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    label: &str,
    value: &mut String,
    choices: &[String],
    search: &mut String,
) -> bool {
    let mut changed = false;
    ui.push_id(id_salt, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(label);
            let selected = value
                .rsplit('/')
                .next()
                .filter(|name| !name.is_empty())
                .unwrap_or("Choose…")
                .to_owned();
            ui.menu_button(selected, |ui| {
                let response = ui.add(
                    egui::TextEdit::singleline(search)
                        .hint_text("Type to filter…")
                        .desired_width(280.0),
                );
                if response.gained_focus() {
                    response.request_focus();
                }
                let needle = search.trim().to_ascii_lowercase();
                egui::ScrollArea::vertical()
                    .id_salt("searchable_choices")
                    .max_height(280.0)
                    .show(ui, |ui| {
                        for choice in choices.iter().filter(|choice| {
                            needle.is_empty() || choice.to_ascii_lowercase().contains(&needle)
                        }) {
                            if ui.selectable_label(choice == value, choice).clicked() {
                                value.clone_from(choice);
                                changed = true;
                                search.clear();
                                ui.close();
                            }
                        }
                    });
            });
        });
    });
    changed
}

fn source_model_for_asset_path(catalog: &ContentCatalog, asset_path: &str) -> String {
    catalog
        .archetypes
        .values()
        .flat_map(|archetype| archetype.scenes.iter())
        .find(|scene| scene.asset_path == asset_path)
        .map_or_else(|| asset_path.to_owned(), |scene| scene.source_model.clone())
}

fn create_model_archetype(state: &mut ToolState) -> Result<(), String> {
    let id = StableId::new(state.new_archetype_id.trim().to_owned())
        .map_err(|error| error.to_string())?;
    if state.catalog.archetypes.contains_key(&id) {
        return Err(format!("model archetype {id} already exists"));
    }
    let display_name = state.new_archetype_name.trim();
    if display_name.is_empty() {
        return Err("model display name cannot be empty".to_owned());
    }
    let asset_path = state.new_archetype_asset.trim();
    if !state
        .discovered_model_assets
        .iter()
        .any(|candidate| candidate == asset_path)
    {
        return Err("choose a discovered GLB asset".to_owned());
    }
    let scene = ArchetypeScene {
        source_model: source_model_for_asset_path(&state.catalog, asset_path),
        asset_path: asset_path.to_owned(),
        age: None,
        is_default: true,
    };
    let archetype = ArchetypeDef {
        display_name: display_name.to_owned(),
        kind: state.new_archetype_kind,
        source_guid: format!("authored:{}", id.as_str()),
        source_path: format!("Authored/{}.glb", display_name.replace(' ', "_")),
        bounds: ArchetypeBounds {
            center: [0.0, 0.5, 0.0],
            size: [1.0, 1.0, 1.0],
        },
        footprint: [1, 1],
        scenes: vec![scene],
        component_types: Vec::new(),
        disable_after_milliseconds: None,
        health_bar_hide_milliseconds: None,
        rotating_nodes: Vec::new(),
        target_size_milli_cells: 0,
        health: None,
        enemy: None,
        enemy_models: None,
        enemy_spawner: None,
        pet: None,
    };
    let mut candidate = state.catalog.clone();
    candidate.archetypes.insert(id.clone(), archetype);
    commit_catalog_candidate(state, candidate)?;
    state.selected_archetype = Some(id);
    state.selected_archetype_scene = 0;
    Ok(())
}

fn add_archetype_scene(state: &mut ToolState, id: &StableId) -> Result<(), String> {
    let asset_path = state.new_archetype_asset.trim().to_owned();
    if !state
        .discovered_model_assets
        .iter()
        .any(|candidate| candidate == &asset_path)
    {
        return Err("choose a discovered GLB asset in Add model archetype first".to_owned());
    }
    let source_model = source_model_for_asset_path(&state.catalog, &asset_path);
    let mut candidate = state.catalog.clone();
    let archetype = candidate
        .archetypes
        .get_mut(id)
        .ok_or_else(|| format!("missing model archetype {id}"))?;
    if archetype
        .scenes
        .iter()
        .any(|scene| scene.asset_path == asset_path)
    {
        return Err("that GLB is already a variant of this archetype".to_owned());
    }
    let is_default = archetype.scenes.is_empty();
    archetype.scenes.push(ArchetypeScene {
        source_model,
        asset_path,
        age: None,
        is_default,
    });
    let selected = archetype.scenes.len() - 1;
    commit_catalog_candidate(state, candidate)?;
    state.selected_archetype_scene = selected;
    Ok(())
}

fn remove_archetype_scene(
    state: &mut ToolState,
    id: &StableId,
    index: usize,
) -> Result<(), String> {
    let mut candidate = state.catalog.clone();
    let archetype = candidate
        .archetypes
        .get_mut(id)
        .ok_or_else(|| format!("missing model archetype {id}"))?;
    if index >= archetype.scenes.len() {
        return Err("model variant index is stale".to_owned());
    }
    let removed_default = archetype.scenes[index].is_default;
    archetype.scenes.remove(index);
    if removed_default && let Some(first) = archetype.scenes.first_mut() {
        first.is_default = true;
    }
    commit_catalog_candidate(state, candidate)?;
    state.selected_archetype_scene = state
        .selected_archetype_scene
        .min(state.catalog.archetypes[id].scenes.len().saturating_sub(1));
    Ok(())
}

fn delete_model_archetype(state: &mut ToolState, id: &StableId) -> Result<(), String> {
    let mut candidate = state.catalog.clone();
    candidate
        .archetypes
        .remove(id)
        .ok_or_else(|| format!("missing model archetype {id}"))?;
    commit_catalog_candidate(state, candidate)?;
    state.selected_archetype = state.catalog.archetypes.keys().next().cloned();
    state.selected_archetype_scene = 0;
    Ok(())
}

fn stable_id_required_choice(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut StableId,
    choices: &[(StableId, String)],
) -> bool {
    let previous = value.clone();
    let selected = choices
        .iter()
        .find(|(id, _)| id == value)
        .map_or_else(|| value.to_string(), |(_, label)| label.clone());
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt((label, ui.next_auto_id()))
            .height(AUTHORING_COMBO_MAX_HEIGHT)
            .selected_text(selected)
            .show_ui(ui, |ui| {
                for (id, display) in choices {
                    ui.selectable_value(value, id.clone(), format!("{display}  ({id})"));
                }
            });
    });
    *value != previous
}

fn stable_id_option_choice(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<StableId>,
    choices: &[(StableId, String)],
) {
    let selected = value
        .as_ref()
        .and_then(|selected| choices.iter().find(|(id, _)| id == selected))
        .map_or("None", |(_, label)| label.as_str());
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt((label, ui.next_auto_id()))
            .height(AUTHORING_COMBO_MAX_HEIGHT)
            .selected_text(selected)
            .show_ui(ui, |ui| {
                ui.selectable_value(value, None, "None");
                for (id, display) in choices {
                    ui.selectable_value(value, Some(id.clone()), format!("{display}  ({id})"));
                }
            });
    });
}

fn stable_id_set_choices(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut BTreeSet<StableId>,
    choices: &[(StableId, String)],
) {
    ui.collapsing(format!("{label} ({})", values.len()), |ui| {
        ui.horizontal_wrapped(|ui| {
            for (id, display) in choices {
                let mut selected = values.contains(id);
                if ui.checkbox(&mut selected, display).changed() {
                    if selected {
                        values.insert(id.clone());
                    } else {
                        values.remove(id);
                    }
                }
            }
        });
    });
}

fn stable_id_vec_choices(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut Vec<StableId>,
    choices: &[(StableId, String)],
) {
    ui.collapsing(format!("{label} ({})", values.len()), |ui| {
        ui.horizontal_wrapped(|ui| {
            for (id, display) in choices {
                let mut selected = values.contains(id);
                if ui.checkbox(&mut selected, display).changed() {
                    if selected {
                        values.push(id.clone());
                    } else {
                        values.retain(|value| value != id);
                    }
                }
            }
        });
    });
}

fn searchable_stable_id_vec_editor(
    ui: &mut egui::Ui,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    label: &str,
    values: &mut Vec<StableId>,
    choices: &[(StableId, String)],
    search: &mut String,
) {
    ui.push_id(id_salt, |ui| {
        ui.label(format!("{label} ({})", values.len()));
        let mut remove = None;
        ui.horizontal_wrapped(|ui| {
            for value in values.iter() {
                let display = choices
                    .iter()
                    .find(|(id, _)| id == value)
                    .map_or(value.as_str(), |(_, display)| display.as_str());
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(display);
                        if ui.small_button("×").clicked() {
                            remove = Some(value.clone());
                        }
                    });
                });
            }
        });
        if let Some(remove) = remove {
            values.retain(|value| value != &remove);
        }
        ui.menu_button(format!("Add {label}…"), |ui| {
            ui.add(
                egui::TextEdit::singleline(search)
                    .hint_text("Type a name or stable ID…")
                    .desired_width(300.0),
            );
            let needle = search.trim().to_ascii_lowercase();
            let candidates = choices
                .iter()
                .filter(|(id, display)| {
                    !values.contains(id)
                        && (needle.is_empty()
                            || id.as_str().to_ascii_lowercase().contains(&needle)
                            || display.to_ascii_lowercase().contains(&needle))
                })
                .cloned()
                .collect::<Vec<_>>();
            egui::ScrollArea::vertical()
                .id_salt("stable_id_search_results")
                .max_height(260.0)
                .show(ui, |ui| {
                    for (id, display) in candidates {
                        if ui
                            .selectable_label(false, format!("{display}  ({id})"))
                            .clicked()
                        {
                            values.push(id);
                            search.clear();
                            ui.close();
                        }
                    }
                });
        });
    });
}

fn string_choice(ui: &mut egui::Ui, label: &str, value: &mut String, choices: &[String]) {
    let selected = if value.is_empty() {
        "None".to_owned()
    } else {
        value.clone()
    };
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt((label, ui.next_auto_id()))
            .height(AUTHORING_COMBO_MAX_HEIGHT)
            .selected_text(selected)
            .show_ui(ui, |ui| {
                for choice in choices {
                    ui.selectable_value(&mut *value, choice.clone(), choice);
                }
            });
    });
}

fn optional_string_choice(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<String>,
    choices: &[String],
) {
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt((label, ui.next_auto_id()))
            .height(AUTHORING_COMBO_MAX_HEIGHT)
            .selected_text(value.as_deref().unwrap_or("None"))
            .show_ui(ui, |ui| {
                ui.selectable_value(value, None, "None");
                for choice in choices {
                    ui.selectable_value(value, Some(choice.clone()), choice);
                }
            });
    });
}

fn string_vec_choices(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut Vec<String>,
    choices: &[String],
) {
    ui.collapsing(format!("{label} ({})", values.len()), |ui| {
        let mut remove = None;
        for (index, value) in values.iter_mut().enumerate() {
            ui.push_id((label, index), |ui| {
                ui.horizontal(|ui| {
                    string_choice(ui, "Model", value, choices);
                    if ui.small_button("Remove").clicked() {
                        remove = Some(index);
                    }
                });
            });
        }
        if let Some(index) = remove {
            values.remove(index);
        }
        if ui.button("Add model").clicked()
            && let Some(first) = choices.first()
        {
            values.push(first.clone());
        }
    });
}

fn stable_u32_map_editor(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut BTreeMap<StableId, u32>,
    choices: &[(StableId, String)],
) {
    ui.label(label);
    let mut remove = None;
    for id in values.keys().cloned().collect::<Vec<_>>() {
        ui.horizontal(|ui| {
            ui.label(
                choices
                    .iter()
                    .find(|(choice, _)| choice == &id)
                    .map_or(id.as_str(), |(_, label)| label),
            );
            ui.add(egui::DragValue::new(
                values.get_mut(&id).expect("map key exists"),
            ));
            if ui.small_button("Remove").clicked() {
                remove = Some(id);
            }
        });
    }
    if let Some(id) = remove {
        values.remove(&id);
    }
    let mut add = None;
    egui::ComboBox::from_id_salt((label, "add", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text("Add…")
        .show_ui(ui, |ui| {
            for (id, display) in choices {
                if !values.contains_key(id) && ui.selectable_label(false, display).clicked() {
                    add = Some(id.clone());
                }
            }
        });
    if let Some(id) = add {
        values.insert(id, 1);
    }
}

fn stable_u16_map_editor(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut BTreeMap<StableId, u16>,
    choices: &[(StableId, String)],
) {
    ui.label(label);
    let mut remove = None;
    for id in values.keys().cloned().collect::<Vec<_>>() {
        ui.horizontal(|ui| {
            ui.label(
                choices
                    .iter()
                    .find(|(choice, _)| choice == &id)
                    .map_or(id.as_str(), |(_, label)| label),
            );
            ui.add(egui::DragValue::new(
                values.get_mut(&id).expect("map key exists"),
            ));
            if ui.small_button("Remove").clicked() {
                remove = Some(id);
            }
        });
    }
    if let Some(id) = remove {
        values.remove(&id);
    }
    let mut add = None;
    egui::ComboBox::from_id_salt((label, "add", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text("Add…")
        .show_ui(ui, |ui| {
            for (id, display) in choices {
                if !values.contains_key(id) && ui.selectable_label(false, display).clicked() {
                    add = Some(id.clone());
                }
            }
        });
    if let Some(id) = add {
        values.insert(id, 1);
    }
}

fn stable_i32_map_editor(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut BTreeMap<StableId, i32>,
    choices: &[(StableId, String)],
) {
    ui.label(label);
    let mut remove = None;
    for id in values.keys().cloned().collect::<Vec<_>>() {
        ui.horizontal(|ui| {
            ui.label(
                choices
                    .iter()
                    .find(|(choice, _)| choice == &id)
                    .map_or(id.as_str(), |(_, label)| label),
            );
            ui.add(egui::DragValue::new(values.get_mut(&id).expect("map key exists")).suffix("%"));
            if ui.small_button("Remove").clicked() {
                remove = Some(id);
            }
        });
    }
    if let Some(id) = remove {
        values.remove(&id);
    }
    let mut add = None;
    egui::ComboBox::from_id_salt((label, "add", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text("Add…")
        .show_ui(ui, |ui| {
            for (id, display) in choices {
                if !values.contains_key(id) && ui.selectable_label(false, display).clicked() {
                    add = Some(id.clone());
                }
            }
        });
    if let Some(id) = add {
        values.insert(id, 0);
    }
}

fn role_stat_map_editor(
    ui: &mut egui::Ui,
    values: &mut BTreeMap<StableId, BTreeMap<StableId, i32>>,
    roles: &[(StableId, String)],
    stats: &[(StableId, String)],
) {
    ui.label("Role-specific stat boosts");
    let mut remove = None;
    for role in values.keys().cloned().collect::<Vec<_>>() {
        let label = roles
            .iter()
            .find(|(id, _)| id == &role)
            .map_or(role.as_str(), |(_, label)| label);
        ui.collapsing(label, |ui| {
            stable_i32_map_editor(
                ui,
                "Stats",
                values.get_mut(&role).expect("role map exists"),
                stats,
            );
            if ui.button("Remove role effects").clicked() {
                remove = Some(role.clone());
            }
        });
    }
    if let Some(role) = remove {
        values.remove(&role);
    }
    let mut add = None;
    egui::ComboBox::from_id_salt(("role stat", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text("Add role…")
        .show_ui(ui, |ui| {
            for (id, display) in roles {
                if !values.contains_key(id) && ui.selectable_label(false, display).clicked() {
                    add = Some(id.clone());
                }
            }
        });
    if let Some(role) = add {
        values.insert(role, BTreeMap::new());
    }
}

fn draw_building_visual(
    ui: &mut egui::Ui,
    building: &BuildingDef,
    catalog: &ContentCatalog,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    ui.group(|ui| {
        let archetype = catalog.archetypes.get(&building.archetype);
        ui.heading(archetype.map_or("Missing archetype", |value| value.display_name.as_str()));
        if let Some(archetype) = archetype {
            for scene in &archetype.scenes {
                ui.monospace(format!(
                    "{}{}",
                    if scene.is_default { "● " } else { "○ " },
                    scene.asset_path
                ));
            }
        }
        draw_model_preview(
            ui,
            preview_texture,
            preview_status,
            egui::vec2(320.0, 214.0),
            preview_controls,
            false,
        );
        ui.small(format!(
            "Runtime placement occupies {} × {} cells",
            building.footprint[0], building.footprint[1]
        ));
    });
}

fn footprint_editor(
    ui: &mut egui::Ui,
    footprint: &mut [u16; 2],
    label: &str,
    maximum: u16,
) -> bool {
    let previous = *footprint;
    ui.group(|ui| {
        ui.horizontal_wrapped(|ui| {
            ui.strong(label);
            ui.add(
                egui::DragValue::new(&mut footprint[0])
                    .range(1..=maximum)
                    .prefix("Width "),
            );
            ui.add(
                egui::DragValue::new(&mut footprint[1])
                    .range(1..=maximum)
                    .prefix("Depth "),
            );
            if ui.small_button("− column").clicked() {
                footprint[0] = footprint[0].saturating_sub(1).max(1);
            }
            if ui.small_button("+ column").clicked() {
                footprint[0] = footprint[0].saturating_add(1).min(maximum);
            }
            if ui.small_button("− row").clicked() {
                footprint[1] = footprint[1].saturating_sub(1).max(1);
            }
            if ui.small_button("+ row").clicked() {
                footprint[1] = footprint[1].saturating_add(1).min(maximum);
            }
            if ui.small_button("Rotate 90°").clicked() {
                footprint.swap(0, 1);
            }
        });
        draw_footprint_grid(ui, *footprint, egui::vec2(300.0, 170.0));
        ui.small(format!(
            "{} occupied cell{} · origin is the highlighted top-left cell",
            u32::from(footprint[0]) * u32::from(footprint[1]),
            if *footprint == [1, 1] { "" } else { "s" }
        ));
    });
    *footprint != previous
}

fn default_navigation_footprint_thirds(placement: [u16; 2]) -> [u16; 2] {
    placement.map(|axis| axis.saturating_mul(3).saturating_sub(2).max(1))
}

fn draw_footprint_grid(ui: &mut egui::Ui, footprint: [u16; 2], desired: egui::Vec2) {
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    ui.painter()
        .rect_filled(rect, 5.0, egui::Color32::from_rgb(18, 27, 34));
    let width = f32::from(footprint[0].max(1));
    let depth = f32::from(footprint[1].max(1));
    let scale = (rect.width() / width).min(rect.height() / depth) * 0.86;
    let grid =
        egui::Rect::from_center_size(rect.center(), egui::vec2(width * scale, depth * scale));
    ui.painter()
        .rect_filled(grid, 3.0, egui::Color32::from_rgb(71, 120, 145));
    let line = egui::Stroke::new(0.8, egui::Color32::from_rgb(132, 184, 207));
    if footprint[0] <= 64 {
        for x in 1..footprint[0] {
            let x = grid.left() + f32::from(x) * scale;
            ui.painter().line_segment(
                [egui::pos2(x, grid.top()), egui::pos2(x, grid.bottom())],
                line,
            );
        }
    }
    if footprint[1] <= 64 {
        for z in 1..footprint[1] {
            let y = grid.top() + f32::from(z) * scale;
            ui.painter().line_segment(
                [egui::pos2(grid.left(), y), egui::pos2(grid.right(), y)],
                line,
            );
        }
    }
    let origin = egui::Rect::from_min_size(grid.min, egui::Vec2::splat(scale));
    ui.painter().rect_filled(
        origin.shrink(1.0),
        2.0,
        egui::Color32::from_rgb(218, 167, 67),
    );
    ui.painter().rect_stroke(
        grid,
        3.0,
        egui::Stroke::new(1.5, egui::Color32::from_rgb(183, 218, 232)),
        egui::StrokeKind::Inside,
    );
}

fn free_string_vec_editor(ui: &mut egui::Ui, label: &str, values: &mut Vec<String>) {
    ui.collapsing(format!("{label} ({})", values.len()), |ui| {
        let mut remove = None;
        for (index, value) in values.iter_mut().enumerate() {
            ui.horizontal_wrapped(|ui| {
                ui.text_edit_singleline(value);
                if ui.small_button("Remove").clicked() {
                    remove = Some(index);
                }
            });
        }
        if let Some(index) = remove {
            values.remove(index);
        }
        if ui.button(format!("Add {label}")).clicked() {
            values.push(String::new());
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn archetype_runtime_editor(
    ui: &mut egui::Ui,
    archetype: &mut ArchetypeDef,
    resources: &[(StableId, String)],
    target_kinds: &[(StableId, String)],
    enemy_archetypes: &[(StableId, String)],
    enemy_types: &[(StableId, String)],
    enemy_pools: &[(StableId, String)],
    model_nodes: &[String],
    animation_names: &[String],
) {
    ui.collapsing("Runtime behavior", |ui| {
        optional_u32_editor(
            ui,
            "Disable after",
            &mut archetype.disable_after_milliseconds,
            1_000,
            " ms",
        );
        optional_u32_editor(
            ui,
            "Health bar hide delay",
            &mut archetype.health_bar_hide_milliseconds,
            1_000,
            " ms",
        );
        ui.add(
            egui::DragValue::new(&mut archetype.target_size_milli_cells)
                .range(0..=u32::MAX)
                .prefix("Target size ")
                .suffix(" milli-cells"),
        );
        rotating_nodes_editor(ui, &mut archetype.rotating_nodes, model_nodes);
        health_definition_editor(ui, &mut archetype.health);
        enemy_definition_editor(
            ui,
            &mut archetype.enemy,
            resources,
            target_kinds,
            enemy_types,
            enemy_pools,
        );
        enemy_model_set_editor(
            ui,
            &mut archetype.enemy_models,
            model_nodes,
            animation_names,
        );
        enemy_spawner_editor(ui, &mut archetype.enemy_spawner, enemy_archetypes);
        pet_definition_editor(ui, &mut archetype.pet, model_nodes);
    });
}

fn optional_u32_editor(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<u32>,
    default_value: u32,
    suffix: &str,
) {
    ui.horizontal_wrapped(|ui| {
        let mut enabled = value.is_some();
        if ui.checkbox(&mut enabled, label).changed() {
            *value = enabled.then_some(default_value);
        }
        if let Some(value) = value {
            ui.add(egui::DragValue::new(value).suffix(suffix));
        }
    });
}

fn rotating_nodes_editor(
    ui: &mut egui::Ui,
    values: &mut Vec<RotatingNodeDef>,
    model_nodes: &[String],
) {
    ui.collapsing(format!("Rotating nodes ({})", values.len()), |ui| {
        let mut remove = None;
        for (index, value) in values.iter_mut().enumerate() {
            ui.push_id(("rotating_node", index), |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Hierarchy path");
                        ui.text_edit_singleline(&mut value.hierarchy_path);
                        if model_nodes.is_empty() {
                            ui.label("Node");
                            ui.text_edit_singleline(&mut value.node);
                        } else {
                            string_choice(ui, "Node", &mut value.node, model_nodes);
                        }
                        if ui.small_button("Remove").clicked() {
                            remove = Some(index);
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        for (component, axis) in value.axis.iter_mut().zip(["Axis X", "Y", "Z"]) {
                            ui.add(egui::DragValue::new(component).prefix(format!("{axis} ")));
                        }
                        ui.add(
                            egui::DragValue::new(&mut value.degrees_per_second)
                                .suffix(" degrees/s"),
                        );
                        let mut aged = value.age.is_some();
                        if ui.checkbox(&mut aged, "Age-specific").changed() {
                            value.age = aged.then_some(1);
                        }
                        if let Some(age) = value.age.as_mut() {
                            ui.add(egui::DragValue::new(age).range(1..=u8::MAX));
                        }
                    });
                });
            });
        }
        if let Some(index) = remove {
            values.remove(index);
        }
        if ui.button("Add rotating node").clicked() {
            values.push(RotatingNodeDef {
                hierarchy_path: String::new(),
                age: None,
                node: model_nodes.first().cloned().unwrap_or_default(),
                axis: [0.0, 1.0, 0.0],
                degrees_per_second: 30.0,
            });
        }
    });
}

fn health_definition_editor(ui: &mut egui::Ui, value: &mut Option<HealthDef>) {
    ui.collapsing("Health and revival", |ui| {
        let mut enabled = value.is_some();
        if ui.checkbox(&mut enabled, "Has health").changed() {
            *value = enabled.then_some(HealthDef {
                max_health: 100,
                health_gain_per_level: 0,
                regeneration_milli_per_second: 0,
                regeneration_requires_food: false,
                revive_milliseconds: None,
            });
        }
        if let Some(value) = value {
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::DragValue::new(&mut value.max_health).prefix("Maximum "));
                ui.add(egui::DragValue::new(&mut value.health_gain_per_level).prefix("Per level "));
                ui.add(
                    egui::DragValue::new(&mut value.regeneration_milli_per_second)
                        .prefix("Regeneration milli/s "),
                );
                ui.checkbox(&mut value.regeneration_requires_food, "Requires food");
            });
            optional_u32_editor(
                ui,
                "Revives after",
                &mut value.revive_milliseconds,
                5_000,
                " ms",
            );
        }
    });
}

fn enemy_definition_editor(
    ui: &mut egui::Ui,
    value: &mut Option<EnemyDef>,
    resources: &[(StableId, String)],
    target_kinds: &[(StableId, String)],
    enemy_types: &[(StableId, String)],
    enemy_pools: &[(StableId, String)],
) {
    ui.collapsing("Enemy combat behavior", |ui| {
        let mut enabled = value.is_some();
        if ui.checkbox(&mut enabled, "Is an enemy").changed() {
            *value = enabled.then(|| EnemyDef {
                enemy_type: enemy_types.first().map_or_else(
                    || StableId::new("enemy_type:new").unwrap(),
                    |value| value.0.clone(),
                ),
                pool: enemy_pools.first().map_or_else(
                    || StableId::new("pool:enemy_new").unwrap(),
                    |value| value.0.clone(),
                ),
                additional_health_milli_per_player: 0,
                action_amount: 5,
                action_milliseconds: 1_000,
                action_range_milli_cells: 1_000,
                target_search_range_milli_cells: 10_000,
                attack_attacker: true,
                kill_reward: ResourceReward {
                    resource: resources.first().map_or_else(
                        || StableId::new("resource:food").unwrap(),
                        |value| value.0.clone(),
                    ),
                    amount: 1,
                },
                targets_all: true,
                target_kinds: BTreeSet::new(),
            });
        }
        let Some(value) = value else {
            return;
        };
        if enemy_types.is_empty() {
            ui.label("Enemy type");
            ui.monospace(value.enemy_type.to_string());
        } else {
            stable_id_required_choice(ui, "Enemy type", &mut value.enemy_type, enemy_types);
        }
        if enemy_pools.is_empty() {
            ui.label("Pool");
            ui.monospace(value.pool.to_string());
        } else {
            stable_id_required_choice(ui, "Pool", &mut value.pool, enemy_pools);
        }
        ui.horizontal_wrapped(|ui| {
            ui.add(
                egui::DragValue::new(&mut value.additional_health_milli_per_player)
                    .prefix("Health/player milli "),
            );
            ui.add(egui::DragValue::new(&mut value.action_amount).prefix("Damage "));
            ui.add(egui::DragValue::new(&mut value.action_milliseconds).prefix("Cadence ms "));
            ui.add(
                egui::DragValue::new(&mut value.action_range_milli_cells).prefix("Action range "),
            );
            ui.add(
                egui::DragValue::new(&mut value.target_search_range_milli_cells)
                    .prefix("Search range "),
            );
            ui.checkbox(&mut value.attack_attacker, "Retaliates");
        });
        ui.horizontal_wrapped(|ui| {
            if !resources.is_empty() {
                stable_id_required_choice(
                    ui,
                    "Kill reward",
                    &mut value.kill_reward.resource,
                    resources,
                );
            }
            ui.add(egui::DragValue::new(&mut value.kill_reward.amount).prefix("Amount "));
        });
        ui.checkbox(&mut value.targets_all, "Targets every kind");
        stable_id_set_choices(ui, "Target kinds", &mut value.target_kinds, target_kinds);
    });
}

fn enemy_model_set_editor(
    ui: &mut egui::Ui,
    value: &mut Option<EnemyModelSetDef>,
    model_nodes: &[String],
    animation_names: &[String],
) {
    ui.collapsing("Enemy model variants", |ui| {
        let mut enabled = value.is_some();
        if ui
            .checkbox(&mut enabled, "Uses enemy model handler")
            .changed()
        {
            *value = enabled.then_some(EnemyModelSetDef {
                base_models: Vec::new(),
                permanent_models: Vec::new(),
                optional_models: Vec::new(),
                weapons: Vec::new(),
                base_animation_variants: 1,
            });
        }
        let Some(value) = value else {
            return;
        };
        ui.add(
            egui::DragValue::new(&mut value.base_animation_variants)
                .range(1..=u8::MAX)
                .prefix("Base animation variants "),
        );
        model_string_vec_editor(ui, "Base models", &mut value.base_models, model_nodes);
        model_string_vec_editor(
            ui,
            "Permanent models",
            &mut value.permanent_models,
            model_nodes,
        );
        model_string_vec_editor(
            ui,
            "Optional models",
            &mut value.optional_models,
            model_nodes,
        );
        let mut remove_weapon = None;
        for (index, weapon) in value.weapons.iter_mut().enumerate() {
            ui.push_id(("enemy_weapon", index), |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        if model_nodes.is_empty() {
                            ui.text_edit_singleline(&mut weapon.main_model);
                        } else {
                            string_choice(ui, "Main model", &mut weapon.main_model, model_nodes);
                        }
                        if animation_names.is_empty() {
                            ui.text_edit_singleline(&mut weapon.action_animation);
                        } else {
                            string_choice(
                                ui,
                                "Action animation",
                                &mut weapon.action_animation,
                                animation_names,
                            );
                        }
                        ui.add(
                            egui::DragValue::new(&mut weapon.action_animation_variants)
                                .range(1..=u8::MAX)
                                .prefix("Variants "),
                        );
                        enemy_run_animation_choice(ui, &mut weapon.run_animation);
                        if ui.small_button("Remove weapon").clicked() {
                            remove_weapon = Some(index);
                        }
                    });
                    model_string_vec_editor(
                        ui,
                        "Off-hand models",
                        &mut weapon.off_hand_models,
                        model_nodes,
                    );
                });
            });
        }
        if let Some(index) = remove_weapon {
            value.weapons.remove(index);
        }
        if ui.button("Add weapon model").clicked() {
            value.weapons.push(EnemyWeaponModelDef {
                main_model: model_nodes.first().cloned().unwrap_or_default(),
                off_hand_models: Vec::new(),
                action_animation: animation_names
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "Attack".to_owned()),
                action_animation_variants: 1,
                run_animation: EnemyRunAnimation::Generic,
            });
        }
    });
}

fn model_string_vec_editor(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut Vec<String>,
    choices: &[String],
) {
    if choices.is_empty() {
        free_string_vec_editor(ui, label, values);
    } else {
        string_vec_choices(ui, label, values, choices);
    }
}

fn enemy_run_animation_choice(ui: &mut egui::Ui, value: &mut EnemyRunAnimation) {
    egui::ComboBox::from_id_salt(("enemy_run_animation", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(format!("{value:?}"))
        .show_ui(ui, |ui| {
            ui.selectable_value(value, EnemyRunAnimation::Generic, "Generic");
            ui.selectable_value(value, EnemyRunAnimation::TwoHanded, "Two handed");
        });
}

fn enemy_spawner_editor(
    ui: &mut egui::Ui,
    value: &mut Option<EnemySpawnerDef>,
    enemies: &[(StableId, String)],
) {
    ui.collapsing("Enemy spawner", |ui| {
        let mut enabled = value.is_some();
        if ui.checkbox(&mut enabled, "Spawns enemies").changed() {
            *value = enabled.then_some(EnemySpawnerDef {
                min_total_enemies: 1,
                max_total_enemies: 3,
                spawn_milliseconds: 5_000,
                weighted_enemies: Vec::new(),
                spawn_offsets_milli_cells: vec![[0, 0]],
            });
        }
        let Some(value) = value else {
            return;
        };
        ui.horizontal_wrapped(|ui| {
            ui.add(egui::DragValue::new(&mut value.min_total_enemies).prefix("Minimum "));
            ui.add(egui::DragValue::new(&mut value.max_total_enemies).prefix("Maximum "));
            ui.add(egui::DragValue::new(&mut value.spawn_milliseconds).prefix("Spawn cadence ms "));
        });
        let mut remove_enemy = None;
        for (index, spawn) in value.weighted_enemies.iter_mut().enumerate() {
            ui.horizontal_wrapped(|ui| {
                stable_id_required_choice(ui, "Enemy", &mut spawn.enemy_archetype, enemies);
                ui.add(egui::DragValue::new(&mut spawn.weight_milli).prefix("Weight milli "));
                if ui.small_button("Remove").clicked() {
                    remove_enemy = Some(index);
                }
            });
        }
        if let Some(index) = remove_enemy {
            value.weighted_enemies.remove(index);
        }
        if ui
            .add_enabled(!enemies.is_empty(), egui::Button::new("Add weighted enemy"))
            .clicked()
        {
            value.weighted_enemies.push(WeightedEnemySpawn {
                enemy_archetype: enemies[0].0.clone(),
                weight_milli: 1_000,
            });
        }
        let mut remove_offset = None;
        for (index, offset) in value.spawn_offsets_milli_cells.iter_mut().enumerate() {
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::DragValue::new(&mut offset[0]).prefix("Offset X "));
                ui.add(egui::DragValue::new(&mut offset[1]).prefix("Z "));
                if ui.small_button("Remove offset").clicked() {
                    remove_offset = Some(index);
                }
            });
        }
        if let Some(index) = remove_offset {
            value.spawn_offsets_milli_cells.remove(index);
        }
        if ui.button("Add spawn offset").clicked() {
            value.spawn_offsets_milli_cells.push([0, 0]);
        }
    });
}

fn pet_definition_editor(ui: &mut egui::Ui, value: &mut Option<PetDef>, models: &[String]) {
    ui.collapsing("Pet follower", |ui| {
        let mut enabled = value.is_some();
        if ui.checkbox(&mut enabled, "Is a pet follower").changed() {
            *value = enabled.then_some(PetDef {
                closest_distance: 0.5,
                max_distance: 5.0,
                min_move_speed: 1.0,
                max_move_speed: 4.0,
                rotation_radians_per_second: 3.0,
                models: BTreeMap::new(),
            });
        }
        let Some(value) = value else {
            return;
        };
        ui.horizontal_wrapped(|ui| {
            ui.add(egui::DragValue::new(&mut value.closest_distance).prefix("Closest "));
            ui.add(egui::DragValue::new(&mut value.max_distance).prefix("Maximum "));
            ui.add(egui::DragValue::new(&mut value.min_move_speed).prefix("Minimum speed "));
            ui.add(egui::DragValue::new(&mut value.max_move_speed).prefix("Maximum speed "));
            ui.add(
                egui::DragValue::new(&mut value.rotation_radians_per_second)
                    .prefix("Rotation rad/s "),
            );
        });
        let ids = value.models.keys().cloned().collect::<Vec<_>>();
        let mut remove = None;
        for id in ids {
            let Some(model) = value.models.get_mut(&id) else {
                continue;
            };
            ui.push_id(("pet_model", id.as_str()), |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.monospace(id.to_string());
                        if models.is_empty() {
                            ui.text_edit_singleline(&mut model.source_model);
                        } else {
                            string_choice(ui, "Source model", &mut model.source_model, models);
                        }
                        if ui.small_button("Remove").clicked() {
                            remove = Some(id.clone());
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        for (component, label) in
                            model
                                .local_position
                                .iter_mut()
                                .zip(["Position X", "Y", "Z"])
                        {
                            ui.add(egui::DragValue::new(component).prefix(format!("{label} ")));
                        }
                        for (component, label) in
                            model.local_scale.iter_mut().zip(["Scale X", "Y", "Z"])
                        {
                            ui.add(egui::DragValue::new(component).prefix(format!("{label} ")));
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        for (component, label) in
                            model
                                .local_rotation
                                .iter_mut()
                                .zip(["Rotation X", "Y", "Z", "W"])
                        {
                            ui.add(egui::DragValue::new(component).prefix(format!("{label} ")));
                        }
                    });
                });
            });
        }
        if let Some(id) = remove {
            value.models.remove(&id);
        }
        if ui.button("Add pet model").clicked() {
            let mut suffix = value.models.len();
            loop {
                let id = StableId::new(format!("pet_model:authored_{suffix}"))
                    .expect("generated pet model ID is valid");
                if let std::collections::btree_map::Entry::Vacant(entry) = value.models.entry(id) {
                    entry.insert(PetModelDef {
                        source_model: models.first().cloned().unwrap_or_default(),
                        local_position: [0.0; 3],
                        local_rotation: [0.0, 0.0, 0.0, 1.0],
                        local_scale: [1.0; 3],
                    });
                    break;
                }
                suffix += 1;
            }
        }
    });
}

fn building_draft(catalog: &ContentCatalog, id: &StableId) -> Option<BuildingDraft> {
    Some(BuildingDraft {
        id: id.clone(),
        value: catalog.buildings.get(id)?.clone(),
    })
}

fn refresh_building_draft(state: &mut ToolState) {
    if state
        .selected_building
        .as_ref()
        .is_none_or(|id| !state.catalog.buildings.contains_key(id))
    {
        state.selected_building = state.catalog.buildings.keys().next().cloned();
    }
    state.building_draft = state
        .selected_building
        .as_ref()
        .and_then(|id| building_draft(&state.catalog, id));
}

fn apply_building_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .building_draft
        .clone()
        .ok_or_else(|| "no building selected".to_owned())?;
    if draft.value.display_name.trim().is_empty() {
        return Err("building display name cannot be empty".to_owned());
    }
    let mut candidate = state.catalog.clone();
    candidate
        .archetypes
        .get_mut(&draft.value.archetype)
        .ok_or_else(|| format!("missing model archetype {}", draft.value.archetype))?
        .footprint = draft.value.footprint;
    synchronize_buildings_for_archetype(
        &mut candidate,
        &draft.value.archetype,
        draft.value.footprint,
    );
    candidate.buildings.insert(draft.id, draft.value);
    commit_catalog_candidate(state, candidate)
}

fn synchronize_buildings_for_archetype(
    catalog: &mut ContentCatalog,
    archetype: &StableId,
    footprint: [u16; 2],
) -> usize {
    let mut updated = 0;
    for building in catalog
        .buildings
        .values_mut()
        .filter(|building| &building.archetype == archetype)
    {
        building.footprint = footprint;
        updated += 1;
    }
    updated
}

fn duplicate_selected_building(state: &mut ToolState) -> Result<(), String> {
    let source = state
        .selected_building
        .as_ref()
        .and_then(|id| state.catalog.buildings.get(id))
        .cloned()
        .ok_or_else(|| "no building selected".to_owned())?;
    let id = StableId::new(state.new_building_id.trim().to_owned())
        .map_err(|error| error.to_string())?;
    if state.catalog.buildings.contains_key(&id) {
        return Err(format!("building {id} already exists"));
    }
    let display_name = state.new_building_name.trim();
    if display_name.is_empty() {
        return Err("new building name cannot be empty".to_owned());
    }
    let mut value = source;
    display_name.clone_into(&mut value.display_name);
    let mut candidate = state.catalog.clone();
    candidate.buildings.insert(id.clone(), value);
    commit_catalog_candidate(state, candidate)?;
    state.selected_building = Some(id);
    refresh_building_draft(state);
    Ok(())
}

fn delete_selected_building(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_building
        .clone()
        .ok_or_else(|| "no building selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    candidate
        .buildings
        .remove(&id)
        .ok_or_else(|| format!("missing building {id}"))?;
    commit_catalog_candidate(state, candidate)?;
    state.selected_building = state.catalog.buildings.keys().next().cloned();
    refresh_building_draft(state);
    Ok(())
}

fn role_draft(catalog: &ContentCatalog, id: &StableId) -> Option<RoleDraft> {
    let value = catalog.roles.get(id)?.clone();
    let equipment = value.equipment.as_ref();
    Some(RoleDraft {
        id: id.clone(),
        resource: value
            .resource
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
        station_kinds: value
            .station_kinds
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        target_kinds: value
            .target_kinds
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        granted_abilities: value
            .granted_abilities
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        has_equipment: equipment.is_some(),
        body_nodes: equipment.map_or_else(
            || [String::new(), String::new(), String::new()],
            |equipment| equipment.body_nodes.clone(),
        ),
        left_hand_node: equipment
            .and_then(|equipment| equipment.left_hand_node.clone())
            .unwrap_or_default(),
        right_hand_node: equipment
            .and_then(|equipment| equipment.right_hand_node.clone())
            .unwrap_or_default(),
        helmet_node: equipment
            .and_then(|equipment| equipment.helmet_node.clone())
            .unwrap_or_default(),
        carry_animation: equipment
            .and_then(|equipment| equipment.carry_animation.clone())
            .unwrap_or_default(),
        left_hand_permanent: equipment.is_some_and(|equipment| equipment.left_hand_permanent),
        value,
    })
}

fn apply_role_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .role_draft
        .clone()
        .ok_or_else(|| "no role selected".to_owned())?;
    if draft.value.display_name.trim().is_empty() {
        return Err("role display name cannot be empty".to_owned());
    }
    if draft.value.action_animation.trim().is_empty() || draft.value.action_animation_variants == 0
    {
        return Err("role action animation and variant count must be defined".to_owned());
    }
    if draft.value.movement_speed_multiplier_per_thousand == 0
        || draft.value.experience_multiplier_per_thousand == 0
        || draft.value.base_health == 0
        || draft.value.base_movement_speed_milli_cells_per_second == 0
    {
        return Err("movement, experience, and health base values must be positive".to_owned());
    }
    let mut candidate = state.catalog.clone();
    candidate.roles.insert(draft.id, draft.value);
    commit_catalog_candidate(state, candidate)
}

fn duplicate_selected_role(state: &mut ToolState) -> Result<(), String> {
    let source = state
        .selected_role
        .as_ref()
        .and_then(|id| state.catalog.roles.get(id))
        .cloned()
        .ok_or_else(|| "no role selected".to_owned())?;
    let id =
        StableId::new(state.new_role_id.trim().to_owned()).map_err(|error| error.to_string())?;
    if state.catalog.roles.contains_key(&id) {
        return Err(format!("role {id} already exists"));
    }
    let display_name = state.new_role_name.trim();
    if display_name.is_empty() {
        return Err("new role name cannot be empty".to_owned());
    }
    let mut value = source;
    display_name.clone_into(&mut value.display_name);
    let mut candidate = state.catalog.clone();
    candidate.roles.insert(id.clone(), value);
    commit_catalog_candidate(state, candidate)?;
    state.selected_role = Some(id);
    refresh_role_draft(state);
    Ok(())
}

fn delete_selected_role(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_role
        .clone()
        .ok_or_else(|| "no role selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    candidate
        .roles
        .remove(&id)
        .ok_or_else(|| format!("missing role {id}"))?;
    commit_catalog_candidate(state, candidate)?;
    state.selected_role = state.catalog.roles.keys().next().cloned();
    refresh_role_draft(state);
    Ok(())
}

fn apply_foliage_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .foliage_draft
        .clone()
        .ok_or_else(|| "no foliage layer selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    let layer = candidate
        .foliage
        .iter_mut()
        .find(|layer| layer.id == draft.id)
        .ok_or_else(|| format!("missing foliage layer {}", draft.id))?;
    *layer = draft;
    commit_catalog_candidate(state, candidate)
}

fn duplicate_foliage_layer(state: &mut ToolState) -> Result<(), String> {
    let mut layer = state
        .foliage_draft
        .clone()
        .ok_or_else(|| "no foliage layer selected".to_owned())?;
    let id =
        StableId::new(state.new_foliage_id.trim().to_owned()).map_err(|error| error.to_string())?;
    if state.catalog.foliage.iter().any(|layer| layer.id == id) {
        return Err(format!("foliage layer {id} already exists"));
    }
    layer.id = id.clone();
    let mut candidate = state.catalog.clone();
    candidate.foliage.push(layer);
    commit_catalog_candidate(state, candidate)?;
    state.selected_foliage = Some(id);
    state.selected_foliage_variant = 0;
    refresh_foliage_draft(state);
    Ok(())
}

fn delete_foliage_layer(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_foliage
        .clone()
        .ok_or_else(|| "no foliage layer selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    let before = candidate.foliage.len();
    candidate.foliage.retain(|layer| layer.id != id);
    if candidate.foliage.len() == before {
        return Err(format!("missing foliage layer {id}"));
    }
    commit_catalog_candidate(state, candidate)?;
    state.selected_foliage = state.catalog.foliage.first().map(|layer| layer.id.clone());
    state.selected_foliage_variant = 0;
    refresh_foliage_draft(state);
    Ok(())
}

fn apply_resource_generation_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .resource_generation_draft
        .clone()
        .ok_or_else(|| "no resource generation layer selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    let layer = candidate
        .resource_generation
        .iter_mut()
        .find(|layer| layer.id == draft.id)
        .ok_or_else(|| format!("missing resource generation layer {}", draft.id))?;
    *layer = draft;
    commit_catalog_candidate(state, candidate)
}

fn duplicate_resource_generation_layer(state: &mut ToolState) -> Result<(), String> {
    let mut layer = state
        .resource_generation_draft
        .clone()
        .ok_or_else(|| "no resource generation layer selected".to_owned())?;
    let id = StableId::new(state.new_resource_generation_id.trim().to_owned())
        .map_err(|error| error.to_string())?;
    if state
        .catalog
        .resource_generation
        .iter()
        .any(|layer| layer.id == id)
    {
        return Err(format!("resource generation layer {id} already exists"));
    }
    let display_name = state.new_resource_generation_name.trim();
    if display_name.is_empty() {
        return Err("resource generation layer name cannot be empty".to_owned());
    }
    layer.id = id.clone();
    display_name.clone_into(&mut layer.display_name);
    let mut candidate = state.catalog.clone();
    candidate.resource_generation.push(layer);
    commit_catalog_candidate(state, candidate)?;
    state.selected_resource_generation = Some(id);
    refresh_resource_generation_draft(state);
    Ok(())
}

fn delete_resource_generation_layer(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_resource_generation
        .clone()
        .ok_or_else(|| "no resource generation layer selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    let before = candidate.resource_generation.len();
    candidate.resource_generation.retain(|layer| layer.id != id);
    if candidate.resource_generation.len() == before {
        return Err(format!("missing resource generation layer {id}"));
    }
    commit_catalog_candidate(state, candidate)?;
    state.selected_resource_generation = state
        .catalog
        .resource_generation
        .first()
        .map(|layer| layer.id.clone());
    refresh_resource_generation_draft(state);
    Ok(())
}

fn authoring_snapshot(state: &ToolState) -> AuthoringSnapshot {
    AuthoringSnapshot {
        catalog: state.catalog.clone(),
        technology_layout: state.technology_layout.clone(),
    }
}

fn push_authoring_undo(state: &mut ToolState, previous: AuthoringSnapshot) {
    state.undo_authoring.push(previous);
    state.redo_authoring.clear();
}

fn undo_authoring_edit(state: &mut ToolState) {
    if let Some(previous) = state.undo_authoring.pop() {
        let current = authoring_snapshot(state);
        state.redo_authoring.push(current);
        state.catalog = previous.catalog;
        state.technology_layout = previous.technology_layout;
        refresh_catalog_drafts(state);
        "Authoring edit undone".clone_into(&mut state.status);
    }
}

fn redo_authoring_edit(state: &mut ToolState) {
    if let Some(next) = state.redo_authoring.pop() {
        let current = authoring_snapshot(state);
        state.undo_authoring.push(current);
        state.catalog = next.catalog;
        state.technology_layout = next.technology_layout;
        refresh_catalog_drafts(state);
        "Authoring edit redone".clone_into(&mut state.status);
    }
}

fn commit_catalog_candidate(
    state: &mut ToolState,
    candidate: ContentCatalog,
) -> Result<(), String> {
    candidate.validate().map_err(|error| error.to_string())?;
    let previous = authoring_snapshot(state);
    let mut technology_layout = state.technology_layout.clone();
    technology_layout.reconcile(&candidate.technology);
    technology_layout
        .validate(&candidate.technology)
        .map_err(|error| error.to_string())?;
    push_authoring_undo(state, previous);
    state.catalog = candidate;
    state.technology_layout = technology_layout;
    refresh_catalog_drafts(state);
    Ok(())
}

fn create_technology_group(state: &mut ToolState) -> Result<(), String> {
    let id =
        StableId::new(state.new_group_id.trim().to_owned()).map_err(|error| error.to_string())?;
    let display_name = state.new_group_name.trim();
    if display_name.is_empty() {
        return Err("group name cannot be empty".to_owned());
    }
    if state.catalog.technology.groups.contains_key(&id) {
        return Err(format!("technology group {id} already exists"));
    }
    let mut candidate = state.catalog.clone();
    candidate.technology.groups.insert(
        id.clone(),
        TechGroup {
            display_name: display_name.to_owned(),
            nodes: Vec::new(),
        },
    );
    commit_catalog_candidate(state, candidate)?;
    state.selected_group = Some(id);
    state.technology_graph_view.request_fit();
    Ok(())
}

fn delete_selected_technology_group(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_group
        .clone()
        .ok_or_else(|| "no technology group selected".to_owned())?;
    let group = state
        .catalog
        .technology
        .groups
        .get(&id)
        .ok_or_else(|| format!("missing technology group {id}"))?;
    if !group.nodes.is_empty()
        || state
            .catalog
            .technology
            .nodes
            .values()
            .any(|node| node.group.as_ref() == Some(&id))
    {
        return Err("move or delete every node before deleting its group".to_owned());
    }
    let mut candidate = state.catalog.clone();
    candidate.technology.groups.remove(&id);
    commit_catalog_candidate(state, candidate)?;
    state.selected_group = state.catalog.technology.groups.keys().next().cloned();
    state.technology_graph_view.request_fit();
    Ok(())
}

fn create_technology_node(state: &mut ToolState) -> Result<(), String> {
    let group_id = state
        .selected_group
        .clone()
        .ok_or_else(|| "select a technology group first".to_owned())?;
    let id = StableId::new(state.new_technology_id.trim().to_owned())
        .map_err(|error| error.to_string())?;
    let display_name = state.new_technology_name.trim();
    if display_name.is_empty() {
        return Err("technology name cannot be empty".to_owned());
    }
    if state.catalog.technology.nodes.contains_key(&id) {
        return Err(format!("technology {id} already exists"));
    }
    let mut candidate = state.catalog.clone();
    candidate.technology.nodes.insert(
        id.clone(),
        TechNode {
            display_name: display_name.to_owned(),
            group: Some(group_id.clone()),
            ..TechNode::default()
        },
    );
    candidate
        .technology
        .groups
        .get_mut(&group_id)
        .ok_or_else(|| format!("missing technology group {group_id}"))?
        .nodes
        .push(id.clone());
    commit_catalog_candidate(state, candidate)?;
    state.technology_draft = technology_draft(&state.catalog, &id);
    state.technology_graph_view.request_focus(id);
    Ok(())
}

fn delete_selected_technology_node(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .technology_draft
        .as_ref()
        .map(|draft| draft.id.clone())
        .ok_or_else(|| "no technology selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    candidate
        .technology
        .nodes
        .remove(&id)
        .ok_or_else(|| format!("missing technology {id}"))?;
    for node in candidate.technology.nodes.values_mut() {
        node.prerequisites.retain(|reference| reference != &id);
        node.unlocks.retain(|reference| reference != &id);
    }
    for group in candidate.technology.groups.values_mut() {
        group.nodes.retain(|reference| reference != &id);
    }
    commit_catalog_candidate(state, candidate)?;
    state.technology_draft = None;
    Ok(())
}

fn connect_technology_nodes(
    state: &mut ToolState,
    prerequisite: &StableId,
    dependent: &StableId,
) -> Result<(), String> {
    if prerequisite == dependent {
        return Err("a technology cannot require itself".to_owned());
    }
    let mut candidate = state.catalog.clone();
    if !candidate.technology.nodes.contains_key(prerequisite) {
        return Err(format!("missing prerequisite technology {prerequisite}"));
    }
    let node = candidate
        .technology
        .nodes
        .get_mut(dependent)
        .ok_or_else(|| format!("missing dependent technology {dependent}"))?;
    if node.prerequisites.contains(prerequisite) {
        return Err("that prerequisite connection already exists".to_owned());
    }
    node.prerequisites.push(prerequisite.clone());
    commit_catalog_candidate(state, candidate)
}

fn disconnect_technology_nodes(
    state: &mut ToolState,
    prerequisite: &StableId,
    dependent: &StableId,
) -> Result<(), String> {
    let mut candidate = state.catalog.clone();
    let node = candidate
        .technology
        .nodes
        .get_mut(dependent)
        .ok_or_else(|| format!("missing dependent technology {dependent}"))?;
    let previous = node.prerequisites.len();
    node.prerequisites.retain(|value| value != prerequisite);
    if node.prerequisites.len() == previous {
        return Err("that prerequisite connection does not exist".to_owned());
    }
    commit_catalog_candidate(state, candidate)
}

#[cfg(test)]
mod runtime_console_tests {
    use super::*;

    #[test]
    fn runtime_actions_sequence_after_latest_acknowledgement() {
        let directory = tempfile::tempdir().unwrap();
        let mut state = ToolState {
            runtime_console: RuntimeConsoleStore::new(directory.path()),
            runtime_status: Some(RuntimeConsoleStatus {
                schema_version: stream_town_domain::CURRENT_RUNTIME_CONSOLE_SCHEMA,
                updated_unix_millis: u64::MAX,
                last_processed_sequence: 41,
                ..RuntimeConsoleStatus::default()
            }),
            ..ToolState::default()
        };

        send_runtime_action(&mut state, RuntimeConsoleAction::Save).unwrap();

        let request = state.runtime_console.read_request().unwrap().unwrap();
        assert_eq!(request.sequence, 42);
        assert_eq!(request.action, RuntimeConsoleAction::Save);
    }

    #[test]
    fn stale_runtime_status_is_not_treated_as_attached() {
        let stale = RuntimeConsoleStatus {
            schema_version: stream_town_domain::CURRENT_RUNTIME_CONSOLE_SCHEMA,
            updated_unix_millis: 1,
            ..RuntimeConsoleStatus::default()
        };
        assert!(!runtime_console_attached(Some(&stale)));
        let live = RuntimeConsoleStatus {
            updated_unix_millis: u64::MAX,
            ..stale
        };
        assert!(runtime_console_attached(Some(&live)));
    }
}

fn refresh_technology_draft(state: &mut ToolState) {
    let selected = state
        .technology_draft
        .as_ref()
        .map(|draft| draft.id.clone());
    state.technology_draft = selected
        .as_ref()
        .and_then(|id| technology_draft(&state.catalog, id));
}

fn refresh_objective_draft(state: &mut ToolState) {
    if state
        .selected_objective
        .as_ref()
        .is_none_or(|id| !state.catalog.objectives.contains_key(id))
    {
        state.selected_objective = state.catalog.objectives.keys().next().cloned();
    }
    state.objective_draft = state.selected_objective.as_ref().and_then(|id| {
        state
            .catalog
            .objectives
            .get(id)
            .cloned()
            .map(|value| ObjectiveDraft {
                id: id.clone(),
                value,
            })
    });
}

fn refresh_role_draft(state: &mut ToolState) {
    if state
        .selected_role
        .as_ref()
        .is_none_or(|id| !state.catalog.roles.contains_key(id))
    {
        state.selected_role = state.catalog.roles.keys().next().cloned();
    }
    state.role_draft = state
        .selected_role
        .as_ref()
        .and_then(|id| role_draft(&state.catalog, id));
    state.role_preview_animation = state.role_draft.as_ref().and_then(|draft| {
        matching_role_animation_state(&state.presentation, &draft.value.action_animation)
    });
}

fn refresh_foliage_draft(state: &mut ToolState) {
    if state
        .selected_foliage
        .as_ref()
        .is_none_or(|id| !state.catalog.foliage.iter().any(|layer| &layer.id == id))
    {
        state.selected_foliage = state.catalog.foliage.first().map(|layer| layer.id.clone());
    }
    state.foliage_draft = state.selected_foliage.as_ref().and_then(|id| {
        state
            .catalog
            .foliage
            .iter()
            .find(|layer| &layer.id == id)
            .cloned()
    });
    state.selected_foliage_variant = state.foliage_draft.as_ref().map_or(0, |layer| {
        state
            .selected_foliage_variant
            .min(layer.variants.len().saturating_sub(1))
    });
}

fn refresh_resource_generation_draft(state: &mut ToolState) {
    if state
        .selected_resource_generation
        .as_ref()
        .is_none_or(|id| {
            !state
                .catalog
                .resource_generation
                .iter()
                .any(|layer| &layer.id == id)
        })
    {
        state.selected_resource_generation = state
            .catalog
            .resource_generation
            .first()
            .map(|layer| layer.id.clone());
    }
    state.resource_generation_draft = state.selected_resource_generation.as_ref().and_then(|id| {
        state
            .catalog
            .resource_generation
            .iter()
            .find(|layer| &layer.id == id)
            .cloned()
    });
}

fn refresh_enemy_camp_generation_draft(state: &mut ToolState) {
    if state
        .selected_enemy_camp_generation
        .as_ref()
        .is_none_or(|id| {
            !state
                .catalog
                .enemy_camp_generation
                .iter()
                .any(|layer| &layer.id == id)
        })
    {
        state.selected_enemy_camp_generation = state
            .catalog
            .enemy_camp_generation
            .first()
            .map(|layer| layer.id.clone());
    }
    state.enemy_camp_generation_draft =
        state
            .selected_enemy_camp_generation
            .as_ref()
            .and_then(|id| {
                state
                    .catalog
                    .enemy_camp_generation
                    .iter()
                    .find(|layer| &layer.id == id)
                    .cloned()
            });
}

fn refresh_catalog_drafts(state: &mut ToolState) {
    refresh_technology_draft(state);
    refresh_objective_draft(state);
    refresh_role_draft(state);
    refresh_building_draft(state);
    refresh_foliage_draft(state);
    refresh_resource_generation_draft(state);
    refresh_enemy_camp_generation_draft(state);
    state.generated_world = None;
}

fn apply_enemy_camp_generation_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .enemy_camp_generation_draft
        .clone()
        .ok_or_else(|| "no enemy camp generation layer selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    let layer = candidate
        .enemy_camp_generation
        .iter_mut()
        .find(|layer| layer.id == draft.id)
        .ok_or_else(|| format!("missing enemy camp layer {}", draft.id))?;
    *layer = draft;
    commit_catalog_candidate(state, candidate)
}

fn duplicate_enemy_camp_generation_layer(state: &mut ToolState) -> Result<(), String> {
    let mut layer = state
        .enemy_camp_generation_draft
        .clone()
        .ok_or_else(|| "no enemy camp generation layer selected".to_owned())?;
    let id = StableId::new(state.new_enemy_camp_generation_id.trim().to_owned())
        .map_err(|error| error.to_string())?;
    if state
        .catalog
        .enemy_camp_generation
        .iter()
        .any(|layer| layer.id == id)
    {
        return Err(format!("enemy camp generation layer {id} already exists"));
    }
    layer.id = id.clone();
    let mut candidate = state.catalog.clone();
    candidate.enemy_camp_generation.push(layer);
    commit_catalog_candidate(state, candidate)?;
    state.selected_enemy_camp_generation = Some(id);
    refresh_enemy_camp_generation_draft(state);
    Ok(())
}

fn delete_enemy_camp_generation_layer(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_enemy_camp_generation
        .clone()
        .ok_or_else(|| "no enemy camp generation layer selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    let before = candidate.enemy_camp_generation.len();
    candidate
        .enemy_camp_generation
        .retain(|layer| layer.id != id);
    if candidate.enemy_camp_generation.len() == before {
        return Err(format!("missing enemy camp generation layer {id}"));
    }
    commit_catalog_candidate(state, candidate)?;
    state.selected_enemy_camp_generation = state
        .catalog
        .enemy_camp_generation
        .first()
        .map(|layer| layer.id.clone());
    refresh_enemy_camp_generation_draft(state);
    Ok(())
}

fn apply_objective_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .objective_draft
        .clone()
        .ok_or_else(|| "no vote requirement selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    if !candidate.objectives.contains_key(&draft.id) {
        return Err(format!("missing vote requirement {}", draft.id));
    }
    candidate.objectives.insert(draft.id, draft.value);
    commit_catalog_candidate(state, candidate)
}

fn duplicate_selected_objective(state: &mut ToolState) -> Result<(), String> {
    let source = state
        .objective_draft
        .as_ref()
        .map(|draft| draft.value.clone())
        .ok_or_else(|| "no vote requirement selected".to_owned())?;
    let id = StableId::new(state.new_objective_id.trim().to_owned())
        .map_err(|error| error.to_string())?;
    if state.catalog.objectives.contains_key(&id) {
        return Err(format!("vote requirement {id} already exists"));
    }
    let mut candidate = state.catalog.clone();
    candidate.objectives.insert(id.clone(), source);
    commit_catalog_candidate(state, candidate)?;
    state.selected_objective = Some(id);
    refresh_objective_draft(state);
    Ok(())
}

fn delete_selected_objective(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_objective
        .clone()
        .ok_or_else(|| "no vote requirement selected".to_owned())?;
    if state
        .catalog
        .technology
        .nodes
        .values()
        .any(|node| node.objectives.contains(&id))
    {
        return Err("remove this requirement from every technology before deleting it".to_owned());
    }
    let mut candidate = state.catalog.clone();
    candidate
        .objectives
        .remove(&id)
        .ok_or_else(|| format!("missing vote requirement {id}"))?;
    commit_catalog_candidate(state, candidate)?;
    state.selected_objective = state.catalog.objectives.keys().next().cloned();
    refresh_objective_draft(state);
    Ok(())
}

fn apply_technology_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .technology_draft
        .clone()
        .ok_or_else(|| "no technology selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    if !candidate.technology.nodes.contains_key(&draft.id) {
        return Err(format!("missing technology {}", draft.id));
    }
    let group = draft.value.group.clone();
    candidate
        .technology
        .nodes
        .insert(draft.id.clone(), draft.value);
    for group in candidate.technology.groups.values_mut() {
        group.nodes.retain(|reference| reference != &draft.id);
    }
    if let Some(group_id) = &group {
        candidate
            .technology
            .groups
            .get_mut(group_id)
            .ok_or_else(|| format!("missing technology group {group_id}"))?
            .nodes
            .push(draft.id.clone());
    }
    commit_catalog_candidate(state, candidate)
}

fn draw_world_preview(
    ui: &mut egui::Ui,
    world: &GeneratedWorld,
    layer: WorldPreviewLayer,
    selected_foliage: Option<&StableId>,
    selected_resource_target: Option<&StableId>,
    enemy_camp_layer: Option<&EnemyCampGenerationDef>,
    water_level_centimetres: i16,
) {
    let width = world.navigation.width();
    let height = world.navigation.height();
    let desired_width = ui.available_width().min(720.0);
    let desired = egui::vec2(
        desired_width,
        desired_width * f32::from(height) / f32::from(width),
    );
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let stride = usize::from(width.max(height)).div_ceil(96).max(1);
    let stride_u16 = u16::try_from(stride).unwrap_or(u16::MAX);
    let (minimum_height, maximum_height) = (0..height)
        .flat_map(|z| (0..width).map(move |x| GridPos { x, z }))
        .filter_map(|position| world.navigation.height_at(position))
        .fold((i16::MAX, i16::MIN), |(minimum, maximum), value| {
            (minimum.min(value), maximum.max(value))
        });
    for z in (0..height).step_by(stride) {
        for x in (0..width).step_by(stride) {
            let position = GridPos { x, z };
            let left = rect.left() + f32::from(x) * rect.width() / f32::from(width);
            let right = rect.left()
                + f32::from(x.saturating_add(stride_u16).min(width)) * rect.width()
                    / f32::from(width);
            let top = rect.top() + f32::from(z) * rect.height() / f32::from(height);
            let bottom = rect.top()
                + f32::from(z.saturating_add(stride_u16).min(height)) * rect.height()
                    / f32::from(height);
            let cell_height = world.navigation.height_at(position).unwrap_or_default();
            let color = match layer {
                WorldPreviewLayer::Elevation => {
                    if cell_height <= water_level_centimetres {
                        egui::Color32::from_rgb(34, 91, 145)
                    } else {
                        let range =
                            f32::from(maximum_height.saturating_sub(minimum_height)).max(1.0);
                        let normalized =
                            f32::from(cell_height.saturating_sub(minimum_height)) / range;
                        terrain_preview_color(normalized)
                    }
                }
                WorldPreviewLayer::Navigation => {
                    if world.navigation.is_walkable(position) {
                        egui::Color32::from_rgb(70, 145, 83)
                    } else {
                        egui::Color32::from_rgb(42, 52, 62)
                    }
                }
                WorldPreviewLayer::LandResources
                | WorldPreviewLayer::WaterResources
                | WorldPreviewLayer::Foliage
                | WorldPreviewLayer::EnemyCamps => {
                    if cell_height <= water_level_centimetres {
                        egui::Color32::from_rgb(29, 70, 105)
                    } else if world.navigation.is_walkable(position) {
                        egui::Color32::from_rgb(54, 93, 59)
                    } else {
                        egui::Color32::from_rgb(51, 57, 54)
                    }
                }
            };
            ui.painter().rect_filled(
                egui::Rect::from_min_max(egui::pos2(left, top), egui::pos2(right, bottom)),
                0.0,
                color,
            );
        }
    }
    if matches!(
        layer,
        WorldPreviewLayer::LandResources | WorldPreviewLayer::WaterResources
    ) {
        for resource in world.resources.iter().filter(|resource| {
            let water = resource.target_kind.as_str() == "target:fish";
            (layer == WorldPreviewLayer::WaterResources) == water
        }) {
            let highlighted =
                selected_resource_target.is_none_or(|target| target == &resource.target_kind);
            let mut color = match (resource.kind.as_str(), resource.target_kind.as_str()) {
                (_, "target:fish") => egui::Color32::from_rgb(83, 207, 229),
                (value, _) if value.contains("wood") => egui::Color32::from_rgb(46, 185, 76),
                (value, _) if value.contains("ore") => egui::Color32::from_rgb(190, 196, 207),
                (value, _) if value.contains("food") => egui::Color32::from_rgb(219, 82, 81),
                _ => egui::Color32::GOLD,
            };
            if !highlighted {
                color = egui::Color32::from_rgba_unmultiplied(
                    color.r() / 2,
                    color.g() / 2,
                    color.b() / 2,
                    100,
                );
            }
            ui.painter().circle_filled(
                preview_grid_point(rect, width, height, resource.position),
                if highlighted { 2.5 } else { 1.2 },
                color,
            );
        }
    }
    if layer == WorldPreviewLayer::Foliage {
        for foliage in &world.foliage {
            let highlighted = selected_foliage.is_none_or(|id| id == &foliage.layer);
            let color = match (foliage.habitat, highlighted) {
                (FoliageHabitat::Land, true) => egui::Color32::from_rgb(115, 235, 100),
                (FoliageHabitat::Underwater, true) => egui::Color32::from_rgb(92, 220, 225),
                (_, false) => egui::Color32::from_rgba_unmultiplied(145, 145, 145, 90),
            };
            ui.painter().circle_filled(
                preview_grid_point(rect, width, height, foliage.position),
                if highlighted { 1.8 } else { 1.0 },
                color,
            );
        }
    }
    if layer == WorldPreviewLayer::EnemyCamps
        && let Some(camp) = enemy_camp_layer
    {
        for z in 0..height {
            for x in 0..width {
                let horizontal_milli = (i32::from(x) * 2 + 1 - i32::from(width))
                    .unsigned_abs()
                    .saturating_mul(500);
                let vertical_milli = (i32::from(z) * 2 + 1 - i32::from(height))
                    .unsigned_abs()
                    .saturating_mul(500);
                let squared_distance =
                    u64::from(horizontal_milli).pow(2) + u64::from(vertical_milli).pow(2);
                let minimum_squared =
                    u64::from(camp.minimum_distance_from_centre_milli_cells).pow(2);
                let in_bounds = horizontal_milli >= camp.minimum_absolute_offset_milli_cells[0]
                    && vertical_milli >= camp.minimum_absolute_offset_milli_cells[1]
                    && horizontal_milli <= camp.maximum_absolute_offset_milli_cells[0]
                    && vertical_milli <= camp.maximum_absolute_offset_milli_cells[1]
                    && squared_distance >= minimum_squared;
                if in_bounds && world.navigation.is_walkable(GridPos { x, z }) {
                    ui.painter().circle_filled(
                        preview_grid_point(rect, width, height, GridPos { x, z }),
                        1.1,
                        egui::Color32::from_rgba_unmultiplied(230, 92, 76, 145),
                    );
                }
            }
        }
    }
    ui.label(match layer {
        WorldPreviewLayer::Elevation => "Blue is at/below water level; green through stone encodes terrain elevation.",
        WorldPreviewLayer::Navigation => "Green cells are walkable; charcoal cells are blocked.",
        WorldPreviewLayer::LandResources => "Green, grey, and red markers identify generated trees, ore, and berry bushes.",
        WorldPreviewLayer::WaterResources => "Cyan markers identify generated shoreline-water resources such as fish.",
        WorldPreviewLayer::Foliage => "Bright markers belong to the selected foliage layer; dim markers belong to other layers.",
        WorldPreviewLayer::EnemyCamps => "Red cells satisfy the selected camp layer's absolute-offset and centre-distance constraints; the runtime still applies spacing, occupancy, and maximum-count rules.",
    });
}

fn preview_grid_point(rect: egui::Rect, width: u16, height: u16, position: GridPos) -> egui::Pos2 {
    egui::pos2(
        rect.left() + (f32::from(position.x) + 0.5) * rect.width() / f32::from(width),
        rect.top() + (f32::from(position.z) + 0.5) * rect.height() / f32::from(height),
    )
}

fn terrain_preview_color(value: f32) -> egui::Color32 {
    let value = value.clamp(0.0, 1.0);
    if value < 0.55 {
        let t = value / 0.55;
        preview_lerp_color(
            egui::Color32::from_rgb(60, 113, 63),
            egui::Color32::from_rgb(157, 154, 91),
            t,
        )
    } else {
        let t = (value - 0.55) / 0.45;
        preview_lerp_color(
            egui::Color32::from_rgb(157, 154, 91),
            egui::Color32::from_rgb(218, 220, 218),
            t,
        )
    }
}

fn preview_lerp_color(from: egui::Color32, to: egui::Color32, t: f32) -> egui::Color32 {
    let lerp = |start: u8, end: u8| {
        f32::from(start) / 255.0 + (f32::from(end) - f32::from(start)) * t / 255.0
    };
    egui::Rgba::from_rgb(
        lerp(from.r(), to.r()),
        lerp(from.g(), to.g()),
        lerp(from.b(), to.b()),
    )
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_tabs_are_focused_authoring_workflows() {
        assert_eq!(
            ToolTab::ALL.map(ToolTab::label),
            [
                "Migration",
                "Game Authority",
                "Models + Assets",
                "Buildings",
                "Roles",
                "Technology",
                "Terrain",
                "Music",
                "World + Nav",
                "Validation",
            ]
        );
        assert_eq!(
            AssetEditorSection::ALL.map(AssetEditorSection::label),
            ["Models", "Textures", "Materials", "Animations"]
        );
    }

    #[test]
    fn character_model_choices_include_converted_hierarchy_nodes() {
        let state = ToolState::default();
        let choices = equipment_node_choices(&state.catalog, &state.presentation);
        assert!(choices.iter().any(|node| node == "Body_Blacksmith_Slim"));
        assert!(choices.iter().any(|node| node == "Body_Logger_Feminine"));
        assert!(choices.iter().all(|node| !node.trim().is_empty()));
    }

    #[test]
    fn discovered_model_assets_are_project_relative_glbs() {
        let assets = discover_model_assets();
        assert!(!assets.is_empty());
        assert!(assets.iter().all(|path| {
            std::path::Path::new(path)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
        }));
        assert!(
            assets
                .iter()
                .all(|path| !std::path::Path::new(path).is_absolute() && !path.contains('\\'))
        );
    }

    #[test]
    fn texture_discovery_and_gltf_metadata_are_typed_project_assets() {
        let textures = discover_texture_assets();
        assert!(!textures.is_empty());
        assert!(textures.iter().all(|path| {
            path.starts_with("migrated/textures/")
                && !path.contains('\\')
                && matches!(
                    std::path::Path::new(path)
                        .extension()
                        .unwrap()
                        .to_string_lossy()
                        .to_ascii_lowercase()
                        .as_str(),
                    "png" | "tga" | "jpg" | "jpeg"
                )
        }));
        let metadata =
            inspect_gltf_asset("migrated/models/Models/Characters/Characters.glb").unwrap();
        assert!(!metadata.nodes.is_empty());
        assert!(!metadata.materials.is_empty());
        assert!(!metadata.animations.is_empty());
    }

    #[test]
    fn preview_material_resolution_matches_runtime_binding_precedence() {
        let mut assets = Assets::<StandardMaterial>::default();
        let fallback = assets.add(StandardMaterial::default());
        let model = assets.add(StandardMaterial::default());
        let renderer = assets.add(StandardMaterial::default());
        let overrides = PreviewMaterialOverrides {
            fallback: Some(fallback.clone()),
            model_materials: BTreeMap::from([("MainMaterial".to_owned(), model.clone())]),
            renderer_materials: vec![PreviewRendererMaterialBinding {
                target_path: "Root/Body".to_owned(),
                materials: BTreeMap::from([("MainMaterial".to_owned(), renderer.clone())]),
            }],
        };

        let resolved = resolved_preview_material(
            &overrides,
            "Imported/Root/Body/Primitive0",
            Some("Body"),
            Some("MainMaterial"),
        )
        .unwrap();
        assert_eq!(resolved, &renderer);
        assert_eq!(
            resolved_preview_material(
                &overrides,
                "Unmatched/Primitive0",
                None,
                Some("MainMaterial")
            ),
            Some(&model)
        );
        assert_eq!(
            resolved_preview_material(&overrides, "Unmatched", None, None),
            Some(&fallback)
        );
    }

    #[test]
    fn preview_camera_starts_in_front_and_pans_in_camera_space() {
        let controls = ModelPreviewControls::default();
        let offset = preview_camera_offset(controls.yaw, controls.pitch, controls.distance);
        assert!(
            offset.z > 0.0,
            "the default camera must view +Z-facing models from the front"
        );

        let front_pan = preview_pan_delta(0.0, 0.0, 1.0, 0.0, 1.0);
        let side_pan = preview_pan_delta(std::f32::consts::FRAC_PI_2, 0.0, 1.0, 0.0, 1.0);
        assert!(front_pan.x < -0.99 && front_pan.z.abs() < 0.01);
        assert!(side_pan.z > 0.99 && side_pan.x.abs() < 0.01);
    }

    #[test]
    fn presentation_asset_crud_stays_valid_and_reference_safe() {
        let mut state = ToolState::default();

        state.new_texture_id = "texture:test_authoring".to_owned();
        state.new_texture_name = "Test Texture".to_owned();
        state.new_texture_asset = state.discovered_texture_assets[0].clone();
        create_texture_definition(&mut state).unwrap();
        let texture = StableId::new("texture:test_authoring").unwrap();
        assert!(state.presentation.textures.contains_key(&texture));
        delete_selected_texture(&mut state).unwrap();

        state.new_material_id = "material:test_authoring".to_owned();
        state.new_material_name = "Test Material".to_owned();
        create_material_definition(&mut state).unwrap();
        let material = StableId::new("material:test_authoring").unwrap();
        assert!(state.presentation.materials.contains_key(&material));
        delete_selected_material(&mut state).unwrap();

        state.new_clip_id = "clip:test_authoring".to_owned();
        state.new_clip_name = "Test Clip".to_owned();
        state.new_clip_asset = "migrated/models/Models/Characters/Characters.glb".to_owned();
        create_animation_clip(&mut state).unwrap();
        let clip = StableId::new("clip:test_authoring").unwrap();
        assert!(state.presentation.clips.contains_key(&clip));
        delete_selected_animation_clip(&mut state).unwrap();

        state.new_controller_id = "animation_controller:test_authoring".to_owned();
        state.new_controller_name = "Test Controller".to_owned();
        create_animation_controller(&mut state).unwrap();
        let controller = StableId::new("animation_controller:test_authoring").unwrap();
        assert!(state.presentation.controllers.contains_key(&controller));
        delete_selected_animation_controller(&mut state).unwrap();
        state.presentation.validate().unwrap();
    }

    #[test]
    fn model_archetype_and_variant_lifecycle_remains_valid() {
        let mut state = ToolState::default();
        state.new_archetype_id = "archetype:test:model".to_owned();
        state.new_archetype_name = "Test Model".to_owned();
        state.new_archetype_asset = state.discovered_model_assets[0].clone();

        create_model_archetype(&mut state).unwrap();
        let id = StableId::new("archetype:test:model").unwrap();
        assert_eq!(state.catalog.archetypes[&id].scenes.len(), 1);
        state.new_archetype_asset = state.discovered_model_assets[1].clone();
        add_archetype_scene(&mut state, &id).unwrap();
        assert_eq!(state.catalog.archetypes[&id].scenes.len(), 2);
        remove_archetype_scene(&mut state, &id, 1).unwrap();
        delete_model_archetype(&mut state, &id).unwrap();
        assert!(!state.catalog.archetypes.contains_key(&id));
        state.catalog.validate().unwrap();
    }

    #[test]
    fn technology_editor_rejects_cycles_without_mutating_catalog() {
        let mut state = ToolState::default();
        let (node_id, prerequisite) = state
            .catalog
            .technology
            .nodes
            .iter()
            .find_map(|(id, node)| {
                node.prerequisites
                    .first()
                    .map(|parent| (id.clone(), parent.clone()))
            })
            .expect("shipping graph has an edge");
        state.technology_draft = technology_draft(&state.catalog, &prerequisite);
        state.technology_draft.as_mut().unwrap().value.prerequisites = vec![node_id];
        let before = state.catalog.clone();
        assert!(apply_technology_draft(&mut state).is_err());
        assert_eq!(state.catalog, before);
    }

    #[test]
    fn technology_socket_connection_rejects_cycles_without_mutation() {
        let mut state = ToolState::default();
        let (dependent, prerequisite) = state
            .catalog
            .technology
            .nodes
            .iter()
            .find_map(|(id, node)| {
                node.prerequisites
                    .first()
                    .map(|parent| (id.clone(), parent.clone()))
            })
            .expect("shipping graph has an edge");
        let before = state.catalog.clone();

        assert!(connect_technology_nodes(&mut state, &dependent, &prerequisite).is_err());
        assert_eq!(state.catalog, before);
    }

    #[test]
    fn technology_connections_can_be_removed_and_restored() {
        let mut state = ToolState::default();
        let (dependent, prerequisite) = state
            .catalog
            .technology
            .nodes
            .iter()
            .find_map(|(id, node)| {
                node.prerequisites
                    .first()
                    .map(|parent| (id.clone(), parent.clone()))
            })
            .expect("shipping graph has an edge");
        disconnect_technology_nodes(&mut state, &prerequisite, &dependent).unwrap();
        assert!(
            !state.catalog.technology.nodes[&dependent]
                .prerequisites
                .contains(&prerequisite)
        );
        connect_technology_nodes(&mut state, &prerequisite, &dependent).unwrap();
        assert!(
            state.catalog.technology.nodes[&dependent]
                .prerequisites
                .contains(&prerequisite)
        );
    }

    #[test]
    fn vote_requirement_lifecycle_is_typed_validated_and_reference_safe() {
        let mut state = ToolState::default();
        let original_count = state.catalog.objectives.len();
        state.new_objective_id = "objective:test".to_owned();
        duplicate_selected_objective(&mut state).unwrap();
        let id = StableId::new("objective:test").unwrap();
        assert!(state.catalog.objectives.contains_key(&id));

        state
            .objective_draft
            .as_mut()
            .unwrap()
            .value
            .required_amount = 0;
        let before = state.catalog.clone();
        assert!(apply_objective_draft(&mut state).is_err());
        assert_eq!(state.catalog, before);

        refresh_objective_draft(&mut state);
        delete_selected_objective(&mut state).unwrap();
        assert_eq!(state.catalog.objectives.len(), original_count);
        state.catalog.validate().unwrap();
    }

    #[test]
    fn technology_authoring_create_move_delete_and_undo_remains_valid() {
        let mut state = ToolState {
            new_group_id: "technology_group:test".to_owned(),
            new_group_name: "Test Group".to_owned(),
            ..ToolState::default()
        };
        create_technology_group(&mut state).unwrap();
        let group_id = StableId::new("technology_group:test").unwrap();
        assert_eq!(state.selected_group.as_ref(), Some(&group_id));

        state.new_technology_id = "technology:test".to_owned();
        state.new_technology_name = "Test Technology".to_owned();
        create_technology_node(&mut state).unwrap();
        let node_id = StableId::new("technology:test").unwrap();
        assert_eq!(
            state.catalog.technology.nodes[&node_id].group.as_ref(),
            Some(&group_id)
        );
        assert!(
            state.catalog.technology.groups[&group_id]
                .nodes
                .contains(&node_id)
        );
        assert!(delete_selected_technology_group(&mut state).is_err());

        delete_selected_technology_node(&mut state).unwrap();
        assert!(!state.catalog.technology.nodes.contains_key(&node_id));
        delete_selected_technology_group(&mut state).unwrap();
        assert!(!state.catalog.technology.groups.contains_key(&group_id));
        state.catalog.validate().unwrap();

        let previous = state.undo_authoring.pop().unwrap();
        state.catalog = previous.catalog;
        state.technology_layout = previous.technology_layout;
        assert!(state.catalog.technology.groups.contains_key(&group_id));
        state.catalog.validate().unwrap();
        state
            .technology_layout
            .validate(&state.catalog.technology)
            .unwrap();
    }

    #[test]
    fn technology_editor_preserves_the_complete_effect_record() {
        let mut state = ToolState::default();
        let (id, before) = state
            .catalog
            .technology
            .nodes
            .iter()
            .find(|(_, node)| {
                !node.building_level_caps.is_empty()
                    || !node.global_stat_boost_percent.is_empty()
                    || !node.role_stat_boost_percent.is_empty()
            })
            .map(|(id, node)| (id.clone(), node.clone()))
            .expect("shipping technology graph has authored effects");
        state.technology_draft = technology_draft(&state.catalog, &id);
        state
            .technology_draft
            .as_mut()
            .unwrap()
            .value
            .description
            .push_str(" [edited]");

        apply_technology_draft(&mut state).unwrap();

        let mut expected = before;
        expected.description.push_str(" [edited]");
        assert_eq!(state.catalog.technology.nodes[&id], expected);
    }

    #[test]
    fn building_editor_preserves_the_complete_template_record() {
        let mut state = ToolState::default();
        let (id, before) = state
            .catalog
            .buildings
            .iter()
            .find(|(_, building)| {
                !building.model_handlers.is_empty()
                    && (!building.storage.is_empty() || building.station.is_some())
            })
            .map(|(id, building)| (id.clone(), building.clone()))
            .expect("shipping catalog has a fully authored building");
        state.selected_building = Some(id.clone());
        state.building_draft = building_draft(&state.catalog, &id);
        state
            .building_draft
            .as_mut()
            .unwrap()
            .value
            .display_name
            .push_str(" Edited");

        apply_building_draft(&mut state).unwrap();

        let mut expected = before;
        expected.display_name.push_str(" Edited");
        assert_eq!(state.catalog.buildings[&id], expected);
    }

    #[test]
    fn logical_footprint_sync_updates_the_runtime_archetype_record() {
        let mut state = ToolState::default();
        let building = state.catalog.buildings.keys().next().unwrap().clone();
        state.selected_building = Some(building.clone());
        state.building_draft = building_draft(&state.catalog, &building);
        let archetype = state
            .building_draft
            .as_ref()
            .unwrap()
            .value
            .archetype
            .clone();
        state.building_draft.as_mut().unwrap().value.footprint = [7, 3];

        apply_building_draft(&mut state).unwrap();

        assert_eq!(state.catalog.buildings[&building].footprint, [7, 3]);
        assert_eq!(state.catalog.archetypes[&archetype].footprint, [7, 3]);
        state.building_draft.as_mut().unwrap().value.footprint = [0, 3];
        assert!(apply_building_draft(&mut state).is_err());
    }

    #[test]
    fn role_preview_uses_shipping_rig_animation_and_composition_rules() {
        let state = ToolState::default();
        let logger = state
            .catalog
            .roles
            .get(&StableId::new("role:logger").unwrap())
            .unwrap();
        let animation_state =
            matching_role_animation_state(&state.presentation, &logger.action_animation).unwrap();
        let (asset_path, animation_index, _) =
            role_preview_animation_request(&state.presentation, &animation_state).unwrap();
        assert_eq!(asset_path, PLAYER_ANIMATED_MODEL_PATH);
        assert_eq!(
            animation_index, 24,
            "logger preview should use CharacterWoodCutting"
        );

        let equipment = logger.equipment.as_ref().unwrap();
        let idle = role_preview_visible_nodes(
            logger,
            2,
            false,
            Some("Eyes_Normal"),
            Some("Hair_Short_Normal"),
            None,
        );
        assert!(idle.contains(&equipment.body_nodes[2]));
        if let Some(left_hand) = equipment.left_hand_node.as_ref() {
            assert_eq!(idle.contains(left_hand), equipment.left_hand_permanent);
            let carrying = role_preview_visible_nodes(
                logger,
                2,
                true,
                Some("Eyes_Normal"),
                Some("Hair_Short_Normal"),
                None,
            );
            assert!(carrying.contains(left_hand));
        }
    }

    #[test]
    fn technology_catalog_save_is_atomic_validated_and_round_trips() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("catalog.ron");
        let catalog = ToolState::default().catalog;

        save_content_catalog(&catalog, path.to_str().unwrap()).unwrap();
        save_content_catalog(&catalog, path.to_str().unwrap()).unwrap();

        let reloaded: ContentCatalog = ron::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(reloaded, catalog);
        assert!(PathBuf::from(format!("{}.bak", path.display())).is_file());
        assert!(!PathBuf::from(format!("{}.tmp", path.display())).exists());
    }

    #[test]
    fn technology_layout_save_is_atomic_validated_and_round_trips() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("technology_layout.ron");
        let state = ToolState::default();

        save_technology_layout(
            &state.technology_layout,
            &state.catalog,
            path.to_str().unwrap(),
        )
        .unwrap();
        save_technology_layout(
            &state.technology_layout,
            &state.catalog,
            path.to_str().unwrap(),
        )
        .unwrap();

        let reloaded = load_technology_layout(path.to_str().unwrap(), &state.catalog).unwrap();
        assert_eq!(reloaded, state.technology_layout);
        assert!(PathBuf::from(format!("{}.bak", path.display())).is_file());
        assert!(!PathBuf::from(format!("{}.tmp", path.display())).exists());
    }

    #[test]
    fn presentation_save_is_atomic_validated_and_round_trips() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("presentation.ron");
        let presentation = ToolState::default().presentation;

        save_presentation_catalog(&presentation, path.to_str().unwrap()).unwrap();
        save_presentation_catalog(&presentation, path.to_str().unwrap()).unwrap();

        let reloaded: PresentationCatalog =
            ron::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(reloaded, presentation);
        assert!(PathBuf::from(format!("{}.bak", path.display())).is_file());
        assert!(!PathBuf::from(format!("{}.tmp", path.display())).exists());
    }

    #[test]
    fn game_config_save_is_atomic_validated_and_round_trips() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("game.ron");
        let mut config = GameConfig::default();
        config.world.seed = 42;

        save_game_config(&config, path.to_str().unwrap()).unwrap();
        save_game_config(&config, path.to_str().unwrap()).unwrap();

        assert_eq!(load_game_config(path.to_str().unwrap()).unwrap(), config);
        assert!(PathBuf::from(format!("{}.bak", path.display())).is_file());
        assert!(!PathBuf::from(format!("{}.tmp", path.display())).exists());
        config.world.cell_size = 0.0;
        assert!(save_game_config(&config, path.to_str().unwrap()).is_err());
        assert_eq!(
            load_game_config(path.to_str().unwrap()).unwrap().world.seed,
            42
        );
    }

    #[test]
    fn authoring_apply_preserves_the_complete_local_twitch_setup() {
        let mut authored = GameConfig::default();
        authored.time.seconds_per_day = 900;
        authored.world.seed = 42;

        let mut runtime = GameConfig::default();
        runtime.twitch.enabled = true;
        runtime.twitch.client_id = "local-public-client-id".to_owned();
        runtime.twitch.bot_login = "localbot".to_owned();
        runtime.twitch.channel_login = "localchannel".to_owned();
        runtime.twitch.broadcast.enabled = true;
        runtime.twitch.broadcast.width = 1_920;
        runtime.twitch.broadcast.height = 1_080;
        runtime.twitch.broadcast.frames_per_second = 60;
        runtime.twitch.broadcast.video_bitrate_kbps = 6_000;
        runtime.twitch.broadcast.audio_bitrate_kbps = 160;
        runtime.twitch.broadcast.encoder = BroadcastEncoderPreference::Amd;
        runtime.twitch.broadcast.ingest = "Sydney".to_owned();

        let merged = merge_authoring_config_with_runtime(&authored, Some(&runtime));

        assert_eq!(merged.time.seconds_per_day, 900);
        assert_eq!(merged.world.seed, 42);
        assert_eq!(merged.twitch, runtime.twitch);
    }

    #[test]
    fn first_authoring_apply_uses_authored_twitch_defaults_without_a_runtime_override() {
        let authored = GameConfig::default();

        let merged = merge_authoring_config_with_runtime(&authored, None);

        assert_eq!(merged, authored);
    }

    #[test]
    fn twitch_tool_normalizes_game_master_ids_and_reward_field() {
        let ids = parse_game_master_ids(" 42,7,42, ");
        assert_eq!(format_game_master_ids(&ids), "42, 7");

        let mut state = ToolState::default();
        state.config.twitch.game_master_ids = ids;
        state.config.twitch.fish_god_reward_id = None;
        state.game_master_ids.clear();
        state.fish_god_reward_id = "stale".to_owned();
        sync_twitch_tool_fields(&mut state);
        assert_eq!(state.game_master_ids, "42, 7");
        assert!(state.fish_god_reward_id.is_empty());
    }

    #[test]
    fn role_editor_applies_every_reference_family_without_partial_mutation() {
        let mut state = ToolState::default();
        let id = StableId::new("role:gatherer").unwrap();
        state.selected_role = Some(id.clone());
        state.role_draft = role_draft(&state.catalog, &id);
        let draft = state.role_draft.as_mut().unwrap();
        draft.value.base_action_amount = 7;
        draft.value.station_kinds = ["station:food", "station:fish"]
            .into_iter()
            .map(|value| StableId::new(value).unwrap())
            .collect();
        draft.value.target_kinds = ["target:bush", "target:fish"]
            .into_iter()
            .map(|value| StableId::new(value).unwrap())
            .collect();
        draft.value.granted_abilities = ["resource:food", "role_flag:resource"]
            .into_iter()
            .map(|value| StableId::new(value).unwrap())
            .collect();

        apply_role_draft(&mut state).unwrap();
        let role = &state.catalog.roles[&id];
        assert_eq!(role.base_action_amount, 7);
        assert!(
            role.station_kinds
                .contains(&StableId::new("station:fish").unwrap())
        );
        state.catalog.validate().unwrap();

        let before = state.catalog.clone();
        state
            .role_draft
            .as_mut()
            .unwrap()
            .value
            .equipment
            .as_mut()
            .unwrap()
            .body_nodes[0]
            .clear();
        assert!(apply_role_draft(&mut state).is_err());
        assert_eq!(state.catalog, before);
    }

    #[test]
    fn foliage_editor_rejects_invalid_generation_values_without_mutation() {
        let mut state = ToolState::default();
        let before = state.catalog.clone();
        state.foliage_draft.as_mut().unwrap().noise_scale = 0.0;

        assert!(apply_foliage_draft(&mut state).is_err());
        assert_eq!(state.catalog, before);
    }

    #[test]
    fn resource_generation_layers_are_explicit_editable_and_validated() {
        let mut state = ToolState::default();
        assert_eq!(state.catalog.resource_generation.len(), 4);
        let baseline =
            stream_town_domain::generate_world_with_content(&state.config.world, &state.catalog)
                .deterministic_hash;

        let before = state.catalog.clone();
        state.resource_generation_draft.as_mut().unwrap().amount = 0;
        assert!(apply_resource_generation_draft(&mut state).is_err());
        assert_eq!(state.catalog, before);

        refresh_resource_generation_draft(&mut state);
        state.new_resource_generation_id = "resource_generation:test".to_owned();
        state.new_resource_generation_name = "Test layer".to_owned();
        duplicate_resource_generation_layer(&mut state).unwrap();
        assert_eq!(state.catalog.resource_generation.len(), 5);
        delete_resource_generation_layer(&mut state).unwrap();
        assert_eq!(state.catalog.resource_generation.len(), 4);
        assert_eq!(
            stream_town_domain::generate_world_with_content(&state.config.world, &state.catalog)
                .deterministic_hash,
            baseline
        );
    }

    #[test]
    fn enemy_camp_generation_layer_lifecycle_is_complete() {
        let mut state = ToolState::default();
        let original_count = state.catalog.enemy_camp_generation.len();
        let before = state.catalog.clone();
        state
            .enemy_camp_generation_draft
            .as_mut()
            .unwrap()
            .minimum_distance_between_camps_milli_cells = 0;
        assert!(apply_enemy_camp_generation_draft(&mut state).is_err());
        assert_eq!(state.catalog, before);

        refresh_enemy_camp_generation_draft(&mut state);
        state.new_enemy_camp_generation_id = "enemy_camp_generation:test".to_owned();
        duplicate_enemy_camp_generation_layer(&mut state).unwrap();
        assert_eq!(
            state.catalog.enemy_camp_generation.len(),
            original_count + 1
        );
        delete_enemy_camp_generation_layer(&mut state).unwrap();
        assert_eq!(state.catalog.enemy_camp_generation.len(), original_count);
        state.catalog.validate().unwrap();
    }

    #[test]
    fn checked_in_authoring_assets_pass_headless_validation() {
        let summary = validate_authoring_assets().unwrap();
        assert!(summary.contains("roles"));
        assert!(summary.contains("technologies"));
    }
}
