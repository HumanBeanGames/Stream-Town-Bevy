pub(crate) fn spawn_vote_track(
    parent: &mut ChildSpawnerCommands,
    render: &RenderAssets,
    fill_kind: VoteFillKind,
    unfilled_path: &str,
    filled_path: &str,
    node: Node,
) {
    let mut track = parent.spawn((
        authored_ui_image(render, unfilled_path, vote_texture(render, unfilled_path)),
        node,
    ));
    if fill_kind == VoteFillKind::TechnologyTimer {
        track.insert(TechnologyVoteTimerTrack);
    }
    track.with_children(|track| {
        track.spawn((
            fill_kind,
            authored_ui_image(render, filled_path, vote_texture(render, filled_path)),
            Node {
                width: percent(100.0),
                height: percent(100.0),
                overflow: Overflow::clip_x(),
                ..default()
            },
        ));
    });
}

pub(crate) fn spawn_technology_vote_option_row(
    panel: &mut ChildSpawnerCommands,
    render: &RenderAssets,
    index: u8,
) {
    let top =
        TECHNOLOGY_VOTE_FIRST_ROW_TOP + f32::from(index) * TECHNOLOGY_VOTE_SINGLE_LINE_ADVANCE;
    panel
        .spawn((
            TechnologyVoteOptionRow(index),
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                top: px(top),
                left: px(32),
                right: px(32),
                height: px(92),
                ..default()
            },
        ))
        .with_children(|row| {
            spawn_vote_track(
                row,
                render,
                VoteFillKind::TechnologyOption(index),
                VOTE_TEXTURE_PATHS[6],
                VOTE_TEXTURE_PATHS[5],
                Node {
                    position_type: PositionType::Absolute,
                    top: px(0),
                    left: px(0),
                    right: px(0),
                    height: px(28),
                    ..default()
                },
            );
            // A flex container centers the label against the bar's real
            // bounds. This remains correct when the display font changes.
            row.spawn((
                TechnologyVoteTitleBar,
                ZIndex(2),
                Pickable::IGNORE,
                Node {
                    position_type: PositionType::Absolute,
                    top: px(TECHNOLOGY_VOTE_LABEL_OFFSET_Y),
                    left: px(6),
                    right: px(6),
                    height: px(28),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::clip(),
                    ..default()
                },
            ))
            .with_children(|title_bar| {
                title_bar.spawn((
                    VoteTextKind::TechnologyOptionTitle(index),
                    UiDisplayFont,
                    Text::new(format!("{}. Technology", index + 1)),
                    TextFont {
                        font_size: FontSize::Px(13.5),
                        ..default()
                    },
                    TextLayout::new(Justify::Center, LineBreak::NoWrap),
                    TextColor(Color::srgb(0.98, 0.94, 0.78)),
                    TextShadow {
                        offset: Vec2::splat(1.0),
                        color: Color::linear_rgba(0.0, 0.0, 0.0, 0.95),
                    },
                    Pickable::IGNORE,
                    Node {
                        width: percent(100),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                ));
            });
            row.spawn((
                TechnologyVoteIcon(index),
                ImageNode::default(),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(2),
                    top: px(TECHNOLOGY_VOTE_ICON_TOP),
                    width: px(30),
                    height: px(30),
                    ..default()
                },
            ));
            row.spawn((
                TechnologyVoteDepthBadge(index),
                Visibility::Hidden,
                ZIndex(3),
                Pickable::IGNORE,
                BackgroundColor(Color::srgb(0.96, 0.76, 0.20)),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(4),
                    left: px(-78),
                    width: px(80),
                    height: px(20),
                    border_radius: BorderRadius::all(px(10)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::clip(),
                    ..default()
                },
            ))
            .with_children(|badge| {
                badge.spawn((
                    VoteTextKind::TechnologyOptionDepthTag(index),
                    UiDisplayFont,
                    Text::new("Fundamental"),
                    TextFont {
                        font_size: FontSize::Px(9.0),
                        ..default()
                    },
                    TextLayout::new(Justify::Center, LineBreak::NoWrap),
                    TextColor(Color::srgb(0.06, 0.08, 0.16)),
                    Pickable::IGNORE,
                ));
            });
            row.spawn((
                VoteTextKind::TechnologyOptionRequirements(index),
                Text::new("Requirements"),
                TextFont {
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextColor(Color::srgb(0.91, 0.89, 0.81)),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(36),
                    left: px(38),
                    right: px(3),
                    height: px(56),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
        });
}

pub(crate) fn technology_vote_row_advance(requirement_lines: usize) -> f32 {
    let extra_lines = u16::try_from(requirement_lines.saturating_sub(1)).unwrap_or(u16::MAX);
    TECHNOLOGY_VOTE_SINGLE_LINE_ADVANCE
        + f32::from(extra_lines) * TECHNOLOGY_VOTE_EXTRA_LINE_ADVANCE
}

pub(crate) fn spawn_vote_panels(commands: &mut Commands, render: &RenderAssets) {
    commands
        .spawn((
            WorldEntity,
            VotePanelKind::Technology,
            Name::new("Technology voting menu"),
            authored_ui_image(
                render,
                VOTE_TEXTURE_PATHS[0],
                vote_texture(render, VOTE_TEXTURE_PATHS[0]),
            ),
            GlobalZIndex(24),
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                top: px(96),
                right: px(18),
                width: px(260),
                height: px(TECHNOLOGY_VOTE_PANEL_HEIGHT),
                ..default()
            },
        ))
        .with_children(|panel| {
            panel.spawn((
                VoteTextKind::TechnologyTitle,
                UiDisplayFont,
                Text::new("TECHNOLOGY VOTE"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.97, 0.88, 0.58)),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(TECHNOLOGY_VOTE_TITLE_TOP),
                    left: px(32),
                    right: px(32),
                    ..default()
                },
            ));
            for index in 0..u8::try_from(TECHNOLOGY_VOTE_OPTION_COUNT)
                .expect("technology vote option count fits u8")
            {
                spawn_technology_vote_option_row(panel, render, index);
            }
            panel
                .spawn((
                    TechnologyVoteTimerGroup,
                    Pickable::IGNORE,
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(0),
                        top: px(TECHNOLOGY_VOTE_TIMER_GROUP_TOP),
                        right: px(0),
                        height: px(24),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                ))
                .with_children(|timer_group| {
                    timer_group.spawn((
                        TechnologyVoteTimerIcon,
                        ImageNode::new(vote_texture(render, VOTE_TEXTURE_PATHS[3])),
                        Pickable::IGNORE,
                        Node {
                            width: px(24),
                            height: px(24),
                            ..default()
                        },
                    ));
                    timer_group.spawn((
                        TechnologyVoteTimerGap,
                        Pickable::IGNORE,
                        Node {
                            width: px(TECHNOLOGY_VOTE_TIMER_GAP),
                            height: px(1),
                            ..default()
                        },
                    ));
                    timer_group
                        .spawn((
                            TechnologyVoteTimerBar,
                            Pickable::IGNORE,
                            Node {
                                width: px(TECHNOLOGY_VOTE_TIMER_BAR_WIDTH),
                                height: px(20),
                                ..default()
                            },
                        ))
                        .with_children(|timer_bar| {
                            spawn_vote_track(
                                timer_bar,
                                render,
                                VoteFillKind::TechnologyTimer,
                                VOTE_TEXTURE_PATHS[2],
                                VOTE_TEXTURE_PATHS[1],
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: px(0),
                                    top: px(0),
                                    right: px(0),
                                    height: px(20),
                                    ..default()
                                },
                            );
                            timer_bar
                                .spawn((
                                    TechnologyVoteTimerLabel,
                                    ZIndex(2),
                                    Pickable::IGNORE,
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: px(0),
                                        top: px(0),
                                        right: px(0),
                                        height: px(20),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        overflow: Overflow::clip(),
                                        ..default()
                                    },
                                ))
                                .with_children(|timer_label| {
                                    timer_label.spawn((
                                        VoteTextKind::TechnologyTimer,
                                        Text::new("00:30"),
                                        TextFont {
                                            font_size: FontSize::Px(16.0),
                                            ..default()
                                        },
                                        TextLayout::new(Justify::Center, LineBreak::NoWrap),
                                        TextColor(Color::srgb(0.06, 0.08, 0.16)),
                                        UiTransform::from_xy(
                                            px(0),
                                            px(TECHNOLOGY_VOTE_TIMER_TEXT_OFFSET_Y),
                                        ),
                                        Pickable::IGNORE,
                                        Node {
                                            overflow: Overflow::clip(),
                                            ..default()
                                        },
                                    ));
                                });
                        });
                });
        });

    commands
        .spawn((
            WorldEntity,
            VotePanelKind::Ruler,
            Name::new("Ruler voting menu"),
            GlobalZIndex(24),
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                right: percent(1.2),
                bottom: percent(0.0),
                width: percent(14.6),
                height: percent(31.4),
                ..default()
            },
        ))
        .with_children(|panel| {
            // Unity extends the sliced panel 156 authored pixels beneath its
            // anchored content rect. Keeping that offset separate prevents the
            // title and options from being scaled along with the decorative frame.
            panel.spawn((
                authored_ui_image(
                    render,
                    VOTE_TEXTURE_PATHS[0],
                    vote_texture(render, VOTE_TEXTURE_PATHS[0]),
                )
                .with_color(Color::srgba(1.0, 1.0, 1.0, 0.95)),
                Pickable::IGNORE,
                Node {
                    position_type: PositionType::Absolute,
                    top: px(0),
                    left: px(0),
                    right: px(0),
                    bottom: px(-156.0 * render.main_ui_scale),
                    ..default()
                },
            ));
            panel.spawn((
                ImageNode::new(vote_texture(render, VOTE_TEXTURE_PATHS[7])),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(21.6),
                    top: percent(10.5),
                    width: percent(9.6),
                    aspect_ratio: Some(1.0),
                    ..default()
                },
            ));
            panel.spawn((
                VoteTextKind::RulerTitle,
                UiDisplayFont,
                Text::new("VOTE FOR RULER"),
                TextFont {
                    // Bevy's converted display font is wider than Unity's
                    // TextMeshPro atlas. Fit the complete authored label rather
                    // than clipping "RULER" at the right edge.
                    font_size: FontSize::Px(17.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.827_451, 0.745_098_05, 0.498_039_22)),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(31.2),
                    right: percent(18.8),
                    top: percent(11.4),
                    height: percent(7.5),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
            panel.spawn((
                VoteTextKind::RulerDescription,
                Text::new("Who should be Ruler?"),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.91, 0.89, 0.81)),
                TextShadow {
                    offset: Vec2::splat(1.0),
                    color: Color::linear_rgba(0.0, 0.0, 0.0, 0.9),
                },
                ZIndex(2),
                Node {
                    position_type: PositionType::Absolute,
                    top: percent(18.9),
                    left: percent(10.9),
                    right: percent(11.1),
                    height: percent(10.5),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
            panel.spawn((
                RulerOptionsContainer,
                Name::new("Ruler vote options"),
                Node {
                    position_type: PositionType::Absolute,
                    top: percent(30.6),
                    bottom: percent(22.8),
                    left: percent(17.8),
                    right: percent(17.8),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(3),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
            spawn_vote_track(
                panel,
                render,
                VoteFillKind::RulerTimer,
                RULER_VOTE_TIMER_UNFILLED_PATH,
                VOTE_TEXTURE_PATHS[8],
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(17.9),
                    right: percent(17.9),
                    bottom: percent(10.3),
                    height: percent(7.4),
                    ..default()
                },
            );
            panel.spawn((
                VoteTextKind::RulerTimer,
                Text::new("02:00"),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.91, 0.89, 0.81)),
                TextShadow {
                    offset: Vec2::splat(1.0),
                    color: Color::linear_rgba(0.0, 0.0, 0.0, 0.9),
                },
                ZIndex(2),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(17.9),
                    right: percent(17.9),
                    bottom: percent(10.3),
                    ..default()
                },
            ));
        });
}

pub(crate) fn spawn_current_event_panel(commands: &mut Commands, render: &RenderAssets) {
    commands
        .spawn((
            WorldEntity,
            CurrentEventPanel,
            Name::new("Current town event"),
            authored_ui_image(
                render,
                CURRENT_EVENT_TEXTURE_PATHS[0],
                current_event_texture(render, CURRENT_EVENT_TEXTURE_PATHS[0]),
            ),
            GlobalZIndex(23),
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                top: px(76),
                left: px(18),
                width: px(290),
                height: px(104),
                padding: UiRect::all(px(10)),
                ..default()
            },
        ))
        .with_children(|panel| {
            panel.spawn((
                CurrentEventText::Title,
                UiDisplayFont,
                Text::new("Current Event"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::srgb(0.97, 0.88, 0.58)),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(8),
                    left: px(10),
                    right: px(10),
                    ..default()
                },
            ));
            panel.spawn((
                CurrentEventText::Description,
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(31),
                    left: px(10),
                    right: px(10),
                    ..default()
                },
            ));
            panel
                .spawn((
                    authored_ui_image(
                        render,
                        CURRENT_EVENT_TEXTURE_PATHS[1],
                        current_event_texture(render, CURRENT_EVENT_TEXTURE_PATHS[1]),
                    ),
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(12),
                        right: px(12),
                        bottom: px(10),
                        height: px(20),
                        overflow: Overflow::clip_x(),
                        ..default()
                    },
                ))
                .with_children(|track| {
                    track.spawn((
                        CurrentEventFill,
                        authored_ui_image(
                            render,
                            CURRENT_EVENT_TEXTURE_PATHS[2],
                            current_event_texture(render, CURRENT_EVENT_TEXTURE_PATHS[2]),
                        ),
                        Node {
                            width: percent(0.0),
                            height: percent(100.0),
                            ..default()
                        },
                    ));
                    track.spawn((
                        CurrentEventText::Progress,
                        Text::new(""),
                        TextFont {
                            font_size: FontSize::Px(12.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::srgb(0.91, 0.89, 0.81)),
                        Pickable::IGNORE,
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(4),
                            right: px(4),
                            top: px(1),
                            ..default()
                        },
                    ));
                });
        });
}

pub(crate) fn spawn_hud_metric(
    parent: &mut ChildSpawnerCommands,
    render: &RenderAssets,
    metric: HudMetric,
    source_path: &str,
    width: Val,
) {
    spawn_hud_metric_sized(parent, render, metric, source_path, width, 36.0, 20.0);
}

pub(crate) fn spawn_hud_metric_sized(
    parent: &mut ChildSpawnerCommands,
    render: &RenderAssets,
    metric: HudMetric,
    source_path: &str,
    width: Val,
    icon_size: f32,
    font_size: f32,
) {
    parent
        .spawn((
            HudMetricRow,
            Node {
                width,
                height: px(44),
                flex_shrink: 0.0,
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::clip(),
                ..default()
            },
        ))
        .with_children(|metric_parent| {
            if let Some(icon) = top_bar_texture(render, source_path) {
                metric_parent.spawn((
                    ImageNode::new(icon),
                    Node {
                        width: px(icon_size),
                        height: px(icon_size),
                        flex_shrink: 0.0,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                ));
            }
            metric_parent
                .spawn(Node {
                    flex_grow: 1.0,
                    height: px(40),
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::left(px(5)),
                    overflow: Overflow::clip(),
                    ..default()
                })
                .with_children(|values| {
                    values.spawn((
                        metric,
                        Text::new("0"),
                        TextFont {
                            font_size: FontSize::Px(font_size),
                            ..default()
                        },
                        TextColor(Color::srgb(0.91, 0.89, 0.81)),
                        TextLayout::no_wrap(),
                        Node {
                            height: px(23),
                            flex_shrink: 0.0,
                            overflow: Overflow::clip(),
                            ..default()
                        },
                    ));
                    if metric.shows_maximum() {
                        values.spawn((
                            HudMetricMaximum(metric),
                            Text::new("/0"),
                            TextFont {
                                font_size: FontSize::Px((font_size * 0.55).max(9.0)),
                                ..default()
                            },
                            TextColor(Color::srgb(0.72, 0.72, 0.68)),
                            TextLayout::no_wrap(),
                            Node {
                                height: px(12),
                                flex_shrink: 0.0,
                                overflow: Overflow::clip(),
                                ..default()
                            },
                        ));
                    }
                });
        });
}

pub(crate) fn spawn_hud(
    commands: &mut Commands,
    render: &RenderAssets,
    agents: u16,
    world_hash: &str,
) {
    let complete_art = TOP_BAR_TEXTURE_PATHS
        .iter()
        .all(|path| render.top_bar_textures.contains_key(*path));
    let mut root = commands.spawn((
        WorldEntity,
        HudTopBar,
        Name::new("Shipping top bar"),
        authored_ui_image(
            render,
            VOTE_TEXTURE_PATHS[0],
            vote_texture(render, VOTE_TEXTURE_PATHS[0]),
        ),
        GlobalZIndex(20),
        Node {
            position_type: PositionType::Absolute,
            top: px(0),
            left: px(0),
            width: percent(100.0),
            height: percent(6.2),
            min_height: px(62),
            max_height: px(76),
            ..default()
        },
        BackgroundColor(Color::srgba(0.025, 0.05, 0.035, 0.88)),
    ));
    root.with_children(|parent| {
        parent
            .spawn((
                HudResourceStrip,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(18),
                    top: percent(0.0),
                    width: percent(39.7),
                    height: percent(100.0),
                    align_items: AlignItems::Center,
                    overflow: Overflow::clip(),
                    ..default()
                },
            ))
            .with_children(|resources| {
                for (metric, path) in [
                    (HudMetric::Food, TOP_BAR_TEXTURE_PATHS[1]),
                    (HudMetric::Gold, TOP_BAR_TEXTURE_PATHS[2]),
                    (HudMetric::Ore, TOP_BAR_TEXTURE_PATHS[3]),
                    (HudMetric::Wood, TOP_BAR_TEXTURE_PATHS[4]),
                ] {
                    spawn_hud_metric(resources, render, metric, path, percent(25.0));
                }
            });
        parent
            .spawn((
                HudStatsStrip,
                Node {
                    position_type: PositionType::Absolute,
                    right: px(10),
                    top: percent(0.0),
                    width: percent(40.5),
                    height: percent(100.0),
                    align_items: AlignItems::Center,
                    overflow: Overflow::clip(),
                    ..default()
                },
            ))
            .with_children(|stats| {
                spawn_hud_metric(
                    stats,
                    render,
                    HudMetric::Players,
                    TOP_BAR_TEXTURE_PATHS[5],
                    percent(10.0),
                );
                spawn_hud_metric_sized(
                    stats,
                    render,
                    HudMetric::Npcs,
                    TOP_BAR_TEXTURE_PATHS[5],
                    percent(10.0),
                    27.0,
                    18.0,
                );
                spawn_hud_metric(
                    stats,
                    render,
                    HudMetric::Buildings,
                    TOP_BAR_TEXTURE_PATHS[6],
                    percent(13.0),
                );
                spawn_hud_metric(
                    stats,
                    render,
                    HudMetric::PlayTime,
                    TOP_BAR_TEXTURE_PATHS[7],
                    percent(22.0),
                );
                stats
                    .spawn((
                        HudTechnologyObjectivePanel,
                        Node {
                            width: percent(45.0),
                            height: percent(100.0),
                            align_self: AlignSelf::Center,
                            overflow: Overflow::clip(),
                            ..default()
                        },
                    ))
                    .with_children(|objective| {
                        objective.spawn((
                            HudTechnologyTextKind::Title,
                            UiDisplayFont,
                            Text::new("TECHNOLOGY"),
                            TextFont {
                                font_size: FontSize::Px(10.0),
                                ..default()
                            },
                            TextLayout::no_wrap(),
                            TextColor(Color::srgb(0.97, 0.88, 0.58)),
                            TextShadow {
                                offset: Vec2::splat(1.0),
                                color: Color::linear_rgba(0.0, 0.0, 0.0, 0.9),
                            },
                            Pickable::IGNORE,
                            Node {
                                position_type: PositionType::Absolute,
                                top: px(13),
                                right: px(20),
                                left: px(10),
                                height: px(13),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                        ));
                        objective.spawn((
                            HudTechnologyTextKind::Requirement,
                            Text::new("Ballot pending"),
                            TextFont {
                                font_size: FontSize::Px(8.5),
                                ..default()
                            },
                            TextLayout::no_wrap(),
                            TextColor(Color::srgb(0.91, 0.89, 0.81)),
                            TextShadow {
                                offset: Vec2::splat(1.0),
                                color: Color::linear_rgba(0.0, 0.0, 0.0, 0.85),
                            },
                            Pickable::IGNORE,
                            Node {
                                position_type: PositionType::Absolute,
                                top: px(25),
                                right: px(20),
                                left: px(10),
                                height: px(11),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                        ));
                        objective
                            .spawn((
                                authored_ui_image(
                                    render,
                                    OBJECTIVE_TEXTURE_PATHS[1],
                                    objective_texture(render, OBJECTIVE_TEXTURE_PATHS[1]),
                                ),
                                Pickable::IGNORE,
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: px(10),
                                    right: px(20),
                                    bottom: px(17),
                                    height: px(9),
                                    overflow: Overflow::clip_x(),
                                    ..default()
                                },
                            ))
                            .with_children(|track| {
                                track.spawn((
                                    HudTechnologyProgressFill,
                                    authored_ui_image(
                                        render,
                                        OBJECTIVE_TEXTURE_PATHS[2],
                                        objective_texture(render, OBJECTIVE_TEXTURE_PATHS[2]),
                                    ),
                                    Pickable::IGNORE,
                                    Node {
                                        width: percent(0.0),
                                        height: percent(100.0),
                                        ..default()
                                    },
                                ));
                            });
                    });
            });
        if let (Some(gauge), Some(meter)) = (
            top_bar_texture(render, TOP_BAR_TEXTURE_PATHS[8]),
            top_bar_texture(render, TOP_BAR_TEXTURE_PATHS[9]),
        ) {
            parent
                .spawn((
                    ImageNode::new(gauge),
                    Node {
                        position_type: PositionType::Absolute,
                        left: percent(42.15),
                        top: percent(12.6),
                        width: px(300),
                        height: percent(58.9),
                        ..default()
                    },
                ))
                .with_children(|gauge_parent| {
                    gauge_parent.spawn((
                        SeasonMeter,
                        ImageNode::new(meter),
                        Node {
                            position_type: PositionType::Absolute,
                            top: px(-3),
                            left: percent(0.0),
                            width: px(10),
                            height: px(38),
                            ..default()
                        },
                    ));
                });
        }
    });

    commands.spawn((
        WorldEntity,
        Hud,
        Name::new("Runtime diagnostics"),
        Text::new(if complete_art {
            format!("{agents} agents | world {}", &world_hash[..12])
        } else {
            format!(
                "{agents} agents | world {} | HUD art fallback",
                &world_hash[..12]
            )
        }),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.96, 0.9)),
        GlobalZIndex(20),
        if std::env::var_os("STREAM_TOWN_RUNTIME_DIAGNOSTICS").is_some() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: px(8),
            left: px(12),
            width: px(750),
            ..default()
        },
    ));

    commands.spawn((
        WorldEntity,
        HudCommandGuidance,
        Name::new("Twitch command guidance"),
        UiDisplayFont,
        Text::new("type !join to apply for citizenship\ntype !help for more commands"),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextLayout::new(Justify::Center, LineBreak::WordBoundary),
        TextColor(Color::srgb(0.97, 0.91, 0.7)),
        TextShadow {
            offset: Vec2::new(1.5, 1.5),
            color: Color::linear_rgba(0.0, 0.0, 0.0, 0.9),
        },
        Pickable::IGNORE,
        GlobalZIndex(20),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: percent(25.0),
            width: percent(50.0),
            ..default()
        },
    ));

    let slider_background = render
        .selection_panel_textures
        .get(SELECTION_PANEL_TEXTURE_PATHS[0])
        .cloned();
    let slider_fill = render
        .selection_panel_textures
        .get(SELECTION_PANEL_TEXTURE_PATHS[1])
        .cloned();
    let mut selection_panel = commands.spawn((
        WorldEntity,
        SelectionPanel,
        Name::new("Selection window"),
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(19.0),
            ..default()
        },
        TextColor(Color::WHITE),
        GlobalZIndex(20),
        Visibility::Hidden,
        Node {
            position_type: PositionType::Absolute,
            right: px(20),
            bottom: px(82),
            width: px(460),
            min_height: px(160),
            padding: UiRect {
                left: px(18),
                right: px(18),
                top: px(18),
                bottom: px(60),
            },
            border: UiRect::all(px(2)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.035, 0.075, 0.05, 0.94)),
        BorderColor::all(Color::srgb(0.78, 0.68, 0.24)),
    ));
    selection_panel.with_children(|parent| {
        for (bar, bottom) in [
            (SelectionPanelBar::Health, 14.0),
            (SelectionPanelBar::Experience, 36.0),
        ] {
            let mut track = parent.spawn((
                SelectionPanelSliderTrack(bar),
                Visibility::Hidden,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(18),
                    right: px(18),
                    bottom: px(bottom),
                    height: px(16),
                    overflow: Overflow::clip_x(),
                    ..default()
                },
            ));
            if let Some(background) = slider_background.clone() {
                track.insert(ImageNode::new(background).with_mode(NodeImageMode::Stretch));
            }
            track.with_children(|track_parent| {
                let slider_image = slider_fill
                    .clone()
                    .map_or_else(ImageNode::default, |image| {
                        ImageNode::new(image).with_mode(NodeImageMode::Stretch)
                    });
                track_parent.spawn((
                    SelectionPanelSlider(bar),
                    slider_image,
                    Node {
                        width: percent(100.0),
                        height: percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.8, 0.24)),
                ));
            });
        }
    });

    spawn_vote_panels(commands, render);
    spawn_current_event_panel(commands, render);
}
