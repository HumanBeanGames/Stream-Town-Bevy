# Stream Town Bevy

This workspace is the engine-independent rewrite of Stream Town. The Unity
project at the repository root is frozen migration input; new runtime work lives
here.

## Binaries

- `stream_town_game`: shipping Bevy application and an explicit 300-agent
  vertical-slice benchmark.
- `stream_town_tools`: focused content, migration, world-generation, navigation,
  Twitch, validation, and runtime tooling. Its catalog browser, validated
  role and technology editors with shared undo/redo, multi-layer terrain,
  foliage, occupancy, resource, and path lab, prefab/archetype browser,
  material/texture and Animator-controller diagnostics, and Twitch
  device-OAuth/vault diagnostics are functional. Its Runtime tab can launch or
  attach to the game, inject validated commands, request save/load/frame
  capture, and inspect stable world/session/profiling status.
- `stream_town_migrate`: Unity metadata/YAML inventory and validated legacy-save
  conversion.
- `xtask`: repository validation and repeatable developer automation.

The Technology tab is an interactive editor rather than only a record inspector.
It renders all 363 nodes, 20 groups, and cross-group edges on a pannable/zoomable
canvas; supports node and group dragging, group resizing, search highlighting and
focus, fit-all, a clickable minimap, and deterministic auto-layout; and retains
the existing node/group CRUD plus prerequisite, unlock, objective, icon, and
metadata editing. Catalog and stable-ID layout edits share undo/redo. Both
`catalog.ron` and `technology_layout.ron` are validated, backed up, atomically
written, reloaded, and compared before success is reported.

The Game Authority tab edits the source-controlled simulation configuration and
can also write a machine-local runtime override. The Roles tab covers every
persisted balance, station, target, ability, animation, and equipment binding.
The World + Nav tab edits the Unity-scale terrain inputs and every converted
foliage noise layer, then renders elevation, occupancy, resource, and foliage
previews using the production deterministic generator. See
[`AUTHORING.md`](AUTHORING.md) for paths and the safe-save workflow.

## Commands

```powershell
cd bevy-port
cargo xtask validate
cargo test --workspace
cargo run -p stream_town_game
cargo run -p stream_town_tools
cargo run -p stream_town_tools -- --validate-authoring
cargo run -p stream_town_migrate -- inventory .. --out generated/content-manifest.json
cargo run -p stream_town_migrate -- validate-unity-export generated/unity-export.json
cargo run -p stream_town_migrate -- convert-content generated/unity-export.json --unity-root .. --out-dir assets/content
cargo run -p stream_town_migrate -- convert-technology-layout ../Assets/Scripts/TechTree/Editor/Graphs/TechTreeV2Graph.asset assets/content/catalog.ron --out assets/content/technology_layout.ron
cargo run -p stream_town_migrate -- validate-models assets/migrated/models/model-conversion.json --repository-root .. --expected-count 253
cargo run -p stream_town_migrate -- convert-main-menu-reference ../generated/main-menu-reference.json --out generated/main-menu-scene.ron
cargo run -p stream_town_migrate -- bake-main-menu-scene generated/main-menu-scene.ron --config assets/config/game.ron --content assets/content/catalog.ron --out assets/content/main_menu_scene.ron
cargo run -p stream_town_migrate -- import-save StreamTownSave.stsave --out generated/imported.stbevy --config assets/config/game.ron
cargo run -p stream_town_migrate -- export-world-oracle StreamTownSave.stsave --out generated/unity-world-oracle.json
cargo run -p xtask -- package-windows --output dist
```

Run the deterministic in-game foliage acceptance sweep (stationary plus moving
and zooming camera, exact-transform duplicate audit, and shadow-policy audit)
with:

```powershell
.\scripts\capture-foliage-acceptance.ps1
```

The ignored output contains twelve full-resolution frames, a machine-readable
manifest, and an MP4 when `ffmpeg` is available.

Run the complete curated GPU acceptance matrix with:

```powershell
.\scripts\capture-visual-acceptance.ps1
```

The runner performs twelve independent fresh-process launches at 1920x1080
and waits for the real scene-reveal gate before timing each capture. It covers
the Main Menu, town overview, Settings, game menu, build menu, ruler vote,
current event, foliage, generated shoreline, live character animation, the
converted Giraffe pet follower, and Credits. `xtask visual-acceptance` downsamples
the results to the checked 320x180 references, verifies their SHA-256 integrity,
and applies a scenario-specific mean-error budget. Use `-Scenario shoreline`
for a focused run. Only use `-UpdateBaseline` after manually reviewing a full
capture matrix; partial baseline replacement is rejected.

The audio acceptance gate is part of `cargo test -p stream_town_game`. It
regenerates all 39 deterministic replacement waveforms, compares their exact
SHA-256 fingerprints with `assets/acceptance/audio-baseline.json`, and rejects
inaudible output or static-like sample discontinuities. `xtask validate` checks
both visual and audio baseline coverage and integrity.

On Windows, `scripts\launch-tools.ps1` launches the authoring suite from any
working directory. Pass `-Release` for an optimized build or `-ValidateOnly`
for a headless authoring-data check.

The tools Runtime tab launches the game with an opt-in atomic control directory
at `.stream-town/runtime-console`. A separately launched game can be attached by
setting `STREAM_TOWN_RUNTIME_CONSOLE=1`, or both processes can share an explicit
`STREAM_TOWN_RUNTIME_CONSOLE_DIR`. The channel carries stable IDs, actions, and
status only; Twitch OAuth tokens remain exclusively in the operating-system
credential vault. Normal game launches do not read or write console files.

The Windows packager builds optimized game and tools executables, bundles the
validated runtime assets, README, GPL license, replaceable LGPL
FFmpeg/OpenH264 DLLs, exact corresponding source archives, vcpkg
recipes/patches, SPDX metadata, and relinking instructions, validates safe
archive paths and required files, and atomically writes
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
level curves, health, retained defense data, movement, carry, resource-affinity, station/target
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
the four transform-free clips, plus all twelve authored animation events, and
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
Legacy Unity saves are never modified by migration tools. `import-save` is an
optional one-time compatibility command only. The production generator never
loads a Unity save: `export-world-oracle` emits sanitized counts and position
hashes solely for offline tests that compare generated output with ground truth.

Player preferences are written atomically to `.stream-town/settings.ron` with a
backup. On first launch, the Windows game imports Unity's
`Documents/Panda Belly/Stream Town/SettingsData.json` when available; set
`STREAM_TOWN_PLAYER_SETTINGS_PATH` or `STREAM_TOWN_UNITY_SETTINGS_PATH` to use
explicit paths. The Settings tab in `stream_town_tools` and the shipping Main
Menu/in-game Escape panel edit and validate the native file. The runtime panel
extends Unity's authored Video, Audio, Gameplay, and Connection shell with a
focused Accessibility tab, pointer controls, Apply, Defaults, Back, and the
unsaved-draft confirmation. Value rows keep stable UI entities while a draft is
edited, so changing one audio gain updates only that readout instead of rebuilding
and flashing every row.
The Connection tab reports runtime Twitch status while OAuth and secret storage
remain in the focused tools application.
Window mode/resolution, VSync/FPS limit, MSAA/post-process AA,
shadows/shadow-map size, SSAO, brightness/gamma, four independent audio gains,
all camera controls/sensitivities, name/building-health overlays, the Unity
0/5/10/30/60-minute autosave choices, UI scale, high contrast, and reduced
motion are applied by the runtime. Schema-2 settings upgrade without changing
their existing appearance; the three new accessibility fields use neutral
defaults. See [`docs/accessibility.md`](docs/accessibility.md) for keyboard and
screen-reader controls and the manual Windows Narrator acceptance procedure.
On Bevy 0.19, the saved Unity `Fullscreen` choice is implemented as borderless
fullscreen compatibility. This mode is selected before DX12 surface creation,
avoiding Bevy's missing-current-monitor panic and driver-dependent device loss
from switching into exclusive fullscreen after renderer initialization.
Because the Unity repository contains no redistributable soundtrack files, the
runtime synthesizes four seasonal day/night music beds and an in-game-only,
seamless band-limited ambient wind/bird loop. Unity's separate master, music,
ambience, and sound-effect gains,
fade behavior, day/night and season reselection, and deterministic 600–900 second
inter-track waits are preserved without adding licensed media.

The cloud material also filters its high-frequency authored noise at subpixel
sizes, matching Unity's generated mipmaps and preventing menu/credits shimmer.

Twitch is disabled in the checked-in configuration. The tools application writes
public settings to `.stream-town/config.ron`; OAuth access and refresh tokens are
stored only in the operating-system credential vault. The game validates and
proactively refreshes the token before starting IRC, revalidates hourly, rebuilds
the IRC connection after token rotation, and keeps the Unity broadcaster
`!connect` safety gate. The Twitch tab can verify a real channel join, resolve
operator logins to stable numeric IDs, and capture a live Channel Points reward
ID. See [`TWITCH_SETUP.md`](../TWITCH_SETUP.md).
On Windows, the same Twitch tab can separately authorize the configured channel
with `channel:read:stream_key` and enable direct broadcasting. Bevy render
readback feeds a bounded in-process H.264 encoder, WASAPI application loopback
captures only Stream Town's Bevy and Bevy Tidal audio, and FFmpeg's FLV muxer
publishes directly to Twitch RTMP. No OBS installation, desktop capture, virtual
audio device, `ffmpeg.exe` subprocess, serialized stream key, or unbounded frame
queue is involved. Automatic reconnect and runtime-console counters expose the
encoder, ingest, video drops, and audio progress. The independently stored
broadcaster token and fetched stream key are always redacted.
The encoder worker owns the constant-rate video clock and repeats the latest
completed GPU frame if the game thread stalls. Audio starts against the first
video frame and continues on its 48 kHz capture clock, preventing loading work
from advancing audio while leaving video timestamps behind.
The protected Main Menu > Secrets screen also reports credential presence, live
bot/command-gate state, and the direct encoder phase with advancing media-frame
counts. Its Restart stream control reapplies the visible settings and rebuilds
the in-process Twitch connection without restarting the game. Save and apply
restarts only a connection whose client ID or login actually changed; a no-op
save preserves the live bot gate, `!connect` authorization, and broadcast worker.
Unity-compatible game-master commands use a separate explicit list of numeric
Twitch user IDs. Broadcaster/moderator status alone never grants those cheats;
local `STREAM_TOWN_DEBUG_COMMANDS` injection retains Unity's debug-bridge bypass.

In-game gameplay interaction is text-command only. Keyboard, edge, and
middle-mouse panning are disabled, pointer selection is opt-in for future
automatic-camera work, and the pointer is hidden until Escape opens the game
menu or one of its Settings children. Mouse-wheel and Q/E zoom remain
view-only controls; F12 remains a diagnostic screenshot shortcut. Gameplay uses
the shipping `MainCamera.prefab` contract directly: a 60-degree perspective
lens with 0.3/1000 clipping planes, the authored 45-degree downward view from
the town's negative-X side and physical 11-60 height zoom. The pose is translated to the generated
Town Hall so different deterministic terrain seeds retain Unity's opening
composition. Use
arrow keys and Enter to select Save Game, Load Game, Settings, Exit Game, or
Idle Mode. On the Main Menu and Credits, Tab/Shift+Tab moves
focus, arrow keys continue from visible keyboard focus, and Enter or Space
activates the focused control. Settings retain keyboard control: Tab/Shift+Tab changes category,
arrow keys select or change values, Enter confirms, and Escape invokes the same
unsaved-change prompt as Back. The Main Menu's `S` shortcut opens its keyboard
menu, from which Settings uses the same workflow. The
stable chat grammar executes the player query, role/station/target selection,
cosmetic/pet, building/catalog, recruit administration, ruler economy, camera,
governance, moderation, save, and event commands documented by `!help` with
catalog/prerequisite validation and HUD/Twitch feedback.

For an automated legacy-load smoke test, set `STREAM_TOWN_SAVE_PATH` to an
imported `.stbevy` file and `STREAM_TOWN_AUTO_LOAD=1`. A retained schema-1
terrain mesh is validated during conversion and native-save reads, rebuilt as a
Bevy render mesh and Avian collider on load, and preserved by later native saves.

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
The IRC parser also retains Unity's complete `!create`/`!join` typo-alias set,
returns the source-authored usage for malformed registered commands, and routes
source-compatible player, global, and silent responses instead of applying one
generic bot prefix to every command.
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
World-generator schema 6 fingerprints these target identities, exact Unity
source-space offsets, fish nodes, and generated-resource navigation occupancy.
Every seed, including the shipping seed, is generated independently by the Bevy
domain code. Its Unity-compatible path reproduces the seeded .NET random stream,
Mono floating-point expression boundaries, Unity's native `Mathf.PerlinNoise`,
min/max normalization, terrain curve and island quantization, traversal order,
habitat tests, and shared occupancy. Unity-generated terrain samples, candidate
coordinates, and save placements are never generator inputs; output-only hashes
can fail parity tests but cannot supply world data. Land nodes block their cell,
workers act from the nearest walkable edge, and depletion clears the cell through
a dirty-region update only after the last overlapping node is depleted.
Every shipping generated resource starts at Unity's
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
regeneration, movement, and carrying use the reachable converted level curves
plus role-specific unlocked technology percentages. Serialized defense curves
remain inspectable but do not reduce melee or projectile damage because neither
shipping Unity attack helper consumes `PlayerRoleData.DamageReduction`.
`!experience` reports the active role's current level and XP threshold.
Like Unity's `PlayerInventory`, each resource keeps an independent carried
amount across role changes. Capacity, carry props, and carry animation use only
the current role's resource, and depositing transfers only that bucket rather
than silently unloading resources retained from earlier roles.
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
The shipping Fish God event is also live: its configurable channel-point reward
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

The vertical slice renders the deterministic 200x200 navigation height field as
a single continuous 636,804-vertex voxel-style terrain mesh, a water surface at
the authored level, an Avian
trimesh collider used for spatial queries, lighting, converted GLB scenes for
the representative town hall and actors, and primitive fallbacks when an asset
is unavailable. Actors, resources, buildings, movement, joins, save restores,
and retained programmatic selection outlines use the same centimetre height data
as navigation. The selection model keeps stable actor and grid IDs, complete
rotated building footprints, terrain-following outline geometry, and authoritative
actor/building/resource detail resolution for future automatic-camera work. It
is cleared across load/new-world boundaries rather than entering save data. The
shipping game does not schedule the pointer picker or expose selection actions.
The generated heightfield uses a Bevy PBR material extension whose WGSL port
reconstructs the Unity terrain shader's authored sand/grass height blend, grid
texture, palette, and tint controls. Runtime-generated terrain now follows the
shipping Unity scale: 200x200 samples, two-unit cells, half-unit height
quantization, the authored terrain curve and island falloff, and globally
normalized multi-octave noise generated from Unity-compatible `System.Random`
offsets. It is emitted as one mesh and one full-resolution Avian collider, as
Unity's `ProceduralMeshGenerator` did. Removing independently shaded and
LOD-switched render chunks eliminates visible cracks and lighting changes at
former chunk boundaries without changing navigation or save hashes. Schema-1
saves with an explicit Unity terrain mesh retain that mesh as one exact legacy
surface. A second PBR
extension ports the reachable water material's shallow/deep colors, animated
dual-noise wind, foam controls, transparency, and winter ice pattern. A
terrain-matched 47,089-vertex water mesh carries deterministic depth into the
shader and extends eight cells beyond the island. Water uses energy-conserving
bloom and a bounded stylized shader output instead of direct PBR highlights in
the +1.1 EV/ACES path, preventing the coastline from clipping to white. The
Unity 0-10 foam cutoff is normalized before sampling the averaged dual-noise
field, foam color is applied once, and seasonal color remains a tint rather than
replacing the authored cyan surface. Exact
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
regenerates stable instances from the source layer sizes, seeds, thresholds,
spacing, normalized octave noise, fixed base scale, and quarter-turn rotations.
Resources use their three shipping generation layers and 100-unit amounts.
Unity's shared generation-occupancy keys are preserved exactly: valid
half-cell candidates and same-cell clusters are not collapsed, while subsequent
foliage layers still reject occupied source keys. Raw glTF primitive loads restore
Blender's omitted 0.01 centimetre-to-metre scene-node conversion before applying
authored scale; this prevents tree, ore, bush, grass, and coral primitives from
becoming roughly one hundred times too large and overlapping. Runtime resource
and foliage presentation derives a second deterministic offset from the world
seed, stable generated identity, source location, and source sub-cell position.
Each axis is constrained to the central 50% of its navigation cell, so the
result is visibly distributed without drifting into neighboring gameplay cells,
using entity IDs, or reading legacy-save coordinates. The converted
grass, flower, seaweed, and coral primitives use Bevy's native mesh/material
instancing: the 16,581 shipping records collapse to 12 maximum GPU mesh batches
and 281 deterministic 32-cell spatial audit groups without duplicating geometry.
A camera-relative 96-unit ground-cover range with an 18-unit dither band streams
sub-pixel distant instances while preserving their authoritative generation
records, transforms, PBR lighting, and shadows. Current building and enemy-camp
footprints hide intersecting
foliage, and the visibility is derived again after removal or save load so stale
clearings cannot leak between world states. Resource trees use a typed
`TreeMaterial` WGSL port with the authored atlas, world-synchronized vertex
wind, per-object color variation, and spring/autumn/winter controls. The Blender
pipeline promotes Unity's FBX `colorSet1` masks to glTF `COLOR_0`, preserving
the red wind, green snow, and blue bark-exclusion channels Bevy consumes. Tree,
bush, and grass materials retain visible wind deformation. Grass keeps its
existing depth path; tree visible and shadow vertices now import one identical
bind-group-free deformation function, replacing both the mismatched approximate
prepass and the later undeformed shadow silhouette. Trees and bushes cast and
receive ordinary shadows and remain PBR-lit; only the older unsynchronized
Grass/Critter paths retain self-shadow suppression. Object-stable palette
hashing separately prevents blue palette flashes. The synchronized lit result
has local before/after evidence but still awaits user confirmation. Do not
broaden or repeat the previous shadow/material changes without consulting
[`docs/visual-regressions/tree-foliage-flicker.md`](docs/visual-regressions/tree-foliage-flicker.md),
which records the failed approaches and the next isolation matrix.
Missing converted assets retain the resource-cube fallback.

Content schema 21 also preserves each building prefab's authored base maximum
health and per-level health increase. Construction starts at the Unity-authored
ceil of ten percent and building work clamps to the current level's
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
`STREAM_TOWN_SMOKE_BUILDING_VFX=1` frames construction smoke, the
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
`STREAM_TOWN_SMOKE_STATIC_RIG=1` frames the unanimated shipping Player rig, and
`STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP=1` frames the converted-controller diagnostic.
The runtime uses `Characters.glb`'s single authoritative armature and matching
renderer variants instead of applying animation curves to the TPose export's
nine independent skins. The full translated Animator controller keeps its
authored states, layers, transitions, blends, speeds, and actions, while each of
its 20 motions now resolves to the matching native take on that same visible
armature. `STREAM_TOWN_DEBUG_ANIMATION_BINDINGS=1` logs consecutive-frame clip
time, representative joint rotation deltas, exact `SkinnedMesh` joint
references, and the inherited-visible model slots. The runtime now controls
every equipment-shaped GLB node, including inactive source-only defaults that
are absent from the Unity role list, so `Body_Default_*` and the commander
banner cannot overlap the selected role body. Follow
[`docs/visual-regressions/character-animation.md`](docs/visual-regressions/character-animation.md)
before changing clip priority or retargeting. Player skins cast ordinary world
shadows and receive self/world shadows through the typed character material.
Standard player surfaces are promoted into that material through a shared
cache, preserving imported textures, vertex colours, alpha state, and cosmetic
recolouring. A character-only receiver normal offset removes the unstable body
facets without changing the global light bias or the synchronized foliage
path. The loading reveal verifies every initial animated-player mesh has the
compatible receiver, has no `NotShadowReceiver`, and owns a GPU-ready material
before exposing the world.
`STREAM_TOWN_DEBUG_PLAYER_BOUNDS=1` logs settled actor world bounds for
repeatable axis and retargeting checks.
`STREAM_TOWN_SMOKE_OVERLAYS=1` frames the Town Hall and starting actors while
temporarily forcing all player-name and building-health overlays visible; it
does not modify the saved player settings.
The developer status block is hidden in shipping presentation by default;
set `STREAM_TOWN_RUNTIME_DIAGNOSTICS=1` when its route, simulation, and input
telemetry is needed during local debugging.
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
prefab/model material overrides as the diagnostic scene. Content schema 33 also
retains the shipping prefab's world-space 1–5 metre follow band, zero-to-10
metre-per-second speed remap, local `+Z` visible-forward convention, and model
child transforms rather than applying a runtime cell-size multiplier. Fish
God's authored 1.403-metre child lift and all five unit-scale model roots are
validated from the converter output.
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
`STREAM_TOWN_DEBUG_AGE_TWO=1` unlocks the authored Town Hall age upgrade for a
repeatable presentation smoke without modifying production configuration.
`STREAM_TOWN_DEBUG_CARRY=1` equips the converted Player smoke actor as a Logger
with one wood so the independent Top additive layer enters its authored Carry
state deterministically.
`STREAM_TOWN_AUTOSTART_CREDITS=1` opens the Credits state directly, and
`STREAM_TOWN_DEBUG_CREDITS_TIME=<seconds>` starts its authored timeline at a
specific point for repeatable property-curve smoke captures.
`STREAM_TOWN_AUTOSTART_SETTINGS=1` opens the authored settings shell on state
entry so all five categories and the complete runtime draft can be captured
repeatably.
`STREAM_TOWN_AUTOSTART_GAME_MENU=1` opens the shipping in-game menu after world
loading so its Save, Load, Settings, Exit Game, Close, and Idle Mode controls
can be captured without pointer or keyboard automation.
`STREAM_TOWN_EXIT_AFTER_SCREENSHOT=1` exits one second after an automatic frame
capture so GPU smoke runs can terminate without an external process killer.
Compatible embedded GLB clips and translated standalone clips share Bevy
animation graphs. The nine shipping enemy prefabs resolve their authored rig and
full controller clip sets; the Player controller binds all 20 reachable motions
to skin-compatible takes on the single `Characters.glb` armature. An engine-independent interpreter evaluates
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
The shipping `Custom/CharacterSimple` surface used by Eyes and Hair now resolves
to a typed Bevy material and WGSL port of `_characterTexture * _albedoColor`.
Actor-specific hair, facial-hair, and eye colours update that typed uniform just
as Unity's `MaterialPropertyBlock` did; inherited glTF fallback materials are
promoted to the same material before recolouring, so converted model bindings
cannot bypass customization.
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
Set `STREAM_TOWN_PERFORMANCE_REPORT_PATH` to write the same gate plus terrain,
foliage instance/native-GPU-batch/spatial-group/fallback, actor-detail, and
crowd-separation counters as JSON. `STREAM_TOWN_BENCHMARK_FOLIAGE=0` is an
explicit diagnostic A/B that omits only generated ground cover; it is never
enabled by ordinary gameplay. Set
`STREAM_TOWN_EXIT_AFTER_FRAME_TIME=1` for an unattended run that exits once the
report is complete. Generated terrain uses one continuous authored-style render
mesh and one full-resolution collider, so terrain LOD boundaries cannot crack.
The 300-agent gate fixes its default scene/animation detail budgets to the
recorded 16/16 reference while ordinary gameplay retains 64 detailed actors;
both remain explicitly overrideable. Foliage uses scale-aware fade ranges and
native GPU instancing, and deterministic local crowd
separation changes only presentation transforms, never navigation or saves.
`cargo run -p xtask -- stress --agents 300 --ticks 3600` performs the matching
one-minute-at-60-Hz CPU soak while repeatedly mutating dirty navigation cells.
Windows builds pin wgpu to the self-contained DX12 path. This avoids the
driver-specific Vulkan swapchain failures observed on the reference machine.
Bevy 0.19's motion-blur pass is disabled on Windows because its varying loop
does not compile with FXC; the rest of the authored post-processing stack stays
enabled, and packaged builds need no Vulkan SDK or loose compiler DLL.
The current Windows release reference run (Ryzen 5 7600X, Radeon RX 7800 XT,
64 GB, DX12, 1920x1080) measured 600 post-warmup frames at 11.47 ms average and
16.50 ms p95 with 300 simulated agents, the single continuous 200x200 terrain
surface, all 16,581 generated ground-cover records, 12 native GPU mesh groups,
and the benchmark's 16-character detail budget, below the 16.7 ms gate.
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
record also covers the synthesized ambience. Seasonal music now comes directly
from the revision-pinned public native Rust
[`bevy_tidal`](https://github.com/HumanBeanGames/bevy-tidal) library and the
eight authored expressions in
[`assets/music/patterns`](assets/music/patterns); the renderer runs in-process
without TidalCycles, SuperCollider, or a sidecar and applies the player's
master/music gain to the active season/day-night pattern. The reachable town
seagull now uses its converted GLB, exact 32-second cross-town flight contract,
three generated calls on the source's random 1–5 second cadence, and authored
ambience rolloff. Its converted +X nose axis receives the handedness-corrected
-90-degree Bevy yaw, so the complete flock faces its flight direction.
`Env_Grass` now uses a dedicated WGSL material preserving its
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
fallback. A shipping-scene source audit found no additional reachable custom
shader or VFX prefab to port; the remaining shader/VFX assets belong to
Necrolands or development test scenes.
Property curves
support Unity's constant, unweighted Hermite, and weighted Bezier segments; the
shipping catalog currently contains 261 unweighted keys. The Credits panels/end
fade, fireworks activation, and live level-up toast consume the converted
float-property curves directly. The reachable `vfx_fireworks` graph is converted
to engine-neutral launch/burst rates, capacities, ranges, delays, and its eight
HDR gradient colors, while both exact Credits scene placements are retained.
At the Unity-authored 56-second key, a deterministic Bevy UI particle runtime
launches rockets, delayed flashes, and radial sparks instead of displaying the
earlier text-glyph placeholder. The scene's authored bottom-right `SKIP` button
uses the packaged sliced `UI_Button` art and returns to the Main Menu through the
same state transition and cleanup path as Escape.
Presentation schema 17 also converts the reachable `VFX_Chimney_Smoke`
particle prefab and all seven authored placements across House, Forge,
Stonemason, Barracks, and Fishing Hut models. Completed buildings emit
deterministic local-space smoke from the exact age-specific chimney locations,
using Unity's five-particles-per-second rate, five-second lifetime, cone, size,
color, and fade settings; incomplete or destroyed buildings do not emit.
The Fish God event's `VFX_RainingFish` built-in mesh particle system is also
converted rather than approximated by cubes: the catalog retains its Fish3
model and Critters material, 500-per-second authored emission, 15-second
lifetime, 0.2-1.0 size range and size curve, 300x300x5 box, gravity, noise,
bounce, collision lifetime loss, prewarm, and 5,000-particle ceiling. The Bevy
renderer deterministically samples that logical effect into a 320-fish visual
budget, preserving the authored coverage and timing while keeping the 300-agent
performance gate bounded.
Presentation schema 18 replaces the earlier hand-tuned healing visuals with two
typed Unity VFX Graph records. `VFX_Healing_Channeling` retains its 32-particle
capacity, 16-per-second rate, 1-3 second particle lifetime, prefab size override,
four-key five-second curve, and HDR color/alpha gradient. `VFX_healing` retains
its 1.2-second disable contract, eight-capacity/100-count plus burst, converted
`VFX_Plus.glb`, 128-capacity/one-count disc, both particle-age size curves, the
two prefab HDR gradients, and `Particle_02` texture provenance. Bevy samples
those budgets, curves, and gradients into short-lived ECS effects and uses the
converted plus mesh when assets are available.
Presentation schema 20 converts the shipping `Fish.prefab` field instead of
omitting it. The catalog retains its Fish3 mesh, Critters material, zero start
speed, 120-second lifetime, 40-per-second logical emission, 0.2-1.0 size range,
rotated 300x300x5 spawn box, two-octave scrolling noise, velocity alignment, and
world-space/prewarm flags. Only the two effectively active scene bindings are
retained: the Main Menu's 800-particle field and the town's 2,000-particle field.
The second town YAML instance is excluded because its parent prefab is inactive.
Bevy deterministically prewarms 160 shared-mesh representatives per binding,
samples the box once, and applies bounded smooth noise rather than turning the
box extents into an orbit.
Use `STREAM_TOWN_AUTOSTART=1` with `STREAM_TOWN_SMOKE_FISH_SCHOOL=1` for the
focused town capture.

All Unity color properties consumed by custom WGSL/PBR materials are converted
from sRGB to linear values before upload. This keeps terrain, water, grass,
building detail/emission, placement bounds, and flags within the authored
+1.1 EV/ACES daytime range instead of washing the world into yellow-white.

The same schema retains all 35 Unity role-action `AudioClip` GUIDs across the
14 roles that authored them. The ten reachable audio events now choose an
original variant deterministically and play smooth role-specific procedural WAV
cues spatially at the actor, using Unity's 20-unit enable distance and the live
master/SFX mix. No missing recording is redistributed. Use
`STREAM_TOWN_AUTOSTART=1` with `STREAM_TOWN_SMOKE_ROLE_AUDIO=1` to keep the
camera/listener near the starting Logger while workers enter authored actions.
The two remaining events are the model-importer `ToggleOn`/`ToggleOff` pair on
`CharacterFishing`; their normalized times become seconds on conversion and
show the carry prop only during the catch window before returning to the normal
inventory-on-return-walk rule.
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
disabled sprite and cannot activate until a native save exists. Its background
is no longer a synthetic blank stage: the migration exporter resolves the
authored `Main_Menu_02` camera, 285 model instances, and its 4,900-vertex island
mesh into checked RON. The converter reflects camera and instance positions,
rotations, mesh vertices/normals, and triangle winding together into Bevy's
right-handed coordinates, preserving the left UI/right town composition. Bevy
reconstructs that scene with the converted GLBs and restores the town camera when
gameplay loading begins.
The landscape is a schema-3 corrective bake produced once by the ordinary
deterministic world generator, not by reading a Unity save. The bake weights
generated height by the authored shoreline mask, flattens building foundations,
lifts model instances, and writes generated resource/foliage placements into
`main_menu_scene.ron` for load-only runtime use. Re-run the documented
`bake-main-menu-scene` command from an unbaked schema-2 reference whenever
generator configuration intentionally changes. Corrective-bake version 3
samples every foundation from the untouched generated surface before flattening;
this prevents the dense farm/wall layout from propagating one artificial plateau
through its neighbors. It applies a presentation-only 3x vertical multiplier so
the unchanged generator's half-metre terraces remain visible from the side-on
menu camera; the seed, generator hash, horizontal positions, 2,565 resources,
and 12,392 foliage records remain unchanged.
Runtime frustum filtering now schedules every visible baked decoration instead
of sampling away most trees and grass. Foliage resolves the layer's authored
material (including `Env_Grass`), and menu/world construction uses a three
millisecond plus count ceiling per update so the loading UI keeps presenting
while the complete scene is assembled.
The menu uses a -1.5 EV scene baseline at the neutral brightness setting, a
fixed-depth translucent water material over a uniform ocean floor, and 21
non-shadowing rectangular-prism clouds. Fully submerged checker-terrain
triangles are omitted from the menu render so one flat water plane cannot expose
two conflicting floor colours. Menu-only trees and bushes use stable instance
colouring and no long-range wind deformation; gameplay foliage keeps authored
wind and moving shadows. Completed building and farm variants remain visible
while construction stages and inactive crop-growth meshes are hidden.

The checked bake remains the full parity/validation source, but the static menu
deterministically renders at most 900 visible resources and 3,200 visible foliage
instances to bound startup work. The 186 repeated completed farms bypass generic
scene spawning and load their exact Base/Full GLB primitives. Boot preloads the
same `Scene(0)` handles later consumed by the menu, and the loading overlay stays
up until every remaining scene root has children. New Game keeps that already
rendered cover on a dedicated SDR UI camera while the town camera changes from
the menu projection to the HDR gameplay pipeline. The camera and cover survive
the `MainMenu` -> `WorldLoading` -> `InGame` boundary as one operation, then
retire together after a fully prepared gameplay frame; leaving the no-clear UI
camera alive would retain its last loading frame over the world. Progress is now
the recursive aggregate of observable work leaves: loading-cover entities and
artwork, asset I/O, five deterministic generator stages, ECS construction,
scene-root instantiation, material overrides, actual animation-controller
attachment, lighting receivers, GPU image/mesh/material uploads, pipeline and
selection-draw readiness, and stable presented frames. There are no fixed
percentage bands or synthetic 99% cap. The exact aggregate reaches 100% only
when every leaf is complete, then presents that completed frame before removal.
Simulation, gameplay input, Twitch command application, autosave,
ambience, seagull calls, and Tidal music share a `GameplayReady` system gate.
That marker is absent throughout reveal validation and is inserted only after
both loading runtimes and every loading overlay/camera entity have left the ECS
world, so no town time or sound can advance behind the cover.
Diagnostic logs report boot
asset time and menu-scene reveal time separately; on the recorded DX12 debug
machine asset I/O is about 0.4s while first-use scene/material GPU preparation is
still about 12s and is the dominant remaining startup cost.

The in-game HUD uses the shipping top-bar artwork rather than a full-width debug
text block. Its dark/gold background, food/gold/ore/wood icons, player/building/
play-time counters, and four-season gauge are loaded from the converted catalog
and updated from authoritative ECS state. Optional runtime diagnostics remain
available for migration testing without becoming an interactive HUD.

The authoritative object-selection model remains available for future automatic
camera targeting. Pointer selection is not enabled in the shipping game, so its
selection window and outline remain dormant unless another system selects an
actor, resource, building, or cell programmatically.

The Unity Build, Recruit, and Technology bottom bar is intentionally omitted.
Building placement, recruitment, and technology voting are reached exclusively
through the stable chat grammar and its typed command queues.

Active technology and governance votes use the shipping voting-menu art.
Technology votes show their converted icon, live approval share, total votes,
countdown, and a text-command reminder. Ruler elections and
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

`STREAM_TOWN_SMOKE_NEW_GAME_TRANSITION=1` exercises the real cold Main Menu New
Game handoff instead of the direct autostart shortcut. An optional
`STREAM_TOWN_SMOKE_NEW_GAME_DELAY_SECONDS` delay lets external frame recorders
attach after the menu is stable without changing the production button path.

Loading animation remains in Bevy's main/render schedule because window events,
UI extraction, and GPU presentation cannot safely be driven from an independent
application thread. CPU-only world generation, terrain/water mesh construction,
and terrain collider construction run on `AsyncComputeTaskPool`. Main-thread ECS
construction yields after bounded batches (32 menu descriptors, 96 world
resources, or 192 world foliage descriptors), so every batch can be extracted
and presented before loading continues. Runtime logs report both construction
updates and actually presented render frames for cold-load diagnostics.

The migration now has source-diff closure for reachable gameplay and balance,
shipping-scene shader/VFX reachability, converted authoring data and assets,
curated visual/audio acceptance contracts, and automated runtime/package gates.
The repository-level [`MIGRATION_STATUS.md`](../MIGRATION_STATUS.md) records the
few remaining external release-certification constraints.
