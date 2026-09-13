fn cleanup_credits(mut commands: Commands) {
    commands.remove_resource::<CreditsTimeline>();
}

fn cleanup_loading_screen(
    mut commands: Commands,
    entities: Query<Entity, With<LoadingScreenEntity>>,
    cameras: Query<Entity, With<LoadingUiCamera>>,
) {
    for entity in &entities {
        commands.entity(entity).try_despawn();
    }
    for entity in &cameras {
        commands.entity(entity).try_despawn();
    }
    clear_loading_runtime(&mut commands);
}

fn cleanup_loading_runtime(mut commands: Commands) {
    clear_loading_runtime(&mut commands);
}

fn clear_loading_runtime(commands: &mut Commands) {
    commands.remove_resource::<GameplayReady>();
    commands.remove_resource::<WorldLoadingRuntime>();
    commands.remove_resource::<WorldGenerationTask>();
    commands.remove_resource::<WorldLoadingCoverRuntime>();
    commands.remove_resource::<MenuLoadingRuntime>();
    commands.remove_resource::<MenuRevealRuntime>();
    commands.remove_resource::<MainMenuSpawnRuntime>();
    commands.remove_resource::<WorldRevealRuntime>();
}

fn cleanup_world(
    mut commands: Commands,
    entities: Query<Entity, With<WorldEntity>>,
    mut building_instances: ResMut<BuildingMaterialInstances>,
    mut building_update_runtime: ResMut<BuildingMaterialUpdateRuntime>,
    mut building_materials: Option<ResMut<Assets<BuildingMaterial>>>,
    mut diagnostic_view: ResMut<WorldDiagnosticRuntime>,
    mut station_targets: ResMut<StationTargetRuntime>,
    mut ruler_announcements: ResMut<RulerVoteAnnouncementRuntime>,
    mut death_announcements: ResMut<CitizenDeathAnnouncementRuntime>,
    mut night_waves: ResMut<NightEnemyWaveRuntime>,
) {
    for entity in &entities {
        commands.entity(entity).try_despawn();
    }
    if let Some(materials) = building_materials.as_deref_mut() {
        for instance in building_instances.0.values() {
            materials.remove(&instance.handle);
        }
    }
    building_instances.0.clear();
    *building_update_runtime = BuildingMaterialUpdateRuntime::default();
    commands.remove_resource::<WorldRuntime>();
    commands.insert_resource(FineNavigationRuntime::default());
    *diagnostic_view = WorldDiagnosticRuntime::default();
    commands.remove_resource::<SimulationRuntime>();
    commands.remove_resource::<WorldRevealRuntime>();
    commands.remove_resource::<GameplayReady>();
    commands.insert_resource(BuildingPlacers::default());
    commands.insert_resource(BuildingCommandQueue::default());
    *station_targets = StationTargetRuntime::default();
    *ruler_announcements = RulerVoteAnnouncementRuntime::default();
    *death_announcements = CitizenDeathAnnouncementRuntime::default();
    *night_waves = NightEnemyWaveRuntime::default();
}

fn cleanup_menu_overlay(mut commands: Commands, overlays: MenuOverlayEntityQuery) {
    for entity in &overlays {
        commands.entity(entity).try_despawn();
    }
}

fn should_show_actor_name(mode: NameDisplayMode, user_type: StreamUserType) -> bool {
    match mode {
        NameDisplayMode::None => false,
        NameDisplayMode::StaffAndSubscribers => user_type.is_staff_or_subscriber(),
        NameDisplayMode::AllPlayers => true,
    }
}

fn is_stream_player_actor(actor_id: &StableId) -> bool {
    actor_id.as_str().starts_with("twitch:")
}

fn stream_user_color(user_type: StreamUserType) -> Color {
    match user_type {
        StreamUserType::GameMaster => Color::srgb(1.0, 0.22, 0.0),
        StreamUserType::Broadcaster => Color::srgb(1.0, 0.12, 0.12),
        StreamUserType::Moderator => Color::srgb(0.2, 1.0, 0.3),
        StreamUserType::Subscriber => Color::srgb(0.48, 0.3, 0.78),
        StreamUserType::Normal => Color::srgb(0.88, 0.87, 0.80),
    }
}

fn actor_name_color(actor: &ActorState) -> Color {
    actor
        .customization
        .name_color
        .map_or_else(|| stream_user_color(actor.user_type), color_from_rgb8)
}

fn should_show_building_health(
    mode: BuildingHealthDisplayMode,
    health: i32,
    max_health: i32,
) -> bool {
    match mode {
        BuildingHealthDisplayMode::None => false,
        BuildingHealthDisplayMode::DamagedOnly => health < max_health,
        BuildingHealthDisplayMode::Always => true,
    }
}

fn overlay_viewport_position(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    world_position: Vec3,
) -> Option<Vec2> {
    let viewport = camera.logical_viewport_size()?;
    camera
        .world_to_viewport(camera_transform, world_position)
        .ok()
        .filter(|position| overlay_position_clears_hud(viewport, *position))
}

fn overlay_position_clears_hud(viewport: Vec2, position: Vec2) -> bool {
    position.x >= 0.0
        && position.y >= WORLD_UI_SAFE_TOP + WORLD_UI_OVERLAY_MARGIN
        && position.x <= viewport.x
        && position.y <= viewport.y - WORLD_UI_SAFE_BOTTOM - WORLD_UI_OVERLAY_MARGIN
}

#[allow(clippy::type_complexity)]
fn sync_actor_name_overlays(
    mut commands: Commands,
    settings: Res<RuntimePlayerSettings>,
    config: Res<RuntimeConfig>,
    simulation: Res<SimulationRuntime>,
    cameras: Query<(&Camera, &GlobalTransform), With<TownCamera>>,
    agents: Query<(&Agent, &GlobalTransform)>,
    mut overlays: Query<
        (
            Entity,
            &ActorNameOverlay,
            &mut Text,
            &mut TextColor,
            &mut Node,
            &mut Visibility,
        ),
        Without<BuildingHealthOverlay>,
    >,
) {
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };
    let actor_positions: BTreeMap<_, _> = agents
        .iter()
        .filter(|(agent, _)| agent.kind == ActorKind::Player && is_stream_player_actor(&agent.id))
        .map(|(agent, transform)| (agent.id.clone(), transform.translation()))
        .collect();
    let mut existing = BTreeSet::new();
    for (entity, overlay, mut text, mut color, mut node, mut visibility) in &mut overlays {
        existing.insert(overlay.actor.clone());
        let Some(actor) = simulation.0.actors.get(&overlay.actor) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let Some(position) = actor_positions.get(&overlay.actor) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let show = actor.alive
            && should_show_actor_name(settings.0.interface.display_names, actor.user_type);
        let screen = show.then(|| {
            overlay_viewport_position(
                camera,
                camera_transform,
                *position + Vec3::Y * config.0.world.cell_size * ACTOR_OVERLAY_ANCHOR_HEIGHT_CELLS,
            )
        });
        let Some(screen) = screen.flatten() else {
            *visibility = Visibility::Hidden;
            continue;
        };
        actor
            .display_name
            .as_deref()
            .or(actor.login_name.as_deref())
            .unwrap_or_else(|| overlay.actor.as_str())
            .clone_into(&mut *text);
        color.0 = actor_name_color(actor);
        node.left = px(screen.x - 80.0);
        node.top = px(screen.y + ACTOR_NAME_OVERLAY_TOP_PX);
        *visibility = Visibility::Visible;
    }
    for (actor_id, position) in actor_positions {
        let Some(actor) = simulation.0.actors.get(&actor_id) else {
            continue;
        };
        if existing.contains(&actor_id)
            || !actor.alive
            || !should_show_actor_name(settings.0.interface.display_names, actor.user_type)
        {
            continue;
        }
        let Some(screen) = overlay_viewport_position(
            camera,
            camera_transform,
            position + Vec3::Y * config.0.world.cell_size * ACTOR_OVERLAY_ANCHOR_HEIGHT_CELLS,
        ) else {
            continue;
        };
        commands.spawn((
            WorldEntity,
            ActorNameOverlay {
                actor: actor_id.clone(),
            },
            Text::new(
                actor
                    .display_name
                    .as_deref()
                    .or(actor.login_name.as_deref())
                    .unwrap_or_else(|| actor_id.as_str()),
            ),
            TextFont {
                font_size: FontSize::Px(15.0),
                ..default()
            },
            TextLayout::justify(Justify::Center),
            TextColor(actor_name_color(actor)),
            TextShadow {
                offset: Vec2::splat(1.5),
                color: Color::linear_rgba(0.0, 0.0, 0.0, 0.98),
            },
            Pickable::IGNORE,
            GlobalZIndex(18),
            Node {
                position_type: PositionType::Absolute,
                left: px(screen.x - 80.0),
                top: px(screen.y + ACTOR_NAME_OVERLAY_TOP_PX),
                width: px(160),
                ..default()
            },
        ));
    }
}

fn actor_health_bar_hide_seconds(content: &ContentCatalog, actor: &ActorState) -> Option<f32> {
    actor_archetype(content, actor)
        .and_then(|archetype| archetype.health_bar_hide_milliseconds)
        .or_else(|| (actor.role.as_str() == "role:enemy").then_some(3_000))
        .map(|milliseconds| Duration::from_millis(u64::from(milliseconds)).as_secs_f32())
}

fn update_actor_health_overlay_timer(
    overlay: &mut ActorHealthOverlay,
    health: i32,
    max_health: i32,
    hide_delay_seconds: f32,
    delta_seconds: f32,
) -> bool {
    if health < overlay.last_health {
        overlay.was_damaged = true;
        overlay.hide_remaining_seconds = hide_delay_seconds;
    } else if health != overlay.last_health && overlay.was_damaged {
        overlay.hide_remaining_seconds = hide_delay_seconds;
    }
    overlay.last_health = health;
    if !overlay.was_damaged {
        return false;
    }
    if health < max_health {
        return true;
    }
    overlay.hide_remaining_seconds =
        (overlay.hide_remaining_seconds - delta_seconds.max(0.0)).max(0.0);
    if overlay.hide_remaining_seconds > 0.0 {
        true
    } else {
        overlay.was_damaged = false;
        false
    }
}

fn actor_health_fill_color(actor: &ActorState) -> Color {
    if actor.role.as_str() == "role:enemy" {
        Color::srgb(0.58, 0.16, 0.86)
    } else {
        Color::srgb(0.85, 0.1, 0.1)
    }
}

#[allow(clippy::type_complexity)]
fn sync_actor_health_overlays(
    mut commands: Commands,
    cadence: Res<AgentSimulationCadence>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    cameras: Query<(&Camera, &GlobalTransform), With<TownCamera>>,
    agents: Query<(&Agent, &GlobalTransform)>,
    mut overlays: Query<(
        Entity,
        &mut ActorHealthOverlay,
        &Children,
        &mut Node,
        &mut Visibility,
    )>,
    mut fills: Query<
        (&mut Node, &mut BackgroundColor),
        (
            With<ActorHealthFill>,
            Without<ActorHealthOverlay>,
            Without<ActorNameOverlay>,
            Without<BuildingHealthOverlay>,
        ),
    >,
) {
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };
    let actor_positions: BTreeMap<_, _> = agents
        .iter()
        .filter(|(agent, _)| matches!(agent.kind, ActorKind::Player | ActorKind::Enemy))
        .map(|(agent, transform)| (agent.id.clone(), transform.translation()))
        .collect();
    let mut existing = BTreeSet::new();
    for (entity, mut overlay, children, mut node, mut visibility) in &mut overlays {
        existing.insert(overlay.actor.clone());
        let Some(actor) = simulation.0.actors.get(&overlay.actor) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let Some(position) = actor_positions.get(&overlay.actor) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let Some(hide_delay) = actor_health_bar_hide_seconds(&content.0, actor) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let show = actor.alive
            && update_actor_health_overlay_timer(
                &mut overlay,
                actor.health,
                actor.max_health.max(1),
                hide_delay,
                cadence.delta().as_secs_f32(),
            );
        let screen = show.then(|| {
            overlay_viewport_position(
                camera,
                camera_transform,
                *position + Vec3::Y * config.0.world.cell_size * ACTOR_OVERLAY_ANCHOR_HEIGHT_CELLS,
            )
        });
        let Some(screen) = screen.flatten() else {
            *visibility = Visibility::Hidden;
            continue;
        };
        node.left = px(screen.x - 45.0);
        node.top = px(screen.y + ACTOR_HEALTH_OVERLAY_TOP_PX);
        *visibility = Visibility::Visible;
        let health_fraction = building_health_fraction(actor.health, actor.max_health.max(1));
        for child in children.iter() {
            if let Ok((mut fill, mut color)) = fills.get_mut(child) {
                fill.width = percent(health_fraction * 100.0);
                color.0 = actor_health_fill_color(actor);
            }
        }
    }
    for (actor_id, position) in actor_positions {
        let Some(actor) = simulation.0.actors.get(&actor_id) else {
            continue;
        };
        let Some(hide_delay) = actor_health_bar_hide_seconds(&content.0, actor) else {
            continue;
        };
        if existing.contains(&actor_id) || !actor.alive || actor.health >= actor.max_health.max(1) {
            continue;
        }
        let Some(screen) = overlay_viewport_position(
            camera,
            camera_transform,
            position + Vec3::Y * config.0.world.cell_size * ACTOR_OVERLAY_ANCHOR_HEIGHT_CELLS,
        ) else {
            continue;
        };
        let health_fraction = building_health_fraction(actor.health, actor.max_health.max(1));
        commands
            .spawn((
                WorldEntity,
                ActorHealthOverlay {
                    actor: actor_id,
                    last_health: actor.health,
                    was_damaged: true,
                    hide_remaining_seconds: hide_delay,
                },
                Node {
                    position_type: PositionType::Absolute,
                    left: px(screen.x - 45.0),
                    top: px(screen.y + ACTOR_HEALTH_OVERLAY_TOP_PX),
                    width: px(90),
                    height: px(10),
                    padding: UiRect::all(px(1)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
                GlobalZIndex(19),
            ))
            .with_children(|parent| {
                parent.spawn((
                    ActorHealthFill,
                    Node {
                        width: percent(health_fraction * 100.0),
                        height: percent(100.0),
                        ..default()
                    },
                    BackgroundColor(actor_health_fill_color(actor)),
                ));
            });
    }
}

fn temporary_world_label_is_live(label: &mut TemporaryWorldLabel, delta_seconds: f32) -> bool {
    label.remaining_seconds = (label.remaining_seconds - delta_seconds.max(0.0)).max(0.0);
    label.remaining_seconds > 0.0
}

#[allow(clippy::type_complexity)]
fn sync_temporary_world_labels(
    mut commands: Commands,
    cadence: Res<AgentSimulationCadence>,
    config: Res<RuntimeConfig>,
    cameras: Query<(&Camera, &GlobalTransform), With<TownCamera>>,
    agents: Query<(&Agent, &GlobalTransform)>,
    buildings: Query<(&RuntimeBuilding, &GlobalTransform)>,
    resources: Query<(&ResourceNode, &GlobalTransform)>,
    mut labels: Query<(Entity, &mut TemporaryWorldLabel, &mut Node, &mut Visibility)>,
) {
    // Numbered labels exist only briefly after a matching chat command. Avoid
    // cloning every resource ID into a new map on ordinary unlabeled frames.
    if labels.is_empty() {
        return;
    }
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };
    let mut target_positions = BTreeMap::new();
    target_positions.extend(
        agents
            .iter()
            .map(|(agent, transform)| (agent.id.clone(), transform.translation())),
    );
    target_positions.extend(
        buildings
            .iter()
            .map(|(building, transform)| (building.id.clone(), transform.translation())),
    );
    target_positions.extend(
        resources
            .iter()
            .map(|(resource, transform)| (resource.id.clone(), transform.translation())),
    );
    for (entity, mut label, mut node, mut visibility) in &mut labels {
        if !temporary_world_label_is_live(&mut label, cadence.delta().as_secs_f32()) {
            commands.entity(entity).try_despawn();
            continue;
        }
        let Some(position) = target_positions.get(&label.target) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let Some(screen) = overlay_viewport_position(
            camera,
            camera_transform,
            *position + Vec3::Y * config.0.world.cell_size * 1.35,
        ) else {
            *visibility = Visibility::Hidden;
            continue;
        };
        node.left = px(screen.x - 20.0);
        node.top = px(screen.y - 12.0);
        *visibility = Visibility::Visible;
    }
}

fn building_health_overlay_world_position(
    building: &RuntimeBuilding,
    fallback: Vec3,
    simulation: &WorldSimulation,
    config: &GameConfig,
    world: &GeneratedWorld,
) -> Vec3 {
    simulation
        .path_navigation_positions
        .get(&building.id)
        .map_or_else(
            || fallback,
            |fine| navigation_to_world_on_surface(*fine, config, world),
        )
}

fn building_health_overlay_width_px(content: &ContentCatalog, building: &BuildingState) -> f32 {
    if building_def_for_archetype(content, &building.archetype)
        .is_some_and(|definition| definition.archetype.as_str() == "archetype:building:path")
    {
        PATH_HEALTH_OVERLAY_WIDTH_PX
    } else {
        BUILDING_HEALTH_OVERLAY_WIDTH_PX
    }
}

#[allow(clippy::type_complexity)]
fn sync_building_health_overlays(
    mut commands: Commands,
    settings: Res<RuntimePlayerSettings>,
    config: Res<RuntimeConfig>,
    content: Res<RuntimeContent>,
    simulation: Res<SimulationRuntime>,
    world: Res<WorldRuntime>,
    cameras: Query<(&Camera, &GlobalTransform), With<TownCamera>>,
    buildings: Query<(&RuntimeBuilding, &GlobalTransform)>,
    mut overlays: Query<
        (
            Entity,
            &BuildingHealthOverlay,
            &Children,
            &mut Node,
            &mut Visibility,
        ),
        Without<ActorNameOverlay>,
    >,
    mut fills: Query<
        (&mut Node, &mut BackgroundColor),
        (
            With<BuildingHealthFill>,
            Without<ActorNameOverlay>,
            Without<BuildingHealthOverlay>,
        ),
    >,
) {
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };
    let building_positions: BTreeMap<_, _> = buildings
        .iter()
        .filter_map(|(building, transform)| {
            let state = simulation.0.buildings.get(&building.id)?;
            let max_health = building_max_health(&content.0, state).max(1);
            if !should_show_building_health(
                settings.0.interface.display_building_health,
                state.health,
                max_health,
            ) {
                return None;
            }
            let position = building_health_overlay_world_position(
                building,
                transform.translation(),
                &simulation.0,
                &config.0,
                &world.generated,
            );
            let width = building_health_overlay_width_px(&content.0, state);
            Some((building.id.clone(), (position, width)))
        })
        .collect();
    let mut existing = BTreeSet::new();
    for (entity, overlay, children, mut node, mut visibility) in &mut overlays {
        existing.insert(overlay.building.clone());
        let Some(building) = simulation.0.buildings.get(&overlay.building) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let max_health = building_max_health(&content.0, building).max(1);
        let health_fraction = building_health_fraction(building.health, max_health);
        let show = should_show_building_health(
            settings.0.interface.display_building_health,
            building.health,
            max_health,
        );
        if !show {
            *visibility = Visibility::Hidden;
            continue;
        }
        let Some((position, width)) = building_positions.get(&overlay.building) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        let screen = show.then(|| {
            overlay_viewport_position(
                camera,
                camera_transform,
                *position + Vec3::Y * config.0.world.cell_size * 1.45,
            )
        });
        let Some(screen) = screen.flatten() else {
            *visibility = Visibility::Hidden;
            continue;
        };
        node.left = px(screen.x - *width * 0.5);
        node.top = px(screen.y - 5.0);
        node.width = px(*width);
        *visibility = Visibility::Visible;
        for child in children.iter() {
            if let Ok((mut fill, mut color)) = fills.get_mut(child) {
                fill.width = percent(health_fraction * 100.0);
                color.0 = building_health_color(health_fraction);
            }
        }
    }
    for (building_id, (position, width)) in building_positions {
        let Some(building) = simulation.0.buildings.get(&building_id) else {
            continue;
        };
        let max_health = building_max_health(&content.0, building).max(1);
        if existing.contains(&building_id)
            || !should_show_building_health(
                settings.0.interface.display_building_health,
                building.health,
                max_health,
            )
        {
            continue;
        }
        let Some(screen) = overlay_viewport_position(
            camera,
            camera_transform,
            position + Vec3::Y * config.0.world.cell_size * 1.45,
        ) else {
            continue;
        };
        let health_fraction = building_health_fraction(building.health, max_health);
        commands
            .spawn((
                WorldEntity,
                BuildingHealthOverlay {
                    building: building_id,
                },
                Node {
                    position_type: PositionType::Absolute,
                    left: px(screen.x - width * 0.5),
                    top: px(screen.y - 5.0),
                    width: px(width),
                    height: px(10),
                    border: UiRect::all(px(1)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.02, 0.025, 0.03, 0.9)),
                BorderColor::all(Color::srgba(0.8, 0.86, 0.82, 0.8)),
                GlobalZIndex(19),
            ))
            .with_children(|parent| {
                parent.spawn((
                    BuildingHealthFill,
                    Node {
                        width: percent(health_fraction * 100.0),
                        height: percent(100.0),
                        ..default()
                    },
                    BackgroundColor(building_health_color(health_fraction)),
                ));
            });
    }
}

fn building_health_fraction(health: i32, max_health: i32) -> f32 {
    let thousandths = health
        .max(0)
        .saturating_mul(1_000)
        .checked_div(max_health.max(1))
        .unwrap_or_default()
        .clamp(0, 1_000);
    f32::from(u16::try_from(thousandths).expect("clamped health fraction fits u16")) / 1_000.0
}

fn building_health_color(health_fraction: f32) -> Color {
    Color::srgb(
        1.0 - health_fraction * 0.72,
        0.18 + health_fraction * 0.68,
        0.12,
    )
}

fn grid_to_world(position: GridPos, config: &GameConfig) -> Vec3 {
    Vec3::new(
        (f32::from(position.x) - f32::from(config.world.width.saturating_sub(1)) * 0.5)
            * config.world.cell_size,
        0.0,
        (f32::from(position.z) - f32::from(config.world.height.saturating_sub(1)) * 0.5)
            * config.world.cell_size,
    )
}

fn grid_to_world_on_surface(
    position: GridPos,
    config: &GameConfig,
    world: &GeneratedWorld,
) -> Vec3 {
    let mut position_world = grid_to_world(position, config);
    position_world.y = terrain_height(world, position);
    position_world
}

fn terrain_height(world: &GeneratedWorld, position: GridPos) -> f32 {
    f32::from(world.navigation.height_at(position).unwrap_or_default()) * 0.01
}

fn shoreline_focus(world: &GeneratedWorld, config: &GameConfig) -> Vec3 {
    let centre = GridPos {
        x: world.navigation.width() / 2,
        z: world.navigation.height() / 2,
    };
    let water_height = f32::from(config.world.water_level_centimetres) * 0.01;
    let best = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .filter(|position| terrain_height(world, *position) <= water_height)
        .filter(|position| {
            [
                position
                    .x
                    .checked_sub(1)
                    .map(|x| GridPos { x, z: position.z }),
                (position.x + 1 < world.navigation.width()).then_some(GridPos {
                    x: position.x + 1,
                    z: position.z,
                }),
                position
                    .z
                    .checked_sub(1)
                    .map(|z| GridPos { x: position.x, z }),
                (position.z + 1 < world.navigation.height()).then_some(GridPos {
                    x: position.x,
                    z: position.z + 1,
                }),
            ]
            .into_iter()
            .flatten()
            .any(|neighbor| terrain_height(world, neighbor) > water_height)
        })
        .min_by_key(|position| position.x.abs_diff(centre.x) + position.z.abs_diff(centre.z))
        .unwrap_or(centre);
    let mut boundary = grid_to_world_on_surface(best, config, world);
    boundary.y = water_height;
    let centre_world = grid_to_world_on_surface(centre, config, world);
    let inward = Vec3::new(
        centre_world.x - boundary.x,
        0.0,
        centre_world.z - boundary.z,
    )
    .normalize_or_zero();
    boundary + inward * config.world.cell_size * 2.5
}

fn shoreline_camera_transform(world: &GeneratedWorld, config: &GameConfig) -> Transform {
    let focus = shoreline_focus(world, config);
    let centre = GridPos {
        x: world.navigation.width() / 2,
        z: world.navigation.height() / 2,
    };
    let centre_world = grid_to_world_on_surface(centre, config, world);
    let inward =
        Vec3::new(centre_world.x - focus.x, 0.0, centre_world.z - focus.z).normalize_or_zero();
    let outward = -inward;
    let tangent = Vec3::new(-inward.z, 0.0, inward.x);
    Transform::from_translation(focus + outward * 28.0 + tangent * 8.0 + Vec3::Y * 24.0)
        .looking_at(focus + Vec3::Y, Vec3::Y)
}

struct GeneratedTerrainChunk {
    chunk_x: u16,
    chunk_z: u16,
    centre: Vec2,
    high: Mesh,
    medium: Mesh,
    low: Mesh,
}

fn generated_terrain_chunks(
    world: &GeneratedWorld,
    config: &GameConfig,
) -> Vec<GeneratedTerrainChunk> {
    let width = world.navigation.width();
    let height = world.navigation.height();
    (0..height.div_ceil(TERRAIN_CHUNK_CELLS))
        .flat_map(|chunk_z| {
            (0..width.div_ceil(TERRAIN_CHUNK_CELLS)).map(move |chunk_x| {
                let start_x = chunk_x * TERRAIN_CHUNK_CELLS;
                let start_z = chunk_z * TERRAIN_CHUNK_CELLS;
                let cells_x = (width - start_x).min(TERRAIN_CHUNK_CELLS);
                let cells_z = (height - start_z).min(TERRAIN_CHUNK_CELLS);
                let centre = Vec2::new(
                    (f32::from(start_x) + f32::from(cells_x) * 0.5
                        - f32::from(width.saturating_sub(1)) * 0.5)
                        * config.world.cell_size,
                    (f32::from(start_z) + f32::from(cells_z) * 0.5
                        - f32::from(height.saturating_sub(1)) * 0.5)
                        * config.world.cell_size,
                );
                GeneratedTerrainChunk {
                    chunk_x,
                    chunk_z,
                    centre,
                    high: generated_terrain_chunk_mesh(
                        world, config, start_x, start_z, cells_x, cells_z, 1, false,
                    ),
                    medium: generated_terrain_chunk_mesh(
                        world, config, start_x, start_z, cells_x, cells_z, 2, true,
                    ),
                    low: generated_terrain_chunk_mesh(
                        world, config, start_x, start_z, cells_x, cells_z, 4, true,
                    ),
                }
            })
        })
        .collect()
}

fn generated_terrain_mesh(world: &GeneratedWorld, config: &GameConfig) -> Mesh {
    let width = world.navigation.width();
    let height = world.navigation.height();
    let half = config.world.cell_size * 0.5 * 0.7;
    let estimated_faces = usize::from(width) * usize::from(height) * 4;
    let mut positions = Vec::with_capacity(estimated_faces * 4);
    let mut colors = Vec::with_capacity(estimated_faces * 4);
    let mut uvs = Vec::with_capacity(estimated_faces * 4);
    let mut indices = Vec::with_capacity(estimated_faces * 6);
    let centre = |x: u16, z: u16| {
        Vec2::new(
            (f32::from(x) - f32::from(width.saturating_sub(1)) * 0.5) * config.world.cell_size,
            (f32::from(z) - f32::from(height.saturating_sub(1)) * 0.5) * config.world.cell_size,
        )
    };
    let elevation = |x: u16, z: u16| {
        f32::from(
            world
                .navigation
                .height_at(GridPos { x, z })
                .unwrap_or_default(),
        ) * 0.01
    };
    for z in 0..height {
        for x in 0..width {
            let c = centre(x, z);
            let y = elevation(x, z);
            append_terrain_quad(
                [
                    [c.x - half, y, c.y - half],
                    [c.x + half, y, c.y - half],
                    [c.x + half, y, c.y + half],
                    [c.x - half, y, c.y + half],
                ],
                terrain_vertex_color(y, config),
                [
                    f32::from(x) / f32::from(width),
                    f32::from(z) / f32::from(height),
                ],
                &mut positions,
                &mut colors,
                &mut uvs,
                &mut indices,
            );
            if x > 0 {
                let previous = centre(x - 1, z);
                let previous_y = elevation(x - 1, z);
                append_terrain_quad(
                    [
                        [previous.x + half, previous_y, c.y - half],
                        [c.x - half, y, c.y - half],
                        [c.x - half, y, c.y + half],
                        [previous.x + half, previous_y, c.y + half],
                    ],
                    terrain_vertex_color((y + previous_y) * 0.5, config),
                    [
                        f32::from(x) / f32::from(width),
                        f32::from(z) / f32::from(height),
                    ],
                    &mut positions,
                    &mut colors,
                    &mut uvs,
                    &mut indices,
                );
            }
            if z > 0 {
                let previous = centre(x, z - 1);
                let previous_y = elevation(x, z - 1);
                append_terrain_quad(
                    [
                        [c.x - half, previous_y, previous.y + half],
                        [c.x + half, previous_y, previous.y + half],
                        [c.x + half, y, c.y - half],
                        [c.x - half, y, c.y - half],
                    ],
                    terrain_vertex_color((y + previous_y) * 0.5, config),
                    [
                        f32::from(x) / f32::from(width),
                        f32::from(z) / f32::from(height),
                    ],
                    &mut positions,
                    &mut colors,
                    &mut uvs,
                    &mut indices,
                );
            }
            if x > 0 && z > 0 {
                let upper_left = centre(x - 1, z - 1);
                let upper_right = centre(x, z - 1);
                let lower_left = centre(x - 1, z);
                let heights = [
                    elevation(x - 1, z - 1),
                    elevation(x, z - 1),
                    y,
                    elevation(x - 1, z),
                ];
                append_terrain_quad(
                    [
                        [upper_left.x + half, heights[0], upper_left.y + half],
                        [upper_right.x - half, heights[1], upper_right.y + half],
                        [c.x - half, heights[2], c.y - half],
                        [lower_left.x + half, heights[3], lower_left.y - half],
                    ],
                    terrain_vertex_color(heights.into_iter().sum::<f32>() * 0.25, config),
                    [
                        f32::from(x) / f32::from(width),
                        f32::from(z) / f32::from(height),
                    ],
                    &mut positions,
                    &mut colors,
                    &mut uvs,
                    &mut indices,
                );
            }
        }
    }
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices));
    mesh.compute_smooth_normals();
    mesh
}

#[allow(clippy::too_many_arguments)]
fn append_terrain_quad(
    quad: [[f32; 3]; 4],
    color: [f32; 4],
    uv: [f32; 2],
    positions: &mut Vec<[f32; 3]>,
    colors: &mut Vec<[f32; 4]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
) {
    let base = u32::try_from(positions.len()).expect("terrain vertex count fits u32");
    positions.extend_from_slice(&quad);
    colors.extend_from_slice(&[color; 4]);
    uvs.extend_from_slice(&[uv; 4]);
    indices.extend_from_slice(&[base, base + 3, base + 1, base + 1, base + 3, base + 2]);
}

fn generated_terrain_chunk_mesh(
    world: &GeneratedWorld,
    config: &GameConfig,
    start_x: u16,
    start_z: u16,
    cells_x: u16,
    cells_z: u16,
    detail_step: u16,
    add_skirts: bool,
) -> Mesh {
    let world_width = world.navigation.width();
    let world_height = world.navigation.height();
    let sample_x = terrain_lod_samples(cells_x, detail_step);
    let sample_z = terrain_lod_samples(cells_z, detail_step);
    let columns = u32::try_from(sample_x.len()).expect("terrain LOD column count fits u32");
    let mut positions = Vec::with_capacity(sample_x.len() * sample_z.len());
    let mut colors = Vec::with_capacity(positions.capacity());
    let mut uvs = Vec::with_capacity(positions.capacity());
    for &local_z in &sample_z {
        for &local_x in &sample_x {
            let x = start_x + local_x;
            let z = start_z + local_z;
            let elevation = terrain_corner_height(world, x, z);
            positions.push([
                (f32::from(x) - f32::from(world_width.saturating_sub(1)) * 0.5)
                    * config.world.cell_size,
                elevation,
                (f32::from(z) - f32::from(world_height.saturating_sub(1)) * 0.5)
                    * config.world.cell_size,
            ]);
            colors.push(terrain_vertex_color(elevation, config));
            uvs.push([
                f32::from(x) / f32::from(world_width),
                f32::from(z) / f32::from(world_height),
            ]);
        }
    }

    let mut indices = Vec::with_capacity((sample_x.len() - 1) * (sample_z.len() - 1) * 6);
    for z in 0..u32::try_from(sample_z.len() - 1).expect("terrain rows fit u32") {
        for x in 0..u32::try_from(sample_x.len() - 1).expect("terrain columns fit u32") {
            let top_left = z * columns + x;
            let top_right = top_left + 1;
            let bottom_left = top_left + columns;
            let bottom_right = bottom_left + 1;
            indices.extend_from_slice(&[
                top_left,
                bottom_left,
                top_right,
                top_right,
                bottom_left,
                bottom_right,
            ]);
        }
    }

    if add_skirts {
        let skirt_depth = config.world.cell_size.max(1.0) * 0.75;
        let top = sample_x.iter().copied().map(|x| (x, 0)).collect::<Vec<_>>();
        let bottom = sample_x
            .iter()
            .copied()
            .map(|x| (x, cells_z))
            .collect::<Vec<_>>();
        let left = sample_z.iter().copied().map(|z| (0, z)).collect::<Vec<_>>();
        let right = sample_z
            .iter()
            .copied()
            .map(|z| (cells_x, z))
            .collect::<Vec<_>>();
        for edge in [&top, &bottom, &left, &right] {
            append_terrain_skirt(
                world,
                config,
                start_x,
                start_z,
                edge,
                skirt_depth,
                &mut positions,
                &mut colors,
                &mut uvs,
                &mut indices,
            );
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices));
    mesh.compute_smooth_normals();
    mesh
}

fn terrain_lod_samples(cells: u16, step: u16) -> Vec<u16> {
    let step = step.max(1);
    let mut samples: Vec<_> = (0..=cells).step_by(usize::from(step)).collect();
    if samples.last().copied() != Some(cells) {
        samples.push(cells);
    }
    samples
}

#[allow(clippy::too_many_arguments)]
fn append_terrain_skirt(
    world: &GeneratedWorld,
    config: &GameConfig,
    start_x: u16,
    start_z: u16,
    edge: &[(u16, u16)],
    depth: f32,
    positions: &mut Vec<[f32; 3]>,
    colors: &mut Vec<[f32; 4]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
) {
    let world_width = world.navigation.width();
    let world_height = world.navigation.height();
    for pair in edge.windows(2) {
        let base = u32::try_from(positions.len()).expect("terrain mesh vertex count fits u32");
        for (vertex_index, &(local_x, local_z)) in
            [pair[0], pair[0], pair[1], pair[1]].iter().enumerate()
        {
            let x = start_x + local_x;
            let z = start_z + local_z;
            let elevation = terrain_corner_height(world, x, z);
            let lower = vertex_index % 2 == 1;
            positions.push([
                (f32::from(x) - f32::from(world_width.saturating_sub(1)) * 0.5)
                    * config.world.cell_size,
                elevation - if lower { depth } else { 0.0 },
                (f32::from(z) - f32::from(world_height.saturating_sub(1)) * 0.5)
                    * config.world.cell_size,
            ]);
            colors.push(terrain_vertex_color(elevation, config));
            uvs.push([
                f32::from(x) / f32::from(world_width),
                f32::from(z) / f32::from(world_height),
            ]);
        }
        indices.extend_from_slice(&[
            base,
            base + 1,
            base + 2,
            base + 2,
            base + 1,
            base + 3,
            base,
            base + 2,
            base + 1,
            base + 2,
            base + 3,
            base + 1,
        ]);
    }
}

fn generated_water_mesh(world: &GeneratedWorld, config: &GameConfig) -> Mesh {
    // Unity's Water_Main plane is vastly wider than the authored terrain. The
    // previous eight-cell skirt exposed the clear colour as a black bar at the
    // legal maximum zoom, so retain enough ocean to cover that full view.
    let width = world.navigation.width();
    let height = world.navigation.height();
    let padded_width = width + OCEAN_PADDING_CELLS * 2;
    let padded_height = height + OCEAN_PADDING_CELLS * 2;
    let columns = u32::from(padded_width) + 1;
    let water_height = f32::from(config.world.water_level_centimetres) * 0.01;
    let authored_depth_range = 10.0_f32;
    let mut positions =
        Vec::with_capacity(usize::from(padded_width + 1) * usize::from(padded_height + 1));
    let mut normals = Vec::with_capacity(positions.capacity());
    let mut colors = Vec::with_capacity(positions.capacity());
    let mut uvs = Vec::with_capacity(positions.capacity());
    for z in 0..=padded_height {
        for x in 0..=padded_width {
            let terrain_height = if (OCEAN_PADDING_CELLS..=OCEAN_PADDING_CELLS + width).contains(&x)
                && (OCEAN_PADDING_CELLS..=OCEAN_PADDING_CELLS + height).contains(&z)
            {
                terrain_corner_height(world, x - OCEAN_PADDING_CELLS, z - OCEAN_PADDING_CELLS)
            } else {
                water_height - authored_depth_range
            };
            let depth = (water_height - terrain_height).max(0.0);
            let normalized_depth = (depth / authored_depth_range).clamp(0.0, 1.0);
            positions.push([
                (f32::from(x)
                    - f32::from(OCEAN_PADDING_CELLS)
                    - f32::from(width.saturating_sub(1)) * 0.5)
                    * config.world.cell_size,
                water_height + VISIBLE_WATER_SURFACE_LIFT_METRES,
                (f32::from(z)
                    - f32::from(OCEAN_PADDING_CELLS)
                    - f32::from(height.saturating_sub(1)) * 0.5)
                    * config.world.cell_size,
            ]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push([normalized_depth, 0.0, 0.0, 1.0]);
            uvs.push([
                f32::from(x) / f32::from(padded_width),
                f32::from(z) / f32::from(padded_height),
            ]);
        }
    }

    let mut indices =
        Vec::with_capacity(usize::from(padded_width) * usize::from(padded_height) * 6);
    for z in 0..u32::from(padded_height) {
        for x in 0..u32::from(padded_width) {
            let top_left = z * columns + x;
            let top_right = top_left + 1;
            let bottom_left = top_left + columns;
            let bottom_right = bottom_left + 1;
            indices.extend_from_slice(&[
                top_left,
                bottom_left,
                top_right,
                top_right,
                bottom_left,
                bottom_right,
            ]);
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

fn healing_ring_mesh(segments: u32) -> Mesh {
    let segments = u16::try_from(segments.max(3).min(u32::from(u16::MAX)))
        .expect("clamped ring segment count fits u16");
    let mut positions = Vec::with_capacity(usize::from(segments) * 2);
    let mut normals = Vec::with_capacity(positions.capacity());
    let mut uvs = Vec::with_capacity(positions.capacity());
    for index in 0..segments {
        let angle = f32::from(index) / f32::from(segments) * std::f32::consts::TAU;
        let direction = Vec2::new(angle.cos(), angle.sin());
        for (radius, uv_y) in [(0.84, 0.0), (1.0, 1.0)] {
            positions.push([direction.x * radius, 0.0, direction.y * radius]);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([f32::from(index) / f32::from(segments), uv_y]);
        }
    }
    let mut indices = Vec::with_capacity(usize::from(segments) * 6);
    for index in 0..segments {
        let next = (index + 1) % segments;
        let inner = u32::from(index) * 2;
        let outer = inner + 1;
        let next_inner = u32::from(next) * 2;
        let next_outer = next_inner + 1;
        indices.extend_from_slice(&[inner, next_inner, outer, next_inner, next_outer, outer]);
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

fn terrain_corner_height(world: &GeneratedWorld, corner_x: u16, corner_z: u16) -> f32 {
    navigation_corner_height_metres(&world.navigation, corner_x, corner_z)
}

/// Samples the same two triangles emitted by `generated_terrain_chunk_mesh` at
/// full detail. Decorative objects must use this rendered surface rather than
/// a navigation cell's raw height: terrain vertices average adjacent cells,
/// and the object's deterministic X/Z offset can move it across a slope.
fn terrain_surface_height_at_world(
    world: &GeneratedWorld,
    config: &GameConfig,
    world_x: f32,
    world_z: f32,
) -> Option<f32> {
    navigation_surface_height_at_world(&world.navigation, &config.world, world_x, world_z)
}

fn resolved_foliage_ground_position(
    world: &GeneratedWorld,
    config: &GameConfig,
    foliage: &GeneratedFoliage,
) -> Option<Vec3> {
    let mut position = grid_to_world(foliage.position, config);
    let offset = locational_visual_offset(
        world.seed,
        &foliage.id,
        foliage.position,
        foliage.offset_milli_cells,
        config.world.cell_size,
    );
    position.x += offset.x;
    position.z += offset.y;
    position.y = terrain_surface_height_at_world(world, config, position.x, position.z)?;

    let visible_water_height = visible_water_surface_height(&config.world);
    match foliage.habitat {
        FoliageHabitat::Land if position.y <= visible_water_height => None,
        FoliageHabitat::Underwater if position.y >= visible_water_height => None,
        FoliageHabitat::Land | FoliageHabitat::Underwater => Some(position),
    }
}

fn terrain_vertex_color(elevation: f32, config: &GameConfig) -> [f32; 4] {
    let water = f32::from(config.world.water_level_centimetres) * 0.01;
    if elevation <= water {
        return [0.12, 0.17, 0.13, 1.0];
    }
    let peak = f32::from(config.world.height_scale_centimetres).max(1.0) * 0.01;
    let normalized = ((elevation - water) / (peak - water).max(0.01)).clamp(0.0, 1.0);
    [
        0.10 + normalized * 0.22,
        0.28 + normalized * 0.25,
        0.12 + normalized * 0.12,
        1.0,
    ]
}

fn world_to_grid(position: Vec3, config: &GameConfig) -> Option<GridPos> {
    let x = (position.x / config.world.cell_size
        + f32::from(config.world.width.saturating_sub(1)) * 0.5)
        .round();
    let z = (position.z / config.world.cell_size
        + f32::from(config.world.height.saturating_sub(1)) * 0.5)
        .round();
    if x < 0.0
        || z < 0.0
        || x >= f32::from(config.world.width)
        || z >= f32::from(config.world.height)
    {
        return None;
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Some(GridPos {
        x: x as u16,
        z: z as u16,
    })
}

fn world_to_navigation(position: Vec3, config: &GameConfig) -> Option<GridPos> {
    let subdivision = f32::from(NAVIGATION_SUBDIVISIONS);
    let half = f32::from(NAVIGATION_SUBDIVISIONS / 2);
    let x = ((position.x / config.world.cell_size
        + f32::from(config.world.width.saturating_sub(1)) * 0.5)
        * subdivision
        + half)
        .round();
    let z = ((position.z / config.world.cell_size
        + f32::from(config.world.height.saturating_sub(1)) * 0.5)
        * subdivision
        + half)
        .round();
    let width = f32::from(config.world.width.saturating_mul(NAVIGATION_SUBDIVISIONS));
    let height = f32::from(config.world.height.saturating_mul(NAVIGATION_SUBDIVISIONS));
    if x < 0.0 || z < 0.0 || x >= width || z >= height {
        return None;
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Some(GridPos {
        x: x as u16,
        z: z as u16,
    })
}
