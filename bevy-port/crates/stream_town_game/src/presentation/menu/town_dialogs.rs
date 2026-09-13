pub(crate) fn town_save_entry(path: &Path) -> TownSaveEntry {
    let protected = is_jump_start_path(path);
    let stem = path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("Town");
    TownSaveEntry {
        name: if protected {
            format!("{} (Jump-Start)", stem.trim_end_matches(".jumpstart"))
        } else {
            stem.to_owned()
        },
        path: path.to_path_buf(),
        protected,
    }
}

pub(crate) fn is_jump_start_path(path: &Path) -> bool {
    path.file_stem()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".jumpstart"))
}

pub(crate) fn unique_save_path(directory: &Path, preferred_stem: &str) -> PathBuf {
    let preferred_stem = safe_town_filename(preferred_stem);
    let direct = directory.join(format!("{preferred_stem}.stbevy"));
    if !direct.exists() {
        return direct;
    }
    for index in 2_u32.. {
        let candidate = directory.join(format!("{preferred_stem} {index}.stbevy"));
        if !candidate.exists() {
            return candidate;
        }
    }
    unreachable!("the jump-start suffix range is unbounded")
}

pub(crate) fn jump_start_snapshot_path(active: &Path) -> PathBuf {
    let directory = active.parent().unwrap_or_else(|| Path::new("."));
    let stem = safe_town_filename(
        active
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("Town"),
    );
    let direct = directory.join(format!("{stem}.jumpstart.stbevy"));
    if !direct.exists() {
        return direct;
    }
    for index in 2_u32.. {
        let candidate = directory.join(format!("{stem} {index}.jumpstart.stbevy"));
        if !candidate.exists() {
            return candidate;
        }
    }
    unreachable!("the jump-start suffix range is unbounded")
}

pub(crate) fn jump_start_working_path(
    catalog: &TownSaveCatalogRuntime,
    template: &Path,
) -> PathBuf {
    let stem = template
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("Town")
        .trim_end_matches(".jumpstart");
    unique_save_path(&catalog.directory, &format!("{stem} Jump-Start Copy"))
}

pub(crate) fn safe_town_filename(name: &str) -> String {
    let mut safe = name
        .trim()
        .chars()
        .map(|character| {
            if character.is_control()
                || matches!(
                    character,
                    '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
                )
            {
                '_'
            } else {
                character
            }
        })
        .collect::<String>();
    while safe.ends_with(' ') || safe.ends_with('.') {
        safe.pop();
    }
    if safe.is_empty()
        || matches!(
            safe.to_ascii_uppercase().as_str(),
            "CON"
                | "PRN"
                | "AUX"
                | "NUL"
                | "COM1"
                | "COM2"
                | "COM3"
                | "COM4"
                | "COM5"
                | "COM6"
                | "COM7"
                | "COM8"
                | "COM9"
                | "LPT1"
                | "LPT2"
                | "LPT3"
                | "LPT4"
                | "LPT5"
                | "LPT6"
                | "LPT7"
                | "LPT8"
                | "LPT9"
        )
    {
        safe.insert_str(0, "Town_");
    }
    safe
}

pub(crate) fn town_name_seed(name: &str) -> u64 {
    // Stable FNV-1a: unlike DefaultHasher this is identical across processes,
    // platforms, save/load cycles, and future Rust standard-library releases.
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in name.trim().to_lowercase().bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash.max(1)
}

pub(crate) fn open_new_town_dialog(menu: &mut MenuRuntime) {
    menu.page = MenuPage::NewTown;
    menu.return_page = MenuPage::Closed;
    menu.selected = 0;
    menu.feedback.clear();
}

pub(crate) fn open_load_town_dialog(menu: &mut MenuRuntime) {
    menu.page = MenuPage::LoadTown;
    menu.return_page = MenuPage::Closed;
    menu.selected = 0;
    menu.feedback.clear();
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn town_dialog_buttons(
    keyboard: Res<ButtonInput<KeyCode>>,
    actions: Query<(&Interaction, &TownDialogAction), Changed<Interaction>>,
    choices: Query<(&Interaction, &TownLoadChoice), Changed<Interaction>>,
    fields: Query<&EditableText, With<TownNameField>>,
    mut menu: ResMut<MenuRuntime>,
    mut catalog: ResMut<TownSaveCatalogRuntime>,
    mut save: ResMut<SaveRuntime>,
    mut config: ResMut<RuntimeConfig>,
    mut focus: ResMut<InputFocus>,
) {
    if !matches!(menu.page, MenuPage::NewTown | MenuPage::LoadTown) {
        return;
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        menu.page = MenuPage::Closed;
        menu.feedback.clear();
        focus.clear();
        return;
    }
    for (interaction, action) in &actions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            TownDialogAction::Back => {
                menu.page = MenuPage::Closed;
                menu.feedback.clear();
                focus.clear();
            }
            TownDialogAction::Create if menu.page == MenuPage::NewTown => {
                let name = fields
                    .single()
                    .ok()
                    .map(|field| field.value().to_string().trim().to_owned())
                    .unwrap_or_default();
                if name.is_empty() {
                    "Enter a town name first.".clone_into(&mut menu.feedback);
                    continue;
                }
                let path = catalog.new_town_path(&name);
                if path.is_file() {
                    format!("A town named '{name}' already exists. Choose Load Game instead.")
                        .clone_into(&mut menu.feedback);
                    continue;
                }
                save.store = NativeSaveStore::new(path);
                catalog.active_town = Some(name.clone());
                config.0.world.seed = town_name_seed(&name);
                focus.clear();
                request_go_live_confirmation(&mut menu, PendingTownStart::NewGame);
            }
            TownDialogAction::Create => {}
        }
    }
    if menu.page != MenuPage::LoadTown {
        return;
    }
    for (interaction, choice) in &choices {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let store = NativeSaveStore::new(&choice.path);
        match store.load() {
            Ok(snapshot) => {
                config.0.world.seed = snapshot.world_seed;
                if choice.protected {
                    let working_path = jump_start_working_path(&catalog, &choice.path);
                    save.store = NativeSaveStore::new(working_path);
                    catalog.active_town = Some(format!(
                        "{} Copy",
                        choice.name.trim_end_matches(" (Jump-Start)")
                    ));
                } else {
                    save.store = store;
                    catalog.active_town = Some(choice.name.clone());
                }
                focus.clear();
                request_go_live_confirmation(
                    &mut menu,
                    PendingTownStart::LoadGame {
                        source: choice.path.clone(),
                    },
                );
            }
            Err(error) => {
                format!("Could not load '{}': {error}", choice.name).clone_into(&mut menu.feedback);
            }
        }
    }
}

pub(crate) fn update_town_dialog_ui(
    menu: Res<MenuRuntime>,
    mut roots: Query<(&TownDialogRoot, &mut Visibility)>,
    mut feedback: Query<&mut Text, With<TownDialogFeedback>>,
) {
    for (root, mut visibility) in &mut roots {
        *visibility = if menu.page == root.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut text in &mut feedback {
        (**text).clone_from(&menu.feedback);
    }
}

pub(crate) fn scroll_town_load_list(
    menu: Res<MenuRuntime>,
    mut wheel: MessageReader<MouseWheel>,
    mut lists: Query<(&mut ScrollPosition, &ComputedNode), With<TownLoadList>>,
) {
    if menu.page != MenuPage::LoadTown {
        return;
    }
    let delta = wheel.read().fold(0.0, |total, event| {
        total
            + event.y
                * if event.unit == MouseScrollUnit::Line {
                    32.0
                } else {
                    1.0
                }
    });
    if delta.abs() <= f32::EPSILON {
        return;
    }
    for (mut position, computed) in &mut lists {
        let maximum = (computed.content_size().y - computed.size().y).max(0.0)
            * computed.inverse_scale_factor();
        position.y = (position.y - delta).clamp(0.0, maximum);
    }
}

pub(crate) fn main_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<RuntimeConfig>,
    secrets: Res<SecretsRuntime>,
    connection: Res<TwitchConnection>,
    mut menu: ResMut<MenuRuntime>,
    focus: Res<InputFocus>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if menu.page != MenuPage::Closed {
        return;
    }
    if keyboard.just_pressed(KeyCode::Enter) && focus.get().is_none() {
        if twitch_accounts_connected(&config.0, &secrets, &connection) {
            open_new_town_dialog(&mut menu);
        } else {
            open_twitch_setup_required(&mut menu);
        }
    } else if keyboard.just_pressed(KeyCode::KeyC) {
        next_state.set(GameState::Credits);
    } else if keyboard.just_pressed(KeyCode::Escape)
        && !keyboard.pressed(KeyCode::ShiftLeft)
        && !keyboard.pressed(KeyCode::ShiftRight)
    {
        exit.write(AppExit::Success);
    }
}

pub(crate) fn main_menu_action_enabled(action: MainMenuAction, has_save: bool) -> bool {
    action != MainMenuAction::LoadGame || has_save
}

pub(crate) fn main_menu_buttons(
    town_catalog: Res<TownSaveCatalogRuntime>,
    settings: Res<RuntimePlayerSettings>,
    config: Res<RuntimeConfig>,
    secrets: Res<SecretsRuntime>,
    connection: Res<TwitchConnection>,
    mut menu: ResMut<MenuRuntime>,
    buttons: Query<(&Interaction, &MainMenuAction), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if menu.page != MenuPage::Closed {
        return;
    }
    let has_save = !town_catalog.entries().is_empty();
    for (interaction, action) in &buttons {
        if *interaction != Interaction::Pressed || !main_menu_action_enabled(*action, has_save) {
            continue;
        }
        match action {
            MainMenuAction::NewGame => {
                if twitch_accounts_connected(&config.0, &secrets, &connection) {
                    open_new_town_dialog(&mut menu);
                } else {
                    open_twitch_setup_required(&mut menu);
                }
            }
            MainMenuAction::LoadGame => {
                if twitch_accounts_connected(&config.0, &secrets, &connection) {
                    open_load_town_dialog(&mut menu);
                } else {
                    open_twitch_setup_required(&mut menu);
                }
            }
            MainMenuAction::Settings => {
                open_settings_menu(
                    &mut menu,
                    MenuPage::Closed,
                    &settings.0,
                    &config.0.twitch.broadcast,
                );
            }
            MainMenuAction::Secrets => {
                menu.page = MenuPage::SecretsDisclaimer;
                menu.return_page = MenuPage::Closed;
                menu.selected = 0;
                menu.feedback.clear();
            }
            MainMenuAction::Credits => next_state.set(GameState::Credits),
            MainMenuAction::Quit => {
                exit.write(AppExit::Success);
            }
        }
    }
}

pub(crate) fn request_go_live_confirmation(menu: &mut MenuRuntime, start: PendingTownStart) {
    menu.page = MenuPage::GoLiveConfirmation;
    menu.return_page = MenuPage::Closed;
    menu.selected = 1;
    menu.pending_town_start = Some(start);
    menu.feedback.clear();
}

pub(crate) fn cancel_go_live_confirmation(menu: &mut MenuRuntime) {
    menu.page = MenuPage::Closed;
    menu.return_page = MenuPage::Closed;
    menu.selected = 0;
    menu.pending_town_start = None;
    menu.feedback.clear();
}

pub(crate) fn confirm_go_live_and_start_town(
    commands: &mut Commands,
    menu: &mut MenuRuntime,
    io: &mut MenuIoRequest,
    #[cfg(target_os = "windows")] broadcast: &mut direct_broadcast::DirectBroadcastControl,
) {
    let Some(start) = menu.pending_town_start.take() else {
        cancel_go_live_confirmation(menu);
        return;
    };
    match start {
        PendingTownStart::NewGame => {
            io.load = false;
            io.load_source = None;
        }
        PendingTownStart::LoadGame { source } => {
            io.load = true;
            io.load_source = Some(source);
        }
    }
    menu.page = MenuPage::Closed;
    menu.return_page = MenuPage::Closed;
    menu.selected = 0;
    menu.feedback.clear();
    #[cfg(target_os = "windows")]
    broadcast.request_restart();
    queue_world_loading(commands);
}

pub(crate) fn go_live_confirmation_buttons(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut menu: ResMut<MenuRuntime>,
    mut io: ResMut<MenuIoRequest>,
    buttons: Query<(&Interaction, &GoLiveConfirmationAction), Changed<Interaction>>,
    #[cfg(target_os = "windows")] mut broadcast: ResMut<direct_broadcast::DirectBroadcastControl>,
) {
    if menu.page != MenuPage::GoLiveConfirmation {
        return;
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft)
        || keyboard.just_pressed(KeyCode::ArrowRight)
        || keyboard.just_pressed(KeyCode::Tab)
    {
        menu.selected = usize::from(menu.selected == 0);
    }
    let mut action = if keyboard.just_pressed(KeyCode::Escape) {
        Some(GoLiveConfirmationAction::No)
    } else if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        Some(if menu.selected == 0 {
            GoLiveConfirmationAction::No
        } else {
            GoLiveConfirmationAction::Yes
        })
    } else {
        None
    };
    for (interaction, pressed) in &buttons {
        if *interaction == Interaction::Pressed {
            action = Some(*pressed);
        }
    }
    match action {
        Some(GoLiveConfirmationAction::No) => cancel_go_live_confirmation(&mut menu),
        Some(GoLiveConfirmationAction::Yes) => confirm_go_live_and_start_town(
            &mut commands,
            &mut menu,
            &mut io,
            #[cfg(target_os = "windows")]
            &mut broadcast,
        ),
        None => {}
    }
}

pub(crate) fn update_main_menu_buttons(
    town_catalog: Res<TownSaveCatalogRuntime>,
    menu: Res<MenuRuntime>,
    render: Res<RenderAssets>,
    mut buttons: Query<(&Interaction, &MainMenuAction, &mut ImageNode)>,
) {
    let has_save = !town_catalog.entries().is_empty();
    for (interaction, action, mut image) in &mut buttons {
        let enabled = menu.page == MenuPage::Closed && main_menu_action_enabled(*action, has_save);
        let source_path = if !enabled {
            MAIN_MENU_TEXTURE_PATHS[2]
        } else if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
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

pub(crate) fn update_go_live_confirmation_ui(
    menu: Res<MenuRuntime>,
    config: Res<RuntimeConfig>,
    render: Res<RenderAssets>,
    mut root: Query<&mut Visibility, With<GoLiveConfirmationRoot>>,
    mut body: Query<&mut Text, With<GoLiveConfirmationBody>>,
    mut buttons: Query<(&Interaction, &GoLiveConfirmationAction, &mut ImageNode)>,
) {
    if let Ok(mut visibility) = root.single_mut() {
        *visibility = if menu.page == MenuPage::GoLiveConfirmation {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if let Ok(mut text) = body.single_mut() {
        if config.0.twitch.broadcast.bandwidth_test {
            "Starting this town will begin Twitch bandwidth-test output once loading is complete. The test is not publicly listed, but it uses the full stream path. Do you agree to start it?"
        } else {
            "Starting this town will begin the internal Twitch stream once loading is complete. Do you agree to go live?"
        }
        .clone_into(&mut **text);
    }
    for (interaction, action, mut image) in &mut buttons {
        let index = usize::from(*action == GoLiveConfirmationAction::Yes);
        let enabled = menu.page == MenuPage::GoLiveConfirmation;
        let highlighted = enabled
            && (menu.selected == index
                || *interaction == Interaction::Hovered
                || *interaction == Interaction::Pressed);
        let source = if !enabled {
            MAIN_MENU_TEXTURE_PATHS[2]
        } else if highlighted {
            MAIN_MENU_TEXTURE_PATHS[1]
        } else {
            MAIN_MENU_TEXTURE_PATHS[0]
        };
        image.image = main_menu_texture(&render, source);
    }
}

pub(crate) const fn game_menu_action_label(
    action: GameMenuAction,
    broadcast_active: bool,
) -> &'static str {
    match action {
        GameMenuAction::SaveGame => "Save Game",
        GameMenuAction::SaveJumpStart => "Save Jump-Start",
        GameMenuAction::LoadGame => "Load Game",
        GameMenuAction::Settings => "Settings",
        GameMenuAction::GoLive if broadcast_active => "End Stream",
        GameMenuAction::GoLive => "Go Live",
        GameMenuAction::ExitGame => "Exit Game",
        GameMenuAction::Close => "Close",
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn toggle_direct_broadcast(
    config: &GameConfig,
    secrets: &SecretsRuntime,
    connection: &TwitchConnection,
    runtime: &direct_broadcast::DirectBroadcastRuntime,
    control: &mut direct_broadcast::DirectBroadcastControl,
    feedback: &mut String,
) {
    if runtime.snapshot().phase.is_active() {
        control.request_stop_and_return_to_main_menu();
        "Ending the internal Twitch stream and returning to Main Menu...".clone_into(feedback);
    } else if twitch_accounts_connected(config, secrets, connection) {
        control.request_restart();
        "Starting the internal Twitch stream...".clone_into(feedback);
    } else {
        "Connect both Twitch accounts from Main Menu → Secrets before going live."
            .clone_into(feedback);
    }
}

pub(crate) fn game_menu_action_enabled(action: GameMenuAction, has_save: bool) -> bool {
    action != GameMenuAction::LoadGame || has_save
}

pub(crate) const fn game_menu_action_index(action: GameMenuAction) -> usize {
    match action {
        GameMenuAction::SaveGame => 0,
        GameMenuAction::SaveJumpStart => 1,
        GameMenuAction::LoadGame => 2,
        GameMenuAction::Settings => 3,
        GameMenuAction::GoLive => 4,
        GameMenuAction::ExitGame => 5,
        GameMenuAction::Close => 6,
    }
}

pub(crate) fn game_menu_buttons(
    save: Res<SaveRuntime>,
    settings: Res<RuntimePlayerSettings>,
    config: Res<RuntimeConfig>,
    secrets: Res<SecretsRuntime>,
    connection: Res<TwitchConnection>,
    mut menu: ResMut<MenuRuntime>,
    mut io: ResMut<MenuIoRequest>,
    buttons: Query<(&Interaction, &GameMenuAction), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    #[cfg(target_os = "windows")] broadcast_runtime: Res<direct_broadcast::DirectBroadcastRuntime>,
    #[cfg(target_os = "windows")] mut broadcast_control: ResMut<
        direct_broadcast::DirectBroadcastControl,
    >,
) {
    if menu.page != MenuPage::Game {
        return;
    }
    let has_save = save.store.path().is_file();
    for (interaction, action) in &buttons {
        if *interaction != Interaction::Pressed || !game_menu_action_enabled(*action, has_save) {
            continue;
        }
        match action {
            GameMenuAction::SaveGame => {
                io.save = true;
                menu.page = MenuPage::Closed;
            }
            GameMenuAction::SaveJumpStart => {
                io.save_jump_start = true;
                menu.page = MenuPage::Closed;
            }
            GameMenuAction::LoadGame => {
                io.load = true;
                io.load_source = None;
                menu.page = MenuPage::Closed;
            }
            GameMenuAction::Settings => {
                open_settings_menu(
                    &mut menu,
                    MenuPage::Game,
                    &settings.0,
                    &config.0.twitch.broadcast,
                );
            }
            GameMenuAction::GoLive => {
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
            GameMenuAction::ExitGame => {
                menu.page = MenuPage::Closed;
                next_state.set(GameState::MainMenu);
            }
            GameMenuAction::Close => {
                menu.page = MenuPage::Closed;
            }
        }
    }
}
