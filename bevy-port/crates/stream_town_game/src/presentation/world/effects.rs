pub(crate) fn healing_channel_effect(
    presentation: &PresentationCatalog,
) -> (&StableId, &HealingChannelVfxDef) {
    presentation
        .healing_channel_effects
        .iter()
        .next()
        .expect("validated presentation contains the healing-channel graph")
}

pub(crate) fn healing_burst_effect(
    presentation: &PresentationCatalog,
) -> (&StableId, &HealingBurstVfxDef) {
    presentation
        .healing_burst_effects
        .iter()
        .next()
        .expect("validated presentation contains the completed-heal graph")
}

pub(crate) fn healing_material(color: [f32; 4], alpha_scale: f32) -> StandardMaterial {
    // Unity's VFX gradients are authored in HDR linear colour. Feeding values
    // such as green=16.9 directly into Bevy bloom makes a character-sized cue
    // look screen-sized. Preserve the hue and authored HDR intent while
    // bounding the emission to the same range as the rest of the port's VFX.
    let peak = color[0].max(color[1]).max(color[2]).max(1.0);
    let normalized = [color[0] / peak, color[1] / peak, color[2] / peak];
    let emissive_gain = peak.min(HEALING_EMISSIVE_MAX_COMPONENT);
    StandardMaterial {
        base_color: Color::linear_rgba(
            normalized[0],
            normalized[1],
            normalized[2],
            color[3] * alpha_scale,
        ),
        emissive: LinearRgba::new(
            normalized[0] * emissive_gain,
            normalized[1] * emissive_gain,
            normalized[2] * emissive_gain,
            1.0,
        ),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    }
}

pub(crate) fn healing_gradient_materials(
    materials: &mut Assets<StandardMaterial>,
    gradient: &VfxGradientDef,
) -> Vec<Handle<StandardMaterial>> {
    (0..CHIMNEY_ALPHA_STEPS)
        .filter_map(|step| {
            gradient
                .sample(chimney_alpha_progress(step))
                .map(|color| materials.add(healing_material(color, 1.0)))
        })
        .collect()
}

pub(crate) fn gradient_material(
    materials: &[Handle<StandardMaterial>],
    normalized_age: f32,
) -> Handle<StandardMaterial> {
    let last = materials.len().saturating_sub(1);
    let index = (0..=last)
        .rev()
        .find(|index| {
            let threshold = if last == 0 {
                0.0
            } else {
                f32::from(u16::try_from(*index).unwrap_or(u16::MAX))
                    / f32::from(u16::try_from(last).unwrap_or(u16::MAX))
            };
            normalized_age.clamp(0.0, 1.0) >= threshold
        })
        .unwrap_or_default();
    materials[index].clone()
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub(crate) fn f32_to_u16_saturating(value: f32) -> u16 {
    value.round().clamp(0.0, f32::from(u16::MAX)) as u16
}

pub(crate) fn healing_effect_duration(
    presentation: &PresentationCatalog,
    kind: HealingEffectKind,
) -> f32 {
    match kind {
        HealingEffectKind::Channel => healing_channel_effect(presentation)
            .1
            .duration_seconds()
            .expect("validated channel curve has a final key"),
        HealingEffectKind::Burst | HealingEffectKind::Revive => {
            healing_burst_effect(presentation).1.duration_seconds
        }
    }
}

pub(crate) fn healing_effect_sample(
    presentation: &PresentationCatalog,
    kind: HealingEffectKind,
    elapsed_seconds: f32,
    duration_seconds: f32,
) -> HealingEffectSample {
    let duration = duration_seconds.max(f32::EPSILON);
    let progress = (elapsed_seconds / duration).clamp(0.0, 1.0);
    let envelope = (std::f32::consts::PI * progress).sin().max(0.0);
    let channel_size = healing_channel_effect(presentation)
        .1
        .size_multiplier(elapsed_seconds)
        .unwrap_or_default();
    let disc_size = healing_burst_effect(presentation)
        .1
        .disc_size_multiplier_at(progress)
        .unwrap_or_default();
    match kind {
        HealingEffectKind::Channel => HealingEffectSample {
            ring_scale: channel_size,
            mote_scale: envelope.sqrt(),
            radial_distance: 0.1 + progress * 0.2,
            rise: 0.08 + progress * 0.48,
            rotation_radians: progress * std::f32::consts::TAU * 1.5,
        },
        HealingEffectKind::Burst => HealingEffectSample {
            ring_scale: disc_size,
            mote_scale: envelope,
            radial_distance: 0.12 + progress * 0.32,
            rise: 0.1 + progress * 0.6,
            rotation_radians: progress * std::f32::consts::TAU,
        },
        HealingEffectKind::Revive => HealingEffectSample {
            ring_scale: disc_size * 1.15,
            mote_scale: envelope,
            radial_distance: 0.14 + progress * 0.42,
            rise: 0.12 + progress * 0.78,
            rotation_radians: progress * std::f32::consts::TAU * 1.25,
        },
    }
}

pub(crate) fn spawn_healing_effect(
    commands: &mut Commands,
    presentation: &PresentationCatalog,
    render: &RenderAssets,
    origin: Vec3,
    kind: HealingEffectKind,
    cell_size: f32,
    follow_target: Option<StableId>,
) {
    // Unity-authored VFX sizes are already world-space values. The initial
    // port multiplied them by Bevy's two-metre grid cell a second time, which
    // made the green heal/revive burst look like a monster-sized screen flash.
    let world_scale = (cell_size / UNITY_AUTHORED_GRID_CELL_SIZE).clamp(0.25, 4.0);
    // The on-target heal flash is deliberately much smaller than the caster
    // channel and the ten-minute revive burst. It remains attached to the
    // healed actor without washing over nearby characters or the camera.
    let target_scale = if kind == HealingEffectKind::Burst {
        HEALING_TARGET_EFFECT_SCALE
    } else {
        1.0
    };
    let duration_seconds = healing_effect_duration(presentation, kind);
    let (effect, material, base_scale) = match kind {
        HealingEffectKind::Channel => {
            let (effect, definition) = healing_channel_effect(presentation);
            (
                effect.clone(),
                render
                    .healing_channel
                    .get(effect)
                    .cloned()
                    .unwrap_or_else(|| render.healing_green.clone()),
                world_scale * definition.exposed_size * HEALING_CHANNEL_RING_SCALE,
            )
        }
        HealingEffectKind::Burst | HealingEffectKind::Revive => {
            let (effect, _) = healing_burst_effect(presentation);
            (
                effect.clone(),
                render
                    .healing_disc_materials
                    .get(effect)
                    .and_then(|materials| materials.first())
                    .cloned()
                    .unwrap_or_else(|| render.healing_green.clone()),
                world_scale * HEALING_BURST_RING_SCALE * target_scale,
            )
        }
    };
    commands.spawn((
        WorldEntity,
        HealingRingEffect {
            kind,
            effect: effect.clone(),
            origin,
            elapsed_seconds: 0.0,
            duration_seconds,
            base_scale,
            follow_target: follow_target.clone(),
        },
        Mesh3d(render.healing_ring.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(origin + Vec3::Y * 0.08).with_scale(Vec3::ZERO),
    ));

    let (mote_count, mote_size_multiplier): (u16, f32) = match kind {
        HealingEffectKind::Channel => {
            let definition = healing_channel_effect(presentation).1;
            let average_lifetime = (definition.particle_lifetime_seconds[0]
                + definition.particle_lifetime_seconds[1])
                * 0.5;
            (
                f32_to_u16_saturating(definition.emission_rate_per_second * average_lifetime)
                    .min(definition.particle_capacity),
                1.2,
            )
        }
        HealingEffectKind::Burst | HealingEffectKind::Revive => {
            let definition = healing_burst_effect(presentation).1;
            (
                definition.plus_burst_count.min(definition.plus_capacity),
                if kind == HealingEffectKind::Revive {
                    1.25
                } else {
                    1.0
                },
            )
        }
    };
    for mote_index in 0..mote_count {
        let phase = f32::from(mote_index) / f32::from(mote_count);
        let angle_radians = phase * std::f32::consts::TAU;
        let (mesh, material) = if kind == HealingEffectKind::Channel {
            (render.chimney_particle.clone(), material.clone())
        } else {
            (
                render
                    .healing_plus
                    .clone()
                    .unwrap_or_else(|| render.cube.clone()),
                render
                    .healing_plus_materials
                    .get(&effect)
                    .and_then(|materials| materials.first())
                    .cloned()
                    .unwrap_or_else(|| render.healing_green.clone()),
            )
        };
        commands.spawn((
            WorldEntity,
            HealingMoteEffect {
                kind,
                effect: effect.clone(),
                origin,
                elapsed_seconds: -phase * 0.22,
                duration_seconds,
                angle_radians,
                phase,
                base_scale: Vec3::splat(world_scale * HEALING_MOTE_SCALE * target_scale),
                distance_scale: world_scale * target_scale,
                size_multiplier: mote_size_multiplier,
                follow_target: follow_target.clone(),
            },
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(origin).with_scale(Vec3::ZERO),
            bevy::light::NotShadowCaster,
            bevy::light::NotShadowReceiver,
        ));
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn animate_healing_effects(
    mut commands: Commands,
    time: Res<Time>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    actors: Query<(&Agent, &Transform), (Without<HealingRingEffect>, Without<HealingMoteEffect>)>,
    mut rings: Query<(
        Entity,
        &mut HealingRingEffect,
        &mut Transform,
        &mut MeshMaterial3d<StandardMaterial>,
    )>,
    mut motes: Query<
        (
            Entity,
            &mut HealingMoteEffect,
            &mut Transform,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<HealingRingEffect>,
    >,
) {
    let actor_positions: BTreeMap<_, _> = actors
        .iter()
        .map(|(agent, transform)| (agent.id.clone(), transform.translation))
        .collect();
    for (entity, mut effect, mut transform, mut material) in &mut rings {
        effect.elapsed_seconds += time.delta_secs();
        if effect.elapsed_seconds >= effect.duration_seconds {
            commands.entity(entity).try_despawn();
            continue;
        }
        if let Some(target) = effect.follow_target.as_ref()
            && let Some(position) = actor_positions.get(target)
        {
            effect.origin = *position;
        }
        let sample = healing_effect_sample(
            &presentation.0,
            effect.kind,
            effect.elapsed_seconds,
            effect.duration_seconds,
        );
        transform.translation = effect.origin + Vec3::Y * 0.08;
        transform.rotation = Quat::from_rotation_y(sample.rotation_radians * 0.2);
        transform.scale = Vec3::splat(effect.base_scale * sample.ring_scale);
        if effect.kind != HealingEffectKind::Channel
            && let Some(materials) = render.healing_disc_materials.get(&effect.effect)
        {
            material.0 =
                gradient_material(materials, effect.elapsed_seconds / effect.duration_seconds);
        }
    }
    for (entity, mut effect, mut transform, mut material) in &mut motes {
        effect.elapsed_seconds += time.delta_secs();
        if effect.elapsed_seconds >= effect.duration_seconds {
            commands.entity(entity).try_despawn();
            continue;
        }
        if let Some(target) = effect.follow_target.as_ref()
            && let Some(position) = actor_positions.get(target)
        {
            effect.origin = *position;
        }
        let sample = healing_effect_sample(
            &presentation.0,
            effect.kind,
            effect.elapsed_seconds.max(0.0),
            effect.duration_seconds,
        );
        let angle =
            effect.angle_radians + sample.rotation_radians + effect.phase * std::f32::consts::PI;
        let radius = sample.radial_distance * effect.distance_scale;
        transform.translation = effect.origin
            + Vec3::new(
                angle.cos() * radius,
                sample.rise * effect.distance_scale,
                angle.sin() * radius,
            );
        transform.rotation = Quat::from_rotation_y(-angle * 0.35);
        let particle_age =
            (effect.elapsed_seconds.max(0.0) / effect.duration_seconds).clamp(0.0, 1.0);
        let authored_scale = if effect.kind == HealingEffectKind::Channel {
            sample.mote_scale
        } else {
            healing_burst_effect(&presentation.0)
                .1
                .plus_size_multiplier(particle_age)
                .unwrap_or_default()
        };
        transform.scale = effect.base_scale * authored_scale * effect.size_multiplier;
        if effect.kind != HealingEffectKind::Channel
            && let Some(materials) = render.healing_plus_materials.get(&effect.effect)
        {
            material.0 = gradient_material(materials, particle_age);
        }
    }
}

pub(crate) fn move_combat_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    render: Res<RenderAssets>,
    mut simulation: ResMut<SimulationRuntime>,
    actors: Query<(&Agent, &Transform), Without<CombatProjectile>>,
    mut projectiles: Query<(Entity, &mut CombatProjectile, &mut Transform), Without<Agent>>,
) {
    let positions: BTreeMap<_, _> = actors
        .iter()
        .map(|(agent, transform)| (agent.id.clone(), transform.translation))
        .collect();
    for (entity, mut projectile, mut transform) in &mut projectiles {
        projectile.remaining_seconds -= time.delta_secs();
        if projectile.remaining_seconds <= 0.0 {
            commands.entity(entity).try_despawn();
            continue;
        }
        let source_valid = match &projectile.source {
            ProjectileSource::Actor(actor) => simulation
                .0
                .actors
                .get(actor)
                .is_some_and(|actor| actor.alive),
            ProjectileSource::Building(building) => simulation
                .0
                .buildings
                .get(building)
                .is_some_and(|building| building.complete),
        };
        let Some(target) = simulation.0.actors.get(&projectile.target) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let Some(target_position) = positions.get(&projectile.target).copied() else {
            commands.entity(entity).try_despawn();
            continue;
        };
        if !source_valid || !target.alive {
            commands.entity(entity).try_despawn();
            continue;
        }
        let target_position = target_position + Vec3::Y * config.0.world.cell_size * 0.35;
        let delta = target_position - transform.translation;
        let step = projectile.speed_cells_per_second * config.0.world.cell_size * time.delta_secs();
        if delta.length_squared() <= step.max(0.1).powi(2) {
            if let Err(error) = resolve_combat_projectile_impact(
                &config.0,
                &mut simulation.0,
                &content.0,
                &projectile,
            ) {
                warn!(target = %projectile.target, %error, "projectile impact failed");
            }
            spawn_combat_impact(
                &mut commands,
                &render,
                target_position,
                projectile.visual,
                config.0.world.cell_size,
            );
            commands.entity(entity).try_despawn();
        } else {
            transform.translation += delta.normalize_or_zero() * step;
            transform.look_to(delta.normalize_or_zero(), Vec3::Y);
            projectile.trail_cooldown_seconds -= time.delta_secs();
            if projectile.trail_cooldown_seconds <= 0.0 {
                spawn_combat_trail(
                    &mut commands,
                    &render,
                    transform.translation,
                    delta,
                    projectile.visual,
                    config.0.world.cell_size,
                );
                projectile.trail_cooldown_seconds = 0.045;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn update_enemy_encounters(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    enemy_navigation: Res<EnemyNavigationRuntime>,
    mut simulation: ResMut<SimulationRuntime>,
    mut night_waves: ResMut<NightEnemyWaveRuntime>,
    agents: Query<(Entity, &Agent)>,
) {
    let dead_enemies: Vec<_> = agents
        .iter()
        .filter(|(_, agent)| agent.kind == ActorKind::Enemy)
        .filter(|(_, agent)| {
            !simulation
                .0
                .actors
                .get(&agent.id)
                .is_some_and(|actor| actor.alive)
        })
        .map(|(entity, agent)| (entity, agent.id.clone()))
        .collect();
    for (entity, enemy) in dead_enemies {
        commands.entity(entity).try_despawn();
        simulation.0.actors.remove(&enemy);
        for camp in simulation.0.enemy_camps.values_mut() {
            camp.spawned_enemies.remove(&enemy);
        }
        if let Some(raid) = &mut simulation.0.active_raid {
            raid.tracked_enemies.remove(&enemy);
        }
    }

    if !night_waves.pending_spawns.is_empty() {
        night_waves.spawn_remaining_seconds =
            (night_waves.spawn_remaining_seconds - time.delta_secs_f64()).max(0.0);
        if night_waves.spawn_remaining_seconds <= f64::EPSILON
            && let Some(pending) = night_waves.pending_spawns.pop_front()
        {
            let spawn_source = simulation
                .0
                .enemy_camps
                .get(&pending.camp)
                .and_then(|camp| {
                    let camp_archetype = content.0.archetypes.get(&camp.archetype)?;
                    Some((
                        camp.clone(),
                        camp_archetype.enemy_spawner.clone()?,
                        camp_archetype.footprint,
                    ))
                });
            if let Some((camp, spawner, footprint)) = spawn_source {
                let position = enemy_spawn_position(
                    &world.generated,
                    &content.0,
                    &simulation.0,
                    enemy_navigation.field.as_ref(),
                    &camp,
                    &spawner,
                    footprint,
                    pending.serial,
                );
                if let Some(enemy) = spawn_runtime_enemy(
                    &mut commands,
                    &config.0,
                    &world.generated,
                    &content.0,
                    &presentation.0,
                    asset_server.as_deref(),
                    &asset_root.0,
                    &render,
                    &mut simulation.0,
                    pending.archetype,
                    position,
                ) {
                    if pending.final_raid_wave {
                        let player_count = simulation_player_count(&simulation.0);
                        let boss_health =
                            i32::try_from(50_usize.saturating_mul(player_count).max(1_000))
                                .unwrap_or(i32::MAX);
                        if let Some(actor) = simulation.0.actors.get_mut(&enemy) {
                            actor.health = boss_health;
                            actor.max_health = boss_health;
                        }
                    }
                    if let Some(camp) = simulation.0.enemy_camps.get_mut(&pending.camp) {
                        camp.spawned_enemies.insert(enemy.clone());
                    }
                    if let Some(raid) = &mut simulation.0.active_raid {
                        raid.tracked_enemies.insert(enemy);
                    }
                }
            }
            night_waves.spawn_remaining_seconds = night_waves.spawn_interval_seconds;
        }
        return;
    }

    if simulation.0.active_event == Some(TownEvent::EnemyRaid)
        && simulation.0.active_raid.is_none()
        && let (Some(enemy), Some(boss)) = (
            archetype_id_by_source(&content.0, ArchetypeKind::Enemy, "Enemy_Minotaur.prefab"),
            archetype_id_by_source(
                &content.0,
                ArchetypeKind::Enemy,
                "Enemy_MinotaurBoss.prefab",
            ),
        )
    {
        let player_count = simulation_player_count(&simulation.0);
        let wave_size = event_adjusted_wave_size(
            raid_enemies_per_wave(player_count),
            simulation.0.active_community_event,
        );
        let _ = simulation.0.start_raid(5, wave_size, enemy, boss);
    }

    let next_wave = simulation.0.active_raid.as_ref().and_then(|raid| {
        raid.tracked_enemies.is_empty().then(|| {
            if raid.current_wave >= raid.total_waves {
                None
            } else {
                let final_wave = raid.current_wave + 1 == raid.total_waves;
                Some((
                    if final_wave {
                        raid.boss_archetype.clone()
                    } else {
                        raid.enemy_archetype.clone()
                    },
                    if final_wave { 1 } else { raid.enemies_per_wave },
                    final_wave,
                ))
            }
        })
    });
    if matches!(next_wave, Some(None)) {
        simulation.0.finish_raid();
    } else if let Some(Some((archetype, count, final_wave))) = next_wave {
        let camps = simulation
            .0
            .enemy_camps
            .iter()
            .filter(|(_, camp)| {
                content
                    .0
                    .archetypes
                    .get(&camp.archetype)
                    .is_some_and(|archetype| archetype.enemy_spawner.is_some())
            })
            .map(|(id, _)| id.clone())
            .collect::<Vec<_>>();
        if camps.is_empty() {
            simulation.0.finish_raid();
        } else {
            for offset in 0..u64::from(count) {
                let serial = simulation.0.next_enemy_serial.saturating_add(offset);
                // Unity chooses a random eligible camp for every raid member.
                // Use the same per-enemy distribution with a stable hash so
                // fixed seeds and replays remain deterministic in Bevy.
                let camp_index = usize::try_from(
                    generated_enemy_camp_hash(simulation.0.world_seed, &archetype, serial)
                        % u64::try_from(camps.len()).expect("camp count fits u64"),
                )
                .expect("raid camp index fits usize");
                night_waves.pending_spawns.push_back(PendingEnemySpawn {
                    camp: camps[camp_index].clone(),
                    archetype: archetype.clone(),
                    serial,
                    final_raid_wave: final_wave,
                });
            }
            night_waves.spawn_interval_seconds =
                ENEMY_WAVE_SPAWN_SPREAD_SECONDS / f64::from(count.max(1));
            night_waves.spawn_remaining_seconds = 0.0;
            if let Some(raid) = &mut simulation.0.active_raid {
                raid.current_wave = raid.current_wave.saturating_add(1);
                raid.tracked_enemies.clear();
            }
        }
        return;
    }

    if simulation.0.active_raid.is_some() {
        return;
    }
    if config
        .0
        .time
        .sample(simulation.0.elapsed_seconds)
        .is_daytime
    {
        *night_waves = NightEnemyWaveRuntime::default();
        return;
    }
    if !night_enemy_wave_due(&mut night_waves, simulation.0.day, time.delta_secs_f64()) {
        return;
    }
    let player_count = simulation_player_count(&simulation.0);
    let camps = simulation
        .0
        .enemy_camps
        .iter()
        .filter_map(|(camp_id, camp)| {
            let archetype = content.0.archetypes.get(&camp.archetype)?;
            Some((
                camp_id.clone(),
                camp.clone(),
                archetype.enemy_spawner.clone()?,
                archetype.footprint,
            ))
        })
        .collect::<Vec<_>>();
    if camps.is_empty() {
        return;
    }
    let wave_archetype = StableId::new("enemy:night_wave").expect("static stable ID");
    let wave_count = event_adjusted_wave_size(
        raid_enemies_per_wave(player_count),
        simulation.0.active_community_event,
    );
    for offset in 0..u64::from(wave_count) {
        let serial = simulation.0.next_enemy_serial.saturating_add(offset);
        let camp_index = usize::try_from(
            generated_enemy_camp_hash(simulation.0.world_seed, &wave_archetype, serial)
                % u64::try_from(camps.len()).expect("camp count fits u64"),
        )
        .expect("night-wave camp index fits usize");
        let (camp_id, _, spawner, _) = &camps[camp_index];
        let archetype = weighted_enemy_archetype(spawner, simulation.0.world_seed, serial);
        night_waves.pending_spawns.push_back(PendingEnemySpawn {
            camp: camp_id.clone(),
            archetype,
            serial,
            final_raid_wave: false,
        });
    }
    night_waves.spawn_interval_seconds =
        ENEMY_WAVE_SPAWN_SPREAD_SECONDS / f64::from(wave_count.max(1));
    night_waves.spawn_remaining_seconds = 0.0;
}
