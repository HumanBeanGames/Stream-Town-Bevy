pub(crate) fn setup_accessibility(mut commands: Commands) {
    let mut announcement = AccessibleNode::new(Role::Status);
    announcement.set_label("Stream Town status");
    announcement.set_value("Starting Stream Town");
    announcement.set_live(Live::Polite);
    commands.spawn((
        Name::new("Accessibility announcements"),
        AccessibilityAnnouncement,
        AccessibilityNode(announcement),
    ));
}

#[allow(clippy::type_complexity)]
pub(crate) fn tag_accessible_buttons(
    mut commands: Commands,
    buttons: Query<
        (
            Entity,
            Option<&MainMenuAction>,
            Option<&GoLiveConfirmationAction>,
            Option<&GameMenuAction>,
            Option<&SettingsTabButton>,
            Option<&SettingsValueButton>,
            Option<&SettingsAction>,
            Option<&CreditsSkipButton>,
        ),
        (With<Button>, Without<AccessibleButtonScope>),
    >,
) {
    for (entity, main, go_live, game, tab, value, settings, credits) in &buttons {
        let scope = if main.is_some() {
            Some(AccessibleButtonScope::MainMenu)
        } else if go_live.is_some() {
            Some(AccessibleButtonScope::GoLiveConfirmation)
        } else if game.is_some() {
            Some(AccessibleButtonScope::GameMenu)
        } else if settings.is_some_and(|action| {
            matches!(
                action,
                SettingsAction::ConfirmApply | SettingsAction::ConfirmDiscard
            )
        }) {
            Some(AccessibleButtonScope::SettingsConfirm)
        } else if tab.is_some() || value.is_some() || settings.is_some() {
            Some(AccessibleButtonScope::Settings)
        } else if credits.is_some() {
            Some(AccessibleButtonScope::Credits)
        } else {
            None
        };
        if let Some(scope) = scope {
            commands
                .entity(entity)
                .try_insert((scope, Outline::new(px(0), px(0), Color::NONE)));
        }
    }
}

pub(crate) fn tag_accessible_text(
    mut commands: Commands,
    text: Query<Entity, (Added<Text>, Without<Label>)>,
    parents: Query<&ChildOf>,
    buttons: Query<(), With<Button>>,
) {
    for entity in &text {
        let mut ancestor = entity;
        let mut inside_button = false;
        for _ in 0..16 {
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
            if buttons.contains(ancestor) {
                inside_button = true;
                break;
            }
        }
        if !inside_button {
            commands.entity(entity).try_insert(Label);
        }
    }
}

type DecorativeAccessibilityImageQuery<'w, 's> =
    Query<'w, 's, Entity, (With<ImageNode>, With<AccessibilityNode>, Without<Button>)>;

pub(crate) fn prune_decorative_accessibility_nodes(
    mut commands: Commands,
    images: DecorativeAccessibilityImageQuery,
) {
    for entity in &images {
        commands.entity(entity).try_remove::<AccessibilityNode>();
    }
}

pub(crate) fn accessibility_scope_active(
    scope: AccessibleButtonScope,
    state: GameState,
    menu: &MenuRuntime,
) -> bool {
    match scope {
        AccessibleButtonScope::MainMenu => {
            state == GameState::MainMenu && menu.page == MenuPage::Closed
        }
        AccessibleButtonScope::GoLiveConfirmation => {
            state == GameState::MainMenu && menu.page == MenuPage::GoLiveConfirmation
        }
        AccessibleButtonScope::GameMenu => {
            state == GameState::InGame && menu.page == MenuPage::Game
        }
        AccessibleButtonScope::Settings => {
            menu.page == MenuPage::Settings && !menu.confirm_settings_close
        }
        AccessibleButtonScope::SettingsConfirm => {
            menu.page == MenuPage::Settings && menu.confirm_settings_close
        }
        AccessibleButtonScope::Credits => state == GameState::Credits,
    }
}

pub(crate) fn accessibility_button_enabled(
    main: Option<&MainMenuAction>,
    game: Option<&GameMenuAction>,
    has_save: bool,
) -> bool {
    main.is_none_or(|action| main_menu_action_enabled(*action, has_save))
        && game.is_none_or(|action| game_menu_action_enabled(*action, has_save))
}

pub(crate) fn set_accessibility_label(node: &mut AccessibleNode, label: impl Into<String>) {
    node.set_label(label.into().into_boxed_str());
}

#[derive(QueryData)]
#[query_data(mutable)]
pub(crate) struct AccessibleButtonNodeQuery {
    scope: &'static AccessibleButtonScope,
    node: &'static mut AccessibilityNode,
    visibility: Option<&'static Visibility>,
    inherited_visibility: Option<&'static InheritedVisibility>,
    main: Option<&'static MainMenuAction>,
    game: Option<&'static GameMenuAction>,
    settings_tab: Option<&'static SettingsTabButton>,
    settings_value: Option<&'static SettingsValueButton>,
    settings_action: Option<&'static SettingsAction>,
    credits: Option<&'static CreditsSkipButton>,
}

#[allow(clippy::type_complexity)]
pub(crate) fn enhance_accessible_buttons(
    state: Res<State<GameState>>,
    menu: Res<MenuRuntime>,
    save: Res<SaveRuntime>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
    mut buttons: Query<AccessibleButtonNodeQuery>,
) {
    let has_save = save.store.path().is_file();
    #[cfg(target_os = "windows")]
    let broadcast_active = broadcast.snapshot().phase.is_active();
    #[cfg(not(target_os = "windows"))]
    let broadcast_active = false;
    for mut button in &mut buttons {
        let node = &mut button.node;
        node.add_action(AccessAction::Click);
        node.add_action(AccessAction::Focus);
        node.add_action(AccessAction::Blur);
        let visible = !matches!(button.visibility, Some(Visibility::Hidden))
            && button
                .inherited_visibility
                .is_none_or(|visibility| visibility.get())
            && accessibility_scope_active(*button.scope, *state.get(), &menu);
        if visible {
            node.clear_hidden();
        } else {
            node.set_hidden();
        }
        let enabled = accessibility_button_enabled(button.main, button.game, has_save)
            && button
                .settings_value
                .is_none_or(|value| settings_value_enabled(value.index, broadcast_active));
        if enabled {
            node.clear_disabled();
        } else {
            node.set_disabled();
        }
        if let Some(action) = button.main {
            set_accessibility_label(node, main_menu_action_label(*action));
        } else if let Some(action) = button.game {
            set_accessibility_label(node, game_menu_action_label(*action, broadcast_active));
        } else if let Some(tab) = button.settings_tab {
            node.set_role(Role::Tab);
            set_accessibility_label(node, settings_tab_label(tab.0));
            node.set_selected(tab.0 == menu.settings_tab);
        } else if let Some(value_button) = button.settings_value {
            let (label, value) =
                settings_value_label(&menu.draft, &menu.streaming_draft, value_button.index);
            let direction = if value_button.direction < 0 {
                "Previous"
            } else {
                "Next"
            };
            set_accessibility_label(node, format!("{direction} {label}"));
            node.set_value(value.into_boxed_str());
        } else if let Some(action) = button.settings_action {
            set_accessibility_label(
                node,
                match action {
                    SettingsAction::Apply => "Apply and save settings",
                    SettingsAction::Defaults => "Restore default settings",
                    SettingsAction::Back => "Back from settings",
                    SettingsAction::ConfirmApply => "Apply unsaved changes",
                    SettingsAction::ConfirmDiscard => "Discard unsaved changes",
                },
            );
        } else if button.credits.is_some() {
            set_accessibility_label(node, "Skip credits");
        }
    }
}

#[derive(Clone, Copy)]
struct AccessibilityCandidate {
    entity: Entity,
    position: Vec2,
}

#[allow(clippy::type_complexity)]
pub(crate) fn accessibility_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    menu: Res<MenuRuntime>,
    save: Res<SaveRuntime>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut runtime: ResMut<AccessibilityRuntime>,
    mut action_requests: MessageReader<AccessibilityActionRequest>,
    focusable_widgets: Query<(), Or<(With<EditableText>, With<TabIndex>)>>,
    mut buttons: Query<(
        Entity,
        &mut Interaction,
        &AccessibleButtonScope,
        Option<&Visibility>,
        Option<&InheritedVisibility>,
        Option<&UiGlobalTransform>,
        Option<&MainMenuAction>,
        Option<&GameMenuAction>,
    )>,
) {
    if let Some(previous) = runtime.synthetic_pressed.take()
        && let Ok((_, mut interaction, ..)) = buttons.get_mut(previous)
    {
        *interaction = Interaction::None;
    }

    let has_save = save.store.path().is_file();
    let mut candidates = buttons
        .iter_mut()
        .filter_map(
            |(
                entity,
                interaction,
                scope,
                visibility,
                inherited_visibility,
                transform,
                main,
                game,
            )| {
                let visible = !matches!(visibility, Some(Visibility::Hidden))
                    && inherited_visibility.is_none_or(|visibility| visibility.get())
                    && accessibility_scope_active(*scope, *state.get(), &menu);
                let enabled = accessibility_button_enabled(main, game, has_save);
                (visible && enabled).then(|| {
                    let position = transform.map_or(Vec2::ZERO, |transform| {
                        transform.to_scale_angle_translation().2
                    });
                    if *interaction == Interaction::Pressed {
                        focus.set(entity, FocusCause::Pressed);
                        focus_visible.0 = false;
                    }
                    AccessibilityCandidate { entity, position }
                })
            },
        )
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        left.position
            .y
            .total_cmp(&right.position.y)
            .then_with(|| left.position.x.total_cmp(&right.position.x))
            .then_with(|| left.entity.to_bits().cmp(&right.entity.to_bits()))
    });
    let candidate_entities = candidates
        .iter()
        .map(|candidate| candidate.entity)
        .collect::<HashSet<_>>();
    if focus.get().is_some_and(|entity| {
        accessibility_should_clear_focus(
            candidate_entities.contains(&entity),
            focusable_widgets.contains(entity),
        )
    }) {
        focus.clear();
    }

    let mut requested_click = None;
    for request in action_requests.read() {
        let entity = Entity::from_bits(request.target_node.0);
        if !candidate_entities.contains(&entity) {
            continue;
        }
        match request.action {
            AccessAction::Click => requested_click = Some(entity),
            AccessAction::Focus => {
                focus.set(entity, FocusCause::Navigated);
                focus_visible.0 = true;
            }
            AccessAction::Blur if focus.get() == Some(entity) => focus.clear(),
            _ => {}
        }
    }

    // The shipping Game and Settings menus already have complete arrow-key
    // navigation tied to their highlighted row. Keep that deterministic path;
    // this generic traversal covers the main menu, HUD, votes, and credits.
    if menu.page == MenuPage::Closed && !candidates.is_empty() {
        let backwards = keyboard.pressed(KeyCode::ShiftLeft)
            || keyboard.pressed(KeyCode::ShiftRight)
            || keyboard.just_pressed(KeyCode::ArrowUp)
            || keyboard.just_pressed(KeyCode::ArrowLeft);
        let navigation = keyboard.just_pressed(KeyCode::Tab)
            || (focus_visible.0
                && (keyboard.just_pressed(KeyCode::ArrowUp)
                    || keyboard.just_pressed(KeyCode::ArrowDown)
                    || keyboard.just_pressed(KeyCode::ArrowLeft)
                    || keyboard.just_pressed(KeyCode::ArrowRight)));
        if navigation {
            let current = focus.get().and_then(|entity| {
                candidates
                    .iter()
                    .position(|candidate| candidate.entity == entity)
            });
            let next = match (current, backwards) {
                (Some(index), false) => (index + 1) % candidates.len(),
                (Some(index), true) => index.checked_sub(1).unwrap_or(candidates.len() - 1),
                (None, false) => 0,
                (None, true) => candidates.len() - 1,
            };
            focus.set(candidates[next].entity, FocusCause::Navigated);
            focus_visible.0 = true;
        }
        if (keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space))
            && let Some(entity) = focus.get()
            && candidate_entities.contains(&entity)
        {
            requested_click = Some(entity);
        }
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        focus.clear();
        focus_visible.0 = false;
    }
    if let Some(entity) = requested_click
        && let Ok((_, mut interaction, ..)) = buttons.get_mut(entity)
    {
        *interaction = Interaction::Pressed;
        runtime.synthetic_pressed = Some(entity);
    }
}

pub(crate) const fn accessibility_should_clear_focus(
    is_accessibility_candidate: bool,
    is_focusable_widget: bool,
) -> bool {
    !is_accessibility_candidate && !is_focusable_widget
}

type AccessibilityFocusVisualQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static AccessibleButtonScope,
        Option<&'static Visibility>,
        Option<&'static InheritedVisibility>,
        &'static mut Outline,
    ),
>;

pub(crate) fn sync_accessibility_focus_visuals(
    state: Res<State<GameState>>,
    menu: Res<MenuRuntime>,
    settings: Res<RuntimePlayerSettings>,
    focus: Res<InputFocus>,
    focus_visible: Res<InputFocusVisible>,
    mut buttons: AccessibilityFocusVisualQuery,
) {
    for (entity, scope, visibility, inherited_visibility, mut outline) in &mut buttons {
        let visible = !matches!(visibility, Some(Visibility::Hidden))
            && inherited_visibility.is_none_or(|visibility| visibility.get())
            && accessibility_scope_active(*scope, *state.get(), &menu);
        let focused = visible && focus.get() == Some(entity) && focus_visible.0;
        if focused {
            outline.width = px(4);
            outline.offset = px(2);
            outline.color = Color::srgb(1.0, 0.86, 0.16);
        } else if visible && settings.0.interface.high_contrast {
            outline.width = px(1);
            outline.offset = px(1);
            outline.color = Color::srgba(1.0, 1.0, 1.0, 0.82);
        } else {
            outline.width = px(0);
            outline.offset = px(0);
            outline.color = Color::NONE;
        }
    }
}

pub(crate) fn sync_accessibility_contrast(
    mut commands: Commands,
    settings: Res<RuntimePlayerSettings>,
    text: Query<(Entity, Option<&AccessibilityHighContrastText>), With<Text>>,
) {
    for (entity, applied) in &text {
        if settings.0.interface.high_contrast && applied.is_none() {
            commands.entity(entity).insert((
                AccessibilityHighContrastText,
                TextShadow {
                    offset: Vec2::splat(2.0),
                    color: Color::linear_rgba(0.0, 0.0, 0.0, 0.94),
                },
            ));
        } else if !settings.0.interface.high_contrast && applied.is_some() {
            commands
                .entity(entity)
                .remove::<AccessibilityHighContrastText>()
                .remove::<TextShadow>();
        }
    }
}

pub(crate) fn reduced_tree_wind(defaults: Vec4, reduced: bool) -> Vec4 {
    if reduced {
        Vec4::new(defaults.x, 0.0, 0.0, defaults.w)
    } else {
        defaults
    }
}

pub(crate) fn reduced_grass_wind(defaults: Vec4, reduced: bool) -> Vec4 {
    if reduced {
        Vec4::new(0.0, defaults.y, 0.0, defaults.w)
    } else {
        defaults
    }
}

pub(crate) fn reduced_water_wind(defaults: Vec4, reduced: bool) -> Vec4 {
    if reduced {
        Vec4::new(0.0, 0.0, -defaults.z.abs(), defaults.w)
    } else {
        defaults
    }
}

pub(crate) fn sync_accessibility_preferences(
    settings: Res<RuntimePlayerSettings>,
    mut applied: Local<Option<(u16, bool)>>,
    mut ui_scale: Option<ResMut<UiScale>>,
    defaults: Option<Res<AccessibilityMotionDefaults>>,
    render: Option<Res<RenderAssets>>,
    mut tree_materials: Option<ResMut<Assets<TreeMaterial>>>,
    mut grass_materials: Option<ResMut<Assets<GrassMaterial>>>,
    mut water_materials: Option<ResMut<Assets<WaterMaterial>>>,
) {
    let signature = (
        settings.0.interface.ui_scale_percent,
        settings.0.interface.reduced_motion,
    );
    if *applied == Some(signature) {
        return;
    }
    if let Some(ui_scale) = ui_scale.as_deref_mut() {
        ui_scale.0 = f32::from(settings.0.interface.ui_scale_percent) / 100.0;
    }
    let (Some(defaults), Some(render)) = (defaults, render) else {
        return;
    };
    let reduced = settings.0.interface.reduced_motion;
    if let Some(materials) = tree_materials.as_deref_mut()
        && let Some(mut tree) = materials.get_mut(&render.tree)
    {
        tree.extension.parameters.wind_controls = reduced_tree_wind(defaults.tree, reduced);
    }
    if let Some(materials) = grass_materials.as_deref_mut()
        && let Some(mut grass) = materials.get_mut(&render.grass)
    {
        grass.extension.parameters.wind_controls = reduced_grass_wind(defaults.grass, reduced);
    }
    if let Some(materials) = water_materials.as_deref_mut() {
        let wind = reduced_water_wind(defaults.water, reduced);
        if let Some(mut water) = materials.get_mut(&render.water) {
            water.extension.parameters.wind_speed_noise_alpha = wind;
        }
        if let Some(mut menu_water) = materials.get_mut(&render.menu_water) {
            menu_water.extension.parameters.wind_speed_noise_alpha = wind;
        }
    }
    *applied = Some(signature);
}

pub(crate) fn accessibility_settings_selection(menu: &MenuRuntime) -> String {
    if menu.confirm_settings_close {
        return if menu.selected == 0 {
            "Apply unsaved changes".to_owned()
        } else {
            "Discard unsaved changes".to_owned()
        };
    }
    if menu.selected < SETTINGS_APPLY_INDEX {
        let (label, value) =
            settings_value_label(&menu.draft, &menu.streaming_draft, menu.selected);
        format!("{label}: {value}")
    } else {
        match menu.selected {
            SETTINGS_APPLY_INDEX => "Apply and save settings".to_owned(),
            SETTINGS_DEFAULTS_INDEX => "Restore default settings".to_owned(),
            SETTINGS_BACK_INDEX => "Back from settings".to_owned(),
            _ => "Settings".to_owned(),
        }
    }
}

pub(crate) fn announce_accessibility_state(
    state: Res<State<GameState>>,
    menu: Res<MenuRuntime>,
    loading: Option<Res<WorldLoadingRuntime>>,
    command_feedback: Res<CommandFeedback>,
    mut runtime: ResMut<AccessibilityRuntime>,
    mut announcements: Query<&mut AccessibilityNode, With<AccessibilityAnnouncement>>,
) {
    let message = match *state.get() {
        GameState::Boot => "Loading Stream Town".to_owned(),
        GameState::MainMenu | GameState::InGame if menu.page == MenuPage::Settings => format!(
            "Settings, {} tab. {}",
            settings_tab_label(menu.settings_tab),
            accessibility_settings_selection(&menu)
        ),
        GameState::MainMenu if menu.page == MenuPage::SecretsDisclaimer => {
            "Sensitive Twitch setup disclaimer. Choose Yes to continue or No to return.".to_owned()
        }
        GameState::MainMenu if menu.page == MenuPage::GoLiveConfirmation => {
            "Go-live confirmation. Choose Yes to start the town and begin streaming after loading, or No to remain on the main menu.".to_owned()
        }
        GameState::MainMenu if menu.page == MenuPage::Secrets => {
            "Twitch secrets setup. Internal Twitch video is blacked out. Bot and broadcaster accounts are authorized separately.".to_owned()
        }
        GameState::MainMenu if menu.page == MenuPage::NewTown => {
            "Name your town. The name determines its world seed and save file.".to_owned()
        }
        GameState::MainMenu if menu.page == MenuPage::LoadTown => {
            "Choose a saved town to load.".to_owned()
        }
        GameState::MainMenu => "Main menu. Press Tab to move between actions.".to_owned(),
        GameState::WorldLoading => loading.as_ref().map_or_else(
            || "Loading town".to_owned(),
            |loading| format!("{}. {}", loading.status, loading.substatus),
        ),
        GameState::InGame if menu.page == MenuPage::Game => {
            "Game menu. Use arrow keys and Enter.".to_owned()
        }
        GameState::InGame if !command_feedback.0.is_empty() => command_feedback.0.clone(),
        GameState::InGame => {
            "Town ready. Text commands are active; press Escape for the game menu.".to_owned()
        }
        GameState::Credits => {
            "Credits. Press Escape or activate Skip credits to return.".to_owned()
        }
    };
    if runtime.last_announcement == message {
        return;
    }
    runtime.last_announcement.clone_from(&message);
    if let Ok(mut node) = announcements.single_mut() {
        node.set_value(message.into_boxed_str());
    }
}
