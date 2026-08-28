use std::{path::Path, time::Duration};

use bevy::prelude::*;
use bevy_tidal::{
    NativeAudioRouting, NativeAudioState, NativeAudioStatus, TidalBackendState, TidalBackendStatus,
    TidalConfig, TidalController, TidalPlugin,
};
use stream_town_domain::Season;

use super::{RuntimeConfig, RuntimePlayerSettings, SimulationRuntime};

const MUSIC_TRACK: u64 = 1;
const MUSIC_GAIN: f32 = 0.22;
const MUSIC_CYCLES_PER_SECOND: f64 = 0.35;

const SPRING_DAY: &str = include_str!("../../../assets/music/patterns/spring_day.tidal");
const SPRING_NIGHT: &str = include_str!("../../../assets/music/patterns/spring_night.tidal");
const SUMMER_DAY: &str = include_str!("../../../assets/music/patterns/summer_day.tidal");
const SUMMER_NIGHT: &str = include_str!("../../../assets/music/patterns/summer_night.tidal");
const AUTUMN_DAY: &str = include_str!("../../../assets/music/patterns/autumn_day.tidal");
const AUTUMN_NIGHT: &str = include_str!("../../../assets/music/patterns/autumn_night.tidal");
const WINTER_DAY: &str = include_str!("../../../assets/music/patterns/winter_day.tidal");
const WINTER_NIGHT: &str = include_str!("../../../assets/music/patterns/winter_night.tidal");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MusicSignature {
    season: Season,
    daytime: bool,
}

#[derive(Resource, Default)]
pub(super) struct TidalMusicRuntime {
    applied: Option<MusicSignature>,
    failed: Option<MusicSignature>,
    diagnostic: Option<String>,
}

pub(super) fn tidal_plugin(asset_root: &Path) -> TidalPlugin {
    TidalPlugin {
        config: TidalConfig {
            samples_path: asset_root.join("music").join("samples"),
            cycles_per_second: MUSIC_CYCLES_PER_SECOND,
            scheduler_lookahead: Duration::from_millis(100),
        },
    }
}

pub(super) fn drive_tidal_music(
    controller: Option<Res<TidalController>>,
    backend: Option<Res<TidalBackendStatus>>,
    audio: Option<Res<NativeAudioStatus>>,
    routing: Option<Res<NativeAudioRouting>>,
    simulation: Option<Res<SimulationRuntime>>,
    config: Res<RuntimeConfig>,
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

    let Some(simulation) = simulation else {
        silence_music(&controller, &mut runtime);
        return;
    };
    let daytime = config
        .0
        .time
        .sample(simulation.0.elapsed_seconds)
        .is_daytime;
    let signature = MusicSignature {
        season: simulation.0.season,
        daytime,
    };
    if runtime.applied == Some(signature) || runtime.failed == Some(signature) {
        return;
    }

    let expression = music_expression(signature.season, signature.daytime);
    match controller.play(MUSIC_TRACK, &expression) {
        Ok(()) => {
            info!(
                target: "stream_town::music",
                "Started native Tidal music for {:?} {}",
                signature.season,
                if signature.daytime { "day" } else { "night" },
            );
            runtime.applied = Some(signature);
            runtime.failed = None;
            runtime.diagnostic = None;
        }
        Err(error) => {
            runtime.failed = Some(signature);
            report_once(
                &mut runtime,
                format!("Could not apply the Stream Town Tidal pattern: {error}"),
            );
        }
    }
}

pub(super) fn stop_tidal_music(
    controller: Option<Res<TidalController>>,
    mut runtime: ResMut<TidalMusicRuntime>,
) {
    if let Some(controller) = controller {
        silence_music(&controller, &mut runtime);
    } else {
        runtime.applied = None;
        runtime.failed = None;
    }
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
    if runtime.applied.is_some()
        && let Err(error) = controller.silence(MUSIC_TRACK)
    {
        report_once(
            runtime,
            format!("Could not silence the Stream Town Tidal track: {error}"),
        );
    }
    runtime.applied = None;
    runtime.failed = None;
}

fn report_once(runtime: &mut TidalMusicRuntime, diagnostic: String) {
    if runtime.diagnostic.as_deref() != Some(diagnostic.as_str()) {
        warn!(target: "stream_town::music", "{diagnostic}");
        runtime.diagnostic = Some(diagnostic);
    }
}

fn player_music_gain(settings: &stream_town_domain::PlayerSettings) -> f32 {
    (settings.audio.master * settings.audio.music).clamp(0.0, 1.0)
}

fn music_expression(season: Season, daytime: bool) -> String {
    music_template(season, daytime).replace("{{gain}}", &format!("{MUSIC_GAIN:.6}"))
}

fn music_template(season: Season, daytime: bool) -> &'static str {
    match (season, daytime) {
        (Season::Spring, true) => SPRING_DAY,
        (Season::Spring, false) => SPRING_NIGHT,
        (Season::Summer, true) => SUMMER_DAY,
        (Season::Summer, false) => SUMMER_NIGHT,
        (Season::Autumn, true) => AUTUMN_DAY,
        (Season::Autumn, false) => AUTUMN_NIGHT,
        (Season::Winter, true) => WINTER_DAY,
        (Season::Winter, false) => WINTER_NIGHT,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn every_authored_pattern_parses_in_the_native_engine() {
        let mut app = App::new();
        app.add_plugins(TidalPlugin::default());
        let controller = app.world().resource::<TidalController>();
        for season in [
            Season::Spring,
            Season::Summer,
            Season::Autumn,
            Season::Winter,
        ] {
            for daytime in [true, false] {
                let expression = music_expression(season, daytime);
                controller
                    .play(MUSIC_TRACK, expression)
                    .unwrap_or_else(|error| panic!("{season:?}/{daytime} did not parse: {error}"));
            }
        }
    }

    #[test]
    fn every_season_and_time_of_day_has_a_distinct_pattern() {
        let patterns: HashSet<_> = [
            Season::Spring,
            Season::Summer,
            Season::Autumn,
            Season::Winter,
        ]
        .into_iter()
        .flat_map(|season| [true, false].map(move |daytime| music_template(season, daytime).trim()))
        .collect();
        assert_eq!(patterns.len(), 8);
    }

    #[test]
    fn authored_gain_is_finite_bounded_and_fully_substituted() {
        let mut settings = stream_town_domain::PlayerSettings::default();
        settings.audio.master = 0.5;
        settings.audio.music = 0.4;
        let gain = player_music_gain(&settings);
        assert!((gain - 0.2).abs() < f32::EPSILON);
        let expression = music_expression(Season::Spring, true);
        assert!(!expression.contains("{{gain}}"));
        assert!(expression.contains(&format!("{MUSIC_GAIN:.6}")));
    }

    #[test]
    fn volume_is_not_part_of_the_pattern_signature() {
        let signature = MusicSignature {
            season: Season::Spring,
            daytime: true,
        };
        let mut settings = stream_town_domain::PlayerSettings::default();
        let before = music_expression(signature.season, signature.daytime);
        settings.audio.master = 0.1;
        settings.audio.music = 0.2;
        let after = music_expression(signature.season, signature.daytime);
        assert_eq!(before, after);
        assert!((player_music_gain(&settings) - 0.02).abs() < f32::EPSILON);
    }
}
