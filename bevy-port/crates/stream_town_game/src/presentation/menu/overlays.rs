pub(crate) fn spawn_menu_overlay(
    mut commands: Commands,
    state: Res<State<GameState>>,
    mut menu: ResMut<MenuRuntime>,
    config: Res<RuntimeConfig>,
    settings: Res<RuntimePlayerSettings>,
    render: Res<RenderAssets>,
    town_catalog: Res<TownSaveCatalogRuntime>,
    mut settings_ui: ResMut<SettingsUiCache>,
) {
    settings_ui.signature.clear();
    if std::env::var_os("STREAM_TOWN_AUTOSTART_GAME_MENU").is_some()
        && *state.get() == GameState::InGame
    {
        menu.page = MenuPage::Game;
        menu.return_page = MenuPage::Closed;
        menu.selected = 0;
        menu.feedback.clear();
    } else if std::env::var_os("STREAM_TOWN_AUTOSTART_SETTINGS").is_some() {
        open_settings_menu(
            &mut menu,
            MenuPage::Game,
            &settings.0,
            &config.0.twitch.broadcast,
        );
        if std::env::var("STREAM_TOWN_AUTOSTART_SETTINGS_TAB")
            .is_ok_and(|tab| tab.eq_ignore_ascii_case("streaming"))
        {
            menu.settings_tab = SettingsTab::Streaming;
            menu.selected = SETTINGS_STREAMING_FIRST_INDEX;
        }
    } else if std::env::var_os("STREAM_TOWN_AUTOSTART_SECRETS_DISCLAIMER").is_some() {
        menu.page = MenuPage::SecretsDisclaimer;
        menu.return_page = MenuPage::Closed;
        menu.selected = 0;
        menu.feedback.clear();
    }
    commands.spawn((
        StateEntity,
        MenuOverlay,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::srgb(0.92, 0.97, 0.91)),
        Visibility::Hidden,
        GlobalZIndex(100),
        Node {
            position_type: PositionType::Absolute,
            left: percent(5.0),
            top: percent(3.0),
            width: percent(90.0),
            min_height: percent(90.0),
            padding: UiRect::all(px(28)),
            border: UiRect::all(px(2)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.025, 0.045, 0.04, 0.97)),
        BorderColor::all(Color::srgb(0.42, 0.76, 0.52)),
    ));
    commands
        .spawn((
            StateEntity,
            GameMenuRoot,
            Name::new("Shipping in-game menu"),
            Visibility::Hidden,
            GlobalZIndex(100),
            Node {
                position_type: PositionType::Absolute,
                left: percent(37.5),
                top: percent(25.0),
                width: percent(25.0),
                height: percent(50.0),
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                Name::new("Game menu background"),
                authored_main_ui_image_with_ppu(
                    &render,
                    GAME_MENU_TEXTURE_PATHS[0],
                    main_menu_texture(&render, GAME_MENU_TEXTURE_PATHS[0]),
                    1.5,
                ),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(20.35),
                    top: percent(17.08),
                    width: percent(59.45),
                    height: percent(61.62),
                    ..default()
                },
            ));
            root.spawn((
                Name::new("Game menu controls"),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(29.05),
                    top: percent(22.7),
                    width: percent(42.05),
                    height: percent(50.1),
                    flex_direction: FlexDirection::Column,
                    row_gap: percent(2.4),
                    ..default()
                },
            ))
            .with_children(|controls| {
                for action in [
                    GameMenuAction::SaveGame,
                    GameMenuAction::SaveJumpStart,
                    GameMenuAction::LoadGame,
                    GameMenuAction::Settings,
                    GameMenuAction::GoLive,
                    GameMenuAction::ExitGame,
                ] {
                    controls
                        .spawn((
                            action,
                            Button,
                            authored_ui_image(
                                &render,
                                MAIN_MENU_TEXTURE_PATHS[0],
                                main_menu_texture(&render, MAIN_MENU_TEXTURE_PATHS[0]),
                            ),
                            Node {
                                width: percent(100.0),
                                flex_grow: 1.0,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                        ))
                        .with_children(|button| {
                            button.spawn((
                                GameMenuActionLabel(action),
                                Text::new(game_menu_action_label(action, false)),
                                TextFont {
                                    font_size: FontSize::Px(18.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                                Pickable::IGNORE,
                            ));
                        });
                }
            });
            root.spawn((
                GameMenuAction::Close,
                Button,
                Name::new("Game menu close"),
                ImageNode::new(main_menu_texture(&render, MAIN_MENU_TEXTURE_PATHS[1])),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(69.1),
                    top: percent(15.2),
                    width: percent(12.62),
                    height: percent(11.28),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|close| {
                close.spawn((
                    ImageNode::new(main_menu_texture(&render, GAME_MENU_TEXTURE_PATHS[1])),
                    Pickable::IGNORE,
                    Node {
                        width: percent(45.0),
                        height: percent(45.0),
                        ..default()
                    },
                ));
            });
        });
    commands
        .spawn((
            StateEntity,
            SettingsRoot,
            Name::new("Settings modal overlay"),
            BackgroundColor(Color::srgba(0.008, 0.012, 0.025, 0.58)),
            Visibility::Hidden,
            GlobalZIndex(110),
            Node {
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                width: percent(100.0),
                height: percent(100.0),
                padding: UiRect::all(px(24)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|settings_overlay| {
            settings_overlay
                .spawn((
                    SettingsPanel,
                    Name::new("Shipping settings menu"),
                    BackgroundColor(Color::srgb(0.035, 0.046, 0.09)),
                    BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                    Node {
                        width: percent(78.0),
                        height: percent(80.0),
                        max_width: px(1_420),
                        max_height: px(800),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(36)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                ))
                .with_children(|settings_root| {
            settings_root.spawn((
                Name::new("Settings panel nine-slice surface"),
                settings_panel_ui_image(
                    &render,
                    SETTINGS_BACKGROUND_TEXTURE_PATH,
                    main_menu_texture(&render, SETTINGS_BACKGROUND_TEXTURE_PATH),
                ),
                Pickable::IGNORE,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(3),
                    top: px(3),
                    right: px(3),
                    bottom: px(3),
                    ..default()
                },
            ));
            settings_root.spawn((
                UiDisplayFont,
                Text::new("SETTINGS"),
                TextFont {
                    font_size: FontSize::Px(34.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(3.0),
                    top: percent(3.0),
                    width: percent(16.0),
                    height: percent(7.0),
                    ..default()
                },
            ));
            settings_root
                .spawn((
                    Name::new("Settings tab buttons"),
                    Node {
                        position_type: PositionType::Absolute,
                        left: percent(3.0),
                        top: percent(12.0),
                        width: percent(16.0),
                        height: percent(42.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(10),
                        ..default()
                    },
                ))
                .with_children(|tabs| {
                    for tab in SETTINGS_TABS {
                        tabs.spawn((
                            SettingsTabButton(tab),
                            Button,
                            BackgroundColor(Color::srgb(0.055, 0.071, 0.141)),
                            BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                            Node {
                                width: percent(100.0),
                                flex_grow: 1.0,
                                border: UiRect::bottom(px(2)),
                                padding: UiRect::axes(px(18), px(10)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new(settings_tab_label(tab)),
                                TextFont {
                                    font_size: FontSize::Px(19.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                                Pickable::IGNORE,
                            ));
                        });
                    }
                });
            settings_root.spawn((
                SettingsRows,
                Name::new("Settings value rows"),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(23.0),
                    top: percent(8.0),
                    width: percent(72.0),
                    height: percent(82.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: percent(4.0),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
            settings_root.spawn((
                SettingsFeedbackText,
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.92, 0.97, 0.91)),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(23.0),
                    bottom: percent(4.0),
                    width: percent(72.0),
                    height: px(22),
                    ..default()
                },
            ));
            settings_root
                .spawn((
                    Name::new("Settings actions"),
                    Node {
                        position_type: PositionType::Absolute,
                        left: percent(3.0),
                        bottom: percent(7.0),
                        width: percent(16.0),
                        height: percent(27.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(10),
                        ..default()
                    },
                ))
                .with_children(|actions| {
                    spawn_settings_button(actions, SettingsAction::Apply, "Apply", &render);
                    spawn_settings_button(actions, SettingsAction::Defaults, "Defaults", &render);
                    spawn_settings_button(actions, SettingsAction::Back, "Back", &render);
                });
            settings_root
                .spawn((
                    SettingsConfirmModal,
                    Name::new("Confirm settings changes"),
                    Visibility::Hidden,
                    GlobalZIndex(120),
                    BackgroundColor(Color::srgb(0.035, 0.046, 0.09)),
                    BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                    Node {
                        position_type: PositionType::Absolute,
                        left: percent(25.0),
                        top: percent(27.0),
                        width: percent(50.0),
                        height: percent(38.0),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(28)),
                        padding: UiRect::all(percent(5.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                ))
                .with_children(|modal| {
                    modal.spawn((
                        Name::new("Settings confirmation nine-slice surface"),
                        settings_panel_ui_image(
                            &render,
                            SETTINGS_BACKGROUND_TEXTURE_PATH,
                            main_menu_texture(&render, SETTINGS_BACKGROUND_TEXTURE_PATH),
                        ),
                        Pickable::IGNORE,
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(3),
                            top: px(3),
                            right: px(3),
                            bottom: px(3),
                            ..default()
                        },
                    ));
                    modal.spawn((
                        Text::new("CONFIRM CHANGES\n\nYou have unsaved changes.\nDo you want to apply these changes?"),
                        TextFont {
                            font_size: FontSize::Px(19.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::WHITE),
                    ));
                    modal
                        .spawn((
                            Node {
                                width: percent(100.0),
                                height: px(48),
                                column_gap: px(18),
                                ..default()
                            },
                        ))
                        .with_children(|buttons| {
                            spawn_settings_button(
                                buttons,
                                SettingsAction::ConfirmApply,
                                "Yes",
                                &render,
                            );
                            spawn_settings_button(
                                buttons,
                                SettingsAction::ConfirmDiscard,
                                "No",
                                &render,
                            );
                        });
                });
                });
        });
    spawn_secrets_overlays(&mut commands, &render, &config.0);
    spawn_go_live_confirmation(&mut commands, &render);
    spawn_town_dialogs(&mut commands, &render, &town_catalog.entries());
}

pub(crate) fn spawn_town_dialog_button(
    parent: &mut ChildSpawnerCommands,
    action: TownDialogAction,
    label: &str,
    render: &RenderAssets,
    tab_index: i32,
) {
    parent
        .spawn((
            action,
            Button,
            TabIndex(tab_index),
            authored_ui_image(
                render,
                MAIN_MENU_TEXTURE_PATHS[0],
                main_menu_texture(render, MAIN_MENU_TEXTURE_PATHS[0]),
            ),
            Node {
                width: percent(48.0),
                height: px(52),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(Color::srgb(0.827, 0.745, 0.498)),
                Pickable::IGNORE,
            ));
        });
}

pub(crate) fn spawn_town_dialogs(
    commands: &mut Commands,
    render: &RenderAssets,
    saves: &[TownSaveEntry],
) {
    for page in [MenuPage::NewTown, MenuPage::LoadTown] {
        commands
            .spawn((
                StateEntity,
                TownDialogRoot(page),
                Visibility::Hidden,
                GlobalZIndex(185),
                BackgroundColor(Color::srgba(0.004, 0.006, 0.012, 0.88)),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0),
                    top: px(0),
                    width: percent(100.0),
                    height: percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|overlay| {
                overlay
                    .spawn((
                        TabGroup::default(),
                        BackgroundColor(Color::srgb(0.035, 0.046, 0.09)),
                        BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                        Node {
                            width: percent(52.0),
                            max_width: px(840),
                            min_height: px(390),
                            max_height: percent(82.0),
                            border: UiRect::all(px(3)),
                            border_radius: BorderRadius::all(px(34)),
                            padding: UiRect::all(px(42)),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(20),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                    ))
                    .with_children(|panel| {
                        panel.spawn((
                            settings_panel_ui_image(
                                render,
                                SETTINGS_BACKGROUND_TEXTURE_PATH,
                                main_menu_texture(render, SETTINGS_BACKGROUND_TEXTURE_PATH),
                            ),
                            Pickable::IGNORE,
                            Node {
                                position_type: PositionType::Absolute,
                                left: px(3),
                                top: px(3),
                                right: px(3),
                                bottom: px(3),
                                ..default()
                            },
                        ));
                        panel.spawn((
                            UiDisplayFont,
                            Text::new(if page == MenuPage::NewTown {
                                "NAME YOUR TOWN"
                            } else {
                                "LOAD A TOWN"
                            }),
                            TextFont {
                                font_size: FontSize::Px(32.0),
                                ..default()
                            },
                            TextLayout::justify(Justify::Center),
                            TextColor(Color::WHITE),
                            Pickable::IGNORE,
                            Node {
                                width: percent(100.0),
                                ..default()
                            },
                        ));
                        if page == MenuPage::NewTown {
                            panel.spawn((
                                Text::new("The name determines the deterministic world seed and its independent save file."),
                                TextFont {
                                    font_size: FontSize::Px(17.0),
                                    ..default()
                                },
                                TextLayout::justify(Justify::Center),
                                TextColor(Color::srgb(0.84, 0.88, 0.94)),
                                Pickable::IGNORE,
                            ));
                            panel.spawn((
                                TownNameField,
                                EditableText {
                                    max_characters: Some(80),
                                    ..EditableText::new("")
                                },
                                SelectAllOnFocus,
                                TabIndex(0),
                                TextCursorStyle {
                                    color: Color::WHITE,
                                    selected_text_color: Some(Color::BLACK),
                                    ..default()
                                },
                                TextFont {
                                    font_size: FontSize::Px(24.0),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                BackgroundColor(Color::srgb(0.018, 0.026, 0.055)),
                                BorderColor::all(Color::srgb(0.40, 0.49, 0.67)),
                                Node {
                                    width: percent(100.0),
                                    height: px(56),
                                    border: UiRect::all(px(2)),
                                    border_radius: BorderRadius::all(px(8)),
                                    padding: UiRect::axes(px(14), px(10)),
                                    overflow: Overflow::clip_x(),
                                    ..default()
                                },
                            ));
                            panel
                                .spawn(Node {
                                    width: percent(100.0),
                                    height: px(52),
                                    justify_content: JustifyContent::SpaceBetween,
                                    ..default()
                                })
                                .with_children(|buttons| {
                                    spawn_town_dialog_button(
                                        buttons,
                                        TownDialogAction::Back,
                                        "Back",
                                        render,
                                        1,
                                    );
                                    spawn_town_dialog_button(
                                        buttons,
                                        TownDialogAction::Create,
                                        "Create Town",
                                        render,
                                        2,
                                    );
                                });
                        } else {
                            panel
                                .spawn((
                                    TownLoadList,
                                    ScrollPosition::default(),
                                    Node {
                                        width: percent(100.0),
                                        flex_grow: 1.0,
                                        flex_direction: FlexDirection::Column,
                                        row_gap: px(8),
                                        overflow: Overflow::scroll_y(),
                                        ..default()
                                    },
                                ))
                                .with_children(|list| {
                                    if saves.is_empty() {
                                        list.spawn((
                                            Text::new("No saved towns were found."),
                                            TextFont {
                                                font_size: FontSize::Px(19.0),
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    }
                                    for (index, save) in saves.iter().enumerate() {
                                        list.spawn((
                                            TownLoadChoice {
                                                name: save.name.clone(),
                                                path: save.path.clone(),
                                                protected: save.protected,
                                            },
                                            Button,
                                            TabIndex(i32::try_from(index).unwrap_or(i32::MAX)),
                                            authored_ui_image(
                                                render,
                                                MAIN_MENU_TEXTURE_PATHS[0],
                                                main_menu_texture(render, MAIN_MENU_TEXTURE_PATHS[0]),
                                            ),
                                            Node {
                                                width: percent(100.0),
                                                height: px(45),
                                                flex_shrink: 0.0,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                        ))
                                        .with_children(|button| {
                                            button.spawn((
                                                Text::new(save.name.clone()),
                                                TextFont {
                                                    font_size: FontSize::Px(18.0),
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.827, 0.745, 0.498)),
                                                Pickable::IGNORE,
                                            ));
                                        });
                                    }
                                });
                            panel
                                .spawn(Node {
                                    width: percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                })
                                .with_children(|buttons| {
                                    spawn_town_dialog_button(
                                        buttons,
                                        TownDialogAction::Back,
                                        "Back",
                                        render,
                                        i32::try_from(saves.len()).unwrap_or(i32::MAX),
                                    );
                                });
                        }
                        panel.spawn((
                            TownDialogFeedback,
                            Text::new(""),
                            TextFont {
                                font_size: FontSize::Px(15.0),
                                ..default()
                            },
                            TextLayout::justify(Justify::Center),
                            TextColor(Color::srgb(1.0, 0.70, 0.38)),
                            Pickable::IGNORE,
                            Node {
                                width: percent(100.0),
                                min_height: px(20),
                                ..default()
                            },
                        ));
                    });
            });
    }
}

pub(crate) fn spawn_go_live_confirmation(commands: &mut Commands, render: &RenderAssets) {
    commands
        .spawn((
            StateEntity,
            GoLiveConfirmationRoot,
            Name::new("Go-live town-start confirmation"),
            Visibility::Hidden,
            GlobalZIndex(190),
            BackgroundColor(Color::srgba(0.004, 0.006, 0.012, 0.86)),
            Node {
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                width: percent(100.0),
                height: percent(100.0),
                padding: UiRect::all(px(24)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Name::new("Go-live confirmation panel"),
                    TabGroup::default(),
                    Node {
                        width: percent(54.0),
                        max_width: px(860),
                        min_height: px(330),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(30)),
                        padding: UiRect::all(px(42)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(24),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.035, 0.046, 0.09)),
                    BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Name::new("Go-live confirmation nine-slice surface"),
                        settings_panel_ui_image(
                            render,
                            SETTINGS_BACKGROUND_TEXTURE_PATH,
                            main_menu_texture(render, SETTINGS_BACKGROUND_TEXTURE_PATH),
                        ),
                        Pickable::IGNORE,
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(3),
                            top: px(3),
                            right: px(3),
                            bottom: px(3),
                            ..default()
                        },
                    ));
                    panel.spawn((
                        Text::new("GO LIVE WITH THIS TOWN?"),
                        TextFont {
                            font_size: FontSize::Px(32.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::WHITE),
                        Pickable::IGNORE,
                    ));
                    panel.spawn((
                        GoLiveConfirmationBody,
                        Text::new("Starting this town will begin the internal Twitch stream once loading is complete. Do you agree to go live?"),
                        TextFont {
                            font_size: FontSize::Px(21.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            linebreak: LineBreak::WordBoundary,
                        },
                        TextColor(Color::srgb(0.94, 0.94, 0.91)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(92.0),
                            ..default()
                        },
                    ));
                    panel
                        .spawn(Node {
                            width: percent(74.0),
                            height: px(56),
                            column_gap: px(24),
                            ..default()
                        })
                        .with_children(|buttons| {
                            for (action, label, tab_index) in [
                                (GoLiveConfirmationAction::No, "No — stay offline", 0),
                                (GoLiveConfirmationAction::Yes, "Yes — go live", 1),
                            ] {
                                buttons
                                    .spawn((
                                        action,
                                        Button,
                                        TabIndex(tab_index),
                                        authored_ui_image(
                                            render,
                                            MAIN_MENU_TEXTURE_PATHS[0],
                                            main_menu_texture(render, MAIN_MENU_TEXTURE_PATHS[0]),
                                        ),
                                        Node {
                                            flex_grow: 1.0,
                                            height: percent(100.0),
                                            padding: UiRect::horizontal(px(14)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                    ))
                                    .with_child((
                                        Text::new(label),
                                        TextFont {
                                            font_size: FontSize::Px(17.0),
                                            ..default()
                                        },
                                        TextLayout::justify(Justify::Center),
                                        TextColor(Color::srgb(0.827, 0.745, 0.498)),
                                        Pickable::IGNORE,
                                    ));
                            }
                        });
                });
        });
}

pub(crate) fn spawn_secrets_overlays(
    commands: &mut Commands,
    render: &RenderAssets,
    config: &GameConfig,
) {
    commands
        .spawn((
            StateEntity,
            SecretsDisclaimerRoot,
            Name::new("Twitch secrets disclaimer"),
            Visibility::Hidden,
            GlobalZIndex(200),
            BackgroundColor(Color::srgba(0.004, 0.006, 0.012, 0.96)),
            Node {
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                width: percent(100.0),
                height: percent(100.0),
                padding: UiRect::all(px(24)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Name::new("Twitch secrets disclaimer panel"),
                    TabGroup::default(),
                    BackgroundColor(Color::srgb(0.035, 0.046, 0.09)),
                    BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                    Node {
                        width: percent(66.0),
                        max_width: px(1_050),
                        min_height: percent(52.0),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(34)),
                        padding: UiRect::all(px(48)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(22),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Name::new("Twitch secrets disclaimer nine-slice surface"),
                        settings_panel_ui_image(
                            render,
                            SETTINGS_BACKGROUND_TEXTURE_PATH,
                            main_menu_texture(render, SETTINGS_BACKGROUND_TEXTURE_PATH),
                        ),
                        Pickable::IGNORE,
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(3),
                            top: px(3),
                            right: px(3),
                            bottom: px(3),
                            ..default()
                        },
                    ));
                    panel.spawn((
                        Text::new("SENSITIVE TWITCH SETUP"),
                        TextFont {
                            font_size: FontSize::Px(34.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::WHITE),
                        Pickable::IGNORE,
                    ));
                    panel.spawn((
                        Text::new(
                            "Both Twitch accounts must be connected before starting a town. Complete bot and stream authorization in this menu.",
                        ),
                        TextFont {
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            linebreak: LineBreak::WordBoundary,
                        },
                        TextColor(Color::srgb(1.0, 0.78, 0.34)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(92.0),
                            ..default()
                        },
                    ));
                    panel.spawn((
                        Text::new(SECRETS_DISCLAIMER),
                        TextFont {
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            linebreak: LineBreak::WordBoundary,
                        },
                        TextColor(Color::srgb(0.94, 0.94, 0.91)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(92.0),
                            ..default()
                        },
                    ));
                    panel.spawn((
                        Text::new("Do you agree and want to continue?"),
                        TextFont {
                            font_size: FontSize::Px(22.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::srgb(0.94, 0.80, 0.43)),
                        Pickable::IGNORE,
                    ));
                    panel
                        .spawn(Node {
                            width: percent(72.0),
                            height: px(54),
                            column_gap: px(24),
                            ..default()
                        })
                        .with_children(|buttons| {
                            spawn_secrets_button(
                                buttons,
                                SecretsAction::DisclaimerNo,
                                config,
                                render,
                                0,
                            );
                            spawn_secrets_button(
                                buttons,
                                SecretsAction::DisclaimerYes,
                                config,
                                render,
                                1,
                            );
                        });
                });
        });

    commands
        .spawn((
            StateEntity,
            SecretsRoot,
            Name::new("Twitch secrets setup"),
            Visibility::Hidden,
            GlobalZIndex(210),
            BackgroundColor(Color::srgb(0.004, 0.006, 0.012)),
            Node {
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                width: percent(100.0),
                height: percent(100.0),
                padding: UiRect::all(px(22)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Name::new("Twitch secrets setup panel"),
                    TabGroup::default(),
                    BackgroundColor(Color::srgb(0.035, 0.046, 0.09)),
                    BorderColor::all(Color::srgb(0.827, 0.745, 0.498)),
                    Node {
                        width: percent(88.0),
                        height: percent(92.0),
                        max_width: px(1_520),
                        max_height: px(940),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(36)),
                        padding: UiRect::all(px(38)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(14),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Name::new("Twitch secrets setup nine-slice surface"),
                        settings_panel_ui_image(
                            render,
                            SETTINGS_BACKGROUND_TEXTURE_PATH,
                            main_menu_texture(render, SETTINGS_BACKGROUND_TEXTURE_PATH),
                        ),
                        Pickable::IGNORE,
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(3),
                            top: px(3),
                            right: px(3),
                            bottom: px(3),
                            ..default()
                        },
                    ));
                    panel.spawn((
                        Text::new("TWITCH SECRETS"),
                        TextFont {
                            font_size: FontSize::Px(32.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::WHITE),
                        Pickable::IGNORE,
                    ));
                    panel.spawn((
                        Text::new(SECRETS_PRIVACY_NOTICE),
                        TextFont {
                            font_size: FontSize::Px(17.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::srgb(1.0, 0.72, 0.28)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(100.0),
                            ..default()
                        },
                    ));
                    panel
                        .spawn(Node {
                            width: percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(10),
                            ..default()
                        })
                        .with_children(|fields| {
                            spawn_secrets_field(
                                fields,
                                SecretsField::ClientId,
                                "Twitch application Client ID",
                                &config.twitch.client_id,
                                0,
                            );
                            spawn_secrets_field(
                                fields,
                                SecretsField::BotLogin,
                                "Bot account login",
                                &config.twitch.bot_login,
                                1,
                            );
                            spawn_secrets_field(
                                fields,
                                SecretsField::ChannelLogin,
                                "Broadcaster/channel login",
                                &config.twitch.channel_login,
                                2,
                            );
                        });
                    panel
                        .spawn(Node {
                            width: percent(100.0),
                            flex_grow: 1.0,
                            column_gap: px(20),
                            ..default()
                        })
                        .with_children(|columns| {
                            spawn_secrets_account_column(
                                columns,
                                "CHAT BOT ACCOUNT",
                                "Authorizes chat:read and chat:edit for the bot login above. The token is stored only in the operating-system credential vault.",
                                SecretsConnectionKind::Bot,
                                SecretsAction::ToggleBot,
                                SecretsAction::AuthorizeBot,
                                config,
                                render,
                                3,
                            );
                            spawn_secrets_account_column(
                                columns,
                                "BROADCASTER / STREAM ACCOUNT",
                                "Separately authorizes stream-key access, operator chat output, and moderation for the broadcaster login above. The stream key is never saved; operator messages, timeouts, and bans all come from this streamer account, never the chat bot.",
                                SecretsConnectionKind::Broadcast,
                                SecretsAction::ToggleBroadcast,
                                SecretsAction::AuthorizeBroadcaster,
                                config,
                                render,
                                5,
                            );
                        });
                    panel.spawn((
                        Text::new("Bandwidth test sends the complete encoded stream to Twitch without making the channel live. Use it to verify bitrate and stability, then disable it before a real stream."),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            linebreak: LineBreak::WordBoundary,
                        },
                        TextColor(Color::srgb(0.76, 0.82, 0.92)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(100.0),
                            min_height: px(34),
                            ..default()
                        },
                    ));
                    panel
                        .spawn(Node {
                            width: percent(100.0),
                            min_height: px(46),
                            column_gap: px(14),
                            ..default()
                        })
                        .with_children(|buttons| {
                            spawn_secrets_button(
                                buttons,
                                SecretsAction::ToggleBandwidthTest,
                                config,
                                render,
                                7,
                            );
                            spawn_secrets_button(
                                buttons,
                                SecretsAction::Save,
                                config,
                                render,
                                8,
                            );
                            spawn_secrets_button(
                                buttons,
                                SecretsAction::Back,
                                config,
                                render,
                                9,
                            );
                        });
                    panel.spawn((
                        SecretsDeviceText,
                        Text::new("No Twitch authorization is in progress."),
                        TextFont {
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::srgb(0.66, 0.86, 1.0)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(100.0),
                            min_height: px(48),
                            ..default()
                        },
                    ));
                    panel.spawn((
                        SecretsStatusText,
                        Text::new(SECRETS_INITIAL_FEEDBACK),
                        TextFont {
                            font_size: FontSize::Px(16.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::srgb(0.86, 0.92, 0.88)),
                        Pickable::IGNORE,
                        Node {
                            width: percent(100.0),
                            min_height: px(34),
                            ..default()
                        },
                    ));
                });
        });
}

pub(crate) fn spawn_secrets_field(
    parent: &mut ChildSpawnerCommands,
    field: SecretsField,
    label: &'static str,
    value: &str,
    tab_index: i32,
) {
    parent
        .spawn(Node {
            width: percent(100.0),
            height: px(48),
            column_gap: px(18),
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(17.0),
                    ..default()
                },
                TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                Pickable::IGNORE,
                Node {
                    width: percent(31.0),
                    ..default()
                },
            ));
            row.spawn((
                field,
                EditableText {
                    max_characters: Some(128),
                    ..EditableText::new(value)
                },
                SelectAllOnFocus,
                TabIndex(tab_index),
                TextCursorStyle {
                    color: Color::WHITE,
                    selected_text_color: Some(Color::BLACK),
                    ..default()
                },
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                BackgroundColor(Color::srgb(0.018, 0.026, 0.055)),
                BorderColor::all(Color::srgb(0.40, 0.49, 0.67)),
                Node {
                    width: percent(69.0),
                    height: px(44),
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(px(8)),
                    padding: UiRect::axes(px(12), px(8)),
                    overflow: Overflow::clip_x(),
                    ..default()
                },
            ));
        });
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_secrets_account_column(
    parent: &mut ChildSpawnerCommands,
    heading: &'static str,
    description: &'static str,
    connection: SecretsConnectionKind,
    toggle: SecretsAction,
    authorize: SecretsAction,
    config: &GameConfig,
    render: &RenderAssets,
    tab_index: i32,
) {
    parent
        .spawn((
            BackgroundColor(Color::srgba(0.018, 0.026, 0.055, 0.88)),
            BorderColor::all(Color::srgb(0.32, 0.39, 0.56)),
            Node {
                width: percent(50.0),
                border: UiRect::all(px(2)),
                border_radius: BorderRadius::all(px(14)),
                padding: UiRect::all(px(18)),
                flex_direction: FlexDirection::Column,
                row_gap: px(12),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .with_children(|column| {
            column.spawn((
                Text::new(heading),
                TextFont {
                    font_size: FontSize::Px(21.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            column.spawn((
                Text::new(description),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextLayout {
                    justify: Justify::Center,
                    linebreak: LineBreak::WordBoundary,
                },
                TextColor(Color::srgb(0.82, 0.86, 0.91)),
                Pickable::IGNORE,
            ));
            column.spawn((
                SecretsConnectionText(connection),
                Text::new("● Checking status..."),
                TextFont {
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextLayout {
                    justify: Justify::Center,
                    linebreak: LineBreak::WordBoundary,
                },
                TextColor(Color::srgb(0.88, 0.78, 0.46)),
                Pickable::IGNORE,
                Node {
                    min_height: px(38),
                    ..default()
                },
            ));
            spawn_secrets_button(column, toggle, config, render, tab_index);
            spawn_secrets_button(column, authorize, config, render, tab_index + 1);
        });
}

pub(crate) fn spawn_secrets_button(
    parent: &mut ChildSpawnerCommands,
    action: SecretsAction,
    config: &GameConfig,
    render: &RenderAssets,
    tab_index: i32,
) {
    parent
        .spawn((
            action,
            Button,
            TabIndex(tab_index),
            authored_ui_image(
                render,
                MAIN_MENU_TEXTURE_PATHS[0],
                main_menu_texture(render, MAIN_MENU_TEXTURE_PATHS[0]),
            ),
            Node {
                min_width: px(150),
                flex_grow: 1.0,
                height: px(48),
                padding: UiRect::axes(px(16), px(9)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|button| {
            button.spawn((
                SecretsDynamicLabel(action),
                Text::new(secrets_action_label(action, config)),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                Pickable::IGNORE,
            ));
        });
}
