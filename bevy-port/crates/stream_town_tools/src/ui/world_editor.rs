fn world_tab(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    ui.heading("World-generation lab");
    ui.label(
        "Tune authored terrain and foliage generation, then inspect deterministic elevation, occupancy, resource, and foliage previews.",
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
                ui.label(format!("Source provenance: {}", layer.source_path));
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
                "Source provenance",
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

fn settings_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Player settings");
    ui.label("Native player settings with validated, atomic RON persistence.");
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
        if ui.button("Restore defaults").clicked() {
            state.player_settings = PlayerSettings::default();
            "Restored default settings; save to persist them".clone_into(&mut state.status);
        }
    });
    ui.label("Restart the game after saving to apply window, renderer, and audio changes.");
}
