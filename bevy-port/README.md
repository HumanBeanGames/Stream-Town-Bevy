# Stream Town Bevy

This workspace is the engine-independent rewrite of Stream Town. The Unity
project at the repository root is frozen migration input; new runtime work lives
here.

## Binaries

- `stream_town_game`: shipping Bevy application and the 300-agent vertical slice.
- `stream_town_tools`: focused content, migration, world-generation, navigation,
  Twitch, validation, and runtime tooling. Its catalog browser, validated
  technology editor with undo/redo, occupancy/path lab, prefab/archetype browser,
  material/texture and Animator-controller diagnostics, and Twitch
  device-OAuth/vault diagnostics are functional; the runtime panel remains a
  diagnostic shell.
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
  shooter, and 422 typed objectives from the production technology graph in content schema 17,
  and follows nested prefabs to
their source FBX models. Those effects comprise 28 building unlocks, 177 level
caps, 104 role/global stat boosts, 80 building-cost reductions, 12 storage
boosts, and 12 building-age upgrades.
The same command copies all 133 reachable
textures and emits `presentation.ron`: 33 material definitions with 141 retained
Unity vector/color shader parameters and 32 texture transforms, 75 clip records,
31 controller definitions, 94 stable states, 166 transitions, and inherited
prefab/controller/model bindings. The YAML fallback converts 57 standalone
`.anim` files into 1,196 stable transform tracks. Presentation schema 11 also
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
record.

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
geometry, rigs, and translation animation curves.

The first native save is written to `.stream-town/StreamTownSave.stbevy`.
Legacy Unity saves are never modified by migration tools.

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
save/load, F12 to capture a screenshot, and Escape to return to the menu. The
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

The Unity starting NPC roster is present as stable Defender, Logger, Miner,
Gatherer, and Builder actors. Resource roles select their authored resource and
the nearest matching generated node, gather using `BaseActionAmount`, carry the
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
Enemy camps are placed on a deterministic valid edge site because the shipping
Unity camp-generation setting list is empty and its former placement loop is
commented out. At night, the camp applies Unity's day/player population cap,
three-second authored cadence, weighted Goblin/Blargul/Goblin Boss selection,
and four converted spawn offsets. `!event raid` disables normal camp spawning
and starts the Unity-authored five-wave Minotaur raid: 50 tracked enemies must
be defeated before each next wave, followed by a Minotaur Boss whose health is
at least 1,000 and otherwise scales by 50 per player. Enemy archetype, camp
timers and members, wave progress, tracked enemies, and the next stable enemy ID
survive native saves; legacy enemy and camp names resolve back to catalog
archetypes.
The shipping Fish God event is also live: its exact Unity channel-point reward
ID and `!praise` feed the same deterministic command path, with Unity's one-in-ten
summon chance, 20-praise requirement, 300-second timeout, 1,000-food reward, and
70% Fish God pet roll across joined Twitch players. `!event fish_god` provides a
forced moderator/debug start. Event attempts, remaining time, praise progress,
rewarded pet IDs, and resources persist in native saves. Live player pets use
converted GLBs: subscribers receive and auto-equip Red Panda, while Gatherer,
Fisher, and Logger actions retain Unity's deterministic one-in-5,000 Giraffe,
Duck, and Butterfly unlock chance. `!pet` switches unlocked pets and each pet
follows its owning actor. The converted Fish God GLB appears at town center while a deterministic falling-fish field provides the
missing Unity VFX Graph equivalent; HUD and tools expose event progress and
repeatable injection commands. Initial actors are
placed by a deterministic flood fill from the connected town centre so combat
and work targets are reachable rather than stranded on isolated land cells.

The vertical slice renders the deterministic navigation height field as a colored
4,225-vertex terrain mesh, a water surface at the authored level, an Avian
trimesh collider used for surface picking, lighting, converted GLB scenes for
the representative town hall and actors, and primitive fallbacks when an asset
is unavailable. Actors, resources, buildings, movement, joins, save restores,
and selection markers use the same centimetre height data as navigation.
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
mesh. Ore uses the shared building material. Content schema 17 converts Unity's
two land and two underwater foliage-generation layers, including their noise,
threshold, seed, LOD, scale, material, and 21 FBX variant references. Bevy
regenerates stable land/underwater instances from the world seed, excludes
resource cells, and renders the converted grass, flower, seaweed, and coral
primitives with deterministic jitter, rotation, scale, and a 420-unit
visibility budget. Resource trees use a typed
`TreeMaterial` WGSL port with the authored atlas, world-synchronized vertex
wind, per-object color variation, and spring/autumn/winter controls. The Blender
pipeline promotes Unity's FBX `colorSet1` masks to glTF `COLOR_0`, preserving
the red wind, green snow, and blue bark-exclusion channels Bevy consumes.
Missing converted assets retain the resource-cube fallback.

For repeatable rendering diagnostics,
`STREAM_TOWN_DEBUG_DAY=21` selects a starting day and
`STREAM_TOWN_DEBUG_WEATHER=snow` temporarily overrides simulated weather.
`STREAM_TOWN_DEBUG_BUILDING_HEALTH=125` starts the Town Hall at that clamped
health value so its per-building damage material can be captured repeatably.
`STREAM_TOWN_SMOKE_RESOURCE_CLOSEUP=1` focuses on the nearest wood node;
`STREAM_TOWN_SMOKE_RESOURCE_KIND=resource:ore` or `resource:food` selects
another production resource. `STREAM_TOWN_DEBUG_INITIAL_AGENTS=<n>` reduces
diagnostic actor clutter without changing the validated production default.
`STREAM_TOWN_SMOKE_HEALING_VFX=1` frames deterministic channel, healed-burst,
and revival cues together for a repeatable gameplay-VFX capture.
`STREAM_TOWN_SMOKE_COMBAT_VFX=1` retriggers the four typed impact/trail styles
and frames the converted Arrow GLB for a repeatable combat-VFX capture.
`STREAM_TOWN_SMOKE_BUILDING_VFX=1` frames construction/repair smoke, the
spark-emitting construction hit, building-level arrows, and persistent
damage fire/smoke together. Enemy `TargetSensor` masks are authoritative in
content schema 17: ordinary enemies choose the nearest allowed actor/building,
while the battering ram attacks only construction and buildings. Zero-health
buildings are removed and release their dirty navigation region.
`STREAM_TOWN_SMOKE_FOLIAGE=1` frames the generated foliage field for a
repeatable land/shoreline visual capture.
`STREAM_TOWN_SMOKE_SHORELINE=1` finds and frames the nearest generated
land/water boundary for a repeatable depth-blend and edge-foam capture.
`STREAM_TOWN_DEBUG_AGE_TWO=1` unlocks the authored Town Hall age upgrade for a
repeatable presentation smoke without modifying production configuration.
`STREAM_TOWN_DEBUG_CARRY=1` equips the converted Player smoke actor as a Logger
with one wood so the independent Top additive layer enters its authored Carry
state deterministically.
`STREAM_TOWN_AUTOSTART_CREDITS=1` opens the Credits state directly, and
`STREAM_TOWN_DEBUG_CREDITS_TIME=<seconds>` starts its authored timeline at a
specific point for repeatable property-curve smoke captures.
`STREAM_TOWN_EXIT_AFTER_SCREENSHOT=1` exits one second after an automatic frame
capture so GPU smoke runs can terminate without an external process killer.
Compatible embedded GLB clips use Bevy animation graphs (currently the shipping
Goblin path). The Player controller builds 19 converted clips retargeted onto 23
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

`STREAM_TOWN_REPORT_FRAME_TIME=1` enables the explicit 300-agent GPU benchmark,
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
[`assets/audio/PROVENANCE.md`](assets/audio/PROVENANCE.md). Rare/non-gameplay action
emitters and the remaining reachable WGSL shader ports remain presentation
work. Property curves
support Unity's constant, unweighted Hermite, and weighted Bezier segments; the
shipping catalog currently contains 261 unweighted keys. The Credits panels/fireworks/end fade and
the live level-up toast consume the converted float-property curves directly.
Main Menu and Credits also recreate the reachable `VFX_Clouds` prefab's 21
stacked built-in planes directly in Bevy. A typed WGSL material consumes its
authored texture and exact dual world-space time offsets, cutoff, tint, and
surface exponent; this deliberately preserves the shipping material's extreme
`_CloudTint=200` rather than silently normalizing its look.

This is an early migration milestone, not a parity release. The repository-level
[`MIGRATION_STATUS.md`](../MIGRATION_STATUS.md) lists implemented behavior and
remaining work.
