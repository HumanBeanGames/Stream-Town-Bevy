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
| Models, textures, materials, animation, buildings, roles, resource/foliage/camp generation | `assets/content/catalog.ron`, `assets/content/presentation.ron`, `assets/user/models/*.glb`, `assets/migrated/textures/user/*` | None |
| Technology content, vote requirements, and graph layout | `assets/content/catalog.ron`, `assets/content/technology_layout.ron` | None |
| Player settings | None | `.stream-town/settings.ron`, edited from the local operator panel or game menus |
| Twitch accounts and credentials | None | Main Menu > Secrets and the OS credential vault |

`Save + apply to game` changes the source-controlled baseline and the local
runtime override together. The runtime override takes precedence and is read on
the next game launch, so authority values such as day duration no longer depend
on rebuilding the executable. `Save baseline only` and `Apply locally only` are
available when that separation is intentional. The tool never writes Twitch
access or refresh tokens to RON.

## Workflows

- Game Authority edits window, boot, simulation cadence, starting resources,
  capacities, time-of-day lighting, and emission settings.
- Models + Assets is split into Models, Textures, Materials, and Animations.
  It imports GLBs and images without overwriting existing files; creates,
  duplicates, edits, and removes archetypes, scenes, texture records, PBR
  materials, clips, controllers, layers, states, transitions, parameters,
  curves, events, and renderer bindings; and uses parsed GLB nodes, materials,
  and animation takes for typed choices. Live offscreen previews apply the same
  renderer/model/fallback material precedence as the game. Asset editors and
  long choice popups scroll independently of the application window. Drag to
  orbit, Shift+drag or right-drag to pan relative to the current camera facing,
  use the wheel to zoom, and use the playback, looping, speed, restart, and reset
  controls for animation clips. It does not duplicate building, role, or
  technology records.
- Buildings provides template-based creation, an interactive logical placement
  footprint editor with rotate/row/column controls and atomic model/runtime
  synchronization, visual GLB inspection,
  construction and level costs, storage, role capacity, passive production,
  station and target settings, projectiles, and model hierarchy bindings. Every
  stable reference is chosen from the loaded catalog.
- Roles edits every shipping role's balance/progression values, stable resource,
  station, target and ability references, Animator action contracts, character
  body nodes, hand items, helmets, and carry behavior. Role/model fields use
  catalog and converted-hierarchy choices instead of free-form reference text.
  Its live composed-character preview uses the shipping character rig and
  authored materials, applies the selected body/cosmetics/role equipment, and
  plays any player Animator state with the normal playback and camera controls.
  Roles can be duplicated; deletion is allowed only when the full catalog has
  no remaining reference.
- Technology makes the full-width graph canvas the primary surface. Cards show
  prerequisite, unlock, vote-requirement, building, economy, and stat summaries;
  hovering exposes the complete record, while selecting opens a movable editor
  over the canvas. Drag output-to-input sockets to add prerequisite edges.
  Select an edge and press Delete/Backspace, right-click it, or click its red
  midpoint × to remove it; searchable prerequisite chips remain another removal
  route. The same tab creates, edits, and safely
  removes typed vote-requirement records. It edits groups, descriptions, age/tier
  placement, explicit unlocks,
  icons, building caps/unlocks/cost reductions, storage boosts, building ages,
  global/role stat boosts, and availability. Use the wheel to zoom, middle-drag or
  Space+primary-drag to pan, drag nodes or group headers to move them, and drag
  a group's lower-right handle to resize it. Search highlights every matching
  node and selecting a search result focuses it. Fit-all, deterministic
  auto-layout, cycle/dangling-reference checks, and a clickable/drag-recentered
  minimap are included.
- World + Nav edits the terrain seed, dimensions, spatial scale, height/water
  levels and resource density; land and shoreline-water resource rules; every
  foliage layer's Unity-compatible noise, habitat, spacing, material, GLB
  variants, and base scales; and enemy-camp placement rules. Resource, foliage,
  and camp archetype fields are catalog/search backed. Preview modes show
  elevation/water, navigation occupancy, land resources, water resources,
  selected foliage layers, camp placement ranges, and live 3D models. The
  redundant point-to-point A* probe has been removed.
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

## Future role behavior scripting (not implemented)

External behavior scripting is a useful extension point for roles such as a
forester, but it should not expose Bevy's `World` or transient entity IDs. Keep
`RoleDef` as the declarative, validated contract and add an optional,
versioned behavior module that receives stable-ID observations and returns a
small set of deterministic commands such as select target, move, act, deposit,
or retry. File/network access, unbounded execution, and direct ECS mutation
should remain unavailable so saves, replays, Twitch outcomes, and world hashes
stay reproducible.

Lua via `mlua` is the easiest author-facing option and has excellent hot-reload
ergonomics. Sandboxed WebAssembly offers a stronger capability boundary and
better typed Rust interfaces, but asks more of content authors. Before choosing
either runtime, the next design pass should specify the callback/command API,
instruction budget, persistent script-state schema, validation diagnostics, and
hot-reload behavior, then prototype the same small role (for example, a
forester) in both formats. No scripting dependency or runtime has been added by
the current authoring work.
