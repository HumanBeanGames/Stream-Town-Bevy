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
        graph::{AnimationGraph, AnimationGraphHandle, AnimationNodeIndex},
        prelude::{AnimatableCurve, AnimatableKeyframeCurve},
    },
    asset::{AssetPlugin, RenderAssetUsages},
    camera::ScalingMode,
    color::LinearRgba,
    mesh::Indices,
    prelude::*,
    render::render_resource::PrimitiveTopology,
    render::view::screenshot::{Screenshot, save_to_disk},
    window::{PrimaryWindow, WindowResolution},
};
use stream_town_domain::{
    ActorKind, AnimationBlendSelection, AnimationClipDef, AnimationControllerRuntime,
    AnimationTransformTrack, ArchetypeDef, ArchetypeKind, ArchetypeScene, ChatCommand,
    ContentCatalog, GameConfig, GeneratedWorld, GridPos, MaterialAlphaMode as AuthoredAlphaMode,
    MaterialDef, NativeSaveStore, PresentationCatalog, SavedActor, Season, StableId, TownEvent,
    Weather, WorldSimulation, WorldSnapshot, generate_world,
};
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
}

#[derive(Resource)]
struct SaveRuntime {
    store: NativeSaveStore,
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
    display_name: String,
    command: ChatCommand,
}

#[derive(Resource, Default)]
struct InjectedCommands(VecDeque<PendingChatCommand>);

#[derive(Resource, Default)]
struct CommandFeedback(String);

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
struct RenderAssets {
    cube: Handle<Mesh>,
    ground: Handle<StandardMaterial>,
    water: Handle<StandardMaterial>,
    wood: Handle<StandardMaterial>,
    ore: Handle<StandardMaterial>,
    food: Handle<StandardMaterial>,
    building: Handle<StandardMaterial>,
    enemy_idle: Handle<StandardMaterial>,
    enemy_moving: Handle<StandardMaterial>,
    player_idle: Handle<StandardMaterial>,
    player_moving: Handle<StandardMaterial>,
    selection: Handle<StandardMaterial>,
    rain: Handle<StandardMaterial>,
    snow: Handle<StandardMaterial>,
    presentation_materials: BTreeMap<StableId, Handle<StandardMaterial>>,
}

#[derive(Component)]
struct StateEntity;

#[derive(Component)]
struct WorldEntity;

#[derive(Component)]
struct Agent {
    id: StableId,
    kind: ActorKind,
    goal: AgentGoal,
    origin: GridPos,
    path: Vec<GridPos>,
    path_index: usize,
    target: GridPos,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
enum AgentGoal {
    #[default]
    Wander,
    Gather(StableId),
    Deposit,
}

#[derive(Component, Clone, Copy)]
struct GridLocation(GridPos);

#[derive(Component)]
struct ResourceNode {
    id: StableId,
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct TownHall;

#[derive(Component)]
struct RuntimeBuilding {
    id: StableId,
}

#[derive(Component)]
struct SelectionMarker;

#[derive(Component)]
struct TownCamera;

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

#[derive(Component, Clone)]
struct NativeAnimationSpec {
    graph: Handle<AnimationGraph>,
    idle: AnimationNodeIndex,
    moving: AnimationNodeIndex,
}

#[derive(Component, Clone)]
struct ConvertedAnimationSpec {
    controller: StableId,
    state: StableId,
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
    fallback_state: StableId,
    runtime: AnimationControllerRuntime,
    nodes: BTreeMap<StableId, AnimationNodeIndex>,
    active: Vec<(AnimationNodeIndex, f32)>,
    last_alive: Option<bool>,
}

#[derive(Component, Clone)]
struct MaterialOverrideSpec(Handle<StandardMaterial>);

#[derive(Component)]
struct MaterialOverrideApplied;

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
            .init_resource::<TwitchConnection>()
            .init_resource::<SelectedCell>()
            .init_resource::<EnvironmentPresentation>()
            .insert_resource(SaveRuntime {
                store: NativeSaveStore::new(
                    PathBuf::from(".stream-town").join("StreamTownSave.stbevy"),
                ),
            })
            .add_systems(Startup, (setup_rendering, start_twitch_transport))
            .add_systems(OnEnter(GameState::Boot), finish_boot)
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(Update, (poll_twitch_transport, twitch_connection_input))
            .add_systems(
                Update,
                main_menu_input.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_state_entities)
            .add_systems(OnEnter(GameState::WorldLoading), generate_and_spawn_world)
            .add_systems(
                Update,
                (
                    move_agents,
                    sync_resource_nodes.after(move_agents),
                    animate_agents,
                    attach_native_animations,
                    attach_converted_animations,
                    drive_native_animations,
                    drive_converted_animations,
                    apply_material_overrides,
                    update_environment_presentation.after(move_agents),
                    animate_weather_particles.after(update_environment_presentation),
                    camera_controls,
                    select_grid_cell,
                    game_input,
                    save_input,
                    load_input,
                    capture_screenshot,
                    process_injected_commands.after(game_input),
                    update_hud,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), cleanup_world)
            .add_systems(OnEnter(GameState::Credits), spawn_credits)
            .add_systems(Update, credits_input.run_if(in_state(GameState::Credits)))
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
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(PhysicsPlugins::default())
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
    presentation: Res<RuntimePresentation>,
    asset_server: Option<Res<AssetServer>>,
    meshes: Option<ResMut<Assets<Mesh>>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
) {
    let (Some(mut meshes), Some(mut materials)) = (meshes, materials) else {
        commands.insert_resource(RenderAssets::default());
        return;
    };
    let material_closeup = std::env::var_os("STREAM_TOWN_SMOKE_CLOSEUP").is_some();
    let animation_closeup = std::env::var_os("STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP").is_some();
    commands.spawn((
        TownCamera,
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: if material_closeup {
                    96.0
                } else if animation_closeup {
                    180.0
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
    let presentation_materials = presentation
        .0
        .materials
        .iter()
        .map(|(id, material)| {
            (
                id.clone(),
                materials.add(standard_material(
                    material,
                    &presentation.0,
                    asset_server.as_deref(),
                )),
            )
        })
        .collect();
    commands.insert_resource(RenderAssets {
        cube: meshes.add(Cuboid::default()),
        ground: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.96,
            ..default()
        }),
        water: materials.add(StandardMaterial {
            base_color: Color::srgba(0.04, 0.24, 0.42, 0.62),
            perceptual_roughness: 0.18,
            metallic: 0.05,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        wood: materials.add(Color::srgb(0.16, 0.46, 0.18)),
        ore: materials.add(Color::srgb(0.46, 0.50, 0.55)),
        food: materials.add(Color::srgb(0.74, 0.64, 0.18)),
        building: materials.add(Color::srgb(0.42, 0.26, 0.12)),
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
        presentation_materials,
    });
}

fn standard_material(
    material: &MaterialDef,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
) -> StandardMaterial {
    let base_color_texture = asset_server.and_then(|asset_server| {
        primary_material_texture(material, presentation)
            .map(|path| asset_server.load(path.to_owned()))
    });
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
        ..default()
    }
}

fn primary_material_texture<'a>(
    material: &'a MaterialDef,
    presentation: &'a PresentationCatalog,
) -> Option<&'a str> {
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
        .filter_map(|slot| material.textures.get(*slot))
        .chain(material.textures.values())
        .find_map(|id| presentation.textures.get(id))
        .map(|texture| texture.asset_path.as_str())
}

fn finish_boot(mut next_state: ResMut<NextState<GameState>>) {
    info!("Stream Town boot validation complete");
    if std::env::var_os("STREAM_TOWN_AUTOSTART").is_some() {
        next_state.set(GameState::WorldLoading);
    } else {
        next_state.set(GameState::MainMenu);
    }
}

fn spawn_main_menu(mut commands: Commands) {
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
    meshes: Option<ResMut<Assets<Mesh>>>,
    asset_server: Option<Res<AssetServer>>,
    animation_graphs: Option<ResMut<Assets<AnimationGraph>>>,
    asset_root: Res<RuntimeAssetRoot>,
    mut selected: ResMut<SelectedCell>,
    mut cameras: Query<&mut Transform, With<TownCamera>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut animation_graphs = animation_graphs;
    selected.0 = None;
    let mut generated = generate_world(&config.0.world);
    let centre = GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    };
    let town_hall_position = GridPos {
        x: (centre.x + 4).min(config.0.world.width - 2),
        z: centre.z,
    };
    if let Ok(mut camera) = cameras.single_mut() {
        *camera = if std::env::var_os("STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP").is_some() {
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
    let _ = generated.navigation.set_blocked(
        stream_town_domain::DirtyRegion {
            min: town_hall_position,
            max: GridPos {
                x: town_hall_position.x + 1,
                z: town_hall_position.z + 1,
            },
        },
        true,
    );

    let world_size = Vec2::new(
        f32::from(config.0.world.width) * config.0.world.cell_size,
        f32::from(config.0.world.height) * config.0.world.cell_size,
    );
    if let Some(mut meshes) = meshes {
        let terrain_mesh = generated_terrain_mesh(&generated, &config.0);
        let terrain_collider = Collider::trimesh_from_mesh(&terrain_mesh)
            .expect("generated terrain mesh has indexed triangle geometry");
        commands.spawn((
            WorldEntity,
            Mesh3d(meshes.add(terrain_mesh)),
            MeshMaterial3d(render.ground.clone()),
            terrain_collider,
            RigidBody::Static,
        ));
    } else {
        commands.spawn((
            WorldEntity,
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.ground.clone()),
            Transform::from_xyz(0.0, -0.15, 0.0).with_scale(Vec3::new(
                world_size.x,
                0.3,
                world_size.y,
            )),
        ));
    }
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

    for resource in &generated.resources {
        let position = grid_to_world_on_surface(resource.position, &config.0, &generated);
        let material = match resource.kind.as_str() {
            "resource:wood" => render.wood.clone(),
            "resource:ore" => render.ore.clone(),
            _ => render.food.clone(),
        };
        let scale = config.0.world.cell_size * 0.55;
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

    let hall = grid_to_world_on_surface(town_hall_position, &config.0, &generated);
    let mut hall_entity = commands.spawn((
        WorldEntity,
        TownHall,
        GridLocation(town_hall_position),
        Transform::from_translation(hall),
    ));
    let town_hall_id = StableId::new("building:townhall").expect("static ID");
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
            Transform::from_translation(hall)
                .with_scale(Vec3::splat(config.0.world.cell_size / 2.0)),
        ));
        if let Some(material) = town_hall
            .and_then(|archetype| prefab_material_handle(archetype, &presentation.0, &render))
        {
            hall_entity.insert(MaterialOverrideSpec(material));
        }
    } else {
        let footprint = town_hall.map_or([2, 2], |archetype| archetype.footprint);
        let size = Vec3::new(
            f32::from(footprint[0]) * config.0.world.cell_size,
            config.0.world.cell_size * 1.5,
            f32::from(footprint[1]) * config.0.world.cell_size,
        );
        hall_entity.insert((
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
    'cells: for z in 0..generated.navigation.height() {
        for x in 0..generated.navigation.width() {
            let position = GridPos { x, z };
            if !generated.navigation.is_walkable(position) || position == town_hall_position {
                continue;
            }
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
            let real_archetype = if spawned == 0 {
                archetype_by_source(&content.0, ArchetypeKind::Enemy, "Enemy_Goblin.prefab")
            } else if spawned == 1 {
                archetype_by_source(&content.0, ArchetypeKind::Player, "Player_Character.prefab")
            } else {
                None
            };
            let real_scene = real_archetype
                .and_then(default_archetype_scene)
                .filter(|scene| {
                    asset_server.is_some()
                        && converted_asset_exists(&asset_root.0, &scene.asset_path)
                });
            let native_animation = real_archetype
                .zip(real_scene)
                .and_then(|(archetype, scene)| {
                    native_animation_spec(
                        archetype,
                        scene,
                        &presentation.0,
                        asset_server.as_deref(),
                        animation_graphs.as_deref_mut(),
                    )
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
                    goal: AgentGoal::Wander,
                    origin: position,
                    path: Vec::new(),
                    path_index: 0,
                    target,
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
                    prefab_material_handle(archetype, &presentation.0, &render)
                }) {
                    entity.insert(MaterialOverrideSpec(material));
                }
            } else {
                entity.insert((
                    Mesh3d(render.cube.clone()),
                    MeshMaterial3d(actor_material(&render, &kind, false)),
                ));
            }
            spawned += 1;
            if spawned >= config.0.gameplay.initial_agents {
                break 'cells;
            }
        }
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
    commands.insert_resource(WorldRuntime { generated });
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

fn native_animation_spec(
    archetype: &ArchetypeDef,
    scene: &ArchetypeScene,
    presentation: &PresentationCatalog,
    asset_server: Option<&AssetServer>,
    animation_graphs: Option<&mut Assets<AnimationGraph>>,
) -> Option<NativeAnimationSpec> {
    let binding = presentation.prefab_bindings.get(&archetype.source_guid)?;
    let animation_index = binding.gltf_animation_index?;
    if binding.animated_scene.as_deref() != Some(scene.asset_path.as_str()) {
        return None;
    }
    let asset_server = asset_server?;
    let animation_graphs = animation_graphs?;
    let (graph, node) = AnimationGraph::from_clip(
        asset_server.load(
            GltfAssetLabel::Animation(usize::try_from(animation_index).ok()?)
                .from_asset(scene.asset_path.clone()),
        ),
    );
    Some(NativeAnimationSpec {
        graph: animation_graphs.add(graph),
        idle: node,
        moving: node,
    })
}

fn converted_animation_spec(
    archetype: &ArchetypeDef,
    presentation: &PresentationCatalog,
) -> Option<ConvertedAnimationSpec> {
    let binding = presentation.prefab_bindings.get(&archetype.source_guid)?;
    let controller = presentation.controllers.get(&binding.controller)?;
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
        }
    })
}

fn prefab_material_handle(
    archetype: &ArchetypeDef,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
) -> Option<Handle<StandardMaterial>> {
    presentation
        .prefab_materials
        .get(&archetype.source_guid)?
        .iter()
        .find_map(|id| render.presentation_materials.get(id))
        .cloned()
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

fn resource_for_role(role: &StableId) -> Option<StableId> {
    let resource = match role.as_str() {
        "role:logger" => "resource:wood",
        "role:miner" => "resource:ore",
        "role:farmer" | "role:fisher" | "role:gatherer" => "resource:food",
        _ => return None,
    };
    Some(StableId::new(resource).expect("role resource IDs are valid"))
}

fn town_hall_grid_position(config: &GameConfig) -> GridPos {
    GridPos {
        x: (config.world.width / 2 + 4).min(config.world.width - 2),
        z: config.world.height / 2,
    }
}

fn next_agent_goal(
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    actor_id: &StableId,
    current: GridPos,
) -> (AgentGoal, GridPos) {
    const CARRY_CAPACITY: u32 = 25;
    let Some(actor) = simulation.actors.get(actor_id) else {
        return (AgentGoal::Wander, mirrored_target(world, current));
    };
    let carried = actor
        .inventory
        .values()
        .copied()
        .fold(0_u32, u32::saturating_add);
    if carried >= CARRY_CAPACITY {
        let town_hall = town_hall_grid_position(config);
        let target = nearest_walkable(world, town_hall).unwrap_or(current);
        return (AgentGoal::Deposit, target);
    }
    let Some(resource_kind) = resource_for_role(&actor.role) else {
        return (AgentGoal::Wander, mirrored_target(world, current));
    };
    let resource = world
        .resources
        .iter()
        .filter(|resource| resource.kind == resource_kind && resource.amount > 0)
        .min_by_key(|resource| {
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
    actor_id: &StableId,
    goal: &AgentGoal,
) {
    const GATHER_AMOUNT: u32 = 5;
    match goal {
        AgentGoal::Gather(resource_id) => {
            let Some(resource) = world
                .resources
                .iter_mut()
                .find(|resource| resource.id == *resource_id && resource.amount > 0)
            else {
                return;
            };
            let amount = resource.amount.min(GATHER_AMOUNT);
            resource.amount -= amount;
            let resource_kind = resource.kind.clone();
            if let Err(error) = simulation.gather(actor_id, resource_kind, amount) {
                warn!(actor = %actor_id, %error, "resource gather action failed");
                resource.amount = resource.amount.saturating_add(amount);
            }
        }
        AgentGoal::Deposit => {
            if let Err(error) = simulation.deposit_all(actor_id) {
                warn!(actor = %actor_id, %error, "resource deposit action failed");
            }
        }
        AgentGoal::Wander => {}
    }
}

fn move_agents(
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    mut world: ResMut<WorldRuntime>,
    mut simulation: ResMut<SimulationRuntime>,
    mut stats: ResMut<SessionStats>,
    mut agents: Query<(
        &mut Agent,
        &mut GridLocation,
        &AgentAnimation,
        &mut Transform,
    )>,
) {
    stats.elapsed_seconds += time.delta_secs_f64();
    simulation.0.tick(time.delta_secs());
    if let Some(weather) = debug_weather_override() {
        simulation.0.weather = weather;
    }
    for (mut agent, mut location, animation, mut transform) in &mut agents {
        if agent.path.is_empty() || agent.path_index >= agent.path.len() {
            if !agent.path.is_empty() {
                stats.paths_completed += 1;
                if location.0 == agent.target {
                    complete_agent_goal(
                        &mut simulation.0,
                        &mut world.generated,
                        &agent.id,
                        &agent.goal,
                    );
                }
            }
            agent.origin = location.0;
            let (goal, target) = next_agent_goal(
                &simulation.0,
                &world.generated,
                &config.0,
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
        let step = config.0.gameplay.agent_speed_cells_per_second
            * config.0.world.cell_size
            * time.delta_secs();
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

#[allow(clippy::too_many_arguments)]
fn update_environment_presentation(
    mut commands: Commands,
    simulation: Res<SimulationRuntime>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    mut presentation: ResMut<EnvironmentPresentation>,
    mut clear_color: Option<ResMut<ClearColor>>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
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
    if let Some(materials) = materials.as_deref_mut() {
        if let Some(mut ground) = materials.get_mut(&render.ground) {
            ground.base_color = Color::srgb(
                palette.terrain_tint[0],
                palette.terrain_tint[1],
                palette.terrain_tint[2],
            );
        }
        if let Some(mut water) = materials.get_mut(&render.water) {
            water.base_color = Color::srgba(
                palette.water_color[0],
                palette.water_color[1],
                palette.water_color[2],
                palette.water_color[3],
            );
        }
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
        let moving = !agent.path.is_empty() && agent.path_index < agent.path.len();
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
    specs: Query<(Entity, &ConvertedAnimationSpec), Without<ConvertedAnimationApplied>>,
    children: Query<&Children>,
    names: Query<&Name>,
    transforms: Query<&Transform>,
) {
    let (Some(mut animation_clips), Some(mut animation_graphs)) =
        (animation_clips, animation_graphs)
    else {
        return;
    };
    for (actor_root, spec) in &specs {
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
            let Some(source) = presentation.0.clips.get(&motion.clip) else {
                continue;
            };
            let Some(clip) = retargeted_animation_clip(source, &targets) else {
                continue;
            };
            converted.push((motion.clip.clone(), animation_clips.add(clip)));
        }
        if converted.is_empty() {
            continue;
        }
        let (graph, graph_nodes) =
            AnimationGraph::from_clips(converted.iter().map(|(_, clip)| clip.clone()));
        let nodes: BTreeMap<_, _> = converted
            .iter()
            .zip(graph_nodes)
            .map(|((clip, _), node)| (clip.clone(), node))
            .collect();
        let Ok(runtime) = AnimationControllerRuntime::in_state(controller, spec.state.clone())
        else {
            continue;
        };
        let graph = animation_graphs.add(graph);
        for (path, (entity, _)) in &targets {
            commands.entity(*entity).insert((
                path.split('/').collect::<AnimationTargetId>(),
                AnimatedBy(animation_root),
            ));
        }
        commands.entity(animation_root).insert((
            AnimationPlayer::default(),
            AnimationGraphHandle(graph),
            ConvertedAnimationDriver {
                actor_root,
                controller: spec.controller.clone(),
                fallback_state: spec.state.clone(),
                runtime,
                nodes,
                active: Vec::new(),
                last_alive: None,
            },
        ));
        commands
            .entity(actor_root)
            .insert(ConvertedAnimationApplied);
        info!(
            actor = ?actor_root,
            controller = %spec.controller,
            state = %spec.state,
            clips = converted.len(),
            targets = targets.len(),
            "attached translated Unity animation controller"
        );
    }
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
    config: Res<RuntimeConfig>,
    presentation: Res<RuntimePresentation>,
    simulation: Res<SimulationRuntime>,
    agents: Query<&Agent>,
    mut players: Query<(&mut AnimationPlayer, &mut ConvertedAnimationDriver)>,
) {
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
        let _ = driver.runtime.set_float("Move Speed", move_speed);

        if let Some(alive) = simulation.0.actors.get(&agent.id).map(|actor| actor.alive) {
            if let Some(previous) = driver.last_alive
                && alive != previous
            {
                let trigger = if alive { "Revive" } else { "Death" };
                let _ = driver.runtime.set_trigger(trigger);
            }
            driver.last_alive = Some(alive);
        }

        let normalized_time = current_normalized_time(&player, &driver, &presentation.0);
        let transition = driver
            .runtime
            .evaluate_transitions(controller, normalized_time)
            .ok();
        if matches!(
            transition,
            Some(stream_town_domain::AnimationTransitionOutcome::Exited)
        ) {
            let fallback_state = driver.fallback_state.clone();
            let _ = driver.runtime.enter_state(controller, fallback_state);
        }
        if let Some(stream_town_domain::AnimationTransitionOutcome::Entered(state)) = &transition {
            info!(
                actor = ?driver.actor_root,
                state = %state,
                "translated animation controller entered state"
            );
        }

        let Ok(Some(selection)) = driver.runtime.motion_selection(controller) else {
            continue;
        };
        let desired = animation_nodes_for_selection(&selection, &driver.nodes);
        if desired.is_empty() {
            continue;
        }
        let changed = !same_animation_blend(&driver.active, &desired);
        apply_animation_blend(&mut player, &desired);
        if changed {
            info!(
                actor = ?driver.actor_root,
                state = %driver.runtime.current_state(),
                primary = %selection.first.clip,
                primary_weight = selection.first.weight,
                secondary = selection.second.as_ref().map(|motion| motion.clip.as_str()),
                secondary_weight = selection.second.as_ref().map(|motion| motion.weight),
                "applied translated animation blend"
            );
        }
        driver.active = desired;
    }
}

fn current_normalized_time(
    player: &AnimationPlayer,
    driver: &ConvertedAnimationDriver,
    presentation: &PresentationCatalog,
) -> f32 {
    player
        .playing_animations()
        .filter_map(|(node, animation)| {
            let clip = driver
                .nodes
                .iter()
                .find_map(|(clip, candidate)| (candidate == node).then_some(clip))?;
            let duration = presentation.clips.get(clip)?.duration_seconds;
            (duration > f32::EPSILON).then_some(animation.elapsed() / duration)
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

fn apply_animation_blend(player: &mut AnimationPlayer, desired: &[(AnimationNodeIndex, f32)]) {
    let playing: Vec<_> = player.playing_animations().map(|(node, _)| *node).collect();
    for node in playing {
        if !desired.iter().any(|(desired, _)| *desired == node) {
            player.stop(node);
        }
    }
    for (node, weight) in desired {
        player.play(*node).repeat().set_weight(*weight);
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

fn apply_material_overrides(
    mut commands: Commands,
    specs: Query<&MaterialOverrideSpec>,
    parents: Query<&ChildOf>,
    mut renderers: Query<
        (Entity, &mut MeshMaterial3d<StandardMaterial>),
        Without<MaterialOverrideApplied>,
    >,
) {
    for (entity, mut material) in &mut renderers {
        let mut ancestor = entity;
        for _ in 0..64 {
            if let Ok(spec) = specs.get(ancestor) {
                material.0 = spec.0.clone();
                commands.entity(entity).insert(MaterialOverrideApplied);
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
    }
}

fn camera_controls(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
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
            display_name: "debug_viewer".to_owned(),
            command: "!join".parse().expect("static chat command"),
        });
    }
    if !*injected_debug_commands {
        *injected_debug_commands = true;
        if let Some(commands) = std::env::var_os("STREAM_TOWN_DEBUG_COMMANDS") {
            for command in commands.to_string_lossy().split(';') {
                match command.trim().parse() {
                    Ok(command) => injected.0.push_back(PendingChatCommand {
                        actor_id: StableId::new("twitch:debug_viewer").expect("static ID"),
                        display_name: "debug_viewer".to_owned(),
                        command,
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
            match message.message.parse::<ChatCommand>() {
                Ok(command) => injected.0.push_back(PendingChatCommand {
                    actor_id: message.actor_id,
                    display_name: message.display_name,
                    command,
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
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    mut stats: ResMut<SessionStats>,
    mut simulation: ResMut<SimulationRuntime>,
    mut agents: Query<(
        Entity,
        &mut Agent,
        &mut GridLocation,
        &AgentAnimation,
        &mut Transform,
    )>,
    runtime_buildings: Query<(Entity, &RuntimeBuilding)>,
) {
    if !keyboard.just_pressed(KeyCode::F9) {
        return;
    }
    let snapshot = match save.store.load() {
        Ok(snapshot) => snapshot,
        Err(error) => {
            error!(%error, "native load failed");
            return;
        }
    };
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

    let mut restored_world = generate_world(&config.0.world);
    if !snapshot.resource_nodes.is_empty() {
        for resource in &mut restored_world.resources {
            if let Some(remaining) = snapshot.resource_nodes.get(&resource.id) {
                resource.amount = *remaining;
            }
        }
    }
    let centre = GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    };
    let town_hall_position = GridPos {
        x: (centre.x + 4).min(config.0.world.width - 2),
        z: centre.z,
    };
    let _ = restored_world.navigation.set_blocked(
        stream_town_domain::DirtyRegion {
            min: town_hall_position,
            max: GridPos {
                x: town_hall_position.x + 1,
                z: town_hall_position.z + 1,
            },
        },
        true,
    );
    for (entity, building) in &runtime_buildings {
        debug!(building = %building.id, "despawning runtime building before native load");
        ecs.entity(entity).despawn();
    }
    for saved in snapshot.simulation.buildings.values() {
        let Some(building) = content
            .0
            .buildings
            .values()
            .find(|building| building.archetype == saved.archetype)
        else {
            error!(
                building = %saved.id,
                archetype = %saved.archetype,
                "native save references an unknown building archetype"
            );
            return;
        };
        let Some(region) = building_region(saved.position, building.footprint, &restored_world)
        else {
            error!(building = %saved.id, "native save building lies outside the world");
            return;
        };
        if let Err(error) = restored_world.navigation.set_blocked(region, true) {
            error!(building = %saved.id, %error, "native save building could not update navigation");
            return;
        }
        spawn_runtime_building(
            &mut ecs,
            &config.0,
            &restored_world,
            &presentation.0,
            asset_server.as_deref(),
            &asset_root.0,
            &render,
            saved.id.clone(),
            &content.0.archetypes[&building.archetype],
            saved.position,
            building.footprint,
        );
    }
    world.generated = restored_world;

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
        agent.origin = position;
        agent.path.clear();
        agent.path_index = 0;
        agent.target = mirrored_target(&world.generated, position);
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
                goal: AgentGoal::Wander,
                origin: position,
                path: Vec::new(),
                path_index: 0,
                target: mirrored_target(&world.generated, position),
            },
            AgentAnimation {
                base_scale,
                ..default()
            },
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(actor_material(&render, &saved.kind, false)),
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
    info!(path = %save.store.path().display(), "native save loaded and applied");
}

fn capture_screenshot(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut elapsed: Local<f32>,
    mut automatic_complete: Local<bool>,
    mut counter: Local<u32>,
) {
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

fn town_event_from_id(requested: &StableId) -> Option<TownEvent> {
    match requested.as_str().trim_start_matches("event:") {
        "festival" => Some(TownEvent::Festival),
        "raid" | "enemy_raid" => Some(TownEvent::EnemyRaid),
        "harsh_weather" | "weather" => Some(TownEvent::HarshWeather),
        "resource_boom" | "wood_boom" => Some(TownEvent::ResourceBoom(
            StableId::new("resource:wood").expect("static ID"),
        )),
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

fn find_building_site(
    world: &GeneratedWorld,
    near: GridPos,
    footprint: [u16; 2],
) -> Option<GridPos> {
    let mut candidates = Vec::new();
    for z in 0..world.navigation.height() {
        for x in 0..world.navigation.width() {
            let position = GridPos { x, z };
            let Some(region) = building_region(position, footprint, world) else {
                continue;
            };
            let available = (region.min.z..=region.max.z).all(|cell_z| {
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
            });
            if available {
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

fn runtime_building_id(simulation: &WorldSimulation) -> StableId {
    for sequence in simulation.buildings.len()..usize::MAX {
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
    id: StableId,
    archetype: &ArchetypeDef,
    position: GridPos,
    footprint: [u16; 2],
) {
    let centre = GridPos {
        x: position.x + footprint[0] / 2,
        z: position.z + footprint[1] / 2,
    };
    let world_position = grid_to_world_on_surface(centre, config, world);
    let mut entity = commands.spawn((
        WorldEntity,
        RuntimeBuilding { id },
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
        if let Some(material) = prefab_material_handle(archetype, presentation, render) {
            entity.insert(MaterialOverrideSpec(material));
        }
    } else {
        let size = Vec3::new(
            f32::from(footprint[0]) * config.world.cell_size * 0.88,
            config.world.cell_size * 1.25,
            f32::from(footprint[1]) * config.world.cell_size * 0.88,
        );
        entity.insert((
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.building.clone()),
            Transform::from_translation(world_position + Vec3::Y * size.y * 0.5).with_scale(size),
        ));
    }
}

fn send_command_feedback(connection: &TwitchConnection, display_name: &str, message: &str) {
    if let Some(transport) = &connection.transport {
        let _ = transport.send(TwitchControl::SendMessage(format!(
            "@{display_name} {message}"
        )));
    }
}

#[allow(clippy::too_many_arguments)]
fn process_injected_commands(
    mut ecs: Commands,
    mut queue: ResMut<InjectedCommands>,
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
    while let Some(pending) = queue.0.pop_front() {
        let actor_id = pending.actor_id;
        let command = pending.command;
        let result: Result<String, String> = match &command {
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
                                goal: AgentGoal::Wander,
                                origin: position,
                                path: Vec::new(),
                                path_index: 0,
                                target,
                            },
                            AgentAnimation {
                                base_scale,
                                ..default()
                            },
                            Mesh3d(render.cube.clone()),
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
                    simulation
                        .0
                        .assign_role(&actor_id, role.clone())
                        .map(|()| format!("role changed to {role}"))
                        .map_err(|error| error.to_string())
                })
            }
            ChatCommand::Build(requested) => {
                let building_id = prefixed_id(requested, "building:")
                    .filter(|building| content.0.buildings.contains_key(building))
                    .ok_or_else(|| format!("unknown building {}", requested.as_str()));
                building_id.and_then(|building_id| {
                    let building = &content.0.buildings[&building_id];
                    let near = selected.0.or_else(|| {
                        simulation
                            .0
                            .actors
                            .get(&actor_id)
                            .map(|actor| actor.position)
                    });
                    let near = near.ok_or_else(|| "join before building".to_owned())?;
                    let position = find_building_site(&world.generated, near, building.footprint)
                        .ok_or_else(|| "no valid building site is available".to_owned())?;
                    let runtime_id = runtime_building_id(&simulation.0);
                    simulation
                        .0
                        .construct(
                            runtime_id.clone(),
                            building.archetype.clone(),
                            position,
                            &building.cost,
                        )
                        .map_err(|error| error.to_string())?;
                    let region = building_region(position, building.footprint, &world.generated)
                        .ok_or_else(|| "selected building site left the world".to_owned())?;
                    world
                        .generated
                        .navigation
                        .set_blocked(region, true)
                        .map_err(|error| error.to_string())?;
                    spawn_runtime_building(
                        &mut ecs,
                        &config.0,
                        &world.generated,
                        &presentation.0,
                        asset_server.as_deref(),
                        &asset_root.0,
                        &render,
                        runtime_id,
                        &content.0.archetypes[&building.archetype],
                        position,
                        building.footprint,
                    );
                    Ok(format!("built {}", building.display_name))
                })
            }
            ChatCommand::Vote(requested) => {
                let technology = resolve_technology_id(&content.0, requested)
                    .ok_or_else(|| format!("unknown technology {}", requested.as_str()));
                technology.and_then(|technology| {
                    let node = &content.0.technology.nodes[&technology];
                    if node.unavailable {
                        return Err(format!("{} is unavailable", node.display_name));
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
            ChatCommand::TriggerEvent(event) => {
                town_event_from_id(event)
                    .ok_or_else(|| format!("unknown event {}", event.as_str()))
                    .map(|event| {
                        simulation.0.trigger_event(event);
                        "event started".to_owned()
                    })
            }
            ChatCommand::Save => {
                let snapshot = snapshot_world(&world, &stats, &simulation, &agents);
                save.store
                    .write(&snapshot)
                    .map(|()| "town saved".to_owned())
                    .map_err(|error| format!("save failed: {error}"))
            }
            ChatCommand::Help => Ok(
                "commands: !join, !role <role>, !build <building>, !vote <technology>, !event <event>, !save, !help"
                    .to_owned(),
            ),
        };
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
    simulation: Res<SimulationRuntime>,
    feedback: Res<CommandFeedback>,
    agents: Query<&Agent>,
    mut hud: Single<&mut Text, With<Hud>>,
) {
    if !stats.is_changed() && !twitch.is_changed() && !feedback.is_changed() {
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
    hud.0 = format!(
        "{} agents | {:.0}s | {} routes | workers {gathering} gather/{depositing} deposit | {} commands | {:?} / {:?} | Twitch: {}\nResources F:{} G:{} O:{} W:{} | {}\nF1 Twitch Off | F2 Twitch On | F5 Save | F9 Load | F12 Capture | J Inject !join | WASD Pan | Q/E Zoom | Click Select | ESC Menu | first {first_id}",
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
        feedback.0,
    );
}

fn town_resource_amount(simulation: &WorldSimulation, resource: &str) -> u32 {
    StableId::new(resource)
        .ok()
        .and_then(|resource| simulation.town_resources.get(&resource).copied())
        .unwrap_or_default()
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
                archetype: StableId::new("archetype:viewer").expect("static ID"),
                grid_position: location.0,
                height_centimetres: world
                    .generated
                    .navigation
                    .height_at(location.0)
                    .unwrap_or_default(),
                health: 100,
            })
            .collect(),
        simulation: simulation.0.clone(),
        resource_nodes: world
            .generated
            .resources
            .iter()
            .map(|resource| (resource.id.clone(), resource.amount))
            .collect(),
        legacy_terrain_mesh: None,
        legacy_migration: None,
    }
}

fn spawn_credits(mut commands: Commands) {
    commands.spawn((
        StateEntity,
        Text::new(
            "STREAM TOWN\nOriginal project by Jayden Hunter and contributors\nBevy migration by Human Bean Games\n\nESC  Main Menu",
        ),
        TextFont {
            font_size: FontSize::Px(36.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::srgb(0.86, 0.95, 0.84)),
        Node {
            position_type: PositionType::Absolute,
            top: percent(34.0),
            left: percent(24.0),
            ..default()
        },
    ));
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

    #[test]
    fn role_driven_resource_loop_depletes_and_deposits() {
        let config = GameConfig::default();
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
        for _ in 0..5 {
            complete_agent_goal(&mut simulation, &mut world, &actor_id, &gather);
        }
        assert_eq!(
            world
                .resources
                .iter()
                .find(|candidate| candidate.id == resource.id)
                .unwrap()
                .amount,
            starting_amount - 25
        );
        assert_eq!(simulation.actors[&actor_id].inventory[&resource.kind], 25);
        let (goal, _) = next_agent_goal(&simulation, &world, &config, &actor_id, resource.position);
        assert_eq!(goal, AgentGoal::Deposit);
        complete_agent_goal(&mut simulation, &mut world, &actor_id, &AgentGoal::Deposit);
        assert!(simulation.actors[&actor_id].inventory.is_empty());
        assert_eq!(simulation.town_resources[&resource.kind], 25);
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
        assert_eq!(content.buildings.len(), 26);
        assert_eq!(content.roles.len(), 15);
        assert_eq!(content.technology.nodes.len(), 363);
        assert_eq!(content.technology.groups.len(), 20);
    }

    #[test]
    fn embedded_presentation_binds_native_and_converted_animation_paths() {
        let content = embedded_content();
        let presentation = embedded_presentation();
        assert_eq!(presentation.textures.len(), 133);
        assert_eq!(presentation.materials.len(), 33);
        assert_eq!(presentation.controllers.len(), 31);
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
        let state = controller.states.get(&spec.state).unwrap();
        assert_eq!(state.blend_parameter.as_deref(), Some("Move Speed"));
        assert_eq!(state.motions.len(), 3);
        let idle = presentation.clips.get(&state.motions[0].clip).unwrap();
        assert!(!idle.transform_tracks.is_empty());
        let mut runtime = AnimationControllerRuntime::in_state(controller, spec.state).unwrap();
        runtime.set_float("Move Speed", 0.25).unwrap();
        let selection = runtime.motion_selection(controller).unwrap().unwrap();
        assert!(selection.second.is_some());
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
                display_name: "debug_viewer".to_owned(),
                command: "!join".parse().unwrap(),
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
        let actor_id = StableId::new("twitch:debug_viewer").unwrap();
        let commands = [
            ChatCommand::SelectRole(StableId::new("builder").unwrap()),
            ChatCommand::Build(StableId::new("house").unwrap()),
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
                    display_name: "debug_viewer".to_owned(),
                    command,
                });
        }
        app.update();

        let simulation = &app.world().resource::<SimulationRuntime>().0;
        assert_eq!(simulation.actors[&actor_id].role.as_str(), "role:builder");
        assert_eq!(simulation.buildings.len(), 1);
        assert_eq!(town_resource_amount(simulation, "resource:food"), 4_500);
        assert_eq!(town_resource_amount(simulation, "resource:gold"), 4_750);
        assert_eq!(town_resource_amount(simulation, "resource:ore"), 4_500);
        assert_eq!(town_resource_amount(simulation, "resource:wood"), 4_500);
        assert_eq!(
            simulation.active_vote.as_ref().map(|vote| &vote.technology),
            Some(&eligible_technology)
        );
        assert_eq!(simulation.active_event, Some(TownEvent::Festival));
        let saved_building_id = simulation.buildings.keys().next().unwrap().clone();
        let runtime_building_ids: Vec<_> = app
            .world_mut()
            .query::<&RuntimeBuilding>()
            .iter(app.world())
            .map(|building| building.id.clone())
            .collect();
        assert_eq!(runtime_building_ids, vec![saved_building_id]);
        assert!(save_path.is_file());
        assert!(
            app.world()
                .resource::<CommandFeedback>()
                .0
                .contains("commands: !join")
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
}
