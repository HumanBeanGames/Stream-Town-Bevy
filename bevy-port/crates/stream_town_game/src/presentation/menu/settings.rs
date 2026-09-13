pub(crate) fn update_game_menu_controls(
    save: Res<SaveRuntime>,
    menu: Res<MenuRuntime>,
    render: Res<RenderAssets>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
    mut buttons: Query<(&Interaction, &GameMenuAction, &mut ImageNode)>,
    mut labels: Query<(&GameMenuActionLabel, &mut Text)>,
) {
    let has_save = save.store.path().is_file();
    #[cfg(target_os = "windows")]
    let broadcast_active = broadcast.snapshot().phase.is_active();
    #[cfg(not(target_os = "windows"))]
    let broadcast_active = false;
    for (label, mut text) in &mut labels {
        let desired = game_menu_action_label(label.0, broadcast_active);
        if text.0 != desired {
            desired.clone_into(&mut text.0);
        }
    }
    for (interaction, action, mut image) in &mut buttons {
        let enabled = menu.page == MenuPage::Game && game_menu_action_enabled(*action, has_save);
        let selected =
            menu.page == MenuPage::Game && menu.selected == game_menu_action_index(*action);
        let source_path = if !enabled {
            MAIN_MENU_TEXTURE_PATHS[2]
        } else if selected
            || *interaction == Interaction::Hovered
            || *interaction == Interaction::Pressed
        {
            MAIN_MENU_TEXTURE_PATHS[1]
        } else {
            MAIN_MENU_TEXTURE_PATHS[0]
        };
        image.image = main_menu_texture(&render, source_path);
        image.color = if enabled {
            Color::WHITE
        } else {
            Color::srgba(0.78, 0.78, 0.78, 0.5)
        };
    }
}

pub(crate) const SETTINGS_TABS: [SettingsTab; 6] = [
    SettingsTab::Video,
    SettingsTab::Audio,
    SettingsTab::Gameplay,
    SettingsTab::Accessibility,
    SettingsTab::Streaming,
    SettingsTab::Connection,
];

pub(crate) const fn settings_tab_label(tab: SettingsTab) -> &'static str {
    match tab {
        SettingsTab::Video => "Video",
        SettingsTab::Audio => "Audio",
        SettingsTab::Gameplay => "Gameplay",
        SettingsTab::Accessibility => "Accessibility",
        SettingsTab::Streaming => "Streaming",
        SettingsTab::Connection => "Connection",
    }
}

pub(crate) fn settings_tab_indices(tab: SettingsTab) -> &'static [usize] {
    match tab {
        SettingsTab::Video => &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        SettingsTab::Audio => &[11, 12, 13, 14],
        SettingsTab::Gameplay => &[15, 16, 17, 18, 19, 20, 21],
        SettingsTab::Accessibility => &[22, 23, 24],
        SettingsTab::Streaming => &[25, 26, 27, 28, 29, 30, 31, 32],
        SettingsTab::Connection => &[],
    }
}

pub(crate) const fn settings_index_is_streaming(index: usize) -> bool {
    index >= SETTINGS_STREAMING_FIRST_INDEX && index <= SETTINGS_STREAMING_LAST_INDEX
}

pub(crate) const fn settings_value_enabled(index: usize, streaming_locked: bool) -> bool {
    !streaming_locked || !settings_index_is_streaming(index)
}

pub(crate) const fn settings_tab_for_index(index: usize) -> Option<SettingsTab> {
    match index {
        0..=10 => Some(SettingsTab::Video),
        11..=14 => Some(SettingsTab::Audio),
        15..=21 => Some(SettingsTab::Gameplay),
        22..=24 => Some(SettingsTab::Accessibility),
        SETTINGS_STREAMING_FIRST_INDEX..=SETTINGS_STREAMING_LAST_INDEX => {
            Some(SettingsTab::Streaming)
        }
        _ => None,
    }
}

pub(crate) fn cycle_settings_tab(tab: SettingsTab, forward: bool) -> SettingsTab {
    let index = SETTINGS_TABS
        .iter()
        .position(|candidate| *candidate == tab)
        .expect("the active settings tab is part of the shipping tab order");
    let next = if forward {
        (index + 1) % SETTINGS_TABS.len()
    } else {
        index.checked_sub(1).unwrap_or(SETTINGS_TABS.len() - 1)
    };
    SETTINGS_TABS[next]
}

pub(crate) fn settings_value_label(
    settings: &PlayerSettings,
    streaming: &BroadcastConfig,
    index: usize,
) -> (&'static str, String) {
    let video = &settings.video;
    let camera = &settings.camera;
    let interface = &settings.interface;
    match index {
        0 => ("Display Mode", format!("{:?}", video.display_mode)),
        1 => ("Resolution", format!("{} x {}", video.width, video.height)),
        2 => ("VSync", on_off(video.vsync).to_owned()),
        3 => (
            "FPS Limiter",
            video
                .fps_limit
                .map_or("Unlimited".to_owned(), |value| value.to_string()),
        ),
        4 => ("Shadows", on_off(video.shadows_enabled).to_owned()),
        5 => ("Shadow Quality", video.shadow_map_resolution.to_string()),
        6 => (
            "Ambient Occlusion",
            on_off(video.ambient_occlusion).to_owned(),
        ),
        7 => ("MSAA", format!("x{}", video.msaa_samples)),
        8 => ("FXAA / SMAA", format!("{:?}", video.post_process_aa)),
        9 => ("Brightness", format!("{:.1}", video.brightness_ev)),
        10 => ("Gamma", format!("{:.1}", video.gamma)),
        11 => (
            "Master",
            format!("{}%", volume_percent(settings.audio.master)),
        ),
        12 => (
            "Music",
            format!("{}%", volume_percent(settings.audio.music)),
        ),
        13 => (
            "Sound Effects",
            format!("{}%", volume_percent(settings.audio.sound_effects)),
        ),
        14 => (
            "Ambience",
            format!("{}%", volume_percent(settings.audio.ambience)),
        ),
        15 => (
            "Zoom Sensitivity",
            format!("{:.0}", camera.zoom_sensitivity),
        ),
        16 => ("Field of View", camera.field_of_view_degrees.to_string()),
        17 => ("Username Display", format!("{:?}", interface.display_names)),
        18 => (
            "Building Health Display",
            format!("{:?}", interface.display_building_health),
        ),
        19 => (
            "Autosave Time",
            if settings.autosave_minutes == 0 {
                "Disabled".to_owned()
            } else {
                format!("{} Minutes", settings.autosave_minutes)
            },
        ),
        20 => (
            "Timelapse Frequency",
            timelapse_interval_label(settings.timelapse.interval).to_owned(),
        ),
        21 => (
            "Timelapse Dynamic",
            on_off(settings.timelapse.dynamic).to_owned(),
        ),
        22 => ("UI Scale", format!("{}%", interface.ui_scale_percent)),
        23 => ("High Contrast", on_off(interface.high_contrast).to_owned()),
        24 => (
            "Reduced Motion",
            on_off(interface.reduced_motion).to_owned(),
        ),
        25 => ("Direct Streaming", on_off(streaming.enabled).to_owned()),
        26 => (
            "Output Resolution",
            format!("{} x {}", streaming.width, streaming.height),
        ),
        27 => ("Frame Rate", format!("{} FPS", streaming.frames_per_second)),
        28 => (
            "Video Bitrate",
            format!("{} kbps", streaming.video_bitrate_kbps),
        ),
        29 => (
            "Audio Bitrate",
            format!("{} kbps", streaming.audio_bitrate_kbps),
        ),
        30 => ("Encoder", format!("{:?}", streaming.encoder)),
        31 => (
            "Bandwidth Test",
            on_off(streaming.bandwidth_test).to_owned(),
        ),
        32 => ("Render Mode", format!("{:?}", streaming.render_mode)),
        _ => ("Unknown", String::new()),
    }
}

pub(crate) fn spawn_settings_button(
    parent: &mut ChildSpawnerCommands,
    action: SettingsAction,
    label: &'static str,
    render: &RenderAssets,
) {
    parent
        .spawn((
            action,
            Button,
            settings_ui_image(
                render,
                MAIN_MENU_TEXTURE_PATHS[0],
                main_menu_texture(render, MAIN_MENU_TEXTURE_PATHS[0]),
            ),
            Node {
                flex_grow: 1.0,
                height: px(44),
                padding: UiRect::axes(px(18), px(10)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                Pickable::IGNORE,
            ));
        });
}

pub(crate) fn spawn_settings_value_row(
    parent: &mut ChildSpawnerCommands,
    settings: &PlayerSettings,
    streaming: &BroadcastConfig,
    selected: usize,
    index: usize,
    enabled: bool,
    render: &RenderAssets,
) {
    let (label, value) = settings_value_label(settings, streaming, index);
    parent
        .spawn((
            SettingsValueRow(index),
            BackgroundColor(if selected == index && enabled {
                Color::srgb(0.211, 0.240, 0.358)
            } else {
                Color::srgb(0.055, 0.071, 0.141)
            }),
            Node {
                width: percent(100.0),
                height: px(48),
                flex_shrink: 0.0,
                padding: UiRect::axes(px(14), px(6)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                overflow: Overflow::clip(),
                ..default()
            },
        ))
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextColor(if enabled {
                    Color::srgb(0.827, 0.745, 0.498)
                } else {
                    Color::srgb(0.48, 0.48, 0.52)
                }),
                Node {
                    width: percent(43.0),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
            for direction in [-1, 1] {
                if direction == 1 {
                    row.spawn((
                        SettingsValueText(index),
                        Text::new(value.clone()),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(if enabled {
                            Color::srgb(0.90, 0.88, 0.80)
                        } else {
                            Color::srgb(0.56, 0.56, 0.60)
                        }),
                        Pickable::IGNORE,
                        Node {
                            width: percent(30.0),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                    ));
                }
                row.spawn((
                    SettingsValueButton { index, direction },
                    Button,
                    settings_ui_image(
                        render,
                        MAIN_MENU_TEXTURE_PATHS[0],
                        main_menu_texture(render, MAIN_MENU_TEXTURE_PATHS[0]),
                    ),
                    Node {
                        width: px(36),
                        height: px(34),
                        flex_shrink: 0.0,
                        padding: UiRect::axes(px(8), px(6)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(if direction < 0 { "<" } else { ">" }),
                        TextColor(if enabled {
                            Color::WHITE
                        } else {
                            Color::srgb(0.42, 0.42, 0.46)
                        }),
                        Pickable::IGNORE,
                    ));
                });
            }
        });
}

pub(crate) fn settings_tab_buttons(
    mut menu: ResMut<MenuRuntime>,
    buttons: Query<(&Interaction, &SettingsTabButton), Changed<Interaction>>,
) {
    if menu.page != MenuPage::Settings || menu.confirm_settings_close {
        return;
    }
    for (interaction, tab) in &buttons {
        if *interaction == Interaction::Pressed {
            menu.settings_tab = tab.0;
            menu.selected = settings_tab_indices(tab.0)
                .first()
                .copied()
                .unwrap_or(SETTINGS_APPLY_INDEX);
            menu.feedback.clear();
        }
    }
}

pub(crate) fn settings_value_buttons(
    mut menu: ResMut<MenuRuntime>,
    buttons: Query<(&Interaction, &SettingsValueButton), Changed<Interaction>>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
) {
    if menu.page != MenuPage::Settings || menu.confirm_settings_close {
        return;
    }
    for (interaction, button) in &buttons {
        if *interaction == Interaction::Pressed {
            menu.selected = button.index;
            #[cfg(target_os = "windows")]
            let streaming_locked = broadcast.snapshot().phase.is_active();
            #[cfg(not(target_os = "windows"))]
            let streaming_locked = false;
            if !settings_value_enabled(button.index, streaming_locked) {
                "End the stream before changing streaming settings.".clone_into(&mut menu.feedback);
                continue;
            }
            let menu = &mut *menu;
            adjust_settings_menu(
                &mut menu.draft,
                &mut menu.streaming_draft,
                button.index,
                button.direction,
            );
            menu.feedback.clear();
        }
    }
}

pub(crate) fn apply_settings_draft(
    menu: &mut MenuRuntime,
    player_settings: &mut RuntimePlayerSettings,
    config: &mut RuntimeConfig,
    streaming_locked: bool,
) -> bool {
    if let Err(error) = menu.draft.validate() {
        menu.feedback = format!("Settings are invalid: {error}");
        return false;
    }
    if streaming_locked && menu.streaming_draft != config.0.twitch.broadcast {
        "End the stream before applying streaming settings.".clone_into(&mut menu.feedback);
        return false;
    }
    let mut config_draft = config.0.clone();
    config_draft.twitch.broadcast = menu.streaming_draft.clone();
    if let Err(error) = config_draft.validate() {
        menu.feedback = format!("Streaming settings are invalid: {error}");
        return false;
    }
    match PlayerSettingsStore::new(player_settings_path()).write(&menu.draft) {
        Ok(()) => match save_runtime_config(&config_draft) {
            Ok(_) => {
                config.0 = config_draft;
                player_settings.0 = menu.draft.clone();
                "Applied and saved settings".clone_into(&mut menu.feedback);
                true
            }
            Err(error) => {
                menu.feedback = format!("Streaming settings could not be saved: {error}");
                false
            }
        },
        Err(error) => {
            menu.feedback = format!("Settings could not be saved: {error}");
            false
        }
    }
}

pub(crate) fn close_settings_menu(menu: &mut MenuRuntime) {
    let target = menu.return_page;
    menu.page = target;
    if target == MenuPage::Game {
        menu.return_page = MenuPage::Closed;
    }
    menu.selected = 0;
    menu.settings_tab = SettingsTab::Video;
    menu.confirm_settings_close = false;
    menu.feedback.clear();
}

pub(crate) fn request_settings_close(
    menu: &mut MenuRuntime,
    player_settings: &PlayerSettings,
    streaming: &BroadcastConfig,
) {
    if menu.draft == *player_settings && menu.streaming_draft == *streaming {
        close_settings_menu(menu);
    } else {
        menu.confirm_settings_close = true;
        menu.selected = 0;
        menu.feedback.clear();
    }
}

pub(crate) fn settings_action_buttons(
    mut menu: ResMut<MenuRuntime>,
    mut player_settings: ResMut<RuntimePlayerSettings>,
    mut config: ResMut<RuntimeConfig>,
    buttons: Query<(&Interaction, &SettingsAction), Changed<Interaction>>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
) {
    if menu.page != MenuPage::Settings {
        return;
    }
    for (interaction, action) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        #[cfg(target_os = "windows")]
        let streaming_locked = broadcast.snapshot().phase.is_active();
        #[cfg(not(target_os = "windows"))]
        let streaming_locked = false;
        match action {
            SettingsAction::Apply if !menu.confirm_settings_close => {
                apply_settings_draft(
                    &mut menu,
                    &mut player_settings,
                    &mut config,
                    streaming_locked,
                );
            }
            SettingsAction::Defaults if !menu.confirm_settings_close => {
                menu.draft = PlayerSettings::default();
                if !streaming_locked {
                    menu.streaming_draft = BroadcastConfig::default();
                }
                "Restored default settings in this draft".clone_into(&mut menu.feedback);
            }
            SettingsAction::Back if !menu.confirm_settings_close => {
                request_settings_close(&mut menu, &player_settings.0, &config.0.twitch.broadcast);
            }
            SettingsAction::ConfirmApply
                if menu.confirm_settings_close
                    && apply_settings_draft(
                        &mut menu,
                        &mut player_settings,
                        &mut config,
                        streaming_locked,
                    ) =>
            {
                close_settings_menu(&mut menu);
            }
            SettingsAction::ConfirmDiscard if menu.confirm_settings_close => {
                menu.draft = player_settings.0.clone();
                menu.streaming_draft = config.0.twitch.broadcast.clone();
                close_settings_menu(&mut menu);
            }
            _ => {}
        }
    }
}

pub(crate) fn rebuild_settings_rows(
    mut commands: Commands,
    menu: Res<MenuRuntime>,
    twitch: Res<TwitchConnection>,
    render: Res<RenderAssets>,
    mut cache: ResMut<SettingsUiCache>,
    rows: Query<Entity, With<SettingsRows>>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
) {
    #[cfg(target_os = "windows")]
    let streaming_locked = broadcast.snapshot().phase.is_active();
    #[cfg(not(target_os = "windows"))]
    let streaming_locked = false;
    let signature = settings_rows_signature(&menu, &twitch, streaming_locked);
    if cache.signature == signature {
        return;
    }
    cache.signature = signature;
    let Ok(rows) = rows.single() else {
        return;
    };
    commands.entity(rows).despawn_children();
    commands.entity(rows).with_children(|parent| {
        if menu.settings_tab == SettingsTab::Connection {
            parent.spawn((
                Text::new(format!(
                    "CONNECTION\n\nTwitch: {}\n\nOAuth setup and credential storage are managed in stream_town_tools.\nThe game reconnects automatically from the saved public configuration.",
                    twitch_status_text(&twitch)
                )),
                TextFont {
                    font_size: FontSize::Px(19.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::WHITE),
                Node {
                    width: percent(100.0),
                    margin: UiRect::top(percent(8.0)),
                    ..default()
                },
            ));
            return;
        }
        if menu.settings_tab == SettingsTab::Streaming {
            parent.spawn((
                Text::new(if streaming_locked {
                    "STREAMING IS LIVE — END THE STREAM TO CHANGE THESE SETTINGS"
                } else {
                    "Changes are saved for the next stream. Preferred ingest remains in Secrets."
                }),
                TextFont {
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(if streaming_locked {
                    Color::srgb(1.0, 0.54, 0.38)
                } else {
                    Color::srgb(0.76, 0.82, 0.92)
                }),
                Pickable::IGNORE,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0),
                    top: px(0),
                    width: percent(100.0),
                    height: px(30),
                    ..default()
                },
            ));
        }
        let indices = settings_tab_indices(menu.settings_tab);
        let split = indices.len().div_ceil(2);
        for column_indices in [&indices[..split], &indices[split..]] {
            parent
                .spawn(Node {
                    width: percent(48.0),
                    height: percent(100.0),
                    margin: UiRect::top(if menu.settings_tab == SettingsTab::Streaming {
                        px(38)
                    } else {
                        px(0)
                    }),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(8),
                    overflow: Overflow::clip(),
                    ..default()
                })
                .with_children(|column| {
                    for &index in column_indices {
                        spawn_settings_value_row(
                            column,
                            &menu.draft,
                            &menu.streaming_draft,
                            menu.selected,
                            index,
                            settings_value_enabled(index, streaming_locked),
                            &render,
                        );
                    }
                });
        }
    });
}

pub(crate) fn settings_rows_signature(
    menu: &MenuRuntime,
    twitch: &TwitchConnection,
    streaming_locked: bool,
) -> String {
    let connection_status = if menu.settings_tab == SettingsTab::Connection {
        twitch_status_text(twitch)
    } else {
        String::new()
    };
    format!(
        "{:?}:{:?}:{streaming_locked}:{connection_status}",
        menu.page, menu.settings_tab,
    )
}

pub(crate) fn update_settings_value_rows(
    menu: Res<MenuRuntime>,
    mut rows: Query<(&SettingsValueRow, &mut BackgroundColor)>,
    mut values: Query<(&SettingsValueText, &mut Text)>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
) {
    if menu.page != MenuPage::Settings || menu.settings_tab == SettingsTab::Connection {
        return;
    }
    #[cfg(target_os = "windows")]
    let streaming_locked = broadcast.snapshot().phase.is_active();
    #[cfg(not(target_os = "windows"))]
    let streaming_locked = false;
    for (row, mut background) in &mut rows {
        let enabled = settings_value_enabled(row.0, streaming_locked);
        let desired = BackgroundColor(if menu.selected == row.0 && enabled {
            Color::srgb(0.211, 0.240, 0.358)
        } else {
            Color::srgb(0.055, 0.071, 0.141)
        });
        if *background != desired {
            *background = desired;
        }
    }
    for (value_text, mut text) in &mut values {
        let (_, desired) = settings_value_label(&menu.draft, &menu.streaming_draft, value_text.0);
        if text.0 != desired {
            text.0 = desired;
        }
    }
}

pub(crate) fn update_settings_controls(
    menu: Res<MenuRuntime>,
    render: Res<RenderAssets>,
    mut root: Query<&mut Visibility, (With<SettingsRoot>, Without<SettingsConfirmModal>)>,
    mut modal: Query<&mut Visibility, (With<SettingsConfirmModal>, Without<SettingsRoot>)>,
    mut tabs: Query<(&Interaction, &SettingsTabButton, &mut BackgroundColor)>,
    mut actions: Query<
        (&Interaction, &SettingsAction, &mut ImageNode),
        Without<SettingsValueButton>,
    >,
    mut values: Query<
        (&Interaction, &SettingsValueButton, &mut ImageNode),
        Without<SettingsAction>,
    >,
    mut feedback: Query<&mut Text, With<SettingsFeedbackText>>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
) {
    #[cfg(target_os = "windows")]
    let streaming_locked = broadcast.snapshot().phase.is_active();
    #[cfg(not(target_os = "windows"))]
    let streaming_locked = false;
    if let Ok(mut visibility) = root.single_mut() {
        *visibility = if menu.page == MenuPage::Settings {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if let Ok(mut visibility) = modal.single_mut() {
        *visibility = if menu.page == MenuPage::Settings && menu.confirm_settings_close {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for (interaction, tab, mut background) in &mut tabs {
        background.0 = if tab.0 == menu.settings_tab
            || *interaction == Interaction::Hovered
            || *interaction == Interaction::Pressed
        {
            Color::srgba(0.211, 0.240, 0.358, 0.95)
        } else {
            Color::srgba(0.055, 0.071, 0.141, 0.95)
        };
    }
    for (interaction, action, mut image) in &mut actions {
        let modal_action = matches!(
            action,
            SettingsAction::ConfirmApply | SettingsAction::ConfirmDiscard
        );
        let enabled =
            menu.page == MenuPage::Settings && (modal_action == menu.confirm_settings_close);
        let keyboard_selected = match action {
            SettingsAction::Apply => menu.selected == SETTINGS_APPLY_INDEX,
            SettingsAction::Defaults => menu.selected == SETTINGS_DEFAULTS_INDEX,
            SettingsAction::Back => menu.selected == SETTINGS_BACK_INDEX,
            SettingsAction::ConfirmApply => menu.confirm_settings_close && menu.selected == 0,
            SettingsAction::ConfirmDiscard => menu.confirm_settings_close && menu.selected == 1,
        };
        let source_path = if !enabled {
            MAIN_MENU_TEXTURE_PATHS[2]
        } else if keyboard_selected
            || *interaction == Interaction::Hovered
            || *interaction == Interaction::Pressed
        {
            MAIN_MENU_TEXTURE_PATHS[1]
        } else {
            MAIN_MENU_TEXTURE_PATHS[0]
        };
        image.image = main_menu_texture(&render, source_path);
    }
    for (interaction, button, mut image) in &mut values {
        let enabled = settings_value_enabled(button.index, streaming_locked);
        let source_path = if !enabled {
            MAIN_MENU_TEXTURE_PATHS[2]
        } else if menu.selected == button.index
            || *interaction == Interaction::Hovered
            || *interaction == Interaction::Pressed
        {
            MAIN_MENU_TEXTURE_PATHS[1]
        } else {
            MAIN_MENU_TEXTURE_PATHS[0]
        };
        image.image = main_menu_texture(&render, source_path);
        image.color = if enabled {
            Color::WHITE
        } else {
            Color::srgba(0.55, 0.55, 0.58, 0.55)
        };
    }
    if let Ok(mut text) = feedback.single_mut() {
        (**text).clone_from(&menu.feedback);
    }
}
