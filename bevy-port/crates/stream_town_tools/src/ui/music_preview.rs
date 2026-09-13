fn music_tab(
    ui: &mut egui::Ui,
    state: &mut ToolState,
    tidal: &TidalController,
    tidal_backend: &TidalBackendStatus,
    native_audio: &NativeAudioStatus,
    audio_routing: &NativeAudioRouting,
) {
    ui.heading("Adaptive Bevy Tidal score");
    ui.label(
        "This editor and preview use the same score renderer as the live world. The tab initially loads the live runtime song when one exists.",
    );
    let preview_ready = tidal_backend.is_ready() && native_audio.state() == NativeAudioState::Ready;
    ui.horizontal_wrapped(|ui| {
        if ui.button("Load saved song").clicked() {
            state.status = match load_game_config(&state.config_path) {
                Ok(config) => {
                    state.config.music = config.music;
                    "Loaded the project song".to_owned()
                }
                Err(error) => format!("Could not load the project song: {error:#}"),
            };
        }
        if ui.button("Load live song").clicked() {
            state.status = match stream_town_game::load_runtime_config() {
                Ok(config) => {
                    state.config.music = config.music;
                    "Loaded the song currently configured for the live game".to_owned()
                }
                Err(error) => format!("Could not load the live song: {error:#}"),
            };
        }
        if ui.button("Save song").clicked() {
            state.status = match save_and_apply_game_config(&state.config, &state.config_path) {
                Ok((project, runtime)) => format!(
                    "Saved the song to {} and {}; restart the game to apply it to the world",
                    project.display(),
                    runtime.display()
                ),
                Err(error) => format!("Could not save the song: {error:#}"),
            };
        }
        if ui
            .add_enabled(preview_ready, egui::Button::new("Start preview"))
            .clicked()
        {
            state.status = match stream_town_game::adaptive_music_preview_program(
                &state.config.music,
                state.music_preview_intensity,
                state.music_preview_season,
                state.music_preview_time_of_day,
                state.music_preview_population,
                state.music_preview_building_count,
            )
            .and_then(|(expression, cycles_per_second)| {
                tidal
                    .transition(TOOL_MUSIC_PREVIEW_TRACK, &expression, cycles_per_second)
                    .map(|()| (expression, cycles_per_second))
            }) {
                Ok((expression, cycles_per_second)) => {
                    audio_routing.set_local_monitor_enabled(true);
                    audio_routing.set_master_gain(1.0);
                    state.music_preview_state = MusicPreviewState::Playing;
                    state.music_preview_rendered_score = expression;
                    format!(
                        "Playing the exact world score preview at {:.2} cycles/minute",
                        cycles_per_second * 60.0
                    )
                }
                Err(error) => {
                    state.music_preview_state = MusicPreviewState::Stopped;
                    format!("Could not start the song preview: {error}")
                }
            };
        }
        if ui
            .add_enabled(
                state.music_preview_state == MusicPreviewState::Playing,
                egui::Button::new("Stop preview"),
            )
            .clicked()
        {
            state.status = match tidal.silence(TOOL_MUSIC_PREVIEW_TRACK) {
                Ok(()) => {
                    state.music_preview_state = MusicPreviewState::Stopped;
                    "Stopped the song preview".to_owned()
                }
                Err(error) => format!("Could not stop the song preview: {error}"),
            };
        }
    });
    if preview_ready {
        ui.colored_label(
            egui::Color32::LIGHT_GREEN,
            if state.music_preview_state == MusicPreviewState::Playing {
                "Native music preview: playing"
            } else {
                "Native music preview: ready"
            },
        );
    } else {
        ui.colored_label(
            egui::Color32::LIGHT_YELLOW,
            format!(
                "Music preview is starting: {} | {}",
                tidal_backend.detail(),
                native_audio.detail()
            ),
        );
    }
    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading("Preview town state");
        ui.label("These values replace the corresponding live-world inputs while auditioning.");
        ui.add(
            egui::Slider::new(
                &mut state.music_preview_intensity,
                0.0..=f64::from(state.config.music.maximum_energy.max(0.1)),
            )
            .text("Enemy intensity"),
        );
        ui.add(
            egui::Slider::new(&mut state.music_preview_season, 0.0..=3.0)
                .text("Season (0 spring .. 3 winter)"),
        );
        ui.add(
            egui::Slider::new(&mut state.music_preview_time_of_day, 0.0..=1.0)
                .text("Time of day"),
        );
        ui.horizontal(|ui| {
            ui.add(
                egui::DragValue::new(&mut state.music_preview_population)
                    .range(0..=10_000)
                    .prefix("Population "),
            );
            ui.add(
                egui::DragValue::new(&mut state.music_preview_building_count)
                    .range(0..=10_000)
                    .prefix("Buildings "),
            );
        });
        if !state.music_preview_rendered_score.is_empty() {
            ui.collapsing("Rendered world score sent to the preview", |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut state.music_preview_rendered_score)
                        .code_editor()
                        .interactive(false)
                        .desired_rows(10)
                        .desired_width(f32::INFINITY),
                );
            });
        }
        ui.separator();
        let music = &mut state.config.music;
        ui.add(
            egui::DragValue::new(&mut music.intensity_smoothing_seconds)
                .range(0.05..=120.0)
                .speed(0.1)
                .prefix("Enemy intensity smoothing ")
                .suffix(" seconds"),
        );
        ui.add(
            egui::DragValue::new(&mut music.maximum_energy)
                .range(0.1..=1_000.0)
                .speed(0.25)
                .prefix("Maximum energy "),
        );
        ui.horizontal(|ui| {
            ui.add(
                egui::DragValue::new(&mut music.cycles_per_minute_base)
                    .range(1.0..=1_000.0)
                    .prefix("Base CPM "),
            );
            ui.add(
                egui::DragValue::new(&mut music.cycles_per_minute_per_energy)
                    .range(-100.0..=100.0)
                    .speed(0.1)
                    .prefix("CPM / energy "),
            );
        });
        ui.separator();
        egui::Grid::new("adaptive_music_variables")
            .striped(true)
            .show(ui, |ui| {
                ui.strong("Live variable");
                ui.strong("Template token");
                ui.strong("Energy weight");
                ui.end_row();
                for (name, token, weight) in [
                    ("Enemies on screen (smoothed)", "${intensity}", &mut music.intensity_weight),
                    ("Season (0 spring .. 3 winter)", "${season}", &mut music.season_weight),
                    ("Time of day (0 .. 1)", "${time_of_day}", &mut music.time_of_day_weight),
                    ("Living citizen population", "${population}", &mut music.population_weight),
                    ("Completed building count", "${building_count}", &mut music.building_count_weight),
                ] {
                    ui.label(name);
                    ui.monospace(token);
                    ui.add(egui::DragValue::new(weight).speed(0.01));
                    ui.end_row();
                }
                ui.label("Weighted/clamped result");
                ui.monospace("${energy}");
                ui.label("—");
                ui.end_row();
            });
        ui.separator();
        ui.label("Tidal score template");
        ui.add(
            egui::TextEdit::multiline(&mut music.score_template)
                .code_editor()
                .desired_rows(24)
                .desired_width(f32::INFINITY),
        );
        ui.small(
            "Generated voice placeholders such as ${kick}, ${roots}, ${melody_attack}, and ${hat_gain} remain available. Unresolved placeholders are rejected at runtime.",
        );
    });
}

fn player_animation_controller(
    presentation: &PresentationCatalog,
) -> Option<&AnimationControllerDef> {
    let binding = presentation.prefab_bindings.values().find(|binding| {
        binding
            .source_prefab_path
            .ends_with("Player_Character.prefab")
    })?;
    presentation.controllers.get(&binding.controller)
}

fn role_preview_animation_choices(presentation: &PresentationCatalog) -> Vec<(StableId, String)> {
    player_animation_controller(presentation).map_or_else(Vec::new, |controller| {
        controller
            .states
            .iter()
            .filter(|(_, state)| !state.motions.is_empty())
            .map(|(id, state)| (id.clone(), state.display_name.clone()))
            .collect()
    })
}

fn matching_role_animation_state(
    presentation: &PresentationCatalog,
    action_animation: &str,
) -> Option<StableId> {
    let controller = player_animation_controller(presentation)?;
    controller
        .states
        .iter()
        .find(|(_, state)| {
            state.display_name == action_animation
                || state
                    .display_name
                    .strip_suffix(" 0")
                    .is_some_and(|name| name == action_animation)
        })
        .or_else(|| {
            controller
                .states
                .iter()
                .find(|(_, state)| state.display_name == "Locomotion")
        })
        .map(|(id, _)| id.clone())
}

fn default_role_preview_animation(
    presentation: &PresentationCatalog,
    catalog: &ContentCatalog,
) -> Option<StableId> {
    catalog
        .roles
        .values()
        .next()
        .and_then(|role| matching_role_animation_state(presentation, &role.action_animation))
}

fn role_preview_animation_request(
    presentation: &PresentationCatalog,
    state_id: &StableId,
) -> Option<(String, u32, bool)> {
    let state = player_animation_controller(presentation)?
        .states
        .get(state_id)?;
    let motion = state.motions.iter().max_by(|left, right| {
        left.threshold
            .unwrap_or_default()
            .total_cmp(&right.threshold.unwrap_or_default())
    })?;
    let source = presentation.clips.get(&motion.clip)?;
    let (path, index) =
        preview_animation_asset_for_rig(source, PLAYER_ANIMATED_MODEL_PATH, presentation)?;
    Some((path, index, source.looping))
}

fn role_preview_visible_nodes(
    role: &RoleDef,
    body_type: u8,
    carrying: bool,
    eye: Option<&str>,
    hair: Option<&str>,
    facial_hair: Option<&str>,
) -> BTreeSet<String> {
    let mut visible = BTreeSet::new();
    let helmet_equipped = role
        .equipment
        .as_ref()
        .and_then(|equipment| equipment.helmet_node.as_ref())
        .is_some();
    if let Some(equipment) = role.equipment.as_ref() {
        visible.insert(
            equipment.body_nodes[usize::from(body_type).min(equipment.body_nodes.len() - 1)]
                .clone(),
        );
        visible.extend(equipment.right_hand_node.iter().cloned());
        visible.extend(equipment.helmet_node.iter().cloned());
        if equipment.left_hand_permanent || carrying {
            visible.extend(equipment.left_hand_node.iter().cloned());
        }
    } else {
        visible.insert(
            [
                "Body_Default_Slim",
                "Body_Default_Bulk",
                "Body_Default_Feminine",
            ][usize::from(body_type).min(2)]
            .to_owned(),
        );
    }
    visible.extend(eye.map(str::to_owned));
    if !helmet_equipped {
        visible.extend(hair.map(str::to_owned));
    }
    visible.extend(facial_hair.map(str::to_owned));
    visible
        .into_iter()
        .map(|name| canonical_preview_node_name(&name).to_owned())
        .collect()
}

fn update_preview_request(state: &mut ToolState) {
    let requested = match state.tab {
        ToolTab::Assets => match state.asset_section {
            AssetEditorSection::Models => state
                .selected_archetype
                .as_ref()
                .and_then(|id| state.catalog.archetypes.get(id))
                .and_then(|archetype| {
                    let index = state
                        .selected_archetype_scene
                        .min(archetype.scenes.len().saturating_sub(1));
                    archetype.scenes.get(index).map(|scene| {
                        (
                            PreviewRequest::Model(scene.asset_path.clone()),
                            format!("{} · {}", archetype.display_name, scene.asset_path),
                        )
                    })
                }),
            AssetEditorSection::Textures => state.selected_texture.as_ref().and_then(|id| {
                state.presentation.textures.get(id).map(|texture| {
                    (
                        PreviewRequest::Texture {
                            id: id.clone(),
                            fingerprint: debug_fingerprint(texture),
                        },
                        format!("{} · {}", texture.display_name, texture.asset_path),
                    )
                })
            }),
            AssetEditorSection::Materials => state.selected_material.as_ref().and_then(|id| {
                state.presentation.materials.get(id).map(|material| {
                    (
                        PreviewRequest::Material {
                            id: id.clone(),
                            fingerprint: debug_fingerprint(material),
                        },
                        format!("{} · {id}", material.display_name),
                    )
                })
            }),
            AssetEditorSection::Animations => state.selected_clip.as_ref().and_then(|id| {
                let clip = state.presentation.clips.get(id)?;
                let (asset_path, animation_index) = preview_animation_asset_for_rig(
                    clip,
                    PLAYER_ANIMATED_MODEL_PATH,
                    &state.presentation,
                )
                .or_else(|| {
                    Some((
                        clip.converted_asset_path
                            .as_ref()
                            .or(clip.rig_asset_path.as_ref())?
                            .clone(),
                        clip.gltf_animation_index?,
                    ))
                })?;
                Some((
                    PreviewRequest::Animation {
                        clip: id.clone(),
                        asset_path,
                        animation_index,
                        fingerprint: debug_fingerprint(clip),
                    },
                    format!("{} · {id}", clip.display_name),
                ))
            }),
        },
        ToolTab::Buildings => state
            .building_draft
            .as_ref()
            .and_then(|draft| state.catalog.archetypes.get(&draft.value.archetype))
            .and_then(|archetype| {
                archetype
                    .scenes
                    .iter()
                    .find(|scene| scene.is_default)
                    .or_else(|| archetype.scenes.first())
                    .map(|scene| {
                        (
                            PreviewRequest::Model(scene.asset_path.clone()),
                            format!("{} · {}", archetype.display_name, scene.asset_path),
                        )
                    })
            }),
        ToolTab::Roles => state.role_draft.as_ref().and_then(|draft| {
            let animation_state = state.role_preview_animation.clone().or_else(|| {
                matching_role_animation_state(&state.presentation, &draft.value.action_animation)
            })?;
            let (asset_path, animation_index, looping) =
                role_preview_animation_request(&state.presentation, &animation_state)?;
            let visible_nodes = role_preview_visible_nodes(
                &draft.value,
                state.role_preview_body_type,
                state.role_preview_carrying,
                state.role_preview_eye.as_deref(),
                state.role_preview_hair.as_deref(),
                state.role_preview_facial_hair.as_deref(),
            );
            let fingerprint = debug_fingerprint(&(
                &draft.value,
                &animation_state,
                &visible_nodes,
                state.role_preview_body_type,
                state.role_preview_carrying,
            ));
            Some((
                PreviewRequest::Role {
                    role: draft.id.clone(),
                    asset_path,
                    animation_index,
                    looping,
                    visible_nodes,
                    fingerprint,
                },
                format!("{} · composed character", draft.value.display_name),
            ))
        }),
        ToolTab::World
            if matches!(
                state.world_preview_layer,
                WorldPreviewLayer::LandResources | WorldPreviewLayer::WaterResources
            ) =>
        {
            state.resource_generation_draft.as_ref().map(|layer| {
                (
                    PreviewRequest::Model(layer.visual_asset_path.clone()),
                    format!("Resource · {}", layer.display_name),
                )
            })
        }
        ToolTab::World if state.world_preview_layer == WorldPreviewLayer::EnemyCamps => state
            .enemy_camp_generation_draft
            .as_ref()
            .and_then(|layer| state.catalog.archetypes.get(&layer.camp_archetype))
            .and_then(|archetype| {
                archetype
                    .scenes
                    .iter()
                    .find(|scene| scene.is_default)
                    .or_else(|| archetype.scenes.first())
                    .map(|scene| {
                        (
                            PreviewRequest::Model(scene.asset_path.clone()),
                            format!("Enemy camp · {}", archetype.display_name),
                        )
                    })
            }),
        ToolTab::World => state
            .foliage_draft
            .as_ref()
            .and_then(|layer| layer.variants.get(state.selected_foliage_variant))
            .map(|variant| {
                (
                    PreviewRequest::Model(variant.asset_path.clone()),
                    format!("Foliage · {}", variant.asset_path),
                )
            }),
        _ => None,
    };
    if let Some((request, label)) = requested {
        state.preview_request = Some(request);
        state.preview_label = label;
    } else if matches!(
        state.tab,
        ToolTab::Assets | ToolTab::Buildings | ToolTab::Roles | ToolTab::World
    ) {
        state.preview_request = None;
        "Nothing previewable is assigned".clone_into(&mut state.preview_label);
    }
}

fn debug_fingerprint(value: &impl std::fmt::Debug) -> u64 {
    let mut hasher = DefaultHasher::new();
    format!("{value:?}").hash(&mut hasher);
    hasher.finish()
}

fn draw_model_preview(
    ui: &mut egui::Ui,
    texture: Option<egui::TextureId>,
    status: &str,
    desired: egui::Vec2,
    controls: &mut ModelPreviewControls,
    animation_controls: bool,
) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.strong("Live 3D preview");
            if ui.small_button("Reset view").clicked() {
                controls.reset_view();
            }
            ui.small("drag: orbit · Shift/right-drag: pan · wheel: zoom");
        });
        let response = if let Some(texture) = texture {
            ui.add(
                egui::Image::new(egui::load::SizedTexture::new(texture, desired))
                    .sense(egui::Sense::click_and_drag()),
            )
        } else {
            let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click_and_drag());
            ui.painter()
                .rect_filled(rect, 5.0, egui::Color32::from_rgb(9, 14, 19));
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Preview render target unavailable",
                egui::FontId::proportional(13.0),
                egui::Color32::LIGHT_RED,
            );
            response
        };
        if response.double_clicked() {
            controls.reset_view();
        }
        if response.hovered() {
            let (delta, scroll, shift) = ui.input(|input| {
                (
                    input.pointer.delta(),
                    input.smooth_scroll_delta.y,
                    input.modifiers.shift,
                )
            });
            if response.dragged_by(egui::PointerButton::Primary) && !shift {
                controls.yaw = (controls.yaw - delta.x * 0.012).rem_euclid(std::f32::consts::TAU);
                controls.pitch = (controls.pitch + delta.y * 0.01).clamp(-1.35, 1.35);
            }
            if response.dragged_by(egui::PointerButton::Secondary)
                || (response.dragged_by(egui::PointerButton::Primary) && shift)
            {
                let pan_scale = controls.distance * 0.0028;
                controls.pan +=
                    preview_pan_delta(controls.yaw, controls.pitch, delta.x, delta.y, pan_scale);
            }
            if scroll.abs() > f32::EPSILON {
                controls.distance =
                    (controls.distance * (-scroll * 0.0015).exp()).clamp(2.25, 30.0);
            }
        }
        if animation_controls {
            ui.horizontal_wrapped(|ui| {
                if ui
                    .button(if controls.animation_playing {
                        "Pause"
                    } else {
                        "Play"
                    })
                    .clicked()
                {
                    controls.animation_playing = !controls.animation_playing;
                }
                if ui.button("Restart").clicked() {
                    controls.animation_playing = true;
                    controls.restart_animation = true;
                }
                ui.checkbox(&mut controls.animation_looping, "Loop");
                ui.add(
                    egui::Slider::new(&mut controls.animation_speed, 0.05..=3.0)
                        .logarithmic(true)
                        .text("Speed"),
                );
            });
        }
        ui.small(status);
    });
}

fn discover_model_assets() -> Vec<String> {
    fn visit(root: &std::path::Path, current: &std::path::Path, output: &mut Vec<String>) {
        let Ok(entries) = fs::read_dir(current) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, output);
            } else if path
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
                && let Ok(relative) = path.strip_prefix(root)
            {
                output.push(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    let mut output = Vec::new();
    visit(&root, &root, &mut output);
    output.sort();
    output.dedup();
    output
}

fn discover_texture_assets() -> Vec<String> {
    fn visit(root: &std::path::Path, current: &std::path::Path, output: &mut Vec<String>) {
        let Ok(entries) = fs::read_dir(current) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, output);
            } else if path.extension().is_some_and(|extension| {
                matches!(
                    extension.to_string_lossy().to_ascii_lowercase().as_str(),
                    "png" | "tga" | "jpg" | "jpeg"
                )
            }) && let Ok(relative) = path.strip_prefix(root)
            {
                let relative = relative.to_string_lossy().replace('\\', "/");
                if relative.starts_with("shipping/textures/") {
                    output.push(relative);
                }
            }
        }
    }

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    let mut output = Vec::new();
    visit(&root, &root, &mut output);
    output.sort();
    output.dedup();
    output
}

fn inspect_gltf_asset(asset_path: &str) -> Result<GltfMetadata, String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join(asset_path);
    let document = gltf::Gltf::open(&path)
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
    let mut nodes = document
        .nodes()
        .filter_map(|node| node.name().map(ToOwned::to_owned))
        .collect::<Vec<_>>();
    let mut materials = document
        .materials()
        .enumerate()
        .map(|(index, material)| {
            material
                .name()
                .map_or_else(|| format!("Material {index}"), ToOwned::to_owned)
        })
        .collect::<Vec<_>>();
    let animations = document
        .animations()
        .enumerate()
        .map(|(index, animation)| {
            (
                index,
                animation
                    .name()
                    .map_or_else(|| format!("Animation {index}"), ToOwned::to_owned),
            )
        })
        .collect::<Vec<_>>();
    nodes.sort();
    nodes.dedup();
    materials.sort();
    materials.dedup();
    Ok(GltfMetadata {
        nodes,
        materials,
        animations,
    })
}

fn cached_gltf_metadata(state: &mut ToolState, asset_path: &str) -> GltfMetadata {
    if let Some(value) = state.gltf_metadata.get(asset_path) {
        return value.clone();
    }
    match inspect_gltf_asset(asset_path) {
        Ok(value) => {
            state
                .gltf_metadata
                .insert(asset_path.to_owned(), value.clone());
            value
        }
        Err(error) => {
            state.status = error;
            GltfMetadata::default()
        }
    }
}

fn import_model_asset(state: &mut ToolState) -> Result<String, String> {
    let source_text = state.model_import_source.trim();
    if source_text.is_empty() {
        return Err("choose a source .glb file".to_owned());
    }
    let source = fs::canonicalize(source_text)
        .map_err(|error| format!("could not read {source_text}: {error}"))?;
    if !source.is_file()
        || !source
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
    {
        return Err("the source must be an existing .glb file".to_owned());
    }
    let requested_name = state.model_import_name.trim();
    let file_name = if requested_name.is_empty() {
        source
            .file_name()
            .ok_or_else(|| "the source has no filename".to_owned())?
            .to_owned()
    } else {
        let candidate = std::path::Path::new(requested_name);
        if candidate.file_name() != Some(candidate.as_os_str()) {
            return Err("destination name must be a filename, not a path".to_owned());
        }
        candidate.as_os_str().to_owned()
    };
    if !std::path::Path::new(&file_name)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
    {
        return Err("destination name must end in .glb".to_owned());
    }

    let relative = PathBuf::from("user/models").join(&file_name);
    let destination = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join(&relative);
    if destination.exists() {
        return Err(format!(
            "{} already exists; choose another destination name",
            destination.display()
        ));
    }
    let parent = destination
        .parent()
        .ok_or_else(|| "destination has no parent directory".to_owned())?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    fs::copy(&source, &destination)
        .map_err(|error| format!("could not copy model into the project: {error}"))?;

    state.discovered_model_assets = discover_model_assets();
    let relative = relative.to_string_lossy().replace('\\', "/");
    relative.clone_into(&mut state.new_archetype_asset);
    state.model_import_source.clear();
    state.model_import_name.clear();
    Ok(relative)
}

fn import_texture_asset(state: &mut ToolState) -> Result<String, String> {
    let source_text = state.texture_import_source.trim();
    if source_text.is_empty() {
        return Err("choose a source PNG, TGA, JPG, or JPEG file".to_owned());
    }
    let source = fs::canonicalize(source_text)
        .map_err(|error| format!("could not read {source_text}: {error}"))?;
    let extension = source
        .extension()
        .map(|value| value.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    if !source.is_file() || !matches!(extension.as_str(), "png" | "tga" | "jpg" | "jpeg") {
        return Err("the source must be a supported image file".to_owned());
    }
    let requested_name = state.texture_import_name.trim();
    let file_name = if requested_name.is_empty() {
        source
            .file_name()
            .ok_or_else(|| "the source has no filename".to_owned())?
            .to_owned()
    } else {
        let candidate = std::path::Path::new(requested_name);
        if candidate.file_name() != Some(candidate.as_os_str()) {
            return Err("destination name must be a filename, not a path".to_owned());
        }
        candidate.as_os_str().to_owned()
    };
    let output_extension = std::path::Path::new(&file_name)
        .extension()
        .map(|value| value.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    if !matches!(output_extension.as_str(), "png" | "tga" | "jpg" | "jpeg") {
        return Err("destination name must retain a supported image extension".to_owned());
    }
    let relative = PathBuf::from("shipping/textures/user").join(&file_name);
    let destination = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join(&relative);
    if destination.exists() {
        return Err(format!(
            "{} already exists; choose another destination name",
            destination.display()
        ));
    }
    let parent = destination
        .parent()
        .ok_or_else(|| "destination has no parent directory".to_owned())?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    fs::copy(&source, &destination)
        .map_err(|error| format!("could not copy texture into the project: {error}"))?;
    state.discovered_texture_assets = discover_texture_assets();
    let relative = relative.to_string_lossy().replace('\\', "/");
    relative.clone_into(&mut state.new_texture_asset);
    state.texture_import_source.clear();
    state.texture_import_name.clear();
    Ok(relative)
}
