# Bevy Tidal

A zero-setup procedural music plugin for Bevy 0.19. Both the Tidal-inspired
pattern scheduler and the sample/synthesis renderer are written in Rust and run
inside the game process.

Players do **not** install or run GHC, Cabal, TidalCycles, SuperCollider,
SuperDirt, or a sidecar. The plugin opens the normal system audio device through
CPAL, reads WAV banks from the game assets, and shuts down with the Bevy app.

## Use it in a game

Add the private Git repository as a dependency:

```toml
[dependencies]
bevy_tidal = { git = "ssh://git@github.com/HumanBeanGames/bevy-tidal.git" }
```

Add the plugin and submit a pattern from a Bevy system:

```rust
use bevy::prelude::*;
use bevy_tidal::{TidalController, TidalPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TidalPlugin::default())
        .add_systems(Startup, start_music)
        .run();
}

fn start_music(controller: Res<TidalController>) {
    controller
        .play(1, r#"stack [sound "bd(3,8)", sound "hh*4?0.8"]"#)
        .expect("valid music pattern");
}
```

That is the complete runtime setup. Put sample banks at
`samples/<bank-name>/*.wav`, or set `TidalConfig::samples_path`. Missing banks
fall back to built-in kick, snare, hi-hat, clap, and tonal synthesizers.

## Native pattern language

The controller accepts familiar live-coding statements such as:

```haskell
d1 $ stack
  [ sound "bd(3,8)" # gain 1
  , sound "hh(5,8)" # gain 0.8 # cut 1
  , sound "cp*2?0.7" # gain 0.5 # shape 0.3
  ]

d2 $ n (slow 2 "0 3 5 7 <10 12 14>")
  # sound "cpu"
  # legato 1
  # cutoff (range 500 4000 $ slow 4 $ sine)
  # resonance 0.3
```

Implemented natively:

- classic, numbered, and named patterns (`d1`, `p 1234`, `p "name"`), plus
  `once`, `silence`, `hush`, `panic`, and live `setcps`
- recursive mini-notation: nesting, stacks, rests, alternation, random choice,
  replication, elongation, weights, ratios, degradation, Euclidean rhythms,
  sample indexes, and polymeters
- `stack`, `cat`/`slowcat`, `fastcat`, `randcat`, `timeCat`, `run`, `scan`,
  `randrun`, list patterns, and boolean `struct`/`mask`
- the documented Tidal scale and chord tables, named notes, chord voicings,
  inversions, arpeggiation, and rolled chords
- common time, condition, probability, repetition, stereo, echo, slicing, and
  sample transforms
- patterned controls, all standard oscillators, range/rangex, quantisation,
  segmentation, and deterministic random/Perlin signals
- native ASR envelopes, sample trimming/reverse playback, filters, distortion,
  bit/sample reduction, tremolo, ring modulation, delay, reverb, and stand-ins
  for the documented built-in SuperDirt synth names

[TIDAL_COVERAGE.md](TIDAL_COVERAGE.md) records the audit against the official
Tidal documentation, including the deliberately excluded integration-specific
features that would reintroduce Haskell, OSC/MIDI, SuperCollider, or third-party
UGens. This is source-compatible with the documented portable music surface;
it is not an embedded Haskell interpreter or a bit-identical SuperDirt clone.

## Configuration

```rust
use bevy_tidal::{TidalConfig, TidalPlugin};
use std::path::PathBuf;
use std::time::Duration;

let plugin = TidalPlugin {
    config: TidalConfig {
        samples_path: PathBuf::from("assets/music/samples"),
        cycles_per_second: 0.5,
        scheduler_lookahead: Duration::from_millis(100),
    },
};
```

`TidalEvent` is a normal Bevy message, so game systems can react to scheduled
notes for animation, lighting, combat timing, or debugging.

## Verify the complete path

The headless integration runner parses a real pattern file, schedules it in
Rust, renders native audio for five seconds, requires actual audio events, and
then calls `hush`:

```powershell
cargo run --features console -- --test-file test1.tidal
cargo run --features console -- --test-file tidal_documentation.tidal
```

The optional `console` feature is only for this repository's live editor and
integration runner. Games importing the library do not compile `bevy_egui`.

Normal Rust validation is also available:

```powershell
cargo test --all-targets --features console
cargo clippy --all-targets --features console -- -D warnings
```
