fn content_tab(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    egui::ScrollArea::vertical()
        .id_salt("models_assets_editor_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            content_tab_contents(ui, state, preview_texture, preview_status, preview_controls);
        });
}

fn content_tab_contents(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    ui.heading("Models and presentation assets");
    ui.label(
        "Every shipping presentation catalog is editable here: models, imported textures, PBR materials, and animation clips.",
    );
    ui.horizontal_wrapped(|ui| {
        for section in AssetEditorSection::ALL {
            ui.selectable_value(&mut state.asset_section, section, section.label());
        }
        ui.separator();
        if ui.button("Save presentation catalog").clicked() {
            state.status =
                match save_presentation_catalog(&state.presentation, &state.presentation_path) {
                    Ok(path) => format!("Saved presentation catalog to {}", path.display()),
                    Err(error) => format!("Could not save presentation catalog: {error:#}"),
                };
        }
    });
    match state.asset_section {
        AssetEditorSection::Models => {}
        AssetEditorSection::Textures => {
            texture_assets_editor(ui, state, preview_texture, preview_status, preview_controls);
            return;
        }
        AssetEditorSection::Materials => {
            material_assets_editor(ui, state, preview_texture, preview_status, preview_controls);
            return;
        }
        AssetEditorSection::Animations => {
            animation_assets_editor(ui, state, preview_texture, preview_status, preview_controls);
            return;
        }
    }
    let archetype_choices: Vec<_> = state
        .catalog
        .archetypes
        .iter()
        .map(|(id, archetype)| (id.clone(), archetype.display_name.clone()))
        .collect();
    ui.horizontal_wrapped(|ui| {
        stable_id_option_choice(
            ui,
            "Selected model",
            &mut state.selected_archetype,
            &archetype_choices,
        );
        if ui.button("Refresh discovered GLBs").clicked() {
            state.discovered_model_assets = discover_model_assets();
            state.status = format!(
                "Discovered {} GLB assets below bevy-port/assets",
                state.discovered_model_assets.len()
            );
        }
        if ui.button("Save catalog").clicked() {
            state.status = match save_content_catalog(&state.catalog, &state.catalog_path) {
                Ok(path) => format!("Saved model catalog to {}", path.display()),
                Err(error) => format!("Could not save model catalog: {error:#}"),
            };
        }
    });
    draw_model_preview(
        ui,
        preview_texture,
        preview_status,
        egui::vec2(500.0, 333.0),
        preview_controls,
        false,
    );
    ui.collapsing("Import a GLB asset", |ui| {
        ui.label(
            "Copies an existing .glb into assets/user/models. The original is retained and existing destination files are never overwritten.",
        );
        ui.horizontal_wrapped(|ui| {
            ui.label("Source file");
            ui.add(
                egui::TextEdit::singleline(&mut state.model_import_source)
                    .hint_text("C:\\path\\to\\model.glb")
                    .desired_width(420.0),
            );
            ui.label("Destination name (optional)");
            ui.add(
                egui::TextEdit::singleline(&mut state.model_import_name)
                    .hint_text("my_model.glb")
                    .desired_width(220.0),
            );
            if ui.button("Import GLB").clicked() {
                state.status = match import_model_asset(state) {
                    Ok(path) => format!("Imported model asset as {path}"),
                    Err(error) => format!("Model import rejected: {error}"),
                };
            }
        });
    });
    ui.collapsing("Add model archetype", |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label("Stable ID");
            ui.text_edit_singleline(&mut state.new_archetype_id);
            ui.label("Display name");
            ui.text_edit_singleline(&mut state.new_archetype_name);
            archetype_kind_choice(ui, &mut state.new_archetype_kind);
        });
        searchable_string_choice(
            ui,
            "new_archetype_asset",
            "GLB asset",
            &mut state.new_archetype_asset,
            &state.discovered_model_assets,
            &mut state.asset_search,
        );
        if ui.button("Add model archetype").clicked() {
            state.status = match create_model_archetype(state) {
                Ok(()) => "Added a validated model archetype and selected it".to_owned(),
                Err(error) => format!("Model creation rejected: {error}"),
            };
        }
    });

    let selected_archetype = state.selected_archetype.clone();
    if let Some(id) = selected_archetype {
        let model_assets = state.discovered_model_assets.clone();
        let selected_scene = state
            .catalog
            .archetypes
            .get(&id)
            .and_then(|archetype| {
                let index = state
                    .selected_archetype_scene
                    .min(archetype.scenes.len().saturating_sub(1));
                archetype.scenes.get(index)
            })
            .cloned();
        let selected_source_guid = state
            .catalog
            .archetypes
            .get(&id)
            .map(|archetype| archetype.source_guid.clone())
            .unwrap_or_default();
        let selected_metadata = selected_scene
            .as_ref()
            .map(|scene| cached_gltf_metadata(state, &scene.asset_path))
            .unwrap_or_default();
        let resources = resource_choices(&state.catalog);
        let target_kinds = target_kind_choices(&state.catalog);
        let enemy_archetypes = state
            .catalog
            .archetypes
            .iter()
            .filter(|(_, archetype)| archetype.enemy.is_some())
            .map(|(id, archetype)| (id.clone(), archetype.display_name.clone()))
            .collect::<Vec<_>>();
        let enemy_types = labeled_ids(
            state
                .catalog
                .archetypes
                .values()
                .filter_map(|archetype| archetype.enemy.as_ref())
                .map(|enemy| enemy.enemy_type.clone())
                .collect(),
        );
        let enemy_pools = labeled_ids(
            state
                .catalog
                .archetypes
                .values()
                .filter_map(|archetype| archetype.enemy.as_ref())
                .map(|enemy| enemy.pool.clone())
                .collect(),
        );
        let mut model_nodes = building_model_node_choices(&state.catalog);
        model_nodes.extend(selected_metadata.nodes.iter().cloned());
        model_nodes.sort();
        model_nodes.dedup();
        let animation_names = action_animation_choices(&state.catalog);
        let asset_source_models: BTreeMap<_, _> = state
            .catalog
            .archetypes
            .values()
            .flat_map(|archetype| archetype.scenes.iter())
            .map(|scene| (scene.asset_path.clone(), scene.source_model.clone()))
            .collect();
        let mut remove_scene = None;
        let mut make_default = None;
        let mut add_scene = false;
        let mut delete_archetype = false;
        let mut synchronized_footprint = None;
        let asset_search = &mut state.asset_search;
        if let Some(archetype) = state.catalog.archetypes.get_mut(&id) {
            ui.collapsing("Selected archetype and variants", |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Display name");
                    ui.text_edit_singleline(&mut archetype.display_name);
                    archetype_kind_choice(ui, &mut archetype.kind);
                });
                if footprint_editor(
                    ui,
                    &mut archetype.footprint,
                    "Default logical footprint",
                    4_096,
                ) {
                    synchronized_footprint = Some(archetype.footprint);
                }
                ui.collapsing("Bounds, provenance, and components", |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Source GUID");
                        ui.text_edit_singleline(&mut archetype.source_guid);
                        ui.label("Source path");
                        ui.text_edit_singleline(&mut archetype.source_path);
                    });
                    ui.label("Bounds centre / size");
                    ui.horizontal_wrapped(|ui| {
                        for (value, label) in archetype
                            .bounds
                            .center
                            .iter_mut()
                            .zip(["Centre X", "Y", "Z"])
                        {
                            ui.add(egui::DragValue::new(value).prefix(format!("{label} ")));
                        }
                        for (value, label) in
                            archetype.bounds.size.iter_mut().zip(["Size X", "Y", "Z"])
                        {
                            ui.add(
                                egui::DragValue::new(value)
                                    .range(0.001..=100_000.0)
                                    .prefix(format!("{label} ")),
                            );
                        }
                    });
                    free_string_vec_editor(ui, "Component types", &mut archetype.component_types);
                });
                archetype_runtime_editor(
                    ui,
                    archetype,
                    &resources,
                    &target_kinds,
                    &enemy_archetypes,
                    &enemy_types,
                    &enemy_pools,
                    &model_nodes,
                    &animation_names,
                );
                for (index, scene) in archetype.scenes.iter_mut().enumerate() {
                    ui.push_id(("archetype_scene", index), |ui| {
                        ui.group(|ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.strong(format!("Variant {}", index + 1));
                                if ui.radio(scene.is_default, "Default").clicked() {
                                    state.selected_archetype_scene = index;
                                    make_default = Some(index);
                                }
                                ui.add(
                                    egui::DragValue::new(scene.age.get_or_insert(1))
                                        .range(1..=u8::MAX)
                                        .prefix("Age "),
                                );
                                if ui.small_button("No age").clicked() {
                                    scene.age = None;
                                }
                                if ui.small_button("Remove").clicked() {
                                    remove_scene = Some(index);
                                }
                            });
                            if searchable_string_choice(
                                ui,
                                "scene_asset",
                                "GLB asset",
                                &mut scene.asset_path,
                                &model_assets,
                                asset_search,
                            ) {
                                scene.source_model = asset_source_models
                                    .get(&scene.asset_path)
                                    .cloned()
                                    .unwrap_or_else(|| scene.asset_path.clone());
                            }
                        });
                    });
                }
                add_scene = ui.button("Add GLB variant").clicked();
                delete_archetype = ui.button("Delete unreferenced archetype").clicked();
            });
        }
        if let Some(footprint) = synchronized_footprint {
            let updated = synchronize_buildings_for_archetype(&mut state.catalog, &id, footprint);
            if let Some(draft) = state
                .building_draft
                .as_mut()
                .filter(|draft| draft.value.archetype == id)
            {
                draft.value.footprint = footprint;
            }
            state.status = format!(
                "Updated model footprint and synchronized {updated} logical building placement record(s)"
            );
        }
        if let Some(index) = make_default
            && let Some(archetype) = state.catalog.archetypes.get_mut(&id)
        {
            for (scene_index, scene) in archetype.scenes.iter_mut().enumerate() {
                scene.is_default = scene_index == index;
            }
        }
        if let Some(index) = remove_scene {
            state.status = match remove_archetype_scene(state, &id, index) {
                Ok(()) => "Removed model variant".to_owned(),
                Err(error) => format!("Could not remove model variant: {error}"),
            };
        } else if add_scene {
            state.status = match add_archetype_scene(state, &id) {
                Ok(()) => "Added model variant".to_owned(),
                Err(error) => format!("Could not add model variant: {error}"),
            };
        } else if delete_archetype {
            state.status = match delete_model_archetype(state, &id) {
                Ok(()) => "Deleted unreferenced model archetype".to_owned(),
                Err(error) => format!("Could not delete model archetype: {error}"),
            };
        }
        if let Some(scene) = selected_scene {
            gltf_material_bindings_editor(
                ui,
                &mut state.presentation,
                &id,
                &selected_source_guid,
                &scene,
                &selected_metadata,
            );
        }
    }
    ui.horizontal_wrapped(|ui| {
        ui.label(format!("Archetypes: {}", state.catalog.archetypes.len()));
        ui.separator();
        ui.label(format!(
            "GLB variants: {}",
            state
                .catalog
                .archetypes
                .values()
                .map(|archetype| archetype.scenes.len())
                .sum::<usize>()
        ));
        ui.separator();
        ui.label(format!("Materials: {}", state.presentation.materials.len()));
        ui.separator();
        ui.label(format!("Textures: {}", state.presentation.textures.len()));
        ui.separator();
        ui.label(format!(
            "Controllers: {}",
            state.presentation.controllers.len()
        ));
        ui.separator();
        ui.label(format!("Clips: {}", state.presentation.clips.len()));
    });
    ui.collapsing("Complete catalog reference inventory", |ui| {
        ui.collapsing("Prefab archetypes and GLB variants", |ui| {
            for (id, archetype) in &state.catalog.archetypes {
                ui.collapsing(format!("{}  ({id})", archetype.display_name), |ui| {
                    ui.label(format!("Kind: {:?}", archetype.kind));
                    ui.monospace(format!("Source provenance: {}", archetype.source_path));
                    for scene in &archetype.scenes {
                        ui.horizontal(|ui| {
                            ui.label(if scene.is_default { "●" } else { "○" });
                            ui.monospace(&scene.asset_path);
                        });
                    }
                    if let Some(bindings) = state
                        .presentation
                        .prefab_renderer_materials
                        .get(&archetype.source_guid)
                    {
                        ui.label(format!("{} renderer binding(s)", bindings.len()));
                    }
                });
            }
        });
        ui.collapsing("Materials and texture bindings", |ui| {
            for (id, material) in &state.presentation.materials {
                ui.collapsing(format!("{}  ({id})", material.display_name), |ui| {
                    let color: egui::Color32 = egui::Rgba::from_rgba_unmultiplied(
                        material.base_color[0].clamp(0.0, 1.0),
                        material.base_color[1].clamp(0.0, 1.0),
                        material.base_color[2].clamp(0.0, 1.0),
                        material.base_color[3].clamp(0.0, 1.0),
                    )
                    .into();
                    ui.colored_label(color, "████  authored base colour");
                    ui.monospace(format!("Source provenance: {}", material.source_path));
                    ui.label(format!(
                        "Metallic {:.2} · roughness {:.2} · {:?}",
                        material.metallic, material.perceptual_roughness, material.alpha_mode
                    ));
                    for (slot, texture) in &material.textures {
                        let path = state
                            .presentation
                            .textures
                            .get(texture)
                            .map_or("missing", |texture| texture.asset_path.as_str());
                        ui.monospace(format!("{slot}: {path}"));
                    }
                });
            }
        });
        ui.collapsing("Animation controllers and clips", |ui| {
            for (id, controller) in &state.presentation.controllers {
                ui.collapsing(format!("{}  ({id})", controller.display_name), |ui| {
                    ui.monospace(format!("Source provenance: {}", controller.source_path));
                    ui.label(format!(
                        "{} layers · {} states · {} transitions · {} parameters",
                        controller.layers.len(),
                        controller.states.len(),
                        controller.transitions.len(),
                        controller.parameters.len()
                    ));
                    for state_def in controller.states.values() {
                        ui.monospace(format!(
                            "{} · {:.2}x · {} motion(s)",
                            state_def.display_name,
                            state_def.speed,
                            state_def.motions.len()
                        ));
                    }
                });
            }
            ui.separator();
            for (id, clip) in &state.presentation.clips {
                ui.collapsing(format!("{}  ({id})", clip.display_name), |ui| {
                    ui.monospace(format!("Source provenance: {}", clip.source_path));
                    ui.label(format!(
                        "{:.3}s · {:.1} Hz · {} tracks · {} events · looping {}",
                        clip.duration_seconds,
                        clip.sample_rate,
                        clip.transform_tracks.len(),
                        clip.events.len(),
                        clip.looping
                    ));
                });
            }
        });
    });
}

fn gltf_material_bindings_editor(
    ui: &mut egui::Ui,
    presentation: &mut PresentationCatalog,
    archetype_id: &StableId,
    prefab_guid: &str,
    scene: &ArchetypeScene,
    metadata: &GltfMetadata,
) {
    let material_choices = presentation
        .materials
        .iter()
        .map(|(id, material)| (id.clone(), material.display_name.clone()))
        .collect::<Vec<_>>();
    ui.collapsing("GLB hierarchy, animations, and material assignments", |ui| {
        ui.label(format!(
            "{} named nodes · {} embedded materials · {} embedded animations",
            metadata.nodes.len(),
            metadata.materials.len(),
            metadata.animations.len()
        ));
        if !metadata.animations.is_empty() {
            ui.horizontal_wrapped(|ui| {
                ui.strong("Animations:");
                for (index, name) in &metadata.animations {
                    ui.monospace(format!("{index}: {name}"));
                }
            });
        }
        ui.label(
            "Model-wide material assignments use names read directly from the GLB; no embedded material name needs to be typed.",
        );
        let bindings = presentation
            .model_materials
            .entry(scene.source_model.clone())
            .or_default();
        for embedded in &metadata.materials {
            ui.push_id(("model_material_binding", embedded), |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.monospace(embedded);
                    let mut selected = bindings.get(embedded).cloned();
                    stable_id_option_choice(
                        ui,
                        "Authored material",
                        &mut selected,
                        &material_choices,
                    );
                    if let Some(material) = selected {
                        bindings.insert(embedded.clone(), material);
                    } else {
                        bindings.remove(embedded);
                    }
                });
            });
        }
        if bindings.is_empty() {
            presentation.model_materials.remove(&scene.source_model);
        }

        ui.separator();
        ui.label(format!(
            "Per-renderer overrides for {} ({archetype_id})",
            scene.asset_path
        ));
        let renderers = presentation
            .prefab_renderer_materials
            .entry(prefab_guid.to_owned())
            .or_default();
        let mut remove_renderer = None;
        for (renderer_index, renderer) in renderers.iter_mut().enumerate() {
            ui.push_id(("renderer_material_binding", renderer_index), |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        string_choice(
                            ui,
                            "Renderer node",
                            &mut renderer.target_path,
                            &metadata.nodes,
                        );
                        if ui.small_button("Remove renderer override").clicked() {
                            remove_renderer = Some(renderer_index);
                        }
                    });
                    let slots = renderer.materials.keys().cloned().collect::<Vec<_>>();
                    let mut remove_slot = None;
                    for embedded in slots {
                        ui.horizontal_wrapped(|ui| {
                            ui.monospace(&embedded);
                            if let Some(material) = renderer.materials.get_mut(&embedded) {
                                stable_id_required_choice(
                                    ui,
                                    "Material",
                                    material,
                                    &material_choices,
                                );
                            }
                            if ui.small_button("Remove slot").clicked() {
                                remove_slot = Some(embedded.clone());
                            }
                        });
                    }
                    if let Some(slot) = remove_slot
                        && renderer.materials.len() > 1
                    {
                        renderer.materials.remove(&slot);
                    }
                    let unused_material = metadata
                        .materials
                        .iter()
                        .find(|name| !renderer.materials.contains_key(*name));
                    if ui
                        .add_enabled(
                            unused_material.is_some() && !material_choices.is_empty(),
                            egui::Button::new("Add renderer material slot"),
                        )
                        .clicked()
                        && let (Some(embedded), Some((material, _))) =
                            (unused_material, material_choices.first())
                    {
                        renderer
                            .materials
                            .insert(embedded.clone(), material.clone());
                    }
                });
            });
        }
        if let Some(index) = remove_renderer {
            renderers.remove(index);
        }
        let unused_node = metadata
            .nodes
            .iter()
            .find(|node| {
                !renderers
                    .iter()
                    .any(|renderer| renderer.target_path == node.as_str())
            });
        if ui
            .add_enabled(
                unused_node.is_some()
                    && !metadata.materials.is_empty()
                    && !material_choices.is_empty(),
                egui::Button::new("Add renderer override"),
            )
            .clicked()
            && let (Some(node), Some(embedded), Some((material, _))) = (
                unused_node,
                metadata.materials.first(),
                material_choices.first(),
            )
        {
            renderers.push(RendererMaterialBinding {
                target_path: node.clone(),
                materials: BTreeMap::from([(embedded.clone(), material.clone())]),
            });
        }
        if renderers.is_empty() {
            presentation.prefab_renderer_materials.remove(prefab_guid);
        }
    });
}
