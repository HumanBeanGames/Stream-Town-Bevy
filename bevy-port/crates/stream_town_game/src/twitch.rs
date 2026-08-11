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
const VAULT_SERVICE: &str = "stream-town-twitch";
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
    let mut token = vault
        .load()?
        .context("Twitch bot is not authorized; use stream_town_tools first")?;
    let validation = if let Ok(validation) = oauth.validate(&token).await {
        validation
    } else {
        token = oauth.refresh(&token).await?;
        vault.save(&token)?;
        oauth.validate(&token).await?
    };
    if validation.login != config.bot_login {
        bail!(
            "Twitch token belongs to '{}', expected '{}'",
            validation.login,
            config.bot_login
        );
    }

    let credentials =
        StaticLoginCredentials::new(config.bot_login.clone(), Some(token.access_token.clone()));
    let client_config = ClientConfig::new_simple(credentials);
    let (mut incoming, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client_config);
    events.send(TwitchEvent::Status(TwitchStatus::Connecting))?;
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
                    events.send(TwitchEvent::Status(TwitchStatus::Disconnected))?;
                    return Ok(());
                };
                if !announced_connection {
                    announced_connection = true;
                    events.send(TwitchEvent::Status(TwitchStatus::Connected))?;
                }
                match message {
                    ServerMessage::Privmsg(message)
                        if message.channel_login == config.channel_login
                            && message.message_text.starts_with('!') =>
                    {
                        events.send(TwitchEvent::Chat(envelope_from_privmsg(message)?))?;
                    }
                    ServerMessage::Reconnect(_) => {
                        events.send(TwitchEvent::Status(TwitchStatus::Reconnecting))?;
                    }
                    _ => {}
                }
            }
            control = controls.recv() => {
                match control {
                    Some(TwitchControl::SendMessage(message)) => {
                        client
                            .privmsg(config.channel_login.clone(), message)
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
                if let Err(error) = oauth.validate(&token).await {
                    events.send(TwitchEvent::Status(TwitchStatus::Error(format!(
                        "Twitch hourly token validation failed: {error}"
                    ))))?;
                    return Ok(());
                }
            }
        }
    }
}

fn envelope_from_privmsg(
    message: twitch_irc::message::PrivmsgMessage,
) -> Result<TwitchChatEnvelope> {
    let actor_id = StableId::new(format!("twitch:{}", message.sender.id))?;
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
}
