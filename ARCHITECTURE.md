# Stream Town architecture

Stream Town is a native Bevy workspace. Runtime behavior is expressed as ECS
state, resources, components, messages, observers, plugins, and ordered system
sets; authored gameplay data is loaded from validated RON catalogs.

## Workspace boundaries

- `stream_town_domain` owns deterministic simulation, commands, configuration,
  authored content schemas, world generation, and native save/settings formats.
  It does not depend on Bevy rendering or Twitch transport.
- `stream_town_game` adapts the domain to Bevy. `app.rs` is the compact
  composition root; lifecycle schedule ownership is split across the plugins in
  `app/`. `runtime/` contains focused ECS features such as city timelapse, and
  the broadcast, profiling, music, and Twitch modules own their respective
  external boundaries.
- `stream_town_tools` is the native authoring application for game balance,
  buildings, roles, technology, assets, terrain, music, and validation.
- `stream_town_ffmpeg_bridge` isolates the native FFmpeg ABI needed by direct
  broadcasting.
- `xtask` contains repeatable validation, packaging, acceptance, stress, and
  explicit town-maintenance commands.

## Data flow

Authored RON is validated into domain types. Bevy resources hold the loaded
configuration, catalogs, and authoritative simulation. Input adapters translate
Twitch, operator, and local UI input into typed domain commands. Systems mutate
the simulation, and presentation systems reconcile ECS entities from that state.

Cross-feature reactions should use typed Bevy messages or observers. For
example, a confirmed build emits `CityTimelapseBuildConfirmed`; the timelapse
plugin consumes it without the command processor knowing timer internals.
Systems that require strict ordering belong to named `SystemSet`s and focused
lifecycle plugins under `app/`.

## Design rules

- Keep deterministic gameplay and serialization in `stream_town_domain`.
- Keep rendering, audio, OS integration, Twitch, and native encoders in focused
  game plugins or adapters.
- Prefer messages for one-to-many facts and observers for entity-scoped
  reactions; avoid direct mutation of another feature's private runtime state.
- Keep plugins responsible for registering their own resources, messages,
  observers, and systems.
- Store tunable values in `assets/config/*.ron` or `assets/content/*.ron`, not in
  Rust, unless they are implementation invariants.
- Preserve native save-schema upgrades needed by existing towns. Historical
  source-engine provenance in authored data may remain metadata, but no runtime
  or tool may require the retired source project.
- Put tests in focused files under `src/tests/` rather than embedding them in
  runtime source or collecting them in a monolith.

## Verification

```powershell
cd bevy-port
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p xtask -- validate
```

After source changes, run `graphify update .` from the repository root so the
checked-in architecture graph remains current.
