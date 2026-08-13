# Stream Town Bevy

This workspace is the engine-independent rewrite of Stream Town. The Unity
project at the repository root is frozen migration input; new runtime work lives
here.

## Binaries

- `stream_town_game`: shipping Bevy application and an explicit 300-agent
  vertical-slice benchmark.
- `stream_town_tools`: focused content, migration, world-generation, navigation,
  Twitch, validation, and runtime tooling. Its catalog browser, validated
  technology editor with undo/redo, occupancy/path lab, prefab/archetype browser,
  material/texture and Animator-controller diagnostics, and Twitch
  device-OAuth/vault diagnostics are functional. Its Runtime tab can launch or
  attach to the game, inject validated commands, request save/load/frame
  capture, and inspect stable world/session/profiling status.

The Technology tab supports searchable group/node authoring rather than only
inspection: create or delete groups and nodes, move nodes between groups, edit
prerequisites and metadata, validate cycles/dangling references, undo/redo, and
save the complete content catalog atomically. Catalog saves preserve a `.bak`
copy and are reloaded and revalidated before success is reported.
- `stream_town_migrate`: Unity metadata/YAML inventory and validated legacy-save
  conversion.
- `xtask`: repository validation and repeatable developer automation.

## Commands

```powershell
cd bevy-port
cargo xtask validate
cargo test --workspace
cargo run -p stream_town_game
cargo run -p stream_town_tools
cargo run -p stream_town_migrate -- inventory .. --out generated/content-manifest.json
cargo run -p stream_town_migrate -- validate-unity-export generated/unity-export.json
cargo run -p stream_town_migrate -- convert-content generated/unity-export.json --unity-root .. --out-dir assets/content
cargo run -p stream_town_migrate -- validate-models assets/migrated/models/model-conversion.json --repository-root .. --expected-count 253
cargo run -p stream_town_migrate -- import-save StreamTownSave.stsave --out generated/imported.stbevy --config assets/config/game.ron
cargo run -p xtask -- package-windows --output dist
```

The tools Runtime tab launches the game with an opt-in atomic control directory
at `.stream-town/runtime-console`. A separately launched game can be attached by
setting `STREAM_TOWN_RUNTIME_CONSOLE=1`, or both processes can share an explicit
`STREAM_TOWN_RUNTIME_CONSOLE_DIR`. The channel carries stable IDs, actions, and
status only; Twitch OAuth tokens remain exclusively in the operating-system
credential vault. Normal game launches do not read or write console files.

The Windows packager builds optimized game and tools executables, bundles the
validated runtime assets, README, and GPL license, validates safe archive paths
and required files, and atomically writes
`dist/stream-town-windows-x86_64.zip`. The tools Validation tab can launch the
same repository validator and release-packaging jobs. CI publishes the validated
ZIP as a branch artifact after the full test job passes.

Generate the ignored neutral Unity export from the repository root with the
exact editor version recorded by the project:

```powershell
.\bevy-port\scripts\export-unity.ps1
cd bevy-port
cargo run -p stream_town_migrate -- validate-unity-export generated/unity-export.json
cargo run -p stream_town_migrate -- convert-content generated/unity-export.json --unity-root .. --out-dir assets/content
```

The editor exporter resolves GUIDs, object references, prefab sources and
overrides, ScriptableObject data, and the four shipping scene hierarchies. Its
migration-only A* types are inert compile stubs and are not navigation code.
The content conversion selects the active Unity containers and emits a validated
catalog of 26 production buildings, 215 prefab archetypes, 288 model scene
variants, 15 roles, and the 363-node shipping technology graph. It derives
building footprints from Unity's authored two-unit grid sizes, emits typed build
and level costs, `Placeable`, `CanLevel`, per-level multipliers, and all 413
authored technology effects plus every shipping role's action, XP multiplier,
level curves, health, defense, movement, carry, resource-affinity, station/target
  masks, all 15 equipment sets, all four reachable building storage components,
  all 11 reachable building role-slot modifiers,
  all 42 reachable prefab health definitions, all nine pooled enemy combat definitions,
  the authored Goblin camp weights and spawn transforms, the Tower's consolidated projectile
  shooter, the Marketplace's authored level-scaled passive gold generator, all
  nine authored enemy kill rewards, all 42 reachable construction/upgrade model
handlers, all six storage-fill model handlers, and 422 typed objectives from the production
technology graph, all three authored prefab rotors, the heal-burst prefab's
authored self-disable lifetime, and the player prefab's three-second damaged-health-bar
hide contract in content schema 30. The same schema promotes the shipping loader's
50%-per-second display rate, half-second completion hold, and tooltip catalog, and
follows nested prefabs to
their source FBX models. Those effects comprise 28 building unlocks, 177 level
caps, 104 role/global stat boosts, 80 building-cost reductions, 12 storage
boosts, and 12 building-age upgrades.
The same command copies all 133 reachable
textures and emits `presentation.ron`: 33 material definitions with 141 retained
Unity vector/color shader parameters and 32 texture transforms, 184 clip records,
31 controller definitions, 94 stable states, 166 transitions, and inherited
prefab/controller/model bindings. The YAML fallback converts 57 standalone
`.anim` files into 1,196 stable transform tracks. Presentation schema 15 also
maps 122 embedded FBX takes to stable model-GUID/local-ID clip records and their
exact GLB animation indexes. It
retains 110 component/UI property curves with 261 keys across 18 clips, including
the four transform-free clips, plus all ten authored animation events, and
fixed/normalized duration plus destination offset for all 166 transitions. It retains
11 authored 1D blend states and
typed transition conditions; the stale `Slam` and `Swipe` conditions become
provenance-marked inferred parameters. Renderer inheritance resolves to 141
prefab material bindings and 181 material dependencies. Presentation schema 7
additionally resolves 241 model-importer material names plus 912 authored slots
across 903 prefab renderers, preserving distinct materials on multi-primitive
GLB meshes. The known missing Necrolands camera clip is an explicit validated
record. The same schema converts the two reachable global URP volume profiles
and all three shipping scene bindings. Runtime mapping applies the authored
bloom, vignette, motion blur, ACES tonemapping, exposure, and the town's
inverse-daylight night color grade while retaining player brightness/gamma.
An HDR post-process pass applies the authored RGB filter directly in WGSL after
tonemapping instead of approximating the blue night tint with white balance.

Convert all FBX models with the pinned Blender version, then validate every
source/output hash and GLB header:

```powershell
.\bevy-port\scripts\convert-models.ps1
cd bevy-port
cargo run -p stream_town_migrate -- validate-models assets/migrated/models/model-conversion.json --repository-root .. --expected-count 253
```

Converted GLBs are reproducible package inputs under `assets/migrated/models`.
They are versioned with Git LFS; reports and Blender logs remain generated files.
The converter uses exported Unity renderer bounds to bake model units into
geometry, rigs, and translation animation curves. It exports every imported FBX
take as a named GLB animation rather than retaining only the active action.

The first native save is written to `.stream-town/StreamTownSave.stbevy`.
Legacy Unity saves are never modified by migration tools.

Player preferences are written atomically to `.stream-town/settings.ron` with a
backup. On first launch, the Windows game imports Unity's
`Documents/Panda Belly/Stream Town/SettingsData.json` when available; set
`STREAM_TOWN_PLAYER_SETTINGS_PATH` or `STREAM_TOWN_UNITY_SETTINGS_PATH` to use
explicit paths. The Settings tab in `stream_town_tools` and the shipping Main
Menu/in-game Escape panel edit and validate the native file. The runtime panel
uses a draft workflow with apply/save, restore-defaults, and cancel actions.
Window mode/resolution, VSync/FPS limit, MSAA/post-process AA,
shadows/shadow-map size, SSAO, brightness/gamma, four independent audio gains,
all camera controls/sensitivities, name/building-health overlays, and the Unity
0/5/10/30/60-minute autosave choices are applied by the runtime.
Because the Unity repository contains no redistributable soundtrack files, the
runtime synthesizes four seasonal day/night music beds and a continuous ambient
wind/bird loop. Unity's separate master, music, ambience, and sound-effect gains,
fade behavior, day/night and season reselection, and deterministic 600–900 second
inter-track waits are preserved without adding licensed media.

Twitch is disabled in the checked-in configuration. The tools application writes
public settings to `.stream-town/config.ron`; OAuth access and refresh tokens are
stored only in the operating-system credential vault. The game validates or
refreshes the token before starting IRC, revalidates hourly, and keeps the Unity
broadcaster `!connect` safety gate. See [`TWITCH_SETUP.md`](../TWITCH_SETUP.md).
Unity-compatible game-master commands use a separate explicit list of numeric
Twitch user IDs. Broadcaster/moderator status alone never grants those cheats;
local `STREAM_TOWN_DEBUG_COMMANDS` injection retains Unity's debug-bridge bypass.

In game: use WASD to pan, Q/E to zoom, left-click to select a grid cell,
J to inject a parsed `!join`, F1/F2 to disconnect/reconnect Twitch, F5/F9 to
save/load, F12 to capture a screenshot, and Escape to open the game menu. Use
arrow keys and Enter to select Resume, Save, Load, Settings, or Main Menu. The
Main Menu's `S` shortcut opens the same keyboard-driven settings workflow. The
stable chat grammar executes the player query, role/station/target selection,
cosmetic/pet, building/catalog, recruit administration, ruler economy, camera,
governance, moderation, save, and event commands documented by `!help` with
catalog/prerequisite validation and HUD/Twitch feedback.

For an automated legacy-load smoke test, set `STREAM_TOWN_SAVE_PATH` to an
imported `.stbevy` file and `STREAM_TOWN_AUTO_LOAD=1`. A retained schema-1
terrain mesh is validated during conversion and native-save reads, rebuilt as a
Bevy render mesh and Avian collider on load, and preserved by later F5 saves.

`!build` starts a Unity-style per-player placement preview at the last successful
position, `!move`/direction aliases and `!rotate` adjust
the exact grid cell and retained 90-degree rotation, `!confirm`/`!accept` spends
schema-4 resources and commits valid occupancy, and `!cancel` exits without
spending. `!level <building> <id> [times]`, `!levelall`, and `!remove` use
one-based per-building-type IDs. Placed rotation, occupancy, station/target
geometry, and last player placement round-trip through native saves; the legacy
importer retains authored building Y rotation. Confirmed structures spawn the
converted building GLB with a primitive fallback. New
structures start at Unity's 10% construction health; Builder agents path to a
reachable perimeter cell and advance the 33%/66% presentation stages to
completion. Upgrades spend the authored level cost and multiplier and respect
maximum levels granted by unlocked technology. Unlocked technologies also apply
their authored placement/upgrade discounts, expand storage-building
contributions, modify actor health/movement/action/combat percentages, and select
age-two GLB variants for constructed buildings and the always-present Town Hall.
The Town Hall has stable authoritative level/health state, is upgradeable by the
same commands, and round-trips through native saves; old saves synthesize this
state during load. Building placement also respects
the converted Unity `Placeable` flag and the
persisted technology set: the authored initial technologies expose Lumbermill,
  Stonemason, Tower, and Windmill, while later votes expose the buildings named by
  their `Unlock Building` effects. Winning votes with authored objectives now
  start persistent town goals; gathering deposits, construction, combat, buying,
  and selling update HUD-visible progress before the technology unlocks. A semicolon-delimited
`STREAM_TOWN_DEBUG_COMMANDS` value can inject the same path
for repeatable diagnostics.

The registered Unity game-master surface is also live: `!tbuildcosts`,
`!trolelimits`, `!addresource`, `!kill`, `!grevive`, `!givexp`, `!givexpall`,
`!levelup`, `!givepet`, `!qevent`, `!stopevent`, `!cobj`, `!randtech`,
`!techvote`, `!gaction`, `!unlockall`, `!unlockage2`, and `!resetid`. Build-cost
and role-limit toggles plus unique queued events persist in native saves.
Technology administration chooses stable catalog order instead of Unity's
process-global random state. `!resetid` reports the stable-ID state because Bevy
does not have Unity's pooled per-type counter to repair. `!stdiscord` preserves
the remaining registered no-character utility command.
The numbered-list forms of `!rid`, `!station`, and `!target` also project their
matching IDs over live world objects for Unity's authored 15-second lifetime.

Ruler governance follows the shipping Unity rules: the first election is
scheduled after 30 seconds, ballots wait indefinitely for the first vote and
then run for 120 seconds, each joined player votes once, and elected rulers face
an hourly yes/no retention vote. Ruler identity, previous role, active ballot,
tallies, and cooldown survive native saves; the legacy importer restores ruler
names and vote cooldowns. Election wins assign `role:ruler`, while replacement
or `!resign` restores the prior role. `!buy`, `!sell`, `!recruit`, `!recruits`,
and `!save` enforce ruler-or-staff access, while `!rulervote` and forced `!event`
remain broadcaster/moderator commands. Recruiting creates stable NPC entities,
consumes the House-backed recruit capacity, and persists normally.

New towns start with exactly Unity's stable five-NPC roster: Defender, Logger,
Miner, Gatherer, and Builder. The 300-agent crowd is confined to explicit tests
and benchmark mode, and no bootstrap Goblin is invented. Resource roles select
their authored resource and
the nearest matching generated node and exact target mask: trees, ore, bushes,
and reachable shoreline fish no longer collapse into interchangeable resource
targets. Farmers harvest the completed Farm's authored unlimited food holder,
while Fishers approach invisible water targets from a walkable shore cell.
World-generator schema 4 fingerprints these target identities, fish nodes, and
generated-resource navigation occupancy. Land nodes block their cell, workers
act from the nearest walkable edge, and depletion clears the cell through a
dirty-region update. Every shipping generated resource starts at Unity's
hard-coded 100 units (`SetByDistance` is false). Schema-1 through schema-3 native
saves are hash-verified during load, preserve their existing depletion, and add
newer world features without discarding saves.
Generated nodes also reproduce Unity's zero-assignment claim rule: deterministic
stable-ID ordering gives each active node one worker, other workers fail over to
the next compatible target, and claims release on depletion, death, role change,
or retargeting. Farms retain station-controlled multi-worker capacity.
Workers gather using `BaseActionAmount`, carry the
authored 10-unit `BaseMaxResource`, then path back to the Town Hall and deposit
into the town balances shown by the HUD. Successful actions award Unity-scaled
role XP; per-role progress survives role changes and saves, levels follow the
Unity curve through level 99, and action amount/cadence/range, health and
regeneration, defense, movement, and carrying use converted level curves plus
the unlocked technology percentages. `!experience` reports the active role's
current level and XP threshold.
Food, ore, and wood start with Unity's 15,000-unit capacity, recruit capacity
starts at five, and gold is unbounded. Completed Food/Ore/Wood Storage buildings
add the authored level-scaled capacity; Houses add recruit slots. A capped
deposit leaves overflow on the actor until spending or new construction creates
space.
Node depletion and carried inventories are part of native save/load state.
Combat roles acquire a living target and path into authored range. Melee roles
apply deterministic damage directly; Necromancer, Ranger, and Wizard attacks
spawn visible homing ECS projectiles with distinct violet, converted-arrow, and
orange fireball presentation. Towers share the converted Arrow GLB and Unity's
two-second tapering grey trail. Melee hits and projectile arrivals emit the
authored 0.25-second physical burst or a typed fire/necrotic variant. Converted
enemy target masks select valid players and buildings; zero-health buildings
are removed and release their navigation region. Priests select the nearest
injured player, heal on their authored cadence, and release full-health targets. Successful
heals emit the Unity-authored 1.2-second target burst plus a purpose-built
five-second green channel field using the serialized 0/0.289/1/0 size curve.
Automatic, paid, and game-master revives emit a larger gold-accented variant.
Completed Towers
launch their converted one-damage projectile every three seconds at the nearest
enemy within 10 cells. Players trigger Death at zero health and use Unity's
authored 60-second automatic revival. `!revive` pays 400 food for self-revival;
Priests and Paladins can pay 200 food to revive another stable Twitch actor and
receive role XP. Pending revival time and actor health survive native saves.
New towns do not create an enemy camp: Unity's procedural placement loop is
commented out and the four shipping scenes contain no camp instance (the prefab
appears only in the disabled Necrolands scene). Camp simulation/presentation is
retained for imported legacy/native saves. The clock uses Unity's shipping
3,600-second day, 66.6% daylight
boundary, and 100-second dusk/dawn transitions; its 10/5 day/night light values
drive sun, ambient, sky, and building emission, and day/season progression uses
the same clock. When a restored camp exists, at night it applies Unity's day/player population cap,
three-second authored cadence, weighted Goblin/Blargul/Goblin Boss selection,
and four converted spawn offsets. `!event raid` disables normal camp spawning
and starts the Unity-authored five-wave Minotaur raid: 50 tracked enemies must
be defeated before each next wave, followed by a Minotaur Boss whose health is
at least 1,000 and otherwise scales by 50 per player. Enemy archetype, camp
timers and members, wave progress, tracked enemies, and the next stable enemy ID
survive native saves; legacy enemy and camp names resolve back to catalog
archetypes.
Native load also repositions the persistent Town Hall ECS root, presentation
origin, rotation, and lower-left grid location from the saved footprint. Its
rendered position therefore stays aligned with navigation occupancy, worker
stations, selection, combat targeting, and legacy-imported transforms.
Actor restore uses the same player-only completed-gate exception as live pathing.
When a saved actor must otherwise move off a blocked cell, the relocated grid
position is written to both the ECS agent and authoritative simulation state.
The shipping Fish God event is also live: its exact Unity channel-point reward
ID and `!praise` feed the same deterministic command path, with Unity's one-in-ten
summon chance, 20-praise requirement, 300-second timeout, 1,000-food reward, and
70% Fish God pet roll across joined Twitch players. `!event fish_god` provides a
forced moderator/debug start. Event attempts, remaining time, praise progress,
rewarded pet IDs, and resources persist in native saves. Live player pets use
converted GLBs: subscribers receive and auto-equip Red Panda, while Gatherer,
Fisher, and Logger actions retain Unity's deterministic one-in-5,000 Giraffe,
Duck, and Butterfly unlock chance. `!pet` switches unlocked pets and each pet
follows its owning actor using Unity's distance-squared speed remap, ten-unit
speed cap, and smoothed facing. Red Panda, Giraffe, Duck, and Butterfly use
their own translated Unity `MoveSpeed` controllers and retargeted idle/walk
clips; the unanimated Fish God keeps its authored static model. The converted
Fish God GLB appears at town center while a deterministic falling-fish field provides the
missing Unity VFX Graph equivalent; HUD and tools expose event progress and
repeatable injection commands. The event presentation also plays the converted
`FishGod` controller's Belly Flop entry, loops its idle, consumes Unity's `Exit`
trigger, and retains the Jump exit for the authored 2.5-second removal delay.
Initial actors are
placed by a deterministic flood fill from the connected town centre so combat
and work targets are reachable rather than stranded on isolated land cells.
Their visible roots also reproduce the reachable `RotationHandler` contract:
movement turns smoothly at the authored five-radians-per-second rate, action
states face their actor, resource, or building target, and gathering keeps
Unity's explicit immediate snap toward the resource.

The vertical slice renders the deterministic navigation height field as a colored
4,225-vertex terrain mesh, a water surface at the authored level, an Avian
trimesh collider used for surface picking, lighting, converted GLB scenes for
the representative town hall and actors, and primitive fallbacks when an asset
is unavailable. Actors, resources, buildings, movement, joins, save restores,
and selection outlines use the same centimetre height data as navigation. The
live picker uses the converted `SelectionMask` texture and authored emissive
alpha-cutout material on a terrain-following plane; actor outlines preserve
Unity's 1.5-unit collider and 1.25 scale, while building selections cover their
full rotated footprint. Drag selection recreates Unity's local-recruit group
selection with one outline per member, mass role assignment, confirmed mass
dismissal, and compatible right-click orders for stations, enemies, and
resources. Transient selection stores stable actor IDs and is cleared across
load/new-world boundaries rather than entering save data. The same action
surface supports single-recruit role changes and confirmed dismissal, plus
selected-building level-up and confirmed removal through the authoritative
cost, level-cap, navigation, station, and target cleanup transactions; the Town
Hall remains protected. Role choices now follow Unity's live `RoleSlot`
availability contract: full and zero-slot roles are omitted, building-granted
roles such as Blacksmith appear when their station contributes capacity, and
mass changes fill the remaining slots deterministically. Enemy selection
resolves the converted enemy type, and
enemy camps are selectable across their complete authored footprint with live
health, maximum-health, and correctly sized outline feedback. Player selection
restores distinct live health and role-experience meters (including level caps
and XP requirements), building details show authored maximum levels, and finite
resources retain Unity's amount-only presentation instead of inventing a cap.
Selected marketplaces also report their level-scaled hourly output, while
building level-up controls live-refresh from the authoritative technology cap,
construction state, adjusted resource cost, and town inventory; unaffordable
upgrades are visibly disabled and cannot dispatch.
The generated heightfield uses a Bevy PBR material extension whose WGSL port
reconstructs the Unity terrain shader's authored sand/grass height blend, grid
texture, palette, and tint controls; Bevy's configured waterline adapts that
blend to the deterministic replacement terrain generator. A second PBR
extension ports the reachable water material's shallow/deep colors, animated
dual-noise wind, foam controls, transparency, and winter ice pattern. A
terrain-matched 6,561-vertex water mesh carries deterministic depth into the
shader, producing the authored shallow/deep blend and animated edge foam along
the replacement terrain shoreline, with an eight-cell deep-water apron beyond
the island. Exact
prefab/model renderer bindings can also replace a loaded glTF primitive with a
typed material extension. The heavily reused shipping `Building.shader` now
preserves its authored base/detail texture sampling, red-channel ambient
occlusion and snow exclusion, green-channel metal/smoothness, blue-channel
emission, alpha-channel roof variation, texture transform, and destruction
threshold. Winter drives its snow controls; because the replacement terrain has
a different vertical datum, damage wear uses the same authored threshold on a
height-independent mask. Bevy clones that custom material once per stable
building, shares the clone across that building's primitives, drives
`_DestructionValue` from its authoritative health percentage, and releases the
asset when the building leaves the simulation. Seasons and weather drive terrain/water tint, clear color, directional and
ambient lighting, distance fog, and deterministic rain/snow fields from the
authoritative simulation. Generated wood, ore, and food nodes load the converted
`Env_Tree`, `Env_Ore`, and `Env_Bush` GLB primitives selected through the
versioned content and presentation catalogs. Deterministic grid parity selects
Unity's two tree and ore variants, while bushes retain Unity's duplicated first
mesh. Ore uses the shared building material. Content schema 21 converts Unity's
two land and two underwater foliage-generation layers, including their noise,
threshold, seed, LOD, scale, material, and 21 FBX variant references. Bevy
regenerates stable land/underwater instances from the world seed, excludes
resource cells, and renders the converted grass, flower, seaweed, and coral
primitives with deterministic jitter, rotation, scale, and a 420-unit
visibility budget. Current building and enemy-camp footprints hide intersecting
foliage, and the visibility is derived again after removal or save load so stale
clearings cannot leak between world states. Resource trees use a typed
`TreeMaterial` WGSL port with the authored atlas, world-synchronized vertex
wind, per-object color variation, and spring/autumn/winter controls. The Blender
pipeline promotes Unity's FBX `colorSet1` masks to glTF `COLOR_0`, preserving
the red wind, green snow, and blue bark-exclusion channels Bevy consumes.
Missing converted assets retain the resource-cube fallback.

Content schema 21 also preserves each building prefab's authored base maximum
health and per-level health increase. Construction starts at the Unity-authored
ceil of ten percent, building work and repairs clamp to the current level's
maximum, damage presentation uses that same ratio, and upgrades add health while
preserving the existing damage deficit. Older native saves are normalized to
the authored maximum when loaded.

Completed Marketplaces generate the converted 0.5 gold/second base rate. Each
level preserves both serialized Unity `OnLevelUp` callbacks, adding 0.5
gold/second in total. Fixed-point fractional progress is keyed by stable building
and resource IDs and round-trips through native saves.

Resource workers also mirror Unity's storage backpressure: they stop acquiring
and gathering from nodes while the role's technology-expanded town storage is
full, retain any carried overflow, and resume automatically when capacity opens.

For repeatable rendering diagnostics,
`STREAM_TOWN_DEBUG_DAY=21` selects a starting day and
`STREAM_TOWN_DEBUG_WEATHER=snow` temporarily overrides simulated weather.
`STREAM_TOWN_DEBUG_BUILDING_HEALTH=125` starts the Town Hall at that clamped
health value so its per-building damage material can be captured repeatably.
`STREAM_TOWN_SMOKE_RESOURCE_CLOSEUP=1` focuses on the nearest wood node;
`STREAM_TOWN_SMOKE_RESOURCE_KIND=resource:ore` or `resource:food` selects
another production resource. `STREAM_TOWN_DEBUG_INITIAL_AGENTS=<n>` sets the
diagnostic actor count (up to 5,000) without changing the validated five-NPC
production default.
`STREAM_TOWN_SMOKE_HEALING_VFX=1` frames deterministic channel, healed-burst,
and revival cues together for a repeatable gameplay-VFX capture.
`STREAM_TOWN_SMOKE_COMBAT_VFX=1` retriggers the four typed impact/trail styles
and frames the converted Arrow GLB for a repeatable combat-VFX capture.
`STREAM_TOWN_SMOKE_BUILDING_VFX=1` frames construction/repair smoke, the
spark-emitting construction hit, building-level arrows, and persistent
damage fire/smoke together.
`STREAM_TOWN_SMOKE_CHIMNEY=1` spawns and frames a completed House so its
age-specific authored chimney position and looping local-space smoke can be
captured without issuing construction commands.
Enemy `TargetSensor` masks are authoritative in
content schema 21: ordinary enemies choose the nearest allowed actor/building,
while the battering ram attacks only construction and buildings. Zero-health
buildings are removed and release their dirty navigation region. Completed
gates are actor-aware passages: player routes may cross their occupied cells
while enemy routes remain blocked, matching the shipping Unity rule; gate
construction sites remain blocked to both sides. Their visual controllers are
live as well: age-one wood and age-two stone gates retarget the converted
`Open`/`Close` FBX takes onto the building model, preserve the Unity Animator's
quarter-second crossfade, and react only to living players inside the authored
4x4 trigger. `STREAM_TOWN_SMOKE_GATE=1` places a completed gate around the first
starting player and frames it for a deterministic opened-pose capture; combine
it with `STREAM_TOWN_DEBUG_AGE_TWO=1` to exercise the stone-root retarget.
The shipping `!ping` command now attaches the converted `PointerArrow.glb` and
red emissive `VFX_Pointer` material to the requesting actor by stable ID. It
follows living actors, evaluates the prefab's exact five-key Hermite size curve,
deduplicates repeated requests, and expires at the authored eight-second
duration without recreating Unity's general object pool. `STREAM_TOWN_SMOKE_PING=1`
queues the starting defender's pointer and frames it for deterministic capture.
`STREAM_TOWN_SMOKE_FOLIAGE=1` frames the generated foliage field for a
repeatable land/shoreline visual capture.
`STREAM_TOWN_SMOKE_SHORELINE=1` finds and frames the nearest generated
land/water boundary for a repeatable depth-blend and edge-foam capture.
`STREAM_TOWN_SMOKE_OVERLAYS=1` frames the Town Hall and starting actors while
temporarily forcing all player-name and building-health overlays visible; it
does not modify the saved player settings.
`STREAM_TOWN_SMOKE_FLAG=1` spawns and frames the converted Age 2 Castle so every
authored `Flag` renderer binding and the custom wind/color material can be
validated together.
`STREAM_TOWN_SMOKE_GODRAY=1` spawns and frames the converted Age 2 Necromancer
Tower so its exact `Env_Godrays_08` renderer binding and transparent material can
be validated together.
`STREAM_TOWN_SMOKE_GIRAFFE=1` spawns and frames the converted giraffe pet with
its exact `MainMaterial` binding. The typed material retains Unity's
vertex-colour neck/head masks, non-normalized rotation-axis math, textured PBR
surface, and Bevy skinning path. Live equipped pets now receive the same exact
prefab/model material overrides as the diagnostic scene.
`STREAM_TOWN_SMOKE_PET=red_panda`, `giraffe`, `duck`, `butterfly`, or `fish_god`
equips that pet on the starting defender and frames the live follow path. The
first four exercise translated animation controllers; Fish God verifies the
intentional static-model fallback.
`STREAM_TOWN_SMOKE_PLACEMENT=1` frames valid and blocked House placement bounds
at once. The live placer uses a typed port of Unity's transparent lit
`BoundsVisualiser` material, the serialized success/failure colours from
`BuildingPlacer.prefab`, and the authored `_Alpha` value.
`STREAM_TOWN_SMOKE_SELECTION=1` selects the Town Hall after world generation so
the image-backed selection window and authored footprint outline can be
captured without pointer automation.
`STREAM_TOWN_SMOKE_GROUP_SELECTION=1` selects all starting recruits so their
authored outlines and mass-action panel can be captured without drag input.
`STREAM_TOWN_SMOKE_BOTTOM_BAR=build`, `recruit`, or `technology` opens the
corresponding shipping bottom-bar context for repeatable UI captures.
`STREAM_TOWN_DEBUG_AGE_TWO=1` unlocks the authored Town Hall age upgrade for a
repeatable presentation smoke without modifying production configuration.
`STREAM_TOWN_DEBUG_CARRY=1` equips the converted Player smoke actor as a Logger
with one wood so the independent Top additive layer enters its authored Carry
state deterministically.
`STREAM_TOWN_AUTOSTART_CREDITS=1` opens the Credits state directly, and
`STREAM_TOWN_DEBUG_CREDITS_TIME=<seconds>` starts its authored timeline at a
specific point for repeatable property-curve smoke captures.
`STREAM_TOWN_AUTOSTART_SETTINGS=1` opens the settings overlay on state entry so
its complete runtime draft can be captured repeatably.
`STREAM_TOWN_EXIT_AFTER_SCREENSHOT=1` exits one second after an automatic frame
capture so GPU smoke runs can terminate without an external process killer.
Compatible embedded GLB clips and translated standalone clips share Bevy
animation graphs. The nine shipping enemy prefabs resolve their authored rig and
full controller clip sets; the Player controller builds 19 converted clips retargeted onto 23
bones in the Player GLB rest pose. An engine-independent interpreter evaluates
typed parameters, trigger consumption, direct transitions, exit gates, and 1D
threshold blending; runtime movement feeds the authored velocity/5 `Move Speed`
parameter into Idle/Walk/Run. Converted renderer descendants receive cached Bevy
PBR approximations of their Unity materials, including PNG/TGA base textures,
color, emission, metallic, smoothness, and alpha settings. Bevy glTF mesh and
material names select model-importer mappings and exact per-renderer overrides;
the inherited material is retained only as a compatibility fallback.
Player GLB descendants also apply persisted body, hair, facial-hair, eye,
hair-color, and eye-color selections in Unity's serialized order. Cosmetic
material variants are cached per source material and helmets suppress hair.
Converted animation clips and graphs are shared by rig/controller instead of
being rebuilt per actor. The runtime keeps 16 fully authored, animated character
scenes and represents the rest of a large crowd with lightweight capsule LODs;
all actors retain authoritative gameplay, movement, identity, selection, and
save state. `STREAM_TOWN_ACTOR_SCENE_BUDGET` and
`STREAM_TOWN_ANIMATION_BUDGET` override those diagnostic budgets. Twitch joins,
recruits, runtime enemies, and save-restored actors automatically promote from
the lightweight representation whenever detail capacity is available.
All 16 reachable Unity `EnemyModelHandler` records are typed content. Detailed
enemy scenes deterministically select one base and weapon plus independent
optional nodes from the persistent actor ID, keep permanent nodes visible, and
hide every unselected glTF node. The selected weapon supplies its authored
attack parameter, action-variant count, and generic/two-handed
`RunAnimationIndex`, so skeleton sword-and-shield and pole-weapon animation
contracts remain coupled across save/reload.
`xtask validate` parses the packaged GLB JSON chunks and rejects any content
catalog enemy-model reference whose exact node name was lost during conversion.

`STREAM_TOWN_REPORT_FRAME_TIME=1` raises startup to at least 300 agents and
enables the explicit GPU benchmark,
uses an unsynchronized present mode for meaningful capacity measurements, and
reports average and 95th-percentile frame time after a warmup. The optional
`STREAM_TOWN_FRAME_TIME_WARMUP` and
`STREAM_TOWN_FRAME_TIME_SAMPLE_SECONDS` values default to ten seconds each.
The current Windows reference run (Ryzen 5 7600X, Radeon RX 7800 XT, 64 GB,
DX12, 1920×1080) measured 514 post-warmup frames at 9.73 ms average and
12.45 ms p95 with 300 simulated agents, 457 deterministic foliage instances,
and the production 16-character detail budget, below the 16.7 ms gate.
Live gather, construction, combat, and healing goals feed the converted Player
controller's authored role trigger, `Action`, deterministic `AnimationIndex`,
and Unity-remapped `ActionSpeed`; locomotion, carry props, death, and revival use
that controller contract as well. Presentation schema 4 preserves the 32
shipping states with active float speed parameters and applies their authored
base-speed multiplier during playback. Presentation schema 6 also preserves all
45 state machines and 33 layers; the runtime follows conditioned child-machine
entries and parent defaults/exits. Independent layer runtimes share live parameters
and route their clips through Bevy override/additive graph nodes with Unity-correct
effective weights. State changes crossfade those graph outputs with the authored
timing and destination offset, retaining locomotion blend-tree proportions. The
three AvatarMask assets retain 477 stable transform weights
(118 exclusions), and excluded bones are assigned to Bevy animation mask groups.
Unity fixes layer zero to weight one; higher layers use their serialized default,
so the shipping Character Top layer continues to evaluate its carry state machine
but correctly has zero pose influence. The ten converted `PlayRoleActionAudio`
events dispatch once per animation cycle from Bevy's monotonic clip clock and play
short deterministic procedural cues; their no-sample provenance is documented in
[`assets/audio/PROVENANCE.md`](assets/audio/PROVENANCE.md). The same provenance
record covers the synthesized seasonal music and ambience. The reachable town
seagull now uses its converted GLB, exact 32-second cross-town flight contract,
three generated calls on the source's random 1–5 second cadence, and authored
ambience rolloff. `Env_Grass` now uses a dedicated WGSL material preserving its
vertex-color wind mask, scrolling main/noise textures, authored color blending,
and exact seasonal color/tint endpoints. `Critters` also has a dedicated
vertex-stage WGSL port: seagull and fish geometry samples the authored texture
mask through UV2 and applies the source speed/synchronization/stretch wave while
Bevy retains the textured PBR surface. The Age 2 Castle's `Flags` material also
uses a dedicated vertex/fragment WGSL port: vertex alpha anchors the cloth while
the authored scrolling noise and time-rotated displacement animate it; vertex
red blends yellow-to-red color and reduces the metallic/smoothness edge. The
Necromancer Tower's reachable `Env_Godrays_08` renderer now resolves its exact
`VFX_Godrays` material slot to a dedicated transparent, double-sided WGSL port.
It preserves the source mesh's vertex-color albedo, the authored 0.06 emission
strength, and 1.64 vertex-alpha multiplier instead of receiving a generic PBR
fallback. The remaining reachable WGSL shader ports remain presentation work.
Property curves
support Unity's constant, unweighted Hermite, and weighted Bezier segments; the
shipping catalog currently contains 261 unweighted keys. The Credits panels/end
fade, fireworks activation, and live level-up toast consume the converted
float-property curves directly. The reachable `vfx_fireworks` graph is converted
to engine-neutral launch/burst rates, capacities, ranges, delays, and its eight
HDR gradient colors, while both exact Credits scene placements are retained.
At the Unity-authored 56-second key, a deterministic Bevy UI particle runtime
launches rockets, delayed flashes, and radial sparks instead of displaying the
earlier text-glyph placeholder.
Presentation schema 16 also converts the reachable `VFX_Chimney_Smoke`
particle prefab and all seven authored placements across House, Forge,
Stonemason, Barracks, and Fishing Hut models. Completed buildings emit
deterministic local-space smoke from the exact age-specific chimney locations,
using Unity's five-particles-per-second rate, five-second lifetime, cone, size,
color, and fade settings; incomplete or destroyed buildings do not emit.
Main Menu and Credits also recreate the reachable `VFX_Clouds` prefab's 21
stacked built-in planes directly in Bevy. A typed WGSL material consumes its
authored texture and exact dual world-space time offsets, cutoff, tint, and
surface exponent; this deliberately preserves the shipping material's extreme
`_CloudTint=200` rather than silently normalizing its look. Both scenes now use
the packaged 2048x1227 `Game_Logo_DropShadow` sprite at its source aspect ratio
instead of substituting a text heading; assetless/headless runs retain a text
fallback. The Main Menu's reachable `UI_MainMenu.prefab` is reconstructed with
its tinted left-half panel, copyright line, and five packaged sliced-image
buttons in authored order. New Game, Load Game, Settings, Credits, and Quit are
mouse-operable while retaining their keyboard paths; Load Game selects the exact
disabled sprite and cannot activate until a native save exists.

The in-game HUD uses the shipping top-bar artwork rather than a full-width debug
text block. Its dark/gold background, food/gold/ore/wood icons, player/building/
play-time counters, and four-season gauge are loaded from the converted catalog
and updated from authoritative ECS state. The remaining runtime diagnostics and
keyboard hints occupy a compact bottom strip and stay available for migration
testing.

Selecting an occupied grid cell now opens a shipping-style selection window.
Actors, resource nodes, and every cell in a building footprint resolve against
authoritative runtime state; the packaged unfilled, green, and red slider art
shows live health or remaining resources. Selecting empty terrain keeps the
window hidden.

The permanent bottom bar reconstructs Unity's Build, Recruit, and Technology
buttons from packaged nine-slice, keybind, arrow, and icon artwork. Mouse clicks
or the original B/R/T shortcuts open a ten-item paged context. Entries preserve
Unity's serialized ordering and live enabled state, and dispatch through the
same typed building-placement, NPC recruitment, and technology-vote command
path used by Twitch rather than duplicating gameplay rules.

Active technology and governance votes use the shipping voting-menu art.
Technology votes show their converted icon, live approval share, total votes,
countdown, and a broadcaster-equivalent local vote button. Ruler elections and
retention votes show the Unity prompt, deterministic top-five/yes-no tally, and
the persisted 120-second countdown. `STREAM_TOWN_SMOKE_VOTE=technology|ruler|keep`
opens reproducible visual-acceptance fixtures without bypassing the real vote
state or timer systems.

Accepted technology votes that create objective-backed town goals now open the
shipping objective tracker. It renders the converted technology icon and title,
Unity-equivalent requirement labels, exact authoritative current/required
amounts, and the packaged objective progress slider; it rebuilds after native
save load and yields temporarily while a new technology ballot occupies the
same screen region. `STREAM_TOWN_SMOKE_GOAL=1` starts a reproducible real-catalog
goal fixture for visual acceptance.

Reachable Fish God and raid events now open Unity's shared current-event panel
using the three packaged CurrentEvent sprites. Fish God shows the live praise
command and exact praise count; raids show live enemies in the active wave and
completed-wave progress (not merely the spawned-wave index). Both rebuild from
native saved event state. `STREAM_TOWN_SMOKE_EVENT=fish|raid` opens deterministic
real-runtime fixtures for visual acceptance.

The automated headless acceptance path now traverses `Boot` -> `MainMenu` ->
`WorldLoading` -> `InGame`, joins a Twitch viewer through the production command
queue, saves and reloads the generated town, then enters and exits `Credits`
after its authored fireworks activation. It also guards state cleanup; this
found and fixed a stale Credits timeline resource that previously survived a
return to the Main Menu.

This is an early migration milestone, not a parity release. The repository-level
[`MIGRATION_STATUS.md`](../MIGRATION_STATUS.md) lists implemented behavior and
remaining work.
