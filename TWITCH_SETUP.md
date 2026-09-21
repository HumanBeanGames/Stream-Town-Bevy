# Connect Twitch to Stream Town

This guide connects both Twitch accounts through the Bevy game: the **chat bot** that receives player commands and posts announcements, and the **stream account** that broadcasts your town. No coding is needed. The game captures and streams its own picture and audio; OBS is not required.

**Both Twitch connections are required for normal play.** The town stays paused until Twitch confirms your channel is publicly live. Discord is optional; follow the separate [Discord setup guide](DISCORD_SETUP.md) if you want announcements there.

Open this document from **Main Menu > Connections > Twitch accounts > Open Twitch setup guide**, or **Tools > Twitch**. **Open command guide** opens the [complete chat command reference](TWITCH_COMMANDS.md). These buttons open readable local browser pages with keyboard-accessible links and printable text. Guide edits appear when you reopen them, without rebuilding the game.

## Find what you need

- [Before you start](#before-you-start)
- [1. Register a Twitch application](#1-register-a-twitch-application)
- [2. Enter your account details](#2-enter-your-account-details)
- [3. Connect the chat bot](#3-connect-the-chat-bot)
- [4. Connect the stream account](#4-connect-the-stream-account)
- [5. Choose stream settings](#5-choose-stream-settings)
- [6. Start a town and confirm it is live](#6-start-a-town-and-confirm-it-is-live)
- [Optional bandwidth test](#optional-bandwidth-test)
- [Stopping and reconnecting](#stopping-and-reconnecting)
- [Troubleshooting](#troubleshooting)
- [Using Tools instead](#using-tools-instead)
- [Credentials and permissions](#credentials-and-permissions)
- [Scheduled fresh Beanville launch](#scheduled-fresh-beanville-launch)

## Before you start

Have the game, a browser, and access to both Twitch accounts ready.

| Item | Meaning | Example for this project |
| --- | --- | --- |
| Stream account login | The account whose channel broadcasts the game. | `humanbeangames` |
| Bot login | A separate account that speaks in chat as the game bot. | `humanbeanbot` |
| Client ID | The public identifier for your Twitch application, created in Step 1. | A string of letters and numbers. |

Use account **logins**, without `@`, spaces, or a Twitch URL. Substitute your accounts if different. A private browser window for the bot helps keep it separate from the streamer. Menu paths mean open each item in sequence. No step depends on screenshots or colour recognition. Scroll within the setup page if controls extend below the window; keyboard focus also brings controls into view.

## 1. Register a Twitch application

Skip registration if you already have this application's public Client ID.

1. Sign into the [Twitch Developer Console](https://dev.twitch.tv/console/apps) with the account that will own the application, usually the streamer.
2. Enable two-factor authentication if Twitch asks you to do so.
3. Select **Register Your Application** and give it a unique name, such as **Stream Town - HumanBeanGames**.
4. Enter `http://localhost:3000` as an OAuth redirect URL. The game uses device authorization; you do not need to run a local website there.
5. Choose **Chat Bot** as the category and **Public** as the client type.
6. Complete registration, open the app's management page, and copy **Client ID**.

Do not create or paste a Client Secret. This desktop app uses a public client. [Twitch's registration instructions](https://dev.twitch.tv/docs/authentication/register-app/)

**Check:** you have the Client ID and both correct account logins.

## 2. Enter your account details

1. Launch the game using the **Stream Town** desktop shortcut.
2. Open **Main Menu > Connections**, read the privacy prompt, and choose **Yes**.
3. Select **Twitch accounts**.
4. Paste the public **Client ID**.
5. Set **Bot login** to the bot account, for example `humanbeanbot`.
6. Set **Stream account login** to the broadcaster, for example `humanbeangames`.
7. Select **Save and apply**.

While playing, use **Settings > Connection > Set up connections**. The privacy prompt and Connections screen replace the game's own stream picture with **Sensitive Information Hidden**. Close separate capture or screen-sharing apps before handling credentials; the game cannot hide their recordings.

## 3. Connect the chat bot

1. Select **Authorize bot account**. Twitch's activation page opens in your browser. If it does not open, use the address displayed in the game.
2. Enter the device code shown in the game if the page asks for it.
3. Check that Twitch is signed in as the **bot**, for example **HumanBeanBot**. If it shows the streamer, switch accounts or use a private window.
4. Approve the chat permissions. The bot uses `chat:read` and `chat:edit`.
5. Return to the game and wait for its authorization result.
6. Set **Chat bot** to **Enabled**. Save and apply any changed account fields, then wait for the bot card to report **Connected**.
7. In your channel's chat, use the streamer account to enter `/mod HumanBeanBot`, replacing the name if needed. This gives channel moderator status, not game-master cheats.

**Check:** the connected bot's login matches the intended account. A stored token alone is not an active chat connection.

## 4. Connect the stream account

1. In the same tab, select **Authorize stream account**.
2. Use the activation page and device code as before, this time signed in as the **streamer**, for example **HumanBeanGames**.
3. Approve `channel:read:stream_key`, `user:write:chat`, and `moderator:manage:banned_users`.
4. Return to the game and wait for successful broadcaster authorization for the expected stream account.

This grant lets the game fetch your stream key, send operator-panel messages as the streamer, and perform the panel's timeout/ban actions. Automatic replies and town announcements still use the bot.

**Check:** the bot is connected and the stream account is separately authorized. The wrong account is rejected. If an older grant lacks a permission, authorize that account again; refreshing a token cannot add scopes. [Twitch device-code flow](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#device-code-grant-flow) and [permission reference](https://dev.twitch.tv/docs/authentication/scopes/)

## 5. Choose stream settings

1. In **Connections > Twitch accounts**, enable **Direct stream**.
2. For a real public stream, turn **Bandwidth test** off.
3. Leave **Preferred ingest** empty for Twitch's default, or enter a regional name such as `Sydney`.
4. Leave Connections and open **Settings > Streaming** to choose resolution, frame rate, bitrate, and render mode. Apply your settings.

A modest starting profile is **1280 × 720, 30 FPS, 3000 Kbps video, 160 Kbps audio**. Choose higher settings when your PC and upload can sustain them. Automatic encoder selection tries available hardware before CPU encoding. **Stream-only rendering** uses a separate local operator window with preview and chat; that window is never sent to Twitch. End the current broadcast before changing streaming settings.

Packaged Windows builds include the required media DLLs. Desktop launch scripts also stage the local runtime.

## 6. Start a town and confirm it is live

1. Close Connections. Choose **New Town** or **Load Town**.
2. If the game reports missing Twitch setup, complete Steps 3 and 4 first.
3. Answer **Yes** to the go-live confirmation when ready to broadcast.
4. Wait for world loading and the encoder connection to finish.
5. Wait for Twitch's public-live check. The operator panel then reports **LIVE** and the town begins running.
6. Check the public picture and audio on your Twitch channel from another device or browser. As a viewer, send `!join`, then `!help` in chat.

**Connected chat or an active encoder does not unlock gameplay.** While Twitch is being checked, the town displays a paused notice. Town simulation, construction, citizen actions, and game command execution wait. Connections, encoding, checks, and menus keep working. Verification retries after temporary failures. Reconnection pauses gameplay until public-live status is confirmed again. Commands received while paused wait to execute when play resumes.

Bandwidth tests never pass this gate. Discord setup and delivery do not affect it. The game captures its own audio, including music; microphones, webcams, voice calls, browser audio, and unrelated desktop applications are not included.

## Optional bandwidth test

A bandwidth test sends video to Twitch for inspection without publishing a live channel. It tests transport and encoding while the town remains **paused**.

1. Before starting a session, enable **Bandwidth test** in Connections.
2. Start/load a town and accept the stream confirmation.
3. Open [Twitch Inspector](https://inspector.twitch.tv/) as the streamer and check the session's connection stability and bitrate.
4. Finish with **End Stream**, then disable bandwidth testing.
5. Start the real stream and wait for public-live confirmation to begin gameplay.

## Stopping and reconnecting

- **End Stream** closes the broadcast and gameplay pauses.
- **Go Live** starts another session from the loaded town; play resumes after Twitch confirms it is live.
- **Restart stream** rebuilds the encoder connection and checks Twitch again. It keeps town progress.
- At the main menu, `F1` disconnects bot chat and `F2` reconnects it. These shortcuts do not operate during gameplay. Use Connections to change account setup.
- Normal startup opens the main menu offline. The **Redeploy** desktop shortcut deliberately resumes a saved town and requests go-live automatically.

## Troubleshooting

| What you see | What to check |
| --- | --- |
| Wrong-account error | Use the bot for **Authorize bot account**, and the streamer for **Authorize stream account**. Check both saved logins. |
| Activation page does not open | Open the displayed address manually. Enter the current device code; request a new code if it expires. |
| Missing Client ID / invalid client | Copy the app's **Client ID**, save it, and ensure its client type is **Public**. |
| Login authentication failed | Reauthorize the bot; its grant may be revoked or belong to another app. |
| Channel join times out | Check the stream account login and internet connection, then reconnect the bot. |
| Broadcaster authorization missing | Complete **Authorize stream account**. Authorizing the bot alone does not fix it. |
| Connecting/verifying; town paused | Wait for the public-live check. Check the broadcaster grant, network, and Twitch channel; ensure bandwidth testing is off. Failed verification retries automatically. |
| Bandwidth test / not publicly live | End the test, disable it, and go live again. A paused town is expected during the test. |
| Reconnecting | Gameplay pauses during reconnection and resumes after public-live confirmation. Check the operator status for the connection error. |
| Missing media DLL | Use the desktop launcher or a complete packaged build. Copying only the executable omits required libraries. |
| Operator chat/moderation fails | Reauthorize the stream account with all three permissions in Step 4. |
| Discord disconnected | This does not block Twitch or gameplay. Follow the optional [Discord guide](DISCORD_SETUP.md) to set it up. |

The old Fish God Channel Points reward maps to `!praise`, which is recognized but not implemented. Redeeming it does not complete the event. The command reference lists supported event controls.

## Using Tools instead

**Tools > Twitch** exposes the same account authorizations and public settings. Its **Open Twitch setup guide** and **Open command guide** links open these documents. Use the same Client ID, logins, and permissions described above. Save settings in Tools, authorize both accounts, then launch or reconnect in the game. Save and close one editor before editing the same settings in the other.

## Credentials and permissions

Public settings are saved in `bevy-port/.stream-town/config.ron` for this desktop installation. OAuth tokens live in the OS credential vault, with separate entries for bot and streamer. Stream keys are fetched in memory and are not displayed or saved. Keep secrets out of source files, chat, issue reports, and documentation.

The bot token is validated at startup and periodically, and refreshed automatically when possible. Revoked authorization requires signing in again. Twitch moderator and broadcaster badges grant staff commands. Game-master cheats require an exact numeric Twitch ID in `twitch.game_master_ids`; its default list is empty. The [command guide](TWITCH_COMMANDS.md) explains each permission.

## Scheduled fresh Beanville launch

The requested one-time launch is **Tuesday 22 September 2026 at 8am Adelaide time**. It closes an existing game normally, archives Beanville's save and rotating backups under `bevy-port/.stream-town/town-backups/Beanville-<timestamp>/`, verifies hashes, then starts a **new Beanville** and goes live. Other towns and saved credentials are kept. The new town waits for Twitch's public-live check before progressing.

The task uses the already-built release game and does not compile at 8am. Its launcher is `bevy-port/scripts/restart-beanville.ps1`; `-PlanOnly` previews actions without closing the game, moving saves, or broadcasting. Keep this PC awake and Codex open for the local task to run. [Codex local automation requirements](https://learn.chatgpt.com/docs/automations)

To restore the old town, close the game, preserve any new Beanville save, and copy the archived `Beanville.stbevy` back to `.stream-town/saves/`. Select **Load Town**; avoid the fresh-reset launcher when restoring a backup.
