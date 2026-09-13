pub(crate) fn normalized_content_name(value: &str) -> String {
    value
        .chars()
        .filter_map(|character| {
            if character.is_ascii_alphanumeric() {
                Some(character.to_ascii_lowercase())
            } else if character == ' ' || character == '-' || character == '_' {
                Some('_')
            } else {
                None
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_owned()
}

pub(crate) fn resolve_technology_id(
    content: &ContentCatalog,
    requested: &StableId,
) -> Option<StableId> {
    if content.technology.nodes.contains_key(requested) {
        return Some(requested.clone());
    }
    let requested_name = normalized_content_name(requested.as_str());
    content
        .technology
        .nodes
        .iter()
        .find(|(_, technology)| normalized_content_name(&technology.display_name) == requested_name)
        .map(|(id, _)| id.clone())
}

pub(crate) fn resolve_active_technology_vote_option(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    requested: &StableId,
) -> Option<StableId> {
    let vote = simulation.active_vote.as_ref()?;
    let options = technology_vote_options(vote);
    if let Ok(index) = requested.as_str().parse::<usize>() {
        return index
            .checked_sub(1)
            .and_then(|index| options.get(index))
            .copied()
            .cloned();
    }
    let technology = resolve_technology_id(content, requested)?;
    options.contains(&&technology).then_some(technology)
}

pub(crate) fn eligible_technology_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> Vec<StableId> {
    let town_hall = StableId::new("building:townhall").expect("static building ID");
    let global_age = unlocked_building_age(content, simulation, &town_hall);
    let town_hall_level = completed_building_level(content, simulation, &town_hall);
    content
        .technology
        .nodes
        .iter()
        .filter(|(id, node)| {
            let technology_age = authored_technology_age(&node.age);
            let is_global_age_up = node.aged_buildings.contains(&town_hall);
            let age_is_available = if is_global_age_up {
                technology_age == global_age.saturating_add(1)
                    && town_hall_level >= u16::from(technology_age)
            } else {
                technology_age <= global_age
            };
            !node.unavailable
                && age_is_available
                && !simulation.unlocked_technology.contains(*id)
                && !simulation
                    .active_goals
                    .iter()
                    .any(|goal| goal.technology == **id)
                && simulation
                    .active_vote
                    .as_ref()
                    .is_none_or(|vote| !technology_vote_options(vote).contains(id))
                && node
                    .prerequisites
                    .iter()
                    .all(|required| simulation.unlocked_technology.contains(required))
        })
        .map(|(id, _)| id.clone())
        .collect()
}

pub(crate) fn authored_technology_age(value: &str) -> u8 {
    value
        .split_whitespace()
        .last()
        .and_then(|value| value.parse::<u8>().ok())
        .unwrap_or(1)
        .max(1)
}

pub(crate) fn completed_building_level(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
) -> u16 {
    let Some(archetype) = content
        .buildings
        .get(building)
        .map(|definition| &definition.archetype)
    else {
        return 0;
    };
    simulation
        .buildings
        .values()
        .filter(|state| state.complete && &state.archetype == archetype)
        .map(|state| state.level)
        .max()
        .unwrap_or(0)
}

pub(crate) fn technology_ballot_rank(
    world_seed: u64,
    unlocked_count: usize,
    salt: u64,
    id: &StableId,
) -> u64 {
    let mut hash = world_seed
        ^ salt
        ^ u64::try_from(unlocked_count)
            .unwrap_or(u64::MAX)
            .wrapping_mul(0x9E37_79B9_7F4A_7C15);
    for byte in id.as_str().bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash
}

pub(crate) fn technology_node_depth(
    content: &ContentCatalog,
    id: &StableId,
    memo: &mut BTreeMap<StableId, u32>,
    visiting: &mut BTreeSet<StableId>,
) -> u32 {
    if let Some(depth) = memo.get(id) {
        return *depth;
    }
    if !visiting.insert(id.clone()) {
        return 0;
    }
    let depth = content.technology.nodes.get(id).map_or(0, |node| {
        node.prerequisites
            .iter()
            .filter(|required| content.technology.nodes.contains_key(*required))
            .map(|required| technology_node_depth(content, required, memo, visiting))
            .max()
            .map_or(0, |depth| depth.saturating_add(1))
    });
    visiting.remove(id);
    memo.insert(id.clone(), depth);
    depth
}

pub(crate) fn technology_depths(content: &ContentCatalog) -> BTreeMap<StableId, u32> {
    let mut memo = BTreeMap::new();
    for id in content.technology.nodes.keys() {
        technology_node_depth(content, id, &mut memo, &mut BTreeSet::new());
    }
    memo
}

pub(crate) fn take_ranked_technology(
    eligible: &mut Vec<StableId>,
    world_seed: u64,
    unlocked_count: usize,
    salt: u64,
    required_depth: Option<u32>,
    depths: &BTreeMap<StableId, u32>,
) -> Option<StableId> {
    let index = eligible
        .iter()
        .enumerate()
        .filter(|(_, id)| required_depth.is_none_or(|depth| depths.get(*id) == Some(&depth)))
        .min_by_key(|(_, id)| {
            (
                technology_ballot_rank(world_seed, unlocked_count, salt, id),
                (*id).clone(),
            )
        })
        .map(|(index, _)| index)?;
    Some(eligible.remove(index))
}

pub(crate) fn technology_ballot_options(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> Vec<StableId> {
    let mut eligible = eligible_technology_ids(content, simulation);
    let depths = technology_depths(content);
    let deepest = eligible
        .iter()
        .filter_map(|id| depths.get(id))
        .max()
        .copied();
    let shallowest = eligible
        .iter()
        .filter_map(|id| depths.get(id))
        .min()
        .copied();
    let mut options = Vec::with_capacity(TECHNOLOGY_VOTE_OPTION_COUNT);
    if let Some(technology) = take_ranked_technology(
        &mut eligible,
        simulation.world_seed,
        simulation.unlocked_technology.len(),
        0x0073_7065_6369_616c,
        deepest,
        &depths,
    ) {
        options.push(technology);
    }
    if let Some(technology) = take_ranked_technology(
        &mut eligible,
        simulation.world_seed,
        simulation.unlocked_technology.len(),
        0x6675_6e64_616d_656e,
        shallowest,
        &depths,
    ) {
        options.push(technology);
    }
    if let Some(technology) = take_ranked_technology(
        &mut eligible,
        simulation.world_seed,
        simulation.unlocked_technology.len(),
        0x7261_6e64_6f6d,
        None,
        &depths,
    ) {
        options.push(technology);
    }
    options
}

pub(crate) fn technology_vote_depth_tag(option_count: usize, index: u8) -> &'static str {
    match (option_count, index) {
        (1, 0) => "Both",
        (_, 0) => "Specialized",
        (_, 1) => "Fundamental",
        _ => "",
    }
}

pub(crate) fn start_scheduled_technology_vote(
    mut simulation: ResMut<SimulationRuntime>,
    content: Res<RuntimeContent>,
) {
    if simulation.0.active_vote.is_some()
        || !simulation.0.active_goals.is_empty()
        || simulation.0.active_event.is_some()
    {
        return;
    }
    if !simulation
        .0
        .technology_vote_cooldown_seconds
        .is_some_and(|remaining| remaining <= f32::EPSILON)
    {
        return;
    }
    let options = technology_ballot_options(&content.0, &simulation.0);
    simulation.0.technology_vote_cooldown_seconds = None;
    if options.is_empty() {
        return;
    }
    if let Err(error) = simulation
        .0
        .start_technology_ballot(options, TECHNOLOGY_VOTE_DURATION_SECONDS)
    {
        warn!(%error, "scheduled technology vote could not start");
    }
}

pub(crate) fn technology_vote_announcement(
    content: &ContentCatalog,
    vote: &stream_town_domain::TechVote,
) -> String {
    let options = technology_vote_options(vote);
    let labels = options
        .iter()
        .enumerate()
        .map(|(index, technology)| {
            let label = content.technology.nodes.get(*technology).map_or_else(
                || technology.to_string(),
                |node| compact_technology_label(&node.display_name).replace('\n', " "),
            );
            format!("{}: {label}", index + 1)
        })
        .collect::<Vec<_>>()
        .join("; ");
    let commands = (1..=options.len())
        .map(|index| format!("!vote {index}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("Technology vote started! {labels}. Vote with {commands}.")
}

pub(crate) fn announce_technology_vote(
    simulation: Res<SimulationRuntime>,
    content: Res<RuntimeContent>,
    connection: Res<TwitchConnection>,
    mut announced_vote: Local<Option<String>>,
) {
    let Some(vote) = simulation.0.active_vote.as_ref() else {
        *announced_vote = None;
        return;
    };
    let signature = technology_vote_options(vote)
        .iter()
        .map(|technology| technology.as_str())
        .collect::<Vec<_>>()
        .join("|");
    if announced_vote.as_deref() == Some(signature.as_str()) || connection.transport.is_none() {
        return;
    }
    let message = technology_vote_announcement(&content.0, vote);
    info!(announcement = %message, "technology vote announced");
    send_command_feedback(&connection, message);
    *announced_vote = Some(signature);
}

pub(crate) fn unlock_reachable_technologies(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    age_one_only: bool,
) -> usize {
    let mut unlocked = 0;
    loop {
        let available = eligible_technology_ids(content, simulation)
            .into_iter()
            .filter(|id| {
                !age_one_only
                    || content.technology.nodes[id]
                        .age
                        .trim()
                        .eq_ignore_ascii_case("Age 1")
            })
            .collect::<Vec<_>>();
        if available.is_empty() {
            break;
        }
        unlocked += available.len();
        simulation.unlocked_technology.extend(available);
    }
    unlocked
}

pub(crate) fn maximum_building_level(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
) -> u16 {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .filter_map(|technology| technology.building_level_caps.get(building).copied())
        .max()
        .unwrap_or(1)
}

pub(crate) fn building_is_unlocked(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
) -> bool {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .any(|technology| technology.unlocked_buildings.contains(building))
}

pub(crate) fn technology_stat_boost_percent(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    role: &StableId,
    stat: &StableId,
) -> i32 {
    if role.as_str() == "role:enemy" {
        return 0;
    }
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .fold(0_i32, |total, technology| {
            // Unity records PlayerRole.Count boosts in GlobalStatModifiers, but
            // PlayerRoleData only consumes role modifiers and per-character
            // passives. No shipping caller copies the global technology table
            // into those passives, so activating it here changes live balance.
            total.saturating_add(
                technology
                    .role_stat_boost_percent
                    .get(role)
                    .and_then(|stats| stats.get(stat))
                    .copied()
                    .unwrap_or_default(),
            )
        })
}

pub(crate) fn building_cost_reduction_percent(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
) -> i32 {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .fold(0_i32, |total, technology| {
            total
                .saturating_add(technology.global_building_cost_reduction_percent)
                .saturating_add(
                    technology
                        .building_cost_reduction_percent
                        .get(building)
                        .copied()
                        .unwrap_or_default(),
                )
        })
}

pub(crate) fn storage_boost_percent(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    resource: &StableId,
) -> i32 {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .filter_map(|technology| technology.storage_boost_percent.get(resource).copied())
        .fold(0_i32, i32::saturating_add)
}

pub(crate) fn unlocked_building_age(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
) -> u8 {
    simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .filter(|technology| technology.aged_buildings.contains(building))
        .map(|technology| authored_technology_age(&technology.age))
        .max()
        .unwrap_or(1)
}

pub(crate) fn building_age(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building: &StableId,
    level: u16,
) -> u8 {
    let town_hall = StableId::new("building:townhall").expect("static building ID");
    let local_age = simulation
        .unlocked_technology
        .iter()
        .filter_map(|technology| content.technology.nodes.get(technology))
        .filter(|technology| technology.aged_buildings.contains(building))
        .filter(|technology| {
            technology
                .building_level_caps
                .get(building)
                .is_none_or(|minimum_level| level >= *minimum_level)
        })
        .map(|technology| authored_technology_age(&technology.age))
        .max()
        .unwrap_or(1);
    let global_age = unlocked_building_age(content, simulation, &town_hall);
    local_age.min(global_age)
}

pub(crate) fn percentage_adjusted(base: u32, percent: i32) -> u32 {
    let base = i64::from(base);
    let adjusted = base.saturating_add(base.saturating_mul(i64::from(percent)) / 100);
    u32::try_from(adjusted.clamp(0, i64::from(u32::MAX))).unwrap_or(u32::MAX)
}

pub(crate) fn percentage_reduced(base: u32, reduction_percent: i32) -> u32 {
    percentage_adjusted(base, reduction_percent.saturating_neg())
}

pub(crate) fn percentage_adjusted_i64(base: i64, percent: i32) -> i64 {
    base.saturating_add(base.saturating_mul(i64::from(percent)) / 100)
}

pub(crate) fn milli_units_as_f32(value: u32) -> f32 {
    f32::from(u16::try_from(value.min(u32::from(u16::MAX))).expect("clamped milli-unit fits u16"))
        / 1_000.0
}

pub(crate) fn resource_storage_capacity(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    resource: &StableId,
) -> u32 {
    let Some(base) = config
        .gameplay
        .base_town_resource_capacity
        .get(resource)
        .copied()
    else {
        return u32::MAX;
    };
    let boost = storage_boost_percent(content, simulation, resource);
    simulation
        .buildings
        .values()
        .filter(|building| building.complete)
        .filter_map(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            Some((definition, building.level))
        })
        .flat_map(|(definition, level)| {
            definition
                .storage
                .iter()
                .filter(move |storage| storage.resource == *resource)
                .map(move |storage| {
                    let amount = storage.base_amount.saturating_add(
                        storage
                            .increment_amount
                            .saturating_mul(u32::from(level.saturating_sub(1))),
                    );
                    percentage_adjusted(amount, boost)
                })
        })
        .fold(base, u32::saturating_add)
}

pub(crate) fn take_passive_building_income_delta(
    accumulated: &mut Duration,
    delta: Duration,
) -> Option<Duration> {
    *accumulated = accumulated.saturating_add(delta);
    if *accumulated < PASSIVE_BUILDING_INCOME_INTERVAL {
        return None;
    }
    Some(std::mem::take(accumulated))
}

pub(crate) fn apply_passive_building_income(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    delta: Duration,
) {
    let mut rates = BTreeMap::<(StableId, StableId), u64>::new();
    for building in simulation
        .buildings
        .values()
        .filter(|building| building.complete)
    {
        let Some(definition) = building_def_for_archetype(content, &building.archetype) else {
            continue;
        };
        for (resource, rate) in passive_resource_rate_milli_per_second(definition, building.level) {
            let entry = rates.entry((building.id.clone(), resource)).or_default();
            *entry = entry.saturating_add(rate);
        }
    }

    let mut active = BTreeMap::<StableId, BTreeSet<StableId>>::new();
    for (building, resource) in rates.keys() {
        active
            .entry(building.clone())
            .or_default()
            .insert(resource.clone());
    }
    simulation
        .passive_resource_accumulators
        .retain(|building, resources| {
            let Some(active_resources) = active.get(building) else {
                return false;
            };
            resources.retain(|resource, _| active_resources.contains(resource));
            !resources.is_empty()
        });

    let delta_nanos = delta.as_nanos();
    if delta_nanos == 0 {
        return;
    }
    for ((building, resource), rate) in rates {
        let previous = simulation
            .passive_resource_accumulators
            .entry(building)
            .or_default()
            .entry(resource.clone())
            .or_default();
        let accumulated =
            u128::from(*previous).saturating_add(u128::from(rate).saturating_mul(delta_nanos));
        // Unity uses a strict `> 1` threshold, so exactly one accumulated unit waits
        // until the next positive update rather than being emitted immediately.
        let generated = if accumulated > PASSIVE_RESOURCE_FIXED_POINT_DENOMINATOR {
            accumulated / PASSIVE_RESOURCE_FIXED_POINT_DENOMINATOR
        } else {
            0
        };
        *previous = u64::try_from(
            accumulated
                .saturating_sub(generated.saturating_mul(PASSIVE_RESOURCE_FIXED_POINT_DENOMINATOR)),
        )
        .expect("passive resource remainder fits u64");
        if generated == 0 {
            continue;
        }
        let generated = u32::try_from(generated).unwrap_or(u32::MAX);
        let capacity = resource_storage_capacity(config, content, simulation, &resource);
        let current = simulation
            .town_resources
            .get(&resource)
            .copied()
            .unwrap_or_default();
        simulation.town_resources.insert(
            resource.clone(),
            current.saturating_add(generated).min(capacity),
        );
        let _ = simulation.record_objective_event(
            &content.objectives,
            &ObjectiveEvent::ResourceGained {
                resource,
                amount: generated,
            },
        );
    }
}

pub(crate) fn building_construction_cost(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building_id: &StableId,
    building: &BuildingDef,
) -> BTreeMap<StableId, u32> {
    let reduction = building_cost_reduction_percent(content, simulation, building_id);
    let existing_count = simulation
        .buildings
        .values()
        .filter(|existing| existing.archetype == building.archetype)
        .count();
    let existing_count = u64::try_from(existing_count).unwrap_or(u64::MAX);
    let multiplier_per_thousand = 1_000_u64.saturating_add(existing_count.saturating_mul(
        u64::from(building.construction_cost_multiplier_per_thousand),
    ));
    building
        .cost
        .iter()
        .map(|(resource, cost)| {
            let scaled = u64::from(*cost)
                .saturating_mul(multiplier_per_thousand)
                .saturating_add(999)
                / 1_000;
            (
                resource.clone(),
                percentage_reduced(u32::try_from(scaled).unwrap_or(u32::MAX), reduction),
            )
        })
        .collect()
}

pub(crate) fn building_upgrade_cost(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    building_id: &StableId,
    building: &BuildingDef,
    current_level: u16,
    target_level: u16,
) -> BTreeMap<StableId, u32> {
    let target_level = target_level.max(current_level);
    let reduction = building_cost_reduction_percent(content, simulation, building_id);
    building
        .cost
        .iter()
        .map(|(resource, base)| {
            let scaled = (u64::from(current_level)..u64::from(target_level)).fold(
                0_u64,
                |total, target_minus_one| {
                    let rung = u64::from(*base)
                        .saturating_mul(target_minus_one)
                        .saturating_mul(u64::from(
                            building.upgrade_cost_per_target_level_per_thousand,
                        ))
                        .saturating_add(999)
                        / 1_000;
                    total.saturating_add(rung)
                },
            );
            (
                resource.clone(),
                percentage_reduced(u32::try_from(scaled).unwrap_or(u32::MAX), reduction),
            )
        })
        .collect()
}
