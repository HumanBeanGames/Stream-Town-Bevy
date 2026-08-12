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
```

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
  shooter, and 422 typed objectives from the production technology graph in content schema 15,
  and follows nested prefabs to
their source FBX models. Those effects comprise 28 building unlocks, 177 level
caps, 104 role/global stat boosts, 80 building-cost reductions, 12 storage
boosts, and 12 building-age upgrades.
The same command copies all 133 reachable
textures and emits `presentation.ron`: 33 material definitions, 75 clip records,
31 controller definitions, 94 stable states, 166 transitions, and inherited
prefab/controller/model bindings. The YAML fallback converts 57 standalone
`.anim` files into 1,196 stable transform tracks. Presentation schema 8 also
retains 110 component/UI property curves with 261 keys across 18 clips, including
the four transform-free clips, plus all ten authored animation events. It retains
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
  cosmetic/pet, building/catalog, recruit administration, ruler economy,
  camera, governance, moderation, save, and event commands documented by `!help`
  with catalog/prerequisite validation and
HUD/Twitch feedback. `!build` starts a Unity-style per-player placement preview
at the last successful position, `!move`/direction aliases and `!rotate` adjust
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
spawn visible homing ECS projectiles. Priests select the nearest injured player,
heal on their authored cadence, and release full-health targets. Completed Towers
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
Seasons and weather drive terrain/water tint, clear color, directional and
ambient lighting, distance fog, and deterministic rain/snow fields from the
authoritative simulation. For repeatable rendering diagnostics,
`STREAM_TOWN_DEBUG_DAY=21` selects a starting day and
`STREAM_TOWN_DEBUG_WEATHER=snow` temporarily overrides simulated weather.
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
Live gather, construction, combat, and healing goals feed the converted Player
controller's authored role trigger, `Action`, deterministic `AnimationIndex`,
and Unity-remapped `ActionSpeed`; locomotion, carry props, death, and revival use
that controller contract as well. Presentation schema 4 preserves the 32
shipping states with active float speed parameters and applies their authored
base-speed multiplier during playback. Presentation schema 6 also preserves all
45 state machines and 33 layers; the runtime follows conditioned child-machine
entries and parent defaults/exits. Independent layer runtimes share live parameters
and route their clips through Bevy override/additive graph nodes with Unity-correct
effective weights. The three AvatarMask assets retain 477 stable transform weights
(118 exclusions), and excluded bones are assigned to Bevy animation mask groups.
Unity fixes layer zero to weight one; higher layers use their serialized default,
so the shipping Character Top layer continues to evaluate its carry state machine
but correctly has zero pose influence. The ten converted `PlayRoleActionAudio`
events dispatch once per animation cycle from Bevy's monotonic clip clock and play
short deterministic procedural cues; their no-sample provenance is documented in
[`assets/audio/PROVENANCE.md`](assets/audio/PROVENANCE.md). Rare/non-gameplay action
emitters, exact weighted Unity tangent semantics, and custom WGSL shader parity
remain presentation work. The Credits panels/fireworks/end fade and
the live level-up toast consume the converted float-property curves directly.

This is an early migration milestone, not a parity release. The repository-level
[`MIGRATION_STATUS.md`](../MIGRATION_STATUS.md) lists implemented behavior and
remaining work.
