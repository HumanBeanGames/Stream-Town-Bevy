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
- Validated, versioned RON configuration (schema 3) and stable authored/runtime
  IDs that do not expose Bevy entity identifiers. Gameplay configuration now
  carries Unity's 5,000-unit starting food/gold/ore/wood balances and zero
  recruits instead of hiding those values in runtime code.
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
  path to them, gather/deplete five units per visit, carry 25 units, return to a
  walkable Town Hall approach, and deposit into the authoritative town economy;
  exhausted resource visuals are hidden.
- Combat roles acquire living enemies while the Goblin holds its ground and
  retaliates. Both sides path into range, attack on a one-second cadence, apply
  deterministic role-specific damage, enter the controller's Death state at
  zero health, wait five seconds, respawn on a connected stable cell, and emit
  the Revive transition. Actor health is captured in native snapshots.
- Player-placed buildings now follow Unity's health-driven construction
  lifecycle. They start at 10% health, Builder-role actors select a reachable
  perimeter cell and work on the nearest incomplete structure, presentation
  advances across the authored 33%/66% stages, and completion restores the full
  converted model. `!upgrade` uses typed Unity `CanLevel`, level-cost,
  cost-multiplier, and technology-issued maximum-level data; health, completion,
  level, navigation occupancy, and presentation stage round-trip through saves.
- End-to-end execution of the stable Bevy chat grammar: `!join`, role selection,
  catalog-priced building placement and upgrades, eligible technology voting,
  mapped events, atomic saves, and help. Commands validate catalog references and prerequisites,
  return HUD/Twitch feedback, and use the authoritative actor/selection position.
  Constructed buildings block the deterministic grid, spawn their converted GLB
  with a primitive fallback, persist in native saves, and are reconstructed with
  their navigation regions on load.
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
  provenance, and post-write reload verification. Legacy input is never modified.
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
  technology level caps into content schema 3, preserves typed Unity fields as
  provenance, validates stable IDs/references/cycles, and reloads its own RON output.
- A versioned presentation RON converter and YAML fallback that packages all
  133 reachable PNG/TGA textures (19,291,847 bytes), preserves 33 Unity
  materials with shader source, PBR approximations, texture slots, and custom
  shader properties, and translates all 31 Animator controllers into 94 stable
  states, 165 transitions, parameter schemas, layer defaults, and 75 referenced
  clip records. The YAML fallback converts 57 of the 61 standalone `.anim`
  files into 1,196 stable transform tracks with rig-relative reference poses;
  the remaining four clips contain property/UI animation rather than transform
  curves. Eleven authored 1D blend states retain their driving parameters and
  thresholds, and transition conditions use typed Unity modes. The stale `Slam`
  and `Swipe` conditions are retained as two inferred Boolean parameters rather
  than silently discarded. It resolves inherited prefab/controller/model
  dependencies into 22 animation bindings and resolves inherited renderer
  dependencies into 141 prefab material bindings containing 181 slots; 18
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
  first inherited prefab material is applied to spawned GLB renderer descendants;
  the Town Hall path is covered by the DirectX 12 GPU smoke capture.
- A focused Bevy/egui tool application with the planned eight work areas and an
  embedded ECS inspector. It loads the real catalogs, browses stable building and
  role references plus prefab archetypes, GLB variants, materials, texture slots,
  shader provenance, and Animator states/transitions, provides grouped
  technology search/topology plus validated metadata/prerequisite editing with
  undo/redo, and renders deterministic world occupancy, resources, and planned
  paths. Twitch setup is connected to the OS vault; the general runtime panel
  remains a diagnostic shell rather than a connected control surface.
- Windows CI covering formatting, compilation, Clippy, tests, and repository
  validation.

## Not yet at parity

- The four standalone property/UI animation clips, non-transform event/property
  curves, nested layer/state-machine routing, transition-duration crossfades,
  layer masks, and exact Unity tangent/cubic interpolation semantics. The
  controller interpreter can execute direct state transitions, but gameplay
  systems still need to emit every gathering/building/combat/equipment action
  parameter and nested state-machine exits currently fall back to Locomotion.
  Per-prefab renderer activation for player role/equipment variants co-located
  in source FBX files also remains.
- Production terrain material/shader parity, shoreline treatment, chunked LOD,
  foliage/biome rendering, production-grade actor steering, complete role/
  station/inventory/equipment behavior beyond the live resource-worker loop,
  projectiles/area attacks/healing beyond the live melee combat loop, building
  station activation and derived level-up effects beyond the live construction/
  upgrade loop, and every reachable balance rule from the Unity scenes.
- Full Unity Twitch command coverage, per-command permissions/cooldowns, and
  production outbound response wording. The authenticated IRC path currently
  executes the complete stable Bevy grammar listed above and relies on
  `twitch-irc` for reconnect and chat rate limiting; Unity's larger ruler,
  game-master, player-action, trade, and event command surface still remains.
- WGSL shader ports, exact multi-slot/custom-shader material parity, VFX, UI
  parity, post-processing, replacement audio, and accessibility.
- Rendering schema-1 retained terrain meshes and full semantic reconstruction of
  legacy target/station/pet/customization data. The importer currently preserves
  or maps gameplay-critical world, entity, inventory, economy, and technology
  fields; unsupported presentation/relationship fields are consumed and
  validated but not represented in the native runtime yet.
- Persistent catalog writes, node/group creation and deletion, interactive graph
  layout, live runtime bridging, release packaging, frame capture, and profiling
  controls.
- The reference-machine 60 FPS GPU gate, screenshot baselines, launch-through-
  credits gameplay acceptance suite, and Windows release artifacts.

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
Builder construction, and technology-gated upgrades. It is still missing nested
controller layers and complete gameplay action emitters,
production terrain/foliage shaders beyond the new environment palettes and
particle fields, exact curve tangents and multi-slot/custom-shader parity, the
broader Unity-only command surface, and the recorded reference-machine GPU
measurement required to close the milestone.
Gameplay parity, presentation, and hardening remain long-term work.
