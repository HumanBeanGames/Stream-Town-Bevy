fn building_draft(catalog: &ContentCatalog, id: &StableId) -> Option<BuildingDraft> {
    Some(BuildingDraft {
        id: id.clone(),
        value: catalog.buildings.get(id)?.clone(),
    })
}

fn refresh_building_draft(state: &mut ToolState) {
    if state
        .selected_building
        .as_ref()
        .is_none_or(|id| !state.catalog.buildings.contains_key(id))
    {
        state.selected_building = state.catalog.buildings.keys().next().cloned();
    }
    state.building_draft = state
        .selected_building
        .as_ref()
        .and_then(|id| building_draft(&state.catalog, id));
}

fn apply_building_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .building_draft
        .clone()
        .ok_or_else(|| "no building selected".to_owned())?;
    if draft.value.display_name.trim().is_empty() {
        return Err("building display name cannot be empty".to_owned());
    }
    let mut candidate = state.catalog.clone();
    candidate
        .archetypes
        .get_mut(&draft.value.archetype)
        .ok_or_else(|| format!("missing model archetype {}", draft.value.archetype))?
        .footprint = draft.value.footprint;
    synchronize_buildings_for_archetype(
        &mut candidate,
        &draft.value.archetype,
        draft.value.footprint,
    );
    candidate.buildings.insert(draft.id, draft.value);
    commit_catalog_candidate(state, candidate)
}

fn synchronize_buildings_for_archetype(
    catalog: &mut ContentCatalog,
    archetype: &StableId,
    footprint: [u16; 2],
) -> usize {
    let mut updated = 0;
    for building in catalog
        .buildings
        .values_mut()
        .filter(|building| &building.archetype == archetype)
    {
        building.footprint = footprint;
        updated += 1;
    }
    updated
}

fn duplicate_selected_building(state: &mut ToolState) -> Result<(), String> {
    let source = state
        .selected_building
        .as_ref()
        .and_then(|id| state.catalog.buildings.get(id))
        .cloned()
        .ok_or_else(|| "no building selected".to_owned())?;
    let id = StableId::new(state.new_building_id.trim().to_owned())
        .map_err(|error| error.to_string())?;
    if state.catalog.buildings.contains_key(&id) {
        return Err(format!("building {id} already exists"));
    }
    let display_name = state.new_building_name.trim();
    if display_name.is_empty() {
        return Err("new building name cannot be empty".to_owned());
    }
    let mut value = source;
    display_name.clone_into(&mut value.display_name);
    let mut candidate = state.catalog.clone();
    candidate.buildings.insert(id.clone(), value);
    commit_catalog_candidate(state, candidate)?;
    state.selected_building = Some(id);
    refresh_building_draft(state);
    Ok(())
}

fn delete_selected_building(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_building
        .clone()
        .ok_or_else(|| "no building selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    candidate
        .buildings
        .remove(&id)
        .ok_or_else(|| format!("missing building {id}"))?;
    commit_catalog_candidate(state, candidate)?;
    state.selected_building = state.catalog.buildings.keys().next().cloned();
    refresh_building_draft(state);
    Ok(())
}

fn role_draft(catalog: &ContentCatalog, id: &StableId) -> Option<RoleDraft> {
    let value = catalog.roles.get(id)?.clone();
    Some(RoleDraft {
        id: id.clone(),
        value,
    })
}

fn apply_role_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .role_draft
        .clone()
        .ok_or_else(|| "no role selected".to_owned())?;
    if draft.value.display_name.trim().is_empty() {
        return Err("role display name cannot be empty".to_owned());
    }
    if draft.value.action_animation.trim().is_empty() || draft.value.action_animation_variants == 0
    {
        return Err("role action animation and variant count must be defined".to_owned());
    }
    if draft.value.movement_speed_multiplier_per_thousand == 0
        || draft.value.experience_multiplier_per_thousand == 0
        || draft.value.base_health == 0
        || draft.value.base_movement_speed_milli_cells_per_second == 0
    {
        return Err("movement, experience, and health base values must be positive".to_owned());
    }
    let mut candidate = state.catalog.clone();
    candidate.roles.insert(draft.id, draft.value);
    commit_catalog_candidate(state, candidate)
}

fn duplicate_selected_role(state: &mut ToolState) -> Result<(), String> {
    let source = state
        .selected_role
        .as_ref()
        .and_then(|id| state.catalog.roles.get(id))
        .cloned()
        .ok_or_else(|| "no role selected".to_owned())?;
    let id =
        StableId::new(state.new_role_id.trim().to_owned()).map_err(|error| error.to_string())?;
    if state.catalog.roles.contains_key(&id) {
        return Err(format!("role {id} already exists"));
    }
    let display_name = state.new_role_name.trim();
    if display_name.is_empty() {
        return Err("new role name cannot be empty".to_owned());
    }
    let mut value = source;
    display_name.clone_into(&mut value.display_name);
    let mut candidate = state.catalog.clone();
    candidate.roles.insert(id.clone(), value);
    commit_catalog_candidate(state, candidate)?;
    state.selected_role = Some(id);
    refresh_role_draft(state);
    Ok(())
}

fn delete_selected_role(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_role
        .clone()
        .ok_or_else(|| "no role selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    candidate
        .roles
        .remove(&id)
        .ok_or_else(|| format!("missing role {id}"))?;
    commit_catalog_candidate(state, candidate)?;
    state.selected_role = state.catalog.roles.keys().next().cloned();
    refresh_role_draft(state);
    Ok(())
}

fn apply_foliage_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .foliage_draft
        .clone()
        .ok_or_else(|| "no foliage layer selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    let layer = candidate
        .foliage
        .iter_mut()
        .find(|layer| layer.id == draft.id)
        .ok_or_else(|| format!("missing foliage layer {}", draft.id))?;
    *layer = draft;
    commit_catalog_candidate(state, candidate)
}

fn duplicate_foliage_layer(state: &mut ToolState) -> Result<(), String> {
    let mut layer = state
        .foliage_draft
        .clone()
        .ok_or_else(|| "no foliage layer selected".to_owned())?;
    let id =
        StableId::new(state.new_foliage_id.trim().to_owned()).map_err(|error| error.to_string())?;
    if state.catalog.foliage.iter().any(|layer| layer.id == id) {
        return Err(format!("foliage layer {id} already exists"));
    }
    layer.id = id.clone();
    let mut candidate = state.catalog.clone();
    candidate.foliage.push(layer);
    commit_catalog_candidate(state, candidate)?;
    state.selected_foliage = Some(id);
    state.selected_foliage_variant = 0;
    refresh_foliage_draft(state);
    Ok(())
}

fn delete_foliage_layer(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_foliage
        .clone()
        .ok_or_else(|| "no foliage layer selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    let before = candidate.foliage.len();
    candidate.foliage.retain(|layer| layer.id != id);
    if candidate.foliage.len() == before {
        return Err(format!("missing foliage layer {id}"));
    }
    commit_catalog_candidate(state, candidate)?;
    state.selected_foliage = state.catalog.foliage.first().map(|layer| layer.id.clone());
    state.selected_foliage_variant = 0;
    refresh_foliage_draft(state);
    Ok(())
}

fn apply_resource_generation_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .resource_generation_draft
        .clone()
        .ok_or_else(|| "no resource generation layer selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    let layer = candidate
        .resource_generation
        .iter_mut()
        .find(|layer| layer.id == draft.id)
        .ok_or_else(|| format!("missing resource generation layer {}", draft.id))?;
    *layer = draft;
    commit_catalog_candidate(state, candidate)
}

fn duplicate_resource_generation_layer(state: &mut ToolState) -> Result<(), String> {
    let mut layer = state
        .resource_generation_draft
        .clone()
        .ok_or_else(|| "no resource generation layer selected".to_owned())?;
    let id = StableId::new(state.new_resource_generation_id.trim().to_owned())
        .map_err(|error| error.to_string())?;
    if state
        .catalog
        .resource_generation
        .iter()
        .any(|layer| layer.id == id)
    {
        return Err(format!("resource generation layer {id} already exists"));
    }
    let display_name = state.new_resource_generation_name.trim();
    if display_name.is_empty() {
        return Err("resource generation layer name cannot be empty".to_owned());
    }
    layer.id = id.clone();
    display_name.clone_into(&mut layer.display_name);
    let mut candidate = state.catalog.clone();
    candidate.resource_generation.push(layer);
    commit_catalog_candidate(state, candidate)?;
    state.selected_resource_generation = Some(id);
    refresh_resource_generation_draft(state);
    Ok(())
}

fn delete_resource_generation_layer(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_resource_generation
        .clone()
        .ok_or_else(|| "no resource generation layer selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    let before = candidate.resource_generation.len();
    candidate.resource_generation.retain(|layer| layer.id != id);
    if candidate.resource_generation.len() == before {
        return Err(format!("missing resource generation layer {id}"));
    }
    commit_catalog_candidate(state, candidate)?;
    state.selected_resource_generation = state
        .catalog
        .resource_generation
        .first()
        .map(|layer| layer.id.clone());
    refresh_resource_generation_draft(state);
    Ok(())
}

fn authoring_snapshot(state: &ToolState) -> AuthoringSnapshot {
    AuthoringSnapshot {
        catalog: state.catalog.clone(),
        technology_layout: state.technology_layout.clone(),
    }
}

fn push_authoring_undo(state: &mut ToolState, previous: AuthoringSnapshot) {
    state.undo_authoring.push(previous);
    state.redo_authoring.clear();
}

fn undo_authoring_edit(state: &mut ToolState) {
    if let Some(previous) = state.undo_authoring.pop() {
        let current = authoring_snapshot(state);
        state.redo_authoring.push(current);
        state.catalog = previous.catalog;
        state.technology_layout = previous.technology_layout;
        refresh_catalog_drafts(state);
        "Authoring edit undone".clone_into(&mut state.status);
    }
}

fn redo_authoring_edit(state: &mut ToolState) {
    if let Some(next) = state.redo_authoring.pop() {
        let current = authoring_snapshot(state);
        state.undo_authoring.push(current);
        state.catalog = next.catalog;
        state.technology_layout = next.technology_layout;
        refresh_catalog_drafts(state);
        "Authoring edit redone".clone_into(&mut state.status);
    }
}

fn commit_catalog_candidate(
    state: &mut ToolState,
    candidate: ContentCatalog,
) -> Result<(), String> {
    candidate.validate().map_err(|error| error.to_string())?;
    let previous = authoring_snapshot(state);
    let mut technology_layout = state.technology_layout.clone();
    technology_layout.reconcile(&candidate.technology);
    technology_layout
        .validate(&candidate.technology)
        .map_err(|error| error.to_string())?;
    push_authoring_undo(state, previous);
    state.catalog = candidate;
    state.technology_layout = technology_layout;
    refresh_catalog_drafts(state);
    Ok(())
}

fn create_technology_group(state: &mut ToolState) -> Result<(), String> {
    let id =
        StableId::new(state.new_group_id.trim().to_owned()).map_err(|error| error.to_string())?;
    let display_name = state.new_group_name.trim();
    if display_name.is_empty() {
        return Err("group name cannot be empty".to_owned());
    }
    if state.catalog.technology.groups.contains_key(&id) {
        return Err(format!("technology group {id} already exists"));
    }
    let mut candidate = state.catalog.clone();
    candidate.technology.groups.insert(
        id.clone(),
        TechGroup {
            display_name: display_name.to_owned(),
            nodes: Vec::new(),
        },
    );
    commit_catalog_candidate(state, candidate)?;
    state.selected_group = Some(id);
    state.technology_graph_view.request_fit();
    Ok(())
}

fn delete_selected_technology_group(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_group
        .clone()
        .ok_or_else(|| "no technology group selected".to_owned())?;
    let group = state
        .catalog
        .technology
        .groups
        .get(&id)
        .ok_or_else(|| format!("missing technology group {id}"))?;
    if !group.nodes.is_empty()
        || state
            .catalog
            .technology
            .nodes
            .values()
            .any(|node| node.group.as_ref() == Some(&id))
    {
        return Err("move or delete every node before deleting its group".to_owned());
    }
    let mut candidate = state.catalog.clone();
    candidate.technology.groups.remove(&id);
    commit_catalog_candidate(state, candidate)?;
    state.selected_group = state.catalog.technology.groups.keys().next().cloned();
    state.technology_graph_view.request_fit();
    Ok(())
}

fn create_technology_node(state: &mut ToolState) -> Result<(), String> {
    let group_id = state
        .selected_group
        .clone()
        .ok_or_else(|| "select a technology group first".to_owned())?;
    let id = StableId::new(state.new_technology_id.trim().to_owned())
        .map_err(|error| error.to_string())?;
    let display_name = state.new_technology_name.trim();
    if display_name.is_empty() {
        return Err("technology name cannot be empty".to_owned());
    }
    if state.catalog.technology.nodes.contains_key(&id) {
        return Err(format!("technology {id} already exists"));
    }
    let mut candidate = state.catalog.clone();
    candidate.technology.nodes.insert(
        id.clone(),
        TechNode {
            display_name: display_name.to_owned(),
            group: Some(group_id.clone()),
            ..TechNode::default()
        },
    );
    candidate
        .technology
        .groups
        .get_mut(&group_id)
        .ok_or_else(|| format!("missing technology group {group_id}"))?
        .nodes
        .push(id.clone());
    commit_catalog_candidate(state, candidate)?;
    state.technology_draft = technology_draft(&state.catalog, &id);
    state.technology_graph_view.request_focus(id);
    Ok(())
}

fn delete_selected_technology_node(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .technology_draft
        .as_ref()
        .map(|draft| draft.id.clone())
        .ok_or_else(|| "no technology selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    candidate
        .technology
        .nodes
        .remove(&id)
        .ok_or_else(|| format!("missing technology {id}"))?;
    for node in candidate.technology.nodes.values_mut() {
        node.prerequisites.retain(|reference| reference != &id);
        node.unlocks.retain(|reference| reference != &id);
    }
    for group in candidate.technology.groups.values_mut() {
        group.nodes.retain(|reference| reference != &id);
    }
    commit_catalog_candidate(state, candidate)?;
    state.technology_draft = None;
    Ok(())
}

fn connect_technology_nodes(
    state: &mut ToolState,
    prerequisite: &StableId,
    dependent: &StableId,
) -> Result<(), String> {
    if prerequisite == dependent {
        return Err("a technology cannot require itself".to_owned());
    }
    let mut candidate = state.catalog.clone();
    if !candidate.technology.nodes.contains_key(prerequisite) {
        return Err(format!("missing prerequisite technology {prerequisite}"));
    }
    let node = candidate
        .technology
        .nodes
        .get_mut(dependent)
        .ok_or_else(|| format!("missing dependent technology {dependent}"))?;
    if node.prerequisites.contains(prerequisite) {
        return Err("that prerequisite connection already exists".to_owned());
    }
    node.prerequisites.push(prerequisite.clone());
    commit_catalog_candidate(state, candidate)
}

fn disconnect_technology_nodes(
    state: &mut ToolState,
    prerequisite: &StableId,
    dependent: &StableId,
) -> Result<(), String> {
    let mut candidate = state.catalog.clone();
    let node = candidate
        .technology
        .nodes
        .get_mut(dependent)
        .ok_or_else(|| format!("missing dependent technology {dependent}"))?;
    let previous = node.prerequisites.len();
    node.prerequisites.retain(|value| value != prerequisite);
    if node.prerequisites.len() == previous {
        return Err("that prerequisite connection does not exist".to_owned());
    }
    commit_catalog_candidate(state, candidate)
}

#[cfg(test)]
mod runtime_console_tests {
    use super::*;

    #[test]
    fn runtime_actions_sequence_after_latest_acknowledgement() {
        let directory = tempfile::tempdir().unwrap();
        let mut state = ToolState {
            runtime_console: RuntimeConsoleStore::new(directory.path()),
            runtime_status: Some(RuntimeConsoleStatus {
                schema_version: stream_town_domain::CURRENT_RUNTIME_CONSOLE_SCHEMA,
                updated_unix_millis: u64::MAX,
                last_processed_sequence: 41,
                ..RuntimeConsoleStatus::default()
            }),
            ..ToolState::default()
        };

        send_runtime_action(&mut state, RuntimeConsoleAction::Save).unwrap();

        let request = state.runtime_console.read_request().unwrap().unwrap();
        assert_eq!(request.sequence, 42);
        assert_eq!(request.action, RuntimeConsoleAction::Save);
    }

    #[test]
    fn stale_runtime_status_is_not_treated_as_attached() {
        let stale = RuntimeConsoleStatus {
            schema_version: stream_town_domain::CURRENT_RUNTIME_CONSOLE_SCHEMA,
            updated_unix_millis: 1,
            ..RuntimeConsoleStatus::default()
        };
        assert!(!runtime_console_attached(Some(&stale)));
        let live = RuntimeConsoleStatus {
            updated_unix_millis: u64::MAX,
            ..stale
        };
        assert!(runtime_console_attached(Some(&live)));
    }
}

fn refresh_technology_draft(state: &mut ToolState) {
    let selected = state
        .technology_draft
        .as_ref()
        .map(|draft| draft.id.clone());
    state.technology_draft = selected
        .as_ref()
        .and_then(|id| technology_draft(&state.catalog, id));
}

fn refresh_objective_draft(state: &mut ToolState) {
    if state
        .selected_objective
        .as_ref()
        .is_none_or(|id| !state.catalog.objectives.contains_key(id))
    {
        state.selected_objective = state.catalog.objectives.keys().next().cloned();
    }
    state.objective_draft = state.selected_objective.as_ref().and_then(|id| {
        state
            .catalog
            .objectives
            .get(id)
            .cloned()
            .map(|value| ObjectiveDraft {
                id: id.clone(),
                value,
            })
    });
}

fn refresh_role_draft(state: &mut ToolState) {
    if state
        .selected_role
        .as_ref()
        .is_none_or(|id| !state.catalog.roles.contains_key(id))
    {
        state.selected_role = state.catalog.roles.keys().next().cloned();
    }
    state.role_draft = state
        .selected_role
        .as_ref()
        .and_then(|id| role_draft(&state.catalog, id));
    state.role_preview_animation = state.role_draft.as_ref().and_then(|draft| {
        matching_role_animation_state(&state.presentation, &draft.value.action_animation)
    });
}

fn refresh_foliage_draft(state: &mut ToolState) {
    if state
        .selected_foliage
        .as_ref()
        .is_none_or(|id| !state.catalog.foliage.iter().any(|layer| &layer.id == id))
    {
        state.selected_foliage = state.catalog.foliage.first().map(|layer| layer.id.clone());
    }
    state.foliage_draft = state.selected_foliage.as_ref().and_then(|id| {
        state
            .catalog
            .foliage
            .iter()
            .find(|layer| &layer.id == id)
            .cloned()
    });
    state.selected_foliage_variant = state.foliage_draft.as_ref().map_or(0, |layer| {
        state
            .selected_foliage_variant
            .min(layer.variants.len().saturating_sub(1))
    });
}

fn refresh_resource_generation_draft(state: &mut ToolState) {
    if state
        .selected_resource_generation
        .as_ref()
        .is_none_or(|id| {
            !state
                .catalog
                .resource_generation
                .iter()
                .any(|layer| &layer.id == id)
        })
    {
        state.selected_resource_generation = state
            .catalog
            .resource_generation
            .first()
            .map(|layer| layer.id.clone());
    }
    state.resource_generation_draft = state.selected_resource_generation.as_ref().and_then(|id| {
        state
            .catalog
            .resource_generation
            .iter()
            .find(|layer| &layer.id == id)
            .cloned()
    });
}

fn refresh_enemy_camp_generation_draft(state: &mut ToolState) {
    if state
        .selected_enemy_camp_generation
        .as_ref()
        .is_none_or(|id| {
            !state
                .catalog
                .enemy_camp_generation
                .iter()
                .any(|layer| &layer.id == id)
        })
    {
        state.selected_enemy_camp_generation = state
            .catalog
            .enemy_camp_generation
            .first()
            .map(|layer| layer.id.clone());
    }
    state.enemy_camp_generation_draft =
        state
            .selected_enemy_camp_generation
            .as_ref()
            .and_then(|id| {
                state
                    .catalog
                    .enemy_camp_generation
                    .iter()
                    .find(|layer| &layer.id == id)
                    .cloned()
            });
}

fn refresh_catalog_drafts(state: &mut ToolState) {
    refresh_technology_draft(state);
    refresh_objective_draft(state);
    refresh_role_draft(state);
    refresh_building_draft(state);
    refresh_foliage_draft(state);
    refresh_resource_generation_draft(state);
    refresh_enemy_camp_generation_draft(state);
    state.generated_world = None;
}

fn apply_enemy_camp_generation_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .enemy_camp_generation_draft
        .clone()
        .ok_or_else(|| "no enemy camp generation layer selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    let layer = candidate
        .enemy_camp_generation
        .iter_mut()
        .find(|layer| layer.id == draft.id)
        .ok_or_else(|| format!("missing enemy camp layer {}", draft.id))?;
    *layer = draft;
    commit_catalog_candidate(state, candidate)
}

fn duplicate_enemy_camp_generation_layer(state: &mut ToolState) -> Result<(), String> {
    let mut layer = state
        .enemy_camp_generation_draft
        .clone()
        .ok_or_else(|| "no enemy camp generation layer selected".to_owned())?;
    let id = StableId::new(state.new_enemy_camp_generation_id.trim().to_owned())
        .map_err(|error| error.to_string())?;
    if state
        .catalog
        .enemy_camp_generation
        .iter()
        .any(|layer| layer.id == id)
    {
        return Err(format!("enemy camp generation layer {id} already exists"));
    }
    layer.id = id.clone();
    let mut candidate = state.catalog.clone();
    candidate.enemy_camp_generation.push(layer);
    commit_catalog_candidate(state, candidate)?;
    state.selected_enemy_camp_generation = Some(id);
    refresh_enemy_camp_generation_draft(state);
    Ok(())
}

fn delete_enemy_camp_generation_layer(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_enemy_camp_generation
        .clone()
        .ok_or_else(|| "no enemy camp generation layer selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    let before = candidate.enemy_camp_generation.len();
    candidate
        .enemy_camp_generation
        .retain(|layer| layer.id != id);
    if candidate.enemy_camp_generation.len() == before {
        return Err(format!("missing enemy camp generation layer {id}"));
    }
    commit_catalog_candidate(state, candidate)?;
    state.selected_enemy_camp_generation = state
        .catalog
        .enemy_camp_generation
        .first()
        .map(|layer| layer.id.clone());
    refresh_enemy_camp_generation_draft(state);
    Ok(())
}

fn apply_objective_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .objective_draft
        .clone()
        .ok_or_else(|| "no vote requirement selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    if !candidate.objectives.contains_key(&draft.id) {
        return Err(format!("missing vote requirement {}", draft.id));
    }
    candidate.objectives.insert(draft.id, draft.value);
    commit_catalog_candidate(state, candidate)
}

fn duplicate_selected_objective(state: &mut ToolState) -> Result<(), String> {
    let source = state
        .objective_draft
        .as_ref()
        .map(|draft| draft.value.clone())
        .ok_or_else(|| "no vote requirement selected".to_owned())?;
    let id = StableId::new(state.new_objective_id.trim().to_owned())
        .map_err(|error| error.to_string())?;
    if state.catalog.objectives.contains_key(&id) {
        return Err(format!("vote requirement {id} already exists"));
    }
    let mut candidate = state.catalog.clone();
    candidate.objectives.insert(id.clone(), source);
    commit_catalog_candidate(state, candidate)?;
    state.selected_objective = Some(id);
    refresh_objective_draft(state);
    Ok(())
}

fn delete_selected_objective(state: &mut ToolState) -> Result<(), String> {
    let id = state
        .selected_objective
        .clone()
        .ok_or_else(|| "no vote requirement selected".to_owned())?;
    if state
        .catalog
        .technology
        .nodes
        .values()
        .any(|node| node.objectives.contains(&id))
    {
        return Err("remove this requirement from every technology before deleting it".to_owned());
    }
    let mut candidate = state.catalog.clone();
    candidate
        .objectives
        .remove(&id)
        .ok_or_else(|| format!("missing vote requirement {id}"))?;
    commit_catalog_candidate(state, candidate)?;
    state.selected_objective = state.catalog.objectives.keys().next().cloned();
    refresh_objective_draft(state);
    Ok(())
}

fn apply_technology_draft(state: &mut ToolState) -> Result<(), String> {
    let draft = state
        .technology_draft
        .clone()
        .ok_or_else(|| "no technology selected".to_owned())?;
    let mut candidate = state.catalog.clone();
    if !candidate.technology.nodes.contains_key(&draft.id) {
        return Err(format!("missing technology {}", draft.id));
    }
    let group = draft.value.group.clone();
    candidate
        .technology
        .nodes
        .insert(draft.id.clone(), draft.value);
    for group in candidate.technology.groups.values_mut() {
        group.nodes.retain(|reference| reference != &draft.id);
    }
    if let Some(group_id) = &group {
        candidate
            .technology
            .groups
            .get_mut(group_id)
            .ok_or_else(|| format!("missing technology group {group_id}"))?
            .nodes
            .push(draft.id.clone());
    }
    commit_catalog_candidate(state, candidate)
}

fn draw_world_preview(
    ui: &mut egui::Ui,
    world: &GeneratedWorld,
    layer: WorldPreviewLayer,
    selected_foliage: Option<&StableId>,
    selected_resource_target: Option<&StableId>,
    enemy_camp_layer: Option<&EnemyCampGenerationDef>,
    water_level_centimetres: i16,
) {
    let width = world.navigation.width();
    let height = world.navigation.height();
    let desired_width = ui.available_width().min(720.0);
    let desired = egui::vec2(
        desired_width,
        desired_width * f32::from(height) / f32::from(width),
    );
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let stride = usize::from(width.max(height)).div_ceil(96).max(1);
    let stride_u16 = u16::try_from(stride).unwrap_or(u16::MAX);
    let (minimum_height, maximum_height) = (0..height)
        .flat_map(|z| (0..width).map(move |x| GridPos { x, z }))
        .filter_map(|position| world.navigation.height_at(position))
        .fold((i16::MAX, i16::MIN), |(minimum, maximum), value| {
            (minimum.min(value), maximum.max(value))
        });
    for z in (0..height).step_by(stride) {
        for x in (0..width).step_by(stride) {
            let position = GridPos { x, z };
            let left = rect.left() + f32::from(x) * rect.width() / f32::from(width);
            let right = rect.left()
                + f32::from(x.saturating_add(stride_u16).min(width)) * rect.width()
                    / f32::from(width);
            let top = rect.top() + f32::from(z) * rect.height() / f32::from(height);
            let bottom = rect.top()
                + f32::from(z.saturating_add(stride_u16).min(height)) * rect.height()
                    / f32::from(height);
            let cell_height = world.navigation.height_at(position).unwrap_or_default();
            let color = match layer {
                WorldPreviewLayer::Elevation => {
                    if cell_height <= water_level_centimetres {
                        egui::Color32::from_rgb(34, 91, 145)
                    } else {
                        let range =
                            f32::from(maximum_height.saturating_sub(minimum_height)).max(1.0);
                        let normalized =
                            f32::from(cell_height.saturating_sub(minimum_height)) / range;
                        terrain_preview_color(normalized)
                    }
                }
                WorldPreviewLayer::Navigation => {
                    if world.navigation.is_walkable(position) {
                        egui::Color32::from_rgb(70, 145, 83)
                    } else {
                        egui::Color32::from_rgb(42, 52, 62)
                    }
                }
                WorldPreviewLayer::LandResources
                | WorldPreviewLayer::WaterResources
                | WorldPreviewLayer::Foliage
                | WorldPreviewLayer::EnemyCamps => {
                    if cell_height <= water_level_centimetres {
                        egui::Color32::from_rgb(29, 70, 105)
                    } else if world.navigation.is_walkable(position) {
                        egui::Color32::from_rgb(54, 93, 59)
                    } else {
                        egui::Color32::from_rgb(51, 57, 54)
                    }
                }
            };
            ui.painter().rect_filled(
                egui::Rect::from_min_max(egui::pos2(left, top), egui::pos2(right, bottom)),
                0.0,
                color,
            );
        }
    }
    if matches!(
        layer,
        WorldPreviewLayer::LandResources | WorldPreviewLayer::WaterResources
    ) {
        for resource in world.resources.iter().filter(|resource| {
            let water = resource.target_kind.as_str() == "target:fish";
            (layer == WorldPreviewLayer::WaterResources) == water
        }) {
            let highlighted =
                selected_resource_target.is_none_or(|target| target == &resource.target_kind);
            let mut color = match (resource.kind.as_str(), resource.target_kind.as_str()) {
                (_, "target:fish") => egui::Color32::from_rgb(83, 207, 229),
                (value, _) if value.contains("wood") => egui::Color32::from_rgb(46, 185, 76),
                (value, _) if value.contains("ore") => egui::Color32::from_rgb(190, 196, 207),
                (value, _) if value.contains("food") => egui::Color32::from_rgb(219, 82, 81),
                _ => egui::Color32::GOLD,
            };
            if !highlighted {
                color = egui::Color32::from_rgba_unmultiplied(
                    color.r() / 2,
                    color.g() / 2,
                    color.b() / 2,
                    100,
                );
            }
            ui.painter().circle_filled(
                preview_grid_point(rect, width, height, resource.position),
                if highlighted { 2.5 } else { 1.2 },
                color,
            );
        }
    }
    if layer == WorldPreviewLayer::Foliage {
        for foliage in &world.foliage {
            let highlighted = selected_foliage.is_none_or(|id| id == &foliage.layer);
            let color = match (foliage.habitat, highlighted) {
                (FoliageHabitat::Land, true) => egui::Color32::from_rgb(115, 235, 100),
                (FoliageHabitat::Underwater, true) => egui::Color32::from_rgb(92, 220, 225),
                (_, false) => egui::Color32::from_rgba_unmultiplied(145, 145, 145, 90),
            };
            ui.painter().circle_filled(
                preview_grid_point(rect, width, height, foliage.position),
                if highlighted { 1.8 } else { 1.0 },
                color,
            );
        }
    }
    if layer == WorldPreviewLayer::EnemyCamps
        && let Some(camp) = enemy_camp_layer
    {
        for z in 0..height {
            for x in 0..width {
                let horizontal_milli = (i32::from(x) * 2 + 1 - i32::from(width))
                    .unsigned_abs()
                    .saturating_mul(500);
                let vertical_milli = (i32::from(z) * 2 + 1 - i32::from(height))
                    .unsigned_abs()
                    .saturating_mul(500);
                let squared_distance =
                    u64::from(horizontal_milli).pow(2) + u64::from(vertical_milli).pow(2);
                let minimum_squared =
                    u64::from(camp.minimum_distance_from_centre_milli_cells).pow(2);
                let in_bounds = horizontal_milli >= camp.minimum_absolute_offset_milli_cells[0]
                    && vertical_milli >= camp.minimum_absolute_offset_milli_cells[1]
                    && horizontal_milli <= camp.maximum_absolute_offset_milli_cells[0]
                    && vertical_milli <= camp.maximum_absolute_offset_milli_cells[1]
                    && squared_distance >= minimum_squared;
                if in_bounds && world.navigation.is_walkable(GridPos { x, z }) {
                    ui.painter().circle_filled(
                        preview_grid_point(rect, width, height, GridPos { x, z }),
                        1.1,
                        egui::Color32::from_rgba_unmultiplied(230, 92, 76, 145),
                    );
                }
            }
        }
    }
    ui.label(match layer {
        WorldPreviewLayer::Elevation => "Blue is at/below water level; green through stone encodes terrain elevation.",
        WorldPreviewLayer::Navigation => "Green cells are walkable; charcoal cells are blocked.",
        WorldPreviewLayer::LandResources => "Green, grey, and red markers identify generated trees, ore, and berry bushes.",
        WorldPreviewLayer::WaterResources => "Cyan markers identify generated shoreline-water resources such as fish.",
        WorldPreviewLayer::Foliage => "Bright markers belong to the selected foliage layer; dim markers belong to other layers.",
        WorldPreviewLayer::EnemyCamps => "Red cells satisfy the selected camp layer's absolute-offset and centre-distance constraints; the runtime still applies spacing, occupancy, and maximum-count rules.",
    });
}

fn preview_grid_point(rect: egui::Rect, width: u16, height: u16, position: GridPos) -> egui::Pos2 {
    egui::pos2(
        rect.left() + (f32::from(position.x) + 0.5) * rect.width() / f32::from(width),
        rect.top() + (f32::from(position.z) + 0.5) * rect.height() / f32::from(height),
    )
}

fn terrain_preview_color(value: f32) -> egui::Color32 {
    let value = value.clamp(0.0, 1.0);
    if value < 0.55 {
        let t = value / 0.55;
        preview_lerp_color(
            egui::Color32::from_rgb(60, 113, 63),
            egui::Color32::from_rgb(157, 154, 91),
            t,
        )
    } else {
        let t = (value - 0.55) / 0.45;
        preview_lerp_color(
            egui::Color32::from_rgb(157, 154, 91),
            egui::Color32::from_rgb(218, 220, 218),
            t,
        )
    }
}

fn preview_lerp_color(from: egui::Color32, to: egui::Color32, t: f32) -> egui::Color32 {
    let lerp = |start: u8, end: u8| {
        f32::from(start) / 255.0 + (f32::from(end) - f32::from(start)) * t / 255.0
    };
    egui::Rgba::from_rgb(
        lerp(from.r(), to.r()),
        lerp(from.g(), to.g()),
        lerp(from.b(), to.b()),
    )
    .into()
}
