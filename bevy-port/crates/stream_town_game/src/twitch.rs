//! Twitch OAuth, OS-vault storage, and IRC transport.
//!
//! This module deliberately owns no gameplay state. It emits authenticated chat
//! envelopes for Bevy systems to validate and dispatch on the main thread.

use std::{
    fmt,
    sync::{Arc, Mutex, mpsc},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail};
use keyring::Entry;
use serde::{Deserialize, Serialize};
use stream_town_domain::{StableId, TwitchConfig};
use tokio::sync::mpsc as tokio_mpsc;
use twitch_irc::{
    ClientConfig, SecureTCPTransport, TwitchIRCClient, login::StaticLoginCredentials,
    message::ServerMessage,
};

const DEVICE_ENDPOINT: &str = "https://id.twitch.tv/oauth2/device";
const TOKEN_ENDPOINT: &str = "https://id.twitch.tv/oauth2/token";
const VALIDATE_ENDPOINT: &str = "https://id.twitch.tv/oauth2/validate";
const USERS_ENDPOINT: &str = "https://api.twitch.tv/helix/users";
const VAULT_SERVICE: &str = "stream-town-twitch";
const TOKEN_REFRESH_WINDOW_SECONDS: u64 = 90 * 60;
pub const REQUIRED_SCOPES: [&str; 2] = ["chat:read", "chat:edit"];

#[derive(Clone, Deserialize, Serialize)]
pub struct StoredOAuthToken {
    access_token: String,
    refresh_token: String,
    created_at_unix_seconds: u64,
    expires_at_unix_seconds: u64,
    scopes: Vec<String>,
}

impl fmt::Debug for StoredOAuthToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StoredOAuthToken")
            .field("access_token", &"[redacted]")
            .field("refresh_token", &"[redacted]")
            .field("created_at_unix_seconds", &self.created_at_unix_seconds)
            .field("expires_at_unix_seconds", &self.expires_at_unix_seconds)
            .field("scopes", &self.scopes)
            .finish()
    }
}

impl StoredOAuthToken {
    #[must_use]
    pub fn expires_at_unix_seconds(&self) -> u64 {
        self.expires_at_unix_seconds
    }

    #[must_use]
    pub fn scopes(&self) -> &[String] {
        &self.scopes
    }
}

#[derive(Clone, Deserialize)]
pub struct DeviceAuthorization {
    device_code: String,
    pub expires_in: u64,
    pub interval: u64,
    pub user_code: String,
    pub verification_uri: String,
}

impl fmt::Debug for DeviceAuthorization {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeviceAuthorization")
            .field("device_code", &"[redacted]")
            .field("expires_in", &self.expires_in)
            .field("interval", &self.interval)
            .field("user_code", &self.user_code)
            .field("verification_uri", &self.verification_uri)
            .finish()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct TokenValidation {
    pub client_id: String,
    pub login: String,
    pub scopes: Vec<String>,
    pub expires_in: u64,
    pub user_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct TwitchUserIdentity {
    pub id: String,
    pub login: String,
    pub display_name: String,
}

#[derive(Deserialize)]
struct UsersResponse {
    data: Vec<TwitchUserIdentity>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    expires_in: u64,
    #[serde(default)]
    scope: Vec<String>,
}

#[derive(Deserialize)]
struct OAuthErrorResponse {
    message: String,
}

#[derive(Clone)]
pub struct OAuthClient {
    client_id: String,
    http: reqwest::Client,
}

impl OAuthClient {
    pub fn new(client_id: impl Into<String>) -> Result<Self> {
        let client_id = client_id.into();
        if client_id.trim().is_empty() {
            bail!("Twitch public client ID is empty");
        }
        Ok(Self {
            client_id,
            http: reqwest::Client::builder()
                .user_agent("Stream-Town-Bevy/0.1")
                .build()
                .context("failed to construct Twitch HTTP client")?,
        })
    }

    pub async fn begin_device_authorization(&self) -> Result<DeviceAuthorization> {
        let scopes = REQUIRED_SCOPES.join(" ");
        self.http
            .post(DEVICE_ENDPOINT)
            .form(&[("client_id", self.client_id.as_str()), ("scopes", &scopes)])
            .send()
            .await
            .context("Twitch device authorization request failed")?
            .error_for_status()
            .context("Twitch rejected the device authorization request")?
            .json()
            .await
            .context("Twitch returned an invalid device authorization response")
    }

    pub async fn complete_device_authorization(
        &self,
        authorization: &DeviceAuthorization,
    ) -> Result<StoredOAuthToken> {
        let started = tokio::time::Instant::now();
        let mut interval = Duration::from_secs(authorization.interval.max(1));
        loop {
            if started.elapsed() >= Duration::from_secs(authorization.expires_in) {
                bail!("Twitch device authorization expired");
            }
            tokio::time::sleep(interval).await;
            let scopes = REQUIRED_SCOPES.join(" ");
            let response = self
                .http
                .post(TOKEN_ENDPOINT)
                .form(&[
                    ("client_id", self.client_id.as_str()),
                    ("scopes", scopes.as_str()),
                    ("device_code", authorization.device_code.as_str()),
                    ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ])
                .send()
                .await
                .context("Twitch device token request failed")?;
            if response.status().is_success() {
                let token: TokenResponse = response
                    .json()
                    .await
                    .context("Twitch returned an invalid token response")?;
                return token_from_response(token, None);
            }
            let error: OAuthErrorResponse = response
                .json()
                .await
                .context("Twitch returned an invalid OAuth error response")?;
            match error.message.as_str() {
                "authorization_pending" => {}
                "slow_down" => interval += Duration::from_secs(5),
                message => bail!("Twitch device authorization failed: {message}"),
            }
        }
    }

    pub async fn refresh(&self, token: &StoredOAuthToken) -> Result<StoredOAuthToken> {
        let response: TokenResponse = self
            .http
            .post(TOKEN_ENDPOINT)
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("grant_type", "refresh_token"),
                ("refresh_token", token.refresh_token.as_str()),
            ])
            .send()
            .await
            .context("Twitch refresh request failed")?
            .error_for_status()
            .context("Twitch rejected the refresh token")?
            .json()
            .await
            .context("Twitch returned an invalid refresh response")?;
        token_from_response(response, Some(token.refresh_token.clone()))
    }

    pub async fn validate(&self, token: &StoredOAuthToken) -> Result<TokenValidation> {
        let validation: TokenValidation = self
            .http
            .get(VALIDATE_ENDPOINT)
            .header("Authorization", format!("OAuth {}", token.access_token))
            .send()
            .await
            .context("Twitch token validation request failed")?
            .error_for_status()
            .context("Twitch rejected the stored token")?
            .json()
            .await
            .context("Twitch returned an invalid validation response")?;
        if validation.client_id != self.client_id {
            bail!("stored Twitch token belongs to a different application");
        }
        for required in REQUIRED_SCOPES {
            if !validation.scopes.iter().any(|scope| scope == required) {
                bail!("stored Twitch token is missing scope {required}");
            }
        }
        Ok(validation)
    }

    pub async fn load_validated_token(
        &self,
        vault: &CredentialVault,
    ) -> Result<(StoredOAuthToken, TokenValidation)> {
        let mut token = vault
            .load()?
            .context("Twitch bot is not authorized; use stream_town_tools first")?;
        let validation = self.validate(&token).await;
        if let Ok(validation) = validation
            && validation.expires_in > TOKEN_REFRESH_WINDOW_SECONDS
        {
            return Ok((token, validation));
        }
        token = self.refresh(&token).await?;
        vault.save(&token)?;
        let validation = self.validate(&token).await?;
        Ok((token, validation))
    }

    pub async fn lookup_user(
        &self,
        token: &StoredOAuthToken,
        login: &str,
    ) -> Result<TwitchUserIdentity> {
        let login = login.trim().to_ascii_lowercase();
        if login.is_empty() {
            bail!("Twitch login is empty");
        }
        let response: UsersResponse = self
            .http
            .get(USERS_ENDPOINT)
            .query(&[("login", login.as_str())])
            .header("Client-Id", &self.client_id)
            .bearer_auth(&token.access_token)
            .send()
            .await
            .context("Twitch user lookup failed")?
            .error_for_status()
            .context("Twitch rejected the user lookup")?
            .json()
            .await
            .context("Twitch returned an invalid user lookup response")?;
        response
            .data
            .into_iter()
            .next()
            .with_context(|| format!("Twitch user '{login}' does not exist"))
    }
}

fn token_from_response(
    response: TokenResponse,
    previous_refresh_token: Option<String>,
) -> Result<StoredOAuthToken> {
    let now = unix_seconds()?;
    let refresh_token = response
        .refresh_token
        .or(previous_refresh_token)
        .context("Twitch token response omitted the refresh token")?;
    Ok(StoredOAuthToken {
        access_token: response.access_token,
        refresh_token,
        created_at_unix_seconds: now,
        expires_at_unix_seconds: now.saturating_add(response.expires_in),
        scopes: response.scope,
    })
}

fn unix_seconds() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?
        .as_secs())
}

#[derive(Clone, Debug)]
pub struct CredentialVault {
    username: String,
}

impl CredentialVault {
    #[must_use]
    pub fn new(client_id: &str, bot_login: &str) -> Self {
        Self {
            username: format!("{client_id}:{bot_login}"),
        }
    }

    pub fn load(&self) -> Result<Option<StoredOAuthToken>> {
        let entry = Entry::new(VAULT_SERVICE, &self.username)
            .context("failed to open the operating-system credential vault")?;
        let encoded = match entry.get_password() {
            Ok(encoded) => encoded,
            Err(keyring::Error::NoEntry) => return Ok(None),
            Err(error) => return Err(error).context("failed to read Twitch credentials"),
        };
        serde_json::from_str(&encoded)
            .map(Some)
            .context("stored Twitch credentials are corrupt")
    }

    pub fn save(&self, token: &StoredOAuthToken) -> Result<()> {
        let encoded =
            serde_json::to_string(token).context("failed to encode Twitch credentials")?;
        Entry::new(VAULT_SERVICE, &self.username)
            .context("failed to open the operating-system credential vault")?
            .set_password(&encoded)
            .context("failed to store Twitch credentials")
    }

    pub fn clear(&self) -> Result<()> {
        let entry = Entry::new(VAULT_SERVICE, &self.username)
            .context("failed to open the operating-system credential vault")?;
        match entry.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(error) => Err(error).context("failed to delete Twitch credentials"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TwitchChatEnvelope {
    pub actor_id: StableId,
    pub user_id: String,
    pub login: String,
    pub display_name: String,
    pub message: String,
    pub is_broadcaster: bool,
    pub is_moderator: bool,
    pub is_subscriber: bool,
    pub custom_reward_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TwitchStatus {
    Disabled,
    Authorizing,
    Connecting,
    Connected,
    Reconnecting,
    Disconnected,
    Error(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TwitchEvent {
    Status(TwitchStatus),
    Chat(TwitchChatEnvelope),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TwitchControl {
    SendMessage(String),
    Disconnect,
}

pub struct TwitchTransport {
    events: Arc<Mutex<mpsc::Receiver<TwitchEvent>>>,
    controls: tokio_mpsc::UnboundedSender<TwitchControl>,
}

impl TwitchTransport {
    pub fn start(config: TwitchConfig) -> Result<Self> {
        let (event_sender, event_receiver) = mpsc::channel();
        let (control_sender, control_receiver) = tokio_mpsc::unbounded_channel();
        thread::Builder::new()
            .name("stream-town-twitch".to_owned())
            .spawn(move || {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .thread_name("stream-town-twitch-worker")
                    .build();
                match runtime {
                    Ok(runtime) => {
                        if let Err(error) = runtime.block_on(run_transport(
                            config,
                            event_sender.clone(),
                            control_receiver,
                        )) {
                            let _ = event_sender
                                .send(TwitchEvent::Status(TwitchStatus::Error(error.to_string())));
                        }
                    }
                    Err(error) => {
                        let _ = event_sender.send(TwitchEvent::Status(TwitchStatus::Error(
                            format!("failed to start Twitch runtime: {error}"),
                        )));
                    }
                }
            })
            .context("failed to spawn Twitch transport thread")?;
        Ok(Self {
            events: Arc::new(Mutex::new(event_receiver)),
            controls: control_sender,
        })
    }

    #[must_use]
    pub fn try_recv(&self) -> Option<TwitchEvent> {
        self.events.lock().ok()?.try_recv().ok()
    }

    pub fn send(&self, control: TwitchControl) -> Result<()> {
        self.controls
            .send(control)
            .context("Twitch transport is not running")
    }
}

impl Drop for TwitchTransport {
    fn drop(&mut self) {
        let _ = self.controls.send(TwitchControl::Disconnect);
    }
}

async fn run_transport(
    config: TwitchConfig,
    events: mpsc::Sender<TwitchEvent>,
    mut controls: tokio_mpsc::UnboundedReceiver<TwitchControl>,
) -> Result<()> {
    events.send(TwitchEvent::Status(TwitchStatus::Authorizing))?;
    let oauth = OAuthClient::new(config.client_id.clone())?;
    let vault = CredentialVault::new(&config.client_id, &config.bot_login);
    let (mut token, validation) = oauth.load_validated_token(&vault).await?;
    ensure_bot_identity(&validation, &config.bot_login)?;
    let mut first_connection = true;

    'connection: loop {
        events.send(TwitchEvent::Status(if first_connection {
            TwitchStatus::Connecting
        } else {
            TwitchStatus::Reconnecting
        }))?;
        first_connection = false;
        let credentials =
            StaticLoginCredentials::new(config.bot_login.clone(), Some(token.access_token.clone()));
        let client_config = ClientConfig::new_simple(credentials);
        let (mut incoming, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client_config);
        client
            .join(config.channel_login.clone())
            .context("invalid Twitch channel login")?;
        let mut announced_connection = false;
        let mut validation_timer = tokio::time::interval(Duration::from_hours(1));
        validation_timer.tick().await;

        loop {
            tokio::select! {
                message = incoming.recv() => {
                    let Some(message) = message else {
                        events.send(TwitchEvent::Status(TwitchStatus::Reconnecting))?;
                        tokio::time::sleep(Duration::from_secs(2)).await;
                        let (validated_token, validation) =
                            oauth.load_validated_token(&vault).await?;
                        ensure_bot_identity(&validation, &config.bot_login)?;
                        token = validated_token;
                        continue 'connection;
                    };
                    let joined_target_channel =
                        message_confirms_channel_join(&message, &config.channel_login);
                    if !announced_connection && joined_target_channel {
                        announced_connection = true;
                        events.send(TwitchEvent::Status(TwitchStatus::Connected))?;
                    }
                    match message {
                        ServerMessage::Privmsg(message)
                            if message.channel_login == config.channel_login
                                && (message.message_text.starts_with('!')
                                    || message.source.tags.0.contains_key("custom-reward-id")) =>
                        {
                            events.send(TwitchEvent::Chat(envelope_from_privmsg(message)?))?;
                        }
                        ServerMessage::Reconnect(_) => {
                            announced_connection = false;
                            events.send(TwitchEvent::Status(TwitchStatus::Reconnecting))?;
                        }
                        _ => {}
                    }
                }
                control = controls.recv() => {
                    match control {
                        Some(TwitchControl::SendMessage(message)) => {
                            client
                                .say(config.channel_login.clone(), message)
                                .await
                                .context("failed to send Twitch chat message")?;
                        }
                        Some(TwitchControl::Disconnect) | None => {
                            client.part(config.channel_login.clone());
                            events.send(TwitchEvent::Status(TwitchStatus::Disconnected))?;
                            return Ok(());
                        }
                    }
                }
                _ = validation_timer.tick() => {
                    let (validated_token, validation) = oauth
                        .load_validated_token(&vault)
                        .await
                        .context("Twitch hourly token validation/refresh failed")?;
                    ensure_bot_identity(&validation, &config.bot_login)?;
                    if validated_token.access_token != token.access_token {
                        token = validated_token;
                        client.part(config.channel_login.clone());
                        events.send(TwitchEvent::Status(TwitchStatus::Reconnecting))?;
                        continue 'connection;
                    }
                    token = validated_token;
                }
            }
        }
    }
}

fn ensure_bot_identity(validation: &TokenValidation, expected_login: &str) -> Result<()> {
    if validation.login != expected_login {
        bail!(
            "Twitch token belongs to '{}', expected '{}'",
            validation.login,
            expected_login
        );
    }
    Ok(())
}

fn message_confirms_channel_join(message: &ServerMessage, channel_login: &str) -> bool {
    match message {
        ServerMessage::Join(message) => message.channel_login == channel_login,
        ServerMessage::RoomState(message) => message.channel_login == channel_login,
        _ => false,
    }
}

fn envelope_from_privmsg(
    message: twitch_irc::message::PrivmsgMessage,
) -> Result<TwitchChatEnvelope> {
    let actor_id = StableId::new(format!("twitch:{}", message.sender.id))?;
    let custom_reward_id = message.source.tags.0.get("custom-reward-id").cloned();
    Ok(TwitchChatEnvelope {
        actor_id,
        user_id: message.sender.id,
        login: message.sender.login,
        display_name: message.sender.name,
        message: message.message_text,
        is_broadcaster: message
            .badges
            .iter()
            .any(|badge| badge.name == "broadcaster"),
        is_moderator: message.badges.iter().any(|badge| badge.name == "moderator"),
        is_subscriber: message
            .badges
            .iter()
            .any(|badge| matches!(badge.name.as_str(), "subscriber" | "founder")),
        custom_reward_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_debug_never_exposes_credentials() {
        let token = StoredOAuthToken {
            access_token: "secret-access".to_owned(),
            refresh_token: "secret-refresh".to_owned(),
            created_at_unix_seconds: 1,
            expires_at_unix_seconds: 2,
            scopes: REQUIRED_SCOPES.iter().map(ToString::to_string).collect(),
        };
        let debug = format!("{token:?}");
        assert!(!debug.contains("secret-access"));
        assert!(!debug.contains("secret-refresh"));
        assert!(debug.contains("[redacted]"));
    }

    #[test]
    fn token_response_keeps_rotated_refresh_token() {
        let token = token_from_response(
            TokenResponse {
                access_token: "access".to_owned(),
                refresh_token: Some("rotated".to_owned()),
                expires_in: 60,
                scope: REQUIRED_SCOPES.iter().map(ToString::to_string).collect(),
            },
            Some("old".to_owned()),
        )
        .unwrap();
        assert_eq!(token.refresh_token, "rotated");
        assert!(token.expires_at_unix_seconds > token.created_at_unix_seconds);
    }

    #[test]
    fn channel_point_reward_tag_survives_privmsg_conversion() {
        use twitch_irc::message::{IRCMessage, PrivmsgMessage};

        let raw = "@badge-info=;badges=;color=;custom-reward-id=5a760033-50b5-4e47-911b-d63993d2860c;display-name=Viewer;emotes=;id=message;mod=0;room-id=7;subscriber=0;tmi-sent-ts=1594545155039;user-id=42;user-type= :viewer!viewer@viewer.tmi.twitch.tv PRIVMSG #channel :Praise!";
        let message = PrivmsgMessage::try_from(IRCMessage::parse(raw).unwrap()).unwrap();
        let envelope = envelope_from_privmsg(message).unwrap();
        assert_eq!(
            envelope.custom_reward_id.as_deref(),
            Some("5a760033-50b5-4e47-911b-d63993d2860c")
        );
        assert_eq!(envelope.actor_id.as_str(), "twitch:42");
        assert_eq!(envelope.message, "Praise!");
        assert!(!envelope.is_subscriber);
    }

    #[test]
    fn connected_status_requires_confirmation_for_the_target_channel() {
        use twitch_irc::message::IRCMessage;

        let joined = ServerMessage::try_from(
            IRCMessage::parse(":bot!bot@bot.tmi.twitch.tv JOIN #channel").unwrap(),
        )
        .unwrap();
        assert!(message_confirms_channel_join(&joined, "channel"));
        assert!(!message_confirms_channel_join(&joined, "somewhere_else"));

        let welcome = ServerMessage::try_from(
            IRCMessage::parse(":tmi.twitch.tv 001 bot :Welcome, GLHF!").unwrap(),
        )
        .unwrap();
        assert!(!message_confirms_channel_join(&welcome, "channel"));
    }
}
