pub(crate) const fn menu_page_is_sensitive(page: MenuPage) -> bool {
    matches!(page, MenuPage::SecretsDisclaimer | MenuPage::Secrets)
}

pub(crate) fn update_menu_overlay(
    state: Res<State<GameState>>,
    menu: Res<MenuRuntime>,
    save: Res<SaveRuntime>,
    mut overlay: Query<(&mut Text, &mut Visibility), With<MenuOverlay>>,
    mut game_menu: Query<&mut Visibility, (With<GameMenuRoot>, Without<MenuOverlay>)>,
) {
    let Ok((mut text, mut visibility)) = overlay.single_mut() else {
        return;
    };
    let image_game_menu_visible = menu.page == MenuPage::Game && *state.get() == GameState::InGame;
    if let Ok(mut game_menu_visibility) = game_menu.single_mut() {
        *game_menu_visibility = if image_game_menu_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if matches!(
        menu.page,
        MenuPage::Closed
            | MenuPage::GoLiveConfirmation
            | MenuPage::NewTown
            | MenuPage::LoadTown
            | MenuPage::Settings
            | MenuPage::SecretsDisclaimer
            | MenuPage::Secrets
    ) || image_game_menu_visible
    {
        *visibility = Visibility::Hidden;
        return;
    }
    **text = match menu.page {
        MenuPage::Game => game_menu_text(*state.get(), menu.selected, save.store.path().is_file()),
        MenuPage::Settings => settings_menu_text(
            &menu.draft,
            &menu.streaming_draft,
            menu.selected,
            &menu.feedback,
        ),
        MenuPage::Closed
        | MenuPage::GoLiveConfirmation
        | MenuPage::NewTown
        | MenuPage::LoadTown
        | MenuPage::SecretsDisclaimer
        | MenuPage::Secrets => String::new(),
    };
    *visibility = Visibility::Visible;
}

pub(crate) fn sync_cursor_visibility(
    state: Res<State<GameState>>,
    menu: Res<MenuRuntime>,
    mut cursors: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    let visible = cursor_visible_for_state(*state.get(), menu.page);
    for mut cursor in &mut cursors {
        cursor.visible = visible;
    }
}

pub(crate) const fn cursor_visible_for_state(state: GameState, menu_page: MenuPage) -> bool {
    match state {
        GameState::MainMenu | GameState::Credits => true,
        GameState::InGame => !matches!(menu_page, MenuPage::Closed),
        GameState::Boot | GameState::WorldLoading => false,
    }
}

pub(crate) fn game_menu_text(state: GameState, selected: usize, has_save: bool) -> String {
    use std::fmt::Write as _;

    let items: &[(&str, bool)] = if state == GameState::InGame {
        &[
            ("Save Game", true),
            ("Save Jump-Start", true),
            ("Load Game", has_save),
            ("Settings", true),
            ("Go Live", true),
            ("Exit Game", true),
        ]
    } else {
        &[
            ("New town", true),
            ("Load game", has_save),
            ("Settings", true),
            ("Credits", true),
            ("Quit", true),
        ]
    };
    let mut text = String::from("STREAM TOWN MENU\n\n");
    for (index, (label, enabled)) in items.iter().enumerate() {
        let marker = if index == selected { ">" } else { " " };
        let suffix = if *enabled { "" } else { "  [No save]" };
        writeln!(text, "{marker} {label}{suffix}").expect("writing to String cannot fail");
    }
    text.push_str("\nUP/DOWN Select   ENTER Confirm   ESC Resume/Close");
    text
}

pub(crate) fn settings_menu_text(
    settings: &PlayerSettings,
    streaming: &BroadcastConfig,
    selected: usize,
    feedback: &str,
) -> String {
    use std::fmt::Write as _;
    const COLUMN_BREAK: usize = 18;

    let video = &settings.video;
    let camera = &settings.camera;
    let interface = &settings.interface;
    let items = [
        format!("Display mode: {:?}", video.display_mode),
        format!("Resolution: {} x {}", video.width, video.height),
        format!("VSync: {}", on_off(video.vsync)),
        format!(
            "FPS limit: {}",
            video
                .fps_limit
                .map_or("Unlimited".to_owned(), |value| value.to_string())
        ),
        format!("Shadows: {}", on_off(video.shadows_enabled)),
        format!("Shadow map: {}", video.shadow_map_resolution),
        format!("Ambient occlusion: {}", on_off(video.ambient_occlusion)),
        format!("MSAA samples: {}", video.msaa_samples),
        format!("Post-process AA: {:?}", video.post_process_aa),
        format!("Brightness: {:.1}", video.brightness_ev),
        format!("Gamma: {:.1}", video.gamma),
        format!("Master volume: {}%", volume_percent(settings.audio.master)),
        format!("Music volume: {}%", volume_percent(settings.audio.music)),
        format!(
            "Sound effects: {}%",
            volume_percent(settings.audio.sound_effects)
        ),
        format!(
            "Ambience volume: {}%",
            volume_percent(settings.audio.ambience)
        ),
        format!("Zoom sensitivity: {:.0}", camera.zoom_sensitivity),
        format!("Camera field of view: {}", camera.field_of_view_degrees),
        format!("Name display: {:?}", interface.display_names),
        format!("Building health: {:?}", interface.display_building_health),
        format!(
            "Autosave: {}",
            if settings.autosave_minutes == 0 {
                "Off".to_owned()
            } else {
                format!("{} min", settings.autosave_minutes)
            }
        ),
        format!(
            "Timelapse frequency: {}",
            timelapse_interval_label(settings.timelapse.interval)
        ),
        format!("Timelapse dynamic: {}", on_off(settings.timelapse.dynamic)),
        format!("UI scale: {}%", interface.ui_scale_percent),
        format!("High contrast: {}", on_off(interface.high_contrast)),
        format!("Reduced motion: {}", on_off(interface.reduced_motion)),
        format!("Direct streaming: {}", on_off(streaming.enabled)),
        format!("Stream output: {} x {}", streaming.width, streaming.height),
        format!("Stream frame rate: {} FPS", streaming.frames_per_second),
        format!("Video bitrate: {} kbps", streaming.video_bitrate_kbps),
        format!("Audio bitrate: {} kbps", streaming.audio_bitrate_kbps),
        format!("Stream encoder: {:?}", streaming.encoder),
        format!("Bandwidth test: {}", on_off(streaming.bandwidth_test)),
        format!("Render mode: {:?}", streaming.render_mode),
        "Apply and save".to_owned(),
        "Restore defaults".to_owned(),
        "Cancel changes".to_owned(),
    ];
    let mut text = String::from("SETTINGS\n\n");
    for row in 0..COLUMN_BREAK {
        let left = settings_menu_item(&items, row, selected);
        let right = settings_menu_item(&items, row + COLUMN_BREAK, selected);
        writeln!(text, "{left:<44}{right}").expect("writing to String cannot fail");
    }
    text.push_str("\nUP/DOWN Select   LEFT/RIGHT Change   ENTER Confirm   ESC Cancel");
    if !feedback.is_empty() {
        text.push_str("\n\n");
        text.push_str(feedback);
    }
    text
}

pub(crate) fn settings_menu_item(items: &[String], index: usize, selected: usize) -> String {
    items.get(index).map_or_else(String::new, |item| {
        format!("{} {item}", if index == selected { ">" } else { " " })
    })
}

pub(crate) fn on_off(value: bool) -> &'static str {
    if value { "On" } else { "Off" }
}

pub(crate) const fn timelapse_interval_label(interval: TimelapseInterval) -> &'static str {
    match interval {
        TimelapseInterval::Off => "Off",
        TimelapseInterval::TenMinutes => "10 Minutes",
        TimelapseInterval::OneHour => "1 Hour",
        TimelapseInterval::ThreeHours => "3 Hours",
        TimelapseInterval::SixHours => "6 Hours",
        TimelapseInterval::TwelveHours => "12 Hours",
        TimelapseInterval::TwentyFourHours => "24 Hours",
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub(crate) fn volume_percent(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 100.0).round() as u8
}

pub(crate) fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    save: Res<SaveRuntime>,
    town_catalog: Res<TownSaveCatalogRuntime>,
    mut config: ResMut<RuntimeConfig>,
    secrets: Res<SecretsRuntime>,
    connection: Res<TwitchConnection>,
    mut menu: ResMut<MenuRuntime>,
    mut io: ResMut<MenuIoRequest>,
    mut player_settings: ResMut<RuntimePlayerSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
    #[cfg(target_os = "windows")] broadcast_runtime: Res<direct_broadcast::DirectBroadcastRuntime>,
    #[cfg(target_os = "windows")] mut broadcast_control: ResMut<
        direct_broadcast::DirectBroadcastControl,
    >,
) {
    #[cfg(target_os = "windows")]
    let streaming_locked = broadcast_runtime.snapshot().phase.is_active();
    #[cfg(not(target_os = "windows"))]
    let streaming_locked = false;
    let shift_escape = keyboard.just_pressed(KeyCode::Escape)
        && (keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight));
    let open_key = if *state.get() == GameState::InGame {
        false
    } else {
        shift_escape || keyboard.just_pressed(KeyCode::KeyS)
    };
    if menu.page == MenuPage::Closed {
        if open_key {
            menu.page = MenuPage::Game;
            menu.return_page = MenuPage::Closed;
            menu.selected = 0;
            menu.feedback.clear();
        }
        return;
    }
    if matches!(
        menu.page,
        MenuPage::GoLiveConfirmation
            | MenuPage::NewTown
            | MenuPage::LoadTown
            | MenuPage::SecretsDisclaimer
            | MenuPage::Secrets
    ) {
        return;
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        if menu.page == MenuPage::Settings {
            if menu.confirm_settings_close {
                menu.confirm_settings_close = false;
                menu.selected = SETTINGS_BACK_INDEX;
            } else {
                request_settings_close(&mut menu, &player_settings.0, &config.0.twitch.broadcast);
                if menu.confirm_settings_close {
                    menu.selected = 0;
                }
            }
        } else {
            let target = menu.return_page;
            menu.page = target;
            if target == MenuPage::Game {
                menu.return_page = MenuPage::Closed;
            }
            menu.selected = 0;
            menu.feedback.clear();
        }
        return;
    }
    if menu.page == MenuPage::Settings && menu.confirm_settings_close {
        if keyboard.just_pressed(KeyCode::ArrowLeft)
            || keyboard.just_pressed(KeyCode::ArrowRight)
            || keyboard.just_pressed(KeyCode::ArrowUp)
            || keyboard.just_pressed(KeyCode::ArrowDown)
        {
            menu.selected = usize::from(menu.selected == 0);
        }
        if keyboard.just_pressed(KeyCode::Enter) {
            if menu.selected == 0 {
                if apply_settings_draft(
                    &mut menu,
                    &mut player_settings,
                    &mut config,
                    streaming_locked,
                ) {
                    close_settings_menu(&mut menu);
                }
            } else {
                menu.draft = player_settings.0.clone();
                menu.streaming_draft = config.0.twitch.broadcast.clone();
                close_settings_menu(&mut menu);
            }
        }
        return;
    }
    let item_count = if menu.page == MenuPage::Settings {
        SETTINGS_MENU_ITEM_COUNT
    } else if *state.get() == GameState::InGame {
        6
    } else {
        5
    };
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        menu.selected = menu.selected.checked_sub(1).unwrap_or(item_count - 1);
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        menu.selected = (menu.selected + 1) % item_count;
    }
    if menu.page == MenuPage::Settings {
        let previous_tab =
            keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
        if keyboard.just_pressed(KeyCode::Tab) {
            menu.settings_tab = cycle_settings_tab(menu.settings_tab, !previous_tab);
            menu.selected = settings_tab_indices(menu.settings_tab)
                .first()
                .copied()
                .unwrap_or(SETTINGS_APPLY_INDEX);
            menu.feedback.clear();
            return;
        }
        if let Some(tab) = settings_tab_for_index(menu.selected) {
            menu.settings_tab = tab;
        }
        let adjustment = i8::from(keyboard.just_pressed(KeyCode::ArrowRight))
            - i8::from(keyboard.just_pressed(KeyCode::ArrowLeft));
        if adjustment != 0 && menu.selected < SETTINGS_APPLY_INDEX {
            let selected = menu.selected;
            if settings_value_enabled(selected, streaming_locked) {
                let menu = &mut *menu;
                adjust_settings_menu(
                    &mut menu.draft,
                    &mut menu.streaming_draft,
                    selected,
                    adjustment,
                );
                menu.feedback.clear();
            } else {
                "End the stream before changing streaming settings.".clone_into(&mut menu.feedback);
            }
        }
        if !keyboard.just_pressed(KeyCode::Enter) {
            return;
        }
        match menu.selected {
            SETTINGS_APPLY_INDEX => {
                apply_settings_draft(
                    &mut menu,
                    &mut player_settings,
                    &mut config,
                    streaming_locked,
                );
            }
            SETTINGS_DEFAULTS_INDEX => {
                menu.draft = PlayerSettings::default();
                if !streaming_locked {
                    menu.streaming_draft = BroadcastConfig::default();
                }
                "Restored default settings in this draft".clone_into(&mut menu.feedback);
            }
            SETTINGS_BACK_INDEX => {
                request_settings_close(&mut menu, &player_settings.0, &config.0.twitch.broadcast);
                if menu.confirm_settings_close {
                    menu.selected = 0;
                }
            }
            _ => {
                let selected = menu.selected;
                if settings_value_enabled(selected, streaming_locked) {
                    let menu = &mut *menu;
                    adjust_settings_menu(&mut menu.draft, &mut menu.streaming_draft, selected, 1);
                    menu.feedback.clear();
                } else {
                    "End the stream before changing streaming settings."
                        .clone_into(&mut menu.feedback);
                }
            }
        }
        return;
    }
    if !keyboard.just_pressed(KeyCode::Enter) {
        return;
    }
    if *state.get() == GameState::InGame {
        match menu.selected {
            0 => {
                io.save = true;
                "Save requested".clone_into(&mut menu.feedback);
                menu.page = MenuPage::Closed;
            }
            1 => {
                io.save_jump_start = true;
                "Protected jump-start save requested".clone_into(&mut menu.feedback);
                menu.page = MenuPage::Closed;
            }
            2 if save.store.path().is_file() => {
                io.load = true;
                io.load_source = None;
                menu.page = MenuPage::Closed;
            }
            3 => open_settings_menu(
                &mut menu,
                MenuPage::Game,
                &player_settings.0,
                &config.0.twitch.broadcast,
            ),
            4 => {
                #[cfg(target_os = "windows")]
                toggle_direct_broadcast(
                    &config.0,
                    &secrets,
                    &connection,
                    &broadcast_runtime,
                    &mut broadcast_control,
                    &mut menu.feedback,
                );
                #[cfg(not(target_os = "windows"))]
                "Direct Twitch streaming is available only on Windows."
                    .clone_into(&mut menu.feedback);
            }
            5 => {
                menu.page = MenuPage::Closed;
                next_state.set(GameState::MainMenu);
            }
            _ => "No native save exists yet".clone_into(&mut menu.feedback),
        }
    } else {
        match menu.selected {
            0 => {
                if twitch_accounts_connected(&config.0, &secrets, &connection) {
                    open_new_town_dialog(&mut menu);
                } else {
                    open_twitch_setup_required(&mut menu);
                }
            }
            1 if !town_catalog.entries().is_empty() => {
                if twitch_accounts_connected(&config.0, &secrets, &connection) {
                    open_load_town_dialog(&mut menu);
                } else {
                    open_twitch_setup_required(&mut menu);
                }
            }
            2 => open_settings_menu(
                &mut menu,
                MenuPage::Game,
                &player_settings.0,
                &config.0.twitch.broadcast,
            ),
            3 => {
                menu.page = MenuPage::Closed;
                next_state.set(GameState::Credits);
            }
            4 => {
                exit.write(AppExit::Success);
            }
            _ => "No native save exists yet".clone_into(&mut menu.feedback),
        }
    }
}

pub(crate) fn open_settings_menu(
    menu: &mut MenuRuntime,
    return_page: MenuPage,
    settings: &PlayerSettings,
    streaming: &BroadcastConfig,
) {
    menu.page = MenuPage::Settings;
    menu.return_page = return_page;
    menu.selected = 0;
    menu.settings_tab = SettingsTab::Video;
    menu.confirm_settings_close = false;
    menu.draft = settings.clone();
    menu.streaming_draft = streaming.clone();
    menu.feedback.clear();
}

pub(crate) fn adjust_settings_menu(
    settings: &mut PlayerSettings,
    streaming: &mut BroadcastConfig,
    selected: usize,
    direction: i8,
) {
    let increase = direction > 0;
    match selected {
        0 => {
            settings.video.display_mode = match (settings.video.display_mode, increase) {
                (DisplayMode::Windowed, true) | (DisplayMode::Fullscreen, false) => {
                    DisplayMode::Borderless
                }
                (DisplayMode::Borderless, true) | (DisplayMode::Windowed, false) => {
                    DisplayMode::Fullscreen
                }
                (DisplayMode::Borderless, false) | (DisplayMode::Fullscreen, true) => {
                    DisplayMode::Windowed
                }
            };
        }
        1 => {
            const RESOLUTIONS: [(u32, u32); 6] = [
                (1_280, 720),
                (1_600, 900),
                (1_920, 1_080),
                (2_560, 1_440),
                (3_440, 1_440),
                (3_840, 2_160),
            ];
            let resolution = cycle_choice(
                &RESOLUTIONS,
                (settings.video.width, settings.video.height),
                increase,
            );
            (settings.video.width, settings.video.height) = resolution;
        }
        2 => settings.video.vsync = !settings.video.vsync,
        3 => {
            const LIMITS: [Option<u16>; 6] =
                [Some(24), Some(30), Some(60), Some(120), Some(240), None];
            settings.video.fps_limit = cycle_choice(&LIMITS, settings.video.fps_limit, increase);
        }
        4 => settings.video.shadows_enabled = !settings.video.shadows_enabled,
        5 => {
            const RESOLUTIONS: [u16; 5] = [256, 512, 1_024, 2_048, 4_096];
            settings.video.shadow_map_resolution =
                cycle_choice(&RESOLUTIONS, settings.video.shadow_map_resolution, increase);
        }
        6 => settings.video.ambient_occlusion = !settings.video.ambient_occlusion,
        7 => {
            const SAMPLES: [u8; 4] = [1, 2, 4, 8];
            settings.video.msaa_samples =
                cycle_choice(&SAMPLES, settings.video.msaa_samples, increase);
        }
        8 => {
            const MODES: [PostProcessAntiAliasing; 3] = [
                PostProcessAntiAliasing::None,
                PostProcessAntiAliasing::Fxaa,
                PostProcessAntiAliasing::Smaa,
            ];
            settings.video.post_process_aa =
                cycle_choice(&MODES, settings.video.post_process_aa, increase);
        }
        9 => {
            settings.video.brightness_ev =
                step_f32(settings.video.brightness_ev, direction, -5.0, 5.0, 0.5);
        }
        10 => settings.video.gamma = step_f32(settings.video.gamma, direction, -5.0, 5.0, 0.5),
        11 => settings.audio.master = step_f32(settings.audio.master, direction, 0.0, 1.0, 0.05),
        12 => settings.audio.music = step_f32(settings.audio.music, direction, 0.0, 1.0, 0.05),
        13 => {
            settings.audio.sound_effects =
                step_f32(settings.audio.sound_effects, direction, 0.0, 1.0, 0.05);
        }
        14 => {
            settings.audio.ambience = step_f32(settings.audio.ambience, direction, 0.0, 1.0, 0.05);
        }
        15 => {
            settings.camera.zoom_sensitivity =
                step_f32(settings.camera.zoom_sensitivity, direction, 0.0, 100.0, 1.0);
        }
        16 => {
            settings.camera.field_of_view_degrees =
                step_u16(settings.camera.field_of_view_degrees, direction, 30, 120, 5);
        }
        17 => {
            const MODES: [NameDisplayMode; 3] = [
                NameDisplayMode::None,
                NameDisplayMode::StaffAndSubscribers,
                NameDisplayMode::AllPlayers,
            ];
            settings.interface.display_names =
                cycle_choice(&MODES, settings.interface.display_names, increase);
        }
        18 => {
            const MODES: [BuildingHealthDisplayMode; 3] = [
                BuildingHealthDisplayMode::None,
                BuildingHealthDisplayMode::DamagedOnly,
                BuildingHealthDisplayMode::Always,
            ];
            settings.interface.display_building_health =
                cycle_choice(&MODES, settings.interface.display_building_health, increase);
        }
        19 => {
            const MINUTES: [u16; 6] = [0, 1, 5, 10, 30, 60];
            settings.autosave_minutes = cycle_choice(&MINUTES, settings.autosave_minutes, increase);
        }
        20 => {
            const INTERVALS: [TimelapseInterval; 7] = [
                TimelapseInterval::Off,
                TimelapseInterval::TenMinutes,
                TimelapseInterval::OneHour,
                TimelapseInterval::ThreeHours,
                TimelapseInterval::SixHours,
                TimelapseInterval::TwelveHours,
                TimelapseInterval::TwentyFourHours,
            ];
            settings.timelapse.interval =
                cycle_choice(&INTERVALS, settings.timelapse.interval, increase);
        }
        21 => settings.timelapse.dynamic = !settings.timelapse.dynamic,
        22 => {
            const SCALES: [u16; 6] = [75, 90, 100, 110, 125, 150];
            settings.interface.ui_scale_percent =
                cycle_choice(&SCALES, settings.interface.ui_scale_percent, increase);
        }
        23 => settings.interface.high_contrast = !settings.interface.high_contrast,
        24 => settings.interface.reduced_motion = !settings.interface.reduced_motion,
        25 => streaming.enabled = !streaming.enabled,
        26 => {
            const RESOLUTIONS: [(u16, u16); 4] =
                [(640, 360), (854, 480), (1_280, 720), (1_920, 1_080)];
            (streaming.width, streaming.height) =
                cycle_choice(&RESOLUTIONS, (streaming.width, streaming.height), increase);
        }
        27 => {
            const FRAME_RATES: [u8; 2] = [30, 60];
            streaming.frames_per_second =
                cycle_choice(&FRAME_RATES, streaming.frames_per_second, increase);
        }
        28 => {
            const VIDEO_BITRATES: [u32; 7] = [500, 1_000, 2_000, 3_000, 4_500, 5_000, 6_000];
            streaming.video_bitrate_kbps =
                cycle_choice(&VIDEO_BITRATES, streaming.video_bitrate_kbps, increase);
        }
        29 => {
            const AUDIO_BITRATES: [u16; 4] = [64, 96, 128, 160];
            streaming.audio_bitrate_kbps =
                cycle_choice(&AUDIO_BITRATES, streaming.audio_bitrate_kbps, increase);
        }
        30 => {
            const ENCODERS: [BroadcastEncoderPreference; 7] = [
                BroadcastEncoderPreference::Auto,
                BroadcastEncoderPreference::Nvidia,
                BroadcastEncoderPreference::Intel,
                BroadcastEncoderPreference::Amd,
                BroadcastEncoderPreference::MediaFoundation,
                BroadcastEncoderPreference::X264,
                BroadcastEncoderPreference::OpenH264,
            ];
            streaming.encoder = cycle_choice(&ENCODERS, streaming.encoder, increase);
        }
        31 => streaming.bandwidth_test = !streaming.bandwidth_test,
        32 => {
            const MODES: [BroadcastRenderMode; 2] =
                [BroadcastRenderMode::StreamOnly, BroadcastRenderMode::Headed];
            streaming.render_mode = cycle_choice(&MODES, streaming.render_mode, increase);
        }
        _ => {}
    }
}

pub(crate) fn cycle_choice<T: Copy + PartialEq>(choices: &[T], current: T, increase: bool) -> T {
    let current = choices
        .iter()
        .position(|choice| *choice == current)
        .unwrap_or_default();
    let next = if increase {
        (current + 1) % choices.len()
    } else {
        current.checked_sub(1).unwrap_or(choices.len() - 1)
    };
    choices[next]
}

pub(crate) fn step_f32(value: f32, direction: i8, minimum: f32, maximum: f32, step: f32) -> f32 {
    (value + f32::from(direction) * step).clamp(minimum, maximum)
}

pub(crate) fn step_u16(value: u16, direction: i8, minimum: u16, maximum: u16, step: u16) -> u16 {
    let delta = i32::from(direction) * i32::from(step);
    u16::try_from((i32::from(value) + delta).clamp(i32::from(minimum), i32::from(maximum)))
        .expect("clamped camera setting fits u16")
}

pub(crate) fn debug_start_day() -> Option<u32> {
    std::env::var_os("STREAM_TOWN_DEBUG_DAY")
        .and_then(|value| value.to_str().and_then(|value| value.parse().ok()))
}

pub(crate) fn debug_weather_override() -> Option<Weather> {
    std::env::var_os("STREAM_TOWN_DEBUG_WEATHER")
        .and_then(|value| value.to_str().and_then(parse_weather))
}

pub(crate) fn debug_building_health(max_health: i32) -> Option<i32> {
    std::env::var_os("STREAM_TOWN_DEBUG_BUILDING_HEALTH")
        .and_then(|value| value.to_str().and_then(|value| value.parse().ok()))
        .map(|health: i32| health.clamp(0, max_health))
}

pub(crate) fn runtime_initial_agents(configured: u16) -> u16 {
    let configured = if std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some() {
        configured.max(300)
    } else {
        configured
    };
    std::env::var_os("STREAM_TOWN_DEBUG_INITIAL_AGENTS")
        .and_then(|value| value.to_str().and_then(|value| value.parse().ok()))
        .map_or(configured, |agents: u16| agents.clamp(1, 5_000))
}

pub(crate) fn debug_smoke_pet() -> Option<StableId> {
    let value = std::env::var("STREAM_TOWN_SMOKE_PET").ok()?;
    let suffix = match value.trim().to_ascii_lowercase().as_str() {
        "red_panda" | "red-panda" | "panda" => "red_panda",
        "giraffe" => "giraffe",
        "duck" => "duck",
        "butterfly" => "butterfly",
        "fish_god" | "fish-god" | "fishgod" => "fish_god",
        _ => return None,
    };
    StableId::new(format!("pet:{suffix}")).ok()
}

pub(crate) fn actor_scene_budget() -> usize {
    actor_detail_budget(
        std::env::var("STREAM_TOWN_ACTOR_SCENE_BUDGET")
            .ok()
            .as_deref(),
        std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some(),
    )
}

pub(crate) fn actor_detail_budget(value: Option<&str>, benchmarking: bool) -> usize {
    value
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(if benchmarking {
            PERFORMANCE_ACTOR_DETAIL_BUDGET
        } else {
            DEFAULT_ACTOR_DETAIL_BUDGET
        })
}

pub(crate) fn animation_detail_budget() -> usize {
    actor_detail_budget(
        std::env::var("STREAM_TOWN_ANIMATION_BUDGET")
            .ok()
            .as_deref(),
        std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some(),
    )
}

pub(crate) fn parse_weather(value: &str) -> Option<Weather> {
    match value.to_ascii_lowercase().as_str() {
        "clear" => Some(Weather::Clear),
        "rain" => Some(Weather::Rain),
        "fog" => Some(Weather::Fog),
        "snow" => Some(Weather::Snow),
        _ => None,
    }
}

pub(crate) fn initial_actor_identity(index: u16) -> (String, Option<&'static str>) {
    match index {
        0 => ("npc:starting_defender".to_owned(), Some("role:defender")),
        1 => ("npc:starting_logger".to_owned(), Some("role:logger")),
        2 => ("npc:starting_miner".to_owned(), Some("role:miner")),
        3 => ("npc:starting_gatherer".to_owned(), Some("role:gatherer")),
        4 => ("npc:starting_builder".to_owned(), Some("role:builder")),
        _ => (format!("actor:viewer_{index:04}"), None),
    }
}
