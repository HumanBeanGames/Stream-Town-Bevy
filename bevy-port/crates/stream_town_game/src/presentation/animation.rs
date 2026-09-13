use super::super::*;

pub(crate) fn animate_agents(
    time: Res<Time>,
    render: Res<RenderAssets>,
    mut agents: Query<(
        &Agent,
        &mut AgentAnimation,
        Option<&mut MeshMaterial3d<StandardMaterial>>,
        &mut Transform,
    )>,
) {
    for (agent, mut animation, material, mut transform) in &mut agents {
        let moving = agent_is_moving(agent);
        let next_state = if moving {
            MovementAnimationState::Moving
        } else {
            MovementAnimationState::Idle
        };
        if animation.state != next_state {
            animation.state = next_state;
            animation.phase = 0.0;
        }
        animation.phase += time.delta_secs() * if moving { 9.0 } else { 2.0 };
        if animation.native {
            if transform.scale != animation.base_scale {
                transform.scale = animation.base_scale;
            }
            continue;
        }
        let pulse = if moving {
            1.0 + animation.phase.sin().abs() * 0.16
        } else {
            1.0
        };
        let desired_scale = animation.base_scale * Vec3::new(1.0, pulse, 1.0);
        if transform.scale != desired_scale {
            transform.scale = desired_scale;
        }
        if let Some(mut material) = material {
            let desired_material = actor_material(&render, &agent.kind, moving);
            if material.0 != desired_material {
                material.0 = desired_material;
            }
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn upgrade_actor_placeholders(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    asset_server: Option<Res<AssetServer>>,
    asset_root: Res<RuntimeAssetRoot>,
    world: Res<WorldRuntime>,
    mut placeholders: Query<
        (
            Entity,
            &Agent,
            &GridLocation,
            &mut AgentAnimation,
            &mut Transform,
        ),
        With<Mesh3d>,
    >,
    detailed: Query<(), (With<Agent>, With<WorldAssetRoot>)>,
) {
    let Some(asset_server) = asset_server else {
        return;
    };
    let mut remaining = actor_scene_budget().saturating_sub(detailed.iter().count());
    for (entity, agent, location, mut animation, mut transform) in &mut placeholders {
        if remaining == 0 {
            break;
        }
        let Some(archetype) = content.0.archetypes.get(&agent.archetype) else {
            continue;
        };
        let Some(scene) = runtime_archetype_scene(archetype)
            .filter(|scene| converted_asset_exists(&asset_root.0, &scene.asset_path))
        else {
            continue;
        };
        // Imported FBXs often contain a bind/default-pose clip. The authored
        // Animator controller remains authoritative whenever all of its
        // locomotion clips can be translated; native GLB animation is only the
        // fallback for assets without a complete converted controller.
        let converted = converted_animation_spec(archetype, &presentation.0).map(|mut spec| {
            spec.rig_scene.clone_from(&scene.asset_path);
            spec
        });
        let native = converted
            .is_none()
            .then(|| native_animation_request(archetype, &scene, &presentation.0))
            .flatten();
        let base_scale = Vec3::splat(config.0.world.cell_size / 2.0);
        animation.base_scale = base_scale;
        animation.native = native.is_some() || converted.is_some();
        *transform = Transform::from_translation(grid_to_world_on_surface(
            location.0,
            &config.0,
            &world.generated,
        ))
        .with_scale(base_scale);
        let mut actor = commands.entity(entity);
        actor
            .remove::<Mesh3d>()
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .insert(WorldAssetRoot(asset_server.load(
                GltfAssetLabel::Scene(0).from_asset(scene.asset_path.clone()),
            )));
        if let Some(native) = native {
            actor.insert(native);
        } else if let Some(converted) = converted {
            actor.insert(converted);
        }
        if let Some(material) = prefab_material_spec(archetype, &scene, &presentation.0, &render) {
            actor.insert(material);
        }
        if scene.asset_path == PLAYER_ANIMATED_MODEL_PATH {
            actor.insert(PlayerAnimatedRig);
        } else if archetype.kind == ArchetypeKind::Player {
            actor.insert(PlayerRigAxisCorrectionRequired);
        }
        remaining -= 1;
    }
}

pub(crate) fn resolve_native_animation_requests(
    mut commands: Commands,
    asset_server: Option<Res<AssetServer>>,
    animation_graphs: Option<ResMut<Assets<AnimationGraph>>>,
    mut cache: ResMut<NativeAnimationCache>,
    requests: Query<(Entity, &NativeAnimationRequest), Without<NativeAnimationSpec>>,
) {
    let (Some(asset_server), Some(mut animation_graphs)) = (asset_server, animation_graphs) else {
        return;
    };
    for (entity, request) in &requests {
        let key = (request.asset_path.clone(), request.animation_index);
        let spec = cache.0.entry(key).or_insert_with(|| {
            let (graph, node) = AnimationGraph::from_clip(
                asset_server.load(
                    GltfAssetLabel::Animation(
                        usize::try_from(request.animation_index)
                            .expect("animation index fits the current platform"),
                    )
                    .from_asset(request.asset_path.clone()),
                ),
            );
            NativeAnimationSpec {
                graph: animation_graphs.add(graph),
                idle: node,
                moving: node,
            }
        });
        commands
            .entity(entity)
            .insert(spec.clone())
            .remove::<NativeAnimationRequest>();
    }
}

pub(crate) fn attach_native_animations(
    mut commands: Commands,
    specs: Query<&NativeAnimationSpec>,
    parents: Query<&ChildOf>,
    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
) {
    for (entity, mut player) in &mut players {
        let mut ancestor = entity;
        for _ in 0..64 {
            if let Ok(spec) = specs.get(ancestor) {
                commands.entity(entity).insert((
                    AnimationGraphHandle(spec.graph.clone()),
                    ActorAnimationDriver {
                        actor_root: ancestor,
                        idle: spec.idle,
                        moving: spec.moving,
                        current: MovementAnimationState::Idle,
                    },
                ));
                player.play(spec.idle).repeat();
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn repair_replaced_converted_animation_instances(
    mut commands: Commands,
    roots: Query<
        Entity,
        (
            With<ConvertedAnimationSpec>,
            With<ConvertedAnimationInstanceReady>,
            With<ConvertedAnimationApplied>,
        ),
    >,
    drivers: Query<&ConvertedAnimationDriver>,
) {
    for actor_root in &roots {
        if drivers.iter().any(|driver| driver.actor_root == actor_root) {
            continue;
        }
        // The scene spawner can replace an imported hierarchy after a GLB
        // subasset finishes loading. Never let the marker on the stable actor
        // root strand that replacement without its controller.
        commands
            .entity(actor_root)
            .remove::<ConvertedAnimationApplied>();
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn attach_converted_animations(
    mut commands: Commands,
    asset_server: Option<Res<AssetServer>>,
    presentation: Res<RuntimePresentation>,
    animation_clips: Option<ResMut<Assets<AnimationClip>>>,
    animation_graphs: Option<ResMut<Assets<AnimationGraph>>>,
    mut cache: ResMut<ConvertedAnimationCache>,
    specs: Query<
        (Entity, &ConvertedAnimationSpec),
        (
            With<ConvertedAnimationInstanceReady>,
            Without<ConvertedAnimationApplied>,
        ),
    >,
    children: Query<&Children>,
    names: Query<&Name>,
    transforms: Query<&Transform>,
    native_players: Query<(), With<AnimationPlayer>>,
    applied: Query<(), (With<ConvertedAnimationDriver>, Without<ActivePetVisual>)>,
    pets: Query<(), With<ActivePetVisual>>,
    fish_gods: Query<(), With<FishGodAnimation>>,
    actors: Query<&Agent>,
) {
    let (Some(asset_server), Some(mut animation_clips), Some(mut animation_graphs)) =
        (asset_server, animation_clips, animation_graphs)
    else {
        return;
    };
    let animation_budget = animation_detail_budget();
    let mut remaining = animation_budget.saturating_sub(applied.iter().count());
    for (actor_root, spec) in &specs {
        let is_unbudgeted = pets.contains(actor_root)
            || fish_gods.contains(actor_root)
            || actors
                .get(actor_root)
                .is_ok_and(|actor| actor.kind == ActorKind::Enemy);
        if !is_unbudgeted && remaining == 0 {
            continue;
        }
        let Some(controller) = presentation.0.controllers.get(&spec.controller) else {
            continue;
        };
        let Some(state) = controller.states.get(&spec.state) else {
            continue;
        };
        let Some(root_clip) = state
            .motions
            .iter()
            .find_map(|motion| presentation.0.clips.get(&motion.clip))
        else {
            continue;
        };
        let animation_root = find_component_descendant(actor_root, &children, &native_players)
            .or_else(|| {
                rig_animation_root_name(&spec.rig_scene)
                    .and_then(|root| find_named_descendant(actor_root, root, &children, &names))
            })
            .or_else(|| {
                controller_animation_root_names(controller, &presentation.0)
                    .into_iter()
                    .find_map(|root| find_named_descendant(actor_root, root, &children, &names))
            })
            .or_else(|| {
                animation_root_name(root_clip)
                    .and_then(|root| find_named_descendant(actor_root, root, &children, &names))
            });
        let Some(animation_root) = animation_root else {
            continue;
        };
        let targets = collect_animation_targets(animation_root, &children, &names, &transforms);
        let cache_key = (
            spec.controller.clone(),
            spec.state.clone(),
            spec.rig_scene.clone(),
        );
        if !cache.0.contains_key(&cache_key) {
            let Some(cached) = build_converted_animation(
                controller,
                spec,
                &presentation.0,
                &targets,
                &asset_server,
                &mut animation_clips,
                &mut animation_graphs,
            ) else {
                continue;
            };
            cache.0.insert(cache_key.clone(), cached);
        }
        let cached = cache
            .0
            .get(&cache_key)
            .expect("converted animation cache was populated");
        let layers = cached
            .layers
            .iter()
            .filter_map(|template| {
                AnimationControllerRuntime::in_state(controller, template.fallback_state.clone())
                    .ok()
                    .map(|runtime| ConvertedAnimationLayerDriver {
                        fallback_state: template.fallback_state.clone(),
                        runtime,
                        nodes: template.nodes.clone(),
                        active: Vec::new(),
                        applied: Vec::new(),
                        crossfade: None,
                        state_offset: 0.0,
                        event_elapsed: BTreeMap::new(),
                    })
            })
            .collect();
        for (path, (entity, _)) in &targets {
            commands.entity(*entity).insert((
                path.split('/').collect::<AnimationTargetId>(),
                AnimatedBy(animation_root),
            ));
        }
        commands.entity(animation_root).insert((
            AnimationPlayer::default(),
            AnimationGraphHandle(cached.graph.clone()),
            ConvertedAnimationDriver {
                actor_root,
                controller: spec.controller.clone(),
                layers,
                last_alive: None,
                active_action: None,
                transient_carry_visible: false,
            },
        ));
        commands
            .entity(actor_root)
            .insert(ConvertedAnimationApplied);
        if !is_unbudgeted {
            remaining -= 1;
        }
    }
}

pub(crate) fn mark_converted_animation_instance_ready(
    ready: On<WorldInstanceReady>,
    mut commands: Commands,
    specs: Query<(), With<ConvertedAnimationSpec>>,
    applied: Query<(), With<ConvertedAnimationApplied>>,
    parents: Query<&ChildOf>,
) {
    let mut actor_root = ready.entity;
    for _ in 0..64 {
        if specs.contains(actor_root) {
            break;
        }
        let Ok(parent) = parents.get(actor_root) else {
            return;
        };
        actor_root = parent.parent();
    }
    if !specs.contains(actor_root) {
        return;
    }
    let mut entity = commands.entity(actor_root);
    entity.insert(ConvertedAnimationInstanceReady);
    // WorldInstanceReady is emitted again after a hot reload or any other
    // scene-instance replacement. The old player belongs to the discarded
    // hierarchy, so make the replacement hierarchy eligible for attachment.
    if applied.contains(actor_root) {
        entity.remove::<ConvertedAnimationApplied>();
    }
}

pub(crate) fn rig_animation_root_name(rig_scene: &str) -> Option<&'static str> {
    // The consolidated player GLB is rooted at CharacterArmature, while the
    // legacy Unity controller's raw curve paths begin at an FBX-only `pelvis`
    // node that does not exist in the converted hierarchy.
    (rig_scene == PLAYER_ANIMATED_MODEL_PATH).then_some("CharacterArmature")
}

pub(crate) fn gate_animation_contract(
    age: u8,
    presentation: &PresentationCatalog,
) -> Option<GateAnimationContract> {
    let (controller_name, model_root_name, animation_root_name) = if age >= 2 {
        ("GateStone", "Armature", "Age02_Gate")
    } else {
        ("GateWood", "Age01_Gate01", "Age01_Gate01")
    };
    let (controller_id, controller) = presentation
        .controllers
        .iter()
        .find(|(_, controller)| controller.display_name == controller_name)?;
    let request = |state_name: &str| {
        let (state_id, state) = controller
            .states
            .iter()
            .find(|(_, state)| state.display_name.eq_ignore_ascii_case(state_name))?;
        let clip = presentation.clips.get(&state.motions.first()?.clip)?;
        let asset_path = clip.converted_asset_path.clone()?;
        let animation_index = clip.gltf_animation_index?;
        let transition = controller
            .transitions
            .iter()
            .filter(|transition| transition.destination.as_ref() == Some(state_id))
            .filter(|transition| transition.is_any_state)
            .find(|transition| {
                transition
                    .conditions
                    .iter()
                    .any(|condition| condition.parameter.eq_ignore_ascii_case(state_name))
            });
        let transition_seconds = transition.map_or(0.0, |transition| {
            if transition.fixed_duration {
                transition.duration
            } else {
                transition.duration * clip.duration_seconds / state.speed.abs().max(f32::EPSILON)
            }
        });
        Some(GateAnimationClipRequest {
            asset_path,
            animation_index,
            speed: state.speed,
            transition_seconds: transition_seconds.max(0.0),
        })
    };
    Some(GateAnimationContract {
        controller: controller_id.clone(),
        model_root_name,
        animation_root_name,
        open: request("Open")?,
        close: request("Close")?,
    })
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn attach_gate_animations(
    mut commands: Commands,
    asset_server: Option<Res<AssetServer>>,
    animation_graphs: Option<ResMut<Assets<AnimationGraph>>>,
    content: Res<RuntimeContent>,
    presentation: Res<RuntimePresentation>,
    simulation: Res<SimulationRuntime>,
    mut cache: ResMut<GateAnimationCache>,
    buildings: Query<(
        Entity,
        &RuntimeBuilding,
        &BuildingPresentation,
        Option<&GateAnimationBinding>,
    )>,
    children: Query<&Children>,
    names: Query<&Name>,
    transforms: Query<&Transform>,
) {
    let (Some(asset_server), Some(mut animation_graphs)) = (asset_server, animation_graphs) else {
        return;
    };
    for (building_root, runtime, building_presentation, binding) in &buildings {
        let Some(state) = simulation.0.buildings.get(&runtime.id) else {
            continue;
        };
        let Some((building_id, _)) = content
            .0
            .buildings
            .iter()
            .find(|(_, definition)| definition.archetype == state.archetype)
        else {
            continue;
        };
        if building_id.as_str() != "building:gate" {
            continue;
        }
        let age = building_presentation.applied_age;
        if binding
            .is_some_and(|binding| binding.age == age && names.get(binding.animation_root).is_ok())
        {
            continue;
        }
        if binding.is_some() {
            commands
                .entity(building_root)
                .remove::<GateAnimationBinding>();
        }
        let Some(contract) = gate_animation_contract(age, &presentation.0) else {
            continue;
        };
        let Some(animation_root) =
            find_named_descendant(building_root, contract.model_root_name, &children, &names)
        else {
            continue;
        };
        let targets = collect_animation_targets_with_root_name(
            animation_root,
            contract.animation_root_name,
            &children,
            &names,
            &transforms,
        );
        let key = (
            contract.close.asset_path.clone(),
            contract.close.animation_index,
            contract.open.asset_path.clone(),
            contract.open.animation_index,
        );
        if !cache.0.contains_key(&key) {
            let clips = [&contract.close, &contract.open].map(|request| {
                asset_server.load(
                    GltfAssetLabel::Animation(
                        usize::try_from(request.animation_index)
                            .expect("animation index fits the current platform"),
                    )
                    .from_asset(request.asset_path.clone()),
                )
            });
            let (graph, nodes) = AnimationGraph::from_clips(clips);
            cache.0.insert(
                key.clone(),
                CachedGateAnimation {
                    graph: animation_graphs.add(graph),
                    close: nodes[0],
                    open: nodes[1],
                },
            );
        }
        let cached = cache
            .0
            .get(&key)
            .expect("gate animation cache was populated");
        for (path, (entity, _)) in &targets {
            commands.entity(*entity).insert((
                path.split('/').collect::<AnimationTargetId>(),
                AnimatedBy(animation_root),
            ));
        }
        let mut player = AnimationPlayer::default();
        let mut transitions = AnimationTransitions::new();
        transitions
            .play(&mut player, cached.close, Duration::ZERO)
            .set_speed(contract.close.speed);
        commands.entity(animation_root).insert((
            player,
            transitions,
            AnimationGraphHandle(cached.graph.clone()),
            GateAnimationDriver {
                building_root,
                open: cached.open,
                close: cached.close,
                open_speed: contract.open.speed,
                close_speed: contract.close.speed,
                open_transition_seconds: contract.open.transition_seconds,
                close_transition_seconds: contract.close.transition_seconds,
                is_open: false,
            },
        ));
        commands.entity(building_root).insert(GateAnimationBinding {
            age,
            animation_root,
        });
        info!(
            building = %runtime.id,
            controller = %contract.controller,
            age,
            targets = targets.len(),
            "attached converted Unity gate controller"
        );
    }
}

pub(crate) fn gate_trigger_contains(gate: Vec3, player: Vec3, half_extent: f32) -> bool {
    let relative = player - gate;
    relative.x.abs() <= half_extent && relative.z.abs() <= half_extent
}

pub(crate) fn drive_gate_animations(
    config: Res<RuntimeConfig>,
    simulation: Res<SimulationRuntime>,
    buildings: Query<(&RuntimeBuilding, &Transform)>,
    agents: Query<(&Agent, &Transform)>,
    mut players: Query<(
        &mut AnimationPlayer,
        &mut AnimationTransitions,
        &mut GateAnimationDriver,
    )>,
) {
    let half_extent = GATE_TRIGGER_HALF_EXTENT_UNITY_UNITS * config.0.world.cell_size / 2.0;
    for (mut player, mut transitions, mut driver) in &mut players {
        let Ok((runtime, gate_transform)) = buildings.get(driver.building_root) else {
            continue;
        };
        let Some(gate) = simulation.0.buildings.get(&runtime.id) else {
            continue;
        };
        let should_open = gate.complete
            && gate.health > 0
            && agents.iter().any(|(agent, transform)| {
                agent.kind == ActorKind::Player
                    && simulation
                        .0
                        .actors
                        .get(&agent.id)
                        .is_some_and(|actor| actor.alive)
                    && gate_trigger_contains(
                        gate_transform.translation,
                        transform.translation,
                        half_extent,
                    )
            });
        if should_open == driver.is_open {
            continue;
        }
        let (node, speed, transition_seconds) = if should_open {
            (
                driver.open,
                driver.open_speed,
                driver.open_transition_seconds,
            )
        } else {
            (
                driver.close,
                driver.close_speed,
                driver.close_transition_seconds,
            )
        };
        transitions
            .play(
                &mut player,
                node,
                Duration::from_secs_f32(transition_seconds),
            )
            .set_speed(speed);
        driver.is_open = should_open;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConvertedClipRequest {
    Asset { path: String, index: usize },
    Retargeted,
}

pub(crate) fn converted_clip_request(
    source: &AnimationClipDef,
    rig_scene: &str,
    presentation: &PresentationCatalog,
) -> Option<ConvertedClipRequest> {
    if let Some((path, index)) = skin_compatible_animation_request(source, rig_scene, presentation)
    {
        return Some(ConvertedClipRequest::Asset { path, index });
    }
    if !source.transform_tracks.is_empty() {
        return Some(ConvertedClipRequest::Retargeted);
    }
    Some(ConvertedClipRequest::Asset {
        path: source.converted_asset_path.clone()?,
        index: usize::try_from(source.gltf_animation_index?).ok()?,
    })
}

pub(crate) fn build_converted_animation(
    controller: &stream_town_domain::AnimationControllerDef,
    spec: &ConvertedAnimationSpec,
    presentation: &PresentationCatalog,
    targets: &BTreeMap<String, (Entity, Transform)>,
    asset_server: &AssetServer,
    animation_clips: &mut Assets<AnimationClip>,
    animation_graphs: &mut Assets<AnimationGraph>,
) -> Option<CachedConvertedAnimation> {
    let mut clip_ids = BTreeSet::new();
    let mut converted = Vec::new();
    for motion in controller
        .states
        .values()
        .flat_map(|state| state.motions.iter())
    {
        if !clip_ids.insert(motion.clip.clone()) {
            continue;
        }
        let source = presentation.clips.get(&motion.clip)?;
        let handle = match converted_clip_request(source, &spec.rig_scene, presentation)? {
            // A clip authored on the visible rig must win over similarly named
            // standalone Unity curves. Retargeting those curves onto the
            // assembled player hierarchy can rotate the whole model instead
            // of deforming its skin, even when the bone names appear to match.
            ConvertedClipRequest::Asset { path, index } => {
                asset_server.load(GltfAssetLabel::Animation(index).from_asset(path))
            }
            ConvertedClipRequest::Retargeted => {
                animation_clips.add(retargeted_animation_clip(source, targets)?)
            }
        };
        converted.push((motion.clip.clone(), handle));
    }
    if converted.is_empty() {
        return None;
    }
    let mut graph = AnimationGraph::new();
    // Unity's outer Animator composition is an override blend. Only layers
    // explicitly authored as additive may add deltas. Treating absolute glTF
    // clips as one large additive pose stacks them on the rest transform and
    // breaks the rig before any locomotion blend is evaluated.
    let composition = add_animation_composition(&mut graph);
    let layer_specs: Vec<_> = controller
        .layers
        .iter()
        .enumerate()
        .filter_map(|layer| {
            let (layer_index, layer) = layer;
            let machine = controller.state_machines.get(&layer.state_machine)?;
            let state = machine.default_state.clone()?;
            Some((layer_index, layer.clone(), state))
        })
        .collect();
    let layer_specs = if layer_specs.is_empty() {
        vec![(
            0,
            AnimationLayerDef {
                display_name: "Base Layer".to_owned(),
                state_machine: StableId::new("animation_state_machine:fallback:base")
                    .expect("fallback animation state-machine ID is valid"),
                blend_mode: AnimationLayerBlendMode::Override,
                default_weight: 0.0,
                avatar_mask: None,
            },
            spec.state.clone(),
        )]
    } else {
        layer_specs
    };
    let mut layers = Vec::new();
    for (layer_index, layer, state) in layer_specs {
        let mask = register_avatar_mask(
            &mut graph,
            u32::try_from(layer_index).expect("Animator layer count fits a Bevy mask"),
            layer
                .avatar_mask
                .as_ref()
                .and_then(|mask| presentation.avatar_masks.get(mask)),
            targets,
        );
        let parent = add_animation_layer_branch(
            &mut graph,
            layer.blend_mode,
            layer.effective_weight(layer_index),
            mask,
            composition,
        );
        let nodes = converted
            .iter()
            .filter(|(clip, _)| state_layer_owns_clip(controller, &state, clip))
            .map(|(clip, handle)| (clip.clone(), graph.add_clip(handle.clone(), 1.0, parent)))
            .collect();
        layers.push(ConvertedAnimationLayerTemplate {
            fallback_state: state,
            nodes,
        });
    }
    if layers.is_empty() {
        return None;
    }
    debug_assert!(matches!(
        graph.graph[composition].node_type,
        AnimationNodeType::Blend
    ));
    Some(CachedConvertedAnimation {
        graph: animation_graphs.add(graph),
        layers,
    })
}

pub(crate) fn skin_compatible_animation_request(
    source: &AnimationClipDef,
    rig_scene: &str,
    presentation: &PresentationCatalog,
) -> Option<(String, usize)> {
    if source.rig_asset_path.as_deref() == Some(rig_scene) {
        let path = source.converted_asset_path.clone()?;
        let index = usize::try_from(source.gltf_animation_index?).ok()?;
        return Some((path, index));
    }
    if rig_scene != PLAYER_ANIMATED_MODEL_PATH {
        return None;
    }
    let native_name = native_character_animation_name(&source.display_name)?;
    presentation.clips.values().find_map(|candidate| {
        if candidate.display_name == native_name
            && candidate.rig_asset_path.as_deref() == Some(rig_scene)
            && candidate.converted_asset_path.as_deref() == Some(rig_scene)
        {
            Some((
                rig_scene.to_owned(),
                usize::try_from(candidate.gltf_animation_index?).ok()?,
            ))
        } else {
            None
        }
    })
}

/// Resolves an authored clip to an animation embedded in a compatible visible
/// rig. Focused authoring tools use this same mapping as runtime animation so a
/// role preview cannot silently select a different take from the shipping game.
#[must_use]
pub fn preview_animation_asset_for_rig(
    source: &AnimationClipDef,
    rig_scene: &str,
    presentation: &PresentationCatalog,
) -> Option<(String, u32)> {
    let (path, index) = skin_compatible_animation_request(source, rig_scene, presentation)?;
    Some((path, u32::try_from(index).ok()?))
}

pub(crate) fn native_character_animation_name(source_name: &str) -> Option<&'static str> {
    match source_name {
        "PlayerChar_Idle_01" => Some("CharacterIdleBase"),
        "PlayerChar_Walk_01" => Some("CharacterWalk"),
        "PlayerChar_Run_01" => Some("CharacterRun"),
        "PlayerChar_Logging_05" => Some("CharacterWoodCutting"),
        "PlayerChar_SpearAttack_01" => Some("CharacterSpearAttack"),
        "PlayerChar_Building_01" => Some("CharacterBuild"),
        "PlayerChar_Farming_01" => Some("CharacterFarming"),
        "PlayerChar_DEATH_01" => Some("CharacterDeath"),
        "PlayerChar_Healing" => Some("CharacterHeal"),
        "PlayerChar_HammerAttack_01" => Some("CharacterHammerAttack"),
        "PlayerChar_BowShoot_01" => Some("CharacterBowShoot"),
        "PlayerChar_LoggerCarryWood_01" => Some("Carry_Wood"),
        "PlayerChar_Gathering_01" => Some("CharacterGathering"),
        "PlayerChar_Summon" | "PlayerChar_Magic" => Some("CharacterCasting"),
        "PlayerChar_Fishing_01" => Some("CharacterFishing"),
        "PlayerChar_Mining_02" => Some("CharacterMining"),
        "PlayerChar_SwordAttack_01" => Some("CharacterLongSwordAttack"),
        "PlayerChar_CarryItems_01" => Some("CarryAtHip"),
        _ => None,
    }
}

pub(crate) fn add_animation_composition(graph: &mut AnimationGraph) -> AnimationNodeIndex {
    graph.add_blend(1.0, graph.root)
}

pub(crate) fn add_animation_layer_branch(
    graph: &mut AnimationGraph,
    blend_mode: AnimationLayerBlendMode,
    weight: f32,
    mask: u64,
    parent: AnimationNodeIndex,
) -> AnimationNodeIndex {
    match (blend_mode, mask) {
        (AnimationLayerBlendMode::Override, 0) => graph.add_blend(weight, parent),
        (AnimationLayerBlendMode::Override, mask) => {
            graph.add_blend_with_mask(mask, weight, parent)
        }
        (AnimationLayerBlendMode::Additive, 0) => graph.add_additive_blend(weight, parent),
        (AnimationLayerBlendMode::Additive, mask) => {
            graph.add_additive_blend_with_mask(mask, weight, parent)
        }
    }
}

pub(crate) fn register_avatar_mask(
    graph: &mut AnimationGraph,
    mask_group: u32,
    authored: Option<&AvatarMaskDef>,
    targets: &BTreeMap<String, (Entity, Transform)>,
) -> u64 {
    let Some(authored) = authored else {
        return 0;
    };
    let mut excluded = false;
    for path in targets.keys() {
        if authored
            .transform_weights
            .get(path)
            .is_some_and(|weight| weight.abs() < f32::EPSILON)
        {
            graph.add_target_to_mask_group(path.split('/').collect(), mask_group);
            excluded = true;
        }
    }
    if excluded { 1_u64 << mask_group } else { 0 }
}

pub(crate) fn state_layer_owns_clip(
    controller: &stream_town_domain::AnimationControllerDef,
    layer_state: &StableId,
    clip: &StableId,
) -> bool {
    let Some(root) = controller.layers.iter().find_map(|layer| {
        controller
            .state_machines
            .get(&layer.state_machine)
            .and_then(|machine| {
                (machine.default_state.as_ref() == Some(layer_state))
                    .then_some(&layer.state_machine)
            })
    }) else {
        return true;
    };
    state_machine_state_ids(controller, root).any(|state| {
        controller.states[state]
            .motions
            .iter()
            .any(|motion| &motion.clip == clip)
    })
}

pub(crate) fn state_machine_state_ids<'a>(
    controller: &'a stream_town_domain::AnimationControllerDef,
    root: &'a StableId,
) -> impl Iterator<Item = &'a StableId> {
    let mut pending = vec![root];
    let mut states = Vec::new();
    let mut visited = BTreeSet::new();
    while let Some(machine_id) = pending.pop() {
        if !visited.insert(machine_id) {
            continue;
        }
        let Some(machine) = controller.state_machines.get(machine_id) else {
            continue;
        };
        states.extend(&machine.states);
        pending.extend(&machine.child_state_machines);
    }
    states.into_iter()
}

pub(crate) fn animation_root_name(clip: &AnimationClipDef) -> Option<&str> {
    clip.transform_tracks
        .iter()
        .filter(|track| track.target_path != "$root")
        .find_map(|track| track.target_path.split('/').next())
}

pub(crate) fn controller_animation_root_names<'a>(
    controller: &stream_town_domain::AnimationControllerDef,
    presentation: &'a PresentationCatalog,
) -> Vec<&'a str> {
    // Native GLB clips intentionally have no converted transform tracks. Scan
    // the rest of the same Unity controller for its authored armature root so
    // the native idle/run clips can attach to the imported scene hierarchy.
    controller
        .states
        .values()
        .flat_map(|state| &state.motions)
        .filter_map(|motion| presentation.clips.get(&motion.clip))
        .filter_map(animation_root_name)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn find_named_descendant(
    root: Entity,
    target: &str,
    children: &Query<&Children>,
    names: &Query<&Name>,
) -> Option<Entity> {
    let mut pending = vec![root];
    while let Some(entity) = pending.pop() {
        if names.get(entity).is_ok_and(|name| name.as_str() == target) {
            return Some(entity);
        }
        if let Ok(entity_children) = children.get(entity) {
            pending.extend(entity_children.iter().rev());
        }
    }
    None
}

pub(crate) fn find_component_descendant<T: Component>(
    root: Entity,
    children: &Query<&Children>,
    components: &Query<(), With<T>>,
) -> Option<Entity> {
    let mut pending = vec![root];
    while let Some(entity) = pending.pop() {
        if components.contains(entity) {
            return Some(entity);
        }
        if let Ok(entity_children) = children.get(entity) {
            pending.extend(entity_children.iter().rev());
        }
    }
    None
}

pub(crate) fn equipment_node_names(content: &ContentCatalog) -> BTreeSet<String> {
    content
        .roles
        .values()
        .filter_map(|role| role.equipment.as_ref())
        .flat_map(|equipment| {
            equipment
                .body_nodes
                .iter()
                .cloned()
                .chain(equipment.left_hand_node.iter().cloned())
                .chain(equipment.right_hand_node.iter().cloned())
                .chain(equipment.helmet_node.iter().cloned())
        })
        .collect()
}

pub(crate) fn canonical_equipment_node_name(name: &str) -> &str {
    // Blender retained the suffix on starter tool object names while Unity's
    // role catalog references the same renderers without it.
    name.strip_suffix("_Starter").unwrap_or(name)
}

pub(crate) fn player_equipment_slot_node(name: &str) -> bool {
    let name = canonical_equipment_node_name(name);
    ["Body_", "Back_", "LHand_", "RHand_", "Helmet_"]
        .iter()
        .any(|prefix| name.starts_with(prefix))
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct EnemyModelSelection {
    pub(crate) base_model: Option<usize>,
    pub(crate) optional_models: BTreeSet<usize>,
    pub(crate) weapon: Option<usize>,
}

pub(crate) fn enemy_model_hash(actor: &StableId, choice: &str, index: usize) -> u32 {
    actor
        .as_str()
        .bytes()
        .chain(choice.bytes())
        .chain(
            u64::try_from(index)
                .expect("model-choice index fits u64")
                .to_le_bytes(),
        )
        .fold(2_166_136_261_u32, |hash, byte| {
            hash.wrapping_mul(16_777_619) ^ u32::from(byte)
        })
}

pub(crate) fn enemy_model_selection(
    actor: &StableId,
    models: &EnemyModelSetDef,
) -> EnemyModelSelection {
    let selected_index = |choice: &str, count: usize| {
        (count > 0).then(|| {
            usize::try_from(enemy_model_hash(actor, choice, 0)).expect("u32 fits usize") % count
        })
    };
    EnemyModelSelection {
        base_model: selected_index("base", models.base_models.len()),
        optional_models: (0..models.optional_models.len())
            .filter(|index| enemy_model_hash(actor, "optional", *index) & 1 == 1)
            .collect(),
        weapon: selected_index("weapon", models.weapons.len()),
    }
}

pub(crate) fn enemy_model_node_names(content: &ContentCatalog) -> BTreeSet<String> {
    content
        .archetypes
        .values()
        .filter_map(|archetype| archetype.enemy_models.as_ref())
        .flat_map(|models| {
            models
                .base_models
                .iter()
                .chain(&models.permanent_models)
                .chain(&models.optional_models)
                .cloned()
                .chain(models.weapons.iter().flat_map(|weapon| {
                    std::iter::once(weapon.main_model.clone())
                        .chain(weapon.off_hand_models.iter().cloned())
                }))
        })
        .collect()
}

/// Blender's GLB export preserves the player's FBX pre-rotation as a +90-degree
/// local X rotation. Unity consumes that axis conversion while importing the
/// FBX. Correct the complete actor presentation above the glTF scene so all
/// skinned meshes, joints, and equipment receive one coherent rigid transform.
#[allow(clippy::type_complexity)]
pub(crate) fn correct_player_rig_axis(
    mut commands: Commands,
    mut agents: Query<
        (Entity, &mut Transform),
        (
            Without<PlayerRigAxisCorrected>,
            With<WorldAssetRoot>,
            With<PlayerRigAxisCorrectionRequired>,
        ),
    >,
) {
    for (entity, mut transform) in &mut agents {
        transform.rotation *= Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
        commands.entity(entity).insert(PlayerRigAxisCorrected);
    }
}

pub(crate) fn debug_player_model_bounds(
    mut settled_frames: Local<u8>,
    agents: Query<(), With<Agent>>,
    parents: Query<&ChildOf>,
    renderers: Query<(Entity, &Aabb, &GlobalTransform), With<Mesh3d>>,
) {
    if *settled_frames == u8::MAX || std::env::var_os("STREAM_TOWN_DEBUG_PLAYER_BOUNDS").is_none() {
        return;
    }
    let mut minimum = Vec3::splat(f32::INFINITY);
    let mut maximum = Vec3::splat(f32::NEG_INFINITY);
    let mut count = 0_usize;
    for (entity, aabb, global) in &renderers {
        let mut ancestor = entity;
        let mut belongs_to_actor = agents.contains(ancestor);
        for _ in 0..64 {
            if belongs_to_actor {
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
            belongs_to_actor = agents.contains(ancestor);
        }
        if !belongs_to_actor {
            continue;
        }
        let centre = Vec3::from(aabb.center);
        let half = Vec3::from(aabb.half_extents);
        for x in [-1.0, 1.0] {
            for y in [-1.0, 1.0] {
                for z in [-1.0, 1.0] {
                    let point = global.transform_point(centre + half * Vec3::new(x, y, z));
                    minimum = minimum.min(point);
                    maximum = maximum.max(point);
                }
            }
        }
        count += 1;
    }
    if count >= 80 {
        if *settled_frames < 3 {
            *settled_frames += 1;
            return;
        }
        info!(
            renderers = count,
            ?minimum,
            ?maximum,
            "resolved player model bounds"
        );
        *settled_frames = u8::MAX;
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn tag_enemy_model_nodes(
    mut commands: Commands,
    content: Res<RuntimeContent>,
    agents: Query<&Agent>,
    parents: Query<&ChildOf>,
    nodes: Query<(Entity, &Name), (Without<EnemyModelNodeProcessed>, Without<Agent>)>,
) {
    if nodes.is_empty() {
        return;
    }
    let names = enemy_model_node_names(&content.0);
    for (entity, name) in &nodes {
        if !names.contains(name.as_str()) {
            commands.entity(entity).insert(EnemyModelNodeProcessed);
            continue;
        }
        let mut ancestor = entity;
        let mut traversed_parent = false;
        for _ in 0..64 {
            let Ok(parent) = parents.get(ancestor) else {
                if traversed_parent {
                    commands.entity(entity).insert(EnemyModelNodeProcessed);
                }
                break;
            };
            traversed_parent = true;
            ancestor = parent.parent();
            let Ok(agent) = agents.get(ancestor) else {
                continue;
            };
            let Some(models) = content
                .0
                .archetypes
                .get(&agent.archetype)
                .and_then(|archetype| archetype.enemy_models.as_ref())
            else {
                break;
            };
            let visible = enemy_model_node_visible(
                models,
                &enemy_model_selection(&agent.id, models),
                name.as_str(),
            );
            commands.entity(entity).insert(if visible {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            });
            commands.entity(entity).insert(EnemyModelNodeProcessed);
            break;
        }
    }
}

pub(crate) fn enemy_model_node_visible(
    models: &EnemyModelSetDef,
    selection: &EnemyModelSelection,
    name: &str,
) -> bool {
    if models.permanent_models.iter().any(|model| model == name) {
        return true;
    }
    let weapon_membership: Vec<_> = models
        .weapons
        .iter()
        .enumerate()
        .filter(|(_, weapon)| {
            weapon.main_model == name || weapon.off_hand_models.iter().any(|model| model == name)
        })
        .map(|(index, _)| index)
        .collect();
    if !weapon_membership.is_empty() {
        return selection
            .weapon
            .is_some_and(|index| weapon_membership.contains(&index));
    }
    let optional_membership: Vec<_> = models
        .optional_models
        .iter()
        .enumerate()
        .filter_map(|(index, model)| (model == name).then_some(index))
        .collect();
    if !optional_membership.is_empty() {
        return optional_membership
            .iter()
            .any(|index| selection.optional_models.contains(index));
    }
    models
        .base_models
        .iter()
        .enumerate()
        .any(|(index, model)| selection.base_model == Some(index) && model == name)
}

#[allow(clippy::type_complexity)]
pub(crate) fn tag_equipment_nodes(
    mut commands: Commands,
    content: Res<RuntimeContent>,
    agents: Query<Entity, (With<Agent>, With<PlayerAnimatedRig>)>,
    parents: Query<&ChildOf>,
    nodes: Query<(Entity, &Name), (Without<EquipmentNodeProcessed>, Without<Agent>)>,
) {
    if nodes.is_empty() {
        return;
    }
    let names = equipment_node_names(&content.0);
    for (entity, name) in &nodes {
        // Characters.glb includes a handful of model slots that are inactive
        // in Player_Character.prefab and therefore absent from the serialized
        // role equipment list (the three Body_Default variants and commander
        // banner). Treat every player equipment-shaped node as a controlled
        // slot so those imported defaults cannot leak through underneath the
        // selected role body and z-fight around the shoulders.
        if !names.contains(canonical_equipment_node_name(name.as_str()))
            && !player_equipment_slot_node(name.as_str())
        {
            commands.entity(entity).insert(EquipmentNodeProcessed);
            continue;
        }
        let mut ancestor = entity;
        let mut traversed_parent = false;
        for _ in 0..64 {
            let Ok(parent) = parents.get(ancestor) else {
                if traversed_parent {
                    commands.entity(entity).insert(EquipmentNodeProcessed);
                }
                break;
            };
            traversed_parent = true;
            ancestor = parent.parent();
            if agents.contains(ancestor) {
                commands.entity(entity).insert(EquipmentNode {
                    actor_root: ancestor,
                    name: name.as_str().to_owned(),
                });
                commands.entity(entity).insert(EquipmentNodeProcessed);
                commands.entity(ancestor).insert(AgentEquipmentPresentation);
                break;
            }
        }
    }
}

pub(crate) fn equipment_node_visible(
    equipment: &RoleEquipmentDef,
    body_type: u8,
    name: &str,
    carrying: bool,
) -> bool {
    let name = canonical_equipment_node_name(name);
    equipment.body_nodes[usize::from(body_type).min(equipment.body_nodes.len() - 1)] == name
        || equipment.right_hand_node.as_deref() == Some(name)
        || equipment.helmet_node.as_deref() == Some(name)
        || (equipment.left_hand_node.as_deref() == Some(name)
            && (equipment.left_hand_permanent || carrying))
}

pub(crate) fn carried_resource_visible(state: MovementAnimationState, has_inventory: bool) -> bool {
    state == MovementAnimationState::Moving && has_inventory
}

pub(crate) fn actor_carries_role_resource(content: &ContentCatalog, actor: &ActorState) -> bool {
    resource_for_role(content, &actor.role)
        .is_some_and(|resource| actor.inventory.get(&resource).copied().unwrap_or_default() > 0)
}

pub(crate) fn sync_equipment_nodes(
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    agents: Query<
        (&Agent, &AgentAnimation, Option<&TransientCarryVisibility>),
        With<AgentEquipmentPresentation>,
    >,
    mut nodes: Query<(&EquipmentNode, &mut Visibility)>,
) {
    for (node, mut visibility) in &mut nodes {
        let Ok((agent, animation, transient_carry)) = agents.get(node.actor_root) else {
            continue;
        };
        let Some(actor) = simulation.0.actors.get(&agent.id) else {
            continue;
        };
        let equipment = content
            .0
            .roles
            .get(&actor.role)
            .and_then(|role| role.equipment.as_ref());
        // Unity's carried-resource prop is hidden during the collection action
        // and shown on the return walk. Inventory alone was too broad and left
        // the gathered item in-hand throughout the gathering animation.
        let carrying = transient_carry.is_some_and(|carry| carry.0)
            || carried_resource_visible(
                animation.state,
                actor_carries_role_resource(&content.0, actor),
            );
        let visible = equipment.map_or_else(
            || {
                [
                    "Body_Default_Slim",
                    "Body_Default_Bulk",
                    "Body_Default_Feminine",
                ][usize::from(actor.customization.body_type).min(2)]
                    == canonical_equipment_node_name(&node.name)
            },
            |equipment| {
                equipment_node_visible(
                    equipment,
                    actor.customization.body_type,
                    &node.name,
                    carrying,
                )
            },
        );
        let desired = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != desired {
            *visibility = desired;
        }
    }
}

pub(crate) fn cosmetic_node(name: &str) -> Option<(CosmeticNodeKind, u8)> {
    EYE_NODES
        .iter()
        .position(|candidate| *candidate == name)
        .map(|index| {
            (
                CosmeticNodeKind::Eyes,
                u8::try_from(index).expect("eye index fits"),
            )
        })
        .or_else(|| {
            HAIR_NODES
                .iter()
                .position(|candidate| *candidate == name)
                .map(|index| {
                    (
                        CosmeticNodeKind::Hair,
                        u8::try_from(index).expect("hair index fits"),
                    )
                })
        })
        .or_else(|| {
            FACIAL_HAIR_NODES
                .iter()
                .position(|candidate| *candidate == name)
                .map(|index| {
                    (
                        CosmeticNodeKind::FacialHair,
                        u8::try_from(index).expect("facial-hair index fits"),
                    )
                })
        })
}

#[allow(clippy::type_complexity)]
pub(crate) fn tag_cosmetic_nodes(
    mut commands: Commands,
    agents: Query<Entity, With<Agent>>,
    parents: Query<&ChildOf>,
    nodes: Query<(Entity, &Name), (Without<CosmeticNodeProcessed>, Without<Agent>)>,
) {
    if nodes.is_empty() {
        return;
    }
    for (entity, name) in &nodes {
        let Some((kind, index)) = cosmetic_node(name.as_str()) else {
            commands.entity(entity).insert(CosmeticNodeProcessed);
            continue;
        };
        let mut ancestor = entity;
        let mut traversed_parent = false;
        for _ in 0..64 {
            let Ok(parent) = parents.get(ancestor) else {
                if traversed_parent {
                    commands.entity(entity).insert(CosmeticNodeProcessed);
                }
                break;
            };
            traversed_parent = true;
            ancestor = parent.parent();
            if agents.contains(ancestor) {
                commands.entity(entity).insert(CosmeticNode {
                    actor_root: ancestor,
                    kind,
                    index,
                });
                commands.entity(entity).insert(CosmeticNodeProcessed);
                break;
            }
        }
    }
}

pub(crate) fn cosmetic_node_visible(
    customization: ActorCustomization,
    kind: CosmeticNodeKind,
    index: u8,
    helmet_equipped: bool,
) -> bool {
    match kind {
        CosmeticNodeKind::Eyes => customization.eyes == index,
        CosmeticNodeKind::Hair => customization.hair == index && !helmet_equipped,
        CosmeticNodeKind::FacialHair => customization.facial_hair == index,
        CosmeticNodeKind::Skin => true,
    }
}

pub(crate) fn sync_cosmetic_nodes(
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    agents: Query<&Agent>,
    mut nodes: Query<(&CosmeticNode, &mut Visibility)>,
) {
    for (node, mut visibility) in &mut nodes {
        let Ok(agent) = agents.get(node.actor_root) else {
            continue;
        };
        let Some(actor) = simulation.0.actors.get(&agent.id) else {
            continue;
        };
        let helmet_equipped = content
            .0
            .roles
            .get(&actor.role)
            .and_then(|role| role.equipment.as_ref())
            .and_then(|equipment| equipment.helmet_node.as_ref())
            .is_some();
        let desired =
            if cosmetic_node_visible(actor.customization, node.kind, node.index, helmet_equipped) {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        if *visibility != desired {
            *visibility = desired;
        }
    }
}

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

pub(crate) fn animation_binding_diagnostics_enabled() -> bool {
    std::env::var_os("STREAM_TOWN_DEBUG_ANIMATION_BINDINGS").is_some()
        || std::env::var_os("STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP").is_some()
}

pub(crate) fn report_animation_loop_diagnostics(
    time: Res<Time>,
    agents: Query<&Agent>,
    players: Query<(&AnimationPlayer, &ConvertedAnimationDriver)>,
    mut observed: Local<HashMap<(Entity, AnimationNodeIndex), u32>>,
    mut last_report_second: Local<u64>,
) {
    if !animation_binding_diagnostics_enabled() {
        return;
    }
    let report_second = time.elapsed().as_secs();
    let heartbeat = report_second > *last_report_second;
    if heartbeat {
        *last_report_second = report_second;
    }
    for (player, driver) in &players {
        let Ok(agent) = agents.get(driver.actor_root) else {
            continue;
        };
        for (node, animation) in player.playing_animations() {
            if animation.repeat_mode() != RepeatAnimation::Forever {
                continue;
            }
            let clip = driver.layers.iter().find_map(|layer| {
                layer
                    .nodes
                    .iter()
                    .find_map(|(clip, candidate)| (candidate == node).then_some(clip.as_str()))
            });
            if clip != Some("clip:d9990a3e12bab1c4787d70db804821e3") {
                continue;
            }
            let completions = animation.completions();
            let previous = observed
                .insert((driver.actor_root, *node), completions)
                .unwrap_or(completions);
            if heartbeat {
                info!(
                    stable_id = %agent.id,
                    ?node,
                    clip,
                    elapsed = animation.elapsed(),
                    completions,
                    seek_time = animation.seek_time(),
                    "run-loop playback heartbeat"
                );
            }
            if completions <= previous {
                continue;
            }
            info!(
                stable_id = %agent.id,
                ?node,
                clip,
                completions,
                seek_time = animation.seek_time(),
                "looping animation completed without a playback restart"
            );
        }
    }
}

pub(crate) fn report_animation_binding_diagnostics(
    agents: Query<&Agent>,
    players: Query<(Entity, &AnimationPlayer, &ConvertedAnimationDriver)>,
    targets: Query<(Entity, &Name, &Transform, &AnimatedBy)>,
    skins: Query<(
        Entity,
        &SkinnedMesh,
        Option<&Visibility>,
        Option<&InheritedVisibility>,
    )>,
    parents: Query<&ChildOf>,
    names: Query<&Name>,
    mut announced: Local<bool>,
    mut samples: Local<u8>,
    mut previous: Local<HashMap<Entity, (f32, Quat)>>,
) {
    if !animation_binding_diagnostics_enabled() {
        return;
    }
    if !*announced {
        info!("animation binding diagnostics enabled");
        *announced = true;
    }
    // Capture consecutive real frames. This distinguishes an active graph from
    // an animation that actually changes a joint used by the visible skin,
    // even in automated windows whose wall-clock updates can be throttled.
    if *samples >= 8 {
        return;
    }
    *samples += 1;
    for (player_entity, player, driver) in &players {
        let stable_id = agents
            .get(driver.actor_root)
            .map_or("<non-agent>", |agent| agent.id.as_str());
        let player_elapsed = player
            .playing_animations()
            .map(|(_, animation)| animation.elapsed())
            .fold(0.0, f32::max);
        let player_seek_time = player
            .playing_animations()
            .map(|(_, animation)| animation.seek_time())
            .fold(0.0, f32::max);
        let playback_speeds = player
            .playing_animations()
            .map(|(_, animation)| animation.speed())
            .collect::<Vec<_>>();
        let Some((joint, name, transform, _)) = targets.iter().find(|(_, name, _, animated_by)| {
            animated_by.0 == player_entity
                && matches!(name.as_str(), "UpperArm_L" | "Thigh_L" | "Body")
        }) else {
            warn!(
                actor = ?driver.actor_root,
                stable_id,
                player = ?player_entity,
                player_elapsed,
                "animation binding diagnostic found no representative joint"
            );
            continue;
        };
        let actor_skins: Vec<_> = skins
            .iter()
            .filter(|(entity, _, _, _)| {
                let mut ancestor = *entity;
                for _ in 0..64 {
                    if ancestor == driver.actor_root {
                        return true;
                    }
                    let Ok(parent) = parents.get(ancestor) else {
                        break;
                    };
                    ancestor = parent.parent();
                }
                false
            })
            .collect();
        let skinned_meshes = actor_skins.len();
        let joint_skin_references = skins
            .iter()
            .filter(|(_, skin, _, _)| skin.joints.contains(&joint))
            .count();
        let visible_skinned_meshes = actor_skins
            .iter()
            .filter(|(_, _, visibility, inherited)| {
                !matches!(visibility, Some(Visibility::Hidden))
                    && inherited.is_none_or(|visibility| visibility.get())
            })
            .count();
        let visible_skin_names = if visible_skinned_meshes < skinned_meshes {
            actor_skins
                .iter()
                .filter(|(_, _, visibility, inherited)| {
                    !matches!(visibility, Some(Visibility::Hidden))
                        && inherited.is_none_or(|visibility| visibility.get())
                })
                .filter_map(|(entity, _, _, _)| {
                    let mut ancestor = *entity;
                    for _ in 0..8 {
                        if let Ok(name) = names.get(ancestor)
                            && (player_equipment_slot_node(name.as_str())
                                || cosmetic_node(name.as_str()).is_some()
                                || name.as_str() == "Head_Box")
                        {
                            return Some(name.as_str().to_owned());
                        }
                        ancestor = parents.get(ancestor).ok()?.parent();
                    }
                    None
                })
                .collect::<BTreeSet<_>>()
        } else {
            BTreeSet::new()
        };
        let (elapsed_delta, rotation_delta) =
            previous
                .get(&player_entity)
                .map_or((0.0, 0.0), |(old_elapsed, old_rotation)| {
                    (
                        player_elapsed - old_elapsed,
                        old_rotation.angle_between(transform.rotation),
                    )
                });
        info!(
            actor = ?driver.actor_root,
            stable_id,
            player = ?player_entity,
            active_nodes = player.playing_animations().count(),
            player_elapsed,
            player_seek_time,
            playback_speeds = ?playback_speeds,
            elapsed_delta,
            joint = %name,
            joint_entity = ?joint,
            rotation_delta,
            joint_skin_references,
            skinned_meshes,
            visible_skinned_meshes,
            visible_skin_names = ?visible_skin_names,
            "animation binding diagnostic"
        );
        previous.insert(player_entity, (player_elapsed, transform.rotation));
    }
}

#[allow(
    clippy::cast_precision_loss,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
pub(crate) fn report_live_skin_deformation(
    meshes: Option<Res<Assets<Mesh>>>,
    inverse_bindposes: Option<Res<Assets<SkinnedMeshInverseBindposes>>>,
    agents: Query<(Entity, &Agent, &GlobalTransform)>,
    players: Query<&ConvertedAnimationDriver>,
    skins: Query<(
        Entity,
        &SkinnedMesh,
        &Mesh3d,
        Option<&Visibility>,
        Option<&InheritedVisibility>,
    )>,
    globals: Query<&GlobalTransform>,
    parents: Query<&ChildOf>,
    names: Query<&Name>,
    mut samples: Local<u8>,
    mut previous_vertices: Local<HashMap<Entity, Vec<Vec3>>>,
) {
    if !animation_binding_diagnostics_enabled() || *samples >= 8 {
        return;
    }
    let (Some(meshes), Some(inverse_bindposes)) = (meshes, inverse_bindposes) else {
        return;
    };
    let Some(driver) = players.iter().find(|driver| {
        agents
            .get(driver.actor_root)
            .is_ok_and(|(_, agent, _)| agent.id.as_str() == "npc:starting_defender")
    }) else {
        return;
    };
    let Ok((_, agent, actor_global)) = agents.get(driver.actor_root) else {
        return;
    };
    let actor_from_world = actor_global.affine().inverse();
    let visible_body_skins = skins
        .iter()
        .filter(|(skin_entity, _, _, visibility, inherited_visibility)| {
            !matches!(visibility, Some(Visibility::Hidden))
                && inherited_visibility.is_none_or(|visibility| visibility.get())
                && is_descendant_of(*skin_entity, driver.actor_root, &parents)
        })
        .filter_map(|skin| {
            let slot_name = named_character_slot_ancestor(skin.0, &parents, &names)?;
            slot_name.starts_with("Body_").then_some((skin, slot_name))
        })
        .collect::<Vec<_>>();
    // The imported scene begins with all source variants inherited-visible for
    // one frame, before the role/cosmetic visibility pass settles. Sampling
    // that frame is both misleading and needlessly expensive; the renderer we
    // need to prove is the one selected body after propagation.
    if visible_body_skins.len() != 1 {
        return;
    }
    for ((skin_entity, skin, mesh_handle, _, _), slot_name) in visible_body_skins {
        let Some(mesh) = meshes.get(&mesh_handle.0) else {
            continue;
        };
        let Some(bindposes) = inverse_bindposes.get(&skin.inverse_bindposes) else {
            continue;
        };
        let Some(VertexAttributeValues::Float32x3(positions)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            continue;
        };
        let Some(VertexAttributeValues::Uint16x4(joint_indices)) =
            mesh.attribute(Mesh::ATTRIBUTE_JOINT_INDEX)
        else {
            continue;
        };
        let Some(VertexAttributeValues::Float32x4(joint_weights)) =
            mesh.attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT)
        else {
            continue;
        };
        if positions.len() != joint_indices.len() || positions.len() != joint_weights.len() {
            warn!(
                stable_id = %agent.id,
                slot = slot_name,
                positions = positions.len(),
                joint_indices = joint_indices.len(),
                joint_weights = joint_weights.len(),
                "live skin diagnostic found mismatched vertex attributes"
            );
            continue;
        }
        let vertices = positions
            .iter()
            .zip(joint_indices)
            .zip(joint_weights)
            .map(|((position, indices), weights)| {
                let position = Vec3::from_array(*position);
                let world_position = indices.iter().zip(weights).fold(
                    Vec3::ZERO,
                    |deformed, (&joint_index, &weight)| {
                        if weight <= f32::EPSILON {
                            return deformed;
                        }
                        let index = usize::from(joint_index);
                        let Some(&joint) = skin.joints.get(index) else {
                            return deformed;
                        };
                        let (Ok(joint_global), Some(inverse_bindpose)) =
                            (globals.get(joint), bindposes.get(index))
                        else {
                            return deformed;
                        };
                        let joint_matrix = joint_global.affine() * *inverse_bindpose;
                        deformed + joint_matrix.transform_point3(position) * weight
                    },
                );
                actor_from_world.transform_point3(world_position)
            })
            .collect::<Vec<_>>();
        let (mean_vertex_delta, max_vertex_delta) = previous_vertices
            .get(&skin_entity)
            .filter(|previous| previous.len() == vertices.len())
            .map_or((0.0, 0.0), |previous| {
                let mut sum = 0.0;
                let mut maximum = 0.0_f32;
                for (previous, current) in previous.iter().zip(&vertices) {
                    let delta = previous.distance(*current);
                    sum += delta;
                    maximum = maximum.max(delta);
                }
                (sum / vertices.len().max(1) as f32, maximum)
            });
        let (bounds_min, bounds_max) = vertices.iter().fold(
            (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY)),
            |(minimum, maximum), &vertex| (minimum.min(vertex), maximum.max(vertex)),
        );
        info!(
            stable_id = %agent.id,
            skin = ?skin_entity,
            slot = slot_name,
            vertices = vertices.len(),
            joints = skin.joints.len(),
            mean_vertex_delta,
            max_vertex_delta,
            bounds_size = ?(bounds_max - bounds_min),
            "live post-propagation skin deformation"
        );
        previous_vertices.insert(skin_entity, vertices);
    }
    *samples += 1;
}

pub(crate) fn report_pre_animation_stage(
    players: Query<&AnimationPlayer, With<ConvertedAnimationDriver>>,
    mut samples: Local<u8>,
) {
    if animation_binding_diagnostics_enabled() && *samples < 8 {
        info!(
            sample = *samples,
            converted_players = players.iter().count(),
            active_nodes = players
                .iter()
                .map(|player| player.playing_animations().count())
                .sum::<usize>(),
            "animation stage: before Bevy animation evaluation"
        );
        *samples += 1;
    }
}

pub(crate) fn report_post_animation_stage(mut samples: Local<u8>) {
    if animation_binding_diagnostics_enabled() && *samples < 8 {
        info!(
            sample = *samples,
            "animation stage: after Bevy animation evaluation"
        );
        *samples += 1;
    }
}

pub(crate) fn is_descendant_of(
    entity: Entity,
    ancestor: Entity,
    parents: &Query<&ChildOf>,
) -> bool {
    let mut entity = entity;
    for _ in 0..64 {
        if entity == ancestor {
            return true;
        }
        let Ok(parent) = parents.get(entity) else {
            break;
        };
        entity = parent.parent();
    }
    false
}

pub(crate) fn named_character_slot_ancestor<'a>(
    entity: Entity,
    parents: &Query<&ChildOf>,
    names: &'a Query<&Name>,
) -> Option<&'a str> {
    let mut entity = entity;
    for _ in 0..8 {
        if let Ok(name) = names.get(entity)
            && player_equipment_slot_node(name.as_str())
        {
            return Some(name.as_str());
        }
        entity = parents.get(entity).ok()?.parent();
    }
    None
}

#[allow(clippy::type_complexity)]
pub(crate) fn apply_material_overrides(
    mut commands: Commands,
    specs: Query<&MaterialOverrideSpec>,
    parents: Query<&ChildOf>,
    names: Query<&Name>,
    mut renderers: Query<
        (
            Entity,
            &mut MeshMaterial3d<StandardMaterial>,
            Option<&GltfMeshName>,
            Option<&GltfMaterialName>,
        ),
        (
            Without<MaterialOverrideApplied>,
            Without<ChimneySmokeParticle>,
        ),
    >,
) {
    for (entity, mut material, mesh_name, material_name) in &mut renderers {
        let mut ancestor = entity;
        let mut path = Vec::new();
        for _ in 0..64 {
            if let Ok(spec) = specs.get(ancestor) {
                let hierarchy_path = path.iter().rev().cloned().collect::<Vec<_>>().join("/");
                let authored = resolved_renderer_material(
                    spec,
                    &hierarchy_path,
                    mesh_name.map(|name| name.0.as_str()),
                    material_name.map(|name| name.0.as_str()),
                );
                if let Some(authored) = authored {
                    match authored {
                        ResolvedMaterialHandle::Standard(authored) => {
                            material.0 = authored.clone();
                        }
                        ResolvedMaterialHandle::Building(authored) => {
                            commands
                                .entity(entity)
                                .remove::<MeshMaterial3d<StandardMaterial>>()
                                .insert(MeshMaterial3d(authored.clone()));
                        }
                        ResolvedMaterialHandle::Cloud(authored) => {
                            commands
                                .entity(entity)
                                .remove::<MeshMaterial3d<StandardMaterial>>()
                                .insert(MeshMaterial3d(authored.clone()));
                        }
                        ResolvedMaterialHandle::Godray(authored) => {
                            commands
                                .entity(entity)
                                .remove::<MeshMaterial3d<StandardMaterial>>()
                                .insert(MeshMaterial3d(authored.clone()));
                        }
                        ResolvedMaterialHandle::Giraffe(authored) => {
                            commands
                                .entity(entity)
                                .remove::<MeshMaterial3d<StandardMaterial>>()
                                .insert(MeshMaterial3d(authored.clone()));
                        }
                        ResolvedMaterialHandle::Bounds(authored) => {
                            commands
                                .entity(entity)
                                .remove::<MeshMaterial3d<StandardMaterial>>()
                                .insert(MeshMaterial3d(authored.clone()));
                        }
                        ResolvedMaterialHandle::Tree(authored) => {
                            commands
                                .entity(entity)
                                .remove::<MeshMaterial3d<StandardMaterial>>()
                                .insert(MeshMaterial3d(authored.clone()));
                        }
                        ResolvedMaterialHandle::Grass(authored) => {
                            commands
                                .entity(entity)
                                .remove::<MeshMaterial3d<StandardMaterial>>()
                                .insert(MeshMaterial3d(authored.clone()));
                        }
                        ResolvedMaterialHandle::Critter(authored) => {
                            commands
                                .entity(entity)
                                .remove::<MeshMaterial3d<StandardMaterial>>()
                                .insert(MeshMaterial3d(authored.clone()));
                        }
                        ResolvedMaterialHandle::Flag(authored) => {
                            commands
                                .entity(entity)
                                .remove::<MeshMaterial3d<StandardMaterial>>()
                                .insert(MeshMaterial3d(authored.clone()));
                        }
                        ResolvedMaterialHandle::Character(authored) => {
                            commands
                                .entity(entity)
                                .remove::<MeshMaterial3d<StandardMaterial>>()
                                .insert(MeshMaterial3d(authored.clone()));
                        }
                    }
                    if spec.suppress_self_shadows {
                        // Gate doors are skinned while their surrounding frame is static.
                        // Excluding the whole gate renderer from both shadow directions avoids
                        // coplanar animated/static depth disagreement in Bevy's shadow pass.
                        commands
                            .entity(entity)
                            .insert((bevy::light::NotShadowCaster, bevy::light::NotShadowReceiver));
                    } else if material_needs_self_shadow_suppression(authored) {
                        commands
                            .entity(entity)
                            .insert(bevy::light::NotShadowReceiver);
                    }
                }
                commands.entity(entity).insert(MaterialOverrideApplied);
                break;
            }
            if let Ok(name) = names.get(ancestor) {
                path.push(name.as_str().to_owned());
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
    }
}

pub(crate) fn resolved_renderer_material<'a>(
    spec: &'a MaterialOverrideSpec,
    hierarchy_path: &str,
    mesh_name: Option<&str>,
    material_name: Option<&str>,
) -> Option<&'a ResolvedMaterialHandle> {
    // Bevy puts each glTF primitive on a material-bearing child below its named
    // mesh node. Strip that primitive child before comparing the hierarchy to
    // the Unity renderer path, while retaining the mesh name as the stable
    // fallback for Blender-added scene roots.
    let node_path = if mesh_name.is_some() {
        hierarchy_path
            .rsplit_once('/')
            .map_or(hierarchy_path, |(path, _)| path)
    } else {
        hierarchy_path
    };
    let target_name = mesh_name
        .or_else(|| node_path.rsplit('/').next())
        .unwrap_or_default();
    let full_path_matches = |binding: &&ResolvedRendererMaterialBinding| {
        binding.target_path == node_path
            || node_path.ends_with(&format!("/{}", binding.target_path))
            || binding.target_path.ends_with(&format!("/{node_path}"))
    };
    let name_matches = |binding: &&ResolvedRendererMaterialBinding| {
        binding.target_path.rsplit('/').next() == Some(target_name)
    };
    let exact_path = material_name
        .and_then(|name| {
            spec.renderer_materials
                .iter()
                .filter(full_path_matches)
                .find(|binding| binding.materials.contains_key(name))
        })
        .or_else(|| spec.renderer_materials.iter().find(full_path_matches));
    let exact = exact_path.or_else(|| {
        material_name
            .and_then(|name| {
                spec.renderer_materials
                    .iter()
                    .filter(name_matches)
                    .find(|binding| binding.materials.contains_key(name))
            })
            .or_else(|| spec.renderer_materials.iter().find(name_matches))
    });
    material_name
        .and_then(|name| exact.and_then(|binding| binding.materials.get(name)))
        .or_else(|| material_name.and_then(|name| spec.model_materials.get(name)))
        .or_else(|| exact.and_then(|binding| binding.materials.values().next()))
        .or(spec.fallback.as_ref())
}

pub(crate) fn instantiate_building_materials(
    mut commands: Commands,
    simulation: Res<SimulationRuntime>,
    config: Res<RuntimeConfig>,
    time: Res<Time>,
    content: Res<RuntimeContent>,
    parents: Query<&ChildOf>,
    buildings: Query<&RuntimeBuilding>,
    mut instances: ResMut<BuildingMaterialInstances>,
    mut materials: Option<ResMut<Assets<BuildingMaterial>>>,
    renderers: Query<
        (Entity, &MeshMaterial3d<BuildingMaterial>),
        Without<BuildingMaterialInstanced>,
    >,
) {
    let Some(materials) = materials.as_deref_mut() else {
        return;
    };
    for (entity, source) in &renderers {
        let mut ancestor = entity;
        let mut building = None;
        for _ in 0..64 {
            if let Ok(runtime) = buildings.get(ancestor) {
                building = Some(runtime.id.clone());
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
        let Some(building) = building else {
            commands.entity(entity).insert(BuildingMaterialInstanced);
            continue;
        };
        let handle =
            if let Some(instance) = instances.0.get(&building) {
                instance.handle.clone()
            } else {
                let Some(mut material) = materials.get(&source.0).cloned() else {
                    continue;
                };
                let health = simulation
                    .0
                    .buildings
                    .get(&building)
                    .map_or(BUILDING_MAX_HEALTH, |state| state.health);
                let season = simulation.0.season;
                let (season_from, season_to, season_blend) = season_visual_blend(
                    simulation.0.elapsed_seconds,
                    config.0.time.seconds_per_day,
                    season,
                );
                let snow = building_snow_strength(season_from)
                    + (building_snow_strength(season_to) - building_snow_strength(season_from))
                        * season_blend;
                material.extension.parameters.snow_damage.x = snow;
                material.extension.parameters.snow_damage.y = snow;
                let max_health = simulation
                    .0
                    .buildings
                    .get(&building)
                    .map_or(BUILDING_MAX_HEALTH, |state| {
                        building_max_health(&content.0, state)
                    });
                material.extension.parameters.snow_damage.z =
                    building_damage_value(health, max_health);
                if simulation.0.buildings.get(&building).is_some_and(|state| {
                    state.archetype.as_str() == "archetype:building:guardhouse"
                }) {
                    material.extension.parameters.tint_color_strength =
                        Vec4::new(0.22, 0.62, 1.0, 0.72);
                }
                material.extension.parameters.surface_controls.z =
                    f32::from(config.0.time.max_building_emission_milli) / 1_000.0;
                material.extension.parameters.time_cycle = building_material_time_cycle(
                    &config.0.time,
                    simulation.0.elapsed_seconds,
                    time.elapsed_secs_wrapped_f64(),
                );
                let handle = materials.add(material);
                instances.0.insert(
                    building,
                    BuildingMaterialInstance {
                        handle: handle.clone(),
                        applied_health: health,
                        applied_season: season,
                        applied_season_blend_bits: daylight_signature(season_blend),
                        applied_time_cycle: BuildingTimeCycleSignature::from(&config.0.time),
                    },
                );
                handle
            };
        commands
            .entity(entity)
            .insert((MeshMaterial3d(handle), BuildingMaterialInstanced));
    }
}

pub(crate) fn sync_building_material_instances(
    simulation: Res<SimulationRuntime>,
    config: Res<RuntimeConfig>,
    time: Res<Time>,
    content: Res<RuntimeContent>,
    mut instances: ResMut<BuildingMaterialInstances>,
    mut update_runtime: ResMut<BuildingMaterialUpdateRuntime>,
    mut materials: Option<ResMut<Assets<BuildingMaterial>>>,
) {
    let Some(materials) = materials.as_deref_mut() else {
        return;
    };
    let removed: Vec<_> = instances
        .0
        .iter()
        .filter(|(id, _)| !simulation.0.buildings.contains_key(*id))
        .map(|(id, instance)| (id.clone(), instance.handle.id()))
        .collect();
    for (id, handle) in removed {
        materials.remove(handle);
        instances.0.remove(&id);
    }
    if update_runtime.order.len() != instances.0.len()
        || update_runtime
            .order
            .iter()
            .any(|id| !instances.0.contains_key(id))
    {
        update_runtime.order.clear();
        update_runtime.order.extend(instances.0.keys().cloned());
        update_runtime.next_index = update_runtime
            .next_index
            .min(update_runtime.order.len().saturating_sub(1));
    }
    if update_runtime.order.is_empty() {
        update_runtime.next_index = 0;
        return;
    }

    let mut visited = 0_usize;
    let mut updated = 0_usize;
    for index in round_robin_indices(update_runtime.order.len(), update_runtime.next_index) {
        if updated >= MAX_BUILDING_MATERIAL_UPDATES_PER_FRAME {
            break;
        }
        let id = &update_runtime.order[index];
        visited += 1;
        let Some(instance) = instances.0.get_mut(id) else {
            continue;
        };
        let Some(building) = simulation.0.buildings.get(id) else {
            continue;
        };
        let (season_from, season_to, season_blend) = season_visual_blend(
            simulation.0.elapsed_seconds,
            config.0.time.seconds_per_day,
            simulation.0.season,
        );
        let season_blend_bits = daylight_signature(season_blend);
        let time_cycle_signature = BuildingTimeCycleSignature::from(&config.0.time);
        if instance.applied_health == building.health
            && instance.applied_season == simulation.0.season
            && instance.applied_season_blend_bits == season_blend_bits
            && instance.applied_time_cycle == time_cycle_signature
        {
            continue;
        }
        let Some(mut material) = materials.get_mut(&instance.handle) else {
            continue;
        };
        let snow = building_snow_strength(season_from)
            + (building_snow_strength(season_to) - building_snow_strength(season_from))
                * season_blend;
        material.extension.parameters.snow_damage.x = snow;
        material.extension.parameters.snow_damage.y = snow;
        if instance.applied_time_cycle != time_cycle_signature {
            material.extension.parameters.surface_controls.z =
                f32::from(config.0.time.max_building_emission_milli) / 1_000.0;
            material.extension.parameters.time_cycle = building_material_time_cycle(
                &config.0.time,
                simulation.0.elapsed_seconds,
                time.elapsed_secs_wrapped_f64(),
            );
        }
        material.extension.parameters.snow_damage.z =
            building_damage_value(building.health, building_max_health(&content.0, building));
        instance.applied_health = building.health;
        instance.applied_season = simulation.0.season;
        instance.applied_season_blend_bits = season_blend_bits;
        instance.applied_time_cycle = time_cycle_signature;
        updated += 1;
    }
    update_runtime.next_index = (update_runtime.next_index + visited) % update_runtime.order.len();
}

pub(crate) fn building_damage_value(health: i32, max_health: i32) -> f32 {
    if max_health <= 0 {
        return 0.0;
    }
    let bounded_max = u16::try_from(max_health).unwrap_or(u16::MAX).max(1);
    let bounded_health = u16::try_from(health.clamp(0, i32::from(bounded_max))).unwrap_or_default();
    f32::from(bounded_health) / f32::from(bounded_max)
}

pub(crate) fn foliage_capture_camera(
    starting_camera: Transform,
    elapsed_seconds: f32,
) -> Transform {
    if elapsed_seconds <= 1.0 {
        return starting_camera;
    }
    let progress = ((elapsed_seconds - 1.0) / 5.0).clamp(0.0, 1.0);
    let focus = starting_camera.translation + starting_camera.forward() * 55.0;
    let initial_offset = starting_camera.translation - focus;
    let orbit_radians = (progress * std::f32::consts::TAU).sin() * 0.52;
    let distance_scale = 1.0 - (progress * std::f32::consts::PI).sin().powi(2) * 0.38;
    let vertical_lift = (progress * std::f32::consts::TAU).sin() * 4.0;
    let translation = focus
        + Quat::from_rotation_y(orbit_radians) * initial_offset * distance_scale
        + Vec3::Y * vertical_lift;
    Transform::from_translation(translation).looking_at(focus, Vec3::Y)
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) fn quantized_render_transform(transform: &GlobalTransform) -> [i64; 10] {
    let (scale, rotation, translation) = transform.to_scale_rotation_translation();
    let rotation = rotation.to_array();
    let quantize = |value: f32| (value * 10_000.0).round() as i64;
    [
        quantize(translation.x),
        quantize(translation.y),
        quantize(translation.z),
        quantize(rotation[0]),
        quantize(rotation[1]),
        quantize(rotation[2]),
        quantize(rotation[3]),
        quantize(scale.x),
        quantize(scale.y),
        quantize(scale.z),
    ]
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn capture_foliage_acceptance(
    mut commands: Commands,
    time: Res<Time>,
    reveal: Option<Res<WorldRevealRuntime>>,
    config: Option<Res<RuntimeConfig>>,
    world: Option<Res<WorldRuntime>>,
    meshes: Option<Res<Assets<Mesh>>>,
    mut cameras: Query<(&mut Transform, &mut TownCameraControllerRuntime), With<TownCamera>>,
    renderers: Query<
        (
            Entity,
            &Mesh3d,
            &GlobalTransform,
            Option<&ResourceNode>,
            Option<&FoliageVisual>,
            Option<&SurfaceFoliageHabitat>,
            Option<&PendingSurfaceGrounding>,
            Option<&bevy::light::NotShadowCaster>,
            Option<&bevy::light::NotShadowReceiver>,
        ),
        Or<(With<ResourceNode>, With<FoliageVisual>)>,
    >,
    active_screenshots: Query<(), With<FoliageAcceptanceScreenshot>>,
    mut capture: Local<FoliageAcceptanceCapture>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(output_directory) =
        std::env::var_os("STREAM_TOWN_FOLIAGE_CAPTURE_DIR").map(PathBuf::from)
    else {
        return;
    };
    let (Some(config), Some(world), Some(meshes)) =
        (config.as_deref(), world.as_deref(), meshes.as_deref())
    else {
        return;
    };
    if reveal.is_some() {
        return;
    }
    if !active_screenshots.is_empty() {
        return;
    }
    if capture.capture_cooldown_seconds > 0.0 {
        capture.capture_cooldown_seconds -= time.delta_secs();
        return;
    }
    if let Some(delay) = capture.completion_delay_seconds.as_mut() {
        *delay -= time.delta_secs();
        if *delay <= 0.0 {
            exit.write(AppExit::Success);
            capture.completion_delay_seconds = None;
        }
        return;
    }
    let Ok((mut camera, mut controller)) = cameras.single_mut() else {
        return;
    };
    if !capture.initialized {
        capture.initialized = true;
        capture.configured_seed = config.0.world.seed;
        capture.generated_seed = world.generated.seed;
        capture.starting_camera = Some(*camera);
        capture.output_directory = Some(output_directory.clone());
        if let Err(error) = std::fs::create_dir_all(&output_directory) {
            error!(%error, path = %output_directory.display(), "could not create foliage capture directory");
            return;
        }

        let mut renderer_groups: HashMap<String, Vec<String>> = HashMap::new();
        for (
            entity,
            mesh,
            global,
            resource,
            foliage,
            habitat,
            pending_grounding,
            shadow_caster_disabled,
            shadow_receiver_disabled,
        ) in &renderers
        {
            capture.renderer_count += 1;
            capture.shadow_caster_count += usize::from(shadow_caster_disabled.is_none());
            capture.shadow_receiver_count += usize::from(shadow_receiver_disabled.is_none());
            capture.pending_grounding_count += usize::from(pending_grounding.is_some());
            let (scale, rotation, translation) = global.to_scale_rotation_translation();
            let audit_transform = Transform {
                translation,
                rotation,
                scale,
            };
            if let (Some(habitat), Some(mesh_asset)) = (habitat, meshes.get(&mesh.0))
                && let Some(vertical_extent) = transformed_mesh_vertical_extent(
                    mesh_asset,
                    audit_transform.rotation,
                    audit_transform.scale,
                )
                && !foliage_mesh_fits_habitat(
                    mesh_asset,
                    &audit_transform,
                    vertical_extent,
                    habitat.0,
                    &config.0,
                    &world.generated,
                )
            {
                capture.habitat_violation_count += 1;
            }
            let transform = quantized_render_transform(global);
            let key = format!("{:?}|{transform:?}", mesh.0.id());
            let label = resource.map_or_else(
                || {
                    foliage.map_or_else(
                        || format!("unknown:{entity:?}"),
                        |foliage| foliage.0.to_string(),
                    )
                },
                |resource| resource.id.to_string(),
            );
            renderer_groups.entry(key).or_default().push(label);
        }
        let mut duplicate_groups = renderer_groups
            .into_iter()
            .filter_map(|(key, entities)| {
                (entities.len() > 1).then(|| {
                    serde_json::json!({
                        "mesh_and_transform": key,
                        "renderers": entities,
                    })
                })
            })
            .collect::<Vec<_>>();
        duplicate_groups.sort_by(|left, right| {
            left["mesh_and_transform"]
                .as_str()
                .cmp(&right["mesh_and_transform"].as_str())
        });
        capture.duplicate_group_count = duplicate_groups.len();
        duplicate_groups.truncate(64);
        capture.duplicate_groups = duplicate_groups;
        info!(
            renderers = capture.renderer_count,
            shadow_casters = capture.shadow_caster_count,
            shadow_receivers = capture.shadow_receiver_count,
            pending_grounding = capture.pending_grounding_count,
            habitat_violations = capture.habitat_violation_count,
            configured_seed = capture.configured_seed,
            generated_seed = capture.generated_seed,
            duplicate_groups = capture.duplicate_group_count,
            "foliage acceptance renderer audit complete"
        );
    }

    capture.elapsed_seconds += time.delta_secs();
    let starting_camera = capture.starting_camera.unwrap_or(*camera);
    let sampled_camera = foliage_capture_camera(starting_camera, capture.elapsed_seconds);
    *camera = sampled_camera;
    controller.move_target = sampled_camera.translation;
    controller.zoom_target_height = sampled_camera.translation.y;

    // Screenshot readback is asynchronous. Do not enqueue the next primary
    // window screenshot until the previous observer has actually written its
    // PNG; overlapping requests can target the same surface and silently skip
    // one of the promised acceptance frames.
    if capture.next_capture > capture.written_captures {
        let previous =
            output_directory.join(format!("foliage-sweep-{:02}.png", capture.next_capture - 1));
        if !previous.is_file() {
            return;
        }
        // Bevy marks the completed Screenshot entity for cleanup in the same
        // frame that its observer writes the file. Its extracted render-world
        // target can outlive the main-world entity for a few more frames, so
        // leave a short deterministic gap before targeting the surface again.
        capture.written_captures = capture.next_capture;
        capture.capture_cooldown_seconds = 0.75;
        return;
    }
    let Some(&capture_time) = FOLIAGE_CAPTURE_TIMES_SECONDS.get(capture.next_capture) else {
        let structural_passed = capture.duplicate_group_count == 0
            && capture.shadow_caster_count == capture.renderer_count
            && capture.pending_grounding_count == 0
            && capture.habitat_violation_count == 0
            && capture.configured_seed == capture.generated_seed;
        let manifest = serde_json::json!({
            "schema_version": 1,
            "mode": "fixed-seed moving-camera foliage acceptance",
            "visual_review_required": true,
            "structural_passed": structural_passed,
            "renderer_audit": {
                "resource_and_foliage_renderers": capture.renderer_count,
                "shadow_casters": capture.shadow_caster_count,
                "shadow_receivers": capture.shadow_receiver_count,
                "pending_surface_grounding": capture.pending_grounding_count,
                "final_height_habitat_violations": capture.habitat_violation_count,
                "configured_world_seed": capture.configured_seed,
                "generated_world_seed": capture.generated_seed,
                "world_seed_consistent": capture.configured_seed == capture.generated_seed,
                "duplicate_mesh_transform_group_count": capture.duplicate_group_count,
                "duplicate_mesh_transform_group_examples": capture.duplicate_groups,
            },
            "frames": capture.frames,
        });
        let manifest_path = output_directory.join("foliage-sweep-manifest.json");
        let result = serde_json::to_vec_pretty(&manifest)
            .map_err(anyhow::Error::from)
            .and_then(|bytes| std::fs::write(&manifest_path, bytes).map_err(anyhow::Error::from));
        if let Err(error) = result {
            error!(%error, path = %manifest_path.display(), "could not write foliage acceptance manifest");
        } else {
            info!(
                structural_passed,
                path = %manifest_path.display(),
                "foliage acceptance capture complete"
            );
        }
        capture.completion_delay_seconds = Some(0.5);
        return;
    };
    if capture.elapsed_seconds < capture_time {
        return;
    }
    let frame_number = capture.next_capture;
    let path = output_directory.join(format!("foliage-sweep-{frame_number:02}.png"));
    commands
        .spawn((Screenshot::primary_window(), FoliageAcceptanceScreenshot))
        .observe(save_to_disk(path.clone()));
    let captured_seconds = capture.elapsed_seconds;
    capture.frames.push(serde_json::json!({
        "frame": frame_number,
        "scheduled_seconds": capture_time,
        "captured_seconds": captured_seconds,
        "path": path.file_name().map_or_else(String::new, |name| name.to_string_lossy().into_owned()),
        "camera_translation": sampled_camera.translation.to_array(),
        "camera_rotation": sampled_camera.rotation.to_array(),
    }));
    capture.next_capture += 1;
    info!(
        frame = frame_number,
        elapsed_seconds = capture.elapsed_seconds,
        path = %path.display(),
        "capturing foliage acceptance frame"
    );
}

pub(crate) fn next_auto_camera_shot(
    world_seed: u64,
    sequence: u64,
    citizens: &[StableId],
    previous: &AutoCameraShot,
) -> AutoCameraShot {
    if citizens.is_empty() || sequence.is_multiple_of(AUTO_CAMERA_TOWN_SHOT_INTERVAL) {
        return AutoCameraShot::Town;
    }
    let mut index = usize::try_from(
        seagull_hash(
            world_seed ^ 0x6175_746f_5f63_616d,
            sequence,
            0x6369_7469_7a65_6e73,
        ) % u64::try_from(citizens.len()).expect("citizen count fits u64"),
    )
    .expect("camera choice fits usize");
    if citizens.len() > 1
        && matches!(previous, AutoCameraShot::Citizen(previous) if previous == &citizens[index])
    {
        index = (index + 1) % citizens.len();
    }
    AutoCameraShot::Citizen(citizens[index].clone())
}

pub(crate) fn auto_camera_focus_translation(
    home: &Transform,
    target: Vec3,
    camera_height: f32,
    focus_height: f32,
) -> Vec3 {
    let focus = target + Vec3::Y * focus_height;
    let forward = home.forward().as_vec3();
    let height = (target.y + camera_height)
        .clamp(UNITY_TOWN_CAMERA_MIN_HEIGHT, UNITY_TOWN_CAMERA_MAX_HEIGHT);
    let distance = (height - focus.y) / (-forward.y).max(0.001);
    focus - forward * distance
}

pub(crate) fn auto_camera_citizen_translation(home: &Transform, citizen: Vec3) -> Vec3 {
    auto_camera_focus_translation(
        home,
        citizen,
        AUTO_CAMERA_CITIZEN_HEIGHT,
        AUTO_CAMERA_CITIZEN_FOCUS_HEIGHT,
    )
}

pub(crate) fn frame_independent_lerp_factor(smoothing: f32, delta_seconds: f32) -> f32 {
    1.0 - (-smoothing.max(0.0) * delta_seconds.max(0.0)).exp()
}

pub(crate) fn track_camera_damage_focus(
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    mut damage: ResMut<CameraDamageRuntime>,
) {
    let actor_health = simulation
        .0
        .actors
        .iter()
        .map(|(id, actor)| (id.clone(), actor.health))
        .collect::<BTreeMap<_, _>>();
    let building_health = simulation
        .0
        .buildings
        .iter()
        .map(|(id, building)| (id.clone(), building.health))
        .collect::<BTreeMap<_, _>>();
    if !damage.initialized {
        damage.actor_health = actor_health;
        damage.building_health = building_health;
        damage.initialized = true;
        return;
    }

    let town_hall = StableId::new("building:townhall").expect("static building ID");
    let town_hall_position = simulation
        .0
        .buildings
        .get(&town_hall)
        .map_or(GridPos { x: 0, z: 0 }, |building| {
            building_visual_grid(&content.0, building)
        });
    let mut candidates = Vec::new();
    for (id, actor) in &simulation.0.actors {
        if actor.role.as_str() != "role:enemy"
            && damage
                .actor_health
                .get(id)
                .is_some_and(|previous| actor.health < *previous)
        {
            candidates.push((
                grid_distance_squared(actor.position, town_hall_position),
                id.clone(),
                CameraFocusTarget::Citizen(id.clone()),
            ));
        }
    }
    for (id, building) in &simulation.0.buildings {
        if damage
            .building_health
            .get(id)
            .is_some_and(|previous| building.health < *previous)
        {
            candidates.push((
                grid_distance_squared(
                    building_visual_grid(&content.0, building),
                    town_hall_position,
                ),
                id.clone(),
                CameraFocusTarget::Building(id.clone()),
            ));
        }
    }
    candidates.sort_by(|left, right| (left.0, &left.1).cmp(&(right.0, &right.1)));
    if let Some((_, _, target)) = candidates.into_iter().next() {
        damage.latest_target = Some(target);
        damage.sequence = damage.sequence.saturating_add(1);
    }
    damage.actor_health = actor_health;
    damage.building_health = building_health;
}

pub(crate) fn reset_gameplay_attention_runtime(
    mut damage: ResMut<CameraDamageRuntime>,
    mut retreating: ResMut<RetreatingCitizens>,
) {
    *damage = CameraDamageRuntime::default();
    retreating.0.clear();
}

pub(crate) fn camera_zoom_and_commands(
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    menu: Res<MenuRuntime>,
    settings: Res<RuntimePlayerSettings>,
    simulation: Option<Res<SimulationRuntime>>,
    acknowledgements: Res<CommandAcknowledgementRuntime>,
    damage: Res<CameraDamageRuntime>,
    mut requests: ResMut<CameraCommandQueue>,
    agents: Query<(&Agent, &Transform), Without<TownCamera>>,
    buildings: Query<(&RuntimeBuilding, &Transform), Without<TownCamera>>,
    mut cameras: Query<
        (
            &mut Transform,
            &mut Projection,
            &mut TownCameraControllerRuntime,
        ),
        With<TownCamera>,
    >,
) {
    if menu.page != MenuPage::Closed {
        return;
    }
    let Ok((mut transform, mut projection, mut controller)) = cameras.single_mut() else {
        return;
    };
    // Keep deterministic close-up diagnostics pinned to their authored focus.
    if std::env::var_os("STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP").is_some()
        || std::env::var_os("STREAM_TOWN_FOLIAGE_CAPTURE_DIR").is_some()
    {
        controller.move_target = transform.translation;
        return;
    }

    let delta_seconds = time.delta_secs();
    controller.combat_redirect_cooldown_seconds =
        (controller.combat_redirect_cooldown_seconds - delta_seconds).max(0.0);
    if let Projection::Perspective(perspective) = &mut *projection {
        perspective.fov = f32::from(settings.0.camera.field_of_view_degrees).to_radians();
    }

    let command_acknowledged =
        controller.observed_command_acknowledgements != acknowledgements.sequence;
    if command_acknowledged {
        controller.observed_command_acknowledgements = acknowledgements.sequence;
        controller.seconds_since_acknowledgement = 0.0;
        if controller.auto_shot != AutoCameraShot::Inactive {
            controller.return_home();
        }
        controller.cancel_auto_camera();
        controller.temporary_focus = None;
    }

    let damage_changed = controller.observed_damage_sequence != damage.sequence;
    if damage_changed {
        controller.observed_damage_sequence = damage.sequence;
        if controller.combat_redirect_cooldown_seconds <= f32::EPSILON
            && auto_camera_is_managed(&controller)
            && let Some(target) = damage.latest_target.clone()
        {
            controller.temporary_focus = Some(TemporaryCameraFocus {
                target,
                remaining_seconds: AUTO_CAMERA_ATTENTION_SECONDS,
                return_to_auto_camera: true,
            });
            controller.combat_redirect_cooldown_seconds =
                AUTO_CAMERA_COMBAT_REDIRECT_COOLDOWN_SECONDS;
        }
    }

    if let Some(request) = requests.0.pop_front() {
        if let Some(building) = request.focus_building {
            let return_to_auto_camera = auto_camera_is_managed(&controller);
            controller.temporary_focus = Some(TemporaryCameraFocus {
                target: CameraFocusTarget::Building(building),
                remaining_seconds: AUTO_CAMERA_ATTENTION_SECONDS,
                return_to_auto_camera,
            });
        } else if let Some(citizen) = request.follow {
            let valid_target = auto_camera_is_managed(&controller)
                && is_stream_player_actor(&citizen)
                && simulation.as_ref().is_some_and(|simulation| {
                    simulation
                        .0
                        .actors
                        .get(&citizen)
                        .is_some_and(|actor| actor.alive && is_stream_player_actor(&actor.id))
                });
            if valid_target
                && let Some((_, citizen_transform)) =
                    agents.iter().find(|(agent, _)| agent.id == citizen)
            {
                controller.auto_shot = AutoCameraShot::Citizen(citizen);
                controller.auto_shot_elapsed_seconds = 0.0;
                controller.move_target = constrain_town_camera_position(
                    auto_camera_citizen_translation(
                        &controller.home,
                        citizen_transform.translation,
                    ),
                    &config.0.world,
                );
                controller.zoom_target_height = controller.move_target.y;
            }
        } else {
            controller.seconds_since_acknowledgement = 0.0;
            controller.cancel_auto_camera();
            controller.temporary_focus = None;
            if request.reset {
                controller.return_home();
            } else {
                for action in request.actions {
                    let amount =
                        i16::try_from(action.amount.clamp(-100, 100)).map_or(0.0, f32::from);
                    match action.direction {
                        CameraDirection::Up
                        | CameraDirection::Down
                        | CameraDirection::Left
                        | CameraDirection::Right => {
                            let screen = match action.direction {
                                CameraDirection::Up => Vec2::Y,
                                CameraDirection::Down => Vec2::NEG_Y,
                                CameraDirection::Left => Vec2::NEG_X,
                                CameraDirection::Right => Vec2::X,
                                CameraDirection::In | CameraDirection::Out => unreachable!(),
                            };
                            let direction = unity_camera_world_direction(screen);
                            controller.move_target += Vec3::new(direction.x, 0.0, direction.y)
                                * amount
                                * twitch_camera_pan_distance(action.direction);
                        }
                        CameraDirection::In | CameraDirection::Out => {
                            let signed = if action.direction == CameraDirection::In {
                                -amount
                            } else {
                                amount
                            };
                            controller.zoom_target_height = (controller.zoom_target_height
                                + signed)
                                .clamp(UNITY_TOWN_CAMERA_MIN_HEIGHT, UNITY_TOWN_CAMERA_MAX_HEIGHT);
                        }
                    }
                }
            }
            controller.move_target.y = controller.zoom_target_height;
            controller.move_target =
                constrain_town_camera_position(controller.move_target, &config.0.world);
        }
    } else {
        let was_idle = controller.seconds_since_acknowledgement >= AUTO_CAMERA_IDLE_SECONDS;
        if !command_acknowledged {
            controller.seconds_since_acknowledgement = (controller.seconds_since_acknowledgement
                + delta_seconds)
                .min(AUTO_CAMERA_IDLE_SECONDS);
        }
        let entered_auto_camera =
            !was_idle && controller.seconds_since_acknowledgement >= AUTO_CAMERA_IDLE_SECONDS;
        if entered_auto_camera {
            controller.auto_shot = AutoCameraShot::Town;
            controller.auto_shot_elapsed_seconds = 0.0;
            controller.return_home();
        }
        if controller.seconds_since_acknowledgement >= AUTO_CAMERA_IDLE_SECONDS {
            let mut citizens = agents
                .iter()
                .filter(|(agent, _)| {
                    agent.kind == ActorKind::Player && is_stream_player_actor(&agent.id)
                })
                .filter(|(agent, _)| {
                    simulation.as_ref().is_none_or(|simulation| {
                        simulation
                            .0
                            .actors
                            .get(&agent.id)
                            .is_some_and(|actor| actor.alive)
                    })
                })
                .map(|(agent, _)| agent.id.clone())
                .collect::<Vec<_>>();
            citizens.sort();
            if controller.auto_shot == AutoCameraShot::Inactive {
                controller.auto_shot = AutoCameraShot::Town;
                controller.auto_shot_elapsed_seconds = 0.0;
                controller.return_home();
            } else if !entered_auto_camera {
                controller.auto_shot_elapsed_seconds += delta_seconds;
            }
            let duration = match controller.auto_shot {
                AutoCameraShot::Inactive | AutoCameraShot::Town => AUTO_CAMERA_TOWN_SHOT_SECONDS,
                AutoCameraShot::Citizen(_) => AUTO_CAMERA_CITIZEN_SHOT_SECONDS,
            };
            if controller.auto_shot_elapsed_seconds >= duration {
                let previous = controller.auto_shot.clone();
                controller.auto_sequence = controller.auto_sequence.saturating_add(1);
                controller.auto_shot = next_auto_camera_shot(
                    simulation
                        .as_ref()
                        .map_or(0, |simulation| simulation.0.world_seed),
                    controller.auto_sequence,
                    &citizens,
                    &previous,
                );
                controller.auto_shot_elapsed_seconds = 0.0;
            }
            match controller.auto_shot.clone() {
                AutoCameraShot::Inactive => {}
                AutoCameraShot::Town => controller.return_home(),
                AutoCameraShot::Citizen(citizen) => {
                    if citizens.binary_search(&citizen).is_ok()
                        && let Some((_, citizen_transform)) =
                            agents.iter().find(|(agent, _)| agent.id == citizen)
                    {
                        controller.move_target = constrain_town_camera_position(
                            auto_camera_citizen_translation(
                                &controller.home,
                                citizen_transform.translation,
                            ),
                            &config.0.world,
                        );
                        controller.zoom_target_height = controller.move_target.y;
                    } else {
                        controller.auto_shot_elapsed_seconds = AUTO_CAMERA_CITIZEN_SHOT_SECONDS;
                    }
                }
            }
        }
    }

    let focus_target = controller
        .temporary_focus
        .as_ref()
        .map(|focus| focus.target.clone());
    if let Some(focus_target) = focus_target {
        let target = match &focus_target {
            CameraFocusTarget::Citizen(citizen) => agents
                .iter()
                .find(|(agent, _)| &agent.id == citizen)
                .map(|(_, transform)| {
                    auto_camera_citizen_translation(&controller.home, transform.translation)
                }),
            CameraFocusTarget::Building(building) => buildings
                .iter()
                .find(|(runtime, _)| &runtime.id == building)
                .map(|(_, transform)| {
                    auto_camera_focus_translation(
                        &controller.home,
                        transform.translation,
                        AUTO_CAMERA_BUILDING_HEIGHT,
                        AUTO_CAMERA_BUILDING_FOCUS_HEIGHT,
                    )
                }),
        };
        if let Some(target) = target {
            controller.move_target = constrain_town_camera_position(target, &config.0.world);
            controller.zoom_target_height = controller.move_target.y;
            if let Some(focus) = controller.temporary_focus.as_mut() {
                focus.remaining_seconds = (focus.remaining_seconds - delta_seconds).max(0.0);
            }
        } else if let Some(focus) = controller.temporary_focus.as_mut() {
            focus.remaining_seconds = 0.0;
        }
        let expired = controller
            .temporary_focus
            .as_ref()
            .is_some_and(|focus| focus.remaining_seconds <= f32::EPSILON);
        if expired {
            let return_to_auto_camera = controller
                .temporary_focus
                .take()
                .is_some_and(|focus| focus.return_to_auto_camera);
            controller.return_home();
            if return_to_auto_camera {
                controller.auto_shot = AutoCameraShot::Town;
                controller.auto_shot_elapsed_seconds = 0.0;
            } else {
                controller.cancel_auto_camera();
            }
        }
    }

    let smoothing = frame_independent_lerp_factor(UNITY_TOWN_CAMERA_MOVE_SMOOTHNESS, delta_seconds);
    let target = Vec3::new(
        controller.move_target.x,
        controller.zoom_target_height,
        controller.move_target.z,
    );
    transform.translation = transform.translation.lerp(target, smoothing);
    transform.rotation = transform
        .rotation
        .slerp(controller.home.rotation, smoothing);
}

#[allow(clippy::type_complexity)]
pub(crate) fn follow_animation_closeup_camera(
    agents: Query<(&Agent, &Transform), Without<TownCamera>>,
    mut cameras: Query<
        (&mut Transform, &mut TownCameraControllerRuntime),
        (With<TownCamera>, Without<Agent>),
    >,
) {
    if std::env::var_os("STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP").is_none() {
        return;
    }
    let Some((_, actor)) = agents
        .iter()
        .find(|(agent, _)| agent.id.as_str() == "npc:starting_defender")
    else {
        return;
    };
    let Ok((mut camera, mut controller)) = cameras.single_mut() else {
        return;
    };
    let focus = actor.translation;
    let transform = Transform::from_xyz(focus.x + 7.0, focus.y + 6.0, focus.z + 7.0)
        .looking_at(focus + Vec3::Y * 1.6, Vec3::Y);
    *camera = transform;
    controller.set_home(transform);
}

#[allow(clippy::type_complexity)]
pub(crate) fn follow_pet_closeup_camera(
    agents: Query<(&Agent, &Transform), Without<TownCamera>>,
    pets: Query<(&ActivePetVisual, &Transform), Without<TownCamera>>,
    mut cameras: Query<
        (&mut Transform, &mut TownCameraControllerRuntime),
        (With<TownCamera>, Without<Agent>, Without<ActivePetVisual>),
    >,
) {
    if std::env::var_os("STREAM_TOWN_SMOKE_PET").is_none() {
        return;
    }
    let Some((pet, pet_transform)) = pets.iter().next() else {
        return;
    };
    let Some((_, owner_transform)) = agents.iter().find(|(agent, _)| agent.id == pet.owner) else {
        return;
    };
    let Ok((mut camera, mut controller)) = cameras.single_mut() else {
        return;
    };
    let focus = owner_transform
        .translation
        .lerp(pet_transform.translation, 0.5);
    let transform = Transform::from_translation(focus + Vec3::new(7.0, 6.0, 7.0))
        .looking_at(focus + Vec3::Y * 1.8, Vec3::Y);
    *camera = transform;
    controller.set_home(transform);
}

pub(crate) fn unity_camera_world_direction(screen_direction: Vec2) -> Vec2 {
    Vec2::new(screen_direction.y, screen_direction.x)
}

pub(crate) const fn twitch_camera_pan_distance(direction: CameraDirection) -> f32 {
    match direction {
        CameraDirection::Left | CameraDirection::Right => TWITCH_CAMERA_HORIZONTAL_PAN_DISTANCE,
        CameraDirection::Up | CameraDirection::Down => TWITCH_CAMERA_VERTICAL_PAN_DISTANCE,
        CameraDirection::In | CameraDirection::Out => 0.0,
    }
}

pub(crate) fn constrain_town_camera_position(
    position: Vec3,
    world: &stream_town_domain::WorldGenConfig,
) -> Vec3 {
    let half_x = f32::from(world.width) * world.cell_size * 0.5;
    let half_z = f32::from(world.height) * world.cell_size * 0.5;
    // This camera looks down +X at 45 degrees, so its terrain focus is one
    // camera-height ahead of its body. Clamp the focus to the generated map;
    // fixed body bounds clipped the far shore increasingly as zoom increased.
    let minimum_x = -half_x - position.y;
    let maximum_x = half_x - position.y;
    Vec3::new(
        position.x.clamp(minimum_x, maximum_x),
        position.y,
        position.z.clamp(-half_z, half_z),
    )
}

pub(crate) fn camera_ground_focus(transform: &Transform) -> Vec2 {
    let forward = transform.forward();
    let intersection = if forward.y.abs() > 0.001 {
        transform.translation + forward * (-transform.translation.y / forward.y)
    } else {
        transform.translation
    };
    Vec2::new(intersection.x, intersection.z)
}

pub(crate) fn terrain_lod_for_distance(current: TerrainLodLevel, distance: f32) -> TerrainLodLevel {
    match current {
        TerrainLodLevel::High => {
            if distance > TERRAIN_MEDIUM_DETAIL_RADIUS + TERRAIN_LOD_HYSTERESIS {
                TerrainLodLevel::Low
            } else if distance > TERRAIN_HIGH_DETAIL_RADIUS + TERRAIN_LOD_HYSTERESIS {
                TerrainLodLevel::Medium
            } else {
                TerrainLodLevel::High
            }
        }
        TerrainLodLevel::Medium => {
            if distance < TERRAIN_HIGH_DETAIL_RADIUS - TERRAIN_LOD_HYSTERESIS {
                TerrainLodLevel::High
            } else if distance > TERRAIN_MEDIUM_DETAIL_RADIUS + TERRAIN_LOD_HYSTERESIS {
                TerrainLodLevel::Low
            } else {
                TerrainLodLevel::Medium
            }
        }
        TerrainLodLevel::Low => {
            if distance < TERRAIN_HIGH_DETAIL_RADIUS - TERRAIN_LOD_HYSTERESIS {
                TerrainLodLevel::High
            } else if distance < TERRAIN_MEDIUM_DETAIL_RADIUS - TERRAIN_LOD_HYSTERESIS {
                TerrainLodLevel::Medium
            } else {
                TerrainLodLevel::Low
            }
        }
    }
}
