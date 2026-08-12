# Audio provenance

The Bevy migration does not redistribute the Unity project's removed audio files.

Role-action cues are generated at runtime by `stream_town_game` with Bevy's
`Pitch` sine-wave source. The cue frequency is chosen deterministically from the
converted animation name (or stable clip ID as a fallback), lasts 85 milliseconds,
and is played at 8% linear volume. These sounds contain no sampled or third-party
media and require no external attribution.

Future replacement recordings must be added to this file with their author,
source URL, license, and any attribution text before release packaging.
