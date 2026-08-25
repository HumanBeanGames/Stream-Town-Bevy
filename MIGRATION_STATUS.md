# Bevy Migration Status

Last updated: 2026-08-25

The Unity project remains at the repository root as read-only migration input.
The new Rust workspace is in `bevy-port`. This document records delivered
behavior separately from planned parity so partially implemented tools cannot be
mistaken for production-ready systems.

## Delivered in this milestone

- Shipping-scale world rendering now replaces the 64x64 Bevy prototype. Config
  schema 7 upgrades untouched schema-5 worlds to Unity's 200x200 samples,
  two-unit cells, one-metre height scale, and 0.05-unit waterline while
  preserving customized world settings. Generator version 5 ports the authored
  terrain curve, island falloff, half-unit quantization, Unity-compatible random
  octave offsets, globally normalized noise maps, all three resource layers,
  and all four foliage layers. Source-space placement and shared one-cell
  clearance remove coincident resource/foliage groups. Raw converted primitive
  loads restore their omitted 0.01 glTF scene-node scale. Generated terrain is
  one continuous render mesh/collider rather than independently lit LOD chunks,
  eliminating chunk seams. Water uses energy-conserving bloom and bounded color
  output so the high-exposure coastline cannot clip to white; normalized foam
  cutoff, single-pass foam color, and partial seasonal tint keep its authored
  cyan depth surface visible. Tree/foliage shadow casting and object-stable
  colour variation are joined by one shared visible/shadow tree deformation.
  Trees now cast and receive shadows and remain PBR-lit. A deterministic
  stationary/orbit/zoom GPU sweep validates all 19,901 resource and foliage
  renderers as casters and receivers with no duplicate mesh/transform groups.
  That sweep also exposed and fixed a renderer-only error that discarded the
  generator's exact source-space sub-cell offsets and collapsed distinct Unity
  positions onto one Bevy cell hash; generation inputs and counts were unchanged.
  The Player now
  uses the animation FBX's single mesh-compatible armature instead of the TPose
  asset's nine independent skins. Its translated controller now sources each
  motion from the matching native take on that same armature. Live diagnostics
  measure advancing clip time, non-zero named-joint rotation, and references
  from those joints into the rendered skin, pending user visual confirmation;
  see the dedicated animation ledger. The player-only broken animated shadow-skinning output is
  disabled while all other world shadows remain live. The in-game camera now reproduces
  the shipping Unity `MainCamera.prefab`: perspective 60-degree FOV, 0.3/1000
  clipping planes, the 45-degree negative-X-side pose, physical height zoom,
  ten-pixel edge detection, movement smoothing, and serialized XZ bounds.
- Boot loading now starts after render assets exist, preloads its authored town
  background, overlay, and spinner as part of the readiness gate, and presents
  decoded artwork before changing state. Terrain now follows Unity's constant
  zero-smoothness output rather than the generic material inspector value, has
  no metallic/specular contribution, and preserves a bounded ambient colour
  floor under the authored rain/day lighting. Transparent rain VFX no longer
  cast long moving cuboid shadows. Fish schools reject generated land cells,
  while resources retain generator-authoritative coordinates and compensate
  only for asymmetric converted prefab pivots.
- The player controller attachment path targets the consolidated GLB's explicit
  `CharacterArmature` root and retargets Unity's FBX-only `pelvis/...` curve
  paths by their unchanged bone suffix. Rendered smoke runs report all five
  starting controllers with 20 clips and 245 targets each, but these counts have
  not translated into user-visible motion and are not treated as visual proof.
  Clicking uses
  an explicit stable actor selection rather than inferring an occupant from a
  selected cell, so overlapping/moving actors remain individually selectable.
  Middle-button drag now uses Unity's 0.5-second `Vector3.SmoothDamp` contract,
  discards the press-edge accumulated delta, and preserves the correct vertical
  direction without frame-time scaling a pixel delta.
- The one-off menu foundation repair now places all 186 adjacent farm tiles on
  one median plateau, shifts their authored instances to that height, recomputes
  normals, and is idempotent. The checked-in schema-3 bake has been repaired;
  ordinary building foundations and deterministic generation coordinates are
  not reprocessed.
- A Bevy 0.19/Rust 1.95 workspace split into domain, game, tools, migration, and
  `xtask` crates.
- The `Boot`, `MainMenu`, `WorldLoading`, `InGame`, and `Credits` application
  states, with state-scoped entity cleanup. A headless launch-through-credits
  acceptance test now traverses the complete shipping state sequence, generates
  a town, joins a Twitch viewer through the live command path, writes and reloads
  a native save, enters the authored Credits timeline/fireworks, and verifies
  world and Credits resources are released on exit.
- Versioned player settings reproduce Unity's defaults and its saved video,
  audio, camera/input, autosave, name-display, and building-health preferences.
  The runtime imports the legacy `SettingsData.json` once when present, writes
  validated RON atomically with backup recovery, applies display mode,
  resolution, VSync/FPS limiting, MSAA, shadows, shadow-map size, SSAO,
  brightness/gamma, independent master/music/ambience/SFX volume, and camera
  controls/sensitivities, and performs autosaves at Unity's
  0/5/10/30/60-minute intervals. The keyboard-driven Main Menu and in-game
  Escape panel expose every persisted setting through an apply/default/cancel
  draft workflow plus new/load/save/credits/quit actions. The tools application
  has the same dedicated settings editor, reset, validation, and save workflow.
  Bevy 0.19 cannot safely enter exclusive fullscreen during either window
  creation or DX12 surface reconfiguration on all Windows drivers. The retained
  Unity `Fullscreen` preference therefore uses borderless-fullscreen
  compatibility, selected before renderer surface creation, while `Windowed`
  remains distinct. This avoids both the missing-current-monitor panic and the
  subsequent DX12 swapchain/device-loss cascade.
- Accessibility settings schema 3 adds persisted 75-150% UI scaling, high
  contrast, and reduced motion without changing upgraded schema-2 files.
  Every shipping UI button participates in modal-aware keyboard focus with a
  visible focus ring; Enter/Space and AccessKit Click requests dispatch through
  the same typed actions as pointer input. Buttons, tabs, checkboxes, labels,
  disabled states, loading progress, and polite state/feedback announcements
  are exposed through Bevy's AccessKit bridge. Reduced motion freezes loading,
  vegetation, water, menu-cloud/fish/windmill, weather, and Credits-firework
  presentation loops while preserving authoritative gameplay movement. The
  tools Settings tab edits the same validated fields. Manual and automated
  acceptance instructions are in `bevy-port/docs/accessibility.md`.
- Unity's name-display index semantics now drive camera-projected player labels,
  including the privileged-user-only mode and the original game-master,
  broadcaster, moderator, subscriber, and normal-user colors. Twitch privilege
  class is stable native actor state, survives save/reload, and is recovered
  from both binary and JSON legacy saves. Building-health preferences now drive
  continuously updated in-world bars using authored per-level maximum health.
- With no redistributable Unity soundtrack in the source repository, the
  shipping music path now uses the native Rust `bevy_tidal` scheduler and CPAL
  renderer. Eight authored Tidal expressions provide distinct seasonal
  day/night arrangements, switch when the simulation crosses those boundaries,
  and incorporate master/music gain directly into the active pattern. Muting or
  leaving the town silences track 1. The public upstream crate is consumed
  directly from GitHub at a pinned revision, so clean builds and CI use the
  library without copied source or repository credentials. Its built-in
  synthesizers require no GHC, SuperCollider, sidecar, or downloaded sample
  bank. Bevy Audio still owns ambience and gameplay cues on their independent
  ambience/SFX gain paths. Ambient wind remains a seamless band-limited
  oscillator loop rather than sample-rate white noise, avoiding the earlier
  placeholder's static-like hiss.
- Presentation schema 19 retains all 35 original Unity `ActionClips` GUID
  references across the 14 roles that authored them (Blacksmith authored none).
  Each reachable animation event selects its original variant deterministically
  and plays a role-shaped, smooth-envelope procedural WAV at the actor's world
  position with Unity's 20-unit enable distance. These cues contain no source
  recording or broadband-noise placeholder, respect the SFX/master mix, and
  are cached by original clip GUID.
- The cloud WGSL material now analytically filters the authored 20x noise layer
  when it is smaller than a screen pixel. This recreates Unity's mipmapped TGA
  sampling without the prior moving white-static aliasing on menu and credits
  cloud planes.
- The authored main menu corrective bake is now version 3. It retains the
  generator seed/hash and all 2,565 resource plus 12,392 foliage records while
  applying a menu-only 3x vertical scale, independently sampled/flattened
  foundations, and recomputed normals. Its water uses one fixed depth and lower
  alpha over a uniform floor; submerged checker triangles no longer split the
  ocean into dark/light regions. Menu foliage has stable object-space colour
  variation and a static no-wind material to eliminate blue/black temporal
  flashes. Clouds are explicit non-shadowing cuboids, and both windmill ages
  rotate around the normal of their emitted GLB blade plane rather than the
  pre-conversion Unity FBX axis.
- Main-menu startup keeps the complete bake for deterministic validation but
  evenly samples the static presentation to 900 visible resources and 3,200
  visible foliage entities. The 186 repeated farms spawn only their completed
  Base/Full primitives, reducing asynchronous scene roots from 285 to 99, and
  boot tracks the exact `Scene(0)` handles consumed after transition. Runtime
  timing logs distinguish the approximately 0.4s asset preload from the
  remaining approximately 12s first-use DX12 scene/material preparation cost.
- Unity shader color properties are now converted from authored sRGB into the
  linear values Bevy's PBR passes require. Terrain, water, grass, building
  detail/emission, placement bounds, and flags therefore retain detail under
  the shipping +1.1 EV/ACES profile instead of clipping into yellow-white.
- Validated, versioned RON configuration (schema 7) and stable authored/runtime
  IDs that do not expose Bevy entity identifiers. Gameplay configuration now
  carries Unity's 5,000-unit starting food/gold/ore/wood balances and zero
  recruits before roster creation, plus Unity's 15,000 food/ore/wood and five-recruit base capacities;
  gold remains intentionally unbounded. Time configuration carries the shipping
  3,600-second day, 66.6% daylight boundary, 100-second dusk/dawn transitions,
  10/5 day/night light intensities, and five-unit building emission ceiling.
- Deterministic source-guided island height generation, occupancy, A* routing,
  dirty regions, grounding data, repeatable world hashes, and a continuous
  636,804-vertex/318,402-triangle Bevy terrain surface generated directly from
  the 200x200 navigation height field.
- A runnable explicit 300-agent ECS benchmark, dynamic obstacles, path
  following, a town hall, resources, a status HUD, Unity-matched perspective
  3D pan/zoom camera controls, collider-backed surface picking, screenshot capture,
  directional lighting, converted representative GLB scenes with primitive
  fallbacks, and an explicit idle/moving visual state machine. Terrain vertex
  colors encode elevation, a translucent water surface uses the authored water
  level, an Avian trimesh collider mirrors the visible surface, and actors,
  resources, buildings, save restores, joins, paths, and selection markers are
  grounded to deterministic centimetre heights.
- Normal new towns now contain exactly Unity's five-member starting NPC roster
  (Defender, Logger, Miner, Gatherer, Builder), with no invented bootstrap enemy;
  the 300-agent workload is isolated to tests and explicit benchmark mode.
  Resource workers choose role-matched stable nodes,
  path to them, gather/deplete their authored `BaseActionAmount`, carry the
  authored `BaseMaxResource` (10 for the shipping resource roles), return to a
  walkable assigned-station approach, and deposit into the authoritative town
  economy; exhausted resource visuals are hidden. Content schema 21 converts each role's
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
- Generated resource nodes retain Unity's tree, ore, bush, and fish target
  identities. Shoreline fish are deterministic invisible food nodes with a
  guaranteed walkable approach, so Fishers cannot harvest bushes and Gatherers
  cannot fish. Completed Farms expose Unity's unlimited food holder to Farmers;
  target selection, manual `!target`, carry limits, XP, depletion, and native
  save fingerprints share the typed contract. Generator version 5 also marks
  land resource cells unwalkable, routes workers to a nearest edge, validates
  the action at that edge, and clears the dirty navigation cell on depletion,
  matching `ResourceProcessor.UpdateAllGraphBounds`/`ClearGraphBounds`. All four
  shipping generation settings leave `SetByDistance` false, so generated nodes
  now use Unity's hard-coded 100-unit amount instead of an invented random range.
  Native generator-v1 through v3 saves are recognized by reconstructed former
  hashes, retain saved depletion, and upgrade without being discarded.
- Unity's data-driven generated-resource reservation rule is live. Stable actor
  ordering grants each active node to one worker, conflicting workers select the
  next compatible node, and claims are reconstructed each frame then released
  after depletion, death, role changes, or retargeting. Farm holders remain on
  the separate station-capacity path, matching Unity's `Targetable` behavior.
- All 15 `CharacterModelHandler` equipment sets are typed using exact converted
  GLB node names. Bevy activates the selected slim role body and permanent tools,
  shields, and helmets; toggles carry-only props from authoritative inventory;
  and drives translated `CarryWood`/`CarryHip` parameters. The content tool shows
  station masks/ranges/capacities and role equipment bindings.
- Content schema 21 preserves every role's exact Unity `AnimationName` action
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
- Presentation schema 15 retains 110 Unity float-property curves with 261 keys
  across 18 clips and all ten authored `PlayRoleActionAudio` events. The runtime
  samples constant, unweighted Hermite, and weighted Bezier segments (all 261
  shipping keys are currently unweighted); the Credits state uses the
  converted seven-panel visibility timeline and end fade, while actor level
  changes use the converted LevelUp position/alpha curves. It also converts the
  reachable `vfx_fireworks` graph's launch/burst rates, capacities, lifetime and
  velocity ranges, HDR color gradient, and both exact Credits scene placements.
  A deterministic Bevy particle runtime replaces the former text-glyph cue with
  visible rockets, delayed flashes, and radial sparks beginning at Unity's
  authored 56-second activation key.
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
  Content schema 21 also promotes Unity `Placeable` and all six technology effect
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
  The authoritative clock now follows Unity's shipping day/night assets; it
  drives eased sky/light transitions, building emission, calendar/season
  boundaries, and a restored camp's night-only spawn gate. New games no longer
  invent a camp: Unity's placement implementation is fully commented out and
  the prefab occurs only in the disabled Necrolands scene, not the four shipping
  scenes. Camp state and presentation remain supported for imported saves.
  Native simulation schema-1
  clocks and user configuration schema 4 upgrade on load without advancing
  gameplay timers.
- Checksummed native RON saves written atomically with backup recovery, plus
  in-game F5/F9 save/load that restores stable actors, constructed buildings,
  dynamic navigation occupancy, town resources, per-node depletion, carried
  inventories, votes, events, and simulation state. Empty depletion maps are
  omitted so native format-1 checksum serialization remains compatible. The
  persistent Town Hall visual, presentation origin, rotation, and lower-left
  grid location now move to its saved footprint during load, keeping rendering,
  navigation, selection, stations, and combat on the same authoritative state.
  Restored players may remain on completed gate cells just as live player paths
  can; every other blocked-cell relocation updates both ECS and simulation actor
  positions so the first post-load tick cannot observe split state.
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
  422 technology objectives in content schema 21, preserves the remaining typed Unity
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
  Presentation schema 19 additionally retains all 110 component/UI property
  curves (261 keys) across 18 clips—including the four transform-free clips—and
  all ten authored animation events. The runtime dispatches each converted
  `PlayRoleActionAudio` event exactly once per crossed clip cycle and plays the
  deterministic source-GUID-guided spatial replacement described above.
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
  imported geometry, rigs, and translation curves. All imported FBX actions are
  exported as named GLB animations. All 253 GLBs are reproducible and tracked
  with Git LFS: 820 meshes, 43 skins, 165 embedded animations, 253 materials,
  one embedded image, and 96,889,124 output bytes.
- Bevy runtime animation binding for compatible converted scenes. Presentation
  schema 13 assigns model-GUID/local-ID stable IDs and exact GLB indexes to 122
  reachable embedded FBX clips. All nine shipping enemy prefabs resolve their
  authored model rig and full controller clip set instead of falling back to a
  default-pose animation. The Player locomotion controller uses the single-skin
  `Characters.glb` scene that owns its 25 embedded clips, resolves stable Idle
  and Walk clip IDs, builds Bevy `AnimationClip` assets from the converted Unity
  tracks, resolves standalone paths by hierarchy suffix onto the compatible GLB
  rest pose, and drives a native `AnimationGraph` across matching bone targets.
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
  and tint controls. Its shoreline-height transition is anchored to the
  shipping waterline and season tint stays live. Runtime-generated terrain now
  spawns as one continuous authored-style render mesh and one full-resolution
  Avian collider. This matches Unity's single `ProceduralMeshGenerator` surface
  closely enough to remove independent chunk normals, LOD edge topology, and
  the resulting cracks; the authoritative height/navigation grid and save
  hashes remain unchanged.
  Explicit schema-1 Unity terrain meshes still reload as one exact retained
  surface rather than being resampled.
- A separate PBR water extension consumes `Env_Water`'s shallow/deep, foam,
  wind, noise, alpha, and ice controls. Its WGSL port animates both authored
  noise textures and restores Unity's winter ice toggle while preserving live
  season/weather tint. The generated 217×217 water mesh encodes terrain depth
  for shallow/deep color and extends eight cells beyond the island as deep
  ocean. Energy-conserving bloom, high roughness, zero reflectance, and bounded
  shader color prevent shoreline highlights from blowing out under ACES. Its
  Unity 0-10 foam cutoff is normalized against averaged dual noise, foam color
  is applied once, and seasonal tint retains most of the authored cyan albedo.
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
  Both scenes render the shipping 2048x1227 `Game_Logo_DropShadow` sprite with
  its source aspect ratio while preserving a text fallback for assetless tests.
  The Main Menu now also reconstructs `UI_MainMenu.prefab`: its left-half tinted
  image panel, copyright line, and five sliced-image New Game, Load Game,
  Settings, Credits, and Quit buttons appear in authored order. Mouse actions
  share the existing typed state/settings paths, and Load Game uses Unity's
  disabled sprite and rejects activation until a native save exists.
  The authored camera, 285 model transforms, and island mesh are fully reflected
  from Unity's left-handed coordinates into Bevy's right-handed coordinates,
  including normals and triangle winding, so the town remains on the right of
  the left-side menu panel. The once-off generator bake now samples all 285
  foundations from the untouched generated surface before flattening, preserving
  four authored town-height terraces instead of propagating adjacent farm/wall
  plateaus. Menu model controllers show completed building/farm variants and
  hide construction and inactive growth meshes. Menu-only exposure is normalized
  by -1.5 EV, its water uses a stable opaque-blue pass, and the 21 cloud layers
  span over seven vertical metres rather than collapsing into one thin sheet.
- Presentation schema 20 converts the reachable `Fish.prefab` field used by the
  Main Menu and town: Fish3 mesh, Critters material, zero start speed,
  120-second lifetime, 40-per-second logical emission, 0.2-1.0 size range,
  rotated 300x300x5 spawn box, two-octave scrolling noise, velocity alignment,
  prewarm, and world-space simulation. It retains the two effectively active
  scene instances and excludes the second town YAML instance under its inactive
  parent prefab.
  Bevy renders a deterministic prewarmed representative budget of 160 shared-
  mesh fish per binding, samples the box only for initial placement, and applies
  bounded smooth noise without adding the full 4,800 logical particles to the
  300-agent performance gate. Primitive extraction reapplies Fish3's raw +Y nose
  correction before velocity alignment, so menu and town fish face travel.
- Credits now recreates both reachable `VFX_FireWorks` instances as a
  deterministic purpose-built particle effect driven by presentation schema 15.
  The converter resolves their exact Unity scene positions and graph GUID, the
  graph data remains engine-neutral RON, and the runtime respects the converted
  `CreditsFireworks` activation curve rather than carrying a hard-coded start.
  Its bottom-right sliced `UI_Button` SKIP control now reproduces the reachable
  scene button and shares Unity's return-to-Main-Menu behavior with Escape.
- The in-game HUD now reconstructs Unity's image-backed top bar from all ten
  packaged TopBar sprites: tiled dark/gold background, four town-resource
  icons, player/building/play-time counters, and the seasonal gauge/meter. The
  counters and 28-day meter position are driven directly from live ECS state;
  migration diagnostics remain available in a compact bottom strip.
- Cell picking now drives an image-backed selection window for actors, resource
  nodes, and complete building footprints. Its text resolves names, roles,
  construction/level state, health, and remaining resources from authoritative
  simulation data, while the three packaged selection-slider sprites provide
  live green/red progress and an assetless fallback remains functional.
- The shipping bottom bar now provides functional Build, Recruit, and Technology
  contexts using converted button, keybind, background, arrow, building, role,
  and technology artwork. Mouse input and Unity's B/R/T shortcuts preserve the
  serialized item ordering, ten-item paging, and live affordability/role-slot/
  storage/vote disabled states; actions reuse the authoritative typed command
  path for placement, recruitment, and voting.
- Shipping technology and ruler vote panels now consume the converted menu,
  slider, prompt, timer, and governance artwork. Technology icons, approval
  shares, vote totals, deterministic top-five candidate/yes-no tallies, and
  save-resumed countdowns read authoritative simulation state; the local vote
  button feeds the same typed `!vote` path used by Twitch.
- Objective-backed technology goals now render through the shipping town-goal
  surface: converted technology icon/title, Unity-equivalent objective wording,
  exact saved progress counts, and packaged objective sliders. The tracker
  follows the authoritative first goal and temporarily yields to an overlapping
  technology ballot without losing state.
- Fish God and raid progress now render through Unity's shared current-event
  panel with the converted background and slider sprites. Praise counts,
  enemies currently alive, and waves actually completed read authoritative
  saved event state; Twitch commands remain the original participation path.
- The reachable Fish God event model now uses its converted shipping Animator
  rather than a static pose: the Belly Flop entry transitions to the looping
  idle, event completion/stoppage sends the typed `Exit` trigger, and the Jump
  exit remains visible for Unity's authored 2.5-second delayed removal. This is
  separate from the unlockable Fish God pet, whose prefab has no Animator.
- The shipping Eyes and Hair materials now resolve their
  `Custom/CharacterSimple` shader to a typed Bevy material and WGSL port. Its
  `_characterTexture * _albedoColor` surface contract is preserved, and the
  per-renderer eye/hair/facial-hair colour updates formerly applied through
  Unity `MaterialPropertyBlock`s now create cached typed-material variants.
  Inherited glTF materials are promoted through the same path before tinting.
- The Age 2 Castle's base and seven individual flag renderer bindings now use a
  typed WGSL port of `Flag.shader`. It preserves vertex-alpha anchoring, the
  authored scrolling-noise and time-rotated displacement, vertex-red
  yellow-to-red color blending, and vertex-red metallic/smoothness falloff. A
  DirectX 12 castle close-up exercises every converted flag slot.
- Priest heals now drive a purpose-built ECS replacement for the reachable
  `VFX_healing` and `VFX_Healing_Channeling` graphs. The target receives the
  Unity-authored 1.2-second healed burst, while the healer emits the serialized
  five-second channel field. Presentation schema 18 now converts both graphs and
  their prefab overrides: capacities, rate, lifetime ranges, burst counts,
  particle-age and five-second size curves, HDR color/alpha gradients, converted
  plus mesh, and disc texture provenance. The runtime samples those records for
  ECS particle budgets, curve motion, mesh selection, and gradient materials;
  automatic respawns, paid `!revive`, and game-master revival use a larger
  revival treatment built from the authored burst geometry. All three remain short-lived shared render entities,
  and a deterministic DirectX 12 smoke scene covers them.
- Combat presentation now preserves role identity instead of rendering every
  ranged attack as the same cube. Rangers and Towers use the converted
  `Arrow.glb`, Wizards use the serialized orange `VFX_Fireball` size/trail
  values, and Necromancers use the violet channel palette. Unity's two-second,
  0.1-width Tower trail and 0.25-second `VFX_HitCharacter` burst drive shared
  short-lived ECS trail/impact entities. Melee damage and every projectile
  arrival emit the appropriate physical, arrow, fire, or necrotic impact, with
  a repeatable DirectX 12 smoke field covering all four styles.
- Content schema 21 promotes every enemy prefab's serialized `TargetSensor`
  mask into typed target IDs. The live selector now chooses the nearest valid
  player or building; the battering ram's building-only mask is preserved, and
  destroyed buildings release their deterministic navigation region. Building
  construction/repair, enemy hits, upgrades, and damage now drive purpose-built
  Bevy effects using the Unity VFX Graph values: 0.5-second work smoke/sparks,
  1.5-second level arrows, and health-scaled persistent fire/smoke over the
  authored 1.4036398 spawn radius. The existing per-building WGSL
  `_DestructionValue` response remains the underlying damage material layer.
- Unity's two land and two underwater foliage layers now convert to typed
  content with all 21 grass, flower, seaweed, and coral variants. Generation is
  deterministic and save-independent, excludes resource and cross-layer
  collisions, respects land/water habitat, applies authored scale plus stable
  transform variation, and renders the converted GLBs with distance culling.
  Building and enemy-camp footprints clear intersecting foliage; deriving that
  visibility from current stable simulation state restores it on removal and
  recomputes it after native save load.
- A source audit of `STSM_Action_PlayerAttack`, `STSM_Action_Attack`, and
  `STSM_Helper_Attack` confirms that shipping Unity combat has one `Targetable`
  and one `TargetHealth.TakeDamage` call per action. The migration therefore
  preserves single-target melee/projectile attacks instead of inventing an
  unauthored area-damage rule.
- Resource workers now honor Unity's town-storage backpressure before acquiring
  or gathering from another node. Full technology-expanded storage sends the
  actor back to its idle route while preserving carried overflow, and gathering
  resumes when capacity becomes available.
- Content schema 21 promotes the shipping Marketplace's reachable
  `PassiveResourceIncrementer`: 0.5 gold/second at level one and 0.25 per
  authored level-up callback. The prefab serializes that callback twice, so the
  deterministic runtime preserves both invocations. Income starts on completed
  construction, clamps through town storage rules, advances resource-gained
  objectives, and retains per-building fractional progress across native saves.
- All nine shipping enemy `ActiveResourceIncrementer` components are now typed
  kill rewards. Combat-caused kills grant their exact authored gold values
  (Goblin 15 through Minotaur Boss 10,000), clamp using the town inventory rule,
  advance resource-gained objectives, and cannot pay twice for an already-dead
  actor. Direct debug/state removal remains unrewarded, matching Unity's
  misleadingly named `killedByPlayer` damage-event gate.
- Content schema 25 converts all 16 reachable `EnemyModelHandler` records: 21
  base, nine permanent, 66 independently optional, and 16 weapon model sets
  comprising 114 stable node references. Each enemy spawn uses its persistent
  actor ID to choose a replay-stable base, optional set, and weapon, hides every
  unselected converted glTF node, and restores the same appearance after load.
  Weapon choice also drives the authored action name, variant count, and
  generic/two-handed `RunAnimationIndex`; enemies without weapon sets use their
  base variant count and `GenericAction`, matching Unity's fallback contract.
  Repository validation parses every referenced GLB JSON chunk and proves that
  each of those authored node names survives the Blender pipeline.
- Content schema 21 promotes all 42 reachable Unity `BuildingModelHandler`
  records and all six `BuildingResourceModelHandler` records with exact glTF
  node names. Bevy now swaps authored construction stages, full and upgrade
  layers, age variants, and empty/half/full storage models instead of rendering
  every imported node simultaneously. Wall and gate adjacency also follows the
  shipping Unity tile-value model/rotation tables; placeholder cubes retain the
  old scale-based construction fallback.
- Content schema 26 promotes both shipping `SimpleRotateOnAxis` records instead
  of special-casing the Windmill: its converted Age 1 blade node rotates around
  local Z and its Age 2 blade node around local Y, both at the authored 35
  degrees/second and only while the matching completed age model is active.
  Repository validation proves both typed node names survive in their packaged
  GLBs, and the focused content tool exposes their axes and rates.
- Content schema 27 promotes the sole shipping `SimpleDisableAfterTime` record
  from `VFX_healing`: the converter and validator preserve its 1.2-second
  lifetime, the focused content tool exposes it, and Bevy's heal-burst and
  revival presentation now use that typed value instead of a duplicated runtime
  constant.
- Content schema 28 moves the reusable rotation contract onto prefab
  archetypes and captures all three shipping `SimpleRotateOnAxis` records. The
  `WorldLoading` state now renders the converted background, overlay, and icon,
  rotating that icon around local Z at its authored 500 degrees/second; the two
  age-specific Windmill rotors continue through the same validated data path.
- Content schema 29 captures the sole shipping `UnitHealthBar` contract from
  `Player_Character.prefab`, including its authored three-second full-health
  hide delay. Bevy now renders a camera-projected red-on-black player health bar
  after damage, keeps it visible while injured, resets its timer after healing,
  and removes it after the authored delay at full health.
- Content schema 30 promotes `LOADER_INITIAL`'s serialized `LoadingManager`
  contract: its 0.5 progress rate, 0.5-second completion hold, and tooltip list.
  `WorldLoading` now presents status, percentage, a progress fill, and a
  deterministic authored tip; it renders the loading screen for a frame before
  synchronous generation and holds the truthful `Ready`/100% state for the
  authored interval before entering the game. The focused tool exposes those
  values and repository validation locks them to the verified Unity baseline.
- The `!rid`, parameterless `!station`, and parameterless `!target` command
  paths now reproduce Unity's `UnitTextDisplay` feedback by projecting matching
  numbered labels over recruits, compatible stations, resources, buildings, or
  actors for the authored 15 seconds. Labels are ordinary short-lived ECS UI
  entities rather than a recreated Unity object pool.
- Completed gates now preserve Unity's asymmetric traversal rule: player A* may
  cross their occupied cells while enemies still see the same cells as blocked.
  Gates under construction remain blocked for everyone, and the exception is
  derived from stable building state after placement and native-save restore.
  The matching presentation path now resolves both `GateWood` and `GateStone`
  controllers from converted data, shares cached Bevy graphs for their exact
  embedded `Open`/`Close` clips, retargets the stone model's `Armature` root to
  the take's `Age02_Gate` target path, and preserves the source 0.25-second
  transitions. Completed, living gates open while at least one living player is
  within the prefab's 4x4 trigger and close after the last player leaves. A
  deterministic `STREAM_TOWN_SMOKE_GATE` capture path covers the opened pose.
- `!ping` now reproduces the reachable `VFXArrowPointer` behavior with the
  converted `PointerArrow.glb`, its exact red emissive material, and the source
  particle system's five-key Hermite size curve and eight-second duration. One
  pointer follows each living actor by stable ID; repeat commands are
  deduplicated, missing/dead actors clean it up, and its converted mesh bounds
  are compensated so the visible base remains anchored above the character.
  `STREAM_TOWN_SMOKE_PING` provides a deterministic close-up capture path.
- Content schema 21 preserves the exact serialized maximum health and per-level
  health increase from every shipping building prefab. Construction, building
  work, repairs, upgrades, construction models, damage materials, and persistent
  damage VFX now share the authoritative level-adjusted maximum. Upgrades retain
  the building's existing damage deficit, and older native snapshots are clamped
  to authored bounds on load.
- A full source/settings search found no per-command cooldown state in the
  shipping Unity Twitch dispatcher; its runtime data retains only last-command
  diagnostics. That unauthored subsystem is no longer listed as parity work.
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
- The focused tools Runtime tab now uses a versioned, opt-in local control
  channel instead of a diagnostic shell. It can launch or attach to the Bevy
  game, inject validated chat commands under a stable debug actor ID, request
  save/load/frame capture/Main Menu/exit actions, and show live state, world
  identity, actor/building/resource counts, Twitch status, acknowledgements,
  and rolling average/p95 frame time. Atomic JSON requests/status never contain
  OAuth credentials or Bevy entity IDs.
- The technology graph tool is now a complete persistent graph-authoring
  workflow. The YAML fallback imports Unity's authored positions for all 363
  nodes and 20 groups into a versioned stable-ID sidecar. The canvas renders
  cross-group edges and supports pan/zoom, node and group dragging, group
  resizing, search highlighting/focus, fit-all, deterministic auto-layout, and
  a clickable minimap with a live viewport. Catalog and layout edits share one
  undo/redo history. Both files use validated atomic writes, backups, reloads,
  and round-trip comparison; cycle and dangling-reference checks still gate
  every catalog mutation.
- The external authoring suite now separates authoritative game configuration
  from per-player settings. It edits every shipping role stat/reference and
  equipment binding, all terrain scale/resource-density inputs, all converted
  foliage noise layers and model variants, and expanded technology references.
  Role, foliage, and technology edits share catalog-wide validation and
  undo/redo. Project configuration and the content catalog use atomic temporary
  files, `.bak` recovery copies, and reload/compare verification. The world lab
  previews elevation/water, occupancy, resources, selected foliage layers, and
  A* routes through the production deterministic generator. A Windows launcher
  and a monitor-free `--validate-authoring` mode are included.
- Generated ground cover now follows Unity's mesh/material batching intent
  through Bevy's native opaque and shadow instancing instead of duplicating
  source geometry. The exact 16,581 generated records map to 12 GPU mesh groups
  and 281 deterministic 32-cell audit groups with zero fallback instances.
  Camera-relative 96-unit range streaming and an 18-unit dither band remove only
  sub-pixel distant ground cover; generation hashes, stable IDs, transforms,
  materials, lighting, shadows, and building/camp clearance behavior are
  unchanged. Runtime console and performance JSON expose logical, active,
  GPU-group, spatial-group, and fallback counts. The 300-agent benchmark now
  defaults to its recorded 16/16 scene/animation detail reference regardless of
  the ordinary game's 64-character detail budget.
- A 300-agent presentation LOD: 16 actors use authored GLB rigs and shared
  animation graphs while the remaining crowd uses lightweight capsule visuals
  without changing authoritative gameplay or persistence. The GPU gate emits
  JSON and exits unattended; `xtask stress` soaks 300 agents for 3,600 ticks
  with repeated dirty-grid mutations. A 1920x1080 release DX12 run on the
  documented reference machine measured 600 post-warmup frames at 11.47 ms
  average and 16.50 ms p95 for the corrected 200x200 world with all 16,581
  generated ground-cover records active, below the 16.7 ms budget. The former
  64x64 chunked-world timing is retired.
- Reachable unit facing now follows Unity's `RotationHandler`: agents slerp
  toward travel or actor/building action targets at the prefab's authored five
  radians per second, while gathering preserves its explicit immediate snap to
  the resource. Facing remains presentation-only and does not enter stable save
  state or deterministic navigation hashes.
- Live pets now reproduce the shipping `Pet.Update` follow behavior with the
  authored distance-squared speed remap, 10-unit cap, and five-radians-per-
  second smoothed facing. Red Panda, Giraffe, Duck, and Butterfly resolve their
  individual Unity controllers and retarget exported idle/walk transform tracks
  onto their own converted GLB rigs. Fish God intentionally stays static because
  its shipping model has no Animator.
- Content schema 31 promotes the remaining reachable enemy `TargetSensor`
  contract into neutral data. All nine shipping enemy archetypes now use their
  authored acquisition radius (four logical cells for Goblin and 12.5 for the
  other eight) and `attackAttacker` behavior. A damaged enemy retains a valid
  player or building attacker outside its normal acquisition radius, and the
  stable target survives native serialization. The migration converter, asset
  validator, runtime tests, and focused archetype inspector all cover the new
  fields.
- Resource workers and `!target` now consume the assigned station's actual
  target catalog. Candidate resources/farms are constrained by the station's
  authored rectangular search region, per-target-kind list capacity, compatible
  masks, and path reachability; deterministic distance/ID ordering replaces
  Unity's frame-order tie behavior. Specialized stations can no longer leak to
  valid-but-out-of-range world resources, while the Town Hall retains its
  authored 30-target fallback catalog. Combat, healing, and construction keep
  Unity's separate nearest-target sensor path.
- Content schema 32 converts all 13 policies from Unity's shipping
  `D_TargetSettings.asset` and the runtime now reproduces the station cache
  lifecycle rather than rescanning the world on demand. Each station begins at
  a deterministic stagger within its authored interval, at most one expired
  station cache is repopulated per rendered frame, `Update` retains valid
  tree/ore/bush/farm entries while filling vacancies, `Clear` rebuilds the
  other nine target lists nearest-first, and one queued reachability entry per
  station is revalidated on Unity's two-second cadence. Autonomous work,
  Twitch `!target`, and direct recruit orders share the same live cache. The
  authored 2.5-second inventory-deposit delay is also restored.
- Agent actions now follow Unity's state cadence instead of mutating the world
  on arrival and cooling down afterward. Each gather, farm, build, attack,
  heal, and deposit action starts its authored animation/timer first and
  invokes the simulation effect only when that timer expires. A carrier whose
  personal inventory and town storage are both full walks to the Town Hall and
  waits with the resource instead of abandoning its return loop. Movement now
  adds deterministic predictive yielding for following, crossing, and head-on
  agents ahead of the existing bounded local separation; stable IDs resolve
  right-of-way without reciprocal deadlock. Both separation and yielding counts
  are exposed in the external runtime console, and the 300-agent soak remains
  below its acceptance budget.
- Twitch command dispatch now reproduces Unity's complete ten-name character-
  creation alias set (including its historical misspellings), command-specific
  validation usages, no-character rejection, player/global attribution, exact
  high-traffic role/health/progression/station/target/cosmetic/pet/recruit/help
  wording, C# boolean casing, and intentionally silent success paths. Malformed
  registered commands are no longer discarded by the IRC bridge; unknown
  commands remain silent as in Unity. Tests pin the source-authored usages and
  reply routing independently of a live Twitch account.
- Animated players now cast and receive shadows without the previous
  hierarchy-wide `NotShadowReceiver` workaround. Every Standard surface below
  the single shipping armature is promoted through a cached typed character
  material, preserving imported textures, vertex colours, alpha state, and the
  existing cosmetic recolour contract. Its fragment path applies a small
  character-only world-normal receiver offset; terrain, foliage, and global
  directional-light bias are unchanged. The first-frame reveal gate verifies
  that every animated-player mesh uses the compatible receiver, that none is
  suppressed, and that all resulting material pipelines are GPU-ready. The
  reference close-up proved 615/615 initial player primitives ready with zero
  failed pipelines while the run controller completed eight uninterrupted
  wraps across sampled, cleanly shaded poses.
- The animation-event source audit is closed. All twelve shipping events are
  now converted: ten standalone `PlayRoleActionAudio` events plus the two
  normalized FBX-importer `ToggleOn`/`ToggleOff` events on
  `CharacterFishing`. The latter drive only the authored catch window
  (7.545s–13.203s), reset when the fishing take exits, and leave the ordinary
  carried-resource prop restricted to the return walk. Converter, catalog,
  timing, and runtime dispatch tests pin the complete event surface.
- Worker inventory now retains Unity's independent per-resource buckets across
  role changes. Capacity checks, carry animation parameters, and equipment
  visibility inspect only the current role's resource, while a station deposit
  transfers only that resource and leaves any previous role's cargo untouched.
  Bounded town storage still leaves current-role overflow on the actor. Domain
  and runtime regressions cover a Logger-to-Miner switch, capacity isolation,
  role-specific presentation, partial deposit, and retained Wood.

## Not yet at parity

- Final source-diff closure for every reachable balance edge case outside the
  live worker/combat/healing/construction and role-specific inventory loops. All nine shipping enemy combat archetypes, the
  weighted night camp spawner, raid waves/boss, retaliation, and prefab-specific
  target acquisition ranges are now live. The source audit
  confirms the Tower contains the shipping project's sole `ProjectileShooter`;
  there is no second reachable combat building to reproduce.
- Final art-direction polish, curated screenshot/audio acceptance baselines,
  assistive-technology certification on the release hardware matrix, and source-diff closure for any
  presentation behavior only reachable through missing media. A shipping-scene
  reachability audit found no additional custom shaders, VFX prefabs, or UI
  sound bindings to port: the remaining custom shader/VFX assets belong to
  Necrolands or development test scenes, and the repository contains no audio
  media files. Authored global post-processing, menus/HUD, particles, procedural
  replacement audio, and persisted presentation settings are live.
- Legacy target, active/unlocked pet, and customization data now map into native
  actor state and live presentation. Pet locomotion and per-model animation are
  live; remaining pet work is presentation polish rather than static follow.
- Curated screenshot baselines, signed/notarized distribution, and a hosted
  Windows release. The headless launch-through-credits acceptance suite,
  unsigned CI/local Windows archive, and measured reference-machine 60 FPS gate
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
now also has complete presentation metadata, packaged textures, 122 indexed
embedded FBX clips, full shipping-enemy controller binding, 57 converted standalone transform clips, Player locomotion
retargeting, a typed controller interpreter, 19-clip Player blend-graph playback,
the generated heightfield/water/collider surface, inherited prefab material
bindings, runtime PBR material reconstruction, simulation-driven season/weather
presentation, and the live stable command grammar with constructed-building
persistence, plus autonomous role-driven gathering/deposit and persistent node
depletion, connected actor spawning, live combat/death/respawn, health-staged
Builder construction, technology-gated upgrades, and typed technology discounts,
storage, stat, and building-age effects, production command wording, saved
player settings, a complete runtime settings/menu workflow, and
camera-projected player/building overlays. The shipping town seagull also flies
its authored 32-second boundary route using the converted model and emits three
generated ambience calls at the source's random cadence and rolloff. The
shipping grass, critter, and castle-flag custom materials now have typed WGSL
ports backed by converted renderer bindings. Shipping-scene particle and custom
shader reachability is closed; remaining presentation work is final art tuning,
release-matrix accessibility certification, and broader curated
screenshot/audio acceptance coverage.
Gameplay parity is materially advanced but still needs final source-diff closure;
presentation and hardening remain long-term work.
