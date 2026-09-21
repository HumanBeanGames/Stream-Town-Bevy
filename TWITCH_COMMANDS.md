# Stream Town Twitch command reference

For the current Bevy game. Open this reference from **Connections > Twitch accounts >
Open command guide** or **Tools > Twitch**. For account setup, see the
[Twitch walkthrough](TWITCH_SETUP.md); Discord has its own [optional setup guide](DISCORD_SETUP.md).

Commands are case-insensitive and begin with `!`. Building type names are always written as one
no-space PascalCase word, such as `TownHall`, `OreStorage`, `ProspectorHut`, and `FishingHut`.
Run `!buildings` in chat for the building names unlocked in the current town.
Several commands can be chained in one message and execute from left to right, for example
`!cam right 3 !cam up 2` or `!build Wall !left 2 !beginplace !up 3 !confirm`. If any segment is
not valid command syntax, the whole message is rejected before anything executes.
Runtime failures, such as insufficient resources, reject that action; they do not
roll back earlier successful actions in the message. Angle brackets mean a required
value; square brackets mean an optional value. Do not type the brackets.

Gameplay and game command execution wait until Twitch confirms the stream is publicly
live. Chat can connect earlier; commands received while paused wait to execute.

## Find what you need

- [Getting started](#getting-started)
- [Work assignment and recovery](#work-assignment-and-recovery)
- [Character appearance](#character-appearance)
- [Town economy](#town-economy)
- [Building catalog and costs](#building-catalog-and-costs)
- [Building IDs](#building-ids-bids)
- [Recruitment](#recruitment-and-npc-citizens)
- [Camera controls](#camera-and-locating-citizens)
- [Town planning](#town-planning-and-diagnostics)
- [Voting and rulership](#voting-technology-and-rulership)
- [Information](#information)
- [Automatic announcements](#automatic-announcements)
- [City timelapse](#city-timelapse-output)
- [Staff and game-master commands](#moderator-and-game-master-commands)
- [Recognized but not implemented](#recognized-but-not-implemented)

## Getting started

| Command | Purpose |
|---|---|
| `!join` | Create your citizen. Aliases: `!create`, `!start`, `!crate`, `!crete`, `!creta`, `!ceate`, `!cate`, `!crtea`, `!ligma`. |
| `!help` | Post a link to this reference. |
| `!role` | Show your current role. |
| `!role <role>` | Change to an available role with an open role slot. |
| `!roles` | List currently available roles. Nursery, ProspectorHut, and Greenhouse add Forester, Prospector, and Tender slots. |
| `!health` | Show your health. |
| `!experience` / `!exp` / `!level` | Show your current role level and experience. |
| `!level <role>` | Show your saved level and experience for a particular profession. |

## Work assignment and recovery

| Command | Purpose |
|---|---|
| `!station` | List compatible work-station numbers. |
| `!station <id>` | Prefer one compatible station. |
| `!target` | List compatible work-target numbers. |
| `!target <id>` | Prefer one compatible work target. |
| `!stuck` | Return your citizen to a reachable cell. |
| `!revive [player]` | Revive yourself, or another player if your role supports revival. |

Foresters plant trees, Prospectors survey a 5–20-cell spiral around their hut and can discover ore
clusters, and Tenders plant berry bushes in open fields. These workers require their matching
completed building before the role is available.

## Character appearance

| Command | Purpose |
|---|---|
| `!hair <index>` | Change hair style. |
| `!facialhair <index>` | Change facial-hair style. |
| `!eyes <index>` | Change eye style. |
| `!body <index>` | Change body type. |
| `!haircolor <index>` | Change hair colour. |
| `!eyecolor <index>` | Change eye colour. |
| `!namecolor <name\|#RRGGBB>` | Change the colour of your in-world name. Alias: `!namecolour`. |
| `!light <name\|#RRGGBB>` | Change your personal night-light colour. Aliases: `!lightcolor`, `!lightcolour`. |

Named colours are `red`, `orange`, `yellow`, `green`, `cyan`, `blue`, `purple`, `pink`, `white`,
and `warmwhite`. Hex colours use six digits, such as `#72C8FF`.
Appearance indices start at 1; the available range depends on the authored model variants.

## Town economy

| Command | Purpose |
|---|---|
| `!buy <amount> <resource>` | Ruler or staff: spend town gold to buy a resource at the authored rate and storage limit. |
| `!sell <amount> <resource>` | Ruler or staff: sell available town stock for gold at the authored rate and tax. |
| `!townstats` | Show town statistics. |
| `!population` | List living Twitch players (`P`) and NPC recruits (`R`) separately for every currently available role. |

## Building catalog and costs

| Command | Purpose |
|---|---|
| `!buildings` | List unlocked building types using their exact no-space PascalCase command names. |
| `!cost <BuildingName>` | Show the current construction cost and technology-limited maximum level. Alias: `!buildcost`. Example: `!cost OreStorage`. |
| `!build <BuildingName>` | Start a placement preview. The preview times out after 60 seconds without another placement command. |
| `!build thickpath` | Place ordinary paths three fine cells wide. Diagonal steps fill their two inner bridge cells; only the routed centreline must be valid, while blocked flank/bridge sections are skipped independently. Existing paths may be crossed and are not rebuilt. |

### Placing ordinary buildings

After `!build <BuildingName>`, use any of the following commands to position the translucent
blueprint. Every move resets its 60-second timeout.

| Command | Purpose |
|---|---|
| `!move <up\|down\|left\|right\|rotate> [amount]` | Move in visible screen directions or rotate in 90-degree quarter turns. Several actions may be supplied in one command. |
| `!up [amount]`, `!down [amount]`, `!left [amount]`, `!right [amount]` | Short forms for moving the pending blueprint. |
| `!center` | Move the pending blueprint to the town position under the exact centre of the broadcast view. Paths retain one-third-cell precision and refresh an active routed preview. |
| `!rotate [amount]` | Rotate the pending blueprint by quarter turns. |
| `!confirm` / `!accept` | Pay the displayed cost and create the construction site if the preview is valid. Existing path sections under its footprint are removed. |
| `!cancel` | Discard the pending blueprint immediately. |

If the town cannot afford the building, the rejection identifies every missing resource and amount.
Square-footprint buildings can also be rotated after construction through their BID.

### Placing walls and paths

Immediately using `!confirm` after `!build Wall` or `!build Path` places one section. To place
multiple sections instead:

1. Move the placement cursor to the first cell and use `!beginplace`.
2. For walls, move only horizontally or vertically toward the other endpoint. A Path cursor instead
   moves one third of a town cell per step. After each move, the preview uses diagonal-aware A* to
   show the complete shortest walkable route from the saved start to the cursor, automatically
   snaking between buildings and resources.
3. Use `!confirm` to set the current cursor as the endpoint, pay for, and place every section; use
   `!cancel` to discard it.

Calling `!beginplace` again replaces the start. `!endplace` remains a compatibility alias for
freezing the current endpoint but is not required. A diagonal wall endpoint is rejected, and a route
that exceeds available resources is not constructed. Path routing softly favours the central third
of each town cell while retaining narrow side passages and completed gatehouse doors as valid
options. Paths cost 150 Wood and 130 Ore per successfully placed fine-grid section at level 1,
conform to the terrain, remain walkable, and do not consume building capacity.
Each completed Path level gives citizens on that fine-grid section 5% additional movement speed. Path
technologies form their own branch beginning in Age 1. Citizen route planning prices that speed into
its A* cost, so a faster path can be preferred over a slightly shorter unsurfaced route.

## Building IDs (BIDs)

A BID is the small number assigned to one instance of a building type. BIDs are scoped to that type:
`Tower 3` means the third Tower, while `House 3` means the third House. You never need the internal
`building:runtime_...` identifier.

| Command | Purpose |
|---|---|
| `!bid <BuildingName>` | List the current BID numbers and briefly draw each number over its building. |
| `!info <BuildingName> <BID>` | Show details for one building instance. |
| `!upgrade <BuildingName> <BID> [levels]` | Pay for and begin one or several levels as a single construction effort. Each requested target rung costs the building-authored fraction of base cost multiplied by `(target level - 1)` and stops at the researched cap. Examples: `!upgrade Tower 3` or `!upgrade Tower 3 4`. |
| `!rotatebuilding <BuildingName> <BID> [quarter turns]` | Rotate an already placed square-footprint building. Walls and gates orient from their neighbours. |
| `!buildinglight <BuildingName> <BID> <name\|#RRGGBB>` | Ruler only: change that building's night-light colour. |
| `!remove <BuildingName> <BID>` | Ruler only: permanently remove the selected building. The TownHall cannot be removed. |

`!level <BuildingName> <BID> [amount]` also requests paid upgrades, with the same resource
costs and technology caps. `!levelall <BuildingName> <level>` attempts paid upgrades for
each instance below the target level. These commands are not restricted to game masters.
The batch can succeed for some buildings and fail for others; it does not undo successful
upgrades. Use `!upgrade` when you want to select a single building clearly.

## Recruitment and NPC citizens

| Command | Purpose |
|---|---|
| `!recruit <role> [amount]` | Ruler or staff: recruit town-controlled citizens when role and housing capacity allow. |
| `!recruits` | Ruler or staff: show the recruit count. |
| `!rid` | Ruler or staff: list recruit numbers. |
| `!rinfo <id>` | Ruler or staff: show one recruit. |
| `!rrole <id> <role>` | Ruler or staff: assign a recruit's role. |
| `!rdismiss <id>` | Ruler or staff: dismiss a recruit. |

Recruits remain at level 1 in every profession and do not retain profession XP. Twitch player
citizens retain the full authored progression range (currently level 1000); chat announces every
multiple-of-ten role level.

## Camera and locating citizens

| Command | Purpose |
|---|---|
| `!ping` | Briefly mark your citizen in the world. |
| `!cam <direction> [amount]` | Ruler/operator: smoothly move the broadcast camera with `up`, `down`, `left`, `right`, `in`, or `out`. Left/right use finer steps than up/down; multiple direction/amount pairs are accepted. |
| `!cam home` / `!resetcam` | Ruler/operator: return to the authored town composition and base zoom. |
| `!follow <username>` / `!follow me` | During an automatic-camera shot, follow one living player citizen without leaving automatic mode. |
| `!focus <BuildingName> <BID>` | During automatic direction or while you are placing a building, focus that building for 15 seconds. Successful placement commands refresh the focus timer. |

After 30 seconds without any acknowledged command, the broadcast camera can enter its automatic
director. Votes do not interrupt it. Damage switches the director to the damaged citizen or
building closest to the TownHall for 15 seconds, with a five-second redirect cooldown that prevents
simultaneous fights from making the camera strobe between targets. Any other ordinary command during
an automatic shot returns it home and restarts that timer.

## Town planning and diagnostics

These commands are restricted to the Ruler, broadcaster, moderators, and configured game masters
because they temporarily change the view for everyone watching the stream.

| Command | Purpose |
|---|---|
| `!pathfinding` | For 10 seconds, hide buildings and resource models and paint every inaccessible third-cell of the live movement grid black. Accessible terrain remains normally rendered. |
| `!floorplan` | For 10 seconds, hide buildings and resource models and paint the complete authored placement/exclusion footprint of every building, enemy camp, and active resource black. |

Issuing either command again restarts its ten-second timer. Issuing the other switches directly to
that view. Both modes are visual only: they do not modify navigation, placement, saves, or gameplay.

## Voting, technology, and rulership

| Command | Purpose |
|---|---|
| `!vote <option>` | Vote in a technology ballot by number (`!vote 1`, `!vote 2`, or `!vote 3`), in a ruler ballot by player name, or in an event/keep-ruler ballot with `!vote yes` or `!vote no`. |
| `!event <event type>` | Request a public vote to switch the active community event. Valid types are `prospecting`, `reforestation`, `agricultural`, `rebalance`, `awakening`, `economic`, and `invasion`. Valid requests share a one-hour global cooldown. |
| `!rulervote` | Broadcaster or moderator: start a ruler vote. Being Ruler alone does not grant this command. |
| `!resign` | Resign as ruler. |

Use `!vote yes` or `!vote no` for a yes/no ballot; standalone yes/no shortcuts are not supported.
Anyone can request an event. For example, enter `!event prospecting` in Twitch chat. When the
event ballot appears, viewers vote with `!vote yes` or `!vote no` before the two-minute timer ends.
The accepted event requests and their effects are:

| Request | Effect when the vote passes |
|---|---|
| `!event prospecting` | Prospectors work at 3x speed; Foresters and Tenders work at 0.5x speed. |
| `!event reforestation` | Foresters work at 3x speed; Prospectors and Tenders work at 0.5x speed. |
| `!event agricultural` | Tenders work at 3x speed; Prospectors and Foresters work at 0.5x speed. |
| `!event rebalance` | Restore normal Prospector, Forester, and Tender rates. |
| `!event awakening` | Increase all experience gains by 20%. |
| `!event economic` | Make all gathering actions 10% faster. |
| `!event invasion` | Increase monster-wave size by 50% and double gold from monster kills. |

Only one community event is active at a time. A request waits in the reusable vote queue when a
ruler election is already in progress, then uses the same two-minute ballot panel. A strict
majority of cast votes activates the proposal; a tie or majority `no` retains the current event.
The one-hour request cooldown is global, rather than per viewer, and starts when a valid request is
accepted.

The Nursery, ProspectorHut, and Greenhouse technologies begin locked but have no prerequisite
technology, so each can appear in a technology vote immediately. Their post-vote objectives use a
different resource from the one the unlocked worker restores.

Guardhouse research becomes available from the Age 1 Tower branch and requires collecting 30,000
Gold. A Guardhouse costs three times a Tower, uses the Tower footprint and blue-tinted model, and
provides one permanent free Defender that does not consume recruit or Defender capacity, appear in
the recruit population UI, or increase enemy-wave scaling. That guard prioritizes enemies nearest
its own Guardhouse; unlike a Tower, the Guardhouse fires no projectiles.

Each three-choice technology ballot contains one deepest eligible specialization, one shallowest
eligible fundamental, and one random remaining technology. Equal-depth choices are tie-broken
deterministically from the town seed. The UI labels the first two choices `Specialized!` and
`Fundamental!`.

Food, Wood, and Ore collection objectives count only resources actually gathered and delivered by
citizens; bought or passive stock does not count. Gold spent on resource purchases while a Gold
collection objective is active increases its target by the same amount. The same adjustment is
tracked while a three-choice ballot is visible and follows the Gold option only if it wins.

## Information

| Command | Purpose |
|---|---|
| `!info <username>` | Show a player citizen's health, current role, and level in every profession. |
| `!info <resource\|role\|building\|enemy> [BID]` | Show authored information; add a BID for one building instance. |
| `!stdiscord` | Show the Stream Town community link. |

`!stdiscord` displays the community invite; it does not configure the optional announcement
bot or reveal your configured server/channel. Those settings are in Connections.

## Automatic announcements

These are generated by the game and need no viewer command. Resource stockpiles becoming
full and buildings being destroyed are announced in **Twitch bot chat** by default.
Buildings removed by the Ruler also count as removed/destroyed for this notification.
Full-stockpile messages include the resource and amount/capacity. They fire once on reaching
capacity and rearm after the stock falls below 90%, preventing repeated messages for every
small deposit. Loading a town does not replay alerts for already-full stockpiles.

The optional Discord bot can also publish these two categories. Both new checkboxes start
selected, including when upgrading an existing checklist; Discord's master delivery switch
still stays off unless you enable it. Change categories in **Settings > Discord**. Existing
level-up preferences are preserved. Turning off a Discord category affects Discord only.

Discord can additionally announce confirmed go-live links, level milestones, citizen deaths,
technology votes, ruler election results, and community vote results. Requested command
replies and acknowledgements stay in Twitch chat. See the [Discord guide](DISCORD_SETUP.md).

## City timelapse output

Timelapse frequency and Dynamic mode are configured in the in-game Settings menu. The defaults are
`1 hour` and Dynamic enabled. Each validly confirmed build subtracts ten minutes from the current
countdown while Dynamic mode is enabled.

For each town, screenshots, source frames, and the continuously rebuilt video are stored below the
Bevy project in `.stream-town/timelapses/<town>/`. For Beanville, the files are:

| Output | Location |
|---|---|
| Clean screenshots | `.stream-town/timelapses/Beanville/city-<timestamp>-<rate>-<mode>.png` |
| Video source frames | `.stream-town/timelapses/Beanville/video-frames/` |
| Timelapse video | `.stream-town/timelapses/Beanville/city-timelapse.mp4` |

Each capture uses the `!cam home` composition with gameplay UI, health bars, diagnostics, and build
previews hidden. Screenshot names include the active frequency and Fixed/Dynamic mode, and every
video frame burns the current values into its lower-left corner.

## Moderator and game-master commands

**Staff** means the broadcaster or a Twitch moderator. **Game master** means an exact
numeric Twitch user ID in `twitch.game_master_ids`; a staff badge or the Ruler role alone
does not grant it. The default game-master list is empty. Camera/planning commands accept
the Ruler, staff, or a configured game master. Recruitment and saving accept the Ruler or
staff; configured game-master status alone does not bypass those particular checks.

| Command | Purpose |
|---|---|
| `!modrole <player> <role>` | Staff: change another player's role, subject to role availability; cannot appoint an unelected Ruler. |
| `!save` | Ruler or staff: force-save the active town. |

The following require **game-master** access:

| Command | Purpose |
|---|---|
| `!qevent <event>` | Queue `fishgod` / `fish_god` or `monsterraid` / `monster_raid` / `raid`. Public community-event changes use `!event` and a vote. |
| `!stopevent` | Stop the active event. |
| `!tbuildcosts` | Toggle building costs. |
| `!trolelimits` | Toggle player role limits. |
| `!addresource <resource> <amount>` | Add or remove town resources. |
| `!kill <player>` / `!grevive <player>` | Kill or game-master-revive a player. |
| `!givexp <player> <amount>` / `!givexpall <amount>` | Grant role experience. |
| `!levelup <player> [amount]` | Grant role levels. |
| `!cobj` | Complete the first active technology objective. |
| `!randtech` / `!techvote` | Start an eligible technology goal or a three-option technology vote. |
| `!gaction` | Perform the active game-master event action, when supported. |
| `!unlockall` / `!unlockage2` | Unlock reachable technologies, or the reachable Age 1 technologies that lead toward Age 2. |
| `!resetid building <BuildingName>` | Report the building count; stable IDs already exist, so no counter reset is performed. |

## Recognized but not implemented

These commands are retained so old chat macros receive an explicit response rather than silently
failing. They do not currently change gameplay:

| Command | Current response |
|---|---|
| `!pets` | Pet commands are not implemented yet. |
| `!pet [pet]` | Pet commands are not implemented yet. |
| `!givepet <player> <pet>` | Requires game-master permission, then reports that pet commands are not implemented yet. |
| `!praise` | Praise is not implemented yet. |

Subscriber/game-master red-panda pets can be assigned automatically, but the chat commands
above cannot select or grant them. The configured Fish God Channel Points reward also maps
to the unimplemented praise command; it does not complete that event.

An unknown command receives: `Invalid Command! Type !help for the list of commands!`
