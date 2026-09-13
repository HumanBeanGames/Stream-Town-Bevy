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
        cached_gltf_metadata(state, "shipping/models/Models/Characters/Characters.glb");
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
                            "Base health bonus/level (/1000)",
                            &mut draft.value.health_bonus_per_level_per_thousand,
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
