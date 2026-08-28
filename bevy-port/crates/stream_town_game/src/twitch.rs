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
const STREAMS_ENDPOINT: &str = "https://api.twitch.tv/helix/streams";
const STREAM_KEY_ENDPOINT: &str = "https://api.twitch.tv/helix/streams/key";
const MODERATION_BANS_ENDPOINT: &str = "https://api.twitch.tv/helix/moderation/bans";
const INGESTS_ENDPOINT: &str = "https://ingest.twitch.tv/ingests";
const VAULT_SERVICE: &str = "stream-town-twitch";
const TOKEN_REFRESH_WINDOW_SECONDS: u64 = 90 * 60;
const TWITCH_HTTP_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const TWITCH_HTTP_REQUEST_TIMEOUT: Duration = Duration::from_secs(20);
pub const REQUIRED_SCOPES: [&str; 2] = ["chat:read", "chat:edit"];
pub const BROADCAST_SCOPES: [&str; 2] =
    ["channel:read:stream_key", "moderator:manage:banned_users"];

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
struct StreamKeyResponse {
    data: Vec<StreamKeyData>,
}

#[derive(Deserialize)]
struct StreamsResponse {
    data: Vec<LiveStreamData>,
}

#[derive(Deserialize)]
struct LiveStreamData {
    user_id: String,
    #[serde(rename = "type")]
    stream_type: String,
}

#[derive(Deserialize)]
struct StreamKeyData {
    stream_key: String,
}

#[derive(Clone)]
pub struct TwitchStreamKey(String);

impl fmt::Debug for TwitchStreamKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("TwitchStreamKey([redacted])")
    }
}

impl TwitchStreamKey {
    #[must_use]
    pub(crate) fn expose(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TwitchIngest {
    pub name: String,
    pub url_template: String,
    pub priority: u32,
    pub availability: f32,
    #[serde(rename = "default")]
    pub is_default: bool,
}

#[derive(Deserialize)]
struct IngestsResponse {
    ingests: Vec<TwitchIngest>,
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

#[derive(Serialize)]
struct ModerationRequest<'a> {
    data: ModerationRequestData<'a>,
}

#[derive(Serialize)]
struct ModerationRequestData<'a> {
    user_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u32>,
    reason: &'a str,
}

#[derive(Clone)]
pub struct OAuthClient {
    client_id: String,
    required_scopes: Vec<String>,
    http: reqwest::Client,
}

impl OAuthClient {
    pub fn new(client_id: impl Into<String>) -> Result<Self> {
        Self::new_with_scopes(client_id, REQUIRED_SCOPES)
    }

    pub fn broadcaster(client_id: impl Into<String>) -> Result<Self> {
        Self::new_with_scopes(client_id, BROADCAST_SCOPES)
    }

    pub fn new_with_scopes<I, S>(client_id: impl Into<String>, scopes: I) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let client_id = client_id.into();
        if client_id.trim().is_empty() {
            bail!("Twitch public client ID is empty");
        }
        let required_scopes = scopes.into_iter().map(Into::into).collect::<Vec<_>>();
        if required_scopes.is_empty() {
            bail!("Twitch OAuth requires at least one scope");
        }
        Ok(Self {
            client_id,
            required_scopes,
            http: reqwest::Client::builder()
                .user_agent("Stream-Town-Bevy/0.1")
                .connect_timeout(TWITCH_HTTP_CONNECT_TIMEOUT)
                .timeout(TWITCH_HTTP_REQUEST_TIMEOUT)
                .build()
                .context("failed to construct Twitch HTTP client")?,
        })
    }

    pub async fn begin_device_authorization(&self) -> Result<DeviceAuthorization> {
        let scopes = self.required_scopes.join(" ");
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
            let scopes = self.required_scopes.join(" ");
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
        for required in &self.required_scopes {
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
            .context("Twitch account is not authorized; open Main Menu > Secrets")?;
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

    pub async fn stream_key(
        &self,
        token: &StoredOAuthToken,
        broadcaster_id: &str,
    ) -> Result<TwitchStreamKey> {
        if broadcaster_id.is_empty() || !broadcaster_id.bytes().all(|byte| byte.is_ascii_digit()) {
            bail!("Twitch broadcaster ID is invalid");
        }
        let response: StreamKeyResponse = self
            .http
            .get(STREAM_KEY_ENDPOINT)
            .query(&[("broadcaster_id", broadcaster_id)])
            .header("Client-Id", &self.client_id)
            .bearer_auth(&token.access_token)
            .send()
            .await
            .context("Twitch stream-key lookup failed")?
            .error_for_status()
            .context("Twitch rejected the stream-key lookup")?
            .json()
            .await
            .context("Twitch returned an invalid stream-key response")?;
        let key = response
            .data
            .into_iter()
            .next()
            .context("Twitch returned no stream key for the authorized broadcaster")?
            .stream_key;
        if key.trim().is_empty() {
            bail!("Twitch returned an empty stream key");
        }
        Ok(TwitchStreamKey(key))
    }

    pub async fn is_stream_live(
        &self,
        token: &StoredOAuthToken,
        broadcaster_id: &str,
    ) -> Result<bool> {
        if broadcaster_id.is_empty() || !broadcaster_id.bytes().all(|byte| byte.is_ascii_digit()) {
            bail!("Twitch broadcaster ID is invalid");
        }
        let response: StreamsResponse = self
            .http
            .get(STREAMS_ENDPOINT)
            .query(&[("user_id", broadcaster_id)])
            .header("Client-Id", &self.client_id)
            .bearer_auth(&token.access_token)
            .send()
            .await
            .context("Twitch live-status lookup failed")?
            .error_for_status()
            .context("Twitch rejected the live-status lookup")?
            .json()
            .await
            .context("Twitch returned an invalid live-status response")?;
        Ok(response_contains_live_stream(&response, broadcaster_id))
    }

    pub async fn ingests(&self) -> Result<Vec<TwitchIngest>> {
        let mut ingests = self
            .http
            .get(INGESTS_ENDPOINT)
            .send()
            .await
            .context("Twitch ingest lookup failed")?
            .error_for_status()
            .context("Twitch rejected the ingest lookup")?
            .json::<IngestsResponse>()
            .await
            .context("Twitch returned an invalid ingest response")?
            .ingests;
        ingests.retain(|ingest| ingest.url_template.contains("{stream_key}"));
        ingests.sort_by_key(|ingest| (!ingest.is_default, ingest.priority));
        if ingests.is_empty() {
            bail!("Twitch returned no usable RTMP ingest endpoints");
        }
        Ok(ingests)
    }

    async fn moderate_user(
        &self,
        token: &StoredOAuthToken,
        broadcaster_id: &str,
        moderator_id: &str,
        user_id: &str,
        duration_seconds: Option<u32>,
        reason: &str,
    ) -> Result<()> {
        if broadcaster_id.is_empty() || moderator_id.is_empty() || user_id.is_empty() {
            bail!("Twitch moderation requires broadcaster, moderator, and target user IDs");
        }
        self.http
            .post(MODERATION_BANS_ENDPOINT)
            .query(&[
                ("broadcaster_id", broadcaster_id),
                ("moderator_id", moderator_id),
            ])
            .header("Client-Id", &self.client_id)
            .bearer_auth(&token.access_token)
            .json(&ModerationRequest {
                data: ModerationRequestData {
                    user_id,
                    duration: duration_seconds,
                    reason,
                },
            })
            .send()
            .await
            .context("Twitch moderation request failed")?
            .error_for_status()
            .context("Twitch rejected the moderation request")?;
        Ok(())
    }
}

fn response_contains_live_stream(response: &StreamsResponse, broadcaster_id: &str) -> bool {
    response
        .data
        .iter()
        .any(|stream| stream.user_id == broadcaster_id && stream.stream_type == "live")
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

    #[must_use]
    pub fn broadcaster(client_id: &str, channel_login: &str) -> Self {
        Self {
            username: format!("broadcast:{client_id}:{channel_login}"),
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
pub enum TwitchModerationStatus {
    Disabled,
    Authorizing,
    Ready,
    Error(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TwitchEvent {
    Status(TwitchStatus),
    ModerationStatus(TwitchModerationStatus),
    Chat(TwitchChatEnvelope),
    Notice(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TwitchControl {
    SendMessage(String),
    Timeout {
        user_id: String,
        duration_seconds: u32,
        reason: String,
    },
    Ban {
        user_id: String,
        reason: String,
    },
    ReloadModeration,
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
    ensure_oauth_identity(&validation, &config.bot_login)?;
    let (moderation_sender, mut moderation_receiver) = tokio_mpsc::unbounded_channel();
    let mut moderation_generation = 1;
    let mut moderation_session: Option<ModerationSession> = None;
    request_moderation_session(&config, moderation_generation, &events, &moderation_sender)?;
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
                        ensure_oauth_identity(&validation, &config.bot_login)?;
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
                            if message.channel_login == config.channel_login =>
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
                        Some(TwitchControl::Timeout { user_id, duration_seconds, reason }) => {
                            let duration_seconds = duration_seconds.clamp(1, 1_209_600);
                            let notice = match moderation_session.as_ref() {
                                Some(session) => match session.oauth.moderate_user(
                                    &session.token,
                                    &session.broadcaster_id,
                                    &session.moderator_id,
                                    &user_id,
                                    Some(duration_seconds),
                                    &reason,
                                )
                                .await
                                {
                                    Ok(()) => format!("Timed out Twitch user {user_id} for {duration_seconds} seconds"),
                                    Err(error) => format!("Timeout failed for Twitch user {user_id}: {error:#}"),
                                },
                                None => "Timeout unavailable: authorize the broadcaster account for moderation in Main Menu > Secrets".to_owned(),
                            };
                            events.send(TwitchEvent::Notice(notice))?;
                        }
                        Some(TwitchControl::Ban { user_id, reason }) => {
                            let notice = match moderation_session.as_ref() {
                                Some(session) => match session.oauth.moderate_user(
                                    &session.token,
                                    &session.broadcaster_id,
                                    &session.moderator_id,
                                    &user_id,
                                    None,
                                    &reason,
                                )
                                .await
                                {
                                    Ok(()) => format!("Banned Twitch user {user_id}"),
                                    Err(error) => format!("Ban failed for Twitch user {user_id}: {error:#}"),
                                },
                                None => "Ban unavailable: authorize the broadcaster account for moderation in Main Menu > Secrets".to_owned(),
                            };
                            events.send(TwitchEvent::Notice(notice))?;
                        }
                        Some(TwitchControl::ReloadModeration) => {
                            moderation_generation = moderation_generation.saturating_add(1);
                            request_moderation_session(
                                &config,
                                moderation_generation,
                                &events,
                                &moderation_sender,
                            )?;
                        }
                        Some(TwitchControl::Disconnect) | None => {
                            client.part(config.channel_login.clone());
                            events.send(TwitchEvent::Status(TwitchStatus::Disconnected))?;
                            events.send(TwitchEvent::ModerationStatus(TwitchModerationStatus::Disabled))?;
                            return Ok(());
                        }
                    }
                }
                Some(result) = moderation_receiver.recv() => {
                    if result.generation != moderation_generation {
                        continue;
                    }
                    match result.session {
                        Ok(session) => {
                            moderation_session = Some(session);
                            events.send(TwitchEvent::ModerationStatus(TwitchModerationStatus::Ready))?;
                        }
                        Err(error) => {
                            moderation_session = None;
                            events.send(TwitchEvent::ModerationStatus(
                                TwitchModerationStatus::Error(error),
                            ))?;
                        }
                    }
                }
                _ = validation_timer.tick() => {
                    let (validated_token, validation) = oauth
                        .load_validated_token(&vault)
                        .await
                        .context("Twitch hourly token validation/refresh failed")?;
                    ensure_oauth_identity(&validation, &config.bot_login)?;
                    if validated_token.access_token != token.access_token {
                        token = validated_token;
                        client.part(config.channel_login.clone());
                        events.send(TwitchEvent::Status(TwitchStatus::Reconnecting))?;
                        continue 'connection;
                    }
                    token = validated_token;
                    moderation_generation = moderation_generation.saturating_add(1);
                    request_moderation_session(
                        &config,
                        moderation_generation,
                        &events,
                        &moderation_sender,
                    )?;
                }
            }
        }
    }
}

struct ModerationSession {
    oauth: OAuthClient,
    token: StoredOAuthToken,
    broadcaster_id: String,
    moderator_id: String,
}

struct ModerationSessionResult {
    generation: u64,
    session: std::result::Result<ModerationSession, String>,
}

fn request_moderation_session(
    config: &TwitchConfig,
    generation: u64,
    events: &mpsc::Sender<TwitchEvent>,
    results: &tokio_mpsc::UnboundedSender<ModerationSessionResult>,
) -> Result<()> {
    events.send(TwitchEvent::ModerationStatus(
        TwitchModerationStatus::Authorizing,
    ))?;
    let client_id = config.client_id.clone();
    let channel_login = config.channel_login.clone();
    let results = results.clone();
    tokio::spawn(async move {
        let session = load_moderation_session(client_id, channel_login)
            .await
            .map_err(|error| format!("{error:#}"));
        let _ = results.send(ModerationSessionResult {
            generation,
            session,
        });
    });
    Ok(())
}

async fn load_moderation_session(
    client_id: String,
    channel_login: String,
) -> Result<ModerationSession> {
    let oauth = OAuthClient::broadcaster(client_id.clone())?;
    let vault = CredentialVault::broadcaster(&client_id, &channel_login);
    let (token, validation) = oauth
        .load_validated_token(&vault)
        .await
        .context("Twitch broadcaster account is not authorized for moderation")?;
    ensure_oauth_identity(&validation, &channel_login)
        .context("Twitch broadcaster moderation account does not match the configured channel")?;
    Ok(ModerationSession {
        oauth,
        token,
        broadcaster_id: validation.user_id.clone(),
        moderator_id: validation.user_id,
    })
}

fn ensure_oauth_identity(validation: &TokenValidation, expected_login: &str) -> Result<()> {
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
        let stream_key = TwitchStreamKey("live_secret_stream_key".to_owned());
        assert!(!format!("{stream_key:?}").contains("live_secret_stream_key"));
    }

    #[test]
    fn bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate() {
        let client = OAuthClient::broadcaster("public-client-id").unwrap();
        assert_eq!(
            client.required_scopes,
            BROADCAST_SCOPES.map(ToString::to_string)
        );
        assert_eq!(REQUIRED_SCOPES, ["chat:read", "chat:edit"]);
        assert!(BROADCAST_SCOPES.contains(&"channel:read:stream_key"));
        assert!(BROADCAST_SCOPES.contains(&"moderator:manage:banned_users"));
        assert!(!REQUIRED_SCOPES.contains(&"moderator:manage:banned_users"));
    }

    #[test]
    fn bot_and_broadcaster_tokens_use_distinct_vault_entries() {
        let bot = CredentialVault::new("public-client-id", "humanbeanbot");
        let broadcaster = CredentialVault::broadcaster("public-client-id", "humanbeangames");
        assert_ne!(bot.username, broadcaster.username);
        assert!(!bot.username.contains("broadcast:"));
        assert!(broadcaster.username.starts_with("broadcast:"));
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

    #[test]
    fn live_status_requires_the_requested_broadcaster_and_live_type() {
        let response: StreamsResponse = serde_json::from_str(
            r#"{"data":[{"user_id":"42","type":"live"},{"user_id":"7","type":"rerun"}]}"#,
        )
        .unwrap();
        assert!(response_contains_live_stream(&response, "42"));
        assert!(!response_contains_live_stream(&response, "7"));
        assert!(!response_contains_live_stream(&response, "99"));
    }
}
