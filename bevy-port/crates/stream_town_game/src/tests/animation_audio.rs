#[test]
fn animation_events_fire_once_per_elapsed_clip_cycle() {
    assert_eq!(animation_event_occurrences(0.25, 1.0, None, 0.24), 0);
    assert_eq!(animation_event_occurrences(0.25, 1.0, None, 0.25), 1);
    assert_eq!(animation_event_occurrences(0.25, 1.0, Some(0.25), 1.24), 0);
    assert_eq!(animation_event_occurrences(0.25, 1.0, Some(1.24), 1.25), 1);
    assert_eq!(animation_event_occurrences(0.25, 1.0, Some(0.10), 3.10), 3);
    assert_eq!(animation_event_occurrences(0.0, 1.0, None, 0.0), 1);
    assert_eq!(animation_event_occurrences(0.25, 0.0, None, 1.0), 0);
}

#[test]
fn fishing_carry_events_toggle_only_inside_the_authored_catch_window() {
    let mut visible = false;
    assert!(apply_transient_carry_event("ToggleOn", 0, &mut visible));
    assert!(!visible);
    assert!(apply_transient_carry_event("ToggleOn", 1, &mut visible));
    assert!(visible);
    assert!(apply_transient_carry_event("ToggleOff", 1, &mut visible));
    assert!(!visible);
    assert!(!apply_transient_carry_event(
        "PlayRoleActionAudio",
        1,
        &mut visible
    ));
}

#[test]
fn locomotion_velocity_grace_prevents_one_frame_run_restart() {
    let delta_seconds = 1.0 / 60.0;
    let cell_size = 2.0;
    let frame_step = 4.0 * cell_size * delta_seconds;
    let mut locomotion = AgentLocomotion::default();
    update_agent_locomotion_sample(
        &mut locomotion,
        Vec3::ZERO,
        delta_seconds,
        cell_size,
        0.8,
        true,
    );
    update_agent_locomotion_sample(
        &mut locomotion,
        Vec3::X * frame_step,
        delta_seconds,
        cell_size,
        0.8,
        false,
    );
    assert!((locomotion.normalized_speed - 0.8).abs() < 1.0e-5);

    // A route handoff can produce one stationary simulation frame. It must
    // not stop the active run node and restart it at time zero.
    update_agent_locomotion_sample(
        &mut locomotion,
        Vec3::X * frame_step,
        delta_seconds,
        cell_size,
        0.8,
        false,
    );
    assert!((locomotion.normalized_speed - 0.8).abs() < 1.0e-5);
    update_agent_locomotion_sample(
        &mut locomotion,
        Vec3::X * frame_step * 2.0,
        delta_seconds,
        cell_size,
        0.8,
        false,
    );
    assert!((locomotion.normalized_speed - 0.8).abs() < 1.0e-5);

    for _ in 0..10 {
        update_agent_locomotion_sample(
            &mut locomotion,
            Vec3::X * frame_step * 2.0,
            delta_seconds,
            cell_size,
            0.8,
            false,
        );
    }
    assert!(locomotion.normalized_speed.abs() < f32::EPSILON);
}

#[test]
fn converted_role_audio_events_use_source_guided_spatial_procedural_cues() {
    let presentation = embedded_presentation();
    let clips: Vec<_> = presentation
        .clips
        .iter()
        .filter(|(_, clip)| {
            clip.events
                .iter()
                .any(|event| event.function_name == "PlayRoleActionAudio")
        })
        .collect();
    assert_eq!(clips.len(), 10);
    assert_eq!(presentation.role_action_audio.len(), 14);
    assert_eq!(
        presentation
            .role_action_audio
            .values()
            .map(|audio| audio.clip_guids.len())
            .sum::<usize>(),
        35
    );
    let actor = StableId::new("actor:audio-test").unwrap();
    let role = StableId::new("role:miner").unwrap();
    let (clip_id, clip) = clips[0];
    let guid = role_action_audio_variant(&presentation, &actor, &role, clip_id).unwrap();
    assert_eq!(
        guid,
        role_action_audio_variant(&presentation, &actor, &role, clip_id).unwrap()
    );
    let wav = procedural_role_action_wav(
        &role,
        &clip.display_name,
        guid,
        PROCEDURAL_AUDIO_SAMPLE_RATE,
    );
    assert_eq!(&wav[0..4], b"RIFF");
    assert_eq!(&wav[8..12], b"WAVE");
    assert_eq!(
        wav,
        procedural_role_action_wav(
            &role,
            &clip.display_name,
            guid,
            PROCEDURAL_AUDIO_SAMPLE_RATE,
        )
    );
    let samples: Vec<_> = wav[44..]
        .chunks_exact(2)
        .map(|bytes| i16::from_le_bytes([bytes[0], bytes[1]]))
        .collect();
    assert!(samples.iter().any(|sample| sample.abs() > 512));
    assert!(i32::from(*samples.last().unwrap()).abs() < 256);
    assert!(
        samples
            .windows(2)
            .map(|pair| (i32::from(pair[1]) - i32::from(pair[0])).abs())
            .max()
            .unwrap_or_default()
            < 12_000,
        "role cue contains a static-like discontinuity"
    );
}

#[test]
fn fish_school_uses_authored_spawn_volume_noise_and_velocity_alignment() {
    let presentation = embedded_presentation();
    let effect = presentation.fish_school_effects.values().next().unwrap();
    assert!(
        effect
            .shape_scale
            .into_iter()
            .zip([300.0, 300.0, 5.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert_eq!(
        presentation
            .scene_fish_schools
            .values()
            .map(Vec::len)
            .sum::<usize>(),
        2
    );
    assert!(effect.start_speed.abs() < f32::EPSILON);
    assert!(
        effect
            .shape_rotation_degrees
            .into_iter()
            .zip([-90.0, 0.0, 0.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
    assert!((effect.noise_scroll_speed - 1.0).abs() < f32::EPSILON);
    assert!((effect.noise_position_amount - 2.0).abs() < f32::EPSILON);
    assert_eq!(effect.noise_octaves, 2);
    assert!(effect.align_to_velocity);
    assert_eq!(
        presentation.scene_fish_schools[WORLD_SCENE_PATH]
            .iter()
            .map(|binding| binding.hierarchy_path.as_str())
            .collect::<Vec<_>>(),
        ["Fish"]
    );
    let particle = FishSchoolParticle {
        base_position: Vec3::new(2.0, -2.0, 3.0),
        noise_amplitude: Vec3::new(20.0, 0.04, 20.0),
        phase: Vec3::new(0.4, 1.7, 2.8),
        frequency: 0.22,
        scroll_speed: 1.0,
        octaves: 2,
        octave_multiplier: 1.0,
        octave_scale: 2.0,
        align_to_velocity: true,
    };
    for elapsed in [0.0, 1.0, 30.0, 120.0] {
        let transform = fish_school_transform(&particle, elapsed);
        assert_eq!(
            transform,
            fish_school_transform(&particle, elapsed),
            "fish transform must be deterministic"
        );
        let offset = transform.translation - particle.base_position;
        assert!(offset.x.abs() <= particle.noise_amplitude.x);
        assert!(offset.y.abs() <= particle.noise_amplitude.y);
        assert!(offset.z.abs() <= particle.noise_amplitude.z);
        let next = fish_school_transform(&particle, elapsed + 0.001);
        let movement = (next.translation - transform.translation).normalize();
        // Primitive extraction bypasses the imported node; its raw +Y nose
        // must still face the instantaneous velocity after correction.
        let model_forward = transform.rotation * Vec3::Y;
        assert!(model_forward.dot(movement) > 0.999);
    }
    let one_second = fish_school_transform(&particle, 1.0).translation
        - fish_school_transform(&particle, 0.0).translation;
    assert!(
        one_second.length() < 15.0,
        "fish movement must remain smooth"
    );

    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world_with_content(&config.world, &content);
    let water_height = f32::from(config.world.water_level_centimetres) * 0.01;
    let land = (0..config.world.height)
        .flat_map(|z| (0..config.world.width).map(move |x| GridPos { x, z }))
        .find(|position| terrain_height(&world, *position) > water_height + 0.05)
        .expect("generated world contains land");
    assert!(fish_school_intersects_generated_land(
        grid_to_world(land, &config),
        &world,
        &config,
        water_height,
    ));
}

#[test]
fn procedural_ambience_wav_is_valid_deterministic_and_seamless() {
    let ambience = procedural_ambience_wav(8_000, 1.0);
    assert_eq!(&ambience[0..4], b"RIFF");
    assert_eq!(&ambience[8..12], b"WAVE");
    assert_eq!(ambience.len(), 44 + 8_000 * 2);
    assert_eq!(ambience, procedural_ambience_wav(8_000, 1.0));
    let ambience_samples: Vec<_> = ambience[44..]
        .chunks_exact(2)
        .map(|bytes| i16::from_le_bytes([bytes[0], bytes[1]]))
        .collect();
    let peak = ambience_samples
        .iter()
        .map(|sample| i32::from(*sample).abs())
        .max()
        .unwrap_or_default();
    let maximum_step = ambience_samples
        .windows(2)
        .map(|samples| (i32::from(samples[1]) - i32::from(samples[0])).abs())
        .max()
        .unwrap_or_default();
    let loop_step = (i32::from(ambience_samples[0])
        - i32::from(*ambience_samples.last().expect("ambience has samples")))
    .abs();
    assert!(peak > 512, "ambience must remain audible");
    assert!(maximum_step < 128, "ambience contains static-like jumps");
    assert!(loop_step < 128, "ambience loop contains an audible seam");
}

#[test]
fn seagull_flight_preserves_shipping_prefab_bounds_and_is_deterministic() {
    for serial in 0..64 {
        let (start, end) = deterministic_seagull_leg(0x5eed_2026, serial);
        assert_eq!(start.y.to_bits(), SEAGULL_HEIGHT.to_bits());
        assert_eq!(end.y.to_bits(), SEAGULL_HEIGHT.to_bits());
        assert_eq!(start.x.to_bits(), (-end.x).to_bits());
        assert_eq!(start.z.to_bits(), (-end.z).to_bits());
        assert!(start.x.abs() <= 202.5);
        assert!(start.z.abs() <= 202.5);
        assert_eq!((start, end), deterministic_seagull_leg(0x5eed_2026, serial));
    }

    let position = Vec3::new(-12.0, SEAGULL_HEIGHT, 8.0);
    let target = Vec3::new(15.0, SEAGULL_HEIGHT, -21.0);
    let flight = seagull_flight_transform(position, target);
    let model = seagull_model_transform();
    let visible_forward = flight.rotation * model.rotation * Vec3::X;
    assert!(
        visible_forward.dot((target - position).normalize()) > 0.999,
        "the visible bird, not merely its parent entity, must face its flight direction"
    );
    assert!((model.rotation * Vec3::X).dot(Vec3::Z) > 0.999);
}

#[test]
fn seagull_calls_preserve_unity_cadence_variants_and_rolloff() {
    let mut variants = BTreeSet::new();
    for serial in 0..64 {
        let wait = deterministic_seagull_call_wait(17, serial);
        assert!((1.0..=5.0).contains(&wait));
        assert_eq!(
            wait.to_bits(),
            deterministic_seagull_call_wait(17, serial).to_bits()
        );
        variants.insert(deterministic_seagull_call_variant(17, serial));
    }
    assert_eq!(variants, BTreeSet::from([0, 1, 2]));
    assert_eq!(unity_seagull_rolloff(0.0).to_bits(), 1.0_f32.to_bits());
    assert_eq!(unity_seagull_rolloff(5.0).to_bits(), 0.5_f32.to_bits());
    assert_eq!(unity_seagull_rolloff(50.0).to_bits(), 0.0_f32.to_bits());
    assert_eq!(unity_seagull_rolloff(500.0).to_bits(), 0.0_f32.to_bits());
}

#[test]
fn procedural_seagull_calls_are_valid_distinct_wav_sources() {
    let calls: Vec<_> = (0..3)
        .map(|variant| procedural_seagull_call_wav(variant, 8_000))
        .collect();
    assert!(calls.iter().all(|call| &call[0..4] == b"RIFF"));
    assert!(calls.iter().all(|call| &call[8..12] == b"WAVE"));
    assert_ne!(calls[0], calls[1]);
    assert_ne!(calls[1], calls[2]);
    assert_eq!(calls[0], procedural_seagull_call_wav(0, 8_000));
}

#[test]
fn procedural_audio_matches_curated_acceptance_baseline() {
    let actual = audio_acceptance_manifest();
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/acceptance/audio-baseline.json");
    let expected: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(actual, expected);
    let clips = audio_acceptance_wavs();
    assert_eq!(clips.len(), 39);
    for (name, wav) in clips {
        let record = audio_acceptance_record(&wav);
        assert!(
            record["peak"].as_i64().unwrap() > 512,
            "{name} is inaudible"
        );
        let maximum_step = record["maximum_step"].as_i64().unwrap();
        let limit = if name == "ambience" { 128 } else { 12_000 };
        assert!(
            maximum_step < limit,
            "{name} contains a static-like discontinuity"
        );
    }
}

#[test]
fn shoreline_acceptance_camera_frames_water_and_land_from_the_ocean() {
    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world_with_content(&config.world, &content);
    let focus = shoreline_focus(&world, &config);
    let centre = GridPos {
        x: world.navigation.width() / 2,
        z: world.navigation.height() / 2,
    };
    let centre_world = grid_to_world_on_surface(centre, &config, &world);
    let inward = Vec3::new(centre_world.x - focus.x, 0.0, centre_world.z - focus.z).normalize();
    let boundary_world = focus - inward * config.world.cell_size * 2.5;
    let boundary = world_to_grid(boundary_world, &config).expect("shoreline is on the grid");
    let water_height = f32::from(config.world.water_level_centimetres) * 0.01;
    assert!(terrain_height(&world, boundary) <= water_height);
    assert!(
        [
            boundary
                .x
                .checked_sub(1)
                .map(|x| GridPos { x, z: boundary.z }),
            (boundary.x + 1 < world.navigation.width()).then_some(GridPos {
                x: boundary.x + 1,
                z: boundary.z,
            }),
            boundary
                .z
                .checked_sub(1)
                .map(|z| GridPos { x: boundary.x, z }),
            (boundary.z + 1 < world.navigation.height()).then_some(GridPos {
                x: boundary.x,
                z: boundary.z + 1,
            }),
        ]
        .into_iter()
        .flatten()
        .any(|neighbor| terrain_height(&world, neighbor) > water_height),
        "acceptance focus must be derived from the generated water/land boundary"
    );

    let camera = shoreline_camera_transform(&world, &config);
    let horizontal_camera_offset = Vec3::new(
        camera.translation.x - focus.x,
        0.0,
        camera.translation.z - focus.z,
    );
    assert!(horizontal_camera_offset.dot(inward) < -20.0);
    let to_focus = (focus + Vec3::Y - camera.translation).normalize();
    assert!(camera.forward().dot(to_focus) > 0.999);
}

#[test]
fn converted_state_crossfade_preserves_weights_and_finishes_at_destination() {
    let source = AnimationNodeIndex::new(1);
    let destination = AnimationNodeIndex::new(2);
    let mut crossfade = Some(ConvertedAnimationCrossfade {
        source: vec![ConvertedAnimationPlayback {
            node: source,
            weight: 1.0,
            speed: 1.0,
            looping: true,
        }],
        elapsed: 0.0,
        duration: 0.25,
    });
    let desired = vec![ConvertedAnimationPlayback {
        node: destination,
        weight: 1.0,
        speed: 2.0,
        looping: false,
    }];
    let half = advance_animation_crossfade(&mut crossfade, &desired, 0.125);
    assert_eq!(half.len(), 2);
    assert!((half[0].weight - 0.5).abs() < f32::EPSILON);
    assert!((half[1].weight - 0.5).abs() < f32::EPSILON);
    let finished = advance_animation_crossfade(&mut crossfade, &desired, 0.125);
    assert_eq!(finished, desired);
    assert!(crossfade.is_none());
}

#[test]
fn converted_playback_preserves_authored_loop_modes() {
    let looping = AnimationNodeIndex::new(1);
    let one_shot = AnimationNodeIndex::new(2);
    let mut player = AnimationPlayer::default();
    apply_animation_blend(
        &mut player,
        &[
            ConvertedAnimationPlayback {
                node: looping,
                weight: 0.75,
                speed: 1.0,
                looping: true,
            },
            ConvertedAnimationPlayback {
                node: one_shot,
                weight: 0.25,
                speed: 1.0,
                looping: false,
            },
        ],
        &[],
    );

    assert_eq!(
        player.animation(looping).unwrap().repeat_mode(),
        RepeatAnimation::Forever
    );
    assert_eq!(
        player.animation(one_shot).unwrap().repeat_mode(),
        RepeatAnimation::Never
    );
}

#[test]
fn in_game_sun_lights_camera_facing_surfaces_with_authored_probe_compensation() {
    let sun = in_game_sun_transform();
    let camera = default_town_camera_transform();
    assert!(sun.translation.x < 0.0);
    assert!(sun.forward().dot(*camera.forward()) > 0.7);
    assert!((in_game_ambient_brightness(90.0) - 117.0).abs() < 0.001);
}

#[test]
fn converted_crossfade_uses_fixed_or_normalized_authored_duration() {
    let source = AnimationNodeIndex::new(1);
    let presentation = embedded_presentation();
    let controller = presentation.controllers.values().next().unwrap();
    let mut layer = ConvertedAnimationLayerDriver {
        fallback_state: StableId::new("state:fallback").unwrap(),
        runtime: AnimationControllerRuntime::in_state(
            controller,
            controller.default_states[0].clone(),
        )
        .unwrap(),
        nodes: BTreeMap::new(),
        active: Vec::new(),
        applied: vec![ConvertedAnimationPlayback {
            node: source,
            weight: 1.0,
            speed: 1.0,
            looping: true,
        }],
        crossfade: None,
        state_offset: 0.0,
        event_elapsed: BTreeMap::new(),
    };
    begin_animation_crossfade(
        &mut layer,
        AnimationTransitionPlayback {
            duration: 0.25,
            fixed_duration: false,
            destination_offset: 0.2,
        },
        2.0,
    );
    assert!((layer.crossfade.as_ref().unwrap().duration - 0.5).abs() < f32::EPSILON);
    assert!((layer.state_offset - 0.2).abs() < f32::EPSILON);
    begin_animation_crossfade(
        &mut layer,
        AnimationTransitionPlayback {
            duration: 0.25,
            fixed_duration: true,
            destination_offset: 0.0,
        },
        2.0,
    );
    assert!((layer.crossfade.as_ref().unwrap().duration - 0.25).abs() < f32::EPSILON);
}

#[test]
fn cosmetic_nodes_preserve_unity_order_and_visibility_rules() {
    for (index, name) in EYE_NODES.iter().enumerate() {
        assert_eq!(
            cosmetic_node(name),
            Some((CosmeticNodeKind::Eyes, u8::try_from(index).unwrap()))
        );
    }
    for (index, name) in HAIR_NODES.iter().enumerate() {
        assert_eq!(
            cosmetic_node(name),
            Some((CosmeticNodeKind::Hair, u8::try_from(index).unwrap()))
        );
    }
    for (index, name) in FACIAL_HAIR_NODES.iter().enumerate() {
        assert_eq!(
            cosmetic_node(name),
            Some((CosmeticNodeKind::FacialHair, u8::try_from(index).unwrap()))
        );
    }
    assert_eq!(cosmetic_node("Helmet_Defender"), None);

    let customization = ActorCustomization {
        eyes: 3,
        hair: 6,
        facial_hair: 1,
        hair_color: 4,
        eye_color: 1,
        skin_color: 3,
        body_type: 2,
        ..ActorCustomization::default()
    };
    assert!(cosmetic_node_visible(
        customization,
        CosmeticNodeKind::Eyes,
        3,
        false
    ));
    assert!(!cosmetic_node_visible(
        customization,
        CosmeticNodeKind::Eyes,
        2,
        false
    ));
    assert!(cosmetic_node_visible(
        customization,
        CosmeticNodeKind::Hair,
        6,
        false
    ));
    assert!(!cosmetic_node_visible(
        customization,
        CosmeticNodeKind::Hair,
        6,
        true
    ));
    assert!(cosmetic_node_visible(
        customization,
        CosmeticNodeKind::FacialHair,
        1,
        true
    ));
    assert_eq!(
        cosmetic_color(customization, CosmeticNodeKind::Hair),
        (4, HAIR_COLORS[4])
    );
    assert_eq!(
        cosmetic_color(customization, CosmeticNodeKind::FacialHair),
        (4, HAIR_COLORS[4])
    );
    assert_eq!(
        cosmetic_color(customization, CosmeticNodeKind::Eyes),
        (1, EYE_COLORS[1])
    );
    assert_eq!(
        cosmetic_color(customization, CosmeticNodeKind::Skin),
        (3, SKIN_COLORS[3])
    );
}
