fn progression_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Global progression balance");
    ui.label(
        "These catalog values are the runtime authority for role caps, the XP curve, level announcements, and the minimum action cadence.",
    );
    egui::Grid::new("global_progression_balance")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            ui.label("Maximum player role level");
            ui.add(
                egui::DragValue::new(&mut state.catalog.progression.maximum_role_level)
                    .range(1..=u16::MAX),
            );
            ui.end_row();
            ui.label("Maximum recruit role level");
            ui.add(
                egui::DragValue::new(&mut state.catalog.progression.maximum_recruit_role_level)
                    .range(1..=state.catalog.progression.maximum_role_level),
            );
            ui.end_row();
            ui.label("XP curve plateau level");
            ui.add(
                egui::DragValue::new(
                    &mut state.catalog.progression.role_experience_curve_level_span,
                )
                .range(2..=u16::MAX),
            );
            ui.end_row();
            ui.label("XP required at plateau");
            ui.add(
                egui::DragValue::new(&mut state.catalog.progression.role_experience_curve_maximum)
                    .range(1..=u32::MAX),
            );
            ui.end_row();
            ui.label("Chat announcement interval");
            ui.add(
                egui::DragValue::new(&mut state.catalog.progression.level_announcement_interval)
                    .range(1..=u16::MAX),
            );
            ui.end_row();
            ui.label("Minimum action cadence (ms)");
            ui.add(
                egui::DragValue::new(&mut state.catalog.progression.minimum_action_milliseconds)
                    .range(1..=u32::MAX),
            );
            ui.end_row();
        });
    ui.small(
        "The curved XP formula is unchanged: it rises to the authored plateau, then remains flat through the authored maximum role level.",
    );
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("Validate and save progression").clicked() {
            state.status = match save_content_catalog(&state.catalog, &state.catalog_path) {
                Ok(path) => format!("Saved progression balance to {}", path.display()),
                Err(error) => format!("Progression save failed: {error:#}"),
            };
        }
        ui.label("Catalog path");
        ui.text_edit_singleline(&mut state.catalog_path);
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
                    "Source provenance",
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
