# Stream Town external authoring suite

`stream_town_tools` is the focused replacement for the Unity inspectors and
custom editor windows needed by the shipping game. It deliberately does not try
to recreate Unity's generic scene, prefab, ProBuilder, Odin, or DI debugging
interfaces.

## Launch

From the repository root on Windows:

```powershell
.\bevy-port\scripts\launch-tools.ps1
```

Use `-Release` for a faster optimized executable. The tool resolves project
asset paths from its Cargo package, so the launcher is safe to invoke from any
working directory. A window/monitor is not required for the data-only gate:

```powershell
.\bevy-port\scripts\launch-tools.ps1 -ValidateOnly
```

The equivalent commands from `bevy-port` are:

```powershell
cargo run -p stream_town_tools
cargo run -p stream_town_tools -- --validate-authoring
```

## Authoritative files

| Workflow | Project data | Local/runtime data |
|---|---|---|
| Game Authority and World + Nav | `assets/config/game.ron` | `.stream-town/config.ron` |
| Buildings, roles, and foliage | `assets/content/catalog.ron` | None |
| Technology content and graph layout | `assets/content/catalog.ron`, `assets/content/technology_layout.ron` | None |
| Player settings | None | `.stream-town/settings.ron`, edited from the local operator panel or game menus |
| Twitch accounts and credentials | None | Main Menu > Secrets and the OS credential vault |

`Save project baseline` changes the source-controlled defaults embedded in the
next game build. `Write local runtime override` is intended for rapid testing;
it takes precedence when the game starts from the same working directory. The
tool never writes Twitch access or refresh tokens to RON.

## Workflows

- Game Authority edits window, boot, simulation cadence, starting resources,
  capacities, time-of-day lighting, and emission settings.
- Models + Assets is a focused visual browser for converted GLB variants,
  renderer/material bindings, textures, Animator controllers, and clips. It no
  longer duplicates building, role, or technology records.
- Buildings provides template-based creation, visual footprint/GLB inspection,
  construction and level costs, storage, role capacity, passive production,
  station and target settings, projectiles, and model hierarchy bindings. Every
  stable reference is chosen from the loaded catalog.
- Roles edits every shipping role's balance/progression values, stable resource,
  station, target and ability references, Animator action contracts, character
  body nodes, hand items, helmets, and carry behavior. Role/model fields use
  catalog and converted-hierarchy choices instead of free-form reference text. Roles can be duplicated;
  deletion is allowed only when the full catalog has no remaining reference.
- Technology makes the graph canvas the primary surface and keeps the complete
  selected-node inspector beside it. It edits groups, descriptions, age/tier
  placement, prerequisites, explicit unlocks, objectives and requirements,
  icons, building caps/unlocks/cost reductions, storage boosts, building ages,
  global/role stat boosts, and availability. Use the wheel to zoom, middle-drag or
  Space+primary-drag to pan, drag nodes or group headers to move them, and drag
  a group's lower-right handle to resize it. Search highlights every matching
  node and selecting a search result focuses it. Fit-all, deterministic
  auto-layout, cycle/dangling-reference checks, and a clickable/drag-recentered
  minimap are included.
- World + Nav edits the terrain seed, dimensions, spatial scale, height/water
  levels and resource density plus every foliage layer's Unity-compatible noise,
  habitat, spacing, material, GLB variants, and base scales. Preview modes show
  elevation/water, navigation occupancy, generated resource kinds, and selected
  foliage layers. The redundant point-to-point A* probe has been removed.
- Migration and Validation retain conversion, data-integrity, and packaging
  workflows. Runtime control, player settings, Twitch setup, and inspection are
  no longer duplicate tool tabs: Twitch setup lives in Main Menu > Secrets, and
  the local operator panel owns stream telemetry, player settings, Twitch chat,
  and moderation.

The account-bound Twitch setup sequence is documented in
[`TWITCH_SETUP.md`](../TWITCH_SETUP.md).

## Safe persistence

Role, foliage, and technology changes are drafts until `Apply validated ...` is
pressed. Applying clones and validates the complete catalog before replacing the
in-memory version. Graph-layout moves validate against the same stable-ID
catalog. Undo/redo is shared across the catalog and layout, so one operation
restores both halves of authoring state. Saving first validates, writes and
synchronizes a `.tmp` file, preserves the previous file as `.bak`, moves the new
file into place, reloads it, validates it again, and compares the round trip with
the in-memory value. Game configuration uses the same sequence.

After saving project data, run the Validation tab or:

```powershell
cargo xtask validate
```
