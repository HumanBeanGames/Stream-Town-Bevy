# Audio provenance

The Bevy migration does not redistribute the Unity project's removed audio files.

Role-action cues are generated at runtime by `stream_town_game` with Bevy's
`Pitch` sine-wave source. The cue frequency is chosen deterministically from the
converted animation name (or stable clip ID as a fallback), lasts 85 milliseconds,
and is played at 8% linear volume. These sounds contain no sampled or third-party
media and require no external attribution.

Seasonal day/night music beds and the ambient wind loop are also generated
entirely at runtime by `stream_town_game`. The synthesizer writes mono 16-bit PCM
WAV data from deterministic oscillators and pseudorandom noise; it does not use,
derive from, or redistribute any recording or third-party composition. The code
is GPL-3.0-only with the rest of this repository and requires no separate media
attribution.

The three seagull-call variants are likewise generated at runtime from swept
oscillators and envelopes. They replace the unavailable source recordings used
by the shipping `SeagulSpawner` prefab; only that prefab's three-variant choice,
random 1–5 second cadence, ambience routing, and custom distance rolloff are
retained. No audio samples or third-party recordings are present.

Future replacement recordings must be added to this file with their author,
source URL, license, and any attribution text before release packaging.
