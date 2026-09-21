# Set up Discord announcements in Stream Town

This guide walks you through creating a Discord bot, inviting it to your server,
and connecting it through the Bevy game. No coding or command-line tools are needed.

**Discord is optional.** You can play and stream without it. The existing Twitch
account requirements still apply. Discord announcements are disabled by default.

The bot publishes automatic town announcements and your Twitch stream link when
you go live. Replies to player commands, requested status information, and action
acknowledgements are not sent to Discord.

Open this document with **Open Discord setup guide** in **Connections > Discord bot**
or **Tools > Discord**. It opens as a readable local browser page with keyboard-accessible
links and printable text. Changes to the guide appear when reopened without a rebuild.
For the required Twitch connections, use the [Twitch setup guide](TWITCH_SETUP.md).

## Find what you need

- [Before you start](#before-you-start)
- [1. Create the bot](#1-create-the-bot)
- [2. Set its installation permissions](#2-set-its-installation-permissions)
- [3. Invite it and choose a channel](#3-invite-it-and-choose-a-channel)
- [4. Open Connections in the game](#4-open-connections-in-the-game)
- [5. Copy the server and channel IDs](#5-copy-the-server-and-channel-ids)
- [6. Store the bot token](#6-store-the-bot-token)
- [7. Save and check access](#7-save-and-check-access)
- [8. Choose your announcements](#8-choose-your-announcements)
- [Check the first announcement](#check-the-first-announcement)
- [Change or disable announcements later](#change-or-disable-announcements-later)
- [Troubleshooting](#troubleshooting)
- [Using Tools instead](#using-tools-instead)
- [Storage and delivery details](#storage-and-delivery-details)

## Before you start

Have Stream Town, Discord, and a web browser available. Use a Discord account
that owns the destination server or has **Manage Server** permission to install
apps. [Discord's installation requirements](https://docs.discord.com/developers/quick-start/getting-started#choosing-installation-contexts)

In this guide, a **server** is your Discord community and a **channel** is the
place within it where announcements appear. Discord calls a server a **guild**
in some setup screens. A **bot token** is the private credential the game uses
to send messages as your bot.

Menu paths such as **Settings > Discord** mean open Settings, then select Discord.
All steps are written out; no screenshot or colour recognition is required.

## 1. Create the bot

1. Open the [Discord Developer Portal](https://discord.com/developers/applications)
   and sign in.
2. Select **New Application**, enter a name such as **Stream Town**, and create it.
3. Open **Bot** in the sidebar. New applications already have a bot user.

Leave the privileged intent switches off; this integration only sends town
announcements. [Discord's bot setup guide](https://docs.discord.com/developers/quick-start/getting-started#configuring-your-bot)

## 2. Set its installation permissions

1. Open **Installation** in the application's sidebar.
2. Enable **Guild Install** under Installation Contexts.
3. Under Install Link, select **Discord Provided Link**.
4. Under **Default Install Settings > Guild Install**, add the **bot** scope.
5. In the bot permissions list, select **View Channels** and **Send Messages**.
6. Save the changes.

These permissions let the bot access the destination and post announcements;
Administrator permission is unnecessary. [Discord's install settings](https://docs.discord.com/developers/quick-start/getting-started#adding-scopes-and-bot-permissions)
and [permission definitions](https://docs.discord.com/developers/topics/permissions)

## 3. Invite it and choose a channel

1. Copy the **Install Link** from Installation and open it in your browser.
2. Choose **Add to server**, select your server, and complete authorization.
3. Check that the bot appears in the server's member list.

[Discord's server installation instructions](https://docs.discord.com/developers/quick-start/getting-started#installing-your-app)

Choose a text channel, such as `#town-events`, or an announcement channel.
Stream Town supports those two channel types; use the channel itself rather
than a thread, forum, or direct message.

For a private or restricted channel, open **Edit Channel > Permissions**, add
the bot as a member override, and allow **View Channel** and **Send Messages**.
Save the channel changes. Channel permissions can override the permissions
granted during installation. [Discord's channel permission rules](https://docs.discord.com/developers/topics/permissions)

## 4. Open Connections in the game

1. Launch Stream Town using the **Stream Town** desktop shortcut.
2. Open **Main Menu > Connections**.
3. Read and accept the privacy prompt, then select **Discord bot**.

While already playing, use **Settings > Connection > Set up connections**,
then choose **Discord bot** after the privacy prompt. You can also reach this
screen through **Settings > Discord > Set up optional Discord bot**.

The protected Connections screen blacks out Stream Town's own Twitch video
output while it is open. Separate capture software is outside the game's
control. The public announcement checklist in Settings does not black out
the stream.

## 5. Copy the server and channel IDs

1. In the Discord desktop app, open **User Settings > Advanced** and enable
   **Developer Mode**.
2. Right-click your server's icon and select **Copy Server ID**.
3. Return to the game's Discord bot screen and paste it into **Discord server ID**.
4. In Discord, right-click the destination channel and select **Copy Channel ID**.
5. Paste that value into **Discord channel ID** in the game.

Both fields need the copied numbers, not a server name, channel name, invite
link, or the application's ID. [Discord's instructions for copying IDs](https://support.discord.com/hc/en-us/articles/206346498-Where-can-I-find-my-User-Server-Message-ID)

**Check:** both fields contain numbers, and the channel belongs to the server
you selected.

## 6. Store the bot token

1. Return to your application in the Developer Portal and open **Bot**.
2. Under Token, select **Reset Token** and complete any verification.
3. Copy the new token.
4. Immediately return to the game and select **Paste and store bot token**.
   Copying another item first would replace the token on your clipboard.
5. Confirm that the game reports **Bot token is stored securely.**
6. Clear the token from your clipboard when finished.

Use the token exactly as copied, without adding a `Bot` prefix. The game stores
it in the operating system's credential vault and never displays it in the
connection screen. Do not paste the token into chat, documentation, or an ID field.

Discord shows a new token when it is generated; if you lose it, reset it and
store the replacement in the game. Resetting an existing bot's token invalidates
its previous token, so any other software using that bot will also need the
replacement. [Discord's token instructions](https://support-dev.discord.com/hc/en-us/articles/6470840524311-Why-can-t-I-copy-my-bot-s-token)

## 7. Save and check access

1. Select **Save Discord connection** to save the server and channel IDs.
2. Select **Check channel access** and wait for the result.
3. Look for **Channel verified: #your-channel**, using your actual channel name.

This confirms that the token can access the channel and that it belongs to the
configured server. **The check posts no message.** Send Messages permission is
checked when an announcement is delivered, so a successful lookup alone does
not prove that posting is allowed.

If verification fails, follow [Troubleshooting](#troubleshooting) before enabling
announcements.

## 8. Choose your announcements

1. Leave Connections and open **Settings > Discord**.
2. Select the categories you want from the checklist below.
3. Select **Enable Discord announcements** to turn on delivery.

In the game, `[x]` means selected and `[ ]` means cleared. These switches save
immediately; the general Settings **Apply** button is not needed for them.
Changes take effect for subsequent announcements without a restart or rebuild.

| Checkbox | What it announces |
| --- | --- |
| Going live (with Twitch link) | A confirmed public Twitch broadcast, including your stream link. |
| Citizen level milestones | Automatic citizen level-up milestones. Clear this to silence level-up messages. |
| Citizen deaths | Automatic citizen death announcements. |
| Technology vote announcements | Automatic announcements about technology votes. |
| Ruler election results | The outcome of ruler elections. |
| Community event vote results | The outcome of community event votes. |
| Resource stockpiles full | A resource reaches its storage capacity. |
| Buildings destroyed | A building is destroyed or removed from the town, including Ruler removal. |

All eight categories start selected, but the master **Enable Discord announcements**
switch starts off. Selecting categories while the master switch is off sends nothing.

The two new categories are selected automatically when upgrading an older checklist.
Existing choices, including disabled level-up announcements, are preserved. You can turn
the new categories off and that choice stays saved. They also announce in Twitch bot chat
by default; these Discord checkboxes only control Discord delivery.

A full-stockpile notification fires once when capacity is reached. Stock must drop below
90% before it can announce another refill. Loading a town does not replay notifications
for already-full stockpiles or buildings that were destroyed before loading.

## Check the first announcement

Keep the game running and watch the destination channel when an enabled town
event occurs. A matching message confirms delivery, including sending permission.
Player command replies are deliberately excluded and will not test this path.

For a go-live announcement, keep **Going live (with Twitch link)** selected before
your next normal stream. The message is sent after Twitch confirms the public
broadcast and includes `https://www.twitch.tv/your-channel`. Opening the game or
running a bandwidth test does not trigger it. There is no need to start a public
stream just to run the channel-access check.

Successful go-live messages are remembered by Twitch stream and Discord
destination, avoiding repeats after reconnecting or restarting the game.

## Change or disable announcements later

- **Silence one category:** clear its checkbox in **Settings > Discord**.
- **Stop all announcements:** clear **Enable Discord announcements**. You can
  still play and stream, and your saved connection remains available.
- **Change the destination:** edit the IDs in **Connections > Discord bot**,
  select **Save Discord connection**, and run **Check channel access** again.
- **Replace the token:** repeat [Step 6](#6-store-the-bot-token), then check access.
- **Remove the saved credential:** first disable announcements, then select
  **Remove stored token** in Connections. This removes the local credential;
  it does not remove the bot from your Discord server.

## Troubleshooting

### My server is missing from the installation list

Check that you are signed into the intended Discord account, that it can
manage the server, and that **Guild Install** is enabled. Use **Add to server**
when following the install link. See [Discord's installation requirements](https://docs.discord.com/developers/quick-start/getting-started#choosing-installation-contexts).

### I cannot find Copy Server ID or Copy Channel ID

Enable **Developer Mode** in Discord's **User Settings > Advanced**, then open
the server or channel context menu again. Follow [Step 5](#5-copy-the-server-and-channel-ids).

### The game asks for valid server and channel IDs

Copy both numeric IDs again and save the connection before enabling announcements.
Do not use the application's ID, a channel link, or a name. The two IDs must
refer to the same server.

### The game says no token is stored, or the token is invalid

Copy the bot token again and immediately use **Paste and store bot token**.
Do not add spaces or a `Bot` prefix. If the token was reset in Discord, store
the replacement. A credential-vault error means storage failed; resolve the
reported error before continuing.

### Check channel access fails

Confirm that the bot is a member of the chosen server, the channel ID is correct,
and the bot can view that channel. Private channels may need an explicit member
permission. If the message reports rate limiting or a temporary connection failure,
wait and retry. If it rejects the channel type, choose a text or announcement channel.

### The channel check succeeds, but no announcements arrive

Confirm the master switch and the relevant category are both selected. Wait for
a new matching event while the game is running. Check **Send Messages** for the
bot on the exact destination channel, including any channel overrides.

Open **Tools > Discord** to read **Last game delivery status** when available.
A go-live message requires a confirmed public broadcast; bandwidth tests and
player-requested replies do not produce announcements.

## Using Tools instead

The game provides the complete setup path. **Tools > Discord** remains an
alternative and uses the same saved configuration and credential vault.

In Tools, enter the IDs and category choices, then select **Save announcement
settings**. Its checklist requires that explicit save, unlike the game's
immediate-save checklist. Paste the token into Tools' masked token field and
select **Store token**, then use **Check channel access**.

Save and close one editor before editing in the other so an old draft does not
replace newer choices.

## Storage and delivery details

The usual location for public configuration is
`bevy-port/.stream-town/discord.ron`. Delivery status is written to
`bevy-port/.stream-town/discord-status.txt`; Tools also displays that status.
The bot token is stored separately in the OS credential vault. You do not need
to edit these files to complete setup.

Delivery runs in the background. Temporary failures are retried with Discord's
rate-limit delays. Pending messages expire after ten minutes, and they are not
preserved across closing the game. Disabling a category also cancels its pending
retries. Announcements do not trigger Discord mentions.

For implementation reference, see Discord's
[Create Message API](https://docs.discord.com/developers/resources/message#create-message)
and [rate-limit documentation](https://docs.discord.com/developers/topics/rate-limits).
