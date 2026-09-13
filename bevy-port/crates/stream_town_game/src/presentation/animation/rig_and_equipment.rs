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
