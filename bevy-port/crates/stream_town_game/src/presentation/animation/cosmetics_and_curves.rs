pub(crate) fn tag_cosmetic_renderers(
    mut commands: Commands,
    parents: Query<&ChildOf>,
    names: Query<&Name>,
    material_names: Query<&GltfMaterialName>,
    agents: Query<Entity, With<Agent>>,
    animated_player_rigs: Query<Entity, With<PlayerAnimatedRig>>,
    standard_renderers: Query<
        (Entity, &MeshMaterial3d<StandardMaterial>),
        Added<MeshMaterial3d<StandardMaterial>>,
    >,
    character_renderers: Query<
        (Entity, &MeshMaterial3d<CharacterMaterial>),
        Added<MeshMaterial3d<CharacterMaterial>>,
    >,
    standard_materials: Option<Res<Assets<StandardMaterial>>>,
    character_materials: Option<ResMut<Assets<CharacterMaterial>>>,
    mut base_cache: ResMut<CharacterBaseMaterialCache>,
) {
    for (entity, material) in &character_renderers {
        let mut entity_commands = commands.entity(entity);
        if animated_player_renderer(entity, &parents, &animated_player_rigs) {
            // Skinned bounds describe the bind pose, not every animated pose.
            // More importantly, Bevy 0.19 frees and reallocates the skin's
            // joint-buffer slot whenever aggregate ViewVisibility changes.
            // The automatic camera repeatedly crosses those static bounds;
            // keeping only animated character renderers out of frustum culling
            // makes their skin indices stable in both the main and shadow pass.
            entity_commands.insert((AnimatedCharacterShadowReceiver, NoFrustumCulling));
        }
        if let Some((actor_root, kind)) =
            cosmetic_renderer_context(entity, &parents, &names, &material_names, &agents)
        {
            entity_commands.insert(CosmeticRenderer {
                actor_root,
                kind,
                base_material: material.0.clone(),
                applied_color: None,
            });
        }
    }

    let (Some(standard_materials), Some(mut character_materials)) =
        (standard_materials, character_materials)
    else {
        return;
    };
    for (entity, material) in &standard_renderers {
        let cosmetic_context =
            cosmetic_renderer_context(entity, &parents, &names, &material_names, &agents);
        let animated_player = animated_player_renderer(entity, &parents, &animated_player_rigs);
        if !animated_player && cosmetic_context.is_none() {
            continue;
        }
        let character_material = if let Some(variant) = base_cache
            .0
            .iter()
            .find(|variant| variant.source == material.0)
        {
            variant.material.clone()
        } else {
            let Some(source) = standard_materials.get(&material.0).cloned() else {
                continue;
            };
            let character = character_materials.add(character_material_from_standard(source));
            base_cache.0.push(CharacterBaseMaterialVariant {
                source: material.0.clone(),
                material: character.clone(),
            });
            character
        };
        let mut entity_commands = commands.entity(entity);
        entity_commands
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .insert(MeshMaterial3d(character_material.clone()));
        if animated_player {
            entity_commands.insert((AnimatedCharacterShadowReceiver, NoFrustumCulling));
        }
        if let Some((actor_root, kind)) = cosmetic_context {
            entity_commands.insert(CosmeticRenderer {
                actor_root,
                kind,
                base_material: character_material,
                applied_color: None,
            });
        }
    }
}

pub(crate) fn animated_player_renderer(
    entity: Entity,
    parents: &Query<&ChildOf>,
    animated_player_rigs: &Query<Entity, With<PlayerAnimatedRig>>,
) -> bool {
    let mut ancestor = entity;
    for _ in 0..64 {
        if animated_player_rigs.contains(ancestor) {
            return true;
        }
        let Ok(parent) = parents.get(ancestor) else {
            break;
        };
        ancestor = parent.parent();
    }
    false
}

pub(crate) fn cosmetic_renderer_context(
    entity: Entity,
    parents: &Query<&ChildOf>,
    names: &Query<&Name>,
    material_names: &Query<&GltfMaterialName>,
    agents: &Query<Entity, With<Agent>>,
) -> Option<(Entity, CosmeticNodeKind)> {
    let mut ancestor = entity;
    let mut cosmetic_kind = None;
    for _ in 0..64 {
        cosmetic_kind = cosmetic_kind.or_else(|| {
            names
                .get(ancestor)
                .ok()
                .and_then(|name| cosmetic_node(name.as_str()))
                .map(|(kind, _)| kind)
        });
        if cosmetic_kind.is_none()
            && material_names
                .get(ancestor)
                .is_ok_and(|name| name.0 == "SkinMaterial")
        {
            cosmetic_kind = Some(CosmeticNodeKind::Skin);
        }
        if let (true, Some(kind)) = (agents.contains(ancestor), cosmetic_kind) {
            return Some((ancestor, kind));
        }
        let Ok(parent) = parents.get(ancestor) else {
            break;
        };
        ancestor = parent.parent();
    }
    None
}

pub(crate) fn cosmetic_color(
    customization: ActorCustomization,
    kind: CosmeticNodeKind,
) -> (u8, [f32; 3]) {
    match kind {
        CosmeticNodeKind::Eyes => {
            let index = usize::from(customization.eye_color).min(EYE_COLORS.len() - 1);
            (
                u8::try_from(index).expect("eye color index fits"),
                EYE_COLORS[index],
            )
        }
        CosmeticNodeKind::Hair | CosmeticNodeKind::FacialHair => {
            let index = usize::from(customization.hair_color).min(HAIR_COLORS.len() - 1);
            (
                u8::try_from(index).expect("hair color index fits"),
                HAIR_COLORS[index],
            )
        }
        CosmeticNodeKind::Skin => {
            let index = usize::from(customization.skin_color).min(SKIN_COLORS.len() - 1);
            (
                u8::try_from(index).expect("skin color index fits"),
                SKIN_COLORS[index],
            )
        }
    }
}

pub(crate) fn sync_cosmetic_materials(
    simulation: Res<SimulationRuntime>,
    agents: Query<&Agent>,
    materials: Option<ResMut<Assets<CharacterMaterial>>>,
    mut cache: ResMut<CosmeticMaterialCache>,
    mut renderers: Query<(
        &mut CosmeticRenderer,
        &mut MeshMaterial3d<CharacterMaterial>,
    )>,
) {
    let Some(mut materials) = materials else {
        return;
    };
    for (mut cosmetic, mut renderer_material) in &mut renderers {
        let Ok(agent) = agents.get(cosmetic.actor_root) else {
            continue;
        };
        let Some(actor) = simulation.0.actors.get(&agent.id) else {
            continue;
        };
        let (color_index, color) = cosmetic_color(actor.customization, cosmetic.kind);
        if cosmetic.applied_color == Some(color_index) {
            continue;
        }
        let material = if let Some(variant) = cache.0.iter().find(|variant| {
            variant.base_material == cosmetic.base_material
                && variant.kind == cosmetic.kind
                && variant.color == color_index
        }) {
            variant.material.clone()
        } else {
            let Some(mut material) = materials.get(&cosmetic.base_material).cloned() else {
                continue;
            };
            material.extension.parameters.albedo_color = Color::srgb(color[0], color[1], color[2])
                .to_linear()
                .to_f32_array()
                .into();
            let handle = materials.add(material);
            cache.0.push(CosmeticMaterialVariant {
                base_material: cosmetic.base_material.clone(),
                kind: cosmetic.kind,
                color: color_index,
                material: handle.clone(),
            });
            handle
        };
        renderer_material.0 = material;
        cosmetic.applied_color = Some(color_index);
    }
}

pub(crate) fn collect_animation_targets(
    root: Entity,
    children: &Query<&Children>,
    names: &Query<&Name>,
    transforms: &Query<&Transform>,
) -> BTreeMap<String, (Entity, Transform)> {
    let Some(root_name) = names.get(root).ok().map(Name::as_str) else {
        return BTreeMap::new();
    };
    collect_animation_targets_with_root_name(root, root_name, children, names, transforms)
}

pub(crate) fn collect_animation_targets_with_root_name(
    root: Entity,
    animation_root_name: &str,
    children: &Query<&Children>,
    names: &Query<&Name>,
    transforms: &Query<&Transform>,
) -> BTreeMap<String, (Entity, Transform)> {
    let mut targets = BTreeMap::new();
    let mut pending = vec![(root, animation_root_name.to_owned())];
    while let Some((entity, path)) = pending.pop() {
        if let Ok(transform) = transforms.get(entity) {
            targets.insert(path.clone(), (entity, *transform));
        }
        if let Ok(entity_children) = children.get(entity) {
            for child in entity_children.iter().rev() {
                if let Ok(name) = names.get(child) {
                    pending.push((child, format!("{path}/{}", name.as_str())));
                }
            }
        }
    }
    targets
}

pub(crate) fn retargeted_animation_clip(
    source: &AnimationClipDef,
    targets: &BTreeMap<String, (Entity, Transform)>,
) -> Option<AnimationClip> {
    let mut clip = AnimationClip::default();
    let skeletal_character = source
        .rig_asset_path
        .as_deref()
        .is_some_and(|path| path.contains("/Characters/") || path.contains("/Enemies/"));
    for track in &source.transform_tracks {
        let Some((target_path, (_, rest))) =
            animation_target_for_track(targets, &track.target_path)
        else {
            continue;
        };
        let target = target_path.split('/').collect::<AnimationTargetId>();
        // Unity's imported character clips contain baked position curves on
        // several spine bones. Scaling those deltas to a glTF rest pose pulls
        // the independently skinned body pieces apart. Character locomotion is
        // rotation-driven; preserve only the root-bone translation. Mechanical
        // clips (gates, buildings, props) retain their full transform curves.
        if !skeletal_character || !track.target_path.contains('/') {
            add_translation_curve(
                &mut clip,
                target,
                track,
                rest,
                source.looping,
                source.duration_seconds,
            );
        }
        add_rotation_curve(
            &mut clip,
            target,
            track,
            rest,
            source.looping,
            source.duration_seconds,
        );
        if !skeletal_character {
            add_scale_curve(
                &mut clip,
                target,
                track,
                rest,
                source.looping,
                source.duration_seconds,
            );
        }
    }
    if clip.curves().is_empty() {
        return None;
    }
    clip.set_duration(clip.duration().max(source.duration_seconds));
    Some(clip)
}

pub(crate) fn animation_target_for_track<'a>(
    targets: &'a BTreeMap<String, (Entity, Transform)>,
    track_path: &str,
) -> Option<(&'a str, &'a (Entity, Transform))> {
    targets
        .get_key_value(track_path)
        .map(|(path, target)| (path.as_str(), target))
        .or_else(|| {
            let suffix = format!("/{track_path}");
            targets
                .iter()
                .find(|(path, _)| path.ends_with(&suffix))
                .map(|(path, target)| (path.as_str(), target))
        })
        .or_else(|| {
            // The source Character controller calls its FBX root `pelvis`,
            // while the consolidated runtime GLB calls that root
            // `CharacterArmature`. Match the unchanged bone suffix after the
            // differing root instead of dropping every locomotion curve.
            let (_, bone_path) = track_path.split_once('/')?;
            let suffix = format!("/{bone_path}");
            targets
                .iter()
                .find(|(path, _)| path.ends_with(&suffix))
                .map(|(path, target)| (path.as_str(), target))
        })
        .or_else(|| {
            if track_path.contains('/') {
                return None;
            }
            targets
                .iter()
                .find(|(path, _)| !path.contains('/'))
                .map(|(path, target)| (path.as_str(), target))
        })
}

pub(crate) fn add_translation_curve(
    clip: &mut AnimationClip,
    target: AnimationTargetId,
    track: &AnimationTransformTrack,
    rest: &Transform,
    looping: bool,
    loop_duration: f32,
) {
    if track.translation.is_empty() {
        return;
    }
    let unity_reference = track.reference_translation.map_or_else(
        || Vec3::from_array(track.translation[0].value),
        Vec3::from_array,
    );
    let unity_rotation = track
        .reference_rotation
        .map_or(Quat::IDENTITY, normalized_quat);
    let basis = rest.rotation * unity_rotation.inverse();
    let unit_scale = if unity_reference.length_squared() > 1.0e-8 {
        rest.translation.length() / unity_reference.length()
    } else {
        1.0
    };
    let mut keys = ensure_two_keyframes(
        track.translation.iter().map(|key| {
            let delta = Vec3::from_array(key.value) - unity_reference;
            (key.time, rest.translation + basis * delta * unit_scale)
        }),
        clip.duration(),
    );
    if looping {
        close_translation_loop(&mut keys, loop_duration);
    }
    if let Ok(curve) = AnimatableKeyframeCurve::new(keys) {
        clip.add_curve_to_target(
            target,
            AnimatableCurve::new(animated_field!(Transform::translation), curve),
        );
    }
}

pub(crate) fn add_rotation_curve(
    clip: &mut AnimationClip,
    target: AnimationTargetId,
    track: &AnimationTransformTrack,
    rest: &Transform,
    looping: bool,
    loop_duration: f32,
) {
    let mut keys = if track.rotation.is_empty() {
        let Some(first) = track.euler_degrees.first() else {
            return;
        };
        let reference = unity_euler(first.value);
        ensure_two_keyframes(
            track.euler_degrees.iter().map(|key| {
                let delta = reference.inverse() * unity_euler(key.value);
                (key.time, (rest.rotation * delta).normalize())
            }),
            clip.duration(),
        )
    } else {
        let reference = track
            .reference_rotation
            .map_or_else(|| normalized_quat(track.rotation[0].value), normalized_quat);
        ensure_two_keyframes(
            track.rotation.iter().map(|key| {
                let delta = reference.inverse() * normalized_quat(key.value);
                (key.time, (rest.rotation * delta).normalize())
            }),
            clip.duration(),
        )
    };
    if looping {
        close_rotation_loop(&mut keys, loop_duration);
    }
    if let Ok(curve) = AnimatableKeyframeCurve::new(keys) {
        clip.add_curve_to_target(
            target,
            AnimatableCurve::new(animated_field!(Transform::rotation), curve),
        );
    }
}

pub(crate) fn add_scale_curve(
    clip: &mut AnimationClip,
    target: AnimationTargetId,
    track: &AnimationTransformTrack,
    rest: &Transform,
    looping: bool,
    loop_duration: f32,
) {
    if track.scale.is_empty() {
        return;
    }
    let unity_reference = track
        .reference_scale
        .map_or_else(|| Vec3::from_array(track.scale[0].value), Vec3::from_array);
    let mut keys = ensure_two_keyframes(
        track.scale.iter().map(|key| {
            let value = Vec3::from_array(key.value);
            let ratio = Vec3::new(
                safe_ratio(value.x, unity_reference.x),
                safe_ratio(value.y, unity_reference.y),
                safe_ratio(value.z, unity_reference.z),
            );
            (key.time, rest.scale * ratio)
        }),
        clip.duration(),
    );
    if looping {
        close_scale_loop(&mut keys, loop_duration);
    }
    if let Ok(curve) = AnimatableKeyframeCurve::new(keys) {
        clip.add_curve_to_target(
            target,
            AnimatableCurve::new(animated_field!(Transform::scale), curve),
        );
    }
}

pub(crate) fn close_translation_loop(keys: &mut Vec<(f32, Vec3)>, duration: f32) {
    let Some(((start_time, start), (_, end), end_time)) = loop_endpoints(keys, duration) else {
        return;
    };
    let correction = start - end;
    for (time, value) in keys.iter_mut() {
        *value += correction * loop_phase(*time, start_time, end_time);
    }
    finish_loop_at(keys, end_time, start);
}

pub(crate) fn close_rotation_loop(keys: &mut Vec<(f32, Quat)>, duration: f32) {
    let Some(((start_time, start), (_, end), end_time)) = loop_endpoints(keys, duration) else {
        return;
    };
    // Apply the minimum end-to-start correction progressively through the
    // cycle. This preserves the authored motion while making the pose C0
    // continuous at Bevy's wrap point instead of snapping the limbs.
    let correction = end.inverse() * start;
    for (time, value) in keys.iter_mut() {
        let correction = Quat::IDENTITY.slerp(correction, loop_phase(*time, start_time, end_time));
        *value = (*value * correction).normalize();
    }
    finish_loop_at(keys, end_time, start);
}

pub(crate) fn close_scale_loop(keys: &mut Vec<(f32, Vec3)>, duration: f32) {
    let Some(((start_time, start), (_, end), end_time)) = loop_endpoints(keys, duration) else {
        return;
    };
    let correction = start - end;
    for (time, value) in keys.iter_mut() {
        *value += correction * loop_phase(*time, start_time, end_time);
    }
    finish_loop_at(keys, end_time, start);
}

type LoopEndpoints<T> = ((f32, T), (f32, T), f32);

pub(crate) fn loop_endpoints<T: Copy>(
    keys: &[(f32, T)],
    duration: f32,
) -> Option<LoopEndpoints<T>> {
    let first = *keys.first()?;
    let last = *keys.last()?;
    let end_time = duration.max(last.0);
    (end_time - first.0 > f32::EPSILON).then_some((first, last, end_time))
}

pub(crate) fn finish_loop_at<T: Copy>(keys: &mut Vec<(f32, T)>, end_time: f32, start: T) {
    let Some(last) = keys.last_mut() else {
        return;
    };
    if end_time - last.0 > f32::EPSILON {
        keys.push((end_time, start));
    } else {
        last.1 = start;
    }
}

pub(crate) fn loop_phase(time: f32, start_time: f32, end_time: f32) -> f32 {
    ((time - start_time) / (end_time - start_time)).clamp(0.0, 1.0)
}

pub(crate) fn ensure_two_keyframes<T: Clone>(
    keyframes: impl Iterator<Item = (f32, T)>,
    duration: f32,
) -> Vec<(f32, T)> {
    let mut keyframes: Vec<_> = keyframes.collect();
    if keyframes.len() == 1 {
        let (time, value) = keyframes[0].clone();
        keyframes.push((duration.max(time + 1.0 / 60.0), value));
    }
    keyframes
}

pub(crate) fn normalized_quat(value: [f32; 4]) -> Quat {
    let value = Quat::from_array(value);
    if value.length_squared() > 1.0e-8 {
        value.normalize()
    } else {
        Quat::IDENTITY
    }
}

pub(crate) fn unity_euler(value: [f32; 3]) -> Quat {
    Quat::from_euler(
        EulerRot::ZXY,
        value[2].to_radians(),
        value[0].to_radians(),
        value[1].to_radians(),
    )
}

pub(crate) fn safe_ratio(value: f32, reference: f32) -> f32 {
    if reference.abs() > 1.0e-6 {
        value / reference
    } else {
        1.0
    }
}
