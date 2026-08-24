use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex, mpsc},
    thread,
    time::{Duration, Instant},
};

mod technology_graph;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, EguiStartupSet, egui};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use stream_town_domain::{
    BuildingHealthDisplayMode, ChatCommand, ContentCatalog, DisplayMode, FoliageHabitat,
    FoliageLayerDef, GameConfig, GeneratedWorld, GridPos, NameDisplayMode, PlayerSettings,
    PlayerSettingsStore, PostProcessAntiAliasing, PresentationCatalog, RoleDef, RoleEquipmentDef,
    RuntimeConsoleAction, RuntimeConsoleRequest, RuntimeConsoleStatus, RuntimeConsoleStore,
    StableId, TechGroup, TechNode, TechnologyGraphLayout,
};
use stream_town_game::twitch::{
    CredentialVault, DeviceAuthorization, OAuthClient, TokenValidation, TwitchControl, TwitchEvent,
    TwitchStatus, TwitchTransport, TwitchUserIdentity,
};
use technology_graph::{TechnologyGraphViewState, show as show_technology_graph};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum ToolTab {
    #[default]
    Migration,
    Authority,
    Content,
    Roles,
    Technology,
    World,
    Runtime,
    Settings,
    Twitch,
    Validation,
    Inspector,
}

impl ToolTab {
    const ALL: [Self; 11] = [
        Self::Migration,
        Self::Authority,
        Self::Content,
        Self::Roles,
        Self::Technology,
        Self::World,
        Self::Runtime,
        Self::Settings,
        Self::Twitch,
        Self::Validation,
        Self::Inspector,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Migration => "Migration",
            Self::Authority => "Game Authority",
            Self::Content => "Content",
            Self::Roles => "Roles",
            Self::Technology => "Technology",
            Self::World => "World + Nav",
            Self::Runtime => "Runtime",
            Self::Settings => "Settings",
            Self::Twitch => "Twitch",
            Self::Validation => "Validation",
            Self::Inspector => "ECS Inspector",
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
    generated_world: Option<GeneratedWorld>,
    preview_path: Vec<GridPos>,
    path_start: GridPos,
    path_goal: GridPos,
    technology_search: String,
    selected_group: Option<StableId>,
    technology_draft: Option<TechnologyDraft>,
    catalog_path: String,
    technology_layout: TechnologyGraphLayout,
    technology_layout_path: String,
    technology_graph_view: TechnologyGraphViewState,
    role_search: String,
    selected_role: Option<StableId>,
    role_draft: Option<RoleDraft>,
    new_role_id: String,
    new_role_name: String,
    selected_foliage: Option<StableId>,
    foliage_draft: Option<FoliageLayerDef>,
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

#[derive(Clone)]
struct AuthoringSnapshot {
    catalog: ContentCatalog,
    technology_layout: TechnologyGraphLayout,
}

#[derive(Debug)]
enum TwitchToolEvent {
    Device(DeviceAuthorization),
    Authorized(TokenValidation),
    Progress(String),
    Diagnostic {
        validation: TokenValidation,
        channel: TwitchUserIdentity,
    },
    GameMasterResolved(TwitchUserIdentity),
    RewardCaptured(String),
    Cleared,
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum WorldPreviewLayer {
    #[default]
    Elevation,
    Navigation,
    Resources,
    Foliage,
}

impl WorldPreviewLayer {
    const ALL: [Self; 4] = [
        Self::Elevation,
        Self::Navigation,
        Self::Resources,
        Self::Foliage,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Elevation => "Elevation + water",
            Self::Navigation => "Navigation occupancy",
            Self::Resources => "Resources",
            Self::Foliage => "Foliage layers",
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
        let selected_role = catalog.roles.keys().next().cloned();
        let role_draft = selected_role
            .as_ref()
            .and_then(|id| role_draft(&catalog, id));
        let selected_foliage = catalog.foliage.first().map(|layer| layer.id.clone());
        let foliage_draft = selected_foliage.as_ref().and_then(|id| {
            catalog
                .foliage
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
        Self {
            tab: ToolTab::default(),
            unity_root: "..".to_owned(),
            command: "!join".to_owned(),
            status: "Ready. Migration operations are read-only by default.".to_owned(),
            config,
            config_path: config_path.display().to_string(),
            player_settings,
            catalog,
            presentation,
            generated_world: None,
            preview_path: Vec::new(),
            path_start: GridPos { x: 20, z: 32 },
            path_goal: GridPos { x: 44, z: 32 },
            technology_search: String::new(),
            selected_group,
            technology_draft: None,
            catalog_path: default_catalog_path().display().to_string(),
            technology_layout,
            technology_layout_path: default_technology_layout_path().display().to_string(),
            technology_graph_view: TechnologyGraphViewState::default(),
            role_search: String::new(),
            selected_role,
            role_draft,
            new_role_id: "role:new".to_owned(),
            new_role_name: "New Role".to_owned(),
            selected_foliage,
            foliage_draft,
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
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Stream Town Tools".to_owned(),
                resolution: (1_440, 900).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new().run_if(resource_equals(ToolInspector(true))))
        .init_resource::<ToolState>()
        .insert_resource(ToolInspector(false))
        .add_systems(
            PreStartup,
            setup_camera.before(EguiStartupSet::InitContexts),
        )
        .add_systems(EguiPrimaryContextPass, tools_ui)
        .run();
    Ok(())
}

#[derive(Resource, Clone, Copy, Eq, PartialEq)]
struct ToolInspector(bool);

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn tools_ui(
    mut contexts: EguiContexts,
    mut state: ResMut<ToolState>,
    mut inspector: ResMut<ToolInspector>,
) -> Result {
    poll_twitch_tool_events(&mut state);
    poll_tool_job_events(&mut state);
    poll_runtime_console(&mut state);
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
                    inspector.0 = tab == ToolTab::Inspector;
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
        ToolTab::Content => content_tab(ui, &state),
        ToolTab::Roles => roles_tab(ui, &mut state),
        ToolTab::Technology => technology_tab(ui, &mut state),
        ToolTab::World => world_tab(ui, &mut state),
        ToolTab::Runtime => runtime_tab(ui, &mut state),
        ToolTab::Settings => settings_tab(ui, &mut state),
        ToolTab::Twitch => twitch_tab(ui, &mut state),
        ToolTab::Validation => validation_tab(ui, &mut state),
        ToolTab::Inspector => inspector_tab(ui),
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
        "These values drive deterministic world creation and simulation. Save the project baseline for source-controlled defaults, or write a local runtime override for immediate testing.",
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
        if ui.button("Save project baseline").clicked() {
            state.status = match save_game_config(&state.config, &state.config_path) {
                Ok(path) => format!("Saved project game configuration to {}", path.display()),
                Err(error) => format!("Could not save project game configuration: {error:#}"),
            };
        }
        if ui.button("Write local runtime override").clicked() {
            state.status = match save_runtime_config(&state.config) {
                Ok(path) => format!("Saved runtime override to {}", path.display()),
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

fn content_tab(ui: &mut egui::Ui, state: &ToolState) {
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
        ui.label(format!("Enemies / camps: {enemies} / {camps}"));
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

fn roles_tab(ui: &mut egui::Ui, state: &mut ToolState) {
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

fn technology_tab(ui: &mut egui::Ui, state: &mut ToolState) {
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

fn world_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("World-generation and navigation lab");
    ui.label(
        "Tune the Unity-compatible terrain scale and authored foliage noise, generate deterministic previews, inspect occupancy, and exercise the same A* grid used by the game.",
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
        if ui.button("Save project world config").clicked() {
            state.status = match save_game_config(&state.config, &state.config_path) {
                Ok(path) => format!("Saved project game configuration to {}", path.display()),
                Err(error) => format!("Could not save game configuration: {error:#}"),
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
        let mut apply_foliage = false;
        let mut reset_foliage = false;
        if let Some(layer) = state.foliage_draft.as_mut() {
            ui.horizontal_wrapped(|ui| {
                ui.monospace(layer.id.to_string());
                egui::ComboBox::from_label("Habitat")
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
            ui.horizontal(|ui| {
                ui.label("Unity source");
                ui.text_edit_singleline(&mut layer.source_path);
            });
            ui.horizontal(|ui| {
                ui.label("Material source");
                ui.text_edit_singleline(&mut layer.material_source_path);
            });
            ui.collapsing(format!("Model variants ({})", layer.variants.len()), |ui| {
                for (index, variant) in layer.variants.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.label(format!("Variant {}", index + 1));
                        ui.horizontal(|ui| {
                            ui.label("Source model");
                            ui.text_edit_singleline(&mut variant.source_model);
                        });
                        ui.horizontal(|ui| {
                            ui.label("GLB asset path");
                            ui.text_edit_singleline(&mut variant.asset_path);
                        });
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
                }
            });
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
                    state.preview_path.clear();
                    status
                }
                (Err(error), _) => format!("World configuration is invalid: {error}"),
                (_, Err(error)) => format!("Foliage catalog is invalid: {error}"),
            };
        }
        egui::ComboBox::from_label("Preview")
            .selected_text(state.world_preview_layer.label())
            .show_ui(ui, |ui| {
                for layer in WorldPreviewLayer::ALL {
                    ui.selectable_value(&mut state.world_preview_layer, layer, layer.label());
                }
            });
    });
    let max_x = state.config.world.width.saturating_sub(1);
    let max_z = state.config.world.height.saturating_sub(1);
    ui.horizontal_wrapped(|ui| {
        ui.add(
            egui::DragValue::new(&mut state.path_start.x)
                .range(0..=max_x)
                .prefix("Start x "),
        );
        ui.add(
            egui::DragValue::new(&mut state.path_start.z)
                .range(0..=max_z)
                .prefix("z "),
        );
        ui.add(
            egui::DragValue::new(&mut state.path_goal.x)
                .range(0..=max_x)
                .prefix("Goal x "),
        );
        ui.add(
            egui::DragValue::new(&mut state.path_goal.z)
                .range(0..=max_z)
                .prefix("z "),
        );
        if ui.button("Plan A* path").clicked() {
            let result = state.generated_world.as_ref().map(|world| {
                world
                    .navigation
                    .find_path(state.path_start, state.path_goal)
            });
            match result {
                Some(Ok(path)) => {
                    state.status = format!("Planned {} navigation steps", path.len());
                    state.preview_path = path;
                }
                Some(Err(error)) => {
                    state.status = format!("Navigation error: {error}");
                    state.preview_path.clear();
                }
                None => "Generate a world before planning a path".clone_into(&mut state.status),
            }
        }
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
            &state.preview_path,
            state.world_preview_layer,
            state.selected_foliage.as_ref(),
            state.config.world.water_level_centimetres,
        );
    }
}

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
    ui.label("Camera and input");
    ui.add(
        egui::Slider::new(
            &mut state.player_settings.camera.pan_sensitivity,
            0.0..=100.0,
        )
        .text("Mouse pan sensitivity"),
    );
    ui.add(
        egui::Slider::new(
            &mut state.player_settings.camera.keyboard_pan_sensitivity,
            0.0..=100.0,
        )
        .text("WASD sensitivity"),
    );
    ui.add(
        egui::Slider::new(
            &mut state.player_settings.camera.zoom_sensitivity,
            0.0..=100.0,
        )
        .text("Zoom sensitivity"),
    );
    ui.add(
        egui::Slider::new(
            &mut state.player_settings.camera.edge_scroll_sensitivity,
            0.0..=100.0,
        )
        .text("Edge-scroll sensitivity"),
    );
    ui.add(
        egui::Slider::new(
            &mut state.player_settings.camera.field_of_view_degrees,
            30..=120,
        )
        .text("Field of view"),
    );
    ui.checkbox(
        &mut state.player_settings.camera.keyboard_movement,
        "Keyboard movement",
    );
    ui.checkbox(
        &mut state.player_settings.camera.edge_scrolling,
        "Edge scrolling",
    );
    ui.checkbox(
        &mut state.player_settings.camera.mouse_controls,
        "Mouse controls",
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
                ui.label("Foliage / separated");
                ui.monospace(format!(
                    "{} / {}",
                    status.foliage_instances, status.crowd_adjusted_agents
                ));
                ui.end_row();
                ui.label("Twitch");
                ui.monospace(&status.twitch_status);
                ui.label("Save");
                ui.monospace(if status.save_exists {
                    "Available"
                } else {
                    "Missing"
                });
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
        .args(["run", "-p", "stream_town_game"])
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
    ui.checkbox(
        &mut state.config.twitch.require_broadcaster_connect,
        "Require the broadcaster's per-session !connect code",
    );
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
    ui.colored_label(
        egui::Color32::LIGHT_BLUE,
        "No client secret or OAuth credential is stored in repository assets.",
    );
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

fn save_runtime_config(config: &GameConfig) -> anyhow::Result<std::path::PathBuf> {
    save_game_config(config, ".stream-town/config.ron")
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

fn inspector_tab(ui: &mut egui::Ui) {
    ui.heading("ECS/resource inspector");
    ui.label("The inspector window is enabled while this tab is selected.");
}

fn technology_draft(catalog: &ContentCatalog, id: &StableId) -> Option<TechnologyDraft> {
    let node = catalog.technology.nodes.get(id)?;
    Some(TechnologyDraft {
        id: id.clone(),
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
    })
}

fn default_catalog_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content/catalog.ron")
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

fn parse_stable_id_list(value: &str) -> Result<Vec<StableId>, String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| StableId::new(value.to_owned()).map_err(|error| error.to_string()))
        .collect()
}

fn parse_stable_id_set(value: &str) -> Result<BTreeSet<StableId>, String> {
    parse_stable_id_list(value).map(|values| values.into_iter().collect())
}

fn optional_text(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_owned())
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
    let resource = optional_text(&draft.resource)
        .map(StableId::new)
        .transpose()
        .map_err(|error| error.to_string())?;
    let mut value = draft.value;
    value.resource = resource;
    value.station_kinds = parse_stable_id_set(&draft.station_kinds)?;
    value.target_kinds = parse_stable_id_set(&draft.target_kinds)?;
    value.granted_abilities = parse_stable_id_list(&draft.granted_abilities)?;
    value.equipment = draft.has_equipment.then(|| RoleEquipmentDef {
        body_nodes: draft.body_nodes.map(|node| node.trim().to_owned()),
        left_hand_node: optional_text(&draft.left_hand_node),
        right_hand_node: optional_text(&draft.right_hand_node),
        helmet_node: optional_text(&draft.helmet_node),
        carry_animation: optional_text(&draft.carry_animation),
        left_hand_permanent: draft.left_hand_permanent,
    });
    let mut candidate = state.catalog.clone();
    candidate.roles.insert(draft.id, value);
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
}

fn refresh_catalog_drafts(state: &mut ToolState) {
    refresh_technology_draft(state);
    refresh_role_draft(state);
    refresh_foliage_draft(state);
    state.generated_world = None;
    state.preview_path.clear();
}

fn apply_technology_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .technology_draft
        .clone()
        .ok_or_else(|| "no technology selected".to_owned())?;
    let prerequisites = draft
        .prerequisites
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| StableId::new(value.to_owned()).map_err(|error| error.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    let unlocks = parse_stable_id_list(&draft.unlocks)?;
    let objectives = parse_stable_id_list(&draft.objectives)?;
    let mut candidate = state.catalog.clone();
    let node = candidate
        .technology
        .nodes
        .get_mut(&draft.id)
        .ok_or_else(|| format!("missing technology {}", draft.id))?;
    node.display_name = draft.display_name;
    node.description = draft.description;
    node.age = draft.age;
    node.tier = draft.tier;
    node.group.clone_from(&draft.group);
    node.prerequisites = prerequisites;
    node.unlocks = unlocks;
    node.objectives = objectives;
    node.icon_path = draft.icon_path;
    node.initially_unlocked = draft.initially_unlocked;
    node.unavailable = draft.unavailable;
    for group in candidate.technology.groups.values_mut() {
        group.nodes.retain(|reference| reference != &draft.id);
    }
    if let Some(group_id) = &draft.group {
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
    path: &[GridPos],
    layer: WorldPreviewLayer,
    selected_foliage: Option<&StableId>,
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
                WorldPreviewLayer::Resources | WorldPreviewLayer::Foliage => {
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
    if layer == WorldPreviewLayer::Resources {
        for resource in &world.resources {
            let color = match resource.kind.as_str() {
                value if value.contains("wood") => egui::Color32::from_rgb(46, 185, 76),
                value if value.contains("ore") => egui::Color32::from_rgb(190, 196, 207),
                value if value.contains("food") => egui::Color32::from_rgb(219, 82, 81),
                value if value.contains("fish") => egui::Color32::from_rgb(83, 207, 229),
                _ => egui::Color32::GOLD,
            };
            ui.painter().circle_filled(
                preview_grid_point(rect, width, height, resource.position),
                2.25,
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
    for segment in path.windows(2) {
        ui.painter().line_segment(
            [
                preview_grid_point(rect, width, height, segment[0]),
                preview_grid_point(rect, width, height, segment[1]),
            ],
            egui::Stroke::new(2.0, egui::Color32::YELLOW),
        );
    }
    if let Some(start) = path.first() {
        ui.painter().circle_filled(
            preview_grid_point(rect, width, height, *start),
            3.5,
            egui::Color32::LIGHT_GREEN,
        );
    }
    if let Some(goal) = path.last() {
        ui.painter().circle_filled(
            preview_grid_point(rect, width, height, *goal),
            3.5,
            egui::Color32::LIGHT_RED,
        );
    }
    ui.label(match layer {
        WorldPreviewLayer::Elevation => "Blue is at/below water level; green through stone encodes terrain elevation.",
        WorldPreviewLayer::Navigation => "Green cells are walkable; charcoal cells are blocked.",
        WorldPreviewLayer::Resources => "Green/grey/red/cyan markers identify generated wood, ore, food, and fish resources.",
        WorldPreviewLayer::Foliage => "Bright markers belong to the selected foliage layer; dim markers belong to other layers.",
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
        state.technology_draft.as_mut().unwrap().prerequisites = node_id.to_string();
        let before = state.catalog.clone();
        assert!(apply_technology_draft(&mut state).is_err());
        assert_eq!(state.catalog, before);
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
        draft.station_kinds = "station:food, station:fish".to_owned();
        draft.target_kinds = "target:bush, target:fish".to_owned();
        draft.granted_abilities = "resource:food, role_flag:resource".to_owned();

        apply_role_draft(&mut state).unwrap();
        let role = &state.catalog.roles[&id];
        assert_eq!(role.base_action_amount, 7);
        assert!(
            role.station_kinds
                .contains(&StableId::new("station:fish").unwrap())
        );
        state.catalog.validate().unwrap();

        let before = state.catalog.clone();
        state.role_draft.as_mut().unwrap().body_nodes[0].clear();
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
    fn checked_in_authoring_assets_pass_headless_validation() {
        let summary = validate_authoring_assets().unwrap();
        assert!(summary.contains("roles"));
        assert!(summary.contains("technologies"));
    }
}
