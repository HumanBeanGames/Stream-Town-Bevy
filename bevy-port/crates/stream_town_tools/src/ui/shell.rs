fn tools_ui(
    mut contexts: EguiContexts,
    mut state: ResMut<ToolState>,
    mut preview: ResMut<ModelPreviewRuntime>,
    tidal: Res<TidalController>,
    tidal_backend: Res<TidalBackendStatus>,
    native_audio: Res<NativeAudioStatus>,
    audio_routing: Res<NativeAudioRouting>,
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
        ToolTab::Balance => progression_tab(ui, &mut state),
        ToolTab::Technology => technology_tab(ui, &mut state),
        ToolTab::Terrain => terrain_tab(ui, &mut state),
        ToolTab::Music => music_tab(
            ui,
            &mut state,
            &tidal,
            &tidal_backend,
            &native_audio,
            &audio_routing,
        ),
        ToolTab::World => world_tab(
            ui,
            &mut state,
            preview_texture,
            &preview_status,
            &mut preview.controls,
        ),
        ToolTab::Settings => settings_tab(ui, &mut state),
        ToolTab::Runtime => runtime_tab(ui, &mut state),
        ToolTab::Twitch => twitch_tab(ui, &mut state),
        ToolTab::Validation => validation_tab(ui, &mut state),
    });
    Ok(())
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
