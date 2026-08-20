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
| Roles, foliage, and technology | `assets/content/catalog.ron` | None |
| Player Settings | None | the platform player-settings path shown in the tool |
| Twitch credentials | public connection fields only | OS credential vault |

`Save project baseline` changes the source-controlled defaults embedded in the
next game build. `Write local runtime override` is intended for rapid testing;
it takes precedence when the game starts from the same working directory. The
tool never writes Twitch access or refresh tokens to RON.

## Workflows

- Game Authority edits window, boot, simulation cadence, starting resources,
  capacities, time-of-day lighting, and emission settings.
- Roles edits every shipping role's balance/progression values, stable resource,
  station, target and ability references, Animator action contracts, character
  body nodes, hand items, helmets, and carry behavior. Roles can be duplicated;
  deletion is allowed only when the full catalog has no remaining reference.
- Technology edits groups and nodes, descriptions, age/tier placement,
  prerequisites, explicit unlocks, objectives, icons, and availability. The
  minimap, search, cycle detection, and dangling-reference validation are live.
- World + Nav edits the terrain seed, dimensions, spatial scale, height/water
  levels and resource density plus every foliage layer's Unity-compatible noise,
  habitat, spacing, material, GLB variants, and base scales. Preview modes show
  elevation/water, navigation occupancy, generated resource kinds, and selected
  foliage layers. Path probes call the production A* implementation.
- Player Settings edits the complete video, audio, camera/input, interface, and
  autosave schema used by the game.
- Migration, Content, Runtime, Twitch, Validation, and ECS Inspector retain the
  existing conversion, diagnostics, live-control, OAuth, packaging, and
  inspection workflows. The Twitch tab validates/refreshes the vault token,
  proves an authenticated channel join, resolves logins to stable game-master
  IDs, and captures the next redeemed Channel Points reward ID.

The account-bound Twitch setup sequence is documented in
[`TWITCH_SETUP.md`](../TWITCH_SETUP.md).

## Safe persistence

Role, foliage, and technology changes are drafts until `Apply validated ...` is
pressed. Applying clones and validates the complete catalog before replacing the
in-memory version. Undo/redo is shared across those tabs. Saving first validates,
writes and synchronizes a `.tmp` file, preserves the previous file as `.bak`,
moves the new file into place, reloads it, validates it again, and compares the
round trip with the in-memory value. Game configuration uses the same sequence.

After saving project data, run the Validation tab or:

```powershell
cargo xtask validate
```
