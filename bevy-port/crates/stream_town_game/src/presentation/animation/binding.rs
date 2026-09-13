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
