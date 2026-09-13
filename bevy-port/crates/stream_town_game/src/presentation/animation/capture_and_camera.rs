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
