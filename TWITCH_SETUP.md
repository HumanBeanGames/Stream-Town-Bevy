# Twitch setup

Stream Town uses three deliberately separate identities:

- **Broadcaster/channel:** `HumanBeanGames`
- **Chat bot:** `HumanBeanBot`
- **Twitch application:** the OAuth registration that lets Stream Town act as the bot

Stream Town authorizes the chat bot for IRC and independently authorizes the
broadcaster for stream-key lookup and channel moderation. The bot sends and
receives chat, but operator-panel timeouts and bans are made through the
streamer account and appear as streamer moderation actions. The game captures,
encodes, and publishes its own output; OBS is not required. Never authorize the
bot grant while signed in as the broadcaster, or the broadcaster grant while
signed in as the bot.

The shipping setup path is **Main Menu > Secrets**. Opening it requires an
explicit Yes/No privacy confirmation. From the instant the confirmation appears
until the Secrets screen closes, Stream Town's own Twitch video output is
replaced with opaque black frames labelled **Sensitive Information Hidden**.
Stream Town cannot stop OBS, Streamlabs,
screen sharing, a capture card, or another third-party recorder, so close all of
those applications before choosing Yes. The public Client ID and account logins
are saved to `.stream-town/config.ron`; OAuth access and refresh tokens stay in
the operating-system credential vault. The stream key is fetched only in memory
and is never displayed or saved.

## 1. Secure the old credentials

The former `Assets/StreamingAssets/twitch_secrets.json` contained credential material and was tracked in Git. It has been removed, but removal does not erase older commits.

Before using this repository publicly:

1. In the Twitch Developer Console, replace/revoke the old application secret.
2. While signed in to the old bot account, disconnect the old application in Twitch **Settings > Connections** so its user tokens are revoked.
3. Scrub `Assets/StreamingAssets/twitch_secrets.json` from Git history before publishing or mirroring the repository.

Do not add Twitch tokens, refresh tokens, client secrets, or OBS stream keys anywhere under `Assets`.

## 2. Register the Twitch application

1. Sign in to the [Twitch Developer Console](https://dev.twitch.tv/console/apps) as the account that should own the app. `HumanBeanGames` is the sensible owner.
2. Enable two-factor authentication; Twitch requires it for developer application registration.
3. Register a uniquely named application, for example `Stream Town Reloaded - HumanBeanGames`.
4. Use `http://localhost:3000` as the OAuth redirect URL. The device flow does not redirect there, but Twitch requires a registered URL.
5. Choose **Chat Bot** as the category.
6. Set the client type to **Public**. Stream Town runs on an end user's PC and must not embed a client secret.
7. Copy the **Client ID**. A Client ID is public; a Client Secret is not needed and must not be pasted into Stream Town.

## 3. Configure and authorize `HumanBeanBot`

1. Close OBS and every other screen-capture or streaming application.
2. Launch Stream Town and choose **Secrets** on the main menu.
3. Read the confirmation. Choose **No** to return safely, or **Yes** to enter the
   protected screen and black out Stream Town's internal stream.
4. Paste the application's public **Client ID** and set **Bot login** to
   `humanbeanbot`.
5. Set **Stream account login** to `humanbeangames`. Keeping both names visible
   makes it harder to authorize the wrong identity.
6. Choose **Save and apply**, then **Authorize bot account**. Stream Town opens
   Twitch's activation page in the default browser as soon as Twitch returns the
   device code. If Windows blocks that launch, the same URL remains visible in
   the Secrets screen as a manual fallback.
7. Enter the device code displayed by Stream Town into the opened page.
8. Verify that the signed-in account is `HumanBeanBot`. Use a private browser
   window if the browser keeps selecting `HumanBeanGames`.
9. Approve only `chat:read` and `chat:edit`. Stream Town rejects the grant if
   Twitch returns a different account, Client ID, or scope set.
10. Turn **Chat bot** to **Enabled**. The connection restarts immediately; a game
    restart is not required.

The Secrets screen validates the returned token, refuses a token for the wrong
account or app, and writes it to the operating-system credential vault (Windows
Credential Manager on the initial supported platform). No token file is created.
Stream Town validates the token at startup and hourly, refreshes it before the
last 90 minutes of its lifetime, securely replaces Twitch's rotated refresh
token, and rebuilds the IRC connection with the new access token. The tools app
remains available as a diagnostic and advanced-configuration alternative.

## 4. Prepare the channel

1. In `HumanBeanGames` chat, run `/mod HumanBeanBot`. This is recommended for normal bot rate limits and moderation visibility.
2. The main-menu Secrets screen has already saved the channel as
   `humanbeangames`.
3. **Chat bot** should already read **Enabled** in Main Menu > Secrets. When
   launched from `bevy-port`, its public settings are in
   `bevy-port/.stream-town/config.ron`; credentials remain in Windows Credential
   Manager.
4. Wait for the bot card to report that it connected automatically. No chat-side
   connection command is required.
5. Start or load the world. Stream Town blocks this action and points back to
   Secrets until the bot is connected and both account grants are stored.
6. From a separate viewer account, send `!join`, then `!help`. The viewer should receive a stable actor and the bot should return command help in chat.

Broadcaster and moderator status grants the existing staff command set, but it
does not grant game-master cheats. Those commands require an exact ID from
`twitch.game_master_ids`; the checked-in list is intentionally empty. Local
debug injection bypasses the list in the same way Unity's session bridge did.

## 5. Bind the Fish God Channel Points reward

The checked-in configuration retains the reward ID recovered from the Unity
project. In the `HumanBeanGames` Creator Dashboard, use that reward with
**Require Viewer to Enter Text** enabled so Twitch emits the chat message and
its `custom-reward-id` IRC tag. Redeeming it should enter the same deterministic
praise path as `!praise`. If the production reward is replaced, update
`twitch.fish_god_reward_id` in the authoritative or local `config.ron`; no OAuth
credential is stored there.

## 6. Authorize direct broadcasting

1. Install/run a packaged Windows build, which already contains the required
   shared FFmpeg/OpenH264 DLLs. Developers building from source must complete
   `bevy-port/third_party/ffmpeg/README.md` first.
2. Close every third-party capture application, then open **Main Menu > Secrets**
   and choose **Yes**. Confirm that the public Client ID and stream account login
   are correct.
3. Click **Authorize stream account**. Stream Town automatically opens Twitch's
   activation page; enter the device code, sign in as `HumanBeanGames` (not
   `HumanBeanBot`), and approve `channel:read:stream_key` plus
   `moderator:manage:banned_users`. Stream Town
   rejects a grant for a different account. The broadcaster status changes to
   **Broadcaster authorized** when the validated token is in Windows Credential
   Manager.
4. If the streamer was authorized by an older build, authorize it again once;
   refreshing an old token cannot add the moderation scope.

This second token has a distinct Windows Credential Manager entry. Stream Town
uses it to fetch the stream key from Twitch Helix only after an explicit Go Live
or Restart stream action, and to submit operator-panel timeout/ban requests. The
key is never written to configuration, the repository, logs, diagnostics, or
the runtime console.

## 7. Choose broadcast quality and test bandwidth

Start in Main Menu > **Secrets**:

1. Set **Direct stream** to **Enabled**. Keep **Bandwidth test** enabled for the
   first test, then save the settings. Choose New Town or Load Town and answer
   **Yes** to the go-live confirmation. Authorization and ingest selection begin
   during loading, but this preparation does not open RTMP or start the media
   clocks. The encoder starts only after every loading/GPU-readiness leaf has
   completed, the loading cover has retired, and gameplay has unpaused.
2. Exit the Secrets screen and open **Settings > Streaming**. Start with
   **1280×720, 30 FPS, 3000 Kbps video, 160 Kbps audio**, with **Stream-only
   rendering** enabled. Streaming settings are
   read-only while a session is connecting, live, reconnecting, or stopping;
   use the operator panel's **End Stream** control first. Automatic encoder
   selection prefers AMD AMF, then tries other available hardware paths and
   forced-hardware Windows Media Foundation before the OpenH264 CPU fallback.
   Select a specific encoder only when diagnosing hardware support. The local
   operator telemetry reports the selected backend and rejected fallback
   candidates.
3. Leave **Preferred ingest** empty for Twitch's default, or enter a region name
   substring such as `Sydney`.
4. With **Bandwidth test** enabled, launch the game. It will send
   the full configured bitrate but Twitch will not put
   the channel live. Open [Twitch Inspector](https://inspector.twitch.tv/) while
   signed in as the broadcaster and confirm that the test session is stable.
   Let gameplay run for at least five minutes. The local operator dashboard
   reports rolling captured/output FPS, selected encoder/ingest,
   capture replacements, cadence skips, actual audio/video rejection counts,
   queue depth, and encode latency. Stream-only mode
   hides the original game window and opens a lightweight 1100×680 operator
   window with a low-resolution preview, Twitch chat, moderation, and local
   settings. The operator surface is never sent to Twitch.
5. End the test with **End Stream** in the operator panel. Relaunch to the Main
   Menu, open **Secrets**, accept the warning, and turn **Bandwidth test** off.
   Start/load a town, answer **Yes**, and wait for the operator panel to show
   **LIVE** after loading completes.

Twitch requires H.264 video, AAC audio, constant bitrate, and a two-second
keyframe interval; the game sets those details internally. The AMD AMF path
also enables HRD enforcement and filler packets so low-motion or static scenes
remain at the configured transport bitrate instead of collapsing to a nominal
CBR stream with almost no data. It uses the quality preset with variance-based
adaptive quantization and disables AMF preanalysis so static, high-frequency
terrain detail is not periodically reclassified and softened between IDR
frames. Higher presets may be selected as follows,
subject to Twitch's current guidance and the available upload bandwidth:

- 1920×1080 30 FPS: 4500 Kbps
- 1280×720 60 FPS: 4500 Kbps
- 1920×1080 60 FPS: 6000 Kbps

`Output FPS` is the constant encoder cadence; `captured FPS` is the number of
distinct game renders delivered to it. The validated development machine held
59.9–60.1 output FPS with zero steady-state A/V drops at 1080p60, while the
maximum visual profile supplied about 50–52 distinct frames per second. Prefer
720p60 when motion cadence matters most, or 1080p30 when resolution matters
most. Repeated latest frames preserve timestamps and A/V sync during isolated
render stalls without allowing latency to accumulate.

## 8. Go live without OBS

1. Verify **Bandwidth-test mode** is off.
2. Launch `stream_town_game.exe`; it always starts offline. Choose New Town or
   Load Town. If setup is incomplete, the game directs you to Secrets. Otherwise,
   answer **Yes** to the go-live confirmation. No media is sent until the final
   loading cover has retired. The local operator panel appears with gameplay and
   changes its toggle from the starting state to **LIVE · END STREAM**.
3. Check Twitch's Stream Manager/Inspector from another device. To stop the
   broadcast, click **LIVE · END STREAM** in the operator panel. The encoder
   flushes the stream trailer, restores the ordinary game window in stream-only
   mode, and closes RTMP without exiting the game. The same toggle can start a
   new session from the already-loaded town.

WASAPI capture is scoped to the Stream Town process tree, so both the Bevy sound
engine and Bevy Tidal music are included while unrelated desktop/application
audio is excluded. A microphone, voice call, browser alert, webcam, composited
overlay, BRB scene, or capture-card input is intentionally not included. Those
sources require a future in-game source/mixer feature or an external production
switcher; they are not silently captured from the desktop.

## Connection controls and diagnostics

- `F1`: intentionally disconnect the Twitch bot.
- `F2`: reconnect after credentials or channel settings change.
- Connection success: the Secrets bot card reports the validated bot and
  authenticated IRC join; the stream card reports the separately authorized
  broadcaster.
- Main-menu status success: the bot card says **Connected** and New/Load Town
  opens the explicit go-live confirmation. After Yes, the operator panel appears
  only with gameplay and changes to **LIVE · END STREAM** after gameplay
  readiness. Encoded video and audio frame counts then increase.
- **Restart stream** saves the visible public settings, stops the current
  in-process encoder if one exists, revalidates the broadcaster grant, fetches a
  fresh stream key and ingest list, and starts a new encoder connection.
- In-game HUD state: `Twitch: Connected`; the local-only operator toggle reports
  **NOT LIVE · GO LIVE**, a transitional starting state, or **LIVE · END STREAM**.
  In stream-only mode the HUD is sent to Twitch while the operator sees the
  separate diagnostics/preview window.
- A wrong-account authorization is rejected before IRC is started.
- If authorization is revoked, the app registration changes, the streamer token
  lacks the moderation scope, or a refresh token expires, reopen Main Menu >
  Secrets and authorize the affected account again.
- `Login authentication failed` normally means the wrong bot authorized the app or the stored grant was revoked.
- A channel-join timeout normally means the channel login is wrong or Twitch IRC is unreachable from the machine.
- `WaitingForBroadcasterAuthorization` means the broadcaster grant has not been
  completed or was revoked. Reauthorize the broadcaster, not the bot.
- `Reconnecting` means the in-process RTMP worker encountered an ingest/network
  error. It retries with bounded exponential backoff and never buffers an
  unbounded number of render frames.
- `VerifyingTwitch` means Twitch has accepted at least one encoded video packet
  and the app is waiting for the channel to appear in Twitch's public live API.
  It is not used as a substitute for a real public-live confirmation.
- Missing `avcodec-62.dll` (or another FFmpeg/OpenH264 DLL) means a development
  build was launched without the pinned vcpkg runtime on `PATH`; packaged builds
  put the replaceable DLLs beside the executable.

Official references:

- [Register a Twitch app](https://dev.twitch.tv/docs/authentication/register-app)
- [Twitch device-code OAuth](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#device-code-grant-flow)
- [Twitch IRC authentication and scopes](https://dev.twitch.tv/docs/chat/irc/#authenticating-with-the-twitch-irc-server)
- [Twitch token validation](https://dev.twitch.tv/docs/authentication/validate-tokens/)
- [Twitch video broadcast requirements](https://dev.twitch.tv/docs/video-broadcast/)
- [Twitch Get Stream Key API](https://dev.twitch.tv/docs/api/reference#get-stream-key)
- [Twitch Ban User API](https://dev.twitch.tv/docs/api/reference/#ban-user)
- [FFmpeg LGPL compliance guidance](https://ffmpeg.org/legal.html)
