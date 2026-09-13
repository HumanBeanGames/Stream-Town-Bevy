# Stream Town

Stream Town is a Bevy-based town simulation controlled through Twitch chat. The
shipping Rust workspace, authored data, runtime assets, tools, and deployment
scripts live in [`bevy-port`](bevy-port/README.md).

The former Unity source and one-time conversion pipeline have been retired. The
checked-in RON catalogs and GLB/texture assets are now the project’s native
authoring source. Historical engine provenance retained inside those catalogs is
metadata only and is not a build-time dependency.

## Development

```powershell
cd bevy-port
cargo run -p xtask -- validate
cargo test --workspace
cargo run -p stream_town_tools
.\scripts\launch-game.ps1
```

For the crate boundaries and ECS conventions, see
[`ARCHITECTURE.md`](ARCHITECTURE.md). Twitch setup is documented in
[`TWITCH_SETUP.md`](TWITCH_SETUP.md), and the public command list is in
[`TWITCH_COMMANDS.md`](TWITCH_COMMANDS.md).

## License and media

The code is licensed under GPL-3.0-only. Some original soundtrack files were not
redistributable and are not included; the Bevy runtime provides its own adaptive
score and procedural ambience.

The original student-project design documents remain useful historical context:

- [Game Design Document](https://docs.google.com/document/d/1vqnT3kpXjyaRRbm3R8cjF-40W9evqaksKq3ECvn-7AM/edit?usp=share_link)
- [Technical Design Document](https://docs.google.com/document/d/12nDNNhhOYKf_gBjGFj0iP7-hvdjTWxNs_fvL98HrrQ8/edit?usp=share_link)
- [Art Bible](https://docs.google.com/presentation/d/1VG11I8TGiMwpkelAd9ERPDhKpsiAkukuqHoUEt8hr7E/edit?usp=share_link)
