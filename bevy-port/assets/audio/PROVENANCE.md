# Audio provenance

The Bevy migration does not redistribute the Unity project's removed audio files.

Role-action cues are generated at runtime by `stream_town_game` as mono 16-bit
PCM WAV data. Presentation schema 19 retains the 35 original `AudioClip` GUID
references across the 14 role assets that authored clips; those GUIDs select a
deterministic variant but no source recording is present or copied. Smooth
oscillator/envelope profiles distinguish mining, logging, building, fishing,
archery, healing, magic, farming/gathering, and melee actions. Playback is
spatial at the actor, preserves Unity's 20-unit enable distance, and follows the
master/SFX mix. The signals use harmonic partials rather than sample-by-sample
noise, avoiding static-like hiss or discontinuities. These sounds contain no
sampled or third-party media and require no external attribution.

Seasonal day/night music beds and the ambient wind loop are also generated
entirely at runtime by `stream_town_game`. The synthesizer writes mono 16-bit PCM
WAV data from deterministic oscillators. The wind uses low-frequency integer
harmonics over the loop duration so it is band-limited and seam-free; it does
not use, derive from, or redistribute any recording or third-party composition.
The code is GPL-3.0-only with the rest of this repository and requires no
separate media attribution.

The three seagull-call variants are likewise generated at runtime from swept
oscillators and envelopes. They replace the unavailable source recordings used
by the shipping `SeagulSpawner` prefab; only that prefab's three-variant choice,
random 1–5 second cadence, ambience routing, and custom distance rolloff are
retained. No audio samples or third-party recordings are present.

Future replacement recordings must be added to this file with their author,
source URL, license, and any attribution text before release packaging.
