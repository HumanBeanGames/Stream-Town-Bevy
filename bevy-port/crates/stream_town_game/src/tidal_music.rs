use std::{path::Path, time::Duration};

use bevy::prelude::*;
use bevy_tidal::{
    NativeAudioRouting, NativeAudioState, NativeAudioStatus, TidalBackendState, TidalBackendStatus,
    TidalConfig, TidalController, TidalPlugin,
};
use stream_town_domain::{ActorKind, PlayerSettings};

use super::{Agent, RuntimeConfig, RuntimePlayerSettings, SimulationRuntime, TownCamera};

const MUSIC_TRACK: u64 = 1;
const INTENSITY_SMOOTHING_SECONDS: f64 = 15.0;
const INTENSITY_PROGRAM_THRESHOLD: f64 = 0.25;
const MAX_SONG_INTENSITY: f64 = 12.0;

const HARMONY_ROOTS: [&str; 7] = [
    "<c4 a3 f3 g3 c4 e3 f3 g3>",
    "<c4 a3 d3 g3 c4 e3 d3 g3>",
    "<c4 a3 f3 g3 c4 eb3 f3 g3>",
    "<c4 eb3 f3 g3 c4 ab3 f3 g3>",
    "<c4 ab3 f3 g3 c4 eb3 ab3 g3>",
    "<c4 db3 f3 g3 c4 ab3 db3 g3>",
    "<c4 db3 fs3 g3 c4 ab3 fs3 g3>",
];

const HARMONY_CHORDS: [&str; 7] = [
    "<c4'M a3'm f3'M g3'M c4'M e3'm f3'M g3'M>",
    "<c4'M a3'm d3'm g3'M c4'M e3'm d3'm g3'M>",
    "<c4'M a3'm f3'm g3'M c4'M eb3'M f3'm g3'M>",
    "<c4'M eb3'M f3'm g3'M c4'M ab3'M f3'm g3'M>",
    "<c4'M ab3'M f3'm g3'dom7 c4'M eb3'M ab3'M g3'dom7>",
    "<c4'M db3'M f3'm g3'dom7 c4'M ab3'M db3'M g3'dom7>",
    "<c4'M db3'M fs3'dim g3'7f9 c4'M ab3'M fs3'dim g3'7f9>",
];

const SCORE_TEMPLATE: &str = r#"stack
  [ sound "${kick}" # lpf ${kick_brightness} # gain ${kick_gain}
  , struct "${root_pattern}" $ ${roots}
      # sound "gm_harpsichord"
      # legato 0.9
      # attack ${melody_attack}
      # decay 0.02
      # release 0.035
      # lpf ${melody_brightness}
      # gain ${melody_gain}
      # room 0.08
      # roomsize 1
  , struct "${chord_pattern}" $ ${chords}
      # sound "gm_harpsichord"
      # attack ${chord_attack}
      # release 0.4
      # lpf ${chord_brightness}
      # gain ${chord_gain}
      # room 0.20
      # roomsize 2
  , sound "${hats}" # lpf ${hat_brightness} # gain ${hat_gain}
  ]"#;

/// Externally driven input to the downstream-owned intensity composition.
#[derive(Resource, Clone, Debug, Default)]
pub(super) struct IntensitySongInput {
    pub(super) intensity: f64,
    pub(super) visible_enemies: usize,
}

#[derive(Resource, Default)]
pub(super) struct TidalMusicRuntime {
    applied_intensity: Option<f64>,
    failed_intensity: Option<f64>,
    diagnostic: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct IntensitySongProgram {
    expression: String,
    cycles_per_second: f64,
}

pub(super) fn tidal_plugin(asset_root: &Path) -> TidalPlugin {
    TidalPlugin {
        config: TidalConfig {
            samples_path: asset_root.join("music").join("samples"),
            cycles_per_second: 0.3125,
            scheduler_lookahead: Duration::from_millis(100),
        },
    }
}

pub(super) fn update_enemy_music_intensity(
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    simulation: Res<SimulationRuntime>,
    cameras: Query<(&Camera, &GlobalTransform), With<TownCamera>>,
    agents: Query<(&Agent, &GlobalTransform)>,
    mut input: ResMut<IntensitySongInput>,
) {
    let visible_enemies = cameras.single().map_or(0, |(camera, camera_transform)| {
        let actor_height = config.0.world.cell_size * 0.6;
        agents
            .iter()
            .filter(|(agent, _)| {
                agent.kind == ActorKind::Enemy
                    && simulation
                        .0
                        .actors
                        .get(&agent.id)
                        .is_some_and(|actor| actor.alive)
            })
            .filter(|(_, transform)| {
                position_is_onscreen(
                    camera,
                    camera_transform,
                    transform.translation() + Vec3::Y * actor_height,
                )
            })
            .count()
    });
    input.visible_enemies = visible_enemies;
    let visible_enemies = u32::try_from(visible_enemies).unwrap_or(u32::MAX);
    input.intensity = smoothed_intensity(
        input.intensity,
        f64::from(visible_enemies),
        time.delta_secs_f64(),
    );
}

pub(super) fn drive_tidal_music(
    controller: Option<Res<TidalController>>,
    backend: Option<Res<TidalBackendStatus>>,
    audio: Option<Res<NativeAudioStatus>>,
    routing: Option<Res<NativeAudioRouting>>,
    input: Res<IntensitySongInput>,
    player_settings: Res<RuntimePlayerSettings>,
    mut runtime: ResMut<TidalMusicRuntime>,
) {
    let (Some(controller), Some(backend), Some(audio), Some(routing)) =
        (controller, backend, audio, routing)
    else {
        report_once(
            &mut runtime,
            "The bevy-tidal plugin resources are unavailable".to_owned(),
        );
        return;
    };

    if !tidal_is_ready(&backend, &audio, &mut runtime) {
        return;
    }

    routing.set_master_gain(player_music_gain(&player_settings.0));
    let requested_intensity = input.intensity.clamp(0.0, MAX_SONG_INTENSITY);
    if !intensity_program_needs_update(
        runtime.applied_intensity,
        runtime.failed_intensity,
        requested_intensity,
    ) {
        return;
    }

    match intensity_song_program(requested_intensity).and_then(|program| {
        controller.transition(MUSIC_TRACK, program.expression, program.cycles_per_second)
    }) {
        Ok(()) => {
            if runtime.applied_intensity.is_none() {
                info!(
                    target: "stream_town::music",
                    "Started native Tidal intensity score"
                );
            }
            runtime.applied_intensity = Some(requested_intensity);
            runtime.failed_intensity = None;
            runtime.diagnostic = None;
        }
        Err(error) => {
            runtime.failed_intensity = Some(requested_intensity);
            report_once(
                &mut runtime,
                format!("Could not apply the Stream Town intensity score: {error}"),
            );
        }
    }
}

pub(super) fn stop_tidal_music(
    controller: Option<Res<TidalController>>,
    mut input: ResMut<IntensitySongInput>,
    mut runtime: ResMut<TidalMusicRuntime>,
) {
    if let Some(controller) = controller {
        silence_music(&controller, &mut runtime);
    } else {
        runtime.applied_intensity = None;
        runtime.failed_intensity = None;
    }
    *input = IntensitySongInput::default();
}

fn position_is_onscreen(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    world_position: Vec3,
) -> bool {
    if !camera.is_active {
        return false;
    }
    let Some(viewport) = camera.logical_viewport_size() else {
        return false;
    };
    camera
        .world_to_viewport(camera_transform, world_position)
        .is_ok_and(|position| point_inside_viewport(viewport, position))
}

fn point_inside_viewport(viewport: Vec2, position: Vec2) -> bool {
    position.x >= 0.0 && position.y >= 0.0 && position.x <= viewport.x && position.y <= viewport.y
}

fn smoothed_intensity(previous: f64, visible_enemies: f64, delta_seconds: f64) -> f64 {
    let previous = if previous.is_finite() {
        previous.max(0.0)
    } else {
        0.0
    };
    let target = if visible_enemies.is_finite() {
        visible_enemies.max(0.0)
    } else {
        0.0
    };
    let alpha = 1.0 - (-delta_seconds.max(0.0) / INTENSITY_SMOOTHING_SECONDS).exp();
    previous + (target - previous) * alpha
}

fn intensity_program_needs_update(
    applied: Option<f64>,
    failed: Option<f64>,
    requested: f64,
) -> bool {
    if applied.is_none() && failed.is_none() {
        return true;
    }
    if failed.is_some() {
        return true;
    }
    let previous = applied.unwrap_or_default();
    (requested - previous).abs() >= INTENSITY_PROGRAM_THRESHOLD
}

fn intensity_cycles_per_second(intensity: f64) -> f64 {
    (75.0 + intensity.clamp(0.0, MAX_SONG_INTENSITY) * 5.0) / 240.0
}

fn tidal_is_ready(
    backend: &TidalBackendStatus,
    audio: &NativeAudioStatus,
    runtime: &mut TidalMusicRuntime,
) -> bool {
    if backend.state() == TidalBackendState::Error {
        report_once(
            runtime,
            format!("Native Tidal pattern engine error: {}", backend.detail()),
        );
        return false;
    }
    if audio.state() == NativeAudioState::Error {
        report_once(
            runtime,
            format!("Native Tidal audio error: {}", audio.detail()),
        );
        return false;
    }
    backend.is_ready() && audio.is_ready()
}

fn silence_music(controller: &TidalController, runtime: &mut TidalMusicRuntime) {
    if runtime.applied_intensity.is_some()
        && let Err(error) = controller.silence(MUSIC_TRACK)
    {
        report_once(
            runtime,
            format!("Could not silence the Stream Town Tidal track: {error}"),
        );
    }
    runtime.applied_intensity = None;
    runtime.failed_intensity = None;
}

fn report_once(runtime: &mut TidalMusicRuntime, diagnostic: String) {
    if runtime.diagnostic.as_deref() != Some(diagnostic.as_str()) {
        warn!(target: "stream_town::music", "{diagnostic}");
        runtime.diagnostic = Some(diagnostic);
    }
}

fn player_music_gain(settings: &PlayerSettings) -> f32 {
    (settings.audio.master * settings.audio.music).clamp(0.0, 1.0)
}

/// Renders the complete score from its externally supplied intensity.
fn intensity_song_program(intensity: f64) -> Result<IntensitySongProgram, String> {
    if !intensity.is_finite() {
        return Err("The downstream song intensity must be finite".to_owned());
    }
    let intensity = intensity.clamp(0.0, MAX_SONG_INTENSITY);
    let t = intensity / MAX_SONG_INTENSITY;
    let attack_t = t * t * t;

    let melody_attack = lerp(0.14, 0.01, attack_t);
    let melody_brightness = lerp(1_200.0, 4_000.0, t);
    let melody_gain = lerp(0.35, 0.85, t);
    let chord_attack = lerp(0.11, 0.03, attack_t);
    let chord_brightness = lerp(1_100.0, 3_000.0, t);
    let chord_gain = lerp(0.14, 0.30, t);
    let kick_brightness = lerp(500.0, 2_200.0, t);
    let kick_gain = lerp(0.04, 0.12, t);
    let hat_brightness = lerp(1_200.0, 5_000.0, t);
    let hat_gain = lerp(0.12, 0.38, t);

    let slow_hits = 1 + intensity_steps(intensity, 4.0, 2);
    let mid_hits = 2 + intensity_steps(intensity, 2.0, 5);
    let fast_hits = 3 + intensity_steps(intensity, 1.0, 12);
    let mid_rotation = intensity_steps(intensity, 3.0, 4);
    let fast_rotation = 2 + intensity_steps(intensity, 2.0, 6);

    let mid_positions = euclidean_steps(mid_hits, 8, mid_rotation)
        .iter()
        .enumerate()
        .filter_map(|(index, hit)| hit.then_some(index))
        .collect::<Vec<_>>();
    let first_hit = mid_positions[0];
    let second_hit = mid_positions[1];

    let mut root_steps = ["~"; 8];
    root_steps[first_hit] = "x";
    root_steps
        .iter_mut()
        .take(second_hit)
        .skip(first_hit + 1)
        .for_each(|step| *step = "_");
    let root_pattern = root_steps.join(" ");

    let mut chord_steps = ["~"; 8];
    for &position in mid_positions.iter().skip(1) {
        chord_steps[position] = "x";
    }
    let chord_pattern = chord_steps.join(" ");

    let harmony_level = intensity_steps(intensity, 2.0, HARMONY_ROOTS.len() - 1);
    let lower_level = harmony_level.saturating_sub(1);
    let upper_level = (harmony_level + 1).min(HARMONY_ROOTS.len() - 1);

    let values = [
        (
            "kick",
            sound_pattern(&euclidean_steps(slow_hits, 4, 0), "bd"),
        ),
        ("kick_brightness", format_number(kick_brightness)),
        ("kick_gain", format_number(kick_gain)),
        ("root_pattern", root_pattern),
        (
            "roots",
            weighted_harmony(HARMONY_ROOTS, harmony_level, lower_level, upper_level),
        ),
        ("melody_attack", format_number(melody_attack)),
        ("melody_brightness", format_number(melody_brightness)),
        ("melody_gain", format_number(melody_gain)),
        ("chord_pattern", chord_pattern),
        (
            "chords",
            weighted_harmony(HARMONY_CHORDS, harmony_level, lower_level, upper_level),
        ),
        ("chord_attack", format_number(chord_attack)),
        ("chord_brightness", format_number(chord_brightness)),
        ("chord_gain", format_number(chord_gain)),
        (
            "hats",
            sound_pattern(&euclidean_steps(fast_hits, 16, fast_rotation), "hh"),
        ),
        ("hat_brightness", format_number(hat_brightness)),
        ("hat_gain", format_number(hat_gain)),
    ];
    Ok(IntensitySongProgram {
        expression: render_score_template(&values)?,
        cycles_per_second: intensity_cycles_per_second(intensity),
    })
}

fn render_score_template(values: &[(&str, String)]) -> Result<String, String> {
    let mut program = SCORE_TEMPLATE.to_owned();
    for (name, value) in values {
        program = program.replace(&format!("${{{name}}}"), value);
    }
    if program.contains("${") {
        Err("The downstream score has an unresolved variable".to_owned())
    } else {
        Ok(program)
    }
}

fn format_number(value: f64) -> String {
    format!("{value:.6}")
}

fn lerp(low: f64, high: f64, amount: f64) -> f64 {
    low + (high - low) * amount
}

fn intensity_steps(intensity: f64, step_size: f64, maximum: usize) -> usize {
    (1..=maximum)
        .take_while(|step| {
            let step = u32::try_from(*step).unwrap_or(u32::MAX);
            intensity >= f64::from(step) * step_size
        })
        .count()
}

fn euclidean_steps(pulses: usize, steps: usize, rotation: usize) -> Vec<bool> {
    let base = (0..steps)
        .map(|step| (step * pulses) % steps < pulses)
        .collect::<Vec<_>>();
    let rotation = rotation % steps;
    (0..steps)
        .map(|step| base[(step + steps - rotation) % steps])
        .collect()
}

fn sound_pattern(steps: &[bool], sound: &str) -> String {
    steps
        .iter()
        .map(|hit| if *hit { sound } else { "~" })
        .collect::<Vec<_>>()
        .join(" ")
}

fn weighted_harmony(pools: [&str; 7], current: usize, lower: usize, upper: usize) -> String {
    format!(
        "slowcat [n \"{}\", n \"{}\", n \"{}\", n \"{}\", n \"{}\"]",
        pools[current], pools[current], pools[current], pools[lower], pools[upper]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_supported_intensity_parses_in_the_native_engine() {
        let mut app = App::new();
        app.add_plugins(TidalPlugin::default());
        let controller = app.world().resource::<TidalController>();
        for intensity in [0.0, 0.5, 1.0, 3.0, 6.0, 9.0, 12.0, 100.0] {
            let program = intensity_song_program(intensity).unwrap();
            controller
                .transition(MUSIC_TRACK, program.expression, program.cycles_per_second)
                .unwrap_or_else(|error| panic!("intensity {intensity} did not parse: {error}"));
        }
    }

    #[test]
    fn intensity_smoothing_has_a_fifteen_second_time_constant() {
        let one_step = smoothed_intensity(0.0, 12.0, 15.0);
        let many_steps = (0..900).fold(0.0, |value, _| {
            smoothed_intensity(value, 12.0, 15.0 / 900.0)
        });
        let expected = 12.0 * (1.0 - (-1.0_f64).exp());
        assert!((one_step - expected).abs() < 1.0e-10);
        assert!((many_steps - expected).abs() < 1.0e-9);
    }

    #[test]
    fn score_submissions_retain_external_precision_and_are_thresholded() {
        assert!(intensity_program_needs_update(None, None, 0.0));
        assert!(!intensity_program_needs_update(Some(1.0), None, 1.1));
        assert!(intensity_program_needs_update(Some(1.0), None, 1.3));
        assert!(intensity_program_needs_update(Some(1.0), Some(1.3), 1.3));
    }

    #[test]
    fn authored_tempo_still_spans_the_intensity_range() {
        assert!((intensity_cycles_per_second(0.0).recip() - 3.2).abs() < 1.0e-10);
        assert!((intensity_cycles_per_second(12.0) - 0.5625).abs() < 1.0e-10);
    }

    #[test]
    fn composition_is_one_transitionable_native_expression() {
        let program = intensity_song_program(6.0).unwrap();
        assert!(program.expression.starts_with("stack"));
        assert!(program.expression.contains("slowcat [n"));
        assert!(!program.expression.contains("setcps"));
        assert!(!program.expression.contains("d1"));
        assert!(!program.expression.contains("wchooseCycles"));
        assert!(!program.expression.contains("${"));
    }

    #[test]
    fn intensity_changes_every_authored_low_pass_filter() {
        let quiet = intensity_song_program(0.0).unwrap().expression;
        let intense = intensity_song_program(12.0).unwrap().expression;
        for cutoff in ["lpf 500.000000", "lpf 1200.000000", "lpf 1100.000000"] {
            assert!(quiet.contains(cutoff), "quiet score omitted {cutoff}");
        }
        for cutoff in ["lpf 2200.000000", "lpf 5000.000000", "lpf 4000.000000"] {
            assert!(intense.contains(cutoff), "intense score omitted {cutoff}");
        }
    }

    #[test]
    fn viewport_bounds_include_edges_and_exclude_offscreen_points() {
        let viewport = Vec2::new(1920.0, 1080.0);
        assert!(point_inside_viewport(viewport, Vec2::ZERO));
        assert!(point_inside_viewport(viewport, viewport));
        assert!(!point_inside_viewport(viewport, Vec2::new(-0.1, 500.0)));
        assert!(!point_inside_viewport(viewport, Vec2::new(960.0, 1080.1)));
    }

    #[test]
    fn volume_changes_do_not_change_the_composition() {
        let mut settings = PlayerSettings::default();
        settings.audio.master = 0.5;
        settings.audio.music = 0.4;
        let before = intensity_song_program(4.0).unwrap();
        settings.audio.master = 0.1;
        settings.audio.music = 0.2;
        let after = intensity_song_program(4.0).unwrap();
        assert_eq!(before, after);
        assert!((player_music_gain(&settings) - 0.02).abs() < f32::EPSILON);
    }
}
