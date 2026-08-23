# Stream Town music

Music is scheduled and rendered in-process by the revision-pinned
[`bevy_tidal`](https://github.com/HumanBeanGames/bevy-tidal) library. The eight
expressions in `patterns/` are selected from the current season and day/night
state. They are Tidal expressions rather than complete `d1 $ ...` statements;
the game assigns the selected expression to music track 1 and applies the player
master/music volume as the final `gain` control.

Optional WAV sample banks can be placed at `samples/<bank-name>/*.wav`. The
shipping patterns use only the library's built-in native Rust synthesizers, so
no external TidalCycles, SuperCollider, sidecar, or sample download is needed.
