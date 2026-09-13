fn twitch_tab(ui: &mut egui::Ui, state: &mut ToolState) {
    ui.heading("Twitch setup and diagnostics");
    ui.label("Use a Twitch public client with the exact chat:read and chat:edit scopes. OAuth tokens live only in the operating-system credential vault.");
    ui.checkbox(
        &mut state.config.twitch.enabled,
        "Enable Twitch in the game",
    );
    ui.horizontal(|ui| {
        ui.label("Client ID");
        ui.text_edit_singleline(&mut state.config.twitch.client_id);
    });
    ui.horizontal(|ui| {
        ui.label("Bot login");
        ui.text_edit_singleline(&mut state.config.twitch.bot_login);
    });
    ui.horizontal(|ui| {
        ui.label("Channel login");
        ui.text_edit_singleline(&mut state.config.twitch.channel_login);
    });
    let busy = state.twitch_auth_events.is_some();
    ui.horizontal(|ui| {
        ui.label("Game-master Twitch user IDs");
        if ui
            .text_edit_singleline(&mut state.game_master_ids)
            .changed()
        {
            state.config.twitch.game_master_ids = parse_game_master_ids(&state.game_master_ids);
        }
    });
    ui.horizontal(|ui| {
        ui.label("Resolve GM login");
        ui.text_edit_singleline(&mut state.game_master_lookup);
        if ui
            .add_enabled(
                !busy
                    && !state.config.twitch.client_id.trim().is_empty()
                    && !state.game_master_lookup.trim().is_empty(),
                egui::Button::new("Resolve and add ID"),
            )
            .clicked()
        {
            start_twitch_game_master_lookup(state);
        }
    });
    ui.label("GM IDs are explicit numeric Twitch user IDs; broadcaster/moderator status does not grant GM commands.");
    ui.horizontal(|ui| {
        ui.label("Fish God reward ID");
        if ui
            .text_edit_singleline(&mut state.fish_god_reward_id)
            .changed()
        {
            let reward_id = state.fish_god_reward_id.trim();
            state.config.twitch.fish_god_reward_id = if reward_id.is_empty() {
                None
            } else {
                Some(reward_id.to_owned())
            };
        }
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Capture next reward"),
            )
            .on_hover_text("Connect to chat, then redeem the intended Channel Points reward within three minutes")
            .clicked()
        {
            start_twitch_reward_capture(state);
        }
    });
    ui.horizontal(|ui| {
        if ui.button("Save runtime config").clicked() {
            state.status = match save_runtime_config(&state.config) {
                Ok(path) => format!("Saved public runtime configuration to {}", path.display()),
                Err(error) => format!("Could not save runtime configuration: {error:#}"),
            };
        }
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Authorize bot"),
            )
            .clicked()
        {
            start_twitch_authorization(state);
        }
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Run end-to-end diagnostic"),
            )
            .clicked()
        {
            start_twitch_diagnostic(state);
        }
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Forget token"),
            )
            .clicked()
        {
            start_twitch_clear(state);
        }
    });
    if let Some(device) = &state.twitch_device {
        ui.separator();
        ui.heading("Authorization pending");
        ui.label(format!("Enter code: {}", device.user_code));
        ui.hyperlink_to("Open Twitch activation page", &device.verification_uri);
        ui.label(format!(
            "This request expires in {} minutes; polling every {} seconds.",
            device.expires_in.div_ceil(60),
            device.interval
        ));
    }
    if let Some(validation) = &state.twitch_validation {
        ui.separator();
        ui.label(format!(
            "Validated bot '{}' (user {}) for client {}",
            validation.login, validation.user_id, validation.client_id
        ));
        ui.label(format!("Scopes: {}", validation.scopes.join(", ")));
        ui.label(format!(
            "Access token valid for {} seconds",
            validation.expires_in
        ));
    }
    if let Some(channel) = &state.twitch_channel_identity {
        ui.label(format!(
            "Resolved channel '{}' ({}, user {})",
            channel.display_name, channel.login, channel.id
        ));
    }
    if state.twitch_irc_verified {
        ui.colored_label(
            egui::Color32::LIGHT_GREEN,
            "Authenticated IRC connection and channel join verified.",
        );
    }

    ui.separator();
    ui.heading("Direct Twitch broadcast (Windows)");
    ui.label("The game captures its own Bevy render target and process audio, then publishes H.264/AAC through shared LGPL FFmpeg libraries. It does not launch FFmpeg, capture the desktop, or require OBS.");
    ui.checkbox(
        &mut state.config.twitch.broadcast.enabled,
        "Enable direct broadcast",
    );
    ui.label("The game always starts offline. Use Go Live from the main-menu status badge or the in-game Esc menu.");
    ui.horizontal(|ui| {
        ui.label("Output");
        ui.add(
            egui::DragValue::new(&mut state.config.twitch.broadcast.width)
                .range(320..=1_920)
                .suffix(" px"),
        );
        ui.label("×");
        ui.add(
            egui::DragValue::new(&mut state.config.twitch.broadcast.height)
                .range(180..=1_080)
                .suffix(" px"),
        );
        egui::ComboBox::from_id_salt("broadcast-fps")
            .selected_text(format!(
                "{} FPS",
                state.config.twitch.broadcast.frames_per_second
            ))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut state.config.twitch.broadcast.frames_per_second,
                    30,
                    "30 FPS",
                );
                ui.selectable_value(
                    &mut state.config.twitch.broadcast.frames_per_second,
                    60,
                    "60 FPS",
                );
            });
    });
    ui.horizontal(|ui| {
        ui.label("Video bitrate");
        ui.add(
            egui::DragValue::new(&mut state.config.twitch.broadcast.video_bitrate_kbps)
                .range(500..=6_000)
                .suffix(" Kbps"),
        );
        ui.label("Audio bitrate");
        ui.add(
            egui::DragValue::new(&mut state.config.twitch.broadcast.audio_bitrate_kbps)
                .range(64..=160)
                .suffix(" Kbps"),
        );
    });
    ui.horizontal(|ui| {
        ui.label("H.264 encoder");
        egui::ComboBox::from_id_salt("broadcast-encoder")
            .selected_text(broadcast_encoder_label(
                state.config.twitch.broadcast.encoder,
            ))
            .show_ui(ui, |ui| {
                for encoder in [
                    BroadcastEncoderPreference::Auto,
                    BroadcastEncoderPreference::Nvidia,
                    BroadcastEncoderPreference::Intel,
                    BroadcastEncoderPreference::Amd,
                    BroadcastEncoderPreference::MediaFoundation,
                    BroadcastEncoderPreference::X264,
                    BroadcastEncoderPreference::OpenH264,
                ] {
                    ui.selectable_value(
                        &mut state.config.twitch.broadcast.encoder,
                        encoder,
                        broadcast_encoder_label(encoder),
                    );
                }
            });
        ui.label("Preferred ingest");
        ui.text_edit_singleline(&mut state.config.twitch.broadcast.ingest)
            .on_hover_text(
                "Optional name substring such as Sydney; empty uses Twitch's default ingest",
            );
    });
    ui.checkbox(
        &mut state.config.twitch.broadcast.bandwidth_test,
        "Bandwidth-test mode (does not go live)",
    );
    if state.config.twitch.broadcast.bandwidth_test {
        ui.colored_label(
            egui::Color32::YELLOW,
            "Bandwidth-test mode still sends the configured bitrate to Twitch. Turn it off before a real broadcast.",
        );
    }
    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Authorize broadcaster"),
            )
            .on_hover_text(
                "Requests only channel:read:stream_key for the configured channel account",
            )
            .clicked()
        {
            start_twitch_broadcast_authorization(state);
        }
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Test broadcast prerequisites"),
            )
            .clicked()
        {
            start_twitch_broadcast_diagnostic(state);
        }
        if ui
            .add_enabled(
                !busy && !state.config.twitch.client_id.trim().is_empty(),
                egui::Button::new("Forget broadcaster token"),
            )
            .clicked()
        {
            start_twitch_broadcast_clear(state);
        }
    });
    if let Some(validation) = &state.twitch_broadcast_validation {
        ui.colored_label(
            egui::Color32::LIGHT_GREEN,
            format!(
                "Broadcaster '{}' is authorized (user {}, token valid for {} seconds).",
                validation.login, validation.user_id, validation.expires_in
            ),
        );
    }
    ui.colored_label(
        egui::Color32::LIGHT_BLUE,
        "No client secret, OAuth credential, or stream key is stored in repository assets.",
    );
}

const fn broadcast_encoder_label(encoder: BroadcastEncoderPreference) -> &'static str {
    match encoder {
        BroadcastEncoderPreference::Auto => "Automatic",
        BroadcastEncoderPreference::Nvidia => "NVIDIA NVENC",
        BroadcastEncoderPreference::Intel => "Intel Quick Sync",
        BroadcastEncoderPreference::Amd => "AMD AMF",
        BroadcastEncoderPreference::MediaFoundation => "Windows Media Foundation",
        BroadcastEncoderPreference::X264 => "x264 (CPU, GPL)",
        BroadcastEncoderPreference::OpenH264 => "OpenH264 (CPU)",
    }
}

fn poll_twitch_tool_events(state: &mut ToolState) {
    let events: Vec<_> = state
        .twitch_auth_events
        .as_ref()
        .and_then(|receiver| receiver.lock().ok())
        .map(|receiver| receiver.try_iter().collect())
        .unwrap_or_default();
    let mut finished = false;
    for event in events {
        match event {
            TwitchToolEvent::Device(device) => {
                "Twitch is waiting for device authorization".clone_into(&mut state.status);
                state.twitch_device = Some(device);
            }
            TwitchToolEvent::Authorized(validation) => {
                state.status = format!(
                    "Authorized and securely stored Twitch bot '{}'",
                    validation.login
                );
                state.twitch_validation = Some(validation);
                state.twitch_channel_identity = None;
                state.twitch_irc_verified = false;
                state.twitch_device = None;
                finished = true;
            }
            TwitchToolEvent::BroadcastAuthorized(validation) => {
                state.status = format!(
                    "Authorized and securely stored Twitch broadcaster '{}'",
                    validation.login
                );
                state.twitch_broadcast_validation = Some(validation);
                state.twitch_device = None;
                finished = true;
            }
            TwitchToolEvent::Progress(message) => {
                state.status = message;
            }
            TwitchToolEvent::Diagnostic {
                validation,
                channel,
            } => {
                state.status = format!(
                    "Twitch bot '{}' validated and joined channel '{}'",
                    validation.login, channel.login
                );
                state.twitch_validation = Some(validation);
                state.twitch_channel_identity = Some(channel);
                state.twitch_irc_verified = true;
                finished = true;
            }
            TwitchToolEvent::BroadcastDiagnostic {
                validation,
                ingest_count,
                prerequisites,
            } => {
                state.status = format!(
                    "Direct broadcast ready for '{}': {ingest_count} Twitch ingests; selected encoder: {}; rejected before selection: {}; available: {}; process audio: {}",
                    validation.login,
                    prerequisites.selected_encoder,
                    if prerequisites.rejected_encoders.is_empty() {
                        "none".to_owned()
                    } else {
                        prerequisites.rejected_encoders.join(" | ")
                    },
                    prerequisites.available_encoders.join(", "),
                    if prerequisites.process_audio_capture_available {
                        "ready"
                    } else {
                        "unavailable"
                    }
                );
                state.twitch_broadcast_validation = Some(validation);
                state.twitch_device = None;
                finished = true;
            }
            TwitchToolEvent::GameMasterResolved(identity) => {
                state
                    .config
                    .twitch
                    .game_master_ids
                    .insert(identity.id.clone());
                state.game_master_ids =
                    format_game_master_ids(&state.config.twitch.game_master_ids);
                state.status = format!(
                    "Resolved Twitch GM '{}' to numeric user ID {}",
                    identity.login, identity.id
                );
                finished = true;
            }
            TwitchToolEvent::RewardCaptured(reward_id) => {
                state.config.twitch.fish_god_reward_id = Some(reward_id.clone());
                state.fish_god_reward_id.clone_from(&reward_id);
                state.status = format!(
                    "Captured Channel Points reward ID {reward_id}; save the runtime config"
                );
                finished = true;
            }
            TwitchToolEvent::Cleared => {
                "Removed the Twitch token from the OS credential vault"
                    .clone_into(&mut state.status);
                state.twitch_validation = None;
                state.twitch_channel_identity = None;
                state.twitch_irc_verified = false;
                state.twitch_device = None;
                finished = true;
            }
            TwitchToolEvent::BroadcastCleared => {
                "Removed the Twitch broadcaster token from the OS credential vault"
                    .clone_into(&mut state.status);
                state.twitch_broadcast_validation = None;
                state.twitch_device = None;
                finished = true;
            }
            TwitchToolEvent::Error(error) => {
                state.status = format!("Twitch setup failed: {error}");
                state.twitch_device = None;
                state.twitch_irc_verified = false;
                finished = true;
            }
        }
    }
    if finished {
        state.twitch_auth_events = None;
    }
}

fn twitch_event_channel(state: &mut ToolState) -> mpsc::Sender<TwitchToolEvent> {
    let (sender, receiver) = mpsc::channel();
    state.twitch_auth_events = Some(Arc::new(Mutex::new(receiver)));
    sender
}

fn start_twitch_authorization(state: &mut ToolState) {
    state.twitch_device = None;
    state.twitch_validation = None;
    state.twitch_channel_identity = None;
    state.twitch_irc_verified = false;
    "Starting Twitch device authorization...".clone_into(&mut state.status);
    let config = state.config.twitch.clone();
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-oauth".to_owned())
        .spawn(move || {
            let outcome = (|| -> anyhow::Result<()> {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                runtime.block_on(async {
                    let oauth = OAuthClient::new(config.client_id.clone())?;
                    let authorization = oauth.begin_device_authorization().await?;
                    sender
                        .send(TwitchToolEvent::Device(authorization.clone()))
                        .map_err(|_| anyhow::anyhow!("Twitch setup window closed"))?;
                    let token = oauth.complete_device_authorization(&authorization).await?;
                    let validation = oauth.validate(&token).await?;
                    anyhow::ensure!(
                        validation.login == config.bot_login,
                        "authorized account '{}' does not match configured bot '{}'",
                        validation.login,
                        config.bot_login
                    );
                    CredentialVault::new(&config.client_id, &config.bot_login).save(&token)?;
                    sender
                        .send(TwitchToolEvent::Authorized(validation))
                        .map_err(|_| anyhow::anyhow!("Twitch setup window closed"))?;
                    Ok(())
                })
            })();
            if let Err(error) = outcome {
                let _ = sender.send(TwitchToolEvent::Error(format!("{error:#}")));
            }
        });
    if let Err(error) = worker {
        state.status = format!("Could not start Twitch authorization worker: {error}");
        state.twitch_auth_events = None;
    }
}

fn start_twitch_broadcast_authorization(state: &mut ToolState) {
    state.twitch_device = None;
    state.twitch_broadcast_validation = None;
    "Starting Twitch broadcaster authorization...".clone_into(&mut state.status);
    let config = state.config.twitch.clone();
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-broadcast-oauth".to_owned())
        .spawn(move || {
            let outcome = (|| -> anyhow::Result<()> {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                runtime.block_on(async {
                    let oauth = OAuthClient::broadcaster(config.client_id.clone())?;
                    let authorization = oauth.begin_device_authorization().await?;
                    sender
                        .send(TwitchToolEvent::Device(authorization.clone()))
                        .map_err(|_| anyhow::anyhow!("Twitch setup window closed"))?;
                    let token = oauth.complete_device_authorization(&authorization).await?;
                    let validation = oauth.validate(&token).await?;
                    anyhow::ensure!(
                        validation.login == config.channel_login,
                        "authorized account '{}' does not match configured channel '{}'",
                        validation.login,
                        config.channel_login
                    );
                    CredentialVault::broadcaster(&config.client_id, &config.channel_login)
                        .save(&token)?;
                    sender
                        .send(TwitchToolEvent::BroadcastAuthorized(validation))
                        .map_err(|_| anyhow::anyhow!("Twitch setup window closed"))?;
                    Ok(())
                })
            })();
            if let Err(error) = outcome {
                let _ = sender.send(TwitchToolEvent::Error(format!("{error:#}")));
            }
        });
    if let Err(error) = worker {
        state.status = format!("Could not start Twitch broadcaster authorization: {error}");
        state.twitch_auth_events = None;
    }
}

fn start_twitch_broadcast_diagnostic(state: &mut ToolState) {
    "Validating broadcaster authorization, Twitch ingest access, shared FFmpeg, and process audio..."
        .clone_into(&mut state.status);
    let config = state.config.twitch.clone();
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-broadcast-check".to_owned())
        .spawn(move || {
            let outcome =
                (|| -> anyhow::Result<(TokenValidation, usize, BroadcastPrerequisites)> {
                    let runtime = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()?;
                    let (validation, ingest_count) = runtime.block_on(async {
                        let oauth = OAuthClient::broadcaster(config.client_id.clone())?;
                        let vault =
                            CredentialVault::broadcaster(&config.client_id, &config.channel_login);
                        let (token, validation) = oauth.load_validated_token(&vault).await?;
                        anyhow::ensure!(
                            validation.login == config.channel_login,
                            "stored broadcaster token belongs to '{}', expected '{}'",
                            validation.login,
                            config.channel_login
                        );
                        // Confirm Helix can return a non-empty key without exposing or persisting it.
                        let stream_key = oauth.stream_key(&token, &validation.user_id).await?;
                        drop(stream_key);
                        let ingests = oauth.ingests().await?;
                        Ok::<_, anyhow::Error>((validation, ingests.len()))
                    })?;
                    let prerequisites = inspect_broadcast_prerequisites(&config.broadcast)?;
                    anyhow::ensure!(
                        prerequisites.process_audio_capture_available,
                        "Windows process-scoped audio capture is unavailable"
                    );
                    Ok((validation, ingest_count, prerequisites))
                })();
            let event = outcome.map_or_else(
                |error| TwitchToolEvent::Error(format!("{error:#}")),
                |(validation, ingest_count, prerequisites)| TwitchToolEvent::BroadcastDiagnostic {
                    validation,
                    ingest_count,
                    prerequisites,
                },
            );
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not start direct-broadcast diagnostic: {error}");
        state.twitch_auth_events = None;
    }
}

fn start_twitch_diagnostic(state: &mut ToolState) {
    "Validating and refreshing the Twitch token...".clone_into(&mut state.status);
    let config = state.config.twitch.clone();
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-twitch-check".to_owned())
        .spawn(move || {
            let outcome = (|| -> anyhow::Result<(TokenValidation, TwitchUserIdentity)> {
                let vault = CredentialVault::new(&config.client_id, &config.bot_login);
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                let (validation, channel) = runtime.block_on(async {
                    let oauth = OAuthClient::new(config.client_id.clone())?;
                    let (token, validation) = oauth.load_validated_token(&vault).await?;
                    anyhow::ensure!(
                        validation.login == config.bot_login,
                        "stored token belongs to '{}', expected '{}'",
                        validation.login,
                        config.bot_login
                    );
                    let channel = oauth.lookup_user(&token, &config.channel_login).await?;
                    Ok::<_, anyhow::Error>((validation, channel))
                })?;
                sender
                    .send(TwitchToolEvent::Progress(format!(
                        "Token and channel validated; joining #{}...",
                        config.channel_login
                    )))
                    .map_err(|_| anyhow::anyhow!("Twitch setup window closed"))?;
                let transport = TwitchTransport::start(config.clone())?;
                wait_for_twitch_connection(&transport, Duration::from_secs(30))?;
                let _ = transport.send(TwitchControl::Disconnect);
                Ok((validation, channel))
            })();
            let event = outcome.map_or_else(
                |error| TwitchToolEvent::Error(format!("{error:#}")),
                |(validation, channel)| TwitchToolEvent::Diagnostic {
                    validation,
                    channel,
                },
            );
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not start Twitch diagnostic worker: {error}");
        state.twitch_auth_events = None;
    }
}

fn start_twitch_game_master_lookup(state: &mut ToolState) {
    let config = state.config.twitch.clone();
    let login = state.game_master_lookup.trim().to_ascii_lowercase();
    state.status = format!("Resolving Twitch user '{login}'...");
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-twitch-user-lookup".to_owned())
        .spawn(move || {
            let outcome = (|| -> anyhow::Result<TwitchUserIdentity> {
                let vault = CredentialVault::new(&config.client_id, &config.bot_login);
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                runtime.block_on(async {
                    let oauth = OAuthClient::new(config.client_id.clone())?;
                    let (token, validation) = oauth.load_validated_token(&vault).await?;
                    anyhow::ensure!(
                        validation.login == config.bot_login,
                        "stored token belongs to '{}', expected '{}'",
                        validation.login,
                        config.bot_login
                    );
                    oauth.lookup_user(&token, &login).await
                })
            })();
            let event = outcome.map_or_else(
                |error| TwitchToolEvent::Error(format!("{error:#}")),
                TwitchToolEvent::GameMasterResolved,
            );
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not start Twitch user lookup worker: {error}");
        state.twitch_auth_events = None;
    }
}

fn start_twitch_reward_capture(state: &mut ToolState) {
    let config = state.config.twitch.clone();
    "Connecting to Twitch; redeem the intended Channel Points reward after the tool confirms the channel join..."
        .clone_into(&mut state.status);
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-twitch-reward-capture".to_owned())
        .spawn(move || {
            let outcome = (|| -> anyhow::Result<String> {
                let transport = TwitchTransport::start(config)?;
                wait_for_twitch_connection(&transport, Duration::from_secs(30))?;
                sender
                    .send(TwitchToolEvent::Progress(
                        "Connected. Redeem the intended Channel Points reward now (three-minute timeout)."
                            .to_owned(),
                    ))
                    .map_err(|_| anyhow::anyhow!("Twitch setup window closed"))?;
                let reward_id = wait_for_twitch_reward(&transport, Duration::from_mins(3))?;
                let _ = transport.send(TwitchControl::Disconnect);
                Ok(reward_id)
            })();
            let event = outcome.map_or_else(
                |error| TwitchToolEvent::Error(format!("{error:#}")),
                TwitchToolEvent::RewardCaptured,
            );
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not start Twitch reward capture worker: {error}");
        state.twitch_auth_events = None;
    }
}

fn wait_for_twitch_connection(
    transport: &TwitchTransport,
    timeout: Duration,
) -> anyhow::Result<()> {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        match transport.try_recv() {
            Some(TwitchEvent::Status(TwitchStatus::Connected)) => return Ok(()),
            Some(TwitchEvent::Status(TwitchStatus::Error(error))) => anyhow::bail!("{error}"),
            Some(TwitchEvent::Status(TwitchStatus::Disconnected)) => {
                anyhow::bail!("Twitch disconnected before joining the configured channel")
            }
            _ => thread::sleep(Duration::from_millis(50)),
        }
    }
    anyhow::bail!("timed out waiting for Twitch to join the configured channel")
}

fn wait_for_twitch_reward(
    transport: &TwitchTransport,
    timeout: Duration,
) -> anyhow::Result<String> {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        match transport.try_recv() {
            Some(TwitchEvent::Chat(envelope)) => {
                if let Some(reward_id) = envelope.custom_reward_id {
                    return Ok(reward_id);
                }
            }
            Some(TwitchEvent::Status(TwitchStatus::Error(error))) => anyhow::bail!("{error}"),
            Some(TwitchEvent::Status(TwitchStatus::Disconnected)) => {
                anyhow::bail!("Twitch disconnected while waiting for a reward redemption")
            }
            _ => thread::sleep(Duration::from_millis(50)),
        }
    }
    anyhow::bail!("timed out waiting for a Channel Points reward redemption")
}

fn start_twitch_clear(state: &mut ToolState) {
    "Removing Twitch token from the OS credential vault...".clone_into(&mut state.status);
    let config = state.config.twitch.clone();
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-twitch-clear".to_owned())
        .spawn(move || {
            let event = CredentialVault::new(&config.client_id, &config.bot_login)
                .clear()
                .map_or_else(
                    |error| TwitchToolEvent::Error(format!("{error:#}")),
                    |()| TwitchToolEvent::Cleared,
                );
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not start Twitch credential cleanup worker: {error}");
        state.twitch_auth_events = None;
    }
}

fn start_twitch_broadcast_clear(state: &mut ToolState) {
    "Removing Twitch broadcaster token from the OS credential vault..."
        .clone_into(&mut state.status);
    let config = state.config.twitch.clone();
    let sender = twitch_event_channel(state);
    let worker = thread::Builder::new()
        .name("stream-town-tools-broadcast-clear".to_owned())
        .spawn(move || {
            let event = CredentialVault::broadcaster(&config.client_id, &config.channel_login)
                .clear()
                .map_or_else(
                    |error| TwitchToolEvent::Error(format!("{error:#}")),
                    |()| TwitchToolEvent::BroadcastCleared,
                );
            let _ = sender.send(event);
        });
    if let Err(error) = worker {
        state.status = format!("Could not remove Twitch broadcaster credentials: {error}");
        state.twitch_auth_events = None;
    }
}

fn save_runtime_config(config: &GameConfig) -> anyhow::Result<std::path::PathBuf> {
    stream_town_game::save_runtime_config(config)
}

fn merge_authoring_config_with_runtime(
    authored: &GameConfig,
    existing_runtime: Option<&GameConfig>,
) -> GameConfig {
    let mut applied = authored.clone();
    if let Some(existing_runtime) = existing_runtime {
        // Twitch identities and broadcast preferences are local operator data.
        // The tools app authors simulation/content settings, so applying those
        // settings must never replace setup entered through the game's Secrets
        // and Streaming screens with the repository baseline.
        applied.twitch = existing_runtime.twitch.clone();
    }
    applied
}

fn save_authoring_runtime_config(config: &GameConfig) -> anyhow::Result<PathBuf> {
    let runtime_path = stream_town_game::runtime_config_path();
    let existing_runtime = runtime_path
        .is_file()
        .then(stream_town_game::load_runtime_config)
        .transpose()
        .with_context(|| {
            format!(
                "could not preserve local Twitch setup from {}",
                runtime_path.display()
            )
        })?;
    let applied = merge_authoring_config_with_runtime(config, existing_runtime.as_ref());
    save_runtime_config(&applied)
}

fn save_and_apply_game_config(
    config: &GameConfig,
    project_path: &str,
) -> anyhow::Result<(PathBuf, PathBuf)> {
    let project = save_game_config(config, project_path)?;
    let runtime = save_authoring_runtime_config(config)?;
    Ok((project, runtime))
}
