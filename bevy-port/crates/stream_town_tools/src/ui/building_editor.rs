fn buildings_tab(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    ui.heading("Building authoring");
    ui.label(
        "Create a building from a complete shipping template, choose its model archetype visually, and edit gameplay references without typing stable IDs.",
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
                Ok(path) => format!("Saved validated building catalog to {}", path.display()),
                Err(error) => format!("Could not save building catalog: {error:#}"),
            };
        }
    });
    let building_choices: Vec<_> = state
        .catalog
        .buildings
        .iter()
        .map(|(id, building)| (id.clone(), building.display_name.clone()))
        .collect();
    let mut changed = false;
    egui::ComboBox::from_label("Building")
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(
            state
                .selected_building
                .as_ref()
                .and_then(|id| state.catalog.buildings.get(id))
                .map_or("Select building", |building| building.display_name.as_str()),
        )
        .show_ui(ui, |ui| {
            for (id, name) in &building_choices {
                changed |= ui
                    .selectable_value(&mut state.selected_building, Some(id.clone()), name)
                    .changed();
            }
        });
    if changed {
        refresh_building_draft(state);
    }
    ui.horizontal_wrapped(|ui| {
        ui.label("New stable ID");
        ui.text_edit_singleline(&mut state.new_building_id);
        ui.label("Name");
        ui.text_edit_singleline(&mut state.new_building_name);
        if ui
            .add_enabled(
                state.selected_building.is_some(),
                egui::Button::new("Create from selected"),
            )
            .on_hover_text("Copies every model, balance, station, and presentation field")
            .clicked()
        {
            state.status = match duplicate_selected_building(state) {
                Ok(()) => "Created a complete building draft from the selected template".to_owned(),
                Err(error) => format!("Building creation rejected: {error}"),
            };
        }
        if ui
            .add_enabled(
                state.selected_building.is_some(),
                egui::Button::new("Delete"),
            )
            .clicked()
        {
            state.status = match delete_selected_building(state) {
                Ok(()) => "Deleted unreferenced building".to_owned(),
                Err(error) => format!("Building deletion rejected: {error}"),
            };
        }
    });

    let archetype_choices: Vec<_> = state
        .catalog
        .archetypes
        .iter()
        .filter(|(_, archetype)| archetype.kind == ArchetypeKind::Building)
        .map(|(id, archetype)| (id.clone(), archetype.display_name.clone()))
        .collect();
    let resources = resource_choices(&state.catalog);
    let roles: Vec<_> = state
        .catalog
        .roles
        .iter()
        .map(|(id, role)| (id.clone(), role.display_name.clone()))
        .collect();
    let station_kinds = station_kind_choices(&state.catalog);
    let target_kinds = target_kind_choices(&state.catalog);
    let building_asset_path = state
        .building_draft
        .as_ref()
        .and_then(|draft| state.catalog.archetypes.get(&draft.value.archetype))
        .and_then(|archetype| {
            archetype
                .scenes
                .iter()
                .find(|scene| scene.is_default)
                .or_else(|| archetype.scenes.first())
        })
        .map(|scene| scene.asset_path.clone());
    let building_metadata = building_asset_path
        .as_deref()
        .map(|path| cached_gltf_metadata(state, path))
        .unwrap_or_default();
    let mut model_nodes = building_model_node_choices(&state.catalog);
    model_nodes.extend(building_metadata.nodes);
    model_nodes.sort();
    model_nodes.dedup();
    let projectile_pools = projectile_pool_choices(&state.catalog);
    let mut apply = false;
    let mut reset = false;
    if let Some(draft) = state.building_draft.as_mut() {
        ui.separator();
        ui.monospace(draft.id.to_string());
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.columns(2, |columns| {
                columns[0].label("Display name");
                columns[0].text_edit_singleline(&mut draft.value.display_name);
                if stable_id_required_choice(
                    &mut columns[0],
                    "Model archetype",
                    &mut draft.value.archetype,
                    &archetype_choices,
                ) && let Some(archetype) = state.catalog.archetypes.get(&draft.value.archetype)
                {
                    draft.value.footprint = archetype.footprint;
                }
                footprint_editor(
                    &mut columns[0],
                    &mut draft.value.footprint,
                    "Model anchor footprint",
                    64,
                );
                let mut custom_placement = draft.value.placement_footprint_thirds.is_some();
                if columns[0]
                    .checkbox(&mut custom_placement, "Override exact placement footprint")
                    .changed()
                {
                    draft.value.placement_footprint_thirds = custom_placement.then(|| {
                        draft
                            .value
                            .footprint
                            .map(|axis| axis.saturating_mul(3))
                    });
                }
                if let Some(placement) = draft.value.placement_footprint_thirds.as_mut() {
                    columns[0].label("Placement/exclusion box (third-cell units)");
                    columns[0].horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut placement[0])
                                .range(1..=192)
                                .prefix("Width "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut placement[1])
                                .range(1..=192)
                                .prefix("Depth "),
                        );
                    });
                    placement[0] = placement[0].max(1);
                    placement[1] = placement[1].max(1);
                }
                let mut custom_navigation = draft.value.navigation_footprint_thirds.is_some();
                if columns[0]
                    .checkbox(
                        &mut custom_navigation,
                        "Override physical navigation footprint",
                    )
                    .changed()
                {
                    draft.value.navigation_footprint_thirds = custom_navigation.then(|| {
                        draft.value.placement_footprint_thirds.map_or_else(
                            || default_navigation_footprint_thirds(draft.value.footprint),
                            |placement| placement.map(|axis| axis.saturating_sub(2).max(1)),
                        )
                    });
                }
                if let Some(navigation) = draft.value.navigation_footprint_thirds.as_mut() {
                    let maximum = draft.value.placement_footprint_thirds.unwrap_or_else(|| {
                        draft.value.footprint.map(|axis| axis.saturating_mul(3))
                    });
                    let minimum = u16::from(draft.id.as_str() != "building:path");
                    columns[0].label("Physical footprint (third-cell navigation units)");
                    columns[0].horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut navigation[0])
                                .range(minimum..=maximum[0])
                                .prefix("Width "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut navigation[1])
                                .range(minimum..=maximum[1])
                                .prefix("Depth "),
                        );
                    });
                    navigation[0] = navigation[0].clamp(minimum, maximum[0]);
                    navigation[1] = navigation[1].clamp(minimum, maximum[1]);
                    columns[0].small(format!(
                        "Occupies {:.2} × {:.2} authored cells, centered inside the exact placement box.",
                        f32::from(navigation[0]) / 3.0,
                        f32::from(navigation[1]) / 3.0,
                    ));
                } else {
                    let inferred = default_navigation_footprint_thirds(draft.value.footprint);
                    columns[0].small(format!(
                        "Default physical footprint: {} × {} third-cells (one unit inset per side).",
                        inferred[0], inferred[1],
                    ));
                }
                columns[0].small(
                    "The model footprint anchors the scene. Exact placement and physical navigation are independently authored on the shared fine grid.",
                );
                columns[0].horizontal_wrapped(|ui| {
                    ui.checkbox(&mut draft.value.placeable, "Placeable");
                    ui.checkbox(&mut draft.value.can_level, "Can level");
                });
                columns[0].add(
                    egui::DragValue::new(
                        &mut draft.value.construction_cost_multiplier_per_thousand,
                    )
                    .range(0..=100_000)
                    .prefix("Existing-building cost multiplier /1000 "),
                );
                columns[0].small("Set this to 0 for a fixed construction price.");
                columns[0].add(
                    egui::DragValue::new(
                        &mut draft.value.upgrade_cost_per_target_level_per_thousand,
                    )
                        .range(1..=100_000)
                        .prefix("Base cost fraction per target level /1000 "),
                );
                draw_building_visual(
                    &mut columns[1],
                    &draft.value,
                    &state.catalog,
                    preview_texture,
                    preview_status,
                    preview_controls,
                );
            });
            ui.collapsing("Construction and upgrade costs", |ui| {
                stable_u32_map_editor(ui, "Construction cost", &mut draft.value.cost, &resources);
                ui.small(
                    "Upgrade cost is derived from the construction cost above × the authored fraction × (target level - 1). Multi-level purchases sum each target level.",
                );
            });
            ui.collapsing("Per-level building effects", |ui| {
                ui.add(
                    egui::DragValue::new(
                        &mut draft.value.health_bonus_per_level_per_thousand,
                    )
                    .range(0..=100_000)
                    .prefix("Base health bonus /1000 "),
                );
                ui.add(
                    egui::DragValue::new(
                        &mut draft
                            .value
                            .global_gather_rate_bonus_per_level_per_thousand,
                    )
                    .range(0..=100_000)
                    .prefix("Global gather-rate bonus /1000 "),
                );
                ui.add(
                    egui::DragValue::new(
                        &mut draft.value.kill_experience_bonus_per_level_per_thousand,
                    )
                    .range(0..=100_000)
                    .prefix("Kill XP bonus /1000 "),
                );
            });
            ui.collapsing("Storage", |ui| {
                let mut remove = None;
                for (index, storage) in draft.value.storage.iter_mut().enumerate() {
                    ui.horizontal_wrapped(|ui| {
                        stable_id_required_choice(
                            ui,
                            "Resource",
                            &mut storage.resource,
                            &resources,
                        );
                        ui.add(egui::DragValue::new(&mut storage.base_amount).prefix("Base "));
                        ui.add(
                            egui::DragValue::new(&mut storage.increment_amount)
                                .prefix("Per level "),
                        );
                        if ui.small_button("Remove").clicked() {
                            remove = Some(index);
                        }
                    });
                }
                if let Some(index) = remove {
                    draft.value.storage.remove(index);
                }
                if ui.button("Add storage contribution").clicked()
                    && let Some((resource, _)) = resources.first()
                {
                    draft.value.storage.push(StorageContribution {
                        resource: resource.clone(),
                        base_amount: 100,
                        increment_amount: 0,
                    });
                }
            });
            ui.collapsing("Role capacity", |ui| {
                let mut remove = None;
                for (index, slot) in draft.value.role_slots.iter_mut().enumerate() {
                    ui.horizontal_wrapped(|ui| {
                        stable_id_required_choice(ui, "Role", &mut slot.role, &roles);
                        ui.add(egui::DragValue::new(&mut slot.base_amount).prefix("Base slots "));
                        ui.add(
                            egui::DragValue::new(&mut slot.increment_amount).prefix("Per level "),
                        );
                        if ui.small_button("Remove").clicked() {
                            remove = Some(index);
                        }
                    });
                }
                if let Some(index) = remove {
                    draft.value.role_slots.remove(index);
                }
                if ui.button("Add role capacity").clicked()
                    && let Some((role, _)) = roles.first()
                {
                    draft.value.role_slots.push(RoleSlotContribution {
                        role: role.clone(),
                        base_amount: 1,
                        increment_amount: 0,
                    });
                }
            });
            ui.collapsing("Passive production", |ui| {
                let mut remove = None;
                for (index, income) in draft.value.passive_resources.iter_mut().enumerate() {
                    ui.horizontal_wrapped(|ui| {
                        stable_id_required_choice(ui, "Resource", &mut income.resource, &resources);
                        ui.add(
                            egui::DragValue::new(&mut income.base_milli_per_second)
                                .prefix("Base milli/s "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut income.increment_milli_per_level)
                                .prefix("Per level "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut income.level_event_repetitions)
                                .prefix("Callbacks "),
                        );
                        if ui.small_button("Remove").clicked() {
                            remove = Some(index);
                        }
                    });
                }
                if let Some(index) = remove {
                    draft.value.passive_resources.remove(index);
                }
                if ui.button("Add passive resource").clicked()
                    && let Some((resource, _)) = resources.first()
                {
                    draft
                        .value
                        .passive_resources
                        .push(PassiveResourceContribution {
                            resource: resource.clone(),
                            base_milli_per_second: 1_000,
                            increment_milli_per_level: 0,
                            level_event_repetitions: 1,
                        });
                }
            });
            ui.collapsing("Station and target selection", |ui| {
                let mut enabled = draft.value.station.is_some();
                if ui
                    .checkbox(&mut enabled, "Acts as a work station")
                    .changed()
                {
                    draft.value.station = enabled.then(|| StationDef {
                        accepts_all_roles: false,
                        accepted_role_kinds: BTreeSet::new(),
                        targets_all: false,
                        target_kinds: BTreeSet::new(),
                        max_targets: 1,
                        update_milliseconds: 1_000,
                        search_range_milli_cells: 10_000,
                        experience_bonus_per_level_per_thousand: 0,
                    });
                }
                if let Some(station) = draft.value.station.as_mut() {
                    ui.checkbox(&mut station.accepts_all_roles, "Accept all roles");
                    stable_id_set_choices(
                        ui,
                        "Accepted role kinds",
                        &mut station.accepted_role_kinds,
                        &station_kinds,
                    );
                    ui.checkbox(&mut station.targets_all, "Target every kind");
                    stable_id_set_choices(
                        ui,
                        "Target kinds",
                        &mut station.target_kinds,
                        &target_kinds,
                    );
                    ui.horizontal_wrapped(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut station.max_targets)
                                .prefix("Maximum targets "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut station.update_milliseconds)
                                .prefix("Refresh ms "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut station.search_range_milli_cells)
                                .prefix("Range milli-cells "),
                        );
                        ui.add(
                            egui::DragValue::new(
                                &mut station.experience_bonus_per_level_per_thousand,
                            )
                            .range(0..=100_000)
                            .prefix("Role XP/level /1000 "),
                        );
                    });
                }
                let mut targeting = draft.value.targeting.is_some();
                if ui.checkbox(&mut targeting, "Uses target scoring").changed() {
                    draft.value.targeting = targeting.then_some(TargetingScoreDef {
                        assignment_penalty_milli: 1_000,
                        distance_penalty_milli_per_cell: 1_000,
                    });
                }
                if let Some(targeting) = draft.value.targeting.as_mut() {
                    ui.add(
                        egui::DragValue::new(&mut targeting.assignment_penalty_milli)
                            .prefix("Assignment penalty "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut targeting.distance_penalty_milli_per_cell)
                            .prefix("Distance penalty/cell "),
                    );
                }
            });
            ui.collapsing("Projectile attack", |ui| {
                let mut enabled = draft.value.projectile_shooter.is_some();
                if ui.checkbox(&mut enabled, "Shoots projectiles").changed() {
                    let pool = projectile_pools
                        .first()
                        .cloned()
                        .unwrap_or_else(|| "projectile:default".to_owned());
                    draft.value.projectile_shooter = enabled.then_some(ProjectileShooterDef {
                        projectile_pool: pool,
                        movement_milli_cells_per_second: 10_000,
                        damage: 1,
                        damage_bonus_per_level_per_thousand: 0,
                        range_milli_cells: 10_000,
                        fire_milliseconds: 1_000,
                    });
                }
                if let Some(shooter) = draft.value.projectile_shooter.as_mut() {
                    string_choice(
                        ui,
                        "Projectile pool",
                        &mut shooter.projectile_pool,
                        &projectile_pools,
                    );
                    ui.horizontal_wrapped(|ui| {
                        ui.add(egui::DragValue::new(&mut shooter.damage).prefix("Damage "));
                        ui.add(
                            egui::DragValue::new(
                                &mut shooter.damage_bonus_per_level_per_thousand,
                            )
                            .range(0..=100_000)
                            .prefix("Damage/level /1000 "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut shooter.fire_milliseconds)
                                .prefix("Cadence ms "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut shooter.range_milli_cells)
                                .prefix("Range milli-cells "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut shooter.movement_milli_cells_per_second)
                                .prefix("Projectile speed "),
                        );
                    });
                }
            });
            ui.collapsing("Model hierarchy bindings", |ui| {
                let mut remove_handler = None;
                for (index, model) in draft.value.model_handlers.iter_mut().enumerate() {
                    ui.push_id(("building_model_handler", index), |ui| {
                        ui.group(|ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.strong(format!("Model handler {}", index + 1));
                                ui.add(egui::DragValue::new(&mut model.age).prefix("Age "));
                                if ui.small_button("Remove handler").clicked() {
                                    remove_handler = Some(index);
                                }
                            });
                            string_choice(
                                ui,
                                "Complete model",
                                &mut model.full_model,
                                &model_nodes,
                            );
                            for (stage, node) in model.construction_stages.iter_mut().enumerate() {
                                ui.push_id(("construction_stage", stage), |ui| {
                                    string_choice(
                                        ui,
                                        &format!("Construction stage {}", stage + 1),
                                        node,
                                        &model_nodes,
                                    );
                                });
                            }
                            string_vec_choices(
                                ui,
                                "Upgrade layers",
                                &mut model.upgrades,
                                &model_nodes,
                            );
                            string_vec_choices(
                                ui,
                                "Other controlled models",
                                &mut model.other_models,
                                &model_nodes,
                            );
                        })
                    });
                }
                if let Some(index) = remove_handler {
                    draft.value.model_handlers.remove(index);
                }
                if ui.button("Add model handler").clicked()
                    && let Some(first) = model_nodes.first()
                {
                    draft.value.model_handlers.push(BuildingModelDef {
                        age: 1,
                        full_model: first.clone(),
                        construction_stages: [first.clone(), first.clone(), first.clone()],
                        upgrades: Vec::new(),
                        other_models: Vec::new(),
                    });
                }
                ui.separator();
                let mut remove_storage_model = None;
                for (index, model) in draft.value.storage_models.iter_mut().enumerate() {
                    ui.push_id(("building_storage_model", index), |ui| {
                        ui.group(|ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.strong(format!("Storage model {}", index + 1));
                                ui.add(egui::DragValue::new(&mut model.age).prefix("Age "));
                                if ui.small_button("Remove storage model").clicked() {
                                    remove_storage_model = Some(index);
                                }
                            });
                            stable_id_required_choice(
                                ui,
                                "Resource",
                                &mut model.resource,
                                &resources,
                            );
                            string_choice(ui, "Empty", &mut model.empty_model, &model_nodes);
                            string_choice(
                                ui,
                                "Half full",
                                &mut model.half_full_model,
                                &model_nodes,
                            );
                            string_choice(ui, "Full", &mut model.full_model, &model_nodes);
                        })
                    });
                }
                if let Some(index) = remove_storage_model {
                    draft.value.storage_models.remove(index);
                }
                if ui.button("Add storage model").clicked()
                    && let (Some(first), Some((resource, _))) =
                        (model_nodes.first(), resources.first())
                {
                    draft.value.storage_models.push(StorageModelDef {
                        age: 1,
                        resource: resource.clone(),
                        empty_model: first.clone(),
                        half_full_model: first.clone(),
                        full_model: first.clone(),
                    });
                }
            });
            ui.horizontal(|ui| {
                apply = ui.button("Apply validated building").clicked();
                reset = ui.button("Discard draft").clicked();
            });
        });
    } else {
        ui.label("Select a building to edit it.");
    }
    if apply {
        state.status = match apply_building_draft(state) {
            Ok(()) => "Building edit applied; every reference remains valid".to_owned(),
            Err(error) => format!("Building edit rejected: {error}"),
        };
    } else if reset {
        refresh_building_draft(state);
        "Discarded building draft".clone_into(&mut state.status);
    }
}
