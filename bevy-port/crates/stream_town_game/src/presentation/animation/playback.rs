pub(crate) fn drive_native_animations(
    agents: Query<(&Agent, &AgentLocomotion)>,
    mut players: Query<(&mut AnimationPlayer, &mut ActorAnimationDriver)>,
) {
    for (mut player, mut driver) in &mut players {
        let Ok((_, locomotion)) = agents.get(driver.actor_root) else {
            continue;
        };
        let next = if locomotion.normalized_speed > 0.01 {
            MovementAnimationState::Moving
        } else {
            MovementAnimationState::Idle
        };
        if next == driver.current {
            continue;
        }
        driver.current = next;
        let node = match next {
            MovementAnimationState::Idle => driver.idle,
            MovementAnimationState::Moving => driver.moving,
        };
        player.stop_all().play(node).repeat();
    }
}

pub(crate) fn drive_converted_animations(
    mut commands: Commands,
    cadence: Res<AgentSimulationCadence>,
    player_settings: Res<RuntimePlayerSettings>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    simulation: Res<SimulationRuntime>,
    agents: Query<(&Agent, &AgentLocomotion)>,
    agent_transforms: Query<&GlobalTransform, With<Agent>>,
    camera: Query<&GlobalTransform, (With<TownCamera>, Without<Agent>)>,
    pets: Query<&ActivePetVisual>,
    fish_gods: Query<(), With<FishGodAnimation>>,
    mut players: Query<(&mut AnimationPlayer, &mut ConvertedAnimationDriver)>,
    mut audio_cache: ResMut<RoleActionAudioCache>,
    mut procedural_audio: Option<ResMut<Assets<AudioSource>>>,
    #[cfg(target_os = "windows")] native_audio: Res<direct_broadcast::NativeGameAudioRouting>,
) {
    let mut audio_cues = Vec::new();
    for (mut player, mut driver) in &mut players {
        let agent = agents.get(driver.actor_root).ok();
        let pet = pets.get(driver.actor_root).ok();
        let fish_god = fish_gods.contains(driver.actor_root);
        if agent.is_none() && pet.is_none() && !fish_god {
            continue;
        }
        let Some(controller) = presentation.0.controllers.get(&driver.controller) else {
            continue;
        };
        let move_speed = if let Some(pet) = pet {
            pet.movement_speed
        } else {
            agent.map_or(0.0, |(_, locomotion)| locomotion.normalized_speed)
        };
        for layer in &mut driver.layers {
            let _ = layer.runtime.set_float("Move Speed", move_speed);
            let _ = layer.runtime.set_float("MoveSpeed", move_speed);
        }

        if let Some((agent, _)) = agent
            && let Some(alive) = simulation.0.actors.get(&agent.id).map(|actor| actor.alive)
        {
            if let Some(previous) = driver.last_alive
                && alive != previous
            {
                let trigger = if alive { "Revive" } else { "Death" };
                for layer in &mut driver.layers {
                    let _ = layer.runtime.set_trigger(trigger);
                }
            }
            driver.last_alive = Some(alive);
        }
        if let Some((agent, _)) = agent
            && let Some(actor) = simulation.0.actors.get(&agent.id)
        {
            let enemy_contract = enemy_animation_contract(&content.0, actor, &agent.id);
            let run_animation = enemy_contract
                .as_ref()
                .map_or(EnemyRunAnimation::Generic, |contract| {
                    contract.run_animation
                });
            for layer in &mut driver.layers {
                let _ = layer
                    .runtime
                    .set_integer("RunAnimationIndex", run_animation.controller_index());
            }
            let action = agent_action_animation(&content.0, agent, actor);
            if driver.active_action.as_deref() != action.as_deref() {
                if let Some(previous) = driver.active_action.take() {
                    for layer in &mut driver.layers {
                        let _ = layer.runtime.reset_trigger(&previous);
                    }
                }
                for layer in &mut driver.layers {
                    let _ = layer.runtime.set_boolean("Action", action.is_some());
                }
                if let Some(action) = &action {
                    let variants = enemy_contract.as_ref().map_or_else(
                        || {
                            content
                                .0
                                .roles
                                .get(&actor.role)
                                .map_or(1, |role| role.action_animation_variants.max(1))
                        },
                        |contract| contract.action_animation_variants,
                    );
                    let index = deterministic_animation_variant(&agent.id, action, variants);
                    let speed = action_animation_speed(&content.0, &simulation.0, actor);
                    for layer in &mut driver.layers {
                        let _ = layer.runtime.set_trigger(action);
                        let _ = layer
                            .runtime
                            .set_integer("AnimationIndex", i32::from(index));
                        let _ = layer.runtime.set_float("ActionSpeed", speed);
                    }
                }
                driver.active_action = action;
            }
            let carrying = actor_carries_role_resource(&content.0, actor);
            let carry_kind = content
                .0
                .roles
                .get(&actor.role)
                .and_then(|role| role.equipment.as_ref())
                .and_then(|equipment| equipment.carry_animation.as_deref());
            for layer in &mut driver.layers {
                let _ = layer
                    .runtime
                    .set_boolean("CarryWood", carrying && carry_kind == Some("Carry Wood"));
                let _ = layer
                    .runtime
                    .set_boolean("CarryHip", carrying && carry_kind == Some("Carry Hip"));
            }
        }

        let actor_root = driver.actor_root;
        let mut combined = Vec::new();
        let mut restarts = Vec::new();
        let mut transient_carry_visible = driver.transient_carry_visible;
        let mut carry_event_clip_selected = false;
        for layer in &mut driver.layers {
            let source_selection = layer.runtime.motion_selection(controller).ok().flatten();
            let source_speed = layer
                .runtime
                .state_speed(controller)
                .unwrap_or(1.0)
                .abs()
                .max(f32::EPSILON);
            let source_duration = source_selection.as_ref().map_or(0.0, |selection| {
                animation_selection_duration(selection, &presentation.0) / source_speed
            });
            let normalized_time = if controller.states[layer.runtime.current_state()]
                .motions
                .is_empty()
            {
                1.0
            } else {
                current_normalized_time(&player, layer, &presentation.0)
            };
            let transition = layer
                .runtime
                .evaluate_transitions(controller, normalized_time)
                .ok();
            let transition_playback = layer.runtime.take_transition_playback();
            let transition_playback = resolve_animation_exit_fallback(
                &mut layer.runtime,
                controller,
                &layer.fallback_state,
                transition.as_ref(),
                transition_playback,
            );
            if let Some(playback) = transition_playback {
                begin_animation_crossfade(layer, playback, source_duration);
            }

            let Ok(Some(selection)) = layer.runtime.motion_selection(controller) else {
                layer.active.clear();
                layer.applied.clear();
                layer.crossfade = None;
                layer.event_elapsed.clear();
                continue;
            };
            let desired = animation_nodes_for_selection(&selection, &layer.nodes);
            if transition_playback.is_some() {
                for (node, _) in &desired {
                    let offset_seconds =
                        layer
                            .nodes
                            .iter()
                            .find_map(|(clip, candidate)| {
                                (candidate == node).then(|| {
                                    presentation.0.clips.get(clip).map_or(0.0, |clip| {
                                        clip.duration_seconds * layer.state_offset
                                    })
                                })
                            })
                            .unwrap_or(0.0);
                    restarts.push((*node, offset_seconds));
                }
            }
            if let Some((agent, _)) = agent
                && let Ok(transform) = agent_transforms.get(actor_root)
            {
                carry_event_clip_selected |= collect_animation_events(
                    &player,
                    layer,
                    &selection,
                    &presentation.0,
                    &agent.id,
                    transform.translation(),
                    &mut audio_cues,
                    &mut transient_carry_visible,
                );
            }
            let state_speed = layer.runtime.state_speed(controller).unwrap_or(1.0);
            let destination = animation_playback_for_selection(
                &selection,
                &layer.nodes,
                &presentation.0,
                state_speed,
            );
            let applied = advance_animation_crossfade(
                &mut layer.crossfade,
                &destination,
                cadence.delta().as_secs_f32(),
            );
            combined.extend(applied.iter().copied());
            layer.active = desired;
            layer.applied = applied;
        }
        if !carry_event_clip_selected {
            transient_carry_visible = false;
        }
        driver.transient_carry_visible = transient_carry_visible;
        if agent.is_some() {
            commands
                .entity(actor_root)
                .insert(TransientCarryVisibility(transient_carry_visible));
        }
        apply_animation_blend(&mut player, &combined, &restarts);
    }
    if let Some(audio_sources) = procedural_audio.as_mut() {
        let camera_position = camera.single().ok().map(GlobalTransform::translation);
        for cue in audio_cues {
            let Some(role) = simulation.0.actors.get(&cue.actor).map(|actor| &actor.role) else {
                continue;
            };
            let Some(variant_guid) =
                role_action_audio_variant(&presentation.0, &cue.actor, role, &cue.clip)
            else {
                continue;
            };
            if camera_position.is_some_and(|camera| {
                camera.distance(cue.position) > ROLE_ACTION_AUDIO_MAX_DISTANCE
            }) {
                continue;
            }
            let cached = audio_cache
                .0
                .entry(variant_guid.to_owned())
                .or_insert_with(|| {
                    let wav: Arc<[u8]> = procedural_role_action_wav(
                        role,
                        &cue.display_name,
                        variant_guid,
                        PROCEDURAL_AUDIO_SAMPLE_RATE,
                    )
                    .into();
                    let source = audio_sources.add(AudioSource { bytes: wav.clone() });
                    CachedRoleActionAudio { source, wav }
                });
            let gain =
                0.24 * player_settings.0.audio.master * player_settings.0.audio.sound_effects;
            #[cfg(target_os = "windows")]
            native_audio.play_pcm16_wav(
                &format!("role-action:{}:{}", cue.actor, variant_guid),
                cached.wav.as_ref(),
                gain,
            );
            #[cfg(target_os = "windows")]
            if !native_audio.local_monitor_enabled() {
                continue;
            }
            commands.spawn((
                WorldEntity,
                Name::new(format!(
                    "Role action audio: {} / {} ({})",
                    role, cue.display_name, cue.actor
                )),
                AudioPlayer(cached.source.clone()),
                PlaybackSettings::DESPAWN
                    .with_volume(Volume::Linear(gain))
                    .with_spatial(true)
                    .with_spatial_scale(SpatialScale::new(1.0 / ROLE_ACTION_AUDIO_MAX_DISTANCE)),
                Transform::from_translation(cue.position),
            ));
        }
    }
}

pub(crate) fn resolve_animation_exit_fallback(
    runtime: &mut AnimationControllerRuntime,
    controller: &stream_town_domain::AnimationControllerDef,
    fallback_state: &StableId,
    transition: Option<&stream_town_domain::AnimationTransitionOutcome>,
    playback: Option<AnimationTransitionPlayback>,
) -> Option<AnimationTransitionPlayback> {
    if !matches!(
        transition,
        Some(stream_town_domain::AnimationTransitionOutcome::Exited)
    ) {
        return playback;
    }
    // A root state-machine Exit has no serialized destination. Unity keeps
    // the layer in its current fallback state in this case. Re-entering that
    // same state made Bevy call AnimationPlayer::start at every run boundary,
    // resetting elapsed/completion state before RepeatAnimation could wrap.
    if runtime.current_state() == fallback_state {
        return None;
    }
    let _ = runtime.enter_state(controller, fallback_state.clone());
    playback
}

pub(crate) fn collect_animation_events(
    player: &AnimationPlayer,
    layer: &mut ConvertedAnimationLayerDriver,
    selection: &AnimationBlendSelection,
    presentation: &PresentationCatalog,
    actor: &StableId,
    position: Vec3,
    output: &mut Vec<PendingRoleActionAudio>,
    transient_carry_visible: &mut bool,
) -> bool {
    let mut selected = BTreeSet::new();
    let mut carry_event_clip_selected = false;
    for motion in std::iter::once(&selection.first).chain(selection.second.as_ref()) {
        if motion.weight <= f32::EPSILON {
            continue;
        }
        selected.insert(motion.clip.clone());
        let Some(node) = layer.nodes.get(&motion.clip) else {
            continue;
        };
        let Some(active) = player.animation(*node) else {
            layer.event_elapsed.remove(&motion.clip);
            continue;
        };
        let current =
            active.elapsed() + clip_event_offset_seconds(layer, &motion.clip, presentation);
        let previous = layer.event_elapsed.insert(motion.clip.clone(), current);
        let Some(clip) = presentation.clips.get(&motion.clip) else {
            continue;
        };
        for event in &clip.events {
            let occurrences =
                animation_event_occurrences(event.time, clip.duration_seconds, previous, current);
            if apply_transient_carry_event(
                &event.function_name,
                occurrences,
                transient_carry_visible,
            ) {
                carry_event_clip_selected = true;
                continue;
            }
            if event.function_name == "PlayRoleActionAudio" {
                for _ in 0..occurrences {
                    output.push(PendingRoleActionAudio {
                        actor: actor.clone(),
                        clip: motion.clip.clone(),
                        display_name: clip.display_name.clone(),
                        position,
                    });
                }
            }
        }
    }
    layer
        .event_elapsed
        .retain(|clip, _| selected.contains(clip));
    carry_event_clip_selected
}

pub(crate) fn apply_transient_carry_event(
    function_name: &str,
    occurrences: u32,
    visible: &mut bool,
) -> bool {
    match function_name {
        "ToggleOn" => {
            if occurrences > 0 {
                *visible = true;
            }
            true
        }
        "ToggleOff" => {
            if occurrences > 0 {
                *visible = false;
            }
            true
        }
        _ => false,
    }
}

pub(crate) fn clip_event_offset_seconds(
    layer: &ConvertedAnimationLayerDriver,
    clip: &StableId,
    presentation: &PresentationCatalog,
) -> f32 {
    presentation
        .clips
        .get(clip)
        .map_or(0.0, |clip| clip.duration_seconds * layer.state_offset)
}

pub(crate) fn animation_event_occurrences(
    event_time: f32,
    duration: f32,
    previous_elapsed: Option<f32>,
    current_elapsed: f32,
) -> u32 {
    if !event_time.is_finite()
        || !duration.is_finite()
        || !current_elapsed.is_finite()
        || event_time < 0.0
        || duration <= f32::EPSILON
        || event_time > duration
    {
        return 0;
    }
    let previous = previous_elapsed
        .filter(|previous| previous.is_finite() && *previous <= current_elapsed)
        .unwrap_or(-f32::EPSILON);
    // The finite, non-negative cycle indices are intentionally quantized from
    // continuous clip time; saturating float-to-integer casts protect corrupt
    // or extremely long-running clocks without changing event boundaries.
    #[allow(clippy::cast_possible_truncation)]
    let first_cycle = (((previous - event_time) / duration).floor() as i64 + 1).max(0);
    #[allow(clippy::cast_possible_truncation)]
    let last_cycle = ((current_elapsed - event_time) / duration).floor() as i64;
    if last_cycle < first_cycle {
        0
    } else {
        u32::try_from(last_cycle - first_cycle + 1).unwrap_or(u32::MAX)
    }
}

pub(crate) fn role_action_audio_variant<'a>(
    presentation: &'a PresentationCatalog,
    actor: &StableId,
    role: &StableId,
    animation_clip: &StableId,
) -> Option<&'a str> {
    let definition = presentation.role_action_audio.get(role)?;
    let hash = actor
        .as_str()
        .bytes()
        .chain(animation_clip.as_str().bytes())
        .fold(2_166_136_261_u32, |hash, byte| {
            hash.wrapping_mul(16_777_619) ^ u32::from(byte)
        });
    let index = usize::try_from(hash).unwrap_or_default() % definition.clip_guids.len();
    Some(&definition.clip_guids[index])
}

pub(crate) fn procedural_role_action_wav(
    role: &StableId,
    display_name: &str,
    variant_guid: &str,
    sample_rate: u32,
) -> Vec<u8> {
    let role_name = role.as_str();
    let clip_name = display_name.to_ascii_lowercase();
    let variant_hash = variant_guid.bytes().fold(2_166_136_261_u32, |hash, byte| {
        hash.wrapping_mul(16_777_619) ^ u32::from(byte)
    });
    let detune = 2.0_f32.powf((deterministic_unit(variant_hash) - 0.5) / 8.0);
    let (duration, fundamental, brightness, sweep) = if role_name.ends_with("miner") {
        (0.24, 760.0, 1.85, -180.0)
    } else if role_name.ends_with("logger") {
        (0.19, 138.0, 0.72, -48.0)
    } else if role_name.ends_with("builder") {
        (0.22, 310.0, 1.42, -115.0)
    } else if role_name.ends_with("fisher") || clip_name.contains("fish") {
        (0.36, 430.0, 0.64, -210.0)
    } else if role_name.ends_with("ranger") || clip_name.contains("bow") {
        (0.21, 520.0, 1.18, -320.0)
    } else if role_name.ends_with("priest") || role_name.ends_with("paladin") {
        (0.48, 660.0, 1.52, 110.0)
    } else if role_name.ends_with("wizard") || role_name.ends_with("necromancer") {
        (0.52, 245.0, 1.26, 260.0)
    } else if role_name.ends_with("farmer") || role_name.ends_with("gatherer") {
        (0.25, 205.0, 0.52, 85.0)
    } else {
        (0.23, 175.0, 1.06, 240.0)
    };
    synthesize_wav(sample_rate, duration, move |time, _| {
        let progress = (time / duration).clamp(0.0, 1.0);
        let attack = (progress / 0.08).clamp(0.0, 1.0);
        let release = (1.0 - progress).powf(1.7);
        let envelope = attack * release;
        let frequency = (fundamental + sweep * progress) * detune;
        let phase = time * std::f32::consts::TAU * frequency;
        let carrier = phase.sin();
        let overtone = (phase * brightness + 0.4).sin() * 0.34;
        // A low-amplitude deterministic partial adds texture without broadband white noise.
        let textured = (time
            * std::f32::consts::TAU
            * (fundamental * (2.1 + deterministic_unit(variant_hash.rotate_left(9)))))
        .sin()
            * (0.07 + 0.025 * pseudo_noise(variant_hash.rotate_left(21)));
        (carrier * 0.34 + overtone * 0.19 + textured) * envelope
    })
}

pub(crate) fn agent_action_animation(
    content: &ContentCatalog,
    agent: &Agent,
    actor: &ActorState,
) -> Option<String> {
    // Unity remains in its action state and ticks DoAction repeatedly while a
    // target remains valid. Bevy replans after each gameplay tick, leaving a
    // one-frame zero-cooldown gap that used to exit and restart the animation.
    // Base visual state on the stationary action goal so looping clips (most
    // visibly the two-second mining cycle) continue across one-second ticks.
    if agent_is_moving(agent) || !actor.alive {
        return None;
    }
    match agent.goal {
        AgentGoal::Gather(_)
        | AgentGoal::HarvestFarm(_)
        | AgentGoal::Construct(_)
        | AgentGoal::Attack(_)
        | AgentGoal::AttackBuilding(_)
        | AgentGoal::Heal(_)
        | AgentGoal::PlantTree(_)
        | AgentGoal::Prospect { .. }
        | AgentGoal::PlantBush(_) => enemy_animation_contract(content, actor, &agent.id)
            .map_or_else(
                || {
                    content
                        .roles
                        .get(&actor.role)
                        .map(|role| role.action_animation.clone())
                },
                |contract| Some(contract.action_animation),
            ),
        AgentGoal::Deposit
        | AgentGoal::VisitRegenerationStation(_)
        | AgentGoal::WaitForStorage
        | AgentGoal::Wander => None,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EnemyAnimationContract {
    pub(crate) action_animation: String,
    pub(crate) action_animation_variants: u8,
    pub(crate) run_animation: EnemyRunAnimation,
}

pub(crate) fn enemy_animation_contract(
    content: &ContentCatalog,
    actor: &ActorState,
    actor_id: &StableId,
) -> Option<EnemyAnimationContract> {
    let models = actor_archetype(content, actor)?.enemy_models.as_ref()?;
    let selection = enemy_model_selection(actor_id, models);
    if let Some(weapon) = selection.weapon.and_then(|index| models.weapons.get(index)) {
        Some(EnemyAnimationContract {
            action_animation: weapon.action_animation.clone(),
            action_animation_variants: weapon.action_animation_variants.max(1),
            run_animation: weapon.run_animation,
        })
    } else {
        Some(EnemyAnimationContract {
            action_animation: "GenericAction".to_owned(),
            action_animation_variants: models.base_animation_variants.max(1),
            run_animation: EnemyRunAnimation::Generic,
        })
    }
}

pub(crate) fn agent_is_moving(agent: &Agent) -> bool {
    agent
        .path
        .get(agent.path_index)
        .is_some_and(|next| *next != agent.origin)
}

pub(crate) fn action_animation_speed(
    content: &ContentCatalog,
    simulation: &WorldSimulation,
    actor: &ActorState,
) -> f32 {
    let action_seconds = actor_archetype(content, actor)
        .and_then(|archetype| archetype.enemy.as_ref())
        .map_or_else(
            || {
                effective_role_stats(content, simulation, actor)
                    .map_or(1.0, |stats| milli_units_as_f32(stats.action_milliseconds))
            },
            |enemy| milli_units_as_f32(enemy.action_milliseconds),
        );
    (3.0 - action_seconds * 2.0).max(1.0)
}

pub(crate) fn deterministic_animation_variant(actor: &StableId, action: &str, variants: u8) -> u8 {
    if variants <= 1 {
        return 0;
    }
    let hash = actor
        .as_str()
        .bytes()
        .chain(action.bytes())
        .fold(2_166_136_261_u32, |hash, byte| {
            hash.wrapping_mul(16_777_619) ^ u32::from(byte)
        });
    u8::try_from(hash % u32::from(variants)).expect("variant modulo fits u8")
}

pub(crate) fn current_normalized_time(
    player: &AnimationPlayer,
    layer: &ConvertedAnimationLayerDriver,
    presentation: &PresentationCatalog,
) -> f32 {
    player
        .playing_animations()
        .filter_map(|(node, animation)| {
            let clip = layer
                .nodes
                .iter()
                .find_map(|(clip, candidate)| (candidate == node).then_some(clip))?;
            let duration = presentation.clips.get(clip)?.duration_seconds;
            (duration > f32::EPSILON).then_some(animation.elapsed() / duration + layer.state_offset)
        })
        .fold(0.0, f32::max)
}

pub(crate) fn animation_nodes_for_selection(
    selection: &AnimationBlendSelection,
    nodes: &BTreeMap<StableId, AnimationNodeIndex>,
) -> Vec<(AnimationNodeIndex, f32)> {
    let mut desired = Vec::with_capacity(2);
    if let Some(node) = nodes.get(&selection.first.clip)
        && selection.first.weight > f32::EPSILON
    {
        desired.push((*node, selection.first.weight));
    }
    if let Some(second) = &selection.second
        && let Some(node) = nodes.get(&second.clip)
        && second.weight > f32::EPSILON
    {
        desired.push((*node, second.weight));
    }
    desired
}

pub(crate) fn animation_playback_for_selection(
    selection: &AnimationBlendSelection,
    nodes: &BTreeMap<StableId, AnimationNodeIndex>,
    presentation: &PresentationCatalog,
    state_speed: f32,
) -> Vec<ConvertedAnimationPlayback> {
    std::iter::once(&selection.first)
        .chain(selection.second.as_ref())
        .filter_map(|motion| {
            let node = *nodes.get(&motion.clip)?;
            let clip = presentation.clips.get(&motion.clip)?;
            (motion.weight > f32::EPSILON).then_some(ConvertedAnimationPlayback {
                node,
                weight: motion.weight,
                speed: state_speed,
                looping: clip.looping,
            })
        })
        .collect()
}

pub(crate) fn begin_animation_crossfade(
    layer: &mut ConvertedAnimationLayerDriver,
    playback: AnimationTransitionPlayback,
    source_duration: f32,
) {
    let duration = if playback.fixed_duration {
        playback.duration
    } else {
        playback.duration * source_duration
    };
    layer.state_offset = playback.destination_offset;
    layer.crossfade = (duration > f32::EPSILON && !layer.applied.is_empty()).then(|| {
        ConvertedAnimationCrossfade {
            source: layer.applied.clone(),
            elapsed: 0.0,
            duration,
        }
    });
}

pub(crate) fn animation_selection_duration(
    selection: &AnimationBlendSelection,
    presentation: &PresentationCatalog,
) -> f32 {
    let (weighted_duration, total_weight) = std::iter::once(&selection.first)
        .chain(selection.second.as_ref())
        .filter_map(|motion| {
            presentation
                .clips
                .get(&motion.clip)
                .map(|clip| (clip.duration_seconds * motion.weight, motion.weight))
        })
        .fold((0.0, 0.0), |(duration, weight), motion| {
            (duration + motion.0, weight + motion.1)
        });
    if total_weight > f32::EPSILON {
        weighted_duration / total_weight
    } else {
        0.0
    }
}

pub(crate) fn advance_animation_crossfade(
    crossfade: &mut Option<ConvertedAnimationCrossfade>,
    destination: &[ConvertedAnimationPlayback],
    delta_seconds: f32,
) -> Vec<ConvertedAnimationPlayback> {
    let Some(active) = crossfade.as_mut() else {
        return destination.to_vec();
    };
    active.elapsed = (active.elapsed + delta_seconds.max(0.0)).min(active.duration);
    let progress = (active.elapsed / active.duration).clamp(0.0, 1.0);
    let mut output = Vec::with_capacity(active.source.len() + destination.len());
    for playback in &active.source {
        merge_animation_weight(
            &mut output,
            ConvertedAnimationPlayback {
                weight: playback.weight * (1.0 - progress),
                ..*playback
            },
        );
    }
    for playback in destination {
        merge_animation_weight(
            &mut output,
            ConvertedAnimationPlayback {
                weight: playback.weight * progress,
                ..*playback
            },
        );
    }
    if active.elapsed >= active.duration {
        *crossfade = None;
    }
    output
}

pub(crate) fn merge_animation_weight(
    output: &mut Vec<ConvertedAnimationPlayback>,
    playback: ConvertedAnimationPlayback,
) {
    if playback.weight <= f32::EPSILON {
        return;
    }
    if let Some(existing) = output
        .iter_mut()
        .find(|candidate| candidate.node == playback.node)
    {
        let total = existing.weight + playback.weight;
        existing.speed =
            (existing.speed * existing.weight + playback.speed * playback.weight) / total;
        existing.weight = total;
        existing.looping |= playback.looping;
    } else {
        output.push(playback);
    }
}

pub(crate) fn apply_animation_blend(
    player: &mut AnimationPlayer,
    desired: &[ConvertedAnimationPlayback],
    restarts: &[(AnimationNodeIndex, f32)],
) {
    let playing: Vec<_> = player.playing_animations().map(|(node, _)| *node).collect();
    for node in playing {
        if !desired.iter().any(|desired| desired.node == node) {
            player.stop(node);
        }
    }
    for playback in desired {
        let animation = if let Some((_, offset)) = restarts
            .iter()
            .find(|(restart, _)| *restart == playback.node)
        {
            player.start(playback.node).set_seek_time(*offset)
        } else {
            player.play(playback.node)
        };
        animation
            .set_repeat(if playback.looping {
                RepeatAnimation::Forever
            } else {
                RepeatAnimation::Never
            })
            .set_weight(playback.weight)
            .set_speed(playback.speed);
    }
}
