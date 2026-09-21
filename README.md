# Stream Town

Stream Town is a Bevy-based town simulation controlled through Twitch chat. The
shipping Rust workspace, authored data, runtime assets, tools, and deployment
scripts live in [`bevy-port`](bevy-port/README.md).

For both required Twitch accounts, follow the [Twitch setup walkthrough](TWITCH_SETUP.md).
The game pauses town gameplay until Twitch confirms the stream is publicly live.
To connect the optional announcement bot, follow the
[Discord bot setup walkthrough](DISCORD_SETUP.md). It covers setup through the
game, event choices, and troubleshooting; no coding is required.
Both setup tabs in the game and Tools link to their guides. The Twitch tab also links
to the [complete command guide](TWITCH_COMMANDS.md).

The former Unity source and one-time conversion pipeline have been retired. The
checked-in RON catalogs and GLB/texture assets are now the project’s native
authoring source. Historical engine provenance retained inside those catalogs is
metadata only and is not a build-time dependency.

## Development

```powershell
cd bevy-port
cargo run -p xtask -- validate
cargo test --workspace
cargo build --release --workspace
.\scripts\launch-tools.ps1
.\scripts\launch-game.ps1
```

For the crate boundaries and ECS conventions, see
[`ARCHITECTURE.md`](ARCHITECTURE.md). Twitch setup is documented in
[`TWITCH_SETUP.md`](TWITCH_SETUP.md), and the public command list is in
[`TWITCH_COMMANDS.md`](TWITCH_COMMANDS.md). The optional Discord bot is configured
in the game's Connections screen, with event switches in Settings > Discord.
Bevy Tools supports the same configuration; see [`DISCORD_SETUP.md`](DISCORD_SETUP.md).

All Tools-authored data loads at runtime. Saving footprints, models, materials,
roles, and other existing tool fields never requires recompilation. Restart the
game to load saved catalog edits. See the
[runtime authoring audit](bevy-port/docs/runtime-authoring-audit.md).

## License and media

The code is licensed under GPL-3.0-only. Some original soundtrack files were not
redistributable and are not included; the Bevy runtime provides its own adaptive
score and procedural ambience.

The original student-project design documents remain useful historical context:

- [Game Design Document](https://docs.google.com/document/d/1vqnT3kpXjyaRRbm3R8cjF-40W9evqaksKq3ECvn-7AM/edit?usp=share_link)
- [Technical Design Document](https://docs.google.com/document/d/12nDNNhhOYKf_gBjGFj0iP7-hvdjTWxNs_fvL98HrrQ8/edit?usp=share_link)
- [Art Bible](https://docs.google.com/presentation/d/1VG11I8TGiMwpkelAd9ERPDhKpsiAkukuqHoUEt8hr7E/edit?usp=share_link)
