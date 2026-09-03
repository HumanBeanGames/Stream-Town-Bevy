# Stream Town music

Music is scheduled and rendered in-process by the revision-pinned
[`bevy_tidal`](https://github.com/HumanBeanGames/bevy-tidal) library. Stream
Town owns one adaptive expression on track 1. Its tempo, rhythm, harmony,
low-pass filters, and gain respond to a 15-second smoothed onscreen-enemy count;
updates coalesce and take effect on the next unscheduled cycle without resetting
the track's phase or layering another copy. Player master/music volume remains a
separate live routing control.

Optional WAV sample banks can be placed at `samples/<bank-name>/*.wav`. The
shipping patterns use only the library's built-in native Rust synthesizers, so
no external TidalCycles, SuperCollider, sidecar, or sample download is needed.
