pub(crate) fn nearest_walkable(world: &GeneratedWorld, desired: GridPos) -> Option<GridPos> {
    let desired = GridPos {
        x: desired.x.min(world.navigation.width() - 1),
        z: desired.z.min(world.navigation.height() - 1),
    };
    if world.navigation.is_walkable(desired) {
        return Some(desired);
    }
    let limit = world.navigation.width().max(world.navigation.height());
    for radius in 1..limit {
        for z in desired.z.saturating_sub(radius)
            ..=desired
                .z
                .saturating_add(radius)
                .min(world.navigation.height() - 1)
        {
            for x in desired.x.saturating_sub(radius)
                ..=desired
                    .x
                    .saturating_add(radius)
                    .min(world.navigation.width() - 1)
            {
                let candidate = GridPos { x, z };
                if world.navigation.is_walkable(candidate) {
                    return Some(candidate);
                }
            }
        }
    }
    None
}

pub(crate) fn restored_actor_position(
    world: &GeneratedWorld,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    kind: &ActorKind,
    desired: GridPos,
) -> Option<GridPos> {
    let desired = GridPos {
        x: desired.x.min(world.navigation.width() - 1),
        z: desired.z.min(world.navigation.height() - 1),
    };
    if *kind == ActorKind::Player
        && completed_player_gate_cells(content, simulation).contains(&desired)
    {
        Some(desired)
    } else {
        nearest_walkable(world, desired)
    }
}

pub(crate) fn initial_actor_position(
    world: &GeneratedWorld,
    excluded: GridPos,
    actor_index: usize,
) -> Option<GridPos> {
    let mut found = 0;
    for z in 0..world.navigation.height() {
        for x in 0..world.navigation.width() {
            let position = GridPos { x, z };
            if position == excluded || !world.navigation.is_walkable(position) {
                continue;
            }
            if found == actor_index {
                return Some(position);
            }
            found += 1;
        }
    }
    None
}

pub(crate) fn connected_actor_positions(
    world: &GeneratedWorld,
    start: GridPos,
    excluded: GridPos,
    count: u16,
) -> Vec<GridPos> {
    let mut positions = Vec::with_capacity(usize::from(count));
    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::from([start]);
    while let Some(position) = queue.pop_front() {
        if !visited.insert(position) || !world.navigation.is_walkable(position) {
            continue;
        }
        if position != excluded {
            positions.push(position);
            if positions.len() >= usize::from(count) {
                break;
            }
        }
        queue.extend(
            world
                .navigation
                .walkable_neighbours(position)
                .into_iter()
                .flatten()
                .map(|(neighbour, _)| neighbour),
        );
    }
    positions
}

pub(crate) fn resource_for_role(content: &ContentCatalog, role: &StableId) -> Option<StableId> {
    content.roles.get(role)?.resource.clone()
}

pub(crate) fn actor_resource_storage_has_room(
    config: &GameConfig,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
) -> bool {
    if simulation.active_community_event == Some(CommunityEvent::Market) {
        return true;
    }
    let Some(resource) = resource_for_role(content, &actor.role) else {
        return true;
    };
    simulation
        .town_resources
        .get(&resource)
        .copied()
        .unwrap_or_default()
        < resource_storage_capacity(config, content, simulation, &resource)
}

pub(crate) fn actor_remaining_carry_capacity(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
) -> u32 {
    let carried = resource_for_role(content, &actor.role)
        .and_then(|resource| actor.inventory.get(&resource).copied())
        .unwrap_or_default();
    let capacity = effective_role_stats(content, simulation, actor)
        .map(|stats| stats.carry_capacity)
        .filter(|capacity| *capacity > 0)
        .unwrap_or(25);
    capacity.saturating_sub(carried)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct EffectiveRoleStats {
    pub(crate) level: u16,
    pub(crate) experience: u32,
    pub(crate) required_experience: u32,
    pub(crate) experience_multiplier_per_thousand: u32,
    pub(crate) action_amount: u32,
    pub(crate) action_milliseconds: u32,
    pub(crate) action_range_milli_cells: u32,
    pub(crate) max_health: u32,
    pub(crate) health_regen_milli_per_second: i64,
    pub(crate) damage_reduction_percent: u32,
    pub(crate) movement_speed_milli_cells_per_second: u32,
    pub(crate) carry_capacity: u32,
}

pub(crate) fn leveled_whole_stat(base: u32, per_level_milli: u32, level: u16) -> u32 {
    let increase =
        u64::from(per_level_milli).saturating_mul(u64::from(level.saturating_sub(1))) / 1_000;
    base.saturating_add(u32::try_from(increase).unwrap_or(u32::MAX))
}

pub(crate) fn leveled_percentage_stat(base: u32, per_level_per_thousand: u32, level: u16) -> u32 {
    let increase = u64::from(base)
        .saturating_mul(u64::from(per_level_per_thousand))
        .saturating_mul(u64::from(level.saturating_sub(1)))
        / 1_000;
    base.saturating_add(u32::try_from(increase).unwrap_or(u32::MAX))
}

pub(crate) fn building_level_bonus_per_thousand(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
    bonus: impl Fn(&BuildingDef) -> u32,
) -> u32 {
    actor
        .station
        .as_ref()
        .and_then(|station| simulation.buildings.get(station))
        .filter(|station| station.complete && station.health > 0)
        .and_then(|station| {
            let definition = building_def_for_archetype(content, &station.archetype)?;
            definition.station.as_ref()?;
            Some(bonus(definition).saturating_mul(u32::from(station.level.saturating_sub(1))))
        })
        .unwrap_or_default()
}

pub(crate) fn global_gather_rate_bonus_per_thousand(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> u32 {
    simulation
        .buildings
        .values()
        .filter(|building| building.complete && building.health > 0)
        .filter_map(|building| {
            building_def_for_archetype(content, &building.archetype).map(|definition| {
                definition
                    .global_gather_rate_bonus_per_level_per_thousand
                    .saturating_mul(u32::from(building.level.saturating_sub(1)))
            })
        })
        .fold(0_u32, u32::saturating_add)
}

pub(crate) fn military_kill_experience_bonus_per_thousand(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
) -> u32 {
    simulation
        .buildings
        .values()
        .filter(|building| building.complete && building.health > 0)
        .filter_map(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            Some(
                definition
                    .kill_experience_bonus_per_level_per_thousand
                    .saturating_mul(u32::from(building.level.saturating_sub(1))),
            )
        })
        .fold(0_u32, u32::saturating_add)
}

pub(crate) fn role_progress(actor: &ActorState) -> stream_town_domain::RoleProgress {
    actor
        .role_progression
        .get(&actor.role)
        .copied()
        .unwrap_or_default()
}

pub(crate) fn effective_role_stats(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
) -> Option<EffectiveRoleStats> {
    let definition = content.roles.get(&actor.role)?;
    let progress = role_progress(actor);
    let levels = u32::from(progress.level.saturating_sub(1));
    let stat = |name: &str| {
        technology_stat_boost_percent(
            content,
            simulation,
            &actor.role,
            &StableId::new(format!("stat:{name}")).expect("static stat IDs are valid"),
        )
    };
    let base_action_milliseconds = definition
        .base_action_milliseconds
        .saturating_sub(
            definition
                .action_milliseconds_reduction_per_level
                .saturating_mul(levels),
        )
        .max(content.progression.minimum_action_milliseconds);
    // Unity retains ActionRangePerLevel in RoleData, but PlayerRoleData gates it
    // behind `_ranged`; the shipping constructor never sets that TODO field.
    // Keep the converted value available to tools without activating dead data.
    let base_range = definition.base_action_range_milli_cells;
    let base_movement = definition
        .base_movement_speed_milli_cells_per_second
        .saturating_add(
            definition
                .movement_speed_milli_cells_per_second_per_level
                .saturating_mul(levels),
        );
    let base_defense = leveled_whole_stat(
        u32::try_from(definition.base_damage_reduction_percent.max(0)).unwrap_or_default(),
        definition.damage_reduction_milli_percent_per_level,
        progress.level,
    );
    let base_regen = i64::from(definition.base_health_regen_per_second)
        .saturating_mul(1_000)
        .saturating_add(
            i64::from(definition.health_regen_milli_per_second_per_level)
                .saturating_mul(i64::from(levels)),
        );
    let max_health = percentage_adjusted(
        leveled_percentage_stat(
            definition.base_health,
            definition.health_bonus_per_level_per_thousand,
            progress.level,
        ),
        stat("health"),
    )
    .max(1);
    let max_health = if is_guardhouse_defender_id(&actor.id) {
        (max_health / 2).max(1)
    } else {
        max_health
    };

    let station_experience_bonus =
        building_level_bonus_per_thousand(content, simulation, actor, |building| {
            building
                .station
                .as_ref()
                .map_or(0, |station| station.experience_bonus_per_level_per_thousand)
        });
    let experience_multiplier_per_thousand = u32::try_from(
        u64::from(definition.experience_multiplier_per_thousand).saturating_mul(u64::from(
            1_000_u32.saturating_add(station_experience_bonus),
        )) / 1_000,
    )
    .unwrap_or(u32::MAX);
    let gather_rate_bonus = definition.resource.as_ref().map_or(0, |_| {
        global_gather_rate_bonus_per_thousand(content, simulation)
    });
    let gathered_action_milliseconds = u32::try_from(
        u64::from(base_action_milliseconds).saturating_mul(1_000)
            / u64::from(1_000_u32.saturating_add(gather_rate_bonus)),
    )
    .unwrap_or(u32::MAX)
    .max(content.progression.minimum_action_milliseconds);

    Some(EffectiveRoleStats {
        level: progress.level,
        experience: progress.experience,
        required_experience: stream_town_domain::required_role_experience(
            progress.level,
            content.progression.role_experience_curve_level_span,
            content.progression.role_experience_curve_maximum,
        ),
        experience_multiplier_per_thousand,
        action_amount: percentage_adjusted(
            leveled_whole_stat(
                definition.base_action_amount,
                definition.action_amount_per_level_milli,
                progress.level,
            ),
            stat("action_amount"),
        ),
        action_milliseconds: percentage_reduced(gathered_action_milliseconds, stat("action_speed"))
            .max(content.progression.minimum_action_milliseconds),
        action_range_milli_cells: percentage_adjusted(base_range, stat("action_range")),
        max_health,
        health_regen_milli_per_second: percentage_adjusted_i64(base_regen, stat("health_regen")),
        damage_reduction_percent: percentage_adjusted(base_defense, stat("defense")),
        movement_speed_milli_cells_per_second: percentage_adjusted(
            base_movement,
            stat("movement_speed"),
        ),
        carry_capacity: percentage_adjusted(
            leveled_whole_stat(
                definition.base_carry_capacity,
                definition.carry_capacity_per_level_milli,
                progress.level,
            ),
            stat("resource_carry"),
        ),
    })
}

pub(crate) fn role_action_range_milli_cells(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
) -> u32 {
    if let Some(enemy) =
        actor_archetype(content, actor).and_then(|archetype| archetype.enemy.as_ref())
    {
        return enemy.action_range_milli_cells;
    }
    effective_role_stats(content, simulation, actor)
        .map_or(1_000, |stats| stats.action_range_milli_cells)
}

pub(crate) fn within_milli_cell_range(
    left: GridPos,
    right: GridPos,
    range_milli_cells: u32,
) -> bool {
    grid_distance_squared_milli_cells(left, right)
        <= u128::from(range_milli_cells).saturating_mul(u128::from(range_milli_cells))
}

pub(crate) fn grid_distance_squared_milli_cells(left: GridPos, right: GridPos) -> u128 {
    u128::from(grid_distance_squared(left, right)).saturating_mul(1_000_000)
}

pub(crate) fn actor_target_size_milli_cells(content: &ContentCatalog, actor: &ActorState) -> u32 {
    actor_archetype(content, actor).map_or(0, |archetype| archetype.target_size_milli_cells)
}

pub(crate) fn building_target_size_milli_cells(
    content: &ContentCatalog,
    building: &BuildingState,
) -> u32 {
    content
        .archetypes
        .get(&building.archetype)
        .map_or(0, |archetype| archetype.target_size_milli_cells)
}

pub(crate) fn within_actor_attack_range(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    attacker: &ActorState,
    target: &ActorState,
    current: GridPos,
) -> bool {
    let range = u128::from(role_action_range_milli_cells(content, simulation, attacker));
    let target_size = u128::from(actor_target_size_milli_cells(content, target));
    // Unity's shipping action states compare squared world distance against
    // `(range * 2[.5]) + SizeSqr` (rather than squaring the range). After
    // converting two world units to one cell, the coefficients are 1.0 for
    // enemies and 1.25 for players.
    let range_term = if attacker.role.as_str() == "role:enemy" {
        range.saturating_mul(1_000)
    } else {
        range.saturating_mul(1_250)
    };
    grid_distance_squared_milli_cells(current, target.position)
        <= range_term.saturating_add(target_size.saturating_mul(target_size))
}

pub(crate) fn within_actor_heal_range(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    healer: &ActorState,
    target: &ActorState,
    current: GridPos,
) -> bool {
    let range = u128::from(role_action_range_milli_cells(content, simulation, healer));
    let target_size = u128::from(actor_target_size_milli_cells(content, target));
    grid_distance_squared_milli_cells(current, target.position)
        <= range
            .saturating_mul(1_000)
            .saturating_add(target_size.saturating_mul(target_size))
}

pub(crate) fn within_actor_heal_hysteresis_range(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    healer: &ActorState,
    target: &ActorState,
    current: GridPos,
) -> bool {
    let range = u128::from(role_action_range_milli_cells(content, simulation, healer));
    let target_size = u128::from(actor_target_size_milli_cells(content, target));
    let normal_squared = range
        .saturating_mul(1_000)
        .saturating_add(target_size.saturating_mul(target_size));
    grid_distance_squared_milli_cells(current, target.position).saturating_mul(10_000)
        <= normal_squared
            .saturating_mul(HEALING_HYSTERESIS_PERCENT)
            .saturating_mul(HEALING_HYSTERESIS_PERCENT)
}

pub(crate) fn retained_healing_goal(
    goal: &AgentGoal,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor_id: &StableId,
    current: GridPos,
) -> Option<(AgentGoal, GridPos)> {
    let AgentGoal::Heal(target_id) = goal else {
        return None;
    };
    let healer = simulation.actors.get(actor_id)?;
    let target = simulation.actors.get(target_id)?;
    (healer.alive
        && target.alive
        && target.role.as_str() != "role:enemy"
        && target.health < target.max_health
        && within_actor_heal_hysteresis_range(content, simulation, healer, target, current))
    .then(|| (AgentGoal::Heal(target_id.clone()), current))
}

pub(crate) fn citizen_should_retreat(actor: &ActorState, was_retreating: bool) -> bool {
    if !actor.alive || actor.role.as_str() == "role:enemy" || actor.max_health <= 0 {
        return false;
    }
    if was_retreating {
        actor.health < actor.max_health
    } else {
        i64::from(actor.health.max(0)).saturating_mul(100)
            < i64::from(actor.max_health).saturating_mul(i64::from(RETREAT_HEALTH_PERCENT))
    }
}

pub(crate) fn within_enemy_building_attack_range(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    attacker: &ActorState,
    building: &BuildingState,
    current: GridPos,
) -> bool {
    let range = u128::from(role_action_range_milli_cells(content, simulation, attacker));
    let target_size = u128::from(building_target_size_milli_cells(content, building));
    grid_distance_squared_milli_cells(current, building_visual_grid(content, building))
        <= range
            .saturating_mul(1_000)
            .saturating_add(target_size.saturating_mul(target_size))
}

pub(crate) fn within_building_work_range(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    builder: &ActorState,
    building: &BuildingState,
    current: GridPos,
) -> bool {
    if building_def_for_archetype(content, &building.archetype).is_some_and(|definition| {
        let footprint = rotated_footprint(definition.footprint, building.rotation_quarter_turns);
        let maximum = GridPos {
            x: building
                .position
                .x
                .saturating_add(footprint[0].saturating_sub(1)),
            z: building
                .position
                .z
                .saturating_add(footprint[1].saturating_sub(1)),
        };
        let dx = if current.x < building.position.x {
            building.position.x - current.x
        } else {
            current.x.saturating_sub(maximum.x)
        };
        let dz = if current.z < building.position.z {
            building.position.z - current.z
        } else {
            current.z.saturating_sub(maximum.z)
        };
        (dx != 0 || dz != 0) && dx <= 1 && dz <= 1
    }) {
        // Every cell offered by `building_approaches` must also be a legal
        // action cell. Previously corner approaches to one-cell paths and
        // walls were planned successfully, but rejected here after the build
        // animation, leaving builders in a permanent no-progress loop.
        return true;
    }
    let range = u128::from(role_action_range_milli_cells(content, simulation, builder));
    let target_size = u128::from(building_target_size_milli_cells(content, building));
    let maximum = range
        .saturating_mul(5)
        .div_ceil(2)
        .saturating_add(target_size);
    grid_distance_squared_milli_cells(current, building_visual_grid(content, building))
        <= maximum.saturating_mul(maximum)
}

pub(crate) fn is_combat_role(role: &StableId) -> bool {
    matches!(
        role.as_str(),
        "role:defender"
            | "role:necromancer"
            | "role:paladin"
            | "role:ranger"
            | "role:ruler"
            | "role:soldier"
            | "role:wizard"
    )
}

pub(crate) fn is_healer_role(role: &StableId) -> bool {
    role.as_str() == "role:priest"
}

pub(crate) fn enemy_targets_kind(
    content: &ContentCatalog,
    actor: &ActorState,
    target: &str,
) -> bool {
    actor_archetype(content, actor)
        .and_then(|archetype| archetype.enemy.as_ref())
        .is_some_and(|enemy| {
            enemy.targets_all || enemy.target_kinds.iter().any(|id| id.as_str() == target)
        })
}

pub(crate) fn enemy_targets_buildings(content: &ContentCatalog, actor: &ActorState) -> bool {
    [
        "target:building",
        "target:damaged_building",
        "target:construction",
    ]
    .into_iter()
    .any(|target| enemy_targets_kind(content, actor, target))
}

pub(crate) fn enemy_target_search_range_milli_cells(
    content: &ContentCatalog,
    actor: &ActorState,
) -> Option<u32> {
    actor_archetype(content, actor)
        .and_then(|archetype| archetype.enemy.as_ref())
        .map(|enemy| enemy.target_search_range_milli_cells)
}

pub(crate) fn enemy_attacks_attacker(content: &ContentCatalog, actor: &ActorState) -> bool {
    actor_archetype(content, actor)
        .and_then(|archetype| archetype.enemy.as_ref())
        .is_some_and(|enemy| enemy.attack_attacker)
}

pub(crate) fn within_enemy_target_search(
    content: &ContentCatalog,
    actor: &ActorState,
    current: GridPos,
    target: GridPos,
) -> bool {
    enemy_target_search_range_milli_cells(content, actor)
        .is_some_and(|range| within_milli_cell_range(current, target, range))
}

pub(crate) fn is_ranged_role(role: &StableId) -> bool {
    matches!(
        role.as_str(),
        "role:necromancer" | "role:ranger" | "role:wizard"
    )
}

pub(crate) fn actor_combat_visual(role: &StableId) -> CombatVisualKind {
    match role.as_str() {
        "role:wizard" => CombatVisualKind::Fireball,
        "role:necromancer" => CombatVisualKind::Necrotic,
        "role:ranger" => CombatVisualKind::Arrow,
        _ => CombatVisualKind::Physical,
    }
}

pub(crate) fn actor_archetype<'a>(
    content: &'a ContentCatalog,
    actor: &ActorState,
) -> Option<&'a ArchetypeDef> {
    if let Some(archetype) = actor
        .archetype
        .as_ref()
        .and_then(|archetype| content.archetypes.get(archetype))
    {
        return Some(archetype);
    }
    let (kind, source) = if actor.role.as_str() == "role:enemy" {
        (ArchetypeKind::Enemy, "Enemy_Goblin.prefab")
    } else {
        (ArchetypeKind::Player, "Player_Character.prefab")
    };
    archetype_by_source(content, kind, source)
}

pub(crate) fn town_hall_grid_position(config: &GameConfig) -> GridPos {
    GridPos {
        x: (config.world.width / 2 + 4).min(config.world.width - 2),
        z: config.world.height / 2,
    }
}

pub(crate) fn town_hall_placement_position(config: &GameConfig, footprint: [u16; 2]) -> GridPos {
    let centre = town_hall_grid_position(config);
    GridPos {
        x: centre.x.saturating_sub(footprint[0] / 2),
        z: centre.z.saturating_sub(footprint[1] / 2),
    }
}

pub(crate) fn ensure_town_hall_state(
    content: &ContentCatalog,
    config: &GameConfig,
    simulation: &mut WorldSimulation,
) {
    let id = StableId::new("building:townhall").expect("static ID");
    if simulation.buildings.contains_key(&id) {
        return;
    }
    let definition = &content.buildings[&id];
    simulation.buildings.insert(
        id.clone(),
        BuildingState {
            id,
            archetype: definition.archetype.clone(),
            position: town_hall_placement_position(config, definition.footprint),
            rotation_quarter_turns: 0,
            level: 1,
            health: i32::try_from(building_base_max_health(content, definition))
                .unwrap_or(i32::MAX),
            complete: true,
        },
    );
}

#[derive(Clone, Copy)]
pub(crate) struct StationCandidate<'a> {
    pub(crate) id: &'a StableId,
    pub(crate) position: GridPos,
    pub(crate) definition: &'a StationDef,
}

pub(crate) fn ensure_actor_station(
    content: &ContentCatalog,
    simulation: &mut WorldSimulation,
    config: &GameConfig,
    actor_id: &StableId,
) {
    let replacement = simulation.actors.get(actor_id).and_then(|actor| {
        let best = best_station_id(content, simulation, config, &actor.role, actor.position);
        (actor.station != best).then_some(best)
    });
    if let Some(station) = replacement
        && let Some(actor) = simulation.actors.get_mut(actor_id)
    {
        actor.station = station;
    }
}

pub(crate) fn station_matches_role(
    station: &StationDef,
    role: &stream_town_domain::RoleDef,
) -> bool {
    station.accepts_all_roles
        || role
            .station_kinds
            .iter()
            .any(|kind| station.accepted_role_kinds.contains(kind))
}

pub(crate) fn station_supports_role_targets(
    station: &StationDef,
    role: &stream_town_domain::RoleDef,
) -> bool {
    station.targets_all
        || role.targets_all
        || role
            .target_kinds
            .iter()
            .any(|kind| station.target_kinds.contains(kind))
}

pub(crate) fn role_accepts_target(
    role: &stream_town_domain::RoleDef,
    target_kind: &StableId,
) -> bool {
    role.targets_all || role.target_kinds.contains(target_kind)
}

pub(crate) fn actor_accepts_resource(
    content: &ContentCatalog,
    actor: &ActorState,
    resource: &stream_town_domain::GeneratedResource,
) -> bool {
    content.roles.get(&actor.role).is_some_and(|role| {
        role.resource.as_ref() == Some(&resource.kind)
            && role_accepts_target(role, &resource.target_kind)
    })
}

pub(crate) fn station_candidate<'a>(
    content: &'a ContentCatalog,
    simulation: &'a WorldSimulation,
    config: &GameConfig,
    station_id: &'a StableId,
) -> Option<StationCandidate<'a>> {
    let Some(state) = simulation.buildings.get(station_id) else {
        if station_id.as_str() != "building:townhall" {
            return None;
        }
        let building = content.buildings.get(station_id)?;
        return Some(StationCandidate {
            id: station_id,
            position: town_hall_grid_position(config),
            definition: building.station.as_ref()?,
        });
    };
    let state = state.complete.then_some(state)?;
    let building = building_def_for_archetype(content, &state.archetype)?;
    let footprint = rotated_footprint(building.footprint, state.rotation_quarter_turns);
    Some(StationCandidate {
        id: station_id,
        position: GridPos {
            x: state.position.x.saturating_add(footprint[0] / 2),
            z: state.position.z.saturating_add(footprint[1] / 2),
        },
        definition: building.station.as_ref()?,
    })
}

pub(crate) fn best_station_id(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    config: &GameConfig,
    role_id: &StableId,
    from: GridPos,
) -> Option<StableId> {
    let role = content.roles.get(role_id)?;
    if role.station_kinds.is_empty() {
        return None;
    }
    let town_hall = StableId::new("building:townhall").expect("static building ID is valid");
    std::iter::once(&town_hall)
        .chain(
            simulation
                .buildings
                .keys()
                .filter(|id| id.as_str() != "building:townhall"),
        )
        .filter_map(|id| station_candidate(content, simulation, config, id))
        .filter(|station| {
            station_matches_role(station.definition, role)
                && station_supports_role_targets(station.definition, role)
        })
        .min_by_key(|station| {
            (
                grid_distance_squared(station.position, from),
                station.id.clone(),
            )
        })
        .map(|station| station.id.clone())
}

pub(crate) fn restored_town_hall_position(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    config: &GameConfig,
) -> GridPos {
    let town_hall = StableId::new("building:townhall").expect("static building ID is valid");
    station_candidate(content, simulation, config, &town_hall).map_or_else(
        || town_hall_grid_position(config),
        |station| station.position,
    )
}

pub(crate) fn assigned_station<'a>(
    content: &'a ContentCatalog,
    simulation: &'a WorldSimulation,
    config: &GameConfig,
    actor: &'a ActorState,
) -> Option<StationCandidate<'a>> {
    actor
        .station
        .as_ref()
        .and_then(|id| station_candidate(content, simulation, config, id))
        .filter(|station| {
            content.roles.get(&actor.role).is_some_and(|role| {
                station_matches_role(station.definition, role)
                    && station_supports_role_targets(station.definition, role)
            })
        })
}

#[cfg(test)]
pub(crate) fn station_deposit_approach(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    station: StationCandidate<'_>,
    current: GridPos,
) -> Option<GridPos> {
    simulation
        .buildings
        .get(station.id)
        .and_then(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            building_approach(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                current,
            )
        })
        .or_else(|| nearest_walkable(world, station.position))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn unoccupied_station_deposit_approach(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    station: StationCandidate<'_>,
    current: GridPos,
    actor: &StableId,
    occupied: &BTreeMap<GridPos, StableId>,
    reservations: &BTreeMap<GridPos, StableId>,
) -> Option<GridPos> {
    simulation
        .buildings
        .get(station.id)
        .and_then(|building| {
            let definition = building_def_for_archetype(content, &building.archetype)?;
            unoccupied_building_approach(
                world,
                building.position,
                rotated_footprint(definition.footprint, building.rotation_quarter_turns),
                current,
                actor,
                occupied,
                reservations,
            )
        })
        .or_else(|| nearest_walkable(world, station.position))
}

pub(crate) fn station_search_range_cells(station: StationCandidate<'_>) -> u16 {
    u16::try_from(station.definition.search_range_milli_cells.div_ceil(1_000)).unwrap_or(u16::MAX)
}

pub(crate) fn grid_distance_squared(left: GridPos, right: GridPos) -> u64 {
    let x = u64::from(left.x.abs_diff(right.x));
    let z = u64::from(left.z.abs_diff(right.z));
    x * x + z * z
}

pub(crate) fn target_score_milli(
    target: GridPos,
    actor: GridPos,
    assigned_count: u32,
    scoring: &TargetingScoreDef,
) -> u128 {
    // Keep Unity's Euclidean `Vector3.Distance` ordering without introducing
    // platform-dependent floating-point decisions into the simulation. The
    // square root is evaluated in thousandths of a logical grid cell.
    let distance_milli_cells = grid_distance_squared(target, actor)
        .saturating_mul(1_000_000)
        .isqrt();
    u128::from(distance_milli_cells)
        .saturating_mul(u128::from(scoring.distance_penalty_milli_per_cell))
        .div_ceil(1_000)
        .saturating_add(
            u128::from(assigned_count).saturating_mul(u128::from(scoring.assignment_penalty_milli)),
        )
}

pub(crate) fn within_station_search_region(
    position: GridPos,
    station: StationCandidate<'_>,
) -> bool {
    let range = station_search_range_cells(station);
    position.x.abs_diff(station.position.x) <= range
        && position.z.abs_diff(station.position.z) <= range
}

pub(crate) fn station_target_is_reachable(
    world: &GeneratedWorld,
    station: StationCandidate<'_>,
    target_kind: &StableId,
    target: GridPos,
) -> bool {
    // Unity intentionally skips its path check for fish, which are approached
    // from the shoreline rather than from the target's water node.
    if target_kind.as_str() == "target:fish" {
        return true;
    }
    let Some(start) = nearest_walkable(world, station.position) else {
        return false;
    };
    let Some(goal) = nearest_walkable(world, target) else {
        return false;
    };
    world.navigation.find_path(start, goal).is_ok()
}

pub(crate) fn stable_station_timer_offset(station: &StableId, update_milliseconds: u32) -> f64 {
    // Unity starts every station at a random point inside its authored refresh
    // interval. A stable FNV-1a offset preserves that stagger without allowing
    // platform RNG or frame order to alter target selection.
    let hash = station
        .as_str()
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3)
        });
    f64::from(u32::try_from(hash % u64::from(update_milliseconds)).unwrap_or_default())
}

pub(crate) fn active_station_ids(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    config: &GameConfig,
) -> Vec<StableId> {
    let town_hall = StableId::new("building:townhall").expect("static building ID");
    let mut stations = BTreeSet::from([town_hall]);
    stations.extend(
        simulation.buildings.keys().filter_map(|id| {
            station_candidate(content, simulation, config, id).map(|_| id.clone())
        }),
    );
    stations.into_iter().collect()
}

pub(crate) fn station_target_position(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    resource_index: &StationResourceTargetIndex,
    kind: &StableId,
    target: &StableId,
) -> Option<GridPos> {
    match kind.as_str() {
        "target:tree" | "target:ore" | "target:bush" | "target:fish" => resource_index
            .by_id
            .get(target)
            .and_then(|index| world.resources.get(*index))
            .filter(|resource| {
                resource.id == *target && resource.amount > 0 && resource.target_kind == *kind
            })
            .map(|resource| resource.position),
        "target:farm" => simulation
            .buildings
            .get(target)
            .filter(|building| is_farm_resource_building(content, building))
            .map(|building| building_visual_grid(content, building)),
        "target:enemy" | "target:boss" => simulation
            .actors
            .get(target)
            .filter(|actor| actor.alive && actor.role.as_str() == "role:enemy")
            .filter(|actor| {
                let is_boss = actor_archetype(content, actor)
                    .and_then(|archetype| archetype.enemy.as_ref())
                    .is_some_and(|enemy| enemy.enemy_type.as_str().ends_with("_boss"));
                is_boss == (kind.as_str() == "target:boss")
            })
            .map(|actor| actor.position),
        "target:player" => simulation
            .actors
            .get(target)
            .filter(|actor| {
                actor.alive
                    && actor.role.as_str() != "role:enemy"
                    && actor.health == actor.max_health
            })
            .map(|actor| actor.position),
        "target:injured_player" => simulation
            .actors
            .get(target)
            .filter(|actor| {
                actor.alive
                    && actor.role.as_str() != "role:enemy"
                    && actor.health < actor.max_health
            })
            .map(|actor| actor.position),
        "target:dead_player" => simulation
            .actors
            .get(target)
            .filter(|actor| !actor.alive && actor.role.as_str() != "role:enemy")
            .map(|actor| actor.position),
        "target:construction" => simulation
            .buildings
            .get(target)
            .filter(|building| !building.complete && building.health > 0)
            .map(|building| building_visual_grid(content, building)),
        "target:damaged_building" => simulation
            .buildings
            .get(target)
            .filter(|building| {
                building.complete
                    && building.health > 0
                    && building.health < building_max_health(content, building)
            })
            .map(|building| building_visual_grid(content, building)),
        "target:building" => simulation
            .buildings
            .get(target)
            .filter(|building| {
                building.complete
                    && building.health == building_max_health(content, building)
                    && !is_farm_resource_building(content, building)
            })
            .map(|building| building_visual_grid(content, building)),
        _ => None,
    }
}

pub(crate) fn populate_station_target_cache(
    runtime: &mut StationTargetRuntime,
    resource_index: &mut StationResourceTargetIndex,
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
    station_id: &StableId,
) {
    resource_index.sync(world);
    let Some(station) = station_candidate(content, simulation, config, station_id) else {
        runtime.stations.remove(station_id);
        return;
    };
    let target_kinds = if station.definition.targets_all {
        content
            .station_target_update_modes
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>()
    } else {
        station.definition.target_kinds.clone()
    };
    let cache = runtime.stations.entry(station_id.clone()).or_default();
    cache.targets.retain(|kind, _| target_kinds.contains(kind));
    for kind in target_kinds {
        let mut candidates = if matches!(
            kind.as_str(),
            "target:tree" | "target:ore" | "target:bush" | "target:fish"
        ) {
            resource_index
                .by_kind
                .get(&kind)
                .into_iter()
                .flatten()
                .filter_map(|index| world.resources.get(*index))
                .filter(|resource| resource.amount > 0 && resource.target_kind == kind)
                .map(|resource| (resource.position, resource.id.clone()))
                .collect()
        } else {
            station_target_catalog_candidates(content, simulation, world, &kind)
        };
        candidates.retain(|(position, _)| within_station_search_region(*position, station));
        candidates.sort_by_key(|(position, id)| {
            (
                grid_distance_squared(*position, station.position),
                id.clone(),
            )
        });
        let available = candidates
            .iter()
            .map(|(_, id)| id.clone())
            .collect::<BTreeSet<_>>();
        let previous = cache.targets.get(&kind).cloned().unwrap_or_default();
        let mut next = match content.station_target_update_modes.get(&kind) {
            Some(StationUpdateMode::Update) => previous
                .iter()
                .filter(|target| available.contains(*target))
                .cloned()
                .collect::<Vec<_>>(),
            Some(StationUpdateMode::Clear) | None => Vec::new(),
        };
        for (_, target) in candidates {
            if next.len() >= usize::from(station.definition.max_targets) {
                break;
            }
            if !next.contains(&target) {
                next.push(target);
            }
        }
        for target in next.iter().filter(|target| !previous.contains(*target)) {
            let queued = (kind.clone(), target.clone());
            if !cache.reachability_queue.contains(&queued) {
                cache.reachability_queue.push_back(queued);
            }
        }
        cache.targets.insert(kind, next);
    }
}

#[cfg(test)]
pub(crate) fn immediate_station_target_runtime(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    world: &GeneratedWorld,
    config: &GameConfig,
) -> StationTargetRuntime {
    let mut runtime = StationTargetRuntime::default();
    let mut resource_index = StationResourceTargetIndex::default();
    for station_id in active_station_ids(content, simulation, config) {
        if let Some(station) = station_candidate(content, simulation, config, &station_id) {
            runtime.stations.insert(
                station_id.clone(),
                CachedStationTargets {
                    refresh_elapsed_milliseconds: stable_station_timer_offset(
                        &station_id,
                        station.definition.update_milliseconds,
                    ),
                    ..default()
                },
            );
            populate_station_target_cache(
                &mut runtime,
                &mut resource_index,
                content,
                simulation,
                world,
                config,
                &station_id,
            );
        }
    }
    let station_ids = runtime.stations.keys().cloned().collect::<Vec<_>>();
    for station_id in station_ids {
        let Some(station) = station_candidate(content, simulation, config, &station_id) else {
            continue;
        };
        let cached = runtime
            .stations
            .get(&station_id)
            .map(|cache| cache.targets.clone())
            .unwrap_or_default();
        for (kind, targets) in cached {
            let reachable = targets
                .into_iter()
                .filter(|target| {
                    station_target_position(
                        content,
                        simulation,
                        world,
                        &resource_index,
                        &kind,
                        target,
                    )
                    .is_some_and(|position| {
                        station_target_is_reachable(world, station, &kind, position)
                    })
                })
                .collect();
            if let Some(cache) = runtime.stations.get_mut(&station_id) {
                cache.targets.insert(kind, reachable);
            }
        }
    }
    runtime
}

pub(crate) fn refresh_station_target_runtime(
    cadence: Res<AgentSimulationCadence>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    config: Res<RuntimeConfig>,
    mut runtime: ResMut<StationTargetRuntime>,
    mut resource_index: ResMut<StationResourceTargetIndex>,
) {
    resource_index.sync(&world.generated);
    let active = active_station_ids(&content.0, &simulation.0, &config.0);
    let active_set = active.iter().cloned().collect::<BTreeSet<_>>();
    runtime.stations.retain(|id, _| active_set.contains(id));
    runtime.refresh_queue.retain(|id| active_set.contains(id));

    for station_id in &active {
        if runtime.stations.contains_key(station_id) {
            continue;
        }
        let Some(station) = station_candidate(&content.0, &simulation.0, &config.0, station_id)
        else {
            continue;
        };
        runtime.stations.insert(
            station_id.clone(),
            CachedStationTargets {
                refresh_elapsed_milliseconds: stable_station_timer_offset(
                    station_id,
                    station.definition.update_milliseconds,
                ),
                ..default()
            },
        );
        populate_station_target_cache(
            &mut runtime,
            &mut resource_index,
            &content.0,
            &simulation.0,
            &world.generated,
            &config.0,
            station_id,
        );
    }

    let delta_milliseconds = cadence.delta().as_secs_f64() * 1_000.0;
    let mut due = Vec::new();
    for station_id in &active {
        let Some(station) = station_candidate(&content.0, &simulation.0, &config.0, station_id)
        else {
            continue;
        };
        let cache = runtime
            .stations
            .get_mut(station_id)
            .expect("active stations are initialized above");
        cache.refresh_elapsed_milliseconds += delta_milliseconds;
        cache.target_check_elapsed_milliseconds += delta_milliseconds;
        if cache.refresh_elapsed_milliseconds >= f64::from(station.definition.update_milliseconds) {
            cache.refresh_elapsed_milliseconds -= f64::from(station.definition.update_milliseconds);
            due.push(station_id.clone());
        }
    }
    for station_id in due {
        if !runtime.refresh_queue.contains(&station_id) {
            runtime.refresh_queue.push_back(station_id);
        }
    }

    // Unity's StationProcessor repopulates at most one queued station per
    // rendered frame, even when several authored timers expire together.
    if let Some(station_id) = runtime.refresh_queue.pop_front() {
        populate_station_target_cache(
            &mut runtime,
            &mut resource_index,
            &content.0,
            &simulation.0,
            &world.generated,
            &config.0,
            &station_id,
        );
    }

    for station_id in active {
        let check = runtime.stations.get_mut(&station_id).and_then(|cache| {
            if cache.target_check_elapsed_milliseconds < STATION_TARGET_CHECK_MILLISECONDS {
                return None;
            }
            cache.target_check_elapsed_milliseconds -= STATION_TARGET_CHECK_MILLISECONDS;
            cache.reachability_queue.pop_front()
        });
        let Some((kind, target)) = check else {
            continue;
        };
        let reachable = station_candidate(&content.0, &simulation.0, &config.0, &station_id)
            .and_then(|station| {
                station_target_position(
                    &content.0,
                    &simulation.0,
                    &world.generated,
                    &resource_index,
                    &kind,
                    &target,
                )
                .map(|position| {
                    station_target_is_reachable(&world.generated, station, &kind, position)
                })
            })
            .unwrap_or(false);
        let cache = runtime
            .stations
            .get_mut(&station_id)
            .expect("active station cache exists");
        if reachable {
            cache.reachability_queue.push_back((kind, target));
        } else if let Some(targets) = cache.targets.get_mut(&kind) {
            targets.retain(|candidate| candidate != &target);
        }
    }
}
