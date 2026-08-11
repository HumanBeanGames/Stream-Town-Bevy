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
building footprints from Unity's authored two-unit grid sizes and follows nested
prefabs to their source FBX models. The same command copies all 133 reachable
textures and emits `presentation.ron`: 33 material definitions, 75 clip records,
31 controller definitions, 94 stable states, 165 transitions, and inherited
prefab/controller/model bindings. The YAML fallback converts 57 standalone
`.anim` files into 1,196 stable transform tracks; four property/UI-only clips do
not contain transform curves. It also retains 11 authored 1D blend states and
typed transition conditions; the stale `Slam` and `Swipe` conditions become
provenance-marked inferred parameters. Renderer inheritance resolves to 141
prefab material bindings and 181 material slots. The known missing Necrolands
camera clip is an explicit validated record.

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

In game: use WASD to pan, Q/E to zoom, left-click to select a grid cell,
J to inject a parsed `!join`, F1/F2 to disconnect/reconnect Twitch, F5/F9 to
save/load, F12 to capture a screenshot, and Escape to return to the menu. The
vertical slice renders the deterministic navigation height field as a colored
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
Compatible embedded GLB clips use Bevy animation graphs (currently the shipping
Goblin path). The Player controller builds 19 converted clips retargeted onto 23
bones in the Player GLB rest pose. An engine-independent interpreter evaluates
typed parameters, trigger consumption, direct transitions, exit gates, and 1D
threshold blending; runtime movement feeds the authored velocity/5 `Move Speed`
parameter into Idle/Walk/Run. Converted renderer descendants receive a cached
Bevy PBR approximation of their first inherited Unity material, including
PNG/TGA base textures, color, emission, metallic, smoothness, and alpha settings.
Nested controller layers/state machines, complete gameplay action emitters,
exact Unity curve tangents, property/UI animation, exact multi-slot assignment,
and custom WGSL shader parity remain presentation work.

This is an early migration milestone, not a parity release. The repository-level
[`MIGRATION_STATUS.md`](../MIGRATION_STATUS.md) lists implemented behavior and
remaining work.
