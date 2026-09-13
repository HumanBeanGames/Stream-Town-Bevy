pub(crate) fn secrets_action_label(action: SecretsAction, config: &GameConfig) -> String {
    match action {
        SecretsAction::DisclaimerYes => "Yes — continue".to_owned(),
        SecretsAction::DisclaimerNo => "No — go back".to_owned(),
        SecretsAction::ToggleBot => format!(
            "Chat bot: {}",
            if config.twitch.enabled {
                "Enabled"
            } else {
                "Disabled"
            }
        ),
        SecretsAction::ToggleBroadcast => format!(
            "Direct stream: {}",
            if config.twitch.broadcast.enabled {
                "Enabled"
            } else {
                "Disabled"
            }
        ),
        SecretsAction::ToggleBandwidthTest => format!(
            "Bandwidth test: {}",
            if config.twitch.broadcast.bandwidth_test {
                "Enabled"
            } else {
                "Disabled"
            }
        ),
        SecretsAction::Save => "Save and apply".to_owned(),
        SecretsAction::AuthorizeBot => "Authorize bot account".to_owned(),
        SecretsAction::AuthorizeBroadcaster => "Authorize stream account".to_owned(),
        SecretsAction::Back => "Back".to_owned(),
    }
}

pub(crate) fn secrets_buttons(
    buttons: Query<(&Interaction, &SecretsAction), Changed<Interaction>>,
    fields: Query<(&SecretsField, &EditableText)>,
    mut menu: ResMut<MenuRuntime>,
    mut secrets: ResMut<SecretsRuntime>,
    mut config: ResMut<RuntimeConfig>,
    mut connection: ResMut<TwitchConnection>,
    mut focus: ResMut<InputFocus>,
) {
    for (interaction, action) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            SecretsAction::DisclaimerNo if menu.page == MenuPage::SecretsDisclaimer => {
                menu.page = MenuPage::Closed;
                focus.clear();
            }
            SecretsAction::DisclaimerYes if menu.page == MenuPage::SecretsDisclaimer => {
                menu.page = MenuPage::Secrets;
                "Internal Twitch video is now blacked out.".clone_into(&mut secrets.feedback);
                focus.clear();
            }
            _ if menu.page != MenuPage::Secrets => {}
            SecretsAction::ToggleBot => {
                let enabled = !config.0.twitch.enabled;
                let outcome = save_secrets_fields(&fields, &mut config, |draft| {
                    draft.twitch.enabled = enabled;
                });
                if outcome.is_ok() {
                    restart_twitch_connection(&config.0, &mut connection);
                }
                set_secrets_save_feedback(outcome, &mut secrets, &mut connection);
            }
            SecretsAction::ToggleBroadcast => {
                let enabled = !config.0.twitch.broadcast.enabled;
                let outcome = save_secrets_fields(&fields, &mut config, |draft| {
                    draft.twitch.broadcast.enabled = enabled;
                });
                set_secrets_save_feedback(outcome, &mut secrets, &mut connection);
            }
            SecretsAction::ToggleBandwidthTest => {
                let enabled = !config.0.twitch.broadcast.bandwidth_test;
                let outcome = save_secrets_fields(&fields, &mut config, |draft| {
                    draft.twitch.broadcast.bandwidth_test = enabled;
                });
                set_secrets_save_feedback(outcome, &mut secrets, &mut connection);
            }
            SecretsAction::Save => {
                let previous = config.0.twitch.clone();
                match save_secrets_fields(&fields, &mut config, |_| {}) {
                    Ok(path) => {
                        let (restart_chat, broadcast_settings_changed) =
                            secrets_restart_requirements(&previous, &config.0.twitch);
                        if restart_chat {
                            restart_twitch_connection(&config.0, &mut connection);
                        }
                        secrets.feedback = if restart_chat {
                            format!(
                                "Saved and applied {}. The bot connection is restarting; the stream remains under manual control.",
                                path.display()
                            )
                        } else if broadcast_settings_changed {
                            format!(
                                "Saved and applied {}. Stream settings will be used the next time you choose Go Live.",
                                path.display()
                            )
                        } else {
                            format!(
                                "Saved and applied {}. Active Twitch connections were preserved.",
                                path.display()
                            )
                        };
                    }
                    Err(error) => {
                        set_secrets_save_feedback(Err(error), &mut secrets, &mut connection);
                    }
                }
            }
            SecretsAction::AuthorizeBot | SecretsAction::AuthorizeBroadcaster => {
                if let Some(kind) = secrets.active_authorization {
                    secrets.feedback = format!(
                        "Finish the current {} authorization before starting another.",
                        kind.label()
                    );
                    continue;
                }
                let kind = if *action == SecretsAction::AuthorizeBot {
                    SecretsAuthorizationKind::Bot
                } else {
                    SecretsAuthorizationKind::Broadcaster
                };
                match save_secrets_fields(&fields, &mut config, |_| {}) {
                    Ok(path) => {
                        secrets.feedback = format!(
                            "Saved {}. Requesting a Twitch {} device code...",
                            path.display(),
                            kind.label()
                        );
                        start_secrets_authorization(kind, config.0.twitch.clone(), &mut secrets);
                    }
                    Err(error) => {
                        secrets.feedback = format!("Cannot authorize: {error:#}");
                    }
                }
            }
            SecretsAction::Back => {
                menu.page = MenuPage::Closed;
                focus.clear();
            }
            SecretsAction::DisclaimerNo | SecretsAction::DisclaimerYes => {}
        }
    }
}

pub(crate) fn save_secrets_fields(
    fields: &Query<(&SecretsField, &EditableText)>,
    config: &mut RuntimeConfig,
    mutate: impl FnOnce(&mut GameConfig),
) -> AnyResult<PathBuf> {
    let mut draft = config.0.clone();
    for (field, editable) in fields {
        let value = editable.value().to_string();
        match field {
            SecretsField::ClientId => value.trim().clone_into(&mut draft.twitch.client_id),
            SecretsField::BotLogin => {
                draft.twitch.bot_login = value.trim().to_ascii_lowercase();
            }
            SecretsField::ChannelLogin => {
                draft.twitch.channel_login = value.trim().to_ascii_lowercase();
            }
        }
    }
    mutate(&mut draft);
    let path = save_runtime_config(&draft)?;
    config.0 = draft;
    Ok(path)
}

pub(crate) fn secrets_restart_requirements(
    previous: &stream_town_domain::TwitchConfig,
    current: &stream_town_domain::TwitchConfig,
) -> (bool, bool) {
    let client_changed = previous.client_id != current.client_id;
    let channel_changed = previous.channel_login != current.channel_login;
    let chat_changed = client_changed || channel_changed || previous.bot_login != current.bot_login;
    let broadcast_changed = client_changed || channel_changed;
    (chat_changed, broadcast_changed)
}

pub(crate) fn set_secrets_save_feedback(
    outcome: AnyResult<PathBuf>,
    secrets: &mut SecretsRuntime,
    connection: &mut TwitchConnection,
) {
    match outcome {
        Ok(path) => {
            secrets.feedback = format!("Saved and applied {}", path.display());
        }
        Err(error) => {
            secrets.feedback = format!("Twitch setup was not saved: {error:#}");
            if matches!(connection.status, TwitchStatus::Authorizing) {
                connection.status = TwitchStatus::Disabled;
            }
        }
    }
}

pub(crate) fn restart_twitch_connection(config: &GameConfig, connection: &mut TwitchConnection) {
    if let Some(transport) = connection.transport.take() {
        let _ = transport.send(TwitchControl::Disconnect);
    }
    if !config.twitch.enabled {
        connection.status = TwitchStatus::Disabled;
        connection.moderation_status = TwitchModerationStatus::Disabled;
        return;
    }
    connection
        .fish_god_reward_id
        .clone_from(&config.twitch.fish_god_reward_id);
    connection.status = TwitchStatus::Authorizing;
    connection.moderation_status = TwitchModerationStatus::Authorizing;
    match TwitchTransport::start(config.twitch.clone()) {
        Ok(transport) => connection.transport = Some(transport),
        Err(error) => {
            connection.status = TwitchStatus::Error(error.to_string());
            connection.moderation_status = TwitchModerationStatus::Disabled;
        }
    }
}

pub(crate) fn reload_twitch_moderation(config: &GameConfig, connection: &mut TwitchConnection) {
    connection.moderation_status = TwitchModerationStatus::Authorizing;
    let reload_sent = connection
        .transport
        .as_ref()
        .is_some_and(|transport| transport.send(TwitchControl::ReloadModeration).is_ok());
    if !reload_sent {
        restart_twitch_connection(config, connection);
    }
}

pub(crate) fn start_secrets_authorization(
    kind: SecretsAuthorizationKind,
    config: stream_town_domain::TwitchConfig,
    secrets: &mut SecretsRuntime,
) {
    let (sender, receiver) = mpsc::channel();
    secrets.authorization_events = Some(Arc::new(Mutex::new(receiver)));
    secrets.active_authorization = Some(kind);
    secrets.device = None;
    let spawn = thread::Builder::new()
        .name(format!("stream-town-menu-{}-oauth", kind.label()))
        .spawn(move || {
            let outcome = (|| -> AnyResult<()> {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .context("failed to construct Twitch authorization runtime")?;
                runtime.block_on(async {
                    let oauth = match kind {
                        SecretsAuthorizationKind::Bot => {
                            OAuthClient::new(config.client_id.clone())?
                        }
                        SecretsAuthorizationKind::Broadcaster => {
                            OAuthClient::broadcaster(config.client_id.clone())?
                        }
                    };
                    let authorization = oauth.begin_device_authorization().await?;
                    sender
                        .send(SecretsAuthorizationEvent::Device {
                            kind,
                            authorization: authorization.clone(),
                        })
                        .context("Twitch secrets screen closed")?;
                    let token = oauth.complete_device_authorization(&authorization).await?;
                    let validation = oauth.validate(&token).await?;
                    let expected_login = match kind {
                        SecretsAuthorizationKind::Bot => &config.bot_login,
                        SecretsAuthorizationKind::Broadcaster => &config.channel_login,
                    };
                    anyhow::ensure!(
                        validation.login == *expected_login,
                        "authorized account '{}' does not match configured {} '{}'",
                        validation.login,
                        kind.label(),
                        expected_login
                    );
                    match kind {
                        SecretsAuthorizationKind::Bot => {
                            CredentialVault::new(&config.client_id, &config.bot_login)
                                .save(&token)?;
                        }
                        SecretsAuthorizationKind::Broadcaster => {
                            CredentialVault::broadcaster(&config.client_id, &config.channel_login)
                                .save(&token)?;
                        }
                    }
                    sender
                        .send(SecretsAuthorizationEvent::Authorized { kind, validation })
                        .context("Twitch secrets screen closed")?;
                    Ok(())
                })
            })();
            if let Err(error) = outcome {
                let _ = sender.send(SecretsAuthorizationEvent::Error {
                    kind,
                    message: format!("{error:#}"),
                });
            }
        });
    if let Err(error) = spawn {
        secrets.authorization_events = None;
        secrets.active_authorization = None;
        secrets.feedback = format!(
            "Could not start Twitch {} authorization: {error}",
            kind.label()
        );
    }
}

pub(crate) fn open_twitch_verification_uri(uri: &str) -> AnyResult<()> {
    open_twitch_verification_uri_with(uri, |uri| {
        webbrowser::open(uri).context("the operating system rejected the browser request")
    })
}

pub(crate) fn open_twitch_verification_uri_with(
    uri: &str,
    open: impl FnOnce(&str) -> AnyResult<()>,
) -> AnyResult<()> {
    let parsed = reqwest::Url::parse(uri).context("Twitch returned an invalid verification URL")?;
    anyhow::ensure!(
        parsed.scheme() == "https",
        "Twitch returned a non-HTTPS verification URL"
    );
    let host = parsed.host_str().unwrap_or_default();
    anyhow::ensure!(
        host == "twitch.tv" || host.ends_with(".twitch.tv"),
        "Twitch returned a verification URL for an unexpected host"
    );
    open(parsed.as_str())
}

pub(crate) fn poll_secrets_authorization(
    mut secrets: ResMut<SecretsRuntime>,
    config: Res<RuntimeConfig>,
    mut connection: ResMut<TwitchConnection>,
) {
    let events = secrets
        .authorization_events
        .as_ref()
        .and_then(|receiver| receiver.lock().ok())
        .map(|receiver| receiver.try_iter().collect::<Vec<_>>())
        .unwrap_or_default();
    for event in events {
        match event {
            SecretsAuthorizationEvent::Device {
                kind,
                authorization,
            } => {
                secrets.feedback = match open_twitch_verification_uri(
                    &authorization.verification_uri,
                ) {
                    Ok(()) => format!(
                        "Opened Twitch {} verification in your browser. Enter code {} to approve it.",
                        kind.label(),
                        authorization.user_code
                    ),
                    Err(error) => format!(
                        "Could not open Twitch {} verification automatically: {error:#}. Open {} and enter code {}.",
                        kind.label(),
                        authorization.verification_uri,
                        authorization.user_code
                    ),
                };
                secrets.device = Some(authorization);
            }
            SecretsAuthorizationEvent::Authorized { kind, validation } => {
                secrets.feedback = format!(
                    "Authorized Twitch {} account '{}'. The token is stored in the OS credential vault.",
                    kind.label(),
                    validation.login
                );
                secrets.device = None;
                secrets.active_authorization = None;
                secrets.authorization_events = None;
                match kind {
                    SecretsAuthorizationKind::Bot => {
                        secrets.bot_credential = SecretsCredentialState::Stored;
                        restart_twitch_connection(&config.0, &mut connection);
                    }
                    SecretsAuthorizationKind::Broadcaster => {
                        secrets.broadcaster_credential = SecretsCredentialState::Stored;
                        reload_twitch_moderation(&config.0, &mut connection);
                    }
                }
            }
            SecretsAuthorizationEvent::Error { kind, message } => {
                secrets.feedback =
                    format!("Twitch {} authorization failed: {message}", kind.label());
                secrets.device = None;
                secrets.active_authorization = None;
                secrets.authorization_events = None;
            }
        }
    }
}

pub(crate) fn refresh_secrets_credential_state(secrets: &mut SecretsRuntime, config: &GameConfig) {
    let signature = (
        config.twitch.client_id.clone(),
        config.twitch.bot_login.clone(),
        config.twitch.channel_login.clone(),
    );
    if secrets.credential_signature.as_ref() == Some(&signature) {
        return;
    }
    secrets.bot_credential = probe_secrets_credential(
        !signature.0.trim().is_empty() && !signature.1.trim().is_empty(),
        CredentialVault::new(&signature.0, &signature.1),
    );
    secrets.broadcaster_credential = probe_secrets_credential(
        !signature.0.trim().is_empty() && !signature.2.trim().is_empty(),
        CredentialVault::broadcaster(&signature.0, &signature.2),
    );
    secrets.credential_signature = Some(signature);
}

pub(crate) fn initialize_twitch_account_state(
    config: Res<RuntimeConfig>,
    mut secrets: ResMut<SecretsRuntime>,
) {
    refresh_secrets_credential_state(&mut secrets, &config.0);
}

pub(crate) fn twitch_accounts_connected(
    config: &GameConfig,
    secrets: &SecretsRuntime,
    connection: &TwitchConnection,
) -> bool {
    config.twitch.enabled
        && config.twitch.broadcast.enabled
        && !config.twitch.client_id.trim().is_empty()
        && !config.twitch.bot_login.trim().is_empty()
        && !config.twitch.channel_login.trim().is_empty()
        && secrets.bot_credential == SecretsCredentialState::Stored
        && secrets.broadcaster_credential == SecretsCredentialState::Stored
        && connection.status == TwitchStatus::Connected
        && connection.moderation_status == TwitchModerationStatus::Ready
}

pub(crate) fn open_twitch_setup_required(menu: &mut MenuRuntime) {
    menu.page = MenuPage::SecretsDisclaimer;
    menu.return_page = MenuPage::Closed;
    menu.selected = 0;
    "Connect both Twitch accounts in Secrets before starting a town."
        .clone_into(&mut menu.feedback);
}

pub(crate) fn probe_secrets_credential(
    configured: bool,
    vault: CredentialVault,
) -> SecretsCredentialState {
    if !configured {
        return SecretsCredentialState::NotConfigured;
    }
    match vault.load() {
        Ok(Some(_)) => SecretsCredentialState::Stored,
        Ok(None) => SecretsCredentialState::Missing,
        Err(error) => SecretsCredentialState::Error(format!("{error:#}")),
    }
}

pub(crate) fn bot_connection_status(
    config: &GameConfig,
    connection: &TwitchConnection,
    credential: &SecretsCredentialState,
) -> (String, SecretsStatusTone) {
    if !config.twitch.enabled {
        return match credential {
            SecretsCredentialState::Stored => (
                "● Bot authorized; chat connection is disabled.".to_owned(),
                SecretsStatusTone::Inactive,
            ),
            SecretsCredentialState::Missing => (
                "● Bot is not authorized and chat is disabled.".to_owned(),
                SecretsStatusTone::Inactive,
            ),
            SecretsCredentialState::NotConfigured => (
                "● Enter a Client ID and bot login to configure chat.".to_owned(),
                SecretsStatusTone::Inactive,
            ),
            SecretsCredentialState::Error(error) => (
                format!("● Could not read bot authorization: {error}"),
                SecretsStatusTone::Error,
            ),
            SecretsCredentialState::Unknown => (
                "● Checking bot authorization...".to_owned(),
                SecretsStatusTone::Pending,
            ),
        };
    }
    match &connection.status {
        TwitchStatus::Connected => (
            format!(
                "● Connected automatically as @{} to #{}; commands are enabled.",
                config.twitch.bot_login, config.twitch.channel_login
            ),
            SecretsStatusTone::Good,
        ),
        TwitchStatus::Authorizing => (
            "● Validating the stored bot authorization...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        TwitchStatus::Connecting => (
            format!(
                "● Connecting the bot to #{}...",
                config.twitch.channel_login
            ),
            SecretsStatusTone::Pending,
        ),
        TwitchStatus::Reconnecting => (
            "● Bot connection was interrupted; reconnecting...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        TwitchStatus::Disconnected => (
            "● Bot is disconnected. Use Save and apply to reconnect.".to_owned(),
            SecretsStatusTone::Error,
        ),
        TwitchStatus::Error(error) => (
            format!("● Bot connection error: {error}"),
            SecretsStatusTone::Error,
        ),
        TwitchStatus::Disabled => (
            "● Bot is enabled in settings and waiting to start.".to_owned(),
            SecretsStatusTone::Pending,
        ),
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn broadcast_connection_status(
    config: &GameConfig,
    credential: &SecretsCredentialState,
    moderation: &TwitchModerationStatus,
    snapshot: &direct_broadcast::DirectBroadcastSnapshot,
) -> (String, SecretsStatusTone) {
    use direct_broadcast::DirectBroadcastPhase;

    if !config.twitch.broadcast.enabled {
        return match credential {
            SecretsCredentialState::Stored => (
                "● Broadcaster authorized; direct stream is disabled.".to_owned(),
                SecretsStatusTone::Inactive,
            ),
            SecretsCredentialState::Missing => (
                "● Broadcaster is not authorized and direct stream is disabled.".to_owned(),
                SecretsStatusTone::Inactive,
            ),
            SecretsCredentialState::NotConfigured => (
                "● Enter a Client ID and channel login to configure streaming.".to_owned(),
                SecretsStatusTone::Inactive,
            ),
            SecretsCredentialState::Error(error) => (
                format!("● Could not read broadcaster authorization: {error}"),
                SecretsStatusTone::Error,
            ),
            SecretsCredentialState::Unknown => (
                "● Checking broadcaster authorization...".to_owned(),
                SecretsStatusTone::Pending,
            ),
        };
    }
    match moderation {
        TwitchModerationStatus::Authorizing => {
            return (
                "● Validating broadcaster streaming, operator chat, and moderation authority..."
                    .to_owned(),
                SecretsStatusTone::Pending,
            );
        }
        TwitchModerationStatus::Error(error) => {
            return (
                format!(
                    "● Broadcaster authorization error: {error}. Use Authorize stream account below."
                ),
                SecretsStatusTone::Error,
            );
        }
        TwitchModerationStatus::Disabled => {
            return (
                "● Broadcaster operator controls are waiting for the Twitch connection to start."
                    .to_owned(),
                SecretsStatusTone::Pending,
            );
        }
        TwitchModerationStatus::Ready => {}
    }
    match &snapshot.phase {
        DirectBroadcastPhase::Disabled => (
            "● Direct stream is enabled and waiting to start.".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::WaitingForBroadcasterAuthorization => (
            "● Validating broadcaster authorization and fetching the stream key...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::WaitingForGameplay => (
            "● Stream prepared; it will begin after the in-game loading cover retires.".to_owned(),
            SecretsStatusTone::Good,
        ),
        DirectBroadcastPhase::ResolvingIngest => (
            "● Selecting a Twitch ingest server...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::Connecting => (
            "● Connecting the encoder to Twitch...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::VerifyingTwitch => (
            "● Encoder output reached Twitch; waiting for Twitch to confirm that the channel is publicly live...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::Broadcasting => {
            (
                format!(
                    "● LIVE (confirmed by Twitch); {:.1} captured / {:.1} output FPS (target {}), {} video / {} audio drops via {} ({})",
                    snapshot.captured_video_fps,
                    snapshot.encoded_video_fps,
                    config.twitch.broadcast.frames_per_second,
                    snapshot.dropped_video_frames,
                    snapshot.dropped_audio_frames,
                    snapshot.encoder.as_deref().unwrap_or("encoder pending"),
                    snapshot.ingest.as_deref().unwrap_or("ingest pending")
                ),
                SecretsStatusTone::Good,
            )
        }
        DirectBroadcastPhase::BandwidthTesting => (
            format!(
                "● BANDWIDTH TEST — not publicly live; {:.1} captured / {:.1} output FPS (target {}), {} video / {} audio drops via {} ({})",
                snapshot.captured_video_fps,
                snapshot.encoded_video_fps,
                config.twitch.broadcast.frames_per_second,
                snapshot.dropped_video_frames,
                snapshot.dropped_audio_frames,
                snapshot.encoder.as_deref().unwrap_or("encoder pending"),
                snapshot.ingest.as_deref().unwrap_or("ingest pending")
            ),
            SecretsStatusTone::Good,
        ),
        DirectBroadcastPhase::Reconnecting => (
            "● Twitch stream interrupted; reconnecting automatically...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::Stopping => (
            "● Stopping the previous stream...".to_owned(),
            SecretsStatusTone::Pending,
        ),
        DirectBroadcastPhase::Stopped => (
            "● Stream stopped. Use Restart stream in the operator panel to start it again."
                .to_owned(),
            SecretsStatusTone::Error,
        ),
        DirectBroadcastPhase::Error(error) => {
            (format!("● Stream error: {error}"), SecretsStatusTone::Error)
        }
    }
}

pub(crate) fn secrets_status_color(tone: SecretsStatusTone) -> Color {
    match tone {
        SecretsStatusTone::Good => Color::srgb(0.48, 0.94, 0.58),
        SecretsStatusTone::Pending => Color::srgb(0.96, 0.78, 0.34),
        SecretsStatusTone::Inactive => Color::srgb(0.68, 0.72, 0.78),
        SecretsStatusTone::Error => Color::srgb(1.0, 0.46, 0.42),
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn update_secrets_ui(
    menu: Res<MenuRuntime>,
    config: Res<RuntimeConfig>,
    mut secrets: ResMut<SecretsRuntime>,
    connection: Res<TwitchConnection>,
    #[cfg(target_os = "windows")] broadcast: Res<direct_broadcast::DirectBroadcastRuntime>,
    render: Res<RenderAssets>,
    mut disclaimer: Query<&mut Visibility, With<SecretsDisclaimerRoot>>,
    mut root: Query<&mut Visibility, (With<SecretsRoot>, Without<SecretsDisclaimerRoot>)>,
    mut status: Query<
        &mut Text,
        (
            With<SecretsStatusText>,
            Without<SecretsDeviceText>,
            Without<SecretsDynamicLabel>,
        ),
    >,
    mut device_text: Query<
        &mut Text,
        (
            With<SecretsDeviceText>,
            Without<SecretsStatusText>,
            Without<SecretsDynamicLabel>,
        ),
    >,
    mut connection_texts: Query<
        (&SecretsConnectionText, &mut Text, &mut TextColor),
        (
            Without<SecretsStatusText>,
            Without<SecretsDeviceText>,
            Without<SecretsDynamicLabel>,
        ),
    >,
    mut labels: Query<(&SecretsDynamicLabel, &mut Text)>,
    mut buttons: Query<(&Interaction, &SecretsAction, &mut ImageNode)>,
) {
    refresh_secrets_credential_state(&mut secrets, &config.0);
    if let Ok(mut visibility) = disclaimer.single_mut() {
        *visibility = if menu.page == MenuPage::SecretsDisclaimer {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if let Ok(mut visibility) = root.single_mut() {
        *visibility = if menu.page == MenuPage::Secrets {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if let Ok(mut text) = status.single_mut() {
        if secrets.feedback.is_empty() {
            SECRETS_INITIAL_FEEDBACK.clone_into(&mut **text);
        } else {
            (**text).clone_from(&secrets.feedback);
        }
    }
    if let Ok(mut text) = device_text.single_mut() {
        **text = secrets.device.as_ref().map_or_else(
            || "No Twitch authorization is in progress.".to_owned(),
            |device| {
                format!(
                    "Open {} and enter code {} (expires in {} minutes).",
                    device.verification_uri,
                    device.user_code,
                    (device.expires_in / 60).max(1)
                )
            },
        );
    }
    for (kind, mut text, mut color) in &mut connection_texts {
        let (status, tone) = match kind.0 {
            SecretsConnectionKind::Bot => {
                bot_connection_status(&config.0, &connection, &secrets.bot_credential)
            }
            SecretsConnectionKind::Broadcast => {
                #[cfg(target_os = "windows")]
                {
                    broadcast_connection_status(
                        &config.0,
                        &secrets.broadcaster_credential,
                        &connection.moderation_status,
                        &broadcast.snapshot(),
                    )
                }
                #[cfg(not(target_os = "windows"))]
                {
                    (
                        "● Direct Twitch streaming is currently available only on Windows."
                            .to_owned(),
                        SecretsStatusTone::Inactive,
                    )
                }
            }
        };
        **text = status;
        color.0 = secrets_status_color(tone);
    }
    for (label, mut text) in &mut labels {
        **text = secrets_action_label(label.0, &config.0);
    }
    for (interaction, action, mut image) in &mut buttons {
        let available = secrets.active_authorization.is_none()
            || !matches!(
                action,
                SecretsAction::AuthorizeBot | SecretsAction::AuthorizeBroadcaster
            );
        let source_path = if !available {
            MAIN_MENU_TEXTURE_PATHS[2]
        } else if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            MAIN_MENU_TEXTURE_PATHS[1]
        } else {
            MAIN_MENU_TEXTURE_PATHS[0]
        };
        image.image = main_menu_texture(&render, source_path);
        image.color = if available {
            Color::WHITE
        } else {
            Color::srgba(0.78, 0.78, 0.78, 0.5)
        };
    }
}

pub(crate) fn sync_sensitive_screen_active(
    menu: Res<MenuRuntime>,
    mut active: ResMut<SensitiveScreenActive>,
) {
    active.0 = menu_page_is_sensitive(menu.page);
}
