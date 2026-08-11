use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex, mpsc},
    thread,
};

use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, EguiStartupSet, egui};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use stream_town_domain::{
    ChatCommand, ContentCatalog, GameConfig, GeneratedWorld, GridPos, PresentationCatalog,
    StableId, generate_world,
};
use stream_town_game::twitch::{
    CredentialVault, DeviceAuthorization, OAuthClient, TokenValidation,
};

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
    catalog: ContentCatalog,
    presentation: PresentationCatalog,
    generated_world: Option<GeneratedWorld>,
    preview_path: Vec<GridPos>,
    path_start: GridPos,
    path_goal: GridPos,
    technology_search: String,
    selected_group: Option<StableId>,
    technology_draft: Option<TechnologyDraft>,
    undo_catalogs: Vec<ContentCatalog>,
    redo_catalogs: Vec<ContentCatalog>,
    twitch_auth_events: Option<Arc<Mutex<mpsc::Receiver<TwitchToolEvent>>>>,
    twitch_device: Option<DeviceAuthorization>,
    twitch_validation: Option<TokenValidation>,
}

#[derive(Debug)]
enum TwitchToolEvent {
    Device(DeviceAuthorization),
    Authorized(TokenValidation),
    Diagnostic(TokenValidation),
    Cleared,
    Error(String),
}

#[derive(Clone)]
struct TechnologyDraft {
    id: StableId,
    display_name: String,
    description: String,
    age: String,
    tier: i32,
    prerequisites: String,
    initially_unlocked: bool,
    unavailable: bool,
}

impl Default for ToolState {
    fn default() -> Self {
        let catalog: ContentCatalog =
            ron::from_str(include_str!("../../../assets/content/catalog.ron"))
                .expect("checked-in content catalog must parse");
        catalog
            .validate()
            .expect("checked-in content catalog must validate");
        let selected_group = catalog.technology.groups.keys().next().cloned();
        let presentation: PresentationCatalog =
            ron::from_str(include_str!("../../../assets/content/presentation.ron"))
                .expect("checked-in presentation catalog must parse");
        presentation
            .validate()
            .expect("checked-in presentation catalog must validate");
        Self {
            tab: ToolTab::default(),
            unity_root: "..".to_owned(),
            command: "!join".to_owned(),
            status: "Ready. Migration operations are read-only by default.".to_owned(),
            config: stream_town_game::load_runtime_config().unwrap_or_default(),
            catalog,
            presentation,
            generated_world: None,
            preview_path: Vec::new(),
            path_start: GridPos { x: 20, z: 32 },
            path_goal: GridPos { x: 44, z: 32 },
            technology_search: String::new(),
            selected_group,
            technology_draft: None,
            undo_catalogs: Vec::new(),
            redo_catalogs: Vec::new(),
            twitch_auth_events: None,
            twitch_device: None,
            twitch_validation: None,
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
    poll_twitch_tool_events(&mut state);
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
        ToolTab::Content => content_tab(ui, &state),
        ToolTab::Technology => technology_tab(ui, &mut state),
        ToolTab::World => world_tab(ui, &mut state),
        ToolTab::Runtime => runtime_tab(ui, &mut state),
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
        "Active catalog: {} archetypes, {} buildings, {} roles, {} technologies, {} materials, {} material-bound prefabs, {} controllers, {} source records",
        state.catalog.archetypes.len(),
        state.catalog.buildings.len(),
        state.catalog.roles.len(),
        state.catalog.technology.nodes.len(),
        state.presentation.materials.len(),
        state.presentation.prefab_materials.len(),
        state.presentation.controllers.len(),
        state.catalog.source_records.len()
    ));
    ui.monospace(".\\bevy-port\\scripts\\export-unity.ps1");
    ui.monospace(".\\bevy-port\\scripts\\convert-models.ps1");
    ui.label(
        "Manifest stages: discovered -> referenced -> converted -> manually reviewed -> packaged",
    );
}

fn content_tab(ui: &mut egui::Ui, state: &ToolState) {
    let converted_clips = state
        .presentation
        .clips
        .values()
        .filter(|clip| !clip.transform_tracks.is_empty())
        .count();
    ui.heading("Content catalog and stable references");
    ui.label("Versioned RON uses stable IDs; Unity GUIDs remain in typed provenance records.");
    ui.horizontal(|ui| {
        ui.label(format!("Archetypes: {}", state.catalog.archetypes.len()));
        ui.separator();
        ui.label(format!("Buildings: {}", state.catalog.buildings.len()));
        ui.separator();
        ui.label(format!("Roles: {}", state.catalog.roles.len()));
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
            "Presentation: {} textures / {} materials / {} material-bound prefabs / {} clips ({} native transform) / {} controllers",
            state.presentation.textures.len(),
            state.presentation.materials.len(),
            state.presentation.prefab_materials.len(),
            state.presentation.clips.len(),
            converted_clips,
            state.presentation.controllers.len()
        ));
    });
    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.collapsing("Prefab archetypes", |ui| {
            for (id, archetype) in &state.catalog.archetypes {
                ui.collapsing(format!("{}  ({id})", archetype.display_name), |ui| {
                    ui.label(format!("Kind: {:?}", archetype.kind));
                    ui.monospace(format!("Unity prefab: {}", archetype.source_path));
                    ui.label(format!(
                        "Footprint: {} x {}; scene variants: {}",
                        archetype.footprint[0],
                        archetype.footprint[1],
                        archetype.scenes.len()
                    ));
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
                });
            }
        });
        ui.collapsing("Materials and texture bindings", |ui| {
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
                        material.custom_properties.len()
                    ));
                });
            }
        });
        ui.collapsing("Animation controllers", |ui| {
            for (id, controller) in &state.presentation.controllers {
                ui.collapsing(format!("{}  ({id})", controller.display_name), |ui| {
                    ui.monospace(format!("Unity controller: {}", controller.source_path));
                    ui.label(format!(
                        "{} parameters, {} states, {} transitions, {} layer defaults",
                        controller.parameters.len(),
                        controller.states.len(),
                        controller.transitions.len(),
                        controller.default_states.len()
                    ));
                    for state_def in controller.states.values() {
                        ui.label(format!(
                            "{} (speed {:.2}, {} motions)",
                            state_def.display_name,
                            state_def.speed,
                            state_def.motions.len()
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
                        "{:.3}s at {:.1} Hz, {} transform tracks, looping: {}",
                        clip.duration_seconds,
                        clip.sample_rate,
                        clip.transform_tracks.len(),
                        clip.looping
                    ));
                    if let Some(rig) = &clip.rig_asset_path {
                        ui.monospace(format!("Retarget rig: {rig}"));
                    }
                });
            }
        });
    });
}

fn technology_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Technology graph editor");
    ui.horizontal(|ui| {
        if ui
            .add_enabled(!state.undo_catalogs.is_empty(), egui::Button::new("Undo"))
            .clicked()
            && let Some(previous) = state.undo_catalogs.pop()
        {
            state.redo_catalogs.push(state.catalog.clone());
            state.catalog = previous;
            refresh_technology_draft(state);
            "Technology edit undone".clone_into(&mut state.status);
        }
        if ui
            .add_enabled(!state.redo_catalogs.is_empty(), egui::Button::new("Redo"))
            .clicked()
            && let Some(next) = state.redo_catalogs.pop()
        {
            state.undo_catalogs.push(state.catalog.clone());
            state.catalog = next;
            refresh_technology_draft(state);
            "Technology edit redone".clone_into(&mut state.status);
        }
        if ui.button("Validate graph").clicked() {
            state.status = match state.catalog.validate() {
                Ok(()) => format!(
                    "Technology graph valid: {} nodes in {} groups",
                    state.catalog.technology.nodes.len(),
                    state.catalog.technology.groups.len()
                ),
                Err(error) => format!("Technology graph error: {error}"),
            };
        }
        ui.label("Search");
        ui.text_edit_singleline(&mut state.technology_search);
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
    technology_minimap(
        ui,
        &state.catalog,
        state.selected_group.as_ref(),
        state.technology_draft.as_ref().map(|draft| &draft.id),
    );

    let search = state.technology_search.to_ascii_lowercase();
    let node_choices: Vec<_> = state
        .selected_group
        .as_ref()
        .and_then(|group| state.catalog.technology.groups.get(group))
        .map(|group| {
            group
                .nodes
                .iter()
                .filter_map(|id| {
                    let node = state.catalog.technology.nodes.get(id)?;
                    let matches = search.is_empty()
                        || node.display_name.to_ascii_lowercase().contains(&search)
                        || id.as_str().contains(&search);
                    matches.then(|| (id.clone(), node.display_name.clone()))
                })
                .collect()
        })
        .unwrap_or_default();
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
    }

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
        ui.label("Description");
        ui.text_edit_multiline(&mut draft.description);
        ui.label("Prerequisite stable IDs (comma separated)");
        ui.text_edit_singleline(&mut draft.prerequisites);
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
    ui.add(egui::DragValue::new(&mut state.config.world.seed).prefix("Seed "));
    ui.add(egui::Slider::new(&mut state.config.world.width, 8..=256).text("Width"));
    ui.add(egui::Slider::new(&mut state.config.world.height, 8..=256).text("Height"));
    if ui.button("Generate deterministic preview").clicked() {
        let world = generate_world(&state.config.world);
        state.status = format!(
            "Generated {}x{} world with {} resources; hash {}",
            world.navigation.width(),
            world.navigation.height(),
            world.resources.len(),
            &world.deterministic_hash[..16]
        );
        state.generated_world = Some(world);
        state.preview_path.clear();
    }
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut state.path_start.x).prefix("Start x "));
        ui.add(egui::DragValue::new(&mut state.path_start.z).prefix("z "));
        ui.add(egui::DragValue::new(&mut state.path_goal.x).prefix("Goal x "));
        ui.add(egui::DragValue::new(&mut state.path_goal.z).prefix("z "));
        if ui.button("Plan path").clicked() {
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
        ui.monospace(format!("Hash: {}", world.deterministic_hash));
        ui.label(format!("Resources: {}", world.resources.len()));
        ui.label(format!("Generator version: {}", world.generator_version));
        draw_world_preview(ui, world, &state.preview_path);
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
    ui.label("Save/load, spawn, vote/event injection, frame capture, and profiling will connect through the runtime bridge.");
}

fn twitch_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Twitch setup and diagnostics");
    ui.label("Public-client settings are separate from OAuth tokens. Tokens live only in the operating-system credential vault.");
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
        let busy = state.twitch_auth_events.is_some();
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
                egui::Button::new("Check vault"),
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
    ui.colored_label(
        egui::Color32::LIGHT_BLUE,
        "No credentials are stored in repository assets.",
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
                state.twitch_device = None;
                finished = true;
            }
            TwitchToolEvent::Diagnostic(validation) => {
                state.status = format!("Twitch token for '{}' is valid", validation.login);
                state.twitch_validation = Some(validation);
                finished = true;
            }
            TwitchToolEvent::Cleared => {
                "Removed the Twitch token from the OS credential vault"
                    .clone_into(&mut state.status);
                state.twitch_validation = None;
                state.twitch_device = None;
                finished = true;
            }
            TwitchToolEvent::Error(error) => {
                state.status = format!("Twitch setup failed: {error}");
                state.twitch_device = None;
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
    "Checking Twitch token in the OS credential vault...".clone_into(&mut state.status);
    let config = state.config.twitch.clone();
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-twitch-check".to_owned())
        .spawn(move || {
            let outcome = (|| -> anyhow::Result<TokenValidation> {
                let vault = CredentialVault::new(&config.client_id, &config.bot_login);
                let mut token = vault
                    .load()?
                    .ok_or_else(|| anyhow::anyhow!("no token is stored for this bot and client"))?;
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                runtime.block_on(async {
                    let oauth = OAuthClient::new(config.client_id.clone())?;
                    let validation = if let Ok(validation) = oauth.validate(&token).await {
                        validation
                    } else {
                        token = oauth.refresh(&token).await?;
                        vault.save(&token)?;
                        oauth.validate(&token).await?
                    };
                    anyhow::ensure!(
                        validation.login == config.bot_login,
                        "stored token belongs to '{}', expected '{}'",
                        validation.login,
                        config.bot_login
                    );
                    Ok(validation)
                })
            })();
            let event = outcome.map_or_else(
                |error| TwitchToolEvent::Error(format!("{error:#}")),
                TwitchToolEvent::Diagnostic,
            );
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not start Twitch diagnostic worker: {error}");
        state.twitch_auth_events = None;
    }
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
    config.validate()?;
    let directory = std::path::Path::new(".stream-town");
    std::fs::create_dir_all(directory)?;
    let path = directory.join("config.ron");
    let temporary = directory.join("config.ron.tmp");
    let backup = directory.join("config.ron.bak");
    let pretty = ron::ser::PrettyConfig::new().struct_names(true);
    std::fs::write(&temporary, ron::ser::to_string_pretty(config, pretty)?)?;
    if path.is_file() {
        if backup.is_file() {
            std::fs::remove_file(&backup)?;
        }
        std::fs::rename(&path, &backup)?;
    }
    if let Err(error) = std::fs::rename(&temporary, &path) {
        if backup.is_file() {
            let _ = std::fs::rename(&backup, &path);
        }
        return Err(error.into());
    }
    Ok(path)
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
    ui.label("Checks include stable IDs, dangling references, technology cycles, GLB hashes/headers, and deterministic baselines.");
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
        prerequisites: node
            .prerequisites
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        initially_unlocked: node.initially_unlocked,
        unavailable: node.unavailable,
    })
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
    node.prerequisites = prerequisites;
    node.initially_unlocked = draft.initially_unlocked;
    node.unavailable = draft.unavailable;
    candidate.validate().map_err(|error| error.to_string())?;
    state.undo_catalogs.push(state.catalog.clone());
    state.redo_catalogs.clear();
    state.catalog = candidate;
    refresh_technology_draft(state);
    Ok(())
}

fn technology_minimap(
    ui: &mut egui::Ui,
    catalog: &ContentCatalog,
    group_id: Option<&StableId>,
    selected: Option<&StableId>,
) {
    let Some(group) = group_id.and_then(|id| catalog.technology.groups.get(id)) else {
        return;
    };
    let desired = egui::vec2(ui.available_width(), 180.0);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    ui.painter()
        .rect_filled(rect, 4.0, egui::Color32::from_rgb(20, 28, 34));
    let mut tiers = BTreeMap::<i32, Vec<StableId>>::new();
    for id in &group.nodes {
        if let Some(node) = catalog.technology.nodes.get(id) {
            tiers.entry(node.tier).or_default().push(id.clone());
        }
    }
    let tier_count = bounded_ui_index(tiers.len().max(1));
    let mut positions = BTreeMap::new();
    for (tier_index, nodes) in tiers.values_mut().enumerate() {
        nodes.sort();
        let node_count = bounded_ui_index(nodes.len().max(1));
        for (row, id) in nodes.iter().enumerate() {
            let position = egui::pos2(
                rect.left() + (bounded_ui_index(tier_index) + 0.5) * rect.width() / tier_count,
                rect.top() + (bounded_ui_index(row) + 0.5) * rect.height() / node_count,
            );
            positions.insert(id.clone(), position);
        }
    }
    for id in &group.nodes {
        let Some(node) = catalog.technology.nodes.get(id) else {
            continue;
        };
        let Some(target) = positions.get(id) else {
            continue;
        };
        for prerequisite in &node.prerequisites {
            if let Some(source) = positions.get(prerequisite) {
                ui.painter().line_segment(
                    [*source, *target],
                    egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
                );
            }
        }
    }
    for (id, position) in positions {
        let color = if selected == Some(&id) {
            egui::Color32::YELLOW
        } else {
            egui::Color32::from_rgb(92, 180, 130)
        };
        ui.painter().circle_filled(position, 3.5, color);
    }
}

fn bounded_ui_index(value: usize) -> f32 {
    f32::from(u16::try_from(value).unwrap_or(u16::MAX))
}

fn draw_world_preview(ui: &mut egui::Ui, world: &GeneratedWorld, path: &[GridPos]) {
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
    let path: BTreeSet<_> = path.iter().copied().collect();
    let resources: BTreeSet<_> = world.resources.iter().map(|item| item.position).collect();
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
            let color = if path.contains(&position) {
                egui::Color32::YELLOW
            } else if resources.contains(&position) {
                egui::Color32::from_rgb(205, 150, 55)
            } else if world.navigation.is_walkable(position) {
                egui::Color32::from_rgb(55, 115, 65)
            } else {
                egui::Color32::from_rgb(35, 70, 105)
            };
            ui.painter().rect_filled(
                egui::Rect::from_min_max(egui::pos2(left, top), egui::pos2(right, bottom)),
                0.0,
                color,
            );
        }
    }
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
}
