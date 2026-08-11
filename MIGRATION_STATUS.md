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
  references, plus a Unity 6000.5.6f1 editor exporter for serialized fields,
  object references, prefab sources and overrides, and the four shipping scene
  hierarchies. The verified export contains 1,429 assets; 61 obsolete or
  missing-script ScriptableObjects require the YAML fallback because Unity did
  not expose a main object.
- Explicit migration-only A* API stubs sufficient to compile and run the editor
  exporter. They are intentionally inert and are not used by the Bevy runtime.
- A deterministic semantic RON converter that follows the active Unity
  containers to select 27 buildings, 15 roles, and the shipping 363-node,
  362-edge technology DAG. It preserves typed Unity fields as provenance,
  validates stable IDs/references/cycles, and reloads its own RON output.
- A focused Bevy/egui tool application with the planned eight work areas and an
  embedded ECS inspector. The World + Navigation tab has a working deterministic
  seed preview; several other tabs are presently diagnostic/authoring shells.
- Windows CI covering formatting, compilation, Clippy, tests, and repository
  validation.

## Not yet at parity

- Semantic prefab/archetype conversion, accurate building footprints,
  animation controller translation, and the Blender-to-GLB conversion pipeline.
  The neutral Unity export preserves prefab source/override records rather than
  claiming those records are already production Bevy archetypes.
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

Generate and validate the ignored, machine-local editor export with:

```powershell
.\bevy-port\scripts\export-unity.ps1
cd bevy-port
cargo run -p stream_town_migrate -- validate-unity-export generated/unity-export.json
cargo run -p stream_town_migrate -- convert-content generated/unity-export.json --out-dir assets/content
```

## Milestone interpretation

Foundation is substantially implemented. The vertical slice is runnable and its
deterministic 300-agent navigation gate passes, but it is still missing rigged
production animation, connected Twitch transport, converted content, and the
recorded reference-machine GPU measurement required to close that milestone.
Gameplay parity, presentation, and hardening remain long-term work.
