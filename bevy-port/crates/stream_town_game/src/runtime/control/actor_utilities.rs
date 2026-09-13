pub(crate) fn actor_idle_anchor(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    config: &GameConfig,
    regeneration: &RegenerationRoleRuntime,
    actor_id: &StableId,
    current: GridPos,
) -> GridPos {
    let Some(actor) = simulation.actors.get(actor_id) else {
        return current;
    };
    if matches!(actor.role.as_str(), "role:forester" | "role:tender")
        && let Some(station_id) = regeneration
            .workers
            .get(actor_id)
            .and_then(|worker| worker.last_station.as_ref())
        && let Some(station) = simulation.buildings.get(station_id)
        && station.complete
        && building_def_for_archetype(content, &station.archetype).is_some_and(|definition| {
            definition
                .role_slots
                .iter()
                .any(|slot| slot.role == actor.role)
        })
    {
        return station.position;
    }
    if let Some(station) = assigned_station(content, simulation, config, actor) {
        return station.position;
    }
    let Some(station_id) = best_station_id(content, simulation, config, &actor.role, current)
    else {
        return current;
    };
    station_candidate(content, simulation, config, &station_id)
        .map_or(current, |station| station.position)
}

pub(crate) fn deterministic_wander_target_step(
    world: &GeneratedWorld,
    actor: &StableId,
    anchor: GridPos,
    current: GridPos,
    sequence: u64,
    avoid: Option<GridPos>,
    mut target_is_usable: impl FnMut(GridPos) -> bool,
) -> GridPos {
    // Unity samples Random.insideUnitCircle around the actor's station whenever
    // its three-second idle timer completes. Stable sequence state supplies a
    // fresh deterministic sample without deriving the next destination only
    // from the current cell (which created A/B pendulum routes).
    let mut actor_hash = 0xcbf2_9ce4_8422_2325_u64 ^ world.seed;
    for byte in actor.as_str().bytes() {
        actor_hash = (actor_hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3);
    }
    for attempt in 0_u64..96 {
        let serial = sequence.wrapping_mul(96).wrapping_add(attempt);
        let mut mixed = actor_hash.wrapping_add(serial.wrapping_mul(0x9e37_79b9_7f4a_7c15));
        mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        mixed ^= mixed >> 31;
        let offset_x = i32::try_from(mixed % 11).expect("bounded idle offset") - 5;
        let offset_z = i32::try_from((mixed >> 16) % 11).expect("bounded idle offset") - 5;
        let distance_squared = offset_x * offset_x + offset_z * offset_z;
        if !(4..=25).contains(&distance_squared) {
            continue;
        }
        let candidate_x = i32::from(anchor.x) + offset_x;
        let candidate_z = i32::from(anchor.z) + offset_z;
        if candidate_x < 0
            || candidate_z < 0
            || candidate_x >= i32::from(world.navigation.width())
            || candidate_z >= i32::from(world.navigation.height())
        {
            continue;
        }
        let candidate = GridPos {
            x: u16::try_from(candidate_x).expect("checked idle x"),
            z: u16::try_from(candidate_z).expect("checked idle z"),
        };
        if candidate == current || Some(candidate) == avoid {
            continue;
        }
        if world.navigation.is_walkable(candidate) && target_is_usable(candidate) {
            return candidate;
        }
    }
    let fallback = deterministic_wander_target(world, actor, current);
    if fallback == current || Some(fallback) == avoid || !target_is_usable(fallback) {
        current
    } else {
        fallback
    }
}

pub(crate) fn deterministic_wander_target(
    world: &GeneratedWorld,
    actor: &StableId,
    position: GridPos,
) -> GridPos {
    // Unity's idle state samples a fresh point from a ten-world-unit circle.
    // Mirroring the whole map produced a deterministic two-point pendulum.
    // These integer offsets cover the same five-cell radius while the stable
    // location hash makes each reached cell lead to a new local destination.
    const OFFSETS: [(i16, i16); 32] = [
        (5, 0),
        (5, 2),
        (4, 3),
        (3, 4),
        (2, 5),
        (0, 5),
        (-2, 5),
        (-3, 4),
        (-4, 3),
        (-5, 2),
        (-5, 0),
        (-5, -2),
        (-4, -3),
        (-3, -4),
        (-2, -5),
        (0, -5),
        (2, -5),
        (3, -4),
        (4, -3),
        (5, -2),
        (3, 0),
        (3, 2),
        (2, 3),
        (0, 3),
        (-2, 3),
        (-3, 2),
        (-3, 0),
        (-3, -2),
        (-2, -3),
        (0, -3),
        (2, -3),
        (3, -2),
    ];
    let mut hash = 0xcbf2_9ce4_8422_2325_u64 ^ world.seed;
    for byte in actor.as_str().bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash ^= u64::from(position.x) << 16 | u64::from(position.z);
    hash = hash.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    let offset_count = u64::try_from(OFFSETS.len()).expect("wander offset count fits u64");
    let start = usize::try_from(hash % offset_count).expect("bounded wander index");
    for attempt in 0..OFFSETS.len() {
        // Seven is coprime with 32, so every direction is visited exactly once.
        let (offset_x, offset_z) = OFFSETS[(start + attempt * 7) % OFFSETS.len()];
        let candidate_x = i32::from(position.x) + i32::from(offset_x);
        let candidate_z = i32::from(position.z) + i32::from(offset_z);
        if candidate_x < 0
            || candidate_z < 0
            || candidate_x >= i32::from(world.navigation.width())
            || candidate_z >= i32::from(world.navigation.height())
        {
            continue;
        }
        let candidate = GridPos {
            x: u16::try_from(candidate_x).expect("checked wander x"),
            z: u16::try_from(candidate_z).expect("checked wander z"),
        };
        if world.navigation.is_walkable(candidate)
            && world.navigation.find_path(position, candidate).is_ok()
        {
            return candidate;
        }
    }
    position
}

pub(crate) fn prefixed_id(requested: &StableId, prefix: &str) -> Option<StableId> {
    if requested.as_str().starts_with(prefix) {
        Some(requested.clone())
    } else {
        StableId::new(format!("{prefix}{}", requested.as_str())).ok()
    }
}
