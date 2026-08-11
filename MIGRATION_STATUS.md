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
- Validated, versioned RON configuration and stable authored/runtime IDs that do
  not expose Bevy entity identifiers.
- Deterministic island height generation, occupancy, A* routing, dirty regions,
  grounding data, and repeatable world hashes.
- A runnable 300-agent ECS simulation with one enemy, dynamic obstacles, path
  following, a town hall, resources, a status HUD, pan/zoom camera controls,
  click-to-select grid picking, and an explicit idle/moving visual state machine.
- An injected `!join` vertical slice that goes through the shipping command
  parser and creates both the stable domain actor and its visible ECS entity.
- Engine-independent gathering, depositing, construction, roles, technology
  voting, trade, combat, death/respawn, timed events, days, seasons, and weather.
- Checksummed native RON saves written atomically with backup recovery, plus
  in-game F5/F9 save/load that restores stable actors and simulation state.
- A one-time legacy importer for JSON and the exact Unity binary field order in
  schemas 1-3, including container/compression/bounds/trailer validation, named
  backup recovery, schema-1 mesh retention, seed-based Bevy regeneration,
  stable IDs, deterministic surface relocation, atomic native output, SHA-256
  provenance, and post-write reload verification. Legacy input is never modified.
- A Unity asset inventory/validator that resolves `.meta` GUIDs and YAML
  references. The first scan found 1,429 source assets: 61 clips, 31 animator
  controllers, 33 materials, 253 models, 215 prefabs, 10 scenes, 654
  ScriptableObjects, 16 shaders, 133 textures, 17 VFX assets, and 6 other files.
- A focused Bevy/egui tool application with the planned eight work areas and an
  embedded ECS inspector. The World + Navigation tab has a working deterministic
  seed preview; several other tabs are presently diagnostic/authoring shells.
- Windows CI covering formatting, compilation, Clippy, tests, and repository
  validation.

## Not yet at parity

- Unity prefab/ScriptableObject export with variant and override flattening, the
  A* compatibility exporter, RON content generation, and the Blender-to-GLB
  conversion pipeline.
- The production terrain renderer, production-grade actor steering, complete
  role/building/resource/station/inventory/equipment behavior, and every
  reachable balance rule from the Unity scenes.
- Twitch IRC/OAuth networking, reconnect and rate limiting, and OS credential
  storage. The command grammar and deterministic injection path exist only.
- Animation graph/controller conversion, rigged production models, WGSL shader
  ports, VFX, UI parity, post-processing, replacement audio, and accessibility.
- Rendering schema-1 retained terrain meshes and full semantic reconstruction of
  legacy target/station/pet/customization data. The importer currently preserves
  or maps gameplay-critical world, entity, inventory, economy, and technology
  fields; unsupported presentation/relationship fields are consumed and
  validated but not represented in the native runtime yet.
- Full technology-graph editing, undo/redo, minimap interaction, live runtime
  bridging, release packaging, frame capture, and profiling controls.
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

## Milestone interpretation

Foundation is substantially implemented. The vertical slice is runnable and its
deterministic 300-agent navigation gate passes, but it is still missing rigged
production animation, connected Twitch transport, converted content, and the
recorded reference-machine GPU measurement required to close that milestone.
Gameplay parity, presentation, and hardening remain long-term work.
