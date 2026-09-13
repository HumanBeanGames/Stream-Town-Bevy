use super::super::*;

pub(crate) fn snapshot_world(
    world: &WorldRuntime,
    stats: &SessionStats,
    simulation: &SimulationRuntime,
    traversal_wear: &TraversalWearRuntime,
) -> WorldSnapshot {
    WorldSnapshot {
        schema_version: CURRENT_WORLD_SNAPSHOT_SCHEMA,
        world_seed: world.generated.seed,
        generator_version: world.generated.generator_version,
        world_hash: world.generated.deterministic_hash.clone(),
        elapsed_seconds: Duration::from_secs_f64(stats.elapsed_seconds.max(0.0)).as_secs(),
        actors: simulation
            .0
            .actors
            .values()
            .map(|actor| {
                let kind = if actor.role.as_str() == "role:enemy" {
                    ActorKind::Enemy
                } else {
                    ActorKind::Player
                };
                let archetype = actor.archetype.clone().unwrap_or_else(|| {
                    StableId::new(match kind {
                        ActorKind::Enemy => "archetype:enemy",
                        _ => "archetype:viewer",
                    })
                    .expect("fallback actor archetype IDs are valid")
                });
                SavedActor {
                    id: actor.id.clone(),
                    kind,
                    archetype,
                    grid_position: actor.position,
                    height_centimetres: world
                        .generated
                        .navigation
                        .height_at(actor.position)
                        .unwrap_or_default(),
                    health: actor.health,
                }
            })
            .collect(),
        simulation: simulation.0.clone(),
        resource_nodes: world
            .generated
            .resources
            .iter()
            .map(|resource| (resource.id.clone(), resource.amount))
            .collect(),
        traversal_wear: traversal_wear.saved_cells(),
        legacy_terrain_mesh: world.legacy_terrain_mesh.clone(),
        legacy_migration: world.legacy_migration.clone(),
    }
}

pub(crate) fn spawn_credits(
    mut commands: Commands,
    render: Res<RenderAssets>,
    presentation: Res<RuntimePresentation>,
) {
    commands.insert_resource(CreditsTimeline::default());
    spawn_cloud_field(&mut commands, &render, 55.0);
    if let Some(game_logo) = &render.game_logo {
        commands.spawn((
            StateEntity,
            Name::new("Credits logo"),
            ImageNode::new(game_logo.clone()),
            GlobalZIndex(10),
            Node {
                position_type: PositionType::Absolute,
                top: percent(4.0),
                left: percent(37.5),
                width: percent(25.0),
                aspect_ratio: Some(GAME_LOGO_ASPECT_RATIO),
                ..default()
            },
        ));
    } else {
        commands.spawn((
            StateEntity,
            Text::new("STREAM TOWN"),
            TextFont {
                font_size: FontSize::Px(54.0),
                ..default()
            },
            TextLayout::justify(Justify::Center),
            TextColor(Color::srgb(0.86, 0.95, 0.84)),
            GlobalZIndex(10),
            Node {
                position_type: PositionType::Absolute,
                top: percent(12.0),
                left: percent(37.0),
                ..default()
            },
        ));
    }
    for (target_path, text) in [
        ("CreatedBy_Canvas", "Created By\nHuman Bean Games"),
        ("ProjectLead_Canvas", "Project Lead\nJayden Hunter"),
        (
            "3DArt_Canvas",
            "Lead Artist\nBen Lindridge\n\n3D Artists\nAaron Hunter | Bella Kovac | William Trimble",
        ),
        (
            "OtherArt_Canvas",
            "Producer and Game Design\nAndrew Nardi | Benjamin Babicka | Jayden Hunter",
        ),
        (
            "AudioAndSpecialThanks_Canvas",
            "Composer\nLeah Crimmins\n\nAudio Design\nFrancisco Inigo Garde | Andrew Nardi\n\nSpecial Thanks\nStream Town community",
        ),
        (
            "Progammer_Canvas",
            "Lead Programmer\nJayden Hunter\n\nProgrammers\nDeclan Doller | Benjamin Babicka",
        ),
        (
            "Design_Canvas",
            "Animation, Technical Art, UI and VFX\nAaron Hunter | Lam Nguyen | Ben Lindridge | Benjamin Babicka | Andrew Nardi",
        ),
    ] {
        commands.spawn((
            StateEntity,
            AuthoredCreditsElement {
                target_path: target_path.into(),
            },
            Text::new(text),
            TextFont {
                font_size: FontSize::Px(32.0),
                ..default()
            },
            TextLayout::justify(Justify::Center),
            TextColor(Color::srgb(0.86, 0.95, 0.84)),
            GlobalZIndex(10),
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                top: percent(31.0),
                left: percent(20.0),
                width: percent(60.0),
                ..default()
            },
        ));
    }
    spawn_credits_fireworks_emitters(&mut commands, &presentation.0);
    commands
        .spawn((
            StateEntity,
            CreditsSkipButton,
            Name::new("Shipping Credits skip button"),
            Button,
            authored_ui_image(
                &render,
                MAIN_MENU_TEXTURE_PATHS[1],
                main_menu_texture(&render, MAIN_MENU_TEXTURE_PATHS[1]),
            ),
            GlobalZIndex(30),
            Node {
                position_type: PositionType::Absolute,
                right: percent(1.0),
                bottom: percent(1.0),
                width: percent(7.5),
                height: percent(4.5),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|button| {
            button.spawn((
                Text::new("SKIP"),
                TextFont {
                    font_size: FontSize::Px(31.2),
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
        });
    commands.spawn((
        StateEntity,
        CreditsFade,
        GlobalZIndex(20),
        BackgroundColor(Color::BLACK),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100.0),
            height: percent(100.0),
            ..default()
        },
    ));
}

pub(crate) fn drive_credits_animation(
    time: Res<Time>,
    presentation: Res<RuntimePresentation>,
    mut timeline: ResMut<CreditsTimeline>,
    mut elements: Query<(&AuthoredCreditsElement, &mut Visibility)>,
    mut fades: Query<&mut BackgroundColor, With<CreditsFade>>,
) {
    timeline.elapsed_seconds = (timeline.elapsed_seconds + time.delta_secs()).min(65.5);
    for (element, mut visibility) in &mut elements {
        let active = animation_property_value(
            &presentation.0,
            "CreditsPopups",
            &element.target_path,
            "m_IsActive",
            timeline.elapsed_seconds,
        )
        .is_some_and(|value| value >= 0.5);
        *visibility = if active {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    let alpha = animation_property_value(
        &presentation.0,
        "EndCredits",
        "Panel",
        "m_Color.a",
        timeline.elapsed_seconds,
    )
    .unwrap_or(0.0)
    .clamp(0.0, 1.0);
    for mut background in &mut fades {
        background.0.set_alpha(alpha);
    }
}

pub(crate) fn spawn_credits_fireworks_emitters(
    commands: &mut Commands,
    presentation: &PresentationCatalog,
) {
    let Some(bindings) = presentation.scene_fireworks.get(CREDITS_SCENE_PATH) else {
        return;
    };
    for (emitter_index, binding) in bindings.iter().enumerate() {
        commands.spawn((
            StateEntity,
            Name::new(format!("Credits fireworks: {}", binding.hierarchy_path)),
            CreditsFireworksEmitter {
                target_path: binding.hierarchy_path.clone(),
                effect: binding.effect.clone(),
                emitter_index: u8::try_from(emitter_index).unwrap_or(u8::MAX),
                origin_percent: credits_firework_origin(binding.local_position),
                next_launch_index: 0,
                active_rockets: 0,
            },
        ));
    }
}

pub(crate) fn update_credits_fireworks(
    mut commands: Commands,
    time: Res<Time>,
    settings: Option<Res<RuntimePlayerSettings>>,
    presentation: Res<RuntimePresentation>,
    timeline: Res<CreditsTimeline>,
    mut emitters: Query<(Entity, &mut CreditsFireworksEmitter)>,
    mut particles: Query<(
        Entity,
        &mut CreditsFireworkParticle,
        &mut Node,
        &mut BackgroundColor,
    )>,
    mut bursts: Query<(Entity, &mut CreditsFireworkBurst)>,
) {
    if settings.is_some_and(|settings| settings.0.interface.reduced_motion) {
        for (entity, ..) in &mut particles {
            commands.entity(entity).try_despawn();
        }
        for (entity, _) in &mut bursts {
            commands.entity(entity).try_despawn();
        }
        return;
    }
    let delta_seconds = time.delta_secs().max(0.0);
    for (entity, mut emitter) in &mut emitters {
        let active = credits_fireworks_active(
            &presentation.0,
            &emitter.target_path,
            timeline.elapsed_seconds,
        );
        let Some(effect) = presentation.0.fireworks_effects.get(&emitter.effect) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let expected_launches = if active {
            expected_firework_launches(
                timeline.elapsed_seconds,
                credits_fireworks_start(&presentation.0, &emitter.target_path),
                effect.launch_rate_per_second,
            )
        } else {
            0
        };
        while emitter.next_launch_index < expected_launches
            && emitter.active_rockets < effect.rocket_capacity
        {
            let _ = spawn_credits_firework_particle(
                &mut commands,
                effect,
                &emitter.effect,
                emitter.emitter_index,
                emitter.next_launch_index,
                CreditsFireworkParticleKind::Rocket,
                emitter.origin_percent,
                0,
            );
            emitter.next_launch_index += 1;
            emitter.active_rockets += 1;
        }
    }

    let mut completed_rockets = Vec::new();
    for (entity, mut particle, mut node, mut background) in &mut particles {
        particle.age_seconds += delta_seconds;
        let life = (particle.age_seconds / particle.lifetime_seconds).clamp(0.0, 1.0);
        particle.velocity_percent_per_second.y -= 5.4 * delta_seconds;
        let velocity = particle.velocity_percent_per_second;
        particle.position_percent += velocity * delta_seconds;
        node.left = percent(particle.position_percent.x);
        node.top = percent(particle.position_percent.y);
        let alpha = firework_particle_alpha(particle.kind, life);
        background.0.set_alpha(alpha);
        let scale = firework_particle_scale(particle.kind, life);
        node.width = percent(scale);
        node.height = percent(scale * 16.0 / 9.0);
        if particle.age_seconds >= particle.lifetime_seconds {
            if particle.kind == CreditsFireworkParticleKind::Rocket {
                completed_rockets.push((
                    particle.effect.clone(),
                    particle.emitter_index,
                    particle.sequence,
                    particle.position_percent,
                    particle.color_index,
                ));
            }
            commands.entity(entity).try_despawn();
        }
    }

    for (effect_id, emitter_index, sequence, position, color_index) in completed_rockets {
        if let Some(effect) = presentation.0.fireworks_effects.get(&effect_id) {
            commands.spawn((
                StateEntity,
                CreditsFireworkBurst {
                    effect: effect_id.clone(),
                    emitter_index,
                    sequence,
                    position_percent: position,
                    color_index,
                    remaining: deterministic_u16_range(
                        effect.burst_count,
                        firework_seed(emitter_index, sequence, 91),
                    ),
                    delay_seconds: deterministic_f32_range(
                        effect.burst_delay_seconds,
                        firework_seed(emitter_index, sequence, 97),
                    ),
                },
            ));
            let _ = spawn_credits_firework_particle(
                &mut commands,
                effect,
                &effect_id,
                emitter_index,
                sequence,
                CreditsFireworkParticleKind::BurstFlash,
                position,
                color_index,
            );
        }
        if let Some((_, mut emitter)) = emitters
            .iter_mut()
            .find(|(_, emitter)| emitter.emitter_index == emitter_index)
        {
            emitter.active_rockets = emitter.active_rockets.saturating_sub(1);
        }
    }

    for (entity, mut burst) in &mut bursts {
        burst.delay_seconds -= delta_seconds;
        if burst.delay_seconds > 0.0 {
            continue;
        }
        let Some(effect) = presentation.0.fireworks_effects.get(&burst.effect) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        for burst_index in 0..burst.remaining {
            for spark_index in 0..effect.burst_particle_rate {
                let sequence = u32::from(burst_index)
                    .saturating_mul(u32::from(effect.burst_particle_rate))
                    .saturating_add(u32::from(spark_index));
                let _ = spawn_credits_firework_particle(
                    &mut commands,
                    effect,
                    &burst.effect,
                    burst.emitter_index,
                    burst.sequence.wrapping_add(sequence),
                    CreditsFireworkParticleKind::Spark,
                    burst.position_percent,
                    burst.color_index,
                );
            }
        }
        commands.entity(entity).try_despawn();
    }
}

pub(crate) fn credits_fireworks_start(
    presentation: &PresentationCatalog,
    target_path: &str,
) -> f32 {
    presentation
        .clips
        .values()
        .find(|clip| clip.display_name == "CreditsFireworks")
        .and_then(|clip| {
            clip.property_curves
                .iter()
                .find(|curve| curve.target_path == target_path && curve.attribute == "m_IsActive")
        })
        .and_then(|curve| {
            curve
                .keys
                .iter()
                .find(|key| key.value >= 0.5)
                .map(|key| key.time)
        })
        .unwrap_or(f32::INFINITY)
}

pub(crate) fn credits_fireworks_active(
    presentation: &PresentationCatalog,
    target_path: &str,
    time: f32,
) -> bool {
    animation_property_value(
        presentation,
        "CreditsFireworks",
        target_path,
        "m_IsActive",
        time,
    )
    .is_some_and(|value| value >= 0.5)
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
pub(crate) fn expected_firework_launches(time: f32, start: f32, rate: f32) -> u32 {
    if !time.is_finite() || !start.is_finite() || !rate.is_finite() || time < start || rate <= 0.0 {
        return 0;
    }
    ((time - start) * rate).floor().clamp(0.0, u32::MAX as f32) as u32 + 1
}

pub(crate) fn credits_firework_origin(local_position: [f32; 3]) -> Vec2 {
    if local_position[2] > -34.0 {
        Vec2::new(25.0, 92.0)
    } else {
        Vec2::new(75.0, 92.0)
    }
}

pub(crate) fn spawn_credits_firework_particle(
    commands: &mut Commands,
    effect: &FireworksVfxDef,
    effect_id: &StableId,
    emitter_index: u8,
    sequence: u32,
    kind: CreditsFireworkParticleKind,
    origin: Vec2,
    inherited_color_index: usize,
) -> Entity {
    let color_index = if kind == CreditsFireworkParticleKind::Rocket {
        usize::try_from(firework_seed(emitter_index, sequence, 11)).unwrap_or_default()
            % effect.colors.len()
    } else {
        inherited_color_index % effect.colors.len()
    };
    let color = effect.colors[color_index];
    let velocity = match kind {
        CreditsFireworkParticleKind::Rocket => Vec2::new(
            deterministic_f32_range(
                [effect.rocket_velocity_min[0], effect.rocket_velocity_max[0]],
                firework_seed(emitter_index, sequence, 17),
            ) * 1.5,
            -deterministic_f32_range(
                [effect.rocket_velocity_min[1], effect.rocket_velocity_max[1]],
                firework_seed(emitter_index, sequence, 23),
            ) * 1.75,
        ),
        CreditsFireworkParticleKind::BurstFlash => Vec2::ZERO,
        CreditsFireworkParticleKind::Spark => {
            let angle = deterministic_unit(firework_seed(emitter_index, sequence, 31))
                * std::f32::consts::TAU;
            let speed = effect.sparks_speed
                * (2.0 + deterministic_unit(firework_seed(emitter_index, sequence, 37)) * 3.0);
            Vec2::new(angle.cos(), angle.sin()) * Vec2::new(speed, speed * 16.0 / 9.0)
        }
    };
    let lifetime = match kind {
        CreditsFireworkParticleKind::Rocket => deterministic_f32_range(
            effect.rocket_lifetime_seconds,
            firework_seed(emitter_index, sequence, 41),
        ),
        CreditsFireworkParticleKind::BurstFlash => deterministic_f32_range(
            effect.burst_lifetime_seconds,
            firework_seed(emitter_index, sequence, 43),
        ),
        CreditsFireworkParticleKind::Spark => deterministic_f32_range(
            effect.spark_lifetime_seconds,
            firework_seed(emitter_index, sequence, 47),
        ),
    };
    commands
        .spawn((
            StateEntity,
            Name::new(format!("Credits firework {kind:?}")),
            CreditsFireworkParticle {
                kind,
                effect: effect_id.clone(),
                emitter_index,
                sequence,
                position_percent: origin,
                velocity_percent_per_second: velocity,
                age_seconds: 0.0,
                lifetime_seconds: lifetime.max(0.01),
                color_index,
            },
            GlobalZIndex(9),
            BackgroundColor(Color::linear_rgba(color[0], color[1], color[2], color[3])),
            Node {
                position_type: PositionType::Absolute,
                left: percent(origin.x),
                top: percent(origin.y),
                width: percent(firework_particle_scale(kind, 0.0)),
                height: percent(firework_particle_scale(kind, 0.0) * 16.0 / 9.0),
                border_radius: BorderRadius::MAX,
                ..default()
            },
        ))
        .id()
}

pub(crate) fn firework_particle_alpha(kind: CreditsFireworkParticleKind, life: f32) -> f32 {
    match kind {
        CreditsFireworkParticleKind::Rocket => (1.0 - life).clamp(0.0, 1.0),
        CreditsFireworkParticleKind::BurstFlash => {
            ((life / 0.1).min(1.0) * (1.0 - life)).clamp(0.0, 1.0)
        }
        CreditsFireworkParticleKind::Spark => {
            ((life / 0.1).min(1.0) * ((1.0 - life) / 0.2).min(1.0)).clamp(0.0, 1.0)
        }
    }
}

pub(crate) fn firework_particle_scale(kind: CreditsFireworkParticleKind, life: f32) -> f32 {
    match kind {
        CreditsFireworkParticleKind::Rocket => 0.28 + life * 0.12,
        CreditsFireworkParticleKind::BurstFlash => 0.45 + life * 2.6,
        CreditsFireworkParticleKind::Spark => 0.20 * (1.0 - life * 0.55),
    }
}

pub(crate) fn firework_seed(emitter_index: u8, sequence: u32, salt: u32) -> u32 {
    sequence
        .wrapping_mul(0x9E37_79B9)
        .wrapping_add(u32::from(emitter_index).wrapping_mul(0x85EB_CA6B))
        .wrapping_add(salt)
}

pub(crate) fn deterministic_unit(seed: u32) -> f32 {
    (pseudo_noise(seed) + 1.0) * 0.5
}

pub(crate) fn deterministic_f32_range(range: [f32; 2], seed: u32) -> f32 {
    range[0] + (range[1] - range[0]) * deterministic_unit(seed)
}

pub(crate) fn deterministic_u16_range(range: [u16; 2], seed: u32) -> u16 {
    let width = u32::from(range[1] - range[0]) + 1;
    range[0] + u16::try_from(seed % width).unwrap_or_default()
}

pub(crate) fn animation_property_value(
    presentation: &PresentationCatalog,
    clip_name: &str,
    target_path: &str,
    attribute: &str,
    time: f32,
) -> Option<f32> {
    presentation
        .clips
        .values()
        .find(|clip| clip.display_name == clip_name)?
        .property_curves
        .iter()
        .find(|curve| curve.target_path == target_path && curve.attribute == attribute)?
        .sample(time)
}

pub(crate) fn spawn_level_up_toast(mut commands: Commands, simulation: Res<SimulationRuntime>) {
    let actor_levels = simulation
        .0
        .actors
        .iter()
        .map(|(id, actor)| (id.clone(), role_progress(actor).level))
        .collect();
    commands.insert_resource(LevelUpPresentation {
        actor_levels,
        elapsed_seconds: None,
    });
    commands.spawn((
        WorldEntity,
        LevelUpToast,
        Text::new("LEVEL UP"),
        TextFont {
            font_size: FontSize::Px(26.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        TextColor(Color::srgba(0.93, 0.82, 0.46, 0.0)),
        GlobalZIndex(19),
        Visibility::Hidden,
        Node {
            position_type: PositionType::Absolute,
            top: px(96),
            left: percent(40.0),
            width: percent(20.0),
            overflow: Overflow::clip(),
            ..default()
        },
    ));
}

pub(crate) fn drive_level_up_presentation(
    time: Res<Time>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    content: Res<RuntimeContent>,
    twitch: Res<TwitchConnection>,
    simulation: Res<SimulationRuntime>,
    mut state: ResMut<LevelUpPresentation>,
    mut toast: Query<(&mut Text, &mut TextColor, &mut Visibility, &mut Node), With<LevelUpToast>>,
) {
    let mut leveled_actor = None;
    for (id, actor) in &simulation.0.actors {
        let level = role_progress(actor).level;
        if let Some(previous) = state.actor_levels.insert(id.clone(), level)
            && level > previous
        {
            leveled_actor = Some((id.clone(), level));
            if id.as_str().starts_with("twitch:") {
                let interval = content.0.progression.level_announcement_interval;
                let first = previous.saturating_add(1);
                for announced_level in first..=level {
                    if announced_level.is_multiple_of(interval) {
                        let label = actor.display_name.as_deref().unwrap_or_else(|| id.as_str());
                        let role = content
                            .0
                            .roles
                            .get(&actor.role)
                            .map_or(actor.role.as_str(), |role| role.display_name.as_str());
                        send_command_feedback(
                            &twitch,
                            format!("{label} reached {role} level {announced_level}!"),
                        );
                    }
                }
            }
        }
    }
    if leveled_actor.is_some() {
        state.elapsed_seconds = Some(0.0);
    }
    let Ok((mut text, mut color, mut visibility, mut node)) = toast.single_mut() else {
        return;
    };
    if let Some((actor, level)) = leveled_actor {
        let label = simulation
            .0
            .actors
            .get(&actor)
            .and_then(|actor| actor.display_name.as_deref())
            .unwrap_or_else(|| actor.as_str());
        **text = format!("LEVEL UP\n{label} | {level}");
    }
    let Some(elapsed) = state.elapsed_seconds.as_mut() else {
        *visibility = Visibility::Hidden;
        return;
    };
    *elapsed += time.delta_secs();
    if *elapsed > 1.0 {
        state.elapsed_seconds = None;
        *visibility = Visibility::Hidden;
        return;
    }
    let y = animation_property_value(
        &presentation.0,
        "LevelUp_Animation",
        "LevelUpText",
        "m_AnchoredPosition.y",
        *elapsed,
    )
    .unwrap_or(0.0);
    let alpha = animation_property_value(
        &presentation.0,
        "LevelUp_Animation",
        "LevelUpText",
        "m_fontColor.a",
        *elapsed,
    )
    .unwrap_or(1.0)
    .clamp(0.0, 1.0);
    node.top = px(96.0 - y * render.main_ui_scale);
    color.0 = Color::srgba(0.93, 0.82, 0.46, alpha * 0.88);
    *visibility = Visibility::Inherited;
}

pub(crate) fn credits_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

pub(crate) fn credits_skip_button(
    buttons: Query<&Interaction, (Changed<Interaction>, With<CreditsSkipButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        next_state.set(GameState::MainMenu);
    }
}
