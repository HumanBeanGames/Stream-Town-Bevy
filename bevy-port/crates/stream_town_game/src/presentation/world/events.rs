pub(crate) fn night_enemy_wave_due(
    runtime: &mut NightEnemyWaveRuntime,
    day: u32,
    delta_seconds: f64,
) -> bool {
    if runtime.night_day != Some(day) {
        runtime.night_day = Some(day);
        runtime.remaining_seconds = 0.0;
        runtime.wave_index = 0;
    }
    runtime.remaining_seconds = (runtime.remaining_seconds - delta_seconds.max(0.0)).max(0.0);
    if runtime.remaining_seconds > f64::EPSILON {
        return false;
    }
    runtime.remaining_seconds = NIGHT_ENEMY_WAVE_INTERVAL_SECONDS;
    runtime.wave_index = runtime.wave_index.saturating_add(1);
    true
}

pub(crate) fn simulation_player_count(simulation: &WorldSimulation) -> usize {
    simulation
        .actors
        .values()
        .filter(|actor| {
            actor.role.as_str() != "role:enemy" && !is_guardhouse_defender_id(&actor.id)
        })
        .count()
}

pub(crate) fn raid_enemies_per_wave(player_count: usize) -> u16 {
    u16::try_from(player_count.clamp(2, 25)).unwrap_or(25)
}

pub(crate) fn restart_world_after_town_hall_falls(
    mut commands: Commands,
    simulation: Res<SimulationRuntime>,
    mut restart: ResMut<TownRestartRuntime>,
) {
    let town_hall = StableId::new("building:townhall").expect("static stable ID");
    if simulation.0.buildings.contains_key(&town_hall) || restart.retained_players.is_some() {
        return;
    }
    let mut retained_players = simulation
        .0
        .actors
        .values()
        .filter(|actor| {
            actor.role.as_str() != "role:enemy" && !is_guardhouse_defender_id(&actor.id)
        })
        .cloned()
        .collect::<Vec<_>>();
    if let (Some(ruler), Some(previous_role)) = (
        simulation.0.current_ruler.as_ref(),
        simulation.0.ruler_previous_role.as_ref(),
    ) && let Some(actor) = retained_players.iter_mut().find(|actor| &actor.id == ruler)
    {
        actor.role.clone_from(previous_role);
    }
    restart.retained_players = Some(retained_players);
    restart.suppress_exit_save = true;
    warn!("Town Hall fell; rebuilding the town while retaining player identities and roles");
    queue_world_loading(&mut commands);
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn sync_fish_god_presentation(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    render: Res<RenderAssets>,
    world: Res<WorldRuntime>,
    simulation: Res<SimulationRuntime>,
    existing: Query<
        (Entity, Option<&FishGodAnimation>, Option<&FishGodExitTimer>),
        With<FishGodPresentation>,
    >,
) {
    let active = simulation.0.fish_god.is_some();
    if !active {
        for (entity, animation, exit_timer) in &existing {
            if animation.is_some() {
                if exit_timer.is_none() {
                    commands.entity(entity).insert(FishGodExitTimer {
                        remaining_seconds: FISH_GOD_EXIT_DELAY_SECONDS,
                    });
                }
            } else {
                commands.entity(entity).try_despawn();
            }
        }
        return;
    }
    if existing.iter().any(|(_, animation, _)| animation.is_some()) {
        return;
    }
    let spawn = nearest_walkable(
        &world.generated,
        GridPos {
            x: config.0.world.width / 2,
            z: config.0.world.height / 2 + 6,
        },
    )
    .unwrap_or(GridPos {
        x: config.0.world.width / 2,
        z: config.0.world.height / 2,
    });
    let position = grid_to_world_on_surface(spawn, &config.0, &world.generated);
    let fish_god = archetype_by_source(&content.0, ArchetypeKind::Other, "Event_FishGod.prefab")
        .or_else(|| {
            content
                .0
                .archetypes
                .values()
                .find(|archetype| archetype.source_path.ends_with("Event_FishGod.prefab"))
        });
    let mut entity = commands.spawn((
        WorldEntity,
        FishGodPresentation,
        FishGodAnimation,
        Transform::from_translation(position),
    ));
    if let Some((archetype, scene)) = fish_god
        .and_then(|archetype| default_archetype_scene(archetype).map(|scene| (archetype, scene)))
        .filter(|(_, scene)| {
            asset_server.is_some() && converted_asset_exists(&asset_root.0, &scene.asset_path)
        })
    {
        entity.insert((
            WorldAssetRoot(
                asset_server
                    .as_deref()
                    .expect("asset server checked above")
                    .load(GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone())),
            ),
            Transform::from_translation(position)
                .with_scale(Vec3::splat(config.0.world.cell_size / 2.0)),
        ));
        if let Some(material) = prefab_material_spec(archetype, scene, &presentation.0, &render) {
            entity.insert(material);
        }
        if let Some(animation) =
            complete_converted_animation_spec(archetype, scene, &presentation.0)
        {
            entity.insert(animation);
        }
    } else {
        let scale = config.0.world.cell_size * 1.5;
        entity.insert((
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.food.clone()),
            Transform::from_translation(position + Vec3::Y * scale * 0.5)
                .with_scale(Vec3::splat(scale)),
        ));
    }
    if let Some((effect, _)) = raining_fish_effect(&presentation.0) {
        commands.spawn((
            Name::new("VFX_RainingFish (Unity parity)"),
            WorldEntity,
            FishGodPresentation,
            FallingFishEmitter {
                effect: effect.clone(),
                emission_remainder: 0.0,
                sequence: 0,
            },
            Transform::default(),
        ));
    }
}

pub(crate) fn raining_fish_effect(
    presentation: &PresentationCatalog,
) -> Option<(&StableId, &RainingFishVfxDef)> {
    presentation
        .raining_fish_effects
        .iter()
        .find(|(_, effect)| effect.source_path.ends_with(RAINING_FISH_PREFAB_SUFFIX))
}

pub(crate) fn emit_falling_fish(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    world: Res<WorldRuntime>,
    mut emitters: Query<&mut FallingFishEmitter>,
    existing: Query<(), With<FallingFish>>,
) {
    let mut visible_count = existing.iter().count();
    for mut emitter in &mut emitters {
        let Some(effect) = presentation.0.raining_fish_effects.get(&emitter.effect) else {
            continue;
        };
        let available = RAINING_FISH_RENDER_BUDGET.saturating_sub(visible_count);
        if available == 0 {
            continue;
        }
        let prewarm_count = if emitter.sequence == 0 && effect.prewarm {
            available
        } else {
            0
        };
        let render_budget =
            u16::try_from(RAINING_FISH_RENDER_BUDGET).expect("raining-fish render budget fits u16");
        let representative_rate = effect.emission_rate_per_second * f32::from(render_budget)
            / f32::from(effect.max_particles);
        emitter.emission_remainder += representative_rate * time.delta_secs();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let emitted = emitter.emission_remainder.floor() as usize;
        emitter.emission_remainder -=
            f32::from(u16::try_from(emitted).expect("bounded raining-fish emission fits u16"));
        let spawn_count = prewarm_count.max(emitted.min(available));
        for index in 0..spawn_count {
            let sequence = emitter.sequence;
            emitter.sequence = emitter.sequence.wrapping_add(1);
            let initial_age = if prewarm_count > 0 {
                effect.lifetime_seconds
                    * f32::from(u16::try_from(index).expect("render budget index fits u16"))
                    / f32::from(
                        u16::try_from(prewarm_count).expect("raining-fish render budget fits u16"),
                    )
            } else {
                0.0
            };
            spawn_falling_fish(
                &mut commands,
                effect,
                sequence,
                world.generated.seed,
                initial_age,
                &config.0,
                &world.generated,
                &render,
                asset_server.as_deref(),
                &asset_root.0,
            );
        }
        visible_count += spawn_count;
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_falling_fish(
    commands: &mut Commands,
    effect: &RainingFishVfxDef,
    sequence: u32,
    world_seed: u64,
    initial_age: f32,
    config: &GameConfig,
    world: &GeneratedWorld,
    render: &RenderAssets,
    asset_server: Option<&AssetServer>,
    asset_root: &Path,
) {
    let seed = falling_fish_seed(world_seed, sequence, 0);
    let local_offset = Vec3::new(
        (deterministic_unit(falling_fish_seed(world_seed, sequence, 1)) - 0.5)
            * effect.shape_scale[0],
        (deterministic_unit(falling_fish_seed(world_seed, sequence, 2)) - 0.5)
            * effect.shape_scale[1],
        (deterministic_unit(falling_fish_seed(world_seed, sequence, 3)) - 0.5)
            * effect.shape_scale[2],
    );
    let shape_rotation = Quat::from_euler(
        EulerRot::XYZ,
        effect.shape_rotation_degrees[0].to_radians(),
        effect.shape_rotation_degrees[1].to_radians(),
        effect.shape_rotation_degrees[2].to_radians(),
    );
    let offset = shape_rotation * local_offset;
    let size = deterministic_f32_range(
        effect.start_size,
        falling_fish_seed(world_seed, sequence, 4),
    );
    let mut particle = FallingFish {
        effect: StableId::new(format!("particle_effect:{}", effect.source_guid))
            .expect("converted effect GUID forms a stable ID"),
        sequence,
        velocity: Vec3::ZERO,
        angular_velocity: Vec3::new(
            1.2 + deterministic_unit(seed.rotate_left(7)) * 2.8,
            0.8 + deterministic_unit(seed.rotate_left(13)) * 2.2,
            1.0 + deterministic_unit(seed.rotate_left(19)) * 2.4,
        ),
        age_seconds: 0.0,
        lifetime_seconds: effect.lifetime_seconds,
        base_scale: size * 0.01,
        collision_count: 0,
    };
    let mut transform =
        Transform::from_translation(Vec3::from_array(effect.emitter_position) + offset)
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                deterministic_unit(seed.rotate_left(3)) * std::f32::consts::TAU,
                deterministic_unit(seed.rotate_left(11)) * std::f32::consts::TAU,
                deterministic_unit(seed.rotate_left(23)) * std::f32::consts::TAU,
            ))
            .with_scale(Vec3::splat(particle.base_scale));
    if initial_age > f32::EPSILON {
        let mut remaining = initial_age;
        while remaining > f32::EPSILON && particle.age_seconds < particle.lifetime_seconds {
            let step = remaining.min(0.05);
            advance_falling_fish(
                effect,
                &mut particle,
                &mut transform,
                world_seed,
                sequence,
                config,
                world,
                step,
            );
            remaining -= step;
        }
    }
    let mut entity = commands.spawn((
        Name::new("Raining fish"),
        WorldEntity,
        FishGodPresentation,
        particle,
        transform,
        bevy::light::NotShadowCaster,
        bevy::light::NotShadowReceiver,
    ));
    let authored =
        asset_server.filter(|_| converted_asset_exists(asset_root, &effect.model_asset_path));
    let critter = render
        .presentation_materials
        .get(&effect.material)
        .and_then(|material| match material {
            ResolvedMaterialHandle::Critter(material) => Some(material.clone()),
            _ => None,
        });
    if let (Some(server), Some(material)) = (authored, critter) {
        entity.insert((
            Mesh3d(
                server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset(effect.model_asset_path.clone()),
                ),
            ),
            MeshMaterial3d(material),
        ));
    } else {
        entity.insert((
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(render.food.clone()),
        ));
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn advance_falling_fish(
    effect: &RainingFishVfxDef,
    fish: &mut FallingFish,
    transform: &mut Transform,
    world_seed: u64,
    sequence: u32,
    config: &GameConfig,
    world: &GeneratedWorld,
    delta_seconds: f32,
) {
    fish.age_seconds += delta_seconds;
    let phase = fish.age_seconds * effect.noise_scroll_speed * std::f32::consts::TAU;
    let x_phase = phase
        + deterministic_unit(falling_fish_seed(world_seed, sequence, 5)) * std::f32::consts::TAU;
    let z_phase = phase
        + deterministic_unit(falling_fish_seed(world_seed, sequence, 6)) * std::f32::consts::TAU;
    fish.velocity.x +=
        (x_phase * effect.noise_frequency).sin() * effect.noise_strength[0] * delta_seconds;
    fish.velocity.z +=
        (z_phase * effect.noise_frequency).cos() * effect.noise_strength[2] * delta_seconds;
    fish.velocity.y +=
        (x_phase * effect.noise_frequency).cos() * effect.noise_strength[1] * delta_seconds;
    fish.velocity.y -= effect.gravity * 9.81 * delta_seconds;
    transform.translation += fish.velocity * delta_seconds;
    transform.rotate_local_x(fish.angular_velocity.x * delta_seconds);
    transform.rotate_local_y(fish.angular_velocity.y * delta_seconds);
    transform.rotate_local_z(fish.angular_velocity.z * delta_seconds);
    if let Some(cell) = world_to_grid(transform.translation, config) {
        let floor = terrain_height(world, cell);
        if transform.translation.y <= floor && fish.velocity.y < 0.0 {
            transform.translation.y = floor;
            fish.velocity.y = -fish.velocity.y * effect.collision_bounce;
            fish.age_seconds += fish.lifetime_seconds * effect.collision_lifetime_loss;
            fish.collision_count = fish.collision_count.saturating_add(1);
        }
    }
    let life = (fish.age_seconds / fish.lifetime_seconds).clamp(0.0, 1.0);
    let scale = effect.size_multiplier(life).unwrap_or(1.0) * fish.base_scale;
    transform.scale = Vec3::splat(scale.max(0.0));
}

pub(crate) fn animate_falling_fish(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    presentation: Res<RuntimePresentation>,
    world: Res<WorldRuntime>,
    mut fish: Query<(Entity, &mut FallingFish, &mut Transform)>,
) {
    for (entity, mut fish, mut transform) in &mut fish {
        let Some(effect) = presentation.0.raining_fish_effects.get(&fish.effect) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let sequence = fish.sequence;
        advance_falling_fish(
            effect,
            &mut fish,
            &mut transform,
            world.generated.seed,
            sequence,
            &config.0,
            &world.generated,
            time.delta_secs(),
        );
        if fish.age_seconds >= fish.lifetime_seconds {
            commands.entity(entity).try_despawn();
        }
    }
}

pub(crate) fn falling_fish_seed(world_seed: u64, sequence: u32, salt: u32) -> u32 {
    let low = u32::try_from(world_seed & u64::from(u32::MAX)).expect("masked seed fits u32");
    low.wrapping_add(sequence.wrapping_mul(0x9E37_79B9))
        .wrapping_add(salt.wrapping_mul(0x85EB_CA6B))
}

pub(crate) fn drive_fish_god_exit(
    mut commands: Commands,
    time: Res<Time>,
    parents: Query<&ChildOf>,
    mut roots: Query<
        (
            Entity,
            &mut FishGodExitTimer,
            Option<&FishGodExitTriggerSent>,
        ),
        With<FishGodAnimation>,
    >,
    mut drivers: Query<&mut ConvertedAnimationDriver>,
) {
    for (entity, mut exit, trigger_already_sent) in &mut roots {
        let mut trigger_sent = false;
        for mut driver in &mut drivers {
            if trigger_already_sent.is_some() {
                break;
            }
            let mut ancestor = driver.actor_root;
            let mut belongs_to_fish_god = ancestor == entity;
            for _ in 0..64 {
                if belongs_to_fish_god {
                    break;
                }
                let Ok(parent) = parents.get(ancestor) else {
                    break;
                };
                ancestor = parent.parent();
                belongs_to_fish_god = ancestor == entity;
            }
            if !belongs_to_fish_god {
                continue;
            }
            for layer in &mut driver.layers {
                let _ = layer.runtime.set_trigger("Exit");
            }
            trigger_sent = true;
        }
        if trigger_sent {
            commands.entity(entity).insert(FishGodExitTriggerSent);
        }
        exit.remaining_seconds -= time.delta_secs();
        if exit.remaining_seconds <= 0.0 {
            commands.entity(entity).try_despawn();
        }
    }
}
