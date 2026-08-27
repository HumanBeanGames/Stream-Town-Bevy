# Stream Town Twitch commands

Commands are case-insensitive and must start with `!`. Names such as `logger`,
`house`, `wood`, and player names are resolved to the game’s stable content or
Twitch IDs. Commands that change the town, camera, events, or other players are
restricted to the current ruler, moderators, broadcaster, or configured game
masters as appropriate.

## Player commands

| Command | Purpose |
|---|---|
| `!join` | Create your town character. Aliases: `!create`, `!start`, `!crate`, `!crete`, `!creta`, `!ceate`, `!cate`, `!crtea`, and the source-authored `!ligma`. |
| `!role [role]` | Show your current role or change role. |
| `!roles` | List available roles. |
| `!health` | Show your health. |
| `!experience` / `!exp` | Show role level and experience. |
| `!station [id]` | List or select a work station. |
| `!target [id]` | List or select a work target. |
| `!stuck` | Move your character to a reachable location. |
| `!pets` | List your unlocked pets. |
| `!pet [pet]` | List pets or equip one. |
| `!hair <index>` | Change hair style. |
| `!facialhair <index>` | Change facial-hair style. |
| `!eyes <index>` | Change eye style. |
| `!body <index>` | Change body type. |
| `!haircolor <index>` | Change hair colour. |
| `!eyecolor <index>` | Change eye colour. |
| `!buy <amount> <resource>` | Buy a town resource. |
| `!sell <amount> <resource>` | Sell a town resource. |
| `!revive [player]` | Revive yourself, or another player when your role permits it. |
| `!praise` | Praise the Fish God. |
| `!vote <option>` | Vote in the active ruler or technology vote. |
| `!townstats` | Show town statistics. |
| `!buildings` | List building types. |
| `!bid <building>` | List instance IDs for a building type. |
| `!recruits` | Show the recruit count. |
| `!rid` | List recruit IDs. |
| `!rinfo <id>` | Show a recruit. |
| `!rrole <id> <role>` | Assign a recruit’s role. |
| `!rdismiss <id>` | Dismiss a recruit. |
| `!info <id>` | Show authored information for content. |
| `!ping` | Show your character’s location marker. |
| `!stdiscord` | Show the Stream Town community link. |
| `!help` | Link to this command reference. |

## Ruler and operator commands

| Command | Purpose |
|---|---|
| `!build <building>` | Begin building placement. |
| `!move <up\|down\|left\|right\|rotate> [amount]` | Move or rotate the pending building. |
| `!up [amount]`, `!down [amount]`, `!left [amount]`, `!right [amount]` | Short forms for moving the pending building in visible screen directions. |
| `!rotate [amount]` | Rotate the pending building by quarter turns. |
| `!confirm` / `!accept` | Confirm pending building placement. |
| `!cancel` | Cancel pending building placement. |
| `!upgrade <building>` | Upgrade a building type. |
| `!level <role>` | Spend resources to level a role. |
| `!level <building> <id> [amount]` | Level a building instance. |
| `!levelall <building> <level>` | Level every instance of a building type. |
| `!remove <building> <id>` | Remove a building instance. |
| `!recruit <role> [amount]` | Recruit town-controlled characters. |
| `!cam <direction> [amount]` | Smoothly move the broadcast camera using `up`, `down`, `left`, `right`, `in`, or `out`. Several direction/amount pairs may be supplied. |
| `!cam home` | Smoothly return to the authored home position and base zoom. The camera also returns home after one minute without camera input. |
| `!resetcam` | Restore the authored town camera. |
| `!rulervote` | Start a ruler vote. |
| `!resign` | Resign as ruler. |
| `!save` | Save the town. |

## Moderator and game-master commands

These are administrative commands. Game-master access is granted only to
Twitch user IDs explicitly configured in Stream Town’s Twitch settings.

| Command | Purpose |
|---|---|
| `!modrole <player> <role>` | Change another player’s role. |
| `!event <event>` | Trigger a supported town event. |
| `!qevent <event>` | Queue a supported event. |
| `!stopevent` | Stop the active event. |
| `!tbuildcosts` | Toggle building costs. |
| `!trolelimits` | Toggle role limits. |
| `!addresource <resource> <amount>` | Add or remove town resources. |
| `!kill <player>` | Kill a player. |
| `!grevive <player>` | Game-master revive. |
| `!givexp <player> <amount>` | Give experience to one player. |
| `!givexpall <amount>` | Give experience to every player. |
| `!levelup <player> [amount]` | Grant role levels. |
| `!givepet <player> <pet>` | Unlock a pet for a player. |
| `!cobj` | Complete the first active technology objective. |
| `!randtech` | Start an eligible technology goal. |
| `!techvote` | Start a technology vote. |
| `!gaction` | Perform the active game-master event action. |
| `!unlockall` | Unlock all reachable technologies. |
| `!unlockage2` | Unlock the technologies leading into Age 2. |
| `!resetid <kind> <value>` | Validate/reset a supported stable-ID sequence. |
