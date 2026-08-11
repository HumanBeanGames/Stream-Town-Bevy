pub mod twitch;

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result as AnyResult};
use avian3d::prelude::PhysicsPlugins;
use bevy::{
    asset::AssetPlugin,
    camera::ScalingMode,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
    window::{PrimaryWindow, WindowResolution},
};
use stream_town_domain::{
    ActorKind, ArchetypeKind, ArchetypeScene, ChatCommand, ContentCatalog, GameConfig,
    GeneratedWorld, GridPos, NativeSaveStore, SavedActor, StableId, TownEvent, WorldSimulation,
    WorldSnapshot, generate_world,
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
struct RenderAssets {
    cube: Handle<Mesh>,
    ground: Handle<StandardMaterial>,
    wood: Handle<StandardMaterial>,
    ore: Handle<StandardMaterial>,
    food: Handle<StandardMaterial>,
    building: Handle<StandardMaterial>,
    enemy_idle: Handle<StandardMaterial>,
    enemy_moving: Handle<StandardMaterial>,
    player_idle: Handle<StandardMaterial>,
    player_moving: Handle<StandardMaterial>,
    selection: Handle<StandardMaterial>,
}

#[derive(Component)]
struct StateEntity;

#[derive(Component)]
struct WorldEntity;

#[derive(Component)]
struct Agent {
    id: StableId,
    kind: ActorKind,
    origin: GridPos,
    path: Vec<GridPos>,
    path_index: usize,
    target: GridPos,
}

#[derive(Component, Clone, Copy)]
struct GridLocation(GridPos);

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct TownHall;

#[derive(Component)]
struct SelectionMarker;

#[derive(Component)]
struct TownCamera;

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
}

pub struct StreamTownGamePlugin;

impl Plugin for StreamTownGamePlugin {
    fn build(&self, app: &mut App) {
        if !app.world().contains_resource::<RuntimeConfig>() {
            app.insert_resource(RuntimeConfig(GameConfig::default()));
        }
        if !app.world().contains_resource::<RuntimeContent>() {
            app.insert_resource(RuntimeContent(embedded_content()));
        }
        if !app.world().contains_resource::<RuntimeAssetRoot>() {
            app.insert_resource(RuntimeAssetRoot(locate_asset_root()));
        }
        app.init_state::<GameState>()
            .init_resource::<SessionStats>()
            .init_resource::<InjectedCommands>()
            .init_resource::<TwitchConnection>()
            .init_resource::<SelectedCell>()
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
                    animate_agents,
                    camera_controls,
                    select_grid_cell,
                    game_input,
                    save_input,
                    load_input,
                    capture_screenshot,
                    process_injected_commands,
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

fn setup_rendering(
    mut commands: Commands,
    meshes: Option<ResMut<Assets<Mesh>>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
) {
    let (Some(mut meshes), Some(mut materials)) = (meshes, materials) else {
        commands.insert_resource(RenderAssets::default());
        return;
    };
    commands.spawn((
        TownCamera,
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 520.0,
            },
            ..OrthographicProjection::default_3d()
        }),
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
    commands.insert_resource(RenderAssets {
        cube: meshes.add(Cuboid::default()),
        ground: materials.add(Color::srgb(0.09, 0.22, 0.12)),
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
    });
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

fn generate_and_spawn_world(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    render: Res<RenderAssets>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    mut selected: ResMut<SelectedCell>,
    mut cameras: Query<&mut Transform, With<TownCamera>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    selected.0 = None;
    if let Ok(mut camera) = cameras.single_mut() {
        *camera = Transform::from_xyz(360.0, 420.0, 360.0).looking_at(Vec3::ZERO, Vec3::Y);
    }
    let mut generated = generate_world(&config.0.world);
    let centre = GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    };
    let town_hall_position = GridPos {
        x: (centre.x + 4).min(config.0.world.width - 2),
        z: centre.z,
    };
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
    commands.spawn((
        WorldEntity,
        Mesh3d(render.cube.clone()),
        MeshMaterial3d(render.ground.clone()),
        Transform::from_xyz(0.0, -0.15, 0.0).with_scale(Vec3::new(world_size.x, 0.3, world_size.y)),
    ));

    for resource in &generated.resources {
        let position = grid_to_world(resource.position, &config.0);
        let material = match resource.kind.as_str() {
            "resource:wood" => render.wood.clone(),
            "resource:ore" => render.ore.clone(),
            _ => render.food.clone(),
        };
        let scale = config.0.world.cell_size * 0.55;
        commands.spawn((
            WorldEntity,
            GridLocation(resource.position),
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(material),
            Transform::from_xyz(position.x, scale * 0.5, position.z).with_scale(Vec3::splat(scale)),
        ));
    }

    let hall = grid_to_world(town_hall_position, &config.0);
    let mut hall_entity = commands.spawn((
        WorldEntity,
        TownHall,
        GridLocation(town_hall_position),
        Transform::from_xyz(hall.x, 0.0, hall.z),
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
            Transform::from_xyz(hall.x, 0.0, hall.z)
                .with_scale(Vec3::splat(config.0.world.cell_size / 2.0)),
        ));
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
            Transform::from_xyz(hall.x, size.y * 0.5, hall.z).with_scale(size),
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
            let world_position = grid_to_world(position, &config.0);
            let actor_id = StableId::new(if spawned == 0 {
                "actor:enemy_0000".to_owned()
            } else {
                format!("actor:viewer_{spawned:04}")
            })
            .expect("generated ID");
            let kind = if spawned == 0 {
                ActorKind::Enemy
            } else {
                ActorKind::Player
            };
            simulation.join_player(actor_id.clone(), position);
            if kind == ActorKind::Enemy {
                let _ = simulation
                    .assign_role(&actor_id, StableId::new("role:enemy").expect("static ID"));
            }
            let real_scene = if spawned == 0 {
                archetype_scene_by_source(&content.0, ArchetypeKind::Enemy, "Enemy_Goblin.prefab")
            } else if spawned == 1 {
                archetype_scene_by_source(
                    &content.0,
                    ArchetypeKind::Player,
                    "Player_Character.prefab",
                )
            } else {
                None
            }
            .filter(|scene| {
                asset_server.is_some() && converted_asset_exists(&asset_root.0, &scene.asset_path)
            });
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
                0.0
            } else {
                base_scale.y * 0.5
            };
            let mut entity = commands.spawn((
                WorldEntity,
                GridLocation(position),
                Agent {
                    id: actor_id,
                    kind: kind.clone(),
                    origin: position,
                    path: Vec::new(),
                    path_index: 0,
                    target,
                },
                AgentAnimation {
                    base_scale,
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

fn archetype_scene_by_source<'a>(
    content: &'a ContentCatalog,
    kind: ArchetypeKind,
    source_suffix: &str,
) -> Option<&'a ArchetypeScene> {
    content
        .archetypes
        .values()
        .find(|archetype| archetype.kind == kind && archetype.source_path.ends_with(source_suffix))
        .and_then(default_archetype_scene)
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

fn move_agents(
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    world: Res<WorldRuntime>,
    mut simulation: ResMut<SimulationRuntime>,
    mut stats: ResMut<SessionStats>,
    mut agents: Query<(&mut Agent, &mut GridLocation, &mut Transform)>,
) {
    stats.elapsed_seconds += time.delta_secs_f64();
    simulation.0.tick(time.delta_secs());
    for (mut agent, mut location, mut transform) in &mut agents {
        if agent.path.is_empty() || agent.path_index >= agent.path.len() {
            if !agent.path.is_empty() {
                stats.paths_completed += 1;
                let previous_target = agent.target;
                agent.target = agent.origin;
                agent.origin = previous_target;
            }
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
        let mut target = grid_to_world(next, &config.0);
        target.y = transform.translation.y;
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
    config: Res<RuntimeConfig>,
    mut selected: ResMut<SelectedCell>,
    mut markers: Query<(&mut Transform, &mut Visibility), With<SelectionMarker>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let (Ok(window), Ok((camera, camera_transform))) = (windows.single(), cameras.single()) else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor) else {
        return;
    };
    let Some(world_position) =
        ray.plane_intersection_point(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
    else {
        return;
    };
    let Some(cell) = world_to_grid(world_position, &config.0) else {
        return;
    };
    selected.0 = Some(cell);
    if let Ok((mut transform, mut visibility)) = markers.single_mut() {
        let marker_position = grid_to_world(cell, &config.0);
        transform.translation.x = marker_position.x;
        transform.translation.z = marker_position.z;
        *visibility = Visibility::Visible;
    }
}

fn game_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut injected: ResMut<InjectedCommands>,
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
    world: Res<WorldRuntime>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    mut stats: ResMut<SessionStats>,
    mut simulation: ResMut<SimulationRuntime>,
    mut agents: Query<(Entity, &mut Agent, &mut GridLocation, &mut Transform)>,
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

    let saved_by_id: BTreeMap<StableId, SavedActor> = snapshot
        .actors
        .iter()
        .filter(|actor| matches!(actor.kind, ActorKind::Player | ActorKind::Enemy))
        .cloned()
        .map(|actor| (actor.id.clone(), actor))
        .collect();
    let mut restored_ids = BTreeSet::new();
    for (entity, mut agent, mut location, mut transform) in &mut agents {
        let Some(saved) = saved_by_id.get(&agent.id) else {
            ecs.entity(entity).despawn();
            continue;
        };
        let position =
            nearest_walkable(&world.generated, saved.grid_position).unwrap_or(saved.grid_position);
        let world_position = grid_to_world(position, &config.0);
        agent.kind = saved.kind.clone();
        agent.origin = position;
        agent.path.clear();
        agent.path_index = 0;
        agent.target = mirrored_target(&world.generated, position);
        location.0 = position;
        transform.translation.x = world_position.x;
        transform.translation.z = world_position.z;
        restored_ids.insert(saved.id.clone());
    }

    for saved in saved_by_id.values() {
        if restored_ids.contains(&saved.id) {
            continue;
        }
        let position =
            nearest_walkable(&world.generated, saved.grid_position).unwrap_or(saved.grid_position);
        let world_position = grid_to_world(position, &config.0);
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
            Transform::from_xyz(world_position.x, base_scale.y * 0.5, world_position.z)
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
    let automatic_path = if !*automatic_complete && *elapsed >= 3.0 {
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

fn process_injected_commands(
    mut ecs: Commands,
    mut queue: ResMut<InjectedCommands>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    mut stats: ResMut<SessionStats>,
    mut simulation: ResMut<SimulationRuntime>,
) {
    while let Some(pending) = queue.0.pop_front() {
        let actor_id = pending.actor_id;
        let command = pending.command;
        match &command {
            ChatCommand::Join => {
                if !simulation.0.actors.contains_key(&actor_id) {
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
                        let world_position = grid_to_world(position, &config.0);
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
                                base_scale.y * 0.5,
                                world_position.z,
                            )
                            .with_scale(base_scale),
                        ));
                    }
                }
            }
            ChatCommand::SelectRole(role) => {
                let _ = simulation.0.assign_role(&actor_id, role.clone());
            }
            ChatCommand::Build(archetype) => {
                let building_id = StableId::random("building");
                let _ = simulation.0.construct(
                    building_id,
                    archetype.clone(),
                    GridPos { x: 36, z: 32 },
                    &std::collections::BTreeMap::new(),
                );
            }
            ChatCommand::Vote(technology) => {
                if simulation.0.active_vote.is_none() {
                    let _ = simulation.0.start_technology_vote(technology.clone(), 30.0);
                }
                let _ = simulation.0.cast_vote(&actor_id, true);
            }
            ChatCommand::TriggerEvent(_) => {
                simulation.0.trigger_event(TownEvent::Festival);
            }
            ChatCommand::Save | ChatCommand::Help => {}
        }
        info!(user = %pending.display_name, ?command, "processed Twitch command");
        stats.commands_processed += 1;
    }
}

fn update_hud(
    stats: Res<SessionStats>,
    twitch: Res<TwitchConnection>,
    simulation: Res<SimulationRuntime>,
    agents: Query<&Agent>,
    mut hud: Single<&mut Text, With<Hud>>,
) {
    if !stats.is_changed() && !twitch.is_changed() {
        return;
    }
    let first_id = agents
        .iter()
        .next()
        .map_or("none", |agent| agent.id.as_str());
    hud.0 = format!(
        "{} agents | {:.0}s | {} routes | {} commands | {:?} / {:?} | Twitch: {}\nF1 Twitch Off | F2 Twitch On | F5 Save | F9 Load | F12 Capture | J Inject !join | WASD Pan | Q/E Zoom | Click Select | ESC Menu | first {first_id}",
        agents.iter().len(),
        stats.elapsed_seconds,
        stats.paths_completed,
        stats.commands_processed,
        simulation.0.season,
        simulation.0.weather,
        twitch_status_text(&twitch),
    );
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
        schema_version: 1,
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
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            bevy::state::app::StatesPlugin,
            bevy::input::InputPlugin,
        ))
        .insert_resource(RuntimeConfig(config))
        .add_plugins(StreamTownGamePlugin);

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
