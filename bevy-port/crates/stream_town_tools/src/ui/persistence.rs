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
    let busy = state.tool_job_events.is_some();
    ui.horizontal(|ui| {
        if ui
            .add_enabled(!busy, egui::Button::new("Run repository validation"))
            .clicked()
        {
            start_xtask_job(state, "Validation", ["validate"]);
        }
        if ui
            .add_enabled(!busy, egui::Button::new("Build Windows release package"))
            .clicked()
        {
            start_xtask_job(state, "Windows package", ["package-windows"]);
        }
    });
    ui.label("Checks include stable IDs, dangling references, technology cycles, GLB hashes/headers, and deterministic baselines.");
    ui.label("Release packaging builds optimized game/tools executables, includes only runtime assets plus GPL attribution, validates the archive, and writes dist/stream-town-windows-x86_64.zip.");
}

fn poll_tool_job_events(state: &mut ToolState) {
    let event = state
        .tool_job_events
        .as_ref()
        .and_then(|receiver| receiver.lock().ok())
        .and_then(|receiver| receiver.try_recv().ok());
    match event {
        Some(ToolJobEvent::Finished(message) | ToolJobEvent::Error(message)) => {
            state.status = message;
            state.tool_job_events = None;
        }
        None => {}
    }
}

fn start_xtask_job<const N: usize>(
    state: &mut ToolState,
    label: &'static str,
    args: [&'static str; N],
) {
    let (sender, receiver) = mpsc::channel();
    state.tool_job_events = Some(Arc::new(Mutex::new(receiver)));
    state.status = format!("{label} is running...");
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let worker = thread::Builder::new()
        .name("stream-town-tools-xtask".to_owned())
        .spawn(move || {
            let outcome = Command::new("cargo")
                .current_dir(&workspace)
                .arg("run")
                .arg("-p")
                .arg("xtask")
                .arg("--")
                .args(args)
                .output();
            let event = match outcome {
                Ok(output) if output.status.success() => {
                    let summary = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .last()
                        .unwrap_or("completed successfully")
                        .to_owned();
                    ToolJobEvent::Finished(format!("{label} complete: {summary}"))
                }
                Ok(output) => {
                    let error = String::from_utf8_lossy(&output.stderr)
                        .lines()
                        .last()
                        .unwrap_or("unknown error")
                        .to_owned();
                    ToolJobEvent::Error(format!("{label} failed: {error}"))
                }
                Err(error) => ToolJobEvent::Error(format!("Could not start {label}: {error}")),
            };
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not start {label} worker: {error}");
        state.tool_job_events = None;
    }
}

fn technology_draft(catalog: &ContentCatalog, id: &StableId) -> Option<TechnologyDraft> {
    let node = catalog.technology.nodes.get(id)?;
    Some(TechnologyDraft {
        id: id.clone(),
        value: node.clone(),
        prerequisite_search: String::new(),
        unlock_search: String::new(),
        objective_search: String::new(),
        icon_search: String::new(),
    })
}

fn default_catalog_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content/catalog.ron")
}

fn default_presentation_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content/presentation.ron")
}

fn default_technology_layout_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content/technology_layout.ron")
}

fn default_config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/config/game.ron")
}

fn validate_authoring_assets() -> anyhow::Result<String> {
    let config = load_game_config(default_config_path().to_string_lossy().as_ref())?;
    let catalog = load_content_catalog(default_catalog_path().to_string_lossy().as_ref())?;
    let technology_layout = load_technology_layout(
        default_technology_layout_path().to_string_lossy().as_ref(),
        &catalog,
    )?;
    let presentation: PresentationCatalog =
        ron::from_str(include_str!("../../../../assets/content/presentation.ron"))?;
    presentation.validate()?;
    Ok(format!(
        "Authoring assets valid: schema {}, {} roles, {} foliage layers, {} technologies, {} graph positions, {} presentation records",
        config.schema_version,
        catalog.roles.len(),
        catalog.foliage.len(),
        catalog.technology.nodes.len(),
        technology_layout.nodes.len(),
        presentation.textures.len()
            + presentation.materials.len()
            + presentation.clips.len()
            + presentation.controllers.len(),
    ))
}

fn load_content_catalog(path: &str) -> anyhow::Result<ContentCatalog> {
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("content-catalog path cannot be empty");
    }
    let catalog: ContentCatalog = ron::from_str(&fs::read_to_string(&path)?)?;
    catalog.validate()?;
    Ok(catalog)
}

fn load_technology_layout(
    path: &str,
    catalog: &ContentCatalog,
) -> anyhow::Result<TechnologyGraphLayout> {
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("technology-layout path cannot be empty");
    }
    let layout: TechnologyGraphLayout = ron::from_str(&fs::read_to_string(&path)?)?;
    layout.validate(&catalog.technology)?;
    Ok(layout)
}

fn load_game_config(path: &str) -> anyhow::Result<GameConfig> {
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("game-config path cannot be empty");
    }
    let config: GameConfig = ron::from_str(&fs::read_to_string(&path)?)?;
    Ok(config.upgrade()?)
}

fn save_game_config(config: &GameConfig, path: &str) -> anyhow::Result<PathBuf> {
    config.validate()?;
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("game-config path cannot be empty");
    }
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    fs::create_dir_all(parent)?;
    let temporary = PathBuf::from(format!("{}.tmp", path.display()));
    let backup = PathBuf::from(format!("{}.bak", path.display()));
    let encoded =
        ron::ser::to_string_pretty(config, ron::ser::PrettyConfig::new().struct_names(true))?;
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(encoded.as_bytes())?;
    file.sync_all()?;
    if path.is_file() {
        fs::copy(&path, &backup)?;
        fs::remove_file(&path)?;
    }
    if let Err(error) = fs::rename(&temporary, &path) {
        if backup.is_file() && !path.exists() {
            let _ = fs::copy(&backup, &path);
        }
        let _ = fs::remove_file(&temporary);
        return Err(error.into());
    }
    let reloaded = load_game_config(path.to_string_lossy().as_ref())?;
    if reloaded != *config {
        anyhow::bail!("reloaded game configuration does not match the authored configuration");
    }
    Ok(path)
}

fn save_content_catalog(catalog: &ContentCatalog, path: &str) -> anyhow::Result<PathBuf> {
    catalog.validate()?;
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("catalog path cannot be empty");
    }
    let encoded = ron::ser::to_string_pretty(catalog, ron::ser::PrettyConfig::default())?;
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    fs::create_dir_all(parent)?;
    let temporary = PathBuf::from(format!("{}.tmp", path.display()));
    let backup = PathBuf::from(format!("{}.bak", path.display()));
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(encoded.as_bytes())?;
    file.sync_all()?;
    if path.is_file() {
        fs::copy(&path, &backup)?;
        fs::remove_file(&path)?;
    }
    if let Err(error) = fs::rename(&temporary, &path) {
        if backup.is_file() && !path.exists() {
            let _ = fs::copy(&backup, &path);
        }
        let _ = fs::remove_file(&temporary);
        return Err(error.into());
    }
    let reloaded: ContentCatalog = ron::from_str(&fs::read_to_string(&path)?)?;
    reloaded.validate()?;
    if reloaded != *catalog {
        anyhow::bail!("reloaded catalog does not match the authored catalog");
    }
    Ok(path)
}

fn save_presentation_catalog(catalog: &PresentationCatalog, path: &str) -> anyhow::Result<PathBuf> {
    catalog.validate()?;
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("presentation-catalog path cannot be empty");
    }
    let encoded = ron::ser::to_string_pretty(catalog, ron::ser::PrettyConfig::default())?;
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    fs::create_dir_all(parent)?;
    let temporary = PathBuf::from(format!("{}.tmp", path.display()));
    let backup = PathBuf::from(format!("{}.bak", path.display()));
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(encoded.as_bytes())?;
    file.sync_all()?;
    if path.is_file() {
        fs::copy(&path, &backup)?;
        fs::remove_file(&path)?;
    }
    if let Err(error) = fs::rename(&temporary, &path) {
        if backup.is_file() && !path.exists() {
            let _ = fs::copy(&backup, &path);
        }
        let _ = fs::remove_file(&temporary);
        return Err(error.into());
    }
    let reloaded: PresentationCatalog = ron::from_str(&fs::read_to_string(&path)?)?;
    reloaded.validate()?;
    if reloaded != *catalog {
        anyhow::bail!("reloaded presentation catalog does not match the authored catalog");
    }
    Ok(path)
}

fn save_technology_layout(
    layout: &TechnologyGraphLayout,
    catalog: &ContentCatalog,
    path: &str,
) -> anyhow::Result<PathBuf> {
    catalog.validate()?;
    layout.validate(&catalog.technology)?;
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        anyhow::bail!("technology-layout path cannot be empty");
    }
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    fs::create_dir_all(parent)?;
    let temporary = PathBuf::from(format!("{}.tmp", path.display()));
    let backup = PathBuf::from(format!("{}.bak", path.display()));
    let encoded = ron::ser::to_string_pretty(layout, ron::ser::PrettyConfig::default())?;
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(encoded.as_bytes())?;
    file.sync_all()?;
    if path.is_file() {
        fs::copy(&path, &backup)?;
        fs::remove_file(&path)?;
    }
    if let Err(error) = fs::rename(&temporary, &path) {
        if backup.is_file() && !path.exists() {
            let _ = fs::copy(&backup, &path);
        }
        let _ = fs::remove_file(&temporary);
        return Err(error.into());
    }
    let reloaded = load_technology_layout(path.to_string_lossy().as_ref(), catalog)?;
    if reloaded != *layout {
        anyhow::bail!("reloaded technology layout does not match the authored layout");
    }
    Ok(path)
}

fn labeled_ids(ids: BTreeSet<StableId>) -> Vec<(StableId, String)> {
    ids.into_iter()
        .map(|id| {
            let label = id
                .as_str()
                .rsplit_once(':')
                .map_or(id.as_str(), |(_, suffix)| suffix)
                .replace(['_', '-'], " ");
            (id, label)
        })
        .collect()
}

fn resource_choices(catalog: &ContentCatalog) -> Vec<(StableId, String)> {
    let mut ids = BTreeSet::new();
    for role in catalog.roles.values() {
        ids.extend(role.resource.iter().cloned());
    }
    for building in catalog.buildings.values() {
        ids.extend(building.cost.keys().cloned());
        ids.extend(building.storage.iter().map(|value| value.resource.clone()));
        ids.extend(
            building
                .passive_resources
                .iter()
                .map(|value| value.resource.clone()),
        );
    }
    for archetype in catalog.archetypes.values() {
        if let Some(enemy) = &archetype.enemy {
            ids.insert(enemy.kill_reward.resource.clone());
        }
    }
    ids.extend(
        catalog
            .objectives
            .values()
            .filter_map(|objective| objective.resource.clone()),
    );
    for node in catalog.technology.nodes.values() {
        ids.extend(node.storage_boost_percent.keys().cloned());
    }
    labeled_ids(ids)
}

fn station_kind_choices(catalog: &ContentCatalog) -> Vec<(StableId, String)> {
    let mut ids = BTreeSet::new();
    for role in catalog.roles.values() {
        ids.extend(role.station_kinds.iter().cloned());
    }
    for building in catalog.buildings.values() {
        if let Some(station) = &building.station {
            ids.extend(station.accepted_role_kinds.iter().cloned());
        }
    }
    labeled_ids(ids)
}

fn target_kind_choices(catalog: &ContentCatalog) -> Vec<(StableId, String)> {
    let mut ids = BTreeSet::new();
    for role in catalog.roles.values() {
        ids.extend(role.target_kinds.iter().cloned());
    }
    for building in catalog.buildings.values() {
        if let Some(station) = &building.station {
            ids.extend(station.target_kinds.iter().cloned());
        }
    }
    for archetype in catalog.archetypes.values() {
        if let Some(enemy) = &archetype.enemy {
            ids.extend(enemy.target_kinds.iter().cloned());
        }
    }
    labeled_ids(ids)
}

fn ability_choices(catalog: &ContentCatalog) -> Vec<(StableId, String)> {
    labeled_ids(
        catalog
            .roles
            .values()
            .flat_map(|role| role.granted_abilities.iter().cloned())
            .collect(),
    )
}

fn stat_choices(catalog: &ContentCatalog) -> Vec<(StableId, String)> {
    let mut ids = BTreeSet::new();
    for node in catalog.technology.nodes.values() {
        ids.extend(node.global_stat_boost_percent.keys().cloned());
        for boosts in node.role_stat_boost_percent.values() {
            ids.extend(boosts.keys().cloned());
        }
    }
    labeled_ids(ids)
}

fn action_animation_choices(catalog: &ContentCatalog) -> Vec<String> {
    let mut values = BTreeSet::new();
    for role in catalog.roles.values() {
        values.insert(role.action_animation.clone());
        if let Some(animation) = role
            .equipment
            .as_ref()
            .and_then(|equipment| equipment.carry_animation.clone())
        {
            values.insert(animation);
        }
    }
    for archetype in catalog.archetypes.values() {
        if let Some(models) = &archetype.enemy_models {
            values.extend(
                models
                    .weapons
                    .iter()
                    .map(|weapon| weapon.action_animation.clone()),
            );
        }
    }
    values.retain(|value| !value.trim().is_empty());
    values.into_iter().collect()
}

fn equipment_node_choices(
    catalog: &ContentCatalog,
    presentation: &PresentationCatalog,
) -> Vec<String> {
    let mut values = BTreeSet::new();
    for equipment in catalog
        .roles
        .values()
        .filter_map(|role| role.equipment.as_ref())
    {
        values.extend(equipment.body_nodes.iter().cloned());
        values.extend(equipment.left_hand_node.iter().cloned());
        values.extend(equipment.right_hand_node.iter().cloned());
        values.extend(equipment.helmet_node.iter().cloned());
    }
    values.extend(
        presentation
            .prefab_renderer_materials
            .values()
            .flatten()
            .filter_map(|binding| binding.target_path.rsplit('/').next())
            .filter(|name| {
                name.starts_with("Body_")
                    || name.starts_with("LHand_")
                    || name.starts_with("RHand_")
                    || name.starts_with("Back_")
                    || name.starts_with("Helmet_")
            })
            .map(ToOwned::to_owned),
    );
    values.retain(|value| !value.trim().is_empty());
    values.into_iter().collect()
}

fn building_model_node_choices(catalog: &ContentCatalog) -> Vec<String> {
    let mut values = BTreeSet::new();
    for building in catalog.buildings.values() {
        for model in &building.model_handlers {
            values.insert(model.full_model.clone());
            values.extend(model.construction_stages.iter().cloned());
            values.extend(model.upgrades.iter().cloned());
            values.extend(model.other_models.iter().cloned());
        }
        for model in &building.storage_models {
            values.insert(model.empty_model.clone());
            values.insert(model.half_full_model.clone());
            values.insert(model.full_model.clone());
        }
    }
    for archetype in catalog.archetypes.values() {
        values.extend(
            archetype
                .rotating_nodes
                .iter()
                .map(|rotating| rotating.node.clone()),
        );
    }
    values.retain(|value| !value.trim().is_empty());
    values.into_iter().collect()
}

fn projectile_pool_choices(catalog: &ContentCatalog) -> Vec<String> {
    catalog
        .buildings
        .values()
        .filter_map(|building| building.projectile_shooter.as_ref())
        .map(|shooter| shooter.projectile_pool.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn technology_icon_choices(state: &ToolState) -> Vec<String> {
    let mut values: BTreeSet<_> = state
        .catalog
        .technology
        .nodes
        .values()
        .map(|node| node.icon_path.clone())
        .collect();
    values.extend(
        state
            .presentation
            .textures
            .values()
            .map(|texture| texture.asset_path.clone()),
    );
    values.retain(|value| !value.trim().is_empty());
    values.into_iter().collect()
}

fn archetype_kind_choice(ui: &mut egui::Ui, value: &mut ArchetypeKind) {
    egui::ComboBox::from_id_salt(("archetype_kind", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text(format!("{value:?}"))
        .show_ui(ui, |ui| {
            for kind in [
                ArchetypeKind::Building,
                ArchetypeKind::Player,
                ArchetypeKind::Enemy,
                ArchetypeKind::Resource,
                ArchetypeKind::Environment,
                ArchetypeKind::Vfx,
                ArchetypeKind::Ui,
                ArchetypeKind::Other,
            ] {
                ui.selectable_value(value, kind, format!("{kind:?}"));
            }
        });
}

fn searchable_string_choice(
    ui: &mut egui::Ui,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    label: &str,
    value: &mut String,
    choices: &[String],
    search: &mut String,
) -> bool {
    let mut changed = false;
    ui.push_id(id_salt, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(label);
            let selected = value
                .rsplit('/')
                .next()
                .filter(|name| !name.is_empty())
                .unwrap_or("Choose…")
                .to_owned();
            ui.menu_button(selected, |ui| {
                let response = ui.add(
                    egui::TextEdit::singleline(search)
                        .hint_text("Type to filter…")
                        .desired_width(280.0),
                );
                if response.gained_focus() {
                    response.request_focus();
                }
                let needle = search.trim().to_ascii_lowercase();
                egui::ScrollArea::vertical()
                    .id_salt("searchable_choices")
                    .max_height(280.0)
                    .show(ui, |ui| {
                        for choice in choices.iter().filter(|choice| {
                            needle.is_empty() || choice.to_ascii_lowercase().contains(&needle)
                        }) {
                            if ui.selectable_label(choice == value, choice).clicked() {
                                value.clone_from(choice);
                                changed = true;
                                search.clear();
                                ui.close();
                            }
                        }
                    });
            });
        });
    });
    changed
}

fn source_model_for_asset_path(catalog: &ContentCatalog, asset_path: &str) -> String {
    catalog
        .archetypes
        .values()
        .flat_map(|archetype| archetype.scenes.iter())
        .find(|scene| scene.asset_path == asset_path)
        .map_or_else(|| asset_path.to_owned(), |scene| scene.source_model.clone())
}

fn create_model_archetype(state: &mut ToolState) -> Result<(), String> {
    let id = StableId::new(state.new_archetype_id.trim().to_owned())
        .map_err(|error| error.to_string())?;
    if state.catalog.archetypes.contains_key(&id) {
        return Err(format!("model archetype {id} already exists"));
    }
    let display_name = state.new_archetype_name.trim();
    if display_name.is_empty() {
        return Err("model display name cannot be empty".to_owned());
    }
    let asset_path = state.new_archetype_asset.trim();
    if !state
        .discovered_model_assets
        .iter()
        .any(|candidate| candidate == asset_path)
    {
        return Err("choose a discovered GLB asset".to_owned());
    }
    let scene = ArchetypeScene {
        source_model: source_model_for_asset_path(&state.catalog, asset_path),
        asset_path: asset_path.to_owned(),
        age: None,
        is_default: true,
    };
    let archetype = ArchetypeDef {
        display_name: display_name.to_owned(),
        kind: state.new_archetype_kind,
        source_guid: format!("authored:{}", id.as_str()),
        source_path: format!("Authored/{}.glb", display_name.replace(' ', "_")),
        bounds: ArchetypeBounds {
            center: [0.0, 0.5, 0.0],
            size: [1.0, 1.0, 1.0],
        },
        footprint: [1, 1],
        scenes: vec![scene],
        component_types: Vec::new(),
        disable_after_milliseconds: None,
        health_bar_hide_milliseconds: None,
        rotating_nodes: Vec::new(),
        target_size_milli_cells: 0,
        health: None,
        enemy: None,
        enemy_models: None,
        enemy_spawner: None,
        pet: None,
    };
    let mut candidate = state.catalog.clone();
    candidate.archetypes.insert(id.clone(), archetype);
    commit_catalog_candidate(state, candidate)?;
    state.selected_archetype = Some(id);
    state.selected_archetype_scene = 0;
    Ok(())
}

fn add_archetype_scene(state: &mut ToolState, id: &StableId) -> Result<(), String> {
    let asset_path = state.new_archetype_asset.trim().to_owned();
    if !state
        .discovered_model_assets
        .iter()
        .any(|candidate| candidate == &asset_path)
    {
        return Err("choose a discovered GLB asset in Add model archetype first".to_owned());
    }
    let source_model = source_model_for_asset_path(&state.catalog, &asset_path);
    let mut candidate = state.catalog.clone();
    let archetype = candidate
        .archetypes
        .get_mut(id)
        .ok_or_else(|| format!("missing model archetype {id}"))?;
    if archetype
        .scenes
        .iter()
        .any(|scene| scene.asset_path == asset_path)
    {
        return Err("that GLB is already a variant of this archetype".to_owned());
    }
    let is_default = archetype.scenes.is_empty();
    archetype.scenes.push(ArchetypeScene {
        source_model,
        asset_path,
        age: None,
        is_default,
    });
    let selected = archetype.scenes.len() - 1;
    commit_catalog_candidate(state, candidate)?;
    state.selected_archetype_scene = selected;
    Ok(())
}

fn remove_archetype_scene(
    state: &mut ToolState,
    id: &StableId,
    index: usize,
) -> Result<(), String> {
    let mut candidate = state.catalog.clone();
    let archetype = candidate
        .archetypes
        .get_mut(id)
        .ok_or_else(|| format!("missing model archetype {id}"))?;
    if index >= archetype.scenes.len() {
        return Err("model variant index is stale".to_owned());
    }
    let removed_default = archetype.scenes[index].is_default;
    archetype.scenes.remove(index);
    if removed_default && let Some(first) = archetype.scenes.first_mut() {
        first.is_default = true;
    }
    commit_catalog_candidate(state, candidate)?;
    state.selected_archetype_scene = state
        .selected_archetype_scene
        .min(state.catalog.archetypes[id].scenes.len().saturating_sub(1));
    Ok(())
}

fn delete_model_archetype(state: &mut ToolState, id: &StableId) -> Result<(), String> {
    let mut candidate = state.catalog.clone();
    candidate
        .archetypes
        .remove(id)
        .ok_or_else(|| format!("missing model archetype {id}"))?;
    commit_catalog_candidate(state, candidate)?;
    state.selected_archetype = state.catalog.archetypes.keys().next().cloned();
    state.selected_archetype_scene = 0;
    Ok(())
}

fn stable_id_required_choice(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut StableId,
    choices: &[(StableId, String)],
) -> bool {
    let previous = value.clone();
    let selected = choices
        .iter()
        .find(|(id, _)| id == value)
        .map_or_else(|| value.to_string(), |(_, label)| label.clone());
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt((label, ui.next_auto_id()))
            .height(AUTHORING_COMBO_MAX_HEIGHT)
            .selected_text(selected)
            .show_ui(ui, |ui| {
                for (id, display) in choices {
                    ui.selectable_value(value, id.clone(), format!("{display}  ({id})"));
                }
            });
    });
    *value != previous
}

fn stable_id_option_choice(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<StableId>,
    choices: &[(StableId, String)],
) {
    let selected = value
        .as_ref()
        .and_then(|selected| choices.iter().find(|(id, _)| id == selected))
        .map_or("None", |(_, label)| label.as_str());
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt((label, ui.next_auto_id()))
            .height(AUTHORING_COMBO_MAX_HEIGHT)
            .selected_text(selected)
            .show_ui(ui, |ui| {
                ui.selectable_value(value, None, "None");
                for (id, display) in choices {
                    ui.selectable_value(value, Some(id.clone()), format!("{display}  ({id})"));
                }
            });
    });
}

fn stable_id_set_choices(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut BTreeSet<StableId>,
    choices: &[(StableId, String)],
) {
    ui.collapsing(format!("{label} ({})", values.len()), |ui| {
        ui.horizontal_wrapped(|ui| {
            for (id, display) in choices {
                let mut selected = values.contains(id);
                if ui.checkbox(&mut selected, display).changed() {
                    if selected {
                        values.insert(id.clone());
                    } else {
                        values.remove(id);
                    }
                }
            }
        });
    });
}

fn stable_id_vec_choices(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut Vec<StableId>,
    choices: &[(StableId, String)],
) {
    ui.collapsing(format!("{label} ({})", values.len()), |ui| {
        ui.horizontal_wrapped(|ui| {
            for (id, display) in choices {
                let mut selected = values.contains(id);
                if ui.checkbox(&mut selected, display).changed() {
                    if selected {
                        values.push(id.clone());
                    } else {
                        values.retain(|value| value != id);
                    }
                }
            }
        });
    });
}

fn searchable_stable_id_vec_editor(
    ui: &mut egui::Ui,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    label: &str,
    values: &mut Vec<StableId>,
    choices: &[(StableId, String)],
    search: &mut String,
) {
    ui.push_id(id_salt, |ui| {
        ui.label(format!("{label} ({})", values.len()));
        let mut remove = None;
        ui.horizontal_wrapped(|ui| {
            for value in values.iter() {
                let display = choices
                    .iter()
                    .find(|(id, _)| id == value)
                    .map_or(value.as_str(), |(_, display)| display.as_str());
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(display);
                        if ui.small_button("×").clicked() {
                            remove = Some(value.clone());
                        }
                    });
                });
            }
        });
        if let Some(remove) = remove {
            values.retain(|value| value != &remove);
        }
        ui.menu_button(format!("Add {label}…"), |ui| {
            ui.add(
                egui::TextEdit::singleline(search)
                    .hint_text("Type a name or stable ID…")
                    .desired_width(300.0),
            );
            let needle = search.trim().to_ascii_lowercase();
            let candidates = choices
                .iter()
                .filter(|(id, display)| {
                    !values.contains(id)
                        && (needle.is_empty()
                            || id.as_str().to_ascii_lowercase().contains(&needle)
                            || display.to_ascii_lowercase().contains(&needle))
                })
                .cloned()
                .collect::<Vec<_>>();
            egui::ScrollArea::vertical()
                .id_salt("stable_id_search_results")
                .max_height(260.0)
                .show(ui, |ui| {
                    for (id, display) in candidates {
                        if ui
                            .selectable_label(false, format!("{display}  ({id})"))
                            .clicked()
                        {
                            values.push(id);
                            search.clear();
                            ui.close();
                        }
                    }
                });
        });
    });
}

fn string_choice(ui: &mut egui::Ui, label: &str, value: &mut String, choices: &[String]) {
    let selected = if value.is_empty() {
        "None".to_owned()
    } else {
        value.clone()
    };
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt((label, ui.next_auto_id()))
            .height(AUTHORING_COMBO_MAX_HEIGHT)
            .selected_text(selected)
            .show_ui(ui, |ui| {
                for choice in choices {
                    ui.selectable_value(&mut *value, choice.clone(), choice);
                }
            });
    });
}

fn optional_string_choice(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<String>,
    choices: &[String],
) {
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt((label, ui.next_auto_id()))
            .height(AUTHORING_COMBO_MAX_HEIGHT)
            .selected_text(value.as_deref().unwrap_or("None"))
            .show_ui(ui, |ui| {
                ui.selectable_value(value, None, "None");
                for choice in choices {
                    ui.selectable_value(value, Some(choice.clone()), choice);
                }
            });
    });
}

fn string_vec_choices(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut Vec<String>,
    choices: &[String],
) {
    ui.collapsing(format!("{label} ({})", values.len()), |ui| {
        let mut remove = None;
        for (index, value) in values.iter_mut().enumerate() {
            ui.push_id((label, index), |ui| {
                ui.horizontal(|ui| {
                    string_choice(ui, "Model", value, choices);
                    if ui.small_button("Remove").clicked() {
                        remove = Some(index);
                    }
                });
            });
        }
        if let Some(index) = remove {
            values.remove(index);
        }
        if ui.button("Add model").clicked()
            && let Some(first) = choices.first()
        {
            values.push(first.clone());
        }
    });
}

fn stable_u32_map_editor(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut BTreeMap<StableId, u32>,
    choices: &[(StableId, String)],
) {
    ui.label(label);
    let mut remove = None;
    for id in values.keys().cloned().collect::<Vec<_>>() {
        ui.horizontal(|ui| {
            ui.label(
                choices
                    .iter()
                    .find(|(choice, _)| choice == &id)
                    .map_or(id.as_str(), |(_, label)| label),
            );
            ui.add(egui::DragValue::new(
                values.get_mut(&id).expect("map key exists"),
            ));
            if ui.small_button("Remove").clicked() {
                remove = Some(id);
            }
        });
    }
    if let Some(id) = remove {
        values.remove(&id);
    }
    let mut add = None;
    egui::ComboBox::from_id_salt((label, "add", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text("Add…")
        .show_ui(ui, |ui| {
            for (id, display) in choices {
                if !values.contains_key(id) && ui.selectable_label(false, display).clicked() {
                    add = Some(id.clone());
                }
            }
        });
    if let Some(id) = add {
        values.insert(id, 1);
    }
}

fn stable_u16_map_editor(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut BTreeMap<StableId, u16>,
    choices: &[(StableId, String)],
) {
    ui.label(label);
    let mut remove = None;
    for id in values.keys().cloned().collect::<Vec<_>>() {
        ui.horizontal(|ui| {
            ui.label(
                choices
                    .iter()
                    .find(|(choice, _)| choice == &id)
                    .map_or(id.as_str(), |(_, label)| label),
            );
            ui.add(egui::DragValue::new(
                values.get_mut(&id).expect("map key exists"),
            ));
            if ui.small_button("Remove").clicked() {
                remove = Some(id);
            }
        });
    }
    if let Some(id) = remove {
        values.remove(&id);
    }
    let mut add = None;
    egui::ComboBox::from_id_salt((label, "add", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text("Add…")
        .show_ui(ui, |ui| {
            for (id, display) in choices {
                if !values.contains_key(id) && ui.selectable_label(false, display).clicked() {
                    add = Some(id.clone());
                }
            }
        });
    if let Some(id) = add {
        values.insert(id, 1);
    }
}

fn stable_i32_map_editor(
    ui: &mut egui::Ui,
    label: &str,
    values: &mut BTreeMap<StableId, i32>,
    choices: &[(StableId, String)],
) {
    ui.label(label);
    let mut remove = None;
    for id in values.keys().cloned().collect::<Vec<_>>() {
        ui.horizontal(|ui| {
            ui.label(
                choices
                    .iter()
                    .find(|(choice, _)| choice == &id)
                    .map_or(id.as_str(), |(_, label)| label),
            );
            ui.add(egui::DragValue::new(values.get_mut(&id).expect("map key exists")).suffix("%"));
            if ui.small_button("Remove").clicked() {
                remove = Some(id);
            }
        });
    }
    if let Some(id) = remove {
        values.remove(&id);
    }
    let mut add = None;
    egui::ComboBox::from_id_salt((label, "add", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text("Add…")
        .show_ui(ui, |ui| {
            for (id, display) in choices {
                if !values.contains_key(id) && ui.selectable_label(false, display).clicked() {
                    add = Some(id.clone());
                }
            }
        });
    if let Some(id) = add {
        values.insert(id, 0);
    }
}

fn role_stat_map_editor(
    ui: &mut egui::Ui,
    values: &mut BTreeMap<StableId, BTreeMap<StableId, i32>>,
    roles: &[(StableId, String)],
    stats: &[(StableId, String)],
) {
    ui.label("Role-specific stat boosts");
    let mut remove = None;
    for role in values.keys().cloned().collect::<Vec<_>>() {
        let label = roles
            .iter()
            .find(|(id, _)| id == &role)
            .map_or(role.as_str(), |(_, label)| label);
        ui.collapsing(label, |ui| {
            stable_i32_map_editor(
                ui,
                "Stats",
                values.get_mut(&role).expect("role map exists"),
                stats,
            );
            if ui.button("Remove role effects").clicked() {
                remove = Some(role.clone());
            }
        });
    }
    if let Some(role) = remove {
        values.remove(&role);
    }
    let mut add = None;
    egui::ComboBox::from_id_salt(("role stat", ui.next_auto_id()))
        .height(AUTHORING_COMBO_MAX_HEIGHT)
        .selected_text("Add role…")
        .show_ui(ui, |ui| {
            for (id, display) in roles {
                if !values.contains_key(id) && ui.selectable_label(false, display).clicked() {
                    add = Some(id.clone());
                }
            }
        });
    if let Some(role) = add {
        values.insert(role, BTreeMap::new());
    }
}
