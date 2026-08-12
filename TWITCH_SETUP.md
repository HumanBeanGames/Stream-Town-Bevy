# Twitch setup

Stream Town uses three deliberately separate identities:

- **Broadcaster/channel:** `HumanBeanGames`
- **Chat bot:** `HumanBeanBot`
- **Twitch application:** the OAuth registration that lets Stream Town act as the bot

OBS signs in as the broadcaster. Stream Town authorizes and signs in as the bot. Never give the bot token to OBS and never authorize the Stream Town bot while signed in as the broadcaster.

## 1. Secure the old credentials

The former `Assets/StreamingAssets/twitch_secrets.json` contained credential material and was tracked in Git. It has been removed, but removal does not erase older commits.

Before using this repository publicly:

1. In the Twitch Developer Console, replace/revoke the old application secret.
2. While signed in to the old bot account, disconnect the old application in Twitch **Settings > Connections** so its user tokens are revoked.
3. Scrub `Assets/StreamingAssets/twitch_secrets.json` from Git history before publishing or mirroring the repository.

Do not add Twitch tokens, refresh tokens, client secrets, or OBS stream keys anywhere under `Assets`.

## 2. Register the Twitch application

1. Sign in to the [Twitch Developer Console](https://dev.twitch.tv/console/apps) as the account that should own the app. `HumanBeanGames` is the sensible owner.
2. Enable two-factor authentication if Twitch requests it.
3. Register a uniquely named application, for example `Stream Town Reloaded - HumanBeanGames`.
4. Use `http://localhost:3000` as the OAuth redirect URL. The device flow does not redirect there, but Twitch requires a registered URL.
5. Choose **Chat Bot** as the category.
6. Set the client type to **Public**. Stream Town runs on an end user's PC and must not embed a client secret.
7. Copy the **Client ID**. A Client ID is public; a Client Secret is not needed.

## 3. Authorize `HumanBeanBot`

1. Run `cargo run -p stream_town_tools` from `bevy-port`, then open the **Twitch** tab.
2. Paste the app's Client ID.
3. Set the bot account to `HumanBeanBot`.
4. Click **Authorize bot**.
5. On the Twitch activation page, verify that the signed-in account is `HumanBeanBot`. Use a private browser window if the browser keeps selecting `HumanBeanGames`.
6. Approve the two requested scopes: `chat:read` and `chat:edit`.

The setup tool validates the returned token, refuses a token for the wrong account or app, and writes it to the operating-system credential vault (Windows Credential Manager on the initial supported platform). No token file is created. Stream Town validates or refreshes the token at startup and validates it hourly.

## 4. Prepare the channel

1. In `HumanBeanGames` chat, run `/mod HumanBeanBot`. This is recommended for normal bot rate limits and moderation visibility.
2. In the tools application's **Twitch** tab, set the channel to `humanbeangames`, enable Twitch, and click **Save runtime config**. This writes public settings only to `.stream-town/config.ron`.
   Enter numeric Twitch user IDs in **Game-master Twitch user IDs** only for
   operators who should receive Unity-compatible cheat commands. Login/display
   names are not accepted because they can change.
3. Start or load the world.
4. When Stream Town displays its six-digit broadcaster connection code, send `!connect 123456` from the `HumanBeanGames` account, replacing `123456` with the displayed code.
5. From a viewer account, test `!join`. The in-game agent and processed-command counts should increase under that viewer's stable Twitch user ID.

The connection code is an application-level safety gate. It proves that the broadcaster present in chat is deliberately enabling bot output for this running game session.

Broadcaster and moderator status grants the existing staff command set, but it
does not grant game-master cheats. Those commands require an exact ID from
`twitch.game_master_ids`; the checked-in list is intentionally empty. Local
debug injection bypasses the list in the same way Unity's session bridge did.

## 5. Configure OBS

1. In OBS, run **Tools > Auto-Configuration Wizard** and choose streaming as the priority.
2. Open **Settings > Stream**, select **Twitch**, and use **Connect Account**.
3. Sign in as `HumanBeanGames`, not `HumanBeanBot`.
4. Create a gameplay scene and add **Game Capture** for the Stream Town game window. Use Window Capture only if Game Capture is incompatible.
5. Confirm that Desktop Audio contains game audio and Mic/Aux contains the intended microphone. Make a local recording before going live.
6. Add separate Starting, Gameplay, BRB, and Ending scenes as needed.
7. Run a Twitch bandwidth test or a short private/test broadcast, watch OBS's dropped-frame and encoder-overload counters, and then disable test mode before the real stream.

OBS's Twitch session, Twitch stream key, and broadcast settings are independent of Stream Town's bot OAuth credentials.

## Connection controls and diagnostics

- `F1`: intentionally disconnect the Twitch bot.
- `F2`: reconnect after credentials or channel settings change.
- In-game HUD state: `Twitch: Connected` (or the broadcaster authorization prompt).
- A wrong-account authorization is rejected before IRC is started.
- If authorization expires or is revoked and cannot be refreshed, reopen the Bevy tools **Twitch** tab and authorize the bot again.

Official references:

- [Register a Twitch app](https://dev.twitch.tv/docs/authentication/register-app)
- [Twitch device-code OAuth](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#device-code-grant-flow)
- [Twitch IRC authentication and scopes](https://dev.twitch.tv/docs/chat/irc/#authenticating-with-the-twitch-irc-server)
- [Twitch token validation](https://dev.twitch.tv/docs/authentication/validate-tokens/)
- [OBS quick-start guide](https://obsproject.com/kb/quick-start-guide)
