# Stream Town Bevy

This workspace is the engine-independent rewrite of Stream Town. The Unity
project at the repository root is frozen migration input; new runtime work lives
here.

## Binaries

- `stream_town_game`: shipping Bevy application and the 300-agent vertical slice.
- `stream_town_tools`: focused content, migration, world-generation, navigation,
  Twitch, validation, and runtime tooling. Its catalog browser, validated
  technology editor with undo/redo, and occupancy/path lab are functional;
  prefab/archetype browser, runtime, and Twitch panels remain diagnostic shells.
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
cargo run -p stream_town_migrate -- convert-content generated/unity-export.json --out-dir assets/content
cargo run -p stream_town_migrate -- validate-models assets/migrated/models/model-conversion.json --repository-root .. --expected-count 253
cargo run -p stream_town_migrate -- import-save StreamTownSave.stsave --out generated/imported.stbevy --config assets/config/game.ron
```

Generate the ignored neutral Unity export from the repository root with the
exact editor version recorded by the project:

```powershell
.\bevy-port\scripts\export-unity.ps1
cd bevy-port
cargo run -p stream_town_migrate -- validate-unity-export generated/unity-export.json
cargo run -p stream_town_migrate -- convert-content generated/unity-export.json --out-dir assets/content
```

The editor exporter resolves GUIDs, object references, prefab sources and
overrides, ScriptableObject data, and the four shipping scene hierarchies. Its
migration-only A* types are inert compile stubs and are not navigation code.
The content conversion selects the active Unity containers and emits a validated
catalog of 26 production buildings, 215 prefab archetypes, 288 model scene
variants, 15 roles, and the 363-node shipping technology graph. It derives
building footprints from Unity's authored two-unit grid sizes and follows nested
prefabs to their source FBX models.

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

In game: use WASD to pan, Q/E to zoom, left-click to select a grid cell,
J to inject a parsed `!join`, F5/F9 to save/load, F12 to capture a screenshot,
and Escape to return to the menu. The vertical slice renders a 3D terrain plane,
lighting, converted GLB scenes for the representative town hall and actors, and
primitive fallbacks when an asset is unavailable.

This is an early migration milestone, not a parity release. The repository-level
[`MIGRATION_STATUS.md`](../MIGRATION_STATUS.md) lists implemented behavior and
remaining work.
