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
