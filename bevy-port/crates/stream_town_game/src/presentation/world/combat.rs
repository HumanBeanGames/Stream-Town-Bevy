#[cfg(test)]
pub(crate) fn complete_agent_goal(
    simulation: &mut WorldSimulation,
    world: &mut GeneratedWorld,
    config: &GameConfig,
    content: &ContentCatalog,
    actor_id: &StableId,
    goal: &AgentGoal,
    current: GridPos,
) -> Option<ActionPresentation> {
    complete_agent_goal_with_regeneration(
        simulation,
        world,
        config,
        content,
        &mut RegenerationRoleRuntime::default(),
        actor_id,
        goal,
        current,
    )
}

pub(crate) fn resource_cell_has_active_generation_occupant(
    resources: &[stream_town_domain::GeneratedResource],
    position: GridPos,
) -> bool {
    resources.iter().any(|resource| {
        resource.amount > 0
            && resource.target_kind.as_str() != "target:fish"
            && resource.position == position
    })
}

pub(crate) fn upgraded_resource_remaining(
    saved_generator_version: u32,
    resource: &stream_town_domain::GeneratedResource,
    remaining: u32,
) -> u32 {
    if saved_generator_version >= 7 {
        return remaining;
    }
    let multiplier = match resource.target_kind.as_str() {
        "target:tree" => 5,
        "target:ore" | "target:bush" => 100,
        _ => 1,
    };
    remaining.saturating_mul(multiplier)
}

pub(crate) fn apply_combat_damage(
    config: &GameConfig,
    simulation: &mut WorldSimulation,
    content: &ContentCatalog,
    attacker_id: Option<&StableId>,
    target_id: &StableId,
    damage: u32,
) -> Result<bool, stream_town_domain::SimulationError> {
    if std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some() {
        return Ok(false);
    }
    let was_alive = simulation
        .actors
        .get(target_id)
        .is_some_and(|target| target.alive);
    let killed_by_player = attacker_id.is_some_and(|attacker| {
        simulation
            .actors
            .get(attacker)
            .is_some_and(|actor| actor.alive && actor.role.as_str() != "role:enemy")
    });
    let target_before_damage = simulation.actors.get(target_id).cloned();
    let enemy = target_before_damage
        .as_ref()
        .filter(|target| target.role.as_str() == "role:enemy")
        .and_then(|target| actor_archetype(content, target))
        .and_then(|archetype| archetype.enemy.as_ref())
        .cloned();
    let killed = simulation.damage_actor(target_id, damage)?;
    if !killed
        && let (Some(attacker_id), Some(target)) = (attacker_id, target_before_damage.as_ref())
        && enemy_attacks_attacker(content, target)
        && enemy_retaliation_target_is_valid(content, simulation, target, attacker_id)
        && let Some(target) = simulation.actors.get_mut(target_id)
    {
        target.preferred_target = Some(attacker_id.clone());
    }
    if killed
        && was_alive
        && killed_by_player
        && let Some(enemy) = enemy
    {
        let _ = simulation.record_objective_event(
            &content.objectives,
            &ObjectiveEvent::EnemyKilled(enemy.enemy_type),
        );
        let resource = enemy.kill_reward.resource;
        let amount = if simulation.active_community_event == Some(CommunityEvent::Invasion)
            && resource.as_str() == "resource:gold"
        {
            enemy.kill_reward.amount.saturating_mul(2)
        } else {
            enemy.kill_reward.amount
        };
        let capacity = resource_storage_capacity(config, content, simulation, &resource);
        let current = simulation
            .town_resources
            .get(&resource)
            .copied()
            .unwrap_or_default();
        simulation.town_resources.insert(
            resource.clone(),
            current.saturating_add(amount).min(capacity),
        );
        let _ = simulation.record_objective_event(
            &content.objectives,
            &ObjectiveEvent::ResourceGained { resource, amount },
        );
        if let Some(attacker_id) = attacker_id
            && let Some(attacker) = simulation.actors.get(attacker_id)
            && let Some(stats) = effective_role_stats(content, simulation, attacker)
        {
            let building_bonus = military_kill_experience_bonus_per_thousand(content, simulation);
            if building_bonus > 0 {
                let base_experience = u64::from(stats.action_amount)
                    .saturating_mul(u64::from(stats.experience_multiplier_per_thousand))
                    / 1_000;
                let extra = u32::try_from(
                    base_experience
                        .saturating_mul(u64::from(building_bonus))
                        .saturating_add(999)
                        / 1_000,
                )
                .unwrap_or(u32::MAX);
                let _ = simulation.grant_role_experience_capped(
                    attacker_id,
                    extra,
                    1_000,
                    actor_role_level_cap(content, attacker_id),
                    content.progression.role_experience_curve_level_span,
                    content.progression.role_experience_curve_maximum,
                );
            }
        }
    }
    Ok(killed)
}

pub(crate) fn resolve_combat_projectile_impact(
    config: &GameConfig,
    simulation: &mut WorldSimulation,
    content: &ContentCatalog,
    projectile: &CombatProjectile,
) -> Result<bool, stream_town_domain::SimulationError> {
    apply_combat_damage(
        config,
        simulation,
        content,
        match &projectile.source {
            ProjectileSource::Actor(actor) => Some(actor),
            // Unity tower projectiles call TakeDamage with a null Targetable,
            // so they neither provoke retaliation nor count as player kills.
            ProjectileSource::Building(_) => None,
        },
        &projectile.target,
        projectile.damage,
    )
}

pub(crate) fn enemy_retaliation_target_is_valid(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    enemy: &ActorState,
    attacker: &StableId,
) -> bool {
    if let Some(actor) = simulation.actors.get(attacker) {
        return actor.alive
            && actor.role.as_str() != "role:enemy"
            && (enemy_targets_kind(content, enemy, "target:player")
                || (actor.health < actor.max_health
                    && enemy_targets_kind(content, enemy, "target:injured_player")));
    }
    simulation.buildings.get(attacker).is_some_and(|building| {
        building.health > 0
            && (enemy_targets_kind(content, enemy, "target:building")
                || (!building.complete
                    && enemy_targets_kind(content, enemy, "target:construction"))
                || (building.complete
                    && building.health < building_max_health(content, building)
                    && enemy_targets_kind(content, enemy, "target:damaged_building")))
    })
}

pub(crate) fn spawn_combat_projectile(
    commands: &mut Commands,
    render: &RenderAssets,
    config: &GameConfig,
    origin: Vec3,
    projectile: ProjectileSpawn,
) {
    let scale = config.world.cell_size * 0.14;
    let material = match projectile.visual {
        CombatVisualKind::Necrotic => render.projectile_necrotic.clone(),
        CombatVisualKind::Arrow => render.projectile_arrow.clone(),
        CombatVisualKind::Physical | CombatVisualKind::Fireball => render.projectile.clone(),
    };
    let mut entity = commands.spawn((
        WorldEntity,
        CombatProjectile {
            source: projectile.source,
            target: projectile.target,
            damage: projectile.damage,
            speed_cells_per_second: projectile.speed_cells_per_second,
            visual: projectile.visual,
            trail_cooldown_seconds: 0.0,
            remaining_seconds: PROJECTILE_MAX_LIFETIME_SECONDS,
        },
        Transform::from_translation(origin + Vec3::Y * config.world.cell_size * 0.35)
            .with_scale(Vec3::splat(scale)),
    ));
    if projectile.visual == CombatVisualKind::Arrow
        && let Some(scene) = render.projectile_arrow_scene.clone()
    {
        entity.insert(WorldAssetRoot(scene));
    } else {
        entity.insert((Mesh3d(render.cube.clone()), MeshMaterial3d(material)));
    }
}

pub(crate) fn combat_material(
    render: &RenderAssets,
    visual: CombatVisualKind,
) -> Handle<StandardMaterial> {
    match visual {
        CombatVisualKind::Physical => render.impact_physical.clone(),
        CombatVisualKind::Arrow => render.projectile_arrow.clone(),
        CombatVisualKind::Fireball => render.projectile.clone(),
        CombatVisualKind::Necrotic => render.projectile_necrotic.clone(),
    }
}

pub(crate) fn spawn_combat_smoke_field(
    commands: &mut Commands,
    render: &RenderAssets,
    focus: Vec3,
    cell_size: f32,
) {
    let spacing = cell_size * 2.4;
    for (offset, visual) in [
        (-1.5_f32, CombatVisualKind::Physical),
        (-0.5, CombatVisualKind::Arrow),
        (0.5, CombatVisualKind::Fireball),
        (1.5, CombatVisualKind::Necrotic),
    ] {
        let origin = focus + Vec3::X * offset * spacing;
        spawn_combat_impact(commands, render, origin, visual, cell_size);
        for trail_index in 0..7_u16 {
            spawn_combat_trail(
                commands,
                render,
                origin + Vec3::Z * f32::from(trail_index) * cell_size * 0.32,
                Vec3::Z,
                visual,
                cell_size,
            );
        }
    }
}

pub(crate) fn spawn_combat_smoke_arrow(
    commands: &mut Commands,
    render: &RenderAssets,
    focus: Vec3,
    cell_size: f32,
) {
    let origin = focus + Vec3::new(-cell_size * 6.0, cell_size * 0.45, -cell_size * 2.8);
    let target = focus + Vec3::new(cell_size * 5.0, cell_size * 0.45, -cell_size * 2.8);
    let mut transform =
        Transform::from_translation(origin).with_scale(Vec3::splat(cell_size * 0.14));
    transform.look_to(target - origin, Vec3::Y);
    let mut entity = commands.spawn((
        WorldEntity,
        CombatProjectile {
            source: ProjectileSource::Building(
                StableId::new("building:townhall").expect("static stable ID"),
            ),
            target: StableId::new("actor:enemy_0000").expect("static stable ID"),
            damage: 0,
            speed_cells_per_second: 0.0,
            visual: CombatVisualKind::Arrow,
            trail_cooldown_seconds: f32::MAX,
            remaining_seconds: f32::MAX,
        },
        transform,
    ));
    if let Some(scene) = render.projectile_arrow_scene.clone() {
        entity.insert(WorldAssetRoot(scene));
    } else {
        entity.insert((
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.projectile_arrow.clone()),
        ));
    }
}

pub(crate) fn repeat_combat_smoke(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    mut cooldown_seconds: Local<f32>,
    mut arrow_spawned: Local<bool>,
) {
    if std::env::var_os("STREAM_TOWN_SMOKE_COMBAT_VFX").is_none() {
        return;
    }
    *cooldown_seconds -= time.delta_secs();
    if *cooldown_seconds > 0.0 {
        return;
    }
    let centre = GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    };
    let focus = grid_to_world_on_surface(centre, &config.0, &world.generated)
        + Vec3::Y * config.0.world.cell_size * 0.35;
    spawn_combat_smoke_field(&mut commands, &render, focus, config.0.world.cell_size);
    if !*arrow_spawned {
        spawn_combat_smoke_arrow(&mut commands, &render, focus, config.0.world.cell_size);
        *arrow_spawned = true;
    }
    *cooldown_seconds = 0.72;
}

pub(crate) fn spawn_combat_trail(
    commands: &mut Commands,
    render: &RenderAssets,
    position: Vec3,
    direction: Vec3,
    visual: CombatVisualKind,
    cell_size: f32,
) {
    let (duration_seconds, scale) = match visual {
        CombatVisualKind::Arrow => (
            TOWER_TRAIL_SECONDS,
            Vec3::new(TOWER_TRAIL_WIDTH * 0.35, TOWER_TRAIL_WIDTH * 0.35, 0.22),
        ),
        CombatVisualKind::Fireball => (
            0.6,
            Vec3::new(FIREBALL_TRAIL_SIZE * 0.27, FIREBALL_TRAIL_SIZE * 0.27, 0.16),
        ),
        CombatVisualKind::Necrotic => (0.75, Vec3::new(0.07, 0.07, 0.14)),
        CombatVisualKind::Physical => return,
    };
    let base_scale = scale * cell_size;
    let mut transform = Transform::from_translation(position).with_scale(base_scale);
    transform.look_to(direction.normalize_or_zero(), Vec3::Y);
    commands.spawn((
        WorldEntity,
        CombatTrailSegment {
            elapsed_seconds: 0.0,
            duration_seconds,
            base_scale,
        },
        Mesh3d(render.cube.clone()),
        MeshMaterial3d(combat_material(render, visual)),
        transform,
    ));
}

pub(crate) fn spawn_combat_impact(
    commands: &mut Commands,
    render: &RenderAssets,
    origin: Vec3,
    visual: CombatVisualKind,
    cell_size: f32,
) {
    let (particle_count, duration_seconds, speed) = match visual {
        CombatVisualKind::Physical | CombatVisualKind::Arrow => {
            (8_u16, CHARACTER_HIT_SECONDS, 1.25)
        }
        CombatVisualKind::Fireball => (14, 0.55, 2.0),
        CombatVisualKind::Necrotic => (12, 0.65, 1.5),
    };
    for index in 0..particle_count {
        let phase = f32::from(index) / f32::from(particle_count);
        let angle = phase * std::f32::consts::TAU;
        let vertical = 0.35 + f32::from(index % 3) * 0.24;
        let velocity = Vec3::new(angle.cos(), vertical, angle.sin()) * speed * cell_size;
        let base_scale = Vec3::splat(
            cell_size
                * if visual == CombatVisualKind::Fireball {
                    FIREBALL_SIZE * 0.275
                } else {
                    0.075
                },
        );
        commands.spawn((
            WorldEntity,
            CombatImpactParticle {
                elapsed_seconds: 0.0,
                duration_seconds,
                origin,
                velocity,
                base_scale,
            },
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(combat_material(render, visual)),
            Transform::from_translation(origin).with_scale(base_scale),
        ));
    }
}

pub(crate) fn animate_combat_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut trails: Query<(Entity, &mut CombatTrailSegment, &mut Transform)>,
    mut impacts: Query<
        (Entity, &mut CombatImpactParticle, &mut Transform),
        Without<CombatTrailSegment>,
    >,
) {
    for (entity, mut trail, mut transform) in &mut trails {
        trail.elapsed_seconds += time.delta_secs();
        let progress = trail.elapsed_seconds / trail.duration_seconds;
        if progress >= 1.0 {
            commands.entity(entity).try_despawn();
            continue;
        }
        transform.scale = trail.base_scale * (1.0 - progress);
    }
    for (entity, mut impact, mut transform) in &mut impacts {
        impact.elapsed_seconds += time.delta_secs();
        let progress = impact.elapsed_seconds / impact.duration_seconds;
        if progress >= 1.0 {
            commands.entity(entity).try_despawn();
            continue;
        }
        transform.translation = impact.origin
            + impact.velocity * impact.elapsed_seconds
            + Vec3::NEG_Y * 0.5 * 9.8 * impact.elapsed_seconds.powi(2);
        transform.rotation *= Quat::from_rotation_y(time.delta_secs() * 5.0);
        transform.scale = impact.base_scale * (1.0 - progress).sqrt();
    }
}

pub(crate) fn building_effect_material(
    render: &RenderAssets,
    kind: BuildingEffectKind,
) -> Handle<StandardMaterial> {
    match kind {
        BuildingEffectKind::WorkSmoke | BuildingEffectKind::DamageSmoke => {
            render.building_smoke.clone()
        }
        BuildingEffectKind::WorkSpark => render.building_spark.clone(),
        BuildingEffectKind::DamageFire => render.building_fire.clone(),
        BuildingEffectKind::LevelArrow => render.building_upgrade.clone(),
    }
}

pub(crate) fn spawn_building_particle(
    commands: &mut Commands,
    render: &RenderAssets,
    kind: BuildingEffectKind,
    origin: Vec3,
    velocity: Vec3,
    base_scale: Vec3,
    duration_seconds: f32,
    phase: f32,
) {
    commands.spawn((
        WorldEntity,
        BuildingEffectParticle {
            kind,
            elapsed_seconds: 0.0,
            duration_seconds,
            origin,
            velocity,
            base_scale,
            phase,
        },
        Mesh3d(render.cube.clone()),
        MeshMaterial3d(building_effect_material(render, kind)),
        bevy::light::NotShadowCaster,
        bevy::light::NotShadowReceiver,
        Transform::from_translation(origin).with_scale(Vec3::ZERO),
    ));
}

pub(crate) fn spawn_building_work_effect(
    commands: &mut Commands,
    render: &RenderAssets,
    origin: Vec3,
    sparks: bool,
    cell_size: f32,
) {
    for index in 0..5_u16 {
        let phase = f32::from(index) / 5.0;
        let angle = phase * std::f32::consts::TAU + 0.35;
        let particle_origin =
            origin + Vec3::new(angle.cos(), 0.16 + phase * 0.24, angle.sin()) * cell_size * 0.34;
        spawn_building_particle(
            commands,
            render,
            BuildingEffectKind::WorkSmoke,
            particle_origin,
            Vec3::new(
                angle.cos() * 0.22,
                BUILDING_HIT_SMOKE_SPEED,
                angle.sin() * 0.22,
            ) * cell_size,
            Vec3::splat(BUILDING_HIT_SMOKE_SIZE * cell_size * 0.34),
            BUILDING_HIT_SECONDS,
            phase,
        );
    }
    if !sparks {
        return;
    }
    for index in 0..8_u16 {
        let phase = f32::from(index) / 8.0;
        let angle = phase * std::f32::consts::TAU;
        spawn_building_particle(
            commands,
            render,
            BuildingEffectKind::WorkSpark,
            origin + Vec3::Y * cell_size * 0.35,
            Vec3::new(angle.cos(), 0.28 + f32::from(index % 3) * 0.12, angle.sin())
                * BUILDING_HIT_SPARK_SPEED
                * cell_size
                * 0.16,
            Vec3::new(0.045, BUILDING_HIT_SPARK_SIZE, 0.045) * cell_size,
            BUILDING_HIT_SECONDS,
            phase,
        );
    }
}

pub(crate) fn spawn_building_level_up_effect(
    commands: &mut Commands,
    render: &RenderAssets,
    origin: Vec3,
    cell_size: f32,
) {
    let radius = BUILDING_LEVEL_UP_TILE_SIZE * cell_size * 0.38;
    for index in 0..8_u16 {
        let phase = f32::from(index) / 8.0;
        let angle = phase * std::f32::consts::TAU;
        let arrow_origin = origin + Vec3::new(angle.cos() * radius, 0.1, angle.sin() * radius);
        spawn_building_particle(
            commands,
            render,
            BuildingEffectKind::LevelArrow,
            arrow_origin,
            Vec3::Y * cell_size * (1.15 + phase * 0.55),
            Vec3::new(0.12, BUILDING_LEVEL_UP_ARROW_SIZE, 0.12) * cell_size,
            BUILDING_LEVEL_UP_SECONDS,
            phase,
        );
    }
}

pub(crate) fn building_damage_intensity(health: i32, max_health: i32, complete: bool) -> f32 {
    if !complete || health <= 0 {
        return 0.0;
    }
    let health_ratio = building_damage_value(health, max_health);
    ((0.65 - health_ratio) / 0.65).clamp(0.0, 1.0)
}

pub(crate) fn emit_damaged_building_effects(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    render: Res<RenderAssets>,
    mut buildings: Query<(
        &RuntimeBuilding,
        &BuildingPresentation,
        &mut BuildingDamageEmitter,
    )>,
) {
    for (runtime, presentation, mut emitter) in &mut buildings {
        let Some(building) = simulation.0.buildings.get(&runtime.id) else {
            continue;
        };
        let intensity = building_damage_intensity(
            building.health,
            building_max_health(&content.0, building),
            building.complete,
        );
        if intensity <= f32::EPSILON {
            emitter.cooldown_seconds = 0.0;
            continue;
        }
        emitter.cooldown_seconds -= time.delta_secs();
        if emitter.cooldown_seconds > 0.0 {
            continue;
        }
        let cell_size = config.0.world.cell_size;
        let phase =
            f32::from(u16::try_from(emitter.sequence % 16).expect("sequence is bounded")) / 16.0;
        let angle = phase * std::f32::consts::TAU * 2.618_034;
        let radial = BUILDING_DAMAGED_RADIUS * cell_size * (0.18 + 0.38 * intensity);
        let origin = presentation.base_translation
            + Vec3::new(angle.cos() * radial, cell_size * 0.44, angle.sin() * radial);
        spawn_building_particle(
            &mut commands,
            &render,
            BuildingEffectKind::DamageSmoke,
            origin,
            Vec3::new(
                angle.cos() * 0.16,
                0.78 + intensity * 0.52,
                angle.sin() * 0.16,
            ) * cell_size,
            Vec3::splat(cell_size * (0.22 + intensity * 0.18)),
            1.55,
            phase,
        );
        if emitter.sequence.is_multiple_of(2) {
            spawn_building_particle(
                &mut commands,
                &render,
                BuildingEffectKind::DamageFire,
                origin,
                Vec3::Y * cell_size * (0.45 + intensity * 0.35),
                Vec3::new(0.16, 0.4, 0.16) * cell_size * (0.65 + intensity * 0.45),
                0.72,
                phase,
            );
        }
        emitter.sequence = emitter.sequence.wrapping_add(1);
        let authored_density =
            f32::from(BUILDING_DAMAGED_FIRE_AMOUNT) / f32::from(BUILDING_DAMAGED_SMOKE_AMOUNT);
        emitter.cooldown_seconds = (0.22 - intensity * 0.12) / authored_density.max(0.1);
    }
}

pub(crate) fn animate_building_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut effects: Query<(Entity, &mut BuildingEffectParticle, &mut Transform)>,
) {
    for (entity, mut effect, mut transform) in &mut effects {
        effect.elapsed_seconds += time.delta_secs();
        let progress = effect.elapsed_seconds / effect.duration_seconds;
        if progress >= 1.0 {
            commands.entity(entity).try_despawn();
            continue;
        }
        let envelope = (std::f32::consts::PI * progress).sin().max(0.0);
        match effect.kind {
            BuildingEffectKind::WorkSmoke => {
                transform.translation = effect.origin + effect.velocity * effect.elapsed_seconds;
                transform.scale = effect.base_scale * envelope.sqrt() * (0.72 + progress * 0.8);
                transform.rotation *= Quat::from_rotation_y(time.delta_secs() * 1.6);
            }
            BuildingEffectKind::WorkSpark => {
                transform.translation = effect.origin
                    + effect.velocity * effect.elapsed_seconds
                    + Vec3::NEG_Y * 0.5 * 9.8 * effect.elapsed_seconds.powi(2);
                transform.scale = effect.base_scale * (1.0 - progress);
                transform.rotation *= Quat::from_rotation_z(time.delta_secs() * 9.0);
            }
            BuildingEffectKind::LevelArrow => {
                transform.translation = effect.origin + effect.velocity * effect.elapsed_seconds;
                transform.scale = effect.base_scale * envelope.sqrt();
                transform.rotation =
                    Quat::from_rotation_y(effect.phase * std::f32::consts::TAU + progress * 0.35);
            }
            BuildingEffectKind::DamageSmoke => {
                transform.translation = effect.origin + effect.velocity * effect.elapsed_seconds;
                transform.scale = effect.base_scale * envelope.sqrt() * (0.6 + progress * 1.45);
                transform.rotation *= Quat::from_rotation_y(time.delta_secs() * 0.8);
            }
            BuildingEffectKind::DamageFire => {
                transform.translation = effect.origin + effect.velocity * effect.elapsed_seconds;
                let flicker = 0.78 + (progress * 31.0 + effect.phase * 13.0).sin().abs() * 0.32;
                transform.scale = effect.base_scale * envelope * flicker;
                transform.rotation *= Quat::from_rotation_y(time.delta_secs() * 4.0);
            }
        }
    }
}

pub(crate) fn spawn_building_smoke_field(
    commands: &mut Commands,
    render: &RenderAssets,
    focus: Vec3,
    cell_size: f32,
) {
    let spacing = cell_size * 3.4;
    spawn_building_work_effect(
        commands,
        render,
        focus - Vec3::X * spacing,
        false,
        cell_size,
    );
    spawn_building_work_effect(commands, render, focus, true, cell_size);
    spawn_building_level_up_effect(commands, render, focus + Vec3::X * spacing, cell_size);
    for sequence in 0..10_u16 {
        let phase = f32::from(sequence) / 10.0;
        let angle = phase * std::f32::consts::TAU;
        let origin = focus
            + Vec3::Z * spacing
            + Vec3::new(angle.cos(), 0.4, angle.sin()) * BUILDING_DAMAGED_RADIUS * cell_size;
        spawn_building_particle(
            commands,
            render,
            if sequence.is_multiple_of(3) {
                BuildingEffectKind::DamageFire
            } else {
                BuildingEffectKind::DamageSmoke
            },
            origin,
            Vec3::Y * cell_size * if sequence.is_multiple_of(3) { 0.7 } else { 1.0 },
            Vec3::splat(cell_size * 0.32),
            if sequence.is_multiple_of(3) {
                0.72
            } else {
                1.55
            },
            phase,
        );
    }
}

pub(crate) fn repeat_building_smoke(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    mut cooldown_seconds: Local<f32>,
) {
    if std::env::var_os("STREAM_TOWN_SMOKE_BUILDING_VFX").is_none() {
        return;
    }
    *cooldown_seconds -= time.delta_secs();
    if *cooldown_seconds > 0.0 {
        return;
    }
    let centre = GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    };
    let focus = grid_to_world_on_surface(centre, &config.0, &world.generated);
    spawn_building_smoke_field(&mut commands, &render, focus, config.0.world.cell_size);
    *cooldown_seconds = 0.82;
}
