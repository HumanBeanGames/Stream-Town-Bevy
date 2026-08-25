# Twitch setup

Stream Town uses three deliberately separate identities:

- **Broadcaster/channel:** `HumanBeanGames`
- **Chat bot:** `HumanBeanBot`
- **Twitch application:** the OAuth registration that lets Stream Town act as the bot

Stream Town authorizes the chat bot for IRC and independently authorizes the
broadcaster for stream-key lookup. The game captures, encodes, and publishes its
own output; OBS is not required. Never authorize the bot grant while signed in
as the broadcaster, or the broadcaster grant while signed in as the bot.

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

## 3. Authorize `HumanBeanBot`

1. From the repository root, run `.\bevy-port\scripts\launch-tools.ps1`, then open the **Twitch** tab. Equivalently, run `cargo run -p stream_town_tools` from `bevy-port`.
2. Paste the app's Client ID.
3. Set the bot account to `HumanBeanBot`.
4. Click **Authorize bot**.
5. On the Twitch activation page, verify that the signed-in account is `HumanBeanBot`. Use a private browser window if the browser keeps selecting `HumanBeanGames`.
6. Approve the two requested scopes: `chat:read` and `chat:edit`.

The setup tool validates the returned token, refuses a token for the wrong account or app, and writes it to the operating-system credential vault (Windows Credential Manager on the initial supported platform). No token file is created. Stream Town validates the token at startup and hourly, refreshes it before the last 90 minutes of its lifetime, securely replaces Twitch's rotated refresh token, and rebuilds the IRC connection with the new access token.

## 4. Prepare the channel

1. In `HumanBeanGames` chat, run `/mod HumanBeanBot`. This is recommended for normal bot rate limits and moderation visibility.
2. In the tools application's **Twitch** tab, set the channel to `humanbeangames` and click **Run end-to-end diagnostic**. Do not continue until it reports the validated bot, resolved channel, and an authenticated IRC channel join.
3. For each trusted operator, enter their login beside **Resolve GM login** and click **Resolve and add ID**. The tool resolves the mutable login to Twitch's stable numeric user ID. Only add people who should receive Unity-compatible game-master/cheat commands.
4. Enable Twitch and click **Save runtime config**. When launched through the supplied script this writes public settings to `bevy-port/.stream-town/config.ron`; credentials remain in Windows Credential Manager.
5. Start or load the world.
6. When Stream Town displays its six-digit broadcaster connection code, send `!connect 123456` from the `HumanBeanGames` account, replacing `123456` with the displayed code.
7. From a separate viewer account, send `!join`, then `!help`. The viewer should receive a stable actor and the bot should return command help in chat.

The connection code is an application-level safety gate. It proves that the broadcaster present in chat is deliberately enabling bot output for this running game session.

Broadcaster and moderator status grants the existing staff command set, but it
does not grant game-master cheats. Those commands require an exact ID from
`twitch.game_master_ids`; the checked-in list is intentionally empty. Local
debug injection bypasses the list in the same way Unity's session bridge did.

## 5. Bind the Fish God Channel Points reward

1. In the `HumanBeanGames` Creator Dashboard, create or select the custom Fish God reward. Enable **Require Viewer to Enter Text** so Twitch emits the chat message carrying its `custom-reward-id` IRC tag.
2. In Stream Town Tools > **Twitch**, click **Capture next reward**.
3. Wait until the tool says the channel is connected, then redeem that exact reward once and enter any text. Do not redeem a different custom reward during this three-minute capture window.
4. Confirm that the UUID appears in **Fish God reward ID**, then click **Save runtime config**.
5. In a running world, redeem it again. It should enter the same deterministic praise path as `!praise`.

The checked-in default retains the reward ID recovered from the Unity project, but the capture workflow is authoritative for the live channel. If Channel Points are unavailable for the channel, clear the reward ID; `!praise` remains usable.

## 6. Authorize direct broadcasting

1. Install/run a packaged Windows build, which already contains the required
   shared FFmpeg/OpenH264 DLLs. Developers building from source must complete
   `bevy-port/third_party/ffmpeg/README.md` first.
2. Open Stream Town Tools > **Twitch** and confirm the Client ID and channel
   login are correct.
3. Click **Authorize broadcaster**. On Twitch's activation page, sign in as
   `HumanBeanGames`, not `HumanBeanBot`, and approve only
   `channel:read:stream_key`.
4. Click **Test broadcast prerequisites**. Do not continue until the tool
   confirms the broadcaster, Twitch ingest list, at least one H.264 encoder, and
   process-scoped Windows audio capture.

This second token has a distinct Windows Credential Manager entry. Stream Town
uses it to fetch the stream key from Twitch Helix when the game launches. The
key is never written to configuration, the repository, logs, diagnostics, or
the runtime console.

## 7. Choose broadcast quality and test bandwidth

In Stream Town Tools > **Twitch**:

1. Enable **Direct broadcast** and **Start when the game launches**.
2. Start with **1280×720, 30 FPS, 3000 Kbps video, 160 Kbps audio**. Automatic
   encoder selection tries hardware paths first, Windows Media Foundation next,
   and the OpenH264 CPU encoder last. Select a specific encoder only when
   diagnosing hardware support.
3. Leave **Preferred ingest** empty for Twitch's default, or enter a region name
   substring such as `Sydney`.
4. Enable **Bandwidth-test mode**, click **Save runtime config**, and launch the
   game. The game will send the full configured bitrate but Twitch will not put
   the channel live. Let it run through the loading screen and gameplay for at
   least five minutes. The external tool's Runtime tab reports the selected
   encoder, ingest, captured/encoded/dropped frames, and audio frames when it
   launches or attaches to the game with the runtime console enabled.
5. Exit the game, turn **Bandwidth-test mode** off, and save again.

Twitch requires H.264 video, AAC audio, constant bitrate, and a two-second
keyframe interval; the game sets those details internally. Higher presets may
be selected as follows, subject to Twitch's current guidance and the available
upload bandwidth:

- 1920×1080 30 FPS: 4500 Kbps
- 1280×720 60 FPS: 4500 Kbps
- 1920×1080 60 FPS: 6000 Kbps

## 8. Go live without OBS

1. Verify **Bandwidth-test mode** is off.
2. Launch `stream_town_game.exe`. Direct publishing begins once the primary
   render target exists, so Twitch receives Stream Town's loading and menu
   presentation as well as gameplay.
3. Check Twitch's Stream Manager/Inspector from another device. To stop the
   broadcast, exit the game; the encoder flushes the stream trailer and closes
   RTMP.

WASAPI capture is scoped to the Stream Town process tree, so both the Bevy sound
engine and Bevy Tidal music are included while unrelated desktop/application
audio is excluded. A microphone, voice call, browser alert, webcam, composited
overlay, BRB scene, or capture-card input is intentionally not included. Those
sources require a future in-game source/mixer feature or an external production
switcher; they are not silently captured from the desktop.

## Connection controls and diagnostics

- `F1`: intentionally disconnect the Twitch bot.
- `F2`: reconnect after credentials or channel settings change.
- Tools diagnostic success: validated bot identity, resolved target channel, and authenticated IRC join.
- Direct-broadcast diagnostic success: validated broadcaster identity, non-empty
  Helix stream-key response, usable ingest list, linked H.264 encoder, and
  process-scoped WASAPI availability.
- In-game HUD state: `Twitch: Connected` (or the broadcaster authorization prompt).
- A wrong-account authorization is rejected before IRC is started.
- If authorization is revoked, the app registration changes, or the refresh token has expired from inactivity, reopen the Bevy tools **Twitch** tab, click **Forget token**, authorize again, and rerun the end-to-end diagnostic.
- `Login authentication failed` normally means the wrong bot authorized the app or the stored grant was revoked.
- A channel-join timeout normally means the channel login is wrong or Twitch IRC is unreachable from the machine.
- No response to `!connect` means it was not sent by the configured channel's broadcaster account or the six-digit code is stale.
- `WaitingForBroadcasterAuthorization` means the broadcaster grant has not been
  completed or was revoked. Reauthorize the broadcaster, not the bot.
- `Reconnecting` means the in-process RTMP worker encountered an ingest/network
  error. It retries with bounded exponential backoff and never buffers an
  unbounded number of render frames.
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
- [FFmpeg LGPL compliance guidance](https://ffmpeg.org/legal.html)
