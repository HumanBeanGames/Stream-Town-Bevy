# Stream Town Twitch command reference

Commands are case-insensitive and begin with `!`. Building type names are always written as one
no-space PascalCase word, such as `TownHall`, `OreStorage`, `ProspectorHut`, and `FishingHut`.
Run `!buildings` in chat for the building names unlocked in the current town.

## Getting started

| Command | Purpose |
|---|---|
| `!join` | Create your citizen. The original spelling aliases (`!create`, `!start`, and the source-authored typo aliases) are still accepted. |
| `!help` | Post a link to this reference. |
| `!role` | Show your current role. |
| `!role <role>` | Change to an available role with an open role slot. |
| `!roles` | List currently available roles. Nursery, ProspectorHut, and Greenhouse add Forester, Prospector, and Tender slots. |
| `!health` | Show your health. |
| `!experience` / `!exp` | Show your current role level and experience. |

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

## Town economy

| Command | Purpose |
|---|---|
| `!buy <amount> <resource>` | Spend town gold to buy a resource at the authored rate and storage limit. |
| `!sell <amount> <resource>` | Sell available town stock for gold at the authored rate and tax. |
| `!townstats` | Show town statistics. |

## Building catalog and costs

| Command | Purpose |
|---|---|
| `!buildings` | List unlocked building types using their exact no-space PascalCase command names. |
| `!cost <BuildingName>` | Show the current construction cost and technology-limited maximum level. Alias: `!buildcost`. Example: `!cost OreStorage`. |
| `!build <BuildingName>` | Start a placement preview. The preview times out after 30 seconds without another placement command. |

### Placing ordinary buildings

After `!build <BuildingName>`, use any of the following commands to position the translucent
blueprint. Every move resets its 30-second timeout.

| Command | Purpose |
|---|---|
| `!move <up\|down\|left\|right\|rotate> [amount]` | Move in visible screen directions or rotate in 90-degree quarter turns. Several actions may be supplied in one command. |
| `!up [amount]`, `!down [amount]`, `!left [amount]`, `!right [amount]` | Short forms for moving the pending blueprint. |
| `!rotate [amount]` | Rotate the pending blueprint by quarter turns. |
| `!confirm` / `!accept` | Pay the displayed cost and create the construction site if the preview is valid. |
| `!cancel` | Discard the pending blueprint immediately. |

If the town cannot afford the building, the rejection identifies every missing resource and amount.
Square-footprint buildings can also be rotated after construction through their BID.

### Placing wall lines

Walls deliberately cannot be confirmed immediately after `!build Wall`.

1. Move the wall cursor to the first cell and use `!beginplace`.
2. Move only horizontally or vertically to the other endpoint and use `!endplace`.
3. Review the complete red/green wall-line preview.
4. Use `!confirm` to pay for and place every section, or `!cancel` to discard it.

Calling `!beginplace` or `!endplace` again replaces that endpoint. A diagonal endpoint is rejected,
and a line that exceeds available resources is not constructed.

## Building IDs (BIDs)

A BID is the small number assigned to one instance of a building type. BIDs are scoped to that type:
`Tower 3` means the third Tower, while `House 3` means the third House. You never need the internal
`building:runtime_...` identifier.

| Command | Purpose |
|---|---|
| `!bid <BuildingName>` | List the current BID numbers and briefly draw each number over its building. |
| `!info <BuildingName> <BID>` | Show details for one building instance. |
| `!upgrade <BuildingName> <BID>` | Pay for and begin the next-level construction effort. Example: `!upgrade Tower 3`. |
| `!rotatebuilding <BuildingName> <BID> [quarter turns]` | Rotate an already placed square-footprint building. Walls and gates orient from their neighbours. |
| `!buildinglight <BuildingName> <BID> <name\|#RRGGBB>` | Ruler only: change that building's night-light colour. |
| `!remove <BuildingName> <BID>` | Ruler only: permanently remove the selected building. The TownHall cannot be removed. |

`!level <BuildingName> <BID> [amount]` and `!levelall <BuildingName> <level>` remain available to
configured operators for bulk progression/testing; normal play should use `!upgrade`.

## Recruitment and NPC citizens

| Command | Purpose |
|---|---|
| `!recruit <role> [amount]` | Recruit town-controlled citizens when role and housing capacity allow. |
| `!recruits` | Show the recruit count. |
| `!rid` | List recruit numbers. |
| `!rinfo <id>` | Show one recruit. |
| `!rrole <id> <role>` | Assign a recruit's role. |
| `!rdismiss <id>` | Dismiss a recruit. |

## Camera and locating citizens

| Command | Purpose |
|---|---|
| `!ping` | Briefly mark your citizen in the world. |
| `!cam <direction> [amount]` | Ruler/operator: smoothly move the broadcast camera with `up`, `down`, `left`, `right`, `in`, or `out`. Multiple direction/amount pairs are accepted. |
| `!cam home` / `!resetcam` | Ruler/operator: return to the authored town composition and base zoom. |
| `!follow <username>` / `!follow me` | During an automatic-camera shot, follow one living player citizen without leaving automatic mode. |

After 30 seconds without any acknowledged command, the broadcast camera can enter its automatic
director. Any ordinary command during an automatic shot returns it home and restarts that timer.

## Voting, technology, and rulership

| Command | Purpose |
|---|---|
| `!vote <option>` | Vote in the active ruler or technology vote using the displayed option number/name. |
| `!rulervote` | Ruler/operator: start a ruler vote. |
| `!resign` | Resign as ruler. |

The Nursery, ProspectorHut, and Greenhouse technologies begin locked but have no prerequisite
technology, so each can appear in a technology vote immediately. Their post-vote objectives use a
different resource from the one the unlocked worker restores.

## Information

| Command | Purpose |
|---|---|
| `!info <resource\|role\|building\|enemy> [BID]` | Show authored information; add a BID for one building instance. |
| `!stdiscord` | Show the Stream Town community link. |

## Moderator and game-master commands

Game-master access is granted only to Twitch user IDs configured in the operator settings.

| Command | Purpose |
|---|---|
| `!modrole <player> <role>` | Change another player's role. |
| `!event <event>` / `!qevent <event>` | Trigger or queue a supported town event. |
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
| `!unlockall` / `!unlockage2` | Unlock reachable technologies or the Age 2 path. |
| `!resetid <kind> <value>` | Validate/reset a supported stable-ID sequence. |
| `!save` | Force-save the active town. |

## Recognized but not implemented

These commands are retained so old chat macros receive an explicit response rather than silently
failing. They do not currently change gameplay:

| Command | Current response |
|---|---|
| `!pets` | Pet commands are not implemented yet. |
| `!pet [pet]` | Pet commands are not implemented yet. |
| `!givepet <player> <pet>` | Pet commands are not implemented yet. |
| `!praise` | Praise is not implemented yet. |

An unknown command receives: `Invalid Command! Type !help for the list of commands!`
