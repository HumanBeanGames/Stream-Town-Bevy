# Bevy Migration Status

Last updated: 2026-08-12

The Unity project remains at the repository root as read-only migration input.
The new Rust workspace is in `bevy-port`. This document records delivered
behavior separately from planned parity so partially implemented tools cannot be
mistaken for production-ready systems.

## Delivered in this milestone

- A Bevy 0.19/Rust 1.95 workspace split into domain, game, tools, migration, and
  `xtask` crates.
- The `Boot`, `MainMenu`, `WorldLoading`, `InGame`, and `Credits` application
  states, with state-scoped entity cleanup.
- Validated, versioned RON configuration (schema 4) and stable authored/runtime
  IDs that do not expose Bevy entity identifiers. Gameplay configuration now
  carries Unity's 5,000-unit starting food/gold/ore/wood balances and zero
  recruits before roster creation, plus Unity's 15,000 food/ore/wood and five-recruit base capacities;
  gold remains intentionally unbounded.
- Deterministic island height generation, occupancy, A* routing, dirty regions,
  grounding data, repeatable world hashes, and a 4,225-vertex/8,192-triangle
  Bevy terrain surface generated directly from that navigation height field.
- A runnable 300-agent ECS simulation with one enemy, dynamic obstacles, path
  following, a town hall, resources, a status HUD, orthographic 3D pan/zoom
  camera controls, collider-backed surface picking, screenshot capture,
  directional lighting, converted representative GLB scenes with primitive
  fallbacks, and an explicit idle/moving visual state machine. Terrain vertex
  colors encode elevation, a translucent water surface uses the authored water
  level, an Avian trimesh collider mirrors the visible surface, and actors,
  resources, buildings, save restores, joins, paths, and selection markers are
  grounded to deterministic centimetre heights.
- Unity's five-member starting NPC roster (Defender, Logger, Miner, Gatherer,
  Builder) is restored. Resource workers choose role-matched stable nodes,
  path to them, gather/deplete their authored `BaseActionAmount`, carry the
  authored `BaseMaxResource` (10 for the shipping resource roles), return to a
  walkable assigned-station approach, and deposit into the authoritative town
  economy; exhausted resource visuals are hidden. Content schema 16 converts each role's
  resource affinity, XP multiplier, level curves, action rate/range,
  health/regeneration/defense, movement speed, and carry capacity. Successful
  actions award the same modified XP as Unity, role progress persists across
  role changes and native saves, levels cap at 99 on Unity's authored curve,
  and level-ups heal to the recalculated maximum. Live gathering, construction,
  combat range/damage, health regeneration, cooldowns, movement, and carrying
  use those leveled values plus technology percentages. `!experience`/`!exp`
  reports the current role, level, XP, and next threshold.
- Unity's Station/StationSensor behavior is represented by stable authored data
  and runtime assignments. The exporter preserves raw `[Flags]` bitmasks, so
  combined masks such as Builder construction/damaged-building, Windmill
  bush/farm, and Tower enemy/injured-player survive conversion. Completed
  production stations compete with Town Hall by compatible role/target masks
  and deterministic distance. Actors reassign when roles or station availability
  change; gathering, construction, combat, and deposits honor assigned range and
  target capacity. Assignments persist in native saves, and resolvable legacy
  JSON/binary building GUID assignments survive one-time import.
- All 15 `CharacterModelHandler` equipment sets are typed using exact converted
  GLB node names. Bevy activates the selected slim role body and permanent tools,
  shields, and helmets; toggles carry-only props from authoritative inventory;
  and drives translated `CarryWood`/`CarryHip` parameters. The content tool shows
  station masks/ranges/capacities and role equipment bindings.
- Content schema 16 preserves every role's exact Unity `AnimationName` action
  parameter and authored variant count. Live gather, construction, attack, and
  heal goals now drive the converted Player controller's `Action`, role trigger,
  `AnimationIndex`, and remapped `ActionSpeed` parameters, while locomotion,
  carrying, death, and revival continue through the same controller runtime.
  Presentation schema 4 also preserves the 32 shipping Animator states with an
  active float speed parameter; Bevy multiplies each state's authored base speed
  by that live parameter, including every Player action bound to `ActionSpeed`.
- Presentation schema 6 preserves all 45 state machines and 33 controller layers,
  including layer roots, nested children, default states, blend modes, stable
  avatar-mask references, Any State ownership, entry transitions, and state-machine destinations.
  The runtime follows the active base hierarchy into conditioned child entries and
  returns child exits through their parent default instead of a hard-coded reset.
- Converted controller layers now keep independent state runtimes with shared live
  parameters. Their clips are routed beneath Bevy override/additive graph nodes
  with Unity-correct weights: layer zero is fixed at one, while later layers use
  their serialized defaults. All three AvatarMask assets retain 477 transform
  weights (118 disabled), stable references, and humanoid-body mask bytes; disabled
  targets become Bevy animation mask groups. Character's Top `Carry`/`CarryHip`
  state machine follows live inventory parameters but, matching its source
  controller, its authored zero weight gives it no pose influence.
- Presentation schema 11 retains 110 Unity float-property curves with 261 keys
  across 18 clips and all ten authored `PlayRoleActionAudio` events. The runtime
  samples constant, unweighted Hermite, and weighted Bezier segments (all 261
  shipping keys are currently unweighted); the Credits state uses the
  converted seven-panel visibility timeline, two fireworks cues, and end fade,
  while actor level changes use the converted LevelUp position/alpha curves.
- Combat roles acquire living enemies while Goblins acquire the nearest living
  player and retaliate. Melee roles apply deterministic damage in range;
  Necromancer, Ranger, and Wizard attacks spawn visible homing ECS projectiles
  whose damage lands on impact. Priests prioritize the nearest injured player,
  path into authored range, heal on their role cadence, and stop at full health.
  Death enters the converted controller state; player revival uses the Unity
  prefab's authored 60-second delay, returns actors to a connected spawn cell,
  and emits the Revive transition. `!revive` spends the authored 400 food for
  self-revival, while Priests and Paladins can spend 200 food to revive another
  stable Twitch actor and earn Unity-equivalent role XP. Dead enemies remain
  inactive for their owning spawner/event system rather than receiving the old
  five-second placeholder respawn. Actor health and pending revival time persist
  in native snapshots.
- The Tower's two identical Unity `ProjectileShooter` variants are consolidated
  into one typed definition: 1 damage, three-second fire cadence, 10-cell range,
  and 15-cell/second projectiles. Completed Tower entities target the nearest
  living enemy and launch the same visible ECS projectile path used by ranged
  actors. The converter also promotes all 42 reachable prefab `HealthHandler`
  records, including regeneration food requirements and player revival timing.
- Player-placed buildings now follow Unity's health-driven construction
  lifecycle. They start at 10% health, Builder-role actors select a reachable
  perimeter cell and work on the nearest incomplete structure, presentation
  advances across the authored 33%/66% stages, and completion restores the full
  converted model. `!upgrade` uses typed Unity `CanLevel`, level-cost,
  cost-multiplier, and technology-issued maximum-level data; health, completion,
  level, navigation occupancy, and presentation stage round-trip through saves.
  Content schema 16 also promotes Unity `Placeable` and all six technology effect
  categories. The `Unlock Building` effects make the four authored starting
  technologies expose Lumbermill, Stonemason, Tower, and Windmill; later
  technology votes expose their referenced buildings, and commands reject
  locked or non-placeable definitions. The catalog's 413 authored effects are
  typed rather than left in provenance: those 28 unlocks, 177 level caps, 104
  role/global stat boosts, 80 building-cost reductions, 12 storage boosts, and
  12 age upgrades.
  Unlocked effects now reduce placement and upgrade costs, cap deposits against
  technology-expanded storage, modify health/movement/action/combat rules, and
  switch constructed GLBs and the always-present Town Hall to their age-two
  scene variants. The Town Hall is a stable, complete level-one building in the
  authoritative simulation, so it participates in upgrades, building counts,
  native save/load, and the same presentation synchronization as construction.
  Twitch construction now uses Unity's staged placer lifecycle: `!build`
  creates a per-player translucent validity preview without spending resources,
  `!move` and the directional aliases adjust it in grid cells, `!rotate`
  applies retained 90-degree turns, `!confirm`/`!accept` charges and commits an
  exact valid site, and `!cancel` discards it. Last successful position and
  cumulative rotation survive native saves. Rotated non-square footprints drive
  occupancy, station centres, builder approaches, Tower origins, native restore,
  and legacy JSON/binary building rotation import. `!level <building> <id>
  [times]`, `!levelall <building> <level>`, and `!remove <building> <id>` use
  Unity's one-based per-type ordering; removal clears stable station/target
  references, ECS presentation, and navigation occupancy.
  All 422 objectives in the reachable production technology graph are typed,
  including build/build-any,
  collect, kill/kill-any, buy/buy-any, and sell/sell-any targets and amounts. A
  winning vote starts a persistent town goal instead of unlocking immediately;
  deposits, completed construction, enemy kills, and the restored `!buy`/`!sell`
  path advance it, with the technology unlocked only after every objective is
  complete. Goal progress appears in the HUD, round-trips through native saves,
  and current legacy JSON/binary objective progress is restored when it matches
  the converted technology.
  The same schema converts all four reachable `ResourceStorageModifier`
  components: House contributes recruit capacity, while Food/Ore/Wood Storage
  contribute 1,000 units at level one and use the authored 2,000 x level x 3
  increment thereafter. Only completed buildings contribute; unlocked storage
  boosts apply to those contributions, and capped deposits retain overflow in
  actor inventory.
- End-to-end execution of the stable Bevy chat grammar: `!join`, role selection,
  role-experience reporting,
  catalog-priced building placement and upgrades, eligible technology voting,
  mapped events, atomic saves, and help. Commands validate catalog references
  and prerequisites, return HUD/Twitch feedback, and use the authoritative
  actor/selection position.
  Constructed buildings block the deterministic grid, spawn their converted GLB
  with a primitive fallback, persist in native saves, and are reconstructed with
  their navigation regions on load.
- Persistent ruler governance matching the shipping Unity flow. The first
  election is scheduled after 30 seconds; new-ruler and hourly keep-ruler
  ballots pause until the first vote, run for 120 seconds, accept one vote per
  stable player, preserve deterministic first-option tie behavior, and restore
  the previous role on replacement or resignation. Ruler identity, previous
  role, ballot tallies/order, and cooldown persist in native saves, while the
  one-time importer restores Unity ruler names and cooldowns. `!vote` routes to
  governance while a ruler ballot is active and to technology otherwise.
  Broadcaster/moderator identity survives Twitch dispatch for permission checks;
  `!buy`, `!sell`, `!recruit`, `!recruits`, and `!save` enforce ruler-or-staff
  access, and `!rulervote`/forced `!event` are staff-only. Recruiting creates
  stable NPC actors and applies converted `HasUserLimit`/`BaseMaxUserLimit`
  role constraints plus the 11 converted building `RoleSlotModifier` components
  and House-backed recruit capacity. The five starting NPCs occupy that capacity;
  recruit IDs, inspection, dismissal, and role reassignment use stable ordering.
- The player-facing Unity command surface now includes role/health and per-role
  progression queries, available-role and building catalogs, stable building/
  recruit IDs and item info, explicit station/target selection, Town Hall unstuck,
  location ping, town stats, ruler camera movement/reset, moderator role changes,
  and all six customization selectors. Selected targets and every customization
  choice affect live ECS behavior and presentation: exact Unity list ordering is
  retained for 10 eye, seven hair, and two facial-hair meshes; the three role-body
  variants, five eye colors, and six shared hair/beard colors bind to converted
  GLB descendants; helmets hide hair as in `CharacterModelHandler`.
- Pet parity covers subscriber Red Panda entitlement, Unity's deterministic
  one-in-5,000 gathering unlocks for Giraffe, Duck, and Butterfly, the Fish God
  reward, `!pets`/`!pet`, converted GLB followers, and native/legacy persistence.
- An opt-in Twitch transport using public-client device OAuth, `twitch-irc`,
  Tokio, Rustls, and OS credential-vault storage. It validates app/account/scopes,
  rotates public-client refresh tokens, revalidates hourly, preserves the
  broadcaster `!connect` safety gate, and derives stable actors from Twitch user
  IDs. The tools application can authorize, diagnose, refresh, and forget the bot
  token without displaying or writing credentials to repository files.
- Engine-independent gathering, depositing, construction, roles, technology
  voting, authored building upgrades, trade, combat, death/respawn, timed events,
  days, seasons, and weather.
- Runtime season and weather presentation driven by that simulation state:
  seasonal terrain/water palettes, clear color, sun and ambient lighting,
  distance fog, plus deterministic rain and snow fields with no second clock.
- Checksummed native RON saves written atomically with backup recovery, plus
  in-game F5/F9 save/load that restores stable actors, constructed buildings,
  dynamic navigation occupancy, town resources, per-node depletion, carried
  inventories, votes, events, and simulation state. Empty depletion maps are
  omitted so native format-1 checksum serialization remains compatible.
- A one-time legacy importer for JSON and the exact Unity binary field order in
  schemas 1-3, including container/compression/bounds/trailer validation, named
  backup recovery, schema-1 mesh retention, seed-based Bevy regeneration,
  stable IDs, deterministic surface relocation, atomic native output, SHA-256
  provenance, and post-write reload verification. Retained schema-1 geometry is
  validated for finite vertices/UVs, triangle integrity, and index bounds; the
  runtime reconstructs its Bevy mesh and Avian collider on load, and subsequent
  native saves preserve both the mesh and migration provenance. Legacy input is
  never modified.
- A Unity asset inventory/validator that resolves `.meta` GUIDs and YAML
  references, plus a Unity 6000.5.6f1 editor exporter for serialized fields,
  object references, prefab sources and overrides, and the four shipping scene
  hierarchies. The verified export contains 1,429 assets; 61 obsolete or
  missing-script ScriptableObjects require the YAML fallback because Unity did
  not expose a main object.
- Explicit migration-only A* API stubs sufficient to compile and run the editor
  exporter. They are intentionally inert and are not used by the Bevy runtime.
- A deterministic semantic RON converter that follows the active Unity
  containers to select 26 production buildings, 215 prefab archetypes, 288
  model scene variants, 15 roles, and the shipping 363-node, 362-edge technology
  DAG. It resolves nested prefab/model dependencies, derives building footprints
  from the authored grid sizes, promotes construction/upgrade balance and
  all 413 authored technology effects, role base stats and level curves, and
  building storage and role-slot contributions, stations, role target masks, and equipment into
  422 technology objectives in content schema 16, preserves the remaining typed Unity
  fields as provenance, validates stable IDs, referenced
  buildings/roles, prerequisites, groups, and cycles, and reloads its own RON
  output.
- A versioned presentation RON converter and YAML fallback that packages all
  133 reachable PNG/TGA textures (19,291,847 bytes), preserves 33 Unity
  materials with shader source, PBR approximations, texture slots, and custom
  shader properties, including 141 authored vector/color parameters and 32
  scale/offset transforms for reachable texture slots, and
  translates all 31 Animator controllers into 94 stable
  states, 166 transitions, parameter schemas, layer defaults, and 75 referenced
  clip records. The YAML fallback converts 57 of the 61 standalone `.anim`
  files into 1,196 stable transform tracks with rig-relative reference poses.
  Presentation schema 11 additionally retains all 110 component/UI property
  curves (261 keys) across 18 clips—including the four transform-free clips—and
  all ten authored animation events. The runtime dispatches each converted
  `PlayRoleActionAudio` event exactly once per crossed clip cycle and plays a
  deterministic 85 ms procedural cue with documented no-sample provenance.
  All 166 transitions preserve their fixed/normalized duration mode and
  destination offset; runtime crossfades preserve the source/destination
  blend-tree weights and layer masking.
  Eleven authored 1D blend states retain their driving parameters and
  thresholds, and transition conditions use typed Unity modes. The stale `Slam`
  and `Swipe` conditions are retained as two inferred Boolean parameters rather
  than silently discarded. It resolves inherited prefab/controller/model
  dependencies into 22 animation bindings and resolves inherited renderer
  dependencies into 141 prefab material bindings containing 181 material
  dependencies. Presentation schema 7 resolves 241 Unity model-importer
  material-name remaps and 912 slots across 903 prefab renderers, including the
  shipping two-material meshes; 18
  animation bindings have an embedded GLB animation available. The sole
  dangling Unity motion GUID in the Necrolands camera controller is retained
  and checked as an explicit missing-source baseline instead of being discarded.
- A pinned Blender 4.2.0 headless FBX-to-GLB pipeline with atomic outputs and
  independent hash/header/unit validation. Unity renderer bounds normalize the
  imported geometry, rigs, and translation curves. All 253 GLBs are reproducible
  and tracked with Git LFS: 820 meshes, 43 skins, 33 embedded animations, 253
  materials, one embedded image, and 95,464,596 output bytes.
- Bevy runtime animation binding for compatible converted scenes. The shipping
  Goblin prefab inherits its controller/model relationship through the nested
  model prefab and plays GLB animation zero. The Player locomotion controller
  resolves stable Idle and Walk clip IDs, builds Bevy `AnimationClip` assets
  from the converted Unity tracks, retargets curve deltas onto the converted
  GLB rest pose, and drives a native `AnimationGraph` across 23 bone targets.
  The engine-independent controller runtime initializes typed Float/Integer/
  Boolean/Trigger parameters, evaluates AnyState and state transitions with
  exit-time gates, consumes triggers, and produces threshold-weighted two-clip
  blend selections. The Player runtime builds 19 compatible controller clips,
  feeds Unity's velocity/5 `Move Speed` convention into the Idle/Walk/Run blend
  state, and maps actor alive-state changes to Death/Revive triggers. Scenes
  without a compatible embedded or converted clip keep the visual fallback.
- Bevy runtime material reconstruction for converted scenes. Unity base color,
  primary texture, emission, metallic, smoothness, and alpha settings are mapped
  into cached `StandardMaterial` assets, with PNG and TGA decoding enabled. The
  glTF material names select their exact model-importer material, then any
  matching per-renderer prefab override. The first inherited prefab material is
  retained as a fallback for unresolved legacy assets; the Town Hall path is
  covered by the DirectX 12 GPU smoke capture.
- A custom Bevy PBR terrain extension and WGSL fragment shader consumes the
  converted `Env_Terrain` palette, noise texture, texture scale, blend height,
  and tint controls. Its shoreline-height transition is anchored to Bevy's
  configured waterline so the authored look remains meaningful on the
  deterministic replacement terrain, and season tint stays live.
- A separate PBR water extension consumes `Env_Water`'s shallow/deep, foam,
  wind, noise, alpha, and ice controls. Its WGSL port animates both authored
  noise textures and restores Unity's winter ice toggle while preserving live
  season/weather tint. Exact scene-depth shoreline foam remains a later render
  pass rather than being represented as completed.
- Exact prefab/model renderer bindings can replace a glTF primitive's standard
  material with a typed custom extension. The 688 reachable references to the
  shared `Building_Material` now use a WGSL port of `Building.shader`, including
  the authored detail texture and transform, vertex-color ambient occlusion,
  winter snow/exclusion, metal/smoothness, emission, roof variation, and damage
  threshold. The damage mask is adapted to the replacement terrain's vertical
  datum. A single cloned material per stable building reproduces Unity's
  `MaterialPropertyBlock` health percentage without multiplying assets per mesh
  primitive, follows season changes, and is released on despawn. Healthy and
  damaged winter rendering are covered by DirectX 12 smoke captures.
- Main Menu and Credits recreate the reachable `VFX_Clouds` prefab's 21 Unity
  built-in planes without routing generated geometry through Blender. A typed
  cloud material ports its two world-space animated noise layers, alpha cutoff,
  tint, surface exponent, and packaged noise texture; a DirectX 12 menu capture
  verifies the transparent layer field and authored `_CloudTint=200` value.
- Priest heals now drive a purpose-built ECS replacement for the reachable
  `VFX_healing` and `VFX_Healing_Channeling` graphs. The target receives the
  Unity-authored 1.2-second green/gold healed burst, while the healer emits the
  serialized five-second channel field with its 0/0.289/1/0 size keys. Automatic
  respawns, paid `!revive`, and game-master revival emit the larger gold-accented
  variant. All three use short-lived shared meshes/materials rather than a
  prewarmed pool, and a deterministic DirectX 12 smoke scene covers them.
- Combat presentation now preserves role identity instead of rendering every
  ranged attack as the same cube. Rangers and Towers use the converted
  `Arrow.glb`, Wizards use the serialized orange `VFX_Fireball` size/trail
  values, and Necromancers use the violet channel palette. Unity's two-second,
  0.1-width Tower trail and 0.25-second `VFX_HitCharacter` burst drive shared
  short-lived ECS trail/impact entities. Melee damage and every projectile
  arrival emit the appropriate physical, arrow, fire, or necrotic impact, with
  a repeatable DirectX 12 smoke field covering all four styles.
- Content schema 16 promotes every enemy prefab's serialized `TargetSensor`
  mask into typed target IDs. The live selector now chooses the nearest valid
  player or building; the battering ram's building-only mask is preserved, and
  destroyed buildings release their deterministic navigation region. Building
  construction/repair, enemy hits, upgrades, and damage now drive purpose-built
  Bevy effects using the Unity VFX Graph values: 0.5-second work smoke/sparks,
  1.5-second level arrows, and health-scaled persistent fire/smoke over the
  authored 1.4036398 spawn radius. The existing per-building WGSL
  `_DestructionValue` response remains the underlying damage material layer.
- A focused Bevy/egui tool application with the planned eight work areas and an
  embedded ECS inspector. It loads the real catalogs, browses stable building and
  role references plus prefab archetypes, GLB variants, materials, texture slots,
  shader provenance, and Animator states/transitions, provides grouped
  technology search/topology plus validated metadata/prerequisite editing with
  undo/redo, and renders deterministic world occupancy, resources, and planned
  paths. Twitch setup is connected to the OS vault; the general runtime panel
  remains a diagnostic shell rather than a connected control surface.
- Windows CI covering formatting, compilation, Clippy, tests, and repository
  validation. The focused tools app now runs repository validation and optimized
  Windows packaging as real background jobs. The package workflow atomically
  produces a validated ZIP containing the game/tools executables, runtime
  assets, README, and GPL license; CI publishes the green archive as an
  artifact.
- A measured 300-agent presentation LOD: 16 actors use authored GLB rigs and
  shared animation graphs while the remaining crowd uses lightweight capsule
  visuals without changing authoritative gameplay or persistence. The recorded
  1920×1080 DX12 reference run reached 10.74 ms average and 14.25 ms p95 across
  559 post-warmup frames on the documented reference machine.

## Not yet at parity

- The remaining rare/non-gameplay action emitters. Direct and nested
  layer/state-machine routing, conditioned entries,
  parent exits, masks, and property curves are converted and live.
- Scene-depth shoreline foam, chunked LOD,
  foliage/biome rendering, production-grade actor steering, complete advanced
  role/inventory behavior beyond the live resource-worker loop,
  area attacks, additional enemy archetype behaviors/spawners, combat buildings
  beyond the Tower, station
  effects derived from building level beyond current activation/range behavior,
  and every reachable balance rule from the Unity scenes.
- Per-command cooldowns and exact production
  outbound response wording. The authenticated IRC path now includes player
  role/health/progression, role/station/target selection, unstuck/ping,
  pets and cosmetics, catalogs/info/town stats, recruit inspection/dismissal/
  re-role, camera movement/reset, moderator role assignment, governance,
  economy, save, event, staged building-placement, indexed/all-building upgrade,
  and indexed removal commands. The full registered Unity game-master surface,
  queued Fish God/Monster Raid administration, current-event action, direct
  goals/technology control, persisted build-cost and role-limit toggles, and
  the remaining `!stdiscord` utility command are implemented behind an explicit
  numeric Twitch-ID allowlist. Unity registers no shipping emote commands.
  `!buy` and `!sell` use Unity's authored rates.
- Remaining reachable WGSL shader ports, VFX, UI parity, post-processing,
  replacement audio, and accessibility.
- Legacy target, active/unlocked pet, and customization data now map into native
  actor state and live presentation.
- Persistent catalog writes, node/group creation and deletion, interactive graph
  layout, live runtime bridging, integrated frame capture, and profiling
  controls beyond the command-line GPU harness.
- Curated screenshot baselines, a launch-through-credits gameplay acceptance
  suite, signed/notarized distribution, and a hosted Windows release. The
  unsigned CI/local Windows archive and measured reference-machine 60 FPS gate
  are now implemented.

## Validation

From `bevy-port`:

```powershell
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo xtask validate
cargo xtask stress --agents 300
```

Run the applications with:

```powershell
cargo run -p stream_town_game
cargo run -p stream_town_tools
```

Generate the ignored, machine-local Unity inventory with:

```powershell
cargo run -p stream_town_migrate -- inventory .. --out generated/content-manifest.json
cargo run -p stream_town_migrate -- validate-manifest generated/content-manifest.json
```

Generate and validate the ignored, machine-local editor export with:

```powershell
.\bevy-port\scripts\export-unity.ps1
cd bevy-port
cargo run -p stream_town_migrate -- validate-unity-export generated/unity-export.json
cargo run -p stream_town_migrate -- convert-content generated/unity-export.json --unity-root .. --out-dir assets/content
```

## Milestone interpretation

Foundation is substantially implemented. The vertical slice is runnable and its
deterministic 300-agent navigation gate passes with semantically converted
prefabs, representative production GLBs, and connected Twitch transport, but it
now also has complete presentation metadata, packaged textures, native Goblin
animation binding, 57 converted standalone transform clips, Player locomotion
retargeting, a typed controller interpreter, 19-clip Player blend-graph playback,
the generated heightfield/water/collider surface, inherited prefab material
bindings, runtime PBR material reconstruction, simulation-driven season/weather
presentation, and the live stable command grammar with constructed-building
persistence, plus autonomous role-driven gathering/deposit and persistent node
depletion, connected actor spawning, live combat/death/respawn, health-staged
Builder construction, technology-gated upgrades, and typed technology discounts,
storage, stat, and building-age effects. It is still missing rare/non-gameplay
action emitters, remaining scene-depth shoreline/LOD and foliage shader work,
particle fields, remaining custom-shader parity, the
exact command cooldown/wording behavior, and the recorded reference-machine GPU
measurement required to close the milestone.
Gameplay parity, presentation, and hardening remain long-term work.
