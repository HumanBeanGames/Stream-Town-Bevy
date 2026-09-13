fn texture_assets_editor(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    let choices = state
        .presentation
        .textures
        .iter()
        .map(|(id, value)| (id.clone(), value.display_name.clone()))
        .collect::<Vec<_>>();
    ui.horizontal_wrapped(|ui| {
        stable_id_option_choice(ui, "Texture", &mut state.selected_texture, &choices);
        if ui.button("Refresh texture files").clicked() {
            state.discovered_texture_assets = discover_texture_assets();
            state.status = format!(
                "Discovered {} supported textures",
                state.discovered_texture_assets.len()
            );
        }
        if ui
            .add_enabled(
                state.selected_texture.is_some(),
                egui::Button::new("Delete"),
            )
            .clicked()
        {
            state.status = match delete_selected_texture(state) {
                Ok(()) => "Deleted unreferenced texture definition".to_owned(),
                Err(error) => format!("Texture deletion rejected: {error}"),
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
    ui.collapsing("Import texture file", |ui| {
        ui.label("Copies a supported image into assets/shipping/textures/user without overwriting existing files.");
        ui.horizontal_wrapped(|ui| {
            ui.label("Source file");
            ui.add(
                egui::TextEdit::singleline(&mut state.texture_import_source)
                    .desired_width(400.0)
                    .hint_text("C:\\path\\to\\texture.png"),
            );
            ui.label("Destination name");
            ui.add(
                egui::TextEdit::singleline(&mut state.texture_import_name)
                    .desired_width(220.0)
                    .hint_text("optional_name.png"),
            );
            if ui.button("Import").clicked() {
                state.status = match import_texture_asset(state) {
                    Ok(path) => format!("Imported texture as {path}"),
                    Err(error) => format!("Texture import rejected: {error}"),
                };
            }
        });
    });
    ui.collapsing("Add texture definition", |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label("Stable ID");
            ui.text_edit_singleline(&mut state.new_texture_id);
            ui.label("Name");
            ui.text_edit_singleline(&mut state.new_texture_name);
        });
        let assets = state.discovered_texture_assets.clone();
        searchable_string_choice(
            ui,
            "new_texture_asset",
            "Texture file",
            &mut state.new_texture_asset,
            &assets,
            &mut state.asset_search,
        );
        if ui.button("Add texture").clicked() {
            state.status = match create_texture_definition(state) {
                Ok(()) => "Added texture definition and selected it".to_owned(),
                Err(error) => format!("Texture creation rejected: {error}"),
            };
        }
    });
    if let Some(id) = state.selected_texture.clone()
        && let Some(texture) = state.presentation.textures.get_mut(&id)
    {
        ui.separator();
        ui.heading(&texture.display_name);
        ui.monospace(id.to_string());
        ui.horizontal_wrapped(|ui| {
            ui.label("Display name");
            ui.text_edit_singleline(&mut texture.display_name);
            ui.label("Source GUID");
            ui.text_edit_singleline(&mut texture.source_guid);
        });
        ui.horizontal_wrapped(|ui| {
            ui.label("Source path");
            ui.text_edit_singleline(&mut texture.source_path);
        });
        let assets = state.discovered_texture_assets.clone();
        searchable_string_choice(
            ui,
            "selected_texture_asset",
            "Asset file",
            &mut texture.asset_path,
            &assets,
            &mut state.asset_search,
        );
        let mut nine_slice = texture.sprite_border.is_some();
        if ui
            .checkbox(&mut nine_slice, "Nine-slice sprite border")
            .changed()
        {
            texture.sprite_border = nine_slice.then_some([8.0; 4]);
        }
        if let Some(border) = texture.sprite_border.as_mut() {
            ui.horizontal_wrapped(|ui| {
                for (value, label) in border.iter_mut().zip(["Left", "Right", "Top", "Bottom"]) {
                    ui.add(
                        egui::DragValue::new(value)
                            .range(0.0..=8_192.0)
                            .prefix(format!("{label} ")),
                    );
                }
            });
        }
    }
}

fn create_texture_definition(state: &mut ToolState) -> Result<(), String> {
    let id = StableId::new(state.new_texture_id.trim()).map_err(|error| error.to_string())?;
    if state.presentation.textures.contains_key(&id) {
        return Err(format!("texture {id} already exists"));
    }
    let mut candidate = state.presentation.clone();
    candidate.textures.insert(
        id.clone(),
        TextureDef {
            display_name: state.new_texture_name.trim().to_owned(),
            source_guid: "authored-in-bevy".to_owned(),
            source_path: "authoring://stream-town-tools".to_owned(),
            asset_path: state.new_texture_asset.clone(),
            sprite_border: None,
        },
    );
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_texture = Some(id);
    "texture:new".clone_into(&mut state.new_texture_id);
    "New Texture".clone_into(&mut state.new_texture_name);
    Ok(())
}

fn delete_selected_texture(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_texture
        .clone()
        .ok_or_else(|| "no texture selected".to_owned())?;
    let mut candidate = state.presentation.clone();
    candidate
        .textures
        .remove(&id)
        .ok_or_else(|| format!("missing texture {id}"))?;
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_texture = state.presentation.textures.keys().next().cloned();
    Ok(())
}

fn material_assets_editor(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    let material_choices = state
        .presentation
        .materials
        .iter()
        .map(|(id, value)| (id.clone(), value.display_name.clone()))
        .collect::<Vec<_>>();
    let texture_choices = state
        .presentation
        .textures
        .iter()
        .map(|(id, value)| (id.clone(), value.display_name.clone()))
        .collect::<Vec<_>>();
    ui.horizontal_wrapped(|ui| {
        stable_id_option_choice(
            ui,
            "Material",
            &mut state.selected_material,
            &material_choices,
        );
        ui.label("New stable ID");
        ui.text_edit_singleline(&mut state.new_material_id);
        ui.label("Name");
        ui.text_edit_singleline(&mut state.new_material_name);
        if ui.button("Add material").clicked() {
            state.status = match create_material_definition(state) {
                Ok(()) => "Added material and selected it".to_owned(),
                Err(error) => format!("Material creation rejected: {error}"),
            };
        }
        if ui
            .add_enabled(
                state.selected_material.is_some(),
                egui::Button::new("Delete"),
            )
            .clicked()
        {
            state.status = match delete_selected_material(state) {
                Ok(()) => "Deleted unreferenced material".to_owned(),
                Err(error) => format!("Material deletion rejected: {error}"),
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
    let Some(id) = state.selected_material.clone() else {
        return;
    };
    let Some(material) = state.presentation.materials.get_mut(&id) else {
        return;
    };
    ui.separator();
    ui.heading(&material.display_name);
    ui.monospace(id.to_string());
    ui.horizontal_wrapped(|ui| {
        ui.label("Display name");
        ui.text_edit_singleline(&mut material.display_name);
        ui.label("Source GUID");
        ui.text_edit_singleline(&mut material.source_guid);
    });
    ui.horizontal_wrapped(|ui| {
        ui.label("Source path");
        ui.text_edit_singleline(&mut material.source_path);
        ui.label("Shader source");
        optional_string_editor(ui, &mut material.shader_source, "Assets/Shaders/...");
    });
    ui.horizontal_wrapped(|ui| {
        ui.label("Base colour");
        ui.color_edit_button_rgba_unmultiplied(&mut material.base_color);
        ui.label("Emissive");
        ui.color_edit_button_rgba_unmultiplied(&mut material.emissive);
        ui.add(egui::Slider::new(&mut material.metallic, 0.0..=1.0).text("Metallic"));
        ui.add(egui::Slider::new(&mut material.perceptual_roughness, 0.0..=1.0).text("Roughness"));
        material_alpha_choice(ui, &mut material.alpha_mode);
    });
    ui.collapsing("Texture slots and UV transforms", |ui| {
        let slots = material.textures.keys().cloned().collect::<Vec<_>>();
        let mut remove = None;
        for slot in slots {
            ui.push_id(("material_texture", &slot), |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.monospace(&slot);
                    if let Some(texture) = material.textures.get_mut(&slot) {
                        stable_id_required_choice(ui, "Texture", texture, &texture_choices);
                    }
                    if ui.small_button("Remove").clicked() {
                        remove = Some(slot.clone());
                    }
                });
                let mut transformed = material.texture_transforms.contains_key(&slot);
                if ui
                    .checkbox(&mut transformed, "Custom tiling/offset")
                    .changed()
                {
                    if transformed {
                        material
                            .texture_transforms
                            .insert(slot.clone(), TextureTransform::default());
                    } else {
                        material.texture_transforms.remove(&slot);
                    }
                }
                if let Some(transform) = material.texture_transforms.get_mut(&slot) {
                    ui.horizontal_wrapped(|ui| {
                        ui.add(egui::DragValue::new(&mut transform.scale[0]).prefix("Tile X "));
                        ui.add(egui::DragValue::new(&mut transform.scale[1]).prefix("Y "));
                        ui.add(egui::DragValue::new(&mut transform.offset[0]).prefix("Offset X "));
                        ui.add(egui::DragValue::new(&mut transform.offset[1]).prefix("Y "));
                    });
                }
            });
        }
        if let Some(slot) = remove {
            material.textures.remove(&slot);
            material.texture_transforms.remove(&slot);
        }
        ui.horizontal_wrapped(|ui| {
            ui.label("New slot");
            ui.text_edit_singleline(&mut state.new_material_texture_slot);
            if ui
                .add_enabled(
                    !texture_choices.is_empty(),
                    egui::Button::new("Add texture slot"),
                )
                .clicked()
                && let Some((texture, _)) = texture_choices.first()
            {
                material
                    .textures
                    .entry(state.new_material_texture_slot.trim().to_owned())
                    .or_insert_with(|| texture.clone());
            }
        });
    });
    ui.collapsing("Custom shader scalar properties", |ui| {
        string_f32_map_editor(
            ui,
            &mut material.custom_properties,
            &mut state.new_material_property,
        );
    });
    ui.collapsing("Custom shader vector/colour properties", |ui| {
        string_vec4_map_editor(
            ui,
            &mut material.custom_vectors,
            &mut state.new_material_vector,
        );
    });
}

fn material_alpha_choice(ui: &mut egui::Ui, value: &mut MaterialAlphaMode) {
    egui::ComboBox::from_id_salt(("material_alpha", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(format!("{value:?}"))
        .show_ui(ui, |ui| {
            ui.selectable_value(value, MaterialAlphaMode::Opaque, "Opaque");
            ui.selectable_value(value, MaterialAlphaMode::Mask, "Alpha mask");
            ui.selectable_value(value, MaterialAlphaMode::Blend, "Transparent blend");
        });
}

fn create_material_definition(state: &mut ToolState) -> Result<(), String> {
    let id = StableId::new(state.new_material_id.trim()).map_err(|error| error.to_string())?;
    if state.presentation.materials.contains_key(&id) {
        return Err(format!("material {id} already exists"));
    }
    let mut candidate = state.presentation.clone();
    candidate.materials.insert(
        id.clone(),
        MaterialDef {
            display_name: state.new_material_name.trim().to_owned(),
            source_guid: "authored-in-bevy".to_owned(),
            source_path: "authoring://stream-town-tools".to_owned(),
            shader_source: None,
            base_color: [1.0; 4],
            emissive: [0.0, 0.0, 0.0, 1.0],
            metallic: 0.0,
            perceptual_roughness: 0.75,
            alpha_mode: MaterialAlphaMode::Opaque,
            textures: BTreeMap::new(),
            texture_transforms: BTreeMap::new(),
            custom_properties: BTreeMap::new(),
            custom_vectors: BTreeMap::new(),
        },
    );
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_material = Some(id);
    "material:new".clone_into(&mut state.new_material_id);
    "New Material".clone_into(&mut state.new_material_name);
    Ok(())
}

fn delete_selected_material(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_material
        .clone()
        .ok_or_else(|| "no material selected".to_owned())?;
    let mut candidate = state.presentation.clone();
    candidate
        .materials
        .remove(&id)
        .ok_or_else(|| format!("missing material {id}"))?;
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_material = state.presentation.materials.keys().next().cloned();
    Ok(())
}

fn string_f32_map_editor(
    ui: &mut egui::Ui,
    values: &mut BTreeMap<String, f32>,
    new_key: &mut String,
) {
    let keys = values.keys().cloned().collect::<Vec<_>>();
    let mut remove = None;
    for key in keys {
        ui.horizontal_wrapped(|ui| {
            ui.monospace(&key);
            if let Some(value) = values.get_mut(&key) {
                ui.add(egui::DragValue::new(value).speed(0.01));
            }
            if ui.small_button("Remove").clicked() {
                remove = Some(key.clone());
            }
        });
    }
    if let Some(key) = remove {
        values.remove(&key);
    }
    ui.horizontal_wrapped(|ui| {
        ui.text_edit_singleline(new_key);
        if ui.button("Add scalar").clicked() && !new_key.trim().is_empty() {
            values.entry(new_key.trim().to_owned()).or_insert(0.0);
        }
    });
}

fn string_vec4_map_editor(
    ui: &mut egui::Ui,
    values: &mut BTreeMap<String, [f32; 4]>,
    new_key: &mut String,
) {
    let keys = values.keys().cloned().collect::<Vec<_>>();
    let mut remove = None;
    for key in keys {
        ui.group(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.monospace(&key);
                if let Some(value) = values.get_mut(&key) {
                    ui.color_edit_button_rgba_unmultiplied(value);
                    for component in value {
                        ui.add(egui::DragValue::new(component).speed(0.01));
                    }
                }
                if ui.small_button("Remove").clicked() {
                    remove = Some(key.clone());
                }
            });
        });
    }
    if let Some(key) = remove {
        values.remove(&key);
    }
    ui.horizontal_wrapped(|ui| {
        ui.text_edit_singleline(new_key);
        if ui.button("Add vector").clicked() && !new_key.trim().is_empty() {
            values.entry(new_key.trim().to_owned()).or_insert([1.0; 4]);
        }
    });
}

fn optional_string_editor(ui: &mut egui::Ui, value: &mut Option<String>, hint: &str) {
    let mut enabled = value.is_some();
    if ui.checkbox(&mut enabled, "Set").changed() {
        *value = enabled.then(String::new);
    }
    if let Some(value) = value {
        ui.add(
            egui::TextEdit::singleline(value)
                .desired_width(280.0)
                .hint_text(hint),
        );
    }
}

fn animation_assets_editor(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    preview_texture: Option<egui::TextureId>,
    preview_status: &str,
    preview_controls: &mut ModelPreviewControls,
) {
    let clip_choices = state
        .presentation
        .clips
        .iter()
        .map(|(id, value)| (id.clone(), value.display_name.clone()))
        .collect::<Vec<_>>();
    let animation_assets = state
        .discovered_model_assets
        .iter()
        .filter(|path| path.starts_with("shipping/models/"))
        .cloned()
        .collect::<Vec<_>>();
    ui.horizontal_wrapped(|ui| {
        stable_id_option_choice(
            ui,
            "Animation clip",
            &mut state.selected_clip,
            &clip_choices,
        );
        ui.label("New stable ID");
        ui.text_edit_singleline(&mut state.new_clip_id);
        ui.label("Name");
        ui.text_edit_singleline(&mut state.new_clip_name);
    });
    searchable_string_choice(
        ui,
        "new_clip_asset",
        "Converted rig/animation GLB",
        &mut state.new_clip_asset,
        &animation_assets,
        &mut state.asset_search,
    );
    ui.horizontal_wrapped(|ui| {
        if ui.button("Add animation clip").clicked() {
            state.status = match create_animation_clip(state) {
                Ok(()) => "Added animation clip and selected it".to_owned(),
                Err(error) => format!("Animation creation rejected: {error}"),
            };
        }
        if ui
            .add_enabled(
                state.selected_clip.is_some(),
                egui::Button::new("Delete clip"),
            )
            .clicked()
        {
            state.status = match delete_selected_animation_clip(state) {
                Ok(()) => "Deleted unreferenced animation clip".to_owned(),
                Err(error) => format!("Animation deletion rejected: {error}"),
            };
        }
    });
    draw_model_preview(
        ui,
        preview_texture,
        preview_status,
        egui::vec2(560.0, 373.0),
        preview_controls,
        true,
    );
    let Some(id) = state.selected_clip.clone() else {
        return;
    };
    let selected_clip_asset = state
        .presentation
        .clips
        .get(&id)
        .and_then(|clip| {
            clip.converted_asset_path
                .as_ref()
                .or(clip.rig_asset_path.as_ref())
        })
        .cloned();
    let clip_metadata = selected_clip_asset
        .as_deref()
        .map(|path| cached_gltf_metadata(state, path))
        .unwrap_or_default();
    let Some(clip) = state.presentation.clips.get_mut(&id) else {
        return;
    };
    ui.separator();
    ui.heading(&clip.display_name);
    ui.monospace(id.to_string());
    ui.horizontal_wrapped(|ui| {
        ui.label("Display name");
        ui.text_edit_singleline(&mut clip.display_name);
        ui.label("Source GUID");
        ui.text_edit_singleline(&mut clip.source_guid);
        ui.checkbox(&mut clip.looping, "Authored looping");
    });
    ui.horizontal_wrapped(|ui| {
        ui.label("Source path");
        ui.text_edit_singleline(&mut clip.source_path);
        ui.add(
            egui::DragValue::new(&mut clip.duration_seconds)
                .range(0.001..=86_400.0)
                .suffix(" seconds"),
        );
        ui.add(
            egui::DragValue::new(&mut clip.sample_rate)
                .range(0.001..=10_000.0)
                .suffix(" Hz"),
        );
    });
    optional_asset_choice(
        ui,
        "clip_rig_asset",
        "Rig GLB",
        &mut clip.rig_asset_path,
        &animation_assets,
        &mut state.asset_search,
    );
    optional_asset_choice(
        ui,
        "clip_converted_asset",
        "Converted animation GLB",
        &mut clip.converted_asset_path,
        &animation_assets,
        &mut state.asset_search,
    );
    let mut has_index = clip.gltf_animation_index.is_some();
    ui.horizontal_wrapped(|ui| {
        if ui
            .checkbox(&mut has_index, "glTF animation index")
            .changed()
        {
            clip.gltf_animation_index = has_index.then_some(0);
        }
        if let Some(index) = clip.gltf_animation_index.as_mut() {
            if clip_metadata.animations.is_empty() {
                ui.add(egui::DragValue::new(index));
            } else {
                egui::ComboBox::from_id_salt("gltf_animation_index")
                    .height(AUTHORING_COMBO_MAX_HEIGHT)
                    .selected_text(
                        clip_metadata
                            .animations
                            .iter()
                            .find(|(candidate, _)| u32::try_from(*candidate).ok() == Some(*index))
                            .map_or_else(|| format!("Animation {index}"), |(_, name)| name.clone()),
                    )
                    .show_ui(ui, |ui| {
                        for (candidate, name) in &clip_metadata.animations {
                            if let Ok(candidate) = u32::try_from(*candidate) {
                                ui.selectable_value(
                                    index,
                                    candidate,
                                    format!("{candidate}: {name}"),
                                );
                            }
                        }
                    });
            }
        }
    });
    animation_transform_tracks_editor(ui, &mut clip.transform_tracks, clip.duration_seconds);
    animation_property_curves_editor(ui, &mut clip.property_curves, clip.duration_seconds);
    animation_events_editor(ui, &mut clip.events, clip.duration_seconds);
    ui.separator();
    animation_controller_editor(ui, state, &clip_choices);
}

fn optional_asset_choice(
    ui: &mut egui::Ui,
    salt: &str,
    label: &str,
    value: &mut Option<String>,
    choices: &[String],
    search: &mut String,
) {
    let mut enabled = value.is_some();
    ui.horizontal_wrapped(|ui| {
        if ui.checkbox(&mut enabled, label).changed() {
            *value = enabled.then(|| choices.first().cloned().unwrap_or_default());
        }
    });
    if let Some(value) = value {
        searchable_string_choice(ui, salt, label, value, choices, search);
    }
}

fn create_animation_clip(state: &mut ToolState) -> Result<(), String> {
    let id = StableId::new(state.new_clip_id.trim()).map_err(|error| error.to_string())?;
    if state.presentation.clips.contains_key(&id) {
        return Err(format!("animation clip {id} already exists"));
    }
    if !state.new_clip_asset.starts_with("shipping/models/") {
        return Err("animation GLBs must live below shipping/models".to_owned());
    }
    let mut candidate = state.presentation.clone();
    candidate.clips.insert(
        id.clone(),
        AnimationClipDef {
            display_name: state.new_clip_name.trim().to_owned(),
            source_guid: "authored-in-bevy".to_owned(),
            source_path: "authoring://stream-town-tools".to_owned(),
            duration_seconds: 1.0,
            sample_rate: 30.0,
            looping: true,
            rig_asset_path: Some(state.new_clip_asset.clone()),
            transform_tracks: Vec::new(),
            property_curves: Vec::new(),
            events: Vec::new(),
            converted_asset_path: Some(state.new_clip_asset.clone()),
            gltf_animation_index: Some(0),
        },
    );
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_clip = Some(id);
    "clip:new".clone_into(&mut state.new_clip_id);
    "New Animation".clone_into(&mut state.new_clip_name);
    Ok(())
}

fn delete_selected_animation_clip(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_clip
        .clone()
        .ok_or_else(|| "no animation clip selected".to_owned())?;
    let mut candidate = state.presentation.clone();
    candidate
        .clips
        .remove(&id)
        .ok_or_else(|| format!("missing animation clip {id}"))?;
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_clip = state.presentation.clips.keys().next().cloned();
    Ok(())
}

fn animation_controller_editor(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    clip_choices: &[(StableId, String)],
) {
    ui.heading("Animation controllers");
    ui.label(
        "Controller parameters, states, blend motions, transitions, state machines, and layers are authored and validated here.",
    );
    let controller_choices = state
        .presentation
        .controllers
        .iter()
        .map(|(id, value)| (id.clone(), value.display_name.clone()))
        .collect::<Vec<_>>();
    ui.horizontal_wrapped(|ui| {
        stable_id_option_choice(
            ui,
            "Controller",
            &mut state.selected_controller,
            &controller_choices,
        );
        ui.label("New stable ID");
        ui.text_edit_singleline(&mut state.new_controller_id);
        ui.label("Name");
        ui.text_edit_singleline(&mut state.new_controller_name);
        if ui.button("Add controller").clicked() {
            state.status = match create_animation_controller(state) {
                Ok(()) => "Added animation controller and selected it".to_owned(),
                Err(error) => format!("Controller creation rejected: {error}"),
            };
        }
        if ui
            .add_enabled(
                state.selected_controller.is_some(),
                egui::Button::new("Delete controller"),
            )
            .clicked()
        {
            state.status = match delete_selected_animation_controller(state) {
                Ok(()) => "Deleted unreferenced animation controller".to_owned(),
                Err(error) => format!("Controller deletion rejected: {error}"),
            };
        }
    });
    let Some(id) = state.selected_controller.clone() else {
        return;
    };
    let avatar_mask_choices = state
        .presentation
        .avatar_masks
        .iter()
        .map(|(id, value)| (id.clone(), value.display_name.clone()))
        .collect::<Vec<_>>();
    let Some(controller) = state.presentation.controllers.get_mut(&id) else {
        return;
    };
    ui.group(|ui| {
        ui.heading(&controller.display_name);
        ui.monospace(id.to_string());
        ui.horizontal_wrapped(|ui| {
            ui.label("Display name");
            ui.text_edit_singleline(&mut controller.display_name);
            ui.label("Source GUID");
            ui.text_edit_singleline(&mut controller.source_guid);
            ui.label("Source path");
            ui.text_edit_singleline(&mut controller.source_path);
        });
    });
    animation_parameters_editor(ui, controller, &mut state.new_animation_parameter);
    animation_states_editor(
        ui,
        controller,
        clip_choices,
        &mut state.new_controller_state_id,
    );
    animation_transitions_editor(ui, controller);
    animation_state_machines_editor(ui, controller, &mut state.new_controller_machine_id);
    animation_layers_editor(ui, controller, &avatar_mask_choices);
}

fn create_animation_controller(state: &mut ToolState) -> Result<(), String> {
    let id = StableId::new(state.new_controller_id.trim()).map_err(|error| error.to_string())?;
    if state.presentation.controllers.contains_key(&id) {
        return Err(format!("animation controller {id} already exists"));
    }
    let mut candidate = state.presentation.clone();
    candidate.controllers.insert(
        id.clone(),
        AnimationControllerDef {
            display_name: state.new_controller_name.trim().to_owned(),
            source_guid: "authored-in-bevy".to_owned(),
            source_path: "authoring://stream-town-tools".to_owned(),
            parameters: Vec::new(),
            states: BTreeMap::new(),
            transitions: Vec::new(),
            state_machines: BTreeMap::new(),
            layers: Vec::new(),
            default_states: Vec::new(),
        },
    );
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_controller = Some(id);
    "animation_controller:new".clone_into(&mut state.new_controller_id);
    "New Controller".clone_into(&mut state.new_controller_name);
    Ok(())
}

fn delete_selected_animation_controller(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_controller
        .clone()
        .ok_or_else(|| "no animation controller selected".to_owned())?;
    let mut candidate = state.presentation.clone();
    candidate
        .controllers
        .remove(&id)
        .ok_or_else(|| format!("missing animation controller {id}"))?;
    candidate.validate().map_err(|error| error.to_string())?;
    state.presentation = candidate;
    state.selected_controller = state.presentation.controllers.keys().next().cloned();
    Ok(())
}

fn animation_parameters_editor(
    ui: &mut egui::Ui,
    controller: &mut AnimationControllerDef,
    new_name: &mut String,
) {
    ui.collapsing(
        format!("Parameters ({})", controller.parameters.len()),
        |ui| {
            let mut remove = None;
            for (index, parameter) in controller.parameters.iter_mut().enumerate() {
                ui.push_id(("animation_parameter", index), |ui| {
                    ui.group(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Name");
                            ui.text_edit_singleline(&mut parameter.name);
                            animation_parameter_kind_choice(ui, &mut parameter.kind);
                            ui.checkbox(&mut parameter.inferred, "Inferred");
                            if ui.small_button("Remove").clicked() {
                                remove = Some(index);
                            }
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.add(
                                egui::DragValue::new(&mut parameter.default_float).prefix("Float "),
                            );
                            ui.add(
                                egui::DragValue::new(&mut parameter.default_integer)
                                    .prefix("Integer "),
                            );
                            ui.checkbox(&mut parameter.default_boolean, "Boolean default");
                        });
                    });
                });
            }
            if let Some(index) = remove {
                let removed = controller.parameters.remove(index).name;
                for state in controller.states.values_mut() {
                    if state.speed_parameter.as_deref() == Some(removed.as_str()) {
                        state.speed_parameter = None;
                    }
                    if state.blend_parameter.as_deref() == Some(removed.as_str()) {
                        state.blend_parameter = None;
                    }
                }
                for transition in &mut controller.transitions {
                    transition
                        .conditions
                        .retain(|condition| condition.parameter != removed);
                }
            }
            ui.horizontal_wrapped(|ui| {
                ui.text_edit_singleline(new_name);
                if ui.button("Add parameter").clicked() && !new_name.trim().is_empty() {
                    controller.parameters.push(AnimationParameterDef {
                        name: new_name.trim().to_owned(),
                        kind: AnimationParameterKind::Float,
                        default_float: 0.0,
                        default_integer: 0,
                        default_boolean: false,
                        inferred: false,
                    });
                }
            });
        },
    );
}

fn animation_parameter_kind_choice(ui: &mut egui::Ui, value: &mut AnimationParameterKind) {
    egui::ComboBox::from_id_salt(("animation_parameter_kind", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(format!("{value:?}"))
        .show_ui(ui, |ui| {
            for kind in [
                AnimationParameterKind::Float,
                AnimationParameterKind::Integer,
                AnimationParameterKind::Boolean,
                AnimationParameterKind::Trigger,
            ] {
                ui.selectable_value(value, kind, format!("{kind:?}"));
            }
        });
}

fn animation_states_editor(
    ui: &mut egui::Ui,
    controller: &mut AnimationControllerDef,
    clip_choices: &[(StableId, String)],
    new_state_id: &mut String,
) {
    ui.collapsing(format!("States ({})", controller.states.len()), |ui| {
        let state_ids = controller.states.keys().cloned().collect::<Vec<_>>();
        let parameter_names = controller
            .parameters
            .iter()
            .filter(|parameter| parameter.kind == AnimationParameterKind::Float)
            .map(|parameter| parameter.name.clone())
            .collect::<Vec<_>>();
        let mut remove_state = None;
        for state_id in state_ids {
            let Some(state) = controller.states.get_mut(&state_id) else {
                continue;
            };
            ui.push_id(("animation_state", state_id.as_str()), |ui| {
                ui.collapsing(format!("{} · {state_id}", state.display_name), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut state.display_name);
                        ui.add(egui::DragValue::new(&mut state.speed).prefix("Speed "));
                        if ui.small_button("Remove state").clicked() {
                            remove_state = Some(state_id.clone());
                        }
                    });
                    optional_string_choice(
                        ui,
                        "Speed parameter",
                        &mut state.speed_parameter,
                        &parameter_names,
                    );
                    optional_string_choice(
                        ui,
                        "Blend parameter",
                        &mut state.blend_parameter,
                        &parameter_names,
                    );
                    let mut remove_motion = None;
                    for (index, motion) in state.motions.iter_mut().enumerate() {
                        ui.horizontal_wrapped(|ui| {
                            stable_id_required_choice(ui, "Clip", &mut motion.clip, clip_choices);
                            let mut threshold = motion.threshold.is_some();
                            if ui.checkbox(&mut threshold, "Blend threshold").changed() {
                                motion.threshold = threshold.then_some(0.0);
                            }
                            if let Some(value) = motion.threshold.as_mut() {
                                ui.add(egui::DragValue::new(value));
                            }
                            if ui.small_button("Remove motion").clicked() {
                                remove_motion = Some(index);
                            }
                        });
                    }
                    if let Some(index) = remove_motion {
                        state.motions.remove(index);
                    }
                    if ui
                        .add_enabled(!clip_choices.is_empty(), egui::Button::new("Add motion"))
                        .clicked()
                        && let Some((clip, _)) = clip_choices.first()
                    {
                        state.motions.push(AnimationMotionDef {
                            clip: clip.clone(),
                            threshold: None,
                        });
                    }
                });
            });
        }
        if let Some(state_id) = remove_state {
            controller.states.remove(&state_id);
            controller.default_states.retain(|id| id != &state_id);
            for machine in controller.state_machines.values_mut() {
                machine.states.retain(|id| id != &state_id);
                if machine.default_state.as_ref() == Some(&state_id) {
                    machine.default_state = None;
                }
            }
            controller.transitions.retain(|transition| {
                transition.source.as_ref() != Some(&state_id)
                    && transition.destination.as_ref() != Some(&state_id)
            });
        }
        ui.horizontal_wrapped(|ui| {
            ui.label("New state ID");
            ui.text_edit_singleline(new_state_id);
            if ui.button("Add state").clicked()
                && let Ok(id) = StableId::new(new_state_id.trim())
            {
                controller.states.entry(id).or_insert(AnimationStateDef {
                    display_name: "New State".to_owned(),
                    speed: 1.0,
                    speed_parameter: None,
                    blend_parameter: None,
                    motions: Vec::new(),
                });
            }
        });
        let state_choices = controller
            .states
            .iter()
            .map(|(id, state)| (id.clone(), state.display_name.clone()))
            .collect::<Vec<_>>();
        stable_id_vec_choices(
            ui,
            "Controller default states",
            &mut controller.default_states,
            &state_choices,
        );
    });
}

fn animation_transitions_editor(ui: &mut egui::Ui, controller: &mut AnimationControllerDef) {
    ui.collapsing(
        format!("Transitions ({})", controller.transitions.len()),
        |ui| {
            let state_choices = controller
                .states
                .iter()
                .map(|(id, state)| (id.clone(), state.display_name.clone()))
                .collect::<Vec<_>>();
            let machine_choices = controller
                .state_machines
                .iter()
                .map(|(id, machine)| (id.clone(), machine.display_name.clone()))
                .collect::<Vec<_>>();
            let parameter_names = controller
                .parameters
                .iter()
                .map(|parameter| parameter.name.clone())
                .collect::<Vec<_>>();
            let mut remove_transition = None;
            for (index, transition) in controller.transitions.iter_mut().enumerate() {
                ui.push_id(("animation_transition", index), |ui| {
                    ui.group(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.strong(format!("Transition {}", index + 1));
                            stable_id_option_choice(
                                ui,
                                "Source",
                                &mut transition.source,
                                &state_choices,
                            );
                            stable_id_option_choice(
                                ui,
                                "Destination",
                                &mut transition.destination,
                                &state_choices,
                            );
                            if ui.small_button("Remove").clicked() {
                                remove_transition = Some(index);
                            }
                        });
                        ui.horizontal_wrapped(|ui| {
                            stable_id_option_choice(
                                ui,
                                "Source machine",
                                &mut transition.source_state_machine,
                                &machine_choices,
                            );
                            stable_id_option_choice(
                                ui,
                                "Destination machine",
                                &mut transition.destination_state_machine,
                                &machine_choices,
                            );
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.checkbox(&mut transition.is_entry, "Entry");
                            ui.checkbox(&mut transition.is_any_state, "Any state");
                            ui.checkbox(&mut transition.is_exit, "Exit");
                            ui.checkbox(&mut transition.has_exit_time, "Has exit time");
                            ui.checkbox(&mut transition.fixed_duration, "Duration in seconds");
                            ui.add(
                                egui::DragValue::new(&mut transition.exit_time)
                                    .prefix("Exit time "),
                            );
                            ui.add(
                                egui::DragValue::new(&mut transition.duration)
                                    .range(0.0..=10_000.0)
                                    .prefix("Duration "),
                            );
                            ui.add(
                                egui::DragValue::new(&mut transition.offset)
                                    .range(0.0..=10_000.0)
                                    .prefix("Offset "),
                            );
                        });
                        let mut remove_condition = None;
                        for (condition_index, condition) in
                            transition.conditions.iter_mut().enumerate()
                        {
                            ui.horizontal_wrapped(|ui| {
                                string_choice(
                                    ui,
                                    "Parameter",
                                    &mut condition.parameter,
                                    &parameter_names,
                                );
                                animation_condition_mode_choice(ui, &mut condition.mode);
                                ui.add(
                                    egui::DragValue::new(&mut condition.threshold)
                                        .prefix("Threshold "),
                                );
                                if ui.small_button("Remove condition").clicked() {
                                    remove_condition = Some(condition_index);
                                }
                            });
                        }
                        if let Some(index) = remove_condition {
                            transition.conditions.remove(index);
                        }
                        if ui
                            .add_enabled(
                                !parameter_names.is_empty(),
                                egui::Button::new("Add condition"),
                            )
                            .clicked()
                        {
                            transition.conditions.push(AnimationConditionDef {
                                parameter: parameter_names[0].clone(),
                                mode: AnimationConditionMode::Greater,
                                threshold: 0.0,
                            });
                        }
                    });
                });
            }
            if let Some(index) = remove_transition {
                controller.transitions.remove(index);
            }
            if ui
                .add_enabled(
                    !state_choices.is_empty(),
                    egui::Button::new("Add transition"),
                )
                .clicked()
            {
                let state = state_choices[0].0.clone();
                controller.transitions.push(AnimationTransitionDef {
                    source: Some(state.clone()),
                    destination: Some(state),
                    source_state_machine: None,
                    destination_state_machine: None,
                    is_entry: false,
                    is_any_state: false,
                    is_exit: false,
                    has_exit_time: true,
                    exit_time: 1.0,
                    duration: 0.1,
                    fixed_duration: true,
                    offset: 0.0,
                    conditions: Vec::new(),
                });
            }
        },
    );
}

fn animation_condition_mode_choice(ui: &mut egui::Ui, value: &mut AnimationConditionMode) {
    egui::ComboBox::from_id_salt(("animation_condition_mode", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(format!("{value:?}"))
        .show_ui(ui, |ui| {
            for mode in [
                AnimationConditionMode::If,
                AnimationConditionMode::IfNot,
                AnimationConditionMode::Greater,
                AnimationConditionMode::Less,
                AnimationConditionMode::Equals,
                AnimationConditionMode::NotEqual,
            ] {
                ui.selectable_value(value, mode, format!("{mode:?}"));
            }
        });
}

fn animation_state_machines_editor(
    ui: &mut egui::Ui,
    controller: &mut AnimationControllerDef,
    new_machine_id: &mut String,
) {
    ui.collapsing(
        format!("State machines ({})", controller.state_machines.len()),
        |ui| {
            let state_choices = controller
                .states
                .iter()
                .map(|(id, state)| (id.clone(), state.display_name.clone()))
                .collect::<Vec<_>>();
            let machine_choices = controller
                .state_machines
                .iter()
                .map(|(id, machine)| (id.clone(), machine.display_name.clone()))
                .collect::<Vec<_>>();
            let machine_ids = controller
                .state_machines
                .keys()
                .cloned()
                .collect::<Vec<_>>();
            let mut remove_machine = None;
            for id in machine_ids {
                let Some(machine) = controller.state_machines.get_mut(&id) else {
                    continue;
                };
                ui.push_id(("animation_state_machine", id.as_str()), |ui| {
                    ui.collapsing(format!("{} · {id}", machine.display_name), |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Name");
                            ui.text_edit_singleline(&mut machine.display_name);
                            if ui.small_button("Remove machine").clicked() {
                                remove_machine = Some(id.clone());
                            }
                        });
                        stable_id_vec_choices(ui, "States", &mut machine.states, &state_choices);
                        let child_choices = machine_choices
                            .iter()
                            .filter(|(choice, _)| choice != &id)
                            .cloned()
                            .collect::<Vec<_>>();
                        stable_id_vec_choices(
                            ui,
                            "Child state machines",
                            &mut machine.child_state_machines,
                            &child_choices,
                        );
                        stable_id_option_choice(
                            ui,
                            "Default state",
                            &mut machine.default_state,
                            &state_choices,
                        );
                    });
                });
            }
            if let Some(id) = remove_machine {
                controller.state_machines.remove(&id);
                controller.layers.retain(|layer| layer.state_machine != id);
                for machine in controller.state_machines.values_mut() {
                    machine.child_state_machines.retain(|child| child != &id);
                }
                controller.transitions.retain(|transition| {
                    transition.source_state_machine.as_ref() != Some(&id)
                        && transition.destination_state_machine.as_ref() != Some(&id)
                });
            }
            ui.horizontal_wrapped(|ui| {
                ui.label("New machine ID");
                ui.text_edit_singleline(new_machine_id);
                if ui.button("Add state machine").clicked()
                    && let Ok(id) = StableId::new(new_machine_id.trim())
                {
                    controller
                        .state_machines
                        .entry(id)
                        .or_insert(AnimationStateMachineDef {
                            display_name: "New State Machine".to_owned(),
                            states: Vec::new(),
                            child_state_machines: Vec::new(),
                            default_state: None,
                        });
                }
            });
        },
    );
}

fn animation_layers_editor(
    ui: &mut egui::Ui,
    controller: &mut AnimationControllerDef,
    avatar_masks: &[(StableId, String)],
) {
    ui.collapsing(format!("Layers ({})", controller.layers.len()), |ui| {
        let machine_choices = controller
            .state_machines
            .iter()
            .map(|(id, machine)| (id.clone(), machine.display_name.clone()))
            .collect::<Vec<_>>();
        let mut remove = None;
        for (index, layer) in controller.layers.iter_mut().enumerate() {
            ui.push_id(("animation_layer", index), |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut layer.display_name);
                        stable_id_required_choice(
                            ui,
                            "State machine",
                            &mut layer.state_machine,
                            &machine_choices,
                        );
                        animation_layer_blend_choice(ui, &mut layer.blend_mode);
                        ui.add(
                            egui::Slider::new(&mut layer.default_weight, 0.0..=1.0).text("Weight"),
                        );
                        if ui.small_button("Remove").clicked() {
                            remove = Some(index);
                        }
                    });
                    stable_id_option_choice(
                        ui,
                        "Avatar mask",
                        &mut layer.avatar_mask,
                        avatar_masks,
                    );
                });
            });
        }
        if let Some(index) = remove {
            controller.layers.remove(index);
        }
        if ui
            .add_enabled(
                !machine_choices.is_empty(),
                egui::Button::new("Add animation layer"),
            )
            .clicked()
        {
            controller.layers.push(AnimationLayerDef {
                display_name: "New Layer".to_owned(),
                state_machine: machine_choices[0].0.clone(),
                blend_mode: AnimationLayerBlendMode::Override,
                default_weight: 1.0,
                avatar_mask: None,
            });
        }
    });
}

fn animation_layer_blend_choice(ui: &mut egui::Ui, value: &mut AnimationLayerBlendMode) {
    egui::ComboBox::from_id_salt(("animation_layer_blend", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(format!("{value:?}"))
        .show_ui(ui, |ui| {
            ui.selectable_value(value, AnimationLayerBlendMode::Override, "Override");
            ui.selectable_value(value, AnimationLayerBlendMode::Additive, "Additive");
        });
}

fn animation_transform_tracks_editor(
    ui: &mut egui::Ui,
    tracks: &mut Vec<AnimationTransformTrack>,
    duration: f32,
) {
    ui.collapsing(format!("Transform tracks ({})", tracks.len()), |ui| {
        let mut remove = None;
        for (index, track) in tracks.iter_mut().enumerate() {
            ui.push_id(("animation_track", index), |ui| {
                ui.collapsing(
                    if track.target_path.is_empty() {
                        format!("Track {} · root", index + 1)
                    } else {
                        format!("Track {} · {}", index + 1, track.target_path)
                    },
                    |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Hierarchy path");
                            ui.text_edit_singleline(&mut track.target_path);
                            if ui.small_button("Remove track").clicked() {
                                remove = Some(index);
                            }
                        });
                        animation_vec3_keys_editor(
                            ui,
                            "Translation",
                            &mut track.translation,
                            duration,
                            [0.0; 3],
                        );
                        animation_quat_keys_editor(ui, &mut track.rotation, duration);
                        animation_vec3_keys_editor(
                            ui,
                            "Scale",
                            &mut track.scale,
                            duration,
                            [1.0; 3],
                        );
                        animation_vec3_keys_editor(
                            ui,
                            "Euler degrees",
                            &mut track.euler_degrees,
                            duration,
                            [0.0; 3],
                        );
                    },
                );
            });
        }
        if let Some(index) = remove {
            tracks.remove(index);
        }
        if ui.button("Add transform track").clicked() {
            tracks.push(AnimationTransformTrack {
                target_path: String::new(),
                reference_translation: None,
                reference_rotation: None,
                reference_scale: None,
                translation: Vec::new(),
                rotation: Vec::new(),
                scale: Vec::new(),
                euler_degrees: Vec::new(),
            });
        }
    });
}

fn animation_vec3_keys_editor(
    ui: &mut egui::Ui,
    label: &str,
    keys: &mut Vec<AnimationVec3Keyframe>,
    duration: f32,
    default_value: [f32; 3],
) {
    ui.collapsing(format!("{label} keys ({})", keys.len()), |ui| {
        let mut remove = None;
        for (index, key) in keys.iter_mut().enumerate() {
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    egui::DragValue::new(&mut key.time)
                        .range(0.0..=duration.max(0.001))
                        .prefix("t "),
                );
                for (component, axis) in key.value.iter_mut().zip(["x", "y", "z"]) {
                    ui.add(egui::DragValue::new(component).prefix(format!("{axis} ")));
                }
                if ui.small_button("Remove").clicked() {
                    remove = Some(index);
                }
            });
        }
        if let Some(index) = remove {
            keys.remove(index);
        }
        if ui.button(format!("Add {label} key")).clicked() {
            keys.push(AnimationVec3Keyframe {
                time: duration.max(0.0),
                value: default_value,
            });
        }
    });
}

fn animation_quat_keys_editor(
    ui: &mut egui::Ui,
    keys: &mut Vec<AnimationQuatKeyframe>,
    duration: f32,
) {
    ui.collapsing(format!("Rotation keys ({})", keys.len()), |ui| {
        let mut remove = None;
        for (index, key) in keys.iter_mut().enumerate() {
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    egui::DragValue::new(&mut key.time)
                        .range(0.0..=duration.max(0.001))
                        .prefix("t "),
                );
                for (component, axis) in key.value.iter_mut().zip(["x", "y", "z", "w"]) {
                    ui.add(egui::DragValue::new(component).prefix(format!("{axis} ")));
                }
                if ui.small_button("Remove").clicked() {
                    remove = Some(index);
                }
            });
        }
        if let Some(index) = remove {
            keys.remove(index);
        }
        if ui.button("Add rotation key").clicked() {
            keys.push(AnimationQuatKeyframe {
                time: duration.max(0.0),
                value: [0.0, 0.0, 0.0, 1.0],
            });
        }
    });
}

fn animation_property_curves_editor(
    ui: &mut egui::Ui,
    curves: &mut Vec<AnimationPropertyCurve>,
    duration: f32,
) {
    ui.collapsing(format!("Property curves ({})", curves.len()), |ui| {
        let mut remove_curve = None;
        for (index, curve) in curves.iter_mut().enumerate() {
            ui.push_id(("property_curve", index), |ui| {
                ui.collapsing(
                    format!("{} :: {}", curve.target_path, curve.attribute),
                    |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Target");
                            ui.text_edit_singleline(&mut curve.target_path);
                            ui.label("Attribute");
                            ui.text_edit_singleline(&mut curve.attribute);
                            ui.add(egui::DragValue::new(&mut curve.class_id).prefix("Class "));
                            if ui.small_button("Remove curve").clicked() {
                                remove_curve = Some(index);
                            }
                        });
                        let mut remove_key = None;
                        for (key_index, key) in curve.keys.iter_mut().enumerate() {
                            ui.horizontal_wrapped(|ui| {
                                ui.add(
                                    egui::DragValue::new(&mut key.time)
                                        .range(0.0..=duration.max(0.001))
                                        .prefix("t "),
                                );
                                ui.add(egui::DragValue::new(&mut key.value).prefix("value "));
                                if ui.small_button("Remove").clicked() {
                                    remove_key = Some(key_index);
                                }
                            });
                        }
                        if let Some(key_index) = remove_key {
                            curve.keys.remove(key_index);
                        }
                        if ui.button("Add property key").clicked() {
                            curve.keys.push(default_float_key(duration.max(0.0), 0.0));
                        }
                    },
                );
            });
        }
        if let Some(index) = remove_curve {
            curves.remove(index);
        }
        if ui.button("Add property curve").clicked() {
            curves.push(AnimationPropertyCurve {
                target_path: String::new(),
                attribute: "m_Enabled".to_owned(),
                class_id: 1,
                script_guid: None,
                keys: vec![default_float_key(0.0, 0.0)],
            });
        }
    });
}

fn default_float_key(time: f32, value: f32) -> AnimationFloatKeyframe {
    AnimationFloatKeyframe {
        time,
        value,
        in_slope: AnimationTangent::Finite(0.0),
        out_slope: AnimationTangent::Finite(0.0),
        tangent_mode: 0,
        weighted_mode: 0,
        in_weight: 1.0 / 3.0,
        out_weight: 1.0 / 3.0,
    }
}

fn animation_events_editor(ui: &mut egui::Ui, events: &mut Vec<AnimationEventDef>, duration: f32) {
    ui.collapsing(format!("Animation events ({})", events.len()), |ui| {
        let mut remove = None;
        for (index, event) in events.iter_mut().enumerate() {
            ui.push_id(("animation_event", index), |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut event.time)
                                .range(0.0..=duration.max(0.001))
                                .prefix("Time "),
                        );
                        ui.label("Function");
                        ui.text_edit_singleline(&mut event.function_name);
                        if ui.small_button("Remove").clicked() {
                            remove = Some(index);
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.label("String");
                        ui.text_edit_singleline(&mut event.string_parameter);
                        ui.add(egui::DragValue::new(&mut event.float_parameter).prefix("Float "));
                        ui.add(egui::DragValue::new(&mut event.int_parameter).prefix("Int "));
                        ui.add(
                            egui::DragValue::new(&mut event.message_options)
                                .prefix("Message options "),
                        );
                    });
                });
            });
        }
        if let Some(index) = remove {
            events.remove(index);
        }
        if ui.button("Add animation event").clicked() {
            events.push(AnimationEventDef {
                time: duration.max(0.0),
                function_name: "OnAnimationEvent".to_owned(),
                string_parameter: String::new(),
                object_reference: None,
                float_parameter: 0.0,
                int_parameter: 0,
                message_options: 0,
            });
        }
    });
}
