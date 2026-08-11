use std::{collections::VecDeque, path::PathBuf, time::Duration};

use avian3d::prelude::PhysicsPlugins;
use bevy::{prelude::*, window::WindowResolution};
use stream_town_domain::{
    ActorKind, ChatCommand, GameConfig, GeneratedWorld, GridPos, NativeSaveStore, SavedActor,
    StableId, TownEvent, WorldSimulation, WorldSnapshot, generate_world,
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

#[derive(Resource)]
pub struct RuntimeConfig(pub GameConfig);

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

#[derive(Resource, Default)]
struct InjectedCommands(VecDeque<ChatCommand>);

#[derive(Component)]
struct StateEntity;

#[derive(Component)]
struct WorldEntity;

#[derive(Component)]
struct Agent {
    id: StableId,
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

pub struct StreamTownGamePlugin;

impl Plugin for StreamTownGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<SessionStats>()
            .init_resource::<InjectedCommands>()
            .insert_resource(SaveRuntime {
                store: NativeSaveStore::new(
                    PathBuf::from(".stream-town").join("StreamTownSave.stbevy"),
                ),
            })
            .add_systems(Startup, setup_camera)
            .add_systems(OnEnter(GameState::Boot), finish_boot)
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
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
                    game_input,
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
    let resolution = WindowResolution::new(config.window.width, config.window.height);
    let title = config.window.title.clone();
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.025, 0.04, 0.055)))
        .insert_resource(RuntimeConfig(config))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title,
                resolution,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(StreamTownGamePlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn finish_boot(mut next_state: ResMut<NextState<GameState>>) {
    info!("Stream Town boot validation complete");
    next_state.set(GameState::MainMenu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        StateEntity,
        Text2d::new("STREAM TOWN\n\nENTER  Generate Town\nC  Credits\nESC  Quit"),
        TextFont {
            font_size: FontSize::Px(48.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::srgb(0.86, 0.95, 0.84)),
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
    mut next_state: ResMut<NextState<GameState>>,
) {
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
        Sprite::from_color(Color::srgb(0.09, 0.22, 0.12), world_size),
        Transform::from_xyz(0.0, 0.0, -5.0),
    ));

    for resource in &generated.resources {
        let position = grid_to_world(resource.position, &config.0);
        let color = match resource.kind.as_str() {
            "resource:wood" => Color::srgb(0.16, 0.46, 0.18),
            "resource:ore" => Color::srgb(0.46, 0.50, 0.55),
            _ => Color::srgb(0.74, 0.64, 0.18),
        };
        commands.spawn((
            WorldEntity,
            GridLocation(resource.position),
            Sprite::from_color(color, Vec2::splat(config.0.world.cell_size * 0.55)),
            Transform::from_xyz(position.x, position.y, 0.0),
        ));
    }

    let hall = grid_to_world(town_hall_position, &config.0);
    commands.spawn((
        WorldEntity,
        TownHall,
        GridLocation(town_hall_position),
        Sprite::from_color(
            Color::srgb(0.42, 0.26, 0.12),
            Vec2::splat(config.0.world.cell_size * 1.8),
        ),
        Transform::from_xyz(hall.x, hall.y, 1.0),
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
            let actor_id =
                StableId::new(format!("actor:viewer_{spawned:04}")).expect("generated ID");
            simulation.join_player(actor_id.clone(), position);
            commands.spawn((
                WorldEntity,
                GridLocation(position),
                Agent {
                    id: actor_id,
                    origin: position,
                    path: Vec::new(),
                    path_index: 0,
                    target,
                },
                Sprite::from_color(
                    Color::srgb(0.35, 0.72, 0.95),
                    Vec2::splat(config.0.world.cell_size * 0.35),
                ),
                Transform::from_xyz(world_position.x, world_position.y, 2.0),
            ));
            spawned += 1;
            if spawned >= config.0.gameplay.initial_agents {
                break 'cells;
            }
        }
    }

    commands.spawn((
        WorldEntity,
        Hud,
        Text2d::new(format!(
            "{} agents  •  world {}\nF5 Save  •  ! commands simulated  •  ESC Menu",
            spawned,
            &generated.deterministic_hash[..12]
        )),
        TextFont {
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, world_size.y * 0.5 + 36.0, 10.0),
    ));
    commands.insert_resource(WorldRuntime { generated });
    commands.insert_resource(SimulationRuntime(simulation));
    next_state.set(GameState::InGame);
}

fn nearest_walkable(world: &GeneratedWorld, desired: GridPos) -> Option<GridPos> {
    if world.navigation.is_walkable(desired) {
        return Some(desired);
    }
    let limit = world.navigation.width().max(world.navigation.height());
    for radius in 1..limit {
        for z in desired.z.saturating_sub(radius)
            ..=(desired.z + radius).min(world.navigation.height() - 1)
        {
            for x in desired.x.saturating_sub(radius)
                ..=(desired.x + radius).min(world.navigation.width() - 1)
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
        let target = grid_to_world(next, &config.0).extend(transform.translation.z);
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

fn game_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    save: Res<SaveRuntime>,
    world: Res<WorldRuntime>,
    stats: Res<SessionStats>,
    simulation: Res<SimulationRuntime>,
    agents: Query<(&Agent, &GridLocation)>,
    mut injected: ResMut<InjectedCommands>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
    if keyboard.just_pressed(KeyCode::F5) {
        let snapshot = snapshot_world(&world, &stats, &simulation, &agents);
        match save.store.write(&snapshot) {
            Ok(()) => info!(path = %save.store.path().display(), "native save written"),
            Err(error) => error!(%error, "native save failed"),
        }
    }
    if keyboard.just_pressed(KeyCode::KeyJ) {
        injected.0.push_back(ChatCommand::Join);
    }
}

fn process_injected_commands(
    mut commands: ResMut<InjectedCommands>,
    mut stats: ResMut<SessionStats>,
    mut simulation: ResMut<SimulationRuntime>,
) {
    let debug_viewer = StableId::new("twitch:debug_viewer").expect("static ID");
    while let Some(command) = commands.0.pop_front() {
        match &command {
            ChatCommand::Join => {
                simulation
                    .0
                    .join_player(debug_viewer.clone(), GridPos { x: 32, z: 32 });
            }
            ChatCommand::SelectRole(role) => {
                let _ = simulation.0.assign_role(&debug_viewer, role.clone());
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
                let _ = simulation.0.cast_vote(&debug_viewer, true);
            }
            ChatCommand::TriggerEvent(_) => {
                simulation.0.trigger_event(TownEvent::Festival);
            }
            ChatCommand::Save | ChatCommand::Help => {}
        }
        info!(?command, "processed injected Twitch command");
        stats.commands_processed += 1;
    }
}

fn update_hud(
    stats: Res<SessionStats>,
    simulation: Res<SimulationRuntime>,
    agents: Query<&Agent>,
    mut hud: Single<&mut Text2d, With<Hud>>,
) {
    if !stats.is_changed() {
        return;
    }
    let first_id = agents
        .iter()
        .next()
        .map_or("none", |agent| agent.id.as_str());
    hud.0 = format!(
        "{} agents  •  {:.0}s  •  {} routes  •  {} commands  •  {:?} / {:?}\nF5 Save  •  J Inject !join  •  ESC Menu  •  first {first_id}",
        agents.iter().len(),
        stats.elapsed_seconds,
        stats.paths_completed,
        stats.commands_processed,
        simulation.0.season,
        simulation.0.weather,
    );
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
                kind: ActorKind::Player,
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
    }
}

fn spawn_credits(mut commands: Commands) {
    commands.spawn((
        StateEntity,
        Text2d::new(
            "STREAM TOWN\nOriginal project by Jayden Hunter and contributors\nBevy migration by Human Bean Games\n\nESC  Main Menu",
        ),
        TextFont {
            font_size: FontSize::Px(36.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::srgb(0.86, 0.95, 0.84)),
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

fn grid_to_world(position: GridPos, config: &GameConfig) -> Vec2 {
    Vec2::new(
        (f32::from(position.x) - f32::from(config.world.width) * 0.5) * config.world.cell_size,
        (f32::from(position.z) - f32::from(config.world.height) * 0.5) * config.world.cell_size,
    )
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
    }
}
