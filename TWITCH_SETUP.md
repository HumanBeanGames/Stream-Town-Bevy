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

1. In Unity, open **Tools > Stream Town > Twitch Bot Setup**.
2. Paste the app's Client ID.
3. Set the bot account to `HumanBeanBot`.
4. Click **Authorize Bot With Twitch**.
5. On the Twitch activation page, verify that the signed-in account is `HumanBeanBot`. Use a private browser window if the browser keeps selecting `HumanBeanGames`.
6. Approve the two requested scopes: `chat:read` and `chat:edit`.

The setup window validates the returned token, refuses a token for the wrong account or app, and writes it to the current user's Unity application-data directory. The file is outside the repository and is not included in builds. Stream Town validates the token at startup and hourly, and refreshes a public-client token when Twitch reports that it has expired.

## 4. Prepare the channel

1. In `HumanBeanGames` chat, run `/mod HumanBeanBot`. This is recommended for normal bot rate limits and moderation visibility.
2. In Stream Town's main menu, set the channel to `HumanBeanGames`.
3. Start or load the world.
4. When Stream Town displays its six-digit broadcaster connection code, send `!connect 123456` from the `HumanBeanGames` account, replacing `123456` with the displayed code.
5. From a viewer account, test `!create` and then `!help`. Replies should be posted by `HumanBeanBot`.

The connection code is an application-level safety gate. It proves that the broadcaster present in chat is deliberately enabling bot output for this running game session.

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
- Unity Console success message: `Twitch bot 'humanbeanbot' connected to 'humanbeangames'.`
- A wrong-account authorization is rejected before IRC is started.
- If authorization expires or is revoked, reopen **Tools > Stream Town > Twitch Bot Setup** and authorize the bot again.

Official references:

- [Register a Twitch app](https://dev.twitch.tv/docs/authentication/register-app)
- [Twitch device-code OAuth](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#device-code-grant-flow)
- [Twitch IRC authentication and scopes](https://dev.twitch.tv/docs/chat/irc/#authenticating-with-the-twitch-irc-server)
- [Twitch token validation](https://dev.twitch.tv/docs/authentication/validate-tokens/)
- [OBS quick-start guide](https://obsproject.com/kb/quick-start-guide)
