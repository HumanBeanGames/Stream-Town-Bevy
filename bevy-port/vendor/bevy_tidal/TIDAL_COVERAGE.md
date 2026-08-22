# Native Tidal documentation coverage

This ledger was audited against the current official TidalCycles reference on
2026-08-20. `bevy_tidal` implements the portable musical concepts directly in
Rust. It does not embed Haskell or SuperCollider, and therefore deliberately
does not claim byte-for-byte or DSP-identical compatibility with those runtimes.

## Implemented in the native engine

| Documentation area | Native coverage |
| --- | --- |
| Pattern lifecycle | `d1`-style tracks, `p 1234`, `p "name"`, `once`, `silence`, `hush`, `panic`, runtime `setcps` |
| Mini-notation | atoms, `~`, nested `[ ]`, dot grouping, in-pattern stacks with `,`, `*`, `/`, `|`, `< >`, `!`, `_`, `@`, `?`, `sound:index`, Euclidean `(p,s,offset)`, polymeters `{ }` and `%` subdivision |
| Concatenation/generation | `stack`, `cat`/`slowcat`, `fastcat`, `randcat`, `timeCat`, `run`, `scan`, `randrun`, `fromList`, `fromMaybes`, `listToPat`, `choose`, `cycleChoose` |
| Harmony | the complete documented scale table (including microtonal maqam scales), complete chord alias table, named notes/octaves, chord tone counts, open/drop/inversion voicings, `arp`/`arpeggiate`, `rolled`/`rolledBy` |
| Time/alteration | `fast`, `slow`, `density`, `sparsity`, `fastGap`/`densityGap`, `hurry`, `rev`, `palindrome`, `ply`, `press`/`pressBy`, `rotL`/`rotR`, `iter`/`iterBack`, `swing`/`swingBy`, `compress`, `zoom`, `trunc`, `linger`, `shuffle`, `scramble`, `loopFirst` |
| Layering/delay/stereo | `jux`/`juxBy`, `spin`, `echo`/`stut`, `off`, `ghost` |
| Conditions/randomness | `every`, `every'`, `sometimesBy`, `someCyclesBy`, `always`, `almostAlways`, `often`, `sometimes`, `rarely`, `almostNever`, `never`, event degradation, deterministic `rand`, `irand`, Perlin, random/cycle choices |
| Boolean structure | `struct`, `substruct`, `mask`, mini-notation Euclidean structure |
| Sampling | bank/index selection, `begin`, `end`, negative `speed`, `accelerate`, `sustain`, `legato`, `cut`, `chop`, `striate`, `striateBy`, `slice`, `splice`, `randslice`, `loopAt` |
| Control signals | patterned numeric values, sine, cosine, square, triangle, saw, inverse saw, smooth, random and Perlin oscillators; `range`, `rangex`, `quantise`, `segment`, `discretise`, `choose`, `cycleChoose` |
| Core sampler controls | `amp`, `gain`, `pan`, `speed`, `accelerate`, `freq`, `sustain`, `legato`, `attack`, `hold`, `release`, `begin`, `end`, `cut` |
| Native audio effects | `crush`, `coarse`, `shape`, `distort`, `triode`, low/high/band-pass filters and resonance, room/reverb, dry, delay time/feedback, tremolo, ring modulation |
| Built-in instruments | native implementations or lightweight stand-ins for the documented `super*`, `sos*`, classic drum aliases, sample banks, and default tonal oscillator |
| Bevy integration | one plugin, no child processes, CPAL device lifecycle, WAV bank cache, Bevy `TidalEvent` messages, synchronous parse errors, scheduler lookahead |

## Intentionally not emulated

These documentation sections describe integrations or host-language abilities,
not a portable pattern/audio feature. Reimplementing them would undo the
zero-setup native requirement or amount to embedding a second language:

- arbitrary Haskell expressions, user-defined functions/types, `StateT`, and
  direct use of the full `tidal-core` API
- SuperDirt OSC routing, orbits, control busses, Nudge/latency configuration,
  MIDI/Link/remote-controller setup, and audio-input synths
- SuperCollider server configuration, custom SynthDefs, exact SuperDirt DSP,
  Mutable Instruments/third-party UGens, and spectral FFT plugins
- editor boot files, package installation, external sample download/setup, and
  other environment-administration chapters

## Partial or native-API equivalents

- Complex higher-order combinators (`ur`, `seqP`, `snowball`, `spread`, custom
  `when` predicates, and user-provided transformation functions) require
  arbitrary Haskell functions. Their commonly documented outcomes are covered
  by native mini-notation, concatenation, conditions, transforms, or the Rust
  controller API, but arbitrary Haskell source is not interpreted.
- Stateful transition names (`xfade`, `clutch`, `anticipate`, `wash`, etc.) are
  SuperDirt/live-coding policies. Games can replace patterns synchronously and
  automate `gain`, filters, and tempo through `TidalController`; named Haskell
  transition functions are not parsed.
- Model-specific SuperDirt synth parameters are accepted as migration-friendly
  unknown controls but ignored unless they map to a native core control. The
  native instrument families are intentionally lightweight game-audio voices,
  not circuit/DSP replicas.

## Verification

`src/pattern.rs` contains unit coverage for the notation, harmony, conditions,
transforms, concatenation, signals, named patterns, and tempo surface.
`src/audio.rs` renders every native instrument family with the effect chain and
checks all output samples are finite. `tidal_documentation.tidal` exercises the
same path through the Bevy integration runner and a real CPAL output device.
