use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, EguiStartupSet, egui};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use stream_town_domain::{ChatCommand, GameConfig, GeneratedWorld, generate_world};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum ToolTab {
    #[default]
    Migration,
    Content,
    Technology,
    World,
    Runtime,
    Twitch,
    Validation,
    Inspector,
}

impl ToolTab {
    const ALL: [Self; 8] = [
        Self::Migration,
        Self::Content,
        Self::Technology,
        Self::World,
        Self::Runtime,
        Self::Twitch,
        Self::Validation,
        Self::Inspector,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Migration => "Migration",
            Self::Content => "Content",
            Self::Technology => "Technology",
            Self::World => "World + Nav",
            Self::Runtime => "Runtime",
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
    generated_world: Option<GeneratedWorld>,
}

impl Default for ToolState {
    fn default() -> Self {
        Self {
            tab: ToolTab::default(),
            unity_root: "..".to_owned(),
            command: "!join".to_owned(),
            status: "Ready. Migration operations are read-only by default.".to_owned(),
            config: GameConfig::default(),
            generated_world: None,
        }
    }
}

fn main() {
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
        ToolTab::Content => content_tab(ui),
        ToolTab::Technology => technology_tab(ui),
        ToolTab::World => world_tab(ui, &mut state),
        ToolTab::Runtime => runtime_tab(ui, &mut state),
        ToolTab::Twitch => twitch_tab(ui),
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
    ui.label("Manifest stages: discovered → referenced → converted → manually reviewed → packaged");
}

fn content_tab(ui: &mut egui::Ui) {
    ui.heading("Content catalog and archetypes");
    ui.label(
        "Versioned RON catalogs use stable IDs. Unity GUIDs exist only in migration manifests.",
    );
    ui.label("Editors for buildings, roles, events, equipment, and archetypes attach here.");
}

fn technology_tab(ui: &mut egui::Ui) {
    ui.heading("Technology graph");
    ui.label("Node/group editing, cycle detection, dangling-reference diagnostics, minimap, and undo/redo share one graph asset.");
    ui.colored_label(
        egui::Color32::YELLOW,
        "Graph canvas wiring is scaffolded; catalog import supplies the initial nodes.",
    );
}

fn world_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("World-generation and navigation lab");
    ui.add(egui::DragValue::new(&mut state.config.world.seed).prefix("Seed "));
    ui.add(egui::Slider::new(&mut state.config.world.width, 8..=256).text("Width"));
    ui.add(egui::Slider::new(&mut state.config.world.height, 8..=256).text("Height"));
    if ui.button("Generate deterministic preview").clicked() {
        let world = generate_world(&state.config.world);
        state.status = format!(
            "Generated {}×{} world with {} resources; hash {}",
            world.navigation.width(),
            world.navigation.height(),
            world.resources.len(),
            &world.deterministic_hash[..16]
        );
        state.generated_world = Some(world);
    }
    if let Some(world) = &state.generated_world {
        ui.monospace(format!("Hash: {}", world.deterministic_hash));
        ui.label(format!("Resources: {}", world.resources.len()));
        ui.label(format!("Generator version: {}", world.generator_version));
    }
}

fn runtime_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Runtime developer console");
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut state.command);
        if ui.button("Validate command").clicked() {
            state.status = match state.command.parse::<ChatCommand>() {
                Ok(command) => format!("Valid command: {command:?}"),
                Err(error) => format!("Command error: {error}"),
            };
        }
    });
    ui.label("Live play/pause, spawn, vote/event injection, frame capture, save/load, and profiling connect to the game bridge here.");
}

fn twitch_tab(ui: &mut egui::Ui) {
    ui.heading("Twitch setup and diagnostics");
    ui.label("Device authorization, account/scope validation, reconnect status, revoke, and OS credential storage.");
    ui.colored_label(
        egui::Color32::LIGHT_BLUE,
        "No credentials are stored in repository assets.",
    );
}

fn validation_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Asset validator and packager");
    if ui.button("Validate current configuration").clicked() {
        state.status = match state.config.validate() {
            Ok(()) => "Configuration valid".to_owned(),
            Err(error) => format!("Configuration error: {error}"),
        };
    }
    ui.label("Checks: duplicate IDs, missing references, orphan content, GLB clips/skins/materials, shader/effect compatibility, and release manifest.");
}

fn inspector_tab(ui: &mut egui::Ui) {
    ui.heading("ECS/resource inspector");
    ui.label("The inspector window is enabled while this tab is selected.");
}
