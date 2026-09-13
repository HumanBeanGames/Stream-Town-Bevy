fn draw_building_visual(
    ui: &mut egui::Ui,
    building: &BuildingDef,
    catalog: &ContentCatalog,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    ui.group(|ui| {
        let archetype = catalog.archetypes.get(&building.archetype);
        ui.heading(archetype.map_or("Missing archetype", |value| value.display_name.as_str()));
        if let Some(archetype) = archetype {
            for scene in &archetype.scenes {
                ui.monospace(format!(
                    "{}{}",
                    if scene.is_default { "● " } else { "○ " },
                    scene.asset_path
                ));
            }
        }
        draw_model_preview(
            ui,
            preview_texture,
            preview_status,
            egui::vec2(320.0, 214.0),
            preview_controls,
            false,
        );
        ui.small(format!(
            "Model anchor: {} × {} cells · exact placement: {} × {} third-cells",
            building.footprint[0],
            building.footprint[1],
            building
                .placement_footprint_thirds
                .unwrap_or_else(|| building.footprint.map(|axis| axis.saturating_mul(3)))[0],
            building
                .placement_footprint_thirds
                .unwrap_or_else(|| building.footprint.map(|axis| axis.saturating_mul(3)))[1],
        ));
        draw_footprint_grid(
            ui,
            building.footprint,
            building.placement_footprint_thirds,
            building.navigation_footprint_thirds,
            egui::vec2(320.0, 160.0),
        );
    });
}

fn footprint_editor(
    ui: &mut egui::Ui,
    footprint: &mut [u16; 2],
    label: &str,
    maximum: u16,
) -> bool {
    let previous = *footprint;
    ui.group(|ui| {
        ui.horizontal_wrapped(|ui| {
            ui.strong(label);
            ui.add(
                egui::DragValue::new(&mut footprint[0])
                    .range(1..=maximum)
                    .prefix("Width "),
            );
            ui.add(
                egui::DragValue::new(&mut footprint[1])
                    .range(1..=maximum)
                    .prefix("Depth "),
            );
            if ui.small_button("− column").clicked() {
                footprint[0] = footprint[0].saturating_sub(1).max(1);
            }
            if ui.small_button("+ column").clicked() {
                footprint[0] = footprint[0].saturating_add(1).min(maximum);
            }
            if ui.small_button("− row").clicked() {
                footprint[1] = footprint[1].saturating_sub(1).max(1);
            }
            if ui.small_button("+ row").clicked() {
                footprint[1] = footprint[1].saturating_add(1).min(maximum);
            }
            if ui.small_button("Rotate 90°").clicked() {
                footprint.swap(0, 1);
            }
        });
        draw_footprint_grid(ui, *footprint, None, None, egui::vec2(300.0, 170.0));
        ui.small(format!(
            "{} logical cell{} / {} fine cells · origin is the highlighted top-left logical cell",
            u32::from(footprint[0]) * u32::from(footprint[1]),
            if *footprint == [1, 1] { "" } else { "s" },
            u32::from(footprint[0]) * u32::from(footprint[1]) * 9,
        ));
    });
    *footprint != previous
}

fn default_navigation_footprint_thirds(placement: [u16; 2]) -> [u16; 2] {
    placement.map(|axis| axis.saturating_mul(3).saturating_sub(2).max(1))
}

fn draw_footprint_grid(
    ui: &mut egui::Ui,
    footprint: [u16; 2],
    placement_footprint_thirds: Option<[u16; 2]>,
    navigation_footprint_thirds: Option<[u16; 2]>,
    desired: egui::Vec2,
) {
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    ui.painter()
        .rect_filled(rect, 5.0, egui::Color32::from_rgb(18, 27, 34));
    let fine = placement_footprint_thirds
        .unwrap_or_else(|| footprint.map(|axis| axis.max(1).saturating_mul(3)));
    let width = f32::from(fine[0]);
    let depth = f32::from(fine[1]);
    let scale = (rect.width() / width).min(rect.height() / depth) * 0.86;
    let grid =
        egui::Rect::from_center_size(rect.center(), egui::vec2(width * scale, depth * scale));
    ui.painter()
        .rect_filled(grid, 3.0, egui::Color32::from_rgb(71, 120, 145));
    let navigation = navigation_footprint_thirds.unwrap_or_else(|| {
        placement_footprint_thirds.map_or_else(
            || default_navigation_footprint_thirds(footprint),
            |placement| placement.map(|axis| axis.saturating_sub(2).max(1)),
        )
    });
    let physical_size = egui::vec2(
        f32::from(navigation[0].min(fine[0])) * scale,
        f32::from(navigation[1].min(fine[1])) * scale,
    );
    let physical = egui::Rect::from_center_size(grid.center(), physical_size);
    ui.painter().rect_filled(
        physical,
        2.0,
        egui::Color32::from_rgba_premultiplied(231, 151, 48, 105),
    );
    let minor = egui::Stroke::new(0.45, egui::Color32::from_rgb(103, 157, 181));
    let major = egui::Stroke::new(1.2, egui::Color32::from_rgb(181, 218, 232));
    if fine[0] <= 192 {
        for x in 1..fine[0] {
            let is_major = x % 3 == 0;
            let x = grid.left() + f32::from(x) * scale;
            ui.painter().line_segment(
                [egui::pos2(x, grid.top()), egui::pos2(x, grid.bottom())],
                if is_major { major } else { minor },
            );
        }
    }
    if fine[1] <= 192 {
        for z in 1..fine[1] {
            let y = grid.top() + f32::from(z) * scale;
            ui.painter().line_segment(
                [egui::pos2(grid.left(), y), egui::pos2(grid.right(), y)],
                if z % 3 == 0 { major } else { minor },
            );
        }
    }
    let origin = egui::Rect::from_min_size(grid.min, egui::Vec2::splat(scale * 3.0));
    ui.painter().rect_filled(
        origin.shrink(1.0),
        2.0,
        egui::Color32::from_rgb(218, 167, 67),
    );
    ui.painter().rect_stroke(
        grid,
        3.0,
        egui::Stroke::new(1.5, egui::Color32::from_rgb(183, 218, 232)),
        egui::StrokeKind::Inside,
    );
}

fn free_string_vec_editor(ui: &mut egui::Ui, label: &str, values: &mut Vec<String>) {
    ui.collapsing(format!("{label} ({})", values.len()), |ui| {
        let mut remove = None;
        for (index, value) in values.iter_mut().enumerate() {
            ui.horizontal_wrapped(|ui| {
                ui.text_edit_singleline(value);
                if ui.small_button("Remove").clicked() {
                    remove = Some(index);
                }
            });
        }
        if let Some(index) = remove {
            values.remove(index);
        }
        if ui.button(format!("Add {label}")).clicked() {
            values.push(String::new());
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn archetype_runtime_editor(
    ui: &mut egui::Ui,
    archetype: &mut ArchetypeDef,
    resources: &[(StableId, String)],
    target_kinds: &[(StableId, String)],
    enemy_archetypes: &[(StableId, String)],
    enemy_types: &[(StableId, String)],
    enemy_pools: &[(StableId, String)],
    model_nodes: &[String],
    animation_names: &[String],
) {
    ui.collapsing("Runtime behavior", |ui| {
        optional_u32_editor(
            ui,
            "Disable after",
            &mut archetype.disable_after_milliseconds,
            1_000,
            " ms",
        );
        optional_u32_editor(
            ui,
            "Health bar hide delay",
            &mut archetype.health_bar_hide_milliseconds,
            1_000,
            " ms",
        );
        ui.add(
            egui::DragValue::new(&mut archetype.target_size_milli_cells)
                .range(0..=u32::MAX)
                .prefix("Target size ")
                .suffix(" milli-cells"),
        );
        rotating_nodes_editor(ui, &mut archetype.rotating_nodes, model_nodes);
        health_definition_editor(ui, &mut archetype.health);
        enemy_definition_editor(
            ui,
            &mut archetype.enemy,
            resources,
            target_kinds,
            enemy_types,
            enemy_pools,
        );
        enemy_model_set_editor(
            ui,
            &mut archetype.enemy_models,
            model_nodes,
            animation_names,
        );
        enemy_spawner_editor(ui, &mut archetype.enemy_spawner, enemy_archetypes);
        pet_definition_editor(ui, &mut archetype.pet, model_nodes);
    });
}

fn optional_u32_editor(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<u32>,
    default_value: u32,
    suffix: &str,
) {
    ui.horizontal_wrapped(|ui| {
        let mut enabled = value.is_some();
        if ui.checkbox(&mut enabled, label).changed() {
            *value = enabled.then_some(default_value);
        }
        if let Some(value) = value {
            ui.add(egui::DragValue::new(value).suffix(suffix));
        }
    });
}

fn rotating_nodes_editor(
    ui: &mut egui::Ui,
    values: &mut Vec<RotatingNodeDef>,
    model_nodes: &[String],
) {
    ui.collapsing(format!("Rotating nodes ({})", values.len()), |ui| {
        let mut remove = None;
        for (index, value) in values.iter_mut().enumerate() {
            ui.push_id(("rotating_node", index), |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Hierarchy path");
                        ui.text_edit_singleline(&mut value.hierarchy_path);
                        if model_nodes.is_empty() {
                            ui.label("Node");
                            ui.text_edit_singleline(&mut value.node);
                        } else {
                            string_choice(ui, "Node", &mut value.node, model_nodes);
                        }
                        if ui.small_button("Remove").clicked() {
                            remove = Some(index);
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        for (component, axis) in value.axis.iter_mut().zip(["Axis X", "Y", "Z"]) {
                            ui.add(egui::DragValue::new(component).prefix(format!("{axis} ")));
                        }
                        ui.add(
                            egui::DragValue::new(&mut value.degrees_per_second)
                                .suffix(" degrees/s"),
                        );
                        let mut aged = value.age.is_some();
                        if ui.checkbox(&mut aged, "Age-specific").changed() {
                            value.age = aged.then_some(1);
                        }
                        if let Some(age) = value.age.as_mut() {
                            ui.add(egui::DragValue::new(age).range(1..=u8::MAX));
                        }
                    });
                });
            });
        }
        if let Some(index) = remove {
            values.remove(index);
        }
        if ui.button("Add rotating node").clicked() {
            values.push(RotatingNodeDef {
                hierarchy_path: String::new(),
                age: None,
                node: model_nodes.first().cloned().unwrap_or_default(),
                axis: [0.0, 1.0, 0.0],
                degrees_per_second: 30.0,
            });
        }
    });
}

fn health_definition_editor(ui: &mut egui::Ui, value: &mut Option<HealthDef>) {
    ui.collapsing("Health and revival", |ui| {
        let mut enabled = value.is_some();
        if ui.checkbox(&mut enabled, "Has health").changed() {
            *value = enabled.then_some(HealthDef {
                max_health: 100,
                health_gain_per_level: 0,
                regeneration_milli_per_second: 0,
                regeneration_requires_food: false,
                revive_milliseconds: None,
            });
        }
        if let Some(value) = value {
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::DragValue::new(&mut value.max_health).prefix("Maximum "));
                ui.add(egui::DragValue::new(&mut value.health_gain_per_level).prefix("Per level "));
                ui.add(
                    egui::DragValue::new(&mut value.regeneration_milli_per_second)
                        .prefix("Regeneration milli/s "),
                );
                ui.checkbox(&mut value.regeneration_requires_food, "Requires food");
            });
            optional_u32_editor(
                ui,
                "Revives after",
                &mut value.revive_milliseconds,
                5_000,
                " ms",
            );
        }
    });
}

fn enemy_definition_editor(
    ui: &mut egui::Ui,
    value: &mut Option<EnemyDef>,
    resources: &[(StableId, String)],
    target_kinds: &[(StableId, String)],
    enemy_types: &[(StableId, String)],
    enemy_pools: &[(StableId, String)],
) {
    ui.collapsing("Enemy combat behavior", |ui| {
        let mut enabled = value.is_some();
        if ui.checkbox(&mut enabled, "Is an enemy").changed() {
            *value = enabled.then(|| EnemyDef {
                enemy_type: enemy_types.first().map_or_else(
                    || StableId::new("enemy_type:new").unwrap(),
                    |value| value.0.clone(),
                ),
                pool: enemy_pools.first().map_or_else(
                    || StableId::new("pool:enemy_new").unwrap(),
                    |value| value.0.clone(),
                ),
                additional_health_milli_per_player: 0,
                action_amount: 5,
                action_milliseconds: 1_000,
                action_range_milli_cells: 1_000,
                target_search_range_milli_cells: 10_000,
                attack_attacker: true,
                kill_reward: ResourceReward {
                    resource: resources.first().map_or_else(
                        || StableId::new("resource:food").unwrap(),
                        |value| value.0.clone(),
                    ),
                    amount: 1,
                },
                targets_all: true,
                target_kinds: BTreeSet::new(),
            });
        }
        let Some(value) = value else {
            return;
        };
        if enemy_types.is_empty() {
            ui.label("Enemy type");
            ui.monospace(value.enemy_type.to_string());
        } else {
            stable_id_required_choice(ui, "Enemy type", &mut value.enemy_type, enemy_types);
        }
        if enemy_pools.is_empty() {
            ui.label("Pool");
            ui.monospace(value.pool.to_string());
        } else {
            stable_id_required_choice(ui, "Pool", &mut value.pool, enemy_pools);
        }
        ui.horizontal_wrapped(|ui| {
            ui.add(
                egui::DragValue::new(&mut value.additional_health_milli_per_player)
                    .prefix("Health/player milli "),
            );
            ui.add(egui::DragValue::new(&mut value.action_amount).prefix("Damage "));
            ui.add(egui::DragValue::new(&mut value.action_milliseconds).prefix("Cadence ms "));
            ui.add(
                egui::DragValue::new(&mut value.action_range_milli_cells).prefix("Action range "),
            );
            ui.add(
                egui::DragValue::new(&mut value.target_search_range_milli_cells)
                    .prefix("Search range "),
            );
            ui.checkbox(&mut value.attack_attacker, "Retaliates");
        });
        ui.horizontal_wrapped(|ui| {
            if !resources.is_empty() {
                stable_id_required_choice(
                    ui,
                    "Kill reward",
                    &mut value.kill_reward.resource,
                    resources,
                );
            }
            ui.add(egui::DragValue::new(&mut value.kill_reward.amount).prefix("Amount "));
        });
        ui.checkbox(&mut value.targets_all, "Targets every kind");
        stable_id_set_choices(ui, "Target kinds", &mut value.target_kinds, target_kinds);
    });
}

fn enemy_model_set_editor(
    ui: &mut egui::Ui,
    value: &mut Option<EnemyModelSetDef>,
    model_nodes: &[String],
    animation_names: &[String],
) {
    ui.collapsing("Enemy model variants", |ui| {
        let mut enabled = value.is_some();
        if ui
            .checkbox(&mut enabled, "Uses enemy model handler")
            .changed()
        {
            *value = enabled.then_some(EnemyModelSetDef {
                base_models: Vec::new(),
                permanent_models: Vec::new(),
                optional_models: Vec::new(),
                weapons: Vec::new(),
                base_animation_variants: 1,
            });
        }
        let Some(value) = value else {
            return;
        };
        ui.add(
            egui::DragValue::new(&mut value.base_animation_variants)
                .range(1..=u8::MAX)
                .prefix("Base animation variants "),
        );
        model_string_vec_editor(ui, "Base models", &mut value.base_models, model_nodes);
        model_string_vec_editor(
            ui,
            "Permanent models",
            &mut value.permanent_models,
            model_nodes,
        );
        model_string_vec_editor(
            ui,
            "Optional models",
            &mut value.optional_models,
            model_nodes,
        );
        let mut remove_weapon = None;
        for (index, weapon) in value.weapons.iter_mut().enumerate() {
            ui.push_id(("enemy_weapon", index), |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        if model_nodes.is_empty() {
                            ui.text_edit_singleline(&mut weapon.main_model);
                        } else {
                            string_choice(ui, "Main model", &mut weapon.main_model, model_nodes);
                        }
                        if animation_names.is_empty() {
                            ui.text_edit_singleline(&mut weapon.action_animation);
                        } else {
                            string_choice(
                                ui,
                                "Action animation",
                                &mut weapon.action_animation,
                                animation_names,
                            );
                        }
                        ui.add(
                            egui::DragValue::new(&mut weapon.action_animation_variants)
                                .range(1..=u8::MAX)
                                .prefix("Variants "),
                        );
                        enemy_run_animation_choice(ui, &mut weapon.run_animation);
                        if ui.small_button("Remove weapon").clicked() {
                            remove_weapon = Some(index);
                        }
                    });
                    model_string_vec_editor(
                        ui,
                        "Off-hand models",
                        &mut weapon.off_hand_models,
                        model_nodes,
                    );
                });
            });
        }
        if let Some(index) = remove_weapon {
            value.weapons.remove(index);
        }
        if ui.button("Add weapon model").clicked() {
            value.weapons.push(EnemyWeaponModelDef {
                main_model: model_nodes.first().cloned().unwrap_or_default(),
                off_hand_models: Vec::new(),
                action_animation: animation_names
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "Attack".to_owned()),
                action_animation_variants: 1,
                run_animation: EnemyRunAnimation::Generic,
            });
        }
    });
}

fn model_string_vec_editor(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut Vec<String>,
    choices: &[String],
) {
    if choices.is_empty() {
        free_string_vec_editor(ui, label, values);
    } else {
        string_vec_choices(ui, label, values, choices);
    }
}

fn enemy_run_animation_choice(ui: &mut egui::Ui, value: &mut EnemyRunAnimation) {
    egui::ComboBox::from_id_salt(("enemy_run_animation", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(format!("{value:?}"))
        .show_ui(ui, |ui| {
            ui.selectable_value(value, EnemyRunAnimation::Generic, "Generic");
            ui.selectable_value(value, EnemyRunAnimation::TwoHanded, "Two handed");
        });
}

fn enemy_spawner_editor(
    ui: &mut egui::Ui,
    value: &mut Option<EnemySpawnerDef>,
    enemies: &[(StableId, String)],
) {
    ui.collapsing("Enemy spawner", |ui| {
        let mut enabled = value.is_some();
        if ui.checkbox(&mut enabled, "Spawns enemies").changed() {
            *value = enabled.then_some(EnemySpawnerDef {
                min_total_enemies: 1,
                max_total_enemies: 3,
                spawn_milliseconds: 5_000,
                weighted_enemies: Vec::new(),
                spawn_offsets_milli_cells: vec![[0, 0]],
            });
        }
        let Some(value) = value else {
            return;
        };
        ui.horizontal_wrapped(|ui| {
            ui.add(egui::DragValue::new(&mut value.min_total_enemies).prefix("Minimum "));
            ui.add(egui::DragValue::new(&mut value.max_total_enemies).prefix("Maximum "));
            ui.add(egui::DragValue::new(&mut value.spawn_milliseconds).prefix("Spawn cadence ms "));
        });
        let mut remove_enemy = None;
        for (index, spawn) in value.weighted_enemies.iter_mut().enumerate() {
            ui.horizontal_wrapped(|ui| {
                stable_id_required_choice(ui, "Enemy", &mut spawn.enemy_archetype, enemies);
                ui.add(egui::DragValue::new(&mut spawn.weight_milli).prefix("Weight milli "));
                if ui.small_button("Remove").clicked() {
                    remove_enemy = Some(index);
                }
            });
        }
        if let Some(index) = remove_enemy {
            value.weighted_enemies.remove(index);
        }
        if ui
            .add_enabled(!enemies.is_empty(), egui::Button::new("Add weighted enemy"))
            .clicked()
        {
            value.weighted_enemies.push(WeightedEnemySpawn {
                enemy_archetype: enemies[0].0.clone(),
                weight_milli: 1_000,
            });
        }
        let mut remove_offset = None;
        for (index, offset) in value.spawn_offsets_milli_cells.iter_mut().enumerate() {
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::DragValue::new(&mut offset[0]).prefix("Offset X "));
                ui.add(egui::DragValue::new(&mut offset[1]).prefix("Z "));
                if ui.small_button("Remove offset").clicked() {
                    remove_offset = Some(index);
                }
            });
        }
        if let Some(index) = remove_offset {
            value.spawn_offsets_milli_cells.remove(index);
        }
        if ui.button("Add spawn offset").clicked() {
            value.spawn_offsets_milli_cells.push([0, 0]);
        }
    });
}

fn pet_definition_editor(ui: &mut egui::Ui, value: &mut Option<PetDef>, models: &[String]) {
    ui.collapsing("Pet follower", |ui| {
        let mut enabled = value.is_some();
        if ui.checkbox(&mut enabled, "Is a pet follower").changed() {
            *value = enabled.then_some(PetDef {
                closest_distance: 0.5,
                max_distance: 5.0,
                min_move_speed: 1.0,
                max_move_speed: 4.0,
                rotation_radians_per_second: 3.0,
                models: BTreeMap::new(),
            });
        }
        let Some(value) = value else {
            return;
        };
        ui.horizontal_wrapped(|ui| {
            ui.add(egui::DragValue::new(&mut value.closest_distance).prefix("Closest "));
            ui.add(egui::DragValue::new(&mut value.max_distance).prefix("Maximum "));
            ui.add(egui::DragValue::new(&mut value.min_move_speed).prefix("Minimum speed "));
            ui.add(egui::DragValue::new(&mut value.max_move_speed).prefix("Maximum speed "));
            ui.add(
                egui::DragValue::new(&mut value.rotation_radians_per_second)
                    .prefix("Rotation rad/s "),
            );
        });
        let ids = value.models.keys().cloned().collect::<Vec<_>>();
        let mut remove = None;
        for id in ids {
            let Some(model) = value.models.get_mut(&id) else {
                continue;
            };
            ui.push_id(("pet_model", id.as_str()), |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.monospace(id.to_string());
                        if models.is_empty() {
                            ui.text_edit_singleline(&mut model.source_model);
                        } else {
                            string_choice(ui, "Source model", &mut model.source_model, models);
                        }
                        if ui.small_button("Remove").clicked() {
                            remove = Some(id.clone());
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        for (component, label) in
                            model
                                .local_position
                                .iter_mut()
                                .zip(["Position X", "Y", "Z"])
                        {
                            ui.add(egui::DragValue::new(component).prefix(format!("{label} ")));
                        }
                        for (component, label) in
                            model.local_scale.iter_mut().zip(["Scale X", "Y", "Z"])
                        {
                            ui.add(egui::DragValue::new(component).prefix(format!("{label} ")));
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        for (component, label) in
                            model
                                .local_rotation
                                .iter_mut()
                                .zip(["Rotation X", "Y", "Z", "W"])
                        {
                            ui.add(egui::DragValue::new(component).prefix(format!("{label} ")));
                        }
                    });
                });
            });
        }
        if let Some(id) = remove {
            value.models.remove(&id);
        }
        if ui.button("Add pet model").clicked() {
            let mut suffix = value.models.len();
            loop {
                let id = StableId::new(format!("pet_model:authored_{suffix}"))
                    .expect("generated pet model ID is valid");
                if let std::collections::btree_map::Entry::Vacant(entry) = value.models.entry(id) {
                    entry.insert(PetModelDef {
                        source_model: models.first().cloned().unwrap_or_default(),
                        local_position: [0.0; 3],
                        local_rotation: [0.0, 0.0, 0.0, 1.0],
                        local_scale: [1.0; 3],
                    });
                    break;
                }
                suffix += 1;
            }
        }
    });
}
